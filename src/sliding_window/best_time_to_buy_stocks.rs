// Problem
// We need to find all equilibrium indices in the provided vector vec.
//  An equilibrium index in the vector vec is an index i where the sum of vec[0..i-1] elements is equal to the sum of vec[i+1..len] elements.

use std::cmp;

#[allow(dead_code)]
fn max_profit(prices: Vec<i32>) -> i32 {
    let (mut l, mut r) = (0, 1);
    let mut max_profit = 0;

    while r < prices.len() {
        if prices[l] < prices[r] {
            let profit = prices[r] - prices[l];
            max_profit = std::cmp::max(max_profit, profit)
        } else {
            l = r
        }
        r += 1;
    }
    max_profit
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(prices), 5);
    }

    #[test]
    fn test_case_2() {
        let prices = vec![7,6,4,3,1];
        assert_eq!(max_profit(prices), 0);
    }
}
