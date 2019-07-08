use crate::num::*;

use core::ops;

pub trait Vector
{
    type Scalar: Real;

    fn dot(&self, v: &Self) -> Self::Scalar;

    #[inline]
    fn length2(&self) -> Self::Scalar
    {
        self.dot(&self)
    }
}

macro_rules! vec_impl {
    ($name:ident: $t:ty, ($($c:ident),+)) => {
        #[repr(C)]
        #[derive(Clone, Copy, Default, PartialEq, Debug)]
        pub struct $name
        {
            $(
                pub $c: $t,
            )+
        }

        impl $name
        {
            pub fn new($($c: $t),+) -> Self
            {
                Self {
                    $(
                        $c,
                    )+
                }
            }
        }

        impl Vector for $name
        {
            type Scalar = $t;

            #[inline]
            fn dot(&self, v: &Self) -> $t
            {
                <$t>::zero() $(
                    + self.$c * v.$c
                )+
            }
        }

        impl From<$t> for $name
        {
            fn from(x: $t) -> Self
            {
                Self {
                    $(
                        $c: x,
                    )+
                }
            }
        }

        impl ops::Neg for $name
        {
            type Output = Self;

            fn neg(self) -> Self
            {
                Self {
                    $(
                        $c: -self.$c,
                    )+
                }
            }
        }

        impl ops::Add for $name
        {
            type Output = Self;

            #[inline]
            fn add(self, v: Self) -> Self
            {
                Self {
                    $(
                        $c: self.$c + v.$c,
                    )+
                }
            }
        }

        impl ops::AddAssign for $name
        {
            #[inline]
            fn add_assign(&mut self, v: Self)
            {
                $(
                    self.$c += v.$c;
                )+
            }
        }

        impl ops::Sub for $name
        {
            type Output = Self;

            #[inline]
            fn sub(self, v: Self) -> Self
            {
                Self {
                    $(
                        $c: self.$c - v.$c,
                    )+
                }
            }
        }

        impl ops::SubAssign for $name
        {
            #[inline]
            fn sub_assign(&mut self, v: Self)
            {
                $(
                    self.$c -= v.$c;
                )+
            }
        }

        impl ops::Mul for $name
        {
            type Output = Self;

            #[inline]
            fn mul(self, v: Self) -> Self
            {
                Self {
                    $(
                        $c: self.$c * v.$c,
                    )+
                }
            }
        }

        impl ops::MulAssign for $name
        {
            #[inline]
            fn mul_assign(&mut self, v: Self)
            {
                $(
                    self.$c *= v.$c;
                )+
            }
        }

        impl ops::Mul<$t> for $name
        {
            type Output = Self;

            #[inline]
            fn mul(self, v: $t) -> Self
            {
                Self {
                    $(
                        $c: self.$c * v,
                    )+
                }
            }
        }

        impl ops::MulAssign<$t> for $name
        {
            #[inline]
            fn mul_assign(&mut self, v: $t)
            {
                $(
                    self.$c *= v;
                )+
            }
        }

        impl ops::Div<$t> for $name
        {
            type Output = Self;

            #[inline]
            fn div(self, v: $t) -> Self
            {
                Self {
                    $(
                        $c: self.$c / v,
                    )+
                }
            }
        }

        impl ops::DivAssign<$t> for $name
        {
            #[inline]
            fn div_assign(&mut self, v: $t)
            {
                $(
                    self.$c /= v;
                )+
            }
        }

        impl ops::Mul<$name> for $t
        {
            type Output = $name;

            #[inline]
            fn mul(self, v: $name) -> $name
            {
                $name {
                    $(
                        $c: self * v.$c,
                    )+
                }
            }
        }

        impl ops::Div<$name> for $t
        {
            type Output = $name;

            #[inline]
            fn div(self, v: $name) -> $name
            {
                $name {
                    $(
                        $c: self / v.$c,
                    )+
                }
            }
        }
    };
}

vec_impl!(Vec2f: f32, (x, y));
vec_impl!(Vec2d: f64, (x, y));
vec_impl!(Vec3f: f32, (x, y, z));
vec_impl!(Vec3d: f64, (x, y, z));
vec_impl!(Vec4f: f32, (x, y, z, w));
vec_impl!(Vec4d: f64, (x, y, z, w));

#[inline]
pub fn dot<T: Vector>(a: &T, b: &T) -> T::Scalar
{
    a.dot(b)
}

// @Todo cross, from tuple, display, normalize, length
