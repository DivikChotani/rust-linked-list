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

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut node) = cur_link {
            cur_link = mem::replace(&mut node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
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

#[cfg(test)]
mod test {
    use crate::first::List;

    #[test]
    fn basics() {

        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
