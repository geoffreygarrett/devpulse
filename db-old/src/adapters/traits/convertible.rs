use std::error::Error;
use std::str::FromStr;

pub use chrono::NaiveDateTime;
pub use prost_types::Timestamp;
use sqlx::Error as SqlxError;
// use sqlx::types::chrono::T as SqlxNaiveDateTime;
use uuid::Uuid as ExternalUuid;

pub trait Convertible<DBType> {
    type Error: Error;

    fn to_db_type(&self) -> Result<DBType, Self::Error>;
    fn from_db_type(db_type: DBType) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl Convertible<ExternalUuid> for String {
    type Error = SqlxError;

    fn to_db_type(&self) -> Result<ExternalUuid, Self::Error> {
        ExternalUuid::from_str(self).map_err(|e| {
            SqlxError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid UUID: {}", e),
            ))
        })
    }

    fn from_db_type(db_type: ExternalUuid) -> Result<Self, Self::Error> {
        Ok(db_type.to_string())
    }
}

impl Convertible<NaiveDateTime> for Timestamp {
    type Error = SqlxError;

    fn to_db_type(&self) -> Result<NaiveDateTime, Self::Error> {
        Ok(NaiveDateTime::from_timestamp(self.seconds, self.nanos as u32))
    }

    fn from_db_type(db_type: NaiveDateTime) -> Result<Self, Self::Error> {
        Ok(Timestamp {
            seconds: db_type.timestamp(),
            nanos: db_type.timestamp_subsec_nanos() as i32,
        })
    }
}

impl Convertible<NaiveDateTime> for Option<Timestamp> {
    type Error = SqlxError;

    fn to_db_type(&self) -> Result<NaiveDateTime, Self::Error> {
        match self {
            Some(ts) => ts.to_db_type(),
            None => Err(SqlxError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Timestamp is None",
            ))),
        }
    }

    fn from_db_type(db_type: NaiveDateTime) -> Result<Self, Self::Error> {
        Ok(Some(Timestamp {
            seconds: db_type.timestamp(),
            nanos: db_type.timestamp_subsec_nanos() as i32,
        }))
    }
}

impl  Convertible<Option<NaiveDateTime>> for Option<Timestamp> {
    type Error = SqlxError;

    fn to_db_type(&self) -> Result<Option<NaiveDateTime>, Self::Error> {
        match self {
            Some(ts) => ts.to_db_type().map(Some),
            None => Ok(None),
        }
    }

    fn from_db_type(db_type: Option<NaiveDateTime>) -> Result<Self, Self::Error> {
        match db_type {
            Some(ts) => Ok(Some(Timestamp {
                seconds: ts.timestamp(),
                nanos: ts.timestamp_subsec_nanos() as i32,
            })),
            None => Ok(None),
        }
    }
}


