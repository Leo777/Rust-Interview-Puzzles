// Problem
// Given a list of strings we have write two functions:
// 1) Encode all strings in to the one string
// 2) Decode all strings from encoded

//Notes:

// Complexity:
// Time: O(n)
// Space: O(1)

#[allow(dead_code)]
fn encode(strings: Vec<String>) -> String {
    let mut store = String::new();

    for s in strings {
        let len = s.chars().count();

        store.push_str(len.to_string().as_str());
        store.push('#');
        store.push_str(&s);
    }

    store
}

#[allow(dead_code)]
fn decode(string: String) -> Vec<String> {
    let chars: Vec<char> = string.chars().collect();
    let mut i = 0;

    let mut res = vec![];
    while i < chars.len() {
        let mut j = i;
        while chars[j] != '#' {
            j += 1;
        }
        let slice_len: String = chars[i..j].iter().collect();
        let length: usize = slice_len.parse().unwrap();
        let slice = &chars[j + 1..j + 1 + length];

        res.push(slice.into_iter().collect::<String>());

        i = j + 1 + length;
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let strings = vec![
            "bulba".to_string(),
            "potato".to_string(),
            "ziemniak".to_string(),
        ];

        let encoded = encode(strings.clone());

        assert_eq!("5#bulba6#potato8#ziemniak".to_string(), encoded);
        assert_eq!(decode(encoded), strings);
    }
}
