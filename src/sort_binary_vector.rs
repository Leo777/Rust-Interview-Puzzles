// Problem
// We need to sort a provided binary vector vec (a vector that contains only 0 or 1 numbers) in linear time.

// Example 1:

// Input: vec = [1, 0, 1, 0, 1, 0, 0, 1]
// Output: [0, 0, 0, 0, 1, 1, 1, 1]

// Example 2:

// Input: vec = [0, 0, 1, 0, 1, 1, 0, 1, 0, 0]
// Output: [0, 0, 0, 0, 0, 0, 1, 1, 1, 1]

// Example 3:

// Input: vec = [0, 0, 1, 1]
// Output: [0, 0, 1, 1]

//Notes:
// Count all zeroes in the first loop, then in the second loop insert zeroes from the beginning than ones at the end.

// Complexity:
// Time: O(n)

use std::panic;

#[allow(dead_code)]
fn sort_binary_array(numbers: &mut Vec<u8>) -> &Vec<u8> {
    let count_zeros = numbers.iter().fold(0, |acc, e| match e {
        0 => acc + 1,
        1 => acc,
        _ => panic!("Not binary"),
    });

    for index in 0..numbers.len() {
        if index < count_zeros {
            numbers[index] = 0;
        } else {
            numbers[index] = 1;
        }
    }

    return numbers;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut numbers: Vec<u8> = vec![1, 0, 1, 0, 1, 0, 0, 1];
        assert_eq!(
            sort_binary_array(&mut numbers),
            &vec![0, 0, 0, 0, 1, 1, 1, 1]
        );
    }

    #[test]
    fn test_case_2() {
        let mut numbers: Vec<u8> = vec![0, 0, 1, 0, 1, 1, 0, 1, 0, 0];
        assert_eq!(
            sort_binary_array(&mut numbers),
            &vec![0, 0, 0, 0, 0, 0, 1, 1, 1, 1]
        );
    }

    #[test]
    fn test_case_3() {
        let mut numbers: Vec<u8> = vec![0, 0, 1, 1];
        assert_eq!(
            sort_binary_array(&mut numbers),
            &vec![0, 0, 1, 1]
        );
    }
}
