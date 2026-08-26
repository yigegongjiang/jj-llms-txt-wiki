> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Usa le funzionalità di Claude Code nell'SDK

> Carica le istruzioni del progetto, le skills, gli hooks e altre funzionalità di Claude Code nei tuoi agenti SDK.

L'Agent SDK è costruito sulla stessa base di Claude Code, il che significa che i tuoi agenti SDK hanno accesso alle stesse funzionalità basate sul filesystem: istruzioni del progetto (`CLAUDE.md` e regole), skills, hooks e altro ancora.

Quando ometti `settingSources`, `query()` legge le stesse impostazioni del filesystem di Claude Code CLI: impostazioni utente, progetto e locali, file `CLAUDE.md` e skills, agenti e comandi in `.claude/`. Per eseguire senza questi, passa `settingSources: []`, che limita l'agente a ciò che configuri programmaticamente. Le impostazioni dei criteri gestiti e la configurazione globale `~/.claude.json` vengono lette indipendentemente da questa opzione. Vedi [Cosa settingSources non controlla](#what-settingsources-does-not-control).

Per una panoramica concettuale di ciò che ogni funzionalità fa e quando usarla, vedi [Estendi Claude Code](/docs/it/features-overview).

<h2 id="control-filesystem-settings-with-settingsources">
  Controlla le impostazioni del filesystem con settingSources
</h2>

L'opzione delle fonti di impostazione ([`setting_sources`](/docs/it/agent-sdk/python#claudeagentoptions) in Python, [`settingSources`](/docs/it/agent-sdk/typescript#settingsource) in TypeScript) controlla quali impostazioni basate sul filesystem carica l'SDK. Passa un elenco esplicito per acconsentire a fonti specifiche, oppure passa un array vuoto per disabilitare le impostazioni utente, progetto e locali.

Questo esempio carica sia le impostazioni a livello di utente che a livello di progetto impostando `settingSources` su `["user", "project"]`:

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ResultMessage

  async for message in query(
      prompt="Help me refactor the auth module",
      options=ClaudeAgentOptions(
          # "user" loads from ~/.claude/, "project" loads from ./.claude/ in cwd.
          # Together they give the agent access to CLAUDE.md, skills, hooks, and
          # permissions from both locations.
          setting_sources=["user", "project"],
          allowed_tools=["Read", "Edit", "Bash"],
      ),
  ):
      if isinstance(message, AssistantMessage):
          for block in message.content:
              if hasattr(block, "text"):
                  print(block.text)
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(f"\nResult: {message.result}")
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Help me refactor the auth module",
    options: {
      // "user" loads from ~/.claude/, "project" loads from ./.claude/ in cwd.
      // Together they give the agent access to CLAUDE.md, skills, hooks, and
      // permissions from both locations.
      settingSources: ["user", "project"],
      allowedTools: ["Read", "Edit", "Bash"]
    }
  })) {
    if (message.type === "assistant") {
      for (const block of message.message.content) {
        if (block.type === "text") console.log(block.text);
      }
    }
    if (message.type === "result" && message.subtype === "success") {
      console.log(`\nResult: ${message.result}`);
    }
  }
  ```
</CodeGroup>

Ogni fonte carica le impostazioni da una posizione specifica, dove `<cwd>` è la directory di lavoro che passi tramite l'opzione `cwd`, o la directory corrente del processo se non impostata. Per la definizione del tipo completo, vedi [`SettingSource`](/docs/it/agent-sdk/typescript#settingsource) (TypeScript) o [`SettingSource`](/docs/it/agent-sdk/python#settingsource) (Python).

| Fonte       | Cosa carica                                                                                                         | Posizione                                                                                                                                                                     |
| :---------- | :------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"project"` | CLAUDE.md del progetto, `.claude/rules/*.md`, skills del progetto, hooks del progetto, `settings.json` del progetto | `<cwd>/.claude/` per `settings.json` e hooks; `<cwd>` e ogni directory padre per CLAUDE.md e rules; `<cwd>` e ogni directory padre fino alla radice del repository per skills |
| `"user"`    | CLAUDE.md dell'utente, `~/.claude/rules/*.md`, skills dell'utente, impostazioni dell'utente                         | `~/.claude/`                                                                                                                                                                  |
| `"local"`   | CLAUDE.local.md, `.claude/settings.local.json`                                                                      | `<cwd>/.claude/` per `settings.local.json`; `<cwd>` e ogni directory padre per CLAUDE.local.md                                                                                |

Omettere `settingSources` equivale a `["user", "project", "local"]`.

L'opzione `cwd` determina dove l'SDK cerca gli input a livello di progetto. CLAUDE.md e rules vengono caricati da `<cwd>` e da ogni directory padre. Skills vengono caricate da `<cwd>` e da ogni directory padre fino alla radice del repository. `settings.json` e hooks del progetto vengono caricati solo da `<cwd>/.claude/` senza fallback di directory padre.

<h3 id="what-settingsources-does-not-control">
  Cosa settingSources non controlla
</h3>

`settingSources` copre le impostazioni utente, progetto e locali. Alcuni input vengono letti indipendentemente dal suo valore:

| Input                                                                 | Comportamento                                                                                                                                                                                                                                                                                                                                                                                                                     | Per disabilitare                                                                                                                                                                                                                            |
| :-------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Impostazioni dei criteri gestiti                                      | Criterio gestito dall'endpoint, sia che si tratti di plist MDM, criterio del registro o file di impostazioni gestite, carica dall'host. Le [impostazioni gestite dal server](/docs/it/server-managed-settings) vengono recuperate su una [configurazione idonea](/docs/it/server-managed-settings#platform-availability) quando la sessione si autentica con un accesso OAuth dell'organizzazione o una chiave API configurata direttamente | Criterio dell'endpoint: rimuovi il file delle impostazioni gestite, plist o criterio del registro dall'host. Impostazioni gestite dal server: controllate dall'amministratore dell'organizzazione; non possono essere disabilitate dall'SDK |
| Configurazione globale `~/.claude.json`                               | Sempre letta                                                                                                                                                                                                                                                                                                                                                                                                                      | Riposiziona con `CLAUDE_CONFIG_DIR` in `env`                                                                                                                                                                                                |
| Memoria automatica in `~/.claude/projects/<project>/memory/`          | Caricata nel prompt di sistema all'avvio della sessione. L'agente scrive nuovi ricordi lì con gli strumenti standard `Write` e `Edit` piuttosto che con uno strumento di memoria dedicato, quindi questi strumenti devono essere abilitati affinché l'agente possa salvare i ricordi                                                                                                                                              | Imposta `autoMemoryEnabled: false` nelle impostazioni, o `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` in `env`                                                                                                                                       |
| [Connettori MCP da claude.ai](/docs/it/mcp#use-mcp-servers-from-claude-ai) | Caricati quando il metodo di autenticazione attivo è un abbonamento a claude.ai. Passare `mcpServers: {}` non li sopprime                                                                                                                                                                                                                                                                                                         | Imposta `strictMcpConfig: true`, [`disableClaudeAiConnectors: true`](/docs/it/mcp#disable-claude-ai-connectors) nelle impostazioni, o `ENABLE_CLAUDEAI_MCP_SERVERS=false` in `env`                                                               |

<Warning>
  Non fare affidamento sulle opzioni predefinite di `query()` per l'isolamento multi-tenant. Poiché gli input di cui sopra vengono letti indipendentemente da `settingSources`, un processo SDK può raccogliere la configurazione a livello di host e la memoria per directory. Per le distribuzioni multi-tenant, esegui ogni tenant nel suo proprio filesystem e imposta `settingSources: []` più `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` in `env`. [Le impostazioni gestite dal server](/docs/it/server-managed-settings) vengono recuperate quando il processo si autentica con una credenziale dell'organizzazione; l'isolamento del filesystem non le rimuove. Vedi [Distribuzione sicura](/docs/it/agent-sdk/secure-deployment).
</Warning>

<h2 id="project-instructions-claude-md-and-rules">
  Istruzioni del progetto (CLAUDE.md e regole)
</h2>

I file `CLAUDE.md` e i file `.claude/rules/*.md` forniscono al tuo agente un contesto persistente sul tuo progetto: convenzioni di codifica, comandi di compilazione, decisioni architettoniche e istruzioni. Quando `settingSources` include `"project"` (come nell'esempio sopra), l'SDK carica questi file nel contesto all'inizio della sessione. L'agente quindi segue le tue convenzioni di progetto senza che tu le ripeta in ogni prompt.

<h3 id="claude-md-load-locations">
  Posizioni di caricamento di CLAUDE.md
</h3>

| Livello                     | Posizione                                                                 | Quando caricato                                                                                               |
| :-------------------------- | :------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------ |
| Progetto (root)             | `<cwd>/CLAUDE.md` o `<cwd>/.claude/CLAUDE.md`                             | `settingSources` include `"project"`                                                                          |
| Regole del progetto         | `<cwd>/.claude/rules/*.md` e `.claude/rules/*.md` in ogni directory padre | `settingSources` include `"project"`                                                                          |
| Progetto (directory padre)  | File `CLAUDE.md` nelle directory sopra `cwd`                              | `settingSources` include `"project"`, caricato all'inizio della sessione                                      |
| Progetto (directory figlie) | File `CLAUDE.md` nelle sottodirectory di `cwd`                            | `settingSources` include `"project"`, caricato su richiesta quando l'agente legge un file in quel sottoalbero |
| Locale                      | `<cwd>/CLAUDE.local.md` e `CLAUDE.local.md` in ogni directory padre       | `settingSources` include `"local"`                                                                            |
| Utente                      | `~/.claude/CLAUDE.md`                                                     | `settingSources` include `"user"`                                                                             |
| Regole dell'utente          | `~/.claude/rules/*.md`                                                    | `settingSources` include `"user"`                                                                             |

Tutti i livelli sono additivi: se esistono sia file `CLAUDE.md` di progetto che di utente, l'agente vede entrambi. Non esiste una regola di precedenza rigida tra i livelli; se le istruzioni entrano in conflitto, il risultato dipende da come Claude le interpreta. Scrivi regole non conflittuali, o dichiara esplicitamente la precedenza nel file più specifico ("Queste istruzioni di progetto sostituiscono qualsiasi impostazione predefinita conflittuale a livello di utente").

<Tip>
  Puoi anche iniettare il contesto direttamente tramite `systemPrompt` senza usare file `CLAUDE.md`. Vedi [Modifica i prompt di sistema](/docs/it/agent-sdk/modifying-system-prompts). Usa `CLAUDE.md` quando vuoi che lo stesso contesto sia condiviso tra le sessioni interattive di Claude Code e i tuoi agenti SDK.
</Tip>

Per come strutturare e organizzare il contenuto di `CLAUDE.md`, vedi [Gestisci la memoria di Claude](/docs/it/memory).

<h2 id="skills">
  Skills
</h2>

Le skills sono file markdown che danno al tuo agente conoscenze specializzate e flussi di lavoro invocabili. A differenza di `CLAUDE.md` (che si carica ogni sessione), le skills si caricano su richiesta. L'agente riceve le descrizioni delle skills all'avvio e carica il contenuto completo quando rilevante.

Le skills vengono scoperte dal filesystem tramite `settingSources`. Quando l'opzione `skills` su `query()` viene omessa, le skills di utente e progetto scoperte vengono abilitate e lo strumento Skill è disponibile, corrispondendo al comportamento della CLI. Per controllare quali skills sono abilitate, passa `skills` come `"all"`, un elenco di nomi di skills, o `[]` per disabilitare tutte. Quando `skills` è impostato, l'SDK aggiunge automaticamente lo strumento Skill a `allowedTools`. Se passi anche un elenco esplicito di `tools`, includi `"Skill"` in quell'elenco in modo che Claude possa invocare le skills.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  # Skills in .claude/skills/ are discovered automatically
  # when settingSources includes "project"
  async for message in query(
      prompt="Review this PR using our code review checklist",
      options=ClaudeAgentOptions(
          setting_sources=["user", "project"],
          skills="all",
          allowed_tools=["Read", "Grep", "Glob"],
      ),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Skills in .claude/skills/ are discovered automatically
  // when settingSources includes "project"
  for await (const message of query({
    prompt: "Review this PR using our code review checklist",
    options: {
      settingSources: ["user", "project"],
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<Note>
  Le skills devono essere create come artefatti del filesystem (`.claude/skills/<name>/SKILL.md`). L'SDK non ha un'API programmatica per registrare le skills. Vedi [Agent Skills nell'SDK](/docs/it/agent-sdk/skills) per i dettagli completi.
</Note>

Per ulteriori informazioni sulla creazione e l'utilizzo delle skills, vedi [Agent Skills nell'SDK](/docs/it/agent-sdk/skills).

<h2 id="hooks">
  Hooks
</h2>

L'SDK supporta due modi per definire gli hooks, e vengono eseguiti fianco a fianco:

* **Filesystem hooks:** comandi shell definiti in `settings.json`, caricati quando `settingSources` include la fonte rilevante. Questi sono gli stessi hooks che configureresti per [sessioni interattive di Claude Code](/docs/it/hooks-guide).
* **Programmatic hooks:** funzioni di callback passate direttamente a `query()`. Questi vengono eseguiti nel tuo processo di applicazione e possono restituire decisioni strutturate. Vedi [Controlla l'esecuzione con gli hooks](/docs/it/agent-sdk/hooks).

Entrambi i tipi vengono eseguiti durante lo stesso ciclo di vita degli hooks. Se hai già degli hooks nel file `.claude/settings.json` del tuo progetto e imposti `settingSources: ["project"]`, quegli hooks vengono eseguiti automaticamente nell'SDK senza configurazione aggiuntiva.

I callback degli hooks ricevono l'input dello strumento e restituiscono un dict di decisione. Restituire `{}` significa consentire allo strumento di procedere. Per bloccare l'esecuzione, restituisci un oggetto `hookSpecificOutput` con `permissionDecision: "deny"` e un `permissionDecisionReason`. Il motivo viene inviato a Claude come risultato dello strumento. I campi di primo livello `decision` e `reason` sono deprecati per `PreToolUse`. Vedi la [guida degli hooks](/docs/it/agent-sdk/hooks) per la firma completa del callback e i tipi di ritorno.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher, ResultMessage


  # PreToolUse hook callback. Positional args:
  #   input_data: HookInput dict with tool_name, tool_input, hook_event_name
  #   tool_use_id: str | None, the ID of the tool call being intercepted
  #   context: HookContext, carries session metadata
  async def audit_bash(input_data, tool_use_id, context):
      command = input_data.get("tool_input", {}).get("command", "")
      if "rm -rf" in command:
          return {
              "hookSpecificOutput": {
                  "hookEventName": "PreToolUse",
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Destructive command blocked",
              }
          }
      return {}  # Empty dict: allow the tool to proceed


  # Filesystem hooks from .claude/settings.json run automatically
  # when settingSources loads them. You can also add programmatic hooks:
  async for message in query(
      prompt="Refactor the auth module",
      options=ClaudeAgentOptions(
          setting_sources=["project"],  # Loads hooks from .claude/settings.json
          hooks={
              "PreToolUse": [
                  HookMatcher(matcher="Bash", hooks=[audit_bash]),
              ]
          },
      ),
  ):
      if isinstance(message, ResultMessage) and message.subtype == "success":
          print(message.result)
  ```

  ```typescript TypeScript theme={null}
  import { query, type HookInput, type HookJSONOutput } from "@anthropic-ai/claude-agent-sdk";

  // PreToolUse hook callback. HookInput is a discriminated union on
  // hook_event_name, so narrowing on it gives TypeScript the right
  // tool_input shape for this event.
  const auditBash = async (input: HookInput): Promise<HookJSONOutput> => {
    if (input.hook_event_name !== "PreToolUse") return {};
    const toolInput = input.tool_input as { command?: string };
    if (toolInput.command?.includes("rm -rf")) {
      return {
        hookSpecificOutput: {
          hookEventName: "PreToolUse",
          permissionDecision: "deny",
          permissionDecisionReason: "Destructive command blocked",
        },
      };
    }
    return {}; // Empty object: allow the tool to proceed
  };

  // Filesystem hooks from .claude/settings.json run automatically
  // when settingSources loads them. You can also add programmatic hooks:
  for await (const message of query({
    prompt: "Refactor the auth module",
    options: {
      settingSources: ["project"], // Loads hooks from .claude/settings.json
      hooks: {
        PreToolUse: [{ matcher: "Bash", hooks: [auditBash] }]
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<h3 id="when-to-use-which-hook-type">
  Quando usare quale tipo di hook
</h3>

| Tipo di hook                             | Migliore per                                                                                                                                                                                                                                                                                                                                      |
| :--------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Filesystem** (`settings.json`)         | Condividere gli hooks tra le sessioni CLI e SDK. Supporta `"command"` (script shell), `"http"` (POST a un endpoint), `"mcp_tool"` (chiama lo strumento di un server MCP connesso), `"prompt"` (LLM valuta un prompt), e `"agent"` (genera un agente verificatore). Questi si attivano nell'agente principale e in qualsiasi subagente che genera. |
| **Programmatic** (callback in `query()`) | Logica specifica dell'applicazione, decisioni strutturate e integrazione in-process. Questi si attivano anche all'interno dei subagenti. Il callback riceve `agent_id` e `agent_type` per distinguere.                                                                                                                                            |

<Note>
  L'SDK TypeScript supporta eventi hook aggiuntivi rispetto a Python, inclusi `SessionStart`, `SessionEnd`, `TeammateIdle` e `TaskCompleted`. Vedi la [guida degli hooks](/docs/it/agent-sdk/hooks) per la tabella di compatibilità degli eventi completa.
</Note>

Per i dettagli completi sugli hooks programmatici, vedi [Controlla l'esecuzione con gli hooks](/docs/it/agent-sdk/hooks). Per la sintassi degli hooks del filesystem, vedi [Hooks](/docs/it/hooks).

<h2 id="choose-the-right-feature">
  Scegli la funzionalità giusta
</h2>

L'Agent SDK ti dà accesso a diversi modi per estendere il comportamento del tuo agente. Se non sei sicuro di quale usare, questa tabella mappa gli obiettivi comuni all'approccio giusto.

| Vuoi...                                                                                                    | Usa                                                   | Superficie SDK                                                                                                                                                                                                 |
| :--------------------------------------------------------------------------------------------------------- | :---------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Impostare le convenzioni del progetto che il tuo agente segue sempre                                       | [CLAUDE.md](/docs/it/memory)                               | `settingSources: ["project"]` lo carica automaticamente                                                                                                                                                        |
| Dare all'agente materiale di riferimento che carica quando rilevante                                       | [Skills](/docs/it/agent-sdk/skills)                        | opzione `settingSources` + `skills`                                                                                                                                                                            |
| Eseguire un flusso di lavoro riutilizzabile (deploy, review, release)                                      | [Skills invocabili dall'utente](/docs/it/agent-sdk/skills) | opzione `settingSources` + `skills`                                                                                                                                                                            |
| Delegare un sottocompito isolato a un contesto fresco (ricerca, review)                                    | [Subagenti](/docs/it/agent-sdk/subagents)                  | parametro `agents` + `allowedTools: ["Agent"]`                                                                                                                                                                 |
| Coordinare più istanze di Claude Code con elenchi di attività condivisi e messaggistica diretta tra agenti | [Team di agenti](/docs/it/agent-teams)                     | Non configurato direttamente tramite le opzioni SDK. I team di agenti sono una funzionalità CLI in cui una sessione agisce come il capo del team, coordinando il lavoro tra i compagni di squadra indipendenti |
| Eseguire logica deterministica sulle chiamate di strumenti (audit, block, transform)                       | [Hooks](/docs/it/agent-sdk/hooks)                          | parametro `hooks` con callback, o script shell caricati tramite `settingSources`                                                                                                                               |
| Dare a Claude accesso strutturato agli strumenti di un servizio esterno                                    | [MCP](/docs/it/agent-sdk/mcp)                              | parametro `mcpServers`                                                                                                                                                                                         |

<Tip>
  **Subagenti versus team di agenti:** I subagenti sono effimeri e isolati: conversazione fresca, un compito, riepilogo restituito al genitore. I team di agenti coordinano più istanze indipendenti di Claude Code che condividono un elenco di attività e si messaggiano direttamente. I team di agenti sono una funzionalità CLI. Vedi [Cosa ereditano i subagenti](/docs/it/agent-sdk/subagents#what-subagents-inherit) e il [confronto dei team di agenti](/docs/it/agent-teams#compare-with-subagents) per i dettagli.
</Tip>

Ogni funzionalità che abiliti aggiunge alla finestra di contesto del tuo agente. Per i costi per funzionalità e come queste funzionalità si stratificano insieme, vedi [Estendi Claude Code](/docs/it/features-overview#understand-context-costs).

<h2 id="related-resources">
  Risorse correlate
</h2>

* [Estendi Claude Code](/docs/it/features-overview): Panoramica concettuale di tutte le funzionalità di estensione, con tabelle di confronto e analisi dei costi di contesto
* [Skills nell'SDK](/docs/it/agent-sdk/skills): Guida completa all'utilizzo delle skills a livello programmatico
* [Subagenti](/docs/it/agent-sdk/subagents): Definisci e invoca subagenti per sottocompiti isolati
* [Hooks](/docs/it/agent-sdk/hooks): Intercetta e controlla il comportamento dell'agente nei punti di esecuzione chiave
* [Permessi](/docs/it/agent-sdk/permissions): Controlla l'accesso agli strumenti con modalità, regole e callback
* [Prompt di sistema](/docs/it/agent-sdk/modifying-system-prompts): Inietta il contesto senza file CLAUDE.md
