impl From<crate::types::v1::Uuid> for sqlx::types::Uuid {
    fn from(uuid: crate::types::v1::Uuid) -> Self {
        sqlx::types::Uuid::from(uuid)
    }
}

impl From<sqlx::types::Uuid> for crate::types::v1::Uuid {
    fn from(uuid: sqlx::types::Uuid) -> Self {
        crate::types::v1::Uuid {
            value: uuid.to_string(),
        }
    }
}

// impl<'q> Encode<'q, Postgres> for crate::types::v1::Uuid {
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
//         Some(<Postgres as Database>::TypeInfo::with_name("uuid"))
//     }
//
//     fn size_hint(&self) -> usize {
//         mem::size_of_val(self)
//     }
// }
//
// impl<'r> Decode<'r, Postgres> for crate::types::v1::Uuid {
//     fn decode(
//         value: <Postgres as Database>::ValueRef<'r>,
//     ) -> Result<Self, Box<dyn Error + Send + Sync>> {
//         let value_str = <&str as Decode<Postgres>>::decode(value)?;
//         Ok(crate::types::v1::Uuid {
//             value: value_str.parse()?,
//         })
//     }
// }

// impl Type<Postgres> for crate::types::v1::Uuid {
//     fn type_info() -> PgTypeInfo {
//         <Postgres as Database>::TypeInfo::with_name("uuid")
//     }
// }

// impl From<crate::types::v1::Uuid> for  Option<crate::types::v1::Uuid> {
//     fn from(uuid: crate::types::v1::Uuid) -> Self {
//         Some(uuid)
//     }
// }

// impl Type<Postgres> for Vec<crate::types::v1::Uuid>
// {
//     #[inline]
//     fn type_info() -> PgTypeInfo {
//         <crate::types::v1::Uuid as Type<Postgres>>::type_info()
//     }
// }

// pub trait PgHasArrayType {
//     // Required method
//     fn array_type_info() -> PgTypeInfo;
//
//     // Provided method
//     fn array_compatible(ty: &PgTypeInfo) -> bool { ... }
// }

// impl PgHasArrayType for crate::types::v1::Uuid {
//     fn array_type_info() -> PgTypeInfo {
//         <Postgres as Database>::TypeInfo::with_name("uuid[]")
//     }
// }

//
// impl PgHasArrayType for crate::types::v1::Uuid {
//     fn array_type_info() -> PgTypeInfo {
//         <Postgres as Database>::TypeInfo::with_name("UUID[]")
//     }
// }
