// Problem
// We need to check if a provided string str containing brackets {}[]() is syntactically valid.
// It means that all brackets are complete and in the correct order. The string could not contain characters other than {}[]().

// Example 1:

// Input: "{([[]])}"
// Output: True

// Example 2:

// Input: []{}()
// Output: True

// Example 3:

// Input: []
// Output: true

// Example: 4
// Input: []{]
// Output: false

//Notes:
// While iterating over the string characters char we check if this char is an opening bracket.
// If yes - we push it to the stack. If it is a closing bracket then we pop the last opening bracket (popped) from the stack. If the popped opening bracket corresponds to the current closing bracket char, then all is good and so far the string is valid.
// Otherwise, we return false right away.

// Complexity:
// Time: O(n)
// Space: O(n)

#[allow(dead_code)]
fn has_valid_brackets(string: &str) -> bool {
    const OPEN_BRACKETS: &str = "{[(";
    const CLOSE_BRACKETS: &str = "}])";
    let mut stack = Vec::new();

    for char in string.chars() {
        if OPEN_BRACKETS.contains(|c| c == char) {
            stack.push(char);
        } else if CLOSE_BRACKETS.contains(|c| c == char) {
            let popped = stack.pop();

            match (popped, char) {
                (Some('('), ')') | (Some('{'), '}') | (Some('['), ']') => continue,
                _ => return false,
            }
        } else {
            panic!("the input string must only contain brackets")
        }
    }
    return stack.is_empty();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let string = "{([[]])}";
        assert_eq!(has_valid_brackets(string), true);
    }

    #[test]
    fn test_case_2() {
        let string = "[]{}()";
        assert_eq!(has_valid_brackets(string), true);
    }

    #[test]
    fn test_case_3() {
        let string = "[]{]"; 
        assert_eq!(has_valid_brackets(string), false);
    }
}
