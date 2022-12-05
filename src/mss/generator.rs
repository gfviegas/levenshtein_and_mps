use std::ops::RangeInclusive;
use rand::Rng;

use super::binary_tree::{BinaryTree, BTNode, leaf_node};
use crate::debug::is_debug;

const LEAF_PROBABILITY: f32 = 0.01;
const VALUE_RANGE: RangeInclusive<i32> = -100000..=450000;

pub fn random_node(leaf_probability: f32, leaf_growth_rate: f32, taken_values: &mut Vec<i32>) -> BTNode<i32> {
    let mut rng = rand::thread_rng();

    let leaf_test: f32 = rand::random();
    let mut value: i32 = rng.gen_range(VALUE_RANGE);

    while taken_values.iter().any(|&i| i == value) {
        value = rng.gen_range(VALUE_RANGE);
    }

    taken_values.push(value);

    if is_debug() {
        println!("Generator ~ LT: {}, LP: {}, V: {}", leaf_test, leaf_probability, value);
    }
    
    if leaf_test < leaf_probability {
        return leaf_node(value);
    }

    let next_leaf_prob = leaf_probability * leaf_growth_rate;

    return BTNode::new(
        value,
        random_node(next_leaf_prob, leaf_growth_rate, taken_values),
        random_node(next_leaf_prob, leaf_growth_rate, taken_values)
    );
}


pub fn generate_random_tree(leaf_growth_rate: f32) -> BinaryTree<i32> {
    let mut taken_values: Vec<i32> = Vec::new();
    let tree = BinaryTree::new(random_node(LEAF_PROBABILITY, leaf_growth_rate, &mut taken_values));

    return tree;
}

