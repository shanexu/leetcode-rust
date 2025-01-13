fn main() {
    println!("{}", Solution::maximum_gap(vec![3, 6, 9, 1]));
}

struct Solution;

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for &num in nums.iter() {
            min = min.min(num);
            max = max.max(num);
        }
        let n = nums.len();
        let bucket_size = std::cmp::max(1, (max - min) / (n - 1) as i32);
        let bucket_count = ((max - min) / bucket_size + 1) as usize;
        let mut buckets = vec![(i32::MAX, i32::MIN); bucket_count];

        for &num in nums.iter() {
            let idx = ((num - min) / bucket_size) as usize;
            buckets[idx].0 = buckets[idx].0.min(num);
            buckets[idx].1 = buckets[idx].1.max(num);
        }

        let mut prev_max = i32::MAX;
        let mut ans = 0;
        for bucket in buckets.iter() {
            if bucket.0 > bucket.1 {
                continue;
            }
            ans = ans.max(bucket.0 - prev_max);
            prev_max = bucket.1;
        }
        ans
    }
}
