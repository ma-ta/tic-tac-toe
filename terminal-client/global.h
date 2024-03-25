#ifndef GLOBAL_H
#define GLOBAL_H

#include <stdio.h>  // for special purposes (ncurses init problems...)
#include <string.h>
#include <ctype.h>
#include <ncursesw/curses.h>
#include <errno.h>
#include <locale.h>
#include <libintl.h>
#include <stdbool.h>
#include <stdlib.h>
#include <time.h>
#include "errors.h"

/* SYMBOLIC CONSTANTS */

#define ERRLOG_FILE  "errors.log"  // log file for errors
#define LOCALE       "cs_CZ.UTF-8"  // set locale

/* MACROS */

#define _(STRING)  gettext(STRING)  // macro for gettext


#endif // {} GLOBAL_H
