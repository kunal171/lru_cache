# LRU Cache

A generic, fixed-capacity LRU (Least Recently Used) cache built from scratch in safe Rust.

## What It Does

A key-value store that evicts the least recently used entry when full. Every access (`get` or `put`) marks that entry as the most recently used.

```text
put(1, "a")  put(2, "b")  put(3, "c")  get(1)  put(4, "d")
                                                      |
                                         capacity reached
                                                      |
                                         evict key 2 (least recent)
```

## Features

- Generic over key and value types (`LruCache<K, V>` where `K: Hash + Eq + Clone`)
- O(1) get, put, and eviction
- Automatic eviction of the least recently used entry at capacity
- Move-to-front on access (both `get` and `put`)
- Update existing keys in place without growing the cache
- Handles edge cases: capacity 0, capacity 1, duplicate keys

## Architecture

```text
LruCache
├── map: HashMap<K, usize>     — O(1) key-to-index lookup
├── nodes: Vec<Node<K, V>>     — arena storing all nodes
├── head ──→ most recent       — front of doubly-linked list
└── tail ──→ least recent      — eviction target
```

Two data structures work together:
- **HashMap** gives O(1) key lookup, mapping keys to indices in the node Vec
- **Arena-based doubly-linked list** gives O(1) reordering and eviction, using `Vec<Node>` with `Option<usize>` index pointers instead of raw pointers

## API

```rust
LruCache::new(capacity)           // create with fixed capacity
cache.put(key, value)             // insert or update, evict if full
cache.get(&key) -> Option<&V>     // lookup + move to front
cache.len() -> usize              // number of entries
cache.capacity() -> usize         // max entries
cache.is_empty() -> bool          // true if no entries
```

## Project Structure

```text
src/
├── lib.rs    — LruCache struct, Node, all methods, 14 unit tests
└── main.rs   — demo: insert, access reordering, update, eviction
```

## Commands

```bash
cargo run           # run the demo
cargo test          # run all 14 tests
cargo clippy        # lint check
cargo fmt --check   # format check
```

## Dependencies

None. Standard library only.
