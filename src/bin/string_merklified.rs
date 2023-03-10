use rusty_merkle_tree::{MerkleTree, Proof};
fn main() {
    let str: String = "Hello World".to_string();

    //str cut in blocks so it can be merklified
    let data: Vec<&str> = str.split(" ").collect();

    // data transformed to bytes.
    let data: Vec<Vec<u8>> = data.iter().map(|x| x.as_bytes().to_vec()).collect();

    let mut tree: MerkleTree = MerkleTree::new(data);
    tree.complete_tree();

    let candidate = "Hello".as_bytes().to_vec();

    //get proof of Hello
    let proof: Proof = tree.get_proof(candidate.clone());
    println!("The validity of the proof is: {:?}", proof.verify_proof(candidate.clone(), tree.get_root()));

}
