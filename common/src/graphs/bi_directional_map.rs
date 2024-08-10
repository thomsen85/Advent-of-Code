use std::hash::Hash;
use std::{collections::HashMap, usize};

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
}

pub struct NodeTranslator<T> {
    current_id: usize,
    bi_map: BiDirectionalMap<T, usize>,
}

impl<T> NodeTranslator<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            current_id: 0,
            bi_map: BiDirectionalMap::new(),
        }
    }

    pub fn get_or_insert(&mut self, key: T) -> usize {
        if let Some(val) = self.bi_map.get_val(&key) {
            return *val;
        }

        let id = self.current_id;
        self.current_id += 1;

        self.bi_map.insert(key, id);

        return id;
    }
}
