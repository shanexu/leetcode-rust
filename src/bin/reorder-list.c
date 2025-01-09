#include "list_node.h"
#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
void reorderList(struct ListNode *head) {
  struct ListNode *slow = head;
  struct ListNode *fast = head;
  while (fast != NULL && fast->next != NULL) {
    slow = slow->next;
    fast = fast->next->next;
  }
  if (slow == head) {
    return;
  }
  struct ListNode *prev = NULL;
  struct ListNode *curr = slow;
  struct ListNode *next = NULL;
  while (curr != NULL) {
    next = curr->next;
    curr->next = prev;
    prev = curr;
    curr = next;
  }
  curr = head;
  while (prev != NULL) {
    next = curr->next;
    curr->next = prev;
    if (next == slow) {
      break;
    }
    prev = prev->next;
    curr->next->next = next;
    curr = next;
  }
}

int main() {
  struct ListNode *node = create_list_from_to(1, 9);
  print_list(node);
  reorderList(node);
  print_list(node);
}
