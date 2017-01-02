pub fn largest_prime_factor(n: u64) -> u64 {
    if n <= 3 {
        return n;
    }
    let mut factors = Vec::new();

    let mut counter = 2;
    let mut remainder = n;

    while counter <= remainder {
        if remainder % counter == 0 && !factors.iter().any(|&x| counter % x == 0) {
            factors.push(counter);
            remainder = remainder / counter;
        }

        counter += 1;
    }

    return factors.pop().unwrap_or(n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_prime_factor_0() {
        assert_eq!(0, largest_prime_factor(0));
    }

    #[test]
    fn test_largest_prime_factor_1() {
        assert_eq!(1, largest_prime_factor(1));
    }

    #[test]
    fn test_largest_prime_factor_2() {
        assert_eq!(2, largest_prime_factor(2));
    }

    #[test]
    fn test_largest_prime_factor_3() {
        assert_eq!(3, largest_prime_factor(3));
    }

    #[test]
    fn test_largest_prime_factor_4() {
        assert_eq!(2, largest_prime_factor(4));
    }

    #[test]
    fn test_largest_prime_factor_5() {
        assert_eq!(5, largest_prime_factor(5));
    }

    #[test]
    fn test_largest_prime_factor_6() {
        assert_eq!(3, largest_prime_factor(6));
    }

    #[test]
    fn test_largest_prime_factor_7() {
        assert_eq!(7, largest_prime_factor(7));
    }

    #[test]
    fn test_largest_prime_factor_8() {
        assert_eq!(2, largest_prime_factor(8));
    }

    #[test]
    fn test_largest_prime_factor_9() {
        assert_eq!(3, largest_prime_factor(9));
    }

    #[test]
    fn test_largest_prime_factor_10() {
        assert_eq!(5, largest_prime_factor(10));
    }

    #[test]
    fn test_largest_prime_factor_13195() {
        assert_eq!(29, largest_prime_factor(13195));
    }
}
