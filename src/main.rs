use self::rbtree::RBTree;
mod rbtree;

fn main() {
    let mut tree = RBTree::new();
    let x = 5;
    tree.insert(3, &x);
    tree.insert(4, &x);
    tree.insert(5, &x);
    //tree.insert(6, &x);
    for i in 0..7 {
        let val = tree.find(i);
        if let Some(v) = val {
            unsafe {println!("Some {}!", *v);}
        } else {
            println!("No {}!", 0);
        }
    }
    println!("Tree size : {}", tree.size());
    println!("Tree check : {}", tree.check());
}
