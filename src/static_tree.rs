/*
Author: Matthew Tindley

Summary:
    A StaticTree is a readonly tree that can only be created once
    The tree exists within a single block of memory for quick lookup times
    The StaticTree must be created using the StaticTreePlanner struct

    Due to the continous nature of StaticTree it is recommended to store as little data as possible in the actual tree structure
*/

use crate::dynamic_array::DynamicArray;
use std::{alloc::Layout, fmt::Debug};

// TreeOffset is an i32 index into the DynamicArray structure in the StaticTree
// It is used to specify the offset to find an item
type TreeOffset = i32;

pub struct TreeNode<T, Idx: PartialEq> {
    pub key: Idx,
    pub value: Option<T>,
    pub list_length: i32,
    pub list_head: TreeOffset
}

// pub struct TreeBranch {
//     pub node: TreeOffset,
//     pub next: TreeOffset
// }

// The readonly static tree
// This contains a DynamicArray which contains the tree data
pub struct StaticTree {
    pool: DynamicArray
}

/* Implementation */
impl StaticTree {

    pub fn new(size: usize) -> Self {
        StaticTree {
            pool: DynamicArray::new(Layout::from_size_align(size, 1).unwrap())
        }
    }

    #[inline]
    pub fn raw(&self) -> &DynamicArray {
        return &self.pool;
    }

    // Looks up a key in the static tree
    // Returns a reference to it if it exists
    // Otherwise returns none
    #[inline(never)]
    pub fn find<T, Idx: 'static + PartialEq + Clone>(&self, index: &[Idx]) -> Option<&T> {

        // State variables
        let node_size = std::mem::size_of::<TreeNode<T, Idx>>();
        let mut current_node: &TreeNode<T, Idx> = self.pool.get(0);
        let mut current_offset: i32= 0;
        let mut branch_idx = 0;
        let mut keychain_idx = 0;

        loop {
            let test_node: &TreeNode<T, Idx> = self.pool.get((current_node.list_head + current_offset) as usize);

            // Check if node matches index
            // If not then we continue to the next node
            if test_node.key != index[keychain_idx] {
                if branch_idx == current_node.list_length { return None; } // Null check.

                // Increment state variables
                current_offset += node_size as i32;
                branch_idx += 1;

                continue; // Skip back
            }

            // Otherwise, we hit the correct node
            // Update state
            keychain_idx += 1;
            if keychain_idx == index.len() { // Reached end of index
                return test_node.value.as_ref();
            }

            if test_node.list_head == -1 {
                return None;
            }

            current_node = test_node;
            branch_idx = 0;
            current_offset = 0;
        }
    }
}



// Debugging Implementations
#[cfg(debug_assertions)]
impl<T: Debug, Idx: PartialEq + Debug> std::fmt::Debug for TreeNode<T, Idx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("key", &self.key)
            .field("value", &self.value)
            .field("list_length", &self.list_length)
            .field("list_head", &self.list_head)
            .finish()
    }
}