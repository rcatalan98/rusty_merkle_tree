use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct MerkleTree {
    nodes: Vec<Node>,
    leafs_offset: usize,
    root_index: usize,
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
        let leafs_index: usize = nodes.len() - 1;
        MerkleTree{nodes, leafs_offset: leafs_index, root_index: 0}
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

    //TODO fix.
    pub fn add_data(&mut self, data: Vec<u8>) {
        //insert the new data 
        for j in 0..data.len() {
            let hash = get_sha256(data[j] as u64);
            self.nodes.insert(self.leafs_offset + 1, Node{hash, left: None, right: None});
        }

        //update the indexes of the nodes
        for i in self.leafs_offset + 1..self.nodes.len() {
            if self.nodes[i].left.is_some() {
                self.nodes[i].left = Some(self.nodes[i].left.unwrap() + data.len());
            }
            if self.nodes[i].right.is_some() {
                self.nodes[i].right = Some(self.nodes[i].right.unwrap() + data.len());
            }
        }

        

        //complete the tree with new data
        let mut i = 0;
        while i < data.len() - 1 {
            let left = self.nodes[self.leafs_offset + 1 + i].hash;
            let right = self.nodes[self.leafs_offset + 1 + i + 1].hash;
            let hash = get_sha256_vec(vec![left, right]);
            let new_node = Node{hash, left: Some(self.leafs_offset + 1 + i), right: Some(self.leafs_offset + 1 + i + 1)};
            self.nodes.push(new_node);
            i+=2;
        }

        //update the indexes
        self.leafs_offset += data.len() ;
        self.root_index += data.len() ;

        //complete the tree from root with new data
        let mut i = self.root_index;
        while i < self.nodes.len() - 1{
            let left = self.nodes[i].hash;
            let right = self.nodes[i+1].hash;
            let hash = get_sha256_vec(vec![left, right]);
            let new_node = Node{hash, left: Some(i), right: Some(i+1)};
            self.nodes.push(new_node);
            i+=2;
        }
        self.root_index = self.nodes.len() - 1;

    }

    
    pub fn complete_tree(&mut self) {
        let mut i = 0;
        while i < self.nodes.len() - 1{
            let left = self.nodes[i].hash;
            let right = self.nodes[i+1].hash;
            let hash = get_sha256_vec(vec![left, right]);
            let new_node = Node{hash, left: Some(i), right: Some(i+1)};
            self.nodes.push(new_node);
            i+=2;
        }
    }

    pub fn contains_hash(&self, candidate: u64) -> bool {
        let mut i = self.root_index;
        let mut to_verify = candidate;
        while i > 0 {
            if self.nodes[i].left.is_some() && self.nodes[i].right.is_some(){
                let left = self.nodes[i].left.unwrap();
                let right = self.nodes[i].right.unwrap();
                if self.nodes[left].hash == to_verify {
                    to_verify = self.nodes[right].hash;
                } else {
                    to_verify = self.nodes[left].hash;
                }
            }
            i = (i-1)/2;
        }
        to_verify == self.nodes[0].hash
    }

    pub fn contains_element(&self, candidate: u64) -> bool {
        self.contains_hash(get_sha256(candidate))
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

    #[test]
    fn test_add_data() {
        //TODO improve test. There is a bug here.
        let data: Vec<u8> = vec![1,2,3,4];
        let mut tree = super::MerkleTree::new(data);
        tree.complete_tree();
        let root = tree.get_root();

        let data2: Vec<u8> = vec![5,6];
        tree.add_data(data2);
        let new_root = tree.get_root();

        assert_ne!(root, new_root);

    }

    #[test]
    fn test_contains_hash() {

        let data: Vec<u8> = vec![1,2,3,4];
        let mut tree = super::MerkleTree::new(data);
        tree.complete_tree();

        let to_check = super::get_sha256(1);
        assert!(tree.contains_hash(to_check));
    }

    #[test]
    fn test_contains_element() {
        let data: Vec<u8> = vec![1,2,3,4];
        let mut tree = super::MerkleTree::new(data);
        tree.complete_tree();
        assert!(tree.contains_element(1));
    }

    #[test]
    fn test_contains_element_false() {
        let data: Vec<u8> = vec![1,2,3,4];
        let mut tree = super::MerkleTree::new(data);
        tree.complete_tree();
        assert!(!tree.contains_element(5));
    }

}
