use crate::parser::{ParserError, Query};
/// Parse a CREATE TABLE query.
/// Example: "CREATE TABLE users (id, name, age);"
pub fn parse_create_table(query: &str) -> Result<Query, ParserError> {
    let query = query.trim_end_matches(';').trim();
    let prefix = "CREATE TABLE";
    if !query.to_uppercase().starts_with(prefix) {
        return Err(ParserError::InvalidSyntax("Not a CREATE TABLE command".into()));
    }
    let after_prefix = query[prefix.len()..].trim();
    let parts: Vec<&str> = after_prefix.splitn(2, '(').collect();
    if parts.len() != 2 {
        return Err(ParserError::InvalidSyntax("Missing column definitions".into()));
    }
    let table_name = parts[0].trim().to_string();
    let columns_str = parts[1].trim_end_matches(')').trim();
    let columns = columns_str.split(',')
        .map(|s| s.trim().to_string())
        .collect();
    Ok(Query::CreateTable { table_name, columns })
}
