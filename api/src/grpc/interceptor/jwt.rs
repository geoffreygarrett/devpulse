use tonic::{Request, Status, service::Interceptor};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct JwtInterceptor {
    pub secret: String,
}

impl Interceptor for JwtInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        let token = request
            .metadata()
            .get("authorization")
            .and_then(|t| t.to_str().ok())
            .and_then(|t| t.strip_prefix("Bearer "))
            .ok_or_else(|| Status::unauthenticated("Missing token"))?;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        ).map_err(|_| Status::unauthenticated("Invalid token"))?;

        request.extensions_mut().insert(token_data.claims);
        Ok(request)
    }
}
