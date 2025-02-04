fn main() {
    assert_eq!(Solution::is_power_of_three(27), true);
    assert_eq!(Solution::is_power_of_three(0), false);
    assert_eq!(Solution::is_power_of_three(9), true);
    assert_eq!(Solution::is_power_of_three(45), false);
    assert_eq!(Solution::is_power_of_three(1), true);
    assert_eq!(Solution::is_power_of_three(3), true);
    assert_eq!(Solution::is_power_of_three(243), true);
    assert_eq!(Solution::is_power_of_three(19683), true);
    assert_eq!(Solution::is_power_of_three(1162261467), true);
    assert_eq!(Solution::is_power_of_three(1162261468), false);
    assert_eq!(Solution::is_power_of_three(1162261466), false);
    assert_eq!(Solution::is_power_of_three(1162261465), false);
    assert_eq!(Solution::is_power_of_three(1162261464), false);
    assert_eq!(Solution::is_power_of_three(1162261463), false);
    assert_eq!(Solution::is_power_of_three(1162261462), false);
    assert_eq!(Solution::is_power_of_three(1162261461), false);
    assert_eq!(Solution::is_power_of_three(1162261460), false);
    assert_eq!(Solution::is_power_of_three(1162261459), false);
    assert_eq!(Solution::is_power_of_three(1162261458), false);
    assert_eq!(Solution::is_power_of_three(1162261457), false);
    assert_eq!(Solution::is_power_of_three(1162261456), false);
    assert_eq!(Solution::is_power_of_three(1162261455), false);
    assert_eq!(Solution::is_power_of_three(1162261454), false);
    assert_eq!(Solution::is_power_of_three(1162261453), false);
    assert_eq!(Solution::is_power_of_three(1162261452), false);
    assert_eq!(Solution::is_power_of_three(1162261451), false);
    assert_eq!(Solution::is_power_of_three(1162261450), false);
    assert_eq!(Solution::is_power_of_three(1162261449), false);
    assert_eq!(Solution::is_power_of_three(1162261448), false);

    assert_eq!(Solution2::is_power_of_three(27), true);
    assert_eq!(Solution2::is_power_of_three(0), false);
    assert_eq!(Solution2::is_power_of_three(9), true);
    assert_eq!(Solution2::is_power_of_three(45), false);
    assert_eq!(Solution2::is_power_of_three(1), true);
    assert_eq!(Solution2::is_power_of_three(3), true);
    assert_eq!(Solution2::is_power_of_three(243), true);
    assert_eq!(Solution2::is_power_of_three(19683), true);
    assert_eq!(Solution2::is_power_of_three(1162261467), true);
    assert_eq!(Solution2::is_power_of_three(1162261468), false);
    assert_eq!(Solution2::is_power_of_three(1162261466), false);
    assert_eq!(Solution2::is_power_of_three(1162261465), false);
    assert_eq!(Solution2::is_power_of_three(1162261464), false);
    assert_eq!(Solution2::is_power_of_three(1162261463), false);
    assert_eq!(Solution2::is_power_of_three(1162261462), false);
    assert_eq!(Solution2::is_power_of_three(1162261461), false);
    assert_eq!(Solution2::is_power_of_three(1162261460), false);
    assert_eq!(Solution2::is_power_of_three(1162261459), false);
    assert_eq!(Solution2::is_power_of_three(1162261458), false);
    assert_eq!(Solution2::is_power_of_three(1162261457), false);
    assert_eq!(Solution2::is_power_of_three(1162261456), false);
    assert_eq!(Solution2::is_power_of_three(1162261455), false);
    assert_eq!(Solution2::is_power_of_three(1162261454), false);
    assert_eq!(Solution2::is_power_of_three(1162261453), false);
    assert_eq!(Solution2::is_power_of_three(1162261452), false);
    assert_eq!(Solution2::is_power_of_three(1162261451), false);
    assert_eq!(Solution2::is_power_of_three(1162261450), false);
    assert_eq!(Solution2::is_power_of_three(1162261449), false);
    assert_eq!(Solution2::is_power_of_three(1162261448), false);
}

struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        POWER_OF_THREE.binary_search(&n).is_ok()
    }
}

const POWER_OF_THREE: [i32; 20] = [
    1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 177147, 531441, 1594323, 4782969,
    14348907, 43046721, 129140163, 387420489, 1162261467,
];

struct Solution2;

impl Solution2 {
    pub fn is_power_of_three(n: i32) -> bool {
        n > 0 && (1162261467 / n * n) == 1162261467
    }
}
