// use async_trait::async_trait;
// use rusty_paseto::prelude::*;
// use serde_json::Value;
// use snafu::ResultExt;
// use std::marker::PhantomData;
//
// use crate::errors::{InternalSnafu, PasetoClaimSnafu, PasetoGenericBuilderSnafu, PasetoGenericParserSnafu, Result};
// use crate::services::token_service::TokenService;
//
// /// Enum to represent the different types of keys that can be used.
// pub enum PasetoKey {
//     Symmetric(PasetoSymmetricKey<V4, Local>),        // For V4.Local (symmetric encryption)
//     AsymmetricPublic(PasetoAsymmetricPublicKey<V4, Public>), // For V4.Public (asymmetric signing - public key)
//     AsymmetricPrivate(PasetoAsymmetricPrivateKey<V4, Public>), // For V4.Public (asymmetric signing - private key)
// }
//
// impl PasetoKey {
//     pub fn from_symmetric(key: Vec<u8>) -> Self {
//         PasetoKey::Symmetric(PasetoSymmetricKey::<V4, Local>::from(Key::from_slice(&key)))
//     }
//
//     pub fn from_asymmetric_private(key: Vec<u8>) -> Self {
//         PasetoKey::AsymmetricPrivate(PasetoAsymmetricPrivateKey::<V4, Public>::from(Key::from_slice(&key)))
//     }
//
//     pub fn from_asymmetric_public(key: Vec<u8>) -> Self {
//         PasetoKey::AsymmetricPublic(PasetoAsymmetricPublicKey::<V4, Public>::from(Key::from_slice(&key)))
//     }
// }
//
// /// A service for generating and validating PASETO tokens.
// pub struct PasetoTokenService<Purpose> {
//     key: PasetoKey,
//     footer: Option<String>,
//     implicit_assertion: Option<String>,
//     _marker: PhantomData<Purpose>,
// }
//
// impl<Purpose> PasetoTokenService<Purpose> {
//     pub fn new(
//         key: PasetoKey,
//         footer: Option<String>,
//         implicit_assertion: Option<String>,
//     ) -> Self {
//         Self {
//             key,
//             footer,
//             implicit_assertion,
//             _marker: PhantomData,
//         }
//     }
// }
//
// #[async_trait]
// impl TokenService for PasetoTokenService<Local> {
//     async fn build(&self, claims: Value) -> Result<String> {
//         let mut builder = PasetoBuilder::<V4, Local>::default();
//
//         if let Some(ref footer) = self.footer {
//             builder.set_footer(Footer::from(footer.as_str()));
//         }
//
//         if let Some(ref assertion) = self.implicit_assertion {
//             builder.set_implicit_assertion(ImplicitAssertion::from(assertion.as_str()));
//         }
//
//         for (key, value) in claims.as_object().unwrap() {
//             builder.set_claim(CustomClaim::try_from((key.clone(), value.clone()))
//                 .context(PasetoClaimSnafu)
//                 .context(InternalSnafu { code: 500_u16 })?);
//         }
//
//         let token = match &self.key {
//             PasetoKey::Symmetric(key) => builder.build(key),
//             _ => unreachable!(),
//         }
//             .context(PasetoGenericBuilderSnafu)
//             .context(InternalSnafu { code: 500_u16 })?;
//
//         Ok(token)
//     }
//
//     async fn parse(&self, token: &str) -> Result<bool> {
//         let mut parser = PasetoParser::<V4, Local>::default();
//
//         if let Some(ref footer) = self.footer {
//             parser.set_footer(Footer::from(footer.as_str()));
//         }
//
//         if let Some(ref assertion) = self.implicit_assertion {
//             parser.set_implicit_assertion(ImplicitAssertion::from(assertion.as_str()));
//         }
//
//         let result = match &self.key {
//             PasetoKey::Symmetric(key) => parser.parse(token, key),
//             _ => unreachable!(),
//         }
//             .context(PasetoGenericParserSnafu)
//             .context(InternalSnafu { code: 500_u16 });
//
//         match result {
//             Ok(_) => Ok(true),
//             Err(_) => Ok(false),
//         }
//     }
// }
//
// #[async_trait]
// impl TokenService for PasetoTokenService<Public> {
//     async fn build(&self, claims: Value) -> Result<String> {
//         let mut builder = PasetoBuilder::<V4, Public>::default();
//
//         if let Some(ref footer) = self.footer {
//             builder.set_footer(Footer::from(footer.as_str()));
//         }
//
//         if let Some(ref assertion) = self.implicit_assertion {
//             builder.set_implicit_assertion(ImplicitAssertion::from(assertion.as_str()));
//         }
//
//         for (key, value) in claims.as_object().unwrap() {
//             builder.set_claim(CustomClaim::try_from((key.clone(), value.clone()))
//                 .context(PasetoClaimSnafu)
//                 .context(InternalSnafu { code: 500_u16 })?);
//         }
//
//         let token = match &self.key {
//             PasetoKey::AsymmetricPrivate(key) => builder.build(key),
//             _ => unreachable!(),
//         }
//             .context(PasetoGenericBuilderSnafu)
//             .context(InternalSnafu { code: 500_u16 })?;
//
//         Ok(token)
//     }
//
//     async fn parse(&self, token: &str) -> Result<bool> {
//         let mut parser = PasetoParser::<V4, Public>::default();
//
//         if let Some(ref footer) = self.footer {
//             parser.set_footer(Footer::from(footer.as_str()));
//         }
//
//         if let Some(ref assertion) = self.implicit_assertion {
//             parser.set_implicit_assertion(ImplicitAssertion::from(assertion.as_str()));
//         }
//
//         let result = match &self.key {
//             PasetoKey::AsymmetricPublic(key) => parser.parse(token, key),
//             // PasetoKey::AsymmetricPrivate(key) => parser.parse(token, key),
//             _ => unreachable!(),
//         }
//             .context(PasetoGenericParserSnafu)
//             .context(InternalSnafu { code: 500_u16 });
//
//         match result {
//             Ok(_) => Ok(true),
//             Err(_) => Ok(false),
//         }
//     }
// }
