fn main() {
    println!("{:?}", Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8));
    println!("{:?}", Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6));
    println!("{:?}", Solution::search_range(vec![], 0));
}

struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        match nums.binary_search(&target) {
            Err(_) => vec![-1, -1],
            Ok(_) => {
                let l = nums
                    .binary_search_by(|&p| (p * 2).cmp(&(target * 2 - 1)))
                    .unwrap_err();
                let r = nums
                    .binary_search_by(|&p| (p * 2).cmp(&(target * 2 + 1)))
                    .unwrap_err();
                vec![l as i32, (r - 1) as i32]
            }
        }
    }
}
