use std::cmp::Ordering;

pub trait Comparable {
    fn compare(&self, other: &Self) -> Ordering;
}

impl Comparable for i32 {
    fn compare(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl Comparable for f64 {
    fn compare(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl Comparable for String {
    fn compare(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

pub fn generic_sort<T: Comparable>(data: &mut [T]) {
    let n = data.len();
    for i in 0..n {
        for j in 0..n-1-i {
            if data[j].compare(&data[j+1]) == Ordering::Greater {
                data.swap(j, j+1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_integers() {
       let mut data = vec![5,2,8,1,3];
       generic_sort(&mut data);
       assert_eq!(data, vec![1,2,3,5,8]);
    }

    #[test]
    fn test_sort_floats() {
        let mut data = vec![5.5, 2.2, 8.8, 1.1, 3.3];
        generic_sort(&mut data);
        assert_eq!(data, vec![1.1, 2.2, 3.3, 5.5, 8.8]);
    }

    #[test]
    fn test_sort_strings() {
        let mut data = vec!["apple".to_string(), "orange".to_string(), "banana".to_string()];
        generic_sort(&mut data);
        assert_eq!(data, vec!["apple".to_string(), "banana".to_string(), "orange".to_string()]);
    }
}
