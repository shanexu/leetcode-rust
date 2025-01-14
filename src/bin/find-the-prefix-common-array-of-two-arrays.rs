fn main() {}

struct Solution;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut ans = vec![0; n];
        let mut fa = vec![0; n + 1];
        let mut fb = vec![0; n + 1];

        for i in 0..n {
            let ia = a[i] as usize;
            let ib = b[i] as usize;
            fa[ia] = 1;
            ans[i] += fb[ia];
            fb[ib] = 1;
            ans[i] += fa[ib];
            if i < n - 1 {
                ans[i + 1] = ans[i];
            }
        }

        ans
    }
}
