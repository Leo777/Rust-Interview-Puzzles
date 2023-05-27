// Problem
// In a provided vector vec we need to move all 0 elements to the end of the vector. 
// A relative order of other elements should not change.

// Example 1:

// Input: vec = [5, 0, 0, 2, 3, 0, 4, 0, 1]
// Output: [5, 2, 3, 4, 1, 0, 0, 0, 0]
// Example 2:

// Input: vec = [0, 0, 8, 6, 0, 0]
// Output: [8, 6, 0, 0, 0, 0]
// Example 3:

// Input: vec = [1, 2, 3]
// Output: [1, 2, 3]

//Notes:
// As we go through the vector vec, we maintain zero_count of 0 elements we come across. 
// At the same time, we shift non-zero elements to the left to index - zero_count position, and fill their place with 0.

// Complexity:
// Time: O(n)

#[allow(dead_code)]
fn move_all_zeroes_to_the_end(numbers: &mut Vec<i32>) -> &Vec<i32> {
    let mut zeroes_count = 0;

    (0..numbers.len()).for_each(|index| {
        if numbers[index] == 0 {
            zeroes_count += 1;
        } else if zeroes_count > 0 {
            numbers[index - zeroes_count] = numbers[index];
            numbers[index] = 0;
        }
    });
    return numbers;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut numbers: Vec<i32> = vec![5, 0, 0, 2, 3, 0, 4, 0, 1];
        assert_eq!(
            move_all_zeroes_to_the_end(&mut numbers),
            &vec![5, 2, 3, 4, 1, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_case_2() {
        let mut numbers: Vec<i32> = vec![0, 0, 8, 6, 0, 0];
        assert_eq!(
            move_all_zeroes_to_the_end(&mut numbers),
            &vec![8, 6, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_case_3() {
        let mut numbers: Vec<i32> = vec![1, 2, 3];
        assert_eq!(move_all_zeroes_to_the_end(&mut numbers), &vec![1, 2, 3]);
    }
}
