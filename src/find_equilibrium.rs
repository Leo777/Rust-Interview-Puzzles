// Problem
// We need to find all equilibrium indices in the provided vector vec.
//  An equilibrium index in the vector vec is an index i where the sum of vec[0..i-1] elements is equal to the sum of vec[i+1..len] elements.

// Example 1:

// Input: vec = [0, -3, 5, -4, -2, 3, 1, 0]
// Output: [0, 3, 7]
// Explanation: 0 index is in the result because the sum of the elements left to the index 0 is considered to be 0, and the sum of the right elements is 0 as well (-3 + 5 + (-4) + (-2) + 3 + 1 + 0 = 0). 3 index is in the result because 0 + (-3) + 5 = -2 + 3 + 1 + 0. 7 index is in the result because 0 + (-3) + 5 + (-4) + (-2) + 3 + 1 + 0 = 0.
// Example 2:

// Input: vec = [2, 3, 5, 1, 2, 2]
// Output: [2]
// Explanation: 2 index is in the result because 2 + 3 = 1 + 2 + 2.
// Example 3:

// Input: vec = [1, 3, 5]
// Output: []
// Explanation: The result is empty because there are no equilibrium indices in the vector.

//Notes:
// On the first iteration over the reversed vec iterator, we accumulate and save to the created map sums of the right elements for the each index value.
// On the second iteration, we accumulate sums of the left elements for the each index value.
// At the same time we check if for the current index our accumulated left_sum is equal to the right sum extracted from the map for this index.
// If that is the case, then we have found_equilibrium and the filter's predicate would satisfy for that index (which means that index would be included in the result).

// Complexity:
// Time: O(n)

use std::collections::HashMap;

#[allow(dead_code)]
fn find_equilibrium(numbers: &Vec<i32>) -> Vec<usize> {
    let mut map = HashMap::new();

    let mut right_sum = 0;
    (0..numbers.len()).rev().for_each(|index| {
        map.insert(index, right_sum);
        right_sum += numbers[index];
    });

    let mut left_sum = 0;

    numbers
        .iter()
        .enumerate()
        .filter(|(index, elem)| {
            let equilibrium_index = left_sum == map[index];
            left_sum += *elem;
            equilibrium_index
        })
        .map(|(index, _)| index)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers: Vec<i32> = vec![0, -3, 5, -4, -2, 3, 1, 0];
        assert_eq!(find_equilibrium(&numbers), vec![0, 3, 7]);
    }

    #[test]
    fn test_case_2() {
        let numbers: Vec<i32> = vec![2, 3, 5, 1, 2, 2];
        assert_eq!(find_equilibrium(&numbers), vec![2]);
    }

    #[test]
    fn test_case_3() {
        let numbers: Vec<i32> = vec![1, 3, 5];
        assert_eq!(find_equilibrium(&numbers), vec![]);
    }
}
