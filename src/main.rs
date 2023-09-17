mod queue;
use algotrees::queue::queues;
use algotrees::queue::queues::{Queue, TQueue};

fn main() {
    let mut queue: queues::Queue<i32> = queues::new();
    queue.push(1);
    queue.push(2);
    queue.push(3);
    println!("Queue: {:?}", queue);
    println!("Queue: {:?}", queue.pop());
    println!("Queue: {:?}", queue);

    let smaller_queue = queue.peek_range_mut(0, 2);
    println!("Smaller queue: {:?}", smaller_queue);
    let bad_queue = queue.peek_range_mut(0, 3);
    println!("Bad queue: {:?}", bad_queue);
}
