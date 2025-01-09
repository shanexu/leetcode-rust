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
struct ListNode* partition(struct ListNode* head, int x) {
  struct ListNode lt;
  struct ListNode gte;
  struct ListNode* lt_curr = &lt;
  struct ListNode* gte_curr = &gte;
  struct ListNode* curr = head;
  struct ListNode *next = NULL;
  while (curr != NULL) {
    if (curr->val < x) {
      lt_curr->next = curr;
      lt_curr = curr;
    } else {
      gte_curr->next = curr;
      gte_curr = curr;
    }
    next = curr->next;
    curr->next = NULL;
    curr = next;
  }
  if (lt.next == NULL) {
    return gte.next;
  }
  lt_curr->next = gte.next;
  return lt.next;
}
