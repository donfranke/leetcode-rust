//https://leetcode.com/problems/final-value-of-variable-after-performing-operations/

macro_rules! vec_of_strings {
  ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

struct Solution {}

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut curr_val = 0;

        for item in &operations {
            if item.contains("++") {
                curr_val += 1
            }

            if item.contains("--") {
                curr_val -= 1;
            }
        }
        curr_val
    }
}

fn main() {
    println!("Use tests to execute function");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // Input: operations = ["--X","X++","X++"]
        // Output: 1
        let v1: Vec<String> = vec_of_strings!["--X", "X++", "X++"];
        let target = 1;

        let ret = Solution::final_value_after_operations(v1);

        assert!(ret == target, "item 1 {} ", ret);
    }

    #[test]
    fn test2() {
        // Input: operations = ["++X","++X","X++"]
        // Output: 3
        let v1: Vec<String> = vec_of_strings!["++X", "++X", "X++"];
        let target = 3;

        let ret = Solution::final_value_after_operations(v1);

        assert!(ret == target, "item 1 {} ", ret);
    }

    #[test]
    fn test3() {
        // Input: operations = ["X++","++X","--X","X--"]
        // Output: 0
        let v1: Vec<String> = vec_of_strings!["X++", "++X", "--X", "X--"];
        let target = 0;

        let ret = Solution::final_value_after_operations(v1);

        assert!(ret == target, "item 1 {} ", ret);
    }
}
