// Problem
// Given a vector of non-negative integer numbers vec, arrange the numbers in a way that they form the largest possible number and return its string representation.

//Notes:
// The first idea that could come to mind is just to convert the input vector of numbers vec to the vector of strings and sort it alphabetically in descending order.
// It is almost what we need but not exactly - in some cases it would produce an incorrect result. 
// For this example (vec = [40, 45, 4, 5, 8]), this alphabetical sorting would put 40 in front of 4, which would give us "404" instead of the greater "440" number.

// Thus, to resolve the problem for numbers with the same leading digits, we could provide sort_by with a comparator function that compares 2 strings in 2 different orders: b + a and a + b. 
// In our example, a = 40, b = 4; b + a = "440"; a + b = "404". It means that "440".cmp("404") call would return Ordering::Greater, which means that b = 4 would be put before a = 40 in the ordering. 
// And that is exactly what we need.

// After the vector is ordered, we just need to concatenate numbers converted to strings. 
// Also, we handle the special case when the vector contains only zeros - in that case, we return "0" as the result.

// Complexity:
// Time: O(nlog(n))

#[allow(dead_code)]
fn form_largest_number(mut vec: Vec<u32>) -> String {
    vec.sort_by(|a,b| {
        let order_1 = b.to_string() + a.to_string().as_str();
        let order_2 = a.to_string() + b.to_string().as_str();
        (order_1).cmp(&order_2)
    });

    if vec[0] == 0 {
        String::from("0")
    } else {
        vec.iter()
            .fold(String::from(""), |acc, x| acc + x.to_string().as_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let vec = vec![40, 45, 4, 5, 8];
        assert_eq!(form_largest_number(vec), "8545440");
    }

    #[test]
    fn test_case_2() {
        let vec = vec![0, 1, 2, 3];
        assert_eq!(form_largest_number(vec), "3210");
    }

    #[test]
    fn test_case_3() {
        let vec = vec![10, 5];
        assert_eq!(form_largest_number(vec), "510");
    }

    #[test]
    fn test_case_4() {
        let vec = vec![1];
        assert_eq!(form_largest_number(vec), "1");
    }
}
