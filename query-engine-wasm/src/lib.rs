#[cfg(any(target_arch = "wasm32", test))]
use serde::Serialize;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Default)]
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct QueryEngine {
    registered_tables: Vec<String>,
}

#[derive(Serialize)]
#[cfg(target_arch = "wasm32")]
struct QueryResult {
    rows: Vec<QueryRow>,
    schema: Vec<ColumnSchema>,
    row_count: u32,
    elapsed_ms: f64,
}

#[derive(Serialize)]
#[cfg(target_arch = "wasm32")]
struct QueryRow {}

#[derive(Serialize)]
#[cfg(any(target_arch = "wasm32", test))]
struct ColumnSchema {
    name: String,
    data_type: String,
    nullable: bool,
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

        self.registered_tables.push(name.to_owned());
        infer_schema(data)
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

    let columns = parse_header_columns(data).map_err(|error| JsValue::from_str(error))?;

    serde_wasm_bindgen::to_value(&columns).map_err(|error| JsValue::from_str(&error.to_string()))
}

#[cfg(any(target_arch = "wasm32", test))]
fn parse_header_columns(data: &[u8]) -> Result<Vec<ColumnSchema>, &'static str> {
    let header = data
        .split(|byte| *byte == b'\n')
        .next()
        .ok_or("CSV header row is missing.")?;

    let header = std::str::from_utf8(header)
        .map_err(|_| "CSV data must be valid UTF-8 for the bootstrap parser.")?;

    Ok(header
        .trim_end_matches('\r')
        .split(',')
        .filter(|name| !name.trim().is_empty())
        .map(|name| ColumnSchema {
            name: name.trim().to_owned(),
            data_type: "Utf8".to_owned(),
            nullable: true,
        })
        .collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_header_columns() {
        let columns = parse_header_columns(b" region,total \r\nAPAC,42\n").expect("parse header");

        assert_eq!(columns[0].name, "region");
        assert_eq!(columns[1].name, "total");
        assert_eq!(columns[0].data_type, "Utf8");
    }
}
