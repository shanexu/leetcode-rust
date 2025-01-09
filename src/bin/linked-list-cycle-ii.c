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
struct ListNode *detectCycle(struct ListNode *head) {
  struct ListNode *fast = head;
  struct ListNode *slow = head;
  if (head == NULL) {
    return NULL;
  }
  while (1) {
    fast = fast->next;
    if (fast == NULL) {
      return NULL;
    }
    fast = fast->next;
    if (fast == NULL) {
      return NULL;
    }
    slow = slow->next;
    if (fast == slow) {
      break;
    }
  }
  struct ListNode *point = fast;

  slow = head;
  do {
      fast = fast->next;
      slow = slow->next;
  } while (fast != point);

  fast = head;
  while (slow != fast) {
    fast = fast->next;
    slow = slow->next;
  }
  return fast;
}

