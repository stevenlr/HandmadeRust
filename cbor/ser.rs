use super::*;

use fnd::{io::Write, serde::*};

pub struct CborSerializer<W>
{
    write: W,
}

impl<W> CborSerializer<W>
{
    pub fn new(write: W) -> Self
    {
        Self { write }
    }
}

struct CborCompoundSerializer<'a, W>
{
    s:           &'a mut CborSerializer<W>,
    needs_break: bool,
}

pub struct CborArraySerializer<'a, W>(CborCompoundSerializer<'a, W>);
pub struct CborMapSerializer<'a, W>(CborCompoundSerializer<'a, W>);

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
        self.write_u32(data.to_bits())
    }

    #[inline]
    fn write_f64(&mut self, data: f64) -> Result<(), ()>
    {
        self.write_u64(data.to_bits())
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
                        s:           self,
                        needs_break: false,
                    }))
                }),
            None => self
                .write_u8(MajorType::Array.make_header(0x1f))
                .and_then(move |_| {
                    Ok(CborArraySerializer(CborCompoundSerializer {
                        s:           self,
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
                        s:           self,
                        needs_break: false,
                    }))
                }),
            None => self
                .write_u8(MajorType::Map.make_header(0x1f))
                .and_then(move |_| {
                    Ok(CborMapSerializer(CborCompoundSerializer {
                        s:           self,
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
