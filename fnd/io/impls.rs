use super::{Read, Result, Write};
use crate::{alloc::Allocator, containers::Array};

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

#[cfg(test)]
mod tests
{
    use super::*;

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
}
