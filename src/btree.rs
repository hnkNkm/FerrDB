use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BPlusTreeNode<K, V> {
    // 内部ノードでは検索用のキーを保持し、
    // 葉ノードではすべてのキーと対応する値を保持します。
    keys: Vec<K>,
    // 葉ノードにのみ存在する値配列
    values: Option<Vec<V>>,
    // 内部ノードにのみ存在する子ノード群
    children: Option<Vec<Box<BPlusTreeNode<K, V>>>>,
    // ノードが葉かどうかのフラグ
    is_leaf: bool,
    // 葉ノード間のリンク（葉の場合のみ利用）
    next: Option<Box<BPlusTreeNode<K, V>>>,
}

impl<K: Ord + Clone, V: Clone> BPlusTreeNode<K, V> {
    /// 新しいノードを作成する。`is_leaf` が true の場合は葉ノードとなる。
    pub fn new(is_leaf: bool) -> Self {
        BPlusTreeNode {
            keys: Vec::new(),
            values: if is_leaf { Some(Vec::new()) } else { None },
            children: if is_leaf { None } else { Some(Vec::new()) },
            is_leaf,
            next: None,
        }
    }

    /// ノード内のキーと（葉の場合の）値のスライスを返す。
    pub fn get_keys_values(&self) -> (&[K], Option<&[V]>) {
        (&self.keys, self.values.as_ref().map(|v| &v[..]))
    }

    /// 指定キーを検索する。  
    /// 内部ノードの場合、キーが内部に存在しても右側の子ノードに降下します。
    pub fn search(&self, key: &K) -> Option<&V> {
        if self.is_leaf {
            // 葉の場合：キーと一致する位置を返す
            let pos = self.keys.iter().position(|k| k == key)?;
            self.values.as_ref()?.get(pos)
        } else {
            // 内部ノードの場合：子ノードのインデックスは「key 以下のキーの数」
            let pos = self.keys.iter().take_while(|k| *k <= key).count();
            self.children.as_ref()?.get(pos)?.search(key)
        }
    }

    /// ノードが満杯でない前提で key, value を挿入する。  
    /// 重複キーの場合は挿入を無視します。
    pub fn insert_non_full(&mut self, key: K, value: V, degree: usize) {
        if self.is_leaf {
            if self.keys.iter().any(|k| k == &key) {
                return; // 既に存在するキーは挿入しない
            }
            let pos = self.keys.iter().take_while(|k| *k <= &key).count();
            self.keys.insert(pos, key);
            self.values.as_mut().unwrap().insert(pos, value);
        } else {
            // 内部ノードの場合：子インデックスは「key 以下のキーの数」
            let mut pos = self.keys.iter().take_while(|k| *k <= &key).count();
            if self.children.as_ref().unwrap()[pos].keys.len() == 2 * degree - 1 {
                self.split_child(pos, degree);
                // 分割後、再度子インデックスを計算
                pos = self.keys.iter().take_while(|k| *k <= &key).count();
            }
            self.children.as_mut().unwrap()[pos].insert_non_full(key, value, degree);
        }
    }

    /// 子ノード children[index] が満杯の場合、その子ノードを分割して親にキーを追加する。
    /// 葉ノードの場合は、右側ノードの最小キーを親にコピーします。
    /// 内部ノードの場合は、中央値を取り出して親にコピーし、左側ノードには中央値を残しません。
    pub fn split_child(&mut self, index: usize, degree: usize) {
        let child = self.children.as_mut().unwrap()[index].as_mut();
        if child.is_leaf {
            let split_index = degree;
            let mut new_leaf = BPlusTreeNode::new(true);
            new_leaf.keys = child.keys.split_off(split_index);
            if let Some(ref mut vals) = child.values {
                new_leaf.values = Some(vals.split_off(split_index));
            }
            // 連結リストの更新
            new_leaf.next = child.next.take();
            child.next = Some(Box::new(new_leaf.clone()));
            // 親には、new_leaf の最小キーをコピー
            self.keys.insert(index, new_leaf.keys[0].clone());
            self.children.as_mut().unwrap().insert(index + 1, Box::new(new_leaf));
        } else {
            // 内部ノードの分割:
            // child.keys.len() == 2*degree - 1, child.children.len() == 2*degree
            // 左側ノードに child.keys[0..degree-1] と child.children[0..degree] を残し、
            // 中央キー child.keys[degree-1] を親にコピーし、右側ノードに残りを移す。
            let split_index = degree - 1;
            let mut new_node = BPlusTreeNode::new(false);
            new_node.keys = child.keys.split_off(split_index + 1);
            // 取り出す中央値
            let median = child.keys.pop().unwrap();
            if let Some(ref mut child_list) = child.children {
                new_node.children = Some(child_list.split_off(degree));
            }
            self.keys.insert(index, median);
            self.children.as_mut().unwrap().insert(index + 1, Box::new(new_node));
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BPlusTree<K, V> {
    root: Option<Box<BPlusTreeNode<K, V>>>,
    degree: usize,
}

impl<K: Ord + Clone, V: Clone> BPlusTree<K, V> {
    /// 新しい B+Tree を作成する。`degree` は内部ノードの子数が degree+1 となる最低キー数です。
    pub fn new(degree: usize) -> Self {
        BPlusTree { root: None, degree }
    }

    pub fn get_root(&self) -> Option<&BPlusTreeNode<K, V>> {
        self.root.as_deref()
    }

    /// キーを検索する。
    pub fn search(&self, key: &K) -> Option<&V> {
        self.root.as_ref()?.search(key)
    }

    /// キーと値を挿入する。重複キーの場合は最初の値が保持されます。
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
            new_root.children = Some(vec![std::mem::replace(root, Box::new(BPlusTreeNode::new(false)))]);
            new_root.split_child(0, self.degree);
            new_root.insert_non_full(key, value, self.degree);
            self.root = Some(Box::new(new_root));
        } else {
            self.root.as_mut().unwrap().insert_non_full(key, value, self.degree);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};
    use rand::seq::SliceRandom;

    #[test]
    fn test_insert_and_search() {
        let mut bptree = BPlusTree::new(2);
        bptree.insert("key1".to_string(), vec!["value1".to_string()]);
        bptree.insert("key2".to_string(), vec!["value2".to_string()]);
        bptree.insert("key3".to_string(), vec!["value3".to_string()]);

        assert_eq!(
            bptree.search(&"key1".to_string()),
            Some(&vec!["value1".to_string()])
        );
        assert_eq!(
            bptree.search(&"key2".to_string()),
            Some(&vec!["value2".to_string()])
        );
        assert_eq!(
            bptree.search(&"key3".to_string()),
            Some(&vec!["value3".to_string()])
        );
        assert_eq!(bptree.search(&"key4".to_string()), None);
    }

    #[test]
    fn test_insert_with_duplicates() {
        let mut bptree = BPlusTree::new(2);
        bptree.insert("key1".to_string(), vec!["value1".to_string()]);
        bptree.insert("key1".to_string(), vec!["value2".to_string()]);
        assert_eq!(
            bptree.search(&"key1".to_string()),
            Some(&vec!["value1".to_string()])
        );
    }

    #[test]
    fn test_split_child_leaf() {
        let mut bptree = BPlusTree::new(2);
        bptree.insert("key1".to_string(), vec!["value1".to_string()]);
        bptree.insert("key2".to_string(), vec!["value2".to_string()]);
        bptree.insert("key3".to_string(), vec!["value3".to_string()]);
        bptree.insert("key4".to_string(), vec!["value4".to_string()]);

        if let Some(root) = bptree.get_root() {
            if root.is_leaf {
                panic!("ルートが葉ノードのままでは分割が行われていません。");
            }
            assert!(!root.keys.is_empty());
        } else {
            panic!("ルートは None ではならない");
        }
    }

    #[test]
    fn test_get_keys_values() {
        let mut bptree = BPlusTree::new(2);
        bptree.insert("key1".to_string(), vec!["value1".to_string()]);
        bptree.insert("key2".to_string(), vec!["value2".to_string()]);

        if let Some(root) = bptree.get_root() {
            if !root.is_leaf {
                return;
            }
            let (keys, values) = root.get_keys_values();
            assert_eq!(keys, &["key1", "key2"]);
            let vals = values.unwrap();
            assert_eq!(vals[0], vec!["value1".to_string()]);
            assert_eq!(vals[1], vec!["value2".to_string()]);
        } else {
            panic!("ルートは None ではならない");
        }
    }

    #[test]
    fn test_bplus_tree_node_search() {
        let mut node = BPlusTreeNode::new(true);
        node.keys.push("key1".to_string());
        node.values.as_mut().unwrap().push("value1".to_string());

        assert_eq!(node.search(&"key1".to_string()), Some(&"value1".to_string()));
        assert_eq!(node.search(&"key2".to_string()), None);
    }

    // ── 以下、大きなツリーを検証するテストケース ──

    /// 葉ノードの連結リストをたどり、全てのキーを収集するヘルパー関数
    fn collect_leaf_keys<K: Ord + Clone + std::fmt::Debug, V>(root: &BPlusTreeNode<K, V>) -> Vec<K> {
        let mut node = root;
        while !node.is_leaf {
            node = node.children.as_ref().unwrap().first().unwrap().as_ref();
        }
        let mut keys = Vec::new();
        loop {
            keys.extend(node.keys.clone());
            if let Some(ref next) = node.next {
                node = next;
            } else {
                break;
            }
        }
        keys
    }

    #[test]
    fn test_large_tree() {
        let mut bptree = BPlusTree::new(4);
        let total = 1000;

        for i in 0..total {
            let key = format!("{:04}", i);
            let value = vec![format!("value{}", i)];
            bptree.insert(key, value);
        }

        for i in 0..total {
            let key = format!("{:04}", i);
            let expected = vec![format!("value{}", i)];
            assert_eq!(bptree.search(&key), Some(&expected), "Failed for key: {}", key);
        }
        assert_eq!(bptree.search(&"9999".to_string()), None);

        if let Some(root) = bptree.get_root() {
            let leaf_keys = collect_leaf_keys(root);
            let mut sorted_keys = leaf_keys.clone();
            sorted_keys.sort();
            assert_eq!(leaf_keys, sorted_keys, "Leaf keys are not in sorted order");
        } else {
            panic!("Root should not be None");
        }
    }

    #[test]
    fn test_large_tree_random_insertions() {
        let mut bptree = BPlusTree::new(4);
        let total = 100000;
        let mut keys: Vec<String> = (0..total).map(|i| format!("{:04}", i)).collect();

        let mut rng = StdRng::seed_from_u64(42);
        keys.shuffle(&mut rng);

        for (i, key) in keys.iter().enumerate() {
            let value = vec![format!("value{}", i)];
            bptree.insert(key.clone(), value);
        }

        for i in 0..total {
            let key = format!("{:04}", i);
            assert!(bptree.search(&key).is_some(), "Missing key: {}", key);
        }
    }
}
