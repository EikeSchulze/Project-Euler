pub fn smallest_multiple(n: usize) -> usize {
    use std::cmp::max;
    let mut result = max(1, n);
    while !is_multiple(result, n) {
        result += 1;
    }
    result
}

fn is_multiple(number: usize, n: usize) -> bool {
    for i in 2..n + 1 {
        if number % i != 0 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_multiple_10() {
        assert_eq!(2520, smallest_multiple(10));
    }

    #[test]
    fn test_smallest_multiple_0() {
        assert_eq!(1, smallest_multiple(0));
    }

    #[test]
    fn test_smallest_multiple_1() {
        assert_eq!(1, smallest_multiple(1));
    }

    #[test]
    fn test_smallest_multiple_2() {
        assert_eq!(2, smallest_multiple(2));
    }

    #[test]
    fn test_smallest_multiple_3() {
        assert_eq!(6, smallest_multiple(3));
    }

    #[test]
    fn test_smallest_multiple_4() {
        assert_eq!(12, smallest_multiple(4));
    }

    #[test]
    fn test_smallest_multiple_5() {
        assert_eq!(60, smallest_multiple(5));
    }
}
