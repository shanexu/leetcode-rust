#include "list_node.h"
#include <stdio.h>
#include <stdlib.h>

struct ListNode* rotateRight(struct ListNode* head, int k) {
  struct ListNode* node = head;
  struct ListNode* prev_node = NULL;
  struct ListNode* tail = NULL;
  int size = 0;
  int i = 0;
  while (node) {
    size++;
    tail = node;
    node = node->next;
  }
  if (size == 0 || size == 1) {
    return head;
  }
  k = k % size;
  if (k == 0) {
    return head;
  }
  node = head;
  for (i = 0; i < size - k; i++) {
    prev_node = node;
    node = node->next;
  }
  tail->next = head;
  prev_node->next = NULL;
  return node;
}

int main() {
  struct ListNode* list = create_list(5);
  list = rotateRight(list, 2);
  print_list(list);
}
