use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime, Utc};
use derive_more::{From, Into};
use prost_types::Timestamp as ProstTimestamp;
use thiserror::Error;
use uuid::Uuid as ExternalUuid;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Invalid timestamp")]
    InvalidTimestamp,
    #[error("Invalid UUID string")]
    InvalidUuidString,
}
//
// #[derive(Debug, Clone, PartialEq, From, Into, sqlx::Type)]
// pub struct InternalUuid(pub uuid::Uuid);
//
// #[derive(Debug, Clone, PartialEq, From, Into, sqlx::Type)]
// pub struct InternalTimestamp(pub chrono::DateTime<chrono::Utc>);

// // Implement TryFrom for String to InternalUuid
// impl TryFrom<String> for InternalUuid {
//     type Error = ConversionError;
//
//     fn try_from(s: String) -> Result<Self, Self::Error> {
//         ExternalUuid::from_str(&s)
//             .map(InternalUuid)
//             .map_err(|_| ConversionError::InvalidUuidString)
//     }
// }
//
// // Implement From for InternalUuid to String
// impl From<InternalUuid> for String {
//     fn from(uuid: InternalUuid) -> Self {
//         uuid.0.to_string()
//     }
// }

// // Implement TryFrom for ProstTimestamp to InternalTimestamp
// impl TryFrom<ProstTimestamp> for InternalTimestamp {
//     type Error = ConversionError;
//
//     fn try_from(ts: ProstTimestamp) -> Result<Self, Self::Error> {
//         let naive = NaiveDateTime::from_timestamp(ts.seconds, ts.nanos as u32);
//         Ok(InternalTimestamp(DateTime::<Utc>::from_utc(naive, Utc)))
//     }
// }
//
// // Implement From for InternalTimestamp to ProstTimestamp
// impl From<InternalTimestamp> for prost_types::Timestamp {
//     fn from(ts: InternalTimestamp) -> Self {
//         ProstTimestamp {
//             seconds: ts.0.timestamp(),
//             nanos: ts.0.timestamp_subsec_nanos() as i32,
//         }
//     }
// }
//

