use linkedlist::{List, Queue, Stack};
use linkedlist::ListOperation;

// #[test]
// fn test_list() {
//     let mut list = List::new();
//     for i in 1..=10 {
// 	list.push(i);
//     }

//     for i in list.iter() {
// 	println!("{}", i);
//     }
// }

#[test]
fn test_queue() {
    let mut queue = Queue::new();
    for i in 1..=10 {
	queue.push(i);
    }

    for i in queue.iter() {
	println!("{}", i);
    }

    while !queue.isempty() {
	println!("{}", queue.front().unwrap());
	queue.pop();
    }

    assert_eq!(queue.pop(), None);
}

#[test]
fn test_stack() {
    let mut stack = Stack::new();
    for i in 1..=10 {
	stack.push(i);
    }

    for i in stack.iter() {
	println!("{}", i);
    }

    while !stack.isempty() {
	println!("{}", stack.top().unwrap());
	stack.pop();
    }
}