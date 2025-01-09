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
int getDecimalValue(struct ListNode *head) {
  struct ListNode *curr = head;
  int ans = 0;
  while (curr != NULL) {
    ans *= 2;
    ans += curr->val;
    curr = curr->next;
  }
  return ans;
}
