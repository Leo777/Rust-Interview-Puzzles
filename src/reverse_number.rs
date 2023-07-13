// Problem
// We need to reverse a provider integer (i32) number num without converting it to a string or using axillary data structures.
//  Also, we should take care of a possible i32 overflow when reversing the number - in that case, we should return 0.

// Example 1:

// Input: 16
// Output: 4

// Example 2:

// Input: 4
// Output: 2

// Example 3:

// Input: 225
// Output: 15

//Notes:
// The idea is to decompose the original num in the loop, digit by digit from the right, using the division and modulo operations, and at the same time compose a reversed number from these digits. 
// The catch here is to avoid a possible i32 overflow in case we try to reverse some very large i32 number like 1463847413. 
// This number itself could fit in i32 range (it is less than i32::MAX = 2147483647) but the reversed number would not fit. In that case, we should return 0. 

// Complexity:
// Time: O(n)


#[allow(dead_code)]
fn reverse_number(mut num: i32) -> i32 {
    let mut num_reversed: i32 = 0;
    while num != 0 {
        if num_reversed.abs() > i32::MAX / 10 {
            return 0;
        } else {
            num_reversed = num_reversed * 10 + num % 10;
            num /= 10;
        }
    }
    num_reversed
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let num: i32 = 123;
        assert_eq!(
            reverse_number(num),
            321
        );
    }

    #[test]
    fn test_case_2() {
        let num: i32 = 120;
        assert_eq!(
            reverse_number(num),
            21
        );
    }

    #[test]
    fn test_case_3() {
        let num: i32 = 1;
        assert_eq!(
            reverse_number(num),
            1
        );
    }

    #[test]
    fn test_case_4() {
        let num: i32 = -12;
        assert_eq!(
            reverse_number(num),
            -21
        );
    }

    #[test]
    fn test_case_5() {
        let num: i32 = 0;
        assert_eq!(
            reverse_number(num),
            0
        );
    }

    #[test]
    fn test_case_6() {
        let num: i32 = -1234;
        assert_eq!(
            reverse_number(num),
            -4321
        );
    }

    #[test]
    fn test_case_7() {
        let num: i32 = -1463847413;
        assert_eq!(
            reverse_number(num),
            0
        );
    } 
}
