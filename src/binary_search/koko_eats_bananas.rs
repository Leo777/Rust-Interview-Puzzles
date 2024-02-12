// Problem
// Koko loves to eat bananas. There are n piles of bananas, the ith pile has piles[i] bananas. The guards have gone and will come back in h hours.

// Koko can decide her bananas-per-hour eating speed of k. Each hour, she chooses some pile of bananas and eats k bananas from that pile. If the pile has less than k bananas, she eats all of them instead and will not eat any more bananas during this hour.

// Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.

// Return the minimum integer k such that she can eat all the bananas within h hours.

// Complexity:
// Time: O(log n)

use std::cmp::Ordering;

#[allow(dead_code)]

fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut l = 1;
    let mut r = *piles.iter().max().unwrap();
    let mut res = r;
    while l <= r {
        let middle = l + (r - l) / 2;

        let hours: i32 = piles.iter().map(|&pile| (pile + middle - 1) / middle).sum();

        match hours.cmp(&h) {
            Ordering::Greater => l = middle + 1,
            Ordering::Less | Ordering::Equal => {
                res = res.min(middle);
                r = middle - 1
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let piles: Vec<i32> = vec![3, 6, 7, 11];
        assert_eq!(min_eating_speed(piles, 8), 4);
    }
}
