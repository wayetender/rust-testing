// Pure ordered linked list
struct Node {
    elem: i32,
    next: Option<Box<Node>>
}
pub struct List {
    head: Option<Box<Node>>
}

impl List {
    pub fn new() -> List {
        List { head: None }
    }
    pub fn insert(&mut self, elem: i32) {
        let mut cur = &mut self.head;
        loop {
            let tmp = cur;
            if tmp.as_ref().map(|n| n.elem <= elem).unwrap_or(false) { 
                cur = &mut tmp.as_mut().unwrap().next; 
            } else {
                cur = tmp; 
                break; 
            }
        };
        replace!(cur, |cur| Some(Box::new(Node { elem: elem, next: cur })))
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
    pub fn peek(&self) -> Option<i32> {
        self.head.as_ref().map(|n| { n.elem })
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
        assert_eq!(list.peek(), Some(3));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }
}
