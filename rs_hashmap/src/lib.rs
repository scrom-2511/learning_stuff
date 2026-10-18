use std::{
    hash::{DefaultHasher, Hash, Hasher},
    mem,
};

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            buckets: vec![],
            items: 0,
        }
    }
}

impl<K: Hash + Eq, V> HashMap<K, V> {
    fn bucket(key: &K, buckets_len: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        let bucket_index = hasher.finish() % buckets_len as u64;

        bucket_index as usize
    }

    fn resize(&mut self) {
        let current_size = self.buckets.len();

        let new_size = match current_size {
            0 => 1,
            _ => current_size * 2,
        };

        let mut new_buckets: Vec<Vec<(K, V)>> = (0..new_size).map(|_| Vec::new()).collect();

        for (k, v) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let bucket = Self::bucket(&k, new_buckets.len());

            new_buckets[bucket as usize].push((k, v));
        }

        self.buckets = new_buckets;
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.len() == 0 || (self.items + 1) * 4 > self.buckets.len() * 3 {
            self.resize();
        }

        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        let bucket = Self::bucket(&key, self.buckets.len());

        let bucket_to_insert = &mut self.buckets[bucket];

        for (k, v) in bucket_to_insert.iter_mut() {
            if key == *k {
                return Some(mem::replace(v, value));
            }
        }

        bucket_to_insert.push((key, value));
        self.items += 1;

        return None;
    }

    fn get(&self, key: &K) -> Option<&V> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        let bucket = hasher.finish() % self.buckets.len() as u64;
        let bucket = &self.buckets[bucket as usize];

        let val = bucket.iter().find(|(k, _)| k == key).map(|(_, v)| v);

        val
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let bucket_ind = Self::bucket(key, self.buckets.len());
        let bucket = &mut self.buckets[bucket_ind];
        let index = bucket.iter().position(|(k, _)| k == key)?;

        self.items -= 1;

        Some(bucket.swap_remove(index).1)
    }
}

#[test]

fn test_insert_get() {
    let mut hashmap = HashMap::new();
    hashmap.insert("foo", 42);

    let key = &"foo";

    let get_result = hashmap.get(key);
    assert_eq!(get_result, Some(&42));

    let remove_result = hashmap.remove(key);
    assert_eq!(remove_result, Some(42));
}
