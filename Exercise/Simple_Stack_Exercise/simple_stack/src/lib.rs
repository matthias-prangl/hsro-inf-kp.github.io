
#[derive(Debug)]
pub struct Stack {
    //a stack points to the first associated StackElem
}

#[derive(Debug)]
struct StackElem {
    //a stack element needs some content 
    //and information about the following element
}

impl Stack {
    //create an empty stack
    pub fn new() -> Stack {
        unimplemented!();
    }

    //create a new StackElem and push it on top of the stack
    pub fn push(&mut self, content: i32) {
        unimplemented!();
    }

    //take the first elemnt off of the stack and return its content
    pub fn pop(&mut self) -> Option<i32> {
        unimplemented!();
    }
}
