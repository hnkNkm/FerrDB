use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BPlusTreeNode<K, V> {
    keys: Vec<K>,
    values: Option<Vec<V>>,
    children: Option<Vec<Box<BPlusTreeNode<K, V>>>>,
    is_leaf: bool,
    next: Option<Box<BPlusTreeNode<K, V>>>,
}

impl<K: Ord + Clone, V: Clone> BPlusTreeNode<K, V> {
    pub fn new(is_leaf: bool) -> Self {
        BPlusTreeNode {
            keys: Vec::new(),
            values: if is_leaf { Some(Vec::new()) } else { None },
            children: if is_leaf { None } else { Some(Vec::new()) },
            is_leaf,
            next: None,
        }
    }

    pub fn get_keys_values(&self) -> (&[K], Option<&[V]>) {
        (&self.keys, self.values.as_ref().map(|v| &v[..]))
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

    pub fn search(&self, key: &K) -> Option<&V> {
        if self.is_leaf() {
            let pos = self.keys.iter().position(|k| k == key)?;
            self.values()?.get(pos)
        } else {
            let pos = self.keys.iter().take_while(|k| *k <= key).count();
            self.children()?.get(pos)?.search(key)
        }
    }

    pub fn insert_non_full(&mut self, key: K, value: V, degree: usize) {
        if self.is_leaf {
            if self.keys.iter().any(|k| k == &key) {
                return;
            }
            let pos = self.keys.iter().take_while(|k| *k <= &key).count();
            self.keys.insert(pos, key);
            self.values.as_mut().unwrap().insert(pos, value);
        } else {
            let mut pos = self.keys.iter().take_while(|k| *k <= &key).count();
            if self.children.as_ref().unwrap()[pos].keys.len() == 2 * degree - 1 {
                self.split_child(pos, degree);
                pos = self.keys.iter().take_while(|k| *k <= &key).count();
            }
            self.children.as_mut().unwrap()[pos].insert_non_full(key, value, degree);
        }
    }

    pub fn split_child(&mut self, index: usize, degree: usize) {
        let child = self.children.as_mut().unwrap()[index].as_mut();
        if child.is_leaf {
            let split_index = degree;
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
            let split_index = degree - 1;
            let mut new_node = BPlusTreeNode::new(false);
            new_node.keys = child.keys.split_off(split_index + 1);
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
    pub fn new(degree: usize) -> Self {
        BPlusTree { root: None, degree }
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
            new_root.children = Some(vec![std::mem::replace(root, Box::new(BPlusTreeNode::new(false)))]);
            new_root.split_child(0, self.degree);
            new_root.insert_non_full(key, value, self.degree);
            self.root = Some(Box::new(new_root));
        } else {
            self.root.as_mut().unwrap().insert_non_full(key, value, self.degree);
        }
    }

    /// プライマリキー検索：主キーであるキーに一致する行を返す
    pub fn search(&self, key: &K) -> Option<&V> {
        (**self.root.as_ref()?).search(key)
    }    
}
