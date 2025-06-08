struct Stack {
    items: Vec<i32>,
}

impl Stack {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, value: i32) {
        self.items.push(value);
    }

    fn pop(&mut self) -> Option<i32> {
        self.items.pop()
    }

    fn peek(&mut self) -> Option<&i32> {
        self.items.last()
    }
}

fn main() {
    // Stacks are LIFO

    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    println!("Top of the stack: {:?}", stack.peek());

    stack.pop();

    println!("After popping, top of the stack: {:?}", stack.peek());
}
