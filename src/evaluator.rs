use crate::types::{ArgValue, SuiteConfig, TableExpression};
use std::collections::HashMap;

type ColumnName = String;
type CheckArgs = HashMap<String, ArgValue>;
type MappedTableExpression = HashMap<ColumnName, CheckArgs>;
type MappedTableExpressions = Vec<MappedTableExpression>;

fn map_table_expression(table_expression: &TableExpression<ArgValue>) -> MappedTableExpression {
    table_expression
        .columns
        .iter()
        .map(|column| (column.clone(), table_expression.args.clone()))
        .collect()
}

pub fn execute_suite(suite_config: &SuiteConfig<ArgValue>) {
    let mapped_expressions: MappedTableExpressions = suite_config
        .schemas
        .iter()
        .flat_map(|schema| &schema.tables)
        .flat_map(|(_, expressions)| expressions)
        .map(|(_, expr)| map_table_expression(expr))
        .collect();

    mapped_expressions
        .iter()
        .for_each(|expr_map| println!("{:#?}", expr_map));
}
