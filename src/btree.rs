use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BPlusTreeNode<K, V> {
    keys: Vec<K>,
    // 葉ノードの場合のみ、キーに対応する値のベクタ
    values: Option<Vec<V>>,
    // 内部ノードの場合のみ、子ノードへの参照（子の数は keys.len() + 1 になる）
    children: Option<Vec<Box<BPlusTreeNode<K, V>>>>,
    is_leaf: bool,
    // 葉ノード同士の連結（範囲検索高速化用）
    next: Option<Box<BPlusTreeNode<K, V>>>,
}

impl<K: Ord + Clone, V: Clone> BPlusTreeNode<K, V> {
    /// 新しいノードを作成する。is_leaf が true なら値用、false なら子ノード用のベクタを初期化する。
    pub fn new(is_leaf: bool) -> Self {
        BPlusTreeNode {
            keys: Vec::new(),
            values: if is_leaf { Some(Vec::new()) } else { None },
            children: if is_leaf { None } else { Some(Vec::new()) },
            is_leaf,
            next: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.is_leaf
    }

    pub fn values(&self) -> Option<&Vec<V>> {
        self.values.as_ref()
    }

    pub fn children(&self) -> Option<&Vec<Box<BPlusTreeNode<K, V>>>> {
        self.children.as_ref()
    }

    pub fn next(&self) -> Option<&Box<BPlusTreeNode<K, V>>> {
        self.next.as_ref()
    }

    /// 検索処理  
    /// - 葉ノードでは binary_search() でキーの位置を求め、その位置の値を返す。  
    /// - 内部ノードでは、binary_search() の結果に応じて適切な子ノードへ降りる。
    pub fn search(&self, key: &K) -> Option<&V> {
        if self.is_leaf {
            self.keys.binary_search(key).ok().and_then(|i| self.values.as_ref()?.get(i))
        } else {
            match self.keys.binary_search(key) {
                Ok(i) => self.children.as_ref()?.get(i + 1)?.search(key),
                Err(i) => self.children.as_ref()?.get(i)?.search(key),
            }
        }
    }

    /// 非満杯ノードへの挿入  
    /// - 葉ノードの場合：binary_search() により挿入位置を決定し、重複キーは無視する。  
    /// - 内部ノードの場合：対象の子ノードが満杯なら先に分割し、再度挿入先を決定して再帰的に挿入する。
    pub fn insert_non_full(&mut self, key: K, value: V, t: usize) {
        if self.is_leaf {
            match self.keys.binary_search(&key) {
                Ok(_) => return, // 重複キーは挿入しない
                Err(pos) => {
                    self.keys.insert(pos, key);
                    self.values.as_mut().unwrap().insert(pos, value);
                }
            }
        } else {
            let mut idx = match self.keys.binary_search(&key) {
                Ok(i) => i + 1,
                Err(i) => i,
            };
            if self.children.as_ref().unwrap()[idx].keys.len() == 2 * t - 1 {
                self.split_child(idx, t);
                // 分割後、再度正しい子ノードのインデックスを求める
                idx = match self.keys.binary_search(&key) {
                    Ok(i) => i + 1,
                    Err(i) => i,
                };
            }
            self.children.as_mut().unwrap()[idx].insert_non_full(key, value, t);
        }
    }

    /// 子ノード分割処理  
    /// 【葉ノードの場合】  
    /// - 満杯の葉ノード（2t-1 個のキー）を、左側に t 個、右側に (t-1) 個に分割する。  
    /// - 右側新しい葉の先頭キーを親にコピーし、葉連結（next）を更新する。  
    /// 【内部ノードの場合】  
    /// - 満杯の内部ノード（2t-1 個のキー）を、左側に t-1 個、右側に t 個とし、  
    ///   左側の最後のキーを親に昇格させる。
    pub fn split_child(&mut self, index: usize, t: usize) {
        let child = self.children.as_mut().unwrap()[index].as_mut();
        if child.is_leaf {
            let split_index = t;
            let mut new_leaf = BPlusTreeNode::new(true);
            new_leaf.keys = child.keys.split_off(split_index);
            if let Some(ref mut vals) = child.values {
                new_leaf.values = Some(vals.split_off(split_index));
            }
            new_leaf.next = child.next.take();
            child.next = Some(Box::new(new_leaf.clone()));
            self.keys.insert(index, new_leaf.keys[0].clone());
            self.children.as_mut().unwrap().insert(index + 1, Box::new(new_leaf));
        } else {
            let split_index = t - 1;
            let mut new_node = BPlusTreeNode::new(false);
            new_node.keys = child.keys.split_off(split_index + 1);
            let median = child.keys.pop().unwrap();
            if let Some(ref mut child_list) = child.children {
                new_node.children = Some(child_list.split_off(t));
            }
            self.keys.insert(index, median);
            self.children.as_mut().unwrap().insert(index + 1, Box::new(new_node));
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BPlusTree<K, V> {
    root: Option<Box<BPlusTreeNode<K, V>>>,
    degree: usize,
}

impl<K: Ord + Clone, V: Clone> BPlusTree<K, V> {
    pub fn new(t: usize) -> Self {
        BPlusTree { root: None, degree: t }
    }

    pub fn get_root(&self) -> Option<&BPlusTreeNode<K, V>> {
        self.root.as_deref()
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.root.is_none() {
            let mut root = BPlusTreeNode::new(true);
            root.keys.push(key);
            root.values.as_mut().unwrap().push(value);
            self.root = Some(Box::new(root));
            return;
        }
        let root = self.root.as_mut().unwrap();
        if root.keys.len() == 2 * self.degree - 1 {
            let mut new_root = BPlusTreeNode::new(false);
            new_root.children = Some(vec![std::mem::replace(root, Box::new(BPlusTreeNode::new(true)))]);
            new_root.split_child(0, self.degree);
            new_root.insert_non_full(key, value, self.degree);
            self.root = Some(Box::new(new_root));
        } else {
            root.insert_non_full(key, value, self.degree);
        }
    }

    pub fn search(&self, key: &K) -> Option<&V> {
        (**self.root.as_ref()?).search(key)
    }
}

impl<K: Ord + Clone, V: Clone> Default for BPlusTree<K, V> {
    fn default() -> Self {
        Self::new(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    // 再帰的に葉ノードを走査してキーを収集する
    fn collect_leaf_keys<K: Ord + Clone, V: Clone>(node: &BPlusTreeNode<K, V>, keys: &mut Vec<K>) {
        if node.is_leaf() {
            keys.extend_from_slice(&node.keys);
        } else if let Some(children) = node.children() {
            for child in children {
                collect_leaf_keys(child, keys);
            }
        }
    }

    #[test]
    fn test_single_insert_search() {
        let mut tree = BPlusTree::new(2);
        tree.insert(10, "A".to_string());
        assert_eq!(tree.search(&10), Some(&"A".to_string()));
        assert_eq!(tree.search(&20), None);
    }

    #[test]
    fn test_multiple_inserts_search() {
        let mut tree = BPlusTree::new(2);
        let pairs = vec![(20, "B"), (10, "A"), (30, "C"), (40, "D"), (50, "E")];
        for (k, v) in pairs {
            tree.insert(k, v.to_string());
        }
        assert_eq!(tree.search(&10), Some(&"A".to_string()));
        assert_eq!(tree.search(&20), Some(&"B".to_string()));
        assert_eq!(tree.search(&30), Some(&"C".to_string()));
        assert_eq!(tree.search(&40), Some(&"D".to_string()));
        assert_eq!(tree.search(&50), Some(&"E".to_string()));
        assert_eq!(tree.search(&60), None);
    }

    #[test]
    fn test_tree_balance() {
        let mut tree = BPlusTree::new(2);
        for i in 1..=2000000 {
            tree.insert(i, i * 10);
        }
        let mut collected_keys = Vec::new();
        if let Some(root) = tree.get_root() {
            collect_leaf_keys(root, &mut collected_keys);
        }
        let mut sorted_keys = collected_keys.clone();
        sorted_keys.sort();
        assert_eq!(collected_keys, sorted_keys);
        assert_eq!(collected_keys, (1..=2000000).collect::<Vec<_>>());
    }

    #[test]
    fn test_duplicate_insertion() {
        let mut tree = BPlusTree::new(2);
        tree.insert(10, "A".to_string());
        // 重複キー挿入は無視される
        tree.insert(10, "B".to_string());
        assert_eq!(tree.search(&10), Some(&"A".to_string()));
    }
    
    #[test]
    fn test_random_insert_search() {
        let mut tree = BPlusTree::new(2);
        let mut rng = rand::thread_rng();
        let num_keys = 2_000_000;
        let mut keys: Vec<i32> = (0..num_keys)
            .map(|_| rng.gen_range(1..=3_000_000))
            .collect();
        keys.sort_unstable();
        keys.dedup();
        for &k in &keys {
            tree.insert(k, k * 10);
        }
        // 各キーに対して正しい値が返るかチェック
        for &k in &keys {
            assert_eq!(tree.search(&k), Some(&(k * 10)));
        }
    }
}
