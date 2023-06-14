// Problem
//In a provided vector vec we need to find the pivot index.
// The pivot index is an index of the pivot element, the element before which all elements are less than the pivot and after which all elements are greater than the pivot.

// Example 1:
// Input: vec = [5, 3, 4, 6, 2, 7, 10, 8]
// Output: 5
// Explanation: The element 7 (with the index 5) is the pivot because all elements before 7 are less and all elements after are greater.

// Example 2:
// Input: vec = [3, 1, 12, 10, 23, 50, 25]
// Output: 4
// Explanation: The element 23 (with the index 4) is the pivot because all elements before 23 are less and all elements after are greater.

// Example 3:
// Input: vec = [-4, 1, 25, 50, 8, 10, 23]
// Output: 0
// Explanation: The element -4 (with the index 0) is the pivot because all elements before -4 are less (the empty sub-vector is assumed to always satisfy) and all elements after are greater.

// Example 4:
// Input: vec = [3, 2, 1]
// Output: None
// Explanation: There is no any pivot element in the vector.

//Notes:
// Data structures: HashMap
// Implementation: In firsr loop add elements to the HashSet. In second check if set contains element + diff if yes, remove from the set and collect the pair

// Complexity:
// Time: O(n)
// Space: O(n)

use std::collections::HashMap;

#[allow(dead_code)]
fn find_pivot_index(numbers: &Vec<i32>) -> Option<usize> {
    let mut map = HashMap::new();
    (0..numbers.len()).rev().fold(i32::MAX, |min, i| {
        map.insert(i, min);

        if numbers[i] < min {
            numbers[i]
        } else {
            min
        }
    });

    let mut max_left = i32::MIN;
    (0..numbers.len()).find_map(|i| {
        let min_right = map[&i];
        let found = if numbers[i] > max_left && numbers[i] < min_right {
            Some(i)
        } else {
            None
        };

        if numbers[i] > max_left {
            max_left = numbers[i];
        }
        found
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers = vec![5, 3, 4, 6, 2, 7, 10, 8];

        assert_eq!(find_pivot_index(&numbers), Some(5));
    }

    #[test]
    fn test_case_2() {
        let numbers = vec![3, 1, 12, 10, 23, 50, 25];

        assert_eq!(find_pivot_index(&numbers), Some(4));
    }

    #[test]
    fn test_case_3() {
        let numbers = vec![-4, 1, 25, 50, 8, 10, 23];

        assert_eq!(find_pivot_index(&numbers), Some(0));
    }

    #[test]
    fn test_case_4() {
        let numbers = vec![3, 2, 1];

        assert_eq!(find_pivot_index(&numbers), None);
    }
}
