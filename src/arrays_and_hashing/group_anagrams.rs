// Problem
// Given an array of strings strs, group the anagrams together. You can return the answer in any order.
// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

//Notes:

// Complexity:
// Time: O(n*m)
// Space: O(n)

#[allow(dead_code)]
fn group_anagrams(strings: Vec<String>) -> Vec<Vec<String>> {
    let mut map = std::collections::HashMap::new();
    let offset = 'a' as usize;

    for s in strings {
        let mut chars: [u8; 26] = [0; 26];

        for c in s.chars() {
            chars[c.to_ascii_lowercase() as usize - offset] += 1;
        }

        map.entry(chars)
            .and_modify(|v: &mut Vec<String>| v.push(s.clone()))
            .or_insert(vec![s]);
    }
    map.values().cloned().collect::<Vec<Vec<String>>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let strs: Vec<String> = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
            .into_iter()
            .map(|s| s.into())
            .collect();
        let result = group_anagrams(strs);
        let expected: Vec<Vec<String>> = vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
        .into_iter()
        .map(|inner_vec| {
            inner_vec.into_iter().map(|s| s.into()).collect()
        })
        .collect();

        assert!(compare_vecs(result, expected));
    }

    // #[test]
    // fn test_case_2() {
    //     let s1 = "anagram";
    //     let s2 = "nagaram";

    //     assert_eq!(valid_anagrams(s1, s2), true);
    //     assert_eq!(valid_anagrams_2(s1, s2), true);
    // }

    // #[test]
    // fn test_case_3() {
    //     let s1 = "rat";
    //     let s2 = "car";

    //     assert_eq!(valid_anagrams(s1, s2), false);
    //     assert_eq!(valid_anagrams_2(s1, s2), false);
    // }

    fn compare_vecs(mut a: Vec<Vec<String>>, mut b: Vec<Vec<String>>) -> bool {
        a.iter_mut().for_each(|inner| inner.sort());
        b.iter_mut().for_each(|inner| inner.sort());

        a.sort();
        b.sort();

        a == b
    }
}
