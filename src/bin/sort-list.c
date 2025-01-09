#include "list_node.h"
#include <stdio.h>
#include <stdlib.h>

typedef struct ListNode *Node;

Node middle(Node head) {
  if (head == NULL) {
    return head;
  }
  Node slow = head;
  Node fast = head;
  while (fast->next != NULL && fast->next->next != NULL) {
    slow = slow->next;
    fast = fast->next->next;
  }
  return slow;
}

Node merge(Node a, Node b) {
  Node result = NULL;
  if (a == NULL) {
    return b;
  }
  if (b == NULL) {
    return a;
  }
  if (a->val < b->val) {
    result = a;
    result->next = merge(a->next, b);
  } else {
    result = b;
    result->next = merge(a, b->next);
  }
  return result;
}

Node sort(Node h) {
  if (h == NULL || h->next == NULL) {
    return h;
  }
  Node m = middle(h);
  Node mn = m->next;
  m->next = NULL;
  Node left = sort(h);
  Node right = sort(mn);
  return merge(left, right);
}

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode *sortList(struct ListNode *head) {
  return sort(head);
}

int main() {
  int arr[] = {-1, 5, 3, 4, 0};
  struct ListNode *node = create_list_from_array(arr, 5);
  print_list(node);
  node = sortList(node);
  print_list(node);
}
