#include "global.h"
#include "engine.h"


int exit_value = EXIT_SUCCESS;     // návratová hodnota programu (jako extern v errors.h)
bool err_log_on = true;            // zap/vyp logování chyb (jako extern v errors.h)
unsigned long long err_count = 0;  // celkový počet zalogovaných chyb od spuštění programu (jako extern v errors.h)
FILE *p_err_log_f = NULL;          // soubor pro log chyb


/* HLAVIČKY LOKÁLNÍCH FCÍ. */


static bool program_init(void);
static void at_exit(void);


/* VSTUPNÍ BOD PROGRAMU */


int main(void)
{
    // inicializace
    if (!program_init()) {
        puts(_("The program cannot be started and will exit now..."));
        return (exit_value = EXIT_FAILURE);
    }

    // PLAYGROUND
    char buffer[1000] = "";
    while(1) {
        fgets(buffer, sizeof(buffer), stdin);
        // oříznutí znaků ukončení řádku
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


/* LOKÁLNÍ FCE. */

/* inicializace ncurses etc. */
static bool program_init(void) {
    bool ret_val = true;

    // registrace funkce před ukončením programu
    atexit(at_exit);

    // přesměrování stderr do LOG souboru
    p_err_log_f = freopen(ERRLOG_FILE, "w", stderr);
    if (p_err_log_f == NULL) {
        err_log_on = false;
    }

    // i18n inicializace
    setlocale(LC_ALL, LOCALE);


    return ret_val;
}  // {} program_init()

/* před ukončením programu */
static void at_exit(void)
{
    if (p_err_log_f)  {  fclose(p_err_log_f);  p_err_log_f = NULL;  }
    exit(exit_value);
}  // {} at_exit()
