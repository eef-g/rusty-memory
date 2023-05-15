mod node;

#[derive(Debug)]
pub struct List<T> {
    head: Option<Box<node::Node<T>>>,
    tail: Option<Box<node::Node<T>>>,
    length: u64,
}

impl<T> List<T> {
    // Constructor
    pub fn new() -> Self {
        println!("Making a new List");
        List {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn ToString(&mut self) -> String {
        let mut result = String::new();
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
        result
    }

    pub fn push(&mut self, data: T) {
        let mut new_node = node::Node::new(data);
        match self.head.take() {
            Some(old_head) => {
                new_node.next = Some(old_head);
                self.head = Some(Box::new(new_node));
            }
            None => {
                self.head = Some(Box::new(new_node));
            }
        }
        self.length += 1;
    }
}