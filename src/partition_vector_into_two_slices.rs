// Problem
// We need to partition a provided vector vec into 2 slices, each having the same sum of the elements.
// Example 1:

// Input: vec = [7, -5, -4, 2, 4]
// Output: ([7, -5], [-4, 2, 4])
// Explanation: Each of the 2 partitions has the equal sum of the elements 2.
// Example 2:

// Input: vec = [7, -6, 3, -5, 1]
// Output: ([], [7, -6, 3, -5, 1])
// Explanation: Each of the 2 partitions has the equal sum of the elements 0 (the empty partition is assumed to have 0 sum).
// Example 3:

// Input: vec = [1, 3, 5, 7]
// Output: None
// Explanation: This vector could not be partitioned (there are no slices with an equal sum).

//Notes:
// Calculate sum off all elements, than in second run calculate left sum and right sum (sum - left sum) if equal return result. 

// Complexity:
// Time: O(n)

#[allow(dead_code)]
fn partition_vector_2_slices(numbers: &mut Vec<i32>) -> Option<(&[i32], &[i32])> {
    let sum = numbers.iter().fold(0, |acc, e| acc + e);
    let mut left_sum = 0;
    let start_index = 0;
    let end_index = numbers.len();

    for (index, element) in numbers.iter().enumerate() {
        let right_sum = sum - left_sum;

        if left_sum == right_sum {
            return Some((&numbers[start_index..index], &numbers[index..end_index]));
        }
        left_sum += element;
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut numbers: Vec<i32> = vec![7, -5, -4, 2, 4];
        assert_eq!(
            partition_vector_2_slices(&mut numbers),
            Some((vec![7, -5].as_slice(), vec![-4, 2, 4].as_slice()))
        );
    }

    #[test]
    fn test_case_2() {
        let mut numbers: Vec<i32> = vec![7, -6, 3, -5, 1];
        assert_eq!(
            partition_vector_2_slices(&mut numbers),
            Some((vec![].as_slice(), vec![7, -6, 3, -5, 1].as_slice()))
        );
    }

    #[test]
    fn test_case_3() {
        let mut numbers: Vec<i32> = vec![1, 3, 5, 7];
        assert_eq!(partition_vector_2_slices(&mut numbers), None) 
    }
}
