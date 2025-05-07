use std::path::Path;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

use env_logger::Builder;

use crate::structure::bst::BstNode;
use crate::structure::bst::BstNodeLink;
use crate::tool::generate_dotfile_bst;


pub fn commence() {
    customized_debug();
    
    println!("\n============================== Assignment 4 - Binary Search Tree ===============================\n");
    println!("    - Instruction: Before starting, please choose one of the following options");
    println!("      1. Use a defined generated graph");
    println!("      2. Create the graph from the start");
    println!("      3. Exit the program");

    let rootlink: BstNodeLink;
    let mut value: i32;

    loop {
        value = get_user_input("");

        match value {
            1 => {
                rootlink = get_predefined_tree();
                handle_main_menu(&rootlink);
            },
            2 => {
                rootlink = create_custom_tree();
                handle_main_menu(&rootlink);
            },
            3 => {
                terminate();
            },
            _ => {
                invalid_option(&value);

                continue;
            }
        }

        break;
    }
}

fn customized_debug() {
    Builder::new()
        .format(|buf, record| { writeln!(buf, "    {}", record.args()) })
        .filter_level(log::LevelFilter::Debug)
        .init();
}

fn get_user_input(prompt: &str) -> i32 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("    - Error: Failed to read input");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("    - Error: Invalid input, please type a number")
        }
    }
}

fn get_predefined_tree() -> BstNodeLink {
    let rootlink = BstNode::new_bst_nodelink(15);
    let query_keys = vec![
        5, 18,                      // 1st gen
        3, 7, 17, 20,               // 2nd gen
        2, 4, 6, 10, 16, 19, 25,    // 3rd gen
        1, 8, 11, 24,               // 4th gen
        9, 13, 23,                  // 5th gen
        12, 14, 22,                 // 6th gen
        21                          // 7th/last gen
    ];

    for key in query_keys {
        rootlink.borrow_mut().tree_insert(&rootlink, &key);
    }

    rootlink
}

fn create_custom_tree() -> BstNodeLink {
    let rootlink = BstNode::new_bst_nodelink(get_user_input("    - Instruction: Please enter a key value of the root node"));

    println!("\n============================================= Info =============================================\n");

    println!("    - The tree root with value {:?} is created successfully", rootlink.clone().borrow().key.unwrap());

    println!("\n================================================================================================\n");

    rootlink
}

fn handle_main_menu(rootlink: &BstNodeLink) {
    let mut value: i32;
    
    loop {
        println!("\n============================================= Menu =============================================\n");
        println!("    - Instruction: Please choose one of the following options provided by entering its number\n");
        println!("      1. Insert a new node");
        println!("      2. Delete a node");
        println!("      3. Find the successor of a node");
        println!("      4. Find the root node of the tree");
        println!("      5. Find the minimum node of the tree");
        println!("      6. Find the maximum node of the tree");
        println!("      7. Save the current graph");
        println!("      8. Exit the program");

        value = get_user_input("");

        match value {
            1 => tree_insert(rootlink),
            2 => tree_delete(rootlink),
            3 => find_successor(rootlink),
            4 => find_root(rootlink),
            5 => find_minimum(rootlink),
            6 => find_maximum(rootlink),
            7 => save_graph(rootlink),
            8 => {
                terminate();

                break;
            },
            _ => invalid_option(&value)
        }
    }
}

fn terminate() {
    println!("Exited");
}

fn invalid_option(value: &i32) {
    println!("    - Error: Invalid input, there's no option number {}", value);
}

fn tree_insert(rootlink: &BstNodeLink) {
    println!("\n============================================ Insert ============================================\n");

    let value = get_user_input("    - Instruction: Please enter a key value of the new node");

    println!("\n============================================= Info =============================================\n");

    let result = rootlink.clone().borrow().tree_search(&value);

    if let Some(exist) = result {
        println!("    - Unable to insert the key value of {}", value);
        println!("    - The node {:?} already existed", exist.clone().borrow().key);
    } else {
        rootlink
            .borrow_mut()
            .tree_insert(&rootlink, &value);
    }

    println!("\n================================================================================================\n");
}

fn tree_delete(rootlink: &BstNodeLink) {
    println!("\n============================================ Delete ============================================\n");

    let value = get_user_input("    - Instruction: Please enter a key value of the node to delete");

    println!("\n============================================= Info =============================================\n");

    rootlink
        .borrow_mut()
        .tree_delete(&value);

    println!("\n================================================================================================\n");
}

fn find_successor(rootlink: &BstNodeLink) {
    println!("\n====================================== Find the successor ======================================\n");

    let value = get_user_input("    - Instruction: Please enter a key value of the node in order to find its successor");

    println!("\n============================================= Info =============================================\n");

    let result = rootlink.clone().borrow().tree_search(&value);

    if let Some(exist) = result {
        BstNode::tree_successor(&exist);
    } else {
        println!("    - Node with key of {} does not exist, failed to get successor", value);
    }

    println!("\n================================================================================================\n");
}

fn find_root(rootlink: &BstNodeLink) {
    println!("\n============================================= Info =============================================\n");

    println!("    - The root node of the tree is {:?}", BstNode::get_root(&rootlink).borrow().key);

    println!("\n================================================================================================\n");   
}

fn find_minimum(rootlink: &BstNodeLink) {
    println!("\n============================================= Info =============================================\n");

    println!("    - The minimum node of the tree is {:?}", rootlink.borrow().minimum().borrow().key);

    println!("\n================================================================================================\n");
}

fn find_maximum(rootlink: &BstNodeLink) {
    println!("\n============================================= Info =============================================\n");

    println!("    - The maximum node of the tree is {:?}", rootlink.borrow().maximum().borrow().key);

    println!("\n================================================================================================\n");
}

fn save_graph(rootlink: &BstNodeLink) {
    println!("    - Instruction: Please define a name for file, along with the extension, for example 'example.dot'");

    let output_dir = Path::new("graph");
    let _ = fs::create_dir_all(output_dir);

    loop {
        let mut dot_path = String::new();

        io::stdin()
            .read_line(&mut dot_path)
            .expect("    - Error: Failed to read input");

        let dot_path = dot_path.trim();

        if dot_path.is_empty() {
            println!("    - Error: Filename cannot be empty");

            continue;
        }

        if !dot_path.ends_with(".dot") {
            println!("    - Error: Filename must end with .dot");

            continue;
        }

        if dot_path.contains(' ') {
            println!("    - Error: Use underscores instead of spaces");

            continue;
        }

        if !dot_path.chars().all(|c| 
            c.is_ascii_lowercase() || 
            c.is_numeric() || 
            c == '_' || 
            c == '.'
        ) {
            println!("    - Error: Use only lowercase letters, numbers, and underscores");

            continue;
        }

        let dot_full_path = output_dir.join(&dot_path);
        let png_full_path = output_dir.join(&dot_path.replace(".dot", ".png"));

        println!("\n============================================= Info =============================================\n");

        generate_dotfile_bst(&rootlink, dot_full_path.to_str().unwrap());

        println!("    - The graph has been written to {}", dot_full_path.display());

        match Command::new("dot")
            .arg("-Tpng")
            .arg(dot_full_path)
            .arg("-o")
            .arg(&png_full_path)
            .output() 
        {
            Ok(output) => {
                if output.status.success() {
                    println!("    - Successfully converted to PNG: {}", png_full_path.display());
                } else {
                    println!("    - {}", String::from_utf8_lossy(&output.stderr));
                }
            },
            Err(_) => {
                println!("    - Error: Failed to execute Graphviz");
            }
        }

        println!("\n================================================================================================\n");

        break;
    }
}