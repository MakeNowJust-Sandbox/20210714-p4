const fn med(min: u64, max: u64) -> u64 {
    (min + max) / 2
}

const fn has_factor(n: u64, min: u64, max: u64) -> bool {
    if min * min > n {
        false
    } else if min + 1 >= max {
        n % min == 0
    } else {
        has_factor(n, min, med(min, max)) || has_factor(n, med(min, max), max)
    }
}

pub const fn next_prime(n: u64) -> u64 {
    if has_factor(n, 2, n) {
        next_prime(n + 1)
    } else {
        n
    }
}

#[derive(Debug)]
pub struct TranspositionTable<K, V> {
    pub keys: Box<[K]>,
    pub values: Box<[V]>,
}

impl<K, V> TranspositionTable<K, V> {
    pub fn capacity(&self) -> u64 {
        self.keys.len() as u64
    }
}

impl<K: FromKey, V> TranspositionTable<K, V> {
    fn index(&self, key: u64) -> usize {
        (key % self.capacity()) as usize
    }

    pub fn insert(&mut self, key: u64, value: V) {
        let index = self.index(key);
        unsafe {
            *self.keys.get_unchecked_mut(index) = key.into_key();
            *self.values.get_unchecked_mut(index) = value;
        }
    }
}

impl<K: Clone + Default, V: Clone + Default> TranspositionTable<K, V> {
    pub fn new<const N: usize>() -> TranspositionTable<K, V> {
        let keys = vec![K::default(); N].into_boxed_slice();
        let values = vec![V::default(); N].into_boxed_slice();
        TranspositionTable {
            keys: keys,
            values: values,
        }
    }
}

impl<K: FromKey + Eq, V: Clone + Default> TranspositionTable<K, V> {
    pub fn get(&self, key: u64) -> V {
        let index = self.index(key);
        unsafe {
            if *self.keys.get_unchecked(index) == key.into_key() {
                self.values.get_unchecked(index).clone()
            } else {
                V::default()
            }
        }
    }
}

pub trait FromKey {
    fn from_key(key: u64) -> Self;
}

pub trait IntoKey<K> {
    fn into_key(self) -> K;
}

impl<K: FromKey> IntoKey<K> for u64 {
    fn into_key(self) -> K {
        FromKey::from_key(self)
    }
}

impl FromKey for u8 {
    fn from_key(key: u64) -> Self {
        (key & 0b11111111) as u8
    }
}

impl FromKey for u64 {
    fn from_key(key: u64) -> Self {
        key
    }
}
