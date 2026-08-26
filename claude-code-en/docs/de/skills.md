> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude mit Skills erweitern

> Erstellen, verwalten und teilen Sie Skills, um Claudes Funktionen in Claude Code zu erweitern. Umfasst benutzerdefinierte Befehle und gebündelte Skills.

Skills erweitern das, was Claude tun kann. Erstellen Sie eine `SKILL.md`-Datei mit Anweisungen, und Claude fügt sie zu seinem Toolkit hinzu. Claude verwendet Skills, wenn sie relevant sind, oder Sie können einen direkt mit `/skill-name` aufrufen.

Erstellen Sie einen Skill, wenn Sie immer wieder die gleichen Anweisungen, eine Checkliste oder ein mehrstufiges Verfahren in den Chat einfügen, oder wenn ein Abschnitt von CLAUDE.md zu einem Verfahren statt zu einer Tatsache geworden ist. Im Gegensatz zu CLAUDE.md-Inhalten wird der Body eines Skills nur geladen, wenn er verwendet wird, sodass lange Referenzmaterialien fast nichts kosten, bis Sie sie benötigen.

<Note>
  Für integrierte Befehle wie `/help` und `/compact` sowie gebündelte Skills wie `/debug` und `/code-review` siehe die [Befehlsreferenz](/docs/de/commands).

  **Benutzerdefinierte Befehle wurden in Skills zusammengeführt.** Eine Datei unter `.claude/commands/deploy.md` und ein Skill unter `.claude/skills/deploy/SKILL.md` erstellen beide `/deploy` und funktionieren auf die gleiche Weise. Ihre vorhandenen `.claude/commands/`-Dateien funktionieren weiterhin. Skills fügen optionale Funktionen hinzu: ein Verzeichnis für unterstützende Dateien, Frontmatter zum [Steuern, wer einen Skill aufruft](#control-who-invokes-a-skill), und die Möglichkeit für Claude, sie automatisch zu laden, wenn sie relevant sind.
</Note>

Claude Code Skills folgen dem [Agent Skills](https://agentskills.io) offenen Standard, der über mehrere KI-Tools funktioniert. Claude Code erweitert den Standard mit zusätzlichen Funktionen wie [Invocation Control](#control-who-invokes-a-skill), [Subagent-Ausführung](#run-skills-in-a-subagent) und [dynamischer Kontexteinspritzung](#inject-dynamic-context).

<h2 id="bundled-skills">
  Gebündelte Skills
</h2>

Claude Code wird mit einer Reihe von gebündelten Skills ausgeliefert, die in jeder Sitzung verfügbar sind, sofern sie nicht mit der Einstellung [`disableBundledSkills`](/docs/de/settings#available-settings) deaktiviert werden, einschließlich `/doctor`, `/code-review`, `/batch`, `/debug`, `/loop` und `/claude-api`. Im Gegensatz zu den meisten integrierten Befehlen, die direkt feste Logik ausführen, sind gebündelte Skills prompt-basiert: Sie geben Claude detaillierte Anweisungen und lassen es die Arbeit mit seinen Tools orchestrieren. Sie rufen sie auf die gleiche Weise auf wie jeden anderen Skill, indem Sie `/` gefolgt vom Skill-Namen eingeben.

Die [`/doctor`](/docs/de/commands#all-commands) Setup-Überprüfung ist die eine Ausnahme zu `disableBundledSkills` in Claude Code v2.1.205 und später: Sie bleibt eingabbar, wenn die Einstellung aktiviert ist. Um sie auszublenden, setzen Sie die Umgebungsvariable `DISABLE_DOCTOR_COMMAND` oder einen [`skillOverrides`](#override-skill-visibility-from-settings) Eintrag von `"doctor": "off"`. Vor v2.1.205 war `/doctor` ein integrierter Befehl und kein gebündelter Skill.

Gebündelte Skills sind in der [Befehlsreferenz](/docs/de/commands) neben integrierten Befehlen aufgelistet und mit **Skill** in der Spalte „Zweck" gekennzeichnet.

<h3 id="run-and-verify-your-app">
  Ihre App ausführen und überprüfen
</h3>

Drei gebündelte Skills arbeiten zusammen, um Ihre App zu starten und Änderungen gegen die laufende App zu bestätigen, anstatt nur Tests durchzuführen:

| Skill                  | Zweck                                                                                                                                                   |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `/run`                 | Starten und steuern Sie Ihre App, um eine Änderung in Aktion zu sehen                                                                                   |
| `/verify`              | Erstellen und führen Sie Ihre App aus, um zu bestätigen, dass eine Codeänderung das tut, was sie soll, ohne auf Tests oder Typprüfungen zurückzugreifen |
| `/run-skill-generator` | Lehren Sie `/run` und `/verify`, wie Sie Ihr Projekt erstellen und starten                                                                              |

Alle drei Skills erfordern Claude Code v2.1.145 oder später.

`/run` und `/verify` funktionieren ohne Einrichtung. Sie leiten den Start von Ihrem Projekttyp ab (CLI, Server, TUI, browsergesteuert) und von dem, was sich in Ihrer README, `package.json` oder `Makefile` befindet. Diese Ableitung wird unzuverlässig für Projekte, die mehr als einen Standard-Start benötigen: eine Datenbank, eine Env-Datei, eine grafische Sitzung, einen mehrstufigen Build.

`/run-skill-generator` zeichnet stattdessen das Rezept auf. Es bringt Ihre App aus einer sauberen Umgebung zum Laufen, erfasst, was funktioniert hat (die Installationsbefehle, die Umgebungsvariablen, das Startskript), und speichert es als projektspezifischen Skill unter `.claude/skills/run-<name>/`. Danach folgen `/run`, `/verify` und alle anderen Agenten im Repository dem aufgezeichneten Rezept, anstatt es neu zu entdecken. Führen Sie `/run-skill-generator` einmal pro Projekt aus, und erneut, wenn sich der Build- oder Startprozess ändert.

<h2 id="getting-started">
  Erste Schritte
</h2>

<h3 id="create-your-first-skill">
  Erstellen Sie Ihren ersten Skill
</h3>

Dieses Beispiel erstellt einen Skill, der die nicht committeten Änderungen in Ihrem Git-Repository zusammenfasst und alles Riskante kennzeichnet. Es zieht den Live-Diff in den Prompt, bevor Claude ihn liest, sodass die Antwort in Ihrem tatsächlichen Arbeitsbaum verankert ist, anstatt auf dem zu basieren, was Claude aus offenen Dateien erraten kann. Claude lädt den Skill automatisch, wenn Sie nach Ihren Änderungen fragen, oder Sie können ihn direkt mit `/summarize-changes` aufrufen.

<Steps>
  <Step title="Erstellen Sie das Skill-Verzeichnis">
    Erstellen Sie ein Verzeichnis für den Skill in Ihrem persönlichen Skills-Ordner. Persönliche Skills sind über alle Ihre Projekte hinweg verfügbar.

    ```bash theme={null}
    mkdir -p ~/.claude/skills/summarize-changes
    ```
  </Step>

  <Step title="Schreiben Sie SKILL.md">
    Jeder Skill benötigt eine `SKILL.md`-Datei mit zwei Teilen: YAML-Frontmatter zwischen `---`-Markierungen, das Claude mitteilt, wann der Skill verwendet werden soll, und Markdown-Inhalt mit Anweisungen, die Claude befolgt, wenn der Skill ausgeführt wird. Das Verzeichnisname wird zum Befehl, den Sie eingeben, und die `description` hilft Claude zu entscheiden, wann der Skill automatisch geladen werden soll.

    Speichern Sie dies unter `~/.claude/skills/summarize-changes/SKILL.md`:

    ```yaml theme={null}
    ---
    description: Summarizes uncommitted changes and flags anything risky. Use when the user asks what changed, wants a commit message, or asks to review their diff.
    ---

    ## Current changes

    !`git diff HEAD`

    ## Instructions

    Summarize the changes above in two or three bullet points, then list any risks you notice such as missing error handling, hardcoded values, or tests that need updating. If the diff is empty, say there are no uncommitted changes.
    ```

    Die Zeile `` !`git diff HEAD` `` verwendet [dynamische Kontextinjektion](#inject-dynamic-context): Claude Code führt den Befehl aus und ersetzt die Zeile mit seiner Ausgabe, bevor Claude den Skill-Inhalt sieht, sodass die Anweisungen mit dem aktuellen Diff bereits inline ankommen.
  </Step>

  <Step title="Testen Sie den Skill">
    Öffnen Sie ein Git-Projekt, nehmen Sie eine kleine Änderung an einer beliebigen Datei vor, und starten Sie Claude Code, indem Sie `claude` ausführen. Sie können den Skill auf zwei Arten testen.

    **Lassen Sie Claude ihn automatisch aufrufen**, indem Sie etwas eingeben, das der Beschreibung entspricht:

    ```text theme={null}
    What did I change?
    ```

    **Oder rufen Sie ihn direkt auf** mit dem Skill-Namen:

    ```text theme={null}
    /summarize-changes
    ```

    In beiden Fällen sollte Claude mit einer kurzen Zusammenfassung Ihrer Änderung und einer Liste von Risiken antworten.
  </Step>
</Steps>

<h3 id="where-skills-live">
  Wo Skills leben
</h3>

Wo Sie einen Skill speichern, bestimmt, wer ihn verwenden kann:

| Ort         | Pfad                                                          | Gilt für                            |
| :---------- | :------------------------------------------------------------ | :---------------------------------- |
| Unternehmen | Siehe [verwaltete Einstellungen](/docs/de/settings#settings-files) | Alle Benutzer in Ihrer Organisation |
| Persönlich  | `~/.claude/skills/<skill-name>/SKILL.md`                      | Alle Ihre Projekte                  |
| Projekt     | `.claude/skills/<skill-name>/SKILL.md`                        | Nur dieses Projekt                  |
| Plugin      | `<plugin>/skills/<skill-name>/SKILL.md`                       | Wo das Plugin aktiviert ist         |

Wenn Skills auf verschiedenen Ebenen denselben Namen haben, gewinnt Unternehmen gegenüber Persönlich, und Persönlich gewinnt gegenüber Projekt. Ein Skill auf jeder dieser Ebenen setzt auch einen gebündelten Skill mit demselben Namen außer Kraft. Beispielsweise ersetzt ein `code-review`-Skill in Ihrem Projekt `.claude/skills/` den gebündelten `/code-review`. Plugin-Skills verwenden einen `plugin-name:skill-name`-Namespace, sodass sie nicht mit anderen Ebenen in Konflikt geraten können. Wenn Sie Dateien in `.claude/commands/` haben, funktionieren diese auf die gleiche Weise, aber wenn ein Skill und ein Befehl denselben Namen haben, hat der Skill Vorrang.

Skills werden auch aus verschachtelten `.claude/skills/`-Verzeichnissen unter Ihrem Arbeitsverzeichnis geladen. Wenn Claude eine Datei in einem Unterverzeichnis liest oder bearbeitet, werden Skills aus dem `.claude/skills/` dieses Unterverzeichnisses verfügbar. Dies ermöglicht es einem Monorepo-Paket, seine eigenen Skills bereitzustellen, die beim Arbeiten an diesem Paket gelten, auch wenn die Sitzung in der Repository-Root gestartet wurde.

Wenn ein verschachtelter Skill denselben Namen wie ein anderer Skill hat, bleiben beide verfügbar. Beispielsweise mit einem `deploy`-Skill in der Projekt-Root und einem anderen in `apps/web/.claude/skills/`:

* Der verschachtelte wird unter einem verzeichnisqualifizierten Namen angezeigt, `apps/web:deploy`.
* Seine Beschreibung sagt, auf welches Verzeichnis er sich bezieht.
* Claude wählt die Variante, die zu den Dateien passt, an denen er arbeitet.

Wenn Sie `/deploy` eingeben, wird der Skill in der Projekt-Root ausgeführt. Geben Sie den qualifizierten Namen `/apps/web:deploy` ein, um die verschachtelte Variante explizit auszuführen.

Wenn Sie oder Claude den unqualifizierten Namen aufrufen, wird der Skill in der Projekt-Root geladen, und Claude Code fügt eine Liste der verzeichnisqualifizierten Varianten zu seinem Inhalt mit einer Anweisung hinzu, auch jede Variante aufzurufen, deren Verzeichnis die Dateien enthält, an denen Claude arbeitet. Ein verschachtelter Skill gilt daher immer noch für die Arbeit in seinem Verzeichnis, wenn nur der unqualifizierte Name aufgerufen wird. Erfordert Claude Code v2.1.203 oder später.

Ein `<skill-name>`-Eintrag an den Standorten Unternehmen, Persönlich oder Projekt kann ein Symlink zu einem Verzeichnis an anderer Stelle auf der Festplatte sein. Claude Code folgt dem Symlink und liest `SKILL.md` aus dem Zielverzeichnis, und wenn dasselbe Ziel von mehr als einem Ort aus erreichbar ist, lädt Claude Code den Skill einmal. Plugin-Skills handhaben Symlinks anders; siehe [Dateien in einem Marketplace mit Symlinks teilen](/docs/de/plugins-reference#share-files-within-a-marketplace-with-symlinks).

<Note>
  Fügen Sie eine `.claude-plugin/plugin.json` zu einem Skill-Ordner hinzu und er wird als [Plugin](/docs/de/plugins-reference#skills-directory-plugins) mit dem Namen `<name>@skills-dir` geladen, sodass er Agenten, hooks und MCP-Server bündeln kann. In einem Projekt `.claude/skills/` ist dies erforderlich, um zuerst das Workspace-Trust-Dialogfeld zu akzeptieren.
</Note>

<h4 id="live-change-detection">
  Live-Änderungserkennung
</h4>

Claude Code überwacht Skill-Verzeichnisse auf Dateiänderungen. Das Hinzufügen, Bearbeiten oder Entfernen eines Skills unter `~/.claude/skills/`, dem Projekt `.claude/skills/` oder einem `.claude/skills/` in einem `--add-dir`-Verzeichnis wird in der aktuellen Sitzung wirksam, ohne Claude Code neu zu starten. Das Erstellen eines Skill-Verzeichnisses auf oberster Ebene, das nicht vorhanden war, als die Sitzung gestartet wurde, erfordert einen Neustart von Claude Code, damit das neue Verzeichnis überwacht werden kann.

<Note>
  Die Live-Änderungserkennung umfasst nur `SKILL.md`-Text. Für einen Skill-Ordner, der auch ein [Plugin](/docs/de/plugins-reference#skills-directory-plugins) ist, benötigen Änderungen an `hooks/`, `.mcp.json`, `agents/` und `output-styles/` `/reload-plugins`, um wirksam zu werden.
</Note>

<h4 id="automatic-discovery-from-parent-and-nested-directories">
  Automatische Erkennung aus übergeordneten und verschachtelten Verzeichnissen
</h4>

Projekt-Skills werden aus `.claude/skills/` in Ihrem Startverzeichnis und in jedem übergeordneten Verzeichnis bis zur Repository-Root geladen, sodass das Starten von Claude in einem Unterverzeichnis immer noch Skills erfasst, die in der Root definiert sind. Wenn Sie mit Dateien in Unterverzeichnissen unter Ihrem Startverzeichnis arbeiten, erkennt Claude Code auch Skills aus verschachtelten `.claude/skills/`-Verzeichnissen bei Bedarf. Wenn Sie beispielsweise eine Datei in `packages/frontend/` bearbeiten, sucht Claude Code auch nach Skills in `packages/frontend/.claude/skills/`. Dies unterstützt Monorepo-Setups, bei denen Pakete ihre eigenen Skills haben.

Jeder Skill ist ein Verzeichnis mit `SKILL.md` als Einstiegspunkt:

```text theme={null}
my-skill/
├── SKILL.md           # Main instructions (required)
├── template.md        # Template for Claude to fill in
├── examples/
│   └── sample.md      # Example output showing expected format
└── scripts/
    └── validate.sh    # Script Claude can execute
```

Die `SKILL.md` enthält die Hauptanweisungen und ist erforderlich. Andere Dateien sind optional und ermöglichen es Ihnen, leistungsfähigere Skills zu erstellen: Vorlagen für Claude zum Ausfüllen, Beispielausgaben, die das erwartete Format zeigen, Scripts, die Claude ausführen kann, oder detaillierte Referenzdokumentation. Verweisen Sie auf diese Dateien von Ihrer `SKILL.md` aus, damit Claude weiß, was sie enthalten und wann sie geladen werden sollen. Siehe [Unterstützende Dateien hinzufügen](#add-supporting-files) für weitere Details.

<Note>
  Dateien in `.claude/commands/` funktionieren weiterhin und unterstützen das gleiche [Frontmatter](#frontmatter-reference). Skills werden empfohlen, da sie zusätzliche Funktionen wie unterstützende Dateien unterstützen.
</Note>

<h4 id="skills-from-additional-directories">
  Skills aus zusätzlichen Verzeichnissen
</h4>

Das Flag `--add-dir` und der Befehl `/add-dir` [gewähren Dateizugriff](/docs/de/permissions#additional-directories-grant-file-access-not-configuration) statt Konfigurationserkennung, aber Skills sind eine Ausnahme: `.claude/skills/` in einem hinzugefügten Verzeichnis wird automatisch geladen. Diese Ausnahme gilt nur für `--add-dir` und `/add-dir`. Die Einstellung `permissions.additionalDirectories` in `settings.json` gewährt nur Dateizugriff und lädt keine Skills. Siehe [Live-Änderungserkennung](#live-change-detection) für die Aufnahme von Änderungen während einer Sitzung.

Andere `.claude/`-Konfigurationen wie Befehle und Ausgabestile werden nicht aus zusätzlichen Verzeichnissen geladen. Siehe die [Ausnahmetabelle](/docs/de/permissions#additional-directories-grant-file-access-not-configuration) für die vollständige Liste dessen, was geladen wird und was nicht, sowie die empfohlenen Wege zum Teilen von Konfigurationen über Projekte hinweg.

<Note>
  CLAUDE.md-Dateien aus `--add-dir`-Verzeichnissen werden standardmäßig nicht geladen. Um sie zu laden, setzen Sie `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1`. Siehe [Aus zusätzlichen Verzeichnissen laden](/docs/de/memory#load-from-additional-directories).
</Note>

<h2 id="configure-skills">
  Skills konfigurieren
</h2>

Skills werden durch YAML-Frontmatter oben in `SKILL.md` und den Markdown-Inhalt, der folgt, konfiguriert.

<h3 id="types-of-skill-content">
  Arten von Skill-Inhalten
</h3>

Skill-Dateien können beliebige Anweisungen enthalten, aber das Nachdenken darüber, wie Sie sie aufrufen möchten, hilft zu leiten, was Sie einbeziehen:

**Referenzinhalt** fügt Wissen hinzu, das Claude auf Ihre aktuelle Arbeit anwendet. Konventionen, Muster, Stilhandbücher, Domänenwissen. Dieser Inhalt wird inline ausgeführt, sodass Claude ihn neben Ihrem Gesprächskontext verwenden kann.

```yaml theme={null}
---
name: api-conventions
description: API design patterns for this codebase
---

When writing API endpoints:
- Use RESTful naming conventions
- Return consistent error formats
- Include request validation
```

**Task-Inhalt** gibt Claude Schritt-für-Schritt-Anweisungen für eine bestimmte Aktion, wie Bereitstellungen, Commits oder Code-Generierung. Dies sind oft Aktionen, die Sie direkt mit `/skill-name` aufrufen möchten, anstatt Claude entscheiden zu lassen, wann sie ausgeführt werden. Fügen Sie `disable-model-invocation: true` hinzu, um zu verhindern, dass Claude sie automatisch auslöst.

```yaml theme={null}
---
name: deploy
description: Deploy the application to production
context: fork
disable-model-invocation: true
---

Deploy the application:
1. Run the test suite
2. Build the application
3. Push to the deployment target
```

Ihre `SKILL.md` kann alles enthalten, aber das Nachdenken darüber, wie Sie den Skill aufrufen möchten (von Ihnen, von Claude oder von beiden) und wo Sie ihn ausführen möchten (inline oder in einem Subagent) hilft zu leiten, was Sie einbeziehen. Für komplexe Skills können Sie auch [unterstützende Dateien hinzufügen](#add-supporting-files), um den Hauptskill fokussiert zu halten.

Halten Sie den Text selbst prägnant. Sobald ein Skill geladen ist, bleibt sein Inhalt [über Züge hinweg im Kontext](#skill-content-lifecycle), sodass jede Zeile eine wiederkehrende Token-Kosten darstellt. Geben Sie an, was zu tun ist, anstatt zu erzählen, wie oder warum, und wenden Sie denselben Prägnanz-Test an, den Sie für [CLAUDE.md-Inhalt](/docs/de/best-practices#write-an-effective-claude-md) verwenden würden.

<h3 id="frontmatter-reference">
  Frontmatter-Referenz
</h3>

Über den Markdown-Inhalt hinaus können Sie das Skill-Verhalten mit YAML-Frontmatter-Feldern zwischen `---`-Markierungen oben in Ihrer `SKILL.md`-Datei konfigurieren:

```yaml theme={null}
---
name: my-skill
description: What this skill does
disable-model-invocation: true
allowed-tools: Read Grep
---

Your skill instructions here...
```

Alle Felder sind optional. Nur `description` wird empfohlen, damit Claude weiß, wann der Skill verwendet werden soll.

| Feld                       | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :------------------------- | :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                     | Nein         | Anzeigename, der in Skill-Auflistungen angezeigt wird. Standardmäßig der Verzeichnisname. Siehe [Wie ein Skill seinen Befehlsnamen erhält](#how-a-skill-gets-its-command-name), um zu verstehen, wie sich dies vom Namen unterscheidet, den Sie eingeben, um den Skill aufzurufen.                                                                                                                                                                                                                                                                                           |
| `description`              | Empfohlen    | Was der Skill tut und wann er verwendet werden soll. Claude verwendet dies, um zu entscheiden, wann der Skill angewendet werden soll. Falls weggelassen, wird der erste Absatz des Markdown-Inhalts verwendet. Stellen Sie den wichtigsten Anwendungsfall an den Anfang: Der kombinierte Text `description` und `when_to_use` wird in der Skill-Auflistung bei 1.536 Zeichen gekürzt, um die Kontextnutzung zu reduzieren.                                                                                                                                                   |
| `when_to_use`              | Nein         | Zusätzlicher Kontext für den Zeitpunkt, zu dem Claude den Skill aufrufen sollte, z. B. Trigger-Phrasen oder Beispielanfragen. An `description` in der Skill-Auflistung angehängt und zählt zur 1.536-Zeichen-Obergrenze.                                                                                                                                                                                                                                                                                                                                                     |
| `argument-hint`            | Nein         | Hinweis, der während der Autovervollständigung angezeigt wird, um erwartete Argumente anzuzeigen. Beispiel: `[issue-number]` oder `[filename] [format]`.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `arguments`                | Nein         | Benannte positionelle Argumente für [`$name`-Substitution](#available-string-substitutions) im Skill-Inhalt. Akzeptiert eine durch Leerzeichen getrennte Zeichenkette oder eine YAML-Liste. Namen werden in Reihenfolge auf Argumentpositionen abgebildet.                                                                                                                                                                                                                                                                                                                   |
| `disable-model-invocation` | Nein         | Setzen Sie auf `true`, um zu verhindern, dass Claude diesen Skill automatisch lädt. Verwenden Sie für Workflows, die Sie manuell mit `/name` auslösen möchten. Verhindert auch, dass der Skill [in Subagenten vorgeladen wird](/docs/de/sub-agents#preload-skills-into-subagents). Ab v2.1.196 verhindert dies auch, dass der Skill ausgeführt wird, wenn eine [geplante Aufgabe](/docs/de/scheduled-tasks) mit dem Skill als Eingabe ausgelöst wird. Standard: `false`.                                                                                                               |
| `user-invocable`           | Nein         | Setzen Sie auf `false`, um aus dem `/`-Menü auszublenden. Verwenden Sie für Hintergrundwissen, das Benutzer nicht direkt aufrufen sollten. Standard: `true`.                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `allowed-tools`            | Nein         | Tools, die Claude ohne Genehmigung verwenden kann, wenn dieser Skill aktiv ist. Akzeptiert eine durch Leerzeichen oder Komma getrennte Zeichenkette oder eine YAML-Liste.                                                                                                                                                                                                                                                                                                                                                                                                    |
| `disallowed-tools`         | Nein         | Tools, die aus Claudes verfügbarem Pool entfernt werden, während dieser Skill aktiv ist. Verwenden Sie für autonome Skills, die niemals bestimmte Tools aufrufen sollten, wie `AskUserQuestion` für eine Hintergrund-Schleife. Akzeptiert eine durch Leerzeichen oder Komma getrennte Zeichenkette oder eine YAML-Liste. Die Einschränkung wird gelöscht, wenn Sie Ihre nächste Nachricht senden.                                                                                                                                                                            |
| `model`                    | Nein         | Modell, das verwendet werden soll, wenn dieser Skill aktiv ist. Die Überschreibung gilt für den Rest des aktuellen Zuges und wird nicht in den Einstellungen gespeichert; das Sitzungsmodell wird bei Ihrer nächsten Eingabe fortgesetzt. Akzeptiert die gleichen Werte wie [`/model`](/docs/de/model-config) oder `inherit`, um das aktive Modell beizubehalten. Ein Wert, der durch die [`availableModels`](/docs/de/model-config#restrict-model-selection)-Zulassungsliste Ihrer Organisation ausgeschlossen ist, wird nicht verwendet und die Sitzung behält ihr aktuelles Modell. |
| `effort`                   | Nein         | [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level) wenn dieser Skill aktiv ist. Überschreibt die Anstrengungsstufe der Sitzung. Standard: erbt von Sitzung. Optionen: `low`, `medium`, `high`, `xhigh`, `max`; verfügbare Stufen hängen vom Modell ab.                                                                                                                                                                                                                                                                                                                |
| `context`                  | Nein         | Setzen Sie auf `fork`, um in einem verzweigten Subagent-Kontext ausgeführt zu werden.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `agent`                    | Nein         | Welcher Subagent-Typ verwendet werden soll, wenn `context: fork` gesetzt ist.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `hooks`                    | Nein         | Hooks, die auf den Lebenszyklus dieses Skills beschränkt sind. Siehe [Hooks in Skills und Agenten](/docs/de/hooks#hooks-in-skills-and-agents) für das Konfigurationsformat.                                                                                                                                                                                                                                                                                                                                                                                                       |
| `paths`                    | Nein         | Glob-Muster, die begrenzen, wann dieser Skill aktiviert wird. Akzeptiert eine kommagetrennte Zeichenkette oder eine YAML-Liste. Wenn gesetzt, lädt Claude den Skill automatisch nur, wenn mit Dateien arbeitet, die den Mustern entsprechen. Verwendet das gleiche Format wie [pfadspezifische Regeln](/docs/de/memory#path-specific-rules).                                                                                                                                                                                                                                      |
| `shell`                    | Nein         | Shell, die für `` !`command` `` und ` ```! ` Blöcke in diesem Skill verwendet werden soll. Akzeptiert `bash` (Standard) oder `powershell`. Das Setzen von `powershell` führt Inline-Shell-Befehle über PowerShell unter Windows aus. Erfordert `CLAUDE_CODE_USE_POWERSHELL_TOOL=1`.                                                                                                                                                                                                                                                                                          |

<h4 id="how-a-skill-gets-its-command-name">
  Wie ein Skill seinen Befehlsnamen erhält
</h4>

Der Befehl, den Sie eingeben, um einen Skill aufzurufen, kommt von dem Ort, an dem sich die Skill-Datei befindet. Das Frontmatter-Feld `name` setzt die Anzeigebeschriftung, die in Skill-Auflistungen angezeigt wird, und ändert außer bei einem Plugin-Root-`SKILL.md` nicht, was Sie nach `/` eingeben.

Die folgende Tabelle zeigt, woher der Befehlsname für jedes Layout kommt:

| Skill-Speicherort                                                                                                     | Befehlsnamen-Quelle                                                                 | Beispiel                                                                                                                                |
| :-------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------- |
| Skill-Verzeichnis unter `~/.claude/skills/` oder `.claude/skills/`                                                    | Verzeichnisname                                                                     | `.claude/skills/deploy-staging/SKILL.md` → `/deploy-staging`                                                                            |
| [Verschachteltes](#where-skills-live) `.claude/skills/`-Verzeichnis, wenn der Name mit einem anderen Skill kollidiert | Unterverzeichnispfad relativ zum Arbeitsverzeichnis, dann der Skill-Verzeichnisname | `apps/web/.claude/skills/deploy/SKILL.md` → `/apps/web:deploy`                                                                          |
| Datei unter `.claude/commands/`                                                                                       | Dateiname ohne Erweiterung                                                          | `.claude/commands/deploy.md` → `/deploy`                                                                                                |
| Plugin-`skills/`-Unterverzeichnis                                                                                     | Verzeichnisname, mit Namespace durch Plugin                                         | `my-plugin/skills/review/SKILL.md` → `/my-plugin:review`                                                                                |
| Plugin-Root-`SKILL.md`                                                                                                | Frontmatter `name`, mit dem Plugin-Verzeichnisnamen als Fallback                    | `my-plugin/SKILL.md` mit `name: review` → `/my-plugin:review`. Siehe [Pfad-Verhaltensregeln](/docs/de/plugins-reference#path-behavior-rules) |

Der Plugin-Root-Fall ist der einzige Ort, an dem `name` den Befehlsnamen setzt, da es kein Skill-Verzeichnis gibt, das ihn übernehmen könnte. Wenn `name` nicht im Frontmatter gesetzt ist, wird stattdessen der Plugin-Verzeichnisname verwendet.

<h4 id="available-string-substitutions">
  Verfügbare String-Substitutionen
</h4>

Skills unterstützen String-Substitution für dynamische Werte im Skill-Inhalt:

| Variable                | Beschreibung                                                                                                                                                                                                                                                                                                                                 |
| :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$ARGUMENTS`            | Alle Argumente, die beim Aufrufen des Skills übergeben werden. Wenn `$ARGUMENTS` nicht im Inhalt vorhanden ist, werden Argumente als `ARGUMENTS: <value>` angehängt.                                                                                                                                                                         |
| `$ARGUMENTS[N]`         | Greifen Sie auf ein bestimmtes Argument nach 0-basiertem Index zu, z. B. `$ARGUMENTS[0]` für das erste Argument.                                                                                                                                                                                                                             |
| `$N`                    | Kurzform für `$ARGUMENTS[N]`, z. B. `$0` für das erste Argument oder `$1` für das zweite.                                                                                                                                                                                                                                                    |
| `$name`                 | Benanntes Argument, das in der [`arguments`](#frontmatter-reference)-Frontmatter-Liste deklariert ist. Namen werden in Reihenfolge auf Positionen abgebildet, daher wird mit `arguments: [issue, branch]` der Platzhalter `$issue` zum ersten Argument erweitert und `$branch` zum zweiten.                                                  |
| `${CLAUDE_SESSION_ID}`  | Die aktuelle Sitzungs-ID. Nützlich zum Protokollieren, Erstellen sitzungsspezifischer Dateien oder Korrelieren der Skill-Ausgabe mit Sitzungen.                                                                                                                                                                                              |
| `${CLAUDE_EFFORT}`      | Die aktuelle Anstrengungsstufe: `low`, `medium`, `high`, `xhigh` oder `max`. Ultracode ist keine separate Stufe und wird als `xhigh` gemeldet. Verwenden Sie dies, um Skill-Anweisungen an die aktive Anstrengungseinstellung anzupassen.                                                                                                    |
| `${CLAUDE_SKILL_DIR}`   | Das Verzeichnis, das die `SKILL.md`-Datei des Skills enthält. Für Plugin-Skills ist dies das Skill-Unterverzeichnis im Plugin, nicht das Plugin-Root. Verwenden Sie dies in Bash-Injektionsbefehlen, um auf Scripts oder Dateien zu verweisen, die mit dem Skill gebündelt sind, unabhängig vom aktuellen Arbeitsverzeichnis.                |
| `${CLAUDE_PROJECT_DIR}` | Das Projekt-Root-Verzeichnis. Dies ist der gleiche Pfad, den [Hooks](/docs/de/hooks#reference-scripts-by-path) und MCP-Server als `CLAUDE_PROJECT_DIR` erhalten. Verwenden Sie dies, um auf projektlokale Scripts oder Dateien zu verweisen, wie `${CLAUDE_PROJECT_DIR}/.claude/hooks/helper.sh`, unabhängig davon, wo der Skill installiert ist. |

Die `${CLAUDE_PROJECT_DIR}`-Substitution erfordert Claude Code v2.1.196 oder später. Sie gilt sowohl für den Skill-Text als auch für das [`allowed-tools`](#frontmatter-reference)-Frontmatter, sodass eine Berechtigungsregel wie `Bash(${CLAUDE_PROJECT_DIR}/scripts/lint.sh *)` zum gleichen Pfad aufgelöst wird, den der Skill-Text verwendet.

Indizierte Argumente verwenden Shell-ähnliche Anführungszeichen, daher müssen Sie mehrteilige Werte in Anführungszeichen setzen, um sie als einzelnes Argument zu übergeben. Zum Beispiel macht `/my-skill "hello world" second` `$0` zu `hello world` und `$1` zu `second`. Der `$ARGUMENTS`-Platzhalter wird immer zur vollständigen Argumentzeichenkette erweitert, wie eingegeben.

Um ein Literal `$` vor einer Ziffer, `ARGUMENTS` oder einem deklarierten Argumentnamen einzufügen, z. B. `$1.00` in Prosa, maskieren Sie es mit einem Backslash: `\$1.00`. Ein Backslash vor jedem anderen `$` wird unverändert gelassen. Nur ein einzelner Backslash direkt vor dem Token maskiert ihn. Ein doppelter Backslash wie `\\$1` lässt beide Backslashes an Ort und Stelle, und `$1` wird immer noch zum Argumentwert erweitert.

**Beispiel mit Substitutionen:**

```yaml theme={null}
---
name: session-logger
description: Log activity for this session
---

Log the following to logs/${CLAUDE_SESSION_ID}.log:

$ARGUMENTS
```

<h3 id="add-supporting-files">
  Unterstützende Dateien hinzufügen
</h3>

Skills können mehrere Dateien in ihrem Verzeichnis enthalten. Dies hält `SKILL.md` auf das Wesentliche konzentriert, während Claude detailliertes Referenzmaterial nur bei Bedarf abrufen kann. Große Referenzdokumente, API-Spezifikationen oder Beispielsammlungen müssen nicht jedes Mal geladen werden, wenn der Skill ausgeführt wird.

```text theme={null}
my-skill/
├── SKILL.md (required - overview and navigation)
├── reference.md (detailed API docs - loaded when needed)
├── examples.md (usage examples - loaded when needed)
└── scripts/
    └── helper.py (utility script - executed, not loaded)
```

Verweisen Sie auf unterstützende Dateien von `SKILL.md` aus, damit Claude weiß, was jede Datei enthält und wann sie geladen werden soll:

```markdown theme={null}
## Additional resources

- For complete API details, see [reference.md](reference.md)
- For usage examples, see [examples.md](examples.md)
```

<Tip>Halten Sie `SKILL.md` unter 500 Zeilen. Verschieben Sie detailliertes Referenzmaterial in separate Dateien.</Tip>

<h3 id="control-who-invokes-a-skill">
  Steuern Sie, wer einen Skill aufruft
</h3>

Standardmäßig können sowohl Sie als auch Claude jeden Skill aufrufen. Sie können `/skill-name` eingeben, um ihn direkt aufzurufen, und Claude kann ihn automatisch laden, wenn er für Ihr Gespräch relevant ist. Zwei Frontmatter-Felder ermöglichen es Ihnen, dies einzuschränken:

* **`disable-model-invocation: true`**: Nur Sie können den Skill aufrufen. Verwenden Sie dies für Workflows mit Nebenwirkungen oder die Sie zeitlich steuern möchten, wie `/commit`, `/deploy` oder `/send-slack-message`. Sie möchten nicht, dass Claude bereitstellt, weil Ihr Code bereit aussieht.

* **`user-invocable: false`**: Nur Claude kann den Skill aufrufen. Verwenden Sie dies für Hintergrundwissen, das nicht als Befehl umsetzbar ist. Ein `legacy-system-context`-Skill erklärt, wie ein altes System funktioniert. Claude sollte dies kennen, wenn es relevant ist, aber `/legacy-system-context` ist keine aussagekräftige Aktion für Benutzer.

Dieses Beispiel erstellt einen Deploy-Skill, den nur Sie auslösen können. Das `disable-model-invocation: true`-Feld verhindert, dass Claude ihn automatisch ausführt:

```yaml theme={null}
---
name: deploy
description: Deploy the application to production
disable-model-invocation: true
---

Deploy $ARGUMENTS to production:

1. Run the test suite
2. Build the application
3. Push to the deployment target
4. Verify the deployment succeeded
```

Hier ist, wie die beiden Felder Aufrufe und Kontextladung beeinflussen:

| Frontmatter                      | Sie können aufrufen | Claude kann aufrufen | Wann in Kontext geladen                                                            |
| :------------------------------- | :------------------ | :------------------- | :--------------------------------------------------------------------------------- |
| (Standard)                       | Ja                  | Ja                   | Beschreibung immer im Kontext, vollständiger Skill wird beim Aufrufen geladen      |
| `disable-model-invocation: true` | Ja                  | Nein                 | Beschreibung nicht im Kontext, vollständiger Skill wird geladen, wenn Sie aufrufen |
| `user-invocable: false`          | Nein                | Ja                   | Beschreibung immer im Kontext, vollständiger Skill wird beim Aufrufen geladen      |

<Note>
  In einer regulären Sitzung werden Skill-Beschreibungen in den Kontext geladen, damit Claude weiß, was verfügbar ist, aber vollständiger Skill-Inhalt wird nur beim Aufrufen geladen. [Subagenten mit vorgeladenen Skills](/docs/de/sub-agents#preload-skills-into-subagents) funktionieren anders: Der vollständige Skill-Inhalt wird beim Start eingespritzt.
</Note>

<h3 id="skill-content-lifecycle">
  Skill-Inhalts-Lebenszyklus
</h3>

Wenn Sie oder Claude einen Skill aufrufen, wird der gerenderte `SKILL.md`-Inhalt als einzelne Nachricht in das Gespräch eingegeben und bleibt dort für den Rest der Sitzung. Claude Code liest die Skill-Datei bei späteren Zügen nicht erneut, daher schreiben Sie Anleitung, die während einer Aufgabe gelten sollte, als stehende Anweisungen statt als einmalige Schritte.

Wenn Claude einen Skill erneut aufruft, dessen gerenderter Inhalt identisch mit der bereits im Kontext vorhandenen Kopie ist, fügt Claude Code eine kurze Notiz hinzu, dass der Skill bereits geladen ist, anstatt eine zweite Kopie des Inhalts zu erstellen. Wenn sich der gerenderte Inhalt unterscheidet, weil sich die Argumente geändert haben oder ein [dynamischer Kontext](#inject-dynamic-context)-Befehl neue Ausgabe erzeugt hat, hängt Claude Code den vollständigen Inhalt erneut an. Vor v2.1.202 hängte jeder erneute Aufruf eine weitere vollständige Kopie der Skill-Anweisungen an.

[Auto-Komprimierung](/docs/de/how-claude-code-works#when-context-fills-up) trägt aufgerufene Skills innerhalb eines Token-Budgets weiter. Wenn das Gespräch zusammengefasst wird, um Kontext freizugeben, hängt Claude Code die neueste Aufrufe jedes Skills nach der Zusammenfassung wieder an und behält die ersten 5.000 Token jedes Skills. Wieder angehängte Skills teilen sich ein kombiniertes Budget von 25.000 Token. Claude Code füllt dieses Budget ab dem zuletzt aufgerufenen Skill, sodass ältere Skills vollständig gelöscht werden können, wenn Sie viele in einer Sitzung aufgerufen haben.

Wenn ein Skill das Verhalten nach der ersten Antwort nicht mehr zu beeinflussen scheint, ist der Inhalt normalerweise immer noch vorhanden und das Modell wählt andere Tools oder Ansätze. Stärken Sie die `description` und Anweisungen des Skills, damit das Modell es weiterhin bevorzugt, oder verwenden Sie [Hooks](/docs/de/hooks), um Verhalten deterministisch zu erzwingen. Wenn der Skill groß ist oder Sie mehrere andere danach aufgerufen haben, rufen Sie ihn nach der Komprimierung erneut auf, um den vollständigen Inhalt wiederherzustellen.

<h3 id="pre-approve-tools-for-a-skill">
  Tools für einen Skill vorab genehmigen
</h3>

Das `allowed-tools`-Feld gewährt Berechtigung für die aufgelisteten Tools, während der Skill aktiv ist, sodass Claude sie verwenden kann, ohne Sie um Genehmigung zu bitten. Es schränkt nicht ein, welche Tools verfügbar sind: Jedes Tool bleibt aufrufbar, und Ihre [Berechtigungseinstellungen](/docs/de/permissions) regeln weiterhin Tools, die nicht aufgelistet sind.

Für Skills, die in das Verzeichnis `.claude/skills/` eines Projekts eingecheckt werden, tritt `allowed-tools` in Kraft, nachdem Sie den Workspace-Trust-Dialog für diesen Ordner akzeptiert haben, genauso wie Berechtigungsregeln in `.claude/settings.json`. Überprüfen Sie Projekt-Skills vor dem Vertrauen in ein Repository, da ein Skill sich selbst breiten Tool-Zugriff gewähren kann.

Dieser Skill lässt Claude Git-Befehle ohne Genehmigung pro Verwendung ausführen, wenn Sie ihn aufrufen:

```yaml theme={null}
---
name: commit
description: Stage and commit the current changes
disable-model-invocation: true
allowed-tools: Bash(git add *) Bash(git commit *) Bash(git status *)
---
```

Um Tools aus Claudes verfügbarem Pool zu entfernen, während ein Skill aktiv ist, listen Sie sie in `disallowed-tools` im Frontmatter des Skills auf. Die Einschränkung wird gelöscht, wenn Sie Ihre nächste Nachricht senden. Um Tools über alle Skills und Eingaben hinweg zu blockieren, fügen Sie Ablehnungsregeln in Ihren [Berechtigungseinstellungen](/docs/de/permissions) hinzu.

<h3 id="pass-arguments-to-skills">
  Argumente an Skills übergeben
</h3>

Sowohl Sie als auch Claude können Argumente beim Aufrufen eines Skills übergeben. Argumente sind über den `$ARGUMENTS`-Platzhalter verfügbar.

Dieser Skill behebt ein GitHub-Problem nach Nummer. Der `$ARGUMENTS`-Platzhalter wird durch alles ersetzt, was dem Skill-Namen folgt:

```yaml theme={null}
---
name: fix-issue
description: Fix a GitHub issue
disable-model-invocation: true
---

Fix GitHub issue $ARGUMENTS following our coding standards.

1. Read the issue description
2. Understand the requirements
3. Implement the fix
4. Write tests
5. Create a commit
```

Wenn Sie `/fix-issue 123` ausführen, erhält Claude „Fix GitHub issue 123 following our coding standards..."

Wenn Sie einen Skill mit Argumenten aufrufen, aber der Skill `$ARGUMENTS` nicht enthält, hängt Claude Code `ARGUMENTS: <your input>` am Ende des Skill-Inhalts an, damit Claude immer noch sieht, was Sie eingegeben haben.

Sie können auch mehrere Skills am Anfang einer Nachricht stapeln. Ab v2.1.199 lädt das Eingeben von `/code-review /fix-issue 123` beide Skills und übergibt den nachfolgenden Text `123` als `$ARGUMENTS` an jeden von ihnen. In früheren Versionen wurde nur der erste Skill geladen und erhielt `/fix-issue 123` als wörtlichen Argumenttext.

Claude Code erweitert den ersten Skill plus bis zu fünf weitere, die danach gestapelt sind. Die Erweiterung stoppt beim ersten Token, das kein Inline-Benutzer-aufgerufener Skill ist, daher endet auch ein Skill, der als [verzweigter Subagent](#run-skills-in-a-subagent) ausgeführt wird, oder einer, dessen Argumente selbst mit einem Schrägstrich-Befehl beginnen können, wie `/loop`, auch dort; dieses Token und alles danach werden zum Argumenttext für jeden erweiterten Skill.

Um auf einzelne Argumente nach Position zuzugreifen, verwenden Sie `$ARGUMENTS[N]` oder die kürzere Form `$N`:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one framework to another
---

Migrate the $ARGUMENTS[0] component from $ARGUMENTS[1] to $ARGUMENTS[2].
Preserve all existing behavior and tests.
```

Wenn Sie `/migrate-component SearchBar React Vue` ausführen, wird `$ARGUMENTS[0]` durch `SearchBar`, `$ARGUMENTS[1]` durch `React` und `$ARGUMENTS[2]` durch `Vue` ersetzt. Der gleiche Skill mit der `$N`-Kurzform:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one framework to another
---

Migrate the $0 component from $1 to $2.
Preserve all existing behavior and tests.
```

<h2 id="advanced-patterns">
  Fortgeschrittene Muster
</h2>

<h3 id="inject-dynamic-context">
  Dynamischen Kontext einspritzen
</h3>

Die `` !`<command>` `` Syntax führt Shell-Befehle aus, bevor der Skill-Inhalt an Claude gesendet wird. Die Befehlsausgabe ersetzt den Platzhalter, sodass Claude tatsächliche Daten erhält, nicht den Befehl selbst.

Dieser Skill fasst einen Pull Request zusammen, indem er Live-PR-Daten mit der GitHub CLI abruft. Die `` !`gh pr diff` `` und andere Befehle werden zuerst ausgeführt, und ihre Ausgabe wird in den Prompt eingefügt:

```yaml theme={null}
---
name: pr-summary
description: Summarize changes in a pull request
context: fork
agent: Explore
allowed-tools: Bash(gh *)
---

## Pull request context
- PR diff: !`gh pr diff`
- PR comments: !`gh pr view --comments`
- Changed files: !`gh pr diff --name-only`

## Your task
Summarize this pull request...
```

Wenn dieser Skill ausgeführt wird:

1. Jeder `` !`<command>` `` wird sofort ausgeführt (bevor Claude etwas sieht)
2. Die Ausgabe ersetzt den Platzhalter im Skill-Inhalt
3. Claude erhält den vollständig gerenderten Prompt mit tatsächlichen PR-Daten

Dies ist Vorverarbeitung, nicht etwas, das Claude ausführt. Claude sieht nur das Endergebnis.

Die Substitution wird einmal über die ursprüngliche Datei ausgeführt. Die Befehlsausgabe wird als Klartext eingefügt und wird nicht erneut auf weitere `` !`<command>` `` Platzhalter gescannt, sodass ein Befehl keinen Platzhalter für einen späteren Durchgang ausgeben kann.

Die Inline-Form wird nur erkannt, wenn `!` am Anfang einer Zeile oder unmittelbar nach Leerzeichen erscheint. Wenn `!` auf ein anderes Zeichen folgt, wie in `` KEY=!`cmd` ``, wird der Platzhalter als Literaltext belassen und der Befehl wird nicht ausgeführt.

Für mehrzeilige Befehle verwenden Sie einen Codeblock, der mit ` ```! ` statt der Inline-Form geöffnet wird:

````markdown theme={null}
## Environment
```!
node --version
npm --version
git status --short
```
````

Um dieses Verhalten für Skills und benutzerdefinierte Befehle aus Benutzer-, Projekt-, Plugin- oder [zusätzlichen Verzeichnis](#skills-from-additional-directories)-Quellen zu deaktivieren, setzen Sie `"disableSkillShellExecution": true` in [Einstellungen](/docs/de/settings). Jeder Befehl wird stattdessen durch `[shell command execution disabled by policy]` ersetzt. Gebündelte und verwaltete Skills sind nicht betroffen. Diese Einstellung ist am nützlichsten in [verwalteten Einstellungen](/docs/de/permissions#managed-settings), wo Benutzer sie nicht überschreiben können.

<Tip>
  Um tiefere Überlegungen zu anfordern, wenn ein Skill ausgeführt wird, fügen Sie `ultrathink` irgendwo im Skill-Inhalt ein. Siehe [Verwenden Sie ultrathink für einmalige tiefe Überlegungen](/docs/de/model-config#use-ultrathink-for-one-off-deep-reasoning).
</Tip>

<h3 id="run-skills-in-a-subagent">
  Skills in einem Subagent ausführen
</h3>

Fügen Sie `context: fork` zu Ihrem Frontmatter hinzu, wenn Sie möchten, dass ein Skill isoliert ausgeführt wird. Der Skill-Inhalt wird zum Prompt, der den Subagent antreibt. Er hat keinen Zugriff auf Ihren Gesprächsverlauf.

<Warning>
  `context: fork` macht nur Sinn für Skills mit expliziten Anweisungen. Wenn Ihr Skill Richtlinien wie „verwenden Sie diese API-Konventionen" ohne eine Aufgabe enthält, erhält der Subagent die Richtlinien, aber keinen umsetzbaren Prompt, und gibt ohne aussagekräftige Ausgabe zurück.
</Warning>

Skills und [Subagenten](/docs/de/sub-agents) funktionieren in zwei Richtungen zusammen:

| Ansatz                     | System-Prompt          | Aufgabe                      | Lädt auch                                             |
| :------------------------- | :--------------------- | :--------------------------- | :---------------------------------------------------- |
| Skill mit `context: fork`  | Vom Agent-Typ          | SKILL.md-Inhalt              | CLAUDE.md, außer wenn der Agent Explore oder Plan ist |
| Subagent mit `skills`-Feld | Subagent-Markdown-Body | Claudes Delegationsnachricht | Vorgeladene Skills + CLAUDE.md                        |

Mit `context: fork` schreiben Sie die Aufgabe in Ihren Skill und wählen einen Agent-Typ aus, um sie auszuführen. Die integrierten Explore- und Plan-Agenten [überspringen CLAUDE.md und Git-Status](/docs/de/sub-agents#what-loads-at-startup), um ihren Kontext klein zu halten, sodass ein verzweigter Skill mit `agent: Explore` nur den SKILL.md-Inhalt und den eigenen System-Prompt des Agenten sieht. Für das Inverse, bei dem Sie einen benutzerdefinierten Subagenten definieren, der Skills als Referenzmaterial verwendet, siehe [Subagenten](/docs/de/sub-agents#preload-skills-into-subagents).

<h4 id="example-research-skill-using-explore-agent">
  Beispiel: Research-Skill mit Explore-Agent
</h4>

Dieser Skill führt Recherchen in einem verzweigten Explore-Agent aus. Der Skill-Inhalt wird zur Aufgabe, und der Agent bietet schreibgeschützte Tools, die für die Codebase-Erkundung optimiert sind:

```yaml theme={null}
---
name: deep-research
description: Research a topic thoroughly
context: fork
agent: Explore
---

Research $ARGUMENTS thoroughly:

1. Find relevant files using Glob and Grep
2. Read and analyze the code
3. Summarize findings with specific file references
```

Wenn dieser Skill ausgeführt wird:

1. Ein neuer isolierter Kontext wird erstellt
2. Der Subagent erhält den Skill-Inhalt als seinen Prompt („Research \$ARGUMENTS thoroughly...")
3. Das `agent`-Feld bestimmt die Ausführungsumgebung (Modell, Tools und Berechtigungen)
4. Ergebnisse werden zusammengefasst und an Ihr Hauptgespräch zurückgegeben

Das `agent`-Feld gibt an, welche Subagent-Konfiguration verwendet werden soll. Optionen umfassen integrierte Agenten (`Explore`, `Plan`, `general-purpose`) oder jeden benutzerdefinierten Subagenten aus `.claude/agents/`. Falls weggelassen, wird `general-purpose` verwendet.

<h3 id="restrict-claude’s-skill-access">
  Beschränken Sie Claudes Skill-Zugriff
</h3>

Standardmäßig kann Claude jeden Skill aufrufen, der nicht `disable-model-invocation: true` gesetzt hat. Skills, die `allowed-tools` definieren, gewähren Claude Zugriff auf diese Tools ohne Genehmigung pro Verwendung, wenn der Skill aktiv ist. Ihre [Berechtigungseinstellungen](/docs/de/permissions) regeln weiterhin das Baseline-Genehmigungsverhalten für alle anderen Tools. Einige integrierte Befehle sind auch über das Skill-Tool verfügbar, einschließlich `/init`, `/review` und `/security-review`. Andere integrierte Befehle wie `/compact` sind nicht verfügbar.

Drei Möglichkeiten, um zu steuern, welche Skills Claude aufrufen kann:

**Deaktivieren Sie alle Skills**, indem Sie das Skill-Tool in `/permissions` ablehnen:

```text theme={null}
# Add to deny rules:
Skill
```

**Erlauben oder verweigern Sie bestimmte Skills** mit [Berechtigungsregeln](/docs/de/permissions):

```text theme={null}
# Allow only specific skills
Skill(commit)
Skill(review-pr *)

# Deny specific skills
Skill(deploy *)
```

Berechtigungssyntax: `Skill(name)` für exakte Übereinstimmung, `Skill(name *)` für Präfixübereinstimmung mit beliebigen Argumenten.

**Verstecken Sie einzelne Skills**, indem Sie `disable-model-invocation: true` zu ihrem Frontmatter hinzufügen. Dies entfernt den Skill vollständig aus Claudes Kontext.

<Note>
  Das `user-invocable`-Feld steuert nur die Menüsichtbarkeit, nicht den Skill-Tool-Zugriff. Verwenden Sie `disable-model-invocation: true`, um die programmgesteuerte Aufrufe zu blockieren.
</Note>

<h3 id="override-skill-visibility-from-settings">
  Skill-Sichtbarkeit aus Einstellungen überschreiben
</h3>

Die `skillOverrides`-Einstellung steuert die Skill-Sichtbarkeit aus Ihren [Einstellungen](/docs/de/settings) statt aus dem Frontmatter des Skills selbst. Verwenden Sie sie für Skills, deren SKILL.md Sie nicht bearbeiten möchten, z. B. solche, die in ein gemeinsames Projekt-Repository eingecheckt sind oder von einem MCP-Server bereitgestellt werden. Das `/skills`-Menü schreibt es für Sie: Markieren Sie einen Skill und drücken Sie `Space`, um die Zustände zu durchlaufen, dann `Enter`, um in `.claude/settings.local.json` zu speichern.

Jeder Schlüssel ist ein Skill-Name und jeder Wert ist einer von vier Zuständen:

| Wert                    | Aufgelistet für Claude | Im `/`-Menü |
| :---------------------- | :--------------------- | :---------- |
| `"on"`                  | Name und Beschreibung  | Ja          |
| `"name-only"`           | Nur Name               | Ja          |
| `"user-invocable-only"` | Versteckt              | Ja          |
| `"off"`                 | Versteckt              | Versteckt   |

Ab v2.1.199 versteckt `"off"` den Skill auch vor den Befehlslisten, die an [Remote Control](/docs/de/remote-control)-Clients und an [Agent SDK](/docs/de/agent-sdk/slash-commands)-Aufrufer angekündigt werden, nicht nur das Terminal-Menü `/`. Das Aufrufen eines versteckten Skills nach seinem vollständigen Namen gibt stattdessen den `skillOverrides`-Fehler zurück, anstatt ihn auszuführen.

Ein Skill, der in `skillOverrides` fehlt, wird als `"on"` behandelt. Das folgende Beispiel reduziert einen Skill auf seinen Namen und deaktiviert einen anderen vollständig:

```json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

Plugin-Skills sind nicht von `skillOverrides` betroffen. Verwalten Sie diese stattdessen über `/plugin`.

<h2 id="evaluate-and-iterate-on-a-skill">
  Evaluieren und iterieren Sie einen Skill
</h2>

Zu sehen, dass ein Skill ausgelöst wird, sagt Ihnen, dass Claude ihn gefunden hat, nicht dass er das tat, was Sie beabsichtigten. Um zu wissen, dass ein Skill funktioniert, messen Sie zwei Dinge separat: ob Claude ihn auf den Eingaben aufruft, die er sollte, und ob die Ausgabe dem entspricht, was Sie erwarten, wenn er es tut.

Die Überprüfung für beide ist ein Baseline-Vergleich. Sammeln Sie ein paar realistische Eingaben, führen Sie jede in einer neuen Sitzung mit dem verfügbaren Skill aus und erneut mit ihm [deaktiviert](#override-skill-visibility-from-settings), und vergleichen Sie die Ergebnisse. Eine neue Sitzung ist wichtig, da übrig gebliebener Kontext aus der Erstellung des Skills Lücken in den geschriebenen Anweisungen maskiert.

<h3 id="run-evals-with-skill-creator">
  Führen Sie Evals mit skill-creator aus
</h3>

Das [`skill-creator`-Plugin](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/skill-creator) automatisiert die Vergleichsschleife in Claude Code. Installieren Sie es aus dem offiziellen Marketplace:

```text theme={null}
/plugin install skill-creator@claude-plugins-official
```

Wenn Claude Code meldet, dass das Plugin in keinem Marketplace gefunden wird, fehlt Ihr Marketplace entweder oder ist veraltet. Führen Sie `/plugin marketplace update claude-plugins-official` aus, um es zu aktualisieren, oder `/plugin marketplace add anthropics/claude-plugins-official`, wenn Sie es noch nicht hinzugefügt haben. Versuchen Sie dann erneut zu installieren.

Nach der Installation führen Sie `/reload-plugins` aus, um die Skills des Plugins in der aktuellen Sitzung verfügbar zu machen. Bitten Sie dann Claude, einen vorhandenen Skill zu evaluieren, zum Beispiel `evaluate my summarize-changes skill with skill-creator`. Das Plugin führt Sie durch das Schreiben von Testfällen und führt die Schleife aus:

* **Testfälle**: speichert Eingaben, Eingabedateien und erwartetes Verhalten in `evals/evals.json` im Skill-Verzeichnis
* **Isolierte Läufe**: spawnt einen [Subagent](/docs/de/sub-agents) pro Testfall, sodass jeder Lauf mit einem sauberen Kontext beginnt, und zeichnet Token-Anzahl und Dauer auf
* **Bewertung**: überprüft jede Assertion gegen die Ausgabe und schreibt Pass oder Fail mit Beweis in `grading.json`
* **Benchmark**: aggregiert Pass-Rate, Zeit und Token für mit-Skill versus ohne-Skill in `benchmark.json`, sodass Sie die Pass-Rate-Verbesserung gegen den Token- und Zeit-Overhead vergleichen können
* **Versionsvergleich**: führt einen blinden A/B zwischen zwei Versionen des Skills durch, sodass Sie bestätigen können, dass eine Bearbeitung eine Verbesserung ist, bevor Sie sie committen
* **Beschreibungsabstimmung**: generiert sollte-auslösen und sollte-nicht-auslösen Eingaben, misst die Hit-Rate und schlägt Beschreibungsbearbeitungen vor, wenn der Skill auf den falschen Anfragen aktiviert wird
* **Review-Viewer**: öffnet einen HTML-Bericht, in dem Sie jede Ausgabe überprüfen und qualitatives Feedback aufzeichnen, das die nächste Iteration liest

Für das Eval-Dateiformat und den vollständigen Iterations-Workflow siehe [Evaluating skill output quality](https://agentskills.io/skill-creation/evaluating-skills) auf agentskills.io. Für Hintergrund zum Benchmark- und Vergleichsmodus siehe die [skill-creator-Ankündigung](https://claude.com/blog/improving-skill-creator-test-measure-and-refine-agent-skills).

<h2 id="share-skills">
  Skills teilen
</h2>

Skills können je nach Ihrer Zielgruppe in verschiedenen Bereichen verteilt werden:

* **Projekt-Skills**: Committen Sie `.claude/skills/` zur Versionskontrolle
* **Plugins**: Erstellen Sie ein `skills/`-Verzeichnis in Ihrem [Plugin](/docs/de/plugins)
* **Verwaltet**: Stellen Sie organisationsweit über [verwaltete Einstellungen](/docs/de/settings#settings-files) bereit

<h3 id="generate-visual-output">
  Visuelle Ausgabe generieren
</h3>

Skills können Scripts in jeder Sprache bündeln und ausführen, was Claude Funktionen gibt, die über das hinausgehen, was in einem einzelnen Prompt möglich ist. Ein leistungsstarkes Muster ist die Generierung visueller Ausgabe: interaktive HTML-Dateien, die in Ihrem Browser geöffnet werden, um Daten zu erkunden, zu debuggen oder Berichte zu erstellen.

Dieses Beispiel erstellt einen Codebase-Explorer: eine interaktive Baumansicht, in der Sie Verzeichnisse erweitern und reduzieren, Dateigröße auf einen Blick sehen und Dateitypen nach Farbe identifizieren können.

Erstellen Sie das Skill-Verzeichnis:

```bash theme={null}
mkdir -p ~/.claude/skills/codebase-visualizer/scripts
```

Speichern Sie dies unter `~/.claude/skills/codebase-visualizer/SKILL.md`. Die Beschreibung teilt Claude mit, wann dieser Skill aktiviert werden soll, und die Anweisungen teilen Claude mit, das gebündelte Script auszuführen. Der Script-Pfad verwendet [`${CLAUDE_SKILL_DIR}`](#available-string-substitutions), damit er korrekt aufgelöst wird, unabhängig davon, ob der Skill auf persönlicher, Projekt- oder Plugin-Ebene installiert ist:

````yaml theme={null}
---
name: codebase-visualizer
description: Generate an interactive collapsible tree visualization of your codebase. Use when exploring a new repo, understanding project structure, or identifying large files.
allowed-tools: Bash(python3 *)
---

# Codebase Visualizer

Generate an interactive HTML tree view that shows your project's file structure with collapsible directories.

## Usage

Run the visualization script from your project root:

```bash
python3 ${CLAUDE_SKILL_DIR}/scripts/visualize.py .
```

This creates `codebase-map.html` in the current directory and opens it in your default browser.

## What the visualization shows

- **Collapsible directories**: Click folders to expand/collapse
- **File sizes**: Displayed next to each file
- **Colors**: Different colors for different file types
- **Directory totals**: Shows aggregate size of each folder
````

Speichern Sie dies unter `~/.claude/skills/codebase-visualizer/scripts/visualize.py`. Dieses Script scannt einen Verzeichnisbaum und generiert eine eigenständige HTML-Datei mit:

* Eine **Zusammenfassungs-Seitenleiste**, die Dateianzahl, Verzeichnisanzahl, Gesamtgröße und Anzahl der Dateitypen anzeigt
* Ein **Balkendiagramm**, das die Codebasis nach Dateityp aufschlüsselt (Top 8 nach Größe)
* Einen **zusammenklappbaren Baum**, in dem Sie Verzeichnisse erweitern und reduzieren können, mit farbcodierten Dateityp-Indikatoren

Das Script erfordert Python 3, verwendet aber nur integrierte Bibliotheken, daher müssen keine Pakete installiert werden:

```python expandable theme={null}
#!/usr/bin/env python3
"""Generate an interactive collapsible tree visualization of a codebase."""

import json
import sys
import webbrowser
from html import escape
from pathlib import Path
from collections import Counter

IGNORE = {'.git', 'node_modules', '__pycache__', '.venv', 'venv', 'dist', 'build'}

def scan(path: Path, stats: dict) -> dict:
    result = {"name": path.name, "children": [], "size": 0}
    try:
        for item in sorted(path.iterdir()):
            if item.name in IGNORE or item.name.startswith('.'):
                continue
            if item.is_file():
                size = item.stat().st_size
                ext = item.suffix.lower() or '(no ext)'
                result["children"].append({"name": item.name, "size": size, "ext": ext})
                result["size"] += size
                stats["files"] += 1
                stats["extensions"][ext] += 1
                stats["ext_sizes"][ext] += size
            elif item.is_dir():
                stats["dirs"] += 1
                child = scan(item, stats)
                if child["children"]:
                    result["children"].append(child)
                    result["size"] += child["size"]
    except PermissionError:
        pass
    return result

def generate_html(data: dict, stats: dict, output: Path) -> None:
    ext_sizes = stats["ext_sizes"]
    total_size = sum(ext_sizes.values()) or 1
    sorted_exts = sorted(ext_sizes.items(), key=lambda x: -x[1])[:8]
    colors = {
        '.js': '#f7df1e', '.ts': '#3178c6', '.py': '#3776ab', '.go': '#00add8',
        '.rs': '#dea584', '.rb': '#cc342d', '.css': '#264de4', '.html': '#e34c26',
        '.json': '#6b7280', '.md': '#083fa1', '.yaml': '#cb171e', '.yml': '#cb171e',
        '.mdx': '#083fa1', '.tsx': '#3178c6', '.jsx': '#61dafb', '.sh': '#4eaa25',
    }
    lang_bars = "".join(
        f'<div class="bar-row"><span class="bar-label">{ext}</span>'
        f'<div class="bar" style="width:{(size/total_size)*100}%;background:{colors.get(ext,"#6b7280")}"></div>'
        f'<span class="bar-pct">{(size/total_size)*100:.1f}%</span></div>'
        for ext, size in sorted_exts
    )
    def fmt(b):
        if b < 1024: return f"{b} B"
        if b < 1048576: return f"{b/1024:.1f} KB"
        return f"{b/1048576:.1f} MB"

    html = f'''<!DOCTYPE html>
<html><head>
  <meta charset="utf-8"><title>Codebase Explorer</title>
  <style>
    body {{ font: 14px/1.5 system-ui, sans-serif; margin: 0; background: #1a1a2e; color: #eee; }}
    .container {{ display: flex; height: 100vh; }}
    .sidebar {{ width: 280px; background: #252542; padding: 20px; border-right: 1px solid #3d3d5c; overflow-y: auto; flex-shrink: 0; }}
    .main {{ flex: 1; padding: 20px; overflow-y: auto; }}
    h1 {{ margin: 0 0 10px 0; font-size: 18px; }}
    h2 {{ margin: 20px 0 10px 0; font-size: 14px; color: #888; text-transform: uppercase; }}
    .stat {{ display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #3d3d5c; }}
    .stat-value {{ font-weight: bold; }}
    .bar-row {{ display: flex; align-items: center; margin: 6px 0; }}
    .bar-label {{ width: 55px; font-size: 12px; color: #aaa; }}
    .bar {{ height: 18px; border-radius: 3px; }}
    .bar-pct {{ margin-left: 8px; font-size: 12px; color: #666; }}
    .tree {{ list-style: none; padding-left: 20px; }}
    details {{ cursor: pointer; }}
    summary {{ padding: 4px 8px; border-radius: 4px; }}
    summary:hover {{ background: #2d2d44; }}
    .folder {{ color: #ffd700; }}
    .file {{ display: flex; align-items: center; padding: 4px 8px; border-radius: 4px; }}
    .file:hover {{ background: #2d2d44; }}
    .size {{ color: #888; margin-left: auto; font-size: 12px; }}
    .dot {{ width: 8px; height: 8px; border-radius: 50%; margin-right: 8px; }}
  </style>
</head><body>
  <div class="container">
    <div class="sidebar">
      <h1>📊 Summary</h1>
      <div class="stat"><span>Files</span><span class="stat-value">{stats["files"]:,}</span></div>
      <div class="stat"><span>Directories</span><span class="stat-value">{stats["dirs"]:,}</span></div>
      <div class="stat"><span>Total size</span><span class="stat-value">{fmt(data["size"])}</span></div>
      <div class="stat"><span>File types</span><span class="stat-value">{len(stats["extensions"])}</span></div>
      <h2>By file type</h2>
      {lang_bars}
    </div>
    <div class="main">
      <h1>📁 {escape(data["name"])}</h1>
      <ul class="tree" id="root"></ul>
    </div>
  </div>
  <script>
    const data = {json.dumps(data)};
    const colors = {json.dumps(colors)};
    function fmt(b) {{ if (b < 1024) return b + ' B'; if (b < 1048576) return (b/1024).toFixed(1) + ' KB'; return (b/1048576).toFixed(1) + ' MB'; }}
    function esc(s) {{ return s.replace(/[&<>"']/g, c => ({{"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}}[c])); }}
    function render(node, parent) {{
      if (node.children) {{
        const det = document.createElement('details');
        det.open = parent === document.getElementById('root');
        det.innerHTML = `<summary><span class="folder">📁 ${{esc(node.name)}}</span><span class="size">${{fmt(node.size)}}</span></summary>`;
        const ul = document.createElement('ul'); ul.className = 'tree';
        node.children.sort((a,b) => (b.children?1:0)-(a.children?1:0) || a.name.localeCompare(b.name));
        node.children.forEach(c => render(c, ul));
        det.appendChild(ul);
        const li = document.createElement('li'); li.appendChild(det); parent.appendChild(li);
      }} else {{
        const li = document.createElement('li'); li.className = 'file';
        li.innerHTML = `<span class="dot" style="background:${{colors[node.ext]||'#6b7280'}}"></span>${{esc(node.name)}}<span class="size">${{fmt(node.size)}}</span>`;
        parent.appendChild(li);
      }}
    }}
    data.children.forEach(c => render(c, document.getElementById('root')));
  </script>
</body></html>'''
    output.write_text(html)

if __name__ == '__main__':
    target = Path(sys.argv[1] if len(sys.argv) > 1 else '.').resolve()
    stats = {"files": 0, "dirs": 0, "extensions": Counter(), "ext_sizes": Counter()}
    data = scan(target, stats)
    out = Path('codebase-map.html')
    generate_html(data, stats, out)
    print(f'Generated {out.absolute()}')
    webbrowser.open(f'file://{out.absolute()}')
```

Um zu testen, öffnen Sie Claude Code in einem beliebigen Projekt und fragen Sie „Visualize this codebase." Claude führt das Script aus, generiert `codebase-map.html` und öffnet es in Ihrem Browser.

Dieses Muster funktioniert für jede visuelle Ausgabe: Abhängigkeitsgraphen, Test-Coverage-Berichte, API-Dokumentation oder Datenbankschema-Visualisierungen. Das gebündelte Script erledigt die schwere Arbeit, während Claude die Orchestrierung übernimmt.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="skill-not-triggering">
  Skill wird nicht ausgelöst
</h3>

Wenn Claude Ihren Skill nicht verwendet, wenn erwartet:

1. Überprüfen Sie, ob die Beschreibung Schlüsselwörter enthält, die Benutzer natürlicherweise sagen würden
2. Überprüfen Sie, ob der Skill in `What skills are available?` angezeigt wird
3. Versuchen Sie, Ihre Anfrage umzuformulieren, um die Beschreibung besser zu treffen
4. Rufen Sie ihn direkt mit `/skill-name` auf, wenn der Skill vom Benutzer aufgerufen werden kann

Wenn die Frontmatter-YAML fehlerhaft ist, lädt Claude Code den Skill-Body mit leeren Metadaten, sodass `/skill-name` immer noch funktioniert, aber Claude keine `description` zum Abgleichen hat. Führen Sie mit `--debug` aus, um den Parse-Fehler zu sehen.

<h3 id="skill-triggers-too-often">
  Skill wird zu oft ausgelöst
</h3>

Wenn Claude Ihren Skill verwendet, wenn Sie das nicht möchten:

1. Machen Sie die Beschreibung spezifischer
2. Fügen Sie `disable-model-invocation: true` hinzu, wenn Sie nur manuelle Aufrufe möchten

<h3 id="skill-descriptions-are-cut-short">
  Skill-Beschreibungen werden gekürzt
</h3>

Claude Code lädt eine Auflistung von Skill-Namen und Beschreibungen in den Kontext, damit Claude weiß, was verfügbar ist. Die Auflistung enthält immer jeden Skill-Namen, aber wenn Sie viele Skills haben, kürzt Claude Code Beschreibungen, um in das Zeichenbudget der Auflistung zu passen, was die Schlüsselwörter entfernen kann, die Claude benötigt, um Ihre Anfrage zu erfüllen. Das Budget skaliert bei 1% des Kontextfensters des Modells. Wenn die Auflistung überläuft, löscht Claude Code Beschreibungen beginnend mit den Skills, die Sie am wenigsten aufrufen, sodass die Skills, die Sie am meisten verwenden, ihren vollständigen Text behalten.

Führen Sie `/doctor` aus, um eine Schätzung der Kontextkosten der Auflistung und ihrer größten Beitragenden zu erhalten. Wenn die Auflistung ihr Budget überschreitet, schreibt Claude Code auch eine Warnung in das Debug-Protokoll, sichtbar mit [`--debug`](/docs/de/cli-reference#cli-flags).

Die Skills-Zeile in `/context` meldet die Größe der Auflistung nach Anwendung des Budgets, sodass sie dem entspricht, was das Modell erhält. Vor v2.1.196 zählte die Zeile den vollständigen Text jeder Beschreibung und konnte einen Wert anzeigen, der mehrmals größer als das konfigurierte Budget ist.

Um das Budget zu erhöhen, setzen Sie die Einstellung [`skillListingBudgetFraction`](/docs/de/settings#available-settings) (z. B. `0.02` = 2%) oder die Umgebungsvariable `SLASH_COMMAND_TOOL_CHAR_BUDGET` auf eine feste Zeichenanzahl. Um Budget für andere Skills freizugeben, setzen Sie Einträge mit niedriger Priorität auf `"name-only"` in [`skillOverrides`](#override-skill-visibility-from-settings), damit sie ohne Beschreibung aufgelistet werden. Sie können auch den Text `description` und `when_to_use` an der Quelle kürzen: Stellen Sie den wichtigsten Anwendungsfall an den Anfang, da der kombinierte Text jedes Eintrags unabhängig vom Budget auf 1.536 Zeichen begrenzt ist. Die Obergrenze ist mit [`skillListingMaxDescChars`](/docs/de/settings#available-settings) konfigurierbar.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* **[Debuggen Sie Ihre Konfiguration](/docs/de/debug-your-config)**: Diagnostizieren Sie, warum ein Skill nicht angezeigt oder ausgelöst wird
* **[Evaluating skill output quality](https://agentskills.io/skill-creation/evaluating-skills)**: das Eval-Dateiformat und Iterations-Workflow auf agentskills.io
* **[Skill authoring best practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices)**: Schreibanleitung, die über Claude-Produkte hinweg gilt
* **[Subagenten](/docs/de/sub-agents)**: Delegieren Sie Aufgaben an spezialisierte Agenten
* **[Plugins](/docs/de/plugins)**: Packen und verteilen Sie Skills mit anderen Erweiterungen
* **[Hooks](/docs/de/hooks)**: Automatisieren Sie Workflows um Tool-Ereignisse
* **[Memory](/docs/de/memory)**: Verwalten Sie CLAUDE.md-Dateien für persistenten Kontext
* **[Befehle](/docs/de/commands)**: Referenz für integrierte Befehle und gebündelte Skills
* **[Berechtigungen](/docs/de/permissions)**: Steuern Sie Tool- und Skill-Zugriff
* **[Claude Tag Skills](https://claude.com/docs/claude-tag/admins/skills-repo)**: Projekt-Skills, die in ein Repository übernommen wurden, werden auch geladen, wenn dieses Repository in einem Claude Tag-Kanal verwendet wird
