fn main() {
    println!("{}", Solution::maximum_sum(vec![18, 43, 36, 13, 7]));
}

struct Solution;

use std::collections::{BinaryHeap, HashMap};
impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, BinaryHeap<i32>> = HashMap::new();
        for num in nums {
            let s = digits_sum(num);
            map.entry(s).or_insert_with(BinaryHeap::new).push(num);
        }
        let mut ans = 0;
        for (_, mut v) in map {
            if v.len() > 1 {
                ans = ans.max(v.pop().unwrap() + v.pop().unwrap());
            }
        }
        ans
    }
}

#[inline(always)]
fn digits_sum(mut num: i32) -> i32 {
    let mut ans = -1;
    while num > 0 {
        let tmp = num / 10;
        ans += num - tmp * 10;
        num = tmp;
    }
    ans
}
