/*
Author: Matthew Tindley

Summary:
    A StaticTree is a readonly tree that can only be created once
    The tree exists within a single block of memory for quick lookup times
    The StaticTree must be created using the StaticTreePlanner struct

    Due to the continous nature of StaticTree it is recommended to store as little data as possible in the actual tree structure
*/

use crate::dynamic_array::DynamicArray;

// TreeOffset is an i32 index into the DynamicArray structure in the StaticTree
// It is used to specify the offset to find an item
type TreeOffset = i32;

pub struct TreeNode<T, Idx: PartialEq + Clone> {
    key: Idx,
    value: Option<T>,
    list_head: TreeOffset
}

pub struct TreeBranch {
    node: TreeOffset,
    next: TreeOffset
}

// The readonly static tree
// This contains a DynamicArray which contains the tree data
pub struct StaticTree {
    pool: DynamicArray
}



/* Implementation */
impl StaticTree {

    // Looks up a key in the static tree
    // Returns a reference to it if it exists
    // Otherwise returns none
    pub fn find<T, Idx: 'static + PartialEq + Clone>(&self, index: &[Idx]) -> Option<&T> {

        // State variables
        // Note that the tree is constructed with an implicit root node
        // We skip straight to the branches undernath the node
        let mut is_branch = true;
        let mut current_branch: Option<&TreeBranch> = Some(self.pool.get(0)); // Get first branch
        let mut current_node: Option<&TreeNode<T, Idx>> = None;
        let mut keychain = index;

        loop {

            // Handle looping over branches
            if is_branch {
                // Check current branch
                current_node = Some(self.pool.get(current_branch.unwrap().node as usize));

                // Check if the node matches the index
                // If the node matches then disable the is_branch flag. The current_node state is already set. We can slice off the first item in the keychain
                if current_node.unwrap().key == keychain[0] {
                    keychain = &keychain[1..];
                    is_branch = false;
                }
                else { // Otherwise we step to the next branch
                    if current_branch.unwrap().next == -1 { return None; } // Null check
                    
                    // Set branch
                    current_branch = Some(self.pool.get(current_branch.unwrap().next as usize));
                }

                continue; // Skip back to top of loop
            }

            // Otherwise we must be at a TreeNode<T, Idx> type
            // First check if we have any items left in the keychain
            if keychain.len() == 0 {
                return current_node.unwrap().value.as_ref();
            }

            // Check if we have a node
            if current_node.unwrap().list_head == -1 {
                return None;
            }


            // Otherwise begin traversing the branch
            current_branch = Some(self.pool.get(current_node.unwrap().list_head as usize));
            is_branch = true;
        }
    }

}