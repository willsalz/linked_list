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

    fn peek(&self) -> Option<Data> {
        match self.head {
            Node::Head(data, ref tail) => Some(data),
            Node::Tail => None
        }
    }

}

fn main() {
    let mut list = List::new(1);
    println!("{:?}", list);
    println!("{:?}", list.peek());
    list.push(2);
    println!("{:?}", list);
}
