#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

enum TreeNodeState {
    FirstTimeNodeVisit,
    AllChildrenNodesDone
}

fn width_of_binary_tree_iterative(root: Option<Box<TreeNode>>) -> usize {
    // iterative depth first search right to left traversal
    if root.is_none() {
        return 0;
    }

    let mut stack: Vec<(Option<Box<TreeNode>>, TreeNodeState)> = Vec::new();
    let mut max_diameter = 0;
    let mut depths: HashMap<Option<Box<TreeNode>>, usize> = std::collections::HashMap::new();

    if let Some(root) = root {
        stack.push((Some(root), TreeNodeState::FirstTimeNodeVisit));
    }

    while let Some((node, state)) = stack.pop() {
        match (node, state) {
            (Some(n), TreeNodeState::FirstTimeNodeVisit) => {
                stack.push((Some(n.clone()), TreeNodeState::AllChildrenNodesDone));

                if n.right.is_some() {
                    stack.push((n.right.clone(), TreeNodeState::FirstTimeNodeVisit));
                }
                if n.left.is_some() {
                    stack.push((n.left.clone(), TreeNodeState::FirstTimeNodeVisit));
                }
            }
            (Some(n), TreeNodeState::AllChildrenNodesDone) => {
                let left_depth = depths.get(&n.left).cloned().unwrap_or(0);
                let right_depth = depths.get(&n.right).cloned().unwrap_or(0);
                max_diameter = max_diameter.max(left_depth + right_depth);
                depths.insert(Some(n), 1 + left_depth.max(right_depth));
            }
            _ => {}
        }
    }

    max_diameter
}

fn width_of_binary_tree_recursive(root: Option<Box<TreeNode>>) -> usize {
    fn dfs(node: &Option<Box<TreeNode>>, max_diameter: &mut usize) -> usize {
        if let Some(n) = node {
            let left_depth = dfs(&n.left, max_diameter);
            let right_depth = dfs(&n.right, max_diameter);
            *max_diameter = (*max_diameter).max(left_depth + right_depth);
            1 + left_depth.max(right_depth)
        } else {
            0
        }
    }

    let mut max_diameter = 0;
    dfs(&root, &mut max_diameter);
    max_diameter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        // L0
        let root = Box::new(TreeNode::new(1));
        assert_eq!(width_of_binary_tree_iterative(Some(root.clone())), 0);
        assert_eq!(width_of_binary_tree_recursive(Some(root)), 0);
    }

    #[test]
    fn test_1() {
        // L0
        let mut root = Box::new(TreeNode::new(1));
        // L1
        root.left = Some(Box::new(TreeNode::new(2)));
        assert_eq!(width_of_binary_tree_iterative(Some(root.clone())), 1);
        assert_eq!(width_of_binary_tree_recursive(Some(root)), 1);
    }

    #[test]
    fn test_cross_root_chain() {
        // L0
        let mut root = Box::new(TreeNode::new(1));
        // L1
        root.left = Some(Box::new(TreeNode::new(2)));
        root.right = Some(Box::new(TreeNode::new(3)));
        // L2
        root.left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(4)));
        root.left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(5)));
        root.right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(8)));
        // L3
        root.left.as_mut().unwrap().right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(6)));
        root.left.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(7)));

        assert_eq!(width_of_binary_tree_iterative(Some(root.clone())), 5);
        assert_eq!(width_of_binary_tree_recursive(Some(root)), 5);
    }

    #[test]
    fn test_inter_branch_chain() {
        // L0
        let mut root = Box::new(TreeNode::new(1));
        // L1
        root.left = Some(Box::new(TreeNode::new(2)));
        root.right = Some(Box::new(TreeNode::new(3)));
        // L2
        root.left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(4)));
        root.left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(5)));
        //root.right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(8)));
        // L3
        root.left.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(6)));
        root.left.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(7)));
        // L4
        root.left.as_mut().unwrap().left.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(8)));
        root.left.as_mut().unwrap().right.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(9)));

        assert_eq!(width_of_binary_tree_iterative(Some(root.clone())), 6);
        assert_eq!(width_of_binary_tree_recursive(Some(root)), 6);
    }
}