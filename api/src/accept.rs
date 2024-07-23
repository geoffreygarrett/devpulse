use std::future::Future;
use std::ops::{Deref, DerefMut};

use axum::http::{header, HeaderValue, StatusCode};
use axum::{extract::FromRequest, response::IntoResponse};
use axum_core::body::Body;
use axum_core::response::Response;
use quick_xml::se as serde_xml;
use serde::{de::DeserializeOwned, Serialize};

use crate::accept::error::header::HeaderMap;

// export macro
#[macro_export]
macro_rules! application_vnd_devpulse_v1_json {
    () => {
        "application/vnd.devpulse.v1+json"
    };
}

#[macro_export]
macro_rules! application_vnd_devpulse_v1_yaml {
    () => {
        "application/vnd.devpulse.v1+yaml"
    };
}

#[macro_export]
macro_rules! application_vnd_devpulse_v1_xml {
    () => {
        "application/vnd.devpulse.v1+xml"
    };
}

#[macro_export]
macro_rules! text_vnd_devpulse_v1_toml {
    () => {
        "text/vnd.devpulse.v1+toml"
    };
}

#[macro_export]
macro_rules! my_content (
    ( $input:expr) => {
        #[content($input)]
    }
);

#[macro_export]
macro_rules! create_response_enum {
    ($name:ident, $content_type:expr, $data_type:ty) => {
        #[derive(utoipa::ToResponse, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
        #[response(description = $content_type)]
        pub(crate) enum $name {
            Json(
                #[content("application/vnd.devpulse.v1+json")]
                #[schema(example = json!({"status": "ok", "uptime": 1234}))]
                $data_type,
            ),

            Yaml(
                #[content("application/vnd.devpulse.v1+yaml")]
                #[schema(example = "status: ok\nuptime: 1234")]
                $data_type,
            ),

            Xml(
                #[content("application/vnd.devpulse.v1+xml")]
                #[schema(example = "<status>ok</status><uptime>1234</uptime>")]
                $data_type,
            ),

            Toml(
                #[content("text/vnd.devpulse.v1+toml")]
                #[schema(example = "status = \"ok\"\nuptime = 1234")]
                $data_type,
            ),
        }
    };
}

pub fn serialize_response<T>(result: &T, headers: &HeaderMap) -> Response<Body>
where
    T: Serialize,
{
    let accept_header = headers
        .get(header::ACCEPT)
        .and_then(|value| value.to_str().ok());
    match accept_header {
        Some("application/vnd.devpulse.v1+xml") => XmlV1(result).into_response(),
        Some("application/vnd.devpulse.v1+yaml") => YamlV1(result).into_response(),
        Some("text/vnd.devpulse.v1+toml") => TomlV1(result).into_response(),
        Some("application/vnd.devpulse.v1+json") | _ => JsonV1(result).into_response(),
    }
}

mod error {
    pub(crate) use axum::http::{header, HeaderValue, StatusCode};
    pub(crate) use axum_core::response::{IntoResponse, Response};
    pub(crate) use snafu::{prelude::*, ResultExt, Snafu};

    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(crate)))]
    pub enum ContentTypeError {
        #[snafu(display("Missing Content-Type: {content_type}"))]
        MissingContentType { content_type: String },

        #[snafu(display("Invalid JSON body"))]
        InvalidJsonBody { source: serde_json::Error },

        #[snafu(display("Invalid YAML body"))]
        InvalidYamlBody { source: serde_yaml::Error },

        #[snafu(display("Invalid XML body"))]
        InvalidXmlBody { source: serde_xml_rs::Error },

        #[snafu(display("Invalid TOML body"))]
        InvalidTomlBody { source: toml::de::Error },

        #[snafu(display("UTF-8 error"))]
        Utf8Error { source: std::str::Utf8Error },

        #[snafu(display("Bytes error"))]
        BytesError {
            source: axum_core::extract::rejection::BytesRejection,
        },
    }

    impl IntoResponse for ContentTypeError {
        fn into_response(self) -> Response {
            let (status, msg) = match &self {
                ContentTypeError::MissingContentType { .. } => {
                    (StatusCode::BAD_REQUEST, self.to_string())
                }
                ContentTypeError::InvalidJsonBody { .. } => {
                    (StatusCode::BAD_REQUEST, self.to_string())
                }
                ContentTypeError::InvalidYamlBody { .. } => {
                    (StatusCode::BAD_REQUEST, self.to_string())
                }
                ContentTypeError::InvalidXmlBody { .. } => {
                    (StatusCode::BAD_REQUEST, self.to_string())
                }
                ContentTypeError::InvalidTomlBody { .. } => {
                    (StatusCode::BAD_REQUEST, self.to_string())
                }
                ContentTypeError::Utf8Error { .. } => (StatusCode::BAD_REQUEST, self.to_string()),
                ContentTypeError::BytesError { source } => {
                    (StatusCode::BAD_REQUEST, source.to_string())
                }
            };
            (
                status,
                [
                    (
                        header::CONTENT_TYPE,
                        HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                    ),
                ],
                msg,
            )
                .into_response()
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct JsonV1<T>(pub T);

#[derive(Debug, Clone, Copy, Default)]
pub struct YamlV1<T>(pub T);

#[derive(Debug, Clone, Copy, Default)]
pub struct XmlV1<T>(pub T);

#[derive(Debug, Clone, Copy, Default)]
pub struct TomlV1<T>(pub T);

/// Generic implementation for deserializing JSON, YAML, or XML requests
macro_rules! impl_from_request {
    ($type:ident, $content_type:expr, $parser:ident, $method:expr, $invalid_body_variant:ident) => {
        impl<T, S> FromRequest<S> for $type<T>
        where
            T: DeserializeOwned,
            S: Send + Sync,
        {
            type Rejection = ContentTypeError;

            fn from_request<'state, 'future>(
                req: Request<Body>, state: &'state S,
            ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Rejection>> + Send + 'future>>
            where
                'state: 'future,
                Self: 'future,
            {
                Box::pin(async move {
                    let content_type = req.headers().get(header::CONTENT_TYPE);
                    if content_type != Some(&HeaderValue::from_static($content_type)) {
                        return Err(ContentTypeError::MissingContentType {
                            content_type: $content_type.to_string(),
                        });
                    }

                    let bytes = Bytes::from_request(req, state).await.context(BytesSnafu)?;
                    let value = $method(&bytes).context($invalid_body_variant)?;

                    Ok(Self(value))
                })
            }
        }
    };
}

mod impl_from {
    use std::pin::Pin;

    use axum_core::body::Body;
    use axum_core::extract::Request;
    use bytes::Bytes;
    use serde::de::Error;

    use crate::application_vnd_devpulse_v1_json;
    use crate::application_vnd_devpulse_v1_xml;
    use crate::application_vnd_devpulse_v1_yaml;
    use crate::text_vnd_devpulse_v1_toml;

    use super::{error::*, *};

    impl_from_request!(
        JsonV1,
        application_vnd_devpulse_v1_json!(),
        serde_json,
        |bytes: &Bytes| serde_json::from_slice(bytes),
        InvalidJsonBodySnafu
    );
    impl_from_request!(
        YamlV1,
        application_vnd_devpulse_v1_yaml!(),
        serde_yaml,
        |bytes: &Bytes| serde_yaml::from_slice(bytes),
        InvalidYamlBodySnafu
    );
    impl_from_request!(
        XmlV1,
        application_vnd_devpulse_v1_xml!(),
        serde_xml_rs,
        |bytes: &Bytes| {
            let reader = bytes.as_ref();
            serde_xml_rs::from_reader(reader)
        },
        InvalidXmlBodySnafu
    );
    impl_from_request!(
        TomlV1,
        text_vnd_devpulse_v1_toml!(),
        toml,
        |bytes: &Bytes| {
            std::str::from_utf8(bytes)
                .map_err(|e| toml::de::Error::custom(e.to_string()))
                .and_then(|s| toml::from_str::<T>(s))
        },
        InvalidTomlBodySnafu
    );
}

/// Generic implementation for serializing JSON, YAML, or XML responses
macro_rules! impl_into_response {
    ($type:ident, $content_type:expr, $serializer:ident) => {
        impl<T> IntoResponse for $type<T>
        where
            T: Serialize,
        {
            fn into_response(self) -> Response {
                match $serializer::to_string(&self.0) {
                    Ok(value) => {
                        ([(header::CONTENT_TYPE, HeaderValue::from_static($content_type))], value)
                            .into_response()
                    }
                    Err(err) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        [(
                            header::CONTENT_TYPE,
                            HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                        )],
                        err.to_string(),
                    )
                        .into_response(),
                }
            }
        }
    };
}

impl_into_response!(JsonV1, application_vnd_devpulse_v1_json!(), serde_json);
impl_into_response!(YamlV1, application_vnd_devpulse_v1_yaml!(), serde_yaml);
impl_into_response!(XmlV1, application_vnd_devpulse_v1_xml!(), serde_xml);
impl_into_response!(TomlV1, text_vnd_devpulse_v1_toml!(), toml);

/// Deref and DerefMut implementations for convenience
macro_rules! impl_deref {
    ($type:ident) => {
        impl<T> Deref for $type<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> DerefMut for $type<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<T> From<T> for $type<T> {
            fn from(inner: T) -> Self {
                Self(inner)
            }
        }
    };
}

impl_deref!(JsonV1);
impl_deref!(YamlV1);
impl_deref!(XmlV1);
impl_deref!(TomlV1);
