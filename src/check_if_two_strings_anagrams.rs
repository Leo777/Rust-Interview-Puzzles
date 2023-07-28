// Problem
// We need to check whether 2 provided strings s1 and s2 are Anagrams.
// An anagram is a word or phrase formed by rearranging the letters of a different word or phrase.
// For this puzzle we could assume that we work only with ASCII characters, there is no distinction between lower and upper case letters, and that we can ignore spaces in the strings.

// Example 1:

// Input s1: "New York Times"
// Input s2: "monkeys write"
// Output: True

// Example 2:

// Input s1: "McDonald's restaurants"
// Input s2: "Uncle Sam's standard rot"
// Output: True

// Example 3:

// Input s1: "coronavirus"
// Input s2: "carnivorous"
// Output: True

// Example: 4
/// Input s1: "Daddy"
// Input s2: "Mummy"
// Output: False

//Notes:

// Complexity:
// Time: O(n)
// Space: O(n)

use std::collections::HashMap;

#[allow(dead_code)]
fn check_if_anagrams(s1: &str, s2: &str) -> bool {
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();

    s1.chars()
        .filter(|&c| c != ' ')
        .zip(s2.chars().filter(|&c| c != ' '))
        .for_each(|(char1, char2)| {
            let count1 = map1.entry(char1.to_ascii_lowercase()).or_insert(0);
            *count1 += 1;
            let count2 = map2.entry(char2.to_ascii_lowercase()).or_insert(0);
            *count2 += 1;
        });
    map1 == map2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let s1 = "New York Times";
        let s2 = "monkeys write";
        assert_eq!(check_if_anagrams(s1, s2), true);
    }

    #[test]
    fn test_case_2() {
        let s1 = "McDonald's restaurants";
        let s2 = "Uncle Sam's standard rot";
        assert_eq!(check_if_anagrams(s1, s2), true);
    }

    #[test]
    fn test_case_3() {
        let s1 = "Daddy";
        let s2 = "Mummy";
        assert_eq!(check_if_anagrams(s1, s2), false);
    }

    #[test]
    fn test_case_4() {
        let s1 = "coronavirus";
        let s2 = "carnivorous";
        assert_eq!(check_if_anagrams(s1, s2), true);
    }
}
