impl QueryBuilder for Operation<String> {
    fn build(&self, schema: &str, table: &str, filters: &[String], returns: &Option<String>) -> String {
        match self {
            Operation::Insert(values) => format!(
                "INSERT INTO {}.{} ({}) VALUES ({}){}",
                schema,
                table,
                values.join(", "),
                values.iter().map(|_| "?").collect::<Vec<_>>().join(", "),
                returns.as_ref().map(|r| format!(" RETURNING {}", r)).unwrap_or_default(),
            ),
            Operation::Update(values) => format!(
                "UPDATE {}.{} SET {} WHERE {}{}",
                schema,
                table,
                values.iter().map(|v| format!("{} = ?", v)).collect::<Vec<_>>().join(", "),
                filters.join(" AND "),
                returns.as_ref().map(|r| format!(" RETURNING {}", r)).unwrap_or_default(),
            ),
            Operation::Delete => format!(
                "DELETE FROM {}.{} WHERE {}",
                schema,
                table,
                filters.join(" AND ")
            ),
            Operation::Upsert(values, config) => {
                let conflict_clause = config.on_conflict.as_ref().map_or(String::new(), |field| format!(" ON CONFLICT({}) DO UPDATE SET ", field));
                format!(
                    "INSERT INTO {}.{} ({}) VALUES ({}){}{}",
                    schema,
                    table,
                    values.join(", "),
                    values.iter().map(|_| "?").collect::<Vec<_>>().join(", "),
                    conflict_clause,
                    returns.as_ref().map(|r| format!(" RETURNING {}", r)).unwrap_or_default(),
                )
            },
            Operation::Select(columns) => format!(
                "SELECT {} FROM {}.{} WHERE {}",
                columns,
                schema,
                table,
                filters.join(" AND ")
            ),
        }
    }
}


#[derive(Debug, Clone)]
enum Filter {
    Equal(String, String),
    NotEqual(String, String),
    GreaterThan(String, String),
    LessThan(String, String),
    Like(String, String),
    ILike(String, String),
    In(String, Vec<String>),
    NotIn(String, Vec<String>),
}

struct FilterBuilder {
    filters: Vec<Filter>,
}

#[derive(Debug)]
struct QueryStageOne {
    schema: Option<String>,
    table: String,
}

impl QueryStageOne {
    fn into_stage_two<T>(self, operation: Operation<T>) -> QueryStageTwo<T> {
        QueryStageTwo {
            operation,
            schema: self.schema,
            table: self.table,
        }
    }
}

#[derive(Debug)]
struct QueryStageTwo<T> {
    schema: Option<String>,
    table: String,
    operation: Operation<T>,
}

impl<T> QueryStageTwo<T> {
    fn into_stage_three(self, filters: Vec<String>, returns: Option<String>) -> QueryStageThree<T> {
        QueryStageThree {
            schema: self.schema.unwrap_or_else(|| "public".to_string()),
            table: self.table,
            operation: self.operation,
            filters,
            returns,
        }
    }
}

#[derive(Debug)]
struct QueryStageThree<T> {
    schema: String,
    table: String,
    operation: Operation<T>,
    filters: Vec<String>,
    returns: Option<String>,
}


impl FilterBuilder {
    fn new() -> Self {
        Self { filters: Vec::new() }
    }

    fn equal(mut self, column: &str, value: &str) -> Self {
        self.filters.push(Filter::Equal(column.to_string(), value.to_string()));
        self
    }

    fn not_equal(mut self, column: &str, value: &str) -> Self {
        self.filters.push(Filter::NotEqual(column.to_string(), value.to_string()));
        self
    }

    fn greater_than(mut self, column: &str, value: &str) -> Self {
        self.filters.push(Filter::GreaterThan(column.to_string(), value.to_string()));
        self
    }

    fn less_than(mut self, column: &str, value: &str) -> Self {
        self.filters.push(Filter::LessThan(column.to_string(), value.to_string()));
        self
    }

    fn like(mut self, column: &str, pattern: &str) -> Self {
        self.filters.push(Filter::Like(column.to_string(), pattern.to_string()));
        self
    }

    fn ilike(mut self, column: &str, pattern: &str) -> Self {
        self.filters.push(Filter::ILike(column.to_string(), pattern.to_string()));
        self
    }

    fn in_values(mut self, column: &str, values: Vec<&str>) -> Self {
        let values = values.into_iter().map(String::from).collect();
        self.filters.push(Filter::In(column.to_string(), values));
        self
    }

    fn not_in_values(mut self, column: &str, values: Vec<&str>) -> Self {
        let values = values.into_iter().map(String::from).collect();
        self.filters.push(Filter::NotIn(column.to_string(), values));
        self
    }

    fn build(self) -> Vec<String> {
        self.filters.iter().map(|filter| match filter {
            Filter::Equal(column, value) => format!("{} = '{}'", column, value),
            Filter::NotEqual(column, value) => format!("{} != '{}'", column, value),
            Filter::GreaterThan(column, value) => format!("{} > '{}'", column, value),
            Filter::LessThan(column, value) => format!("{} < '{}'", column, value),
            Filter::Like(column, pattern) => format!("{} LIKE '{}'", column, pattern),
            Filter::ILike(column, pattern) => format!("{} ILIKE '{}'", column, pattern),
            Filter::In(column, values) => format!("{} IN ({})", column, values.join(", ")),
            Filter::NotIn(column, values) => format!("{} NOT IN ({})", column, values.join(", ")),
        }).collect()
    }
}



// #[derive(Debug, Clone)]
// enum Filter {
//     Equal(String, String),
//     NotEqual(String, String),
//     GreaterThan(String, String),
//     LessThan(String, String),
//     Like(String, String),
//     ILike(String, String),
//     In(String, Vec<String>),
//     NotIn(String, Vec<String>),
// }
//
// enum Operation {
//     Insert,
//     Update,
//     Delete,
//     Upsert(UpsertConfig),
//     Select,
// }
//
// #[derive(Debug, Clone)]
// struct UpsertConfig {
//     on_conflict: Option<String>,
// }
//
// struct Client {
//     schema: String,
//     table: String,
//     operation: Operation,
//     filters: Vec<Filter>,
//     returns: Vec<String>, // List of columns to return
// }
//
// impl Client {
//     fn schema(schema: &str) -> Self {
//         Self {
//             schema: schema.to_string(),
//             table: String::new(),
//             operation: Operation::Select, // Default to select for initialization
//             filters: vec![],
//             returns: vec![],
//         }
//     }
//
//     fn from(mut self, table: &str) -> Self {
//         self.table = table.to_string();
//         self
//     }
//
//     fn operation(mut self, op: Operation) -> Self {
//         self.operation = op;
//         self
//     }
//
//     // Filter methods
//     fn eq(mut self, column: &str, value: &str) -> Self {
//         self.filters.push(Filter::Equal(column.to_string(), value.to_string()));
//         self
//     }
//
//     // Additional filter methods can be added here in a similar fashion
//
//     // Execute method to build the final SQL query
//     fn execute(&self) -> String {
//         // Logic to build the SQL query based on operation and filters
//         // This is a placeholder for actual SQL generation logic
//         format!("Executing operation on schema: '{}', table: '{}'", self.schema, self.table)
//     }
// }
//
// // Example of usage:
// let query = Client::schema("public")
//    .from("users")
//    .operation(Operation::Delete) // Define the type of operation (Insert, Update, Delete, Upsert, Select)
//    .eq("id", "xyz")
//    .execute();
//
// println!("{}", query);