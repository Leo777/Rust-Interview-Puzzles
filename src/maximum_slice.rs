// Problem
// In a provided vector vec we need to find a slice with the largest sum of the elements.

// Example 1:
// Input: vec = [-2, 1, -3, 4, -1, 2, 1, -5, 4]
// Output: [4, -1, 2, 1]
// Explanation: The slice [4, -1, 2, 1] has the largest sum of elements (6).

// Example 2:
// Input: vec = [-4, 1, 2, 3, -5]
// Output: [1, 2, 3]
// Explanation: The slice [1, 2, 3] has the largest sum of elements (6).

// Example 3:
// Input: vec = [-4, 1, -5]
// Output: [1]
// Explanation: The slice [1] has the largest sum of elements (1).

// Example 4:
// Input: vec = [-4, -3, -5]
// Output: [-3]
// Explanation: The slice [-3] has the largest sum of elements (-3).

//Notes: The solution using Kadanes algorithm, make 3 vars to sotre max_sum, start_index, end_index and 2 vars to track current_sum and current_start index
// initialize to zero and zero_element for the sum's. Starting from the 1 element check if current sum <= 0 if yes reset it to current element and update current start index.
// if no add element to the current sum. At the end of the loop check when current sum >= max_sum update it and indexes.

// Complexity:
// Time: O(n)
// Space: O(1);

#[allow(dead_code)]
fn find_max_sum_slice(numbers: &Vec<i32>) -> &[i32] {
    if numbers.is_empty() {
        return &[];
    }

    let mut max_sum = numbers[0];
    let mut start_index = 0;
    let mut end_index = 0;

    let mut current_sum = numbers[0];
    let mut current_start_index = 0;

    (1..numbers.len()).for_each(|index| {
        if current_sum <= 0 {
            current_sum = numbers[index];
            current_start_index = index;
        } else {
            current_sum += numbers[index];
        }

        if current_sum >= max_sum {
            max_sum = current_sum;
            start_index = current_start_index;
            end_index = index;
        }
    });

    &numbers[start_index..end_index + 1]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(find_max_sum_slice(&numbers), &[4, -1, 2, 1]);
    }

    #[test]
    fn test_case_2() {
        let numbers = vec![-4, 1, 2, 3, -5];
        assert_eq!(find_max_sum_slice(&numbers), &[1, 2, 3]);
    }

    #[test]
    fn test_case_3() {
        let numbers = vec![-4, 1, -5];
        assert_eq!(find_max_sum_slice(&numbers), &[1]);
    }

    #[test]
    fn test_case_4() {
        let numbers = vec![-4, -3, -5];
        assert_eq!(find_max_sum_slice(&numbers), &[-3]);
    }
}
