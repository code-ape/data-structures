
use super::Void;
use std::fmt::Debug;
use std::ptr;
use std::mem;

#[derive(Debug)]
pub struct Leaf<K,V> where K: Ord + Clone + Debug, V: Debug {
    pub items: Vec<(K,V)>,
    pub next_node: *mut Leaf<K,V>
}

#[derive(Debug)]
pub enum InsertResult<K,V> where K: Ord + Clone + Debug, V: Debug {
    Ok,
    Overwrite(V),
    Split(K,*mut Leaf<K,V>)
}

impl<K:Ord+Clone+Debug,V:Debug> Leaf<K,V> {
    pub fn new_on_heap(capacity: usize) -> Box<Self> {
        Box::new(Leaf {
            items: Vec::with_capacity(capacity),
            next_node: ptr::null_mut()
        })
    }

    pub fn key_index(&self, key: &K) -> Result<usize,usize> {
        self.items.binary_search_by_key(key, |&(ref a,_)| a.clone())
    }

    pub fn insert_from_access_path(&mut self, access_path_index: Result<usize,usize>, data: (K,V))
        -> InsertResult<K,V>
    {
        debug!("insert_from_access_path({:?},{:?})", access_path_index, data);
        match (access_path_index, self.items.capacity() - self.items.len()) {
            // Swap with existing item.
            (Ok(index),_) => {
                let prior_items = self.items.get_mut(index).unwrap();
                // Swap prior_items and data.
                let old_value = mem::replace(prior_items,data).1;
                // Return old value.
                InsertResult::Overwrite(old_value)
            },
            // Need to split vector.
            // size = 8 & insert_lower => split 3&5
            // size = 8 & insert_upper => split 4&4
            // size = 7 & insert_lower => split 3&4
            // size = 7 & insert_upper => split 4&3
            (Err(index),0) => {
                let mut split_index = (self.items.capacity() - 1) / 2;
                let new_vec = match index > split_index {
                    true => {
                        split_index += 1;
                        let mut v = self.items.split_off(split_index);
                        let additional_reserve = self.items.capacity() - v.capacity();
                        v.reserve_exact(additional_reserve);
                        assert_eq!(v.capacity(), self.items.capacity());
                        v.insert(index - split_index, data);
                        v
                    },
                    false => {
                        let mut v = self.items.split_off(split_index);
                        let additional_reserve = self.items.capacity() - v.capacity();
                        v.reserve_exact(additional_reserve);
                        assert_eq!(v.capacity(), self.items.capacity());
                        self.items.insert(index, data);
                        v
                    }
                };
                debug!("self.items = {:?}", self.items);
                debug!("new_vec = {:?}", new_vec);

                let split_edge_key = new_vec.get(0).unwrap().0.clone();

                let new_leaf_ptr = Box::into_raw(Box::new(Leaf {
                    items: new_vec,
                    next_node: self.next_node
                }));
                debug!("new_leaf_ptr = {:?}", new_leaf_ptr);
                self.next_node = new_leaf_ptr;
                InsertResult::Split(split_edge_key, new_leaf_ptr)
            },
            // Just insert at index
            (Err(index),_) => {
                self.items.insert(index,data);
                InsertResult::Ok
            }

        }
    }
}

pub enum BranchInsertResult<K> where K: Ord + Clone + Debug {
    Ok,
    Overwrite(K),
    Split(K,*mut Branch<K>)
}

#[derive(Debug)]
pub struct Branch<K:Ord+Clone+Debug> {
    pub head:*mut Void,
    pub body: Vec<(K,*mut Void)>
}

impl<K:Ord+Clone+Debug> Branch<K> {
    pub fn new(node_capacity: usize, head: *mut Void, tail: *mut Void, key: K) -> Box<Self> {
        Box::new(Branch {
            head: head,
            body: {
                let mut v = Vec::with_capacity(node_capacity);
                v.push((key,tail));
                v
            }
        })
    }

    /// Given
    /// - head = ptr1
    /// - body = [(5,ptr2), (10,ptr3)]
    ///
    /// Lookup1, key = 2
    /// - index = Err(0) (head, miss)
    /// Lookup2, key = 5
    /// - index = Ok(1) (body[0], hit)
    /// Lookup3, key = 7
    /// - index = Err(1) (body[0], miss)
    /// Lookup4, key = 10
    /// - index = Ok(2) (body[1], hit)
    /// Lookup5, key = 12
    /// - index = Err(2) (body[1], miss)
    ///
    /// Err = below key val
    /// Ok = hit key val
    pub fn branch_index(&self, key: &K) -> Result<usize,usize> {
        match self.body.binary_search_by_key(key, |&(ref a,_)| a.clone()) {
            Ok(index) => Ok(index+1),
            Err(index) => Err(index)
        }
    }

    pub fn branch_ptr_from_index_result(&self, index_result: Result<usize,usize>) -> *mut Void {
        match index_result {
            Err(0) => self.head,
            Ok(index) => self.body.get(index-1).unwrap().1,
            Err(index) => self.body.get(index-1).unwrap().1
        }
    }

    pub fn insert_from_access_path(&mut self, index_result: Result<usize,usize>, data: (K,*mut Void))
        -> BranchInsertResult<K>
    {
        debug!("Branch::insert_from_access_path: starting ...");
        debug!("self.body.capacity = {:?}", self.body.capacity());
        debug!("self.body.len = {:?}", self.body.len());
        
        match self.body.capacity() - self.body.len() {
            // must split
            0 => {
                let result_index = match index_result {
                    Err(index) => index,
                    Ok(index) => index
                };

                let mut split_index = ((self.body.capacity() - 1) / 2) + 1;
                let new_vec = match result_index > split_index {
                    true => {
                        split_index += 1;
                        let mut v = self.body.split_off(split_index);
                        let additional_reserve = self.body.capacity() - v.capacity();
                        v.reserve_exact(additional_reserve);
                        assert_eq!(v.capacity(), self.body.capacity());
                        v.insert(result_index - split_index, data);
                        v
                    },
                    false => {
                        let mut v = self.body.split_off(split_index);
                        let additional_reserve = self.body.capacity() - v.capacity();
                        v.reserve_exact(additional_reserve);
                        assert_eq!(v.capacity(), self.body.capacity());
                        self.body.insert(result_index, data);
                        v
                    }
                };
                let (split_edge_key,new_head) = self.body.pop().unwrap();
                 
                debug!("self.body = {:?}", self.body);
                debug!("new_vec = {:?}", new_vec);

                //let split_edge_key = new_vec.get(0).unwrap().0.clone();

                let new_branch_ptr = Box::into_raw(Box::new(Branch {
                    head: new_head,
                    body: new_vec
                }));
                debug!("new_branch_ptr = {:?}", new_branch_ptr);

                BranchInsertResult::Split(split_edge_key, new_branch_ptr)
            },
            // just insert
            _ => {
                // Extract index
                let insert_index = match index_result {
                    Err(index) => index,
                    Ok(index) => index
                };
                // do insert
                self.body.insert(insert_index,data);
                debug!("Branch post insert = {:?}", self);
                BranchInsertResult::Ok
            }
        }
    }
}