#![allow(unused)]

use core::{convert::TryInto, mem::transmute};
use fnd::{
    alloc::{Allocator, GlobalAllocator},
    containers::{Array, String},
};
use std::{
    io::{Bytes, Cursor, Read, Write},
    vec::Vec,
};

mod impls;
mod traits;

use traits::*;

struct PeekOneReader<R>
{
    r: R,
    peek: Option<std::io::Result<u8>>,
}

impl<R: Read> PeekOneReader<R>
{
    fn new(r: R) -> Self
    {
        Self { r, peek: None }
    }

    fn peek_one(&mut self) -> &std::io::Result<u8>
    {
        if self.peek.is_none()
        {
            let mut b = [0u8; 1];
            self.peek = match self.r.read_exact(&mut b)
            {
                Ok(_) => Some(Ok(b[0])),
                Err(e) => Some(Err(e)),
            };
        }

        return self.peek.as_ref().unwrap();
    }
}

impl<R: Read> Read for PeekOneReader<R>
{
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize>
    {
        if buf.len() == 0
        {
            return Ok(0);
        }

        let already_read = if self.peek.is_some()
        {
            let result = self.peek.take().unwrap()?;
            buf[0] = result;
            buf = &mut buf[1..];
            1
        }
        else
        {
            0
        };

        Ok(already_read + self.r.read(buf)?)
    }
}

#[test]
fn peek_one()
{
    let mut buffer = [0u8; 8];
    let reader: &[u8] = &[1, 2, 3, 4, 5, 6];
    let mut reader = PeekOneReader::new(reader);

    assert_eq!(*reader.peek_one().as_ref().unwrap(), 1u8);
    assert_eq!(*reader.peek_one().as_ref().unwrap(), 1u8);

    reader.read_exact(&mut buffer[0..2]).unwrap();
    assert_eq!(&buffer[0..2], &[1, 2]);

    assert_eq!(*reader.peek_one().as_ref().unwrap(), 3u8);

    reader.read_exact(&mut buffer[0..1]).unwrap();
    assert_eq!(&buffer[0..1], &[3]);

    reader.read_exact(&mut buffer[0..2]).unwrap();
    assert_eq!(&buffer[0..2], &[4, 5]);

    assert_eq!(*reader.peek_one().as_ref().unwrap(), 6u8);

    reader.read_exact(&mut buffer[0..1]).unwrap();
    assert_eq!(&buffer[0..1], &[6]);

    assert!(reader.peek_one().is_err());
}

struct CborSerializer<W>
{
    write: W,
}

struct CborCompoundSerializer<'a, W>
{
    s: &'a mut CborSerializer<W>,
    needs_break: bool,
}

struct CborArraySerializer<'a, W>(CborCompoundSerializer<'a, W>);
struct CborMapSerializer<'a, W>(CborCompoundSerializer<'a, W>);

#[derive(PartialEq)]
enum MajorType
{
    UnsignedInteger,
    NegativeInteger,
    ByteString,
    TextString,
    Array,
    Map,
    FloatingPoint,
    Special,
}

const BREAK: u8 = 31;
const TRUE: u8 = 21;
const FALSE: u8 = 20;
const NULL: u8 = 22;
const F64: u8 = 27;
const F32: u8 = 26;

impl MajorType
{
    fn from_raw(raw: u8) -> Option<(MajorType, u8)>
    {
        let major_type_raw = raw >> 5;
        let payload = raw & 0x1f;

        match major_type_raw
        {
            0 => Some((MajorType::UnsignedInteger, payload)),
            1 => Some((MajorType::NegativeInteger, payload)),
            2 => Some((MajorType::ByteString, payload)),
            3 => Some((MajorType::TextString, payload)),
            4 => Some((MajorType::Array, payload)),
            5 => Some((MajorType::Map, payload)),
            7 => match payload
            {
                F32 | F64 => Some((MajorType::FloatingPoint, payload)),
                BREAK | TRUE | FALSE | NULL => Some((MajorType::Special, payload)),
                _ => None,
            },
            _ => None,
        }
    }

    fn as_int(self) -> u8
    {
        match self
        {
            MajorType::UnsignedInteger => 0,
            MajorType::NegativeInteger => 1,
            MajorType::ByteString => 2,
            MajorType::TextString => 3,
            MajorType::Array => 4,
            MajorType::Map => 5,
            MajorType::FloatingPoint => 7,
            MajorType::Special => 7,
        }
    }

    #[inline]
    fn make_header(self, additional_data: u8) -> u8
    {
        debug_assert!(additional_data < 32);
        (self.as_int() << 5) | additional_data
    }
}

impl<W> CborSerializer<W>
where
    W: Write,
{
    fn write_header_with_value(&mut self, major_type: MajorType, value: u64) -> Result<(), ()>
    {
        if value <= 23
        {
            self.write_u8(major_type.make_header(value as u8))
        }
        else if value <= core::u8::MAX as u64
        {
            self.write_u8(major_type.make_header(24))
                .and_then(|_| self.write_u8(value as u8))
        }
        else if value <= core::u16::MAX as u64
        {
            self.write_u8(major_type.make_header(25))
                .and_then(|_| self.write_u16(value as u16))
        }
        else if value <= core::u32::MAX as u64
        {
            self.write_u8(major_type.make_header(26))
                .and_then(|_| self.write_u32(value as u32))
        }
        else if value <= core::u64::MAX as u64
        {
            self.write_u8(major_type.make_header(27))
                .and_then(|_| self.write_u64(value as u64))
        }
        else
        {
            Err(())
        }
    }

    #[inline]
    fn write_u8(&mut self, data: u8) -> Result<(), ()>
    {
        self.write_bytes(&[data])
    }

    #[inline]
    fn write_u16(&mut self, data: u16) -> Result<(), ()>
    {
        self.write_bytes(&data.to_be_bytes())
    }

    #[inline]
    fn write_u32(&mut self, data: u32) -> Result<(), ()>
    {
        self.write_bytes(&data.to_be_bytes())
    }

    #[inline]
    fn write_u64(&mut self, data: u64) -> Result<(), ()>
    {
        self.write_bytes(&data.to_be_bytes())
    }

    #[inline]
    fn write_bytes(&mut self, data: &[u8]) -> Result<(), ()>
    {
        self.write.write_all(data).map(|_| ()).map_err(|_| ())
    }

    #[inline]
    fn write_f32(&mut self, data: f32) -> Result<(), ()>
    {
        self.write_u32(unsafe { transmute(data) })
    }

    #[inline]
    fn write_f64(&mut self, data: f64) -> Result<(), ()>
    {
        self.write_u64(unsafe { transmute(data) })
    }
}

impl<'a, W> ArraySerializer for CborArraySerializer<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Err = ();

    #[inline]
    fn serialize_element<T: Serialize>(&mut self, elmt: T) -> Result<(), ()>
    {
        elmt.serialize(&mut *self.0.s)
    }

    fn end(self) -> Result<(), ()>
    {
        if self.0.needs_break
        {
            self.0.s.write_u8(MajorType::Special.make_header(BREAK))
        }
        else
        {
            Ok(())
        }
    }
}

impl<'a, W> MapSerializer for CborMapSerializer<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Err = ();

    #[inline]
    fn serialize_entry<K: Serialize, V: Serialize>(&mut self, k: K, v: V) -> Result<(), ()>
    {
        k.serialize(&mut *self.0.s)?;
        v.serialize(&mut *self.0.s)
    }

    fn end(self) -> Result<(), ()>
    {
        if self.0.needs_break
        {
            self.0.s.write_u8(MajorType::Special.make_header(BREAK))
        }
        else
        {
            Ok(())
        }
    }
}

impl<'a, W> Serializer for &'a mut CborSerializer<W>
where
    W: Write,
{
    type Ok = ();
    type Err = ();
    type ArraySerializer = CborArraySerializer<'a, W>;
    type MapSerializer = CborMapSerializer<'a, W>;

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Err>
    {
        self.write_header_with_value(MajorType::UnsignedInteger, value as u64)
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Err>
    {
        if value >= 0
        {
            self.write_header_with_value(MajorType::UnsignedInteger, value as u64)
        }
        else
        {
            self.write_header_with_value(MajorType::NegativeInteger, (-1 - value) as u64)
        }
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Err>
    {
        self.write_header_with_value(MajorType::UnsignedInteger, value as u64)
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Err>
    {
        if value >= 0
        {
            self.write_header_with_value(MajorType::UnsignedInteger, value as u64)
        }
        else
        {
            self.write_header_with_value(MajorType::NegativeInteger, (-1 - value) as u64)
        }
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Err>
    {
        self.write_header_with_value(MajorType::UnsignedInteger, value as u64)
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Err>
    {
        if value >= 0
        {
            self.write_header_with_value(MajorType::UnsignedInteger, value as u64)
        }
        else
        {
            self.write_header_with_value(MajorType::NegativeInteger, (-1 - value) as u64)
        }
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Err>
    {
        self.write_header_with_value(MajorType::UnsignedInteger, value)
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Err>
    {
        if value >= 0
        {
            self.write_header_with_value(MajorType::UnsignedInteger, value as u64)
        }
        else
        {
            self.write_header_with_value(MajorType::NegativeInteger, (-1 - value) as u64)
        }
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Err>
    {
        self.write_header_with_value(MajorType::ByteString, value.len() as u64)
            .and_then(|_| self.write_bytes(value))
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Err>
    {
        self.write_header_with_value(MajorType::TextString, value.as_bytes().len() as u64)
            .and_then(|_| self.write_bytes(value.as_bytes()))
    }

    fn serialize_array(self, len: Option<usize>) -> Result<CborArraySerializer<'a, W>, Self::Err>
    {
        match len
        {
            Some(len) => self
                .write_header_with_value(MajorType::Array, len as u64)
                .and_then(move |_| {
                    Ok(CborArraySerializer(CborCompoundSerializer {
                        s: self,
                        needs_break: false,
                    }))
                }),
            None => self
                .write_u8(MajorType::Array.make_header(0x1f))
                .and_then(move |_| {
                    Ok(CborArraySerializer(CborCompoundSerializer {
                        s: self,
                        needs_break: true,
                    }))
                }),
        }
    }

    fn serialize_map(self, len: Option<usize>) -> Result<CborMapSerializer<'a, W>, Self::Err>
    {
        match len
        {
            Some(len) => self
                .write_header_with_value(MajorType::Map, len as u64)
                .and_then(move |_| {
                    Ok(CborMapSerializer(CborCompoundSerializer {
                        s: self,
                        needs_break: false,
                    }))
                }),
            None => self
                .write_u8(MajorType::Map.make_header(0x1f))
                .and_then(move |_| {
                    Ok(CborMapSerializer(CborCompoundSerializer {
                        s: self,
                        needs_break: true,
                    }))
                }),
        }
    }

    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Err>
    {
        self.write_u8(MajorType::FloatingPoint.make_header(F32))?;
        self.write_f32(value)
    }

    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Err>
    {
        self.write_u8(MajorType::FloatingPoint.make_header(F64))?;
        self.write_f64(value)
    }

    fn serialize_null(self) -> Result<Self::Ok, Self::Err>
    {
        self.write_u8(MajorType::Special.make_header(NULL))
    }

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Err>
    {
        let value = if value { TRUE } else { FALSE };
        self.write_u8(MajorType::Special.make_header(value))
    }
}

#[derive(Debug)]
enum CborDeserializeError
{
    UnexpectedEof,
    ReadError,
    CorruptedHeader,
    UnexpectedType,
    IntegerOverflow,
    VisitorError,
    Utf8Error,
}

struct CborDeserializer<R>
{
    r: PeekOneReader<R>,
}

impl<R: Read> CborDeserializer<R>
{
    #[inline]
    fn read_byte(&mut self) -> Result<u8, CborDeserializeError>
    {
        let mut b = [0u8; 1];
        self.read_bytes(&mut b)?;
        return Ok(b[0]);
    }

    #[inline]
    fn peek_byte(&mut self) -> Result<u8, CborDeserializeError>
    {
        self.r
            .peek_one()
            .as_ref()
            .map(|r| *r)
            .map_err(|e| match e.kind()
            {
                std::io::ErrorKind::UnexpectedEof => CborDeserializeError::UnexpectedEof,
                _ => CborDeserializeError::ReadError,
            })
    }

    #[inline]
    fn read_bytes(&mut self, bytes: &mut [u8]) -> Result<(), CborDeserializeError>
    {
        self.r.read_exact(bytes).map_err(|e| match e.kind()
        {
            std::io::ErrorKind::UnexpectedEof => CborDeserializeError::UnexpectedEof,
            _ => CborDeserializeError::ReadError,
        })
    }

    fn read_integer_from_stream(&mut self, count: usize) -> Result<u64, CborDeserializeError>
    {
        assert!(count <= 8);

        let mut value = 0u64;

        for _ in 0..count
        {
            let byte = self.read_byte()? as u64;
            value = (value << 8) + byte;
        }

        return Ok(value);
    }

    fn read_integer_value(&mut self, payload: u8) -> Result<u64, CborDeserializeError>
    {
        if payload <= 23
        {
            Ok(payload as u64)
        }
        else if payload == 24
        {
            Ok(self.read_integer_from_stream(1)?)
        }
        else if payload == 25
        {
            Ok(self.read_integer_from_stream(2)?)
        }
        else if payload == 26
        {
            Ok(self.read_integer_from_stream(4)?)
        }
        else if payload == 27
        {
            Ok(self.read_integer_from_stream(8)?)
        }
        else
        {
            Err(CborDeserializeError::CorruptedHeader)
        }
    }

    fn read_byte_string<A: Allocator>(
        &mut self,
        vec: &mut Array<u8, A>,
    ) -> Result<(), CborDeserializeError>
    {
        let header = self.read_byte()?;

        let (major_type, payload) =
            MajorType::from_raw(header).ok_or(CborDeserializeError::CorruptedHeader)?;

        if major_type != MajorType::ByteString
        {
            return Err(CborDeserializeError::UnexpectedType);
        }

        let len = self.read_integer_value(payload)? as usize;

        vec.clear();
        vec.resize(len, 0);

        return self.read_bytes(&mut *vec);
    }

    fn read_string<A: Allocator>(&mut self, a: A) -> Result<String<A>, CborDeserializeError>
    {
        let header = self.read_byte()?;

        let (major_type, payload) =
            MajorType::from_raw(header).ok_or(CborDeserializeError::CorruptedHeader)?;

        if major_type != MajorType::TextString
        {
            return Err(CborDeserializeError::UnexpectedType);
        }

        let len = self.read_integer_value(payload)? as usize;

        let mut array = Array::new_with(a);
        array.clear();
        array.resize(len, 0);
        self.read_bytes(&mut *array)?;

        return array
            .try_into()
            .map_err(|_| CborDeserializeError::Utf8Error);
    }

    fn read_integer(&mut self) -> Result<(bool, u64), CborDeserializeError>
    {
        let header = self.read_byte()?;

        let (major_type, payload) =
            MajorType::from_raw(header).ok_or(CborDeserializeError::CorruptedHeader)?;

        let is_negative = match major_type
        {
            MajorType::UnsignedInteger => false,
            MajorType::NegativeInteger => true,
            _ => return Err(CborDeserializeError::UnexpectedType),
        };

        let value = self.read_integer_value(payload)?;

        return Ok((is_negative, value));
    }

    fn read_float(&mut self) -> Result<f64, CborDeserializeError>
    {
        let header = self.read_byte()?;

        let (major_type, payload) =
            MajorType::from_raw(header).ok_or(CborDeserializeError::CorruptedHeader)?;

        match (major_type, payload)
        {
            (MajorType::FloatingPoint, F32) =>
            {
                Ok(
                    unsafe { transmute::<u32, f32>(self.read_integer_from_stream(4)? as u32) }
                        as f64,
                )
            }
            (MajorType::FloatingPoint, F64) =>
            {
                Ok(unsafe { transmute::<u64, f64>(self.read_integer_from_stream(8)?) })
            }
            _ => Err(CborDeserializeError::UnexpectedType),
        }
    }

    fn read_bool(&mut self) -> Result<bool, CborDeserializeError>
    {
        let header = self.read_byte()?;

        let (major_type, payload) =
            MajorType::from_raw(header).ok_or(CborDeserializeError::CorruptedHeader)?;

        match (major_type, payload)
        {
            (MajorType::Special, TRUE) => Ok(true),
            (MajorType::Special, FALSE) => Ok(false),
            _ => Err(CborDeserializeError::UnexpectedType),
        }
    }

    fn read_null(&mut self) -> Result<(), CborDeserializeError>
    {
        let header = self.read_byte()?;

        let (major_type, payload) =
            MajorType::from_raw(header).ok_or(CborDeserializeError::CorruptedHeader)?;

        match (major_type, payload)
        {
            (MajorType::Special, NULL) => Ok(()),
            _ => Err(CborDeserializeError::UnexpectedType),
        }
    }

    fn read_array_length(&mut self) -> Result<Option<usize>, CborDeserializeError>
    {
        let header = self.read_byte()?;

        let (major_type, payload) =
            MajorType::from_raw(header).ok_or(CborDeserializeError::CorruptedHeader)?;

        match (major_type, payload)
        {
            (MajorType::Array, 0x1f) => Ok(None),
            (MajorType::Array, _) => Ok(Some(self.read_integer_value(payload)? as usize)),
            _ => Err(CborDeserializeError::UnexpectedType),
        }
    }

    fn read_break_maybe(&mut self) -> Result<bool, CborDeserializeError>
    {
        let header = self.peek_byte()?;

        let (major_type, payload) = match MajorType::from_raw(header)
        {
            Some(pair) => pair,
            None => return Ok(false),
        };

        match (major_type, payload)
        {
            (MajorType::Special, BREAK) =>
            {
                self.read_byte()?; // Consume the break byte
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

impl<'de, R: Read> Deserializer<'de> for &mut CborDeserializer<R>
{
    type Err = CborDeserializeError;

    fn deserialize_any<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        unimplemented!();
    }

    fn deserialize_u8<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        let (neg, value) = self.read_integer()?;

        if neg || value > core::u8::MAX as u64
        {
            return Err(CborDeserializeError::IntegerOverflow);
        }

        v.accept_u8(value as u8)
            .map_err(|_| CborDeserializeError::VisitorError)
    }

    fn deserialize_u16<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        let (neg, value) = self.read_integer()?;

        if neg || value > core::u16::MAX as u64
        {
            return Err(CborDeserializeError::IntegerOverflow);
        }

        v.accept_u16(value as u16)
            .map_err(|_| CborDeserializeError::VisitorError)
    }

    fn deserialize_u32<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        let (neg, value) = self.read_integer()?;

        if neg || value > core::u32::MAX as u64
        {
            return Err(CborDeserializeError::IntegerOverflow);
        }

        v.accept_u32(value as u32)
            .map_err(|_| CborDeserializeError::VisitorError)
    }

    fn deserialize_u64<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        let (neg, value) = self.read_integer()?;

        if neg || value > core::u64::MAX
        {
            return Err(CborDeserializeError::IntegerOverflow);
        }

        v.accept_u64(value)
            .map_err(|_| CborDeserializeError::VisitorError)
    }

    fn deserialize_i8<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        let (neg, value) = self.read_integer()?;
        if value > core::i8::MAX as u64
        {
            return Err(CborDeserializeError::IntegerOverflow);
        }

        let value = match neg
        {
            false => value as i64,
            true => -1 - value as i64,
        };

        v.accept_i8(value as i8)
            .map_err(|_| CborDeserializeError::VisitorError)
    }

    fn deserialize_i16<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        let (neg, value) = self.read_integer()?;
        if value > core::i16::MAX as u64
        {
            return Err(CborDeserializeError::IntegerOverflow);
        }

        let value = match neg
        {
            false => value as i64,
            true => -1 - value as i64,
        };

        v.accept_i16(value as i16)
            .map_err(|_| CborDeserializeError::VisitorError)
    }

    fn deserialize_i32<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        let (neg, value) = self.read_integer()?;
        if value > core::i32::MAX as u64
        {
            return Err(CborDeserializeError::IntegerOverflow);
        }

        let value = match neg
        {
            false => value as i64,
            true => -1 - value as i64,
        };

        v.accept_i32(value as i32)
            .map_err(|_| CborDeserializeError::VisitorError)
    }

    fn deserialize_i64<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        let (neg, value) = self.read_integer()?;
        if value > core::i64::MAX as u64
        {
            return Err(CborDeserializeError::IntegerOverflow);
        }

        let value = match neg
        {
            false => value as i64,
            true => -1 - value as i64,
        };

        v.accept_i64(value)
            .map_err(|_| CborDeserializeError::VisitorError)
    }

    fn deserialize_f32<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        let value = self.read_float()?;
        v.accept_f32(value as f32)
            .map_err(|_| CborDeserializeError::VisitorError)
    }

    fn deserialize_f64<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        let value = self.read_float()?;
        v.accept_f64(value as f64)
            .map_err(|_| CborDeserializeError::VisitorError)
    }

    fn deserialize_bool<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        let value = self.read_bool()?;
        v.accept_bool(value)
            .map_err(|_| CborDeserializeError::VisitorError)
    }

    fn deserialize_str<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        let string = self.read_string(GlobalAllocator)?;
        v.accept_str(&string)
            .map_err(|_| CborDeserializeError::VisitorError)
    }

    fn deserialize_string<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        let string = self.read_string(GlobalAllocator)?;
        v.accept_string(string)
            .map_err(|_| CborDeserializeError::VisitorError)
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        let mut array = Array::new();
        self.read_byte_string(&mut array)?;
        return v
            .accept_bytes(&array)
            .map_err(|_| CborDeserializeError::VisitorError);
    }

    fn deserialize_bytes_array<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        let mut array = Array::new();
        self.read_byte_string(&mut array)?;
        return v
            .accept_bytes_array(array)
            .map_err(|_| CborDeserializeError::VisitorError);
    }

    fn deserialize_null<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        self.read_null()?;
        v.accept_null()
            .map_err(|_| CborDeserializeError::VisitorError)
    }

    fn deserialize_array<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        let array_len = self.read_array_length()?;

        let mut accessor = CborDeserializerArrayAccessor {
            de: self,
            size: array_len,
        };

        v.accept_array(accessor)
            .map_err(|_| CborDeserializeError::VisitorError)
    }

    fn deserialize_map<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        unimplemented!();
    }
}

struct CborDeserializerArrayAccessor<'a, R>
{
    de: &'a mut CborDeserializer<R>,
    size: Option<usize>,
}

impl<'a, 'de, R: Read> ArrayAccessor<'de> for CborDeserializerArrayAccessor<'a, R>
{
    type Err = CborDeserializeError;

    fn size_hint(&self) -> Option<usize>
    {
        self.size
    }

    fn next_element<T>(&mut self) -> Result<Option<T>, Self::Err>
    where
        T: Deserialize<'de>,
    {
        if self.size.is_some()
        {
            if self.size.unwrap() > 0
            {
                self.size.replace(self.size.unwrap() - 1);
                T::deserialize(&mut *self.de).map(|x| Some(x))
            }
            else
            {
                Ok(None)
            }
        }
        else
        {
            match self.de.read_break_maybe()?
            {
                true => Ok(None),
                false => T::deserialize(&mut *self.de).map(|x| Some(x)),
            }
        }
    }
}

#[test]
fn deserialize()
{
    macro_rules! check_value {
        ($value:expr, $type:ident, $serialized:expr) => {
            let mut de = CborDeserializer {
                r: PeekOneReader::new(Cursor::new($serialized)),
            };
            let value = $type::deserialize(&mut de).unwrap();
            assert_eq!(value, $value);
        };
    }

    check_value!(0u32, u32, [0x00]);
    check_value!(0u8, u8, [0x00]);
    check_value!(0i8, i8, [0x00]);
    check_value!(1u32, u32, [0x01]);
    check_value!(10u32, u32, [0x0a]);
    check_value!(23i32, i32, [0x17]);
    check_value!(24i32, i32, [0x18, 0x18]);
    check_value!(25u32, u32, [0x18, 0x19]);
    check_value!(100u32, u32, [0x18, 0x64]);
    check_value!(1000i64, i64, [0x19, 0x03, 0xe8]);
    check_value!(1000000u32, u32, [0x1a, 0x00, 0x0f, 0x42, 0x40]);
    check_value!(
        1000000000000u64,
        u64,
        [0x1b, 0x00, 0x00, 0x00, 0xe8, 0xd4, 0xa5, 0x10, 0x00]
    );
    check_value!(
        18446744073709551615u64,
        u64,
        [0x1b, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
    );
    check_value!(-1i8, i8, [0x20]);
    check_value!(-10i8, i8, [0x29]);
    check_value!(-100i8, i8, [0x38, 0x63]);
    check_value!(-1000i32, i32, [0x39, 0x03, 0xe7]);
    check_value!(-543514321i64, i64, [0x3A, 0x20, 0x65, 0x5E, 0xD0]);
    check_value!(true, bool, [0xf5]);
    check_value!(false, bool, [0xf4]);
    check_value!(0.0f32, f32, [250, 0, 0, 0, 0]);
    check_value!(1.0f64, f64, [251, 63, 240, 0, 0, 0, 0, 0, 0]);

    struct MyVisitorError;
    impl VisitorError for MyVisitorError
    {
        fn unexpected_type(t: VisitorType) -> Self
        {
            panic!();
        }
    }

    {
        struct BytesCheck;

        impl<'de> Visitor<'de> for BytesCheck
        {
            type Value = ();
            type Err = MyVisitorError;

            fn accept_bytes(self, bytes: &[u8]) -> Result<Self::Value, Self::Err>
            {
                assert_eq!(bytes, [1, 5, 10, 16, 7]);
                Ok(())
            }
        }

        let mut de = CborDeserializer {
            r: PeekOneReader::new(Cursor::new(&[69, 1, 5, 10, 16, 7])),
        };

        de.deserialize_bytes(BytesCheck).unwrap();
    }

    macro_rules! check_string {
        ($reference:expr, $bytes:expr) => {{
            struct StringCheck;

            impl<'de> Visitor<'de> for StringCheck
            {
                type Value = ();
                type Err = MyVisitorError;

                fn accept_str(self, bytes: &str) -> Result<Self::Value, Self::Err>
                {
                    assert_eq!(bytes, $reference);
                    Ok(())
                }
            }

            let mut de = CborDeserializer {
                r: PeekOneReader::new(Cursor::new($bytes)),
            };

            de.deserialize_string(StringCheck).unwrap();
        }};
    }

    check_string!("", &[0x60]);
    check_string!("a", &[0x61, 0x61]);
    check_string!("IETF", &[0x64, 0x49, 0x45, 0x54, 0x46]);
    check_string!("\u{20ac}¢", &[101, 226, 130, 172, 194, 162]);

    {
        struct NullCheck
        {
            found: bool,
        }

        impl<'de> Visitor<'de> for &mut NullCheck
        {
            type Value = ();
            type Err = MyVisitorError;

            fn accept_null(self) -> Result<Self::Value, Self::Err>
            {
                self.found = true;
                Ok(())
            }
        }

        let mut de = CborDeserializer {
            r: PeekOneReader::new(Cursor::new(&[0xf6])),
        };

        let mut checker = NullCheck { found: false };
        de.deserialize_null(&mut checker).unwrap();
        assert!(checker.found);
    }

    macro_rules! check_array {
        ($reference:expr, $bytes:expr) => {{
            struct ArrayVisitor;

            impl<'de> Visitor<'de> for ArrayVisitor
            {
                type Value = Array<u32>;
                type Err = MyVisitorError;

                fn accept_array<A>(self, mut accessor: A) -> Result<Self::Value, Self::Err>
                where
                    A: ArrayAccessor<'de>,
                {
                    let mut array = Array::<u32>::new();

                    while let Some(value) = accessor.next_element().map_err(|_| MyVisitorError)?
                    {
                        array.push(value);
                    }

                    return Ok(array);
                }
            }

            let mut de = CborDeserializer {
                r: PeekOneReader::new(Cursor::new($bytes)),
            };

            let result = de.deserialize_array(ArrayVisitor).unwrap();
            assert_eq!(&*result, $reference);
        }};
    }

    check_array!((&[] as &[u32]), &[0x80, 8, 9, 7]);
    check_array!((&[1u32, 2u32, 3u32] as &[u32]), &[131, 1, 2, 3, 4, 5]);
    check_array!((&[1u32, 2u32, 3u32] as &[u32]), &[159, 1, 2, 3, 255, 4, 5]);

    /*

    {
        v.clear();

        let cursor = Cursor::new(&mut v);
        let mut ser = CborSerializer { write: cursor };

        let mut map = ser.serialize_map(None).unwrap();
        map.serialize_entry(&"a", 1).unwrap();
        map.serialize_entry(2, &"hello").unwrap();
        map.end().unwrap();

        assert_eq!(v, &[191, 97, 97, 1, 2, 101, 104, 101, 108, 108, 111, 255]);
    }
    */
}

#[test]
fn serialize()
{
    let mut v = Vec::<u8>::new();

    macro_rules! check_value {
        ($value:expr, $expected:expr) => {
            v.clear();

            let cursor = Cursor::new(&mut v);
            let mut serializer = CborSerializer { write: cursor };

            $value.serialize(&mut serializer).unwrap();
            assert_eq!(v, $expected);
        };
    }

    check_value!(0u32, &[0x00]);
    check_value!(0u8, &[0x00]);
    check_value!(0i8, &[0x00]);
    check_value!(1u32, &[0x01]);
    check_value!(10u32, &[0x0a]);
    check_value!(23u32, &[0x17]);
    check_value!(24u32, &[0x18, 0x18]);
    check_value!(25u32, &[0x18, 0x19]);
    check_value!(100u32, &[0x18, 0x64]);
    check_value!(1000u32, &[0x19, 0x03, 0xe8]);
    check_value!(1000000u32, &[0x1a, 0x00, 0x0f, 0x42, 0x40]);
    check_value!(
        1000000000000u64,
        &[0x1b, 0x00, 0x00, 0x00, 0xe8, 0xd4, 0xa5, 0x10, 0x00]
    );
    check_value!(
        18446744073709551615u64,
        &[0x1b, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
    );
    check_value!(-1i8, &[0x20]);
    check_value!(-10i8, &[0x29]);
    check_value!(-100i8, &[0x38, 0x63]);
    check_value!(-1000i32, &[0x39, 0x03, 0xe7]);
    check_value!(-543514321i64, &[0x3A, 0x20, 0x65, 0x5E, 0xD0]);
    check_value!("", &[0x60]);
    check_value!("a", &[0x61, 0x61]);
    check_value!("IETF", &[0x64, 0x49, 0x45, 0x54, 0x46]);
    check_value!("\u{20ac}¢", &[101, 226, 130, 172, 194, 162]);
    check_value!((&[] as &[u32])[..], &[0x80]);
    check_value!(&[1, 2, 3][..], &[131, 1, 2, 3]);
    check_value!(true, &[0xf5]);
    check_value!(false, &[0xf4]);
    check_value!(0.0f32, &[250, 0, 0, 0, 0]);
    check_value!(1.0f64, &[251, 63, 240, 0, 0, 0, 0, 0, 0]);

    {
        v.clear();

        let cursor = Cursor::new(&mut v);
        let mut serializer = CborSerializer { write: cursor };

        let mut array_ser = serializer.serialize_array(None).unwrap();
        array_ser.serialize_element(1u32).unwrap();
        array_ser.serialize_element(2u32).unwrap();
        array_ser.serialize_element(3u32).unwrap();
        array_ser.end();

        assert_eq!(v, &[159, 1, 2, 3, 255]);
    }

    {
        v.clear();

        let cursor = Cursor::new(&mut v);
        let mut ser = CborSerializer { write: cursor };

        let mut map = ser.serialize_map(None).unwrap();
        map.serialize_entry(&"a", 1).unwrap();
        map.serialize_entry(2, &"hello").unwrap();
        map.end().unwrap();

        assert_eq!(v, &[191, 97, 97, 1, 2, 101, 104, 101, 108, 108, 111, 255]);
    }

    {
        v.clear();

        let cursor = Cursor::new(&mut v);
        let mut ser = CborSerializer { write: cursor };

        let mut map = ser.serialize_null().unwrap();
        assert_eq!(v, &[0xf6]);
    }

    {
        v.clear();

        let cursor = Cursor::new(&mut v);
        let mut ser = CborSerializer { write: cursor };

        let mut map = ser.serialize_bytes(&[1, 5, 10, 16, 7]).unwrap();
        assert_eq!(v, &[69, 1, 5, 10, 16, 7]);
    }
}

fn main() {}
