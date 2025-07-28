use std::mem;

pub struct List<T>{
    head: Link<T>
}

impl<T> List<T>  {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, val: T){
        let new_node = Box::new(Node {
            val: val,
            next: self.head.take()
        });
        self.head = Some(new_node)
    }

    pub fn pop(&mut self) -> Option<T>{
        self.head.take().map(|x| {
            self.head = x.next;
            x.val
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|x| {
            &mut x.val
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut node) = cur_link {
            cur_link = node.next.take();
        }
    }
}

// pub enum Link{
//     Empty,
//     More(Box<Node>)
// }
// basically just an option, you can use an alias

type Link<T> = Option<Box<Node<T>>>;
pub struct Node<T>{
    val: T,
    next: Link<T>
}

#[cfg(test)]
mod test {
    use crate::second::List;

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
