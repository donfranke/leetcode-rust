// https://leetcode.com/problems/baseball-game/
struct Solution {
}

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut list_of_scores: Vec<i32> = Vec::new();
 
        for (_, x) in ops.iter().enumerate() {
            if x == "C" {
                list_of_scores.pop();
            } else if x == "D" {
                let mut new_score: i32 = 0;
 
                new_score = list_of_scores[list_of_scores.len() - 1];
                new_score = new_score * 2;
                list_of_scores.push(new_score);
            } else if x == "+" {
                let mut new_score: i32 = 0;
 
                new_score = list_of_scores[list_of_scores.len() - 1]
                    + list_of_scores[list_of_scores.len() - 2];
                list_of_scores.push(new_score);
            } else {
                let new_score = x.parse::<i32>().unwrap();
                
                list_of_scores.push(new_score);
            }
        }
        let mut final_score = 0;
        for (_, y) in list_of_scores.iter().enumerate() {
            final_score += y;
        }
 
        final_score
    }
}

fn main() {
    let v1: Vec<String> = vec![
            "8".into(),
            "4".into(),
            "-1".into(),
            "D".into(),
            "7".into(),
            "D".into(),
            "+".into(),
    ];
    let ret = Solution::cal_points(v1);
    println!("{}", ret);
}

#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn test1() {
        //ops = ["5","2","C","D","+"]
        // Output: 30
        let v1: Vec<String> = vec!["5".into(), "2".into(), "C".into(), "D".into(), "+".into()];
        let ret = Solution::cal_points(v1);
 
        assert_eq!(ret, 30);
    }
 
    #[test]
    fn test2() {
        //Input: ops = ["5","-2","4","C","D","9","+","+"]
        // Output: 27
        let v1: Vec<String> = vec![
            "5".into(),
            "-2".into(),
            "4".into(),
            "C".into(),
            "D".into(),
            "9".into(),
            "+".into(),
            "+".into(),
        ];
        let ret = Solution::cal_points(v1);
 
        assert_eq!(ret, 27);
    }
 }