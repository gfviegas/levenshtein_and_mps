use super::binary_tree::BTNode;
use crate::debug::is_debug;

pub fn mps(node: &Box<BTNode<i32>>) -> (i32, i32) {
    let mut evaluations = 0;
    let mut sum = 0;

    let mut current_node = node;

    loop {
        evaluations += 1;
        sum += current_node.value;

        if is_debug() {
            println!(
                "Evaluation #{}, Current Node: {}, Current Sum: {}",
                evaluations, current_node.value, sum
            );
        }

        if let (Some(left), Some(right)) = (&current_node.left, &current_node.right) {
            if is_debug() {
                println!("\t\tCASE 1, Left: {}, Right: {}", left.value, right.value);
            }
            current_node = if left.value >= right.value {
                left
            } else {
                right
            };
            continue;
        }

        if let (true, Some(left)) = (current_node.right.is_none(), &current_node.left) {
            if is_debug() {
                println!("\t\tCASE 2, Picking Left: {}", left.value);
            }
            current_node = left;
            continue;
        }

        if let (true, Some(right)) = (current_node.left.is_none(), &current_node.right) {
            if is_debug() {
                println!("\t\tCASE 3, Picking Right: {}", right.value);
            }
            current_node = right;
            continue;
        }

        break;
    }

    return (sum, evaluations);
}
