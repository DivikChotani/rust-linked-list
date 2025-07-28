use std::mem;

pub struct List{
    head: Link
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, val: i32){
        //create new list
        let new_node = Box::new(Node {
            val: val,
            next: mem::replace(&mut self.head, Link::Empty)
        });
        self.head = Link::More(new_node)
    }

    pub fn pop(&mut self) -> Option<i32>{
        let a;
        match mem::replace(&mut self.head, Link::Empty){
            Link::Empty => {
                a = None
            }

            Link::More(node) => {
                a = Some(node.val);
                self.head = node.next;
            }
        };
        a
    }
}

pub enum Link{
    Empty,
    More(Box<Node>)
}

pub struct Node{
    val: i32,
    next: Link
}