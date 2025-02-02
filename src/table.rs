use serde::{Deserialize, Serialize};

// 修正: BTree ではなく BPlusTree をインポートする
use crate::btree::BPlusTree;

#[derive(Serialize, Deserialize)]
pub struct Table {
    columns: Vec<String>,
    // 修正: BTree<String, Vec<String>> から BPlusTree<String, Vec<String>> に変更
    data: BPlusTree<String, Vec<String>>,
}

impl Table {
    pub fn new(columns: Vec<String>) -> Self {
        Table {
            columns,
            data: BPlusTree::new(2),
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
            let (keys, values_opt) = root.get_keys_values();
            if let Some(values) = values_opt {
                for (key, value) in keys.iter().zip(values.iter()) {
                    println!("Key: {}, Values: {:?}", key, value);
                }
            } else {
                println!("No leaf data available.");
            }
        } else {
            println!("No data available.");
        }
    }
}
