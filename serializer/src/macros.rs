#[macro_export]
macro_rules! store {
    ($type:ty, $value:expr, $writer:expr) => {
        <$type as borsh::BorshSerialize>::serialize($value, $writer)
    };
}

#[macro_export]
macro_rules! load {
    ($type:ty, $reader:expr) => {
        <$type as borsh::BorshDeserialize>::deserialize_reader($reader)
    };
}

#[macro_export]
macro_rules! serialize {
    ($type:ty, $value:expr, $writer:expr) => {
        <$type as $crate::serializer::Serializer>::serialize($value, $writer)
    };
}

#[macro_export]
macro_rules! deserialize {
    ($type:ty, $reader:expr) => {
        <$type as $crate::serializer::Serializer>::deserialize($reader)
    };
}
