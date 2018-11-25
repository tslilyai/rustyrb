use std::rc::Rc;

fn rc_ptrs_equal <K>(left: Rc<K>, right: Rc<K>) -> bool {
    let a = left.as_ref() as *const _;
    let b = right.as_ref() as *const _;
    return a == b;
}

struct Node<K:Ord, V> {
    key: K,
    val: *const V,
    color: usize,
    size: usize,
    parent: nodeptr_t<K,V>,
    left: nodeptr_t<K,V>,
    right: nodeptr_t<K,V>,
}

type nodeptr_t<K,V> = Option<Rc<Node<K,V>>>;

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

    fn is_node_red(node_opt: nodeptr_t<K,V>) -> bool {
        if let Some (node_ptr) = node_opt {
            return node_ptr.color == 1;
        }
        return false;
    }
    
    fn size(node_opt: nodeptr_t<K,V>) -> usize {
        if let Some (node_ptr) = node_opt {
            return node_ptr.size;
        }
        return 0;
    }
    
    fn put(&self, root: &mut nodeptr_t<K,V>, key: K, val: *const V) -> nodeptr_t<K,V> {
        if let Some(rptr) = root {
            if let Some(mut r) = Rc::get_mut(rptr) {
                if key == r.key {
                    r.val = val;
                } else if key < r.key {
                    self.put(&mut r.left, key, val);
                } else {
                    self.put(&mut r.right, key, val);
                }
            }
            // TODO fix up
            return None;
        } else {
            return Some(Rc::new(Node::new(key, val, 1, 1)));
        }
    }

    pub fn insert(&mut self, key: K, val: *const V) {
        if let Some(new_root) = self.put(&mut self.root.clone(), key, val) {
            self.root = Some(new_root);
        }
        if let Some(rptr) = &mut self.root {
            if let Some(mut r) = Rc::get_mut(rptr) {
                r.color = 0;
            }
        } 
    }

    pub fn find(&self, key: K) -> Option<*const V> {
        let mut x_opt = &self.root;
        while let Some(x) = x_opt {
            if key == x.key {
                return Some(x.val);
            } else if key < x.key {
                x_opt = &x.left;
            } else {
                x_opt = &x.right;
            }
        }
        return None;
    }
}
 
    /*fn child(&self, isright: bool) -> Option<Rc<Node<K>>> {
        return if isright {self.children[0]} else {self.children[1]};
    }
    fn parent(&self) -> Option<Rc<Node<K>>> {
        return self.parent;
    }
    fn black_parent(&self) -> Option<Rc<Node<K>>> {
        if let Some (p) = self.parent {
            let node = Node::new(p.val, 0);
            node.children = p.children;
            return Some (Rc::new(node));
        }
        return None;
    }
    fn red(&self) -> bool {
        return self.color==1;
    }
    fn children_same_color() -> bool {
        return false;
    }
    fn find_child(&self, node: Rc<Node<K>>) -> usize {
        if let Some(child) = node.children[1] {
            if rc_ptrs_equal(child, node) {
                return 1;
            }
        }
        return 0;
    }
    fn load_color(&mut self, myNode: Rc<Node<K>>) {
        //assert (!red);
        let pOpt = self.black_parent();
        if let Some(p) = pOpt {
            if let Some(child) = p.children[p.find_child(myNode)] {
                if child.red() {
                    self.color = 1;
                }
            }
        }
    }
    fn change_color(&mut self, color: i32) {
        self.color = color;
    }
    fn reverse_color(&mut self) {
        self.color ^ 1;
    }
    fn flip(&mut self) {
        if let Some(c0) = self.children[0] {
            c0.reverse_color();
        }
        if let Some(c1) = self.children[1] {
            c1.reverse_color();
        }
        self.reverse_color();
    }
    /*fn set_child(bool is_right, Rc<Node<K>> x, Rc<Node<K>> root) -> Rc<Node<K>> {
    }*/
    
    fn rotate(&mut self, myNode: Rc<Node<K>>, side: usize) {
        let old_color = if self.red() {1} else {0};
        if let Some(child) = self.children[side^1] {
            if let Some(gc) = child.children[side] {
                if rc_ptrs_equal(child, gc) {
                    gc.parent = Some(myNode);
                }
                gc.change_color(1);
            } 
            child.parent = self.parent;
            child.change_color(old_color);
        }
        self.parent = self.children[side^1];
    }
    fn rotate_left() {}
    fn rotate_right() {}
    
    fn size() -> usize { return 0; }
    /*fn check(Rc<Node<K>> parent, int this_black_height, int black_height, 
             Rc<Node<K>> root, Rc<Node<K>> begin, Rc<Node<K>> end) -> usize {}*/
}

pub struct RBKree<K:Ord> {
    root_: Option<Rc<Node<K>>>,
    limits_ : [Option<Rc<Node<K>>>;2],
}

impl<K:Ord> RBKree<K> {
    pub fn new() -> Self {
        RBKree {
            root_: None,
            limit_: [None, None],
        }
    }
    pub fn insert(&mut self, val: K) {
        let mut side = 0;
        return;
    }

    pub fn insert_commit(&mut self, val: K, mut p: Rc<Node<K>>, side: usize) {
        // link in new node; it's red
        let mut node = Rc::new(Node::new(val, 1));
        node.parent = Some (p);
        node.children[0] = None;
        node.children[1] = None; 

        // maybe set limits
        p.children[side] = Some (node.clone());
        if let Some(limit) = self.limits_[side] {
            if rc_ptrs_equal(limit, p) {
                self.limits_[side] = Some(node.clone()); 
            }
        } 
        else {
            self.root_ = Some(node.clone());
            self.limits_[0] = Some(node.clone());
            self.limits_[1] = Some(node.clone());
        }

        // ignore reshaping for now
        // flip up the tree
        // invariant: we are looking at the `side` of `p`
        while p.red() {
            let gpOpt = p.black_parent();
            if let Some (gp) = gpOpt {
                let z = gp.clone();
                if let Some(child0) = gp.children[0] {
                    if let Some(child1) = gp.children[0] {
                        if child0.red() && child1.red() {
                            gp.flip();
                            z = gp.clone();
                            if let Some (zbp) = z.black_parent() {
                                zbp.load_color(zbp);
                                p = zbp.clone();
                            } 
                        }
                    }
                } else {
                    let gpside = gp.find_child(p);
                    if gpside != side {
                        p.rotate(p, gpside);
                        gp.children[gpside] = Some(p);
                    } 
                    gp.rotate(gp,!gpside);
                    z = gp.clone();
                    if let Some (zbp) = z.black_parent() {
                        p = zbp.clone();
                    }
                }
            }
            side = p.find_child(gp);
            p.set_child(side, z, root_);
        }
    }        */
