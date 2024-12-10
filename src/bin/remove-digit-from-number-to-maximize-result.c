#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *removeDigit(char *number, char digit) {
  int i = 0;
  char c = 0;
  int mi = -1;
  while ((c = number[i++]) != 0) {
    int idx = i - 1;
    if (c == digit) {
      if (mi == -1) {
        mi = idx;
      } else if (mi + 1 == idx) {
        mi = idx;
      } else if (number[mi+1] < digit) {
        mi = idx;
      }
    }
  }
  if (mi == -1) {
    return number;
  }
  char* newNumber = (char *)malloc(i - 1);
  memcpy(newNumber, number, mi);
  memcpy(newNumber + mi, number+mi+1, i - mi - 1);
  return newNumber;
}

int main() {
  char *n = removeDigit("551", '5');
  printf("%s\n", n);
}
