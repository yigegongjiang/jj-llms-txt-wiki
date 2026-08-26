> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Subagenti nell'SDK

> Definisci e richiama subagenti per isolare il contesto, eseguire attività in parallelo e applicare istruzioni specializzate nelle tue applicazioni Claude Agent SDK.

I subagenti sono istanze di agente separate che il tuo agente principale può generare per gestire sottoattività mirate.
Utilizza i subagenti per isolare il contesto, eseguire più analisi in parallelo e applicare istruzioni specializzate senza appesantire il prompt dell'agente principale.

Questa guida spiega come definire e utilizzare i subagenti nell'SDK utilizzando il parametro `agents`.

<h2 id="overview">
  Panoramica
</h2>

Puoi creare subagenti in tre modi:

* **A livello di programmazione**: utilizza il parametro `agents` nelle tue opzioni `query()`. Vedi i riferimenti [TypeScript](/docs/it/agent-sdk/typescript#agentdefinition) e [Python](/docs/it/agent-sdk/python#agentdefinition)
* **Basato su file system**: definisci gli agenti come file markdown nelle directory `.claude/agents/`. Vedi [definizione di subagenti come file](/docs/it/sub-agents)
* **Generale integrato**: Claude può richiamare il subagente `general-purpose` integrato in qualsiasi momento tramite lo strumento Agent senza che tu debba definire nulla

Questa guida si concentra sull'approccio programmatico, che è consigliato per le applicazioni SDK.

Quando definisci i subagenti, Claude determina se richiamarli in base al campo `description` di ogni subagente. Scrivi descrizioni chiare che spieghino quando il subagente dovrebbe essere utilizzato, e Claude delegherà automaticamente i compiti appropriati. Puoi anche richiedere esplicitamente un subagente per nome nel tuo prompt, ad esempio "Usa l'agente code-reviewer per...".

<h2 id="benefits-of-using-subagents">
  Vantaggi dell'utilizzo dei subagenti
</h2>

<h3 id="context-isolation">
  Isolamento del contesto
</h3>

Ogni subagente viene eseguito nella propria conversazione nuova. Le chiamate agli strumenti intermedi e i risultati rimangono all'interno del subagente; solo il suo messaggio finale ritorna al genitore. Vedi [Cosa ereditano i subagenti](#what-subagents-inherit) per sapere esattamente cosa c'è nel contesto del subagente.

**Esempio:** un subagente `research-assistant` può esplorare dozzine di file senza che nessuno di questi contenuti si accumuli nella conversazione principale. Il genitore riceve un riassunto conciso, non ogni file che il subagente ha letto.

<h3 id="parallelization">
  Parallelizzazione
</h3>

Più subagenti possono essere eseguiti contemporaneamente, quindi i sottoincarichi indipendenti si completano nel tempo di quello più lento piuttosto che nella somma di tutti loro.

**Esempio:** durante una revisione del codice, puoi eseguire i subagenti `style-checker`, `security-scanner` e `test-coverage` simultaneamente invece che sequenzialmente.

<h3 id="specialized-instructions-and-knowledge">
  Istruzioni e conoscenze specializzate
</h3>

Ogni subagente può avere prompt di sistema personalizzati con competenze specifiche, best practice e vincoli.

**Esempio:** un subagente `database-migration` può avere conoscenze dettagliate sulle best practice SQL, strategie di rollback e controlli di integrità dei dati che sarebbero rumore inutile nelle istruzioni dell'agente principale.

<h3 id="tool-restrictions">
  Restrizioni degli strumenti
</h3>

I subagenti possono essere limitati a strumenti specifici, riducendo il rischio di azioni indesiderate.

**Esempio:** un subagente `doc-reviewer` potrebbe avere accesso solo agli strumenti Read e Grep, assicurando che possa analizzare ma non modifichi mai accidentalmente i tuoi file di documentazione.

<h2 id="create-subagents">
  Creazione di subagenti
</h2>

<h3 id="programmatic-definition-recommended">
  Definizione programmatica (consigliata)
</h3>

Definisci i subagenti direttamente nel tuo codice utilizzando il parametro `agents`. Claude richiama i subagenti tramite lo strumento `Agent`, quindi includi `Agent` in `allowedTools` per approvare automaticamente le invocazioni dei subagenti senza una richiesta di autorizzazione.

La maggior parte degli esempi in questa pagina stampa solo il risultato finale. Per confermare che Claude ha delegato a un subagente piuttosto che rispondere direttamente, vedi [Rilevamento dell'invocazione del subagente](#detect-subagent-invocation).

Questo esempio crea due subagenti: un revisore di codice con accesso in sola lettura e un esecutore di test che può eseguire comandi.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Review the authentication module for security issues",
          options=ClaudeAgentOptions(
              # Auto-approve these tools, including Agent for subagent invocation
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  "code-reviewer": AgentDefinition(
                      # description tells Claude when to use this subagent
                      description="Expert code review specialist. Use for quality, security, and maintainability reviews.",
                      # prompt defines the subagent's behavior and expertise
                      prompt="""You are a code review specialist with expertise in security, performance, and best practices.

  When reviewing code:
  - Identify security vulnerabilities
  - Check for performance issues
  - Verify adherence to coding standards
  - Suggest specific improvements

  Be thorough but concise in your feedback.""",
                      # tools restricts what the subagent can do (read-only here)
                      tools=["Read", "Grep", "Glob"],
                      # model overrides the default model for this subagent
                      model="sonnet",
                  ),
                  "test-runner": AgentDefinition(
                      description="Runs and analyzes test suites. Use for test execution and coverage analysis.",
                      prompt="""You are a test execution specialist. Run tests and provide clear analysis of results.

  Focus on:
  - Running test commands
  - Analyzing test output
  - Identifying failing tests
  - Suggesting fixes for failures""",
                      # Bash access lets this subagent run test commands
                      tools=["Bash", "Read", "Grep"],
                  ),
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Review the authentication module for security issues",
    options: {
      // Auto-approve these tools, including Agent for subagent invocation
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        "code-reviewer": {
          // description tells Claude when to use this subagent
          description:
            "Expert code review specialist. Use for quality, security, and maintainability reviews.",
          // prompt defines the subagent's behavior and expertise
          prompt: `You are a code review specialist with expertise in security, performance, and best practices.

  When reviewing code:
  - Identify security vulnerabilities
  - Check for performance issues
  - Verify adherence to coding standards
  - Suggest specific improvements

  Be thorough but concise in your feedback.`,
          // tools restricts what the subagent can do (read-only here)
          tools: ["Read", "Grep", "Glob"],
          // model overrides the default model for this subagent
          model: "sonnet"
        },
        "test-runner": {
          description:
            "Runs and analyzes test suites. Use for test execution and coverage analysis.",
          prompt: `You are a test execution specialist. Run tests and provide clear analysis of results.

  Focus on:
  - Running test commands
  - Analyzing test output
  - Identifying failing tests
  - Suggesting fixes for failures`,
          // Bash access lets this subagent run test commands
          tools: ["Bash", "Read", "Grep"]
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h3 id="agentdefinition-configuration">
  Configurazione di AgentDefinition
</h3>

| Campo             | Tipo                                                        | Obbligatorio | Descrizione                                                                                                                                                                                                                                                         |
| :---------------- | :---------------------------------------------------------- | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `description`     | `string`                                                    | Sì           | Descrizione in linguaggio naturale di quando utilizzare questo agente                                                                                                                                                                                               |
| `prompt`          | `string`                                                    | Sì           | Il prompt di sistema dell'agente che definisce il suo ruolo e comportamento                                                                                                                                                                                         |
| `tools`           | `string[]`                                                  | No           | Array di nomi di strumenti consentiti. Se omesso, eredita tutti gli strumenti                                                                                                                                                                                       |
| `disallowedTools` | `string[]`                                                  | No           | Array di nomi di strumenti da rimuovere dal set di strumenti dell'agente. Sono accettati anche i pattern a livello di server MCP: `mcp__server` o `mcp__server__*` rimuove ogni strumento da quel server, e `mcp__*` rimuove ogni strumento MCP da qualsiasi server |
| `model`           | `string`                                                    | No           | Override del modello per questo agente. Accetta un alias come `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, o un ID modello completo. Predefinito al modello principale se omesso                                                                        |
| `skills`          | `string[]`                                                  | No           | Elenco di nomi di skill da precaricare nel contesto dell'agente all'avvio. Le skill non elencate rimangono invocabili tramite lo strumento Skill                                                                                                                    |
| `memory`          | `'user' \| 'project' \| 'local'`                            | No           | Fonte di memoria per questo agente                                                                                                                                                                                                                                  |
| `mcpServers`      | `(string \| object)[]`                                      | No           | Server MCP disponibili per questo agente, per nome o configurazione inline                                                                                                                                                                                          |
| `initialPrompt`   | `string`                                                    | No           | Inviato automaticamente come primo turno utente quando questo agente viene eseguito come agente del thread principale. Ignorato quando l'agente viene richiamato come subagente                                                                                     |
| `maxTurns`        | `number`                                                    | No           | Numero massimo di turni agentici prima che l'agente si fermi                                                                                                                                                                                                        |
| `background`      | `boolean`                                                   | No           | Esegui questo agente come attività di background non bloccante quando richiamato                                                                                                                                                                                    |
| `effort`          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max' \| number` | No           | Livello di sforzo di ragionamento per questo agente                                                                                                                                                                                                                 |
| `permissionMode`  | `PermissionMode`                                            | No           | Modalità di autorizzazione per l'esecuzione dello strumento all'interno di questo agente                                                                                                                                                                            |

Nell'SDK Python, i nomi di campo con più parole come `disallowedTools` e `mcpServers` mantengono la loro ortografia camelCase per corrispondere al formato wire piuttosto che seguire la convenzione snake\_case di Python. Vedi il riferimento [`AgentDefinition`](/docs/it/agent-sdk/python#agentdefinition) per i dettagli.

Due comportamenti dei subagenti sono cambiati in Claude Code v2.1.198:

* I subagenti vengono eseguiti in background per impostazione predefinita. Una chiamata dello strumento Agent che omette l'input [`run_in_background`](/docs/it/agent-sdk/typescript) avvia un subagente in background, e Claude imposta `run_in_background: false` quando ha bisogno del risultato prima di continuare. Prima della v2.1.198, omettere `run_in_background` eseguiva il subagente in modo sincrono. Imposta il campo `background` su `true` per forzare l'esecuzione in background per un agente specifico indipendentemente da ciò che Claude richiede.
* Un subagente eredita la configurazione del pensiero esteso della sessione principale. Nelle versioni precedenti, il pensiero esteso è disabilitato all'interno dei subagenti indipendentemente dall'impostazione della sessione principale.

<Note>
  A partire da Claude Code v2.1.172, i subagenti possono generare i propri subagenti. Un subagente cinque livelli al di sotto dell'agente principale non può generare ulteriori subagenti, indipendentemente dal fatto che venga eseguito in primo piano o in background. Per impedire a un subagente di generare altri, ometti `Agent` dal suo array `tools` o aggiungilo a `disallowedTools`. Vedi [subagenti annidati](/docs/it/sub-agents#spawn-nested-subagents) per le regole di profondità complete.
</Note>

<h3 id="filesystem-based-definition-alternative">
  Definizione basata su file system (alternativa)
</h3>

Puoi anche definire i subagenti come file markdown nelle directory `.claude/agents/`. Vedi la [documentazione dei subagenti Claude Code](/docs/it/sub-agents) per i dettagli su questo approccio. Gli agenti definiti a livello di programmazione hanno la precedenza sugli agenti basati su file system con lo stesso nome.

<Note>
  Anche senza definire subagenti personalizzati, Claude può generare il subagente `general-purpose` integrato. Questo è utile per delegare attività di ricerca o esplorazione senza creare agenti specializzati. Includi `Agent` in `allowedTools` affinché queste invocazioni si approvino automaticamente senza una richiesta di autorizzazione.
</Note>

<h2 id="what-subagents-inherit">
  Cosa ereditano i subagenti
</h2>

La finestra di contesto di un subagente inizia da zero, senza conversazione genitore, ma non è vuota. L'unico contenuto che passi dal genitore al subagente è la stringa di prompt dello strumento Agent, quindi includi direttamente nel prompt qualsiasi percorso di file, messaggio di errore o decisione di cui il subagente ha bisogno.

Un subagente che ha lo strumento [`SendMessage`](/docs/it/tools-reference) inizia con un elenco degli altri agenti denominati in esecuzione nella sessione, quindi sa quali nomi può utilizzare per inviare messaggi. Claude Code aggiunge automaticamente l'elenco al primo turno del subagente. Un [fork](/docs/it/sub-agents#fork-the-current-conversation) non riceve l'elenco perché eredita invece la conversazione genitore. L'elenco richiede Claude Code v2.1.206 o successivo.

| Il subagente riceve                                                                                                                              | Il subagente non riceve                                                                 |
| :----------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------- |
| Il suo prompt di sistema (`AgentDefinition.prompt`) e il prompt dello strumento Agent                                                            | La cronologia della conversazione del genitore o i risultati degli strumenti            |
| CLAUDE.md del progetto (caricato tramite [`settingSources`](/docs/it/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)) | Contenuto di skill precaricato, a meno che non sia elencato in `AgentDefinition.skills` |
| Definizioni degli strumenti (ereditate dal genitore, o il sottoinsieme in `tools`)                                                               | Il prompt di sistema del genitore                                                       |

<Note>
  Il genitore riceve il messaggio finale del subagente verbatim come risultato dello strumento Agent, ma potrebbe riassumerlo nella sua risposta. Per preservare l'output del subagente verbatim nella risposta rivolta all'utente, includi un'istruzione per farlo nel prompt o nell'opzione `systemPrompt` che passi alla chiamata principale `query()`.
</Note>

Un errore API che termina il subagente anticipatamente, come un limite di velocità, non viene mai consegnato come risultato. Se un limite di velocità, un sovraccarico o un errore del server interrompe un subagente in primo piano che ha già prodotto output di testo, lo strumento Agent restituisce quell'output parziale con una nota che il subagente non ha terminato. Un subagente che non ha prodotto nulla, o il cui unico output erano chiamate di strumenti senza testo, fallisce con un messaggio di errore, `Agent terminated early due to an API error`, seguito dal dettaglio dell'errore. Vedi [API errors in subagents](/docs/it/sub-agents#api-errors-in-subagents) per il comportamento in primo piano e in background.

Questa gestione dell'output parziale richiede Claude Code v2.1.199 o successivo. Nella v2.1.199, un limite di velocità, un sovraccarico o un errore del server ha lasciato la forma con sole chiamate di strumenti con un risultato parziale vuoto contenente solo la nota di interruzione.

<h2 id="invoke-subagents">
  Richiamo dei subagenti
</h2>

<h3 id="automatic-invocation">
  Richiamo automatico
</h3>

Claude decide automaticamente quando richiamare i subagenti in base al compito e alla `description` di ogni subagente. Ad esempio, se definisci un subagente `performance-optimizer` con la descrizione "Performance optimization specialist for query tuning", Claude lo richiamerà quando il tuo prompt menziona l'ottimizzazione delle query.

Scrivi descrizioni chiare e specifiche in modo che Claude possa abbinare i compiti al subagente giusto.

<h3 id="explicit-invocation">
  Richiamo esplicito
</h3>

Per garantire che Claude utilizzi un subagente specifico, menzionalo per nome nel tuo prompt:

```text theme={null}
"Use the code-reviewer agent to check the authentication module"
```

Questo bypassa l'abbinamento automatico e richiama direttamente il subagente denominato.

<h3 id="dynamic-agent-configuration">
  Configurazione dinamica dell'agente
</h3>

Puoi creare definizioni di agente dinamicamente in base alle condizioni di runtime. Questo esempio crea un revisore di sicurezza con diversi livelli di rigore, utilizzando un modello più potente per revisioni rigorose.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  # Factory function that returns an AgentDefinition
  # This pattern lets you customize agents based on runtime conditions
  def create_security_agent(security_level: str) -> AgentDefinition:
      is_strict = security_level == "strict"
      return AgentDefinition(
          description="Security code reviewer",
          # Customize the prompt based on strictness level
          prompt=f"You are a {'strict' if is_strict else 'balanced'} security reviewer...",
          tools=["Read", "Grep", "Glob"],
          # Key insight: use a more capable model for high-stakes reviews
          model="opus" if is_strict else "sonnet",
      )


  async def main():
      # The agent is created at query time, so each request can use different settings
      async for message in query(
          prompt="Review this PR for security issues",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  # Call the factory with your desired configuration
                  "security-reviewer": create_security_agent("strict")
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type AgentDefinition } from "@anthropic-ai/claude-agent-sdk";

  // Factory function that returns an AgentDefinition
  // This pattern lets you customize agents based on runtime conditions
  function createSecurityAgent(securityLevel: "basic" | "strict"): AgentDefinition {
    const isStrict = securityLevel === "strict";
    return {
      description: "Security code reviewer",
      // Customize the prompt based on strictness level
      prompt: `You are a ${isStrict ? "strict" : "balanced"} security reviewer...`,
      tools: ["Read", "Grep", "Glob"],
      // Key insight: use a more capable model for high-stakes reviews
      model: isStrict ? "opus" : "sonnet"
    };
  }

  // The agent is created at query time, so each request can use different settings
  for await (const message of query({
    prompt: "Review this PR for security issues",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        // Call the factory with your desired configuration
        "security-reviewer": createSecurityAgent("strict")
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h2 id="detect-subagent-invocation">
  Rilevamento dell'invocazione del subagente
</h2>

Claude richiama i subagenti tramite lo strumento Agent. Per rilevare quando un subagente viene richiamato, controlla i blocchi `tool_use` dove `name` è `"Agent"`. I messaggi provenienti dal contesto di un subagente includono un campo `parent_tool_use_id`.

<Note>
  Il nome dello strumento è stato rinominato da `"Task"` a `"Agent"` in Claude Code v2.1.63. Le versioni attuali dell'SDK emettono `"Agent"` nei blocchi `tool_use` ma utilizzano ancora `"Task"` nell'elenco degli strumenti `system:init` e in `result.permission_denials[].tool_name`. Controllare entrambi i valori in `block.name` garantisce la compatibilità tra le versioni dell'SDK.
</Note>

La struttura del messaggio differisce tra gli SDK. In Python, i blocchi di contenuto sono accessibili direttamente tramite `message.content`. In TypeScript, `SDKAssistantMessage` avvolge il messaggio dell'API Claude, quindi il contenuto è accessibile tramite `message.message.content`.

Questo esempio itera attraverso i messaggi trasmessi, registrando quando un subagente viene richiamato e quando i messaggi successivi provengono dal contesto di esecuzione di quel subagente.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolUseBlock


  async def main():
      async for message in query(
          prompt="Use the code-reviewer agent to review this codebase",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Glob", "Grep", "Agent"],
              agents={
                  "code-reviewer": AgentDefinition(
                      description="Expert code reviewer.",
                      prompt="Analyze code quality and suggest improvements.",
                      tools=["Read", "Glob", "Grep"],
                  )
              },
          ),
      ):
          # Check for subagent invocation. Match both names: older SDK
          # versions emitted "Task", current versions emit "Agent".
          if hasattr(message, "content") and message.content:
              for block in message.content:
                  if isinstance(block, ToolUseBlock) and block.name in (
                      "Task",
                      "Agent",
                  ):
                      print(f"Subagent invoked: {block.input.get('subagent_type')}")

          # Check if this message is from within a subagent's context
          if hasattr(message, "parent_tool_use_id") and message.parent_tool_use_id:
              print("  (running inside subagent)")

          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Use the code-reviewer agent to review this codebase",
    options: {
      allowedTools: ["Read", "Glob", "Grep", "Agent"],
      agents: {
        "code-reviewer": {
          description: "Expert code reviewer.",
          prompt: "Analyze code quality and suggest improvements.",
          tools: ["Read", "Glob", "Grep"]
        }
      }
    }
  })) {
    const msg = message as any;

    // Check for subagent invocation. Match both names: older SDK versions
    // emitted "Task", current versions emit "Agent".
    for (const block of msg.message?.content ?? []) {
      if (block.type === "tool_use" && (block.name === "Task" || block.name === "Agent")) {
        console.log(`Subagent invoked: ${block.input.subagent_type}`);
      }
    }

    // Check if this message is from within a subagent's context
    if (msg.parent_tool_use_id) {
      console.log("  (running inside subagent)");
    }

    if ("result" in message) {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<h2 id="resume-subagents">
  Ripresa dei subagenti
</h2>

È possibile riprendere un subagente per continuare da dove si era fermato piuttosto che iniziare da zero. Un subagente ripreso mantiene la sua cronologia di conversazione completa, incluse tutte le chiamate agli strumenti precedenti, i risultati e il ragionamento.

Quando un subagente si completa, il risultato dello strumento Agent include un blocco di testo contenente `agentId: <id>`. Gli agenti integrati [`Explore` e `Plan`](/docs/it/sub-agents#built-in-subagents) sono monouso e non restituiscono un `agentId`, quindi utilizzare un agente personalizzato o `general-purpose` quando è necessario riprendere. Per riprendere un subagente a livello di programmazione:

1. **Cattura l'ID della sessione**: estrai `session_id` dai messaggi durante la prima query
2. **Estrai l'ID dell'agente**: analizza `agentId` dal testo del risultato dello strumento Agent
3. **Riprendi la sessione**: passa `resume: sessionId` nelle opzioni della seconda query e includi l'ID dell'agente nel tuo prompt

<Note>
  Devi riprendere la stessa sessione per accedere alla trascrizione del subagente. Ogni chiamata `query()` avvia una nuova sessione per impostazione predefinita, quindi passa `resume: sessionId` per continuare nella stessa sessione.

  Quando utilizzi un agente personalizzato, passa la stessa definizione di agente nel parametro `agents` per entrambe le query.
</Note>

L'esempio seguente definisce un agente personalizzato `endpoint-finder`. La prima query lo esegue e cattura l'ID della sessione e l'ID dell'agente dal risultato dello strumento Agent, quindi la seconda query riprende la sessione per porre una domanda di follow-up che richiede il contesto della prima analisi.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import re
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolResultBlock

  AGENTS = {
      "endpoint-finder": AgentDefinition(
          description="Locates and catalogs API endpoints in a codebase.",
          prompt="You find and document API endpoints. Report each endpoint's path, method, and handler.",
          tools=["Read", "Grep", "Glob"],
      )
  }


  def extract_agent_id(block: ToolResultBlock) -> str | None:
      """Extract agentId from an Agent tool result's text content."""
      parts = block.content if isinstance(block.content, list) else [{"text": block.content}]
      for part in parts:
          if match := re.search(r"agentId:\s*([\w-]+)", part.get("text") or ""):
              return match.group(1)
      return None


  async def main():
      agent_id = None
      session_id = None

      # First invocation - run the endpoint-finder subagent
      try:
          async for message in query(
              prompt="Use the endpoint-finder agent to find all API endpoints in this codebase",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS),
          ):
              # Capture session_id from ResultMessage (needed to resume this session)
              if hasattr(message, "session_id"):
                  session_id = message.session_id
              # Search tool results for the agentId trailer
              for block in getattr(message, "content", None) or []:
                  if isinstance(block, ToolResultBlock):
                      agent_id = extract_agent_id(block) or agent_id
              # Print the final result
              if hasattr(message, "result"):
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so session_id and agent_id have already been captured by the loop above.
          print(f"Session ended with an error: {error}")

      # Second invocation - resume and ask follow-up
      if agent_id and session_id:
          async for message in query(
              prompt=f"Resume agent {agent_id} and list the top 3 most complex endpoints",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS, resume=session_id
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)
      else:
          print("No agentId found in the first query, so there is no subagent to resume.")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type SDKMessage } from "@anthropic-ai/claude-agent-sdk";

  const agents = {
    "endpoint-finder": {
      description: "Locates and catalogs API endpoints in a codebase.",
      prompt: "You find and document API endpoints. Report each endpoint's path, method, and handler.",
      tools: ["Read", "Grep", "Glob"]
    }
  };

  // Stringify content to search for agentId without traversing nested block types
  function extractAgentId(message: SDKMessage): string | undefined {
    if (message.type !== "assistant" && message.type !== "user") return undefined;
    const content = JSON.stringify(message.message.content);
    const match = content.match(/agentId:\s*([\w-]+)/);
    return match?.[1];
  }

  let agentId: string | undefined;
  let sessionId: string | undefined;

  // First invocation - run the endpoint-finder subagent
  try {
    for await (const message of query({
      prompt: "Use the endpoint-finder agent to find all API endpoints in this codebase",
      options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents }
    })) {
      // Capture session_id from ResultMessage (needed to resume this session)
      if ("session_id" in message) sessionId = message.session_id;
      // Search message content for the agentId (appears in Agent tool results)
      const extractedId = extractAgentId(message);
      if (extractedId) agentId = extractedId;
      // Print the final result
      if ("result" in message) console.log(message.result);
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so sessionId and agentId have already been captured by the loop above.
    console.error(`Session ended with an error: ${error}`);
  }

  // Second invocation - resume and ask follow-up
  if (agentId && sessionId) {
    for await (const message of query({
      prompt: `Resume agent ${agentId} and list the top 3 most complex endpoints`,
      options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents, resume: sessionId }
    })) {
      if ("result" in message) console.log(message.result);
    }
  } else {
    console.log("No agentId found in the first query, so there is no subagent to resume.");
  }
  ```
</CodeGroup>

Le trascrizioni dei subagenti persistono indipendentemente dalla conversazione principale:

* **Compattazione della conversazione principale**: quando la conversazione principale si compatta, le trascrizioni dei subagenti non sono interessate. Sono archiviate in file separati.
* **Persistenza della sessione**: le trascrizioni dei subagenti persistono all'interno della loro sessione. È possibile riprendere un subagente dopo il riavvio di Claude Code riprendendo la stessa sessione.
* **Pulizia automatica**: le trascrizioni vengono pulite in base all'impostazione `cleanupPeriodDays`, che per impostazione predefinita è di 30 giorni.

<h2 id="tool-restrictions-2">
  Restrizioni degli strumenti
</h2>

I subagenti possono avere accesso agli strumenti limitato tramite il campo `tools`:

* **Ometti il campo**: l'agente eredita tutti gli strumenti disponibili (predefinito)
* **Specifica gli strumenti**: l'agente può utilizzare solo gli strumenti elencati

Questo esempio crea un agente di analisi in sola lettura che può esaminare il codice ma non può modificare file o eseguire comandi.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Analyze the architecture of this codebase",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  "code-analyzer": AgentDefinition(
                      description="Static code analysis and architecture review",
                      prompt="""You are a code architecture analyst. Analyze code structure,
  identify patterns, and suggest improvements without making changes.""",
                      # Read-only tools: no Edit, Write, or Bash access
                      tools=["Read", "Grep", "Glob"],
                  )
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Analyze the architecture of this codebase",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        "code-analyzer": {
          description: "Static code analysis and architecture review",
          prompt: `You are a code architecture analyst. Analyze code structure,
  identify patterns, and suggest improvements without making changes.`,
          // Read-only tools: no Edit, Write, or Bash access
          tools: ["Read", "Grep", "Glob"]
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h3 id="common-tool-combinations">
  Combinazioni di strumenti comuni
</h3>

| Caso d'uso              | Strumenti                               | Descrizione                                                        |
| :---------------------- | :-------------------------------------- | :----------------------------------------------------------------- |
| Analisi in sola lettura | `Read`, `Grep`, `Glob`                  | Può esaminare il codice ma non modificare o eseguire               |
| Esecuzione di test      | `Bash`, `Read`, `Grep`                  | Può eseguire comandi e analizzare l'output                         |
| Modifica del codice     | `Read`, `Edit`, `Write`, `Grep`, `Glob` | Accesso completo in lettura/scrittura senza esecuzione di comandi  |
| Accesso completo        | Tutti gli strumenti                     | Eredita tutti gli strumenti dal genitore (ometti il campo `tools`) |

<h2 id="scale-up-with-dynamic-workflows">
  Scalare con flussi di lavoro dinamici
</h2>

I subagenti funzionano bene per alcuni compiti delegati per turno. Per esecuzioni che coordinano dozzine o centinaia di agenti, utilizza lo strumento `Workflow`, che sposta l'orchestrazione in uno script che il runtime esegue al di fuori del contesto della conversazione. Vedi [flussi di lavoro dinamici](/docs/it/workflows) per come i flussi di lavoro differiscono dalla delegazione dei subagenti turno per turno.

Lo strumento `Workflow` è disponibile nell'SDK TypeScript Agent v0.3.149 e versioni successive. Includi `Workflow` in `allowedTools` per approvare automaticamente le esecuzioni dei flussi di lavoro. Gli schemi di input e output dello strumento sono elencati nel [riferimento TypeScript](/docs/it/agent-sdk/typescript#workflow).

<h2 id="troubleshooting">
  Risoluzione dei problemi
</h2>

<h3 id="claude-not-delegating-to-subagents">
  Claude non delega ai subagenti
</h3>

Se Claude completa i compiti direttamente invece di delegare al tuo subagente:

* **Verifica che le invocazioni di Agent siano approvate**: includi `Agent` in `allowedTools` per approvare automaticamente le chiamate ai subagenti. Senza di esso, le invocazioni di Agent passano al tuo callback `canUseTool` oppure, in modalità `dontAsk`, vengono negate
* **Usa prompt espliciti**: menziona il subagente per nome nel tuo prompt, ad esempio "Usa l'agente code-reviewer per..."
* **Scrivi una descrizione chiara**: spiega esattamente quando utilizzare il subagente in modo che Claude possa abbinare i compiti in modo appropriato

<h3 id="filesystem-based-agents-not-loading">
  Agenti basati su file system non caricati
</h3>

Claude Code monitora `~/.claude/agents/` e `.claude/agents/` e rileva un file di agente nuovo o modificato entro pochi secondi, senza necessità di riavvio. Se una definizione non appare mai, esamina queste cause:

* **Nuova directory `agents`**: il monitoraggio copre solo le directory che esistevano all'avvio della sessione, quindi il primo file in una nuova directory richiede un riavvio della sessione. Questa è la causa più comune.
* **Frontmatter non valido o un `name` duplicato**: controlla il YAML del file e se un agente esistente utilizza già il `name`.
* **`--disable-slash-commands`**: le sessioni avviate con questo flag non monitorano queste directory e richiedono sempre un riavvio per caricare i nuovi file.
* **Un agente programmatico con lo stesso nome**: gli `agents` passati a `query()` sovrascrivono un agente del file system con lo stesso nome.

Per il formato del file, vedi [come scrivere file di subagente](/docs/it/sub-agents#write-subagent-files).

<h3 id="long-prompt-failures-on-windows">
  Errori di prompt lungo su Windows
</h3>

Su Windows, i subagenti con prompt molto lunghi potrebbero non riuscire a causa del limite di lunghezza della riga di comando di 8191 caratteri. Mantieni i prompt concisi o utilizza agenti basati su file system per istruzioni complesse.

<h2 id="related-documentation">
  Documentazione correlata
</h2>

* [Subagenti Claude Code](/docs/it/sub-agents): documentazione completa sui subagenti incluse le definizioni basate su file system
* [Flussi di lavoro dinamici](/docs/it/workflows): orchestra molti subagenti da uno script per lavori troppo grandi per una conversazione
* [Panoramica dell'SDK](/docs/it/agent-sdk/overview): introduzione all'SDK Claude Agent
