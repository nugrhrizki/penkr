use crate::models::schema::Field;

pub fn table_fields(fields: &Vec<Field>) -> String {
    let mut field_str = String::new();
    for field in fields {
        field_str = format!("{}, {} {}", field_str, field.name, field.r#type);
    }
    field_str
}
