use crate::node_sys::{
    class::{
        Buffer,
        crypto::{
            Cipher, Decipher, DiffieHellman, DiffieHellmanGroup, Ecdh, Hash, Hmac, KeyObject, Sign,
            Verify,
        },
    },
    interface::{
        CryptoConstants, GenerateKeyPairOptions, GenerateKeyPairSyncReturn, ScryptOptions,
        StreamTransformOptions, StreamWritableOptions,
    },
};
use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "crypto")]
extern "C" {
    //*******************//
    // Module Properties //
    //*******************//

    pub static constants: CryptoConstants;

    //****************//
    // Module Methods //
    //****************//

    #[wasm_bindgen(js_name = "createCipheriv")]
    pub fn create_cipheriv(
        algorithm: &str,
        key: &JsValue,
        iv: &JsValue,
        options: Option<StreamTransformOptions>,
    ) -> Cipher;

    #[wasm_bindgen(js_name = "createDecipheriv")]
    pub fn create_decipheriv(
        algorithm: &str,
        key: &JsValue,
        iv: &JsValue,
        options: Option<StreamTransformOptions>,
    ) -> Decipher;

    #[wasm_bindgen(js_name = "createDiffieHellman")]
    pub fn create_diffie_hellman_with_prime(
        prime: &JsValue,
        prime_encoding: Option<&str>,
        generator: &JsValue,
        generator_encoding: Option<&str>,
    ) -> DiffieHellman;

    #[wasm_bindgen(js_name = "createDiffieHellman")]
    pub fn create_diffie_hellman(prime_length: u32, generator: &JsValue) -> DiffieHellman;

    #[wasm_bindgen(js_name = "createECDH")]
    pub fn create_ecdh(curve_name: &str) -> Ecdh;

    #[wasm_bindgen(js_name = "createHash")]
    pub fn create_hash(algorithm: &str, options: Option<StreamTransformOptions>) -> Hash;

    #[wasm_bindgen(js_name = "createHmac")]
    pub fn create_hmac(
        algorithm: &str,
        key: &JsValue,
        option: Option<StreamTransformOptions>,
    ) -> Hmac;

    #[wasm_bindgen(js_name = "createPrivateKey")]
    pub fn create_private_key(key: &JsValue) -> KeyObject;

    #[wasm_bindgen(js_name = "createPublicKey")]
    pub fn create_public_key(key: &JsValue) -> KeyObject;

    #[wasm_bindgen(js_name = "createSecretKey")]
    pub fn create_secret_key(key: &Buffer) -> KeyObject;

    #[wasm_bindgen(js_name = "createSign")]
    pub fn create_sign(algorithm: &str, options: Option<StreamWritableOptions>) -> Sign;

    #[wasm_bindgen(js_name = "createVerify")]
    pub fn create_verify(algorithm: &str, options: Option<StreamWritableOptions>) -> Verify;

    #[wasm_bindgen(js_name = "generateKeyPair")]
    pub fn generate_key_pair(kind: &str, options: &GenerateKeyPairOptions, callback: &Function);

    #[wasm_bindgen(js_name = "generateKeyPairSync")]
    pub fn generate_key_pair_sync(
        kind: &str,
        options: &GenerateKeyPairOptions,
    ) -> GenerateKeyPairSyncReturn;

    #[wasm_bindgen(js_name = "getCiphers")]
    pub fn get_ciphers() -> Box<[JsValue]>;

    #[wasm_bindgen(js_name = "getCurves")]
    pub fn get_curves() -> Box<[JsValue]>;

    #[wasm_bindgen(js_name = "getDiffieHellman")]
    pub fn get_diffie_hellman(group_name: &str) -> DiffieHellmanGroup;

    #[wasm_bindgen(js_name = "getFips")]
    pub fn get_fips(group_name: &str) -> DiffieHellmanGroup;

    #[wasm_bindgen(js_name = "getHashes")]
    pub fn get_hashes() -> Box<[JsValue]>;

    #[wasm_bindgen(js_name = "pbkdf2")]
    pub fn pbkdf2(
        password: &JsValue,
        salt: &JsValue,
        iterations: u32,
        keylen: u32,
        digest: &str,
        callback: &Function,
    );

    #[wasm_bindgen(js_name = "pbkdf2Sync")]
    pub fn pbkdf2_sync(
        password: &JsValue,
        salt: &JsValue,
        iterations: u32,
        keylen: u32,
        digest: &str,
    ) -> Buffer;

    #[wasm_bindgen(js_name = "privateDecrypt")]
    pub fn private_decrypt(private_key: &JsValue, buffer: &JsValue) -> Buffer;

    #[wasm_bindgen(js_name = "privateEncrypt")]
    pub fn private_encrypt(private_key: &JsValue, buffer: &JsValue) -> Buffer;

    #[wasm_bindgen(js_name = "publicDecrypt")]
    pub fn public_decrypt(key: &JsValue, buffer: &JsValue) -> Buffer;

    #[wasm_bindgen(js_name = "publicEncrypt")]
    pub fn public_encrypt(key: &JsValue, buffer: &JsValue) -> Buffer;

    #[wasm_bindgen(js_name = "randomBytes")]
    pub fn random_bytes(size: f64, callback: Option<&Function>) -> Buffer;

    #[wasm_bindgen(js_name = "randomFillSync")]
    pub fn random_fill_sync(buffer: &JsValue, offset: Option<f64>, size: Option<f64>) -> JsValue;

    #[wasm_bindgen(js_name = "randomFill")]
    pub fn random_fill(
        buffer: &JsValue,
        offset: Option<f64>,
        size: Option<f64>,
        callback: &Function,
    );

    pub fn scrypt(
        password: &JsValue,
        salt: &JsValue,
        keylen: f64,
        options: Option<ScryptOptions>,
        callback: &Function,
    );

    #[wasm_bindgen(js_name = "scryptSync")]
    pub fn scrypt_sync(
        password: &JsValue,
        salt: &JsValue,
        keylen: f64,
        options: Option<ScryptOptions>,
    ) -> Buffer;

    #[wasm_bindgen(js_name = "setEngine")]
    pub fn set_engine(engine: &str, flags: Option<i32>);

    #[wasm_bindgen(js_name = "setFips")]
    pub fn set_fips(enable: bool);

    pub fn sign(algorithm: &JsValue, data: &JsValue, key: &JsValue) -> Buffer;

    #[wasm_bindgen(js_name = "timingSafeEqual")]
    pub fn timing_safe_equal(a: &JsValue, b: &JsValue) -> bool;

    pub fn verify(algorithm: &JsValue, data: &JsValue, key: &JsValue, signature: &JsValue) -> bool;
}
