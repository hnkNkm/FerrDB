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
    
    pub fn get_table(&self, name: &str) -> Option<&Table> {
        self.tables.get(name)
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

    pub fn select_all(&self, table_name: &str, selected_colums: Vec<String>) {
        if let Some(table) = self.tables.get(table_name) {
            table.select_all_with_columns(selected_colums);
        } else {
            println!("Error: Table '{}' does not exist.", table_name);
        }
    }

    /// WHERE 句による検索：指定されたテーブルの、任意カラムの値が search_value と一致する行を表示する。
    /// 主キー検索は B+Tree による高速検索で行い、それ以外は全件走査してフィルタリングする。
    pub fn select_where(&self, table_name: &str, selected_columns: &Vec<String>, condition_column: &str, search_value: &str) {
       if let Some(table) = self.get_table(table_name) {
           let results = table.select_where(condition_column, search_value);
           if results.is_empty() {
               println!("No matching row found for {} = '{}'.", condition_column, search_value);
           } else {
               println!("Columns: {:?}", selected_columns);
               for row in results {
                   println!("Row: {:?}", row);
               }
           }
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
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            *self = serde_json::from_reader(reader).expect("Failed to load data");
        }
    }
}
