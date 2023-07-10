// Problem
// We need to check whether a provided number num is a Palindrome. 
// A Palindrome number is a number that reads the same forward and backward. 
// This puzzle does not allow the conversion of the number to a string or the use of other auxiliary data structures.


//Notes:
// On the first iteration over the reversed vec iterator, we accumulate and save to the created map sums of the right elements for the each index value.
// On the second iteration, we accumulate sums of the left elements for the each index value.
// At the same time we check if for the current index our accumulated left_sum is equal to the right sum extracted from the map for this index.
// If that is the case, then we have found_equilibrium and the filter's predicate would satisfy for that index (which means that index would be included in the result).

// Complexity:
// Time: O(n)


#[allow(dead_code)]
fn is_palindrom(mut number: i32) -> bool {
    match number {
        n if n == 0 => return true,       //0 is palindrome
        n if n < 0 => return false,       //negative num is not
        n if n % 10 == 0 => return false, //num ending 0 is not
        _ => (),
    }

   let mut num_reversed = 0;
   while number > num_reversed {
    num_reversed = num_reversed * 10 + number % 10;
     number /= 10;
   }

    //if num originally had an even number of digits e.g. 1221 then
    //[num == num_reversed == 12].
    //if num had an odd number of digits e.g. 12321 then
    //[num == 12, num_reversed = 123], the mid number (3)
    //could be ignored in this case.
   number == num_reversed || number == num_reversed / 10
   
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let number:i32 = 212;
        assert_eq!(is_palindrom(number), true);
    }

    #[test]
    fn test_case_2() {
        let number:i32 = 112211;
        assert_eq!(is_palindrom(number), true);
    }
}
