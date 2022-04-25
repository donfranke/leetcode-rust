// https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

struct Solution {}

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut i_longest = 0;
        for string_item in sentences {
            let mut temp_len = string_item.split_whitespace().count();
            if temp_len > i_longest {
                i_longest = temp_len;
            }
        }

        i_longest as i32
    }
}

fn main() {
    println!("!! Use test cases to execute function !!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        //Input: sentences = ["alice and bob love leetcode", "i think so too", "this is great thanks very much"]
        //Output: 6
        let v: Vec<String> = vec_of_strings![
            "alice and bob love leetcode",
            "i think so too",
            "this is great thanks very much"
        ];
        let target = 6;

        let ret = Solution::most_words_found(v);

        assert!(ret == target, "item {} ", ret);
    }

    #[test]
    fn test2() {
        //Input: sentences = ["please wait", "continue to fight", "continue to win"]
        //Output: 3
        let v: Vec<String> = vec_of_strings!["please wait", "continue to fight", "continue to win"];
        let target = 3;

        let ret = Solution::most_words_found(v);

        assert!(ret == target, "item {} ", ret);
    }
}
