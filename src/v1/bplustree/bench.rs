#[cfg(bench)]
mod tests {

    use super::super::bptree::BPlusTree;
    use test::Bencher;
    use std::collections::{HashMap,BTreeMap};

    #[bench]
    fn bench_bptree_get_for_1million_entries(b: &mut Bencher) {

        let mut bpt = BPlusTree::<u64,u64>::new(1024);
        let num = 1_000_000;
        for i in 0..num {
            let _ = bpt.insert( i, i*10);
        }

        let mut counter = 0;
        b.iter(|| {
            bpt.get(counter).unwrap();
            counter = (counter + 1) % num;
        });
    }

    #[bench]
    fn bench_bptree_insert_for_1million_entries(b: &mut Bencher) {

        let mut bpt = BPlusTree::<u64,u64>::new(1024);
        let num = 1_000_000;
        for i in 0..num {
            let _ = bpt.insert( i, i*10);
        }

        let mut counter = 0;
        b.iter(|| {
            let _ = bpt.insert(counter+num,counter+num);
            counter += 1;
        });
    }


    #[bench]
    fn bench_std_hashmap_get_for_1million_entries(b: &mut Bencher) {

        let mut hm = HashMap::<u64,u64>::new();
        let num = 1_000_000;
        for i in 0..num {
            let _ = hm.insert( i, i*10);
        }

        let mut counter = 0;
        b.iter(|| {
            hm.get(&counter).unwrap();
            counter = (counter + 1) % num;
        });
    }

    #[bench]
    fn bench_std_hashmap_insert_for_1million_entries(b: &mut Bencher) {

        let mut hm = HashMap::<u64,u64>::new();
        let num = 1_000_000;
        for i in 0..num {
            let _ = hm.insert( i, i*10);
        }

        let mut counter = 0;
        b.iter(|| {
            let _ = hm.insert(counter+num,counter+num);
            counter += 1;
        });
    }

    #[bench]
    fn bench_std_btreemap_get_for_1million_entries(b: &mut Bencher) {

        let mut hm = BTreeMap::<u64,u64>::new();
        let num = 1_000_000;
        for i in 0..num {
            let _ = hm.insert( i, i*10);
        }

        let mut counter = 0;
        b.iter(|| {
            hm.get(&counter).unwrap();
            counter = (counter + 1) % num;
        });
    }

    #[bench]
    fn bench_std_btreemap_insert_for_1million_entries(b: &mut Bencher) {

        let mut hm = BTreeMap::<u64,u64>::new();
        let num = 1_000_000;
        for i in 0..num {
            let _ = hm.insert( i, i*10);
        }

        let mut counter = 0;
        b.iter(|| {
            let _ = hm.insert(counter+num,counter+num);
            counter += 1;
        });
    }

}