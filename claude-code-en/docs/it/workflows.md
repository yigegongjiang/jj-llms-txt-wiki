> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Orchestrare subagenti su larga scala con flussi di lavoro dinamici

> I flussi di lavoro dinamici orchestrano molti subagenti da uno script che Claude scrive e che puoi rieseguire. Usali per audit di codebase, migrazioni su larga scala e ricerche con verifica incrociata.

<Note>
  I flussi di lavoro dinamici richiedono Claude Code v2.1.154 o successivo e sono disponibili su tutti i piani a pagamento, con accesso all'API Anthropic, e su Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry. Su Pro, attivali dalla riga Dynamic workflows in `/config`.
</Note>

Un flusso di lavoro dinamico è uno script JavaScript che orchestra [subagenti](/docs/it/sub-agents) su larga scala. Claude scrive lo script per il compito che descrivi, e un runtime lo esegue in background mentre la tua sessione rimane reattiva.

Ricorri a un flusso di lavoro quando un compito richiede più agenti di quanti una conversazione possa coordinare, o quando vuoi che l'orchestrazione sia codificata come uno script che puoi leggere e rieseguire. Gli esempi includono una ricerca di bug a livello di codebase, una migrazione di 500 file, una domanda di ricerca che richiede fonti verificate l'una rispetto all'altra, e un piano difficile che vale la pena di elaborare da diversi angoli indipendenti prima di impegnarsi in uno.

<h2 id="when-to-use-a-workflow">
  Quando usare un flusso di lavoro
</h2>

[Subagenti](/docs/it/sub-agents), [skills](/docs/it/skills), [team di agenti](/docs/it/agent-teams) e flussi di lavoro possono tutti eseguire un compito multi-step. La differenza è chi tiene il piano:

|                                     | Subagenti                         | Skills                         | Team di agenti                                | Flussi di lavoro                            |
| :---------------------------------- | :-------------------------------- | :----------------------------- | :-------------------------------------------- | :------------------------------------------ |
| Che cosa è                          | Un worker Claude che genera       | Istruzioni che Claude segue    | Un agente lead che supervisiona sessioni peer | Uno script che il runtime esegue            |
| Chi decide cosa viene eseguito dopo | Claude, turno per turno           | Claude, seguendo il prompt     | L'agente lead, turno per turno                | Lo script                                   |
| Dove vivono i risultati intermedi   | Finestra di contesto di Claude    | Finestra di contesto di Claude | Un elenco di compiti condiviso                | Variabili dello script                      |
| Che cosa è ripetibile               | La definizione del worker         | Le istruzioni                  | La definizione del team                       | L'orchestrazione stessa                     |
| Scala                               | Alcuni compiti delegati per turno | Uguale ai subagenti            | Una manciata di peer a lunga esecuzione       | Decine a centinaia di agenti per esecuzione |
| Interruzione                        | Riavvia il turno                  | Riavvia il turno               | I compagni di team continuano a funzionare    | Riprendibile nella stessa sessione          |

Un flusso di lavoro sposta il piano nel codice. Con subagenti, skills e team di agenti, Claude è l'orchestratore: decide turno per turno cosa generare o assegnare dopo, e ogni risultato finisce nella finestra di contesto. Uno script di flusso di lavoro tiene il ciclo, la ramificazione e i risultati intermedi stessi, quindi il contesto di Claude contiene solo la risposta finale.

Spostare il piano nel codice consente anche a un flusso di lavoro di applicare un modello di qualità ripetibile, non solo eseguire più agenti: può avere agenti indipendenti che si rivedono avversarialmente i risultati l'uno dell'altro prima che vengano segnalati, o elaborare un piano da diversi angoli e pesarli l'uno rispetto all'altro, così ottieni un risultato più affidabile di un singolo passaggio.

<h2 id="run-a-bundled-workflow">
  Eseguire un flusso di lavoro in bundle
</h2>

Il modo più veloce per vedere un flusso di lavoro in azione è eseguire `/deep-research`, il [flusso di lavoro integrato](#bundled-workflows) che Claude Code include per investigare una domanda su molte fonti. Vedrai gli agenti lavorare attraverso una serie di fasi in background mentre la tua sessione rimane libera, e otterrai un rapporto alla fine invece di una trascrizione turno per turno.

<Steps>
  <Step title="Eseguire il flusso di lavoro">
    Esegui `/deep-research` con una domanda che vuoi investigare. Distribuisce ricerche web su diversi angoli, recupera e verifica in modo incrociato le fonti che trova, e sintetizza un rapporto citato.

    ```text theme={null}
    /deep-research What changed in the Node.js permission model between v20 and v22?
    ```
  </Step>

  <Step title="Consentire i flussi di lavoro">
    Claude Code chiede se consentire il flusso di lavoro. Seleziona **Sì** per continuare. Il prompt esatto dipende dalla tua modalità di autorizzazione. Vedi [Approvare il piano prima che venga eseguito](#approve-the-plan-before-it-runs) per le opzioni per modalità.
  </Step>

  <Step title="Guardare il progresso">
    L'esecuzione inizia in background. Esegui `/workflows`, usa i tasti freccia per selezionare l'esecuzione e premi Invio per aprire la sua vista di progresso:

    ```text theme={null}
    /workflows
    ```

    La vista mostra ogni fase con il suo conteggio di agenti, totale di token e tempo trascorso. Approfondisci qualsiasi fase per vedere i suoi agenti e cosa ha trovato ognuno. Vedi [Guardare l'esecuzione](#watch-the-run) per l'insieme completo di controlli.

    Puoi anche guardare dal pannello attività sotto la casella di input: un riepilogo di progresso su una riga appare lì mentre l'esecuzione è in corso. Premi la freccia giù per focalizzarlo, quindi Invio per espandere.
  </Step>

  <Step title="Leggere il rapporto">
    Quando l'esecuzione finisce, il rapporto arriva nella tua sessione. Cita le fonti da cui proviene ogni affermazione, con affermazioni che non hanno superato la verifica incrociata già filtrate.

    A partire da v2.1.196, quando gli agenti verificatori non riescono a controllare un'affermazione, ad esempio dopo un limite di velocità o un errore API, il rapporto elenca tale affermazione come non verificata invece di contarla come confutata.
  </Step>
</Steps>

Per eseguire un flusso di lavoro per il tuo compito, [fai scrivere uno a Claude](#have-claude-write-a-workflow), e una volta che un'esecuzione fa quello che volevi puoi [salvarlo](#save-the-workflow-for-reuse) come comando tuo.

<h3 id="bundled-workflows">
  Flussi di lavoro in bundle
</h3>

Claude Code include `/deep-research` come flusso di lavoro integrato:

| Comando                     | Che cosa fa                                                                                                                                                                                                                                                                                                                                                   |
| :-------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `/deep-research <question>` | Distribuisce ricerche web su una domanda su diversi angoli, recupera e verifica in modo incrociato le fonti che trova, vota su ogni affermazione e restituisce un rapporto citato con affermazioni che non hanno superato la verifica incrociata filtrate. Richiede che lo strumento [WebSearch](/docs/it/tools-reference#websearch-tool-behavior) sia disponibile |

[I flussi di lavoro che salvi](#save-the-workflow-for-reuse) tu stesso diventano comandi allo stesso modo e appaiono nell'autocompletamento `/` insieme a quelli in bundle.

<h3 id="watch-the-run">
  Guardare l'esecuzione
</h3>

I flussi di lavoro vengono eseguiti in background, quindi la sessione rimane reattiva mentre gli agenti lavorano. Esegui `/workflows` in qualsiasi momento per elencare i flussi di lavoro in esecuzione e completati, quindi selezionane uno per aprire la sua vista di progresso.

```text theme={null}
/workflows
```

La vista di progresso mostra ogni fase con i suoi conteggi di agenti, totali di token e tempo trascorso. Il piè di pagina elenca il tasto per ogni azione:

| Tasto         | Azione                                                                                                                                                  |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `↑` / `↓`     | Selezionare una fase o un agente                                                                                                                        |
| `Invio` o `→` | Approfondire la fase selezionata, quindi in un agente per leggere il suo prompt, le recenti chiamate di strumenti e il risultato                        |
| `Esc` o `←`   | Tornare indietro di un livello. Nelle versioni da v2.1.203 a v2.1.205, `←` non è tornato indietro da una fase o un agente; usa `Esc` su quelle versioni |
| `j` / `k`     | Scorrere all'interno del dettaglio dell'agente quando trabocca                                                                                          |
| `f`           | Filtrare l'elenco degli agenti nella fase selezionata per stato. Premi di nuovo per ciclare                                                             |
| `p`           | Mettere in pausa o riprendere l'esecuzione                                                                                                              |
| `x`           | Fermare l'agente selezionato, o fermare l'intero flusso di lavoro quando il focus è sull'esecuzione                                                     |
| `r`           | Riavviare l'agente in esecuzione selezionato                                                                                                            |
| `s`           | [Salvare](#save-the-workflow-for-reuse) lo script dell'esecuzione come comando                                                                          |

<h2 id="have-claude-write-a-workflow">
  Far scrivere a Claude un flusso di lavoro
</h2>

Puoi far scrivere a Claude un flusso di lavoro per il tuo compito in due modi:

* [Chiedere un flusso di lavoro](#ask-for-a-workflow-in-your-prompt) nel tuo prompt, con le tue stesse parole o includendo la parola chiave `ultracode`, e Claude ne scrive uno per il compito.
* [Lasciare che Claude decida con ultracode](#let-claude-decide-with-ultracode): imposta `/effort ultracode` e Claude pianifica un flusso di lavoro per ogni compito sostanziale nella sessione.

Puoi anche eseguire un comando di flusso di lavoro che già esiste: un [flusso di lavoro in bundle](#bundled-workflows) come `/deep-research`, o uno che hai [salvato](#save-the-workflow-for-reuse).

<h3 id="ask-for-a-workflow-in-your-prompt">
  Chiedere un flusso di lavoro nel tuo prompt
</h3>

Per eseguire un singolo compito come flusso di lavoro senza cambiare il livello di sforzo della sessione, includi la parola chiave `ultracode` nel tuo prompt. Chiedere con le tue stesse parole, ad esempio "usa un flusso di lavoro" o "esegui un flusso di lavoro", funziona anche: Claude tratta una richiesta diretta come lo stesso opt-in. Prima della v2.1.160 la parola chiave letterale era `workflow`; le richieste in linguaggio naturale funzionano in entrambe le versioni.

```text theme={null}
ultracode: audit every API endpoint under src/routes/ for missing auth checks
```

Claude Code evidenzia la parola chiave nel tuo input e Claude scrive uno script di flusso di lavoro per il compito invece di lavorarci turno per turno. Se non intendevi avviare un flusso di lavoro, premi `Option+W` su macOS o `Alt+W` su Windows e Linux per ignorare l'evidenziazione per questo prompt, oppure premi backspace mentre il cursore è subito dopo la parola chiave evidenziata. Per impedire che la parola chiave attivi un flusso di lavoro del tutto, disattiva il trigger della parola chiave Ultracode in `/config`.

Se l'esecuzione fa quello che volevi, puoi [salvarlo come comando](#save-the-workflow-for-reuse) dopo.

Se hai già un orchestrator costruito in un altro modo, come una cartella di prompt di subagent o una skill che distribuisce il lavoro, puoi indicare a Claude dove si trova e chiedere un flusso di lavoro che faccia la stessa cosa.

<h3 id="let-claude-decide-with-ultracode">
  Lasciare che Claude decida con ultracode
</h3>

Ultracode è un'impostazione di Claude Code che combina `xhigh` [sforzo di ragionamento](/docs/it/model-config#adjust-effort-level) con orchestrazione automatica del flusso di lavoro. Con essa attiva, Claude pianifica un flusso di lavoro per ogni compito sostanziale invece di aspettare che tu lo chieda.

```text theme={null}
/effort ultracode
```

Per avviare una sessione con ultracode già attivo, avvia con `claude --effort ultracode`. Richiede Claude Code v2.1.203 o successivo.

Con ultracode attivo, Claude decide quando un compito merita un flusso di lavoro. Una singola richiesta può trasformarsi in diversi flussi di lavoro di fila: uno per comprendere il codice, uno per fare il cambiamento e uno per verificarlo. Questo si applica a ogni compito nella sessione, quindi ogni richiesta usa più token e richiede più tempo rispetto ai livelli di sforzo inferiori.

Ultracode dura per la sessione corrente e si ripristina quando inizi una nuova. Torna indietro con `/effort high` quando ritorni al lavoro di routine. È disponibile su modelli che supportano `xhigh` [sforzo](/docs/it/model-config#adjust-effort-level); su altri modelli il menu `/effort` non lo offre.

<h3 id="approve-the-plan-before-it-runs">
  Approvare il piano prima che venga eseguito
</h3>

Nel CLI, il prompt per esecuzione mostra le fasi pianificate e queste opzioni:

* **Sì, eseguilo**: avvia l'esecuzione
* **Sì, e non chiedere di nuovo per `<name>` in `<path>`**: avvia e salta questo prompt per questo flusso di lavoro in questo progetto da ora in poi
* **Visualizza script grezzo**: leggi lo script prima di decidere
* **No**: annulla

`Ctrl+G` apre lo script nel tuo editor. `Tab` ti consente di regolare il prompt prima che l'esecuzione inizi.

Se vedi questo prompt dipende dalla tua [modalità di autorizzazione](/docs/it/permission-modes):

| Modalità di autorizzazione                 | Quando sei richiesto                                                                                                                                                                      |
| :----------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Predefinito, accetta modifiche             | Ogni esecuzione, a meno che tu non abbia selezionato **Sì, e non chiedere di nuovo** per quel flusso di lavoro in questo progetto                                                         |
| Auto                                       | Solo al primo avvio. Qualsiasi **Sì** registra il consenso nelle tue impostazioni utente, e i successivi avvii iniziano senza richiedere. Saltato completamente quando ultracode è attivo |
| Bypass permissions, `claude -p`, Agent SDK | Mai. L'esecuzione inizia immediatamente                                                                                                                                                   |

Nell'app Desktop, una scheda di approvazione mostra il nome del flusso di lavoro, l'elenco delle fasi e un avvertimento di utilizzo dei token, con azioni **Una volta**, **Sempre** e **Nega**. La vista di progresso appare nel riquadro laterale Attività in background.

La tua modalità di autorizzazione controlla solo il prompt di avvio sopra. I subagenti che il flusso di lavoro genera vengono sempre eseguiti in modalità `acceptEdits` e ereditano la tua [lista di autorizzazione degli strumenti](/docs/it/settings#permission-settings), indipendentemente dalla modalità della tua sessione. Le modifiche ai file vengono approvate automaticamente.

I comandi shell, i recuperi web e gli strumenti MCP che non sono nella tua lista di autorizzazione possono comunque richiederti durante l'esecuzione. Per evitare questo su un'esecuzione lunga, aggiungi i comandi di cui gli agenti hanno bisogno alla tua lista di autorizzazione prima di iniziare.

In `claude -p` e nell'Agent SDK non c'è nessuno a cui richiedere, quindi le chiamate di strumenti seguono le tue regole di autorizzazione configurate senza conferma interattiva.

<h3 id="save-the-workflow-for-reuse">
  Salvare il flusso di lavoro per il riutilizzo
</h3>

Quando Claude scrive un flusso di lavoro per un compito che ripeterai, puoi salvare lo script di quell'esecuzione come comando. Un processo come una revisione che esegui su ogni ramo quindi esegue la stessa orchestrazione ogni volta.

Esegui `/workflows`, seleziona l'esecuzione che vuoi mantenere e premi `s`. Nella finestra di dialogo di salvataggio, Tab attiva/disattiva tra i due percorsi di salvataggio:

* `.claude/workflows/` nel tuo progetto: condiviso con chiunque cloni il repo
* `~/.claude/workflows/` nella tua home directory: disponibile in ogni progetto, visibile solo a te. Se imposti [`CLAUDE_CONFIG_DIR`](/docs/it/env-vars), questa posizione è la directory `workflows/` sotto quel percorso.

La finestra di dialogo di salvataggio mostra il percorso risolto per la posizione personale. Prima della v2.1.208, mostrava `~/.claude/workflows/` anche quando `CLAUDE_CONFIG_DIR` era impostato; il file era comunque salvato nella directory configurata.

Premi Invio per salvare. Il flusso di lavoro viene eseguito come `/<name>` nelle future sessioni da entrambi i percorsi.

In un monorepo con diverse directory `.claude/`, puoi mantenere i flussi di lavoro insieme al pacchetto a cui si applicano. A partire dalla v2.1.178, il salvataggio nella posizione del progetto scrive nella directory `.claude/workflows/` più vicina che già esiste tra la tua directory di lavoro e la radice del repository, o nella radice del repository se non ne esiste ancora nessuna. I flussi di lavoro del progetto si caricano anche da ogni `.claude/workflows/` lungo quel percorso, e quando più di uno definisce lo stesso nome Claude Code esegue quello più vicino alla directory di lavoro.

Se un flusso di lavoro di progetto e un flusso di lavoro personale condividono un nome, viene eseguito quello di progetto.

<h3 id="pass-input-to-a-saved-workflow">
  Passare input a un flusso di lavoro salvato
</h3>

Un flusso di lavoro salvato può accettare input attraverso il parametro `args`. Lo script lo legge come una variabile globale denominata `args`. Usa questo per fornire una domanda di ricerca, un elenco di percorsi target o un oggetto di configurazione al momento dell'invocazione invece di modificare lo script per ogni esecuzione.

Il seguente prompt esegue un flusso di lavoro salvato con un elenco di numeri di issue:

```text theme={null}
> Run /triage-issues on issues 1024, 1025, and 1030
```

Claude passa l'elenco come dati strutturati, quindi lo script può chiamare metodi di array e oggetto su `args` direttamente senza analizzarlo prima. Se `args` viene omesso, la variabile globale è `undefined` all'interno dello script.

<h2 id="example-workflow-prompts">
  Esempi di prompt di flusso di lavoro
</h2>

Un flusso di lavoro si adatta meglio quando il compito è più grande di quanto un agente possa tenere in contesto, o quando lo stesso passaggio deve essere eseguito su molti elementi. I prompt seguenti mostrano forme comuni. Ognuno chiede a Claude di scrivere ed eseguire un flusso di lavoro per quel compito; non scrivi lo script tu stesso.

<h3 id="audit-many-files-for-the-same-issue">
  Audit di molti file per lo stesso problema
</h3>

Distribuisci un agente per file, quindi raccogli e verifica i risultati.

```text theme={null}
> use a workflow to audit every route handler under src/routes/ for missing authentication checks, and adversarially verify each finding before reporting it
```

<h3 id="keep-fixing-until-a-check-passes">
  Continua a correggere finché un controllo non passa
</h3>

Esegui un controllo, correggi quello che ha fallito e ripeti finché non passa o smette di fare progressi.

```text theme={null}
> use a workflow to run npx tsc --noEmit and keep fixing the reported errors until the type check passes or two rounds in a row make no progress
```

<h3 id="migrate-many-files-in-parallel">
  Migrare molti file in parallelo
</h3>

Scopri i file da migrare, trasforma ognuno in una copia isolata in modo che le modifiche non entrino in conflitto e verifica ogni risultato.

```text theme={null}
> use a workflow to migrate every component under src/components/ from styled-components to Tailwind, working on each file in its own isolated copy
```

<h3 id="review-every-changed-file-and-write-one-summary">
  Rivedere ogni file modificato e scrivere un riepilogo
</h3>

Esegui un revisore per file, quindi passa tutti i risultati a un agente che li classifica e deduplica.

```text theme={null}
> use a workflow to review every file changed in this PR for correctness issues, then merge the per-file findings into one ranked summary
```

<h3 id="research-a-topic-across-many-sources">
  Ricercare un argomento su molte fonti
</h3>

Distribuisci lettori su changelog, issue e documenti, quindi sintetizza. Il flusso di lavoro `/deep-research` in bundle fa questo; puoi anche descrivere una versione più ristretta.

```text theme={null}
> use a workflow to research how our three competitors handle rate limiting: read their public docs and recent changelog entries in parallel, then compare the approaches
```

<h3 id="find-issues-until-the-list-stops-growing">
  Trovare problemi finché l'elenco non smette di crescere
</h3>

Continua a cercare in round e fermati quando i nuovi round non trovano nulla di nuovo.

```text theme={null}
> use a workflow to find flaky tests in this repo: run the suite repeatedly, record which tests fail intermittently, and stop once two rounds in a row find nothing new
```

<h3 id="what-the-saved-script-looks-like">
  Come appare lo script salvato
</h3>

Quando [salvi un flusso di lavoro](#save-the-workflow-for-reuse), il file in `.claude/workflows/` contiene un blocco `meta` seguito da un corpo di script che orchestra subagenti. Di solito non hai bisogno di modificarlo, ma ecco la forma di uno piccolo in modo che tu possa riconoscere quello che Claude ha generato:

```javascript theme={null}
export const meta = {
  name: 'audit-routes',
  description: 'Audit every route handler for missing auth checks',
}

const found = await agent('List every .ts file under src/routes/.', {
  schema: { type: 'object', required: ['files'], properties: { files: { type: 'array', items: { type: 'string' } } } },
})

const audits = await pipeline(found.files, file =>
  agent(`Audit ${file} for missing authentication checks.`, { label: file }),
)

return audits.filter(Boolean)
```

Il corpo è JavaScript semplice con `await` di livello superiore. `agent()` genera un subagente e `pipeline()` ne esegue uno per elemento in un elenco. Se vuoi modificare uno script a mano, chiedi a Claude di guidarti attraverso il cambiamento, o vedi la voce dello strumento Workflow nel [riferimento dell'Agent SDK](/docs/it/agent-sdk/typescript) per l'insieme completo di opzioni.

<h2 id="how-a-workflow-runs">
  Come viene eseguito un flusso di lavoro
</h2>

Il runtime del flusso di lavoro esegue lo script in un ambiente isolato, separato dalla tua conversazione. I risultati intermedi rimangono nelle variabili dello script invece di finire nel contesto di Claude.

Ogni esecuzione scrive il suo script in un file nella directory della tua sessione in `~/.claude/projects/`. Claude riceve il percorso quando l'esecuzione inizia, quindi puoi chiederglielo. Puoi aprire quel file per leggere l'orchestrazione che Claude ha scritto, confrontarlo con lo script di un'esecuzione precedente, o modificarlo e chiedere a Claude di riavviare dalla versione modificata.

Il runtime traccia il risultato di ogni agente mentre l'esecuzione progredisce, il che è quello che rende un'esecuzione [riprendibile](#resume-after-a-pause) all'interno della stessa sessione.

<h3 id="behavior-and-limits">
  Comportamento e limiti
</h3>

Il runtime applica i seguenti vincoli:

| Vincolo                                                                               | Perché                                                                                                                                                                 |
| :------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Nessun input dell'utente durante l'esecuzione                                         | Solo i prompt di autorizzazione dell'agente possono mettere in pausa un'esecuzione. Per l'approvazione tra le fasi, esegui ogni fase come suo proprio flusso di lavoro |
| Nessun accesso diretto al filesystem o shell dallo script del flusso di lavoro stesso | Gli agenti leggono, scrivono ed eseguono comandi. Lo script coordina gli agenti                                                                                        |
| Fino a 16 agenti concorrenti, meno su macchine con core CPU limitati                  | Limita l'uso delle risorse locali                                                                                                                                      |
| 1.000 agenti totali per esecuzione                                                    | Previene cicli incontrollati                                                                                                                                           |

<h2 id="manage-runs">
  Gestire le esecuzioni
</h2>

Una volta che un'esecuzione inizia, la gestisci dalla vista `/workflows`, o espandendo la sua riga di progresso nel pannello attività sotto la casella di input.

<h3 id="resume-after-a-pause">
  Riprendere dopo una pausa
</h3>

Se fermi un'esecuzione, puoi riprenderla: gli agenti che hanno già completato restituiscono i loro risultati memorizzati nella cache, e il resto viene eseguito dal vivo. Un agente che era ancora in esecuzione quando hai fermato non viene salvato e ricomincia da capo al ripristino, quindi un flusso di lavoro che distribuisce il lavoro su molti piccoli agenti preserva più progresso rispetto a un singolo agente lungo. Riprendi un'esecuzione in pausa da `/workflows` selezionandola e premendo `p`, o chiedi a Claude di riavviare il flusso di lavoro con lo stesso script.

La ripresa funziona all'interno della stessa sessione di Claude Code. Se esci da Claude Code mentre un flusso di lavoro è in esecuzione, la sessione successiva avvia il flusso di lavoro da capo.

<h3 id="cost">
  Costo
</h3>

Un flusso di lavoro genera molti agenti, quindi una singola esecuzione può usare significativamente più token rispetto al lavoro attraverso lo stesso compito in conversazione. Le esecuzioni contano verso l'utilizzo del tuo piano e i limiti di velocità come qualsiasi altra sessione.

Per valutare la spesa prima di impegnarsi in un compito di grandi dimensioni, esegui il flusso di lavoro su una piccola porzione prima: una directory invece dell'intero repository, o una domanda ristretta invece di una ampia. La vista `/workflows` mostra l'utilizzo dei token di ogni agente mentre l'esecuzione progredisce, e puoi fermare l'esecuzione lì in qualsiasi momento senza perdere il lavoro completato. I [limiti degli agenti](#behavior-and-limits) del runtime limitano quanti agenti una singola esecuzione può generare, il che limita il costo di uno script fuori controllo. Per mantenere ogni esecuzione più piccola per impostazione predefinita, [imposta una linea guida sulla dimensione](#set-a-size-guideline) in `/config`.

Claude Code inoltre segnala un'esecuzione che cresce insolitamente. Quando un flusso di lavoro pianifica più di 25 agenti, o il suo totale di token previsto supera 1,5 milioni, la sua riga di progresso nel pannello attività sotto la casella di input mostra un avviso `Large workflow`. L'avviso ti indirizza a [`/workflows`](#watch-the-run), dove puoi fermare l'esecuzione. Richiede Claude Code v2.1.203 o successivo.

L'avviso è consultivo: non mette in pausa o limita l'esecuzione. Due impostazioni cambiano quando lo vedi:

* Se [imposti una linea guida sulla dimensione](#set-a-size-guideline), il numero di agenti della linea guida sostituisce la soglia di 25 agenti.
* Le sessioni con [ultracode](#let-claude-decide-with-ultracode) attivato non mostrano l'avviso, perché attivare ultracode già ti consente di optare per esecuzioni di grandi dimensioni.

Ogni agente in un flusso di lavoro usa il modello della tua sessione a meno che lo script non instrada una fase a uno diverso o la variabile di ambiente [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/it/model-config#environment-variables) sia impostata, che sovrascrive entrambi. Per controllare il costo del modello:

* Controlla `/model` prima di un'esecuzione grande se di solito passi a un modello più piccolo per il lavoro di routine
* Chiedi a Claude di usare un modello più piccolo per le fasi che non hanno bisogno di quello più forte quando descrivi il compito

<h3 id="set-a-size-guideline">
  Imposta una linea guida sulla dimensione
</h3>

L'impostazione Dynamic workflow size in `/config` mantiene i flussi di lavoro che Claude scrive a una scala più piccola per impostazione predefinita. Claude Code invia l'impostazione a Claude come consiglio, quindi un prompt che richiede una scala diversa la sovrascrive comunque. Richiede Claude Code v2.1.202 o successivo.

Ogni valore imposta il numero di agenti a cui Claude mira negli script che scrive.

| Valore         | Guida inviata a Claude                               |
| :------------- | :--------------------------------------------------- |
| `unrestricted` | Nessuna linea guida. Questo è il valore predefinito. |
| `small`        | Mira a meno di 5 agenti.                             |
| `medium`       | Mira a meno di 15 agenti.                            |
| `large`        | Mira a meno di 50 agenti.                            |

Le modifiche hanno effetto al prompt successivo. I [limiti degli agenti del runtime](#behavior-and-limits) si applicano comunque indipendentemente dall'impostazione.

<h3 id="turn-workflows-off">
  Disattivare i flussi di lavoro
</h3>

I flussi di lavoro sono disponibili nel CLI, nell'app Desktop, nelle estensioni IDE, [modalità non interattiva](/docs/it/headless) con `claude -p` e nell'[Agent SDK](/docs/it/agent-sdk/overview). Le stesse impostazioni di disabilitazione si applicano su ogni superficie.

Per disattivare i flussi di lavoro per te:

* Attiva/disattiva Dynamic workflows in `/config`. Persiste tra le sessioni.
* Imposta `"disableWorkflows": true` in `~/.claude/settings.json`. Persiste tra le sessioni.
* Imposta `CLAUDE_CODE_DISABLE_WORKFLOWS=1`. Letto all'avvio, quindi si applica ovunque lo imposti.

Per disattivare i flussi di lavoro per tutta la tua organizzazione, imposta `"disableWorkflows": true` in [impostazioni gestite](/docs/it/server-managed-settings), o usa l'interruttore sulla pagina [impostazioni amministratore di Claude Code](https://claude.ai/admin-settings/claude-code).

Quando i flussi di lavoro sono disabilitati, i comandi di flusso di lavoro in bundle non sono disponibili, la parola chiave `ultracode` non attiva più un'esecuzione e `ultracode` viene rimosso dal menu `/effort`.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Eseguire agenti in parallelo](/docs/it/agents): confrontare subagenti, vista agente, team di agenti e flussi di lavoro
* [Creare subagenti personalizzati](/docs/it/sub-agents): la primitiva worker che i flussi di lavoro orchestrano
* [Gestire i costi](/docs/it/costs): come le esecuzioni multi-agente contano verso i limiti di utilizzo
