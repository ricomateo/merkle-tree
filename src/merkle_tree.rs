use sha256::{digest};



#[derive(Debug)]
pub struct MerkleTree {
    root: Node,
}

/// A Merkle Tree node which contains its hashed value,
/// height (0 corresponds to a leaf), left and right 
/// child nodes.
/// Leaf nodes has no childs.
#[derive(Debug, Clone)]
struct Node {
    hash: String,
    height: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn hash(data: String) -> String {
    digest(data)
}

impl MerkleTree {

    /// Creates a merkle tree from a vector of strings
    pub fn new(elements: Vec<String>) -> Self {
        let leafs = elements.iter().map(|e| Node{hash: hash(e.clone()), height: 0, left: None, right: None}).collect();
        let root = Self::build_tree(leafs)[0].clone();
        MerkleTree{root}
    }

    /// Takes a vector of nodes and builds the tree up to the root
    /// Returns a vector containing the root
    fn build_tree(nodes: Vec<Node>) -> Vec<Node> {
        let len = nodes.len();
        if len == 1 {
            return nodes;
        }
        let mut parent_nodes: Vec<Node> = Vec::new();
        let mut i = 0;

        // For each pair of two consecutive nodes (there are len/2 pairs),
        // we create the corresponding parent, which holds a reference
        // to both of these nodes.
        //
        // For example, if we have the following nodes
        // nodes = [1, 2, 3, 4]
        // then (1, 2) is a pair, so we create their parent.
        // the same goes for (3, 4). 
        for _ in 0..len/2 {
            let left_node = &nodes[i];
            let right_node = &nodes[i + 1];
            i += 2;

            let parent = Node{
                hash: hash(left_node.hash.clone() + &right_node.hash.clone()),
                height: left_node.height + 1,
                left: Some(Box::new(left_node.clone())),
                right: Some(Box::new(right_node.clone())),
            };
            parent_nodes.push(parent);
        }
        // Then we call recursively but with the parent nodes
        Self::build_tree(parent_nodes)
    }
}

