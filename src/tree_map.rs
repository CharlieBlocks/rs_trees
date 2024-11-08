/*
Author: Matthew Tindley

Summary:
    The internal tree_map module contains defenitions for a basic, unoptimised tree map stucture
    This structure is used by the StaticTreePlanner structure to optimise the tree structure
*/


// TreeNode<T, Idx>
// T - The value to store
// Idx - The index type. Must be equatable and clonable
//
// This is a structure that acts as the storage for a basic tree map
// The structure formulates a recursive tree pattern where each node has references to child nodes
// This does not allow for reverse lookups
pub struct TreeNode<T, Idx: PartialEq + Clone>  {
    // The key at the current depth
    // This is a subset of the total index
    // This is owned by the TreeNode type
    key: Idx,

    // The value stored by the tree node
    // Not all TreeNodes store values as some are intermediaries
    // The TreeNode owns T
    value: Option<T>,

    // A vector of owned TreeNodes
    // This represents the sub-nodes underneath this node
    nodes: Vec<TreeNode<T, Idx>>
}


// TreeMap<T, Idx>
// T - The value type to store
// Idx - the index type. Must be equatable and clonable and defaultable
//
// This structure acts as a container for the caller to interface with TreeNodes
// It can be considered a partial TreeNode but without the key and value fields
pub struct TreeMap<T, Idx: PartialEq + Clone + Default> {
    // The top-most node of the tree
    // This node has no value or key and is the root of the tree
    //
    // Note that it would be more memory efficient to use the TreeMap as an implicit root node
    // however that would require more code duplication
    head: TreeNode<T, Idx>
}




/*
Implementations
*/
impl<T, Idx: PartialEq + Clone> TreeNode<T, Idx> {

    // Insets a value into the tree
    // If the index doesn't exist then a new path is created
    // otherwise the value is overwritten
    //
    // The index must be provided as a reference to a slice of keys that formulate the index
    pub fn insert(&mut self, index: &[Idx], value: T) {
        // Check if the length of the index is 0
        // In this case we have reached the end of the slice and there are no more indices
        // Hence we can set the value
        if index.len() == 0 {
            self.value = Some(value); // Set value
            return; // Return early
        }

        // Get the first key in the index slice
        let key = index.first().unwrap();


        // Get next node
        let next = match self.find_node_mut(key) {
            // If we found a node just return it
            Some(n) => n,

            // Otherwise we need to create a new node
            None => {
                // Create new tree node
                let n = TreeNode {
                    key: (*key).clone(),
                    value: None,
                    nodes: vec!()
                };

                // Add new node to the nodes list
                // This takes ownership of n
                self.nodes.push(n);

                // Return reference to n
                self.nodes.last_mut().unwrap()
            }
        };

        // With this new node we can recursivly call insert again
        next.insert(&index[1..], value);
    }

    // Iterates over the tree and finds a value at the given index
    // If the value exists then a reference to it is returned
    // Otherwise None is returned
    #[inline(never)]
    pub fn find(&self, index: &[Idx]) -> Option<&T> {
        // Check if we have consumed the whole index
        // In this case we have no more sub-nodes to iterate over
        if index.len() == 0 {
            return self.value.as_ref();
        }

        // Get first key in index
        let key = index.first().unwrap();


        // Find the node
        let next = match self.find_node(key) {
            // If we found the node, return it
            Some(n) => n,

            // Otherwise the node doesn't exist
            // So return None
            None => { return None; }
        };

        // Recursivly find the value
        return next.find(&index[1..]);

    }



    /* Internal Methods */
    // Finds a requested node in the nodes list
    // Returns None if the node doesn't exist
    pub fn find_node(&self, key: &Idx) -> Option<&TreeNode<T, Idx>> {
        // Iterate over the self.nodes vector
        for node in &self.nodes {
            if node.key == *key { // Check if key matches
                return Some(node); // Return reference to node
            }
        };

        return None;
    }
    // Mutable counterpart of find_node
    fn find_node_mut(&mut self, key: &Idx) -> Option<&mut TreeNode<T, Idx>> {
        for node in &mut self.nodes {
            if node.key == *key {
                return Some(node);
            }
        };

        return None;
    }
}



impl<T, Idx: PartialEq + Clone + Default> TreeMap<T, Idx> {
    pub fn new() -> Self {
        TreeMap {
            head: TreeNode {
                key: Idx::default(),
                value: None,
                nodes: vec!()
            }
        }
    }

    // Inserts a value into the tree
    // This takes a slice of keys that make up the whole index
    // And the value to insert
    // If the value already exists it is ovewritten
    pub fn insert(&mut self, index: &[Idx], value: T) {
        // Check that index actually contains values
        if index.len() == 0 { return; }

        // Insert the key
        self.head.insert(index, value);
    }

    // Finds and returns a value from the tree
    // If the value exists then a reference to it is returned
    // Otherwise None is returned
    pub fn find(&self, index: &[Idx]) -> Option<&T> {
        if index.len() == 0 { return None; }

        return self.head.find(index);
    }
}




/*
Tests:
    - TreeMap::insert
    - TreeMap::find     On valid target (Return Some)
    - TreeMap::find     On invalid target (Return None)


*/
#[cfg(test)]
mod tests {
    use super::*;

    /*
    Test: Insert

    Summary:
        This test is to ensure that TreeMAp<i32, &str> can correctly insert elements without panicing.
        It does not check that the elemnt has been inserted correctly
     */
    #[test]
    fn insert() {
        let mut map: TreeMap<i32, &str> = TreeMap::new();

        // Check insert
        map.insert(vec!["a", "b", "c"].as_slice(), 1);

        // Check insert along created path
        map.insert(vec!["a", "b", "d"].as_slice(), 1);

        // Insert along new path
        map.insert(vec!["e", "f"].as_slice(), 1);

        // Long insert
        map.insert(vec!["e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p"].as_slice(), 1);
    }


    /*
    Test: Find

    Summary:
        Validates that TreeMap can correctly read back inserted elements
     */
    #[test]
    fn find() {
        let mut map: TreeMap<i32, &str> = TreeMap::new();

        map.insert(vec!["a", "b", "c"].as_slice(), 1);
        map.insert(vec!["a", "b", "d"].as_slice(), 2);
        map.insert(vec!["e", "f"].as_slice(), 3);


        // Check response
        assert_eq!(*map.find(vec!["a", "b", "c"].as_slice()).unwrap(), 1);
        assert_eq!(*map.find(vec!["a", "b", "d"].as_slice()).unwrap(), 2);
        assert_eq!(*map.find(vec!["e", "f"].as_slice()).unwrap(), 3);
    }


    /*
    Test: Find Invalid items

    Summary:
        Validates that TreeMap will error correctly when find is called on an unknown index
     */
    #[test]
    fn find_invalid() {
        let mut map: TreeMap<i32, &str> = TreeMap::new();
        map.insert(vec!["a", "b", "c"].as_slice(), 1);

        // Wrong from root
        assert_eq!(map.find(vec!["e", "f"].as_slice()).is_none(), true);

        // Wrong along index
        assert_eq!(map.find(vec!["a", "b", "z"].as_slice()).is_none(), true);
    }

}