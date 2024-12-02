
#include <ctype.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

int max(int a, int b);

int main() {
  int left, right;
  FILE* fp = fopen("./input.txt", "r");
  char line[60];
  int m = 0;
  long sum = 0;
  while ((fgets(line, 60, fp)) != NULL) {
    printf("%s", line);
    left = 0; right = strlen(line);
    while (!isdigit(line[left]))
      left++;
    while(!isdigit(line[right]))
      right--;
    printf("%c%c: ", line[left], line[right]);
    long val = (line[left]-'0')*10 + (line[right]-'0');
    printf("%ld\n", val);
    sum += val;
  }
  printf("sum: %ld\n", sum);
}

int max(int a, int b) {
  if (a > b) return a;
  return b;
}
