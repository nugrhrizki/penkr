use indexmap::IndexMap;
use serde::Serialize;
use sqlx::{postgres::PgRow, Column, FromRow, Row, TypeInfo};

#[derive(Serialize)]
pub struct QueryResult(IndexMap<String, serde_json::Value>);

impl FromRow<'_, PgRow> for QueryResult {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let columns = row.columns();
        let data: IndexMap<String, serde_json::Value> = columns
            .iter()
            .map(|column| {
                let name = column.name();
                let column_type = column.type_info();
                map_pg_column(name, row, column_type.name())
            })
            .collect();
        Ok(QueryResult(data))
    }
}

fn map_pg_column(name: &str, row: &PgRow, pg_type: &str) -> (String, serde_json::Value) {
    match pg_type {
        "BOOL" => row
            .try_get::<bool, _>(name)
            .map_or((name.to_string(), serde_json::Value::Null), |value| {
                (name.to_string(), serde_json::Value::Bool(value))
            }),
        "INT2" => row
            .try_get::<i16, _>(name)
            .map_or((name.to_string(), serde_json::Value::Null), |value| {
                (name.to_string(), serde_json::Value::Number(value.into()))
            }),
        "INT4" => row
            .try_get::<i32, _>(name)
            .map_or((name.to_string(), serde_json::Value::Null), |value| {
                (name.to_string(), serde_json::Value::Number(value.into()))
            }),
        "INT8" => row
            .try_get::<i64, _>(name)
            .map_or((name.to_string(), serde_json::Value::Null), |value| {
                (name.to_string(), serde_json::Value::Number(value.into()))
            }),
        "FLOAT4" => row.try_get::<f32, _>(name).map_or(
            (name.to_string(), serde_json::Value::Null),
            |value| {
                serde_json::Number::from_f64(value.into())
                    .map_or((name.to_string(), serde_json::Value::Null), |number| {
                        (name.to_string(), serde_json::Value::Number(number))
                    })
            },
        ),
        "FLOAT8" => row.try_get::<f64, _>(name).map_or(
            (name.to_string(), serde_json::Value::Null),
            |value| {
                serde_json::Number::from_f64(value.into())
                    .map_or((name.to_string(), serde_json::Value::Null), |number| {
                        (name.to_string(), serde_json::Value::Number(number))
                    })
            },
        ),
        "TEXT" | "VARCHAR" | "NAME" => row
            .try_get::<String, _>(name)
            .map_or((name.to_string(), serde_json::Value::Null), |value| {
                (name.to_string(), serde_json::Value::String(value.into()))
            }),
        "JSON" | "JSONB" => row
            .try_get::<serde_json::Value, _>(name)
            .map_or((name.to_string(), serde_json::Value::Null), |value| {
                (name.to_string(), value)
            }),
        "TIMESTAMPTZ" => row
            .try_get::<chrono::DateTime<chrono::Utc>, _>(name)
            .map_or((name.to_string(), serde_json::Value::Null), |value| {
                (
                    name.to_string(),
                    serde_json::Value::String(value.to_rfc3339()),
                )
            }),
        "TIMESTAMP" => row.try_get::<chrono::NaiveDateTime, _>(name).map_or(
            (name.to_string(), serde_json::Value::Null),
            |value| {
                (
                    name.to_string(),
                    serde_json::Value::String(value.to_string()),
                )
            },
        ),
        "DATE" => row.try_get::<chrono::NaiveDate, _>(name).map_or(
            (name.to_string(), serde_json::Value::Null),
            |value| {
                (
                    name.to_string(),
                    serde_json::Value::String(value.to_string()),
                )
            },
        ),
        "TIME" => row.try_get::<chrono::NaiveTime, _>(name).map_or(
            (name.to_string(), serde_json::Value::Null),
            |value| {
                (
                    name.to_string(),
                    serde_json::Value::String(value.to_string()),
                )
            },
        ),
        "NULL" => (name.to_string(), serde_json::Value::Null),
        _ => {
            println!("Unknown type: {}", pg_type);
            (
                name.to_string(),
                serde_json::Value::String("not supported".into()),
            )
        }
    }
}
