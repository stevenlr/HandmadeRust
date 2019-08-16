use core::mem::{align_of, size_of};

#[derive(Copy, Clone)]
pub struct Layout
{
    pub size:  usize,
    pub align: usize,
}

impl Layout
{
    pub fn new(size: usize) -> Self
    {
        Self { size, align: 4 }
    }

    pub fn from_type<T>() -> Self
    {
        Self {
            size:  size_of::<T>(),
            align: align_of::<T>(),
        }
    }

    pub fn from_type_array<T>(length: usize) -> Self
    {
        Self {
            size:  size_of::<T>() * length,
            align: align_of::<T>(),
        }
    }

    pub fn align_up(&self, i: usize) -> usize
    {
        let p = i + self.align - 1;
        return p - (p % self.align);
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn layout_from_type()
    {
        let l1 = Layout::from_type::<i64>();
        assert!(l1.size == 8);
        assert!(l1.align == 8);

        let l2 = Layout::from_type::<u8>();
        assert!(l2.size == 1);
        assert!(l2.align == 1);
    }

    #[test]
    fn align_up()
    {
        let layout = Layout::from_type::<i32>();
        assert!(layout.align_up(0) == 0);
        assert!(layout.align_up(1) == 4);
        assert!(layout.align_up(2) == 4);
        assert!(layout.align_up(3) == 4);
        assert!(layout.align_up(4) == 4);
        assert!(layout.align_up(5) == 8);
        assert!(layout.align_up(6) == 8);
    }
}
