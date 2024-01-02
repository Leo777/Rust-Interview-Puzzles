// Problem
// You are given an array of strings tokens that represents an arithmetic expression in a Reverse Polish Notation.

// Evaluate the expression. Return an integer that represents the value of the expression.

// Note that:

use std::i32;

// The valid operators are '+', '-', '*', and '/'.
// Each operand may be an integer or another expression.
// The division between two integers always truncates toward zero.
// There will not be any division by zero.
// The input represents a valid arithmetic expression in a reverse polish notation.
// The answer and all the intermediate calculations can be represented in a 32-bit integer.
// // Complexity:
// // Time: O(1)
// Space: O(n)
#[allow(dead_code)]
fn rpn(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::new();
    for token in tokens {
        match &token[..] {
            "+" => {
                let result = stack.pop().unwrap() as i32 + stack.pop().unwrap() as i32;
                stack.push(result);
            }
            "-" => {
                let a = stack.pop().unwrap() as i32;
                let b = stack.pop().unwrap() as i32;

                stack.push(b - a);
            }
            "*" => {
                let result = stack.pop().unwrap() as i32 * stack.pop().unwrap() as i32;
                stack.push(result);
            }
            "/" => {
                let a = stack.pop().unwrap() as i32;
                let b = stack.pop().unwrap() as i32;

                stack.push(b / a);
            },
            value => stack.push(value.parse::<i32>().unwrap()),
        }
    }
    return stack.pop().unwrap() as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let strings: Vec<String> = vec!["2".to_string(),"1".to_string(),"+".to_string(),"3".to_string(),"*".to_string()];
        assert_eq!(rpn(strings), 9);
    }

    #[test]
    fn test_case_2() {
    let strings: Vec<String> = vec!["4".to_string(),"13".to_string(),"5".to_string(),"/".to_string(),"+".to_string()];
    assert_eq!(rpn(strings), 6);
    }

    // #[test]
    // fn test_case_3() {
    //     let string = "[]{]".to_string();
    //     assert_eq!(has_valid_brackets(string), false);
    // }
}
