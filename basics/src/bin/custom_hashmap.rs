fn main() {
    let mut m = custom_hashmap::MyHashMap::new(5);
    m.insert(0, 10);
    let val_ref = m.entry(2).or_insert(200);
    *val_ref = 30;
    dbg!(m);
}

mod custom_hashmap {
    // This is an example implementation of a simple hashmap
    // that is backed a vector, with vector indices as keys.
    #[derive(Debug)]
    pub(super) struct MyHashMap {
        base: Vec<i32>,
    }

    impl MyHashMap {
        pub(super) fn new(size: usize) -> MyHashMap {
            MyHashMap {
                base: vec![0; size],
            }
        }

        pub(crate) fn insert(&mut self, k: usize, v: i32) {
            self.base[k] = v;
        }

        pub(crate) fn entry(&mut self, k: usize) -> MyHashMapEntry {
            match self.base.get(k) {
                Some(_) => MyHashMapEntry::occupied(k, self),
                None => MyHashMapEntry::vacant(k, self),
            }
        }
    }

    pub(crate) struct MyHashMapEntry<'a> {
        k: usize,
        m: &'a mut MyHashMap,
    }

    impl<'a> MyHashMapEntry<'a> {
        fn vacant(k: usize, m: &'a mut MyHashMap) -> MyHashMapEntry<'a> {
            MyHashMapEntry { k, m }
        }

        fn occupied(k: usize, m: &'a mut MyHashMap) -> MyHashMapEntry<'a> {
            MyHashMapEntry { k, m }
        }

        pub(crate) fn or_insert(self, v: i32) -> &'a mut i32 {
            self.m.insert(self.k, v);
            self.m.base.get_mut(self.k).unwrap()
        }
    }
}
