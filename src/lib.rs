use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
struct Node<T> where T: Copy {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> where T: Copy {
    fn new(data: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node{ data: data, next: None, prev: None }))
    }
}

pub struct List<T> where T: Copy {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> List<T> where T: Copy {
    pub fn new() -> Self {
        List{ head: None, tail: None }
    }
}

impl<T> List<T> where T: Copy {
    pub fn push(&mut self, data: T) {
        let node = Node::new(data);
        match self.tail.take() {
            None => {
                self.head = Some(node.clone());
                self.tail = Some(node.clone());
            }
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(old_tail);
                self.tail = Some(node);
            }
        }
    }
}


impl<T> List<T> where T: Copy {
    pub fn pop(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                None => {
                    self.head.take();
                }
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
            }
            old_tail.borrow().data
        })
    }
}

impl<T> List<T> where T: Copy {
    fn _len(& self, node: & Option<Rc<RefCell<Node<T>>>>) -> usize {
        match *node {
            None => 0,
            Some(ref n) => 1 + self._len(& n.borrow().next)
        }
    }
}

impl<T> List<T> where T: Copy {
    pub fn len(& self) -> usize {
        self._len(& self.head)
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn check_push() {
        let mut list = List::<i32>::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.len(), 3);
    }

    #[test]
    fn check_pop() {
        let mut list = List::<i32>::new();

        list.push(1);
        list.push(2);
        list.push(3);

        let mut data = list.pop().unwrap();
        assert_eq!(data, 3);
        assert_eq!(list.len(), 2);

        data = list.pop().unwrap();
        assert_eq!(data, 2);
        assert_eq!(list.len(), 1);

        data = list.pop().unwrap();
        assert_eq!(data, 1);
        assert_eq!(list.len(), 0);

        assert_eq!(list.pop(), None);
    }
}
