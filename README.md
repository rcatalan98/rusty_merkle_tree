# rusty_merkle_tree
This is a solution to a proposed exercise to learn Rust. The objective is to implement a simple Merkle Tree with the following characteristics:
- A Merkle Tree can be built out of an array.
- A Merkle Tree can generate a proof that it contains an element.
- A Merkle Tree can verify that a given hash is contained in it.
- A Merke Tree can be dynamic, this means that elements can be added once it is built.

Steps to use the code:
1. Install rust and cargo if needed.
2. Clone the repo.
3. Run the tests ```cargo test```
4. Try the implemented main code with ```cargo run```

How to use the library:
- Create an instance of the MerkleTree struct by calling the new() method and passing in a vector of data. This will initialize the tree with the given data.
```
let data = vec![1, 2, 3, 4, 5];
let mut tree = MerkleTree::new(data);
```
- Retrieve the root hash of the tree by calling the get_root() method on the tree instance.
``` 
let root_hash = tree.get_root(); 
```
- To add new data to the tree, call the add_data() method on the tree instance and pass in a vector of new data.
``` 
let new_data = vec![6, 7, 8, 9];
tree.add_data(new_data);
```
- To verify that a given hash is contained in the tree, call the verify_hash() method on the tree instance and pass in the hash to verify. This method will return a Boolean value indicating if the hash is present in the tree.
```
let hash_to_verify = get_sha256(6);
let is_present = tree.verify_hash(hash_to_verify);
```
- To generate a proof that a specific element is contained in the tree, call the generate_proof() method on the tree instance and pass in the element to prove. This method will return a vector of the hashes of the nodes in the path from the leaf node containing the element to the root.
```
let element_to_prove = 6;
let proof = tree.generate_proof(element_to_prove);
```
