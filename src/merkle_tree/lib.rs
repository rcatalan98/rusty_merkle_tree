use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct MerkleTree {
    nodes: Vec<Node>,
    leafs_offset: usize,
    root_index: usize,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Node{
    hash: u64,
    left: Option<usize>,
    right: Option<usize>,
}

#[derive(Debug)]
pub struct Proof{
    path: Option<Vec<u64>>,
    direction: Option<Vec<bool>>,
}

impl MerkleTree {
    pub fn new(data: Vec<u64>) -> MerkleTree {
        let nodes: Vec<Node> = MerkleTree::create_leafs(data);
        let leafs_index: usize = nodes.len() - 1;
        MerkleTree{nodes, leafs_offset: leafs_index, root_index: 0}
    }

    pub fn get_root(&self) -> u64 {
        self.nodes[self.nodes.len()-1].hash
    }

    fn create_leafs(data: Vec<u64>) -> Vec<Node> {
        let mut to_return = Vec::new();
        for i in 0..data.len() {
            let hash = get_sha256(data[i]);
            to_return.push(Node{hash, left: None, right: None});
        }
        if to_return.len() % 2 != 0 {
            to_return.push(to_return.last().unwrap().clone())
        }
        to_return
    }

    pub fn add_data(&mut self, data: Vec<u64>) {

        let mut new_data = data.clone();
        if new_data.len() % 2 != 0 {
            new_data.push(new_data.last().unwrap().clone())
        }
        if new_data.len() < self.leafs_offset + 1{
            for i in 0..new_data.len() {
                new_data.push(new_data[i]);
            }
        }

        //insert the new data 
        for j in 0..new_data.len() {
            let hash = get_sha256(new_data[j]);
            self.nodes.insert(self.leafs_offset + 1 + j, Node{hash, left: None, right: None});
        }

        //update the indexes of the data structure
        let new_data_index = self.leafs_offset;
        self.leafs_offset += new_data.len() ;
        self.root_index += new_data.len() ;
        
        

        //update the indexes of the nodes
        for i in (self.leafs_offset+1)..self.nodes.len(){
            if self.nodes[i].left.is_some() && self.nodes[i].left.unwrap() > new_data_index {
                self.nodes[i].left = Some(self.nodes[i].left.unwrap() + new_data.len());
            }
            if self.nodes[i].right.is_some() && self.nodes[i].right.unwrap() > new_data_index {
                self.nodes[i].right = Some(self.nodes[i].right.unwrap() + new_data.len());
            }
        }


        //complete the tree with new data
        let mut i = new_data_index + 1;
        let amount_nodes = 2usize.pow(new_data.len() as u32 - 1) - 1;
        let mut j =  new_data.len();

        println!("amount_nodes:{}", amount_nodes);
        println!("j:{}", j);
        while j < amount_nodes - 1{
            let left = self.nodes[i].hash;
            let right = self.nodes[i + 1].hash;
            let hash = get_sha256_vec(vec![left, right]);
            let new_node = Node{hash, left: Some(i), right: Some(i + 1)};
            self.nodes.insert(self.root_index, new_node);
            i+=2;
            j += 1;
            self.root_index += 1;
        }
        //add the last node from the new data
        let left = self.nodes[self.root_index-2].hash;
        let right = self.nodes[self.root_index-1].hash;
        let hash = get_sha256_vec(vec![left, right]);
        let new_node = Node{hash, left: Some(self.root_index-2), right: Some(self.root_index-1)};
        self.nodes.push(new_node);

        //compute root with the last two elements
        let root_hash = get_sha256_vec(vec![self.nodes[self.nodes.len() - 2].hash, self.nodes[self.nodes.len() - 1].hash]);
        let root_node = Node{hash: root_hash, left: Some(self.nodes.len() - 2), right: Some(self.nodes.len() - 1)};
        self.nodes.push(root_node);
        self.root_index = self.nodes.len() - 1;
        
        





        

        //complete the tree from root with new data
        // let mut i = self.root_index;
        // while i < self.nodes.len() - 1{
        //     let left = self.nodes[i].hash;
        //     let right = self.nodes[i+1].hash;
        //     let hash = get_sha256_vec(vec![left, right]);
        //     let new_node = Node{hash, left: Some(i), right: Some(i+1)};
        //     self.nodes.insert(i, new_node);
        //     i+=2;
        // }
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
        self.root_index = self.nodes.len() - 1;
    }

    pub fn is_leaf(&self, index: usize) -> bool {
        index <= self.leafs_offset
    }

    // receives a candidate element, hashes and sends to the proof function.
    // Returns the hashes to complete the tree and the directions to follow.
    // In the directions, true means right and false means left. Indicating where to position the hash you are using.
    // Returns empty vectors if the candidate is not in the tree.
    pub fn get_proof(&self, candidate:u64) -> Proof{
       for i in 0..self.leafs_offset + 1 {
           if self.nodes[i].hash == get_sha256(candidate) {
               return self.get_proof_from_index(i);
           }
       }
        Proof { path: None, direction: None }
    }

    fn get_proof_from_index(&self, index: usize) -> Proof {
        let mut path = Vec::new();
        let mut direction = Vec::new();
        let mut current_index = index;
        let mut i: u32 = 1;
        let mut node_accumulator = 0;
    
        while current_index < self.root_index {
            let base: usize = 2;
            let amount_level = self.nodes.len()/(base.pow(i)) + 1;
            let side = if current_index < amount_level/2 + node_accumulator  {0} else {1};

            let parent_index = amount_level + side + node_accumulator;
            i += 1;
            node_accumulator += amount_level;
            
            if current_index % 2 == 0 {
                direction.push(false);
                path.push(self.nodes[current_index + 1].hash);
            } else {
                direction.push(true);
                path.push(self.nodes[current_index - 1].hash);
            }
            current_index = parent_index;
        }
        Proof::new(Some(path), Some(direction))
    }

    


    pub fn get_leafs(&self) -> Vec<Node> {
        let mut to_return = Vec::new();
        for i in 0..self.leafs_offset + 1 {
            to_return.push(self.nodes[i].to_owned());
        }
        to_return
    }

    
}

impl Proof {

    pub fn new(path: Option<Vec<u64>>, direction: Option<Vec<bool>>) -> Proof {
        Proof{path, direction}
    }

    // Verifies the proof of a candidate element. Returns true if the proof is valid.
    // The candidate must be the element trying to check if it is in the tree. It's hashed internally.
    pub fn verify_proof(&self, candidate: u64, root: u64) -> bool {

        if self.path.is_none() || self.direction.is_none() {
            return false;
        }

        let mut hash = get_sha256(candidate);
        let mut i = 0;
        let direction = self.get_direction().unwrap();
        let path = self.get_path().unwrap();
        while i < direction.len() {
            match direction[i] {
                true => hash = get_sha256_vec(vec![path[i], hash]),
                false => hash = get_sha256_vec(vec![hash, path[i]]),
            }
            i += 1;
        }
        hash == root

    }

    pub fn is_empty(&self) -> bool {
        self.path.is_none() && self.direction.is_none()
    }

    pub fn get_path(&self) -> Option<Vec<u64>> {
        self.path.to_owned()
    }

    pub fn get_direction(&self) -> Option<Vec<bool>> {
        self.direction.to_owned()
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

        let data: Vec<u64> = vec![1,2,3,4];
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

        let data: Vec<u64> = vec![1,2,3,4];
        let mut tree = super::MerkleTree::new(data);
        tree.complete_tree();
        assert_eq!(tree.nodes.len(), 7);
        
        let h5 = super::get_sha256_vec(vec![tree.nodes[0].hash, tree.nodes[1].hash]);
        assert_eq!(tree.nodes[4].hash, h5);

        let h6 = super::get_sha256_vec(vec![tree.nodes[2].hash, tree.nodes[3].hash]);
        assert_eq!(tree.nodes[5].hash, h6);

        let h7 = super::get_sha256_vec(vec![h5, h6]);
        assert_eq!(tree.get_root(), h7);

        assert_eq!(tree.root_index, 6);
    }

    #[test]
    fn test_add_data() {
        //TODO improve test. There is a bug here.
        let data: Vec<u64> = vec![1,2];
        let mut tree = super::MerkleTree::new(data);
        tree.complete_tree();
        let root = tree.get_root();

        let data2: Vec<u64> = vec![3];
        tree.add_data(data2);
        let new_root = tree.get_root();
        println!("TREE: {:?}", tree.nodes);

        //if the tree is completed as it should then, the roots should be the same
        let data3: Vec<u64> = vec![1,2,3,3];
        let mut tree2 = super::MerkleTree::new(data3);
        tree2.complete_tree();
        let new_root2 = tree2.get_root();

        assert_ne!(root, new_root);
        assert_eq!(new_root, new_root2);


    }

    #[test]
    fn test_add_large_data() {
        //TODO improve test. There is a bug here.
        let data: Vec<u64> = vec![1,2,3,4];
        let mut tree = super::MerkleTree::new(data);
        tree.complete_tree();
        let root = tree.get_root();

        let data2: Vec<u64> = vec![5];
        tree.add_data(data2);
        let new_root = tree.get_root();
        println!("TREE: {:?}", tree.nodes);
        println!("");

        //if the tree is completed as it should then, the roots should be the same
        let data3: Vec<u64> = vec![1,2,3,4,5,5,5,5];
        let mut tree2 = super::MerkleTree::new(data3);
        tree2.complete_tree();
        let new_root2 = tree2.get_root();
        println!("TREE2: {:?}", tree2.nodes);

        assert_ne!(root, new_root);
        assert_eq!(new_root, new_root2);


    }


    #[test]
    fn test_add_data_verify_new(){
        let data: Vec<u64> = vec![1,2];
        let mut tree = super::MerkleTree::new(data);
        tree.complete_tree();

        let data2: Vec<u64> = vec![3,4];
        tree.add_data(data2);
        let new_root = tree.get_root();

        println!("TREE: {:?}", tree.nodes);


        let proof = tree.get_proof(3);
        println!("PROOF: {:?}", proof);
        assert!(proof.verify_proof(3, new_root));


        let proof = tree.get_proof(4);
        assert!(proof.verify_proof(4, new_root));
    }


    #[test]
    fn test_get_leafs(){
        let data: Vec<u64> = vec![1,2,3,4];
        let mut tree = super::MerkleTree::new(data);
        tree.complete_tree();
        let leafs = tree.get_leafs();
        assert_eq!(leafs.len(), 4);
        assert_eq!(leafs[0].hash, super::get_sha256(1));
        assert_eq!(leafs[1].hash, super::get_sha256(2));
        assert_eq!(leafs[2].hash, super::get_sha256(3));
        assert_eq!(leafs[3].hash, super::get_sha256(4));

    }

    #[test]
    fn test_get_proof(){
        let data: Vec<u64> = vec![1,2,3,4];
        let mut tree = super::MerkleTree::new(data);
        tree.complete_tree();
        let candidate = 3;
        let a_proof = tree.get_proof(candidate);
        let proof = a_proof.get_path().unwrap();
        assert_eq!(proof.len(),2);
        assert_eq!(proof[0], super::get_sha256(4));
        assert_eq!(proof[1], super::get_sha256_vec(vec![super::get_sha256(1), super::get_sha256(2)]));
        assert_eq!(tree.get_root(), super::get_sha256_vec(vec![proof[1], super::get_sha256_vec(vec![super::get_sha256(candidate), proof[0]])]));
    }

    #[test]
    fn test_verifier(){
        let data: Vec<u64> = vec![1,2,3,4];
        let mut tree = super::MerkleTree::new(data);
        tree.complete_tree();

        let candidate = 4;
        let proof = tree.get_proof(candidate);
        assert!(proof.verify_proof(candidate, tree.get_root()));
    }

    #[test]
    fn verifier_fails(){
        let data: Vec<u64> = vec![1,2,3,4];
        let mut tree = super::MerkleTree::new(data);
        tree.complete_tree();

        let proof = tree.get_proof(10000);
        assert!(proof.is_empty());
    }

    #[test]
    fn invalid_proof(){
        let data: Vec<u64> = vec![1,2,3,4];
        let mut tree = super::MerkleTree::new(data);
        tree.complete_tree();

        let candidate = 4;
        let proof = tree.get_proof(candidate);
        assert!(!proof.verify_proof(10, tree.get_root()));
    }

    #[test]
    fn invalid_proof_element_include(){
        let data: Vec<u64> = vec![1,2,3,4];
        let mut tree = super::MerkleTree::new(data);
        tree.complete_tree();

        let candidate = 4;
        let proof = tree.get_proof(candidate);
        assert!(!proof.verify_proof(1, tree.get_root()));
    }

    

}
