> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Usa Claude Code con un lettore di schermo

> Configura Claude Code per lettori di schermo come VoiceOver e NVDA, oltre alle impostazioni per ingranditori dello schermo, movimento ridotto e temi adatti ai daltonici.

Claude Code ha una modalità lettore di schermo che sostituisce la sua interfaccia terminale visiva con testo semplice e lineare. Invece di caselle, animazioni di progresso e ridisegni in-place, Claude Code stampa righe etichettate che un lettore di schermo come VoiceOver o NVDA legge in ordine. Potete mantenere una conversazione completa, approvare i permessi degli strumenti e rivedere l'output da capo a fondo.

La modalità lettore di schermo è facoltativa. Se utilizzate un ingranditore dello schermo, movimento ridotto o un tema adatto ai daltonici invece di un lettore di schermo, impostate `CLAUDE_CODE_ACCESSIBILITY`, `prefersReducedMotion` o `theme` dalla tabella [Impostazioni di accessibilità](#accessibility-settings). La modalità lettore di schermo adatta solo l'interfaccia terminale, quindi non è necessaria nel pannello chat dell'estensione VS Code. Su Claude Code v2.1.236 o successivo, l'estensione [annuncia l'attività della conversazione al vostro lettore di schermo](/docs/it/vs-code#use-a-screen-reader) senza alcuna impostazione.

<h2 id="turn-on-screen-reader-mode">
  Attiva la modalità lettore di schermo
</h2>

Scegliete il metodo che corrisponde a quanto spesso utilizzate un lettore di schermo:

* Per una sessione: eseguite `claude --ax-screen-reader`.
* Per sessioni avviate da una shell: impostate la variabile di ambiente `CLAUDE_AX_SCREEN_READER` su `1`. In Bash o Zsh, eseguite `export CLAUDE_AX_SCREEN_READER=1`. In PowerShell, eseguite `$env:CLAUDE_AX_SCREEN_READER = "1"`. Aggiungete quella riga al vostro profilo shell per mantenerla per le shell future.
* Per ogni sessione sulla macchina: aggiungete `"axScreenReader": true` al vostro [file di impostazioni](/docs/it/settings) dell'utente. L'impostazione si applica in qualsiasi terminale, incluso il terminale integrato di VS Code.

Se combinate i metodi, Claude Code applica il flag [`--ax-screen-reader`](/docs/it/cli-reference#cli-flags) sulla variabile di ambiente [`CLAUDE_AX_SCREEN_READER`](/docs/it/env-vars#variables), e la variabile sull'impostazione [`axScreenReader`](/docs/it/settings-reference#axscreenreader).

Se utilizzate Claude Code su SSH, impostate la variabile di ambiente o l'impostazione sulla macchina remota dove Claude Code viene eseguito.

La prima riga che Claude Code stampa conferma la modalità: `[Screen Reader Mode: on via flag]`, `[Screen Reader Mode: on via env]` o `[Screen Reader Mode: on via settings]`.

<h2 id="turn-off-screen-reader-mode">
  Disattiva la modalità lettore di schermo
</h2>

Invertite il metodo che ha attivato la modalità: avviate senza il flag, annullate l'impostazione della variabile di ambiente o impostate `axScreenReader` su `false`. Se impostate `CLAUDE_AX_SCREEN_READER` su `0`, Claude Code mantiene la modalità disattivata anche quando l'impostazione è `true`.

<h2 id="accessibility-settings">
  Impostazioni di accessibilità
</h2>

La tabella elenca ogni opzione di accessibilità, se la impostate come flag, variabile di ambiente o impostazione, e cosa cambia.

| Opzione                                                                 | Tipo                  | Cosa cambia                                                                                                                                                                                                                                                          |
| :---------------------------------------------------------------------- | :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`--ax-screen-reader`](/docs/it/cli-reference#cli-flags)                     | Flag                  | Modalità lettore di schermo per una sessione.                                                                                                                                                                                                                        |
| [`CLAUDE_AX_SCREEN_READER`](/docs/it/env-vars#variables)                     | Variabile di ambiente | Modalità lettore di schermo per sessioni avviate dalla shell dove l'avete impostata.                                                                                                                                                                                 |
| [`axScreenReader`](/docs/it/settings-reference#axscreenreader)               | Impostazione          | Modalità lettore di schermo per ogni sessione quando `true`.                                                                                                                                                                                                         |
| [`CLAUDE_AX_STARTUP_QUIET_MS`](/docs/it/env-vars#variables)                  | Variabile di ambiente | Quanto tempo Claude Code attende dopo la riga di conferma prima di disegnare il primo prompt in modalità lettore di schermo. Richiede Claude Code v2.1.217 o successivo.                                                                                             |
| [`CLAUDE_AX_PREPARK_MS`](/docs/it/env-vars#variables)                        | Variabile di ambiente | Quanto tempo Claude Code attende, con il cursore all'inizio della riga, prima di scrivere una riga nuova o modificata in modalità lettore di schermo. Richiede Claude Code v2.1.233 o successivo.                                                                    |
| [`CLAUDE_CODE_ACCESSIBILITY`](/docs/it/env-vars#variables)                   | Variabile di ambiente | Un cursore terminale che rimane visibile per ingranditori dello schermo come macOS Zoom quando lo impostate su `1`. Il cursore segue il cursore di input e, su Claude Code v2.1.218 o successivo, la riga evidenziata in menu e pannelli come `/config` e `/plugin`. |
| [`prefersReducedMotion`](/docs/it/settings-reference#prefersreducedmotion)   | Impostazione          | Spinner, shimmer e altre animazioni ridotti o assenti quando `true`.                                                                                                                                                                                                 |
| [`theme`](/docs/it/settings-reference#theme)                                 | Impostazione          | I colori dell'interfaccia, inclusi i temi adatti ai daltonici `dark-daltonized` e `light-daltonized`. Potete anche sceglierne uno con [`/theme`](/docs/it/commands#all-commands).                                                                                         |
| [`preferredNotifChannel`](/docs/it/settings-reference#preferrednotifchannel) | Impostazione          | Con il valore `"terminal_bell"`, un campanello terminale al di fuori della modalità lettore di schermo quando Claude è in attesa di voi.                                                                                                                             |

<h2 id="what-your-screen-reader-hears">
  Cosa sente il vostro lettore di schermo
</h2>

In modalità lettore di schermo, Claude Code scrive testo semplice:

* Nessun carattere di disegno di caselle per il chrome dell'interfaccia
* Nessun suggerimento basato solo sul colore
* Nessun ridisegno del contenuto che non è cambiato. Gli spinner di progresso vengono renderizzati come testo statico
* Le tabelle nelle risposte di Claude vengono lette come frasi `Header: value` invece di una griglia di caratteri di casella

Claude Code lascia tutto ciò che stampa nello scrollback del vostro terminale, in modo che possiate rileggere i turni precedenti con i comandi di revisione del vostro lettore di schermo o la ricerca del vostro terminale. Claude Code ignora l'[impostazione `tui`](/docs/it/settings-reference#tui) in modalità lettore di schermo. A parte le sessioni in background allegate elencate sotto [Limitazioni note](#known-limitations), stampa testo scorrevole invece di [rendering a schermo intero](/docs/it/fullscreen).

Claude Code attende anche in due punti in modo che il vostro lettore di schermo possa stare al passo:

* Dopo che Claude Code stampa la riga di conferma, attende 3 secondi prima di disegnare il prompt, in modo che il vostro lettore di schermo possa finire la riga. Premete un tasto qualsiasi per terminare l'attesa. Per cambiare la durata dell'attesa, impostate [`CLAUDE_AX_STARTUP_QUIET_MS`](/docs/it/env-vars#variables).
* Prima che Claude Code scriva una riga nuova o modificata, come un suggerimento o più della risposta di Claude, sposta il cursore all'inizio della riga e attende 50 millisecondi. Il vostro lettore di schermo legge quindi la riga dal suo primo carattere. I caratteri che digitate o eliminate alla fine della riga di input appaiono immediatamente. Per cambiare la durata dell'attesa, impostate [`CLAUDE_AX_PREPARK_MS`](/docs/it/env-vars#variables).

Ogni messaggio nella trascrizione inizia con un'etichetta che il vostro lettore di schermo annuncia, denominando cosa sia: i vostri messaggi, le risposte e il pensiero di Claude, l'attività degli strumenti, gli errori e gli avvisi e i prompt. Le etichette sono anche ricercabili, in modo che possiate saltare tra le sezioni della trascrizione cercando nello scrollback del vostro terminale:

| Etichetta              | Significato                                                                                                     |
| :--------------------- | :-------------------------------------------------------------------------------------------------------------- |
| `you:`                 | I vostri messaggi                                                                                               |
| `claude:`              | Le risposte di Claude                                                                                           |
| `thinking:`            | Il pensiero di Claude                                                                                           |
| `tool:`                | Attività degli strumenti, come una modifica di file o un comando eseguito                                       |
| `tool error:`          | Uno strumento che ha fallito                                                                                    |
| `error:`               | Un errore nella conversazione, come una richiesta API non riuscita                                              |
| `warning:`             | Un avviso da Claude Code, come un passaggio a un modello di fallback                                            |
| `Permission Required:` | Un prompt di permesso in attesa della vostra risposta                                                           |
| `Cost:`                | Il riepilogo dei costi della sessione quando Claude Code esce, se il vostro account [mostra i costi](/docs/it/costs) |

Claude Code mantiene il cursore del terminale sul cursore di input, in modo che il comando di lettura della riga corrente del vostro lettore di schermo legga il prompt che state modificando.

Mentre digitate alla fine della riga di input, o premete `Backspace` lì, Claude Code scrive solo i caratteri che cambiano. Il vostro lettore di schermo ripete solo quei caratteri.

Quando eliminate una parola o una riga con una delle [scorciatoie di modifica del testo](/docs/it/interactive-mode#text-editing), Claude Code annuncia il testo eliminato:

* Eliminazione di parole con `Ctrl+W` o `Alt+D`, o con `Option+Delete` su macOS o `Ctrl+Backspace` su Windows
* Eliminazione fino all'inizio della riga con `Ctrl+U` o `Cmd+Backspace`
* Eliminazione fino alla fine della riga con `Ctrl+K`

Quando ciclete [modalità di permesso](/docs/it/permission-modes) con `Shift+Tab`, Claude Code annuncia la modalità di permesso su cui atterrate, come `[plan mode on]` o `[accept edits on]`. Claude Code stampa l'annuncio una volta e non lo ripete su ridisegni successivi.

<h3 id="jump-between-turns">
  Salta tra i turni
</h3>

Claude Code emette marcatori di integrazione shell OSC 133 ai confini dei turni, in modo che il tasto per saltare al prompt precedente del vostro terminale si sposti tra i turni senza leggere l'intera trascrizione:

* iTerm2: Cmd+Shift+Up
* Terminale VS Code: Ctrl+Up su Windows, Cmd+Up su macOS
* Windows Terminal: nessun tasto per impostazione predefinita; associate l'azione `scrollToMark` nelle sue impostazioni
* Kitty e Ghostty: consultate la documentazione del terminale per il suo tasto di salto al prompt

macOS Terminal non agisce sui marcatori e Claude Code non li emette in WezTerm. In quei terminali, cercate nello scrollback l'etichetta `you:` invece.

<h2 id="answer-menus-and-prompts">
  Rispondete a menu e prompt
</h2>

In modalità lettore di schermo, i menu che normalmente navighereste con i tasti freccia, inclusi i prompt di permesso, diventano elenchi numerati. Claude Code annuncia ogni opzione come una riga numerata, quindi un prompt `Enter selection` che nomina l'intervallo valido. Digitate il numero dell'opzione che desiderate e premete Invio.

* Premete Escape per annullare un menu il cui prompt termina con `or Escape to cancel`.
* Se digitate un numero che non è nell'elenco, Claude Code annuncia l'intervallo valido e vi consente di riprovare.

Il selettore [`/effort`](/docs/it/model-config#adjust-effort-level), che è un cursore al di fuori della modalità lettore di schermo, diventa lo stesso tipo di elenco numerato.

I prompt sì-o-no chiedono una risposta digitata invece di un menu a due opzioni. Rispondete con `y` o `n` e premete Invio. Funzionano anche `yes` e `no`.

<h2 id="hear-when-claude-code-needs-you">
  Ascoltate quando Claude Code ha bisogno di voi
</h2>

In modalità lettore di schermo, Claude Code suona il campanello del terminale quando ha bisogno della vostra attenzione, in modo che non dobbiate continuare a controllare la trascrizione. Il campanello suona quando:

* Claude finisce una risposta
* Un prompt o una finestra di dialogo ha bisogno della vostra risposta, come un prompt di permesso
* Uno strumento che è stato eseguito per più di 5 secondi finisce

Il campanello è l'avviso standard del vostro terminale. Per silenziarlo, cambiate l'impostazione del campanello nella vostra applicazione terminale. Al di fuori della modalità lettore di schermo, impostate [`preferredNotifChannel`](/docs/it/settings-reference#preferrednotifchannel) su `"terminal_bell"` per ottenere un [campanello simile](/docs/it/terminal-config#get-a-terminal-bell-or-notification) quando Claude è in attesa di voi.

<h2 id="known-limitations">
  Limitazioni note
</h2>

Alcuni comportamenti non sono adattati per la modalità lettore di schermo:

* La modalità lettore di schermo non si attiva automaticamente quando un lettore di schermo è in esecuzione.
* Claude Code non annuncia un cambio di modalità di permesso effettuato in qualsiasi modo diverso dal ciclo con `Shift+Tab`, come l'ingresso in [plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode) da un comando.
* L'allegato a una [sessione in background](/docs/it/agent-view) con `claude attach` o dalla vista agente entra nello schermo alternativo del terminale, che non ha scrollback nativo. Questo è lo [stesso comportamento di altre sessioni allegate](/docs/it/fullscreen). Per uscire, premete la freccia sinistra su un prompt vuoto, o Ctrl+Z se una finestra di dialogo ha il focus.
* Claude Code annuncia i costi nel riepilogo che stampa all'uscita, non per turno.
* La modalità lettore di schermo non modifica la [modalità non interattiva](/docs/it/headless) con il flag `-p`. La modalità non interattiva scrive già testo semplice e rimane un'alternativa per lo scripting.

<h2 id="report-an-issue">
  Segnalate un problema
</h2>

Se qualcosa non funziona con il vostro lettore di schermo, ingranditore o terminale, aprite un problema sul [tracker dei problemi di Claude Code](https://github.com/anthropics/claude-code/issues) e menzionate la vostra tecnologia assistiva nel titolo. Includete il vostro sistema operativo, l'applicazione terminale e il nome e la versione della tecnologia assistiva nel rapporto.
