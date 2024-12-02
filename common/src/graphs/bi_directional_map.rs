use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct BiDirectionalMap<K, V> {
    pub key_val: HashMap<K, V>,
    pub val_key: HashMap<V, K>,
}

impl<K, V> BiDirectionalMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            key_val: HashMap::new(),
            val_key: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, val: V) {
        self.key_val.insert(key.clone(), val.clone());
        self.val_key.insert(val, key);
    }

    pub fn get_val(&self, key: &K) -> Option<&V> {
        self.key_val.get(key)
    }

    pub fn get_key(&self, val: &V) -> Option<&K> {
        self.val_key.get(val)
    }

    pub fn contains_key(&self, node_name: &K) -> bool {
        self.key_val.contains_key(node_name)
    }
}

impl<K, V> Default for BiDirectionalMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
