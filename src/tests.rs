use super::{RedBlackTree, RedBlackTreeWithReps};
use rand::Rng;

#[test]
fn get_sucessor_predecessor_test() {

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


    assert_eq!(b.inorder(), vec![(&2, &"D"), (&3, &"C"), (&4, &"E"), (&6, &"B"), (&7, &"F"), (&9, &"H"),
                                 (&13, &"G"), (&15, &"A"), (&17, &"J"), (&18, &"I"), (&20, &"K")]);

    assert!(b.is_red_black_tree());

    assert_eq!(b.get_sucessor(&2), Some((&3, &"C")));
    assert_eq!(b.get_sucessor(&3), Some((&4, &"E")));
    assert_eq!(b.get_sucessor(&4), Some((&6, &"B")));
    assert_eq!(b.get_sucessor(&6), Some((&7, &"F")));
    assert_eq!(b.get_sucessor(&7), Some((&9, &"H")));
    assert_eq!(b.get_sucessor(&9), Some((&13, &"G")));
    assert_eq!(b.get_sucessor(&13), Some((&15, &"A")));
    assert_eq!(b.get_sucessor(&15), Some((&17, &"J")));
    assert_eq!(b.get_sucessor(&17), Some((&18, &"I")));
    assert_eq!(b.get_sucessor(&18), Some((&20, &"K")));
    assert_eq!(b.get_sucessor(&20), None);

    assert_eq!(b.get_predecessor(&2), None);
    assert_eq!(b.get_predecessor(&3), Some((&2, &"D")));
    assert_eq!(b.get_predecessor(&4), Some((&3, &"C")));
    assert_eq!(b.get_predecessor(&6), Some((&4, &"E")));
    assert_eq!(b.get_predecessor(&7), Some((&6, &"B")));
    assert_eq!(b.get_predecessor(&9), Some((&7, &"F")));
    assert_eq!(b.get_predecessor(&13), Some((&9, &"H")));
    assert_eq!(b.get_predecessor(&15), Some((&13, &"G")));
    assert_eq!(b.get_predecessor(&17), Some((&15, &"A")));
    assert_eq!(b.get_predecessor(&18), Some((&17, &"J")));
    assert_eq!(b.get_predecessor(&20), Some((&18, &"I")));


    //------

    let mut b = RedBlackTreeWithReps::new();

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

    assert_eq!(b.inorder(), vec![(&2, &"D"), (&2, &"L"), (&3, &"C"), (&4, &"E"), (&6, &"B"), (&7, &"F"), (&9, &"H"),
                                 (&13, &"G"), (&15, &"A"), (&17, &"J"), (&18, &"I"), (&18, &"M"), (&20, &"K")]);

    assert!(b.is_red_black_tree());

    assert_eq!(b.get_sucessor(&2), Some((&2, &"L")));
    assert_eq!(b.get_sucessor(&3), Some((&4, &"E")));
    assert_eq!(b.get_sucessor(&4), Some((&6, &"B")));
    assert_eq!(b.get_sucessor(&6), Some((&7, &"F")));
    assert_eq!(b.get_sucessor(&7), Some((&9, &"H")));
    assert_eq!(b.get_sucessor(&9), Some((&13, &"G")));
    assert_eq!(b.get_sucessor(&13), Some((&15, &"A")));
    assert_eq!(b.get_sucessor(&15), Some((&17, &"J")));
    assert_eq!(b.get_sucessor(&17), Some((&18, &"I")));
    assert_eq!(b.get(&18), Some(&"M"));  // devido a rotação o 18 M vem primeiro em altura. O 18I é seu filho esquerdo
    assert_eq!(b.get_sucessor(&18), Some((&20, &"K")));
    assert_eq!(b.get_sucessor(&20), None);

    assert_eq!(b.get_predecessor(&2), None);
    assert_eq!(b.get_predecessor(&3), Some((&2, &"L")));
    assert_eq!(b.get_predecessor(&4), Some((&3, &"C")));
    assert_eq!(b.get_predecessor(&6), Some((&4, &"E")));
    assert_eq!(b.get_predecessor(&7), Some((&6, &"B")));
    assert_eq!(b.get_predecessor(&9), Some((&7, &"F")));
    assert_eq!(b.get_predecessor(&13), Some((&9, &"H")));
    assert_eq!(b.get_predecessor(&15), Some((&13, &"G")));
    assert_eq!(b.get_predecessor(&17), Some((&15, &"A")));
    assert_eq!(b.get(&18), Some(&"M"));  // devido a rotação o 18 M vem primeiro em altura. O 18I é seu filho esquerdo
    assert_eq!(b.get_predecessor(&18), Some((&18, &"I")));
    assert_eq!(b.get_predecessor(&20), Some((&18, &"M")));

}

#[test]
fn properties_test() {

    let mut b = RedBlackTree::new();

    for i in 0..10000 {
        b.insert(rand::random::<i32>(), "A")
    }

    assert!(b.is_red_black_tree());

    //--------

    let mut b = RedBlackTree::new();

    for i in 0..10000 {
        b.insert(rand::random::<u8>(), "A")
    }

    assert!(b.is_red_black_tree());
}

#[test]
fn properties_with_reps_test() {

    let mut b = RedBlackTreeWithReps::new();

    for i in 0..10000 {
        b.insert(rand::random::<i32>(), "A")
    }
    assert!(b.is_red_black_tree());

    //--------

    let mut b = RedBlackTreeWithReps::new();

    for i in 0..10000 {
        b.insert(rand::random::<u8>(), "A")
    }
    assert!(b.is_red_black_tree());

}

fn deletion_test() {

    let list = [(&15,8), (&6,4), (&3,2), (&2,0), (&4,3), (&7,5), (&13,7), (&9,6), (&18,10), (&17,9), (&20,12)];

    for element in list {
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

        let mut v = vec![(&2, &"D"), (&2, &"L"), (&3, &"C"), (&4, &"E"), (&6, &"B"), (&7, &"F"), (&9, &"H"),
                         (&13, &"G"), (&15, &"A"), (&17, &"J"), (&18, &"I"), (&18, &"M"), (&20, &"K")];

        v.remove(element.1);

        b.deletion(element.0);

        assert!(b.is_red_black_tree());
    }

    //-------

    let list = [(&15,8), (&6,4), (&3,2), (&2,0), (&4,3), (&7,5), (&13,7), (&9,6), (&18,10), (&17,9), (&20,12)];

    for element in list {
        let mut b = RedBlackTreeWithReps::new();

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

        let mut v = vec![(&2, &"D"), (&2, &"L"), (&3, &"C"), (&4, &"E"), (&6, &"B"), (&7, &"F"), (&9, &"H"),
                         (&13, &"G"), (&15, &"A"), (&17, &"J"), (&18, &"I"), (&18, &"M"), (&20, &"K")];

        v.remove(element.1);

        b.deletion(element.0);

        assert!(b.is_red_black_tree());
    }

}