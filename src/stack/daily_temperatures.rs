// Problem
// Given an array of integers temperatures represents the daily temperatures, return an array answer such that answer[i] is the number of days you have to wait after the ith day to get a warmer temperature. 
// If there is no future day for which this is possible, keep answer[i] == 0 instead.

// Note that:
// Complexity:
// Time: O(n)
// Space: O(n)
#[allow(dead_code)]
fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; temperatures.len()];
    let mut stack: Vec<(usize, i32)> = vec![];

    for (i, &temp) in temperatures.iter().enumerate() {
        while !stack.is_empty() && stack.last().unwrap().1 < temp {
            let (index, _) = stack.pop().unwrap();
            result[index] = i as i32 - index as i32;
        }

        stack.push((i, temp));
    }    

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
      let temperatures = vec![73,74,75,71,69,72,76,73]; 
        assert_eq!(daily_temperatures(temperatures), vec![1,1,4,2,1,1,0,0]);
    }
}
