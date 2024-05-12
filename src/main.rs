use RedBlackTrees::{RedBlackTree, RedBlackTreeWithReps};
fn main() {

    let mut b = RedBlackTree::new();

    b.insert(15, "A");
    b.insert(6, "B");
    b.insert(3, "C");
    b.insert(2, "D");
    b.insert(4, "E");
    b.insert(7, "F");
    b.insert(13, "G");
    b.insert(9, "H");
    b.insert(18, "I");
    b.insert(17, "J");
    b.insert(20, "K");
    b.insert(2, "L");
    b.insert(18, "M");

    b.print_elements();

    let mut b = RedBlackTreeWithReps::new();

    for i in 0..10000 {
        b.insert(rand::random::<u8>() % 2, "A")
    }

    for index in 0..b.len() {
        assert_eq!(b.counting_blacks(index), true);
    }

    assert!(b.red_not_parent_red());

    assert!(b.binary_tree_property());

    assert!(b.root_is_black());

    b.print_elements();

}
