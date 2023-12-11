// Problem
// You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

// Find two lines that together with the x-axis form a container, such that the container contains the most water.

// Return the maximum amount of water a container can store.

// Notice that you may not slant the container.

//Notes:

// Complexity:
// Time: O(n2)
// Space: O(1) or O(n) dependce on sorting implementation.

#[allow(dead_code)]
fn max_area(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let area = (right - left) as i32 * std::cmp::min(nums[left], nums[right]);
        result = std::cmp::max(result, area);
       
        if nums[right] < nums[left] {
            right -= 1;
        } else {
            left += 1;
        }
        
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];

        assert_eq!(max_area(numbers), 49);
    }

    #[test]
    fn test_case_2() {
        let numbers = vec![1, 1];
        assert_eq!(max_area(numbers), 1);
    }
}
