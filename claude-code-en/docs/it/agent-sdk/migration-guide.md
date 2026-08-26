> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Migrazione a Claude Agent SDK

> Guida per la migrazione dei Claude Code SDK TypeScript e Python a Claude Agent SDK

<h2 id="overview">
  Panoramica
</h2>

Claude Code SDK è stato rinominato in **Claude Agent SDK** e la sua documentazione è stata riorganizzata. Questo cambiamento riflette le capacità più ampie dell'SDK per la creazione di agenti AI oltre ai soli compiti di codifica.

<h2 id="what’s-changed">
  Cosa è cambiato
</h2>

| Aspetto                            | Precedente                  | Nuovo                            |
| :--------------------------------- | :-------------------------- | :------------------------------- |
| **Nome del pacchetto (TS/JS)**     | `@anthropic-ai/claude-code` | `@anthropic-ai/claude-agent-sdk` |
| **Pacchetto Python**               | `claude-code-sdk`           | `claude-agent-sdk`               |
| **Posizione della documentazione** | Documentazione Claude Code  | API Guide → Sezione Agent SDK    |

<Note>
  **Modifiche alla documentazione:** La documentazione di Agent SDK è stata spostata dalla documentazione di Claude Code alla API Guide in una sezione dedicata [Agent SDK](/docs/it/agent-sdk/overview). La documentazione di Claude Code ora si concentra sullo strumento CLI e sulle funzionalità di automazione.
</Note>

<h2 id="migration-steps">
  Passaggi di migrazione
</h2>

<h3 id="for-typescript/javascript-projects">
  Per progetti TypeScript/JavaScript
</h3>

**1. Disinstallare il pacchetto precedente:**

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

**4. Aggiornare le dipendenze in package.json:**

Se il pacchetto è elencato nel vostro `package.json`, aggiornarlo:

Prima:

```json theme={null}
{
  "dependencies": {
    "@anthropic-ai/claude-code": "^0.0.42"
  }
}
```

Dopo:

```json theme={null}
{
  "dependencies": {
    "@anthropic-ai/claude-agent-sdk": "^0.2.0"
  }
}
```

**5. Rivedere i [cambiamenti significativi](#breaking-changes)**

Apportare le modifiche al codice necessarie per completare la migrazione.

<h3 id="for-python-projects">
  Per progetti Python
</h3>

**1. Disinstallare il pacchetto precedente:**

```bash theme={null}
pip uninstall claude-code-sdk
```

**2. Installare il nuovo pacchetto:**

```bash theme={null}
pip install claude-agent-sdk
```

**3. Aggiornare gli import:**

Modificare tutti gli import da `claude_code_sdk` a `claude_agent_sdk`:

```python theme={null}
# Prima
from claude_code_sdk import query, ClaudeCodeOptions

# Dopo
from claude_agent_sdk import query, ClaudeAgentOptions
```

**4. Aggiornare i nomi dei tipi:**

Modificare `ClaudeCodeOptions` in `ClaudeAgentOptions`:

```python theme={null}
# Prima
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7")

# Dopo
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7")
```

**5. Rivedere i [cambiamenti significativi](#breaking-changes)**

Apportare le modifiche al codice necessarie per completare la migrazione.

<h2 id="breaking-changes">
  Cambiamenti significativi
</h2>

<Warning>
  Per migliorare l'isolamento e la configurazione esplicita, Claude Agent SDK v0.1.0 introduce cambiamenti significativi per gli utenti che migrano da Claude Code SDK. Rivedere attentamente questa sezione prima di eseguire la migrazione.
</Warning>

<h3 id="python-claudecodeoptions-renamed-to-claudeagentoptions">
  Python: ClaudeCodeOptions rinominato in ClaudeAgentOptions
</h3>

**Cosa è cambiato:** Il tipo Python SDK `ClaudeCodeOptions` è stato rinominato in `ClaudeAgentOptions`.

**Migrazione:**

```python theme={null}
# PRIMA (claude-code-sdk)
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7", permission_mode="acceptEdits")

# DOPO (claude-agent-sdk)
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7", permission_mode="acceptEdits")
```

**Perché è cambiato:** Il nome del tipo ora corrisponde al branding "Claude Agent SDK" e fornisce coerenza nelle convenzioni di denominazione dell'SDK.

<h3 id="system-prompt-no-longer-default">
  System prompt non è più predefinito
</h3>

**Cosa è cambiato:** L'SDK non utilizza più il system prompt di Claude Code per impostazione predefinita.

**Migrazione:**

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // PRIMA (v0.0.x) - Utilizzava il system prompt di Claude Code per impostazione predefinita
  const before = query({ prompt: "Hello" });

  // DOPO (v0.1.0) - Utilizza un system prompt minimo per impostazione predefinita
  // Per ottenere il comportamento precedente, richiedere esplicitamente il preset di Claude Code:
  const presetResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: { type: "preset", preset: "claude_code" }
    }
  });

  // Oppure utilizzare un system prompt personalizzato:
  const customResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: "You are a helpful coding assistant"
    }
  });
  ```

  ```python Python theme={null}
  # PRIMA (v0.0.x) - Utilizzava il system prompt di Claude Code per impostazione predefinita
  async for message in query(prompt="Hello"):
      print(message)

  # DOPO (v0.1.0) - Utilizza un system prompt minimo per impostazione predefinita
  # Per ottenere il comportamento precedente, richiedere esplicitamente il preset di Claude Code:
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(
          system_prompt={"type": "preset", "preset": "claude_code"}  # Utilizzare il preset
      ),
  ):
      print(message)

  # Oppure utilizzare un system prompt personalizzato:
  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(system_prompt="You are a helpful coding assistant"),
  ):
      print(message)
  ```
</CodeGroup>

**Perché è cambiato:** Fornisce un migliore controllo e isolamento per le applicazioni SDK. Ora è possibile creare agenti con comportamento personalizzato senza ereditare le istruzioni focalizzate sulla CLI di Claude Code.

<h3 id="settings-sources-default">
  Impostazioni predefinite delle fonti
</h3>

Questo valore predefinito è stato brevemente modificato in v0.1.0 e successivamente ripristinato, quindi non è necessaria alcuna azione di migrazione.

**Comportamento attuale:** Omettendo `settingSources` su `query()` vengono caricate le impostazioni dell'utente, del progetto e del file system locale, corrispondendo alla CLI. Questo include `~/.claude/settings.json`, `.claude/settings.json`, `.claude/settings.local.json`, file CLAUDE.md e comandi personalizzati.

Per eseguire l'isolamento dalle impostazioni del file system, passare un array vuoto:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const isolatedResult = query({
    prompt: "Hello",
    options: {
      settingSources: [] // Nessuna impostazione del file system caricata
    }
  });

  // Oppure caricare solo fonti specifiche:
  const projectOnlyResult = query({
    prompt: "Hello",
    options: {
      settingSources: ["project"] // Solo impostazioni del progetto
    }
  });
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(setting_sources=[]),  # Nessuna impostazione del file system caricata
  ):
      print(message)

  # Oppure caricare solo fonti specifiche:
  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(
          setting_sources=["project"]  # Solo impostazioni del progetto
      ),
  ):
      print(message)
  ```
</CodeGroup>

L'isolamento è particolarmente importante per le pipeline CI/CD, le applicazioni distribuite, gli ambienti di test e i sistemi multi-tenant dove le personalizzazioni locali non dovrebbero infiltrarsi.

<Note>
  SDK v0.1.0 ha brevemente impostato per impostazione predefinita nessuna impostazione caricata; questo è stato ripristinato nelle versioni successive. Python SDK 0.1.59 e versioni precedenti hanno trattato un elenco vuoto come l'omissione dell'opzione, quindi eseguire l'aggiornamento prima di fare affidamento su `setting_sources=[]`. Vedere [Cosa settingSources non controlla](/docs/it/agent-sdk/claude-code-features#what-settingsources-does-not-control) per gli input che vengono letti anche quando `settingSources` è `[]`.
</Note>

<h2 id="why-the-rename">
  Perché il cambio di nome?
</h2>

Claude Code SDK è stato originariamente progettato per compiti di codifica, ma si è evoluto in un framework potente per la creazione di tutti i tipi di agenti AI. Il nuovo nome "Claude Agent SDK" riflette meglio le sue capacità:

* Creazione di agenti aziendali (assistenti legali, consulenti finanziari, supporto clienti)
* Creazione di agenti di codifica specializzati (bot SRE, revisori di sicurezza, agenti di revisione del codice)
* Sviluppo di agenti personalizzati per qualsiasi dominio con utilizzo di strumenti, integrazione MCP e altro ancora

<h2 id="getting-help">
  Ottenere aiuto
</h2>

Se si incontrano problemi durante la migrazione:

**Per TypeScript/JavaScript:**

1. Verificare che tutti gli import siano aggiornati per utilizzare `@anthropic-ai/claude-agent-sdk`
2. Verificare che il vostro package.json abbia il nuovo nome del pacchetto
3. Eseguire `npm install` per assicurarsi che le dipendenze siano aggiornate

**Per Python:**

1. Verificare che tutti gli import siano aggiornati per utilizzare `claude_agent_sdk`
2. Verificare che il vostro requirements.txt o pyproject.toml abbia il nuovo nome del pacchetto
3. Eseguire `pip install claude-agent-sdk` per assicurarsi che il pacchetto sia installato

<h2 id="next-steps">
  Passaggi successivi
</h2>

* Esplorare la [Panoramica di Agent SDK](/docs/it/agent-sdk/overview) per conoscere le funzionalità disponibili
* Consultare il [Riferimento SDK TypeScript](/docs/it/agent-sdk/typescript) per la documentazione dettagliata dell'API
* Rivedere il [Riferimento SDK Python](/docs/it/agent-sdk/python) per la documentazione specifica di Python
* Scopri di più su [Strumenti personalizzati](/docs/it/agent-sdk/custom-tools) e [Integrazione MCP](/docs/it/agent-sdk/mcp)
