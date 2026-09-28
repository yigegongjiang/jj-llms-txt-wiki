> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Subagents nell'SDK

> Definisci e richiama subagenti per isolare il contesto, eseguire attività in parallelo e applicare istruzioni specializzate nelle tue applicazioni Claude Agent SDK.

I subagenti sono istanze di agenti separate che il tuo agente principale può generare per gestire sottoattività mirate.
Usali per isolare il contesto, eseguire più analisi in parallelo e applicare istruzioni specializzate senza aggiungere al prompt dell'agente principale.

<h2 id="overview">
  Panoramica
</h2>

È possibile creare subagent in tre modi:

* **A livello programmatico**: utilizzare il parametro `agents` nelle opzioni di `query()`. Consultare i riferimenti [TypeScript](/docs/it/agent-sdk/typescript#agentdefinition) e [Python](/docs/it/agent-sdk/python#agentdefinition)
* **Basato sul file system**: definire gli agenti come file markdown nelle directory `.claude/agents/`. Consultare [definizione di subagent come file](/docs/it/sub-agents)
* **Generale integrato**: Claude può invocare il subagent `general-purpose` integrato in qualsiasi momento tramite lo strumento Agent senza che sia necessario definire nulla

Questa guida si concentra sull'approccio programmatico, consigliato per le applicazioni SDK.

<h2 id="benefits-of-using-subagents">
  Vantaggi dell'utilizzo di subagent
</h2>

Poiché i subagent sono istanze di agente separate, delegare il lavoro a loro offre quattro vantaggi:

* **Isolamento del contesto**: ogni subagent viene eseguito nella propria conversazione, che inizia da zero a meno che il subagent non sia un [fork](/docs/it/sub-agents#fork-the-current-conversation). In ogni caso, le chiamate agli strumenti intermedi e i risultati rimangono all'interno del subagent; solo il suo messaggio finale ritorna al genitore. Un subagent `research-assistant` può esplorare dozzine di file senza che nessuno di questi contenuti si accumuli nella conversazione principale. Il genitore riceve un riassunto conciso, non ogni file che il subagent ha letto. Vedere [What subagents inherit](#what-subagents-inherit) per sapere esattamente cosa c'è nel contesto del subagent.
* **Parallelizzazione**: più subagent possono essere eseguiti contemporaneamente, quindi i sottocompiti indipendenti si completano nel tempo di quello più lento piuttosto che nella somma di tutti loro. Durante una revisione del codice, è possibile eseguire i subagent `style-checker`, `security-scanner` e `test-coverage` simultaneamente invece che sequenzialmente.
* **Istruzioni e conoscenze specializzate**: ogni subagent può avere un prompt di sistema personalizzato con competenze specifiche, best practice e vincoli. Un subagent `database-migration` può avere conoscenze dettagliate sulle best practice SQL, strategie di rollback e controlli di integrità dei dati che sarebbero rumore inutile nelle istruzioni dell'agente principale.
* **Restrizioni degli strumenti**: i subagent possono essere limitati a strumenti specifici, riducendo il rischio di azioni indesiderate. Un subagent `doc-reviewer` potrebbe avere accesso solo ai tool Read e Grep, assicurando che possa analizzare ma non modifichi mai accidentalmente i file di documentazione.

<h2 id="create-subagents">
  Creare subagent
</h2>

<h3 id="programmatic-definition-recommended">
  Definizione programmatica (consigliata)
</h3>

Definisci i subagent direttamente nel tuo codice utilizzando il parametro `agents`. Claude invoca i subagent attraverso lo strumento `Agent`.

La maggior parte degli esempi in questa pagina stampa solo il risultato finale. Per confermare che Claude ha delegato a un subagent piuttosto che rispondere direttamente, vedi [Rilevare l'invocazione di subagent](#detect-subagent-invocation).

Questo esempio crea due subagent: un revisore di codice con accesso in sola lettura e un esecutore di test che può eseguire comandi.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Review the authentication module for security issues",
          options=ClaudeAgentOptions(
              # Auto-approve these tools
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
      // Auto-approve these tools
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

| Campo             | Tipo                                                        | Obbligatorio | Descrizione                                                                                                                                                                                                                                                                                                                                                                                         |
| :---------------- | :---------------------------------------------------------- | :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`     | `string`                                                    | Sì           | Descrizione in linguaggio naturale di quando utilizzare questo agente                                                                                                                                                                                                                                                                                                                               |
| `prompt`          | `string`                                                    | Sì           | Il prompt di sistema dell'agente che definisce il suo ruolo e comportamento                                                                                                                                                                                                                                                                                                                         |
| `tools`           | `string[]`                                                  | No           | Array di nomi di strumenti consentiti. Se omesso, eredita ogni [strumento disponibile per i subagent](/docs/it/sub-agents#available-tools)                                                                                                                                                                                                                                                               |
| `disallowedTools` | `string[]`                                                  | No           | Array di nomi di strumenti da rimuovere dal set di strumenti dell'agente. Sono accettati anche pattern a livello di server MCP: `mcp__server` o `mcp__server__*` rimuove ogni strumento da quel server, e `mcp__*` rimuove ogni strumento MCP da qualsiasi server                                                                                                                                   |
| `model`           | `string`                                                    | No           | Override del modello per questo agente. Accetta un alias come `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, o un ID modello completo. `'inherit'` utilizza il modello principale. Quando lo ometti, Claude Code sceglie il modello nell'[ordine del modello di subagent](/docs/it/sub-agents#choose-a-model)                                                                                  |
| `skills`          | `string[]`                                                  | No           | Elenco di nomi di skill da precaricare nel contesto dell'agente all'avvio. Le skill non elencate rimangono invocabili attraverso lo strumento Skill                                                                                                                                                                                                                                                 |
| `memory`          | `'user' \| 'project' \| 'local'`                            | No           | Fonte di memoria per questo agente                                                                                                                                                                                                                                                                                                                                                                  |
| `mcpServers`      | `(string \| object)[]`                                      | No           | Server MCP disponibili per questo agente, per nome o configurazione inline                                                                                                                                                                                                                                                                                                                          |
| `initialPrompt`   | `string`                                                    | No           | Inviato automaticamente come primo turno utente quando questo agente viene eseguito come agente del thread principale. Ignorato quando l'agente viene invocato come subagent                                                                                                                                                                                                                        |
| `maxTurns`        | `number`                                                    | No           | Numero massimo di turni agentici prima che l'agente si fermi. Quando l'agente raggiunge il limite, Claude Code restituisce il suo output contrassegnato come parziale, e puoi [riprendere l'agente](#resume-subagents) per continuare. Il contrassegno parziale richiede Claude Code v2.1.246 o successivo                                                                                          |
| `background`      | `boolean`                                                   | No           | Esegui questo agente come attività di background non bloccante quando invocato                                                                                                                                                                                                                                                                                                                      |
| `omitClaudeMd`    | `boolean`                                                   | No           | Esegui questo agente senza i file CLAUDE.md dell'utente, del progetto e locali quando viene eseguito come subagent; i file di policy gestiti vengono comunque caricati. Ignorato quando l'agente viene eseguito come agente del thread principale. Richiede TypeScript Agent SDK v0.3.271 o successivo. Il Python SDK [`AgentDefinition`](/docs/it/agent-sdk/python#agentdefinition) non ha questo campo |
| `effort`          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max' \| number` | No           | Livello di sforzo di ragionamento per questo agente                                                                                                                                                                                                                                                                                                                                                 |
| `permissionMode`  | `PermissionMode`                                            | No           | Modalità di permesso per l'esecuzione dello strumento all'interno di questo agente. Le [regole di ereditarietà dei subagent](/docs/it/agent-sdk/permissions#available-modes) decidono quando si applica                                                                                                                                                                                                  |

In Python SDK, i nomi di campo con più parole come `disallowedTools` e `mcpServers` mantengono la loro ortografia camelCase per corrispondere al formato wire piuttosto che seguire la convenzione snake\_case di Python. Vedi il riferimento [`AgentDefinition`](/docs/it/agent-sdk/python#agentdefinition) per i dettagli.

I subagent vengono eseguiti in background per impostazione predefinita. Una chiamata dello strumento Agent che omette l'input [`run_in_background`](/docs/it/sub-agents#run-subagents-in-foreground-or-background) avvia un subagent di background, e Claude imposta `run_in_background: false` quando ha bisogno del risultato prima di continuare. Imposta il campo `background` su `true` per forzare l'esecuzione in background per un agente specifico indipendentemente da ciò che Claude richiede. Prima di Claude Code v2.1.198, l'impostazione predefinita di background era in fase di implementazione graduale, e una chiamata dello strumento Agent che ometteva `run_in_background` poteva eseguire il subagent in modo sincrono.

I subagent possono anche generare subagent propri. Per limitare quanto profonda sia quella nidificazione, quanti subagent vengono eseguiti contemporaneamente e quanto una query spende, vedi [Limitare la profondità, la concorrenza e la spesa dei subagent](#cap-subagent-depth-concurrency-and-spend).

<h3 id="filesystem-based-definition-alternative">
  Definizione basata su filesystem (alternativa)
</h3>

Puoi anche definire i subagent come file markdown nelle directory `.claude/agents/`. Vedi la [documentazione dei subagent di Claude Code](/docs/it/sub-agents) per i dettagli su questo approccio. Gli agenti definiti programmaticamente hanno la precedenza sugli agenti basati su filesystem con lo stesso nome.

<Note>
  Quando Claude chiama lo strumento Agent senza un `subagent_type`, ottiene il subagent `general-purpose` integrato, che Claude può generare anche quando non definisci agenti tuoi. Impostando [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/it/env-vars) rimuovi quel valore predefinito, e tale chiamata fallisce con [`subagent_type is required`](/docs/it/errors#subagent-type-is-required).
</Note>

<h2 id="what-subagents-inherit">
  Cosa ereditano i subagent
</h2>

A meno che il subagent non sia un [fork](/docs/it/sub-agents#fork-the-current-conversation), la sua finestra di contesto inizia da zero, senza la conversazione padre, ma non è vuota. L'unico contenuto che trasmetti dal padre al subagent è la stringa di prompt dello strumento Agent, quindi includi direttamente in quel prompt tutti i percorsi di file, i messaggi di errore o le decisioni di cui il subagent ha bisogno.

Un subagent che dispone dello strumento [`SendMessage`](/docs/it/tools-reference) inizia con un elenco degli altri agenti denominati in esecuzione nella sessione, quindi sa quali nomi può utilizzare per inviare messaggi. Claude Code aggiunge automaticamente l'elenco al primo turno del subagent. Un [fork](/docs/it/sub-agents#fork-the-current-conversation) non riceve l'elenco perché eredita invece la conversazione padre.

Un subagent eredita anche la configurazione del pensiero esteso della sessione principale.

La tabella seguente elenca ciò che il contesto di un subagent non-fork contiene e ciò che omette.

| Il subagent riceve                                                                                                                                                                                                            | Il subagent non riceve                                                                  |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------- |
| Il suo prompt di sistema (`AgentDefinition.prompt`) e il prompt dello strumento Agent                                                                                                                                         | La cronologia della conversazione padre o i risultati degli strumenti                   |
| Project CLAUDE.md (caricato tramite [`settingSources`](/docs/it/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)), a meno che l'agente non imposti [`omitClaudeMd`](#agentdefinition-configuration) | Contenuto di skill precaricato, a meno che non sia elencato in `AgentDefinition.skills` |
| Definizioni degli strumenti (ereditate dal padre o il sottoinsieme in `tools`, [filtrate per esecuzioni in background](/docs/it/sub-agents#available-tools))                                                                       | Il prompt di sistema del padre                                                          |

<Note>
  Il padre riceve il messaggio finale del subagent come risultato dello strumento Agent, ma potrebbe riassumerlo nella sua risposta. Per preservare l'output del subagent verbatim nella risposta rivolta all'utente, includi un'istruzione per farlo nel prompt o nell'opzione `systemPrompt` che passi alla chiamata principale `query()`.

  Nella versione 2.1.210 e successive, Claude Code [scansiona il messaggio finale per modelli a forma di istruzione](/docs/it/sub-agents#subagent-output-scanning) prima che il padre lo legga. La scansione tratta tre tipi di modello diversamente:

  * **Imitazione di tag di controllo**: Claude Code neutralizza un tag che solo l'harness emette, come un blocco `<system-reminder>`, sul posto. Inserisce una barra rovesciata dopo la parentesi angolare di apertura e non elimina nulla.
  * **Menzioni di configurazione delle autorizzazioni**: Claude Code mantiene i riferimenti alla configurazione delle autorizzazioni, come `.claude/settings.json`, `bypassPermissions`, o `--dangerously-skip-permissions`, come scritti.
  * **Marcatori di turno**: una riga che inizia con `Human:` o `Assistant:` riceve una barra rovesciata prima dei due punti, in modo che il messaggio non possa imitare un confine di turno di conversazione.

  Per una corrispondenza di tag di controllo o configurazione delle autorizzazioni, Claude Code antepone una riga marcatore `[harness: ...]` che nomina i modelli corrispondenti; una corrispondenza di marcatore di turno non aggiunge la riga marcatore. Queste sono le uniche modifiche che la scansione apporta: non rimuove mai o riformula il testo del subagent.
</Note>

Un errore API che termina il subagent anticipatamente, come un limite di velocità, non viene mai consegnato come suo risultato. Vedi [Errori API nei subagent](/docs/it/sub-agents#api-errors-in-subagents) per il comportamento in primo piano e in background.

<h2 id="invoke-subagents">
  Invocare subagenti
</h2>

<h3 id="automatic-invocation">
  Invocazione automatica
</h3>

Claude decide automaticamente quando invocare i subagenti in base al compito e alla `description` di ogni subagente. Ad esempio, se definisci un subagente `performance-optimizer` con la descrizione "Performance optimization specialist for query tuning", Claude lo invocherà quando il tuo prompt menziona l'ottimizzazione delle query.

Scrivi descrizioni chiare e specifiche in modo che Claude possa abbinare i compiti al subagente giusto.

<h3 id="explicit-invocation">
  Invocazione esplicita
</h3>

Per garantire che Claude utilizzi un subagente specifico, menzionalo per nome nel tuo prompt:

```text theme={null}
"Use the code-reviewer agent to check the authentication module"
```

Questo bypassa l'abbinamento automatico e invoca direttamente il subagente denominato.

<h3 id="dynamic-agent-configuration">
  Configurazione dinamica dell'agente
</h3>

Puoi creare definizioni di agenti dinamicamente in base alle condizioni di runtime. Questo esempio crea un revisore di sicurezza con diversi livelli di rigore, utilizzando un modello più capace per revisioni rigorose.

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
  Rilevare l'invocazione di subagent
</h2>

Claude invoca i subagent tramite lo strumento Agent. Per rilevare quando un subagent viene invocato, verificare i blocchi `tool_use` dove `name` è `"Agent"`. I messaggi provenienti dal contesto di un subagent includono un campo `parent_tool_use_id`.

<Note>
  Lo strumento appare come `"Agent"` nei blocchi `tool_use` ma come `"Task"` nell'elenco degli strumenti `system:init`. Prima di Claude Code v2.1.63, i blocchi `tool_use` lo denominano anche `"Task"`. Per mantenere il rilevamento funzionante tra le versioni dell'SDK, abbinare entrambi i valori in `block.name`.
</Note>

La struttura del messaggio differisce tra gli SDK. In Python, si accede ai blocchi di contenuto direttamente tramite `message.content`. In TypeScript, `SDKAssistantMessage` racchiude il messaggio dell'API Claude, quindi si accede al contenuto tramite `message.message.content`.

Questo esempio itera attraverso i messaggi in streaming, registrando quando un subagent viene invocato e quando i messaggi successivi provengono dal contesto di esecuzione di quel subagent.

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
  Riprendere i subagent
</h2>

È possibile riprendere un subagent per continuare da dove si era fermato piuttosto che ricominciare da capo. Un subagent ripreso mantiene la sua intera cronologia di conversazione, incluse tutte le chiamate agli strumenti precedenti, i risultati e il ragionamento.

Quando un subagent si ferma al limite di [`maxTurns`](#agentdefinition-configuration), Claude Code contrassegna l'output nel risultato dello strumento Agent come parziale, in modo che Claude sappia che l'esecuzione è incompleta.

Quando un subagent si completa, il risultato dello strumento Agent include un blocco di testo contenente `agentId: <id>`. Gli agenti [`Explore` e `Plan`](/docs/it/sub-agents#built-in-subagents) integrati sono monouso e non restituiscono un `agentId`, quindi utilizzare un agente personalizzato o `general-purpose` quando è necessario riprendere. Per riprendere un subagent a livello di programmazione:

1. **Acquisire l'ID della sessione**: estrarre `session_id` dai messaggi durante la prima query
2. **Estrarre l'ID dell'agente**: analizzare `agentId` dal testo del risultato dello strumento Agent
3. **Riprendere la sessione**: passare `resume: sessionId` nelle opzioni della seconda query e includere l'ID dell'agente nel prompt. Ogni chiamata `query()` avvia una nuova sessione per impostazione predefinita ed è necessario riprendere la stessa sessione per accedere alla trascrizione del subagent.

<Note>
  Quando si utilizza un agente personalizzato, passare la stessa definizione di agente nel parametro `agents` per entrambe le query.
</Note>

L'esempio seguente definisce un agente personalizzato `endpoint-finder`. La prima query lo esegue e acquisisce l'ID della sessione e l'ID dell'agente dal risultato dello strumento Agent, quindi la seconda query riprende la sessione per porre una domanda di follow-up che richiede il contesto della prima analisi.

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

Le trascrizioni dei subagent sono archiviate in file separati e persistono indipendentemente dalla conversazione principale. Vedere [riprendere i subagent in Claude Code](/docs/it/sub-agents#resume-subagents) per il comportamento di compattazione e il periodo di pulizia `cleanupPeriodDays`.

<h2 id="tool-restrictions">
  Restrizioni degli strumenti
</h2>

Utilizzare il campo `tools` per limitare ciò che un subagent può fare:

* **Omettere `tools`**: il subagent ottiene ogni [strumento disponibile per i subagent](/docs/it/sub-agents#available-tools)
* **Elencare gli strumenti**: il subagent ottiene solo quelli. Ad esempio, un revisore di codice che non dovrebbe mai modificare file ottiene `["Read", "Grep", "Glob"]`

Uno strumento che si omette non è affatto nella sessione del subagent: Claude funziona senza di esso, senza alcun prompt di autorizzazione o errore.

Questo esempio crea un agente di analisi di sola lettura che può esaminare il codice ma non può modificare file o eseguire comandi.

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
  Combinazioni comuni di strumenti
</h3>

| Caso d'uso              | Strumenti                               | Descrizione                                                                  |
| :---------------------- | :-------------------------------------- | :--------------------------------------------------------------------------- |
| Analisi di sola lettura | `Read`, `Grep`, `Glob`                  | Può esaminare il codice ma non modificare o eseguire                         |
| Esecuzione di test      | `Bash`, `Read`, `Grep`                  | Può eseguire comandi e analizzare l'output                                   |
| Modifica del codice     | `Read`, `Edit`, `Write`, `Grep`, `Glob` | Accesso completo in lettura/scrittura senza esecuzione di comandi            |
| Accesso completo        | Tutti gli strumenti                     | Eredita gli strumenti disponibili per i subagent (omettere il campo `tools`) |

<h2 id="cap-subagent-depth-concurrency-and-spend">
  Limitare la profondità, la concorrenza e la spesa dei subagent
</h2>

<Note>
  Questa sezione descrive TypeScript SDK v0.3.219 e Python SDK v0.2.127 e versioni successive, i rilasci che includono Claude Code v2.1.219 o versioni successive. Nei rilasci precedenti, alcuni di questi limiti sono assenti o hanno valori predefiniti diversi, quindi eseguire l'aggiornamento prima di fare affidamento su di essi per limitare un'esecuzione. Il [riferimento alle variabili di ambiente](/docs/it/env-vars) e [turni e budget](/docs/it/agent-sdk/agent-loop#turns-and-budget) registrano la versione di Claude Code che ha aggiunto ogni variabile e l'applicazione del limite di spesa del subagent.
</Note>

Claude decide autonomamente quando generare un subagent e quanti generare. Ogni subagent effettua le proprie richieste API, che contano verso il `total_cost_usd` della query, e un subagent può generare subagent propri, quindi un prompt può trasformarsi in un albero di agenti.

È possibile limitare questa crescita in tre modi: quanto profondamente i subagent si annidano, quanti vengono eseguiti contemporaneamente e quanto spende l'intera query. Impostare i limiti di profondità e concorrenza come variabili di ambiente tramite l'opzione [`env`](/docs/it/agent-sdk/typescript#options) e il limite di spesa come opzione di query:

| Limite      | Impostarlo con                                           | Predefinito                                                                                                     | Cosa fa Claude Code al limite                                                                                                                                                                                                                                                                                                                           |
| :---------- | :------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Profondità  | [`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`](/docs/it/env-vars)   | `3` livelli di subagent sotto l'agente principale. `1` impedisce ai subagent di generare altri subagent         | Lascia un subagent al livello inferiore incapace di generare, quindi esegue il lavoro delegato da solo. Vedere [subagent annidati](/docs/it/sub-agents#let-subagents-spawn-their-own-subagents)                                                                                                                                                              |
| Concorrenza | [`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`](/docs/it/env-vars)   | `20` subagent in esecuzione contemporaneamente, contando ogni subagent che Claude genera con lo strumento Agent | Rifiuta di generare un altro subagent, restituendo `Concurrent subagent limit reached`, finché il conteggio in esecuzione non scende al di sotto del limite. Le sessioni con [ultracode](/docs/it/model-config#adjust-effort-level) attivo non vengono mai rifiutate. Vedere il [limite di subagent concorrenti](/docs/it/sub-agents#concurrent-subagent-limit)   |
| Spesa       | `maxBudgetUsd` in TypeScript, `max_budget_usd` in Python | Nessun limite. Conta la spesa della chiamata, incluse le richieste dei subagent                                 | Applica il limite in tre modi: rifiuta di generare più subagent, restituendo `Budget limit reached`, arresta i subagent in background ancora in esecuzione e termina la query con il sottotipo di risultato `error_max_budget_usd`. Per il comportamento dei limiti in una sessione, vedere [turni e budget](/docs/it/agent-sdk/agent-loop#turns-and-budget) |

I due SDK trattano l'opzione `env` diversamente: TypeScript SDK sostituisce l'ambiente del sottoprocesso con esso, quindi diffondere `process.env` in esso per mantenere variabili come `PATH`, mentre Python SDK lo unisce all'ambiente ereditato. Questo esempio disattiva l'annidamento, consente al massimo cinque subagent alla volta e arresta la query una volta che la spesa stimata raggiunge \$5:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      try:
          async for message in query(
              prompt="Audit every service in this repo for unhandled promise rejections",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Grep", "Glob", "Agent"],
                  # env is merged on top of the inherited environment
                  env={
                      "CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH": "1",
                      "CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS": "5",
                  },
                  max_budget_usd=5.0,
              ),
          ):
              if isinstance(message, ResultMessage):
                  print(f"{message.subtype}: ${message.total_cost_usd}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so the budget-capped result has already been printed above.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Audit every service in this repo for unhandled promise rejections",
      options: {
        allowedTools: ["Read", "Grep", "Glob", "Agent"],
        // env replaces the subprocess environment, so spread process.env to keep PATH
        env: {
          ...process.env,
          CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH: "1",
          CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS: "5",
        },
        maxBudgetUsd: 5,
      },
    })) {
      if (message.type === "result") {
        console.log(`${message.subtype}: $${message.total_cost_usd}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so the budget-capped result has already been logged above.
    console.error(`Session ended with an error: ${error}`);
  }
  ```
</CodeGroup>

Quello che vedete dipende da quale limite, se presente, raggiunge la query:

* **Sotto il limite di spesa**: vedete `success` e il costo stimato.
* **Al limite di spesa**: vedete `error_max_budget_usd` con un costo pari o superiore a `5`, e quindi il gestore degli errori viene eseguito.
* **Al limite di concorrenza**: vedete un blocco `tool_result` nel flusso di messaggi che contiene `Concurrent subagent limit reached`. Claude riceve lo stesso blocco come risultato dello strumento Agent.

<h3 id="run-opus-5-with-subagents">
  Eseguire Opus 5 con subagent
</h3>

Claude Opus 5 delega ai subagent più prontamente rispetto ai modelli precedenti, quindi i [limiti di profondità, concorrenza e spesa](#cap-subagent-depth-concurrency-and-spend) sono più importanti nelle query che eseguono Opus 5. La [guida al prompting di Opus 5](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-opus-5#controlling-subagent-spawning) contiene un'istruzione di delega che è possibile aggiungere a qualsiasi prompt. Se Claude Code aggiunge un'istruzione propria dipende da quale [system prompt](/docs/it/agent-sdk/modifying-system-prompts#how-system-prompts-work) utilizzate:

* **Preset `claude_code`**: quando il modello è Opus 5, Claude Code aggiunge una riga al suo system prompt dicendo a Claude di non chiamare lo strumento Agent a meno che non gli venga chiesto. Lo strumento Agent rimane disponibile.
* **Un prompt personalizzato, o nessun `systemPrompt`**: Claude Code non costruisce il suo system prompt, quindi quella riga è assente. Aggiungete l'istruzione di delega della guida al prompting al vostro prompt.

Entrambe le istruzioni solo indirizzano Claude, quindi impostate anche i limiti. Claude Code li applica comunque in base a come Claude decide di delegare.

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

* **Usa prompt espliciti**: menziona il subagente per nome nel tuo prompt, ad esempio "Usa l'agente code-reviewer per controllare il modulo di autenticazione"
* **Scrivi una descrizione chiara**: spiega esattamente quando utilizzare il subagente in modo che Claude possa abbinare i compiti in modo appropriato

<h3 id="filesystem-based-agents-not-loading">
  Agenti basati su file system non caricati
</h3>

Claude Code monitora `~/.claude/agents/` e `.claude/agents/` e rileva un file di agente nuovo o modificato entro pochi secondi, senza necessità di riavvio. Se una definizione non appare mai, esamina queste cause:

* **Nuova directory `agents`**: il monitoraggio copre solo le directory che esistevano all'avvio della sessione, quindi il primo file in una nuova directory richiede un riavvio della sessione. Questa è la causa più comune.
* **Frontmatter non valido o un `name` duplicato**: controlla il YAML del file e se un agente esistente utilizza già il `name`.
* **`--disable-slash-commands`**: le sessioni avviate con questo flag non monitorano queste directory e richiedono sempre un riavvio per caricare i nuovi file.
* **Un file in una directory aggiunta**: Claude Code carica `.claude/agents/` dalle directory aggiunte con l'opzione `add_dirs` (Python) o `additionalDirectories` (TypeScript), oppure con `--add-dir` o `/add-dir` della CLI, ma non le monitora, quindi un file nuovo o modificato lì richiede un riavvio della sessione.
* **Un agente programmatico con lo stesso nome**: gli `agents` passati a `query()` sovrascrivono un agente del file system con lo stesso nome.

Per il formato del file, vedi [come scrivere file di subagente](/docs/it/sub-agents#write-subagent-files).

<h2 id="related-documentation">
  Documentazione correlata
</h2>

* [Subagenti Claude Code](/docs/it/sub-agents): documentazione completa sui subagenti incluse le definizioni basate su file system
* [Flussi di lavoro dinamici](/docs/it/workflows): orchestra molti subagenti da uno script per lavori troppo grandi per una conversazione
* [Panoramica dell'SDK](/docs/it/agent-sdk/overview): introduzione all'SDK Claude Agent
