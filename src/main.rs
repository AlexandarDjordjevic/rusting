enum Link{
    Empty,
    NoneEmpty(Box<Node>),
}

struct Node{
    left : Link,
    right : Link,
    value : u32
}

pub struct BST {
    nodes : Vec<Node>,
    item_count : u32
}

impl BST{

    pub fn size(&self) -> u32{
        self.item_count
    }
}

impl BST{
    pub fn new() -> Self {
        BST {
            item_count : 0,
            nodes : Vec::new()
        }
    }
}

fn main() {
    println!("Hello, world!");
}
