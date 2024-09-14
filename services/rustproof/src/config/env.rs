//! Envy is a library for deserializing environment variables into typesafe structs
//!
//! # Examples
//!
//! A typical usecase for envy is deserializing configuration store in an process' environment into a struct
//! whose fields map to the names of env vars.
//!
//! Serde makes it easy to provide a deserializable struct with its [deriveable Deserialize](https://serde.rs/derive.html)
//! procedural macro.
//!
//! Simply ask for an instance of that struct from envy's `from_env` function.
//!
//! ```no_run
//! use serde::Deserialize;
//!
//! #[derive(Deserialize, Debug)]
//! struct Config {
//!     foo: u16,
//!     bar: bool,
//!     baz: String,
//!     boom: Option<u64>,
//! }
//!
//! match envy::from_env::<Config>() {
//!     Ok(config) => println!("{:#?}", config),
//!     Err(error) => eprintln!("{:#?}", error),
//! }
//! ```
//!
//! Special treatment is given to collections. For config fields that store a `Vec` of values,
//! use an env var that uses a comma separated value.
//!
//! All serde modifiers should work as is.
//!
//! Enums with unit variants can be used as values:
//!
//! ```no_run
//! # use serde::Deserialize;
//!
//! #[derive(Deserialize, Debug, PartialEq)]
//! #[serde(rename_all = "lowercase")]
//! pub enum Size {
//!     Small,
//!     Medium,
//!     Large,
//! }
//!
//! #[derive(Deserialize, Debug)]
//! struct Config {
//!     size: Size,
//! }
//!
//! // set env var for size as `SIZE=medium`
//! match envy::from_env::<Config>() {
//!     Ok(config) => println!("{:#?}", config),
//!     Err(error) => eprintln!("{:#?}", error),
//! }
//! ```

use serde::de::Error as SerdeError;
use serde::de::{
    self,
    value::{MapDeserializer, SeqDeserializer},
    IntoDeserializer,
};
use std::{
    borrow::Cow,
    env,
    iter::{empty, IntoIterator},
};
use std::{error::Error as StdError, fmt};

/// Types of errors that may result from failed attempts
/// to deserialize a type from env vars
// #[derive(Debug, Clone, PartialEq)]
// pub enum Error {
//     MissingValue(&'static str),
//     Custom(String),
// }
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    MissingValue {
        field: &'static str,
        env_var: Option<String>,  // Optional to capture prefixed or nested field name
    },
    Custom(String),
}

impl StdError for Error {}

// impl fmt::Display for Error {
//     fn fmt(
//         &self,
//         fmt: &mut fmt::Formatter,
//     ) -> fmt::Result {
//         match *self {
//             Error::MissingValue(field) => write!(fmt, "missing value for field {}", field),
//             Error::Custom(ref msg) => write!(fmt, "{}", msg),
//         }
//     }
// }


impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::MissingValue { field, env_var } => {
                if let Some(ref var) = env_var {
                    write!(
                        fmt,
                        "missing value for field `{}`.\n\nExpected environment variable: `{}`\n\nHint: Ensure that `{}` is set in your environment, e.g.,\nexport {}=\"<your-value>\"",
                        field, var, var, var
                    )
                } else {
                    write!(fmt, "missing value for field `{}`.", field)
                }
            }
            Error::Custom(ref msg) => write!(fmt, "{}", msg),
        }
    }
}
impl SerdeError for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(format!("{}", msg))
    }

    fn missing_field(field: &'static str) -> Error {
        Error::MissingValue {
            field,
            env_var: Some(field.to_string().to_uppercase()),  // Placeholder: You can customize this
        }
    }
}

// Ours
// mod error;
// pub use crate::error::Error;

/// A type result type specific to `envy::Errors`
pub type Result<T> = std::result::Result<T, Error>;

struct Vars<Iter>(Iter)
where
    Iter: IntoIterator<Item=(String, String)>;

#[derive(Debug)]
struct Val(String, String);

impl<'de> IntoDeserializer<'de, Error> for Val {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

#[derive(Debug)]
struct VarName(String);

impl<'de> IntoDeserializer<'de, Error> for VarName {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
    }
}

impl<Iter: Iterator<Item=(String, String)>> Iterator for Vars<Iter> {
    type Item = (VarName, Val);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(k, v)| {
            let key = k.to_lowercase();
            (VarName(key.clone()), Val(k, v))
        })
    }
}


// macro_rules! forward_parsed_values {
//     ($($ty:ident => $method:ident,)*) => {
//         $(
//             fn $method<V>(self, visitor: V) -> Result<V::Value>
//                 where V: de::Visitor<'de>
//             {
//                 match self.1.parse::<$ty>() {
//                     Ok(val) => val.into_deserializer().$method(visitor),
//                     Err(e) => Err(de::Error::custom(format_args!("{} while parsing value '{}' provided by {}", e, self.1, self.0)))
//                 }
//             }
//         )*
//     }
// }
macro_rules! forward_parsed_values {
    ($($ty:ident => $method:ident,)*) => {
        $(
            fn $method<V>(self, visitor: V) -> Result<V::Value>
                where V: de::Visitor<'de>
            {
                match self.1.parse::<$ty>() {
                    Ok(val) => val.into_deserializer().$method(visitor),
                    Err(e) => Err(de::Error::custom(format_args!("{} while parsing value '{}' provided by {}", e, self.1, self.0)))
                }
            }
        )*
    }
}
impl<'de> de::Deserializer<'de> for Val {
    type Error = Error;
    fn deserialize_any<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.1.into_deserializer().deserialize_any(visitor)
    }

    fn deserialize_seq<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        // std::str::split doesn't work as expected for our use case: when we
        // get an empty string we want to produce an empty Vec, but split would
        // still yield an iterator with an empty string in it. So we need to
        // special case empty strings.
        if self.1.is_empty() {
            SeqDeserializer::new(empty::<Val>()).deserialize_seq(visitor)
        } else {
            let values = self.1.split(',').map(|v| Val(self.0.clone(), v.to_owned()));
            SeqDeserializer::new(values).deserialize_seq(visitor)
        }
    }

    fn deserialize_option<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    forward_parsed_values! {
        bool => deserialize_bool,
        u8 => deserialize_u8,
        u16 => deserialize_u16,
        u32 => deserialize_u32,
        u64 => deserialize_u64,
        i8 => deserialize_i8,
        i16 => deserialize_i16,
        i32 => deserialize_i32,
        i64 => deserialize_i64,
        f32 => deserialize_f32,
        f64 => deserialize_f64,
    }

    #[inline]
    fn deserialize_newtype_struct<V>(
        self,
        _: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(self.1.into_deserializer())
    }

    serde::forward_to_deserialize_any! {
        char str string unit
        bytes byte_buf map unit_struct tuple_struct
        identifier tuple ignored_any
        struct
    }
}


impl<'de> de::Deserializer<'de> for VarName {
    type Error = Error;

    fn deserialize_any<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.0.into_deserializer().deserialize_any(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    serde::forward_to_deserialize_any! {
        char str string unit seq option
        bytes byte_buf map unit_struct tuple_struct
        identifier tuple ignored_any enum
        struct bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64
    }
}

/// A deserializer for env vars
struct Deserializer<'de, Iter: Iterator<Item=(String, String)>> {
    inner: MapDeserializer<'de, Vars<Iter>, Error>,
}

impl<'de, Iter: Iterator<Item=(String, String)>> Deserializer<'de, Iter> {
    fn new(vars: Iter) -> Self {
        Deserializer {
            inner: MapDeserializer::new(Vars(vars)),
        }
    }
}

impl<'de, Iter: Iterator<Item=(String, String)>> de::Deserializer<'de>
for Deserializer<'de, Iter>
{
    type Error = Error;
    fn deserialize_any<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_map<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_map(self.inner)
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit seq
        bytes byte_buf unit_struct tuple_struct
        identifier tuple ignored_any option newtype_struct enum
        struct
    }
}

/// Deserializes a type based on information stored in env variables
pub fn from_env<T>() -> Result<T>
where
    T: de::DeserializeOwned,
{
    from_iter(env::vars())
}

/// Deserializes a type based on an iterable of `(String, String)`
/// representing keys and values
pub fn from_iter<Iter, T>(iter: Iter) -> Result<T>
where
    T: de::DeserializeOwned,
    Iter: IntoIterator<Item=(String, String)>,
{
    T::deserialize(Deserializer::new(iter.into_iter()))
}

/// A type which filters env vars with a prefix for use as serde field inputs
///
/// These types are created with with the [prefixed](fn.prefixed.html) module function
pub struct Prefixed<'a>(Cow<'a, str>);

impl<'a> Prefixed<'a> {
    /// Deserializes a type based on prefixed env variables
    pub fn from_env<T>(&self) -> Result<T>
    where
        T: de::DeserializeOwned,
    {
        self.from_iter(env::vars())
    }

    pub fn from_iter<Iter, T>(
        &self,
        iter: Iter,
    ) -> Result<T>
    where
        T: de::DeserializeOwned,
        Iter: IntoIterator<Item=(String, String)>,
    {
        from_iter(iter.into_iter().filter_map(|(k, v)| {
            if k.starts_with(self.0.as_ref()) {
                let key = k.trim_start_matches(self.0.as_ref()).to_owned();
                Some((key, v))
            } else {
                None
            }
        }))
            .map_err(|err| {
                // Wrap the original error and add the prefixed variable name if missing value
                match err {
                    Error::MissingValue { field, .. } => Error::MissingValue {
                        field,
                        env_var: Some(format!("{}{}", self.0, field.to_uppercase())),
                    },
                    other => other,
                }
            })
    }
}


/// Produces a instance of `Prefixed` for prefixing env variable names
///
/// # Example
///
/// ```no_run
/// use serde::Deserialize;
///
/// #[derive(Deserialize, Debug)]
/// struct Config {
///     foo: u16,
///     bar: bool,
///     baz: String,
///     boom: Option<u64>,
/// }
///
/// // all env variables will be expected to be prefixed with APP_
/// // i.e. APP_FOO, APP_BAR, ect
/// match envy::prefixed("APP_").from_env::<Config>() {
///     Ok(config) => println!("{:#?}", config),
///     Err(error) => eprintln!("{:#?}", error),
/// }
/// ```
pub fn prefixed<'a, C>(prefix: C) -> Prefixed<'a>
where
    C: Into<Cow<'a, str>>,
{
    Prefixed(prefix.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "lowercase")]
    pub enum Size {
        Small,
        Medium,
        Large,
    }

    impl Default for Size {
        fn default() -> Size {
            Size::Medium
        }
    }

    pub fn default_kaboom() -> u16 {
        8080
    }

    #[derive(Deserialize, Debug, PartialEq)]
    pub struct CustomNewType(u32);

    #[derive(Deserialize, Debug, PartialEq)]
    pub struct NestedType {
        field: String,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    pub struct Foo {
        bar: String,
        baz: bool,
        zoom: Option<u16>,
        doom: Vec<u64>,
        boom: Vec<String>,
        #[serde(default = "default_kaboom")]
        kaboom: u16,
        #[serde(default)]
        debug_mode: bool,
        #[serde(default)]
        size: Size,
        provided: Option<String>,
        newtype: CustomNewType,
        nested_type: NestedType,
    }

    #[test]
    fn deserialize_from_iter() {
        let data = vec![
            (String::from("BAR"), String::from("test")),
            (String::from("BAZ"), String::from("true")),
            (String::from("DOOM"), String::from("1,2,3")),
            // Empty string should result in an empty vector.
            (String::from("BOOM"), String::from("")),
            (String::from("SIZE"), String::from("small")),
            (String::from("PROVIDED"), String::from("test")),
            (String::from("NEWTYPE"), String::from("42")),
            (String::from("NESTED_TYPE__FIELD"), String::from("nested")),
        ];
        match from_iter::<_, Foo>(data) {
            Ok(actual) => assert_eq!(
                actual,
                Foo {
                    bar: String::from("test"),
                    baz: true,
                    zoom: None,
                    doom: vec![1, 2, 3],
                    boom: vec![],
                    kaboom: 8080,
                    debug_mode: false,
                    size: Size::Small,
                    provided: Some(String::from("test")),
                    newtype: CustomNewType(42),
                    nested_type: NestedType { field: "nested".to_string() },
                }
            ),
            Err(e) => panic!("{:#?}", e),
        }
    }
    fn collect_env_vars(prefix: &str) -> HashMap<String, String> {
        env::vars()
            .filter_map(|(k, v)| {
                if k.starts_with(prefix) {
                    Some((k.trim_start_matches(prefix).to_lowercase(), v))
                } else {
                    None
                }
            })
            .collect()
    }
    use serde_json::Value as JsonValue;

    /// Build a nested map from the flat environment variable map
    fn build_nested_map(flat_map: HashMap<String, String>) -> serde_json::Map<String, JsonValue> {
        let mut nested_map = serde_json::Map::new();

        for (key, value) in flat_map {
            let keys: Vec<&str> = key.split('.').collect();
            let mut current_map = &mut nested_map;

            for &part in &keys[0..keys.len() - 1] {
                current_map = current_map
                    .entry(part.to_string())
                    .or_insert_with(|| JsonValue::Object(serde_json::Map::new()))
                    .as_object_mut()
                    .unwrap();
            }

            current_map.insert(keys[keys.len() - 1].to_string(), JsonValue::String(value));
        }

        nested_map
    }

    #[derive(Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "lowercase")]
    struct Inner2 {
        deep: String,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "lowercase")]
    struct Inner {
        doom: Vec<u64>,
        inner2: Option<Inner2>,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "lowercase")]
    struct Outer {
        inner: Inner,
    }

    #[test]
    fn fails_with_missing_value() {
        std::env::set_var("OUTER_INNER_DOOM", "1,2,3");
        std::env::set_var("OUTER_INNER_INNER2_DEEP", "test");

        let outer = from_env::<Outer>();


        let collected = collect_env_vars("");
        let nested_map = build_nested_map(collected.clone());
        println!("collected: {:#?}", collected);
        println!("nested_map: {:#?}", nested_map);
        let data = vec![
            (String::from("BAR"), String::from("test")),
            (String::from("BAZ"), String::from("true")),
        ];
        match from_iter::<_, Foo>(data) {
            Ok(_) => panic!("expected failure"),
            Err(e) => assert_eq!(
                e,
                Error::MissingValue {
                    field: "doom",
                    env_var: None,
                }
            ),
        }
    }

    #[test]
    fn fails_with_invalid_type() {
        let data = vec![
            (String::from("BAR"), String::from("test")),
            (String::from("BAZ"), String::from("notabool")),
            (String::from("DOOM"), String::from("1,2,3")),
        ];
        match from_iter::<_, Foo>(data) {
            Ok(_) => panic!("expected failure"),
            Err(e) => assert_eq!(
                e,
                Error::Custom(String::from(
                    "provided string was not `true` or `false` while parsing value 'notabool' provided by BAZ"
                ))
            ),
        }
    }

    #[test]
    fn deserializes_from_prefixed_fieldnames() {
        let data = vec![
            (String::from("APP_BAR"), String::from("test")),
            (String::from("APP_BAZ"), String::from("true")),
            (String::from("APP_DOOM"), String::from("")),
            (String::from("APP_BOOM"), String::from("4,5")),
            (String::from("APP_SIZE"), String::from("small")),
            (String::from("APP_PROVIDED"), String::from("test")),
            (String::from("APP_NEWTYPE"), String::from("42")),
        ];
        match prefixed("APP_").from_iter::<_, Foo>(data) {
            Ok(actual) => assert_eq!(
                actual,
                Foo {
                    bar: String::from("test"),
                    baz: true,
                    zoom: None,
                    doom: vec![],
                    boom: vec!["4".to_string(), "5".to_string()],
                    kaboom: 8080,
                    debug_mode: false,
                    size: Size::Small,
                    provided: Some(String::from("test")),
                    newtype: CustomNewType(42),
                    nested_type: NestedType { field: "nested".to_string() },
                }
            ),
            Err(e) => panic!("{:#?}", e),
        }
    }

    #[test]
    fn prefixed_strips_prefixes() {
        let mut expected = HashMap::new();
        expected.insert("foo".to_string(), "bar".to_string());
        assert_eq!(
            prefixed("PRE_").from_iter(vec![("PRE_FOO".to_string(), "bar".to_string())]),
            Ok(expected)
        );
    }

    #[test]
    fn prefixed_doesnt_parse_non_prefixed() {
        let mut expected = HashMap::new();
        expected.insert("foo".to_string(), 12);
        assert_eq!(
            prefixed("PRE_").from_iter(vec![
                ("FOO".to_string(), "asd".to_string()),
                ("PRE_FOO".to_string(), "12".to_string())
            ]),
            Ok(expected)
        );
    }

    fn impl_std_error<E: StdError>(_: E) {}

    #[test]
    fn error_impl_std_error() {
        impl_std_error(Error::MissingValue {
            field: "foo_bar",
            env_var: Some("FOO_BAR".into()),
        });
        impl_std_error(Error::Custom("whoops".into()))
    }

    #[test]
    fn error_display() {
        assert_eq!(
            format!(
                "{}",
                Error::MissingValue {
                    field: "foo_bar",
                    env_var: Some("FOO_BAR".into())
                }
            ),
            "Configuration could not be loaded: missing value for field `foo_bar`.\n\nExpected environment variable: `FOO_BAR`\n\nHint: Ensure that `FOO_BAR` is set in your environment, e.g.,\nexport FOO_BAR=\"<your-value>\""
        );

        assert_eq!(format!("{}", Error::Custom("whoops".into())), "whoops")
    }
}
