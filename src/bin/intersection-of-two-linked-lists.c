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
struct ListNode *getIntersectionNode(struct ListNode *headA, struct ListNode *headB) {
  struct ListNode *currA = headA;
  struct ListNode *prevA = NULL;
  int countA = 0;
  struct ListNode *currB = headB;
  struct ListNode *prevB = NULL;
  int countB = 0;

  while (currA != NULL) {
    prevA = currA;
    currA = currA->next;
    countA++;
  }
  while (currB != NULL) {
    prevB = currB;
    currB = currB->next;
    countB++;
  }
  if (prevA != NULL && prevA == prevB) {
    if (countA < countB) {
      currA = headB;
      currB = headA;
      countA ^= countB;
      countB ^= countA;
      countA ^= countB;
    } else {
      currA = headA;
      currB = headB;
    }
    int skip = countA - countB;
    countA = 0;
    while (countA < skip) {
      currA = currA->next;
      countA++;
    }
    while (currA != currB) {
      currA = currA->next;
      currB = currB->next;
    }
    return currA;
  } else {
    return NULL;
  }
}
