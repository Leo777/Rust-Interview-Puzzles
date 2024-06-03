// Problem
// You are given a string s and an integer k. You can choose any character of the string and change it to any other uppercase English character. You can perform this operation at most k times.

// Return the length of the longest substring containing the same letter you can get after performing the above operations.

use std::collections::HashMap;

#[allow(dead_code)]
fn character_replacement(s: String, k: i32) -> i32 {
    let mut count = HashMap::new();
    let mut l = 0;
    let mut result = 0;
    let mut maxf = 0;
    let s: Vec<char> = s.chars().collect();

    for r in 0..s.len() {
        *count.entry(s[r]).or_default() += 1;
        maxf = maxf.max(*count.get(&s[r]).unwrap());

        while (r - l + 1) - maxf as usize > k as usize {
            
            *count.get_mut(&s[l]).unwrap() -= 1;
            l += 1 
        }
        result = result.max(r - l + 1); 
    }
    result as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "ABAB".to_string();
        let k = 2;
        assert_eq!(character_replacement(s, k), 4);
    }

    // #[test]
    // fn test_case_2() {
    //     let s = "AABABBA".to_string();
    //     let k = 1;
    //     assert_eq!(character_replacement(s, k), 4);
    // }
}
