#ifndef ENGINE_H
#define ENGINE_H


/*        A       B       C
 *    /////////////////////////
 *    |       |       |       |
 *  1 |   X   |       |       |
 *    |       |       |       |
 *    /////////////////////////
 *    |       |       |       |
 *  2 |       |   X   |   O   |
 *    |       |       |       |
 *    /////////////////////////
 *    |       |       |       |
 *  3 |   O   |   X   |   O   |
 *    |       |       |       |
 *    /////////////////////////
 */



typedef enum {
    HRAC_0 = (-1),
    VOLNO,
    HRAC_1
} hraci_t;

typedef enum {
    PROHRA,
    PROBIHA,
    REMIZA,
    VYHRA
} stav_hry_t;

//TODO vymyslet úspornější variantu (na políčko jen 2 bity => 9*2 = 18 => +/- 3*char na pole)
typedef char herni_pole_t;
typedef int souradnice_t[2];

//TODO vytvořit strukturu jednoho řádku (tahu) - pole všech možných variant daného tahu

typedef struct hra_uzel_t {
    stav_hry_t          vysledek_hry;        // vyhodnocení aktuálního uzlu
    long long           hodnota;             // pro rekurzivní skóring min-max
    unsigned long long  hloubka_zanoreni;    // číslo tahu
    hraci_t             na_tahu;             // kdo v tomto uzlu táhne
    herni_pole_t*       herni_pole;          // herní deska (!FREE)
    unsigned long long  pocet_moznych_tahu;  // počet volných políček
    struct hra_uzel_t*  predchozi_uzel;      // předchozí tah
    struct hra_uzel_t** mozne_tahy;          // všechny varianty tahů (!FREE)
} hra_uzel_t;

typedef struct {
    unsigned char velikost_pole;    // počet sloupců (řádků) čtvercového pole
    unsigned char souvisla_rada;    // počet obsazených políček nutných k výhře
    hra_uzel_t*   zacatek;          // vstupní uzel hry
    hra_uzel_t*   soucasna_pozice;  // aktuálně zpracovávaný uzel
    unsigned char zanoreni;         // nejvyšší dosažený počet tahů
} hra_strom_t;


/* vytvoří nový strom hry */
bool inicializuj(hra_strom_t* strom_hry, unsigned char velikost_pole, unsigned char souvisla_rada);
/* spustí prověřování možných cest hry
   vrátí počet možných stavů hry */
unsigned long long trenuj(hra_strom_t* hra_strom);
/* uloží vytvořené herní stavy do binárního souboru */
bool uloz_data(FILE* nazev_souboru);
/* uvolní alokovanou paměť (nody) */
bool uklid(hra_strom_t* hraStrom);
void vypis_uzel(hra_uzel_t *uzel, unsigned velikost_pole);


#endif // # ENGINE_H
