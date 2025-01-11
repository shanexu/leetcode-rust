fn main() {
    println!("{:?}", Solution::subsets_with_dup(vec![4, 4, 4, 1, 4]));
}

struct Solution;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut map = std::collections::BTreeMap::new();
        let mut total = 1;
        for num in nums {
            let v = map.entry(num).and_modify(|x| *x += 1).or_insert(1);
            if *v >= 1 {
                total = total / *v * (*v + 1)
            }
        }
        let pairs: Vec<(i32, usize)> = map.iter().map(|(k, v)| (*k, *v)).collect();
        for mut i in 0..total {
            let mut s = vec![];
            let mut c = 0usize;
            while i > 0 {
                let (k, v) = pairs[c];
                let r = i % (v + 1);
                s.extend(vec![k; r]);
                i /= v + 1;
                c += 1;
            }
            ans.push(s);
        }
        ans
    }
}
