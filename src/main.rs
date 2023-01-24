use rusty_merkle_tree::MerkleTree;
fn main() {
    let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let tree = MerkleTree::new(data);
    println!("{:?}", tree);
    println!("Root: {}", tree.get_root());
}
