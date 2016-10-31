use std::mem;

type Data = i32;

#[derive(Debug)]
enum Node {
    Head(Data, Box<Node>),
    Tail
}

impl Node {
    fn new(data: Data) -> Self {
        Node::Head(data, Box::new(Node::Tail))
    }
}


#[derive(Debug)]
struct List {
    head: Node
}

impl List {

    fn new(data: Data) -> Self {
        List{head: Node::new(data)}
    }

    fn push(&mut self, data: Data) {
        self.head = Node::Head(data, Box::new(mem::replace(&mut self.head, Node::Tail)));
    }

    fn pop(&mut self) -> Option<Data> {
        match mem::replace(&mut self.head, Node::Tail) {
            Node::Tail => None,
            Node::Head(data, tail) => {
                self.head = *tail;
                Some(data)
            }
        }
    }

    fn peek(&self) -> Option<Data> {
        match self.head {
            Node::Head(data, ref _tail) => Some(data),
            Node::Tail => None
        }
    }

}

fn main() {
    let mut list = List::new(1);
    list.push(2);
    list.push(3);
    list.peek();
    list.pop();
    println!("{:?}", list);
}
