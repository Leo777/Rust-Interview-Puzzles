// Problem
// Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.

// You must write an algorithm that runs in O(n) time.

//Notes:
// Add numbers to a set to identify if number has left neighbour.
// If it doesn't it means it is a start of the sequence, count all numbers after it.

// Complexity:
// Time: O(n)
// Space: O(n)

use std::collections::HashSet;

#[allow(dead_code)]
fn find_longest_consecutive_sequence(nums: Vec<i32>) -> i32 {
    let mut set : HashSet<i32> = HashSet::from_iter(nums.into_iter());
        
        let mut max_cnt = 0;
        
        for n in &set{
            if !set.contains(&(n-1)){
                let mut next = n + 1;
                let mut cnt = 1;
                while set.contains(&next){
                    cnt +=1;
                    next+=1;
                }
                
                max_cnt = max_cnt.max(cnt);
            }
        }

        max_cnt
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
       let nums = vec![100,4,200,1,3,2]; 

        assert_eq!(find_longest_consecutive_sequence(nums), 4);
    }

    // #[test]
    // fn test_case_2() {
    //     let s1 = "anagram";
    //     let s2 = "nagaram";

    //     assert_eq!(valid_anagrams(s1, s2), true);
    //     assert_eq!(valid_anagrams_2(s1, s2), true);
    // }

    // #[test]
    // fn test_case_3() {
    //     let s1 = "rat";
    //     let s2 = "car";

    //     assert_eq!(valid_anagrams(s1, s2), false);
    //     assert_eq!(valid_anagrams_2(s1, s2), false);
    // }

    fn compare_vecs(mut a: Vec<Vec<String>>, mut b: Vec<Vec<String>>) -> bool {
        a.iter_mut().for_each(|inner| inner.sort());
        b.iter_mut().for_each(|inner| inner.sort());

        a.sort();
        b.sort();

        a == b
    }
}
