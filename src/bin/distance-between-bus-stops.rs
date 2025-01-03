fn main() {
    println!(
        "{}",
        Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 1)
    );
}

struct Solution;

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        if start == destination {
            return 0;
        }
        let n = distance.len();
        let mut dist1 = 0;
        for i in 0..n {
            let k = (i + start as usize) % n;
            dist1 += distance[k];
            if (k + 1) % n == destination as usize {
                break;
            }
        }
        let mut dist2 = 0;
        for i in 0..n {
            let k = (i + destination as usize) % n;
            dist2 += distance[k];
            if (k + 1) % n == start as usize {
                break;
            }
        }
        dist1.min(dist2)
    }
}
