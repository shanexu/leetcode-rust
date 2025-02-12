fn main() {
    println!("{}", Solution::maximum_sum(vec![18, 43, 36, 13, 7]));
}

struct Solution;

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut map = vec![0; 82];
        let mut ans = -1;
        for num in nums {
            let ds = digits_sum(num) as usize;
            let x = map[ds];
            if x != 0 {
                ans = ans.max(x + num);
            }
            map[ds] = x.max(num);
        }
        ans
    }
}

#[inline(always)]
fn digits_sum(mut num: i32) -> i32 {
    let mut ans = 0;
    while num > 0 {
        ans += num % 10;
        num /= 10;
    }
    ans
}
