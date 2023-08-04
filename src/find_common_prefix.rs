// Problem
// In a provided vector of strings vec we need to find the longest string prefix shared by all strings in the vector.

// Example 1:

// Input: ["abcd", "abcdef", "abc", "abcde"]
// Output: "abc"

// Example 2:

// Input: ["a", "abc", "ab"]
// Output: "a"

// Example 3:

// Input: ["def"]
// Output: "def"

//Notes:
// 

// Complexity:
// Time: O(S) - Where S is sum of all strings characters
// Space: O(1)

#[allow(dead_code)]
fn find_longest_common_prefix_string<'a>(vec: &Vec<&'a str>) -> &'a str {
    fn common_prefix<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        let prefix_end_index = s1
            .chars()
            .zip(s2.chars())
            .enumerate()
            .find(|(_, (char1, char2))| char1 != char2)
            .map(|(index, _)| index)
            .unwrap_or(usize::min(s1.len(), s2.len()));
        &s1[0..prefix_end_index]
    }

    if vec.is_empty() {
        return "";
    }

    (1..vec.len()).fold(vec[0], |prx, i| common_prefix(prx, vec[i]))
}

#[allow(dead_code)]
fn find_longest_common_prefix_string2<'a>(vec: &Vec<&'a str>) -> &'a str {

    if vec.is_empty() {
        return "";
    }

    let first_string = vec[0];
    let mut min = vec[0].len();

    for i in 1..vec.len() {
        for (j, (ch1,ch2)) in first_string.chars().zip(vec[i].chars()).enumerate() {
            if ch1 != ch2 {
                return &first_string[0..j];
            }
            min = usize::min(min, vec[i].len()) 
        }
    }
    &first_string[0..min]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let strings = &vec!["abcd", "abc"];
        assert_eq!(find_longest_common_prefix_string2(strings), "abc");
    }

    #[test]
    fn test_case_2() {
        let strings = &vec!["abcd", "abcdef", "abc", "abcde"];
        assert_eq!(find_longest_common_prefix_string2(strings), "abc");
    }

    #[test]
    fn test_case_3() {
        let strings = &vec!["def"];
        assert_eq!(find_longest_common_prefix_string2(strings), "def");
    }
}
