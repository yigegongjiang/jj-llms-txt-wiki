> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Estendi Claude con skills

> Crea, gestisci e condividi skills per estendere le capacità di Claude in Claude Code. Include comandi personalizzati e skills raggruppate.

Skills estendono ciò che Claude può fare. Crea un file `SKILL.md` con istruzioni, e Claude lo aggiunge al suo toolkit. Claude utilizza skills quando rilevante, oppure puoi invocare uno direttamente con `/skill-name`.

Crea una skill quando continui a incollare le stesse istruzioni, checklist o procedure multi-step nella chat, oppure quando una sezione di CLAUDE.md è cresciuta fino a diventare una procedura piuttosto che un fatto. A differenza del contenuto di CLAUDE.md, il corpo di una skill si carica solo quando viene utilizzato, quindi il materiale di riferimento lungo costa quasi nulla fino a quando non ne hai bisogno.

<Note>
  Per i comandi integrati come `/help` e `/compact`, e per le skills raggruppate come `/debug` e `/code-review`, consulta il [riferimento dei comandi](/docs/it/commands).

  **I comandi personalizzati sono stati uniti alle skills.** Un file in `.claude/commands/deploy.md` e una skill in `.claude/skills/deploy/SKILL.md` creano entrambi `/deploy` e funzionano allo stesso modo. I tuoi file `.claude/commands/` esistenti continuano a funzionare. Le skills aggiungono funzionalità opzionali: una directory per i file di supporto, frontmatter per [controllare se tu o Claude le invocate](#control-who-invokes-a-skill), e la capacità per Claude di caricarle automaticamente quando rilevante.
</Note>

Le skills di Claude Code seguono lo standard aperto [Agent Skills](https://agentskills.io), che funziona su più strumenti AI. Claude Code estende lo standard con funzionalità aggiuntive come il [controllo dell'invocazione](#control-who-invokes-a-skill), l'[esecuzione di subagent](#run-skills-in-a-subagent), e l'[iniezione di contesto dinamico](#inject-dynamic-context). Consulta [Utilizzo del frontmatter delle skills al di fuori di Claude Code](#using-skill-frontmatter-outside-claude-code) per sapere quali campi frontmatter fanno parte dello standard e quali sono estensioni di Claude Code.

<h2 id="bundled-skills">
  Skill raggruppati
</h2>

Claude Code include un set di skill raggruppati, come `/doctor`, `/code-review`, `/batch`, `/debug`, `/loop`, e `/claude-api`. Gli skill raggruppati sono basati su prompt: forniscono a Claude istruzioni dettagliate e gli permettono di orchestrare il lavoro utilizzando i suoi strumenti. La maggior parte dei comandi built-in invece eseguono logica fissa direttamente.

Invocate uno skill raggruppato nello stesso modo di qualsiasi altro skill, digitando `/` seguito dal nome dello skill. Claude invoca alcuni skill raggruppati automaticamente quando rilevante; altri, incluso `/verify`, vengono eseguiti solo quando li invocate, il che vi mantiene in controllo di quando questi controlli più lunghi spendono tempo e token.

La maggior parte degli skill raggruppati sono disponibili in ogni sessione. Alcuni dipendono da una funzionalità specifica: `/workflow-authoring`, ad esempio, è disponibile solo quando i [dynamic workflows](/docs/it/workflows) sono abilitati.

Per disattivare gli skill raggruppati, utilizzate l'impostazione [`disableBundledSkills`](/docs/it/settings-reference#disablebundledskills).

<Note>
  Il controllo di configurazione [`/doctor`](/docs/it/commands#all-commands) rimane digitabile quando `disableBundledSkills` è attivo, in Claude Code v2.1.205 e successivi. Per nasconderlo, impostate la variabile d'ambiente `DISABLE_DOCTOR_COMMAND` o una voce [`skillOverrides`](#override-skill-visibility-from-settings) di `"doctor": "off"`. Prima della v2.1.205, `/doctor` era un comando built-in piuttosto che uno skill raggruppato.
</Note>

Gli skill raggruppati sono elencati insieme ai comandi built-in nel [riferimento dei comandi](/docs/it/commands), contrassegnati come **Skill** nella colonna Purpose.

<h3 id="run-and-verify-your-app">
  Eseguire e verificare la vostra app
</h3>

Tre skill raggruppati lavorano insieme per lanciare la vostra app e confermare i cambiamenti rispetto all'app in esecuzione invece di solo test:

| Skill                  | Purpose                                                                                                                                            |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/run`                 | Lanciare e guidare la vostra app per vedere un cambiamento funzionante                                                                             |
| `/verify`              | Compilare ed eseguire la vostra app per confermare che un cambiamento di codice fa quello che dovrebbe, senza ricorrere a test o controlli di tipo |
| `/run-skill-generator` | Insegnare a `/run` e `/verify` come compilare e lanciare il vostro progetto                                                                        |

`/run` e `/verify` funzionano senza configurazione. Deducono il lancio dal tipo di progetto (CLI, server, TUI, browser-driven) e da ciò che si trova nel vostro README, `package.json`, o `Makefile`. Quella deduzione diventa inaffidabile per progetti che necessitano di qualcosa oltre a un lancio standard: un database, un file env, una sessione grafica, una compilazione multi-step.

`/run-skill-generator` registra la ricetta invece. Fa funzionare la vostra app da un ambiente pulito, cattura ciò che ha funzionato (i comandi di installazione, le variabili d'ambiente, lo script di lancio), e lo commit come skill per-progetto in `.claude/skills/run-<name>/`. Dopo di ciò, `/run`, `/verify`, e qualsiasi altro agente nel repository seguono la ricetta registrata invece di riscoprirla. Eseguite `/run-skill-generator` una volta per progetto, e di nuovo se il processo di compilazione o lancio cambia.

`/verify` può anche registrare la sua propria ricetta. Quando deve compilare e guidare la vostra app senza una ricetta registrata, scrive ciò che ha funzionato in `.claude/skills/verify/SKILL.md` alla radice del repository, o nella directory del pacchetto toccato in un monorepo, così le esecuzioni successive e altri agenti seguono gli stessi step. Alla radice del repository, lo skill registrato sostituisce il `/verify` raggruppato. Questo richiede Claude Code v2.1.200 o successivo.

Claude modifica il file registrato solo quando ha guidato male un'esecuzione, come un comando che ha fallito o uno step mancante, così potete commit il file senza diff per-sessione. Prima della v2.1.205, lo skill raggruppato ha detto a Claude di incorporare qualsiasi cosa un'esecuzione ha imparato, il che ha causato frequenti conflitti di merge.

<h2 id="getting-started">
  Iniziare
</h2>

<h3 id="create-your-first-skill">
  Crea la tua prima skill
</h3>

Questo esempio crea una skill che riassume i cambiamenti non committati nel tuo repository git e segnala qualsiasi cosa rischiosa. Estrae il diff live nel prompt prima che Claude lo legga, quindi la risposta è basata sul tuo albero di lavoro effettivo piuttosto che su quello che Claude può indovinare dai file aperti. Claude carica la skill automaticamente quando chiedi dei tuoi cambiamenti, oppure puoi invocarla direttamente con `/summarize-changes`.

<Steps>
  <Step title="Crea la directory della skill">
    Crea una directory per la skill nella tua cartella skills personale. Le skill personali sono disponibili in tutti i tuoi progetti.

    ```bash theme={null}
    mkdir -p ~/.claude/skills/summarize-changes
    ```
  </Step>

  <Step title="Scrivi SKILL.md">
    Ogni skill ha bisogno di un file `SKILL.md` con due parti: frontmatter YAML tra i marcatori `---` che dice a Claude quando usare la skill, e contenuto markdown con le istruzioni che Claude segue quando la skill viene eseguita. Il nome della directory diventa il comando che digiti, e la `description` aiuta Claude a decidere quando caricare la skill automaticamente.

    Salva questo in `~/.claude/skills/summarize-changes/SKILL.md`:

    ```yaml theme={null}
    ---
    description: Summarizes uncommitted changes and flags anything risky. Use when the user asks what changed, wants a commit message, or asks to review their diff.
    ---

    ## Current changes

    !`git diff HEAD`

    ## Instructions

    Summarize the changes above in two or three bullet points, then list any risks you notice such as missing error handling, hardcoded values, or tests that need updating. If the diff is empty, say there are no uncommitted changes.
    ```

    La riga `` !`git diff HEAD` `` usa [dynamic context injection](#inject-dynamic-context): Claude Code esegue il comando e sostituisce la riga con il suo output prima che Claude veda il contenuto della skill, quindi le istruzioni arrivano con il diff attuale già inline.
  </Step>

  <Step title="Testa la skill">
    Apri un progetto git, fai una piccola modifica a qualsiasi file, e avvia Claude Code eseguendo `claude`. Puoi testare la skill in due modi.

    **Lascia che Claude la invochi automaticamente** chiedendo qualcosa che corrisponda alla descrizione:

    ```text theme={null}
    What did I change?
    ```

    **Oppure invocala direttamente** con il nome della skill:

    ```text theme={null}
    /summarize-changes
    ```

    In entrambi i casi, Claude dovrebbe rispondere con un breve riassunto della tua modifica e un elenco di rischi.
  </Step>
</Steps>

<h2 id="where-skills-live">
  Scegliere dove caricano le skills
</h2>

Il luogo in cui salvi una skill determina quali sessioni la caricano. Salvala nella tua directory home per ottenerla in ogni progetto, committala in un repository per condividerla con tutti coloro che lavorano lì, oppure distribuiscila tramite un plugin o impostazioni gestite per raggiungere un intero team.

| Posizione            | Percorso                                                                                                                      | Carica in                                                                                                                                                                                                           |
| :------------------- | :---------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Enterprise           | `.claude/skills/<skill-name>/SKILL.md` nella [directory delle impostazioni gestite](/docs/it/managed-settings#delivery-mechanisms) | Tutti gli utenti su macchine in cui la tua organizzazione la distribuisce                                                                                                                                           |
| Personale            | `~/.claude/skills/<skill-name>/SKILL.md`                                                                                      | Tutti i tuoi progetti su questa macchina, ma non [sessioni Cowork o cloud](#skills-in-cowork-and-cloud-sessions)                                                                                                    |
| Progetto             | `.claude/skills/<skill-name>/SKILL.md`                                                                                        | Sessioni in questo repository. Committala in modo che il tuo team la ottenga anche                                                                                                                                  |
| Annidato             | `<subdir>/.claude/skills/<skill-name>/SKILL.md`                                                                               | Sessioni avviate in o sotto `<subdir>`. Una sessione avviata sopra di essa carica la skill una volta che Claude lavora su file lì. Vedi [monorepos e sottodirectory](#discovery-from-parent-and-nested-directories) |
| Directory aggiuntiva | `.claude/skills/<skill-name>/SKILL.md` in una directory che passi con `--add-dir`                                             | Quella sessione. Vedi [directory al di fuori del progetto](#skills-from-additional-directories)                                                                                                                     |
| Plugin               | `<plugin>/skills/<skill-name>/SKILL.md`                                                                                       | Ovunque il [plugin](/docs/it/plugins/overview) sia abilitato, come `/plugin-name:skill-name`                                                                                                                             |
| Account claude.ai    | Skills abilitate per il tuo account claude.ai                                                                                 | Sessioni Cowork, sessioni cloud e sessioni di terminale in cui accedi con quell'account. Vedi [Skills sincronizzate da claude.ai](#how-synced-skills-behave)                                                        |

Le cartelle delle skill seguono anche queste regole:

* **Cartelle con symlink**: una voce `<skill-name>` nella posizione enterprise, personale o di progetto può essere un symlink a una directory altrove su disco. Claude Code legge `SKILL.md` dal target e carica la skill una sola volta anche se più posizioni puntano allo stesso target. Le skill dei plugin [gestiscono i symlink diversamente](/docs/it/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks).
* **Nome riservato**: non nominare una cartella di skill `synced`, in nessuna capitalizzazione. Claude Code usa `~/.claude/skills/synced/` per [skills scaricate da claude.ai](#where-synced-skills-load) e salta una skill che crei con quel nome nelle posizioni enterprise, personale e di progetto.
* **File di comando**: un file Markdown in `.claude/commands/` è il formato più vecchio e funziona ancora. Supporta lo stesso [frontmatter](#frontmatter-reference) tranne `name` e `paths`. Per trovare il nome che digiti per invocarlo, vedi [Come una skill ottiene il suo nome di comando](#how-a-skill-gets-its-command-name). Preferisci una skill per il nuovo lavoro, poiché le skill supportano anche [file di supporto](#add-supporting-files).
* **Cartella di skill come plugin**: aggiungi un `.claude-plugin/plugin.json` a una cartella di skill e si carica come [plugin](/docs/it/plugins/loading#plugins-shared-through-a-repository) denominato `<name>@skills-dir`, in modo che possa raggruppare agenti, hooks e server MCP. In un `.claude/skills/` di un progetto, questo richiede di accettare prima la finestra di dialogo di fiducia dell'area di lavoro.

<h3 id="discovery-from-parent-and-nested-directories">
  Carica skills in monorepos e sottodirectory
</h3>

Claude Code carica le skill di progetto da `.claude/skills/` nella directory in cui lo avvii e in ogni directory padre fino alla radice del repository, quindi avviare in `packages/frontend/` raccoglie comunque le skill definite alla radice. Quando [sposti la sessione con `/cd`](/docs/it/permissions#move-the-session-to-another-directory) su v2.1.246 o successiva, Claude Code aggiunge le skill di progetto della nuova directory.

In una sessione in esecuzione in un [git worktree](/docs/it/worktrees) collegato, Claude Code cerca le directory padre solo fino alla radice del worktree. Su Claude Code v2.1.277 o successiva, quando il checkout del worktree non ha una directory `.claude/skills` alla sua radice, Claude Code carica invece le skill di progetto del checkout principale. Vedi [Cosa i worktree condividono con il checkout principale](/docs/it/worktrees#what-worktrees-share-with-the-main-checkout).

Le skill in una directory `.claude/skills/` sotto il punto in cui hai iniziato non si caricano all'avvio. Si caricano la prima volta che Claude legge o modifica un file in quella sottodirectory e rimangono disponibili per il resto della sessione. Fino ad allora non appaiono nel menu `/` e non puoi invocarle per nome. Per caricarle prima, esegui `/add-dir` con il percorso della sottodirectory, che richiede Claude Code v2.1.257 o successiva.

Quando una skill annidato condivide un nome con un'altra skill, entrambe rimangono disponibili. Con una skill `deploy` alla radice del repository e un'altra in `apps/web/.claude/skills/`:

* `/deploy` esegue la skill radice. Claude Code elenca anche le varianti qualificate dalla directory per Claude, con un'istruzione per invocare quella la cui directory contiene i file su cui sta lavorando, in modo che la skill annidato si applichi comunque al lavoro in `apps/web/`.
* `/apps/web:deploy` esegue la skill annidato da sola. La sua descrizione nomina la directory a cui si applica.

<h3 id="skills-from-additional-directories">
  Carica skills da una directory al di fuori del progetto
</h3>

Quando aggiungi una directory con `--add-dir` o `/add-dir`, Claude Code carica le skill nella `.claude/skills/` di quella directory, insieme ai suoi `.claude/commands/` e `.claude/agents/`. Le directory che l'Agent SDK aggiunge tramite [`additionalDirectories`](/docs/it/agent-sdk/typescript#options) in TypeScript o [`add_dirs`](/docs/it/agent-sdk/python#claudeagentoptions) in Python si caricano allo stesso modo, perché l'SDK le passa come `--add-dir`. L'impostazione `permissions.additionalDirectories` in `settings.json` concede solo l'accesso ai file e non carica nessuno di questi.

Claude Code osserva `.claude/skills/` in una directory che passi con `--add-dir` all'avvio, come [Modifica una skill durante una sessione](#live-change-detection) descrive. Non osserva `.claude/commands/` o `.claude/agents/` della directory aggiunta, quindi riavvia la sessione dopo aver modificato un file lì.

Questi caricamenti dipendono dall'[origine dell'impostazione](/docs/it/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) `project`, che è attiva per impostazione predefinita. Una politica [`strictPluginOnlyCustomization`](/docs/it/settings-reference#strictpluginonlycustomization), [modalità bare](/docs/it/headless#start-faster-with-bare-mode) e [`--safe-mode`](/docs/it/cli-reference#cli-flags) le limitano ulteriormente, come quelle pagine descrivono. Vedi [Le directory aggiuntive concedono l'accesso ai file, non la configurazione](/docs/it/permissions#additional-directories-grant-file-access-not-configuration) per la tabella completa di ciò che una directory aggiunta carica, inclusi `CLAUDE.md` e le impostazioni dei plugin.

<h3 id="resolve-skills-that-share-a-name">
  Risolvi skills che condividono un nome
</h3>

Quando due skill condividono un nome, il luogo da cui proviene ciascuna decide quale `/name` esegue. La tabella copre le posizioni enterprise, personale, progetto, annidato, plugin e claude.ai, le skill raggruppate e i file di comando:

| Stesso nome in                                                                                                | Quale esegue                                                                                                                                                                                                                    |
| :------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Due di enterprise, personale e progetto                                                                       | Enterprise su personale, e personale su progetto. Con `deploy` sia in `~/.claude/skills/` che in `.claude/skills/` del progetto, `/deploy` esegue quello personale                                                              |
| Uno qualsiasi di quelle posizioni e una [skill raggruppata](#bundled-skills)                                  | La tua skill sostituisce il comando raggruppato, ma non i suoi alias. Una skill `code-review` di progetto sostituisce `/code-review`, e l'alias raggruppato `/review` non esegue mai la tua skill                               |
| Una skill e un file in `.claude/commands/`                                                                    | La skill                                                                                                                                                                                                                        |
| Una skill radice di progetto e una skill annidato                                                             | Entrambe si caricano. Vedi [monorepos e sottodirectory](#discovery-from-parent-and-nested-directories)                                                                                                                          |
| Una skill di plugin e una skill in uno qualsiasi dei luoghi sopra                                             | Entrambe si caricano, perché le skill di plugin sono nello spazio dei nomi come `/plugin-name:skill-name`                                                                                                                       |
| Uno qualsiasi dei precedenti e una skill [sincronizzata dal tuo account claude.ai](#how-synced-skills-behave) | L'altra skill o comando. La skill sincronizzato esegue comunque come `/anthropic-skills:<name>`. Vedi [Quando un nome di skill sincronizzato corrisponde a un altro comando](#when-a-synced-skill-name-matches-another-command) |

<h3 id="skills-in-cowork-and-cloud-sessions">
  Usa skills in sessioni Cowork e cloud
</h3>

Le sessioni [Cowork](https://claude.com/product/cowork) e [cloud sessions](/docs/it/cloud-environments#what-carries-over-from-your-setup), incluse [routine](/docs/it/routines), non leggono `~/.claude/skills/` sulla tua macchina. Sia le sessioni Cowork interattive che quelle programmate caricano le skill abilitate per il tuo account claude.ai, sincronizzate all'avvio della sessione; gestiscile da **Customize** nella barra laterale dell'app Desktop o dalle impostazioni delle skill su claude.ai. Le sessioni cloud caricano inoltre le skill di progetto committate in `.claude/skills/` del repository clonato.

Se una skill esiste solo in `~/.claude/skills/` sulla tua macchina, Claude Code segnala che la skill non è stata trovata quando una [routine](/docs/it/routines) la invoca, perché ogni esecuzione di routine inizia come una sessione cloud nuova. Per rendere disponibile una skill personale in queste sessioni:

* Per le sessioni Cowork e cloud, abilita la skill per il tuo account claude.ai.
* Per le sessioni cloud, puoi invece committare la skill in `.claude/skills/` del repository. I plugin dichiarati in `.claude/settings.json` del repository e i plugin abilitati solo nelle tue impostazioni utente [non si caricano nelle sessioni cloud](/docs/it/cloud-environments#what-carries-over-from-your-setup).

[I compiti programmati del Desktop](/docs/it/desktop-scheduled-tasks) vengono eseguiti localmente sulla tua macchina, quindi caricano `~/.claude/skills/`.

<h3 id="how-synced-skills-behave">
  Skills sincronizzate da claude.ai
</h3>

Questa sezione si applica a te se usi sessioni Cowork o cloud, o accedi a Claude Code nel tuo terminale con un account claude.ai. In quelle sessioni, Claude Code carica le skill abilitate per il tuo account claude.ai, senza alcuna configurazione da parte tua, come [Dove si caricano le skill sincronizzate](#where-synced-skills-load) descrive. Quelle skill includono quelle che crei o attivi nelle tue impostazioni claude.ai, le skill che la tua organizzazione fornisce lì, e le skill integrate di Anthropic come `pdf` e `xlsx`.

Claude Code scarica una skill sincronizzato dal tuo account piuttosto che leggere un file che hai scritto sulla macchina in cui la sessione viene eseguita, quindi applica regole alle skill sincronizzate che non si applicano alle skill che memorizzi nelle [posizioni delle skill](#where-skills-live).

<h4 id="where-synced-skills-load">
  Dove si caricano le skill sincronizzate
</h4>

In una sessione Cowork o cloud, Claude Code carica le skill abilitate per il tuo account claude.ai, e [Skills in Cowork and cloud sessions](#skills-in-cowork-and-cloud-sessions) dice come scegliere quali skill quelle sessioni ottengono.

Nel tuo terminale, Claude Code sincronizza quelle skill in sessioni in cui accedi con il tuo account claude.ai. Quando la sessione inizia, Claude Code scarica le skill del tuo account in `~/.claude/skills/synced/` in background, quindi controlla claude.ai per i cambiamenti circa ogni 10 minuti mentre la sessione è in esecuzione. Quando un controllo scopre che una skill è stata aggiunta, modificata o disattivata su claude.ai, Claude Code la aggiunge, aggiorna o rimuove nella sessione in esecuzione senza un riavvio. La sincronizzazione nelle sessioni di terminale richiede Claude Code v2.1.273 o successiva.

La sincronizzazione non ritarda mai l'avvio, perché Claude attende il download di una skill solo quando la invoca. Un'esecuzione breve [non interattiva](/docs/it/headless) può quindi terminare prima che una skill appena aggiunta si scarichi, nel qual caso una sessione successiva la scarica. Per fare in modo che un'esecuzione non interattiva scarichi le tue skill e attenda l'elenco prima di rispondere al prompt, imposta [`CLAUDE_CODE_SYNC_SKILLS`](/docs/it/env-vars#variables) su `1`.

Claude Code sincronizza solo in una sessione che accede con il tuo account claude.ai e [recupera i flag delle funzionalità da Anthropic](/docs/it/env-vars#features-that-need-feature-flag-fetching). Non sincronizza in queste sessioni:

* Una sessione che non utilizza un accesso memorizzato da `/login`, come una che si autentica con una chiave API, o una in cui `ANTHROPIC_AUTH_TOKEN`, `CLAUDE_CODE_OAUTH_TOKEN`, o uno script `apiKeyHelper` fornisce la credenziale
* Una sessione che non recupera i flag delle funzionalità, come una su Amazon Bedrock o una in cui imposti `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`
* Una sessione in [modalità bare](/docs/it/headless#start-faster-with-bare-mode) o una che avvii con `--safe-mode`
* Una sessione in cui le impostazioni gestite della tua organizzazione [bloccano le skill alle fonti dei plugin](/docs/it/settings-reference#strictpluginonlycustomization-skills), o una che avvii con un elenco [`--setting-sources`](/docs/it/cli-reference#cli-flags) che esclude `user`

Se accedi con `/login` durante una sessione, riavvia Claude Code per iniziare la sincronizzazione.

Le skill che una sessione precedente ha sincronizzato rimangono su disco. Claude Code le carica in sessioni successive accedute allo stesso account, anche quando non riesce a raggiungere claude.ai.

Claude Code scarica le skill sincronizzate e non le carica mai. Se tu o Claude modificate un file sotto `~/.claude/skills/synced/`, la modifica non viene salvata nel tuo account claude.ai, e una sincronizzazione successiva può sovrascriverla o rimuoverla. Per modificare una skill sincronizzato, aggiornala su claude.ai; la prossima sincronizzazione scarica la nuova versione.

Per vedere quali skill hanno sincronizzato, esegui `/skills`. Il menu le elenca sotto `claude.ai sync`.

Alcune skill di Anthropic, come `pdf` e `xlsx`, sincronizzano sempre. Per il resto, attiva o disattiva una skill nelle tue impostazioni delle skill su claude.ai per cambiare se sincronizza.

Per smettere di sincronizzare su una macchina, imposta [`syncClaudeAiSkills`](/docs/it/settings-reference#syncclaudeaiskills) su `false` nelle tue impostazioni utente. Claude Code smette di scaricare, e la prossima volta che si avvia sposta le skill che ha già sincronizzato in `~/.claude/skills/.trash/` e non le carica più. La tua organizzazione può disattivare la sincronizzazione per tutti disattivando le Skills su claude.ai. Per smettere di sincronizzare mantenendo le Skills attive, può impostare la stessa chiave nelle [impostazioni gestite](/docs/it/managed-settings).

Se la tua organizzazione disattiva le Skills su claude.ai, Claude Code rimuove le skill scaricate e smettono di caricarsi. Le skill rimosse si spostano in `~/.claude/skills/.trash/`, dove puoi recuperare i file fino a quando la [pulizia della conservazione](/docs/it/claude-directory#cleaned-up-automatically) li elimina. Una volta che la tua organizzazione riattiva le Skills, Claude Code scarica le skill che hai abilitato alla prossima sincronizzazione.

<h4 id="when-a-synced-skill-name-matches-another-command">
  Quando un nome di skill sincronizzato corrisponde a un altro comando
</h4>

Puoi invocare una skill sincronizzato con il suo nome completo, `/anthropic-skills:<name>`, o con il suo nome breve, `/<name>`. Quando un altro comando usa quel nome breve, `/<name>` esegue l'altro comando, e la skill sincronizzato esegue solo come `/anthropic-skills:<name>`. Con una skill `deploy` locale e una `deploy` sincronizzato, `/deploy` esegue la skill locale e `/anthropic-skills:deploy` esegue quella sincronizzato. Prima di v2.1.269, una skill sincronizzato aveva solo il suo nome breve.

L'altro comando può essere uno qualsiasi di questi:

* Un comando integrato o una [skill raggruppata](#bundled-skills), inclusa una che non è disponibile nella tua sessione, ad esempio dopo aver disattivato le skill raggruppate
* Una skill a qualsiasi [livello locale](#where-skills-live) o un file in `.claude/commands/`
* Una skill di plugin
* Un [prompt MCP](/docs/it/mcp#use-mcp-prompts-as-commands)

Claude Code etichetta le skill sincronizzate in modo che tu possa dire da dove provengono. Il menu `/skills` e `/context` raggruppano le skill sincronizzate sotto `claude.ai sync`, e il menu del comando `/` le contrassegna come provenienti da claude.ai.

Quando confronta i nomi, Claude Code ignora maiuscole/minuscole, spaziatura e caratteri invisibili, e tratta forme di compatibilità come lettere a larghezza intera e varianti di trattini come i loro equivalenti semplici. Ad esempio, una skill sincronizzato denominata `Commit` e una skill locale denominata `commit` contano come lo stesso nome, quindi `/commit` continua a eseguire la tua skill locale.

Un nome che differisce solo per una lettera simile da un altro alfabeto conta come un nome diverso, e l'etichetta `claude.ai sync` è come distingui i due. Questi controlli e etichette richiedono Claude Code v2.1.228 o successiva.

<h4 id="how-claude-code-handles-the-frontmatter-of-a-synced-skill">
  Come Claude Code gestisce il frontmatter di una skill sincronizzato
</h4>

Claude Code applica due regole al frontmatter di una skill sincronizzato:

* Claude Code onora il frontmatter in ogni tipo di sessione, quindi una concessione `allowed-tools` passa attraverso il normale [flusso di permessi](/docs/it/permissions).
* Claude Code igienizza il testo di visualizzazione che la skill fornisce, come la sua descrizione. Rimuove i caratteri di controllo, e nel testo che raggiunge Claude, come la descrizione, sfugge anche alle parentesi angolari in modo che il testo non possa imitare la formattazione interna di Claude Code. Questa igienizzazione richiede Claude Code v2.1.228 o successiva.

<h4 id="how-claude-code-handles-the-body-of-a-synced-skill">
  Come Claude Code gestisce il corpo di una skill sincronizzato
</h4>

Ciò che Claude Code fa con il corpo di una skill sincronizzato dipende da dove la sessione viene eseguita:

* In una sessione cloud, il corpo mantiene il comportamento che una skill locale ha, perché la sessione viene eseguita in un contenitore isolato.
* In una sessione Cowork sul tuo desktop, il corpo mantiene il comportamento che una skill locale ha, tranne che Claude Code sostituisce ogni riga di comando `!` con il placeholder [`disableSkillShellExecution`](#inject-dynamic-context), come fa per ogni skill che fornisci lì.
* In qualsiasi altra sessione sulla tua macchina, Claude Code non esegue i comandi [`!`](#inject-dynamic-context), non allega i file che i riferimenti `@` nominano nel modo in cui lo fa per una skill locale, e non sostituisce i placeholder `${CLAUDE_PROJECT_DIR}` e `${CLAUDE_SESSION_ID}`, quindi i riferimenti `@` e entrambi i placeholder raggiungono Claude come testo letterale. Una riga di comando `!` raggiunge Claude come testo letterale anche, o come quel placeholder quando `disableSkillShellExecution` è attivo. Questa gestione richiede Claude Code v2.1.228 o successiva.

<h3 id="live-change-detection">
  Modifica una skill durante una sessione
</h3>

Claude Code osserva le directory delle skill per i cambiamenti di file, tranne in [modalità bare](/docs/it/headless#start-faster-with-bare-mode). Quando aggiungi, modifichi o rimuovi una skill sotto `~/.claude/skills/`, il progetto `.claude/skills/`, o una `.claude/skills/` all'interno di una directory `--add-dir`, Claude Code raccoglie il cambiamento all'interno della sessione corrente, senza un riavvio. Se crei una directory di skill di livello superiore che non esisteva quando la sessione è iniziata, riavvia Claude Code in modo che possa osservare la nuova directory.

La rilevazione dei cambiamenti in tempo reale copre solo il testo `SKILL.md`. Per una cartella di skill che è anche un [plugin](/docs/it/plugins/loading#plugins-shared-through-a-repository), i cambiamenti a `hooks/`, `.mcp.json`, `agents/` e `output-styles/` richiedono `/reload-plugins` per avere effetto.

<h3 id="remove-a-skill">
  Rimuovi una skill
</h3>

Come rimuovi una skill dipende da dove proviene:

* **Skill personale o di progetto**: elimina la directory della skill, `~/.claude/skills/<skill-name>/` o `.claude/skills/<skill-name>/`. Claude Code [la elimina da `/skills` nella sessione corrente](#live-change-detection); il contenuto che Claude Code ha già caricato da essa segue il [ciclo di vita del contenuto della skill](#skill-content-lifecycle).
* **Skill enterprise**: un amministratore elimina la directory della skill da `.claude/skills/` all'interno della [directory delle impostazioni gestite](/docs/it/managed-settings#delivery-mechanisms), ad esempio `/etc/claude-code/.claude/skills/<skill-name>/` su Linux.
* **Skill di plugin**: disabilita o disinstalla il plugin che la fornisce, dal menu `/plugin` o con `/plugin uninstall <plugin-name>@<marketplace-name>`. Claude Code scarica le skill del plugin quando [il cambiamento si applica](/docs/it/plugins/cli-reference#reload-plugins) o quando riavvii.
* **Skill sincronizzato da claude.ai**: disattiva la skill per il tuo account claude.ai, nello stesso luogo in cui l'hai [abilitata](#skills-in-cowork-and-cloud-sessions). Claude Code la rimuove da `~/.claude/skills/synced/` la prossima volta che [sincronizza le tue skill](#how-synced-skills-behave). Se elimini la directory manualmente, la prossima sincronizzazione la scarica di nuovo mentre la skill rimane abilitata su claude.ai.
* **Skill raggruppata**: imposta [`disableBundledSkills`](#bundled-skills) su `true` per disattivare le skill raggruppate, o imposta una skill su `"off"` in [`skillOverrides`](#override-skill-visibility-from-settings) per nasconderla.

Per mantenere una skill personale o di progetto ma impedire a Claude di invocarla da solo, imposta [`disable-model-invocation: true`](#control-who-invokes-a-skill) nel suo frontmatter, o `"user-invocable-only"` in [`skillOverrides`](#override-skill-visibility-from-settings) quando non vuoi modificare il file.

<h2 id="configure-skills">
  Configurare skills
</h2>

Le skills sono configurate tramite frontmatter YAML nella parte superiore di `SKILL.md` e il contenuto markdown che segue.

<h3 id="types-of-skill-content">
  Tipi di contenuto skill
</h3>

I file skill possono contenere qualsiasi istruzione, ma pensare a come vuoi invocarli aiuta a guidare cosa includere:

**Contenuto di riferimento** aggiunge conoscenze che Claude applica al tuo lavoro attuale. Convenzioni, pattern, guide di stile, conoscenza del dominio. Questo contenuto viene eseguito inline in modo che Claude possa usarlo insieme al contesto della tua conversazione.

```yaml theme={null}
---
name: api-conventions
description: API design patterns for this codebase
---

When writing API endpoints:
- Use RESTful naming conventions
- Return consistent error formats
- Include request validation
```

**Contenuto di attività** fornisce a Claude istruzioni passo dopo passo per un'azione specifica, come distribuzioni, commit o generazione di codice. Spesso sono azioni che vuoi invocare direttamente con `/skill-name` piuttosto che lasciare che Claude decida quando eseguirle. Aggiungi `disable-model-invocation: true` per impedire a Claude di attivarla automaticamente. L'esempio seguente aggiunge `context: fork`, che esegue la skill nel suo contesto di subagent; vedi [Eseguire skills in un subagent](#run-skills-in-a-subagent).

```yaml theme={null}
---
name: deploy
description: Deploy the application to production
context: fork
disable-model-invocation: true
---

Deploy the application:
1. Run the test suite
2. Build the application
3. Push to the deployment target
```

Mantieni il corpo stesso conciso. Una volta che una skill si carica, il suo contenuto [rimane nel contesto tra i turni](#skill-content-lifecycle), quindi ogni riga è un costo di token ricorrente. Dichiara cosa fare piuttosto che narrare come o perché, e applica lo stesso test di concisione che faresti per il [contenuto CLAUDE.md](/docs/it/best-practices#write-an-effective-claude-md).

<h3 id="frontmatter-reference">
  Riferimento frontmatter
</h3>

Configura una skill con YAML [frontmatter](/docs/it/glossary#frontmatter) tra i marcatori `---` nella parte superiore di `SKILL.md`, e scrivi le istruzioni della skill come Markdown dopo il `---` di chiusura. I nomi dei campi usano parole minuscole separate da trattini, eccetto `when_to_use`. Un [file di comando](#where-skills-live) in `.claude/commands/` accetta gli stessi campi eccetto `name` e `paths`. Questo esempio imposta quattro campi:

```yaml theme={null}
---
name: my-skill
description: What this skill does
disable-model-invocation: true
allowed-tools: Read Grep
---

Your skill instructions here...
```

Tutti i campi sono facoltativi. Solo `description` è consigliato in modo che Claude sappia quando usare la skill. Un nome di campo deve corrispondere esattamente alla tabella, trattini inclusi: Claude Code ignora un campo che non riconosce senza segnalare un errore.

Claude Code legge il frontmatter solo quando l'apertura `---` è la prima riga del file. Altrimenti tratta l'intero file, marcatori `---` inclusi, come contenuto skill. Se lo YAML tra i marcatori non viene analizzato, la skill si carica comunque senza campi impostati; vedi [Skill non si attiva](#skill-not-triggering) per trovare e correggere l'errore.

I campi booleani accettano `yes`, `no`, `on`, `off`, `1` e `0` in qualsiasi maiuscola, oltre a `true` e `false`. Prima della v2.1.218, Claude Code riconosceva solo `true` e `false`.

| Campo                      | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :------------------------- | :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                     | No           | Nome visualizzato mostrato negli elenchi di skills. Predefinito al nome della directory. Vedi [Come una skill ottiene il suo nome di comando](#how-a-skill-gets-its-command-name) per come il campo interagisce con il nome che digiti per invocare la skill.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `description`              | Consigliato  | Cosa fa la skill e quando usarla. Claude usa questo per decidere quando applicare la skill. Se omesso, usa la prima riga non vuota del contenuto markdown. Metti il caso d'uso chiave per primo: il testo combinato `description` e `when_to_use` viene troncato a 1.536 caratteri nell'elenco delle skills per ridurre l'utilizzo del contesto.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `when_to_use`              | No           | Contesto aggiuntivo per quando Claude dovrebbe invocare la skill, come frasi trigger o richieste di esempio. Aggiunto a `description` nell'elenco delle skills e conta verso il limite di 1.536 caratteri.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `argument-hint`            | No           | Suggerimento mostrato durante l'autocompletamento per indicare gli argomenti previsti. Esempio: `[issue-number]` o `[filename] [format]`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `arguments`                | No           | Argomenti posizionali denominati per la [sostituzione `$name`](#available-string-substitutions) nel contenuto della skill. Accetta una stringa separata da spazi o un elenco YAML. I nomi si mappano alle posizioni degli argomenti in ordine.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `disable-model-invocation` | No           | Impostato su `true` per impedire a Claude di caricare automaticamente questa skill. Usa per flussi di lavoro che vuoi attivare manualmente con `/name`. Impedisce anche alla skill di essere [precaricata nei subagent](/docs/it/sub-agents#preload-skills-into-subagents). A partire dalla v2.1.196, impedisce anche alla skill di eseguirsi quando un [compito programmato](/docs/it/scheduled-tasks) si attiva con la skill come suo prompt. Predefinito: `false`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `user-invocable`           | No           | Impostato su `false` quando solo Claude dovrebbe invocare la skill: Claude Code la nasconde dal menu `/` e non la esegue quando digiti `/name`. Usa per conoscenze di background che gli utenti non dovrebbero invocare direttamente. Predefinito: `true`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `allowed-tools`            | No           | Strumenti che Claude può usare senza chiedere permesso durante il turno che invoca questa skill. La concessione si cancella quando invii il tuo prossimo messaggio. Accetta una stringa separata da spazi o virgole, o un elenco YAML. Vedi [Pre-approvare strumenti per una skill](#pre-approve-tools-for-a-skill).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `disallowed-tools`         | No           | Strumenti rimossi dal pool disponibile di Claude mentre questa skill è attiva. Usa per skills autonome che non dovrebbero mai chiamare determinati strumenti, come `AskUserQuestion` per un loop di background. Accetta una stringa separata da spazi o virgole, o un elenco YAML. La restrizione si cancella quando invii il tuo prossimo messaggio. Come le regole di negazione, il campo non può rimuovere [`EndConversation`](/docs/it/tools-reference#endconversation-tool-behavior) mentre rimane qualsiasi altro strumento.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `model`                    | No           | Modello da usare quando questa skill è attiva. L'override si applica per il resto del turno attuale e non viene salvato nelle impostazioni. Il modello della sessione riprende quando invii il tuo prossimo prompt. Accetta gli stessi valori di [`/model`](/docs/it/model-config), o `inherit` per mantenere il modello attivo. Un valore escluso dalla lista di consentiti [`availableModels`](/docs/it/model-config#restrict-model-selection) della tua organizzazione non viene usato, e la sessione mantiene il suo modello attuale. In [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode), e in [modalità piano mentre il classificatore esamina i comandi](/docs/it/permission-modes#analyze-before-you-edit-with-plan-mode), un modello che la modalità auto non supporta non viene usato, e la sessione mantiene il suo modello attuale. Con `context: fork`, il valore imposta il [modello del subagent biforcato](#run-skills-in-a-subagent) invece, e un valore escluso segue le [stesse regole di un override del modello del subagent](/docs/it/model-config#restrict-model-selection). |
| `effort`                   | No           | [Livello di sforzo](/docs/it/model-config#adjust-effort-level) quando questa skill è attiva. Sostituisce il livello di sforzo della sessione. Predefinito: eredita dalla sessione. Opzioni: `low`, `medium`, `high`, `xhigh`, `max`; i livelli disponibili dipendono dal modello.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `context`                  | No           | Impostato su `fork` per eseguire in un contesto di subagent biforcato. Vedi [Eseguire skills in un subagent](#run-skills-in-a-subagent).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `agent`                    | No           | Quale tipo di subagent usare quando `context: fork` è impostato.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `background`               | No           | Si applica solo con `context: fork`. Impostato su `false` per attendere il risultato del subagent biforcato nel turno che ha invocato la skill, invece di [eseguirla in background](#run-skills-in-a-subagent). Predefinito: `true`. Richiede Claude Code v2.1.218 o successivo.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `hooks`                    | No           | Hooks che Claude Code registra quando la skill viene invocata e continua a eseguire per il resto della sessione. Vedi [Hooks in skills e agents](/docs/it/hooks#hooks-in-skills-and-agents) per il formato di configurazione e l'opzione `once`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `paths`                    | No           | Pattern Glob che limitano quando questa skill viene attivata. Accetta una stringa separata da virgole o un elenco YAML. Quando impostato, Claude carica la skill automaticamente solo quando lavora con file che corrispondono ai pattern. Usa lo stesso formato delle [regole specifiche del percorso](/docs/it/memory#path-specific-rules).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `shell`                    | No           | Shell da usare per `` !`command` `` e ` ```! ` blocchi in questa skill. Accetta `bash` (predefinito) o `powershell`. L'impostazione di `powershell` esegue i comandi shell inline tramite PowerShell quando lo [strumento PowerShell](/it/tools-reference#powershell-tool) è abilitato: è attivo per impostazione predefinita su Windows senza Git Bash, attivo per impostazione predefinita con Git Bash per account claude.ai e Console, e richiede `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` in sessioni Amazon Bedrock, Google Cloud's Agent Platform e Microsoft Foundry e su macOS, Linux e WSL. Impostalo su `0` per disattivare lo strumento.                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `metadata`                 | No           | Mappa YAML libera per i tuoi dati chiave-valore, come campi di diritto o catalogo, letti dal tuo tooling da `SKILL.md`. Claude Code non agisce sui suoi contenuti e scarta un valore che non è una mappa. Non riutilizzare nomi di campi frontmatter come `paths` come chiavi.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `license`                  | No           | Licenza che copre la skill. Parte della specifica [Agent Skills](https://agentskills.io); vedi [Usare il frontmatter della skill al di fuori di Claude Code](#using-skill-frontmatter-outside-claude-code). Claude Code accetta il campo ma non agisce su di esso.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `compatibility`            | No           | Requisiti di ambiente per la skill, come prodotti previsti o prerequisiti di sistema, come definito dalla specifica [Agent Skills](https://agentskills.io); vedi [Usare il frontmatter della skill al di fuori di Claude Code](#using-skill-frontmatter-outside-claude-code). Accetta una stringa di massimo 500 caratteri. Claude Code accetta il campo ma non agisce su di esso.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |

<h4 id="using-skill-frontmatter-outside-claude-code">
  Usare il frontmatter della skill al di fuori di Claude Code
</h4>

Claude Code accetta ogni campo nella tabella sopra. Al di fuori di Claude Code, puoi usare solo i campi nella specifica [Agent Skills](https://agentskills.io):

| Percorso di distribuzione                                                                                                                      | Campi frontmatter che puoi usare                                               |
| :--------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------- |
| Skills di Claude Code a [qualsiasi livello](#where-skills-live), incluse skills di [plugin](/docs/it/plugins/overview)                              | Ogni campo nella tabella sopra                                                 |
| Upload di skills su claude.ai, l'API Skills e il packaging con `package_skill.py` da [anthropics/skills](https://github.com/anthropics/skills) | `name`, `description`, `license`, `compatibility`, `metadata`, `allowed-tools` |

Quando abiliti una skill personale per il tuo account claude.ai, ad esempio per usarla in [sessioni Cowork e cloud](#skills-in-cowork-and-cloud-sessions) e routine, la carichi su claude.ai, quindi si applicano le stesse regole.

Se includi un campo che la specifica non consente, il packaging o l'upload fallisce con un errore grave invece di ignorare il campo:

```
Unexpected key(s) in SKILL.md frontmatter: argument-hint. Allowed properties are: allowed-tools, compatibility, description, license, metadata, name
```

Limitare il frontmatter ai sei campi della specifica evita l'errore di chiave inaspettata sopra. La [specifica Agent Skills](https://agentskills.io) e i [requisiti dell'API Skills](https://docs.claude.com/en/api/skills-guide) definiscono tutto il resto che questi percorsi convalidano. Le funzionalità del corpo solo di Claude Code, come l'[iniezione di contesto dinamico](#inject-dynamic-context), non funzionano nella chat claude.ai o tramite l'API. Claude Code accetta tutti e sei i campi, quindi il frontmatter che segue la specifica si carica in Claude Code senza modifiche.

<h4 id="how-a-skill-gets-its-command-name">
  Come una skill ottiene il suo nome di comando
</h4>

Il comando che digiti per invocare una skill proviene da dove vive il file skill e, per skills di plugin, anche dal campo frontmatter `name`. In una skill personale o di progetto, `name` imposta solo l'etichetta di visualizzazione mostrata negli elenchi di skills, e il comando proviene ancora dal nome della directory. In una skill di plugin, `name` imposta l'ultimo segmento del comando e il prefisso del plugin rimane in posizione.

La tabella seguente mostra da dove proviene il nome del comando per ogni layout:

| Posizione della skill                                                                                      | Fonte del nome del comando                                                                                                    | Esempio                                                                                                                                       |
| :--------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------- |
| Directory skill sotto `~/.claude/skills/` o `.claude/skills/`                                              | Nome della directory                                                                                                          | `.claude/skills/deploy-staging/SKILL.md` → `/deploy-staging`                                                                                  |
| Directory [nidificata](#where-skills-live) `.claude/skills/`, quando il nome si scontra con un'altra skill | Percorso della sottodirectory relativo alla directory di lavoro, quindi il nome della directory skill                         | `apps/web/.claude/skills/deploy/SKILL.md` → `/apps/web:deploy`                                                                                |
| File sotto `.claude/commands/`                                                                             | Nome del file senza estensione                                                                                                | `.claude/commands/deploy.md` → `/deploy`                                                                                                      |
| File in una sottodirectory di `.claude/commands/`                                                          | Percorso della sottodirectory relativo a `commands/` con ogni `/` sostituito da `:`, quindi il nome del file senza estensione | `.claude/commands/frontend/component.md` → `/frontend:component`                                                                              |
| Sottodirectory `skills/` del plugin                                                                        | Frontmatter `name` o il nome della directory, con namespace dal plugin                                                        | `my-plugin/skills/review/SKILL.md` → `/my-plugin:review`, o `/my-plugin:fancy` con `name: fancy`                                              |
| `SKILL.md` radice del plugin                                                                               | Frontmatter `name`, con il nome della directory del plugin come fallback                                                      | `my-plugin/SKILL.md` con `name: review` → `/my-plugin:review`. Vedi [una singola skill alla radice del plugin](/docs/it/plugins/components#skills) |
| Skill [sincronizzata da claude.ai](#how-synced-skills-behave)                                              | Il nome della skill sul tuo account claude.ai, con prefisso `anthropic-skills:`                                               | Skill dell'account `deploy` → `/anthropic-skills:deploy`, o `/deploy` mentre nessun altro comando usa quel nome                               |

In una skill di plugin, il frontmatter `name` sostituisce il nome della directory nell'ultimo segmento del comando, quindi `my-plugin/skills/review/SKILL.md` con `name: fancy` diventa `/my-plugin:fancy`. Il bare `/fancy` invoca anche la skill a meno che un altro comando non usi già quel nome. Se il `name` che scrivi inizia già con il prefisso del plugin stesso, Claude Code non aggiunge il prefisso di nuovo nella v2.1.246 o successivo. Ad esempio, `name: my-plugin:fancy` diventa comunque `/my-plugin:fancy`. Dalla v2.1.216 alla v2.1.245, Claude Code ha raddoppiato il prefisso quando il `name` lo portava già.

In [sessioni non interattive](/docs/it/headless), i nomi `help` e `feedback` non sono riservati ai loro comandi built-in solo per il terminale, quindi una skill di plugin con uno di questi nomi mantiene il suo comando bare lì. Ogni altro built-in solo per il terminale, come `/login`, rimane riservato anche se il comando non può eseguirsi in quelle sessioni.

Per un `SKILL.md` radice del plugin, non c'è directory skill da cui prendere il nome, quindi `name` fornisce l'intero segmento finale. Senza un campo `name`, Claude Code ricade al nome della directory del plugin.

<h4 id="available-string-substitutions">
  Sostituzioni di stringhe disponibili
</h4>

Le skills supportano la sostituzione di stringhe per valori dinamici nel contenuto della skill:

| Variabile               | Descrizione                                                                                                                                                                                                                                                                                                                                 |
| :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `$ARGUMENTS`            | Tutti gli argomenti passati quando si invoca la skill. Quando nessun placeholder riceve un argomento, Claude Code li aggiunge come `ARGUMENTS: <value>`. Vedi [Passare argomenti alle skills](#pass-arguments-to-skills).                                                                                                                   |
| `$ARGUMENTS[N]`         | Accedi a un argomento specifico per indice a base 0, come `$ARGUMENTS[0]` per il primo argomento.                                                                                                                                                                                                                                           |
| `$N`                    | Abbreviazione per `$ARGUMENTS[N]`, come `$0` per il primo argomento o `$1` per il secondo.                                                                                                                                                                                                                                                  |
| `$name`                 | Argomento denominato dichiarato nell'elenco frontmatter [`arguments`](#frontmatter-reference). I nomi si mappano alle posizioni in ordine, quindi con `arguments: [issue, branch]` il placeholder `$issue` si espande al primo argomento e `$branch` al secondo.                                                                            |
| `${CLAUDE_SESSION_ID}`  | L'ID della sessione attuale. Utile per il logging, la creazione di file specifici della sessione o la correlazione dell'output della skill con le sessioni.                                                                                                                                                                                 |
| `${CLAUDE_EFFORT}`      | Il livello di sforzo attuale: `low`, `medium`, `high`, `xhigh` o `max`. Ultracode non è un livello distinto e viene segnalato come `xhigh`. Usa questo per adattare le istruzioni della skill all'impostazione di sforzo attiva.                                                                                                            |
| `${CLAUDE_SKILL_DIR}`   | La directory contenente il file `SKILL.md` della skill. Per skills di plugin, questa è la sottodirectory della skill all'interno del plugin, non la radice del plugin. Usa questo nei comandi di iniezione bash per fare riferimento a script o file forniti con la skill, indipendentemente dalla directory di lavoro attuale.             |
| `${CLAUDE_PROJECT_DIR}` | La directory radice del progetto. Questo è lo stesso percorso che [hooks](/docs/it/hooks#reference-scripts-by-path) e server MCP ricevono come `CLAUDE_PROJECT_DIR`. Usa questo per fare riferimento a script o file locali del progetto, come `${CLAUDE_PROJECT_DIR}/.claude/hooks/helper.sh`, indipendentemente da dove la skill è installata. |
| `${CLAUDE_PLUGIN_ROOT}` | La directory di installazione del plugin. Sostituito solo in skills di plugin. Usa questo per fare riferimento a script o file forniti in qualsiasi punto del plugin, incluse risorse condivise tra le skills del plugin. Vedi [variabili di ambiente del plugin](/docs/it/plugins/manifest-reference#environment-variables).                    |
| `${CLAUDE_PLUGIN_DATA}` | La [directory di dati persistenti](/docs/it/plugins/components#path-variables-and-persistent-data) del plugin, che sopravvive agli aggiornamenti del plugin. Sostituito solo in skills di plugin. Usa questo per fare riferimento a dipendenze installate, file generati o cache che devono sopravvivere a un aggiornamento.                     |

Claude Code sostituisce `${CLAUDE_SKILL_DIR}` e `${CLAUDE_PROJECT_DIR}` in due posti: il contenuto markdown della skill e le regole Bash nel frontmatter [`allowed-tools`](#frontmatter-reference). In una skill di plugin, Claude Code sostituisce `${CLAUDE_PLUGIN_ROOT}` e `${CLAUDE_PLUGIN_DATA}` negli stessi due posti. Usare la stessa variabile in entrambi i posti consente a una skill di eseguire uno script fornito senza un prompt di permesso. La seguente skill mostra il pattern:

```yaml theme={null}
---
name: render-chart
description: Render a chart from a CSV file
allowed-tools: Bash(${CLAUDE_SKILL_DIR}/scripts/render.sh *)
---

Run `${CLAUDE_SKILL_DIR}/scripts/render.sh <csv-file>` to render the chart.
```

Se questa skill è installata in `~/.claude/skills/render-chart/`, entrambi gli occorrenze di `${CLAUDE_SKILL_DIR}` si espandono a quella directory. La regola `allowed-tools` corrisponde quindi al comando esatto che il corpo della skill dice a Claude di eseguire, quindi lo script viene eseguito senza prompt.

La sostituzione `${CLAUDE_PROJECT_DIR}` richiede Claude Code v2.1.196 o successivo.

Gli argomenti indicizzati usano le virgolette in stile shell, quindi racchiudi i valori multi-parola tra virgolette per passarli come un singolo argomento. Ad esempio, `/my-skill "hello world" second` fa sì che `$0` si espanda a `hello world` e `$1` a `second`. Il placeholder `$ARGUMENTS` si espande sempre alla stringa di argomenti completa come digitata.

Un placeholder indicizzato senza argomento corrispondente, come `$2` quando è stato passato un solo argomento, rimane nel contenuto invariato. Un placeholder denominato dal frontmatter [`arguments`](#frontmatter-reference) senza argomento corrispondente si espande a una stringa vuota.

Se passi un valore di argomento che contiene testo come `$1` o `$ARGUMENTS`, Claude Code lo inserisce come testo letterale e non lo espande. Ad esempio, se il corpo di una skill contiene `Summarize $0` e esegui `/summarize "$ARGUMENTS from yesterday"`, Claude riceve `Summarize $ARGUMENTS from yesterday`. Claude Code sostituisce comunque le variabili `${CLAUDE_*}` come `${CLAUDE_SKILL_DIR}` dopo aver inserito gli argomenti.

Per includere un `$` letterale prima di una cifra, `ARGUMENTS` o un nome di argomento dichiarato, come `$1.00` in prosa, sfuggilo con una barra rovesciata: `\$1.00`. Una barra rovesciata prima di qualsiasi altro `$` viene lasciata invariata. Solo una singola barra rovesciata direttamente prima del token lo sfugge. Una barra rovesciata raddoppiata come `\\$1` lascia entrambe le barre rovesciate in posizione, e `$1` si espande comunque al valore dell'argomento. L'escape della barra rovesciata copre solo questi placeholder di argomenti. Una barra rovesciata non impedisce la sostituzione di una variabile `${CLAUDE_*}` dove la variabile si applica.

**Esempio usando sostituzioni:**

```yaml theme={null}
---
name: session-logger
description: Log activity for this session
---

Log the following to logs/${CLAUDE_SESSION_ID}.log:

$ARGUMENTS
```

<h3 id="add-supporting-files">
  Aggiungere file di supporto
</h3>

Le skills possono includere più file nella loro directory. Questo mantiene `SKILL.md` focalizzato sull'essenziale mentre consente a Claude di accedere a materiale di riferimento dettagliato solo quando necessario. Grandi documenti di riferimento, specifiche API o raccolte di esempi non hanno bisogno di caricarsi nel contesto ogni volta che la skill viene eseguita.

```text theme={null}
my-skill/
├── SKILL.md (required - overview and navigation)
├── reference.md (detailed API docs - loaded when needed)
├── examples.md (usage examples - loaded when needed)
└── scripts/
    └── helper.py (utility script - executed, not loaded)
```

Fai riferimento ai file di supporto da `SKILL.md` in modo che Claude sappia cosa contiene ogni file e quando caricarlo:

```markdown theme={null}
## Additional resources

- For complete API details, see [reference.md](reference.md)
- For usage examples, see [examples.md](examples.md)
```

<Tip>Mantieni `SKILL.md` sotto 500 righe. Sposta il materiale di riferimento dettagliato in file separati.</Tip>

<h3 id="control-who-invokes-a-skill">
  Controllare chi invoca una skill
</h3>

Per impostazione predefinita, sia tu che Claude potete invocare qualsiasi skill. Puoi digitare `/skill-name` per invocarla direttamente, e Claude può caricarla automaticamente quando rilevante per la tua conversazione. Due campi frontmatter ti permettono di limitare questo:

* **`disable-model-invocation: true`**: Solo tu puoi invocare la skill. Usa questo per flussi di lavoro con effetti collaterali o che vuoi controllare il timing, come `/commit`, `/deploy` o `/send-slack-message`. Non vuoi che Claude decida di distribuire perché il tuo codice sembra pronto.

* **`user-invocable: false`**: Solo Claude può invocare la skill. Usa questo per conoscenze di background che non sono azionabili come comando. Una skill `legacy-system-context` spiega come funziona un vecchio sistema. Claude dovrebbe saperlo quando rilevante, ma `/legacy-system-context` non è un'azione significativa per gli utenti da intraprendere.

Questo esempio crea una skill di distribuzione che solo tu puoi attivare. Se imposti `disable-model-invocation: true`, Claude non può eseguire la skill automaticamente:

```yaml theme={null}
---
name: deploy
description: Deploy the application to production
disable-model-invocation: true
---

Deploy $ARGUMENTS to production:

1. Run the test suite
2. Build the application
3. Push to the deployment target
4. Verify the deployment succeeded
```

Se Claude ci prova comunque, Claude Code blocca la chiamata e gli istruisce di non riprodurre i passaggi di distribuzione in un altro modo, quindi aspettati che Claude suggerisca di eseguire `/deploy` tu stesso.

Ecco come i due campi influenzano l'invocazione e il caricamento del contesto:

| Frontmatter                      | Puoi invocare | Claude può invocare | Quando caricato nel contesto                                              |
| :------------------------------- | :------------ | :------------------ | :------------------------------------------------------------------------ |
| (predefinito)                    | Sì            | Sì                  | Descrizione sempre nel contesto, skill completa si carica quando invocata |
| `disable-model-invocation: true` | Sì            | No                  | Descrizione non nel contesto, skill completa si carica quando la invochi  |
| `user-invocable: false`          | No            | Sì                  | Descrizione sempre nel contesto, skill completa si carica quando invocata |

<Note>
  In una sessione regolare, le descrizioni delle skills vengono caricate nel contesto in modo che Claude sappia cosa è disponibile, ma il contenuto completo della skill si carica solo quando invocato. [Subagent con skills precaricate](/docs/it/sub-agents#preload-skills-into-subagents) funzionano diversamente: il contenuto completo della skill viene iniettato all'avvio.
</Note>

<h3 id="skill-content-lifecycle">
  Ciclo di vita del contenuto della skill
</h3>

Quando tu o Claude invocate una skill, il contenuto `SKILL.md` renderizzato entra nella conversazione come un singolo messaggio e rimane lì nei turni successivi. Questa persistenza si applica alle istruzioni della skill, non ai suoi permessi: una concessione [`allowed-tools`](#pre-approve-tools-for-a-skill) si cancella quando invii il tuo prossimo messaggio. Claude Code non rilegge il file skill nei turni successivi, quindi scrivi una guida che dovrebbe applicarsi durante un'attività come istruzioni permanenti piuttosto che passaggi una tantum.

Quando Claude reinvoca una skill il cui contenuto renderizzato è identico alla copia già nel contesto, Claude Code aggiunge una breve nota che la skill è già caricata piuttosto che una seconda copia del contenuto. Quando il contenuto renderizzato differisce, perché gli argomenti sono cambiati o un comando di [contesto dinamico](#inject-dynamic-context) ha prodotto un nuovo output, Claude Code aggiunge il contenuto completo di nuovo.

La [compattazione automatica](/docs/it/how-claude-code-works#when-context-fills-up) porta avanti le skills invocate all'interno di un budget di token. Quando la conversazione viene riassunta per liberare contesto, Claude Code riallega l'invocazione più recente di ogni skill dopo il riassunto, mantenendo i primi 5.000 token di ciascuna. Le skills riallegate condividono un budget combinato di 25.000 token. Claude Code riempie questo budget a partire dalla skill invocata più di recente, quindi le skill più vecchie possono essere completamente eliminate dopo la compattazione se ne hai invocate molte in una sessione.

Se una skill sembra smettere di influenzare il comportamento dopo la prima risposta, il contenuto è solitamente ancora presente e il modello sta scegliendo altri strumenti o approcci. Rafforza la `description` della skill e le istruzioni in modo che il modello continui a preferirla, o usa [hooks](/docs/it/hooks) per applicare il comportamento in modo deterministico. Se la skill è grande o hai invocato molte altre dopo di essa, reinvocala dopo la compattazione per ripristinare il contenuto completo.

<h3 id="pre-approve-tools-for-a-skill">
  Pre-approvare strumenti per una skill
</h3>

Il campo `allowed-tools` concede il permesso per gli strumenti elencati durante il turno che invoca la skill, in modo che Claude possa usarli senza chiederti l'approvazione. La concessione si cancella quando invii il tuo prossimo messaggio, anche se il contenuto della skill [rimane nel contesto](#skill-content-lifecycle); invocare di nuovo la skill riapplica il permesso per quel turno. Non limita quali strumenti sono disponibili: ogni strumento rimane chiamabile, e le tue [impostazioni di permesso](/docs/it/permissions) governano ancora gli strumenti che non sono elencati. Per pre-approvare strumenti per l'intera sessione piuttosto che un singolo turno, aggiungi regole di consentimento a quelle impostazioni di permesso invece.

La fiducia dell'area di lavoro non controlla questo campo. Claude Code applica l'`allowed-tools` di una skill di progetto ogni volta che tu o Claude invocate la skill, incluso in un'esecuzione `-p` in una cartella che non hai mai fidata. Una skill può concedere a se stessa un accesso ampio agli strumenti, quindi esamina l'`allowed-tools` delle skills archiviate in un repository prima di eseguire Claude Code lì.

Questa skill consente a Claude di eseguire comandi git senza approvazione per uso ogni volta che invochi la skill:

```yaml theme={null}
---
name: commit
description: Stage and commit the current changes
disable-model-invocation: true
allowed-tools: Bash(git add *) Bash(git commit *) Bash(git status *)
---
```

Per rimuovere strumenti dal pool disponibile di Claude mentre una skill è attiva, elencali in `disallowed-tools` nel frontmatter della skill. La restrizione si cancella quando invii il tuo prossimo messaggio. Come le regole di negazione, il campo non può rimuovere [`EndConversation`](/docs/it/tools-reference#endconversation-tool-behavior) mentre rimane qualsiasi altro strumento. Per bloccare gli strumenti in tutte le skills e i prompt, aggiungi regole di negazione nelle tue [impostazioni di permesso](/docs/it/permissions).

<h3 id="pass-arguments-to-skills">
  Passare argomenti alle skills
</h3>

Sia tu che Claude potete passare argomenti quando invocate una skill. Gli argomenti sono disponibili tramite il placeholder `$ARGUMENTS`.

Questa skill corregge un problema GitHub per numero. Il placeholder `$ARGUMENTS` viene sostituito con qualsiasi cosa segua il nome della skill:

```yaml theme={null}
---
name: fix-issue
description: Fix a GitHub issue
disable-model-invocation: true
---

Fix GitHub issue $ARGUMENTS following our coding standards.

1. Read the issue description
2. Understand the requirements
3. Implement the fix
4. Write tests
5. Create a commit
```

Quando esegui `/fix-issue 123`, Claude riceve "Fix GitHub issue 123 following our coding standards..."

Se invochi una skill con argomenti ma nessun placeholder nel contenuto della skill riceve uno, Claude Code aggiunge `ARGUMENTS: <your input>` alla fine del contenuto della skill in modo che Claude veda comunque quello che hai digitato. Un placeholder è `$ARGUMENTS`, una forma indicizzata come `$1`, o un argomento denominato. Un placeholder indicizzato senza argomento alla sua posizione rimane come testo letterale e non conta come ricevente uno. Un placeholder denominato conta anche quando la sua posizione non ha argomento, perché si espande a una stringa vuota.

Puoi anche impilare diverse skills all'inizio di un messaggio. Digitando `/write-tests /fix-issue 123` carichi entrambe le skills e passi il testo finale `123` come `$ARGUMENTS` a ciascuna di esse. Prima della v2.1.199, solo la prima skill si caricava e riceveva `/fix-issue 123` come testo di argomento letterale.

Claude Code espande la prima skill più fino a cinque altre impilate dopo di essa. L'espansione si ferma al primo token che non è una skill invocabile dall'utente inline, quindi una skill che viene eseguita come [subagent biforcato](#run-skills-in-a-subagent), come [`/code-review`](/docs/it/code-review#review-a-diff-locally), o una i cui argomenti potrebbero essi stessi iniziare con un comando slash, come `/loop`, termina anche lì. Quel token e tutto ciò che segue diventa il testo di argomento per ogni skill espansa. `/code-review` viene eseguito come subagent biforcato dalla v2.1.218; nelle versioni precedenti veniva eseguito inline e impilato.

Per accedere ai singoli argomenti per posizione, usa `$ARGUMENTS[N]` o il più breve `$N`:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one language to another
---

Migrate the $ARGUMENTS[0] component from $ARGUMENTS[1] to $ARGUMENTS[2].
Preserve all existing behavior and tests.
```

Eseguendo `/migrate-component SearchBar JavaScript TypeScript` sostituisce `$ARGUMENTS[0]` con `SearchBar`, `$ARGUMENTS[1]` con `JavaScript` e `$ARGUMENTS[2]` con `TypeScript`. La stessa skill usando l'abbreviazione `$N`:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one language to another
---

Migrate the $0 component from $1 to $2.
Preserve all existing behavior and tests.
```

<h2 id="advanced-patterns">
  Modelli avanzati
</h2>

<h3 id="inject-dynamic-context">
  Iniettare contesto dinamico
</h3>

La sintassi `` !`<command>` `` esegue comandi shell prima che il contenuto della skill sia inviato a Claude. L'output del comando sostituisce il placeholder, quindi Claude riceve dati effettivi, non il comando stesso. Claude Code non esegue questi comandi sulla tua macchina quando la skill è [sincronizzata dal tuo account claude.ai](#how-claude-code-handles-the-body-of-a-synced-skill). Questa restrizione richiede Claude Code v2.1.228 o successivo.

Questa skill riassume una pull request recuperando dati PR live con GitHub CLI. I comandi `` !`gh pr diff` `` e altri vengono eseguiti per primi, e il loro output viene inserito nel prompt:

```yaml theme={null}
---
name: pr-summary
description: Summarize changes in a pull request
context: fork
agent: Explore
allowed-tools: Bash(gh *)
---

## Pull request context
- PR diff: !`gh pr diff`
- PR comments: !`gh pr view --comments`
- Changed files: !`gh pr diff --name-only`

## Your task
Summarize this pull request...
```

La sostituzione viene eseguita una sola volta sul file originale. L'output del comando viene inserito come testo semplice e non viene nuovamente scansionato per ulteriori placeholder `` !`<command>` ``, quindi un comando non può emettere un placeholder per un passaggio successivo da espandere.

La forma inline viene riconosciuta solo quando `!` appare all'inizio di una riga o immediatamente dopo uno spazio. Se `!` segue un altro carattere, come in `` KEY=!`cmd` ``, il placeholder viene lasciato come testo letterale e il comando non viene eseguito.

Per comandi multi-riga, utilizza un blocco di codice delimitato con ` ```! ` invece della forma inline:

````markdown theme={null}
## Environment
```!
node --version
git status --short
```
````

Per disabilitare questo comportamento per le skill e i comandi personalizzati da fonti utente, progetto, plugin o [additional-directory](#skills-from-additional-directories), imposta `"disableSkillShellExecution": true` in [settings](/docs/it/settings). Ogni comando viene sostituito con `[shell command execution disabled by policy]` invece di essere eseguito. Le skill bundled e gestite non sono interessate. Questa impostazione è più utile in [managed settings](/docs/it/managed-settings), dove gli utenti non possono sovrascriverla.

Claude Code non esegue mai questi comandi sulla tua macchina quando appaiono in skill [sincronizzate dal tuo account claude.ai](#how-synced-skills-behave), indipendentemente da questa impostazione. Questa restrizione richiede Claude Code v2.1.228 o successivo. [How Claude Code handles the body of a synced skill](#how-claude-code-handles-the-body-of-a-synced-skill) dice cosa Claude riceve al posto del comando in ogni tipo di sessione.

<Tip>
  Per richiedere un ragionamento più profondo quando una skill viene eseguita, includi `ultrathink` da qualsiasi parte nel contenuto della skill. Vedi [Use ultrathink for one-off deep reasoning](/docs/it/model-config#use-ultrathink-for-one-off-deep-reasoning).
</Tip>

<h4 id="how-injected-commands-run">
  Come vengono eseguiti i comandi iniettati
</h4>

Claude Code sceglie lo strumento che esegue i comandi iniettati di una skill dalla chiave `shell` nel frontmatter della skill e dal tuo ambiente. Ogni combinazione esegue i comandi attraverso lo strumento Bash o lo strumento PowerShell, tranne una che fallisce completamente l'invocazione:

* `shell: powershell`, con lo [strumento PowerShell](/docs/it/tools-reference#powershell-tool) abilitato: i comandi vengono eseguiti attraverso lo strumento PowerShell.
* `shell: bash` quando bash non è disponibile: l'invocazione fallisce prima che qualsiasi comando venga eseguito. Questo accade su Windows senza Git Bash. Claude Code mostra ``Skill <name> requires bash (`shell: bash` in frontmatter) but Git Bash was not found``.
* Qualsiasi altra combinazione: i comandi vengono eseguiti attraverso lo strumento Bash quando bash è disponibile. Quando non lo è, vengono eseguiti attraverso lo strumento PowerShell.

Entrambi gli strumenti eseguono i comandi nello stesso modo in cui Claude esegue i propri comandi shell. Condividono la directory di lavoro, il timeout e la gestione dell'output:

* **Directory di lavoro**: Claude Code esegue ogni comando nella directory di lavoro corrente della shell della sessione. Quella directory si sposta quando Claude esegue `cd`. Utilizza [`${CLAUDE_SKILL_DIR}` o `${CLAUDE_PROJECT_DIR}`](#available-string-substitutions) nei percorsi che devono risolversi nello stesso modo ogni volta.
* **stderr**: con la shell `bash` predefinita, Claude Code unisce stderr in stdout. Tutto ciò che il comando scrive su stderr appare nel testo iniettato.
* **Timeout**: ogni comando viene eseguito sotto il [timeout](/docs/it/tools-reference#timeout-and-output-limits) predefinito di 2 minuti dello strumento Bash. Quando lo strumento Bash [sposta un comando scaduto in background](/docs/it/tools-reference#background-commands), la skill viene comunque renderizzata. Il testo iniettato segnala lo spostamento e nomina l'attività in background e il file che raccoglie l'output del comando. Quando il comando è uno che lo strumento Bash non mette mai automaticamente in background, Claude Code lo termina al timeout. Quel fallimento [interrompe l'invocazione](#when-an-injected-command-fails).
* **Dimensione dell'output**: l'output oltre il limite inline dello strumento Bash arriva come percorso file più un'anteprima breve, non testo troncato. [Output limits](/docs/it/tools-reference#output-limits) copre il limite e come regolare ogni confine.

Lo strumento PowerShell applica lo stesso timeout, backgrounding e comportamento di output-ceiling ai comandi che esegue. Vedi la sezione [PowerShell tool](/docs/it/tools-reference#powershell-tool) per i suoi dettagli.

<h4 id="when-an-injected-command-fails">
  Quando un comando iniettato fallisce
</h4>

Un comando fallito interrompe l'intera invocazione della skill, non solo il suo placeholder. Claude non vede mai il contenuto della skill per quella invocazione. L'interruzione mostra `Shell command failed for pattern "..."`. Il messaggio di errore include l'output del comando sotto `[stderr]`.

Con la shell `bash` predefinita, qualsiasi codice di uscita diverso da zero conta come un fallimento. Si applica un'eccezione: Claude Code tratta il codice di uscita 1 dai [comandi di ricerca e confronto](/docs/it/tools-reference#output-limits) come un risultato normale e inietta il loro output. I codici di uscita 2 o superiori falliscono anche per quei comandi.

Quali comandi ottengono l'eccezione dipende dalla shell:

* Shell `bash` predefinita: i comandi elencati sotto [Output limits](/docs/it/tools-reference#output-limits)
* `shell: powershell`, quando lo strumento PowerShell è abilitato: un [set diverso](/docs/it/tools-reference#shell-selection-in-settings-hooks-and-skills) che include `grep` e `git diff` ma non `find` o `diff`

Con la shell `bash` predefinita, aggiungi `|| true` a qualsiasi altro comando che ti aspetti esca con un codice non zero. Uno script di controllo che esce con 1 quando trova problemi è un esempio.

<h4 id="permission-checks-on-injected-commands">
  Controlli di permesso sui comandi iniettati
</h4>

I comandi iniettati non richiedono mai il permesso mentre la skill viene renderizzata. Claude Code controlla ogni comando rispetto alle tue [regole di permesso](/docs/it/permissions) per primo. Un comando che una regola deny corrisponde interrompe l'invocazione con `Shell command permission check failed for pattern "..."`.

Al di fuori della [modalità auto](/docs/it/permission-modes#eliminate-prompts-with-auto-mode), quando il controllo dei permessi di un comando restituisce qualcosa di diverso da allow, Claude Code interrompe l'invocazione con lo stesso errore. Questo include una regola che normalmente ti chiederebbe. Per evitare che un comando non corrispondente interrompa qui, pre-approvalo con [`allowed-tools`](#pre-approve-tools-for-a-skill). Le regole deny e ask sovrascrivono comunque `allowed-tools`. Vedi [Manage permissions](/docs/it/permissions#manage-permissions).

In modalità auto, un comando che altrimenti avrebbe bisogno della tua approvazione non interrompe l'invocazione. La skill si carica con un'istruzione che dice a Claude di eseguire il comando per primo, e la chiamata di Claude passa quindi attraverso i [controlli usuali della modalità auto](/docs/it/permission-modes#how-the-classifier-evaluates-actions). L'invocazione interrompe comunque in una [skill forkata](#run-skills-in-a-subagent) che imposta `agent`, e in una sessione dove Claude non ha lo [strumento shell che esegue i comandi iniettati](#how-injected-commands-run).

<h3 id="run-skills-in-a-subagent">
  Eseguire skill in un subagent
</h3>

Aggiungi `context: fork` al tuo frontmatter quando vuoi che una skill venga eseguita in isolamento. Claude Code avvia un nuovo subagent del tipo impostato nel campo `agent` e gli fornisce il contenuto della skill come suo prompt. Il subagent non vede la cronologia della tua conversazione, quindi le istruzioni della skill devono stare da sole.

<Note>
  Nonostante il nome, una skill con `context: fork` non viene eseguita in un [fork della conversazione corrente](/docs/it/sub-agents#fork-the-current-conversation), che darebbe al subagent tutto ciò che hai discusso finora. Quando l'attività dipende da quella cronologia, fai il fork della conversazione invece di usare `context: fork`.
</Note>

Il subagent forkato viene eseguito in [background](/docs/it/sub-agents#run-subagents-in-foreground-or-background): continui a lavorare mentre viene eseguito, e il suo risultato arriva nella tua conversazione quando si completa. Imposta `background: false` nel frontmatter per invece attendere il risultato nel turno che ha invocato la skill. Prima della v2.1.218, le skill forkate bloccavano sempre il turno fino al completamento.

Claude Code attende anche il risultato, anche quando la skill non imposta `background: false`, in casi come questi:

* In modalità non interattiva, con il flag `-p` o l'Agent SDK
* Quando imposti [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/it/env-vars) a `1`, che disattiva anche tutte le altre funzionalità di attività in background
* Quando invochi una skill forkata mentre un'invocazione precedente della stessa skill è ancora in esecuzione
* Quando un [scheduled task](/docs/it/scheduled-tasks) si attiva con la skill come suo prompt

Un fork in background viene eseguito anche con il [set di strumenti più ristretto che si applica ai subagent in background](/docs/it/sub-agents#run-subagents-in-foreground-or-background): il subagent della skill è un tipo di agent regolare, quindi l'esenzione per i subagent che forkano la conversazione non lo copre. Se i passaggi della tua skill dipendono da uno strumento al di fuori di quel set, imposta `background: false` per mantenere il set di strumenti completo.

Una skill forkata che viene eseguita in background applica i suoi edit al di fuori dei [checkpoints](/docs/it/checkpointing) della tua sessione, quindi `/rewind` non li annulla; usa git per ripristinarli.

<Warning>
  `context: fork` ha senso solo per skill con istruzioni esplicite. Se la tua skill contiene linee guida come "usa queste convenzioni API" senza un compito, il subagent riceve le linee guida ma nessun prompt azionabile, e ritorna senza output significativo.
</Warning>

Le skill e i [subagent](/docs/it/sub-agents) lavorano insieme in due direzioni:

| Approccio                   | System prompt               | Task                          | Carica anche                                                                                               |
| :-------------------------- | :-------------------------- | :---------------------------- | :--------------------------------------------------------------------------------------------------------- |
| Skill con `context: fork`   | Da tipo di agent            | Contenuto SKILL.md            | CLAUDE.md, per il [startup context](/docs/it/sub-agents#what-loads-at-startup) dell'agent                       |
| Subagent con campo `skills` | Corpo markdown del subagent | Messaggio di delega di Claude | Skill precaricate + CLAUDE.md, per il [startup context](/docs/it/sub-agents#what-loads-at-startup) del subagent |

Con `context: fork`, scrivi il compito nella tua skill e scegli un tipo di agent per eseguirlo. Gli agent Explore e Plan built-in [saltano CLAUDE.md e git status](/docs/it/sub-agents#what-loads-at-startup) per mantenere il loro contesto piccolo, quindi una skill forkata che usa `agent: Explore` vede solo il contenuto SKILL.md e il prompt di sistema dell'agent. Per l'inverso, dove definisci un subagent personalizzato che usa le skill come materiale di riferimento, vedi [Subagents](/docs/it/sub-agents#preload-skills-into-subagents).

<h4 id="example-research-skill-using-explore-agent">
  Esempio: Skill di ricerca usando l'agent Explore
</h4>

Questa skill esegue ricerche in un agent Explore forkato. Il contenuto della skill diventa il compito, e l'agent fornisce strumenti di sola lettura ottimizzati per l'esplorazione della codebase:

```yaml theme={null}
---
name: deep-research
description: Research a topic thoroughly
context: fork
agent: Explore
---

Research $ARGUMENTS thoroughly:

1. Find relevant files using Glob and Grep
2. Read and analyze the code
3. Summarize findings with specific file references
```

Quando questa skill viene eseguita:

1. Viene creato un nuovo contesto isolato
2. Il subagent riceve il contenuto della skill come suo prompt (le istruzioni "Research \$ARGUMENTS thoroughly")
3. Il campo `agent` determina l'ambiente di esecuzione (modello, strumenti e permessi)
4. Il subagent riassume i suoi risultati e li restituisce alla tua conversazione principale quando finisce

Il campo `agent` specifica quale configurazione di subagent utilizzare. Le opzioni includono agent built-in (`Explore`, `Plan`, `general-purpose`) o qualsiasi subagent personalizzato da `.claude/agents/`. Se omesso, utilizza `general-purpose`.

<h3 id="restrict-claude’s-skill-access">
  Limitare l'accesso alle skill di Claude
</h3>

Per impostazione predefinita, Claude può invocare qualsiasi skill che non abbia `disable-model-invocation: true` impostato. Le skill che definiscono `allowed-tools` concedono a Claude l'accesso a quegli strumenti senza approvazione per uso durante il turno che invoca la skill; la concessione si cancella quando invii il tuo prossimo messaggio. Le tue [impostazioni di permesso](/docs/it/permissions) governano comunque il comportamento di approvazione di base per tutti gli altri strumenti. Alcuni comandi built-in sono anche disponibili attraverso lo strumento Skill, inclusi `/init` e `/security-review`. Altri comandi built-in come `/compact` non lo sono.

Tre modi per controllare quali skill Claude può invocare:

**Disabilitare tutte le skill** negando lo strumento Skill in `/permissions`:

```text theme={null}
# Add to deny rules:
Skill
```

**Consentire o negare skill specifiche** usando [regole di permesso](/docs/it/permissions):

```text theme={null}
# Allow only specific skills
Skill(commit)
Skill(review-pr *)

# Deny specific skills
Skill(deploy *)
```

Sintassi di permesso: `Skill(name)` per corrispondenza esatta, `Skill(name *)` per corrispondenza di prefisso con qualsiasi argomento.

Se la tua regola `deny` nomina un alias o un nome non qualificato piuttosto che il nome della skill stessa, Claude Code blocca comunque la skill: con `Skill(review)` blocca il bundled `/code-review` attraverso il suo alias `/review`, e con `Skill(deploy)` blocca una [skill annidata](#where-skills-live) elencata come `apps/web:deploy` attraverso il suo nome non qualificato. Prima della v2.1.260, Claude Code non bloccava una skill annidata elencata sotto il suo nome qualificato quando la regola deny nominava solo il nome non qualificato.

Claude Code corrisponde a una regola `allow` solo contro il nome della skill stessa e il nome nell'invocazione di Claude.

**Nascondere skill individuali** aggiungendo `disable-model-invocation: true` al loro frontmatter. Questo rimuove la skill dal contesto di Claude interamente.

<Note>
  Con `user-invocable: false`, non puoi invocare la skill, ma Claude ancora può. Per impedire a Claude di invocarla attraverso lo strumento Skill, imposta `disable-model-invocation: true`.
</Note>

<h3 id="override-skill-visibility-from-settings">
  Sovrascrivere la visibilità della skill dalle impostazioni
</h3>

L'impostazione `skillOverrides` controlla la visibilità della skill dalle tue [impostazioni](/docs/it/settings) invece dal frontmatter della skill stessa. Usala per skill il cui SKILL.md non vuoi modificare, come quelle controllate in un repo di progetto condiviso. Il menu `/skills` lo scrive per te: evidenzia una skill e premi `Space` per ciclo gli stati, quindi `Esc` per salvare in `.claude/settings.local.json`.

Ogni chiave è un nome di skill e ogni valore è uno di quattro stati:

| Valore                  | Elencato a Claude  | Nel menu `/` |
| :---------------------- | :----------------- | :----------- |
| `"on"`                  | Nome e descrizione | Sì           |
| `"name-only"`           | Solo nome          | Sì           |
| `"user-invocable-only"` | Nascosto           | Sì           |
| `"off"`                 | Nascosto           | Nascosto     |

Il menu `/skills` etichetta lo stato `"user-invocable-only"` come `user-only`.

A partire dalla v2.1.199, `"off"` nasconde anche la skill dagli elenchi di comandi pubblicizzati ai client [Remote Control](/docs/it/remote-control) e ai chiamanti [Agent SDK](/docs/it/agent-sdk/skills#discover-available-commands), oltre al menu `/` del terminale. Invocare una skill nascosta per il suo nome completo restituisce comunque l'errore `skillOverrides` invece di eseguirla.

Una skill assente da `skillOverrides` viene trattata come `"on"`. L'esempio seguente comprime una skill al suo nome e disattiva completamente un'altra:

```json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

Alcune skill bundled hanno alias, come `checkup` per `/doctor`. Se imposti una voce `skillOverrides` sotto un alias in [managed settings](/docs/it/managed-settings) o in un file che passi con il flag `--settings`, Claude Code la applica alla skill dietro l'alias. Puoi solo limitare una skill ulteriormente attraverso un alias, mai renderla più visibile, e se imposti anche una voce sotto il nome della skill stessa in managed settings, quella voce ha la precedenza. Prima della v2.1.260, Claude Code non applicava una voce sotto un alias alla skill in nessuna fonte di impostazioni.

Nelle impostazioni utente, progetto e locale, Claude Code corrisponde alle voci solo contro i nomi delle skill. Se imposti una voce per `review` lì, si applica a una skill denominata `review`, non al bundled `/code-review` attraverso il suo alias `/review`.

Le skill dei plugin non sono interessate da `skillOverrides`. Gestisci quelle attraverso `/plugin` invece.

<h3 id="find-unused-skills">
  Trovare skill inutilizzate
</h3>

Ogni skill nell'[elenco delle skill](#skill-descriptions-are-cut-short) aggiunge al tuo contesto ad ogni turno, indipendentemente dal fatto che Claude la usi mai. Esegui `/skill-doctor` per vedere cosa costa ogni tua skill e quanto spesso viene utilizzata, così puoi decidere quali disattivare. In una sessione interattiva, il rapporto si apre nella scheda **Stats** del gestore `/plugin`. In [modalità non interattiva](/docs/it/headless) con `-p`, Claude Code lo stampa come testo.

Il rapporto copre le skill nella tua sessione diverse dalle skill bundled e dalle skill enterprise. Segnala le skill nell'elenco che non sono mai state invocate e dice dove disattivarle. Delle skill che ti dice dove disattivare, inizia con quelle che hanno il costo di contesto più alto. Il rapporto elenca anche i plugin che non hai utilizzato di recente.

`/skill-doctor` richiede Claude Code v2.1.252 o successivo e non è disponibile in sessioni che saltano [feature-flag fetching](/docs/it/env-vars#features-that-need-feature-flag-fetching). Se esegui `/skill-doctor` su [Remote Control](/docs/it/remote-control) dal tuo telefono o browser, Claude Code risponde [`Skill usage reports are not available on this connection.`](/docs/it/errors#skill-usage-reports-are-not-available-on-this-connection) invece. Esegui `/skill-doctor` nel terminale sulla macchina dove la sessione è in esecuzione.

<h2 id="evaluate-and-iterate-on-a-skill">
  Valutare e iterare su una skill
</h2>

Vedere una skill attivata ti dice che Claude l'ha trovata, non che abbia fatto quello che intendevi. Per sapere che una skill funziona, misura separatamente se Claude la invoca sui prompt che dovrebbe, e se l'output corrisponde a quello che ti aspetti quando lo fa.

Il controllo per entrambi è un confronto di base. Raccogli alcuni prompt realistici, esegui ognuno in una sessione nuova con la skill disponibile e di nuovo con essa [disabilitata](#override-skill-visibility-from-settings), e confronta i risultati. Una sessione nuova è importante perché il contesto residuo dalla creazione della skill maschererà le lacune nelle istruzioni scritte.

Due strumenti automatizzano quel confronto. Per una skill che viene fornita in un [plugin](/docs/it/plugins/overview), [`claude plugin eval`](/docs/it/plugin-evals) esegue ogni prompt in una sessione isolata con e senza il plugin, la valuta con grader che definisci o che scrive per te, e esce con un codice diverso da zero al di sotto di una soglia in modo da poter controllare il CI su di essa. Per iterare su una singola skill all'interno di una conversazione Claude Code, il plugin skill-creator di seguito esegue un ciclo simile con il suo formato `evals/evals.json`. I due formati non sono intercambiabili.

<h3 id="run-evals-with-skill-creator">
  Esegui eval con skill-creator
</h3>

Il [`skill-creator` plugin](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/skill-creator) automatizza il ciclo di confronto all'interno di Claude Code. Installalo dal marketplace ufficiale:

```text theme={null}
/plugin install skill-creator@claude-plugins-official
```

Se l'installazione fallisce, fai corrispondere il messaggio che Claude Code segnala:

* `Marketplace "claude-plugins-official" not found`: aggiungi il marketplace con `/plugin marketplace add anthropics/claude-plugins-official`, quindi riprova l'installazione.
* Il plugin [non è trovato nel marketplace](/docs/it/plugins/install#install-a-plugin): controlla il nome del plugin.

Se il riepilogo dell'installazione segnala `Run /reload-plugins to activate.`, Claude Code esegue quindi quel ricaricamento per te. Se il ricaricamento avverte che il tuo prossimo messaggio rileggerebbe la conversazione, esegui `/reload-plugins --force` per rendere disponibili le skill del plugin nella sessione corrente. Quindi chiedi a Claude di valutare una skill esistente, ad esempio `evaluate my summarize-changes skill with skill-creator`. Il plugin ti guida attraverso la scrittura dei test case ed esegue il ciclo:

* **Test cases**: memorizza prompt, file di input e comportamento previsto in `evals/evals.json` all'interno della directory della skill
* **Isolated runs**: genera un [subagent](/docs/it/sub-agents) per test case in modo che ogni esecuzione inizi con un contesto pulito, e registra il conteggio dei token e la durata
* **Grading**: controlla ogni asserzione rispetto all'output e scrive pass o fail con evidenza in `grading.json`
* **Benchmark**: aggrega il pass rate, il tempo e i token per with-skill versus without-skill in `benchmark.json` in modo da poter confrontare il miglioramento del pass-rate rispetto al sovraccarico di token e tempo
* **Version comparison**: esegue un blind A/B tra due versioni della skill in modo da poter confermare che una modifica è un miglioramento prima di eseguirne il commit
* **Description tuning**: genera prompt should-trigger e should-not-trigger, misura il hit rate e propone modifiche alla descrizione quando la skill si attiva su richieste sbagliate
* **Review viewer**: apre un report HTML dove puoi ispezionare ogni output e registrare feedback qualitativo che l'iterazione successiva legge

Per il formato del file eval e il flusso di lavoro di iterazione completo, vedi [Evaluating skill output quality](https://agentskills.io/skill-creation/evaluating-skills) su agentskills.io. Per informazioni di base sui modi benchmark e comparison, vedi l'[annuncio skill-creator](https://claude.com/blog/improving-skill-creator-test-measure-and-refine-agent-skills).

<h2 id="share-skills">
  Condividere skills
</h2>

Gli skills possono essere distribuiti a diversi livelli a seconda del vostro pubblico:

* **Project skills**: Eseguire il commit di `.claude/skills/` nel controllo versione
* **Plugins**: Creare una directory `skills/` nel vostro [plugin](/docs/it/plugins/overview)
* **Managed**: Distribuire a livello organizzativo tramite [managed settings](/docs/it/managed-settings)

<h3 id="generate-visual-output">
  Generare output visuale
</h3>

Gli skills possono raggruppare ed eseguire script in qualsiasi linguaggio, fornendo a Claude capacità oltre ciò che è possibile in un singolo prompt. Un modello è la generazione di output visuale: file HTML interattivi che si aprono nel vostro browser per esplorare dati, eseguire il debug o creare report.

Questo esempio crea un codebase explorer: una vista ad albero interattiva dove potete espandere e comprimere directory, visualizzare le dimensioni dei file a colpo d'occhio e identificare i tipi di file per colore.

Creare la directory Skill:

```bash theme={null}
mkdir -p ~/.claude/skills/codebase-visualizer/scripts
```

Salvare questo in `~/.claude/skills/codebase-visualizer/SKILL.md`. La descrizione dice a Claude quando attivare questo Skill e le istruzioni dicono a Claude di eseguire lo script raggruppato. Il percorso dello script utilizza [`${CLAUDE_SKILL_DIR}`](#available-string-substitutions) in modo che si risolva correttamente indipendentemente dal fatto che lo skill sia installato a livello personale, di progetto o di plugin:

````yaml theme={null}
---
name: codebase-visualizer
description: Generate an interactive collapsible tree visualization of your codebase. Use when exploring a new repo, understanding project structure, or identifying large files.
allowed-tools: Bash(python3 *)
---

# Codebase Visualizer

Generate an interactive HTML tree view that shows your project's file structure with collapsible directories.

## Usage

Run the visualization script from your project root:

```bash
python3 ${CLAUDE_SKILL_DIR}/scripts/visualize.py .
```

This creates `codebase-map.html` in the current directory and opens it in your default browser.

## What the visualization shows

- **Collapsible directories**: Click folders to expand/collapse
- **File sizes**: Displayed next to each file
- **Colors**: Different colors for different file types
- **Directory totals**: Shows aggregate size of each folder
````

Salvare questo in `~/.claude/skills/codebase-visualizer/scripts/visualize.py`. Questo script scansiona un albero di directory e genera un file HTML autonomo con:

* Una **barra laterale di riepilogo** che mostra il conteggio dei file, il conteggio delle directory, la dimensione totale e il numero di tipi di file
* Un **grafico a barre** che suddivide la codebase per tipo di file (i primi 8 per dimensione)
* Un **albero comprimibile** dove potete espandere e comprimere directory, con indicatori di tipo di file codificati per colore

Lo script richiede Python 3 ma utilizza solo librerie integrate, quindi non ci sono pacchetti da installare:

```python expandable theme={null}
#!/usr/bin/env python3
"""Generate an interactive collapsible tree visualization of a codebase."""

import json
import sys
import webbrowser
from html import escape
from pathlib import Path
from collections import Counter

IGNORE = {'.git', 'node_modules', '__pycache__', '.venv', 'venv', 'dist', 'build'}

def scan(path: Path, stats: dict) -> dict:
    result = {"name": path.name, "children": [], "size": 0}
    try:
        for item in sorted(path.iterdir()):
            if item.name in IGNORE or item.name.startswith('.'):
                continue
            if item.is_file():
                size = item.stat().st_size
                ext = item.suffix.lower() or '(no ext)'
                result["children"].append({"name": item.name, "size": size, "ext": ext})
                result["size"] += size
                stats["files"] += 1
                stats["extensions"][ext] += 1
                stats["ext_sizes"][ext] += size
            elif item.is_dir():
                stats["dirs"] += 1
                child = scan(item, stats)
                if child["children"]:
                    result["children"].append(child)
                    result["size"] += child["size"]
    except PermissionError:
        pass
    return result

def generate_html(data: dict, stats: dict, output: Path) -> None:
    ext_sizes = stats["ext_sizes"]
    total_size = sum(ext_sizes.values()) or 1
    sorted_exts = sorted(ext_sizes.items(), key=lambda x: -x[1])[:8]
    colors = {
        '.js': '#f7df1e', '.ts': '#3178c6', '.py': '#3776ab', '.go': '#00add8',
        '.rs': '#dea584', '.rb': '#cc342d', '.css': '#264de4', '.html': '#e34c26',
        '.json': '#6b7280', '.md': '#083fa1', '.yaml': '#cb171e', '.yml': '#cb171e',
        '.mdx': '#083fa1', '.tsx': '#3178c6', '.jsx': '#61dafb', '.sh': '#4eaa25',
    }
    lang_bars = "".join(
        f'<div class="bar-row"><span class="bar-label">{ext}</span>'
        f'<div class="bar" style="width:{(size/total_size)*100}%;background:{colors.get(ext,"#6b7280")}"></div>'
        f'<span class="bar-pct">{(size/total_size)*100:.1f}%</span></div>'
        for ext, size in sorted_exts
    )
    def fmt(b):
        if b < 1024: return f"{b} B"
        if b < 1048576: return f"{b/1024:.1f} KB"
        return f"{b/1048576:.1f} MB"

    html = f'''<!DOCTYPE html>
<html><head>
  <meta charset="utf-8"><title>Codebase Explorer</title>
  <style>
    body {{ font: 14px/1.5 system-ui, sans-serif; margin: 0; background: #1a1a2e; color: #eee; }}
    .container {{ display: flex; height: 100vh; }}
    .sidebar {{ width: 280px; background: #252542; padding: 20px; border-right: 1px solid #3d3d5c; overflow-y: auto; flex-shrink: 0; }}
    .main {{ flex: 1; padding: 20px; overflow-y: auto; }}
    h1 {{ margin: 0 0 10px 0; font-size: 18px; }}
    h2 {{ margin: 20px 0 10px 0; font-size: 14px; color: #888; text-transform: uppercase; }}
    .stat {{ display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #3d3d5c; }}
    .stat-value {{ font-weight: bold; }}
    .bar-row {{ display: flex; align-items: center; margin: 6px 0; }}
    .bar-label {{ width: 55px; font-size: 12px; color: #aaa; }}
    .bar {{ height: 18px; border-radius: 3px; }}
    .bar-pct {{ margin-left: 8px; font-size: 12px; color: #666; }}
    .tree {{ list-style: none; padding-left: 20px; }}
    details {{ cursor: pointer; }}
    summary {{ padding: 4px 8px; border-radius: 4px; }}
    summary:hover {{ background: #2d2d44; }}
    .folder {{ color: #ffd700; }}
    .file {{ display: flex; align-items: center; padding: 4px 8px; border-radius: 4px; }}
    .file:hover {{ background: #2d2d44; }}
    .size {{ color: #888; margin-left: auto; font-size: 12px; }}
    .dot {{ width: 8px; height: 8px; border-radius: 50%; margin-right: 8px; }}
  </style>
</head><body>
  <div class="container">
    <div class="sidebar">
      <h1>📊 Summary</h1>
      <div class="stat"><span>Files</span><span class="stat-value">{stats["files"]:,}</span></div>
      <div class="stat"><span>Directories</span><span class="stat-value">{stats["dirs"]:,}</span></div>
      <div class="stat"><span>Total size</span><span class="stat-value">{fmt(data["size"])}</span></div>
      <div class="stat"><span>File types</span><span class="stat-value">{len(stats["extensions"])}</span></div>
      <h2>By file type</h2>
      {lang_bars}
    </div>
    <div class="main">
      <h1>📁 {escape(data["name"])}</h1>
      <ul class="tree" id="root"></ul>
    </div>
  </div>
  <script>
    const data = {json.dumps(data)};
    const colors = {json.dumps(colors)};
    function fmt(b) {{ if (b < 1024) return b + ' B'; if (b < 1048576) return (b/1024).toFixed(1) + ' KB'; return (b/1048576).toFixed(1) + ' MB'; }}
    function esc(s) {{ return s.replace(/[&<>"']/g, c => ({{"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}}[c])); }}
    function render(node, parent) {{
      if (node.children) {{
        const det = document.createElement('details');
        det.open = parent === document.getElementById('root');
        det.innerHTML = `<summary><span class="folder">📁 ${{esc(node.name)}}</span><span class="size">${{fmt(node.size)}}</span></summary>`;
        const ul = document.createElement('ul'); ul.className = 'tree';
        node.children.sort((a,b) => (b.children?1:0)-(a.children?1:0) || a.name.localeCompare(b.name));
        node.children.forEach(c => render(c, ul));
        det.appendChild(ul);
        const li = document.createElement('li'); li.appendChild(det); parent.appendChild(li);
      }} else {{
        const li = document.createElement('li'); li.className = 'file';
        li.innerHTML = `<span class="dot" style="background:${{colors[node.ext]||'#6b7280'}}"></span>${{esc(node.name)}}<span class="size">${{fmt(node.size)}}</span>`;
        parent.appendChild(li);
      }}
    }}
    data.children.forEach(c => render(c, document.getElementById('root')));
  </script>
</body></html>'''
    output.write_text(html)

if __name__ == '__main__':
    target = Path(sys.argv[1] if len(sys.argv) > 1 else '.').resolve()
    stats = {"files": 0, "dirs": 0, "extensions": Counter(), "ext_sizes": Counter()}
    data = scan(target, stats)
    out = Path('codebase-map.html')
    generate_html(data, stats, out)
    print(f'Generated {out.absolute()}')
    webbrowser.open(f'file://{out.absolute()}')
```

Per testare, aprite Claude Code in qualsiasi progetto e chiedete "Visualize this codebase." Claude esegue lo script, che stampa il percorso del file generato, come `Generated /path/to/codebase-map.html`, e lo apre nel vostro browser. Se lavorate in un ambiente headless dove nessun browser si apre, il percorso stampato conferma che lo script ha avuto successo.

Questo modello funziona per qualsiasi output visuale: grafici di dipendenza, report di copertura dei test, documentazione API o visualizzazioni dello schema del database. Lo script raggruppato fa il lavoro mentre Claude gestisce l'orchestrazione.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="skill-not-triggering">
  Skill not triggering
</h3>

Se Claude non utilizza la vostra skill quando previsto:

1. Verificate che la descrizione includa parole chiave che gli utenti direbbero naturalmente
2. Verificate che la skill appaia in `What skills are available?`
3. Provate a riformulare la vostra richiesta per corrispondere più strettamente alla descrizione
4. Invocatela direttamente con `/skill-name` se la skill è invocabile dall'utente

Se il YAML del frontmatter è malformato, Claude Code carica il corpo della skill con metadati vuoti, quindi `/skill-name` funziona comunque ma Claude non può corrispondere alla vostra `description`. Eseguite con `--debug` per vedere l'errore di parsing.

Se la skill è fornita in un plugin, potete misurare con quale frequenza si attiva su prompt realistici piuttosto che controllare uno alla volta: scrivete un caso di eval con un [grader `tool_used: Skill`](/docs/it/plugin-evals#create-your-first-eval-suite) ed eseguitelo con `claude plugin eval` dopo ogni modifica della descrizione.

Per trovare file `SKILL.md` il cui frontmatter non viene analizzato, eseguite [`claude plugin validate`](/docs/it/plugins/cli-reference#validate-a-directory) sulla directory delle skills, ad esempio `claude plugin validate .claude/skills` per le skills del progetto o `claude plugin validate ~/.claude/skills` per le skills personali. Richiede Claude Code v2.1.233 o successivo.

<h3 id="skill-triggers-too-often">
  Skill triggers too often
</h3>

Se Claude utilizza la vostra skill quando non lo desiderate:

1. Rendete la descrizione più specifica
2. Aggiungete `disable-model-invocation: true` se desiderate solo l'invocazione manuale

<h3 id="skill-descriptions-are-cut-short">
  Skill descriptions are cut short
</h3>

Claude Code carica un elenco di nomi e descrizioni delle skill nel contesto in modo che Claude sappia cosa è disponibile. L'elenco contiene sempre ogni nome di skill, ma se avete molte skills, Claude Code accorcia le descrizioni per adattarsi al budget di caratteri dell'elenco, il che può rimuovere le parole chiave di cui Claude ha bisogno per corrispondere alla vostra richiesta. Il budget si scala all'1% della finestra di contesto del modello. Quando l'elenco supera il limite, Claude Code elimina le descrizioni a partire dalle skills che invocate meno, quindi le skills che utilizzate di più mantengono il loro testo completo.

Eseguite `/doctor` per una stima del costo di contesto dell'elenco e dei suoi maggiori contributori. Per trovare le skills che vale la pena disattivare, eseguite [`/skill-doctor`](#find-unused-skills). Quando l'elenco supera il suo budget, Claude Code scrive anche un avviso nel log di debug, visibile con [`--debug`](/docs/it/cli-reference#cli-flags).

La riga Skills in `/context` riporta la dimensione dell'elenco dopo l'applicazione del budget, quindi corrisponde a ciò che il modello riceve. Prima della v2.1.196, la riga contava il testo completo di ogni descrizione e poteva mostrare un valore diverse volte più grande del budget configurato.

Per aumentare il budget, impostate l'impostazione [`skillListingBudgetFraction`](/docs/it/settings-reference#skilllistingbudgetfraction) (ad esempio `0.02` = 2%) o la variabile di ambiente `SLASH_COMMAND_TOOL_CHAR_BUDGET` a un conteggio di caratteri fisso. Per liberare budget per altre skills, impostate le voci a bassa priorità su `"name-only"` in [`skillOverrides`](#override-skill-visibility-from-settings) in modo che si elenchino senza una descrizione. Potete anche ridurre il testo di `description` e `when_to_use` alla fonte: mettete il caso d'uso chiave per primo, poiché il testo combinato di ogni voce è limitato a 1.536 caratteri indipendentemente dal budget. Il limite è configurabile con [`skillListingMaxDescChars`](/docs/it/settings-reference#skilllistingmaxdescchars).

<h3 id="personal-skills-disappeared">
  Personal skills disappeared
</h3>

Se le cartelle delle skill che avete creato in `~/.claude/skills/` sono scomparse, cercate in `~/.claude/skills/.trash/`. Quando Claude Code [sincronizza le skills da claude.ai](#how-synced-skills-behave), le scarica nella sottocartella separata `synced` e non sposta o elimina le cartelle che create.

Prima della v2.1.280, un file denominato `manifest.json` in `~/.claude/skills/` causava a Claude Code di spostare le cartelle delle skill elencate in quel file in una cartella con timestamp sotto `~/.claude/skills/.trash/`, e quelle skills smettevano di caricarsi.

Per ripristinare una skill, spostate la sua cartella dalla cartella con timestamp di nuovo in `~/.claude/skills/`. Fatelo prima che la [pulizia della conservazione](/docs/it/claude-directory#cleaned-up-automatically) elimini le voci del cestino, per impostazione predefinita 30 giorni dopo che sono state spostate nel cestino.

<h2 id="related-resources">
  Risorse correlate
</h2>

* **[Debug della tua configurazione](/docs/it/debug-your-config)**: diagnostica il motivo per cui una skill non appare o non si attiva
* **[Valutazione della qualità dell'output della skill](https://agentskills.io/skill-creation/evaluating-skills)**: il formato del file eval e il flusso di lavoro di iterazione su agentskills.io
* **[Best practice di authoring delle skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices)**: guida di scrittura che si applica ai prodotti Claude
* **[Subagents](/docs/it/sub-agents)**: delega attività ad agenti specializzati
* **[Plugins](/docs/it/plugins/overview)**: pacchetto e distribuisci skills con altre estensioni
* **[Hooks](/docs/it/hooks)**: automatizza i flussi di lavoro intorno agli eventi degli strumenti
* **[Memory](/docs/it/memory)**: gestisci i file CLAUDE.md per il contesto persistente
* **[Commands](/docs/it/commands)**: riferimento per i comandi integrati e le skills raggruppate
* **[Permissions](/docs/it/permissions)**: controlla l'accesso agli strumenti e alle skills
* **[Claude Tag skills](https://claude.com/docs/claude-tag/admins/skills-repo)**: le skills del progetto sottoposte a commit in un repository si caricano anche quando quel repository viene utilizzato in un canale Claude Tag
