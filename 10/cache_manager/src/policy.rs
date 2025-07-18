pub mod eviction {
    use crate::policy::eviction::policy_consts::MAX_KEYS_IN_QUEUE;
    use std::collections::{LinkedList, VecDeque};
    use std::fmt::Debug;

    pub trait EvictionPolicy<K, V> 
    where 
    K: Debug,
    V: Debug,
    {
        fn on_access(&mut self, key: &K);
        fn on_insert(&mut self, key: K);
        fn evict(&mut self) -> Option<K>;
    }

    #[derive(Debug)]
    pub struct FifoPolicy<K>
    where
        K: Debug,
    {
        keys_in_order: VecDeque<K>,
    }

    impl<K: Debug> FifoPolicy<K> {
        pub fn new() -> Self {
            Self {
                keys_in_order: VecDeque::with_capacity(MAX_KEYS_IN_QUEUE),
            }
        }
    }

    impl<K: Debug, V: Debug> EvictionPolicy<K, V> for FifoPolicy<K> {
        fn on_access(&mut self, key: &K) {
            println!(
                "{self:?} does not care about access, access happens either on back or on front"
            );
        }

        fn on_insert(&mut self, key: K) {
            if self.keys_in_order.len() < MAX_KEYS_IN_QUEUE {
                self.keys_in_order.push_back(key);
            }
        }

        fn evict(&mut self) -> Option<K> {
            if self.keys_in_order.is_empty() {
                None
            } else {
                self.keys_in_order.pop_front()
            }
        }
    }

    pub struct LruPolicy<K>
    where
        K: Debug + Eq + Copy,
    {
        keys_in_order: LinkedList<K>,
    }

    impl<K: Debug + Eq + Copy> LruPolicy<K> {
        pub fn new() -> Self {
            Self {
                keys_in_order: LinkedList::new(),
            }
        }
    }

    impl<K: Debug + Eq + Copy, V: Debug> EvictionPolicy<K, V> for LruPolicy<K> {
        fn on_access(&mut self, key: &K) {
            if !self.keys_in_order.is_empty() {
                let mut updated_list: LinkedList<K> = self
                    .keys_in_order
                    .clone()
                    .into_iter()
                    .filter(|a| a != key)
                    .collect();
                updated_list.push_back(*key);
                self.keys_in_order.clear();
                self.keys_in_order = updated_list;
            }
        }

        fn on_insert(&mut self, key: K) {
            if self.keys_in_order.len() < MAX_KEYS_IN_QUEUE {
                self.keys_in_order.push_back(key);
            }
        }

        fn evict(&mut self) -> Option<K> {
            if self.keys_in_order.is_empty() {
                None
            } else {
                self.keys_in_order.pop_front()
            }
        }
    }

    mod policy_consts {
        pub const MAX_KEYS_IN_QUEUE: usize = 256;
    }
}
