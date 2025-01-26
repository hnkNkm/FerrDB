pub fn parse_insert_into(command: &str) -> Option<(String, Vec<String>)> {
    let parts: Vec<&str> = command.splitn(4, ' ').collect();
    if parts.len() != 4 {
        return None;
    }
    let table_name = parts[2].trim();
    let values_part = parts[3].trim();

    if !values_part.starts_with("VALUES") {
        return None;
    }
    let after_values = values_part.trim_start_matches("VALUES").trim();
    let after_values = after_values.trim_end_matches(';').trim();

    if !after_values.starts_with('(') || !after_values.ends_with(')') {
        return None;
    }
    let inner = &after_values[1..after_values.len() - 1].trim();
    Some((table_name.to_string(), crate::utils::parse_values(inner)))
}
