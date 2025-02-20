use crate::parser::{ParserError, Query, Condition, Operator};
use crate::utils::trim_quotes;

/// Parse a SELECT query.
/// Examples:
///   "SELECT id, name FROM users;"
///   "SELECT id, name FROM users WHERE age = '30';"
pub fn parse_select(query: &str) -> Result<Query, ParserError> {
    let query = query.trim_end_matches(';').trim();
    let prefix = "SELECT";
    if !query.to_uppercase().starts_with(prefix) {
        return Err(ParserError::InvalidSyntax("Not a SELECT command".into()));
    }
    let without_select = query[prefix.len()..].trim();
    // Split into SELECT columns and the rest using " FROM "
    let parts: Vec<&str> = without_select.splitn(2, " FROM ").collect();
    if parts.len() != 2 {
        return Err(ParserError::InvalidSyntax("Missing FROM clause in SELECT".into()));
    }
    let columns_str = parts[0].trim();
    let select_columns: Vec<String> = if columns_str == "*" {
        vec!["*".to_string()]
    } else {
        columns_str.split(',')
            .map(|s| s.trim().to_string())
            .collect()
    };
    let remaining = parts[1].trim();
    // Check if there's a WHERE clause
    if remaining.to_uppercase().contains(" WHERE ") {
        let parts: Vec<&str> = remaining.splitn(2, " WHERE ").collect();
        if parts.len() != 2 {
            return Err(ParserError::InvalidSyntax("Invalid WHERE clause".into()));
        }
        let table_name = parts[0].trim().to_string();
        let condition_str = parts[1].trim();
        let cond_parts: Vec<&str> = condition_str.splitn(2, '=').collect();
        if cond_parts.len() != 2 {
            return Err(ParserError::InvalidSyntax("Invalid condition format".into()));
        }
        let cond_column = cond_parts[0].trim().to_string();
        let cond_value = trim_quotes(cond_parts[1].trim());
        let condition = Condition {
            column: cond_column,
            operator: Operator::Eq, // For simplicity, only '=' is supported here.
            value: cond_value,
        };
        Ok(Query::Select {
            table_name,
            select_columns,
            condition: Some(condition),
        })
    } else {
        let table_name = remaining.to_string();
        Ok(Query::Select {
            table_name,
            select_columns,
            condition: None,
        })
    }
}