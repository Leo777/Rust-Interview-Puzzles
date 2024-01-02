// Problem
// We need to check if a provided string str containing brackets {}[]() is syntactically valid.
// It means that all brackets are complete and in the correct order. The string could not contain characters other than {}[]().

// Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.

// Implement the MinStack class:

// MinStack() initializes the stack object.
//void push(int val) pushes the element val onto the stack.
// void pop() removes the element on the top of the stack.
// int top() gets the top element of the stack.
// int getMin() retrieves the minimum element in the stack.
// You must implement a solution with O(1) time complexity for each function.

// Complexity:
// Time: O(1)
// Space: O(n)
struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            stack: vec![],
            min_stack: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);

        if self.min_stack.is_empty() || Some(&val) <= self.min_stack.last() {
            self.min_stack.push(val)
        }
    }

    fn pop(&mut self) {
        let val = self.stack.pop().unwrap();
        if Some(&val) == self.min_stack.last() {
            self.min_stack.pop();
        }
    }

    fn top(&self) -> i32 {
        self.stack.last().cloned().unwrap()
    }

    fn get_min(&self) -> i32 {
        self.min_stack.last().cloned().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut min_stack = MinStack::new();
        min_stack.push(1);
        assert_eq!(min_stack.top(), 1)
    }

    #[test]
    fn test_case_2() {
        let mut min_stack = MinStack::new();
        min_stack.push(1);
        min_stack.push(2);
        min_stack.pop();
        assert_eq!(min_stack.top(), 1)
    }

    #[test]
    fn test_case_3() {
        let mut min_stack = MinStack::new();
        min_stack.push(1);
        min_stack.push(2);
        min_stack.push(3);
        assert_eq!(min_stack.get_min(), 1)
    }
}
