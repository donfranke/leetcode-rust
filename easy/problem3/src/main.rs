// https://leetcode.com/problems/build-array-from-permutation/

struct Solution {

}

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut v1 = Vec::new();
        for i in 0..nums.len() {
            v1.push(nums[nums[i] as usize]);
        }
        v1
    }
}

fn main() {
    let v1 = vec![5,0,1,2,3,4];
    let v2 = Solution::build_array(v1);
    for i in v2 {
        println!("{}", i);
    }
}

