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
    let mut left_subtree = &rootlink.borrow().left;
    if let Some(left_tree_extract)  = left_subtree {
        left_tree_extract.borrow_mut().add_left_child(left_tree_extract, 4);
    }

    //print the tree again, now been added with more values
    main_tree_path = "prime_t2";
    generate_dotfile(&rootlink, main_tree_path);

    //add new child values to the right subtree
    //TODO

    //Call tree depth function at this time
    //TODO

    //Call count_nodes function
    //TODO

    //Call count_nodes_by_nodelink function, supplied right subtree as parameter
    //TODO

    //Get the sibling of the leftsubtree from parent
    //TODO

    //get the left subtree by value
    //TODO

    //get the left subtree by full properties
    //TODO

    //Discard the right subtree from parent
    //TODO

    //print the tree again
    main_tree_path = "prime_t3";
    generate_dotfile(&rootlink, main_tree_path);

    //Call tree depth function at this time
    //TODO

    //Call count_nodes function 
    //TODO

    //print the tree again
    main_tree_path = "prime_t4";
    generate_dotfile(&rootlink, main_tree_path);
}

