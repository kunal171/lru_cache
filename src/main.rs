//! Demo showing LRU cache insert, access reordering, and eviction behavior.

use lru_cache::LruCache;

fn main() {
    let mut cache = LruCache::new(3);

    println!("=== Inserting 3 items ===");
    cache.put(1, "apple");
    cache.put(2, "banana");
    cache.put(3, "cherry");
    println!("len: {}, capacity: {}", cache.len(), cache.capacity());

    println!("\n=== Getting key 1 (moves to front) ===");
    match cache.get(&1) {
        Some(val) => println!("get(1) = {}", val),
        None => println!("get(1) = not found"),
    }

    println!("\n=== Inserting key 4 (should evict least recent) ===");
    cache.put(4, "date");
    println!("len: {}", cache.len());

    // key 2 should be evicted (it was least recent after we accessed key 1)
    match cache.get(&2) {
        Some(val) => println!("get(2) = {}", val),
        None => println!("get(2) = evicted!"),
    }

    match cache.get(&3) {
        Some(val) => println!("get(3) = {}", val),
        None => println!("get(3) = evicted!"),
    }
}