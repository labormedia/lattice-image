use alloc::collections::btree_set::{
    BTreeSet,
    Iter,
};

#[derive(Clone, Debug)]
pub struct ExchangeSet<T> {
    set: BTreeSet<T>
}

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

}