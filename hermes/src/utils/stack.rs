struct Node<T> {
    data: T,
    prev: Option<Box<Node<T>>>,
}

pub struct Stack<T> {
    size: u32,
    top: Option<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node { data, prev: None }
    }
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { size: 0, top: None }
    }

    pub fn push(&mut self, data: T) {
        let mut node = Node::<T>::new(data);

        if let Some(top) = std::mem::replace(&mut self.top, None) {
            node.prev = Some(Box::new(top));
        }

        self.top = Some(node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(top) = std::mem::replace(&mut self.top, None) {
            self.top = top.prev.map(|n| *n);
            Some(top.data)
        } else {
            None
        }
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}
