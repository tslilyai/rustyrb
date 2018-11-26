use self::rbtree::RBTree;
mod rbtree;

fn main() {
    let mut tree = RBTree::new();
    let x = 5;
    for i in 0..70 {
        tree.insert(i, &x);
    }
    for i in 0..10{
        println!("Delete {}, {}", i, tree.delete(i));
    }
    for i in 0..100 {
        let val = tree.find(&i);
        if let Some(v) = val {
            unsafe {println!("{}: Some {}!", i, *v);}
        } else {
            println!("No {}!", i);
        }
    }
    println!("Tree size : {}", tree.size());
    println!("Tree check : {}", tree.check());
}
