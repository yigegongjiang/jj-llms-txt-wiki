> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Wie Claude sich Ihr Projekt merkt

> Geben Sie Claude persistente Anweisungen mit CLAUDE.md- oder AGENTS.md-Dateien, und lassen Sie Claude automatisch Erkenntnisse mit Auto-Memory sammeln.

Jede Claude Code-Sitzung beginnt mit einem frischen Context Window. Zwei Mechanismen tragen Wissen über Sitzungen hinweg:

* **CLAUDE.md-Dateien**: Anweisungen, die Sie schreiben, um Claude persistenten Kontext zu geben. Claude kann auch die [`AGENTS.md`-Dateien](#agents-md) eines Repositorys lesen, eigenständig oder zusammen mit CLAUDE.md
* **Auto-Memory**: Notizen, die Claude selbst basierend auf Ihren Korrektionen und Vorlieben schreibt

Diese Seite behandelt folgende Themen:

* [CLAUDE.md-Dateien schreiben und organisieren](#claude-md-files)
* [Ein vorhandenes AGENTS.md](#agents-md) als Ihre Projektanweisungen verwenden, eigenständig oder zusammen mit CLAUDE.md
* [Regeln auf bestimmte Dateitypen beschränken](#organize-rules-with-claude/rules/) mit `.claude/rules/`
* [Auto-Memory konfigurieren](#auto-memory), damit Claude automatisch Notizen macht
* [Fehlerbehebung](#troubleshoot-memory-issues), wenn Anweisungen nicht befolgt werden

<h2 id="claude-md-vs-auto-memory">
  CLAUDE.md vs. Auto-Memory
</h2>

Claude Code hat zwei komplementäre Memory-Systeme. Beide werden zu Beginn jeder Konversation geladen. Claude behandelt sie als Kontext, nicht als erzwungene Konfiguration. Um eine Aktion unabhängig davon zu blockieren, was Claude entscheidet, verwenden Sie stattdessen einen [PreToolUse Hook](/docs/de/hooks-guide). Je spezifischer und prägnanter Ihre Anweisungen sind, desto konsistenter folgt Claude ihnen.

|                     | CLAUDE.md-Dateien                               | Auto-Memory                                                                                                     |
| :------------------ | :---------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- |
| **Wer schreibt es** | Sie                                             | Claude                                                                                                          |
| **Was es enthält**  | Anweisungen und Regeln                          | Erkenntnisse und Muster                                                                                         |
| **Umfang**          | Projekt, Benutzer oder Organisation             | Pro Repository, gemeinsam über Worktrees hinweg                                                                 |
| **Geladen in**      | Jede Sitzung                                    | Jede Sitzung (erste 200 Zeilen oder 25 KB)                                                                      |
| **Verwenden für**   | Coding-Standards, Workflows, Projektarchitektur | Ihre Vorlieben, Korrektionen, die Sie Claude geben, Projektkontext, den Claude nicht aus dem Code ableiten kann |

Verwenden Sie CLAUDE.md-Dateien, wenn Sie Claudes Verhalten lenken möchten. Auto-Memory lässt Claude aus Ihren Korrektionen lernen, ohne manuelle Anstrengung.

Subagents können auch ihre eigene Auto-Memory pflegen. Weitere Informationen finden Sie unter [Subagent-Konfiguration](/docs/de/sub-agents#enable-persistent-memory).

<h2 id="claude-md-files">
  CLAUDE.md-Dateien
</h2>

CLAUDE.md-Dateien sind Markdown-Dateien, die Claude persistente Anweisungen für ein Projekt, Ihren persönlichen Arbeitsablauf oder Ihre gesamte Organisation geben. Sie schreiben diese Dateien in Klartext; Claude liest sie zu Beginn jeder Sitzung. Wenn Ihr Repository stattdessen `AGENTS.md` verwendet, siehe [AGENTS.md](#agents-md).

<h3 id="when-to-add-to-claude-md">
  Wann Sie zu CLAUDE.md hinzufügen
</h3>

Behandeln Sie CLAUDE.md als den Ort, an dem Sie aufschreiben, was Sie sonst erklären würden. Fügen Sie hinzu, wenn:

* Claude denselben Fehler ein zweites Mal macht
* Eine Code-Überprüfung etwas findet, das Claude über diese Codebasis hätte wissen sollen
* Sie dieselbe Korrektur oder Klarstellung in den Chat eingeben, die Sie in der letzten Sitzung eingegeben haben
* Ein neues Teammitglied denselben Kontext benötigen würde, um produktiv zu sein

Halten Sie es bei Fakten, die Claude in jeder Sitzung behalten sollte: Build-Befehle, Konventionen, Projektlayout, „immer X tun"-Regeln. Wenn ein Eintrag ein mehrstufiges Verfahren ist oder nur für einen Teil der Codebasis relevant ist, verschieben Sie ihn zu einem [Skill](/docs/de/skills) oder einer [pfadgebundenen Regel](#organize-rules-with-claude/rules/) statt. Die [Erweiterungsübersicht](/docs/de/features-overview#build-your-setup-over-time) behandelt, wann Sie jeden Mechanismus verwenden.

<h3 id="choose-where-to-put-claude-md-files">
  Wählen Sie, wo Sie CLAUDE.md-Dateien ablegen
</h3>

CLAUDE.md-Dateien können sich an mehreren Orten befinden, jeder mit einem anderen Umfang. Die folgende Tabelle listet sie in Ladereihenfolge auf, vom breitesten Umfang zum spezifischsten, sodass eine Projektanweisung im Kontext nach einer Benutzeranweisung erscheint.

| Umfang                    | Ort                                                                                                                                                                     | Zweck                                                                       | Anwendungsbeispiele                                                                | Geteilt mit                           |
| ------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- | ------------------------------------- |
| **Verwaltete Richtlinie** | • macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md`<br />• Linux und WSL: `/etc/claude-code/CLAUDE.md`<br />• Windows: `C:\Program Files\ClaudeCode\CLAUDE.md` | Organisationsweite Anweisungen, verwaltet von IT/DevOps                     | Unternehmens-Codierungsstandards, Sicherheitsrichtlinien, Compliance-Anforderungen | Alle Benutzer in der Organisation     |
| **Benutzeranweisungen**   | `~/.claude/CLAUDE.md`                                                                                                                                                   | Persönliche Voreinstellungen für alle Projekte                              | Code-Stilvoreinstellungen, persönliche Tooling-Verknüpfungen                       | Nur Sie (alle Projekte)               |
| **Projektanweisungen**    | `./CLAUDE.md` oder `./.claude/CLAUDE.md`. Siehe [AGENTS.md](#agents-md) für den Fall, dass `./AGENTS.md` stattdessen oder zusammen mit ihnen geladen wird               | Team-gemeinsame Anweisungen für das Projekt                                 | Projektarchitektur, Codierungsstandards, häufige Arbeitsabläufe                    | Teammitglieder über Versionskontrolle |
| **Lokale Anweisungen**    | `./CLAUDE.local.md`                                                                                                                                                     | Persönliche projektspezifische Voreinstellungen; zu `.gitignore` hinzufügen | Ihre Sandbox-URLs, bevorzugte Testdaten                                            | Nur Sie (aktuelles Projekt)           |

CLAUDE.md- und CLAUDE.local.md-Dateien in der Verzeichnishierarchie über dem Arbeitsverzeichnis werden beim Start geladen. Dateien in Unterverzeichnissen werden bei Bedarf geladen, wenn Claude Dateien in diesen Verzeichnissen liest. Siehe [Wie CLAUDE.md-Dateien geladen werden](#how-claude-md-files-load) für die vollständige Auflösungsreihenfolge.

Für große Projekte können Sie Anweisungen in themaspezifische Dateien aufteilen, indem Sie [Projektregeln](#organize-rules-with-claude/rules/) verwenden. Regeln ermöglichen es Ihnen, Anweisungen auf bestimmte Dateitypen oder Unterverzeichnisse zu beschränken.

<h3 id="set-up-a-project-claude-md">
  Richten Sie ein Projekt-CLAUDE.md ein
</h3>

Ein Projekt-CLAUDE.md kann entweder in `./CLAUDE.md` oder `./.claude/CLAUDE.md` gespeichert werden. Erstellen Sie diese Datei und fügen Sie Anweisungen hinzu, die für jeden gelten, der am Projekt arbeitet: Build- und Test-Befehle, Codierungsstandards, architektonische Entscheidungen, Namenskonventionen und häufige Arbeitsabläufe. Diese Anweisungen werden über Versionskontrolle mit Ihrem Team geteilt, daher konzentrieren Sie sich auf projektweite Standards statt auf persönliche Voreinstellungen. Um zu bestätigen, dass die Datei geladen wurde, führen Sie `/context` in einer Sitzung aus und überprüfen Sie die Liste unter **Memory-Dateien**.

<Tip>
  Führen Sie `/init` aus, um automatisch ein startendes CLAUDE.md zu generieren. Claude analysiert Ihre Codebasis und erstellt eine Datei mit Build-Befehlen, Test-Anweisungen und Projektkonventionen, die es entdeckt. Wenn bereits ein CLAUDE.md vorhanden ist, schlägt `/init` Verbesserungen vor, statt es zu überschreiben. Verfeinern Sie es von dort mit Anweisungen, die Claude nicht selbst entdecken würde.

  Für einen interaktiven mehrstufigen Ablauf setzen Sie die Umgebungsvariable `CLAUDE_CODE_NEW_INIT` auf `1`, bevor Sie `/init` ausführen. Setzen Sie sie in Ihrer Shell oder im `env`-Block einer Einstellungsdatei, wie in [Umgebungsvariablen setzen](/docs/de/env-vars#set-environment-variables) gezeigt. Mit dieser Einstellung fragt `/init`, welche Artefakte eingerichtet werden sollen: CLAUDE.md-Dateien, Skills und Hooks. Es erkundet dann Ihre Codebasis mit einem Subagenten, füllt Lücken durch Folgefragen aus und präsentiert einen überprüfbaren Vorschlag, bevor Dateien geschrieben werden. Die Variable ändert nur, wie `/init` ausgeführt wird, sodass Sie sie gesetzt lassen können.
</Tip>

<h3 id="write-effective-instructions">
  Schreiben Sie effektive Anweisungen
</h3>

CLAUDE.md-Dateien werden zu Beginn jeder Sitzung in das Kontextfenster geladen und verbrauchen Token zusammen mit Ihrer Konversation. Die [Kontextfenster-Visualisierung](/docs/de/context-window) zeigt, wo CLAUDE.md relativ zum Rest des Startkontexts geladen wird. Da es sich um Kontext statt um erzwungene Konfiguration handelt, beeinflusst die Art, wie Sie Anweisungen schreiben, wie zuverlässig Claude sie befolgt. Spezifische, prägnante, gut strukturierte Anweisungen funktionieren am besten.

**Größe**: Ziel unter 200 Zeilen pro CLAUDE.md-Datei. Längere Dateien verbrauchen mehr Kontext und verringern die Einhaltung. Wenn Ihre Anweisungen zu groß werden, verwenden Sie [pfadgebundene Regeln](#path-specific-rules), damit Anweisungen nur geladen werden, wenn Claude mit übereinstimmenden Dateien arbeitet. Sie können Inhalte auch in [Importe](#import-additional-files) aufteilen, um sie zu organisieren, obwohl importierte Dateien immer noch geladen werden und beim Start in das Kontextfenster eingehen.

**Struktur**: Verwenden Sie Markdown-Header und Aufzählungszeichen, um verwandte Anweisungen zu gruppieren. Claude scannt die Struktur genauso wie Leser: organisierte Abschnitte sind leichter zu folgen als dichte Absätze.

**Spezifität**: Schreiben Sie Anweisungen, die konkret genug sind, um überprüft zu werden. Zum Beispiel:

* „Verwenden Sie 2-Leerzeichen-Einzug" statt „Formatieren Sie Code ordnungsgemäß"
* „Führen Sie `npm test` vor dem Commit aus" statt „Testen Sie Ihre Änderungen"
* „API-Handler befinden sich in `src/api/handlers/`" statt „Halten Sie Dateien organisiert"

**Konsistenz**: Wenn zwei Regeln sich widersprechen, kann Claude eine willkürlich auswählen. Überprüfen Sie Ihre CLAUDE.md-Dateien, verschachtelte CLAUDE.md-Dateien in Unterverzeichnissen und [`.claude/rules/`](#organize-rules-with-claude/rules/) regelmäßig, um veraltete oder widersprüchliche Anweisungen zu entfernen. In Monorepos verwenden Sie [`claudeMdExcludes`](#exclude-specific-claude-md-files), um CLAUDE.md-Dateien von anderen Teams zu überspringen, die für Ihre Arbeit nicht relevant sind.

<h3 id="import-additional-files">
  Importieren Sie zusätzliche Dateien
</h3>

CLAUDE.md-Dateien können zusätzliche Dateien mit der Syntax `@path/to/import` importieren. Importierte Dateien werden erweitert und beim Start zusammen mit dem CLAUDE.md, das sie referenziert, in den Kontext geladen.

Sowohl relative als auch absolute Pfade sind zulässig. Relative Pfade werden relativ zur Datei aufgelöst, die den Import enthält, nicht zum Arbeitsverzeichnis. Importierte Dateien können rekursiv andere Dateien importieren, mit einer maximalen Tiefe von vier Hops.

Das Import-Parsing überspringt Markdown-Code-Spannweiten und eingezäunte Code-Blöcke. Um einen Pfad in Ihrem CLAUDE.md zu erwähnen, ohne ihn zu importieren, wickeln Sie ihn in Backticks ein: Das Schreiben von `` `@README` `` hält den Text literal, während `@README` außerhalb von Backticks die Datei importiert.

Um eine README, package.json und einen Workflow-Leitfaden einzubeziehen, referenzieren Sie sie mit `@`-Syntax überall in Ihrem CLAUDE.md:

```text theme={null}
Siehe @README für Projektübersicht und @package.json für verfügbare npm-Befehle für dieses Projekt.

# Zusätzliche Anweisungen
- Git-Workflow @docs/git-instructions.md
```

Für private projektspezifische Voreinstellungen, die nicht in die Versionskontrolle eingecheckt werden sollten, erstellen Sie ein `CLAUDE.local.md` im Projektstammverzeichnis. Es wird zusammen mit `CLAUDE.md` geladen und wird genauso behandelt. Fügen Sie `CLAUDE.local.md` zu Ihrer `.gitignore` hinzu, damit es nicht committed wird. Mit `CLAUDE_CODE_NEW_INIT=1` gesetzt, führt das Ausführen von `/init` und das Auswählen der persönlichen Option dies für Sie durch.

Wenn Sie über mehrere Git-Worktrees desselben Repositorys arbeiten, existiert ein gitignoriertes `CLAUDE.local.md` nur in dem Worktree, in dem Sie es erstellt haben. Um persönliche Anweisungen über Worktrees hinweg zu teilen, importieren Sie stattdessen eine Datei aus Ihrem Home-Verzeichnis:

```text theme={null}
# Individuelle Voreinstellungen
- @~/.claude/my-project-instructions.md
```

<Warning>
  Ein Import in einer projektweiten Memory-Datei ist extern, wenn sein Pfad außerhalb Ihres Arbeitsverzeichnisses aufgelöst wird, wie der Home-Verzeichnis-Import oben. Wenn Claude Code zum ersten Mal externe Importe in einem Projekt antrifft, zeigt es einen Genehmigungsdialog an, der die Dateien auflistet. Wenn Sie ablehnen, bleiben die Importe deaktiviert und der Dialog wird nicht erneut angezeigt.

  Claude Code zeigt den Dialog an, um Sie vor Dateien zu schützen, die andere Personen in ein gemeinsames Projekt committen. Benutzerbereichs-Memory-Dateien wie `~/.claude/CLAUDE.md` und `~/.claude/rules/` sind Dateien, die Sie selbst geschrieben haben. Außer in [Cowork](https://claude.com/product/cowork)-Sitzungen auf Ihrem Desktop lädt Claude Code ihre Importe ohne Dialog und vertraut ihnen wie dem Rest Ihrer persönlichen Konfiguration.

  In Cowork-Sitzungen auf Ihrem Desktop überspringt Claude Code jeden Import in einer Benutzerbereichs-Datei, die zu einem Pfad außerhalb des Arbeitsverzeichnisses der Sitzung aufgelöst wird, und lädt den Rest der Datei. In diesen Sitzungen überspringt es auch ein `~/.claude/CLAUDE.md`, das selbst ein Symlink oder Hard Link ist, und ein symverlinktes `~/.claude/rules/`-Verzeichnis oder eine Regeldatei, die außerhalb des Arbeitsverzeichnisses zeigt.
</Warning>

<h3 id="how-claude-md-files-load">
  Wie CLAUDE.md-Dateien geladen werden
</h3>

Claude Code lädt `CLAUDE.md` und `CLAUDE.local.md` aus Ihrem aktuellen Arbeitsverzeichnis und jedem Verzeichnis darüber. Führen Sie Claude Code in `foo/bar/` aus und es lädt Anweisungen aus `foo/bar/CLAUDE.md`, `foo/CLAUDE.md` und allen `CLAUDE.local.md`-Dateien daneben.

Alle entdeckten Dateien werden in den Kontext verkettet, statt sich gegenseitig zu überschreiben. Über den Verzeichnisbaum hinweg wird Inhalt vom Dateisystem-Root bis zu Ihrem Arbeitsverzeichnis geordnet. Für das Beispiel `foo/bar/` erscheint `foo/CLAUDE.md` im Kontext vor `foo/bar/CLAUDE.md`, sodass Anweisungen näher an dem Ort, an dem Sie Claude gestartet haben, zuletzt gelesen werden. Innerhalb jedes Verzeichnisses wird `CLAUDE.local.md` nach `CLAUDE.md` angehängt, sodass Ihre persönlichen Notizen das Letzte sind, das Claude auf dieser Ebene liest.

Claude entdeckt auch `CLAUDE.md`- und `CLAUDE.local.md`-Dateien in Unterverzeichnissen unter Ihrem aktuellen Arbeitsverzeichnis. Statt sie beim Start zu laden, werden sie eingebunden, wenn Claude Dateien in diesen Verzeichnissen liest.

Wenn Sie in einem großen Monorepo arbeiten, in dem CLAUDE.md-Dateien anderer Teams aufgegriffen werden, verwenden Sie [`claudeMdExcludes`](#exclude-specific-claude-md-files), um sie zu überspringen. Für das vollständige Layout von Root- und Pro-Verzeichnis-CLAUDE.md-Dateien und Regeln siehe [Monorepos und große Repos](/docs/de/large-codebases).

Block-Level-HTML-Kommentare (`<!-- maintainer notes -->`) in CLAUDE.md-Dateien werden vor dem Einfügen des Inhalts in Claudes Kontext entfernt. Verwenden Sie sie, um Notizen für menschliche Betreuer zu hinterlassen, ohne Kontext-Token darauf zu verschwenden. Kommentare in Code-Blöcken werden beibehalten. Wenn Sie eine CLAUDE.md-Datei direkt mit dem Read-Tool öffnen, bleiben Kommentare sichtbar.

<h4 id="load-from-additional-directories">
  Laden aus zusätzlichen Verzeichnissen
</h4>

Das Flag `--add-dir` gibt Claude Zugriff auf zusätzliche Verzeichnisse außerhalb Ihres Hauptarbeitsverzeichnisses. Standardmäßig werden CLAUDE.md-Dateien aus diesen Verzeichnissen nicht geladen.

Um auch Memory-Dateien aus zusätzlichen Verzeichnissen zu laden, setzen Sie die Umgebungsvariable `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`:

```bash theme={null}
CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 claude --add-dir ../shared-config
```

Die Inline-Form setzt die Variable für diesen einen Start in Bash oder Zsh. Um sie für jede Sitzung aktiviert zu halten, fügen Sie sie zum `env`-Block in `~/.claude/settings.json` hinzu, wie in [Umgebungsvariablen setzen](/docs/de/env-vars#set-environment-variables) gezeigt.

Dies lädt `CLAUDE.md`, `.claude/CLAUDE.md`, `.claude/rules/*.md` und `CLAUDE.local.md` aus dem zusätzlichen Verzeichnis. `CLAUDE.local.md` wird übersprungen, wenn Sie `local` aus [`--setting-sources`](/docs/de/cli-reference) ausschließen.

<h3 id="organize-rules-with-claude/rules/">
  Organisieren Sie Regeln mit `.claude/rules/`
</h3>

Für größere Projekte können Sie Anweisungen in mehrere Dateien mit dem Verzeichnis `.claude/rules/` organisieren. Dies hält Anweisungen modular und leichter für Teams zu verwalten. Regeln können auch [auf bestimmte Dateipfade beschränkt werden](#path-specific-rules), sodass sie nur in den Kontext geladen werden, wenn Claude mit übereinstimmenden Dateien arbeitet, was Rauschen reduziert und Kontextraum spart.

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
│       ├── code-style.md   # Code-Stilrichtlinien
│       ├── testing.md      # Test-Konventionen
│       └── security.md     # Sicherheitsanforderungen
```

Regeln ohne [`paths`-Frontmatter](#path-specific-rules) werden beim Start mit derselben Priorität wie `.claude/CLAUDE.md` geladen.

Projektregeln werden übersprungen, wenn Sie `project` aus [`--setting-sources`](/docs/de/cli-reference) ausschließen. Vor v2.1.211 wurden Regeln, die bei Bedarf geladen werden, einschließlich pfadgebundener Regeln und Regeln in verschachtelten `.claude/rules/`-Verzeichnissen, auch geladen, wenn `project` ausgeschlossen war.

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

Verwenden Sie Glob-Muster im Feld `paths`, um Dateien nach Erweiterung, Verzeichnis oder einer beliebigen Kombination zu entsprechen:

| Muster                 | Entspricht                                        |
| ---------------------- | ------------------------------------------------- |
| `**/*.ts`              | Alle TypeScript-Dateien in jedem Verzeichnis      |
| `src/**/*`             | Alle Dateien unter dem Verzeichnis `src/`         |
| `*.md`                 | Markdown-Dateien im Projektstammverzeichnis       |
| `src/components/*.tsx` | React-Komponenten in einem bestimmten Verzeichnis |

Sie können mehrere Muster angeben und Klammer-Erweiterung verwenden, um mehrere Erweiterungen in einem Muster zu entsprechen:

```markdown theme={null}
---
paths:
  - "src/**/*.{ts,tsx}"
  - "lib/**/*.ts"
  - "tests/**/*.test.ts"
---
```

Jede Klammer-Gruppe multipliziert die Anzahl der erweiterten Muster: `src/*.{ts,tsx}` wird zu zwei Mustern erweitert, und `{a,b}/{c,d}/*.{ts,tsx}` zu acht. Um die Erweiterung begrenzt zu halten, teilt sich die gesamte `paths`-Liste einer Regel ein Budget von 1.000 erweiterten Mustern und 4 MiB, und Muster ohne Klammern zählen nicht dagegen.

Claude Code verwendet jedes Muster, das das Budget unerweitert überschreiten würde, und seine literalen Klammern entsprechen keinen Dateien. Vor v2.1.217 stagnierte oder stürzte eine `paths`-Wert mit vielen Klammer-Gruppen die CLI beim Start ab.

Die Glob-Syntax behandelt `[` als den Anfang eines Klammer-Ausdrucks wie `[abc]`. Ein Muster mit einem `[`, das nicht als Klammer-Ausdruck gelesen werden kann, wie `photos [2024/**`, ist ungültig: es entspricht nichts, und die anderen Muster der Regel funktionieren weiterhin. Um ein literales `[` in einem Dateinamen zu entsprechen, escapen Sie es als `photos \[2024/**`. Vor v2.1.207 machte ein ungültiges Muster das Read-Tool für jede Datei fehlschlagen, gegen die die Regel bewertet wurde, statt nichts zu entsprechen.

<h4 id="rules-frontmatter-reference">
  Referenz für Regel-Frontmatter
</h4>

Konfigurieren Sie eine Regel mit YAML-[Frontmatter](/docs/de/glossary#frontmatter) zwischen `---`-Markierungen am Anfang der Datei. `paths` ist das einzige Feld, das Claude Code aus einer Regel liest; jedes andere Feld wird ohne Fehler ignoriert. Claude Code entfernt das Frontmatter, bevor die Regel in den Kontext geladen wird.

| Feld    | Erforderlich | Beschreibung                                                                                                                                                  |
| :------ | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `paths` | Nein         | Glob-Muster, die [die Regel auf übereinstimmende Dateien beschränken](#path-specific-rules). Akzeptiert eine YAML-Liste oder eine kommagetrennte Zeichenkette |

Wenn das YAML zwischen den Markierungen nicht analysiert wird, ignoriert Claude Code das Frontmatter und lädt die Regel so, als hätte sie kein `paths`. Führen Sie `claude --debug` aus, um den Parse-Fehler zu sehen.

<h4 id="share-rules-across-projects-with-symlinks">
  Teilen Sie Regeln über Projekte mit Symlinks
</h4>

Das Verzeichnis `.claude/rules/` unterstützt Symlinks, sodass Sie einen gemeinsamen Satz von Regeln verwalten und in mehrere Projekte verlinken können. Zirkuläre Symlinks werden erkannt und elegant behandelt.

Claude Code behandelt einen Symlink, dessen Ziel außerhalb Ihres Arbeitsverzeichnisses liegt, wie einen [externen Import](#import-additional-files). Die verlinkten Regeln werden nicht geladen, bis Sie externe Importe für das Projekt genehmigen, und danach werden nur die ohne ein [`paths`-Feld](#path-specific-rules) geladen. Claude Code fragt nach dieser Genehmigung nur, wenn eine Projektmemory-Datei eine Datei außerhalb des Arbeitsverzeichnisses mit `@path` importiert, nicht für Symlinks allein. Um gemeinsame Regeln ohne diese Genehmigung zu laden, halten Sie sie in [`~/.claude/rules/`](#user-level-rules), wo sie auf jedem Projekt auf Ihrem Computer gelten.

Dieses Beispiel verlinkt sowohl ein gemeinsames Verzeichnis als auch eine einzelne Datei:

```bash theme={null}
ln -s ~/shared-claude-rules .claude/rules/shared
ln -s ~/company-standards/security.md .claude/rules/security.md
```

<h4 id="user-level-rules">
  Benutzerweite Regeln
</h4>

Persönliche Regeln in `~/.claude/rules/` gelten für jedes Projekt auf Ihrem Computer. Verwenden Sie sie für Voreinstellungen, die nicht projektspezifisch sind:

```text theme={null}
~/.claude/rules/
├── preferences.md    # Ihre persönlichen Codierungsvoreinstellungen
└── workflows.md      # Ihre bevorzugten Arbeitsabläufe
```

Claude Code lädt Benutzerweite Regeln vor Projektregeln, sodass eine Projektregel später in Claudes Kontext als eine Benutzerregel erscheint. Keine der beiden Regelgruppen überschreibt die andere: Wenn eine Benutzerregel und eine Projektregeln sich widersprechen, kann Claude eine von beiden befolgen, daher halten Sie die beiden konsistent.

<h3 id="manage-claude-md-for-large-teams">
  Verwalten Sie CLAUDE.md für große Teams
</h3>

Für Organisationen, die Claude Code über Teams bereitstellen, können Sie Anweisungen zentralisieren und steuern, welche CLAUDE.md-Dateien geladen werden.

<h4 id="deploy-organization-wide-claude-md">
  Stellen Sie organisationsweite CLAUDE.md bereit
</h4>

Organisationen können ein zentral verwaltetes CLAUDE.md bereitstellen, das für alle Benutzer auf einem Computer gilt. Diese Datei kann nicht durch individuelle Einstellungen ausgeschlossen werden.

<Steps>
  <Step title="Erstellen Sie die Datei am verwalteten Richtlinienort">
    * macOS: `/Library/Application Support/ClaudeCode/CLAUDE.md`
    * Linux und WSL: `/etc/claude-code/CLAUDE.md`
    * Windows: `C:\Program Files\ClaudeCode\CLAUDE.md`
  </Step>

  <Step title="Stellen Sie mit Ihrem Konfigurationsverwaltungssystem bereit">
    Verwenden Sie MDM, Group Policy, Ansible oder ähnliche Tools, um die Datei über Entwicklermaschinen zu verteilen. Siehe [verwaltete Einstellungen](/docs/de/managed-settings) für andere organisationsweite Konfigurationsoptionen.
  </Step>
</Steps>

Der Schlüssel `claudeMd` ermöglicht es Ihnen, verwaltete CLAUDE.md-Inhalte direkt in `managed-settings.json` zu platzieren, statt eine separate Datei bereitzustellen.

**Umfang**: jede Claude Code-Sitzung auf dem Computer, in jedem Repository. Für repositoryspezifische Anleitung committen Sie stattdessen ein Projekt-CLAUDE.md.

**Vorrang**: gleich wie eine verwaltete CLAUDE.md-Datei. Wird vor Benutzer- und Projekt-CLAUDE.md geladen.

**Wo es berücksichtigt wird**: nur verwaltete und Richtlinieneinstellungen. Das Setzen von `claudeMd` in Benutzer-, Projekt- oder lokalen Einstellungen hat keine Auswirkung.

Das folgende Beispiel fügt Verhaltensanweisungen direkt in eine verwaltete Einstellungsdatei ein:

```json theme={null}
{
  "claudeMd": "Führen Sie immer `make lint` vor dem Commit aus.\nNiemand pusht direkt zu main."
}
```

Ein verwaltetes CLAUDE.md und [verwaltete Einstellungen](/docs/de/managed-settings) dienen unterschiedlichen Zwecken. Verwenden Sie Einstellungen für technische Durchsetzung und CLAUDE.md für Verhaltensanleitung:

| Bedenken                                                | Konfigurieren in                                                  |
| :------------------------------------------------------ | :---------------------------------------------------------------- |
| Blockieren Sie bestimmte Tools, Befehle oder Dateipfade | Verwaltete Einstellungen: `permissions.deny`                      |
| Erzwingen Sie Sandbox-Isolation                         | Verwaltete Einstellungen: `sandbox.enabled`                       |
| Umgebungsvariablen und API-Provider-Routing             | Verwaltete Einstellungen: `env`                                   |
| Anmeldemethode und Organisationsbeschränkungen          | Verwaltete Einstellungen: `forceLoginMethod`, `forceLoginOrgUUID` |
| Code-Stil und Qualitätsrichtlinien                      | Verwaltetes CLAUDE.md                                             |
| Datenbehandlung und Compliance-Erinnerungen             | Verwaltetes CLAUDE.md                                             |
| Verhaltensanweisungen für Claude                        | Verwaltetes CLAUDE.md                                             |

Einstellungsregeln werden vom Client unabhängig davon durchgesetzt, was Claude zu tun beschließt. CLAUDE.md-Anweisungen prägen Claudes Verhalten, sind aber keine harte Durchsetzungsebene.

<h4 id="exclude-specific-claude-md-files">
  Schließen Sie bestimmte CLAUDE.md-Dateien aus
</h4>

In großen Monorepos können Vorgänger-CLAUDE.md-Dateien Anweisungen enthalten, die für Ihre Arbeit nicht relevant sind. Die Einstellung `claudeMdExcludes` ermöglicht es Ihnen, bestimmte Dateien nach Pfad oder Glob-Muster zu überspringen.

Dieses Beispiel schließt ein Top-Level-CLAUDE.md und ein Regelverzeichnis aus einem übergeordneten Ordner aus. Fügen Sie es zu `.claude/settings.local.json` hinzu, damit der Ausschluss lokal auf Ihrem Computer bleibt:

```json theme={null}
{
  "claudeMdExcludes": [
    "**/monorepo/CLAUDE.md",
    "/home/user/monorepo/other-team/.claude/rules/**"
  ]
}
```

Muster werden gegen absolute Dateipfade mit Glob-Syntax abgeglichen. Sie können `claudeMdExcludes` auf jeder [Einstellungsebene](/docs/de/settings#where-settings-live) konfigurieren: Benutzer, Projekt, lokal oder verwaltete Richtlinie. Arrays werden über Ebenen hinweg zusammengeführt.

Um eine Regeldatei auszuschließen, die Sie über einen [Symlink](#share-rules-across-projects-with-symlinks) erreichen, ob die Datei oder ihr Verzeichnis der Link ist, schreiben Sie das Muster gegen einen der beiden Pfade: den Pfad der Datei unter `.claude/rules/` oder ihr Link-Ziel. Ein Muster, das einen der beiden Pfade entspricht, schließt die Datei aus. Vor v2.1.239 schloss nur ein Muster, das das Link-Ziel entsprach, die Datei aus.

Verwaltete Richtlinien-CLAUDE.md-Dateien können nicht ausgeschlossen werden. Dies stellt sicher, dass organisationsweite Anweisungen unabhängig von individuellen Einstellungen immer gelten.

<h2 id="agents-md">
  AGENTS.md
</h2>

Claude Code kann [`AGENTS.md`](/docs/de/glossary#agents-md) als Ihre Projektanweisungen lesen, daher funktioniert ein Repository, das bereits für andere Codierungs-Agenten eingerichtet ist, ohne dass Sie eine `CLAUDE.md`, einen Import oder eine Einstellung hinzufügen müssen. Diese Tabelle zeigt, was Claude standardmäßig für jede Kombination von Anweisungsdateien in Ihrem Repository liest:

| Ihr Repository hat                                                                                     | Claude liest                                                   |
| :----------------------------------------------------------------------------------------------------- | :------------------------------------------------------------- |
| Eine `AGENTS.md` und keine `CLAUDE.md` oder `CLAUDE.local.md` in Ihrem Arbeitsverzeichnis oder darüber | Ihre `AGENTS.md`                                               |
| Eine `AGENTS.md` und eine `CLAUDE.md` oder `CLAUDE.local.md` in Ihrem Arbeitsverzeichnis oder darüber  | Nur Ihre `CLAUDE.md`-Dateien                                   |
| Eine `CLAUDE.md`, die bereits [`AGENTS.md` importiert](#share-one-file-with-other-coding-tools)        | Ihre `CLAUDE.md`, mit `AGENTS.md` durch den Import eingebunden |

Um die Standardeinstellung zu ändern, z. B. um Claude beide Dateien lesen zu lassen, nur `CLAUDE.md` zu lesen oder nur die verwalteten Anweisungen Ihrer Organisation zu lesen, [ändern Sie die Einstellung **Projektanweisungen**](#choose-which-instruction-files-load).

<Note>
  Das direkte Lesen von `AGENTS.md` erfordert Claude Code v2.1.277 oder später. In einigen Sitzungen kann Claude [`AGENTS.md` nicht lesen](#when-agents-md-support-is-unavailable), daher [importieren Sie es stattdessen aus einer `CLAUDE.md`](#share-one-file-with-other-coding-tools).
</Note>

<h3 id="when-claude-code-reads-agents-md">
  Wann Claude Code AGENTS.md liest
</h3>

Standardmäßig liest Claude `AGENTS.md` nur, wenn Sie keine `CLAUDE.md` in Ihrem Arbeitsverzeichnis oder darüber haben. Hier sind die Dateien, die für diese Prüfung zählen:

* **Zählen, daher liest Claude diese statt `AGENTS.md`**: eine `CLAUDE.md`, `.claude/CLAUDE.md` oder `CLAUDE.local.md` in Ihrem Arbeitsverzeichnis oder einem Verzeichnis darüber
* **Zählen nicht und laden weiterhin neben `AGENTS.md`**: Ihre `~/.claude/CLAUDE.md`, die verwaltete `CLAUDE.md` Ihrer Organisation und `.claude/rules/`-Dateien

Wenn keine zählen, liest Claude hier, was es liest und wie Sie es erkennen können:

* **Beim Sitzungsstart**: jede `AGENTS.md` und `.claude/AGENTS.md` in Ihrem Arbeitsverzeichnis und den Verzeichnissen darüber. In einer interaktiven Sitzung sehen Sie eine Zeile wie `no CLAUDE.md found; AGENTS.md loaded: /home/you/repo/AGENTS.md` in der Konversation
* **Wenn Claude in Unterverzeichnissen arbeitet**: eine `AGENTS.md` eines Unterverzeichnisses, wenn Claude dort eine Datei mit dem Read-Tool öffnet und dieses Unterverzeichnis keine der drei `CLAUDE.md`-Dateien selbst hat
* **Innerhalb jeder `AGENTS.md`**: [`@path`-Importe](#import-additional-files) werden erweitert, [`claudeMdExcludes`](#exclude-specific-claude-md-files)-Muster gelten, und Subagenten, die [Projektanweisungen überspringen](/docs/de/sub-agents#what-loads-at-startup), überspringen diese Dateien auch
* **Nicht gelesen**: `AGENTS.local.md`, `AGENTS.override.md` oder alles unter einem `.agents/`-Verzeichnis

<Note>
  Da `CLAUDE.local.md` zählt, stoppt das Hinzufügen einer zum Speichern Ihrer eigenen nicht committeten Anweisungen in einem Projekt, das auf `AGENTS.md` angewiesen ist, Claude vom Lesen von `AGENTS.md` für Sie. Um Ihre `CLAUDE.local.md` zu behalten und Claude trotzdem `AGENTS.md` lesen zu lassen, setzen Sie **Projektanweisungen** auf [`claude-md-and-agents-md`](#choose-which-instruction-files-load).
</Note>

<h3 id="choose-which-instruction-files-load">
  Wählen Sie, welche Anweisungsdateien geladen werden
</h3>

Um zu ändern, welche Dateien Claude liest, geben Sie `/config` in einer Claude Code-Sitzung ein, um das Einstellungsfenster zu öffnen, und setzen Sie dann **Projektanweisungen** auf einen dieser Werte:

| Wert                      | Was Claude liest                                                                                                                                                                                                                                                                                                                                                                                                        |
| :------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `claude-md-or-agents-md`  | Ihre `CLAUDE.md`-Dateien oder Ihre `AGENTS.md`-Dateien, wenn Sie keine `CLAUDE.md` oder `CLAUDE.local.md` in Ihrem Arbeitsverzeichnis oder darüber haben. Dies ist die Standardeinstellung                                                                                                                                                                                                                              |
| `claude-md-and-agents-md` | Ihre `CLAUDE.md`- und `AGENTS.md`-Dateien zusammen, jede `CLAUDE.md`-Datei eines Verzeichnisses zuerst und seine `AGENTS.md` danach. Claude Code überspringt eine `AGENTS.md`, die es bereits geladen hat, daher wird eine, die Ihre `CLAUDE.md` importiert oder verlinkt, nicht zweimal gelesen                                                                                                                        |
| `claude-md`               | Nur Ihre `CLAUDE.md`-Dateien                                                                                                                                                                                                                                                                                                                                                                                            |
| `managed-only`            | Nur die verwaltete `CLAUDE.md` Ihrer Organisation und [automatisches Gedächtnis](#auto-memory) beim Start. Ihre Projekt-, lokalen und Benutzer-`CLAUDE.md`-Dateien, Ihre `.claude/rules/`-Dateien und jede `AGENTS.md` werden ausgelassen. Die `CLAUDE.md`- und `.claude/rules/`-Dateien eines Unterverzeichnisses und [pfadgebundene Regeln](#path-specific-rules) laden immer noch, wenn Claude eine Datei dort liest |

Sie können den Wert auch in einer Einstellungsdatei statt in `/config` setzen. Fügen Sie ihn unter der ID des integrierten `agents-md`-Plugins in [`pluginConfigs`](/docs/de/settings-reference#pluginconfigs) in `~/.claude/settings.json`, einer `--settings`-Datei oder [verwalteten Einstellungen](/docs/de/managed-settings) hinzu. Claude Code ignoriert ihn in Projekt- und lokalen Einstellungsdateien. Dieses Beispiel lässt Claude beide Dateien lesen:

```json settings.json theme={null}
{
  "pluginConfigs": {
    "agents-md@builtin": {
      "options": { "instructionFiles": "claude-md-and-agents-md" }
    }
  }
}
```

Ihre Änderung gilt ab der nächsten Nachricht, die Sie senden, und in jeder neuen Sitzung.

<h3 id="when-agents-md-support-is-unavailable">
  Wenn AGENTS.md-Unterstützung nicht verfügbar ist
</h3>

In diesen Sitzungen liest Claude nur `CLAUDE.md`-Dateien, und **Projektanweisungen** erscheint nicht im Einstellungsfenster `/config`:

* Sie verwenden eine Claude Code-Version vor v2.1.277
* Sie haben das integrierte `agents-md`-Plugin in `/plugin` deaktiviert
* In einigen Fällen ist es Ihre [erste Sitzung nach dem Upgrade](/docs/de/env-vars#first-session-after-an-install-or-upgrade) von v2.1.276 oder früher. Claude liest `AGENTS.md` ab Ihrer nächsten Sitzung

Vor v2.1.281 lasen einige Sitzungen, z. B. solche auf Amazon Bedrock oder mit deaktivierter Telemetrie, nur `CLAUDE.md`-Dateien. Aktualisieren Sie Claude Code auf diesen Versionen. Um Claude Ihre `AGENTS.md` in diesen Sitzungen zu geben, [importieren Sie sie aus einer `CLAUDE.md`](#share-one-file-with-other-coding-tools).

<h3 id="where-agents-md-differs-from-claude-md">
  Wo sich AGENTS.md von CLAUDE.md unterscheidet
</h3>

Eine `AGENTS.md`, die Claude durch die Einstellung **Projektanweisungen** liest, unterscheidet sich von einer `CLAUDE.md` an diesen Stellen:

|                                                                                                                                                            | `CLAUDE.md`                                                                            | `AGENTS.md` gelesen durch die Einstellung                                                                                    |
| :--------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| [`InstructionsLoaded`-Hooks](/docs/de/hooks#instructionsloaded)                                                                                                 | Werden ausgelöst                                                                       | Werden nicht ausgelöst. Sie werden wie gewohnt ausgelöst für eine `AGENTS.md`, die eine `CLAUDE.md` importiert oder verlinkt |
| Verzeichnisse, die Sie mit `--add-dir` hinzufügen, während [`CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`](#load-from-additional-directories) gesetzt ist | Ihre `CLAUDE.md` lädt                                                                  | Ihre `AGENTS.md` lädt nicht                                                                                                  |
| Ein `@path`-Import einer Datei außerhalb Ihres Arbeitsverzeichnisses                                                                                       | Claude Code fordert Sie auf, [externe Importe](#import-additional-files) zu genehmigen | Lädt nur, wenn Sie bereits externe Importe für dieses Projekt genehmigt haben, ohne Aufforderung                             |

<h3 id="remove-an-earlier-agents-md-workaround">
  Entfernen Sie einen früheren AGENTS.md-Workaround
</h3>

Wenn Sie Claude Code so eingerichtet haben, dass `AGENTS.md` gelesen wird, bevor es dies von selbst tat, hier ist, was Sie mit jedem häufigen Setup tun sollten:

* **Eine `CLAUDE.md` mit `@AGENTS.md`**: Sie können sie behalten. Das Behalten des Imports führt niemals dazu, dass Claude `AGENTS.md` zweimal liest, unabhängig davon, welchen **Projektanweisungen**-Wert Sie verwenden. Entfernen Sie die `CLAUDE.md`, wenn sie nichts anderes enthält, oder behalten Sie sie, wenn einige Ihrer Sitzungen [`AGENTS.md` nicht direkt laden können](#when-agents-md-support-is-unavailable).
* **Eine `CLAUDE.md`, die Claude in Worten anweist, `AGENTS.md` zu lesen**: Claude sieht `AGENTS.md` nur, wenn es sich entscheidet, die Datei zu öffnen. Löschen Sie die `CLAUDE.md`, damit Claude `AGENTS.md` direkt liest, oder ersetzen Sie den Satz durch einen `@AGENTS.md`-Import.
* **Eine `CLAUDE.md`, die mit `AGENTS.md` verlinkt ist**: nichts, oder löschen Sie den Symlink. Auf jeden Fall liest Claude den Inhalt einmal.
* **Ein `SessionStart`-Hook, der `AGENTS.md` ausgibt**: entfernen Sie ihn. Sobald Claude `AGENTS.md` direkt liest, fügt der Hook eine zweite Kopie zum Kontext hinzu.

<h3 id="share-one-file-with-other-coding-tools">
  Teilen Sie eine Datei mit anderen Codierungs-Tools
</h3>

Wenn Claude Ihre `AGENTS.md` nicht direkt liest, können Sie sie trotzdem als die eine Datei behalten, die jedes Tool teilt, indem Sie einen `@AGENTS.md`-Import in eine `CLAUDE.md` neben ihr einfügen. Tun Sie dies, wenn Ihr Projekt auch eine `CLAUDE.md` hat, wenn Sie **Projektanweisungen** auf `claude-md` gesetzt haben, oder in Sitzungen, die [`AGENTS.md` nicht laden können](#when-agents-md-support-is-unavailable). Fügen Sie alle Claude-spezifischen Anweisungen unter dem Import hinzu, und Claude liest die importierte Datei zuerst, dann den Rest:

```markdown CLAUDE.md theme={null}
@AGENTS.md

## Claude Code

Use plan mode for changes under `src/billing/`.
```

Wenn Sie keinen Claude-spezifischen Inhalt benötigen, funktioniert auch ein Symlink:

```bash theme={null}
ln -s AGENTS.md CLAUDE.md
```

Der Befehl gibt bei Erfolg keine Ausgabe aus. Bevor Sie den Symlink dem Import vorziehen, überprüfen Sie diese Einschränkungen:

* **Bearbeitung**: Claude liest `CLAUDE.md` durch den Link, aber die Edit- und Write-Tools [weigern sich, durch einen Symlink zu schreiben](/docs/de/errors#refusing-after-a-symlink-changed), und die Weigerung weist Claude an, stattdessen das Ziel des Links, `AGENTS.md`, zu bearbeiten
* **Windows**: Wenn Sie oder jemand, der das Repository klont, unter Windows arbeitet, verwenden Sie stattdessen den `@AGENTS.md`-Import. Das Erstellen eines Symlinks dort erfordert Administratorrechte oder den Entwicklermodus, und Git checkt einen committeten Symlink als Nur-Text-Datei aus, es sei denn, `core.symlinks` ist aktiviert, was diesen Klon mit einer einzeiligen `CLAUDE.md` anstelle Ihrer Anweisungen hinterlässt

Führen Sie mit beiden Ansätzen `/context` in Ihrer nächsten Sitzung aus und bestätigen Sie, dass `CLAUDE.md` unter **Speicherdateien** angezeigt wird.

<h3 id="migrate-instructions-from-other-tools">
  Migrieren Sie Anweisungen von anderen Tools
</h3>

Das Ausführen von [`/init`](/docs/de/commands) liest die Anweisungsdateien anderer Tools und integriert die relevanten Teile in die generierte `CLAUDE.md`:

* Cursor-Regeln in `.cursor/rules/` oder `.cursorrules`
* Copilot-Regeln in `.github/copilot-instructions.md`
* Mit `CLAUDE_CODE_NEW_INIT=1` gesetzt: `AGENTS.md`, `.devin/rules/`, `.windsurf/rules/` oder `.windsurfrules`, und `.clinerules`

Sie können auch [`/import`](/docs/de/commands) ausführen, um die Konfiguration eines unterstützten Codierungs-Agenten in Claude Code zu bringen, was eine einmalige Kopie von Anweisungsdateien wie `AGENTS.md` an die entsprechende `CLAUDE.md` anhängt und MCP-Server, Befehle, Subagenten und Skills überträgt. Erfordert Claude Code v2.1.213 oder später.

<h2 id="auto-memory">
  Auto-Memory
</h2>

Auto-Memory lässt Claude Wissen über Sitzungen hinweg sammeln, ohne dass Sie etwas schreiben müssen. Während Claude arbeitet, speichert es vier Arten von Notizen für sich selbst. Claude speichert die Art als `type`-Feld in der Frontmatter der Memory-Datei:

* `user`: Ihre Rolle, Expertise und Arbeitspräferenzen
* `feedback`: Korrektionen, die Sie Claude geben, und Ansätze, die Sie bestätigen
* `project`: laufende Arbeiten, Fristen und Entscheidungen, die Claude nicht aus dem Code oder der Git-Historie ableiten kann
* `reference`: wo man Informationen außerhalb des Projekts findet, wie einen Issue-Tracker oder ein Dashboard

Claude überspringt alles, was es aus der Codebasis ableiten kann, wie Architektur, Dateipfade oder Debugging-Fixes. Es überspringt auch alles, was Ihre CLAUDE.md-Dateien bereits sagen.

Claude speichert nicht jede Sitzung etwas. Es entscheidet, was es sich merken sollte, basierend darauf, ob die Information in einer zukünftigen Konversation nützlich wäre.

<h3 id="enable-or-disable-auto-memory">
  Aktivieren oder deaktivieren Sie Auto-Memory
</h3>

Auto-Memory ist standardmäßig aktiviert. Um es umzuschalten, öffnen Sie `/memory` in einer Sitzung und verwenden Sie den Auto-Memory-Schalter, der `autoMemoryEnabled` in Ihren Benutzereinstellungen unter `~/.claude/settings.json` speichert. Um es für ein einzelnes Projekt auszuschalten, setzen Sie `autoMemoryEnabled` in den Einstellungen dieses Projekts:

```json theme={null}
{
  "autoMemoryEnabled": false
}
```

Um Auto-Memory über eine Umgebungsvariable zu deaktivieren, setzen Sie `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1`.

<h3 id="storage-location">
  Speicherort
</h3>

Jedes Projekt erhält sein eigenes Memory-Verzeichnis unter `~/.claude/projects/<project>/memory/`. Der `<project>`-Pfad wird aus dem Git-Repository abgeleitet, sodass alle Worktrees und Unterverzeichnisse innerhalb desselben Repos ein Auto-Memory-Verzeichnis teilen. Außerhalb eines Git-Repos wird stattdessen das Projektstammverzeichnis verwendet.

Wenn Sie [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/de/sessions#name-the-project-directory-yourself) neben `CLAUDE_CONFIG_DIR` setzen, verwendet Claude Code diesen Namen als `<project>`-Verzeichnis unter `<config dir>/projects/` unabhängig davon, welches Repository Sie starten, sodass Projekte, die mit diesem Konfigurationsverzeichnis gestartet werden, ein Auto-Memory-Verzeichnis teilen. Erfordert Claude Code v2.1.234 oder später.

Um Auto-Memory an einem anderen Ort zu speichern, setzen Sie `autoMemoryDirectory` in Ihrer `settings.json`. Es wird aus jedem [Einstellungsbereich](/docs/de/settings#settings-precedence) gelesen: Benutzer, Projekt, lokal, Richtlinie oder `--settings`.

```json theme={null}
{
  "autoMemoryDirectory": "~/my-custom-memory-dir"
}
```

Der Wert muss ein absoluter Pfad sein oder mit `~/` beginnen.

Wenn Sie ihn in der `.claude/settings.json` oder `.claude/settings.local.json` eines Projekts festlegen, berücksichtigt Claude Code ihn unter der gleichen [Workspace-Trust-Regel wie Hooks in Einstellungsdateien](/docs/de/permissions#what-runs-before-you-trust-a-folder). Während [`permissions.blockReadsOutsideWorkingDirectories`](/docs/de/settings-reference#permissions-blockreadsoutsideworkingdirectories) aktiviert ist, lädt Claude Code kein Auto-Memory aus einem Verzeichnis, das eine [von einem Repository bereitgestellte Einstellungsdatei](/docs/de/permissions#when-your-local-settings-file-needs-trust) auswählt, und speichert keines darin, unabhängig davon, wo sich dieses Verzeichnis befindet.

Das Verzeichnis enthält einen `MEMORY.md`-Index und eine Themadatei pro Memory:

```text theme={null}
~/.claude/projects/<project>/memory/
├── MEMORY.md           # Index, eine Zeile pro Memory, geladen in jede Sitzung
├── user_role.md        # Ein Memory
├── feedback_testing.md # Ein Memory
└── ...                 # Alle anderen Themadateien, die Claude erstellt
```

`MEMORY.md` fungiert als Index des Memory-Verzeichnisses. Claude liest und schreibt Dateien in diesem Verzeichnis während Ihrer Sitzung und verwendet `MEMORY.md`, um den Überblick zu behalten, was wo gespeichert ist.

Auto-Memory ist maschinenlokal. Alle Worktrees und Unterverzeichnisse innerhalb desselben Git-Repositories teilen ein Auto-Memory-Verzeichnis. Dateien werden nicht über Maschinen oder Cloud-Umgebungen hinweg geteilt.

Claude Code löscht alte Sitzungstranskripte nach der [`cleanupPeriodDays`](/docs/de/settings-reference#cleanupperioddays)-Aufbewahrungsfrist, schließt aber die Memory-Dateien im Memory-Verzeichnis von dieser [Aufbewahrungslöschung](/docs/de/claude-directory#cleaned-up-automatically) aus. `MEMORY.md` und Themadateien bleiben bestehen, bis Sie oder Claude sie bearbeiten oder löschen.

<h3 id="how-it-works">
  Wie es funktioniert
</h3>

Die ersten 200 Zeilen von `MEMORY.md`, oder die ersten 25 KB, je nachdem, was zuerst erreicht wird, werden zu Beginn jeder Konversation geladen. Inhalte über diese Schwelle hinaus werden nicht beim Sitzungsstart geladen. Claude hält `MEMORY.md` prägnant, indem es detaillierte Notizen in separate Themadateien verschiebt.

Nachdem Claude in `MEMORY.md` schreibt, misst Claude Code die Datei gegen die 200-Zeilen- und 25-KB-Lesegrenzen. Wenn die Datei sich einer Grenze nähert, erinnert Claude Code Claude daran, sie zu verkürzen: eine Zeile pro Eintrag behalten, Details in Themadateien verschieben und veraltete Einträge zusammenführen oder löschen. Wenn die Datei über einer Grenze liegt, wird der Schreibvorgang trotzdem erfolgreich ausgeführt, aber Claude Code gibt einen [Fehler zurück, der Claude auffordert, den Index umzuschreiben](/docs/de/errors#memory-index-is-over-its-read-limit), da alles über der Grenze beim nächsten Laden verworfen wird.

Diese Grenze gilt nur für `MEMORY.md`. Claude Code lädt eine CLAUDE.md-Datei von bis zu 4 MiB vollständig und überspringt eine größere Datei. Kürzere Dateien erzeugen bessere Einhaltung.

Claude Code lädt Themadateien wie `user_role.md` oder `feedback_testing.md` nicht beim Start. Claude liest sie bei Bedarf mit seinen Standard-Datei-Tools, wenn es die Informationen benötigt.

Das Auto-Memory der Hauptkonversation wird nicht in [Subagenten](/docs/de/sub-agents#what-loads-at-startup) geladen; die Ausnahme ist ein [Fork](/docs/de/sub-agents#fork-the-current-conversation), der die übergeordnete Konversation und den System-Prompt erbt. Das eigene Auto-Memory eines Subagenten, aktiviert mit dem Subagenten-`memory`-Feld, ist ein separates Verzeichnis.

Claude liest und schreibt Memory-Dateien während Ihrer Sitzung. Wenn Sie Meldungen wie „Saved 2 memories" oder „Recalled 2 memories" in der Claude Code-Schnittstelle sehen, aktualisiert oder liest Claude aktiv aus `~/.claude/projects/<project>/memory/`.

Wenn Claude eine Memory-Datei schreibt, die mit YAML-Frontmatter beginnt, speichert Claude Code die Schreibzeit in einem `modified`-Frontmatter-Feld als ISO-8601-Zeitstempel. Der Zeitstempel zeigt, wie aktuell die Tatsache ist, sowohl für Sie als auch für Claude, wenn es das Memory zurückliest. Jede Datei, die Frontmatter hat, erhält das Feld beim nächsten Schreiben durch Claude, einschließlich Dateien, die in früheren Versionen erstellt wurden; Claude Code fügt niemals Frontmatter zu einer Datei hinzu, die keine hat. Das `modified`-Feld erfordert Claude Code v2.1.214 oder später.

<h3 id="audit-and-edit-your-memory">
  Überprüfen und bearbeiten Sie Ihr Memory
</h3>

Auto-Memory-Dateien sind einfaches Markdown, das Sie jederzeit bearbeiten oder löschen können. Führen Sie [`/memory`](#view-and-edit-with-%2Fmemory) aus, um Memory-Dateien innerhalb einer Sitzung zu durchsuchen und zu öffnen.

<h2 id="view-and-edit-with-/memory">
  Anzeigen und Bearbeiten mit `/memory`
</h2>

Der Befehl `/memory` listet Ihre CLAUDE.md, CLAUDE.local.md und andere Speicherdateien an verschiedenen Orten im Benutzer- und Projektbereich auf, einschließlich Benutzer- und Projekt-CLAUDE.md-Einträge für Dateien, die noch nicht existieren. Er ermöglicht es Ihnen auch, Auto-Memory ein- oder auszuschalten, und bietet eine Option zum Öffnen des Auto-Memory-Ordners. Wählen Sie eine beliebige Datei aus, um sie in Ihrem Editor zu öffnen; wenn Sie eine Datei auswählen, die noch nicht existiert, wird sie zuerst erstellt. Um zu überprüfen, welche `CLAUDE.md`- und Rules-Dateien in die aktuelle Sitzung geladen wurden, führen Sie `/context` aus.

GUI-Editoren wie VS Code öffnen die Datei in einem separaten Fenster, und Sie können die Sitzung weiterhin nutzen, während sie offen ist. Vor v2.1.216 wartete `/memory` darauf, dass Sie die Datei schließen, bevor es antwortete. Terminal-Editoren wie Vim übernehmen das Terminal, bis Sie es beenden.

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

* Führen Sie `/context` aus und überprüfen Sie die Liste unter **Memory-Dateien**, um zu überprüfen, dass Ihre CLAUDE.md- und CLAUDE.local.md-Dateien geladen wurden. Wenn eine `CLAUDE.md`-Datei dort fehlt, kann Claude sie nicht sehen. Verwenden Sie `/memory`, um die Dateien zu öffnen und zu bearbeiten.
* Überprüfen Sie, dass die relevante CLAUDE.md an einem Ort ist, der für Ihre Sitzung geladen wird (siehe [Wählen Sie, wo Sie CLAUDE.md-Dateien ablegen](#choose-where-to-put-claude-md-files)).
* Machen Sie Anweisungen spezifischer. „Verwenden Sie 2-Leerzeichen-Einrückung" funktioniert besser als „formatieren Sie Code schön".
* Suchen Sie nach widersprüchlichen Anweisungen über CLAUDE.md-Dateien hinweg. Wenn zwei Dateien unterschiedliche Anleitungen für das gleiche Verhalten geben, kann Claude eine willkürlich auswählen.

Wenn die Anweisung etwas ist, das an einem bestimmten Punkt ausgeführt werden muss, z. B. vor jedem Commit oder nach jeder Dateibearbeitung, schreiben Sie sie stattdessen als [Hook](/docs/de/hooks-guide). Hooks werden als Shell-Befehle bei festen Lebenszyklusereignissen ausgeführt und gelten unabhängig davon, was Claude entscheidet zu tun.

Für Anweisungen, die Sie auf System-Prompt-Ebene haben möchten, verwenden Sie [`--append-system-prompt`](/docs/de/cli-reference#system-prompt-flags). Sie übergeben sie beim Start, daher ist es besser für Skripte und Automatisierung als für interaktive Nutzung geeignet. Informationen zum Verhalten beim Fortsetzen einer Konversation finden Sie unter [System-Prompt-Flags in fortgesetzten Konversationen](/docs/de/cli-reference#system-prompt-flags-in-resumed-conversations).

<Tip>
  Verwenden Sie den [`InstructionsLoaded`-Hook](/docs/de/hooks#instructionsloaded), um genau zu protokollieren, welche `CLAUDE.md`- und Regeldateien geladen sind, wann sie geladen werden und warum. Dies ist nützlich zum Debuggen von pfadspezifischen Regeln oder Lazy-Loading-Dateien in Unterverzeichnissen.
</Tip>

<h3 id="my-agents-md-isn’t-loading">
  Meine AGENTS.md wird nicht geladen
</h3>

Wenn Ihr Repository eine `AGENTS.md` hat und Claude scheint nicht zu wissen, was sie sagt, ist die übliche Ursache eine `CLAUDE.md` irgendwo auf dem Projektpfad. Standardmäßig liest Claude `AGENTS.md` nur, wenn Sie keine `CLAUDE.md` oder `CLAUDE.local.md` in Ihrem Arbeitsverzeichnis oder darüber haben. Überprüfen Sie diese in dieser Reihenfolge:

1. Suchen Sie nach einer `CLAUDE.md`, `.claude/CLAUDE.md` oder `CLAUDE.local.md` in Ihrem Arbeitsverzeichnis oder einem Verzeichnis darüber, außer Ihrer `~/.claude/CLAUDE.md`. Wenn Sie eine finden, liest Claude sie statt `AGENTS.md`, es sei denn, Sie setzen **Projektanweisungen** auf `claude-md-and-agents-md`.
2. Führen Sie `claude --version` aus und bestätigen Sie v2.1.277 oder später. Vor v2.1.281 konnten einige Sitzungen, wie z. B. solche auf Amazon Bedrock oder mit deaktivierter Telemetrie, [`AGENTS.md` nicht laden](#when-agents-md-support-is-unavailable), daher aktualisieren Sie auf v2.1.281 oder später.
3. Geben Sie `/config` in Ihrer Sitzung ein, um das Einstellungsfenster zu öffnen und bestätigen Sie, dass **Projektanweisungen** nicht auf `claude-md` oder `managed-only` gesetzt ist. Wenn Sie die Einstellung dort überhaupt nicht sehen, ist Ihre Sitzung eine, die [`AGENTS.md` nicht laden kann](#when-agents-md-support-is-unavailable).

Um zu überprüfen, ob Claude Ihre `AGENTS.md` gelesen hat, führen Sie `/memory` aus und suchen Sie nach ihrem Pfad in der Liste.

Vor v2.1.280 haben `/memory` und `/context` eine `AGENTS.md`, die Claude direkt gelesen hat, nicht aufgelistet. Fragen Sie Claude auf diesen Versionen stattdessen, was seine Projektanweisungen sagen.

Wenn Sie die `CLAUDE.md` behalten möchten, die Sie gefunden haben, oder Ihre Sitzung kann `AGENTS.md` nicht laden, [fügen Sie eine `CLAUDE.md` neben Ihrer `AGENTS.md` hinzu, die sie importiert](#share-one-file-with-other-coding-tools).

<h3 id="i-don’t-know-what-auto-memory-saved">
  Ich weiß nicht, was Auto-Memory gespeichert hat
</h3>

Führen Sie `/memory` aus und wählen Sie den Auto-Memory-Ordner aus, um zu durchsuchen, was Claude gespeichert hat. Alles ist einfaches Markdown, das Sie lesen, bearbeiten oder löschen können.

<h3 id="my-claude-md-is-too-large">
  Meine CLAUDE.md ist zu groß
</h3>

Dateien über 200 Zeilen verbrauchen mehr Kontext und können die Einhaltung reduzieren. Claude Code überspringt eine Datei über 4 MiB. Verwenden Sie [pfadgebundene Regeln](#path-specific-rules), um Anweisungen nur zu laden, wenn Claude mit übereinstimmenden Dateien arbeitet, oder trimmen Sie Inhalte, die nicht in jeder Sitzung benötigt werden. Das Aufteilen in [`@path`-Importe](#import-additional-files) hilft bei der Organisation, reduziert aber nicht den Kontext, da importierte Dateien beim Start geladen werden.

Die [`/doctor`](/docs/de/commands#all-commands)-Überprüfung schlägt Kürzungen für eine eingecheckte CLAUDE.md vor: Sie entfernt Inhalte, die Claude aus der Codebasis ableiten kann, wie Verzeichnislayouts, Abhängigkeitslisten und Architekturübersichten, und behält Fallstricke, Begründungen und Konventionen, die sich von Tool-Standardwerten unterscheiden. Die Trim-Überprüfung erfordert Claude Code v2.1.206 oder später.

<h3 id="instructions-seem-lost-after-/compact">
  Anweisungen scheinen nach `/compact` verloren zu gehen
</h3>

Projekt-Root-CLAUDE.md übersteht Komprimierung: Nach `/compact` liest Claude sie neu von der Festplatte und injiziert sie frisch in die Sitzung. Verschachtelte CLAUDE.md-Dateien in Unterverzeichnissen und Regeln mit [`paths:`-Frontmatter](#path-specific-rules) werden neu geladen, wenn Claude Dateien liest, auf die sie zutreffen.

Wenn eine Anweisung nach der Komprimierung verschwunden ist, wurde sie entweder nur in der Konversation gegeben, befindet sich in einer verschachtelten CLAUDE.md, die noch nicht neu geladen wurde, oder ist eine pfadgebundene Regel, die seit der Komprimierung keine Datei gefunden hat. Fügen Sie Anweisungen, die nur in der Konversation gegeben wurden, zu CLAUDE.md hinzu, um sie über Sitzungen hinweg zu erhalten. Weitere Informationen finden Sie unter [Was übersteht Komprimierung](/docs/de/context-window#what-survives-compaction) für die vollständige Aufschlüsselung.

Weitere Informationen finden Sie unter [Schreiben Sie effektive Anweisungen](#write-effective-instructions) für Anleitungen zu Größe, Struktur und Spezifität.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Debuggen Sie Ihre Konfiguration](/docs/de/debug-your-config): Diagnostizieren Sie, warum CLAUDE.md oder Einstellungen nicht wirksam werden
* [Skills](/docs/de/skills): Verpacken Sie wiederholbare Workflows, die bei Bedarf geladen werden
* [Einstellungen](/docs/de/settings): Konfigurieren Sie Claude Code-Verhalten mit Einstellungsdateien
* [Subagent-Memory](/docs/de/sub-agents#enable-persistent-memory): Lassen Sie Subagents ihre eigene Auto-Memory pflegen
