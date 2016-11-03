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

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self)
    }
}

#[derive(Debug)]
struct ListIterator<T> {
    list: List<T>
}

impl<T> ListIterator<T> {
    fn new(list: List<T>) -> Self {
        ListIterator { list: list }
    }
}

impl<T> Iterator for ListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}


fn main() {
    let mut list = List::new("hello");
    list.push("world");
    list.push("bye");
    list.pop();
    for val in list {
        println!("{:?}", val);
    }
}
