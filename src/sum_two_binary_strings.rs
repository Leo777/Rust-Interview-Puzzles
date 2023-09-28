// Problem
// Given 2 strings s1 and s2 that contain binary numbers, we need to produce another string containing the sum of these numbers.

//Notes:
// While iterating over the string characters char we check if this char is an opening bracket.
// If yes - we push it to the stack. If it is a closing bracket then we pop the last opening bracket (popped) from the stack. If the popped opening bracket corresponds to the current closing bracket char, then all is good and so far the string is valid.
// Otherwise, we return false right away.

// Complexity:
// Time: O(n)
// Space: O(n)

#[allow(dead_code)]
fn add_binary_strings(s1: &str, s2: &str) -> String {
    let (long_str, short_str) = if s1.len() > s2.len() {
        (s1, s2)
    } else {
        (s2, s1)
    };

    let mut short_iter = short_str.chars().rev();
    let mut carry = 0;
    let mut res: String = long_str
        .chars()
        .rev()
        .map(|lchar| {
            let (char, car) = match (lchar, short_iter.next(), carry) {
                ('0', Some('0'), 0) => ('0', 0),
                ('0', Some('0'), 1) => ('1', 0),
                ('0', Some('1'), 0) => ('1', 0),
                ('0', Some('1'), 1) => ('0', 1),
                ('1', Some('0'), 0) => ('1', 0),
                ('1', Some('0'), 1) => ('0', 1),
                ('1', Some('1'), 0) => ('0', 1),
                ('1', Some('1'), 1) => ('1', 1),
                ('0', None, 0) => ('0', 0),
                ('1', None, 0) => ('1', 0),
                ('0', None, 1) => ('1', 0),
                ('1', None, 1) => ('0', 1),
                _ => panic!("incorrect input"),
            };
            carry = car;
            char
        })
        .collect();

    if carry == 1 {
        res.push('1');
    }

    res.chars().rev().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let s1 = "10000";
        let s2 = "1";
        assert_eq!(add_binary_strings(s1, s2), "10001");
    }

    #[test]
    fn test_case_2() {
        let s1 = "1";
        let s2 = "111";
        assert_eq!(add_binary_strings(s1, s2), "1000");
    }

    #[test]
    fn test_case_3() {
        let s1 = "111";
        let s2 = "111";
        assert_eq!(add_binary_strings(s1, s2), "1110");
    }

    #[test]
    fn test_case_4() {
        let s1 = "1011";
        let s2 = "1010";
        assert_eq!(add_binary_strings(s1, s2), "10101");
    }
}
