fn main() {
    // $0: an initial value for the accumulator, $1: a closure(the accumulator, an item)
    // fold can be used when to transform a collection into a single value.
    let even_sum = (1..=10).fold(0, |acc,num| if num%2==0{acc+num} else{acc});
    println!("{even_sum:?}");
    let even_sum_2: i32 = (0..=10).filter(|&n| n%2==0).sum();
    println!("{even_sum_2:?}");

    let numbers = vec![3,7,-2,9,1,-5,4,8,6];
    let non_negative: Vec<i32> = numbers.iter().cloned().filter(|&x| x>=0).collect();
    println!("{numbers:?}");
    println!("{non_negative:?}");
    let mean = calculate_mean(&non_negative);
    println!("Mean: {:.2}", mean);
    let median = calculate_median(&non_negative);
    println!("Median: {:.2}", median);
}

fn calculate_mean(numbers: &[i32]) -> f64 {
    let sum: i32 = numbers.iter().sum();
    let count = numbers.len();
    sum as f64 / count as f64
}

fn calculate_median(numbers: &[i32]) -> f64 {
    let mut sorted = numbers.to_vec();
    sorted.sort();
    let len = sorted.len();
    if len % 2 == 0 {
        (sorted[len/2-1] + sorted[len/2]) as f64 / 2.0
    } else {
        sorted[len/2] as f64
    }    
}
