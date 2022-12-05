mod debug;
mod levenshtein;
mod mss;

use std::io::{stdin, stdout, Write};

use levenshtein::brute_force;
use levenshtein::dynamic;


use mss::generator;
use mss::binary_tree::BinaryTree;
use mss::greedy;
use mss::divide_and_conquer;

fn test_levenshtein() {
    println!("~~ TESTING LEVENSHTEIN ~~");


    let mut x = String::new();
    print!("Please enter the first word: ");
    let _ = stdout().flush();

    stdin().read_line(&mut x).expect("Did not enter a correct string");

    let mut y = String::new();
    print!("Please enter the second word: ");
    let _ = stdout().flush();

    stdin().read_line(&mut y).expect("Did not enter a correct string");


    let (d1, e1) = brute_force::levenshtein_distance(&x, &y);
    println!("(BRUTE_FORCE) The Levenshtein Distance is: {} with {} evaluations", d1, e1);
    
    let (d2, e2) = dynamic::levenshtein_distance(&x, &y);
    println!("(DYNAMIC) The Levenshtein Distance is: {} with {} evaluations", d2, e2);
}

fn test_mss(print: bool, leaf_growth_rate: f32) -> (i32, i32, i32, i32) {
    let tree = generator::generate_random_tree(leaf_growth_rate);
    let tree_box = &Box::new(tree.head.expect("No head initialized."));

    let nodes_count = BinaryTree::nodes_count(tree_box);
    let tree_height: i32 = BinaryTree::height(tree_box);

    if print {
        println!("Nodes count: {}", nodes_count);
        BinaryTree::to_dot(tree_box);
    }

    let (greedy_sum, greedy_executions) = greedy::mss(tree_box);
    let (divide_and_conquer_sum, divide_and_conquer_executions) = divide_and_conquer::mss(tree_box);

    if print {
        println!("Greedy MSS: {} in {} evaluations", greedy_sum, greedy_executions);
        println!("Divide & Conquer MSS: {} in {} evaluations", divide_and_conquer_sum, divide_and_conquer_executions);
    }

    return (nodes_count, tree_height, greedy_executions, divide_and_conquer_executions);
}

fn run() {
    let x = 1;

    if x != 1 {
        test_levenshtein();
    } else {
        // test_mss(true)
        let mut leaf_growth_rate = 1.3;
        println!("nodes_count,greedy_sum,greedy_executions,divide_and_conquer_sum,divide_and_conquer_executions");

        for _ in 0..50 {
            let (n, m, x, y) = test_mss(false, leaf_growth_rate);
            println!("{},{},{},{}", n, m, x, y);

            leaf_growth_rate *= 1.005;
        }
    }
}

fn main() {
    // Spawn thread with explicit stack size
    let child = std::thread::Builder::new()
        .stack_size(7000 * 1024 * 1024)
        .spawn(run)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}