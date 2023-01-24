//import hash
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
        let mut nodes: Vec<Node> = Vec::new();
        // create the leafs
        for i in 0..data.len() {
            let mut hasher = Sha256::new();
            hasher.update(&data[i..]);
            let result = hasher.finalize();
            let hash = hash_to_u64(result.to_vec());
            nodes.push(Node{hash, left: None, right: None});
        }
        // create the rest of the tree
        for i in 0..nodes.len(){
            if i%2 != 0 {
                continue;
            }
            let mut hasher = Sha256::new();
            let left = nodes[i].hash;
            let right = nodes[i+1].hash;
            hasher.update(left.to_be_bytes());
            hasher.update(right.to_be_bytes());
            let result = hasher.finalize();
            let hash = hash_to_u64(result.to_vec());
            nodes.push(Node{hash, left: Some(i), right: Some(i+1)});
        }
        MerkleTree{nodes}

    }

    pub fn get_root(&self) -> u64 {
        self.nodes[self.nodes.len()-1].hash
    }
}

//function to convert the hash returned by the hasher to an u64. Using the first 8bytes
fn hash_to_u64(hash: Vec<u8>) -> u64 {
    let mut to_return: u64 = 0;
    for i in 0..8 {
        to_return += (hash[i] as u64) << (i * 8);
    }
    to_return
}