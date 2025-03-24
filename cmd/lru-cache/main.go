package main

import (
	"fmt"
)

func main() {
	list := NewDoublyLinkedList()
	for i := range 10 {
		list.PushBack(&Node{Val: i})
	}
	PrintList(list)
	PrintListReverse(list)

	list = NewDoublyLinkedList()
	for i := range 10 {
		list.PushFront(&Node{Val: i})
	}
	PrintList(list)
	PrintListReverse(list)
	cache := Constructor(2)
	cache.Put(1, 1)
	cache.Put(2, 2)
	fmt.Println(cache.Get(1))
	cache.Put(3, 3)
	fmt.Println(cache.Get(2))
	cache.Put(4, 4)
	fmt.Println(cache.Get(1))
	fmt.Println(cache.Get(3))
	fmt.Println(cache.Get(4))
}

type DoublyLinkedList struct {
	Head *Node
	Tail *Node
}

func NewDoublyLinkedList() *DoublyLinkedList {
	head := &Node{}
	tail := &Node{}
	head.Next = tail
	tail.Prev = head
	return &DoublyLinkedList{
		Head: head,
		Tail: tail,
	}
}

type Node struct {
	Key  int
	Val  int
	Prev *Node
	Next *Node
}

func DeleteNode(node *Node) {
	prev := node.Prev
	next := node.Next
	prev.Next = next
	next.Prev = prev
}

func AddBeforeNode(tail *Node, node *Node) {
	prev := tail.Prev
	prev.Next = node
	node.Next = tail
	tail.Prev = node
	node.Prev = prev
}

func AddAfterNode(head *Node, node *Node) {
	next := head.Next
	next.Prev = node
	node.Prev = head
	head.Next = node
	node.Next = next
}

func (list *DoublyLinkedList) PushBack(node *Node) {
	AddBeforeNode(list.Tail, node)
}

func (list *DoublyLinkedList) PushFront(node *Node) {
	AddAfterNode(list.Head, node)
}

func (list *DoublyLinkedList) DeleteNode(node *Node) {
	DeleteNode(node)
}

func (list *DoublyLinkedList) PopFront() *Node {
	node := list.Head.Next
	if node == list.Tail {
		return nil
	}
	DeleteNode(node)
	return node
}

func PrintList(list *DoublyLinkedList) {
	node := list.Head.Next
	for node != list.Tail {
		fmt.Printf("%d ", node.Val)
		node = node.Next
	}
	fmt.Println()
}

func PrintListReverse(list *DoublyLinkedList) {
	node := list.Tail.Prev
	for node != list.Head {
		fmt.Printf("%d ", node.Val)
		node = node.Prev
	}
	fmt.Println()
}

type LRUCache struct {
	capacity int
	list     *DoublyLinkedList
	m        map[int]*Node
}

func Constructor(capacity int) LRUCache {
	return LRUCache{
		capacity: capacity,
		list:     NewDoublyLinkedList(),
		m:        make(map[int]*Node),
	}
}

func (this *LRUCache) Get(key int) int {
	node, ok := this.m[key]
	if !ok {
		return -1
	}
	this.list.DeleteNode(node)
	this.list.PushBack(node)
	return node.Val
}

func (this *LRUCache) Put(key int, value int) {
	node, ok := this.m[key]
	if ok {
		node.Val = value
		this.list.DeleteNode(node)
		this.list.PushBack(node)
		return
	}
	if len(this.m) == this.capacity {
		node := this.list.PopFront()
		delete(this.m, node.Key)
	}
	node = &Node{Key: key, Val: value}
	this.list.PushBack(node)
	this.m[key] = node
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * obj := Constructor(capacity);
 * param_1 := obj.Get(key);
 * obj.Put(key,value);
 */
