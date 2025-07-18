use crate::policy::eviction::EvictionPolicy;
use std::{collections::HashMap, fmt::Debug, hash::Hash};

pub struct Cache<K, V>
where
    K: Debug + Clone + Eq + Hash,
    V: Debug,
{
    data: HashMap<K, V>,
    capacity: usize,
    policy: Box<dyn EvictionPolicy<K, V>>, // Notice we did not cover this topic at all yet! We've touched some dyn, but it wasn't clear yer
}

impl<K, V> Cache<K, V>
where
    K: Debug + Clone + Eq + Hash,
    V: Debug,
{
    pub fn new(capacity: usize, policy: Box<dyn EvictionPolicy<K, V>>) -> Self {
        Self {
            data: HashMap::new(),
            capacity,
            policy,
        }
    }

    pub fn get_data(&self) -> &HashMap<K, V> {
        &self.data
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.data.len() >= self.capacity {
            if let Some(key_to_evict) = self.policy.evict() {
                self.data.remove(&key_to_evict);
            } else {
                println!("Enough space for more values...");
            }
        }

        if self.data.contains_key(&key) {
            return None;
        }

        self.policy.on_insert(key.clone());
        self.data.insert(key, value)
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.data.contains_key(key) {
            self.policy.on_access(key);
            self.data.get(key)
        } else {
            None
        }
    }
}
