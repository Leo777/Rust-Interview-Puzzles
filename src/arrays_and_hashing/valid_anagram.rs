// Problem
// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

//Notes:

// Complexity:
// Time: O(nlogn) with sorting O(N+M) with hash map
// Space: O(1)

#[allow(dead_code)]
fn valid_anagrams(s1: &str, s2: &str) -> bool{
    assert!(s1.len() == s2.len(), "s1 & s2 len must be equal");

    let mut chars1: Vec<char> = s1.chars().collect();
    let mut chars2: Vec<char> = s2.chars().collect();

    chars1.sort();
    chars2.sort();

    chars1 == chars2
    
}

// #TODO: with hashmap

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let s1 = "cinema";
        let s2 = "iceman";

        assert_eq!(valid_anagrams(s1, s2), true);
    }

    #[test]
    fn test_case_2() {
        let s1 = "anagram";
        let s2 = "nagaram";

        assert_eq!(valid_anagrams(s1, s2), true);
    }

    #[test]
    fn test_case_3() {
        let s1 = "rat";
        let s2 = "car";

        assert_eq!(valid_anagrams(s1, s2), false);
    }
}
