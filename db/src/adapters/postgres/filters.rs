#[derive(Debug, Clone)]
pub enum Filter {
    Equal(String, String),
    NotEqual(String, String),
    GreaterThan(String, String),
    LessThan(String, String),
    Like(String, String),
    ILike(String, String),
    In(String, Vec<String>),
    NotIn(String, Vec<String>),
}

pub trait ToSqlString {
    fn to_sql_string(&self) -> String;
}

impl ToSqlString for Filter {
    fn to_sql_string(&self) -> String {
        match self {
            Filter::Equal(col, val) => format!("{} = '{}'", col, val),
            Filter::NotEqual(col, val) => format!("{} != '{}'", col, val),
            Filter::GreaterThan(col, val) => format!("{} > '{}'", col, val),
            Filter::LessThan(col, val) => format!("{} < '{}'", col, val),
            Filter::Like(col, val) => format!("{} LIKE '{}'", col, val),
            Filter::ILike(col, val) => format!("{} ILIKE '{}'", col, val),
            Filter::In(col, vals) => format!("{} IN ({})", col, vals.join(", ")),
            Filter::NotIn(col, vals) => format!("{} NOT IN ({})", col, vals.join(", ")),
        }
    }
}

pub struct FilterBuilder {
    filters: Vec<Filter>,
}

impl FilterBuilder {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    pub fn equal(mut self, column: &str, value: &str) -> Self {
        self.filters
            .push(Filter::Equal(column.to_string(), value.to_string()));
        self
    }

    pub fn not_equal(mut self, column: &str, value: &str) -> Self {
        self.filters
            .push(Filter::NotEqual(column.to_string(), value.to_string()));
        self
    }

    pub fn greater_than(mut self, column: &str, value: &str) -> Self {
        self.filters
            .push(Filter::GreaterThan(column.to_string(), value.to_string()));
        self
    }

    pub fn less_than(mut self, column: &str, value: &str) -> Self {
        self.filters
            .push(Filter::LessThan(column.to_string(), value.to_string()));
        self
    }

    pub fn like(mut self, column: &str, pattern: &str) -> Self {
        self.filters
            .push(Filter::Like(column.to_string(), pattern.to_string()));
        self
    }

    pub fn ilike(mut self, column: &str, pattern: &str) -> Self {
        self.filters
            .push(Filter::ILike(column.to_string(), pattern.to_string()));
        self
    }

    pub fn in_values(mut self, column: &str, values: Vec<&str>) -> Self {
        let values = values.into_iter().map(String::from).collect();
        self.filters.push(Filter::In(column.to_string(), values));
        self
    }

    pub fn not_in_values(mut self, column: &str, values: Vec<&str>) -> Self {
        let values = values.into_iter().map(String::from).collect();
        self.filters.push(Filter::NotIn(column.to_string(), values));
        self
    }

    pub fn build(self) -> Vec<Filter> {
        self.filters
    }
}

pub struct QueryParams {
    filters: Vec<Filter>,
    returns: Option<String>,
}

impl QueryParams {
    pub fn new(filters: Vec<Filter>, returns: Option<String>) -> Self {
        Self { filters, returns }
    }

    pub fn returns(&self) -> Option<String> {
        self.returns.clone()
    }

    pub fn filters(&self) -> Vec<Filter> {
        self.filters.clone()
    }

    fn build_where_clause(&self) -> String {
        if self.filters().is_empty() {
            return String::new();
        }

        let filters_str = self
            .filters()
            .iter()
            .map(ToSqlString::to_sql_string)
            .collect::<Vec<_>>()
            .join(" AND ");

        format!(" WHERE {}", filters_str)
    }

    fn build_returning_clause(&self) -> String {
        self.returns()
            .map(|fields| format!(" RETURNING {}", fields))
            .unwrap_or_default()
    }
}
