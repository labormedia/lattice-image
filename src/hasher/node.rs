use crate::hasher::{
    CipherBlock,
    Blake2b,
    hash_visit,
    U32,
};

///   Node depends on implementations of PartialEq, Eq, PartialOrd and Ord
///   that consider the values in its original order, even though the CipheBlock
///   produced for its hash id is previously ordering the input left and right values
///   of the points. The original ordering is then used to compare Node that would have the same
///   id but have a different ordering of its values.
#[derive(Clone, Hash, PartialEq, Eq, Debug, Ord, PartialOrd)]
pub struct Node {
    id: CipherBlock,
    l: u32,
    r: u32,
}

impl Node {
    pub fn get_id_ref(&self) -> &CipherBlock {
        &self.id
    }
    pub fn pair_node_id(&self, other: &Self) -> CipherBlock{
        let (order_left, order_right) = if self <= other {
            (self, other)
        } else {
            (other, self)
        };
        let mut output = CipherBlock::default();
        let _ = hash_visit::<Blake2b<U32>>(order_left.get_id_ref(), order_right.get_id_ref(), &mut output);
        output
    }
    pub fn get_pair(&self) -> (u32, u32) {
        (self.l, self.r)
    }
}

impl From<(u32, u32)> for Node {
    fn from(value: (u32, u32)) -> Self {
        let mut output_pre_order = CipherBlock::default();
        let pre_ordered = (value.0.to_le_bytes(), value.1.to_le_bytes());
        hash_visit::<Blake2b<U32>>(&pre_ordered.0, &pre_ordered.1, &mut output_pre_order);
        let post_ordered = if value.0 <= value.1 {
            pre_ordered
        } else {
            (pre_ordered.1, pre_ordered.0)
        };
        let mut output_post_order = CipherBlock::default();
        hash_visit::<Blake2b<U32>>(&post_ordered.0, &post_ordered.1, &mut output_post_order);
        
        let mut output_id = CipherBlock::default();
        hash_visit::<Blake2b<U32>>(&output_pre_order, &output_post_order, &mut output_id);
        Node {
            id: output_id,
            l: value.0,
            r: value.1,
        }
    }
}