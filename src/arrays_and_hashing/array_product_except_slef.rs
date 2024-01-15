// Problem
// Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
// You must write an algorithm that runs in O(n) time and without using the division operation.

//Notes:

// Complexity:
// Time: O(n)
// Space: O(n)

#[allow(dead_code)]
fn array_product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![1; nums.len()];

    for i in 1..nums.len() {
        result[i] = nums[i - 1] * result[i-1];

    }

    let mut right = 1;

    for (i, n) in result.iter_mut().enumerate().rev() {
        *n = *n * right;
        right *= nums[i]; 
    }

    result
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let num: Vec<i32> = vec![1, 2, 3];

        let result = array_product_except_self(num);

        assert_eq!(result, vec![6, 3, 2]);
    }

    #[test]
    fn test_case_2() {}

    #[test]
    fn test_case_3() {}
}
