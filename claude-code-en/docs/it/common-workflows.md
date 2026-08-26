> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Flussi di lavoro comuni

> Guide passo dopo passo per esplorare basi di codice, correggere bug, effettuare refactoring, testare e altri compiti quotidiani con Claude Code.

Questa pagina raccoglie brevi ricette per lo sviluppo quotidiano. Per una guida di livello superiore sulla formulazione di prompt e la gestione del contesto, vedi [Best practices](/docs/it/best-practices).

Questa pagina copre:

* [Ricette di prompt](#prompt-recipes) per esplorare il codice, correggere bug, effettuare refactoring, testare, PR e documentazione
* [Riprendere conversazioni precedenti](#resume-previous-conversations) in modo che un'attività possa durare più sessioni
* [Eseguire sessioni parallele con worktrees](#run-parallel-sessions-with-worktrees) in modo che le modifiche simultanee non si scontrino
* [Pianificare prima di modificare](#plan-before-editing) per rivedere le modifiche prima che tocchino il disco
* [Delegare la ricerca ai subagent](#delegate-research-to-subagents) per mantenere il tuo contesto principale pulito
* [Inviare Claude negli script](#pipe-claude-into-scripts) per CI e elaborazione batch

<h2 id="prompt-recipes">
  Ricette di prompt
</h2>

Questi sono modelli di prompt per compiti quotidiani come esplorare codice non familiare, eseguire il debug, effettuare refactoring, scrivere test e creare PR. Ognuno funziona su qualsiasi superficie di Claude Code; adatta la formulazione al tuo progetto.

<h3 id="understand-new-codebases">
  Comprendere nuove basi di codice
</h3>

Per configurare Claude Code in un monorepo o in una base di codice di grandi dimensioni, vedi [Monorepos e repository di grandi dimensioni](/docs/it/large-codebases).

<h4 id="get-a-quick-codebase-overview">
  Ottenere una rapida panoramica della base di codice
</h4>

Supponiamo che tu abbia appena aderito a un nuovo progetto e debba comprendere rapidamente la sua struttura.

<Steps>
  <Step title="Navigare alla directory radice del progetto">
    ```bash theme={null}
    cd /path/to/project 
    ```
  </Step>

  <Step title="Avviare Claude Code">
    ```bash theme={null}
    claude 
    ```
  </Step>

  <Step title="Chiedere una panoramica di alto livello">
    ```text theme={null}
    give me an overview of this codebase
    ```
  </Step>

  <Step title="Approfondire componenti specifici">
    ```text theme={null}
    explain the main architecture patterns used here
    ```

    ```text theme={null}
    what are the key data models?
    ```

    ```text theme={null}
    how is authentication handled?
    ```
  </Step>
</Steps>

<Tip>
  Suggerimenti:

  * Inizia con domande ampie, quindi restringi a aree specifiche
  * Chiedi informazioni sulle convenzioni di codifica e sui modelli utilizzati nel progetto
  * Richiedi un glossario di termini specifici del progetto
</Tip>

<h4 id="find-relevant-code">
  Trovare codice rilevante
</h4>

Supponiamo che tu debba individuare il codice relativo a una funzionalità o funzione specifica.

<Steps>
  <Step title="Chiedere a Claude di trovare file rilevanti">
    ```text theme={null}
    find the files that handle user authentication
    ```
  </Step>

  <Step title="Ottenere contesto su come i componenti interagiscono">
    ```text theme={null}
    how do these authentication files work together?
    ```
  </Step>

  <Step title="Comprendere il flusso di esecuzione">
    ```text theme={null}
    trace the login process from front-end to database
    ```
  </Step>
</Steps>

<Tip>
  Suggerimenti:

  * Sii specifico su ciò che stai cercando
  * Usa il linguaggio del dominio dal progetto
  * Installa un [plugin di code intelligence](/docs/it/discover-plugins#code-intelligence) per il tuo linguaggio per dare a Claude una navigazione precisa "go to definition" e "find references"
</Tip>

***

<h3 id="fix-bugs-efficiently">
  Correggere bug in modo efficiente
</h3>

Supponiamo che tu abbia riscontrato un messaggio di errore e debba trovare e correggere la sua fonte.

<Steps>
  <Step title="Condividere l'errore con Claude">
    ```text theme={null}
    I'm seeing an error when I run npm test
    ```
  </Step>

  <Step title="Chiedere raccomandazioni per la correzione">
    ```text theme={null}
    suggest a few ways to fix the @ts-ignore in user.ts
    ```
  </Step>

  <Step title="Applicare la correzione">
    ```text theme={null}
    update user.ts to add the null check you suggested
    ```
  </Step>
</Steps>

<Tip>
  Suggerimenti:

  * Comunica a Claude il comando per riprodurre il problema e ottenere una stack trace
  * Menziona eventuali passaggi per riprodurre l'errore
  * Fai sapere a Claude se l'errore è intermittente o coerente
</Tip>

***

<h3 id="refactor-code">
  Effettuare refactoring del codice
</h3>

Supponiamo che tu debba aggiornare il codice precedente per utilizzare modelli e pratiche moderne.

<Steps>
  <Step title="Identificare il codice legacy per il refactoring">
    ```text theme={null}
    find deprecated API usage in our codebase
    ```
  </Step>

  <Step title="Ottenere raccomandazioni per il refactoring">
    ```text theme={null}
    suggest how to refactor utils.js to use modern JavaScript features
    ```
  </Step>

  <Step title="Applicare le modifiche in modo sicuro">
    ```text theme={null}
    refactor utils.js to use ES2024 features while maintaining the same behavior
    ```
  </Step>

  <Step title="Verificare il refactoring">
    ```text theme={null}
    run tests for the refactored code
    ```
  </Step>
</Steps>

<Tip>
  Suggerimenti:

  * Chiedi a Claude di spiegare i vantaggi dell'approccio moderno
  * Richiedi che le modifiche mantengano la compatibilità all'indietro quando necessario
  * Effettua il refactoring in piccoli incrementi testabili
</Tip>

***

<h3 id="work-with-tests">
  Lavorare con i test
</h3>

Supponiamo che tu debba aggiungere test per il codice non coperto.

<Steps>
  <Step title="Identificare il codice non testato">
    ```text theme={null}
    find functions in NotificationsService.swift that are not covered by tests
    ```
  </Step>

  <Step title="Generare lo scaffolding dei test">
    ```text theme={null}
    add tests for the notification service
    ```
  </Step>

  <Step title="Aggiungere casi di test significativi">
    ```text theme={null}
    add test cases for edge conditions in the notification service
    ```
  </Step>

  <Step title="Eseguire e verificare i test">
    ```text theme={null}
    run the new tests and fix any failures
    ```
  </Step>
</Steps>

Claude può generare test che seguono i modelli e le convenzioni esistenti del tuo progetto. Quando chiedi test, sii specifico sul comportamento che desideri verificare. Claude esamina i tuoi file di test esistenti per abbinare lo stile, i framework e i modelli di asserzione già in uso.

Per una copertura completa, chiedi a Claude di identificare i casi limite che potresti aver perso. Claude può analizzare i tuoi percorsi di codice e suggerire test per condizioni di errore, valori limite e input inaspettati che sono facili da trascurare.

***

<h3 id="create-pull-requests">
  Creare pull request
</h3>

Puoi creare pull request chiedendo direttamente a Claude ("create a pr for my changes"), oppure guidare Claude attraverso i passaggi:

<Steps>
  <Step title="Riassumere le tue modifiche">
    ```text theme={null}
    summarize the changes I've made to the authentication module
    ```
  </Step>

  <Step title="Generare una pull request">
    ```text theme={null}
    create a pr
    ```
  </Step>

  <Step title="Rivedere e affinare">
    ```text theme={null}
    enhance the PR description with more context about the security improvements
    ```
  </Step>
</Steps>

Quando crei una PR utilizzando `gh pr create`, la sessione viene automaticamente collegata a quella PR. Per tornare a essa in seguito, esegui `claude --from-pr 123`, sostituendo 123 con il numero della PR, o incolla l'URL della PR nel selettore [`/resume`](/docs/it/sessions#use-the-session-picker).

<Tip>
  Rivedi la PR generata da Claude prima di inviarla e chiedi a Claude di evidenziare i rischi potenziali o le considerazioni.
</Tip>

<h3 id="handle-documentation">
  Gestire la documentazione
</h3>

Supponiamo che tu debba aggiungere o aggiornare la documentazione per il tuo codice.

<Steps>
  <Step title="Identificare il codice non documentato">
    ```text theme={null}
    find functions without proper JSDoc comments in the auth module
    ```
  </Step>

  <Step title="Generare la documentazione">
    ```text theme={null}
    add JSDoc comments to the undocumented functions in auth.js
    ```
  </Step>

  <Step title="Rivedere e migliorare">
    ```text theme={null}
    improve the generated documentation with more context and examples
    ```
  </Step>

  <Step title="Verificare la documentazione">
    ```text theme={null}
    check if the documentation follows our project standards
    ```
  </Step>
</Steps>

<Tip>
  Suggerimenti:

  * Specifica lo stile di documentazione che desideri (JSDoc, docstrings, ecc.)
  * Chiedi esempi nella documentazione
  * Richiedi documentazione per API pubbliche, interfacce e logica complessa
</Tip>

***

<h3 id="work-in-notes-and-non-code-folders">
  Lavorare in note e cartelle non di codice
</h3>

Claude Code funziona in qualsiasi directory. Eseguilo all'interno di un vault di note, una cartella di documentazione o qualsiasi raccolta di file markdown per cercare, modificare e riorganizzare il contenuto nello stesso modo in cui faresti con il codice.

La directory `.claude/` e `CLAUDE.md` si trovano insieme alle directory di configurazione di altri strumenti senza conflitti. Claude legge i file freschi ad ogni chiamata dello strumento, quindi vede le modifiche che fai in un'altra applicazione la prossima volta che legge quel file.

***

<h3 id="work-with-images">
  Lavorare con le immagini
</h3>

Supponiamo che tu debba lavorare con immagini nella tua base di codice e desideri l'aiuto di Claude nell'analizzare il contenuto dell'immagine.

<Steps>
  <Step title="Aggiungere un'immagine alla conversazione">
    Puoi utilizzare uno di questi metodi:

    1. Trascina e rilascia un'immagine nella finestra di Claude Code
    2. Copia un'immagine e incollala nella CLI con Ctrl+V. Su macOS, Cmd+V funziona anche in iTerm2.
    3. Fornisci un percorso di immagine a Claude. Ad esempio, "Analyze this image: /path/to/your/image.png"
  </Step>

  <Step title="Chiedere a Claude di analizzare l'immagine">
    ```text theme={null}
    What does this image show?
    ```

    ```text theme={null}
    Describe the UI elements in this screenshot
    ```

    ```text theme={null}
    Are there any problematic elements in this diagram?
    ```
  </Step>

  <Step title="Utilizzare le immagini per il contesto">
    ```text theme={null}
    Here's a screenshot of the error. What's causing it?
    ```

    ```text theme={null}
    This is our current database schema. How should we modify it for the new feature?
    ```
  </Step>

  <Step title="Ottenere suggerimenti di codice dal contenuto visivo">
    ```text theme={null}
    Generate CSS to match this design mockup
    ```

    ```text theme={null}
    What HTML structure would recreate this component?
    ```
  </Step>
</Steps>

<Tip>
  Suggerimenti:

  * Usa le immagini quando le descrizioni di testo sarebbero poco chiare o ingombranti
  * Includi screenshot di errori, design dell'interfaccia utente o diagrammi per un contesto migliore
  * Puoi lavorare con più immagini in una conversazione
  * L'analisi delle immagini funziona con diagrammi, screenshot, mockup e altro
  * Quando Claude fa riferimento a immagini (ad esempio, `[Image #1]`), `Cmd+Click` (Mac) o `Ctrl+Click` (Windows/Linux) il collegamento per aprire l'immagine nel tuo visualizzatore predefinito
</Tip>

***

<h3 id="reference-files-and-directories">
  Fare riferimento a file e directory
</h3>

Usa @ per includere rapidamente file o directory senza aspettare che Claude li legga.

<Steps>
  <Step title="Fare riferimento a un singolo file">
    ```text theme={null}
    Explain the logic in @src/utils/auth.js
    ```

    Questo include il contenuto completo del file nella conversazione.
  </Step>

  <Step title="Fare riferimento a una directory">
    ```text theme={null}
    What's the structure of @src/components?
    ```

    Questo fornisce un elenco di directory con informazioni sui file.
  </Step>

  <Step title="Fare riferimento alle risorse MCP">
    ```text theme={null}
    Show me the data from @github:repos/owner/repo/issues
    ```

    Questo recupera i dati dai server MCP connessi utilizzando il formato @server:resource. Vedi [risorse MCP](/docs/it/mcp#use-mcp-resources) per i dettagli.
  </Step>
</Steps>

<Tip>
  Suggerimenti:

  * I percorsi dei file possono essere relativi o assoluti
  * I riferimenti ai file @ aggiungono `CLAUDE.md` nella directory del file e nelle directory padre al contesto
  * I riferimenti alle directory mostrano elenchi di file, non contenuti
  * Puoi fare riferimento a più file in un singolo messaggio (ad esempio, "@file1.js and @file2.js")
</Tip>

***

<h3 id="run-claude-on-a-schedule">
  Eseguire Claude su una pianificazione
</h3>

Supponiamo che tu voglia che Claude gestisca un'attività automaticamente su base ricorrente, come rivedere le PR aperte ogni mattina, controllare le dipendenze settimanalmente o verificare i fallimenti di CI durante la notte.

Scegli un'opzione di pianificazione in base a dove desideri che l'attività venga eseguita:

| Opzione                                                     | Dove viene eseguita                    | Migliore per                                                                                                                                                                                                                                |
| :---------------------------------------------------------- | :------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [Routines](/docs/it/routines)                                    | Infrastruttura gestita da Anthropic    | Attività che dovrebbero essere eseguite anche quando il tuo computer è spento. Possono anche attivarsi su chiamate API o eventi GitHub oltre a una pianificazione. Configura su [claude.ai/code/routines](https://claude.ai/code/routines). |
| [Attività pianificate desktop](/docs/it/desktop-scheduled-tasks) | La tua macchina, tramite l'app desktop | Attività che hanno bisogno di accesso diretto a file locali, strumenti o modifiche non sottoposte a commit.                                                                                                                                 |
| [GitHub Actions](/docs/it/github-actions)                        | La tua pipeline CI                     | Attività legate a eventi del repository come PR aperte, o pianificazioni cron che dovrebbero vivere insieme alla tua configurazione del flusso di lavoro.                                                                                   |
| [`/loop`](/docs/it/scheduled-tasks)                              | La sessione CLI corrente               | Polling rapido mentre una sessione è aperta. Le attività si fermano quando inizi una nuova conversazione; `--resume` e `--continue` ripristinano quelle non scadute.                                                                        |

<Tip>
  Quando scrivi prompt per attività pianificate, sii esplicito su cosa significhi il successo e cosa fare con i risultati. L'attività viene eseguita autonomamente, quindi non può fare domande di chiarimento. Ad esempio: "Review open PRs labeled `needs-review`, leave inline comments on any issues, and post a summary in the `#eng-reviews` Slack channel."
</Tip>

***

<h3 id="ask-claude-about-its-capabilities">
  Chiedere a Claude delle sue capacità
</h3>

Claude ha accesso integrato alla sua documentazione e può rispondere a domande sulle sue stesse funzionalità e limitazioni.

<h4 id="example-questions">
  Domande di esempio
</h4>

```text theme={null}
can Claude Code create pull requests?
```

```text theme={null}
how does Claude Code handle permissions?
```

```text theme={null}
what skills are available?
```

```text theme={null}
how do I use MCP with Claude Code?
```

```text theme={null}
how do I configure Claude Code for Amazon Bedrock?
```

```text theme={null}
what are the limitations of Claude Code?
```

<Note>
  Claude fornisce risposte basate sulla documentazione a queste domande. Per dimostrazioni pratiche, esegui `/powerup` per lezioni interattive con demo animate, o fai riferimento alle sezioni di flusso di lavoro specifiche sopra.
</Note>

<Tip>
  Suggerimenti:

  * Claude ha sempre accesso alla documentazione più recente di Claude Code, indipendentemente dalla versione che stai utilizzando
  * Fai domande specifiche per ottenere risposte dettagliate
  * Claude può spiegare funzionalità complesse come integrazione MCP, configurazioni aziendali e flussi di lavoro avanzati
</Tip>

***

<h2 id="resume-previous-conversations">
  Riprendere conversazioni precedenti
</h2>

Quando un'attività dura più sessioni, riprendi da dove hai lasciato invece di rispiegare il contesto. Claude Code salva ogni conversazione localmente.

```bash theme={null}
claude --continue
```

Questo riprende la sessione più recente nella directory corrente; se non ce n'è ancora una, stampa `No conversation found to continue` ed esce. Usa `claude --resume` per scegliere da un elenco, o `/resume` da una sessione in esecuzione. Vedi [Gestire le sessioni](/docs/it/sessions) per denominazione, branching e il riferimento completo del selettore.

<h2 id="run-parallel-sessions-with-worktrees">
  Eseguire sessioni parallele con worktrees
</h2>

Lavora su una funzionalità in un terminale mentre Claude corregge un bug in un altro, senza che le modifiche si scontrino. Ogni worktree è un checkout separato sul suo ramo.

```bash theme={null}
claude --worktree feature-auth
```

Esegui lo stesso comando con un nome diverso in un secondo terminale per avviare una sessione parallela isolata. Vedi [Worktrees](/docs/it/worktrees) per pulizia, `.worktreeinclude` e supporto VCS non-git. Per monitorare sessioni parallele da uno schermo invece di terminali separati, vedi [agenti in background](/docs/it/agent-view).

<h2 id="plan-before-editing">
  Pianificare prima di modificare
</h2>

Per le modifiche che desideri rivedere prima che tocchino il disco, passa a plan mode. Claude legge i file e propone un piano ma non apporta modifiche finché non approvi.

```bash theme={null}
claude --permission-mode plan
```

Puoi anche premere `Shift+Tab` durante una sessione per attivare/disattivare plan mode. Vedi [Plan mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode) per il flusso di approvazione e la modifica del piano nell'editor di testo.

<h2 id="delegate-research-to-subagents">
  Delegare la ricerca ai subagent
</h2>

Esplorare una base di codice grande riempie il tuo contesto con letture di file. Delega l'esplorazione in modo che solo i risultati tornino indietro.

```text theme={null}
use a subagent to investigate how our auth system handles token refresh
```

Il subagent legge i file nel suo proprio contesto e segnala un riassunto. Vedi [Subagent](/docs/it/sub-agents) per definire agenti personalizzati con i loro strumenti e prompt.

<h2 id="pipe-claude-into-scripts">
  Inviare Claude negli script
</h2>

Esegui Claude in modo non interattivo per CI, hook pre-commit o elaborazione batch. Stdin e stdout funzionano come qualsiasi strumento Unix.

```bash theme={null}
git log --oneline -20 | claude -p "summarize these recent commits"
```

Vedi [Modalità non interattiva](/docs/it/headless) per formati di output, flag di autorizzazione e modelli fan-out.

<h2 id="next-steps">
  Passaggi successivi
</h2>

<CardGroup cols={2}>
  <Card title="Best practices" icon="lightbulb" href="/docs/it/best-practices">
    Modelli per ottenere il massimo da Claude Code
  </Card>

  <Card title="Gestire le sessioni" icon="rotate-left" href="/docs/it/sessions">
    Riprendere, denominare e dirammare conversazioni
  </Card>

  <Card title="Worktrees" icon="code-branch" href="/docs/it/worktrees">
    Eseguire sessioni parallele isolate
  </Card>

  <Card title="Estendere Claude Code" icon="puzzle-piece" href="/docs/it/features-overview">
    Aggiungere skills, hooks, MCP, subagent e plugin
  </Card>
</CardGroup>
