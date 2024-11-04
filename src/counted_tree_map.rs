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
    pub node_offset: i32,

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
                    node_offset: -1,
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
        return self.nodes.pop();
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
        if self.idx_counts > other.idx_counts { // Backwards ordering so that the items are sorted in reverse
            return Some(std::cmp::Ordering::Greater);
        }
        else if self.idx_counts < other.idx_counts {
            return Some(std::cmp::Ordering::Less);
        };

        return Some(std::cmp::Ordering::Equal);
    }
}
impl<T, Idx: PartialEq + Clone> Ord for CountedTreeNode<T, Idx> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.idx_counts > other.idx_counts { // Backwards ordering
            return std::cmp::Ordering::Greater;
        }
        else if self.idx_counts < other.idx_counts {
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
                node_offset: -1,
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
