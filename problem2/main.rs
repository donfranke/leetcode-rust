// solution for: Remove Element
// https://leetcode.com/problems/remove-element/
// five elements can be returned in any order.
struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> usize {
        let mut ary_size = nums.len();
        let mut i = 0;

        while i < nums.len() {
            if nums[i] == val {
                ary_size = ary_size - 1;

                for j in i..nums.len() {
                    if j < nums.len() - 1 {
                        nums[j] = nums[j + 1];
                    } else {
                        nums[j] = -1;
                    }
                }
            } else {
                i = i + 1;
            }
        }
        return ary_size;
    }
}

fn main() {
    println!("Running...");
    //Input: nums = [3,2,2,3], val = 3
    //Output: 2, nums = [2,2,_,_]
    //let mut v1 = vec![3, 2, 2, 3];
    //let x = 3;

    //Input: nums = [0,1,2,2,3,0,4,2], val = 2
    //Output: 5, nums = [0,1,4,0,3,_,_,_]
    let mut v1 = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let x = 2;

    let y = Solution::remove_element(&mut v1, x);
    println!("Output: {}", y);

    for z in &v1 {
        println!("{}", z);
    }
}
