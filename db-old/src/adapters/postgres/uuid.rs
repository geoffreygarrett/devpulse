use std::error::Error;
use std::mem;
use sqlx::{Decode, Encode, encode::IsNull, TypeInfo};
use sqlx::{Database, Postgres};
use sqlx::postgres::PgTypeInfo;
use sqlx::types::Type;
use crate::proto::db_auth_v1;


// From<sqlx::types::Uuid>` is not implemented for `Option<db_auth_v1::Uuid>`,

// impl From<db_auth_v1::Uuid> for sqlx::types::Uuid {
//     fn from(uuid: db_auth_v1::Uuid) -> Self {
//         sqlx::types::Uuid::from(uuid)
//     }
// }
//
// impl From<sqlx::types::Uuid> for db_auth_v1::Uuid {
//     fn from(uuid: sqlx::types::Uuid) -> Self {
//         db_auth_v1::Uuid {
//             value: uuid.to_string(),
//         }
//     }
// }
// impl<'q> Encode<'q, Postgres> for db_auth_v1::Uuid {
//     fn encode(
//         self, buf: &mut <Postgres as Database>::ArgumentBuffer<'q>,
//     ) -> Result<IsNull, Box<dyn Error + Send + Sync>> {
//         self.encode_by_ref(buf)
//     }
//
//     fn encode_by_ref(
//         &self, buf: &mut <Postgres as Database>::ArgumentBuffer<'q>,
//     ) -> Result<IsNull, Box<dyn Error + Send + Sync>> {
//         <&str as Encode<Postgres>>::encode(&self.value.to_string(), buf)
//     }
//
//     fn produces(&self) -> Option<<Postgres as Database>::TypeInfo> {
//         Some(<Postgres as Database>::TypeInfo::with_name("UUID"))
//     }
//
//     fn size_hint(&self) -> usize {
//         mem::size_of_val(self)
//     }
// }
//
// impl<'r> Decode<'r, Postgres> for db_auth_v1::Uuid {
//     fn decode(
//         value: <Postgres as Database>::ValueRef<'r>,
//     ) -> Result<Self, Box<dyn Error + Send + Sync>> {
//         let value_str = <&str as Decode<Postgres>>::decode(value)?;
//         Ok(db_auth_v1::Uuid {
//             value: value_str.parse()?,
//         })
//     }
// }
//
// impl Type<Postgres> for db_auth_v1::Uuid {
//     fn type_info() -> PgTypeInfo {
//         <Postgres as Database>::TypeInfo::with_name("UUID")
//     }
// }
