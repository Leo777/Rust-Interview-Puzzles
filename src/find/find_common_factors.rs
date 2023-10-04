// Problem
// Given two positive integer numbers a and b, we need to find their common factors.
//  A common factor of a and b is an integer number that divides both a and b without a remainder.

//Notes:

// Complexity:
// Time: O(gcd(a, b))
// Space: O(1)

#[allow(dead_code)]
fn find_common_factors(a: u32, b: u32) -> Vec<u32> {
    assert!(a > 0 && b > 0, "a & b must be positive");
    fn gcd(mut a: u32, mut b: u32) -> u32 {
        while a != b {
            if a > b {
                a = a - b;
            } else {
                b = b - a;
            }
        }
        a
    }

    (1..=gcd(a, b))
        .filter(|n| a % n == 0 && b % n == 0)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(find_common_factors(18, 6), vec![1, 2, 3, 6]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_common_factors(5, 10), vec![1, 5]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(find_common_factors(252, 105), vec![1, 3, 7, 21]);
    }
}
