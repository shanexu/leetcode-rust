fn main() {
    println!("{:?}", Solution::get_final_state(vec![2, 1, 3, 5, 6], 5, 2));
    println!(
        "{:?}",
        Solution2::get_final_state(vec![2, 1, 3, 5, 6], 5, 2)
    );
    println!("{:?}", Solution::get_final_state(vec![1, 2], 3, 4));
    println!("{:?}", Solution2::get_final_state(vec![1, 2], 3, 4));
}

struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::with_capacity(nums.len());
        for (i, &v) in nums.iter().enumerate() {
            {
                heap.push(Reverse((v, i)));
            }
        }
        for _ in 0..k {
            if let Some(Reverse((v, i))) = heap.pop() {
                heap.push(Reverse((v * multiplier, i)));
            }
        }
        let mut result = vec![0; nums.len()];
        while let Some(Reverse((v, i))) = heap.pop() {
            result[i] = v;
        }
        result
    }
}

struct Solution2;
impl Solution2 {
    pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut heap: Vec<usize> = (0..nums.len()).collect();
        binary_heap_from(&mut heap, |a, b| {
            let av = nums[*a];
            let bv = nums[*b];
            if av > bv {
                return true;
            }
            if av < bv {
                return false;
            }
            return *a > *b;
        });
        for _ in 0..k {
            let i = heap[0];
            nums[i] *= multiplier;
            binary_heap_heapify(0, &mut heap, |a, b| {
                let av = nums[*a];
                let bv = nums[*b];
                if av < bv {
                    return true;
                }
                if av > bv {
                    return false;
                }
                return *a < *b;
            })
        }
        nums
    }
}

#[inline]
fn binary_heap_from<T, F>(xs: &mut Vec<T>, great_than: F)
where
    F: Fn(&T, &T) -> bool,
{
    for i in 0..xs.len() {
        let mut i = i;
        while i != 0 && great_than(&xs[binary_heap_parent(i)], &xs[i]) {
            xs.swap(i, binary_heap_parent(i));
            i = binary_heap_parent(i);
        }
    }
}

#[inline]
fn binary_heap_insert<T, F>(xs: &mut Vec<T>, x: T, great_than: F)
where
    F: Fn(&T, &T) -> bool,
{
    xs.push(x);
    let mut i = xs.len() - 1;
    while i != 0 && great_than(&xs[binary_heap_parent(i)], &xs[i]) {
        xs.swap(i, binary_heap_parent(i));
        i = binary_heap_parent(i);
    }
}

#[inline]
fn binary_heap_pop<T, F>(xs: &mut Vec<T>, less_than: F) -> Option<T>
where
    F: Fn(&T, &T) -> bool,
    T: Clone,
{
    let heap_size = xs.len();
    if heap_size == 1 {
        return xs.pop();
    }
    xs.swap(0, heap_size - 1);
    let root = xs.pop();
    binary_heap_heapify(0, xs, less_than);
    root
}

#[inline]
fn binary_heap_heapify<T, F>(mut i: usize, xs: &mut Vec<T>, less_than: F)
where
    F: Fn(&T, &T) -> bool,
{
    let heap_size = xs.len();
    loop {
        let mut s = i;
        let l = binary_heap_left(i);
        let r = binary_heap_right(i);
        if l < heap_size && less_than(&xs[l], &xs[s]) {
            s = l;
        }
        if r < heap_size && less_than(&xs[r], &xs[s]) {
            s = r;
        }
        if s != i {
            xs.swap(i, s);
            i = s;
        } else {
            break;
        }
    }
}

#[inline]
fn binary_heap_parent(i: usize) -> usize {
    (i - 1) / 2
}

#[inline]
fn binary_heap_left(i: usize) -> usize {
    2 * i + 1
}

#[inline]
fn binary_heap_right(i: usize) -> usize {
    2 * i + 2
}
