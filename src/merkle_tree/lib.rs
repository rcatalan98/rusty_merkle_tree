use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct MerkleTree {
    nodes: Vec<Node>,
    leavs_offset: usize,
    root_index: usize,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Node{
    hash: Vec<u8>,
    left: Option<usize>,
    right: Option<usize>,
}

#[derive(Debug)]
pub struct Proof{
    path: Option<Vec<Vec<u8>>>,
    direction: Option<Vec<bool>>,
}

impl MerkleTree {
    pub fn new(data: Vec<Vec<u8>>) -> MerkleTree{
        let nodes: Vec<Node> = MerkleTree::create_leavs(data);
        let leafs_index: usize = nodes.len() - 1;
        MerkleTree{nodes, leavs_offset: leafs_index, root_index: 0}
    }

    pub fn get_root(&self) -> Vec<u8> {
        self.nodes[self.nodes.len()-1].hash.clone()
    }

    fn create_leavs(data: Vec<Vec<u8>>) -> Vec<Node> {
        let mut to_return = Vec::new();
        if !is_pwr_two(data.len()) {
            //add the last element n times till len is a power of two.
            let mut i = 0;
            while !is_pwr_two(data.len() + i) {
                i += 1;
            }
            for _ in 0..i {
                let hash = get_sha256(data.last().unwrap());
                to_return.push(Node{hash, left: None, right: None});
            }
        }
        for i in 0..data.len() {
            let hash = get_sha256(&data[i]);
            to_return.push(Node{hash, left: None, right: None});
        }
        if to_return.len() % 2 != 0 {
            to_return.push(to_return.last().unwrap().clone())
        }
        to_return
    }

    pub fn add_data(&mut self, data: Vec<Vec<u8>>) {

        let mut new_data = data.clone();
        if new_data.len() % 2 != 0 {
            new_data.push(new_data.last().unwrap().clone())
        }
        if new_data.len() < self.leavs_offset + 1{
            for i in 0..new_data.len() {
                new_data.push(new_data[i].clone());
            }
        }

        let mut new_tree = MerkleTree::new(new_data);
        new_tree.complete_tree();
        self.merge_trees(new_tree);

    }

    //adds the nodes of the tree to the current one. Should add it in order to mantian path selection.
    fn merge_trees(&mut self, tree: MerkleTree) {
        let tree_levels = tree.get_iterable_level();
        let mut counter = 0;
        let mut self_counter = 0;
        for j in 0..tree_levels.len() {
            counter += tree_levels[j].len();
            for i in 0..tree_levels[j].len() {
                let mut node = tree_levels[j][i].clone();
                if node.left.is_some() {
                    node.left = Some(node.left.unwrap() + tree_levels[j-1].len());
                }
                if node.right.is_some() {
                    node.right = Some(node.right.unwrap() + tree_levels[j-1].len());
                }
                self.nodes.insert(counter + i, node);
                if self.nodes[i + self_counter].left.is_some() && self.nodes[i+self_counter].left.unwrap() >= self.leavs_offset + 1{
                    self.nodes[i + self_counter].left = Some(self.nodes[i + self_counter].left.unwrap() + tree_levels[j-1].len());
                }
                if self.nodes[i + self_counter].right.is_some() && self.nodes[i+self_counter].right.unwrap() >= self.leavs_offset + 1{
                    self.nodes[i + self_counter].right = Some(self.nodes[i + self_counter].right.unwrap() + tree_levels[j-1].len());
                }
            }
            self_counter += tree_levels[j].len();
            counter += tree_levels[j].len();
            
            if j == 0{
                self.leavs_offset += tree_levels[j].len();
            }
        }

        //compute new root
        let left = self.nodes[self.nodes.len()-2].hash.clone();
        let right = self.nodes[self.nodes.len()-1].hash.clone();
        let hash = get_sha256_vec(&vec![left, right]);
        let new_root = Node{hash, left: Some(self.nodes.len()-2), right: Some(self.nodes.len()-1)};
        self.nodes.push(new_root);
        self.root_index = self.nodes.len() - 1;

    }   

    fn get_iterable_level(&self) -> Vec<Vec<Node>> {
        let mut levels = Vec::new();
        let mut level = Vec::new();
        let mut j = self.leavs_offset + 1;
        let mut i = 0;
        let mut k = 1;
        while i < self.nodes.len()  {
            level.push(self.nodes[i].clone());
            i+=1;
            if i == j{
                levels.push(level.clone());
                level = Vec::new();
                let to_add = (self.leavs_offset + 1)/(2_i32.pow(k)) as usize;
                j += to_add;
                k += 1;
            }
        }
        levels
    }

    
    pub fn complete_tree(&mut self) {
        let mut i = 0;
        while i < self.nodes.len() - 1{
            let left = self.nodes[i].hash.clone();
            let right = self.nodes[i+1].hash.clone();
            let hash = get_sha256_vec(&vec![left, right]);
            let new_node = Node{hash, left: Some(i), right: Some(i+1)};
            self.nodes.push(new_node);
            i+=2;
        }
        self.root_index = self.nodes.len() - 1;
    }

    pub fn is_leaf(&self, index: usize) -> bool {
        index <= self.leavs_offset
    }

    // receives a candidate element, hashes and sends to the proof function.
    // Returns the hashes to complete the tree and the directions to follow.
    // In the directions, true means right and false means left. Indicating where to position the hash you are using.
    // Returns empty vectors if the candidate is not in the tree.
    pub fn get_proof(&self, candidate: Vec<u8>) -> Proof{
       for i in 0..self.leavs_offset + 1 {
           if self.nodes[i].hash == get_sha256(&candidate) {
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
                path.push(self.nodes[current_index + 1].hash.clone());
            } else {
                direction.push(true);
                path.push(self.nodes[current_index - 1].hash.clone());
            }
            current_index = parent_index;
        }
        Proof::new(Some(path), Some(direction))
    }

    


    pub fn get_leafs(&self) -> Vec<Node> {
        let mut to_return = Vec::new();
        for i in 0..self.leavs_offset + 1 {
            to_return.push(self.nodes[i].to_owned());
        }
        to_return
    }
}

impl Proof {

    pub fn new(path: Option<Vec<Vec<u8>>>, direction: Option<Vec<bool>>) -> Proof {
        Proof{path, direction}
    }

    // Verifies the proof of a candidate element. Returns true if the proof is valid.
    // The candidate must be the element trying to check if it is in the tree. It's hashed internally.
    pub fn verify_proof(&self, candidate: Vec<u8>, root: Vec<u8>) -> bool {

        if self.path.is_none() || self.direction.is_none() {
            return false;
        }

        let mut hash = get_sha256(&candidate);
        let mut i = 0;
        let direction = self.get_direction().unwrap();
        let path = self.get_path().unwrap();
        while i < direction.len() {
            match direction[i] {
                true => hash = get_sha256_vec( &vec![path[i].clone(), hash]),
                false => hash = get_sha256_vec(&vec![hash, path[i].clone()]),
            }
            i += 1;
        }
        hash == root

    }

    pub fn is_empty(&self) -> bool {
        self.path.is_none() && self.direction.is_none()
    }

    pub fn get_path(&self) -> Option<Vec<Vec<u8>>> {
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
fn get_sha256(data: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.to_vec()[0..8].to_vec()
}

//function like get_sha256 but receives a vector of u64
fn get_sha256_vec(data: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    for i in 0..data.len() {
        hasher.update(data[i].clone());
    }
    let result = hasher.finalize();
    result.to_vec()[0..8].to_vec()
}

fn is_pwr_two(n: usize) -> bool {
    n != 0 && n & (n - 1) == 0
}

fn raw_numbers_to_vector(data: Vec<u8>) -> Vec<Vec<u8>> {
    let mut to_return = Vec::new();
    for i in 0..data.len() {
        to_return.push(vec![data[i]]);
    }
    to_return
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
    fn raw_numbers_to_vector_test(){
        let data = vec![1,2,3,4];
        let expected = vec![vec![1], vec![2], vec![3], vec![4]];
        let result = super::raw_numbers_to_vector(data);
        assert_eq!(result, expected);
    }

    #[test]
    fn leafs_creation() {

        let data = super::raw_numbers_to_vector(vec![1,2,3,4]);
        let leafs = super::MerkleTree::create_leavs(data.clone());
        let h1 = super::get_sha256(&data[0]);
        assert_eq!(leafs[0].hash, h1);
        let h2 = super::get_sha256(&data[1].clone());
        assert_eq!(leafs[1].hash, h2);
        let h3 = super::get_sha256(&data[2].clone());
        assert_eq!(leafs[2].hash, h3);
        let h4 = super::get_sha256(&data[3].clone());
        assert_eq!(leafs[3].hash, h4);

    }

    #[test]
    fn test_tree_generation() {

        let data = super::raw_numbers_to_vector(vec![1,2,3,4]);
        let mut tree = super::MerkleTree::new(data.clone());
        tree.complete_tree();
        assert_eq!(tree.nodes.len(), 7);
        
        let h5 = super::get_sha256_vec(&vec![tree.nodes[0].hash.clone(), tree.nodes[1].hash.clone()]);
        assert_eq!(tree.nodes[4].hash, h5);

        let h6 = super::get_sha256_vec(&vec![tree.nodes[2].hash.clone(), tree.nodes[3].hash.clone()]);
        assert_eq!(tree.nodes[5].hash, h6);

        let h7 = super::get_sha256_vec(&vec![h5, h6]);
        assert_eq!(tree.get_root(), h7);

        assert_eq!(tree.root_index, 6);
    }

    #[test]
    fn test_add_data() {
        let data = super::raw_numbers_to_vector(vec![1,2]);
        let mut tree = super::MerkleTree::new(data.clone());
        tree.complete_tree();
        let root = tree.get_root();

        let data2 = super::raw_numbers_to_vector(vec![3]);
        tree.add_data(data2);
        let new_root = tree.get_root();

        //if the tree is completed as it should then, the roots should be the same
        let data3 = super::raw_numbers_to_vector(vec![1,2,3,3]);

        let mut tree2 = super::MerkleTree::new(data3);
        tree2.complete_tree();
        let new_root2 = tree2.get_root();

        assert_ne!(root, new_root);
        assert_eq!(new_root, new_root2);
        
        println!("TREE: {:?}", tree.nodes);
        println!();
        println!("TREE2: {:?}", tree2.nodes);



    }

    #[test]
    fn test_add_large_data() {
        let data: Vec<Vec<u8>> = super::raw_numbers_to_vector(vec![1,2,3,4]);
        let mut tree = super::MerkleTree::new(data);
        tree.complete_tree();
        let root = tree.get_root();

        let data2: Vec<Vec<u8>> = super::raw_numbers_to_vector(vec![5]);
        tree.add_data(data2);
        let new_root = tree.get_root();

        //if the tree is completed as it should then, the roots should be the same
        let data3: Vec<Vec<u8>> = super::raw_numbers_to_vector(vec![1,2,3,4,5,5,5,5]);
        let mut tree2 = super::MerkleTree::new(data3);
        tree2.complete_tree();
        let new_root2 = tree2.get_root();

        assert_ne!(root, new_root);
        assert_eq!(new_root, new_root2);

    }


    #[test]
    fn test_add_data_verify_new(){
        let data: Vec<Vec<u8>> = super::raw_numbers_to_vector(vec![1,2]);
        let mut tree = super::MerkleTree::new(data);
        tree.complete_tree();

        let data2: Vec<Vec<u8>> = super::raw_numbers_to_vector(vec![3,4]);
        tree.add_data(data2);
        let new_root = tree.get_root();
        println!("TREE: {:?}", tree.nodes);


        let candidate = 3_u8.to_be_bytes().to_vec();
        let proof = tree.get_proof(candidate.clone());
        println!("PROOF: {:?}", proof);
        assert!(proof.verify_proof(candidate.clone(), new_root.clone()));

        let candidate = 4_u8.to_be_bytes().to_vec();
        let proof = tree.get_proof(candidate.clone());
        assert!(proof.verify_proof(candidate.clone(), new_root.clone()));
    }


    #[test]
    fn test_get_leafs(){
        let data: Vec<Vec<u8>> = super::raw_numbers_to_vector(vec![1,2,3,4]);
        let mut tree = super::MerkleTree::new(data.clone());
        tree.complete_tree();
        let leafs = tree.get_leafs();
        assert_eq!(leafs.len(), 4);
        assert_eq!(leafs[0].hash, super::get_sha256(&data[0]));
        assert_eq!(leafs[1].hash, super::get_sha256(&data[1]));
        assert_eq!(leafs[2].hash, super::get_sha256(&data[2]));
        assert_eq!(leafs[3].hash, super::get_sha256(&data[3]));

    }

    #[test]
    fn test_get_proof(){
        let data: Vec<Vec<u8>> = super::raw_numbers_to_vector(vec![1,2,3,4]);
        let mut tree = super::MerkleTree::new(data.clone());
        tree.complete_tree();
        let candidate: Vec<u8> = 3_u8.to_be_bytes().to_vec();
        let a_proof = tree.get_proof(candidate.clone());
        let proof = a_proof.get_path().unwrap();
        assert_eq!(proof.len(),2);
        assert_eq!(proof[0].clone(), super::get_sha256(&data[3]));
        assert_eq!(proof[1].clone(), super::get_sha256_vec(&vec![super::get_sha256(&data[0]), super::get_sha256(&data[1])]));
        assert_eq!(tree.get_root(), super::get_sha256_vec(&vec![proof[1].clone(), super::get_sha256_vec(&vec![super::get_sha256(&candidate), proof[0].clone()])]));
    }

    #[test]
    fn test_verifier(){
        let data: Vec<Vec<u8>> = super::raw_numbers_to_vector(vec![1,2,3,4]);
        let mut tree = super::MerkleTree::new(data.clone());
        tree.complete_tree();

        let candidate:Vec<u8> = 4_u8.to_be_bytes().to_vec();
        let proof = tree.get_proof(candidate.clone());
        assert!(proof.verify_proof(candidate.clone(), tree.get_root()));
    }

    #[test]
    fn verifier_fails(){
        let data: Vec<Vec<u8>> = super::raw_numbers_to_vector(vec![1,2,3,4]);
        let mut tree = super::MerkleTree::new(data.clone());
        tree.complete_tree();

        let proof = tree.get_proof(100_u8.to_be_bytes().to_vec());
        assert!(proof.is_empty());
    }

    #[test]
    fn invalid_proof(){
        let data: Vec<Vec<u8>> = super::raw_numbers_to_vector(vec![1,2,3,4]);
        let mut tree = super::MerkleTree::new(data.clone());
        tree.complete_tree();

        let candidate:Vec<u8> = 4_u8.to_be_bytes().to_vec();
        let proof = tree.get_proof(candidate.clone());
        assert!(!proof.verify_proof(10_u8.to_be_bytes().to_vec(), tree.get_root()));
    }

    #[test]
    fn invalid_proof_element_include(){
        let data: Vec<Vec<u8>> = super::raw_numbers_to_vector(vec![1,2,3,4]);
        let mut tree = super::MerkleTree::new(data.clone());
        tree.complete_tree();

        let candidate: Vec<u8> = 4_u8.to_be_bytes().to_vec();
        let proof = tree.get_proof(candidate.clone());
        assert!(!proof.verify_proof(1_u8.to_be_bytes().to_vec(), tree.get_root()));
    }

    #[test]
    fn test_merge_trees(){
        let data: Vec<Vec<u8>> = super::raw_numbers_to_vector(vec![1,2,3,4]);
        let mut tree = super::MerkleTree::new(data.clone());
        tree.complete_tree();
        let old_root = tree.get_root();

        let data1: Vec<Vec<u8>> = super::raw_numbers_to_vector(vec![1,2]);
        let mut tree1 = super::MerkleTree::new(data1.clone());
        tree1.complete_tree();

        let data2: Vec<Vec<u8>> = super::raw_numbers_to_vector(vec![3,4]);
        let mut tree2 = super::MerkleTree::new(data2.clone());
        tree2.complete_tree();

        tree1.merge_trees(tree2);
        assert_eq!(old_root, tree1.get_root());

    }

    #[test]
    fn test_is_pwr_two(){
        assert!(super::is_pwr_two(2));
        assert!(super::is_pwr_two(4));
        assert!(super::is_pwr_two(8));
        assert!(super::is_pwr_two(16));
        assert!(super::is_pwr_two(32));
        assert!(super::is_pwr_two(64));
        assert!(super::is_pwr_two(128));
        assert!(super::is_pwr_two(256));
    }

    #[test]
    fn test_is_pwr_two_fails(){
        assert!(!super::is_pwr_two(3));
        assert!(!super::is_pwr_two(5));
        assert!(!super::is_pwr_two(6));
        assert!(!super::is_pwr_two(7));
        assert!(!super::is_pwr_two(9));
        assert!(!super::is_pwr_two(11));
        assert!(!super::is_pwr_two(13));
        assert!(!super::is_pwr_two(15));
        assert!(!super::is_pwr_two(17));
    }

    #[test]
    fn test_get_iterable_level(){
        let data: Vec<Vec<u8>> = super::raw_numbers_to_vector(vec![1,2,3,4]);
        let mut tree = super::MerkleTree::new(data.clone());
        tree.complete_tree();

        let levels = tree.get_iterable_level();

        assert_eq!(levels.len(), 3);
        assert_eq!(levels[0].len(), 4);
        assert_eq!(levels[1].len(), 2);
        assert_eq!(levels[2].len(), 1);

        let mut i = 0;
        for l in levels{
            for n in l{
                assert_eq!(n.hash, tree.nodes[i].hash);
                i += 1;
            }
        }

    }

}
