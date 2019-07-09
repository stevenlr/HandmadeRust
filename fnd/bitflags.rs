#[macro_export]
macro_rules! bitflags {
    (
        $vis:vis enum $name:ident: $type:ty {
            $(
                $vname:ident = $value:expr,
            )+
        }
    ) => {
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Hash, PartialOrd)]
        $vis struct $name
        {
            bits: $type,
        }

        impl $name
        {
            $(
                pub const $vname: Self = Self{ bits: $value };
            )+

            #[inline]
            pub fn none() -> Self
            {
                Self {
                    bits: 0,
                }
            }

            #[inline]
            pub fn all() -> Self
            {
                Self {
                    bits: $($value)|+,
                }
            }

            #[inline]
            pub fn as_raw(&self) -> $type
            {
                self.bits
            }

            #[inline]
            pub fn contains(&self, other: Self) -> bool
            {
                (self.bits & other.bits) == other.bits
            }

            #[inline]
            pub fn insert(&mut self, other: Self)
            {
                self.bits |= other.bits;
            }

            #[inline]
            pub fn toggle(&mut self, other: Self)
            {
                self.bits ^= other.bits;
            }

            #[inline]
            pub fn remove(&mut self, other: Self)
            {
                self.bits &= !other.bits;
            }

            #[inline]
            pub fn is_empty(&self) -> bool
            {
                self.bits == 0
            }
        }

        impl Default for $name
        {
            fn default() -> $name
            {
                $name { bits: 0 }
            }
        }

        impl core::convert::TryFrom<$type> for $name
        {
            type Error = ();

            #[inline]
            fn try_from(bits: $type) -> Result<Self, ()>
            {
                if (bits & Self::all().bits) != bits
                {
                    Err(())
                }
                else
                {
                    Ok(Self {
                        bits
                    })
                }
            }
        }

        impl core::fmt::Debug for $name
        {
            #[allow(unused_assignments)]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error>
            {
                let mut first = true;
                f.write_str(stringify!($name))?;
                f.write_str("(")?;

                $(
                    if self.contains(Self::$vname)
                    {
                        if !first
                        {
                            f.write_str(" | ")?;
                        }

                        f.write_str(stringify!($vname))?;
                        first = false;
                    }
                )+

                f.write_str(")")?;
                Ok(())
            }
        }

        impl core::ops::BitOr for $name
        {
            type Output = Self;

            #[inline]
            fn bitor(mut self, other: Self) -> Self
            {
                self.insert(other);
                return self;
            }
        }

        impl core::ops::BitOrAssign for $name
        {
            #[inline]
            fn bitor_assign(&mut self, other: Self)
            {
                self.insert(other);
            }
        }

        impl core::ops::Sub for $name
        {
            type Output = Self;

            #[inline]
            fn sub(mut self, other: Self) -> Self
            {
                self.remove(other);
                return self;
            }
        }

        impl core::ops::SubAssign for $name
        {
            #[inline]
            fn sub_assign(&mut self, other: Self)
            {
                self.remove(other);
            }
        }

        impl core::ops::BitAnd for $name
        {
            type Output = Self;

            #[inline]
            fn bitand(mut self, other: Self) -> Self
            {
                self.bits &= other.bits;
                return self;
            }
        }

        impl core::ops::BitAndAssign for $name
        {
            #[inline]
            fn bitand_assign(&mut self, other: Self)
            {
                self.bits &= other.bits;
            }
        }

        impl core::ops::BitXor for $name
        {
            type Output = Self;

            #[inline]
            fn bitxor(mut self, other: Self) -> Self
            {
                self.toggle(other);
                return self;
            }
        }

        impl core::ops::BitXorAssign for $name
        {
            #[inline]
            fn bitxor_assign(&mut self, other: Self)
            {
                self.toggle(other);
            }
        }
    }
}

#[cfg(test)]
mod tests
{
    use core::convert::TryInto;

    bitflags! {
        enum MyFlags: u32 {
            A = 1,
            B = 2,
            C = 4,
        }
    }

    #[test]
    fn try_from()
    {
        assert_eq!(0u32.try_into(), Ok(MyFlags::none()));
        assert_eq!(1u32.try_into(), Ok(MyFlags::A));
        assert_eq!(2u32.try_into(), Ok(MyFlags::B));
        assert_eq!(5u32.try_into(), Ok(MyFlags::A | MyFlags::C));
        assert_eq!(7u32.try_into(), Ok(MyFlags::all()));
        assert_eq!(8u32.try_into(), Result::<MyFlags, ()>::Err(()));
    }

    #[test]
    fn bitflags()
    {
        assert_eq!(MyFlags::none().as_raw(), 0);
        assert_eq!(MyFlags::all().as_raw(), 7);

        assert!(MyFlags::all().contains(MyFlags::A));
        assert!(MyFlags::all().contains(MyFlags::B));
        assert!(MyFlags::all().contains(MyFlags::C));

        assert!(!MyFlags::none().contains(MyFlags::A));
        assert!(!MyFlags::none().contains(MyFlags::B));
        assert!(!MyFlags::none().contains(MyFlags::C));

        assert!(MyFlags::A.contains(MyFlags::A));
        assert!(!MyFlags::A.contains(MyFlags::B));

        assert!(!MyFlags::B.contains(MyFlags::A));
        assert!(MyFlags::B.contains(MyFlags::B));

        assert!(!MyFlags::A.is_empty());
        assert!(MyFlags::none().is_empty());
    }

    #[test]
    fn remove()
    {
        let mut x = MyFlags::all();

        assert!(x.contains(MyFlags::A));
        assert!(x.contains(MyFlags::B));
        assert!(x.contains(MyFlags::C));

        x.remove(MyFlags::B);
        assert!(x.contains(MyFlags::A));
        assert!(!x.contains(MyFlags::B));
        assert!(x.contains(MyFlags::C));

        x.remove(MyFlags::A);
        assert!(!x.contains(MyFlags::A));
        assert!(!x.contains(MyFlags::B));
        assert!(x.contains(MyFlags::C));

        x.remove(MyFlags::all());
        assert!(!x.contains(MyFlags::A));
        assert!(!x.contains(MyFlags::B));
        assert!(!x.contains(MyFlags::C));
        assert_eq!(x, MyFlags::none());
    }

    #[test]
    fn insert()
    {
        let mut x = MyFlags::A;
        assert!(x.contains(MyFlags::A));
        assert!(!x.contains(MyFlags::B));
        assert!(!x.contains(MyFlags::C));

        x.insert(MyFlags::B);
        assert!(x.contains(MyFlags::A));
        assert!(x.contains(MyFlags::B));
        assert!(!x.contains(MyFlags::C));
    }

    #[test]
    fn toggle()
    {
        let mut x = MyFlags::A;
        assert!(x.contains(MyFlags::A));
        assert!(!x.contains(MyFlags::B));
        assert!(!x.contains(MyFlags::C));

        x.toggle(MyFlags::B);
        assert!(x.contains(MyFlags::A));
        assert!(x.contains(MyFlags::B));
        assert!(!x.contains(MyFlags::C));

        x.toggle(MyFlags::A);
        assert!(!x.contains(MyFlags::A));
        assert!(x.contains(MyFlags::B));
        assert!(!x.contains(MyFlags::C));
    }
}
