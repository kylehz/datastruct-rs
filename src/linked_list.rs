use std::rc::Weak;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

pub struct List<T> {
    head : Link<T>,
    tail : Link<T>,
    len: usize,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: WeakLink<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> List<T>  where T: Copy + Debug{
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
            len: 0
        }
    }
    pub fn push_back(&mut self, elem: T){
        let mut node = Node::new(elem);
        match self.tail.take() {
            Some(tail) => {
                tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(Rc::downgrade(&tail));
                self.tail = Some(node);
            },
            None => {
                self.head = Some(node.clone());
                self.tail = Some(node);
            },
        }
    }

    pub fn pop_back(&mut self) -> Option<T>{
        match self.tail.take() {
            Some(tail) => {
                match tail.borrow_mut().prev.take() {
                    Some(wk) => {
                        self.tail = wk.upgrade();
                        // cut the tail
                        if let Some(t) = self.tail.take() {
                            t.borrow_mut().next = None;
                            self.tail = Some(t);
                        }
                    },
                    None => (),
                }
                Some(tail.borrow_mut().elem)
            },
            None => None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let mut node = Node::new(elem);
        match self.head.take() {
            Some(head) => {
                node.borrow_mut().next = Some(head.clone());
                head.borrow_mut().prev = Some(Rc::downgrade(&node));
                self.head = Some(node);
            },
            None => {
                self.head = Some(node.clone());
                self.tail = Some(node);
            },
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head) => {
                head.borrow_mut().prev = None;
                self.head = head.borrow_mut().next.take();
                Some(head.borrow_mut().elem)
            },
            None => None,
        }
    }

    pub fn to_vec(&mut self) -> Vec<T>{
        let mut vec = vec![];
        loop {
            // self.head = None, head = nodehead
            match self.head.take() {
                None => break,
                Some(head) => {
                    // head.next = None, self.head = nextnode or None
                    vec.push(head.borrow_mut().elem);
                    self.head = head.borrow_mut().next.take();
                }
            }
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1(){
        let mut list = List::new();
        list.push_back(3);
        list.push_back(10);
        list.push_back(5);
        list.push_front(9);
        assert_eq!(list.to_vec(), vec![9, 3, 10, 5])
    }
    #[test]
    fn test2(){
        let mut list = List::new();
        list.push_back(3);
        list.push_back(10);
        list.push_back(5);
        list.push_front(9);
        list.push_back(20);
        list.push_front(30);
        assert_eq!(list.pop_back(), Some(20));
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.to_vec(), vec![9, 3, 10])
    }
}