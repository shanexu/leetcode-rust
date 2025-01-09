#include "list_node.h"
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
bool isPalindrome(struct ListNode *head) {
  int size = 0;
  struct ListNode *curr = head;
  while (curr != NULL) {
    curr = curr->next;
    size++;
  }
  int half = size / 2;
  int i = 0;
  struct ListNode *prev = NULL;
  struct ListNode *next = NULL;
  curr = head;
  while (i < half) {
    next = curr->next;
    curr->next = prev;
    prev = curr;
    curr = next;
    i++;
  }
  if ((size & 1) == 1) {
    curr = curr->next;
  }
  while (prev != NULL) {
    if (prev->val != curr->val) {
      return false;
    }
    prev = prev->next;
    curr = curr->next;
  }
  return true;
}
