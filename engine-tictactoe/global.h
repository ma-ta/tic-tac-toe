#ifndef GLOBAL_H
#define GLOBAL_H


#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <locale.h>
#include <libintl.h>
#include <string.h>
#include <ctype.h>
#include <errno.h>
#include "errors.h"


/* SYMBOLIC CONSTANTS */

#define ERRLOG_FILE  "errors.log"  // log file for errors
#define LOCALE       ""  // set locale ("" => default)


/* MACROS */

#define _(STRING)  gettext(STRING)  // macro for gettext


#endif // # GLOBAL_H
