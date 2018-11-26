use std::fmt::Debug;

#[derive(Debug)]
struct Node<K:Ord+Clone+Debug, V> {
    key: K,
    val: *const V,
    color: usize,
    size: usize,
    parent: NodeptrT<K,V>,
    left: NodeptrT<K,V>,
    right: NodeptrT<K,V>,
}

type NodeptrT<K,V> = Option<Box<Node<K,V>>>;

impl<K:Ord+Clone+Debug, V> Clone for Node<K, V> {
    fn clone(&self) -> Self{
        Node {
            key: Clone::clone(&self.key),
            val: Clone::clone(&self.val),
            color: Clone::clone(&self.color),
            size: Clone::clone(&self.size),
            parent: Clone::clone(&self.parent),
            left: Clone::clone(&self.left),
            right: Clone::clone(&self.right),
        }
    }
}

impl<K:Ord+Clone+Debug, V> Node<K, V> {
    fn new(key: K, val: *const V, color: usize, size: usize) -> Self {
        Node {
            key: key,
            val: val,
            color: color,
            size: size,
            parent: None,
            left: None,
            right: None,
        }
    }

    fn is_red(&self) -> bool {
        return self.color == 1;
    }
    
    fn size(node: &NodeptrT<K,V>) -> usize {
        if let Some(n) = node {
            return n.size;
        }
        return 0;
    }
}

pub struct RBTree<K:Ord+Clone+Debug, V> {
    root: NodeptrT<K,V>,
}

impl<K:Ord+Clone+Debug,V> RBTree<K,V> {
    pub fn new() -> Self {
        RBTree {
            root: None,
        }
    }
    
    /***************************************************************************
    *  Insertion.
    ***************************************************************************/

    fn put(root: NodeptrT<K,V>, key: K, val: *const V) -> NodeptrT<K,V> {
        if let Some(mut r) = root {
            if key == (*r).key {
                (*r).val = val;
            } else if key < (*r).key {
                (*r).left = RBTree::put((*r).left.clone(), key, val);
            } else {
                (*r).right = RBTree::put((*r).right.clone(), key, val);
            }
            r = RBTree::balance(&r, true); 
            return Some(r);
        } else {
            return Some(Box::new(Node::new(key, val, 1, 1)));
        }
    }
   

    pub fn insert(&mut self, key: K, val: *const V) {
        if let Some(new_root) = RBTree::put(self.root.clone(), key, val) {
            self.root = Some(new_root);
        }
        if let Some(ref mut rptr) = &mut self.root {
            (*rptr).color = 0;
        } 
    }
 
    /***************************************************************************
    *  Delete.
    ***************************************************************************/
    pub fn delete(&mut self, key: K) -> bool { 
        if !self.contains(&key) {
            return false;
        }

        // if both children of root are black, set root to red
        if let Some(ref mut r) = self.root {
            if !RBTree::is_node_red(&r.left) && !RBTree::is_node_red(&r.right) {
                r.color = 1;
            }
        }

        // delete and set new root
        let size = self.size();
        self.root = self.delete_helper(&self.root, key);
        if let Some(ref mut r) = self.root {
            r.color = 0;
        } 
        return self.size() < size;
    }

    fn move_red_left(h: &mut Box<Node<K,V>>) {
        RBTree::flip_colors(h);
        if RBTree::is_child_node_red(&h.right, 0) {
            if let Some(mut right) = h.right.clone() {
                RBTree::rotate_right(&mut right);
                h.right = Some(right);
                RBTree::rotate_left(h);
                RBTree::flip_colors(h);
            }
        }
    }

    fn move_red_right(h : &mut Box<Node<K,V>>) {
        RBTree::flip_colors(h);
        if RBTree::is_child_node_red(&h.left, 0) {
            RBTree::rotate_right(h);
            RBTree::flip_colors(h);
        }    
    }

    fn min(hptr: &NodeptrT<K,V>) -> &NodeptrT<K,V> {
        if let Some(h) = hptr {
            return RBTree::min(&h.left);
        }
        return hptr;
    }

    fn delete_min(hptr: &mut NodeptrT<K,V>) -> NodeptrT<K,V> {
        if let Some(ref mut h) = hptr {
            if let Some(ref left) = h.left.clone() {
                if !left.is_red() && !RBTree::is_node_red(&left.left) {
                    RBTree::move_red_left(h);
                }
                h.left = RBTree::delete_min(&mut h.left);
                return Some(RBTree::balance(&h, false));
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    pub fn delete_helper(&self, hptr: &NodeptrT<K,V>, key: K) -> NodeptrT<K,V> { 
        if let Some(mut h) = hptr.clone() {
            if key < h.key  {
                if !RBTree::is_node_red(&h.left) && !RBTree::is_child_node_red(&h.left, 0) {
                    RBTree::move_red_left(&mut h);
                }
                h.left = self.delete_helper(&h.left, key);
            } else {
                if RBTree::is_node_red(&h.left) {
                    RBTree::rotate_right(&mut h);
                }
                if key == h.key {
                    if let None = h.right {
                        return None;
                    }
                }
                if !RBTree::is_node_red(&h.right) && !RBTree::is_child_node_red(&h.right, 0) {
                    RBTree::move_red_right(&mut h);
                }
                if key == h.key {
                    // this must be the case because h.right is not None
                    if let Some(x) = RBTree::min(&h.right.clone()) {
                        h.key = x.key.clone();
                        h.val = x.val.clone();
                        h.right = RBTree::delete_min(&mut h.right);
                    }
                }
                else {
                    h.right = self.delete_helper(&h.right, key);
                }
            }
            return Some(RBTree::balance(&h, false));
        }
        return None;
    }
 
    /***************************************************************************
    *  Find.
    ***************************************************************************/
    pub fn find(&self, key: &K) -> Option<*const V> {
        let mut x_opt = &self.root;
        while let Some(x) = x_opt {
            if *key == (*x).key {
                let found = (*x).val;
                return Some(found);
            } else if *key < (*x).key {
                x_opt = &(*x).left;
            } else {
                x_opt = &(*x).right;
            }
        }
        return None;
    }
    pub fn contains(&self, key: &K) -> bool {
        if let Some(x) = self.find(key) {
            return true;
        } 
        return false;
    }

   /***************************************************************************
    *  Rotation and Coloring Functions.
    ***************************************************************************/
    fn is_node_red(node: &NodeptrT<K,V>) -> bool {
        if let Some(n) = node {
            return n.is_red();
        }
        return false;
    }

    fn is_child_node_red(node: &NodeptrT<K,V>, side: usize) -> bool {
        if let Some(n) = node {
            if side == 0 {
                if let Some(ref c) = n.left {
                    return (*c).is_red();
                }
            } else if side == 1 {
                if let Some(ref c) = n.right {
                    return (*c).is_red();
                }
            }
        }
        return false;
    }

    fn rotate_left(nodeptr: &mut Box<Node<K,V>>) {
        if let Some(mut xptr) = nodeptr.right.clone() {
            let mut nodeptr_clone = nodeptr.clone();
            nodeptr_clone.right = xptr.left.clone();
            nodeptr_clone.color = 1;
            nodeptr_clone.size = 
                Node::size(&nodeptr_clone.left) + Node::size(&nodeptr_clone.right) + 1;
            
            xptr.left = Some(nodeptr_clone);
            xptr.color = nodeptr.color;
            xptr.size = nodeptr.size;
            *nodeptr = xptr;
        }
    }

    fn rotate_right(nodeptr: &mut Box<Node<K,V>>) {
        if let Some(mut xptr) = nodeptr.left.clone() {
            let mut nodeptr_clone = nodeptr.clone();
            nodeptr_clone.left = xptr.right.clone();
            nodeptr_clone.color = 1;
            nodeptr_clone.size = 
                Node::size(&nodeptr_clone.left) + Node::size(&nodeptr_clone.right) + 1;
            
            xptr.right = Some(nodeptr_clone);
            xptr.color = nodeptr.color;
            xptr.size = nodeptr.size;
            *nodeptr = xptr;
        }
    }
    
    fn set_color(nodeptr: &mut NodeptrT<K,V>, color: usize) {
        if let Some(ref mut n) = nodeptr {
            n.color = color;
        }
    }

    fn flip_colors(nodeptr: &mut Box<Node<K,V>>) {
        let color = nodeptr.color;
        if nodeptr.color == 0 {
            nodeptr.color = 1;
        } else {
            nodeptr.color = 0;
        }
        RBTree::set_color(&mut nodeptr.left, color);
        RBTree::set_color(&mut nodeptr.right, color);
    }

    fn balance(rbox: &Box<Node<K,V>>, is_put: bool) -> Box<Node<K,V>> {
        let mut r = rbox.clone();
        if RBTree::is_node_red(&(*r).right) {
            if !is_put || (is_put && !RBTree::is_node_red(&(*r).left)) {
                RBTree::rotate_left(&mut r);
            }
        }
        if RBTree::is_node_red(&(*r).left) && RBTree::is_child_node_red(&(*r).left, 0) {
            RBTree::rotate_right(&mut r);
        }
        if RBTree::is_node_red(&(*r).left) &&  RBTree::is_node_red(&(*r).right) {
            RBTree::flip_colors(&mut r);
        }
        r.size = Node::size(&r.right) + Node::size(&r.left) + 1;
        return r;
    }

   /***************************************************************************
    *  Utility functions.
    ***************************************************************************/
    pub fn size(&self) -> usize {
        return Node::size(&self.root);
    }

    /**
     * Returns the height of the bst (for debugging).
     * @return the height of the bst (a 1-node tree has height 0)
     */
    pub fn height(&self) -> isize {
        return self.height_helper(&self.root);
    }
    fn height_helper(&self, xptr: &NodeptrT<K,V>) -> isize {
        if let Some(x) = xptr {
            let lheight = self.height_helper(&x.left);
            let rheight = self.height_helper(&x.right);
            let childheight = if lheight > rheight {lheight} else {rheight};
            return 1 + childheight;
        } else {
            return -1;
        }
    }

    /***************************************************************************
    *  Check integrity of red-black tree data structure.
    ***************************************************************************/
    fn print_tree(root: &NodeptrT<K,V>, iter: usize) {
        if let Some(x) = root {
            RBTree::print_tree(&x.left, iter+1);
            print!("--Iter{}, {:?}:({},{})--", iter, x.key, x.size, x.color);
            RBTree::print_tree(&x.right, iter+1);
        } else {
            print!("--NULL--");
        }
    }
 
    pub fn check(&self) -> bool {
        if !self.is_bst() {
            println!("Not in symmetric order");
        }
        if !self.is_size_consistent() {
            println!("Subtree counts not consistent");
        }
        if !self.is_rank_consistent() {
            println!("Ranks not consis_tent");
        }
        if !self.is_23() {
            println!("Not a 2-3 tree");
        }
        if !self.is_balanced() {
            println!("Not balanced");
        }
        RBTree::print_tree(&self.root, 0);
        println!("\n");
        return self.is_bst() 
            && self.is_size_consistent() 
            && self.is_rank_consistent() 
            && self.is_23() 
            && self.is_balanced();
    }

    // does this binary tree satisfy symmetric order?
    // Note: this test also ensures that data structure is a binary tree since order is strict
    fn is_bst(&self) -> bool {
        return self.is_bst_helper(&self.root, None, None);
    }

    // is the tree rooted at x a bst with all keys strictly between min and max
    // (if min or max is null, treat as empty constraint)
    // Credit: Bob Dondero's elegant solution
    fn is_bst_helper(&self, xptr: &NodeptrT<K,V>, min: Option<&K>, max: Option<&K>) -> bool {
        if let Some(x) = xptr {
            if let Some(mn) = min {
                if x.key <= *mn {
                    return false;
                } 
            }
            if let Some(mx) = max {
                if x.key >= *mx {
                    return false;
                } 
            }
            return self.is_bst_helper(&x.left, min, Some(&x.key)) && 
                self.is_bst_helper(&x.right, Some(&x.key), max);
        }
        return true;
    }
    // are the size fields correct?
    fn is_size_consistent(&self) -> bool { 
        return self.is_size_consistent_helper(&self.root); 
    }
    fn is_size_consistent_helper(&self, xptr: &NodeptrT<K,V>) -> bool {
        if let Some(x) = xptr {
            if x.size != Node::size(&x.left) + Node::size(&x.right) + 1 {
                return false;
            }
            return self.is_size_consistent_helper(&x.left) 
                && self.is_size_consistent_helper(&x.right);
        }
        return true;
    }

    // check that ranks are consistent
    fn is_rank_consistent(&self) -> bool {
        //TODO
        return true;
    }

    // Does the tree have no red right links, and at most one (left)
    // red links in a row on any path?
    fn is_23(&self) -> bool { 
        return self.is_23_helper(&self.root); 
    }

    fn is_23_helper(&self, xptr: &NodeptrT<K,V>) -> bool {
        if let Some(x) = xptr {
            if RBTree::is_node_red(&x.right) {
                return false;
            }
            if RBTree::is_node_red(&xptr) && RBTree::is_node_red(&x.left) {
                if let Some(ref y) = &self.root {
                    if y.key != x.key {
                        return false;
                    }
                }
            }
            return self.is_23_helper(&x.left) && self.is_23_helper(&x.right);
        }
        return true;
    }

    // do all paths from root to leaf have same number of black edges?
    fn is_balanced(&self) -> bool {
        let mut black = 0;     // number of black links on path from root to min
        let mut xptr = self.root.clone();
        while let Some(x) = xptr {
            if !x.is_red() {
                black += 1;
            }
            xptr = x.left;
        }
        return self.is_balanced_helper(&self.root, black);
    }

    // does every path from the root to a leaf have the given number of black links?
    fn is_balanced_helper(&self, xptr: &NodeptrT<K,V>, black: isize) -> bool {
        let mut new_black = black;
        if let Some(x) = xptr {
            if !x.is_red() {
                new_black -= new_black;
            } 
            return self.is_balanced_helper(&x.left, new_black) && 
                self.is_balanced_helper(&x.right, new_black);
        }
        return black == 0;
    }
}
