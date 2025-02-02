fn main() {
    assert_eq!(
        Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
        2
    );
    assert_eq!(
        Solution2::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
        2
    );
    assert_eq!(
        Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
        2
    );
    assert_eq!(
        Solution2::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
        2
    );
    assert_eq!(
        Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
        2
    );
    assert_eq!(
        Solution2::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
        2
    );
    assert_eq!(
        Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
        2
    );
    assert_eq!(
        Solution2::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
        2
    );
    assert_eq!(
        Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
        2
    );
    assert_eq!(
        Solution2::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
        2
    );
}

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let mut ans = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for &n1 in nums1.iter() {
            for &n2 in nums2.iter() {
                *map.entry(n1 + n2).or_insert(0) += 1;
            }
        }
        for &n3 in nums3.iter() {
            for &n4 in nums4.iter() {
                ans += *map.get(&(-n3 - n4)).unwrap_or(&0);
            }
        }
        ans
    }
}

struct Solution2;

impl Solution2 {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        fn freq(nums: &Vec<i32>) -> HashMap<i32, i32> {
            let mut map = HashMap::new();
            for &n in nums.iter() {
                *map.entry(n).or_insert(0) += 1;
            }
            map
        }

        fn freq_add(f1: &HashMap<i32, i32>, f2: &HashMap<i32, i32>) -> HashMap<i32, i32> {
            let mut map = HashMap::new();
            for (&v1, &c1) in f1 {
                for (&v2, &c2) in f2 {
                    *map.entry(v1 + v2).or_insert(0) += c1 * c2;
                }
            }
            map
        }

        let f1 = freq(&nums1);
        let f2 = freq(&nums2);
        let f3 = freq(&nums3);
        let f4 = freq(&nums4);

        let f12 = freq_add(&f1, &f2);
        let f34 = freq_add(&f3, &f4);

        let mut ans = 0;
        for (x, n) in f12.iter() {
            ans += *f34.get(&(-x)).unwrap_or(&0) * n;
        }
        ans
    }
}
