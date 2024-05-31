#[allow(clippy::module_inception)]
#[cfg(test)]
mod tests {

    use borsh::{BorshDeserialize, BorshSerialize};
    use crate::prelude::{load, store, Serializable, Serializer};
    use crate::result::IoResult;

    #[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
    struct MyStruct {
        field1: u32,
        field2: String,
    }

    #[test]
    fn test_serializer_store_and_load() -> Result<(), Box<dyn std::error::Error>> {
        let value = MyStruct {
            field1: 42,
            field2: String::from("Hello, world!"),
        };

        // Serialize (store)
        let mut buffer = Vec::new();
        store!(MyStruct, &value, &mut buffer)?;

        // Deserialize (load)
        let deserialized_value: MyStruct = load!(MyStruct, &mut buffer.as_slice())?;

        // Assert the original and deserialized values are the same
        assert_eq!(value, deserialized_value);

        Ok(())
    }

    struct MyVersionedStruct {
        field1: u32,
        field2: String,
        field3: bool,
    }

    impl Serializer for MyVersionedStruct {
        fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
            // Serialize the version
            store!(u32, &1, writer)?;
            // Serialize the fields
            store!(u32, &self.field1, writer)?;
            store!(String, &self.field2, writer)?;
            store!(bool, &self.field3, writer)?;

            Ok(())
        }

        fn deserialize(buf: &mut &[u8]) -> IoResult<Self> {
            // Deserialize the version
            let version: u32 = load!(u32, buf)?;
            // Deserialize the fields
            let field1: u32 = load!(u32, buf)?;
            let field2: String = load!(String, buf)?;
            let field3: bool = load!(bool, buf)?;

            assert_eq!(version, 1);

            Ok(Self {
                field1,
                field2,
                field3,
            })
        }
    }

    #[test]
    fn test_serializer_versioning() -> Result<(), Box<dyn std::error::Error>> {
        let value = MyVersionedStruct {
            field1: 42,
            field2: String::from("Hello, world!"),
            field3: true,
        };

        let serializable = Serializable(value);
        let mut buffer = Vec::new();
        borsh::BorshSerialize::serialize(&serializable, &mut buffer)?;

        let deserialized: Serializable<MyVersionedStruct> =
            borsh::BorshDeserialize::deserialize(&mut buffer.as_slice())?;
        let deserialized_value = deserialized.into_inner();

        // assert_eq!(version, 1);
        assert_eq!(deserialized_value.field1, 42);
        assert!(deserialized_value.field2 == "Hello, world!");
        assert!(deserialized_value.field3);

        Ok(())
    }
}
