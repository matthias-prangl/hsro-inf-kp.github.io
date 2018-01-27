extern crate simple_stack;

use simple_stack::Stack;

#[test]
fn empty_test() {
    let mut stack = Stack::new();
    assert_eq!(stack.pop(), None);
}

#[test] 
fn push_elems_test() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    assert_eq!(stack.pop(), Some(4));
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
}

#[test] 
fn iter_test() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    assert_eq!(stack.next(), Some(4));
    assert_eq!(stack.next(), Some(3));
    assert_eq!(stack.next(), Some(2));
    assert_eq!(stack.next(), Some(1));
    assert_eq!(stack.next(), None);
}

// #[test] 
// fn push_elems_test() {
//     let mut stack = Stack::new();
//     stack.push("hallo");
//     stack.push("ciao");
//     stack.push("ahoj");
//     stack.push("今日は");
//     assert_eq!(stack.pop(), Some("今日は"));
//     assert_eq!(stack.pop(), Some("ahoj"));
//     assert_eq!(stack.pop(), Some("ciao"));
//     assert_eq!(stack.pop(), Some("hallo"));
//     assert_eq!(stack.pop(), None);
// }

// #[test]
// fn iter_test() {
//     let mut stack = Stack::new();
//     stack.push("hallo");
//     stack.push("ciao");
//     stack.push("ahoj");
//     stack.push("今日は");
//     assert_eq!(stack.next(), Some("今日は"));
//     assert_eq!(stack.next(), Some("ahoj"));
//     assert_eq!(stack.next(), Some("ciao"));
//     assert_eq!(stack.next(), Some("hallo"));
//     assert_eq!(stack.next(), None);
// }