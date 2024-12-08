#include "list_node.h"
#include <stdio.h>
#include <stdlib.h>

struct ListNode *deleteDuplicates(struct ListNode *head) {
  struct ListNode *prev_node = NULL;
  struct ListNode *node = head;
  struct ListNode *tmp_node = NULL;
  while (node != NULL) {
    tmp_node = node->next;
    if (prev_node != NULL) {
      if (prev_node->val != node->val) {
        prev_node->next = node;
        prev_node = node;
        prev_node->next = NULL;
      }
    } else {
      prev_node = node;
      prev_node->next = NULL;
    }
    node = tmp_node;
  }
  return head;
}

int main() {
  int ns[] = {1, 1, 2, 3, 4, 5, 5};
  struct ListNode *list = create_list_from_array(ns, 7);
  list = deleteDuplicates(list);
  print_list(list);
}
