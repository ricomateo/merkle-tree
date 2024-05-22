pub mod merkle_tree;
use merkle_tree::MerkleTree;

fn main() {
    let elements: Vec<String> = vec![
        "A".to_string(),
        "B".to_string(),
        "C".to_string(),
        "D".to_string(),
    ];
    let tree = MerkleTree::new(elements);
    println!("tree = {:?}", tree);
}
