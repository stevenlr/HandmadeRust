use fnd::alloc::{set_global_allocator, SystemAllocator};

use core::mem::transmute;
use std::io::Write;

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

impl<'se, W: 'se> Serializer<'se> for CborSerializer<W>
where
    W: Write,
{
    type Ok = ();
    type Err = ();
    type ArraySerializer = CborArraySerializer<'se, W>;
    type MapSerializer = CborMapSerializer<'se, W>;

    fn serialize_u8(&'se mut self, value: u8) -> Result<Self::Ok, Self::Err>
    {
        self.write_header_with_value(MajorType::UnsignedInteger, value as u64)
    }

    fn serialize_i8(&'se mut self, value: i8) -> Result<Self::Ok, Self::Err>
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

    fn serialize_u16(&'se mut self, value: u16) -> Result<Self::Ok, Self::Err>
    {
        self.write_header_with_value(MajorType::UnsignedInteger, value as u64)
    }

    fn serialize_i16(&'se mut self, value: i16) -> Result<Self::Ok, Self::Err>
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

    fn serialize_u32(&'se mut self, value: u32) -> Result<Self::Ok, Self::Err>
    {
        self.write_header_with_value(MajorType::UnsignedInteger, value as u64)
    }

    fn serialize_i32(&'se mut self, value: i32) -> Result<Self::Ok, Self::Err>
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

    fn serialize_u64(&'se mut self, value: u64) -> Result<Self::Ok, Self::Err>
    {
        self.write_header_with_value(MajorType::UnsignedInteger, value)
    }

    fn serialize_i64(&'se mut self, value: i64) -> Result<Self::Ok, Self::Err>
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

    fn serialize_bytes(&'se mut self, value: &[u8]) -> Result<Self::Ok, Self::Err>
    {
        self.write_header_with_value(MajorType::ByteString, value.len() as u64)
            .and_then(|_| self.write_bytes(value))
    }

    fn serialize_str(&'se mut self, value: &str) -> Result<Self::Ok, Self::Err>
    {
        self.write_header_with_value(MajorType::TextString, value.as_bytes().len() as u64)
            .and_then(|_| self.write_bytes(value.as_bytes()))
    }

    fn serialize_array(
        &'se mut self,
        len: Option<usize>,
    ) -> Result<CborArraySerializer<'se, W>, Self::Err>
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

    fn serialize_map(
        &'se mut self,
        len: Option<usize>,
    ) -> Result<CborMapSerializer<'se, W>, Self::Err>
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

    fn serialize_f32(&'se mut self, value: f32) -> Result<Self::Ok, Self::Err>
    {
        self.write_u8(MajorType::FloatingPoint.make_header(F32))?;
        self.write_f32(value)
    }

    fn serialize_f64(&'se mut self, value: f64) -> Result<Self::Ok, Self::Err>
    {
        self.write_u8(MajorType::FloatingPoint.make_header(F64))?;
        self.write_f64(value)
    }

    fn serialize_null(&'se mut self) -> Result<Self::Ok, Self::Err>
    {
        self.write_u8(MajorType::Special.make_header(NULL))
    }

    fn serialize_bool(&'se mut self, value: bool) -> Result<Self::Ok, Self::Err>
    {
        let value = if value { TRUE } else { FALSE };
        self.write_u8(MajorType::Special.make_header(value))
    }
}

fn init_global_allocator()
{
    static mut ALLOCATOR: Option<SystemAllocator> = None;
    static mut ALLOCATOR_REF: Option<&SystemAllocator> = None;

    unsafe {
        ALLOCATOR = Some(SystemAllocator::default());
        ALLOCATOR_REF = Some(ALLOCATOR.as_ref().unwrap());
        set_global_allocator(ALLOCATOR_REF.as_mut().unwrap());
    }
}

fn main()
{
    init_global_allocator();

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("test.bin")
        .unwrap();

    let mut serializer = CborSerializer { write: &mut file };

    0u32.serialize(&mut serializer).unwrap();
    1u32.serialize(&mut serializer).unwrap();
    10u32.serialize(&mut serializer).unwrap();
    23u32.serialize(&mut serializer).unwrap();
    24u32.serialize(&mut serializer).unwrap();
    25u32.serialize(&mut serializer).unwrap();
    100u32.serialize(&mut serializer).unwrap();
    1000u32.serialize(&mut serializer).unwrap();
    1000000u32.serialize(&mut serializer).unwrap();
    1000000000000u64.serialize(&mut serializer).unwrap();
    18446744073709551615u64.serialize(&mut serializer).unwrap();
    (-543514321i64).serialize(&mut serializer).unwrap();
    (-1i8).serialize(&mut serializer).unwrap();
    (-10i8).serialize(&mut serializer).unwrap();
    (-100i8).serialize(&mut serializer).unwrap();
    (-1000i32).serialize(&mut serializer).unwrap();
    (0i64).serialize(&mut serializer).unwrap();

    "".serialize(&mut serializer).unwrap();
    "a".serialize(&mut serializer).unwrap();
    "IETF".serialize(&mut serializer).unwrap();
    "\u{20ac}¢".serialize(&mut serializer).unwrap();

    ((&[] as &[u32])[..]).serialize(&mut serializer).unwrap();
    (&[1, 2, 3][..]).serialize(&mut serializer).unwrap();

    {
        let mut map = serializer.serialize_map(None).unwrap();
        map.serialize_entry(&"a", 1).unwrap();
        map.serialize_entry(2, &"hello").unwrap();
        map.end().unwrap();
    }

    true.serialize(&mut serializer).unwrap();
    false.serialize(&mut serializer).unwrap();
    0.0f32.serialize(&mut serializer).unwrap();
    1.0f64.serialize(&mut serializer).unwrap();

    file.flush().unwrap();
}