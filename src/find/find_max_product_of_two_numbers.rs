// Problem
// We need to find a maximum product of 2 numbers in a provided vector of numbers vec.
// In math, a product of 2 numbers is a result of multiplication of these 2 numbers.

// Example 1:

// Input: vec = [-10, -3, 5, 7, -2]
// Output: 35
// Explanation: A product of (5, 7) is the max product.
// Example 2:

// Input: vec = [-10, -3, 5, 4, -2]
// Output: 30
// Explanation: A product of (-10, -3) is the max product.
// Example 3:

// Input: vec = [-5, 0, 10]
// Output: 0
// Explanation: A product of (-5, 0) or (0, 10) is the max product.

//Notes:
// Implementation: Create four varaibles two for positive multiplicands and two for negative. Loop through the vector and if element is positive compare it positive variables,
// if not compare with with negative. Calculate product for negarive and positive multiplicands. Compare which one is greater, also cover special cases
// - has only a product of positives
// - has only a product of negatives 
// - vector has only two elements one positive and one negative 

// Complexity:
// Time: O(n)


#[allow(dead_code)]
fn max_product_of_to_numbers(numbers: &Vec<i32>) -> i32 {
    let mut max_pos1 = 0;
    let mut max_pos2 = 0;
    let mut min_neg1 = 0;
    let mut min_neg2 = 0;

    for e in numbers {
        if *e >= 0 && *e >= max_pos1 {
            max_pos2 = max_pos1;
            max_pos1 = *e;
        } else if *e >= 0 && *e >= max_pos2 {
            max_pos2 = *e;
        }

        if *e <= 0 && *e <= min_neg1 {
            min_neg2 = min_neg1;
            min_neg1 = *e
        } else if *e <= 0 && *e <= min_neg2 {
            min_neg2 = *e;
        }
    }

    let pos_numbers_product = max_pos1 * max_pos2;
    let negative_numbers_product = min_neg1 * min_neg2;

    match (pos_numbers_product, negative_numbers_product) {
        (pos, neg) if pos != 0 && neg != 0 && pos >= neg => pos, //product of positives is larger
        (pos, neg) if pos != 0 && neg != 0 && pos <= neg => neg, //product of negatives is larger
        (pos, 0) if pos != 0 => pos, //has only a product of positives
        (0, neg) if neg != 0 => neg, //has only a product of negatives
        _ if max_pos1 != 0 && min_neg1 != 0 && numbers.len() == 2 => max_pos1 * min_neg1, // has 1 pos & 1 neg number
        _ => 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers = vec![-10, -3, 5, 7, -2];
        assert_eq!(max_product_of_to_numbers(&numbers), 35);
    }

    #[test]
    fn test_case_2() {
        let numbers = vec![-10, -3, 5, 4, -2];
        assert_eq!(max_product_of_to_numbers(&numbers), 30);
    }

    #[test]
    fn test_case_3() {
        let numbers = vec![-5, 0, 10];
        assert_eq!(max_product_of_to_numbers(&numbers), 0);
    }

    #[test]
    fn test_case_4() {
        let numbers = vec![-3, 5];
        assert_eq!(max_product_of_to_numbers(&numbers), -15);
    }
}
