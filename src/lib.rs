use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

/// A simple LRU cache implementation using a HashMap and a doubly linked list.
struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<usize>,
    next: Option<usize>,
}

pub struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, usize>,
    nodes: Vec<Node<K, V>>,
    head: Option<usize>,
    tail: Option<usize>,
}

// implementation of LRUCache
impl <K: Hash + Eq, V> LruCache<K, V> {
    // Create a new LRUCache with the given capacity.
    pub fn new(capacity: usize) -> Self {
        LruCache {
            capacity,
            map: HashMap::new(),
            nodes: Vec::new(),
            head: None,
            tail: None,
        }
    }
    
    // Get the length of the cache.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    // get the capacity of the cache.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    // Check if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_cache_is_empty() {
        let cache: LruCache<i32, &str> = LruCache::new(3);
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.capacity(), 3);
        assert!(cache.is_empty());
    }
}