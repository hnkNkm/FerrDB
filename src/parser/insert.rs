use crate::parser::{ParserError, Query};

/// Parse an INSERT query.
/// Example: "INSERT INTO users VALUES (1, 'John', 30);"
pub fn parse_insert(query: &str) -> Result<Query, ParserError> {
    let query = query.trim_end_matches(';').trim();
    let prefix = "INSERT INTO";
    if !query.to_uppercase().starts_with(prefix) {
        return Err(ParserError::InvalidSyntax("Not an INSERT command".into()));
    }
    let after_prefix = query[prefix.len()..].trim();
    let parts: Vec<&str> = after_prefix.splitn(2, "VALUES").collect();
    if parts.len() != 2 {
        return Err(ParserError::InvalidSyntax("Missing VALUES clause".into()));
    }
    let table_name = parts[0].trim().to_string();
    let values_str = parts[1]
        .trim()
        .trim_start_matches('(')
        .trim_end_matches(')')
        .trim();
    let values = values_str
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    Ok(Query::Insert { table_name, values })
}
