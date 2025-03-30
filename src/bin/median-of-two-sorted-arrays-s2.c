#include <stdio.h>
#include <stdlib.h>

double findMedianSortedArrays(int *nums1, int nums1Size, int *nums2,
                              int nums2Size);

int main() {
  int *nums1 = malloc(sizeof(int) * 2);
  nums1[0] = 1;
  nums1[1] = 2;
  int *nums2 = malloc(sizeof(int) * 2);
  nums2[0] = 3;
  nums2[1] = 4;
  double median = findMedianSortedArrays(nums1, 2, nums2, 2);
  printf("%f\n", median);

  free(nums1);
  free(nums2);

  nums1 = malloc(sizeof(int) * 2);
  nums1[0] = 1;
  nums1[1] = 2;
  nums2 = malloc(sizeof(int) * 1);
  nums2[0] = 3;
  median = findMedianSortedArrays(nums1, 2, nums2, 1);
  printf("%f\n", median);

  return 0;
}

#define MAX(a, b) ((a) > (b) ? (a) : (b))

#define MIN(a, b) ((a) < (b) ? (a) : (b))

double findMedianSortedArrays(int *nums1, int nums1Size, int *nums2,
                              int nums2Size) {
  const int MIN_VALUE = -10e6 - 1;
  const int MAX_VALUE = 10e6 + 1;
  if (nums1Size > nums2Size) {
    int tmpSize = nums1Size;
    nums1Size = nums2Size;
    nums2Size = tmpSize;
    int *tmpNums = nums1;
    nums1 = nums2;
    nums2 = tmpNums;
  }
  int left = 0;
  int right = nums1Size;
  while (left <= right) {
    int pa = (left + right) / 2;
    int pb = (nums1Size + nums2Size + 1) / 2 - pa;
    int leftMaxA = MIN_VALUE;
    if (pa != 0) {
      leftMaxA = nums1[pa - 1];
    }
    int rightMinA = MAX_VALUE;
    if (pa != nums1Size) {
      rightMinA = nums1[pa];
    }
    int leftMaxB = MIN_VALUE;
    if (pb != 0) {
      leftMaxB = nums2[pb - 1];
    }
    int rightMinB = MAX_VALUE;
    if (pb != nums2Size) {
      rightMinB = nums2[pb];
    }
    if (leftMaxA <= rightMinB && leftMaxB <= rightMinA) {
      if (((nums1Size + nums2Size) & 1) == 1) {
        return (double)MAX(leftMaxA, leftMaxB);
      } else {
        return (double)(MAX(leftMaxA, leftMaxB) + MIN(rightMinA, rightMinB)) /
               2.0;
      }
    } else if (leftMaxA > rightMinB) {
      right = pa - 1;
    } else {
      left = pa + 1;
    }
  }
  return 0.0;
}
