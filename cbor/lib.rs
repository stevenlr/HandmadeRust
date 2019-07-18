mod de;
mod ser;

pub use de::*;
pub use ser::*;

#[cfg(test)]
mod tests;

use core::{convert::TryInto, mem::transmute};
use fnd::{
    alloc::{Allocator, GlobalAllocator},
    containers::{Array, String},
    serde::*,
};
use std::io::{Read, Write};

pub(crate) struct PeekOneReader<R>
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

#[cfg(test)]
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
