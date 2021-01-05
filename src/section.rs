use crate::ordered_hashmap::OrderedHashMap;

#[derive(Debug)]
pub struct Section {
    #[doc(hidden)]
    base: OrderedHashMap<String, String>,
}

impl Section {
    pub fn new() -> Section {
        Section { base: OrderedHashMap::new() }
    }

    pub fn iter(&self) -> Iter {
        return self.base.iter();
    }

    pub fn iter_mut(&mut self) -> IterMut {
        return self.base.iter_mut();
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.base.get(key)
    }

    pub fn keys(&self) -> std::slice::Iter<String> {
        self.base.keys()
    }

    pub fn insert(&mut self, key: String, value: String) -> Option<String> {
        self.base.insert(key, value)
    }

    pub fn remove(&mut self, key: &String) -> Option<String> {
        self.base.remove(key)
    }
}

impl IntoIterator for Section {
    type Item = (String, String);
    type IntoIter = crate::ordered_hashmap::IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.base.into_iter()
    }
}

pub type Iter<'a> = crate::ordered_hashmap::Iter<'a, String, String>;
pub type IterMut<'a> = crate::ordered_hashmap::IterMut<'a, String, String>;
