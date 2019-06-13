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
