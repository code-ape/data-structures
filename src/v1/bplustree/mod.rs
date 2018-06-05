#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

#[macro_use]
mod macros;
mod node;
mod bptree;
mod bench;

pub enum Void {}

///! Types of nodes:
///! - Leaf
///! - Branch


#[cfg(test)]
mod tests {

    use super::bptree::BPlusTree;
    use rand::{Rng, SeedableRng, StdRng};

    const RAND_SEED: [u8;32] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
    ];

    #[test]
    fn it_works1() {
        let mut bpt = BPlusTree::<u64,u64>::new(8);
        assert_eq!(bpt.node_capacity,8);
        assert_eq!(bpt.tree_depth,0);
        assert_eq!(bpt.item_count,0);
    }

    #[test]
    fn it_works2() {
        let mut bpt = BPlusTree::<u64,u64>::new(8);

        bpt.insert(1,1);
        bpt.insert(2,2);

        assert_eq!(bpt.node_capacity,8);
        assert_eq!(bpt.tree_depth,1);
        assert_eq!(bpt.item_count,2);
    }

    #[test]
    fn insert_multiple() {
        let mut bpt = BPlusTree::<u64,u64>::new(8);

        assert_eq!(bpt.get(1), None);
        bpt.insert(1,1);
        bpt.insert(2,2);
        assert_eq!(bpt.get(1), Some(&1));
        assert_eq!(bpt.get(2), Some(&2));
        assert_eq!(bpt.get(3), None);

        assert_eq!(bpt.node_capacity,8);
        assert_eq!(bpt.tree_depth,1);
        assert_eq!(bpt.item_count,2);
    }


    #[test]
    fn update_single_item() {
        let mut bpt = BPlusTree::<u64,u64>::new(8);

        assert_eq!(bpt.get(1), None);
        let old_val1 = bpt.insert(1,10);
        assert_eq!(old_val1, None);
        assert_eq!(bpt.get(1), Some(&10));

        let old_val2 = bpt.insert(1,20);
        assert_eq!(old_val2, Some(10));
        assert_eq!(bpt.get(1), Some(&20));

        assert_eq!(bpt.node_capacity,8);
        assert_eq!(bpt.tree_depth,1);
        assert_eq!(bpt.item_count,1);
    }

    #[test]
    fn split_first_leaf() {
        let mut bpt = BPlusTree::<u64,u64>::new(8);

        for i in 0..9 {
            let _ = bpt.insert( i, i*10);
        }
        
        for i in 0..9 {
            assert_eq!(bpt.get( i), Some(&(i*10)));
        }

        assert_eq!(bpt.node_capacity,8);
        assert_eq!(bpt.tree_depth,2);
        assert_eq!(bpt.item_count,9);
    }

    #[test]
    fn split_two_leafs_but_no_branches() {
        let mut bpt = BPlusTree::<u64,u64>::new(8);

        let num = 16;

        for i in 0..num {
            println!("");
            let _ = bpt.insert( i, i*10);
        }
        
        for i in 0..num {
            println!("");
            assert_eq!(bpt.get( i), Some(&(i*10)));
        }

        assert_eq!(bpt.node_capacity,8);
        assert_eq!(bpt.tree_depth,2);
        assert_eq!(bpt.item_count, num as usize);
    }

    #[test]
    fn split_root_branch() {
        let mut bpt = BPlusTree::<u64,u64>::new(8);

        let num = 41;

        for i in 0..num {
            println!("");
            let _ = bpt.insert( i, i*10);
        }
        
        for i in 0..num {
            println!("");
            assert_eq!(bpt.get( i), Some(&(i*10)));
        }

        assert_eq!(bpt.node_capacity,8);
        assert_eq!(bpt.tree_depth,3);
        assert_eq!(bpt.item_count, num as usize);
    }

    #[test]
    fn insert_and_get_10_000_seq_entries() {
        let mut bpt = BPlusTree::<u64,u64>::new(1024);

        let num = 10_000;

        for i in 0..num {
            let _ = bpt.insert( i, i*10);
        }
        
        for i in 0..num {
            assert_eq!(bpt.get(i), Some(&(i*10)));
        }

        assert_eq!(
            (bpt.node_capacity, bpt.tree_depth, bpt.item_count),
            (1024, 2, num as usize)
        );
    }

    use std::collections::BTreeMap;
    #[test]
    fn insert_and_get_10_000_rand_entries() {
        
        let mut bpt = BPlusTree::<u8,u8>::new(4);

        let mut rng: StdRng = SeedableRng::from_seed(RAND_SEED);

        let num = 10_000;

        let mut action_record = BTreeMap::<u8,u8>::new();

        for i in 0..num {
            println!("");
            let data = rng.gen::<(u8,u8)>();
            action_record.insert(data.0, data.1);
            let _ = bpt.insert(data.0, data.1);
        }
        
        println!("\nPrinting out BPT items in their order:");
        for item in &bpt {
            println!("{:?}",item);
        }
        println!("");
        
        for action in action_record {
            println!("");
            assert_eq!(bpt.get(action.0), Some(&(action.1)));
        }
    }

    //#[test]
    //fn rand_access_for_10_000_operations() {
    //    
    //    let mut bpt = BPlusTree::<u8,u8>::new(4);
//
    //    let mut rng: StdRng = SeedableRng::from_seed(RAND_SEED);
//
    //    let num = 10_000;
//
    //    let mut action_record = BTreeMap::<u8,u8>::new();
//
    //    for i in 0..num {
    //        println!("");
    //        let data = rng.gen::<(u8,u8)>();
    //        action_record.insert(data.0, data.1);
    //        let _ = bpt.insert(data.0, data.1);
    //    }
    //    
    //    println!("\nPrinting out BPT items in their order:");
    //    for item in &bpt {
    //        println!("{:?}",item);
    //    }
    //    println!("");
    //    
    //    for action in action_record {
    //        println!("");
    //        assert_eq!(bpt.get(action.0), Some(&(action.1)));
    //    }
    //}

}
