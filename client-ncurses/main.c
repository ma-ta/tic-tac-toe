#include <stdio.h>  // for special purposes (ncurses init problems...)
#include "global.h"


/* GLOBAL VARS */

int exit_value = EXIT_SUCCESS;     // exit value of the program (extern in errors.h)
bool err_log_on = true;            // errors.h switch on/of (extern in errors.h)
unsigned long long err_count = 0;  // global var for counting errors in a log file (extern in errors.h)
FILE *p_err_log_f = NULL;          // log file for errors
short color_pair = -1;

/* LOCAL FUNC. HEADERS */

static bool program_init(void);
static void at_exit(void);

/* PROGRAM ENTRY POINT */

int main(void)
{
    // initialization (ncurses...)
    if (!program_init()) {
        // ncurses not initialized
        puts(_("The program cannot be started - will exit now..."));
        return (exit_value = EXIT_FAILURE);
    }

    wborder(stdscr, '|', '|', '-', '-', '+', '+', '+', '+');
    mvprintw(LINES - 1, COLS - 12, "[ Ctrl+C ]");
    refresh();

    // waiting for Ctrl+C
    while (1)
        ;  // waiting for Enter


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

    // ncurses init
    if (initscr() == NULL) {
        err_log(_("Impossible to init ncurses."), 0);
        ret_val = false;
    }
    else {
        if (has_colors()) {
            if (start_color() != OK) {
                err_log(_("Impossible to use colors."), 0);
            }
            else {
                use_default_colors();
                init_pair(color_pair, -1, -1);
            }
        }
        else {
            err_log(_("This terminal does not support colors."), 0);
        }

        noecho();     // disable echo characters when typing
        cbreak();     // disable the keyboard buffer
        curs_set(0);  // disable cursor
    }

    return ret_val;
}  // {} program_init()

/* at the end of the program */
static void at_exit(void)
{
    if (stdscr && endwin() != OK)  {  err_log(_("An error when ending the ncurses."), 0);  }
    if (p_err_log_f)               {  fclose(p_err_log_f);  p_err_log_f = NULL;  }

    exit(exit_value);
}  // {} at_exit()
