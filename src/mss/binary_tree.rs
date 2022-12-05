type ChildNode<T> = Option<Box<BTNode<T>>>;

#[derive(Debug)]
pub struct BTNode<T> {
    pub left: ChildNode<T>,
    pub right: ChildNode<T>,
    pub value: T
}

impl BTNode<i32> {
    pub fn new(value: i32, l: BTNode<i32>, r: BTNode<i32>) -> Self {
        BTNode::<i32> {  
            value,
            left: Some(Box::new(l)),
            right: Some(Box::new(r))
        }
    }
}

pub fn leaf_node(value: i32) -> BTNode<i32> {
    BTNode {
        value, 
        left: None, 
        right: None
    }
}

pub struct BinaryTree<T> {
    pub head: Option<BTNode<T>>
}

fn print_dot_notation(node: &Box<BTNode<i32>>) {
    if let Some(left) = &node.left {
        println!("\t{} -> {};", node.value, left.value);
        print_dot_notation(left);
    }
    
    if let Some(right) = &node.right {
        println!("\t{} -> {};", node.value, right.value);
        print_dot_notation(right);
    }
}

fn count_nodes(node: &Box<BTNode<i32>>, count: &mut i32) {
    *count += 1;

    if let Some(left) = &node.left {
        count_nodes(left, count);
    }
    
    if let Some(right) = &node.right {
        count_nodes(right, count);
    }
}

impl BinaryTree<i32> {
    pub fn new(head: BTNode<i32>) -> Self {
        BinaryTree::<i32> { head: Some(head) }
    }
    
    // pub fn print(node: &Box<BTNode<i32>>) {
    //     if let Some(left) = &node.left {
    //         BinaryTree::print(left);
    //     }
        
    //     print!("{} ", node.value);
        
    //     if let Some(right) = &node.right {
    //         BinaryTree::print(right);
    //     }
    // }

    pub fn to_dot(node: &Box<BTNode<i32>>) {
        println!("\n\n Copy the contents on the row below onwards: \n\n");
        println!("digraph BST {{\nnode [fontname=\"Arial\"];\n");
        
        print_dot_notation(node);
        
        println!("\n}}\n");
    }

    pub fn nodes_count(node: &Box<BTNode<i32>>) -> i32 {
        let mut count = 0;
        count_nodes(node, &mut count);

        return count;
    }

    pub fn height(node: &Box<BTNode<i32>>) -> i32 {
        let mut left_height: i32 = 0;
        let mut right_height: i32 = 0;
        
        if let Some(left) = &node.left {
            left_height = BinaryTree::height(left);
        }
        
        if let Some(right) = &node.right {
            right_height = BinaryTree::height(right);
        }

        return std::cmp::max(left_height, right_height) + 1;
    }
}