use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub tables: HashMap<String, Table>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            tables: HashMap::new(),
        }
    }

    pub fn load_data(&mut self, path: &str) {
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

    pub fn save_data(&self, path: &str) {
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

    pub fn create_table(&mut self, name: &str, columns: Vec<String>) {
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

    pub fn insert_into(&mut self, table_name: &str, values: Vec<String>, db_path: &str) {
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

    pub fn select_all(&self, table_name: &str) {
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
