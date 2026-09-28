> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK – Übersicht

> Erstellen Sie produktive KI-Agenten mit Claude Code als Bibliothek

Ein Agent ist eine Anwendung, die eine Aufgabe durch Planung eigener Schritte und Aufrufen von Tools abschließt, die Dateien lesen, Befehle ausführen oder Code bearbeiten. Das Agent SDK bietet Ihnen die gleichen Tools, die [Agent-Schleife](/docs/de/agent-sdk/agent-loop) und das Kontextmanagement, die Claude Code antreiben, programmierbar in Python und TypeScript.

<h2 id="compare-the-agent-sdk-to-other-claude-tools">
  Vergleichen Sie das Agent SDK mit anderen Claude-Tools
</h2>

Das Agent SDK, die CLI, das Client SDK und Managed Agents unterscheiden sich darin, wer den Agenten ausführt, welche Funktionen bereits integriert sind und wie Sie darauf zugreifen. Finden Sie die Zeile, die Ihrer gewünschten Vorgehensweise entspricht.

| Sie möchten                                                                                                               | Verwenden Sie                                                                     | Was Sie erhalten                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code's Agenten in Ihrer eigenen Python- oder TypeScript-Anwendung in einem von Ihnen betriebenen Prozess einbetten | **Agent SDK**                                                                     | Eine Bibliothek, die die Claude Code-Binärdatei ausführt, mit Claude Code's [Funktionen](#capabilities), wie z. B. integrierte Tools, Berechtigungen, Sitzungen und Hooks.                                                                                                                                                                                                                                                                        |
| Interaktive Entwicklung durchführen oder einmalige Aufgaben von einem Terminal aus ausführen                              | [**Claude Code CLI**](/docs/de/overview)                                               | Die Terminal-Schnittstelle, entwickelt für tägliche interaktive Nutzung.                                                                                                                                                                                                                                                                                                                                                                          |
| Die Claude API direkt aus Ihrem eigenen Code aufrufen                                                                     | [**Client SDK**](https://platform.claude.com/docs/en/cli-sdks-libraries/overview) | Direkter Zugriff auf die Claude API aus einer beliebigen Client SDK-Sprache. Sie schreiben die Tool-Schleife selbst, oder lassen Sie den [Tool Runner](https://platform.claude.com/docs/en/agents-and-tools/tool-use/tool-runner) des Client SDK diese steuern.                                                                                                                                                                                   |
| Den Agenten von Anthropic hosten lassen, konfiguriert über die Claude API                                                 | [**Managed Agents**](https://platform.claude.com/docs/en/managed-agents/overview) | Ein gehostetes Agent-Harness, das die Agent-Schleife ausführt, mit Sitzungen in einer von Anthropic verwalteten Cloud-Sandbox oder einer [selbst gehosteten Sandbox](https://platform.claude.com/docs/en/managed-agents/self-hosted-sandboxes) auf Ihrer eigenen Infrastruktur. Verwenden Sie es über das [SDK für Ihre Sprache](https://platform.claude.com/docs/en/managed-agents/quickstart#install-the-sdk), die `ant` CLI oder die REST API. |

Um die gleiche Agent-Schleife aus einer anderen Sprache als Python oder TypeScript zu steuern, [führen Sie die CLI als Unterprozess aus](/docs/de/headless) mit dem Flag `-p` und `--output-format json`.

<h2 id="capabilities">
  Funktionen
</h2>

Diese Claude Code-Funktionen sind im SDK verfügbar:

| Funktion                   | Was es tut                                                                                               | Weitere Informationen                                                                                                                                                                                        |
| -------------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Integrierte Tools          | Dateien lesen, schreiben, bearbeiten, Befehle ausführen und das Web durchsuchen                          | [Tools-Referenz](/docs/de/tools-reference)                                                                                                                                                                        |
| Hooks                      | Benutzerdefinierten Code an wichtigen Punkten im Agent-Lebenszyklus ausführen                            | [Hooks](/docs/de/agent-sdk/hooks)                                                                                                                                                                                 |
| Subagenten                 | Spezialisierte Agenten für fokussierte Teilaufgaben spawnen                                              | [Subagenten](/docs/de/agent-sdk/subagents)                                                                                                                                                                        |
| MCP                        | Externe Tools und Datenquellen über das Model Context Protocol verbinden                                 | [MCP](/docs/de/agent-sdk/mcp)                                                                                                                                                                                     |
| Berechtigungen             | Kontrollieren, welche Tools automatisch ausgeführt werden, welche Genehmigung benötigen                  | [Berechtigungen](/docs/de/agent-sdk/permissions)                                                                                                                                                                  |
| Sitzungen                  | Kontext über mehrere Austausche hinweg beibehalten, später fortsetzen oder verzweigen                    | [Sitzungen](/docs/de/agent-sdk/sessions)                                                                                                                                                                          |
| Skills, Befehle und Memory | Automatisch aus dem `.claude/`-Verzeichnis Ihres Projekts und aus `~/.claude/` laden, wie in Claude Code | [Skills](/docs/de/agent-sdk/skills), [Befehle](/docs/de/agent-sdk/skills#commands-in-agent-sdk-sessions), [Memory](/docs/de/agent-sdk/modifying-system-prompts), [Konfigurationsladung](/docs/de/agent-sdk/claude-code-features) |
| Plugins                    | Skills, Agenten, Hooks und MCP-Server verpacken und nach lokalem Pfad laden                              | [Plugins](/docs/de/agent-sdk/plugins)                                                                                                                                                                             |

<h2 id="get-started">
  Erste Schritte
</h2>

Folgen Sie der [Schnellstartanleitung](/docs/de/agent-sdk/quickstart), um das SDK zu installieren, Ihren API-Schlüssel festzulegen und Ihren ersten Agent zu erstellen – einen, der Fehler in vorhandenem Code findet und behebt.

<Note>
  Sofern nicht vorher genehmigt, erlaubt Anthropic Drittentwicklern nicht, claude.ai-Anmeldungen oder Ratenlimits für ihre Produkte anzubieten, einschließlich Agents, die auf dem Claude Agent SDK basieren. Verwenden Sie stattdessen die in der [Schnellstartanleitung](/docs/de/agent-sdk/quickstart) beschriebenen API-Schlüssel-Authentifizierungsmethoden.
</Note>

<h2 id="changelog">
  Änderungsprotokoll
</h2>

Sehen Sie sich das vollständige Änderungsprotokoll für SDK-Updates, Fehlerbehebungen und neue Funktionen an:

* **TypeScript SDK**: [CHANGELOG.md anzeigen](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md)
* **Python SDK**: [CHANGELOG.md anzeigen](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md)

<h2 id="report-bugs">
  Fehler melden
</h2>

Wenn Sie auf Fehler oder Probleme mit dem Agent SDK stoßen:

* **TypeScript SDK**: [Probleme auf GitHub melden](https://github.com/anthropics/claude-agent-sdk-typescript/issues)
* **Python SDK**: [Probleme auf GitHub melden](https://github.com/anthropics/claude-agent-sdk-python/issues)

<h2 id="branding-guidelines">
  Richtlinien für die Markennutzung
</h2>

Für Partner, die das Claude Agent SDK integrieren, ist die Verwendung von Claude-Branding optional. Wenn Sie Claude in Ihrem Produkt referenzieren:

**Erlaubt:**

* „Claude Agent" (bevorzugt für Dropdown-Menüs)
* „Claude" (wenn bereits in einem Menü mit der Bezeichnung „Agents")
* „\{YourAgentName} Powered by Claude" (wenn Sie einen vorhandenen Agentennamen haben)

**Nicht erlaubt:**

* „Claude Code" oder „Claude Code Agent"
* Claude Code-Branding ASCII-Art oder visuelle Elemente, die Claude Code nachahmen

Ihr Produkt sollte sein eigenes Branding beibehalten und nicht wie Claude Code oder ein anderes Anthropic-Produkt aussehen. Wenden Sie sich bei Fragen zur Markenkonformität an das Anthropic-[Vertriebsteam](https://www.anthropic.com/contact-sales).

<h2 id="license-and-terms">
  Lizenz und Bedingungen
</h2>

Die Verwendung des Claude Agent SDK unterliegt den [Anthropic Commercial Terms of Service](https://www.anthropic.com/legal/commercial-terms), auch wenn Sie es verwenden, um Produkte und Dienste bereitzustellen, die Sie Ihren eigenen Kunden und Endbenutzern zur Verfügung stellen, außer soweit eine bestimmte Komponente oder Abhängigkeit unter einer anderen Lizenz abgedeckt ist, wie in der LICENSE-Datei dieser Komponente angegeben.

<h2 id="next-steps">
  Nächste Schritte
</h2>

Diese Ressourcen behandeln tiefere technische Details und Beispielprojekte für die Entwicklung mit dem Agent SDK.

* [Schnellstart](/docs/de/agent-sdk/quickstart): Erstellen Sie Ihren ersten Agenten, der Fehler findet und behebt
* [Migrationsleitfaden](/docs/de/agent-sdk/migration-guide): Migrieren Sie von den Claude Code SDK-Paketen zum Agent SDK
* [Agent-Schleife](/docs/de/agent-sdk/agent-loop): Wie Claude plant, Tools aufruft und entscheidet, wann eine Aufgabe abgeschlossen ist
* [Beispielagenten](https://github.com/anthropics/claude-agent-sdk-demos): Demo-Apps für die lokale Entwicklung
* [TypeScript SDK](/docs/de/agent-sdk/typescript): Vollständige TypeScript-API-Referenz und Beispiele
* [Python SDK](/docs/de/agent-sdk/python): Vollständige Python-API-Referenz und Beispiele
* [Agent-Harness-Design](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code): Wie das Claude Code-Team dynamische Workflows verwendet, um viele Subagenten gleichzeitig zu orchestrieren
