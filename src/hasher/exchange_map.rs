use alloc::collections::btree_map::BTreeMap;
#[cfg(not(feature = "rayon"))]
use alloc::collections::btree_map::Iter;
#[cfg(feature = "rayon")]
use rayon::collections::btree_map::Iter;

#[derive(Clone, Debug)]
pub struct ExchangeMap<K, V>  {
    map: BTreeMap<K, V> 
}

impl<K, V> ExchangeMap<K, V> 
where 
 #[cfg(not(feature = "rayon"))]
 K: PartialOrd + Ord,
 #[cfg(feature = "rayon")]
 K: PartialOrd + Ord + core::marker::Sync, V: core::marker::Sync,
{
    pub fn new() -> Self {
        ExchangeMap {
            map: BTreeMap::new()
        }
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.map.insert(key, value)
    }
    pub fn clear(&mut self) {
        self.map.clear();
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }
    pub fn iter(&self) -> Iter<'_, K, V> {
        #[cfg(not(feature = "rayon"))]
        self.set.iter()
        #[cfg(feature = "rayon")]
        self.set.par_iter()
    }
    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::hasher::ExchangeMap;
    #[test]
    fn create_and_append_exchange_map() {
        let mut new_map = ExchangeMap::<u32, u8>::new();
        new_map.insert(3, 2);
        new_map.insert(4, 3);
        new_map.insert(5, 4);
        assert_eq!(new_map.insert(3,1), Some(2));
        assert_eq!(new_map.contains_key(&3), true);
        assert_eq!(new_map.contains_key(&4), true);
        assert_eq!(new_map.contains_key(&5), true);
        assert_eq!(new_map.contains_key(&6), false);
        assert_eq!(new_map.get(&3), Some(&1));
        new_map.clear();
        assert_eq!(new_map.get(&3), None);
    }
}