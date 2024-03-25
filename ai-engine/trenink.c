#include "trenink.h"
#include "chyby.h"


static const char HRACI_SYM[] = { 'X', 'O' };


/* hlavičky neveřejných funkcí modulu */


static hra_uzel_t* alokuj_uzel(hra_uzel_t* predchozi_uzel, unsigned velikost_pole);
static bool vymaz_node(hra_uzel_t* herni_uzel);
static bool tah(hra_strom_t* strom_hry);
static void odehraj_uzel(hra_uzel_t* herni_uzel, unsigned velikost_pole);
static stav_hry_t vyhodnot_uzel(hra_uzel_t* herni_uzel, unsigned char velikost_pole, unsigned char souvisla_rada);
static bool pole_zapis(hra_uzel_t *uzel, unsigned velikost_pole, souradnice_t xy, hraci_t hrac);
static hraci_t pole_precti(hra_uzel_t *uzel, unsigned velikost_pole, souradnice_t xy);


/* neveřejné funkce */

/* TEST MODULU */
#define DEBUG
#ifdef DEBUG

int main(void) {
    hra_strom_t hra = { NULL };
    inicializuj(&hra, 3);
    printf("%d\n", vyhodnot_uzel(hra.zacatek, hra.velikost_pole, hra.souvisla_rada));
    pole_zapis(hra.zacatek, hra.velikost_pole, (souradnice_t) { 1, 1 }, HRAC_1);
    pole_zapis(hra.zacatek, hra.velikost_pole, (souradnice_t) { 0, 2 }, HRAC_1);
    pole_zapis(hra.zacatek, hra.velikost_pole, (souradnice_t) { 2, 1 }, HRAC_1);
    vypis_uzel(hra.zacatek, hra.velikost_pole);
    printf("%d\n", vyhodnot_uzel(hra.zacatek, hra.velikost_pole, hra.souvisla_rada));

    return 0;
}

#endif
// TEST MODULU


static bool pole_zapis(hra_uzel_t *uzel, unsigned velikost_pole, souradnice_t xy, hraci_t hrac)
{
    int offset = xy[0] + (xy[1] * velikost_pole);

    // kontrola argumentů
    if (!uzel) {
        chyby_log("Ukazatel nesmi byt NULL.", 0);
        return false;
    }
    else if (   xy[0] < 0
             || xy[1] < 0
             || xy[0] >= velikost_pole
             || xy[1] >= velikost_pole
    ) {
        chyby_log("Souradnice mimo herni plochu.", 0);
        return false;
    }
    else if (offset < 0 || offset >= velikost_pole * velikost_pole) {
        chyby_log("Zapis mimo pole.", 0);
        return false;
    }
    else if (uzel->herni_pole[offset] != VOLNO) {
        chyby_log("Herni pole je jiz obsazeno.", 0);
        return false;
    }

    uzel->herni_pole[offset] = hrac;
    return true;
}

static hraci_t pole_precti(hra_uzel_t *uzel, unsigned velikost_pole, souradnice_t xy)
{
    int offset = xy[0] + (xy[1] * velikost_pole);

    // kontrola argumentů
    if (!uzel) {
        chyby_log("Ukazatel nesmi byt NULL.", 0);
        return false;
    }
    else if (   xy[0] < 0
             || xy[1] < 0
             || xy[0] >= velikost_pole
             || xy[1] >= velikost_pole
    ) {
        chyby_log("Souradnice mimo herni plochu.", 0);
        return false;
    }
    else if (offset < 0 || offset >= velikost_pole * velikost_pole) {
        chyby_log("Cteni mimo herni pole.", 0);
        return false;
    }
    else if (uzel->herni_pole[offset] != HRAC_0 && uzel->herni_pole[offset] != HRAC_1) {
        return VOLNO;
    }
    else {
        return (uzel->herni_pole[offset]);
    }
}



/* zapíše všechny možné tahy do poduzlů */
static void odehraj_uzel(hra_uzel_t* herni_uzel, unsigned velikost_pole)
{
    hra_uzel_t* aktualni_uzel = NULL;

    // kontrola argumentů
    if (!herni_uzel) {
        chyby_log("Ukazatel nesmi byt NULL.", 0);
        return;
    }


    // zápis všech možných tahů

}

/* vyhodnotí stav hry v uzlu - probíhá, remíza, výhra */
static stav_hry_t vyhodnot_uzel
    (
        hra_uzel_t* herni_uzel,
        unsigned char velikost_pole,
        unsigned char souvisla_rada
    )
{
    hraci_t    symbol, cteny_symbol;  // symbol hráče na předchozím políčku
    int        vyskytu = 0;           // počet výskytů symbolu
    stav_hry_t stav    = PROBIHA;     // návratová hodnota

    // kontrola argumentů
    if (!herni_uzel) {
        chyby_log("Ukazatel nesmi byt NULL.", 0);
        return PROBIHA;
    }


    // řádky
    for (unsigned radek = 0; radek < velikost_pole; radek++) {
        symbol = pole_precti(herni_uzel, velikost_pole, (souradnice_t) { 0, radek });
        for (unsigned sloupec = 1; sloupec < velikost_pole; sloupec++) {
            cteny_symbol = pole_precti(herni_uzel, velikost_pole, (souradnice_t) { sloupec, radek });
            if (symbol != VOLNO && symbol == cteny_symbol) {
                vyskytu++;
                if (vyskytu >= souvisla_rada) {
                    goto konec_vyhodnot_uzel;
                }
            }
            else if
        }
    }
    // sloupce


konec_vyhodnot_uzel:
    return stav;
}


bool tah(hra_strom_t* strom_hry)
{
    hra_uzel_t* aktualni_uzel = strom_hry->soucasna_pozice;
    // kontrola argumentů
    if (!strom_hry) {
        chyby_log("Ukazatel nesmi byt NULL.", 0);
        return false;
    }

    unsigned pocet_policek = (strom_hry->velikost_pole) * (strom_hry->velikost_pole);

    for (size_t i = 0; i < aktualni_uzel->pocet_moznych_tahu; i++) {
        aktualni_uzel->mozne_tahy[i] = alokuj_uzel(aktualni_uzel, strom_hry->velikost_pole);

        // alokace pole pro další možné tahy
        if ((aktualni_uzel->mozne_tahy[i]->mozne_tahy = (hra_uzel_t**) malloc(sizeof(void*) * aktualni_uzel->mozne_tahy[i]->pocet_moznych_tahu))
            == NULL)
        {
            chyby_log("Zrejme dosla operacni pamet - herni pole", 0);
            uklid(strom_hry);
            exit(1);
        }

        /* zbývá nastavit:
         *   zapsat všechny možné tahy do herních polí poduzlů,
         *   následně vyhodnotit stavy hry v poduzlech
         */
        odehraj_uzel(aktualni_uzel, strom_hry->velikost_pole);
        /*
         * vyhodnotí herní stav v novém uzlu a zapíše do vysledek_hry
         */
        vyhodnot_uzel(aktualni_uzel, strom_hry->velikost_pole, strom_hry->souvisla_rada);
    }

    strom_hry->zanoreni += 1;
}


static hra_uzel_t* alokuj_uzel(hra_uzel_t* predchozi_uzel, unsigned velikost_pole) {
    hra_uzel_t* novy_uzel    = NULL;
    herni_pole_t* herni_pole = NULL;
    hra_uzel_t** mozne_tahy  = NULL;
    unsigned char zanoreni   = (predchozi_uzel) ? (predchozi_uzel->hloubka_zanoreni + 1) : 0;

    // pokus o alokaci paměti pro nový node
    if ((novy_uzel = (hra_uzel_t*) malloc(sizeof(hra_uzel_t))) == NULL) {
        chyby_log("Zrejme dosla operacni pamet - novy uzel.", 0);
        return NULL;
    }
    // pokus o alokaci paměti pro herní pole
    else if ((herni_pole = (herni_pole_t *) malloc(sizeof(herni_pole_t) * velikost_pole * velikost_pole)) == NULL) {
        chyby_log("Zrejme dosla operacni pamet - herni pole", 0);
        free((void*) novy_uzel);
        novy_uzel = NULL;
        return NULL;
    }
    // pokus o alokaci pole pro další tahy
    else if ((velikost_pole * velikost_pole) - zanoreni > 0) {
        if ((mozne_tahy = (hra_uzel_t **) malloc(sizeof(void*) * ((velikost_pole * velikost_pole) - zanoreni))) == NULL) {
            chyby_log("Zrejme dosla operacni pamet - pole pro dalsi uzly", 0);
            free((void *) novy_uzel);
            free((void *) herni_pole);
            novy_uzel = NULL;
            herni_pole = NULL;
            return NULL;
        }
    }

    novy_uzel->vysledek_hry       = PROBIHA;
    novy_uzel->hodnota            = 0;
    novy_uzel->hloubka_zanoreni   = zanoreni;
    novy_uzel->na_tahu            = (predchozi_uzel)
                                      ? (predchozi_uzel->na_tahu == HRAC_0) ? HRAC_1 : HRAC_0
                                      : HRAC_0;
    novy_uzel->pocet_moznych_tahu = (velikost_pole * velikost_pole) - zanoreni;
    novy_uzel->predchozi_uzel     = predchozi_uzel;
    novy_uzel->herni_pole         = herni_pole;
    novy_uzel->mozne_tahy         = mozne_tahy;

    for (unsigned long long i = 0; i < novy_uzel->pocet_moznych_tahu; i++) {
        novy_uzel->mozne_tahy[i] = NULL;  // příp. užít calloc() při alokaci
    }

    // kopie předchozího stavu pole
    if (predchozi_uzel) {
        for (unsigned i = 0; i < velikost_pole * velikost_pole; i++) {
            novy_uzel->herni_pole[i] = predchozi_uzel->herni_pole[i];
        }
    }

    return novy_uzel;
}

static bool vymaz_node(hra_uzel_t* herni_uzel)
{
    // kontrola argumentů
    if (!herni_uzel) {
        chyby_log("Ukazatel na uzel nesmi byt NULL.", 0);
        return false;
    }

    if (herni_uzel->herni_pole) {
        free((void *) (herni_uzel->herni_pole));
        herni_uzel->herni_pole = NULL;
    }

    for (unsigned long long i = 0; i < herni_uzel->pocet_moznych_tahu; i++) {
        if (herni_uzel->mozne_tahy[i]) {
            vymaz_node(herni_uzel->mozne_tahy[i]);
            free((void *) (herni_uzel->mozne_tahy[i]));
            herni_uzel->mozne_tahy[i] = NULL;
        }
    }

    if (herni_uzel->mozne_tahy) {
        free((void *) (herni_uzel->mozne_tahy));
        herni_uzel->mozne_tahy = NULL;
    }

    free((void*) herni_uzel);
    herni_uzel = NULL;

    return true;
}



/* veřejné funkce */


bool inicializuj(
    hra_strom_t* strom_hry,
    unsigned char velikost_pole,
    unsigned char souvisla_rada)
{
    hra_uzel_t* novy_uzel = NULL;

    // kontrola argumentů
    if (velikost_pole < 3) {
        chyby_log("Velikost herniho pole nesmi byt < 3", 0);
        return false;
    }
    else if (souvisla_rada > velikost_pole) {
        chyby_log("Souvisla rada nemuze byt delsi nez herni pole.", 0);
        return false;
    }
    else if (!strom_hry) {
        chyby_log("Ukazatel na strom nesmi byt NULL.", 0);
        return false;
    }
    else if (strom_hry->zacatek) {
        chyby_log("Nastavte prvek { .zacatek } na NULL nebo "
                  "uvolnete jiz inicializovany strom.", 0);
        return false;
    }
    else if ((novy_uzel = alokuj_uzel(NULL, velikost_pole)) == NULL) {
        chyby_log("Nelze vytvorit vstupni bod hry.", 0);
        return false;
    }

    strom_hry->velikost_pole   = velikost_pole;
    strom_hry->souvisla_rada   = souvisla_rada;
    strom_hry->zacatek         = novy_uzel;
    strom_hry->soucasna_pozice = novy_uzel;
    strom_hry->zanoreni        = 0;

    return true;
}

unsigned long long trenuj(hra_strom_t* hra_strom)
{
    tah(hra_strom);
}

void vypis_uzel(hra_uzel_t *uzel, unsigned velikost_pole)
{
    const char* STAVY_HRY[] = { "prohra", "probiha", "remiza", "vyhra" };
    char symbol = ' ';

    // kontrola argumentů
    if (!uzel) {
        chyby_log("Ukazatel nesmi byt NULL.", 0);
        return;
    }

    puts("{");
    printf("vysledek_hry: %s\n", STAVY_HRY[uzel->vysledek_hry]);
    printf("hodnota: %lld\n", uzel->hodnota);
    printf("hloubka_zanoreni: %llu\n", uzel->hloubka_zanoreni);
    printf("na_tahu: %d\n", uzel->na_tahu);
    printf("herni_pole: %p\n", uzel->herni_pole);
    printf("pocet_moznych_tahu: %llu\n", uzel->pocet_moznych_tahu);
    printf("predchozi_uzel: %p\n", uzel->predchozi_uzel);
    printf("mozne_tahy: %p\n", uzel->mozne_tahy);

    // výpis herního pole (provizorní)
    // TODO předělat do samostatné funkce

    // vodorovná čára
    for (size_t j = 0; j < velikost_pole * 4 + 1; j++)
        putchar('-');
    putchar('\n');
    for (unsigned i = 0; i < velikost_pole * velikost_pole; i++) {
        switch (uzel->herni_pole[i]) {
            case HRAC_0:
                symbol = HRACI_SYM[0];
                break;
            case HRAC_1:
                symbol = HRACI_SYM[1];
                break;
            default:
                symbol = ' ';
                break;
        }
        printf("| %c ", symbol);
        if ((i + 1) % velikost_pole == 0) {
            puts("|");
            // vodorovná čára
            for (size_t j = 0; j < velikost_pole * 4 + 1; j++)
                putchar('-');
            putchar('\n');
        }

    }
    // /výpis herního pole

    puts("}");
}

bool uklid(hra_strom_t* hraStrom)
{
    // kontrola argumentů
    if (!hraStrom) {
        chyby_log("Ukazatel na strom nesmi byt NULL.", 0);
        return false;
    }
    else if (!(hraStrom->zacatek)) {
        chyby_log("Strom hry neni inicializovan.", 0);
        return false;
    }

    if (vymaz_node(hraStrom->zacatek)) {
        hraStrom->zacatek         = NULL;
        hraStrom->soucasna_pozice = NULL;
        hraStrom->velikost_pole   = 0;
        hraStrom->zanoreni        = 0;
        return true;
    }
    else {
        return false;
    }
}
