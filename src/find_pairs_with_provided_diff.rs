// Problem
//In a vector vec we need to find all pairs of the elements with a provided difference diff.

// Example 1:
// Input: vec = [1, 5, 2, 2, 2, 5, 5, 4], diff = 3
// Output: [(1, 4), (2, 5)]
// Explanation: (1, 4) and (2, 5) pairs have the difference of the elements 3.

// Example 2:
// Input: vec = [1, 5, 6], diff = 2
// Output: []
// Explanation: There are no pairs with the difference of the elements 2.

//Notes:
// Data structures: HashSet
// Implementation: In firsr loop add elements to the HashSet. In second check if set contains element + diff if yes, remove from the set and collect the pair

// Complexity:
// Time: O(n)
// Space: O(n)

use std::collections::HashSet;

#[allow(dead_code)]
fn find_pairs_with_provided_difference(numbers: &Vec<i32>, diff: i32) -> Vec<(i32, i32)> {
    let mut set = HashSet::new();

    for e in numbers {
        numbers.iter().for_each(|&e| {
            set.insert(e);
        });
    }
    numbers
        .iter()
        .flat_map(|&e| {
            let another = e + diff;
            if set.contains(&another) {
                set.remove(&another);
                Some((e, another))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers = vec![1, 5, 2, 2, 2, 5, 5, 4];
        let diff = 3;
        assert_eq!(
            find_pairs_with_provided_difference(&numbers, diff),
            vec![(1, 4), (2, 5)]
        );
    }

    #[test]
    fn test_case_2() {
        let numbers = vec![1, 5, 6];
        let diff = 2;
        assert_eq!(find_pairs_with_provided_difference(&numbers, diff), vec![]);
    }
}
