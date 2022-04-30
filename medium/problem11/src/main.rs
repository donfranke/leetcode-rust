// https://leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers/

struct Solution {}

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut highest_num = 0;
        let cs = n.chars();
        for c in cs {
            let num = c.to_digit(10).unwrap();
            if num > highest_num {
                highest_num = num;
            }
        }
        highest_num as i32
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

    #[test]
    fn test2() {
        // Input: n = "82734"
        // Output: 8
        let s: String = "82734".to_string();
        let target = 8;

        let ret = Solution::min_partitions(s);

        assert!(ret == target, "item 2 {} ", ret);
    }

    #[test]
    fn test3() {
        // Input: n = "27346209830709182346"
        // Output: 9

        let s: String = "27346209830709182346".to_string();
        let target = 9;

        let ret = Solution::min_partitions(s);

        assert!(ret == target, "item 3 {} ", ret);
    }
}
