#include "global.h"

/* GLOBAL VARS */

int exit_value = EXIT_SUCCESS;     // exit value of the program (extern in errors.h)
bool err_log_on = true;            // errors.h switch on/of (extern in errors.h)
unsigned long long err_count = 0;  // global var for counting errors in a log file (extern in errors.h)
FILE *p_err_log_f = NULL;          // log file for errors

/* LOCAL FUNC. HEADERS */

bool program_init(void);
void at_exit(void);

/* PROGRAM ENTRY POINT */

int main(void)
{
    // init of program (ncurses...)
    if (!program_init()) {
        // ncurses not initialized
        puts(_("The program cannot be started - will exit now..."));
        return (exit_value = EXIT_FAILURE);
    }

    const char *text = "Hello, World!";

    printw("%s\n", text);  refresh();

    return exit_value;
}  // {} main()


/* LOCAL FUNC. */

/* initialization of ncurses etc. */
bool program_init(void) {
    bool ret_val = true;

    // registering an exit function
    atexit(at_exit);

    // redirection of the stderr to a LOG file
    p_err_log_f = freopen(ERRLOG_FILE, "w", stderr);
    if (p_err_log_f == NULL) {
        err_log_on = false;
    }

    // locale init
    setlocale(LC_ALL, LOCALE);
    //bindtextdomain("main", "./locale/");
    //textdomain("main");

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
            }
        }
        else {
            err_log(_("This terminal does not support colors."), 0);
        }

        noecho();    // disable echo characters when typing
        nocbreak();  // disable the keyboard buffer
    }

    return ret_val;
}  // {} program_init()

/* at the end of the program */
void at_exit(void)
{
    do {  // waiting for Enter before exit
        printw("\n[Enter...]");  refresh();
    } while (getch() != '\n');

    if (stdscr && endwin() != OK)  {  err_log(_("An error when ending the ncurses."), 0);  }
    if (p_err_log_f)               {  fclose(p_err_log_f);  p_err_log_f = NULL;  }

    exit(exit_value);
}  // {} at_exit()
