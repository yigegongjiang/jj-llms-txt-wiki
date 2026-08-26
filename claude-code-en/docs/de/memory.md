> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Wie Claude sich Ihr Projekt merkt

> Geben Sie Claude persistente Anweisungen mit CLAUDE.md-Dateien, und lassen Sie Claude automatisch Erkenntnisse mit Auto-Memory sammeln.

Jede Claude Code-Sitzung beginnt mit einem frischen Context Window. Zwei Mechanismen tragen Wissen über Sitzungen hinweg:

* **CLAUDE.md-Dateien**: Anweisungen, die Sie schreiben, um Claude persistenten Kontext zu geben
* **Auto-Memory**: Notizen, die Claude selbst basierend auf Ihren Korrektionen und Vorlieben schreibt

Diese Seite behandelt folgende Themen:

* [CLAUDE.md-Dateien schreiben und organisieren](#claude-md-files)
* [Regeln auf bestimmte Dateitypen beschränken](#organize-rules-with-claude/rules/) mit `.claude/rules/`
* [Auto-Memory konfigurieren](#auto-memory), damit Claude automatisch Notizen macht
* [Fehlerbehebung](#troubleshoot-memory-issues), wenn Anweisungen nicht befolgt werden

<h2 id="claude-md-vs-auto-memory">
  CLAUDE.md vs. Auto-Memory
</h2>

Claude Code hat zwei komplementäre Memory-Systeme. Beide werden zu Beginn jeder Konversation geladen. Claude behandelt sie als Kontext, nicht als erzwungene Konfiguration. Um eine Aktion unabhängig davon zu blockieren, was Claude entscheidet, verwenden Sie stattdessen einen [PreToolUse Hook](/docs/de/hooks-guide). Je spezifischer und prägnanter Ihre Anweisungen sind, desto konsistenter folgt Claude ihnen.

|                     | CLAUDE.md-Dateien                               | Auto-Memory                                                           |
| :------------------ | :---------------------------------------------- | :-------------------------------------------------------------------- |
| **Wer schreibt es** | Sie                                             | Claude                                                                |
| **Was es enthält**  | Anweisungen und Regeln                          | Erkenntnisse und Muster                                               |
| **Umfang**          | Projekt, Benutzer oder Organisation             | Pro Repository, gemeinsam über Worktrees hinweg                       |
| **Geladen in**      | Jede Sitzung                                    | Jede Sitzung (erste 200 Zeilen oder 25 KB)                            |
| **Verwenden für**   | Coding-Standards, Workflows, Projektarchitektur | Build-Befehle, Debugging-Erkenntnisse, Vorlieben, die Claude entdeckt |

Verwenden Sie CLAUDE.md-Dateien, wenn Sie Claudes Verhalten lenken möchten. Auto-Memory lässt Claude aus Ihren Korrektionen lernen, ohne manuelle Anstrengung.

Subagents können auch ihre eigene Auto-Memory pflegen. Weitere Informationen finden Sie unter [Subagent-Konfiguration](/docs/de/sub-agents#enable-persistent-memory).

<h2 id="claude-md-files">
  CLAUDE.md-Dateien
</h2>

CLAUDE.md-Dateien sind Markdown-Dateien, die Claude persistente Anweisungen für ein Projekt, Ihren persönlichen Workflow oder Ihre gesamte Organisation geben. Sie schreiben diese Dateien in Klartext; Claude liest sie zu Beginn jeder Sitzung.

<h3 id="when-to-add-to-claude-md">
  Wann sollte ich zu CLAUDE.md hinzufügen
</h3>

Behandeln Sie CLAUDE.md als den Ort, an dem Sie aufschreiben, was Sie sonst erneut erklären würden. Fügen Sie hinzu, wenn:

* Claude denselben Fehler ein zweites Mal macht
* Eine Code-Review etwas findet, das Claude über diese Codebasis hätte wissen sollen
* Sie tippen die gleiche Korrektur oder Klarstellung in den Chat, die Sie letzte Sitzung eingegeben haben
* Ein neuer Teamkollege den gleichen Kontext benötigen würde, um produktiv zu sein

Halten Sie es bei Fakten, die Claude in jeder Sitzung behalten sollte: Build-Befehle, Konventionen, Projektlayout, „immer X machen"-Regeln. Wenn ein Eintrag ein mehrstufiges Verfahren ist oder nur für einen Teil der Codebasis wichtig ist, verschieben Sie ihn stattdessen zu einem [Skill](/docs/de/skills) oder einer [pfadgebundenen Regel](#organize-rules-with-claude/rules/). Die [Funktionsübersicht](/docs/de/features-overview#build-your-setup-over-time) behandelt, wann Sie jeden Mechanismus verwenden.

<h3 id="choose-where-to-put-claude-md-files">
  Wählen Sie, wo Sie CLAUDE.md-Dateien ablegen
</h3>

CLAUDE.md-Dateien können sich an mehreren Orten befinden, jeder mit einem anderen Umfang. Die folgende Tabelle listet sie in Ladereihenfolge auf, vom breitesten Umfang zum spezifischsten, sodass eine Projektanweisung im Kontext nach einer Benutzeranweisung erscheint.

| Umfang                    | Ort                                                                                                                                                                     | Zweck                                                                | Anwendungsbeispiele                                                             | Geteilt mit                            |
| ------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------- | ------------------------------------------------------------------------------- | -------------------------------------- |
| **Verwaltete Richtlinie** | • macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md`<br />• Linux und WSL: `/etc/claude-code/CLAUDE.md`<br />• Windows: `C:\Program Files\ClaudeCode\CLAUDE.md` | Organisationsweite Anweisungen, verwaltet von IT/DevOps              | Unternehmens-Coding-Standards, Sicherheitsrichtlinien, Compliance-Anforderungen | Alle Benutzer in der Organisation      |
| **Benutzeranweisungen**   | `~/.claude/CLAUDE.md`                                                                                                                                                   | Persönliche Vorlieben für alle Projekte                              | Code-Styling-Vorlieben, persönliche Tooling-Shortcuts                           | Nur Sie (alle Projekte)                |
| **Projektanweisungen**    | `./CLAUDE.md` oder `./.claude/CLAUDE.md`                                                                                                                                | Team-gemeinsame Anweisungen für das Projekt                          | Projektarchitektur, Coding-Standards, häufige Workflows                         | Team-Mitglieder über Versionskontrolle |
| **Lokale Anweisungen**    | `./CLAUDE.local.md`                                                                                                                                                     | Persönliche projektspezifische Vorlieben; zu `.gitignore` hinzufügen | Ihre Sandbox-URLs, bevorzugte Testdaten                                         | Nur Sie (aktuelles Projekt)            |

CLAUDE.md- und CLAUDE.local.md-Dateien in der Verzeichnishierarchie über dem Arbeitsverzeichnis werden beim Start vollständig geladen. Dateien in Unterverzeichnissen werden bei Bedarf geladen, wenn Claude Dateien in diesen Verzeichnissen liest. Weitere Informationen finden Sie unter [Wie CLAUDE.md-Dateien geladen werden](#how-claude-md-files-load) für die vollständige Auflösungsreihenfolge.

Für große Projekte können Sie Anweisungen in themaspezifische Dateien aufteilen, indem Sie [Projektregeln](#organize-rules-with-claude/rules/) verwenden. Regeln ermöglichen es Ihnen, Anweisungen auf bestimmte Dateitypen oder Unterverzeichnisse zu beschränken.

<h3 id="set-up-a-project-claude-md">
  Richten Sie eine Projekt-CLAUDE.md ein
</h3>

Eine Projekt-CLAUDE.md kann entweder in `./CLAUDE.md` oder `./.claude/CLAUDE.md` gespeichert werden. Erstellen Sie diese Datei und fügen Sie Anweisungen hinzu, die für jeden gelten, der am Projekt arbeitet: Build- und Test-Befehle, Coding-Standards, architektonische Entscheidungen, Namenskonventionen und häufige Workflows. Diese Anweisungen werden über Versionskontrolle mit Ihrem Team geteilt, daher konzentrieren Sie sich auf projektweite Standards statt auf persönliche Vorlieben.

<Tip>
  Führen Sie `/init` aus, um automatisch eine Start-CLAUDE.md zu generieren. Claude analysiert Ihre Codebasis und erstellt eine Datei mit Build-Befehlen, Test-Anweisungen und Projektkonventionen, die es entdeckt. Wenn bereits eine CLAUDE.md vorhanden ist, schlägt `/init` Verbesserungen vor, statt sie zu überschreiben. Verfeinern Sie sie von dort aus mit Anweisungen, die Claude nicht selbst entdecken würde.

  Setzen Sie `CLAUDE_CODE_NEW_INIT=1`, um einen interaktiven mehrstufigen Ablauf zu aktivieren. `/init` fragt, welche Artefakte eingerichtet werden sollen: CLAUDE.md-Dateien, Skills und Hooks. Es erkundet dann Ihre Codebasis mit einem Subagent, füllt Lücken durch Folgefragen aus und präsentiert einen überprüfbaren Vorschlag, bevor Dateien geschrieben werden.
</Tip>

<h3 id="write-effective-instructions">
  Schreiben Sie effektive Anweisungen
</h3>

CLAUDE.md-Dateien werden zu Beginn jeder Sitzung in das Context Window geladen und verbrauchen Token zusammen mit Ihrer Konversation. Die [Context Window-Visualisierung](/docs/de/context-window) zeigt, wo CLAUDE.md relativ zum Rest des Startup-Kontexts geladen wird. Da sie Kontext statt erzwungene Konfiguration sind, beeinflusst die Art, wie Sie Anweisungen schreiben, wie zuverlässig Claude ihnen folgt. Spezifische, prägnante, gut strukturierte Anweisungen funktionieren am besten.

**Größe**: Ziel unter 200 Zeilen pro CLAUDE.md-Datei. Längere Dateien verbrauchen mehr Kontext und reduzieren die Einhaltung. Wenn Ihre Anweisungen zu groß werden, verwenden Sie [pfadgebundene Regeln](#path-specific-rules), damit Anweisungen nur geladen werden, wenn Claude mit übereinstimmenden Dateien arbeitet, um Rauschen zu reduzieren und Kontextraum zu sparen. Sie können auch Inhalte in [Importe](#import-additional-files) aufteilen, um sie zu organisieren, obwohl importierte Dateien immer noch geladen werden und beim Start in das Context Window eingehen.

**Struktur**: Verwenden Sie Markdown-Header und Aufzählungszeichen, um verwandte Anweisungen zu gruppieren. Claude scannt die Struktur genauso wie Leser: organisierte Abschnitte sind leichter zu befolgen als dichte Absätze.

**Spezifität**: Schreiben Sie Anweisungen, die konkret genug sind, um überprüft zu werden. Zum Beispiel:

* „Verwenden Sie 2-Leerzeichen-Einrückung" statt „Formatieren Sie Code ordnungsgemäß"
* „Führen Sie `npm test` vor dem Commit aus" statt „Testen Sie Ihre Änderungen"
* „API-Handler befinden sich in `src/api/handlers/`" statt „Halten Sie Dateien organisiert"

**Konsistenz**: Wenn zwei Regeln sich widersprechen, kann Claude eine willkürlich auswählen. Überprüfen Sie Ihre CLAUDE.md-Dateien, verschachtelte CLAUDE.md-Dateien in Unterverzeichnissen und [`.claude/rules/`](#organize-rules-with-claude/rules/) regelmäßig, um veraltete oder widersprüchliche Anweisungen zu entfernen. In Monorepos verwenden Sie [`claudeMdExcludes`](#exclude-specific-claude-md-files), um CLAUDE.md-Dateien von anderen Teams zu überspringen, die für Ihre Arbeit nicht relevant sind.

<h3 id="import-additional-files">
  Importieren Sie zusätzliche Dateien
</h3>

CLAUDE.md-Dateien können zusätzliche Dateien mit der Syntax `@path/to/import` importieren. Importierte Dateien werden erweitert und beim Start zusammen mit der CLAUDE.md, die sie referenziert, in den Kontext geladen.

Sowohl relative als auch absolute Pfade sind zulässig. Relative Pfade werden relativ zur Datei aufgelöst, die den Import enthält, nicht zum Arbeitsverzeichnis. Importierte Dateien können rekursiv andere Dateien importieren, mit einer maximalen Tiefe von vier Hops.

Das Parsing von Importen überspringt Markdown-Code-Spans und eingezäunte Code-Blöcke. Um einen Pfad in Ihrer CLAUDE.md zu erwähnen, ohne ihn zu importieren, wickeln Sie ihn in Backticks ein: Das Schreiben von `` `@README` `` hält den Text literal, während `@README` außerhalb von Backticks die Datei importiert.

Um eine README, package.json und einen Workflow-Leitfaden einzubeziehen, referenzieren Sie sie mit der `@`-Syntax überall in Ihrer CLAUDE.md:

```text theme={null}
Siehe @README für Projektübersicht und @package.json für verfügbare npm-Befehle für dieses Projekt.

# Zusätzliche Anweisungen
- Git-Workflow @docs/git-instructions.md
```

Für persönliche Vorlieben pro Projekt, die nicht in die Versionskontrolle eingecheckt werden sollten, erstellen Sie eine `CLAUDE.local.md` im Projektstammverzeichnis. Sie wird zusammen mit `CLAUDE.md` geladen und wird auf die gleiche Weise behandelt. Fügen Sie `CLAUDE.local.md` zu Ihrer `.gitignore` hinzu, damit sie nicht committed wird; das Ausführen von `/init` und das Auswählen der persönlichen Option tut dies für Sie.

Wenn Sie über mehrere Git-Worktrees desselben Repositories arbeiten, existiert eine gitignorierte `CLAUDE.local.md` nur in dem Worktree, in dem Sie sie erstellt haben. Um persönliche Anweisungen über Worktrees hinweg zu teilen, importieren Sie stattdessen eine Datei aus Ihrem Home-Verzeichnis:

```text theme={null}
# Individuelle Vorlieben
- @~/.claude/my-project-instructions.md
```

<Warning>
  Wenn Claude Code zum ersten Mal externe Importe in einem Projekt antrifft, zeigt es einen Genehmigungsdialog an, der die Dateien auflistet. Wenn Sie ablehnen, bleiben die Importe deaktiviert und der Dialog wird nicht erneut angezeigt.
</Warning>

Für einen strukturierteren Ansatz zur Organisation von Anweisungen siehe [`.claude/rules/`](#organize-rules-with-claude/rules/).

<h3 id="agents-md">
  AGENTS.md
</h3>

Claude Code liest `CLAUDE.md`, nicht `AGENTS.md`. Wenn Ihr Repository bereits `AGENTS.md` für andere Coding-Agenten verwendet, erstellen Sie eine `CLAUDE.md`, die es importiert, damit beide Tools die gleichen Anweisungen lesen, ohne sie zu duplizieren. Sie können auch Claude-spezifische Anweisungen unter dem Import hinzufügen. Claude lädt die importierte Datei beim Sitzungsstart und hängt dann den Rest an:

```markdown CLAUDE.md theme={null}
@AGENTS.md

## Claude Code

Verwenden Sie Plan Mode für Änderungen unter `src/billing/`.
```

Ein Symlink funktioniert auch, wenn Sie keine Claude-spezifischen Inhalte hinzufügen müssen:

```bash theme={null}
ln -s AGENTS.md CLAUDE.md
```

Unter Windows erfordert das Erstellen eines Symlinks Administratorrechte oder Developer Mode, daher verwenden Sie stattdessen den Import `@AGENTS.md`.

Das Ausführen von [`/init`](/docs/de/commands) in einem Repo, das bereits eine `AGENTS.md` hat, liest diese und integriert die relevanten Teile in die generierte `CLAUDE.md`. Es liest auch andere Tool-Konfigurationen wie `.cursorrules`, `.devin/rules/` und `.windsurfrules`.

<h3 id="how-claude-md-files-load">
  Wie CLAUDE.md-Dateien geladen werden
</h3>

Claude Code liest CLAUDE.md-Dateien, indem es die Verzeichnisstruktur von Ihrem aktuellen Arbeitsverzeichnis aus durchläuft und jedes Verzeichnis unterwegs auf `CLAUDE.md`- und `CLAUDE.local.md`-Dateien überprüft. Das bedeutet, wenn Sie Claude Code in `foo/bar/` ausführen, lädt es Anweisungen aus `foo/bar/CLAUDE.md`, `foo/CLAUDE.md` und allen `CLAUDE.local.md`-Dateien daneben.

Alle entdeckten Dateien werden in den Kontext verkettet, statt sich gegenseitig zu überschreiben. Innerhalb der Verzeichnishierarchie wird Inhalt vom Dateisystem-Root bis zu Ihrem Arbeitsverzeichnis geordnet. Für das Beispiel `foo/bar/` erscheint `foo/CLAUDE.md` im Kontext vor `foo/bar/CLAUDE.md`, sodass Anweisungen näher an dem Ort, an dem Sie Claude gestartet haben, zuletzt gelesen werden. Innerhalb jedes Verzeichnisses wird `CLAUDE.local.md` nach `CLAUDE.md` angehängt, sodass Ihre persönlichen Notizen das letzte sind, das Claude auf dieser Ebene liest.

Claude entdeckt auch `CLAUDE.md`- und `CLAUDE.local.md`-Dateien in Unterverzeichnissen unter Ihrem aktuellen Arbeitsverzeichnis. Statt sie beim Start zu laden, werden sie eingebunden, wenn Claude Dateien in diesen Unterverzeichnissen liest.

Wenn Sie in einem großen Monorepo arbeiten, in dem CLAUDE.md-Dateien anderer Teams aufgegriffen werden, verwenden Sie [`claudeMdExcludes`](#exclude-specific-claude-md-files), um sie zu überspringen. Für das vollständige Layout von Root- und Pro-Verzeichnis-CLAUDE.md-Dateien und Regeln siehe [Monorepos und große Repos](/docs/de/large-codebases).

Block-Level-HTML-Kommentare (`<!-- maintainer notes -->`) in CLAUDE.md-Dateien werden vor der Injektion in Claudes Kontext entfernt. Verwenden Sie sie, um Notizen für menschliche Betreuer zu hinterlassen, ohne Kontext-Token darauf zu verschwenden. Kommentare innerhalb von Code-Blöcken werden beibehalten. Wenn Sie eine CLAUDE.md-Datei direkt mit dem Read-Tool öffnen, bleiben Kommentare sichtbar.

<h4 id="load-from-additional-directories">
  Laden aus zusätzlichen Verzeichnissen
</h4>

Das Flag `--add-dir` gibt Claude Zugriff auf zusätzliche Verzeichnisse außerhalb Ihres Hauptarbeitsverzeichnisses. Standardmäßig werden CLAUDE.md-Dateien aus diesen Verzeichnissen nicht geladen.

Um auch Memory-Dateien aus zusätzlichen Verzeichnissen zu laden, setzen Sie die Umgebungsvariable `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`:

```bash theme={null}
CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 claude --add-dir ../shared-config
```

Dies lädt `CLAUDE.md`, `.claude/CLAUDE.md`, `.claude/rules/*.md` und `CLAUDE.local.md` aus dem zusätzlichen Verzeichnis. `CLAUDE.local.md` wird übersprungen, wenn Sie `local` aus [`--setting-sources`](/docs/de/cli-reference) ausschließen.

<h3 id="organize-rules-with-claude/rules/">
  Organisieren Sie Regeln mit `.claude/rules/`
</h3>

Für größere Projekte können Sie Anweisungen in mehrere Dateien mit dem Verzeichnis `.claude/rules/` organisieren. Dies hält Anweisungen modular und leichter für Teams zu pflegen. Regeln können auch [auf bestimmte Dateipfade beschränkt werden](#path-specific-rules), sodass sie nur in den Kontext geladen werden, wenn Claude mit übereinstimmenden Dateien arbeitet, was Rauschen reduziert und Kontextraum spart.

<Note>
  Regeln werden in jeder Sitzung oder beim Öffnen übereinstimmender Dateien in den Kontext geladen. Für aufgabenspezifische Anweisungen, die nicht ständig im Kontext sein müssen, verwenden Sie stattdessen [Skills](/docs/de/skills), die nur geladen werden, wenn Sie sie aufrufen oder wenn Claude bestimmt, dass sie für Ihren Prompt relevant sind.
</Note>

<h4 id="set-up-rules">
  Richten Sie Regeln ein
</h4>

Platzieren Sie Markdown-Dateien im Verzeichnis `.claude/rules/` Ihres Projekts. Jede Datei sollte ein Thema abdecken, mit einem beschreibenden Dateinamen wie `testing.md` oder `api-design.md`. Alle `.md`-Dateien werden rekursiv entdeckt, sodass Sie Regeln in Unterverzeichnisse wie `frontend/` oder `backend/` organisieren können:

```text theme={null}
your-project/
├── .claude/
│   ├── CLAUDE.md           # Hauptprojektanweisungen
│   └── rules/
│       ├── code-style.md   # Code-Style-Richtlinien
│       ├── testing.md      # Test-Konventionen
│       └── security.md     # Sicherheitsanforderungen
```

Regeln ohne [`paths`-Frontmatter](#path-specific-rules) werden beim Start mit der gleichen Priorität wie `.claude/CLAUDE.md` geladen.

<h4 id="path-specific-rules">
  Pfadspezifische Regeln
</h4>

Regeln können mit YAML-Frontmatter mit dem Feld `paths` auf bestimmte Dateien beschränkt werden. Diese bedingten Regeln gelten nur, wenn Claude mit Dateien arbeitet, die den angegebenen Mustern entsprechen.

```markdown theme={null}
---
paths:
  - "src/api/**/*.ts"
---

# API-Entwicklungsregeln

- Alle API-Endpunkte müssen Eingabevalidierung enthalten
- Verwenden Sie das Standard-Fehlerantwortformat
- Fügen Sie OpenAPI-Dokumentationskommentare ein
```

Regeln ohne ein `paths`-Feld werden bedingungslos geladen und gelten für alle Dateien. Pfadgebundene Regeln werden ausgelöst, wenn Claude Dateien liest, die dem Muster entsprechen, nicht bei jedem Tool-Einsatz. Ab v2.1.198 funktioniert das Matching auch, wenn Claude eine Datei über einen symverlinkten Pfad zum Projektverzeichnis erreicht, zum Beispiel in einem symverlinkten Checkout.

Verwenden Sie Glob-Muster im Feld `paths`, um Dateien nach Erweiterung, Verzeichnis oder einer beliebigen Kombination zu vergleichen:

| Muster                 | Passt zu                                          |
| ---------------------- | ------------------------------------------------- |
| `**/*.ts`              | Alle TypeScript-Dateien in jedem Verzeichnis      |
| `src/**/*`             | Alle Dateien unter dem Verzeichnis `src/`         |
| `*.md`                 | Markdown-Dateien im Projektstamm                  |
| `src/components/*.tsx` | React-Komponenten in einem bestimmten Verzeichnis |

Sie können mehrere Muster angeben und Klammer-Expansion verwenden, um mehrere Erweiterungen in einem Muster zu vergleichen:

```markdown theme={null}
---
paths:
  - "src/**/*.{ts,tsx}"
  - "lib/**/*.ts"
  - "tests/**/*.test.ts"
---
```

Glob-Syntax behandelt `[` als den Anfang eines Klammer-Ausdrucks wie `[abc]`. Ein Muster mit einem `[`, das nicht als Klammer-Ausdruck gelesen werden kann, wie `photos [2024/**`, ist ungültig: es passt zu nichts, und die anderen Muster der Regel funktionieren weiterhin. Um ein literales `[` in einem Dateinamen zu vergleichen, escapen Sie es als `photos \[2024/**`. Vor v2.1.207 führte ein ungültiges Muster dazu, dass das Read-Tool für jede Datei fehlschlug, gegen die die Regel ausgewertet wurde, statt zu nichts zu passen.

<h4 id="share-rules-across-projects-with-symlinks">
  Teilen Sie Regeln über Projekte hinweg mit Symlinks
</h4>

Das Verzeichnis `.claude/rules/` unterstützt Symlinks, sodass Sie einen gemeinsamen Satz von Regeln pflegen und in mehrere Projekte verlinken können. Symlinks werden aufgelöst und normal geladen, und zirkuläre Symlinks werden erkannt und elegant behandelt.

Dieses Beispiel verlinkt sowohl ein gemeinsames Verzeichnis als auch eine einzelne Datei:

```bash theme={null}
ln -s ~/shared-claude-rules .claude/rules/shared
ln -s ~/company-standards/security.md .claude/rules/security.md
```

<h4 id="user-level-rules">
  Benutzerebenen-Regeln
</h4>

Persönliche Regeln in `~/.claude/rules/` gelten für jedes Projekt auf Ihrem Computer. Verwenden Sie sie für Vorlieben, die nicht projektspezifisch sind:

```text theme={null}
~/.claude/rules/
├── preferences.md    # Ihre persönlichen Coding-Vorlieben
└── workflows.md      # Ihre bevorzugten Workflows
```

Benutzerebenen-Regeln werden vor Projektregeln geladen, was Projektregeln höhere Priorität gibt.

<h3 id="manage-claude-md-for-large-teams">
  Verwalten Sie CLAUDE.md für große Teams
</h3>

Für Organisationen, die Claude Code über Teams bereitstellen, können Sie Anweisungen zentralisieren und steuern, welche CLAUDE.md-Dateien geladen werden.

<h4 id="deploy-organization-wide-claude-md">
  Stellen Sie organisationsweite CLAUDE.md bereit
</h4>

Organisationen können eine zentral verwaltete CLAUDE.md bereitstellen, die für alle Benutzer auf einem Computer gilt. Diese Datei kann nicht durch individuelle Einstellungen ausgeschlossen werden.

<Steps>
  <Step title="Erstellen Sie die Datei am Ort der verwalteten Richtlinie">
    * macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md`
    * Linux und WSL: `/etc/claude-code/CLAUDE.md`
    * Windows: `C:\Program Files\ClaudeCode\CLAUDE.md`
  </Step>

  <Step title="Stellen Sie mit Ihrem Konfigurationsverwaltungssystem bereit">
    Verwenden Sie MDM, Group Policy, Ansible oder ähnliche Tools, um die Datei über Entwicklermaschinen zu verteilen. Weitere Informationen finden Sie unter [verwaltete Einstellungen](/docs/de/permissions#managed-settings) für andere organisationsweite Konfigurationsoptionen.
  </Step>
</Steps>

Der Schlüssel `claudeMd` ermöglicht es Ihnen, verwaltete CLAUDE.md-Inhalte direkt in `managed-settings.json` zu platzieren, statt eine separate Datei bereitzustellen.

**Umfang**: jede Claude Code-Sitzung auf dem Computer, in jedem Repository. Für Repository-spezifische Anleitung committen Sie stattdessen eine Projekt-CLAUDE.md.

**Vorrang**: gleich wie eine verwaltete CLAUDE.md-Datei. Wird vor Benutzer- und Projekt-CLAUDE.md geladen.

**Wo es berücksichtigt wird**: nur verwaltete und Richtlinien-Einstellungen. Das Setzen von `claudeMd` in Benutzer-, Projekt- oder lokalen Einstellungen hat keine Auswirkung.

Das folgende Beispiel fügt Verhaltensanweisungen direkt in eine verwaltete Einstellungsdatei ein:

```json theme={null}
{
  "claudeMd": "Always run `make lint` before committing.\nNever push directly to main."
}
```

Eine verwaltete CLAUDE.md und [verwaltete Einstellungen](/docs/de/settings#settings-files) dienen unterschiedlichen Zwecken. Verwenden Sie Einstellungen für technische Durchsetzung und CLAUDE.md für Verhaltensanleitung:

| Anliegen                                                | Konfigurieren in                                                  |
| :------------------------------------------------------ | :---------------------------------------------------------------- |
| Blockieren Sie bestimmte Tools, Befehle oder Dateipfade | Verwaltete Einstellungen: `permissions.deny`                      |
| Erzwingen Sie Sandbox-Isolation                         | Verwaltete Einstellungen: `sandbox.enabled`                       |
| Umgebungsvariablen und API-Provider-Routing             | Verwaltete Einstellungen: `env`                                   |
| Authentifizierungsmethode und Organisationssperre       | Verwaltete Einstellungen: `forceLoginMethod`, `forceLoginOrgUUID` |
| Code-Style und Qualitätsrichtlinien                     | Verwaltete CLAUDE.md                                              |
| Datenbehandlung und Compliance-Erinnerungen             | Verwaltete CLAUDE.md                                              |
| Verhaltensanweisungen für Claude                        | Verwaltete CLAUDE.md                                              |

Einstellungsregeln werden vom Client unabhängig davon durchgesetzt, was Claude entscheidet zu tun. CLAUDE.md-Anweisungen prägen Claudes Verhalten, sind aber keine harte Durchsetzungsebene.

<h4 id="exclude-specific-claude-md-files">
  Schließen Sie bestimmte CLAUDE.md-Dateien aus
</h4>

In großen Monorepos können Vorgänger-CLAUDE.md-Dateien Anweisungen enthalten, die für Ihre Arbeit nicht relevant sind. Die Einstellung `claudeMdExcludes` ermöglicht es Ihnen, bestimmte Dateien nach Pfad oder Glob-Muster zu überspringen.

Dieses Beispiel schließt eine CLAUDE.md auf oberster Ebene und ein Regelverzeichnis aus einem übergeordneten Ordner aus. Fügen Sie es zu `.claude/settings.local.json` hinzu, damit der Ausschluss lokal auf Ihrem Computer bleibt:

```json theme={null}
{
  "claudeMdExcludes": [
    "**/monorepo/CLAUDE.md",
    "/home/user/monorepo/other-team/.claude/rules/**"
  ]
}
```

Muster werden mit Glob-Syntax gegen absolute Dateipfade abgeglichen. Sie können `claudeMdExcludes` auf jeder [Einstellungsebene](/docs/de/settings#settings-files) konfigurieren: Benutzer, Projekt, lokal oder verwaltete Richtlinie. Arrays werden über Ebenen hinweg zusammengeführt.

CLAUDE.md-Dateien mit verwalteter Richtlinie können nicht ausgeschlossen werden. Dies stellt sicher, dass organisationsweite Anweisungen unabhängig von individuellen Einstellungen immer gelten.

<h2 id="auto-memory">
  Auto-Memory
</h2>

Auto-Memory lässt Claude Wissen über Sitzungen hinweg sammeln, ohne dass Sie etwas schreiben müssen. Claude speichert Notizen für sich selbst, während es arbeitet: Build-Befehle, Debugging-Erkenntnisse, Architektur-Notizen, Code-Style-Vorlieben und Workflow-Gewohnheiten. Claude speichert nicht jede Sitzung etwas. Es entscheidet, was es sich merken sollte, basierend darauf, ob die Information in einer zukünftigen Konversation nützlich wäre.

<h3 id="enable-or-disable-auto-memory">
  Aktivieren oder deaktivieren Sie Auto-Memory
</h3>

Auto-Memory ist standardmäßig aktiviert. Um es umzuschalten, öffnen Sie `/memory` in einer Sitzung und verwenden Sie den Auto-Memory-Schalter, oder setzen Sie `autoMemoryEnabled` in Ihren Projekteinstellungen:

```json theme={null}
{
  "autoMemoryEnabled": false
}
```

Um Auto-Memory über eine Umgebungsvariable zu deaktivieren, setzen Sie `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1`.

<h3 id="storage-location">
  Speicherort
</h3>

Jedes Projekt erhält sein eigenes Memory-Verzeichnis unter `~/.claude/projects/<project>/memory/`. Der Pfad `<project>` wird aus dem Git-Repository abgeleitet, sodass alle Worktrees und Unterverzeichnisse innerhalb desselben Repos ein Auto-Memory-Verzeichnis teilen. Außerhalb eines Git-Repos wird stattdessen das Projektstammverzeichnis verwendet.

Um Auto-Memory an einem anderen Ort zu speichern, setzen Sie `autoMemoryDirectory` in Ihrer `settings.json`. Es wird aus jedem [Einstellungsbereich](/docs/de/settings#settings-precedence) gelesen: Benutzer, Projekt, lokal, Richtlinie oder `--settings`.

```json theme={null}
{
  "autoMemoryDirectory": "~/my-custom-memory-dir"
}
```

Der Wert muss ein absoluter Pfad sein oder mit `~/` beginnen. Wenn er in der `.claude/settings.json` oder `.claude/settings.local.json` eines Projekts festgelegt ist, wird der Wert nur berücksichtigt, nachdem Sie den Workspace-Trust-Dialog für diesen Ordner akzeptiert haben. Dies ist das gleiche Gate, das Hooks regelt.

Das Verzeichnis enthält einen `MEMORY.md`-Einstiegspunkt und optionale Themadateien:

```text theme={null}
~/.claude/projects/<project>/memory/
├── MEMORY.md          # Prägnanter Index, geladen in jede Sitzung
├── debugging.md       # Detaillierte Notizen zu Debugging-Mustern
├── api-conventions.md # API-Design-Entscheidungen
└── ...                # Alle anderen Themadateien, die Claude erstellt
```

`MEMORY.md` fungiert als Index des Memory-Verzeichnisses. Claude liest und schreibt Dateien in diesem Verzeichnis während Ihrer Sitzung und verwendet `MEMORY.md`, um den Überblick zu behalten, was wo gespeichert ist.

Auto-Memory ist maschinenlokal. Alle Worktrees und Unterverzeichnisse innerhalb desselben Git-Repositories teilen ein Auto-Memory-Verzeichnis. Dateien werden nicht über Maschinen oder Cloud-Umgebungen hinweg geteilt.

<h3 id="how-it-works">
  Wie es funktioniert
</h3>

Die ersten 200 Zeilen von `MEMORY.md`, oder die ersten 25 KB, je nachdem, was zuerst erreicht wird, werden zu Beginn jeder Konversation geladen. Inhalte über diese Schwelle hinaus werden nicht beim Sitzungsstart geladen. Claude hält `MEMORY.md` prägnant, indem es detaillierte Notizen in separate Themadateien verschiebt.

Diese Grenze gilt nur für `MEMORY.md`. CLAUDE.md-Dateien werden unabhängig von der Länge vollständig geladen, obwohl kürzere Dateien bessere Einhaltung erzeugen.

Themadateien wie `debugging.md` oder `patterns.md` werden nicht beim Start geladen. Claude liest sie bei Bedarf mit seinen Standard-Datei-Tools, wenn es die Informationen benötigt.

Claude liest und schreibt Memory-Dateien während Ihrer Sitzung. Wenn Sie „Writing memory" oder „Recalled memory" in der Claude Code-Schnittstelle sehen, aktualisiert oder liest Claude aktiv aus `~/.claude/projects/<project>/memory/`.

<h3 id="audit-and-edit-your-memory">
  Überprüfen und bearbeiten Sie Ihr Memory
</h3>

Auto-Memory-Dateien sind einfaches Markdown, das Sie jederzeit bearbeiten oder löschen können. Führen Sie [`/memory`](#view-and-edit-with-%2Fmemory) aus, um Memory-Dateien innerhalb einer Sitzung zu durchsuchen und zu öffnen.

<h2 id="view-and-edit-with-/memory">
  Anzeigen und Bearbeiten mit `/memory`
</h2>

Der Befehl `/memory` listet alle CLAUDE.md-, CLAUDE.local.md- und Regelsdateien auf, die in Ihrer aktuellen Sitzung geladen sind, ermöglicht es Ihnen, Auto-Memory ein- oder auszuschalten, und bietet einen Link zum Öffnen des Auto-Memory-Ordners. Wählen Sie eine beliebige Datei aus, um sie in Ihrem Editor zu öffnen.

Wenn Sie Claude bitten, sich etwas zu merken, wie „immer pnpm verwenden, nicht npm" oder „denken Sie daran, dass die API-Tests eine lokale Redis-Instanz erfordern", speichert Claude es in Auto-Memory. Um Anweisungen stattdessen zu CLAUDE.md hinzuzufügen, bitten Sie Claude direkt, wie „fügen Sie dies zu CLAUDE.md hinzu", oder bearbeiten Sie die Datei selbst über `/memory`.

<h2 id="troubleshoot-memory-issues">
  Fehlerbehebung bei Memory-Problemen
</h2>

Dies sind die häufigsten Probleme mit CLAUDE.md und Auto-Memory, zusammen mit Schritten zum Debuggen.

<h3 id="claude-isn’t-following-my-claude-md">
  Claude folgt meiner CLAUDE.md nicht
</h3>

CLAUDE.md-Inhalte werden als Benutzernachricht nach dem System-Prompt bereitgestellt, nicht als Teil des System-Prompts selbst. Claude liest ihn und versucht, ihm zu folgen, aber es gibt keine Garantie für strikte Einhaltung, besonders bei vagen oder widersprüchlichen Anweisungen.

Zum Debuggen:

* Führen Sie `/memory` aus, um zu überprüfen, dass Ihre CLAUDE.md- und CLAUDE.local.md-Dateien geladen werden. Wenn eine Datei nicht aufgelistet ist, kann Claude sie nicht sehen.
* Überprüfen Sie, dass die relevante CLAUDE.md an einem Ort ist, der für Ihre Sitzung geladen wird (siehe [Wählen Sie, wo Sie CLAUDE.md-Dateien ablegen](#choose-where-to-put-claude-md-files)).
* Machen Sie Anweisungen spezifischer. „Verwenden Sie 2-Leerzeichen-Einrückung" funktioniert besser als „formatieren Sie Code schön".
* Suchen Sie nach widersprüchlichen Anweisungen über CLAUDE.md-Dateien hinweg. Wenn zwei Dateien unterschiedliche Anleitungen für das gleiche Verhalten geben, kann Claude eine willkürlich auswählen.

Wenn die Anweisung etwas ist, das an einem bestimmten Punkt ausgeführt werden muss, z. B. vor jedem Commit oder nach jeder Dateibearbeitung, schreiben Sie sie stattdessen als [Hook](/docs/de/hooks-guide). Hooks werden als Shell-Befehle bei festen Lebenszyklusereignissen ausgeführt und gelten unabhängig davon, was Claude entscheidet zu tun.

Für Anweisungen, die Sie auf System-Prompt-Ebene haben möchten, verwenden Sie [`--append-system-prompt`](/docs/de/cli-reference#system-prompt-flags). Dies muss bei jeder Invokation übergeben werden, daher ist es besser für Skripte und Automatisierung als für interaktive Nutzung geeignet.

<Tip>
  Verwenden Sie den [`InstructionsLoaded`-Hook](/docs/de/hooks#instructionsloaded), um genau zu protokollieren, welche Anweisungsdateien geladen sind, wann sie geladen werden und warum. Dies ist nützlich zum Debuggen von pfadspezifischen Regeln oder Lazy-Loading-Dateien in Unterverzeichnissen.
</Tip>

<h3 id="i-don’t-know-what-auto-memory-saved">
  Ich weiß nicht, was Auto-Memory gespeichert hat
</h3>

Führen Sie `/memory` aus und wählen Sie den Auto-Memory-Ordner aus, um zu durchsuchen, was Claude gespeichert hat. Alles ist einfaches Markdown, das Sie lesen, bearbeiten oder löschen können.

<h3 id="my-claude-md-is-too-large">
  Meine CLAUDE.md ist zu groß
</h3>

Dateien über 200 Zeilen verbrauchen mehr Kontext und können die Einhaltung reduzieren. Verwenden Sie [pfadgebundene Regeln](#path-specific-rules), um Anweisungen nur zu laden, wenn Claude mit übereinstimmenden Dateien arbeitet, oder trimmen Sie Inhalte, die nicht in jeder Sitzung benötigt werden. Das Aufteilen in [`@path`-Importe](#import-additional-files) hilft bei der Organisation, reduziert aber nicht den Kontext, da importierte Dateien beim Start geladen werden.

Die [`/doctor`](/docs/de/commands#all-commands)-Überprüfung schlägt Kürzungen für eine eingecheckte CLAUDE.md vor: Sie entfernt Inhalte, die Claude aus der Codebasis ableiten kann, wie Verzeichnislayouts, Abhängigkeitslisten und Architekturübersichten, und behält Fallstricke, Begründungen und Konventionen, die sich von Tool-Standardwerten unterscheiden. Die Trim-Überprüfung erfordert Claude Code v2.1.206 oder später.

<h3 id="instructions-seem-lost-after-/compact">
  Anweisungen scheinen nach `/compact` verloren zu gehen
</h3>

Projekt-Root-CLAUDE.md übersteht Komprimierung: Nach `/compact` liest Claude sie neu von der Festplatte und injiziert sie frisch in die Sitzung. Verschachtelte CLAUDE.md-Dateien in Unterverzeichnissen werden nicht automatisch erneut injiziert; sie werden neu geladen, wenn Claude das nächste Mal eine Datei in diesem Unterverzeichnis liest.

Wenn eine Anweisung nach der Komprimierung verschwunden ist, wurde sie entweder nur in der Konversation gegeben oder befindet sich in einer verschachtelten CLAUDE.md, die noch nicht neu geladen wurde. Fügen Sie Anweisungen, die nur in der Konversation gegeben wurden, zu CLAUDE.md hinzu, um sie über Sitzungen hinweg zu erhalten. Weitere Informationen finden Sie unter [Was übersteht Komprimierung](/docs/de/context-window#what-survives-compaction) für die vollständige Aufschlüsselung.

Weitere Informationen finden Sie unter [Schreiben Sie effektive Anweisungen](#write-effective-instructions) für Anleitungen zu Größe, Struktur und Spezifität.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Debuggen Sie Ihre Konfiguration](/docs/de/debug-your-config): Diagnostizieren Sie, warum CLAUDE.md oder Einstellungen nicht wirksam werden
* [Skills](/docs/de/skills): Verpacken Sie wiederholbare Workflows, die bei Bedarf geladen werden
* [Einstellungen](/docs/de/settings): Konfigurieren Sie Claude Code-Verhalten mit Einstellungsdateien
* [Subagent-Memory](/docs/de/sub-agents#enable-persistent-memory): Lassen Sie Subagents ihre eigene Auto-Memory pflegen
