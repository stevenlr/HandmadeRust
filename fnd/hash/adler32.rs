pub struct Adler32
{
    a: u32,
    b: u32,
}

impl Adler32
{
    pub fn new() -> Self
    {
        Self { a: 1, b: 0 }
    }

    #[inline]
    pub fn eat(&mut self, c: u8)
    {
        self.a = (self.a + c as u32) % 65521;
        self.b = (self.b + self.a) % 65521;
    }

    #[inline]
    pub fn eat_slice(&mut self, s: &[u8])
    {
        for c in s
        {
            self.eat(*c);
        }
    }

    #[inline]
    pub fn finish(self) -> u32
    {
        (self.b << 16) + self.a
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn simple()
    {
        let mut a = Adler32::new();
        a.eat_slice(b"Wikipedia");
        assert_eq!(a.finish(), 0x11E60398);
    }
}
