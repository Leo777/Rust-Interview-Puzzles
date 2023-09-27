// Problem
// Given a provided integer number n, we need to check whether this number is Ugly.
// Ugly numbers are a sequence of numbers whose only prime factors are 2, 3 or 5. In other words, these numbers are divisible only by 2, 3 or 5 prime numbers (but not by other prime numbers).
// The first few ugly numbers are 1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 16, 18... By convention, 1 is also included.

//Notes:
// One simple way to solve the puzzle is to iteratively divide the input number n by 2 as long as it is divisible (n % 2 == 0).
// If the current number n is not divisible by 2, then we try to divide it by 3 and 5. If the current number n that is not divisible by any of 2, 3, 5, then we could conclude that the number is not ugly (return false).
// Otherwise, we continue while n > 1. In the end, the remaining number n should be equal to 1 for the input number to be deemed Ugly.

// Complexity:
// Time: O(log(n))

#[allow(dead_code)]
fn is_ugly_number(mut n: i32) -> bool {
    while n > 1 {
        match n {
            _ if n % 2 == 0 => n /= 2,
            _ if n % 3 == 0 => n /= 3,
            _ if n % 5 == 0 => n /= 5,
            _ => return false,
        }
    }
    return n == 1;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 10;
        assert!(is_ugly_number(n));
    }

    #[test]
    fn test_case_2() {
        let n = 11;
        assert!(!is_ugly_number(n));
    }

    #[test]
    fn test_case_3() {
        let n = 12;
        assert!(is_ugly_number(n));
    }

    #[test]
    fn test_case_4() {
        let n = 13;
        assert!(!is_ugly_number(n));
    }
}
