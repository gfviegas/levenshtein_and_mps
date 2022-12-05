use super::binary_tree::BTNode;
use crate::debug::is_debug;
use std::cmp::max;

fn max_subsum(node: &Box<BTNode<i32>>, evaluations: &mut i32) -> i32 {
    *evaluations += 1;

    if is_debug() {
        println!("Evaluation #{}, Current Node: {}, Current Sum: ?", evaluations, node.value);
    }
    
    if node.left.is_none() && node.right.is_none() {
        if is_debug() {
            println!("CASE 0, Returning value {}", node.value);
        }

        return node.value;
    }

    if let (Some(left), Some(right)) = (&node.left, &node.right) {
            if is_debug() {
                println!("\t\tCASE 1, Picking the max between left and right subtrees");
            }
            
            let left_mss = max_subsum(left, evaluations);
            let right_mss = max_subsum(right, evaluations);

            return max(left_mss, right_mss) + node.value;
        }

    if let (true, Some(left)) = (node.right.is_none(), &node.left) {
        if is_debug() {
            println!("\t\tCASE 2, Picking Left: {}", left.value);
        }
        
        return max_subsum(left, evaluations);
    }
    
    if let (true, Some(right)) = (node.left.is_none(), &node.right) {
        if is_debug() {
            println!("\t\tCASE 3, Picking Right: {}", right.value);
        }
        
        return max_subsum(right, evaluations);
    }
    
    panic!("Invalid condition");
}


pub fn mss(node: &Box<BTNode<i32>>) -> (i32, i32) {
    let mut evaluations = 0;
    let sum = max_subsum(node, &mut evaluations);

    return (sum, evaluations);
}