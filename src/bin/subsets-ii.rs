fn main() {
    println!("{:?}", Solution::subsets_with_dup(vec![4, 4, 4, 1, 4]));
}

struct Solution;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut freq: Vec<usize> = vec![1; 21];
        for num in nums {
            freq[(num + 10) as usize] += 1;
        }
        let total = freq.iter().product();
        for mut i in 0..total {
            let mut s = vec![];
            let mut c = 0usize;
            while i > 0 {
                let f = freq[c];
                if f != 1 {
                    let r = i % f;
                    s.extend(vec![c as i32 - 10; r]);
                    i /= f;
                }
                c += 1;
            }
            ans.push(s);
        }
        ans
    }
}
