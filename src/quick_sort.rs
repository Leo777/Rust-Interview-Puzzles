// Problem
// We need to find a maximum length slice (contiguous sub-vector) with a provided sum of the elements in a given vector vec.

// Example 1:

// Input: vec = [5, 6, -5, 5, 3, 4, 1], sum = 7
// Output: [-5, 5, 3, 4]
// Explanation: This is the longest slice (length = 4) that has a sum of the elements equals to 7.
// Example 2:

// Input: vec = [5, 6, -5, 5, 3, 4, 1], sum = 5
// Output: [4, 1]
// Explanation: This is the longest slice (length = 2) that has a sum of the elements equals to 5.
// Example 3:

// Input: vec = [1, 2, 3], sum = 10
// Output: None
// Explanation: There are no slices having a sum of the elements equals to 10.

//Notes:
// Data structures: HashMap
// Implementation: Create HashMap with 0,0 KV and four variables for tracking
// max_length,
// max_length_start_index,
// max_length_end_index,
// current_sum

// Iterate through the elements of the numbers array and accumulate each element to the current_sum.
// Next, check if the difference between the current_sum and the sum is already stored as a key in the map.
//  If it exists, update the max_length and the indices (max_length_start_index and max_length_end_index) based on the current indices.
//  If the current_sum is unique, add it to the map as a key with the value of index + 1.
//  Finally, compare the max_length_start_index and max_length_end_index and
//  return a slice from the numbers array using these indices to represent the resulting longest contiguous subarray with the given sum.

// Complexity:
// Time: O(n)
// Space: O(n)

#[allow(dead_code)]
fn quick_sort(numbers: &mut Vec<i32>) -> &Vec<i32> {
    fn partition(numbers: &mut [i32]) -> usize {
        let pivot_idx = numbers.len() - 1;
        let pivot = numbers[pivot_idx];
        let mut next_idx = 0;

        (0..numbers.len()).for_each(|idx| {
            if numbers[idx] < pivot {
                numbers.swap(next_idx, idx);
                next_idx += 1;
            }
        });
        numbers.swap(pivot_idx, next_idx);

        next_idx
    }

    fn qsort(numbers: &mut [i32]) {
        let pivot_index = partition(numbers);
        let len = numbers.len();

        if pivot_index > 0 {
            qsort(&mut numbers[0..pivot_index])
        }

        if pivot_index < len - 1 {
            qsort(&mut numbers[pivot_index + 1..len])
        }
    }

    if !numbers.is_empty() {
        qsort(&mut numbers[..]);
    }

    numbers
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut numbers = vec![5, 6, 4, 2, 3, 2, 1];

        assert_eq!(quick_sort(&mut numbers), &vec![1, 2, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_case_2() {
        let mut numbers = vec![-4, 1, 25, 50, 8, 10, 23];
        assert_eq!(quick_sort(&mut numbers), &vec![-4, 1, 8, 10, 23, 25, 50]
    );
    }

    // #[test]
    // fn test_case_3() {
    //     let numbers = vec![1, 2, 3];
    //     let sum = 10;
    //     assert_eq!(longest_slice_with_given_sum(&numbers, sum), None);
    // }
}
