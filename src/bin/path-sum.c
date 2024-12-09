#include "tree_node.h"
#include <stdbool.h>
#include <stdlib.h>

#define BUFFER_SIZE 32

struct StackBuffer {
  int top;
  void *buffer[BUFFER_SIZE];
};

struct StackBuffer _stack;
struct StackBuffer *stack = &_stack;

void stack_push(void *p) {
  stack->buffer[stack->top] = p;
  stack->top++;
}

void *stack_pop() {
  if (stack->top == 0) {
    return NULL;
  }
  return stack->buffer[--stack->top];
}

bool hasPathSum(struct TreeNode *root, int targetSum) {
  stack->top = 0;
  struct TreeNode *node = root;

  if (root == NULL) {
    return false;
  }

  stack_push(root);
  while ((node = stack_pop()) != NULL) {
    if (node->val == targetSum && node->left == NULL && node->right == NULL) {
      return true;
    }
    if (node->right != NULL) {
      node->right->val += node->val;
      stack_push(node->right);
    }
    if (node->left != NULL) {
      node->left->val += node->val;
      stack_push(node->left);
    }
  }
  return false;
}

