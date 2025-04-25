package main

import "fmt"

func main() {
	fmt.Println(countCompleteSubarrays([]int{5, 5, 5, 5}))
}

func countCompleteSubarrays(nums []int) int {
	freq := make([]int, 2001)
	k := 0
	for _, num := range nums {
		freq[num] += 1
		if freq[num] == 1 {
			k++
		}
	}
	for i := range freq {
		freq[i] = 0
	}
	left := 0
	right := 0
	n := len(nums)
	s := 0
	ans := 0
	for right < n {
		numRight := nums[right]
		freq[numRight]++
		if freq[numRight] == 1 {
			s++
		}
		right++
		for s == k {
			ans += n - right + 1
			numLeft := nums[left]
			freq[numLeft]--
			if freq[numLeft] == 0 {
				s--
			}
			left++
		}
	}
	return ans
}
