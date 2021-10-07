fn main() {
    println!(
        "{:?}",
        Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]])
    );
}

struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        let mut results = vec![];
        binary_heap_from(&mut intervals);
        while intervals.len() > 1 {
            let x = binary_heap_pop(&mut intervals);
            let y = binary_heap_peek(&intervals);
            if x[1] < y[0] {
                results.push(x);
            } else {
                intervals[0] = vec![x[0], if x[1] > y[1] { x[1] } else { y[1] }];
            }
        }
        results.push(intervals.pop().unwrap());
        results
    }
}

fn binary_heap_from(xs: &mut Vec<Vec<i32>>) {
    let heap_size = xs.len();
    for i in 0..heap_size {
        let mut i = i;
        while i != 0 && great_than(&xs[binary_heap_parent(i)], &xs[i]) {
            xs.swap(i, binary_heap_parent(i));
            i = binary_heap_parent(i);
        }
    }
}

// fn binary_heap_insert(xs: &mut Vec<Vec<i32>>, x: Vec<i32>) {
//     xs.push(x);
//     let mut i = xs.len() - 1;
//     while i != 0 && great_than(&xs[binary_heap_parent(i)], &xs[i]) {
//         xs.swap(i, binary_heap_parent(i));
//         i = binary_heap_parent(i);
//     }
// }

fn binary_heap_peek(xs: &Vec<Vec<i32>>) -> &Vec<i32> {
    &xs[0]
}

fn binary_heap_pop(xs: &mut Vec<Vec<i32>>) -> Vec<i32> {
    let heap_size = xs.len();
    if heap_size == 1 {
        return xs.pop().unwrap();
    }
    xs.swap(0, heap_size - 1);
    let root = xs.pop().unwrap();
    binary_heap_heapify(0, xs);
    root
}

fn binary_heap_heapify(i: usize, xs: &mut Vec<Vec<i32>>) {
    let heap_size = xs.len();
    let l = binary_heap_left(i);
    let r = binary_heap_right(i);
    let mut smallest = i;
    if l < heap_size && less_than(&xs[l], &xs[i]) {
        smallest = l;
    }
    if r < heap_size && less_than(&xs[r], &xs[smallest]) {
        smallest = r;
    }
    if smallest != i {
        xs.swap(i, smallest);
        binary_heap_heapify(smallest, xs);
    }
}

fn great_than(x: &Vec<i32>, y: &Vec<i32>) -> bool {
    if x[0] > y[0] {
        true
    } else if x[0] == y[0] {
        x[1] > y[1]
    } else {
        false
    }
}

fn less_than(x: &Vec<i32>, y: &Vec<i32>) -> bool {
    if x[0] < y[0] {
        true
    } else if x[0] == y[0] {
        x[1] < y[1]
    } else {
        false
    }
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
