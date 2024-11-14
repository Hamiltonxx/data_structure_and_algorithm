fn main() {
    let numbers = vec![1,2,3,4,5];
    let mut sum = 0;
    for num in &numbers {
        sum += num;
    }
    println!("{sum}");
    let sum2:i32 = numbers.iter().fold(0, |acc,&x| acc+x);
    println!("{sum2}");
    let product = numbers.iter().fold(1, |acc,&x| acc*x);
    println!("{product}");

//    let even_nums: Vec<i32> = numbers.iter().filter(|&x| x%2==0).collect();
//    不管是.filter(|&x|)还是.filter(|&&x|)都通不过
//    let even_nums: Vec<i32> = numbers.iter().map(|&x| x).filter(|x| x%2==0).collect();
//    let even_nums: Vec<i32> = numbers.iter().cloned().filter(|x| x%2==0).collect();
    let even_nums: Vec<i32> = numbers.into_iter().filter(|x| x%2==0).collect(); //numbers moved
//    let even_nums: Vec<i32> = numbers.clone().into_iter().filter(|x| x%2==0).collect();
    println!("{even_nums:?}");
//    println!("{numbers:?}");

    let numbers = vec![10,20,30,-40,50];
    let result: Result<i32,&str> = numbers.iter().try_fold(0, |acc, &n| {
        if n < 0 {
            Err("Negative number encountered")
        } else {
            Ok(acc + n)
        }
    });
    match result {
        Ok(sum) => println!("{sum}"),
        Err(e) => println!("{e}"),
    }
}
