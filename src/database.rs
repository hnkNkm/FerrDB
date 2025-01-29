use serde::{Deserialize, Serialize};

use crate::table::Table;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize)]
pub struct Database {
    tables: HashMap<String, Table>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            tables: HashMap::new(),
        }
    }

    pub fn create_table(&mut self, name: &str, columns: Vec<String>) {
        if self.tables.contains_key(name) {
            println!("Error: Table '{}' already exists.", name);
            return;
        }
        self.tables.insert(name.to_string(), Table::new(columns));
        println!("Table '{}' created.", name);
    }

    pub fn insert_into(&mut self, table_name: &str, values: Vec<String>) {
        if let Some(table) = self.tables.get_mut(table_name) {
            table.insert(values);
        } else {
            println!("Error: Table '{}' does not exist.", table_name);
        }
    }

    pub fn select_all(&self, table_name: &str) {
        if let Some(table) = self.tables.get(table_name) {
            table.select_all();
        } else {
            println!("Error: Table '{}' does not exist.", table_name);
        }
    }

    pub fn save_data(&self, path: &str) {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .expect("Unable to open file for saving data");
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self).expect("Failed to save data");
    }

    pub fn load_data(&mut self, path: &str) {
        let file = File::open(path);
        if let Ok(file) = file {
            let reader = BufReader::new(file);
            *self = serde_json::from_reader(reader).expect("Failed to load data");
        }
    }
}
