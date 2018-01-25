
#[derive(Debug)]
pub struct Stack<'a> {
    first: Option<Box<StackElem<'a>>>,
}

#[derive(Debug)]
struct StackElem<'a> {
    content: &'a str,
    next: Option<Box<StackElem<'a>>>,
}

impl<'a> Stack<'a> {
    pub fn new() -> Stack<'a> {
        Stack{ first: None }
    }

    pub fn push(&mut self, content: &'a str) {
        let new_elem = Some(Box::new(StackElem {
            content, 
            next: self.first.take(),
        }));
        self.first = new_elem;
    }

    pub fn pop(&mut self) -> Option<&str> {
        match self.first.take() {
            Some(elem) => {
                let ret_elem = *elem;
                self.first = ret_elem.next;
                Some(ret_elem.content)
            },
            None => None,
        }

        //Solution using map and closure to avoid match
        // self.first.take().map(|elem| {
        //     let ret_elem = *elem;
        //     self.first = ret_elem.next;
        //     ret_elem.content
        // })
    }
}