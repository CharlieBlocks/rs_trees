/*
Author: Matthew Tindley

Summary:
    To construct a StaticTree a StaticTreePlanner must be used. This formulates a standard tree and then
    compiles it into the readonly StaticTree type

    The StaticTreePlanner follows a builder style pattern
*/


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

        // Create state variables
        let node_size = std::mem::size_of::<TreeNode<T, Idx>>() as i32; // Precompute
        let mut pool_offset: i32 = node_size; // Placement of the first branch
        let mut last_branch_offset: i32 = 0; // Offset of the current node's parent branch


        // Push root node to stack
        let mut root = self.map.root();
        root.visited = true;
        root.list_offset = node_size;
        stack.push_back(root);



        // Write root node
        let node0 = tree.raw().get_mut::<TreeNode<T, Idx>>(0);
        node0.list_length = stack.get(0).unwrap().nodes.len() as i32;
        node0.list_head = node_size;

        // Write root subnodes
        for i in 0..stack.get(0).unwrap().nodes.len() {
            let sub_node = &mut stack.get_mut(0).unwrap().nodes[i];
            let branch = tree.raw().get_mut::<TreeNode<T, Idx>>(pool_offset as usize);

            branch.key          = sub_node.key.clone();
            branch.value        = sub_node.value.take();
            branch.list_length  = sub_node.nodes.len() as i32;
            branch.list_head    = -1;

            pool_offset += node_size;
        }
        

        // Loop while there are nodes in the stack
        while let Some(mut node) = stack.pop_back() {

            // Do branches need creating?
            if !node.visited {

                // Set the list_offset of the node
                // This is used to compute the branch offsets later
                node.list_offset = pool_offset;

                // Set previous TreeNode::list_head value
                tree.raw().get_mut::<TreeNode<T, Idx>>(last_branch_offset as usize).list_head = pool_offset;

                // Create branches by looping over nodes
                for i in 0..node.nodes.len() {
                    let branch = tree.raw().get_mut::<TreeNode<T, Idx>>(pool_offset as usize);

                    // Initialise branch values
                    branch.key          = node.nodes[i].key.clone();
                    branch.value        = node.nodes[i].value.take();
                    branch.list_length  = node.nodes[i].nodes.len() as i32;
                    branch.list_head    = -1;

                    // Increment to next branch
                    pool_offset += node_size;
                }

                node.visited = true;
            }


            // Get next node
            // None this is a leaf node
            // Some(n) there are 1 or more child nodes
            let next_node = match node.consume_next_node() {
                Some(n) => n,
                None => { continue; }
            };


            // Calculate last branch offset
            // list_offset is the offset of of the node + node_size
            last_branch_offset = node.list_offset + (node_size as i32 * node.built_sub_nodes);
            node.built_sub_nodes += 1; // Increment the built node count. This technically happens on before the node is built as it's branches are built on the next iteration


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

        let mut open_nodes: VecDeque<&CountedTreeNode<T, Idx>> = VecDeque::new();
        open_nodes.push_back(self.map.root_ref());

        
        while open_nodes.len() > 0 {
            // Pop front
            let node = open_nodes.pop_front().unwrap();

            // Add node and branches
            node_count += 1;

            // Push back sub-nodes
            for child_node in &node.nodes {
                open_nodes.push_back(child_node);
            }
        };

        println!("Calcultaed node count: {}", node_count);
        return node_count * std::mem::size_of::<TreeNode<T, Idx>>();

    }

}