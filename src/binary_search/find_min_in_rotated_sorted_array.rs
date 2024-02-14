// Problem
// Suppose an array of length n sorted in ascending order is rotated between 1 and n times. For example, the array nums = [0,1,2,4,5,6,7] might become:

// [4,5,6,7,0,1,2] if it was rotated 4 times.
// [0,1,2,4,5,6,7] if it was rotated 7 times.
// Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time results in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].

// Given the sorted rotated array nums of unique elements, return the minimum element of this array.

// You must write an algorithm that runs in O(log n) time.

// Complexity:
// Time: O(log n)

use std::cmp::min;

#[allow(dead_code)]

pub fn find_min(nums: Vec<i32>) -> i32 {
  let mut res = nums[0];
  let (mut l, mut r) = (0, nums.len() - 1);

    while l <= r {
        if nums[l] < nums[r] {
            res = min(res, nums[l]);
            break;
        }

        let middle = l + (r - l) / 2;
        res = min(res, nums[middle]);
        if nums[middle] >= nums[l] {
            l = middle + 1;
        } else {
            r = middle - 1;
        }
    }

  res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums: Vec<i32> = vec![3,4,5,1,2];
        assert_eq!(find_min(nums), 1);
    }
}
