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
  struct ListNode *slow = head;
  struct ListNode *fast = head;
  while (fast != NULL && fast->next != NULL) {
    slow = slow->next;
    fast = fast->next->next;
  }
  return slow;
}
