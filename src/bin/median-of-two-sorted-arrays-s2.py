#!/usr/bin/env python3

from typing import List


class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        max_value = 10e6 + 1
        min_value = -10e6 - 1
        if len(nums1) > len(nums2):
            nums1, nums2 = nums2, nums1
        m = len(nums1)
        n = len(nums2)
        half = (m + n + 1) // 2
        left, right = 0, m
        while left <= right:
            pa = (left + right) // 2
            pb = half - pa
            max_left_a = min_value
            if pa != 0:
                max_left_a = nums1[pa - 1]
            min_right_a = max_value
            if pa != m:
                min_right_a = nums1[pa]
            max_left_b = min_value
            if pb != 0:
                max_left_b = nums2[pb - 1]
            min_right_b = max_value
            if pb != n:
                min_right_b = nums2[pb]
            if max_left_a <= min_right_b and max_left_b <= min_right_a:
                if (m + n) % 2 == 1:
                    return float(max(max_left_a, max_left_b))
                else:
                    return (max(max_left_a, max_left_b) + min(min_right_a, min_right_b)) / 2
            elif max_left_a > min_right_b:
                right = pa - 1
            else:
                left = pa + 1

        return 0.0



if __name__ == '__main__':
    s = Solution()
    print(s.findMedianSortedArrays([1,2], [3]))
    print(s.findMedianSortedArrays([1,2], [3, 4]))
