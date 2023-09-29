// Problem
// Given a provided ASCII string str, we need to find the first unique (non-repeating) character. 


//Notes:
// use the auxiliary HashMap and to iterate the input string characters twice. 
// When we iterate it for the first time using fold, we count how many times each character appears in the string, saving counts to the map. 
// When we iterate the string for the second time using find, we just return the first character (wrapped in Some) appearing in the string exactly once. 
// The function find is short-circuiting, so it will stop processing as soon as the first character is found, or otherwise, it will return None.

// Complexity:
// Time: O(n)

use std::collections::HashMap;

#[allow(dead_code)]
fn find_unique_character(string: &str) -> Option<char> {
    let mut map = HashMap::new();

    string.chars().for_each(|c| {
       map.entry(c).and_modify(|counter| *counter += 1).or_insert(1u8);
    });

    string.chars().find(|c| map.get(c) == Some(&1))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let string = "yesterday";
        assert_eq!(find_unique_character(&string), Some('s'));
    }

    #[test]
    fn test_case_2() {
        let string = "ifeelfine";
        assert_eq!(find_unique_character(&string), Some('l'));
    }

    #[test]
    fn test_case_3() {
        let string = "ob-la-di,ob-la-da";
        assert_eq!(find_unique_character(&string), Some('i'));
    }

    #[test]
    fn test_case_4() {
        let string = "help!";
        assert_eq!(find_unique_character(&string), Some('h'));
    }
}
