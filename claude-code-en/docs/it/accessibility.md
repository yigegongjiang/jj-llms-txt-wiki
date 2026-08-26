> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Usa Claude Code con un lettore di schermo

> Configura Claude Code per lettori di schermo come VoiceOver e NVDA, oltre alle impostazioni per ingranditori dello schermo, movimento ridotto e temi adatti ai daltonici.

Claude Code ha una modalità lettore di schermo che sostituisce la sua interfaccia terminale visiva con testo semplice e lineare. Invece di caselle, animazioni di progresso e ridisegni in-place, la modalità stampa righe etichettate che un lettore di schermo come VoiceOver o NVDA legge in ordine, in modo che possiate mantenere una conversazione completa, approvare i permessi degli strumenti e rivedere l'output da capo a fondo.

La modalità lettore di schermo è facoltativa. Se utilizzate un ingranditore dello schermo, movimento ridotto o un tema adatto ai daltonici invece di un lettore di schermo, consultate [Impostazioni di accessibilità oltre la modalità lettore di schermo](#accessibility-settings-beyond-screen-reader-mode).

<Note>
  La modalità lettore di schermo richiede Claude Code v2.1.181 o successivo. Le versioni precedenti rifiutano il flag `--ax-screen-reader` con `error: unknown option '--ax-screen-reader'`.
</Note>

<h2 id="turn-on-screen-reader-mode">
  Attiva la modalità lettore di schermo
</h2>

Scegli il metodo che corrisponde a quanto spesso utilizzi un lettore di schermo:

* Per una sessione: esegui `claude --ax-screen-reader`.
* Per sessioni avviate da una shell: imposta la variabile di ambiente `CLAUDE_AX_SCREEN_READER` su `1`. In Bash o Zsh, esegui `export CLAUDE_AX_SCREEN_READER=1`; in PowerShell, esegui `$env:CLAUDE_AX_SCREEN_READER = "1"`. Aggiungi la riga al tuo profilo shell per coprire ogni shell.
* Per ogni sessione sulla macchina: aggiungi `"axScreenReader": true` al tuo [file di impostazioni](/docs/it/settings) dell'utente. Questo copre qualsiasi terminale, incluso il terminale integrato di VS Code.

<Note>
  I metodi sono elencati in ordine di precedenza: il flag [`--ax-screen-reader`](/docs/it/cli-reference#cli-flags) sostituisce la variabile di ambiente [`CLAUDE_AX_SCREEN_READER`](/docs/it/env-vars), che sostituisce l'impostazione [`axScreenReader`](/docs/it/settings#available-settings).
</Note>

Se utilizzi Claude Code su SSH, imposta la variabile di ambiente o l'impostazione sulla macchina remota dove Claude Code viene eseguito.

Quando la modalità è attiva, la prima cosa che Claude Code stampa è una riga di conferma che nomina il metodo che l'ha attivata: `[Screen Reader Mode: on via flag]`, `[Screen Reader Mode: on via env]`, o `[Screen Reader Mode: on via settings]`. Il formato di denominazione del metodo richiede Claude Code v2.1.206 o successivo. Quando Claude Code si riavvia, ad esempio per completare l'installazione di un aggiornamento, il nuovo processo eredita la modalità attraverso la variabile di ambiente `CLAUDE_AX_SCREEN_READER`, quindi la sua riga di conferma legge `[Screen Reader Mode: on via env]` indipendentemente dal metodo che hai utilizzato.
Le versioni precedenti stampano `[Accessible screen reader mode: on]`.

<h2 id="turn-off-screen-reader-mode">
  Disattiva la modalità lettore di schermo
</h2>

Inverti il metodo che ha attivato la modalità: avvia senza il flag, annulla l'impostazione della variabile di ambiente o imposta `axScreenReader` su `false`. L'impostazione di `CLAUDE_AX_SCREEN_READER=0` mantiene la modalità disattivata anche quando l'impostazione è `true`.

<h2 id="what-your-screen-reader-hears">
  Cosa sente il tuo lettore di schermo
</h2>

In modalità lettore di schermo, Claude Code scrive testo semplice:

* nessun carattere di disegno di caselle per il chrome dell'interfaccia
* nessun suggerimento basato solo sul colore
* nessun ridisegno del contenuto che non è cambiato; i spinner di progresso vengono renderizzati come testo statico
* le tabelle nelle risposte di Claude vengono lette come frasi `Header: value` invece di una griglia di caratteri di casella. Richiede Claude Code v2.1.198 o successivo; le versioni precedenti disegnano le tabelle come griglie anche in modalità lettore di schermo.

L'output si accumula nello scrollback del tuo terminale, in modo che tu possa rileggere i turni precedenti con i comandi di revisione del tuo lettore di schermo o la ricerca del tuo terminale.

La modalità lettore di schermo viene renderizzata come testo scorrevole semplice, anche se hai attivato il [rendering a schermo intero](/docs/it/fullscreen) con l'[impostazione `tui`](/docs/it/settings#available-settings); l'impostazione non ha effetto mentre la modalità è attiva. Le sessioni in background allegate vengono comunque renderizzate a schermo intero; consulta [Limitazioni note](#known-limitations).

Ogni messaggio nella trascrizione inizia con un'etichetta che il tuo lettore di schermo annuncia, denominando cosa sia: i tuoi messaggi, le risposte di Claude, l'attività degli strumenti, gli errori e i prompt. Le etichette sono anche ricercabili, in modo che tu possa saltare tra le sezioni della trascrizione cercando nello scrollback del tuo terminale:

| Etichetta              | Significato                                                                                                  |
| :--------------------- | :----------------------------------------------------------------------------------------------------------- |
| `you:`                 | I tuoi messaggi                                                                                              |
| `claude:`              | Le risposte di Claude                                                                                        |
| `tool:`                | Attività degli strumenti, come una modifica di file o un comando eseguito                                    |
| `tool error:`          | Uno strumento che ha fallito                                                                                 |
| `error:`               | Un errore nella conversazione, come una richiesta API non riuscita                                           |
| `Permission Required:` | Un prompt di permesso in attesa della tua risposta                                                           |
| `Cost:`                | Il riepilogo dei costi della sessione quando Claude Code esce, se il tuo account [mostra i costi](/docs/it/costs) |

Il cursore del terminale segue il cursore di input, quindi il comando di lettura della riga corrente di un lettore di schermo risponde a "dove sono" con il prompt che stai modificando.

<h3 id="jump-between-turns">
  Salta tra i turni
</h3>

Claude Code emette marcatori di integrazione shell OSC 133 ai confini dei turni, in modo che il tasto per saltare al prompt precedente del tuo terminale si sposti tra i turni senza leggere l'intera trascrizione:

* iTerm2: Cmd+Shift+Up
* Terminale VS Code: Ctrl+Up su Windows, Cmd+Up su macOS
* Windows Terminal: nessun tasto per impostazione predefinita; associa l'azione `scrollToMark` nelle sue impostazioni
* Kitty e Ghostty: consulta la documentazione del terminale per il suo tasto di salto al prompt

macOS Terminal non agisce sui marcatori e Claude Code non li emette in WezTerm. In quei terminali, cerca nello scrollback l'etichetta `you:` invece.

<h2 id="answer-menus-and-prompts">
  Rispondi a menu e prompt
</h2>

In modalità lettore di schermo, i menu che normalmente navigheresti con i tasti freccia, inclusi i prompt di permesso, diventano elenchi numerati. Ogni opzione viene annunciata come una riga numerata, seguita da un prompt `Enter selection` che nomina l'intervallo valido. Digita il numero dell'opzione che desideri e premi Invio.

* Per annullare un menu dismissibile: premi Escape. Il suo prompt termina con `or Escape to cancel`.
* Se digiti un numero che non è nell'elenco: Claude Code annuncia l'intervallo valido e ti consente di riprovare.

I prompt sì-o-no chiedono una risposta digitata invece di un menu a due opzioni. Rispondi con `y` o `n` e premi Invio. Funzionano anche `yes` e `no`.

<h2 id="hear-when-claude-code-needs-you">
  Ascolta quando Claude Code ha bisogno di te
</h2>

In modalità lettore di schermo, Claude Code suona il campanello del terminale quando ha bisogno della tua attenzione, in modo che tu non debba continuare a controllare la trascrizione. Il campanello suona quando:

* Claude finisce una risposta
* appare un prompt di permesso
* uno strumento che è stato eseguito per più di 5 secondi finisce

Il campanello è l'avviso standard del tuo terminale. Per silenziarlo, modifica l'impostazione del campanello nella tua applicazione terminale. Il campanello non richiede la modalità lettore di schermo: al di fuori della modalità, imposta [`preferredNotifChannel`](/docs/it/settings#available-settings) su `"terminal_bell"` per avvisi simili quando Claude è in attesa di te. Consulta [Ottieni un campanello terminale o una notifica](/docs/it/terminal-config#get-a-terminal-bell-or-notification).

<h2 id="accessibility-settings-beyond-screen-reader-mode">
  Impostazioni di accessibilità oltre la modalità lettore di schermo
</h2>

Queste opzioni affrontano le esigenze di accessibilità al di fuori della modalità lettore di schermo. Tutte funzionano insieme ad essa.

* La [variabile di ambiente](/docs/it/env-vars) `CLAUDE_CODE_ACCESSIBILITY` è per gli ingranditori di schermo. Imposta `CLAUDE_CODE_ACCESSIBILITY=1` per mantenere visibile il cursore del terminale nativo in modo che gli ingranditori, come macOS Zoom, possano tracciare la posizione del cursore.
* L'[impostazione](/docs/it/settings#available-settings) `prefersReducedMotion` riduce o disabilita spinner, shimmer e altre animazioni senza modificare il resto dell'interfaccia.
* L'[impostazione](/docs/it/settings#available-settings) `theme` seleziona i colori dell'interfaccia, inclusi i temi adatti ai daltonici `dark-daltonized` e `light-daltonized`.

<h2 id="known-limitations">
  Limitazioni note
</h2>

Alcuni comportamenti non sono adattati per la modalità lettore di schermo:

* La modalità lettore di schermo non si attiva automaticamente quando un lettore di schermo è in esecuzione.
* I cambiamenti di modalità, come l'ingresso in [plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode), non vengono ancora annunciati.
* L'allegato a una [sessione in background](/docs/it/agent-view) con `claude attach` o dalla vista agente entra nello schermo alternativo del terminale, che non ha scrollback nativo. Questo è lo [stesso comportamento di altre sessioni allegate](/docs/it/fullscreen). Per uscire, premi la freccia sinistra su un prompt vuoto, o Ctrl+Z se un dialogo ha il focus.
* Claude Code annuncia i costi nel riepilogo che stampa all'uscita, non per turno.
* La modalità lettore di schermo non modifica la [modalità non interattiva](/docs/it/headless) con il flag `-p`. La modalità non interattiva scrive già testo semplice e rimane un'alternativa per lo scripting.

<h2 id="report-an-issue">
  Segnala un problema
</h2>

Se qualcosa non funziona con il tuo lettore di schermo, ingranditore o terminale, apri un problema sul [tracker dei problemi di Claude Code](https://github.com/anthropics/claude-code/issues) e menziona la tua tecnologia assistiva nel titolo. Includi il tuo sistema operativo, l'applicazione terminale e il nome e la versione della tecnologia assistiva nel rapporto.

<h2 id="related-resources">
  Risorse correlate
</h2>

Queste pagine contengono le voci di riferimento complete e la configurazione correlata per ciò che questa pagina copre:

* [Settings](/docs/it/settings#available-settings): le voci `axScreenReader`, `prefersReducedMotion`, `theme` e `preferredNotifChannel`
* [Environment variables](/docs/it/env-vars): le voci `CLAUDE_AX_SCREEN_READER` e `CLAUDE_CODE_ACCESSIBILITY`
* [CLI reference](/docs/it/cli-reference#cli-flags): il flag `--ax-screen-reader`
* [Terminal configuration](/docs/it/terminal-config): campanelli, notifiche e temi al di fuori della modalità lettore di schermo
* [Non-interactive mode](/docs/it/headless): esecuzioni `claude -p` con script, che scrivono testo semplice senza modalità lettore di schermo
