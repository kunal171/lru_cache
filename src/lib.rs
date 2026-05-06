//! LRU cache implementation using a HashMap for O(1) lookup and an arena-based
//! doubly-linked list for O(1) reordering and eviction.

use std::collections::HashMap;
use std::hash::Hash;

/// A simple LRU cache implementation using a HashMap and a doubly linked list.
struct Node<K, V> {
    key: K,// the key of the cache entry
    value: V,// the value of the cache entry
    prev: Option<usize>,// index of the previous node in the linked list
    next: Option<usize>,//index of the next node in the linked list
}

pub struct LruCache<K, V> {
    capacity: usize,// maximum number of items in the cache
    map: HashMap<K, usize>,// maps keys to their corresponding node index in the nodes vector
    nodes: Vec<Node<K, V>>,// a vector to store the nodes of the doubly linked list
    head: Option<usize>,// index of the most recently used node (head of the list)
    tail: Option<usize>,//index of the least recently used node (tail of the list)
}

// implementation of LRUCache
impl <K: Hash + Eq + Clone, V> LruCache<K, V> {
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


    // Push a node to the front of the linked list.
    fn push_front(&mut self, index:usize) {
        // Update the new head's pointers.
        self.nodes[index].prev = None;
        self.nodes[index].next = self.head;

        // Update the old head's previous pointer to the new head.
        if let Some(old_head) = self.head {
            self.nodes[old_head].prev = Some(index);
        } else {
            self.tail = Some(index);
        }
        // Update the head to the new node.
        self.head = Some(index);
    }

    // Put a key-value pair into the cache.
    pub fn put(&mut self, key: K, value: V) {
        if self.capacity == 0 {
            return;
        }

        // If key already exists, update value and move to front.
        if let Some(&index) = self.map.get(&key) {
            self.nodes[index].value = value;
            self.detach(index);
            self.push_front(index);
            return;
        }

        // Evict the tail if at capacity.
        if self.map.len() == self.capacity
            && let Some(tail_index) = self.tail
        {
            let old_key = self.nodes[tail_index].key.clone();
            self.detach(tail_index);
            self.map.remove(&old_key);
        }

        let index = self.nodes.len();
        self.nodes.push(Node {
            key: key.clone(),
            value,
            prev: None,
            next: None,
        });

        self.map.insert(key, index);
        self.push_front(index);
    }

    // Detach a node from the linked list.
    fn detach(&mut self, index:usize) {
        let prev = self.nodes[index].prev;
        let next = self.nodes[index].next;

        match prev {
            Some(p) => self.nodes[p].next = next,
            None => self.head = next 
        }

        match next{
            Some(n) => self.nodes[n].prev = prev,
            None => self.tail = prev,
        }

        self.nodes[index].prev = None;
        self.nodes[index].next = None;
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let index = *self.map.get(key)?;
        self.detach(index);
        self.push_front(index);
        Some(&self.nodes[index].value)
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

    #[test]
    fn put_and_len() {
        let mut cache = LruCache::new(3);
        cache.put(1, "a");
        cache.put(2, "b");
        assert_eq!(cache.len(), 2);
        assert!(!cache.is_empty());
    }

    #[test]
    fn get_returns_value() {
        let mut cache = LruCache::new(3);
        cache.put(1, "a");
        cache.put(2, "b");
        assert_eq!(cache.get(&1), Some(&"a"));
        assert_eq!(cache.get(&2), Some(&"b"));
    }

    #[test]
    fn get_missing_key_returns_none() {
        let mut cache = LruCache::new(3);
        cache.put(1, "a");
        assert_eq!(cache.get(&99), None);
    }

    #[test]
    fn get_moves_to_front() {
        let mut cache = LruCache::new(3);
        cache.put(1, "a");
        cache.put(2, "b");
        cache.put(3, "c");
        // order: 3, 2, 1

        cache.get(&1);
        // order: 1, 3, 2

        assert_eq!(cache.nodes[cache.head.unwrap()].key, 1);
    }

    #[test]
    fn evicts_least_recent() {
        let mut cache = LruCache::new(2);
        cache.put(1, "a");
        cache.put(2, "b");
        cache.put(3, "c");
        // key 1 evicted (was tail)

        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&"b"));
        assert_eq!(cache.get(&3), Some(&"c"));
    }

    #[test]
    fn eviction_respects_access_order() {
        let mut cache = LruCache::new(2);
        cache.put(1, "a");
        cache.put(2, "b");
        // order: 2, 1

        cache.get(&1);
        // order: 1, 2

        cache.put(3, "c");
        // key 2 evicted (was tail after get(&1))

        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&1), Some(&"a"));
        assert_eq!(cache.get(&3), Some(&"c"));
    }

    #[test]
    fn put_existing_key_updates_value() {
        let mut cache = LruCache::new(3);
        cache.put(1, "a");
        cache.put(2, "b");

        cache.put(1, "updated");

        assert_eq!(cache.get(&1), Some(&"updated"));
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn put_existing_key_moves_to_front() {
        let mut cache = LruCache::new(3);
        cache.put(1, "a");
        cache.put(2, "b");
        cache.put(3, "c");
        // order: 3, 2, 1

        cache.put(1, "updated");
        // order: 1, 3, 2

        assert_eq!(cache.nodes[cache.head.unwrap()].key, 1);

        cache.put(4, "d");
        // key 2 evicted (was tail)

        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&1), Some(&"updated"));
    }

    #[test]
    fn capacity_zero() {
        let mut cache = LruCache::new(0);
        cache.put(1, "a");
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.get(&1), None);
        assert!(cache.is_empty());
    }

    #[test]
    fn capacity_one() {
        let mut cache = LruCache::new(1);
        cache.put(1, "a");
        assert_eq!(cache.get(&1), Some(&"a"));
        assert_eq!(cache.len(), 1);

        cache.put(2, "b");
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&"b"));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn capacity_one_update_existing() {
        let mut cache = LruCache::new(1);
        cache.put(1, "a");
        cache.put(1, "b");
        assert_eq!(cache.get(&1), Some(&"b"));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn string_keys() {
        let mut cache = LruCache::new(2);
        cache.put("hello".to_string(), 1);
        cache.put("world".to_string(), 2);

        assert_eq!(cache.get(&"hello".to_string()), Some(&1));
        assert_eq!(cache.get(&"world".to_string()), Some(&2));
    }

    #[test]
    fn many_evictions() {
        let mut cache = LruCache::new(2);
        for i in 0..100 {
            cache.put(i, i * 10);
        }
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(&98), Some(&980));
        assert_eq!(cache.get(&99), Some(&990));
        assert_eq!(cache.get(&97), None);
    }
}