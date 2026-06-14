#[cfg(any(target_arch = "wasm32", test))]
use serde::Serialize;
#[cfg(any(target_arch = "wasm32", test))]
use sqlparser::{
    ast::{Expr, LimitClause, Select, SelectItem, SetExpr, Statement, TableFactor, Value},
    dialect::GenericDialect,
    parser::Parser,
};
#[cfg(any(target_arch = "wasm32", test))]
use std::collections::{BTreeMap, HashSet};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Default)]
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct QueryEngine {
    registered_tables: Vec<RegisteredCsvTable>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg(any(target_arch = "wasm32", test))]
struct CsvSchema {
    columns: Vec<ColumnSchema>,
    row_count: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg(any(target_arch = "wasm32", test))]
struct QueryResult {
    rows: Vec<QueryRow>,
    schema: Vec<ColumnSchema>,
    row_count: usize,
    elapsed_ms: f64,
}

#[cfg(any(target_arch = "wasm32", test))]
type QueryRow = BTreeMap<String, Option<String>>;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg(any(target_arch = "wasm32", test))]
struct ColumnSchema {
    name: String,
    data_type: String,
    nullable: bool,
    null_count: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg(target_arch = "wasm32")]
struct RegisteredCsvSchema {
    name: String,
    columns: Vec<ColumnSchema>,
    row_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg(any(target_arch = "wasm32", test))]
struct RegisteredCsvTable {
    name: String,
    schema: CsvSchema,
    rows: Vec<Vec<Option<String>>>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg(any(target_arch = "wasm32", test))]
struct SelectedColumn {
    source_index: usize,
    output_name: String,
    schema: ColumnSchema,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg(any(target_arch = "wasm32", test))]
enum InferredType {
    Boolean,
    Int64,
    Float64,
    Utf8,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl QueryEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_csv(&mut self, name: &str, data: &[u8]) -> Result<JsValue, JsValue> {
        if name.trim().is_empty() {
            return Err(JsValue::from_str("CSV table name is required."));
        }

        if data.is_empty() {
            return Err(JsValue::from_str("CSV data is empty."));
        }

        let table = parse_csv_table(name, data).map_err(|error| JsValue::from_str(&error))?;
        let registered_schema = RegisteredCsvSchema {
            name: table.name.clone(),
            columns: table.schema.columns.clone(),
            row_count: table.schema.row_count,
        };

        if self
            .registered_tables
            .iter()
            .any(|registered_table| identifiers_match(&registered_table.name, &table.name))
        {
            return Err(JsValue::from_str("CSV table name is already registered."));
        }

        self.registered_tables.push(table);

        serde_wasm_bindgen::to_value(&registered_schema)
            .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    pub fn execute(&self, sql: &str) -> Result<JsValue, JsValue> {
        if sql.trim().is_empty() {
            return Err(JsValue::from_str("SQL query is required."));
        }

        let started_at = js_sys::Date::now();
        let mut result = execute_registered_query(sql, &self.registered_tables)
            .map_err(|error| JsValue::from_str(&error))?;
        result.elapsed_ms = js_sys::Date::now() - started_at;

        serde_wasm_bindgen::to_value(&result).map_err(|error| JsValue::from_str(&error.to_string()))
    }

    pub fn table_count(&self) -> usize {
        self.registered_tables.len()
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn infer_schema(data: &[u8]) -> Result<JsValue, JsValue> {
    if data.is_empty() {
        return Err(JsValue::from_str("CSV data is empty."));
    }

    let schema = infer_csv_schema(data).map_err(|error| JsValue::from_str(&error))?;

    serde_wasm_bindgen::to_value(&schema).map_err(|error| JsValue::from_str(&error.to_string()))
}

#[cfg(any(target_arch = "wasm32", test))]
fn infer_csv_schema(data: &[u8]) -> Result<CsvSchema, String> {
    parse_csv_table("__schema__", data).map(|table| table.schema)
}

#[cfg(any(target_arch = "wasm32", test))]
fn parse_csv_table(name: &str, data: &[u8]) -> Result<RegisteredCsvTable, String> {
    let name = name.trim();

    if name.is_empty() {
        return Err("CSV table name is required.".to_owned());
    }

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .trim(csv::Trim::All)
        .from_reader(data);

    let headers = reader
        .headers()
        .map_err(|error| format!("CSV header row is invalid: {error}"))?
        .clone();

    if headers.is_empty() {
        return Err("CSV header row is missing.".to_owned());
    }

    let mut seen_headers = HashSet::new();
    let mut columns = headers
        .iter()
        .map(|name| {
            let name = name.trim();

            if name.is_empty() {
                return Err("CSV header names are required.".to_owned());
            }

            if !seen_headers.insert(name.to_owned()) {
                return Err(format!("CSV header name `{name}` is duplicated."));
            }

            Ok(ColumnSchema {
                name: name.to_owned(),
                data_type: "Utf8".to_owned(),
                nullable: false,
                null_count: 0,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut inferred_types = vec![None; columns.len()];
    let mut rows = Vec::new();

    for record in reader.records() {
        let record = record.map_err(|error| format!("CSV row is invalid: {error}"))?;

        if record.len() > columns.len() {
            return Err(format!(
                "CSV row {} has more fields than the header row.",
                rows.len() + 1
            ));
        }

        let mut row = vec![None; columns.len()];

        for (index, column) in columns.iter_mut().enumerate() {
            let Some(value) = record.get(index).map(str::trim) else {
                column.nullable = true;
                column.null_count += 1;
                continue;
            };

            if value.is_empty() {
                column.nullable = true;
                column.null_count += 1;
                continue;
            }

            row[index] = Some(value.to_owned());
            let value_type = infer_value_type(value);
            inferred_types[index] = Some(merge_inferred_type(inferred_types[index], value_type));
        }

        rows.push(row);
    }

    for (column, inferred_type) in columns.iter_mut().zip(inferred_types) {
        column.data_type = inferred_type
            .unwrap_or(InferredType::Utf8)
            .as_str()
            .to_owned();
    }

    Ok(RegisteredCsvTable {
        name: name.to_owned(),
        schema: CsvSchema {
            row_count: rows.len(),
            columns,
        },
        rows,
    })
}

#[cfg(any(target_arch = "wasm32", test))]
fn execute_registered_query(
    sql: &str,
    registered_tables: &[RegisteredCsvTable],
) -> Result<QueryResult, String> {
    if registered_tables.is_empty() {
        return Err("Register at least one CSV table before executing SQL.".to_owned());
    }

    let dialect = GenericDialect {};
    let statements = Parser::parse_sql(&dialect, sql)
        .map_err(|error| format!("SQL query is invalid: {error}"))?;

    let [statement] = statements.as_slice() else {
        return Err("SQL query must contain exactly one statement.".to_owned());
    };

    let Statement::Query(query) = statement else {
        return Err("Only SELECT queries are supported.".to_owned());
    };

    if query.with.is_some()
        || query.order_by.is_some()
        || query.fetch.is_some()
        || !query.locks.is_empty()
        || query.for_clause.is_some()
        || query.settings.is_some()
        || query.format_clause.is_some()
        || !query.pipe_operators.is_empty()
    {
        return Err("Only simple SELECT queries are supported.".to_owned());
    }

    let select = match query.body.as_ref() {
        SetExpr::Select(select) => select,
        _ => return Err("Only simple SELECT queries are supported.".to_owned()),
    };

    validate_simple_select(select)?;

    let table_name = select_table_name(select)?;
    let table = registered_tables
        .iter()
        .find(|table| identifiers_match(&table.name, &table_name))
        .ok_or_else(|| format!("CSV table `{table_name}` is not registered."))?;

    let selected_columns = select_columns(select, table)?;
    let limit = query
        .limit_clause
        .as_ref()
        .map(parse_limit_clause)
        .transpose()?;

    let mut rows = Vec::new();
    for source_row in table.rows.iter().take(limit.unwrap_or(usize::MAX)) {
        let row = selected_columns
            .iter()
            .map(|column| {
                (
                    column.output_name.clone(),
                    source_row[column.source_index].clone(),
                )
            })
            .collect::<BTreeMap<_, _>>();

        rows.push(row);
    }

    Ok(QueryResult {
        row_count: rows.len(),
        schema: selected_columns
            .into_iter()
            .map(|column| column.schema)
            .collect(),
        rows,
        elapsed_ms: 0.0,
    })
}

#[cfg(any(target_arch = "wasm32", test))]
fn validate_simple_select(select: &Select) -> Result<(), String> {
    if select.distinct.is_some()
        || select.select_modifiers.is_some()
        || select.top.is_some()
        || select.into.is_some()
        || !select.lateral_views.is_empty()
        || select.prewhere.is_some()
        || select.selection.is_some()
        || !select.connect_by.is_empty()
        || !select.cluster_by.is_empty()
        || !select.distribute_by.is_empty()
        || !select.sort_by.is_empty()
        || select.having.is_some()
        || !select.named_window.is_empty()
        || select.qualify.is_some()
    {
        return Err("Only projection, FROM, and LIMIT are supported.".to_owned());
    }

    Ok(())
}

#[cfg(any(target_arch = "wasm32", test))]
fn select_table_name(select: &Select) -> Result<String, String> {
    let [table_with_joins] = select.from.as_slice() else {
        return Err("SELECT query must reference exactly one table.".to_owned());
    };

    if !table_with_joins.joins.is_empty() {
        return Err("JOIN queries are not supported yet.".to_owned());
    }

    match &table_with_joins.relation {
        TableFactor::Table {
            name,
            alias: _,
            args,
            with_hints,
            version,
            with_ordinality,
            partitions,
            json_path,
            sample,
            index_hints,
        } if args.is_none()
            && with_hints.is_empty()
            && version.is_none()
            && !with_ordinality
            && partitions.is_empty()
            && json_path.is_none()
            && sample.is_none()
            && index_hints.is_empty() =>
        {
            Ok(name.to_string())
        }
        _ => Err("Only direct table references are supported.".to_owned()),
    }
}

#[cfg(any(target_arch = "wasm32", test))]
fn select_columns(
    select: &Select,
    table: &RegisteredCsvTable,
) -> Result<Vec<SelectedColumn>, String> {
    let mut selected_columns = Vec::new();

    for item in &select.projection {
        match item {
            SelectItem::Wildcard(_) => {
                selected_columns.extend(table.schema.columns.iter().enumerate().map(
                    |(source_index, column)| SelectedColumn {
                        source_index,
                        output_name: column.name.clone(),
                        schema: column.clone(),
                    },
                ));
            }
            SelectItem::UnnamedExpr(expr) => {
                selected_columns.push(select_expression_column(expr, table, None)?);
            }
            SelectItem::ExprWithAlias { expr, alias } => {
                selected_columns.push(select_expression_column(expr, table, Some(&alias.value))?);
            }
            SelectItem::QualifiedWildcard(_, _) => {
                return Err("Qualified wildcards are not supported yet.".to_owned());
            }
        }
    }

    if selected_columns.is_empty() {
        return Err("SELECT query must include at least one column.".to_owned());
    }

    Ok(selected_columns)
}

#[cfg(any(target_arch = "wasm32", test))]
fn select_expression_column(
    expr: &Expr,
    table: &RegisteredCsvTable,
    alias: Option<&str>,
) -> Result<SelectedColumn, String> {
    let column_name = match expr {
        Expr::Identifier(identifier) => identifier.value.as_str(),
        Expr::CompoundIdentifier(identifiers) if identifiers.len() == 2 => {
            let table_name = &identifiers[0].value;
            if !identifiers_match(table_name, &table.name) {
                return Err(format!(
                    "Column qualifier `{table_name}` does not match the table."
                ));
            }

            identifiers[1].value.as_str()
        }
        _ => return Err("Only direct column projections are supported.".to_owned()),
    };

    let source_index = table
        .schema
        .columns
        .iter()
        .position(|column| identifiers_match(&column.name, column_name))
        .ok_or_else(|| format!("Column `{column_name}` does not exist."))?;

    let mut schema = table.schema.columns[source_index].clone();
    if let Some(alias) = alias {
        schema.name = alias.to_owned();
    }

    Ok(SelectedColumn {
        source_index,
        output_name: schema.name.clone(),
        schema,
    })
}

#[cfg(any(target_arch = "wasm32", test))]
fn parse_limit_clause(limit_clause: &LimitClause) -> Result<usize, String> {
    match limit_clause {
        LimitClause::LimitOffset {
            limit: Some(limit),
            offset: None,
            limit_by,
        } if limit_by.is_empty() => parse_positive_integer(limit),
        LimitClause::LimitOffset { limit: None, .. } => Ok(usize::MAX),
        LimitClause::OffsetCommaLimit { .. } | LimitClause::LimitOffset { .. } => {
            Err("Only LIMIT <number> without OFFSET is supported.".to_owned())
        }
    }
}

#[cfg(any(target_arch = "wasm32", test))]
fn parse_positive_integer(expr: &Expr) -> Result<usize, String> {
    match expr {
        Expr::Value(value_with_span) => match &value_with_span.value {
            Value::Number(value, false) => value
                .parse::<usize>()
                .map_err(|_| "LIMIT must be a non-negative integer.".to_owned()),
            _ => Err("LIMIT must be a non-negative integer.".to_owned()),
        },
        _ => Err("LIMIT must be a non-negative integer.".to_owned()),
    }
}

#[cfg(any(target_arch = "wasm32", test))]
fn identifiers_match(left: &str, right: &str) -> bool {
    left.eq_ignore_ascii_case(right)
}

#[cfg(any(target_arch = "wasm32", test))]
fn infer_value_type(value: &str) -> InferredType {
    if is_boolean(value) {
        return InferredType::Boolean;
    }

    if value.parse::<i64>().is_ok() {
        return InferredType::Int64;
    }

    if value.parse::<f64>().is_ok_and(|number| number.is_finite()) {
        return InferredType::Float64;
    }

    InferredType::Utf8
}

#[cfg(any(target_arch = "wasm32", test))]
fn is_boolean(value: &str) -> bool {
    matches!(value.to_ascii_lowercase().as_str(), "true" | "false")
}

#[cfg(any(target_arch = "wasm32", test))]
fn merge_inferred_type(current: Option<InferredType>, next: InferredType) -> InferredType {
    match (current, next) {
        (None, next) => next,
        (Some(InferredType::Utf8), _) | (_, InferredType::Utf8) => InferredType::Utf8,
        (Some(InferredType::Float64), InferredType::Int64)
        | (Some(InferredType::Int64), InferredType::Float64)
        | (Some(InferredType::Float64), InferredType::Float64) => InferredType::Float64,
        (Some(InferredType::Int64), InferredType::Int64) => InferredType::Int64,
        (Some(InferredType::Boolean), InferredType::Boolean) => InferredType::Boolean,
        (Some(_), _) => InferredType::Utf8,
    }
}

#[cfg(any(target_arch = "wasm32", test))]
impl InferredType {
    fn as_str(self) -> &'static str {
        match self {
            InferredType::Boolean => "Boolean",
            InferredType::Int64 => "Int64",
            InferredType::Float64 => "Float64",
            InferredType::Utf8 => "Utf8",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infers_column_types_and_nullability() {
        let schema = infer_csv_schema(
            b"region,total,rate,active,notes\nAPAC,42,0.75,true,\nEMEA,,2,false,manual\n",
        )
        .expect("infer schema");

        assert_eq!(schema.row_count, 2);
        assert_eq!(schema.columns[0].name, "region");
        assert_eq!(schema.columns[0].data_type, "Utf8");
        assert!(!schema.columns[0].nullable);
        assert_eq!(schema.columns[1].data_type, "Int64");
        assert!(schema.columns[1].nullable);
        assert_eq!(schema.columns[1].null_count, 1);
        assert_eq!(schema.columns[2].data_type, "Float64");
        assert_eq!(schema.columns[3].data_type, "Boolean");
        assert_eq!(schema.columns[4].data_type, "Utf8");
        assert!(schema.columns[4].nullable);
        assert_eq!(schema.columns[4].null_count, 1);
    }

    #[test]
    fn treats_mixed_scalar_types_as_utf8() {
        let schema = infer_csv_schema(b"value\ntrue\n42\n").expect("infer mixed scalar schema");

        assert_eq!(schema.columns[0].data_type, "Utf8");
    }

    #[test]
    fn rejects_duplicate_headers() {
        let error = infer_csv_schema(b"region,region\nAPAC,EMEA\n").expect_err("duplicate header");

        assert_eq!(error, "CSV header name `region` is duplicated.");
    }

    #[test]
    fn executes_select_columns_with_limit() {
        let table = parse_csv_table(
            "sales",
            b"region,total,rate,active\nAPAC,42,0.75,true\nEMEA,7,1.5,false\n",
        )
        .expect("parse table");

        let result = execute_registered_query("SELECT region, total FROM sales LIMIT 1", &[table])
            .expect("execute query");

        assert_eq!(result.row_count, 1);
        assert_eq!(result.schema[0].name, "region");
        assert_eq!(result.schema[1].name, "total");
        assert_eq!(result.rows[0].get("region"), Some(&Some("APAC".to_owned())));
        assert_eq!(result.rows[0].get("total"), Some(&Some("42".to_owned())));
        assert_eq!(result.elapsed_ms, 0.0);
    }

    #[test]
    fn executes_wildcard_and_column_alias() {
        let table = parse_csv_table("sales", b"region,total\nAPAC,42\n").expect("parse table");

        let wildcard_result =
            execute_registered_query("SELECT * FROM sales", std::slice::from_ref(&table))
                .expect("execute wildcard query");
        let alias_result = execute_registered_query("SELECT total AS amount FROM sales", &[table])
            .expect("execute alias query");

        assert_eq!(wildcard_result.schema.len(), 2);
        assert_eq!(
            wildcard_result.rows[0].get("region"),
            Some(&Some("APAC".to_owned()))
        );
        assert_eq!(alias_result.schema[0].name, "amount");
        assert_eq!(
            alias_result.rows[0].get("amount"),
            Some(&Some("42".to_owned()))
        );
    }

    #[test]
    fn rejects_queries_for_unknown_tables() {
        let table = parse_csv_table("sales", b"region,total\nAPAC,42\n").expect("parse table");

        let error = execute_registered_query("SELECT * FROM inventory", &[table])
            .expect_err("unknown table");

        assert_eq!(error, "CSV table `inventory` is not registered.");
    }
}
