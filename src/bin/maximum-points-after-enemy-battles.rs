fn main() {
    println!("{}", Solution::maximum_points(vec![3, 2, 2], 2));
    println!("{}", Solution::maximum_points(vec![2], 10));
    println!("{}", Solution::maximum_points(vec![1, 2, 3], 0));
}

struct Solution;

impl Solution {
    pub fn maximum_points(enemy_energies: Vec<i32>, current_energy: i32) -> i64 {
        let mut total_energy = current_energy as i64;
        let mut min = i32::MAX;
        for energy in enemy_energies {
            total_energy += energy as i64;
            min = min.min(energy);
        }
        if current_energy < min {
            return 0;
        }
        (total_energy - min as i64) / min as i64
    }
}
