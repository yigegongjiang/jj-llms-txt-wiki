> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Gestire più agenti con agent view

> Invia e gestisci molte sessioni di Claude Code da una sola schermata. Agent view mostra cosa sta facendo ogni sessione e quali hanno bisogno del tuo input.

Agent view, aperto con `claude agents`, è una sola schermata per tutte le tue sessioni in background: cosa sta girando, cosa ha bisogno del tuo input e cosa è fatto. Invia nuove sessioni, osserva il loro stato a colpo d'occhio invece di scorrere i transcript, e intervieni solo quando uno ne ha bisogno. Ogni sessione in background è una conversazione completa di Claude Code che continua a girare senza un terminale collegato, quindi puoi aprirla, rispondere e andare via quando vuoi.

<img src="https://mintcdn.com/claude-code/1B48Qz2Z9hac4SLG/images/agent-view-light.png?fit=max&auto=format&n=1B48Qz2Z9hac4SLG&q=85&s=7a186c96ed47d6700d084d77e786be65" className="dark:hidden" alt="Agent view in a terminal: the header shows Claude Code v2.1.140, the model, the working directory, and a summary count. Sessions are grouped under Needs input, Working, and Completed, with a dispatch input at the bottom and a footer of keyboard hints." width="1772" height="780" data-path="images/agent-view-light.png" />

<img src="https://mintcdn.com/claude-code/1B48Qz2Z9hac4SLG/images/agent-view-dark.png?fit=max&auto=format&n=1B48Qz2Z9hac4SLG&q=85&s=a5bed7434bae368faea3a8f023b52aa2" className="hidden dark:block" alt="Agent view in a terminal: the header shows Claude Code v2.1.140, the model, the working directory, and a summary count. Sessions are grouped under Needs input, Working, and Completed, with a dispatch input at the bottom and a footer of keyboard hints." width="1772" height="780" data-path="images/agent-view-dark.png" />

Usa agent view quando hai diversi compiti indipendenti su cui Claude può lavorare senza che tu guardi ogni passo. Invia una correzione di bug, una revisione di pull request e un'investigazione di test instabile come tre righe, continua a lavorare in un'altra finestra e controlla quando una riga mostra che ha bisogno di te o ha un risultato.

Quando vuoi lavorare più direttamente nella sessione di qualsiasi agente, collegati alla riga per entrare nella conversazione completa.

Per confrontare agent view con subagenti, team di agenti e worktrees, vedi [Esegui agenti in parallelo](/docs/it/agents).

<Note>
  Agent view è in anteprima di ricerca e richiede Claude Code v2.1.139 o successivo. Controlla la tua versione con `claude --version`. L'interfaccia e le scorciatoie da tastiera potrebbero cambiare mentre la funzione si evolve.
</Note>

Questa pagina copre:

* [Avvio rapido](#quick-start): dai a Claude un compito su cui lavorare in background, controlla come sta andando e intervieni quando necessario
* [Monitorare le sessioni con agent view](#monitor-sessions-with-agent-view), inclusi icone di stato, peek e reply, collegamento, organizzazione e scorciatoie da tastiera
* [Invia nuovi agenti](#dispatch-new-agents) da agent view, da dentro una sessione, o dalla tua shell
* [Gestire le sessioni dalla shell](#manage-sessions-from-the-shell) con `claude agents`, `claude attach` e comandi correlati
* [Come sono ospitate le sessioni in background](#how-background-sessions-are-hosted) dal processo supervisor

<h2 id="quick-start">
  Avvio rapido
</h2>

Questa procedura illustra il ciclo principale della vista agente: invia un'attività, osserva l'aggiornamento della sua riga mentre Claude lavora, controlla per verificare lo stato e rispondi, e collegati per la conversazione completa. La sessione che invii continua a funzionare dopo che chiudi la vista agente, quindi puoi andare via e tornare ad essa.

<Steps>
  <Step title="Apri agent view">
    Dalla tua shell, esegui:

    ```bash theme={null}
    claude agents
    ```

    Agent view si apre con un input in basso e una tabella che si riempie quando le sessioni iniziano. Premi `Esc` in qualsiasi momento per tornare alla tua shell. Le tue sessioni continuano a funzionare mentre sei via e riappaiono la prossima volta che apri agent view.
  </Step>

  <Step title="Invia una sessione">
    Digita un prompt che descrive un'attività e premi `Enter`. Una nuova sessione in background inizia su quell'attività e appare come una riga che mostra se sta lavorando, aspettando te, o è completata. La nuova sessione utilizza il modello mostrato nell'intestazione di agent view e la stessa [modalità di autorizzazione](#permission-mode-model-and-effort) che otterresti eseguendo `claude` in quella directory.

    Ogni prompt che inserisci qui avvia la sua propria nuova sessione. Digitando un altro prompt e premendo `Enter` avvia una seconda sessione accanto alla prima piuttosto che inviare un follow-up ad essa. Puoi eseguire diversi in parallelo in questo modo.

    Ogni sessione utilizza la tua quota di abbonamento indipendentemente, quindi consulta [Limitazioni](#limitations) prima di inviare molte sessioni contemporaneamente.
  </Step>

  <Step title="Peek e rispondi">
    Seleziona una riga con i tasti freccia e premi `Space` per aprire il pannello peek. Mostra l'output più recente della sessione, o la domanda su cui sta aspettando, piuttosto che la trascrizione completa. Digita una risposta e premi `Enter` per inviarla senza lasciare agent view.
  </Step>

  <Step title="Collegati e scollega">
    Premi `Enter` o `→` su una riga per collegarti quando vuoi la conversazione completa. La sessione prende il controllo del terminale come una sessione Claude Code interattiva completa. Premi `←` su un prompt vuoto per scollegar e tornare alla tabella.
  </Step>

  <Step title="Porta una sessione esistente dentro">
    Per spostare una sessione che hai già aperta in agent view, esegui `/bg` dentro di essa, o premi `←` su un prompt vuoto per metterla in background e aprire agent view in un unico passaggio. La sessione continua a funzionare e appare come una riga accanto a quelle che hai inviato.
  </Step>
</Steps>

Puoi usare `claude agents` come tuo punto di ingresso principale invece di `claude`: invia ogni attività da agent view, collegati quando vuoi la conversazione completa, e premi `←` per tornare alla tabella.

All'interno di una sessione `claude` regolare, l'hint `←` del footer del prompt conta gli agenti in background che stanno aspettando te, come `← 2 agents`, e ritorna a `← for agents` quando nessuno ha bisogno di input. I conteggi superiori a 99 vengono visualizzati come `99+`. Il conteggio si aggiorna circa ogni dieci secondi mentre il terminale è focalizzato e immediatamente quando il focus ritorna. Cambia brevemente colore quando si sposta e quando un agente si completa, a meno che l'impostazione [`prefersReducedMotion`](/docs/it/settings#available-settings) sia attiva, ed è nascosto in [modalità screen reader](/docs/it/accessibility). Su [Amazon Bedrock, Google Cloud's Agent Platform, e Microsoft Foundry](/docs/it/third-party-integrations), l'hint rimane nella sua forma semplice `← for agents` senza il conteggio. Richiede Claude Code v2.1.205 o successivo.

<h2 id="monitor-sessions-with-agent-view">
  Monitorare le sessioni con agent view
</h2>

Esegui `claude agents` per aprire agent view. Prende il controllo del terminale completo ed elenca ogni sessione raggruppata per stato, con sessioni fissate e quelle che hanno bisogno di te in cima. Ogni riga mostra il nome della sessione, l'attività corrente e la sua età, contata da quando la sessione è stata creata; l'età di una sessione terminata si congela a quanto tempo ha impiegato l'esecuzione.

Il nome è tinto con il colore impostato da [`/color`](/docs/it/commands) in quella sessione. A partire da v2.1.199 il colore si trasporta quando [metti in background una sessione](#from-inside-a-session) con `←` o `/background`.

Per impostazione predefinita, l'elenco mostra ogni sessione in background che hai avviato, in tutti i tuoi progetti. Una sessione che lavora in un repository e un'altra in un worktree diverso appaiono entrambe qui, indipendentemente da quale directory hai aperto agent view. Per limitare l'elenco a un progetto, passa `--cwd`:

```bash theme={null}
claude agents --cwd ~/projects/my-app
```

Questo mostra solo le sessioni avviate in quella directory. Una sessione che si è [spostata in un worktree](#how-file-edits-are-isolated) sotto `~/projects/my-app/.claude/worktrees/` conta ancora come appartenente a `~/projects/my-app`.

Le sessioni interattive che hai aperto in altri terminali non appaiono finché non le [metti in background](#from-inside-a-session). I [subagents](/docs/it/sub-agents) e i [teammates](/docs/it/agent-teams) che una sessione genera non sono elencati come righe separate.

```text theme={null}
Pinned
  ✽ clawd walk cycle          Drawing the walk-cycle sprite frames          3m

Ready for review
  ∙ jump physics              Opened PR with collision fix                 #2048  2h

Needs input
  ✻ power-up design           double jump or wall climb?                    1m

Working
  ✽ collision detection       Adding swept-AABB checks to CollisionSystem   2m
  ✢ playtest level 3          run 12 · all checkpoints cleared           in 4m

Completed
  ✻ title screen              result: menu, options, and credits done       9m
  ∙ sound effects             result: 14 SFX exported to assets/audio       4h
  … 6 more
```

<h3 id="read-session-state">
  Leggere lo stato della sessione
</h3>

Ogni riga inizia con un'icona il cui colore e animazione mostrano lo stato della sessione:

| Stato       | L'icona mostra | Cosa significa                                                                |
| :---------- | :------------- | :---------------------------------------------------------------------------- |
| Working     | Animato        | Claude sta attivamente eseguendo strumenti o generando una risposta           |
| Needs input | Giallo         | Claude sta aspettando una domanda specifica o una decisione di permesso da te |
| Idle        | Attenuato      | La sessione non ha nulla da fare ed è pronta per il tuo prossimo prompt       |
| Completed   | Verde          | Il compito è terminato con successo                                           |
| Failed      | Rosso          | Il compito è terminato con un errore                                          |
| Stopped     | Grigio         | La sessione è stata fermata con `Ctrl+X` o `claude stop`                      |

Separatamente, la forma dell'icona mostra se il processo sottostante è in esecuzione:

| Forma             | Cosa significa                                                                                                                                 |
| :---------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| `✻` o animato `✽` | Il processo della sessione è vivo e risponde immediatamente                                                                                    |
| `∙`               | Il processo è uscito. Puoi ancora fare peek, rispondere o collegarti, e Claude riavvia da dove l'ha lasciato                                   |
| `✢`               | Una sessione [`/loop`](/docs/it/scheduled-tasks) che dorme tra le iterazioni. La riga mostra il conteggio delle esecuzioni e un conto alla rovescia |

L'etichetta `#N` che può apparire al bordo destro di una riga è una [pull request che la sessione è collegata a](#pull-request-status), non parte dell'icona di stato.

Il titolo della scheda del terminale mostra il conteggio awaiting-input mentre agent view è aperto: `2 awaiting input · claude agents` quando le sessioni hanno bisogno di input, o `claude agents` quando nessuna lo fa.

A partire da v2.1.198, mentre agent view è aperto, Claude Code invia anche una notifica attraverso il tuo [canale di notifica del terminale](/docs/it/terminal-config#get-a-terminal-bell-or-notification) configurato quando una sessione in background locale inizia ad aver bisogno del tuo input, finisce, o fallisce. Le sessioni che vengono eseguite secondo una pianificazione, come le sessioni [`/loop`](/docs/it/scheduled-tasks), notificano solo quando hanno bisogno del tuo input. Le notifiche utilizzano la stessa impostazione [`preferredNotifChannel`](/docs/it/settings#available-settings) del resto di Claude Code e attivano l'hook [`Notification`](/docs/it/hooks#notification) con il tipo `agent_needs_input` o `agent_completed`.

Le sessioni in background non hanno bisogno di alcun terminale aperto per continuare a lavorare. Un [processo supervisor](#the-supervisor-process) separato le esegue, quindi puoi chiudere agent view, chiudere la tua shell, o avviare una nuova sessione interattiva e il tuo lavoro inviato continua.

Lo stato della sessione persiste su disco attraverso gli auto-update e i riavvii del supervisor. Le sessioni sono anche preservate quando la tua macchina dorme. I loro processi riprendono al risveglio e il supervisor si ricollega a loro invece di trattare il gap di tempo come inattività. Lo spegnimento ferma comunque le sessioni in esecuzione; vedi [Le sessioni mostrano come non riuscite dopo lo spegnimento](#sessions-show-as-failed-after-shutdown) per come recuperarle.

Quando apri una sessione che ha smesso di rispondere, il supervisor riavvia il suo processo e la sessione continua la risposta interrotta da dove l'ha lasciata. Una sessione può finire in quello stato quando la macchina dorme mentre è a metà risposta. Richiede Claude Code v2.1.200 o successivo.

<h3 id="row-summaries">
  Riassunti delle righe
</h3>

Il riassunto su una riga è generato da un [modello di classe Haiku](/docs/it/model-config) in modo che la riga possa dirti cosa sta facendo la sessione, cosa ha bisogno, o cosa ha prodotto senza aprire il transcript. Mentre una sessione sta attivamente lavorando, il testo della riga si aggiorna al massimo una volta ogni 15 secondi dall'output recente della sessione stessa senza inviare una richiesta di modello, e il modello scrive un riassunto fresco quando ogni turno termina.

Una riga di lavoro mostra cosa la sessione dice che sta facendo, e una riga bloccata mostra la domanda che sta ponendo. Durante un turno lungo, il modello riscrive anche il riassunto circa una volta al minuto, aspettando il doppio dopo ogni riscrittura fino a quattro minuti, quindi una riga occupata non continua a mostrare un riassunto obsoleto. Prima di v2.1.205, una riga di lavoro potrebbe mostrare un'invocazione di strumento grezzo invece di un rapporto, e una sessione che esegue elementi di lavoro paralleli mostrava un conteggio `done/total` come `2/5` prima del testo.

Il testo di riepilogo riempie la larghezza rimanente della riga e si tronca solo al bordo destro del terminale; apri il [pannello peek](#peek-and-reply) per leggere una frase che il bordo taglia. Prima di v2.1.206, il testo era tagliato a 64 colonne indipendentemente dalla larghezza del terminale.

Quando l'elenco è [raggruppato per directory](#organize-the-list), il riassunto si apre con lo stato della sessione come una parola colorata, come `Needs input · double jump or wall climb?`. Nel raggruppamento di stato predefinito, l'intestazione del gruppo nomina già lo stato, quindi la riga mostra solo il riassunto. Prima di v2.1.205, le righe raggruppate per directory non portavano alcuna parola di stato.

Un turno il cui intero output non contiene lettere o cifre, come una sessione [`/loop`](/docs/it/scheduled-tasks) che stampa un simbolo solitario su un'iterazione tranquilla, mantiene il riassunto e lo stato precedenti della riga. Prima di v2.1.205, quel turno è stato riclassificato e potrebbe capovolgere una sessione che stava aspettando il tuo input di nuovo a `Working`.

Il riassunto di fine turno e ogni riscrittura a metà turno sono una breve richiesta di classe Haiku attraverso il tuo provider normale, fatturata e gestita secondo gli stessi [termini di utilizzo dei dati](/docs/it/data-usage) della sessione stessa. Gli aggiornamenti di 15 secondi tra le riscritture del modello riutilizzano l'output della sessione stessa e non inviano una richiesta. Su provider di terze parti come Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry e gateway personalizzati, la richiesta ritorna al modello principale della sessione quando nessun modello Haiku è configurato. Imposta [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/it/model-config#environment-variables) per scegliere il modello per questi riassunti su quei provider.

<h3 id="pull-request-status">
  Stato della pull request
</h3>

Quando una sessione apre una pull request, un'etichetta `#1234` appare al bordo destro della riga, collegata alla pull request nei terminali che supportano i hyperlink. L'etichetta persiste quando invii un follow-up alla sessione, quindi la pull request rimane visibile mentre la riga ritorna al progresso live. Le sessioni in background che hanno isolato i loro cambiamenti in un worktree aprono queste pull request da sole; [Come i file edits sono isolati](#how-file-edits-are-isolated) copre quando accade e cosa una sessione non fa mai senza chiedere.

Una sessione che lavora su una pull request esistente è collegata a essa allo stesso modo. Modificare, commentare, chiudere, o contrassegnare una pull request come pronta con `gh` collega la pull request che l'output del comando stesso nomina, quindi un comando `gh` il cui output catturato non nomina alcuna pull request non crea un collegamento; `gh pr merge` è il caso comune, perché stampa il suo risultato solo su un terminale interattivo. Controllare una pull request con `gh pr checkout`, o eseguire il push su un ramo che ha una pull request aperta, la collega cercando quel ramo con `gh pr view` invece. Prima di v2.1.205, solo le pull request che la sessione ha creato o controllato erano collegate, e un push ne collegava una solo quando il nome del ramo locale corrispondeva.

Claude Code legge la pull request dall'output completo del comando, inclusa la parte salvata in un file quando l'output di un comando supera il limite inline. Prima di v2.1.205, una pull request creata in una chiamata Bash il cui output superava circa 30.000 caratteri non era collegata.

Quando una sessione è collegata a più di una pull request, l'etichetta mostra un conteggio invece, come `3 PRs`, colorato dalla pull request aperta che ha più bisogno di attenzione. Apri il [pannello peek](#peek-and-reply) per vederle tutte.

Il numero della pull request è colorato dal suo stato:

| Colore | Stato della pull request                                |
| :----- | :------------------------------------------------------ |
| Giallo | In attesa di controlli o revisione, o controlli falliti |
| Verde  | Controlli superati e nessuna revisione sta bloccando    |
| Viola  | Unito                                                   |
| Grigio | Bozza o chiuso                                          |

Per la maggior parte dei compiti questa colonna è dove raccogli il risultato: rivedi e unisci la pull request quando il suo numero diventa verde.

<h3 id="peek-and-reply">
  Peek e rispondi
</h3>

Premi `Space` su una riga selezionata per aprire il pannello peek. Si apre con la frase che la riga tronca al bordo del terminale, e quale frase sia dipende dallo stato della sessione:

* Una sessione che sta aspettando te: la domanda esatta che sta ponendo, sopra l'input di risposta
* Una sessione terminata: il suo risultato
* Una sessione che lavora: la sua frase di stato completa

Qualsiasi pull request collegata alla sessione è elencata dopo. Per una sessione che sta aspettando te, una riga come `waiting 3m` sotto di loro mostra quanto tempo è stata in attesa, ed è l'unico momento mostrato nel pannello. L'età al bordo destro della riga è un numero diverso: conta da quando la sessione è iniziata.

La maggior parte delle volte il pannello peek è sufficiente e non hai bisogno di aprire il transcript completo.

Prima di v2.1.207, ogni peek si apriva con la frase di stato e un timestamp nudo, e una sessione bloccata aveva la sua domanda che appariva sotto di loro con lo stesso timestamp ripetuto una seconda volta.

Digita una risposta nel pannello peek e premi `Enter` per inviarla a quella sessione. Quando la sessione sta facendo una domanda a scelta multipla, il pannello peek mostra le opzioni e puoi premere un tasto numero per sceglierne uno. Per altre sessioni bloccate, premi `Tab` per riempire l'input con una risposta suggerita che puoi modificare prima di inviare. Prefissa una risposta con `!` per inviare un comando Bash invece.

Una risposta che non può essere consegnata, perché il servizio in background non è raggiungibile o l'invio fallisce, viene salvata e inviata alla sessione come suo prossimo prompt quando il suo processo si avvia di nuovo, e il messaggio di errore dice che la risposta è stata salvata. Una risposta con prefisso `!` non viene salvata, perché il testo salvato raggiungerebbe la sessione come un prompt semplice piuttosto che eseguire come comando Bash.

Con la [dettatura vocale](/docs/it/voice-dictation) abilitata, tieni premuto o tocca il tuo tasto push-to-talk mentre l'input di risposta è focalizzato per dettare una risposta invece di digitarla. Lo stesso funziona nell'input di dispatch in fondo a agent view.

Usa `↑` e `↓` per fare peek alle sessioni adiacenti senza chiudere il pannello, o `→` per collegarti.

<h3 id="attach-to-a-session">
  Collegarsi a una sessione
</h3>

Premi `Enter` o `→` su una riga selezionata per collegarti. Agent view è sostituito dalla sessione interattiva completa. Quando ti colleghi, Claude pubblica un breve riassunto di cosa è successo mentre eri via.

Mentre collegato, la sessione si comporta come qualsiasi altra sessione di Claude Code: [comandi](/docs/it/commands), scorciatoie da tastiera e funzioni funzionano tutti, con le eccezioni sottostanti.

Una sessione in background rifiuta `/install-github-app` e l'elenco delle impostazioni [`/mcp`](/docs/it/mcp), incluse le sue azioni di autenticazione, sia che tu sia collegato o che rispondi dal pannello peek. Il messaggio ti indirizza a una sessione `claude` regolare, e `/mcp reconnect <server>`, `/mcp enable`, e `/mcp disable` funzionano ancora.

Le sessioni collegate si rendono sempre in [modalità fullscreen](/docs/it/fullscreen), indipendentemente dalla tua impostazione `tui`, perché una sessione in background non ha scrollback del terminale a cui aggiungere. Scorri con `PgUp`, `PgDn`, o la rotella del mouse, e premi `Ctrl+O` per la modalità transcript. Lo scroll nativo del tuo terminale e la modalità copia di tmux mostrano solo il viewport corrente, lo stesso di quando esegui qualsiasi applicazione fullscreen.

Premi `←` su un prompt vuoto, o esegui `/exit`, per scollegar e tornare a agent view. A partire da v2.1.198 questo funziona allo stesso modo sia che tu abbia aperto la sessione da agent view o con `claude attach <id>` dalla tua shell.

`Ctrl+Z` scollega anche ma torna a dove hai iniziato invece: agent view se ti sei collegato da lì, o la tua shell se hai eseguito `claude attach`. Usa `Ctrl+Z` quando una finestra di dialogo ha il focus e non risponde a `←`.

`Ctrl+C` mantiene il suo comportamento di interruzione standard mentre collegato: annulla una risposta in esecuzione o un comando shell `!` piuttosto che scollegar. Premere `Ctrl+C` due volte su un prompt vuoto scollega, come in qualsiasi sessione.

Scollegar non ferma mai una sessione in background: `←`, `Ctrl+Z`, `/exit`, e doppio `Ctrl+C` o doppio `Ctrl+D` la lasciano tutte in esecuzione. Per terminare una sessione da dentro di essa, esegui `/stop`.

In una sessione in esecuzione in primo piano, una che hai avviato nel terminale piuttosto che collegato da agent view, premere `←` su un prompt vuoto la mette in background e apre agent view con quella riga selezionata, quindi puoi cambiare sessioni senza lasciare il terminale. Lo stesso singolo press scollega una sessione collegata.

Se uno strumento è in esecuzione quando premi `←`, Claude Code aspetta fino a circa dieci secondi che finisca prima di metterla in background, e la risposta continua nella sessione in background. Premi `←` di nuovo per metterla in background immediatamente invece di aspettare. Quando il lavoro in corso non può trasferirsi alla sessione in background, appare prima la finestra di dialogo `Background this session?`, come con [`/background`](#from-inside-a-session).

Il limite di dieci secondi non si applica mentre i [subagents](/docs/it/sub-agents) sono in esecuzione. Claude Code continua ad aspettare in modo che il loro lavoro si trasferisca, e mostra un avviso `Still backgrounding after the current tool` mentre aspetta; premi `←` di nuovo per metterla in background senza aspettare, il che riavvia i subagents dall'inizio. Prima di v2.1.203, l'attesa è terminata dopo dieci secondi e i subagents in esecuzione sono stati riavviati dall'inizio senza avvertimento.

La riga viene creata anche da una sessione nuova senza cronologia di conversazione, quindi `→` vi ritorna. Prima di v2.1.203, agent view mostrava un suggerimento di onboarding sotto quella riga quando era l'unica.

Puoi disattivare questa scorciatoia con l'impostazione `leftArrowOpensAgents` in `/config`.

<h3 id="organize-the-list">
  Organizzare l'elenco
</h3>

Agent view raggruppa le sessioni in modo che quelle che hanno bisogno di input siano in cima, con `Ready for review` e `Needs input` sopra `Working` e `Completed`. Questi nomi di gruppo non corrispondono uno a uno agli [stati](#read-session-state) sopra: una sessione si sposta a `Ready for review` quando ha una pull request aperta, e `Completed` raccoglie insieme sessioni finite, fallite e fermate.

Premi `Ctrl+S` per raggruppare per directory invece. La tua scelta persiste tra le esecuzioni.

All'interno di un gruppo:

* Premi `Ctrl+T` per fissare una sessione in cima e [mantenere il suo processo in esecuzione](#the-supervisor-process) mentre inattivo
* Premi `Shift+↑` o `Shift+↓` per riordinare le sessioni
* Premi `Ctrl+R` per rinominare una sessione
* Premi `Enter` su un'intestazione di gruppo per comprimerla

Per rimuovere una sessione dall'elenco, premi `Ctrl+X` per fermarla e `Ctrl+X` di nuovo entro due secondi per eliminarla. Premere `Ctrl+X` su un'intestazione di gruppo elimina ogni sessione in quel gruppo dopo conferma.

L'eliminazione rimuove la sessione da agent view. Se Claude ha [creato un worktree](#how-file-edits-are-isolated) per la sessione, l'eliminazione rimuove anche quel worktree, inclusi eventuali cambiamenti non committati in esso, quindi esegui il commit del lavoro che vuoi mantenere prima. Un worktree che hai creato tu stesso e in cui hai avviato la sessione viene lasciato in posto. Il transcript della conversazione rimane sulla tua macchina locale e rimane disponibile attraverso `claude --resume`.

L'eliminazione non rimuove mai un worktree con commit che non sono stati spinti da nessuna parte, o uno che un'altra sessione in esecuzione rivendica o ha bloccato. Claude Code mantiene il worktree e la sessione, e il footer nomina il percorso mantenuto e il motivo. Spingi i commit, o chiudi l'altra sessione, quindi elimina di nuovo.

L'eliminazione cancella anche la sessione dall'[elenco delle sessioni del supervisor](#the-supervisor-process), sia che tu elimini con `Ctrl+X` o con [`claude rm`](#manage-sessions-from-the-shell) dalla shell, quindi la rimozione persiste attraverso i riavvii del supervisor. Prima di v2.1.206, rimuovere una sessione mentre il supervisor stava riavviando o era irraggiungibile la lasciava in quell'elenco, e il prossimo supervisor riavviava il suo processo e mostrava la riga di nuovo.

Le sessioni completate che non si adattano allo schermo si ripiegano in una riga `… N more`. I fallimenti e le sessioni con una pull request aperta rimangono sempre visibili. Il gruppo `Completed` riempie lo spazio verticale rimasto dopo i gruppi live, e su un terminale corto l'intestazione si compatta in una singola riga di riepilogo in modo che le sessioni che stanno lavorando o hanno bisogno di input rimangono visibili.

<h3 id="filter-sessions">
  Filtrare le sessioni
</h3>

Digita nell'input di dispatch per filtrare invece di inviare:

| Filtro                  | Mostra                                                                                                     |
| :---------------------- | :--------------------------------------------------------------------------------------------------------- |
| `a:<name>`              | Sessioni che eseguono l'agente denominato                                                                  |
| `s:<state>`             | Sessioni nello stato dato, come `s:working`. Accetta anche `s:blocked` per tutto ciò che sta aspettando te |
| `#<number>` o un URL PR | La sessione che lavora su quella pull request                                                              |
| Qualsiasi altro URL     | La sessione il cui primo prompt conteneva quell'URL                                                        |

<h3 id="keyboard-shortcuts">
  Scorciatoie da tastiera
</h3>

Premi `?` in agent view per vedere ogni scorciatoia nel contesto. La tabella sottostante le riassume.

| Scorciatoia           | Azione                                                                                       |
| :-------------------- | :------------------------------------------------------------------------------------------- |
| `↑` / `↓`             | Muoviti tra le righe                                                                         |
| `Enter`               | Collegati alla sessione selezionata, o invia se c'è testo nell'input                         |
| `Space`               | Apri o chiudi il pannello peek per la sessione selezionata                                   |
| `Shift+Enter`         | Invia e collegati immediatamente                                                             |
| `→`                   | Collegati alla sessione selezionata                                                          |
| `Alt+1`..`Alt+9`      | Collegati alla sessione 1–9 nella directory della sessione focalizzata                       |
| `Tab`                 | Su un input vuoto, sfoglia tutti i subagents. Altrimenti applica il suggerimento evidenziato |
| `Ctrl+S`              | Cambia raggruppamento tra stato e directory                                                  |
| `Ctrl+T`              | Fissa o scollega la sessione selezionata                                                     |
| `Ctrl+R`              | Rinomina la sessione selezionata                                                             |
| `Ctrl+G`              | Apri il prompt di dispatch nel tuo `$VISUAL` o `$EDITOR`                                     |
| `Ctrl+X`              | Ferma la sessione; premi di nuovo entro due secondi per eliminarla                           |
| `Shift+↑` / `Shift+↓` | Riordina la sessione selezionata                                                             |
| `Esc`                 | Chiudi il pannello peek, cancella l'input, o esci                                            |
| `Ctrl+C`              | Cancella l'input; premi due volte per uscire                                                 |
| `?`                   | Mostra tutte le scorciatoie                                                                  |

<h2 id="dispatch-new-agents">
  Invia nuovi agenti
</h2>

Puoi inviare nuove sessioni in background da agent view, inviare una sessione interattiva esistente in background, o avviarne una direttamente dalla shell.

<h3 id="from-agent-view">
  Da agent view
</h3>

Digita un prompt nell'input in basso di agent view e premi `Enter` per avviare una nuova sessione in background. La sessione è denominata automaticamente dal prompt; rinominala in seguito con `Ctrl+R`.

Un nome che la sessione riceve in seguito appare anche sulla sua riga, incluso il nome che Claude deriva quando [accetti un piano](/docs/it/permission-modes#review-and-approve-a-plan) in quella sessione. Prima di v2.1.207, una sessione in background denominata accettando un piano mostrava quel nome in `/status` ma non sulla sua riga di agent view finché non la rinominavi tu stesso.

Incolla un'immagine nel prompt per includere uno screenshot o un diagramma con il compito.

Il testo incollato più lungo di 800 caratteri o più di due righe si comprime in un placeholder `[Pasted text #N]` in modo che l'input rimanga su una riga; il testo completo viene inviato quando invii. Per rivedere o modificare il testo compresso prima di inviare, incolla lo stesso testo di nuovo e il placeholder si espande di nuovo nell'input. Un promemoria `paste again to expand` appare sotto l'input per alcuni secondi dopo l'incolla su terminali larghi almeno 90 colonne. Prima di v2.1.207, incollare lo stesso testo di nuovo aggiungeva un secondo placeholder invece di espandere il primo.

Prefissa o menziona parti del prompt per controllare come la sessione inizia:

| Input                                | Effetto                                                                                                                                                                                             |
| :----------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `<agent-name> <prompt>`              | Se la prima parola corrisponde a un nome di [subagent](/docs/it/sub-agents) personalizzato, quel subagent viene eseguito come agente principale della sessione con la configurazione dal suo frontmatter |
| `@<agent-name>`                      | Menziona un subagent personalizzato in qualsiasi punto del prompt per eseguirlo come agente principale                                                                                              |
| `@<repo>`                            | Menziona un repository per eseguire la sessione lì. Vedi [Invia a una directory specifica](#dispatch-to-a-specific-directory) per quali repository sono elencati                                    |
| `/<command>`                         | Suggerisci [skills](/docs/it/skills) e [commands](/docs/it/commands) da inviare come prompt                                                                                                                   |
| `! <command>`                        | Esegui un comando shell come un job in background invece di avviare una sessione Claude. Il job appare come una riga a cui puoi collegarti, osservare e staccarti                                   |
| `#<number>` o un URL di pull request | Se una sessione sta già lavorando su quel PR, selezionala invece di inviare                                                                                                                         |
| `Shift+Enter`                        | Invia e collegati immediatamente alla nuova sessione                                                                                                                                                |

Un piccolo insieme di comandi viene eseguito in agent view stesso invece di essere inviato:

* `/exit` e `/quit` chiudono agent view
* `/logout` ti disconnette
* `/model` imposta il [modello di dispatch](#set-the-model)
* A partire da v2.1.198, `/login` apre la finestra di dialogo di accesso in modo che tu possa accedere di nuovo senza collegarti a una sessione

Skills, i tuoi comandi personali, e built-in che espandono il prompt come `/init` vengono inviati a una nuova sessione in background come loro primo prompt. Gli altri comandi built-in mostrano un suggerimento `attach to a session to run it` invece. Tutto ciò che hai digitato rimane nell'input accanto al suggerimento in modo che tu possa modificarlo. Prima di v2.1.203, il suggerimento cancellava l'input e il testo digitato andava perso.

Confezionare un compito ricorrente come una [skill](/docs/it/skills) ti permette di avviare lo stesso workflow da agent view ripetutamente senza riscrivere il prompt.

Quando lo stesso `@name` corrisponde sia a un subagent che a un repository fratello, il subagent ha la precedenza. La corrispondenza della prima parola senza `@` si applica anche, quindi un prompt che inizia con uno dei tuoi nomi di subagent invia quel subagent piuttosto che trattare la parola come testo semplice. Usa la forma `@` quando vuoi essere esplicito, o inizia il prompt con una parola diversa per evitare la corrispondenza.

<h4 id="dispatch-to-a-specific-directory">
  Invia a una directory specifica
</h4>

Una nuova sessione viene eseguita nella directory da cui hai aperto agent view. Per indirizzare una directory diversa, utilizza uno qualsiasi di questi:

* Apri `claude agents` in quella directory.
* Apri `claude agents` in una directory padre e menziona un repository figlio con `@<repo>` nel prompt. Digitando `@` vengono elencati questi target:

  * Repository Git un livello sotto la directory di lancio
  * I [git worktrees](/docs/it/worktrees) registrati del repository da cui hai lanciato che si trovano all'interno del suo albero di directory, come quelli che Claude crea sotto `.claude/worktrees/`, etichettati con il loro ramo estratto. I worktrees aggiunti al di fuori del repository, come con `git worktree add ../feature`, non sono elencati
  * Qualsiasi directory che ha già una sessione nell'elenco

  Una directory il cui nome contiene uno spazio non è elencata. Prima di v2.1.203, i worktrees registrati non erano elencati, quindi l'invio in uno significava eseguire `claude --bg` dalla directory di quel worktree.
* Dalla shell, `cd` nella directory e esegui `claude --bg "<prompt>"`.

Quando agent view è raggruppato per directory, la directory della riga evidenziata diventa il target di dispatch, quindi puoi scorrere a un gruppo e inviare in esso senza riscrivere il percorso.

<h3 id="from-inside-a-session">
  Da dentro una sessione
</h3>

Esegui `/background` o il suo alias `/bg` per spostare la conversazione corrente in una sessione in background. Passa un prompt come `/bg run the test suite and fix any failures` per inviare un'istruzione in più prima. Se Claude sta rispondendo quando esegui `/bg`, la risposta continua nella sessione in background.

L'uscita da una sessione interattiva che ha ancora lavoro in background in esecuzione, come subagent, comandi shell in background, workflow, o [monitor](/docs/it/tools-reference#monitor-tool), mostra una finestra di dialogo `Background work is running` invece di uscire immediatamente. A partire da v2.1.198 la finestra di dialogo offre `Move to background and exit` insieme a `Exit anyway` e `Stay`. Sceglierla sposta la sessione in background nello stesso modo in cui `/background` fa, quindi ti restituisce alla tua shell, in modo che il lavoro che può continuare rimane in esecuzione e la sessione appare in agent view. L'opzione non viene mostrata quando agent view è [disattivato](#turn-off-agent-view).

Lo spostamento in background da una sessione interattiva avvia un nuovo processo che riprende dalla conversazione salvata, e il lavoro in corso si trasferisce ad esso: comandi shell in background in esecuzione, subagent in background, workflow dinamici, e compiti programmati che hai creato con [`/loop`](/docs/it/scheduled-tasks) si trasferiscono alla sessione in background e continuano a girare lì. Un subagent si sposta insieme a tutto ciò che ha avviato, quindi si trasferisce solo quando tutto quel lavoro può trasferirsi anche, incluso su Windows. Per fermare il lavoro in corso invece di trasferirlo, imposta la variabile di ambiente [`CLAUDE_DISABLE_ADOPT=1`](/docs/it/env-vars#variables); Claude Code ti chiede di confermare prima di metterla in background.

Il lavoro che non può trasferirsi, come un [monitor](/docs/it/tools-reference#monitor-tool) in esecuzione, viene fermato. Un subagent messo in background che possiede un monitor viene fermato insieme ad esso. Quando uno qualsiasi di essi è in esecuzione, Claude Code mostra una finestra di dialogo `Background this session?` in modo che tu possa confermare prima che sia fermato.

Una volta in background, la sessione può avviare nuovi subagent, monitor, e comandi in background, e questi continuano a essere eseguiti durante successivi distacchi e ricollegamenti.

I flag di configurazione dal lancio originale si trasferiscono alla sessione messa in background, quindi i suoi server MCP, le impostazioni e il modello di fallback rimangono in vigore:

* `--mcp-config` e `--strict-mcp-config`
* `--settings`
* `--add-dir`
* `--plugin-dir`
* `--fallback-model`
* `--allow-dangerously-skip-permissions`

Le directory che hai aggiunto durante la sessione con [`/add-dir`](/docs/it/permissions#additional-directories-grant-file-access-not-configuration) si trasferiscono anche.

Il trasferimento di `--allow-dangerously-skip-permissions` mantiene `bypassPermissions` raggiungibile nella sessione messa in background, ma non concede nulla di nuovo. La modalità richiede comunque la stessa accettazione interattiva una tantum descritta in [Permission mode, model, and effort](#permission-mode-model-and-effort) prima che qualsiasi sessione possa utilizzarla.

<h3 id="from-your-shell">
  Dalla tua shell
</h3>

Passa `--bg` o la sua forma lunga `--background` per avviare una sessione che va direttamente in background:

```bash theme={null}
claude --bg "investigate the flaky SettingsChangeDetector test"
```

Il prompt è l'argomento posizionale, non un valore `-p`. A partire da v2.1.198, combinare `--bg` con `-p` o `--print` viene rifiutato con un errore prima che qualsiasi sessione sia creata, perché `--print` non avvia mai la sessione interattiva a cui `claude agents` si collega.

Per eseguire un subagent specifico come agente principale della sessione, combina `--bg` con `--agent`:

```bash theme={null}
claude --agent code-reviewer --bg "address review comments on PR 1234"
```

Passa `--name` per impostare il nome di visualizzazione della sessione in agent view invece di quello generato automaticamente:

```bash theme={null}
claude --bg --name "flaky-test-fix" "investigate the flaky SettingsChangeDetector test"
```

Dopo aver messo in background, Claude stampa l'ID breve della sessione e i comandi per gestirla. Quando il servizio che ospita le sessioni in background non è già in esecuzione, `--bg` potrebbe prima stampare `Starting background service…` sopra questo output. Quando passi `--name`, il nome appare dopo l'ID breve:

```text theme={null}
backgrounded · 7c5dcf5d · flaky-test-fix
  claude agents             list sessions
  claude attach 7c5dcf5d    open in this terminal
  claude logs 7c5dcf5d      show recent output
  claude stop 7c5dcf5d      stop this session
```

<h4 id="run-a-shell-command">
  Esegui un comando shell
</h4>

Per eseguire un comando shell come un job in background invece di una sessione Claude, digita `!` come primo carattere dell'input di dispatch di agent view. Il `!` viene visualizzato come prefisso e tutto ciò che digiti dopo di esso è il comando. L'esempio seguente invia `pytest -x` dalla casella di input di agent view:

```text theme={null}
! pytest -x
```

Premi `Enter` per avviare il job. Lo stesso job può anche essere lanciato direttamente dalla tua shell con `--exec`:

```bash theme={null}
claude --bg --exec 'pytest -x'
```

Il comando viene eseguito come un job supportato da PTY e appare come una riga in agent view, con la riga di output più recente come suo stato. Un job shell esegue il comando al posto di Claude, quindi nessun modello viene invocato e l'output non viene inviato a nessuna sessione.

Per vedere l'output, collegati alla riga, premi `Space` per visualizzare l'anteprima senza collegarti, o esegui `claude logs <id>` dalla tua shell. L'output acquisito rimane in memoria e non viene scritto su disco. La riga e il suo output si puliscono automaticamente circa cinque minuti dopo l'uscita del comando, quindi leggi prima se hai bisogno del risultato.

<h3 id="how-file-edits-are-isolated">
  Come sono isolate le modifiche ai file
</h3>

Ogni sessione in background, che sia avviata da agent view, `/bg`, o `claude --bg`, inizia nella tua directory di lavoro. Prima di modificare i file, Claude sposta la sessione in un [git worktree](/docs/it/worktrees) isolato sotto `.claude/worktrees/`, quindi le sessioni parallele possono leggere lo stesso checkout ma ognuna scrive nel suo.

Claude salta il worktree quando:

* La sessione è già all'interno di un git worktree collegato, che Claude l'abbia creato sotto `.claude/worktrees/` o tu l'abbia creato con `git worktree add` da qualche altra parte
* La directory di lavoro non è un repository git e nessun [`WorktreeCreate` hook](/docs/it/hooks#worktreecreate) è configurato
* La scrittura è al di fuori della directory di lavoro

Per disattivare l'isolamento del worktree per un repository dove i git worktree sono impraticabili, imposta [`worktree.bgIsolation`](/docs/it/settings#worktree-settings) su `"none"`. Le sessioni in background modificheranno quindi la tua copia di lavoro direttamente senza spostarsi prima in un worktree. Aggiungi l'impostazione al file `.claude/settings.json` del progetto:

```json theme={null}
{
  "worktree": {
    "bgIsolation": "none"
  }
}
```

Al di fuori di un repository git, le sessioni scrivono direttamente nella directory di lavoro e non sono isolate l'una dall'altra, quindi evita di inviare sessioni parallele che modificano gli stessi file. Se utilizzi un sistema di controllo versione diverso, configura un [`WorktreeCreate` hook](/docs/it/worktrees#non-git-version-control) e Claude isola le modifiche nello stesso modo in cui lo fa per git.

Quando l'hook fallisce in una directory che non è un repository git, la sessione salta l'isolamento per quella directory e modifica la directory di lavoro in posizione. All'interno di un repository git, le scritture rimangono bloccate finché la sessione non si isola. Prima di v2.1.203, una sessione in background in quello stato non poteva modificare alcun file: ogni scrittura era rifiutata finché non si isolava, e l'hook non poteva mai isolare quella directory.

L'eliminazione di una sessione rimuove o mantiene il worktree che Claude ha creato per essa, a seconda di come lo elimini e di cosa contiene il worktree:

* L'eliminazione in agent view con `Ctrl+X` due volte rimuove il worktree, inclusi eventuali cambiamenti non committati, quindi unisci o spingi i cambiamenti che vuoi mantenere prima.
* L'eliminazione dalla shell con [`claude rm`](#manage-sessions-from-the-shell) mantiene un worktree che ha cambiamenti non committati, insieme alla sua riga di sessione.
* Nessuno dei due percorsi rimuove un worktree con commit che non sono stati spinti da nessuna parte: il worktree è [mantenuto insieme alla sua sessione](#organize-the-list) e l'output nomina il percorso mantenuto e il motivo.
* Un worktree che hai creato tu stesso e in cui hai avviato la sessione viene lasciato in posizione in entrambi i casi.

Per trovare il percorso del worktree di una sessione, visualizza l'anteprima della sessione o collegati e controlla la sua directory di lavoro.

Un [subagent](/docs/it/sub-agents) che la sessione in background genera eredita la directory di lavoro della sessione, quindi le sue modifiche ai file finiscono nel worktree della sessione piuttosto che nella tua copia di lavoro. Per dare a un subagent il suo proprio worktree separato, imposta [`isolation: worktree`](/docs/it/sub-agents#supported-frontmatter-fields) nel suo frontmatter o passa `isolation: "worktree"` quando lo generi.

A partire da v2.1.198, una sessione in background che ha isolato i suoi cambiamenti di codice in un worktree esegue anche il commit, spinge il suo ramo, e apre una bozza di pull request senza fermarsi a chiedere. L'etichetta [`#N`](#pull-request-status) appare sulla sua riga quando il pull request si apre. Non spinge mai a `main` o `master`, non fa mai force-push o merge, e salta il pull request quando gli hai detto di non aprirne uno o il repository non ha un remote.

Una sessione che modifica un checkout che non ha isolato da sola chiede comunque prima di fare il commit o di cambiare rami. Questo si applica quando l'isolamento è impostato su `"none"`, quando lo spostamento del worktree è fallito, o quando la sessione è stata avviata all'interno di un worktree che già esisteva.

<h3 id="set-the-model">
  Imposta il modello
</h3>

Il nome del modello mostrato nell'intestazione di agent view è il default di dispatch. Le nuove sessioni che avvii dall'input utilizzano questo modello, che proviene dall'impostazione [`model`](/docs/it/settings#available-settings) nei tuoi settings utente. Impostalo selezionando un modello nel picker [`/model`](/docs/it/model-config), o modifica l'impostazione direttamente.

Per sovrascrivere il default di dispatch per l'intera sessione di agent view, passa `--model` quando apri agent view. Vedi [Permission mode, model, and effort](#permission-mode-model-and-effort).

Per cambiare il default di dispatch da dentro agent view, digita `/model` seguito da un nome di modello nell'input di dispatch e premi `Enter`. L'intestazione si aggiorna per mostrare quel modello con un marcatore `(session)`, e le sessioni che invii successivamente lo utilizzano. Digita `/model default` per cancellare l'override e tornare al default di dispatch. Questo override dura per il resto della corrente esecuzione di `claude agents` e non scrive nel tuo file di settings. L'esempio seguente invia una sessione su Opus e la successiva su Sonnet:

```text theme={null}
/model opus
refactor auth
/model sonnet
run the test suite
```

Ogni sessione in background può essere eseguita su un modello diverso. Per sovrascriverlo per una sessione:

* Dalla shell, passa `--model` con `claude --bg`.
* Collegati a una sessione in esecuzione e esegui `/model` per passare: una scelta dal picker, o un `/model <name>` digitato, viene salvato come tuo default per le nuove sessioni a meno che tu non premi `s` nel picker per un passaggio solo per la sessione. Un passaggio solo per la sessione persiste se la sessione viene riavviata.
* Invia un [subagent](/docs/it/sub-agents) il cui frontmatter imposta un campo `model`.

<h3 id="permission-mode-model-and-effort">
  Permission mode, model, and effort
</h3>

Una sessione in background legge i suoi [settings](/docs/it/settings) dalla directory in cui viene eseguita, come se avessi avviato `claude` lì. Questo include i valori [`env`](/docs/it/settings#available-settings) nei settings del progetto, quindi un `ANTHROPIC_MODEL` o una variabile del provider impostata lì si applica alle sessioni in background in quella directory.

La selezione del cloud provider, come `CLAUDE_CODE_USE_BEDROCK` o `CLAUDE_CODE_USE_VERTEX`, e gli alias `ANTHROPIC_DEFAULT_*_MODEL` seguono la shell che ha inviato la sessione. Se esporti un override del corpo della richiesta [`CLAUDE_CODE_EXTRA_BODY`](/docs/it/env-vars) in quella shell, raggiunge la sessione nello stesso modo. Prima di v2.1.206, i worker in background ignoravano un `CLAUDE_CODE_EXTRA_BODY` esportato dalla shell.

Se esporti un gateway `ANTHROPIC_BASE_URL` nella shell di dispatch, raggiunge la sessione anche, insieme a `ANTHROPIC_CUSTOM_HEADERS`, quando il supervisore viene eseguito con lo stesso ambiente gateway e la sessione viene eseguita nella directory da cui hai inviato o è la tua sessione messa in background con `←` o `/background`. Questo è il caso normale quando la prima shell ad aprire agent view o inviare una sessione in background è la shell gateway. L'invio in una directory diversa con `@repo` o `--cwd` non trasporta il gateway della shell; le [impostazioni](/docs/it/settings) di quel progetto forniscono l'endpoint. Vedi [il processo supervisore](#the-supervisor-process) per come le sessioni in background ottengono le impostazioni del provider e le credenziali.

Il [permission mode](/docs/it/permissions) dipende da come hai avviato la sessione. Lo spostamento in background di una sessione esistente con `/bg` o `←` mantiene il permission mode corrente, quindi una sessione che hai cambiato in `acceptEdits` o `auto` rimane in quella modalità dopo il distacco. L'invio da agent view input o l'esecuzione di `claude --bg` dalla tua shell utilizza il `defaultMode` dai settings di quella directory, o il `permissionMode` dal frontmatter del [subagent inviato](/docs/it/sub-agents#supported-frontmatter-fields).

Il permission mode, il modello e lo sforzo con cui una sessione in background è stata avviata, insieme ai [flag di configurazione che trasporta](#from-inside-a-session), persistono tutti quando il supervisore successivamente [arresta e riavvia](#the-supervisor-process) il processo della sessione. Una sessione che hai lanciato con `claude --bg --dangerously-skip-permissions` o `claude --bg --permission-mode bypassPermissions` rimane in `bypassPermissions` dopo quel riavvio invece di tornare al `defaultMode` della directory, e un modello o sforzo che hai cambiato a metà sessione con `/model` o `/effort` viene mantenuto.

Uno sforzo che la sessione ha preso dall'impostazione [`effortLevel`](/docs/it/settings#available-settings) piuttosto che da `--effort` o `/effort` non è fissato al dispatch: ogni processo avviato per la sessione legge di nuovo l'impostazione, quindi modificare `effortLevel` in `settings.json` raggiunge le sessioni che metti in background con `←` o `/bg` e i loro successivi riavvii. Prima di v2.1.203, mettere in background una sessione registrava il suo sforzo derivato dalle impostazioni come se avessi passato `--effort`, quindi i successivi edit di `effortLevel` non lo raggiungevano mai.

Un nome che hai impostato con [`/rename`](/docs/it/commands) o `Ctrl+R` persiste anche attraverso quel riavvio, quindi [`claude --resume <name>`](/docs/it/sessions#name-your-sessions) risolve ancora la sessione. Prima di v2.1.202, il riavvio ripristinava la sessione al nome con cui era stata inviata e il nuovo nome smetteva di risolvere.

Per impostare i default per ogni sessione che invii da agent view, passa uno qualsiasi di `--permission-mode`, `--model`, `--effort`, o `--agent` quando lo apri:

```bash theme={null}
claude agents --permission-mode plan --model opus --effort high
```

`--agent` imposta il [subagent](/docs/it/sub-agents) utilizzato quando un prompt di dispatch non ne nomina uno, sia con `@name` che come prima parola. Per impostazione predefinita, utilizza l'impostazione [`agent`](/docs/it/settings#available-settings) se ne è impostata una, altrimenti l'agente catch-all integrato `claude`. Nominare un subagent nell'input di dispatch sovrascrive entrambi.

`claude agents` accetta anche `--dangerously-skip-permissions` come abbreviazione per `--permission-mode bypassPermissions`, e `--allow-dangerously-skip-permissions` per rendere `bypassPermissions` disponibile nel ciclo `Shift+Tab` di ogni sessione inviata senza avviare in quella modalità. Entrambi corrispondono ai [flag CLI di livello superiore](/docs/it/cli-reference).

I default attivi appaiono nel footer sotto l'input di dispatch.

Senza questi flag, la sessione utilizza il `defaultMode` dai settings di quella directory o il `permissionMode` dal frontmatter del [subagent inviato](/docs/it/sub-agents#supported-frontmatter-fields), e il modello mostrato nell'intestazione di agent view.

L'uso di `bypassPermissions` con `claude --bg --permission-mode` è rifiutato finché non hai accettato il disclaimer di bypass eseguendo `claude --dangerously-skip-permissions` una volta in modo interattivo, poiché quella modalità permette a una sessione che non stai guardando di agire senza approvazione. Passare `--dangerously-skip-permissions` o `--permission-mode bypassPermissions` a `claude agents` mostra lo stesso disclaimer quando non l'hai accettato prima, e accettare applica `bypassPermissions` alle sessioni che avvii dalla vista. Passare `--allow-dangerously-skip-permissions` mostra lo stesso disclaimer anche, e accettare rende `bypassPermissions` disponibile nel ciclo `Shift+Tab` di quelle sessioni senza avviarle in esso.

<h3 id="settings-plugins-and-mcp-servers">
  Settings, plugins, and MCP servers
</h3>

Agent view accetta gli stessi flag di configurazione di `claude` per caricare settings, plugins, server MCP, e directory aggiuntive. Ogni flag si applica a agent view stesso e viene passato a ogni sessione che invii da esso, quindi un plugin o server MCP che carichi in questo modo è disponibile anche in quelle sessioni.

| Flag                                                                                             | Effetto                                                                     |
| :----------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------- |
| [`--settings <file-or-json>`](/docs/it/settings)                                                      | Sovrascrivi settings per agent view e sessioni inviate                      |
| [`--add-dir <path>`](/docs/it/permissions#additional-directories-grant-file-access-not-configuration) | Concedi accesso ai file a una directory aggiuntiva                          |
| [`--plugin-dir <path>`](/docs/it/plugins)                                                             | Carica un plugin da una directory locale                                    |
| [`--mcp-config <file-or-json>`](/docs/it/mcp)                                                         | Carica server MCP da un file di configurazione o stringa JSON               |
| `--strict-mcp-config`                                                                            | Usa solo i server MCP da `--mcp-config`, ignorando altre configurazioni MCP |

Ripeti `--add-dir`, `--plugin-dir`, o `--mcp-config` una volta per valore. La forma separata da spazi, come `--add-dir a b c`, non è supportata con `claude agents`.

L'esempio seguente apre agent view con un override di settings e una directory extra:

```bash theme={null}
claude agents --settings ./ci-settings.json --add-dir ../shared-lib
```

<h2 id="manage-sessions-from-the-shell">
  Gestire le sessioni dalla shell
</h2>

Ogni sessione in background ha un ID breve che puoi usare dalla shell. L'ID viene stampato quando avvii una sessione con `claude --bg`, e l'ID di ogni sessione è il nome della sua directory sotto `~/.claude/jobs/`. Questi comandi sono utili per lo scripting o quando non vuoi aprire agent view.

| Comando                      | Scopo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :--------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `claude agents`              | Apri agent view                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `claude agents --cwd <path>` | Apri agent view limitato alle sessioni avviate sotto `<path>`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `claude agents --json`       | Stampa le sessioni attive come un array JSON e esci: ogni sessione live, più sessioni in background che stanno ancora lavorando o sono bloccate anche quando il loro processo è uscito. Aggiungi `--all` per includere anche le sessioni in background completate. Ogni voce ha `cwd`, `kind`, e `startedAt`. Le voci in background hanno anche `id`, utilizzabile con `claude attach`/`logs`/`stop`, e `state`: uno di `working`, `blocked`, `done`, `failed`, o `stopped`. `pid` e `status` sono presenti solo mentre il processo è attivo, più `waitingFor` quando status è `waiting`, che dice su cosa è bloccata la sessione, come `permission prompt` o `input needed`; `sessionId` e `name` appaiono quando impostati. Una voce interattiva che non hai mai nominato porta un nome predefinito costruito dal nome della directory di lavoro più un suffisso di due caratteri, come `my-app-3f`. Combina con `--cwd <path>` per filtrare |
| `claude attach <id>`         | Collegati a una sessione in questo terminale                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `claude logs <id>`           | Stampa l'output recente della sessione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `claude stop <id>`           | Ferma una sessione. Accetta anche `claude kill`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `claude respawn <id>`        | Riavvia una sessione, in esecuzione o fermata, con la sua conversazione intatta, ad esempio per utilizzare un binario Claude Code aggiornato                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `claude respawn --all`       | Riavvia ogni sessione in esecuzione, ad esempio per spostare tutte le sessioni su un binario Claude Code aggiornato contemporaneamente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `claude rm <id>`             | Rimuovi una sessione dall'elenco. Rimuove un worktree creato da Claude per la sessione se non ha modifiche non sottoposte a commit e nessun commit che non sia stato sottoposto a push da nessuna parte; altrimenti la sessione viene mantenuta anche, e il comando stampa il percorso del worktree e il motivo in modo che tu possa risolverlo ed eseguire di nuovo `claude rm`. Lascia in posizione un worktree che hai creato tu stesso. La trascrizione della conversazione rimane sulla tua macchina locale e rimane disponibile tramite `claude --resume`                                                                                                                                                                                                                                                                                                                                                                                |
| `claude daemon status`       | Stampa lo stato del [supervisore](#the-supervisor-process), la versione, la directory socket e il numero di worker                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `claude daemon stop --any`   | Ferma il processo supervisore e le sessioni in background che ospita. Passa `--keep-workers` per lasciare le sessioni in background in esecuzione in modo che il supervisore successivo si riconnetta ad esse. Il prossimo `claude agents` o `claude --bg` avvia un supervisore nuovo                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |

<h2 id="how-background-sessions-are-hosted">
  Come sono ospitate le sessioni in background
</h2>

Ogni sessione elencata nella vista agente è considerata una sessione in background, indipendentemente dal fatto che tu sia attualmente collegato ad essa. Al contrario, una sessione avviata eseguendo `claude` direttamente è legata a quel terminale e termina quando si chiude, a meno che tu non la [invii in background](#from-inside-a-session).

<h3 id="the-supervisor-process">
  Il processo supervisor
</h3>

Le sessioni in background sono ospitate da un processo supervisor per utente, separato dal tuo terminale e da agent view. Il supervisor si avvia automaticamente la prima volta che metti una sessione in background o apri agent view, e non lo gestisci direttamente.

Quando un aggiornamento ha sostituito o rimosso il binario da cui è stato avviato un processo Claude Code in esecuzione, quel processo avvia il supervisor da un'altra copia installata, come il launcher `claude` installato o la versione più recente su disco.

Il supervisor mantiene un processo worker pre-riscaldato pronto in modo che un dispatch da agent view o `claude --bg` si avvii senza il ritardo di un avvio a freddo. Quando esegui un dispatch, il supervisor assegna il worker pre-riscaldato alla tua sessione, applica la directory, le impostazioni e le credenziali di quella sessione ad esso, e quindi avvia una sostituzione per il prossimo dispatch. Se nessun worker pre-riscaldato sano è disponibile, il supervisor avvia un processo fresco invece.

Il supervisor e le sue sessioni si autenticano con le stesse credenziali archiviate delle tue sessioni interattive e non fanno connessioni di rete aggiuntive oltre l'API del modello. Le variabili di selezione del provider come `CLAUDE_CODE_USE_BEDROCK` e gli alias `ANTHROPIC_DEFAULT_*_MODEL` vengono letti dalla shell che ha eseguito il dispatch di ogni sessione e vengono applicati al suo worker.

Il `PATH` della shell di dispatch viene applicato al worker allo stesso modo, quindi i comandi shell che la sessione esegue trovano gli stessi strumenti del tuo terminale. Prima di v2.1.203, una sessione in background manteneva il `PATH` della shell che ha avviato per primo il supervisor, quindi gli strumenti aggiunti al tuo `PATH` da allora potevano mancare, il più delle volte su Windows.

Una sessione in background non eredita variabili di endpoint gateway come `ANTHROPIC_BASE_URL` o le variabili di URL base equivalenti di Amazon Bedrock, Google Cloud's Agent Platform, e Microsoft Foundry dalla shell che ha avviato il supervisor. Senza un gateway esportato nella shell da cui esegui il dispatch, la sessione utilizza le tue credenziali archiviate e qualsiasi valore `env` nella directory del progetto [settings](/docs/it/settings). Per puntare ogni sessione in un progetto a un [gateway LLM](/docs/it/llm-gateway), imposta `ANTHROPIC_BASE_URL` nel blocco `env` del file `.claude/settings.json` di quel progetto.

Se esporti un gateway `ANTHROPIC_BASE_URL` nella shell da cui esegui il dispatch, raggiunge il worker di quella sessione. `ANTHROPIC_CUSTOM_HEADERS` e la credenziale esportata insieme ad essi vengono inoltrati con esso. Questo accade quando il supervisor è stato avviato da un ambiente con lo stesso gateway. Il supervisor cattura il suo ambiente dalla prima shell che apre agent view o esegue il dispatch di una sessione in background, quindi avviare dalla shell del gateway gli dà quell'ambiente. L'inoltro si applica anche solo alle sessioni inviate nella directory da cui stai eseguendo il dispatch, o messe in background dalla tua stessa sessione con `←` o `/background`: eseguire il dispatch in una directory diversa con `@repo` o `--cwd` non porta il gateway della shell, e il blocco `env` del file `settings.json` di quel progetto fornisce l'endpoint invece. Quando l'ambiente del supervisor porta un gateway diverso o nessuno, il worker mantiene le tue credenziali archiviate contro l'endpoint predefinito invece di mescolare la credenziale di un ambiente con l'endpoint di un altro. Prima di v2.1.203, il `ANTHROPIC_BASE_URL` della shell di dispatch veniva eliminato mentre il `ANTHROPIC_API_KEY` esportato insieme ad esso veniva mantenuto, quindi la chiave del gateway veniva inviata all'endpoint predefinito e ogni richiesta falliva con un 401.

L'endpoint inoltrato si applica solo a quel processo attivo e non viene mai scritto su disco. Quando il supervisor ferma una sessione inattiva e successivamente la riavvia, il processo riavviato legge il suo endpoint dalle tue impostazioni di nuovo: con un `ANTHROPIC_AUTH_TOKEN` del gateway torna alle tue credenziali archiviate, e con un `ANTHROPIC_API_KEY` emesso dal gateway può non riuscire ad autenticarsi fino a quando il gateway non è impostato nelle impostazioni.

Ogni sessione in background è il suo proprio processo Claude Code, gestito dal supervisor piuttosto che legato al tuo terminale. Una sessione che sta attivamente lavorando, aspettando il tuo input, o ha un terminale collegato mantiene il suo processo in esecuzione. Un comando shell in background in esecuzione, un subagente, un workflow dinamico, o un monitor conta come lavoro attivo, quindi un processo a lunga esecuzione come un server di sviluppo mantiene la sessione attiva.

Una volta che una sessione finisce e rimane scollega per circa un'ora, il supervisor ferma il suo processo per liberare risorse. Una sessione che hai [fissato](#organize-the-list) con `Ctrl+T` è esente e mantiene il suo processo in esecuzione mentre inattivo. Il transcript e lo stato rimangono su disco comunque, e la prossima volta che ti colleghi, fai peek, o rispondi a una sessione fermata, il supervisor avvia un processo fresco da dove l'ha lasciato. Quando ogni sessione è finita e nessun terminale è collegato, il supervisor stesso esce e si avvia di nuovo la prossima volta che ne hai bisogno.

Il lavoro in background che la sessione stessa ha avviato al livello superiore viene consegnato quando il suo processo viene fermato, riavviato, o aggiornato, incluso su Windows. Il prossimo processo avviato per quella sessione lo riprende:

* Un comando shell in background che è finito nel frattempo viene segnalato come completato con il suo output
* Un workflow dinamico riprende da dove l'ha lasciato
* Un [subagente in background](/docs/it/sub-agents#run-subagents-in-foreground-or-background) riprende dal suo proprio transcript

A partire da v2.1.198 la consegna copre tutti e tre. Prima di v2.1.198 copriva solo i comandi shell e i workflow, quindi un subagente in background si fermava con il processo e veniva segnalato come fallito al prossimo risveglio.

Il lavoro il cui stato vive solo all'interno del processo stesso si ferma con esso invece di essere consegnato. Questo riguarda i comandi shell che un subagente ha avviato, che il subagente ripreso può avviare di nuovo, e i [monitor](/docs/it/tools-reference#monitor-tool) in esecuzione, il cui flusso di eventi non può essere spostato in un altro processo.

L'eliminazione della sessione ferma tutto ciò che ha consegnato. Per fermare tutto il lavoro in background della sessione con il processo invece di consegnarlo, imposta la variabile di ambiente [`CLAUDE_CODE_DISABLE_BG_EXIT_HANDOFF`](/docs/it/env-vars#variables) su `1`.

Un processo riavviato trova la conversazione di una sessione che [si è spostata in un worktree](#how-file-edits-are-isolated) a metà attività: quando il transcript non è dove la sessione è stata avviata, Claude Code cerca anche sotto i worktree registrati del repository. Prima di v2.1.207, riaprire quella sessione da agent view dopo che il suo processo si era fermato poteva mostrare una conversazione vuota con solo il suo prompt originale, con il transcript ancora intatto su disco; aprire la sessione di nuovo su v2.1.207 o successivo lo recupera.

Se una sessione riavviata torna mostrando solo il suo prompt originale perché Claude Code ha frainteso il suo transcript come vuoto, il transcript della conversazione viene rinominato con un suffisso `.orphaned-` invece di essere eliminato, quindi rimane sulla tua macchina.

Una riga vuota rimasta dalla pressione di `←` che non è mai stata data un prompt viene rimossa completamente dopo circa cinque minuti in modo che l'elenco si cancelli da solo. Le sessioni avviate con `claude --bg` e le sessioni in attesa di un prompt di configurazione come una finestra di dialogo di fiducia non vengono rimosse in questo modo.

Quando l'host ha poca memoria, il supervisor ferma prima le sessioni inattive non fissate e ferma quelle fissate inattive solo se ciò non ha liberato nulla.

Il supervisor osserva il binario Claude Code installato su disco e si riavvia nella nuova versione dopo che l'[auto-updater](/docs/it/setup#auto-updates) regolare lo sostituisce. Questo è un watch di file locale, non un controllo di rete. Le sessioni in background sono processi scollegati, quindi continuano a girare attraverso il riavvio e il nuovo supervisor si ricollega a loro. Una sessione fissata inattiva viene anche riavviata in posizione sulla nuova versione in modo che raccolga l'aggiornamento senza che tu ti ricollega.

Una volta che il nuovo supervisor prende il controllo, riavvia anche le sessioni inattive rimanenti sulla nuova versione, un po' alla volta in background, dopo un breve ritardo che consente ai terminali collegati attraverso il riavvio di ricollegarsi per primi. Una sessione che sta lavorando, aspettando il tuo input, o ha un terminale collegato non viene interrotta; si sposta sulla nuova versione la prossima volta che il suo processo si riavvia. Prima di v2.1.206, il supervisor spostava solo poche sessioni inattive al minuto su una nuova versione, quindi le sessioni potevano continuare a eseguire quella vecchia per un po' dopo un aggiornamento.

Questi riavvii spostano solo una sessione su una versione più recente. Un supervisor che esegue una versione di Claude Code più vecchia di quella con cui è stato avviato il processo della sessione lascia quel processo da solo; la sessione continua a eseguire la versione più recente fino a quando un supervisor più recente non prende il controllo.

L'esecuzione di `claude attach` mentre il supervisor sta riavviando una sessione, sia per un aggiornamento, un blocco, o una migrazione, attende il processo di sostituzione invece di fallire. Una riga di stato come `Agent is updating to the new Claude Code…` nomina ciò che sta aspettando e conta i secondi trascorsi, e il comando si connette non appena la sessione è pronta. Dopo circa 60 secondi smette di aspettare e segnala un errore. Prima di v2.1.205, `claude attach` smetteva di riprovare dopo pochi secondi e stampava un errore mentre la sessione era ancora in fase di riavvio.

<h3 id="where-state-is-stored">
  Dove lo stato è archiviato
</h3>

Lo stato della sessione è archiviato sotto la tua directory di configurazione Claude Code. Se imposti [`CLAUDE_CONFIG_DIR`](/docs/it/env-vars), il supervisor usa quella directory invece di `~/.claude` e viene eseguito come un'istanza separata con le sue proprie sessioni.

| Percorso                         | Contenuti                                                                                                                  |
| :------------------------------- | :------------------------------------------------------------------------------------------------------------------------- |
| `~/.claude/daemon.log`           | Log del supervisor                                                                                                         |
| `~/.claude/daemon/roster.json`   | Elenco delle sessioni in background in esecuzione, usato per ricollegarsi dopo un riavvio                                  |
| `~/.claude/jobs/<id>/state.json` | Stato per sessione mostrato in agent view                                                                                  |
| `~/.claude/jobs/<id>/tmp/`       | Directory di scratch per sessione. Le scritture qui non richiedono il permesso. Rimossa quando la sessione viene eliminata |

Ogni sessione in background ha la variabile di ambiente `CLAUDE_JOB_DIR` impostata sulla sua directory `~/.claude/jobs/<id>`, quindi i comandi shell che la sessione esegue possono scrivere file temporanei su `$CLAUDE_JOB_DIR/tmp` senza collidere con sessioni parallele.

Per ispezionare questo stato senza leggere direttamente i file, esegui `claude daemon status`. Riporta se il supervisor è raggiungibile, il suo ID processo e versione, la directory socket, e quante sessioni in background sono attive.

Il comando avverte anche quando il supervisor in esecuzione è su una versione diversa da quella di `claude` che hai invocato, il che accade dopo un aggiornamento in cui il supervisor non si è ancora riavviato. L'avviso mostra entrambe le versioni e ti dice di eseguire `claude daemon stop --any` per raccogliere la nuova versione. Quando Claude Code è installato come servizio del sistema operativo, il comando suggerito è `claude daemon stop` senza il flag.

Le sessioni sopravvivono a quel mismatch di versione intatto: una versione più vecchia di Claude Code che aggiorna il file `state.json` di una sessione preserva i campi che non riconosce e mantiene la sessione elencata. L'elenco delle sessioni in `roster.json` segue la stessa regola: una versione più vecchia che lo riscrive preserva i campi che una versione più nuova ha scritto, quindi le sessioni avviate dalla versione più nuova rimangono raggiungibili e continuano ad accettare input dopo il riavvio del supervisor. Prima di v2.1.200, le versioni più vecchie potevano eliminare quei campi al riscrittura.

Su Windows, `claude daemon status` espone l'errore di file sottostante quando il file della chiave pipe del daemon è bloccato o illeggibile invece di segnalare un errore di connessione generico.

<h3 id="turn-off-agent-view">
  Disattiva agent view
</h3>

Per disattivare completamente gli agenti in background e agent view, imposta l'[impostazione](/docs/it/settings) `disableAgentView` su `true` o imposta la variabile di ambiente `CLAUDE_CODE_DISABLE_AGENT_VIEW`. Gli amministratori possono applicare questo attraverso [impostazioni gestite](/docs/it/permissions#managed-settings).

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="claude-agents-lists-subagents-instead-of-opening-agent-view">
  `claude agents` elenca subagenti invece di aprire agent view
</h3>

Se `claude agents` stampa un conteggio seguito dai tuoi subagenti configurati e poi esce, agent view non è disponibile nel tuo ambiente. Esegui `claude update` per installare la versione più recente.

Se agent view ancora non si apre dopo l'aggiornamento, verifica se è stata [disattivata](#turn-off-agent-view) da un'impostazione o da una variabile di ambiente.

<h3 id="agent-view-opens-with-no-sessions">
  Agent view si apre senza sessioni
</h3>

Prima di inviare la tua prima sessione, agent view mostra le intestazioni di sezione vuote con una descrizione sotto ognuna, più una spiegazione su una riga sopra l'input, al posto dell'elenco delle sessioni. Digita un prompt nell'input in basso e premi `Enter` per inviare la tua prima sessione.

<h3 id="backgrounding-shows-a-background-this-session-dialog">
  Lo spostamento in background mostra una finestra di dialogo `Background this session?`
</h3>

Se premere `←` per mettere in background la sessione corrente mostra una finestra di dialogo `Background this session?`, la sessione ha lavoro in corso che non può trasferirsi alla sessione in background, come un [monitor](/docs/it/tools-reference#monitor-tool) in esecuzione, e Claude Code non lo fermerà silenziosamente. La finestra di dialogo nomina il lavoro che verrà fermato e, separatamente, conta i compiti che si trasferiscono. Esegui `/tasks` per vedere tutto ciò che è in esecuzione, quindi conferma per metterla in background comunque o scegli `Stay` per lasciare che il lavoro finisca prima. Vedi [Da dentro una sessione](#from-inside-a-session) per sapere quali tipi di compiti si trasferiscono e quali vengono fermati.

<h3 id="prompt-rejected-as-too-short">
  Prompt rifiutato come troppo breve
</h3>

L'input di dispatch si aspetta una descrizione del compito, non un'apertura conversazionale. Un prompt più corto di quattro caratteri viene rifiutato con un suggerimento `Too short` in modo che una pressione accidentale non avvii una sessione. Descrivi cosa vuoi che la sessione faccia, come `investigate the flaky checkout test`.

<h3 id="sessions-show-as-failed-after-shutdown">
  Le sessioni mostrano come non riuscite dopo lo spegnimento
</h3>

Lo spegnimento o il riavvio della tua macchina interrompe le sessioni in background in esecuzione, quindi mostrano come non riuscite quando apri di nuovo agent view. Collegati, fai peek, o rispondi a qualsiasi sessione e la sessione si riavvia da dove l'ha lasciata.

Il sonno da solo non causa questo. Le sessioni vengono preservate durante il sonno e il supervisor si ricollega ad esse al risveglio.

<h3 id="opening-a-session-says-the-conversation-is-already-open">
  L'apertura di una sessione dice che la conversazione è già aperta
</h3>

L'apertura di una riga interrotta la cui conversazione è anche tenuta aperta da un altro processo Claude Code non interattivo in esecuzione, ad esempio un worker in background per la stessa conversazione che sta ancora terminando, mostra `This conversation is already open in another running Claude session` invece di avviare il processo della riga, perché due processi non possono scrivere sulla stessa trascrizione. Rispondi nella sessione che ha già la conversazione aperta, o esci da essa e apri di nuovo la riga. Una risposta che hai digitato con il tentativo rifiutato non viene persa; viene inviata la prossima volta che la sessione si avvia.

Prima della v2.1.203, questo stato avviava comunque un secondo processo. Quel processo usciva con un errore `currently running as a background agent` e la riga mostrava come non riuscita.

<h3 id="a-session-fails-before-starting-with-a-possibly-low-memory-note">
  Una sessione non riesce prima di iniziare con una nota `possibly low memory`
</h3>

A partire dalla v2.1.199, quando il processo di una sessione in background esce prima di finire l'avvio e l'host ha poca memoria, lo stato della riga nomina l'uscita e aggiunge `possibly low memory — free some up and retry`. Le versioni precedenti mostravano solo il motivo dell'uscita nudo per questo errore.

La nota è un'ipotesi, non una causa confermata. Claude Code la aggiunge solo quando il processo è uscito silenziosamente, senza scrivere un errore e senza essere fermato da un segnale, e l'host ha segnalato poca memoria in quel momento. Quando il processo ha scritto un errore prima di uscire, la riga mostra invece quell'errore.

Libera memoria sulla macchina, quindi collegati, fai peek, o rispondi alla riga e il supervisor avvia un processo fresco per la sessione. Quando la memoria rimane bassa, il supervisor [ferma anche le sessioni inattive](#the-supervisor-process) per liberare risorse da solo.

<h3 id="agent-view-says-the-background-service-did-not-respond">
  Agent view dice che il servizio in background non ha risposto
</h3>

Se il collegamento, il peek, o `claude logs` segnala che il servizio in background non ha risposto, il processo supervisor ha probabilmente subito un blocco. Fermalo e lascia che il prossimo `claude agents` avvii uno nuovo. Per mantenere le tue sessioni in background in esecuzione durante il riavvio, passa `--keep-workers`:

```bash theme={null}
claude daemon stop --any --keep-workers
```

Il nuovo supervisor si ricollega alle sessioni in esecuzione. Senza `--keep-workers`, il comando termina anche le sessioni in background. Il flag `--any` conferma che desideri fermare un supervisor che è stato avviato su richiesta piuttosto che come servizio installato, che è l'impostazione predefinita.

Un supervisor che si avvia ma non riesce ad accettare connessioni esce e rilascia il suo blocco da solo, quindi il prossimo `claude agents` avvia uno nuovo senza questo arresto manuale. I passaggi precedenti si applicano quando un supervisor in esecuzione subisce un blocco.

Su Windows, se il supervisor non risponde alla richiesta di arresto, il comando stampa il suo ID di processo. Termina quel processo con `taskkill /PID <pid>` per completare il recupero. Le sessioni in background vengono comunque preservate quando hai passato `--keep-workers`.

<h3 id="dispatch-fails-with-could-not-resolve-authentication-method">
  Dispatch non riesce con `Could not resolve authentication method`
</h3>

Se un dispatch in background non riesce con `Could not resolve authentication method` mentre le sessioni interattive si autenticano normalmente, il worker che ha ricevuto il dispatch non ha raccolto le credenziali. Il supervisor fornisce uno snapshot di credenziali fresco quando assegna un [worker pre-riscaldato](#the-supervisor-process), quindi questo errore significa che nessuna credenziale memorizzata era disponibile per il processo supervisor stesso. Conferma di aver eseguito `/login` o di aver configurato una chiave API, quindi ferma il supervisor:

```bash theme={null}
claude daemon stop --any --keep-workers
```

Il prossimo `claude agents` o `claude --bg` avvia un supervisor fresco che legge le tue credenziali memorizzate. Se ti autentichi con una variabile di ambiente come `ANTHROPIC_API_KEY` piuttosto che con `/login`, esegui quel comando successivo da una shell dove la variabile è impostata.

Vedi il [riferimento degli errori](/docs/it/errors#could-not-resolve-authentication-method) per l'elenco completo delle cause e delle correzioni.

<h3 id="background-sessions-can’t-read-desktop-documents-or-downloads-on-macos">
  Le sessioni in background non riescono a leggere Desktop, Documents o Downloads su macOS
</h3>

Su macOS, l'host della sessione in background viene eseguito come processo separato e richiede l'accesso alle cartelle protette separatamente dal tuo terminale. Se una sessione in background segnala `Operation not permitted` durante la lettura di `~/Desktop`, `~/Documents`, `~/Downloads`, o un'altra posizione protetta, concedi l'accesso in Impostazioni di Sistema sotto Privacy e Sicurezza > File e Cartelle, o abilita Accesso Completo al Disco per la voce.

Con il programma di installazione nativo, la voce appare come Claude Code e la concessione persiste tra gli aggiornamenti. Con altri metodi di installazione come Homebrew o npm, la voce mostra il percorso del binario e potrebbe dover essere concessa di nuovo dopo l'aggiornamento.

<h3 id="background-sessions-can’t-reach-local-network-hosts-on-macos">
  Le sessioni in background non riescono a raggiungere host della rete locale su macOS
</h3>

Su macOS 15 e versioni successive, il sistema blocca un processo dal raggiungimento di dispositivi sulla tua rete locale finché non concedi il permesso di Rete Locale. Prima della v2.1.198 l'host della sessione in background non ha mai richiesto quel permesso, quindi i comandi che mirano a un indirizzo LAN non riuscivano con `connect: no route to host` anche se lo stesso comando funzionava in un terminale in primo piano. A partire dalla v2.1.198, il primo comando in una sessione in background che si connette a un indirizzo della rete locale attiva il prompt di permesso di Rete Locale di macOS per Claude Code. Concedilo una volta e quei comandi raggiungono gli host LAN nello stesso modo in cui lo fanno in un terminale in primo piano.

<h3 id="a-session-is-slow-to-respond-after-attaching">
  Una sessione è lenta a rispondere dopo il collegamento
</h3>

Una volta che una sessione è finita e rimane scollega per circa un'ora, il supervisor ferma il suo processo per liberare risorse. Il collegamento avvia un processo fresco da dove l'ha lasciato e passa alla sessione immediatamente mentre il processo si riavvia. Le sessioni che stanno lavorando, aspettando te, o [fissate](#organize-the-list) non sono fermate in questo modo, quindi fissa una sessione con `Ctrl+T` per mantenerla reattiva.

Mentre il processo si avvia, l'ultimo schermo della trascrizione della sessione viene mostrato con una nota `Session is starting` sotto di esso, e la sessione live lo sostituisce non appena è pronta.

<h3 id="claude/worktrees/-is-filling-up">
  `.claude/worktrees/` si sta riempiendo
</h3>

L'eliminazione di una sessione in agent view rimuove il worktree che Claude ha creato per essa, e un worktree che non può essere rimosso in sicurezza [mantiene la sua riga di sessione](#organize-the-list) in modo che non sia orfano. `claude rm` mantiene un worktree che ha modifiche non committate, e la sua riga di sessione, e stampa il percorso mantenuto. Elenca le voci rimaste con `git worktree list` nella directory del progetto e rimuovi ognuna con `git worktree remove <path>`. Vedi [Pulisci i worktrees](/docs/it/worktrees#clean-up-worktrees).

<h2 id="limitations">
  Limitazioni
</h2>

Agent view è un'anteprima di ricerca con le seguenti limitazioni:

* **I limiti di velocità si applicano**: le sessioni in background consumano l'utilizzo dell'abbonamento allo stesso modo delle sessioni interattive, quindi eseguire dieci agenti in parallelo utilizza la quota approssimativamente dieci volte più velocemente rispetto all'esecuzione di uno.
* **Le sessioni sono locali**: le sessioni in background vengono eseguite sulla vostra macchina. Vengono preservate durante la sospensione ma si fermano se la macchina si spegne.
* **I worktrees creati da Claude vengono eliminati con la sessione in agent view**: unite i cambiamenti prima di eliminare una sessione che ha modificato file nel suo proprio worktree. Un worktree con commit che non sono stati inviati da nessuna parte viene mantenuto insieme alla sessione. `claude rm` mantiene anche un worktree che ha modifiche non sottoposte a commit insieme alla sua sessione, e un worktree che avete creato voi stessi viene lasciato in posizione.

<h2 id="related-resources">
  Risorse correlate
</h2>

Per altri modi di eseguire Claude in parallelo, vedi:

* [Esegui agenti in parallelo](/docs/it/agents): confronta agent view con subagenti, team di agenti e worktrees
* [Team di agenti](/docs/it/agent-teams): coordina più sessioni che si messaggiano l'una con l'altra
* [Claude Code on the web](/docs/it/claude-code-on-the-web): esegui sessioni in un ambiente cloud gestito invece che localmente

<h2 id="version-history">
  Cronologia delle versioni
</h2>

Agent view si è evoluto rapidamente durante l'anteprima di ricerca. Se sei su una versione più vecchia di Claude Code, alcuni comportamenti su questa pagina potrebbero differire; in particolare, `claude agents` rifiuta i flag che non supporta ancora con un errore di opzione sconosciuta. La tabella sottostante elenca quando ogni flag e comportamento è stato aggiunto.

| Versione | Cambiamento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| v2.1.208 | Il collegamento a una sessione il cui processo si è fermato mostra l'ultimo schermo pieno della sua trascrizione mentre il processo si avvia, invece di solo una nota `Session is starting`. Una risposta che non può essere consegnata perché il servizio in background non è raggiungibile o l'invio fallisce viene salvata e inviata come prompt successivo della sessione quando il suo processo si avvia di nuovo; prima di questa versione, una risposta persa mentre il servizio in background non era raggiungibile veniva scartata. Un processo il cui binario è stato sostituito da un aggiornamento può comunque avviare il supervisor, dal launcher `claude` installato o dalla versione più recente su disco, invece di fallire fino al riavvio di Claude Code. Un supervisor che esegue una versione più vecchia non riavvia mai una sessione inattiva avviata da una versione più nuova sul suo binario più vecchio. L'eliminazione di una sessione rimuove il suo worktree anche dopo che la sessione ha spostato il worktree su un ramo diverso, e mantiene il worktree insieme alla riga della sessione quando il worktree ha commit che non sono stati spinti da nessuna parte o un'altra sessione lo rivendica, invece di distruggere i commit o lasciare il worktree orfano. `/install-github-app` e l'elenco delle impostazioni `/mcp` e le sue azioni di autenticazione vengono rifiutate in una sessione in background con un messaggio che nomina l'alternativa; solo in v2.1.208, il picker `/model` è stato rifiutato allo stesso modo e un `/model <name>` digitato ha cambiato solo quella sessione invece di salvare anche il tuo modello predefinito. |
| v2.1.207 | Il pannello peek si apre con la frase che la riga tronca, come la domanda esatta per una sessione che ti sta aspettando, e mostra quanto a lungo una sessione bloccata è stata in attesa come una singola riga `waiting 3m` invece di anteporre lo stesso timestamp alla frase di stato e alla domanda. Incollare di nuovo lo stesso testo nell'input di dispatch espande il placeholder `[Pasted text #N]` compresso invece di aggiungerne un secondo. Una sessione in background denominata accettando un piano mostra quel nome sulla sua riga. Una sessione in background che si è spostata in un worktree mantiene la sua conversazione quando il suo processo viene riavviato dalla vista agente.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| v2.1.206 | I riassunti delle righe riempiono la larghezza rimanente della riga e si troncano solo al bordo destro del terminale invece che a 64 colonne. Dopo che il supervisor si riavvia in una nuova versione di Claude Code, riavvia le restanti sessioni in background inattive su quella versione in background invece di poche al minuto. L'eliminazione di una sessione con `Ctrl+X` o `claude rm` la cancella anche dall'elenco delle sessioni del supervisor, quindi la riga non riappare più dopo un riavvio del supervisor.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| v2.1.205 | I riassunti delle righe mostrano il rapporto di una riga della sessione, troncato a 64 colonne, invece di una raw tool invocation o un conteggio `done/total`; le righe raggruppate per directory si aprono con una parola di stato colorata. Il pannello peek si apre con la frase di stato completa e, per una sessione in attesa di te, la sua domanda esatta sopra l'input di risposta. Le sessioni che modificano, commentano, chiudono, o contrassegnano una pull request come pronta con `gh` sono collegate ad essa, non solo quelle che creano o controllano una pull request, un push collega una pull request anche quando il nome del ramo locale non corrisponde, e una pull request il cui output del comando di creazione ha superato il limite inline è collegata anche. Un turno senza testo leggibile mantiene lo stato precedente della sessione invece di capovolgere di nuovo a `Working`. `claude attach` attende fino a circa 60 secondi una sessione che si sta riavviando, con una riga di stato che nomina il motivo, invece di fallire.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| v2.1.203 | Un gateway `ANTHROPIC_BASE_URL` esportato nella shell di dispatching raggiunge le sessioni inviate da essa nella stessa directory quando il supervisor condivide quel gateway environment, invece di essere scartato mentre la chiave API esportata insieme ad esso veniva mantenuta. Il `PATH` della shell di dispatching viene applicato a ogni worker della sessione. Premere `←` mentre i subagent sono in esecuzione attende che finiscano invece di riavviarli dopo dieci secondi. L'elenco vuoto mostra sempre le intestazioni della sezione con una descrizione sotto ciascuna. Digitare `@` nell'input di dispatch elenca anche i git worktrees registrati del repository di avvio che si trovano all'interno del suo albero di directory. Uno sforzo ereditato dall'impostazione `effortLevel` segue i successivi edit a quell'impostazione invece di essere fissato al dispatch. Aprire una sessione fermata la cui conversazione è già aperta in un'altra sessione in esecuzione viene rifiutato con un messaggio invece di far fallire la riga. Un comando che non è disponibile in agent view lascia il testo digitato nell'input. Un hook `WorktreeCreate` che fallisce al di fuori di un repository git non blocca più la sessione dall'editing dei file.                                                                                                                                                                                                                                                                                                                                                                                                            |
| v2.1.202 | Un nome impostato con `/rename` o `Ctrl+R` su una sessione in background persiste quando il supervisor arresta e riavvia il suo processo, invece di ripristinare il nome con cui la sessione è stata inviata.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| v2.1.200 | Una versione più vecchia di Claude Code che riscrive l'elenco delle sessioni in `roster.json` preserva i campi scritti da una versione più nuova, corrispondendo alla garanzia `state.json` esistente, quindi le sessioni avviate dalla versione più nuova continuano ad accettare input dopo il riavvio del supervisor. Quando apri una sessione che ha smesso di rispondere, il supervisor riavvia il suo processo e la sessione continua la risposta interrotta da dove si era fermata.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| v2.1.199 | Una sessione in background il cui processo esce prima di finire l'avvio su un host con poca memoria mostra `possibly low memory — free some up and retry` nello stato della sua riga invece di solo il motivo dell'uscita nudo. Mettere in background una sessione con `←` o `/background` trasporta il suo `/color` alla nuova riga.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| v2.1.198 | Agent view invia una notifica attraverso `preferredNotifChannel` quando una sessione in background ha bisogno di input, finisce, o fallisce, e attiva l'hook `Notification` con il tipo `agent_needs_input` o `agent_completed`. `←` e `/exit` dentro `claude attach <id>` ritornano a agent view invece di uscire alla shell; `Ctrl+Z` ritorna alla shell. Una sessione in background che ha isolato il suo lavoro in un worktree esegue il commit, spinge il suo ramo isolato proprio, mai `main` o `master`, e apre una pull request in bozza quando finisce invece di chiedere prima. `/login` viene eseguito in agent view e apre la finestra di dialogo di accesso. La finestra di dialogo di uscita `Background work is running` offre `Move to background and exit`. Il trasferimento di uscita copre anche i subagent in background, che riprendono dal loro transcript al prossimo risveglio invece di essere segnalati come falliti. `claude --bg` combinato con `-p` o `--print` viene rifiutato con un errore.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| v2.1.196 | Una singola pressione di `←` mette in background una sessione in primo piano; le versioni precedenti richiedevano due pressioni, con un suggerimento nel footer e una conferma. `--dangerously-skip-permissions` passato a `claude agents` mostra il disclaimer di bypass invece di essere silenziosamente scartato. Le sessioni interattive che non hai mai nominato portano un nome predefinito come `my-app-3f` negli elenchi di sessioni e `claude agents --json`. I comandi shell in background e i workflow dinamici sopravvivono al processo della sessione che viene fermato, riavviato, o aggiornato, incluso su Windows; imposta `CLAUDE_CODE_DISABLE_BG_EXIT_HANDOFF=1` per disattivare il trasferimento. Un transcript frainteso come vuoto al riavvio viene rinominato con un suffisso `.orphaned-` invece di essere eliminato.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| v2.1.195 | Il lavoro in corso si trasferisce anche quando metti in background una sessione su Windows; imposta `CLAUDE_DISABLE_ADOPT=1` per fermarlo invece. Il gruppo `Completed` riempie lo spazio verticale rimanente e l'intestazione si compatta su terminali corti. Una versione più vecchia di Claude Code non scarta più i campi `state.json` delle sessioni più nuove o nasconde quelle sessioni da `claude agents`. Il collegamento a una sessione fermata passa immediatamente invece di mostrare una schermata vuota per fino a cinque secondi. Un supervisor che non riesce ad accettare connessioni esce e rilascia il suo blocco da solo.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| v2.1.174 | Le sessioni in background non ereditano più variabili di endpoint gateway come `ANTHROPIC_BASE_URL` dalla shell di avvio del supervisor; il supervisor fornisce uno snapshot di credenziali fresco ai worker pre-riscaldati, correggendo gli errori spurii di `Could not resolve authentication method`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| v2.1.172 | `/model` nell'input di dispatch imposta un override del modello di dispatch con ambito di sessione.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| v2.1.161 | I riassunti delle righe mostrano un conteggio `done/total` per elementi di lavoro paralleli; il pannello peek nomina l'elemento di lavoro parallelo che sta girando più a lungo.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| v2.1.157 | `claude agents` accetta `--agent`; le sessioni inviate rispettano l'impostazione `agent`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| v2.1.145 | Dettatura vocale supportata nell'input di risposta del pannello peek e nell'input di dispatch.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| v2.1.143 | Impostazione `worktree.bgIsolation` aggiunta; `claude agents` accetta `--allow-dangerously-skip-permissions`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| v2.1.142 | `claude agents` accetta `--permission-mode`, `--model`, `--effort`, `--dangerously-skip-permissions`, `--settings`, `--add-dir`, `--plugin-dir`, `--mcp-config`, e `--strict-mcp-config`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| v2.1.141 | `claude agents` accetta `--cwd` per limitare l'elenco a un progetto.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| v2.1.139 | Agent view introdotto come anteprima di ricerca.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
