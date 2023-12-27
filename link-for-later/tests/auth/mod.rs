use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

const JWT_SECRET_KEY: &str = "JWT_SECRET";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
}

pub fn generate_token(email: &str) -> String {
    let now = Utc::now();
    let claims = Claims {
        sub: email.to_string(),
        iat: now.timestamp() as usize,
        exp: 10000000000,
    };

    let secret = std::env::var(JWT_SECRET_KEY).map_or_else(|_| String::default(), |secret| secret);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();

    token
}
