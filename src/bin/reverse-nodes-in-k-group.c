#include <stdio.h>
#include <stdlib.h>
#include "list_node.h"

struct ListNode *reverseList(struct ListNode *head) {
  struct ListNode *node = head, *prev_node = NULL, *tmp_node = NULL;

  while (node != NULL) {
    tmp_node = node->next;
    node->next = prev_node;
    prev_node = node;
    node = tmp_node;
  }
  return prev_node;
}

struct ListNode *reverseKGroup(struct ListNode *head, int k) {
  struct ListNode *node = NULL;
  struct ListNode *tmp_node = NULL;
  struct ListNode *prev_node = NULL;
  struct ListNode *group_head_node = NULL;
  struct ListNode *prev_group_head_node = NULL;
  struct ListNode *new_head = NULL;
  int i = 0;
  int r = 0;

  if (k == 1) {
    return head;
  }

  node = head;
  while (node != NULL) {
    r = i % k;
    if (r == 0) {
      group_head_node = node;
    } else if (r == (k - 1)) {
      if (prev_group_head_node != NULL) {
        prev_group_head_node->next = node;
      } else {
        new_head = node;
      }
      prev_group_head_node = group_head_node;
    }
    tmp_node = node->next;
    node->next = r == 0 ? NULL : prev_node;
    prev_node = node;
    node = tmp_node;
    i++;
  }
  r = i % k;
  if (r != 0) {
    prev_node = reverseList(prev_node);
    if (prev_group_head_node != NULL) {
      prev_group_head_node->next = prev_node;
    } else {
      new_head = prev_node;
    }
  }
  return new_head;
}

int main() {
  struct ListNode *list = create_list(3);
  struct ListNode *new_list = reverseKGroup(list, 4);
  print_list(new_list);
}
