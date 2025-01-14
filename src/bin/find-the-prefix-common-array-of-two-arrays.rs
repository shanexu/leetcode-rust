fn main() {}

struct Solution;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut ans = vec![0; n];
        let mut fa: u64 = 0;
        let mut fb: u64 = 0;

        for i in 0..n {
            let ia = a[i] as usize;
            let ib = b[i] as usize;
            fa |= 1 << ia;
            ans[i] += ((fb >> ia) & 1) as i32;
            fb |= 1 << ib;
            ans[i] += ((fa >> ib) & 1) as i32;
            if i < n - 1 {
                ans[i + 1] = ans[i];
            }
        }

        ans
    }
}
