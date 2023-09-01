// Problem
// For a provided string str we need to remove adjacent duplicate characters.
//  It is acceptable for this puzzle to return a new string as a result instead of doing the actual in-place mutation.

//Notes:

// Complexity:
// Time: O(n)

#[allow(dead_code)]
fn remove_duplicate_char(s: &str) -> String {
    let mut prev: Option<char> = None;
    s.chars()
        .into_iter()
        .filter(|&char| {
            if Some(char) != prev {
                prev = Some(char);
                true
            } else {
                false
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "aaabccccddde";

        assert_eq!(remove_duplicate_char(s), "abcde");
    }

    #[test]
    fn test_case_2() {
        let s = "aaabccccddde";

        assert_eq!(remove_duplicate_char(s), "abcde");
    }

    #[test]
    fn test_case_3() {
        let s = "aaa";

        assert_eq!(remove_duplicate_char(s), "a");
    }

    #[test]
    fn test_case_4() {
        let s = "abccccddde";

        assert_eq!(remove_duplicate_char(s), "abcde");
    }
}
