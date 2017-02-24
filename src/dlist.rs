pub mod dlist {
    use std::mem;
    use std::ptr;

    pub struct Dlist<T> {
        head: Option<Box<Node<T>>>,
        tail: Link<Node<T>>,
    }

    struct Link<T> { 
        item: *mut T 
    }

    impl<T> Copy for Link<T> {}

    impl<T> Clone for Link<T> {
        fn clone(&self) -> Self { 
            Link { item: self.item}
        }   
    }

    pub struct Node<T> {
        next: Option<Box<Node<T>>>,
        prev: Link<Node<T>>,
        val: T,
    }

    impl<T> Dlist<T> {
        //checks if list is empty
        pub fn empty(&self) -> bool {
            self.head.is_none()
        }

        //gets total number of nodes in the list
        pub fn length(&self) -> usize {
            let mut node = &self.head;
            let mut i = 0;

            loop {
                match *node {
                    Some(ref n) => {
                        i += 1;
                        node = &n.next;
                    }
                    None => {
                        return i;
                    }
                }
            }
        }

        //default constructor
        pub fn new() -> Dlist<T> {
            Dlist {head: None, tail: Link::none()}
        }

        //pushes a new node onto the list
        pub fn push(&mut self, entry: T) {
            self.push_front_node(Box::new(Node::new(entry)))
        }

        //helper function for push()
        pub fn push_front_node(&mut self, mut new_head: Box<Node<T>>) {
            match self.head {
                None => {
                    self.tail = Link::some(&mut new_head);
                    new_head.prev = Link::none();
                    self.head = Some(new_head);
                }
                Some(ref mut head) => {
                    new_head.prev = Link::none();
                    head.prev = Link::some(&mut new_head);
                    mem::swap(head, &mut new_head);
                    head.next = Some(new_head);
                }
            }
        }

        //ListIterator call function
        pub fn look<'a>(&'a self) -> ListIterator<'a, T> {
            ListIterator {nelem: self.length(), head: &self.head, tail: self.tail}
        }
    }   

    impl<T> Node<T> {
        //Creates a new node with a value
        fn new(v: T) -> Node<T> {
            Node{val: v, next: None, prev: Link::none()}
        }
    }

    //Link contains a raw pointer for type T, as well as methods to handle it
    impl<T> Link<T> {
        //Sets an Link item to null pointer
        fn none() -> Link<T> {
            Link {item: ptr::null_mut()}
        }   

        //Creates a Link for the item
        fn some(n: &mut T) -> Link<T> {
            Link {item: n as *mut T}
        }

        //Convert Link to an Option value, immutable
        fn resolve_immut<'a>(&self) -> Option<&'a T> {
            unsafe { self.item.as_ref() }
        }

        //Convert Link to an Option value, mutable 
        fn resolve<'a>(&mut self) -> Option<&'a mut T> {
            unsafe { self.item.as_mut() }
        }

        //Returns a Link and replaces it with a null pointer, using none()
        fn take(&mut self) -> Link<T> {
            mem::replace(self, Link::none())
        }
    }

    pub struct ListIterator<'a, T: 'a> {
        head: &'a Option<Box<Node<T>>>,
        tail: Link<Node<T>>,
        nelem: usize,
    }

    impl<'a, A> Iterator for ListIterator<'a, A> {
        type Item = &'a A;

        #[inline]
        fn next(&mut self) -> Option<&'a A> {
            if self.nelem == 0 {
                return None;
            }

            self.head.as_ref().map(|head| {
                self.nelem -= 1;
                self.head = &head.next;
                &head.val
            })
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            (self.nelem, Some(self.nelem))
        }
    }

    impl<'a, A> DoubleEndedIterator for ListIterator<'a, A> {
        #[inline]
        fn next_back(&mut self) -> Option<&'a A> {
            if self.nelem == 0 {
                return None;
            }

            let temp = self.tail.resolve_immut();

            temp.as_ref().map(|prev| {
                self.nelem -= 1;
                self.tail = prev.prev;
                &prev.val
            })
        }
    }
}