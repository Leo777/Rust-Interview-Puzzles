// Problem
// Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.

// You must write an algorithm with O(log n) runtime complexity.

// Complexity:
// Time: O(log n)
use std::cmp::Ordering;

#[allow(dead_code)]
fn binary_search(numbers: Vec<i32>, target: i32) -> i32 {
    let (mut l, mut r) = (0 as usize, numbers.len() -1);

    while l <= r {
        let middle = l + (r - l) /2;
        match target.cmp(&numbers[middle]) {
            Ordering::Equal => return middle as i32,
            Ordering::Greater => r = middle,
            Ordering::Less => l = middle - 1,
        }
    }
    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers: Vec<i32> = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(binary_search(numbers, 9), 4);
    }

    #[test]
    fn test_case_2() {
        let numbers: Vec<i32> = vec![-1,0,3,5,9,12];
        assert_eq!(binary_search(numbers, 2), -1);
    }
}
