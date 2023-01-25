//import hash
//TODO cambiar por sha3
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct MerkleTree {
    nodes: Vec<Node>
}

#[derive(Debug)]
pub struct Node{
    hash: u64,
    left: Option<usize>,
    right: Option<usize>,
}

impl MerkleTree {
    pub fn new(data: Vec<u8>) -> MerkleTree {
        let nodes: Vec<Node> = MerkleTree::create_leafs(data);
        MerkleTree{nodes}

    }

    pub fn get_root(&self) -> u64 {
        self.nodes[self.nodes.len()-1].hash
    }

    fn create_leafs(data: Vec<u8>) -> Vec<Node> {
        let mut to_return = Vec::new();
        for i in 0..data.len() {
            let hash = get_sha256(data[i] as u64);
            to_return.push(Node{hash, left: None, right: None});
        }
        to_return
    }

    pub fn complete_tree(&mut self) {
        let mut i = 0;
        while i < self.nodes.len() - 1{

            let left = self.nodes[i].left;
            let right = self.nodes[i].right;

            if left.is_some() && right.is_some() {
                i+=1;
                continue;
            }

            //update left and right
            self.nodes[i].left = Some(i);
            self.nodes[i].right = Some(i+1);

            //create new node
            let hash = get_sha256_vec(vec![self.nodes[i].hash, self.nodes[i+1].hash]);
            self.nodes.push(Node{hash, left: None, right: None});

            i+=2;

        }
    }
}

//function to convert the hash returned by the hasher to an u64. Using the first 8bytes
fn hash_to_u64(hash: Vec<u8>) -> u64 {
    let mut to_return: u64 = 0;
    for i in 0..8 {
        to_return += (hash[i] as u64) << ((7-i) * 8);
    }
    to_return
}

// calculate sha256 and returns the first 8 bytes of the hash
fn get_sha256(data: u64) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(data.to_be_bytes());
    let result = hasher.finalize();
    hash_to_u64(result.to_vec())
}

//function like get_sha256 but receives a vector of u64
fn get_sha256_vec(data: Vec<u64>) -> u64 {
    let mut hasher = Sha256::new();
    for i in 0..data.len() {
        hasher.update(data[i].to_be_bytes());
    }
    let result = hasher.finalize();
    hash_to_u64(result.to_vec())
}

#[cfg(test)]
mod tests {
    #[test]
    fn u64_convertion() {
        let hash: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 1];
        let expected = 1;
        let result = super::hash_to_u64(hash);
        assert_eq!(result, expected);

        let hash: Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255, 254];
        let expected = 18446744073709551614;
        let result = super::hash_to_u64(hash);
        assert_eq!(result, expected);

    }

    #[test]
    fn leafs_creation() {

        let data: Vec<u8> = vec![1,2,3,4];
        let leafs = super::MerkleTree::create_leafs(data);
        let h1 = super::get_sha256(1);
        assert_eq!(leafs[0].hash, h1);
        let h2 = super::get_sha256(2);
        assert_eq!(leafs[1].hash, h2);
        let h3 = super::get_sha256(3);
        assert_eq!(leafs[2].hash, h3);
        let h4 = super::get_sha256(4);
        assert_eq!(leafs[3].hash, h4);

    }

    #[test]
    fn test_tree_generation() {
        let data: Vec<u8> = vec![1,2,3,4];
        let mut tree = super::MerkleTree::new(data);
        tree.complete_tree();
        assert_eq!(tree.nodes.len(), 7);
        
        let h5 = super::get_sha256_vec(vec![tree.nodes[0].hash, tree.nodes[1].hash]);
        assert_eq!(tree.nodes[4].hash, h5);

        let h6 = super::get_sha256_vec(vec![tree.nodes[2].hash, tree.nodes[3].hash]);
        assert_eq!(tree.nodes[5].hash, h6);

        let h7 = super::get_sha256_vec(vec![h5, h6]);
        assert_eq!(tree.get_root(), h7);
        
    }

}
