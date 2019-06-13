use crate::traits::*;

impl Serialize for u8
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_u8(*self)
    }
}

impl Serialize for u16
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_u16(*self)
    }
}

impl Serialize for u32
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_u32(*self)
    }
}

impl Serialize for u64
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_u64(*self)
    }
}

impl Serialize for i8
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_i8(*self)
    }
}

impl Serialize for i16
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_i16(*self)
    }
}

impl Serialize for i32
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_i32(*self)
    }
}

impl Serialize for i64
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_i64(*self)
    }
}

impl Serialize for f32
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_f32(*self)
    }
}

impl Serialize for f64
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_f64(*self)
    }
}

impl Serialize for &str
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_str(self)
    }
}

impl Serialize for bool
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>
    {
        s.serialize_bool(*self)
    }
}

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
