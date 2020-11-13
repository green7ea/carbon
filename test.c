#include <stdio.h>

#include "my_header.h"

int main(int argc, char **argv)
{
  HostIterator *iter = create_host_iter();

  char *host = 0;
  while (host = next_host(iter))
  {
    printf("%s\n", host);
    free_host(host);
  }

  free_host_iter(iter);

  return 0;
}
