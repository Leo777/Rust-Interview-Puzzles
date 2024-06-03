// Problem
// You are given two strings s1 and s2.
// Return true if s2 contains a permutation of s1, or false otherwise. That means if a permutation of s1 exists as a substring of s2, then return true.
// Both strings only contain lowercase letters.

#[allow(dead_code)]
fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }

    let (mut s1_cnt, mut s2_cnt) = ([0; 26], [0; 26]);

    for i in 0..s1.len() {
        s1_cnt[s1.chars().nth(i).unwrap() as usize - 'a' as usize] += 1;
        s2_cnt[s2.chars().nth(i).unwrap() as usize - 'a' as usize] += 1;
    }

    if s1_cnt == s2_cnt {
        return true;
    }

    for i in s1.len()..s2.len() {
        s2_cnt[s2.chars().nth(i).unwrap() as usize - 'a' as usize] += 1;
        s2_cnt[s2.chars().nth(i - s1.len()).unwrap() as usize - 'a' as usize] -= 1;

        if s1_cnt == s2_cnt {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let s1 = "abc".to_string();
        let s2 = "lecabee".to_string();
        assert_eq!(check_inclusion(s1, s2), true);
    }

    #[test]
    fn test_case_2() {
        let s1 = "abc".to_string();
        let s2 = "lecaabee".to_string();
        assert_eq!(check_inclusion(s1, s2), false);
    }
}
