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
    stack.push("hallo");
    stack.push("ciao");
    stack.push("ahoj");
    stack.push("今日は");
    assert_eq!(stack.pop(), Some("今日は"));
    assert_eq!(stack.pop(), Some("ahoj"));
    assert_eq!(stack.pop(), Some("ciao"));
    assert_eq!(stack.pop(), Some("hallo"));
    assert_eq!(stack.pop(), None);
}

#[test]
fn iter_test() {
    let mut stack = Stack::new();
    stack.push("hallo");
    stack.push("ciao");
    stack.push("ahoj");
    stack.push("今日は");
    assert_eq!(stack.next(), Some("今日は"));
    assert_eq!(stack.next(), Some("ahoj"));
    assert_eq!(stack.next(), Some("ciao"));
    assert_eq!(stack.next(), Some("hallo"));
    assert_eq!(stack.next(), None);
}