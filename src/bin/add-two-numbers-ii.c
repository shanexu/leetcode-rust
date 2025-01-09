#include "list_node.h"
#include <stdio.h>
#include <stdlib.h>

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

typedef struct ListNode *Node;
Node reverse(Node node) {
  Node prev = NULL;
  Node next = NULL;
  while (node != NULL) {
    next = node->next;
    node->next = prev;
    prev = node;
    node = next;
  }
  return prev;
}

struct ListNode *addTwoNumbers(struct ListNode *l1, struct ListNode *l2) {
  l1 = reverse(l1);
  l2 = reverse(l2);
  int carry = 0;
  int value = 0;
  Node c = NULL;
  Node c1 = l1;
  Node c2 = l2;
  Node ans = NULL;
  Node curr = NULL;
  while (c1 !=NULL || c2 != NULL) {
    value = carry;
    if (c1 != NULL) {
      value += c1->val;
      c = c1;
      c1 = c1->next;
    }
    if (c2 != NULL) {
      value += c2->val;
      c = c2;
      c2 = c2->next;
    }
    c->next = NULL;
    carry = value / 10;
    c->val = value % 10;
    value = 0;
    if (curr == NULL) {
      ans = c;
      curr = c;
    } else {
      curr->next = c;
      curr = c;
    }
  }
  if (carry > 0) {
    Node newNode = (Node)malloc(sizeof(struct ListNode));
    newNode->next = NULL;
    newNode->val = carry;
    curr->next = newNode;
  }
  return reverse(ans);
}

int main() {
  Node n1 = create_list_from_to(5, 5);
  Node n2 = create_list_from_to(5, 5);
  Node n = addTwoNumbers(n1, n2);
  print_list(n);
}

