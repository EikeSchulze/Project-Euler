pub fn sum_square_difference(n: u32) -> u64 {
    let n = n as u64;
    let gauss_sum = (n * (n + 1)) / 2;
    let gauss_sum_squared = gauss_sum * gauss_sum;
    let sum_of_squares: u64 = (1..n + 1).map(|x| x * x).sum();
    gauss_sum_squared - sum_of_squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_square_difference() {
        assert_eq!(0, sum_square_difference(0));
        assert_eq!(0, sum_square_difference(1));
        assert_eq!(4, sum_square_difference(2));
        assert_eq!(22, sum_square_difference(3));
        assert_eq!(2640, sum_square_difference(10));
    }
}
