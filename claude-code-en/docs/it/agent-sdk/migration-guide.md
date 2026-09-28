> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Migrazione a Claude Agent SDK

> Guida per la migrazione dei Claude Code SDK TypeScript e Python a Claude Agent SDK

<h2 id="overview">
  Panoramica
</h2>

Claude Code SDK è stato rinominato in **Claude Agent SDK** e la sua documentazione è stata riorganizzata. Questo cambiamento riflette le capacità più ampie dell'SDK per la creazione di agenti AI oltre ai soli compiti di codifica.

State migrando da OpenAI Agents SDK? La [ricetta di migrazione da OpenAI Agents SDK](https://platform.claude.com/cookbook/claude-agent-sdk-04-migrating-from-openai-agents-sdk) mappa ogni primitiva su Claude Agent SDK attraverso un singolo esempio pratico.

<h2 id="what’s-changed">
  Cosa è Cambiato
</h2>

| Aspetto                      | Precedente                  | Nuovo                                                                   |
| :--------------------------- | :-------------------------- | :---------------------------------------------------------------------- |
| **Nome Pacchetto (TS/JS)**   | `@anthropic-ai/claude-code` | `@anthropic-ai/claude-agent-sdk`                                        |
| **Pacchetto Python**         | `claude-code-sdk`           | `claude-agent-sdk`                                                      |
| **Posizione Documentazione** | Claude Code docs            | Claude Code docs → sezione dedicata [Agent SDK](/docs/it/agent-sdk/overview) |

<h2 id="migration-steps">
  Passaggi di migrazione
</h2>

<h3 id="for-typescript/javascript-projects">
  Per progetti TypeScript/JavaScript
</h3>

**1. Disinstallare il vecchio pacchetto:**

```bash theme={null}
npm uninstall @anthropic-ai/claude-code
```

**2. Installare il nuovo pacchetto:**

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

**3. Aggiornare gli import:**

Modificare tutti gli import da `@anthropic-ai/claude-code` a `@anthropic-ai/claude-agent-sdk`:

```typescript theme={null}
// Prima
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-code";

// Dopo
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
```

**4. Aggiornare package.json:**

Se `@anthropic-ai/claude-code` è ancora elencato nel vostro `package.json`, sostituirlo con `@anthropic-ai/claude-agent-sdk` e aggiornare anche l'intervallo di versione, ad esempio da `"^0.0.42"` a `"^0.3.0"`.

**5. Consultare [le modifiche di rilievo](#breaking-changes)**

Apportare le modifiche al codice necessarie per completare la migrazione.

<h3 id="for-python-projects">
  Per progetti Python
</h3>

**1. Disinstallare il vecchio pacchetto:**

```bash theme={null}
pip uninstall -y claude-code-sdk
```

Se il vecchio pacchetto non è installato, pip stampa `WARNING: Skipping claude-code-sdk as it is not installed.` Questo è previsto e potete continuare al passaggio successivo.

**2. Installare il nuovo pacchetto:**

```bash theme={null}
pip install claude-agent-sdk
```

Se `claude-code-sdk` è elencato nel vostro `requirements.txt` o `pyproject.toml`, sostituirlo con `claude-agent-sdk`.

**3. Aggiornare gli import:**

Modificare tutti gli import da `claude_code_sdk` a `claude_agent_sdk`:

```python theme={null}
# Prima
from claude_code_sdk import query, ClaudeCodeOptions

# Dopo
from claude_agent_sdk import query, ClaudeAgentOptions
```

**4. Consultare [le modifiche di rilievo](#breaking-changes)**

Apportare le modifiche al codice necessarie per completare la migrazione.

<h2 id="breaking-changes">
  Modifiche di rilievo
</h2>

<Warning>
  Per migliorare l'isolamento e la configurazione esplicita, Claude Agent SDK v0.1.0 introduce modifiche di rilievo per gli utenti che eseguono la migrazione da Claude Code SDK.
</Warning>

<h3 id="python-claudecodeoptions-renamed-to-claudeagentoptions">
  Python: ClaudeCodeOptions rinominato in ClaudeAgentOptions
</h3>

**Cosa è cambiato:** Il tipo SDK Python `ClaudeCodeOptions` è stato rinominato in `ClaudeAgentOptions`.

**Migrazione:**

```python theme={null}
# BEFORE (claude-code-sdk)
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7", permission_mode="acceptEdits")

# AFTER (claude-agent-sdk)
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7", permission_mode="acceptEdits")
```

<h3 id="system-prompt-no-longer-default">
  System prompt non è più predefinito
</h3>

**Cosa è cambiato:** L'SDK non utilizza più il system prompt di Claude Code per impostazione predefinita.

**Migrazione:**

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // BEFORE (v0.0.x) - Used Claude Code's system prompt by default
  const before = query({ prompt: "Hello" });

  // AFTER (v0.1.0) - Uses minimal system prompt by default
  // To get the old behavior, explicitly request Claude Code's preset:
  const presetResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: { type: "preset", preset: "claude_code" }
    }
  });

  // Or use a custom system prompt:
  const customResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: "You are a helpful coding assistant"
    }
  });
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions
  import asyncio


  async def main():
      # BEFORE (v0.0.x) - Used Claude Code's system prompt by default
      async for message in query(prompt="Hello"):
          print(message)

      # AFTER (v0.1.0) - Uses minimal system prompt by default
      # To get the old behavior, explicitly request Claude Code's preset:
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(
              system_prompt={"type": "preset", "preset": "claude_code"}  # Use the preset
          ),
      ):
          print(message)

      # Or use a custom system prompt:
      async for message in query(
          prompt="Hello",
          options=ClaudeAgentOptions(system_prompt="You are a helpful coding assistant"),
      ):
          print(message)


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="settings-sources-default">
  Impostazioni predefinite delle origini
</h3>

Questa impostazione predefinita è stata brevemente modificata in v0.1.0 per non caricare alcuna impostazione del file system e successivamente ripristinata, quindi non è necessaria alcuna azione di migrazione.

**Comportamento attuale:** Omettendo `settingSources` su `query()` vengono caricate le impostazioni del file system dell'utente, del progetto e locale, corrispondendo alla CLI. Ciò include `~/.claude/settings.json`, `.claude/settings.json`, `.claude/settings.local.json`, file CLAUDE.md e comandi personalizzati.

Per eseguire l'isolamento dalle impostazioni del file system, passare `settingSources: []`, oppure `setting_sources=[]` in Python. Vedere [Control filesystem settings with settingSources](/docs/it/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) per informazioni su cosa carica ogni origine.

L'isolamento è particolarmente importante per le pipeline CI/CD, le applicazioni distribuite, gli ambienti di test e i sistemi multi-tenant in cui le personalizzazioni locali non dovrebbero trapelare.

<Note>
  Python SDK 0.1.59 e versioni precedenti hanno trattato un elenco vuoto allo stesso modo dell'omissione dell'opzione, quindi eseguire l'aggiornamento prima di fare affidamento su `setting_sources=[]`. Vedere [What settingSources does not control](/docs/it/agent-sdk/claude-code-features#what-settingsources-does-not-control) per gli input che vengono letti anche quando `settingSources` è `[]`.
</Note>

<h2 id="next-steps">
  Passaggi successivi
</h2>

* Esplorare la [Panoramica di Agent SDK](/docs/it/agent-sdk/overview) per conoscere le funzionalità disponibili
* Consultare il [Riferimento SDK TypeScript](/docs/it/agent-sdk/typescript) per la documentazione dettagliata dell'API
* Rivedere il [Riferimento SDK Python](/docs/it/agent-sdk/python) per la documentazione specifica di Python
* Scopri di più su [Strumenti personalizzati](/docs/it/agent-sdk/custom-tools) e [Integrazione MCP](/docs/it/agent-sdk/mcp)
