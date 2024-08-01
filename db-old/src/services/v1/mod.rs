#![cfg(feature = "server")]
use derive_more::{AsMut, AsRef, Deref, DerefMut, Display, Error, From};
use sqlx::{Database, Decode, Encode, FromRow, Postgres};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::postgres::PgArgumentBuffer;

pub mod account;
pub(crate) mod identity;
pub(crate) mod refresh_token;



// trait ProxyEncode<'q, T> {
//     fn encode(self, buf: &mut <T as Database>::ArgumentBuffer<'q>) -> Result<IsNull, BoxDynError>;
//     fn encode_by_ref(
//         &self, buf: &mut <T as Database>::ArgumentBuffer<'q>,
//     ) -> Result<IsNull, BoxDynError>;
//     fn size_hint(&self) -> usize;
// }

// impl<'q, T> Encode<'q, Postgres> for GenericWrapper<T>
// where
//     T: Encode<'q, Postgres> + Clone,
// {
//     fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
//         self.as_ref().encode(buf)
//     }
//
//     fn encode_by_ref(
//         &self, buf: &mut <Postgres as Database>::ArgumentBuffer<'q>,
//     ) -> Result<IsNull, BoxDynError> {
//         self.as_ref().encode(buf)
//     }
//
//     fn size_hint(&self) -> usize {
//         self.as_ref().size_hint()
//     }
// }

// impl<'r, T> Decode<'r, Postgres> for GenericWrapper<T>
// where
//     T: Decode<'r, Postgres>,
// {
//     fn decode(value: <Postgres as sqlx::Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
//         Ok(GenericWrapper(T::decode(value)?))
//     }
// }

// impl<'r, T> FromRow<'r, PgRow> for GenericWrapper<T>
// where
//     T: FromRow<'r, PgRow>,
// {
//     fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
//         Ok(GenericWrapper(T::from_row(row)?))
//     }
// }

#[derive(Debug, Clone, From, AsRef, AsMut, Deref, DerefMut, Display, Error)]
pub struct GenericWrapper<T>(pub T);

trait Wrapper {
    type Inner;
    fn new(inner: Self::Inner) -> Self;
    fn inner(&self) -> &Self::Inner;
    fn inner_mut(&mut self) -> &mut Self::Inner;
}

trait Wrap<T> {
    fn wrap(self) -> GenericWrapper<T>;
}

impl<T> Wrap<T> for T {
    fn wrap(self) -> GenericWrapper<T> {
        GenericWrapper(self)
    }
}

impl<T> Wrapper for GenericWrapper<T> {
    type Inner = T;

    fn new(inner: Self::Inner) -> Self {
        Self(inner)
    }

    fn inner(&self) -> &Self::Inner {
        &self.as_ref()
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        self.as_mut()
    }
}

// test impl of encode for timestamp now which transforms it into a sqlx::types::chrono::DateTime<Utc>
impl<'q> Encode<'q, Postgres> for GenericWrapper<prost_types::Timestamp> {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        let ts = self.inner();
        sqlx::types::chrono::DateTime::<chrono::Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp(ts.seconds, ts.nanos as u32),
            chrono::Utc,
        )
        .encode(buf)
    }

    fn encode_by_ref(
        &self, buf: &mut <Postgres as Database>::ArgumentBuffer<'q>,
    ) -> Result<IsNull, BoxDynError> {
        let ts = self.inner();
        sqlx::types::chrono::DateTime::<chrono::Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp(ts.seconds, ts.nanos as u32),
            chrono::Utc,
        )
        .encode(buf)
    }

    fn size_hint(&self) -> usize {
        let ts = self.inner();
        sqlx::types::chrono::DateTime::<chrono::Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp(ts.seconds, ts.nanos as u32),
            chrono::Utc,
        )
        .size_hint()
    }
}

impl<'r> Decode<'r, Postgres> for GenericWrapper<prost_types::Timestamp> {
    fn decode(value: <Postgres as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        let ts = sqlx::types::chrono::DateTime::<chrono::Utc>::decode(value)?;
        Ok(GenericWrapper::new(prost_types::Timestamp {
            seconds: ts.timestamp(),
            nanos: ts.timestamp_subsec_nanos() as i32,
        }))
    }
}

#[cfg(test)]
mod tests {
    use sqlx::postgres::PgArgumentBuffer;
    use sqlx::types::chrono::{NaiveDateTime, Utc};
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_wrapper() {
        let uuid = Uuid::new_v4();
        let ts = NaiveDateTime::from_timestamp(0, 0);
        let prost_datetime = prost_types::Timestamp {
            seconds: 0,
            nanos: 0,
        };

        let wrapped_prost_datetime = prost_datetime.wrap();
        let wrapped_uuid = uuid.wrap();
        let wrapped_ts = ts.wrap();

        // Encode wrapped values
        let mut buf = PgArgumentBuffer::default();
        let _ = wrapped_prost_datetime.encode_by_ref(&mut buf).unwrap();
        let encoded_wrapped_prost_datetime = buf.clone();
        // let x: PgValueRef = buf.as_ref().into();

        buf.clear(); // Clear buffer for the next encoding
        let _ = wrapped_uuid.encode_by_ref(&mut buf).unwrap();
        let encoded_wrapped_uuid = buf.clone();

        buf.clear(); // Clear buffer for the next encoding
        let _ = wrapped_ts.encode_by_ref(&mut buf).unwrap();
        let encoded_wrapped_ts = buf.clone();

        // Decode wrapped values
        // let decoded_wrapped_prost_datetime = GenericWrapper::<prost_types::Timestamp>::decode(&encoded_wrapped_prost_datetime).unwrap();
        let decoded_wrapped_uuid =
            GenericWrapper::<uuid::Uuid>::decode(&encoded_wrapped_uuid).unwrap();
        let decoded_wrapped_ts =
            GenericWrapper::<chrono::DateTime<Utc>>::decode(&encoded_wrapped_ts).unwrap();

        // Assertions
        assert_eq!(wrapped_uuid.inner(), &uuid);
        assert_eq!(wrapped_ts.inner(), &ts);
        assert_eq!(wrapped_prost_datetime.inner(), &prost_datetime);
        // assert_eq!(decoded_wrapped_prost_datetime.inner(), &prost_datetime);
        assert_eq!(decoded_wrapped_uuid.inner(), &uuid);
        assert_eq!(decoded_wrapped_ts.inner(), &ts);
    }
}
