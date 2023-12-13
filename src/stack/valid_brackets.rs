// Problem
// We need to check if a provided string str containing brackets {}[]() is syntactically valid.
// It means that all brackets are complete and in the correct order. The string could not contain characters other than {}[]().

//Notes:
// While iterating over the string characters char we check if this char is an opening bracket.
// If yes - we push it to the stack. If it is a closing bracket then we pop the last opening bracket (popped) from the stack. If the popped opening bracket corresponds to the current closing bracket char, then all is good and so far the string is valid.
// Otherwise, we return false right away.

// Complexity:
// Time: O(n)
// Space: O(n)

#[allow(dead_code)]
fn has_valid_brackets(s: String) -> bool {
    let mut stack = Vec::new();
        for i in s.chars() {
            match i {
                '{' => stack.push('}'),
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '}'|')'|']' if Some(i) != stack.pop() => return false,
                _ => (),
            }
        }   
        return stack.is_empty();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let string = "{([[]])}".to_string();
        assert_eq!(has_valid_brackets(string), true);
    }

    #[test]
    fn test_case_2() {
        let string = "[]{}()".to_string();
        assert_eq!(has_valid_brackets(string), true);
    }

    #[test]
    fn test_case_3() {
        let string = "[]{]".to_string();
        assert_eq!(has_valid_brackets(string), false);
    }
}
