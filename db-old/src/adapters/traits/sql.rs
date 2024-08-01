pub trait IntoSqlInsert {
    fn sql_insert(&self) -> String;
}
