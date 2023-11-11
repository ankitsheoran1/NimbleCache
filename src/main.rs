mod cache;
mod evictionpolicy;

use cache::Cache;
use evictionpolicy::LruPolicy;

fn main() {
    let mut cache = Cache::new(LruPolicy::new(2), 2);


    // Insert key-value pairs into the cache
    cache.insert("key1", "value1");
    cache.insert("key2", "value2");
    cache.get(&"key1");
    cache.insert("key3", "value3");

    // Get values from the cache
    println!("Value for key 'key1': {:?}", cache.get(&"key1"));
    println!("Value for key 'key2': {:?}", cache.get(&"key2"));
    println!("Value for key 'key3': {:?}", cache.get(&"key3"));

    // print cache len
    println!("Cache len: {}", cache.len());

    // Remove a key from the cache
    cache.remove(&"key2");
}



#[cfg(test)]
mod tests {
    use super::*;
    use evictionpolicy::LruPolicy;
    
    #[test]
    fn test_cache() {
        let mut cache = Cache::new(LruPolicy::new(2), 2);
        
        // Insert key-value pairs into the cache
        cache.insert("key1", "value1");
        cache.insert("key2", "value2");
        
        // Get values from the cache
        assert_eq!(cache.get(&"key1"), Some("value1"));
        assert_eq!(cache.get(&"key2"), Some("value2"));
        
        // Insert another key-value pair, causing eviction
        cache.insert("key3", "value3");
        
        // Get values from the cache
        assert_eq!(cache.get(&"key1"), None);
        assert_eq!(cache.get(&"key2"), Some("value2"));
        assert_eq!(cache.get(&"key3"), Some("value3"));
        
        // Remove a key from the cache
        cache.remove(&"key2");
        
        // Get values from the cache
        assert_eq!(cache.get(&"key1"), None);
        assert_eq!(cache.get(&"key2"), None);
        assert_eq!(cache.get(&"key3"), Some("value3"));
    }
}

