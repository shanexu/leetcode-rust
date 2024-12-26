void helper(int* nums, int numsSize, int target, int* count) {
  if (numsSize == 0) {
    if (target == 0) {
      (*count)++;
    }
    return;
  }
  int n = *nums;
  helper(nums + 1, numsSize - 1, target - n, count);
  helper(nums + 1, numsSize - 1, target + n, count);
}

int findTargetSumWays(int* nums, int numsSize, int target) {
  int count = 0;
  helper(nums, numsSize, target, &count);
  return count;
}
