> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Modalità interattiva

> Riferimento completo per le scorciatoie da tastiera, le modalità di input e le funzioni interattive nelle sessioni di Claude Code.

<h2 id="keyboard-shortcuts">
  Scorciatoie da tastiera
</h2>

<Note>
  Le scorciatoie da tastiera possono variare a seconda della piattaforma e del terminale. Nel [rendering a schermo intero](/docs/it/fullscreen), premi `?` nel visualizzatore della trascrizione per vedere le scorciatoie disponibili lì.

  **Utenti macOS**: Le scorciatoie del tasto Option/Alt (`Alt+B`, `Alt+F`, `Alt+D`, `Alt+Y`, `Alt+P`) richiedono la configurazione di Option come Meta nel tuo terminale. Vedi [Abilita le scorciatoie del tasto Option su macOS](/docs/it/terminal-config#enable-option-key-shortcuts-on-macos) per l'impostazione in ogni terminale.
</Note>

<h3 id="general-controls">
  Controlli generali
</h3>

| Scorciatoia                                                                                        | Descrizione                                                                                                                                                                                                                                                                                                    | Contesto                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Ctrl+C`                                                                                           | Interrompi, o cancella l'input                                                                                                                                                                                                                                                                                 | Interrompe un'operazione in esecuzione. Se nulla è in esecuzione, il primo pressione cancella l'input del prompt e una seconda pressione esce da Claude Code                                                                                                                                                                                                                                                                                                                                                                                                  |
| `Ctrl+X Ctrl+K`                                                                                    | Arresta tutti i [subagent in background](/docs/it/sub-agents#run-subagents-in-foreground-or-background) in questa sessione e disattiva le [risposte automatiche degli artefatti](/docs/it/artifacts#let-claude-reply-to-comments-on-its-own) per il resto della sessione. Premi due volte entro 3 secondi per confermare | Controllo subagent                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `Ctrl+D`                                                                                           | Esci dalla sessione Claude Code                                                                                                                                                                                                                                                                                | Il primo pressione mostra un suggerimento di conferma e un secondo pressione entro 800ms esce. Quando il prompt contiene testo, `Ctrl+D` elimina il carattere dopo il cursore invece                                                                                                                                                                                                                                                                                                                                                                          |
| `Ctrl+G` o `Ctrl+X Ctrl+E`                                                                         | Apri nell'editor di testo predefinito                                                                                                                                                                                                                                                                          | Modifica il tuo prompt o la risposta personalizzata nell'editor di testo predefinito. `Ctrl+X Ctrl+E` è il binding nativo di readline. Attiva **Mostra l'ultima risposta nell'editor esterno** in `/config` per anteporre la risposta precedente di Claude come contesto commentato con `#` sopra il tuo prompt; Claude Code rimuove il blocco di commento quando salvi                                                                                                                                                                                       |
| `Ctrl+L`                                                                                           | Ridisegna lo schermo                                                                                                                                                                                                                                                                                           | Forza un ridisegno completo del terminale, mantenendo l'input e la cronologia della conversazione. Usa questo per recuperare se il display diventa distorto o parzialmente vuoto. Vedi [Cancella la conversazione](/docs/it/fullscreen#clear-the-conversation) per il rendering a schermo intero                                                                                                                                                                                                                                                                   |
| `Ctrl+O`                                                                                           | Attiva/disattiva il visualizzatore della trascrizione                                                                                                                                                                                                                                                          | Mostra l'utilizzo dettagliato degli strumenti e l'esecuzione, con un timestamp e il modello utilizzato su ogni messaggio dell'assistente. Espande anche le righe che si comprimono per impostazione predefinita, come le chiamate MCP, mostrate come una singola riga `Called slack 3 times`, e i [messaggi dalle tue altre sessioni](/docs/it/cross-session-messaging#what-a-message-looks-like), mostrati come un'anteprima `Message from @<sender>` su una riga                                                                                                 |
| `Ctrl+R`                                                                                           | Ricerca inversa nella cronologia dei comandi                                                                                                                                                                                                                                                                   | Cerca attraverso i comandi precedenti in modo interattivo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `Ctrl+V` o `Cmd+V` (iTerm2) o `Alt+V` (Windows e WSL)                                              | Incolla l'immagine dagli appunti                                                                                                                                                                                                                                                                               | Inserisce un chip `[Image #N]` al cursore in modo da potervi fare riferimento posizionalmente nel tuo prompt. Su WSL, sia `Ctrl+V` che `Alt+V` sono associati; usa `Alt+V` se il tuo terminale intercetta `Ctrl+V`                                                                                                                                                                                                                                                                                                                                            |
| `Ctrl+B`                                                                                           | Attività in esecuzione in background                                                                                                                                                                                                                                                                           | Mette in background i comandi Bash e gli agenti. Gli utenti di Tmux premono due volte                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `Ctrl+T`                                                                                           | Attiva/disattiva la lista di controllo delle attività di Claude                                                                                                                                                                                                                                                | Mostra o nascondi la [lista di controllo delle attività di Claude](#task-list) nell'area di stato. Questo non è il visualizzatore delle attività in background; usa [`/tasks`](/docs/it/commands) per vedere le shell e i subagent in esecuzione                                                                                                                                                                                                                                                                                                                   |
| `Ctrl+S`                                                                                           | Nascondi o ripristina il prompt                                                                                                                                                                                                                                                                                | Con testo nell'input, lo nasconde e cancella il prompt. Premuto di nuovo su un prompt vuoto, ripristina il testo nascosto, la posizione del cursore, il contenuto incollato e la modalità di input, quindi un `!` nascosto [comando shell](#shell-mode-with-prefix) ritorna in modalità shell                                                                                                                                                                                                                                                                 |
| `Ctrl+Z`                                                                                           | Sospendi Claude Code                                                                                                                                                                                                                                                                                           | Solo Unix. Sospende il processo alla tua shell; esegui `fg` per riprendere                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `Frecce sinistra/destra`                                                                           | Cicla attraverso le schede della finestra di dialogo                                                                                                                                                                                                                                                           | Naviga tra le schede nelle finestre di dialogo dei permessi e nei menu                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `Tab`                                                                                              | Accetta un suggerimento di completamento automatico, o aggiungi un commento a una risposta di permesso                                                                                                                                                                                                         | Mentre i suggerimenti di completamento automatico vengono visualizzati nell'input del prompt, accetta il suggerimento selezionato. Sulla maggior parte dei prompt di permesso, con **Sì** o **No** focalizzato, apre un campo di commento su quell'opzione e premendolo di nuovo chiude il campo. Vedi [aggiungi un commento quando rispondi a un prompt di permesso](/docs/it/permissions#add-a-comment-when-you-answer-a-permission-prompt)                                                                                                                      |
| `Frecce su/giù` o `Ctrl+P`/`Ctrl+N`                                                                | Sposta il cursore o naviga nella cronologia dei comandi                                                                                                                                                                                                                                                        | Quando l'input si estende su più di una riga visiva, sia avvolta che multilinea, prima sposta il cursore all'interno del prompt. Una volta che il cursore è sulla prima o ultima riga visiva, premendo di nuovo naviga nella cronologia dei comandi. Mentre hai messaggi in coda, `Su` dalla prima riga invece [li riporta indietro](#take-back-what-you-queued)                                                                                                                                                                                              |
| `Esc`                                                                                              | Interrompi Claude, o chiudi una finestra di dialogo                                                                                                                                                                                                                                                            | Arresta la risposta corrente o la chiamata dello strumento a metà turno in modo da poter reindirizzare. Claude mantiene il lavoro svolto finora. Se hai [messaggi in coda](#queue-messages-while-claude-works), Claude Code li invia dopo. Quando una finestra di dialogo è aperta, `Esc` chiude la finestra di dialogo. Su un prompt di permesso, `Esc` rifiuta l'azione, lo stesso di [**No** senza un commento](/docs/it/permissions#add-a-comment-when-you-answer-a-permission-prompt)                                                                         |
| `Esc` + `Esc`                                                                                      | Cancella la bozza di input, o riavvolgi                                                                                                                                                                                                                                                                        | Quando l'input del prompt contiene testo, doppio `Esc` lo cancella e salva la bozza nella cronologia in modo che `Su` la richiami. Quando l'input è vuoto, doppio `Esc` apre il [menu di riavvolgimento](/docs/it/checkpointing) per ripristinare o riassumere il codice e la conversazione da un punto precedente                                                                                                                                                                                                                                                 |
| `Ctrl+Enter` o `Ctrl+X Ctrl+S`                                                                     | Invia i messaggi in coda adesso                                                                                                                                                                                                                                                                                | Invia i tuoi [messaggi in coda](#queue-messages-while-claude-works), e la tua bozza con loro, subito. [Quando Claude Code invia quello che hai messo in coda](#when-claude-code-sends-what-you-queued) copre cosa succede al turno su cui Claude sta lavorando. Nella [modalità shell](#shell-mode-with-prefix), il tasto mette in coda il tuo comando. Nei terminali che non segnalano tasti estesi, `Ctrl+Enter` arriva come semplice `Enter`; `Ctrl+X Ctrl+S` funziona in qualsiasi terminale. Richiede Claude Code v2.1.275 o versione successiva         |
| `Shift+Tab`, o `Alt+M` su Windows quando il runtime Node o Bun non abilita la modalità di input VT | Cicla le modalità di permesso                                                                                                                                                                                                                                                                                  | Cicla attraverso `default` (etichettato Manual nell'indicatore di modalità), `acceptEdits`, `plan`, e, quando disponibile, `bypassPermissions` e poi `auto`. Da `auto`, il primo pressione passa a `default`. Vedi [modalità di permesso](/docs/it/permission-modes). Su un prompt di permesso del file, la stessa chiave chiude un [campo di commento](/docs/it/permissions#add-a-comment-when-you-answer-a-permission-prompt) aperto. Senza campo aperto, seleziona l'opzione che consente l'azione per il resto della sessione, quando il prompt offre quell'opzione |
| `Option+P` (macOS) o `Alt+P` (Windows/Linux)                                                       | Cambia modello                                                                                                                                                                                                                                                                                                 | Cambia modelli senza cancellare il tuo prompt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `Option+T` (macOS) o `Alt+T` (Windows/Linux)                                                       | Attiva/disattiva il pensiero esteso                                                                                                                                                                                                                                                                            | Abilita o disabilita la modalità di pensiero esteso. Non ha effetto su Opus 5.5 o i modelli Fable, che utilizzano sempre il pensiero esteso. Funziona su macOS senza configurare Option come Meta                                                                                                                                                                                                                                                                                                                                                             |
| `Option+O` (macOS) o `Alt+O` (Windows/Linux)                                                       | Attiva/disattiva la modalità veloce                                                                                                                                                                                                                                                                            | Abilita o disabilita la [modalità veloce](/docs/it/fast-mode)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |

<h3 id="text-editing">
  Modifica del testo
</h3>

| Scorciatoia               | Descrizione                                         | Contesto                                                                                                                                                                                                                        |
| :------------------------ | :-------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Ctrl+A`                  | Sposta il cursore all'inizio della riga corrente    | Nell'input multilinea, sposta all'inizio della riga logica corrente                                                                                                                                                             |
| `Ctrl+E`                  | Sposta il cursore alla fine della riga corrente     | Nell'input multilinea, sposta alla fine della riga logica corrente                                                                                                                                                              |
| `Ctrl+K`                  | Elimina fino alla fine della riga                   | Memorizza il testo eliminato per l'incollamento                                                                                                                                                                                 |
| `Ctrl+U`                  | Elimina dal cursore all'inizio della riga           | Memorizza il testo eliminato per l'incollamento. Ripeti per cancellare su più righe nell'input multilinea. Su macOS, gli emulatori di terminale inclusi iTerm2 e Terminal.app mappano `Cmd+Backspace` a questa scorciatoia      |
| `Ctrl+W`                  | Elimina indietro fino allo spazio bianco precedente | Memorizza il testo eliminato per l'incollamento. Un pressione rimuove un intero percorso o `--flag=value`. Per eliminare solo la parola precedente, premi `Option+Delete` su macOS o `Ctrl+Backspace` su Windows                |
| `Ctrl+Y`                  | Incolla il testo eliminato                          | Incolla il testo che hai eliminato per ultimo con una delle scorciatoie di eliminazione di parole o righe, come `Ctrl+K`, `Ctrl+U`, o `Ctrl+W`                                                                                  |
| `Alt+Y` (dopo `Ctrl+Y`)   | Cicla la cronologia degli incollamenti              | Dopo l'incollamento, cicla attraverso il testo eliminato in precedenza. Richiede [Option come Meta](#keyboard-shortcuts) su macOS                                                                                               |
| `Alt+B`                   | Sposta il cursore indietro di una parola            | Navigazione delle parole. Richiede [Option come Meta](#keyboard-shortcuts) su macOS                                                                                                                                             |
| `Alt+F`                   | Sposta il cursore in avanti di una parola           | Si sposta alla fine della parola corrente, o alla fine della parola successiva quando il cursore è tra le parole. Richiede [Option come Meta](#keyboard-shortcuts) su macOS                                                     |
| `Alt+D`                   | Elimina fino alla fine della parola                 | Elimina fino alla fine della parola corrente, o alla fine della parola successiva quando il cursore è tra le parole. Memorizza il testo eliminato per l'incollamento. Richiede [Option come Meta](#keyboard-shortcuts) su macOS |
| `Ctrl+_` o `Ctrl+Shift+-` | Annulla l'ultima modifica dell'input                | Ripristina il testo di input precedente e la posizione del cursore                                                                                                                                                              |

<h3 id="make-ctrl-w-delete-back-to-whitespace">
  Limiti delle parole nelle scorciatoie di modifica
</h3>

Le scorciatoie delle parole `Alt+B`, `Alt+F`, `Alt+D`, `Option+Delete`, e `Ctrl+Backspace` trattano una parola come una sequenza di lettere e cifre, quindi la punteggiatura come `_`, `.`, e `/` separa le parole. Con `src/utils/foo.ts` nel prompt, i pressioni ripetuti di `Alt+B` si fermano all'inizio di `ts`, `foo`, `utils`, e `src`.

`Ctrl+W` è diverso: ignora la punteggiatura e elimina indietro fino allo spazio bianco precedente, quindi un pressione rimuove tutto `src/utils/foo.ts`.

Nel testo scritto senza spazi, come il cinese o il giapponese, le scorciatoie delle parole si muovono o eliminano comunque una parola alla volta.

Queste convenzioni di readline si applicano in Claude Code v2.1.261 e versioni successive. L'impostazione [`keybindingFlavor`](/docs/it/settings-reference#keybindingflavor) che le ha attivate nelle versioni precedenti è deprecata e non ha effetto.

Non puoi rimappare queste scorciatoie nel [file di configurazione delle scorciatoie da tastiera](/docs/it/keybindings), che non ha azioni per loro.

<h3 id="theme-and-display">
  Tema e visualizzazione
</h3>

| Scorciatoia | Descrizione                                                              | Contesto                                                                                                                                         |
| :---------- | :----------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+T`    | Attiva/disattiva l'evidenziazione della sintassi per i blocchi di codice | Funziona solo all'interno del menu di selezione `/theme`. Controlla se il codice nelle risposte di Claude utilizza la colorazione della sintassi |

<h3 id="multiline-input">
  Input multilinea
</h3>

| Metodo                | Scorciatoia          | Contesto                                                                                                                                                                                |
| :-------------------- | :------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Escape rapido         | `\` + `Enter`        | Funziona in tutti i terminali                                                                                                                                                           |
| Tasto Option          | `Option+Enter`       | Dopo aver abilitato [Option come Meta](/docs/it/terminal-config#enable-option-key-shortcuts-on-macos) su macOS                                                                               |
| Shift+Enter           | `Shift+Enter`        | Nativo in iTerm2, WezTerm, Ghostty, Kitty, Warp, Apple Terminal, Windows Terminal. Per altri terminali, vedi [Inserisci prompt multilinea](/docs/it/terminal-config#enter-multiline-prompts) |
| Sequenza di controllo | `Ctrl+J`             | Funziona in qualsiasi terminale senza configurazione                                                                                                                                    |
| Modalità incolla      | Incolla direttamente | Per blocchi di codice, log                                                                                                                                                              |

<h3 id="quick-commands">
  Comandi rapidi
</h3>

| Scorciatoia        | Descrizione                                             | Note                                                                                                                                                                                                                                                                                                                                                                                                        |
| :----------------- | :------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/` all'inizio     | Comando o skill                                         | Vedi [comandi](#commands) e [skills](/docs/it/skills)                                                                                                                                                                                                                                                                                                                                                            |
| `!` all'inizio     | Modalità shell                                          | Esegui un comando direttamente, aggiungi il suo output alla sessione e fai rispondere Claude                                                                                                                                                                                                                                                                                                                |
| `@`                | Menzione del percorso del file                          | Attiva il completamento automatico del percorso del file. Nelle sessioni con [messaggistica tra sessioni](/docs/it/cross-session-messaging#message-another-session), quando digiti almeno una lettera dopo `@`, Claude Code suggerisce anche le tue altre sessioni live su questa macchina, in modo da poter dire a Claude di messaggiare quella che scegli. Richiede Claude Code v2.1.232 o versione successiva |
| `:`                | Codice shortcode emoji                                  | Digita un `:name:` completo per inserire l'emoji, o due o più caratteri per i suggerimenti. Vedi [Codici shortcode emoji](#emoji-shortcodes). Richiede Claude Code v2.1.217 o versione successiva                                                                                                                                                                                                           |
| `?` su input vuoto | Attiva/disattiva il pannello di aiuto delle scorciatoie | Digitare `?` quando l'input contiene già testo inserisce il carattere                                                                                                                                                                                                                                                                                                                                       |

<h3 id="transcript-viewer">
  Visualizzatore della trascrizione
</h3>

Quando il visualizzatore della trascrizione è aperto (attivato con `Ctrl+O`), queste scorciatoie sono disponibili. Esegui `/tui` senza argomenti per verificare quale renderer è attivo. `Ctrl+E` può essere rimappato tramite [`transcript:toggleShowAll`](/docs/it/keybindings).

| Scorciatoia          | Descrizione                                                                                                                                                                                                                                                      |
| :------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `?`                  | Attiva/disattiva il pannello di aiuto delle scorciatoie da tastiera. Richiede il [rendering a schermo intero](/docs/it/fullscreen)                                                                                                                                    |
| `{` / `}`            | Salta al prompt dell'utente precedente o successivo, come il movimento di paragrafo di vim. Richiede il [rendering a schermo intero](/docs/it/fullscreen)                                                                                                             |
| `Ctrl+E`             | Attiva/disattiva mostra tutto il contenuto. Disponibile solo nel renderer classico, non nel [rendering a schermo intero](/docs/it/fullscreen)                                                                                                                         |
| `[`                  | Scrivi l'intera conversazione nello scrollback nativo del tuo terminale in modo che `Cmd+F`, la modalità di copia di tmux e altri strumenti nativi possano cercarla. Richiede il [rendering a schermo intero](/docs/it/fullscreen#search-and-review-the-conversation) |
| `v`                  | Scrivi la conversazione in un file temporaneo e aprilo in `$VISUAL` o `$EDITOR`. Richiede il [rendering a schermo intero](/docs/it/fullscreen)                                                                                                                        |
| `q`, `Ctrl+C`, `Esc` | Esci dalla visualizzazione della trascrizione. Tutti e tre possono essere rimappati tramite [`transcript:exit`](/docs/it/keybindings)                                                                                                                                 |

<h3 id="voice-input">
  Input vocale
</h3>

| Scorciatoia                   | Descrizione      | Note                                                                                                                                                                                                                  |
| :---------------------------- | :--------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Tieni premuto o tocca `Space` | Dettatura vocale | Richiede che la [dettatura vocale](/docs/it/voice-dictation) sia abilitata. Tieni premuto per registrare, o esegui `/voice tap` per attiva/disattiva al tocco. [Rimappabile](/docs/it/voice-dictation#rebind-the-dictation-key) |

<h2 id="commands">
  Comandi
</h2>

Digita `/` in Claude Code per visualizzare i comandi disponibili, oppure digita `/` seguito da qualsiasi lettera per filtrare. Il menu `/` elenca i comandi integrati, le [skills](/docs/it/skills) raggruppate e create dagli utenti, e i comandi forniti da [plugins](/docs/it/plugins/overview) e [server MCP](/docs/it/mcp#use-mcp-prompts-as-commands). Non tutti i comandi integrati sono visibili a ogni utente poiché alcuni dipendono dalla tua piattaforma o dal tuo piano, e [alcuni comandi disponibili sono nascosti dal menu per scelta progettuale](/docs/it/commands#how-the-command-menu-matches-what-you-type) e vengono eseguiti quando digiti il loro nome completo.

Nel [rendering a schermo intero](/docs/it/fullscreen#use-the-mouse), il comando `/` e gli elenchi di suggerimenti file `@` rispondono anche al mouse: passare il mouse evidenzia una riga e fare clic la accetta.

Consulta il [riferimento dei comandi](/docs/it/commands) per l'elenco completo dei comandi inclusi in Claude Code.

<h3 id="complete-a-command-mid-prompt">
  Completare un comando a metà del prompt
</h3>

Il completamento dei comandi funziona anche a metà di un prompt: digita `/` dopo uno spazio, quindi le prime lettere di un nome, come in `esegui i test, quindi /com`. Solo i comandi i cui nomi iniziano con quelle lettere corrispondono, quindi un percorso di file come `/tmp/notes.md` non mantiene un elenco aperto. Claude Code esegue un comando solo quando il comando [inizia il tuo messaggio](/docs/it/commands).

* **Nel [rendering a schermo intero](/docs/it/fullscreen)**: i risultati si aprono come un elenco mentre digiti, senza alcuna riga evidenziata, quindi `Invio` invia comunque il tuo prompt così come digitato. Premi `Tab` per inserire il risultato principale, oppure seleziona una riga con i tasti freccia e `Invio`.
* **Al di fuori dello schermo intero**: il resto del risultato principale appare come testo fantasma al tuo cursore, con un conteggio come `+2` quando più comandi corrispondono. Premi `Tab` per inserire l'unico risultato, oppure per aprire l'elenco quando più risultati corrispondono, quindi seleziona una riga con i tasti freccia e `Invio`.

In entrambi i renderer, premi `Tab` su un `/` nudo a metà prompt per elencare ogni comando.

Una skill di plugin corrisponde anche al suo nome nudo, quindi `/deploy` trova una skill denominata `myplugin:deploy-app`. Quando inserisci il risultato, Claude Code scrive il `/myplugin:deploy-app` completo.

<h2 id="vim-editor-mode">
  Modalità editor Vim
</h2>

Abilita la modifica in stile vim tramite `/config` → Editor mode.

Claude Code mantiene la modalità vim e la posizione del cursore quando attivi il [visualizzatore di trascrizioni](#transcript-viewer) con `Ctrl+O` o apri e chiudi un pannello come `/config`. Se lasci il prompt in modalità NORMAL, rimane in modalità NORMAL quando ritorni, con il cursore dove l'hai lasciato.

<h3 id="mode-switching">
  Cambio di modalità
</h3>

| Comando          | Azione                                                                                                                          | Dalla modalità |
| :--------------- | :------------------------------------------------------------------------------------------------------------------------------ | :------------- |
| `Esc` o `Ctrl+[` | Entra in modalità NORMAL. Nei terminali che utilizzano il protocollo di tastiera Kitty, `Ctrl+[` richiede v2.1.242 o successivo | INSERT, VISUAL |
| `i`              | Inserisci prima del cursore                                                                                                     | NORMAL         |
| `I`              | Inserisci all'inizio della riga                                                                                                 | NORMAL         |
| `a`              | Inserisci dopo il cursore                                                                                                       | NORMAL         |
| `A`              | Inserisci alla fine della riga                                                                                                  | NORMAL         |
| `o`              | Apri riga sotto                                                                                                                 | NORMAL         |
| `O`              | Apri riga sopra                                                                                                                 | NORMAL         |
| `v`              | Avvia selezione per caratteri                                                                                                   | NORMAL         |
| `V`              | Avvia selezione per righe                                                                                                       | NORMAL         |

<h3 id="remap-insert-mode-key-sequences">
  Rimappa sequenze di tasti in modalità INSERT
</h3>

L'impostazione [`vimInsertModeRemaps`](/docs/it/settings-reference#viminsertmoderemaps) mappa una sequenza di due tasti in modalità INSERT su Escape, quindi una mappatura come `jj` ti riporta in modalità NORMAL. Richiede Claude Code v2.1.208 o successivo.

Il seguente esempio di `~/.claude/settings.json` attiva la modalità vim e mappa `jj` su Escape:

```json theme={null}
{
  "editorMode": "vim",
  "vimInsertModeRemaps": { "jj": "<Esc>" }
}
```

Ogni chiave è esattamente due caratteri stampabili digitati in sequenza, e `"<Esc>"` è l'unico target supportato. Le voci con una lunghezza o un target diverso vengono ignorate.

Digitare il primo carattere di una sequenza lo inserisce normalmente. Premere il secondo carattere entro un secondo rimuove quel carattere in sospeso e passa alla modalità NORMAL, lasciando nessuno dei due caratteri nel tuo input. Dopo la finestra di un secondo, o se segue un tasto diverso, entrambi i caratteri rimangono come testo letterale, quindi puoi comunque digitare una parola contenente la sequenza facendo una pausa tra i due tasti.

Claude Code legge questa impostazione dal tuo file di impostazioni utente, dal flag `--settings` e dalle [impostazioni gestite](/docs/it/managed-settings) solo. Le voci nel `.claude/settings.json` o `.claude/settings.local.json` di un progetto vengono ignorate, quindi un repository estratto non può rimappare le tue scorciatoie da tastiera.

<h3 id="navigation-normal-mode">
  Navigazione (modalità NORMAL)
</h3>

| Comando         | Azione                                                                                                                                                        |
| :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `h`/`j`/`k`/`l` | Sposta sinistra/giù/su/destra                                                                                                                                 |
| `Space`         | Sposta destra                                                                                                                                                 |
| `w`             | Parola successiva                                                                                                                                             |
| `e`             | Fine della parola                                                                                                                                             |
| `b`             | Parola precedente                                                                                                                                             |
| `0`             | Inizio della riga                                                                                                                                             |
| `$`             | Fine della riga                                                                                                                                               |
| `^`             | Primo carattere non vuoto                                                                                                                                     |
| `gg`            | Inizio dell'input                                                                                                                                             |
| `G`             | Fine dell'input                                                                                                                                               |
| `f{char}`       | Salta alla prossima occorrenza del carattere                                                                                                                  |
| `F{char}`       | Salta alla precedente occorrenza del carattere                                                                                                                |
| `t{char}`       | Salta appena prima della prossima occorrenza del carattere                                                                                                    |
| `T{char}`       | Salta appena dopo la precedente occorrenza del carattere                                                                                                      |
| `;`             | Ripeti l'ultimo movimento f/F/t/T                                                                                                                             |
| `,`             | Ripeti l'ultimo movimento f/F/t/T in ordine inverso                                                                                                           |
| `/`             | Apri ricerca cronologia inversa, come `Ctrl+R`. Il prompt di ricerca vuoto mostra un suggerimento: premi `Esc` poi `i` poi `/` per aprire il menu dei comandi |

<Note>
  In modalità NORMAL vim, se il cursore è all'inizio o alla fine dell'input e non può muoversi ulteriormente, `j`/`k` e `↑`/`↓` navigano la cronologia dei comandi. `←` su un prompt vuoto apre la [visualizzazione agente](/docs/it/agent-view) sia dalla modalità NORMAL che INSERT; prima di v2.1.219, `←` su un prompt vuoto non faceva nulla in modalità NORMAL.
</Note>

<h3 id="editing-normal-mode">
  Modifica (modalità NORMAL)
</h3>

| Comando               | Azione                                                                                                                              |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------- |
| `x`                   | Elimina carattere                                                                                                                   |
| `dd`                  | Elimina riga                                                                                                                        |
| `D`                   | Elimina fino alla fine della riga                                                                                                   |
| `dw`/`de`/`db`        | Elimina parola/fino alla fine/indietro                                                                                              |
| `df{char}`/`dt{char}` | Elimina fino a e incluso, o fino a, la prossima occorrenza di un carattere                                                          |
| `cc`                  | Cambia riga                                                                                                                         |
| `C`                   | Cambia fino alla fine della riga                                                                                                    |
| `cw`/`ce`/`cb`        | Cambia parola/fino alla fine/indietro                                                                                               |
| `s`                   | Sostituisci carattere: elimina il carattere sotto il cursore e entra in modalità INSERT. Richiede Claude Code v2.1.211 o successivo |
| `S`                   | Sostituisci riga: cancella la riga e entra in modalità INSERT. Richiede Claude Code v2.1.211 o successivo                           |
| `yy`/`Y`              | Copia riga                                                                                                                          |
| `yw`/`ye`/`yb`        | Copia parola/fino alla fine/indietro                                                                                                |
| `p`                   | Incolla dopo il cursore                                                                                                             |
| `P`                   | Incolla prima del cursore                                                                                                           |
| `>>`                  | Indenta riga                                                                                                                        |
| `<<`                  | Dedenta riga                                                                                                                        |
| `J`                   | Unisci righe                                                                                                                        |
| `u`                   | Annulla                                                                                                                             |
| `.`                   | Ripeti l'ultima modifica                                                                                                            |

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

Premi `v` per la selezione per caratteri o `V` per la selezione per righe. I movimenti estendono la selezione e gli operatori agiscono su di essa direttamente.

| Comando          | Azione                                                 |
| :--------------- | :----------------------------------------------------- |
| `d`/`x`          | Elimina selezione                                      |
| `y`              | Copia selezione                                        |
| `c`/`s`          | Cambia selezione                                       |
| `p`              | Sostituisci selezione con il contenuto del registro    |
| `r{char}`        | Sostituisci ogni carattere selezionato con `{char}`    |
| `~`/`u`/`U`      | Attiva/disattiva, minuscole o maiuscole selezione      |
| `>`/`<`          | Indenta o dedenta righe selezionate                    |
| `J`              | Unisci righe selezionate                               |
| `o`              | Scambia cursore e ancoraggio                           |
| `iw`/`aw`/`i"`/… | Seleziona un oggetto di testo                          |
| `v`/`V`          | Attiva/disattiva tra per caratteri e per righe, o esci |

La modalità visuale a blocchi con `Ctrl+V` non è supportata.

<h2 id="command-history">
  Cronologia dei comandi
</h2>

Claude Code mantiene una cronologia dei prompt che digitate, e il richiamo con freccia Su raggiunge i prompt delle sessioni precedenti dello stesso progetto:

* La cronologia degli input è archiviata per directory di lavoro
* Eseguendo `/clear` si avvia una nuova sessione: il richiamo elenca quindi i prompt della nuova sessione per primi, seguiti dai prompt delle sessioni precedenti. La conversazione della sessione precedente viene preservata e può essere ripresa.
* L'invio dello stesso prompt due volte di seguito registra una voce di cronologia, quindi premendo Su si passa al prompt distinto precedente
* Quando richiamate un prompt che includeva testo incollato, Claude Code invia di nuovo il contenuto incollato completo quando lo reinviate. Se il contenuto è stato nel frattempo [pulito](/docs/it/claude-directory#cleaned-up-automatically), Claude Code non invia la stringa letterale `[Pasted text #N]`; vedere [Incollare contenuto di grandi dimensioni](/docs/it/terminal-config#paste-large-content) per sapere cosa accade al prompt
* L'espansione della cronologia con `!` è disabilitata per impostazione predefinita

<h3 id="reverse-search-with-ctrl-r">
  Ricerca inversa con Ctrl+R
</h3>

Premete `Ctrl+R` per cercare in modo interattivo nella cronologia dei comandi. Nel [rendering a schermo intero](/docs/it/fullscreen), `Ctrl+R` apre una finestra di dialogo di ricerca: digitate per filtrare, premete `Su` e `Giù` per spostarvi tra i risultati e premete `Ctrl+S` per ciclo l'ambito attraverso questa sessione, questo progetto e tutti i progetti. Premete `Invio` o `Tab` per posizionare una corrispondenza nell'input del prompt, oppure `Esc` per annullare. I passaggi seguenti descrivono la ricerca inline del renderer classico:

1. **Avvia ricerca**: premete `Ctrl+R` per attivare la ricerca inversa della cronologia
2. **Digita query**: inserite il testo da cercare nei comandi precedenti. Il termine di ricerca è evidenziato nei risultati corrispondenti
3. **Naviga tra i risultati**: premete di nuovo `Ctrl+R` per ciclo tra i risultati più vecchi
4. **Ambito di ricerca**: la ricerca inline cerca sempre i prompt da tutti i progetti
5. **Accetta corrispondenza**:
   * Premete `Tab` o `Esc` per accettare la corrispondenza corrente e continuare la modifica
   * Premete `Invio` per accettare ed eseguire il comando immediatamente
6. **Annulla ricerca**:
   * Premete `Ctrl+C` per annullare e ripristinare l'input originale
   * Premete `Backspace` su una ricerca vuota per annullare

La ricerca inline scansiona la cronologia completa dei prompt, dal più recente al più vecchio, con i duplicati compressi all'occorrenza più recente. La finestra di dialogo a schermo intero cerca l'intera cronologia dei prompt nell'ambito selezionato, dal più recente al più vecchio, con i duplicati compressi all'occorrenza più recente: i prompt più recenti appaiono immediatamente e i risultati dai prompt più vecchi si riempiono mentre Claude Code carica il resto. I prompt corrispondenti vengono visualizzati con il termine di ricerca evidenziato, in modo da poter trovare e riutilizzare gli input precedenti.

L'accettazione di una corrispondenza o l'annullamento della ricerca hanno effetto immediato, anche mentre Claude Code sta ancora caricando la cronologia.

<h2 id="background-bash-commands">
  Comandi Bash in background
</h2>

Claude Code supporta l'esecuzione di comandi Bash in background, permettendovi di continuare a lavorare mentre i processi a lunga esecuzione vengono eseguiti.

<h3 id="how-backgrounding-works">
  Come funziona il backgrounding
</h3>

Quando Claude Code esegue un comando in background, lo esegue in modo asincrono e restituisce immediatamente un ID di attività in background. Claude Code può rispondere a nuovi prompt mentre il comando continua a essere eseguito in background.

Per eseguire comandi in background, potete:

* Chiedere a Claude Code di eseguire un comando in background
* Premere `Ctrl+B` per spostare una normale invocazione dello strumento Bash in background. Gli utenti di Tmux devono premere `Ctrl+B` due volte a causa della chiave di prefisso di tmux.

**Caratteristiche principali:**

* L'output viene scritto in un file e Claude può recuperarlo utilizzando lo strumento Read
* Le attività in background hanno ID univoci per il tracciamento e il recupero dell'output
* Le attività in background vengono pulite automaticamente quando Claude Code esce. Su macOS e Linux, quando interrompete un'attività in background da [`/tasks`](/docs/it/commands) o Claude Code la interrompe all'uscita, i processi che si sono staccati dalla shell dell'attività, come quelli avviati con `setsid` o `timeout`, si fermano anche loro
* Se mettete in background la sessione invece di uscire, le vostre attività in background continuano a essere eseguite nella sessione in background. Vedere [mettere in background una sessione in esecuzione](/docs/it/agent-view#from-inside-a-session)
* Le attività in background vengono terminate automaticamente se l'output supera 5GB, con una nota in stderr che spiega il motivo
* Su macOS e Linux, Claude Code termina le attività in background in esecuzione quando il sistema operativo segnala pressione della memoria, a condizione che la sessione sia stata inattiva per almeno 30 minuti e nessun turno o subagent sia in esecuzione. Richiede Claude Code v2.1.193 o successivo
  * Il [log di debug](/docs/it/debug-your-config) spiega perché le attività sono state interrotte, o perché un evento di pressione le ha lasciate in esecuzione
  * Impostare [`CLAUDE_CODE_DISABLE_BG_SHELL_PRESSURE_REAP`](/docs/it/env-vars) a `1` per disattivare gli arresti dovuti alla pressione della memoria
* I comandi in background di proprietà di un [subagent](/docs/it/sub-agents) non hanno limite di tempo, tranne che un comando di proprietà di un subagent in esecuzione in foreground termina quando quel subagent fornisce la sua risposta finale; vedere [Comandi in background](/docs/it/tools-reference#background-commands) nel riferimento degli strumenti. Prima della v2.1.218, né il reap della pressione della memoria né il precedente limite di 60 minuti sui comandi del subagent coprivano i comandi spostati in background con `Ctrl+B`

Per disabilitare tutta la funzionalità di attività in background, impostare la variabile di ambiente `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` a `1`. Vedere [Variabili di ambiente](/docs/it/env-vars) per i dettagli.

**Comandi comuni messi in background:**

* Strumenti di build (webpack, vite, make)
* Gestori di pacchetti (npm, yarn, pnpm)
* Test runner (jest, pytest)
* Server di sviluppo
* Processi a lunga esecuzione (docker, terraform)

<h3 id="shell-mode-with-prefix">
  Modalità shell con prefisso `!`
</h3>

Eseguite comandi shell direttamente senza passare per Claude aggiungendo il prefisso `!` al vostro input:

```bash theme={null}
! npm test
! git status
! ls -la
```

Modalità shell:

* Aggiunge il comando e il suo output al contesto della conversazione
* Mostra il progresso e l'output in tempo reale
* Supporta lo stesso backgrounding `Ctrl+B` per i comandi a lunga esecuzione
* Non richiede a Claude di interpretare o approvare il comando
* Supporta l'autocompletamento basato sulla cronologia: digitate un comando parziale e premete `Tab` per completare dai comandi `!` precedenti nel progetto corrente
* Supporta l'autocompletamento del percorso file live a partire dalla v2.1.193 su tutte le piattaforme: digitate un token contenente una barra, come `./src/` o `~/`, per vedere un elenco a discesa dei file e delle directory corrispondenti, quindi premete `Tab` per accettare. Utilizzate le barre in avanti anche su Windows; l'elenco a discesa viene attivato da `/`, non da `\`
* Uscite con `Escape`, `Backspace`, o `Ctrl+U` su un prompt vuoto
* Incollare testo che inizia con `!` in un prompt vuoto entra automaticamente in modalità shell, corrispondendo al comportamento di `!` digitato

A meno che la vostra sessione non sia una di quelle elencate in [modalità sandbox rigorosa](/docs/it/sandboxing#the-unsandboxed-retry-escape-hatch), i comandi che digitate in modalità shell vengono eseguiti al di fuori della [sandbox](/docs/it/sandboxing) anche quando avete abilitato il sandboxing, perché la sandbox si applica ai comandi che Claude esegue.

Claude risponde automaticamente all'output del comando una volta che arriva nella trascrizione, quindi potete eseguire `! npm test` e ottenere una spiegazione degli errori senza un secondo prompt. La risposta costa lo stesso di inviare un prompt normale. Per ripristinare il comportamento precedente in cui l'output viene aggiunto al contesto senza una risposta, impostare [`respondToBashCommands`](/docs/it/settings-reference#respondtobashcommands) a `false` in `settings.json`. Prima della v2.1.186, la modalità shell aggiungeva sempre l'output al contesto senza una risposta.

<h2 id="queue-messages-while-claude-works">
  Accodare messaggi mentre Claude lavora
</h2>

Digitate un messaggio e premete `Enter` mentre Claude sta lavorando. Claude Code accoda il messaggio invece di interrompere il turno, e elenca le voci in coda sopra la casella di input finché non le invia. Potete accodare i comandi `!` [shell](#shell-mode-with-prefix) e la maggior parte dei [comandi](/docs/it/commands) allo stesso modo, ad eccezione dei comandi, come `/status`, che Claude Code esegue non appena li inviate.

I messaggi inviati e accodati vengono visualizzati in grigio finché Claude non inizia a rispondere, quindi potete capire quali messaggi Claude non ha ancora iniziato.

<h3 id="when-claude-code-sends-what-you-queued">
  Quando Claude Code invia ciò che avete accodato
</h3>

Quando una voce in coda raggiunge Claude dipende da ciò che avete accodato.

* Messaggi: se accodate un messaggio mentre Claude sta eseguendo chiamate di strumenti, Claude Code lo passa a Claude non appena quelle chiamate di strumenti terminano, all'interno dello stesso turno. Quando il turno termina con messaggi ancora in coda, vengono inviati senza un altro pressione di tasto, nell'ordine in cui li avete digitati
* Comandi e comandi shell: Claude Code li mantiene finché il turno non termina, quindi li esegue uno alla volta, mantenendo l'ordine in cui li avete accodati

Per inviare ciò che avete accodato senza aspettare, premete `Ctrl+Enter`. I vostri messaggi in coda vengono inviati subito, con la vostra bozza accodata dietro di essi se ne avevate digitata una. Richiede Claude Code v2.1.275 o successivo.

Se avete accodato un comando shell `!` davanti ai vostri messaggi, il tasto interrompe il turno. Altrimenti, ciò che accade al turno dipende da ciò che Claude sta facendo quando premete il tasto:

* Esecuzione di comandi shell, subagenti o altro lavoro che può spostarsi sullo [sfondo](#background-bash-commands): quel lavoro si sposta sullo sfondo e continua a essere eseguito, e Claude legge i vostri messaggi nello stesso turno
* Solo la scrittura di una risposta, o l'esecuzione di qualcosa che non può spostarsi sullo sfondo: Claude Code interrompe il turno e invia i vostri messaggi dopo. Prima della v2.1.281, il tasto interrompeva il turno in entrambi i casi

In [modalità shell](#shell-mode-with-prefix), il tasto accoda solo il vostro comando. Nei terminali che non segnalano tasti estesi, `Ctrl+Enter` arriva come semplice `Enter` e accoda la bozza invece; `Ctrl+X Ctrl+S` funziona in qualsiasi terminale. Entrambi i tasti sono associazioni dell'[azione `chat:sendNow`](/docs/it/keybindings#chat-actions).

Premete `Esc` per interrompere il turno senza inviare la vostra bozza. Claude Code mantiene ciò che avete accodato e lo invia subito.

Claude Code esegue alcuni comandi non appena li inviate invece di accodarli, tra cui `/model`, `/effort` e `/fast`. Ognuno dei tre cambia un'impostazione: il modello, il livello di sforzo o la modalità veloce. Se Claude Code applica la nuova impostazione al turno su cui Claude sta già lavorando, o solo dal vostro turno successivo, differisce per comando:

* [`/model`](/docs/it/model-config#setting-your-model): una volta confermato l'[avviso di cache](/docs/it/prompt-caching#switching-models), se Claude Code ne mostra uno, Claude Code applica il vostro cambio alla prossima richiesta che effettua in quel turno
* [`/effort`](/docs/it/model-config#adjust-effort-level): una volta confermato l'[avviso di cache](/docs/it/prompt-caching#changing-effort-level), se Claude Code ne mostra uno, Claude Code applica il vostro cambio alla prossima richiesta che effettua in quel turno
* [`/fast`](/docs/it/fast-mode#toggle-fast-mode): Claude Code mantiene l'impostazione della modalità veloce che era attiva quando il turno è iniziato, quindi il vostro cambio di velocità si applica dal vostro turno successivo. Se il vostro modello attuale non supporta la modalità veloce, attivarla [cambia anche il vostro modello](/docs/it/prompt-caching#turning-on-fast-mode), e Claude Code utilizza il nuovo modello dalla sua prossima richiesta in quel turno

<h3 id="take-back-what-you-queued">
  Riprendete ciò che avete accodato
</h3>

Premete `Up` dalla prima riga della casella di input per riprendere i messaggi e i comandi in coda. Claude Code li rimuove dalla coda e li mette nella casella di input, uno per riga, davanti a qualsiasi testo che avevate digitato. Modificate il testo e premete `Enter` per accodarlo di nuovo come una voce, o cancellate la casella di input per scartarlo.

Claude Code riprende i comandi shell in coda solo quando la casella di input è vuota e non avete nient'altro in coda, e cambia la casella di input alla modalità shell quando lo fa. Altrimenti li lascia in coda, elencati con il loro prefisso `!`, e li esegue dopo che il turno termina.

<h2 id="prompt-suggestions">
  Suggerimenti di prompt
</h2>

Quando aprite una sessione per la prima volta, Claude Code mostra un comando di esempio attenuato nell'input del prompt per aiutarvi a iniziare. Lo seleziona dalla cronologia git del vostro progetto, quindi l'esempio riflette i file su cui avete lavorato di recente.

Dopo che Claude risponde, Claude Code può suggerire il vostro prossimo prompt in base alla cronologia della conversazione, come un passaggio successivo da una richiesta in più parti o una continuazione naturale del vostro flusso di lavoro.

* Premete `Tab` o `Freccia destra` per inserire il suggerimento nell'input del prompt, quindi `Invio` per inviare
* Iniziate a digitare per chiudere il suggerimento

Claude Code genera ciascuno di questi suggerimenti di prompt successivo con una richiesta in background al medesimo modello che la vostra sessione sta utilizzando. La richiesta conta verso i limiti di utilizzo del vostro piano o i vostri costi API. Poiché riutilizza la cache del prompt della conversazione, è principalmente costituita da letture della cache più alcuni token di output, quindi il costo aggiuntivo è minimo.

<h3 id="when-claude-code-skips-suggestions">
  Quando Claude Code salta i suggerimenti
</h3>

In modalità interattiva, Claude Code lascia i suggerimenti di prompt disattivati per impostazione predefinita e nasconde l'interruttore **Prompt suggestions** in `/config` in una [sessione che non recupera i flag delle funzionalità](/docs/it/env-vars#features-that-need-feature-flag-fetching), come una su un provider di terze parti o attraverso un gateway di app Claude, e in una [prima sessione dopo un'installazione o un aggiornamento](/docs/it/env-vars#first-session-after-an-install-or-upgrade) i cui flag non sono ancora arrivati.

Claude Code salta anche i singoli suggerimenti in diverse situazioni, tra cui:

* La cache del prompt è fredda, per evitare costi inutili
* Dopo il primo turno di una conversazione, in alcune sessioni
* La risposta precedente è terminata con un errore
* Mentre siete in plan mode
* Il vostro account è vicino o al limite di utilizzo. Per mantenere i suggerimenti attivi fino a quando non raggiungete il limite, impostate [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](/docs/it/env-vars) su `true`. Prima della v2.1.238, Claude Code li saltava vicino al limite anche con la variabile impostata su `true`
* In un [agent team](/docs/it/agent-teams), nelle sessioni dei compagni di squadra per impostazione predefinita. La sessione del lead mostra i suggerimenti

In modalità stampa, Claude Code non genera suggerimenti per impostazione predefinita. Passate [`--prompt-suggestions`](/docs/it/cli-reference#cli-flags) con `-p "<prompt>" --output-format stream-json --verbose` per fare in modo che Claude Code emetta un messaggio `prompt_suggestion` dopo ogni turno che ne genera uno. Il generatore salta anche le conversazioni molto brevi e le cache di prompt fredde qui, quindi una singola query `-p` breve potrebbe non emetterne nessuna.

<h3 id="turn-prompt-suggestions-off">
  Disattivare i suggerimenti di prompt
</h3>

Per disattivare completamente i suggerimenti di prompt, utilizzate uno dei seguenti metodi:

* Disattivate **Prompt suggestions** in `/config`
* Impostate [`promptSuggestionEnabled`](/docs/it/settings-reference#promptsuggestionenabled) su `false` nel vostro file di impostazioni
* Impostate la variabile di ambiente [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](/docs/it/env-vars) su `false`, che ha la precedenza sull'impostazione:
  ```bash theme={null}
  export CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=false
  ```

Per disattivare i suggerimenti di prompt in tutta un'organizzazione, impostate `promptSuggestionEnabled` su `false` nelle [impostazioni gestite](/docs/it/managed-settings). Impostate anche `CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION` su `false` sotto la chiave [`env`](/docs/it/settings-reference#env) gestita in modo che gli utenti non possano riattivarli con la propria variabile di ambiente.

<h2 id="emoji-shortcodes">
  Scorciatoie emoji
</h2>

Digita un `:` seguito da una scorciatoia emoji nell'input del prompt per inserire l'emoji. Richiede Claude Code v2.1.217 o versione successiva.

* Digita una scorciatoia completa come `:heart:` e Claude Code la sostituisce con ❤️ non appena digiti i due punti di chiusura `:`
* Digita `:` più almeno due caratteri di un nome, come `:hea`, per aprire un popup di suggerimento, quindi premi `Tab` o `Enter` per inserire l'emoji evidenziato

La scorciatoia deve iniziare l'input o seguire uno spazio, quindi un `:` all'interno di una parola o URL non apre i suggerimenti.

Per disattivare la funzione, imposta [`emojiCompletionEnabled`](/docs/it/settings-reference#emojicompletionenabled) su `false` in `settings.json`. Questo disabilita sia il popup di suggerimento che la sostituzione inline.

<h2 id="check-spelling-as-you-type">
  Controlla l'ortografia mentre digiti
</h2>

Claude Code può sottolineare le parole scritte male nell'input del prompt mentre digiti. Controlla solo il testo nella casella di input, mai le risposte di Claude o i tuoi file. Non controlla nulla nemmeno quando la casella di input è in [modalità shell](#shell-mode-with-prefix), ricerca della cronologia `Ctrl+R`, o [dettatura vocale](/docs/it/voice-dictation).

Il controllo ortografico è disattivato per impostazione predefinita e Claude Code non controlla nulla in [modalità lettore di schermo](/docs/it/accessibility). Richiede Claude Code v2.1.235 o successivo.

<h3 id="prerequisites">
  Prerequisiti
</h3>

* Installa [aspell](https://github.com/GNUAspell/aspell), [hunspell](https://github.com/hunspell/hunspell), o [ispell](https://en.wikipedia.org/wiki/Ispell) e assicurati che sia nel tuo `PATH`. Claude Code esegue il primo dei tre che trova, in quell'ordine, su ogni piattaforma, incluso uno shim `.cmd` che un gestore di pacchetti installa su Windows.
* Per verificare che il programma sia nel tuo `PATH`, esegui `aspell --version`, `hunspell --version`, o `ispell -v` nel tuo terminale. Un errore "command not found" significa che non è ancora nel tuo `PATH`.

<h3 id="turn-spell-checking-on-or-off">
  Attiva o disattiva il controllo ortografico
</h3>

Claude Code legge l'impostazione [`spellcheck`](/docs/it/settings-reference#spellcheck) da tre posizioni e la ignora nel file `.claude/settings.json` e `.claude/settings.local.json` di un progetto. Attivala da quella che utilizzi:

<Tabs>
  <Tab title="Impostazioni utente">
    Aggiungi `spellcheck` a `~/.claude/settings.json`. Si applica in ogni progetto che apri, come il resto delle tue [impostazioni utente](/docs/it/settings#where-settings-live):

    ```json theme={null}
    {
      "spellcheck": { "enabled": true }
    }
    ```
  </Tab>

  <Tab title="Riga di comando">
    Salva `spellcheck` in un file JSON, come `spellcheck.json`:

    ```json theme={null}
    {
      "spellcheck": { "enabled": true }
    }
    ```

    Quindi passa il file a `--settings`. Si applica solo a quella sessione:

    ```bash theme={null}
    claude --settings spellcheck.json
    ```
  </Tab>

  <Tab title="Impostazioni gestite">
    Aggiungi `spellcheck` a una delle [fonti di impostazioni gestite](/docs/it/permissions#managed-settings) della tua organizzazione. Si applica a ogni utente che riceve quelle impostazioni e non può disattivarle:

    ```json theme={null}
    {
      "spellcheck": { "enabled": true }
    }
    ```
  </Tab>
</Tabs>

Per verificare che il controllo ortografico sia attivo, digita una parola scritta male e uno spazio. Claude Code sottolinea la parola. Se non lo fa, vedi [Quando Claude Code non sottolinea nulla](#when-claude-code-underlines-nothing). Per disattivare di nuovo il controllo ortografico, imposta `enabled` a `false` nello stesso posto, o rimuovi `spellcheck`.

Per scegliere quale dei tre programmi Claude Code esegue, quale dizionario utilizza, o il colore della sottolineatura, aggiungi uno di questi campi accanto a `enabled`, nello stesso posto:

* `checker`: `aspell`, `hunspell`, o `ispell`. Claude Code non ritorna a un checker che nomini e tratta qualsiasi altro valore come `auto`.
* `language`: un nome di dizionario nella forma del tuo checker, come `en_GB`. Claude Code ignora qualsiasi valore che non sia un nome di dizionario semplice, come un percorso o un nome con spazi, e il checker utilizza il suo dizionario predefinito.
* `color`: un nome di colore come `yellow`, o un valore `#rrggbb`, `#rgb`, `rgb(r,g,b)`, `ansi256(n)`, o `ansi:<name>`. Claude Code utilizza il colore di errore del tuo tema per impostazione predefinita e per qualsiasi valore che non riconosce.

Ad esempio, questa impostazione `spellcheck` esegue hunspell con il suo dizionario `en_GB` e sottolinea le parole in giallo. Funziona allo stesso modo in `~/.claude/settings.json`, nel file che passi a `--settings`, e nelle impostazioni gestite:

```json theme={null}
{
  "spellcheck": {
    "enabled": true,
    "checker": "hunspell",
    "language": "en_GB",
    "color": "yellow"
  }
}
```

Se più di uno dei tre posti ha un'impostazione `spellcheck`, Claude Code utilizza solo uno di essi: prima le impostazioni gestite, poi `--settings`, poi le impostazioni utente. Non combina campi da due posti. Ad esempio, quando `--settings` imposta `spellcheck`, un `language` nelle tue impostazioni utente non ha effetto.

<h3 id="what-claude-code-underlines">
  Cosa sottolinea Claude Code
</h3>

Poco dopo che smetti di digitare, Claude Code sottolinea le parole che il dizionario non conosce. Lascia sola la parola che stai ancora digitando finché non la superi, e non cambia mai il tuo testo. Salta anche il testo che sembra codice:

* Comandi come `/help`, menzioni `@`, URL, percorsi di file e flag come `--verbose`
* Parole con cifre, trattini bassi, o una lettera maiuscola dopo la prima, e testo tra backtick

Claude Code salta anche il testo cinese, giapponese, coreano, tailandese, laotiano, khmer e birmano.

Claude Code non ha una propria lista di parole: una parola è scritta male quando il tuo checker dice così. Per impedire a Claude Code di sottolineare una parola, aggiungi la parola al dizionario personale del tuo checker, seguendo la documentazione del checker. Claude Code rileva la nuova parola dopo averlo riavviato.

<h3 id="when-claude-code-underlines-nothing">
  Quando Claude Code non sottolinea nulla
</h3>

Claude Code non sottolinea nulla quando non riesce a mantenere un checker in esecuzione:

* Nessun checker è installato, o quello che hai nominato in `checker` manca
* Il checker fallisce due volte di seguito, all'avvio o più tardi nella sessione. Claude Code lo riavvia dopo il primo fallimento e smette di controllare dopo il secondo, finché non riavvii Claude Code
* Il checker impiega più di 15 secondi per rispondere, tre volte. Ogni volta, Claude Code lascia le parole su cui stava aspettando non marcate; dopo la terza, smette di controllare finché non riavvii Claude Code

Per scoprire quale di questi è accaduto, avvia `claude --debug` con il controllo ortografico attivo e digita una parola. Quindi cerca le righe `[spellcheck]` nel log di debug in `~/.claude/debug/<session-id>.txt`. Una riga nomina il programma che Claude Code ha avviato, o elenca quelli che ha cercato e non ha trovato. Le righe successive dicono perché ha smesso. Un errore di dizionario mancante lì significa che il checker non ha un dizionario per il tuo valore `language`, o nessuno predefinito quando `language` non è impostato. Installane uno, o imposta `language` a un dizionario che hai.

<h2 id="invisible-characters-in-prompts">
  Caratteri invisibili nei prompt
</h2>

Il testo incollato può contenere caratteri Unicode che un terminale disegna come nulla, come caratteri di tag, controlli bidirezionali e spazi a larghezza zero, quindi un prompt può contenere testo che non vedete mai. Per evitare che il testo copiato contenga istruzioni che il terminale non disegna, Claude Code rimuove questi caratteri quando premete Invio, prima di inviare qualsiasi cosa. Pulisce sia il prompt che il contenuto di qualsiasi [riferimento di testo incollato](/docs/it/terminal-config#paste-large-content) che il prompt include. Claude Code mantiene i caratteri di unione che gli script persiani e indici scrivono e i selettori all'interno delle sequenze emoji.

Se Claude Code ha rimosso qualcosa, quel tasto Invio non invia nulla. Il prompt pulito ritorna nella casella di input con un avviso come `Removed 3 invisible characters · review and press Enter to send`, e premendo Invio di nuovo si invia il testo come mostrato.

Quando passate un prompt sulla riga di comando, come in `claude "fix the login bug"`, o ne incanalate uno in una sessione interattiva, Claude Code non aspetta un secondo Invio. Rimuove i caratteri, mostra un avviso e invia il prompt pulito. Se il prompt pulito inizierebbe con `/`, Claude Code lo mette nella casella di input affinché lo rivediate e lo inviate.

<h2 id="review-changes-with-/diff">
  Esamina le modifiche con /diff
</h2>

Esegui `/diff` per rivedere le modifiche nel tuo albero di lavoro senza lasciare Claude Code. Vedrai le modifiche che Claude ha apportato finora insieme a qualsiasi altra cosa che non hai ancora committato.

Nelle modifiche che `/diff` legge da git, un submodule appare come una singola voce, e solo quando cambia il commit a cui punta; le modifiche ai file all'interno del submodule non appaiono lì.

Nel [rendering a schermo intero](/docs/it/fullscreen), `/diff` apre il [pannello diff](#diff-panel) accanto alla conversazione, che rimane aperto e si aggiorna mentre continui a lavorare. Nel renderer classico, `/diff` apre il [visualizzatore diff](#diff-viewer) al posto del prompt, e lo chiudi quando hai finito di leggere.

<h3 id="diff-panel">
  Diff panel
</h3>

Il pannello diff elenca i file modificati con i conteggi delle righe aggiunte e rimosse, e mostra il diff di ogni file sotto l'elenco. Claude Code lo aggiorna ogni volta che Claude modifica un file o esegue un comando shell. Per chiuderlo, esegui `/diff` di nuovo o fai clic sulla `✕` nell'intestazione.

Per utilizzare il pannello hai bisogno di:

* [Rendering a schermo intero](/docs/it/fullscreen)
* Un repository git
* Un terminale largo almeno 110 colonne
* Claude Code v2.1.260 o successivo

Quando il pannello non può aprirsi, `/diff` apre il visualizzatore diff invece o ti dice il motivo.

Il pannello si apre anche automaticamente una volta che Claude inizia a modificare i file, se il tuo terminale è largo almeno 144 colonne. Dopo averlo aperto tu stesso con `/diff`, le sessioni successive lo aprono non appena Claude modifica un file in qualsiasi terminale abbastanza largo per contenerlo. Chiudi il pannello e rimane chiuso, in questa sessione e nelle successive, finché non esegui `/diff` di nuovo.

Mentre il pannello è aperto, puoi:

* **Saltare a un file**: fai clic sulla sua riga nell'elenco. Scorri il pannello con la rotella del mouse. Quando l'elenco dei file stesso è troppo lungo per stare, scorri con `Alt+Up` e `Alt+Down`, o `Ctrl+Up` e `Ctrl+Down`.
* **Chiedere a Claude informazioni su righe specifiche**: selezionale nel pannello con il mouse. Claude Code allega la selezione al tuo prossimo prompt e mostra un conteggio delle righe nell'input finché non lo invii.
  * Per inviare il prompt senza la selezione, sposta il cursore subito dopo l'indicatore di conteggio delle righe e premi `Backspace` per eliminarlo. Richiede Claude Code v2.1.271 o successivo.
* **Mostrare i file che il pannello omette**: l'elenco salta i file di test e i file generati, e comprime le modifiche da prima di questa sessione in una riga in fondo. Fai clic su una riga di conteggio per espanderla.
* **Cambiare ciò che il pannello confronta**: premi `Ctrl+X B` per passare dalle modifiche di questa sessione, alle tue modifiche non committate come un elenco, a tutto ciò che è accaduto da quando il tuo ramo si è diviso dal ramo predefinito. Claude Code ricorda la scelta per ogni progetto.

Per associare i tasti a queste azioni, vedi [Azioni del pannello diff](/docs/it/keybindings#diff-panel-actions).

<h3 id="diff-viewer">
  Diff viewer
</h3>

Il visualizzatore diff prende il posto del prompt finché non lo chiudi. La sua vista **Current** mostra le tue modifiche non committate da git, o, quando non ce ne sono, ciò che il tuo ramo aggiunge in cima al ramo predefinito. Il visualizzatore ha anche una vista di turno per ogni prompt dopo il quale Claude ha modificato i file, mostrando solo quelle modifiche. Claude Code costruisce le viste di turno dalle modifiche dei file di Claude piuttosto che da git, quindi una modifica che Claude apporta tramite un comando shell appare solo sotto Current.

Usa questi tasti nel visualizzatore:

* **Sinistra e Destra**: spostati tra Current e le viste di turno.
* **Su e Giù**: seleziona un file.
* **Invio**: apri il diff del file selezionato. Scorri con Su e Giù, o PaginaSu e PaginaGiù.
* **Esc**: ritorna dal diff di un file all'elenco, o chiudi il visualizzatore dall'elenco.

Per riassociare questi tasti, vedi [Azioni diff](/docs/it/keybindings#diff-actions).

<h2 id="side-questions-with-/btw">
  Domande laterali con /btw
</h2>

Usa `/btw` per fare una domanda sul tuo lavoro attuale senza aggiungerla alla cronologia della conversazione.

```
/btw what was the name of that config file again?
```

Claude risponde a una domanda laterale da ciò che è già nella conversazione: i tuoi messaggi, le sue risposte e i risultati degli strumenti che ha raccolto. Potete chiedere informazioni sul codice che Claude ha già letto, sulle decisioni che ha preso in precedenza, o su qualsiasi altra cosa della sessione. Una domanda laterale successiva vede anche le vostre domande laterali precedenti: Claude Code riproduce i 20 scambi più recenti con ogni richiesta, finché non li cancellate. La domanda e la risposta non entrano mai nella cronologia della conversazione. Nel terminale, appaiono in un overlay dismissibile. Il terminale mantiene il thread in memoria: premete `x` per cancellare gli scambi precedenti, e scompariranno quando uscite da Claude Code.

Nel [pannello di chat dell'estensione VS Code](/docs/it/vs-code#use-the-prompt-box), `/btw` apre un pannello piuttosto che l'overlay descritto in questa sezione, e potete fare domande di follow-up direttamente nel pannello. Il thread del pannello sopravvive ai ricaricamenti della finestra, secondo la pianificazione di conservazione descritta in quella pagina. Avete bisogno dell'estensione alla versione v2.1.227 o successiva. Le versioni precedenti dell'estensione non offrono `/btw`.

* **Disponibile mentre Claude sta lavorando**: potete eseguire `/btw` anche mentre Claude sta elaborando una risposta. La domanda laterale viene eseguita in modo indipendente e non interrompe il turno principale. Vede tutto ciò che è nella conversazione finora, tranne la risposta che Claude sta ancora scrivendo.
* **Nessun accesso agli strumenti**: le domande laterali rispondono solo da ciò che è già nel contesto. Claude non può leggere file, eseguire comandi o cercare quando risponde a una domanda laterale. Se Claude scrive comunque le chiamate agli strumenti come testo, la risposta termina con una nota che nulla è stato eseguito.
* **Risposta singola**: non ci sono turni di follow-up nell'overlay. Per continuare il thread, fate un'altra domanda `/btw`. Per continuare con accesso completo agli strumenti in una sessione locale, premete `f` per eseguire il fork di questa domanda e risposta in un [subagent in background](/docs/it/sub-agents#fork-the-current-conversation).
* **Costo basso**: mentre la [prompt cache](/docs/it/prompt-caching) della conversazione è attiva, una domanda laterale costa poco oltre la risposta stessa.

Le vostre cinque domande laterali precedenti più recenti appaiono come un elenco attenuato sopra la risposta attuale, con un conteggio di quelle più vecchie. Rimangono fuori dalla cronologia della conversazione.

Per tornare all'overlay dopo averlo dismissito, eseguite `/btw` senza una domanda. L'overlay si riapre sul vostro scambio più recente. Prima della v2.1.212, `/btw` senza una domanda stampava un messaggio di utilizzo.

Una volta che la risposta appare, l'overlay accetta questi tasti.

| Tasto                        | Azione                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `Space`, `Enter`, `Escape`   | Dismissi la risposta e torna al prompt                                                                                                                                                                                                                                                                                                                                                                                    |
| `Up` / `Down`                | Scorri la risposta                                                                                                                                                                                                                                                                                                                                                                                                        |
| `Shift+Left` / `Shift+Right` | Passa tra questa risposta e le vostre risposte `/btw` precedenti. `Shift+Left` si sposta verso risposte più vecchie e `Shift+Right` ritorna verso quella attuale. `[` e `]` fanno lo stesso, per i terminali che non segnalano `Shift` con i tasti freccia. `Tab` / `Shift+Tab` scorrono le stesse risposte. Richiede Claude Code v2.1.257 o successiva. Tra v2.1.187 e v2.1.256, i tasti erano semplici `Left` / `Right` |
| `c`                          | Copia la risposta negli appunti come Markdown grezzo. Usate questo invece della selezione del mouse, che cattura il rendering del terminale con ritorno a capo rigido piuttosto che il testo sorgente                                                                                                                                                                                                                     |
| `f`                          | Avvia un [subagent con fork](/docs/it/sub-agents#fork-the-current-conversation) che eredita la conversazione padre più questa domanda e risposta, in modo che possa continuare con accesso completo agli strumenti. Rimanete nella sessione attuale e trovate il fork nel [pannello sotto il vostro prompt](/docs/it/sub-agents#observe-and-steer-running-forks). Disponibile solo in sessioni locali                               |
| `x`                          | Cancella l'elenco degli scambi `/btw` precedenti mostrati sopra la risposta attuale                                                                                                                                                                                                                                                                                                                                       |

In una [sessione in background](/docs/it/agent-view#attach-to-a-session) collegata, `Left` si scollega e vi restituisce alla vista agente, anche mentre la risposta sta ancora arrivando. La domanda laterale continua a essere eseguita mentre siete assenti. La prossima volta che vi collegate alla sessione, l'overlay si riapre con la domanda laterale, o con la sua risposta. Prima della v2.1.257, `Left` non si scollegava lì.

`/btw` vede la vostra conversazione completa ma non ha strumenti. Un [subagent](/docs/it/sub-agents) ha strumenti e inizia dal prompt che riceve, o, per un [fork](/docs/it/sub-agents#fork-the-current-conversation), da una copia di questa conversazione. Usate `/btw` per chiedere informazioni su ciò che Claude già conosce da questa sessione; usate un subagent per andare a scoprire qualcosa di nuovo.

<h2 id="task-list">
  Elenco attività
</h2>

L'elenco attività è la lista di controllo di Claude: elementi che Claude ha creato per pianificare il lavoro multi-step, con indicatori che mostrano cosa è in sospeso, in corso o completato. È separato dalla visualizzazione delle attività in background. Per visualizzare shell in esecuzione e subagent, utilizzare [`/tasks`](/docs/it/commands) invece.

L'elenco si riempie solo nelle sessioni che dispongono degli strumenti di tracciamento delle attività, che Claude Code fornisce per impostazione predefinita su [modelli Claude 3.x, Opus 4 fino a 4.7, Sonnet 4 fino a 4.6 e Haiku 4.5](/docs/it/tools-reference#task-tool-availability). Su qualsiasi altro modello, incluso un ID modello che Claude Code non riconosce, l'elenco rimane vuoto a meno che non optiate per `CLAUDE_CODE_ENABLE_TODO_TOOLS=1` o uno degli altri modi in [Disponibilità dello strumento Task](/docs/it/tools-reference#task-tool-availability). Quando la sessione dispone degli strumenti, l'elenco attività funziona come segue:

* Premere `Ctrl+T` per attivare/disattivare la visualizzazione dell'elenco attività. La visualizzazione mostra fino a cinque attività alla volta. Quando Claude non ha ancora creato elementi della lista di controllo, l'attivazione non ha effetto visibile perché non c'è nulla da visualizzare
* Se lasciate l'elenco espanso, Claude Code ripristina la visualizzazione espansa la prossima volta che avviate una sessione che contiene ancora attività, ad esempio con `--resume` o `--continue`. Quando l'elenco attività è vuoto, Claude Code lo avvia compresso
* Per visualizzare tutte le attività o cancellarle, chiedete direttamente a Claude: "mostrami tutte le attività" o "cancella tutte le attività"
* Le attività persistono attraverso le compattazioni di contesto, aiutando Claude a rimanere organizzato su progetti più grandi
* Per condividere un elenco attività tra sessioni, impostare `CLAUDE_CODE_TASK_LIST_ID` per utilizzare una directory denominata in `~/.claude/tasks/`: `CLAUDE_CODE_TASK_LIST_ID=my-project claude`

<h2 id="session-recap">
  Riepilogo della sessione
</h2>

Quando tornate al terminale dopo esservi allontanati, Claude Code mostra un riepilogo di una riga di ciò che è accaduto nella sessione finora. Il riepilogo viene generato in background una volta che sono trascorsi almeno tre minuti dall'ultimo turno completato e il terminale non è a fuoco, quindi è pronto quando tornate indietro. I riepiloghi appaiono solo una volta che la sessione ha almeno tre turni e mai due volte di seguito.

Eseguite `/recap` per generare un riepilogo su richiesta. Claude Code limita sia i riepiloghi automatici che l'output di `/recap` a 400 caratteri. Per disattivare i riepiloghi automatici, aprite `/config` e disattivate **Session recap**.

Il riepilogo della sessione è attivato per impostazione predefinita per ogni piano e provider. Il riepilogo viene sempre saltato in modalità non interattiva.

<h2 id="wait-for-a-usage-limit-to-reset">
  Attendere il ripristino di un limite di utilizzo
</h2>

Quando un [limite di utilizzo](/docs/it/errors#youve-hit-your-session-limit) di claude.ai interrompe Claude a metà attività, Claude Code rimane nella sessione aperta e continua l'attività automaticamente dopo il ripristino del limite. La continuazione automatica è attivata per impostazione predefinita nelle sessioni interattive con accesso tramite un abbonamento claude.ai. Richiede Claude Code v2.1.234 o versione successiva.

Mentre Claude Code attende, una riga in fondo alla sessione mostra quando continuerà:

```text theme={null}
Usage limit reached · continuing automatically at 3:45pm · esc to cancel
```

Mantenere la sessione aperta. Ciò che accade dopo dipende da come termina l'attesa:

* **Al ripristino**: la riga legge `continuing shortly`, quindi `Usage limit reset · continuing automatically`, e Claude Code invia a Claude un prompt fisso per riprendere l'attività da dove si era fermata. Non rinvia il vostro ultimo messaggio.
* **Dopo che il computer è andato in sospensione**: se è andato in sospensione per più di circa 30 minuti e il limite si è ripristinato mentre era in sospensione, la riga legge `Your usage limit has reset · press enter to continue`. Premere `Invio` per continuare. Dopo una sospensione più breve, Claude Code continua automaticamente.
* **Anticipatamente**: quando completate l'aggiunta di [crediti di utilizzo](/docs/it/costs#add-usage-credits-to-your-subscription) con `/usage-credits`, effettuate di nuovo l'accesso dopo `/upgrade`, o cambiate modello con `/model` durante l'attesa, Claude Code verifica se l'utilizzo è di nuovo disponibile e continua immediatamente se lo è. Non verifica dopo un aggiornamento o un acquisto che effettuate in un browser da soli. In [`opusplan`](/docs/it/model-config#opusplan-model-setting) e altre impostazioni di modello che eseguono la modalità plan su un modello diverso, Claude Code attende il ripristino.

L'attività continuata viene eseguita come qualsiasi altro turno. Claude Code continua a chiedere [autorizzazioni](/docs/it/permissions) come al solito, quindi l'attività può fermarsi su un prompt mentre siete assenti. Se raggiunge di nuovo il limite, Claude Code riattiva l'attesa automaticamente al massimo due volte di seguito, quindi si ferma e mostra `Automatic continue stopped after repeated usage-limit hits · /rate-limit-options to try again`.

<h3 id="cancel-the-wait">
  Annullare l'attesa
</h3>

Premere `Esc` a un prompt vuoto, o `Ctrl+C`, mentre la riga è visibile, oppure eseguire [`/rate-limit-options`](/docs/it/commands#all-commands) e selezionare **Don't continue automatically**. Claude Code conferma con una riga che inizia con `Automatic continue cancelled`.

Dopo un annullamento, nulla continua finché non inviate un prompt o non selezionate la riga che inizia con **Wait here, then continue automatically** da `/rate-limit-options` di nuovo. Claude Code non avvia un'attesa da solo per quella finestra di ripristino; la finestra di ripristino successiva ricomincia da capo.

L'attesa termina anche senza continuare l'attività in questi casi:

* **Inviate un prompt**: Claude Code esegue il vostro prompt invece di attendere.
* **Uscite da Claude Code**: l'attesa non si riavvia quando riprendete la sessione.
* **La conversazione cambia proprietario**: cambiate account con `/login`, cancellate o riavvolgete la conversazione, `/resume` un'altra sessione, ne tirate una con `/teleport`, riavviate con `/tui`, o consegnate la sessione a Claude Desktop, una sessione in background, o il cloud.
* **L'impostazione si disattiva, o il ripristino supera le 24 ore**: questo termina solo un'attesa che Claude Code ha avviato da solo. Un'attesa che avete selezionato da `/rate-limit-options` continua il conto alla rovescia.
* **La continuazione è bloccata**: un hook [`UserPromptSubmit`](/docs/it/hooks#userpromptsubmit) che blocca il prompt di continuazione, o un errore prima che raggiunga il modello, termina l'attesa. Claude Code vi comunica che la continuazione non è stata eseguita. Inviate un prompt per continuare.

<h3 id="start-a-wait-yourself">
  Avviare un'attesa da soli
</h3>

Claude Code non avvia l'attesa da solo in questi casi:

* **Sessioni di Remote Control e team di agenti**: una persona a quel terminale può comunque avviarne una.
* **Un ripristino a più di 24 ore di distanza**: un limite settimanale può ripristinarsi giorni dopo.
* **Un limite Opus o Sonnet mentre eseguite un modello al di fuori di quella famiglia**: il vostro turno successivo potrebbe non raggiungere quel limite. [`opusplan`](/docs/it/model-config#opusplan-model-setting) e altre impostazioni di modello che eseguono la modalità plan sulla famiglia limitata non ottengono questa eccezione.

In questi casi, e ogni volta che la continuazione automatica è disattivata, Claude Code apre il menu delle opzioni di limite di utilizzo una volta per finestra di ripristino quando raggiungete un limite al vostro terminale. Selezionate la riga che inizia con **Wait here, then continue automatically** per avviare l'attesa. In una sessione [Remote Control](/docs/it/remote-control) o [team di agenti](/docs/it/agent-teams), eseguite `/rate-limit-options` da soli per aprire il menu.

Claude Code non offre affatto l'attesa in questi casi:

* **Sessioni in background e esecuzioni `-p`**: la riga del menu non è disponibile.
* **Chiavi API, provider cloud e fatturazione basata sull'utilizzo**: l'utilizzo lì viene misurato per richiesta, quindi non c'è alcun ripristino per cui attendere.
* **Un [gateway LLM](/docs/it/llm-gateway#subscriptions-and-gateways) senza un accesso claude.ai salvato**: Claude Code offre l'attesa solo mentre un accesso claude.ai salvato è la credenziale attiva.

<h3 id="turn-automatic-continue-off">
  Disattivare la continuazione automatica
</h3>

In `/config`, disattivate **Continue automatically at usage limit**, oppure impostate [`autoContinueAtUsageLimit`](/docs/it/settings-reference#autocontinueatusagelimit) su `false` nelle vostre impostazioni utente. `/config autoContinueAtUsageLimit=false` funziona anche, incluso con `-p`, ma la forma `key=value` non può riattivarlo, perché l'impostazione concede l'esecuzione incustodita. Quali file di impostazioni Claude Code legge per questa chiave è nel [riferimento delle impostazioni](/docs/it/settings-reference#autocontinueatusagelimit).

<h2 id="pr-review-status">
  Stato della revisione PR
</h2>

Quando si lavora su un ramo con una pull request aperta, Claude Code visualizza un collegamento PR cliccabile nel footer, ad esempio "PR #446". Il collegamento ha una sottolineatura colorata che indica lo stato della revisione:

* Verde: approvato
* Giallo: revisione in sospeso
* Rosso: modifiche richieste
* Grigio: bozza

Il badge scompare una volta che la pull request viene unita o chiusa.

`Cmd+click` (macOS) o `Ctrl+click` (Windows/Linux) sul collegamento per aprire la pull request nel vostro browser.

Lo stato si aggiorna non appena un `git push`, o un comando `gh pr` che modifica la pull request, come `gh pr create` o `gh pr merge`, ha successo nella sessione.

Claude Code visualizza il badge come un collegamento ipertestuale anche quando non riesce a rilevare il supporto dei collegamenti ipertestuali nel vostro terminale, il che accade comunemente su SSH o in tmux. Impostare [`FORCE_HYPERLINK=0`](/docs/it/env-vars) per visualizzare il badge come testo semplice.

Quando impostate [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/it/env-vars), Claude Code non controlla lo stato della pull request o della merge request.

<Note>
  Lo stato PR per i repository GitHub richiede un token GitHub. Claude Code ne trova uno in base all'host del remote:

  * **github.com**: `GH_TOKEN` o `GITHUB_TOKEN`, o il token salvato da `gh auth login`. Senza uno, il footer mostra `install gh for PR status` quando la CLI `gh` non è installata, o `gh auth login for PR status` quando lo è
  * **Un host GitHub Enterprise impostato come `GH_HOST`**: `GH_ENTERPRISE_TOKEN` o `GITHUB_ENTERPRISE_TOKEN`, o il token salvato da `gh auth login --hostname <host>`. Senza uno, il footer mostra gli stessi suggerimenti
  * **Qualsiasi altro host GitHub**: il token salvato da `gh auth login --hostname <host>`. Senza uno, Claude Code non mostra alcun badge e nessun suggerimento
</Note>

<h3 id="gitlab-merge-requests">
  Merge request GitLab
</h3>

Quando lavorate su un ramo con una merge request GitLab aperta, Claude Code mostra un badge `MR !N` cliccabile nello slot del footer che altrimenti contiene il collegamento GitHub PR. `!N` è la sintassi di riferimento di GitLab per il numero di merge request N. La sottolineatura colorata mostra lo stato della merge request:

* Verde: GitLab segnala la merge request come unibile
* Giallo: qualsiasi altro stato aperto
* Grigio: bozza

Il badge scompare una volta che la merge request viene unita o chiusa.

Si aggiorna non appena un `git push`, o un comando `glab mr` che modifica la merge request, come `glab mr create` o `glab mr merge`, ha successo nella sessione.

Per ottenere il badge, avete bisogno di:

* Claude Code v2.1.234 o successivo
* Un remote del repository che punta al vostro host GitLab, sia gitlab.com che un'istanza auto-gestita
* La [`glab` CLI](https://gitlab.com/gitlab-org/cli) nel vostro `PATH`, autenticata con `glab auth login`

Claude Code ignora le variabili di ambiente del token di `glab`, come `GITLAB_TOKEN`, quando controlla lo stato, quindi non ottenete alcun badge da un token esportato da solo. Claude Code cerca anche `glab` e il suo login una volta per sessione, quindi riavviate Claude Code dopo aver installato `glab` o aver eseguito `glab auth login`.

<h2 id="issue-reference-links">
  Link di riferimento ai problemi
</h2>

Quando Claude menziona un problema come `owner/repo#123`, potete fare clic sul riferimento per aprirlo, purché il vostro terminale supporti i hyperlink. Se Claude Code non rileva il supporto dei hyperlink nel vostro terminale, impostate [`FORCE_HYPERLINK`](/docs/it/env-vars) su `1` per attivare i link, oppure su `0` per mantenere i riferimenti come testo semplice.

Ottenete un link solo per il modulo a due parti `owner/repo#123`. Questi rimangono come testo semplice:

* Un `#123` isolato
* Un percorso GitLab annidato come `group/subgroup/project#123`
* Qualsiasi riferimento all'interno di uno span di codice o di un blocco di codice

Claude Code costruisce il link per l'host del repository che identifica dal vostro git remote, non per il repository che il riferimento nomina:

| Host del vostro repository                                                      | Dove `owner/repo#123` si collega                       |
| :------------------------------------------------------------------------------ | :----------------------------------------------------- |
| github.com, un host GitHub Enterprise, o qualsiasi host non elencato di seguito | `https://<host>/owner/repo/issues/123`                 |
| gitlab.com                                                                      | `https://gitlab.com/owner/repo/-/issues/123`           |
| bitbucket.org, codeberg.org, o gitea.com                                        | Nessun link; il riferimento rimane come testo semplice |

<h2 id="see-also">
  Vedere anche
</h2>

* [Skills](/docs/it/skills) - Prompt personalizzati e flussi di lavoro
* [Checkpointing](/docs/it/checkpointing) - Riavvolgi le modifiche di Claude e ripristina gli stati precedenti
* [Riferimento CLI](/docs/it/cli-reference) - Flag e opzioni della riga di comando
* [Impostazioni](/docs/it/settings) - Opzioni di configurazione
* [Gestione della memoria](/docs/it/memory) - Gestione dei file CLAUDE.md
