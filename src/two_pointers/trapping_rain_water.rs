// Problem
// Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.

//Notes:

// Complexity:
// Time: O(n)
// Space: O(1)

#[allow(dead_code)]
fn trap(height: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut max_left = height[left];
    let mut max_right = height[right];

    while left < right {
        if max_left < max_right {
            left += 1;
            max_left = std::cmp::max(max_left, height[left]);
            result += max_left - height[left];
        } else {
            right -= 1;
            max_right = std::cmp::max(max_right, height[right]);
            result += max_right - height[right];
        }
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers = vec![0,1,0,2,1,0,1,3,2,1,2,1];

        assert_eq!(trap(numbers), 6);
    }

    #[test]
    fn test_case_2() {
        let numbers = vec![4,2,0,3,2,5];
        assert_eq!(trap(numbers), 9);
    }
}
