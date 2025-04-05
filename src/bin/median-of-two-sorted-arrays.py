#!/usr/bin/env python3

from typing import List


class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        m = len(nums1)
        n = len(nums2)
        k, i, j = 0, 0, 0
        prev, curr = 0, 0
        mid = (m + n) // 2
        while k <= mid:
            if i < m and j < n:
                prev = curr
                k += 1
                if nums1[i] < nums2[j]:
                    curr = nums1[i]
                    i+=1
                else:
                    curr = nums2[j]
                    j+=1
            else:
                break

        if i < m:
            while k <= mid:
                prev = curr
                k += 1
                curr = nums1[i]
                i+=1

        if j < n:
            while k <= mid:
                prev = curr
                k += 1
                curr = nums2[j]
                j += 1
        if (m + n) % 2 == 1:
            return float(curr)
        return (curr + prev) / 2


if __name__ == '__main__':
    s = Solution()
    print(s.findMedianSortedArrays([1,2], [3]))
    print(s.findMedianSortedArrays([1,2], [3, 4]))
