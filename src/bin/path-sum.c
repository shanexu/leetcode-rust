#include "tree_node.h"
#include <stdbool.h>
#include <stdlib.h>

bool visit(struct TreeNode *node, int sum, int target_sum) {
  sum = node->val + sum;
  if (node->left == NULL && node->right == NULL && sum == target_sum) {
    return true;
  }
  if (node->left != NULL) {
    if (visit(node->left, sum, target_sum)) {
      return true;
    }
  }
  if (node->right != NULL) {
    if (visit(node->right, sum, target_sum)) {
      return true;
    }
  }
  return false;
}

bool hasPathSum(struct TreeNode *root, int targetSum) {
  if (root == NULL) {
    return false;
  }
  return visit(root, 0, targetSum);
}

