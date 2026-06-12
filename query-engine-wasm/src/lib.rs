#[cfg(any(target_arch = "wasm32", test))]
use serde::Serialize;
#[cfg(any(target_arch = "wasm32", test))]
use std::collections::HashSet;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Default)]
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct QueryEngine {
    registered_tables: Vec<RegisteredCsvSchema>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg(any(target_arch = "wasm32", test))]
struct CsvSchema {
    columns: Vec<ColumnSchema>,
    row_count: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg(target_arch = "wasm32")]
struct QueryResult {
    rows: Vec<QueryRow>,
    schema: Vec<ColumnSchema>,
    row_count: u32,
    elapsed_ms: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg(target_arch = "wasm32")]
struct QueryRow {}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg(any(target_arch = "wasm32", test))]
struct ColumnSchema {
    name: String,
    data_type: String,
    nullable: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg(target_arch = "wasm32")]
struct RegisteredCsvSchema {
    name: String,
    columns: Vec<ColumnSchema>,
    row_count: usize,
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

        let schema = infer_csv_schema(data).map_err(|error| JsValue::from_str(&error))?;
        let registered_schema = RegisteredCsvSchema {
            name: name.trim().to_owned(),
            columns: schema.columns.clone(),
            row_count: schema.row_count,
        };

        self.registered_tables.push(registered_schema.clone());

        serde_wasm_bindgen::to_value(&registered_schema)
            .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    pub fn execute(&self, sql: &str) -> Result<JsValue, JsValue> {
        if sql.trim().is_empty() {
            return Err(JsValue::from_str("SQL query is required."));
        }

        let result = QueryResult {
            rows: Vec::new(),
            schema: Vec::new(),
            row_count: 0,
            elapsed_ms: 0.0,
        };

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
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut inferred_types = vec![None; columns.len()];
    let mut row_count = 0;

    for record in reader.records() {
        let record = record.map_err(|error| format!("CSV row is invalid: {error}"))?;

        if record.len() > columns.len() {
            return Err(format!(
                "CSV row {} has more fields than the header row.",
                row_count + 1
            ));
        }

        row_count += 1;

        for (index, column) in columns.iter_mut().enumerate() {
            let Some(value) = record.get(index).map(str::trim) else {
                column.nullable = true;
                continue;
            };

            if value.is_empty() {
                column.nullable = true;
                continue;
            }

            let value_type = infer_value_type(value);
            inferred_types[index] = Some(merge_inferred_type(inferred_types[index], value_type));
        }
    }

    for (column, inferred_type) in columns.iter_mut().zip(inferred_types) {
        column.data_type = inferred_type
            .unwrap_or(InferredType::Utf8)
            .as_str()
            .to_owned();
    }

    Ok(CsvSchema { columns, row_count })
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
        assert_eq!(schema.columns[2].data_type, "Float64");
        assert_eq!(schema.columns[3].data_type, "Boolean");
        assert_eq!(schema.columns[4].data_type, "Utf8");
        assert!(schema.columns[4].nullable);
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
}
