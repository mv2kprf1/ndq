use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum ArgValue {
    Float(f64),
    Int(i64),
    Str(String),
    Bool(bool),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SuiteConfig<T> {
    source: String,
    pub schemas: Vec<SchemaExpression<T>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SchemaExpression<T> {
    pub schema: String,
    pub tables: HashMap<String, HashMap<String, TableExpression<T>>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TableExpression<T> {
    pub columns: Vec<String>,
    #[serde(flatten)]
    pub args: HashMap<String, T>,
}
