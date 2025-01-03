fn main() {
    println!("{:?}", Solution::combine(6, 3))
}

struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut stack: Vec<Vec<i32>> = vec![];
        stack.push(Vec::with_capacity(k as usize));
        let mut ans: Vec<Vec<i32>> = vec![];
        while !stack.is_empty() {
            let c = stack.pop().unwrap();
            if c.len() == k as usize {
                ans.push(c);
                continue;
            }
            let prev = *c.last().unwrap_or(&0);
            for i in (prev + 1)..=(n - (k - c.len() as i32) + 1) {
                let mut c = c.clone();
                c.push(i);
                stack.push(c);
            }
        }
        ans
    }
}
