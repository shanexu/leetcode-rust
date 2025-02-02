fn main() {
    println!(
        "{}",
        Solution2::continuous_subarrays(vec![65, 66, 67, 66, 66, 65, 64, 65, 65, 64])
    );
    println!(
        "{}",
        SlowSolution::continuous_subarrays(vec![65, 66, 67, 66, 66, 65, 64, 65, 65, 64])
    );
    println!(
        "{}",
        Solution::continuous_subarrays(vec![65, 66, 67, 66, 66, 65, 64, 65, 65, 64])
    );
}

struct SlowSolution;
impl SlowSolution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let mut count = 0;
        for i in 0..nums.len() {
            count += 1;
            let mut min = nums[i];
            let mut max = nums[i];
            for j in i + 1..nums.len() {
                min = std::cmp::min(nums[j], min);
                max = std::cmp::max(nums[j], max);
                if max - min > 2 {
                    break;
                }
                count += 1;
            }
        }
        count
    }
}

struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let mut count = 0;
        let mut min_heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::with_capacity(nums.len());
        let mut max_heap: BinaryHeap<(i32, Reverse<usize>)> = BinaryHeap::with_capacity(nums.len());
        min_heap.push(Reverse((nums[0], 0)));
        max_heap.push((nums[0], Reverse(0)));
        let mut i = 0;
        let mut j = i + 1;
        let mut min = nums[0];
        let mut max = nums[0];
        while j < nums.len() {
            let vj = nums[j];
            min_heap.push(Reverse((vj, j)));
            max_heap.push((vj, Reverse(j)));
            if vj < min {
                min = vj;
                if max - min > 2 {
                    // 排除最大值
                    let mut gate = i;
                    loop {
                        if let Some(&(v, Reverse(pos))) = max_heap.peek() {
                            if pos < gate {
                                max_heap.pop();
                                continue;
                            }
                            if v >= max {
                                gate = pos + 1;
                                max_heap.pop();
                                continue;
                            }
                            max = v;
                            count += (j - i) * (j - i + 1) / 2;
                            count -= (j - pos) * (j - pos + 1) / 2;
                            i = pos;
                            break;
                        } else {
                            break;
                        }
                    }
                }
            } else if vj > max {
                max = vj;
                if max - min > 2 {
                    // 排除最小值
                    let mut gate = i;
                    loop {
                        if let Some(&Reverse((v, pos))) = min_heap.peek() {
                            if pos < gate {
                                min_heap.pop();
                                continue;
                            }
                            if v <= min {
                                gate = pos + 1;
                                min_heap.pop();
                                continue;
                            }
                            min = v;
                            count += (j - i) * (j - i + 1) / 2;
                            count -= (j - pos) * (j - pos + 1) / 2;
                            i = pos;
                            break;
                        } else {
                            break;
                        }
                    }
                }
            } else {
                j += 1;
            }
        }
        count += (j - i) * (j - i + 1) / 2;
        count as i64
    }
}

/// 参考 <https://algo.monster/liteproblems/2762>
struct Solution2;
use std::collections::BTreeMap;
impl Solution2 {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let mut map: BTreeMap<i32, usize> = BTreeMap::new();
        let mut count: usize = 0;
        let mut win_start = 0;
        let mut win_end = 0;
        while win_end < nums.len() {
            *map.entry(nums[win_end]).or_insert(0) += 1;
            while *map.last_key_value().unwrap().0 - *map.first_key_value().unwrap().0 > 2 {
                let v = map.get_mut(&nums[win_start]).unwrap();
                *v -= 1;
                if *v == 0 {
                    map.remove(&nums[win_start]);
                }
                win_start += 1;
            }
            count += win_end - win_start + 1;
            win_end += 1;
        }
        count as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_continuous_subarrays1() {
        for (cn, val) in vec![
            ("case1", vec![65, 62]),
            ("case2", vec![65, 64, 63, 62]),
            ("case3", vec![65, 64, 63, 62, 62]),
            ("case4", vec![65, 65, 63, 62]),
            ("case5", vec![65, 65, 63, 62, 62]),
            ("case6", vec![65, 63, 62, 62, 60]),
            ("case7", vec![60, 60, 60, 60, 60]),
        ] {
            assert_eq!(
                Solution::continuous_subarrays(val.clone()),
                SlowSolution::continuous_subarrays(val.clone()),
                "{cn}"
            );
        }
    }

    #[test]
    fn test_continuous_subarrays2() {
        for (cn, val) in vec![
            ("case1", vec![62, 65]),
            ("case2", vec![62, 63, 64, 65]),
            ("case3", vec![62, 63, 64, 65, 65]),
            ("case4", vec![62, 62, 63, 64, 65]),
            ("case5", vec![62, 62, 63, 65, 65]),
            ("case6", vec![62, 64, 65, 65, 67]),
            ("case7", vec![67, 67, 67, 67, 67]),
        ] {
            assert_eq!(
                Solution::continuous_subarrays(val.clone()),
                SlowSolution::continuous_subarrays(val.clone()),
                "{cn}"
            );
        }
    }

    #[test]
    fn test_continuous_subarrays3() {
        for (cn, val) in vec![
            ("case0", vec![62, 65, 61]),
            ("case1", vec![65, 66, 67, 66, 66, 65, 64, 65, 65, 64]),
            ("case1", vec![66, 67, 66, 64]),
        ] {
            assert_eq!(
                Solution::continuous_subarrays(val.clone()),
                SlowSolution::continuous_subarrays(val.clone()),
                "{cn}"
            )
        }
    }
}
