use chrono::{Duration, Utc};
use clap::Parser;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config;
use crate::config::env::JWT_SECRET;
use crate::error::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(id: Uuid, exp: i64) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::days(exp);

        Self {
            sub: id,
            exp: exp.timestamp(),
            iat: iat.timestamp(),
        }
    }
}

pub fn sign(id: String) -> Result<String> {
    let uuid = Uuid::parse_str(&id)?;
    let config = config::env::JwtConfig::parse();
    Ok(jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(uuid, config.expires_time),
        &EncodingKey::from_secret(config.signing_key.as_bytes()),
    )?)
}

pub fn verify(token: &str) -> Result<Claims> {
    Ok(jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)?)
}
