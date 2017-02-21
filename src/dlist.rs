use std::ptr::null;
use std::{ptr, mem};

pub mod dlist {
    pub struct Node<T> {
        data: *mut T,
        next: *mut Node,
        prev: *mut Node,
    }

    impl<T> Node<T> {
        pub fn new() -> Node {
            Node {data: ptr::null(), next: ptr::null(), prev: ptr::null()}
        }

        pub fn create(&mut self, t_data: *mut T) -> &mut Node {
            Node {data: t_data, next: ptr::null(), prev: ptr::null()}
        }
    }

    pub struct DList<T> {

    }
}