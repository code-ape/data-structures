
use super::Void;
use std::fmt::Debug;
use std::ptr;
use std::mem;
use super::node;
use std::marker;

pub struct BPlusTree<K,V> where K: Ord + Clone + Debug, V: Debug {
    root: *mut Void,
    pub node_capacity: usize,
    pub tree_depth: usize,
    pub item_count: usize,
    types: marker::PhantomData<(K,V)>
}

impl<K:Ord+Clone+Debug,V:Debug> BPlusTree<K,V> {
    pub fn new(node_capacity: usize) -> Self {
        BPlusTree {
            root: ptr::null_mut(),
            node_capacity: node_capacity,
            tree_depth: 0,
            item_count: 0,
            types: marker::PhantomData
        }
    }

    fn first_leaf<'a>(&'a self) -> &'a node::Leaf<K,V> {
        let mut _current_depth = 1usize;
        let mut current_node = self.root.clone();

        loop {
            match self.tree_depth.checked_sub(_current_depth) {
                None => {
                    debug!("No nodes in tree!");
                    break;
                },
                Some(0) => {
                    break;
                },
                _ => {
                    let branch: &node::Branch<K> = unsafe { void_ptr_to_ref!(current_node) };
                    current_node = branch.head;
                }
            }
            _current_depth += 1;
        }

        let leaf: &node::Leaf<K,V> = unsafe { void_ptr_to_ref!(current_node) };
        return leaf;
    }


    fn access_path(&self, key: &K) -> Vec<(*mut Void,Result<usize,usize>)> {
        let mut _current_depth = 1usize;
        let mut current_node = self.root.clone();

        let mut ret_val = Vec::with_capacity(self.node_capacity);

        loop {
            match self.tree_depth.checked_sub(_current_depth) {
                None => {
                    debug!("No nodes in tree!");
                    break;
                },
                Some(0) => {
                    debug!("Leaf root!");
                    let leaf: &node::Leaf<K,V> = unsafe { void_ptr_to_ref!(current_node) };
                    let index_result = leaf.key_index(key);
                    ret_val.push((current_node,index_result));
                    break;
                },
                _ => {
                    debug!("Branch node!");
                    let branch: &node::Branch<K> = unsafe { void_ptr_to_ref!(current_node) };
                    let index_result = branch.branch_index(key);
                    ret_val.push((current_node,index_result));
                    current_node = branch.branch_ptr_from_index_result(index_result);
                }
            }
            _current_depth += 1;
        }

        return ret_val;
        
    }


    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        debug!("Inserting key/value of {:?}", (&key,&value));

        let mut node_access_path = self.access_path(&key);
        debug!("BPlusTree::insert: node_access_path = {:?}", node_access_path);

        let insert_result_option: Option<node::InsertResult<K,V>> = match node_access_path.pop() {
            None => {
                debug!("Allocating initial root node!");
                let mut new_root = node::Leaf::new_on_heap(self.node_capacity);
                debug!("Writing first item");
                new_root.items.push((key, value));
                self.root = unsafe { mem::transmute(Box::into_raw(new_root)) };
                self.tree_depth = 1;
                self.item_count = 1;

                None
            },
            Some((leaf_ptr,index_result)) => {
                let leaf: &mut node::Leaf<K,V> = unsafe { void_ptr_to_mut_ref!(leaf_ptr) };

                Some(leaf.insert_from_access_path(index_result, (key,value)))
            }
        };

        if insert_result_option.is_none() {
            return None;
        }

        let insert_result = insert_result_option.unwrap();
        
        debug!("node_access_path = {:?}", node_access_path);

        debug!("Handling leaf insert result of: {:?}", insert_result);
        match insert_result {
            node::InsertResult::Split(edge_key,leaf_ptr) => {
                self.item_count += 1;

                let mut remaining_node = unsafe { mut_ptr_to_void_ptr!(leaf_ptr) };
                let mut remaining_edge_key = edge_key;

                debug!("remaining_node = {:?}", remaining_node);

                for (node,node_index) in node_access_path.iter().rev() {
                    debug!("node (ptr) = {:?}", node);
                    let branch: &mut node::Branch<K> = unsafe { void_ptr_to_mut_ref!(*node) };
                    debug!("branch = {:?}", branch);
                    let branch_insert_result = branch.insert_from_access_path(
                        *node_index, (remaining_edge_key.clone(),remaining_node)
                    );
                    match branch_insert_result {
                        node::BranchInsertResult::Ok => {
                            remaining_node = ptr::null_mut();
                            break;
                        },
                        node::BranchInsertResult::Split(split_edge_key, new_branch_ptr) => {
                            remaining_edge_key = split_edge_key;
                            remaining_node = unsafe { mut_ptr_to_void_ptr!(new_branch_ptr) };
                        },
                        _ => unimplemented!("AAAHHHHH!!!!")
                    }
                }

                if !remaining_node.is_null() {
                    let new_branch = node::Branch::new(
                        self.node_capacity, self.root, remaining_node, remaining_edge_key
                    );
                    self.root = unsafe { mem::transmute(Box::into_raw(new_branch)) };
                    self.tree_depth += 1;
                }

                None
            },
            node::InsertResult::Ok => {
                self.item_count += 1;
                None
            },
            node::InsertResult::Overwrite(old_val) => Some(old_val)
        }
    }

    pub fn get<'a>(&'a self, key: K) -> Option<&'a V> {
        debug!("Getting value for {:?}", key);
        let mut node_access_path = self.access_path(&key);

        if node_access_path.len() == 0 {
            None
        } else {
            let (leaf_ptr,leaf_index_result) = node_access_path.pop().unwrap();

            match leaf_index_result {
                Err(_) => None,
                Ok(node_index) => {
                    let leaf: &node::Leaf<K,V> = unsafe { void_ptr_to_ref!(leaf_ptr) };
                    leaf.items
                        .get(node_index)
                        .map(|x| { debug!("item = {:?}", x); x })
                        .map(|x| &(x.1))
                }
            }
        }
    }

    pub fn delete(&self, key: K) -> Option<V> {
        unimplemented!("Don't know how to delete yet!");
    }
}

pub struct BPlusTreeIterator<'a,K:'a,V:'a> where K: Ord + Clone + Debug, V: Debug {
    bpt: &'a BPlusTree<K,V>,
    current_leaf: &'a node::Leaf<K,V>,
    leaf_index: usize
}


impl<'a,K:Ord+Clone+Debug,V:Debug> IntoIterator for &'a BPlusTree<K,V> {
    type Item = &'a (K,V);
    type IntoIter = BPlusTreeIterator<'a,K,V>;

    fn into_iter(self) -> Self::IntoIter {
        let first_leaf = self.first_leaf();
        debug!("first_leaf = {:?}", first_leaf);
        BPlusTreeIterator { bpt: self, current_leaf: self.first_leaf(), leaf_index: 0 }
    }
}

impl<'a,K:Ord+Clone+Debug,V:Debug> Iterator for BPlusTreeIterator<'a,K,V> {
    type Item = &'a (K,V);
    fn next(&mut self) -> Option<&'a (K,V)> {

        let result: Option<&(K,V)> = match self.current_leaf.items.get(self.leaf_index) {
            Some(x) => Some(x),
            None => {
                self.leaf_index = 0;
                if self.current_leaf.next_node.is_null() {
                    debug!("Next node is null!");
                    None
                } else {
                    debug!("Hopping to node at {:?}", self.current_leaf.next_node);
                    self.current_leaf = unsafe { &*(self.current_leaf.next_node) };
                    Some(self.current_leaf.items.get(self.leaf_index).unwrap())
                }
            }
        };

        self.leaf_index += 1;
        return result;
    }
}
