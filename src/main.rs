mod database;
mod table;
mod btree;
mod parser;
mod utils;

use database::Database;
// use parser::{parse_create_table, parse_insert_into, parse_select_table, parse_select_where};
use parser::{parse_query, Query, ParserError};
use std::io::{self, Write};

fn main() {
    let mut db = Database::new();
    let db_file_path = "db.json";

    db.load_data(db_file_path);

    println!("Welcome to FerrDB CLI. Type 'exit' or 'quit' to quit.");
    let mut input = String::new();

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
        
        match parse_query(command_line) {
            Ok(query) => {
                db.execute_query(query);
                db.save_data(&db_file_path);
            }
            Err(e) => {
                print!("Error: {:?}", e);
            }
        }
    }

    db.save_data(db_file_path);
    println!("Goodbye!");
}
