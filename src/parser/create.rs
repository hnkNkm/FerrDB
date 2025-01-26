pub fn parse_create_table(command: &str) -> Option<(String, Vec<String>)> {
    let parts: Vec<&str> = command.splitn(3, ' ').collect();
    if parts.len() != 3 {
        return None;
    }
    let remainder = parts[2].trim();
    let mut split_iter = remainder.splitn(2, '(');
    let table_name_part = split_iter.next()?.trim_end_matches(')').trim();
    let columns_part_raw = split_iter.next()?.trim();
    let columns_part = columns_part_raw.trim_end_matches(|c| c == ')' || c == ';').trim();
    let table_name = table_name_part.trim_end_matches(';').trim();
    if table_name.is_empty() || columns_part.is_empty() {
        return None;
    }
    let columns: Vec<String> = columns_part
        .split(',')
        .map(|col| col.trim().to_string())
        .collect();
    Some((table_name.to_string(), columns))
}
