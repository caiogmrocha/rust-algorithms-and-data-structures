pub mod sort;
pub mod linked_list;
pub mod bst;

fn main() {
    let mut tree = bst::new(50);

    tree.insert(25);
    tree.insert(75);
    tree.insert(100);
    tree.insert(80);
    tree.insert(125);

    // println!("{:#?}", tree);

    // tree.remove(50);

    // println!("{:#?}", tree);

    tree.level_order_traversal(&|value| println!("{}", value));
}
