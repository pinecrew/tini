//! Ordered Hashmap
//!
//! Needs to iterate over items in predictable order
//! e.g. for save ini sections and items in the same order as loaded or added

use std::borrow::Borrow;
use std::collections::hash_map::{self, Entry};
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;
use std::iter::IntoIterator;

/// Ordered hashmap built on top of std::collections::HashMap
/// Keys are stored in the field `keys` in the order they were added
#[derive(Debug)]
pub struct OrderedHashMap<K, V> {
    #[doc(hidden)]
    base: HashMap<K, V>,
    keys: Vec<K>,
}

impl<K, V> OrderedHashMap<K, V>
where
    K: Eq + Hash + Clone,
{
    /// Creates an empty `OrderedHashMap`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut map: OrderedHashMap<&str, i32> = HashMap::new();
    /// ```
    pub fn new() -> OrderedHashMap<K, V> {
        OrderedHashMap { base: HashMap::<K, V>::new(), keys: Vec::<K>::new() }
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut map = OrderedHashMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.get(&1), Some(&"a"));
    /// assert_eq!(map.get(&2), None);
    /// ```
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.base.get(k)
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ordered_hashmap::OrderedHashMap;
    ///
    /// let mut map = OrderedHashMap::new();
    /// map.insert(1, "a");
    /// if let Some(x) = map.get_mut(&1) {
    ///     *x = "b";
    /// }
    /// assert_eq!(map[&1], "b");
    /// ```
    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.base.get_mut(k)
    }


    pub fn contains_key<Q: ?Sized>(& self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.base.contains_key(k)
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut map = OrderedHashMap::new();
    /// assert_eq!(map.insert(37, "a"), None);
    /// assert_eq!(map.is_empty(), false);
    ///
    /// map.insert(37, "b");
    /// assert_eq!(map.insert(37, "c"), Some("b"));
    /// assert_eq!(map[&37], "c");
    /// ```
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        if !self.base.contains_key(&k) {
            self.keys.push(k.clone());
        }
        self.base.insert(k, v)
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut map = OrderedHashMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.remove(&1), Some("a"));
    /// assert_eq!(map.remove(&1), None);
    /// ```
    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q> + PartialEq<Q>,
        Q: Hash + Eq,
    {
        match self.keys.iter().position(|x| x == k) {
            Some(index) => {
                self.keys.swap_remove(index);
                self.base.remove(k)
            }
            None => None,
        }
    }

    /// An iterator visiting all key-value pairs in the order they were added.
    /// The iterator element type is `(&'a K, &'a V)`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut map = OrderedHashMap::new();
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// for (key, val) in map.iter() {
    ///     println!("key: {} val: {}", key, val);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter { base: &self.base, keys_iterator: self.keys.iter() }
    }

    /// An iterator visiting all key-value pairs in the order they were added,
    /// with mutable references to the values.
    /// The iterator element type is `(&'a K, &'a mut V)`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut map = OrderedHashMap::new();
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// // Update all values
    /// for (_, val) in map.iter_mut() {
    ///     *val *= 2;
    /// }
    ///
    /// for (key, val) in &map {
    ///     println!("key: {} val: {}", key, val);
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        self.base.iter_mut()
    }

    /// An iterator visiting all keys in the order they were added.
    /// The iterator element type is `&'a K`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut map = OrderedHashMap::new();
    /// map.insert("a", 1);
    /// map.insert("b", 2);
    /// map.insert("c", 3);
    ///
    /// for key in map.keys() {
    ///     println!("{}", key);
    /// }
    /// ```
    pub fn keys(&self) -> std::slice::Iter<K> {
        self.keys.iter()
    }

    /// Gets the given key's corresponding entry in the map for in-place manipulation.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let mut letters = OrderedHashMap::new();
    ///
    /// for ch in "a short treatise on fungi".chars() {
    ///     let counter = letters.entry(ch).or_insert(0);
    ///     *counter += 1;
    /// }
    ///
    /// assert_eq!(letters[&'s'], 2);
    /// assert_eq!(letters[&'t'], 3);
    /// assert_eq!(letters[&'u'], 1);
    /// assert_eq!(letters.get(&'y'), None);
    /// ```
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        if !self.base.contains_key(&key) {
            self.keys.push(key.clone());
        }
        self.base.entry(key)
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

impl<'a, K, V> IntoIterator for &'a OrderedHashMap<K, V>
where
    K: Eq + Hash,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { base: &self.base, keys_iterator: self.keys.iter() }
    }
}

impl<K, V> IntoIterator for OrderedHashMap<K, V>
where
    K: Eq + Hash,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { base: self.base, keys_iterator: self.keys.into_iter() }
    }
}

impl<K, V> FromIterator<(K, V)> for OrderedHashMap<K, V>
where
    K: Eq + Hash + Clone,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut map = OrderedHashMap::new();

        for (k, v) in iter {
            map.insert(k, v);
        }

        map
    }
}
/// An iterator over the entries of a `OrderedHashMap`.
///
/// This `struct` is created by the `iter` method on `OrderedHashMap`.
///
/// # Example
///
/// ```ignore
/// let mut map = OrderedHashMap::new();
/// map.insert("a", 1);
/// let iter = map.iter();
/// ```
pub struct Iter<'a, K, V> {
    #[doc(hidden)]
    base: &'a HashMap<K, V>,
    keys_iterator: std::slice::Iter<'a, K>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V>
where
    K: Eq + Hash,
{
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        match self.keys_iterator.next() {
            Some(k) => self.base.get_key_value(&k),
            None => None,
        }
    }
}

/// An owning iterator over the entries of a `OrderedHashMap`.
///
/// This `struct` is created by the `into_iter` method on `OrderedHashMap`
/// (provided by the `IntoIterator` trait)
///
/// # Example
///
/// ```ignore
/// let mut map = OrderedHashMap::new();
/// map.insert("a", 1);
/// let iter = map.into_iter();
/// ```
pub struct IntoIter<K, V> {
    #[doc(hidden)]
    base: HashMap<K, V>,
    keys_iterator: std::vec::IntoIter<K>,
}

impl<K, V> Iterator for IntoIter<K, V>
where
    K: Eq + Hash,
{
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        match self.keys_iterator.next() {
            Some(k) => self.base.remove_entry(&k),
            None => None,
        }
    }
}

/// A mutable iterator over the entries of a `OrderedHashMap`.
/// Note that it iterates in arbitrary order.
pub type IterMut<'a, K, V> = hash_map::IterMut<'a, K, V>;

#[cfg(test)]
mod library_test {
    use super::*;

    #[test]
    fn get() {
        let mut map = OrderedHashMap::new();
        map.insert("a", 1);
        assert_eq!(map.get("a"), Some(&1));
        assert_eq!(map.get("b"), None);
    }
}
