use fnd::{
    alloc::Allocator,
    containers::{Array, String},
};

pub trait Serializer
{
    type Ok;
    type Err;
    type ArraySerializer: ArraySerializer<Ok = Self::Ok, Err = Self::Err>;
    type MapSerializer: MapSerializer<Ok = Self::Ok, Err = Self::Err>;

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Err>;
    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Err>;
    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Err>;
    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Err>;
    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Err>;
    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Err>;
    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Err>;
    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Err>;
    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Err>;
    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Err>;
    fn serialize_array(self, len: Option<usize>) -> Result<Self::ArraySerializer, Self::Err>;
    fn serialize_map(self, len: Option<usize>) -> Result<Self::MapSerializer, Self::Err>;
    fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Err>;
    fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Err>;
    fn serialize_null(self) -> Result<Self::Ok, Self::Err>;
    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Err>;
}

pub trait ArraySerializer
{
    type Ok;
    type Err;

    fn serialize_element<T: Serialize>(&mut self, elmt: T) -> Result<Self::Ok, Self::Err>;
    fn end(self) -> Result<Self::Ok, Self::Err>;
}

pub trait MapSerializer
{
    type Ok;
    type Err;

    fn serialize_entry<K: Serialize, V: Serialize>(
        &mut self,
        k: K,
        v: V,
    ) -> Result<Self::Ok, Self::Err>;
    fn end(self) -> Result<Self::Ok, Self::Err>;
}

pub trait Serialize
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Err>;
}

pub enum VisitorType
{
    UnsignedInteger,
    SignedInteger,
    FloatingPoint,
    Boolean,
    String,
    Bytes,
    Null,
    Array,
    Map,
}

pub trait VisitorError
{
    fn unexpected_type(t: VisitorType) -> Self;
}

pub trait Visitor<'de>: Sized
{
    type Value;
    type Err: VisitorError;

    fn accept_u8(self, value: u8) -> Result<Self::Value, Self::Err>
    {
        self.accept_u16(value as u16)
    }

    fn accept_u16(self, value: u16) -> Result<Self::Value, Self::Err>
    {
        self.accept_u32(value as u32)
    }

    fn accept_u32(self, value: u32) -> Result<Self::Value, Self::Err>
    {
        self.accept_u64(value as u64)
    }

    fn accept_u64(self, _value: u64) -> Result<Self::Value, Self::Err>
    {
        Err(VisitorError::unexpected_type(VisitorType::UnsignedInteger))
    }

    fn accept_i8(self, value: i8) -> Result<Self::Value, Self::Err>
    {
        self.accept_i16(value as i16)
    }

    fn accept_i16(self, value: i16) -> Result<Self::Value, Self::Err>
    {
        self.accept_i32(value as i32)
    }

    fn accept_i32(self, value: i32) -> Result<Self::Value, Self::Err>
    {
        self.accept_i64(value as i64)
    }

    fn accept_i64(self, _value: i64) -> Result<Self::Value, Self::Err>
    {
        Err(VisitorError::unexpected_type(VisitorType::SignedInteger))
    }

    fn accept_f32(self, value: f32) -> Result<Self::Value, Self::Err>
    {
        self.accept_f64(value as f64)
    }

    fn accept_f64(self, _value: f64) -> Result<Self::Value, Self::Err>
    {
        Err(VisitorError::unexpected_type(VisitorType::FloatingPoint))
    }

    fn accept_bool(self, _value: bool) -> Result<Self::Value, Self::Err>
    {
        Err(VisitorError::unexpected_type(VisitorType::Boolean))
    }

    fn accept_str(self, _value: &str) -> Result<Self::Value, Self::Err>
    {
        Err(VisitorError::unexpected_type(VisitorType::String))
    }

    fn accept_borrowed_str(self, value: &'de str) -> Result<Self::Value, Self::Err>
    {
        self.accept_str(value)
    }

    fn accept_string<A: Allocator>(self, value: String<A>) -> Result<Self::Value, Self::Err>
    {
        self.accept_str(&value)
    }

    fn accept_bytes(self, _value: &[u8]) -> Result<Self::Value, Self::Err>
    {
        Err(VisitorError::unexpected_type(VisitorType::Bytes))
    }

    fn accept_borrowed_bytes(self, value: &'de [u8]) -> Result<Self::Value, Self::Err>
    {
        self.accept_bytes(value)
    }

    fn accept_bytes_array<A: Allocator>(self, value: Array<u8, A>)
        -> Result<Self::Value, Self::Err>
    {
        self.accept_bytes(&value)
    }

    fn accept_null(self) -> Result<Self::Value, Self::Err>
    {
        Err(VisitorError::unexpected_type(VisitorType::Null))
    }

    fn accept_array<A>(self, _accessor: A) -> Result<Self::Value, Self::Err>
    where
        A: ArrayAccessor<'de>,
    {
        Err(VisitorError::unexpected_type(VisitorType::Array))
    }

    fn accept_map<A>(self, _accessor: A) -> Result<Self::Value, Self::Err>
    where
        A: MapAccessor<'de>,
    {
        Err(VisitorError::unexpected_type(VisitorType::Map))
    }
}

pub trait ArrayAccessor<'de>
{
    type Err;

    fn size_hint(&self) -> Option<usize>;

    fn next_element<T>(&mut self) -> Result<Option<T>, Self::Err>
    where
        T: Deserialize<'de>;
}

pub trait MapAccessor<'de>
{
    type Err;

    fn size_hint(&self) -> Option<usize>;

    fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Err>
    where
        K: Deserialize<'de>,
        V: Deserialize<'de>;
}

pub trait Deserializer<'de>
{
    type Err;

    fn deserialize_any<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>;
    fn deserialize_u8<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>;
    fn deserialize_u16<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>;
    fn deserialize_u32<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>;
    fn deserialize_u64<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>;
    fn deserialize_i8<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>;
    fn deserialize_i16<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>;
    fn deserialize_i32<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>;
    fn deserialize_i64<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>;
    fn deserialize_f32<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>;
    fn deserialize_f64<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>;
    fn deserialize_bool<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>;
    fn deserialize_str<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>;
    fn deserialize_string<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>;
    fn deserialize_bytes<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>;
    fn deserialize_bytes_array<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>;
    fn deserialize_null<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>;
    fn deserialize_array<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>;
    fn deserialize_map<V: Visitor<'de>>(self, v: V) -> Result<V::Value, Self::Err>;
}

pub trait Deserialize<'de>: Sized
{
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Err>;
}
