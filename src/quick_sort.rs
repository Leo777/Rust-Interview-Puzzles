// Problem
// We need to sort a provided vector vec using Quicksort algorithm. It is a commonly used divide and conquer sorting algorithm that on average has O(n log n) time complexity.

// Example 1:

// Input: vec = [-4, 1, 25, 50, 8, 10, 23]
// Output: [-4, 1, 8, 10, 23, 25, 50]


// Example 2:

// Input: vec = [2, 1, 6, 10, 4, 1, 3, 9, 7]
// Output: [1, 1, 2, 3, 4, 6, 7, 9, 10] 

//Notes: We need 2 functions partion and qsort.
// 1) Provie slice of all elements to qsort
// 2) Find pivot in the slice(could be last, first or middle element) in partition function
//  and move all elements less than pivot to the left and all elements greater than pivot to the right.
// at the end put pivot to the correct position numbers.swap(pivot_idx, next_idx) and return pivon index. 
// 3) In qsort if pivot > 0 put left part of the slice in qsort numbers[0..pivot_index],
// if pivot less then last element put the right part of the slice numbers[pivot_index + 1..len]. 

// Complexity:
// Time: average O(n log n), O(n^2) if array already sorted and we peak first element as pivot
// Space: O(n)

#[allow(dead_code)]
fn quick_sort(numbers: &mut Vec<i32>) -> &Vec<i32> {
    fn partition(numbers: &mut [i32]) -> usize {
        let pivot_idx = numbers.len() - 1;
        let pivot = numbers[pivot_idx];
        let mut next_idx = 0;

        (0..numbers.len()).for_each(|idx| {
            if numbers[idx] < pivot {
                numbers.swap(next_idx, idx);
                next_idx += 1;
            }
        });
        numbers.swap(pivot_idx, next_idx);

        next_idx
    }

    fn qsort(numbers: &mut [i32]) {
        let pivot_index = partition(numbers);
        let len = numbers.len();

        if pivot_index > 0 {
            qsort(&mut numbers[0..pivot_index])
        }

        if pivot_index < len - 1 {
            qsort(&mut numbers[pivot_index + 1..len])
        }
    }

    if !numbers.is_empty() {
        qsort(&mut numbers[..]);
    }

    numbers
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut numbers = vec![5, 6, 4, 2, 3, 2, 1];

        assert_eq!(quick_sort(&mut numbers), &vec![1, 2, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_case_2() {
        let mut numbers = vec![-4, 1, 25, 50, 8, 10, 23];
        assert_eq!(quick_sort(&mut numbers), &vec![-4, 1, 8, 10, 23, 25, 50]
    );
    }

    #[test]
    fn test_case_3() {
        let mut numbers = vec![0, 10, 9, 8, 7, 6, 5, -1];
        assert_eq!(quick_sort(&mut numbers), &vec![-1, 0, 5, 6, 7, 8, 9, 10]
    );
    }  
}
