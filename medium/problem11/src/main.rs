// https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/

struct Solution {}

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        // use deci-binary numbers (numbers that contain 1 or 0) to equal n
        // must be minimum number of deci-binary numbers
        // available options: 01, 10, 11
        // try each option, deal with remainder
        1
    }
}

fn main() {
    println!("!! Use tests to run function !!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // Input: n = "32"
        // Output: 3
        // Explanation: 10 + 11 + 11 = 32

        let s: String = "32".to_string();
        let target = 3;

        let ret = Solution::min_partitions(s);

        assert!(ret == target, "item 1 {} ", ret);
    }
}
