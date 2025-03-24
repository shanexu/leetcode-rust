package main

import (
	"fmt"
)

func main() {
	fmt.Println("hello")
}

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func getMinimumDifference(root *TreeNode) int {
	ans, _, _ := help(root)
	return ans
}

func help(node *TreeNode) (ans int, l *TreeNode, r *TreeNode) {
	ans = 100001
	if node.Left != nil {
		vl, ll, lr := help(node.Left)
		l = ll
		ans = min(ans, vl)
		ans = min(ans, node.Val-lr.Val)
	} else {
		l = node
	}
	if node.Right != nil {
		vr, rl, rr := help(node.Right)
		r = rr
		ans = min(ans, vr)
		ans = min(ans, rl.Val-node.Val)
	} else {
		r = node
	}
	return
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
