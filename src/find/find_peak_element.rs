// Problem
//For a provided vector of numbers vec we need to find the peak element.
//  The peak element is an element that is greater or equal to its neighbors.
//  If there are multiple peak elements in the vector, we can return any of them as the solution. 



//Notes:
// Use binary search 

// Complexity:
// Time: O(logn)
// Space: O(1)


#[allow(dead_code)]
fn find_peak_element(numbers: &Vec<i32>) -> i32{
  fn find_peak(vec: &Vec<i32>, left: usize, right: usize) -> i32 {
        match (left + right) / 2 {
            m if (m == 0 || vec[m] >= vec[m - 1]) && (m == vec.len() - 1 || vec[m] >= vec[m + 1]) => vec[m],
            m if m != 0 && vec[m - 1] > vec[m] => find_peak(vec, 0, m - 1),
            m => find_peak(vec, m + 1, vec.len() -1)
            
        }
  } 
   find_peak(numbers, 0, numbers.len() - 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers = vec![7, 8, 9, 1, 4, 5];
        assert_eq!(find_peak_element(&numbers),9);
    }

    #[test]
    fn test_case_2() {
        let numbers = vec![7, 8, 9, 11, 14];
        assert_eq!(find_peak_element(&numbers), 14);
     }

    #[test]
    fn test_case_3() {
        let numbers = vec![9, 7, 5, 4, 2, 1];
        assert_eq!(find_peak_element(&numbers), 9);
    }

    #[test]
    fn test_case_4() {
        let numbers = vec![1, 2];
        assert_eq!(find_peak_element(&numbers), 2);
    }

    #[test]
    fn test_case_5() {
        let numbers = vec![2, 1];
        assert_eq!(find_peak_element(&numbers), 2);
    }
}
