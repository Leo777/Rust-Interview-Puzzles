// Problem
//For a provided non-negative integer number n we need to check if this number is a power of two. 
// A positive integer number n is a power of 2 if it could be represented as n = 2 ^ m, where m is some non-negative integer number.


// Example 1:

// Input: 0	
// Output: false

// Example 2:

// Input: 1
// Output: true
// Explanation: n = 2^0 = 1

// Example 3:

// Input: 2
// Output: true
// Explanation: n = 2^1 = 2

// Notes:
// The actual solution is based on the observation that the power of 2 numbers n in binary form look like that (n = 1, 2, 4, 8, 16...): 1, 10, 100, 1000, 10000.... 
// In other words, they always have a leftmost 1 and then all zeros to the right. And if we substruct n - 1 from these numbers (n - 1 = 0, 1, 3, 7, 15...), we would get in binary form: 0, 1, 11, 111, 1111.... 
// In other words, the binary numbers would always (except for 0) have all 1s.
//  It means that if we apply the bitwise AND operation to both of these numbers (n & n - 1) we should always get 0 as a result (the numbers would neutralize each other). Also, we process n != 0 specifically, because 0 could not be a power of 2 anyway.

// Time complexity O(1)

#[allow(dead_code)]
fn power_of_two(number: i32) -> bool {
   (number != 0) && (number & number - 1 == 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let number: i32 = 0;
        assert_eq!(power_of_two(number), false);
    }

    #[test]
    fn test_case_2() {
        let number: i32 = 1;
        assert_eq!(power_of_two(number), true);
    }

    #[test]
    fn test_case_3() {
        let number: i32 = 2;
        assert_eq!(power_of_two(number), true);
    }
}
