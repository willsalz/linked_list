#[derive(Debug)]
enum LinkedList {
    Node {data: i32, next: Box<LinkedList>},
    Tail
}

impl LinkedList {
    fn new(data: i32, next: LinkedList) -> LinkedList {
        LinkedList::Node{data: data, next: Box::new(next)}
    }

    fn tail() -> LinkedList {
        LinkedList::Tail
    }
}

fn main() {
    let list: LinkedList = LinkedList::new(1, LinkedList::new(2, LinkedList::tail()));
    println!("{:?}", list);
}
