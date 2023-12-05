// Problem
// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.

// Notice that the solution set must not contain duplicate triplets.

//Notes:

// Complexity:
// Time: O(n2)
// Space: O(1) or O(n) dependce on sorting implementation.

#[allow(dead_code)]
fn three_sum(mut numbers: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    numbers.sort();

    for (i, &e) in numbers.iter().enumerate() {
        if i > 0 && numbers[i] == numbers[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = numbers.len() - 1;
        

        while left < right {
            let current_sum = e + numbers[left] + numbers[right];
            if current_sum > 0 {
                right -= 1;
            } else if current_sum < 0 {
                left += 1;
            } else {
                result.push(vec![e, numbers[left], numbers[right]]);
                left += 1;

                while numbers[left] == numbers[left - 1] && left < right {
                    left += 1;
                }
            }
        }
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let numbers = vec![-1, 0, 1, 2, -1, -4];

        assert_eq!(three_sum(numbers), vec![vec![-1,-1,2], vec![-1,0,1]]);
    }

    #[test]
    fn test_case_2() {
        let numbers = vec![0,1,1];
        assert!(three_sum(numbers).is_empty());
    }
}
