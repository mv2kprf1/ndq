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
pub struct SuiteConfig {
    pub source: String,
    pub schemas: Vec<SchemaExpression>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SchemaExpression {
    pub schema: String,
    pub tables: HashMap<String, HashMap<String, TableExpression>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TableExpression {
    #[serde(flatten)]
    pub args: HashMap<String, ArgValue>,
}
