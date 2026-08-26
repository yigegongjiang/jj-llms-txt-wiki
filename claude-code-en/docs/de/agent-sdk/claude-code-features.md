> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code-Funktionen im SDK verwenden

> Laden Sie Projektanweisungen, Skills, Hooks und andere Claude Code-Funktionen in Ihre SDK-Agenten.

Das Agent SDK basiert auf der gleichen Grundlage wie Claude Code, was bedeutet, dass Ihre SDK-Agenten Zugriff auf die gleichen dateisystemgestützten Funktionen haben: Projektanweisungen (`CLAUDE.md` und Regeln), Skills, Hooks und mehr.

Wenn Sie `settingSources` weglassen, liest `query()` die gleichen Dateisystemeinstellungen wie die Claude Code CLI: Benutzer-, Projekt- und lokale Einstellungen, CLAUDE.md-Dateien und `.claude/`-Skills, Agenten und Befehle. Um ohne diese auszuführen, übergeben Sie `settingSources: []`, was den Agenten auf das beschränkt, was Sie programmgesteuert konfigurieren. Verwaltete Richtlinieneinstellungen und die globale `~/.claude.json`-Konfiguration werden unabhängig von dieser Option gelesen. Siehe [Was settingSources nicht kontrolliert](#what-settingsources-does-not-control).

Für einen konzeptionellen Überblick über das, was jede Funktion tut und wann sie verwendet werden sollte, siehe [Claude Code erweitern](/docs/de/features-overview).

<h2 id="control-filesystem-settings-with-settingsources">
  Dateisystemeinstellungen mit settingSources kontrollieren
</h2>

Die Einstellungsquellen-Option ([`setting_sources`](/docs/de/agent-sdk/python#claudeagentoptions) in Python, [`settingSources`](/docs/de/agent-sdk/typescript#settingsource) in TypeScript) kontrolliert, welche dateisystemgestützten Einstellungen das SDK lädt. Übergeben Sie eine explizite Liste, um sich für bestimmte Quellen anzumelden, oder übergeben Sie ein leeres Array, um Benutzer-, Projekt- und lokale Einstellungen zu deaktivieren.

Dieses Beispiel lädt sowohl Benutzer- als auch Projektebenen-Einstellungen, indem `settingSources` auf `["user", "project"]` gesetzt wird:

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

Jede Quelle lädt Einstellungen von einem bestimmten Ort, wobei `<cwd>` das Arbeitsverzeichnis ist, das Sie über die `cwd`-Option übergeben, oder das aktuelle Verzeichnis des Prozesses, falls nicht gesetzt. Für die vollständige Typdefinition siehe [`SettingSource`](/docs/de/agent-sdk/typescript#settingsource) (TypeScript) oder [`SettingSource`](/docs/de/agent-sdk/python#settingsource) (Python).

| Quelle      | Was wird geladen                                                                                | Ort                                                                                                                                                                                                  |
| :---------- | :---------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"project"` | Projekt-CLAUDE.md, `.claude/rules/*.md`, Projekt-Skills, Projekt-Hooks, Projekt-`settings.json` | `<cwd>/.claude/` für `settings.json` und Hooks; `<cwd>` und jedes übergeordnete Verzeichnis für CLAUDE.md und Regeln; `<cwd>` und jedes übergeordnete Verzeichnis bis zur Repository-Root für Skills |
| `"user"`    | Benutzer-CLAUDE.md, `~/.claude/rules/*.md`, Benutzer-Skills, Benutzereinstellungen              | `~/.claude/`                                                                                                                                                                                         |
| `"local"`   | CLAUDE.local.md, `.claude/settings.local.json`                                                  | `<cwd>/.claude/` für `settings.local.json`; `<cwd>` und jedes übergeordnete Verzeichnis für CLAUDE.local.md                                                                                          |

Das Weglassen von `settingSources` entspricht `["user", "project", "local"]`.

Die `cwd`-Option bestimmt, wo das SDK nach Projekteinstellungen sucht. CLAUDE.md und Regeln werden aus `<cwd>` und aus jedem übergeordneten Verzeichnis geladen. Skills werden aus `<cwd>` und aus jedem übergeordneten Verzeichnis bis zur Repository-Root geladen. Projekt-`settings.json` und Hooks werden nur aus `<cwd>/.claude/` geladen, ohne Fallback für übergeordnete Verzeichnisse.

<h3 id="what-settingsources-does-not-control">
  Was settingSources nicht kontrolliert
</h3>

`settingSources` umfasst Benutzer-, Projekt- und lokale Einstellungen. Einige Eingaben werden unabhängig von ihrem Wert gelesen:

| Eingabe                                                               | Verhalten                                                                                                                                                                                                                                                                                                                                                                                                                                    | Zum Deaktivieren                                                                                                                                                                                                                                      |
| :-------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Verwaltete Richtlinieneinstellungen                                   | Endpunkt-verwaltete Richtlinie, wie eine MDM-Plist, Registrierungsrichtlinie oder verwaltete Einstellungsdatei, wird vom Host geladen. [Server-verwaltete Einstellungen](/docs/de/server-managed-settings) werden auf einer [zulässigen Konfiguration](/docs/de/server-managed-settings#platform-availability) abgerufen, wenn sich die Sitzung mit einer Organisations-OAuth-Anmeldung oder einem direkt konfigurierten API-Schlüssel authentifiziert | Endpunktrichtlinie: Entfernen Sie die verwaltete Einstellungsdatei, Plist oder Registrierungsrichtlinie vom Host. Server-verwaltete Einstellungen: werden von Ihrem Organisations-Administrator kontrolliert; können nicht vom SDK deaktiviert werden |
| `~/.claude.json` globale Konfiguration                                | Immer gelesen                                                                                                                                                                                                                                                                                                                                                                                                                                | Verschieben Sie mit `CLAUDE_CONFIG_DIR` in `env`                                                                                                                                                                                                      |
| Automatisches Gedächtnis unter `~/.claude/projects/<project>/memory/` | Wird beim Sitzungsstart in die Systemaufforderung geladen. Der Agent schreibt neue Erinnerungen dort mit den Standard-Tools `Write` und `Edit` statt mit einem dedizierten Speichertool, daher müssen diese Tools aktiviert sein, damit der Agent Erinnerungen speichern kann                                                                                                                                                                | Setzen Sie `autoMemoryEnabled: false` in Einstellungen oder `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` in `env`                                                                                                                                              |
| [claude.ai MCP-Konnektoren](/docs/de/mcp#use-mcp-servers-from-claude-ai)   | Geladen, wenn die aktive Authentifizierungsmethode ein claude.ai-Abonnement ist. Das Übergeben von `mcpServers: {}` unterdrückt sie nicht                                                                                                                                                                                                                                                                                                    | Setzen Sie `strictMcpConfig: true`, [`disableClaudeAiConnectors: true`](/docs/de/mcp#disable-claude-ai-connectors) in Einstellungen oder `ENABLE_CLAUDEAI_MCP_SERVERS=false` in `env`                                                                      |

<Warning>
  Verlassen Sie sich nicht auf Standard-`query()`-Optionen für Multi-Tenant-Isolation. Da die obigen Eingaben unabhängig von `settingSources` gelesen werden, kann ein SDK-Prozess Host-Level-Konfiguration und Pro-Verzeichnis-Speicher aufgreifen. Für Multi-Tenant-Bereitstellungen führen Sie jeden Mandanten in seinem eigenen Dateisystem aus und setzen Sie `settingSources: []` plus `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1` in `env`. [Server-verwaltete Einstellungen](/docs/de/server-managed-settings) werden abgerufen, wenn sich der Prozess mit einer Organisations-Anmeldedaten authentifiziert; Dateisystem-Isolation entfernt sie nicht. Siehe [Sichere Bereitstellung](/docs/de/agent-sdk/secure-deployment).
</Warning>

<h2 id="project-instructions-claude-md-and-rules">
  Projektanweisungen (CLAUDE.md und Regeln)
</h2>

`CLAUDE.md`-Dateien und `.claude/rules/*.md`-Dateien geben Ihrem Agenten persistenten Kontext über Ihr Projekt: Codierungskonventionen, Build-Befehle, Architekturentscheidungen und Anweisungen. Wenn `settingSources` `"project"` enthält (wie im obigen Beispiel), lädt das SDK diese Dateien beim Sitzungsstart in den Kontext. Der Agent folgt dann Ihren Projektkonventionen, ohne dass Sie sie in jedem Prompt wiederholen müssen.

<h3 id="claude-md-load-locations">
  CLAUDE.md-Ladeorte
</h3>

| Ebene                                 | Ort                                                                                     | Wann geladen                                                                                                  |
| :------------------------------------ | :-------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------ |
| Projekt (Wurzel)                      | `<cwd>/CLAUDE.md` oder `<cwd>/.claude/CLAUDE.md`                                        | `settingSources` enthält `"project"`                                                                          |
| Projektregeln                         | `<cwd>/.claude/rules/*.md` und `.claude/rules/*.md` in jedem übergeordneten Verzeichnis | `settingSources` enthält `"project"`                                                                          |
| Projekt (übergeordnete Verzeichnisse) | `CLAUDE.md`-Dateien in Verzeichnissen über `cwd`                                        | `settingSources` enthält `"project"`, geladen beim Sitzungsstart                                              |
| Projekt (Unterverzeichnisse)          | `CLAUDE.md`-Dateien in Unterverzeichnissen von `cwd`                                    | `settingSources` enthält `"project"`, bei Bedarf geladen, wenn der Agent eine Datei in diesem Unterbaum liest |
| Lokal                                 | `<cwd>/CLAUDE.local.md` und `CLAUDE.local.md` in jedem übergeordneten Verzeichnis       | `settingSources` enthält `"local"`                                                                            |
| Benutzer                              | `~/.claude/CLAUDE.md`                                                                   | `settingSources` enthält `"user"`                                                                             |
| Benutzerregeln                        | `~/.claude/rules/*.md`                                                                  | `settingSources` enthält `"user"`                                                                             |

Alle Ebenen sind additiv: Wenn sowohl Projekt- als auch Benutzer-CLAUDE.md-Dateien vorhanden sind, sieht der Agent beide. Es gibt keine harte Vorrangregel zwischen Ebenen; wenn Anweisungen in Konflikt geraten, hängt das Ergebnis davon ab, wie Claude sie interpretiert. Schreiben Sie nicht in Konflikt stehende Regeln, oder geben Sie den Vorrang explizit in der spezifischeren Datei an („Diese Projektanweisungen überschreiben alle in Konflikt stehenden Benutzer-Level-Standardwerte").

<Tip>
  Sie können auch Kontext direkt über `systemPrompt` injizieren, ohne CLAUDE.md-Dateien zu verwenden. Siehe [Systemaufforderungen ändern](/docs/de/agent-sdk/modifying-system-prompts). Verwenden Sie CLAUDE.md, wenn Sie den gleichen Kontext zwischen interaktiven Claude Code-Sitzungen und Ihren SDK-Agenten teilen möchten.
</Tip>

Informationen zur Strukturierung und Organisation von CLAUDE.md-Inhalten finden Sie unter [Claudes Speicher verwalten](/docs/de/memory).

<h2 id="skills">
  Skills
</h2>

Skills sind Markdown-Dateien, die Ihrem Agenten spezialisiertes Wissen und aufrufbare Workflows geben. Im Gegensatz zu `CLAUDE.md` (das jede Sitzung geladen wird) werden Skills bei Bedarf geladen. Der Agent erhält Skill-Beschreibungen beim Start und lädt den vollständigen Inhalt, wenn relevant.

Skills werden durch `settingSources` vom Dateisystem entdeckt. Wenn die `skills`-Option bei `query()` weggelassen wird, werden entdeckte Benutzer- und Projekt-Skills aktiviert und das Skill-Tool ist verfügbar, was dem CLI-Verhalten entspricht. Um zu steuern, welche Skills aktiviert sind, übergeben Sie `skills` als `"all"`, eine Liste von Skill-Namen oder `[]`, um alle zu deaktivieren. Wenn `skills` gesetzt ist, fügt das SDK das Skill-Tool automatisch zu `allowedTools` hinzu. Wenn Sie auch eine explizite `tools`-Liste übergeben, fügen Sie `"Skill"` in diese Liste ein, damit Claude Skills aufrufen kann.

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
  Skills müssen als Dateisystem-Artefakte erstellt werden (`.claude/skills/<name>/SKILL.md`). Das SDK hat keine programmgesteuerte API zum Registrieren von Skills. Siehe [Agent Skills im SDK](/docs/de/agent-sdk/skills) für vollständige Details.
</Note>

Weitere Informationen zum Erstellen und Verwenden von Skills finden Sie unter [Agent Skills im SDK](/docs/de/agent-sdk/skills).

<h2 id="hooks">
  Hooks
</h2>

Das SDK unterstützt zwei Möglichkeiten, Hooks zu definieren, und sie laufen nebeneinander:

* **Dateisystem-Hooks:** Shell-Befehle, die in `settings.json` definiert sind und geladen werden, wenn `settingSources` die relevante Quelle enthält. Dies sind die gleichen Hooks, die Sie für [interaktive Claude Code-Sitzungen](/docs/de/hooks-guide) konfigurieren würden.
* **Programmgesteuerte Hooks:** Callback-Funktionen, die direkt an `query()` übergeben werden. Diese laufen in Ihrem Anwendungsprozess und können strukturierte Entscheidungen zurückgeben. Siehe [Ausführung mit Hooks kontrollieren](/docs/de/agent-sdk/hooks).

Beide Typen werden während des gleichen Hook-Lebenszyklus ausgeführt. Wenn Sie bereits Hooks in der `.claude/settings.json` Ihres Projekts haben und Sie `settingSources: ["project"]` setzen, werden diese Hooks automatisch im SDK ohne zusätzliche Konfiguration ausgeführt.

Hook-Callbacks erhalten die Tool-Eingabe und geben ein Entscheidungs-Dict zurück. Das Zurückgeben von `{}` bedeutet, dass das Tool fortfahren darf. Um die Ausführung zu blockieren, geben Sie ein `hookSpecificOutput`-Objekt mit `permissionDecision: "deny"` und einem `permissionDecisionReason` zurück. Der Grund wird Claude als Tool-Ergebnis gesendet. Die Top-Level-Felder `decision` und `reason` sind für `PreToolUse` veraltet. Siehe das [Hooks-Handbuch](/docs/de/agent-sdk/hooks) für die vollständige Callback-Signatur und Rückgabetypen.

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
  Wann welcher Hook-Typ verwendet werden sollte
</h3>

| Hook-Typ                                       | Am besten für                                                                                                                                                                                                                                                                                                                                                                       |
| :--------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Dateisystem** (`settings.json`)              | Gemeinsame Nutzung von Hooks zwischen CLI- und SDK-Sitzungen. Unterstützt `"command"` (Shell-Skripte), `"http"` (POST an einen Endpunkt), `"mcp_tool"` (Aufrufen eines Tools eines verbundenen MCP-Servers), `"prompt"` (LLM wertet einen Prompt aus) und `"agent"` (spawnt einen Verifier-Agenten). Diese werden im Haupt-Agenten und allen Subagenten, die er spawnt, ausgeführt. |
| **Programmgesteuert** (Callbacks in `query()`) | Anwendungsspezifische Logik, strukturierte Entscheidungen und In-Process-Integration. Diese werden auch in Subagenten ausgeführt. Der Callback empfängt `agent_id` und `agent_type`, um zu unterscheiden.                                                                                                                                                                           |

<Note>
  Das TypeScript SDK unterstützt zusätzliche Hook-Events über Python hinaus, einschließlich `SessionStart`, `SessionEnd`, `TeammateIdle` und `TaskCompleted`. Siehe das [Hooks-Handbuch](/docs/de/agent-sdk/hooks) für die vollständige Ereigniskompatibilitätstabelle.
</Note>

Vollständige Details zu programmgesteuerten Hooks finden Sie unter [Ausführung mit Hooks kontrollieren](/docs/de/agent-sdk/hooks). Für Dateisystem-Hook-Syntax siehe [Hooks](/docs/de/hooks).

<h2 id="choose-the-right-feature">
  Wählen Sie die richtige Funktion
</h2>

Das Agent SDK gibt Ihnen Zugriff auf mehrere Möglichkeiten, das Verhalten Ihres Agenten zu erweitern. Wenn Sie unsicher sind, welche Sie verwenden sollten, ordnet diese Tabelle häufige Ziele dem richtigen Ansatz zu.

| Sie möchten...                                                                                               | Verwenden Sie                                      | SDK-Oberfläche                                                                                                                                                                     |
| :----------------------------------------------------------------------------------------------------------- | :------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Projektkonventionen festlegen, die Ihr Agent immer befolgt                                                   | [CLAUDE.md](/docs/de/memory)                            | `settingSources: ["project"]` lädt es automatisch                                                                                                                                  |
| Dem Agenten Referenzmaterial geben, das er bei Bedarf lädt                                                   | [Skills](/docs/de/agent-sdk/skills)                     | `settingSources` + `skills`-Option                                                                                                                                                 |
| Einen wiederverwendbaren Workflow ausführen (bereitstellen, überprüfen, freigeben)                           | [Benutzer-aufrufbare Skills](/docs/de/agent-sdk/skills) | `settingSources` + `skills`-Option                                                                                                                                                 |
| Eine isolierte Teilaufgabe an einen frischen Kontext delegieren (Recherche, Überprüfung)                     | [Subagenten](/docs/de/agent-sdk/subagents)              | `agents`-Parameter + `allowedTools: ["Agent"]`                                                                                                                                     |
| Mehrere Claude Code-Instanzen mit gemeinsamen Aufgabenlisten und direkter Inter-Agent-Messaging koordinieren | [Agent-Teams](/docs/de/agent-teams)                     | Nicht direkt über SDK-Optionen konfiguriert. Agent-Teams sind eine CLI-Funktion, bei der eine Sitzung als Team-Lead fungiert und die Arbeit über unabhängige Teammates koordiniert |
| Deterministische Logik auf Tool-Aufrufe ausführen (Audit, Block, Transform)                                  | [Hooks](/docs/de/agent-sdk/hooks)                       | `hooks`-Parameter mit Callbacks oder Shell-Skripte, die über `settingSources` geladen werden                                                                                       |
| Claude strukturierten Tool-Zugriff auf einen externen Service geben                                          | [MCP](/docs/de/agent-sdk/mcp)                           | `mcpServers`-Parameter                                                                                                                                                             |

<Tip>
  **Subagenten versus Agent-Teams:** Subagenten sind kurzlebig und isoliert: frische Konversation, eine Aufgabe, Zusammenfassung an übergeordnete Instanz zurückgegeben. Agent-Teams koordinieren mehrere unabhängige Claude Code-Instanzen, die eine Aufgabenliste teilen und sich direkt gegenseitig Nachrichten senden. Agent-Teams sind eine CLI-Funktion. Siehe [Was Subagenten erben](/docs/de/agent-sdk/subagents#what-subagents-inherit) und den [Agent-Teams-Vergleich](/docs/de/agent-teams#compare-with-subagents) für Details.
</Tip>

Jede Funktion, die Sie aktivieren, trägt zu Ihrem Agent-Kontextfenster bei. Für Pro-Funktion-Kosten und wie diese Funktionen zusammen funktionieren, siehe [Claude Code erweitern](/docs/de/features-overview#understand-context-costs).

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Claude Code erweitern](/docs/de/features-overview): Konzeptioneller Überblick über alle Erweiterungsfunktionen mit Vergleichstabellen und Kontextkostenanalyse
* [Skills im SDK](/docs/de/agent-sdk/skills): Vollständiges Handbuch zur programmgesteuerten Verwendung von Skills
* [Subagenten](/docs/de/agent-sdk/subagents): Definieren und rufen Sie Subagenten für isolierte Teilaufgaben auf
* [Hooks](/docs/de/agent-sdk/hooks): Abfangen und Kontrollieren des Agent-Verhaltens an wichtigen Ausführungspunkten
* [Berechtigungen](/docs/de/agent-sdk/permissions): Kontrollieren Sie Tool-Zugriff mit Modi, Regeln und Callbacks
* [Systemaufforderungen](/docs/de/agent-sdk/modifying-system-prompts): Injizieren Sie Kontext ohne CLAUDE.md-Dateien
