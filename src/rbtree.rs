struct Node<K:Ord, V> {
    key: K,
    val: *const V,
    color: usize,
    size: usize,
    parent: nodeptr_t<K,V>,
    left: nodeptr_t<K,V>,
    right: nodeptr_t<K,V>,
}

type nodeptr_t<K,V> = Option<*const Node<K,V>>;

impl<K:Ord, V> Node<K, V> {
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

pub struct RBTree<K:Ord, V> {
    root: nodeptr_t<K,V>,
}

impl<K:Ord,V> RBTree<K,V> {
    pub fn new() -> Self {
        RBTree {
            root: None,
        }
    }

    fn is_node_red(&self, node: nodeptr_t<K,V>) -> bool {
        if let Some(n) = node {
            return (*n).is_red();
        }
        return false;
    }

    fn is_child_node_red(&self, node: nodeptr_t<K,V>, side: usize) -> bool {
        if let Some(n) = node {
            if side == 0 {
                if let Some(c) = (*n).left {
                    return (*c).is_red();
                }
            } else if side == 1 {
                if let Some(c) = (*n).right {
                    return (*c).is_red();
                }
            }
        }
        return false;
    }

    fn rotate_left(&self, nodeptr: *const Node<K,V>) {
    }

    fn rotate_right(&self, nodeptr: *const Node<K,V>) -> *const Node<K,V> {
        // Clones??
        if let Some(xptr) = (*nodeptr).left {
            (*nodeptr).left = (*xptr).right;
            (*xptr).right = Some(nodeptr);
            return xptr;
        } else {
            return nodeptr;
        }
    }

    fn flip_colors(&self, nodeptr: *const Node<K,V>) {
    }
    
    fn put(&mut self, root: nodeptr_t<K,V>, key: K, val: *const V) -> nodeptr_t<K,V> {
        if let Some(r) = root {
            if key == (*r).key {
                (*r).val = val;
            } else if key < (*r).key {
                self.put((*r).left, key, val);
            } else {
                self.put((*r).right, key, val);
            }
            // TODO fix up tree
            if self.is_node_red((*r).right) && !self.is_node_red((*r).left) {
                self.rotate_left(r);
            }
            if self.is_node_red((*r).right) && !self.is_child_node_red((*r).left, 0) {
                r = self.rotate_right(r);
            }
            if self.is_node_red((*r).left) && self.is_node_red((*r).right) {
                self.flip_colors(r);
            }
            return None;
        } else {
            return Some(&Node::new(key, val, 1, 1));
        }
    }

    pub fn insert(&mut self, key: K, val: *const V) {
        if let Some(new_root) = self.put(self.root, key, val) {
            self.root = Some(new_root);
        }
        if let Some(rptr) = self.root {
            (*rptr).color = 0;
        } 
    }

    pub fn find(&self, key: K) -> Option<*const V> {
        let x_opt = self.root;
        while let Some(x) = x_opt {
            if key == (*x).key {
                return Some((*x).val);
            } else if key < (*x).key {
                x_opt = (*x).left;
            } else {
                x_opt = (*x).right;
            }
        }
        return None;
    }
}
