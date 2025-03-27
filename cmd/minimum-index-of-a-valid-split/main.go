package main

import (
	"fmt"
)

func main() {
	fmt.Println(minimumIndex([]int{1, 2, 2, 2}))
}

func minimumIndex(nums []int) int {
	x := find_x(nums)
	c := find_count(nums, x)
	left := 0
	n := len(nums)
	for i := 0; i < n-1; i++ {
		if nums[i] == x {
			left++
		}
		if left*2 > i+1 && (c-left)*2 > n-i-1 {
			return i
		}
	}
	return -1
}

func find_x(nums []int) int {
	candidate := 0
	votes := 0
	for _, num := range nums {
		if votes == 0 {
			candidate = num
		}
		if num == candidate {
			votes++
		} else {
			votes--
		}
	}
	return candidate
}

func find_count(nums []int, x int) int {
	count := 0
	for _, num := range nums {
		if num == x {
			count++
		}
	}
	return count
}
