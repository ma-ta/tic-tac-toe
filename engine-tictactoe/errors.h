#ifndef ERRORS_H
#define ERRORS_H

extern int exit_value;                // návratová hodnota programu
extern bool err_log_on;               // zap/vyp logování chyb
extern unsigned long long err_count;  // celkový počet zalogovaných chyb od spuštění programu

#define err_log(msg, stop) {  \
    exit_value = EXIT_FAILURE;  \
    if (err_log_on) {  \
        fprintf(stderr, "[!] %06llu  " __FILE__ " | %s() | ln.: %06d:\n            %s\n",  \
                ++(err_count),  \
                __func__,  \
                __LINE__,  \
                msg  \
        );  /* fprintf() */  \
        \
        if (stop)  {  exit(exit_value = EXIT_FAILURE);  }  \
    }  \
}

#endif  // # ERRORS_H
