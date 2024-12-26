use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Index;
use std::ops::IndexMut;

pub struct DefaultMap<K, V>
where
    K: Hash + Eq,
    V: Default,
{
    map: HashMap<K, V>,
}

impl<K, V> DefaultMap<K, V>
where
    K: Hash + Eq,
    V: Default + Clone,
{
    pub fn new() -> Self {
        DefaultMap {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }

    pub fn get_map(&self) -> &HashMap<K, V> {
        &self.map
    }

    pub fn get_or_default(&mut self, key: K, default: V) -> V {
        self.map.get(&key).cloned().unwrap_or(default)
    }
}

impl<K, V> Index<K> for DefaultMap<K, V>
where
    K: std::hash::Hash + Eq,
    V: Default,
{
    type Output = V;

    fn index(&self, key: K) -> &Self::Output {
        self.map.get(&key).expect("Key does not exist")
    }
}

impl<K, V> IndexMut<K> for DefaultMap<K, V>
where
    K: std::hash::Hash + Eq,
    V: Default,
{
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        self.map.entry(key).or_insert_with(Default::default)
    }
}
