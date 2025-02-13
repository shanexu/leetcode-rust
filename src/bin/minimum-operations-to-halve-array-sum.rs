fn main() {
    println!("{}", Solution::halve_array(vec![5, 19, 8, 1]));
}

struct Solution;

impl Solution {
    pub fn halve_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0.0;
        let mut heap = Vec::with_capacity(nums.len());
        for num in nums {
            let num = num as f64;
            sum += num;
            heap.push(num);
        }
        binary_heap_from(&mut heap);
        let n = heap.len();
        let mut ans = 0;
        let half_sum = sum / 2.0;
        while sum > half_sum {
            ans += 1;
            let half = heap[0] / 2.0;
            sum -= half;
            heap[0] = half;
            bubble_down(&mut heap, 0, n);
        }
        ans
    }
}

fn binary_heap_from(xs: &mut Vec<f64>) {
    let n = xs.len();
    for i in (0..n / 2).rev() {
        bubble_down(xs, i, n);
    }
}

fn bubble_down(xs: &mut Vec<f64>, index: usize, heap_size: usize) {
    let current_val = xs[index];
    let mut current_index = index;

    loop {
        let left = 2 * current_index + 1;
        let right = 2 * current_index + 2;
        let mut smallest = current_index;
        let mut min_val = current_val;

        // 比较左子节点
        if left < heap_size && xs[left] > min_val {
            smallest = left;
            min_val = xs[left];
        }
        // 比较右子节点
        if right < heap_size && xs[right] > min_val {
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
