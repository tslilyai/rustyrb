#[derive(Debug)]
struct Node<K:Ord+Clone, V> {
    key: K,
    val: *const V,
    color: usize,
    size: usize,
    parent: NodeptrT<K,V>,
    left: NodeptrT<K,V>,
    right: NodeptrT<K,V>,
}

type NodeptrT<K,V> = Option<Box<Node<K,V>>>;

impl<K:Ord+Clone, V> Clone for Node<K, V> {
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

impl<K:Ord+Clone, V> Node<K, V> {
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
    
    fn size(&self) -> usize {
        return self.size;
    }
}

pub struct RBTree<K:Ord+Clone, V> {
    root: NodeptrT<K,V>,
}

impl<K:Ord+Clone,V> RBTree<K,V> {
    pub fn new() -> Self {
        RBTree {
            root: None,
        }
    }

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

    fn rotate_left(nodeptr: Box<Node<K,V>>) -> Box<Node<K,V>> {
        if let Some(nptr_right) = nodeptr.right.clone() {
            let mut xptr = nptr_right.clone();
          
            let mut nodeptr_clone = nodeptr.clone();
            nodeptr_clone.right = xptr.left.clone();
            nodeptr_clone.color = 1;
            if let Some(nptr_left) = nodeptr.left.clone() {
                nodeptr_clone.size = nptr_left.size + nptr_right.size + 1;
            } else {
                nodeptr_clone.size = nptr_right.size + 1;
            }                
            
            xptr.left = Some(nodeptr_clone);
            xptr.color = nodeptr.color;
            xptr.size = nodeptr.size;
            return xptr;
        } else {
            return nodeptr;
        }    
    }

    fn rotate_right(nodeptr: Box<Node<K,V>>) -> Box<Node<K,V>> {
        if let Some(nptr_left) = nodeptr.left.clone() {
            let mut xptr = nptr_left.clone();
            
            let mut nodeptr_clone = nodeptr.clone();
            nodeptr_clone.left = xptr.right.clone();
            nodeptr_clone.color = 1;
            if let Some(nptr_right) = nodeptr.right.clone() {
                nodeptr_clone.size = nptr_left.size + nptr_right.size + 1;
            } else {
                nodeptr_clone.size = nptr_left.size + 1;
            }
            
            xptr.right = Some(nodeptr_clone);
            xptr.color = nodeptr.color;
            xptr.size = nodeptr.size;
            return xptr;
        } else {
            return nodeptr;
        }
    }

    fn flip_colors(nodeptr: &mut Box<Node<K,V>>) {
        if nodeptr.color == 0 {
            nodeptr.color = 1;
        } else {
            nodeptr.color = 0;
        }
    }

    fn balance(r: Box<Node<K,V>>, is_put: bool) -> Box<Node<K,V>> {
        let mut r = r.clone();
        if RBTree::is_node_red(&(*r).right) {
            if !is_put || (is_put && !RBTree::is_node_red(&(*r).left)) {
                r = RBTree::rotate_left(r);
            }
        }
        if RBTree::is_node_red(&(*r).left) && !RBTree::is_child_node_red(&(*r).left, 0) {
            r = RBTree::rotate_right(r);
        }
        if RBTree::is_node_red(&(*r).left) &&  RBTree::is_node_red(&(*r).right) {
            RBTree::flip_colors(&mut r);
        }
        return r;
    }

    fn put(root: NodeptrT<K,V>, key: K, val: *const V, iter: usize) -> NodeptrT<K,V> {
        if let Some(mut r) = root {
            if key == (*r).key {
                (*r).val = val;
            } else if key < (*r).key {
                (*r).left = RBTree::put((*r).left.clone(), key, val, iter+1);
            } else {
                (*r).right = RBTree::put((*r).right.clone(), key, val, iter+1);
            }
            r = RBTree::balance(r, true); 
            return Some(r);
        } else {
            println!("New in iter! {}", iter);
            return Some(Box::new(Node::new(key, val, 1, 1)));
        }
    }
   
    pub fn insert(&mut self, key: K, val: *const V) {
        if let None = self.root.clone() {
            println!("Clone is None");
        }
        if let Some(new_root) = RBTree::put(self.root.clone(), key, val, 0) {
            self.root = Some(new_root);
            println!("Inserted new root of size {}", self.size());
        }
        if let Some(ref mut rptr) = &mut self.root {
            (*rptr).color = 0;
        } 
    }

    pub fn find(&self, key: K) -> Option<*const V> {
        let mut x_opt = &self.root;
        while let Some(x) = x_opt {
            if key == (*x).key {
                let found = (*x).val;
                return Some(found);
            } else if key < (*x).key {
                x_opt = &(*x).left;
            } else {
                x_opt = &(*x).right;
            }
        }
        return None;
    }

    pub fn size(&self) -> usize {
        if let Some(ref r) = self.root {
            return r.size();
        } else {
            return 0;
        }
    }
}
