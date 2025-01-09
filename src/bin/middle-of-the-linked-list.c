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
struct ListNode *middleNode(struct ListNode *head) {
  int size = 0;
  struct ListNode *curr = head;
  while (curr != NULL) {
    curr = curr->next;
    size++;
  }
  size /= 2;
  int i = 0;
  curr = head;
  while (i < size) {
    curr = curr->next;
    i++;
  }
  return curr;
}
