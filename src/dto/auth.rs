use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
};
use chrono::Duration;
use jsonwebtoken::{
    decode, decode_header,
    jwk::{AlgorithmParameters, JwkSet},
    DecodingKey, Validation,
};
use serde::{Deserialize, Serialize};

use crate::{
    config::OauthSettings,
    redis::{self, RedisPool},
    startup::AppState,
    AppError,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: i64,
    pub iat: i64,
    pub acr: String,
}

#[derive(Deserialize)]
pub struct AuthRequest {
    pub access_token: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub struct UserInfo {
    pub email: String,
    pub email_verified: bool,
    pub sub: String,
}

#[derive(Deserialize)]
pub struct BearerToken {
    pub access_token: String,
}

#[async_trait]
impl FromRequestParts<Arc<AppState>> for Claims {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let reject = || AppError::AuthorizationError("Unauthorized".to_string());
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| reject())?;
        let claims = get_claims(&state.redis_pool, &state.oauth_settings, bearer.token()).await?;
        Ok(claims)
    }
}

pub async fn get_claims(
    redis_pool: &RedisPool,
    oauth_settings: &OauthSettings,
    token: &str,
) -> Result<Claims, AppError> {
    let reject = || AppError::AuthorizationError("Unauthorized".to_string());
    let jwks = redis::command(redis_pool, "jwks").get().await?;
    let jwks = match jwks {
        Some(jwks) => jwks,
        None => {
            let jwks = reqwest::get(&oauth_settings.jwks_url)
                .await
                .map_err(|err| AppError::InternalError(err.into()))?
                .json::<JwkSet>()
                .await
                .map_err(|err| AppError::InternalError(err.into()))?;
            redis::command(redis_pool, "jwks")
                .set(&jwks)
                .await
                .map_err(|err| AppError::InternalError(err.into()))?
                .expire(Duration::days(1))
                .await
                .map_err(|err| AppError::InternalError(err.into()))?;
            jwks
        }
    };
    let header = decode_header(token).map_err(|_| reject())?;
    let kid = header.kid.ok_or_else(reject)?;
    let jwk = jwks.find(&kid).ok_or_else(reject)?;
    let claims = match &jwk.algorithm {
        AlgorithmParameters::RSA(rsa) => {
            let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)?;
            let mut validation = Validation::new(jwk.common.algorithm.unwrap());
            validation.set_audience(&[&oauth_settings.audience]);
            validation.set_issuer(&[&oauth_settings.issuer]);
            decode::<Claims>(token, &decoding_key, &validation)?.claims
        }
        _ => return Err(reject()),
    };
    Ok(claims)
}

#[async_trait]
impl FromRequestParts<Arc<AppState>> for UserInfo {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let reject = || AppError::AuthorizationError("Invalid bearer token".to_string());
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| reject())?;
        let claims = get_claims(&state.redis_pool, &state.oauth_settings, bearer.token()).await?;
        let user = redis::command(&state.redis_pool, &format!("user_info|{}", claims.sub))
            .get()
            .await?;
        let user = match user {
            Some(user) => user,
            None => {
                let user = reqwest::Client::new()
                    .get(state.oauth_settings.userinfo_url.as_str())
                    .bearer_auth(bearer.token())
                    .send()
                    .await
                    .map_err(|_| reject())?
                    .json::<UserInfo>()
                    .await
                    .map_err(|_| reject())?;
                redis::command(&state.redis_pool, &format!("user_info|{}", claims.sub))
                    .set(&user)
                    .await?
                    .expire(Duration::days(1))
                    .await?;
                user
            }
        };
        Ok(user)
    }
}

#[async_trait]
impl FromRequestParts<Arc<AppState>> for BearerToken {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let reject = || AppError::AuthorizationError("Invalid bearer token".to_string());
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| reject())?;
        let bearer_token = BearerToken {
            access_token: bearer.token().to_string(),
        };
        Ok(bearer_token)
    }
}
