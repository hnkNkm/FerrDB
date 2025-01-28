use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BTreeNode<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
    children: Vec<Box<BTreeNode<K, V>>>,
    is_leaf: bool,
}

impl<K: Ord + Clone, V: Clone> BTreeNode<K, V> {
    pub fn new(is_leaf: bool) -> Self {
        BTreeNode {
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
            is_leaf,
        }
    }

    pub fn get_keys_values(&self) -> (&[K], &[V]) {
        (&self.keys, &self.values)
    }

    pub fn split_child(&mut self, index: usize, degree: usize) {
        let mut new_node = BTreeNode::new(self.children[index].is_leaf);
        let full_node = self.children[index].as_mut();

        new_node.keys.extend(full_node.keys.drain(degree..));
        new_node.values.extend(full_node.values.drain(degree..));

        if !full_node.is_leaf {
            new_node.children.extend(full_node.children.drain(degree..));
        }

        self.keys.insert(index, full_node.keys.pop().unwrap());
        self.values.insert(index, full_node.values.pop().unwrap());
        self.children.insert(index + 1, Box::new(new_node));
    }

    pub fn insert_non_full(&mut self, key: K, value: V, degree: usize) {
        let mut i = self.keys.len();

        if self.is_leaf {
            while i > 0 && self.keys[i - 1] > key {
                i -= 1;
            }
            self.keys.insert(i, key);
            self.values.insert(i, value);
        } else {
            while i > 0 && self.keys[i - 1] > key {
                i -= 1;
            }

            if self.children[i].keys.len() == 2 * degree - 1 {
                self.split_child(i, degree);
                if self.keys[i] < key {
                    i += 1;
                }
            }
            self.children[i].insert_non_full(key, value, degree);
        }
    }

    pub fn search(&self, key: &K) -> Option<&V> {
        let mut i = 0;
        while i < self.keys.len() && key > &self.keys[i] {
            i += 1;
        }

        if i < self.keys.len() && key == &self.keys[i] {
            return Some(&self.values[i]);
        }

        if self.is_leaf {
            return None;
        }

        self.children[i].search(key)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BTree<K, V> {
    root: Option<Box<BTreeNode<K, V>>>,
    degree: usize,
}

impl<K: Ord + Clone, V: Clone> BTree<K, V> {
    pub fn new(degree: usize) -> Self {
        BTree {
            root: None,
            degree,
        }
    }

    pub fn get_root(&self) -> Option<&BTreeNode<K, V>> {
        self.root.as_deref()
    }

    pub fn search(&self, key: &K) -> Option<&V> {
        match &self.root {
            Some(root) => root.search(key),
            None => None,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if let Some(root) = self.root.as_mut() {
            if root.keys.len() == 2 * self.degree - 1 {
                let mut new_root = BTreeNode::new(false);
                let old_root = std::mem::replace(root, Box::new(BTreeNode::new(false)));
                new_root.children.push(old_root);
                new_root.split_child(0, self.degree);
                new_root.insert_non_full(key, value, self.degree);
                self.root = Some(Box::new(new_root));
            } else {
                root.insert_non_full(key, value, self.degree);
            }
        } else {
            let mut root = BTreeNode::new(true);
            root.keys.push(key);
            root.values.push(value);
            self.root = Some(Box::new(root));
        }
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btree_insert_and_search() {
        let mut btree = BTree::new(2);

        btree.insert("key1".to_string(), "value1".to_string());
        btree.insert("key2".to_string(), "value2".to_string());
        btree.insert("key3".to_string(), "value3".to_string());

        assert_eq!(btree.search(&"key1".to_string()), Some(&"value1".to_string()));
        assert_eq!(btree.search(&"key2".to_string()), Some(&"value2".to_string()));
        assert_eq!(btree.search(&"key3".to_string()), Some(&"value3".to_string()));
        assert_eq!(btree.search(&"key4".to_string()), None);
    }

    #[test]
    fn test_btree_split_root() {
        let mut btree = BTree::new(2);

        btree.insert("key1".to_string(), "value1".to_string());
        btree.insert("key2".to_string(), "value2".to_string());
        btree.insert("key3".to_string(), "value3".to_string());
        btree.insert("key4".to_string(), "value4".to_string());

        assert!(btree.root.is_some());
        let root = btree.root.as_ref().unwrap();
        assert_eq!(root.keys.len(), 1);
        assert_eq!(root.keys[0], "key2".to_string());
    }
}