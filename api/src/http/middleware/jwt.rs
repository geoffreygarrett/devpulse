use axum::{
    async_trait,
    extract::FromRequest,
    http::StatusCode

    ,
};
use axum_core::extract::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, state: &B) -> Result<Self, Self::Rejection> {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;

        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret("your-256-bit-secret".as_ref()),
            &jsonwebtoken::Validation::default(),
        )
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

        Ok(token_data.claims)
    }
}
