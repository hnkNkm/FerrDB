mod database;
mod parser;
mod utils;

use database::Database;
use parser::{parse_create_table, parse_insert_into, parse_select_table};
use std::io::{self, Write};

fn main() {
    let mut db = Database::new();
    let db_file_path = "db.json";
    db.load_data(db_file_path);

    let mut input = String::new();
    println!("Welcome to SimpleRDB CLI. Type 'exit' or 'quit' to quit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input");
            continue;
        }

        let command_line = input.trim_end_matches(';').trim();
        if command_line.eq_ignore_ascii_case("exit") || command_line.eq_ignore_ascii_case("quit") {
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