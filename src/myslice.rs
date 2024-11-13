fn main() {
    let data = [1,2,3,4,5];
    for window in data.windows(3) {
        println!("{:?}", window);
    }
    let lst = [3,4];
    // 结合 any 
    let contains = data.windows(lst.len()).any(|window| window==lst);
    println!("{}", contains);
}
