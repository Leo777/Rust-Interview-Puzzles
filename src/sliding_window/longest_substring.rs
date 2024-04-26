// Problem
// Given a string s, find the length of the longest
// substring
//  without repeating characters.

use std::{cmp, collections::VecDeque};

#[allow(dead_code)]
fn longest_substring(s: String) -> i32 {
    let mut set: VecDeque<char> = VecDeque::new();
   
    let mut result = 0;

    for c in s.chars() {
        while set.contains(&c) {
            set.pop_front();
        } 
        set.push_back(c);
        result = result.max(set.len());
       
    }
    result as i32 
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "abcabcbb".to_string();
        assert_eq!(longest_substring(s), 3);
    }

    #[test]
    fn test_case_2() {
        let s = "pwwkew".to_string();
        assert_eq!(longest_substring(s), 3);
    }
}
