pub use blake2::{Digest, Blake2b, digest::consts::U32};

mod node_pair;
pub use node_pair::NodePair;

mod node;
pub use node::Node;

mod exchange_set;
pub use exchange_set::ExchangeSet;

mod exchange_map;
pub use exchange_map::ExchangeMap;

pub type CipherBlock = [u8;32];

pub fn data_hash<D: Digest>(data: &[u8], output: &mut [u8]) {
    let mut hasher = D::new();
    hasher.update(data);
    output.copy_from_slice(&hasher.finalize());
}

pub fn hash_visit<D: Digest>(left: &[u8], right: &[u8], output: &mut [u8]) {
    let mut hasher = D::new();
    hasher.update(left);
    hasher.update(right);
    output.copy_from_slice(&hasher.finalize());
}

#[cfg(test)]
mod tests {
    use crate::{
        MatrixImageBuilder,
        MatrixImage,
        Neighborhood,
        traits::LatticeElement,
    };
    use crate::hasher::{
        Node,
        ExchangeSet,
    };
    
    type Precision = f64;
    type Atom = LatticeElement<Precision>;
    use blake2::{Digest, Blake2b};
    
    #[test]
    fn digest_cmp() {
        let mut h1 = Blake2b::new();
        let mut h2 = Blake2b::new();
        
        h1.update(b"message");
        h2.update(b"quote");
        
        let hash_a: [u8; 32] = h1.finalize().into();
        let hash_b: [u8; 32] = h2.finalize().into();

        assert!(((hash_a < hash_b) ^ (hash_a > hash_b)) ^ (hash_a == hash_b));
    }
    
    #[test]
    fn all_hashes_for_all_positions_are_different() {
        let even_size: usize = 1_001; // When even, all_district doesn't collide when doing a turn around the last element of the lattice.
        let size = (even_size, even_size);
        let matrix: MatrixImage<Atom> = MatrixImageBuilder::init()
            .with_initial_value(35.0.into())
            .with_height_and_width(size.0, size.1).build();
        let half: u32 = (even_size/2).try_into().unwrap();
        let center = (half,half);
        let all_district = matrix.get_lattice_neighborhood(center, half as usize, Neighborhood::Moore);
        
        let mut set = ExchangeSet::new();
        
        for point in all_district {
            assert!(set.insert(Node::from(point)), "{:?}", Node::from(point));
        }
    }
}
