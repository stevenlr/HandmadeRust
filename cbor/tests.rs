use super::*;

use std::io::Cursor;

#[test]
fn deserialize()
{
    macro_rules! check_value {
        ($value:expr, $type:ident, $serialized:expr) => {
            let mut de = CborDeserializer::new(Cursor::new($serialized));
            let value = $type::deserialize(&mut de).unwrap();
            assert_eq!(value, $value);
        };
    }

    check_value!(0u32, u32, [0x00]);
    check_value!(0u8, u8, [0x00]);
    check_value!(0i8, i8, [0x00]);
    check_value!(1u32, u32, [0x01]);
    check_value!(10u32, u32, [0x0a]);
    check_value!(23i32, i32, [0x17]);
    check_value!(24i32, i32, [0x18, 0x18]);
    check_value!(25u32, u32, [0x18, 0x19]);
    check_value!(100u32, u32, [0x18, 0x64]);
    check_value!(1000i64, i64, [0x19, 0x03, 0xe8]);
    check_value!(1000000u32, u32, [0x1a, 0x00, 0x0f, 0x42, 0x40]);
    check_value!(
        1000000000000u64,
        u64,
        [0x1b, 0x00, 0x00, 0x00, 0xe8, 0xd4, 0xa5, 0x10, 0x00]
    );
    check_value!(
        18446744073709551615u64,
        u64,
        [0x1b, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
    );
    check_value!(-1i8, i8, [0x20]);
    check_value!(-10i8, i8, [0x29]);
    check_value!(-100i8, i8, [0x38, 0x63]);
    check_value!(-1000i32, i32, [0x39, 0x03, 0xe7]);
    check_value!(-543514321i64, i64, [0x3A, 0x20, 0x65, 0x5E, 0xD0]);
    check_value!(true, bool, [0xf5]);
    check_value!(false, bool, [0xf4]);
    check_value!(0.0f32, f32, [250, 0, 0, 0, 0]);
    check_value!(1.0f64, f64, [251, 63, 240, 0, 0, 0, 0, 0, 0]);

    struct MyVisitorError;
    impl VisitorError for MyVisitorError
    {
        fn unexpected_type(_: VisitorType) -> Self
        {
            panic!();
        }
    }

    {
        struct BytesCheck;

        impl<'de> Visitor<'de> for BytesCheck
        {
            type Value = ();
            type Err = MyVisitorError;

            fn accept_bytes(self, bytes: &[u8]) -> Result<Self::Value, Self::Err>
            {
                assert_eq!(bytes, [1, 5, 10, 16, 7]);
                Ok(())
            }
        }

        let mut de = CborDeserializer::new(Cursor::new(&[69, 1, 5, 10, 16, 7]));
        de.deserialize_bytes(BytesCheck).unwrap();
    }

    macro_rules! check_string {
        ($reference:expr, $bytes:expr) => {{
            struct StringCheck;

            impl<'de> Visitor<'de> for StringCheck
            {
                type Value = ();
                type Err = MyVisitorError;

                fn accept_str(self, bytes: &str) -> Result<Self::Value, Self::Err>
                {
                    assert_eq!(bytes, $reference);
                    Ok(())
                }
            }

            let mut de = CborDeserializer::new(Cursor::new($bytes));
            de.deserialize_string(StringCheck).unwrap();
        }};
    }

    check_string!("", &[0x60]);
    check_string!("a", &[0x61, 0x61]);
    check_string!("IETF", &[0x64, 0x49, 0x45, 0x54, 0x46]);
    check_string!("\u{20ac}¢", &[101, 226, 130, 172, 194, 162]);

    {
        struct NullCheck
        {
            found: bool,
        }

        impl<'de> Visitor<'de> for &mut NullCheck
        {
            type Value = ();
            type Err = MyVisitorError;

            fn accept_null(self) -> Result<Self::Value, Self::Err>
            {
                self.found = true;
                Ok(())
            }
        }

        let mut de = CborDeserializer::new(Cursor::new(&[0xf6]));

        let mut checker = NullCheck { found: false };
        de.deserialize_null(&mut checker).unwrap();
        assert!(checker.found);
    }

    macro_rules! check_array {
        ($reference:expr, $bytes:expr) => {{
            struct ArrayVisitor;

            impl<'de> Visitor<'de> for ArrayVisitor
            {
                type Value = Array<u32>;
                type Err = MyVisitorError;

                fn accept_array<A>(self, mut accessor: A) -> Result<Self::Value, Self::Err>
                where
                    A: ArrayAccessor<'de>,
                {
                    let mut array = Array::<u32>::new();

                    while let Some(value) = accessor.next_element().map_err(|_| MyVisitorError)?
                    {
                        array.push(value);
                    }

                    return Ok(array);
                }
            }

            let mut de = CborDeserializer::new(Cursor::new($bytes));
            let result = de.deserialize_array(ArrayVisitor).unwrap();
            assert_eq!(&*result, $reference);
        }};
    }

    check_array!((&[] as &[u32]), &[0x80, 8, 9, 7]);
    check_array!((&[1u32, 2u32, 3u32] as &[u32]), &[131, 1, 2, 3, 4, 5]);
    check_array!((&[1u32, 2u32, 3u32] as &[u32]), &[159, 1, 2, 3, 255, 4, 5]);

    {
        struct MapVisitor;

        impl<'de> Visitor<'de> for MapVisitor
        {
            type Value = ();
            type Err = MyVisitorError;

            fn accept_map<A>(self, mut accessor: A) -> Result<Self::Value, Self::Err>
            where
                A: MapAccessor<'de>,
            {
                let (key, value) = accessor
                    .next_entry::<String, i32>()
                    .map_err(|_| MyVisitorError)?
                    .unwrap();
                assert_eq!(key, "a");
                assert_eq!(value, 1);
                let (key, value) = accessor
                    .next_entry::<i32, String>()
                    .map_err(|_| MyVisitorError)?
                    .unwrap();
                assert_eq!(key, 2);
                assert_eq!(value, "hello");
                assert!(accessor
                    .next_entry::<u32, u32>()
                    .map_err(|_| MyVisitorError)?
                    .is_none());
                return Ok(());
            }
        }

        let mut de = CborDeserializer::new(Cursor::new(&[
            191, 97, 97, 1, 2, 101, 104, 101, 108, 108, 111, 255, 1, 1, 1,
        ]));

        de.deserialize_map(MapVisitor).unwrap();
    }
}

#[test]
fn serialize()
{
    let mut v = Vec::<u8>::new();

    macro_rules! check_value {
        ($value:expr, $expected:expr) => {
            v.clear();

            let cursor = Cursor::new(&mut v);
            let mut serializer = CborSerializer::new(cursor);

            $value.serialize(&mut serializer).unwrap();
            assert_eq!(v, $expected);
        };
    }

    check_value!(0u32, &[0x00]);
    check_value!(0u8, &[0x00]);
    check_value!(0i8, &[0x00]);
    check_value!(1u32, &[0x01]);
    check_value!(10u32, &[0x0a]);
    check_value!(23u32, &[0x17]);
    check_value!(24u32, &[0x18, 0x18]);
    check_value!(25u32, &[0x18, 0x19]);
    check_value!(100u32, &[0x18, 0x64]);
    check_value!(1000u32, &[0x19, 0x03, 0xe8]);
    check_value!(1000000u32, &[0x1a, 0x00, 0x0f, 0x42, 0x40]);
    check_value!(
        1000000000000u64,
        &[0x1b, 0x00, 0x00, 0x00, 0xe8, 0xd4, 0xa5, 0x10, 0x00]
    );
    check_value!(
        18446744073709551615u64,
        &[0x1b, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
    );
    check_value!(-1i8, &[0x20]);
    check_value!(-10i8, &[0x29]);
    check_value!(-100i8, &[0x38, 0x63]);
    check_value!(-1000i32, &[0x39, 0x03, 0xe7]);
    check_value!(-543514321i64, &[0x3A, 0x20, 0x65, 0x5E, 0xD0]);
    check_value!("", &[0x60]);
    check_value!("a", &[0x61, 0x61]);
    check_value!("IETF", &[0x64, 0x49, 0x45, 0x54, 0x46]);
    check_value!("\u{20ac}¢", &[101, 226, 130, 172, 194, 162]);
    check_value!((&[] as &[u32])[..], &[0x80]);
    check_value!(&[1, 2, 3][..], &[131, 1, 2, 3]);
    check_value!(true, &[0xf5]);
    check_value!(false, &[0xf4]);
    check_value!(0.0f32, &[250, 0, 0, 0, 0]);
    check_value!(1.0f64, &[251, 63, 240, 0, 0, 0, 0, 0, 0]);

    {
        v.clear();

        let cursor = Cursor::new(&mut v);
        let mut serializer = CborSerializer::new(cursor);

        let mut array_ser = serializer.serialize_array(None).unwrap();
        array_ser.serialize_element(1u32).unwrap();
        array_ser.serialize_element(2u32).unwrap();
        array_ser.serialize_element(3u32).unwrap();
        array_ser.end().unwrap();

        assert_eq!(v, &[159, 1, 2, 3, 255]);
    }

    {
        v.clear();

        let cursor = Cursor::new(&mut v);
        let mut ser = CborSerializer::new(cursor);

        let mut map = ser.serialize_map(None).unwrap();
        map.serialize_entry(&"a", 1).unwrap();
        map.serialize_entry(2, &"hello").unwrap();
        map.end().unwrap();

        assert_eq!(v, &[191, 97, 97, 1, 2, 101, 104, 101, 108, 108, 111, 255]);
    }

    {
        v.clear();

        let cursor = Cursor::new(&mut v);
        let mut ser = CborSerializer::new(cursor);

        let mut _map = ser.serialize_null().unwrap();
        assert_eq!(v, &[0xf6]);
    }

    {
        v.clear();

        let cursor = Cursor::new(&mut v);
        let mut ser = CborSerializer::new(cursor);

        let mut _map = ser.serialize_bytes(&[1, 5, 10, 16, 7]).unwrap();
        assert_eq!(v, &[69, 1, 5, 10, 16, 7]);
    }
}
