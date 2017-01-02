pub fn special_pythagorean_triplet_product(sum: usize) -> Option<usize> {
    a_loop(sum).map(|(a, b, c)| a * b * c)
}

fn a_loop(sum: usize) -> Option<(usize, usize, usize)> {
    (1..sum).filter_map(|a| b_loop(a, sum - a)).nth(0)
}

fn b_loop(a: usize, b_max: usize) -> Option<(usize, usize, usize)> {
    (a + 1..b_max + 1).filter(|&b| c_check(a, b, b_max - b)).map(|b| (a, b, b_max - b)).nth(0)
}

fn c_check(a: usize, b: usize, c: usize) -> bool {
    a * a + b * b == c * c
}

pub fn isqrt(n: usize) -> Option<usize> {
    (0..n + 1).find(|x| x * x == n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sptp() {
        assert_eq!(Some(3 * 4 * 5), special_pythagorean_triplet_product(3 + 4 + 5));
    }

    #[test]
    fn test_isqrt() {
        assert_eq!(Some(0), isqrt(0));
        assert_eq!(Some(1), isqrt(1));
        assert_eq!(Some(2), isqrt(4));
        assert_eq!(Some(3), isqrt(9));
        assert_eq!(Some(4), isqrt(16));
        assert_eq!(Some(5), isqrt(25));
    }

    #[test]
    fn test_isqrt_2() {
        assert_eq!(None, isqrt(2));
        assert_eq!(None, isqrt(3));
        assert_eq!(None, isqrt(5));
        assert_eq!(None, isqrt(6));
        assert_eq!(None, isqrt(7));
        assert_eq!(None, isqrt(8));
    }
}
