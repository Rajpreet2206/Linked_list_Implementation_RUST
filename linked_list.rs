struct Node<T>{
    data: T,
    next: Option<Box<Node<T>>>,
}

//Implementing new, push, pop and length methods for LinkedList

impl<T> LinkedList<T>{
    pub fn new() -> Self{
        LinkedList {head: None}
    }

    pub fn push(&mut self, data: T){
        let node = Box::new(Node{
            data: data,
            next: self.head.take(),
        });
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T>{
        let val = self.head.take();
        match val{
            Some(mut node) => {
                self.head = node.next.take();
                Some(node.data)
            },
            None => None
        }
    }

    pub fn length(&self) -> u8{
        let mut len = 0;
        let mut n = &self.head;
        while let Some(ref node) = *n{
            n = &node.next;
            len += 1;
        }
        len
    }
}


// Now we create an instance for a LinkedList structure using the new method,
// and perform the operations push, pop and length

fn main(){
    let mut list = LinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    println!("Pop {}", list.pop().unwrap());
    list.push(4);
    println!("Pop {}", list.pop().unwrap());
    println!("length = {}", list.length());
}