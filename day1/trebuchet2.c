
#include <ctype.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#define END_CH "oerxnt"
#define START_CH "zotfsen"

void getPDigit(const char* line, char*pDigit, int l, int r);
int getVal(char* s);
int min(int a, int b);
int max(int a, int b);

int main() {
  int ll, lr, rl, rr;
  int lval, rval;
  FILE* fp = fopen("./input.txt", "r");
  char line[60];
  int m = 0;
  long sum = 0;
  int ct = 1;
  while ((fgets(line, 60, fp)) != NULL) {
    printf("%s", line);
    // left digit
    lval = -1;
    ll = 0;
    lr = strlen(line)-1;
    while (lval == -1) {
      while (!isdigit(line[ll]) && strchr(START_CH, line[ll]) == NULL)
        ll++;
      //printf("ll: %d\n", ll);
      if (isdigit(line[ll])) {
        lval = line[ll]-'0';
        break;
      }
      lr = min(strlen(line)-1, ll+6);
      while (lr >= ll+2) {
        //printf("lr: %d\n", lr);
        if (strchr(END_CH, line[lr]) != NULL) {
          char pDigit[10] = { 0 };
          getPDigit(line, pDigit, ll, lr);
          //printf("pd: %s\n", pDigit);
          if ((lval = getVal(pDigit)) != -1)
            break;
          }
        lr--;
      }
      ll++;
    }
    //printf("lval: %d\n", lval);
    // right digit
    rval = -1;
    rl = 0;
    rr = strlen(line)-1;
    while (rval == -1) {
      while (!isdigit(line[rr]) && strchr(END_CH, line[rr]) == NULL)
        rr--;
      if (isdigit(line[rr])) {
        rval = line[rr]-'0';
        break;
      }
      rl = max(0, rr-6);
      while (rl <= rr-2) {
        if (strchr(START_CH, line[rl]) != NULL) {
          char pDigit[10] = { 0 };
          getPDigit(line, pDigit, rl, rr);
          //printf("pd: %s\n", pDigit);
          if ((rval = getVal(pDigit)) != -1)
            break;
        }
        rl++;
      }
      rr--;
    }
    //printf("rval: %d\n", rval);
    
    printf("%d%d: ", lval, rval);
    long val = lval*10 + rval;
    printf("%ld\n", val);
    sum += val;
  }
  printf("sum: %ld\n", sum);
}

int min(int a, int b) {
  if (a < b) return a;
  return b;
}

int max(int a, int b) {
  if (a>b) return a;
  return b;
}

void getPDigit(const char* line, char*pDigit, int l, int r) {
  //printf("r-l: %d\n", r-l);
  char buf[strlen(line)+1];
  strcpy(buf, line);
  //printf("ln: %s\n", line);
  buf[r+1] = 0;
  //printf("buf: %s\n", buf);
  char* ptr = &(buf[l]);
  strcpy(pDigit, ptr);
}

int getVal(char* s) {
  if (strcmp(s, "one") == 0) {
    return 1;
  } else if (strcmp(s, "two") == 0) {
    return 2;
  } else if (strcmp(s, "three") == 0) {
    return 3;
  } else if (strcmp(s, "four") == 0) {
    return 4;
  } else if (strcmp(s, "five") == 0) {
    return 5;
  } else if (strcmp(s, "six") == 0) {
    return 6;
  } else if (strcmp(s, "seven") == 0) {
    return 7;
  } else if (strcmp(s, "eight") == 0) {
    return 8;
  } else if (strcmp(s, "nine") == 0) {
    return 9;
  } else if (strcmp(s, "zero") == 0) {
    return 0;
  }
  return -1;
}
