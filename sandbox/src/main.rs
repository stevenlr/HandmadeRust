use fnd::alloc::{set_global_allocator, SystemAllocator};

use std::io::Write;

trait Serializer
{
    type Ok;
    type Err;

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Err>;
    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Err>;
    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Err>;
    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Err>;
    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Err>;
    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Err>;
    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Err>;
    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Err>;
    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Err>;
    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Err>;
}

trait Serialize: Sized
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>;
}

impl Serialize for u8
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_u8(*self)
    }
}

impl Serialize for u16
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_u16(*self)
    }
}

impl Serialize for u32
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_u32(*self)
    }
}

impl Serialize for u64
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_u64(*self)
    }
}

impl Serialize for i8
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_i8(*self)
    }
}

impl Serialize for i16
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_i16(*self)
    }
}

impl Serialize for i32
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_i32(*self)
    }
}

impl Serialize for i64
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_i64(*self)
    }
}

impl Serialize for &[u8]
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_bytes(self)
    }
}

impl Serialize for &str
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_str(self)
    }
}

struct CborSerializer<'a>
{
    write: &'a mut Write,
}

enum MajorType
{
    UnsignedInteger,
    NegativeInteger,
    ByteString,
    TextString,
    Array,
    Map,
    FloatingPoint,
}

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
        }
    }

    #[inline]
    fn make_header(self, additional_data: u8) -> u8
    {
        debug_assert!(additional_data < 32);
        (self.as_int() << 5) | additional_data
    }
}

impl<'a> CborSerializer<'a>
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
                .and_then(|_| {
                    self.write_u8(value as u8)
                })
        }
        else if value <= core::u16::MAX as u64
        {
            self.write_u8(major_type.make_header(25))
                .and_then(|_| {
                    self.write_u16(value as u16)
                })
        }
        else if value <= core::u32::MAX as u64
        {
            self.write_u8(major_type.make_header(26))
                .and_then(|_| {
                    self.write_u32(value as u32)
                })
        }
        else if value <= core::u64::MAX as u64
        {
            self.write_u8(major_type.make_header(27))
                .and_then(|_| {
                    self.write_u64(value as u64)
                })
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
        self.write
            .write_all(data)
            .map(|_| ())
            .map_err(|_| ())
    }
}

impl<'a> Serializer for &mut CborSerializer<'a>
{
    type Ok = ();
    type Err = ();

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
            .and_then(|_| {
                self.write_bytes(value)
            })
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Err>
    {
        self.write_header_with_value(MajorType::TextString, value.as_bytes().len() as u64)
            .and_then(|_| {
                self.write_bytes(value.as_bytes())
            })
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

    (&[1, 3, 4, 5][..]).serialize(&mut serializer).unwrap();
    (&[][..]).serialize(&mut serializer).unwrap();

    "".serialize(&mut serializer).unwrap();
    "a".serialize(&mut serializer).unwrap();
    "IETF".serialize(&mut serializer).unwrap();
    "\u{20ac}¢".serialize(&mut serializer).unwrap();

    file.flush().unwrap();
}
