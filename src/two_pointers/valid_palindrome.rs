// Problem
// A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward.
//  Alphanumeric characters include letters and numbers.

// Given a string s, return true if it is a palindrome, or false otherwise.

//Notes:

// Complexity:
// Time: O(n)
// Space: O(n)


//With extra space and build in functions
#[allow(dead_code)]
fn is_palindrome(string: String) -> bool {
    let s: Vec<char> = string
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();

    let len = s.len();

    for i in 0..(len / 2) {
        if s[i] != s[len - i - 1] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let string = "race a car".to_string();
        assert_eq!(is_palindrome(string), false);
    }

    #[test]
    fn test_case_2() {
        let string = "A man, a plan, a canal: Panama".to_string();
        assert_eq!(is_palindrome(string), true);
    }
}
