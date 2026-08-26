> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Rendering a schermo intero

> Abilita una modalità di rendering più fluida e senza sfarfallio con supporto del mouse e utilizzo stabile della memoria nelle conversazioni lunghe.

<Note>
  Il rendering a schermo intero è un'[anteprima di ricerca](#research-preview) opt-in. Eseguire `/tui fullscreen` per passare nella conversazione corrente. Il comportamento potrebbe cambiare in base al feedback.
</Note>

Il rendering a schermo intero è un percorso di rendering alternativo per la CLI di Claude Code che elimina lo sfarfallio, mantiene l'utilizzo della memoria costante nelle conversazioni lunghe e aggiunge il supporto del mouse. Disegna l'interfaccia sul buffer dello schermo alternativo del terminale, come `vim` o `htop`, e renderizza solo i messaggi attualmente visibili. Questo riduce la quantità di dati inviati al terminale ad ogni aggiornamento.

La differenza è più evidente negli emulatori di terminale dove il throughput di rendering è il collo di bottiglia, come il terminale integrato di VS Code, tmux e iTerm2. Se la posizione di scorrimento del terminale salta in alto mentre Claude sta lavorando, o lo schermo lampeggia mentre l'output dello strumento viene trasmesso, questa modalità affronta questi problemi.

<Note>
  Il termine schermo intero descrive come Claude Code si impadronisce della superficie di disegno del terminale, come fa `vim`. Non ha nulla a che fare con la massimizzazione della finestra del terminale e funziona a qualsiasi dimensione della finestra.
</Note>

<h2 id="enable-fullscreen-rendering">
  Abilita il rendering a schermo intero
</h2>

Eseguire `/tui fullscreen` all'interno di qualsiasi conversazione di Claude Code. La CLI salva l'[impostazione `tui`](/docs/it/settings#available-settings) e si riavvia in modalità schermo intero con la conversazione intatta, quindi è possibile passare a metà sessione senza perdere il contesto. Eseguire `/tui default` per tornare al renderer classico, oppure `/tui` senza argomenti per stampare quale renderer è attivo.

La sessione riavviata mantiene la conversazione così come appare sullo schermo. Se in precedenza nella sessione è stato eseguito [`/rewind`](/docs/it/checkpointing#rewind-and-summarize), il riavvio riprende dal punto di rewind anziché dalla trascrizione più lunga salvata su disco. Prima della versione 2.1.207, il passaggio tra renderer dopo un rewind ripristinava la conversazione che il rewind aveva rimosso.

È inoltre possibile impostare la variabile di ambiente `CLAUDE_CODE_NO_FLICKER` prima di avviare Claude Code:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 claude
```

L'impostazione `tui` e la variabile di ambiente sono equivalenti. Il comando `/tui` cancella `CLAUDE_CODE_NO_FLICKER` dal processo riavviato in modo che l'impostazione che scrive abbia effetto.

<h2 id="what-changes">
  Cosa cambia
</h2>

Il rendering a schermo intero cambia il modo in cui la CLI disegna sul terminale. La casella di input rimane fissa in fondo allo schermo invece di muoversi mentre l'output viene trasmesso. Se l'input rimane fermo mentre Claude sta lavorando, il rendering a schermo intero è attivo. Solo i messaggi visibili vengono mantenuti nell'albero di rendering, quindi la memoria rimane costante indipendentemente dalla lunghezza della conversazione.

Poiché la conversazione vive nel buffer dello schermo alternativo invece dello scrollback del terminale, alcune cose funzionano diversamente:

| Prima                                                               | Ora                                                                                               | Dettagli                                                               |
| :------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------- |
| `Cmd+f` o ricerca tmux per trovare testo                            | `Ctrl+o` per la modalità trascrizione, quindi `/` per cercare o `[` per scrivere nello scrollback | [Cerca e rivedi la conversazione](#search-and-review-the-conversation) |
| Clic e trascinamento nativo del terminale per selezionare e copiare | Selezione in-app, copia automatica al rilascio del mouse                                          | [Usa il mouse](#use-the-mouse)                                         |
| `Cmd`-clic per aprire un URL                                        | `Cmd`-clic su macOS, `Ctrl`-clic altrove                                                          | [Usa il mouse](#use-the-mouse)                                         |

Se l'acquisizione del mouse interferisce con il tuo flusso di lavoro, puoi [disattivarla](#keep-native-text-selection) mantenendo il rendering senza sfarfallio.

<h2 id="use-the-mouse">
  Usa il mouse
</h2>

Il rendering a schermo intero acquisisce gli eventi del mouse e li gestisce all'interno di Claude Code:

* **Fai clic nella casella di input del prompt** per posizionare il cursore in qualsiasi punto del testo che stai digitando.
* **Fai clic su un suggerimento nel comando `/` o nell'elenco di file `@`** per accettarlo. Passare il mouse evidenzia la riga sotto il cursore.
* **Fai clic su un'opzione in un menu di selezione** per sceglierla. Questo copre i prompt di autorizzazione, `/model`, `/config` e altri dialoghi che mostrano un elenco di opzioni. Passare il mouse mostra un puntatore sulla riga sotto il cursore. Richiede Claude Code v2.1.187 o successivo.
* **Fai clic su un'opzione in un menu di selezione multipla** per attivare/disattivare l'opzione, e fai clic sul pulsante di invio per confermare le tue scelte. Facendo clic su una riga di testo libero, come la riga `Other` in una domanda a scelta multipla, metti a fuoco il suo campo di input in modo da poter digitare una risposta. Richiede Claude Code v2.1.208 o successivo.
* **Fai clic su un risultato dello strumento compresso** per espanderlo e vedere l'output completo. Fai clic di nuovo per comprimerlo. La chiamata dello strumento e il suo risultato si espandono insieme. Solo i messaggi che hanno più da mostrare sono cliccabili.
* **Tieni premuto `Cmd` su macOS, o `Ctrl` su Linux e Windows, e fai clic su un URL o un percorso di file** per aprirlo. I percorsi di file nell'output dello strumento, come quelli stampati dopo un Edit o Write, si aprono nell'applicazione predefinita. Gli URL `http://` e `https://` semplici si aprono nel browser. A partire dalla v2.1.181, un semplice clic senza tenere premuto `Cmd` o `Ctrl` non apre più i link, corrispondendo al comportamento del terminale nativo. Alcuni terminali macOS inoltrano `Cmd`+clic all'app in esecuzione invece di aprire il link stessi, e il protocollo del mouse del terminale non ha modo di codificare il tasto `Cmd`, quindi Claude Code lo riceve come un semplice clic. In Ghostty, e a partire dalla v2.1.198 in Warp su macOS, Claude Code rileva questo e consente a un semplice clic su un link di aprirlo, e tenere premuto `Cmd` funziona ancora. Nel terminale integrato di VS Code e nei terminali basati su xterm.js simili, Claude Code si rimette al gestore di link del terminale, che utilizza lo stesso gesto.
* **Fai clic e trascinamento** per selezionare il testo in qualsiasi punto della conversazione. Il doppio clic seleziona una parola, corrispondendo ai confini delle parole di iTerm2 in modo che un percorso di file si selezioni come un'unità. A partire dalla v2.1.198, il doppio clic su un URL seleziona l'intero URL, incluso lo schema. Il triplo clic seleziona la riga.
* **Scorri con la rotella del mouse** per muoverti attraverso la conversazione.

Il testo selezionato viene copiato negli appunti automaticamente al rilascio del mouse. Per disattivarlo, attiva/disattiva Copia alla selezione in `/config`.

Con Copia alla selezione disattivato, premi `Ctrl+Shift+c` per copiare manualmente. Sui terminali che supportano il protocollo della tastiera kitty, come kitty, WezTerm, Ghostty e iTerm2, funziona anche `Cmd+c`. Se hai una selezione attiva, `Ctrl+c` copia invece di annullare.

Con una selezione attiva, tieni premuto `Shift` e premi i tasti freccia per estenderla dalla tastiera. `Shift+↑` e `Shift+↓` scorrono il viewport quando la selezione raggiunge il bordo superiore o inferiore. `Shift+Home` e `Shift+End` estendono all'inizio o alla fine della riga corrente.

<h2 id="scroll-the-conversation">
  Scorri la conversazione
</h2>

Il rendering a schermo intero gestisce lo scorrimento all'interno dell'app. Usa queste scorciatoie per navigare:

| Scorciatoia       | Azione                                                          |
| :---------------- | :-------------------------------------------------------------- |
| `PgUp` / `PgDn`   | Scorri su o giù di mezzo schermo                                |
| `Ctrl+Home`       | Salta all'inizio della conversazione                            |
| `Ctrl+End`        | Salta al messaggio più recente e riabilita il follow automatico |
| Rotella del mouse | Scorri di poche righe alla volta                                |

Su tastiere senza tasti dedicati `PgUp`, `PgDn`, `Home` o `End`, come le tastiere MacBook, tieni premuto `Fn` con i tasti freccia: `Fn+↑` invia `PgUp`, `Fn+↓` invia `PgDn`, `Fn+←` invia `Home` e `Fn+→` invia `End`. `Ctrl+Fn+→` non raggiunge Claude Code su macOS, quindi una tastiera MacBook non ha una scorciatoia funzionante per saltare al fondo per impostazione predefinita. Invece, usa una di queste opzioni:

* Fai clic sul [pulsante salta al fondo](#auto-follow).
* Scorri verso il basso con la rotella del mouse per riprendere il follow.
* Riassegna `scroll:bottom` a una scorciatoia che la tua tastiera può inviare.

Queste azioni sono riassegnabili. Vedi [Azioni di scorrimento](/docs/it/keybindings#scroll-actions) per l'elenco completo dei nomi delle azioni, incluse le varianti di mezza pagina e pagina intera che non hanno binding predefinito.

<h3 id="auto-follow">
  Follow automatico
</h3>

Lo scorrimento verso l'alto mette in pausa il follow automatico in modo che il nuovo output non ti riporti al fondo. Un pulsante `Salta al fondo` galleggia sul bordo inferiore della trascrizione mentre sei scorso verso l'alto e mostra un conteggio come `3 nuovi messaggi` quando arriva un nuovo output. Fai clic su di esso, premi `Ctrl+End` o scorri verso il basso per riprendere il follow.

Mentre il follow automatico è in pausa, la vista rimane anche dove l'hai scrollata quando una risposta finisce di trasmettere. Prima della v2.1.207, la vista potrebbe saltare sopra l'inizio della risposta quando una risposta lunga finiva di trasmettere.

Il suggerimento da tastiera del pulsante riflette ciò che la tua tastiera può inviare. Su macOS suggerisce di fare clic o `Fn+↓` per scorrere, perché `Ctrl+End` non raggiunge Claude Code da una tastiera Mac. Riassegna [`scroll:bottom`](/docs/it/keybindings#scroll-actions) e il pulsante mostra la tua scorciatoia su ogni piattaforma. Prima della v2.1.206, il pulsante suggeriva `Ctrl+End` su macOS.

Su un terminale troppo stretto per l'etichetta completa, il pulsante accorcia il suggerimento invece di andare a capo sulla riga della trascrizione sottostante. Prima della v2.1.206, un'etichetta lunga potrebbe andare a capo sulla trascrizione.

Per disattivare completamente il follow automatico in modo che la vista rimanga dove la lasci, apri `/config` e imposta Auto-scroll su off. Con auto-scroll disabilitato, la vista non salta mai al fondo da sola. I prompt di autorizzazione e altri dialoghi che richiedono una risposta scorrono comunque in vista indipendentemente da questa impostazione.

<h3 id="mouse-wheel-scrolling">
  Scorrimento con la rotella del mouse
</h3>

Lo scorrimento con la rotella del mouse richiede che il terminale inoltri gli eventi del mouse a Claude Code. La maggior parte dei terminali lo fa ogni volta che un'applicazione lo richiede. iTerm2 lo rende un'impostazione per profilo: se la rotella non fa nulla ma `PgUp` e `PgDn` funzionano, apri Impostazioni → Profili → Terminale e attiva Abilita segnalazione mouse. La stessa impostazione è richiesta anche per il clic per espandere e la selezione di testo per funzionare.

Se lo scorrimento con la rotella del mouse sembra lento, il terminale potrebbe inviare un evento di scorrimento per ogni tacca fisica senza moltiplicatore. Alcuni terminali, come Ghostty e iTerm2 con scorrimento più veloce abilitato, amplificano già gli eventi della rotella. Altri, incluso il terminale integrato di VS Code, inviano esattamente un evento per tacca. Claude Code non può rilevare quale.

Imposta `CLAUDE_CODE_SCROLL_SPEED` per moltiplicare la distanza di scorrimento di base:

```bash theme={null}
export CLAUDE_CODE_SCROLL_SPEED=3
```

Un valore di `3` corrisponde al valore predefinito in `vim` e applicazioni simili. L'impostazione accetta valori da 1 a 20 e valori frazionari inferiori a 1 come `0.5` per rallentare lo scorrimento accelerato del trackpad e della rotella del mouse nei terminali che già amplificano gli eventi della rotella.

Per regolare la velocità di scorrimento in modo interattivo, esegui `/scroll-speed`. La finestra di dialogo mostra un righello che puoi scorrere mentre è aperta in modo da poter sentire il cambiamento immediatamente. Premi `←` e `→` per regolare, `r` per ripristinare il valore predefinito rilevato automaticamente e `Invio` per salvare.

Il comando scrive lo stesso valore che la variabile di ambiente `CLAUDE_CODE_SCROLL_SPEED` imposta, persistito in `~/.claude/settings.json`. Il comando non è disponibile nel terminale dell'IDE JetBrains.

Separatamente dalla velocità di base, Claude Code accelera la velocità di scorrimento quando fai girare la rotella rapidamente, quindi una rotazione veloce copre più distanza rispetto allo stesso numero di tacche lente. Per disattivare l'accelerazione e mantenere una velocità costante per tacca, imposta `wheelScrollAccelerationEnabled` su `false` in [`settings.json`](/docs/it/settings#available-settings). Questa impostazione richiede Claude Code v2.1.174 o versioni successive.

<h3 id="scroll-in-the-jetbrains-ide-terminal">
  Scorrimento nel terminale dell'IDE JetBrains
</h3>

Nel terminale dell'IDE JetBrains, Claude Code applica la propria gestione dello scorrimento e ignora `CLAUDE_CODE_SCROLL_SPEED`. Il terminale invia eventi di scorrimento a una velocità molto più elevata rispetto ad altri emulatori, quindi un moltiplicatore sintonizzato altrove va oltre qui.

Nel 2025.2, il terminale ha anche bug di scorrimento con la rotella che producono tasti freccia spuri e eventi di direzione sbagliata. Claude Code rileva questi in fase di esecuzione e li mitiga automaticamente, quindi lo scorrimento con trackpad e rotella del mouse funzionano senza configurazione. Per la migliore esperienza di scorrimento, esegui l'aggiornamento a 2025.3 o versioni successive. Claude Code mostra un suggerimento la prima volta che scorri se rileva il bug.

<h2 id="search-and-review-the-conversation">
  Cerca e rivedi la conversazione
</h2>

`Ctrl+o` attiva/disattiva tra il prompt normale e la modalità trascrizione.

Per una vista più silenziosa che mostra solo l'ultimo prompt, un riassunto di una riga delle chiamate dello strumento con diffstat di modifica e la risposta finale, esegui `/focus`. L'impostazione persiste tra le sessioni. Esegui `/focus` di nuovo per disattivarla.

La modalità trascrizione guadagna navigazione e ricerca in stile `less`:

| Tasto                               | Azione                                                                                                                                    |
| :---------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| `/`                                 | Apri la ricerca. Digita per trovare corrispondenze, `Enter` per accettare, `Esc` per annullare e ripristinare la posizione di scorrimento |
| `n` / `N`                           | Salta alla corrispondenza successiva o precedente. Funziona dopo aver chiuso la barra di ricerca                                          |
| `j` / `k` o `↑` / `↓`               | Scorri una riga                                                                                                                           |
| `g` / `G` o `Home` / `End`          | Salta all'inizio o alla fine                                                                                                              |
| `Ctrl+u` / `Ctrl+d`                 | Scorri mezza pagina                                                                                                                       |
| `Ctrl+b` / `Ctrl+f` o `Space` / `b` | Scorri una pagina intera                                                                                                                  |
| `Ctrl+o`, `Esc`, o `q`              | Esci dalla modalità trascrizione e torna al prompt                                                                                        |

Il `Cmd+f` del terminale e la ricerca tmux non vedono la conversazione perché vive nel buffer dello schermo alternativo, non nello scrollback nativo. Per restituire il contenuto al terminale, premi `Ctrl+o` per entrare prima in modalità trascrizione, quindi:

* **`[`**: scrive la conversazione completa nel buffer dello scrollback nativo del terminale, con tutto l'output dello strumento espanso. La conversazione è ora testo ordinario nel terminale, quindi `Cmd+f`, la modalità copia tmux e qualsiasi altro strumento nativo può cercarla o selezionarla. Le sessioni lunghe potrebbero fare una pausa per un momento mentre questo accade. Questo dura fino a quando non esci dalla modalità trascrizione con `Esc` o `q`, che ti riporta al rendering a schermo intero. Il prossimo `Ctrl+o` ricomincia da capo.
* **`v`**: scrive la conversazione in un file temporaneo e lo apre in `$VISUAL` o `$EDITOR`.

Premi `Esc` o `q` per tornare al prompt.

<h2 id="clear-the-conversation">
  Cancella la conversazione
</h2>

Premi `Ctrl+L` due volte entro due secondi per eseguire `/clear` e avviare una nuova conversazione. Il primo pressione ridisegna lo schermo e mostra un suggerimento; il secondo pressione cancella la conversazione. Su macOS, il doppio pressione di `Cmd+K` esegue anche `/clear`.

<h2 id="use-with-tmux">
  Usa con tmux
</h2>

Il rendering a schermo intero funziona all'interno di tmux, con tre avvertenze.

Lo scorrimento con la rotella del mouse richiede la modalità mouse di tmux. Se il tuo `~/.tmux.conf` non lo abilita già, aggiungi questa riga e ricarica la configurazione:

```bash theme={null}
set -g mouse on
```

Senza la modalità mouse, gli eventi della rotella vanno a tmux invece che a Claude Code. Lo scorrimento da tastiera con `PgUp` e `PgDn` funziona comunque. Claude Code stampa un suggerimento una tantum all'avvio se rileva tmux con la modalità mouse disattivata.

Il rendering a schermo intero è incompatibile con la modalità di integrazione tmux di iTerm2, che è la modalità in cui entri con `tmux -CC`. In modalità integrazione, iTerm2 renderizza ogni riquadro tmux come una divisione nativa piuttosto che lasciare che tmux disegni sul terminale. Il buffer dello schermo alternativo e il tracciamento del mouse non funzionano correttamente lì: la rotella del mouse non fa nulla e il doppio clic può corrompere lo stato del terminale. Non abilitare il rendering a schermo intero nelle sessioni `tmux -CC`. Il tmux regolare all'interno di iTerm2, senza `-CC`, funziona bene.

Non ogni versione di tmux applica l'output sincronizzato dalle applicazioni, quindi potresti vedere più sfarfallio durante i ridisegni sotto tmux rispetto a quando esegui Claude Code direttamente nel tuo terminale. Se lo sfarfallio è evidente, specialmente su SSH, aggiorna a tmux più recente o esegui Claude Code nella sua propria scheda terminale al di fuori di tmux. Controlla la versione di tmux con `tmux -V`.

Claude Code attiva l'output sincronizzato automaticamente quando rileva tmux 3.4 o successivo dalla variabile `TERM_PROGRAM_VERSION`, e ricade alla query diretta del terminale per il supporto dell'output sincronizzato quando la versione non può essere determinata. Se i ridisegni diventano effettivamente atomici dipende dalla versione di tmux che onora l'output sincronizzato; se continui a vedere sfarfallio sotto tmux 3.4 o successivo, aggiorna a tmux più recente. Questo rilevamento richiede Claude Code v2.1.200 o successivo.

<h2 id="keep-native-text-selection">
  Mantieni la selezione di testo nativa
</h2>

L'acquisizione del mouse è il punto di attrito più comune, specialmente su SSH o all'interno di tmux. Quando Claude Code acquisisce gli eventi del mouse, la copia nativa al rilascio della selezione del terminale smette di funzionare. La selezione che fai con clic e trascinamento esiste all'interno di Claude Code, non nel buffer di selezione del terminale, quindi la modalità copia tmux, i suggerimenti Kitty e strumenti simili non la vedono.

Claude Code scrive la selezione negli appunti di sistema, e il percorso che utilizza dipende dalla configurazione. In una sessione locale esegue uno strumento di appunti nativo:

* **macOS**: `pbcopy`
* **Linux**: `wl-copy` su Wayland, oppure `xclip` o `xsel` su X11, a seconda di quale sia installato. Claude Code scrive sia gli appunti che la selezione PRIMARY, quindi il clic centrale per incollare funziona.
* **Windows e WSL**: PowerShell `Set-Clipboard`

All'interno di tmux scrive anche nel buffer di incolla tmux. Su SSH ricade alle sequenze di escape OSC 52. Claude Code stampa un toast dopo ogni copia dicendoti quale percorso ha utilizzato.

Alcuni terminali bloccano OSC 52 per impostazione predefinita. iTerm2 lo blocca finché non attivi Impostazioni → Generale → Selezione → Le applicazioni nel terminale possono accedere agli appunti; eseguire [`/terminal-setup`](/docs/it/terminal-config) in iTerm2 abilita questo per te.

Per una selezione nativa una tantum, il tasto da utilizzare dipende dal terminale:

* **Terminal.app**: `Fn`
* **iTerm2**: `Option`
* **VS Code, Cursor e Devin Desktop**: `Shift`, oppure `Option` su macOS con l'impostazione `terminal.integrated.macOptionClickForcesSelection` abilitata
* **La maggior parte degli altri terminali**: `Shift`

Tieni premuto quel tasto mentre fai clic e trascini. Il terminale gestisce la selezione stesso invece di trasmetterla a Claude Code, quindi scorciatoie di copia come `Cmd+C` funzionano su ciò che selezioni. Claude Code mostra anche il tasto corretto nel suo suggerimento sullo schermo.

Su SSH o all'interno di tmux, Claude Code non sempre riesce a rilevare il terminale da cui ti stai connettendo, quindi il suggerimento elenca i tasti candidati.

Se fai affidamento sulla selezione nativa tutto il tempo, imposta `CLAUDE_CODE_DISABLE_MOUSE=1` per rinunciare all'acquisizione del mouse mantenendo il rendering senza sfarfallio e la memoria piatta:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 CLAUDE_CODE_DISABLE_MOUSE=1 claude
```

Con l'acquisizione del mouse disabilitata, lo scorrimento da tastiera con `PgUp`, `PgDn`, `Ctrl+Home` e `Ctrl+End` funziona ancora, e il terminale gestisce la selezione in modo nativo. Perdi il clic per posizionare il cursore, il clic per espandere l'output dello strumento, il clic su URL e lo scorrimento della rotella all'interno di Claude Code.

Per mantenere lo scorrimento della rotella ma disattivare il clic, il trascinamento e la gestione del passaggio del mouse, imposta invece `CLAUDE_CODE_DISABLE_MOUSE_CLICKS=1`. Richiede Claude Code v2.1.195 o successivo. `CLAUDE_CODE_DISABLE_MOUSE` ha la precedenza quando entrambe le variabili sono impostate.

Con i clic disabilitati, Claude Code continua a catturare il mouse, quindi la rotella e il touchpad scorrono la conversazione ma i clic sinistri non fanno nulla all'interno di Claude Code. Devi comunque tenere premuto il tasto del terminale per la selezione nativa con clic e trascinamento. Il clic destro e il clic centrale per incollare continuano a funzionare sui terminali che li supportano.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="stale-or-misplaced-text-on-screen">
  Testo obsoleto o posizionato male sullo schermo
</h3>

Il rendering a schermo intero invia solo le celle che sono cambiate tra i fotogrammi. Alcuni terminali, più comunemente Windows Terminal e altri host supportati da ConPTY, coalescono questi scritti posizionati in modo non corretto e lasciano frammenti dell'output precedente sullo schermo fino a quando non ridimensionate la finestra.

Impostare [`CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT=1`](/docs/it/env-vars) per ridisegnare ogni cella su ogni fotogramma invece di inviare aggiornamenti incrementali.

Su Windows PowerShell:

```powershell theme={null}
$env:CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT = "1"
claude
```

Su macOS o Linux:

```bash theme={null}
CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT=1 claude
```

Su Windows, Claude Code abilita già il ridisegno completo automaticamente per le sessioni in background e la [visualizzazione agente](/docs/it/agent-view), quindi è necessario impostare la variabile solo per una sessione fullscreen interattiva che avete avviato direttamente.

<h2 id="research-preview">
  Anteprima di ricerca
</h2>

Il rendering a schermo intero è una funzione di anteprima di ricerca. È stato testato su emulatori di terminale comuni, ma potresti incontrare problemi di rendering su terminali meno comuni o configurazioni insolite.

Se riscontri un problema, esegui `/feedback` all'interno di Claude Code per segnalarlo, o apri un problema nel [repository GitHub di claude-code](https://github.com/anthropics/claude-code/issues). Includi il nome e la versione dell'emulatore del terminale.

Per disattivare il rendering a schermo intero, esegui `/tui default`, o annulla l'impostazione di `CLAUDE_CODE_NO_FLICKER` se l'hai abilitata in quel modo. Per forzare il renderer classico indipendentemente dall'impostazione `tui` salvata, imposta `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`. Il renderer classico mantiene la conversazione nello scrollback nativo del tuo terminale, quindi `Cmd+f` e la modalità di copia di tmux funzionano come al solito.

Le sessioni in background aperte dalla [visualizzazione agente](/docs/it/agent-view) o da `claude attach` utilizzano sempre il rendering a schermo intero. Il terminale di collegamento entra nel buffer dello schermo alternativo per mostrare la sessione, e il renderer classico non ha scrollback o gestione del mouse lì, quindi l'impostazione `tui` e `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN` non si applicano a loro.
