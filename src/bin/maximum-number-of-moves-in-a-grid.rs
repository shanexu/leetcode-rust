fn main() {
    println!(
        "{}",
        Solution::max_moves(vec![
            vec![
                1000000, 92910, 1021, 1022, 1023, 1024, 1025, 1026, 1027, 1028, 1029, 1030, 1031,
                1032, 1033, 1034, 1035, 1036, 1037, 1038, 1039, 1040, 1041, 1042, 1043, 1044, 1045,
                1046, 1047, 1048, 1049, 1050, 1051, 1052, 1053, 1054, 1055, 1056, 1057, 1058, 1059,
                1060, 1061, 1062, 1063, 1064, 1065, 1066, 1067, 1068
            ],
            vec![
                1069, 1070, 1071, 1072, 1073, 1074, 1075, 1076, 1077, 1078, 1079, 1080, 1081, 1082,
                1083, 1084, 1085, 1086, 1087, 1088, 1089, 1090, 1091, 1092, 1093, 1094, 1095, 1096,
                1097, 1098, 1099, 1100, 1101, 1102, 1103, 1104, 1105, 1106, 1107, 1108, 1109, 1110,
                1111, 1112, 1113, 1114, 1115, 1116, 1117, 1118
            ]
        ])
    );
}

struct Solution;

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = vec![vec![0; n]; m];
        let max = (n - 1) as i32;
        for j in (0..n - 1).rev() {
            for i in 0..m {
                let mut t = 0;
                if i > 0 {
                    if grid[i][j] < grid[i - 1][j + 1] {
                        t = t.max(ans[i - 1][j + 1]);
                        ans[i][j] = t + 1;
                    }
                }
                if grid[i][j] < grid[i][j + 1] {
                    t = t.max(ans[i][j + 1]);
                    ans[i][j] = t + 1;
                }
                if i + 1 < m {
                    if grid[i][j] < grid[i + 1][j + 1] {
                        t = t.max(ans[i + 1][j + 1]);
                        ans[i][j] = t + 1;
                    }
                }
            }
        }
        let mut result = 0;
        for i in (0..m).rev() {
            result = result.max(ans[i][0]);
            if result == max {
                return max
            }
        }
        result
    }
}
