// Problem
//In a provided vector vec of distinct natural numbers [1, 2, 3...] we need to find one missing number.


// Example 1:

// Input: vec = [1, 2, 3, 5, 6, 7]
// Output: 4

// Example 2:

// Input: vec = [2, 3]
// Output: 1

// Example 3:

// Input: vec = [1, 3]
// Output: 2

//Notes:
// Use Arithmetic progression formula n = n * (n + 1) / 2

// Complexity:
// Time: O(n)


#[allow(dead_code)]
fn find_missing_number_in_vector(numbers: &mut Vec<i32>) -> i32 {
    let sum_vec: i32 = numbers.iter().sum();

    let n = (numbers.len() + 1) as i32;
    let sum_nat = n * (n + 1) / 2;

    sum_nat - sum_vec

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut numbers: Vec<i32> = vec![1, 2, 3, 5, 6, 7];
        assert_eq!(
            find_missing_number_in_vector(&mut numbers),
           4 
        );
    }

    #[test]
    fn test_case_2() {
        let mut numbers: Vec<i32> = vec![1, 3];
        assert_eq!(
            find_missing_number_in_vector(&mut numbers),
            2
        );
    }

    #[test]
    fn test_case_3() {
        let mut numbers: Vec<i32> = vec![2, 3];
        assert_eq!(
            find_missing_number_in_vector(&mut numbers),
            1
        );
    }
}
