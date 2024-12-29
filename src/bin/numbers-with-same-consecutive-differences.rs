
fn main() {
    // println!("{:?}", );
    let mut actual = Solution::nums_same_consec_diff(4, 1);
    actual.sort();
    let mut expected = vec![1010,1012,1210,1212,1232,1234,2101,2121,2123,2321,2323,2343,2345,3210,3212,3232,3234,3432,3434,3454,3456,4321,4323,4343,4345,4543,4545,4565,4567,5432,5434,5454,5456,5654,5656,5676,5678,6543,6545,6565,6567,6765,6767,6787,6789,7654,7656,7676,7678,7876,7878,7898,8765,8767,8787,8789,8987,8989,9876,9878,9898];
    expected.sort();

    println!("{}", actual.len());
    println!("{}", expected.len());

}

struct Solution;

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut ans = vec![];
        for i in 1..=9 {
            if i - k >= 0 {
                ans.push(i * 10 + i - k);
            }
            if k != 0 && i + k < 10 {
                ans.push(i * 10 + i + k);
            }
        }
        for _ in 3..=n {
            for i in 0..ans.len() {
                let a = ans[i];
                let d = a % 10;
                if d - k >= 0 {
                    ans[i] = a * 10 + d - k;
                    if k != 0 && d + k < 10 {
                        ans.push(a * 10 + d + k);
                    }
                } else {
                    if d + k < 10 {
                        ans[i] = a * 10 + d + k;
                    }
                }
            }
        }
        ans
    }
}