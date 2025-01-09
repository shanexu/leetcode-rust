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
struct ListNode *removeElements(struct ListNode *head, int val) {
  struct ListNode *curr = head;
  struct ListNode dummy_head;
  struct ListNode *prev = &dummy_head;
  prev->next = head;
  while (curr != NULL) {
    if (curr->val == val) {
      curr = curr->next;
      prev->next = curr;
    } else {
      prev = curr;
      curr = curr->next;
    }
  }
  return dummy_head.next;
}
