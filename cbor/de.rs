use super::*;

use core::convert::TryInto;

use fnd::{
    alloc::{Allocator, GlobalAllocator},
    containers::{Array, String},
    io::{PeekOne, Read},
    serde::*,
};

#[derive(Debug)]
pub enum CborDeserializeError
{
    UnexpectedEof,
    ReadError,
    CorruptedHeader,
    UnexpectedType,
    IntegerOverflow,
    VisitorError,
    Utf8Error,
}

pub struct CborDeserializer<R>
{
    r: PeekOne<R>,
}

impl<R: Read> CborDeserializer<R>
{
    pub fn new(r: R) -> Self
    {
        Self { r: PeekOne::new(r) }
    }

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
        self.r.peek_one().as_ref().map(|r| *r).map_err(|e| match e
        {
            fnd::io::Error::UnexpectedEof => CborDeserializeError::UnexpectedEof,
            _ => CborDeserializeError::ReadError,
        })
    }

    #[inline]
    fn read_bytes(&mut self, bytes: &mut [u8]) -> Result<(), CborDeserializeError>
    {
        self.r.read_exact(bytes).map_err(|e| match e
        {
            fnd::io::Error::UnexpectedEof => CborDeserializeError::UnexpectedEof,
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

    fn read_f32(&mut self) -> Result<f32, CborDeserializeError>
    {
        let header = self.read_byte()?;

        let (major_type, payload) =
            MajorType::from_raw(header).ok_or(CborDeserializeError::CorruptedHeader)?;

        match (major_type, payload)
        {
            (MajorType::FloatingPoint, F32) =>
            {
                Ok(f32::from_bits(self.read_integer_from_stream(4)? as u32))
            }
            (MajorType::FloatingPoint, F64) =>
            {
                Ok(f64::from_bits(self.read_integer_from_stream(8)?) as f32)
            }
            _ => Err(CborDeserializeError::UnexpectedType),
        }
    }

    fn read_f64(&mut self) -> Result<f64, CborDeserializeError>
    {
        let header = self.read_byte()?;

        let (major_type, payload) =
            MajorType::from_raw(header).ok_or(CborDeserializeError::CorruptedHeader)?;

        match (major_type, payload)
        {
            (MajorType::FloatingPoint, F32) =>
            {
                Ok(f32::from_bits(self.read_integer_from_stream(4)? as u32) as f64)
            }
            (MajorType::FloatingPoint, F64) =>
            {
                Ok(f64::from_bits(self.read_integer_from_stream(8)?))
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

    fn read_map_length(&mut self) -> Result<Option<usize>, CborDeserializeError>
    {
        let header = self.read_byte()?;

        let (major_type, payload) =
            MajorType::from_raw(header).ok_or(CborDeserializeError::CorruptedHeader)?;

        match (major_type, payload)
        {
            (MajorType::Map, 0x1f) => Ok(None),
            (MajorType::Map, _) => Ok(Some(self.read_integer_value(payload)? as usize)),
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
        let value = self.read_f32()?;
        v.accept_f32(value)
            .map_err(|_| CborDeserializeError::VisitorError)
    }

    fn deserialize_f64<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        let value = self.read_f64()?;
        v.accept_f64(value)
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

        let accessor = CborDeserializerCompoundAccessor {
            de: self,
            size: array_len,
        };

        v.accept_array(accessor)
            .map_err(|_| CborDeserializeError::VisitorError)
    }

    fn deserialize_map<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>
    {
        let map_len = self.read_map_length()?;

        let accessor = CborDeserializerCompoundAccessor {
            de: self,
            size: map_len,
        };

        v.accept_map(accessor)
            .map_err(|_| CborDeserializeError::VisitorError)
    }
}

pub struct CborDeserializerCompoundAccessor<'a, R>
{
    de: &'a mut CborDeserializer<R>,
    size: Option<usize>,
}

impl<'a, 'de, R: Read> ArrayAccessor<'de> for CborDeserializerCompoundAccessor<'a, R>
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

impl<'a, 'de, R: Read> MapAccessor<'de> for CborDeserializerCompoundAccessor<'a, R>
{
    type Err = CborDeserializeError;

    fn size_hint(&self) -> Option<usize>
    {
        self.size
    }

    fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Err>
    where
        K: Deserialize<'de>,
        V: Deserialize<'de>,
    {
        if self.size.is_some()
        {
            if self.size.unwrap() > 0
            {
                self.size.replace(self.size.unwrap() - 1);
                let key = K::deserialize(&mut *self.de)?;
                let value = V::deserialize(&mut *self.de)?;
                Ok(Some((key, value)))
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
                false =>
                {
                    let key = K::deserialize(&mut *self.de)?;
                    let value = V::deserialize(&mut *self.de)?;
                    Ok(Some((key, value)))
                }
            }
        }
    }
}
