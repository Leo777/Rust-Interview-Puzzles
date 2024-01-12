// Problem
// Given an array of integers heights representing the histogram's bar height where the width of each bar is 1, return the area of the largest rectangle in the histogram.

use std::iter;

// Note that:
// Complexity:
// Time: O(n)
// Space: O(n)
#[allow(dead_code)]
fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut max_area = 0;
    let mut stack: Vec<(usize, i32)> = Vec::new();

    for (i, &h) in heights.iter().enumerate() {
        let mut start = i;
        while !stack.is_empty() && stack.last().unwrap().1 > h {
            let (index, height) = stack.pop().unwrap();
            max_area = std::cmp::max(max_area, height * (i as i32 - index as i32));
            start = index;
        }
        stack.push((start, h));
    }

    for (i, h) in stack.iter().rev() {
        max_area = std::cmp::max(max_area, h * (heights.len() as i32 - * i as i32));
    }
    max_area
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let heights = vec![2,1,5,6,2,3];
        assert_eq!(largest_rectangle_area(heights), 10);
    }

    #[test]
    fn test_case_2() {
        let heights = vec![2,1,2];
        assert_eq!(largest_rectangle_area(heights), 3);
    }
}
