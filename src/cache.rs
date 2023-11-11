use std::collections::HashMap;
use std::hash::Hash;
use crate::evictionpolicy::EvictionPolicy;

pub struct Cache<K, V, P> 
where 
P: EvictionPolicy<K, V>,
K: Eq + Hash + Clone,
V: Default + Clone {
    capacity: usize,
    storage: HashMap<K, V>,
    policy: P,
  
}


impl <K, V, P> Cache<K, V, P> 
where 
  P: EvictionPolicy<K, V>,
  K: Eq + Hash + Clone, 
  V: Default + Clone, 
  {
    pub fn new(policy: P, cap: usize) -> Self {
        Cache {
            capacity: cap,
            storage: HashMap::new(),
            policy,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.storage.get(key).cloned() {
            self.policy.on_access( key);
            Some(value.clone())

        } else {
            None
        }
    }

    pub fn insert(&mut self, key: K, val: V) {
        self.policy.on_insert( &key);
        self.storage.insert(key, val);
        if self.storage.len() > self.capacity {
            self.evict();
        }
        
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.storage.remove(key) {
            Some(value)
        } else {
            None
        }

    }

    pub fn evict(&mut self) {
        if let Some(key) = self.policy.on_evict() {
            self.storage.remove(&key);
        }
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

}

   