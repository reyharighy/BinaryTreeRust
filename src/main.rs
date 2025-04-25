mod structure;
mod tool;

use crate::structure::tree::Node;
use crate::structure::tree::NodeLink;
use crate::tool::generate_dotfile;

fn main() {
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
    if let Some(left_tree_extract)  = left_subtree {
        left_tree_extract.borrow_mut().add_left_child(left_tree_extract, 2);
        left_tree_extract.borrow_mut().add_right_child(left_tree_extract, 4);
    }

    //add new child values to the right subtree
    let right_subtree = &rootlink.borrow().right;
    if let Some(right_tree_extract) = right_subtree{
        right_tree_extract.borrow_mut().add_right_child(right_tree_extract, 10);
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
    let left_subtree_sibling = Node::get_sibling(&left_subtree.as_ref().unwrap());
    //println!("sibling of left subtree {:?}", left_subtree_sibling);
    
    //get the left subtree by value
    let left_subtree = rootlink.borrow().get_node_by_value(3);
    println!("left subtree seek by value{:?}", left_subtree);
    //get the left subtree by full properties
    //TODO

    //Discard the right subtree from parent
    //TODO

    //print the tree again
    main_tree_path = "prime_t3.dot";
    generate_dotfile(&left_subtree.unwrap(), main_tree_path);

    //Call tree depth function at this time
    //TODO

    //Call count_nodes function 
    //TODO

    //print the tree again
    main_tree_path = "prime_t4.dot";
    generate_dotfile(&rootlink, main_tree_path);
}

