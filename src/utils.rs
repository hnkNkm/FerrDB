pub fn trim_quotes(input: &str) -> String {
    input.trim_matches(|c| c == '\'' || c == '"').trim().to_string()
}

pub fn parse_values(input: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut current = String::new();
    let mut inside_quotes = false;
    let mut quote_char = '\0';
    for c in input.chars() {
        match c {
            '"' | '\'' => {
                if inside_quotes {
                    if c == quote_char {
                        inside_quotes = false;
                    } else {
                        current.push(c);
                    }
                } else {
                    inside_quotes = true;
                    quote_char = c;
                }
            }
            ',' if !inside_quotes => {
                values.push(current.trim().to_string());
                current.clear();
            }
            _ => {
                current.push(c);
            }
        }
    }
    if !current.trim().is_empty() {
        values.push(current.trim().to_string());
    }
    values
}
