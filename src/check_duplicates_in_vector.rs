// Problem
// For a provided vector of numbers vec we need to check if it contains any duplicates.


//Notes:
// If we want to solve the puzzle with constant space O(1) (but with O(n*log n) time) without using additional data structures, then we could use Solution 1. 
// The idea here is just to sort the vector vec in-place and then iterate over the vector from the 1st index and check if vec[i] == vec[i - 1]. 
// If that is the case, then we have a duplicate and we return true. 
// We use the function any which is short-circuiting, so it would stop processing and return true as long as the first true is discovered (in other words, when it has found the first duplicate).

// Solution 2 could be used if we want to "exchange space for time" i.e. we could have a linear time complexity O(n) (but with O(n) axillary space). 
// For that, we use an auxiliary HashSet to store numbers we have seen already. 
// If we encounter a number v which is already in the set, then we return true. 
// Otherwise, we insert the number v to the set and return false. 
// The function any would return true as soon as the first true is discovered.

use std::collections::HashSet;


#[allow(dead_code)]
fn contains_duplicates(vec: &mut Vec<i32>) -> bool {
    vec.sort();
    (1..vec.len()).any(|i| if vec[i] == vec[i - 1] {true} else {false})
}

fn contains_duplicates_2(vec: &mut Vec<i32>) -> bool {
    let mut set = HashSet::new();

    vec.iter().any(|e| 
        if set.contains(e) {
            true
        } else {
            set.insert(e);
            false
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut vec = vec![1, 2, 3, 4, 2];
        assert_eq!(contains_duplicates_2(&mut vec), true);
    }

    #[test]
    fn test_case_2() {
        let mut vec = vec![1, 1];
        assert_eq!(contains_duplicates(&mut vec), true);
    }

    #[test]
    fn test_case_3() {
        let mut vec = vec![1, 2, 3, 4];
        assert_eq!(contains_duplicates(&mut vec), false);
    }
}
