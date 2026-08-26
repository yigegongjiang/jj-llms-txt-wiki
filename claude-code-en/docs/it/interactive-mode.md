> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Modalità interattiva

> Riferimento completo per le scorciatoie da tastiera, le modalità di input e le funzioni interattive nelle sessioni di Claude Code.

<h2 id="keyboard-shortcuts">
  Scorciatoie da tastiera
</h2>

<Note>
  Le scorciatoie da tastiera possono variare a seconda della piattaforma e del terminale. Nel [rendering a schermo intero](/docs/it/fullscreen), premere `?` nel visualizzatore di trascrizione per visualizzare le scorciatoie disponibili lì.

  **Utenti macOS**: Le scorciatoie con il tasto Option/Alt (`Alt+B`, `Alt+F`, `Alt+Y`, `Alt+M`, `Alt+P`) richiedono la configurazione di Option come Meta nel vostro terminale:

  * **iTerm2**: Impostazioni → Profili → Tasti → Generale → impostare il tasto Option sinistro/destro su "Esc+"
  * **Apple Terminal**: Impostazioni → Profili → Tastiera → selezionare "Usa Option come Meta Key"
  * **VS Code**: impostare `"terminal.integrated.macOptionIsMeta": true` nelle impostazioni di VS Code

  Vedere [Configurazione del terminale](/docs/it/terminal-config) per i dettagli.
</Note>

<h3 id="general-controls">
  Controlli generali
</h3>

| Scorciatoia                                           | Descrizione                                                                                                                                                             | Contesto                                                                                                                                                                                                                                                                                                                                                                                                           |
| :---------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+C`                                              | Interrompi, o cancella l'input                                                                                                                                          | Interrompe un'operazione in esecuzione. Se nulla è in esecuzione, il primo pressione cancella l'input del prompt e un secondo pressione esce da Claude Code                                                                                                                                                                                                                                                        |
| `Ctrl+X Ctrl+K`                                       | Termina tutti gli [agenti in background](/docs/it/sub-agents#run-subagents-in-foreground-or-background) in questa sessione. Premere due volte entro 3 secondi per confermare | Controllo agente in background                                                                                                                                                                                                                                                                                                                                                                                     |
| `Ctrl+D`                                              | Esci dalla sessione di Claude Code                                                                                                                                      | Segnale EOF                                                                                                                                                                                                                                                                                                                                                                                                        |
| `Ctrl+G` o `Ctrl+X Ctrl+E`                            | Apri nell'editor di testo predefinito                                                                                                                                   | Modifica il vostro prompt o la risposta personalizzata nell'editor di testo predefinito. `Ctrl+X Ctrl+E` è il binding nativo di readline. Attivare Mostra ultima risposta nell'editor esterno in `/config` per anteporre la risposta precedente di Claude come contesto commentato con `#` sopra il vostro prompt; il blocco di commento viene rimosso quando salvate                                              |
| `Ctrl+L`                                              | Ridisegna lo schermo                                                                                                                                                    | Forza un ridisegno completo del terminale. L'input e la cronologia della conversazione vengono mantenuti. Utilizzate questo per recuperare se la visualizzazione diventa distorta o parzialmente vuota                                                                                                                                                                                                             |
| `Ctrl+O`                                              | Attiva/disattiva il visualizzatore di trascrizione                                                                                                                      | Mostra l'utilizzo e l'esecuzione dettagliati degli strumenti, con un timestamp e il modello utilizzato su ogni messaggio dell'assistente. Inoltre espande le chiamate MCP, che si compattano in una singola riga come "Called slack 3 times" per impostazione predefinita                                                                                                                                          |
| `Ctrl+R`                                              | Ricerca inversa nella cronologia dei comandi                                                                                                                            | Cerca i comandi precedenti in modo interattivo                                                                                                                                                                                                                                                                                                                                                                     |
| `Ctrl+V` o `Cmd+V` (iTerm2) o `Alt+V` (Windows e WSL) | Incolla immagine dagli appunti                                                                                                                                          | Inserisce un chip `[Image #N]` al cursore in modo da poter farvi riferimento posizionalmente nel vostro prompt. Su WSL, sia `Ctrl+V` che `Alt+V` sono associati; utilizzare `Alt+V` se il vostro terminale intercetta `Ctrl+V`                                                                                                                                                                                     |
| `Ctrl+B`                                              | Attività in esecuzione in background                                                                                                                                    | Esegue i comandi bash e gli agenti in background. Gli utenti Tmux premono due volte                                                                                                                                                                                                                                                                                                                                |
| `Ctrl+T`                                              | Attiva/disattiva l'elenco delle attività                                                                                                                                | Mostra o nascondi l'[elenco delle attività di Claude](#task-list) nell'area di stato. Questo non è il visualizzatore di attività in background; utilizzare [`/tasks`](/docs/it/commands) per visualizzare shell e agenti in esecuzione                                                                                                                                                                                  |
| `Frecce sinistra/destra`                              | Cicla attraverso le schede della finestra di dialogo                                                                                                                    | Naviga tra le schede nelle finestre di dialogo dei permessi e nei menu                                                                                                                                                                                                                                                                                                                                             |
| `Frecce su/giù` o `Ctrl+P`/`Ctrl+N`                   | Sposta il cursore o naviga nella cronologia dei comandi                                                                                                                 | Quando l'input si estende su più di una riga visiva, sia avvolta che multilinea, prima sposta il cursore all'interno del prompt. Una volta che il cursore è sulla prima o ultima riga visiva, premere di nuovo naviga nella cronologia dei comandi. A partire dalla v2.1.169, l'input a riga singola avvolto si comporta come l'input multilinea                                                                   |
| `Esc`                                                 | Interrompi Claude, o chiudi una finestra di dialogo                                                                                                                     | Arresta la risposta corrente o la chiamata dello strumento a metà turno in modo da poter reindirizzare. Claude mantiene il lavoro svolto finora. Quando una finestra di dialogo come un prompt di permesso è aperta, `Esc` chiude la finestra di dialogo piuttosto che interrompere Claude. Prima della v2.1.202, `Esc` su alcune finestre di dialogo interrompeva Claude e lasciava la finestra di dialogo aperta |
| `Esc` + `Esc`                                         | Cancella la bozza di input, o riavvolgi                                                                                                                                 | Quando l'input del prompt contiene testo, doppio `Esc` lo cancella e salva la bozza nella cronologia in modo che `Su` la richiami. Quando l'input è vuoto, doppio `Esc` apre il [menu di riavvolgimento](/docs/it/checkpointing) per ripristinare o riassumere il codice e la conversazione da un punto precedente                                                                                                      |
| `Shift+Tab` o `Alt+M` (alcune configurazioni)         | Cicla le modalità di permesso                                                                                                                                           | Cicla attraverso `default` (etichettato Manuale nell'indicatore di modalità), `acceptEdits`, `plan` e qualsiasi modalità abilitata, come `auto` o `bypassPermissions`. Vedere [modalità di permesso](/docs/it/permission-modes).                                                                                                                                                                                        |
| `Option+P` (macOS) o `Alt+P` (Windows/Linux)          | Cambia modello                                                                                                                                                          | Cambia modelli senza cancellare il vostro prompt                                                                                                                                                                                                                                                                                                                                                                   |
| `Option+T` (macOS) o `Alt+T` (Windows/Linux)          | Attiva/disattiva il pensiero esteso                                                                                                                                     | Abilita o disabilita la modalità di pensiero esteso. Non ha effetto su Fable 5, che utilizza sempre il pensiero esteso. A partire dalla v2.1.132 questa scorciatoia funziona su macOS senza configurare Option come Meta                                                                                                                                                                                           |
| `Option+O` (macOS) o `Alt+O` (Windows/Linux)          | Attiva/disattiva la modalità veloce                                                                                                                                     | Abilita o disabilita la [modalità veloce](/docs/it/fast-mode)                                                                                                                                                                                                                                                                                                                                                           |

<h3 id="text-editing">
  Modifica del testo
</h3>

| Scorciatoia             | Descrizione                                      | Contesto                                                                                                                                                                                                                     |
| :---------------------- | :----------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+A`                | Sposta il cursore all'inizio della riga corrente | Nell'input multilinea, sposta all'inizio della riga logica corrente                                                                                                                                                          |
| `Ctrl+E`                | Sposta il cursore alla fine della riga corrente  | Nell'input multilinea, sposta alla fine della riga logica corrente                                                                                                                                                           |
| `Ctrl+K`                | Elimina fino alla fine della riga                | Memorizza il testo eliminato per l'incollamento                                                                                                                                                                              |
| `Ctrl+U`                | Elimina dal cursore all'inizio della riga        | Memorizza il testo eliminato per l'incollamento. Ripetere per cancellare su più righe nell'input multilinea. Su macOS, gli emulatori di terminale inclusi iTerm2 e Terminal.app mappano `Cmd+Backspace` a questa scorciatoia |
| `Ctrl+W`                | Elimina la parola precedente                     | Memorizza il testo eliminato per l'incollamento. Su Windows, `Ctrl+Backspace` elimina anche la parola precedente                                                                                                             |
| `Ctrl+Y`                | Incolla il testo eliminato                       | Incolla il testo eliminato con `Ctrl+K`, `Ctrl+U` o `Ctrl+W`                                                                                                                                                                 |
| `Alt+Y` (dopo `Ctrl+Y`) | Cicla la cronologia degli incollamenti           | Dopo l'incollamento, cicla attraverso il testo precedentemente eliminato. Richiede [Option come Meta](#keyboard-shortcuts) su macOS                                                                                          |
| `Alt+B`                 | Sposta il cursore indietro di una parola         | Navigazione per parole. Richiede [Option come Meta](#keyboard-shortcuts) su macOS                                                                                                                                            |
| `Alt+F`                 | Sposta il cursore in avanti di una parola        | Navigazione per parole. Richiede [Option come Meta](#keyboard-shortcuts) su macOS                                                                                                                                            |

<h3 id="theme-and-display">
  Tema e visualizzazione
</h3>

| Scorciatoia | Descrizione                                                              | Contesto                                                                                                                                         |
| :---------- | :----------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+T`    | Attiva/disattiva l'evidenziazione della sintassi per i blocchi di codice | Funziona solo all'interno del menu di selezione `/theme`. Controlla se il codice nelle risposte di Claude utilizza la colorazione della sintassi |

<h3 id="multiline-input">
  Input multilinea
</h3>

| Metodo                | Scorciatoia          | Contesto                                                                                                  |
| :-------------------- | :------------------- | :-------------------------------------------------------------------------------------------------------- |
| Escape rapido         | `\` + `Enter`        | Funziona in tutti i terminali                                                                             |
| Tasto Option          | `Option+Enter`       | Dopo aver abilitato [Option come Meta](/docs/it/terminal-config#enable-option-key-shortcuts-on-macos) su macOS |
| Shift+Enter           | `Shift+Enter`        | Nativo in iTerm2, WezTerm, Ghostty, Kitty, Warp, Apple Terminal, Windows Terminal                         |
| Sequenza di controllo | `Ctrl+J`             | Funziona in qualsiasi terminale senza configurazione                                                      |
| Modalità incolla      | Incolla direttamente | Per blocchi di codice, log                                                                                |

<Tip>
  Shift+Enter funziona senza configurazione in iTerm2, WezTerm, Ghostty, Kitty, Warp, Apple Terminal e Windows Terminal. Per VS Code, Cursor, Devin Desktop, Alacritty e Zed, eseguire `/terminal-setup` per installare il binding.
</Tip>

<h3 id="quick-commands">
  Comandi rapidi
</h3>

| Scorciatoia    | Descrizione                    | Note                                                                          |
| :------------- | :----------------------------- | :---------------------------------------------------------------------------- |
| `/` all'inizio | Comando o skill                | Vedere [comandi](#commands) e [skills](/docs/it/skills)                            |
| `!` all'inizio | Modalità Bash                  | Esegui i comandi direttamente e aggiungi l'output di esecuzione alla sessione |
| `@`            | Menzione del percorso del file | Attiva l'autocompletamento del percorso del file                              |

<h3 id="transcript-viewer">
  Visualizzatore di trascrizione
</h3>

Quando il visualizzatore di trascrizione è aperto (attivato con `Ctrl+O`), queste scorciatoie sono disponibili. Nel [rendering a schermo intero](/docs/it/fullscreen), premere `?` per visualizzare il pannello di riferimento completo delle scorciatoie all'interno del visualizzatore. `Ctrl+E` può essere riassegnato tramite [`transcript:toggleShowAll`](/docs/it/keybindings).

| Scorciatoia          | Descrizione                                                                                                                                                                                                                                                       |
| :------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `?`                  | Attiva/disattiva il pannello di aiuto delle scorciatoie da tastiera. Richiede il [rendering a schermo intero](/docs/it/fullscreen)                                                                                                                                     |
| `{` / `}`            | Salta al prompt utente precedente o successivo, come il movimento di paragrafo vim. Richiede il [rendering a schermo intero](/docs/it/fullscreen)                                                                                                                      |
| `Ctrl+E`             | Attiva/disattiva mostra tutto il contenuto                                                                                                                                                                                                                        |
| `[`                  | Scrivi la conversazione completa nel scrollback nativo del vostro terminale in modo che `Cmd+F`, la modalità copia di tmux e altri strumenti nativi possano cercarla. Richiede il [rendering a schermo intero](/docs/it/fullscreen#search-and-review-the-conversation) |
| `v`                  | Scrivi la conversazione in un file temporaneo e aprilo in `$VISUAL` o `$EDITOR`. Richiede il [rendering a schermo intero](/docs/it/fullscreen)                                                                                                                         |
| `q`, `Ctrl+C`, `Esc` | Esci dalla visualizzazione della trascrizione. Tutti e tre possono essere riassegnati tramite [`transcript:exit`](/docs/it/keybindings)                                                                                                                                |

<h3 id="voice-input">
  Input vocale
</h3>

| Scorciatoia                   | Descrizione      | Note                                                                                                                                                                                                                        |
| :---------------------------- | :--------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Tieni premuto o tocca `Space` | Dettatura vocale | Richiede che la [dettatura vocale](/docs/it/voice-dictation) sia abilitata. Tieni premuto per registrare, o esegui `/voice tap` per attivare/disattivare al tocco. [Riassegnabile](/docs/it/voice-dictation#rebind-the-dictation-key) |

<h2 id="commands">
  Comandi
</h2>

Digitate `/` in Claude Code per visualizzare tutti i comandi disponibili, oppure digitate `/` seguito da qualsiasi lettera per filtrare. Il menu `/` mostra tutto ciò che potete invocare: comandi integrati, [skills](/docs/it/skills) in bundle e create dagli utenti, e comandi forniti da [plugin](/docs/it/plugins) e [server MCP](/docs/it/mcp#use-mcp-prompts-as-commands). Non tutti i comandi integrati sono visibili a ogni utente poiché alcuni dipendono dalla vostra piattaforma o dal vostro piano.

Nel [rendering a schermo intero](/docs/it/fullscreen#use-the-mouse), il comando `/` e gli elenchi di suggerimento file `@` rispondono anche al mouse: passare il mouse evidenzia una riga e fare clic la accetta.

Vedere il [riferimento dei comandi](/docs/it/commands) per l'elenco completo dei comandi inclusi in Claude Code.

<h2 id="vim-editor-mode">
  Modalità editor Vim
</h2>

Abilitate la modifica in stile vim tramite `/config` → Editor mode.

<h3 id="mode-switching">
  Cambio di modalità
</h3>

| Comando | Azione                                          | Dalla modalità |
| :------ | :---------------------------------------------- | :------------- |
| `Esc`   | Entra in modalità NORMAL                        | INSERT, VISUAL |
| `i`     | Inserisci prima del cursore                     | NORMAL         |
| `I`     | Inserisci all'inizio della riga                 | NORMAL         |
| `a`     | Inserisci dopo il cursore                       | NORMAL         |
| `A`     | Inserisci alla fine della riga                  | NORMAL         |
| `o`     | Apri riga sotto                                 | NORMAL         |
| `O`     | Apri riga sopra                                 | NORMAL         |
| `v`     | Avvia selezione visuale carattere per carattere | NORMAL         |
| `V`     | Avvia selezione visuale riga per riga           | NORMAL         |

<h3 id="remap-insert-mode-key-sequences">
  Rimappare sequenze di tasti in modalità INSERT
</h3>

L'impostazione [`vimInsertModeRemaps`](/docs/it/settings#available-settings) mappa una sequenza di due tasti in modalità INSERT su Escape, quindi una mappatura come `jj` vi riporta in modalità NORMAL. Richiede Claude Code v2.1.208 o successivo.

Il seguente esempio di `~/.claude/settings.json` attiva la modalità vim e mappa `jj` su Escape:

```json theme={null}
{
  "editorMode": "vim",
  "vimInsertModeRemaps": { "jj": "<Esc>" }
}
```

Ogni chiave è esattamente due caratteri stampabili digitati in sequenza, e `"<Esc>"` è l'unico target supportato. Le voci con una lunghezza o un target diverso vengono ignorate.

Digitare il primo carattere di una sequenza lo inserisce normalmente. Premere il secondo carattere entro un secondo rimuove quel carattere in sospeso e passa alla modalità NORMAL, lasciando nessuno dei due caratteri nel vostro input. Dopo la finestra di un secondo, o se segue un tasto diverso, entrambi i caratteri rimangono come testo letterale, quindi potete comunque digitare una parola contenente la sequenza facendo una pausa tra i due tasti.

Claude Code legge questa impostazione dal vostro file di impostazioni utente, dal flag `--settings` e dalle [impostazioni gestite](/docs/it/permissions#managed-settings) solo. Le voci nel `.claude/settings.json` o `.claude/settings.local.json` di un progetto vengono ignorate, quindi un repository estratto non può rimappare i vostri tasti.

<h3 id="navigation-normal-mode">
  Navigazione (modalità NORMAL)
</h3>

| Comando         | Azione                                                                                                                                                                                    |
| :-------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `h`/`j`/`k`/`l` | Sposta sinistra/giù/su/destra                                                                                                                                                             |
| `Space`         | Sposta a destra                                                                                                                                                                           |
| `w`             | Parola successiva                                                                                                                                                                         |
| `e`             | Fine della parola                                                                                                                                                                         |
| `b`             | Parola precedente                                                                                                                                                                         |
| `0`             | Inizio della riga                                                                                                                                                                         |
| `$`             | Fine della riga                                                                                                                                                                           |
| `^`             | Primo carattere non vuoto                                                                                                                                                                 |
| `gg`            | Inizio dell'input                                                                                                                                                                         |
| `G`             | Fine dell'input                                                                                                                                                                           |
| `f{char}`       | Salta alla prossima occorrenza del carattere                                                                                                                                              |
| `F{char}`       | Salta alla precedente occorrenza del carattere                                                                                                                                            |
| `t{char}`       | Salta appena prima della prossima occorrenza del carattere                                                                                                                                |
| `T{char}`       | Salta appena dopo la precedente occorrenza del carattere                                                                                                                                  |
| `;`             | Ripeti l'ultimo movimento f/F/t/T                                                                                                                                                         |
| `,`             | Ripeti l'ultimo movimento f/F/t/T in ordine inverso                                                                                                                                       |
| `/`             | Apri ricerca cronologia inversa, come `Ctrl+R`. A partire dalla v2.1.191, il prompt di ricerca vuoto mostra un suggerimento: premete `Esc` poi `i` poi `/` per aprire il menu dei comandi |

<Note>
  In modalità normale vim, se il cursore è all'inizio o alla fine dell'input e non può muoversi ulteriormente, `j`/`k` e i tasti freccia navigano nella cronologia dei comandi.
</Note>

<h3 id="editing-normal-mode">
  Modifica (modalità NORMAL)
</h3>

| Comando        | Azione                                 |
| :------------- | :------------------------------------- |
| `x`            | Elimina carattere                      |
| `dd`           | Elimina riga                           |
| `D`            | Elimina fino alla fine della riga      |
| `dw`/`de`/`db` | Elimina parola/fino alla fine/indietro |
| `cc`           | Cambia riga                            |
| `C`            | Cambia fino alla fine della riga       |
| `cw`/`ce`/`cb` | Cambia parola/fino alla fine/indietro  |
| `yy`/`Y`       | Copia (yank) riga                      |
| `yw`/`ye`/`yb` | Copia parola/fino alla fine/indietro   |
| `p`            | Incolla dopo il cursore                |
| `P`            | Incolla prima del cursore              |
| `>>`           | Indenta riga                           |
| `<<`           | Dedenta riga                           |
| `J`            | Unisci righe                           |
| `u`            | Annulla                                |
| `.`            | Ripeti l'ultimo cambiamento            |

<h3 id="text-objects-normal-mode">
  Oggetti di testo (modalità NORMAL)
</h3>

Gli oggetti di testo funzionano con operatori come `d`, `c` e `y`:

| Comando   | Azione                                       |
| :-------- | :------------------------------------------- |
| `iw`/`aw` | Parola interna/intorno                       |
| `iW`/`aW` | PAROLA interna/intorno (delimitata da spazi) |
| `i"`/`a"` | Interno/intorno a virgolette doppie          |
| `i'`/`a'` | Interno/intorno a virgolette singole         |
| `i(`/`a(` | Interno/intorno a parentesi tonde            |
| `i[`/`a[` | Interno/intorno a parentesi quadre           |
| `i{`/`a{` | Interno/intorno a parentesi graffe           |

<h3 id="visual-mode">
  Modalità visuale
</h3>

Premete `v` per la selezione carattere per carattere o `V` per la selezione riga per riga. I movimenti estendono la selezione e gli operatori agiscono direttamente su di essa.

| Comando          | Azione                                                               |
| :--------------- | :------------------------------------------------------------------- |
| `d`/`x`          | Elimina selezione                                                    |
| `y`              | Copia selezione                                                      |
| `c`/`s`          | Cambia selezione                                                     |
| `p`              | Sostituisci selezione con il contenuto del registro                  |
| `r{char}`        | Sostituisci ogni carattere selezionato con `{char}`                  |
| `~`/`u`/`U`      | Attiva/disattiva, minuscole o maiuscole selezione                    |
| `>`/`<`          | Indenta o dedenta le righe selezionate                               |
| `J`              | Unisci le righe selezionate                                          |
| `o`              | Scambia cursore e ancoraggio                                         |
| `iw`/`aw`/`i"`/… | Seleziona un oggetto di testo                                        |
| `v`/`V`          | Attiva/disattiva tra carattere per carattere e riga per riga, o esci |

La modalità visuale blocco con `Ctrl+V` non è supportata.

<h2 id="command-history">
  Cronologia dei comandi
</h2>

Claude Code mantiene la cronologia dei comandi per la sessione corrente:

* La cronologia dell'input viene memorizzata per directory di lavoro
* La cronologia dell'input si ripristina quando eseguite `/clear` per avviare una nuova sessione. La conversazione della sessione precedente viene preservata e può essere ripresa.
* Inviare lo stesso prompt due volte di seguito registra una voce di cronologia, quindi premere Su passa al prompt distinto precedente
* Utilizzate i tasti freccia su/giù per navigare (vedere le scorciatoie da tastiera sopra)
* L'espansione della cronologia con `!` è disabilitata per impostazione predefinita

<h3 id="reverse-search-with-ctrl-r">
  Ricerca inversa con Ctrl+R
</h3>

Premete `Ctrl+R` per cercare in modo interattivo nella vostra cronologia dei comandi:

1. **Avvia ricerca**: premete `Ctrl+R` per attivare la ricerca inversa nella cronologia
2. **Digita query**: inserite il testo da cercare nei comandi precedenti. Il termine di ricerca è evidenziato nei risultati corrispondenti
3. **Naviga tra i risultati**: premete `Ctrl+R` di nuovo per scorrere i risultati più vecchi
4. **Cambia ambito**: la ricerca è impostata per impostazione predefinita su prompt da tutti i progetti. Premete `Ctrl+S` per alternare l'ambito tra questa sessione, questo progetto e tutti i progetti
5. **Accetta il risultato**:
   * Premete `Tab` o `Esc` per accettare il risultato corrente e continuare a modificare
   * Premete `Enter` per accettare ed eseguire il comando immediatamente
6. **Annulla ricerca**:
   * Premete `Ctrl+C` per annullare e ripristinare l'input originale
   * Premete `Backspace` su una ricerca vuota per annullare

La ricerca carica i 100 prompt univoci più recenti nell'ambito selezionato, con i duplicati compressi nell'occorrenza più recente. I prompt corrispondenti vengono visualizzati con il termine di ricerca evidenziato, in modo da poter trovare e riutilizzare gli input precedenti.

Accettare un risultato o annullare la ricerca ha effetto immediato, anche mentre Claude Code sta ancora caricando la cronologia. Prima della versione 2.1.202, accettare o annullare durante quel caricamento poteva segnalare un errore interno.

<h2 id="background-bash-commands">
  Comandi bash in background
</h2>

Claude Code supporta l'esecuzione di comandi bash in background, consentendovi di continuare a lavorare mentre i processi a lunga esecuzione vengono eseguiti.

<h3 id="how-backgrounding-works">
  Come funziona l'esecuzione in background
</h3>

Quando Claude Code esegue un comando in background, esegue il comando in modo asincrono e restituisce immediatamente un ID di attività in background. Claude Code può rispondere a nuovi prompt mentre il comando continua a essere eseguito in background.

Per eseguire i comandi in background, potete:

* Chiedere a Claude Code di eseguire un comando in background
* Premere `Ctrl+B` per spostare una normale invocazione dello strumento Bash in background. Gli utenti Tmux devono premere `Ctrl+B` due volte a causa del tasto di prefisso di tmux.

**Caratteristiche principali:**

* L'output viene scritto in un file e Claude può recuperarlo utilizzando lo strumento Read
* Le attività in background hanno ID univoci per il tracciamento e il recupero dell'output
* Le attività in background vengono pulite automaticamente quando Claude Code esce. L'esecuzione in background della sessione invece di uscire le affida alla sessione in background, dove continuano a essere eseguite. Vedere [esecuzione in background di una sessione in esecuzione](/docs/it/agent-view#from-inside-a-session)
* Le attività in background vengono terminate automaticamente se l'output supera 5GB, con una nota in stderr che spiega il motivo
* A partire dalla v2.1.193, su macOS e Linux, le attività in background in esecuzione vengono terminate quando il sistema operativo segnala pressione della memoria, a condizione che la sessione sia rimasta inattiva per almeno 30 minuti senza alcun turno o subagent in esecuzione. Impostare [`CLAUDE_CODE_DISABLE_BG_SHELL_PRESSURE_REAP`](/docs/it/env-vars) su `1` per disattivare questa funzione

Per disabilitare tutta la funzionalità di attività in background, impostare la variabile di ambiente `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` su `1`. Vedere [Variabili di ambiente](/docs/it/env-vars) per i dettagli.

**Comandi comunemente eseguiti in background:**

* Strumenti di build (webpack, vite, make)
* Gestori di pacchetti (npm, yarn, pnpm)
* Test runner (jest, pytest)
* Server di sviluppo
* Processi a lunga esecuzione (docker, terraform)

<h3 id="shell-mode-with-prefix">
  Modalità shell con prefisso `!`
</h3>

Eseguite i comandi shell direttamente senza passare per Claude aggiungendo il prefisso `!` al vostro input:

```bash theme={null}
! npm test
! git status
! ls -la
```

Modalità shell:

* Aggiunge il comando e il suo output al contesto della conversazione
* Mostra l'avanzamento e l'output in tempo reale
* Supporta lo stesso backgrounding `Ctrl+B` per i comandi a lunga esecuzione
* Non richiede a Claude di interpretare o approvare il comando
* Supporta l'autocompletamento basato sulla cronologia: digitate un comando parziale e premete `Tab` per completare dai comandi `!` precedenti nel progetto corrente
* A partire dalla v2.1.193, supporta l'autocompletamento del percorso file in tempo reale su tutte le piattaforme: digitate un token contenente una barra in avanti, come `./src/` o `~/`, per visualizzare un elenco a discesa dei file e delle directory corrispondenti, quindi premete `Tab` per accettare. Utilizzate barre in avanti anche su Windows; l'elenco a discesa viene attivato da `/`, non da `\`
* Esci con `Escape`, `Backspace` o `Ctrl+U` su un prompt vuoto
* Incollare il testo che inizia con `!` in un prompt vuoto entra automaticamente in modalità shell, corrispondendo al comportamento digitato `!`

A partire dalla v2.1.186, Claude risponde automaticamente all'output del comando una volta che arriva nella trascrizione, quindi potete eseguire `! npm test` e ottenere una spiegazione degli errori senza un secondo prompt. La risposta costa lo stesso dell'invio di un prompt normale. Per ripristinare il comportamento precedente in cui l'output viene aggiunto al contesto senza una risposta, impostare [`respondToBashCommands`](/docs/it/settings#available-settings) su `false` in `settings.json`. Prima della v2.1.186, la modalità shell aggiungeva sempre l'output al contesto senza una risposta.

Questo è utile per le operazioni shell rapide mantenendo il contesto della conversazione.

<h2 id="prompt-suggestions">
  Suggerimenti di prompt
</h2>

Quando aprite una sessione per la prima volta, un comando di esempio in grigio appare nell'input del prompt per aiutarvi a iniziare. Claude Code lo sceglie dalla cronologia git del vostro progetto, quindi riflette i file su cui avete lavorato di recente.

Dopo che Claude risponde, i suggerimenti continuano ad apparire in base alla vostra cronologia di conversazione, come un passaggio di follow-up da una richiesta in più parti o una continuazione naturale del vostro flusso di lavoro.

* Premete `Tab` o `Freccia destra` per inserire il suggerimento nell'input del prompt, quindi `Invio` per inviare
* Iniziate a digitare per dismissarlo

Il suggerimento viene eseguito come una richiesta in background che riutilizza la cache del prompt della conversazione padre, quindi il costo aggiuntivo è minimo. Claude Code salta la generazione di suggerimenti quando la cache è fredda per evitare costi inutili.

I suggerimenti vengono automaticamente saltati dopo il primo turno di una conversazione e in Plan Mode. In modalità print sono disabilitati per impostazione predefinita. Passate [`--prompt-suggestions`](/docs/it/cli-reference#cli-flags) con `--output-format stream-json --verbose` per emettere un messaggio `prompt_suggestion` dopo ogni turno invece.

Per disabilitare completamente i suggerimenti di prompt, impostare la variabile di ambiente o attivare/disattivare l'impostazione in `/config`:

```bash theme={null}
export CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=false
```

<h2 id="side-questions-with-/btw">
  Domande laterali con /btw
</h2>

Utilizzate `/btw` per fare una domanda rapida sul vostro lavoro corrente senza aggiungerla alla cronologia della conversazione. Questo è utile quando volete una risposta veloce ma non volete ingombrare il contesto principale o far deviare Claude da un'attività a lunga esecuzione.

```
/btw what was the name of that config file again?
```

Le domande laterali hanno piena visibilità nella conversazione corrente, quindi potete chiedere informazioni sul codice che Claude ha già letto, sulle decisioni che ha preso in precedenza, o su qualsiasi altra cosa della sessione. La domanda e la risposta sono effimere: appaiono in un overlay dismissibile e non entrano mai nella cronologia della conversazione.

* **Disponibile mentre Claude sta lavorando**: potete eseguire `/btw` anche mentre Claude sta elaborando una risposta. La domanda laterale viene eseguita in modo indipendente e non interrompe il turno principale.
* **Nessun accesso agli strumenti**: le domande laterali rispondono solo da ciò che è già nel contesto. Claude non può leggere file, eseguire comandi o cercare quando risponde a una domanda laterale.
* **Risposta singola**: non ci sono turni di follow-up nell'overlay. Per continuare il thread, trasformatelo in una propria sessione con `f`.
* **Costo basso**: la domanda laterale riutilizza la cache del prompt della conversazione padre, quindi il costo aggiuntivo è minimo.

Le domande laterali precedenti della stessa sessione appaiono come un elenco attenuato sopra la risposta corrente. Rimangono fuori dalla cronologia della conversazione ma restano visibili nell'overlay finché non le cancellate.

Una volta che la risposta appare, l'overlay accetta questi tasti.

| Tasto                      | Azione                                                                                                                                                                                                                                                                                                                           |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Space`, `Enter`, `Escape` | Dismissere la risposta e tornare al prompt                                                                                                                                                                                                                                                                                       |
| `Up` / `Down`              | Scorrere la risposta                                                                                                                                                                                                                                                                                                             |
| `Left` / `Right`           | Passare tra questa risposta e le vostre risposte `/btw` precedenti della sessione. `Left` si sposta verso risposte più vecchie e `Right` ritorna verso quella corrente. Richiede Claude Code v2.1.187 o successivo                                                                                                               |
| `c`                        | Copiare la risposta negli appunti come Markdown grezzo. Utilizzate questo invece della selezione del mouse, che cattura il rendering del terminale con ritorno a capo forzato anziché il testo sorgente                                                                                                                          |
| `f`                        | Trasformare in una nuova sessione. La nuova sessione eredita la conversazione padre più questa domanda e risposta come turni di trascrizione reali, quindi potete continuare con accesso completo agli strumenti. La sessione originale viene preservata sotto [`/resume`](/docs/it/commands). Disponibile solo nelle sessioni locali |
| `x`                        | Cancellare l'elenco degli scambi `/btw` precedenti mostrati sopra la risposta corrente                                                                                                                                                                                                                                           |

`/btw` è l'inverso di un [subagent](/docs/it/sub-agents): vede la vostra conversazione completa ma non ha strumenti, mentre un subagent ha strumenti completi ma inizia con un contesto vuoto. Utilizzate `/btw` per chiedere informazioni su ciò che Claude già conosce da questa sessione; utilizzate un subagent per scoprire qualcosa di nuovo.

<h2 id="task-list">
  Elenco delle attività
</h2>

L'elenco delle attività è la lista di controllo di Claude: elementi che Claude ha creato per pianificare lavori multi-step, con indicatori che mostrano cosa è in sospeso, in corso o completato. È separato dalla visualizzazione delle attività in background. Per visualizzare shell in esecuzione e subagent, utilizzare [`/tasks`](/docs/it/commands) invece.

* Premete `Ctrl+T` per attivare/disattivare la visualizzazione dell'elenco delle attività. La visualizzazione mostra fino a cinque attività alla volta. Quando Claude non ha ancora creato elementi della lista di controllo, l'interruttore non ha effetto visibile perché non c'è nulla da visualizzare
* Per visualizzare tutte le attività o cancellarle, chiedete direttamente a Claude: "show me all tasks" o "clear all tasks"
* Le attività persistono attraverso i compattamenti del contesto, aiutando Claude a rimanere organizzato su progetti più grandi
* Per condividere un elenco di attività tra sessioni, impostare `CLAUDE_CODE_TASK_LIST_ID` per utilizzare una directory denominata in `~/.claude/tasks/`: `CLAUDE_CODE_TASK_LIST_ID=my-project claude`

<h2 id="session-recap">
  Riepilogo della sessione
</h2>

Quando tornate al terminale dopo esservi allontanati, Claude Code mostra un riepilogo di una riga di ciò che è accaduto nella sessione finora. Il riepilogo viene generato in background una volta che sono trascorsi almeno tre minuti dall'ultimo turno completato e il terminale non è a fuoco, quindi è pronto quando tornate. I riepiloghi appaiono solo una volta che la sessione ha almeno tre turni, e mai due volte di seguito.

Eseguite `/recap` per generare un riepilogo su richiesta. Per disattivare i riepiloghi automatici, aprite `/config` e disabilitate **Session recap**.

Il riepilogo della sessione è abilitato per impostazione predefinita per ogni piano e provider. Il riepilogo viene sempre saltato in modalità non interattiva.

<h2 id="pr-review-status">
  Stato della revisione PR
</h2>

Quando lavorate su un ramo con una pull request aperta, Claude Code visualizza un collegamento PR cliccabile nel footer, ad esempio "PR #446". Il collegamento ha una sottolineatura colorata che indica lo stato della revisione:

* Verde: approvato
* Giallo: revisione in sospeso
* Rosso: modifiche richieste
* Grigio: bozza

Il badge scompare una volta che la pull request viene unita o chiusa. `Cmd+click` (macOS) o `Ctrl+click` (Windows/Linux) sul collegamento per aprire la pull request nel vostro browser. Lo stato si aggiorna ogni 60 secondi e immediatamente dopo l'esecuzione di un comando `gh pr` o `git push` nella sessione.

<Note>
  Lo stato PR richiede che la CLI `gh` sia installata e autenticata (`gh auth login`).
</Note>

<h2 id="see-also">
  Vedere anche
</h2>

* [Skills](/docs/it/skills) - Prompt personalizzati e flussi di lavoro
* [Checkpointing](/docs/it/checkpointing) - Riavvolgi le modifiche di Claude e ripristina gli stati precedenti
* [Riferimento CLI](/docs/it/cli-reference) - Flag e opzioni della riga di comando
* [Impostazioni](/docs/it/settings) - Opzioni di configurazione
* [Gestione della memoria](/docs/it/memory) - Gestione dei file CLAUDE.md
