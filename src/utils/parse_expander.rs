use crate::core::de::Field;
use crate::utils::common::replace_first_and_last;

pub fn parse_introspect(raw: &str) -> Vec<Field> {
    let raw = replace_first_and_last(raw, "{", "}", "[", "]");
    let raw = serde_json::from_str::<Vec<String>>(&raw).unwrap();

    let vec_column_info = raw
        .iter()
        .map(|x| {
            let column_info = x.trim_start_matches("(").trim_end_matches(")");
            let column_info: Vec<&str> = column_info.split(",").collect();
            let field_info = Field {
                name: column_info[0].to_string(),
                ordinal_position: column_info[1].parse().unwrap(),
                constraint: if column_info[2].is_empty() {
                    None
                } else {
                    let remove_quote = column_info[2]
                        .trim_start_matches("\"")
                        .trim_end_matches("\"");
                    Some(remove_quote.to_string())
                },
                char_max_len: if column_info[3].is_empty() {
                    None
                } else {
                    column_info[3].parse().ok()
                },
                numeric_precision: if column_info[4].is_empty() {
                    None
                } else {
                    column_info[4].parse().ok()
                },
                numeric_scale: if column_info[5].is_empty() {
                    None
                } else {
                    column_info[5].parse().ok()
                },
                data_type: column_info[6]
                    .trim_start_matches("\"")
                    .trim_end_matches("\"")
                    .to_string(),
                is_nullable: column_info[7] == "t",
                is_unique: column_info[8] == "t",
                is_auto_increment: column_info[9] == "t",
                default_value: if column_info[10].is_empty() {
                    None
                } else {
                    let remove_quote = column_info[10]
                        .trim_start_matches("\"")
                        .trim_end_matches("\"");
                    Some(remove_quote.to_string())
                },
            };
            field_info
        })
        .collect();

    vec_column_info
}
