// https://leetcode.com/problems/richest-customer-wealth/

struct Solution {

}

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut highestbalance = 0;
        for (_, account) in accounts.iter().enumerate() {
            let mut k = 0;
            for (_, balance) in account.iter().enumerate() {
                k = k + balance;
            }
            if k>highestbalance {
                highestbalance = k;
            }
        }
        highestbalance
    }
}

fn main() {
    println!("Use cargo test to run these functions");    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        //Input: accounts = [[1,2,3],[3,2,1]]
        let v1 = vec![1,2,3]; 
        let v2 = vec![3,2,1]; 
        
        let mut v3 = Vec::new();
        v3.push(v1);
        v3.push(v2);
        let j = Solution::maximum_wealth(v3);
        println!("{}",j);

        assert_eq!(j,6);
    }

    #[test]
    fn test2() {
        //Input: accounts = [[1,5],[7,3],[3,5]]
        let v1 = vec![1,5]; 
        let v2 = vec![7,3]; 
        let v3 = vec![3,5]; 
        
        let mut v4 = Vec::new();
        v4.push(v1);
        v4.push(v2);
        v4.push(v3);
        let j = Solution::maximum_wealth(v4);
        println!("{}",j);

        assert_eq!(j,10);
    }
    
    #[test]
    fn test3() {
        //Input: accounts = [[2,8,7],[7,1,3],[1,9,5]]
        let v1 = vec![2,8,7]; 
        let v2 = vec![7,1,3]; 
        let v3 = vec![1,9,5]; 
        
        let mut v4 = Vec::new();
        v4.push(v1);
        v4.push(v2);
        v4.push(v3);
        let j = Solution::maximum_wealth(v4);
        println!("{}",j);

        assert_eq!(j,17);
    }

    #[test]
    fn test4() {
        //Input: accounts = [[1,9,3],[8,0],[2,4,4]]
        let v1 = vec![1,9,3]; 
        let v2 = vec![8,0]; 
        let v3 = vec![2,4,4]; 
        
        let mut v4 = Vec::new();
        v4.push(v1);
        v4.push(v2);
        v4.push(v3);
        let j = Solution::maximum_wealth(v4);
        println!("{}",j);

        assert_eq!(j,13);
    }
}