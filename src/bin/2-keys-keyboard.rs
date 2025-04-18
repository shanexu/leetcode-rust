fn main() {
    let mut memo = vec![0; 1001];
    for i in 1..=1000 {
        memo[i as usize] = Solution::min_steps(i);
    }
    println!("{:?}", memo);
    let mut memo = vec![0; 1001];
    for i in 1..=1000 {
        memo[i as usize] = Solution2::min_steps(i);
    }
    println!("{:?}", memo);
}

struct Solution;

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut memo = vec![0; 1001];
        fn helper(n: i32, memo: &mut Vec<i32>) -> i32 {
            if n == 1 {
                return 0;
            }
            if memo[n as usize] != 0 {
                return memo[n as usize];
            }
            let mut ans = n;
            let root = (n as f64).sqrt() as i32;
            for i in (2..=root).rev() {
                if n % i == 0 {
                    let k = n / i;
                    ans = helper(i, memo) + helper(k, memo);
                    break;
                    // ans = ans.min(helper(i, memo) + k);
                    // if i != k {
                    //     ans = ans.min(helper(k, memo) + i);
                    // }
                }
            }
            memo[n as usize] = ans;
            ans
        }
        helper(n, &mut memo)
    }
}

struct Solution2;

impl Solution2 {
    pub fn min_steps(n: i32) -> i32 {
        const MEMO: &[i32] = &[
            0, 0, 2, 3, 4, 5, 5, 7, 6, 6, 7, 11, 7, 13, 9, 8, 8, 17, 8, 19, 9, 10, 13, 23, 9, 10,
            15, 9, 11, 29, 10, 31, 10, 14, 19, 12, 10, 37, 21, 16, 11, 41, 12, 43, 15, 11, 25, 47,
            11, 14, 12, 20, 17, 53, 11, 16, 13, 22, 31, 59, 12, 61, 33, 13, 12, 18, 16, 67, 21, 26,
            14, 71, 12, 73, 39, 13, 23, 18, 18, 79, 13, 12, 43, 83, 14, 22, 45, 32, 17, 89, 13, 20,
            27, 34, 49, 24, 13, 97, 16, 17, 14, 101, 22, 103, 19, 15, 55, 107, 13, 109, 18, 40, 15,
            113, 24, 28, 33, 19, 61, 24, 14, 22, 63, 44, 35, 15, 15, 127, 14, 46, 20, 131, 18, 26,
            69, 14, 23, 137, 28, 139, 16, 50, 73, 24, 14, 34, 75, 17, 41, 149, 15, 151, 25, 23, 20,
            36, 20, 157, 81, 56, 15, 30, 14, 163, 45, 19, 85, 167, 16, 26, 24, 25, 47, 173, 34, 17,
            19, 62, 91, 179, 15, 181, 22, 64, 29, 42, 36, 28, 51, 16, 26, 191, 15, 193, 99, 21, 18,
            197, 19, 199, 16, 70, 103, 36, 24, 46, 105, 29, 21, 30, 17, 211, 57, 74, 109, 48, 15,
            38, 111, 76, 20, 30, 42, 223, 17, 16, 115, 227, 26, 229, 30, 21, 35, 233, 21, 52, 63,
            82, 26, 239, 16, 241, 24, 15, 65, 19, 46, 32, 37, 86, 17, 251, 17, 34, 129, 25, 16,
            257, 48, 44, 22, 35, 133, 263, 20, 58, 28, 92, 71, 269, 16, 271, 25, 23, 139, 21, 30,
            277, 141, 37, 18, 281, 52, 283, 75, 27, 26, 48, 16, 34, 36, 100, 77, 293, 19, 64, 43,
            20, 151, 36, 17, 50, 153, 104, 27, 66, 25, 307, 22, 106, 38, 311, 22, 313, 159, 18, 83,
            317, 58, 40, 17, 110, 32, 36, 16, 23, 165, 112, 47, 54, 21, 331, 87, 43, 169, 72, 18,
            337, 28, 116, 26, 42, 27, 21, 49, 31, 175, 347, 36, 349, 19, 22, 21, 353, 64, 76, 93,
            27, 181, 359, 17, 38, 183, 25, 24, 78, 66, 367, 31, 47, 44, 60, 38, 373, 30, 18, 53,
            42, 18, 379, 28, 130, 193, 383, 17, 23, 195, 49, 101, 389, 23, 40, 20, 134, 199, 84,
            21, 397, 201, 29, 18, 401, 72, 44, 105, 17, 38, 48, 26, 409, 48, 140, 107, 66, 31, 88,
            23, 142, 32, 419, 19, 421, 213, 53, 59, 27, 76, 68, 111, 27, 50, 431, 17, 433, 40, 37,
            113, 42, 78, 439, 22, 20, 32, 443, 44, 94, 225, 152, 19, 449, 18, 52, 117, 154, 229,
            25, 28, 457, 231, 26, 32, 461, 23, 463, 37, 39, 235, 467, 23, 74, 54, 160, 65, 54, 84,
            29, 28, 59, 241, 479, 18, 50, 243, 33, 26, 102, 17, 487, 67, 166, 21, 491, 48, 46, 34,
            22, 39, 78, 88, 499, 19, 170, 253, 503, 19, 106, 36, 29, 131, 509, 27, 80, 18, 28, 259,
            108, 50, 58, 46, 176, 24, 521, 37, 523, 135, 20, 265, 48, 22, 46, 60, 65, 30, 54, 94,
            112, 73, 182, 271, 25, 18, 541, 273, 184, 27, 114, 25, 547, 141, 67, 23, 48, 32, 86,
            279, 45, 143, 557, 39, 56, 20, 31, 283, 563, 54, 118, 285, 19, 77, 569, 29, 571, 28,
            194, 50, 33, 18, 577, 36, 196, 38, 90, 102, 64, 79, 24, 295, 587, 21, 50, 66, 200, 45,
            593, 22, 29, 153, 202, 38, 599, 19, 601, 52, 73, 155, 27, 106, 607, 29, 39, 68, 60, 27,
            613, 309, 49, 24, 617, 108, 619, 40, 32, 313, 96, 24, 20, 315, 33, 161, 54, 20, 631,
            85, 214, 319, 132, 60, 27, 42, 77, 19, 641, 112, 643, 34, 51, 38, 647, 18, 70, 25, 41,
            167, 653, 114, 136, 49, 79, 56, 659, 23, 661, 333, 33, 89, 31, 45, 52, 171, 226, 74,
            72, 20, 673, 339, 19, 30, 677, 118, 104, 28, 230, 44, 683, 29, 142, 23, 232, 51, 66,
            33, 691, 177, 24, 349, 144, 38, 58, 351, 236, 21, 701, 24, 56, 23, 55, 355, 108, 66,
            709, 78, 85, 95, 54, 29, 29, 183, 242, 361, 719, 19, 110, 40, 244, 185, 39, 27, 727,
            26, 18, 80, 60, 68, 733, 369, 22, 33, 78, 49, 739, 46, 35, 62, 743, 40, 154, 375, 89,
            32, 114, 20, 751, 55, 254, 44, 156, 20, 757, 381, 37, 30, 761, 132, 116, 195, 28, 385,
            72, 19, 769, 25, 260, 197, 773, 51, 41, 103, 47, 391, 60, 25, 82, 42, 38, 22, 162, 136,
            787, 201, 266, 86, 120, 23, 74, 399, 61, 203, 797, 31, 64, 20, 95, 403, 84, 74, 35, 46,
            272, 107, 809, 19, 811, 40, 274, 50, 168, 28, 62, 411, 26, 50, 821, 142, 823, 109, 24,
            68, 827, 33, 829, 90, 280, 25, 31, 144, 172, 34, 40, 421, 839, 21, 58, 423, 284, 215,
            31, 55, 29, 61, 286, 29, 60, 78, 853, 70, 30, 113, 857, 29, 859, 52, 51, 433, 863, 19,
            178, 435, 37, 42, 90, 39, 80, 115, 103, 44, 22, 80, 877, 441, 296, 24, 881, 22, 883,
            34, 67, 445, 887, 46, 134, 96, 23, 227, 66, 154, 184, 21, 39, 451, 60, 20, 70, 54, 53,
            119, 186, 156, 907, 231, 107, 27, 911, 30, 94, 459, 69, 233, 138, 28, 919, 34, 310,
            463, 84, 25, 47, 465, 109, 39, 929, 41, 33, 237, 314, 469, 33, 25, 937, 76, 316, 56,
            941, 162, 64, 67, 21, 56, 947, 86, 86, 31, 320, 30, 953, 61, 196, 243, 43, 481, 144,
            20, 62, 52, 113, 245, 198, 35, 967, 28, 39, 104, 971, 19, 146, 489, 26, 69, 977, 168,
            100, 23, 115, 493, 983, 50, 202, 48, 57, 36, 66, 24, 991, 41, 334, 80, 204, 90, 997,
            501, 46, 21,
        ];
        MEMO[n as usize]
    }
}
