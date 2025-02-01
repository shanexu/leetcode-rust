fn main() {
    println!(
        "{}",
        Solution::longest_mountain(vec![875, 884, 239, 731, 723, 685])
    );
    println!(
        "{}",
        Solution::longest_mountain(vec![0, 2, 0, 2, 1, 2, 3, 4, 4, 1])
    );
}

struct Solution;

impl Solution {
    pub fn longest_mountain(arr: Vec<i32>) -> i32 {
        if arr.len() < 3 {
            return 0;
        }
        let mut prev = arr[0];
        let mut state = 1; // 1 上升 -1 下降
        let mut ans = 0;
        let mut len = 1;
        for i in 1..arr.len() {
            let curr = arr[i];
            if state == 1 {
                if prev < curr {
                    len += 1;
                } else if prev == curr {
                    len = 1;
                } else {
                    if len > 1 {
                        state = -1;
                        len += 1;
                    }
                }
            } else {
                if prev > curr {
                    len += 1;
                } else if prev == curr {
                    ans = ans.max(len);
                    len = 1;
                    state = 1;
                } else {
                    ans = ans.max(len);
                    len = 2;
                    state = 1;
                }
            }
            prev = curr;
        }
        if state == -1 {
            ans = ans.max(len);
        }
        ans
    }
}
