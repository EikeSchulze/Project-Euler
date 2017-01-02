pub fn largest_product_in_the_series(series: &[u8], n: usize) -> u64 {
    if n == 0 {
        return 0;
    }

    series.windows(n).map(|subseries| product(subseries)).max().unwrap_or(0)
}

fn product(subseries: &[u8]) -> u64 {
    subseries.iter().map(|&x| x as u64).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_product_in_the_series_empty1() {
        assert_eq!(0, largest_product_in_the_series(&[], 0));
    }

    #[test]
    fn test_largest_product_in_the_series_empty2() {
        assert_eq!(0, largest_product_in_the_series(&[], 12));
    }

    #[test]
    fn test_largest_product_in_the_series_empty3() {
        assert_eq!(0, largest_product_in_the_series(&[1, 2, 3], 0));
    }

    #[test]
    fn test_largest_product_in_the_series_1() {
        assert_eq!(0, largest_product_in_the_series(&[0], 1));
        assert_eq!(1, largest_product_in_the_series(&[1], 1));
        assert_eq!(2, largest_product_in_the_series(&[2], 1));
        assert_eq!(3, largest_product_in_the_series(&[3], 1));
        assert_eq!(4, largest_product_in_the_series(&[4], 1));
        assert_eq!(5, largest_product_in_the_series(&[5], 1));
        assert_eq!(6, largest_product_in_the_series(&[6], 1));
        assert_eq!(7, largest_product_in_the_series(&[7], 1));
        assert_eq!(8, largest_product_in_the_series(&[8], 1));
        assert_eq!(9, largest_product_in_the_series(&[9], 1));
        assert_eq!(9, largest_product_in_the_series(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 1));
    }

    #[test]
    fn test_largest_product_in_the_series_2() {
        assert_eq!(72, largest_product_in_the_series(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 2));
        assert_eq!(56, largest_product_in_the_series(&[9, 0, 1, 2, 3, 4, 5, 6, 7, 8], 2));
    }
}
