use mongodb::bson::{doc, Document};

use crate::adapters::mongo::MongoDbPool;
use crate::proto::db_filter_v1::{
    BooleanFilter, CompositeFilter, DateFilter, FilterCondition, LogicalOperator, NumberFilter,
    QueryRequest, StringFilter,
};

pub struct MongoQuery {
    pub query: Document,
}

impl FilterToQuery<MongoDbPool> for FilterCondition {
    type QueryType = Document;

    fn to_query(&self) -> Document {
        match &self.condition {
            Some(filter_condition::Condition::StringFilter(filter)) => {
                filter.to_query(&self.column)
            }
            Some(filter_condition::Condition::NumberFilter(filter)) => {
                filter.to_query(&self.column)
            }
            Some(filter_condition::Condition::DateFilter(filter)) => filter.to_query(&self.column),
            Some(filter_condition::Condition::BooleanFilter(filter)) => {
                doc! { &self.column: filter.value }
            }
            Some(filter_condition::Condition::Composite(filter)) => filter.to_query(),
            None => Document::new(),
        }
    }
}

impl StringFilter {
    fn to_query(&self, column: &str) -> Document {
        match &self.r#type {
            Some(string_filter::Type::Eq(value)) => doc! { column: value.value.clone().unwrap() },
            Some(string_filter::Type::Like(pattern)) => {
                doc! { column: { "$regex": &pattern.pattern } }
            }
            Some(string_filter::Type::In(values)) => doc! { column: { "$in": &values.values } },
            None => Document::new(),
        }
    }
}

impl NumberFilter {
    fn to_query(&self, column: &str) -> Document {
        match &self.r#type {
            Some(number_filter::Type::Lt(filter)) => doc! { column: { "$lt": filter.value } },
            Some(number_filter::Type::Gt(filter)) => doc! { column: { "$gt": filter.value } },
            Some(number_filter::Type::Range(range)) => {
                doc! { column: { "$gte": range.from, "$lte": range.to } }
            }
            None => Document::new(),
        }
    }
}

impl DateFilter {
    fn to_query(&self, column: &str) -> Document {
        match &self.r#type {
            Some(date_filter::Type::Eq(filter)) => doc! { column: filter.value.clone() },
            Some(date_filter::Type::Range(range)) => {
                doc! { column: { "$gte": &range.from, "$lte": &range.to } }
            }
            None => Document::new(),
        }
    }
}

impl CompositeFilter {
    fn to_query(&self) -> Document {
        let op_str = match LogicalOperator::from_i32(self.operator).unwrap_or(LogicalOperator::And)
        {
            LogicalOperator::And => "$and",
            LogicalOperator::Or => "$or",
            LogicalOperator::Not => "$not",
        };
        let filter_docs: Vec<Document> = self.conditions.iter().map(|f| f.to_query()).collect();
        doc! { op_str: filter_docs }
    }
}

impl FilterToQuery<MongoDbPool> for QueryRequest {
    type QueryType = MongoQuery;

    fn to_query(&self) -> MongoQuery {
        let mut query_doc = Document::new();
        for condition in &self.conditions {
            let condition_doc = condition.to_query();
            for (k, v) in condition_doc.iter() {
                query_doc.insert(k.clone(), v.clone());
            }
        }
        MongoQuery { query: query_doc }
    }
}
