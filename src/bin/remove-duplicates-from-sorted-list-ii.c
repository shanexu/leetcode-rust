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
struct ListNode *deleteDuplicates(struct ListNode *head) {
  struct ListNode dummy;
  struct ListNode *curr = head;
  struct ListNode *prev = &dummy;
  prev->next = head;
  while (curr != NULL) {
    int val = curr->val;
    int count = 0;
    while (curr->next != NULL && curr->next->val == val) {
      curr = curr->next;
      count++;
    }
    if (count > 0) {
      curr = curr->next;
      prev->next = curr;
    } else {
      prev->next = curr;
      prev = curr;
      curr = curr->next;
    }
  }
  return dummy.next;
}
