fn main() {
    println!("{}", Solution::search(vec![1, 2, 3, 4, 5, 6, 0], 2));
    println!("{}", Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
    println!("{}", Solution::search(vec![3, 1, 2], 2));
}

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 1 {
            return if nums[0] == target { 0 } else { -1 };
        }
        let k = find_k(&nums, 0, nums.len() - 1);
        let (l, r) = nums.split_at(k);
        l.binary_search(&target)
            .or_else(|_| r.binary_search(&target).map(|i| i + k))
            .map(|i| i as i32)
            .unwrap_or(-1)
    }
}

fn find_k(nums: &Vec<i32>, begin_idx: usize, end_idx: usize) -> usize {
    if is_increasing(nums, begin_idx, end_idx) {
        return begin_idx;
    }
    let k = (begin_idx + end_idx + 1) / 2;
    let l = is_increasing(nums, begin_idx, k - 1);
    let r = is_increasing(nums, k, end_idx);
    if l && r {
        return k;
    } else if !r {
        return find_k(nums, k, end_idx);
    } else {
        return find_k(nums, begin_idx, k - 1);
    }
}

fn is_increasing(nums: &Vec<i32>, begin_idx: usize, end_idx: usize) -> bool {
    if begin_idx == end_idx {
        true
    } else {
        nums[begin_idx] < nums[end_idx]
    }
}
