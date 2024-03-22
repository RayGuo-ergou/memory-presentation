#include <stdlib.h>

int main() {
  // Allocate memory for an integer
  int *p = (int *)malloc(sizeof(int));
  *p = 10;
  // Memory leak: forgot to free the allocated memory
  // free(p);
  return 0;
}
