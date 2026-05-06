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

        if self.map.len() == self.capacity {
        // remove the tail node
        if let Some(tail_index) = self.tail {
            let old_key = self.nodes[tail_index].key.clone();
            self.detach(tail_index);
            self.map.remove(&old_key);
        }
}

        let index = self.nodes.len();
        self.nodes.push(Node {
            key: key.clone(),
            value,
            prev: None,
            next: None,
        });

        // If the cache exceeds its capacity, remove the least recently used item.
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
}