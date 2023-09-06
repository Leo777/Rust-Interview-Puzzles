// Problem
// Given a non-negative number n, we need to check if the number is a Prime number or not.

//Notes:
// As per the prime number definition, a prime number has to be >1. 
// Also, an even number (except for 2) could not be a prime number because it would be always divisible by 2.

// Then we iterate over odd numbers only i = 3, 5, 7... while i <= sqrt(n) and check if n is divisible by any i number.
// If that is the case, then the number n could not be prime. 
// Note, that we could take a shortcut and iterate only up to sqrt(n) instead of doing it up to n. 
// If the number is not prime, iterating till its square root should be enough to identify that, which saves us some execution time.

// Complexity:
// Time: O(sqrt(n))

#[allow(dead_code)]
fn is_prime_number(n: i32) -> bool {
    match n {
        _ if n <= 1 => return false,
        _ if n == 2 => return true,
        _ if n % 2 == 0 => return false,
        _ => (),
    }

    let sqrt = (n as f32).sqrt() as i32;

    let mut i = 3;

    while i < sqrt {
        if i % 2 == 0 {
            return false;
        }
        i += 2;
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 3;

        assert_eq!(is_prime_number(n), true);
    }

    #[test]
    fn test_case_2() {
        let n = 0;

        assert_eq!(is_prime_number(n), false);
    }

    #[test]
    fn test_case_3() {
        let n = 5;

        assert_eq!(is_prime_number(n), true);
    }

    #[test]
    fn test_case_4() {
        let n = 10;

        assert_eq!(is_prime_number(n), false);
    }
}
