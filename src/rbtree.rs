struct Node<K:Ord+Clone, V> {
    key: K,
    val: *const V,
    color: usize,
    size: usize,
    parent: nodeptr_t<K,V>,
    left: nodeptr_t<K,V>,
    right: nodeptr_t<K,V>,
}

type nodeptr_t<K,V> = Option<Box<Node<K,V>>>;

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
    root: nodeptr_t<K,V>,
}

impl<K:Ord+Clone,V> RBTree<K,V> {
    pub fn new() -> Self {
        RBTree {
            root: None,
        }
    }

    fn is_node_red(node: &nodeptr_t<K,V>) -> bool {
        if let Some(n) = node {
            return n.is_red();
        }
        return false;
    }

    fn is_child_node_red(node: &nodeptr_t<K,V>, side: usize) -> bool {
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
        if let Some(nptr_left) = nodeptr.left.clone() {
            if let Some(mut nptr_right) = nodeptr.right.clone() {
                let mut xptr = nptr_right.clone();
              
                let mut nodeptr_clone = nodeptr.clone();
                nodeptr_clone.right = xptr.left.clone();
                nodeptr_clone.color = 1;
                nodeptr_clone.size = nptr_left.size + nptr_right.size + 1;

                xptr.left = Some(nodeptr_clone);
                xptr.color = nodeptr.color;
                xptr.size = nodeptr.size;
                return xptr;
            }
            return nptr_left;
        } else {
            return nodeptr;
        }    
    }

    fn rotate_right(nodeptr: Box<Node<K,V>>) -> Box<Node<K,V>> {
        if let Some(nptr_left) = nodeptr.left.clone() {
            if let Some(mut nptr_right) = nodeptr.right.clone() {
                let mut xptr = nptr_left.clone();
              
                let mut nodeptr_clone = nodeptr.clone();
                nodeptr_clone.left = xptr.right.clone();
                nodeptr_clone.color = 1;
                nodeptr_clone.size = nptr_left.size + nptr_right.size + 1;

                xptr.right = Some(nodeptr_clone);
                xptr.color = nodeptr.color;
                xptr.size = nodeptr.size;
                return xptr;
            }
            return nptr_left;
        } else {
            return nodeptr;
        }
    }

    fn flip_colors(nodeptr: Box<Node<K,V>>) -> Box<Node<K,V>> {
        return nodeptr;
    }

    fn put(root: nodeptr_t<K,V>, key: K, val: *const V) -> nodeptr_t<K,V> {
        if let Some(mut r) = root {
            if key == (*r).key {
                (*r).val = val;
            } else if key < (*r).key {
                (*r).left = RBTree::put((*r).left.clone(), key, val);
            } else {
                (*r).right = RBTree::put((*r).right.clone(), key, val);
            }
            // fix up tree
            if RBTree::is_node_red(&(*r).right) && !RBTree::is_node_red(&(*r).left) {
                r = RBTree::rotate_left(r);
            }
            if RBTree::is_node_red(&(*r).right) && !RBTree::is_child_node_red(&(*r).left, 0) {
                r = RBTree::rotate_right(r);
            }
            if RBTree::is_node_red(&(*r).left) &&  RBTree::is_node_red(&(*r).right) {
                r = RBTree::flip_colors(r);
            }
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
}
