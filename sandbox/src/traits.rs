pub trait Serializer<'se>
{
    type Ok;
    type Err;
    type ArraySerializer: ArraySerializer<Ok = Self::Ok, Err = Self::Err>;
    type MapSerializer: MapSerializer<Ok = Self::Ok, Err = Self::Err>;

    fn serialize_u8(&'se mut self, value: u8) -> Result<Self::Ok, Self::Err>;
    fn serialize_u16(&'se mut self, value: u16) -> Result<Self::Ok, Self::Err>;
    fn serialize_u32(&'se mut self, value: u32) -> Result<Self::Ok, Self::Err>;
    fn serialize_u64(&'se mut self, value: u64) -> Result<Self::Ok, Self::Err>;
    fn serialize_i8(&'se mut self, value: i8) -> Result<Self::Ok, Self::Err>;
    fn serialize_i16(&'se mut self, value: i16) -> Result<Self::Ok, Self::Err>;
    fn serialize_i32(&'se mut self, value: i32) -> Result<Self::Ok, Self::Err>;
    fn serialize_i64(&'se mut self, value: i64) -> Result<Self::Ok, Self::Err>;
    fn serialize_bytes(&'se mut self, value: &[u8]) -> Result<Self::Ok, Self::Err>;
    fn serialize_str(&'se mut self, value: &str) -> Result<Self::Ok, Self::Err>;
    fn serialize_array(
        &'se mut self,
        len: Option<usize>,
    ) -> Result<Self::ArraySerializer, Self::Err>;
    fn serialize_map(&'se mut self, len: Option<usize>) -> Result<Self::MapSerializer, Self::Err>;
    fn serialize_f32(&'se mut self, value: f32) -> Result<Self::Ok, Self::Err>;
    fn serialize_f64(&'se mut self, value: f64) -> Result<Self::Ok, Self::Err>;
    fn serialize_null(&'se mut self) -> Result<Self::Ok, Self::Err>;
    fn serialize_bool(&'se mut self, value: bool) -> Result<Self::Ok, Self::Err>;
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
    fn serialize<'se, S: Serializer<'se>>(&self, s: &'se mut S) -> Result<S::Ok, S::Err>;
}
