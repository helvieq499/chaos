use std::{collections::HashMap, hash::Hash, sync::RwLock};

pub trait CollectionExt<T, U> {
    fn get(&self, key: T) -> Option<U>;
    fn set(&self, key: T, value: U);
}

impl<T, U> CollectionExt<T, U> for RwLock<HashMap<T, U>>
where
    T: Eq + Hash,
    U: Clone,
{
    fn get(&self, key: T) -> Option<U> {
        let lock = self.read().expect("unpoisoned");
        lock.get(&key).cloned()
    }

    fn set(&self, key: T, value: U) {
        let mut lock = self.write().expect("unpoisoned");
        lock.insert(key, value);
    }
}
