use std::cell::RefCell;
use std::rc::{Rc, Weak};
use log::debug;

pub type BstNodeLink = Rc<RefCell<BstNode>>;
pub type WeakBstNodeLink = Weak<RefCell<BstNode>>;

//this package implement BST wrapper
#[derive(Debug, Clone)]
pub struct BstNode {
    pub key: Option<i32>,
    pub parent: Option<WeakBstNodeLink>,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
}

impl BstNode {
    //private interface
    fn new(key: i32) -> Self {
        BstNode {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_bst_nodelink(value: i32) -> BstNodeLink {
        let currentnode = BstNode::new(value);
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    /**
     * Get a copy of node link
     */
    pub fn get_bst_nodelink_copy(&self) -> BstNodeLink {
        Rc::new(RefCell::new(self.clone()))
    }

    fn downgrade(node: &BstNodeLink) -> WeakBstNodeLink {
        Rc::<RefCell<BstNode>>::downgrade(node)
    }

    //private interface
    fn new_with_parent(parent: &BstNodeLink, value: i32) -> BstNodeLink {
        let mut currentnode = BstNode::new(value);
        currentnode.parent = Some(BstNode::downgrade(parent));
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    //add new left child, set the parent to current_node_link
    pub fn add_left_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.left = Some(new_node);
    }

    //add new right child, set the parent to current_node_link
    pub fn add_right_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.right = Some(new_node);
    }

    //search the current tree which node fit the value
    pub fn tree_search(&self, value: &i32) -> Option<BstNodeLink> {
        if let Some(key) = self.key {
            if key == *value {
                return Some(self.get_bst_nodelink_copy());
            }
            if *value < key && self.left.is_some() {
                return self.left.as_ref().unwrap().borrow().tree_search(value);
            } else if self.right.is_some() {
                return self.right.as_ref().unwrap().borrow().tree_search(value);
            }
        }
        //default if current node is NIL
        None
    }

    /**seek minimum by recurs
     * in BST minimum always on the left
     */
    pub fn minimum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(left_node) = &self.left {
                return left_node.borrow().minimum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    pub fn maximum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(right_node) = &self.right {
                return right_node.borrow().maximum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    /**
     * Return the root of a node, return self if not exist
     */
    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone());
        if parent.is_none() {
            return node.clone();
        }
        return BstNode::get_root(&parent.unwrap());
    }

    /**
     * Find node successor according to the book
     * Should return None, if x_node is the highest key in the tree
     */
    pub fn tree_successor(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        debug!("- Find the successor of node {:?}", x_node.borrow().key);

        if let Some(right_node) = &x_node.borrow().right {
            let minimum = Some(right_node.borrow().minimum());
            let node = minimum.clone().unwrap().clone();

            debug!("- The node {:?} has a right child", x_node.borrow().key);
            debug!("- Then, take the right child as a subtree");
            debug!("- The minimum node of that subtree is {:?}", node.borrow().key);
            debug!("- So, the successor is {:?}", node.borrow().key);

            return minimum;
        } else {
            debug!("- The node {:?} does not have a right child", x_node.borrow().key);

            let mut x_node = x_node.clone();
            let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());

            while let Some(y_ref) = y_node {
                debug!("- The node {:?} has a parent node {:?}", x_node.borrow().key, y_ref.borrow().key);

                if let Some(ref left_child) = y_ref.borrow().left {
                    if BstNode::is_node_match(left_child, &x_node) {
                        debug!("- And, the node {:?} is the left child of that parent", x_node.borrow().key);
                        debug!("- So, the successor is {:?}", y_ref.clone().borrow().key);

                        return Some(y_ref.clone());
                    }
                }

                debug!("- But, the node {:?} is the right child", x_node.borrow().key);
                debug!("- Traverse upward now to the node {:?}", y_ref.borrow().key);

                x_node = y_ref.clone();
                y_node = BstNode::upgrade_weak_to_strong(y_ref.borrow().parent.clone());
            }

            debug!("- The node {:?} does not have a parent, it's the root", x_node.borrow().key);
            debug!("- So, the successor is not found");
            None    
        }
    }

    pub fn tree_insert(&mut self, rootlink: &BstNodeLink, value: &i32) {
        debug!("- Insert a new node with the key value of {value}");

        let mut current_node;
        let mut y_node: BstNodeLink;
        let mut x_node = BstNode::get_root(&self.get_bst_nodelink_copy());
        let new_node = BstNode::new(*value);

        debug!("- Start traverse from the root node {:?}", x_node.clone().borrow().key);

        while let Some(exist) = x_node.clone().borrow().key {
            y_node = BstNode::tree_search(&self, &exist).unwrap();
            current_node = y_node.clone();

            if *value < exist {
                debug!("- {value} is less than the key value of node {:?}", y_node.borrow().key);

                if let Some(left) = y_node.borrow().left.clone() {
                    let left_node = current_node.borrow().left.clone().unwrap().borrow().key;
                    x_node = left;

                    debug!("- The node {:?} has a left child with value {:?}", current_node.borrow().key, left_node);
                    debug!("- Traverse down to the node {:?}", left_node);
                } else {
                    debug!("- The node {:?} does not have a left child", y_node.borrow().key);
                    debug!("- Found the insert point for the new node");

                    break;
                }
            } else {
                debug!("- {value} is greater than the key value of node {:?}", y_node.borrow().key);

                if let Some(right) = y_node.borrow().right.clone() {
                    let right_node = current_node.borrow().right.clone().unwrap().borrow().key;
                    x_node = right;

                    debug!("- The node {:?} has a right child with the key value {:?}", current_node.borrow().key, right_node);
                    debug!("- Traverse down to the node {:?}", right_node);
                } else {
                    debug!("- The node {:?} does not have a right child", y_node.borrow().key);
                    debug!("- Found the insert point for the new node");

                    break;
                }
            }
        }

        if new_node.key < x_node.borrow().key {
            if let Some(_) = self.left.clone() {
                x_node.borrow_mut().add_left_child(&x_node, new_node.key.unwrap());
            } else {
                self.left = Some(BstNode::new_with_parent(rootlink, new_node.key.unwrap()));
            }

            debug!("- Insert the node {:?} as the left child", new_node.key);
        } else {
            if let Some(_) = self.right.clone() {
                x_node.borrow_mut().add_right_child(&x_node, new_node.key.unwrap());
            } else {
                self.right = Some(BstNode::new_with_parent(rootlink, new_node.key.unwrap()));
            }

            debug!("- Insert the node {:?} as the right child", new_node.key);
        }

        debug!("- Insertion is complete");
    }

    pub fn tree_delete(&mut self, value: &i32) {
        debug!("- Try to delete a node with the key value of {}", value);

        if let Some(replaced) = self.tree_search(value) {
            if BstNode::is_node_match(&replaced, &self.get_bst_nodelink_copy()) {
                debug!("- The node {:?} is the root of the tree", replaced.clone().borrow().key);
                debug!("- Set a pointer from the root node");

                let successor = BstNode::tree_successor(&replaced);

                debug!("- Change the node {:?} with a copy of node {:?}", replaced.clone().borrow().key, successor.clone().unwrap().borrow().key);

                self.transplant(&self.get_bst_nodelink_copy(), &replaced, successor);
            }

            else {
                debug!("- The node {:?} is not the root of the tree, which is the node {:?}", replaced.clone().borrow().key, self.key);
                debug!("- Set a pointer from the root node");

                self.tree_pointer(&replaced);
            }
        }

        else { debug!("- There's no node with such key value of {} in the tree, failed to delete", value); }
    }

    fn tree_pointer(&mut self, replaced: &BstNodeLink) {
        if replaced.borrow().key.unwrap() < self.key.unwrap() {
            debug!("- The node {:?} is less than the node {:?}", replaced.borrow().key, self.key);
            debug!("- Peek the left child of the node {:?}, and find node {:?}", self.key, self.left.clone().unwrap().borrow().key);

            if !BstNode::is_node_match(&self.left.clone().unwrap(), replaced) {
                debug!("- Traverse downward the current pointer {:?} to the left", self.key);

                self.left.as_ref().unwrap().borrow_mut().tree_pointer(replaced);
            }

            else { 
                debug!("- Find an entry point from the current pointer");

                self.delete("left"); 
            }
        }

        else {
            debug!("- The node {:?} is greater than the node {:?}", replaced.borrow().key, self.key);
            debug!("- Peek the right child of the node {:?}, and find node {:?}", self.key, self.right.clone().unwrap().borrow().key);

            if !BstNode::is_node_match(&self.right.clone().unwrap(), replaced) {
                debug!("- Traverse downward the current pointer {:?} to the right", self.key);

                self.right.as_ref().unwrap().borrow_mut().tree_pointer(replaced);   
            }

            else {
                debug!("- Find an entry point from the current pointer");

                self.delete("right");
            }
        }
    }

    fn delete(&mut self, entry_point: &str) {
        let replaced: Option<BstNodeLink>;

        if entry_point == "left" { replaced = self.left.clone(); } 
        else { replaced = self.right.clone(); }

        if let Some(replaced) = replaced {
            if replaced.borrow().left.is_none() && replaced.borrow().right.is_none() {
                debug!("- The node does not have any children");
                debug!("- Change the node {:?} with None", replaced.borrow().key);

                self.transplant(&replaced, &replaced, None);
            } else if replaced.borrow().left.is_none() {
                debug!("- The node has a child node {:?} on the right", replaced.borrow().right.clone().unwrap().borrow().key);
                debug!("- Change the node {:?} with a copy of node {:?}", replaced.borrow().key, replaced.borrow().right.clone().unwrap().borrow().key);

                self.transplant(&replaced, &replaced, replaced.borrow().right.clone());
            } else if replaced.borrow().right.is_none() {
                debug!("- The node has a child node {:?} on the left", replaced.borrow().left.clone().unwrap().borrow().key);
                debug!("- Change the node {:?} with a copy of node {:?}", replaced.borrow().key, replaced.borrow().left.clone().unwrap().borrow().key);

                self.transplant(&replaced, &replaced, replaced.borrow().left.clone());
            } else {
                debug!("- The node has two children");

                let successor = BstNode::tree_successor(&replaced);
                let successor_parent = BstNode::upgrade_weak_to_strong(successor.clone().unwrap().borrow().parent.clone());

                if BstNode::is_node_match(&successor_parent.unwrap(), &replaced) {
                    debug!("- The node {:?} has a direct relationship with the successor node {:?}", replaced.clone().borrow().key, successor.clone().unwrap().borrow().key);
                    debug!("- Change the node {:?} with a copy of node {:?}", replaced.borrow().key, successor.clone().unwrap().borrow().key);
                    debug!("- Inspect the successor node {:?} by moving the current pointer, which is the node {:?}", successor.clone().unwrap().borrow().key, self.key);

                    self.transplant(&replaced, &replaced, successor.clone());
                    let right_successor = successor.as_ref().unwrap().borrow().right.clone();

                    if right_successor.is_some() {
                        right_successor.as_ref().unwrap().borrow_mut().parent = Some(BstNode::downgrade(&successor.clone().unwrap()));
                    }
                } else {
                    debug!("- The node {:?} does not have a direct relationship with the successor node {:?}", replaced.clone().borrow().key, successor.clone().unwrap().borrow().key);
                    debug!("- Change the node {:?} with a copy of node {:?}", replaced.borrow().key, successor.clone().unwrap().borrow().key);
                    debug!("- Inspect the successor node {:?} by moving the current pointer, which is the node {:?}", successor.clone().unwrap().borrow().key, self.key);

                    self.transplant(&replaced, &successor.clone().unwrap(), successor.as_ref().unwrap().borrow().right.clone());

                    if BstNode::is_node_match(&self.left.clone().unwrap(), &replaced) {
                        replaced.borrow_mut().key = successor.clone().unwrap().borrow().key;
                    }

                    if BstNode::is_node_match(&self.right.clone().unwrap(), &replaced) {
                        replaced.borrow_mut().key = successor.clone().unwrap().borrow().key;
                    }
                }
            }
        }
    }

    fn transplant(&mut self, reference: &BstNodeLink, replaced: &BstNodeLink, replacement: Option<BstNodeLink>) {
        if let Some(ref replacement) = replacement {
            replacement.borrow_mut().parent = replaced.borrow().parent.clone();
        }

        if replaced.borrow().parent.is_none() {
            if let Some(_) = replacement.clone() {
                self.tree_pointer(&replacement.clone().unwrap());
                self.key = replacement.clone().unwrap().borrow().key;   
            }
        }

        else if replaced.borrow().left.is_some() && replaced.borrow().right.is_some() {
            replacement.as_ref().unwrap().borrow_mut().left = reference.borrow().left.clone();
            replacement.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().parent = Some(BstNode::downgrade(&replacement.clone().unwrap()));

            if let Some(_) = self.left.clone() {
                if BstNode::is_node_match(&self.left.clone().unwrap(), reference) {
                    self.left = replacement.clone();
                }   
            }

            if let Some(_) = self.right.clone() {
                if BstNode::is_node_match(&self.right.clone().unwrap(), reference) {
                    self.right = replacement.clone();
                }
            }
        } 
        
        else {
            if let Some(_) = self.left.clone() {
                if BstNode::is_node_match(&self.left.clone().unwrap(), replaced) {
                    self.left = replacement.clone();
                }   
            }

            if let Some(_) = self.right.clone() {
                if BstNode::is_node_match(&self.right.clone().unwrap(), replaced) {
                    self.right = replacement.clone();
                }
            }

            if !BstNode::is_node_match(replaced, reference) {

                self.tree_pointer(replaced);
            }
        }
    }

    /**
     * Alternate simpler version of tree_successor that made use of is_nil checking
     */
    #[allow(dead_code)]
    pub fn tree_successor_simpler(x_node: &BstNodeLink) -> Option<BstNodeLink>{
        //create a shadow of x_node so it can mutate
        let mut x_node = x_node;
        let right_node = &x_node.borrow().right.clone();
        if BstNode::is_nil(right_node)!=true{
            return Some(right_node.clone().unwrap().borrow().minimum());
        }

        let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
        let y_node_right = &y_node.clone().unwrap().borrow().right.clone();
        let mut y_node2: Rc<RefCell<BstNode>>;
        while BstNode::is_nil(&y_node) && BstNode::is_node_match_option(Some(x_node.clone()), y_node_right.clone()) {
            y_node2 = y_node.clone().unwrap();
            x_node = &y_node2;
            let y_parent = y_node.clone().unwrap().borrow().parent.clone().unwrap();
            y_node = BstNode::upgrade_weak_to_strong(Some(y_parent));
        }

        //in case our sucessor traversal yield root, means self is the highest key
        if BstNode::is_node_match_option(y_node.clone(), Some(BstNode::get_root(&x_node))) {
            return None;
        }

        //default return self / x_node
        return Some(y_node.clone().unwrap())
    }

    /**
     * private function return true if node doesn't has parent nor children nor key
     */
    fn is_nil(node: &Option<BstNodeLink>) -> bool {
        match node {
            None => true,
            Some(x) => {
                if x.borrow().parent.is_none()
                    || x.borrow().left.is_none()
                    || x.borrow().right.is_none()
                {
                    return true;
                }
                return false;
            }
        }
    }

    //helper function to compare both nodelink
    fn is_node_match_option(node1: Option<BstNodeLink>, node2: Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        if let Some(node1v) = node1 {
            return node2.is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    fn is_node_match(anode: &BstNodeLink, bnode: &BstNodeLink) -> bool {
        if anode.borrow().key == bnode.borrow().key {
            return true;
        }
        return false;
    }

    /**
     * As the name implied, used to upgrade parent node to strong nodelink
     */
    fn upgrade_weak_to_strong(node: Option<WeakBstNodeLink>) -> Option<BstNodeLink> {
        match node {
            None => None,
            Some(x) => Some(x.upgrade()).unwrap(),
        }
    }
}
