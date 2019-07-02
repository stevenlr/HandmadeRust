#[derive(Debug, PartialEq)]
pub enum Error
{
    InvalidOpenOptions,
    CannotOpen,
    CannotSeek,
    CannotRead,
    CannotWrite,
    CannotFlush,
    BufferTooLarge,
}

pub type Result<T> = core::result::Result<T, Error>;

pub trait Read
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}

pub trait Write
{
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
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
}
