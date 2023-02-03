use rusty_merkle_tree::MerkleTree;
fn main() {
    let data: Vec<u64> = vec![1, 2, 3, 4, 5,6];
    let mut tree = MerkleTree::new(data);
    tree.complete_tree();
    println!("{:?}", tree);
    println!("Root: {}", tree.get_root());
}
