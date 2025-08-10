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

pub struct Iter<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<&'a T>

}

pub struct IntoIter<T> {
    list: LinkedList<T>,
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
                self.back = Some(new);

            }
            self.front = Some(new);
            self.len +=1;
        }   
    }

    pub fn push_back(&mut self, elem: T) {
        unsafe{
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node{
                front: None, back: None, elem: elem
            })));
        

            if let Some(old) = self.back {
                (*old.as_ptr()).back = Some(new);
                (*new.as_ptr()).front = Some(old);
            }
            else{
                self.front = Some(new);

            }
            self.back = Some(new);
            self.len +=1;
        }   
    }

    pub fn pop_front(&mut self) -> Option<T> {
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
                        self.back = None
                    }
                }

                self.len -=1;
                res
            })
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            self.back.map(|node| {
                let node_boxed = Box::from_raw(node.as_ptr());
                self.back = node_boxed.front;
                let res = node_boxed.elem;

                match self.back {
                    Some(node) =>{
                        (*node.as_ptr()).back = None;
                    }
                    None => {
                        self.front = None
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

    pub fn front(&self) -> Option<& T> {
        unsafe {
            Some(&(*self.front?.as_ptr()).elem)
        }
    }

     pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe {
            Some(&mut (*self.front?.as_ptr()).elem)
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { front: self.front, back: self.back, len: self.len, _boo: PhantomData }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self){
        while let Some(_) = self.pop_front() {
            
        }
    }
}

impl<T> Drop for LinkedList<T>  {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

impl<T> Default for LinkedList<T>{
    fn default() -> Self {
        LinkedList::new()
    }
}

impl<T:Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut clone = Self::new();
        for node in self {
            clone.push_back(node.clone());
        }
        clone
    }
}

impl<T> Extend<T> for LinkedList<T>{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter{
        self.iter()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        
        if self.len > 0 {
            self.front.map(|node| unsafe{
                self.len -=1;
                self.front = (*node.as_ptr()).back;
                &(*node.as_ptr()).elem
            })
        }
        else{
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<T> Iterator for IntoIter<T>{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.list.len
    }
}
impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        
        if self.len > 0 {
            self.back.map(|node| unsafe{
                self.len -=1;
                self.back = (*node.as_ptr()).front;
                &(*node.as_ptr()).elem
            })
        }
        else{
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
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
