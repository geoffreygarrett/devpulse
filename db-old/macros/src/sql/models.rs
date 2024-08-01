use deluxe::ExtractAttributes;

#[derive(ExtractAttributes)]
#[deluxe(attributes(table))]
#[derive(Default)]
pub struct SqlInsert {
    pub schema: Option<String>,
    pub name: Option<String>,
}

#[derive(ExtractAttributes, deluxe::ParseMetaItem, Default)]
#[deluxe(attributes(column))]
pub struct Column {
    pub rename: Option<String>,
}

#[derive(ExtractAttributes)]
#[deluxe(attributes(columns))]
pub struct Columns(#[deluxe(flatten)] pub Vec<Column>);

