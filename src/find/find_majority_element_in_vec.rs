// Problem
// In a provided vector vec we need to find the majority element, if it is present.
// The majority element is the element that appears in the vector more than n / 2 times, where n is a size of the vector vec.

// Example 1:
// Input: vec = [4, 8, 7, 4, 4, 5, 4, 3, 1, 4, 4]
// Output: 4
// Explanation: The majority element is 4 because it appears in the vector 6 times (more than n / 2 = 11 / 2 = 5).

// Example 2:
// Input: vec = [4, 8, 7, 4, 4, 5, 4, 3, 1, 4]
// Output: None
// Explanation: There is no the majority element in the vector. Even though 4 is the most common element, it is not the majority element, because it appears only 5 times (not more than n / 2 = 10 / 2 = 5 times).

// Example 3:
// Input: vec = [1]
// Output: 1

// Example 4:
// Input: vec = [1, 2]
// Output: None

//Notes:

// Implementation: The solution for this problem uses Boyer–Moore majority vote algorithm. 
// Also we need the second pass over the vector vec because, even when there is no the majority element, 
// the algorithm would yield one of the elements as the result (m).
// On the second pass, we calculate a number of times (count) we encounter m element to check if that is the actual majority (it appears more than vec.len() / 2 times). 
// If that is the case - we found the majority element. If not - we return None.

// Complexity:
// Time: O(n)
// Space: O(1)

#[allow(dead_code)]
fn find_majority_element(numbers: &Vec<i32>) -> Option<i32> {
    let mut major_element = 0;
    let mut count = 0;

    numbers.iter().for_each(|element| {
        if count == 0 {
            major_element = *element;
            count += 1;
        } else if major_element == *element {
            count += 1;
        } else {
            count -= 1;
        }
    });

    let count_2 = numbers.iter().fold(0, |acc, e | { if *e == major_element { acc +1 } else { acc }});

    (count_2 > numbers.len() / 2).then(|| major_element)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers = vec![4, 8, 7, 4, 4, 5, 4, 3, 1, 4, 4];
        assert_eq!(find_majority_element(&numbers), Some(4));
    }

    #[test]
    fn test_case_2() {
        let numbers = vec![4, 8, 7, 4, 4, 5, 4, 3, 1, 4];
        assert_eq!(find_majority_element(&numbers), None);
    }

    #[test]
    fn test_case_3() {
        let numbers = vec![1];
        assert_eq!(find_majority_element(&numbers), Some(1));
    }

    #[test]
    fn test_case_4() {
        let numbers = vec![1,2];
        assert_eq!(find_majority_element(&numbers), None);
    }
}
