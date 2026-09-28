> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Orchestrare subagenti su larga scala con flussi di lavoro dinamici

> I flussi di lavoro dinamici orchestrano molti subagenti da uno script che Claude scrive e che puoi rieseguire. Usali per audit di codebase, migrazioni su larga scala e ricerche con verifica incrociata.

<Note>
  I flussi di lavoro dinamici sono disponibili su tutti i piani a pagamento, con accesso all'API Anthropic, e su Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry. Su Pro, attivali dalla riga Dynamic workflows in `/config`.
</Note>

Un flusso di lavoro dinamico è uno script JavaScript che orchestra molti [subagenti](/docs/it/sub-agents) contemporaneamente. Claude scrive lo script per il compito che descrivi, e un runtime lo esegue in background mentre la tua sessione rimane reattiva.

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

    ```text wrap theme={null}
    /deep-research What changed in the Node.js permission model between v20 and v22?
    ```
  </Step>

  <Step title="Consentire i flussi di lavoro">
    Claude Code chiede se consentire il flusso di lavoro. Seleziona **Sì** per continuare. Il prompt esatto dipende dalla tua modalità di autorizzazione. Vedi [Approvare il piano prima che venga eseguito](#approve-the-plan-before-it-runs) per le opzioni per modalità.
  </Step>

  <Step title="Guardare il progresso">
    L'esecuzione inizia in background. Esegui `/workflows`, usa i tasti freccia per selezionare l'esecuzione e premi Invio per aprire la sua vista di progresso:

    ```text wrap theme={null}
    /workflows
    ```

    La vista mostra ogni fase con il suo conteggio di agenti, totale di token e tempo trascorso. Approfondisci qualsiasi fase per vedere i suoi agenti e cosa ha trovato ognuno. Vedi [Guardare l'esecuzione](#watch-the-run) per l'insieme completo di controlli.

    Puoi anche guardare dal pannello attività sotto la casella di input: un riepilogo di progresso su una riga appare lì mentre l'esecuzione è in corso. Premi la freccia giù per focalizzarlo, quindi Invio per espandere.
  </Step>

  <Step title="Leggere il rapporto">
    Quando l'esecuzione finisce, il rapporto arriva nella tua sessione. Cita le fonti da cui proviene ogni affermazione, con affermazioni che non hanno superato la verifica incrociata già filtrate.

    Quando gli agenti verificatori non riescono a controllare un'affermazione, ad esempio dopo un limite di velocità o un errore API, il rapporto elenca tale affermazione come non verificata invece di contarla come confutata.
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

`/deep-research` viene eseguito solo quando lo richiami.

[I flussi di lavoro che salvi](#save-the-workflow-for-reuse) tu stesso diventano comandi allo stesso modo e appaiono nell'autocompletamento `/` insieme a quelli in bundle.

<h3 id="watch-the-run">
  Guardare l'esecuzione
</h3>

I flussi di lavoro vengono eseguiti in background, quindi la sessione rimane reattiva mentre gli agenti lavorano. Esegui `/workflows` in qualsiasi momento per elencare i flussi di lavoro in esecuzione e completati, quindi selezionane uno per aprire la sua vista di progresso.

La vista di progresso mostra ogni fase con i suoi conteggi di agenti, totali di token e tempo trascorso. Il piè di pagina elenca il tasto per ogni azione:

| Tasto         | Azione                                                                                                                                                                      |
| :------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `↑` / `↓`     | Selezionare una fase o un agente                                                                                                                                            |
| `Invio` o `→` | Approfondire la fase selezionata, quindi in un agente per leggere il suo prompt, le recenti chiamate di strumenti e il risultato. Nel dettaglio, `Invio` espande o comprime |
| `Esc` o `←`   | Tornare indietro di un livello. Nelle versioni da v2.1.203 a v2.1.205, `←` non è tornato indietro da una fase o un agente; usa `Esc` su quelle versioni                     |
| `j` / `k`     | Scorrere all'interno del dettaglio dell'agente quando trabocca                                                                                                              |
| `f`           | Filtrare l'elenco degli agenti nella fase selezionata per stato. Premi di nuovo per ciclare                                                                                 |
| `p`           | Mettere in pausa o riprendere l'esecuzione                                                                                                                                  |
| `x`           | Fermare l'agente selezionato, o fermare l'intero flusso di lavoro quando il focus è sull'esecuzione                                                                         |
| `r`           | Riavviare l'agente in esecuzione selezionato                                                                                                                                |
| `s`           | [Salvare](#save-the-workflow-for-reuse) lo script dell'esecuzione come comando                                                                                              |

Il dettaglio dell'agente elenca il prompt dell'agente, le sue recenti chiamate di strumenti e il suo risultato. Ogni chiamata mostra il suo stato, ad esempio ancora in esecuzione o non riuscito. Quando l'agente mantiene un elenco di attività proprio, il dettaglio lo mostra anche, con lo stato di ogni attività.

Premi `Invio` per espandere il dettaglio. Il prompt e il risultato vengono quindi visualizzati per intero, e ogni chiamata elencata mostra il suo input e l'inizio del suo risultato.

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

Per eseguire un singolo compito come flusso di lavoro senza cambiare il livello di sforzo della sessione, includi la parola chiave `ultracode` nel tuo prompt. Chiedere con le tue stesse parole, ad esempio "usa un flusso di lavoro" o "esegui un flusso di lavoro", funziona anche: Claude tratta una richiesta diretta come lo stesso opt-in.

```text wrap theme={null}
ultracode: audit every API endpoint under src/routes/ for missing auth checks
```

Claude Code evidenzia la parola chiave nel tuo input e Claude scrive uno script di flusso di lavoro per il compito invece di lavorarci turno per turno. La parola chiave sceglie solo come Claude struttura il lavoro: le chiamate agli strumenti degli agenti ricevono gli stessi controlli di autorizzazione e [sandboxing](/docs/it/sandboxing) di qualsiasi altra chiamata agli strumenti nella sessione.

Se l'esecuzione fa quello che volevi, puoi [salvarlo come comando](#save-the-workflow-for-reuse) dopo. Se hai già un orchestrator costruito in un altro modo, come una cartella di prompt di subagent o una skill che distribuisce il lavoro, puoi indicare a Claude dove si trova e chiedere un flusso di lavoro che faccia la stessa cosa.

<h4 id="dismiss-or-turn-off-the-keyword">
  Ignorare o disattivare la parola chiave
</h4>

Se non intendevi avviare un flusso di lavoro, premi `Option+W` su macOS o `Alt+W` su Windows e Linux per ignorare l'evidenziazione per questo prompt, oppure premi backspace mentre il cursore è subito dopo la parola chiave evidenziata. Per impedire che la parola chiave attivi un flusso di lavoro del tutto, disattiva il trigger della parola chiave Ultracode in `/config`.

<h4 id="where-the-keyword-works">
  Dove funziona la parola chiave
</h4>

La parola chiave è un opt-in solo in un prompt che digiti tu stesso: al prompt interattivo, in un pannello dell'estensione IDE, in un client [Remote Control](/docs/it/remote-control), o in un'applicazione Agent SDK che contrassegna l'[`origin`](/docs/it/agent-sdk/typescript#sdkmessageorigin) del tuo input da tastiera come `{ kind: "human" }`. Non avvia un flusso di lavoro quando raggiunge la sessione in un altro modo:

* un prompt passato con `-p`
* un prompt che un'applicazione Agent SDK invia senza contrassegnarlo come input umano
* un prompt di compito programmato
* un payload webhook o un commento di pull request inoltrato nella conversazione

<Note>
  Prima della v2.1.210, la parola chiave avviava un flusso di lavoro da uno qualsiasi di questi percorsi, incluso un payload webhook o un commento di pull request inoltrato nella conversazione.
</Note>

<h3 id="let-claude-decide-with-ultracode">
  Lasciare che Claude decida con ultracode
</h3>

Ultracode è un'impostazione di Claude Code che combina `xhigh` [sforzo di ragionamento](/docs/it/model-config#adjust-effort-level) con orchestrazione automatica del flusso di lavoro. Con essa attiva, Claude pianifica un flusso di lavoro per ogni compito sostanziale invece di aspettare che tu lo chieda.

```text wrap theme={null}
/effort ultracode
```

Per avviare una sessione con ultracode già attivo, avvia con `claude --effort ultracode`. Richiede Claude Code v2.1.203 o successivo.

Per attivarlo mentre scegli un modello, sposta il cursore dello sforzo del selettore `/model` su `ultracode` con i tasti freccia. [Regola il livello di sforzo](/docs/it/model-config#adjust-effort-level) elenca i percorsi che attivano ultracode.

Con ultracode attivo, Claude decide quando un compito merita un flusso di lavoro. Una singola richiesta può trasformarsi in diversi flussi di lavoro di fila: uno per comprendere il codice, uno per fare il cambiamento e uno per verificarlo. Questo si applica a ogni compito nella sessione, quindi ogni richiesta usa più token e richiede più tempo rispetto ai livelli di sforzo inferiori.

`/effort ultracode` dura per la sessione corrente; per avere ogni sessione che inizi con esso, imposta l'impostazione [`ultracode`](/docs/it/settings-reference#ultracode). Torna indietro con `/effort high` quando ritorni al lavoro di routine. Il menu `/effort` lo offre solo [quando ultracode è disponibile](/docs/it/model-config#when-ultracode-is-available).

<h3 id="approve-the-plan-before-it-runs">
  Approvare il piano prima che venga eseguito
</h3>

Nel CLI, il prompt per esecuzione mostra le fasi pianificate e queste opzioni:

* **Sì, eseguilo**: avvia l'esecuzione
* **Sì, e non chiedere di nuovo per `<name>` in `<path>`**: avvia e salta questo prompt per questo flusso di lavoro in questo progetto da ora in poi. Claude Code offre questa opzione quando esegui un flusso di lavoro in bundle, salvato o plugin per nome, non per uno script che Claude ha scritto per il compito corrente.
* **Visualizza script grezzo**: leggi lo script prima di decidere
* **No**: annulla

`Ctrl+G` apre lo script nel tuo editor. `Tab` ti consente di regolare il prompt prima che l'esecuzione inizi.

Se vedi questo prompt dipende dalla tua [modalità di autorizzazione](/docs/it/permission-modes):

| Modalità di autorizzazione | Quando sei richiesto                                                                                                                                                                      |
| :------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Auto                       | Solo al primo avvio. Qualsiasi **Sì** registra il consenso nelle tue impostazioni utente, e i successivi avvii iniziano senza richiedere. Saltato completamente quando ultracode è attivo |
| Manuale, accetta modifiche | Ogni esecuzione, a meno che tu non abbia selezionato **Sì, e non chiedere di nuovo** per quel flusso di lavoro in questo progetto                                                         |
| Bypass permissions         | Claude Code non ti richiede. L'esecuzione inizia immediatamente                                                                                                                           |
| `claude -p`, Agent SDK     | Claude Code non ti richiede                                                                                                                                                               |

In `claude -p` e nell'Agent SDK, Claude Code non mostra mai questo prompt. Esegue la chiamata dello strumento Workflow attraverso la stessa [valutazione delle autorizzazioni](/docs/it/agent-sdk/permissions#how-permissions-are-evaluated) del resto della sessione, quindi le regole di negazione, le regole di richiesta e la modalità `dontAsk` si applicano al lancio come si applicano a ogni chiamata agli strumenti. Per consentire al flusso di lavoro di iniziare in queste esecuzioni, usa uno di questi:

* **Regola di autorizzazione**: `Workflow` nelle tue regole di autorizzazione approva ogni flusso di lavoro, e `Workflow(<name>)` approva un flusso di lavoro salvato per nome.
* **Modalità di autorizzazione automatica**: il [classificatore](/docs/it/permission-modes#eliminate-prompts-with-auto-mode) esamina la chiamata e può approvarla.
* **Modalità bypass permissions**: Claude Code approva la chiamata.
* **Un hook `PreToolUse`**: un [hook](/docs/it/hooks#pretooluse) che restituisce `allow` per la chiamata la approva.
* **Il tuo host**: un [`--permission-prompt-tool`](/docs/it/cli-reference#cli-flags) la approva, o, con l'Agent SDK, un callback [`canUseTool`](/docs/it/agent-sdk/permissions) o un hook [`PermissionRequest`](/docs/it/hooks#permissionrequest) la approva.

Nell'app Desktop, una scheda di approvazione mostra il nome del flusso di lavoro, l'elenco delle fasi e un avvertimento di utilizzo dei token, con azioni **Una volta**, **Sempre** e **Nega**. La vista di progresso appare nel riquadro laterale Attività in background.

I subagenti che il flusso di lavoro genera utilizzano le tue [regole di autorizzazione](/docs/it/settings-reference#permission-settings), e Claude Code sceglie la loro modalità di autorizzazione in base alle regole in [quale modalità di autorizzazione un subagente viene eseguito in](/docs/it/sub-agents#permission-modes). Per evitare richieste su un'esecuzione lunga, aggiungi gli strumenti di cui gli agenti hanno bisogno alle tue regole di autorizzazione prima di iniziare.

<h3 id="save-the-workflow-for-reuse">
  Salvare il flusso di lavoro per il riutilizzo
</h3>

Quando Claude scrive un flusso di lavoro per un compito che ripeterai, puoi salvare lo script di quell'esecuzione come comando. Un processo come una revisione che esegui su ogni ramo quindi esegue la stessa orchestrazione ogni volta.

Esegui `/workflows`, seleziona l'esecuzione che vuoi mantenere e premi `s`. Nella finestra di dialogo di salvataggio, Tab attiva/disattiva tra i due percorsi di salvataggio:

* `.claude/workflows/` nel tuo progetto: condiviso con chiunque cloni il repo
* `~/.claude/workflows/` nella tua home directory: disponibile in ogni progetto, visibile solo a te. Se imposti [`CLAUDE_CONFIG_DIR`](/docs/it/env-vars), questa posizione è la directory `workflows/` sotto quel percorso.

La finestra di dialogo di salvataggio mostra il percorso risolto per la posizione personale.

Premi Invio per salvare. Il flusso di lavoro viene eseguito come `/<name>` nelle future sessioni da entrambi i percorsi.

Claude Code controlla il percorso di salvataggio per i symlink prima di scrivere e mostra un errore invece di scrivere attraverso uno. Quello che controlla dipende da dove salvi:

* Posizione del progetto: Claude Code rifiuta se `.claude`, `.claude/workflows`, o il file di destinazione è un symlink.
* Posizione personale: Claude Code rifiuta solo se il file di destinazione stesso è un symlink, quindi una directory `~/.claude` gestita da uno strumento dotfiles funziona comunque.

Prima della v2.1.216, Claude Code seguiva il link, che potrebbe posizionare il file al di fuori della posizione che hai scelto.

In un monorepo con diverse directory `.claude/`, puoi mantenere i flussi di lavoro insieme al pacchetto a cui si applicano. Il salvataggio nella posizione del progetto scrive nella directory `.claude/workflows/` più vicina che già esiste tra la tua directory di lavoro e la radice del repository, o nella radice del repository se non ne esiste ancora nessuna. I flussi di lavoro del progetto si caricano anche da ogni `.claude/workflows/` lungo quel percorso, e quando più di uno definisce lo stesso nome Claude Code esegue quello più vicino alla directory di lavoro.

Se un flusso di lavoro di progetto e un flusso di lavoro personale condividono un nome, viene eseguito quello di progetto.

<h3 id="distribute-a-workflow-in-a-plugin">
  Distribuire un flusso di lavoro in un plugin
</h3>

Per condividere un flusso di lavoro tra team o repository, includilo in un [plugin](/docs/it/plugins/overview). Posiziona lo script in una directory `workflows/` alla radice del plugin, o punta a una posizione diversa con il [`workflows` campo del manifest](/docs/it/plugins/manifest-reference#fields).

I flussi di lavoro del plugin sono nello spazio dei nomi del nome del plugin. Un plugin chiamato `acme-tools` che contiene uno script il cui `meta.name` è `release-audit` viene eseguito come `/acme-tools:release-audit`.

<h3 id="pass-input-to-a-saved-workflow">
  Passare input a un flusso di lavoro salvato
</h3>

Un flusso di lavoro salvato può accettare input attraverso il parametro `args`. Lo script lo legge come una variabile globale denominata `args`. Usa questo per fornire una domanda di ricerca, un elenco di percorsi target o un oggetto di configurazione al momento dell'invocazione invece di modificare lo script per ogni esecuzione.

Il seguente prompt esegue un flusso di lavoro salvato con un elenco di numeri di issue:

```text wrap theme={null}
Run /triage-issues on issues 1024, 1025, and 1030
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

```text wrap theme={null}
use a workflow to audit every route handler under src/routes/ for missing authentication checks, and adversarially verify each finding before reporting it
```

<h3 id="keep-fixing-until-a-check-passes">
  Continua a correggere finché un controllo non passa
</h3>

Esegui un controllo, correggi quello che ha fallito e ripeti finché non passa o smette di fare progressi.

```text wrap theme={null}
use a workflow to run npx tsc --noEmit and keep fixing the reported errors until the type check passes or two rounds in a row make no progress
```

<h3 id="migrate-many-files-in-parallel">
  Migrare molti file in parallelo
</h3>

Scopri i file da migrare, trasforma ognuno in una copia isolata in modo che le modifiche non entrino in conflitto e verifica ogni risultato.

```text wrap theme={null}
use a workflow to migrate every component under src/components/ from JavaScript to TypeScript, working on each file in its own isolated copy
```

<h3 id="review-every-changed-file-and-write-one-summary">
  Rivedere ogni file modificato e scrivere un riepilogo
</h3>

Esegui un revisore per file, quindi passa tutti i risultati a un agente che li classifica e deduplica.

```text wrap theme={null}
use a workflow to review every file changed in this PR for correctness issues, then merge the per-file findings into one ranked summary
```

<h3 id="research-a-topic-across-many-sources">
  Ricercare un argomento su molte fonti
</h3>

Distribuisci lettori su changelog, issue e documenti, quindi sintetizza. Il flusso di lavoro `/deep-research` in bundle fa questo; puoi anche descrivere una versione più ristretta.

```text wrap theme={null}
use a workflow to research how our three competitors handle rate limiting: read their public docs and recent changelog entries in parallel, then compare the approaches
```

<h3 id="find-issues-until-the-list-stops-growing">
  Trovare problemi finché l'elenco non smette di crescere
</h3>

Continua a cercare in round e fermati quando i nuovi round non trovano nulla di nuovo.

```text wrap theme={null}
use a workflow to find flaky tests in this repo: run the suite repeatedly, record which tests fail intermittently, and stop once two rounds in a row find nothing new
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

Il corpo è JavaScript semplice con `await` di livello superiore. `agent()` genera un subagente, `pipeline()` ne esegue uno per elemento in un elenco, e `parallel()` esegue un insieme di attività di agente contemporaneamente e attende che tutte si completino.

Una chiamata `agent()` si risolve in `null` se la interrompi a metà esecuzione o se raggiunge un errore API irrecuperabile. `pipeline()` mantiene ogni `null` nell'array dei risultati, motivo per cui l'esempio termina con `.filter(Boolean)` per eliminare quelle voci.

In [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode), il prompt che il tuo script passa a `agent()` non conta come una richiesta da te quando il classificatore esamina le azioni di quel subagente, perché Claude Code lo contrassegna come testo che lo script ha calcolato.

Se passi uno `schema` su una chiamata `agent()`, quel subagente restituisce JSON corrispondente alla forma invece di prosa. Claude Code verifica lo schema prima di avviare il subagente: quando può provare che lo schema contraddice se stesso, la chiamata fallisce con un errore che nomina la contraddizione, e il subagente non viene mai avviato. Una contraddizione che può provare è una chiave `required` che `additionalProperties: false` esclude.

Se l'output del subagente non supera ancora la convalida dopo cinque tentativi, la chiamata fallisce con un errore che include l'ultimo errore di convalida. Per modificare il numero di tentativi, imposta [`MAX_STRUCTURED_OUTPUT_RETRIES`](/docs/it/env-vars).

<h3 id="edit-a-saved-script">
  Modifica uno script salvato
</h3>

Per modificare un [flusso di lavoro che hai salvato](#save-the-workflow-for-reuse), modifica il suo file `.js` o chiedi a Claude di apportare la modifica. Prima di modificare o chiedere, esegui la [skill in bundle](/docs/it/skills#bundled-skills) `/workflow-authoring` per caricare il riferimento di scrittura di script da cui Claude lavora. La skill richiede Claude Code v2.1.248 o successivo.

Per eseguire la versione modificata nella sessione corrente, esegui [`/reload-skills`](/docs/it/commands#all-commands) per rileggere le directory dei flussi di lavoro, quindi esegui di nuovo `/<name>`.

Claude Code applica queste regole a ogni parte del file quando carica ed esegue lo script:

* **Blocco `meta`**: mantieni `export const meta` come prima istruzione e mantienilo un oggetto letterale semplice con un `name` e una `description`. Se contiene qualcosa di diverso da valori letterali, come una variabile, una chiamata di funzione o uno spread, Claude Code elimina `/<name>` dall'autocompletamento di `/`.
* **Corpo**: oltre a `agent()`, `pipeline()` e `parallel()`, puoi chiamare `phase()` per raggruppare gli agenti che seguono sotto un titolo nella vista di avanzamento, chiamare `log()` per mostrare un messaggio sopra le fasi, e leggere il global [`args`](#pass-input-to-a-saved-workflow). Se il corpo ha un errore di sintassi, Claude Code lo segnala quando esegui il flusso di lavoro.
* **`phases`**: se le elenca in `meta`, dai a ogni voce esattamente il titolo che passi a `phase()`. Un titolo `phase()` senza voce ottiene il suo proprio gruppo di avanzamento.
* **Timestamp e casualità**: Claude Code fa sì che `Date.now()`, `Math.random()` e un `new Date()` senza argomenti lancino un'eccezione all'interno dello script, in modo che un'[esecuzione rilanciata](#resume-after-a-pause) ripeta le stesse chiamate `agent()`. Passa invece un timestamp attraverso `args`.

Puoi anche modificare [lo script di una singola esecuzione](#how-a-workflow-runs) piuttosto che la copia salvata. [Riprendi dopo una pausa](#resume-after-a-pause) copre quali agenti vengono eseguiti di nuovo quando riavvii uno script modificato. Per gli input dello strumento Workflow, vedi la sua voce nel [riferimento dell'Agent SDK](/docs/it/agent-sdk/typescript#workflow).

<h2 id="how-a-workflow-runs">
  Come viene eseguito un flusso di lavoro
</h2>

Il runtime del flusso di lavoro esegue lo script in un ambiente isolato, separato dalla tua conversazione. I risultati intermedi rimangono nelle variabili dello script invece di finire nel contesto di Claude.

Ogni esecuzione scrive il suo script in un file nella directory della tua sessione in `~/.claude/projects/`. Claude riceve il percorso quando l'esecuzione inizia, quindi puoi chiederglielo. Puoi aprire quel file per leggere l'orchestrazione che Claude ha scritto, confrontarlo con lo script di un'esecuzione precedente, o modificarlo e chiedere a Claude di riavviare dalla versione modificata.

Claude può avviare un flusso di lavoro solo da un file di script che la sessione è già autorizzata a leggere. Per eseguire uno script mantenuto al di fuori della tua directory di lavoro, aggiungi prima la sua directory con [`/add-dir`](/docs/it/permissions#working-directories) o una [regola di autorizzazione Read](/docs/it/permissions#read-and-edit).

Il runtime traccia il risultato di ogni agente mentre l'esecuzione progredisce, il che è quello che rende un'esecuzione [riprendibile](#resume-after-a-pause) all'interno della stessa sessione.

<h3 id="prompt-caching-in-a-fan-out">
  Prompt caching in un fan-out
</h3>

Gli agenti nella stessa esecuzione possono leggere la [prompt cache](/docs/it/prompt-caching#subagents-and-the-cache) l'uno dell'altro. Due agenti che vengono eseguiti con lo stesso modello, livello di sforzo, tipo di agente, strumenti, schema di output e directory di lavoro costruiscono lo stesso prefisso di strumenti e prompt di sistema, quindi un agente che inizia dopo che la risposta di un fratello corrispondente ha iniziato legge la cache di quel fratello nella sua prima richiesta.

Le richieste di un agente del flusso di lavoro rientrano al di fuori del bucket [cache TTL](/docs/it/prompt-caching#which-ttl-each-request-gets) della conversazione principale, quindi la sua cache dura cinque minuti per impostazione predefinita, incluso su un abbonamento Claude. Per mantenerla per un'ora, imposta [`subagentPromptCacheTtl`](/docs/it/settings-reference#subagentpromptcachettl) su `1h`. L'API fattura le scritture della cache di 1 ora a una tariffa più elevata.

Quando un fan-out avvia diversi agenti corrispondenti contemporaneamente, Claude Code tiene tutti tranne il primo fino a quando la risposta del primo agente non inizia, quindi rilascia gli agenti trattenuti insieme in modo che le loro prime richieste leggano il prefisso condiviso invece di elaborarlo ciascuno senza cache. Claude Code limita la sospensione a [`CLAUDE_CODE_WORKFLOW_PREFIX_STAGGER_MS`](/docs/it/env-vars) millisecondi, `5000` per impostazione predefinita. Impostalo su `0` per disabilitare la sospensione.

<h3 id="behavior-and-limits">
  Comportamento e limiti
</h3>

Il runtime applica i seguenti vincoli:

| Vincolo                                                                                                                                                                                                                                                                                                                                                    | Perché                                                                                                                                                                                                                                      |
| :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Nessun input dell'utente durante l'esecuzione                                                                                                                                                                                                                                                                                                              | Un'esecuzione si mette in pausa solo per i prompt di autorizzazione dell'agente e un'[attesa del limite di utilizzo](#when-a-run-hits-your-usage-limit). Per l'approvazione tra le fasi, esegui ogni fase come suo proprio flusso di lavoro |
| Nessun accesso diretto al filesystem o shell dallo script del flusso di lavoro stesso                                                                                                                                                                                                                                                                      | Gli agenti leggono, scrivono ed eseguono comandi. Lo script coordina gli agenti                                                                                                                                                             |
| Nessun caricamento di moduli: uno script che contiene `import()` non riesce prima dell'inizio dell'esecuzione                                                                                                                                                                                                                                              | Il corpo dello script è JavaScript semplice. Metti il lavoro che necessita di una libreria nel compito di un agente                                                                                                                         |
| Fino a 16 agenti concorrenti per impostazione predefinita, meno quando Claude Code ha meno CPU disponibili, incluso all'interno di un contenitore con limitazione di CPU. Per modificare il limite, imposta [`CLAUDE_CODE_WORKFLOW_MAX_CONCURRENT_AGENTS`](/docs/it/env-vars#variables) su un valore da 1 a 256, che richiede Claude Code v2.1.269 o successivo | Limita l'uso delle risorse locali                                                                                                                                                                                                           |
| In un fan-out, gli agenti che condividono il prefisso della prompt-cache del primo agente si avviano fino a 5 secondi dopo per impostazione predefinita                                                                                                                                                                                                    | Tutti tranne il primo leggono il [prefisso che il primo agente ha memorizzato nella cache](#prompt-caching-in-a-fan-out) invece di elaborarlo ciascuno senza cache                                                                          |
| Fino a 4.096 elementi in una singola chiamata `parallel()` o `pipeline()`: il runtime rifiuta un elenco più lungo con un errore                                                                                                                                                                                                                            | Un limite silenzioso farebbe cadere parte del carico di lavoro senza dirlo allo script                                                                                                                                                      |
| 1.000 agenti totali per esecuzione                                                                                                                                                                                                                                                                                                                         | Previene cicli incontrollati                                                                                                                                                                                                                |

<h2 id="manage-runs">
  Gestire le esecuzioni
</h2>

Una volta avviata un'esecuzione, la gestisci dalla vista `/workflows`, oppure espandendo la sua linea di progresso nel pannello attività sotto la casella di input.

Quando interrompi un'esecuzione, rimane nel pannello attività finché uno qualsiasi dei processi dei suoi agenti è ancora in esecuzione. Se la interrompi di nuovo, Claude Code invia nuovamente i segnali a quei processi.

<h3 id="resume-after-a-pause">
  Riprendere dopo una pausa
</h3>

Riprendi un'esecuzione in pausa da `/workflows` selezionandola e premendo `p`. Per un'esecuzione che hai interrotto, chiedi a Claude di riavviare il workflow con lo stesso script. Se gli agenti dell'esecuzione interrotta non sono ancora usciti, Claude Code rifiuta il riavvio finché non lo fanno, in modo che una seconda copia di quegli agenti non possa essere eseguita insieme a loro.

Claude Code riproduce l'esecuzione nell'ordine in cui gli agenti sono stati avviati, e ogni agente restituisce il suo risultato salvato oppure viene eseguito di nuovo:

* **Completato**: restituisce il suo risultato salvato. Il primo agente il cui prompt differisce dall'esecuzione precedente, perché hai modificato lo script o un agente precedente ha restituito qualcosa di diverso, viene eseguito di nuovo, così come ogni agente dopo di esso, anche quelli che erano completati.
* **Ancora in esecuzione quando hai interrotto**: ricomincia. Interrompere l'intera esecuzione non conta alcun agente come non riuscito.
* **Non riuscito**: viene eseguito di nuovo, così come ogni agente che è stato avviato dopo di esso, anche quelli che erano completati. Interrompere un solo agente, selezionandolo in [`/workflows`](#watch-the-run) e premendo `x`, conta come non riuscito.

Quest'ultimo caso significa che un errore nel mezzo di un fan-out riesegue il lavoro che era già terminato. Se uno script avvia A, B, C e D in quell'ordine e B non riesce, il riavvio restituisce A dalla cache ed esegue di nuovo B, C e D.

Puoi riprendere un'esecuzione all'interno della stessa sessione di Claude Code. Quello che accade a un workflow in esecuzione quando lasci la sessione dipende da come la lasci:

* Se [metti la sessione in background](/docs/it/agent-view#what-carries-over-when-you-background), Claude Code riproduce l'esecuzione allo stesso modo nella sessione in background e la continua.
* Se esci da Claude Code mentre un workflow è in esecuzione e [agent view è attivo](/docs/it/agent-view#from-inside-a-session), la finestra di dialogo di uscita offre `Move to background and exit`, che trasporta l'esecuzione allo stesso modo. Se scegli invece `Exit and stop tasks`, o l'opzione non è offerta, l'esecuzione si interrompe con la sessione. Claude Code mantiene i risultati salvati dell'esecuzione nella directory di quella sessione in `~/.claude/projects/`, quindi una sessione che riprendi con `claude --resume` può riprodurli quando chiedi a Claude di riavviare il workflow. In una sessione che avvii da zero, Claude non ha un'esecuzione precedente da riavviare e avvia il workflow da capo come una nuova esecuzione.

In una [sessione cloud](/docs/it/claude-code-on-the-web), Claude Code salva anche i risultati dell'esecuzione con la cronologia della conversazione della sessione, che sopravvive quando la VM della sessione viene recuperata. Quando [riapri una tale sessione](/docs/it/claude-code-on-the-web#environment-expired) e chiedi a Claude di riavviare il workflow, gli agenti completati restituiscono ancora i loro risultati salvati.

Nelle sessioni locali e cloud allo stesso modo, quando Claude riavvia un'esecuzione precedente e Claude Code non riesce a trovare affatto i risultati salvati di quell'esecuzione, il riavvio non riesce con un errore `nothing to resume` invece di avviare l'esecuzione da capo da solo. Chiedi a Claude di avviare il workflow da capo come una nuova esecuzione.

<h3 id="when-a-run-hits-your-usage-limit">
  Quando un'esecuzione raggiunge il tuo limite di utilizzo
</h3>

Quando un agente raggiunge il tuo limite di utilizzo [usage limit](/docs/it/interactive-mode#wait-for-a-usage-limit-to-reset) su claude.ai, l'esecuzione si mette in pausa piuttosto che far fallire quell'agente: gli agenti che hanno raggiunto il limite aspettano il ripristino, e nessun nuovo agente si avvia. Poco dopo che il limite si ripristina, gli agenti in attesa vengono eseguiti di nuovo e l'esecuzione continua da sola. Richiede Claude Code v2.1.271 o successivo; nelle versioni precedenti, gli agenti interessati falliscono.

Mentre l'esecuzione aspetta, la sua linea di progresso nel pannello attività e l'intestazione [`/workflows`](#watch-the-run) mostrano quando il limite si ripristina.

L'esecuzione si mette in pausa solo quando tutti questi elementi sono veri; quando uno non lo è, l'agente interessato fallisce invece:

* La sessione è interattiva e accedi con un abbonamento claude.ai. Un'esecuzione non si mette in pausa nella [modalità non interattiva](/docs/it/headless) con `claude -p` o nell'[Agent SDK](/docs/it/agent-sdk/overview), in una [sessione in background](/docs/it/agent-view), o in una sessione di compagno [Remote Control](/docs/it/remote-control) o [agent team](/docs/it/agent-teams).
* [`autoContinueAtUsageLimit`](/docs/it/settings-reference#autocontinueatusagelimit) è attivo, la stessa impostazione che consente alla sessione stessa di [aspettare il ripristino di un limite di utilizzo](/docs/it/interactive-mode#wait-for-a-usage-limit-to-reset). Se lo disattivi durante un'attesa, l'attesa termina e gli agenti in attesa falliscono.
* Il limite si ripristina entro 24 ore. Un limite settimanale può ripristinarsi più avanti.
* L'esecuzione non ha già aspettato due volte. Quando raggiunge il limite una terza volta, l'agente fallisce.

<h3 id="cost">
  Costo
</h3>

Un workflow genera molti agenti, quindi una singola esecuzione può utilizzare significativamente più token rispetto al lavoro attraverso lo stesso compito in conversazione. Le esecuzioni contano verso l'utilizzo del tuo piano e i limiti di velocità.

Per valutare la spesa prima di impegnarsi in un compito di grandi dimensioni, esegui il workflow su una piccola porzione per prima: una directory invece dell'intero repository, o una domanda ristretta invece di una ampia. La vista `/workflows` mostra l'utilizzo dei token di ogni agente mentre l'esecuzione progredisce, e puoi interrompere l'esecuzione lì in qualsiasi momento, solitamente senza perdere il lavoro completato. [Riprendere dopo una pausa](#resume-after-a-pause) copre quello che un'esecuzione interrotta mantiene. I [limiti degli agenti](#behavior-and-limits) del runtime limitano quanti agenti una singola esecuzione può generare, il che limita il costo di uno script fuori controllo. Per mantenere le esecuzioni a meno agenti, scegli la linea guida di dimensione `small` [](#set-a-size-guideline).

Claude Code contrassegna anche un'esecuzione che cresce insolitamente grande. Quando un workflow pianifica più di 25 agenti, o il suo totale di token previsto supera 1,5 milioni, la sua linea di progresso nel pannello attività sotto la casella di input mostra un avviso `Large workflow`. L'avviso ti indirizza a [`/workflows`](#watch-the-run), dove puoi interrompere l'esecuzione.

L'avviso è consultivo: non mette in pausa o limita l'esecuzione. Due impostazioni cambiano quando lo vedi:

* Se scegli una [linea guida di dimensione](#set-a-size-guideline) tu stesso, il suo conteggio di agenti sostituisce la soglia di 25 agenti. La linea guida predefinita incorporata lascia la soglia a 25.
* Le sessioni con [ultracode](#let-claude-decide-with-ultracode) attivo non mostrano l'avviso, perché attivare ultracode già ti consente di optare per esecuzioni di grandi dimensioni.

Claude Code sceglie il modello di ogni agente del workflow nello stesso [ordine che utilizza per i subagenti](/docs/it/sub-agents#choose-a-model). Un modello che lo script nomina per una fase conta come il modello per invocazione in quell'ordine. Quando nient'altro ne assegna uno, l'agente viene eseguito sul modello della tua sessione.

Per controllare il costo del modello:

* Controlla `/model` prima di un'esecuzione di grandi dimensioni se di solito passi a un modello più piccolo per il lavoro di routine
* Chiedi a Claude di utilizzare un modello più piccolo per le fasi che non hanno bisogno di quello più forte quando descrivi il compito

Quando la lista di autorizzazione [`availableModels`](/docs/it/model-config#restrict-model-selection) della tua organizzazione blocca un modello che lo script richiede per un agente, quell'agente viene eseguito su un modello sostituito invece, seguendo le stesse [regole di sostituzione dei subagenti](/docs/it/sub-agents#choose-a-model). La vista di progresso dell'esecuzione in [`/workflows`](#watch-the-run) mostra un avviso che nomina sia i modelli richiesti che quelli sostituiti.

<h3 id="set-a-size-guideline">
  Impostare una linea guida di dimensione
</h3>

Una linea guida di dimensione dice a Claude quanti agenti mirare quando scrive un workflow dinamico. Claude Code invia la linea guida a Claude come consiglio, non come limite, quindi un prompt che richiede una scala diversa la sostituisce comunque. Richiede Claude Code v2.1.202 o successivo.

Ogni valore corrisponde a un conteggio di agenti:

| Valore         | Conteggio di agenti a cui Claude mira                         |
| :------------- | :------------------------------------------------------------ |
| `unrestricted` | Nessuna linea guida: Claude dimensiona il workflow al compito |
| `small`        | Meno di 5 agenti                                              |
| `medium`       | Meno di 10 agenti                                             |
| `large`        | Meno di 50 agenti                                             |

L'impostazione predefinita è `medium`, oppure `small` quando accedi con un piano Pro e Claude Code v2.1.271 o successivo. Finché non scegli un valore, la riga `/config` mostra il valore come predefinito, e la linea `Running in background` del workflow nomina la dimensione in vigore. Richiede Claude Code v2.1.219 o successivo; le versioni precedenti hanno come impostazione predefinita `unrestricted`.

Per modificare la linea guida, scegli un valore per l'impostazione Dynamic workflow size in `/config`, oppure esegui `/config workflowSizeGuideline=small`. Su v2.1.219 e successivo, puoi anche impostare la chiave [`workflowSizeGuideline`](/docs/it/settings-reference#workflowsizeguideline) in qualsiasi file di impostazioni; quel valore ha la precedenza su `/config`, e Claude Code nasconde la riga `/config` mentre un file di impostazioni ne fornisce uno.

Le modifiche hanno effetto al prompt successivo. I [limiti degli agenti del runtime](#behavior-and-limits) si applicano comunque indipendentemente dall'impostazione.

<h3 id="turn-workflows-off">
  Disattivare i workflow
</h3>

I workflow sono disponibili nella CLI, nell'app Desktop, nelle estensioni IDE, nella [modalità non interattiva](/docs/it/headless) con `claude -p`, e nell'[Agent SDK](/docs/it/agent-sdk/overview). Le stesse impostazioni di disabilitazione si applicano su ogni superficie.

Per disattivare i workflow per te stesso:

* Disattiva Dynamic workflows in `/config`. Persiste tra le sessioni.
* Imposta `"disableWorkflows": true` in `~/.claude/settings.json`. Persiste tra le sessioni.
* Imposta `CLAUDE_CODE_DISABLE_WORKFLOWS=1`. Letto all'avvio, quindi si applica ovunque lo imposti.

Per disattivare i workflow per l'intera organizzazione, imposta `"disableWorkflows": true` nelle [impostazioni gestite](/docs/it/server-managed-settings), oppure utilizza l'interruttore nella pagina [impostazioni admin di Claude Code](https://claude.ai/admin-settings/claude-code).

Quando i workflow sono disabilitati, i comandi workflow in bundle e la skill `/workflow-authoring` non sono disponibili, la parola chiave `ultracode` non attiva più un'esecuzione, e `ultracode` viene rimosso dal menu `/effort`.

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Eseguire agenti in parallelo](/docs/it/agents): confrontare subagenti, vista agente, team di agenti e flussi di lavoro
* [Creare subagenti personalizzati](/docs/it/sub-agents): la primitiva worker che i flussi di lavoro orchestrano
* [Gestire i costi](/docs/it/costs): come le esecuzioni multi-agente contano verso i limiti di utilizzo
