fn main() {
        println!("{:?}", Solution::diff_ways_to_compute("2*3-4*5".to_string()));
}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        fn helper<'a>(expression: &'a [u8], memo: &mut HashMap<&'a [u8], Vec<i32>>) -> Vec<i32> {
            if let Some(res) = memo.get(expression) {
                return res.clone();
            }
            let mut res = vec![];
            for i in 0..expression.len() {
                let b = expression[i];
                if !b.is_ascii_digit() {
                    let res_pre = helper(&expression[0..i], memo);
                    let res_post = helper(&expression[i + 1..], memo);
                    for j in 0..res_pre.len() {
                        for k in 0..res_post.len() {
                            if b == b'+' {
                                res.push(res_pre[j] + res_post[k]);
                            } else if b == b'-' {
                                res.push(res_pre[j] - res_post[k]);
                            } else if b == b'*' {
                                res.push(res_pre[j] * res_post[k]);
                            }
                        }
                    }
                }
            }
            if res.is_empty() {
                let mut no = 0;
                for b in expression {
                    no *= 10;
                    no += (b - b'0') as i32;
                }
                res.push(no);
            }
            memo.insert(expression, res.clone());
            res
        }
        let expression = expression.as_bytes();
        let mut memo = HashMap::new();
        helper(expression, &mut memo)
    }
}
