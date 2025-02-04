fn main() {
    println!("{}", Solution {}.first_bad_version(2147483647));
}

struct Solution;

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left: i64 = 1;
        let mut right: i64 = n as i64;
        let mut v: i64 = right + 1; // Initialize v to an out-of-bounds value

        while left <= right {
            // Calculate mid in one place
            let mid = left + (right - left) / 2;

            if self.isBadVersion(mid as i32) {
                v = v.min(mid); // Update the first bad version
                right = mid - 1; // Move the right pointer down
            } else {
                left = mid + 1; // Move the left pointer up
            }
        }
        v as i32 // Return the found first bad version
    }

    #[allow(non_snake_case)]
    fn isBadVersion(&self, version: i32) -> bool {
        // todo!()
        version >= 3
    }
}
