> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Panoramica dell'Agent SDK

> Costruisci agenti AI di produzione con Claude Code come libreria

Costruisci agenti AI che leggono autonomamente file, eseguono comandi, cercano sul web, modificano codice e molto altro. L'Agent SDK ti offre gli stessi strumenti, il ciclo dell'agente e la gestione del contesto che alimentano Claude Code, programmabili in Python e TypeScript. Per il ragionamento dietro la progettazione dell'harness dell'agente, vedi [A harness for every task: dynamic workflows in Claude Code](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code) sul blog.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      async for message in query(
          prompt="Find and fix the bug in auth.py",
          options=ClaudeAgentOptions(allowed_tools=["Read", "Edit", "Bash"]),
      ):
          print(message)  # Claude reads the file, finds the bug, edits it


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Find and fix the bug in auth.ts",
    options: { allowedTools: ["Read", "Edit", "Bash"] }
  })) {
    console.log(message); // Claude reads the file, finds the bug, edits it
  }
  ```
</CodeGroup>

L'Agent SDK include strumenti integrati per leggere file, eseguire comandi e modificare codice, quindi il tuo agente può iniziare a lavorare immediatamente senza che tu implementi l'esecuzione degli strumenti. Tuffati nella guida rapida o esplora agenti reali costruiti con l'SDK:

<CardGroup cols={2}>
  <Card title="Quickstart" icon="play" href="/docs/it/agent-sdk/quickstart">
    Costruisci un agente di correzione dei bug in pochi minuti
  </Card>

  <Card title="Example agents" icon="star" href="https://github.com/anthropics/claude-agent-sdk-demos">
    Assistente email, agente di ricerca e altro ancora
  </Card>
</CardGroup>

<h2 id="get-started">
  Inizia
</h2>

<Steps>
  <Step title="Installa l'SDK">
    <Tabs>
      <Tab title="TypeScript">
        ```bash theme={null}
        npm install @anthropic-ai/claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (uv)">
        [uv](https://docs.astral.sh/uv/) è un gestore di pacchetti Python veloce che gestisce automaticamente gli ambienti virtuali:

        ```bash theme={null}
        uv init
        uv add claude-agent-sdk
        ```
      </Tab>

      <Tab title="Python (pip)">
        Crea e attiva un ambiente virtuale, quindi installa il pacchetto. L'installazione in un ambiente virtuale evita l'errore `error: externally-managed-environment` che Python di sistema su recenti installazioni Debian, Ubuntu e Homebrew restituisce per `pip install` al di fuori di un venv.

        Su macOS o Linux:

        ```bash theme={null}
        python3 -m venv .venv
        source .venv/bin/activate
        pip install claude-agent-sdk
        ```

        Su Windows:

        ```powershell theme={null}
        py -m venv .venv
        .venv\Scripts\Activate.ps1
        pip install claude-agent-sdk
        ```

        Se PowerShell blocca `Activate.ps1` con un errore di criterio di esecuzione, esegui prima `Set-ExecutionPolicy -Scope Process RemoteSigned`.

        Il pacchetto Python richiede Python 3.10 o versioni successive. Se pip segnala `No matching distribution found for claude-agent-sdk`, il tuo interprete è più vecchio di 3.10. Esegui `python3 --version` su macOS o Linux, oppure `py --version` su Windows, per verificare.
      </Tab>
    </Tabs>

    <Note>
      L'SDK TypeScript raggruppa un binario nativo di Claude Code per la tua piattaforma come dipendenza opzionale, quindi non è necessario installare Claude Code separatamente.
    </Note>
  </Step>

  <Step title="Imposta la tua chiave API">
    Ottieni una chiave API dalla [Console](https://platform.claude.com/), quindi impostala come variabile di ambiente.

    Su macOS o Linux:

    ```bash theme={null}
    export ANTHROPIC_API_KEY=sk-ant-xxxxx
    ```

    Su Windows PowerShell:

    ```powershell theme={null}
    $env:ANTHROPIC_API_KEY = "sk-ant-xxxxx"
    ```

    L'SDK supporta anche l'autenticazione tramite provider API di terze parti:

    * **Amazon Bedrock**: imposta la variabile di ambiente `CLAUDE_CODE_USE_BEDROCK=1` e configura le credenziali AWS
    * **Claude Platform on AWS**: imposta `CLAUDE_CODE_USE_ANTHROPIC_AWS=1` e `ANTHROPIC_AWS_WORKSPACE_ID`, quindi configura le credenziali AWS
    * **Google Cloud's Agent Platform**: imposta la variabile di ambiente `CLAUDE_CODE_USE_VERTEX=1` e configura le credenziali di Google Cloud
    * **Microsoft Azure**: imposta la variabile di ambiente `CLAUDE_CODE_USE_FOUNDRY=1` e configura le credenziali di Azure

    Consulta le guide di configurazione per [Amazon Bedrock](/docs/it/amazon-bedrock), [Claude Platform on AWS](/docs/it/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/it/google-vertex-ai), o [Microsoft Foundry](/docs/it/microsoft-foundry) per i dettagli.

    <Note>
      Se non precedentemente approvato, Anthropic non consente agli sviluppatori di terze parti di offrire l'accesso a claude.ai o limiti di velocità per i loro prodotti, inclusi gli agenti costruiti su Claude Agent SDK. Utilizza invece i metodi di autenticazione con chiave API descritti in questo documento.
    </Note>
  </Step>

  <Step title="Esegui il tuo primo agente">
    Questo esempio crea un agente che elenca i file nella tua directory corrente utilizzando strumenti integrati.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="What files are in this directory?",
              options=ClaudeAgentOptions(allowed_tools=["Bash", "Glob"]),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "What files are in this directory?",
        options: { allowedTools: ["Bash", "Glob"] }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>
  </Step>
</Steps>

**Pronto a costruire?** Segui il [Quickstart](/docs/it/agent-sdk/quickstart) per creare un agente che trova e corregge i bug in pochi minuti.

<h2 id="capabilities">
  Capacità
</h2>

Tutto ciò che rende Claude Code potente è disponibile nell'SDK:

<Tabs>
  <Tab title="Built-in tools">
    Il tuo agente può leggere file, eseguire comandi e cercare codebase subito. Gli strumenti chiave includono:

    | Tool                                                                        | Cosa fa                                                                       |
    | --------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
    | **Read**                                                                    | Leggi qualsiasi file nella directory di lavoro                                |
    | **Write**                                                                   | Crea nuovi file                                                               |
    | **Edit**                                                                    | Apporta modifiche precise ai file esistenti                                   |
    | **Bash**                                                                    | Esegui comandi di terminale, script, operazioni git                           |
    | **Monitor**                                                                 | Osserva uno script in background e reagisci a ogni riga di output come evento |
    | **Glob**                                                                    | Trova file per pattern (`**/*.ts`, `src/**/*.py`)                             |
    | **Grep**                                                                    | Cerca contenuti di file con regex                                             |
    | **WebSearch**                                                               | Cerca sul web informazioni attuali                                            |
    | **WebFetch**                                                                | Recupera e analizza il contenuto della pagina web                             |
    | **[AskUserQuestion](/docs/it/agent-sdk/user-input#handle-clarifying-questions)** | Poni all'utente domande di chiarimento con opzioni a scelta multipla          |

    Questo esempio crea un agente che cerca nella tua codebase i commenti TODO:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Find all TODO comments and create a summary",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Glob", "Grep"]),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Find all TODO comments and create a summary",
        options: { allowedTools: ["Read", "Glob", "Grep"] }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Hooks">
    Esegui codice personalizzato in punti chiave del ciclo di vita dell'agente. Gli hooks dell'SDK utilizzano funzioni di callback per convalidare, registrare, bloccare o trasformare il comportamento dell'agente.

    **Hook disponibili:** `PreToolUse`, `PostToolUse`, `Stop`, `SessionStart`, `SessionEnd`, `UserPromptSubmit` e altri.

    Questo esempio registra tutte le modifiche ai file in un file di audit:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from datetime import datetime
      from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher


      async def log_file_change(input_data, tool_use_id, context):
          file_path = input_data.get("tool_input", {}).get("file_path", "unknown")
          with open("./audit.log", "a") as f:
              f.write(f"{datetime.now()}: modified {file_path}\n")
          return {}


      async def main():
          async for message in query(
              prompt="Refactor utils.py to improve readability",
              options=ClaudeAgentOptions(
                  permission_mode="acceptEdits",
                  hooks={
                      "PostToolUse": [
                          HookMatcher(matcher="Edit|Write", hooks=[log_file_change])
                      ]
                  },
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query, HookCallback } from "@anthropic-ai/claude-agent-sdk";
      import { appendFile } from "fs/promises";

      const logFileChange: HookCallback = async (input) => {
        const filePath = (input as any).tool_input?.file_path ?? "unknown";
        await appendFile("./audit.log", `${new Date().toISOString()}: modified ${filePath}\n`);
        return {};
      };

      for await (const message of query({
        prompt: "Refactor utils.py to improve readability",
        options: {
          permissionMode: "acceptEdits",
          hooks: {
            PostToolUse: [{ matcher: "Edit|Write", hooks: [logFileChange] }]
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Scopri di più su hooks →](/docs/it/agent-sdk/hooks)
  </Tab>

  <Tab title="Subagents">
    Genera agenti specializzati per gestire sottoattività mirate. Il tuo agente principale delega il lavoro e i subagenti riferiscono i risultati.

    Definisci agenti personalizzati con istruzioni specializzate. I subagenti vengono invocati tramite lo strumento Agent, quindi includi `Agent` in `allowedTools` per approvare automaticamente quelle invocazioni:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


      async def main():
          async for message in query(
              prompt="Use the code-reviewer agent to review this codebase",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Glob", "Grep", "Agent"],
                  agents={
                      "code-reviewer": AgentDefinition(
                          description="Expert code reviewer for quality and security reviews.",
                          prompt="Analyze code quality and suggest improvements.",
                          tools=["Read", "Glob", "Grep"],
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
        prompt: "Use the code-reviewer agent to review this codebase",
        options: {
          allowedTools: ["Read", "Glob", "Grep", "Agent"],
          agents: {
            "code-reviewer": {
              description: "Expert code reviewer for quality and security reviews.",
              prompt: "Analyze code quality and suggest improvements.",
              tools: ["Read", "Glob", "Grep"]
            }
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    I messaggi dal contesto di un subagente includono un campo `parent_tool_use_id`, che ti consente di tracciare quali messaggi appartengono a quale esecuzione di subagente.

    [Scopri di più su subagenti →](/docs/it/agent-sdk/subagents)
  </Tab>

  <Tab title="MCP">
    Connettiti a sistemi esterni tramite il Model Context Protocol: database, browser, API e [centinaia di altri](https://github.com/modelcontextprotocol/servers).

    Questo esempio connette il [server Playwright MCP](https://github.com/microsoft/playwright-mcp) per dare al tuo agente capacità di automazione del browser:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Open example.com and describe what you see",
              options=ClaudeAgentOptions(
                  mcp_servers={
                      "playwright": {"command": "npx", "args": ["@playwright/mcp@latest"]}
                  }
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Open example.com and describe what you see",
        options: {
          mcpServers: {
            playwright: { command: "npx", args: ["@playwright/mcp@latest"] }
          }
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Scopri di più su MCP →](/docs/it/agent-sdk/mcp)
  </Tab>

  <Tab title="Permissions">
    Controlla esattamente quali strumenti il tuo agente può utilizzare. Consenti operazioni sicure, blocca quelle pericolose o richiedi approvazione per azioni sensibili.

    <Note>
      Per prompt di approvazione interattivi e lo strumento `AskUserQuestion`, consulta [Gestisci approvazioni e input dell'utente](/docs/it/agent-sdk/user-input).
    </Note>

    Questo esempio crea un agente di sola lettura che può analizzare ma non modificare il codice. `allowed_tools` pre-approva `Read`, `Glob` e `Grep`.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Review this code for best practices",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Glob", "Grep"],
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      for await (const message of query({
        prompt: "Review this code for best practices",
        options: {
          allowedTools: ["Read", "Glob", "Grep"]
        }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Scopri di più su permessi →](/docs/it/agent-sdk/permissions)
  </Tab>

  <Tab title="Sessions">
    Mantieni il contesto su più scambi. Claude ricorda i file letti, l'analisi eseguita e la cronologia della conversazione. Riprendi le sessioni in seguito o dividile per esplorare approcci diversi.

    Questo esempio acquisisce l'ID della sessione dalla prima query, quindi riprende per continuare con il contesto completo:

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage, ResultMessage


      async def main():
          session_id = None

          # First query: capture the session ID
          async for message in query(
              prompt="Read the authentication module",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Glob"]),
          ):
              if isinstance(message, SystemMessage) and message.subtype == "init":
                  session_id = message.data["session_id"]

          # Resume with full context from the first query
          async for message in query(
              prompt="Now find all places that call it",  # "it" = auth module
              options=ClaudeAgentOptions(resume=session_id),
          ):
              if isinstance(message, ResultMessage):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      let sessionId: string | undefined;

      // First query: capture the session ID
      for await (const message of query({
        prompt: "Read the authentication module",
        options: { allowedTools: ["Read", "Glob"] }
      })) {
        if (message.type === "system" && message.subtype === "init") {
          sessionId = message.session_id;
        }
      }

      // Resume with full context from the first query
      for await (const message of query({
        prompt: "Now find all places that call it", // "it" = auth module
        options: { resume: sessionId }
      })) {
        if ("result" in message) console.log(message.result);
      }
      ```
    </CodeGroup>

    [Scopri di più su sessioni →](/docs/it/agent-sdk/sessions)
  </Tab>
</Tabs>

<h3 id="claude-code-features">
  Funzionalità di Claude Code
</h3>

L'SDK supporta anche la configurazione basata su filesystem di Claude Code. Con le opzioni predefinite, l'SDK carica questi da `.claude/` nella tua directory di lavoro e `~/.claude/`. Per limitare quali fonti caricare, imposta `setting_sources` (Python) o `settingSources` (TypeScript) nelle tue opzioni.

| Funzionalità                                     | Descrizione                                                                                 | Posizione                               |
| ------------------------------------------------ | ------------------------------------------------------------------------------------------- | --------------------------------------- |
| [Skills](/docs/it/agent-sdk/skills)                   | Capacità specializzate che Claude utilizza automaticamente o che invochi con `/name`        | `.claude/skills/*/SKILL.md`             |
| [Commands](/docs/it/agent-sdk/slash-commands)         | Comandi personalizzati nel formato legacy. Utilizza skills per nuovi comandi personalizzati | `.claude/commands/*.md`                 |
| [Memory](/docs/it/agent-sdk/modifying-system-prompts) | Contesto del progetto e istruzioni                                                          | `CLAUDE.md` o `.claude/CLAUDE.md`       |
| [Plugins](/docs/it/agent-sdk/plugins)                 | Estendi con skills, agenti, hooks e server MCP                                              | Programmatico tramite opzione `plugins` |

<h2 id="compare-the-agent-sdk-to-other-claude-tools">
  Confronta l'Agent SDK con altri strumenti Claude
</h2>

La piattaforma Claude offre più modi per costruire con Claude. Ecco come si inserisce l'Agent SDK:

<Tabs>
  <Tab title="Agent SDK vs Client SDK">
    L'[Anthropic Client SDK](https://platform.claude.com/docs/it/api/client-sdks) ti offre accesso diretto all'API: invii prompt e implementi tu stesso l'esecuzione degli strumenti. L'**Agent SDK** ti offre Claude con esecuzione degli strumenti integrata.

    Con il Client SDK, implementi un ciclo di strumenti. Con l'Agent SDK, Claude lo gestisce:

    <CodeGroup>
      ```python Python theme={null}
      # Client SDK: You implement the tool loop
      response = client.messages.create(...)
      while response.stop_reason == "tool_use":
          result = your_tool_executor(response.tool_use)
          response = client.messages.create(tool_result=result, **params)

      # Agent SDK: Claude handles tools autonomously
      async for message in query(prompt="Fix the bug in auth.py"):
          print(message)
      ```

      ```typescript TypeScript theme={null}
      // Client SDK: You implement the tool loop
      let response = await client.messages.create({ ...params });
      while (response.stop_reason === "tool_use") {
        const result = yourToolExecutor(response.tool_use);
        response = await client.messages.create({ tool_result: result, ...params });
      }

      // Agent SDK: Claude handles tools autonomously
      for await (const message of query({ prompt: "Fix the bug in auth.ts" })) {
        console.log(message);
      }
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Agent SDK vs Claude Code CLI">
    Stesse capacità, interfaccia diversa:

    | Caso d'uso                  | Scelta migliore |
    | --------------------------- | --------------- |
    | Sviluppo interattivo        | CLI             |
    | Pipeline CI/CD              | SDK             |
    | Applicazioni personalizzate | SDK             |
    | Attività una tantum         | CLI             |
    | Automazione di produzione   | SDK             |

    Molti team utilizzano entrambi: CLI per lo sviluppo quotidiano, SDK per la produzione. I flussi di lavoro si traducono direttamente tra loro.
  </Tab>

  <Tab title="Agent SDK vs Managed Agents">
    [Managed Agents](https://platform.claude.com/docs/it/managed-agents/overview) è un'API REST ospitata: Anthropic esegue l'agente e la sandbox, e la tua applicazione invia eventi e riceve i risultati in streaming. L'**Agent SDK** è una libreria che esegue il ciclo dell'agente all'interno del tuo processo.

    |                              | Agent SDK                                                                            | Managed Agents                                                                                               |
    | ---------------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------ |
    | **Viene eseguito in**        | Il tuo processo, la tua infrastruttura                                               | Infrastruttura gestita da Anthropic                                                                          |
    | **Interfaccia**              | Libreria Python o TypeScript                                                         | API REST                                                                                                     |
    | **L'agente lavora su**       | File sulla tua infrastruttura                                                        | Una sandbox gestita per sessione                                                                             |
    | **Stato della sessione**     | JSONL sul tuo filesystem                                                             | Log degli eventi ospitato da Anthropic                                                                       |
    | **Strumenti personalizzati** | Funzioni Python o TypeScript in-process                                              | Claude attiva lo strumento; tu esegui e restituisci i risultati                                              |
    | **Ideale per**               | Prototipazione locale, agenti che lavorano direttamente sul tuo filesystem e servizi | Agenti di produzione senza gestire infrastruttura di sandbox o sessione, sessioni a lunga durata e asincrone |

    Un percorso comune è prototipare con l'Agent SDK localmente, quindi passare a Managed Agents per la produzione.
  </Tab>
</Tabs>

<h2 id="changelog">
  Changelog
</h2>

Visualizza il changelog completo per gli aggiornamenti dell'SDK, le correzioni di bug e le nuove funzionalità:

* **TypeScript SDK**: [visualizza CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md)
* **Python SDK**: [visualizza CHANGELOG.md](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md)

<h2 id="reporting-bugs">
  Segnalazione di bug
</h2>

Se riscontri bug o problemi con l'Agent SDK:

* **TypeScript SDK**: [segnala i problemi su GitHub](https://github.com/anthropics/claude-agent-sdk-typescript/issues)
* **Python SDK**: [segnala i problemi su GitHub](https://github.com/anthropics/claude-agent-sdk-python/issues)

<h2 id="branding-guidelines">
  Linee guida di branding
</h2>

Per i partner che integrano Claude Agent SDK, l'uso del branding Claude è facoltativo. Quando fai riferimento a Claude nel tuo prodotto:

**Consentito:**

* "Claude Agent" (preferito per i menu a discesa)
* "Claude" (quando già all'interno di un menu etichettato "Agents")
* "{YourAgentName} Powered by Claude" (se hai un nome di agente esistente)

**Non consentito:**

* "Claude Code" o "Claude Code Agent"
* Arte ASCII con branding Claude Code o elementi visivi che imitano Claude Code

Il tuo prodotto dovrebbe mantenere il suo proprio branding e non sembrare Claude Code o alcun prodotto Anthropic. Per domande sulla conformità del branding, contatta il [team di vendita](https://www.anthropic.com/contact-sales) di Anthropic.

<h2 id="license-and-terms">
  Licenza e termini
</h2>

L'uso di Claude Agent SDK è disciplinato dai [Termini di servizio commerciali di Anthropic](https://www.anthropic.com/legal/commercial-terms), incluso quando lo utilizzi per alimentare prodotti e servizi che metti a disposizione dei tuoi clienti e utenti finali, tranne nella misura in cui un componente o una dipendenza specifica è coperta da una licenza diversa come indicato nel file LICENSE di quel componente.

<h2 id="next-steps">
  Passaggi successivi
</h2>

<CardGroup cols={2}>
  <Card title="Quickstart" icon="play" href="/docs/it/agent-sdk/quickstart">
    Costruisci un agente che trova e corregge i bug in pochi minuti
  </Card>

  <Card title="Example agents" icon="star" href="https://github.com/anthropics/claude-agent-sdk-demos">
    Assistente email, agente di ricerca e altro ancora
  </Card>

  <Card title="TypeScript SDK" icon="code" href="/docs/it/agent-sdk/typescript">
    Riferimento API TypeScript completo ed esempi
  </Card>

  <Card title="Python SDK" icon="code" href="/docs/it/agent-sdk/python">
    Riferimento API Python completo ed esempi
  </Card>
</CardGroup>
