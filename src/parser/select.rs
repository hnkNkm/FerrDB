pub fn parse_select_table(command: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() < 4 {
        return None;
    }
    if parts[1] != "*" || parts[2].to_uppercase() != "FROM" {
        return None;
    }
    let table_name = parts[3].trim_end_matches(';').trim().to_string();
    Some(("*".to_string(), table_name))
}
