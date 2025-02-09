use serde::{Deserialize, Serialize};
use crate::btree::BPlusTree;

#[derive(Serialize, Deserialize)]
pub struct Table {
    pub columns: Vec<String>,
    pub data: BPlusTree<String, Vec<String>>,
}

impl Table {
    pub fn new(columns: Vec<String>) -> Self {
        Table {
            columns,
            data: BPlusTree::new(2), // プライマリキーは最初のカラム（例: id）を想定
        }
    }
    
    /// B+Tree のルートを取得するゲッター
    pub fn get_root(&self) -> Option<&crate::btree::BPlusTreeNode<String, Vec<String>>> {
        self.data.get_root()
    }

    /// 全ての行を取得する (B+Tree の葉ノードを連結リストとして辿る)
    pub fn get_all_rows(&self) -> Vec<Vec<String>> {
        if let Some(root) = self.get_root() {
            if root.is_leaf() {
                return root.values().unwrap().clone();
            } else {
                let mut rows = Vec::new();
                let mut node = root;
                while !node.is_leaf() {
                    node = node.children().unwrap().first().unwrap();
                }
                loop {
                    for row in node.values().unwrap().iter() {
                        rows.push(row.clone());
                    }
                    if let Some(next) = node.next() {
                        node = next;
                    } else {
                        break;
                    }
                }
                return rows;
            }
        }
        Vec::new()
    }

    /// データ行を挿入する。最初のカラムの値をプライマリキーとして B+Tree に登録する。
    pub fn insert(&mut self, values: Vec<String>) {
        let key = values.get(0).cloned().unwrap_or_default();
        self.data.insert(key, values);
    }

    /// 全件検索して結果を表示する
    pub fn select_all(&self) {
        println!("Columns: {:?}", self.columns);
        let rows = self.get_all_rows();
        for row in rows {
            println!("Row: {:?}", row);
        }
    }

    /// WHERE 句検索：指定カラムの値が search_value と一致する行をフィルタリングして返す
    /// プライマリキーの場合は B+Tree の search を使い、その他は全件走査してフィルタリングする
    pub fn select_where(&self, column_name: &str, search_value: &str) -> Vec<Vec<String>> {
        // 主キー検索：最初のカラムの場合
        if self.columns.get(0).map(|s| s == column_name).unwrap_or(false) {
            if let Some(row) = self.data.search(&search_value.to_string()) {
                return vec![row.clone()];
            }
            return Vec::new();
        }
        // それ以外は全件走査してフィルタリング
        let all_rows = self.get_all_rows();
        let column_index = self.columns.iter().position(|col| col == column_name);
        if let Some(index) = column_index {
            all_rows
                .into_iter()
                .filter(|row| row.get(index) == Some(&search_value.to_string()))
                .collect()
        } else {
            println!("Error: Column '{}' does not exist.", column_name);
            Vec::new()
        }
    }
}
