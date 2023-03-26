use crate::{dto::Claims, AppError};
use jsonwebtoken::{DecodingKey, Validation};
use secrecy::{ExposeSecret, Secret};
use serde::de::DeserializeOwned;

fn decode_token<T: DeserializeOwned>(
    secret: Secret<String>,
    refresh_token: &str,
) -> Result<T, AppError> {
    let key = &DecodingKey::from_secret(secret.expose_secret().as_bytes());
    let claims = jsonwebtoken::decode::<T>(refresh_token, key, &Validation::default())
        .map_err(|_| AppError::AuthorizationError("Unauthorized".to_string()))?
        .claims;
    Ok(claims)
}

pub async fn verify_access_token(secret: Secret<String>, token: &str) -> Result<Claims, AppError> {
    let claims = decode_token::<Claims>(secret, token)?;
    Ok(claims)
}
