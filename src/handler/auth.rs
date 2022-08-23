use axum::{
    async_trait,
    body::BoxBody,
    extract::{FromRequest, RequestParts},
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde_json::json;

pub struct Auth;

pub struct AuthError;

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<BoxBody> {
        let status = StatusCode::UNAUTHORIZED;
        let body = json!({"error": "Unauthorized".to_string()}).to_string();
        (status, body).into_response()
    }
}

#[async_trait]
impl<B> FromRequest<B> for Auth
where
    B: Send,
{
    type Rejection = AuthError;

    #[cfg(not(test))]
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        use std::env;

        use axum::{
            extract::TypedHeader,
            headers::{authorization::Bearer, Authorization},
        };

        match TypedHeader::<Authorization<Bearer>>::from_request(req).await {
            Ok(TypedHeader(Authorization(bearer))) => {
                match bearer.token() == env::var("API_KEY").expect("API_KEY is not defined") {
                    true => Ok(Auth {}),
                    false => Err(Self::Rejection {}),
                }
            }
            Err(_) => Err(Self::Rejection {}),
        }
    }

    #[cfg(test)]
    async fn from_request(_: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        Ok(Auth {})
    }
}
