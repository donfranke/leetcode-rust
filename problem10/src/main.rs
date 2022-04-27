// https://leetcode.com/problems/search-insert-position/

struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut next_num = -1;
        let mut curr_num = -1;

        for i in 0..nums.len() {
            curr_num = nums[i];
            if target < curr_num {
                return 0;
            }
            if curr_num == target {
                return i as i32;
            } else {
                if i == nums.len() - 1 {
                    return i as i32 + 1;
                } else {
                    next_num = nums[i + 1];
                    if target > curr_num && target < next_num {
                        return i as i32 + 1;
                    }
                }
            }
        }
        return target;
    }
}

fn main() {
    println!("!! Use cargo test to run function !!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // Input: nums = [1,3,5,6], target = 5
        // Output: 2
        let v: Vec<i32> = vec![1, 3, 5, 6];
        let target = 5;
        let expected_result = 2;

        let res = Solution::search_insert(v, target);

        assert!(
            res == expected_result,
            "expected result {} ",
            expected_result
        );
    }

    #[test]
    fn test2() {
        // Input: nums = [1,3,5,6], target = 2
        // Output: 1
        let v: Vec<i32> = vec![1, 3, 5, 6];
        let target = 2;
        let expected_result = 1;

        let res = Solution::search_insert(v, target);

        assert!(
            res == expected_result,
            "expected result {} {}",
            expected_result,
            res
        );
    }

    #[test]
    fn test3() {
        // Input: nums = [1,3,5,6], target = 7
        // Output: 4
        let v: Vec<i32> = vec![1, 3, 5, 6];
        let target = 7;
        let expected_result = 4;

        let res = Solution::search_insert(v, target);

        assert!(
            res == expected_result,
            "expected result {} {}",
            expected_result,
            res
        );
    }

    #[test]
    fn test4() {
        // Input: nums = [1,3,5,6], target = 0
        // Output: 0
        let v: Vec<i32> = vec![1, 3, 5, 6];
        let target = 0;
        let expected_result = 0;

        let res = Solution::search_insert(v, target);

        assert!(
            res == expected_result,
            "expected result {} {}",
            expected_result,
            res
        );
    }

    #[test]
    fn test5() {
        // Input: nums = [2,5], target = 1
        // Output: 0
        let v: Vec<i32> = vec![2, 5];
        let target = 1;
        let expected_result = 0;

        let res = Solution::search_insert(v, target);

        assert!(
            res == expected_result,
            "expected result {} {}",
            expected_result,
            res
        );
    }
}
