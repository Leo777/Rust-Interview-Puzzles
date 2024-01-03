// Problem
// Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

// Note that:
// Complexity:
// Time: O(1)
// Space: O(n)
#[allow(dead_code)]
fn generate_parentesis(n: i32) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut stack: String = String::new();

    fn backtrack(res: &mut Vec<String>, stack: &mut String, open: i32, close: i32) {
        if open == 0 && close == 0 {
            res.push(stack.clone());
            return;
        }

        if open > 0 {
            stack.push('(');
            backtrack(res, stack, open - 1, close);
            stack.pop();
        }
        if open < close {
            stack.push(')');
            backtrack(res, stack, open, close - 1);
            stack.pop();
        }  

    }
    backtrack(&mut res, &mut stack, n, n);
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let strings: Vec<String> = vec!["((()))".to_string(),"(()())".to_string(),"(())()".to_string(),"()(())".to_string(),"()()()".to_string()];
        assert_eq!(generate_parentesis(3), strings);
    }
}
