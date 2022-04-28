// https://leetcode.com/problems/find-target-indices-after-sorting-array/
struct Solution {

}

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut v1 = nums;
        v1.sort();
        
        let mut i = 0;
        let mut v2 = Vec::new();
        
        for num in v1 {
            if num==target {
                v2.push(i);
            }
            i = i + 1;
        }
        v2
    }
}

fn main() {
    println!("Use cargo test to run");
}

#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test1() {
        // Input: nums = [1,2,5,2,3], target = 2
        // Output: [1,2]
        let v1: Vec<i32> = vec![1,2,5,2,3];
        let target = 2;
        
        let ret = Solution::target_indices(v1, target);

        assert!(ret[0]==1 && ret[1]==2, "item 1 {} item 2 {}", ret[0],ret[1]);        
    }

    #[test]
    fn test2() {
        // Input: nums = [1,2,5,2,3], target = 3
        // Output: [3]
        let v1: Vec<i32> = vec![1,2,5,2,3];
        let target = 3;
        
        let ret = Solution::target_indices(v1, target);

        assert!(ret[0]==3, "item {} ", ret[0]);        
    }

    #[test]
    fn test3() {
        // Input: nums = [1,2,5,2,3], target = 5
        // Output: [4]
        let v1: Vec<i32> = vec![1,2,5,2,3];
        let target = 5;
        
        let ret = Solution::target_indices(v1, target);

        assert!(ret[0]==4, "item {} ", ret[0]);        
    }
}