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
mod test;
