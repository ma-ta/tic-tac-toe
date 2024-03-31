#include <stdio.h>
#include <stdlib.h>

static int server_rw[2];  // souborové deskriptory

void at_exit(void)
{

}

int main(int argc, char *argv[])
{
    atexit(at_exit);

    if (argc != 2) {
        fprintf(stderr, "%s ENGINE_EXEC\n", argv[0]);
        return 1;
    }


    


    return 0;
}
