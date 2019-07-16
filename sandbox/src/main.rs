#![allow(unused)]

use core::mem::transmute;
use std::{
    io::{Bytes, Cursor, Read, Write},
    vec::Vec,
};

mod impls;
mod traits;

use traits::*;

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
}

struct CborDeserializer<R>
{
    r: Bytes<R>,
}

impl<R: Read> CborDeserializer<R>
{
    #[inline]
    fn read_byte(&mut self) -> Result<u8, CborDeserializeError>
    {
        self.r
            .next()
            .ok_or(CborDeserializeError::UnexpectedEof)?
            .map_err(|_| CborDeserializeError::ReadError)
    }

    #[inline]
    fn read_bytes(&mut self, bytes: &mut [u8]) -> Result<(), CborDeserializeError>
    {
        for b in bytes.iter_mut()
        {
            *b = self.read_byte()?;
        }

        Ok(())
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

        let value = if payload <= 23
        {
            payload as u64
        }
        else if payload == 24
        {
            self.read_integer_from_stream(1)?
        }
        else if payload == 25
        {
            self.read_integer_from_stream(2)?
        }
        else if payload == 26
        {
            self.read_integer_from_stream(4)?
        }
        else if payload == 27
        {
            self.read_integer_from_stream(8)?
        }
        else
        {
            return Err(CborDeserializeError::CorruptedHeader);
        };

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
                let mut bytes = [0u8; 4];
                self.read_bytes(&mut bytes[..])?;
                Ok(unsafe { transmute::<u32, f32>(u32::from_be_bytes(bytes)) } as f64)
            }
            (MajorType::FloatingPoint, F64) =>
            {
                let mut bytes = [0u8; 8];
                self.read_bytes(&mut bytes[..])?;
                Ok(unsafe { transmute::<u64, f64>(u64::from_be_bytes(bytes)) } as f64)
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
        unimplemented!();
    }

    fn deserialize_string<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        unimplemented!();
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        unimplemented!();
    }

    fn deserialize_bytes_array<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        unimplemented!();
    }

    fn deserialize_null<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        self.read_null()?;
        v.accept_null()
            .map_err(|_| CborDeserializeError::VisitorError)
    }

    fn deserialize_array<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        unimplemented!();
    }

    fn deserialize_map<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        unimplemented!();
    }
}

#[test]
fn deserialize()
{
    macro_rules! check_value {
        ($value:expr, $type:ident, $serialized:expr) => {
            let mut de = CborDeserializer {
                r: $serialized.bytes(),
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

    /*
    check_value!("", &[0x60]);
    check_value!("a", &[0x61, 0x61]);
    check_value!("IETF", &[0x64, 0x49, 0x45, 0x54, 0x46]);
    check_value!("\u{20ac}¢", &[101, 226, 130, 172, 194, 162]);
    check_value!((&[] as &[u32])[..], &[0x80]);
    check_value!(&[1, 2, 3][..], &[131, 1, 2, 3]);

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
        let mut ser = CborSerializer { write: cursor };

        let mut map = ser.serialize_map(None).unwrap();
        map.serialize_entry(&"a", 1).unwrap();
        map.serialize_entry(2, &"hello").unwrap();
        map.end().unwrap();

        assert_eq!(v, &[191, 97, 97, 1, 2, 101, 104, 101, 108, 108, 111, 255]);
    }
}

fn main() {}
