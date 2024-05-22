#[macro_export]
macro_rules! store {
    ($type:ty, $value:expr, $writer:expr) => {
        // borsh::BorshSerialize::serialize::<$type>($value, $writer)?
        <$type as borsh::BorshSerialize>::serialize($value, $writer)?
    };
}

#[macro_export]
macro_rules! load {
    ($type:ty, $buffer:expr) => {
        // borsh::BorshDeserialize::<$type>::deserialize($buffer)?
        borsh::BorshDeserialize::deserialize($buffer)?
    };
}
