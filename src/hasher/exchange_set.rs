use alloc::collections::btree_set::BTreeSet;
#[cfg(not(feature = "rayon"))]
use alloc::collections::btree_set::Iter;
#[cfg(feature = "rayon")]
use rayon::collections::btree_set::Iter;
#[cfg(feature = "rayon")]
use rayon::iter::IntoParallelRefIterator;

#[derive(Clone, Debug)]
pub struct ExchangeSet<T> {
    set: BTreeSet<T>
}

 #[cfg(not(feature = "rayon"))]
impl<T> ExchangeSet<T> 
where 
 T: PartialOrd + Ord,
{
    pub fn new() -> Self {
        ExchangeSet {
            set: BTreeSet::new()
        }
    }
    pub fn insert(&mut self, value: T) -> bool {
        self.set.insert(value)
    }
    pub fn clear(&mut self) {
        self.set.clear();
    }
    pub fn get(&self, value: &T) -> Option<&T> {
        self.set.get(value)
    }
    pub fn iter(&mut self) -> Iter<'_, T> {
        self.set.iter()
    }
    pub fn contains(&self, value: &T) -> bool {
        self.set.contains(value)
    }
    pub fn len(&self) -> usize {
        self.set.len()
    }
    pub fn is_empty(&self) -> bool {
        self.set.len() == 0
    }
}

#[cfg(feature = "rayon")]
impl<T> ExchangeSet<T> 
where 
 T: PartialOrd + Ord + core::marker::Sync,
{
    pub fn new() -> Self {
        ExchangeSet {
            set: BTreeSet::new()
        }
    }
    pub fn insert(&mut self, value: T) -> bool {
        self.set.insert(value)
    }
    pub fn clear(&mut self) {
        self.set.clear();
    }
    pub fn get(&self, value: &T) -> Option<&T> {
        self.set.get(value)
    }
    pub fn iter(&mut self) -> Iter<'_, T> {
        self.set.par_iter()
    }
    pub fn contains(&self, value: &T) -> bool {
        self.set.contains(value)
    }
    pub fn len(&self) -> usize {
        self.set.len()
    }
    pub fn is_empty(&self) -> bool {
        self.set.len() == 0
    }
}