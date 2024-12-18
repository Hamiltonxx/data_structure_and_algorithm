use std::time::Instant;
use std::mem;
use rand::random;

#[cfg(test)]
mod tests {
    use super::*;

    const SIZE:usize = 100_000;

    #[test]
    fn test_array_sort() {
        let mut data:[i32;SIZE] = [0;SIZE];
        for i in 0..SIZE {
            data[i] = random::<i32>();
        }
        let now = Instant::now();
        data.sort();
        let duration = now.elapsed();
        println!("Array Sort took: {:?}", duration);
    }

    #[test]
    fn test_vec_sort() {
        let mut data:Vec<i32> = Vec::with_capacity(SIZE);
        for _ in 0..SIZE {
            data.push(random::<i32>());
        }
        let now = Instant::now();
        data.sort();
        let duration = now.elapsed();
        println!("Vector Sort took: {:?}", duration);
    }

    #[test]
    fn test_array_memory_usage() {
        let data: [i32;SIZE] = [0; SIZE];
        println!("Array memory usage: {} bytes", mem::size_of_val(&data));
    }

    #[test]
    fn test_vec_memory_usage() {
        let mut data:Vec<i32> = Vec::with_capacity(SIZE);
        for _ in 0..SIZE {
            data.push(random::<i32>())
        }
        println!("Vector memory usage: {} bytes", data.len() * std::mem::size_of::<i32>()); // mem::size_of_val(&data));
    }
}
