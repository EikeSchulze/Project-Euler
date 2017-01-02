pub fn is_palindrome(n: u64) -> bool {
    let number : Vec<char> = n.to_string().chars().collect();
    let length = number.len();
    for i in 0..length / 2 {
        if number[i] != number[length - i - 1] {
            return false;
        }
    }
    return true;
}

pub fn largest_palindrome_product(n: usize) -> (usize, usize, u64) {
    let mut multiplicant1 = 0;
    let mut multiplicant2 = 0;
    let mut largest: u64 = 0;
    for x in 1..n {
        for y in 1..x + 1 {
            let product: u64 = x as u64 * y as u64;
            if product > largest && is_palindrome(product) {
                largest = product;
                multiplicant1 = x;
                multiplicant2 = y;
            }
        }
    }
    return (multiplicant1, multiplicant2, largest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_true() {
        let tests = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 
                         11, 22, 33, 44, 55, 66, 77, 88, 99, 
                         101, 111, 121, 131, 141, 151, 161, 171, 181, 191,
                         202, 212, 222, 272, 323, 333, 444, 494, 1001, 9449,
                         12321, 123323321];
        for x in tests {
            assert!(is_palindrome(x));
        }
    }

    #[test]
    fn test_is_palindrome_false() {
        let tests = vec![12, 32, 39, 54, 85, 46, 70, 84, 49, 
                         100, 211, 221, 231, 143, 155, 761, 371, 981, 291,
                         1011, 9459,
                         12341, 123223321];
        for x in tests {
            assert!(!is_palindrome(x));
        }
    }

    #[test]
    fn test_largest_palindrome_product() {
        use std::cmp::{min, max};

        let (mul1, mul2, result) = largest_palindrome_product(100);
        assert_eq!(9009, result);
        assert_eq!(91, min(mul1, mul2));
        assert_eq!(99, max(mul1, mul2));
    }
}
