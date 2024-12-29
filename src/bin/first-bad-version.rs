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
        let mut size: i64 = n as i64;
        let mut mid = 0;
        let mut v: i64 = size + 1;
        while size > 0 {
            let k = size / 2;
            mid = left + k;
            if self.isBadVersion(mid as i32) {
                v = v.min(mid);
                mid = size / 2;
            } else {
                left = mid + 1;
            }
            size = k;
        }
        v as i32
    }

    fn isBadVersion(&self, version: i32) -> bool {
        // todo!()
        version >= 3
    }
}
