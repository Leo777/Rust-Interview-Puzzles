// Problem
//Given a vector vec of n natural numbers (each number is in the range [1..n]), we need to return "missing" natural numbers that do not appear in vec.
// This puzzle is similar to Find the missing natural number in a vector puzzle where we had to find just 1 missing number (looking ahead, that would lead to a completely different implementation).

//Notes:
// We can solve the puzzle without using extra space using the negation trick.
//  We know that the input vector vec could contain only numbers in the range [1..n] (where n is the length of the input vector).
// It means that we can iterate over the vector and for each element num = vec[i] find a corresponding index corr_index = num.abs() - 1 and negate the number at that index. By doing so, we effectively mark an element as present by negating another element at the corresponding index.
// After this exercise, all positive numbers (not marked by the negation) would represent the missing numbers: for a positive number num at the index i, the corresponding "missing" number would be equal to i + 1.

// Complexity:
// Time: O(n)

#[allow(dead_code)]
fn find_missing_numbers_in_vector(vec: &mut Vec<i32>) -> Vec<i32> {
    for i in 0..vec.len() {
        let num = vec[i];
        let corr_index = (num.abs() - 1) as usize;
        vec[corr_index] = -1 * vec[corr_index].abs();
    }

    vec.iter()
        .enumerate()
        .filter_map(|(i, &num)| if num > 0 { Some((i + 1) as i32) } else { None })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut vec: Vec<i32> = vec![1, 2, 3, 3, 5, 6, 7];
        assert_eq!(find_missing_numbers_in_vector(&mut vec), vec![4]);
    }

    #[test]
    fn test_case_2() {
        let mut vec: Vec<i32> = vec![6, 3, 3, 2, 1, 1];
        assert_eq!(find_missing_numbers_in_vector(&mut vec), vec![4, 5]);
    }

    #[test]
    fn test_case_3() {
        let mut vec: Vec<i32> = vec![1, 1];
        assert_eq!(find_missing_numbers_in_vector(&mut vec), vec![2]);
    }

    #[test]
    fn test_case_4() {
        let mut vec: Vec<i32> = vec![1, 2, 3, 4, 5];
        assert_eq!(find_missing_numbers_in_vector(&mut vec), vec![]);
    }

    #[test]
    fn test_case_5() {
        let mut vec: Vec<i32> = vec![1];
        assert_eq!(find_missing_numbers_in_vector(&mut vec), vec![]);
    }
}
