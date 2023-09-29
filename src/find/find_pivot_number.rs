// Problem
// Given a non-negative integer number n, we need to find the pivot number. 
// The pivot number p is a number such as the sum of all numbers [0..p] equals the sum of all numbers [p..n].

//Notes:
// Use arithmetic progression formula to get sum of all numbers then calculate left sum compare it to the right sum right_sum = sum - left sum. Or second solution just find sqrt of sum.

// Complexity:
// Time: O(n) or O(1)
// Space: O(1)

#[allow(dead_code)]
fn find_pivot_number_1(n: i32) -> Option<i32> {
    let sum = n * (n + 1) / 2;

    let mut left_sum = 0;
    (0..=n).find_map(|e| {
        let right_sum = sum - left_sum;
        left_sum += e;

        if left_sum == right_sum {
            Some(e)
        } else {
            None
        }
    })
}

#[allow(dead_code)]
fn find_pivot_number_2(n: i32) -> Option<i32> {
    let sum = n * (n + 1) / 2;
    let p = (sum as f32).sqrt() as i32;
    (p * p == sum).then(|| p)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let n = 1;

        assert_eq!(find_pivot_number_1(n), Some(1));
        assert_eq!(find_pivot_number_2(n), Some(1));
    }

    #[test]
    fn test_case_2() {
        let n = 4;

        assert_eq!(find_pivot_number_1(n), None);
        assert_eq!(find_pivot_number_2(n), None);
    }

    #[test]
    fn test_case_3() {
        let n = 5;

        assert_eq!(find_pivot_number_1(n), None);
        assert_eq!(find_pivot_number_2(n), None);
    }

    #[test]
    fn test_case_4() {
        let n = 8;

        assert_eq!(find_pivot_number_1(n), Some(6));
        assert_eq!(find_pivot_number_2(n), Some(6));
    }
}
