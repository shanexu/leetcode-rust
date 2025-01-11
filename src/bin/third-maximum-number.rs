fn main() {
    println!("{}", Solution::third_max(vec![3, 2, 1]));
    println!("{}", Solution::third_max(vec![1, 2]));
    println!("{}", Solution::third_max(vec![2, 2, 3, 1]));
    println!("{}", Solution::third_max(vec![5, 2, 4, 1, 3, 6, 0]));
    println!("{}", Solution::third_max(vec![5, 2, 2]));
}

struct Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut ans = Vec::with_capacity(3);
        'out: for mut num in nums {
            let n = ans.len();
            for i in 0..n {
                if ans[i] < num {
                    std::mem::swap(&mut ans[i], &mut num);
                } else if ans[i] == num {
                    continue 'out;
                }
            }
            if n < 3 {
                ans.push(num);
            }
        }
        *ans.get(2).unwrap_or(&ans[0])
    }
}
