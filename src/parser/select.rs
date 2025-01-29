pub fn parse_select_table(command: &str) -> Option<String> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() < 4 || parts[1] != "*" || parts[2].to_uppercase() != "FROM" {
        return None;
    }
    Some(parts[3].trim_end_matches(';').to_string())
}
