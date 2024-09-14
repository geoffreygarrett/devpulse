// use std::collections::HashSet;
//
// use crate::util::{get_env_var, get_env_var_optional, get_env_var_with_default};
// use expunge::Expunge;
// use jsonwebtoken::Algorithm;
//
// /// Configuration for JWT service.
// #[derive(Clone, Expunge)]
// pub struct AuthConfig {
//     #[expunge(as = "<expunged>".to_string(), zeroize)]
//     pub secret: String,
//     pub expiration: usize, // Expiration time in seconds
//     pub private_key: Option<String>,
//     pub public_key: Option<String>,
//     pub leeway: u64,
//     pub reject_tokens_expiring_in_less_than: u64,
//     pub validate_exp: bool,
//     pub validate_nbf: bool,
//     pub validate_aud: bool,
//     pub aud: Option<HashSet<String>>,
//     pub iss: Option<HashSet<String>>,
//     pub sub: Option<String>,
//     pub algorithms: Vec<Algorithm>,
// }
//
// impl AuthConfig {
//     pub fn from_env() -> Self {
//         Self {
//             secret: get_env_var("JWT_SECRET").expect("JWT_SECRET must be set"),
//             expiration: get_env_var_with_default("JWT_EXPIRATION", 3600),
//             private_key: get_env_var_optional("JWT_PRIVATE_KEY_PATH"),
//             public_key: get_env_var_optional("JWT_PUBLIC_KEY_PATH"),
//             leeway: get_env_var_with_default("JWT_LEEWAY", 60),
//             reject_tokens_expiring_in_less_than: get_env_var_with_default(
//                 "JWT_REJECT_TOKENS_EXPIRING_IN_LESS_THAN",
//                 0,
//             ),
//             validate_exp: get_env_var_with_default("JWT_VALIDATE_EXP", true),
//             validate_nbf: get_env_var_with_default("JWT_VALIDATE_NBF", false),
//             validate_aud: get_env_var_with_default("JWT_VALIDATE_AUD", true),
//             aud: get_env_var_optional("JWT_AUD")
//                 .map(|v| v.split(',').map(|s| s.to_string()).collect()),
//             iss: get_env_var_optional("JWT_ISS")
//                 .map(|v| v.split(',').map(|s| s.to_string()).collect()),
//             sub: get_env_var_optional("JWT_SUB"),
//             algorithms: vec![Algorithm::HS256],
//         }
//     }
// }
//
//
// // Implementation to create AuthConfig from jsonwebtoken::Validation
// impl From<jsonwebtoken::Validation> for AuthConfig {
//     fn from(validation: jsonwebtoken::Validation) -> Self {
//         AuthConfig {
//             secret: String::new(), // Placeholder as secret will not be used in this case
//             expiration: 3600,      // Default expiration time
//             private_key: None,
//             public_key: None,
//             leeway: validation.leeway,
//             reject_tokens_expiring_in_less_than: validation.reject_tokens_expiring_in_less_than,
//             validate_exp: validation.validate_exp,
//             validate_nbf: validation.validate_nbf,
//             validate_aud: validation.validate_aud,
//             aud: validation.aud,
//             iss: validation.iss,
//             sub: validation.sub,
//             algorithms: validation.algorithms,
//         }
//     }
// }
//
// // Implementation to convert AuthConfig to jsonwebtoken::Validation
// impl Into<jsonwebtoken::Validation> for AuthConfig {
//     fn into(self) -> jsonwebtoken::Validation {
//         let mut validation = jsonwebtoken::Validation::new(self.algorithms[0]);
//         validation.leeway = self.leeway;
//         validation.reject_tokens_expiring_in_less_than = self.reject_tokens_expiring_in_less_than;
//         validation.validate_exp = self.validate_exp;
//         validation.validate_nbf = self.validate_nbf;
//         validation.validate_aud = self.validate_aud;
//         validation.aud = self.aud;
//         validation.iss = self.iss;
//         validation.sub = self.sub;
//         validation
//     }
// }
