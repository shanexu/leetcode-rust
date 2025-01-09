#include "list_node.h"
#include <stdio.h>
#include <stdlib.h>

void print_list(struct ListNode *head) {
  struct ListNode *node = head;
  while (node != NULL) {
    printf("%d ", node->val);
    node = node->next;
  }
  printf("\n");
}

struct ListNode *create_list(int n) {
  return create_list_from_to(0, n - 1);
}

struct ListNode *create_list_from_to(int from, int to) {
  struct ListNode *head = NULL;
  struct ListNode *current = NULL;

  for (int i = from; i <= to; i++) {
    struct ListNode *newNode =
        (struct ListNode *)malloc(sizeof(struct ListNode));
    newNode->val = i;
    newNode->next = NULL;

    if (head == NULL) {
      head = newNode;
      current = newNode;
    } else {
      current->next = newNode;
      current = newNode;
    }
  }

  return head;
}

struct ListNode *create_list_from_array(int ns[], int n) {
  struct ListNode *head = NULL;
  struct ListNode *current = NULL;

  for (int i = 0; i < n; i++) {
    struct ListNode *newNode =
        (struct ListNode *)malloc(sizeof(struct ListNode));
    newNode->val = ns[i];
    newNode->next = NULL;

    if (head == NULL) {
      head = newNode;
      current = newNode;
    } else {
      current->next = newNode;
      current = newNode;
    }
  }
  return head;
}
