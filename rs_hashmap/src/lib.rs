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
    fn resize(&mut self) {
        let current_size = self.buckets.len();

        let new_size = match current_size {
            0 => 1,
            _ => current_size * 2,
        };

        let mut new_buckets: Vec<Vec<(K, V)>> = (0..new_size).map(|_| Vec::new()).collect();

        for (k, v) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            k.hash(&mut hasher);

            let bucket_index = hasher.finish() % new_buckets.len() as u64;

            new_buckets[bucket_index as usize].push((k, v));
        }

        self.buckets = new_buckets;
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.len() == 0 {
            self.resize();
        }

        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        let bucket = hasher.finish() % self.buckets.len() as u64;

        let bucket_to_insert = &mut self.buckets[bucket as usize];

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
}

#[test]

fn test_insert_get() {
    let mut hashmap = HashMap::new();
    hashmap.insert("foo", 42);

    let result = hashmap.get(&"foo");
    assert_eq!(result, Some(&42));
}
