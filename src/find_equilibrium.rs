// Problem
// We need to sort a provided binary vector vec (a vector that contains only 0 or 1 numbers) in linear time.

// Example 1:

// Input: vec = [1, 0, 1, 0, 1, 0, 0, 1]
// Output: [0, 0, 0, 0, 1, 1, 1, 1]

// Example 2:

// Input: vec = [0, 0, 1, 0, 1, 1, 0, 1, 0, 0]
// Output: [0, 0, 0, 0, 0, 0, 1, 1, 1, 1]

// Example 3:

// Input: vec = [0, 0, 1, 1]
// Output: [0, 0, 1, 1]

//Notes:
// Count all zeroes in the first loop, then in the second loop insert zeroes from the beginning than ones at the end.

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

    numbers.iter().enumerate().filter(|(index, elem)| {
        let equilibrium_index = left_sum == map[index];
        left_sum += *elem;
        equilibrium_index
    }).map(|(index, _)| index).collect()
    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut numbers: Vec<i32> = vec![0, -3, 5, -4, -2, 3, 1, 0];
        assert_eq!(
            find_equilibrium(&mut numbers),
            vec![0, 3, 7]
        );
    }

    // #[test]
    // fn test_case_2() {
    //     let mut numbers: Vec<u8> = vec![0, -3, 5, -4, -2, 3, 1, 0];
    //     assert_eq!(
    //         sort_binary_array(&mut numbers),
    //         vec![0, 3, 7]
    //     );
    // }

    // #[test]
    // fn test_case_3() {
    //     let mut numbers: Vec<u8> = vec![0, 0, 1, 1];
    //     assert_eq!(
    //         sort_binary_array(&mut numbers),
    //         &vec![0, 0, 1, 1]
    //     );
    // }
}
