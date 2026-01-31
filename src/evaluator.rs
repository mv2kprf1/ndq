use crate::types::{ArgValue, SuiteConfig, TableExpression};
use std::collections::HashMap;

type CheckArgs = HashMap<String, ArgValue>;

pub fn execute_suite(suite_config: &SuiteConfig) {
    let mapped_expressions: Vec<(&str, &str, &str, &CheckArgs)> = suite_config
        .schemas
        .iter()
        .flat_map(|schema| {
            schema.tables.iter().flat_map(move |(table_name, checks)| {
                checks.iter().map(move |(check_name, table_expr)| {
                    (
                        schema.schema.as_str(),
                        table_name.as_str(),
                        check_name.as_str(),
                        &table_expr.args,
                    )
                })
            })
        })
        .collect();

    mapped_expressions
        .iter()
        .for_each(|(schema, table, check, args)| {
            println!("Schema: {}, Table: {}, Check: {}", schema, table, check);
            println!("{:#?}\n", args);
        });
}
