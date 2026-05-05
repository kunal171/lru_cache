# LRU Cache

A Rust learning project for practicing data structure design, generics, and
ownership patterns by building an LRU (Least Recently Used) cache from scratch.

This is the Phase 1D project in the Rust foundations track.

## What Is An LRU Cache

An LRU cache is a fixed-capacity key-value store that evicts the least recently
used entry when full. Every access (get or put) marks that entry as the most
recently used.

```text
put(1, "a")  put(2, "b")  put(3, "c")  get(1)  put(4, "d")
                                                      |
                                         capacity reached
                                                      |
                                         evict key 2 (least recent)
```

## What This Cache Will Do

- Store generic key-value pairs with a fixed capacity
- O(1) get and put operations
- Automatically evict the least recently used entry when at capacity
- Move accessed entries to the most-recent position
- Update existing keys in place

## Current State

Project just initialized. Planning milestones.

## Milestone Plan

### Milestone 1: Basic Structure and Put

Create `LruCache<K, V>` with fixed capacity, insert entries, `len()` and `capacity()`.

### Milestone 2: Get With Move-to-Front

`get(key)` returns the value and moves the entry to the most-recent position.

### Milestone 3: Eviction

When `put()` exceeds capacity, evict the least recently used entry.

### Milestone 4: Full API and Edge Cases

Update existing keys, handle edge cases (capacity 0/1), comprehensive unit tests.

## Planned Concepts

- Generic structs (`LruCache<K, V>` where `K: Hash + Eq`)
- Data structure composition (HashMap + doubly-linked list)
- Arena/index-based linked list (safe Rust alternative to raw pointers)
- O(1) lookup, insertion, and eviction
- API design (clean public interface, hidden internals)
- Thorough unit testing

## Useful Commands

```bash
cargo run
cargo check
cargo test
cargo fmt --check
cargo clippy
```
