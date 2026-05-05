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
        self.nodes[index].prev = None;
        self.nodes[index].next = self.head;

        // Update the old head's previous pointer to the new head.
        if let Some(old_head) = self.head {
            self.nodes[old_head].prev = Some(index);
        } else {
            self.tail = Some(index);
        }

        self.head = Some(index);
    }

    pub fn put(&mut self, key: K, value: V) {
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
}