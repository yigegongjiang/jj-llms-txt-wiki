> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent Skills nell'SDK

> Estendi Claude con capacità specializzate utilizzando Agent Skills nell'SDK dell'Agent Claude

<h2 id="overview">
  Panoramica
</h2>

Agent Skills estendono Claude con capacità specializzate che Claude richiama autonomamente quando rilevante. Le Skills sono confezionate come file `SKILL.md` contenenti istruzioni, descrizioni e risorse di supporto opzionali.

Per informazioni complete su Skills, inclusi vantaggi, architettura e linee guida di authoring, consulta la [panoramica di Agent Skills](https://platform.claude.com/docs/it/agents-and-tools/agent-skills/overview).

<h2 id="how-skills-work-with-the-sdk">
  Come funzionano le Skills con l'SDK
</h2>

Quando si utilizza l'SDK dell'Agent Claude, le Skills sono:

1. **Definite come artefatti del filesystem**: Create come file `SKILL.md` in directory specifiche (`.claude/skills/`)
2. **Caricate dal filesystem**: Le Skills sono caricate da posizioni del filesystem governate da `settingSources` (TypeScript) o `setting_sources` (Python)
3. **Scoperte automaticamente**: Una volta caricate le impostazioni del filesystem, i metadati della Skill vengono scoperti all'avvio dalle directory dell'utente e del progetto; il contenuto completo viene caricato quando attivato
4. **Richiamate dal modello**: Claude sceglie autonomamente quando utilizzarle in base al contesto
5. **Filtrate tramite l'opzione `skills`**: Le Skills scoperte sono abilitate per impostazione predefinita. Passa un elenco di nomi di Skills, `"all"`, o `[]` per controllare quali sono disponibili nella sessione

A differenza dei subagents (che possono essere definiti programmaticamente), le Skills devono essere create come artefatti del filesystem. L'SDK non fornisce un'API programmatica per registrare le Skills.

<Note>
  Le Skills vengono scoperte attraverso le fonti di impostazione del filesystem. Con le opzioni predefinite di `query()`, l'SDK carica le fonti utente e progetto, quindi le Skills in `~/.claude/skills/`, `<cwd>/.claude/skills/`, e `.claude/skills/` in qualsiasi directory padre di `<cwd>` fino alla radice del repository sono disponibili. Se imposti `settingSources` esplicitamente, includi `'user'` o `'project'` per mantenere la scoperta delle Skills, oppure utilizza l'[opzione `plugins`](/docs/it/agent-sdk/plugins) per caricare le Skills da un percorso specifico.
</Note>

<h2 id="using-skills-with-the-sdk">
  Utilizzo delle Skills con l'SDK
</h2>

Imposta l'opzione `skills` su `query()` per controllare quali Skills sono disponibili per la sessione. Se omessa, le Skills scoperte sono abilitate e lo strumento Skill è disponibile, corrispondendo al comportamento della CLI. Passa `"all"` per abilitare ogni Skill scoperta, un elenco di nomi di Skills per abilitare solo quelle, o `[]` per disabilitare tutte. Quando imposti `skills`, l'SDK aggiunge automaticamente lo strumento Skill a `allowedTools`. Se passi anche un elenco esplicito di `tools`, includi `"Skill"` in quell'elenco in modo che Claude possa invocare le skills.

Una volta configurato, Claude scopre automaticamente le Skills dal filesystem e le richiama quando rilevante per la richiesta dell'utente.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      options = ClaudeAgentOptions(
          cwd="/path/to/project",  # Project with .claude/skills/
          setting_sources=["user", "project"],  # Load Skills from filesystem
          skills="all",  # Enable every discovered Skill
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
      cwd: "/path/to/project", // Project with .claude/skills/
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all", // Enable every discovered Skill
      allowedTools: ["Read", "Write", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Per abilitare solo Skills specifiche, passa i loro nomi. I nomi corrispondono al campo `name` in `SKILL.md` o al nome della directory della Skill. Utilizza `plugin:skill` per le Skills fornite da plugin.

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(skills=["pdf", "docx"])
  ```

  ```typescript TypeScript theme={null}
  const options = { skills: ["pdf", "docx"] };
  ```
</CodeGroup>

L'opzione `skills` è un filtro di contesto, non una sandbox. Le Skills non elencate sono nascoste dal modello e rifiutate dallo strumento Skill, ma i loro file rimangono sul disco e sono raggiungibili tramite Read e Bash.

<h2 id="skill-locations">
  Posizioni delle Skills
</h2>

Le Skills vengono caricate dalle directory del filesystem in base alla configurazione di `settingSources`/`setting_sources`:

* **Project Skills** (`.claude/skills/`): Condivise con il tuo team tramite git - caricate quando `setting_sources` include `"project"`
* **User Skills** (`~/.claude/skills/`): Skills personali su tutti i progetti - caricate quando `setting_sources` include `"user"`
* **Plugin Skills**: Fornite con i plugin Claude Code installati

<h2 id="creating-skills">
  Creazione di Skills
</h2>

Le Skills sono definite come directory contenenti un file `SKILL.md` con frontmatter YAML e contenuto Markdown. Il campo `description` determina quando Claude richiama la tua Skill.

**Struttura di directory di esempio**:

```bash theme={null}
.claude/skills/processing-pdfs/
└── SKILL.md
```

Per una guida completa sulla creazione di Skills, inclusa la struttura di SKILL.md, Skills multi-file ed esempi, consulta:

* [Agent Skills in Claude Code](/docs/it/skills): Guida completa con esempi
* [Agent Skills Best Practices](https://platform.claude.com/docs/it/agents-and-tools/agent-skills/best-practices): Linee guida di authoring e convenzioni di denominazione

<h2 id="tool-restrictions">
  Restrizioni degli strumenti
</h2>

<Note>
  Il campo frontmatter `allowed-tools` in SKILL.md è supportato solo quando si utilizza direttamente la CLI di Claude Code. **Non si applica quando si utilizzano Skills tramite l'SDK**.

  Quando si utilizza l'SDK, controlla l'accesso agli strumenti tramite l'opzione principale `allowedTools` nella configurazione della query.
</Note>

Per controllare l'accesso agli strumenti per le Skills nelle applicazioni SDK, utilizza `allowedTools` per pre-approvare strumenti specifici. Senza un callback `canUseTool`, tutto ciò che non è nell'elenco viene negato:

<Note>
  Le istruzioni di importazione dal primo esempio sono assunte nei seguenti frammenti di codice.
</Note>

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
      allowed_tools=["Read", "Grep", "Glob"],
  )

  async for message in query(prompt="Analyze the codebase structure", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Analyze the codebase structure",
    options: {
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"],
      permissionMode: "dontAsk" // Deny anything not in allowedTools
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h2 id="discovering-available-skills">
  Scoperta delle Skills disponibili
</h2>

Per vedere quali Skills sono disponibili nella tua applicazione SDK, chiedi semplicemente a Claude:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
  )

  async for message in query(prompt="What Skills are available?", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "What Skills are available?",
    options: {
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all"
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Claude elencherà le Skills disponibili in base alla tua directory di lavoro corrente e ai plugin installati.

<h2 id="testing-skills">
  Test delle Skills
</h2>

Testa le Skills ponendo domande che corrispondono alle loro descrizioni:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      cwd="/path/to/project",
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
      allowed_tools=["Read", "Bash"],
  )

  async for message in query(prompt="Extract text from invoice.pdf", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Extract text from invoice.pdf",
    options: {
      cwd: "/path/to/project",
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Claude richiama automaticamente la Skill rilevante se la descrizione corrisponde alla tua richiesta.

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

<h3 id="skills-not-found">
  Skills non trovate
</h3>

**Controlla la configurazione di settingSources**: Le Skills vengono scoperte attraverso le fonti di impostazione `user` e `project`. Se imposti `settingSources`/`setting_sources` esplicitamente e ometti quelle fonti, le Skills non vengono caricate:

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
  const options = {
    settingSources: [],
    skills: "all"
  };

  // Skills loaded: user and project sources included
  const options = {
    settingSources: ["user", "project"],
    skills: "all"
  };
  ```
</CodeGroup>

Per ulteriori dettagli su `settingSources`/`setting_sources`, consulta il [riferimento SDK TypeScript](/docs/it/agent-sdk/typescript#settingsource) o il [riferimento SDK Python](/docs/it/agent-sdk/python#settingsource).

**Controlla la directory di lavoro**: L'SDK carica le Skills da `.claude/skills/` nell'opzione `cwd` e in ogni directory padre fino alla radice del repository. Assicurati che `cwd` punti a o al di sotto della directory contenente `.claude/skills/`, all'interno dello stesso repository:

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

Consulta la sezione "Utilizzo delle Skills con l'SDK" sopra per il modello completo.

**Verifica la posizione del filesystem**:

```bash theme={null}
# Check project Skills
ls .claude/skills/*/SKILL.md

# Check personal Skills
ls ~/.claude/skills/*/SKILL.md
```

<h3 id="skill-not-being-used">
  Skill non utilizzata
</h3>

**Controlla l'opzione `skills`**: Se hai passato un elenco di `skills`, conferma che il nome della Skill sia incluso. Passare `[]` disabilita tutte le Skills.

**Controlla la descrizione**: Assicurati che sia specifica e includa parole chiave rilevanti. Consulta [Agent Skills Best Practices](https://platform.claude.com/docs/it/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions) per una guida sulla scrittura di descrizioni efficaci.

<h3 id="additional-troubleshooting">
  Risoluzione dei problemi aggiuntiva
</h3>

Per la risoluzione generale dei problemi delle Skills (sintassi YAML, debug, ecc.), consulta la [sezione di risoluzione dei problemi delle Skills di Claude Code](/docs/it/skills#troubleshooting).

<h2 id="related-documentation">
  Documentazione correlata
</h2>

<h3 id="skills-guides">
  Guide sulle Skills
</h3>

* [Agent Skills in Claude Code](/docs/it/skills): Guida completa delle Skills con creazione, esempi e risoluzione dei problemi
* [Agent Skills Overview](https://platform.claude.com/docs/it/agents-and-tools/agent-skills/overview): Panoramica concettuale, vantaggi e architettura
* [Agent Skills Best Practices](https://platform.claude.com/docs/it/agents-and-tools/agent-skills/best-practices): Linee guida di authoring per Skills efficaci
* [Agent Skills Cookbook](https://platform.claude.com/cookbook/skills-notebooks-01-skills-introduction): Skills di esempio e modelli

<h3 id="sdk-resources">
  Risorse SDK
</h3>

* [Subagents in the SDK](/docs/it/agent-sdk/subagents): Agenti basati su filesystem simili con opzioni programmatiche
* [Slash Commands in the SDK](/docs/it/agent-sdk/slash-commands): Comandi richiamati dall'utente
* [SDK Overview](/docs/it/agent-sdk/overview): Concetti generali dell'SDK
* [TypeScript SDK Reference](/docs/it/agent-sdk/typescript): Documentazione API completa
* [Python SDK Reference](/docs/it/agent-sdk/python): Documentazione API completa
