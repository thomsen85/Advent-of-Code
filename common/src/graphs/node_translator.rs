use std::hash::Hash;

use super::bi_directional_map::BiDirectionalMap;

#[derive(Debug)]
pub struct NodeTranslator<T> {
    current_id: usize,
    pub bi_map: BiDirectionalMap<T, usize>,
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

    pub fn get(&self, key: &T) -> Option<usize> {
        self.bi_map.get_val(key).map(usize::to_owned)
    }

    pub fn get_or_insert(&mut self, key: T) -> usize {
        if let Some(val) = self.get(&key) {
            return val;
        }

        let id = self.current_id;
        self.current_id += 1;

        self.bi_map.insert(key, id);

        id
    }

    pub fn contains(&self, node_name: &T) -> bool {
        self.bi_map.contains_key(node_name)
    }
}

impl<T> Default for NodeTranslator<T>
where
    T: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
