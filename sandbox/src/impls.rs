use crate::traits::*;

macro_rules! impl_primitive_ser {
    ($type:ty, $method:ident) => {
        impl Serialize for $type
        {
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
            {
                s.$method(*self)
            }
        }
    };
}

pub struct PrimitiveVisitorError(VisitorType);

impl VisitorError for PrimitiveVisitorError
{
    fn unexpected_type(t: VisitorType) -> Self
    {
        Self(t)
    }
}

macro_rules! impl_primitive_de {
    ($type:ty, $method:ident, $accept:ident) => {
        impl<'de> Deserialize<'de> for $type
        {
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<$type, D::Err>
            {
                struct PrimVisitor;

                impl<'de> Visitor<'de> for PrimVisitor
                {
                    type Value = $type;
                    type Err = PrimitiveVisitorError;

                    fn $accept(self, value: $type) -> Result<Self::Value, Self::Err>
                    {
                        Ok(value)
                    }
                }

                d.$method(&mut PrimVisitor)
            }
        }
    };
}

impl_primitive_ser!(u8, serialize_u8);
impl_primitive_ser!(u16, serialize_u16);
impl_primitive_ser!(u32, serialize_u32);
impl_primitive_ser!(u64, serialize_u64);
impl_primitive_ser!(i8, serialize_i8);
impl_primitive_ser!(i16, serialize_i16);
impl_primitive_ser!(i32, serialize_i32);
impl_primitive_ser!(i64, serialize_i64);
impl_primitive_ser!(f32, serialize_f32);
impl_primitive_ser!(f64, serialize_f64);
impl_primitive_ser!(&str, serialize_str);
impl_primitive_ser!(bool, serialize_bool);

impl_primitive_de!(u8, deserialize_u8, accept_u8);
impl_primitive_de!(u16, deserialize_u16, accept_u16);
impl_primitive_de!(u32, deserialize_u32, accept_u32);
impl_primitive_de!(u64, deserialize_u64, accept_u64);
impl_primitive_de!(i8, deserialize_i8, accept_i8);
impl_primitive_de!(i16, deserialize_i16, accept_i16);
impl_primitive_de!(i32, deserialize_i32, accept_i32);
impl_primitive_de!(i64, deserialize_i64, accept_i64);
impl_primitive_de!(f32, deserialize_f32, accept_f32);
impl_primitive_de!(f64, deserialize_f64, accept_f64);
impl_primitive_de!(bool, deserialize_bool, accept_bool);

impl<'a, T> Serialize for &'a T
where
    T: Serialize,
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        (**self).serialize(s)
    }
}

impl<'a, T> Serialize for &'a mut T
where
    T: Serialize,
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        (**self).serialize(s)
    }
}

impl<T> Serialize for [T]
where
    T: Serialize,
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        let mut array_serializer = s.serialize_array(Some(self.len()))?;

        for x in self.iter()
        {
            array_serializer.serialize_element(x)?;
        }

        array_serializer.end()
    }
}

impl<'de> Deserialize<'de> for &'de str
{
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<&'de str, D::Err>
    {
        struct PrimVisitor;

        impl<'de> Visitor<'de> for PrimVisitor
        {
            type Value = &'de str;
            type Err = PrimitiveVisitorError;

            fn accept_borrowed_str(self, value: &'de str) -> Result<Self::Value, Self::Err>
            {
                Ok(value)
            }
        }

        d.deserialize_str(&mut PrimVisitor)
    }
}
