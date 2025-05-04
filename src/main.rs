mod structure;
mod tool;

use crate::structure::bst::BstNode;
use crate::structure::tree::Node;
use crate::structure::tree::NodeLink;
use crate::structure::bst::BstNodeLink;
use crate::tool::generate_dotfile;
use crate::tool::generate_dotfile_bst;

// related to logger
use env_logger::Builder;
use std::io::Write;

fn main() {
    //turn on to test the old code
    //test_binary_tree();

    // turn on to test BST module
    // test_binary_search_tree();

    // turn on to use logger
    customized_debug();

    test_binary_search_tree_new_assignment();
}

fn test_binary_search_tree_new_assignment() {
    let rootlink= BstNode::new_bst_nodelink(15);

    // insert each value to the root sequentially
    let query_keys = vec![
        5, 18, 
        3, 7, 17, 20, 
        2, 4, 6, 10, 16, 19, 
        1, 8, 11, 
        9, 13, 
        12, 14,
        25, 24, 23, 22, 21
    ];

    for key in query_keys {
        rootlink.borrow_mut().tree_insert(&rootlink, &key);
    }

    // PASSED
    for key in vec![1, 4, 6, 9, 12, 14, 16, 19, 21] {
        rootlink.borrow_mut().tree_delete(&key);
    }

    // PASSED
    for key in vec![2, 8, 11, 17, 22, 23, 24, 25] {
        rootlink.borrow_mut().tree_delete(&key);
    }

    // PASSED
    for key in vec![3, 10, 13] {
        rootlink.borrow_mut().tree_delete(&key);
    }

    // PASSED
    for key in vec![5, 7, 18, 20] {
        rootlink.borrow_mut().tree_delete(&key);
    }

    // PASSED
    rootlink.borrow_mut().tree_delete(&15);

    // print the tree at this time
    let main_tree_path = "bst_graph.dot";
    generate_dotfile_bst(&rootlink, main_tree_path);

    // root, min, and max test
    println!("\n================= root, min, and max ===================");
    println!("- root node is {:?}", BstNode::get_root(&rootlink).borrow().key);
    println!("- minimum node is {:?}", rootlink.borrow().minimum().borrow().key);
    println!("- maximum node is {:?}", rootlink.borrow().maximum().borrow().key);
    println!("========================================================");

    // successor test
    for key in 1..=25 {
        if let Some(node) = rootlink.borrow().tree_search(&key) {
            BstNode::tree_successor(&node);
        } else {
            // comment the line below to skip non-existent key, otherwise uncomment
            println!("\nnode with key of {} does not exist, failed to get successor", key);
        }
    }
}

#[allow(dead_code)]
fn test_binary_search_tree(){
    let rootlink: BstNodeLink = BstNode::new_bst_nodelink(15);
    rootlink.borrow_mut().add_left_child(&rootlink, 6);
    rootlink.borrow_mut().add_right_child(&rootlink, 18);

    //add right subtree
    let right_subtree: &Option<BstNodeLink> = &rootlink.borrow().right;
    if let Some(right_tree_extract) = right_subtree {
        right_tree_extract
            .borrow_mut()
            .add_left_child(right_tree_extract, 17);
        right_tree_extract
            .borrow_mut()
            .add_right_child(right_tree_extract, 20);
    }

    //add left subtree
    let left_subtree: &Option<BstNodeLink> = &rootlink.borrow().left;
    if let Some(left_tree_extract) = left_subtree {
        left_tree_extract
            .borrow_mut()
            .add_left_child(left_tree_extract, 3);
        left_tree_extract
            .borrow_mut()
            .add_right_child(left_tree_extract, 7);

        //add left subtree terminal
        let left_subtree_terminal = &left_tree_extract.borrow().left;
        if let Some(terminal_left_tree_link) = left_subtree_terminal{
            terminal_left_tree_link.borrow_mut().add_left_child(terminal_left_tree_link, 2);
            terminal_left_tree_link.borrow_mut().add_right_child(terminal_left_tree_link, 4);
        }
        //add 2nd level right subtree of node 7
        let second_right_subtree = &left_tree_extract.borrow().right;
        if let Some(second_right_subtree_link) = second_right_subtree{
            second_right_subtree_link.borrow_mut().add_right_child(second_right_subtree_link, 13);

            let third_left_subtree = &second_right_subtree_link.borrow().right;
            if let Some(third_left_subtree_link) = third_left_subtree{
                third_left_subtree_link.borrow_mut().add_left_child(third_left_subtree_link, 9);
            }
        }
    }

    //print the tree at this time
    let main_tree_path = "bst_graph.dot";
    generate_dotfile_bst(&rootlink, main_tree_path);

    //tree search test
    let search_keys = vec![9, 21];

    for &key in search_keys.iter() {
        print!("tree search result of node with key of {} is ", key);

        if let Some(_node_result) = rootlink.borrow().tree_search(&key) {
            println!("found");
        } else {
            println!("not found");
        }
    }

    //min test
    let min_node = rootlink.borrow().minimum();
    println!("minimum node of the tree is {:?}", min_node.borrow().key);

    //max test
    let max_node = rootlink.borrow().maximum();
    println!("maximum node of the tree is {:?}", max_node.borrow().key);

    //root node get test
    let root_node = BstNode::get_root(&max_node);
    println!("root node of the tree is {:?}", root_node.borrow().key);

    //successor test
    for key in 1..=21 {
        if let Some(node) = rootlink.borrow().tree_search(&key) {
            println!("\n================ successor of node ({}) =================", key);

            if let Some(successor) = BstNode::tree_successor(&node) {
                println!("============= so, the successor is {:?} =============", successor.borrow().key);
            } else {
                println!("============ so, the successor is not found =============");
            }
        }

        else {
            // comment the line below to skip non-existent key, otherwise uncomment
            println!("node with key of {} does not exist, failed to get successor", key);
        }
    }
}

#[allow(dead_code)]
fn customized_debug() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}",
                record.args() // just the log message itself
            )
        })
        .filter_level(log::LevelFilter::Debug)
        .init();
}

#[allow(dead_code)]
fn test_binary_tree() {
    //create the nodelink of the root node
    let rootlink: NodeLink = Node::new_nodelink(5);

    //add a new left node value
    rootlink.borrow_mut().add_left_child(&rootlink, 3);
    //add a new right node value
    rootlink.borrow_mut().add_right_child(&rootlink, 7);

    //println!("{:?}", rootlink);

    //print the tree at this time
    let mut main_tree_path = "prime.dot";
    generate_dotfile(&rootlink, main_tree_path);

    //add new child values to the left subtree
    let left_subtree = &rootlink.borrow().left;
    if let Some(left_tree_extract) = left_subtree {
        left_tree_extract
            .borrow_mut()
            .add_left_child(left_tree_extract, 2);
        left_tree_extract
            .borrow_mut()
            .add_right_child(left_tree_extract, 4);
    }

    //add new child values to the right subtree
    let right_subtree = &rootlink.borrow().right;
    if let Some(right_tree_extract) = right_subtree {
        right_tree_extract
            .borrow_mut()
            .add_right_child(right_tree_extract, 10);
    }

    //print the tree again, now been added with more values
    main_tree_path = "prime_t2.dot";
    generate_dotfile(&rootlink, main_tree_path);

    //Call tree depth function at this time
    let recorded_depth = rootlink.borrow().tree_depth();
    println!("Current tree depth: {0}", recorded_depth);

    //Call count_nodes function
    let total_nodes = rootlink.borrow().count_nodes();
    println!("Amount of nodes in current tree: {0}", total_nodes);

    //Call count_nodes_by_nodelink function, supplied right subtree as parameter
    //TODO
    let subtree_count = Node::count_nodes_by_nodelink(&right_subtree.clone().unwrap(), 0);
    println!("Amount of nodes in current subtree: {0}", subtree_count);

    //Get the sibling of the leftsubtree from parent
    let _left_subtree_sibling = Node::get_sibling(&left_subtree.as_ref().unwrap());
    //println!("sibling of left subtree {:?}", left_subtree_sibling);

    //get the left subtree by value
    let left_subtree = rootlink.borrow().get_node_by_value(3);
    println!("left subtree seek by value {:?}", left_subtree);
    //get the left subtree by full properties
    let another_left_subtree = rootlink
        .borrow()
        .get_node_by_full_property(&left_subtree.as_ref().unwrap());
    println!(
        "left subtree seek by full property {:?}",
        another_left_subtree
    );

    //Discard the right subtree from parent
    let rootlink2 = rootlink.borrow().get_nodelink_copy();

    let flag = rootlink2.borrow_mut().discard_node_by_value(3);
    println!("status of node deletion: {0}", flag);

    //print the tree again
    main_tree_path = "prime_t3.dot";
    generate_dotfile(&rootlink2, main_tree_path);

    //Call tree depth function at this time
    //TODO
    let depth_now = rootlink2.borrow().tree_depth();
    println!("Depth after discard {0}", depth_now);

    //Call count_nodes function
    let count_now = rootlink2.borrow().count_nodes();
    println!("Count nodes after discard {0}", count_now);

    //print the tree again
    main_tree_path = "prime_t4.dot";
    generate_dotfile(&rootlink, main_tree_path);
}
