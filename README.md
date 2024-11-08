# Rust Trees

rs_trees is a library that contains a collection of Tree-like data structures. It contains multiple structures for storing data, these are "mostly" safe.

Structures
- `TreeMap<T, Idx>` - A basic N-branches style of tree. It can contain an arbitary amount of branches and can store a value at each level
- `StaticTree` - Similar to `TreeMap` but it is precompiled using the `StaticTreePlanner` structure to optimise memory placement for faster lookups.


# TreeMap
A `TreeMap` is a basic N-branches tree map. It has a generic storage type and index type. Al nodes are stored in Vec types, this means that the Tree is stored on the Heap. Hence memory fragmentation is likely on larger trees, for better performance use `StaticTree`

```rust
use rs_trees::TreeMap

fn main() {
    // Create a new TreeMap with a storage of i32 and an index of &str
    let mut map: TreeMap<i32, &str> = TreeMap::new();

    // Add values to the map
    // fn insert(&mut self, key: &[Idx], value: T) -> ()
    map.insert(vec!["a", "b"].as_slice(), 1);

    // Retrive value from map
    // fn find(&self, key: &[Idx]) -> Option<&T>
    map.find(vec!["a", "b"].as_slice()).unwrap(); // Returns 1
}
```


## StaticTree
A `StaticTree` is a readonly, N-branches, tree map. It is precompiled from using the `StaticTreePlanner` type. The `StaticTree` stores it's nodes in a single, internal array. This array is sized exactly to the required size. Furthermore the `StaticTreePlanner` optimises the memory layout of the `StaticTree` to increase lookup times for more common paths

```rust
use rs_trees::{ StaticTree, StaticTreePlanner }

fn main() -> () {
    // Create a Planner and then use the builder pattern to create a new StaticTree
    let tree: StaticTree = StaticTreePlanner<i32, &str>::new()
        .add(vec!["a", "b"].as_slice(), 1)
        .add(vec!["a", "c"].as_slice(), 2)
        .compile(); // Create StaticTree

    // Lookup value.
    // Note the generic parameters
    tree.find::<i32, &str>(vec!["a", "b"].unwrap()).unwrap(); // Returns 1
}
```