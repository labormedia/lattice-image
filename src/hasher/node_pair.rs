use core::cmp::Ordering;
use crate::hasher::{
    CipherBlock,
    Node,
    hash_visit,
    Blake2b,
    U32,
};

#[derive(Clone, Hash, Debug)]
pub struct NodePair<T, InternalValues>
where T: PartialOrd,
{
    id: CipherBlock,
    l: Node,
    r: Node,
    objective: T,
    payload: InternalValues
}

impl<T, InternalValues> PartialOrd<NodePair<T, InternalValues>> for NodePair<T, InternalValues>
where T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<T, InternalValues> PartialEq<NodePair<T, InternalValues>> for NodePair<T, InternalValues> 
where T: PartialOrd,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T, InternalValues> Eq for NodePair<T, InternalValues> where T: PartialOrd, {}

impl<T, InternalValues> Ord for NodePair<T, InternalValues> where T: PartialOrd, {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    } 
}

impl<T, InternalValues> NodePair<T, InternalValues>
where T: PartialOrd,
{
    //! Ordered id NodePair creation.
    pub fn from(left: Node, right: Node, objective: (T, InternalValues)) -> Self {
        let (order_left, order_right) = if left <= right {
            (left.clone(), right.clone())
        } else {
            (right.clone(), left.clone())
        };
        let mut output = CipherBlock::default();
        let _ = hash_visit::<Blake2b<U32>>(order_left.get_id_ref(), order_right.get_id_ref(), &mut output);
        NodePair {
            id: output,
            l: left,
            r: right,
            objective: objective.0,
            payload: objective.1,
        }
    }
    pub fn get_pair_id(left: Node, right: Node) -> CipherBlock {
        let (order_left, order_right) = if left <= right {
            (left.clone(), right.clone())
        } else {
            (right.clone(), left.clone())
        };
        let mut output = CipherBlock::default();
        let _ = hash_visit::<Blake2b<U32>>(order_left.get_id_ref(), order_right.get_id_ref(), &mut output);
        output
    }
    pub fn get_pair_order(left: Node, right: Node) -> (Node, Node) {
        if left <= right {
            (left.clone(), right.clone())
        } else {
            (right.clone(), left.clone())
        }
    }
}

impl<T, InternalValues> NodePair<T, InternalValues>
where T: PartialOrd,
{
    pub fn get_id_ref(&self) -> &CipherBlock {
        &self.id
    }
    pub fn get_id(self) -> CipherBlock {
        self.id
    }
    pub fn get_left_node_ref(&self) -> &Node{
        &self.l
    }
    pub fn get_right_node_ref(&self) -> &Node{
        &self.r
    }    
    pub fn get_objective_ref(&self) -> &T {
        &self.objective
    }
    pub fn get_payload_ref(&self) -> &InternalValues {
        &self.payload
    }
    pub fn get_objective_payload(self) -> (T, InternalValues) {
        (self.objective, self.payload)
    }
}