// Problem
// We need to find a maximum product of 2 numbers in a provided vector of numbers vec.
// In math, a product of 2 numbers is a result of multiplication of these 2 numbers.

//Notes:

// Complexity:
// Time: O(log n)



#[allow(dead_code)]
fn raise_number_to_power(mut a: i32, mut b: i32) -> i32 {
    let mut pow = 1;

    while b > 0 {
        if (b % 2) == 1 {
            pow *= a;
        }

        b /= 2;
        a *= a;
    }
    pow
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let a = 2;
        let b = 4;
        assert_eq!(raise_number_to_power(a, b), 16);
    }

    #[test]
    fn test_case_2() {
        let a = -2;
        let b = 3;
        assert_eq!(raise_number_to_power(a, b), -8);
    }

    #[test]
    fn test_case_3() {
        let a = -2;
        let b = 10;
        assert_eq!(raise_number_to_power(a, b), 1024);
    }

    #[test]
    fn test_case_4() {
        let a = 5;
        let b = 0;
        assert_eq!(raise_number_to_power(a, b), 1);
    }
}
