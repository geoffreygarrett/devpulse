use std::error::Error;

pub trait DbTypeAdapter<DBType> {
    type Error: Error;

    fn to_db_type(&self) -> Result<DBType, Self::Error>;
    fn from_db_type(db_type: DBType) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
