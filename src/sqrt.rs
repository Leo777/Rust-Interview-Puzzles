// Problem
// We need to calculate the square root of a provided integer number num without using built-in functions.
// The result should be rounded down in case it is not a whole number.

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
// Use Binary Search. The idea is to divide the interval of numbers by 2 to get the middle number mid and then continue the search within either the left or the right half of the interval, repeating the exercise while left < right.

// Complexity:
// Time: O(log n)


#[allow(dead_code)]
fn sqrt(num: i32) -> i32 {
    assert!(num >= 0);

    let mut left = 0;
    let mut right = num;

    while left < right {
        let mid = (left + right + 1) / 2;

        if mid * mid > num {
            right = mid - 1;
        } else {
            left = mid;
        }
    }
    right
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let num: i32 = 16;
        assert_eq!(
            sqrt(num),
            4
        );
    }

    #[test]
    fn test_case_2() {
        let num: i32 = 4;
        assert_eq!(
            sqrt(num),
            2
        );
    }

    #[test]
    fn test_case_3() {
        let num: i32 = 225;
        assert_eq!(
            sqrt(num),
            15
        );
    }
}
