// A HashMap implementation that uses the first 32 bits added together as a hash

use std::hash::{Hash, Hasher};

pub struct HazMap<K: Hash, V> {
    key: std::marker::PhantomData<K>,
    data: Vec<(u32, V)>,
}

impl<K: Hash, V> HazMap<K, V> {
    pub fn new() -> Self {
        HazMap {
            data: Vec::new(),
            key: std::marker::PhantomData
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let key = get_first_u32(&key);

        for (k, v) in &mut self.data {
            if k == &key {
                *v = value;
                return;
            }
        }

        self.data.push((key, value));
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let key = get_first_u32(key);

        for (k, v) in &self.data {
            if *k == key {
                return Some(v);
            }
        }

        None
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let key = get_first_u32(key);

        for (k, v) in &mut self.data {
            if *k == key {
                return Some(v);
            }
        }

        None
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let key = get_first_u32(key);

        for i in 0..self.data.len() {
            if self.data[i].0 == key {
                return Some(self.data.remove(i).1);
            }
        }

        None
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

// Get the first u32 of a hashable key
// If it does not have 4 bytes of data, pad it with 0s
fn get_first_u32<K: Hash>(key: &K) -> u32 {
    let mut hasher = Masher::default();
    key.hash(&mut hasher);
    let hash = hasher.finish();

    let mut bytes = [0u8; 4];

    for i in 0..4 {
        bytes[i] = (hash >> (i * 8)) as u8;
    }

    u32::from_le_bytes(bytes)
}

#[derive(Default)]
struct Masher(Vec<u8>);

impl std::hash::Hasher for Masher {
    fn write(&mut self, v: &[u8]) {
        self.0.extend(v);
    }

    fn finish(&self) -> u64 {
        let out: [u8; 4] = std::array::from_fn(|n| self.0.get(n).copied().unwrap_or(0));
        u32::from_ne_bytes(out).count_ones().into()
    }
}
