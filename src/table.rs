use serde::{Deserialize, Serialize};

use crate::btree::BTree;

#[derive(Serialize, Deserialize)]
pub struct Table {
    columns: Vec<String>,
    data: BTree<String, Vec<String>>,
}

impl Table {
    pub fn new(columns: Vec<String>) -> Self {
        Table {
            columns,
            data: BTree::new(2),
        }
    }

    pub fn insert(&mut self, values: Vec<String>) {
        if values.len() != self.columns.len() {
            println!(
                "Error: Column count mismatch. Expected {}, got {}.",
                self.columns.len(),
                values.len()
            );
            return;
        }

        let key = values[0].clone();
        if self.data.search(&key).is_some() {
            println!("Error: Duplicate key '{}'.", key);
            return;
        }
        self.data.insert(key, values);
    }

    pub fn select_all(&self) {
        println!("Columns: {:?}", self.columns);
        if let Some(root) = self.data.get_root() {
            let (keys, values) = root.get_keys_values();
            for (key, value) in keys.iter().zip(values.iter()) {
                println!("Key: {}, Values: {:?}", key, value);
            }
        } else {
            println!("No data available.");
        }
    }
}
