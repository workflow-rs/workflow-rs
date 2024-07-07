use crate::imports::*;
use argon2::Argon2;
use sha2::{Digest, Sha256};

/// Produces `SHA256` hash of the given data.
#[inline]
pub fn sha256(data: &[u8]) -> Secret {
    let mut hash = Sha256::default();
    hash.update(data);
    Secret::new(hash.finalize().to_vec())
}

/// Produces `SHA256d` hash of the given data.
#[inline]
pub fn sha256d(data: &[u8]) -> Secret {
    let mut hash = Sha256::default();
    hash.update(data);
    sha256(hash.finalize().as_slice())
}

/// Produces an argon2(sha256(data)) hash of the given data.
pub fn argon2_sha256(data: &[u8], byte_length: usize) -> Result<Secret> {
    let salt = sha256(data);
    let mut key = vec![0u8; byte_length];
    Argon2::default().hash_password_into(data, salt.as_ref(), &mut key)?;
    Ok(key.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use workflow_core::hex::ToHex;

    #[test]
    fn test_wallet_argon2() {
        println!("testing argon2 hash");
        let password = b"user_password";
        let hash = argon2_sha256(password, 32).unwrap();
        let hash_hex = hash.as_ref().to_hex();
        // println!("argon2hash: {:?}", hash_hex);
        assert_eq!(
            hash_hex,
            "a79b661f0defd1960a4770889e19da0ce2fde1e98ca040f84ab9b2519ca46234"
        );
    }
}
