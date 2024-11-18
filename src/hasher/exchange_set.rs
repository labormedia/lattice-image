use alloc::collections::btree_set::BTreeSet;
#[cfg(not(feature = "rayon"))]
use alloc::collections::btree_set::Iter;
#[cfg(feature = "rayon")]
use rayon::collections::btree_set::Iter;

#[derive(Clone, Debug)]
pub struct ExchangeSet<T> {
    set: BTreeSet<T>
}

impl<T> ExchangeSet<T> 
where 
 #[cfg(not(feature = "rayon"))]
 T: PartialOrd + Ord,
 #[cfg(feature = "rayon")]
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
        #[cfg(not(feature = "rayon"))]
        self.set.iter()
        #[cfg(feature = "rayon")]
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