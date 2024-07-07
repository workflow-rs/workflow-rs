use crate::imports::*;
use chacha20poly1305::{
    aead::{AeadCore, AeadInPlace, KeyInit, OsRng},
    Key, XChaCha20Poly1305,
};

/// Encrypts the given data using `XChaCha20Poly1305` algorithm.
pub fn encrypt<T>(data: &T, secret: &Secret) -> Result<Vec<u8>>
where
    T: Serializer,
{
    let mut buffer = vec![];
    data.serialize(&mut buffer)?;
    encrypt_slice(&buffer, secret)
}

pub fn encrypt_slice(data: &[u8], secret: &Secret) -> Result<Vec<u8>> {
    let private_key_bytes = argon2_sha256(secret.as_ref(), 32)?;
    let key = Key::from_slice(private_key_bytes.as_ref());
    let cipher = XChaCha20Poly1305::new(key);
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let mut buffer = data.to_vec();
    buffer.reserve(24);
    cipher.encrypt_in_place(&nonce, &[], &mut buffer)?;
    buffer.extend(nonce.iter().cloned());
    Ok(buffer)
}

pub fn decrypt<T>(data: &[u8], secret: &Secret) -> Result<T>
where
    T: Deserializer,
{
    let data = decrypt_slice(data, secret)?;
    Ok(T::try_from_slice(data.as_bytes())?)
}

/// Decrypts the given data using `XChaCha20Poly1305` algorithm.
pub fn decrypt_slice(data: &[u8], secret: &Secret) -> Result<Secret> {
    let private_key_bytes = argon2_sha256(secret.as_ref(), 32)?;
    let key = Key::from_slice(private_key_bytes.as_ref());
    let cipher = XChaCha20Poly1305::new(key);
    let len = data.len() - 24;
    let nonce = &data[len..];
    let mut buffer = data[..len].to_vec();
    cipher.decrypt_in_place(nonce.into(), &[], &mut buffer)?;
    Ok(Secret::new(buffer))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() -> Result<()> {
        use crate::imports::*;

        println!("testing encrypt/decrypt");

        let password = b"password";
        let original = b"hello world".to_vec();
        // println!("original: {}", original.to_hex());
        let password = Secret::new(password.to_vec());
        let encrypted = encrypt_slice(&original, &password).unwrap();
        // println!("encrypted: {}", encrypted.to_hex());
        let decrypted = decrypt_slice(&encrypted, &password).unwrap();
        // println!("decrypted: {}", decrypted.to_hex());
        assert_eq!(decrypted.as_ref(), original);

        Ok(())
    }
}
