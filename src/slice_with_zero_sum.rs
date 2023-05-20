// Problem
// We need to check if an unsorted vector of numbers vec contains a contiguous sub-vector (slice) of 0 sum.

// Example 1:

// Input: vec = [4, 2, -3, 1, 6]
// Output: true
// Explanation: Because it contains a contiguous sub-vector [2, -3, 1].
// Example 2:

// Input: vec = [4, 2, 0, 1, 6]
// Output: true
// Explanation: Because it contains [0].
// Example 3:

// Input: vec = [-3, 2, 3, 1, 6]
// Output: false
// Explanation: Though it contains a sub-vector [-3, 3], but it's not contiguous.

//Notes:
// Data structures: HashSet
// Implementation: Loop through the vector and add it to the sum then check if set contains sum return true if not insert to the HashSet
//  If set contains sum it means that after we saw that sum value for the first time, 
//  further elements added to the sum negated that original sum value, so that we encountered the same value again.
//  That is only possible if the vector contains a contiguous sub-vector with 0 sum.

// Complexity:
// Time: O(n)
// Space: O(n)

use std::collections::HashSet;

#[allow(dead_code)]
fn contains_slice_with_zero_sum(numbers: &Vec<i32>) -> bool {
    let mut set = HashSet::from([0]);
    let mut sum = 0;

    for e in numbers {
        sum += e;
        if set.contains(&sum) {
            return true
        } else {
            set.insert(sum);
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers = vec![4, 2, -3, 1, 6];
        assert!(contains_slice_with_zero_sum(&numbers));
    }

    #[test]
    fn test_case_2() {
        let numbers = vec![4, 2, 0, 1, 6];
        assert!(contains_slice_with_zero_sum(&numbers));
    }

    #[test]
    fn test_case_3() {
        let numbers = vec![-3, 2, 3, 1, 6];
        assert!(!contains_slice_with_zero_sum(&numbers));
    }

    #[test]
    fn test_case_4() {
        let numbers = vec![0];
        assert!(contains_slice_with_zero_sum(&numbers));
    }
}
