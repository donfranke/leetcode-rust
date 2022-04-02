// https://leetcode.com/problems/running-sum-of-1d-array/

struct Solution {

}

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut v1 = Vec::new();
        let mut j = 0;
        for i in 0..nums.len() {
            j = j + nums[i];
            v1.push(j);
        }
        v1
    }
}

fn main() {
    //let v1 = vec![1,2,3,4];  // 1, 3, 6, 10
    //let v1 = vec![1,1,1,1,1];  // 1, 2, 3, 4, 5
    let v1 = vec![3,1,2,10,1];  // 3,4,6,16,17
    let v2 = Solution::running_sum(v1);
    for i in v2 {
        println!("{}", i);
    }
}

