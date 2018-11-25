use self::rbtree::RBTree;
mod rbtree;

fn main() {
    let mut tree = RBTree::new();
    tree.insert(3, &5);
    let val = tree.find(0);
    if let Some(v) = val {
        unsafe {println!("Some {}!", *v);}
    } else {
        println!("No {}!", 0);
    }
}
