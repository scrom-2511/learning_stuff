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

impl<K, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Hash + Eq, V> HashMap<K, V> {
    pub fn is_empty(&self) -> bool {
        self.items == 0
    }

    pub fn len(&self) -> usize {
        self.items
    }

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

            new_buckets[bucket].push((k, v));
        }

        self.buckets = new_buckets;
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || (self.items + 1) * 4 > self.buckets.len() * 3 {
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

        None
    }

    fn get(&self, key: &K) -> Option<&V> {
        let bucket = Self::bucket(key, self.buckets.len());
        let bucket = &self.buckets[bucket as usize];

        bucket.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    fn contains_key(&self, key: &K) -> bool {
        let val = self.get(key);

        if val.is_some() {
            return true;
        }

        false
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
    let mut hashmap = HashMap::default();
    assert_eq!(hashmap.len(), 0);
    assert_eq!(hashmap.is_empty(), true);

    hashmap.insert("foo", 42);
    assert_eq!(hashmap.len(), 1);
    assert_eq!(hashmap.is_empty(), false);

    let key = &"foo";

    let get_result = hashmap.get(key);
    assert_eq!(get_result, Some(&42));

    let remove_result = hashmap.remove(key);
    assert_eq!(remove_result, Some(42));

    assert_eq!(hashmap.len(), 0);
    assert_eq!(hashmap.is_empty(), true);

    let remove_result = hashmap.remove(key);
    assert_eq!(remove_result, None);
}
