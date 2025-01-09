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
struct ListNode *reverseBetween(struct ListNode *head, int left, int right) {
  struct ListNode *prev = NULL;
  struct ListNode *curr = head;
  struct ListNode *next = NULL;
  int count = 0;

  while (count < left - 1) {
    prev = curr;
    curr = curr->next;
    count++;
  }

  struct ListNode *start = prev;
  struct ListNode *end = curr;

  prev = NULL;
  while (count < right) {
    next = curr->next;
    curr->next = prev;
    prev = curr;
    curr = next;
    count++;
  }

  end->next = curr;

  if (left != 1) {
    start->next = prev;
  } else {
    head = prev;
  }

  return head;
}

int main() {
  struct ListNode *list = create_list_from_to(1, 5);
  print_list(list);
  list = reverseBetween(list, 1, 5);
  print_list(list);
}
