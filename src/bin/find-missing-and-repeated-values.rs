fn main() {
    println!(
        "{:?}",
        Solution::find_missing_and_repeated_values(vec![
            vec![1, 2, 2],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ])
    );
}

struct Solution;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        #[inline]
        fn value_to_index1(x: i32, n: usize) -> usize {
            let x = x as usize;
            let mut k = x / n;
            if k * n == x {
                k -= 1;
            }
            k
        }

        #[inline]
        fn value_to_index2(x: i32, n: usize) -> usize {
            let x = x as usize;
            let k = x - x / n * n;
            if k == 0 {
                n - 1
            } else {
                k - 1
            }
        }

        #[inline]
        fn coordinate_to_value1(i: usize, j: usize, n: usize) -> i32 {
            (i * n + j + 1) as i32
        }

        #[inline]
        fn coordinate_to_value2(i: usize, j: usize, n: usize) -> i32 {
            (j * n + i + 1) as i32
        }

        #[inline]
        fn calculate_check_sums(
            grid: &Vec<Vec<i32>>,
            n: usize,
            v2i: fn(i32, usize) -> usize,
            ij2v: fn(usize, usize, usize) -> i32,
            sums: &mut Vec<i32>,
            std_sums: &mut Vec<i32>,
        ) {
            for i in 0..n {
                for j in 0..n {
                    let x = grid[i][j];
                    sums[v2i(x, n)] += x;
                    std_sums[i] += ij2v(i, j, n);
                }
            }
        }

        #[inline]
        fn check_sums(sums: &Vec<i32>, std_sums: &Vec<i32>) -> Option<Vec<i32>> {
            let mut missing = 0;
            let mut repeated = 0;
            for (&sum, &std_sum) in std::iter::zip(sums, std_sums) {
                if sum > std_sum {
                    repeated = sum - std_sum;
                } else if sum < std_sum {
                    missing = std_sum - sum;
                }
            }
            if missing == 0 || repeated == 0 {
                None
            } else {
                Some(vec![repeated, missing])
            }
        }

        let n = grid.len();
        let mut sums = vec![0; n];
        let mut std_sums = vec![0; n];

        calculate_check_sums(
            &grid,
            n,
            value_to_index1,
            coordinate_to_value1,
            &mut sums,
            &mut std_sums,
        );

        if let Some(v) = check_sums(&sums, &std_sums) {
            return v;
        }

        sums.fill(0);
        std_sums.fill(0);

        calculate_check_sums(
            &grid,
            n,
            value_to_index2,
            coordinate_to_value2,
            &mut sums,
            &mut std_sums,
        );

        check_sums(&sums, &std_sums).unwrap()
    }
}
