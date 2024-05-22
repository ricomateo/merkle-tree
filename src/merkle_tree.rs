use sha256::digest;

#[derive(Debug, PartialEq)]
pub struct MerkleTree {
    root: Node,
}

/// A (merkle tree) Node.
/// Each node contains:
///   * hashed value,
///   * height in the tree (0 corresponds to a leaf),
///   * left and right child nodes (leaf nodes has no childs, i.e, left = right = None)
#[derive(Debug, Clone, PartialEq)]
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
        let leafs = elements
            .iter()
            .map(|e| Node {
                hash: hash(e.clone()),
                height: 0,
                left: None,
                right: None,
            })
            .collect();
        let root = Self::build_tree(leafs)[0].clone();
        MerkleTree { root }
    }

    /// Takes a vector of nodes and builds the tree up to the root
    /// Returns a vector containing the root
    fn build_tree(nodes: Vec<Node>) -> Vec<Node> {
        let len = nodes.len();
        if len == 1 {
            return nodes;
        }
        let mut parent_nodes: Vec<Node> = Vec::new();

        // For each pair of two consecutive nodes (there are len/2 pairs),
        // we create the corresponding parent, which holds a reference
        // to both of these nodes.
        //
        // For example, if we have the following nodes
        // nodes = [1, 2, 3, 4]
        // then (1, 2) is a pair, so we create their parent.
        // the same goes for (3, 4).
        for i in (0..len).step_by(2) {
            let left_node = &nodes[i];
            let right_node = &nodes[i + 1];

            let parent = Node {
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates a Merkle Tree node by node (tree_by_hand) from a vector of strings and compares
    /// it to a Merkle Tree created with the constructor
    #[test]
    fn merkle_tree_is_created_correctly() {
        let elements = vec![
            String::from("A"),
            String::from("B"),
            String::from("C"),
            String::from("D"),
        ];

        let leaf_node_0 = Node {
            hash: hash(elements[0].clone()),
            height: 0,
            left: None,
            right: None,
        };
        let leaf_node_1 = Node {
            hash: hash(elements[1].clone()),
            height: 0,
            left: None,
            right: None,
        };
        let leaf_node_2 = Node {
            hash: hash(elements[2].clone()),
            height: 0,
            left: None,
            right: None,
        };
        let leaf_node_3 = Node {
            hash: hash(elements[3].clone()),
            height: 0,
            left: None,
            right: None,
        };

        let node01 = Node {
            hash: hash(leaf_node_0.hash.clone() + &leaf_node_1.hash),
            height: 1,
            left: Some(Box::new(leaf_node_0)),
            right: Some(Box::new(leaf_node_1)),
        };
        let node23 = Node {
            hash: hash(leaf_node_2.hash.clone() + &leaf_node_3.hash),
            height: 1,
            left: Some(Box::new(leaf_node_2)),
            right: Some(Box::new(leaf_node_3)),
        };
        let root = Node {
            hash: hash(node01.hash.clone() + &node23.hash),
            height: 2,
            left: Some(Box::new(node01)),
            right: Some(Box::new(node23)),
        };

        let tree_by_hand = MerkleTree { root: root };

        let tree = MerkleTree::new(elements);

        assert_eq!(tree_by_hand, tree);
    }
}
