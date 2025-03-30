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

double findMedianSortedArrays(int *nums1, int nums1Size, int *nums2,
                              int nums2Size) {
  int mid = (nums1Size + nums2Size) / 2;
  int prev;
  int curr;
  int k = 0;
  int i = 0;
  int j = 0;
  for (; k <= mid; k++) {
    prev = curr;
    if (i < nums1Size && j < nums2Size) {
      if (nums1[i] < nums2[j]) {
        curr = nums1[i++];
      } else {
        curr = nums2[j++];
      }
    } else {
      break;
    }
  }
  if (i < nums1Size) {
    for (; k <= mid; k++) {
      prev = curr;
      curr = nums1[i++];
    }
  }
  if (j < nums2Size) {
    for (; k <= mid; k++) {
      prev = curr;
      curr = nums2[j++];
    }
  }
  if (((nums1Size + nums2Size) & 1) == 1) {
    return (double)curr;
  } else {
    return (double)(prev + curr) / 2.0;
  }
}
