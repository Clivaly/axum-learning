use axum::http::StatusCode;
use chrono::{Duration, Utc};
use dotenvy_macro::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    // aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
                // iss: String, // Optional. Issuer
                // nbf: usize, // Optional. Not Before (as UTC timestamp)
                // sub: String, // Optional. Subject (whom token refers to)
}

pub fn create_jwt() -> Result<String, StatusCode> {
    let mut now = Utc::now();
    let iat = now.timestamp() as usize;

    let expires_in = Duration::seconds(30);
    now += expires_in;
    let exp = now.timestamp() as usize;

    let claim = Claims { exp, iat };

    let secret: &'static str = dotenv!("JWT_SECRET");
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&Header::default(), &claim, &key).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn is_valid(token: &str) -> Result<bool, AppError> {
    let secret: &'static str = dotenv!("JWT_SECRET");
    let key = DecodingKey::from_secret(secret.as_bytes());
    decode::<Claims>(token, &key, &Validation::new(Algorithm::HS256)).map_err(
        |error| match error.kind() {
            // jsonwebtoken::errors::ErrorKind::InvalidToken => todo!(),
            // jsonwebtoken::errors::ErrorKind::InvalidSignature => todo!(),
            // jsonwebtoken::errors::ErrorKind::InvalidEcdsaKey => todo!(),
            // jsonwebtoken::errors::ErrorKind::RsaFailedSigning => todo!(),
            // jsonwebtoken::errors::ErrorKind::InvalidAlgorithmName => todo!(),
            // jsonwebtoken::errors::ErrorKind::InvalidKeyFormat => todo!(),
            // jsonwebtoken::errors::ErrorKind::MissingRequiredClaim(_) => todo!(),
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::new(
                StatusCode::UNAUTHORIZED,
                "Your session has expired, please login again",
            ),
            // jsonwebtoken::errors::ErrorKind::InvalidIssuer => todo!(),
            // jsonwebtoken::errors::ErrorKind::InvalidAudience => todo!(),
            // jsonwebtoken::errors::ErrorKind::InvalidSubject => todo!(),
            // jsonwebtoken::errors::ErrorKind::ImmatureSignature => todo!(),
            // jsonwebtoken::errors::ErrorKind::InvalidAlgorithm => todo!(),
            // jsonwebtoken::errors::ErrorKind::MissingAlgorithm => todo!(),
            // jsonwebtoken::errors::ErrorKind::Base64(_) => todo!(),
            // jsonwebtoken::errors::ErrorKind::Json(_) => todo!(),
            // jsonwebtoken::errors::ErrorKind::Utf8(_) => todo!(),
            // jsonwebtoken::errors::ErrorKind::Crypto(_) => todo!(),
            _ => AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again",
            ),
        },
    )?;

    Ok(true)
}
