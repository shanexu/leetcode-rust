package main

import (
	"fmt"
	"math"
	"sort"
)

func maximumScore(nums []int, k int) int {
	maxElement := getMax(nums)
	primes := getPrimes(maxElement)
	scores := getScores(nums, primes)
	n := len(nums)
	prev := make([]int, n)
	for i := range n {
		prev[i] = -1
	}
	next := make([]int, n)
	for i := range n {
		next[i] = n
	}
	var stack Stack
	for i := range n {
		for !stack.IsEmpty() && scores[stack.Peek()] < scores[i] {
			top := stack.Pop()
			next[top] = i
		}
		if !stack.IsEmpty() {
			prev[i] = stack.Peek()
		}
		stack.Push(i)
	}
	subarrays := make([]int, n)
	for i := range n {
		subarrays[i] = (next[i] - i) * (i - prev[i])
	}
	heap := make([][2]int, n)
	for i, num := range nums {
		heap[i][0] = num
		heap[i][1] = i
	}
	sort.Slice(heap, func(i, j int) bool {
		if heap[i][0] == heap[j][0] {
			return heap[i][1] < heap[j][1]
		}
		return heap[i][0] > heap[j][0]
	})
	i := 0
	ans := 1
	for k > 0 {
		top := heap[i]
		i++
		num, idx := top[0], top[1]
		ops := min(k, subarrays[idx])
		ans = (ans * power(num, ops)) % MOD
		k -= ops
	}
	return ans
}

func getMax(nums []int) int {
	ans := math.MinInt
	for _, num := range nums {
		ans = max(ans, num)
	}
	return ans
}

func getPrimes(n int) []int {
	primes := make([]bool, n+1)
	for i := range primes {
		primes[i] = true
	}
	p := 2
	for p*p <= n {
		if primes[p] {
			for i := p * p; i <= n; i += p {
				primes[i] = false
			}
		}
		p++
	}
	var ans []int
	for i := 2; i <= n; i++ {
		if primes[i] {
			ans = append(ans, i)
		}
	}
	return ans
}

func getScores(nums []int, primes []int) []int {
	scores := make([]int, len(nums))
	for i, num := range nums {
		for _, p := range primes {
			if p*p > num {
				break
			}
			if num%p != 0 {
				continue
			}
			scores[i]++
			for num%p == 0 {
				num /= p
			}
		}
		if num > 1 {
			scores[i]++
		}
	}
	return scores
}

// Stack
type Stack []int

func (s *Stack) IsEmpty() bool {
	return len(*s) == 0
}

func (s *Stack) Pop() int {
	n := len(*s)
	top := (*s)[n-1]
	*s = (*s)[:n-1]
	return top
}

func (s *Stack) Push(x int) {
	*s = append(*s, x)
}

func (s *Stack) Peek() int {
	n := len(*s)
	return (*s)[n-1]
}

const MOD = 1_000_000_007

func power(base int, exp int) int {
	res := 1
	for exp > 0 {
		if exp%2 == 1 {
			res = (res * base) % MOD
		}
		base = (base * base) % MOD
		exp /= 2
	}
	return res
}

func main() {
	fmt.Println(maximumScore([]int{19, 12, 14, 6, 10, 18}, 3))
}
