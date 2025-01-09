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

typedef struct ListNode *Node;
struct ListNode *oddEvenList(struct ListNode *head) {
  if (head == NULL || head->next == NULL) {
    return head;
  }
  struct ListNode odd;
  odd.next = NULL;
  Node odd_curr = &odd;
  struct ListNode even;
  Node even_curr = &even;
  even.next = NULL;
  Node curr = head;
  Node next = NULL;
  int count = 0;
  while (curr != NULL) {
    next = curr->next;
    curr->next = NULL;
    if (!count) {
      even_curr->next = curr;
      even_curr = curr;
    } else {
      odd_curr->next = curr;
      odd_curr = curr;
    }
    curr = next;
    count=!count;
  }
  even_curr->next = odd.next;
  return even.next;
}

int main() {
  Node node = create_list(10);
  print_list(node);
  node = oddEvenList(node);
  print_list(node);
}
