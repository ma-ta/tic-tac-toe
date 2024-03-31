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


/* SYMBOLICKÉ KONSTANTY */

#define ERRLOG_FILE  "errors.log"  // soubor pro log chyb
#define LOCALE       ""  // nastavení locale ("" => default)


/* MAKRA */

#define _(STRING)  gettext(STRING)  // makro pro gettext


#endif // # GLOBAL_H
