> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Rendering a schermo intero

> Abilita una modalità di rendering più fluida e senza sfarfallio con supporto del mouse e utilizzo stabile della memoria nelle conversazioni lunghe.

<Note>
  Il rendering a schermo intero è un'[anteprima di ricerca](#research-preview). Se [avvii in modalità schermo intero o nel renderer classico](#fullscreen-by-default) dipende dalla tua configurazione. Esegui `/tui fullscreen` o `/tui default` per passare nella conversazione corrente. Il comportamento potrebbe cambiare in base al feedback.
</Note>

Il rendering a schermo intero è un percorso di rendering alternativo per la CLI di Claude Code che elimina lo sfarfallio, mantiene l'utilizzo della memoria costante nelle conversazioni lunghe e aggiunge il supporto del mouse. Disegna l'interfaccia sul buffer dello schermo alternativo del terminale, come `vim` o `htop`, e renderizza solo i messaggi attualmente visibili. Questo riduce la quantità di dati inviati al terminale ad ogni aggiornamento.

La differenza è più evidente negli emulatori di terminale dove il throughput di rendering è il collo di bottiglia, come il terminale integrato di VS Code, tmux e iTerm2. Se la posizione di scorrimento del terminale salta in alto mentre Claude sta lavorando, o lo schermo lampeggia mentre l'output dello strumento viene trasmesso, questa modalità affronta questi problemi.

<Note>
  Il termine schermo intero descrive come Claude Code si impadronisce della superficie di disegno del terminale, come fa `vim`. Non ha nulla a che fare con la massimizzazione della finestra del terminale e funziona a qualsiasi dimensione della finestra.
</Note>

<h2 id="enable-fullscreen-rendering">
  Abilita il rendering a schermo intero
</h2>

Eseguire `/tui fullscreen` all'interno di qualsiasi conversazione di Claude Code. La CLI salva l'[impostazione `tui`](/docs/it/settings-reference#tui) e si riavvia in modalità schermo intero con la conversazione intatta, quindi è possibile passare a metà sessione senza perdere il contesto. Eseguire `/tui default` per tornare al renderer classico, oppure `/tui` senza argomenti per stampare quale renderer è attivo.

In [modalità lettore di schermo](/docs/it/accessibility), Claude Code utilizza sempre il renderer classico tranne nelle [sessioni in background](/docs/it/agent-view) allegate, che ancora eseguono il rendering a schermo intero. Se si esegue `/tui fullscreen` in qualsiasi altra sessione, Claude Code stampa una spiegazione invece di passare e non modifica l'impostazione `tui` salvata.

Claude Code trasporta questi elementi nella sessione riavviata:

* La conversazione così come appare sullo schermo. Dopo un [`/rewind`](/docs/it/checkpointing#rewind-and-summarize), questo significa:
  * Se è stato eseguito un rewind in precedenza nella sessione, Claude Code si riavvia dal punto di rewind, non dalla trascrizione più lunga salvata su disco. Ad esempio, se è stato eseguito un rewind oltre i tre ultimi messaggi, la sessione riavviata si apre senza di essi
  * Se è stato eseguito un rewind prima del primo messaggio, Claude Code si riavvia con una conversazione vuota
* La [modalità di autorizzazione](/docs/it/permission-modes) e il [livello di impegno](/docs/it/model-config#adjust-effort-level)
* Il modello selezionato per ultimo con [`/model`](/docs/it/model-config#setting-your-model)
* Le regole passate con [`--allowed-tools` o `--disallowed-tools`](/docs/it/cli-reference#cli-flags), e i flag `--agent`, `--agents`, `--append-system-prompt`, e `--system-prompt-snapshot`

Claude Code rifiuta di riavviarsi se la sessione ha una restrizione che non può passare al processo riavviato. Le restrizioni che non può passare includono:

* Flag di avvio come una sostituzione [`--system-prompt`](/docs/it/cli-reference#cli-flags), un allowlist [`--tools`](/docs/it/cli-reference#cli-flags), o [`--setting-sources`](/docs/it/cli-reference#cli-flags)
* Regole di negazione o richiesta che un [aggiornamento delle autorizzazioni di hook o SDK](/docs/it/hooks#permission-update-entries) ha aggiunto solo per questa sessione

In questo caso Claude Code stampa [`Cannot switch renderers in this session`](/docs/it/errors#cannot-switch-renderers-in-this-session) con i motivi. Non passa o salva nulla.

È inoltre possibile impostare la variabile di ambiente `CLAUDE_CODE_NO_FLICKER` prima di avviare Claude Code:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 claude
```

Per come l'[impostazione `tui`](/docs/it/settings-reference#tui) e la variabile si combinano quando entrambe sono impostate, vedere la voce dell'impostazione. Dopo un [avvio a schermo intero non riuscito](#fullscreen-renderer-didnt-finish-starting), Claude Code onora ancora la variabile ma non l'impostazione. Il comando `/tui` cancella `CLAUDE_CODE_NO_FLICKER` dal processo riavviato in modo che l'impostazione che scrive abbia effetto.

<h3 id="fullscreen-by-default">
  Schermo intero per impostazione predefinita
</h3>

Le [sessioni in background](/docs/it/agent-view) allegate eseguono il rendering a schermo intero, e altre sessioni in [modalità lettore di schermo](/docs/it/accessibility) utilizzano il renderer classico. Altrimenti, Claude Code avvia nel renderer della prima riga di questa tabella che corrisponde alla configurazione:

| La tua situazione                                                                                                                                                                                                | Renderer in cui inizi                 |
| :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------ |
| Hai impostato [`CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`](/docs/it/env-vars) o `CLAUDE_CODE_NO_FLICKER=0`                                                                                                              | Classico                              |
| Hai impostato `CLAUDE_CODE_NO_FLICKER=1`                                                                                                                                                                         | Schermo intero                        |
| Claude Code ha [disattivato lo schermo intero dopo un avvio a schermo intero non riuscito](#fullscreen-renderer-didnt-finish-starting) su questo computer                                                        | Classico                              |
| Sei nella modalità di integrazione [`tmux -CC`](#use-with-tmux) di iTerm2, oppure sei connesso tramite SSH a Claude Code in esecuzione su Windows                                                                | Classico                              |
| Hai salvato un'[impostazione `tui`](/docs/it/settings-reference#tui)                                                                                                                                                  | Il renderer che l'impostazione nomina |
| La tua sessione non [recupera i flag di funzionalità da Anthropic](/docs/it/env-vars#features-that-need-feature-flag-fetching), e Claude Code ha smesso di offrire la finestra di dialogo di avvio su questo computer | Classico                              |
| La tua sessione non recupera i flag di funzionalità da Anthropic, e il primo avvio di Claude Code su questo computer ha eseguito v2.1.239 o successivo                                                           | Schermo intero                        |
| La tua sessione recupera i flag di funzionalità da Anthropic, e hai utilizzato per la prima volta Claude Code il 6 maggio 2026 o successivamente                                                                 | Schermo intero                        |
| Qualsiasi altra cosa                                                                                                                                                                                             | Classico                              |

Le sessioni che non recuperano i flag di funzionalità includono quelle tramite [Amazon Bedrock](/docs/it/amazon-bedrock), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai), o [Microsoft Foundry](/docs/it/microsoft-foundry), e quelle con telemetria disattivata.

Se inizi nel renderer classico e non hai salvato un'impostazione `tui`, Claude Code potrebbe aprire una finestra di dialogo all'avvio offrendo il passaggio:

* Se accetti, Claude Code si riavvia nello stesso modo in cui lo fa `/tui fullscreen`, trasportando lo stesso stato della sessione, e salva l'impostazione una volta che la sessione riavviata ha [iniziato con successo](#fullscreen-renderer-didnt-finish-starting).
* Se scegli **Non ora**, Claude Code non offre di nuovo su questo computer.
* Claude Code smette di offrire dopo aver mostrato la finestra di dialogo su tre avvii, risposte o meno.

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
  Usare il mouse
</h2>

Il rendering a schermo intero cattura gli eventi del mouse e li gestisce all'interno di Claude Code:

* **Fare clic nell'input del prompt** per posizionare il cursore in qualsiasi punto del testo che state digitando.
* **Fare clic su un suggerimento nel comando `/` o nell'elenco di file `@`** per accettarlo. Passare il mouse evidenzia la riga sotto il cursore.
* **Fare clic su un'opzione in un menu di selezione** per sceglierla. Questo copre i prompt di autorizzazione, `/model`, `/config` e altri dialoghi che mostrano un elenco di opzioni. Passare il mouse mostra un puntatore sulla riga sotto il cursore.
* **Fare clic su un'opzione in un menu di selezione multipla** per attivarla/disattivarla, e fare clic sul pulsante di invio per confermare le scelte. Fare clic su una riga di testo libero, come la riga `Other` in una domanda a scelta multipla, mette a fuoco il suo campo di input in modo da poter digitare una risposta. Richiede Claude Code v2.1.208 o successivo.
* **Fare clic sul valore di un'impostazione nel pannello `/config`** per modificarlo, e scorrere l'elenco delle impostazioni con la rotella del mouse. Richiede Claude Code v2.1.271 o successivo.
* **Scorrere un menu di selezione o selezione multipla con la rotella del mouse** quando ha più opzioni di quante ne mostri contemporaneamente, come l'elenco `/model` in una finestra del terminale breve. La rotella scorre l'elenco mentre il puntatore si trova sopra le sue opzioni. Richiede Claude Code v2.1.280 o successivo.
* **Fare clic su un risultato dello strumento compresso** per espanderlo e vedere l'output completo. Fare clic di nuovo per comprimerlo. La chiamata dello strumento e il suo risultato si espandono insieme. Solo i messaggi che hanno più da mostrare sono cliccabili.
  * Fare clic espande anche l'output di un comando shell `!`, sia un risultato troncato più vecchio che la riga di progresso in tempo reale mentre il comando è in esecuzione. Richiede Claude Code v2.1.257 o successivo.
* **Tenere premuto `Cmd` su macOS, o `Ctrl` su Linux e Windows, e fare clic su un URL o un percorso di file** per aprirlo. Gli URL semplici `http://` e `https://` si aprono nel browser, e i percorsi di file nell'output dello strumento, come quelli stampati dopo un Edit o Write, si aprono nell'applicazione predefinita. Un semplice clic senza il modificatore non apre i link, corrispondendo al comportamento nativo del terminale.
  * Claude Code esegue il rendering di un percorso di rete (UNC), come `\\server\share\file.ts`, come testo semplice senza link, perché aprire un percorso di rete può inviare le credenziali di Windows all'host che nomina.
  * Alcuni terminali macOS inoltrano `Cmd`+clic all'app in esecuzione invece di aprire il link da soli, e il protocollo del mouse del terminale non ha modo di codificare il tasto `Cmd`, quindi Claude Code riceve un semplice clic. In Ghostty, e in Warp su macOS, Claude Code lo rileva e consente a un semplice clic su un link di aprirlo, e tenere premuto `Cmd` funziona ancora.
  * Nel terminale integrato di VS Code e in terminali simili basati su xterm.js, Claude Code si affida al gestore di link del terminale stesso, che utilizza lo stesso gesto.
* **Fare clic e trascinare** per selezionare il testo in qualsiasi punto della conversazione. Un doppio clic seleziona una parola, corrispondendo ai confini delle parole di iTerm2 in modo che un percorso di file si selezioni come un'unità. Un doppio clic su un URL seleziona l'intero URL, incluso lo schema. Un triplo clic seleziona la riga.
* **Scorrere con la rotella del mouse** per muoversi attraverso la conversazione.

Il testo selezionato viene copiato automaticamente negli appunti al rilascio del mouse. Per disattivare questa funzione, attivate/disattivate Copy on select in `/config`.

Con Copy on select disattivato, premete `Ctrl+Shift+c` per copiare manualmente. Su terminali che supportano il protocollo della tastiera kitty, come kitty, WezTerm, Ghostty e iTerm2, funziona anche `Cmd+c`. Se avete una selezione attiva, `Ctrl+c` copia invece di annullare.

Con una selezione attiva, tenete premuto `Shift` e premete i tasti freccia per estenderla dalla tastiera. `Shift+↑` e `Shift+↓` scorrono il viewport quando la selezione raggiunge il bordo superiore o inferiore. `Shift+Home` e `Shift+End` estendono all'inizio o alla fine della riga corrente.

Nella vista del prompt normale, quello che accade a una selezione attiva dipende dal tasto che premete:

* **`Esc`**: Claude Code esegue l'azione usuale del tasto, come interrompere la risposta in esecuzione o chiudere un dialogo aperto, e la selezione rimane evidenziata.
* **`PgUp`, `PgDn`, `Ctrl+Home`, `Ctrl+End`, o `Shift`, `Alt` o `Option`, o `Cmd`, `Win`, o `Super` con un tasto freccia, `Home`, o `End`**: la selezione rimane.
* **Qualsiasi altro tasto, inclusi i tasti freccia semplici, `Enter` e i caratteri digitati**: Claude Code cancella la selezione.
* **Un tasto associato a [`selection:clear`](/docs/it/keybindings#scroll-actions)**: Claude Code cancella la selezione, anche quando il tasto è `Esc` o un altro tasto che altrimenti la mantiene. L'azione non ha un binding predefinito.

In [modalità trascrizione](#search-and-review-the-conversation), i tasti di navigazione e ricerca elencati lì mantengono anche la selezione.

<h2 id="scroll-the-conversation">
  Scorrere la conversazione
</h2>

Il rendering a schermo intero gestisce lo scorrimento all'interno dell'app. Utilizzare queste scorciatoie da tastiera per navigare:

| Scorciatoia da tastiera | Azione                                                              |
| :---------------------- | :------------------------------------------------------------------ |
| `PgUp` / `PgDn`         | Scorrere verso l'alto o verso il basso di mezza schermata           |
| `Ctrl+Home`             | Saltare all'inizio della conversazione                              |
| `Ctrl+End`              | Saltare al messaggio più recente e riabilitare il follow automatico |
| Rotella del mouse       | Scorrere di poche righe alla volta                                  |

È possibile scorrere fino all'inizio della sessione anche dopo la [compattazione](/docs/it/context-window#what-survives-compaction). Claude continua a lavorare dal riepilogo di compattazione, ma Claude Code mantiene ogni messaggio precedente nello scrollback a schermo intero attraverso compattazioni ripetute.

Su tastiere senza tasti dedicati `PgUp`, `PgDn`, `Home` o `End`, come le tastiere MacBook, tenere premuto `Fn` insieme ai tasti freccia: `Fn+↑` invia `PgUp`, `Fn+↓` invia `PgDn`, `Fn+←` invia `Home` e `Fn+→` invia `End`. `Ctrl+Fn+→` non raggiunge Claude Code su macOS, quindi una tastiera MacBook non ha una combinazione di tasti funzionante per saltare al fondo per impostazione predefinita. Utilizzare invece una di queste opzioni:

* Fare clic sul [pulsante salta al fondo](#auto-follow).
* Scorrere fino al fondo con la rotella del mouse per riprendere il follow.
* Riassociare `scroll:bottom` a una combinazione di tasti che la tastiera può inviare.

Queste azioni sono riassociabili. Vedere [Azioni di scorrimento](/docs/it/keybindings#scroll-actions) per l'elenco completo dei nomi delle azioni, incluse le varianti di mezza pagina e pagina intera che non hanno un'associazione predefinita.

Mentre sei scorso verso l'alto, una riga di intestazione attenuata nella parte superiore della conversazione mostra il prompt più recente che è stato scorso al di sopra della vista. Fare clic sulla riga per saltare a quel prompt.

<h3 id="auto-follow">
  Follow automatico
</h3>

Lo scorrimento verso l'alto mette in pausa il follow automatico in modo che il nuovo output non ti riporti al fondo. Un pulsante `Salta al fondo` galleggia sul bordo inferiore della trascrizione mentre sei scorso verso l'alto e mostra un conteggio come `3 nuovi messaggi` quando arriva un nuovo output. Fare clic su di esso, premere `Ctrl+End` o scorrere fino al fondo per riprendere il follow.

Mentre il follow automatico è in pausa, la vista rimane anche dove l'hai scrollata quando una risposta finisce di trasmettere.

L'hint della tastiera del pulsante riflette ciò che la tastiera può inviare. Su macOS suggerisce di fare clic o `Fn+↓` per scorrere, perché `Ctrl+End` non raggiunge Claude Code da una tastiera Mac. Riassociare [`scroll:bottom`](/docs/it/keybindings#scroll-actions) e il pulsante mostra la combinazione di tasti su ogni piattaforma.

Su un terminale troppo stretto per l'etichetta completa, il pulsante accorcia l'hint invece di andare a capo sulla riga della trascrizione sottostante.

Per disattivare completamente il follow automatico in modo che la vista rimanga dove la lasci, apri `/config` e imposta Auto-scroll su off. Con lo scorrimento automatico disabilitato, la vista non salta mai al fondo da sola. I prompt di autorizzazione e altri dialoghi che richiedono una risposta scorrono comunque in vista indipendentemente da questa impostazione.

<h3 id="mouse-wheel-scrolling">
  Scorrimento con rotella del mouse
</h3>

Lo scorrimento con rotella del mouse richiede che il terminale inoltri gli eventi del mouse a Claude Code. La maggior parte dei terminali lo fa ogni volta che un'applicazione lo richiede. iTerm2 lo rende un'impostazione per profilo: se la rotella non fa nulla ma `PgUp` e `PgDn` funzionano, apri Impostazioni → Profili → Terminale e attiva Abilita segnalazione mouse. La stessa impostazione è richiesta anche per il clic per espandere e la selezione del testo.

Se lo scorrimento con rotella del mouse sembra lento, il terminale potrebbe inviare un evento di scorrimento per ogni tacca fisica senza moltiplicatore. Alcuni terminali, come Ghostty e iTerm2 con scorrimento più veloce abilitato, amplificano già gli eventi della rotella. Altri, incluso il terminale integrato di VS Code, inviano esattamente un evento per tacca. Claude Code non può rilevare quale.

Impostare `CLAUDE_CODE_SCROLL_SPEED` per moltiplicare la distanza di scorrimento di base:

```bash theme={null}
export CLAUDE_CODE_SCROLL_SPEED=3
```

Un valore di `3` corrisponde all'impostazione predefinita in `vim` e applicazioni simili. L'impostazione accetta qualsiasi valore positivo fino a 20, inclusi valori frazionari inferiori a 1 come `0.25` per rallentare lo scorrimento accelerato del trackpad e della rotella del mouse nei terminali che amplificano già gli eventi della rotella.

Per regolare la velocità di scorrimento in modo interattivo, eseguire `/scroll-speed`. La finestra di dialogo mostra un righello che è possibile scorrere mentre è aperta in modo da poter sentire il cambiamento immediatamente. Premere `←` e `→` per regolare la velocità, `r` per ripristinare il valore predefinito rilevato automaticamente e `Invio` per salvare. La finestra di dialogo procede a step di numeri interi fino a 10 e sui terminali che supportano un controllo più fine offre anche step di un quarto fino a 0,25.

Il comando scrive lo stesso valore impostato dalla variabile di ambiente `CLAUDE_CODE_SCROLL_SPEED`, persistito in `~/.claude/settings.json`. Il massimo della finestra di dialogo è 10: se imposti un valore più alto tramite la variabile di ambiente, la finestra di dialogo mostra 10 e il salvataggio dalla finestra di dialogo persiste 10. Il comando non è disponibile nel terminale IDE JetBrains.

Separatamente dalla velocità di base, Claude Code accelera la velocità di scorrimento quando fai girare la rotella rapidamente, quindi una rotazione veloce copre più distanza rispetto allo stesso numero di tacche lente. Per disattivare l'accelerazione e mantenere una velocità costante per tacca, impostare `wheelScrollAccelerationEnabled` su `false` in [`settings.json`](/docs/it/settings-reference#all-settings). Questa impostazione richiede Claude Code v2.1.174 o versione successiva.

<h3 id="scroll-in-the-jetbrains-ide-terminal">
  Scorrimento nel terminale IDE JetBrains
</h3>

Nel terminale IDE JetBrains, Claude Code applica la propria gestione dello scorrimento e ignora `CLAUDE_CODE_SCROLL_SPEED`. Il terminale invia eventi di scorrimento a una velocità molto più elevata rispetto ad altri emulatori, quindi un moltiplicatore sintonizzato altrove va oltre qui.

In 2025.2, il terminale ha anche bug di scorrimento della rotella che producono tasti freccia spuri e eventi di direzione errata. Claude Code rileva questi in fase di esecuzione e li mitiga automaticamente, quindi lo scorrimento del trackpad e della rotella del mouse funzionano senza configurazione. Per la migliore esperienza di scorrimento, eseguire l'aggiornamento a 2025.3 o versione successiva. Claude Code mostra un hint la prima volta che scorri se rileva il bug.

<h2 id="search-and-review-the-conversation">
  Cercare e rivedere la conversazione
</h2>

`Ctrl+o` attiva/disattiva tra la modalità prompt normale e la modalità trascrizione.

Per una visualizzazione più silenziosa che mostra solo l'ultimo prompt, un riepilogo di una riga delle chiamate di strumenti con diffstat di modifica e la risposta finale, eseguire `/focus`. L'impostazione persiste tra le sessioni. Eseguire `/focus` di nuovo per disattivarla.

La modalità trascrizione acquisisce la navigazione e la ricerca in stile `less`:

| Tasto                               | Azione                                                                                                                                 |
| :---------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------- |
| `/`                                 | Apri ricerca. Digita per trovare corrispondenze, `Invio` per accettare, `Esc` per annullare e ripristinare la posizione di scorrimento |
| `n` / `N`                           | Salta alla corrispondenza successiva o precedente. Funziona dopo aver chiuso la barra di ricerca                                       |
| `j` / `k` o `↑` / `↓`               | Scorri una riga                                                                                                                        |
| `g` / `G` o `Home` / `End`          | Salta all'inizio o alla fine                                                                                                           |
| `{` / `}`                           | Salta al prompt precedente o successivo                                                                                                |
| `Ctrl+u` / `Ctrl+d`                 | Scorri mezza pagina                                                                                                                    |
| `Ctrl+b` / `Ctrl+f` o `Space` / `b` | Scorri una pagina intera                                                                                                               |
| `Ctrl+o`, `Esc` o `q`               | Esci dalla modalità trascrizione e ritorna al prompt                                                                                   |

La ricerca `Cmd+f` del terminale e la ricerca tmux non vedono la conversazione perché risiede nel buffer dello schermo alternativo, non nello scrollback nativo. Per restituire il contenuto al terminale, premere `Ctrl+o` per entrare prima in modalità trascrizione, quindi:

* **`[`**: scrive l'intera conversazione nel buffer di scrollback nativo del terminale, con tutto l'output dello strumento espanso. La conversazione è ora testo ordinario nel terminale, quindi `Cmd+f`, la modalità copia tmux e qualsiasi altro strumento nativo possono cercarla o selezionarla. Le sessioni lunghe possono fare una pausa per un momento mentre ciò accade. Questo dura fino a quando non si esce dalla modalità trascrizione con `Esc` o `q`, che ti riporta al rendering a schermo intero. Il prossimo `Ctrl+o` ricomincia da capo.
* **`v`**: scrive la conversazione in un file temporaneo e lo apre in `$VISUAL` o `$EDITOR`.

<h2 id="watch-your-changes-in-the-diff-panel">
  Guarda i tuoi cambiamenti nel pannello diff
</h2>

Nel rendering a schermo intero, [`/diff`](/docs/it/interactive-mode#review-changes-with-%2Fdiff) apre un pannello accanto alla conversazione anziché un visualizzatore che devi chiudere, così puoi guardare i cambiamenti accumularsi mentre Claude lavora. In un terminale ampio il pannello può anche aprirsi automaticamente una volta che Claude inizia a modificare i file. [Pannello diff](/docs/it/interactive-mode#diff-panel) copre ciò che mostra, come mantenerlo chiuso e come cambiare ciò che confronta.

<h2 id="clear-the-conversation">
  Cancellare la conversazione
</h2>

Eseguire `/clear` per avviare una nuova conversazione.

Se la visualizzazione appare distorta o parzialmente vuota, premere `Ctrl+L` per ridisegnare lo schermo. Il ridisegno mantiene la conversazione e il vostro input al loro posto.

`Cmd+K` fa lo stesso di `Ctrl+L` quando il vostro terminale lo passa a Claude Code. iTerm2 e Terminal.app gestiscono `Cmd+K` da soli e cancellano il loro schermo, e Claude Code rileva lo schermo cancellato e ridipinge la conversazione. Prima della v2.1.280, a partire dalla v2.1.260, premendo `Ctrl+L` o `Cmd+K` dove raggiunge Claude Code, cancellava lo schermo nel rendering a schermo intero. Prima della v2.1.238, premendo `Ctrl+L` due volte entro due secondi eseguiva `/clear`.

<h2 id="use-with-tmux">
  Utilizzo con tmux
</h2>

Il rendering a schermo intero funziona all'interno di tmux, con tre avvertenze.

Lo scorrimento con la rotella del mouse richiede la modalità mouse di tmux. Se il vostro `~/.tmux.conf` non l'abilita già, aggiungete questa riga e ricaricate la vostra configurazione:

```bash theme={null}
set -g mouse on
```

Senza la modalità mouse, gli eventi della rotella vanno a tmux invece che a Claude Code. Lo scorrimento da tastiera con `PgUp` e `PgDn` funziona comunque. Claude Code stampa un suggerimento una sola volta all'avvio se rileva tmux con la modalità mouse disabilitata.

Il rendering a schermo intero è incompatibile con la modalità di integrazione tmux di iTerm2, che è la modalità in cui entrate con `tmux -CC`. In modalità integrazione, iTerm2 renderizza ogni riquadro tmux come una divisione nativa piuttosto che lasciare che tmux disegni nel terminale. Il buffer dello schermo alternativo e il tracciamento del mouse non funzionano correttamente lì: la rotella del mouse non fa nulla e il doppio clic può corrompere lo stato del terminale. Non abilitate il rendering a schermo intero nelle sessioni `tmux -CC`. Il tmux regolare all'interno di iTerm2, senza `-CC`, funziona perfettamente.

Le versioni di tmux fino alla serie 3.6 non implementano l'output sincronizzato, quindi con quelle versioni potreste vedere più sfarfallio durante i ridisegni rispetto a quando eseguite Claude Code direttamente nel vostro terminale. Claude Code esamina il terminale per il supporto dell'output sincronizzato all'avvio e lo utilizza quando il terminale lo segnala. Se vedete sfarfallio sotto tmux, aggiornate al tmux più recente o eseguite Claude Code nella sua propria scheda di terminale al di fuori di tmux.

<h2 id="keep-native-text-selection">
  Mantenere la selezione di testo nativa
</h2>

La cattura del mouse è il punto di attrito più comune, soprattutto su SSH o all'interno di tmux. Quando Claude Code cattura gli eventi del mouse, la copia nativa al momento della selezione del vostro terminale smette di funzionare. La selezione che effettuate con click-and-drag esiste all'interno di Claude Code, non nel buffer di selezione del vostro terminale, quindi la modalità copia di tmux, gli hint di Kitty e strumenti simili non la vedono.

Claude Code scrive la selezione negli appunti di sistema, e il percorso che utilizza dipende dalla vostra configurazione. In una sessione locale esegue uno strumento di appunti nativo:

* **macOS**: `pbcopy`
* **Linux**: `wl-copy` su Wayland, oppure `xclip` o `xsel` su X11, a seconda di quale sia installato. Claude Code scrive sia gli appunti che la selezione PRIMARY, quindi la colla con il tasto centrale del mouse funziona.
* **Windows e WSL**: PowerShell `Set-Clipboard`

All'interno di tmux scrive anche nel buffer di incolla di tmux. Su SSH ricade sulle sequenze di escape OSC 52. All'interno di GNU screen, Claude Code copia le selezioni lunghe negli appunti. Prima della v2.1.219, se copiavate una selezione più lunga di circa 570 caratteri, GNU screen stampava testo in base64 nella finestra. Claude Code stampa un toast dopo ogni copia indicandovi quale percorso ha utilizzato.

Alcuni terminali bloccano OSC 52 per impostazione predefinita. iTerm2 lo blocca finché non attivate Impostazioni → Generale → Selezione → Le applicazioni nel terminale possono accedere agli appunti; eseguire [`/terminal-setup`](/docs/it/terminal-config) in iTerm2 lo abilita per voi.

Per una selezione nativa una tantum, il tasto da utilizzare dipende dal vostro terminale:

* **Terminal.app**: `Fn`
* **iTerm2**: `Option`
* **VS Code, Cursor e Devin Desktop**: `Shift`, oppure `Option` su macOS con l'impostazione `terminal.integrated.macOptionClickForcesSelection` abilitata
* **La maggior parte degli altri terminali**: `Shift`

Tenete premuto quel tasto mentre fate click e trascinate. Il vostro terminale gestisce la selezione stesso invece di passarla a Claude Code, quindi le scorciatoie di copia come `Cmd+C` funzionano su quello che selezionate. Claude Code mostra anche il tasto corretto nel suo hint sullo schermo.

Su SSH o all'interno di tmux, Claude Code non sempre riesce a rilevare il terminale da cui vi state connettendo, quindi l'hint elenca i tasti candidati.

Se vi affidate alla selezione nativa tutto il tempo, impostate `CLAUDE_CODE_DISABLE_MOUSE=1` per rinunciare alla cattura del mouse mantenendo il rendering senza sfarfallio e la memoria piatta:

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 CLAUDE_CODE_DISABLE_MOUSE=1 claude
```

Con la cattura del mouse disabilitata, lo scorrimento da tastiera con `PgUp`, `PgDn`, `Ctrl+Home` e `Ctrl+End` funziona ancora, e il vostro terminale gestisce la selezione in modo nativo. Perdete il click-per-posizionare-il-cursore, il click-per-espandere-l'output-dello-strumento, il click-su-URL e lo scorrimento con la rotella all'interno di Claude Code.

Per mantenere lo scorrimento con la rotella ma disattivare la gestione di click, trascinamento e hover, impostate invece `CLAUDE_CODE_DISABLE_MOUSE_CLICKS=1`. Richiede Claude Code v2.1.195 o successivo. `CLAUDE_CODE_DISABLE_MOUSE` ha la precedenza quando entrambe le variabili sono impostate.

Con i click disabilitati, Claude Code cattura ancora il mouse, quindi la rotella e il touchpad scorrono la conversazione ma i click sinistri non fanno nulla all'interno di Claude Code. Dovete comunque tenere premuto il tasto del vostro terminale per la selezione nativa con click-and-drag. Il click destro e la colla con il tasto centrale continuano a funzionare sui terminali che li supportano.

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

Su Windows, Claude Code abilita già il ridisegno completo automaticamente per le sessioni in background e la [visualizzazione agente](/docs/it/agent-view), quindi è necessario impostare la variabile solo per una sessione interattiva a schermo intero che avete avviato direttamente.

<h3 id="fullscreen-renderer-didnt-finish-starting">
  `Claude Code's fullscreen renderer didn't finish starting last time` appare all'avvio
</h3>

Se una sessione a schermo intero su questa macchina si arresta in modo anomalo prima di aver iniziato correttamente, Claude Code avvia la sessione successiva nel renderer classico e stampa una di due righe. Una sessione ha iniziato correttamente una volta che ha disegnato il suo primo fotogramma e poi è rimasta attiva per 10 secondi oppure l'avete terminata con `/exit`, Ctrl+C o Ctrl+D. La riga che vedete vi dice cosa Claude Code fa dopo questa sessione:

* Dopo un avvio non riuscito, vedete `Claude Code's fullscreen renderer didn't finish starting last time on this machine`. Claude Code tenta di nuovo il rendering a schermo intero nella sessione successiva che avviate
* Dopo due avvii non riusciti, vedete `Claude Code's fullscreen renderer has repeatedly failed to start on this machine`. Claude Code continua a utilizzare il renderer classico fino a quando non aggiornate Claude Code o eseguite `/tui fullscreen`, e non stampa nulla in quelle sessioni successive

Per confermare che un avvio non riuscito è il motivo per cui siete nel renderer classico, eseguite `/tui` senza argomenti. Mentre un avvio non riuscito è il motivo, la riga `Current renderer` lo indica.

Per mantenere il renderer classico, eseguite `/tui default`, che salva l'impostazione `tui` senza riavviare. Per provare di nuovo il rendering a schermo intero, eseguite `/tui fullscreen`. Se quella sessione non finisce di avviarsi nemmeno, [segnalate il problema](#research-preview).

Prima della v2.1.236, Claude Code continuava ad avviare sessioni nel rendering a schermo intero dopo un avvio non riuscito.

<h4 id="how-claude-code-counts-failed-starts">
  Come Claude Code conta gli avvii non riusciti
</h4>

* Sessioni che contano: solo le sessioni che hanno iniziato nel rendering a schermo intero perché l'impostazione `tui` lo dice, perché avete accettato la [finestra di dialogo all'avvio](#fullscreen-by-default), o perché Claude Code vi avvia a schermo intero per impostazione predefinita
* `CLAUDE_CODE_NO_FLICKER=1`: se la impostate, Claude Code esegue il rendering di quella sessione a schermo intero anche dopo un avvio non riuscito, e non la conta
* Ripristino del conteggio: Claude Code conta gli avvii non riusciti per versione di Claude Code, e un avvio a schermo intero riuscito ripristina il conteggio
* Finestra di dialogo all'avvio: se avete accettato la finestra di dialogo e la sessione riavviata si è arrestata in modo anomalo, Claude Code non stampa nessuna riga e non mostra di nuovo la finestra di dialogo su questa versione di Claude Code

<h2 id="research-preview">
  Anteprima di ricerca
</h2>

Il rendering a schermo intero è una funzione in anteprima di ricerca. È stato testato su emulatori di terminale comuni, ma potresti riscontrare problemi di rendering su terminali meno comuni o configurazioni insolite.

Se riscontri un problema, esegui `/feedback` all'interno di Claude Code per segnalarlo, oppure apri un problema nel [repository GitHub di claude-code](https://github.com/anthropics/claude-code/issues). Includi il nome e la versione del tuo emulatore di terminale.

Per disattivare il rendering a schermo intero, esegui `/tui default`, oppure annulla l'impostazione di `CLAUDE_CODE_NO_FLICKER` se l'hai abilitata in quel modo. Quando torni indietro con `/tui default`, Claude Code potrebbe prima mostrare un prompt di feedback facoltativo che chiede cosa ti ha fatto cambiare. Digita un motivo e premi `Invio` per inviarlo, oppure premi `Esc` per saltare. Il CLI si riavvia nel renderer classico comunque. Per forzare il renderer classico indipendentemente dall'impostazione `tui` salvata, imposta `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`. Il renderer classico mantiene la conversazione nello scrollback nativo del tuo terminale, quindi `Cmd+f` e la modalità di copia di tmux funzionano come al solito.

Le sessioni in background aperte da [agent view](/docs/it/agent-view) o `claude attach` utilizzano sempre il rendering a schermo intero. Il terminale di collegamento entra nel buffer dello schermo alternativo per mostrare la sessione, e il renderer classico non ha scrollback o gestione del mouse lì, quindi l'impostazione `tui` e `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN` non si applicano a loro.
