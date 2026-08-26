> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Creare subagent personalizzati

> Creare e utilizzare subagent AI specializzati in Claude Code per flussi di lavoro specifici di attività e una migliore gestione del contesto.

I subagent sono assistenti AI specializzati che gestiscono tipi specifici di attività. Utilizzi uno quando un'attività secondaria allagherebbe la sua conversazione principale con risultati di ricerca, log o contenuti di file che non farà più riferimento: il subagent svolge quel lavoro nel suo proprio contesto e restituisce solo il riassunto. Definisca un subagent personalizzato quando continua a generare lo stesso tipo di worker con le stesse istruzioni.

Ogni subagent viene eseguito nella propria finestra di contesto con un prompt di sistema personalizzato, accesso a strumenti specifici e autorizzazioni indipendenti. Quando Claude incontra un'attività che corrisponde alla descrizione di un subagent, la delega a quel subagent, che lavora in modo indipendente e restituisce i risultati. Per vedere il risparmio di contesto in pratica, la [visualizzazione della finestra di contesto](/docs/it/context-window) illustra una sessione in cui un subagent gestisce la ricerca nella sua finestra separata.

<Note>
  I subagent funzionano all'interno di una singola sessione. Per eseguire molte sessioni indipendenti in parallelo e monitorarle da un unico posto, consulti [background agents](/docs/it/agent-view). Per sessioni che comunicano tra loro, consulti [agent teams](/docs/it/agent-teams).
</Note>

I subagent la aiutano a:

* **Preservare il contesto** mantenendo l'esplorazione e l'implementazione fuori dalla sua conversazione principale
* **Applicare vincoli** limitando quali strumenti un subagent può utilizzare
* **Riutilizzare configurazioni** tra progetti con subagent a livello utente
* **Specializzare il comportamento** con prompt di sistema focalizzati per domini specifici
* **Controllare i costi** instradando le attività a modelli più veloci e economici come Haiku

Claude utilizza la descrizione di ogni subagent per decidere quando delegare le attività. Quando crea un subagent, scriva una descrizione chiara in modo che Claude sappia quando utilizzarlo.

Claude Code include diversi subagent integrati come Explore, Plan e general-purpose. Può anche creare subagent personalizzati per gestire attività specifiche.

<h2 id="built-in-subagents">
  Subagent integrati
</h2>

Claude Code include subagent integrati che Claude utilizza automaticamente quando appropriato. Ognuno eredita le autorizzazioni della conversazione principale con restrizioni di strumenti aggiuntive.

Explore e Plan saltano i vostri file CLAUDE.md e lo stato git della sessione principale per mantenere la ricerca veloce ed economica. Ogni altro subagent integrato e [subagent personalizzato](#configure-subagents) carica entrambi. Per la suddivisione completa di ciò che raggiunge un subagent, consultate [cosa si carica all'avvio](#what-loads-at-startup).

<Tabs>
  <Tab title="Explore">
    Un agente veloce e di sola lettura ottimizzato per la ricerca e l'analisi delle basi di codice.

    * **Model**: eredita dalla conversazione principale, limitato a Opus sull'API Claude, quindi Explore non viene mai eseguito su un modello più costoso di quello che avete già scelto per la sessione
    * **Tools**: strumenti di sola lettura; Write e Edit sono negati
    * **Purpose**: scoperta di file, ricerca di codice, esplorazione della base di codice

    A partire dalla v2.1.198, Explore eredita il modello della conversazione principale invece di essere sempre eseguito su Haiku. Sull'API Claude, il modello ereditato è limitato a Opus: una conversazione principale su un livello superiore esegue Explore su Opus, e una conversazione principale su Sonnet o Haiku esegue Explore su quello stesso modello. Su qualsiasi altro provider, come [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, o Claude Platform su AWS](/docs/it/third-party-integrations), Explore eredita direttamente il modello della conversazione principale.

    Un [subagent utente o progetto](#choose-the-subagent-scope) denominato `Explore` sostituisce quello integrato e mantiene il proprio campo `model`, quindi definite uno con `model: haiku` per mantenere l'esplorazione su un modello a costo inferiore.

    Claude delega a Explore quando ha bisogno di cercare o comprendere una base di codice senza apportare modifiche. Questo mantiene i risultati dell'esplorazione fuori dal contesto della sua conversazione principale.

    Quando invoca Explore, Claude specifica un livello di accuratezza: **quick** per ricerche mirate, **medium** per esplorazione equilibrata, o **very thorough** per analisi completa.
  </Tab>

  <Tab title="Plan">
    Un agente di ricerca utilizzato durante la [Plan Mode](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode) per raccogliere contesto prima di presentare un piano.

    * **Model**: eredita dalla conversazione principale
    * **Tools**: strumenti di sola lettura; Write e Edit sono negati
    * **Purpose**: ricerca della base di codice per la pianificazione

    Quando è in Plan Mode e Claude ha bisogno di comprendere la sua base di codice, delega la ricerca al subagent Plan in modo che l'output dell'esplorazione rimanga in una finestra di contesto separata mentre la conversazione principale rimane di sola lettura.
  </Tab>

  <Tab title="General-purpose">
    Un agente capace per attività complesse e multi-step che richiedono sia esplorazione che azione.

    * **Model**: eredita dalla conversazione principale
    * **Tools**: tutti gli strumenti
    * **Purpose**: ricerca complessa, operazioni multi-step, modifiche del codice

    Claude delega a general-purpose quando l'attività richiede sia esplorazione che modifica, ragionamento complesso per interpretare i risultati, o più step dipendenti.
  </Tab>

  <Tab title="Other">
    Claude Code include agenti helper aggiuntivi per attività specifiche. Questi vengono generalmente invocati automaticamente, quindi non ha bisogno di utilizzarli direttamente.

    | Agent             | Model  | Quando Claude lo utilizza                                         |
    | :---------------- | :----- | :---------------------------------------------------------------- |
    | statusline-setup  | Sonnet | Quando esegue `/statusline` per configurare la sua linea di stato |
    | claude-code-guide | Haiku  | Quando fa domande sulle funzionalità di Claude Code               |
  </Tab>
</Tabs>

I subagent integrati sono registrati per impostazione predefinita nelle sessioni interattive. Per limitarli:

* Per bloccare un tipo integrato specifico, aggiungetelo a `permissions.deny` come mostrato in [Disabilitare subagent specifici](#disable-specific-subagents).
* Per impedire a Claude di delegare a qualsiasi subagent, negate lo strumento `Agent` stesso con [`permissions.deny`](/docs/it/permissions#tool-specific-permission-rules).
* Per rimuovere solo i subagent integrati `Explore` e `Plan`, impostate [`CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS=1`](/docs/it/env-vars). Claude legge ed esplora i file direttamente invece di delegare a loro. Richiede Claude Code v2.1.198 o successivo.
* In [modalità non interattiva](/docs/it/headless) e in [Agent SDK](/docs/it/agent-sdk/overview), impostate [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/it/env-vars) per rimuovere tutti i tipi integrati e fornire solo i vostri.

Oltre a questi subagent integrati, potete creare i vostri con prompt personalizzati, restrizioni di strumenti, modalità di autorizzazione, hooks e skills. Le sezioni seguenti mostrano come iniziare e personalizzare i subagent.

<h2 id="quickstart-create-your-first-subagent">
  Quickstart: crea il suo primo subagent
</h2>

I subagent sono file Markdown con frontmatter YAML. Per crearne uno, chieda a Claude di scriverlo per lei, oppure [scriva il file manualmente](#write-subagent-files).

A partire dalla v2.1.198, il comando `/agents` non apre più la procedura guidata di creazione interattiva; eseguirlo stampa un promemoria per chiedere a Claude o modificare direttamente `.claude/agents/`. I file subagent, i campi frontmatter e le posizioni `.claude/agents/` e `~/.claude/agents/` rimangono invariati; solo la procedura guidata del terminale è stata rimossa.

Questa procedura crea un subagent a livello utente che esamina il codice e suggerisce miglioramenti.

<Steps>
  <Step title="Chieda a Claude di creare il subagent">
    In Claude Code, descriva il subagent che desidera e dove salvarlo:

    ```text wrap theme={null}
    Create a personal code-improver subagent in ~/.claude/agents/ that scans
    files and suggests improvements for readability, performance, and best
    practices. It should explain each issue, show the current code, and
    provide an improved version. Make it read-only and have it use Sonnet.
    ```

    Claude scrive il file con un `name`, una `description`, un elenco `tools`, un `model` e un prompt di sistema.
  </Step>

  <Step title="Esamini il file">
    Apra `~/.claude/agents/code-improver.md` e confermi che il frontmatter corrisponda a quello che ha richiesto. Il risultato è simile a questo:

    ```markdown theme={null}
    ---
    name: code-improver
    description: Scans files and suggests improvements for readability, performance, and best practices. Use after writing or modifying code.
    tools: Read, Grep, Glob
    model: sonnet
    ---

    You are a code improvement specialist. For each issue you find, explain
    the problem, show the current code, and provide an improved version.
    ```

    Poiché il file si trova in `~/.claude/agents/`, il subagent è disponibile in ogni progetto sulla sua macchina. Per limitarlo a un solo progetto, lo sposti nella directory `.claude/agents/` di quel progetto. [Scelga l'ambito del subagent](#choose-the-subagent-scope) confronta i due.
  </Step>

  <Step title="Lo provi">
    Chieda a Claude di delegare al nuovo subagent:

    ```text wrap theme={null}
    Use the code-improver agent to suggest improvements in this project
    ```

    Claude delega al suo nuovo subagent, che scansiona la base di codice e restituisce suggerimenti di miglioramento.

    Se Claude non riesce a trovare il nuovo subagent, riavvii Claude Code e riprovi. Questo accade solo quando `~/.claude/agents/` non esisteva prima dell'inizio della sessione, perché una sessione in esecuzione non rileva una directory `agents` appena creata.
  </Step>
</Steps>

Ora ha un subagent che può utilizzare in qualsiasi progetto sulla sua macchina per analizzare le basi di codice e suggerire miglioramenti.

Può anche scrivere file subagent manualmente, definirli tramite flag CLI, o distribuirli tramite plugin. Le sezioni seguenti coprono tutte le opzioni di configurazione.

<Note>
  Su Claude Code v2.1.197 e versioni precedenti, `/agents` apre una procedura guidata interattiva con una scheda **Running** che elenca i subagent attivi e una scheda **Library** per crearli, modificarli ed eliminarli.&#x20;
</Note>

<h2 id="configure-subagents">
  Configuri i subagent
</h2>

La posizione del file di un subagent determina chi ha accesso ad esso, e il suo frontmatter determina cosa può fare. Questa sezione copre dove vivono i file dei subagent e ogni campo che supportano.

<h3 id="choose-the-subagent-scope">
  Scelga l'ambito del subagent
</h3>

Archivi i file dei subagent in posizioni diverse a seconda dell'ambito. Quando più subagent condividono lo stesso nome, Claude Code utilizza quello dalla posizione con priorità più alta.

| Location                       | Scope                      | Priority    | Come creare                                          |
| :----------------------------- | :------------------------- | :---------- | :--------------------------------------------------- |
| Managed settings               | Organization-wide          | 1 (massima) | Distribuito tramite [managed settings](/docs/it/settings) |
| Flag CLI `--agents`            | Sessione corrente          | 2           | Passa JSON quando avvia Claude Code                  |
| `.claude/agents/`              | Progetto corrente          | 3           | Chieda a Claude, o crei il file manualmente          |
| `~/.claude/agents/`            | Tutti i suoi progetti      | 4           | Chieda a Claude, o crei il file manualmente          |
| Directory `agents/` del plugin | Dove il plugin è abilitato | 5 (minima)  | Installato con [plugins](/docs/it/plugins)                |

I **subagent di progetto** (`.claude/agents/`) sono ideali per subagent specifici di una base di codice. Li archivi nel controllo della versione in modo che il suo team possa utilizzarli e migliorarli in modo collaborativo.

I subagent di progetto vengono scoperti camminando verso l'alto dalla directory di lavoro corrente, quindi ogni `.claude/agents/` tra lì e la radice del repository viene scansionato. A partire da v2.1.178, quando più di una di queste directory annidate definisce lo stesso `name`, Claude Code utilizza la definizione più vicina alla directory di lavoro.

Le directory aggiunte con `--add-dir` vengono anche scansionate: una cartella `.claude/agents/` all'interno di una directory aggiunta si carica insieme ai subagent di progetto. Consulti [Directory aggiuntive](/docs/it/permissions#additional-directories-grant-file-access-not-configuration) per quali altri tipi di configurazione si caricano da `--add-dir`. Per condividere i subagent tra progetti senza `--add-dir`, usi `~/.claude/agents/` o un [plugin](/docs/it/plugins).

I **subagent utente** (`~/.claude/agents/`) sono subagent personali disponibili in tutti i suoi progetti.

Claude Code scansiona `.claude/agents/` e `~/.claude/agents/` ricorsivamente, quindi può organizzare le definizioni in sottocartelle come `agents/review/` o `agents/research/`. Il percorso della sottodirectory non influisce su come un subagent viene identificato o invocato, perché l'identità proviene solo dal campo frontmatter `name`.

Mantenga i valori `name` univoci in tutto l'albero: se due file all'interno dello stesso ambito `.claude/agents/`, incluse le sue sottocartelle, dichiarano lo stesso nome, Claude Code carica solo uno di essi, scelto dall'ordine di lettura del filesystem piuttosto che da una precedenza documentata. Tra le directory di progetto annidate, la definizione più vicina alla directory di lavoro vince, come descritto sopra. Il controllo di configurazione [`/doctor`](/docs/it/commands#all-commands) segnala i file nello stesso ambito che condividono un nome e propone di rinominare o rimuovere tutti tranne uno. Prima di v2.1.205, `/doctor` apriva una schermata di diagnostica che elencava i duplicati e mostrava quale definizione era attiva.

Le directory `agents/` del plugin vengono scansionate anche ricorsivamente. A differenza degli ambiti di progetto e utente, una sottocartella all'interno della directory `agents/` di un plugin diventa parte dell'[identificatore con ambito](#invoke-subagents-explicitly): un file in `agents/review/security.md` nel plugin `my-plugin` si registra come `my-plugin:review:security`.

I **subagent definiti da CLI** vengono passati come JSON quando avvia Claude Code. Esistono solo per quella sessione e non vengono salvati su disco, rendendoli utili per test rapidi o script di automazione. Può definire più subagent in una singola chiamata `--agents`:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    claude --agents '{
      "code-reviewer": {
        "description": "Expert code reviewer. Use proactively after code changes.",
        "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
        "tools": ["Read", "Grep", "Glob", "Bash"],
        "model": "sonnet"
      },
      "debugger": {
        "description": "Debugging specialist for errors and test failures.",
        "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
      }
    }'
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    claude --agents @'
    {
      "code-reviewer": {
        "description": "Expert code reviewer. Use proactively after code changes.",
        "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
        "tools": ["Read", "Grep", "Glob", "Bash"],
        "model": "sonnet"
      },
      "debugger": {
        "description": "Debugging specialist for errors and test failures.",
        "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
      }
    }
    '@
    ```
  </Tab>
</Tabs>

Il flag `--agents` accetta JSON con gli stessi campi [frontmatter](#supported-frontmatter-fields) dei subagent basati su file: `description`, `prompt`, `tools`, `disallowedTools`, `model`, `permissionMode`, `mcpServers`, `hooks`, `maxTurns`, `skills`, `initialPrompt`, `memory`, `effort`, `background`, `isolation` e `color`. Usi `prompt` per il prompt di sistema, equivalente al corpo markdown nei subagent basati su file.

I **subagent gestiti** vengono distribuiti dagli amministratori dell'organizzazione. Posizioni file markdown in `.claude/agents/` all'interno della [directory managed settings](/docs/it/settings#settings-files), utilizzando lo stesso formato frontmatter dei subagent di progetto e utente. Le definizioni gestite hanno la precedenza sui subagent di progetto e utente con lo stesso nome.

I **subagent plugin** provengono da [plugins](/docs/it/plugins) che ha installato. Si caricano insieme ai suoi subagent personalizzati e appaiono nella typeahead @-mention con il loro nome con ambito. Consulti il [riferimento dei componenti plugin](/docs/it/plugins-reference#agents) per i dettagli sulla creazione di subagent plugin.

<Note>
  Per motivi di sicurezza, i subagent plugin non supportano i campi frontmatter `hooks`, `mcpServers` o `permissionMode`. Questi campi vengono ignorati durante il caricamento degli agenti da un plugin. Se ne ha bisogno, copi il file dell'agente in `.claude/agents/` o `~/.claude/agents/`. Può anche aggiungere regole a [`permissions.allow`](/docs/it/settings#permission-settings) in `settings.json` o `settings.local.json`, ma queste regole si applicano all'intera sessione, non solo al subagent plugin.
</Note>

Le definizioni di subagent da uno qualsiasi di questi ambiti sono anche disponibili per [agent teams](/docs/it/agent-teams#use-subagent-definitions-for-teammates): quando genera un compagno di squadra, può fare riferimento a un tipo di subagent e il compagno di squadra utilizza i suoi `tools` e `model`, con il corpo della definizione aggiunto al prompt di sistema del compagno di squadra come istruzioni aggiuntive. Consulti [agent teams](/docs/it/agent-teams#use-subagent-definitions-for-teammates) per quali campi frontmatter si applicano su quel percorso.

<h3 id="write-subagent-files">
  Scriva file subagent
</h3>

I file subagent utilizzano frontmatter YAML per la configurazione, seguito dal prompt di sistema in Markdown:

<Note>
  Claude Code osserva `~/.claude/agents/` e `.claude/agents/`. Quando aggiunge o modifica un file subagent su disco, o chiede a Claude di scriverne uno per lei, Claude Code rileva il cambiamento entro pochi secondi e la prossima delega utilizza la definizione aggiornata, senza necessità di riavvio.

  Due casi richiedono ancora un riavvio:

  * L'osservatore copre solo le directory che esistevano quando la sessione è iniziata, quindi dopo aver creato il primo file agente di un ambito in una nuova directory `agents`, riavvii per caricarlo.
  * Le sessioni avviate con `--disable-slash-commands` non osservano affatto queste directory.
</Note>

```markdown theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
tools: Read, Glob, Grep
model: sonnet
---

You are a code reviewer. When invoked, analyze the code and provide
specific, actionable feedback on quality, security, and best practices.
```

Il frontmatter definisce i metadati e la configurazione del subagent. Il corpo diventa il prompt di sistema che guida il comportamento del subagent. I subagent ricevono solo questo prompt di sistema più dettagli di base sull'ambiente come la directory di lavoro, non il prompt di sistema completo di Claude Code.

In [modalità non interattiva](/docs/it/headless), il flag [`--append-subagent-system-prompt`](/docs/it/cli-reference#cli-flags) aggiunge il testo che fornisce alla fine del prompt di sistema di ogni subagent, inclusi i subagent annidati. Richiede Claude Code v2.1.205 o successivo.

Un subagent inizia nella directory di lavoro corrente della conversazione principale. All'interno di un subagent, i comandi `cd` non persistono tra le chiamate dello strumento Bash o PowerShell e non influenzano la directory di lavoro della conversazione principale. Per dare al subagent una copia isolata del repository, imposti [`isolation: worktree`](#supported-frontmatter-fields).

Un subagent con `isolation: worktree` esegue i suoi comandi Bash e PowerShell all'interno del suo worktree. Un comando la cui directory di lavoro si risolve nel suo checkout principale, ad esempio perché la directory del worktree è stata rimossa mentre il subagent era in esecuzione, fallisce con un errore. Prima di v2.1.203, tale comando potrebbe essere eseguito nel checkout principale.

<h4 id="supported-frontmatter-fields">
  Campi frontmatter supportati
</h4>

I seguenti campi possono essere utilizzati nel frontmatter YAML. Solo `name` e `description` sono obbligatori.

| Field             | Required | Description                                                                                                                                                                                                                                                                                                                                                                        |
| :---------------- | :------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`            | Yes      | Identificatore univoco utilizzando lettere minuscole e trattini. [Hooks](/docs/it/hooks#subagentstart) ricevono questo valore come `agent_type`. Il nome del file non deve corrispondere                                                                                                                                                                                                |
| `description`     | Yes      | Quando Claude dovrebbe delegare a questo subagent                                                                                                                                                                                                                                                                                                                                  |
| `tools`           | No       | [Strumenti](#available-tools) che il subagent può utilizzare. Eredita tutti gli strumenti se omesso. Se nessuna voce nell'elenco si risolve in uno strumento, il subagent non si avvia con un errore che nomina le voci. Per precaricare Skills nel contesto, usi il campo `skills` piuttosto che elencare `Skill` qui                                                             |
| `disallowedTools` | No       | Strumenti da negare, rimossi dall'elenco ereditato o specificato                                                                                                                                                                                                                                                                                                                   |
| `model`           | No       | [Modello](#choose-a-model) da utilizzare: `sonnet`, `opus`, `haiku`, `fable`, un ID modello completo (ad esempio, `claude-opus-4-8`), o `inherit`. Predefinito: `inherit`                                                                                                                                                                                                          |
| `permissionMode`  | No       | [Modalità di autorizzazione](#permission-modes): `default`, `acceptEdits`, `auto`, `dontAsk`, `bypassPermissions`, `plan`, o `manual` come alias per `default`. L'alias `manual` richiede Claude Code v2.1.200 o successivo. Ignorato per [subagent plugin](#choose-the-subagent-scope)                                                                                            |
| `maxTurns`        | No       | Numero massimo di turni agentici prima che il subagent si fermi                                                                                                                                                                                                                                                                                                                    |
| `skills`          | No       | [Skills](/docs/it/skills) da precaricare nel contesto del subagent all'avvio. Il contenuto completo della skill viene iniettato, non solo la descrizione. I subagent possono ancora invocare skills di progetto, utente e plugin non elencate tramite lo strumento Skill                                                                                                                |
| `mcpServers`      | No       | [MCP servers](/docs/it/mcp) disponibili per questo subagent. Ogni voce è un nome di server che fa riferimento a un server già configurato (ad esempio, `"slack"`) o una definizione inline con il nome del server come chiave e una [configurazione MCP server](/docs/it/mcp#installing-mcp-servers) completa come valore. Ignorato per [subagent plugin](#choose-the-subagent-scope)        |
| `hooks`           | No       | [Lifecycle hooks](#define-hooks-for-subagents) limitati a questo subagent. Ignorato per [subagent plugin](#choose-the-subagent-scope)                                                                                                                                                                                                                                              |
| `memory`          | No       | [Ambito di memoria persistente](#enable-persistent-memory): `user`, `project`, o `local`. Abilita l'apprendimento tra sessioni                                                                                                                                                                                                                                                     |
| `background`      | No       | Imposta su `true` per eseguire sempre questo subagent come [background task](#run-subagents-in-foreground-or-background), anche quando Claude ha bisogno del suo risultato subito. Quando non impostato, Claude sceglie, e a partire da v2.1.198 esegue i subagent in background per impostazione predefinita                                                                      |
| `effort`          | No       | Livello di sforzo quando questo subagent è attivo. Sostituisce il livello di sforzo della sessione. Predefinito: eredita dalla sessione. Opzioni: `low`, `medium`, `high`, `xhigh`, `max`; i livelli disponibili dipendono dal modello                                                                                                                                             |
| `isolation`       | No       | Imposta su `worktree` per eseguire il subagent in un [git worktree](/docs/it/worktrees) temporaneo, dandogli una copia isolata del repository diramata per impostazione predefinita dal suo [ramo predefinito](/docs/it/worktrees#choose-the-base-branch) piuttosto che dall'`HEAD` della sessione principale. Il worktree viene automaticamente pulito se il subagent non apporta modifiche |
| `color`           | No       | Colore di visualizzazione per il subagent nell'elenco attività e nella trascrizione. Accetta `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink`, o `cyan`                                                                                                                                                                                                                |
| `initialPrompt`   | No       | Auto-inviato come primo turno utente quando questo agente viene eseguito come agente della sessione principale (tramite `--agent` o l'impostazione `agent`). [Commands](/docs/it/commands) e [skills](/docs/it/skills) vengono elaborati. Anteposto a qualsiasi prompt fornito dall'utente                                                                                                   |

<h3 id="choose-a-model">
  Scelga un modello
</h3>

Il campo `model` controlla quale [modello AI](/docs/it/model-config) utilizza il subagent:

* **Alias modello**: Usi uno degli alias disponibili: `sonnet`, `opus`, `haiku`, o `fable`
* **ID modello completo**: Usi un ID modello completo come `claude-opus-4-8` o `claude-sonnet-5`. Accetta gli stessi valori del flag `--model`
* **inherit**: Usi lo stesso modello della conversazione principale
* **Omesso**: Se non specificato, predefinito a `inherit` (usa lo stesso modello della conversazione principale)

Quando Claude invoca un subagent, può anche passare un parametro `model` per quella specifica invocazione. Claude Code risolve il modello del subagent in questo ordine:

1. La variabile di ambiente [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/it/model-config#environment-variables), quando impostata a un alias modello o ID modello
2. Il parametro `model` per invocazione
3. Il frontmatter `model` della definizione del subagent
4. Il modello della conversazione principale

A partire da v2.1.196, impostare `CLAUDE_CODE_SUBAGENT_MODEL` su `inherit` è lo stesso che lasciarla non impostata: la risoluzione continua con il parametro `model` per invocazione, quindi il frontmatter. Nelle versioni precedenti, `inherit` forzava i subagent sul modello della conversazione principale e ignorava entrambe quelle fonti.

Claude Code controlla la variabile di ambiente, il parametro per invocazione e i valori frontmatter rispetto alla lista di consentimento [`availableModels`](/docs/it/model-config#restrict-model-selection) della sua organizzazione. Un valore che si risolve in un modello escluso non viene utilizzato e il subagent viene eseguito sul modello ereditato.

A partire da v2.1.198, i subagent ereditano anche la configurazione [extended thinking](/docs/it/model-config#extended-thinking) della conversazione principale: se il thinking è attivo nella sua sessione, è attivo per il subagent, e se è spento, rimane spento. Non c'è un'impostazione di thinking per subagent. Prima di v2.1.198, i subagent venivano eseguiti con extended thinking disabilitato indipendentemente dall'impostazione della conversazione principale.

<h3 id="control-subagent-capabilities">
  Controlli le capacità del subagent
</h3>

Può controllare cosa possono fare i subagent attraverso l'accesso agli strumenti, le modalità di autorizzazione e le regole condizionali.

<h4 id="available-tools">
  Strumenti disponibili
</h4>

I subagent ereditano gli [strumenti interni](/docs/it/tools-reference) e gli strumenti MCP disponibili nella conversazione principale per impostazione predefinita. I seguenti strumenti dipendono dall'interfaccia utente della conversazione principale o dallo stato della sessione e non sono disponibili per i subagent, anche se elencati nel campo `tools`:

* `AskUserQuestion`
* `EnterPlanMode`
* `ExitPlanMode`, a meno che la [`permissionMode`](#permission-modes) del subagent non sia `plan`
* `ScheduleWakeup`
* `WaitForMcpServers`

Per limitare gli strumenti, usi il campo `tools` come allowlist o il campo `disallowedTools` come denylist. Questo esempio usa `tools` per consentire esclusivamente Read, Grep, Glob e Bash. Il subagent non può modificare file, scrivere file o utilizzare alcuno strumento MCP:

```yaml theme={null}
---
name: safe-researcher
description: Research agent with restricted capabilities
tools: Read, Grep, Glob, Bash
---
```

Questo esempio usa `disallowedTools` per ereditare ogni strumento dalla conversazione principale tranne Write e Edit. Il subagent mantiene Bash, strumenti MCP e tutto il resto:

```yaml theme={null}
---
name: no-writes
description: Inherits every tool except file writes
disallowedTools: Write, Edit
---
```

Se entrambi sono impostati, `disallowedTools` viene applicato per primo, quindi `tools` viene risolto rispetto al pool rimanente. Uno strumento elencato in entrambi viene rimosso.

Quando nulla nell'elenco `tools` si risolve in uno strumento, ad esempio perché ogni voce è errata o nomina uno strumento che non è disponibile per i subagent, Claude Code rifiuta di avviare il subagent e lo strumento Agent restituisce un errore che nomina le voci non risolte. Prima di v2.1.208, quel subagent si avviava senza strumenti e potrebbe restituire un risultato vuoto o confuso.

Entrambi i campi accettano modelli a livello di server MCP oltre ai nomi esatti degli strumenti: `mcp__<server>` o `mcp__<server>__*` concede o rimuove ogni strumento dal server denominato. In `disallowedTools`, `mcp__*` rimuove anche ogni strumento MCP da qualsiasi server. Questo esempio rimuove ogni strumento dal server MCP `github` mentre mantiene gli strumenti da altri server e ogni strumento integrato:

```yaml theme={null}
---
name: local-only
description: Inherits every tool except those from the github MCP server
disallowedTools: mcp__github
---
```

<h4 id="restrict-which-subagents-can-be-spawned">
  Limiti quali subagent possono essere generati
</h4>

Quando un agente viene eseguito come thread principale con `claude --agent`, può generare subagent utilizzando lo strumento Agent. Per limitare quali tipi di subagent può generare, usi la sintassi `Agent(agent_type)` nel campo `tools`.

<Note>Nella versione 2.1.63, lo strumento Task è stato rinominato in Agent. I riferimenti `Task(...)` esistenti nelle impostazioni e nelle definizioni degli agenti continuano a funzionare come alias.</Note>

```yaml theme={null}
---
name: coordinator
description: Coordinates work across specialized agents
tools: Agent(worker, researcher), Read, Bash
---
```

Questo è un allowlist: solo i subagent `worker` e `researcher` possono essere generati. Se l'agente tenta di generare qualsiasi altro tipo, la richiesta fallisce e l'agente vede solo i tipi consentiti nel suo prompt. Per bloccare agenti specifici mentre consente tutti gli altri, usi [`permissions.deny`](#disable-specific-subagents) invece.

Per consentire la generazione di qualsiasi subagent senza restrizioni, usi `Agent` senza parentesi:

```yaml theme={null}
tools: Agent, Read, Bash
```

Se `Agent` è completamente omesso dall'elenco `tools`, l'agente non può generare alcun subagent.

La sintassi allowlist `Agent(agent_type)` si applica solo a un agente eseguito come thread principale con `claude --agent`. In una definizione di subagent, elencare `Agent` in `tools` consente a quel subagent di [generare subagent annidati](#spawn-nested-subagents), ma qualsiasi elenco di tipi all'interno delle parentesi viene ignorato.

<h4 id="scope-mcp-servers-to-a-subagent">
  Limiti i server MCP a un subagent
</h4>

Usi il campo `mcpServers` per dare a un subagent accesso ai server [MCP](/docs/it/mcp) che non sono disponibili nella conversazione principale. I server inline definiti qui vengono connessi quando il subagent inizia e disconnessi quando finisce. I riferimenti stringa condividono la connessione della sessione principale.

<Note>
  Il campo `mcpServers` si applica in entrambi i contesti in cui un file agente può essere eseguito:

  * Come subagent, generato tramite lo strumento Agent o un @-mention
  * Come sessione principale, avviato con [`--agent`](#invoke-subagents-explicitly) o l'impostazione `agent`

  Quando l'agente è la sessione principale, le definizioni di server inline si connettono all'avvio insieme ai server da [`.mcp.json`](/docs/it/mcp) e ai file di impostazioni.
</Note>

Ogni voce nell'elenco è una definizione di server inline o una stringa che fa riferimento a un server MCP già configurato nella sua sessione:

```yaml theme={null}
---
name: browser-tester
description: Tests features in a real browser using Playwright
mcpServers:
  # Inline definition: scoped to this subagent only
  - playwright:
      type: stdio
      command: npx
      args: ["-y", "@playwright/mcp@latest"]
  # Reference by name: reuses an already-configured server
  - github
---

Use the Playwright tools to navigate, screenshot, and interact with pages.
```

Le definizioni inline utilizzano lo stesso schema delle voci del server `.mcp.json`, con chiave il nome del server, e supportano i tipi `stdio`, `http`, `sse` e `ws`.

Per mantenere un server MCP fuori dalla conversazione principale e evitare che le descrizioni dei suoi strumenti consumino contesto lì, lo definisca inline qui piuttosto che in `.mcp.json`. Il subagent ottiene gli strumenti; la conversazione principale no.

A partire da v2.1.153, le restrizioni MCP che si applicano alla sessione principale coprono anche i server dichiarati nel frontmatter del subagent:

* [`--strict-mcp-config`](/docs/it/cli-reference) e [`--bare`](/docs/it/cli-reference)
* [Configurazione MCP gestita aziendale](/docs/it/managed-mcp)
* [Politiche `allowedMcpServers` e `deniedMcpServers`](/docs/it/managed-mcp#policy-based-control-with-allowlists-and-denylists)

Quando uno di questi blocca un server, Claude Code lo salta e mostra un avviso che nomina i server bloccati.

Le restrizioni delle impostazioni gestite si applicano a ogni subagent indipendentemente da come è definito. `--strict-mcp-config` non filtra i server che passa inline tramite `--agents` o l'opzione SDK `agents`, poiché si tratta di input esplicito del chiamante.

<h4 id="permission-modes">
  Modalità di autorizzazione
</h4>

Il campo `permissionMode` controlla come il subagent gestisce i prompt di autorizzazione. I subagent ereditano il contesto di autorizzazione dalla conversazione principale e possono sovrascrivere la modalità, tranne quando la modalità principale ha la precedenza come descritto di seguito.

| Mode                | Behavior                                                                                                                                                                                                                                                                                                                                                                                            |
| :------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Controllo di autorizzazione standard con prompt                                                                                                                                                                                                                                                                                                                                                     |
| `acceptEdits`       | Auto-accetta modifiche ai file e comandi comuni del filesystem per i percorsi nella directory di lavoro o `additionalDirectories`                                                                                                                                                                                                                                                                   |
| `auto`              | [Auto mode](/docs/it/permission-modes#eliminate-prompts-with-auto-mode): un classificatore AI valuta ogni chiamata di strumento                                                                                                                                                                                                                                                                          |
| `dontAsk`           | Auto-nega prompt di autorizzazione. Gli strumenti esplicitamente consentiti continuano a funzionare; `AskUserQuestion`, strumenti connettore [che la sua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools), e strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool) vengono negati anche se li ha consentiti |
| `bypassPermissions` | Salta i prompt di autorizzazione                                                                                                                                                                                                                                                                                                                                                                    |
| `plan`              | Plan mode (esplorazione di sola lettura)                                                                                                                                                                                                                                                                                                                                                            |

<Warning>
  Usi `bypassPermissions` con cautela. Salta i prompt di autorizzazione, consentendo al subagent di eseguire operazioni senza approvazione, incluse le scritture in `.git`, `.config/git`, `.claude`, `.vscode`, `.idea`, `.husky`, `.cargo`, `.devcontainer`, `.yarn` e `.mvn`.

  Le regole [`ask`](/docs/it/permissions#manage-permissions) esplicite, gli strumenti connettore [che la sua organizzazione ha impostato su `ask`](/docs/it/mcp#organization-controls-on-connector-tools), gli strumenti MCP contrassegnati [`requiresUserInteraction`](/docs/it/mcp#require-approval-for-a-specific-tool), e le rimozioni della directory root e home come `rm -rf /` continuano a richiedere. Consulti [permission modes](/docs/it/permission-modes#skip-all-checks-with-bypasspermissions-mode) per i dettagli.
</Warning>

Se il principale utilizza `bypassPermissions` o `acceptEdits`, questo ha la precedenza e non può essere sovrascritto. Se il principale utilizza [auto mode](/docs/it/permission-modes#eliminate-prompts-with-auto-mode), il subagent eredita auto mode e qualsiasi `permissionMode` nel suo frontmatter viene ignorato: il classificatore valuta le chiamate di strumenti del subagent con le stesse regole di blocco e consentimento della sessione principale.

<h4 id="preload-skills-into-subagents">
  Precarichi skills nei subagent
</h4>

Usi il campo `skills` per iniettare il contenuto della skill nel contesto del subagent all'avvio. Questo dà al subagent conoscenza del dominio senza richiedere che scopra e carichi le skills durante l'esecuzione.

```yaml theme={null}
---
name: api-developer
description: Implement API endpoints following team conventions
skills:
  - api-conventions
  - error-handling-patterns
---

Implement API endpoints. Follow the conventions and patterns from the preloaded skills.
```

Il contenuto completo di ogni skill elencata viene iniettato nel contesto del subagent all'avvio. Questo campo controlla quali skills vengono precaricate, non quali skills il subagent può accedere: senza di esso, il subagent può comunque scoprire e invocare skills di progetto, utente e plugin tramite lo strumento Skill durante l'esecuzione. Per impedire a un subagent di invocare skills interamente, ometta `Skill` dall'elenco [`tools`](#available-tools) o aggiunga a `disallowedTools`.

Non può precaricare skills che impostano [`disable-model-invocation: true`](/docs/it/skills#control-who-invokes-a-skill), poiché il precaricamento attinge dallo stesso insieme di skills che Claude può invocare. Se una skill elencata è mancante o disabilitata, Claude Code la salta e registra un avviso nel log di debug.

<Note>
  Questo è l'inverso di [eseguire una skill in un subagent](/docs/it/skills#run-skills-in-a-subagent). Con `skills` in un subagent, il subagent controlla il prompt di sistema e carica il contenuto della skill. Con `context: fork` in una skill, il contenuto della skill viene iniettato nell'agente che specifica. Entrambi utilizzano lo stesso sistema sottostante.
</Note>

<h4 id="enable-persistent-memory">
  Abiliti memoria persistente
</h4>

Il campo `memory` dà al subagent una directory persistente che sopravvive tra le conversazioni. Il subagent utilizza questa directory per costruire conoscenza nel tempo, come modelli di base di codice, intuizioni di debug e decisioni architettoniche.

```yaml theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
memory: user
---

You are a code reviewer. As you review code, update your agent memory with
patterns, conventions, and recurring issues you discover.
```

Scelga un ambito in base a quanto ampiamente la memoria dovrebbe applicarsi:

| Scope     | Location                                      | Usi quando                                                                                                         |
| :-------- | :-------------------------------------------- | :----------------------------------------------------------------------------------------------------------------- |
| `user`    | `~/.claude/agent-memory/<name-of-agent>/`     | il subagent dovrebbe ricordare gli insegnamenti tra tutti i progetti                                               |
| `project` | `.claude/agent-memory/<name-of-agent>/`       | la conoscenza del subagent è specifica del progetto e condivisibile tramite controllo della versione               |
| `local`   | `.claude/agent-memory-local/<name-of-agent>/` | la conoscenza del subagent è specifica del progetto ma non dovrebbe essere archiviata nel controllo della versione |

Quando la memoria è abilitata:

* Il prompt di sistema del subagent include istruzioni per leggere e scrivere nella directory di memoria.
* Il prompt di sistema del subagent include anche le prime 200 righe o 25KB di `MEMORY.md` nella directory di memoria, a seconda di quale sia minore, con istruzioni per curare `MEMORY.md` se supera quel limite.
* Gli strumenti Read, Write e Edit vengono automaticamente abilitati in modo che il subagent possa gestire i suoi file di memoria.

<h5 id="persistent-memory-tips">
  Suggerimenti per la memoria persistente
</h5>

* `project` è l'ambito predefinito consigliato. Lo rende condivisibile tramite controllo della versione.
* Chieda al subagent di consultare la sua memoria prima di iniziare il lavoro: "Review this PR, and check your memory for patterns you've seen before."
* Chieda al subagent di aggiornare la sua memoria dopo aver completato un'attività: "Now that you're done, save what you learned to your memory." Nel tempo, questo costruisce una base di conoscenza che rende il subagent più efficace.
* Includa istruzioni di memoria direttamente nel file markdown del subagent in modo che mantenga proattivamente la sua stessa base di conoscenza:

  ```markdown theme={null}
  Update your agent memory as you discover codepaths, patterns, library
  locations, and key architectural decisions. This builds up institutional
  knowledge across conversations. Write concise notes about what you found
  and where.
  ```

<h4 id="conditional-rules-with-hooks">
  Regole condizionali con hooks
</h4>

Per un controllo più dinamico sull'utilizzo degli strumenti, usi gli hook `PreToolUse` per convalidare le operazioni prima che vengono eseguite. Questo è utile quando ha bisogno di consentire alcune operazioni di uno strumento mentre ne blocca altre.

Questo esempio crea un subagent che consente solo query di database di sola lettura. L'hook `PreToolUse` esegue lo script specificato in `command` prima di ogni comando Bash:

```yaml theme={null}
---
name: db-reader
description: Execute read-only database queries
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---
```

Claude Code [passa l'input dell'hook come JSON](/docs/it/hooks#pretooluse-input) tramite stdin ai comandi dell'hook. Lo script di convalida legge questo JSON, estrae il comando Bash e [esce con codice 2](/docs/it/hooks#exit-code-2-behavior-per-event) per bloccare le operazioni di scrittura:

```bash theme={null}
#!/bin/bash
# ./scripts/validate-readonly-query.sh

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# Block SQL write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE)\b' > /dev/null; then
  echo "Blocked: Only SELECT queries are allowed" >&2
  exit 2
fi

exit 0
```

Consulti [Hook input](/docs/it/hooks#pretooluse-input) per lo schema di input completo e [exit codes](/docs/it/hooks#exit-code-output) per come i codici di uscita influenzano il comportamento. Su Windows, scriva gli script dell'hook in PowerShell e aggiunga `shell: powershell` alla voce dell'hook come mostrato in [running hooks in PowerShell](/docs/it/hooks#windows-powershell-tool).

<h4 id="disable-specific-subagents">
  Disabiliti subagent specifici
</h4>

Può impedire a Claude di utilizzare subagent specifici aggiungendoli all'array `deny` nelle sue [impostazioni](/docs/it/settings#permission-settings). Usi il formato `Agent(subagent-name)` dove `subagent-name` corrisponde al campo name del subagent.

```json theme={null}
{
  "permissions": {
    "deny": ["Agent(Explore)", "Agent(my-custom-agent)"]
  }
}
```

Questo funziona sia per i subagent integrati che personalizzati. Può anche usare il flag CLI `--disallowedTools`:

```bash theme={null}
claude --disallowedTools "Agent(Explore)"
```

Consulti la [documentazione Permissions](/docs/it/permissions#tool-specific-permission-rules) per più dettagli sulle regole di autorizzazione.

<h3 id="define-hooks-for-subagents">
  Definisca hook per i subagent
</h3>

I subagent possono definire [hook](/docs/it/hooks) che vengono eseguiti durante il ciclo di vita del subagent. Ci sono due modi per configurare gli hook:

* **Nel frontmatter del subagent**: Definisca hook che vengono eseguiti solo mentre quel subagent è attivo
* **In `settings.json`**: Definisca hook che vengono eseguiti nella sessione principale quando i subagent iniziano o si fermano

<h4 id="hooks-in-subagent-frontmatter">
  Hook nel frontmatter del subagent
</h4>

Definisca gli hook direttamente nel file markdown del subagent. Questi hook vengono eseguiti solo mentre quel subagent specifico è attivo e vengono puliti quando finisce.

<Note>
  Gli hook nel frontmatter si attivano quando l'agente viene generato come subagent tramite lo strumento Agent o un @-mention, e quando l'agente viene eseguito come principale della sessione tramite [`--agent`](#invoke-subagents-explicitly) o l'impostazione `agent`. Nel caso della sessione principale, vengono eseguiti insieme a qualsiasi hook definito in [`settings.json`](/docs/it/hooks).
</Note>

Tutti gli [hook events](/docs/it/hooks#hook-events) sono supportati. Gli eventi più comuni per i subagent sono:

| Event         | Matcher input        | Quando si attiva                                                     |
| :------------ | :------------------- | :------------------------------------------------------------------- |
| `PreToolUse`  | Nome dello strumento | Prima che il subagent utilizzi uno strumento                         |
| `PostToolUse` | Nome dello strumento | Dopo che il subagent ha utilizzato uno strumento                     |
| `Stop`        | (nessuno)            | Quando il subagent finisce (convertito in `SubagentStop` al runtime) |

Questo esempio convalida i comandi Bash con l'hook `PreToolUse` ed esegue un linter dopo le modifiche ai file con `PostToolUse`:

```yaml theme={null}
---
name: code-reviewer
description: Review code changes with automatic linting
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-command.sh $TOOL_INPUT"
  PostToolUse:
    - matcher: "Edit|Write"
      hooks:
        - type: command
          command: "./scripts/run-linter.sh"
---
```

Quando l'agente viene invocato come subagent, gli hook `Stop` nel frontmatter vengono automaticamente convertiti in eventi `SubagentStop`.

<h4 id="project-level-hooks-for-subagent-events">
  Hook a livello di progetto per gli eventi dei subagent
</h4>

Configuri gli hook in `settings.json` che rispondono agli eventi del ciclo di vita dei subagent nella sessione principale.

| Event           | Matcher input           | Quando si attiva                       |
| :-------------- | :---------------------- | :------------------------------------- |
| `SubagentStart` | Nome del tipo di agente | Quando un subagent inizia l'esecuzione |
| `SubagentStop`  | Nome del tipo di agente | Quando un subagent completa            |

Entrambi gli eventi supportano matcher per indirizzare tipi di agenti specifici per nome. Il valore del matcher è il `name` del frontmatter dell'agente per i subagent a livello di progetto e utente, o l'identificatore con ambito del plugin come `my-plugin:db-agent` per [subagent plugin](/docs/it/plugins). Un nome con ambito contiene un due punti, quindi viene valutato come un'[espressione regolare non ancorata](/docs/it/hooks#matcher-patterns); ancoratelo con `^` e `$`, come in `^my-plugin:db-agent$`, per corrispondere solo a quell'agente.

Questo esempio esegue uno script di configurazione solo quando il subagent `db-agent` inizia e uno script di pulizia quando qualsiasi subagent si ferma:

```json theme={null}
{
  "hooks": {
    "SubagentStart": [
      {
        "matcher": "db-agent",
        "hooks": [
          { "type": "command", "command": "./scripts/setup-db-connection.sh" }
        ]
      }
    ],
    "SubagentStop": [
      {
        "hooks": [
          { "type": "command", "command": "./scripts/cleanup-db-connection.sh" }
        ]
      }
    ]
  }
}
```

Un matcher con trattini come `db-agent` corrisponde esattamente su Claude Code v2.1.195 o successivo. Nelle versioni precedenti viene valutato come un'espressione regolare non ancorata e si attiva anche per qualsiasi tipo di agente che lo contiene, come `prod-db-agent`; ancoratelo come `^db-agent$` su quelle versioni.

Consulti [Hooks](/docs/it/hooks) per il formato di configurazione dell'hook completo.

<h2 id="work-with-subagents">
  Lavori con i subagent
</h2>

<h3 id="understand-automatic-delegation">
  Comprenda la delegazione automatica
</h3>

Claude delega automaticamente le attività in base alla descrizione dell'attività nella sua richiesta, al campo `description` nelle configurazioni dei subagent e al contesto attuale. Per incoraggiare la delegazione proattiva, includa frasi come "use proactively" nel campo description del suo subagent.

<h3 id="invoke-subagents-explicitly">
  Invochi i subagent esplicitamente
</h3>

Quando la delegazione automatica non è sufficiente, può richiedere un subagent lei stesso. Tre modelli escalation da un suggerimento una tantum a un predefinito a livello di sessione:

* **Linguaggio naturale**: nomini il subagent nel suo prompt; Claude decide se delegare
* **@-mention**: garantisce che il subagent viene eseguito per un'attività
* **A livello di sessione**: l'intera sessione utilizza il prompt di sistema, le restrizioni di strumenti e il modello di quel subagent tramite il flag `--agent` o l'impostazione `agent`

Per il linguaggio naturale, non c'è sintassi speciale. Nomini il subagent e Claude generalmente delega:

```text wrap theme={null}
Use the test-runner subagent to fix failing tests
Have the code-reviewer subagent look at my recent changes
```

**@-mention il subagent.** Digiti `@` e scelga il subagent dal typeahead, nello stesso modo in cui @-mention i file. Questo assicura che quel subagent specifico viene eseguito piuttosto che lasciare la scelta a Claude:

```text wrap theme={null}
@"code-reviewer (agent)" look at the auth changes
```

Il suo messaggio completo va ancora a Claude, che scrive il prompt dell'attività del subagent in base a quello che ha chiesto. L'@-mention controlla quale subagent Claude invoca, non quale prompt riceve.

I subagent forniti da un [plugin](/docs/it/plugins) abilitato appaiono nel typeahead con il loro nome con ambito, come `my-plugin:code-reviewer` o `my-plugin:review:security` quando il plugin [organizza gli agenti in sottocartelle](#choose-the-subagent-scope). I subagent in background denominati attualmente in esecuzione nella sessione appaiono anche nel typeahead, mostrando il loro stato accanto al nome.

Può anche digitare la mention manualmente senza usare il picker: `@agent-<name>` per i subagent locali, o `@agent-` seguito dal nome con ambito per i subagent plugin, ad esempio `@agent-my-plugin:code-reviewer`.

**Esegua l'intera sessione come un subagent.** Passi [`--agent <name>`](/docs/it/cli-reference) per avviare una sessione in cui il thread principale stesso assume il prompt di sistema, le restrizioni di strumenti e il modello di quel subagent:

```bash theme={null}
claude --agent code-reviewer
```

Il prompt di sistema del subagent sostituisce completamente il prompt di sistema predefinito di Claude Code, nello stesso modo in cui [`--system-prompt`](/docs/it/cli-reference) fa. I file `CLAUDE.md` e la memoria del progetto continuano a caricarsi attraverso il flusso di messaggi normale. Il nome dell'agente appare come `@<name>` nell'intestazione di avvio in modo che possa confermare che è attivo.

Questo funziona con i subagent integrati e personalizzati, e la scelta persiste quando riprende la sessione.

Per un subagent fornito da un plugin, può passare solo il nome dell'agente e Claude Code lo troverà:

```bash theme={null}
claude --agent security-reviewer
```

Se più plugin forniscono agenti con lo stesso nome, passi il nome con ambito per disambiguare:

```bash theme={null}
claude --agent my-plugin:security-reviewer
```

Se il plugin posiziona l'agente in una sottocartella della sua directory `agents/`, includa la sottocartella nel nome con ambito, ad esempio `claude --agent my-plugin:review:security`.

Per renderlo il predefinito per ogni sessione in un progetto, imposti `agent` in `.claude/settings.json`:

```json theme={null}
{
  "agent": "code-reviewer"
}
```

Il flag CLI sostituisce l'impostazione se entrambi sono presenti.

<h3 id="run-subagents-in-foreground-or-background">
  Esegua i subagent in primo piano o in background
</h3>

I subagent possono essere eseguiti in primo piano o in background:

* **Subagent in primo piano** bloccano la conversazione principale fino al completamento. I prompt di autorizzazione vengono passati a lei mentre si presentano.
* **Subagent in background** vengono eseguiti contemporaneamente mentre continua a lavorare. A partire da v2.1.186, quando un subagent in background raggiunge una chiamata di strumento che necessita di autorizzazione, il prompt emerge nella sua sessione principale e nomina il subagent che sta chiedendo. Approvi per consentire al subagent di continuare, o premi Esc per negare quella singola chiamata di strumento senza fermare il subagent. Prima di v2.1.186, i subagent in background auto-negavano qualsiasi chiamata di strumento che avrebbe richiesto un prompt.

A partire da v2.1.198, i subagent vengono eseguiti in background per impostazione predefinita. Claude esegue un subagent in primo piano quando ha bisogno del risultato prima di continuare. L'impostazione predefinita cambia dove viene eseguito un subagent, non cosa gli è consentito fare: i subagent in background continuano a far emergere ogni prompt di autorizzazione nella sua sessione principale. Prima di v2.1.198, Claude sceglieva tra primo piano e background in base all'attività.

Può anche guidare questo lei stesso:

* Chieda a Claude di eseguire un'attività in background o in primo piano
* Premi **Ctrl+B** per mettere in background un'attività in esecuzione

Un subagent in background che si completa rimane elencato in [`/tasks`](/docs/it/commands), contrassegnato come completato e ordinato sotto il lavoro in esecuzione, fino a quando la sessione non pulisce il suo elenco di attività. La sua vista dettagliata rimane aperta quando il subagent termina. I subagent che falliscono o che lei ferma lasciano l'elenco. Prima di v2.1.208, un subagent completato lasciava l'elenco nel momento in cui terminava e la sua vista dettagliata si chiudeva.

Per disabilitare tutta la funzionalità di background task, imposti la variabile di ambiente `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` su `1`. Consulti [Environment variables](/docs/it/env-vars).

Quando [`CLAUDE_CODE_FORK_SUBAGENT`](#fork-the-current-conversation) è impostato su `1`, ogni spawn di subagent viene eseguito in background e il campo frontmatter `background` non ha effetto, perché la modalità fork rimuove il parametro `run_in_background` dallo strumento Agent. `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` ha la precedenza sulla modalità fork e mantiene gli spawn di subagent in primo piano.

<h3 id="api-errors-in-subagents">
  Errori API nei subagent
</h3>

A partire da v2.1.199, un subagent la cui esecuzione termina con un errore API, come un limite di utilizzo o un errore server ripetuto, segnala quel fallimento a Claude invece di restituire il testo di errore come se fossero i risultati del subagent. Quello che Claude riceve dipende da dove è stato eseguito il subagent:

* **Primo piano**: se un limite di velocità, un sovraccarico o un errore server interrompe un subagent che ha già prodotto output di testo, lo strumento Agent restituisce quell'output parziale con una nota che il subagent è stato interrotto e non ha completato la sua attività. Un subagent che non ha prodotto nulla, o il cui unico output erano chiamate di strumenti, fallisce con [`Agent terminated early due to an API error`](/docs/it/errors#agent-terminated-early-due-to-an-api-error), seguito dal dettaglio dell'errore. In v2.1.199, un limite di velocità, un sovraccarico o un errore server che ha interrotto la forma solo-tool-calls ha restituito un risultato parziale vuoto contenente solo la nota di interruzione.
* **Background**: il subagent è contrassegnato come fallito, e il messaggio che Claude riceve quando termina nomina l'errore API e include l'ultimo output del subagent, quindi il lavoro parziale non viene perso.

Una volta che l'errore API sottostante si risolve, chieda a Claude di riprovare l'attività o [riprendere il subagent](#resume-subagents).

<h3 id="common-patterns">
  Modelli comuni
</h3>

<h4 id="isolate-high-volume-operations">
  Isoli operazioni ad alto volume
</h4>

Uno degli usi più efficaci per i subagent è isolare le operazioni che producono grandi quantità di output. L'esecuzione di test, il recupero della documentazione o l'elaborazione di file di log possono consumare contesto significativo. Delegando questi a un subagent, l'output dettagliato rimane nel contesto del subagent mentre solo il riassunto rilevante ritorna alla sua conversazione principale.

```text wrap theme={null}
Use a subagent to run the test suite and report only the failing tests with their error messages
```

<h4 id="run-parallel-research">
  Esegua ricerca parallela
</h4>

Per indagini indipendenti, generi più subagent per lavorare simultaneamente:

```text wrap theme={null}
Research the authentication, database, and API modules in parallel using separate subagents
```

Ogni subagent esplora la sua area in modo indipendente, quindi Claude sintetizza i risultati. Questo funziona meglio quando i percorsi di ricerca non dipendono l'uno dall'altro.

<Warning>
  Quando i subagent completano, i loro risultati ritornano alla sua conversazione principale. L'esecuzione di molti subagent che ognuno restituisce risultati dettagliati può consumare contesto significativo.
</Warning>

Per attività che necessitano di parallelismo sostenuto o superano la sua finestra di contesto, [agent teams](/docs/it/agent-teams) danno a ogni worker il suo contesto indipendente.

<h4 id="chain-subagents">
  Concateni i subagent
</h4>

Per flussi di lavoro multi-step, chieda a Claude di utilizzare i subagent in sequenza. Ogni subagent completa la sua attività e restituisce i risultati a Claude, che poi passa il contesto rilevante al subagent successivo.

```text wrap theme={null}
Use the code-reviewer subagent to find performance issues, then use the optimizer subagent to fix them
```

<h3 id="choose-between-subagents-and-main-conversation">
  Scelga tra subagent e conversazione principale
</h3>

Usi la **conversazione principale** quando:

* L'attività necessita di frequenti scambi o raffinamento iterativo
* Più fasi condividono contesto significativo, come pianificazione, implementazione e test
* Sta facendo un cambio rapido e mirato
* La latenza è importante. I subagent iniziano da zero e potrebbero aver bisogno di tempo per raccogliere contesto

Usi **subagent** quando:

* L'attività produce output dettagliato che non ha bisogno nel suo contesto principale
* Vuole applicare restrizioni di strumenti o autorizzazioni specifiche
* Il lavoro è autonomo e può restituire un riassunto

Consideri [Skills](/docs/it/skills) invece quando vuole prompt o flussi di lavoro riutilizzabili che vengono eseguiti nel contesto della conversazione principale piuttosto che nel contesto isolato del subagent.

Per una domanda rapida su qualcosa già nella sua conversazione, usi [`/btw`](/docs/it/interactive-mode#side-questions-with-%2Fbtw) invece di un subagent. Vede il suo contesto completo ma non ha accesso agli strumenti e la risposta viene scartata piuttosto che aggiunta alla cronologia.

<h3 id="spawn-nested-subagents">
  Generi subagent annidati
</h3>

A partire da Claude Code v2.1.172, un subagent può generare i suoi propri subagent. Usi questo quando un'attività delegata si divide in sottoattività parallele, come un subagent revisore che invia un verificatore per ogni risultato, in modo che l'output intermedio non raggiunga mai la sua conversazione principale. Solo il riassunto del subagent di livello superiore ritorna a lei.

Un subagent annidato è configurato nello stesso modo di uno di livello superiore e si risolve dagli stessi [ambiti](#choose-the-subagent-scope).

La profondità viene conteggiata come il numero di livelli di subagent sotto la conversazione principale, indipendentemente dal fatto che ogni livello venga eseguito in [primo piano o in background](#run-subagents-in-foreground-or-background). Un subagent a profondità cinque non riceve lo strumento Agent e non può generare ulteriormente. Il limite è fisso e non configurabile.

A partire da Claude Code v2.1.187, la profondità di un subagent in background è fissata quando viene generato per la prima volta, e [riprendere](#resume-subagents) successivamente non cambia quella profondità. Ad esempio, se la sua conversazione principale genera il subagent A, e A genera un subagent in background B a profondità due, B è ancora a profondità due quando lo riprende direttamente dalla conversazione principale. Riprendere un subagent da un contesto più superficiale non gli consente di generare livelli aggiuntivi che il limite di profondità ha già impedito.

Per impedire a un subagent specifico di generare altri, ometta `Agent` dal suo elenco [`tools`](#available-tools) o aggiunga a `disallowedTools`.

Una [fork](#fork-the-current-conversation) ancora non può generare un'altra fork. Può generare altri tipi di subagent, e questi contano verso il limite di profondità.

<h3 id="manage-subagent-context">
  Gestisca il contesto del subagent
</h3>

<h4 id="what-loads-at-startup">
  Cosa si carica all'avvio
</h4>

Ogni subagent inizia con una finestra di contesto fresca e isolata. Non vede la cronologia della sua conversazione, le skills che ha già invocato, o i file che Claude ha già letto. Claude compone un messaggio di delegazione che riassume l'attività, e il subagent lavora da lì. L'eccezione è una [fork](#fork-the-current-conversation), che eredita la conversazione genitore invece di iniziare da zero.

Il contesto iniziale di un subagent non-fork contiene:

* **System prompt**: il prompt dell'agente stesso più i dettagli dell'ambiente che Claude Code aggiunge, non il prompt di sistema completo di Claude Code. I subagent personalizzati definiscono il loro nel [corpo markdown](#write-subagent-files) o nel campo `prompt`. Gli agenti integrati hanno prompt predefiniti.
* **Task message**: il prompt di delegazione che Claude scrive quando consegna il lavoro.
* **CLAUDE.md e memory**: ogni livello della [gerarchia di memoria](/docs/it/memory#how-claude-md-files-load) che la conversazione principale carica, inclusi `~/.claude/CLAUDE.md`, regole del progetto, `CLAUDE.local.md` e file di policy gestiti. Gli agenti Explore e Plan integrati saltano questo.
* **Git status**: uno snapshot preso all'inizio della sessione genitore. Assente quando la directory di lavoro non è un repository Git o quando [`includeGitInstructions`](/docs/it/settings#available-settings) è `false`. Explore e Plan lo saltano comunque.
* **Preloaded skills**: contenuto completo di qualsiasi skill denominata nel campo [`skills`](#preload-skills-into-subagents) dell'agente. Gli agenti integrati non precaricano skills.
* **Sibling roster**: un promemoria di sistema che elenca `main` e ogni altro agente denominato nella sessione, ognuno un valore `to` valido per [`SendMessage`](#resume-subagents). Richiede Claude Code v2.1.206 o successivo. L'elenco appare solo quando gli strumenti del subagent includono `SendMessage` e almeno un altro agente ha un nome, sia che Claude lo abbia denominato quando lo ha generato o che venga eseguito come un collega [agent teams](/docs/it/agent-teams). È uno snapshot preso quando il subagent inizia, quindi gli agenti denominati successivamente non appaiono.

Explore e Plan sono gli unici subagent che omettono CLAUDE.md e git status. Non c'è un campo frontmatter o un'impostazione per-agente per cambiare quali agenti li saltano.

La conversazione principale legge i risultati di Explore e Plan con il contesto completo di CLAUDE.md, quindi la maggior parte delle regole non ha bisogno di raggiungere il subagent stesso. Se una regola deve, come "ignora la directory `vendor/`", la rienunci nel prompt che dà a Claude quando delega.

<h4 id="resume-subagents">
  Riprenda i subagent
</h4>

Ogni invocazione di subagent crea una nuova istanza con contesto fresco. Per continuare il lavoro di un subagent esistente invece di ricominciare, chieda a Claude di riprendere.

I subagent ripresi mantengono la loro cronologia di conversazione completa, incluse tutte le precedenti chiamate di strumenti, risultati e ragionamento. Il subagent riprende esattamente da dove si era fermato piuttosto che ricominciare da zero.

Quando un subagent completa, Claude riceve il suo ID agente. Gli agenti integrati Explore e Plan sono una tantum e non restituiscono alcun ID agente, quindi non possono essere ripresi; usi `general-purpose` o un subagent personalizzato quando ha bisogno di continuare il lavoro.

Claude utilizza lo strumento `SendMessage` con l'ID dell'agente o il nome come campo `to` per riprendere. `SendMessage` non richiede che [agent teams](/docs/it/agent-teams) siano abilitati; solo i messaggi strutturati del protocollo di team come `shutdown_request` e `plan_approval_response` lo fanno.

Per riprendere un subagent, chieda a Claude di continuare il lavoro precedente:

```text wrap theme={null}
Use the code-reviewer subagent to review the authentication module
[Agent completes]

Continue that code review and now analyze the authorization logic
[Claude resumes the subagent with full context from previous conversation]
```

Un subagent completato che riceve un `SendMessage` si auto-riprende in background senza una nuova invocazione `Agent`. Lo stesso vale per un subagent che Claude ha fermato con lo strumento `TaskStop`.

A partire da v2.1.191, un subagent che lei ha fermato lei stesso, con `x` in `/tasks` o una richiesta SDK `stop_task`, non si auto-riprende. La chiamata `SendMessage` restituisce un rifiuto dicendo a Claude che l'agente è stato annullato. Digiti nel trascritto di quel subagent nel pannello subagent per riprendere lei stesso, il che cancella lo stop in modo che le successive chiamate `SendMessage` possono auto-riprendere di nuovo.

Riprendere avvia una nuova esecuzione dell'agente con lo stesso ID, quindi un subagent che aveva già fallito o completato si mostra come in esecuzione di nuovo nell'elenco delle attività e negli eventi delle attività dell'Agent SDK. Prima di v2.1.205, continuava a mostrare il suo stato precedente fallito o completato mentre l'esecuzione ripresa stava funzionando.

A partire da v2.1.199, `SendMessage` verifica che un nome si riferisca ancora allo stesso agente che ha raggiunto in precedenza nella conversazione. Se un agente più recente ha preso il nome, come un agente in background ri-generato che lo ha riutilizzato, Claude Code rifiuta l'invio piuttosto che consegnarlo all'agente sbagliato, e l'errore segnala quale agente il nome raggiunge ora in modo che Claude possa reindirizzare. Per raggiungere l'agente precedente mentre è ancora in esecuzione, Claude lo indirizza per l'ID agente dal risultato di spawn. Il controllo è limitato alla conversazione attuale e si ripristina su `/clear`.

A partire da v2.1.198, un subagent tratta i messaggi dall'agente che lo ha lanciato come direzione di attività normale, incluse le correzioni di corso a metà attività, e agisce su di essi all'interno delle sue impostazioni di autorizzazione. Due limiti continuano a valere indipendentemente da chi ha inviato il messaggio: nessun messaggio da alcun agente conta come la sua approvazione per un prompt di autorizzazione in sospeso, e nessun messaggio di agente può cambiare le impostazioni di autorizzazione, `CLAUDE.md` o configurazione di un subagent. Solo il sistema di autorizzazione o i suoi stessi messaggi possono concedere l'approvazione.

Può anche chiedere a Claude l'ID agente se vuole fare riferimento ad esso esplicitamente, o trovare gli ID nei file di trascrizione in `~/.claude/projects/{project}/{sessionId}/subagents/`. Ogni trascrizione è archiviata come `agent-{agentId}.jsonl`.

Le trascrizioni dei subagent persistono indipendentemente dalla conversazione principale:

* **Compattazione della conversazione principale**: quando la conversazione principale si compatta, le trascrizioni dei subagent non sono interessate. Sono archiviate in file separati.
* **Persistenza della sessione**: le trascrizioni dei subagent persistono all'interno della loro sessione. Può [riprendere un subagent](#resume-subagents) dopo aver riavviato Claude Code riprendendo la stessa sessione.
* **Pulizia automatica**: le trascrizioni vengono pulite in base all'impostazione `cleanupPeriodDays`, che per impostazione predefinita è 30 giorni.

<h4 id="auto-compaction">
  Auto-compattazione
</h4>

I subagent supportano la compattazione automatica utilizzando la stessa logica della conversazione principale. La compattazione si attiva nelle stesse condizioni, e `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` si applica anche ai subagent. Consulti [environment variables](/docs/it/env-vars) per quando l'override ha effetto.

Gli eventi di compattazione vengono registrati nei file di trascrizione dei subagent:

```json theme={null}
{
  "type": "system",
  "subtype": "compact_boundary",
  "compactMetadata": {
    "trigger": "auto",
    "preTokens": 167189
  }
}
```

Il valore `preTokens` mostra quanti token sono stati utilizzati prima che si verificasse la compattazione.

<h2 id="fork-the-current-conversation">
  Esegua il fork della conversazione corrente
</h2>

<Note>
  I subagent di fork richiedono Claude Code v2.1.117 o successivo. Da v2.1.161 il comando `/fork` è abilitato per impostazione predefinita; nelle versioni precedenti richiede l'impostazione della variabile di ambiente [`CLAUDE_CODE_FORK_SUBAGENT`](/docs/it/env-vars) su `1`. Consentire a Claude stesso di generare fork è sperimentale e potrebbe cambiare nelle versioni future. Questa capacità può anche essere abilitata nelle sessioni interattive come parte di un rollout graduale.
</Note>

Un fork è un subagent che eredita l'intera conversazione fino ad ora invece di iniziare da zero. Questo elimina l'isolamento dell'input che i subagent altrimenti forniscono: un fork vede lo stesso prompt di sistema, strumenti, modello e cronologia dei messaggi della sessione principale, in modo che possa assegnargli un'attività secondaria senza re-spiegare la situazione. Le proprie chiamate di strumenti del fork rimangono comunque fuori dalla sua conversazione e solo il suo risultato finale ritorna, in modo che la sua finestra di contesto principale rimanga pulita. Usi un fork quando un subagent denominato avrebbe bisogno di troppo background per essere utile, o quando vuole provare diversi approcci in parallelo dallo stesso punto di partenza.

Per controllare la modalità fork indipendentemente dal rollout graduale, imposti [`CLAUDE_CODE_FORK_SUBAGENT`](/docs/it/env-vars) su `1` per abilitarla esplicitamente o su `0` per disabilitarla. La variabile è rispettata in modalità interattiva e tramite SDK o `claude -p`.

L'abilitazione della modalità fork cambia Claude Code in due modi:

* Claude può generare un fork richiedendo esplicitamente il tipo di subagent `fork`. Gli spawn senza un tipo di subagent continuano a utilizzare il subagent [general-purpose](#built-in-subagents), e i subagent denominati come Explore continuano a generarsi come prima.
* Ogni spawn di subagent viene eseguito in [background](#run-subagents-in-foreground-or-background), sia che sia un fork che un subagent denominato. Imposti `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` su `1` per mantenere gli spawn sincroni.

Può avviare un fork lei stesso con `/fork` seguito da una direttiva, con o senza la variabile impostata. Claude Code nomina il fork dalle prime parole della direttiva. L'esempio seguente esegue il fork della conversazione per redigere casi di test mentre continua con l'implementazione nella sessione principale:

```text wrap theme={null}
/fork draft unit tests for the parser changes so far
```

Il fork appare in un pannello sotto il suo prompt e viene eseguito in background mentre continua a lavorare. Quando finisce, il suo risultato arriva come messaggio nella sua conversazione principale. La sezione successiva copre i controlli del pannello per osservare e dirigere i fork mentre vengono eseguiti.

<h3 id="observe-and-steer-running-forks">
  Osservi e dirija i fork in esecuzione
</h3>

I fork in esecuzione appaiono in un pannello sotto l'input del prompt, con una riga per la sessione principale e una per ogni fork. Usi questi tasti per interagire con il pannello:

| Key       | Action                                                                  |
| :-------- | :---------------------------------------------------------------------- |
| `↑` / `↓` | Sposta tra le righe                                                     |
| `Enter`   | Apra la trascrizione del fork selezionato e invii messaggi di follow-up |
| `x`       | Chiuda un fork finito o fermi uno in esecuzione                         |
| `Esc`     | Restituisca il focus all'input del prompt                               |

Con la trascrizione di un fork o di un subagent aperta, i messaggi di follow-up e le [skills](/docs/it/skills) vanno a quell'agente, ma i comandi incorporati continuano a essere eseguiti nella sua conversazione principale. A partire da v2.1.199, digitando `/model` o `/fast` in quella visualizzazione viene visualizzato un avviso che cambia il modello della conversazione principale o la modalità veloce, non quello dell'agente visualizzato, invece di eseguirlo silenziosamente.

<h3 id="how-forks-differ-from-named-subagents">
  Come i fork differiscono dai subagent denominati
</h3>

Un fork eredita tutto ciò che la sessione principale ha nel momento in cui viene generato. Un subagent denominato inizia dalla sua propria definizione.

|                       | Fork                                 | Subagent denominato                                                                                                               |
| :-------------------- | :----------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------- |
| Context               | Cronologia di conversazione completa | Contesto fresco con il prompt che passa                                                                                           |
| System prompt e tools | Uguale alla sessione principale      | Dalla [definition file](#write-subagent-files) del subagent                                                                       |
| Model                 | Uguale alla sessione principale      | Dal campo `model` del subagent                                                                                                    |
| Permissions           | I prompt emergono nel suo terminale  | [I prompt emergono nella sua sessione principale](#run-subagents-in-foreground-or-background) quando viene eseguito in background |
| Prompt cache          | Condiviso con la sessione principale | Cache separata                                                                                                                    |

Poiché il prompt di sistema di un fork e le definizioni di strumenti sono identici al principale, la sua prima richiesta riutilizza la cache del prompt del principale. Questo rende il fork più economico rispetto alla generazione di un subagent fresco per attività che necessitano dello stesso contesto.

Quando Claude genera un fork tramite lo strumento Agent, può passare `isolation: "worktree"` in modo che le modifiche ai file del fork vengano scritte in un git worktree separato invece del suo checkout.

<h3 id="limitations">
  Limitazioni
</h3>

L'impostazione di `CLAUDE_CODE_FORK_SUBAGENT=1` abilita la modalità fork in sessioni interattive, [modalità non interattiva](/docs/it/headless) e SDK Agent; l'impostazione su `0` disabilita la modalità fork ovunque, incluso qualsiasi rollout lato server. Un fork non può generare ulteriori fork.

<h2 id="example-subagents">
  Subagent di esempio
</h2>

Questi esempi dimostrano modelli efficaci per la costruzione di subagent. Li usi come punti di partenza, o generi una versione personalizzata con Claude.

<Tip>
  **Best practices:**

  * **Progetti subagent focalizzati:** ogni subagent dovrebbe eccellere in un'attività specifica
  * **Scriva descrizioni dettagliate:** Claude utilizza la descrizione per decidere quando delegare
  * **Limiti l'accesso agli strumenti:** conceda solo le autorizzazioni necessarie per la sicurezza e la focalizzazione
  * **Archivi nel controllo della versione:** condivida i subagent di progetto con il suo team
</Tip>

<h3 id="code-reviewer">
  Revisore di codice
</h3>

Un subagent di sola lettura che esamina il codice senza modificarlo. Questo esempio mostra come progettare un subagent focalizzato con accesso limitato agli strumenti che esclude Edit e Write, e un prompt dettagliato che specifica esattamente cosa cercare e come formattare l'output.

```markdown theme={null}
---
name: code-reviewer
description: Expert code review specialist. Proactively reviews code for quality, security, and maintainability. Use immediately after writing or modifying code.
tools: Read, Grep, Glob, Bash
model: inherit
---

You are a senior code reviewer ensuring high standards of code quality and security.

When invoked:
1. Run git diff to see recent changes
2. Focus on modified files
3. Begin review immediately

Review checklist:
- Code is clear and readable
- Functions and variables are well-named
- No duplicated code
- Proper error handling
- No exposed secrets or API keys
- Input validation implemented
- Good test coverage
- Performance considerations addressed

Provide feedback organized by priority:
- Critical issues (must fix)
- Warnings (should fix)
- Suggestions (consider improving)

Include specific examples of how to fix issues.
```

<h3 id="debugger">
  Debugger
</h3>

Un subagent che può sia analizzare che correggere i problemi. A differenza del revisore di codice, questo include Edit perché correggere i bug richiede la modifica del codice. Il prompt fornisce un flusso di lavoro chiaro dalla diagnosi alla verifica.

```markdown theme={null}
---
name: debugger
description: Debugging specialist for errors, test failures, and unexpected behavior. Use proactively when encountering any issues.
tools: Read, Edit, Bash, Grep, Glob
---

You are an expert debugger specializing in root cause analysis.

When invoked:
1. Capture error message and stack trace
2. Identify reproduction steps
3. Isolate the failure location
4. Implement minimal fix
5. Verify solution works

Debugging process:
- Analyze error messages and logs
- Check recent code changes
- Form and test hypotheses
- Add strategic debug logging
- Inspect variable states

For each issue, provide:
- Root cause explanation
- Evidence supporting the diagnosis
- Specific code fix
- Testing approach
- Prevention recommendations

Focus on fixing the underlying issue, not the symptoms.
```

<h3 id="data-scientist">
  Data scientist
</h3>

Un subagent specifico del dominio per il lavoro di analisi dei dati. Questo esempio mostra come creare subagent per flussi di lavoro specializzati al di fuori dei tipici compiti di codifica. Imposta esplicitamente `model: sonnet` per un'analisi più capace.

```markdown theme={null}
---
name: data-scientist
description: Data analysis expert for SQL queries, BigQuery operations, and data insights. Use proactively for data analysis tasks and queries.
tools: Bash, Read, Write
model: sonnet
---

You are a data scientist specializing in SQL and BigQuery analysis.

When invoked:
1. Understand the data analysis requirement
2. Write efficient SQL queries
3. Use BigQuery command line tools (bq) when appropriate
4. Analyze and summarize results
5. Present findings clearly

Key practices:
- Write optimized SQL queries with proper filters
- Use appropriate aggregations and joins
- Include comments explaining complex logic
- Format results for readability
- Provide data-driven recommendations

For each analysis:
- Explain the query approach
- Document any assumptions
- Highlight key findings
- Suggest next steps based on data

Always ensure queries are efficient and cost-effective.
```

<h3 id="database-query-validator">
  Validatore di query di database
</h3>

Un subagent che consente l'accesso a Bash ma convalida i comandi per consentire solo query SQL di sola lettura. Questo esempio mostra come usare gli hook `PreToolUse` per la convalida condizionale quando ha bisogno di un controllo più fine di quello che il campo `tools` fornisce.

```markdown theme={null}
---
name: db-reader
description: Execute read-only database queries. Use when analyzing data or generating reports.
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---

You are a database analyst with read-only access. Execute SELECT queries to answer questions about the data.

When asked to analyze data:
1. Identify which tables contain the relevant data
2. Write efficient SELECT queries with appropriate filters
3. Present results clearly with context

You cannot modify data. If asked to INSERT, UPDATE, DELETE, or modify schema, explain that you only have read access.
```

Claude Code [passa l'input dell'hook come JSON](/docs/it/hooks#pretooluse-input) tramite stdin ai comandi dell'hook. Lo script di convalida legge questo JSON, estrae il comando in esecuzione e lo controlla rispetto a un elenco di operazioni di scrittura SQL. Se viene rilevata un'operazione di scrittura, lo script [esce con codice 2](/docs/it/hooks#exit-code-2-behavior-per-event) per bloccare l'esecuzione e restituisce un messaggio di errore a Claude tramite stderr.

Crei lo script di convalida in qualsiasi punto del suo progetto. Il percorso deve corrispondere al campo `command` nella sua configurazione dell'hook:

```bash theme={null}
#!/bin/bash
# Blocks SQL write operations, allows SELECT queries

# Read JSON input from stdin
INPUT=$(cat)

# Extract the command field from tool_input using jq
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

if [ -z "$COMMAND" ]; then
  exit 0
fi

# Block write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE|REPLACE|MERGE)\b' > /dev/null; then
  echo "Blocked: Write operations not allowed. Use SELECT queries only." >&2
  exit 2
fi

exit 0
```

Su macOS e Linux, renda lo script eseguibile:

```bash theme={null}
chmod +x ./scripts/validate-readonly-query.sh
```

Su Windows, scriva lo script di convalida in PowerShell e aggiunga `shell: powershell` alla voce dell'hook. Consulti [esecuzione degli hook in PowerShell](/docs/it/hooks#windows-powershell-tool).

L'hook riceve JSON tramite stdin con il comando Bash in `tool_input.command`. Il codice di uscita 2 blocca l'operazione e alimenta il messaggio di errore a Claude. Consulti [Hooks](/docs/it/hooks#exit-code-output) per i dettagli sui codici di uscita e [Hook input](/docs/it/hooks#pretooluse-input) per lo schema di input completo.

<h2 id="next-steps">
  Passaggi successivi
</h2>

Ora che comprende i subagent, esplori queste funzionalità correlate:

* [Distribuisca subagent con i plugin](/docs/it/plugins) per condividere i subagent tra team o progetti
* [Esegua Claude Code a livello di programmazione](/docs/it/headless) con l'Agent SDK per CI/CD e automazione
* [Usi i server MCP](/docs/it/mcp) per dare ai subagent accesso a strumenti e dati esterni
