#include "list_node.h"
#include <stdio.h>
#include <stdlib.h>

void print_list(struct ListNode *head) {
  struct ListNode *node = head;
  while (node != NULL) {
    printf("%d\n", node->val);
    node = node->next;
  }
}

struct ListNode *create_list(int n) {
  struct ListNode *head = NULL;
  struct ListNode *current = NULL;

  for (int i = 0; i < n; i++) {
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
