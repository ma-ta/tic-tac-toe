```text
/*
 * autor:
 *     jméno     :  Martin TÁBOR
 *     kontakt   :  st72836@student.upce.cz
 *     škola     :  UPCE - FEI
 *     obor      :  Webové technologie (Bc., komb.)
 *
 * předmět:
 *     název     :  Ročníkový projekt I. (KRPW1)
 *     semestr   :  2. | LS 2023/24
 *     vyučující :  Ing. Lukáš Čegan, Ph.D.
/*
```

# Popis konceptu: Webové rozhraní pro enginy deskových her

> Základní myšlenkou je realizace webového GUI pro hraní deskových (tahových) her.

![Rich picture &ndash; koncept projektu](draft_day0.webp)
*Koncept (rich picture)*

## Úvod

Deskové tahové hry s **úplnou znalostí pozic** jsou již dlouho předmětem zájmu informatiků i výpočetních úloh superpočítačů při katedrách matematiky.
Prakticky využívané **algoritmy** se v průběhu času mění a zlepšují na základě vzrůstajícího výpočetního výkonu CPU, GPU, NPU, rostoucích kapacit pamětí, ale i na základě nových přístupů a trendů, kam dnes můžeme zařadit techniky **strojového učení**. Základním principem hledání optimálního tahu již nemusí být prohledávání **stavového prostoru** hrubou silou a explicitně definované funkce modifikující &bdquo;strategii&ldquo;, ale např. autonomně vytrénovaný model na základě stovek či milionů odehrátých her proti soupeři (resp. konkurenční neuronové síti) s různými herními postupy.

## Architektura aplikace

Řešení herní logiky tedy nikdy není zcela triviální úkol a i u strategie založené kupříkladu na generování pseudonáhodných čísel musíme vyhodnocovat přípustnost tahů dle pravidel etc.
Architektura aplikace, by tedy měla tento fakt zohlednit &ndash; **oddělením herní logiky** a rozhraní určeného pro informování člověka o aktuálním stavu hry a jeho interakci s počítačem.

### Herní enginy

V roce 1997 poprvé porazil počítač **IBM Deep Blue** mistra světa Garriho Kasparova v šachu. Od té doby uběhlo již hodně času a velmi obstojným šachystou se může stát téměř jakýkoliv notebook či smartphone prostou instalací šachového programu &ndash; optimálně tzv. šachového enginu.

**Šachový engine** je většinou vysoce výkonná a sofistikovaná aplikace, která zpracovává herní logiku &ndash; v základě jako vstup přijímá různé povely typu *nová hra*, *tah: a2 na a4* a vrací odpověd v podobě zprávy nebo tahu zamýšleného počítačem. Dnes je k dispozici velké množství kvalitních open source projektů, které se nezaměřují jen na šachy, ale i jiné deskové hry v podobě piškvorek, go atd.

Open source projekty:
- **Stockfish**: jeden z nejlepších a nejznámnějších šachových enginů &ndash; multiplatformní, komunikuje přes standardizované API &ndash; UCI (Universal Chess Interface), velké množství grafických klientů
- **Boardgame.io**: engine pro tahové hry
- **Vassal.org**: engine pro deskové a karetní hry

### Koncept projektu

Projekt by měl být rozdělen na čtyři základní komponenty:

1. herní engine (logika hry | offline) &ndash; *využití Stockfish nebo vlastní tic-tac-toe*
2. server enginu (engine <-> klient | online) &ndash; *zřejmě Java (REST API, e-mail&hellip;)*
3. webová aplikace
   - web/backend (server enginu, DB&hellip; <-> frontend | platformě závislý | online)
   - web/frontend (komunikace s backendem | platformě nezávislý | online)

## Důvod volby tématu

Projekt zahrnuje oblasti z mnoha profilujících předmětů studijního oboru Webové technologie a vhodně je propojuje do uceleného aplikovaného řešení (produktu):
- Operační systémy &ndash; IPC komunikace *herní engine* <-> *server enginu*
- Tvorba webových aplikací &ndash; Java pro backend
- World Wide Web &ndash; frontend
- Databázové systémy &ndash; backend webové aplikace
- Programování a algoritmizace, matematický blok &ndash; obecně
- &hellip;
