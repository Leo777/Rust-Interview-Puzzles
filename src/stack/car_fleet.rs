// Problem
// There are n cars going to the same destination along a one-lane road. The destination is target miles away.
//
// // You are given two integer array position and speed, both of length n, where position[i] is the position of the ith car and speed[i] is the speed of the ith car (in miles per hour).

// A car can never pass another car ahead of it, but it can catch up to it and drive bumper to bumper at the same speed. The faster car will slow down to match the slower car's speed. The distance between these two cars is ignored (i.e., they are assumed to have the same position).

// A car fleet is some non-empty set of cars driving at the same position and same speed. Note that a single car is also a car fleet.

// If a car catches up to a car fleet right at the destination point, it will still be considered as one car fleet.

// Return the number of car fleets that will arrive at the destination.

use std::iter;

// Note that:
// Complexity:
// Time: O(nlogn)
// Space: O(n)
#[allow(dead_code)]
fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut position_speed_pair: Vec<(f64, f64)> = position
        .iter()
        .map(|x| *x as f64)
        .zip(speed.iter().map(|x| *x as f64))
        .collect();

    position_speed_pair.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut stack = vec![];
    for (pos, speed) in position_speed_pair.iter().rev() {
        stack.push((target as f64 - pos) / speed);
        if stack.len() >= 2 && stack.last() <= stack.get(stack.len() - 2) {
            stack.pop();
        }
    }
    stack.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let position = vec![10, 8, 0, 5, 3];
        let speed = vec![2, 4, 1, 1, 3];
        let target = 12;
        assert_eq!(
            car_fleet(target, position, speed),
            3
        );
    }
}
