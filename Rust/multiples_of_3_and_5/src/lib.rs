pub fn multiples_of_3_and_5_below_n(n: usize) -> Vec<usize> {
    (1..n).filter(|x| x % 3 == 0 || x % 5 == 0).collect()
}

pub fn sum_of_multiples_of_3_and_5_below_n(n: usize) -> usize {
    multiples_of_3_and_5_below_n(n).into_iter().sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_multiples_of_3_and_5_below_10() {
        let result = multiples_of_3_and_5_below_n(10);
        assert_eq!(4, result.len());
        assert!(result.iter().any(|&x| x == 3));
        assert!(result.iter().any(|&x| x == 5));
        assert!(result.iter().any(|&x| x == 6));
        assert!(result.iter().any(|&x| x == 9));
    }

    #[test]
    fn test_sum_of_multiples_of_3_and_5_below_10() {
        assert_eq!(23, sum_of_multiples_of_3_and_5_below_n(10));
    }
}
