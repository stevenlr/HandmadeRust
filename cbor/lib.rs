#![no_std]

mod de;
mod ser;

pub use de::*;
pub use ser::*;

#[cfg(test)]
mod tests;

#[derive(PartialEq)]
pub(crate) enum MajorType
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

pub(crate) const BREAK: u8 = 31;
pub(crate) const TRUE: u8 = 21;
pub(crate) const FALSE: u8 = 20;
pub(crate) const NULL: u8 = 22;
pub(crate) const F64: u8 = 27;
pub(crate) const F32: u8 = 26;

impl MajorType
{
    pub(crate) fn from_raw(raw: u8) -> Option<(MajorType, u8)>
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

    pub(crate) fn as_int(self) -> u8
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
    pub(crate) fn make_header(self, additional_data: u8) -> u8
    {
        debug_assert!(additional_data < 32);
        (self.as_int() << 5) | additional_data
    }
}
