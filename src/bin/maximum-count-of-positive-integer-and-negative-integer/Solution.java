class Solution {
    public int maximumCount(int[] nums) {
        int posZero = -binarySearch(nums, 0, nums.length, 1) - 1;
        int negZero = -binarySearch(nums, 0, posZero, -1) - 1;
        return Math.max(nums.length - posZero, negZero);
    }

    private static int binarySearch(int[] a, int from, int to, int zero) {
        int low = from;
        int high = to - 1;

        while (low <= high) {
            int mid = (low + high) >>> 1;
            int midVal = a[mid];
            int c = (midVal << 1) - zero;
            if (c < 0) {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        return -(low + 1);  // key not found.
    }
}
