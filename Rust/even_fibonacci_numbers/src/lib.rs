pub fn list_fibonacci_numbers_below(n: usize) -> Vec<usize> {
    let mut vec = Vec::new();
    let mut last = 1;
    let mut next = 2;
    while last < n {
        vec.push(last);
        let temp = last + next;
        last = next;
        next = temp;
    }
    return vec;
}

pub fn list_even_fibonacci_numbers_below(n: usize) -> Vec<usize> {
    list_fibonacci_numbers_below(n).into_iter().filter(|x| x % 2 == 0).collect()
}

pub fn sum_even_fibonacci_numbers_below(n: usize) -> usize {
    list_even_fibonacci_numbers_below(n).into_iter().sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_list_fibonacci_numbers_below_100() {
        let result = list_fibonacci_numbers_below(100);
        assert_eq!(10, result.len());
        assert_eq!(1, result[0]);
        assert_eq!(2, result[1]);
        assert_eq!(3, result[2]);
        assert_eq!(5, result[3]);
        assert_eq!(8, result[4]);
        assert_eq!(13, result[5]);
        assert_eq!(21, result[6]);
        assert_eq!(34, result[7]);
        assert_eq!(55, result[8]);
        assert_eq!(89, result[9]);
    }

    #[test]
    fn test_list_even_fibonacci_numbers_below_100() {
        let result = list_even_fibonacci_numbers_below(100);
        assert_eq!(3, result.len());
        assert_eq!(2, result[0]);
        assert_eq!(8, result[1]);
        assert_eq!(34, result[2]);
    }

    #[test]
    fn test_sum_even_fibonacci_numbers_below_100() {
        assert_eq!(44, sum_even_fibonacci_numbers_below(100));
    }
}
