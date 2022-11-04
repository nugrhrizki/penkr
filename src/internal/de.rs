use serde::Serialize;
use sqlx::{postgres::PgRow, Column, FromRow, Row, TypeInfo};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct QueryResult(HashMap<String, serde_json::Value>);

// FIXME: so slow :(
impl FromRow<'_, PgRow> for QueryResult {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let columns = row.columns();
        let mut data = String::from('{');
        for column in columns {
            match column.type_info().name() {
                "BOOL" => {
                    let value = row.try_get::<bool, _>(column.name());
                    match value {
                        Ok(value) => {
                            data.push_str(&format!("\"{}\":{},", column.name(), value));
                        }
                        Err(_) => {
                            data.push_str(&format!("\"{}\":null,", column.name()));
                        }
                    }
                }
                "INT2" => {
                    let value = row.try_get::<i16, _>(column.name());
                    match value {
                        Ok(value) => {
                            data.push_str(&format!("\"{}\":{},", column.name(), value));
                        }
                        Err(_) => {
                            data.push_str(&format!("\"{}\":null,", column.name()));
                        }
                    }
                }
                "INT4" => {
                    let value = row.try_get::<i32, _>(column.name());
                    match value {
                        Ok(value) => {
                            data.push_str(&format!("\"{}\":{},", column.name(), value));
                        }
                        Err(_) => {
                            data.push_str(&format!("\"{}\":null,", column.name()));
                        }
                    }
                }
                "INT8" => {
                    let value = row.try_get::<i64, _>(column.name());
                    match value {
                        Ok(value) => {
                            data.push_str(&format!("\"{}\":{},", column.name(), value));
                        }
                        Err(_) => {
                            data.push_str(&format!("\"{}\":null,", column.name()));
                        }
                    }
                }
                "FLOAT4" => {
                    let value = row.try_get::<f32, _>(column.name());
                    match value {
                        Ok(value) => {
                            data.push_str(&format!("\"{}\":{},", column.name(), value));
                        }
                        Err(_) => {
                            data.push_str(&format!("\"{}\":null,", column.name()));
                        }
                    }
                }
                "FLOAT8" => {
                    let value = row.try_get::<f64, _>(column.name());
                    match value {
                        Ok(value) => {
                            data.push_str(&format!("\"{}\":{},", column.name(), value));
                        }
                        Err(_) => {
                            data.push_str(&format!("\"{}\":null,", column.name()));
                        }
                    }
                }
                "TEXT" => {
                    let value = row.try_get::<String, _>(column.name());
                    match value {
                        Ok(value) => {
                            data.push_str(&format!(
                                "\"{}\":\"{}\",",
                                column.name(),
                                value.replace("\"", "\\\"")
                            ));
                        }
                        Err(_) => {
                            data.push_str(&format!("\"{}\":null,", column.name()));
                        }
                    }
                }
                "VARCHAR" => {
                    let value = row.try_get::<String, _>(column.name());
                    match value {
                        Ok(value) => {
                            data.push_str(&format!("\"{}\":\"{}\",", column.name(), value));
                        }
                        Err(_) => {
                            data.push_str(&format!("\"{}\":null,", column.name()));
                        }
                    }
                }
                "JSON" => {
                    let value = row.try_get::<serde_json::Value, _>(column.name());
                    match value {
                        Ok(value) => {
                            data.push_str(&format!(
                                "\"{}\":\"{}\",",
                                column.name(),
                                value.to_string()
                            ));
                        }
                        Err(_) => {
                            data.push_str(&format!("\"{}\":null,", column.name()));
                        }
                    }
                }
                _ => {
                    // let value = row.try_get::<serde_json::Value, _>(column.name());
                    // match value {
                    //     Ok(value) => {
                    data.push_str(&format!("\"{}\":\"{}\",", column.name(), "not supported"));
                    //     }
                    //     Err(_) => {
                    //         data.push_str(&format!("\"{}\":null,", column.name()));
                    //     }
                    // }
                }
            }
        }
        if let Some('{') = data.pop() {
            data.push('{');
        }
        data.push('}');
        let data_parsed: HashMap<String, serde_json::Value> =
            match serde_json::from_str(data.as_str()) {
                Ok(data) => data,
                Err(_) => HashMap::new(),
            };
        Ok(QueryResult(data_parsed))
    }
}
