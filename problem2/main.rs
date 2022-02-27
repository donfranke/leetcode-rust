// solution for: Remove Element
struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        for i in 0..nums.len() - 1 {
            let curr_num = nums[i];
            if curr_num == val {
                nums[i] = nums[i + 1];
            }
        }
        return 0;
    }
}

fn main() {
    let mut v1 = vec![3, 2, 2, 3];
    let x = 3;
    println!("BEFORE\n=============");
    for z in &v1 {
        println!("{}", z);
    }

    let y = Solution::remove_element(&mut v1, x);

    println!("AFTER\n=============");
    for z in &v1 {
        println!("{}", z);
    }
}
