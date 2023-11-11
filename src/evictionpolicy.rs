use std::collections::HashMap;
use std::hash::Hash;


pub trait EvictionPolicy<K, V> 
where 
  K: Eq + Hash + Clone,
  V: Default + Clone,
  {
    fn on_insert(&mut self, key: &K);
    fn on_access(&mut self, key: &K);
    fn on_evict(&mut self) -> Option<K>;

  }

  pub struct LruPolicy<K> 
  where K: Eq + Hash + Clone {
    indexes: HashMap<K, usize>,
    capacity: usize
  }

  impl<K> LruPolicy<K> 
  where K: Eq + Hash + Clone
  {
    pub fn new(capacity: usize) -> Self {
        LruPolicy {
            indexes: HashMap::with_capacity(capacity),
            capacity
        }
    } 
  

  fn update(&mut self, k:&K) {
    for (key, idx) in self.indexes.iter_mut() {
            if key == k {
                *idx = 0;
            } else  {
                *idx += 1;
            }
        }

    self.indexes.insert(k.clone(), 0);
  }

  fn candidate_key(&mut self) -> Option<&K> {
    let mut max_value = None;
    let mut max_key = None;

    // Iterate over the indexes HashMap
    for (key, value) in self.indexes.iter() {
        // If max_value is None or the current value is greater than max_value
        if max_value.is_none() || value > max_value.unwrap() {
            // Update max_value and max_key
            max_value = Some(value);
            max_key = Some(key);
        }
    }

    // Return max_key, which is the key associated with the maximum value
    // If the HashMap was empty, max_key will still be None
    max_key

  }
}

  impl<K, V: Clone> EvictionPolicy<K, V> for LruPolicy<K>
where
    K: Eq + Hash + Clone,
    V: Default + Clone,
{
    fn on_insert(&mut self, key: &K) {
        if self.indexes.len() >= self.capacity {
            if let Some(k) = self.candidate_key() {
                self.indexes.remove(key);
            }
        }
            
                self.update(key);
            
        
    }


    fn on_access(&mut self,  key: &K) {
        
            //update_values(data, index);
            self.update(key);
        
    }

    fn on_evict(&mut self) -> Option<K> {
        let candidate_key = self.candidate_key().cloned();
        
        if let Some(k) = candidate_key {
            self.indexes.remove(&k);
            return Some(k);
        }
        
        None
    }
}