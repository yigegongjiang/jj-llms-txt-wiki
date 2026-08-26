> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Benutzerdefinierte Subagenten erstellen

> Erstellen und verwenden Sie spezialisierte KI-Subagenten in Claude Code für aufgabenspezifische Workflows und verbesserte Kontextverwaltung.

Subagenten sind spezialisierte KI-Assistenten, die bestimmte Arten von Aufgaben bearbeiten. Verwenden Sie einen, wenn eine Nebenaufgabe Ihre Hauptkonversation mit Suchergebnissen, Protokollen oder Dateiinhalten überfluten würde, auf die Sie nicht mehr verweisen werden: Der Subagent führt diese Arbeit in seinem eigenen Kontext durch und gibt nur die Zusammenfassung zurück. Definieren Sie einen benutzerdefinierten Subagenten, wenn Sie wiederholt denselben Typ von Worker mit denselben Anweisungen spawnen.

Jeder Subagent läuft in seinem eigenen Kontextfenster mit einem benutzerdefinierten Systemprompt, spezifischem Werkzeugzugriff und unabhängigen Berechtigungen. Wenn Claude auf eine Aufgabe trifft, die der Beschreibung eines Subagenten entspricht, delegiert es an diesen Subagenten, der unabhängig arbeitet und Ergebnisse zurückgibt. Um die Kontexteinsparungen in der Praxis zu sehen, zeigt die [Kontextfenster-Visualisierung](/docs/de/context-window) eine Sitzung, in der ein Subagent Recherchen in seinem eigenen separaten Fenster durchführt.

<Note>
  Subagenten arbeiten innerhalb einer einzelnen Sitzung. Um viele unabhängige Sitzungen parallel auszuführen und sie von einem Ort aus zu überwachen, siehe [Hintergrund-Agenten](/docs/de/agent-view). Für Sitzungen, die miteinander kommunizieren, siehe [Agent-Teams](/docs/de/agent-teams).
</Note>

Subagenten helfen Ihnen:

* **Kontext bewahren**, indem Sie Exploration und Implementierung aus Ihrer Hauptkonversation heraushalten
* **Einschränkungen durchsetzen**, indem Sie begrenzen, welche Werkzeuge ein Subagent verwenden kann
* **Konfigurationen wiederverwenden** über Projekte hinweg mit Subagenten auf Benutzerebene
* **Verhalten spezialisieren** mit fokussierten Systemprompts für spezifische Domänen
* **Kosten kontrollieren**, indem Sie Aufgaben an schnellere, günstigere Modelle wie Haiku weiterleiten

Claude verwendet die Beschreibung jedes Subagenten, um zu entscheiden, wann Aufgaben delegiert werden. Wenn Sie einen Subagenten erstellen, schreiben Sie eine klare Beschreibung, damit Claude weiß, wann er ihn verwenden soll.

Claude Code enthält mehrere integrierte Subagenten wie Explore, Plan und general-purpose. Sie können auch benutzerdefinierte Subagenten erstellen, um spezifische Aufgaben zu bearbeiten.

<h2 id="built-in-subagents">
  Integrierte Subagenten
</h2>

Claude Code enthält integrierte Subagenten, die Claude automatisch bei Bedarf verwendet. Jeder erbt die Berechtigungen der übergeordneten Konversation mit zusätzlichen Werkzeugbeschränkungen.

Explore und Plan überspringen Ihre CLAUDE.md-Dateien und den Git-Status der übergeordneten Sitzung, um die Recherche schnell und kostengünstig zu halten. Alle anderen integrierten und [benutzerdefinierten Subagenten](#configure-subagents) laden beide. Für die vollständige Aufschlüsselung dessen, was einen Subagenten erreicht, siehe [was beim Start geladen wird](#what-loads-at-startup).

<Tabs>
  <Tab title="Explore">
    Ein schneller, schreibgeschützter Agent, der für die Suche und Analyse von Codebases optimiert ist.

    * **Modell**: Erbt von der Hauptkonversation, begrenzt auf Opus in der Claude API, sodass Explore niemals auf einem teureren Modell ausgeführt wird als dem, das Sie bereits für die Sitzung gewählt haben
    * **Werkzeuge**: Schreibgeschützte Werkzeuge; Write und Edit sind nicht zulässig
    * **Zweck**: Dateiermittlung, Codesuche, Codebase-Exploration

    Ab v2.1.198 erbt Explore das Modell der Hauptkonversation, anstatt immer auf Haiku ausgeführt zu werden. In der Claude API ist das geerbte Modell auf Opus begrenzt: Eine Hauptkonversation auf einer höheren Stufe führt Explore auf Opus aus, und eine Hauptkonversation auf Sonnet oder Haiku führt Explore auf demselben Modell aus. Bei jedem anderen Anbieter, wie z. B. [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry oder Claude Platform on AWS](/docs/de/third-party-integrations), erbt Explore das Modell der Hauptkonversation direkt.

    Ein [Benutzer- oder Projekt-Subagent](#choose-the-subagent-scope) mit dem Namen `Explore` überschreibt den integrierten und behält sein eigenes `model`-Feld, daher definieren Sie einen mit `model: haiku`, um die Exploration auf einem kostengünstigeren Modell zu halten.

    Claude delegiert an Explore, wenn es eine Codebase durchsuchen oder verstehen muss, ohne Änderungen vorzunehmen. Dies hält Explorationsergebnisse aus Ihrem Hauptkonversationskontext heraus.

    Beim Aufrufen von Explore gibt Claude ein Gründlichkeitsniveau an: **quick** für gezielte Lookups, **medium** für ausgewogene Exploration oder **very thorough** für umfassende Analyse.
  </Tab>

  <Tab title="Plan">
    Ein Forschungsagent, der während des [Plan-Modus](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode) verwendet wird, um Kontext zu sammeln, bevor ein Plan präsentiert wird.

    * **Modell**: Erbt von der Hauptkonversation
    * **Werkzeuge**: Schreibgeschützte Werkzeuge; Write und Edit sind nicht zulässig
    * **Zweck**: Codebase-Recherche für Planung

    Wenn Sie sich im Plan-Modus befinden und Claude Ihre Codebase verstehen muss, delegiert es die Recherche an den Plan-Subagenten, damit die Explorationsergebnisse in einem separaten Kontextfenster bleiben, während die Hauptkonversation schreibgeschützt bleibt.
  </Tab>

  <Tab title="General-purpose">
    Ein fähiger Agent für komplexe, mehrstufige Aufgaben, die sowohl Exploration als auch Aktion erfordern.

    * **Modell**: Erbt von der Hauptkonversation
    * **Werkzeuge**: Alle Werkzeuge
    * **Zweck**: Komplexe Recherche, mehrstufige Operationen, Code-Änderungen

    Claude delegiert an general-purpose, wenn die Aufgabe sowohl Exploration als auch Änderung, komplexes Denken zur Interpretation von Ergebnissen oder mehrere abhängige Schritte erfordert.
  </Tab>

  <Tab title="Other">
    Claude Code enthält zusätzliche Hilfagenten für spezifische Aufgaben. Diese werden normalerweise automatisch aufgerufen, daher müssen Sie sie nicht direkt verwenden.

    | Agent             | Modell | Wann Claude ihn verwendet                                              |
    | :---------------- | :----- | :--------------------------------------------------------------------- |
    | statusline-setup  | Sonnet | Wenn Sie `/statusline` ausführen, um Ihre Statuszeile zu konfigurieren |
    | claude-code-guide | Haiku  | Wenn Sie Fragen zu Claude Code-Funktionen stellen                      |
  </Tab>
</Tabs>

Integrierte Subagenten sind in interaktiven Sitzungen standardmäßig registriert. Um sie einzuschränken:

* Um einen bestimmten integrierten Typ zu blockieren, fügen Sie ihn zu `permissions.deny` hinzu, wie in [Spezifische Subagenten deaktivieren](#disable-specific-subagents) gezeigt.
* Um zu verhindern, dass Claude an einen Subagenten delegiert, verweigern Sie das `Agent`-Werkzeug selbst mit [`permissions.deny`](/docs/de/permissions#tool-specific-permission-rules).
* Um nur die integrierten `Explore`- und `Plan`-Subagenten zu entfernen, setzen Sie [`CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS=1`](/docs/de/env-vars). Claude liest und erkundet Dateien direkt, anstatt an sie zu delegieren. Erfordert Claude Code v2.1.198 oder später.
* Im [nicht-interaktiven Modus](/docs/de/headless) und dem [Agent SDK](/docs/de/agent-sdk/overview) setzen Sie [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/de/env-vars), um alle integrierten Typen zu entfernen und nur Ihre eigenen bereitzustellen.

Über diese integrierten Subagenten hinaus können Sie Ihre eigenen mit benutzerdefinierten Prompts, Werkzeugbeschränkungen, Berechtigungsmodi, Hooks und Skills erstellen. Die folgenden Abschnitte zeigen, wie Sie anfangen und Subagenten anpassen.

<h2 id="quickstart-create-your-first-subagent">
  Schnellstart: Erstellen Sie Ihren ersten Subagenten
</h2>

Subagenten sind Markdown-Dateien mit YAML-Frontmatter. Um einen zu erstellen, bitten Sie Claude, ihn für Sie zu schreiben, oder [schreiben Sie die Datei selbst](#write-subagent-files).

Ab v2.1.198 öffnet der `/agents`-Befehl nicht mehr den interaktiven Erstellungs-Assistenten; das Ausführen gibt eine Erinnerung aus, Claude zu fragen oder `.claude/agents/` direkt zu bearbeiten. Subagenten-Dateien, Frontmatter-Felder und die Speicherorte `.claude/agents/` und `~/.claude/agents/` bleiben unverändert; nur der Terminal-Assistent wird entfernt.

Diese Anleitung erstellt einen Subagenten auf Benutzerebene, der Code überprüft und Verbesserungen vorschlägt.

<Steps>
  <Step title="Bitten Sie Claude, den Subagenten zu erstellen">
    Beschreiben Sie in Claude Code den Subagenten, den Sie möchten, und wo Sie ihn speichern möchten:

    ```text wrap theme={null}
    Create a personal code-improver subagent in ~/.claude/agents/ that scans
    files and suggests improvements for readability, performance, and best
    practices. It should explain each issue, show the current code, and
    provide an improved version. Make it read-only and have it use Sonnet.
    ```

    Claude schreibt die Datei mit einem `name`, einer `description`, einer `tools`-Liste, einem `model` und einem Systemprompt.
  </Step>

  <Step title="Überprüfen Sie die Datei">
    Öffnen Sie `~/.claude/agents/code-improver.md` und bestätigen Sie, dass das Frontmatter dem entspricht, was Sie angefordert haben. Das Ergebnis sieht so aus:

    ```markdown theme={null}
    ---
    name: code-improver
    description: Scans files and suggests improvements for readability, performance, and best practices. Use after writing or modifying code.
    tools: Read, Grep, Glob
    model: sonnet
    ---

    You are a code improvement specialist. For each issue you find, explain
    the problem, show the current code, and provide an improved version.
    ```

    Da sich die Datei in `~/.claude/agents/` befindet, ist der Subagent in jedem Projekt auf Ihrem Computer verfügbar. Um ihn stattdessen auf ein Projekt zu beschränken, verschieben Sie ihn in das `.claude/agents/`-Verzeichnis dieses Projekts. [Wählen Sie den Subagenten-Umfang](#choose-the-subagent-scope) vergleicht die beiden.
  </Step>

  <Step title="Probieren Sie es aus">
    Bitten Sie Claude, an den neuen Subagenten zu delegieren:

    ```text wrap theme={null}
    Use the code-improver agent to suggest improvements in this project
    ```

    Claude delegiert an Ihren neuen Subagenten, der die Codebase durchsucht und Verbesserungsvorschläge zurückgibt.

    Wenn Claude den neuen Subagenten nicht finden kann, starten Sie Claude Code neu und versuchen Sie es erneut. Dies geschieht nur, wenn `~/.claude/agents/` vor dem Sitzungsstart nicht vorhanden war, da eine laufende Sitzung ein neu erstelltes `agents`-Verzeichnis nicht erkennt.
  </Step>
</Steps>

Sie haben jetzt einen Subagenten, den Sie in jedem Projekt auf Ihrem Computer verwenden können, um Codebases zu analysieren und Verbesserungen vorzuschlagen.

Sie können Subagenten-Dateien auch manuell schreiben, sie über CLI-Flags definieren oder sie über Plugins verteilen. Die folgenden Abschnitte behandeln alle Konfigurationsoptionen.

<Note>
  In Claude Code v2.1.197 und früher öffnet `/agents` einen interaktiven Assistenten mit einer Registerkarte **Running**, die aktive Subagenten auflistet, und einer Registerkarte **Library** zum Erstellen, Bearbeiten und Löschen.&#x20;
</Note>

<h2 id="configure-subagents">
  Konfigurieren Sie Subagenten
</h2>

Der Dateispeicherort eines Subagenten bestimmt, wer darauf zugreifen kann, und sein Frontmatter bestimmt, was er tun kann. Dieser Abschnitt behandelt, wo Subagenten-Dateien gespeichert werden und welche Felder sie unterstützen.

<h3 id="choose-the-subagent-scope">
  Wählen Sie den Subagenten-Umfang
</h3>

Speichern Sie Subagenten-Dateien an verschiedenen Orten je nach Umfang. Wenn mehrere Subagenten denselben Namen haben, verwendet Claude Code den aus dem höherrangigen Ort.

| Ort                          | Umfang                  | Priorität      | Wie zu erstellen                                             |
| :--------------------------- | :---------------------- | :------------- | :----------------------------------------------------------- |
| Verwaltete Einstellungen     | Organisationsweit       | 1 (höchste)    | Bereitgestellt über [verwaltete Einstellungen](/docs/de/settings) |
| `--agents` CLI-Flag          | Aktuelle Sitzung        | 2              | JSON beim Starten von Claude Code übergeben                  |
| `.claude/agents/`            | Aktuelles Projekt       | 3              | Claude fragen oder die Datei manuell erstellen               |
| `~/.claude/agents/`          | Alle Ihre Projekte      | 4              | Claude fragen oder die Datei manuell erstellen               |
| Plugin-Verzeichnis `agents/` | Wo Plugin aktiviert ist | 5 (niedrigste) | Installiert mit [Plugins](/docs/de/plugins)                       |

**Projekt-Subagenten** (`.claude/agents/`) sind ideal für Subagenten, die spezifisch für eine Codebase sind. Checken Sie sie in die Versionskontrolle ein, damit Ihr Team sie gemeinsam verwenden und verbessern kann.

Projekt-Subagenten werden durch Aufwärts-Traversierung vom aktuellen Arbeitsverzeichnis entdeckt, sodass jedes `.claude/agents/`-Verzeichnis zwischen dort und dem Repository-Root gescannt wird. Ab v2.1.178 verwendet Claude Code, wenn mehr als eines dieser verschachtelten Verzeichnisse denselben `name` definiert, die Definition, die dem Arbeitsverzeichnis am nächsten liegt.

Verzeichnisse, die mit `--add-dir` hinzugefügt werden, werden ebenfalls gescannt: Ein `.claude/agents/`-Ordner in einem hinzugefügten Verzeichnis wird zusammen mit Projekt-Subagenten geladen. Siehe [Zusätzliche Verzeichnisse](/docs/de/permissions#additional-directories-grant-file-access-not-configuration) für welche anderen Konfigurationstypen aus `--add-dir` geladen werden. Um Subagenten über Projekte hinweg zu teilen, ohne `--add-dir` zu verwenden, nutzen Sie `~/.claude/agents/` oder ein [Plugin](/docs/de/plugins).

**Benutzer-Subagenten** (`~/.claude/agents/`) sind persönliche Subagenten, die in allen Ihren Projekten verfügbar sind.

Claude Code scannt `.claude/agents/` und `~/.claude/agents/` rekursiv, sodass Sie Definitionen in Unterordnern wie `agents/review/` oder `agents/research/` organisieren können. Der Unterverzeichnis-Pfad beeinflusst nicht, wie ein Subagent identifiziert oder aufgerufen wird, da die Identität nur vom `name`-Frontmatter-Feld stammt.

Halten Sie `name`-Werte über den gesamten Baum eindeutig: Wenn zwei Dateien unter demselben `.claude/agents/`-Verzeichnis, einschließlich seiner Unterordner, denselben Namen deklarieren, lädt Claude Code nur eine von ihnen, ausgewählt nach Dateisystem-Lesereihenfolge statt nach dokumentierter Priorität. Über verschachtelte Projekt-Verzeichnisse hinweg gewinnt die Definition, die dem Arbeitsverzeichnis am nächsten liegt, wie oben beschrieben. Die [`/doctor`](/docs/de/commands#all-commands)-Setup-Überprüfung meldet Dateien im selben Verzeichnis, die einen Namen teilen, und schlägt vor, alle außer einer umzubenennen oder zu entfernen. Vor v2.1.205 öffnete `/doctor` einen Diagnose-Bildschirm, der Duplikate auflistete und zeigte, welche Definition aktiv war.

Plugin-`agents/`-Verzeichnisse werden ebenfalls rekursiv gescannt. Im Gegensatz zu Projekt- und Benutzer-Umfängen wird ein Unterordner in einem Plugin-`agents/`-Verzeichnis Teil des [scoped identifier](#invoke-subagents-explicitly): Eine Datei unter `agents/review/security.md` im Plugin `my-plugin` registriert sich als `my-plugin:review:security`.

**CLI-definierte Subagenten** werden als JSON beim Starten von Claude Code übergeben. Sie existieren nur für diese Sitzung und werden nicht auf der Festplatte gespeichert, was sie für schnelle Tests oder Automatisierungsskripte nützlich macht. Sie können mehrere Subagenten in einem einzigen `--agents`-Aufruf definieren:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    claude --agents '{
      "code-reviewer": {
        "description": "Expert code reviewer. Use proactively after code changes.",
        "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
        "tools": ["Read", "Grep", "Glob", "Bash"],
        "model": "sonnet"
      },
      "debugger": {
        "description": "Debugging specialist for errors and test failures.",
        "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
      }
    }'
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    claude --agents @'
    {
      "code-reviewer": {
        "description": "Expert code reviewer. Use proactively after code changes.",
        "prompt": "You are a senior code reviewer. Focus on code quality, security, and best practices.",
        "tools": ["Read", "Grep", "Glob", "Bash"],
        "model": "sonnet"
      },
      "debugger": {
        "description": "Debugging specialist for errors and test failures.",
        "prompt": "You are an expert debugger. Analyze errors, identify root causes, and provide fixes."
      }
    }
    '@
    ```
  </Tab>
</Tabs>

Das `--agents`-Flag akzeptiert JSON mit denselben [Frontmatter](#supported-frontmatter-fields)-Feldern wie dateibasierte Subagenten: `description`, `prompt`, `tools`, `disallowedTools`, `model`, `permissionMode`, `mcpServers`, `hooks`, `maxTurns`, `skills`, `initialPrompt`, `memory`, `effort`, `background`, `isolation` und `color`. Verwenden Sie `prompt` für den Systemprompt, äquivalent zum Markdown-Body in dateibasierten Subagenten.

**Verwaltete Subagenten** werden von Organisationsadministratoren bereitgestellt. Platzieren Sie Markdown-Dateien in `.claude/agents/` im [Verzeichnis der verwalteten Einstellungen](/docs/de/settings#settings-files), wobei Sie das gleiche Frontmatter-Format wie bei Projekt- und Benutzer-Subagenten verwenden. Verwaltete Definitionen haben Vorrang vor Projekt- und Benutzer-Subagenten mit demselben Namen.

**Plugin-Subagenten** stammen von [Plugins](/docs/de/plugins), die Sie installiert haben. Sie werden zusammen mit Ihren benutzerdefinierten Subagenten geladen und erscheinen in der @-Erwähnung-Typeahead unter ihrem scoped name. Siehe die [Plugin-Komponenten-Referenz](/docs/de/plugins-reference#agents) für Details zum Erstellen von Plugin-Subagenten.

<Note>
  Aus Sicherheitsgründen unterstützen Plugin-Subagenten die Frontmatter-Felder `hooks`, `mcpServers` oder `permissionMode` nicht. Diese Felder werden ignoriert, wenn Agenten aus einem Plugin geladen werden. Wenn Sie sie benötigen, kopieren Sie die Agent-Datei in `.claude/agents/` oder `~/.claude/agents/`. Sie können auch Regeln zu [`permissions.allow`](/docs/de/settings#permission-settings) in `settings.json` oder `settings.local.json` hinzufügen, aber diese Regeln gelten für die gesamte Sitzung, nicht nur für den Plugin-Subagenten.
</Note>

Subagenten-Definitionen aus einem dieser Umfänge sind auch für [Agent-Teams](/docs/de/agent-teams#use-subagent-definitions-for-teammates) verfügbar: Beim Spawnen eines Teammates können Sie auf einen Subagenten-Typ verweisen und der Teammate erbt seine `tools` und sein `model`, wobei der Body der Definition als zusätzliche Anweisungen an den Systemprompt des Teammates angehängt wird. Siehe [Agent-Teams](/docs/de/agent-teams#use-subagent-definitions-for-teammates) für welche Frontmatter-Felder auf diesem Pfad gelten.

<h3 id="write-subagent-files">
  Schreiben Sie Subagenten-Dateien
</h3>

Subagenten-Dateien verwenden YAML-Frontmatter für die Konfiguration, gefolgt vom Systemprompt in Markdown:

<Note>
  Claude Code überwacht `~/.claude/agents/` und `.claude/agents/`. Wenn Sie eine Subagenten-Datei auf der Festplatte hinzufügen oder bearbeiten oder Claude bitten, eine für Sie zu schreiben, erkennt Claude Code die Änderung innerhalb weniger Sekunden und die nächste Delegation verwendet die aktualisierte Definition, ohne dass ein Neustart erforderlich ist.

  Zwei Fälle erfordern immer noch einen Neustart:

  * Der Watcher deckt nur Verzeichnisse ab, die beim Sitzungsstart existierten, daher müssen Sie nach dem Erstellen der ersten Agent-Datei eines Umfangs in einem neuen `agents`-Verzeichnis neu starten, um sie zu laden.
  * Sitzungen, die mit `--disable-slash-commands` gestartet wurden, überwachen diese Verzeichnisse überhaupt nicht.
</Note>

```markdown theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
tools: Read, Glob, Grep
model: sonnet
---

You are a code reviewer. When invoked, analyze the code and provide
specific, actionable feedback on quality, security, and best practices.
```

Das Frontmatter definiert die Metadaten und Konfiguration des Subagenten. Der Body wird zum Systemprompt, der das Verhalten des Subagenten leitet. Subagenten erhalten nur diesen Systemprompt plus grundlegende Umgebungsdetails wie Arbeitsverzeichnis, nicht den vollständigen Claude Code-Systemprompt.

Im [nicht-interaktiven Modus](/docs/de/headless) hängt das Flag [`--append-subagent-system-prompt`](/docs/de/cli-reference#cli-flags) den von Ihnen bereitgestellten Text an das Ende des Systemprompts jedes Subagenten an, einschließlich verschachtelter Subagenten. Erfordert Claude Code v2.1.205 oder später.

Ein Subagent startet im aktuellen Arbeitsverzeichnis der Hauptkonversation. Innerhalb eines Subagenten bleiben `cd`-Befehle nicht zwischen Bash- oder PowerShell-Werkzeugaufrufen bestehen und beeinflussen nicht das Arbeitsverzeichnis der Hauptkonversation. Um dem Subagenten stattdessen eine isolierte Kopie des Repositorys zu geben, setzen Sie [`isolation: worktree`](#supported-frontmatter-fields).

Ein Subagent mit `isolation: worktree` führt seine Bash- und PowerShell-Befehle in seinem Worktree aus. Ein Befehl, dessen Arbeitsverzeichnis sich stattdessen zu Ihrem Haupt-Checkout auflöst, beispielsweise weil das Worktree-Verzeichnis entfernt wurde, während der Subagent lief, schlägt mit einem Fehler fehl. Vor v2.1.203 konnte ein solcher Befehl im Haupt-Checkout ausgeführt werden.

<h4 id="supported-frontmatter-fields">
  Unterstützte Frontmatter-Felder
</h4>

Die folgenden Felder können im YAML-Frontmatter verwendet werden. Nur `name` und `description` sind erforderlich.

| Feld              | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                      |
| :---------------- | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`            | Ja           | Eindeutige Kennung mit Kleinbuchstaben und Bindestrichen. [Hooks](/docs/de/hooks#subagentstart) erhalten diesen Wert als `agent_type`. Der Dateiname muss nicht übereinstimmen                                                                                                                                                                                                                         |
| `description`     | Ja           | Wann Claude an diesen Subagenten delegieren sollte                                                                                                                                                                                                                                                                                                                                                |
| `tools`           | Nein         | [Werkzeuge](#available-tools), die der Subagent verwenden kann. Erbt alle Werkzeuge, wenn weggelassen. Wenn kein Eintrag in der Liste sich zu einem Werkzeug auflöst, schlägt der Subagent mit einem Fehler fehl, der die Einträge benennt. Um Skills in den Kontext zu laden, verwenden Sie das `skills`-Feld statt `Skill` hier aufzulisten                                                     |
| `disallowedTools` | Nein         | Werkzeuge zum Verweigern, entfernt aus geerbter oder angegebener Liste                                                                                                                                                                                                                                                                                                                            |
| `model`           | Nein         | [Modell](#choose-a-model) zu verwenden: `sonnet`, `opus`, `haiku`, `fable`, eine vollständige Modell-ID (z. B. `claude-opus-4-8`) oder `inherit`. Standard ist `inherit`                                                                                                                                                                                                                          |
| `permissionMode`  | Nein         | [Berechtigungsmodus](#permission-modes): `default`, `acceptEdits`, `auto`, `dontAsk`, `bypassPermissions`, `plan` oder `manual` als Alias für `default`. Der `manual`-Alias erfordert Claude Code v2.1.200 oder später. Ignoriert für [Plugin-Subagenten](#choose-the-subagent-scope)                                                                                                             |
| `maxTurns`        | Nein         | Maximale Anzahl von Agenten-Turns, bevor der Subagent stoppt                                                                                                                                                                                                                                                                                                                                      |
| `skills`          | Nein         | [Skills](/docs/de/skills) zum Vorausladen in den Kontext des Subagenten beim Start. Der vollständige Skill-Inhalt wird eingespritzt, nicht nur die Beschreibung. Subagenten können weiterhin unlisted Projekt-, Benutzer- und Plugin-Skills durch das Skill-Werkzeug aufrufen                                                                                                                          |
| `mcpServers`      | Nein         | [MCP-Server](/docs/de/mcp) verfügbar für diesen Subagenten. Jeder Eintrag ist entweder ein Servername, der auf einen bereits konfigurierten Server verweist (z. B. `"slack"`) oder eine Inline-Definition mit dem Servernamen als Schlüssel und einer vollständigen [MCP-Server-Konfiguration](/docs/de/mcp#installing-mcp-servers) als Wert. Ignoriert für [Plugin-Subagenten](#choose-the-subagent-scope) |
| `hooks`           | Nein         | [Lifecycle-Hooks](#define-hooks-for-subagents) mit Umfang auf diesen Subagenten. Ignoriert für [Plugin-Subagenten](#choose-the-subagent-scope)                                                                                                                                                                                                                                                    |
| `memory`          | Nein         | [Persistenter Speicherumfang](#enable-persistent-memory): `user`, `project` oder `local`. Ermöglicht sitzungsübergreifendes Lernen                                                                                                                                                                                                                                                                |
| `background`      | Nein         | Auf `true` setzen, um diesen Subagenten immer als [Hintergrundaufgabe](#run-subagents-in-foreground-or-background) auszuführen, auch wenn Claude sein Ergebnis sofort benötigt. Wenn nicht gesetzt, wählt Claude, und ab v2.1.198 führt es Subagenten standardmäßig im Hintergrund aus                                                                                                            |
| `effort`          | Nein         | Aufwandsstufe, wenn dieser Subagent aktiv ist. Überschreibt die Aufwandsstufe der Sitzung. Standard: erbt von Sitzung. Optionen: `low`, `medium`, `high`, `xhigh`, `max`; verfügbare Stufen hängen vom Modell ab                                                                                                                                                                                  |
| `isolation`       | Nein         | Auf `worktree` setzen, um den Subagenten in einem temporären [Git-Worktree](/docs/de/worktrees) auszuführen, was ihm eine isolierte Kopie des Repositorys gibt, die standardmäßig von Ihrem [Standard-Branch](/docs/de/worktrees#choose-the-base-branch) verzweigt ist, anstatt vom `HEAD` der übergeordneten Sitzung. Der Worktree wird automatisch bereinigt, wenn der Subagent keine Änderungen vornimmt |
| `color`           | Nein         | Anzeigefarbe für den Subagenten in der Aufgabenliste und dem Transkript. Akzeptiert `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink` oder `cyan`                                                                                                                                                                                                                                      |
| `initialPrompt`   | Nein         | Auto-eingereicht als der erste Benutzer-Turn, wenn dieser Agent als Hauptsitzungs-Agent läuft (über `--agent` oder die `agent`-Einstellung). [Befehle](/docs/de/commands) und [Skills](/docs/de/skills) werden verarbeitet. Vorangestellt zu jedem vom Benutzer bereitgestellten Prompt                                                                                                                     |

<h3 id="choose-a-model">
  Wählen Sie ein Modell
</h3>

Das `model`-Feld steuert, welches [KI-Modell](/docs/de/model-config) der Subagent verwendet:

* **Modell-Alias**: Verwenden Sie einen der verfügbaren Aliase: `sonnet`, `opus`, `haiku` oder `fable`
* **Vollständige Modell-ID**: Verwenden Sie eine vollständige Modell-ID wie `claude-opus-4-8` oder `claude-sonnet-5`. Akzeptiert dieselben Werte wie das `--model`-Flag
* **inherit**: Verwenden Sie dasselbe Modell wie die Hauptkonversation
* **Omitted**: Wenn nicht angegeben, wird standardmäßig `inherit` verwendet (verwendet dasselbe Modell wie die Hauptkonversation)

Wenn Claude einen Subagenten aufruft, kann es auch einen `model`-Parameter für diese spezifische Invokation übergeben. Claude Code löst das Modell des Subagenten in dieser Reihenfolge auf:

1. Die Umgebungsvariable [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/de/model-config#environment-variables), wenn gesetzt auf einen Modell-Alias oder eine Modell-ID
2. Der `model`-Parameter pro Invokation
3. Das `model`-Frontmatter der Subagenten-Definition
4. Das Modell der Hauptkonversation

Ab v2.1.196 ist das Setzen von `CLAUDE_CODE_SUBAGENT_MODEL` auf `inherit` dasselbe wie das Nichtsetzen: Die Auflösung wird mit dem `model`-Parameter pro Invokation fortgesetzt, dann mit dem Frontmatter. In früheren Versionen zwang `inherit` Subagenten auf das Modell der Hauptkonversation und ignorierte beide dieser Quellen.

Claude Code überprüft die Umgebungsvariable, den Parameter pro Invokation und die Frontmatter-Werte gegen die [`availableModels`](/docs/de/model-config#restrict-model-selection)-Allowlist Ihrer Organisation. Ein Wert, der sich zu einem ausgeschlossenen Modell auflöst, wird übersprungen und der Subagent läuft stattdessen auf dem geerbten Modell.

Ab v2.1.198 erben Subagenten auch die [Extended Thinking](/docs/de/model-config#extended-thinking)-Konfiguration der Hauptkonversation: Wenn Thinking in Ihrer Sitzung aktiviert ist, ist es für den Subagenten aktiviert, und wenn es deaktiviert ist, bleibt es deaktiviert. Es gibt keine Pro-Subagenten-Thinking-Einstellung. Vor v2.1.198 liefen Subagenten mit deaktiviertem Extended Thinking, unabhängig von der Einstellung der Hauptkonversation.

<h3 id="control-subagent-capabilities">
  Kontrollieren Sie Subagenten-Fähigkeiten
</h3>

Sie können kontrollieren, was Subagenten durch Werkzeugzugriff, Berechtigungsmodi und bedingte Regeln tun können.

<h4 id="available-tools">
  Verfügbare Werkzeuge
</h4>

Subagenten erben die [internen Werkzeuge](/docs/de/tools-reference) und MCP-Werkzeuge, die in der Hauptkonversation verfügbar sind, standardmäßig. Die folgenden Werkzeuge hängen von der Benutzeroberfläche oder dem Sitzungszustand der Hauptkonversation ab und sind nicht für Subagenten verfügbar, auch wenn sie im `tools`-Feld aufgelistet sind:

* `AskUserQuestion`
* `EnterPlanMode`
* `ExitPlanMode`, es sei denn, der [`permissionMode`](#permission-modes) des Subagenten ist `plan`
* `ScheduleWakeup`
* `WaitForMcpServers`

Um Werkzeuge einzuschränken, verwenden Sie das `tools`-Feld als Allowlist oder das `disallowedTools`-Feld als Denylist. Dieses Beispiel verwendet `tools`, um ausschließlich Read, Grep, Glob und Bash zuzulassen. Der Subagent kann keine Dateien bearbeiten, keine Dateien schreiben oder MCP-Werkzeuge verwenden:

```yaml theme={null}
---
name: safe-researcher
description: Research agent with restricted capabilities
tools: Read, Grep, Glob, Bash
---
```

Dieses Beispiel verwendet `disallowedTools`, um alle Werkzeuge von der Hauptkonversation zu erben, außer Write und Edit. Der Subagent behält Bash, MCP-Werkzeuge und alles andere:

```yaml theme={null}
---
name: no-writes
description: Inherits every tool except file writes
disallowedTools: Write, Edit
---
```

Wenn beide gesetzt sind, wird `disallowedTools` zuerst angewendet, dann wird `tools` gegen den verbleibenden Pool aufgelöst. Ein Werkzeug, das in beiden aufgelistet ist, wird entfernt.

Wenn nichts in der `tools`-Liste sich zu einem Werkzeug auflöst, beispielsweise weil jeder Eintrag falsch geschrieben ist oder ein Werkzeug benennt, das nicht für Subagenten verfügbar ist, weigert sich Claude Code, den Subagenten zu starten, und das Agent-Werkzeug gibt einen Fehler zurück, der die ungelösten Einträge benennt. Vor v2.1.208 startete dieser Subagent mit keinen Werkzeugen und konnte ein leeres oder verwirrendes Ergebnis zurückgeben.

Beide Felder akzeptieren MCP-Server-Level-Muster zusätzlich zu exakten Werkzeugnamen: `mcp__<server>` oder `mcp__<server>__*` gewährt oder entfernt jedes Werkzeug vom benannten Server. In `disallowedTools` entfernt `mcp__*` auch jedes MCP-Werkzeug von jedem Server. Dieses Beispiel entfernt jedes Werkzeug vom `github` MCP-Server, während Werkzeuge von anderen Servern und jedes integrierte Werkzeug beibehalten werden:

```yaml theme={null}
---
name: local-only
description: Inherits every tool except those from the github MCP server
disallowedTools: mcp__github
---
```

<h4 id="restrict-which-subagents-can-be-spawned">
  Beschränken Sie, welche Subagenten gespawnt werden können
</h4>

Wenn ein Agent als Hauptthread mit `claude --agent` läuft, kann er Subagenten mit dem Agent-Werkzeug spawnen. Um zu beschränken, welche Subagenten-Typen er spawnen kann, verwenden Sie die `Agent(agent_type)`-Syntax im `tools`-Feld.

<Note>In Version 2.1.63 wurde das Task-Werkzeug in Agent umbenannt. Vorhandene `Task(...)`-Verweise in Einstellungen und Agent-Definitionen funktionieren weiterhin als Aliase.</Note>

```yaml theme={null}
---
name: coordinator
description: Coordinates work across specialized agents
tools: Agent(worker, researcher), Read, Bash
---
```

Dies ist eine Allowlist: Nur die `worker`- und `researcher`-Subagenten können gespawnt werden. Wenn der Agent versucht, einen anderen Typ zu spawnen, schlägt die Anfrage fehl und der Agent sieht nur die zulässigen Typen in seinem Prompt. Um bestimmte Agenten zu blockieren und alle anderen zuzulassen, verwenden Sie stattdessen [`permissions.deny`](#disable-specific-subagents).

Um das Spawnen eines beliebigen Subagenten ohne Einschränkungen zu ermöglichen, verwenden Sie `Agent` ohne Klammern:

```yaml theme={null}
tools: Agent, Read, Bash
```

Wenn `Agent` vollständig aus der `tools`-Liste weggelassen wird, kann der Agent keine Subagenten spawnen.

Die `Agent(agent_type)`-Allowlist-Syntax gilt nur für einen Agent, der als Hauptthread mit `claude --agent` läuft. In einer Subagenten-Definition ermöglicht das Auflisten von `Agent` in `tools` diesem Subagenten, [verschachtelte Subagenten zu spawnen](#spawn-nested-subagents), aber jede Typenliste in den Klammern wird ignoriert.

<h4 id="scope-mcp-servers-to-a-subagent">
  Umfang von MCP-Servern auf einen Subagenten
</h4>

Verwenden Sie das `mcpServers`-Feld, um einem Subagenten Zugriff auf [MCP](/docs/de/mcp)-Server zu geben, die in der Hauptkonversation nicht verfügbar sind. Inline-Server, die hier definiert sind, werden verbunden, wenn der Subagent startet, und getrennt, wenn er endet. String-Verweise teilen die Verbindung der übergeordneten Sitzung.

<Note>
  Das `mcpServers`-Feld gilt in beiden Kontexten, in denen eine Agent-Datei ausgeführt werden kann:

  * Als Subagent, gespawnt durch das Agent-Werkzeug oder eine @-Erwähnung
  * Als Hauptsitzung, gestartet mit [`--agent`](#invoke-subagents-explicitly) oder der `agent`-Einstellung

  Wenn der Agent die Hauptsitzung ist, verbinden sich Inline-Server-Definitionen beim Start zusammen mit Servern aus [`.mcp.json`](/docs/de/mcp) und Einstellungsdateien.
</Note>

Jeder Eintrag in der Liste ist entweder eine Inline-Server-Definition oder ein String, der auf einen bereits konfigurierten MCP-Server in Ihrer Sitzung verweist:

```yaml theme={null}
---
name: browser-tester
description: Tests features in a real browser using Playwright
mcpServers:
  # Inline definition: scoped to this subagent only
  - playwright:
      type: stdio
      command: npx
      args: ["-y", "@playwright/mcp@latest"]
  # Reference by name: reuses an already-configured server
  - github
---

Use the Playwright tools to navigate, screenshot, and interact with pages.
```

Inline-Definitionen verwenden dasselbe Schema wie `.mcp.json`-Server-Einträge, mit dem Servernamen als Schlüssel, und unterstützen die Typen `stdio`, `http`, `sse` und `ws`.

Um einen MCP-Server vollständig aus der Hauptkonversation herauszuhalten und zu vermeiden, dass seine Werkzeugbeschreibungen dort Kontext verbrauchen, definieren Sie ihn inline hier statt in `.mcp.json`. Der Subagent erhält die Werkzeuge; die übergeordnete Konversation nicht.

Ab v2.1.153 gelten die MCP-Einschränkungen, die für die Hauptsitzung gelten, auch für Server, die im Subagenten-Frontmatter deklariert sind:

* [`--strict-mcp-config`](/docs/de/cli-reference) und [`--bare`](/docs/de/cli-reference)
* [Enterprise verwaltete MCP-Konfiguration](/docs/de/managed-mcp)
* [`allowedMcpServers` und `deniedMcpServers` Richtlinien](/docs/de/managed-mcp#policy-based-control-with-allowlists-and-denylists)

Wenn einer dieser Punkte einen Server blockiert, überspringt Claude Code ihn und zeigt eine Warnung mit den Namen der blockierten Server an.

Verwaltete Einstellungseinschränkungen gelten für jeden Subagenten, unabhängig davon, wie er definiert ist. `--strict-mcp-config` filtert keine Server, die Sie inline über `--agents` oder die SDK-Option `agents` übergeben, da diese explizite Eingaben des Aufrufers sind.

<h4 id="permission-modes">
  Berechtigungsmodi
</h4>

Das `permissionMode`-Feld steuert, wie der Subagent Berechtigungsaufforderungen bearbeitet. Subagenten erben den Berechtigungskontext von der Hauptkonversation und können den Modus überschreiben, außer wenn der übergeordnete Modus Vorrang hat, wie unten beschrieben.

| Modus               | Verhalten                                                                                                                                                                                                                                                                                                                                                                                                                            |
| :------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Standardberechtigungsprüfung mit Aufforderungen                                                                                                                                                                                                                                                                                                                                                                                      |
| `acceptEdits`       | Automatische Akzeptanz von Dateibearbeitungen und häufigen Dateisystem-Befehlen für Pfade im Arbeitsverzeichnis oder `additionalDirectories`                                                                                                                                                                                                                                                                                         |
| `auto`              | [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode): ein KI-Klassifizierer bewertet Befehle und Schreibvorgänge in geschützten Verzeichnissen                                                                                                                                                                                                                                                                        |
| `dontAsk`           | Automatische Ablehnung von Berechtigungsaufforderungen. Explizit zulässige Werkzeuge funktionieren weiterhin; `AskUserQuestion`, Connector-Werkzeuge [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und MCP-Werkzeuge, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, werden verweigert, auch wenn Sie sie zugelassen haben |
| `bypassPermissions` | Alle Berechtigungsprüfungen überspringen                                                                                                                                                                                                                                                                                                                                                                                             |
| `plan`              | Plan-Modus (schreibgeschützte Exploration)                                                                                                                                                                                                                                                                                                                                                                                           |

<Warning>
  Verwenden Sie `bypassPermissions` mit Vorsicht. Es überspringt Berechtigungsaufforderungen und ermöglicht dem Subagenten, Operationen ohne Genehmigung auszuführen, einschließlich Schreibvorgänge in `.git`, `.config/git`, `.claude`, `.vscode`, `.idea`, `.husky`, `.cargo`, `.devcontainer`, `.yarn` und `.mvn`.

  Explizite [`ask`-Regeln](/docs/de/permissions#manage-permissions), Connector-Werkzeuge [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools), MCP-Werkzeuge, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, und Root- und Home-Verzeichnis-Entfernungen wie `rm -rf /` werden weiterhin aufgefordert. Siehe [Berechtigungsmodi](/docs/de/permission-modes#skip-all-checks-with-bypasspermissions-mode) für Details.
</Warning>

Wenn das übergeordnete Element `bypassPermissions` oder `acceptEdits` verwendet, hat dies Vorrang und kann nicht überschrieben werden. Wenn das übergeordnete Element den [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) verwendet, erbt der Subagent den Auto-Modus und jedes `permissionMode` in seinem Frontmatter wird ignoriert: der Klassifizierer bewertet die Werkzeugaufrufe des Subagenten mit denselben Block- und Zulassungsregeln wie die übergeordnete Sitzung.

<h4 id="preload-skills-into-subagents">
  Laden Sie Skills in Subagenten vor
</h4>

Verwenden Sie das `skills`-Feld, um Skill-Inhalte beim Start in den Kontext eines Subagenten einzuspeisen. Dies gibt dem Subagenten Domänenwissen, ohne dass er Skills während der Ausführung entdecken und laden muss.

```yaml theme={null}
---
name: api-developer
description: Implement API endpoints following team conventions
skills:
  - api-conventions
  - error-handling-patterns
---

Implement API endpoints. Follow the conventions and patterns from the preloaded skills.
```

Der vollständige Inhalt jedes aufgelisteten Skills wird in den Kontext des Subagenten eingespritzt. Dieses Feld steuert, welche Skills vorausgeladen werden, nicht welche Skills der Subagent zugreifen kann: ohne es kann der Subagent weiterhin Projekt-, Benutzer- und Plugin-Skills durch das Skill-Werkzeug während der Ausführung entdecken und aufrufen. Um einen Subagenten daran zu hindern, Skills überhaupt aufzurufen, lassen Sie `Skill` aus der [`tools`](#available-tools)-Liste weg oder fügen Sie es zu `disallowedTools` hinzu.

Sie können keine Skills vorausladen, die [`disable-model-invocation: true`](/docs/de/skills#control-who-invokes-a-skill) setzen, da das Vorausladen aus demselben Satz von Skills stammt, die Claude aufrufen kann. Wenn ein aufgelisteter Skill fehlt oder deaktiviert ist, überspringt Claude Code ihn und protokolliert eine Warnung im Debug-Protokoll.

<Note>
  Dies ist das Gegenteil von [Ausführen eines Skills in einem Subagenten](/docs/de/skills#run-skills-in-a-subagent). Mit `skills` in einem Subagenten kontrolliert der Subagent den Systemprompt und lädt Skill-Inhalte. Mit `context: fork` in einem Skill wird der Skill-Inhalt in den von Ihnen angegebenen Agent eingespritzt. Beide verwenden dasselbe zugrunde liegende System.
</Note>

<h4 id="enable-persistent-memory">
  Aktivieren Sie persistenten Speicher
</h4>

Das `memory`-Feld gibt dem Subagenten ein persistentes Verzeichnis, das über Konversationen hinweg bestehen bleibt. Der Subagent verwendet dieses Verzeichnis, um im Laufe der Zeit Wissen aufzubauen, wie z. B. Codebase-Muster, Debugging-Erkenntnisse und architektonische Entscheidungen.

```yaml theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
memory: user
---

You are a code reviewer. As you review code, update your agent memory with
patterns, conventions, and recurring issues you discover.
```

Wählen Sie einen Umfang basierend darauf, wie breit der Speicher angewendet werden sollte:

| Umfang    | Ort                                           | Verwenden Sie, wenn                                                                                            |
| :-------- | :-------------------------------------------- | :------------------------------------------------------------------------------------------------------------- |
| `user`    | `~/.claude/agent-memory/<name-of-agent>/`     | der Subagent Erkenntnisse über alle Projekte hinweg merken sollte                                              |
| `project` | `.claude/agent-memory/<name-of-agent>/`       | das Wissen des Subagenten projektspezifisch ist und über Versionskontrolle teilbar ist                         |
| `local`   | `.claude/agent-memory-local/<name-of-agent>/` | das Wissen des Subagenten projektspezifisch ist, aber nicht in die Versionskontrolle eingecheckt werden sollte |

Wenn der Speicher aktiviert ist:

* Der Systemprompt des Subagenten enthält Anweisungen zum Lesen und Schreiben in das Speicherverzeichnis.
* Der Systemprompt des Subagenten enthält auch die ersten 200 Zeilen oder 25 KB von `MEMORY.md` im Speicherverzeichnis, je nachdem, was zuerst kommt, mit Anweisungen zur Verwaltung von `MEMORY.md`, wenn es diese Grenze überschreitet.
* Read-, Write- und Edit-Werkzeuge werden automatisch aktiviert, damit der Subagent seine Speicherdateien verwalten kann.

<h5 id="persistent-memory-tips">
  Tipps zum persistenten Speicher
</h5>

* `project` ist der empfohlene Standard-Umfang. Es macht Subagenten-Wissen über Versionskontrolle teilbar.
* Bitten Sie den Subagenten, seinen Speicher vor dem Start zu konsultieren: "Review this PR, and check your memory for patterns you've seen before."
* Bitten Sie den Subagenten, seinen Speicher nach Abschluss einer Aufgabe zu aktualisieren: "Now that you're done, save what you learned to your memory." Im Laufe der Zeit baut dies eine Wissensdatenbank auf, die den Subagenten effektiver macht.
* Fügen Sie Speicheranweisungen direkt in die Markdown-Datei des Subagenten ein, damit er proaktiv seine eigene Wissensdatenbank verwaltet:

  ```markdown theme={null}
  Update your agent memory as you discover codepaths, patterns, library
  locations, and key architectural decisions. This builds up institutional
  knowledge across conversations. Write concise notes about what you found
  and where.
  ```

<h4 id="conditional-rules-with-hooks">
  Bedingte Regeln mit Hooks
</h4>

Für dynamischere Kontrolle über die Werkzeugnutzung verwenden Sie `PreToolUse`-Hooks, um Operationen vor ihrer Ausführung zu validieren. Dies ist nützlich, wenn Sie einige Operationen eines Werkzeugs zulassen möchten, während Sie andere blockieren.

Dieses Beispiel erstellt einen Subagenten, der nur schreibgeschützte Datenbankabfragen zulässt. Der `PreToolUse`-Hook führt das in `command` angegebene Skript vor jeder Bash-Befehlsausführung aus:

```yaml theme={null}
---
name: db-reader
description: Execute read-only database queries
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---
```

Claude Code [übergibt Hook-Eingabe als JSON](/docs/de/hooks#pretooluse-input) über stdin an Hook-Befehle. Das Validierungsskript liest dieses JSON, extrahiert den Bash-Befehl und [beendet mit Code 2](/docs/de/hooks#exit-code-2-behavior-per-event), um Schreibvorgänge zu blockieren:

```bash theme={null}
#!/bin/bash
# ./scripts/validate-readonly-query.sh

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# Block SQL write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE)\b' > /dev/null; then
  echo "Blocked: Only SELECT queries are allowed" >&2
  exit 2
fi

exit 0
```

Siehe [Hook-Eingabe](/docs/de/hooks#pretooluse-input) für das vollständige Eingabeschema und [Exit-Codes](/docs/de/hooks#exit-code-output) für die Auswirkungen von Exit-Codes auf das Verhalten. Unter Windows schreiben Sie Hook-Skripte in PowerShell und fügen Sie `shell: powershell` zum Hook-Eintrag hinzu, wie in [Ausführen von Hooks in PowerShell](/docs/de/hooks#windows-powershell-tool) gezeigt.

<h4 id="disable-specific-subagents">
  Deaktivieren Sie spezifische Subagenten
</h4>

Sie können verhindern, dass Claude bestimmte Subagenten verwendet, indem Sie sie zum `deny`-Array in Ihren [Einstellungen](/docs/de/settings#permission-settings) hinzufügen. Verwenden Sie das Format `Agent(subagent-name)`, wobei `subagent-name` dem `name`-Feld des Subagenten entspricht.

```json theme={null}
{
  "permissions": {
    "deny": ["Agent(Explore)", "Agent(my-custom-agent)"]
  }
}
```

Dies funktioniert für integrierte und benutzerdefinierte Subagenten. Sie können auch das `--disallowedTools`-CLI-Flag verwenden:

```bash theme={null}
claude --disallowedTools "Agent(Explore)"
```

Siehe [Berechtigungsdokumentation](/docs/de/permissions#tool-specific-permission-rules) für weitere Details zu Berechtigungsregeln.

<h3 id="define-hooks-for-subagents">
  Definieren Sie Hooks für Subagenten
</h3>

Subagenten können [Hooks](/docs/de/hooks) definieren, die während des Lebenszyklus des Subagenten ausgeführt werden. Es gibt zwei Möglichkeiten, Hooks zu konfigurieren:

* **Im Frontmatter des Subagenten**: Definieren Sie Hooks, die nur ausgeführt werden, während dieser Subagent aktiv ist
* **In `settings.json`**: Definieren Sie Hooks, die in der Hauptsitzung ausgeführt werden, wenn Subagenten starten oder stoppen

<h4 id="hooks-in-subagent-frontmatter">
  Hooks im Subagenten-Frontmatter
</h4>

Definieren Sie Hooks direkt in der Markdown-Datei des Subagenten. Diese Hooks werden nur ausgeführt, während dieser spezifische Subagent aktiv ist, und werden bereinigt, wenn er endet.

<Note>
  Frontmatter-Hooks werden ausgelöst, wenn der Agent als Subagent durch das Agent-Werkzeug oder eine @-Erwähnung gespawnt wird, und wenn der Agent als Hauptsitzung über [`--agent`](#invoke-subagents-explicitly) oder die `agent`-Einstellung läuft. Im Hauptsitzungs-Fall werden sie zusammen mit allen Hooks ausgeführt, die in [`settings.json`](/docs/de/hooks) definiert sind.
</Note>

Alle [Hook-Ereignisse](/docs/de/hooks#hook-events) werden unterstützt. Die häufigsten Ereignisse für Subagenten sind:

| Ereignis      | Matcher-Eingabe | Wann es ausgelöst wird                                                    |
| :------------ | :-------------- | :------------------------------------------------------------------------ |
| `PreToolUse`  | Werkzeugname    | Bevor der Subagent ein Werkzeug verwendet                                 |
| `PostToolUse` | Werkzeugname    | Nachdem der Subagent ein Werkzeug verwendet hat                           |
| `Stop`        | (keine)         | Wenn der Subagent endet (wird zur Laufzeit in `SubagentStop` konvertiert) |

Dieses Beispiel validiert Bash-Befehle mit dem `PreToolUse`-Hook und führt einen Linter nach Dateibearbeitungen mit `PostToolUse` aus:

```yaml theme={null}
---
name: code-reviewer
description: Review code changes with automatic linting
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-command.sh $TOOL_INPUT"
  PostToolUse:
    - matcher: "Edit|Write"
      hooks:
        - type: command
          command: "./scripts/run-linter.sh"
---
```

Wenn der Agent als Subagent aufgerufen wird, werden `Stop`-Hooks im Frontmatter automatisch in `SubagentStop`-Ereignisse konvertiert.

<h4 id="project-level-hooks-for-subagent-events">
  Hooks auf Projektebene für Subagenten-Ereignisse
</h4>

Konfigurieren Sie Hooks in `settings.json`, die auf Subagenten-Lebenszyklus-Ereignisse in der Hauptsitzung reagieren.

| Ereignis        | Matcher-Eingabe | Wann es ausgelöst wird                       |
| :-------------- | :-------------- | :------------------------------------------- |
| `SubagentStart` | Agent-Typname   | Wenn ein Subagent mit der Ausführung beginnt |
| `SubagentStop`  | Agent-Typname   | Wenn ein Subagent abgeschlossen ist          |

Beide Ereignisse unterstützen Matcher, um bestimmte Agent-Typen nach Name zu adressieren. Der Matcher-Wert ist der `name` des Frontmatters des Agenten für Projekt- und Benutzer-Subagenten oder der Plugin-Umfang-Identifier wie `my-plugin:db-agent` für [Plugin-Subagenten](/docs/de/plugins). Ein Umfang-Name enthält einen Doppelpunkt, daher wird er als [unverankerte reguläre Ausdrücke](/docs/de/hooks#matcher-patterns) ausgewertet; verankern Sie ihn mit `^` und `$`, wie in `^my-plugin:db-agent$`, um nur diesen Agent zu treffen.

Dieses Beispiel führt ein Setup-Skript nur aus, wenn der `db-agent`-Subagent startet, und ein Cleanup-Skript, wenn ein beliebiger Subagent stoppt:

```json theme={null}
{
  "hooks": {
    "SubagentStart": [
      {
        "matcher": "db-agent",
        "hooks": [
          { "type": "command", "command": "./scripts/setup-db-connection.sh" }
        ]
      }
    ],
    "SubagentStop": [
      {
        "hooks": [
          { "type": "command", "command": "./scripts/cleanup-db-connection.sh" }
        ]
      }
    ]
  }
}
```

Ein Matcher mit Bindestrichen wie `db-agent` passt genau auf Claude Code v2.1.195 oder später. In früheren Versionen wird er als unverankerte reguläre Ausdrücke ausgewertet und wird auch für jeden Agent-Typ ausgelöst, der ihn enthält, wie `prod-db-agent`; verankern Sie ihn als `^db-agent$` in diesen Versionen.

Siehe [Hooks](/docs/de/hooks) für das vollständige Hook-Konfigurationsformat.

<h2 id="work-with-subagents">
  Arbeiten Sie mit Subagenten
</h2>

<h3 id="understand-automatic-delegation">
  Verstehen Sie automatische Delegation
</h3>

Claude delegiert automatisch Aufgaben basierend auf der Aufgabenbeschreibung in Ihrer Anfrage, dem `description`-Feld in Subagenten-Konfigurationen und dem aktuellen Kontext. Um proaktive Delegation zu fördern, fügen Sie Phrasen wie "use proactively" in das `description`-Feld Ihres Subagenten ein.

<h3 id="invoke-subagents-explicitly">
  Rufen Sie Subagenten explizit auf
</h3>

Wenn automatische Delegation nicht ausreicht, können Sie einen Subagenten selbst anfordern. Drei Muster eskalieren von einem einmaligen Vorschlag zu einem sitzungsweiten Standard:

* **Natürliche Sprache**: Nennen Sie den Subagenten in Ihrem Prompt; Claude entscheidet, ob delegiert werden soll
* **@-Erwähnung**: Garantiert, dass der Subagent für eine Aufgabe ausgeführt wird
* **Sitzungsweit**: Die gesamte Sitzung verwendet den Systemprompt, die Werkzeugbeschränkungen und das Modell dieses Subagenten über das `--agent`-Flag oder die `agent`-Einstellung

Für natürliche Sprache gibt es keine spezielle Syntax. Nennen Sie den Subagenten und Claude delegiert normalerweise:

```text wrap theme={null}
Use the test-runner subagent to fix failing tests
Have the code-reviewer subagent look at my recent changes
```

**@-Erwähnen Sie den Subagenten.** Geben Sie `@` ein und wählen Sie den Subagenten aus der Typeahead-Liste, genauso wie Sie Dateien @-erwähnen. Dies stellt sicher, dass dieser spezifische Subagent ausgeführt wird, anstatt die Wahl Claude zu überlassen:

```text wrap theme={null}
@"code-reviewer (agent)" look at the auth changes
```

Ihre vollständige Nachricht geht immer noch an Claude, das den Task-Prompt des Subagenten basierend auf Ihrer Anfrage schreibt. Die @-Erwähnung steuert, welcher Subagent Claude aufruft, nicht welchen Prompt er erhält.

Subagenten, die von einem aktivierten [Plugin](/docs/de/plugins) bereitgestellt werden, erscheinen in der Typeahead-Liste unter ihrem scoped Namen, z. B. `my-plugin:code-reviewer` oder `my-plugin:review:security`, wenn das Plugin [Agenten in Unterordnern organisiert](#choose-the-subagent-scope). Benannte Hintergrund-Subagenten, die derzeit in der Sitzung ausgeführt werden, erscheinen auch in der Typeahead-Liste und zeigen ihren Status neben dem Namen an.

Sie können die Erwähnung auch manuell eingeben, ohne den Picker zu verwenden: `@agent-<name>` für lokale Subagenten, oder `@agent-` gefolgt vom scoped Namen für Plugin-Subagenten, z. B. `@agent-my-plugin:code-reviewer`.

**Führen Sie die gesamte Sitzung als Subagent aus.** Übergeben Sie [`--agent <name>`](/docs/de/cli-reference), um eine Sitzung zu starten, in der der Hauptthread selbst den Systemprompt, die Werkzeugbeschränkungen und das Modell dieses Subagenten annimmt:

```bash theme={null}
claude --agent code-reviewer
```

Der Systemprompt des Subagenten ersetzt den Standard-Claude Code-Systemprompt vollständig, genauso wie [`--system-prompt`](/docs/de/cli-reference) es tut. `CLAUDE.md`-Dateien und Projekt-Memory werden weiterhin durch den normalen Nachrichtenfluss geladen. Der Agent-Name erscheint als `@<name>` in der Startup-Kopfzeile, damit Sie bestätigen können, dass er aktiv ist.

Dies funktioniert mit integrierten und benutzerdefinierten Subagenten, und die Wahl bleibt bestehen, wenn Sie die Sitzung fortsetzen.

Für einen von einem Plugin bereitgestellten Subagenten können Sie einfach den Agent-Namen übergeben und Claude Code findet ihn:

```bash theme={null}
claude --agent security-reviewer
```

Wenn mehrere Plugins Agenten mit demselben Namen bereitstellen, übergeben Sie den scoped Namen zur Disambiguierung:

```bash theme={null}
claude --agent my-plugin:security-reviewer
```

Wenn das Plugin den Agenten in einem Unterordner seines `agents/`-Verzeichnisses platziert, fügen Sie den Unterordner in den scoped Namen ein, z. B. `claude --agent my-plugin:review:security`.

Um es zum Standard für jede Sitzung in einem Projekt zu machen, setzen Sie `agent` in `.claude/settings.json`:

```json theme={null}
{
  "agent": "code-reviewer"
}
```

Das CLI-Flag überschreibt die Einstellung, wenn beide vorhanden sind.

<h3 id="run-subagents-in-foreground-or-background">
  Führen Sie Subagenten im Vordergrund oder Hintergrund aus
</h3>

Subagenten können im Vordergrund oder im Hintergrund ausgeführt werden:

* **Vordergrund-Subagenten** blockieren die Hauptkonversation bis zur Fertigstellung. Berechtigungsaufforderungen werden an Sie weitergeleitet, wenn sie auftreten.
* **Hintergrund-Subagenten** laufen gleichzeitig, während Sie weiterarbeiten. Ab v2.1.186 wird die Aufforderung in Ihrer Hauptsitzung angezeigt, wenn ein Hintergrund-Subagent einen Werkzeugaufruf erreicht, der Berechtigung benötigt, und nennt den Subagenten, der fragt. Genehmigen Sie, um den Subagenten fortzusetzen, oder drücken Sie Esc, um diesen einen Werkzeugaufruf zu verweigern, ohne den Subagenten zu stoppen. Vor v2.1.186 lehnten Hintergrund-Subagenten automatisch jeden Werkzeugaufruf ab, der eine Aufforderung ausgelöst hätte.

Ab v2.1.198 werden Subagenten standardmäßig im Hintergrund ausgeführt. Claude führt einen Subagenten im Vordergrund aus, wenn er das Ergebnis benötigt, bevor er fortfährt. Die Standardeinstellung ändert, wo ein Subagent ausgeführt wird, nicht was ihm erlaubt ist: Hintergrund-Subagenten zeigen immer noch jede Berechtigungsaufforderung in Ihrer Hauptsitzung an. Vor v2.1.198 wählte Claude zwischen Vordergrund und Hintergrund basierend auf der Aufgabe.

Sie können auch selbst steuern:

* Claude bitten, eine Aufgabe im Hintergrund oder im Vordergrund auszuführen
* **Ctrl+B** drücken, um eine laufende Aufgabe in den Hintergrund zu verschieben

Ein Hintergrund-Subagent, der abgeschlossen ist, bleibt in [`/tasks`](/docs/de/commands) aufgelistet, als erledigt markiert und unter laufenden Arbeiten sortiert, bis die Sitzung ihre Aufgabenliste bereinigt. Seine Detailansicht bleibt offen, wenn der Subagent fertig ist. Subagenten, die fehlschlagen oder die Sie stoppen, verlassen die Liste. Vor v2.1.208 verließ ein abgeschlossener Subagent die Liste in dem Moment, in dem er fertig war, und seine Detailansicht schloss sich.

Um alle Hintergrund-Aufgaben-Funktionalität zu deaktivieren, setzen Sie die Umgebungsvariable `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` auf `1`. Siehe [Umgebungsvariablen](/docs/de/env-vars).

Wenn [`CLAUDE_CODE_FORK_SUBAGENT`](#fork-the-current-conversation) auf `1` gesetzt ist, wird jeder Subagenten-Spawn im Hintergrund ausgeführt, und das Frontmatter-Feld `background` hat keine Auswirkung, da der Fork-Modus den Parameter `run_in_background` aus dem `Agent`-Werkzeug entfernt. `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` hat Vorrang vor dem Fork-Modus und hält Subagenten-Spawns im Vordergrund.

<h3 id="api-errors-in-subagents">
  API-Fehler in Subagenten
</h3>

Ab v2.1.199 meldet ein Subagent, dessen Ausführung mit einem API-Fehler endet, z. B. ein Nutzungslimit oder ein wiederholter Serverfehler, diesen Fehler an Claude zurück, anstatt den Fehlertext so zurückzugeben, als wären es die Erkenntnisse des Subagenten. Was Claude erhält, hängt davon ab, wo der Subagent ausgeführt wurde:

* **Vordergrund**: Wenn ein Ratenlimit, eine Überlastung oder ein Serverfehler einen Subagenten unterbricht, der bereits Ausgaben erzeugt hat, gibt das Agent-Werkzeug diese Teilausgabe mit einem Hinweis zurück, dass der Subagent unterbrochen wurde und seine Aufgabe nicht abgeschlossen hat. Ein Subagent, der nichts erzeugt hat, oder dessen einzige Ausgabe Werkzeugaufrufe waren, schlägt mit [`Agent terminated early due to an API error`](/docs/de/errors#agent-terminated-early-due-to-an-api-error) fehl, gefolgt von der Fehlerdetail. In v2.1.199 gab ein Ratenlimit, eine Überlastung oder ein Serverfehler, der die Form nur mit Werkzeugaufrufen unterbrach, ein leeres Teilergebnis zurück, das nur den Unterbrechungshinweis enthielt.
* **Hintergrund**: Der Subagent wird als fehlgeschlagen markiert, und die Nachricht, die Claude erhält, wenn er endet, nennt den API-Fehler und enthält die letzte Ausgabe des Subagenten, sodass Teilarbeit nicht verloren geht.

Sobald der zugrunde liegende API-Fehler behoben ist, bitten Sie Claude, die Aufgabe zu wiederholen oder [setzen Sie den Subagenten fort](#resume-subagents).

<h3 id="common-patterns">
  Häufige Muster
</h3>

<h4 id="isolate-high-volume-operations">
  Isolieren Sie hochvolumige Operationen
</h4>

Eine der effektivsten Verwendungen für Subagenten ist die Isolierung von Operationen, die große Mengen an Ausgaben erzeugen. Das Ausführen von Tests, das Abrufen von Dokumentation oder die Verarbeitung von Protokolldateien kann erheblichen Kontext verbrauchen. Durch die Delegierung an einen Subagenten bleibt die ausführliche Ausgabe im Kontext des Subagenten, während nur die relevante Zusammenfassung zu Ihrer Hauptkonversation zurückkehrt.

```text wrap theme={null}
Use a subagent to run the test suite and report only the failing tests with their error messages
```

<h4 id="run-parallel-research">
  Führen Sie parallele Recherche durch
</h4>

Für unabhängige Untersuchungen spawnen Sie mehrere Subagenten, um gleichzeitig zu arbeiten:

```text wrap theme={null}
Research the authentication, database, and API modules in parallel using separate subagents
```

Jeder Subagent erkundet seinen Bereich unabhängig, dann synthetisiert Claude die Erkenntnisse. Dies funktioniert am besten, wenn die Recherchepfade nicht voneinander abhängen.

<Warning>
  Wenn Subagenten abgeschlossen sind, kehren ihre Ergebnisse zu Ihrer Hauptkonversation zurück. Das Ausführen vieler Subagenten, die jeweils detaillierte Ergebnisse zurückgeben, kann erheblichen Kontext verbrauchen.
</Warning>

Für Aufgaben, die anhaltende Parallelität benötigen oder Ihr Kontextfenster überschreiten, geben [Agent-Teams](/docs/de/agent-teams) jedem Worker seinen eigenen unabhängigen Kontext.

<h4 id="chain-subagents">
  Verketten Sie Subagenten
</h4>

Für mehrstufige Workflows bitten Sie Claude, Subagenten nacheinander zu verwenden. Jeder Subagent vervollständigt seine Aufgabe und gibt Ergebnisse an Claude zurück, das dann relevanten Kontext an den nächsten Subagenten übergibt.

```text wrap theme={null}
Use the code-reviewer subagent to find performance issues, then use the optimizer subagent to fix them
```

<h3 id="choose-between-subagents-and-main-conversation">
  Wählen Sie zwischen Subagenten und Hauptkonversation
</h3>

Verwenden Sie die **Hauptkonversation**, wenn:

* Die Aufgabe häufiges Hin und Her oder iterative Verfeinerung benötigt
* Mehrere Phasen teilen erheblichen Kontext, z. B. Planung, Implementierung und Testen
* Sie eine schnelle, gezielte Änderung vornehmen
* Latenz ist wichtig. Subagenten starten von vorne und benötigen möglicherweise Zeit, um Kontext zu sammeln

Verwenden Sie **Subagenten**, wenn:

* Die Aufgabe ausführliche Ausgaben erzeugt, die Sie nicht in Ihrem Hauptkontext benötigen
* Sie spezifische Werkzeugbeschränkungen oder Berechtigungen durchsetzen möchten
* Die Arbeit in sich geschlossen ist und eine Zusammenfassung zurückgeben kann

Erwägen Sie stattdessen [Skills](/docs/de/skills), wenn Sie wiederverwendbare Prompts oder Workflows möchten, die im Kontext der Hauptkonversation ausgeführt werden, anstatt in isoliertem Subagenten-Kontext.

Für eine schnelle Frage zu etwas, das bereits in Ihrer Konversation ist, verwenden Sie stattdessen [`/btw`](/docs/de/interactive-mode#side-questions-with-%2Fbtw). Es sieht Ihren vollständigen Kontext, hat aber keinen Werkzeugzugriff, und die Antwort wird verworfen, anstatt zur Historie hinzugefügt zu werden.

<h3 id="spawn-nested-subagents">
  Spawnen Sie verschachtelte Subagenten
</h3>

Ab Claude Code v2.1.172 kann ein Subagent seine eigenen Subagenten spawnen. Verwenden Sie dies, wenn sich eine delegierte Aufgabe selbst in parallele Unteraufgaben aufteilt, z. B. ein Reviewer-Subagent, der einen Verifier pro Befund versendet, sodass die Zwischenausgabe niemals Ihre Hauptkonversation erreicht. Nur die Zusammenfassung des Top-Level-Subagenten kehrt zu Ihnen zurück.

Ein verschachtelter Subagent wird genauso konfiguriert wie ein Top-Level-Subagent und wird aus denselben [Scopes](#choose-the-subagent-scope) aufgelöst.

Das Subagenten-Panel unter der Eingabeaufforderung zeigt den vollständigen Baum: Jede Zeile zeigt eine `(+N)`-Anzahl von Nachkommen an, und ab v2.1.193 zeigt das Öffnen einer Zeile die Geschwister und direkten Kinder dieses Subagenten mit einem Pfad zurück zu `main`.

Die Tiefe wird als die Anzahl der Subagenten-Ebenen unter der Hauptkonversation gezählt, unabhängig davon, ob jede Ebene im [Vordergrund oder Hintergrund](#run-subagents-in-foreground-or-background) ausgeführt wird. Ein Subagent in Tiefe fünf erhält das Agent-Werkzeug nicht und kann nicht weiter spawnen. Das Limit ist fest und nicht konfigurierbar.

Ab Claude Code v2.1.187 ist die Tiefe eines Hintergrund-Subagenten festgelegt, wenn er zuerst spawnt wird, und das [Fortsetzen](#resume-subagents) später ändert diese Tiefe nicht. Wenn beispielsweise Ihre Hauptkonversation Subagent A spawnt und A einen Hintergrund-Subagenten B in Tiefe zwei spawnt, ist B immer noch in Tiefe zwei, wenn Sie ihn später direkt aus der Hauptkonversation fortsetzen. Das Fortsetzen eines Subagenten aus einem flacheren Kontext erlaubt ihm nicht, zusätzliche Ebenen zu spawnen, die das Tiefenlimit bereits verhindert hat.

Um zu verhindern, dass ein bestimmter Subagent andere spawnt, lassen Sie `Agent` aus seiner [`tools`](#available-tools)-Liste weg oder fügen Sie es zu `disallowedTools` hinzu.

Ein [Fork](#fork-the-current-conversation) kann immer noch keinen anderen Fork spawnen. Er kann andere Subagenten-Typen spawnen, und diese zählen zum Tiefenlimit.

<h3 id="manage-subagent-context">
  Verwalten Sie den Subagenten-Kontext
</h3>

<h4 id="what-loads-at-startup">
  Was wird beim Start geladen
</h4>

Jeder Subagent startet mit einem frischen, isolierten Kontextfenster. Er sieht nicht Ihre Konversationshistorie, die Skills, die Sie bereits aufgerufen haben, oder die Dateien, die Claude bereits gelesen hat. Claude verfasst eine Delegierungsnachricht, die die Aufgabe zusammenfasst, und der Subagent arbeitet von dort aus. Die Ausnahme ist ein [Fork](#fork-the-current-conversation), der die übergeordnete Konversation erbt, anstatt von vorne zu beginnen.

Der anfängliche Kontext eines Nicht-Fork-Subagenten enthält:

* **Systemprompt**: Der eigene Prompt des Agenten plus Umgebungsdetails, die Claude Code anhängt, nicht der vollständige Claude Code-Systemprompt. Benutzerdefinierte Subagenten definieren ihren in der [Markdown-Datei](#write-subagent-files) oder im `prompt`-Feld. Integrierte Agenten haben vordefinierte Prompts.
* **Task-Nachricht**: Der Delegierungsprompt, den Claude schreibt, wenn er die Arbeit übergibt.
* **CLAUDE.md und Memory**: Jede Ebene der [Memory-Hierarchie](/docs/de/memory#how-claude-md-files-load), die die Hauptkonversation lädt, einschließlich `~/.claude/CLAUDE.md`, Projektregeln, `CLAUDE.local.md` und verwaltete Richtliniendateien. Die integrierten Explore- und Plan-Agenten überspringen dies.
* **Git-Status**: Ein Snapshot, der zu Beginn der übergeordneten Sitzung erstellt wurde. Fehlt, wenn das Arbeitsverzeichnis kein Git-Repository ist oder wenn [`includeGitInstructions`](/docs/de/settings#available-settings) `false` ist. Explore und Plan überspringen es unabhängig davon.
* **Vorgeladene Skills**: Vollständiger Inhalt aller Skills, die im [`skills`-Feld](#preload-skills-into-subagents) des Agenten benannt sind. Integrierte Agenten laden Skills nicht vor.
* **Geschwister-Verzeichnis**: Eine Systemerinnerung, die `main` und jeden anderen benannten Agenten in der Sitzung auflistet, jeweils ein gültiger `to`-Wert für [`SendMessage`](#resume-subagents). Erfordert Claude Code v2.1.206 oder später. Das Verzeichnis erscheint nur, wenn die Tools des Subagenten `SendMessage` enthalten und mindestens ein anderer Agent einen Namen hat, ob Claude ihn beim Spawnen benannt hat oder er als [Agent-Team](/docs/de/agent-teams)-Teamkollege läuft. Es ist ein Snapshot, der erstellt wird, wenn der Subagent startet, daher erscheinen später benannte Agenten nicht.

Explore und Plan sind die einzigen Subagenten, die CLAUDE.md und Git-Status auslassen. Es gibt kein Frontmatter-Feld oder eine Pro-Agent-Einstellung, um zu ändern, welche Agenten sie überspringen.

Die Hauptkonversation liest Explore- und Plan-Ergebnisse mit vollständigem CLAUDE.md-Kontext, daher müssen die meisten Regeln den Subagenten selbst nicht erreichen. Wenn eine Regel dies muss, z. B. "ignore the `vendor/` directory", wiederholen Sie sie in dem Prompt, den Sie Claude geben, wenn Sie delegieren.

<h4 id="resume-subagents">
  Setzen Sie Subagenten fort
</h4>

Jede Subagenten-Invokation erstellt eine neue Instanz mit frischem Kontext. Um die Arbeit eines vorhandenen Subagenten fortzusetzen, anstatt von vorne zu beginnen, bitten Sie Claude, ihn fortzusetzen.

Fortgesetzte Subagenten behalten ihre vollständige Konversationshistorie, einschließlich aller vorherigen Werkzeugaufrufe, Ergebnisse und Überlegungen. Der Subagent setzt genau dort an, wo er gestoppt hat, anstatt von vorne zu beginnen.

Wenn ein Subagent abgeschlossen ist, erhält Claude seine Agent-ID. Die integrierten Explore- und Plan-Agenten sind einmalig und geben keine Agent-ID zurück, daher können sie nicht fortgesetzt werden; verwenden Sie `general-purpose` oder einen benutzerdefinierten Subagenten, wenn Sie die Arbeit fortsetzen müssen.

Claude verwendet das `SendMessage`-Werkzeug mit der Agent-ID oder dem Namen des Agenten als `to`-Feld, um ihn fortzusetzen. `SendMessage` erfordert nicht, dass [Agent-Teams](/docs/de/agent-teams) aktiviert sind; nur strukturierte Team-Protokoll-Nachrichten wie `shutdown_request` und `plan_approval_response` erfordern dies.

Um einen Subagenten fortzusetzen, bitten Sie Claude, die vorherige Arbeit fortzusetzen:

```text wrap theme={null}
Use the code-reviewer subagent to review the authentication module
[Agent completes]

Continue that code review and now analyze the authorization logic
[Claude resumes the subagent with full context from previous conversation]
```

Ein gestoppter Subagent, der eine `SendMessage` erhält, wird automatisch im Hintergrund fortgesetzt, ohne dass eine neue `Agent`-Invokation erforderlich ist. Das Gleiche gilt für einen Subagenten, den Claude mit dem `TaskStop`-Werkzeug gestoppt hat.

Ab v2.1.191 wird ein Subagent, den Sie selbst gestoppt haben, mit `x` in `/tasks` oder einer SDK-`stop_task`-Anfrage, nicht automatisch fortgesetzt. Der `SendMessage`-Aufruf gibt eine Ablehnung zurück, die Claude mitteilt, dass der Agent abgebrochen wurde. Geben Sie in das Transkript dieses Subagenten im Subagenten-Panel ein, um ihn selbst fortzusetzen, was den Stop löscht, damit spätere `SendMessage`-Aufrufe ihn wieder automatisch fortsetzen können.

Das Fortsetzen startet einen neuen Lauf des Agenten unter derselben ID, sodass ein Subagent, der bereits fehlgeschlagen oder abgeschlossen war, in der Aufgabenliste und in den Task-Events des Agent SDK wieder als laufend angezeigt wird. Vor v2.1.205 zeigte er seinen früheren fehlgeschlagenen oder abgeschlossenen Status, während der fortgesetzte Lauf funktionierte.

Ab v2.1.199 überprüft `SendMessage`, dass ein Name immer noch auf denselben Agenten verweist, den er früher in der Konversation erreicht hat. Wenn ein neuerer Agent den Namen übernommen hat, z. B. ein neu gestarteter Hintergrund-Agent, der ihn wiederverwendet hat, weigert sich Claude Code, die Nachricht zu senden, anstatt sie an den falschen Agenten zu liefern, und der Fehler meldet, welchen Agenten der Name jetzt erreicht, damit Claude neu ausrichten kann. Um den früheren Agenten zu erreichen, während er noch läuft, adressiert Claude ihn nach der Agent-ID aus seinem Spawn-Ergebnis. Die Überprüfung ist auf die aktuelle Konversation beschränkt und wird bei `/clear` zurückgesetzt.

Ab v2.1.198 behandelt ein Subagent Nachrichten vom Agenten, der ihn gestartet hat, als normale Aufgabenrichtung, einschließlich Kurskorrektionen während der Aufgabe, und handelt danach innerhalb seiner eigenen Berechtigungseinstellungen. Zwei Limits gelten immer noch unabhängig davon, wer die Nachricht gesendet hat: Keine Nachricht von einem Agenten zählt als Ihre Genehmigung für eine ausstehende Berechtigungsaufforderung, und keine Agent-Nachricht kann die Berechtigungseinstellungen, `CLAUDE.md` oder Konfiguration eines Subagenten ändern. Nur das Berechtigungssystem oder Ihre eigenen Nachrichten können Genehmigung gewähren.

Sie können auch Claude nach der Agent-ID fragen, wenn Sie sie explizit referenzieren möchten, oder IDs in den Transkriptdateien unter `~/.claude/projects/{project}/{sessionId}/subagents/` finden. Jedes Transkript wird als `agent-{agentId}.jsonl` gespeichert.

Subagenten-Transkripte bleiben unabhängig von der Hauptkonversation bestehen:

* **Hauptkonversations-Komprimierung**: Wenn die Hauptkonversation komprimiert wird, sind Subagenten-Transkripte nicht betroffen. Sie werden in separaten Dateien gespeichert.
* **Sitzungs-Persistenz**: Subagenten-Transkripte bleiben innerhalb ihrer Sitzung bestehen. Sie können [einen Subagenten fortsetzen](#resume-subagents), nachdem Sie Claude Code neu gestartet haben, indem Sie dieselbe Sitzung fortsetzen.
* **Automatische Bereinigung**: Transkripte werden basierend auf der `cleanupPeriodDays`-Einstellung bereinigt, die standardmäßig 30 Tage beträgt.

<h4 id="auto-compaction">
  Auto-Komprimierung
</h4>

Subagenten unterstützen automatische Komprimierung mit derselben Logik wie die Hauptkonversation. Die Komprimierung wird unter denselben Bedingungen ausgelöst, und `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` gilt auch für Subagenten. Siehe [Umgebungsvariablen](/docs/de/env-vars) für den Zeitpunkt, zu dem die Überschreibung wirksam wird.

Komprimierungsereignisse werden in Subagenten-Transkriptdateien protokolliert:

```json theme={null}
{
  "type": "system",
  "subtype": "compact_boundary",
  "compactMetadata": {
    "trigger": "auto",
    "preTokens": 167189
  }
}
```

Der `preTokens`-Wert zeigt, wie viele Token vor der Komprimierung verwendet wurden.

<h2 id="fork-the-current-conversation">
  Gegabelte Konversation
</h2>

<Note>
  Gegabelte Subagenten erfordern Claude Code v2.1.117 oder später. Ab v2.1.161 ist der `/fork`-Befehl standardmäßig aktiviert; in früheren Versionen ist die Umgebungsvariable [`CLAUDE_CODE_FORK_SUBAGENT`](/docs/de/env-vars) auf `1` erforderlich. Das Spawning von Forks durch Claude selbst ist experimentell und kann sich in zukünftigen Versionen ändern. Diese Funktion kann auch in interaktiven Sitzungen als Teil eines gestaffelten Rollouts aktiviert werden.
</Note>

Ein Fork ist ein Subagent, der die gesamte bisherige Konversation erbt, anstatt von vorne zu beginnen. Dies lässt die Eingabe-Isolierung fallen, die Subagenten ansonsten bieten: Ein Fork sieht denselben Systemprompt, dieselben Werkzeuge, dasselbe Modell und die Nachrichtenhistorie wie die Hauptsitzung, sodass Sie ihm eine Nebenaufgabe übergeben können, ohne die Situation erneut zu erklären. Die eigenen Werkzeugaufrufe des Forks bleiben weiterhin aus Ihrer Konversation heraus und nur sein endgültiges Ergebnis kommt zurück, sodass Ihr Hauptkontextfenster sauber bleibt. Verwenden Sie einen Fork, wenn ein benannter Subagent zu viel Hintergrund benötigen würde, um nützlich zu sein, oder wenn Sie mehrere Ansätze parallel vom gleichen Ausgangspunkt aus versuchen möchten.

Um den Fork-Modus unabhängig vom gestaffelten Rollout zu steuern, setzen Sie [`CLAUDE_CODE_FORK_SUBAGENT`](/docs/de/env-vars) auf `1`, um ihn explizit zu aktivieren, oder auf `0`, um ihn zu deaktivieren. Die Variable wird im interaktiven Modus und über das SDK oder `claude -p` berücksichtigt.

Das Aktivieren des Fork-Modus ändert Claude Code auf zwei Arten:

* Claude kann einen Fork spawnen, indem es den `fork`-Subagenten-Typ explizit anfordert. Spawns ohne einen Subagenten-Typ verwenden weiterhin den [allgemeinen](#built-in-subagents)-Subagenten, und benannte Subagenten wie Explore werden weiterhin wie zuvor gespawnt.
* Jeder Subagenten-Spawn wird im [Hintergrund](#run-subagents-in-foreground-or-background) ausgeführt, unabhängig davon, ob es sich um einen Fork oder einen benannten Subagenten handelt. Setzen Sie `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` auf `1`, um Spawns synchron zu halten.

Sie können einen Fork selbst mit `/fork` gefolgt von einer Direktive starten, unabhängig davon, ob die Variable gesetzt ist oder nicht. Claude Code benennt den Fork aus den ersten Worten der Direktive. Das folgende Beispiel gabelt die Konversation, um Testfälle zu entwerfen, während Sie mit der Implementierung in der Hauptsitzung fortfahren:

```text wrap theme={null}
/fork draft unit tests for the parser changes so far
```

Der Fork erscheint in einem Panel unter Ihrer Eingabeaufforderung und läuft im Hintergrund, während Sie weiterarbeiten. Wenn er fertig ist, kommt sein Ergebnis als Nachricht in Ihrer Hauptkonversation an. Der nächste Abschnitt behandelt die Panel-Steuerelemente zum Beobachten und Lenken von Forks während ihrer Ausführung.

<h3 id="observe-and-steer-running-forks">
  Beobachten und lenken Sie laufende Forks
</h3>

Laufende Forks erscheinen in einem Panel unter der Eingabeaufforderung, mit einer Zeile für die Hauptsitzung und einer für jeden Fork. Verwenden Sie diese Tasten, um mit dem Panel zu interagieren:

| Taste     | Aktion                                                                          |
| :-------- | :------------------------------------------------------------------------------ |
| `↑` / `↓` | Zwischen Zeilen wechseln                                                        |
| `Enter`   | Öffnen Sie das Transkript des ausgewählten Forks und senden Sie ihm Folgefragen |
| `x`       | Schließen Sie einen fertigen Fork oder stoppen Sie einen laufenden              |
| `Esc`     | Fokus zurück zur Eingabeaufforderung                                            |

Mit einem geöffneten Transkript eines Forks oder Subagenten gehen Folgefragen und [Skills](/docs/de/skills) an diesen Agenten, aber integrierte Befehle werden weiterhin in Ihrer Hauptkonversation ausgeführt. Ab v2.1.199 zeigt die Eingabe von `/model` oder `/fast` in dieser Ansicht einen Hinweis an, dass dies das Modell oder den Schnellmodus der Hauptkonversation ändert, nicht des angezeigten Agenten, anstatt es stillschweigend auszuführen.

<h3 id="how-forks-differ-from-named-subagents">
  Wie sich Forks von benannten Subagenten unterscheiden
</h3>

Ein Fork erbt alles, was die Hauptsitzung zum Zeitpunkt des Spawnens hat. Ein benannter Subagent startet von seiner eigenen Definition.

|                            | Fork                                        | Benannter Subagent                                                                                                          |
| :------------------------- | :------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------- |
| Kontext                    | Vollständige Konversationshistorie          | Frischer Kontext mit dem Prompt, den Sie übergeben                                                                          |
| Systemprompt und Werkzeuge | Gleich wie Hauptsitzung                     | Aus der [Definitionsdatei](#write-subagent-files) des Subagenten                                                            |
| Modell                     | Gleich wie Hauptsitzung                     | Aus dem `model`-Feld des Subagenten                                                                                         |
| Berechtigungen             | Aufforderungen erscheinen in Ihrem Terminal | [Aufforderungen erscheinen in Ihrer Hauptsitzung](#run-subagents-in-foreground-or-background) bei Ausführung im Hintergrund |
| Prompt-Cache               | Mit Hauptsitzung geteilt                    | Separater Cache                                                                                                             |

Da der Systemprompt und die Werkzeugdefinitionen eines Forks identisch mit dem übergeordneten Element sind, wird seine erste Anfrage den [Prompt-Cache](/docs/de/prompt-caching#subagents-and-the-cache) des übergeordneten Elements wiederverwenden. Dies macht das Forking billiger als das Spawnen eines frischen Subagenten für Aufgaben, die denselben Kontext benötigen.

Wenn Claude einen Fork durch das Agent-Werkzeug spawnt, kann es `isolation: "worktree"` übergeben, sodass die Dateibearbeitungen des Forks in einen separaten Git-Worktree geschrieben werden, anstatt in Ihren Checkout.

<h3 id="limitations">
  Einschränkungen
</h3>

Das Setzen von `CLAUDE_CODE_FORK_SUBAGENT=1` aktiviert den Fork-Modus in interaktiven Sitzungen, im [nicht-interaktiven Modus](/docs/de/headless) und im Agent SDK; das Setzen auf `0` deaktiviert den Fork-Modus überall, einschließlich jedes serverseitigen Rollouts. Ein Fork kann keine weiteren Forks spawnen.

<h2 id="example-subagents">
  Beispiel-Subagenten
</h2>

Diese Beispiele demonstrieren effektive Muster für die Erstellung von Subagenten. Verwenden Sie sie als Ausgangspunkte oder generieren Sie eine angepasste Version mit Claude.

<Tip>
  **Best Practices:**

  * **Entwerfen Sie fokussierte Subagenten:** Jeder Subagent sollte bei einer spezifischen Aufgabe hervorragend sein
  * **Schreiben Sie detaillierte Beschreibungen:** Claude verwendet die Beschreibung, um zu entscheiden, wann delegiert werden soll
  * **Begrenzen Sie den Werkzeugzugriff:** Gewähren Sie nur notwendige Berechtigungen für Sicherheit und Fokus
  * **Checken Sie in die Versionskontrolle ein:** Teilen Sie Projekt-Subagenten mit Ihrem Team
</Tip>

<h3 id="code-reviewer">
  Code-Reviewer
</h3>

Ein schreibgeschützter Subagent, der Code überprüft, ohne ihn zu ändern. Dieses Beispiel zeigt, wie man einen fokussierten Subagenten mit begrenztem Werkzeugzugriff entwirft, der Edit und Write ausschließt, und einen detaillierten Prompt, der genau angibt, worauf zu achten ist und wie die Ausgabe formatiert wird.

```markdown theme={null}
---
name: code-reviewer
description: Expert code review specialist. Proactively reviews code for quality, security, and maintainability. Use immediately after writing or modifying code.
tools: Read, Grep, Glob, Bash
model: inherit
---

You are a senior code reviewer ensuring high standards of code quality and security.

When invoked:
1. Run git diff to see recent changes
2. Focus on modified files
3. Begin review immediately

Review checklist:
- Code is clear and readable
- Functions and variables are well-named
- No duplicated code
- Proper error handling
- No exposed secrets or API keys
- Input validation implemented
- Good test coverage
- Performance considerations addressed

Provide feedback organized by priority:
- Critical issues (must fix)
- Warnings (should fix)
- Suggestions (consider improving)

Include specific examples of how to fix issues.
```

<h3 id="debugger">
  Debugger
</h3>

Ein Subagent, der sowohl Probleme analysieren als auch beheben kann. Im Gegensatz zum Code-Reviewer enthält dieser Edit, da das Beheben von Bugs die Änderung von Code erfordert. Der Prompt bietet einen klaren Workflow von der Diagnose zur Verifizierung.

```markdown theme={null}
---
name: debugger
description: Debugging specialist for errors, test failures, and unexpected behavior. Use proactively when encountering any issues.
tools: Read, Edit, Bash, Grep, Glob
---

You are an expert debugger specializing in root cause analysis.

When invoked:
1. Capture error message and stack trace
2. Identify reproduction steps
3. Isolate the failure location
4. Implement minimal fix
5. Verify solution works

Debugging process:
- Analyze error messages and logs
- Check recent code changes
- Form and test hypotheses
- Add strategic debug logging
- Inspect variable states

For each issue, provide:
- Root cause explanation
- Evidence supporting the diagnosis
- Specific code fix
- Testing approach
- Prevention recommendations

Focus on fixing the underlying issue, not the symptoms.
```

<h3 id="data-scientist">
  Data Scientist
</h3>

Ein domänenspezifischer Subagent für Datenanalyse-Arbeiten. Dieses Beispiel zeigt, wie man Subagenten für spezialisierte Workflows außerhalb typischer Coding-Aufgaben erstellt. Es setzt explizit `model: sonnet` für fähigere Analysen.

```markdown theme={null}
---
name: data-scientist
description: Data analysis expert for SQL queries, BigQuery operations, and data insights. Use proactively for data analysis tasks and queries.
tools: Bash, Read, Write
model: sonnet
---

You are a data scientist specializing in SQL and BigQuery analysis.

When invoked:
1. Understand the data analysis requirement
2. Write efficient SQL queries
3. Use BigQuery command line tools (bq) when appropriate
4. Analyze and summarize results
5. Present findings clearly

Key practices:
- Write optimized SQL queries with proper filters
- Use appropriate aggregations and joins
- Include comments explaining complex logic
- Format results for readability
- Provide data-driven recommendations

For each analysis:
- Explain the query approach
- Document any assumptions
- Highlight key findings
- Suggest next steps based on data

Always ensure queries are efficient and cost-effective.
```

<h3 id="database-query-validator">
  Datenbankabfrage-Validator
</h3>

Ein Subagent, der Bash-Zugriff zulässt, aber Befehle validiert, um nur schreibgeschützte SQL-Abfragen zu ermöglichen. Dieses Beispiel zeigt, wie man `PreToolUse`-Hooks für bedingte Validierung verwendet, wenn Sie feinere Kontrolle benötigen, als das `tools`-Feld bietet.

```markdown theme={null}
---
name: db-reader
description: Execute read-only database queries. Use when analyzing data or generating reports.
tools: Bash
hooks:
  PreToolUse:
    - matcher: "Bash"
      hooks:
        - type: command
          command: "./scripts/validate-readonly-query.sh"
---

You are a database analyst with read-only access. Execute SELECT queries to answer questions about the data.

When asked to analyze data:
1. Identify which tables contain the relevant data
2. Write efficient SELECT queries with appropriate filters
3. Present results clearly with context

You cannot modify data. If asked to INSERT, UPDATE, DELETE, or modify schema, explain that you only have read access.
```

Claude Code [übergibt Hook-Eingabe als JSON](/docs/de/hooks#pretooluse-input) über stdin an Hook-Befehle. Das Validierungsskript liest dieses JSON, extrahiert den auszuführenden Befehl und prüft ihn gegen eine Liste von SQL-Schreibvorgängen. Wenn ein Schreibvorgang erkannt wird, [beendet das Skript mit Code 2](/docs/de/hooks#exit-code-2-behavior-per-event), um die Ausführung zu blockieren, und gibt eine Fehlermeldung an Claude über stderr zurück.

Erstellen Sie das Validierungsskript überall in Ihrem Projekt. Der Pfad muss dem `command`-Feld in Ihrer Hook-Konfiguration entsprechen:

```bash theme={null}
#!/bin/bash
# Blocks SQL write operations, allows SELECT queries

# Read JSON input from stdin
INPUT=$(cat)

# Extract the command field from tool_input using jq
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

if [ -z "$COMMAND" ]; then
  exit 0
fi

# Block write operations (case-insensitive)
if echo "$COMMAND" | grep -iE '\b(INSERT|UPDATE|DELETE|DROP|CREATE|ALTER|TRUNCATE|REPLACE|MERGE)\b' > /dev/null; then
  echo "Blocked: Write operations not allowed. Use SELECT queries only." >&2
  exit 2
fi

exit 0
```

Machen Sie das Skript unter macOS und Linux ausführbar:

```bash theme={null}
chmod +x ./scripts/validate-readonly-query.sh
```

Unter Windows schreiben Sie das Validierungsskript in PowerShell und fügen `shell: powershell` zum Hook-Eintrag hinzu. Siehe [Hooks in PowerShell ausführen](/docs/de/hooks#windows-powershell-tool).

Der Hook empfängt JSON über stdin mit dem Bash-Befehl in `tool_input.command`. Exit-Code 2 blockiert die Operation und leitet die Fehlermeldung an Claude weiter. Siehe [Hooks](/docs/de/hooks#exit-code-output) für Details zu Exit-Codes und [Hook-Eingabe](/docs/de/hooks#pretooluse-input) für das vollständige Eingabeschema.

<h2 id="next-steps">
  Nächste Schritte
</h2>

Jetzt, da Sie Subagenten verstehen, erkunden Sie diese verwandten Funktionen:

* [Verteilen Sie Subagenten mit Plugins](/docs/de/plugins), um Subagenten über Teams oder Projekte hinweg zu teilen
* [Führen Sie Claude Code programmgesteuert aus](/docs/de/headless) mit dem Agent SDK für CI/CD und Automatisierung
* [Verwenden Sie MCP-Server](/docs/de/mcp), um Subagenten Zugriff auf externe Werkzeuge und Daten zu geben
