// Problem
// Given an integer array nums and an integer k, return the k most frequent elements.
//  You may return the answer in any order.


//Notes: Use bucket sort. Element count will be index and 

// Complexity:
// Time: O(n)
// Space: O(n)

#[allow(dead_code)]
pub fn top_k_frequent_elements(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();

    for &e in nums.iter() {
        *map.entry(e).or_insert(0) += 1;
    }
    let mut sorted: Vec<Vec<i32>> = vec![Vec::new(); nums.len()];
    for (&e, &count) in map.iter() {
        sorted[nums.len() - count].push(e);
    }
    print!("{:?}", sorted);
    sorted.into_iter().flatten().take(k as usize).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1,1,1,2,2,3];
        let k = 2;
        assert_eq!(two_sum(nums, k), vec![1, 2]);
    }
}
