// Problem
// Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order,
// find two numbers such that they add up to a specific target number.
// Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 < numbers.length.

// Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.

// The tests are generated such that there is exactly one solution. You may not use the same element twice.

// Your solution must use only constant extra space.

//Notes:

// Complexity:
// Time: O(n)
// Space: O(1)

#[allow(dead_code)]
fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<usize> {
    let mut left = 0;
    let mut right = numbers.len() - 1;

    while left < right {
        let current_sum = numbers[left] + numbers[right];
        if current_sum > target {
            right -= 1;
        } else if current_sum < target {
            left += 1;
        } else {
            return vec![left + 1, right + 1];
        }
    }
    unreachable!("Test did not follow the constraints!")
}
use std::cmp::Ordering;
pub fn two_sum_idiomatic(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut l, mut r) = (0, numbers.len() - 1);
    while l < r {
        match (numbers[l] + numbers[r]).cmp(&target) {
            Ordering::Less => l += 1,
            Ordering::Greater => r -= 1,
            Ordering::Equal => return vec![l as i32 + 1, r as i32 + 1],
        }
    }
    unreachable!("Test did not follow the constraints!")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(numbers.clone(), target), vec![1, 2]);
        assert_eq!(two_sum_idiomatic(numbers, target), vec![1, 2]);
    }

    #[test]
    fn test_case_2() {
        let numbers = vec![2, 3, 4];
        let target = 6;
        assert_eq!(two_sum(numbers.clone(), target), vec![1, 3]);
        assert_eq!(two_sum_idiomatic(numbers, target), vec![1, 3]);
    }
}
