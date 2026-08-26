> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Estendi Claude con skills

> Crea, gestisci e condividi skills per estendere le capacità di Claude in Claude Code. Include comandi personalizzati e skills raggruppate.

Le skills estendono ciò che Claude può fare. Crea un file `SKILL.md` con istruzioni, e Claude lo aggiunge al suo toolkit. Claude utilizza le skills quando rilevante, oppure puoi invocare una direttamente con `/skill-name`.

Crea una skill quando continui a incollare le stesse istruzioni, checklist, o procedura multi-step nella chat, oppure quando una sezione di CLAUDE.md è cresciuta in una procedura piuttosto che in un fatto. A differenza del contenuto di CLAUDE.md, il corpo di una skill si carica solo quando viene utilizzato, quindi il materiale di riferimento lungo costa quasi nulla finché non ne hai bisogno.

<Note>
  Per i comandi integrati come `/help` e `/compact`, e le skills raggruppate come `/debug` e `/code-review`, vedi il [riferimento dei comandi](/docs/it/commands).

  **I comandi personalizzati sono stati uniti alle skills.** Un file in `.claude/commands/deploy.md` e una skill in `.claude/skills/deploy/SKILL.md` creano entrambi `/deploy` e funzionano allo stesso modo. I tuoi file `.claude/commands/` esistenti continuano a funzionare. Le skills aggiungono funzionalità opzionali: una directory per i file di supporto, frontmatter per [controllare se sei tu o Claude a invocarle](#control-who-invokes-a-skill), e la capacità per Claude di caricarle automaticamente quando rilevante.
</Note>

Le skills di Claude Code seguono lo standard aperto [Agent Skills](https://agentskills.io), che funziona su più strumenti AI. Claude Code estende lo standard con funzionalità aggiuntive come il [controllo dell'invocazione](#control-who-invokes-a-skill), l'[esecuzione in subagent](#run-skills-in-a-subagent), e l'[iniezione di contesto dinamico](#inject-dynamic-context).

<h2 id="bundled-skills">
  Skills raggruppate
</h2>

Claude Code include un set di skills raggruppate che sono disponibili in ogni sessione a meno che non siano disabilitate con l'impostazione [`disableBundledSkills`](/docs/it/settings#available-settings), incluse `/doctor`, `/code-review`, `/batch`, `/debug`, `/loop`, e `/claude-api`. A differenza della maggior parte dei comandi integrati, che eseguono logica fissa direttamente, le skills raggruppate sono basate su prompt: danno a Claude istruzioni dettagliate e gli permettono di orchestrare il lavoro utilizzando i suoi strumenti. Le invochi allo stesso modo di qualsiasi altra skill, digitando `/` seguito dal nome della skill.

Il setup checkup [`/doctor`](/docs/it/commands#all-commands) è l'unica eccezione a `disableBundledSkills` in Claude Code v2.1.205 e successivo: rimane digitabile quando l'impostazione è attiva. Per nasconderlo, imposta la variabile d'ambiente `DISABLE_DOCTOR_COMMAND` o una voce [`skillOverrides`](#override-skill-visibility-from-settings) di `"doctor": "off"`. Prima della v2.1.205, `/doctor` era un comando integrato piuttosto che una skill raggruppata.

Le skills raggruppate sono elencate insieme ai comandi integrati nel [riferimento dei comandi](/docs/it/commands), contrassegnate come **Skill** nella colonna Scopo.

<h3 id="run-and-verify-your-app">
  Esegui e verifica la tua app
</h3>

Tre skills raggruppate lavorano insieme per avviare la tua app e confermare le modifiche rispetto all'app in esecuzione invece di limitarsi ai test:

| Skill                  | Scopo                                                                                                                                       |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| `/run`                 | Avvia e guida la tua app per vedere una modifica in funzione                                                                                |
| `/verify`              | Compila ed esegui la tua app per confermare che una modifica del codice fa quello che dovrebbe, senza ricorrere a test o controlli dei tipi |
| `/run-skill-generator` | Insegna a `/run` e `/verify` come compilare e avviare il tuo progetto                                                                       |

Tutte e tre le skills richiedono Claude Code v2.1.145 o successivo.

`/run` e `/verify` funzionano senza configurazione. Deducono l'avvio dal tipo di progetto (CLI, server, TUI, guidato dal browser) e da ciò che si trova nel tuo README, `package.json`, o `Makefile`. Questa deduzione diventa inaffidabile per i progetti che richiedono qualcosa di più di un avvio standard: un database, un file env, una sessione grafica, una compilazione multi-step.

`/run-skill-generator` registra invece la ricetta. Fa funzionare la tua app da un ambiente pulito, cattura ciò che ha funzionato (i comandi di installazione, le variabili env, lo script di avvio), e lo commit come skill per progetto in `.claude/skills/run-<name>/`. Dopo di che, `/run`, `/verify`, e qualsiasi altro agente nel repository seguono la ricetta registrata invece di riscoprirla. Esegui `/run-skill-generator` una volta per progetto, e di nuovo se il processo di compilazione o avvio cambia.

<h2 id="getting-started">
  Iniziare
</h2>

<h3 id="create-your-first-skill">
  Crea la tua prima skill
</h3>

Questo esempio crea una skill che riassume i cambiamenti non committati nel tuo repository git e segnala qualsiasi cosa rischiosa. Estrae il diff live nel prompt prima che Claude lo legga, quindi la risposta è basata sul tuo albero di lavoro effettivo piuttosto che su quello che Claude può indovinare dai file aperti. Claude carica la skill automaticamente quando chiedi dei tuoi cambiamenti, oppure puoi invocarla direttamente con `/summarize-changes`.

<Steps>
  <Step title="Crea la directory della skill">
    Crea una directory per la skill nella tua cartella di skills personali. Le skills personali sono disponibili su tutti i tuoi progetti.

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

    La riga `` !`git diff HEAD` `` utilizza [iniezione di contesto dinamico](#inject-dynamic-context): Claude Code esegue il comando e sostituisce la riga con il suo output prima che Claude veda il contenuto della skill, quindi le istruzioni arrivano con il diff attuale già inline.
  </Step>

  <Step title="Testa la skill">
    Apri un progetto git, fai una piccola modifica a qualsiasi file, e avvia Claude Code eseguendo `claude`. Puoi testare la skill in due modi.

    **Lascia che Claude la invochi automaticamente** chiedendo qualcosa che corrisponda alla descrizione:

    ```text theme={null}
    What did I change?
    ```

    **O invocarla direttamente** con il nome della skill:

    ```text theme={null}
    /summarize-changes
    ```

    In entrambi i casi, Claude dovrebbe rispondere con un breve riassunto della tua modifica e un elenco di rischi.
  </Step>
</Steps>

<h3 id="where-skills-live">
  Dove vivono le skills
</h3>

Dove archivi una skill determina chi può usarla:

| Posizione  | Percorso                                                 | Si applica a                              |
| :--------- | :------------------------------------------------------- | :---------------------------------------- |
| Enterprise | Vedi [impostazioni gestite](/docs/it/settings#settings-files) | Tutti gli utenti della tua organizzazione |
| Personale  | `~/.claude/skills/<skill-name>/SKILL.md`                 | Tutti i tuoi progetti                     |
| Progetto   | `.claude/skills/<skill-name>/SKILL.md`                   | Solo questo progetto                      |
| Plugin     | `<plugin>/skills/<skill-name>/SKILL.md`                  | Dove il plugin è abilitato                |

Quando le skills condividono lo stesso nome tra i livelli, enterprise ha la priorità su personale, e personale ha la priorità su progetto. Una skill a uno qualsiasi di questi livelli sostituisce anche una skill raggruppata con lo stesso nome. Ad esempio, una skill `code-review` nella `.claude/skills/` del tuo progetto sostituisce la `/code-review` raggruppata. Le skills dei plugin utilizzano uno spazio dei nomi `plugin-name:skill-name`, quindi non possono entrare in conflitto con altri livelli. Se hai file in `.claude/commands/`, funzionano allo stesso modo, ma se una skill e un comando condividono lo stesso nome, la skill ha la precedenza.

Le skills vengono caricate anche da directory `.claude/skills/` annidate al di sotto della tua directory di lavoro. Quando Claude legge o modifica un file in una sottodirectory, le skills dalla `.claude/skills/` di quella sottodirectory diventano disponibili. Questo consente a un pacchetto monorepo di fornire le proprie skills che si applicano quando si lavora su quel pacchetto, anche se la sessione è iniziata alla radice del repository.

Se una skill annidata condivide un nome con un'altra skill, entrambe rimangono disponibili. Ad esempio, con una skill `deploy` alla radice del progetto e un'altra in `apps/web/.claude/skills/`:

* Quella annidata appare con un nome qualificato dalla directory, `apps/web:deploy`.
* La sua descrizione dice quale directory si applica.
* Claude sceglie la variante che corrisponde ai file su cui sta lavorando.

Digitando `/deploy` esegui la skill della radice del progetto. Digita il nome qualificato `/apps/web:deploy` per eseguire esplicitamente la variante annidata.

Quando tu o Claude invocate il nome non qualificato, la skill della radice del progetto viene caricata, e Claude Code aggiunge un elenco delle varianti qualificate dalla directory al suo contenuto con un'istruzione per invocare anche qualsiasi variante la cui directory contiene i file su cui Claude sta lavorando. Una skill annidata quindi si applica comunque al lavoro nella sua directory quando viene invocato solo il nome non qualificato. Richiede Claude Code v2.1.203 o successivo.

Un'entry `<skill-name>` nelle posizioni enterprise, personale o progetto può essere un symlink a una directory altrove sul disco. Claude Code segue il symlink e legge `SKILL.md` dalla directory di destinazione, e se la stessa destinazione è raggiungibile da più di una posizione, Claude Code carica la skill una sola volta. Le skills dei plugin gestiscono i symlink diversamente; vedi [Condividi file all'interno di un marketplace con symlink](/docs/it/plugins-reference#share-files-within-a-marketplace-with-symlinks).

<Note>
  Aggiungi un `.claude-plugin/plugin.json` a una cartella di skill e viene caricato come un [plugin](/docs/it/plugins-reference#skills-directory-plugins) denominato `<name>@skills-dir`, in modo che possa raggruppare agenti, hooks e server MCP. In una `.claude/skills/` di un progetto, questo richiede di accettare prima la finestra di dialogo di fiducia dell'area di lavoro.
</Note>

<h4 id="live-change-detection">
  Rilevamento dei cambiamenti in tempo reale
</h4>

Claude Code osserva le directory delle skills per i cambiamenti dei file. Aggiungere, modificare o rimuovere una skill in `~/.claude/skills/`, nel progetto `.claude/skills/`, o in una `.claude/skills/` all'interno di una directory `--add-dir` ha effetto nella sessione attuale senza riavviare. Creare una directory di skills di primo livello che non esisteva quando la sessione è iniziata richiede il riavvio di Claude Code in modo che la nuova directory possa essere osservata.

<Note>
  Il rilevamento dei cambiamenti in tempo reale copre solo il testo di `SKILL.md`. Per una cartella di skill che è anche un [plugin](/docs/it/plugins-reference#skills-directory-plugins), i cambiamenti a `hooks/`, `.mcp.json`, `agents/` e `output-styles/` richiedono `/reload-plugins` per avere effetto.
</Note>

<h4 id="automatic-discovery-from-parent-and-nested-directories">
  Scoperta automatica da directory padre e annidate
</h4>

Le skills del progetto vengono caricate da `.claude/skills/` nella tua directory di avvio e in ogni directory padre fino alla radice del repository, quindi avviare Claude in una sottodirectory raccoglie comunque le skills definite alla radice. Quando lavori con file in sottodirectory al di sotto della tua directory di avvio, Claude Code scopre anche le skills da directory `.claude/skills/` annidate su richiesta. Ad esempio, se stai modificando un file in `packages/frontend/`, Claude Code cerca anche le skills in `packages/frontend/.claude/skills/`. Questo supporta configurazioni monorepo dove i pacchetti hanno le loro proprie skills.

Ogni skill è una directory con `SKILL.md` come punto di ingresso:

```text theme={null}
my-skill/
├── SKILL.md           # Istruzioni principali (obbligatorio)
├── template.md        # Template per Claude da compilare
├── examples/
│   └── sample.md      # Output di esempio che mostra il formato previsto
└── scripts/
    └── validate.sh    # Script che Claude può eseguire
```

`SKILL.md` contiene le istruzioni principali ed è obbligatorio. Gli altri file sono opzionali e ti permettono di costruire skills più potenti: template per Claude da compilare, output di esempio che mostrano il formato previsto, script che Claude può eseguire, o documentazione di riferimento dettagliata. Fai riferimento a questi file da `SKILL.md` in modo che Claude sappia cosa contengono e quando caricarli. Vedi [Aggiungi file di supporto](#add-supporting-files) per più dettagli.

<Note>
  I file in `.claude/commands/` continuano a funzionare e supportano lo stesso [frontmatter](#frontmatter-reference). Le skills sono consigliate poiché supportano funzionalità aggiuntive come i file di supporto.
</Note>

<h4 id="skills-from-additional-directories">
  Skills da directory aggiuntive
</h4>

Il flag `--add-dir` e il comando `/add-dir` [concedono l'accesso ai file](/docs/it/permissions#additional-directories-grant-file-access-not-configuration) piuttosto che la scoperta della configurazione, ma le skills sono un'eccezione: `.claude/skills/` all'interno di una directory aggiunta viene caricato automaticamente. Questa eccezione si applica solo a `--add-dir` e `/add-dir`. L'impostazione `permissions.additionalDirectories` in `settings.json` concede solo l'accesso ai file e non carica le skills. Vedi [Rilevamento dei cambiamenti in tempo reale](#live-change-detection) per come gli edits vengono rilevati durante una sessione.

Altre configurazioni `.claude/` come comandi e stili di output non vengono caricate da directory aggiuntive. Vedi la [tabella delle eccezioni](/docs/it/permissions#additional-directories-grant-file-access-not-configuration) per l'elenco completo di ciò che viene e non viene caricato, e i modi consigliati per condividere la configurazione tra i progetti.

<Note>
  I file CLAUDE.md da directory `--add-dir` non vengono caricati per impostazione predefinita. Per caricarli, imposta `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1`. Vedi [Carica da directory aggiuntive](/docs/it/memory#load-from-additional-directories).
</Note>

<h2 id="configure-skills">
  Configura le skills
</h2>

Le skills vengono configurate tramite frontmatter YAML in cima a `SKILL.md` e il contenuto markdown che segue.

<h3 id="types-of-skill-content">
  Tipi di contenuto della skill
</h3>

I file delle skills possono contenere qualsiasi istruzione, ma pensare a come vuoi invocarle aiuta a guidare cosa includere:

**Contenuto di riferimento** aggiunge conoscenza che Claude applica al tuo lavoro attuale. Convenzioni, pattern, guide di stile, conoscenza del dominio. Questo contenuto viene eseguito inline in modo che Claude possa usarlo insieme al contesto della tua conversazione.

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

**Contenuto di attività** dà a Claude istruzioni passo-passo per un'azione specifica, come deployment, commit o generazione di codice. Spesso sono azioni che vuoi invocare direttamente con `/skill-name` piuttosto che lasciare che Claude decida quando eseguirle. Aggiungi `disable-model-invocation: true` per impedire a Claude di attivarla automaticamente.

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

Il tuo `SKILL.md` può contenere qualsiasi cosa, ma pensare a come vuoi che la skill venga invocata (da te, da Claude, o da entrambi) e dove vuoi che venga eseguita (inline o in un subagent) aiuta a guidare cosa includere. Per skills complesse, puoi anche [aggiungere file di supporto](#add-supporting-files) per mantenere la skill principale focalizzata.

Mantieni il corpo stesso conciso. Una volta che una skill si carica, il suo contenuto [rimane nel contesto tra i turni](#skill-content-lifecycle), quindi ogni riga è un costo di token ricorrente. Dichiara cosa fare piuttosto che narrare come o perché, e applica lo stesso test di concisione che faresti per il [contenuto di CLAUDE.md](/docs/it/best-practices#write-an-effective-claude-md).

<h3 id="frontmatter-reference">
  Riferimento del frontmatter
</h3>

Oltre al contenuto markdown, puoi configurare il comportamento della skill utilizzando campi frontmatter YAML tra i marcatori `---` in cima al tuo file `SKILL.md`:

```yaml theme={null}
---
name: my-skill
description: What this skill does
disable-model-invocation: true
allowed-tools: Read Grep
---

Your skill instructions here...
```

Tutti i campi sono opzionali. Solo `description` è consigliato in modo che Claude sappia quando usare la skill.

| Campo                      | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :------------------------- | :----------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                     | No           | Nome visualizzato mostrato negli elenchi delle skills. Predefinito al nome della directory. Vedi [Come una skill ottiene il suo nome di comando](#how-a-skill-gets-its-command-name) per come questo differisce dal nome che digiti per invocare la skill.                                                                                                                                                                                                                                                  |
| `description`              | Consigliato  | Cosa fa la skill e quando usarla. Claude utilizza questo per decidere quando applicare la skill. Se omesso, utilizza il primo paragrafo del contenuto markdown. Metti in primo piano il caso d'uso chiave: il testo combinato di `description` e `when_to_use` viene troncato a 1.536 caratteri nell'elenco delle skills per ridurre l'utilizzo del contesto.                                                                                                                                               |
| `when_to_use`              | No           | Contesto aggiuntivo per quando Claude dovrebbe invocare la skill, come frasi trigger o richieste di esempio. Aggiunto a `description` nell'elenco delle skills e conta verso il limite di 1.536 caratteri.                                                                                                                                                                                                                                                                                                  |
| `argument-hint`            | No           | Suggerimento mostrato durante l'autocompletamento per indicare gli argomenti previsti. Esempio: `[issue-number]` o `[filename] [format]`.                                                                                                                                                                                                                                                                                                                                                                   |
| `arguments`                | No           | Argomenti posizionali denominati per la [sostituzione di stringhe `$name`](#available-string-substitutions) nel contenuto della skill. Accetta una stringa separata da spazi o un elenco YAML. I nomi si mappano alle posizioni degli argomenti in ordine.                                                                                                                                                                                                                                                  |
| `disable-model-invocation` | No           | Imposta su `true` per impedire a Claude di caricare automaticamente questa skill. Usa per i flussi di lavoro che vuoi attivare manualmente con `/name`. Impedisce anche alla skill di essere [precaricata nei subagents](/docs/it/sub-agents#preload-skills-into-subagents). A partire da v2.1.196, impedisce anche alla skill di essere eseguita quando un [task programmato](/docs/it/scheduled-tasks) si attiva con la skill come suo prompt. Predefinito: `false`.                                                |
| `user-invocable`           | No           | Imposta su `false` per nascondere dal menu `/`. Usa per la conoscenza di background che gli utenti non dovrebbero invocare direttamente. Predefinito: `true`.                                                                                                                                                                                                                                                                                                                                               |
| `allowed-tools`            | No           | Strumenti che Claude può usare senza chiedere il permesso quando questa skill è attiva. Accetta una stringa separata da spazi o virgole, o un elenco YAML.                                                                                                                                                                                                                                                                                                                                                  |
| `disallowed-tools`         | No           | Strumenti rimossi dal pool disponibile di Claude mentre questa skill è attiva. Usa per skills autonome che non dovrebbero mai chiamare certi strumenti, come `AskUserQuestion` per un loop di background. Accetta una stringa separata da spazi o virgole, o un elenco YAML. La restrizione si cancella quando invii il tuo prossimo messaggio.                                                                                                                                                             |
| `model`                    | No           | Modello da usare quando questa skill è attiva. L'override si applica per il resto del turno attuale e non viene salvato nelle impostazioni; il modello della sessione riprende al tuo prossimo prompt. Accetta gli stessi valori di [`/model`](/docs/it/model-config), o `inherit` per mantenere il modello attivo. Un valore escluso dall'allowlist [`availableModels`](/docs/it/model-config#restrict-model-selection) della tua organizzazione non viene utilizzato e la sessione mantiene il suo modello attuale. |
| `effort`                   | No           | [Livello di sforzo](/docs/it/model-config#adjust-effort-level) quando questa skill è attiva. Sostituisce il livello di sforzo della sessione. Predefinito: eredita dalla sessione. Opzioni: `low`, `medium`, `high`, `xhigh`, `max`; i livelli disponibili dipendono dal modello.                                                                                                                                                                                                                                |
| `context`                  | No           | Imposta su `fork` per eseguire in un contesto subagent con fork.                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `agent`                    | No           | Quale tipo di subagent usare quando `context: fork` è impostato.                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `hooks`                    | No           | hooks limitati al ciclo di vita di questa skill. Vedi [hooks in skills e agents](/docs/it/hooks#hooks-in-skills-and-agents) per il formato di configurazione.                                                                                                                                                                                                                                                                                                                                                    |
| `paths`                    | No           | Pattern glob che limitano quando questa skill viene attivata. Accetta una stringa separata da virgole o un elenco YAML. Quando impostato, Claude carica la skill automaticamente solo quando lavora con file che corrispondono ai pattern. Utilizza lo stesso formato delle [regole specifiche del percorso](/docs/it/memory#path-specific-rules).                                                                                                                                                               |
| `shell`                    | No           | Shell da usare per i blocchi `` !`command` `` e ` ```! ` in questa skill. Accetta `bash` (predefinito) o `powershell`. L'impostazione di `powershell` esegue i comandi shell inline tramite PowerShell su Windows. Richiede `CLAUDE_CODE_USE_POWERSHELL_TOOL=1`.                                                                                                                                                                                                                                            |

<h4 id="how-a-skill-gets-its-command-name">
  Come una skill ottiene il suo nome di comando
</h4>

Il comando che digiti per invocare una skill proviene da dove risiede il file della skill. Il campo frontmatter `name` imposta l'etichetta di visualizzazione mostrata negli elenchi delle skills e, ad eccezione di un `SKILL.md` nella radice del plugin, non cambia quello che digiti dopo `/`.

La tabella seguente mostra da dove proviene il nome del comando per ogni layout:

| Posizione della skill                                                                                    | Fonte del nome del comando                                                                                  | Esempio                                                                                                                                               |
| :------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------- |
| Directory della skill sotto `~/.claude/skills/` o `.claude/skills/`                                      | Nome della directory                                                                                        | `.claude/skills/deploy-staging/SKILL.md` → `/deploy-staging`                                                                                          |
| [Directory annidata](#where-skills-live) `.claude/skills/`, quando il nome si scontra con un'altra skill | Percorso della sottodirectory relativo alla directory di lavoro, quindi il nome della directory della skill | `apps/web/.claude/skills/deploy/SKILL.md` → `/apps/web:deploy`                                                                                        |
| File sotto `.claude/commands/`                                                                           | Nome del file senza estensione                                                                              | `.claude/commands/deploy.md` → `/deploy`                                                                                                              |
| Sottodirectory `skills/` del plugin                                                                      | Nome della directory, con namespace dal plugin                                                              | `my-plugin/skills/review/SKILL.md` → `/my-plugin:review`                                                                                              |
| Plugin root `SKILL.md`                                                                                   | Frontmatter `name`, con il nome della directory del plugin come fallback                                    | `my-plugin/SKILL.md` con `name: review` → `/my-plugin:review`. Vedi [Regole di comportamento del percorso](/docs/it/plugins-reference#path-behavior-rules) |

Il caso della radice del plugin è l'unico posto dove `name` imposta il nome del comando, perché non c'è una directory della skill da cui prenderlo. Se `name` non è impostato nel frontmatter, il nome della directory del plugin viene utilizzato invece.

<h4 id="available-string-substitutions">
  Sostituzioni di stringhe disponibili
</h4>

Le skills supportano la sostituzione di stringhe per valori dinamici nel contenuto della skill:

| Variabile               | Descrizione                                                                                                                                                                                                                                                                                                                                   |
| :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$ARGUMENTS`            | Tutti gli argomenti passati quando si invoca la skill. Se `$ARGUMENTS` non è presente nel contenuto, gli argomenti vengono aggiunti come `ARGUMENTS: <value>`.                                                                                                                                                                                |
| `$ARGUMENTS[N]`         | Accedi a un argomento specifico per indice a base 0, come `$ARGUMENTS[0]` per il primo argomento.                                                                                                                                                                                                                                             |
| `$N`                    | Abbreviazione per `$ARGUMENTS[N]`, come `$0` per il primo argomento o `$1` per il secondo.                                                                                                                                                                                                                                                    |
| `$name`                 | Argomento denominato dichiarato nell'elenco frontmatter [`arguments`](#frontmatter-reference). I nomi si mappano alle posizioni in ordine, quindi con `arguments: [issue, branch]` il placeholder `$issue` si espande al primo argomento e `$branch` al secondo.                                                                              |
| `${CLAUDE_SESSION_ID}`  | L'ID della sessione attuale. Utile per il logging, la creazione di file specifici della sessione, o la correlazione dell'output della skill con le sessioni.                                                                                                                                                                                  |
| `${CLAUDE_EFFORT}`      | Il livello di sforzo attuale: `low`, `medium`, `high`, `xhigh`, o `max`. Ultracode non è un livello distinto e viene segnalato come `xhigh`. Usa questo per adattare le istruzioni della skill all'impostazione di sforzo attiva.                                                                                                             |
| `${CLAUDE_SKILL_DIR}`   | La directory contenente il file `SKILL.md` della skill. Per le skills dei plugin, questa è la sottodirectory della skill all'interno del plugin, non la radice del plugin. Usa questo nei comandi di iniezione bash per fare riferimento a script o file raggruppati con la skill, indipendentemente dalla directory di lavoro attuale.       |
| `${CLAUDE_PROJECT_DIR}` | La directory radice del progetto. Questo è lo stesso percorso che [hooks](/docs/it/hooks#reference-scripts-by-path) e i server MCP ricevono come `CLAUDE_PROJECT_DIR`. Usa questo per fare riferimento a script o file locali del progetto, come `${CLAUDE_PROJECT_DIR}/.claude/hooks/helper.sh`, indipendentemente da dove la skill è installata. |

La sostituzione `${CLAUDE_PROJECT_DIR}` richiede Claude Code v2.1.196 o successivo. Si applica sia al corpo della skill che al frontmatter [`allowed-tools`](#frontmatter-reference), quindi una regola di permesso come `Bash(${CLAUDE_PROJECT_DIR}/scripts/lint.sh *)` si risolve nello stesso percorso che il corpo della skill utilizza.

Gli argomenti indicizzati utilizzano le virgolette in stile shell, quindi racchiudi i valori multi-parola tra virgolette per passarli come un singolo argomento. Ad esempio, `/my-skill "hello world" second` fa sì che `$0` si espanda a `hello world` e `$1` a `second`. Il placeholder `$ARGUMENTS` si espande sempre alla stringa di argomenti completa come digitata.

Per includere un `$` letterale prima di una cifra, `ARGUMENTS`, o un nome di argomento dichiarato, come `$1.00` in prosa, sfuggilo con una barra rovesciata: `\$1.00`. Una barra rovesciata prima di qualsiasi altro `$` viene lasciata invariata. Solo una singola barra rovesciata direttamente prima del token lo sfugge. Una barra rovesciata doppia come `\\$1` lascia entrambe le barre rovesciate in posizione, e `$1` si espande comunque al valore dell'argomento.

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
  Aggiungi file di supporto
</h3>

Le skills possono includere più file nella loro directory. Questo mantiene `SKILL.md` focalizzato sull'essenziale mentre permette a Claude di accedere a materiale di riferimento dettagliato solo quando necessario. Grandi documenti di riferimento, specifiche API, o collezioni di esempi non hanno bisogno di caricarsi nel contesto ogni volta che la skill viene eseguita.

```text theme={null}
my-skill/
├── SKILL.md (obbligatorio - panoramica e navigazione)
├── reference.md (documentazione API dettagliata - caricata quando necessario)
├── examples.md (esempi di utilizzo - caricati quando necessario)
└── scripts/
    └── helper.py (script di utilità - eseguito, non caricato)
```

Fai riferimento ai file di supporto da `SKILL.md` in modo che Claude sappia cosa contiene ogni file e quando caricarlo:

```markdown theme={null}
## Additional resources

- For complete API details, see [reference.md](reference.md)
- For usage examples, see [examples.md](examples.md)
```

<Tip>Mantieni `SKILL.md` sotto 500 righe. Sposta il materiale di riferimento dettagliato in file separati.</Tip>

<h3 id="control-who-invokes-a-skill">
  Controlla chi invoca una skill
</h3>

Per impostazione predefinita, sia tu che Claude potete invocare qualsiasi skill. Puoi digitare `/skill-name` per invocarla direttamente, e Claude può caricarla automaticamente quando rilevante per la tua conversazione. Due campi frontmatter ti permettono di limitare questo:

* **`disable-model-invocation: true`**: Solo tu puoi invocare la skill. Usa questo per i flussi di lavoro con effetti collaterali o che vuoi controllare il timing, come `/commit`, `/deploy`, o `/send-slack-message`. Non vuoi che Claude decida di fare il deploy perché il tuo codice sembra pronto.

* **`user-invocable: false`**: Solo Claude può invocare la skill. Usa questo per la conoscenza di background che non è azionabile come comando. Una skill `legacy-system-context` spiega come funziona un vecchio sistema. Claude dovrebbe saperlo quando rilevante, ma `/legacy-system-context` non è un'azione significativa per gli utenti da intraprendere.

Questo esempio crea una skill di deploy che solo tu puoi attivare. Se imposti `disable-model-invocation: true`, Claude non può eseguire la skill automaticamente:

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

Ecco come i due campi influenzano l'invocazione e il caricamento del contesto:

| Frontmatter                      | Puoi invocare | Claude può invocare | Quando caricato nel contesto                                                      |
| :------------------------------- | :------------ | :------------------ | :-------------------------------------------------------------------------------- |
| (predefinito)                    | Sì            | Sì                  | La descrizione è sempre nel contesto, la skill completa si carica quando invocata |
| `disable-model-invocation: true` | Sì            | No                  | La descrizione non è nel contesto, la skill completa si carica quando la invochi  |
| `user-invocable: false`          | No            | Sì                  | La descrizione è sempre nel contesto, la skill completa si carica quando invocata |

<Note>
  In una sessione regolare, le descrizioni delle skills vengono caricate nel contesto in modo che Claude sappia cosa è disponibile, ma il contenuto completo della skill si carica solo quando invocato. [I subagents con skills precaricate](/docs/it/sub-agents#preload-skills-into-subagents) funzionano diversamente: il contenuto completo della skill viene iniettato all'avvio.
</Note>

<h3 id="skill-content-lifecycle">
  Ciclo di vita del contenuto della skill
</h3>

Quando tu o Claude invocate una skill, il contenuto `SKILL.md` renderizzato entra nella conversazione come un singolo messaggio e rimane lì per il resto della sessione. Claude Code non rilegge il file della skill nei turni successivi, quindi scrivi la guida che dovrebbe applicarsi durante un'attività come istruzioni permanenti piuttosto che come passaggi una tantum.

Quando Claude reinvoca una skill il cui contenuto renderizzato è identico alla copia già nel contesto, Claude Code aggiunge una breve nota che la skill è già caricata piuttosto che una seconda copia del contenuto. Quando il contenuto renderizzato differisce, perché gli argomenti sono cambiati o un comando [contesto dinamico](#inject-dynamic-context) ha prodotto un nuovo output, Claude Code aggiunge il contenuto completo di nuovo. Prima di v2.1.202, ogni reinvocazione aggiungeva un'altra copia completa delle istruzioni della skill.

[Auto-compaction](/docs/it/how-claude-code-works#when-context-fills-up) porta avanti le skills invocate all'interno di un budget di token. Quando la conversazione viene riassunta per liberare contesto, Claude Code riallega l'invocazione più recente di ogni skill dopo il riassunto, mantenendo i primi 5.000 token di ciascuna. Le skills riallegate condividono un budget combinato di 25.000 token. Claude Code riempie questo budget a partire dalla skill invocata più di recente, quindi le skills più vecchie possono essere completamente eliminate dopo la compaction se ne hai invocate molte in una sessione.

Se una skill sembra smettere di influenzare il comportamento dopo la prima risposta, il contenuto è solitamente ancora presente e il modello sta scegliendo altri strumenti o approcci. Rafforza la `description` della skill e le istruzioni in modo che il modello continui a preferirla, o usa [hooks](/docs/it/hooks) per applicare il comportamento in modo deterministico. Se la skill è grande o ne hai invocate molte altre dopo, reinvocala dopo la compaction per ripristinare il contenuto completo.

<h3 id="pre-approve-tools-for-a-skill">
  Pre-approva gli strumenti per una skill
</h3>

Il campo `allowed-tools` concede il permesso per gli strumenti elencati mentre la skill è attiva, in modo che Claude possa usarli senza chiederti l'approvazione per ogni uso. Non limita quali strumenti sono disponibili: ogni strumento rimane richiamabile, e le tue [impostazioni di permesso](/docs/it/permissions) governano comunque gli strumenti che non sono elencati.

Per le skills controllate in una directory `.claude/skills/` di un progetto, `allowed-tools` ha effetto dopo che accetti la finestra di dialogo di trust dell'area di lavoro per quella cartella, come le regole di permesso in `.claude/settings.json`. Rivedi le skills del progetto prima di fidarti di un repository, poiché una skill può concedere a se stessa un ampio accesso agli strumenti.

Questa skill permette a Claude di eseguire comandi git senza approvazione per uso ogni volta che la invochi:

```yaml theme={null}
---
name: commit
description: Stage and commit the current changes
disable-model-invocation: true
allowed-tools: Bash(git add *) Bash(git commit *) Bash(git status *)
---
```

Per rimuovere gli strumenti dal pool disponibile di Claude mentre una skill è attiva, elencali in `disallowed-tools` nel frontmatter della skill. La restrizione si cancella quando invii il tuo prossimo messaggio. Per bloccare gli strumenti in tutte le skills e i prompt, aggiungi regole di negazione nelle tue [impostazioni di permesso](/docs/it/permissions).

<h3 id="pass-arguments-to-skills">
  Passa argomenti alle skills
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

Se invochi una skill con argomenti ma la skill non include `$ARGUMENTS`, Claude Code aggiunge `ARGUMENTS: <your input>` alla fine del contenuto della skill in modo che Claude veda comunque quello che hai digitato.

Puoi anche impilare diverse skills all'inizio di un messaggio. A partire da v2.1.199, digitando `/code-review /fix-issue 123` carica entrambe le skills e passa il testo finale `123` come `$ARGUMENTS` a ciascuna di esse. Nelle versioni precedenti, solo la prima skill si caricava e riceveva `/fix-issue 123` come testo di argomento letterale.

Claude Code espande la prima skill più fino a cinque altre impilate dopo di essa. L'espansione si ferma al primo token che non è una skill invocabile inline dall'utente, quindi una skill che viene eseguita come [subagent con fork](#run-skills-in-a-subagent) o una i cui argomenti potrebbero essi stessi iniziare con un comando slash, come `/loop`, termina anche lì; quel token e tutto ciò che segue diventano il testo di argomento per ogni skill espansa.

Per accedere agli argomenti individuali per posizione, usa `$ARGUMENTS[N]` o il più breve `$N`:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one framework to another
---

Migrate the $ARGUMENTS[0] component from $ARGUMENTS[1] to $ARGUMENTS[2].
Preserve all existing behavior and tests.
```

Eseguendo `/migrate-component SearchBar React Vue` sostituisce `$ARGUMENTS[0]` con `SearchBar`, `$ARGUMENTS[1]` con `React`, e `$ARGUMENTS[2]` con `Vue`. La stessa skill usando la scorciatoia `$N`:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one framework to another
---

Migrate the $0 component from $1 to $2.
Preserve all existing behavior and tests.
```

<h2 id="advanced-patterns">
  Pattern avanzati
</h2>

<h3 id="inject-dynamic-context">
  Inietta contesto dinamico
</h3>

La sintassi `` !`<command>` `` esegue comandi shell prima che il contenuto della skill venga inviato a Claude. L'output del comando sostituisce il placeholder, in modo che Claude riceva dati effettivi, non il comando stesso.

Questa skill riassume una pull request recuperando dati PR in tempo reale con GitHub CLI. I comandi `` !`gh pr diff` `` e altri vengono eseguiti per primi, e il loro output viene inserito nel prompt:

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

Quando questa skill viene eseguita:

1. Ogni `` !`<command>` `` viene eseguito immediatamente (prima che Claude veda qualsiasi cosa)
2. L'output sostituisce il placeholder nel contenuto della skill
3. Claude riceve il prompt completamente renderizzato con dati PR effettivi

Questo è preprocessing, non qualcosa che Claude esegue. Claude vede solo il risultato finale.

La sostituzione viene eseguita una sola volta sul file originale. L'output del comando viene inserito come testo semplice e non viene nuovamente scansionato per ulteriori placeholder `` !`<command>` ``, quindi un comando non può emettere un placeholder per un passaggio successivo da espandere.

La forma inline viene riconosciuta solo quando `!` appare all'inizio di una riga o immediatamente dopo uno spazio. Se `!` segue un altro carattere, come in `` KEY=!`cmd` ``, il placeholder viene lasciato come testo letterale e il comando non viene eseguito.

Per comandi multi-riga, usa un blocco di codice aperto con ` ```! ` invece della forma inline:

````markdown theme={null}
## Environment
```!
node --version
npm --version
git status --short
```
````

Per disabilitare questo comportamento per skills e comandi personalizzati da fonti utente, progetto, plugin, o [directory aggiuntiva](#skills-from-additional-directories), imposta `"disableSkillShellExecution": true` nelle [impostazioni](/docs/it/settings). Ogni comando viene sostituito con `[shell command execution disabled by policy]` invece di essere eseguito. Le skills raggruppate e gestite non sono interessate. Questa impostazione è più utile nelle [impostazioni gestite](/docs/it/permissions#managed-settings), dove gli utenti non possono sovrascriverla.

<Tip>
  Per richiedere un ragionamento più profondo quando una skill viene eseguita, includi `ultrathink` da qualche parte nel contenuto della skill. Vedi [Usa ultrathink per il ragionamento profondo una tantum](/docs/it/model-config#use-ultrathink-for-one-off-deep-reasoning).
</Tip>

<h3 id="run-skills-in-a-subagent">
  Esegui skills in un subagent
</h3>

Aggiungi `context: fork` al tuo frontmatter quando vuoi che una skill venga eseguita in isolamento. Il contenuto della skill diventa il prompt che guida il subagent. Non avrà accesso alla tua cronologia di conversazione.

<Warning>
  `context: fork` ha senso solo per skills con istruzioni esplicite. Se la tua skill contiene linee guida come "usa queste convenzioni API" senza un'attività, il subagent riceve le linee guida ma nessun prompt azionabile, e ritorna senza output significativo.
</Warning>

Le skills e i [subagents](/docs/it/sub-agents) funzionano insieme in due direzioni:

| Approccio                   | System prompt               | Attività                      | Carica anche                                       |
| :-------------------------- | :-------------------------- | :---------------------------- | :------------------------------------------------- |
| Skill con `context: fork`   | Dal tipo di agent           | Contenuto di SKILL.md         | CLAUDE.md, eccetto quando l'agent è Explore o Plan |
| Subagent con campo `skills` | Corpo markdown del subagent | Messaggio di delega di Claude | Skills precaricate + CLAUDE.md                     |

Con `context: fork`, scrivi l'attività nella tua skill e scegli un tipo di agent per eseguirla. Gli agent integrati Explore e Plan [saltano CLAUDE.md e git status](/docs/it/sub-agents#what-loads-at-startup) per mantenere il loro contesto piccolo, quindi una skill con fork che utilizza `agent: Explore` vede solo il contenuto di SKILL.md e il system prompt dell'agent stesso. Per l'inverso, dove definisci un subagent personalizzato che usa le skills come materiale di riferimento, vedi [Subagents](/docs/it/sub-agents#preload-skills-into-subagents).

<h4 id="example-research-skill-using-explore-agent">
  Esempio: Skill di ricerca usando l'agent Explore
</h4>

Questa skill esegue la ricerca in un agent Explore con fork. Il contenuto della skill diventa l'attività, e l'agent fornisce strumenti di sola lettura ottimizzati per l'esplorazione del codebase:

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
2. Il subagent riceve il contenuto della skill come suo prompt ("Research \$ARGUMENTS thoroughly...")
3. Il campo `agent` determina l'ambiente di esecuzione (modello, strumenti e permessi)
4. I risultati vengono riassunti e restituiti alla tua conversazione principale

Il campo `agent` specifica quale configurazione di subagent usare. Le opzioni includono agent integrati (`Explore`, `Plan`, `general-purpose`) o qualsiasi subagent personalizzato da `.claude/agents/`. Se omesso, utilizza `general-purpose`.

<h3 id="restrict-claude’s-skill-access">
  Limita l'accesso alle skills di Claude
</h3>

Per impostazione predefinita, Claude può invocare qualsiasi skill che non abbia `disable-model-invocation: true` impostato. Le skills che definiscono `allowed-tools` concedono a Claude l'accesso a quegli strumenti senza approvazione per uso quando la skill è attiva. Le tue [impostazioni di permesso](/docs/it/permissions) governano comunque il comportamento di approvazione di base per tutti gli altri strumenti. Alcuni comandi integrati sono anche disponibili tramite lo strumento Skill, inclusi `/init`, `/review`, e `/security-review`. Altri comandi integrati come `/compact` non lo sono.

Tre modi per controllare quali skills Claude può invocare:

**Disabilita tutte le skills** negando lo strumento Skill in `/permissions`:

```text theme={null}
# Add to deny rules:
Skill
```

**Consenti o nega skills specifiche** usando [regole di permesso](/docs/it/permissions):

```text theme={null}
# Allow only specific skills
Skill(commit)
Skill(review-pr *)

# Deny specific skills
Skill(deploy *)
```

Sintassi di permesso: `Skill(name)` per corrispondenza esatta, `Skill(name *)` per corrispondenza di prefisso con qualsiasi argomento.

**Nascondi skills individuali** aggiungendo `disable-model-invocation: true` al loro frontmatter. Questo rimuove la skill dal contesto di Claude completamente.

<Note>
  Il campo `user-invocable` controlla solo la visibilità del menu, non l'accesso allo strumento Skill. Usa `disable-model-invocation: true` per bloccare l'invocazione programmatica.
</Note>

<h3 id="override-skill-visibility-from-settings">
  Sovrascrivi la visibilità della skill dalle impostazioni
</h3>

L'impostazione `skillOverrides` controlla la visibilità della skill dalle tue [impostazioni](/docs/it/settings) invece del frontmatter della skill stessa. Usala per skills il cui SKILL.md non vuoi modificare, come quelle controllate in un repository di progetto condiviso o fornite da un server MCP. Il menu `/skills` lo scrive per te: evidenzia una skill e premi `Space` per ciclo gli stati, quindi `Enter` per salvare in `.claude/settings.local.json`.

Ogni chiave è un nome di skill e ogni valore è uno di quattro stati:

| Valore                  | Elencato a Claude  | Nel menu `/` |
| :---------------------- | :----------------- | :----------- |
| `"on"`                  | Nome e descrizione | Sì           |
| `"name-only"`           | Solo nome          | Sì           |
| `"user-invocable-only"` | Nascosto           | Sì           |
| `"off"`                 | Nascosto           | Nascosto     |

A partire dalla v2.1.199, `"off"` nasconde anche la skill dagli elenchi di comandi pubblicizzati ai client [Remote Control](/docs/it/remote-control) e ai chiamanti [Agent SDK](/docs/it/agent-sdk/slash-commands), non solo dal menu `/` del terminale. Invocare una skill nascosta per il suo nome completo restituisce comunque l'errore `skillOverrides` invece di eseguirla.

Una skill assente da `skillOverrides` è trattata come `"on"`. L'esempio seguente comprime una skill al suo nome e disattiva completamente un'altra:

```json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

Le skills dei plugin non sono interessate da `skillOverrides`. Gestisci quelle tramite `/plugin` invece.

<h2 id="evaluate-and-iterate-on-a-skill">
  Valuta e itera su una skill
</h2>

Vedere una skill attivarsi ti dice che Claude l'ha trovata, non che ha fatto quello che intendevi. Per sapere che una skill funziona, misura due cose separatamente: se Claude la invoca sui prompt che dovrebbe, e se l'output corrisponde a quello che ti aspetti quando lo fa.

Il controllo per entrambi è un confronto di base. Raccogli alcuni prompt realistici, esegui ciascuno in una sessione nuova con la skill disponibile e di nuovo con essa [disabilitata](#override-skill-visibility-from-settings), e confronta i risultati. Una sessione nuova è importante perché il contesto residuo dall'authoring della skill maschererà le lacune nelle istruzioni scritte.

<h3 id="run-evals-with-skill-creator">
  Esegui evals con skill-creator
</h3>

Il [plugin `skill-creator`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/skill-creator) automatizza il loop di confronto all'interno di Claude Code. Installalo dal marketplace ufficiale:

```text theme={null}
/plugin install skill-creator@claude-plugins-official
```

Se Claude Code segnala che il plugin non viene trovato in nessun marketplace, il tuo marketplace è mancante o obsoleto. Esegui `/plugin marketplace update claude-plugins-official` per aggiornarlo, o `/plugin marketplace add anthropics/claude-plugins-official` se non l'hai ancora aggiunto. Quindi riprova l'installazione.

Dopo l'installazione, esegui `/reload-plugins` per rendere disponibili le skills del plugin nella sessione attuale. Quindi chiedi a Claude di valutare una skill esistente, ad esempio `evaluate my summarize-changes skill with skill-creator`. Il plugin ti guida attraverso la scrittura di test case ed esegue il loop:

* **Test cases**: memorizza prompt, file di input e comportamento previsto in `evals/evals.json` all'interno della directory della skill
* **Isolated runs**: genera un [subagent](/docs/it/sub-agents) per test case in modo che ogni esecuzione inizi con un contesto pulito, e registra il conteggio dei token e la durata
* **Grading**: controlla ogni asserzione rispetto all'output e scrive pass o fail con evidenza in `grading.json`
* **Benchmark**: aggrega il tasso di pass, il tempo e i token per con-skill versus senza-skill in `benchmark.json` in modo che tu possa confrontare il miglioramento del tasso di pass rispetto al sovraccarico di token e tempo
* **Version comparison**: esegue un blind A/B tra due versioni della skill in modo che tu possa confermare che un edit è un miglioramento prima di commitarlo
* **Description tuning**: genera prompt should-trigger e should-not-trigger, misura il tasso di hit, e propone edits di descrizione quando la skill si attiva su richieste sbagliate
* **Review viewer**: apre un report HTML dove puoi ispezionare ogni output e registrare feedback qualitativo che l'iterazione successiva legge

Per il formato del file eval e il flusso di lavoro di iterazione completo, vedi [Evaluating skill output quality](https://agentskills.io/skill-creation/evaluating-skills) su agentskills.io. Per il background sul benchmark e sui modi di confronto, vedi l'[annuncio di skill-creator](https://claude.com/blog/improving-skill-creator-test-measure-and-refine-agent-skills).

<h2 id="share-skills">
  Condividi skills
</h2>

Le skills possono essere distribuite a diversi ambiti a seconda del tuo pubblico:

* **Skills di progetto**: Esegui il commit di `.claude/skills/` al controllo di versione
* **Plugin**: Crea una directory `skills/` nel tuo [plugin](/docs/it/plugins)
* **Gestito**: Distribuisci a livello di organizzazione tramite [impostazioni gestite](/docs/it/settings#settings-files)

<h3 id="generate-visual-output">
  Genera output visuale
</h3>

Le skills possono raggruppare ed eseguire script in qualsiasi linguaggio, dando a Claude capacità oltre ciò che è possibile in un singolo prompt. Un pattern potente è generare output visuale: file HTML interattivi che si aprono nel tuo browser per esplorare dati, eseguire il debug, o creare report.

Questo esempio crea un esploratore di codebase: una vista ad albero interattiva dove puoi espandere e comprimere directory, vedere le dimensioni dei file a colpo d'occhio, e identificare i tipi di file per colore.

Crea la directory della Skill:

```bash theme={null}
mkdir -p ~/.claude/skills/codebase-visualizer/scripts
```

Salva questo in `~/.claude/skills/codebase-visualizer/SKILL.md`. La descrizione dice a Claude quando attivare questa Skill, e le istruzioni dicono a Claude di eseguire lo script raggruppato. Il percorso dello script utilizza [`${CLAUDE_SKILL_DIR}`](#available-string-substitutions) in modo che si risolva correttamente indipendentemente dal fatto che la skill sia installata a livello personale, di progetto o di plugin:

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

Salva questo in `~/.claude/skills/codebase-visualizer/scripts/visualize.py`. Questo script scansiona un albero di directory e genera un file HTML autonomo con:

* Una **barra laterale di riepilogo** che mostra il conteggio dei file, il conteggio delle directory, la dimensione totale e il numero di tipi di file
* Un **grafico a barre** che suddivide il codebase per tipo di file (i primi 8 per dimensione)
* Un **albero comprimibile** dove puoi espandere e comprimere directory, con indicatori di tipo di file codificati per colore

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

Per testare, apri Claude Code in qualsiasi progetto e chiedi "Visualizza questo codebase." Claude esegue lo script, genera `codebase-map.html`, e lo apre nel tuo browser.

Questo pattern funziona per qualsiasi output visuale: grafici di dipendenza, report di copertura dei test, documentazione API, o visualizzazioni di schema di database. Lo script raggruppato fa il lavoro pesante mentre Claude gestisce l'orchestrazione.

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

<h3 id="skill-not-triggering">
  Skill non si attiva
</h3>

Se Claude non usa la tua skill quando previsto:

1. Controlla che la descrizione includa parole chiave che gli utenti direbbero naturalmente
2. Verifica che la skill appaia in `What skills are available?`
3. Prova a riformulare la tua richiesta per corrispondere più strettamente alla descrizione
4. Invocarla direttamente con `/skill-name` se la skill è invocabile dall'utente

Se il frontmatter YAML è malformato, Claude Code carica il corpo della skill con metadati vuoti, quindi `/skill-name` funziona comunque ma Claude non ha alcuna `description` per corrispondere. Esegui con `--debug` per vedere l'errore di analisi.

<h3 id="skill-triggers-too-often">
  Skill si attiva troppo spesso
</h3>

Se Claude usa la tua skill quando non vuoi:

1. Rendi la descrizione più specifica
2. Aggiungi `disable-model-invocation: true` se vuoi solo l'invocazione manuale

<h3 id="skill-descriptions-are-cut-short">
  Le descrizioni delle skills vengono tagliate
</h3>

Claude Code carica un elenco di nomi e descrizioni delle skills nel contesto in modo che Claude sappia cosa è disponibile. L'elenco contiene sempre ogni nome di skill, ma se hai molte skills, Claude Code accorcia le descrizioni per adattarsi al budget dei caratteri dell'elenco, il che può rimuovere le parole chiave di cui Claude ha bisogno per corrispondere alla tua richiesta. Il budget si ridimensiona all'1% della finestra di contesto del modello. Quando l'elenco supera il limite, Claude Code elimina le descrizioni iniziando dalle skills che invochi meno frequentemente, in modo che le skills che usi di più mantengano il loro testo completo.

Esegui `/doctor` per una stima del costo di contesto dell'elenco e dei suoi maggiori contributori. Quando l'elenco supera il budget, Claude Code scrive anche un avviso nel log di debug, visibile con [`--debug`](/docs/it/cli-reference#cli-flags).

La riga Skills in `/context` riporta la dimensione dell'elenco dopo l'applicazione del budget, in modo che corrisponda a ciò che il modello riceve. Prima della v2.1.196, la riga contava il testo completo di ogni descrizione e poteva mostrare un valore diverse volte più grande del budget configurato.

Per aumentare il budget, imposta l'impostazione [`skillListingBudgetFraction`](/docs/it/settings#available-settings) (ad esempio `0.02` = 2%) o la variabile di ambiente `SLASH_COMMAND_TOOL_CHAR_BUDGET` su un conteggio di caratteri fisso. Per liberare budget per altre skills, imposta le voci a bassa priorità su `"name-only"` in [`skillOverrides`](#override-skill-visibility-from-settings) in modo che si elenchino senza una descrizione. Puoi anche ridurre il testo di `description` e `when_to_use` alla fonte: metti in primo piano il caso d'uso chiave, poiché il testo combinato di ogni voce è limitato a 1.536 caratteri indipendentemente dal budget. Il limite è configurabile con [`skillListingMaxDescChars`](/docs/it/settings#available-settings).

<h2 id="related-resources">
  Risorse correlate
</h2>

* **[Debug della tua configurazione](/docs/it/debug-your-config)**: diagnostica il motivo per cui una skill non appare o non si attiva
* **[Valutazione della qualità dell'output della skill](https://agentskills.io/skill-creation/evaluating-skills)**: il formato del file eval e il flusso di lavoro di iterazione su agentskills.io
* **[Best practice di authoring delle skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices)**: guida di scrittura che si applica ai prodotti Claude
* **[Subagents](/docs/it/sub-agents)**: delega attività ad agenti specializzati
* **[Plugins](/docs/it/plugins)**: pacchetto e distribuisci skills con altre estensioni
* **[Hooks](/docs/it/hooks)**: automatizza i flussi di lavoro intorno agli eventi degli strumenti
* **[Memory](/docs/it/memory)**: gestisci i file CLAUDE.md per il contesto persistente
* **[Commands](/docs/it/commands)**: riferimento per i comandi integrati e le skills raggruppate
* **[Permissions](/docs/it/permissions)**: controlla l'accesso agli strumenti e alle skills
* **[Claude Tag skills](https://claude.com/docs/claude-tag/admins/skills-repo)**: le skills del progetto sottoposte a commit in un repository si caricano anche quando quel repository viene utilizzato in un canale Claude Tag
