/*
Author: Matthew Tindley

This is a variation of the standard tree map that keeps track of the number of children node at each depth
This is intended for internal use by StaticTreePlanner only

This file will have left comments but is esentially copied from tree_map.rs
*/

pub struct CountedTreeNode<T, Idx: PartialEq + Clone> {
    pub key: Idx,

    pub value: Option<T>,

    idx_counts: i32,
    pub built_sub_nodes: i32,
    pub visited: bool,
    pub list_offset: i32,

    pub nodes: Vec<CountedTreeNode<T, Idx>>
}


pub struct CountedTreeMap<T, Idx: PartialEq + Clone + Default> {
    head: Option<CountedTreeNode<T, Idx>>
}


impl<T, Idx: PartialEq + Clone> CountedTreeNode<T, Idx> {
    pub fn insert(&mut self, index: &[Idx], value: T) {
        if index.len() == 0 {
            self.value = Some(value);
            return;
        }
        self.idx_counts += 1;

        let key = index.first().unwrap();


        let next = match self.find_node_mut(key) {
            Some(n) => n,
            None => {
                let n = CountedTreeNode {
                    key: (*key).clone(),
                    value: None,
                    idx_counts: 0,
                    built_sub_nodes: 0,
                    visited: false,
                    list_offset: -1,
                    nodes: vec!()
                };

                self.nodes.push(n);

                self.nodes.last_mut().unwrap()
            }
        };

        next.insert(&index[1..], value);
    }

    pub fn sort_nodes(&mut self) {
        self.nodes.sort();
        for node in &mut self.nodes {
            node.sort_nodes();
        }
    }

    pub fn consume_next_node(&mut self) -> Option<CountedTreeNode<T, Idx>> {
        if self.nodes.len() == 0 {
            return None;
        } else {
            return Some(self.nodes.remove(0));
        }
    }




    /* Helper Nodes */
    #[allow(dead_code)] // Left here for future reference
    pub fn find_node(&self, key: &Idx) -> Option<&CountedTreeNode<T, Idx>> {
        for node in &self.nodes {
            if node.key == *key {
                return Some(node);
            }
        };

        return None;
    }
    pub fn find_node_mut(&mut self, key: &Idx) -> Option<&mut CountedTreeNode<T, Idx>> {
        for node in &mut self.nodes {
            if node.key == *key {
                return Some(node);
            }
        };

        return None;
    }
}
impl<T, Idx: PartialEq + Clone> PartialEq for CountedTreeNode<T, Idx> {
    fn eq(&self, other: &Self) -> bool {
        return self.idx_counts == other.idx_counts;
    }
}
impl<T, Idx: PartialEq + Clone> Eq for CountedTreeNode<T, Idx> { }
impl<T, Idx: PartialEq + Clone> PartialOrd for CountedTreeNode<T, Idx> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.idx_counts < other.idx_counts { // Backwards ordering so that the items are sorted in reverse
            return Some(std::cmp::Ordering::Greater);
        }
        else if self.idx_counts > other.idx_counts {
            return Some(std::cmp::Ordering::Less);
        };

        return Some(std::cmp::Ordering::Equal);
    }
}
impl<T, Idx: PartialEq + Clone> Ord for CountedTreeNode<T, Idx> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.idx_counts < other.idx_counts { // Backwards ordering
            return std::cmp::Ordering::Greater;
        }
        else if self.idx_counts > other.idx_counts {
            return std::cmp::Ordering::Less;
        };

        return std::cmp::Ordering::Equal;
    }
}


impl<T, Idx: PartialEq + Clone + Default> CountedTreeMap<T, Idx> {
    pub fn new() -> Self {
        CountedTreeMap {
            head: Some(CountedTreeNode {
                key: Idx::default(),
                value: None,
                built_sub_nodes: 0,
                idx_counts: 0,
                visited: false,
                list_offset: -1,
                nodes: vec!()
            })
        }
    }

    pub fn insert(&mut self, index: &[Idx], value: T) {
        if index.len() == 0 { return; }

        self.head.as_mut().unwrap().insert(index, value);
    }

    pub fn root(self) -> CountedTreeNode<T, Idx> {
        return self.head.unwrap();
    }
    pub fn root_ref_mut(&mut self) -> &mut CountedTreeNode<T, Idx> {
        return self.head.as_mut().unwrap();
    }
    pub fn root_ref(&self) -> &CountedTreeNode<T, Idx> {
        return self.head.as_ref().unwrap();
    }
}
impl<T, Idx: PartialEq + Clone + Default> Default for CountedTreeMap<T, Idx> {
    fn default() -> Self {
        CountedTreeMap {
            head: None
        }
    }
}




/*
Tests:
    - Insert
    - Sort
*/
#[cfg(test)]
mod tests {
    use super::*;

    /*
    Test: Insert

    Summary:
        Check that inserting values works correctly
     */
    #[test]
    fn insert() {
        let mut map: CountedTreeMap<i32, &str> = CountedTreeMap::new();

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
    Test: Sort

    Summary:
        Sorts the nodes. This should happen without panicing
        Go through a couple of nodes and check that they have the correct counts
     */
    #[test]
    fn sort() {
        let mut map: CountedTreeMap<i32, &str> = CountedTreeMap::new();
        map.insert(vec!["a", "b", "c"].as_slice(), 1);
        map.insert(vec!["a", "b", "d"].as_slice(), 2);
        map.insert(vec!["e", "f"].as_slice(), 3);

        // Sort nodes
        // Shouldn't panic
        map.root_ref_mut().sort_nodes();

        let root = map.root_ref();
        assert_eq!(root.idx_counts, 3);

        // Step down again into "a"
        let node = root.find_node(&"a").unwrap();
        assert_eq!(node.idx_counts, 2);

        // Step down into "e"
        let node = root.find_node(&"e").unwrap();
        assert_eq!(node.idx_counts, 1);
    }

}