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
// Implementation: To solve the puzzle in linear time we can use an auxiliary data structure: HashSet. 
// We go backwards over the reversed iterator of the vector indices i. 
// When we encounter an element vec[i] that is already contained in the set (which means the element is repeating), 
// we wrap the index i of that element in Some, otherwise we return None for that iteration. 
// Also, on each iteration we insert the current element vec[i] to the set. 
// And since we use flat_map adapter, it would automatically unwrap found indices from Some and would discard None results. 
// All transformations up to last are lazy and would not trigger any traversal. 
// But the last call would trigger the resulting iterator and would traverse it until the index of the last repeating element from the resulting sequence is obtained. 
// That would be the lowest index of the repeating element wrapped in Some, or None if there are no repeating elements. 

// Complexity:
// Time: O(n)
// Space: O(n)

use std::collections::HashMap;
use std::collections::HashSet;

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

#[allow(dead_code)]
fn find_min_index_of_repeating_element_in_vector(vec: &Vec<i32>) -> Option<usize> {
    let mut set = HashSet::new();

    (0..vec.len())
        .rev()
        .flat_map(|i| {
            let found = if set.contains(&vec[i]) { Some(i) } else { None };
            set.insert(vec[i]);
            found
        })
        .last()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers = vec![6, 7, 4, 5, 4, 7, 5];

        assert_eq!(find_lowest_idx(&numbers), Some(1));
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

#[cfg(test)]
mod test2 {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers = vec![6, 7, 4, 5, 4, 7, 5];

        assert_eq!(
            find_min_index_of_repeating_element_in_vector(&numbers),
            Some(1)
        );
    }

    #[test]
    fn test_case_2() {
        let numbers = vec![1, 2, 5, 3, 4, 7, 3, 5, 8, 9];
        assert_eq!(
            find_min_index_of_repeating_element_in_vector(&numbers),
            Some(2)
        );
    }

    #[test]
    fn test_case_3() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(
            find_min_index_of_repeating_element_in_vector(&numbers),
            None
        );
    }
}
