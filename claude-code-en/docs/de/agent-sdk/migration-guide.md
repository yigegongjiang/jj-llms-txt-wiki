> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Migrieren zum Claude Agent SDK

> Leitfaden für die Migration der Claude Code TypeScript- und Python-SDKs zum Claude Agent SDK

<h2 id="overview">
  Übersicht
</h2>

Das Claude Code SDK wurde in das **Claude Agent SDK** umbenannt und seine Dokumentation wurde neu organisiert. Diese Änderung spiegelt die umfassenderen Funktionen des SDK für die Erstellung von KI-Agenten über reine Codierungsaufgaben hinaus wider.

Migrieren Sie stattdessen vom OpenAI Agents SDK? Das [OpenAI Agents SDK-Migrationskochbuch](https://platform.claude.com/cookbook/claude-agent-sdk-04-migrating-from-openai-agents-sdk) ordnet jedes Primitive dem Claude Agent SDK durch ein einzelnes durchgearbeitetes Beispiel zu.

<h2 id="what’s-changed">
  Was hat sich geändert
</h2>

| Aspekt                | Alt                         | Neu                                                                                 |
| :-------------------- | :-------------------------- | :---------------------------------------------------------------------------------- |
| **Paketname (TS/JS)** | `@anthropic-ai/claude-code` | `@anthropic-ai/claude-agent-sdk`                                                    |
| **Python-Paket**      | `claude-code-sdk`           | `claude-agent-sdk`                                                                  |
| **Dokumentationsort** | Claude Code-Dokumentation   | Claude Code-Dokumentation → dedizierter [Agent SDK](/docs/de/agent-sdk/overview)-Bereich |

<h2 id="migration-steps">
  Migrationsschritte
</h2>

<h3 id="for-typescript/javascript-projects">
  Für TypeScript/JavaScript-Projekte
</h3>

**1. Deinstallieren Sie das alte Paket:**

```bash theme={null}
npm uninstall @anthropic-ai/claude-code
```

**2. Installieren Sie das neue Paket:**

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

**3. Aktualisieren Sie Ihre Importe:**

Ändern Sie alle Importe von `@anthropic-ai/claude-code` zu `@anthropic-ai/claude-agent-sdk`:

```typescript theme={null}
// Vorher
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-code";

// Nachher
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
```

**4. Aktualisieren Sie package.json:**

Wenn `@anthropic-ai/claude-code` noch in Ihrer `package.json` aufgelistet ist, ersetzen Sie es durch `@anthropic-ai/claude-agent-sdk` und aktualisieren Sie auch den Versionsbereich, zum Beispiel von `"^0.0.42"` zu `"^0.3.0"`.

**5. Überprüfen Sie [Breaking Changes](#breaking-changes)**

Nehmen Sie alle erforderlichen Codeänderungen vor, um die Migration abzuschließen.

<h3 id="for-python-projects">
  Für Python-Projekte
</h3>

**1. Deinstallieren Sie das alte Paket:**

```bash theme={null}
pip uninstall -y claude-code-sdk
```

Wenn das alte Paket nicht installiert ist, gibt pip `WARNING: Skipping claude-code-sdk as it is not installed.` aus. Das ist zu erwarten und Sie können zum nächsten Schritt übergehen.

**2. Installieren Sie das neue Paket:**

```bash theme={null}
pip install claude-agent-sdk
```

Wenn `claude-code-sdk` in Ihrer `requirements.txt` oder `pyproject.toml` aufgelistet ist, ersetzen Sie es durch `claude-agent-sdk`.

**3. Aktualisieren Sie Ihre Importe:**

Ändern Sie alle Importe von `claude_code_sdk` zu `claude_agent_sdk`:

```python theme={null}
# Vorher
from claude_code_sdk import query, ClaudeCodeOptions

# Nachher
from claude_agent_sdk import query, ClaudeAgentOptions
```

**4. Überprüfen Sie [Breaking Changes](#breaking-changes)**

Nehmen Sie alle erforderlichen Codeänderungen vor, um die Migration abzuschließen.

<h2 id="breaking-changes">
  Grundlegende Änderungen
</h2>

<Warning>
  Um die Isolation zu verbessern und die explizite Konfiguration zu ermöglichen, führt Claude Agent SDK v0.1.0 grundlegende Änderungen für Benutzer ein, die von Claude Code SDK migrieren.
</Warning>

<h3 id="python-claudecodeoptions-renamed-to-claudeagentoptions">
  Python: ClaudeCodeOptions in ClaudeAgentOptions umbenannt
</h3>

**Was hat sich geändert:** Der Python SDK-Typ `ClaudeCodeOptions` wurde in `ClaudeAgentOptions` umbenannt.

**Migration:**

```python theme={null}
# BEFORE (claude-code-sdk)
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7", permission_mode="acceptEdits")

# AFTER (claude-agent-sdk)
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7", permission_mode="acceptEdits")
```

<h3 id="system-prompt-no-longer-default">
  System-Eingabeaufforderung ist nicht mehr Standard
</h3>

**Was hat sich geändert:** Das SDK verwendet nicht mehr standardmäßig die System-Eingabeaufforderung von Claude Code.

**Migration:**

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
  Standardwerte für Einstellungsquellen
</h3>

Dieser Standard wurde in v0.1.0 kurzzeitig geändert, um keine Dateisystem-Einstellungen zu laden, und wurde dann zurückgesetzt, daher ist keine Migrationsaktion erforderlich.

**Aktuelles Verhalten:** Das Weglassen von `settingSources` bei `query()` lädt Benutzer-, Projekt- und lokale Dateisystem-Einstellungen, was der CLI entspricht. Dies umfasst `~/.claude/settings.json`, `.claude/settings.json`, `.claude/settings.local.json`, CLAUDE.md-Dateien und benutzerdefinierte Befehle.

Um isoliert von Dateisystem-Einstellungen zu laufen, übergeben Sie `settingSources: []` oder `setting_sources=[]` in Python. Siehe [Dateisystem-Einstellungen mit settingSources steuern](/docs/de/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources), um zu erfahren, welche Quellen jeweils geladen werden.

Die Isolation ist besonders wichtig für CI/CD-Pipelines, bereitgestellte Anwendungen, Testumgebungen und Multi-Tenant-Systeme, in denen lokale Anpassungen nicht durchsickern sollten.

<Note>
  Python SDK 0.1.59 und früher behandelten eine leere Liste genauso wie das Weglassen der Option, daher sollten Sie ein Upgrade durchführen, bevor Sie sich auf `setting_sources=[]` verlassen. Siehe [Was settingSources nicht steuert](/docs/de/agent-sdk/claude-code-features#what-settingsources-does-not-control), um zu erfahren, welche Eingaben auch dann gelesen werden, wenn `settingSources` auf `[]` gesetzt ist.
</Note>

<h2 id="next-steps">
  Nächste Schritte
</h2>

* Erkunden Sie die [Agent SDK-Übersicht](/docs/de/agent-sdk/overview), um mehr über verfügbare Funktionen zu erfahren
* Schauen Sie sich die [TypeScript SDK-Referenz](/docs/de/agent-sdk/typescript) für detaillierte API-Dokumentation an
* Überprüfen Sie die [Python SDK-Referenz](/docs/de/agent-sdk/python) für Python-spezifische Dokumentation
* Erfahren Sie mehr über [Benutzerdefinierte Tools](/docs/de/agent-sdk/custom-tools) und [MCP-Integration](/docs/de/agent-sdk/mcp)
