// https://leetcode.com/problems/concatenation-of-array

struct Solution {

}

impl Solution {
    pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        let mut cloned_nums = nums.clone();
        cloned_nums.append(&mut nums);
        cloned_nums
    }
}

fn main() {
    // initial array - array to concatenate
    let v1 = vec![1,3,2,1];
    let v2 = Solution::get_concatenation(v1);
    for i in v2 {
}
