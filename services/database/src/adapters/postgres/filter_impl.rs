use crate::adapters::postgres::database_pool_impl::PostgresDbPool;
use crate::adapters::traits::filter::FilterToQuery;
use crate::proto::db_filter_v1::{
    CompositeFilter, date_filter, DateFilter, filter_condition, FilterCondition, LogicalOperator,
    number_filter, NumberFilter, QueryRequest, string_filter, StringFilter,
};

pub struct SqlQuery {
    pub query: String,
}

impl FilterToQuery<PostgresDbPool> for FilterCondition {
    type QueryType = String;

    fn to_query(&self) -> String {
        match &self.condition {
            Some(filter_condition::Condition::StringFilter(filter)) => {
                filter.to_query(&self.column)
            }
            Some(filter_condition::Condition::NumberFilter(filter)) => {
                filter.to_query(&self.column)
            }
            Some(filter_condition::Condition::DateFilter(filter)) => filter.to_query(&self.column),
            Some(filter_condition::Condition::BooleanFilter(filter)) => {
                format!("{} = {}", self.column, filter.value)
            }
            Some(filter_condition::Condition::Composite(filter)) => filter.to_query(),
            None => String::new(),
        }
    }
}

impl StringFilter {
    fn to_query(&self, column: &str) -> String {
        match &self.r#type {
            Some(string_filter::Type::Eq(value)) => {
                format!("{} = '{}'", column, value.value.clone().unwrap())
            }
            Some(string_filter::Type::Like(pattern)) => {
                format!("{} LIKE '{}'", column, pattern.pattern)
            }
            Some(string_filter::Type::In(values)) => {
                let vals = values
                    .values
                    .iter()
                    .map(|v| format!("'{}'", v))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{} IN ({})", column, vals)
            }
            None => String::new(),
        }
    }
}

impl NumberFilter {
    fn to_query(&self, column: &str) -> String {
        match &self.r#type {
            Some(number_filter::Type::Lt(filter)) => format!("{} < {}", column, filter.value),
            Some(number_filter::Type::Gt(filter)) => format!("{} > {}", column, filter.value),
            Some(number_filter::Type::Range(range)) => {
                format!("{} BETWEEN {} AND {}", column, range.from, range.to)
            }
            None => String::new(),
        }
    }
}

impl DateFilter {
    fn to_query(&self, column: &str) -> String {
        match &self.r#type {
            Some(date_filter::Type::Eq(filter)) => format!("{} = '{}'", column, filter.value),
            Some(date_filter::Type::Range(range)) => {
                format!("{} BETWEEN '{}' AND '{}'", column, range.from, range.to)
            }
            None => String::new(),
        }
    }
}

impl CompositeFilter {
    fn to_query(&self) -> String {
        let op_str = match LogicalOperator::from_i32(self.operator).unwrap_or(LogicalOperator::And)
        {
            LogicalOperator::And => " AND ",
            LogicalOperator::Or => " OR ",
            LogicalOperator::Not => " NOT ",
        };
        let filter_strings: Vec<String> = self.conditions.iter().map(|f| f.to_query()).collect();
        format!("({})", filter_strings.join(op_str))
    }
}

impl FilterToQuery<PostgresDbPool> for QueryRequest {
    type QueryType = SqlQuery;

    fn to_query(&self) -> SqlQuery {
        let where_clause = self
            .conditions
            .iter()
            .map(|condition| condition.to_query())
            .collect::<Vec<_>>()
            .join(" AND ");
        let query = if !where_clause.is_empty() {
            format!("SELECT * FROM table_name WHERE {}", where_clause)
        } else {
            "SELECT * FROM table_name".to_string()
        };
        SqlQuery { query }
    }
}
