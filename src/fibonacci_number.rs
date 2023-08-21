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

#[allow(dead_code)]
fn fibonacci_number(number: i32) -> i32 {
    if number <= 1 {
        return number;
    }

    let mut prev = 0;
    let mut curr = 1;
    let mut temp = 0;

    (2..=number).for_each(|_| {
        temp = curr;
        curr = prev + curr;
        prev = temp;
    });

    println!("{}", curr);
    curr
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let number: i32 = 6;
        assert_eq!(fibonacci_number(number), 8);
    }

    #[test]
    fn test_case_2() {
        let number: i32 = 5;
        assert_eq!(fibonacci_number(number), 5);
    }

    #[test]
    fn test_case_3() {
        let number: i32 = 0;
        assert_eq!(fibonacci_number(number), 0);
    }
}
