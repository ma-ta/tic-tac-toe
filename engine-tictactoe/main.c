#include "global.h"
#include "engine.h"


int exit_value = EXIT_SUCCESS;     // exit value of the program (extern in errors.h)
bool err_log_on = true;            // errors.h switch on/of (extern in errors.h)
unsigned long long err_count = 0;  // global var for counting errors in a log file (extern in errors.h)
FILE *p_err_log_f = NULL;          // log file for errors


/* LOCAL FUNC. HEADERS */


static bool program_init(void);
static void at_exit(void);


/* PROGRAM ENTRY POINT */


int main(void)
{
    // initialization
    if (!program_init()) {
        puts(_("The program cannot be started and will exit now..."));
        return (exit_value = EXIT_FAILURE);
    }

    // PLAYGROUND
    char buffer[1000] = "";
    while(1) {
        fgets(buffer, sizeof(buffer), stdin);
        // trim the read string
        buffer[strcspn(buffer, "\r\n")] = '\0';

        if (strcmp("ahoj", buffer) == 0) {
            puts("nazdar");
        }
        else if (strcmp("konec", buffer) == 0) {
            break;
        }
    }


    return exit_value;
}  // {} main()


/* LOCAL FUNC. */

/* initialization of ncurses etc. */
static bool program_init(void) {
    bool ret_val = true;

    // registering an exit function
    atexit(at_exit);

    // redirection of the stderr to a LOG file
    p_err_log_f = freopen(ERRLOG_FILE, "w", stderr);
    if (p_err_log_f == NULL) {
        err_log_on = false;
    }

    // i18n init
    setlocale(LC_ALL, LOCALE);


    return ret_val;
}  // {} program_init()

/* at the end of the program */
static void at_exit(void)
{
    if (p_err_log_f)  {  fclose(p_err_log_f);  p_err_log_f = NULL;  }
    exit(exit_value);
}  // {} at_exit()
