fn main() {
    assert_eq!(1, Solution::find_content_children(vec![1,2,3], vec![1,1]));
}

struct Solution;

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable();
        s.sort_unstable();
        let mut ans = 0;
        let mut j = 0;
        let n = s.len();
        for x in g {
            while j < n && s[j] < x {
                j += 1;
            }
            if j == n {
                break;
            }
            j += 1;
            ans += 1;
        }
        ans
    }
}