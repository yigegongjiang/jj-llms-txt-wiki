> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins-Referenz

> Vollständige technische Referenz für das Claude Code Plugin-System, einschließlich Schemas, CLI-Befehle und Komponentenspezifikationen.

<Tip>
  Möchten Sie Plugins installieren? Siehe [Plugins entdecken und installieren](/docs/de/discover-plugins). Zum Erstellen von Plugins siehe [Plugins](/docs/de/plugins). Zum Verteilen von Plugins siehe [Plugin-Marktplätze](/docs/de/plugin-marketplaces).
</Tip>

Diese Referenz bietet vollständige technische Spezifikationen für das Claude Code Plugin-System, einschließlich Komponentenschemas, CLI-Befehle und Entwicklungstools.

Ein **Plugin** ist ein eigenständiges Verzeichnis von Komponenten, das Claude Code mit benutzerdefinierten Funktionen erweitert. Plugin-Komponenten umfassen Skills, Agents, Hooks, MCP-Server, LSP-Server und Monitore.

<h2 id="plugin-components-reference">
  Plugin-Komponenten-Referenz
</h2>

<h3 id="skills">
  Skills
</h3>

Plugins fügen Skills zu Claude Code hinzu und erstellen `/name` Verknüpfungen, die Sie oder Claude aufrufen können.

**Speicherort**: `skills/` oder `commands/` Verzeichnis im Plugin-Root, oder eine einzelne `SKILL.md` Datei im Plugin-Root

**Dateiformat**: Skills sind Verzeichnisse mit `SKILL.md`; Befehle sind einfache Markdown-Dateien

**Skill-Struktur**:

```text theme={null}
skills/
├── pdf-processor/
│   ├── SKILL.md
│   ├── reference.md (optional)
│   └── scripts/ (optional)
└── code-reviewer/
    └── SKILL.md
```

**Integrationverhalten**:

* Skills und Befehle werden automatisch erkannt, wenn das Plugin installiert wird
* Claude kann sie automatisch basierend auf dem Task-Kontext aufrufen
* Skills können unterstützende Dateien neben SKILL.md enthalten

Wenn ein Plugin kein `skills/` Verzeichnis und kein `skills` Manifest-Feld hat, wird eine `SKILL.md` im Plugin-Root als einzelner Skill geladen. Setzen Sie das Frontmatter-Feld `name`, um den Aufrufen-Namen des Skills zu steuern. Ohne dieses Feld greift Claude Code auf den Installationsverzeichnisnamen zurück, der bei vom Marktplatz installierten Plugins ein Versionsstring ist, der sich bei jedem Update ändert. Für Plugins, die mehr als einen Skill versenden, verwenden Sie das oben gezeigte `skills/` Verzeichnis-Layout.

Vollständige Details finden Sie unter [Skills](/docs/de/skills).

<h3 id="agents">
  Agents
</h3>

Plugins können spezialisierte Subagents für spezifische Aufgaben bereitstellen, die Claude automatisch aufrufen kann, wenn dies angemessen ist.

**Speicherort**: `agents/` Verzeichnis im Plugin-Root

**Dateiformat**: Markdown-Dateien, die Agent-Fähigkeiten beschreiben

**Agent-Struktur**:

```markdown theme={null}
---
name: agent-name
description: Worauf sich dieser Agent spezialisiert und wann Claude ihn aufrufen sollte
model: sonnet
effort: medium
maxTurns: 20
disallowedTools: Write, Edit
---

Detailliertes System-Prompt für den Agent, das seine Rolle, Expertise und sein Verhalten beschreibt.
```

Plugin-Agents unterstützen `name`, `description`, `model`, `effort`, `maxTurns`, `tools`, `disallowedTools`, `skills`, `memory`, `background` und `isolation` Frontmatter-Felder. Der einzige gültige `isolation` Wert ist `"worktree"`. Aus Sicherheitsgründen werden `hooks`, `mcpServers` und `permissionMode` für von Plugins bereitgestellte Agents nicht unterstützt.

**Integrationspunkte**:

* Agents erscheinen in der [@-mention Typeahead](/docs/de/sub-agents#invoke-subagents-explicitly) unter ihrem scoped Namen, wie `my-plugin:code-reviewer`, sobald das Plugin aktiviert ist
* Claude kann Agents automatisch basierend auf dem Task-Kontext aufrufen
* Agents können manuell von Benutzern aufgerufen werden
* Plugin-Agents funktionieren neben integrierten Claude-Agents

Vollständige Details finden Sie unter [Subagents](/docs/de/sub-agents).

<h3 id="hooks">
  Hooks
</h3>

Plugins können Event-Handler bereitstellen, die automatisch auf Claude Code Events reagieren.

**Speicherort**: `hooks/hooks.json` im Plugin-Root oder inline in plugin.json

**Format**: JSON-Konfiguration mit Event-Matchern und Aktionen

**Hook-Konfiguration**:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/format-code.sh"
          }
        ]
      }
    ]
  }
}
```

Plugin-Hooks reagieren auf die gleichen Lifecycle-Events wie [benutzerdefinierte Hooks](/docs/de/hooks):

| Event                 | When it fires                                                                                                                                                                                                                                         |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SessionStart`        | When a session begins or resumes                                                                                                                                                                                                                      |
| `Setup`               | When you start Claude Code with `--init-only`, or with `--init` or `--maintenance` in `-p` mode. For one-time preparation in CI or scripts                                                                                                            |
| `UserPromptSubmit`    | When you submit a prompt, before Claude processes it                                                                                                                                                                                                  |
| `UserPromptExpansion` | When a user-typed command expands into a prompt, before it reaches Claude. Can block the expansion                                                                                                                                                    |
| `PreToolUse`          | Before a tool call executes. Can block it                                                                                                                                                                                                             |
| `PermissionRequest`   | When a tool call needs a permission decision                                                                                                                                                                                                          |
| `PermissionDenied`    | When auto mode denies a tool call, including denials without a classifier verdict. Use JSON `hookSpecificOutput.retry: true` to tell the model it may retry the denied tool call. Claude Code ignores `retry` when the classifier produced no verdict |
| `PostToolUse`         | After a tool call succeeds                                                                                                                                                                                                                            |
| `PostToolUseFailure`  | After a tool call fails                                                                                                                                                                                                                               |
| `PostToolBatch`       | After a full batch of parallel tool calls resolves, before the next model call                                                                                                                                                                        |
| `Notification`        | When Claude Code sends a notification                                                                                                                                                                                                                 |
| `MessageDisplay`      | While assistant message text is displayed                                                                                                                                                                                                             |
| `SubagentStart`       | When a subagent is spawned                                                                                                                                                                                                                            |
| `SubagentStop`        | When a subagent finishes                                                                                                                                                                                                                              |
| `TaskCreated`         | When a task is being created via `TaskCreate`                                                                                                                                                                                                         |
| `TaskCompleted`       | When a task is being marked as completed                                                                                                                                                                                                              |
| `Stop`                | When Claude finishes responding                                                                                                                                                                                                                       |
| `StopFailure`         | When the turn ends due to an API error                                                                                                                                                                                                                |
| `TeammateIdle`        | When an [agent team](/docs/en/agent-teams) teammate is about to go idle                                                                                                                                                                                    |
| `InstructionsLoaded`  | When a CLAUDE.md or `.claude/rules/*.md` file is loaded into context. Fires at session start and when files are lazily loaded during a session                                                                                                        |
| `ConfigChange`        | When a configuration file changes during a session                                                                                                                                                                                                    |
| `CwdChanged`          | When the working directory changes, for example when Claude executes a `cd` command. Useful for reactive environment management with tools like direnv                                                                                                |
| `DirectoryAdded`      | When a working directory is added mid-session via `/add-dir` or the SDK `register_repo_root` control request                                                                                                                                          |
| `FileChanged`         | When a watched file changes on disk. The `matcher` field specifies which filenames to watch                                                                                                                                                           |
| `WorktreeCreate`      | When a worktree is being created via `--worktree`, `isolation: "worktree"`, or for a background session. Replaces default git behavior                                                                                                                |
| `WorktreeRemove`      | When a worktree is being removed at session exit, when a subagent finishes, or when you delete a background session                                                                                                                                   |
| `PreCompact`          | Before context compaction                                                                                                                                                                                                                             |
| `PostCompact`         | After context compaction completes                                                                                                                                                                                                                    |
| `Elicitation`         | When an MCP server requests user input during a tool call                                                                                                                                                                                             |
| `ElicitationResult`   | After a user responds to an MCP elicitation, before the response is sent back to the server                                                                                                                                                           |
| `SessionEnd`          | When a session terminates                                                                                                                                                                                                                             |

**Hook-Typen**:

* `command`: Shell-Befehle oder Skripte ausführen
* `http`: Das Event JSON als POST-Anfrage an eine URL senden
* `mcp_tool`: Ein Tool auf einem konfigurierten [MCP-Server](/docs/de/mcp) aufrufen
* `prompt`: Ein Prompt mit einem LLM evaluieren (verwendet `$ARGUMENTS` Platzhalter für Kontext)
* `agent`: Einen agentic Verifier mit Tools für komplexe Verifikationsaufgaben ausführen

Hooks, die auf den eigenen [gebündelten MCP-Server](#mcp-servers) des Plugins abzielen, müssen seine scoped Namen verwenden. Tool-Matcher und `if` Felder verwenden den scoped Tool-Namen `mcp__plugin_<plugin-name>_<server-name>__<tool>`, und das `mcp_tool` Hook-Feld `server` verwendet `plugin:<plugin-name>:<server-name>`. Ein Matcher, der gegen den bloßen Server-Schlüssel geschrieben wird, wird nie ausgelöst. Siehe [Match MCP tools](/docs/de/hooks#match-mcp-tools) und [Plugin-bereitgestellte MCP-Server](/docs/de/mcp#plugin-provided-mcp-servers).

<h3 id="mcp-servers">
  MCP-Server
</h3>

Plugins können Model Context Protocol (MCP) Server bündeln, um Claude Code mit externen Tools und Services zu verbinden.

**Speicherort**: `.mcp.json` im Plugin-Root oder inline in plugin.json

**Format**: Standard MCP-Server-Konfiguration

**MCP-Server-Konfiguration**:

```json theme={null}
{
  "mcpServers": {
    "plugin-database": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"],
      "env": {
        "DB_PATH": "${CLAUDE_PLUGIN_ROOT}/data"
      }
    },
    "plugin-api-client": {
      "command": "npx",
      "args": ["@company/mcp-server", "--plugin-mode"]
    }
  }
}
```

**Integrationverhalten**:

* Plugin MCP-Server starten automatisch, wenn das Plugin aktiviert wird
* Server erscheinen als Standard MCP-Tools in Claudes Toolkit
* Server-Fähigkeiten integrieren sich nahtlos mit Claudes vorhandenen Tools
* Plugin-Server können unabhängig von Benutzer MCP-Servern konfiguriert werden

<h3 id="lsp-servers">
  LSP-Server
</h3>

<Tip>
  Möchten Sie LSP-Plugins verwenden? Installieren Sie sie vom offiziellen Marktplatz: Suchen Sie nach „lsp" im `/plugin` Discover-Tab. Dieser Abschnitt dokumentiert, wie Sie LSP-Plugins für Sprachen erstellen, die nicht vom offiziellen Marktplatz abgedeckt werden.
</Tip>

Plugins können [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) (LSP) Server bereitstellen, um Claude Echtzeit-Code-Intelligenz beim Arbeiten an Ihrer Codebasis zu geben.

LSP-Integration bietet:

* **Sofortige Diagnose**: Claude sieht Fehler und Warnungen sofort nach jeder Bearbeitung
* **Code-Navigation**: Gehe zu Definition, finde Referenzen und Hover-Informationen
* **Sprachbewusstsein**: Typinformationen und Dokumentation für Code-Symbole

**Speicherort**: `.lsp.json` im Plugin-Root oder inline in `plugin.json`

**Format**: JSON-Konfiguration, die Language Server Namen ihren Konfigurationen zuordnet

**`.lsp.json` Dateiformat**:

```json theme={null}
{
  "go": {
    "command": "gopls",
    "args": ["serve"],
    "extensionToLanguage": {
      ".go": "go"
    }
  }
}
```

**Inline in `plugin.json`**:

```json theme={null}
{
  "name": "my-plugin",
  "lspServers": {
    "go": {
      "command": "gopls",
      "args": ["serve"],
      "extensionToLanguage": {
        ".go": "go"
      }
    }
  }
}
```

**Erforderliche Felder:**

| Feld                  | Beschreibung                                         |
| :-------------------- | :--------------------------------------------------- |
| `command`             | Die auszuführende LSP-Binärdatei (muss in PATH sein) |
| `extensionToLanguage` | Ordnet Dateierweiterungen Sprachbezeichnern zu       |

**Optionale Felder:**

| Feld                    | Beschreibung                                                                                                                                                                                                  |
| :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `args`                  | Befehlszeilenargumente für den LSP-Server                                                                                                                                                                     |
| `transport`             | Kommunikationstransport: `stdio` (Standard) oder `socket`                                                                                                                                                     |
| `env`                   | Umgebungsvariablen, die beim Starten des Servers gesetzt werden                                                                                                                                               |
| `initializationOptions` | Optionen, die während der Initialisierung an den Server übergeben werden                                                                                                                                      |
| `settings`              | Einstellungen, die über `workspace/didChangeConfiguration` übergeben werden                                                                                                                                   |
| `workspaceFolder`       | Workspace-Ordnerpfad für den Server                                                                                                                                                                           |
| `startupTimeout`        | Maximale Zeit zum Warten auf Server-Startup (Millisekunden)                                                                                                                                                   |
| `shutdownTimeout`       | Maximale Zeit zum Warten auf ordnungsgemäßes Herunterfahren (Millisekunden). Wenn das Timeout abläuft, beendet Claude Code den Server-Prozess. Wenn nicht gesetzt, gilt kein Timeout                          |
| `restartOnCrash`        | Ob der Server nach einem Absturz neu gestartet werden soll. Standard ist `true`. Setzen Sie auf `false`, um einen abgestürzten Server gestoppt zu lassen, anstatt ihn neu zu starten                          |
| `maxRestarts`           | Maximale Anzahl von Neustartversuchen, bevor aufgegeben wird                                                                                                                                                  |
| `diagnostics`           | Ob Diagnosen nach Bearbeitungen in Claudes Kontext eingefügt werden sollen (Standard `true`). Setzen Sie auf `false`, um Code-Navigation beizubehalten, aber automatische Diagnose-Injektion zu unterdrücken. |

`restartOnCrash` und `shutdownTimeout` erfordern Claude Code v2.1.205 oder später. Vor v2.1.205 akzeptierte das Konfigurationsschema beide Optionen, aber das Setzen einer dieser Optionen führte dazu, dass Claude Code diesen LSP-Server beim Startup vollständig übersprungen hat, wobei der Grund nur in der `claude --debug` Ausgabe sichtbar war.

**Mehrere Server für die gleiche Erweiterung**: Wenn mehr als ein aktivierter LSP-Server die gleiche Dateierweiterung in `extensionToLanguage` deklariert, ob die Server von einem Plugin oder von verschiedenen Plugins stammen, verarbeitet der zuerst registrierte Server Dateien mit dieser Erweiterung und die anderen starten nie. Die `/plugin` Schnittstelle zeigt eine Warnung an, die das Plugin benennt, dessen Server aktiv ist.

**Server, die nicht initialisiert werden können**: Claude Code überspringt einen Server, dessen Konfiguration ungültig ist, zum Beispiel einer, dem `command` oder `extensionToLanguage` fehlt, und die anderen konfigurierten Server starten trotzdem. Führen Sie `claude --debug` aus, um zu sehen, warum ein Server übersprungen wurde.

Ein übersprungener Server beansprucht seine Dateierweiterungen nicht, daher kann ein anderer gültiger Server, der die gleiche Erweiterung deklariert, von demselben oder einem anderen Plugin, diese Dateien trotzdem verarbeiten. Vor v2.1.205 beanspruchte ein Server, der nicht initialisiert werden konnte, immer noch seine Erweiterungen und blockierte einen anderen gültigen Server für die gleiche Erweiterung.

<Warning>
  **Sie müssen die Language Server Binärdatei separat installieren.** LSP-Plugins konfigurieren, wie Claude Code sich mit einem Language Server verbindet, aber sie enthalten den Server selbst nicht. Wenn Sie `Executable not found in $PATH` im `/plugin` Errors-Tab sehen, installieren Sie die erforderliche Binärdatei für Ihre Sprache.
</Warning>

**Verfügbare LSP-Plugins:**

| Plugin              | Language Server            | Installationsbefehl                                                                          |
| :------------------ | :------------------------- | :------------------------------------------------------------------------------------------- |
| `pyright-lsp`       | Pyright (Python)           | `pip install pyright` oder `npm install -g pyright`                                          |
| `typescript-lsp`    | TypeScript Language Server | `npm install -g typescript-language-server typescript`                                       |
| `rust-analyzer-lsp` | rust-analyzer              | [Siehe rust-analyzer Installation](https://rust-analyzer.github.io/manual.html#installation) |

Installieren Sie zuerst den Language Server, dann installieren Sie das Plugin vom Marktplatz.

<h3 id="monitors">
  Monitore
</h3>

Plugins können Hintergrund-Monitore deklarieren, die Claude Code automatisch startet, wenn das Plugin aktiv ist. Jeder Monitor führt einen Shell-Befehl für die Lebensdauer der Session aus und liefert jede stdout-Zeile an Claude als Benachrichtigung, damit Claude auf Log-Einträge, Statusänderungen oder abgerufene Events reagieren kann, ohne aufgefordert zu werden, die Überwachung selbst zu starten.

Plugin-Monitore verwenden den gleichen Mechanismus wie das [Monitor-Tool](/docs/de/tools-reference#monitor-tool) und teilen seine Verfügbarkeitsbeschränkungen. Sie laufen nur in interaktiven CLI-Sessions, laufen unsandboxed auf der gleichen Vertrauensebene wie [Hooks](#hooks) und werden auf Hosts übersprungen, wo das Monitor-Tool nicht verfügbar ist.

**Speicherort**: `monitors/monitors.json` im Plugin-Root oder inline in `plugin.json`

**Format**: JSON-Array von Monitor-Einträgen

Die folgende `monitors/monitors.json` überwacht einen Deployment-Status-Endpunkt und ein lokales Error-Log:

```json theme={null}
[
  {
    "name": "deploy-status",
    "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/poll-deploy.sh",
    "description": "Deployment status changes"
  },
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log",
    "when": "on-skill-invoke:debug"
  }
]
```

Um Monitore inline zu deklarieren, setzen Sie `experimental.monitors` in `plugin.json` auf das gleiche Array. Um von einem nicht-Standard-Pfad zu laden, setzen Sie `experimental.monitors` auf einen relativen Pfad-String wie `"./config/monitors.json"`. Monitore sind eine [experimentelle Komponente](#experimental-components).

**Erforderliche Felder:**

| Feld          | Beschreibung                                                                                                                                     |
| :------------ | :----------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | Bezeichner eindeutig innerhalb des Plugins. Verhindert doppelte Prozesse, wenn das Plugin neu geladen wird oder ein Skill erneut aufgerufen wird |
| `command`     | Shell-Befehl, der als persistenter Hintergrund-Prozess im Session-Arbeitsverzeichnis ausgeführt wird                                             |
| `description` | Kurze Zusammenfassung dessen, was überwacht wird. Wird im Task-Panel und in Benachrichtigungszusammenfassungen angezeigt                         |

**Optionale Felder:**

| Feld   | Beschreibung                                                                                                                                                                                                                                    |
| :----- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `when` | Steuert, wann der Monitor startet. `"always"` startet ihn beim Session-Start und beim Plugin-Reload und ist der Standard. `"on-skill-invoke:<skill-name>"` startet ihn beim ersten Mal, wenn der benannte Skill in diesem Plugin versendet wird |

Der `command` Wert unterstützt die [Pfad-Substitutionen](#environment-variables) `${CLAUDE_PLUGIN_ROOT}`, `${CLAUDE_PLUGIN_DATA}` und `${CLAUDE_PROJECT_DIR}`, plus alle `${ENV_VAR}` aus der Umgebung. Stellen Sie dem Befehl `cd "${CLAUDE_PLUGIN_ROOT}" && ` voran, wenn das Skript aus dem Plugin-eigenen Verzeichnis ausgeführt werden muss.

Ein Monitor `command` kann nicht auf [`${user_config.*}`](#user-configuration) Werte verweisen. Der Befehl läuft durch eine Shell, daher lehnt Claude Code den Monitor mit einem [Fehler](/docs/de/errors#plugin-command-references-user-config) ab, anstatt den Wert zu ersetzen. Monitor-Prozesse erhalten keine `CLAUDE_PLUGIN_OPTION_<KEY>` Umgebungsvariablen, daher sollte das Monitor-Skript den Wert aus einer Konfigurationsdatei lesen, die es besitzt. Vor v2.1.207 ersetzten Monitor-Befehle `${user_config.*}` Werte.

Das Deaktivieren eines Plugins während einer Session stoppt nicht die Monitore, die bereits laufen. Sie stoppen, wenn die Session endet.

<h3 id="themes">
  Themes
</h3>

Plugins können Farbthemes versenden, die in `/theme` neben den integrierten Voreinstellungen und den lokalen Themes des Benutzers angezeigt werden. Ein Theme ist eine JSON-Datei in `themes/` mit einer `base` Voreinstellung und einer sparsamen `overrides` Map von Farb-Tokens. Themes sind eine [experimentelle Komponente](#experimental-components).

```json theme={null}
{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555",
    "success": "#50fa7b"
  }
}
```

Das Auswählen eines Plugin-Themes speichert `custom:<plugin-name>:<slug>` in der Konfiguration des Benutzers. Plugin-Themes sind schreibgeschützt; das Drücken von `Ctrl+E` auf einem in `/theme` kopiert es in `~/.claude/themes/`, damit der Benutzer die Kopie bearbeiten kann.

***

<h2 id="plugin-installation-scopes">
  Plugin-Installationsbereiche
</h2>

Wenn Sie ein Plugin installieren, wählen Sie einen **Bereich**, der bestimmt, wo das Plugin verfügbar ist und wer es sonst noch verwenden kann:

| Bereich   | Einstellungsdatei                                       | Anwendungsfall                                                        |
| :-------- | :------------------------------------------------------ | :-------------------------------------------------------------------- |
| `user`    | `~/.claude/settings.json`                               | Persönliche Plugins, die in allen Projekten verfügbar sind (Standard) |
| `project` | `.claude/settings.json`                                 | Team-Plugins, die über Versionskontrolle geteilt werden               |
| `local`   | `.claude/settings.local.json`                           | Projektspezifische Plugins, gitignoriert                              |
| `managed` | [Verwaltete Einstellungen](/docs/de/settings#settings-files) | Verwaltete Plugins (schreibgeschützt, nur aktualisierbar)             |

Plugins verwenden das gleiche Bereichssystem wie andere Claude Code Konfigurationen. Installationsanweisungen und Bereichs-Flags finden Sie unter [Plugins installieren](/docs/de/discover-plugins#install-plugins). Eine vollständige Erklärung der Bereiche finden Sie unter [Konfigurationsbereiche](/docs/de/settings#configuration-scopes).

***

<h2 id="skills-directory-plugins">
  Skills-Verzeichnis-Plugins
</h2>

Jeder Ordner unter einem Skills-Verzeichnis, das ein `.claude-plugin/plugin.json` Manifest enthält, wird beim nächsten Session als Plugin mit dem Namen `<name>@skills-dir` geladen, ohne Marktplatz und ohne Installationsschritt. Erstellen Sie einen mit [`plugin init`](#plugin-init). Im Gegensatz zu einer Marktplatz-Installation wird das Plugin an Ort und Stelle erkannt, anstatt in den Plugin-Cache kopiert zu werden.

Ein Skills-Verzeichnis-Baum unterstützt drei unterschiedliche Dinge:

| Was Sie haben                                 | Was es ist                                                                                 |
| :-------------------------------------------- | :----------------------------------------------------------------------------------------- |
| `<skills-dir>/foo/SKILL.md` ohne Manifest     | Ein einfacher [Skill](/docs/de/skills) mit dem Namen `foo`                                      |
| `<skills-dir>/foo/.claude-plugin/plugin.json` | Ein Plugin `foo@skills-dir`, das seine eigenen Skills, Agents, Hooks und mehr bündeln kann |
| `<plugin>/skills/bar/SKILL.md`                | Ein Skill `bar` verpackt in einem Plugin                                                   |

<h3 id="choose-where-the-plugin-loads-from">
  Wählen Sie, von wo das Plugin geladen wird
</h3>

| Skills-Verzeichnis      | Bereich    | Lädt                                                                                     |
| :---------------------- | :--------- | :--------------------------------------------------------------------------------------- |
| `~/.claude/skills/`     | persönlich | In jedem Projekt, da der Speicherort nur Ihnen gehört                                    |
| `<cwd>/.claude/skills/` | Projekt    | Nur nachdem Sie den Workspace [Trust-Dialog](/docs/de/settings) für diesen Ordner akzeptieren |

Ein Projekt-Bereich-Plugin wird in das Repository eingecheckt und erreicht jeden Mitarbeiter, der es klont. Da dieser Inhalt aus dem Repository und nicht von Ihnen stammt, wird er nur nach dem gleichen Trust-Gate geladen, das `.claude/settings.json` regelt, und Komponenten, die Code ausführen, sind weiter eingeschränkt:

* MCP-Server, die es deklariert, durchlaufen die [gleiche Pro-Server-Genehmigung](/docs/de/mcp) wie ein Projekt `.mcp.json`
* LSP-Server starten nur, nachdem Sie dem Workspace vertrauen
* [Hintergrund-Monitore](#monitors) werden nicht geladen

Persönliche Bereich-Plugins haben keine dieser Einschränkungen.

<Warning>
  Projekt-Bereich `@skills-dir` Plugins werden nur aus dem `.claude/skills/` des Verzeichnisses geladen, in dem Sie Claude Code starten. Sie [gehen nicht bis zur Repository-Root](/docs/de/skills#automatic-discovery-from-parent-and-nested-directories) wie einfache Skills und Befehle, daher verpasst das Starten aus einem Unterverzeichnis ein Plugin, das im Repo-Root lebt. Starten Sie vom Repository-Root, oder führen Sie `/reload-plugins` aus, nachdem Sie Verzeichnisse wechseln.
</Warning>

<h3 id="edit-reload-and-disable-a-skills-directory-plugin">
  Bearbeiten, neu laden und deaktivieren Sie ein Skills-Verzeichnis-Plugin
</h3>

Änderungen, die Sie an der `SKILL.md` eines Skills vornehmen, treten sofort in der aktuellen Session in Kraft. Änderungen an den anderen Komponenten des Plugins, wie `hooks/`, `.mcp.json`, `agents/` und `output-styles/`, tun dies nicht. Führen Sie `/reload-plugins` aus oder starten Sie Claude Code neu, um diese zu übernehmen. Siehe [Live-Änderungserkennung](/docs/de/skills#live-change-detection).

Um das Laden eines Skills-Verzeichnis-Plugins zu stoppen, löschen Sie seinen Ordner oder deaktivieren Sie ihn nach Name. Es gibt keinen `uninstall` Schritt, da nichts von einem Marktplatz installiert wurde.

```bash theme={null}
claude plugin disable my-tool@skills-dir
```

***

<h2 id="plugin-manifest-schema">
  Plugin-Manifest-Schema
</h2>

Die `.claude-plugin/plugin.json` Datei definiert die Metadaten und Konfiguration Ihres Plugins. Dieser Abschnitt dokumentiert alle unterstützten Felder und Optionen.

Das Manifest ist optional. Wenn es weggelassen wird, erkennt Claude Code Komponenten automatisch in [Standardspeicherorten](#file-locations-reference) und leitet den Plugin-Namen aus dem Verzeichnisnamen ab. Verwenden Sie ein Manifest, wenn Sie Metadaten oder benutzerdefinierte Komponentenpfade bereitstellen müssen.

<h3 id="complete-schema">
  Vollständiges Schema
</h3>

```json theme={null}
{
  "name": "plugin-name",
  "displayName": "Plugin Name",
  "version": "1.2.0",
  "description": "Brief plugin description",
  "author": {
    "name": "Author Name",
    "email": "author@example.com",
    "url": "https://github.com/author"
  },
  "homepage": "https://docs.example.com/plugin",
  "repository": "https://github.com/author/plugin",
  "license": "MIT",
  "keywords": ["keyword1", "keyword2"],
  "skills": "./custom/skills/",
  "commands": ["./custom/commands/special.md"],
  "agents": ["./custom/agents/reviewer.md"],
  "hooks": "./config/hooks.json",
  "mcpServers": "./mcp-config.json",
  "outputStyles": "./styles/",
  "lspServers": "./.lsp.json",
  "experimental": {
    "themes": "./themes/",
    "monitors": "./monitors.json"
  },
  "dependencies": [
    "helper-lib",
    { "name": "secrets-vault", "version": "~2.1.0" }
  ]
}
```

<h3 id="required-fields">
  Erforderliche Felder
</h3>

Wenn Sie ein Manifest einschließen, ist `name` das einzige erforderliche Feld.

| Feld   | Typ    | Beschreibung                                                                                                                                                                                                                                                                   | Beispiel             |
| :----- | :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------- |
| `name` | string | Eindeutiger Bezeichner (kebab-case, keine Leerzeichen). Wenn ein [Marktplatz-Eintrag](/docs/de/plugin-marketplaces#plugin-entries) das Plugin unter einem anderen Namen auflistet, ist der Name des Marktplatz-Eintrags das, was `enabledPlugins` Schlüssel und `/plugin` verwenden | `"deployment-tools"` |

Dieser Name wird für die Namensgebung von Komponenten verwendet. Beispielsweise wird der Agent `agent-creator` für das Plugin mit dem Namen `plugin-dev` in der Benutzeroberfläche als `plugin-dev:agent-creator` angezeigt.

<h3 id="unrecognized-fields">
  Nicht erkannte Felder
</h3>

Claude Code ignoriert Top-Level-Felder, die es nicht erkennt. Sie können Metadaten aus einem anderen Ökosystem in `plugin.json` behalten und das Plugin wird trotzdem geladen. Dies macht es praktisch, ein Manifest zu verwalten, das gleichzeitig als VS Code- oder Cursor-Erweiterungsmanifest, eine npm `package.json` oder ein MCPB/DXT-Bundle-Manifest dient.

`claude plugin validate` meldet nicht erkannte Felder als Warnungen, nicht als Fehler. Wenn ein Feld ein oder zwei Zeichen von einem erkannten entfernt ist, schlägt die Warnung den wahrscheinlich beabsichtigten Namen vor. Ein Plugin mit nur Warnungen zu nicht erkannten Feldern besteht die Validierung und wird zur Laufzeit geladen.

Felder mit dem falschen Typ schlagen immer noch fehl. Beispielsweise ist ein `keywords` Wert, der ein String statt eines Arrays ist, ein Ladefehler, und `claude plugin validate` meldet ihn als solchen.

Übergeben Sie `--strict`, um Warnungen als Fehler zu behandeln. Verwenden Sie es in CI, um einen falsch geschriebenen Feldnamen oder ein Feld, das von einem anderen Tool-Manifest übrig geblieben ist, vor der Veröffentlichung zu erfassen, obwohl das Plugin zur Laufzeit geladen würde.

```bash theme={null}
claude plugin validate ./my-plugin --strict
```

<h3 id="metadata-fields">
  Metadaten-Felder
</h3>

| Feld             | Typ     | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                      | Beispiel                                                          |
| :--------------- | :------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------- |
| `$schema`        | string  | JSON-Schema-URL für Editor-Autovervollständigung und Validierung. Claude Code ignoriert dieses Feld beim Laden.                                                                                                                                                                                                                                                                                                   | `"https://json.schemastore.org/claude-code-plugin-manifest.json"` |
| `displayName`    | string  | Benutzerfreundlicher Name, der in der `/plugin` Auswahl und anderen UI-Oberflächen angezeigt wird. Fällt auf `name` zurück, wenn weggelassen. Im Gegensatz zu `name` kann es Leerzeichen und beliebige Groß-/Kleinschreibung enthalten. Wird nicht für Namensgebung oder Suche verwendet. Erfordert Claude Code v2.1.143 oder später.                                                                             | `"Deployment Tools"`                                              |
| `version`        | string  | Optional. Semantische Version. Das Setzen dieser Version fixiert das Plugin auf diese Versionsnummer, sodass Benutzer nur Updates erhalten, wenn Sie diese erhöhen. Wenn weggelassen, greift Claude Code auf den Git-Commit-SHA zurück, sodass jeder Commit als neue Version behandelt wird. Wenn auch im Marktplatz-Eintrag gesetzt, hat `plugin.json` Vorrang. Siehe [Versionsverwaltung](#version-management). | `"2.1.0"`                                                         |
| `description`    | string  | Kurze Erklärung des Plugin-Zwecks                                                                                                                                                                                                                                                                                                                                                                                 | `"Deployment automation tools"`                                   |
| `author`         | object  | Autoreninformationen                                                                                                                                                                                                                                                                                                                                                                                              | `{"name": "Dev Team", "email": "dev@company.com"}`                |
| `homepage`       | string  | Dokumentations-URL                                                                                                                                                                                                                                                                                                                                                                                                | `"https://docs.example.com"`                                      |
| `repository`     | string  | Quellcode-URL                                                                                                                                                                                                                                                                                                                                                                                                     | `"https://github.com/user/plugin"`                                |
| `license`        | string  | Lizenzbezeichner                                                                                                                                                                                                                                                                                                                                                                                                  | `"MIT"`, `"Apache-2.0"`                                           |
| `keywords`       | array   | Discovery-Tags                                                                                                                                                                                                                                                                                                                                                                                                    | `["deployment", "ci-cd"]`                                         |
| `defaultEnabled` | boolean | Ob das Plugin in einem aktivierten Zustand startet, wenn der Benutzer keinen gesetzt hat. Standardmäßig `true`. Siehe [Standardaktivierung](#default-enablement). Erfordert Claude Code v2.1.154 oder später.                                                                                                                                                                                                     | `false`                                                           |

<h3 id="default-enablement">
  Standardaktivierung
</h3>

Setzen Sie `defaultEnabled: false` in `plugin.json`, um ein Plugin zu versenden, das deaktiviert installiert wird. Der Benutzer schaltet es mit `claude plugin enable <plugin>` oder der `/plugin` Schnittstelle ein. Verwenden Sie dies für Plugins, die Kosten hinzufügen oder einen Bereich, in den sich ein Benutzer einwählen sollte, wie eines, das sich mit einem externen Service verbindet. Dies erfordert Claude Code v2.1.154 oder später. Frühere Versionen ignorieren das Feld und aktivieren das Plugin bei der Installation.

`defaultEnabled` ist der Fallback, wenn nichts anderes den Zustand des Plugins entschieden hat. Zwei Dinge haben Vorrang vor ihm:

* **Die Einstellung des Benutzers**: Ein Eintrag für das Plugin in `enabledPlugins` in jedem Einstellungsbereich. Einmal geschrieben, bleibt es über Plugin-Updates und Neuinstallationen bestehen, daher ändert das Ändern von `defaultEnabled` in einer späteren Version nicht einen bestehenden Benutzer.
* **Eine Abhängigkeitsanforderung**: Wenn ein Plugin von einem anderen erforderlich ist, das aktiv ist, schreibt Claude Code `true` dafür bei Installation oder Aktivierung. Das gibt ihm eine explizite Einstellung, daher gilt sein eigener Standard nicht mehr. Siehe [Aktivieren oder Deaktivieren eines Plugins mit Abhängigkeiten](/docs/de/plugin-dependencies#enable-or-disable-a-plugin-with-dependencies).

Das gleiche Feld kann im Marktplatz-Eintrag eines Plugins erscheinen, wo es Vorrang vor dem Wert in `plugin.json` hat. Siehe [Optionale Plugin-Felder](/docs/de/plugin-marketplaces#optional-plugin-fields).

<h3 id="component-path-fields">
  Komponentenpfad-Felder
</h3>

| Feld                    | Typ                   | Beschreibung                                                                                                                                                                                            | Beispiel                                             |
| :---------------------- | :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------- |
| `skills`                | string\|array         | Benutzerdefinierte Skill-Verzeichnisse mit `<name>/SKILL.md` Struktur. Ergänzt das Standard `skills/` Verzeichnis. Siehe [Pfad-Verhaltensregeln](#path-behavior-rules) für die Marktplatz-Root-Ausnahme | `"./custom/skills/"`                                 |
| `commands`              | string\|array         | Benutzerdefinierte flache `.md` Skill-Dateien oder Verzeichnisse (ersetzt Standard `commands/`)                                                                                                         | `"./custom/cmd.md"` oder `["./cmd1.md"]`             |
| `agents`                | string\|array         | Benutzerdefinierte Agent-Dateien (ersetzt Standard `agents/`)                                                                                                                                           | `"./custom/agents/reviewer.md"`                      |
| `hooks`                 | string\|array\|object | Hook-Konfigurationspfade oder Inline-Konfiguration                                                                                                                                                      | `"./my-extra-hooks.json"`                            |
| `mcpServers`            | string\|array\|object | MCP-Konfigurationspfade oder Inline-Konfiguration                                                                                                                                                       | `"./my-extra-mcp-config.json"`                       |
| `outputStyles`          | string\|array         | Benutzerdefinierte Output-Style-Dateien/Verzeichnisse (ersetzt Standard `output-styles/`)                                                                                                               | `"./styles/"`                                        |
| `lspServers`            | string\|array\|object | [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) Konfigurationen für Code-Intelligenz (Gehe zu Definition, finde Referenzen, etc.)                                     | `"./.lsp.json"`                                      |
| `experimental.themes`   | string\|array         | Farbthema-Dateien/Verzeichnisse (ersetzt Standard `themes/`). Siehe [Designs](#themes)                                                                                                                  | `"./themes/"`                                        |
| `experimental.monitors` | string\|array         | Hintergrund-[Monitor](/docs/de/tools-reference#monitor-tool) Konfigurationen, die automatisch starten, wenn das Plugin aktiv ist. Siehe [Monitore](#monitors)                                                | `"./monitors.json"`                                  |
| `userConfig`            | object                | Benutzerkonfigurierbare Werte, die bei der Aktivierung abgefragt werden. Siehe [Benutzerkonfiguration](#user-configuration)                                                                             | Siehe unten                                          |
| `channels`              | array                 | Kanal-Deklarationen für Nachrichteninjection (Telegram, Slack, Discord Stil). Siehe [Kanäle](#channels)                                                                                                 | Siehe unten                                          |
| `dependencies`          | array                 | Andere Plugins, die dieses Plugin benötigt, optional mit semver Versionsbeschränkungen. Siehe [Plugin-Abhängigkeitsversionen einschränken](/docs/de/plugin-dependencies)                                     | `[{ "name": "secrets-vault", "version": "~2.1.0" }]` |

<h3 id="experimental-components">
  Experimentelle Komponenten
</h3>

Komponenten unter dem `experimental` Schlüssel, `themes` und `monitors`, haben ein Manifest-Schema, das sich zwischen Releases ändern kann, während sie stabilisieren. Wo Sie sie deklarieren, ist eine separate Migration: das Top-Level funktioniert immer noch, `claude plugin validate` warnt, und eine zukünftige Version wird `experimental.*` erfordern.

<h3 id="user-configuration">
  Benutzerkonfiguration
</h3>

Das `userConfig` Feld deklariert Werte, die Claude Code den Benutzer abfragt, wenn das Plugin aktiviert wird. Verwenden Sie dies, anstatt Benutzer zu zwingen, `settings.json` manuell zu bearbeiten.

```json theme={null}
{
  "userConfig": {
    "api_endpoint": {
      "type": "string",
      "title": "API endpoint",
      "description": "Your team's API endpoint"
    },
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "API authentication token",
      "sensitive": true
    }
  }
}
```

Schlüssel müssen gültige Bezeichner sein. Jede Option unterstützt diese Felder:

| Feld          | Erforderlich | Beschreibung                                                                                               |
| :------------ | :----------- | :--------------------------------------------------------------------------------------------------------- |
| `type`        | Ja           | Einer von `string`, `number`, `boolean`, `directory` oder `file`                                           |
| `title`       | Ja           | Beschriftung im Konfigurationsdialog                                                                       |
| `description` | Ja           | Hilfetext unter dem Feld                                                                                   |
| `sensitive`   | Nein         | Wenn `true`, maskiert die Eingabe und speichert den Wert im sicheren Speicher anstelle von `settings.json` |
| `required`    | Nein         | Wenn `true`, schlägt die Validierung fehl, wenn das Feld leer ist                                          |
| `default`     | Nein         | Wert, der verwendet wird, wenn der Benutzer nichts bereitstellt                                            |
| `multiple`    | Nein         | Für `string` Typ, erlaubt ein Array von Strings                                                            |
| `min` / `max` | Nein         | Grenzen für `number` Typ                                                                                   |

Jeder Wert ist für die Substitution als `${user_config.KEY}` in MCP- und LSP-Server-Konfigurationen und Hook-Befehlen verfügbar. Nicht-sensitive Werte können auch in Skill- und Agent-Inhalten ersetzt werden. Alle Werte werden an Hook-Prozesse als `CLAUDE_PLUGIN_OPTION_<KEY>` Umgebungsvariablen exportiert, wobei `<KEY>` der Optionsschlüssel in Großbuchstaben ist.

Felder, die in einer Shell ausgeführt werden, lehnen `${user_config.*}` ab: Das Ersetzen eines konfigurierten Werts in einem Shell-Befehl würde der Shell ermöglichen, alles auszuführen, was dieser Wert enthält, daher schlägt die Komponente mit einem [Fehler](/docs/de/errors#plugin-command-references-user-config) fehl. Jedes abgelehnte Feld hat eine alternative Möglichkeit, den Wert zu übergeben:

| Abgelehntes Feld                                                             | Wie man den Wert übergibt                                                                                                                        |
| :--------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------- |
| Shell-Form-Hook-Befehle                                                      | Verwenden Sie [Exec-Form](/docs/de/hooks#exec-form-and-shell-form) mit `args`, oder lesen Sie `CLAUDE_PLUGIN_OPTION_<KEY>` aus der Umgebung des Hooks |
| [Monitor](#monitors) Befehle                                                 | Lesen Sie den Wert aus einer Konfigurationsdatei im Skript                                                                                       |
| MCP [`headersHelper`](/docs/de/mcp#use-dynamic-headers-for-custom-authentication) | Lesen Sie den Wert aus einer Konfigurationsdatei im Skript                                                                                       |

Vor v2.1.207 ersetzten diese Felder `${user_config.KEY}` Werte; aktualisieren Sie Plugins, die sich darauf verlassen haben.

Nicht-sensitive Werte werden unter dem [`pluginConfigs`](/docs/de/settings#pluginconfigs) Schlüssel in `settings.json` als `pluginConfigs[<plugin-id>].options` gespeichert. Claude Code schreibt den Schlüssel in Benutzereinstellungen und liest ihn aus Benutzereinstellungen, dem `--settings` Flag und verwalteten Einstellungen zurück; Einträge in einer Projekt-`.claude/settings.json` oder `.claude/settings.local.json` werden ignoriert. Vor v2.1.207 las Claude Code auch Projekt- und lokale Einstellungen.

Sensitive Werte gehen zum macOS Keychain oder zu `~/.claude/.credentials.json` auf Plattformen, wo kein unterstützter Keychain verfügbar ist. Keychain-Speicher wird mit OAuth-Tokens geteilt und hat ein ungefähres Gesamtlimit von 2 KB, daher halten Sie sensitive Werte klein.

<h3 id="channels">
  Kanäle
</h3>

Das `channels` Feld ermöglicht es einem Plugin, einen oder mehrere Nachrichtenkanäle zu deklarieren, die Inhalte in die Konversation injizieren. Jeder Kanal bindet sich an einen MCP-Server, den das Plugin bereitstellt.

```json theme={null}
{
  "channels": [
    {
      "server": "telegram",
      "userConfig": {
        "bot_token": {
          "type": "string",
          "title": "Bot token",
          "description": "Telegram bot token",
          "sensitive": true
        },
        "owner_id": {
          "type": "string",
          "title": "Owner ID",
          "description": "Your Telegram user ID"
        }
      }
    }
  ]
}
```

Das `server` Feld ist erforderlich und muss einem Schlüssel in den `mcpServers` des Plugins entsprechen. Das optionale Pro-Kanal `userConfig` verwendet das gleiche Schema wie das Top-Level-Feld, wodurch das Plugin Bot-Tokens oder Owner-IDs abfragen kann, wenn das Plugin aktiviert wird.

<h3 id="path-behavior-rules">
  Pfad-Verhaltensregeln
</h3>

Ob ein benutzerdefinierter Pfad das Standard-Verzeichnis des Plugins ersetzt oder erweitert, hängt vom Feld ab:

* **Ersetzt den Standard**: `commands`, `agents`, `outputStyles`, `experimental.themes`, `experimental.monitors`. Beispielsweise wird das Standard-Verzeichnis `commands/` nicht gescannt, wenn das Manifest `commands` angibt. Um den Standard zu behalten und mehr hinzuzufügen, listen Sie ihn explizit auf: `"commands": ["./commands/", "./extras/"]`
* **Fügt zum Standard hinzu**: `skills`. Das Standard-Verzeichnis `skills/` wird immer gescannt, und Verzeichnisse, die in `skills` aufgelistet sind, werden zusammen mit ihm geladen. Ausnahme: für einen [Marktplatz-Eintrag, dessen `source` zum Marktplatz-Root aufgelöst wird](/docs/de/plugin-marketplaces#advanced-plugin-entries), ersetzt das Deklarieren spezifischer Unterverzeichnisse den Scan
* **Eigene Merge-Regeln**: [hooks](#hooks), [MCP-Server](#mcp-servers) und [LSP-Server](#lsp-servers). Siehe jeden Abschnitt für die Kombinationsweise mehrerer Quellen

Wenn ein Plugin sowohl einen Standard-Ordner als auch den entsprechenden Manifest-Schlüssel hat, kennzeichnet Claude Code v2.1.140 und später den ignorierten Ordner in `claude plugin list` und der `/plugin` Detailansicht. Das Plugin wird weiterhin mit den Manifest-Pfaden geladen. Es wird keine Warnung angezeigt, wenn der Manifest-Schlüssel auf den Standard-Ordner verweist, beispielsweise `"commands": ["./commands/deploy.md"]`, da der Ordner in diesem Fall explizit adressiert wird.

Für alle Pfadfelder:

* Alle Pfade müssen relativ zum Plugin-Root sein und mit `./` beginnen
* Komponenten aus benutzerdefinierten Pfaden verwenden die gleichen Benennungs- und Namensgebungsregeln
* Mehrere Pfade können als Arrays angegeben werden
* Wenn ein Skill-Pfad auf ein Verzeichnis verweist, das direkt ein `SKILL.md` enthält, beispielsweise `"skills": ["./"]` verweist auf den Plugin-Root, bestimmt das Frontmatter-Feld `name` in `SKILL.md` den Aufrufen-Namen des Skills. Dies gibt einen stabilen Namen unabhängig vom Installationsverzeichnis. Wenn `name` nicht im Frontmatter gesetzt ist, wird der Verzeichnis-Basename als Fallback verwendet.

Ein Plugin, das ein `SKILL.md` in seinem Root hat, kein `skills/` Unterverzeichnis und kein `skills` Manifest-Feld hat, wird automatisch als Single-Skill-Plugin in Claude Code v2.1.142 und später geladen. Sie müssen `"skills": ["./"]` in `plugin.json` für dieses Layout nicht setzen. Der Aufrufen-Name des Skills folgt der gleichen Regel wie oben: das Frontmatter-Feld `name` oder der Verzeichnis-Basename als Fallback.

**Pfad-Beispiele**:

```json theme={null}
{
  "commands": [
    "./specialized/deploy.md",
    "./utilities/batch-process.md"
  ],
  "agents": [
    "./custom-agents/reviewer.md",
    "./custom-agents/tester.md"
  ]
}
```

<h3 id="environment-variables">
  Umgebungsvariablen
</h3>

Claude Code bietet drei Variablen zum Referenzieren von Pfaden:

| Variable                | Wird aufgelöst zu                                                                                                      | Verwenden Sie es für                                                                               |
| :---------------------- | :--------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------- |
| `${CLAUDE_PLUGIN_ROOT}` | Absoluter Pfad zum Installationsverzeichnis des Plugins                                                                | Skripte, Binärdateien und Konfigurationsdateien, die mit dem Plugin gebündelt sind                 |
| `${CLAUDE_PLUGIN_DATA}` | [Persistentes Verzeichnis](#persistent-data-directory), das Plugin-Updates überlebt, wird beim ersten Verweis erstellt | Installierte Abhängigkeiten wie `node_modules` oder Python-Umgebungen, generierter Code und Caches |
| `${CLAUDE_PROJECT_DIR}` | Das Projekt-Root                                                                                                       | Projektlokale Skripte und Konfigurationsdateien                                                    |

Alle drei werden als Umgebungsvariablen an Hook-Prozesse und an MCP- und LSP-Server-Subprozesse exportiert. Welche Felder sie inline ersetzen, hängt von der Plugin-Komponente ab:

| Plugin-Komponente              | Felder, in denen Platzhalter aufgelöst werden |
| :----------------------------- | :-------------------------------------------- |
| Skill- und Agent-Inhalte       | Überall dort, wo der Platzhalter erscheint    |
| Hook- und Monitor-Befehle      | Überall dort, wo der Platzhalter erscheint    |
| MCP `stdio` Server             | `command`, `args`, `env`                      |
| MCP `http`, `sse`, `ws` Server | `url`, `headers`, `headersHelper`             |
| LSP-Server                     | `command`, `args`, `env`, `workspaceFolder`   |

In Hook-Befehlen verwenden Sie [Exec-Form](/docs/de/hooks#exec-form-and-shell-form) mit `args`, damit jeder Pfad als ein Argument ohne Anführungszeichen übergeben wird. In Shell-Form-Hooks und Monitor-Befehlen wickeln Sie die Variablen in doppelte Anführungszeichen ein, wie in `"${CLAUDE_PROJECT_DIR}/scripts/server.sh"`. Dieser Shell-Form-Hook führt ein mit einem Plugin gebündeltes Skript aus:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/process.sh"
          }
        ]
      }
    ]
  }
}
```

`${CLAUDE_PLUGIN_ROOT}` ändert sich, wenn das Plugin aktualisiert wird. Das Verzeichnis der vorherigen Version bleibt etwa sieben Tage nach einem Update auf der Festplatte, bevor es bereinigt wird, aber behandeln Sie es als kurzlebig und schreiben Sie keinen Status dort.

Wenn ein Plugin während einer Sitzung aktualisiert wird, verwenden Hook-Befehle, Monitore, MCP-Server und LSP-Server weiterhin den Pfad der vorherigen Version. Führen Sie `/reload-plugins` aus, um Hooks, MCP-Server und LSP-Server auf den neuen Pfad umzuschalten; Monitore erfordern einen Neustart der Sitzung.

MCP-Server können auch die `roots/list` Anfrage aufrufen, um die Arbeitsverzeichnisse der Sitzung zur Laufzeit zu lesen. Siehe [was `roots/list` zurückgibt und wann Claude Code den Server über Änderungen benachrichtigt](/docs/de/mcp#option-3-add-a-local-stdio-server).

<h4 id="persistent-data-directory">
  Persistentes Datenverzeichnis
</h4>

Das `${CLAUDE_PLUGIN_DATA}` Verzeichnis wird zu `~/.claude/plugins/data/{id}/` aufgelöst, wobei `{id}` der Plugin-Bezeichner mit Zeichen außerhalb von `a-z`, `A-Z`, `0-9`, `_` und `-` ist, die durch `-` ersetzt werden. Für ein Plugin, das als `formatter@my-marketplace` installiert ist, ist das Verzeichnis `~/.claude/plugins/data/formatter-my-marketplace/`.

Eine häufige Verwendung ist die einmalige Installation von Sprachabhängigkeiten und deren Wiederverwendung über Sessions und Plugin-Updates hinweg. Da das Datenverzeichnis länger lebt als jede einzelne Plugin-Version, kann eine Überprüfung auf Verzeichnisexistenz allein nicht erkennen, wenn ein Update das Abhängigkeitsmanifest des Plugins ändert. Das empfohlene Muster vergleicht das gebündelte Manifest mit einer Kopie im Datenverzeichnis und installiert neu, wenn sie sich unterscheiden.

Dieser `SessionStart` Hook installiert `node_modules` beim ersten Durchlauf und erneut, wenn ein Plugin-Update ein geändertes `package.json` enthält:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "diff -q \"${CLAUDE_PLUGIN_ROOT}/package.json\" \"${CLAUDE_PLUGIN_DATA}/package.json\" >/dev/null 2>&1 || (cd \"${CLAUDE_PLUGIN_DATA}\" && cp \"${CLAUDE_PLUGIN_ROOT}/package.json\" . && npm install) || rm -f \"${CLAUDE_PLUGIN_DATA}/package.json\""
          }
        ]
      }
    ]
  }
}
```

Der `diff` beendet sich mit Nonzero, wenn die gespeicherte Kopie fehlt oder sich vom gebündelten unterscheidet, was sowohl den ersten Durchlauf als auch abhängigkeitsändernde Updates abdeckt. Wenn `npm install` fehlschlägt, entfernt das nachfolgende `rm` das kopierte Manifest, damit die nächste Session erneut versucht.

Skripte, die in `${CLAUDE_PLUGIN_ROOT}` gebündelt sind, können dann gegen die persistierten `node_modules` ausgeführt werden:

```json theme={null}
{
  "mcpServers": {
    "routines": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"],
      "env": {
        "NODE_PATH": "${CLAUDE_PLUGIN_DATA}/node_modules"
      }
    }
  }
}
```

Das Datenverzeichnis wird automatisch gelöscht, wenn Sie das Plugin aus dem letzten Bereich deinstallieren, in dem es installiert ist. Die `/plugin` Schnittstelle zeigt die Verzeichnisgröße an und fragt vor dem Löschen. Die CLI löscht standardmäßig; übergeben Sie [`--keep-data`](#plugin-uninstall), um es zu bewahren.

***

<h2 id="plugin-caching-and-file-resolution">
  Plugin-Caching und Dateiauflösung
</h2>

Plugins werden auf eine von zwei Arten angegeben:

* Durch `claude --plugin-dir` oder `claude --plugin-url`, für die Dauer einer Session.
* Durch einen Marktplatz, installiert für zukünftige Sessions.

Aus Sicherheits- und Verifizierungsgründen kopiert Claude Code *Marktplatz*-Plugins in den lokalen **Plugin-Cache** des Benutzers (`~/.claude/plugins/cache`), anstatt sie an Ort und Stelle zu verwenden. Das Verständnis dieses Verhaltens ist wichtig, wenn Sie Plugins entwickeln, die auf externe Dateien verweisen.

Jede installierte Version ist ein separates Verzeichnis im Cache. Wenn Sie ein Plugin aktualisieren oder deinstallieren, wird das vorherige Versionsverzeichnis als verwaist markiert und automatisch 7 Tage später entfernt. Die Gnadenfrist ermöglicht es gleichzeitigen Claude Code Sessions, die bereits die alte Version geladen haben, ohne Fehler weiter zu laufen.

Claudes Glob- und Grep-Tools überspringen verwaiste Versionsverzeichnisse während Suchen, daher enthalten Dateiergebnisse keinen veralteten Plugin-Code.

<h3 id="path-traversal-limitations">
  Pfad-Traversal-Einschränkungen
</h3>

Installierte Plugins können nicht auf Dateien außerhalb ihres Verzeichnisses verweisen. Pfade, die außerhalb des Plugin-Root traversieren (wie `../shared-utils`), funktionieren nach der Installation nicht, da diese externen Dateien nicht in den Cache kopiert werden.

<h3 id="share-files-within-a-marketplace-with-symlinks">
  Dateien innerhalb eines Marktplatzes mit Symlinks freigeben
</h3>

Wenn Ihr Plugin Dateien mit anderen Teilen desselben Marktplatzes freigeben muss, können Sie symbolische Links in Ihrem Plugin-Verzeichnis erstellen. Wie ein Symlink behandelt wird, wenn das Plugin in den Cache kopiert wird, hängt davon ab, wo sein Ziel aufgelöst wird:

* **Innerhalb des eigenen Verzeichnisses des Plugins:** Der Symlink wird als relativer Symlink im Cache beibehalten, sodass er zur Laufzeit weiterhin zum kopierten Ziel aufgelöst wird.
* **Anderswo innerhalb desselben Marktplatzes:** Der Symlink wird dereferenziert. Der Inhalt des Ziels wird stattdessen in den Cache kopiert. Dies ermöglicht es einem Meta-Plugin, sein `skills/`-Verzeichnis mit Skills zu verknüpfen, die von anderen Plugins im Marktplatz definiert werden.
* **Außerhalb des Marktplatzes:** Der Symlink wird aus Sicherheitsgründen übersprungen. Dies verhindert, dass Plugins beliebige Host-Dateien wie Systempfade in den Cache ziehen.

Für Plugins, die mit `--plugin-dir` installiert oder aus einem lokalen Pfad installiert werden, werden nur Symlinks beibehalten, die sich innerhalb des eigenen Verzeichnisses des Plugins auflösen. Alle anderen werden übersprungen.

Der folgende Befehl erstellt einen Link von innerhalb eines Marktplatz-Plugins zu einem gemeinsamen Skill, der von einem Geschwister-Plugin definiert wird. Verwenden Sie unter Windows `mklink /D` von einer erhöhten Eingabeaufforderung oder aktivieren Sie den Entwicklermodus:

```bash theme={null}
ln -s ../../shared-plugin/skills/foo ./skills/foo
```

Dies bietet Flexibilität bei Beibehaltung der Sicherheitsvorteile des Caching-Systems.

***

<h2 id="plugin-directory-structure">
  Plugin-Verzeichnisstruktur
</h2>

<h3 id="standard-plugin-layout">
  Standard-Plugin-Layout
</h3>

Ein vollständiges Plugin folgt dieser Struktur:

```text theme={null}
enterprise-plugin/
├── .claude-plugin/           # Metadaten-Verzeichnis (optional)
│   └── plugin.json             # Plugin-Manifest
├── skills/                   # Skills
│   ├── code-reviewer/
│   │   └── SKILL.md
│   └── pdf-processor/
│       ├── SKILL.md
│       └── scripts/
├── commands/                 # Skills als flache .md Dateien
│   ├── status.md
│   └── logs.md
├── agents/                   # Subagent-Definitionen
│   ├── security-reviewer.md
│   ├── performance-tester.md
│   └── compliance-checker.md
├── output-styles/            # Output-Style-Definitionen
│   └── terse.md
├── themes/                   # Farbschema-Definitionen
│   └── dracula.json
├── monitors/                 # Hintergrund-Monitor-Konfigurationen
│   └── monitors.json
├── hooks/                    # Hook-Konfigurationen
│   ├── hooks.json           # Haupt-Hook-Konfiguration
│   └── security-hooks.json  # Zusätzliche Hooks
├── bin/                      # Plugin-Ausführbare Dateien, die zu PATH hinzugefügt werden
│   └── my-tool               # Aufrufbar als bloßer Befehl im Bash-Tool
├── settings.json            # Standardeinstellungen für das Plugin
├── .mcp.json                # MCP-Server-Definitionen
├── .lsp.json                # LSP-Server-Konfigurationen
├── scripts/                 # Hook- und Utility-Skripte
│   ├── security-scan.sh
│   ├── format-code.py
│   └── deploy.js
├── LICENSE                  # Lizenzdatei
└── CHANGELOG.md             # Versionsverlauf
```

<Warning>
  Das `.claude-plugin/` Verzeichnis enthält die `plugin.json` Datei. Alle anderen Verzeichnisse (commands/, agents/, skills/, output-styles/, themes/, monitors/, hooks/) müssen sich im Plugin-Root befinden, nicht innerhalb von `.claude-plugin/`.
</Warning>

Eine `CLAUDE.md` Datei im Plugin-Root wird nicht als Projektkontext geladen. Plugins tragen Kontext durch Skills, Agents und Hooks bei, anstatt durch CLAUDE.md. Um Anweisungen bereitzustellen, die in Claudes Kontext geladen werden, fügen Sie diese in einen [Skill](#skills) ein.

<h3 id="file-locations-reference">
  Datei-Speicherorte-Referenz
</h3>

| Komponente              | Standard-Speicherort         | Zweck                                                                                                                                                                                                               |
| :---------------------- | :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Manifest**            | `.claude-plugin/plugin.json` | Plugin-Metadaten und Konfiguration (optional)                                                                                                                                                                       |
| **Skills**              | `skills/`                    | Skills mit `<name>/SKILL.md` Struktur                                                                                                                                                                               |
| **Befehle**             | `commands/`                  | Skills als flache Markdown-Dateien. Verwenden Sie `skills/` für neue Plugins                                                                                                                                        |
| **Agents**              | `agents/`                    | Subagent Markdown-Dateien                                                                                                                                                                                           |
| **Output-Styles**       | `output-styles/`             | Output-Style-Definitionen                                                                                                                                                                                           |
| **Themes**              | `themes/`                    | Farbschema-Definitionen                                                                                                                                                                                             |
| **Hooks**               | `hooks/hooks.json`           | Hook-Konfiguration                                                                                                                                                                                                  |
| **MCP-Server**          | `.mcp.json`                  | MCP-Server-Definitionen                                                                                                                                                                                             |
| **LSP-Server**          | `.lsp.json`                  | Language Server Konfigurationen                                                                                                                                                                                     |
| **Monitore**            | `monitors/monitors.json`     | Hintergrund-Monitor-Konfigurationen                                                                                                                                                                                 |
| **Ausführbare Dateien** | `bin/`                       | Ausführbare Dateien, die zum Bash-Tool `PATH` hinzugefügt werden. Dateien hier sind als bloße Befehle in jedem Bash-Tool-Aufruf aufrufbar, während das Plugin aktiviert ist                                         |
| **Einstellungen**       | `settings.json`              | Standardkonfiguration, die angewendet wird, wenn das Plugin aktiviert wird. Derzeit werden nur die [`agent`](/docs/de/sub-agents) und [`subagentStatusLine`](/docs/de/statusline#subagent-status-lines) Schlüssel unterstützt |

***

<h2 id="cli-commands-reference">
  CLI-Befehle-Referenz
</h2>

Claude Code bietet CLI-Befehle für nicht-interaktive Plugin-Verwaltung, nützlich für Scripting und Automatisierung.

<h3 id="plugin-init">
  plugin init
</h3>

Erstellen Sie ein neues Plugin unter `~/.claude/skills/<name>/`. In der nächsten Claude Code Session wird es automatisch als `<name>@skills-dir` geladen und erscheint in `/plugin` und `claude plugin list` ohne Installationsschritt.

Siehe [Skills-Verzeichnis-Plugins](#skills-directory-plugins) für Bereichs- und Vertrauensanforderungen.

```bash theme={null}
claude plugin init <name> [options]
```

**Argumente:**

* `<name>`: Plugin-Name. Wird zum Skill-Namespace und zum Verzeichnisnamen unter `~/.claude/skills/`, daher kann er keine Leerzeichen oder Pfad-Trennzeichen enthalten.

**Optionen:**

| Option                   | Beschreibung                                                                                                          | Standard                |
| :----------------------- | :-------------------------------------------------------------------------------------------------------------------- | :---------------------- |
| `--description <text>`   | Manifest-Beschreibung                                                                                                 |                         |
| `--author <name>`        | Autorenname                                                                                                           | `git config user.name`  |
| `--author-email <email>` | Autoren-E-Mail                                                                                                        | `git config user.email` |
| `--with <components...>` | Auch Komponentenordner erstellen. Gültige Werte: `skills`, `agents`, `hooks`, `mcp`, `lsp`, `output-style`, `channel` |                         |
| `-f, --force`            | Überschreiben Sie ein vorhandenes `.claude-plugin/` am Ziel                                                           |                         |
| `-h, --help`             | Hilfe für Befehl anzeigen                                                                                             |                         |

**Aliase:** `new`

Jeder `--with` Wert fügt eine Starter-Datei für diese Komponente hinzu, bereit zum Bearbeiten:

| Komponente     | Was es erstellt                                                                                                  |
| :------------- | :--------------------------------------------------------------------------------------------------------------- |
| `skills`       | Ein zusätzlicher namensgebundener `<name>:example` Skill neben dem Standard                                      |
| `agents`       | Eine `agents/` Subagent-Definition                                                                               |
| `hooks`        | Eine `hooks/hooks.json` mit einem Beispiel-Event-Handler                                                         |
| `mcp`          | Eine `.mcp.json` mit HTTP- und stdio-Server-Beispielen                                                           |
| `lsp`          | Eine `.lsp.json` Language-Server-Beispiel                                                                        |
| `output-style` | Ein `output-styles/<name>.md`, das automatisch angewendet wird, während das Plugin aktiviert ist                 |
| `channel`      | Ein MCP-basierter [Kanal](/docs/de/channels): ein stdio-Server (`server.ts`), sein `.mcp.json` und ein `package.json` |

Das erstellte Plugin verwendet die `@skills-dir` Quelle anstelle eines Marktplatzes. Administratoren können diese Quelle mit `strictKnownMarketplaces` blockieren oder indem sie `{"source": "skills-dir"}` zu `blockedMarketplaces` in [verwalteten Einstellungen](/docs/de/plugin-marketplaces#managed-marketplace-restrictions) hinzufügen. Wenn blockiert, schlägt `plugin init` fehl, bevor etwas geschrieben wird.

**Beispiele:**

```bash theme={null}
# Erstellen Sie ein minimales Plugin
claude plugin init my-helper

# Erstellen Sie mit Skill- und Hook-Ordnern
claude plugin init my-helper --with skills hooks

# Überschreiben Sie ein vorhandenes Gerüst
claude plugin init my-helper --force
```

<h3 id="plugin-install">
  plugin install
</h3>

Installieren Sie ein Plugin aus verfügbaren Marktplätzen.

```bash theme={null}
claude plugin install <plugin> [options]
```

**Argumente:**

* `<plugin>`: Plugin-Name oder `plugin-name@marketplace-name` für einen bestimmten Marktplatz

**Optionen:**

| Option                | Beschreibung                                         | Standard |
| :-------------------- | :--------------------------------------------------- | :------- |
| `-s, --scope <scope>` | Installationsbereich: `user`, `project` oder `local` | `user`   |
| `-h, --help`          | Hilfe für Befehl anzeigen                            |          |

Der Bereich bestimmt, welche Einstellungsdatei das installierte Plugin hinzugefügt wird. Beispielsweise schreibt `--scope project` zu `enabledPlugins` in .claude/settings.json, wodurch das Plugin für alle verfügbar wird, die das Projekt-Repository klonen.

**Beispiele:**

```bash theme={null}
# Installieren Sie im Benutzerbereich (Standard)
claude plugin install formatter@my-marketplace

# Installieren Sie im Projektbereich (geteilt mit Team)
claude plugin install formatter@my-marketplace --scope project

# Installieren Sie im lokalen Bereich (gitignoriert)
claude plugin install formatter@my-marketplace --scope local
```

<h3 id="plugin-uninstall">
  plugin uninstall
</h3>

Entfernen Sie ein installiertes Plugin.

```bash theme={null}
claude plugin uninstall <plugin> [options]
```

**Argumente:**

* `<plugin>`: Plugin-Name oder `plugin-name@marketplace-name`

**Optionen:**

| Option                | Beschreibung                                                                                                                      | Standard |
| :-------------------- | :-------------------------------------------------------------------------------------------------------------------------------- | :------- |
| `-s, --scope <scope>` | Deinstallieren aus Bereich: `user`, `project` oder `local`                                                                        | `user`   |
| `--keep-data`         | Bewahren Sie das [persistente Datenverzeichnis](#persistent-data-directory) des Plugins                                           |          |
| `--prune`             | Entfernen Sie auch automatisch installierte Abhängigkeiten, die kein anderes Plugin benötigt. Siehe [plugin prune](#plugin-prune) |          |
| `-y, --yes`           | Überspringen Sie die `--prune` Bestätigungsaufforderung. Erforderlich, wenn stdin oder stdout kein TTY ist                        |          |
| `-h, --help`          | Hilfe für Befehl anzeigen                                                                                                         |          |

**Aliase:** `remove`, `rm`

Standardmäßig löscht das Deinstallieren aus dem letzten verbleibenden Bereich auch das `${CLAUDE_PLUGIN_DATA}` Verzeichnis des Plugins. Verwenden Sie `--keep-data`, um es zu bewahren, beispielsweise beim Neuinstallieren nach dem Testen einer neuen Version.

<h3 id="plugin-prune">
  plugin prune
</h3>

Entfernen Sie automatisch installierte Plugin-Abhängigkeiten, die nicht mehr von einem installierten Plugin benötigt werden. Abhängigkeiten, die Claude Code eingezogen hat, um das [`dependencies`](/docs/de/plugin-dependencies) Feld eines anderen Plugins zu erfüllen, werden entfernt; Plugins, die Sie direkt installiert haben, werden niemals berührt.

```bash theme={null}
claude plugin prune [options]
```

**Optionen:**

| Option                | Beschreibung                                                                                     | Standard |
| :-------------------- | :----------------------------------------------------------------------------------------------- | :------- |
| `-s, --scope <scope>` | Bereinigen im Bereich: `user`, `project` oder `local`                                            | `user`   |
| `--dry-run`           | Listet auf, was entfernt würde, ohne etwas zu entfernen                                          |          |
| `-y, --yes`           | Überspringen Sie die Bestätigungsaufforderung. Erforderlich, wenn stdin oder stdout kein TTY ist |          |
| `-h, --help`          | Hilfe für Befehl anzeigen                                                                        |          |

**Aliase:** `autoremove`

Der Befehl listet verwaiste Abhängigkeiten auf und fragt vor dem Entfernen um Bestätigung. Um ein Plugin zu entfernen und seine Abhängigkeiten in einem Schritt zu bereinigen, führen Sie `claude plugin uninstall <plugin> --prune` aus.

<Note>
  `claude plugin prune` erfordert Claude Code v2.1.121 oder später.
</Note>

<h3 id="plugin-enable">
  plugin enable
</h3>

Aktivieren Sie ein deaktiviertes Plugin. Wenn das Plugin [Abhängigkeiten](/docs/de/plugin-dependencies) deklariert, aktiviert Claude Code diese transitiv im gleichen Bereich, und der Befehl schlägt fehl, wenn eine Abhängigkeit nicht installiert ist.

```bash theme={null}
claude plugin enable <plugin> [options]
```

**Argumente:**

* `<plugin>`: Plugin-Name oder `plugin-name@marketplace-name`

**Optionen:**

| Option                | Beschreibung                                           | Standard |
| :-------------------- | :----------------------------------------------------- | :------- |
| `-s, --scope <scope>` | Bereich zum Aktivieren: `user`, `project` oder `local` | `user`   |
| `-h, --help`          | Hilfe für Befehl anzeigen                              |          |

<h3 id="plugin-disable">
  plugin disable
</h3>

Deaktivieren Sie ein Plugin, ohne es zu deinstallieren. Schlägt fehl, wenn ein anderes aktiviertes Plugin [von](/docs/de/plugin-dependencies#enable-or-disable-a-plugin-with-dependencies) dem Ziel abhängt. Die Fehlermeldung enthält einen verketteten Befehl, der zuerst alle abhängigen Plugins deaktiviert.

```bash theme={null}
claude plugin disable <plugin> [options]
```

**Argumente:**

* `<plugin>`: Plugin-Name oder `plugin-name@marketplace-name`

**Optionen:**

| Option                | Beschreibung                                             | Standard |
| :-------------------- | :------------------------------------------------------- | :------- |
| `-s, --scope <scope>` | Bereich zum Deaktivieren: `user`, `project` oder `local` | `user`   |
| `-h, --help`          | Hilfe für Befehl anzeigen                                |          |

<h3 id="plugin-update">
  plugin update
</h3>

Aktualisieren Sie ein Plugin auf die neueste Version.

```bash theme={null}
claude plugin update <plugin> [options]
```

**Argumente:**

* `<plugin>`: Plugin-Name oder `plugin-name@marketplace-name`

**Optionen:**

| Option                | Beschreibung                                                         | Standard |
| :-------------------- | :------------------------------------------------------------------- | :------- |
| `-s, --scope <scope>` | Bereich zum Aktualisieren: `user`, `project`, `local` oder `managed` | `user`   |
| `-h, --help`          | Hilfe für Befehl anzeigen                                            |          |

***

<h3 id="plugin-list">
  plugin list
</h3>

Listet installierte Plugins mit ihrer Version, Quell-Marktplatz und Aktivierungsstatus auf.

```bash theme={null}
claude plugin list [options]
```

**Optionen:**

| Option        | Beschreibung                                                         | Standard |
| :------------ | :------------------------------------------------------------------- | :------- |
| `--json`      | Ausgabe als JSON                                                     |          |
| `--available` | Verfügbare Plugins von Marktplätzen einschließen. Erfordert `--json` |          |
| `-h, --help`  | Hilfe für Befehl anzeigen                                            |          |

Innerhalb einer interaktiven Sitzung druckt `/plugin list` die gleiche Auflistung inline. Die interaktive Form akzeptiert `--enabled` oder `--disabled`, um nur Plugins in diesem Zustand anzuzeigen, und `ls` als Kurzform für `list`.

<h3 id="plugin-details">
  plugin details
</h3>

Zeigen Sie das Komponenten-Inventar eines Plugins und die geschätzten Token-Kosten an. Die Ausgabe listet alle Komponenten auf, die das Plugin bereitstellt, gruppiert als Skills, Agents, Hooks, MCP-Server und LSP-Server, zusammen mit einer Schätzung, wie viele Token es jeder Sitzung hinzufügt. Die Skills-Gruppe umfasst sowohl `skills/` als auch `commands/` Einträge.

```bash theme={null}
claude plugin details <name>
```

**Argumente:**

* `<name>`: Plugin-Name oder `plugin-name@marketplace-name`

**Optionen:**

| Option       | Beschreibung              | Standard |
| :----------- | :------------------------ | :------- |
| `-h, --help` | Hilfe für Befehl anzeigen |          |

Die Ausgabe zeigt zwei Kostenzahlen für jede Komponente:

* **Always-on:** Token, die jeder Sitzung durch den Auflistungstext des Plugins hinzugefügt werden, wie Skill-Beschreibungen, Agent-Beschreibungen und Befehlsnamen, unabhängig davon, ob eine Komponente ausgelöst wird.
* **On-invoke:** Token, die eine Komponente kostet, wenn sie ausgelöst wird. Wird pro Komponente angezeigt, nicht als Plugin-Gesamtsumme, da eine typische Sitzung nur eine Teilmenge von Komponenten aufruft.

Dieses Beispiel zeigt, wie die Ausgabe für ein Plugin mit zwei Skills aussieht:

```
dependency-guard 1.2.0
  Dependency analysis for Claude Code sessions
  Source: dependency-guard@example-marketplace

Component inventory
  Skills (2)  scan-dependencies, review-changes
  Agents (0)
  Hooks (1)  (harness-only — no model context cost)
  MCP servers (0)
  LSP servers (0)

Projected token cost
  Always-on:   ~180 tok   added to every session

Per-component (rounded)
  component            always-on  on-invoke
  scan-dependencies        ~100      ~2400
  review-changes            ~80      ~1800

  On-invoke cost is paid each time a skill or agent fires.
  Token counts are estimates and may differ from actual usage.
```

Die Always-on-Gesamtsumme wird über die `count_tokens` API für Ihr aktives Modell berechnet. Pro-Komponenten-Zahlen werden proportional von dieser Gesamtsumme skaliert. Wenn die API nicht erreichbar ist, greift der Befehl auf eine zeichenbasierte Schätzung zurück.

<h3 id="plugin-tag">
  plugin tag
</h3>

Erstellen Sie ein Release-Git-Tag für das Plugin im aktuellen Verzeichnis. Führen Sie es im Ordner des Plugins aus. Siehe [Tag-Plugin-Releases](/docs/de/plugin-dependencies#tag-plugin-releases-for-version-resolution).

```bash theme={null}
claude plugin tag [options]
```

**Optionen:**

| Option        | Beschreibung                                                                                 | Standard |
| :------------ | :------------------------------------------------------------------------------------------- | :------- |
| `--push`      | Pushen Sie das Tag zum Remote nach dem Erstellen                                             |          |
| `--dry-run`   | Drucken Sie aus, was getaggt würde, ohne das Tag zu erstellen                                |          |
| `-f, --force` | Erstellen Sie das Tag auch wenn der Arbeitsbaum schmutzig ist oder das Tag bereits existiert |          |
| `-h, --help`  | Hilfe für Befehl anzeigen                                                                    |          |

***

<h2 id="debugging-and-development-tools">
  Debugging- und Entwicklungstools
</h2>

<h3 id="debugging-commands">
  Debugging-Befehle
</h3>

Verwenden Sie `claude --debug`, um Plugin-Lade-Details zu sehen:

Dies zeigt:

* Welche Plugins geladen werden
* Alle Fehler in Plugin-Manifesten
* Skill-, Agent- und Hook-Registrierung
* MCP-Server-Initialisierung

<h3 id="common-issues">
  Häufige Probleme
</h3>

| Problem                             | Ursache                           | Lösung                                                                                                                                                                                |
| :---------------------------------- | :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Plugin wird nicht geladen           | Ungültige `plugin.json`           | Führen Sie `claude plugin validate` oder `/plugin validate` aus, um `plugin.json`, Skill/Agent/Command Frontmatter und `hooks/hooks.json` auf Syntax- und Schema-Fehler zu überprüfen |
| Skills nicht erscheinend            | Falsche Verzeichnisstruktur       | Stellen Sie sicher, dass `skills/` oder `commands/` im Plugin-Root ist, nicht in `.claude-plugin/`                                                                                    |
| Hooks werden nicht ausgelöst        | Skript nicht ausführbar           | Führen Sie `chmod +x script.sh` aus                                                                                                                                                   |
| MCP-Server schlägt fehl             | Fehlender `${CLAUDE_PLUGIN_ROOT}` | Verwenden Sie Variable für alle Plugin-Pfade                                                                                                                                          |
| Pfadfehler                          | Absolute Pfade verwendet          | Alle Pfade müssen relativ sein und mit `./` beginnen                                                                                                                                  |
| LSP `Executable not found in $PATH` | Language Server nicht installiert | Installieren Sie die Binärdatei (z.B. `npm install -g typescript-language-server typescript`)                                                                                         |

<h3 id="example-error-messages">
  Beispiel-Fehlermeldungen
</h3>

**Manifest-Validierungsfehler**:

* `Invalid JSON syntax: Unexpected token } in JSON at position 142`: Überprüfen Sie auf fehlende Kommas, zusätzliche Kommas oder nicht zitierte Strings
* `Plugin has an invalid manifest file at .claude-plugin/plugin.json. Validation errors: name: Required`: Ein erforderliches Feld fehlt
* `Plugin has a corrupt manifest file at .claude-plugin/plugin.json. JSON parse error: ...`: JSON-Syntaxfehler

**Plugin-Ladefehler**:

* `Warning: No commands found in plugin my-plugin custom directory: ./cmds. Expected .md files or SKILL.md in subdirectories.`: Befehlspfad existiert, enthält aber keine gültigen Befehlsdateien
* `Plugin directory not found at path: ./plugins/my-plugin. Check that the marketplace entry has the correct path.`: Der `source` Pfad in marketplace.json verweist auf ein nicht existierendes Verzeichnis
* `Plugin my-plugin has conflicting manifests: both plugin.json and marketplace entry specify components.`: Entfernen Sie doppelte Komponentendefinitionen oder entfernen Sie `strict: false` im Marktplatz-Eintrag

<h3 id="hook-troubleshooting">
  Hook-Fehlerbehebung
</h3>

**Hook-Skript wird nicht ausgeführt**:

1. Überprüfen Sie, dass das Skript ausführbar ist: `chmod +x ./scripts/your-script.sh`
2. Überprüfen Sie die Shebang-Zeile: Erste Zeile sollte `#!/bin/bash` oder `#!/usr/bin/env bash` sein
3. Überprüfen Sie, dass der Pfad `${CLAUDE_PLUGIN_ROOT}` verwendet: `"command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/your-script.sh"`
4. Testen Sie das Skript manuell: `./scripts/your-script.sh`

**Hook wird nicht bei erwarteten Events ausgelöst**:

1. Überprüfen Sie, dass der Event-Name korrekt ist (Groß-/Kleinschreibung beachten): `PostToolUse`, nicht `postToolUse`
2. Überprüfen Sie, dass das Matcher-Muster Ihre Tools passt: `"matcher": "Write|Edit"` für Dateivorgänge
3. Bestätigen Sie, dass der Hook-Typ gültig ist: `command`, `http`, `mcp_tool`, `prompt` oder `agent`

<h3 id="mcp-server-troubleshooting">
  MCP-Server-Fehlerbehebung
</h3>

**Server wird nicht gestartet**:

1. Überprüfen Sie, dass der Befehl existiert und ausführbar ist
2. Überprüfen Sie, dass alle Pfade die `${CLAUDE_PLUGIN_ROOT}` Variable verwenden
3. Überprüfen Sie die MCP-Server-Logs: `claude --debug` zeigt Initialisierungsfehler
4. Testen Sie den Server manuell außerhalb von Claude Code

**Server-Tools erscheinen nicht**:

1. Stellen Sie sicher, dass der Server ordnungsgemäß in `.mcp.json` oder `plugin.json` konfiguriert ist
2. Überprüfen Sie, dass der Server das MCP-Protokoll ordnungsgemäß implementiert
3. Überprüfen Sie auf Verbindungs-Timeouts in der Debug-Ausgabe

<h3 id="directory-structure-mistakes">
  Verzeichnisstruktur-Fehler
</h3>

**Symptome**: Plugin wird geladen, aber Komponenten (Skills, Agents, Hooks) fehlen.

**Korrekte Struktur**: Komponenten müssen sich im Plugin-Root befinden, nicht innerhalb von `.claude-plugin/`. Nur `plugin.json` gehört in `.claude-plugin/`.

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json      ← Nur Manifest hier
├── commands/            ← Auf Root-Ebene
├── agents/              ← Auf Root-Ebene
└── hooks/               ← Auf Root-Ebene
```

Wenn sich Ihre Komponenten in `.claude-plugin/` befinden, verschieben Sie sie in den Plugin-Root.

**Debug-Checkliste**:

1. Führen Sie `claude --debug` aus und suchen Sie nach „loading plugin" Meldungen
2. Überprüfen Sie, dass jedes Komponentenverzeichnis in der Debug-Ausgabe aufgelistet ist
3. Überprüfen Sie Dateiberechtigungen, die das Lesen der Plugin-Dateien ermöglichen

***

<h2 id="distribution-and-versioning-reference">
  Verteilungs- und Versionierungs-Referenz
</h2>

<h3 id="version-management">
  Versionsverwaltung
</h3>

Claude Code verwendet die Version des Plugins als Cache-Schlüssel, der bestimmt, ob ein Update verfügbar ist. Wenn Sie `/plugin update` ausführen oder Auto-Update aktiviert ist, berechnet Claude Code die aktuelle Version und überspringt das Update, wenn es mit der bereits installierten Version übereinstimmt.

Die Version wird aus dem ersten dieser Felder aufgelöst, das gesetzt ist:

1. Das Feld `version` in der `plugin.json` des Plugins
2. Das Feld `version` im Marketplace-Eintrag des Plugins in `marketplace.json`
3. Der Git-Commit-SHA des Plugin-Quellcodes für `github`, `url`, `git-subdir` und relative-path-Quellen in einem Git-gehosteten Marketplace
4. `unknown`, für `npm`-Quellen oder lokale Verzeichnisse, die sich nicht in einem Git-Repository befinden

Dies gibt Ihnen zwei Möglichkeiten, ein Plugin zu versionieren:

| Ansatz                 | Wie                                                                              | Update-Verhalten                                                                                                                                                                          | Am besten für                                       |
| :--------------------- | :------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------- |
| **Explizite Version**  | Setzen Sie `"version": "2.1.0"` in `plugin.json`                                 | Benutzer erhalten Updates nur, wenn Sie dieses Feld erhöhen. Das Pushen neuer Commits ohne Erhöhung hat keine Auswirkung, und `/plugin update` meldet „bereits auf der neuesten Version". | Veröffentlichte Plugins mit stabilen Release-Zyklen |
| **Commit-SHA-Version** | Lassen Sie `version` sowohl in `plugin.json` als auch im Marketplace-Eintrag weg | Benutzer erhalten Updates bei jedem neuen Commit zur Git-Quelle des Plugins                                                                                                               | Interne oder Team-Plugins unter aktiver Entwicklung |

<Warning>
  Wenn Sie `version` in `plugin.json` setzen, müssen Sie es jedes Mal erhöhen, wenn Benutzer Änderungen erhalten sollen. Das bloße Pushen neuer Commits reicht nicht aus, da Claude Code die gleiche Versionszeichenkette sieht und die zwischengespeicherte Kopie behält. Wenn Sie schnell iterieren, lassen Sie `version` ungesetzt, damit stattdessen der Git-Commit-SHA verwendet wird.
</Warning>

Wenn Sie explizite Versionen verwenden, folgen Sie [semantischer Versionierung](https://semver.org) (`MAJOR.MINOR.PATCH`): Erhöhen Sie MAJOR für Breaking Changes, MINOR für neue Features, PATCH für Bugfixes. Dokumentieren Sie Änderungen in einer `CHANGELOG.md`.

***

<h2 id="see-also">
  Siehe auch
</h2>

* [Plugins](/docs/de/plugins) - Tutorials und praktische Verwendung
* [Plugin-Marktplätze](/docs/de/plugin-marketplaces) - Erstellen und Verwalten von Marktplätzen
* [Skills](/docs/de/skills) - Skill-Entwicklungsdetails
* [Subagents](/docs/de/sub-agents) - Agent-Konfiguration und Fähigkeiten
* [Hooks](/docs/de/hooks) - Event-Handling und Automatisierung
* [MCP](/docs/de/mcp) - Integration externer Tools
* [Einstellungen](/docs/de/settings) - Konfigurationsoptionen für Plugins
