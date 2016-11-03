use std::mem;

#[derive(Debug)]
enum Node<T> {
    Head(T, Box<Node<T>>),
    Tail
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node::Head(data, Box::new(Node::Tail))
    }
}


#[derive(Debug)]
struct List<T> {
    head: Node<T>
}

impl<T> List<T> {

    fn new(data: T) -> Self {
        List{head: Node::new(data)}
    }

    fn push(&mut self, data: T) {
        self.head = Node::Head(data, Box::new(mem::replace(&mut self.head, Node::Tail)));
    }

    fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, Node::Tail) {
            Node::Tail => None,
            Node::Head(data, tail) => {
                self.head = *tail;
                Some(data)
            }
        }
    }

    fn peek(&self) -> Option<&T> {
        match self.head {
            Node::Tail => None,
            Node::Head(ref data, ref _tail) => {
                Some(data)
            }
        }
    }

}

fn main() {
    let mut list = List::new("hello");
    list.push("world");
    list.push("bye");
    list.pop();
    println!("{:?}", list.peek());
}
