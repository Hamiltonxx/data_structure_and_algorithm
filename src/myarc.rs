// use an Rc to give a variable more than one owner.
// = Arc in a thread. "atomic reference counter"
// let mut x = 10;
// for i in 0..10 { // Thread 1
//     x += 1;
// }
// for i in 0..10 { // Thread 2
//     x += 1;
// }
fn main () {
    let thread1 = std::thread::spawn(|| {
        for _ in 0..5 {
            println!("thread 1 is working");
        }
    });
    let thread2 = std::thread::spawn(|| {
        for _ in 0..5 {
            println!("Thread 2 is working");
        }
    });
    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("Exiting the program");
}
