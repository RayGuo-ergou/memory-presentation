#include <stdio.h>

int main() {
  char buffer[10];
  gets(buffer);
  printf("%s\n", buffer);
  return 0;
}
