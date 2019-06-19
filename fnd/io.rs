pub enum Error
{
    InvalidOpenOptions,
}

pub type Result<T> = core::result::Result<T, Error>;

pub trait Read
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}

pub trait Write
{
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self);
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
