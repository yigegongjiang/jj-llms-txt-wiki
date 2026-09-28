> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Estendi gli agenti con skills

> Controlla quali skills Claude può invocare nelle sessioni dell'Agent SDK, invia comandi per nome e crea skills che le tue sessioni scoprono

Agent Skills estendono Claude con capacità specializzate che Claude richiama quando rilevante. Le Skills sono confezionate come file `SKILL.md` contenenti istruzioni, descrizioni e risorse di supporto opzionali. Questa pagina copre anche i [comandi nelle sessioni dell'Agent SDK](#commands-in-agent-sdk-sessions).

Per informazioni complete su skills, inclusi vantaggi, architettura e linee guida di authoring, consulta la [panoramica di Agent Skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview).

<h2 id="how-skills-work-with-the-agent-sdk">
  Come funzionano le skills con l'Agent SDK
</h2>

Quando si utilizza l'SDK dell'Agent Claude, le skills sono:

* **Definite come artefatti del filesystem**: crei ogni skill come file `SKILL.md` nella sua directory, ad esempio `.claude/skills/<name>/SKILL.md`
* **Caricate dal filesystem**: l'SDK carica le skills dalle posizioni del filesystem governate da `settingSources` (TypeScript) o `setting_sources` (Python)
* **Scoperte automaticamente**: una volta caricate le impostazioni del filesystem, l'SDK scopre i metadati della skill all'avvio dalle directory dell'utente e del progetto, e carica il contenuto completo quando Claude richiama la skill
* **Richiamate dal modello**: Claude sceglie autonomamente quando utilizzarle in base al contesto
* **Richiamate dall'utente**: invii una skill direttamente inviando `/<name>` in un prompt. Vedi [Comandi nelle sessioni dell'Agent SDK](#commands-in-agent-sdk-sessions)
* **Limitate tramite l'opzione `skills`**: le skills scoperte sono abilitate per impostazione predefinita. Passa un elenco di nomi di skills, `"all"`, o `[]` per controllare quali skills Claude può invocare

A differenza dei subagents, che puoi definire nell'[opzione `agents`](/docs/it/agent-sdk/subagents#programmatic-definition-recommended), crei le skills come file su disco. L'SDK non fornisce un'API programmatica per registrarle.

<Note>
  Le skills vengono scoperte attraverso le fonti di impostazione del filesystem. Con le opzioni predefinite di `query()`, l'SDK carica le fonti utente e progetto, quindi le skills in `~/.claude/skills/`, `<cwd>/.claude/skills/`, e `.claude/skills/` in qualsiasi directory padre di `<cwd>` fino alla radice del repository sono disponibili. La fonte del progetto copre anche `<dir>/.claude/skills/` in ogni directory che passi attraverso `additionalDirectories` (TypeScript) o `add_dirs` (Python), perché l'SDK passa quelle directory a Claude Code come [`--add-dir`](/docs/it/skills#skills-from-additional-directories). Se imposti `settingSources` esplicitamente, includi `'project'` per mantenere le skills del progetto e della directory aggiunta e `'user'` per mantenere le tue skills personali, oppure utilizza l'[opzione `plugins`](/docs/it/agent-sdk/plugins) per caricare le skills da un percorso specifico.
</Note>

<h2 id="use-skills-with-the-agent-sdk">
  Utilizza le skills con l'Agent SDK
</h2>

Imposta l'opzione `skills` su `query()` per controllare quali skills Claude può invocare nella sessione. Se omessa, le skills scoperte sono abilitate e lo strumento Skill è disponibile, corrispondendo al comportamento della CLI. Passa `"all"` per consentire a Claude di invocare ogni skill scoperta, un elenco di nomi di skills per consentire solo quelle, o `[]` per consentire a Claude di non invocarne nessuna.

Ad esempio, per consentire a Claude di invocare solo due skills denominate:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(skills=["pdf", "docx"])
  ```

  ```typescript TypeScript theme={null}
  const options = { skills: ["pdf", "docx"] };
  ```
</CodeGroup>

<h3 id="set-up-skills-in-a-session">
  Configura le skills in una sessione
</h3>

Quando imposti `skills`, l'SDK aggiunge automaticamente lo strumento Skill a `allowedTools`. Se passi anche un elenco esplicito di `tools`, includi `"Skill"` in quell'elenco in modo che Claude possa invocare le skills.

Una volta configurato, Claude scopre automaticamente le skills dal filesystem e le richiama quando rilevante per la richiesta dell'utente.

L'esempio seguente abilita ogni skill scoperta in una sessione e pre-approva gli strumenti che le skills comunemente necessitano. L'esempio imposta `cwd` sulla directory di lavoro corrente del processo, quindi eseguilo dall'interno di un progetto che ha una directory `.claude/skills/` nella directory corrente o in qualsiasi directory padre fino alla radice del repository:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import os

  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      options = ClaudeAgentOptions(
          cwd=os.getcwd(),  # .claude/skills/ here or in a parent directory
          setting_sources=["user", "project"],  # Load skills from filesystem
          skills="all",  # Let Claude invoke every discovered skill
          allowed_tools=["Read", "Write", "Bash"],
      )

      async for message in query(
          prompt="Help me process this PDF document", options=options
      ):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Help me process this PDF document",
    options: {
      cwd: process.cwd(), // .claude/skills/ here or in a parent directory
      settingSources: ["user", "project"], // Load skills from filesystem
      skills: "all", // Let Claude invoke every discovered skill
      allowedTools: ["Read", "Write", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h3 id="confirm-skills-loaded">
  Conferma che le skills sono caricate
</h3>

Vicino all'inizio dello stream, l'SDK produce un messaggio di sistema con sottotipo `init`. Controlla il suo array `skills` per confermare che le tue skills siano caricate prima che Claude inizi a lavorare. L'array include le skills invocabili dall'utente che hai definito con un campo frontmatter `description` o `when_to_use`, insieme alle [skills incluse nel bundle con Claude Code](/docs/it/skills#bundled-skills).

L'array elenca solo le skills invocabili dall'utente. Una skill con [`user-invocable: false`](/docs/it/skills#control-who-invokes-a-skill) nel suo frontmatter si carica e rimane disponibile per Claude, ma non appare nell'array. L'array elenca le stesse skills indipendentemente dal fatto che siano nel tuo elenco `skills`.

<h3 id="allow-only-specific-skills">
  Consenti solo skills specifiche
</h3>

Per consentire a Claude di invocare solo skills specifiche, passa i loro nomi nell'elenco `skills`. I nomi corrispondono al campo `name` in `SKILL.md` o al nome della directory della skill. Utilizza `plugin:skill` per le skills fornite da plugin.

L'elenco accetta solo nomi di skills esatti. Se una voce non può funzionare come nome esatto, `query()` rifiuta l'elenco prima che la sessione inizi. Vedi [Errore di nome skill non valido](#invalid-skill-name-error) per le regole dei nomi e l'errore che ogni SDK genera.

Il modello non vede le skills non elencate e lo strumento Skill le rifiuta, mentre i loro file rimangono su disco e rimangono raggiungibili attraverso Read e Bash. Limitare l'elenco non limita l'[invio per nome](#dispatch-commands-by-name).

Per consentire a Claude di invocare ogni skill scoperta, passa `skills: "all"` piuttosto che un wildcard.

<h2 id="commands-in-agent-sdk-sessions">
  Comandi nelle sessioni dell'Agent SDK
</h2>

Questa sezione è la documentazione dei comandi dell'SDK. Un comando è qualsiasi cosa tu esegua inviando `/<name>` in un prompt. Le voci sulla superficie del comando differiscono in ciò che le supporta:

* **Comandi incorporati**: eseguono la logica codificata nel processo Claude Code che l'SDK esegue, ad esempio `/compact`
* **Skills nel bundle**: artefatti prompt inclusi con Claude Code, ad esempio `/code-review`
* **Le tue skills**: artefatti prompt che crei, ognuno una directory che contiene un file `SKILL.md`. Il nome di una skill invocabile dall'utente si unisce automaticamente alla superficie, quindi inviare il tuo `/security-check` e eseguire un incorporato funzionano allo stesso modo
* **File di comando personalizzati**: una forma di artefatto più vecchia con lo stesso comportamento, file Markdown flat in `.claude/commands/` i cui nomi di file diventano nomi di comandi. Le skills sono il loro successore consigliato

Per impostazione predefinita, sia tu che Claude potete invocare qualsiasi skill. Puoi limitare entrambi i percorsi attraverso il [frontmatter](/docs/it/skills#control-who-invokes-a-skill) della skill. Per una definizione dei due termini, vedi le voci del glossario [Comando](/docs/it/glossary#command) e [Skill](/docs/it/glossary#skill). Vedi [Comandi in Claude Code](/docs/it/commands) per ogni incorporato e [Estendi Claude con skills](/docs/it/skills) per la guida completa a entrambe le forme di artefatto.

<h3 id="discover-available-commands">
  Scopri i comandi disponibili
</h3>

Puoi inviare comandi che funzionano senza un terminale interattivo attraverso l'SDK. Il messaggio `system/init` elenca quelli disponibili nella tua sessione nel suo campo `slash_commands`. I comandi che necessitano di un terminale interattivo, come `/theme` e `/terminal-setup`, non appaiono nell'elenco. Accedi al campo quando la tua sessione inizia:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Hello Claude",
    options: { maxTurns: 1 }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      console.log("Available commands:", message.slash_commands);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage


  async def main():
      async for message in query(prompt="Hello Claude", options=ClaudeAgentOptions(max_turns=1)):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print("Available commands:", message.data["slash_commands"])


  asyncio.run(main())
  ```
</CodeGroup>

L'elenco stampato mescola comandi incorporati, skills nel bundle, le tue skills invocabili dall'utente e file `.claude/commands/`:

```text theme={null}
Available commands: ["clear", "compact", "context", "usage", "code-review", "verify", "security-check", ...]
```

Una skill con [`user-invocable: false`](/docs/it/skills#control-who-invokes-a-skill) nel suo frontmatter non appare in questo elenco o nell'array `skills` da [Conferma che le skills sono caricate](#confirm-skills-loaded). Le sessioni che configurano [server MCP](/docs/it/agent-sdk/mcp) possono anche esporre [prompt MCP come comandi](/docs/it/mcp#use-mcp-prompts-as-commands).

<h3 id="dispatch-commands-by-name">
  Invia comandi per nome
</h3>

Invia un comando includendolo nella tua stringa di prompt, allo stesso modo in cui invii testo regolare. L'invio non dipende dall'opzione `skills`. Inviare `/<name>` esegue una skill invocabile dall'utente anche quando il tuo elenco `skills` l'omette. I comandi che agiscono sulla cronologia della conversazione, come `/compact`, necessitano di messaggi precedenti con cui lavorare.

Un `/<name>` che non corrisponde né a un comando nella sessione né a un comando Claude Code incorporato non fa fallire la query. Claude Code invia il prompt a Claude come un messaggio ordinario, con una nota che il comando non è stato eseguito, quindi la query spende un turno del modello e restituisce la risposta di Claude. Prima della v2.1.274, un `/<name>` che non corrispondeva a nulla restituiva `Unknown command: /<name>` come risultato senza un turno del modello.

Un `/<name>` che corrisponde a un comando Claude Code incorporato che non è disponibile nella sessione, come `/theme`, restituisce `/theme isn't available in this environment.` come risultato senza un turno del modello.

<Note>
  Un comando può raggiungere il limite `maxTurns` / `max_turns` come qualsiasi altro prompt, terminando la query con un risultato di errore invece di `success`. Per il contratto del risultato di errore, vedi [Gestisci il risultato](/docs/it/agent-sdk/agent-loop#handle-the-result). Se il tuo comando potrebbe raggiungere il limite, avvolgi il loop in un `try`/`catch` in TypeScript o `try`/`except` in Python, come mostrato in [Input di un singolo messaggio](/docs/it/agent-sdk/streaming-vs-single-mode#single-message-input), oppure imposta `maxTurns` abbastanza alto affinché il lavoro si completi.
</Note>

<h3 id="compact-history-with-/compact">
  Compatta la cronologia con `/compact`
</h3>

Il comando `/compact` riduce la dimensione della tua cronologia di conversazione riassumendo i messaggi più vecchi preservando il contesto importante. La compattazione necessita di una conversazione esistente con abbastanza messaggi precedenti da riassumere. Questo esempio ha prima una conversazione, poi la compatta e legge il messaggio di sistema `compact_boundary` che riporta il risultato:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Compaction needs existing history, so have a conversation first
  try {
    for await (const message of query({
      prompt: "Explain what this project does",
      options: { maxTurns: 2 }
    })) {
      if (message.type === "result" && message.subtype === "success") {
        console.log(message.result);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so the follow-up query below still runs.
    console.error(`Session ended with an error: ${error}`);
  }

  // Compact the same conversation
  for await (const message of query({
    prompt: "/compact",
    options: { continue: true, maxTurns: 1 }
  })) {
    if (message.type === "system" && message.subtype === "compact_boundary") {
      console.log("Compaction completed");
      console.log("Pre-compaction tokens:", message.compact_metadata.pre_tokens);
      console.log("Trigger:", message.compact_metadata.trigger);
      // Example output:
      // Compaction completed
      // Pre-compaction tokens: 1842
      // Trigger: manual
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage, SystemMessage


  async def main():
      # Compaction needs existing history, so have a conversation first
      try:
          async for message in query(
              prompt="Explain what this project does",
              options=ClaudeAgentOptions(max_turns=2),
          ):
              if isinstance(message, ResultMessage) and message.subtype == "success":
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so the follow-up query below still runs.
          print(f"Session ended with an error: {error}")

      # Compact the same conversation
      async for message in query(
          prompt="/compact",
          options=ClaudeAgentOptions(continue_conversation=True, max_turns=1),
      ):
          if isinstance(message, SystemMessage) and message.subtype == "compact_boundary":
              print("Compaction completed")
              print("Pre-compaction tokens:", message.data["compact_metadata"]["pre_tokens"])
              print("Trigger:", message.data["compact_metadata"]["trigger"])
              # Example output:
              # Compaction completed
              # Pre-compaction tokens: 1842
              # Trigger: manual


  asyncio.run(main())
  ```
</CodeGroup>

<Note>
  Un messaggio `compact_boundary` arriva solo quando la compattazione è stata eseguita. Con nulla da riassumere, `/compact` riporta il motivo invece di generare un'eccezione. L'esecuzione termina comunque con un risultato `success` e nessun messaggio `compact_boundary`, e il testo del risultato riporta il motivo, ad esempio `Not enough messages to compact.` dopo un breve scambio singolo. Una nuova chiamata `query()` one-shot inizia con contesto vuoto, quindi utilizza questo modello in una sessione con turni precedenti, ad esempio in [modalità input streaming](/docs/it/agent-sdk/streaming-vs-single-mode) o quando riprendi una sessione.
</Note>

<h3 id="reset-context-with-/clear">
  Reimposta il contesto con `/clear`
</h3>

Il comando `/clear` reimposta la conversazione a un contesto vuoto, quindi i prompt successivi iniziano senza cronologia di conversazione precedente. La conversazione precedente rimane su disco. Puoi tornare a quella conversazione passando il suo ID di sessione all'[opzione `resume`](/docs/it/agent-sdk/sessions#resume-by-id).

`/clear` è utile in [modalità input streaming](/docs/it/agent-sdk/streaming-vs-single-mode), dove invii più prompt su una singola connessione. Per le chiamate `query()` one-shot, ogni chiamata inizia già con contesto vuoto, quindi inviare `/clear` non ha effetto pratico. Avvia una nuova `query()` invece.

<h2 id="create-skills">
  Crea skills
</h2>

Crea ogni skill come una directory contenente un file `SKILL.md` con frontmatter YAML e contenuto Markdown. Il campo `description` determina quando Claude richiama la tua skill.

**Struttura di directory di esempio**:

```text theme={null}
.claude/skills/security-check/
└── SKILL.md
```

<h3 id="choose-a-discovery-level">
  Scegli un livello di scoperta
</h3>

Salva le skills a uno dei due [livelli di scoperta](/docs/it/skills#where-skills-live) più comuni:

* **Skills del progetto**: `.claude/skills/`, disponibili solo nel progetto corrente
* **Skills personali**: `~/.claude/skills/`, disponibili in tutti i tuoi progetti

Se hai file di comando personalizzati esistenti in `.claude/commands/`, continuano a funzionare. Un file di comando in `.claude/commands/deploy.md` crea `/deploy` e funziona allo stesso modo di una skill in `.claude/skills/deploy/SKILL.md`. Se un file di comando e una skill condividono un nome, vedi [Risolvi skills che condividono un nome](/docs/it/skills#resolve-skills-that-share-a-name) per quale viene eseguita. L'SDK carica i file `.claude/commands/` e `~/.claude/commands/` dagli stessi due ambiti delle skills. Vedi [Estendi Claude con skills](/docs/it/skills) per la guida completa a entrambe le forme di artefatto.

<h3 id="create-and-dispatch-your-first-skill">
  Crea e invia la tua prima skill
</h3>

Per vedere il flusso completo, crea `.claude/skills/security-check/SKILL.md`:

```markdown theme={null}
---
name: security-check
description: Run a security vulnerability scan
---

Analyze the codebase for security vulnerabilities including:
- SQL injection risks
- XSS vulnerabilities
- Exposed credentials
- Insecure configurations
```

Una volta che il file esiste, la skill è disponibile attraverso l'SDK. Claude la richiama quando una richiesta corrisponde alla sua descrizione, e puoi inviarla direttamente:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "/security-check",
    options: { maxTurns: 10 }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      async for message in query(
          prompt="/security-check", options=ClaudeAgentOptions(max_turns=10)
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

Un'esecuzione riuscita termina con un risultato `success` il cui testo riporta i risultati della scansione. Contro una piccola app Express con problemi seminati, il testo del risultato inizia:

```text theme={null}
**Security scan of `app.js` — 4 findings (most severe first):**

1. **SQL Injection** (line 8) — `req.query.name` is concatenated directly into the SQL string. Trivially exploitable (`' OR '1'='1`, `'; DROP TABLE users;--`). **Fix:** use parameterized queries, e.g. `db.query("SELECT * FROM users WHERE name = ?", [req.query.name], cb)`.
...
```

Il nome della skill appare anche nell'array `slash_commands` del messaggio init.

<Note>
  Claude Code include skills nel bundle `code-review` e `verify`. Se denomini un file `.claude/commands/` dopo uno di essi, ad esempio `.claude/commands/code-review.md`, il file del comando oscura la skill nel bundle e `slash_commands` elenca il nome una volta.
</Note>

<h2 id="pre-approve-tools-for-skills">
  Pre-approva gli strumenti per le skills
</h2>

<Note>
  Per le skills del progetto e personali, Claude Code applica il campo frontmatter [`allowed-tools`](/docs/it/skills#pre-approve-tools-for-a-skill) nelle sessioni dell'SDK. Puoi anche pre-approvare gli strumenti per queste skills attraverso l'opzione `allowedTools` (`allowed_tools` in Python) nella tua configurazione di query. Le skills [sincronizzate da claude.ai](/docs/it/skills#how-claude-code-handles-the-frontmatter-of-a-synced-skill) seguono le loro proprie regole di frontmatter.
</Note>

Le skills vengono eseguite con gli strumenti della sessione. L'esempio seguente pre-approva `Read`, `Grep` e `Glob` con `allowedTools` (`allowed_tools` in Python), quindi Claude può ispezionare i file mentre esegue la [skill security-check](#create-and-dispatch-your-first-skill) senza fermarsi per l'approvazione:

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load skills from filesystem
      skills="all",
      allowed_tools=["Read", "Grep", "Glob"],
  )


  async def main():
      async for message in query(prompt="Check this project for security issues", options=options):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Check this project for security issues",
    options: {
      settingSources: ["user", "project"], // Load skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Nello stream, l'invocazione della skill appare come un uso dello strumento Skill, seguito da chiamate Read sui file del progetto. L'esecuzione termina con un risultato `success` il cui testo riporta i risultati.

L'elenco pre-approva gli strumenti denominati piuttosto che limitare gli altri. Per il flusso di autorizzazione completo, incluse le modalità di autorizzazione e il callback `canUseTool`, vedi [Autorizzazioni](/docs/it/agent-sdk/permissions).

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

<h3 id="skills-not-found">
  Skills non trovate
</h3>

**Controlla la configurazione di settingSources**: l'SDK scopre le skills attraverso le fonti di impostazione `user` e `project`. Se imposti `settingSources`/`setting_sources` esplicitamente e ometti quelle fonti, l'SDK non carica le skills:

<CodeGroup>
  ```python Python theme={null}
  # Skills not loaded: setting_sources excludes user and project
  options = ClaudeAgentOptions(setting_sources=[], skills="all")

  # Skills loaded: user and project sources included
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],
      skills="all",
  )
  ```

  ```typescript TypeScript theme={null}
  // Skills not loaded: settingSources excludes user and project
  const optionsWithoutSkills = {
    settingSources: [],
    skills: "all"
  };

  // Skills loaded: user and project sources included
  const optionsWithSkills = {
    settingSources: ["user", "project"],
    skills: "all"
  };
  ```
</CodeGroup>

Per quale directory di skills ogni fonte carica, vedi la [tabella delle fonti del filesystem](/docs/it/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources). Per ulteriori dettagli su `settingSources`/`setting_sources`, vedi il [riferimento SDK TypeScript](/docs/it/agent-sdk/typescript#settingsource) o il [riferimento SDK Python](/docs/it/agent-sdk/python#settingsource).

**Controlla la directory di lavoro**: l'SDK carica le skills da `.claude/skills/` nell'opzione `cwd` e in ogni directory padre fino alla radice del repository. Assicurati che `cwd` punti a o al di sotto della directory contenente `.claude/skills/`, all'interno dello stesso repository:

<CodeGroup>
  ```python Python theme={null}
  # Ensure your cwd points to the directory containing .claude/skills/
  options = ClaudeAgentOptions(
      cwd="/path/to/project",  # .claude/skills/ here or in a parent directory
      setting_sources=["user", "project"],  # Loads skills from these sources
      skills="all",
  )
  ```

  ```typescript TypeScript theme={null}
  // Ensure your cwd points to the directory containing .claude/skills/
  const options = {
    cwd: "/path/to/project", // .claude/skills/ here or in a parent directory
    settingSources: ["user", "project"], // Loads skills from these sources
    skills: "all"
  };
  ```
</CodeGroup>

Vedi [Utilizza le skills con l'Agent SDK](#use-skills-with-the-agent-sdk) per il modello completo.

**Verifica la posizione del filesystem**:

```bash theme={null}
# Check project skills
ls .claude/skills/*/SKILL.md

# Check personal skills
ls ~/.claude/skills/*/SKILL.md
```

<h3 id="skill-not-being-used">
  Skill non utilizzata
</h3>

**Controlla l'opzione `skills`**: se hai passato un elenco di `skills`, conferma che il nome della skill sia incluso. Quando Claude tenta di invocare una skill non elencata, lo strumento Skill restituisce `Skill <name> is not in this session's skills allowlist`. Aggiungi il nome al tuo elenco, oppure invia la skill direttamente inviando `/<name>` in un prompt, che funziona senza elencare.

**Controlla la descrizione**: assicurati che sia specifica e includa parole chiave rilevanti. Vedi [Best practices di Agent Skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions) per una guida sulla scrittura di descrizioni efficaci.

<h3 id="invalid-skill-name-error">
  Errore di nome skill non valido
</h3>

Quando un nome nel tuo elenco `skills` non può funzionare come nome di skill esatto, `query()` rifiuta l'elenco prima di avviare il processo Claude Code. I nomi che attivano il rifiuto includono:

* Un nome vuoto
* Un nome contenente parentesi, virgole o caratteri di controllo
* Un nome riempito con spazi bianchi
* Una forma wildcard come un `*` nudo o un suffisso `:*`

Ogni SDK presenta il rifiuto diversamente:

<Tabs>
  <Tab title="TypeScript">
    L'SDK TypeScript genera un `Error` che indica la regola che la voce ha violato. Ad esempio, `skills: ["docs:*"]` genera:

    ```text theme={null}
    Invalid skill name "docs:*": wildcard-suffix names are not allowed; list each skill by its exact name.
    ```

    Un nome vuoto riporta `Skill names must be non-empty strings.`

    Prima dell'Agent SDK TypeScript 0.3.221, l'SDK non eseguiva questo controllo.
  </Tab>

  <Tab title="Python">
    L'SDK Python genera `ValueError` che indica la regola che la voce ha violato. Ad esempio, `skills=["docs:*"]` genera:

    ```text theme={null}
    ValueError: Invalid skill name 'docs:*': wildcard-suffix names are not allowed; list each skill by its exact name.
    ```

    Un nome vuoto riporta `Skill names must be non-empty strings`.

    Prima dell'Agent SDK Python 0.2.129, l'SDK non eseguiva questo controllo.
  </Tab>
</Tabs>

<h3 id="additional-troubleshooting">
  Risoluzione dei problemi aggiuntiva
</h3>

Per la risoluzione generale dei problemi delle skills, come errori di sintassi YAML e debug, vedi la [sezione di risoluzione dei problemi delle skills di Claude Code](/docs/it/skills#troubleshooting).

<h2 id="next-steps">
  Passaggi successivi
</h2>

La [guida alle skills di Claude Code](/docs/it/skills) copre l'authoring in profondità. La sua guida si applica alle sessioni dell'SDK. Inizia con queste sezioni:

* [Riferimento del frontmatter](/docs/it/skills#frontmatter-reference): ogni campo supportato
* [Passa argomenti alle skills](/docs/it/skills#pass-arguments-to-skills): `$ARGUMENTS`, `$0`, `$1` e skill stacking. La [tabella di sostituzione completa](/docs/it/skills#available-string-substitutions) aggiunge argomenti denominati e le variabili `${CLAUDE_*}`
* [Inietta contesto dinamico](/docs/it/skills#inject-dynamic-context): righe `` !`command` `` che vengono eseguite prima che Claude veda il contenuto della skill
* [Scegli dove le skills si caricano](/docs/it/skills#where-skills-live): ogni posizione della skill, namespacing dei plugin e quale skill viene eseguita quando due condividono un nome

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Comandi in Claude Code](/docs/it/commands): la superficie del comando completa, incluso ogni incorporato
* [Panoramica di Agent Skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview): panoramica concettuale, vantaggi e architettura
* [Best practices di Agent Skills](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices): linee guida di authoring per skills efficaci
* [Cookbook di Agent Skills](https://platform.claude.com/cookbook/skills-notebooks-01-skills-introduction): skills di esempio e modelli
* [Subagents nell'SDK](/docs/it/agent-sdk/subagents): agenti basati su filesystem simili con opzioni programmatiche
* [Panoramica dell'SDK](/docs/it/agent-sdk/overview): concetti generali dell'SDK
* [Riferimento SDK TypeScript](/docs/it/agent-sdk/typescript): documentazione API completa
* [Riferimento SDK Python](/docs/it/agent-sdk/python): documentazione API completa
