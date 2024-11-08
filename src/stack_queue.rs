use std::collections::VecDeque;
fn main() {
    let mut stack: Vec<i32> = Vec::new();
    // 压栈
    stack.push(1);stack.push(2);stack.push(3);
    // 出栈
    if let Some(x) = stack.pop() {
        println!("Popped: {}", x);
    }
    // 获取栈顶
    if let Some(&x) = stack.last() {
        println!("Top Element: {}", x);
    }
    // 是否为空
    println!("Is empty? {}", stack.is_empty());

    let mut queue: VecDeque<i32> = VecDeque::new();
    // 入队
    queue.push_back(1);queue.push_back(2);queue.push_back(3);
    // 出队
    if let Some(x) = queue.pop_front() {
        println!("Dequeued: {}", x);
    }
    // 获取队首
    if let Some(&x) = queue.front() {
        println!("Front element: {}", x);
    }
    // 是否为空
    println!("Is empty? {}", queue.is_empty());
}
