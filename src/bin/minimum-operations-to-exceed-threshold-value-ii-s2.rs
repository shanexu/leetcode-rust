fn main() {
    assert_eq!(2, Solution::min_operations(vec![2, 11, 10, 1, 3], 10));
    assert_eq!(
        2,
        Solution::min_operations(vec![999999999, 999999999, 999999999], 1000000000)
    );
}

struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut n = nums.len();
        binary_heap_from(&mut nums);
        let mut ans = 0;
        while nums[0] < k {
            ans += 1;
            let min = binary_heap_pop(&mut nums);
            let max = nums[0];
            nums[0] = min
                .checked_mul(2)
                .and_then(|x| max.checked_add(x))
                .unwrap_or(i32::MAX);
            n -= 1;
            bubble_down(&mut nums, 0, n);
        }
        ans
    }
}

fn binary_heap_from(xs: &mut Vec<i32>) {
    let n = xs.len();
    for i in (0..n / 2).rev() {
        bubble_down(xs, i, n);
    }
}

fn bubble_down(xs: &mut Vec<i32>, index: usize, heap_size: usize) {
    let current_val = xs[index];
    let mut current_index = index;

    loop {
        let left = 2 * current_index + 1;
        let right = 2 * current_index + 2;
        let mut smallest = current_index;
        let mut min_val = current_val;

        // 比较左子节点
        if left < heap_size && xs[left] < min_val {
            smallest = left;
            min_val = xs[left];
        }
        // 比较右子节点
        if right < heap_size && xs[right] < min_val {
            smallest = right;
            min_val = xs[right];
        }

        if smallest == current_index {
            break;
        }

        // 上移较小的子节点值
        xs[current_index] = min_val;
        current_index = smallest;
    }

    // 将保存的原始值放到最终位置
    xs[current_index] = current_val;
}

fn binary_heap_pop(xs: &mut Vec<i32>) -> i32 {
    let min_value = xs.swap_remove(0);
    bubble_down(xs, 0, xs.len());
    min_value
}
