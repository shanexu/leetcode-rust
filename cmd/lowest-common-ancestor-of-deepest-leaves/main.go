package main

import (
	"fmt"
)

func main() {
	fmt.Println("Hello World")
	_ = lcaDeepestLeaves(nil)
}

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func lcaDeepestLeaves(root *TreeNode) *TreeNode {
	n, _ := travel(root, 0)
	return n
}

func travel(node *TreeNode, depth int) (*TreeNode, int) {
	if node == nil {
		return nil, depth - 1
	}
	l, dl := travel(node.Left, depth+1)
	r, dr := travel(node.Right, depth+1)
	if dl > dr {
		return l, dl
	} else if dl < dr {
		return r, dr
	} else {
		return node, dl
	}
}
