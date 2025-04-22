use crate::structure::tree::NodeLink;
use crate::structure::tree::Node;
use std::fs::File;
use std::io::{Error, Write};

/**
 * @root: root node of the tree in NodeLink Type
 * @output_path: write the graphviz structure to output_path
 * Generate graphviz dot file given a NodeLink, you will traverse from root to all leaves incrementally,
 * as you proceed wrote the progress to dot file
 */
pub fn generate_dotfile(root: &NodeLink, output_path: &str){
    let graph_name = "tree";
    let preamble = "digraph".to_owned() + graph_name + "{";
    let epilogue = "}";
    let graph_arrangement = node_traversal(root, "".to_string());
    //traverse the node as usual
    let final_text = preamble + &graph_arrangement + epilogue;
    let mut output = File::create(output_path).expect("Failed to create");
    output.write_all(final_text.as_bytes());
}

/**
 * We will print string as we traverse, node by node
 * at most a line per node printing, e.g: a--b;
 * traversal mode in BFS
 */
fn node_traversal(node: &NodeLink, prev_info: String) -> String{
    let mut new_info = prev_info;
    //get the parent, guaranteed to has parent since we traverse from upper node
    let strong_parent = Node::upgrade_weak_to_strong(node.borrow().parent.clone());
    //we print the child nodes first
    let left_child = &strong_parent;
    new_info += &print_child(&strong_parent.clone().unwrap(), left_child.as_ref(), new_info.clone());
    let right_child = &strong_parent;
    new_info += &print_child(&strong_parent.clone().unwrap(), right_child.as_ref(), new_info.clone());
    //now we need to traverse deeper
    if left_child.is_some(){
        new_info += &node_traversal(&left_child.as_ref().unwrap(), new_info.clone());
    }
    if right_child.is_some(){
        new_info += &node_traversal(&right_child.as_ref().unwrap(), new_info.clone());
    }
    return new_info;
}

fn print_child(parent_node: &NodeLink, child_node: Option<&NodeLink>, prev_info: String) -> String{
    let mut new_info = prev_info;
    if let Some(child) = child_node {
        //concat parent
        new_info += &parent_node.borrow().value.to_string();
        //next_info += node.borrow().parent.unwrap().value;
        new_info += "--";
        new_info += &child.borrow().value.to_string();
        new_info += ";";
    }
    return new_info;
}

/*
pub fn graph_dotfile_string(root: &NodeLink) -> String{
    ""
}
*/