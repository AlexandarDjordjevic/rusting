enum Link {
    Empty,
    NoneEmpty(Box<Node>),
}

struct Node {
    left: Link,
    right: Link,
    value: u32,
}

struct BinarySearchTree {
    head: Link,
    item_count: u32,
}

impl BinarySearchTree {
    fn new() -> Self {
        BinarySearchTree {
            head: Link::Empty,
            item_count: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.item_count == 0
    }

    fn size(&self) -> u32 {
        self.item_count
    }

    fn push(&mut self, item : u32){
        let new_node = Node {
            left : Link::Empty,
            right : Link::Empty,
            value : item,
        };
        if self.is_empty() {
            self.head = Link::NoneEmpty{
                
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    let bst = BinarySearchTree::new();
    if bst.is_empty() {
        println!("Bst is empty!");
    } else {
        println!("Item count {}", bst.size());
    }
}
