fn main() {
    let v = vec![1,2,3];
    for elem in v.iter() { // borrow, without taking ownership
        println!("{}", elem); // elem is an &i32
    }
    println!("{:?}", v);

    let numbers = vec![1,2,3,4,5];
    let sum_of_squares: i32 = numbers.clone()
        .into_iter()
        .filter(|&x| x%2!=0) // x is an i32, filter's closure takes a reference
        .map(|x| x*x) // map's closure consumes the values it works on, receives owned values, not reference
        .sum();
    println!("Sum of squares of odd numbers: {}",sum_of_squares);
//    println!("{:?}", numbers);
    let numbers: Vec<i32> = numbers.into_iter()
        .filter(|&x| x%2!=0)
        .map(|x| x*x)
        .collect();
    println!("{:?}", numbers);

    let nums = vec![1,2,3,4,5,6,7,8];
    let sum_of_odd: i32 = nums.iter().filter(|&x| x%2!=0).sum();
    println!("{}", sum_of_odd);
    let strs = vec!["hello".to_string(),"world".to_string(),"foo".to_string(),"bar".to_string()];
    let count = strs.iter().filter(|x| x.len()<4).count(); // x is &&str, .len() works on a
                                                           // reference
    println!("count: {}", count);
    let dirs = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)];
    let cnt = dirs.iter().filter(|&&(r,c)| r<0||c<0).count();
    println!("cnt: {}",cnt);
}
