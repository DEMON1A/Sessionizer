use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::Value;
use base64::{Engine as _, engine::general_purpose};
use ring::hmac;

pub fn decode_jwt(token: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let validation = Validation::default();
    let s = secret.as_ref();
    let _decoded = decode::<Value>(&token, &DecodingKey::from_secret(s), &validation)?;
    // println!("{:?}", decoded.claims);
    Ok(String::from(secret))
}

pub fn decode_flask_cookie(cookie: &str, secret: &str, salt: &str, hmac_digest: hmac::Algorithm) -> Result<String, ()> {
    let mut parts = cookie.rsplitn(2, '.');
    let signature = general_purpose::STANDARD_NO_PAD.decode(parts.next().unwrap()).unwrap();
    let payload = parts.next().unwrap();
    // Verify signature
    // Flask defaults (https://github.com/pallets/flask/blob/master/src/flask/sessions.py):
    // salt = "cookie-session"
    // digest_method = staticmethod(hashlib.sha1)
    // key_derivation = "hmac"
    let key = hmac::Key::new(hmac_digest, secret.as_bytes());
    let derived_key = hmac::sign(&key, salt.as_bytes());
    let key = hmac::Key::new(hmac_digest, derived_key.as_ref());
    let verified = hmac::verify(&key, payload.as_bytes(), &signature);
    
    // println!("{:?}", verified.is_ok());
    if verified.is_ok() {
        Ok(String::from(secret))
    } else {
        Err(())
    }
}