> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Migrieren zum Claude Agent SDK

> Leitfaden für die Migration der Claude Code TypeScript- und Python-SDKs zum Claude Agent SDK

<h2 id="overview">
  Übersicht
</h2>

Das Claude Code SDK wurde in das **Claude Agent SDK** umbenannt und seine Dokumentation wurde neu organisiert. Diese Änderung spiegelt die umfassenderen Funktionen des SDKs für die Erstellung von KI-Agenten über reine Codierungsaufgaben hinaus wider.

<h2 id="what’s-changed">
  Was hat sich geändert
</h2>

| Aspekt                | Alt                         | Neu                               |
| :-------------------- | :-------------------------- | :-------------------------------- |
| **Paketname (TS/JS)** | `@anthropic-ai/claude-code` | `@anthropic-ai/claude-agent-sdk`  |
| **Python-Paket**      | `claude-code-sdk`           | `claude-agent-sdk`                |
| **Dokumentationsort** | Claude Code-Dokumentation   | API-Leitfaden → Agent SDK-Bereich |

<Note>
  **Dokumentationsänderungen:** Die Agent SDK-Dokumentation wurde aus der Claude Code-Dokumentation in den API-Leitfaden unter einem dedizierten [Agent SDK](/docs/de/agent-sdk/overview)-Bereich verschoben. Die Claude Code-Dokumentation konzentriert sich nun auf das CLI-Tool und Automatisierungsfunktionen.
</Note>

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

**4. Aktualisieren Sie die package.json-Abhängigkeiten:**

Wenn Sie das Paket in Ihrer `package.json` aufgelistet haben, aktualisieren Sie es:

Vorher:

```json theme={null}
{
  "dependencies": {
    "@anthropic-ai/claude-code": "^0.0.42"
  }
}
```

Nachher:

```json theme={null}
{
  "dependencies": {
    "@anthropic-ai/claude-agent-sdk": "^0.2.0"
  }
}
```

**5. Überprüfen Sie [Breaking Changes](#breaking-changes)**

Nehmen Sie alle erforderlichen Codeänderungen vor, um die Migration abzuschließen.

<h3 id="for-python-projects">
  Für Python-Projekte
</h3>

**1. Deinstallieren Sie das alte Paket:**

```bash theme={null}
pip uninstall claude-code-sdk
```

**2. Installieren Sie das neue Paket:**

```bash theme={null}
pip install claude-agent-sdk
```

**3. Aktualisieren Sie Ihre Importe:**

Ändern Sie alle Importe von `claude_code_sdk` zu `claude_agent_sdk`:

```python theme={null}
# Vorher
from claude_code_sdk import query, ClaudeCodeOptions

# Nachher
from claude_agent_sdk import query, ClaudeAgentOptions
```

**4. Aktualisieren Sie die Typnamen:**

Ändern Sie `ClaudeCodeOptions` zu `ClaudeAgentOptions`:

```python theme={null}
# Vorher
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7")

# Nachher
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7")
```

**5. Überprüfen Sie [Breaking Changes](#breaking-changes)**

Nehmen Sie alle erforderlichen Codeänderungen vor, um die Migration abzuschließen.

<h2 id="breaking-changes">
  Breaking Changes
</h2>

<Warning>
  Um die Isolation und explizite Konfiguration zu verbessern, führt Claude Agent SDK v0.1.0 Breaking Changes für Benutzer ein, die vom Claude Code SDK migrieren. Überprüfen Sie diesen Abschnitt sorgfältig vor der Migration.
</Warning>

<h3 id="python-claudecodeoptions-renamed-to-claudeagentoptions">
  Python: ClaudeCodeOptions in ClaudeAgentOptions umbenannt
</h3>

**Was hat sich geändert:** Der Python SDK-Typ `ClaudeCodeOptions` wurde in `ClaudeAgentOptions` umbenannt.

**Migration:**

```python theme={null}
# VORHER (claude-code-sdk)
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7", permission_mode="acceptEdits")

# NACHHER (claude-agent-sdk)
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7", permission_mode="acceptEdits")
```

**Warum sich das geändert hat:** Der Typname entspricht nun dem Branding „Claude Agent SDK" und bietet Konsistenz in den Namenskonventionen des SDKs.

<h3 id="system-prompt-no-longer-default">
  System-Prompt ist nicht mehr Standard
</h3>

**Was hat sich geändert:** Das SDK verwendet nicht mehr standardmäßig Claude Codes System-Prompt.

**Migration:**

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // VORHER (v0.0.x) - Verwendete Claude Codes System-Prompt standardmäßig
  const before = query({ prompt: "Hello" });

  // NACHHER (v0.1.0) - Verwendet standardmäßig minimalen System-Prompt
  // Um das alte Verhalten zu erhalten, fordern Sie explizit Claude Codes Voreinstellung an:
  const presetResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: { type: "preset", preset: "claude_code" }
    }
  });

  // Oder verwenden Sie einen benutzerdefinierten System-Prompt:
  const customResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: "You are a helpful coding assistant"
    }
  });
  ```

  ```python Python theme={null}
  # VORHER (v0.0.x) - Verwendete Claude Codes System-Prompt standardmäßig
  async for message in query(prompt="Hello"):
      print(message)

  # NACHHER (v0.1.0) - Verwendet standardmäßig minimalen System-Prompt
  # Um das alte Verhalten zu erhalten, fordern Sie explizit Claude Codes Voreinstellung an:
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(
          system_prompt={"type": "preset", "preset": "claude_code"}  # Verwenden Sie die Voreinstellung
      ),
  ):
      print(message)

  # Oder verwenden Sie einen benutzerdefinierten System-Prompt:
  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(system_prompt="You are a helpful coding assistant"),
  ):
      print(message)
  ```
</CodeGroup>

**Warum sich das geändert hat:** Bietet bessere Kontrolle und Isolation für SDK-Anwendungen. Sie können nun Agenten mit benutzerdefiniertem Verhalten erstellen, ohne Claude Codes CLI-fokussierte Anweisungen zu erben.

<h3 id="settings-sources-default">
  Einstellungsquellen-Standard
</h3>

Dieser Standard wurde kurzzeitig in v0.1.0 geändert und dann rückgängig gemacht, daher ist keine Migrationsaktion erforderlich.

**Aktuelles Verhalten:** Das Weglassen von `settingSources` auf `query()` lädt Benutzer-, Projekt- und lokale Dateisystem-Einstellungen, was dem CLI entspricht. Dies umfasst `~/.claude/settings.json`, `.claude/settings.json`, `.claude/settings.local.json`, CLAUDE.md-Dateien und benutzerdefinierte Befehle.

Um isoliert von Dateisystem-Einstellungen zu laufen, übergeben Sie ein leeres Array:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const isolatedResult = query({
    prompt: "Hello",
    options: {
      settingSources: [] // Keine Dateisystem-Einstellungen geladen
    }
  });

  // Oder laden Sie nur bestimmte Quellen:
  const projectOnlyResult = query({
    prompt: "Hello",
    options: {
      settingSources: ["project"] // Nur Projekteinstellungen
    }
  });
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(setting_sources=[]),  # Keine Dateisystem-Einstellungen geladen
  ):
      print(message)

  # Oder laden Sie nur bestimmte Quellen:
  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(
          setting_sources=["project"]  # Nur Projekteinstellungen
      ),
  ):
      print(message)
  ```
</CodeGroup>

Isolation ist besonders wichtig für CI/CD-Pipelines, bereitgestellte Anwendungen, Testumgebungen und Multi-Tenant-Systeme, in denen lokale Anpassungen nicht eindringen sollten.

<Note>
  SDK v0.1.0 standardmäßig kurzzeitig auf keine geladenen Einstellungen; dies wurde in nachfolgenden Versionen rückgängig gemacht. Python SDK 0.1.59 und früher behandelten eine leere Liste genauso wie das Weglassen der Option, daher aktualisieren Sie vor dem Verlassen auf `setting_sources=[]`. Siehe [Was settingSources nicht kontrolliert](/docs/de/agent-sdk/claude-code-features#what-settingsources-does-not-control) für Eingaben, die auch gelesen werden, wenn `settingSources` `[]` ist.
</Note>

<h2 id="why-the-rename">
  Warum die Umbenennung?
</h2>

Das Claude Code SDK wurde ursprünglich für Codierungsaufgaben entwickelt, hat sich aber zu einem leistungsstarken Framework für die Erstellung aller Arten von KI-Agenten entwickelt. Der neue Name „Claude Agent SDK" spiegelt seine Funktionen besser wider:

* Erstellung von Business-Agenten (Rechtsassistenten, Finanzberater, Kundensupport)
* Erstellung spezialisierter Codierungs-Agenten (SRE-Bots, Sicherheitsprüfer, Code-Review-Agenten)
* Entwicklung benutzerdefinierter Agenten für jede Domäne mit Tool-Nutzung, MCP-Integration und mehr

<h2 id="getting-help">
  Hilfe erhalten
</h2>

Wenn Sie während der Migration auf Probleme stoßen:

**Für TypeScript/JavaScript:**

1. Überprüfen Sie, dass alle Importe aktualisiert wurden, um `@anthropic-ai/claude-agent-sdk` zu verwenden
2. Überprüfen Sie, dass Ihre package.json den neuen Paketnamen hat
3. Führen Sie `npm install` aus, um sicherzustellen, dass die Abhängigkeiten aktualisiert werden

**Für Python:**

1. Überprüfen Sie, dass alle Importe aktualisiert wurden, um `claude_agent_sdk` zu verwenden
2. Überprüfen Sie, dass Ihre requirements.txt oder pyproject.toml den neuen Paketnamen hat
3. Führen Sie `pip install claude-agent-sdk` aus, um sicherzustellen, dass das Paket installiert ist

<h2 id="next-steps">
  Nächste Schritte
</h2>

* Erkunden Sie die [Agent SDK-Übersicht](/docs/de/agent-sdk/overview), um mehr über verfügbare Funktionen zu erfahren
* Schauen Sie sich die [TypeScript SDK-Referenz](/docs/de/agent-sdk/typescript) für detaillierte API-Dokumentation an
* Überprüfen Sie die [Python SDK-Referenz](/docs/de/agent-sdk/python) für Python-spezifische Dokumentation
* Erfahren Sie mehr über [Benutzerdefinierte Tools](/docs/de/agent-sdk/custom-tools) und [MCP-Integration](/docs/de/agent-sdk/mcp)
