// Problem
// In a provided vector vec we need to find the minimum index of a repeating element.

// Example 1:
// Input: vec = [6, 7, 4, 5, 4, 7, 5]
// Output: 1
// Explanation: There are several repeating elements in the vector but the element 7 has the lowest index 1.

// Example 2:
// Input: vec = [1, 2, 5, 3, 4, 7, 3, 5, 8, 9]
// Output: 2
// Explanation: There are several repeating elements in the vector but the element 5 has the lowest index 2.

// Example 3:
// Input: vec = [1, 2, 3, 4, 5, 6]
// Output: None
// Explanation: There are no repeating elements in the vector.

//Notes:
// Data structures: HashSet
// Implementation: In first pass add elements to the HashSet. In second check if set contains element + diff if yes, remove from the set and collect the pair

// Complexity:
// Time: O(n)
// Space: O(n)

use std::collections::HashMap;

#[allow(dead_code)]
fn find_lowest_idx(numbers: &Vec<i32>) -> Option<usize> {
    let mut map = HashMap::new();
    let mut min_index = None;
  

    for (index, element) in numbers.iter().rev().enumerate() {
        if let Some(&prev_index) = map.get(element) {
            min_index = Some(prev_index);
        }
        map.insert(element, index);
    }

    match min_index {
        Some(e) => Some(e),
        None => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers = vec![6, 7, 4, 5, 4, 7, 5];
     
        assert_eq!(
            find_lowest_idx(&numbers),
           Some(1)
        );
    }

    #[test]
    fn test_case_2() {
        let numbers = vec![1, 2, 5, 3, 4, 7, 3, 5, 8, 9];
        assert_eq!(find_lowest_idx(&numbers), Some(2));
    }

    #[test]
    fn test_case_3() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(find_lowest_idx(&numbers), None);
    }
}
