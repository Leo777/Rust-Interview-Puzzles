// Problem
// In an unsorted vector vec of numbers we need to find any pair (Tuple) with a provided sum.

// Example 1:
// Input: vec = [8, 7, 2, 5, 3, 1], sum = 10
// Output: (2, 8)

// Example 2:
// Input: vec = [5, 2, 6, 8, 6, 9], sum = 12
// Output: (6, 6)

// Example 3:
// Input: vec = [5, 2, 6, 8, 1, 9], sum = 12
// Output: None

use std::collections::HashSet;

#[allow(dead_code)]
fn find_pair_sum(numbers: &Vec<i32>, sum: i32) -> Option<(i32,i32)> {
    let mut set = HashSet::new();

    for e in numbers {
        let another = sum - e;

        match set.get(&another) {
            Some( value) => return Some((*e, *value)),
            _ => { set.insert(*e); }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn find_a_pair() {
        let numbers = vec![8, 7, 2, 5, 3, 1];
        let sum = 10;
        assert_eq!(find_pair_sum(&numbers, sum), Some((2,8)));
        println!("{:?}", numbers);
    }
}
