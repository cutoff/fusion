use std::fmt;
use std::fmt::Debug;

/// The poor man's solution of a Map when you can't count on Eq, Hash or Ord.
/// Of course, very inefficient, so for small collections only.
/// TODO check this, it's ChatGPT generated
pub struct VecMap<K, V> {
    elements: Vec<(K, V)>,
}

impl<K, V> Default for VecMap<K, V> {
    fn default() -> Self {
        VecMap {
            elements: Vec::new(),
        }
    }
}

impl<K, V> Debug for VecMap<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.elements.iter().map(|(k, v)| (k, v))).finish()
    }
}

// Custom iterator for VecMap
pub struct VecMapIter<'a, K, V> {
    map: &'a VecMap<K, V>,
    index: usize,
}

impl<'a, K, V> VecMapIter<'a, K, V> {
    fn new(map: &'a VecMap<K, V>) -> Self {
        VecMapIter { map, index: 0 }
    }
}

// Implementing Iterator for VecMapIter
impl<'a, K, V> Iterator for VecMapIter<'a, K, V> {
    type Item = &'a (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.map.elements.len() {
            let item = &self.map.elements[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<K: PartialEq, V> VecMap<K, V> {
    pub fn new() -> Self {
        VecMap { elements: Vec::new() }
    }

    pub fn insert(&mut self, key: K, value: V) {
        for (k, v) in &mut self.elements {
            if *k == key {
                *v = value;
                return;
            }
        }
        self.elements.push((key, value));
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.elements.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(pos) = self.elements.iter().position(|(k, _)| k == key) {
            Some(self.elements.remove(pos).1)
        } else {
            None
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.elements.iter().any(|(k, _)| k == key)
    }

    pub fn iter(&self) -> VecMapIter<K, V> {
        VecMapIter::new(self)
    }
}
