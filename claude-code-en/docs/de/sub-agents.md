> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Benutzerdefinierte Subagenten erstellen

> Erstellen und verwenden Sie spezialisierte KI-Subagenten in Claude Code für aufgabenspezifische Workflows und verbesserte Kontextverwaltung.

Subagenten sind spezialisierte KI-Assistenten, die bestimmte Arten von Aufgaben bearbeiten. Verwenden Sie einen, wenn eine Nebenaufgabe Ihre Hauptkonversation mit Suchergebnissen, Protokollen oder Dateiinhalten überfluten würde, auf die Sie nicht mehr verweisen werden: Der Subagent führt diese Arbeit in seinem eigenen Kontext durch und gibt nur die Zusammenfassung zurück. Definieren Sie einen benutzerdefinierten Subagenten, wenn Sie wiederholt denselben Typ von Worker mit denselben Anweisungen spawnen.

Jeder Subagent läuft in seinem eigenen Kontextfenster mit einem benutzerdefinierten Systemprompt, spezifischem Werkzeugzugriff und unabhängigen Berechtigungen. Wenn Claude auf eine Aufgabe trifft, die der Beschreibung eines Subagenten entspricht, delegiert es an diesen Subagenten, der unabhängig arbeitet und Ergebnisse zurückgibt. Um die Kontexteinsparungen in der Praxis zu sehen, zeigt die [Kontextfenster-Visualisierung](/docs/de/context-window) eine Sitzung, in der ein Subagent Recherchen in seinem eigenen separaten Fenster durchführt.

<Note>
  Subagenten arbeiten innerhalb einer einzelnen Sitzung. Um viele unabhängige Sitzungen parallel auszuführen und sie von einem Ort aus zu überwachen, siehe [Hintergrund-Agenten](/docs/de/agent-view). Für separate Sitzungen, die Nachrichten aneinander weitergeben, siehe [Sitzungsübergreifendes Messaging](/docs/de/cross-session-messaging). Für ein koordiniertes Team von Sitzungen, das Claude spawnt und beaufsichtigt, siehe [Agent-Teams](/docs/de/agent-teams).
</Note>

Subagenten helfen Ihnen:

* **Kontext bewahren**, indem Sie Exploration und Implementierung aus Ihrer Hauptkonversation heraushalten
* **Einschränkungen durchsetzen**, indem Sie begrenzen, welche Werkzeuge ein Subagent verwenden kann
* **Konfigurationen wiederverwenden** über Projekte hinweg mit Subagenten auf Benutzerebene
* **Verhalten spezialisieren** mit fokussierten Systemprompts für spezifische Domänen
* **Kosten kontrollieren**, indem Sie Aufgaben an schnellere, günstigere Modelle wie Haiku weiterleiten

Claude verwendet die Beschreibung jedes Subagenten, um zu entscheiden, wann Aufgaben delegiert werden. Wenn Sie einen Subagenten erstellen, schreiben Sie eine klare Beschreibung, damit Claude weiß, wann er ihn verwenden soll.

Diese Beschreibungen beanspruchen Kontext, daher halten Sie sie kurz. Wenn die kombinierten Beschreibungen Ihrer Subagenten, mit Ausnahme der integrierten, 15.000 Token überschreiten, zeigt Claude Code [eine Warnung beim Start mit der Gesamtanzahl der Token](/docs/de/errors#agent-descriptions-are-over-the-15000-token-limit). Kürzen Sie die `description`-Felder Ihrer Subagenten und verschieben Sie Details in den Systemprompt jedes Subagenten, der nur geladen wird, wenn dieser Subagent ausgeführt wird.

<h2 id="built-in-subagents">
  Integrierte Subagenten
</h2>

Claude Code enthält integrierte Subagenten, die Claude automatisch bei Bedarf verwendet. Jeder erbt die Berechtigungen der übergeordneten Konversation; die meisten werden mit einem eingeschränkten Werkzeugsatz ausgeführt.

Explore und Plan überspringen Ihre CLAUDE.md-Dateien und den Git-Status-Snapshot, um die Recherche schnell und kostengünstig zu halten. Alle anderen integrierten und [benutzerdefinierten Subagenten](#configure-subagents) laden beide, es sei denn, ihre Definition setzt das Feld [`omitClaudeMd`](#supported-frontmatter-fields), um die Benutzer-, Projekt- und lokalen CLAUDE.md-Dateien zu überspringen. Für die vollständige Aufschlüsselung dessen, was einen Subagenten erreicht, siehe [was beim Start geladen wird](#what-loads-at-startup).

<Tabs>
  <Tab title="Explore">
    Ein schneller, schreibgeschützter Agent, der für die Suche und Analyse von Codebases optimiert ist.

    * **Modell**: Erbt von der Hauptkonversation, begrenzt auf Opus in der Claude API, sodass Explore niemals auf einem teureren Modell ausgeführt wird als dem, das Sie bereits für die Sitzung gewählt haben, es sei denn, Sie setzen `CLAUDE_CODE_SUBAGENT_MODEL` und [erzwingen es auf jeden Subagenten](#run-every-subagent-on-one-model)
    * **Werkzeuge**: Schreibgeschützte Werkzeuge; Write und Edit sind nicht zulässig
    * **Zweck**: Dateiermittlung, Codesuche, Codebase-Exploration

    Ab v2.1.198 erbt Explore das Modell der Hauptkonversation, anstatt immer auf Haiku ausgeführt zu werden. In der Claude API ist das geerbte Modell auf Opus begrenzt: Eine Hauptkonversation auf einer höheren Stufe führt Explore auf Opus aus, und eine Hauptkonversation auf Sonnet oder Haiku führt Explore auf demselben Modell aus. Bei jedem anderen Anbieter, wie z. B. [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry oder Claude Platform on AWS](/docs/de/third-party-integrations), erbt Explore das Modell der Hauptkonversation direkt.

    Ein [Benutzer- oder Projekt-Subagent](#choose-the-subagent-scope) mit dem Namen `Explore` überschreibt den integrierten und behält sein eigenes `model`-Feld, daher definieren Sie einen mit `model: haiku`, um die Exploration auf einem kostengünstigeren Modell zu halten.

    Claude delegiert an Explore, wenn es eine Codebase durchsuchen oder verstehen muss, ohne Änderungen vorzunehmen. Dies hält Explorationsergebnisse aus Ihrem Hauptkonversationskontext heraus.

    Beim Aufrufen von Explore gibt Claude ein Gründlichkeitsniveau an: **quick** für gezielte Lookups, **medium** für ausgewogene Exploration oder **very thorough** für umfassende Analyse.
  </Tab>

  <Tab title="Plan">
    Ein Forschungsagent, der während des [Plan-Modus](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode) verwendet wird, um Kontext zu sammeln, bevor ein Plan präsentiert wird.

    * **Modell**: Erbt von der Hauptkonversation, es sei denn, Sie setzen `CLAUDE_CODE_SUBAGENT_MODEL` und [erzwingen es auf jeden Subagenten](#run-every-subagent-on-one-model)
    * **Werkzeuge**: Schreibgeschützte Werkzeuge; Write und Edit sind nicht zulässig
    * **Zweck**: Codebase-Recherche für Planung

    Wenn Sie sich im Plan-Modus befinden und Claude Ihre Codebase verstehen muss, delegiert es die Recherche an den Plan-Subagenten, damit die Explorationsergebnisse in einem separaten Kontextfenster bleiben, während die Hauptkonversation schreibgeschützt bleibt.
  </Tab>

  <Tab title="General-purpose">
    Ein fähiger Agent für komplexe, mehrstufige Aufgaben, die sowohl Exploration als auch Aktion erfordern.

    * **Modell**: das [`CLAUDE_CODE_SUBAGENT_MODEL`](#choose-a-model)-Modell, wenn Sie eines setzen und nichts anderes ein Modell auf andere Weise zuweist, ansonsten das Modell der Hauptkonversation; [Wählen Sie ein Modell](#choose-a-model) gibt die vollständige Reihenfolge an, und [Führen Sie jeden Subagenten auf einem Modell aus](#run-every-subagent-on-one-model) zeigt, wie die Variable diese Quellen überschreibt
    * **Werkzeuge**: Alle Werkzeuge [verfügbar für Subagenten](#available-tools)
    * **Zweck**: Komplexe Recherche, mehrstufige Operationen, Code-Änderungen

    Claude delegiert an general-purpose, wenn die Aufgabe sowohl Exploration als auch Änderung, komplexes Denken zur Interpretation von Ergebnissen oder mehrere abhängige Schritte erfordert.
  </Tab>

  <Tab title="Other">
    Claude Code enthält zusätzliche Hilfagenten für spezifische Aufgaben. Diese werden normalerweise automatisch aufgerufen, daher müssen Sie sie nicht direkt verwenden.

    | Agent             | Modell                                                                                                 | Wann Claude ihn verwendet                                                                                                                                                                                                                                                                                                                                                  |
    | :---------------- | :----------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | claude            | Keines eigenes; folgt der [Modellreihenfolge](#choose-a-model), wenn Claude ihn als Subagenten erzeugt | Wenn eine Aufgabe nicht zu einem spezialisierteren Agent passt. Ein Catch-All mit jedem Werkzeug [verfügbar für Subagenten](#available-tools). Auch der Standard-Agent für eine verteilte [Hintergrundsitzung](/docs/de/agent-view); [welcher Berechtigungsmodus sie startet](/docs/de/agent-view#permission-mode-model-and-effort), hängt davon ab, wie die Sitzung gestartet wurde |
    | statusline-setup  | Sonnet                                                                                                 | Wenn Sie `/statusline` ausführen, um Ihre Statuszeile zu konfigurieren                                                                                                                                                                                                                                                                                                     |
    | claude-code-guide | Haiku                                                                                                  | Wenn Sie Fragen zu Claude Code-Funktionen stellen                                                                                                                                                                                                                                                                                                                          |
  </Tab>
</Tabs>

Integrierte Subagenten sind in interaktiven Sitzungen standardmäßig registriert. Um sie einzuschränken:

* Um einen bestimmten integrierten Typ zu blockieren, fügen Sie ihn zu `permissions.deny` hinzu, wie in [Spezifische Subagenten deaktivieren](#disable-specific-subagents) gezeigt.
* Um zu verhindern, dass Claude an einen Subagenten delegiert, verweigern Sie das `Agent`-Werkzeug selbst mit [`permissions.deny`](/docs/de/permissions#tool-specific-permission-rules).
* Um nur die integrierten `Explore`- und `Plan`-Subagenten zu entfernen, setzen Sie [`CLAUDE_CODE_DISABLE_EXPLORE_PLAN_AGENTS=1`](/docs/de/env-vars). Claude liest und erkundet Dateien direkt, anstatt an sie zu delegieren. Erfordert Claude Code v2.1.198 oder später.
* Im [nicht-interaktiven Modus](/docs/de/headless) und dem [Agent SDK](/docs/de/agent-sdk/overview) setzen Sie [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/de/env-vars), um alle integrierten Typen zu entfernen und nur Ihre eigenen bereitzustellen.

Ein Agent-Werkzeugaufruf, der `subagent_type` auslässt, schlägt mit [`subagent_type is required`](/docs/de/errors#subagent-type-is-required) fehl, wenn die Sitzung keinen `general-purpose`-Subagenten hat, auf den zurückgegriffen werden kann.

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

    Claude delegiert an Ihren neuen Subagenten, der die Codebase durchsucht und Verbesserungsvorschläge zurückgibt. Im Transkript wird die Delegierung als eine Werkzeugaufrufs-Zeile angezeigt, die den Namen des Subagenten gefolgt von einer kurzen Aufgabenbeschreibung zeigt, wie z. B. `code-improver(Suggest code improvements)`.

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
| Plugin-Verzeichnis `agents/` | Wo Plugin aktiviert ist | 5 (niedrigste) | Installiert mit [Plugins](/docs/de/plugins/overview)              |

**Projekt-Subagenten** (`.claude/agents/`) sind ideal für Subagenten, die spezifisch für eine Codebase sind. Checken Sie sie in die Versionskontrolle ein, damit Ihr Team sie gemeinsam verwenden und verbessern kann.

Projekt-Subagenten werden durch Aufwärts-Traversierung vom aktuellen Arbeitsverzeichnis entdeckt, sodass jedes `.claude/agents/`-Verzeichnis zwischen dort und dem Repository-Root gescannt wird. Wenn mehr als eines dieser verschachtelten Verzeichnisse denselben `name` definiert, verwendet Claude Code die Definition, die dem Arbeitsverzeichnis am nächsten liegt.

Wenn Sie ein Verzeichnis mit `--add-dir` oder `/add-dir` hinzufügen, lädt Claude Code auch seinen `.claude/agents/`-Ordner zusammen mit Ihren Projekt-Subagenten. Siehe [Zusätzliche Verzeichnisse](/docs/de/permissions#additional-directories-grant-file-access-not-configuration) für welche anderen Konfigurationstypen aus `--add-dir` geladen werden. Um Subagenten über Projekte hinweg zu teilen, ohne `--add-dir` zu verwenden, nutzen Sie `~/.claude/agents/` oder ein [Plugin](/docs/de/plugins/overview).

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

Das `--agents`-Flag akzeptiert JSON mit einem `prompt`-Feld plus diese [Frontmatter](#supported-frontmatter-fields)-Felder: `description`, `tools`, `disallowedTools`, `model`, `permissionMode`, `mcpServers`, `hooks`, `maxTurns`, `skills`, `initialPrompt`, `memory`, `effort`, `background`, `omitClaudeMd` und `isolation`. Verwenden Sie `prompt` für den Systemprompt, äquivalent zum Markdown-Body in dateibasierten Subagenten. `color` und `experimental` werden hier nicht akzeptiert und werden ignoriert statt abgelehnt.

Jeder Top-Level-Schlüssel im JSON ist der Name des Agenten. Starten Sie einen Namen nicht mit `-`.

Für das, was Claude Code mit einem Wert tut, den es nicht laden kann, und die Flags und Umgebungsvariable, die diese Überprüfung überspringen, siehe [`Invalid --agents configuration`](/docs/de/errors#invalid-agents-configuration).

**Verwaltete Subagenten** werden von Organisationsadministratoren bereitgestellt. Platzieren Sie Markdown-Dateien in `.claude/agents/` im [Verzeichnis der verwalteten Einstellungen](/docs/de/managed-settings#delivery-mechanisms), wobei Sie das gleiche Frontmatter-Format wie bei Projekt- und Benutzer-Subagenten verwenden. Verwaltete Definitionen haben Vorrang vor Projekt- und Benutzer-Subagenten mit demselben Namen.

**Plugin-Subagenten** stammen von [Plugins](/docs/de/plugins/overview), die Sie installiert haben. Sie werden automatisch zusammen mit Ihren benutzerdefinierten Subagenten geladen und erscheinen in der @-Erwähnung-Typeahead unter ihrem scoped name. Siehe die [Plugin-Komponenten-Referenz](/docs/de/plugins/components#agents) für Details zum Erstellen von Plugin-Subagenten.

<Note>
  Aus Sicherheitsgründen unterstützen Plugin-Subagenten die Frontmatter-Felder `hooks`, `mcpServers` oder `permissionMode` nicht. Diese Felder werden ignoriert, wenn Agenten aus einem Plugin geladen werden. Wenn Sie sie benötigen, kopieren Sie die Agent-Datei in `.claude/agents/` oder `~/.claude/agents/`. Sie können auch Regeln zu [`permissions.allow`](/docs/de/settings-reference#permissions-allow) in `settings.json` oder `settings.local.json` hinzufügen, aber diese Regeln gelten für die gesamte Sitzung, nicht nur für den Plugin-Subagenten.
</Note>

Subagenten-Definitionen aus einem dieser Umfänge sind auch für [Agent-Teams](/docs/de/agent-teams#use-subagent-definitions-for-teammates) verfügbar: Beim Spawnen eines Teammates können Sie auf einen Subagenten-Typ verweisen, und Claude Code wendet Teile dieser Definition auf den Teammate an. Siehe [Agent-Teams](/docs/de/agent-teams#use-subagent-definitions-for-teammates) für welche Teile in jedem Anzeigemodus gelten.

<h3 id="write-subagent-files">
  Schreiben Sie Subagenten-Dateien
</h3>

Subagenten-Dateien verwenden YAML-Frontmatter für die Konfiguration, gefolgt vom Systemprompt in Markdown:

<Note>
  Claude Code überwacht `~/.claude/agents/` und `.claude/agents/`. Wenn Sie eine Subagenten-Datei auf der Festplatte hinzufügen oder bearbeiten oder Claude bitten, eine für Sie zu schreiben, erkennt Claude Code die Änderung innerhalb weniger Sekunden und die nächste Delegation verwendet die aktualisierte Definition, ohne dass ein Neustart erforderlich ist.

  Drei Fälle erfordern immer noch einen Neustart:

  * Der Watcher deckt nur Verzeichnisse ab, die beim Sitzungsstart existierten, daher müssen Sie nach dem Erstellen der ersten Agent-Datei eines Umfangs in einem neuen `agents`-Verzeichnis neu starten, um sie zu laden.
  * Claude Code überwacht `.claude/agents/` nicht in Verzeichnissen, die mit `--add-dir` oder `/add-dir` hinzugefügt wurden, daher müssen Sie nach dem Hinzufügen oder Bearbeiten eines Subagenten dort neu starten, um die Änderung zu laden.
  * Sitzungen, die mit `--disable-slash-commands` gestartet wurden, überwachen diese Verzeichnisse überhaupt nicht.
</Note>

```markdown .claude/agents/code-reviewer.md theme={null}
---
name: code-reviewer
description: Reviews code for quality and best practices
tools: Read, Glob, Grep
model: sonnet
---

You are a code reviewer. When invoked, analyze the code and provide
specific, actionable feedback on quality, security, and best practices.
```

Das Frontmatter definiert die Metadaten und Konfiguration des Subagenten. Der Body wird zum Systemprompt, der das Verhalten des Subagenten leitet. Subagenten erhalten nur diesen Systemprompt plus grundlegende Umgebungsdetails wie Arbeitsverzeichnis, nicht den Claude Code-Systemprompt.

Im [nicht-interaktiven Modus](/docs/de/headless) hängt das Flag [`--append-subagent-system-prompt`](/docs/de/cli-reference#cli-flags) den von Ihnen bereitgestellten Text an das Ende des Systemprompts jedes Subagenten an, einschließlich verschachtelter Subagenten, außer einem [forked Subagenten](#fork-the-current-conversation), der den Prompt der Konversation selbst wiederverwendet. Erfordert Claude Code v2.1.205 oder später. Wenn Ihr Text zu lang ist, um ihn in der Befehlszeile zu übergeben, speichern Sie ihn in einer Datei und übergeben Sie den Pfad mit `--append-subagent-system-prompt-file` stattdessen. Das Datei-Flag erfordert Claude Code v2.1.261 oder später.

Ein Subagent startet im aktuellen Arbeitsverzeichnis der Hauptkonversation. Innerhalb eines Subagenten bleiben `cd`-Befehle nicht zwischen Bash- oder PowerShell-Werkzeugaufrufen bestehen und beeinflussen nicht das Arbeitsverzeichnis der Hauptkonversation. Um dem Subagenten stattdessen eine isolierte Kopie des Repositorys zu geben, setzen Sie [`isolation: worktree`](#supported-frontmatter-fields).

Ein Subagent mit `isolation: worktree` führt seine Bash- und PowerShell-Befehle in seinem Worktree aus. Ein Befehl, dessen Arbeitsverzeichnis sich stattdessen zu Ihrem Haupt-Checkout auflöst, beispielsweise weil das Worktree-Verzeichnis entfernt wurde, während der Subagent lief, schlägt mit einem Fehler fehl. Vor v2.1.203 konnte ein solcher Befehl im Haupt-Checkout ausgeführt werden.

Diese Arbeitsverzeichnis-Überprüfung deckt das gesamte Repository ab, das das Verzeichnis enthält, von dem aus Sie Claude Code gestartet haben. Wenn Ihre Sitzung in einem verknüpften [Worktree](/docs/de/worktrees) läuft, deckt die Überprüfung auch den Haupt-Checkout ab, von dem dieser Worktree verknüpft ist. Vor v2.1.210 deckte die Überprüfung nur das Start-Verzeichnis selbst ab. Ein Befehl, dessen Arbeitsverzeichnis sich anderswo im selben Repository auflöste, wie das Repository-Root, wenn Sie Claude Code von einem Monorepo-Unterverzeichnis aus gestartet haben, wurde dort ausgeführt, anstatt zu fehlschlagen.

Für Bash-Befehle überprüft Claude Code auch den Befehl selbst auf zwei Arten:

* Es blockiert einen Befehl, der Git in den Haupt-Checkout umleitet.
* Es weigert sich, einen Befehl auszuführen, wenn es nicht vom Befehlstext überprüfen kann, dass jedes Git, das der Befehl ausführt, im Worktree bleibt, beispielsweise wenn der Befehlsname zur Laufzeit berechnet wird.

Die Umleitungsvektoren und die Formregeln sind unter [Wie Claude Code Isolation erzwingt](/docs/de/worktrees#how-claude-code-enforces-isolation) aufgelistet. PowerShell-Befehle erhalten nur die Arbeitsverzeichnis-Überprüfung.

[Monitor](/docs/de/tools-reference#monitor-tool)-Befehle durchlaufen die gleichen Arbeitsverzeichnis- und Befehlsinhalts-Überprüfungen wie Bash-Befehle.

Wenn die Hauptkonversation selbst isoliert in einem Worktree läuft, wendet Claude Code die gleichen Überprüfungen auf die Sitzung und jeden Subagenten an, den es spawnt, einschließlich Subagenten ohne `isolation: worktree`; siehe [Wie Claude Code Isolation erzwingt](/docs/de/worktrees#how-claude-code-enforces-isolation).

<h3 id="supported-frontmatter-fields">
  Frontmatter-Referenz
</h3>

Konfigurieren Sie einen Subagenten mit YAML-[Frontmatter](/docs/de/glossary#frontmatter) zwischen `---`-Markierungen am Anfang seiner Datei, und schreiben Sie seinen Systemprompt als Markdown nach dem schließenden `---`. Nur `name` und `description` sind erforderlich.

Mehrteilige Feldnamen verwenden camelCase, wie `maxTurns` und `disallowedTools`, und müssen genau mit der Tabelle übereinstimmen: Claude Code ignoriert ein Feld, das es nicht erkennt, ohne einen Fehler zu melden. Um herauszufinden, warum eine Subagenten-Datei nicht geladen wurde, siehe [Subagenten-Dateien, die Claude Code überspringt](#subagent-files-claude-code-skips).

| Feld              | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :---------------- | :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`            | Ja           | Eindeutige Kennung, wie `code-reviewer` oder `reviewer-v2`. [Hooks](/docs/de/hooks#subagentstart) erhalten diesen Wert als `agent_type`. Der Dateiname muss nicht übereinstimmen. Namen können kein `:` enthalten, das für [Plugin-Umfang-Identifier](/docs/de/plugins/overview) wie `my-plugin:reviewer` reserviert ist. Claude Code lädt eine Datei, deren Name eins enthält, nicht und protokolliert einen Fehler im Debug-Protokoll. Vor v2.1.218 wurden solche Namen akzeptiert                                                                                 |
| `description`     | Ja           | Wann Claude an diesen Subagenten delegieren sollte                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `tools`           | Nein         | [Werkzeuge](#available-tools), die der Subagent verwenden kann, als kommagetrennte Zeichenkette wie `Read, Grep, Bash` oder eine YAML-Liste. Erbt jedes Werkzeug, das für Subagenten verfügbar ist, wenn weggelassen. Wenn kein Eintrag in der Liste sich zu einem Werkzeug auflöst, schlägt der Subagent normalerweise [fehl zu starten](/docs/de/errors#agent-would-be-spawned-with-zero-tools) mit einem Fehler, der die Einträge benennt. Um Skills in den Kontext zu laden, verwenden Sie das `skills`-Feld statt `Skill` hier aufzulisten                 |
| `disallowedTools` | Nein         | Werkzeuge zum Verweigern, entfernt aus geerbter oder angegebener Liste. Gleiches Format wie `tools`. Ein Eintrag mit einem Spezifizierer, wie `Bash(git push *)`, entfernt immer noch das [ganze Werkzeug](#available-tools)                                                                                                                                                                                                                                                                                                                               |
| `model`           | Nein         | [Modell](#choose-a-model) zu verwenden: `sonnet`, `opus`, `haiku`, `fable`, eine vollständige Modell-ID wie `claude-opus-5-5` oder `inherit`. Wenn Sie es weglassen, wählt Claude Code das Modell in der [Subagenten-Modell-Reihenfolge](#choose-a-model)                                                                                                                                                                                                                                                                                                  |
| `permissionMode`  | Nein         | [Berechtigungsmodus](#permission-modes): `default`, `acceptEdits`, `auto`, `dontAsk`, `bypassPermissions`, `plan` oder `manual` als Alias für `default`. Der `manual`-Alias erfordert Claude Code v2.1.200 oder später. Ignoriert für [Plugin-Subagenten](#choose-the-subagent-scope)                                                                                                                                                                                                                                                                      |
| `maxTurns`        | Nein         | Maximale Anzahl von Agenten-Turns, bevor der Subagent stoppt. Wenn der Subagent die Grenze erreicht, gibt Claude Code seine Ausgabe als teilweise markiert zurück, und Claude kann [ihn fortsetzen](#resume-subagents), um fortzufahren. Die teilweise Markierung erfordert Claude Code v2.1.246 oder später                                                                                                                                                                                                                                               |
| `skills`          | Nein         | [Skills](/docs/de/skills) zum Vorausladen in den Kontext des Subagenten beim Start. Der vollständige Skill-Inhalt wird eingespritzt, nicht nur die Beschreibung. Subagenten können weiterhin unlisted Projekt-, Benutzer- und Plugin-Skills durch das Skill-Werkzeug aufrufen                                                                                                                                                                                                                                                                                   |
| `mcpServers`      | Nein         | [MCP-Server](/docs/de/mcp) verfügbar für diesen Subagenten. Jeder Eintrag ist entweder ein Servername, der auf einen bereits konfigurierten Server verweist (z. B. `"slack"`) oder eine Inline-Definition mit dem Servernamen als Schlüssel und einer vollständigen [MCP-Server-Konfiguration](/docs/de/mcp#installing-mcp-servers) als Wert. Ignoriert für [Plugin-Subagenten](#choose-the-subagent-scope)                                                                                                                                                          |
| `hooks`           | Nein         | [Lifecycle-Hooks](#define-hooks-for-subagents) mit Umfang auf diesen Subagenten. Ignoriert für [Plugin-Subagenten](#choose-the-subagent-scope)                                                                                                                                                                                                                                                                                                                                                                                                             |
| `memory`          | Nein         | [Persistenter Speicherumfang](#enable-persistent-memory): `user`, `project` oder `local`. Ermöglicht sitzungsübergreifendes Lernen                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `background`      | Nein         | Auf `true` setzen, um diesen Subagenten im Hintergrund zu halten, auch wenn Claude ihn im Vordergrund ausführen möchte. Wo [Fork-Modus](#turn-fork-mode-on-or-off) aktiviert ist, führt Claude Code die Subagenten, die Claude spawnt, bereits [im Hintergrund](#run-subagents-in-foreground-or-background) aus                                                                                                                                                                                                                                            |
| `omitClaudeMd`    | Nein         | Auf `true` setzen, um diesen Subagenten ohne die Benutzer-, Projekt- und lokalen CLAUDE.md-Dateien zu starten; [verwaltete Richtliniendateien](/docs/de/memory#how-claude-md-files-load) werden immer noch geladen, außer für [verwaltete Subagenten](#choose-the-subagent-scope). Verwenden Sie es für Subagenten, die alles, was sie benötigen, aus dem [Delegationsprompt](#what-loads-at-startup) erhalten. Ignoriert, wenn der Agent als Hauptsitzungs-Agent über `--agent` oder die `agent`-Einstellung läuft. Erfordert Claude Code v2.1.271 oder später |
| `effort`          | Nein         | Aufwandsstufe, wenn dieser Subagent aktiv ist. Überschreibt die Aufwandsstufe der Sitzung. Standard: erbt von Sitzung. Optionen: `low`, `medium`, `high`, `xhigh`, `max`; verfügbare Stufen hängen vom Modell ab                                                                                                                                                                                                                                                                                                                                           |
| `isolation`       | Nein         | Auf `worktree` setzen, um den Subagenten in einem temporären [Git-Worktree](/docs/de/worktrees) auszuführen, was ihm eine isolierte Kopie des Repositorys gibt, die standardmäßig von Ihrem [Standard-Branch](/docs/de/worktrees#choose-the-base-branch) verzweigt ist, anstatt vom `HEAD` der übergeordneten Sitzung. Der Worktree wird automatisch bereinigt, wenn der Subagent keine Änderungen vornimmt                                                                                                                                                          |
| `color`           | Nein         | Anzeigefarbe für den Subagenten in der Aufgabenliste und dem Transkript. Akzeptiert `red`, `blue`, `green`, `yellow`, `purple`, `orange`, `pink` oder `cyan`                                                                                                                                                                                                                                                                                                                                                                                               |
| `initialPrompt`   | Nein         | Auto-eingereicht als der erste Benutzer-Turn, wenn dieser Agent als Hauptsitzungs-Agent läuft (über `--agent` oder die `agent`-Einstellung). [Befehle](/docs/de/commands) und [Skills](/docs/de/skills) werden verarbeitet. Vorangestellt zu jedem vom Benutzer bereitgestellten Prompt. Ignoriert für [Plugin-Subagenten](#choose-the-subagent-scope)                                                                                                                                                                                                               |
| `experimental`    | Nein         | Zuordnung experimenteller Optionen. Setzen Sie seinen `cacheTtl`-Schlüssel auf `5m` oder `1h`, um die [Prompt-Cache-Lebensdauer](/docs/de/prompt-caching#choose-the-ttl-yourself) für die Anfragen dieses Subagenten zu wählen, an der Stelle des Frontmatters in der [Cache-Lebensdauer-Priorität](/docs/de/prompt-caching#choose-the-ttl-yourself). Claude Code ignoriert jeden anderen Wert, ignoriert `1h`, während Ihr Claude-Abonnement Nutzungsguthaben verwendet, und liest das Feld nur aus Subagenten-Dateien. Erfordert Claude Code v2.1.248 oder später  |

Schreiben Sie `cacheTtl` in die `experimental`-Zuordnung, nicht auf die oberste Ebene des Frontmatters.

```yaml theme={null}
---
name: repo-auditor
description: Audits a large repository and reports what it finds
experimental:
  cacheTtl: 1h
---
```

<h4 id="subagent-files-claude-code-skips">
  Subagenten-Dateien, die Claude Code überspringt
</h4>

Claude Code überspringt eine Datei in einem Projekt-, Benutzer- oder verwalteten `agents`-Verzeichnis oder in einem unter einem Verzeichnis, das Sie mit `--add-dir` hinzufügen, ohne es in der Sitzung zu melden, wenn das Frontmatter eines dieser Probleme hat:

* **Kein `name`**: Claude Code behandelt die Datei als Dokumentation, die neben Ihren Agenten aufbewahrt wird.
* **Ein öffnendes `---`, das nicht die erste Zeile der Datei ist**: Claude Code liest die Datei als ohne Frontmatter und behandelt sie als Dokumentation.
* **Ein `name`, der mit `-` beginnt oder `:` enthält**: Claude Code überspringt die Datei und schreibt einen Fehler in das Debug-Protokoll. Siehe die `name`-Zeile in der Tabelle oben.
* **Ein `name`, aber keine `description`**: Claude Code überspringt die Datei und schreibt den Grund in das Debug-Protokoll.
* **YAML, das nicht analysiert wird**: Claude Code liest keine Felder aus der Datei, überspringt sie und schreibt den Parse-Fehler in das Debug-Protokoll.

Um das Debug-Protokoll zu sehen, führen Sie Claude Code mit `--debug` aus.

Ein [Plugin-Subagent](/docs/de/plugins/components#agents), dessen Frontmatter kein `name` hat oder nicht analysiert wird, wird immer noch unter seinem Dateinamen geladen.

<h5 id="check-an-agents-directory-before-a-session">
  Überprüfen Sie ein `agents`-Verzeichnis vor einer Sitzung
</h5>

Um Dateien in einem `agents`-Verzeichnis zu finden, deren Frontmatter nicht analysiert wird, führen Sie `claude plugin validate` gegen das Verzeichnis aus, beispielsweise `.claude/agents` oder `~/.claude/agents`. Claude Code überprüft nur [das Verzeichnis, das Sie benennen](/docs/de/plugins/cli-reference#validate-a-directory), und kennzeichnet keine Datei, deren Frontmatter analysiert wird, aber kein `name` hat. Erfordert Claude Code v2.1.233 oder später.

<h3 id="choose-a-model">
  Wählen Sie ein Modell
</h3>

Das `model`-Feld steuert, welches Modell der Subagent verwendet:

* **Modell-Alias**: Verwenden Sie einen der verfügbaren Aliase: `sonnet`, `opus`, `haiku` oder `fable`
* **Vollständige Modell-ID**: Verwenden Sie eine vollständige Modell-ID wie `claude-opus-5-5` oder `claude-sonnet-5`. Akzeptiert dieselben Werte wie das `--model`-Flag
* **inherit**: Verwenden Sie dasselbe Modell wie die Hauptkonversation

Wenn Claude einen Subagenten aufruft, kann es auch einen `model`-Parameter für diese spezifische Invokation übergeben. Claude Code löst das Modell des Subagenten in dieser Reihenfolge auf:

1. Der Per-Invokation-`model`-Parameter
2. Das `model`-Frontmatter des Subagenten, wobei `inherit` das Modell der Hauptkonversation auswählt
3. Die [`CLAUDE_CODE_SUBAGENT_MODEL`](/docs/de/model-config#environment-variables)-Umgebungsvariable, wenn Sie sie auf einen Modell-Alias oder eine Modell-ID setzen
4. Das Modell der Hauptkonversation

In zwei Fällen wird ein Familien-Alias wie `opus` im Per-Invokation-Parameter oder im Frontmatter stattdessen zum Modell der Hauptkonversation aufgelöst, anstatt zur [Version, auf die der Alias verweist](/docs/de/model-config#model-aliases):

* **Das Modell der Hauptkonversation gehört zu dieser Familie**: Der Subagent läuft auf dem genauen Modell der Hauptkonversation, einschließlich aller `[1m]`-Suffixe, sodass er das gleiche [erweiterte Kontextfenster](/docs/de/model-config#extended-context) wie die Hauptkonversation erhält.
* **Claude Code kann die Modellfamilie der Hauptkonversation nicht bestimmen, auf [einem anderen Provider als der Anthropic API](/docs/de/third-party-integrations)**: Dies kann mit einem [Application Inference Profile ARN](/docs/de/amazon-bedrock#iam-configuration) auf Amazon Bedrock passieren, das Claude Code nicht zu einem Backing-Modell aufgelöst hat. Dieser Fall deckt nur den `opus`-Alias ab und gilt nicht, wenn Sie [`ANTHROPIC_DEFAULT_OPUS_MODEL`](/docs/de/model-config#environment-variables) setzen, da `opus` dann zum von Ihnen gesetzten Modell aufgelöst wird.

Ein Alias in `CLAUDE_CODE_SUBAGENT_MODEL` wird immer zur Version aufgelöst, auf die der Alias verweist, auch wenn er die Familie der Hauptkonversation benennt.

Das Setzen von `CLAUDE_CODE_SUBAGENT_MODEL` allein ändert nicht das Modell, auf dem die integrierten Explore- und Plan-Subagenten laufen. Um es zu ändern, siehe [Führen Sie jeden Subagenten auf einem Modell aus](#run-every-subagent-on-one-model).

Vor v2.1.251 kam `CLAUDE_CODE_SUBAGENT_MODEL` zuerst in dieser Reihenfolge und überschrieb sowohl den Per-Invokation-Parameter als auch das Frontmatter, einschließlich `model: inherit`.

Das Setzen der Variablen auf `inherit` ist dasselbe wie das Nichtsetzen. Vor v2.1.196 erzwang dieser Wert Subagenten auf das Modell der Hauptkonversation und ignorierte die anderen Quellen.

Claude Code überprüft die Per-Invokation-Parameter, Frontmatter und Umgebungsvariablenwerte gegen die [`availableModels`](/docs/de/model-config#restrict-model-selection)-Allowlist Ihrer Organisation. Für einen blockierten Wert ersetzt es ein anderes Modell:

* Wenn der blockierte Wert ein Familien-Alias wie `opus` ist, führt Claude Code den Subagenten auf der neuesten Version dieser Familie aus, die die Allowlist zulässt, und folgt den gleichen [Substitutionsregeln und Provider-Umfang](/docs/de/model-config#restrict-model-selection) wie `/model`. Vor v2.1.222 führte Claude Code den Subagenten auf dem geerbten Modell für einen blockierten Familien-Alias aus.
* Für jeden anderen blockierten Wert, auf Providern, wo diese Substitution nicht funktioniert, oder wenn die Allowlist keine Version der Familie zulässt, führt Claude Code den Subagenten stattdessen auf dem geerbten Modell aus. Wenn Sie `CLAUDE_CODE_SUBAGENT_MODEL` setzen, versucht Claude Code zuerst dieses Modell unter den gleichen Regeln.

In interaktiven Sitzungen zeigt Claude Code eine Warnung an, die das angeforderte Modell und das Modell benennt, auf dem der Subagent läuft, für beide Substitutionen.

Um zu überprüfen, auf welchem Modell ein Subagent läuft, führen Sie [`/tasks`](/docs/de/commands) aus. Claude Code benennt das Modell in der Zeile des Subagenten und fügt die [Aufwandsstufe](/docs/de/model-config#adjust-effort-level) hinzu, wenn die Definition des Subagenten oder der Skill, von dem er geforkt wurde, [`effort`](#supported-frontmatter-fields) setzt. Erfordert Claude Code v2.1.242 oder später.

Ein Per-Invokation-`model`-Parameter gilt auch, wenn der Subagent [fortgesetzt oder eine Folgenachricht gesendet wird](#resume-subagents), sodass der Subagent auf diesem Modell bleibt. Vor v2.1.211 ließ das Fortsetzen den Per-Invokation-Wert fallen und der Subagent kehrte zu seinem `model`-Feld oder, ohne einen, zum Modell der Hauptkonversation zurück.

Ab v2.1.198 erben Subagenten auch die [Extended Thinking](/docs/de/model-config#extended-thinking)-Konfiguration der Hauptkonversation: Wenn Thinking in Ihrer Sitzung aktiviert ist, ist es für den Subagenten aktiviert, und wenn es deaktiviert ist, bleibt es deaktiviert. Es gibt keine Pro-Subagenten-Thinking-Einstellung. Vor v2.1.198 liefen Subagenten mit deaktiviertem Extended Thinking, unabhängig von der Einstellung der Hauptkonversation.

<h4 id="run-every-subagent-on-one-model">
  Führen Sie jeden Subagenten auf einem Modell aus
</h4>

`CLAUDE_CODE_SUBAGENT_MODEL` ist ein Standard, daher hat die Definition eines Subagenten oder ein Modell, das Claude übergibt, immer noch Vorrang vor ihm. Um ein Modell auf jeden Subagenten, [Teammate](/docs/de/agent-teams#specify-teammates-and-models) und [Workflow-Agent](/docs/de/workflows) anzuwenden, setzen Sie auch `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` auf `1`. Erfordert Claude Code v2.1.257 oder später.

* Wenn Sie beide Variablen setzen, laufen Subagenten auf dem Modell in `CLAUDE_CODE_SUBAGENT_MODEL`.
* Wenn Sie nur `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` setzen, laufen Subagenten auf dem Modell der Hauptkonversation.

Um beispielsweise jeden Subagenten auf Haiku auszuführen, setzen Sie beide Variablen im `env`-Block einer [Einstellungsdatei](/docs/de/settings):

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_SUBAGENT_MODEL": "haiku",
    "CLAUDE_CODE_SUBAGENT_MODEL_FORCE": "1"
  }
}
```

Um zu überprüfen, dass die Einstellung wirksam wurde, führen Sie [`/tasks`](/docs/de/commands) aus, während ein Subagent läuft. Die Zeile des Subagenten zeigt das Modell, auf dem er läuft.

Während `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` [aktiviert](/docs/de/env-vars) ist, ignoriert Claude Code das `model`-Feld jeder Subagenten-Definition, einschließlich der integrierten Explore- und Plan-Subagenten, und Claude kann kein Modell übergeben, wenn es einen Subagenten startet. Zwei Arten von Subagenten laufen immer noch auf dem Modell der Hauptkonversation:

* Ein [Fork](#fork-the-current-conversation)
* Ein [Skill, der in einem Subagenten läuft](/docs/de/skills#run-skills-in-a-subagent) mit `model: inherit`

Wenn Sie nur `CLAUDE_CODE_SUBAGENT_MODEL_FORCE` setzen, behält der integrierte Explore-Subagent seine [Modell-Obergrenze](#built-in-subagents).

<h3 id="control-subagent-capabilities">
  Kontrollieren Sie Subagenten-Fähigkeiten
</h3>

Sie können kontrollieren, was Subagenten durch Werkzeugzugriff, Berechtigungsmodi und bedingte Regeln tun können.

<h4 id="available-tools">
  Verfügbare Werkzeuge
</h4>

Subagenten erben die [integrierten Werkzeuge](/docs/de/tools-reference) und MCP-Werkzeuge, die in der Hauptkonversation verfügbar sind, eingeengt durch zwei Filter: Der erste entfernt eine kurze Liste von Werkzeugen aus jedem Subagenten, und der zweite reduziert den integrierten Werkzeugsatz für Subagenten, die im [Hintergrund](#run-subagents-in-foreground-or-background) laufen, was der Standard ist. Auf macOS, Linux und WSL kann ein Subagent auch die Glob- und Grep-Werkzeuge erhalten, wenn die Hauptkonversation sie nicht hat, wie unter [Glob-Werkzeugverhalten](/docs/de/tools-reference#glob-tool-behavior) beschrieben. [Forks](#fork-the-current-conversation) überspringen beide Filter und erhalten den genauen Werkzeugpool der Hauptkonversation. Der erste Filter entfernt diese Werkzeuge, auch wenn sie im `tools`-Feld aufgelistet sind:

* `Agent`, wenn der Subagent die [Tiefengrenze](#let-subagents-spawn-their-own-subagents) erreicht hat; in einem [Fork](#fork-the-current-conversation) bleibt das Werkzeug aufgelistet, gibt aber stattdessen einen Fehler zurück
* `AskUserQuestion`
* `EndConversation`, das nur die Hauptkonversation beenden kann; siehe [EndConversation-Werkzeugverhalten](/docs/de/tools-reference#endconversation-tool-behavior)
* `EnterPlanMode`
* `ExitPlanMode`, es sei denn, der [`permissionMode`](#permission-modes) des Subagenten ist `plan`
* `ScheduleWakeup`
* `WaitForMcpServers`
* `Workflow`

Der zweite Filter gilt für Subagenten, die im Hintergrund laufen. Abgesehen von `Agent` und `ExitPlanMode`, die den Bedingungen des ersten Filters folgen, wo immer der Subagent läuft, behält ein Hintergrund-Subagent jedes MCP-Werkzeug, aber nur diese integrierten Werkzeuge: `Read`, `Grep`, `Glob`, `LSP`, `Bash`, `PowerShell`, `Edit`, `Write`, `NotebookEdit`, `WebFetch`, `WebSearch`, `TodoWrite`, `Skill`, `ToolSearch`, `EnterWorktree`, `ExitWorktree`, `Monitor`, `TaskStop`, `SendMessage` und `Artifact`, plus [`SubagentHandback`](/docs/de/tools-reference) für einen Subagenten, der durch ihn berichtet. Claude Code entfernt jedes andere integrierte Werkzeug aus einem Hintergrund-Subagenten, ob geerbt oder im `tools`-Feld aufgelistet, sodass die gleiche Definition zu verschiedenen Werkzeugen im Vordergrund und im Hintergrund führen kann. Die Entfernung meldet keinen Fehler, es sei denn, sie hinterlässt die `tools`-Liste [aufgelöst zu nichts](/docs/de/errors#agent-would-be-spawned-with-zero-tools).

Vor v2.1.280 konnten Hintergrund-Subagenten `LSP` nicht verwenden.

[`ListAgents`](/docs/de/cross-session-messaging) folgt diesen Filtern wie jedes integrierte Werkzeug: Ein Vordergrund-Subagent erbt es in Sitzungen, wo sitzungsübergreifendes Messaging aktiviert ist, und ein Hintergrund-Subagent behält es nicht.

Teammates in [Agent-Teams](/docs/de/agent-teams) behalten zusätzlich die Task-Werkzeuge und Cron-Werkzeuge: `TaskCreate`, `TaskGet`, `TaskList`, `TaskUpdate`, `CronCreate`, `CronDelete` und `CronList`.

In einer [Sitzung ohne die Task-Werkzeuge](/docs/de/tools-reference#task-tool-availability) stellt Claude Code die Task-Werkzeuge auch nicht für Subagenten bereit, auch wenn der Subagent ein anderes Modell ausführt. Ein In-Process-Teammate folgt Ihrer Sitzung auf die gleiche Weise, während ein Teammate in seinem eigenen [Split-Pane](/docs/de/agent-teams#choose-a-display-mode) als separater Claude Code-Prozess läuft, sodass sein eigenes Modell entscheidet.

Um Werkzeuge einzuschränken, verwenden Sie das `tools`-Feld als Allowlist oder das `disallowedTools`-Feld als Denylist. Dieses Beispiel verwendet `tools`, um ausschließlich Read, Grep, Glob und Bash zuzulassen. Der Subagent kann keine Dateien bearbeiten, keine Dateien schreiben oder MCP-Werkzeuge verwenden:

```yaml theme={null}
---
name: safe-researcher
description: Research agent with restricted capabilities
tools: Read, Grep, Glob, Bash
---
```

Dieses Beispiel verwendet `disallowedTools`, um alle verfügbaren Werkzeuge außer Write und Edit zu erben. Der Subagent behält Bash, MCP-Werkzeuge und den Rest seines Pools:

```yaml theme={null}
---
name: no-writes
description: Inherits the available tools except file writes
disallowedTools: Write, Edit
---
```

Wenn beide gesetzt sind, wird `disallowedTools` zuerst angewendet, dann wird `tools` gegen den verbleibenden Pool aufgelöst. Ein Werkzeug, das in beiden aufgelistet ist, wird entfernt.

Wenn nichts in der `tools`-Liste sich zu einem Werkzeug auflöst, beispielsweise weil jeder Eintrag falsch geschrieben ist oder ein Werkzeug benennt, das nicht für Subagenten verfügbar ist, weigert sich Claude Code normalerweise, den Subagenten zu starten, und das Agent-Werkzeug gibt einen Fehler zurück, der die ungelösten Einträge benennt; siehe [Agent würde mit null Werkzeugen gespawnt](/docs/de/errors#agent-would-be-spawned-with-zero-tools) für die Nachricht und wie man jeden Eintrag behebt. Vor v2.1.208 startete dieser Subagent mit keinen Werkzeugen und konnte ein leeres oder verwirrendes Ergebnis zurückgeben.

Beide Felder akzeptieren MCP-Server-Level-Muster zusätzlich zu exakten Werkzeugnamen: `mcp__<server>` oder `mcp__<server>__*` gewährt oder entfernt jedes Werkzeug vom benannten Server. In `disallowedTools` entfernt `mcp__*` auch jedes MCP-Werkzeug von jedem Server. Dieses Beispiel entfernt jedes Werkzeug vom `github` MCP-Server, während Werkzeuge von anderen Servern und die integrierten Werkzeuge in seinem Pool beibehalten werden:

```yaml theme={null}
---
name: local-only
description: Inherits every tool except those from the github MCP server
disallowedTools: mcp__github
---
```

Ein `disallowedTools`-Eintrag mit einem Spezifizierer, wie `Bash(git push *)`, entfernt immer noch das ganze Werkzeug aus dem Subagenten, nicht nur die übereinstimmenden Befehle. Um Bash zu behalten und bestimmte Befehle zu blockieren, fügen Sie eine [Bash-Ablehnungsregel](/docs/de/permissions#bash) wie `Bash(git push *)` zu `permissions.deny` in Ihren Einstellungen hinzu. Die Regel gilt für die Hauptkonversation und für Subagenten.

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

Die `Agent(agent_type)`-Allowlist-Syntax gilt nur für einen Agent, der als Hauptthread mit `claude --agent` läuft. In einer Subagenten-Definition ermöglicht das Auflisten von `Agent` in `tools` diesem Subagenten, Subagenten zu spawnen, während die [Tiefengrenze](#let-subagents-spawn-their-own-subagents) es zulässt, aber jede Typenliste in den Klammern wird ignoriert.

<h4 id="scope-mcp-servers-to-a-subagent">
  Umfang von MCP-Servern auf einen Subagenten
</h4>

Verwenden Sie das `mcpServers`-Feld, um einem Subagenten Zugriff auf [MCP](/docs/de/mcp)-Server zu geben, die in der Hauptkonversation nicht verfügbar sind. Inline-Server, die hier definiert sind, werden verbunden, wenn der Subagent startet, unter Beachtung der [Vertrauensregel für den Ordner der Agent-Datei](#inline-server-trust), und getrennt, wenn er endet. String-Verweise teilen die Verbindung der übergeordneten Sitzung.

<Note>
  Das `mcpServers`-Feld gilt in beiden Kontexten, in denen eine Agent-Datei ausgeführt werden kann:

  * Als Subagent, gespawnt durch das Agent-Werkzeug oder eine @-Erwähnung
  * Als Hauptsitzung, gestartet mit [`--agent`](#invoke-subagents-explicitly) oder der `agent`-Einstellung

  Wenn der Agent die Hauptsitzung ist, verbinden sich Inline-Server-Definitionen beim Start zusammen mit Servern aus [`.mcp.json`](/docs/de/mcp) und Einstellungsdateien, unter der gleichen [Vertrauensregel für den Ordner der Agent-Datei](#inline-server-trust). In `/mcp` kann ein Remote-Server (HTTP oder SSE), den Sie zuvor verwendet haben, den [`cached`-Status](/docs/de/mcp#managing-your-servers) statt anzeigen; Claude Code verbindet ihn, wenn Claude zuerst eines seiner Werkzeuge aufruft.
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

Inline-Definitionen verwenden das gleiche Schema wie `.mcp.json`-Server-Einträge, mit dem Servernamen als Schlüssel, und unterstützen die Typen `stdio`, `http`, `sse` und `ws`.

Um einen MCP-Server vollständig aus der Hauptkonversation herauszuhalten und zu vermeiden, dass seine Werkzeugbeschreibungen dort Kontext verbrauchen, definieren Sie ihn inline hier statt in `.mcp.json`. Der Subagent erhält die Werkzeuge; die übergeordnete Konversation nicht.

<span id="inline-server-trust" />Claude Code lädt einen Inline-Server aus einer Agent-Datei in Ihrem Projekt-`.claude/agents/`-Verzeichnis oder in einem `--add-dir`-Verzeichnis-`.claude/agents/` nur, nachdem Sie [dem Ordner vertrauen, aus dem die Agent-Datei stammt](/docs/de/permissions#what-runs-before-you-trust-a-folder). Vor v2.1.238 lud Claude Code diese Server ohne Vertrauensprüfung.

* **Vertrauen, das nicht zählt**: Vertrauen eines übergeordneten Ordners und das automatische Vertrauen, das eine `-p`- oder SDK-Sitzung für [Hooks in Einstellungsdateien](/docs/de/permissions#what-runs-before-you-trust-a-folder) erhält
* **Bis dahin**: Claude Code überspringt jeden Inline-Server in dieser Agent-Datei und schreibt den genauen `projects["<path>"].hasTrustDialogAccepted`-Schlüssel für `~/.claude.json` in das Debug-Protokoll
* **`--add-dir`-Verzeichnisse**: Ein Verzeichnis außerhalb des Repositorys Ihres vertrauenswürdigen Arbeitsbereichs benötigt seinen eigenen Vertrauenseintrag, da seine `.claude/agents/`-Dateien das Vertrauen Ihres Arbeitsbereichs nicht erben

Claude Code lädt zwei Arten von Servern ohne Vertrauensprüfung für den Ordner, aus dem die Agent-Datei stammt:

* Ein Name, der auf einen bereits konfigurierten Server verweist
* Ein Inline-Server in einer Agent-Datei aus `~/.claude/agents/`, in einer, die Sie mit `--agents` oder der SDK-Option `agents` übergeben, oder in einer, die verwaltete Einstellungen liefern

Die MCP-Einschränkungen, die für die Hauptsitzung gelten, gelten auch für Server, die im Subagenten-Frontmatter deklariert sind:

* [`--strict-mcp-config`](/docs/de/cli-reference) und [`--bare`](/docs/de/cli-reference)
* [Enterprise verwaltete MCP-Konfiguration](/docs/de/managed-mcp)
* [`allowedMcpServers` und `deniedMcpServers` Richtlinien](/docs/de/managed-mcp#policy-based-control-with-allowlists-and-denylists)

Wenn einer dieser Punkte einen Server blockiert, überspringt Claude Code ihn und zeigt eine Warnung mit den Namen der blockierten Server an.

Verwaltete Einstellungseinschränkungen gelten für jeden Subagenten, unabhängig davon, wie er definiert ist. `--strict-mcp-config` filtert keine Server, die Sie inline über `--agents` oder die SDK-Option `agents` übergeben, da diese explizite Eingaben des Aufrufers sind.

<h4 id="permission-modes">
  Berechtigungsmodi
</h4>

Setzen Sie `permissionMode`, um den Berechtigungsmodus zu wählen, in dem ein Subagent läuft. Verwenden Sie die Konfigurationswerte der Modi, daher ist der Manuelle Modus `default`. Wenn Sie ihn nicht setzen, erbt der Subagent den Modus der Hauptkonversation, der als [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) auf Pro-, Max- und Team-Plänen beginnt, es sei denn, Ihre Einstellungen oder Ihre Organisation ändern ihn.

Die Hauptkonversation's Berechtigungsmodus entscheidet, ob Claude Code den Wert verwendet, den Sie setzen:

* Wenn die Hauptkonversation in `bypassPermissions`, `acceptEdits` oder [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) ist, läuft der Subagent in diesem gleichen Modus und Claude Code ignoriert den `permissionMode`, den Sie setzen. Unter Auto-Modus bewertet der Klassifizierer die Werkzeugaufrufe des Subagenten mit den Block- und Zulassungsregeln der Hauptkonversation. Wenn der Subagent endet, überprüft der Klassifizierer auch seine Arbeit und seinen endgültigen Bericht, bevor der Bericht geliefert wird, wie [Wie Auto-Modus Subagenten handhabt](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) beschreibt.
* Wenn die Hauptkonversation in `default`, `dontAsk` oder `plan` Modus ist, läuft der Subagent in dem Berechtigungsmodus, den Sie setzen, außer `bypassPermissions`. Ein Subagent, der `bypassPermissions` deklariert, behält stattdessen den Modus der Hauptkonversation. Die `bypassPermissions`-Ausnahme erfordert Claude Code v2.1.267 oder später.

`permissionMode` akzeptiert diese Werte und `manual` als Alias für `default`:

| Modus               | Verhalten                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `default`           | Manueller Modus: fordert Berechtigung an                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `acceptEdits`       | Auto-Akzeptanz von Dateibearbeitungen und häufigen Dateisystem-Befehlen für Pfade im Arbeitsverzeichnis oder `additionalDirectories`                                                                                                                                                                                                                                                                                                                                                  |
| `auto`              | [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode): ein Hintergrund-Klassifizierer überprüft Befehle und Schreibvorgänge in geschützten Verzeichnissen                                                                                                                                                                                                                                                                                                               |
| `dontAsk`           | Auto-Ablehnung von Berechtigungsaufforderungen. Explizit zulässige Werkzeuge funktionieren weiterhin; `AskUserQuestion`, MCP-Werkzeuge, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, und Connector-Werkzeuge [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) in Sitzungen, wo diese Einstellung Claude Code erreicht, werden verweigert, auch wenn Sie sie zugelassen haben |
| `bypassPermissions` | [Alle Berechtigungsprüfungen überspringen](/docs/de/permission-modes#skip-all-checks-with-bypasspermissions-mode). Ein Subagent läuft in diesem Modus nur, wenn die Hauptkonversation es tut                                                                                                                                                                                                                                                                                               |
| `plan`              | Plan-Modus (schreibgeschützte Exploration)                                                                                                                                                                                                                                                                                                                                                                                                                                            |

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

Sie können keine Skills vorausladen, die [`disable-model-invocation: true`](/docs/de/skills#control-who-invokes-a-skill) setzen, da das Vorausladen aus dem gleichen Satz von Skills stammt, die Claude aufrufen kann. Dies schließt den gebündelten `/verify`-Skill ein: Nur Sie können ihn ausführen, daher kann er auch nicht vorausgeladen werden.

Wenn ein aufgelisteter Skill fehlt oder deaktiviert ist, beispielsweise durch die Richtlinie Ihrer Organisation, überspringt Claude Code ihn und protokolliert eine Warnung im Debug-Protokoll.

<Note>
  Dies ist das Gegenteil von [Ausführen eines Skills in einem Subagenten](/docs/de/skills#run-skills-in-a-subagent). Mit `skills` in einem Subagenten kontrolliert der Subagent den Systemprompt und lädt Skill-Inhalte. Mit `context: fork` in einem Skill wird der Skill-Inhalt in den von Ihnen angegebenen Agent eingespritzt. In beiden Fällen startet der Subagent ohne Ihre Konversationshistorie.
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

Subagenten-Speicher ist Teil des [Auto-Speichers](/docs/de/memory#auto-memory): Wenn Sie Auto-Speicher mit der `autoMemoryEnabled`-Einstellung oder `CLAUDE_CODE_DISABLE_AUTO_MEMORY` ausschalten, hat das `memory`-Feld keine Auswirkung und der Subagent startet ohne die Speicheranweisungen oder den Speicher-Werkzeugzugriff, der unten beschrieben ist.

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

Auf macOS und Linux machen Sie das Skript ausführbar, oder der Hook schlägt fehl, anstatt etwas zu blockieren:

```bash theme={null}
chmod +x ./scripts/validate-readonly-query.sh
```

Um die Regel zu testen, bitten Sie den Subagenten, eine `UPDATE`-Anweisung auszuführen: Das Skript beendet sich mit Code 2, Claude Code blockiert den Befehl, und der Subagent sieht die Nachricht `Blocked: Only SELECT queries are allowed`.

Siehe [Hook-Eingabe](/docs/de/hooks#pretooluse-input) für das vollständige Eingabeschema und [Exit-Codes](/docs/de/hooks#exit-code-output) für die Auswirkungen von Exit-Codes auf das Verhalten. Unter Windows schreiben Sie Hook-Skripte in PowerShell und fügen Sie `shell: powershell` zum Hook-Eintrag hinzu, wie in [Ausführen von Hooks in PowerShell](/docs/de/hooks#windows-powershell-tool) gezeigt.

<h4 id="disable-specific-subagents">
  Deaktivieren Sie spezifische Subagenten
</h4>

Sie können verhindern, dass Claude bestimmte Subagenten verwendet, indem Sie sie zum `deny`-Array in Ihren [Einstellungen](/docs/de/settings-reference#permission-settings) hinzufügen. Verwenden Sie das Format `Agent(subagent-name)`, wobei `subagent-name` dem `name`-Feld des Subagenten entspricht.

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
* **In `settings.json`**: Definieren Sie Hooks auf Sitzungsebene, die auch in Subagenten ausgelöst werden. Tool-Ereignisse wie `PreToolUse` und `PostToolUse` werden für die Werkzeugaufrufe des Subagenten auf die gleiche Weise ausgelöst wie in der Hauptkonversation, und `SubagentStart` und `SubagentStop` werden ausgelöst, wenn ein Subagent startet oder endet

Hooks aus [Einstellungsdateien, verwalteten Richtlinieneinstellungen und Plugins](/docs/de/hooks#hook-locations) gelten alle in Subagenten, daher wird ein `PreToolUse`-Hook in `settings.json` auch vor jedem Werkzeug ausgeführt, das ein Subagent verwendet.

<h4 id="hooks-in-subagent-frontmatter">
  Hooks im Subagenten-Frontmatter
</h4>

Definieren Sie Hooks direkt in der Markdown-Datei des Subagenten. Diese Hooks werden nur ausgeführt, während dieser spezifische Subagent aktiv ist, und werden bereinigt, wenn er endet.

<Note>
  Frontmatter-Hooks werden ausgelöst, wenn der Agent als Subagent durch das Agent-Werkzeug oder eine @-Erwähnung gespawnt wird, und wenn der Agent als Hauptsitzung über [`--agent`](#invoke-subagents-explicitly) oder die `agent`-Einstellung läuft. Im Hauptsitzungs-Fall werden sie zusammen mit allen Hooks ausgeführt, die in [`settings.json`](/docs/de/hooks) definiert sind.
</Note>

Um Frontmatter-Hooks eines Subagenten auf Projektebene auszuführen, akzeptieren Sie den [Arbeitsbereichs-Vertrauensdialog](/docs/de/permissions#project-allow-rules-and-workspace-trust) für den Ordner, der die Agent-Datei enthält. Hooks von Benutzer-Subagenten in `~/.claude/agents/` und von Definitionen, die Sie mit `--agents` übergeben, werden ohne diesen Schritt ausgeführt. Wenn Sie einen Ordner mit `--add-dir` von außerhalb des Repositorys Ihres vertrauenswürdigen Arbeitsbereichs hinzugefügt haben, vertrauen Sie diesem Ordner separat: Seine `.claude/agents/`-Hooks erben das Vertrauen Ihres Arbeitsbereichs nicht.

Bis Sie dem Ordner vertrauen, wird der Subagent immer noch ausgeführt, aber Claude Code überspringt seine Frontmatter-Hooks und protokolliert einen Fehler im Debug-Protokoll, der erklärt, wie man dem Ordner vertraut. Dies ist eine strengere Regel als die für Hooks in Einstellungsdateien: Das Vertrauen eines übergeordneten Ordners ist nicht ausreichend, und eine `-p`-Sitzung zählt nicht als vertrauenswürdig. [Was vor dem Vertrauen eines Ordners ausgeführt wird](/docs/de/permissions#what-runs-before-you-trust-a-folder) vergleicht die beiden. Vor v2.1.218 konnten Frontmatter-Hooks aus Ordnern ausgeführt werden, denen Sie nicht vertraut haben, auch in nicht-interaktiven Sitzungen.

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

Beide Ereignisse unterstützen Matcher, um bestimmte Agent-Typen nach Name zu adressieren. Der Matcher-Wert ist der `name` des Frontmatters des Agenten für Projekt- und Benutzer-Subagenten oder der Plugin-Umfang-Identifier wie `my-plugin:db-agent` für [Plugin-Subagenten](/docs/de/plugins/components#agents). Ein Umfang-Name enthält einen Doppelpunkt, daher wird er als [unverankerte reguläre Ausdrücke](/docs/de/hooks#matcher-patterns) ausgewertet; verankern Sie ihn mit `^` und `$`, wie in `^my-plugin:db-agent$`, um nur diesen Agent zu treffen.

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
  Mit Subagenten arbeiten
</h2>

<h3 id="understand-automatic-delegation">
  Automatische Delegierung verstehen
</h3>

Claude delegiert Aufgaben automatisch basierend auf der Aufgabenbeschreibung in Ihrer Anfrage, dem Feld `description` in Subagenten-Konfigurationen und dem aktuellen Kontext. Um proaktive Delegierung zu fördern, fügen Sie Phrasen wie „use proactively" in das Beschreibungsfeld Ihres Subagenten ein.

Halten Sie Beschreibungen kurz: Claude Code zeigt eine Startwarnmeldung an, wenn die kombinierten Beschreibungen Ihrer Subagenten das [Limit von 15.000 Token](/docs/de/errors#agent-descriptions-are-over-the-15000-token-limit) überschreiten, und lädt dennoch jeden Subagenten.

Wenn der Subagent in einem [Plugin](/docs/de/plugins/overview) enthalten ist, können Sie messen, wie zuverlässig Claude ihn über realistische Eingabeaufforderungen hinweg delegiert, anstatt sie einzeln zu überprüfen: [`claude plugin eval`](/docs/de/plugin-evals) führt jede Eingabeaufforderung mit und ohne das Plugin aus und bewertet die Ergebnisse.

<h3 id="invoke-subagents-explicitly">
  Subagenten explizit aufrufen
</h3>

Wenn automatische Delegierung nicht ausreicht, können Sie einen Subagenten selbst anfordern. Drei Muster eskalieren von einem einmaligen Vorschlag zu einem sitzungsweiten Standard:

* **Natürliche Sprache**: Nennen Sie den Subagenten in Ihrer Eingabeaufforderung; Claude entscheidet, ob delegiert werden soll
* **@-Erwähnung**: Garantiert, dass der Subagent für eine Aufgabe ausgeführt wird
* **Sitzungsweit**: Die gesamte Sitzung verwendet die Systemeingabeaufforderung, Werkzeugbeschränkungen und das Modell dieses Subagenten über das Flag `--agent` oder die Einstellung `agent`

Für natürliche Sprache gibt es keine spezielle Syntax. Nennen Sie den Subagenten und Claude delegiert normalerweise:

```text wrap theme={null}
Use the test-runner subagent to fix failing tests
Have the code-reviewer subagent look at my recent changes
```

**Erwähnen Sie den Subagenten mit @.** Geben Sie `@` ein und wählen Sie den Subagenten aus der Typvorhersage aus, genauso wie Sie Dateien mit @ erwähnen. Dies stellt sicher, dass dieser spezifische Subagent ausgeführt wird, anstatt die Wahl Claude zu überlassen:

```text wrap theme={null}
@"code-reviewer (agent)" look at the auth changes
```

Ihre vollständige Nachricht geht immer noch an Claude, der die Aufgabeneingabeaufforderung des Subagenten basierend auf Ihrer Anfrage schreibt. Die @-Erwähnung steuert, welcher Subagent Claude aufruft, nicht welche Eingabeaufforderung er erhält.

Subagenten, die von einem aktivierten [Plugin](/docs/de/plugins/overview) bereitgestellt werden, erscheinen in der Typvorhersage unter ihrem Bereichsnamen, z. B. `my-plugin:code-reviewer` oder `my-plugin:review:security`, wenn das Plugin [Agenten in Unterordnern organisiert](#choose-the-subagent-scope). Benannte Hintergrund-Subagenten, die derzeit in der Sitzung ausgeführt werden, erscheinen auch in der Typvorhersage und zeigen ihren Status neben dem Namen an.

Sie können die Erwähnung auch manuell eingeben, ohne die Auswahl zu verwenden: `@agent-<name>` für lokale Subagenten oder `@agent-` gefolgt vom Bereichsnamen für Plugin-Subagenten, z. B. `@agent-my-plugin:code-reviewer`. Während Sie dieses Formular eingeben, zeigt die Typvorhersage Dateienübereinstimmungen anstelle von Agenten. Die Agent-Erwähnung wird trotzdem aufgelöst, wenn Sie absenden.

**Führen Sie die gesamte Sitzung als Subagent aus.** Übergeben Sie [`--agent <name>`](/docs/de/cli-reference), um eine Sitzung zu starten, in der der Hauptthread selbst die Systemeingabeaufforderung, Werkzeugbeschränkungen und das Modell dieses Subagenten übernimmt:

```bash theme={null}
claude --agent code-reviewer
```

Die Systemeingabeaufforderung des Subagenten ersetzt die Standard-Claude-Code-Systemeingabeaufforderung vollständig, genauso wie [`--system-prompt`](/docs/de/cli-reference) es tut. `CLAUDE.md`-Dateien und Projektgedächtnis werden immer noch durch den normalen Nachrichtenfluss geladen, auch wenn die Agent-Definition [`omitClaudeMd`](#supported-frontmatter-fields) setzt.

Der Agent-Name erscheint als `@<name>` in der Startkopfzeile, damit Sie bestätigen können, dass er aktiv ist.

Dies funktioniert mit integrierten und benutzerdefinierten Subagenten, und die Wahl bleibt bestehen, wenn Sie die Sitzung fortsetzen: Claude Code stellt die Werkzeugbeschränkungen und das Modell des Agenten zusammen mit der Konversation wieder her. Wenn der Agent nicht mehr vorhanden ist, wenn Sie fortsetzen, wird die Sitzung mit den Standardwerkzeugen fortgesetzt und zeigt eine [Warnung mit dem Namen des Agenten](/docs/de/errors#session-agent-no-longer-available). Für die Systemeingabeaufforderung in beiden Fällen siehe [Systemeingabeaufforderungs-Flags in fortgesetzten Konversationen](/docs/de/cli-reference#system-prompt-flags-in-resumed-conversations).

Für einen von einem Plugin bereitgestellten Subagenten können Sie nur den Agent-Namen übergeben und Claude Code findet ihn:

```bash theme={null}
claude --agent security-reviewer
```

Wenn mehrere Plugins Agenten mit demselben Namen bereitstellen, übergeben Sie den Bereichsnamen zur Disambiguierung:

```bash theme={null}
claude --agent my-plugin:security-reviewer
```

Wenn das Plugin den Agenten in einem Unterordner seines `agents/`-Verzeichnisses platziert, fügen Sie den Unterordner in den Bereichsnamen ein, z. B. `claude --agent my-plugin:review:security`.

Um es zum Standard für jede Sitzung in einem Projekt zu machen, setzen Sie `agent` in `.claude/settings.json`:

```json theme={null}
{
  "agent": "code-reviewer"
}
```

Das CLI-Flag überschreibt die Einstellung, wenn beide vorhanden sind.

<h3 id="run-subagents-in-foreground-or-background">
  Subagenten im Vordergrund oder Hintergrund ausführen
</h3>

Subagenten können im Vordergrund oder im Hintergrund ausgeführt werden:

* **Vordergrund-Subagenten** blockieren die Hauptkonversation bis zur Fertigstellung. Genehmigungseingabeaufforderungen werden an Sie weitergeleitet, wenn sie auftauchen.
* **Hintergrund-Subagenten** werden gleichzeitig ausgeführt, während Sie weiterarbeiten. Wenn ein Hintergrund-Subagent einen Werkzeugaufruf erreicht, der eine Genehmigung benötigt, zeigt Claude Code die Eingabeaufforderung in Ihrer Hauptsitzung an und nennt den Subagenten, der fragt. Genehmigen Sie, um den Subagenten fortsetzen zu lassen, oder drücken Sie Esc, um diesen einen Werkzeugaufruf zu verweigern, ohne den Subagenten zu stoppen.

Für jeden Subagenten, den Claude mit dem Agent-Werkzeug spawnt, wählt Claude Code Vordergrund oder Hintergrund aus dem ersten dieser Fälle, der zutrifft:

* Wenn ein In-Process-[Agent-Team](/docs/de/agent-teams#limitations)-Teamkollege den Subagenten spawnte, führt Claude Code ihn im Vordergrund aus. Claude Code weigert sich mit einem Fehler, einen Teamkollegen-Subagenten zu spawnen, dessen Definition [`background: true`](#supported-frontmatter-fields) setzt. Wenn [Fork-Modus](#turn-fork-mode-on-or-off) aus ist und Sie [Hintergrund-Aufgaben nicht ausgeschaltet haben](/docs/de/env-vars), weigert sich Claude Code auch mit einem Fehler, wenn ein Teamkollege `run_in_background: true` setzt.
* Wenn Sie [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/de/env-vars) auf `1` setzen, führt Claude Code den Subagenten im Vordergrund aus, in jeder Art von Sitzung und unabhängig davon, ob Fork-Modus an ist.
* Wenn [Fork-Modus](#turn-fork-mode-on-or-off) an ist, wie es standardmäßig in einer interaktiven Sitzung der Fall ist, führt Claude Code den Subagenten im Hintergrund aus, sowohl Fork- als auch Non-Fork-Subagenten, und Claude kann nicht den Vordergrund anfordern.
* Wenn Fork-Modus aus ist, führt Claude den Subagenten standardmäßig im Hintergrund aus und im Vordergrund, wenn er das Ergebnis benötigt, bevor er fortfährt. Fork-Modus ist aus im [nicht-interaktiven Modus](/docs/de/headless) mit `-p` und im Agent SDK, es sei denn, Sie schalten ihn ein. Um einen bestimmten Subagenten im Hintergrund zu halten, auch wenn Claude das Ergebnis möchte, setzen Sie sein Frontmatter-Feld [`background`](#supported-frontmatter-fields) auf `true`.

Für eine Skill mit `context: fork` folgt Claude Code stattdessen den Regeln in [Führen Sie Skills in einem Subagenten aus](/docs/de/skills#run-skills-in-a-subagent), unabhängig davon, ob Fork-Modus an ist.

Hintergrund-Subagenten werden mit einem [kleineren integrierten Werkzeugsatz](#available-tools) als Vordergrund-Subagenten ausgeführt, außer für Konversations-Forks und [fortgesetzte](#resume-subagents) Vordergrund-Subagenten.

Hintergrund-Subagenten zeigen jede Genehmigungseingabeaufforderung in Ihrer Hauptsitzung an. Wenn Sie eine dieser Eingabeaufforderungen mit einer Wahl beantworten, die über diesen einen Werkzeugaufruf hinausgeht, z. B. eine Genehmigung, die für den Rest der Sitzung gilt, wendet Claude Code Ihre Antwort auf die gesamte Sitzung an, einschließlich Ihrer Hauptkonversation.

Ein Hintergrund-Subagent kann einen Hintergrund-[Bash- oder PowerShell-Befehl](/docs/de/tools-reference#background-commands) [über das Ende seines Zuges hinaus laufen lassen](/docs/de/interactive-mode#how-backgrounding-works). Wenn dieser Befehl endet, sendet Claude Code dem Subagenten eine Benachrichtigung.

Die Ergebnisse eines Hintergrund-Subagenten erreichen Claude als Abschlussbenachrichtigung in einem späteren Zug. Claude wartet auf diese Benachrichtigung, bevor er die Ergebnisse des Subagenten meldet, und wenn Sie zuerst nach Fortschritt fragen, meldet er, dass der Subagent noch läuft. Vor v2.1.211 meldete Claude manchmal Ergebnisse für einen Hintergrund-Subagenten, der nicht fertig war.

Sie können dies auch selbst steuern:

* Wenn Fork-Modus aus ist, bitten Sie Claude, eine Aufgabe im Hintergrund oder im Vordergrund auszuführen
* Drücken Sie **Strg+B**, um eine laufende Aufgabe in den Hintergrund zu verschieben

Claude Code löscht die Zeile eines Hintergrund-Subagenten aus dem Subagenten-Panel unter der Eingabeaufforderungseingabe auf eine von zwei Arten, je nachdem, wie der Subagent endete:

* Wenn ein Subagent erfolgreich beendet wird, entfernt Claude Code seine Zeile sofort und zeigt außer im [Bildschirmlesemodus](/docs/de/accessibility) `/tasks to see subagents` in der Fußzeile für 30 Sekunden an. Führen Sie während dieser 30 Sekunden [`/tasks`](/docs/de/commands) aus und drücken Sie `Enter` auf dem Subagenten, um sein Transkript zu öffnen. Vor v2.1.232 behielt Claude Code die Zeile 30 Sekunden nach Beendigung des Subagenten, genauso wie eine fehlgeschlagene, und zeigte keinen Fußzeilentipp.
* Wenn ein Subagent fehlschlägt oder Sie ihn stoppen, behält Claude Code seine Zeile 30 Sekunden lang. Um die Zeile schneller zu löschen, wählen Sie sie aus und drücken Sie `x`.

Ein Hintergrund-Subagent, der abgeschlossen ist, bleibt in [`/tasks`](/docs/de/commands) aufgelistet, als erledigt markiert und unter laufender Arbeit sortiert, für die gleichen 30 Sekunden wie der Fußzeilentipp. Seine Detailansicht bleibt offen, wenn der Subagent beendet wird. Subagenten, die fehlschlagen oder die Sie stoppen, verlassen die Liste. Vor v2.1.208 verließ ein abgeschlossener Subagent die Liste in dem Moment, in dem er beendet wurde, und seine Detailansicht schloss sich.

<h3 id="subagent-names">
  Subagenten-Namen
</h3>

Claude kann einem Subagenten einen Namen geben, indem er einen `name`-Parameter beim Agent-Werkzeugaufruf übergibt, und kann dies auch von selbst tun, ohne Sie zuerst zu fragen. Der Name macht den Subagenten adressierbar: Claude kann ihn [nach Abschluss nach Name ansprechen oder fortsetzen](#resume-subagents).

In einer interaktiven Sitzung mit aktivierten [Agent-Teams](/docs/de/agent-teams) wird ein Subagent, den Claude aus der Hauptkonversation mit einem `name` spawnt, stattdessen als Teamkollege gestartet, es sei denn, der Aufruf ist ein [Fork](#fork-the-current-conversation) oder übergibt `isolation` beim Aufruf selbst. Ein `isolation`-Wert in der Frontmatter des Subagenten verhindert es nicht, und der Teamkollege wird dann im Arbeitsverzeichnis der Hauptsitzung ausgeführt. Siehe [Wie Claude Agent-Teams startet](/docs/de/agent-teams#how-claude-starts-agent-teams).

<h3 id="api-errors-in-subagents">
  API-Fehler in Subagenten
</h3>

Wenn etwas [die Antwort eines Subagenten mitten im Stream unterbricht](/docs/de/errors#the-response-above-may-be-incomplete), und die teilweise Antwort Text enthält, aber keine Werkzeugaufrufe, fordert Claude Code den Subagenten auf, fortzufahren, anstatt den Lauf zu beenden. Dies geschieht auch in interaktiven Sitzungen. Der Lauf endet beim Fehler nur, wenn diese Fortsetzungen aufgebraucht sind.

Ab v2.1.199 meldet ein Subagent, dessen Lauf bei einem API-Fehler endet, z. B. bei einem Nutzungslimit oder wiederholtem Serverfehler, diesen Fehler an Claude zurück, anstatt den Fehlertext so zurückzugeben, als wären es die Erkenntnisse des Subagenten. Was Claude erhält, hängt davon ab, wo der Subagent lief:

* **Vordergrund**: Wenn ein Ratenlimit, eine Überlastung oder ein Serverfehler einen Subagenten unterbricht, der bereits Textausgabe produziert hat, gibt das Agent-Werkzeug diese teilweise Ausgabe mit einer Notiz zurück, dass der Subagent unterbrochen wurde und seine Aufgabe nicht abgeschlossen hat. Ein Subagent, der nichts produziert hat oder dessen einzige Ausgabe Werkzeugaufrufe waren, schlägt mit [`Agent terminated early due to an API error`](/docs/de/errors#agent-terminated-early-due-to-an-api-error) fehl, gefolgt von der Fehlerdetail. In v2.1.199 gab ein Ratenlimit, eine Überlastung oder ein Serverfehler, der die Form „nur Werkzeugaufrufe" unterbrach, ein leeres teilweises Ergebnis zurück, das nur die Unterbrechungsnotiz enthielt.
* **Hintergrund**: Der Subagent wird als fehlgeschlagen markiert, und die Nachricht, die Claude erhält, wenn er endet, nennt den API-Fehler und enthält die letzte Ausgabe des Subagenten, sodass teilweise Arbeit nicht verloren geht.

Wenn Sie eine [Fallback-Modellkette](/docs/de/model-config#fallback-model-chains) konfigurieren und ein Subagent auf einen Fehler trifft, den die Kette abdeckt, z. B. dass sein Modell nicht verfügbar ist, wechselt Claude Code den Subagenten zum ersten Modell in der Kette, das die Anfrage akzeptiert. Der Subagent arbeitet weiter, anstatt beim Fehler zu enden.

Sobald der zugrunde liegende API-Fehler behoben ist, bitten Sie Claude, die Aufgabe zu wiederholen oder [den Subagenten fortzusetzen](#resume-subagents).

<h3 id="subagent-output-scanning">
  Subagenten-Ausgabe-Scanning
</h3>

Claude Code scannt den endgültigen Bericht jedes Subagenten, bevor Claude ihn liest. Ein Subagent kann Dateien, Webseiten oder Befehlsausgaben gelesen haben, die Sie nie überprüft haben, und Text aus diesen Quellen kann Anweisungen enthalten, die auf die Hauptkonversation abzielen. Der Scan entfernt oder umformuliert niemals etwas; er nimmt zwei Arten von Änderungen vor, die Sie möglicherweise in einem Bericht bemerken:

* **Backslash-Einfügung**: Der Scan fügt einen Backslash in Text ein, der Claude Codes eigene Ausgabe imitiert, z. B. ein `<system-reminder>`-Tag oder eine Zeile, die mit `Human:` oder `Assistant:` beginnt, damit die Imitation als gewöhnlicher Text gelesen wird, anstatt als Teil der Konversation verwechselt zu werden.
* **Marker-Zeile**: Der Scan stellt eine Zeile voran, die mit `[harness: subagent output matched instruction-shaped pattern(s):` beginnt, wenn der Bericht ein Tag wie `<system-reminder>` imitiert oder Genehmigungseinstellungen wie `bypassPermissions` oder `--dangerously-skip-permissions` erwähnt. Genehmigungseinstellungs-Erwähnungen erhalten die Marker-Zeile, aber der Text selbst bleibt wie geschrieben.

Der Scan beurteilt nicht, ob Inhalte bösartig sind, und er ändert nicht, was eine Anweisung in einem Bericht tun kann: Ein Werkzeugaufruf, zu dem der Bericht Claude führt, durchläuft immer noch die [Genehmigungsprüfungen](/docs/de/permissions) und [Sandboxing](/docs/de/sandboxing) der Sitzung. Es ist kein Ersatz für [Einschränkung, was ein Subagent erreichen kann](#control-subagent-capabilities).

Ein Bericht, der zu Claude als Ergebnis des Subagenten zurückkehrt, kommt auch unter einer Kopfzeile an, die ihn als Subagenten-Ausgabe kennzeichnet. Die Kopfzeile besagt, dass Anweisungen oder Genehmigungsansprüche im Bericht die Worte des Subagenten sind und keine Autorität von Ihnen tragen.

Ein [Hintergrund-Subagenten-Bericht](#run-subagents-in-foreground-or-background) kommt in einer Abschlussbenachrichtigung an, die als automatisiertes Ereignis gekennzeichnet ist, anstatt als Nachricht von Ihnen.

<Note>
  Das Subagenten-Ausgabe-Scanning erfordert Claude Code v2.1.210 oder später.
</Note>

<h3 id="common-patterns">
  Häufige Muster
</h3>

<h4 id="isolate-high-volume-operations">
  Isolieren Sie hochvolumige Operationen
</h4>

Eine der effektivsten Verwendungen für Subagenten ist die Isolierung von Operationen, die große Mengen an Ausgabe produzieren. Das Ausführen von Tests, das Abrufen von Dokumentation oder das Verarbeiten von Protokolldateien kann erheblichen Kontext verbrauchen. Durch die Delegierung dieser an einen Subagenten bleibt die ausführliche Ausgabe im Kontext des Subagenten, während nur die relevante Zusammenfassung zu Ihrer Hauptkonversation zurückkehrt.

```text wrap theme={null}
Use a subagent to run the test suite and report only the failing tests with their error messages
```

<h4 id="run-parallel-research">
  Führen Sie parallele Recherchen durch
</h4>

Für unabhängige Untersuchungen spawnen Sie mehrere Subagenten, um gleichzeitig zu arbeiten:

```text wrap theme={null}
Research the authentication, database, and API modules in parallel using separate subagents
```

Jeder Subagent erkundet seinen Bereich unabhängig, dann synthetisiert Claude die Erkenntnisse. Dies funktioniert am besten, wenn die Forschungspfade nicht voneinander abhängen.

<Warning>
  Wenn Subagenten abgeschlossen sind, kehren ihre Ergebnisse zu Ihrer Hauptkonversation zurück. Das Ausführen vieler Subagenten, die jeweils detaillierte Ergebnisse zurückgeben, kann erheblichen Kontext verbrauchen.
</Warning>

Für Arbeit, die parallel weiterläuft oder nicht in ein Kontextfenster passt, führen Sie sie in [separaten Sitzungen](/docs/de/agents) aus und lassen Sie Claude [Erkenntnisse zwischen ihnen weitergeben](/docs/de/cross-session-messaging).

<h4 id="chain-subagents">
  Verketten Sie Subagenten
</h4>

Für mehrstufige Workflows bitten Sie Claude, Subagenten nacheinander zu verwenden. Jeder Subagent schließt seine Aufgabe ab und gibt Ergebnisse an Claude zurück, der dann relevanten Kontext an den nächsten Subagenten übergibt.

```text wrap theme={null}
Use the code-reviewer subagent to find performance issues, then use the optimizer subagent to fix them
```

<h3 id="choose-between-subagents-and-main-conversation">
  Wählen Sie zwischen Subagenten und Hauptkonversation
</h3>

Verwenden Sie die **Hauptkonversation**, wenn:

* Die Aufgabe häufiges Hin und Her oder iterative Verfeinerung benötigt
* Mehrere Phasen teilen erheblichen Kontext, z. B. Planung, Implementierung und Tests
* Sie eine schnelle, gezielte Änderung vornehmen
* Latenz wichtig ist. Ein Subagent, der kein [Fork](#fork-the-current-conversation) ist, startet von vorne und benötigt möglicherweise Zeit, um Kontext zu sammeln

Verwenden Sie **Subagenten**, wenn:

* Die Aufgabe ausführliche Ausgabe produziert, die Sie nicht in Ihrem Hauptkontext benötigen
* Sie spezifische Werkzeugbeschränkungen oder Genehmigungen erzwingen möchten
* Die Arbeit in sich geschlossen ist und eine Zusammenfassung zurückgeben kann

Erwägen Sie stattdessen [Skills](/docs/de/skills), wenn Sie wiederverwendbare Eingabeaufforderungen oder Workflows möchten, die im Hauptkonversationskontext ausgeführt werden, anstatt in isoliertem Subagenten-Kontext.

Für eine Frage zu etwas, das bereits in Ihrer Konversation vorhanden ist, verwenden Sie [`/btw`](/docs/de/interactive-mode#side-questions-with-%2Fbtw) anstelle eines Subagenten. Es sieht Ihren vollständigen Kontext, hat aber keinen Werkzeugzugriff, und die Antwort wird nicht zur Historie hinzugefügt.

<h3 id="let-subagents-spawn-their-own-subagents">
  Lassen Sie Subagenten ihre eigenen Subagenten spawnen
</h3>

Standardmäßig kann ein Subagent seine eigenen Subagenten spawnen, bis zu drei Ebenen unter der Hauptkonversation. Bei der Tiefengrenze entzieht Claude Code das `Agent`-Werkzeug jedem Subagenten außer einem [Fork](#fork-the-current-conversation), sodass ein Subagent bei der Grenze seine delegierte Arbeit selbst erledigt und eine Zusammenfassung zurückgibt. Ein Fork bei der Grenze behält `Agent` in seiner geerbten Werkzeugliste, aber das Werkzeug gibt stattdessen einen Fehler zurück.

Verschachtelte Subagenten eignen sich für eine delegierte Aufgabe, die sich selbst in parallele Unteraufgaben aufteilt, z. B. ein Reviewer-Subagent, der einen Verifizierer pro Befund versendet. In einer interaktiven Sitzung kehrt nur die Zusammenfassung des Top-Level-Subagenten zu Ihnen zurück und die Zwischenausgabe bleibt aus Ihrer Hauptkonversation: Ein Subagent, der Hintergrund-Subagenten startet, wartet auf deren Ergebnisse, bevor er beendet wird. Im [nicht-interaktiven Modus](/docs/de/headless) und dem Agent SDK wartet der startende Subagent nicht, sodass ein verschachtelter Hintergrund-Subagent, der beendet wird, nachdem sein Starter beendet wurde, stattdessen an Ihre Hauptkonversation meldet.

Um die Grenze zu ändern, setzen Sie [`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`](/docs/de/env-vars) auf die Anzahl der Subagenten-Ebenen, die Sie unter Ihrer Hauptkonversation möchten. Beispielsweise begrenzt dieser Eintrag in [`settings.json`](/docs/de/settings) die Verschachtelung auf zwei Ebenen:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH": "2"
  }
}
```

Mit diesem Wert können Ihre Subagenten an eine zweite Ebene ihrer eigenen delegieren, und diese zweite Ebene kann nicht weiter delegieren. Setzen Sie `1`, um Verschachtelung auszuschalten.

Ein verschachtelter Subagent wird genauso konfiguriert wie ein Top-Level-Subagent und wird aus denselben [Bereichen](#choose-the-subagent-scope) aufgelöst. Um zu verhindern, dass ein Subagent spawnt, während Verschachtelung an ist, z. B. ein Reviewer, der schreibgeschützt bleiben sollte, lassen Sie `Agent` aus seiner [`tools`](#available-tools)-Liste weg oder fügen Sie es zu `disallowedTools` hinzu.

Claude Code zeigt verschachtelte Subagenten als Baum im Subagenten-Panel unter der Eingabeaufforderungseingabe an und markiert jede Zeile, die noch Nachkommen hat, mit einer `(+N)`-Anzahl von ihnen. Öffnen Sie eine Zeile, um die Geschwister und direkten Kinder dieses Subagenten mit einem Pfad zurück zu `main` zu sehen.

<Note>
  Frühere Versionen verwendeten unterschiedliche Standards:

  * **v2.1.172 bis v2.1.216**: Subagenten konnten standardmäßig verschachtelt werden, bis zu fünf Ebenen tief, und die Grenze konnte nicht geändert werden.
  * **v2.1.217 bis v2.1.218**: Die Grenze betrug standardmäßig eins, sodass ein Subagent nicht seine eigenen spawnen konnte, es sei denn, Sie erhöhten sie; v2.1.219 erhöhte den Standard auf drei.
</Note>

<h3 id="concurrent-subagent-limit">
  Gleichzeitiges Subagenten-Limit
</h3>

Zwei Grenzen steuern die Subagenten-Nutzung, jede mit ihrer eigenen Variablen: Diese stoppt Claude daran, mehr Subagenten zu spawnen, während zu viele laufen, und die [Tiefengrenze](#let-subagents-spawn-their-own-subagents) begrenzt, wie tief Subagenten verschachtelt sind. Es gibt keine Grenze für die Gesamtzahl der Subagenten, die Claude über eine Sitzung spawnen kann.

Standardmäßig schlägt das Spawnen eines anderen mit dem Agent-Werkzeug fehl, wenn 20 Subagenten in einer Sitzung laufen, mit `Concurrent subagent limit reached`, und der Fehler teilt Claude mit, nicht zu wiederholen. Das Spawnen ist erfolgreich, wenn die laufende Anzahl unter die Grenze fällt. Um die Grenze zu ändern, setzen Sie [`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`](/docs/de/env-vars) auf eine beliebige positive ganze Zahl. Sitzungen mit aktivem [ultracode](/docs/de/model-config#adjust-effort-level) sind ausgenommen: Die Grenze wird dort nicht erzwungen. Erfordert Claude Code v2.1.217 oder später.

Die Grenze blockiert nur Subagenten, die Claude mit dem Agent-Werkzeug spawnt, aber andere Läufe belegen die gleichen Slots:

* Ein In-Session-Fork, den Sie mit [`/subtask`](#fork-the-current-conversation) starten, belegt einen Slot während der Ausführung und wird niemals durch die Grenze blockiert.
* [Fortsetzen eines Subagenten](#resume-subagents), der bereits beendet ist, belegt einen neuen Slot, ohne die Grenze zu überprüfen, sodass Fortsetzungen die laufende Anzahl über die Grenze hinaus drücken können.

Agenten, die andere Funktionen ausführen, z. B. [Workflow](/docs/de/workflows)-Agenten und [Agent-Team](/docs/de/agent-teams)-Teamkollegen, folgen stattdessen ihren eigenen Grenzen.

<h3 id="manage-subagent-context">
  Verwalten Sie den Subagenten-Kontext
</h3>

<h4 id="what-loads-at-startup">
  Was beim Start geladen wird
</h4>

Jeder Subagent startet mit einem frischen, isolierten Kontextfenster. Er sieht Ihre Konversationshistorie nicht, die Skills, die Sie bereits aufgerufen haben, oder die Dateien, die Claude bereits gelesen hat. Claude verfasst eine Delegierungsnachricht, die die Aufgabe zusammenfasst, und der Subagent arbeitet von dort aus. Die Ausnahme ist ein [Fork](#fork-the-current-conversation), der die übergeordnete Konversation erbt, anstatt von vorne zu beginnen.

Der anfängliche Kontext eines Non-Fork-Subagenten enthält:

* **Systemeingabeaufforderung**: die eigene Eingabeaufforderung des Agenten plus Umgebungsdetails, die Claude Code anfügt, nicht die Claude-Code-Systemeingabeaufforderung. Benutzerdefinierte Subagenten definieren ihre im [Markdown-Body](#write-subagent-files) oder im Feld `prompt`. Integrierte Agenten haben vordefinierte Eingabeaufforderungen.
* **Aufgabennachricht**: die Delegierungseingabeaufforderung, die Claude schreibt, wenn es die Arbeit übergibt.
* **CLAUDE.md-Dateien**: jede Ebene der [CLAUDE.md-Hierarchie](/docs/de/memory#how-claude-md-files-load), die die Hauptkonversation lädt, einschließlich `~/.claude/CLAUDE.md`, Projektregeln, `CLAUDE.local.md`, verwaltete Richtliniendateien und alle [`AGENTS.md`-Dateien](/docs/de/memory#agents-md), die als Projektanweisungen geladen werden. Die integrierten Explore- und Plan-Agenten überspringen dies. Ein Subagent, dessen Definition [`omitClaudeMd`](#supported-frontmatter-fields) setzt, lädt nur die verwalteten Richtliniendateien oder gar keine, wenn die Definition aus [verwalteten Einstellungen](#choose-the-subagent-scope) stammt.
* **Git-Status**: Ein Snapshot, der zu Beginn der übergeordneten Sitzung aufgenommen wurde. Fehlt, wenn das Arbeitsverzeichnis kein Git-Repository ist oder wenn [`includeGitInstructions`](/docs/de/settings-reference#includegitinstructions) `false` ist. Explore und Plan überspringen es unabhängig davon.
* **Vorgeladene Skills**: Vollständiger Inhalt jeder Skill, die im Feld [`skills`](#preload-skills-into-subagents) des Agenten benannt ist. Integrierte Agenten laden Skills nicht vor.
* **Geschwister-Roster**: Eine Systemerinnerung, die `main` und jeden anderen benannten Agenten in der Sitzung auflistet, jeweils ein gültiger `to`-Wert für [`SendMessage`](#resume-subagents). Erfordert Claude Code v2.1.206 oder später. Das Roster erscheint nur, wenn die Tools des Subagenten `SendMessage` enthalten und mindestens ein anderer Agent einen Namen hat, ob Claude ihn beim Spawnen benannt hat oder er als [Agent-Team](/docs/de/agent-teams)-Teamkollege läuft. Es ist ein Snapshot, der aufgenommen wird, wenn der Subagent startet, sodass später benannte Agenten nicht erscheinen.

Um einen Ihrer eigenen Subagenten ohne die Benutzer-, Projekt- und lokalen CLAUDE.md-Dateien zu starten, setzen Sie [`omitClaudeMd: true`](#supported-frontmatter-fields) in seiner Frontmatter oder `--agents` JSON.

Die Hauptkonversation hat immer noch Ihre vollständige CLAUDE.md, wenn sie die Ergebnisse dieser Subagenten liest, sodass die meisten Regeln den Subagenten selbst nicht erreichen müssen. Wenn eine Regel dies muss, z. B. „ignorieren Sie das `vendor/`-Verzeichnis", wiederholen Sie sie in der Eingabeaufforderung, die Sie Claude geben, wenn Sie delegieren.

Sie können nicht ändern, welche Subagenten Git-Status erhalten. Nur Explore und Plan überspringen es.

Einige Hauptkonversationszustände erreichen niemals einen Non-Fork-Subagenten:

* **Ausgabestil**: Ein Subagent führt seine eigene Systemeingabeaufforderung aus, sodass Ihr [Ausgabestil](/docs/de/output-styles) seine Antworten nicht formt, außer in einem [Fork](#fork-the-current-conversation).
* **Auto-Gedächtnis**: Das [Auto-Gedächtnis](/docs/de/memory#auto-memory) der Hauptkonversation wird nicht geladen. Um einem Subagenten persistentes Gedächtnis seiner eigenen zu geben, verwenden Sie das Feld [`memory`](#enable-persistent-memory).
* **Kontextfenstergröße**: Das Kontextfenster eines Subagenten wird durch sein eigenes Modell dimensioniert, nicht das des übergeordneten. Die Delegierung an ein Modell mit einem kleineren Fenster gibt diesem Subagenten das kleinere Fenster.

<h4 id="resume-subagents">
  Fortsetzen Sie Subagenten
</h4>

Jeder Subagenten-Aufruf erstellt eine neue Instanz, anstatt eine frühere fortzusetzen. Um die Arbeit eines bestehenden Subagenten fortzusetzen, anstatt von vorne zu beginnen, bitten Sie Claude, ihn fortzusetzen.

Fortgesetzte Subagenten behalten ihre vollständige Konversationshistorie, einschließlich aller vorherigen Werkzeugaufrufe, Ergebnisse und Überlegungen. Wenn der Subagent [Hintergrund-Subagenten seiner eigenen](#let-subagents-spawn-their-own-subagents) spawnte, enthält diese Historie die Ergebnisse, die sie während der Ausführung lieferten. Der Subagent setzt genau dort an, wo er stoppte, anstatt von vorne zu beginnen.

* Wenn ein Subagent abgeschlossen ist, erhält Claude seine Agent-ID.
* Die integrierten Explore- und Plan-Agenten sind einmalig und geben keine Agent-ID zurück, sodass Claude sie nicht fortsetzen kann. Verwenden Sie `general-purpose` oder einen benutzerdefinierten Subagenten, wenn Sie die Arbeit fortsetzen müssen.
* Wenn ein Subagent bei seinem [`maxTurns`](#supported-frontmatter-fields)-Limit stoppt, markiert Claude Code die zurückgegebene Ausgabe als teilweise. Für Subagenten, die eine Agent-ID zurückgeben, vermerkt Claude Code auch im Ergebnis, dass Claude den Subagenten ansprechen kann, um von dort fortzufahren, wo er stoppte.

Claude verwendet das `SendMessage`-Werkzeug mit der Agent-ID oder dem Namen des Agenten als `to`-Feld, um ihn fortzusetzen. `SendMessage` erfordert nicht, dass [Agent-Teams](/docs/de/agent-teams) aktiviert sind; nur strukturierte Team-Protokoll-Nachrichten wie `shutdown_request` und `plan_approval_response` tun dies. Über Subagenten und Teamkollegen hinaus können Claude in Sitzungen, in denen Cross-Session-Messaging aktiviert ist, mit demselben Werkzeug [Ihre anderen Claude-Code-Sitzungen](/docs/de/cross-session-messaging) ansprechen, auf dieser Maschine oder [darüber hinaus](/docs/de/cross-session-messaging#message-sessions-on-other-machines).

Um einen Subagenten fortzusetzen, bitten Sie Claude, die vorherige Arbeit fortzusetzen:

```text wrap theme={null}
Use the code-reviewer subagent to review the authentication module
[Agent completes]

Continue that code review and now analyze the authorization logic
[Claude resumes the subagent with full context from previous conversation]
```

Wenn Claude einen abgeschlossenen Subagenten mit dem `SendMessage`-Werkzeug eine Nachricht sendet, wird der Subagent im Hintergrund ohne einen neuen `Agent`-Aufruf fortgesetzt. Das Gleiche gilt für einen Subagenten, den Claude mit dem `TaskStop`-Werkzeug gestoppt hat, sobald sein gestoppter Lauf beendet ist. Der fortgesetzte Lauf behält den [Werkzeugsatz von dort, wo der Subagent zuerst lief](#run-subagents-in-foreground-or-background), und kann weiterhin den [Prompt-Cache lesen, den der ursprüngliche Lauf aufgewärmt hat](/docs/de/prompt-caching#subagents-and-the-cache).

Ein Subagent, der das `SendMessage`-Werkzeug hat, kann diese Nachricht auch senden. In einer interaktiven Sitzung meldet der fortgesetzte Agent dann dem Subagenten, der ihn fortgesetzt hat, zurück, nicht zu Ihrer Hauptkonversation. Dieser Subagent wartet auf das Ergebnis, bevor er seine eigene Arbeit beendet. Wenn ein Subagent einen Agenten anspricht, dem er meldet, z. B. seinen eigenen Launcher, setzt Claude Code diesen Agenten fort, ohne seine Ergebnisse umzuleiten.

Ein Subagent, den Sie selbst gestoppt haben, mit `x` in `/tasks` oder einer SDK-`stop_task`-Anfrage, wird nicht automatisch fortgesetzt. Wenn Claude ihm eine Nachricht sendet, wird die Nachricht verweigert und Claude wird mitgeteilt, dass der Agent abgebrochen wurde.

Während [die Zeile dieses Subagenten immer noch im Subagenten-Panel ist](#run-subagents-in-foreground-or-background), geben Sie in sein Transkript ein, um ihn selbst fortzusetzen. Danach kann eine Nachricht von Claude ihn wieder automatisch fortsetzen.

Das Fortsetzen startet einen neuen Lauf des Agenten unter derselben ID, sodass ein Subagent, der bereits fehlgeschlagen oder abgeschlossen war, in der Aufgabenliste und in den Task-Events des Agent SDK wieder als laufend angezeigt wird. Vor v2.1.205 zeigte er seinen früheren fehlgeschlagenen oder abgeschlossenen Status, während der fortgesetzte Lauf funktionierte.

Ab v2.1.199 überprüft `SendMessage`, dass ein Name immer noch auf denselben Agenten verweist, den er früher in der Konversation erreicht hat. Wenn ein neuerer Agent den Namen übernommen hat, z. B. ein neu gespawnter Hintergrund-Agent, der ihn wiederverwendet hat, weigert sich Claude Code, die Nachricht zu senden, anstatt sie an den falschen Agenten zu liefern, und der Fehler meldet, welcher Agent der Name jetzt erreicht, damit Claude neu ausrichten kann. Um den früheren Agenten zu erreichen, während er noch läuft, spricht Claude ihn nach der Agent-ID an, die er erhielt, als er diesen Agenten spawnte. Die Überprüfung ist auf die aktuelle Konversation beschränkt und wird bei `/clear` zurückgesetzt.

Ab v2.1.198 behandelt ein Subagent Nachrichten vom Agenten, der ihn gestartet hat, als normale Aufgabenrichtung, einschließlich Kurskorrektionen während der Aufgabe, und handelt nach ihnen innerhalb seiner eigenen Genehmigungseinstellungen. Zwei Grenzen gelten trotzdem unabhängig davon, wer die Nachricht gesendet hat: Keine Nachricht von einem Agenten zählt als Ihre Genehmigung für eine ausstehende Genehmigungseingabeaufforderung, und keine Agent-Nachricht kann die Genehmigungseinstellungen, `CLAUDE.md` oder Konfiguration eines Subagenten ändern. Nur das Genehmigungssystem oder Ihre eigenen Nachrichten können Genehmigung gewähren.

Sie können Claude auch nach der Agent-ID fragen, wenn Sie sie explizit referenzieren möchten, oder IDs in den Transkriptdateien unter `~/.claude/projects/{project}/{sessionId}/subagents/` finden. Jedes Transkript wird als `agent-{agentId}.jsonl` gespeichert.

Subagenten-Transkripte bleiben unabhängig von der Hauptkonversation bestehen:

* **Hauptkonversations-Komprimierung**: Wenn die Hauptkonversation komprimiert wird, sind Subagenten-Transkripte nicht betroffen. Sie werden in separaten Dateien gespeichert.
* **Sitzungspersistenz**: Subagenten-Transkripte bleiben innerhalb ihrer Sitzung bestehen. Sie können [einen Subagenten fortsetzen](#resume-subagents), nachdem Sie Claude Code neu gestartet haben, indem Sie dieselbe Sitzung fortsetzen.
* **Automatische Bereinigung**: Claude Code löscht Subagenten-Transkripte nach der Aufbewahrungsfrist `cleanupPeriodDays`, standardmäßig 30 Tage, nach den [Aufbewahrungsfeger-Regeln](/docs/de/claude-directory#cleaned-up-automatically).

<h4 id="auto-compaction">
  Auto-Komprimierung
</h4>

Subagenten unterstützen automatische Komprimierung mit der gleichen Logik wie die Hauptkonversation. Die Komprimierung wird unter den gleichen Bedingungen ausgelöst, und `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE` gilt auch für Subagenten. Siehe [Umgebungsvariablen](/docs/de/env-vars) für den Zeitpunkt, an dem die Überschreibung wirksam wird.

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

Der Wert `preTokens` zeigt, wie viele Token vor der Komprimierung verwendet wurden.

<h2 id="fork-the-current-conversation">
  Gegabelte Konversation
</h2>

<Note>
  Führen Sie einen gegabelten Subagenten mit `/subtask` aus, was Claude Code v2.1.212 oder später erfordert. Wenn [die Agenten-Ansicht ausgeschaltet ist](/docs/de/agent-view#turn-off-agent-view), ist `/subtask` nicht verfügbar und `/fork` startet stattdessen den gegabelten Subagenten; andernfalls kopiert `/fork` die gesamte Sitzung in eine neue [Hintergrund-Sitzung](/docs/de/agent-view#from-inside-a-session).
</Note>

Ein Fork ist ein Subagent, der die gesamte bisherige Konversation erbt, anstatt von vorne zu beginnen. Dies lässt die Eingabe-Isolierung fallen, die Subagenten ansonsten bieten: Ein Fork sieht denselben Systemprompt, dieselben Werkzeuge, dasselbe Modell und die Nachrichtenhistorie wie die Hauptsitzung, sodass Sie ihm eine Nebenaufgabe übergeben können, ohne die Situation erneut zu erklären. Die eigenen Werkzeugaufrufe des Forks bleiben weiterhin aus Ihrer Konversation heraus und nur sein endgültiges Ergebnis kommt zurück, sodass Ihr Hauptkontextfenster sauber bleibt. Verwenden Sie einen Fork, wenn ein anderer Subagent zu viel Hintergrund benötigen würde, um nützlich zu sein, oder wenn Sie mehrere Ansätze parallel vom gleichen Ausgangspunkt aus versuchen möchten.

Claude startet einen Fork, indem er den `fork`-Subagenten-Typ durch das Agent-Werkzeug anfordert. Sie steuern, ob dies möglich ist, mit dem [Fork-Modus](#turn-fork-mode-on-or-off), der in interaktiven Sitzungen standardmäßig aktiviert ist.

Sie können einen Fork selbst mit `/subtask` gefolgt von einer Aufgabe starten, unabhängig davon, ob der Fork-Modus aktiviert ist oder nicht. In v2.1.161 bis v2.1.211 ist der Befehl `/fork`. Claude Code benennt den Fork aus den ersten Worten der Aufgabe. Das folgende Beispiel gabelt die Konversation, um Testfälle zu entwerfen, während Sie mit der Implementierung in der Hauptsitzung fortfahren:

```text wrap theme={null}
/subtask draft unit tests for the parser changes so far
```

Der Fork erscheint in einem Panel unter Ihrer Eingabeaufforderung und läuft im Hintergrund, während Sie weiterarbeiten. Wenn er fertig ist, kommt sein Ergebnis als Nachricht in Ihrer Hauptkonversation an. Der nächste Abschnitt behandelt die Panel-Steuerelemente zum Beobachten und Lenken von Forks während ihrer Ausführung.

<h3 id="observe-and-steer-running-forks">
  Beobachten und lenken Sie laufende Forks
</h3>

Laufende Forks erscheinen in einem Panel unter der Eingabeaufforderung, mit einer Zeile für die Hauptsitzung und einer für jeden Fork.

Wenn ein Fork erfolgreich abgeschlossen ist, entfernt Claude Code seine Zeile. Claude Code behält die Zeile eines Forks, der fehlgeschlagen ist oder den Sie gestoppt haben, für 30 Sekunden bei, [das gleiche wie für jeden anderen Hintergrund-Subagenten](#run-subagents-in-foreground-or-background). Vor v2.1.232 behielt Claude Code die Zeile eines abgeschlossenen Forks ebenfalls für 30 Sekunden bei.

Verwenden Sie diese Tasten, um mit dem Panel zu interagieren:

| Taste     | Aktion                                                                                                                                                                                                                                                                    |
| :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `↑` / `↓` | Zwischen Zeilen wechseln                                                                                                                                                                                                                                                  |
| `Enter`   | Öffnen Sie das Transkript des ausgewählten Forks und senden Sie ihm Folgefragen                                                                                                                                                                                           |
| `x`       | Stoppen Sie den ausgewählten Fork, wenn er läuft, oder schließen Sie seine Zeile, wenn er nicht mehr läuft. In der Hauptsitzungs-Zeile oder in der Zeile des Forks, dessen Transkript Sie mit `Enter` geöffnet haben, gibt `x` stattdessen in die Eingabeaufforderung ein |
| `Esc`     | Fokus zurück zur Eingabeaufforderung                                                                                                                                                                                                                                      |

Mit einem geöffneten Transkript eines Forks oder Subagenten gehen Folgefragen und [Skills](/docs/de/skills) an diesen Agenten, aber integrierte Befehle werden weiterhin in Ihrer Hauptkonversation ausgeführt. Ab v2.1.199 zeigt die Eingabe von `/model` oder `/fast` in dieser Ansicht einen Hinweis an, dass dies das Modell oder den Schnellmodus der Hauptkonversation ändert, nicht des angezeigten Agenten, anstatt es stillschweigend auszuführen.

<h3 id="how-forks-differ-from-other-subagents">
  Wie sich Forks von anderen Subagenten unterscheiden
</h3>

Ein Fork erbt alles, was die Hauptsitzung zum Zeitpunkt des Spawnens hat. Jeder andere Subagent startet von seiner Definition aus.

|                            | Fork                                        | Nicht-Fork-Subagent                                                                                                         |
| :------------------------- | :------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------- |
| Kontext                    | Vollständige Konversationshistorie          | Frischer Kontext mit dem Prompt, den Sie übergeben                                                                          |
| Systemprompt und Werkzeuge | Gleich wie Hauptsitzung                     | Aus der [Definitionsdatei](#write-subagent-files) des Subagenten, [gefiltert für Hintergrund-Läufe](#available-tools)       |
| Modell                     | Gleich wie Hauptsitzung                     | Aus dem `model`-Feld des Subagenten                                                                                         |
| Berechtigungen             | Aufforderungen erscheinen in Ihrem Terminal | [Aufforderungen erscheinen in Ihrer Hauptsitzung](#run-subagents-in-foreground-or-background) bei Ausführung im Hintergrund |
| Prompt-Cache               | Mit Hauptsitzung geteilt                    | Separater Cache                                                                                                             |

Da der Systemprompt und die Werkzeugdefinitionen eines Forks identisch mit dem übergeordneten Element sind, wird seine erste Anfrage den [Prompt-Cache](/docs/de/prompt-caching#subagents-and-the-cache) des übergeordneten Elements wiederverwenden. Dies macht das Forking billiger als das Spawnen eines frischen Subagenten für Aufgaben, die denselben Kontext benötigen.

Wenn Claude einen Fork durch das Agent-Werkzeug spawnt, kann es `isolation: "worktree"` übergeben, sodass die Dateibearbeitungen des Forks in einen separaten Git-Worktree geschrieben werden, anstatt in Ihren Checkout. Ein Fork kann keine weiteren Forks spawnen.

<h3 id="turn-fork-mode-on-or-off">
  Fork-Modus aktivieren oder deaktivieren
</h3>

Claude Code aktiviert den Fork-Modus standardmäßig in interaktiven Sitzungen und lässt ihn standardmäßig im [nicht-interaktiven Modus](/docs/de/headless) mit `-p` und im Agent SDK deaktiviert. Der interaktive Standard erfordert Claude Code v2.1.232 oder später. In früheren Versionen setzen Sie `CLAUDE_CODE_FORK_SUBAGENT` auf `1`, um den Fork-Modus zu aktivieren.

Sie können erkennen, dass der Fork-Modus aktiviert ist, an der Art und Weise, wie Claude Code das Agent-Werkzeug handhabt:

* Claude kann einen Fork spawnen, indem er den `fork`-Subagenten-Typ anfordert. Wenn Claude keinen Typ anfordert, erhält es den [allgemeinen](#built-in-subagents)-Subagenten, falls die Sitzung diesen Typ noch hat. Subagenten, die aus einer Definition gespawnt werden, wie Explore, funktionieren wie gewohnt.
* Claude Code führt die Subagenten, die Claude spawnt, im Hintergrund aus, Forks und Nicht-Fork-Subagenten gleichermaßen, abgesehen von den [Fällen, die im Vordergrund bleiben](#run-subagents-in-foreground-or-background). Claude Code entfernt auch den `run_in_background`-Parameter des Agent-Werkzeugs, sodass Claude nicht den Vordergrund anfordern kann.

Setzen Sie die Umgebungsvariable [`CLAUDE_CODE_FORK_SUBAGENT`](/docs/de/env-vars), um die Standardwerte zu überschreiben:

* `1` aktiviert den Fork-Modus auch im nicht-interaktiven Modus und im Agent SDK
* `0` deaktiviert den Fork-Modus in jeder Art von Sitzung

Um den Fork-Modus aktiviert zu halten, aber Claude daran zu hindern, Forks zu spawnen, [verweigern Sie den `fork`-Subagenten-Typ](#disable-specific-subagents) mit einer `Agent(fork)`-Regel. Claude Code führt die Subagenten, die Claude spawnt, weiterhin im Hintergrund aus, abgesehen von den gleichen [Fällen, die im Vordergrund bleiben](#run-subagents-in-foreground-or-background).

<h2 id="example-subagents">
  Beispiel-Subagenten
</h2>

Diese Beispiele demonstrieren effektive Muster für die Erstellung von Subagenten. Verwenden Sie sie als Ausgangspunkte oder generieren Sie eine angepasste Version mit Claude.

<Tip>
  **Best Practices:**

  * **Entwerfen Sie fokussierte Subagenten:** Jeder Subagent sollte bei einer spezifischen Aufgabe hervorragend sein
  * **Schreiben Sie Beschreibungen, die einen Subagenten hervorheben:** Claude verwendet die Beschreibung, um zu entscheiden, wann delegiert werden soll. Machen Sie jede Beschreibung spezifisch genug, um zum richtigen Subagenten weiterzuleiten, und halten Sie die kombinierte Menge innerhalb des [15.000-Token-Beschreibungsbudgets](#understand-automatic-delegation)
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

Das System-Prompt teilt dem Subagenten mit, Schreibanfragen abzulehnen, daher ist der Hook ein Sicherheitsmechanismus: Wenn der Subagent trotzdem einen Schreibvorgang versucht, blockiert Claude Code den Befehl und der Subagent sieht die Meldung `Blocked: Write operations not allowed. Use SELECT queries only.`.

<h2 id="next-steps">
  Nächste Schritte
</h2>

Jetzt, da Sie Subagenten verstehen, erkunden Sie diese verwandten Funktionen:

* [Verteilen Sie Subagenten mit Plugins](/docs/de/plugins/components#agents), um Subagenten über Teams oder Projekte hinweg zu teilen
* [Führen Sie Claude Code programmgesteuert aus](/docs/de/headless) mit dem Agent SDK für CI/CD und Automatisierung
* [Verwenden Sie MCP-Server](/docs/de/mcp), um Subagenten Zugriff auf externe Werkzeuge und Daten zu geben
