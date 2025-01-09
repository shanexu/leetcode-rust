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
  struct ListNode *left = prev;
  struct ListNode *right = curr;
  if ((size & 1) == 1) {
    right = right->next;
  }
  while (left != NULL) {
    if (left->val != right->val) {
      return false;
    }
    left = left->next;
    right = right->next;
  }
  return true;
}
