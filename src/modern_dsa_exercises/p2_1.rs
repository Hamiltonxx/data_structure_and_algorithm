#[test]
fn test_stack() {    
    let mut stack:Vec<i32> = Vec::new();
    stack.push(1);stack.push(2);stack.push(3);
    assert_eq!(Some(3), stack.pop());
    assert_eq!(Some(&2), stack.last());
    assert!(!stack.is_empty());
}

#[test]
fn test_queue() {    
    use std::collections::VecDeque;
    let mut queue:VecDeque<i32> = VecDeque::new();
    queue.push_back(1);queue.push_back(2);queue.push_back(3);
    assert_eq!(Some(1), queue.pop_front());
    assert_eq!(Some(&2), queue.front());
    assert!(!queue.is_empty());
}

