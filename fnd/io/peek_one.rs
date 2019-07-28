use super::{Read, Result};

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
