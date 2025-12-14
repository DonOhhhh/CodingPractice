#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    // Insert at the beginning (push)
    fn push(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // Remove from the beginning (pop)
    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    // Get the size of the list
    fn size(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        count
    }

    // Find element at index
    fn find(&self, index: usize) -> Option<i32> {
        let mut current = &self.head;
        let mut count = 0;
        while let Some(node) = current {
            if count == index {
                return Some(node.data);
            }
            count += 1;
            current = &node.next;
        }
        None
    }

    // Update element at index
    fn update(&mut self, index: usize, val: i32) -> Result<(), &'static str> {
        let mut current = &mut self.head;
        let mut count = 0;
        while let Some(node) = current {
            if count == index {
                node.data = val;
                return Ok(());
            }
            count += 1;
            current = &mut node.next;
        }
        Err("Index out of bounds")
    }

    // Insert at arbitrary index
    fn insert(&mut self, index: usize, val: i32) -> Result<(), &'static str> {
        if index == 0 {
            self.push(val);
            return Ok(());
        }

        let mut current = &mut self.head;
        let mut count = 0;

        while let Some(node) = current {
            if count == index - 1 {
                let new_node = Box::new(Node {
                    data: val,
                    next: node.next.take(),
                });
                node.next = Some(new_node);
                return Ok(());
            }
            count += 1;
            current = &mut node.next;
        }
        Err("Index out of bounds")
    }

    // Remove at arbitrary index
    fn remove(&mut self, index: usize) -> Result<i32, &'static str> {
        if index == 0 {
            return self.pop().ok_or("List is empty");
        }

        let mut current = &mut self.head;
        let mut count = 0;

        while let Some(node) = current {
            if count == index - 1 {
                if let Some(mut node_to_remove) = node.next.take() {
                    node.next = node_to_remove.next.take();
                    return Ok(node_to_remove.data);
                } else {
                    return Err("Index out of bounds");
                }
            }
            count += 1;
            current = &mut node.next;
        }
        Err("Index out of bounds")
    }

    // Display the list
    fn display(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.data);
            current = &node.next;
        }
        println!("None");
    }
}

fn main() {
    let mut list = LinkedList::new();

    println!("Pushing: 3, 2, 1");
    list.push(3);
    list.push(2);
    list.push(1);
    list.display(); // 1 -> 2 -> 3 -> None

    println!("Size: {}", list.size()); // 3

    println!("Find at index 1: {:?}", list.find(1)); // Some(2)

    println!("Update index 1 to 5");
    let _ = list.update(1, 5);
    list.display(); // 1 -> 5 -> 3 -> None

    println!("Insert 4 at index 2");
    let _ = list.insert(2, 4);
    list.display(); // 1 -> 5 -> 4 -> 3 -> None

    println!("Remove at index 1");
    println!("Removed: {:?}", list.remove(1));
    list.display(); // 1 -> 4 -> 3 -> None
}
