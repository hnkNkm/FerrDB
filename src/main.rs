use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Write};

#[derive(Debug, Serialize, Deserialize)]
struct Table {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Database {
    tables: HashMap<String, Table>,
}

impl Database {
    fn new() -> Self {
        Database {
            tables: HashMap::new(),
        }
    }

    fn load_data(&mut self, path: &str) {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                return;
            }
        };
        let reader = BufReader::new(file);
        match serde_json::from_reader(reader) {
            Ok(db_data) => {
                *self = db_data;
                println!("Data loaded from {}", path);
            }
            Err(e) => {
                println!("Failed to parse {}: {}", path, e);
            }
        }
    }

    fn save_data(&self, path: &str) {
        let file = match OpenOptions::new().write(true).create(true).truncate(true).open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("Failed to open {} for writing: {}", path, e);
                return;
            }
        };
        let writer = BufWriter::new(file);
        if let Err(e) = serde_json::to_writer_pretty(writer, &self) {
            println!("Failed to write {}: {}", path, e);
        } else {
            println!("Data saved to {}", path);
        }
    }

    fn create_table(&mut self, name: &str, columns: Vec<String>) {
        if self.tables.contains_key(name) {
            println!("Error: Table '{}' already exists.", name);
            return;
        }
        self.tables.insert(
            name.to_string(),
            Table {
                columns,
                rows: Vec::new(),
            },
        );
        println!("Table '{}' created.", name);
    }

    fn insert_into(&mut self, table_name: &str, values: Vec<String>, db_path: &str) {
        if let Some(table) = self.tables.get_mut(table_name) {
            if values.len() != table.columns.len() {
                println!(
                    "Error: Column count mismatch. Expected {}, got {}.",
                    table.columns.len(),
                    values.len()
                );
                return;
            }
            table.rows.push(values);
            println!("Data inserted into '{}'.", table_name);
            self.save_data(db_path);
        } else {
            println!("Error: Table '{}' does not exist.", table_name);
        }
    }

    fn select_all(&self, table_name: &str) {
        if let Some(table) = self.tables.get(table_name) {
            println!("Table '{}':", table_name);
            println!("{:?}", table.columns);
            for row in &table.rows {
                println!("{:?}", row);
            }
        } else {
            println!("Error: Table '{}' does not exist.", table_name);
        }
    }
}

fn parse_create_table(command: &str) -> Option<(String, Vec<String>)> {
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

fn parse_insert_into(command: &str) -> Option<(String, Vec<String>)> {
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
    Some((table_name.to_string(), parse_values(inner)))
}

fn parse_select_table(command: &str) -> Option<(String, String)> {
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

fn parse_values(input: &str) -> Vec<String> {
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

fn main() {
    let mut db = Database::new();
    let db_file_path = "db.json";
    db.load_data(db_file_path);
    let mut input = String::new();
    println!("Welcome to SimpleRDB CLI. Type 'exit' to quit.");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input");
            continue;
        }
        let command_line = input.trim_end_matches(';').trim();
        if command_line.eq_ignore_ascii_case("exit") {
            break;
        }
        if command_line.starts_with("CREATE TABLE") {
            if let Some((table_name, columns)) = parse_create_table(command_line) {
                db.create_table(&table_name, columns);
            } else {
                println!("Error: Invalid CREATE TABLE syntax.");
            }
        } else if command_line.starts_with("INSERT INTO") {
            if let Some((table_name, values)) = parse_insert_into(command_line) {
                db.insert_into(&table_name, values, db_file_path);
            } else {
                println!("Error: Invalid INSERT INTO syntax.");
            }
        } else if command_line.starts_with("SELECT") {
            if let Some((cols, table_name)) = parse_select_table(command_line) {
                if cols == "*" {
                    db.select_all(&table_name);
                } else {
                    println!("Error: Specific column selection is not yet implemented.");
                }
            } else {
                println!("Error: Invalid SELECT syntax.");
            }
        } else if command_line.is_empty() {
        } else {
            println!("Unknown command: {}", command_line);
        }
    }
    println!("Goodbye!");
}
