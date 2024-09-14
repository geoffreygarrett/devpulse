// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizationModel {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub schema_version: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub type_definitions: ::prost::alloc::vec::Vec<TypeDefinition>,
    #[prost(map="string, message", tag="4")]
    pub conditions: ::std::collections::HashMap<::prost::alloc::string::String, Condition>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeDefinition {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(map="string, message", tag="2")]
    pub relations: ::std::collections::HashMap<::prost::alloc::string::String, Userset>,
    /// A map whose keys are the name of the relation and whose value is the Metadata for that relation.
    /// It also holds information around the module name and source file if this model was constructed
    /// from a modular model.
    #[prost(message, optional, tag="3")]
    pub metadata: ::core::option::Option<Metadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Relation {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub rewrite: ::core::option::Option<Userset>,
    #[prost(message, optional, tag="3")]
    pub type_info: ::core::option::Option<RelationTypeInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelationTypeInfo {
    #[prost(message, repeated, tag="1")]
    pub directly_related_user_types: ::prost::alloc::vec::Vec<RelationReference>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(map="string, message", tag="1")]
    pub relations: ::std::collections::HashMap<::prost::alloc::string::String, RelationMetadata>,
    #[prost(string, tag="2")]
    pub module: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub source_info: ::core::option::Option<SourceInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceInfo {
    #[prost(string, tag="1")]
    pub file: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelationMetadata {
    #[prost(message, repeated, tag="1")]
    pub directly_related_user_types: ::prost::alloc::vec::Vec<RelationReference>,
    #[prost(string, tag="2")]
    pub module: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub source_info: ::core::option::Option<SourceInfo>,
}
/// RelationReference represents a relation of a particular object type (e.g. 'document#viewer').
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelationReference {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    /// The name of a condition that is enforced over the allowed relation.
    #[prost(string, tag="4")]
    pub condition: ::prost::alloc::string::String,
    #[prost(oneof="relation_reference::RelationOrWildcard", tags="2, 3")]
    pub relation_or_wildcard: ::core::option::Option<relation_reference::RelationOrWildcard>,
}
/// Nested message and enum types in `RelationReference`.
pub mod relation_reference {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RelationOrWildcard {
        #[prost(string, tag="2")]
        Relation(::prost::alloc::string::String),
        #[prost(message, tag="3")]
        Wildcard(super::Wildcard),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Wildcard {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Usersets {
    #[prost(message, repeated, tag="1")]
    pub child: ::prost::alloc::vec::Vec<Userset>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Difference {
    #[prost(message, optional, boxed, tag="1")]
    pub base: ::core::option::Option<::prost::alloc::boxed::Box<Userset>>,
    #[prost(message, optional, boxed, tag="2")]
    pub subtract: ::core::option::Option<::prost::alloc::boxed::Box<Userset>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Userset {
    #[prost(oneof="userset::Userset", tags="1, 2, 3, 4, 5, 6")]
    pub userset: ::core::option::Option<userset::Userset>,
}
/// Nested message and enum types in `Userset`.
pub mod userset {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Userset {
        #[prost(message, tag="1")]
        This(super::DirectUserset),
        #[prost(message, tag="2")]
        ComputedUserset(super::ObjectRelation),
        #[prost(message, tag="3")]
        TupleToUserset(super::TupleToUserset),
        #[prost(message, tag="4")]
        Union(super::Usersets),
        #[prost(message, tag="5")]
        Intersection(super::Usersets),
        #[prost(message, tag="6")]
        Difference(::prost::alloc::boxed::Box<super::Difference>),
    }
}
/// A DirectUserset is a sentinel message for referencing
/// the direct members specified by an object/relation mapping.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectUserset {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectRelation {
    #[prost(string, tag="1")]
    pub object: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub relation: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputedUserset {
    #[prost(string, tag="1")]
    pub relation: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TupleToUserset {
    /// The target object/relation
    #[prost(message, optional, tag="1")]
    pub tupleset: ::core::option::Option<ObjectRelation>,
    #[prost(message, optional, tag="2")]
    pub computed_userset: ::core::option::Option<ObjectRelation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Condition {
    /// A unique name for the condition
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// A Google CEL expression, expressed as a string.
    #[prost(string, tag="2")]
    pub expression: ::prost::alloc::string::String,
    /// A map of parameter names to the parameter's defined type reference.
    #[prost(map="string, message", tag="3")]
    pub parameters: ::std::collections::HashMap<::prost::alloc::string::String, ConditionParamTypeRef>,
    #[prost(message, optional, tag="4")]
    pub metadata: ::core::option::Option<ConditionMetadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConditionMetadata {
    #[prost(string, tag="1")]
    pub module: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub source_info: ::core::option::Option<SourceInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConditionParamTypeRef {
    #[prost(enumeration="condition_param_type_ref::TypeName", tag="1")]
    pub type_name: i32,
    #[prost(message, repeated, tag="2")]
    pub generic_types: ::prost::alloc::vec::Vec<ConditionParamTypeRef>,
}
/// Nested message and enum types in `ConditionParamTypeRef`.
pub mod condition_param_type_ref {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TypeName {
        Unspecified = 0,
        Any = 1,
        Bool = 2,
        String = 3,
        Int = 4,
        Uint = 5,
        Double = 6,
        Duration = 7,
        Timestamp = 8,
        Map = 9,
        List = 10,
        Ipaddress = 11,
    }
    impl TypeName {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TypeName::Unspecified => "TYPE_NAME_UNSPECIFIED",
                TypeName::Any => "TYPE_NAME_ANY",
                TypeName::Bool => "TYPE_NAME_BOOL",
                TypeName::String => "TYPE_NAME_STRING",
                TypeName::Int => "TYPE_NAME_INT",
                TypeName::Uint => "TYPE_NAME_UINT",
                TypeName::Double => "TYPE_NAME_DOUBLE",
                TypeName::Duration => "TYPE_NAME_DURATION",
                TypeName::Timestamp => "TYPE_NAME_TIMESTAMP",
                TypeName::Map => "TYPE_NAME_MAP",
                TypeName::List => "TYPE_NAME_LIST",
                TypeName::Ipaddress => "TYPE_NAME_IPADDRESS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_NAME_UNSPECIFIED" => Some(Self::Unspecified),
                "TYPE_NAME_ANY" => Some(Self::Any),
                "TYPE_NAME_BOOL" => Some(Self::Bool),
                "TYPE_NAME_STRING" => Some(Self::String),
                "TYPE_NAME_INT" => Some(Self::Int),
                "TYPE_NAME_UINT" => Some(Self::Uint),
                "TYPE_NAME_DOUBLE" => Some(Self::Double),
                "TYPE_NAME_DURATION" => Some(Self::Duration),
                "TYPE_NAME_TIMESTAMP" => Some(Self::Timestamp),
                "TYPE_NAME_MAP" => Some(Self::Map),
                "TYPE_NAME_LIST" => Some(Self::List),
                "TYPE_NAME_IPADDRESS" => Some(Self::Ipaddress),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidationErrorMessageResponse {
    #[prost(enumeration="ErrorCode", tag="1")]
    pub code: i32,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnauthenticatedResponse {
    #[prost(enumeration="ErrorCode", tag="1")]
    pub code: i32,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnprocessableContentMessageResponse {
    #[prost(enumeration="UnprocessableContentErrorCode", tag="1")]
    pub code: i32,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalErrorMessageResponse {
    #[prost(enumeration="InternalErrorCode", tag="1")]
    pub code: i32,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PathUnknownErrorMessageResponse {
    #[prost(enumeration="NotFoundErrorCode", tag="1")]
    pub code: i32,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbortedMessageResponse {
    #[prost(string, tag="1")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorMessageRequest {
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthErrorCode {
    NoAuthError = 0,
    AuthFailedInvalidSubject = 1001,
    AuthFailedInvalidAudience = 1002,
    AuthFailedInvalidIssuer = 1003,
    InvalidClaims = 1004,
    AuthFailedInvalidBearerToken = 1005,
    BearerTokenMissing = 1010,
    Unauthenticated = 1500,
}
impl AuthErrorCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AuthErrorCode::NoAuthError => "no_auth_error",
            AuthErrorCode::AuthFailedInvalidSubject => "auth_failed_invalid_subject",
            AuthErrorCode::AuthFailedInvalidAudience => "auth_failed_invalid_audience",
            AuthErrorCode::AuthFailedInvalidIssuer => "auth_failed_invalid_issuer",
            AuthErrorCode::InvalidClaims => "invalid_claims",
            AuthErrorCode::AuthFailedInvalidBearerToken => "auth_failed_invalid_bearer_token",
            AuthErrorCode::BearerTokenMissing => "bearer_token_missing",
            AuthErrorCode::Unauthenticated => "unauthenticated",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "no_auth_error" => Some(Self::NoAuthError),
            "auth_failed_invalid_subject" => Some(Self::AuthFailedInvalidSubject),
            "auth_failed_invalid_audience" => Some(Self::AuthFailedInvalidAudience),
            "auth_failed_invalid_issuer" => Some(Self::AuthFailedInvalidIssuer),
            "invalid_claims" => Some(Self::InvalidClaims),
            "auth_failed_invalid_bearer_token" => Some(Self::AuthFailedInvalidBearerToken),
            "bearer_token_missing" => Some(Self::BearerTokenMissing),
            "unauthenticated" => Some(Self::Unauthenticated),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ErrorCode {
    NoError = 0,
    // 2000 level errors are returned due to input error

    ValidationError = 2000,
    AuthorizationModelNotFound = 2001,
    AuthorizationModelResolutionTooComplex = 2002,
    InvalidWriteInput = 2003,
    CannotAllowDuplicateTuplesInOneRequest = 2004,
    CannotAllowDuplicateTypesInOneRequest = 2005,
    CannotAllowMultipleReferencesToOneRelation = 2006,
    InvalidContinuationToken = 2007,
    InvalidTupleSet = 2008,
    InvalidCheckInput = 2009,
    InvalidExpandInput = 2010,
    UnsupportedUserSet = 2011,
    InvalidObjectFormat = 2012,
    WriteFailedDueToInvalidInput = 2017,
    AuthorizationModelAssertionsNotFound = 2018,
    LatestAuthorizationModelNotFound = 2020,
    TypeNotFound = 2021,
    RelationNotFound = 2022,
    EmptyRelationDefinition = 2023,
    InvalidUser = 2025,
    InvalidTuple = 2027,
    UnknownRelation = 2028,
    StoreIdInvalidLength = 2030,
    AssertionsTooManyItems = 2033,
    IdTooLong = 2034,
    AuthorizationModelIdTooLong = 2036,
    TupleKeyValueNotSpecified = 2037,
    TupleKeysTooManyOrTooFewItems = 2038,
    PageSizeInvalid = 2039,
    ParamMissingValue = 2040,
    DifferenceBaseMissingValue = 2041,
    SubtractBaseMissingValue = 2042,
    ObjectTooLong = 2043,
    RelationTooLong = 2044,
    TypeDefinitionsTooFewItems = 2045,
    TypeInvalidLength = 2046,
    TypeInvalidPattern = 2047,
    RelationsTooFewItems = 2048,
    RelationsTooLong = 2049,
    RelationsInvalidPattern = 2050,
    ObjectInvalidPattern = 2051,
    QueryStringTypeContinuationTokenMismatch = 2052,
    ExceededEntityLimit = 2053,
    InvalidContextualTuple = 2054,
    DuplicateContextualTuple = 2055,
    InvalidAuthorizationModel = 2056,
    UnsupportedSchemaVersion = 2057,
}
impl ErrorCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ErrorCode::NoError => "no_error",
            ErrorCode::ValidationError => "validation_error",
            ErrorCode::AuthorizationModelNotFound => "authorization_model_not_found",
            ErrorCode::AuthorizationModelResolutionTooComplex => "authorization_model_resolution_too_complex",
            ErrorCode::InvalidWriteInput => "invalid_write_input",
            ErrorCode::CannotAllowDuplicateTuplesInOneRequest => "cannot_allow_duplicate_tuples_in_one_request",
            ErrorCode::CannotAllowDuplicateTypesInOneRequest => "cannot_allow_duplicate_types_in_one_request",
            ErrorCode::CannotAllowMultipleReferencesToOneRelation => "cannot_allow_multiple_references_to_one_relation",
            ErrorCode::InvalidContinuationToken => "invalid_continuation_token",
            ErrorCode::InvalidTupleSet => "invalid_tuple_set",
            ErrorCode::InvalidCheckInput => "invalid_check_input",
            ErrorCode::InvalidExpandInput => "invalid_expand_input",
            ErrorCode::UnsupportedUserSet => "unsupported_user_set",
            ErrorCode::InvalidObjectFormat => "invalid_object_format",
            ErrorCode::WriteFailedDueToInvalidInput => "write_failed_due_to_invalid_input",
            ErrorCode::AuthorizationModelAssertionsNotFound => "authorization_model_assertions_not_found",
            ErrorCode::LatestAuthorizationModelNotFound => "latest_authorization_model_not_found",
            ErrorCode::TypeNotFound => "type_not_found",
            ErrorCode::RelationNotFound => "relation_not_found",
            ErrorCode::EmptyRelationDefinition => "empty_relation_definition",
            ErrorCode::InvalidUser => "invalid_user",
            ErrorCode::InvalidTuple => "invalid_tuple",
            ErrorCode::UnknownRelation => "unknown_relation",
            ErrorCode::StoreIdInvalidLength => "store_id_invalid_length",
            ErrorCode::AssertionsTooManyItems => "assertions_too_many_items",
            ErrorCode::IdTooLong => "id_too_long",
            ErrorCode::AuthorizationModelIdTooLong => "authorization_model_id_too_long",
            ErrorCode::TupleKeyValueNotSpecified => "tuple_key_value_not_specified",
            ErrorCode::TupleKeysTooManyOrTooFewItems => "tuple_keys_too_many_or_too_few_items",
            ErrorCode::PageSizeInvalid => "page_size_invalid",
            ErrorCode::ParamMissingValue => "param_missing_value",
            ErrorCode::DifferenceBaseMissingValue => "difference_base_missing_value",
            ErrorCode::SubtractBaseMissingValue => "subtract_base_missing_value",
            ErrorCode::ObjectTooLong => "object_too_long",
            ErrorCode::RelationTooLong => "relation_too_long",
            ErrorCode::TypeDefinitionsTooFewItems => "type_definitions_too_few_items",
            ErrorCode::TypeInvalidLength => "type_invalid_length",
            ErrorCode::TypeInvalidPattern => "type_invalid_pattern",
            ErrorCode::RelationsTooFewItems => "relations_too_few_items",
            ErrorCode::RelationsTooLong => "relations_too_long",
            ErrorCode::RelationsInvalidPattern => "relations_invalid_pattern",
            ErrorCode::ObjectInvalidPattern => "object_invalid_pattern",
            ErrorCode::QueryStringTypeContinuationTokenMismatch => "query_string_type_continuation_token_mismatch",
            ErrorCode::ExceededEntityLimit => "exceeded_entity_limit",
            ErrorCode::InvalidContextualTuple => "invalid_contextual_tuple",
            ErrorCode::DuplicateContextualTuple => "duplicate_contextual_tuple",
            ErrorCode::InvalidAuthorizationModel => "invalid_authorization_model",
            ErrorCode::UnsupportedSchemaVersion => "unsupported_schema_version",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "no_error" => Some(Self::NoError),
            "validation_error" => Some(Self::ValidationError),
            "authorization_model_not_found" => Some(Self::AuthorizationModelNotFound),
            "authorization_model_resolution_too_complex" => Some(Self::AuthorizationModelResolutionTooComplex),
            "invalid_write_input" => Some(Self::InvalidWriteInput),
            "cannot_allow_duplicate_tuples_in_one_request" => Some(Self::CannotAllowDuplicateTuplesInOneRequest),
            "cannot_allow_duplicate_types_in_one_request" => Some(Self::CannotAllowDuplicateTypesInOneRequest),
            "cannot_allow_multiple_references_to_one_relation" => Some(Self::CannotAllowMultipleReferencesToOneRelation),
            "invalid_continuation_token" => Some(Self::InvalidContinuationToken),
            "invalid_tuple_set" => Some(Self::InvalidTupleSet),
            "invalid_check_input" => Some(Self::InvalidCheckInput),
            "invalid_expand_input" => Some(Self::InvalidExpandInput),
            "unsupported_user_set" => Some(Self::UnsupportedUserSet),
            "invalid_object_format" => Some(Self::InvalidObjectFormat),
            "write_failed_due_to_invalid_input" => Some(Self::WriteFailedDueToInvalidInput),
            "authorization_model_assertions_not_found" => Some(Self::AuthorizationModelAssertionsNotFound),
            "latest_authorization_model_not_found" => Some(Self::LatestAuthorizationModelNotFound),
            "type_not_found" => Some(Self::TypeNotFound),
            "relation_not_found" => Some(Self::RelationNotFound),
            "empty_relation_definition" => Some(Self::EmptyRelationDefinition),
            "invalid_user" => Some(Self::InvalidUser),
            "invalid_tuple" => Some(Self::InvalidTuple),
            "unknown_relation" => Some(Self::UnknownRelation),
            "store_id_invalid_length" => Some(Self::StoreIdInvalidLength),
            "assertions_too_many_items" => Some(Self::AssertionsTooManyItems),
            "id_too_long" => Some(Self::IdTooLong),
            "authorization_model_id_too_long" => Some(Self::AuthorizationModelIdTooLong),
            "tuple_key_value_not_specified" => Some(Self::TupleKeyValueNotSpecified),
            "tuple_keys_too_many_or_too_few_items" => Some(Self::TupleKeysTooManyOrTooFewItems),
            "page_size_invalid" => Some(Self::PageSizeInvalid),
            "param_missing_value" => Some(Self::ParamMissingValue),
            "difference_base_missing_value" => Some(Self::DifferenceBaseMissingValue),
            "subtract_base_missing_value" => Some(Self::SubtractBaseMissingValue),
            "object_too_long" => Some(Self::ObjectTooLong),
            "relation_too_long" => Some(Self::RelationTooLong),
            "type_definitions_too_few_items" => Some(Self::TypeDefinitionsTooFewItems),
            "type_invalid_length" => Some(Self::TypeInvalidLength),
            "type_invalid_pattern" => Some(Self::TypeInvalidPattern),
            "relations_too_few_items" => Some(Self::RelationsTooFewItems),
            "relations_too_long" => Some(Self::RelationsTooLong),
            "relations_invalid_pattern" => Some(Self::RelationsInvalidPattern),
            "object_invalid_pattern" => Some(Self::ObjectInvalidPattern),
            "query_string_type_continuation_token_mismatch" => Some(Self::QueryStringTypeContinuationTokenMismatch),
            "exceeded_entity_limit" => Some(Self::ExceededEntityLimit),
            "invalid_contextual_tuple" => Some(Self::InvalidContextualTuple),
            "duplicate_contextual_tuple" => Some(Self::DuplicateContextualTuple),
            "invalid_authorization_model" => Some(Self::InvalidAuthorizationModel),
            "unsupported_schema_version" => Some(Self::UnsupportedSchemaVersion),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UnprocessableContentErrorCode {
    NoThrottledErrorCode = 0,
    // 3500 level errors are timeout due to throttling

    ThrottledTimeoutError = 3500,
}
impl UnprocessableContentErrorCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UnprocessableContentErrorCode::NoThrottledErrorCode => "no_throttled_error_code",
            UnprocessableContentErrorCode::ThrottledTimeoutError => "throttled_timeout_error",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "no_throttled_error_code" => Some(Self::NoThrottledErrorCode),
            "throttled_timeout_error" => Some(Self::ThrottledTimeoutError),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InternalErrorCode {
    NoInternalError = 0,
    // 4000 level errors are returned due to internal error

    InternalError = 4000,
    Cancelled = 4003,
    DeadlineExceeded = 4004,
    AlreadyExists = 4005,
    ResourceExhausted = 4006,
    FailedPrecondition = 4007,
    Aborted = 4008,
    OutOfRange = 4009,
    Unavailable = 4010,
    DataLoss = 4011,
}
impl InternalErrorCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InternalErrorCode::NoInternalError => "no_internal_error",
            InternalErrorCode::InternalError => "internal_error",
            InternalErrorCode::Cancelled => "cancelled",
            InternalErrorCode::DeadlineExceeded => "deadline_exceeded",
            InternalErrorCode::AlreadyExists => "already_exists",
            InternalErrorCode::ResourceExhausted => "resource_exhausted",
            InternalErrorCode::FailedPrecondition => "failed_precondition",
            InternalErrorCode::Aborted => "aborted",
            InternalErrorCode::OutOfRange => "out_of_range",
            InternalErrorCode::Unavailable => "unavailable",
            InternalErrorCode::DataLoss => "data_loss",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "no_internal_error" => Some(Self::NoInternalError),
            "internal_error" => Some(Self::InternalError),
            "cancelled" => Some(Self::Cancelled),
            "deadline_exceeded" => Some(Self::DeadlineExceeded),
            "already_exists" => Some(Self::AlreadyExists),
            "resource_exhausted" => Some(Self::ResourceExhausted),
            "failed_precondition" => Some(Self::FailedPrecondition),
            "aborted" => Some(Self::Aborted),
            "out_of_range" => Some(Self::OutOfRange),
            "unavailable" => Some(Self::Unavailable),
            "data_loss" => Some(Self::DataLoss),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NotFoundErrorCode {
    NoNotFoundError = 0,
    UndefinedEndpoint = 5000,
    StoreIdNotFound = 5002,
    Unimplemented = 5004,
}
impl NotFoundErrorCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NotFoundErrorCode::NoNotFoundError => "no_not_found_error",
            NotFoundErrorCode::UndefinedEndpoint => "undefined_endpoint",
            NotFoundErrorCode::StoreIdNotFound => "store_id_not_found",
            NotFoundErrorCode::Unimplemented => "unimplemented",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "no_not_found_error" => Some(Self::NoNotFoundError),
            "undefined_endpoint" => Some(Self::UndefinedEndpoint),
            "store_id_not_found" => Some(Self::StoreIdNotFound),
            "unimplemented" => Some(Self::Unimplemented),
            _ => None,
        }
    }
}
/// Object represents an OpenFGA Object.
///
/// An Object is composed of a type and identifier (e.g. 'document:1')
///
/// See <https://openfga.dev/docs/concepts#what-is-an-object>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Object {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
/// User.
///
/// Represents any possible value for a user (subject or principal). Can be a:
/// - Specific user object e.g.: 'user:will', 'folder:marketing', 'org:contoso', ...)
/// - Specific userset (e.g. 'group:engineering#member')
/// - Public-typed wildcard (e.g. 'user:*')
///
/// See <https://openfga.dev/docs/concepts#what-is-a-user>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(oneof="user::User", tags="1, 2, 3")]
    pub user: ::core::option::Option<user::User>,
}
/// Nested message and enum types in `User`.
pub mod user {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum User {
        #[prost(message, tag="1")]
        Object(super::Object),
        #[prost(message, tag="2")]
        Userset(super::UsersetUser),
        #[prost(message, tag="3")]
        Wildcard(super::TypedWildcard),
    }
}
/// Userset.
///
/// A set or group of users, represented in the `<type>:<id>#<relation>` format
///
/// `group:fga#member` represents all members of group FGA, not to be confused by `group:fga` which represents the group itself as a specific object.
///
/// See: <https://openfga.dev/docs/modeling/building-blocks/usersets#what-is-a-userset>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsersetUser {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub relation: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelationshipCondition {
    /// A reference (by name) of the relationship condition defined in the authorization model.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional context/data to persist along with the condition.
    /// The keys must match the parameters defined by the condition, and the value types must
    /// match the parameter type definitions.
    #[prost(message, optional, tag="2")]
    pub context: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TupleKeyWithoutCondition {
    #[prost(string, tag="1")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub relation: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub object: ::prost::alloc::string::String,
}
/// Type bound public access.
///
/// Normally represented using the `<type>:*` syntax
///
/// `employee:*` represents every object of type `employee`, including those not currently present in the system
///
/// See <https://openfga.dev/docs/concepts#what-is-type-bound-public-access>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedWildcard {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TupleKey {
    #[prost(string, tag="1")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub relation: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub object: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub condition: ::core::option::Option<RelationshipCondition>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tuple {
    #[prost(message, optional, tag="1")]
    pub key: ::core::option::Option<TupleKey>,
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TupleKeys {
    #[prost(message, repeated, tag="1")]
    pub tuple_keys: ::prost::alloc::vec::Vec<TupleKey>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextualTupleKeys {
    #[prost(message, repeated, tag="1")]
    pub tuple_keys: ::prost::alloc::vec::Vec<TupleKey>,
}
/// A UsersetTree contains the result of an Expansion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsersetTree {
    #[prost(message, optional, tag="1")]
    pub root: ::core::option::Option<userset_tree::Node>,
}
/// Nested message and enum types in `UsersetTree`.
pub mod userset_tree {
    /// A leaf node contains either
    /// - a set of users (which may be individual users, or usersets
    ///    referencing other relations)
    /// - a computed node, which is the result of a computed userset
    ///    value in the authorization model
    /// - a tupleToUserset nodes, containing the result of expanding
    ///    a tupleToUserset value in a authorization model.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Leaf {
        #[prost(oneof="leaf::Value", tags="1, 2, 3")]
        pub value: ::core::option::Option<leaf::Value>,
    }
    /// Nested message and enum types in `Leaf`.
    pub mod leaf {
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Value {
            #[prost(message, tag="1")]
            Users(super::Users),
            #[prost(message, tag="2")]
            Computed(super::Computed),
            #[prost(message, tag="3")]
            TupleToUserset(super::TupleToUserset),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Nodes {
        #[prost(message, repeated, tag="1")]
        pub nodes: ::prost::alloc::vec::Vec<Node>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Users {
        #[prost(string, repeated, tag="1")]
        pub users: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Computed {
        #[prost(string, tag="1")]
        pub userset: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TupleToUserset {
        #[prost(string, tag="1")]
        pub tupleset: ::prost::alloc::string::String,
        #[prost(message, repeated, tag="2")]
        pub computed: ::prost::alloc::vec::Vec<Computed>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Difference {
        #[prost(message, optional, boxed, tag="1")]
        pub base: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
        #[prost(message, optional, boxed, tag="2")]
        pub subtract: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Node {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(oneof="node::Value", tags="2, 5, 6, 7")]
        pub value: ::core::option::Option<node::Value>,
    }
    /// Nested message and enum types in `Node`.
    pub mod node {
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Value {
            #[prost(message, tag="2")]
            Leaf(super::Leaf),
            #[prost(message, tag="5")]
            Difference(::prost::alloc::boxed::Box<super::Difference>),
            #[prost(message, tag="6")]
            Union(super::Nodes),
            #[prost(message, tag="7")]
            Intersection(super::Nodes),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TupleChange {
    #[prost(message, optional, tag="1")]
    pub tuple_key: ::core::option::Option<TupleKey>,
    #[prost(enumeration="TupleOperation", tag="2")]
    pub operation: i32,
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Store {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="4")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="5")]
    pub deleted_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserTypeFilter {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub relation: ::prost::alloc::string::String,
}
/// buf:lint:ignore ENUM_ZERO_VALUE_SUFFIX
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TupleOperation {
    Write = 0,
    Delete = 1,
}
impl TupleOperation {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TupleOperation::Write => "TUPLE_OPERATION_WRITE",
            TupleOperation::Delete => "TUPLE_OPERATION_DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TUPLE_OPERATION_WRITE" => Some(Self::Write),
            "TUPLE_OPERATION_DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListObjectsRequest {
    #[prost(string, tag="1")]
    pub store_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub authorization_model_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub relation: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub user: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub contextual_tuples: ::core::option::Option<ContextualTupleKeys>,
    /// Additional request context that will be used to evaluate any ABAC conditions encountered
    /// in the query evaluation.
    #[prost(message, optional, tag="7")]
    pub context: ::core::option::Option<::prost_types::Struct>,
    /// Controls the consistency preference for this request. Default value is UNSPECIFIED, which will have the same behavior as MINIMIZE_LATENCY.
    #[prost(enumeration="ConsistencyPreference", tag="8")]
    pub consistency: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListObjectsResponse {
    #[prost(string, repeated, tag="1")]
    pub objects: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsersRequest {
    #[prost(string, tag="1")]
    pub store_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub authorization_model_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub object: ::core::option::Option<Object>,
    #[prost(string, tag="4")]
    pub relation: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="5")]
    pub user_filters: ::prost::alloc::vec::Vec<UserTypeFilter>,
    #[prost(message, repeated, tag="6")]
    pub contextual_tuples: ::prost::alloc::vec::Vec<TupleKey>,
    /// Additional request context that will be used to evaluate any ABAC conditions encountered
    /// in the query evaluation.
    #[prost(message, optional, tag="7")]
    pub context: ::core::option::Option<::prost_types::Struct>,
    /// Controls the consistency preference for this request. Default value is UNSPECIFIED, which will have the same behavior as MINIMIZE_LATENCY.
    #[prost(enumeration="ConsistencyPreference", tag="8")]
    pub consistency: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsersResponse {
    #[prost(message, repeated, tag="1")]
    pub users: ::prost::alloc::vec::Vec<User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamedListObjectsRequest {
    #[prost(string, tag="1")]
    pub store_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub authorization_model_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub relation: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub user: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub contextual_tuples: ::core::option::Option<ContextualTupleKeys>,
    /// Additional request context that will be used to evaluate any ABAC conditions encountered
    /// in the query evaluation.
    #[prost(message, optional, tag="7")]
    pub context: ::core::option::Option<::prost_types::Struct>,
    /// Controls the consistency preference for this request. Default value is UNSPECIFIED, which will have the same behavior as MINIMIZE_LATENCY.
    #[prost(enumeration="ConsistencyPreference", tag="8")]
    pub consistency: i32,
}
/// The response for a StreamedListObjects RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamedListObjectsResponse {
    #[prost(string, tag="1")]
    pub object: ::prost::alloc::string::String,
}
// Note: store_id is a ULID using pattern ^\[ABCDEFGHJKMNPQRSTVWXYZ0-9\]{26}$
// which excludes I, L, O, and U
// because of <https://github.com/ulid/spec#encoding>

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRequest {
    #[prost(string, tag="1")]
    pub store_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub tuple_key: ::core::option::Option<ReadRequestTupleKey>,
    #[prost(message, optional, tag="3")]
    pub page_size: ::core::option::Option<i32>,
    #[prost(string, tag="4")]
    pub continuation_token: ::prost::alloc::string::String,
    /// Controls the consistency preference for this request. Default value is UNSPECIFIED, which will have the same behavior as MINIMIZE_LATENCY.
    #[prost(enumeration="ConsistencyPreference", tag="5")]
    pub consistency: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRequestTupleKey {
    #[prost(string, tag="1")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub relation: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub object: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadResponse {
    #[prost(message, repeated, tag="1")]
    pub tuples: ::prost::alloc::vec::Vec<Tuple>,
    #[prost(string, tag="2")]
    pub continuation_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteRequestWrites {
    #[prost(message, repeated, tag="1")]
    pub tuple_keys: ::prost::alloc::vec::Vec<TupleKey>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteRequestDeletes {
    #[prost(message, repeated, tag="1")]
    pub tuple_keys: ::prost::alloc::vec::Vec<TupleKeyWithoutCondition>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteRequest {
    #[prost(string, tag="1")]
    pub store_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub writes: ::core::option::Option<WriteRequestWrites>,
    #[prost(message, optional, tag="3")]
    pub deletes: ::core::option::Option<WriteRequestDeletes>,
    #[prost(string, tag="4")]
    pub authorization_model_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckRequest {
    #[prost(string, tag="1")]
    pub store_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub tuple_key: ::core::option::Option<CheckRequestTupleKey>,
    #[prost(message, optional, tag="3")]
    pub contextual_tuples: ::core::option::Option<ContextualTupleKeys>,
    #[prost(string, tag="4")]
    pub authorization_model_id: ::prost::alloc::string::String,
    /// Defaults to false. Making it true has performance implications.
    #[prost(bool, tag="5")]
    pub trace: bool,
    /// Additional request context that will be used to evaluate any ABAC conditions encountered
    /// in the query evaluation.
    #[prost(message, optional, tag="6")]
    pub context: ::core::option::Option<::prost_types::Struct>,
    /// Controls the consistency preference for this request. Default value is UNSPECIFIED, which will have the same behavior as MINIMIZE_LATENCY.
    #[prost(enumeration="ConsistencyPreference", tag="7")]
    pub consistency: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckRequestTupleKey {
    #[prost(string, tag="1")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub relation: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub object: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckResponse {
    #[prost(bool, tag="1")]
    pub allowed: bool,
    /// For internal use only.
    #[prost(string, tag="2")]
    pub resolution: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandRequest {
    #[prost(string, tag="1")]
    pub store_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub tuple_key: ::core::option::Option<ExpandRequestTupleKey>,
    #[prost(string, tag="3")]
    pub authorization_model_id: ::prost::alloc::string::String,
    /// Controls the consistency preference for this request. Default value is UNSPECIFIED, which will have the same behavior as MINIMIZE_LATENCY.
    #[prost(enumeration="ConsistencyPreference", tag="4")]
    pub consistency: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandRequestTupleKey {
    #[prost(string, tag="1")]
    pub relation: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub object: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandResponse {
    #[prost(message, optional, tag="1")]
    pub tree: ::core::option::Option<UsersetTree>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAuthorizationModelRequest {
    #[prost(string, tag="1")]
    pub store_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAuthorizationModelResponse {
    #[prost(message, optional, tag="1")]
    pub authorization_model: ::core::option::Option<AuthorizationModel>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteAuthorizationModelRequest {
    #[prost(string, tag="1")]
    pub store_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub type_definitions: ::prost::alloc::vec::Vec<TypeDefinition>,
    #[prost(string, tag="3")]
    pub schema_version: ::prost::alloc::string::String,
    #[prost(map="string, message", tag="4")]
    pub conditions: ::std::collections::HashMap<::prost::alloc::string::String, Condition>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteAuthorizationModelResponse {
    #[prost(string, tag="1")]
    pub authorization_model_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAuthorizationModelsRequest {
    #[prost(string, tag="1")]
    pub store_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub page_size: ::core::option::Option<i32>,
    #[prost(string, tag="3")]
    pub continuation_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAuthorizationModelsResponse {
    #[prost(message, repeated, tag="1")]
    pub authorization_models: ::prost::alloc::vec::Vec<AuthorizationModel>,
    #[prost(string, tag="2")]
    pub continuation_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteAssertionsRequest {
    #[prost(string, tag="1")]
    pub store_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub authorization_model_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub assertions: ::prost::alloc::vec::Vec<Assertion>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteAssertionsResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAssertionsRequest {
    #[prost(string, tag="1")]
    pub store_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub authorization_model_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAssertionsResponse {
    #[prost(string, tag="1")]
    pub authorization_model_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub assertions: ::prost::alloc::vec::Vec<Assertion>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadChangesRequest {
    #[prost(string, tag="1")]
    pub store_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub page_size: ::core::option::Option<i32>,
    #[prost(string, tag="4")]
    pub continuation_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadChangesResponse {
    #[prost(message, repeated, tag="1")]
    pub changes: ::prost::alloc::vec::Vec<TupleChange>,
    #[prost(string, tag="2")]
    pub continuation_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStoreRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStoreResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="4")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStoreRequest {
    #[prost(string, tag="1")]
    pub store_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStoreResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="4")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteStoreRequest {
    #[prost(string, tag="1")]
    pub store_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteStoreResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoreRequest {
    #[prost(string, tag="1")]
    pub store_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoreResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="4")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="5")]
    pub deleted_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStoresRequest {
    #[prost(message, optional, tag="1")]
    pub page_size: ::core::option::Option<i32>,
    #[prost(string, tag="2")]
    pub continuation_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStoresResponse {
    #[prost(message, repeated, tag="1")]
    pub stores: ::prost::alloc::vec::Vec<Store>,
    #[prost(string, tag="2")]
    pub continuation_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssertionTupleKey {
    #[prost(string, tag="1")]
    pub object: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub relation: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub user: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Assertion {
    #[prost(message, optional, tag="1")]
    pub tuple_key: ::core::option::Option<AssertionTupleKey>,
    #[prost(bool, tag="2")]
    pub expectation: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Assertions {
    #[prost(message, repeated, tag="1")]
    pub assertions: ::prost::alloc::vec::Vec<Assertion>,
}
/// Controls the consistency preferences when calling the query APIs.
/// buf:lint:ignore ENUM_ZERO_VALUE_SUFFIX
/// buf:lint:ignore ENUM_VALUE_PREFIX
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConsistencyPreference {
    /// Default if not set. Behavior will be the same as MINIMIZE_LATENCY
    Unspecified = 0,
    /// Minimize latency at the potential expense of lower consistency.
    MinimizeLatency = 100,
    /// Prefer higher consistency, at the potential expense of increased latency.
    HigherConsistency = 200,
}
impl ConsistencyPreference {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConsistencyPreference::Unspecified => "UNSPECIFIED",
            ConsistencyPreference::MinimizeLatency => "MINIMIZE_LATENCY",
            ConsistencyPreference::HigherConsistency => "HIGHER_CONSISTENCY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "MINIMIZE_LATENCY" => Some(Self::MinimizeLatency),
            "HIGHER_CONSISTENCY" => Some(Self::HigherConsistency),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
