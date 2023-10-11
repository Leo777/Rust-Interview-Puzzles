// Problem
//Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.

//Notes:

// Complexity:
// Time: O(gcd(a, b))
// Space: O(1)

#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
   
    for (i, e) in nums.iter().enumerate(){
       
       match map.get(&(target - *e)) {
           Some(&j) => return vec![j as i32, i as i32],
           None => map.insert(*e, i as i32),
       };
    }
    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![2,7,11,15];
        let target = 9;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![3,2,4];
        let target = 6;
        assert_eq!(two_sum(nums, target), vec![1, 2]);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![3,3];
        let target = 6;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }
}
