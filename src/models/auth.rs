use actix_http::header::HeaderMap;
use chrono::Local;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{env, time::Duration};
use uuid::Uuid;

use crate::helpers::get_token::get_token;

use super::user::User;

pub struct Auth;

impl Auth {
    pub fn decode_token(header: &HeaderMap) -> Result<AuthClaims, &'static str> {
        let secret = env::var("SECRET_JWT").unwrap();

        let token = match get_token(header) {
            Ok(token) => token,
            Err(_) => return Err("Token not found")
        };

        let user_payload: AuthClaims = match decode::<AuthClaims>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        ) {
            Ok(user_payload) => user_payload.claims,
            Err(_) => return Err("Invalid bearer token")
        };

        Ok(user_payload)
    }

    pub fn encode_token(user: &User) -> Result<String, &'static str> {
        let secret = env::var("SECRET_JWT").unwrap();
        let now = Local::now();
        let exp = (now + Duration::from_secs(10800)).timestamp();

        let user_claims = AuthClaims {
            user_id: user.id,
            exp,
        };

        match encode(
            &Header::default(),
            &user_claims,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(token) => Ok(token),
            Err(_) => Err("Something went wrong encoding token"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthClaims {
    pub user_id: Uuid,
    pub exp: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AuthPayload {
    pub token: String,
}
