use crate::{alloc::Allocator, containers::Array};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Error
{
    InvalidOpenOptions,
    CannotOpen,
    CannotSeek,
    CannotRead,
    CannotWrite,
    CannotFlush,
    BufferTooLarge,
    UnexpectedEof,
}

pub type Result<T> = core::result::Result<T, Error>;

pub trait Read
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()>
    {
        while !buf.is_empty()
        {
            match self.read(buf)
            {
                Ok(0) => break,
                Ok(n) => buf = &mut buf[n..],
                Err(e) => return Err(e),
            }
        }

        if !buf.is_empty()
        {
            Err(Error::UnexpectedEof)
        }
        else
        {
            Ok(())
        }
    }
}

pub trait Write
{
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, mut buf: &[u8]) -> Result<()>
    {
        while !buf.is_empty()
        {
            match self.write(buf)
            {
                Ok(0) => return Err(Error::CannotWrite),
                Ok(n) => buf = &buf[n..],
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}

pub enum SeekOrigin
{
    Start(isize),
    End(isize),
    Current(isize),
}

pub trait Seek
{
    fn seek(&mut self, pos: SeekOrigin) -> Result<usize>;

    fn stream_position(&mut self) -> Result<usize>
    {
        self.seek(SeekOrigin::Current(0))
    }

    fn stream_len(&mut self) -> Result<usize>
    {
        let position = self.stream_position()?;
        let end_pos = self.seek(SeekOrigin::End(0))?;
        self.seek(SeekOrigin::Start(position as isize))?;
        Ok(end_pos)
    }
}

impl Read for &[u8]
{
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>
    {
        let count = buf.len().min(self.len());
        let (to_copy, remaining) = self.split_at(count);

        buf[..count].copy_from_slice(to_copy);

        *self = remaining;
        Ok(count)
    }
}

impl<A: Allocator> Write for &mut Array<u8, A>
{
    fn write(&mut self, buf: &[u8]) -> Result<usize>
    {
        (*self).write(buf)
    }

    fn flush(&mut self) -> Result<()>
    {
        (*self).flush()
    }
}

impl<A: Allocator> Write for Array<u8, A>
{
    fn write(&mut self, buf: &[u8]) -> Result<usize>
    {
        let old_len = self.len();

        self.resize(self.len() + buf.len(), 0);
        (&mut self[old_len..]).copy_from_slice(buf);

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()>
    {
        Ok(())
    }
}

pub struct BitReader<R>
{
    r: R,
    buffer: u64,
    buffer_size: usize,
}

impl<R> BitReader<R>
where
    R: Read,
{
    pub fn new(r: R) -> Self
    {
        Self {
            r,
            buffer: 0,
            buffer_size: 0,
        }
    }

    fn buffer_bits(&mut self, bit_count: usize)
    {
        debug_assert!(bit_count <= 32);

        while self.buffer_size < bit_count
        {
            let mut byte = [0u8];
            match self.r.read(&mut byte)
            {
                Ok(1) =>
                {
                    self.buffer = self.buffer | ((byte[0] as u64) << self.buffer_size);
                    self.buffer_size += 8;
                }
                _ =>
                {
                    break;
                }
            }
        }
    }

    pub fn peek(&mut self, bit_count: usize) -> u32
    {
        assert!(bit_count <= 32);
        self.buffer_bits(bit_count);
        (self.buffer & ((1 << bit_count) - 1)) as u32
    }

    pub fn consume(&mut self, bit_count: usize) -> u32
    {
        let value = self.peek(bit_count);
        self.buffer_size = self.buffer_size.saturating_sub(bit_count);
        self.buffer >>= bit_count;
        return value;
    }

    pub fn skip_to_next_byte(&mut self)
    {
        self.consume(self.buffer_size % 8);
    }
}

pub struct PeekOne<R>
{
    r: R,
    peek: Option<Result<u8>>,
}

impl<R: Read> PeekOne<R>
{
    pub fn new(r: R) -> Self
    {
        Self { r, peek: None }
    }

    pub fn peek_one(&mut self) -> Result<u8>
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

        return self.peek.as_ref().cloned().unwrap();
    }
}

impl<R: Read> Read for PeekOne<R>
{
    fn read(&mut self, mut buf: &mut [u8]) -> Result<usize>
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
mod tests
{
    use super::*;

    #[test]
    fn bit_reader()
    {
        let data: &[u8] = &[
            0b0000_0001,
            0b0010_0011,
            0b0100_0101,
            0b0110_0111,
            0b1000_1001,
            0b1010_1011,
            0b1100_1101,
            0b1110_1111,
        ];

        let mut bit_reader = BitReader::new(data);

        assert_eq!(bit_reader.consume(6), 0b00_0001);
        assert_eq!(bit_reader.consume(6), 0b0011_00);
        assert_eq!(bit_reader.consume(12), 0b0100_0101_0010);
        assert_eq!(bit_reader.consume(4), 0b0111);
        assert_eq!(bit_reader.peek(1), 0b0);
        assert_eq!(bit_reader.peek(2), 0b10);
        assert_eq!(bit_reader.peek(3), 0b110);
        assert_eq!(bit_reader.peek(4), 0b0110);
        assert_eq!(bit_reader.peek(5), 0b1_0110);
        assert_eq!(bit_reader.peek(6), 0b01_0110);
        assert_eq!(bit_reader.peek(7), 0b001_0110);
        assert_eq!(bit_reader.peek(8), 0b1001_0110);
        assert_eq!(bit_reader.consume(3), 0b110);
        bit_reader.skip_to_next_byte();
        assert_eq!(bit_reader.consume(28), 0b1111_1100_1101_1010_1011_1000_1001);
        assert_eq!(bit_reader.peek(8), 0b1110);
        assert_eq!(bit_reader.peek(4), 0b1110);
        assert_eq!(bit_reader.peek(32), 0b1110);
        assert_eq!(bit_reader.consume(32), 0b1110);
    }

    #[test]
    fn read_u8_slice()
    {
        let mut data: &[u8] = &[1, 2, 3, 4, 5];
        let output = &mut [0, 0];

        assert_eq!(data.read(output), Ok(2));
        assert_eq!(output[0], 1);
        assert_eq!(output[1], 2);

        assert_eq!(data.read(output), Ok(2));
        assert_eq!(output[0], 3);
        assert_eq!(output[1], 4);

        assert_eq!(data.read(output), Ok(1));
        assert_eq!(output[0], 5);

        assert_eq!(data.read(output), Ok(0));
    }

    #[test]
    fn write_u8_array()
    {
        let mut array = Array::<u8>::new();
        array.write_all(&[1, 2, 3, 4, 5, 6]).unwrap();
        assert_eq!(&array[..], &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn peek_one()
    {
        let mut buffer = [0u8; 8];
        let reader: &[u8] = &[1, 2, 3, 4, 5, 6];
        let mut reader = PeekOne::new(reader);

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
}
