use std::collections::VecDeque;

struct Queue {
    items: VecDeque<i32>
}

impl Queue {
    fn new() -> Self {
        Queue { items: VecDeque::new() }
    }

    fn enqueue(&mut self, value: i32) {
        self.items.push_back(value);
    }

    fn dequeue(&mut self) -> Option<i32> {
        self.items.pop_front()
    }

    fn peek(&mut self) -> Option<&i32> {
        self.items.front()
    }
}

fn main() {
    
    // Queues are FIFO

    let mut queue = Queue::new();

    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    println!("Front of the queue: {:?}", queue.peek());

    queue.dequeue();

    println!("After dequeue, front of the queue: {:?}", queue.peek());
}
