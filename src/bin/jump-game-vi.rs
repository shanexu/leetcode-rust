fn main() {
    assert_eq!(Solution::max_result(vec![1, -1, -2, 4, -7, 3], 2), 7);
    assert_eq!(Solution::max_result(vec![10, -5, -2, 4, 0, 3], 3), 17);
    assert_eq!(
        Solution::max_result(vec![1, -5, -20, 4, -1, 3, -6, -3], 2),
        0
    );
}

struct Solution;

impl Solution {
    // slow
    #[allow(dead_code)]
    pub fn max_result_slow(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k: usize = k as usize;
        let mut rs = vec![i32::MIN; n];
        rs[0] = nums[0];
        for i in 0..n {
            for j in (i + 1)..=(if i + k < n - 1 { i + k } else { n - 1 }) {
                if rs[i] + nums[j] > rs[j] {
                    rs[j] = rs[i] + nums[j];
                }
            }
        }
        rs[n - 1]
    }

    /*
    f(x) = nums[x] + max(f(x-1),...f(x-k))
    f(x-1) = nums[x-1] + max(f(x-2),...f(x-k-1))

    k = 3;
    f(0) = nums[0]
    f(1) = nums[0] + nums[1]
    f(2) = nums[2] + max(f(0), f(1))
    f(3) = nums[3] + max(f(2), f(1), f(0))
    f(4) = nums[4] + max(f(3), f(2), f(1))
    f(5) = nums[5] + max(f(4), f(3), f(2))
    f(6) = nums[6] + max(f(5), f(4), f(3))
    */
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let k: usize = k as usize;
        let n = nums.len();
        let mut xs = vec![];
        for (i, &x) in nums.iter().enumerate() {
            if i == 0 {
                if i == n - 1 {
                    return x;
                }
                binary_heap_insert(&mut xs, (x, i));
            } else {
                loop {
                    let (mx, idx) = binary_heap_peek(&xs);
                    if i < k || idx >= i - k {
                        if i == n - 1 {
                            return mx + x;
                        }
                        binary_heap_insert(&mut xs, (mx + x, i));
                        break;
                    } else {
                        binary_heap_pop(&mut xs);
                    }
                }
            }
        }
        0
    }
}

fn binary_heap_insert(xs: &mut Vec<(i32, usize)>, x: (i32, usize)) {
    xs.push(x);
    let mut i = xs.len() - 1;
    while i != 0 && xs[binary_heap_parent(i)].0 < xs[i].0 {
        xs.swap(i, binary_heap_parent(i));
        i = binary_heap_parent(i);
    }
}

fn binary_heap_pop(xs: &mut Vec<(i32, usize)>) -> (i32, usize) {
    let heap_size = xs.len();
    if heap_size == 1 {
        return xs.pop().unwrap();
    }
    xs.swap(0, heap_size - 1);
    let root = xs.pop().unwrap();
    binary_heap_heapify(xs, 0);
    root
}

fn binary_heap_heapify(xs: &mut Vec<(i32, usize)>, i: usize) {
    let heap_size = xs.len();
    let mut i = i;
    loop {
        let l = binary_heap_left(i);
        let r = binary_heap_right(i);
        let mut smallest = i;
        if l < heap_size && xs[l].0 > xs[i].0 {
            smallest = l;
        }
        if r < heap_size && xs[r].0 > xs[smallest].0 {
            smallest = r;
        }
        if smallest != i {
            xs.swap(i, smallest);
            i = smallest;
        } else {
            break;
        }
    }
}

fn binary_heap_peek(xs: &Vec<(i32, usize)>) -> (i32, usize) {
    xs[0]
}

fn binary_heap_parent(i: usize) -> usize {
    (i - 1) / 2
}

fn binary_heap_left(i: usize) -> usize {
    2 * i + 1
}

fn binary_heap_right(i: usize) -> usize {
    2 * i + 2
}
