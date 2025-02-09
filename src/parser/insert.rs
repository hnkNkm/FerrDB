use crate::utils::trim_quotes;

pub fn parse_insert_into(command: &str) -> Option<(String, Vec<String>)> {
    let command = command.trim_end_matches(';').trim();
    let parts: Vec<&str> = command.splitn(2, "VALUES").collect();
    if parts.len() != 2 {
        return None;
    }
    let tokens: Vec<&str> = parts[0].split_whitespace().collect();
    if tokens.len() < 3 {
        return None;
    }
    let table_name = tokens[2].to_string();
    let values_str = parts[1].trim().trim_start_matches('(').trim_end_matches(')').trim();
    let values: Vec<String> = values_str
        .split(',')
        .map(|s| trim_quotes(s.trim()))
        .collect();
    Some((table_name, values))
}
