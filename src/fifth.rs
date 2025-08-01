use std::{ptr};

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {head: ptr::null_mut(), tail: ptr::null_mut() }
    }

    pub fn push(&mut self, elem: T) {
        unsafe{
            let node = Box::into_raw( 
                Box::new(
                    Node {elem: elem, next: ptr::null_mut()}
            ));

            if self.tail.is_null() {
                self.head = node;
            }
            else {
                (*self.tail).next = node;
            }
            self.tail = node;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.tail.is_null() {
                None
            }
            else {
                let head = Box::from_raw(self.head);
                self.head = head.next;
                
                if (*self.head).next.is_null() {
                    self.tail = ptr::null_mut();
                }
                Some(head.elem)
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
} 