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
    pub list_head: TreeOffset
}

// OPTIMISATION //
// Move the number of TreeBranches into the TreeNode
// Then drop the next attribute on TreeBranch
// Should reduce memory usage and increase lookup speed
pub struct TreeBranch {
    pub node: TreeOffset,
    pub next: TreeOffset
}

// The readonly static tree
// This contains a DynamicArray which contains the tree data
pub struct StaticTree {
    pool: DynamicArray
}

pub static TREE_BRANCH_SIZE: i32 = std::mem::size_of::<TreeBranch>() as i32;


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
    pub fn find<T, Idx: 'static + PartialEq + Clone>(&self, index: &[Idx]) -> Option<&T> {

        // State variables
        // Note that the tree is constructed with an implicit root node
        // We skip straight to the branches undernath the imaginary node 
        let mut current_branch: Option<&TreeBranch> = Some(self.pool.get(0)); // Get first branch
        let mut keychain_idx = 0;

        loop {
            // Check current branch
            let current_node: &TreeNode<T, Idx> = self.pool.get(current_branch.unwrap().node as usize);

            // Check if the node matches the index
            // If the node matches then disable the is_branch flag. The current_node state is already set. We can slice off the first item in the keychain
            if current_node.key != index[keychain_idx] { // Optimise with hashing?
                if current_branch.unwrap().next == -1 { return None; } // Null check

                current_branch = Some(self.pool.get(current_branch.unwrap().next as usize));
                continue; // Return to top of loop
            }

            keychain_idx += 1;
            if keychain_idx == index.len() { // Reached end of index 
                return current_node.value.as_ref();
            }

            // Not found
            if current_node.list_head == -1 {
                return None
            }

            current_branch = Some(self.pool.get(current_node.list_head as usize));
        }
    }
}



// Debugging Implementations
impl std::fmt::Debug for TreeBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Branch")
            .field("node", &self.node)
            .field("next", &self.next)
            .finish()
    }
}

impl<T: Debug, Idx: PartialEq + Debug> std::fmt::Debug for TreeNode<T, Idx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("key", &self.key)
            .field("value", &self.value)
            .field("list_heda", &self.list_head)
            .finish()
    }
}