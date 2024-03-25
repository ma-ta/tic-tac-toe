#ifndef PISKVORKY_AI_CHYBY_H
#define PISKVORKY_AI_CHYBY_H


#include <stdio.h>
#include <stdlib.h>

static unsigned long long err_count = 0;

#define chyby_log(zprava, stop) {  \
    fprintf(stderr, "[!]  " __FILE__ " | chyba c. %06d |  radek %06d | fce. %s():\n     %s\n",  \
    ++err_count,  \
    __LINE__,  \
    __func__,  \
    zprava);  \
    \
    if (stop)  exit(EXIT_FAILURE);  \
}

#endif //PISKVORKY_AI_CHYBY_H
