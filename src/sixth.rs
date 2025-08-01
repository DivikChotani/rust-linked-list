use std::{marker::PhantomData, ptr::NonNull};
pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<T>
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T, 
}


impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { front: None, back: None, len: 0, _boo: PhantomData }
    }

    pub fn push_front(&mut self, elem: T) {
        unsafe{
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node{
                front: None, back: None, elem: elem
            })));
        

            if let Some(old) = self.front {
                (*old.as_ptr()).front = Some(new);
                (*new.as_ptr()).back = Some(old);
            }
            else{
                // If there's no front, then we're the empty list and need 
                // to set the back too. Also here's some integrity checks
                // for testing, in case we mess up.
                debug_assert!(self.back.is_none());
                debug_assert!(self.front.is_none());
                debug_assert!(self.len == 0);
                self.back = Some(new);

            }
            self.front = Some(new);
            self.len +=1;
        }   
    }

    pub fn pop_front(&mut self) -> T {
        unsafe {
            self.front.map(|node| {
                let node_boxed = Box::from_raw(node.as_ptr());
                self.front = node_boxed.back;
                let res = node_boxed.elem;

                match self.front {
                    Some(node) =>{
                        (*node.as_ptr()).front = None;
                    }
                    None => {
                        debug_assert!(self.len == 1);
                        self.back = None
                    }
                }

                self.len -=1;
                res
            })
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}


#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn test_basic_front() {
        let mut list = LinkedList::new();

        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Try to break a one item list
        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Mess around
        list.push_front(10);
        assert_eq!(list.len(), 1);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        list.push_front(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_front(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }
}
