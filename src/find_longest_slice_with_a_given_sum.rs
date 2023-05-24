// Problem
// We need to check if an unsorted vector of numbers vec contains a contiguous sub-vector (slice) of 0 sum.

// Example 1:

// Input: vec = [5, 6, -5, 5, 3, 4, 1], sum = 7
// Output: [-5, 5, 3, 4]
// Explanation: This is the longest slice (length = 4) that has a sum of the elements equals to 7.
// Example 2:

// Input: vec = [5, 6, -5, 5, 3, 4, 1], sum = 5
// Output: [4, 1]
// Explanation: This is the longest slice (length = 2) that has a sum of the elements equals to 5.
// Example 3:

// Input: vec = [1, 2, 3], sum = 10
// Output: None
// Explanation: There are no slices having a sum of the elements equals to 10.

//Notes:
// Data structures: HashMap
// Implementation: Create HashMap with 0,0 KV and four variables for tracking  
// max_length,
// max_length_start_index,
// max_length_end_index,
// current_sum

// Iterate through the elements of the numbers array and accumulate each element to the current_sum. 
// Next, check if the difference between the current_sum and the sum is already stored as a key in the map.
//  If it exists, update the max_length and the indices (max_length_start_index and max_length_end_index) based on the current indices. 
//  If the current_sum is unique, add it to the map as a key with the value of index + 1. 
//  Finally, compare the max_length_start_index and max_length_end_index and 
//  return a slice from the numbers array using these indices to represent the resulting longest contiguous subarray with the given sum.

// Complexity:
// Time: O(n)
// Space: O(n)

use std::collections::HashMap;

#[allow(dead_code)]
fn longest_slice_with_given_sum(numbers: &[i32], sum: i32) -> Option<&[i32]> {
    let mut map = HashMap::from([(0, 0)]);
    let mut max_length = 0;
    let mut max_length_start_index: Option<usize> = None;
    let mut max_length_end_index: Option<usize> = None;
    let mut current_sum = 0;

    for (index, element) in numbers.iter().enumerate() {
        current_sum += element;

        if let Some(start_position) = map.get(&(current_sum - sum)) {
            let length = index - start_position + 1;
            if length > max_length {
                max_length = length;
                max_length_start_index = Some(*start_position);
                max_length_end_index = Some(index);
            }
        }

        map.entry(current_sum).or_insert(index + 1);    
    }


    match (max_length_start_index, max_length_end_index) {
        (Some(start), Some(end)) => Some(&numbers[start..end + 1]),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers = vec![5, 6, -5, 5, 3, 4, 1];
        let sum = 7;
        assert_eq!(
            longest_slice_with_given_sum(&numbers, sum),
            Some(vec![-5, 5, 3, 4].as_slice())
        );
    }

    #[test]
    fn test_case_2() {
        let numbers = vec![5, 6, -5, 5, 3, 4, 1];
        let sum = 5;
        assert_eq!(
            longest_slice_with_given_sum(&numbers, sum),
            Some(vec![4, 1].as_slice())
        );
    }

    #[test]
    fn test_case_3() {
        let numbers = vec![1, 2, 3];
        let sum = 10;
        assert_eq!(longest_slice_with_given_sum(&numbers, sum), None);
    }
}
