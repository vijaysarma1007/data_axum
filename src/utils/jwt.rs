use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    exp: usize,
    iat: usize,
}

pub fn create_jwt() -> Result<String, StatusCode> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::seconds(30);
    now += expires_in;
    let claim = Claims {
        exp: now.timestamp() as usize,
        iat,
    };
    let secret = dotenv!("JWT_SECRET");
    let key = EncodingKey::from_secret(secret.as_bytes());
    let token = encode(&Header::default(), &claim, &key)
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(token)
}

pub fn is_valid(token: &str) -> Result<bool, StatusCode> {
    let secret = dotenv!("JWT_SECRET");
    let key = DecodingKey::from_secret(secret.as_bytes());
    decode::<Claims>(
        token,
        &key,
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map_err(|error| match error.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => StatusCode::UNAUTHORIZED,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(true)
}
