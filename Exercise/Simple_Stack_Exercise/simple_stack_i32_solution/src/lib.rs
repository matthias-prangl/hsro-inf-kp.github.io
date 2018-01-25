
#[derive(Debug)]
pub struct Stack {
    first: Option<Box<StackElem>>,
}

#[derive(Debug)]
struct StackElem {
    content: i32,
    next: Option<Box<StackElem>>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack{ first: None }
    }

    pub fn push(&mut self, content: i32) {
        let new_elem = Some(Box::new(StackElem {
            content, 
            next: self.first.take(),
        }));
        self.first = new_elem;
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.first.take() {
            Some(elem) => {
                let ret_elem = *elem;
                self.first = ret_elem.next;
                Some(ret_elem.content)
            },
            None => None,
        }

        //Solution using map and closure
        // self.first.take().map(|elem| {
        //     let ret_elem = *elem;
        //     self.first = ret_elem.next;
        //     ret_elem.content
        // })
    }
}