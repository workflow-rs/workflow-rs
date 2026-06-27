use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Clone, Debug)]
    pub type CryptoConstants;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn ALPN_ENABLED(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn DH_CHECK_P_NOT_PRIME(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn DH_CHECK_P_NOT_SAFE_PRIME(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn DH_NOT_SUITABLE_GENERATOR(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn DH_UNABLE_TO_CHECK_GENERATOR(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn ENGINE_METHOD_ALL(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn ENGINE_METHOD_CIPHERS(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn ENGINE_METHOD_DH(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn ENGINE_METHOD_DIGESTS(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn ENGINE_METHOD_DSA(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn ENGINE_METHOD_EC(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn ENGINE_METHOD_NONE(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn ENGINE_METHOD_PKEY_ASN1_METHS(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn ENGINE_METHOD_PKEY_METHS(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn ENGINE_METHOD_RAND(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn ENGINE_METHOD_RSA(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn OPENSSL_VERSION_NUMBER(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn RSA_NO_PADDING(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn RSA_PKCS1_OAEP_PADDING(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn RSA_PKCS1_PADDING(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn RSA_PKCS1_PSS_PADDING(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn RSA_PSS_SALTLEN_AUTO(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn RSA_PSS_SALTLEN_DIGEST(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn RSA_PSS_SALTLEN_MAX_SIGN(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn RSA_SSLV23_PADDING(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn RSA_X931_PADDING(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_ALL(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_ALLOW_UNSAFE_LEGACY_RENEGOTIATION(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_CIPHER_SERVER_PREFERENCE(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_CISCO_ANYCONNECT(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_COOKIE_EXCHANGE(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_CRYPTOPRO_TLSEXT_BUG(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_DONT_INSERT_EMPTY_FRAGMENTS(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_EPHEMERAL_RSA(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_LEGACY_SERVER_CONNECT(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_MICROSOFT_BIG_SSLV3_BUFFER(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_MICROSOFT_SESS_ID_BUG(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_MSIE_SSLV2_RSA_PADDING(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_NETSCAPE_CA_DN_BUG(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_NETSCAPE_CHALLENGE_BUG(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_NETSCAPE_DEMO_CIPHER_CHANGE_BUG(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_NETSCAPE_REUSE_CIPHER_CHANGE_BUG(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_NO_COMPRESSION(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_NO_QUERY_MTU(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_NO_SESSION_RESUMPTION_ON_RENEGOTIATION(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_NO_SSLv2(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_NO_SSLv3(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_NO_TICKET(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_NO_TLSv1_1(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_NO_TLSv1_2(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_NO_TLSv1(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_PKCS1_CHECK_1(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_PKCS1_CHECK_2(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_SINGLE_DH_USE(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_SINGLE_ECDH_USE(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_SSLEAY_080_CLIENT_DH_BUG(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_SSLREF2_REUSE_CERT_TYPE_BUG(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_TLS_BLOCK_PADDING_BUG(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_TLS_D5_BUG(this: &CryptoConstants) -> i32;

    #[allow(non_snake_case)]
    #[wasm_bindgen(method, getter)]
    pub fn SSL_OP_TLS_ROLLBACK_BUG(this: &CryptoConstants) -> i32;
}
