/*
Author: Matthew Tindley

Summary:
    To construct a StaticTree a StaticTreePlanner must be used. This formulates a standard tree and then
    compiles it into the readonly StaticTree type

    The StaticTreePlanner follows a builder style pattern
*/


use std::collections::vec_deque;
use std::collections::VecDeque;

use crate::static_tree::*;
use crate::counted_tree_map::*;


pub struct StaticTreePlanner<T, Idx: PartialEq + Clone + Default> {

    // A TreeMap that contains the values to be inserted into the tree
    map: CountedTreeMap<T, Idx>,
}




/* Implementation */
impl<T, Idx: PartialEq + Clone + Default> StaticTreePlanner<T, Idx> {
    // New function
    // Returns a new, blank StaticTreePlanner
    pub fn new() -> Self {
        StaticTreePlanner {
            map: CountedTreeMap::new(),
        }
    }


    // Adds a key-value to the internal tree
    // This function returns itself
    pub fn add(mut self, key: &[Idx], value: T) -> Self {
        self.map.insert(key, value);

        self
    }

    // Compiles the stored TreeMap into a StaticTree
    pub fn compile(mut self) -> StaticTree {
        // Compute output size
        let pool_size: usize = self.calculate_pool_size();
        let tree: StaticTree = StaticTree::new(pool_size);


        // Sort map
        self.map.root_ref_mut().sort_nodes();

        // Create stack
        // This will hold the nodes that we are currently underneath
        // This specifically allows us to traverse back up the tree
        let mut stack: VecDeque<CountedTreeNode<T, Idx>> = VecDeque::new();
        let mut pool_offset: i32 = 0;
        let mut last_branch_offset: i32 = 0;


        // Push root node to stack
        let mut root = self.map.root();
        root.visited = true;
        root.node_offset = -(std::mem::size_of::<TreeNode<T, Idx>>() as i32);
        stack.push_back(root);


        // Write implicit node
        for i in 0..stack.get(0).unwrap().nodes.len() {
            let branch = tree.raw().get_mut::<TreeBranch>(pool_offset as usize);
            // Zero for safety
            branch.node = -1;
            branch.next = -1;

            if i > 0 {
                tree.raw().get_mut::<TreeBranch>((pool_offset - TREE_BRANCH_SIZE) as usize).next = pool_offset;
            }

            pool_offset += TREE_BRANCH_SIZE as i32;
        }

        
        // Loop while there are nodes in the stack
        while let Some(mut node) = stack.pop_back() {

            if !node.visited {
                // Get a reference to the node in the DynamicPool of the static tree
                let pool_node = tree.raw().get_mut::<TreeNode<T, Idx>>(pool_offset as usize);

                // TODO: Write pool_node.key and pool_node.value
                node.node_offset = pool_offset; // Write the offset of the node for future reference
                pool_node.key = node.key.clone();
                pool_node.value = node.value.take();

                // Get the parent branch and set the node it points to
                println!("{}", last_branch_offset);
                tree.raw().get_mut::<TreeBranch>(last_branch_offset as usize).node = pool_offset;
                pool_offset += std::mem::size_of::<TreeNode<T, Idx>>() as i32; // Increment pool_offset to the beginning of the next branch

                // Set branch head if children exist
                pool_node.list_head = if node.nodes.len() > 0 { pool_offset } else { -1 };


                // Build branches
                for i in 0..node.nodes.len() {
                    let branch = tree.raw().get_mut::<TreeBranch>(pool_offset as usize);

                    // "Zero" branch
                    branch.node = -1; 
                    branch.next = -1;

                    // If i > 0 we can reference the previous node to set the .next attribute
                    if i > 0 {
                        tree.raw().get_mut::<TreeBranch>((pool_offset - TREE_BRANCH_SIZE) as usize).next = pool_offset;
                    }

                    // Increment pool_offset
                    pool_offset += TREE_BRANCH_SIZE as i32;
                }

                // Set visited to be true so we don't recreate the node
                node.visited = true;
            }


            // Add next child node to stack
            let next_node = match node.consume_next_node() {
                Some(n) => n,
                None => { continue; }
            };

            // Set last branch offset
            last_branch_offset = node.node_offset + (std::mem::size_of::<TreeNode<T, Idx>>() as i32) + (node.built_sub_nodes * TREE_BRANCH_SIZE);
            node.built_sub_nodes += 1;

            // Stack must have more nodes
            // So we return the item to the stack
            // And add the next child node
            stack.push_back(node);
            stack.push_back(next_node);
        }

        return tree;
    }



    /* Helper Methods */
    fn calculate_pool_size(&self) -> usize {
        let mut node_count: usize = 0;
        let mut branch_count: usize = 0;

        let mut open_nodes: VecDeque<&CountedTreeNode<T, Idx>> = VecDeque::new();
        open_nodes.push_back(self.map.root_ref());

        
        while open_nodes.len() > 0 {
            // Pop front
            let node = open_nodes.pop_front().unwrap();

            // Add node and branches
            node_count += 1;
            branch_count += node.nodes.len();

            // Push back sub-nodes
            for child_node in &node.nodes {
                open_nodes.push_back(child_node);
            }
        };

        return (node_count * std::mem::size_of::<TreeNode<T, Idx>>()) + (branch_count * std::mem::size_of::<TreeBranch>());

    }

}