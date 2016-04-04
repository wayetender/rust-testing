// Pure ordered linked list -- only uses Box and mem::replace

use std::mem;

struct Node {
    elem: i32,
    next: Option<Box<Node>>,
}

pub struct List {
    head: Option<Box<Node>>
}

impl List {
    pub fn new() -> List {
        List { head: None }
    }
    // // Recursive way:
    // fn find_node<'a>(node : &'a mut Option<Box<Node>>, elem: i32) -> &'a mut Option<Box<Node>> {
    //     let recurse = match *node {
    //         None => false,
    //         Some(ref n) => if n.elem > elem { false } else { true }
    //     };
    //     if recurse { List::find_node(&mut node.as_mut().unwrap().next, elem) } else { node }
    // }

    // Iterative way:
    fn find_node(mut node : &mut Option<Box<Node>>, elem: i32) -> &mut Option<Box<Node>> {
        loop {
            let cur = node;
            if cur.is_none() || cur.as_mut().unwrap().elem > elem { 
                node = cur; 
                break; 
            } else {
                node = &mut cur.as_mut().unwrap().next; 
            }
        };
        node
    }
    pub fn insert(&mut self, elem: i32) {
        let mut node = List::find_node(&mut self.head, elem);
        let insert_point = node.take();
        let new_node = Some(Box::new(Node { elem: elem, next: insert_point }));
        mem::replace(node, new_node);

    }
    pub fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
            None => None,
            Some(node) => {
                let node = *node;
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.insert(2);
        list.insert(4);
        list.insert(1);
        list.insert(3);
        list.insert(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }
}
