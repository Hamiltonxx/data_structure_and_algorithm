// In Rust, the map method is commonly used with the Option and Result types, as well as iterators.
// It enables you to avoid match expressions and makes your code more concise and readable.
fn main() {
    let some_number = Some(5);
    let none_number: Option<i32> = None;
    let doubled_some = some_number.map(|x| x*2);
    let doubled_none = none_number.map(|x| x*2);
    println!("{doubled_some:?}  {doubled_none:?}");
    let double_plus = some_number.map(|x| x*2).map(|x| x+3);
    println!("{double_plus:?}");
    let doublenum = some_number.map_or(0, |x| x*2);
    println!("doublenum: {}", doublenum);
    let doublenum2 = none_number.map_or(0, |x| x*2);
    println!("doublenum2: {}", doublenum2);

    let success: Result<i32, &str> = Ok(5);
    let failure: Result<i32, &str> = Err("error");
    let doubled_success = success.map(|x| x*2);
    let doubled_failure = failure.map(|x| x*2);
    println!("{doubled_success:?}  {doubled_failure:?}");
    let double_plus = success.map(|x| x*2).map(|x| x+3);
    println!("{double_plus:?}");
    let doublenum3 = success.map_or(0, |x| x*2);
    let doublenum4 = failure.map_or(0, |x| x*2);
    println!("doublenum for success failure: {}  {}",doublenum3, doublenum4);

    let numbers = vec![1,2,3,4,5];
    let squared: Vec<i32> = numbers.iter().map(|x| x*x).collect();
    println!("{squared:?}");
    let squared_plus: Vec<i32> = numbers.into_iter().map(|x| x*x).map(|x| x+3).collect();
    println!("{squared_plus:?}");

    // Chaining map 
    let numbers = vec![Some(1), None, Some(3), Some(4)];
    let doubled: Vec<Option<i32>> = numbers.into_iter().map(|x| x.map(|n| n*2)).collect();
    println!("{doubled:?}");
    
}
