use crate::utils::trim_quotes;

pub fn parse_select_table(command: &str) -> Option<String> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() < 4 || parts[1] != "*" || parts[2].to_uppercase() != "FROM" {
        return None;
    }
    Some(parts[3].trim_end_matches(';').to_string())
}

/// 例: "SELECT * FROM users WHERE age = '30';" → ("users", "age", "30")
pub fn parse_select_where(command: &str) -> Option<(String, String, String)> {
    let command = command.trim_end_matches(';').trim();
    let parts: Vec<&str> = command.splitn(2, "WHERE").collect();
    if parts.len() != 2 {
        return None;
    }
    let tokens: Vec<&str> = parts[0].trim().split_whitespace().collect();
    if tokens.len() < 4 {
        return None;
    }
    let table_name = tokens[3].to_string();
    let condition = parts[1].trim();
    let cond_parts: Vec<&str> = condition.split('=').map(|s| s.trim()).collect();
    if cond_parts.len() != 2 {
        return None;
    }
    let column = cond_parts[0].to_string();
    let value = trim_quotes(cond_parts[1]);
    Some((table_name, column, value))
}
