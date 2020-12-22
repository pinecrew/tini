use std::borrow::Borrow;
use std::collections::hash_map::{self, Entry};
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::IntoIterator;

#[derive(Debug)]
pub struct OrderedHashMap<K, V> {
    base: HashMap<K, V>,
    order: Vec<K>,
}

pub struct Iter<'a, K, V> {
    base: &'a HashMap<K, V>,
    order_iterator: std::slice::Iter<'a, K>,
}

pub type IterMut<'a, K, V> = hash_map::IterMut<'a, K, V>;

impl<'a, K, V> IntoIterator for &'a OrderedHashMap<K, V>
where
    K: Eq + Hash,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { base: &self.base, order_iterator: self.order.iter() }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V>
where
    K: Eq + Hash,
{
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        match self.order_iterator.next() {
            Some(k) => self.base.get_key_value(&k),
            None => None,
        }
    }
}

impl<K, V> Default for OrderedHashMap<K, V>
where
    K: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> OrderedHashMap<K, V>
where
    K: Eq + Hash + Clone,
{
    pub fn new() -> OrderedHashMap<K, V> {
        OrderedHashMap { base: HashMap::<K, V>::new(), order: Vec::<K>::new() }
    }

    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.base.get(k)
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        if !self.base.contains_key(&k) {
            self.order.push(k.clone());
        }
        self.base.insert(k, v)
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter { base: &self.base, order_iterator: self.order.iter() }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        self.base.iter_mut()
    }

    pub fn keys(&self) -> std::slice::Iter<K> {
        self.order.iter()
    }

    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        if !self.base.contains_key(&key) {
            self.order.push(key.clone());
        }
        self.base.entry(key)
    }
}
