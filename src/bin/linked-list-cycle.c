#include "list_node.h"
#include <stdbool.h>
#include <stdlib.h>

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
bool hasCycle(struct ListNode *head) {
  struct ListNode *fast = head;
  struct ListNode *middle = head;
  struct ListNode *slow = head;
  if (head == NULL) {
    return false;
  }
  while (true) {
    fast = fast->next;
    if (fast == NULL) {
      return false;
    }
    fast = fast->next;
    if (fast == NULL) {
      return false;
    }
    fast = fast->next;
    if (fast == NULL) {
      return false;
    }
    middle = middle->next->next;
    slow = slow->next;
    if (fast == slow || fast == middle || middle == slow) {
      return true;
    }
  }
}
