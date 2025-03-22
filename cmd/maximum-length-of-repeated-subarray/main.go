package main

import (
	"fmt"
)

func main() {
	fmt.Println(findLength([]int{1, 2, 3, 2, 1}, []int{3, 2, 1, 4, 7}))
}

func findLength(nums1 []int, nums2 []int) int {
	m := len(nums1)
	n := len(nums2)
	dp := make([][]int, m)
	ans := 0
	for i := range m {
		dp[i] = make([]int, n)
	}
	for i := range m {
		for j := range n {
			row := dp[i]
			if nums1[i] == nums2[j] {
				if i == 0 || j == 0 {
					row[j] = 1
				} else {
					row[j] = dp[i-1][j-1] + 1
				}
				if row[j] > ans {
					ans = dp[i][j]
				}
			}
		}
	}
	return ans
}
