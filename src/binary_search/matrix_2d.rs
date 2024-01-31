// Problem
// You are given an m x n integer matrix matrix with the following two properties:

// Each row is sorted in non-decreasing order.
// The first integer of each row is greater than the last integer of the previous row.
// Given an integer target, return true if target is in matrix or false otherwise.

// You must write a solution in O(log(m * n)) time complexity.

// Complexity:
// Time: O(log n)


use std::cmp::Ordering;

#[allow(dead_code)]
fn binary_search_2d_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (mut t, mut b) = (0, matrix.len());
    let mut row = 0;
    while t < b {
        row = t + (b - t) / 2;
        let first = matrix[row][0];
        let last = *matrix[row].last().unwrap();
        
        if target < first {
            b = row;
        } else if target > last {
            t = row + 1;
        } else {
            break;
        }
    }

    if t > b {
        return false;
    }

    let (mut l, mut r) = (0, matrix[row].len());
    while l < r {
        let col = l + (r - l) / 2;
        match target.cmp(&matrix[row][col]) {
            Ordering::Equal => return true,
            Ordering::Less => r = col,
            Ordering::Greater => l = col + 1
        }
    }
    
    false
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let matrix: Vec<Vec<i32>> = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert_eq!(binary_search_2d_matrix(matrix, 20), true);
    }

    // #[test]
    // fn test_case_2() {
    //     let matrix: Vec<Vec<i32>> = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    //     assert_eq!(binary_search_2d_matrix(matrix, 20), false);
    // }
}