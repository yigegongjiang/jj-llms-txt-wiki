> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins mit Evals testen

> Schreiben Sie Eval-Fälle für Ihr Claude Code Plugin, führen Sie sie mit claude plugin eval aus, bewerten Sie die Ergebnisse, vergleichen Sie sie mit einer Baseline ohne Plugin und gaten Sie CI basierend auf dem Score.

Der Shell-Befehl `claude plugin eval` führt Ihr [Plugin](/docs/de/plugins/overview) gegen eine Suite von Testfällen aus und bewertet die Ergebnisse. Jeder Fall ist eine realistische Eingabeaufforderung plus ein oder mehrere Grader. Ein Grader ist eine Bestanden/Nicht-Bestanden-Prüfung auf das, was Claude produziert hat, wie z. B. ein Regex über die Antwort, ob ein bestimmtes Tool aufgerufen wurde, oder eine Rubrik, die ein zweites Modell über die Antwort beurteilt.

Sie müssen die Suite nicht von Hand schreiben; `claude plugin eval init` fragt Sie nach Ihrem Plugin, schlägt die Fälle und Grader vor, probiert sie aus und schreibt die Dateien, und Sie können Claude bitten, dasselbe aus einer bereits offenen Sitzung zu tun.

Verwenden Sie Evals, um zu:

* Messen, wie zuverlässig Ihr Plugin Claude zum richtigen Ergebnis lenkt
* Regressionen erkennen, wenn Sie das Plugin ändern oder ein neues Modell ausgeliefert wird
* Sehen, was das Plugin im Vergleich zu keinem Plugin beiträgt

Diese Seite ist für Plugin- und Skill-Autoren, die ein funktionierendes Plugin haben und sein Verhalten testen möchten, sowie für Teams, die Plugin-Änderungen in CI gaten. Das Fallformat unterscheidet sich von der Datei `evals/evals.json`, die das [skill-creator Plugin](/docs/de/skills#run-evals-with-skill-creator) verwendet. Um ein Plugin zu erstellen, siehe [Plugin erstellen](/docs/de/plugins/create); um die Dateien eines Plugins auf Syntax- und Schemafehler zu überprüfen, anstatt sein Verhalten zu überprüfen, verwenden Sie [`claude plugin validate`](/docs/de/plugins/cli-reference#plugin-validate).

<Note>
  Jeder Eval-Durchlauf und jeder Judge-Grader ist ein echter Modellaufruf auf Ihrem Konto, der gegen die Nutzung Ihres Plans oder Ihre API-Rechnung angerechnet wird. Überprüfen Sie daher zunächst die [Anforderungen](#requirements). Erstellen Sie dann Ihre [erste Eval-Suite](#create-your-first-eval-suite), oder gehen Sie zu [Evals in CI ausführen](#run-evals-in-ci), wenn Sie bereits eine haben.
</Note>

<h2 id="requirements">
  Anforderungen
</h2>

Um Plugin-Evals auszuführen, benötigen Sie:

* Claude Code v2.1.269 oder später. Führen Sie `claude --version` aus, um zu überprüfen, und `claude update`, um zu aktualisieren.
* Ein Plugin-Verzeichnis mit einem `plugin.json` oder `.claude-plugin/plugin.json` Manifest, oder ein [Skills-Directory-Plugin](/docs/de/plugins/loading#plugins-shared-through-a-repository).
* Die gleiche Authentifizierung und den gleichen Modell-Provider, den Ihre normalen Claude Code Sitzungen verwenden. Eval-Durchläufe, Judge-bewertete Grader und `claude plugin eval init` rufen das Modell mit Ihren Anmeldedaten auf, daher werden sie gegen Ihre Plan-Nutzungslimits oder Ihre API-Rechnung angerechnet. Wenn der Befehl Kosten meldet, ist die Zahl eine [Listenpreis-Schätzung](/docs/de/costs) dieser Aufrufe.

<h2 id="how-an-eval-run-works">
  Wie ein Eval-Durchlauf funktioniert
</h2>

Eine Eval-Suite befindet sich in einem Verzeichnis namens `evals/` innerhalb Ihres Plugins, angeordnet wie [Fälle schreiben und verfeinern](#write-and-refine-cases) zeigt. Jeder Fall ist sein eigenes Unterverzeichnis mit einer [Eingabeaufforderung](#set-run-limits-and-tools-in-prompt-md) und einem oder mehreren [Gradern](#grade-the-result). Die Eingabeaufforderung ist etwas, das eine Person, die Ihr Plugin verwendet, eingeben könnte, wie z. B. eine Anfrage, die einer seiner Skills verarbeiten sollte.

<h3 id="what-happens-in-a-run">
  Was in einem Durchlauf passiert
</h3>

Für jeden Durchlauf eines Falls startet Claude Code eine frische, [isolierte](#how-runs-are-isolated) [nicht-interaktive Sitzung](/docs/de/headless) mit nur Ihrem Plugin geladen, sendet die Eingabeaufforderung und lässt Claude arbeiten, bis es fertig ist oder das Limit des Falls für Züge oder Zeit erreicht. Jeder Grader überprüft dann die endgültige Antwort, das Transkript oder eine Datei, die Claude erstellt hat, und bestätigt oder lehnt ab.

<h3 id="how-a-case-is-scored">
  Wie ein Fall bewertet wird
</h3>

Ein Durchlauf eines nicht-deterministischen Agenten sagt Ihnen wenig, daher wird jeder Fall standardmäßig dreimal ausgeführt. Der Score eines Durchlaufs ist der Anteil seiner bestandenen Grader, gewichtet, wenn Sie Gewichte festlegen, und der Score des Falls ist der Durchschnitt über seine Durchläufe. Ein Fall wird bestanden, wenn sein Score den [`--threshold`](#command-options) erfüllt, standardmäßig 1,0. Bei Modellaufrufen führt eine Suite ungefähr Fälle × Durchläufe Agent-Durchläufe mit dem Plugin durch und ebenso viele für die [Baseline ohne Plugin](#the-no-plugin-baseline), plus drei kurze Judge-Aufrufe pro `llm` oder `baseline` Grader pro Durchlauf.

<h3 id="the-no-plugin-baseline">
  Die Baseline ohne Plugin
</h3>

Ein hoher Score allein sagt Ihnen nicht, dass das Plugin geholfen hat, da Claude auch ohne es gleich gut abschneiden könnte. Um die beiden zu trennen, werden die Durchläufe jedes Falls standardmäßig mit keinem Plugin geladen wiederholt, und Sie erhalten zwei Scores, `WITH` und `W/OUT`. Ihr Unterschied, `Δ`, ist das, was das Plugin beigetragen hat. Wenn ein Fall sowohl mit als auch ohne Plugin 1,0 bewertet wird, ist das Plugin nicht das, was ihn bestanden hat. Die beiden Sätze von Durchläufen werden als With-Arm und Without-Arm bezeichnet; [Vergleich mit einer Baseline ohne Plugin](#compare-against-a-no-plugin-baseline) behandelt, wie Grader über sie hinweg bewertet werden und wie man die Baseline ausschaltet.

<h2 id="create-your-first-eval-suite">
  Erstellen Sie Ihre erste Eval-Suite
</h2>

Diese Anleitung schreibt einen Fall für Ihr eigenes Plugin, führt ihn aus und liest das Ergebnis. Bevor Sie beginnen, stellen Sie sicher, dass Sie haben:

* Claude Code v2.1.269 oder später und die anderen [Anforderungen](#requirements)
* Ein Terminal, das in Ihrem Plugin-Stammverzeichnis geöffnet ist, dem Verzeichnis, das `plugin.json` oder `.claude-plugin/plugin.json` enthält
* Einen Skill im Plugin, den Sie testen möchten, und eine Anfrage, die ein Benutzer eingeben würde, die ihn auslösen sollte

<Steps>
  <Step title="Erstellen Sie die Fälle">
    Führen Sie vom Plugin-Stammverzeichnis aus aus:

    ```bash theme={null}
    claude plugin eval init
    ```

    Wenn Claude Code diesem Verzeichnis noch nicht vertraut, fragt es zunächst `Trust this plugin directory?`; antworten Sie mit `y`.

    Eine interaktive Claude Code Sitzung wird dann geöffnet. Claude liest Ihr Plugin und fragt Sie, wie ein gutes Ergebnis aussieht, schlägt Eingabeaufforderungen vor, die das Plugin auslösen sollten und nicht sollten, entwirft Grader für jeden, probiert sie einmal aus, um zu überprüfen, ob sie sich verhalten, und schreibt ein Fallverzeichnis pro Eingabeaufforderung unter `evals/`, jeweils nach seiner Eingabeaufforderung benannt.

    Wenn Claude Ihnen sagt, dass die Suite bereit ist, beenden Sie diese Sitzung mit `/exit` oder Ctrl+D, um zu Ihrer Shell zurückzukehren.

    Wenn Sie bereits eine Claude Code Sitzung am Plugin-Stammverzeichnis geöffnet haben, können Sie stattdessen Claude dort bitten, `claude plugin eval init` auszuführen. Claude führt den Befehl aus und stellt Ihnen dann die gleichen Fragen in dieser Konversation.

    Wenn Sie einen Fall lieber selbst schreiben möchten, um genau zu sehen, welche Dateien enthalten sind, folgen Sie [Schreiben Sie einen Fall von Hand](#write-a-case-manually) und kommen Sie hierher zurück, um ihn auszuführen.
  </Step>

  <Step title="Führen Sie die Suite aus">
    Zurück an Ihrer Shell im Plugin-Stammverzeichnis führen Sie jeden Fall unter `evals/` aus:

    ```bash theme={null}
    claude plugin eval .
    ```

    Sie haben diesem Verzeichnis bereits in Schritt 1 vertraut, daher startet der Durchlauf sofort. Wenn Sie den Fall stattdessen von Hand geschrieben haben, fragt der Durchlauf zunächst `Trust this plugin directory? [y/N]`; antworten Sie mit `y`. [Was ein Durchlauf zugreifen kann](#security) erklärt, wofür Sie zustimmen.

    Jeder Fall wird dreimal mit Ihrem Plugin und dreimal ohne es ausgeführt, daher ist ein Fall sechs Durchläufe. Eine Fortschrittszeile wird gedruckt, wenn jeder Durchlauf fertig ist, mit dem Score dieses Durchlaufs und dem Urteil jedes Graders.
  </Step>

  <Step title="Lesen Sie die Zusammenfassung">
    Wenn die Suite fertig ist, sehen Sie eine Zusammenfassungstabelle, gefolgt von dem Ort, an den der Bericht ging:

    ```text theme={null}
    CASE        WITH  W/OUT Δ      RUNS COST    NOTES
    first-case  1.00  0.33  +0.67  6    $0.41

    1 case(s) · mean Δ +0.67 · 74s · $0.41
    Report: /Users/you/my-plugin/evals/results/2026-09-10T17-02-11-482Z/report.html
    Published: https://claude.ai/... · keep local next time with --no-publish
    ```

    `WITH` ist der Score des Falls mit Ihrem Plugin geladen, `W/OUT` ist der Score ohne es, und ein positives `Δ` bedeutet, dass das Plugin den Score erhöht hat. `COST` ist eine Listenpreis-Schätzung der Modellaufrufe, und `NOTES` zeigt die Erklärung des höchstgewichteten fehlgeschlagenen Graders oder den Fehler des Durchlaufs aus dem With-Arm.
  </Step>

  <Step title="Öffnen Sie den Bericht und iterieren Sie">
    Öffnen Sie die `Published:` URL oder den `Report:` Pfad, wenn keine `Published:` Zeile angezeigt wird, um das Urteil jedes Graders und die Erklärung für jeden Durchlauf zu sehen, und für `llm` Grader die Stimmen des Judges und den Auszug, den er bewertet hat. Die `Published:` Zeile wird nur angezeigt, wenn Ihr Konto [Berichte veröffentlichen](#html-report) kann.

    Der häufigste erste Fund ist ein `Δ` nahe Null mit dem fehlgeschlagenen `tool_used: Skill` Grader des Falls, was bedeutet, dass Claude Ihren Skill bei natürlicher Formulierung nicht auswählt. Passen Sie die [`description`](/docs/de/skills#frontmatter-reference) des Skills an, führen Sie `claude plugin eval .` erneut aus und vergleichen Sie.

    Um einen Fall kostengünstig zu iterieren, führen Sie einen einzelnen Arm einmal aus. Ein einzelner Durchlauf ist verrauscht, daher bestätigen Sie jede Änderung bei den standardmäßigen drei Durchläufen, bevor Sie ihr vertrauen. Mit einem Arm zeigt die Tabelle `SCORE` und `PASS%` Spalten statt `WITH`, `W/OUT` und `Δ`:

    ```bash theme={null}
    claude plugin eval . --case <case-name> --runs 1 --ablation none
    ```

    Ersetzen Sie `<case-name>` durch einen der Verzeichnisnamen unter `evals/`.
  </Step>
</Steps>

<h2 id="write-and-refine-cases">
  Fälle schreiben und verfeinern
</h2>

Die Fälle, die `claude plugin eval init` schreibt, sind einfache Dateien, die Sie öffnen, ändern und hinzufügen können. Ein Fall ist ein Verzeichnis unter dem Eval-Verzeichnis des Plugins, das eine `prompt.md`, eine `case.yaml` oder beide enthält. Um Fälle zu gruppieren, verschachteln Sie sie unter einem Verzeichnis, das selbst kein Fall ist; alles innerhalb eines Fallverzeichnisses, wie `graders/` und Fixture-Dateien, gehört zu diesem Fall.

Dies ist das Layout, das `claude plugin eval init` schreibt und das für neue Suites verwendet werden sollte. Die [Eval-Suite-Referenz](#eval-suite-reference) hat den vollständigen Baum, einschließlich Mocks und Ergebnisse:

```text theme={null}
my-plugin/
├── .claude-plugin/plugin.json
├── skills/...
└── evals/
    ├── first-case/
    │   ├── prompt.md          # frontmatter: case fields; body: the prompt
    │   ├── graders/
    │   │   ├── criteria.md    # frontmatter: type + options; body: rubric or pattern
    │   │   └── skill-fired.md
    │   └── case.yaml          # optional: only for context.* fields
    ├── ignores-unrelated-request/
    │   └── ...
    └── results/               # written by each run; add to .gitignore
```

<h3 id="write-a-case-manually">
  Schreiben Sie einen Fall von Hand
</h3>

Der empfohlene Weg ist, Claude die Fälle mit `claude plugin eval init` schreiben zu lassen. Um stattdessen einen selbst zu schreiben, beginnen Sie mit einer leeren Vorlage. Der folgende Befehl schreibt einen Fall namens `first-case` mit einer Platzhalter-`prompt.md` und einem Platzhalter-Grader und führt nichts aus:

```bash theme={null}
claude plugin eval init --bare first-case
```

```text theme={null}
evals/first-case/
├── prompt.md            # the prompt sent to Claude, plus run limits
└── graders/
    └── criteria.md      # one grader: how to score the result
```

In `prompt.md` schreiben Sie die Nachricht, die Claude in jedem Durchlauf erhält, und legen die Limits und Tools des Durchlaufs in seinem Frontmatter fest. Öffnen Sie `evals/first-case/prompt.md` und ersetzen Sie den Platzhalter-Body durch eine Anfrage, die einer Ihrer Skills verarbeiten sollte, formuliert wie ein Benutzer sie eingeben würde, anstatt den Skill zu benennen. Dieses Beispiel ist für einen Skill, der Commit-Nachrichten entwirft; verwenden Sie Ihre eigene Anfrage:

```markdown theme={null}
---
max_turns: 10
allowed_tools: [Read, Glob, Grep, Skill]
---

Write me a commit message for this change: I renamed getUser to fetchUser and updated the three call sites.
```

Jeder Durchlauf startet in einem leeren Arbeitsverzeichnis, daher legen Sie alles, was die Aufgabe benötigt, in die Eingabeaufforderung selbst, oder [richten Sie den Arbeitsbereich zuerst ein](#add-setup-or-history-with-case-yaml). Die [vollständige Liste der Frontmatter-Felder](#prompt-md-fields) behandelt das Modell, Timeout, Tags und Umgebungsvariablen.

Jede Datei unter `graders/` ist eine Prüfung, die nach dem Durchlauf angewendet wird. Öffnen Sie `evals/first-case/graders/criteria.md` und ersetzen Sie den Platzhalter durch eine Rubrik für das Judge-Modell, geschrieben als konkrete PASS- und FAIL-Bedingungen:

```markdown theme={null}
---
type: llm
---

PASS if <what a correct response contains>.
FAIL if <what a wrong or missing response looks like>.
```

Fügen Sie dann einen zweiten Grader hinzu, der überprüft, ob Ihr Skill das ist, was die Antwort produziert hat. Erstellen Sie `evals/first-case/graders/skill-fired.md`, ersetzen Sie `your-skill-name` durch den Skill-Verzeichnisnamen unter `skills/`, das ist der Name, unter dem Claude ihn aufruft:

```markdown theme={null}
---
type: tool_used
tool: Skill
input_match: '"skill"\s*:\s*"(?:[\w-]+:)?your-skill-name"'
---
```

Dies wird bestanden, wenn Claude diesen Skill mindestens einmal während des Durchlaufs aufgerufen hat, einschließlich seiner namensraum-qualifizierten Form `plugin-name:skill-name`.

[Grader-Typen](#grader-types) listet die anderen verfügbaren Prüfungen auf, wie z. B. das Abgleichen eines Regex oder das Bestätigen, dass eine Datei erstellt wurde.

Wenn beide Dateien gespeichert sind, führen Sie den Fall wie die [Schnellstart](#create-your-first-eval-suite) mit `claude plugin eval .` vom Plugin-Stammverzeichnis aus.

<h3 id="set-run-limits-and-tools-in-prompt-md">
  Legen Sie Durchlauf-Limits und Tools in prompt.md fest
</h3>

Legen Sie `max_turns`, `timeout_seconds`, `model`, `tags` und die `allowed_tools` eines Falls in der Frontmatter von `prompt.md` fest; die [prompt.md Frontmatter](#prompt-md-fields) Referenz listet jedes Feld und seinen Standard auf.

Claude erhält den Body genau wie Sie ihn geschrieben haben. `@path` Erwähnungen darin werden nicht in Datei-Anhänge erweitert, daher müssen Sie, wenn Claude eine Datei lesen muss, ein Tool dafür in `allowed_tools` gewähren.

<h3 id="grade-the-result">
  Wählen Sie Grader und gewichten Sie sie
</h3>

Die Frontmatter eines Graders legt seinen `type` fest und optional ein `weight`, das ihn für mehr des Scores des Durchlaufs zählen lässt, und einen [`arm`](#compare-against-a-no-plugin-baseline), der steuert, wie er gegen die Baseline bewertet wird. Von den sechs Typen werden `regex`, `tool_used`, `tool_order` und `file_exists` aus dem Transkript und den Dateien berechnet und kosten nichts, während `llm` und `baseline` ein Judge-Modell aufrufen und zu den Kosten des Durchlaufs beitragen.

Es gibt keine benutzerdefinierten Code-Grader.

[Grader-Typen](#grader-types) listet die Optionen und Bestanden-Bedingungen jedes Typs auf, und [was ein Grader ansehen kann](#what-a-grader-can-look-at) listet die Werte auf, die `target` und `focus` akzeptieren.

Der Judge für `llm` und `baseline` Grader ist standardmäßig ein kleines schnelles Modell. Übergeben Sie `--judge-model sonnet` oder eine vollständige Modell-ID, um ein stärkeres für nuancierte Rubriken zu verwenden.

<h4 id="choose-graders-that-give-a-stable-signal">
  Wählen Sie Grader, die ein stabiles Signal geben
</h4>

Ein `llm` Grader fragt ein Modell nach einem Urteil, daher kann seine Antwort zwischen Durchläufen unterschiedlich sein, und sie unterscheidet sich mehr, je länger der Text ist, den es lesen muss. Diese Gewohnheiten halten die Scores einer Suite stabil genug, um ihnen zu vertrauen:

* Für lange Ausgaben wie eine generierte Datei bewerten Sie sie mit einem `regex` Grader über den Inhalt der Datei, der die ganze Datei jedes Mal gleich überprüft. Behalten Sie `llm` Grader für kurze Ausgaben, mit Rubriken, die als konkrete PASS- und FAIL-Bedingungen geschrieben sind.
* Geben Sie jedem Fall einen Grader auf das Ergebnis, wie die endgültige Nachricht oder eine produzierte Datei, und einen auf wie Claude dorthin kam, wie `tool_used` oder `tool_order`. Zusammen sagen sie Ihnen sowohl, ob die Antwort richtig war, als auch ob Ihr Plugin sie produziert hat.
* Wenn ein `tool_used: Skill` Grader eines Falls bestanden wird, aber `Δ` negativ ist, verdächtigen Sie den Judge vor dem Plugin. Ein kleines Judge-Modell kann eine korrekte Antwort als falsch markieren, weil sie anders formatiert ist als das, was die Rubrik beschreibt. Führen Sie erneut mit `--judge-model sonnet` aus und straffen Sie die Rubrik, damit die Formatierung das Urteil nicht entscheidet.
* Um zu überprüfen, dass ein Build oder Test innerhalb des Durchlaufs bestanden wurde, bitten Sie die Eingabeaufforderung Claude, ihn auszuführen und das Ergebnis in eine Datei zu schreiben, bewerten Sie diese Datei und bestätigen Sie, dass der Befehl mit einem `tool_used` Grader ausgeführt wurde, dessen `input_match` den Befehl benennt.

<h3 id="compare-against-a-no-plugin-baseline">
  Bewerten Sie gegen die Baseline ohne Plugin
</h3>

Wenn ein Plugin unter Test steht, wird jeder Fall standardmäßig in zwei Arms ausgeführt. Der With-Arm ist seine Durchläufe mit dem Plugin geladen, und der Without-Arm ist die gleiche Anzahl von Durchläufen ohne Plugin. Die Zusammenfassung und der Bericht zeigen beide Scores und `Δ`, den With-Arm Score minus den Without-Arm Score.

Übergeben Sie `--ablation none`, um nur den With-Arm auszuführen, was die Kosten halbiert, wenn Sie den Vergleich nicht benötigen, wie z. B. beim Iterieren über Grader.

In einem Zwei-Arm-Durchlauf werden einige Grader mit `scored: false` gemeldet. Eine Prüfung wie „der Skill wurde aufgerufen" kann ohne das Plugin nie bestanden werden, daher würde das Zählen den Without-Arm gegen Null drücken und `Δ` aufblasen. Um die beiden Arms vergleichbar zu halten, schließt Claude Code solche Grader aus der Bewertung in beiden Arms aus und meldet sie im With-Arm nur als Bestanden/Nicht-Bestanden-Indikatoren. Das umfasst:

* Jeden `tool_used` Grader, dessen `tool` `Skill` ist
* Jeden `regex` Grader mit `target: mock_calls` und jeden `llm` Grader mit `focus: mock_calls`, wenn jeder [gemockte Server](#mock-mcp-servers) im Fall einer ist, den Ihr Plugin deklariert
* Jeden Grader, den Sie mit `arm: with-only` markieren

Drei Einstellungen ändern diese Ausschließung:

* **Jeder Grader ausgeschlossen**: Wenn jeder Grader in einem Fall in der ausgeschlossenen Menge ist, werden sie stattdessen normal bewertet, da es nichts mehr zu bewerten gäbe.
* **`arm: both`**: Setzen Sie `arm: both` auf einen Grader, um ihn in beiden Arms unabhängig zu bewerten, was Sie für eine „darf den Skill nicht aufrufen" Prüfung mit `min: 0` und `max: 0` wollen.
* **`--ablation none`**: Unter `--ablation none` wird nichts ausgeschlossen, daher kann die gleiche Suite in den beiden Modi einen anderen absoluten Score produzieren.

<h3 id="use-a-different-eval-directory">
  Verwenden Sie ein anderes Eval-Verzeichnis
</h3>

Wenn `evals/` bereits von einem anderen Tool verwendet wird, behalten Sie die Suite in einem anderen Verzeichnis. Sie können dieses Verzeichnis in der `plugin.json` des Plugins aufzeichnen, damit jeder Durchlauf und jeder Mitarbeiter es verwendet, oder übergeben Sie es in der Befehlszeile für einen einzelnen Durchlauf:

* **In `plugin.json`**: fügen Sie `"experimental": { "evals": "quality/evals" }` hinzu.
* **In der Befehlszeile**: übergeben Sie `--eval-dir quality/evals` an sowohl `claude plugin eval` als auch `claude plugin eval init`.

Wenn Sie beide setzen, wird das Verzeichnis der Flag verwendet. Geben Sie einen relativen Pfad aus einfachen Verzeichnisnamen wie `qa` oder `quality/evals` an; ein absoluter Pfad oder einer mit `..` wird abgelehnt: als Flag-Wert ist es ein Fehler, während ein nicht verwendbarer Manifest-Wert eine `Warning:` Zeile druckt und der Durchlauf stattdessen `evals/` verwendet. Fälle, Ergebnisse und `init` Ausgabe verschieben sich alle in dieses Verzeichnis.

<h2 id="set-up-fixtures-and-mocks">
  Richten Sie Fixtures und Mocks ein
</h2>

Ein Fall kann mehr als eine Eingabeaufforderung benötigen: Dateien oder ein Git-Repository im Arbeitsbereich, eine frühere Konversation zum Fortsetzen oder Antworten von den MCP-Servern, mit denen Ihr Plugin verbunden ist. Jede davon wird neben dem Fall eingerichtet, damit Durchläufe wiederholbar bleiben.

<h3 id="add-setup-or-history-with-case-yaml">
  Säen Sie den Arbeitsbereich oder die Konversation
</h3>

Jeder Durchlauf startet in einem leeren Arbeitsbereich. Wenn ein Fall mehr als die Eingabeaufforderung benötigt, fügen Sie neben `prompt.md` eine `case.yaml` mit einem `context` Block hinzu:

* **Fixture-Dateien oder ein Git-Repository**: Schreiben Sie ein Bash-Skript im Fallverzeichnis und benennen Sie es in `context.scaffold_script`. Das Skript wird als Sie ausgeführt, außerhalb der Sandbox des Agenten, und nur wenn Sie `--scaffold` übergeben, daher übergeben Sie dieses Flag nur für Suites, die Sie oder Ihre Organisation geschrieben haben.
* **Eine frühere Konversation zum Fortsetzen**: Speichern Sie das Transkript als `.jsonl` Datei und benennen Sie es in `context.history_file`, und die Eingabeaufforderung des Falls wird zum nächsten Benutzer-Turn.
* **Fixture-Verzeichnisse, die Claude während des Durchlaufs lesen kann**: Listen Sie sie in `context.add_dirs` auf.

Eine `case.yaml` benötigt auch `schema_version: "1.1"` und `name`; die [case.yaml Felder](#case-yaml-fields) Referenz hat die vollständige Liste.

Diese `case.yaml` säet einen Arbeitsbereich aus einem Skript und lässt Claude Fixtures aus einem `resources/` Verzeichnis lesen:

```yaml theme={null}
schema_version: "1.1"
name: changelog-from-diff
tags: [smoke]
context:
  scaffold_script: fixture.sh
  add_dirs: [resources]
```

<h3 id="mock-mcp-servers">
  Mock MCP Server
</h3>

Sie können ein Plugin evaluieren, dessen Skills MCP-Tools aufrufen, ohne den echten Service dahinter. Legen Sie eine Markdown-Datei pro Tool unter `evals/mocks/<server>/<tool>.md` für die ganze Suite oder unter einem eigenen `mocks/` Verzeichnis eines Falls für einen Fall, wobei `<server>` der Name des Servers in der [MCP-Konfiguration](/docs/de/plugins/components#mcp-servers) Ihres Plugins ist.

Ein Durchlauf startet niemals die echten MCP-Server Ihres Plugins, es sei denn, Sie fragen danach. Claude Code registriert einen Ersatz-Server unter jedem Server-Namen. Tools mit einer Mock-Datei antworten daraus und sind ohne einen `--allow-tools` Grant erlaubt, und ein Tool ohne Mock-Datei ist für Claude nicht verfügbar. Ein Server ohne Mocks überhaupt erscheint in der `mocked:` Fortschrittszeile des Falls als `plugin_<plugin>_<server>[not started: no mock]`.

Der Body der Datei ist das, was das Tool an Claude zurückgibt. Dieser Mock steht für ein `create_issue` Tool auf einem Server namens `tracker` ein, überprüft die Eingabe, die Claude sendet, und gibt den Titel zurück. Speichern Sie es als `evals/mocks/tracker/create_issue.md`:

```markdown theme={null}
---
expect:
  title: string
  priority: [low, medium, high]
---

Created issue #4821: {{input.title}}
```

Eine Mock-Datei Body und Frontmatter akzeptieren diese Optionen:

* **Substitutionen**: Fügen Sie Felder aus der Eingabe des Aufrufs mit `{{input.<field>}}` ein, und den Inhalt einer Fixture-Datei neben dem Mock mit `{{file:fixtures/{input.<field>}.json}}`.
* **`expect:`**: Der `expect:` Block schützt die Eingabe. Wenn ein Aufruf ihn verletzt, bricht der Durchlauf mit Score 0 ab und zeichnet auf, warum, daher kann ein Fall bestätigen, was Ihr Plugin den Server zu tun bat.
* **`error: true`**: Setzen Sie `error: true`, um den Body stattdessen als Tool-Fehler zurückzugeben.
* **`type: agent`**: Setzen Sie `type: agent`, um ein kleines Modell als Server aus Anweisungen im Body antworten zu lassen.

Die [Mock-Datei-Referenz](#mock-files) listet jeden Schlüssel und die `_server.md` und `_tools.json` Dateien auf.

Um die Aufrufe selbst zu bewerten, zeigen Sie einen Grader auf `target: mock_calls`.

Um stattdessen gegen die echten MCP-Server des Plugins auszuführen, übergeben Sie eines dieser Flags. Entweder Weg laufen diese Prozesse als Sie, außerhalb der Sandbox des Durchlaufs, und ihre Tools benötigen einen [`--allow-tools` Grant](#grant-tools):

* **`--allow-real-servers`**: starten Sie den echten Prozess für jeden Server, den Sie nicht gemockt haben, und beantworten Sie weiterhin gemockte Tools aus ihren Dateien
* **`--mocks off`**: ignorieren Sie `mocks/` vollständig und starten Sie jeden Server, den das Plugin deklariert

<h4 id="replay-agent-mock-answers">
  Wiedergeben Sie Agent-Mock-Antworten
</h4>

Ein `type: agent` Mock antwortet mit einem Aufruf an das [`--judge-model`](#command-options), daher variiert seine Ausgabe zwischen Durchläufen und ändert sich, wenn Sie den Judge ändern. Wenn ein Durchlauf ohne Fehler oder Abbruch abgeschlossen wird, speichert Claude Code jede Antwort, die ein Agent-Mock gab, unter dem Ergebnisverzeichnis in `mock-recordings/`.

Öffnen Sie `ADOPT.txt` dort, um jede Aufzeichnung und das `.replay/<server>/` Verzeichnis zu sehen, um es neben den Mock zu kopieren, der es produziert hat. Nachdem Sie eine Aufzeichnung dort kopiert haben, beantworten spätere Durchläufe den identischen Aufruf daraus ohne Modellaufruf. Committen Sie `mocks/.replay/` mit dem Rest von `mocks/`, damit CI-Durchläufe wiederholbar sind.

<h2 id="run-evals">
  Führen Sie Evals aus
</h2>

Sobald eine Suite existiert, führt `claude plugin eval` sie aus. Sie wählen, welches Plugin und welche Fälle mit dem Zielargument ausgeführt werden, gewähren alle Tools, die die Fälle über die schreibgeschützte Menge hinaus benötigen, mit `--allow-tools`, und steuern Durchlauf-Anzahl, Modelle, Kosten und Ausgabe mit den anderen Optionen.

<h3 id="choose-what-to-evaluate">
  Wählen Sie aus, was zu evaluieren ist
</h3>

Meistens führen Sie `claude plugin eval .` vom Plugin-Stammverzeichnis aus, was jeden Fall in der Suite mit dem Plugin ausführt, in dem Sie stehen, geladen. Um eine einzelne `prompt.md` oder `case.yaml` Datei auszuführen oder um ein Plugin zu evaluieren, das Sie installiert haben, anstatt eines, das Sie entwickeln, übergeben Sie ein anderes Ziel:

| Ziel                                                               | Was wird ausgeführt                                                                                                                                                                                                         |
| :----------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Ein Plugin-Stammverzeichnis, wie `.`                               | Jeder Fall unter seinem Eval-Verzeichnis, mit diesem Plugin geladen                                                                                                                                                         |
| Eine einzelne `prompt.md` oder `case.yaml` Datei                   | Dieser Fall, mit seinem einschließenden Plugin geladen                                                                                                                                                                      |
| Ein installiertes Plugin nach Name, `name` oder `name@marketplace` | Die Fälle im Eval-Verzeichnis der installierten Kopie, mit der installierten Kopie geladen. Ergebnisse werden unter `./evals/results/` in Ihrem aktuellen Verzeichnis geschrieben, oder `./<dir>/results/` mit `--eval-dir` |
| `name@skills-dir`                                                  | Das gleiche, für ein [Skills-Directory-Plugin](/docs/de/plugins/loading#plugins-shared-through-a-repository)                                                                                                                     |
| Weggelassen                                                        | Das aktuelle Verzeichnis als Pfad                                                                                                                                                                                           |

Fügen Sie `--case <glob>` hinzu, um nach Fallname zu filtern, und `--tag <tag>`, um Fälle mit beliebigen der angegebenen Tags zu behalten.

Legen Sie das Ziel vor `--tag`, `--allow-tools` und `--json`. Die ersten beiden nehmen eine Liste und `--json` nimmt einen optionalen Pfad, daher liest jede von ihnen ein Ziel, das folgt, als ihren eigenen Wert.

<h3 id="grant-tools">
  Gewähren Sie Tools
</h3>

Durchläufe halten niemals an, um um Erlaubnis zu fragen. Eingebaute Tools, die einen Grant benötigen, den Sie nicht gegeben haben, wie `Bash`, `Write`, `Edit`, `WebFetch` und `WebSearch`, werden aus der Sitzung entfernt, daher kann Claude sie überhaupt nicht aufrufen.

Ein Durchlauf erlaubt nur die schreibgeschützten Tools, die der Fall in `allowed_tools` auflistet, aus `Read`, `Glob`, `Grep`, `NotebookRead`, `Skill`, `AskUserQuestion`, `Agent`, `TodoWrite` und die Task-Tools `TaskCreate`, `TaskGet`, `TaskList`, `TaskUpdate` und `TaskStop`, plus alles, was Sie mit `--allow-tools` gewähren. Dieser Grant gilt für jeden Fall in dem Durchlauf. Um Fällen die Verwendung von `Bash`, `Write`, `Edit`, `WebFetch` oder `WebSearch` zu ermöglichen, gewähren Sie sie selbst:

```bash theme={null}
claude plugin eval . --allow-tools Write Edit "Bash(npm test *)"
```

Wenn ein Fall ein Tool fragte, das Sie nicht gewährt haben, listet die Fortschrittsausgabe es als `not granted` auf. Tools auf einem [gemockten](#mock-mcp-servers) MCP-Server benötigen keinen Grant. Tools auf einem echten Plugin-MCP-Server benötigen sowohl den Server gestartet, mit `--allow-real-servers` oder `--mocks off`, als auch einen Grant nach Name, wie `--allow-tools "mcp__plugin_my-plugin_github__*"`; Plugin-MCP-Tools werden `mcp__plugin_<plugin>_<server>__<tool>` benannt.

Wenn Sie `Bash` in irgendeiner Form gewähren, wird jeder Befehl unter Claude Code's [OS-Level Sandbox](/docs/de/sandboxing) ausgeführt. Schreibvorgänge sind auf den Arbeitsbereich des Durchlaufs beschränkt, Ihr Home-Verzeichnis und die Claude Code Konfiguration sind nicht lesbar, und der Netzwerkzugriff ist auf Domains beschränkt, die Sie mit `--allow-tools "WebFetch(domain:example.com)"` gewähren. Wenn Sie Bash oder PowerShell auf einer Maschine ohne Sandbox-Backend gewähren, weigert sich Claude Code jeden Durchlauf, anstatt ihn uneingeschränkt auszuführen, und der Fall zeigt einen Durchlauf-Fehler und normalerweise Score 0. Natives Windows hat kein Backend, daher führen Sie Shell-gewährende Suites unter WSL2 aus; unter Linux installieren Sie zuerst `bubblewrap` und `socat`. Siehe die [Sandboxing-Voraussetzungen](/docs/de/sandboxing).

<h3 id="command-options">
  Befehlsoptionen
</h3>

Diese Tabelle behandelt die Optionen für Durchlauf-Anzahl, Modelle, Bewertung, Kosten, Tool-Grants, Mocks und Ausgabe. Führen Sie `claude plugin eval --help` für die vollständige Liste aus, die auch `--case`, `--tag`, `--eval-dir`, `--no-scaffold`, `--report` und `--verbose` umfasst.

| Option                     | Standard                                                                                 | Effekt                                                                                                                                                                                                                                                                                                                                                                                                        |
| :------------------------- | :--------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `--runs <n>`               | Jedes Falls `runs`, sonst 3                                                              | Durchläufe pro Fall pro Arm                                                                                                                                                                                                                                                                                                                                                                                   |
| `-j`, `--concurrency <n>`  | `1`                                                                                      | Führen Sie bis zu dieser vielen Agent-Durchläufen gleichzeitig aus, von 1 bis 8. Sie teilen sich das Rate-Limit Ihres Kontos, daher verkürzt dies die Wall-Clock-Zeit, anstatt den Durchsatz über dieses Limit hinaus zu erhöhen. Ergebnisse behalten Fallreihenfolge                                                                                                                                         |
| `--model <model>`          | Jedes Falls `model`, sonst `ANTHROPIC_MODEL`, wenn gesetzt, sonst Claude Code's Standard | Modell für den Agent unter Test. Heften Sie es in CI fest, damit ein Modell-Rollout nicht mit einer Plugin-Regression verwechselt wird                                                                                                                                                                                                                                                                        |
| `--judge-model <model>`    | Ein kleines schnelles Modell                                                             | Modell für `llm` und `baseline` Grader                                                                                                                                                                                                                                                                                                                                                                        |
| `--ablation <mode>`        | `with-without`, wenn ein Plugin aufgelöst wird, sonst `none`                             | Ob auch jeden Fall ohne das Plugin auszuführen, um zu messen, was es hinzufügt. `none` führt einen Arm aus; `with-without` fügt die Baseline ohne Plugin hinzu                                                                                                                                                                                                                                                |
| `--threshold <0..1>`       | `1.0`                                                                                    | Ein Fall wird bestanden, wenn sein With-Arm Score mindestens dies ist. Jeder Fall darunter lässt den Befehl mit 1 beenden                                                                                                                                                                                                                                                                                     |
| `--max-cost-usd <usd>`     | Keine Obergrenze                                                                         | Eine Obergrenze auf die Listenpreis-Kostenschätzung des Durchlaufs, nicht auf die Plan-Nutzung. Überprüft, bevor jeder Durchlauf startet. Einmal ausgegeben, startet nichts Weiteres; Durchläufe, die bereits in Flug sind, beenden sich, daher kann die Ausgabe die Obergrenze um diese Durchläufe überschreiten. Wenn ein Durchlauf nicht gestartet wird, beendet sich der Befehl mit 2 mit Teilergebnissen |
| `--allow-tools <tools...>` | Keine                                                                                    | Gewähren Sie Tools über die schreibgeschützte Menge. Siehe [Gewähren Sie Tools](#grant-tools)                                                                                                                                                                                                                                                                                                                 |
| `--scaffold`               | Aus                                                                                      | Führen Sie das [`scaffold_script`](#add-setup-or-history-with-case-yaml) jedes Falls aus                                                                                                                                                                                                                                                                                                                      |
| `--trust-plugin`           | Aus                                                                                      | Überspringen Sie die erste Vertrauens-Eingabeaufforderung für ein Plugin, dessen Code und Suite Sie selbst ausführen würden. Übergeben Sie es in CI, damit der Job niemals durch die Eingabeaufforderung verweigert oder daran warten gelassen wird. Siehe [Was ein Durchlauf zugreifen kann](#security)                                                                                                      |
| `--mocks <mode>`           | `record`                                                                                 | `record` beantwortet MCP-Tool-Aufrufe aus [Mocks](#mock-mcp-servers), startet nicht die echten Server des Plugins und speichert Agent-Mock-Antworten zur Wiedergabe. `off` ignoriert Mocks und startet die echten MCP-Server des Plugins                                                                                                                                                                      |
| `--allow-real-servers`     | Aus                                                                                      | Mit `--mocks record`, starten Sie auch die echten MCP-Server des Plugins für Server, die kein Mock haben                                                                                                                                                                                                                                                                                                      |
| `--json [path]`            | Aus                                                                                      | Drucken Sie das [Ergebnis-Dokument](#json-result) auf stdout, oder schreiben Sie es in einen Pfad, der auf `.json` endet. Der Durchlauf ist ruhig: keine Fortschrittszeilen oder Zusammenfassungstabelle                                                                                                                                                                                                      |
| `--output-dir <dir>`       | `<eval dir>/results/<timestamp>/`                                                        | Wo `aggregate-result.json` und `report.html` hingehen                                                                                                                                                                                                                                                                                                                                                         |
| `--no-publish`             |                                                                                          | Behalten Sie den HTML-Bericht lokal. Siehe [HTML-Bericht](#html-report)                                                                                                                                                                                                                                                                                                                                       |
| `--publish-report`         |                                                                                          | Veröffentlichen Sie den Bericht auch dort, wo er standardmäßig lokal bleiben würde, wie ein Durchlauf, den eine Claude Code Sitzung gestartet hat                                                                                                                                                                                                                                                             |
| `--keep-temp`              | Aus                                                                                      | Behalten Sie das Sandbox-Verzeichnis jedes Durchlaufs und drucken Sie seinen Pfad zum Debuggen, was Claude produziert hat                                                                                                                                                                                                                                                                                     |

<h3 id="run-evals-in-ci">
  Führen Sie Evals in CI aus
</h3>

Führen Sie in Ihrem CI-Job die Suite mit `--json` aus, um das Ergebnis zum Archivieren zu schreiben, und lassen Sie den Build beim Exit-Code fehlschlagen. Übergeben Sie `--trust-plugin`, damit der Job niemals bei der [ersten Vertrauens-Eingabeaufforderung](#security) wartet, heften Sie beide Modelle fest, damit Scores über die Zeit vergleichbar sind, behalten Sie den Bericht lokal und setzen Sie eine Kostendecke als Obergrenze:

```bash theme={null}
claude plugin eval . \
  --trust-plugin \
  --json results.json \
  --threshold 0.8 \
  --model claude-sonnet-5 \
  --judge-model claude-haiku-4-5 \
  --no-publish \
  --max-cost-usd 20
```

Der Exit-Code des Jobs sagt Ihnen, was passiert ist:

| Exit-Code | Bedeutung                                                                                                                                                                                                                                                                            |
| :-------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 0         | Jeder Fall hat bei oder über `--threshold` bewertet und jede Falldatei geladen                                                                                                                                                                                                       |
| 1         | Ein Fall hat unter dem Threshold bewertet, eine Falldatei konnte nicht geladen werden, keine Fälle wurden gefunden, ein Durchlauf konnte nicht gestartet werden, das Plugin-Verzeichnis ist nicht vertraut und `--trust-plugin` wurde nicht übergeben, oder eine Option war ungültig |
| 2         | Teilweiser Durchlauf: die `--max-cost-usd` Obergrenze wurde erreicht, oder Ihre Anmeldedaten wurden vor oder beim ersten Durchlauf abgelehnt. `results.json` wird immer noch mit `partial: true` und dem Grund geschrieben                                                           |
| 130       | Unterbrochen. Teilergebnisse werden geschrieben                                                                                                                                                                                                                                      |
| 143       | Beendet, wie durch ein CI-Timeout                                                                                                                                                                                                                                                    |

Probleme beim Schreiben oder Veröffentlichen des HTML-Berichts ändern niemals den Exit-Code.

Um zu sehen, warum ein Fall niedrig bewertet wurde, führen Sie ihn lokal ohne `--json` aus, damit die Pro-Durchlauf-Fortschritts- und Grader-Zeilen drucken.

Ein CI-Runner benötigt auch diese:

* **Installation und Anmeldedaten**: Ein CI-Runner benötigt eine Claude Code Installation und [Anmeldedaten in der Umgebung](/docs/de/authentication) wie `ANTHROPIC_API_KEY`.
* **Vertrauen**: Ohne `--trust-plugin` wird ein Job, dessen Checkout-Verzeichnis Claude Code noch nicht vertraut, die [erste Vertrauens-Eingabeaufforderung](#security) benötigen, und ein Durchlauf, der nicht fragen kann, wird mit Exit 1 verweigert.
* **`init` in CI**: `claude plugin eval init` benötigt ein Terminal, um Ihnen seine Fragen zu stellen; führen Sie in CI `claude plugin eval init --bare <name>` aus, um die leere Vorlage zu erhalten.

Um Kosten vorhersehbar zu halten, geben Sie schnelle Every-Change-Suites nur Grader, die keinen Judge aufrufen, verwenden Sie `--ablation none`, wo Sie `Δ` nicht benötigen, und lassen Sie `partial: true` Dokumente und Durchläufe mit `skippedPaidGraders` aus jedem Trend, den Sie kartieren.

<h2 id="read-the-results">
  Lesen Sie die Ergebnisse
</h2>

Jeder Durchlauf mit mindestens einem Fall schreibt ein `results/<timestamp>/` Verzeichnis innerhalb des Eval-Verzeichnisses, das `aggregate-result.json` und `report.html` enthält. Für ein Pfad-Ziel, das unter dem Plugin liegt; für ein Plugin, das Sie benannt haben, liegt es unter Ihrem aktuellen Verzeichnis, wie die [Ziel-Tabelle](#choose-what-to-evaluate) zeigt. Die Zusammenfassungstabelle, das JSON und der Bericht rendern alle die gleichen Ergebnisdaten.

<h3 id="html-report">
  HTML-Bericht
</h3>

`report.html` ist eine einzelne in sich geschlossene Datei, die keine externen Anfragen stellt, daher können Sie sie an einen CI-Job anhängen oder von der Festplatte öffnen. Dieses Beispiel ist die Oberseite eines Berichts für einen Drei-Fall-Suite-Durchlauf mit `--threshold 0.8`; die angezeigten Kosten sind eine Listenpreis-Schätzung und variieren je nach Modell und Anzahl der Fälle:

<img src="https://mintcdn.com/claude-code/qq7LHDi_F0aeFHgk/images/plugin-eval-report.png?fit=max&auto=format&n=qq7LHDi_F0aeFHgk&q=85&s=106eb6e6a70a6565f891ea3a4564f87d" alt="Oberseite eines Eval-Berichts: eine Urteilszeile mit dem Text „Plugin-Effekt: +33,3 Punkte gegenüber Baseline, 2 verbessert, 1 gleich, 0 von 3 Fällen verschlechtert&#x22;, fünf Zusammenfassungs-Kacheln für Suite-Score, Ablations-Delta, Baseline-Score, Fälle, die den Schwellenwert erfüllen, und perfekte Durchläufe, dann der erste Fall mit seinem Delta, Score-Balken und einem Durchlauf, dessen zwei Grader beide „bestanden&#x22; anzeigen" width="1360" height="1032" data-path="images/plugin-eval-report.png" />

Lesen Sie es von oben nach unten:

* **Die Urteilszeile und Kacheln** beantworten die Frage, ob das Plugin über die gesamte Suite hinweg geholfen hat. Suite-Score ist der Durchschnitt der Pro-Fall-Scores mit Plugin, Ablations-Δ ist, wie weit dieser über oder unter dem Baseline-Score liegt, und Fälle zählt, wie viele den Schwellenwert erfüllt haben. Perfekte Durchläufe ist der Anteil der Mit-Plugin-Durchläufe, bei denen jeder Grader bestanden hat.
* **Jede Fall-Karte** zeigt das `Δ` des Falls und den Score mit Plugin, mit einem Häkchen auf dem Balken beim Schwellenwert. Ein Fall, dessen `Δ` negativ ist, erhält eine rote linke Kante, daher heben sich Verschlechterungen beim Scrollen ab.
* **Innerhalb eines Falls** kommen die Mit-Plugin-Durchläufe zuerst und die Baseline-Durchläufe danach. Jeder Durchlauf listet seine Grader mit einem Bestanden- oder Nicht-Bestanden-Chip auf. Ein fehlgeschlagener Grader ist bereits erweitert mit seiner Erklärung, und ein `llm` Grader zeigt auch die Stimmen des Richters und die Beweise, die ihm gezeigt wurden, was ist, wo Sie herausfinden, warum ein Durchlauf niedrig bewertet wurde. Grader, die nicht zur Bewertung zählen, wie `tool_used: Skill`, tragen ein `plugin-fired indicator` Badge.
* **Eingabeaufforderung und Grader**, unter den Durchläufen, zeigen die Eingabeaufforderung des Falls und die Rubrik oder das Muster jedes Graders, damit jemand, der den Bericht ohne die Suite liest, sehen kann, was gefragt wurde und was als gut zählte.

Wenn Sie mit einem claude.ai Abonnement angemeldet sind und [Artifacts](/docs/de/artifacts) für Ihr Konto verfügbar sind, veröffentlicht Claude Code den Bericht auch als privates Artifact und druckt `Published: <url>`. Übergeben Sie `--no-publish`, um ihn lokal zu behalten. Wenn keine `Published:` Zeile angezeigt wird, wie bei API-Schlüssel-Authentifizierung, ist die lokale Datei der Bericht.

Ein Durchlauf, den eine Claude Code Sitzung gestartet hat, wie wenn Sie Claude bitten, die Suite für Sie auszuführen, bleibt auch lokal, und seine `Report:` Zeile sagt `kept local`. Fügen Sie `--publish-report` zu diesem Befehl hinzu, um ihn zu veröffentlichen.

<h3 id="json-result">
  JSON-Ergebnis
</h3>

`aggregate-result.json` und `--json` Ausgabe ist ein versioniertes Dokument mit `schemaVersion: 1` für CI-Skripte zum Parsen. Feldnamen sind camelCase und neue Felder werden hinzugefügt, ohne bestehende umzubenennen, daher schreiben Sie Ihr Skript, um Felder zu ignorieren, die es nicht erkennt.

Dies sind die Felder, die ein Gating-Skript normalerweise liest. Das Dokument trägt auch die Suite-Konfiguration, jede Grader-Definition und Pro-Durchlauf-Grader-Ergebnisse mit Erklärungen und Beweisen:

| Feld                                              | Bedeutung                                                                                                                                                                                                                                   |
| :------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `partial`, `partialReason`                        | `true` mit `cost_ceiling`, `interrupted` oder `auth_failed`, wenn die Suite nicht fertig wurde. Lassen Sie Teilergebnisse aus Trend-Diagrammen                                                                                              |
| `aggregates.overallScore`                         | Mittlerer Fall-Score über die Suite                                                                                                                                                                                                         |
| `aggregates.casesPassed`, `aggregates.casesTotal` | Fälle bei oder über `--threshold` und die Gesamtzahl                                                                                                                                                                                        |
| `aggregates.meanDelta`                            | Mittleres `Δ` über Fälle, unter dem Zwei-Arm-Modus                                                                                                                                                                                          |
| `cases[].name`                                    | Fallname                                                                                                                                                                                                                                    |
| `cases[].aggregates.score`                        | Mittlerer With-Arm-Durchlauf-Score für den Fall                                                                                                                                                                                             |
| `cases[].aggregates.delta`                        | With-Arm Score minus Without-Arm Score. Weggelassen, wenn die Arms nicht vergleichbar sind                                                                                                                                                  |
| `cases[].arms.with[].error`                       | `null`, oder warum ein Durchlauf abnormal endete, wie `timed out after 300s`. Ein Durchlauf, der gestartet, aber schlecht endete, wird immer noch auf das bewertet, was er produziert, daher impliziert ein nicht-null Fehler nicht Score 0 |
| `cases[].arms.with[].aborted`                     | Vorhanden, wenn ein [Mock](#mock-mcp-servers)'s `expect:` oder `abort_when` den Durchlauf stoppte, mit `server`, `tool` und `reason`. Der Durchlauf bewertet 0 und `error` bleibt `null`                                                    |
| `cases[].arms.with[].skippedPaidGraders`          | `true`, wenn die Kostendecke dieses Durchlaufs Judge-Grader übersprungen hat, daher ist sein Score nicht vergleichbar                                                                                                                       |
| `costUsd`, `durationSeconds`, `claudeVersion`     | Geschätzte Kosten zum Listenpreis einschließlich Judge-Aufrufe, Wall-Clock-Sekunden und die Claude Code Version, die die Suite ausgeführt hat                                                                                               |

<h2 id="security">
  Was ein Durchlauf zugreifen kann
</h2>

`claude plugin eval` lädt die Skills, Hooks und Agents des Ziel-Plugins und führt seine Eval-Suite auf Ihrem Computer als Sie aus. Es auf ein Plugin zu zeigen ist die gleiche Vertrauensentscheidung wie `claude --plugin-dir`, daher evaluieren Sie nur Plugins, denen Sie vertrauen.

Die in diesem Abschnitt beschriebene Isolation begrenzt, was der Agent unter Test erreichen kann; es ist keine Grenze gegen den Code des Plugins selbst, und eine Suite, die bestanden wird, sagt nichts darüber aus, ob das Plugin sicher ist.

<h3 id="trust-the-plugin-directory">
  Vertrauen Sie dem Plugin-Verzeichnis
</h3>

Das erste Mal, wenn Sie `claude plugin eval` gegen ein Verzeichnis ausführen, fragt Claude Code `Trust this plugin directory?`, bevor es etwas daraus lädt, es sei denn, Sie haben die Vertrauens-Eingabeaufforderung dort bereits in einer interaktiven `claude` Sitzung akzeptiert. Innerhalb eines Git-Repositorys vertraut das Beantworten mit Ja dem ganzen Repository, auch für interaktive Sitzungen. Wenn stdin oder stdout kein Terminal ist, unter `--json`, oder wenn die `CI` Umgebungsvariable auf einen wahren Wert wie `true` gesetzt ist, kann der Durchlauf nicht fragen und wird mit Exit 1 verweigert; übergeben Sie `--trust-plugin`, um das Vertrauen selbst zu bestätigen, nur für ein Plugin, das Sie auf Ihrem eigenen Computer ausführen würden. Ein Ziel, das Sie benennen, anstatt als Pfad zu geben, bedeutet ein installiertes Plugin oder ein Skills-Directory-Plugin, überspringt die Eingabeaufforderung.

Einige Teile des Plugins und der Suite werden nur ausgeführt, wenn Sie ihr Flag für diesen Durchlauf übergeben:

* Ein Falls [`scaffold_script`](#add-setup-or-history-with-case-yaml) mit `--scaffold`
* [Tools über die schreibgeschützte Menge](#grant-tools) mit `--allow-tools`
* Die [echten MCP-Server](#mock-mcp-servers) des Plugins mit `--allow-real-servers` oder `--mocks off`

Ein Falls `allowed_tools` und ein Skill's eigenes `allowed-tools` Frontmatter können keines von ihnen erweitern.

Wenn das Plugin Hooks ausliefert, die Sie nicht geschrieben haben, oder Sie seine echten MCP-Server starten, behandeln Sie seine Scores als beratend, es sei denn, Sie führen es in einer isolierten Umgebung wie einem Container oder CI-Runner aus, da Hooks und Server außerhalb der Sandbox des Agenten laufen und die Dateien berühren könnten, die die Grader lesen.

<h3 id="how-runs-are-isolated">
  Wie Durchläufe isoliert sind
</h3>

Jeder Durchlauf bekommt ein Wegwerf-Home-Verzeichnis, Arbeitsverzeichnis und Claude Code Konfiguration, und der Agent unter Test wird dort als `claude -p` Kind-Prozess mit nur Ihrem Plugin geladen ausgeführt. Behalten Sie diese Konsequenzen im Auge, wenn Sie Fälle schreiben:

* **Nichts Persönliches oder Projekt-Ebene lädt.** Ihre Benutzereinstellungen, Hooks, `CLAUDE.md` Dateien, MCP-Server, andere installierte Plugins, Memory und Skills sind abwesend, und kein Projekt-Scoped `.claude/` oder `.mcp.json` über der Sandbox wird gelesen. Die meiste Ihrer Shell-Umgebung wird auch zurückgehalten; nur eine [Allowlist](#prompt-md-fields) und `EVAL_*` Variablen erreichen den Durchlauf. Wenn das Plugin Setup benötigt, versenden Sie es im Plugin, erstellen Sie es in einem `scaffold_script` oder übergeben Sie `EVAL_*` Variablen.
* **Verwaltete Richtlinie kann einen Durchlauf immer noch einschränken.** Einschränkungen in [verwalteten Einstellungen](/docs/de/managed-settings), die ein Administrator auf der Maschine bereitgestellt hat, gelten innerhalb eines Durchlaufs, daher können Ergebnisse auf einer verwalteten Maschine von einer nicht verwalteten durch diese Richtlinie unterschiedlich sein.
* **Das Artifact-Tool ist aus.** Ein Skill, der ein [Artifact](/docs/de/artifacts) veröffentlicht, kann nur auf das bewertet werden, was er vor diesem Schritt produziert.
* **Die Falldefinitionen sind vor dem Agenten verborgen.** Ein Durchlauf kann das Eval-Verzeichnis nicht lesen, daher kann Claude die Eingabeaufforderung des Falls, seine Grader oder Nachbar-Fälle nicht sehen.
* **Keine Netzwerk-Sandbox außerhalb von Shell-Befehlen.** Shell-Befehle, die Sie gewähren, werden unter den Sandbox-Regeln des Netzwerks ausgeführt. Ein `WebFetch(domain:…)` Grant erreicht diese Domain direkt, und die eigenen Hooks des Plugins und alle echten MCP-Server, die Sie starten, können jeden Host erreichen.

<h2 id="eval-suite-reference">
  Eval-Suite-Referenz
</h2>

Alles, was eine Eval-Suite enthalten kann, befindet sich unter dem Eval-Verzeichnis des Plugins, `evals/`, es sei denn, Sie [haben ein anderes konfiguriert](#use-a-different-eval-directory). Dieser Baum zeigt jede Datei, die `claude plugin eval` dort liest oder schreibt; nur `prompt.md` oder `case.yaml` ist erforderlich, damit ein Fall existiert:

```text theme={null}
evals/
├── <case>/                        # one directory per case; nest under a non-case directory to group
│   ├── prompt.md                  # frontmatter: case and run fields; body: the prompt
│   ├── case.yaml                  # optional: context.* fields, or the whole case in one file
│   ├── graders/
│   │   └── <name>.md              # one grader per file; frontmatter: type and options; body: rubric
│   ├── mocks/                     # optional: mocks for this case only, same layout as below
│   └── <fixtures, scripts, transcripts referenced by case.yaml>
├── mocks/                         # optional: suite-wide MCP mocks
│   ├── <server>/
│   │   ├── <tool>.md              # one mocked tool; body: the tool result
│   │   ├── _server.md             # optional: one agent that answers several tools
│   │   ├── _tools.json            # optional: saved tools/list response for real descriptions and schemas
│   │   └── fixtures/              # files inserted with {{file:fixtures/...}}
│   └── .replay/<server>/          # adopted agent-mock recordings, answered without a model call
└── results/<timestamp>/           # written by each run; add results/ to .gitignore
    ├── aggregate-result.json
    ├── report.html
    └── mock-recordings/           # agent-mock answers from clean runs, with ADOPT.txt
```

<h3 id="prompt-md-fields">
  prompt.md Frontmatter
</h3>

`prompt.md` Frontmatter akzeptiert diese Felder. Ein unbekannter Schlüssel ist ein Fehler:

| Feld                   | Standard                          | Zweck                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :--------------------- | :-------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `schema_version`       | `"1.1"`, für Sie gesetzt          | Fallformat-Version. Fälle, die als `prompt.md` geschrieben sind, erhalten sie automatisch, daher setzen Sie sie selten                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `name`                 | Der Verzeichnisname               | Fallname. `--case` Globs passen ihn an und der Bericht schlüsselt ihn auf                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `description`          |                                   | Für Menschen. Nicht zur Laufzeit verwendet                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `tags`                 | `[]`                              | Labels für `--tag` Filterung. Ein Fall wird ausgeführt, wenn einer seiner Tags passt                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `plugins`              | Das nächste einschließende Plugin | Plugin-Verzeichnisse unter Test, relativ zum Fallverzeichnis. Setzen Sie `plugins: ["../.."]`, wenn die Auto-Erkennung Ihr Plugin nicht findet; siehe [Das Plugin wurde nicht geladen](#the-baseline-arm-shows-no-plugin-or-delta-is-zero)                                                                                                                                                                                                                                                                                                                   |
| `runs`                 | `3`                               | Durchläufe pro Arm, 1 bis 50. `--runs` überschreibt es                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `expected_outcome`     |                                   | Für Menschen. Nicht zur Laufzeit verwendet                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `model`                | Der Standard der Kind-Sitzung     | Modell für den Agent unter Test. `--model` überschreibt es                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `max_turns`            | `10`                              | Zug-Obergrenze, bis zu 200. Sie zu treffen wird als Durchlauf-Fehler aufgezeichnet und senkt normalerweise den Score, daher setzen Sie ihn großzügig                                                                                                                                                                                                                                                                                                                                                                                                         |
| `timeout_seconds`      | `300`                             | Wall-Clock-Obergrenze pro Durchlauf, bis zu 3600                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `allowed_tools`        | `[]`                              | Tools, die der Fall möchte, wie `[Read, Glob, Grep, Skill]`. Schreibgeschützte Tools werden gewährt, wenn sie hier aufgelistet sind; für alles andere, siehe [Gewähren Sie Tools](#grant-tools)                                                                                                                                                                                                                                                                                                                                                              |
| `append_system_prompt` |                                   | Text, der an die System-Eingabeaufforderung der Kind-Sitzung angehängt wird                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `env`                  | `{}`                              | Zusätzliche Umgebungsvariablen für die Kind-Sitzung. Schlüssel müssen `EVAL_[A-Z0-9_]*` entsprechen; jeder andere Schlüssel lässt den Durchlauf fehlschlagen. Der Durchlauf erbt nur eine Allowlist aus Ihrer Shell: Grundlagen wie `PATH` und Locale, Proxy- und Zertifikateinstellungen, die Variablen, die Ihren Modell-Provider auswählen und authentifizieren, die meisten `ANTHROPIC_*` und `CLAUDE_CODE_*` Konfiguration und `EVAL_*`. Um dem Plugin etwas anderes zu geben, wie eine Toolchain-Einstellung, exportieren Sie es als `EVAL_*` Variable |

<h3 id="case-yaml-fields">
  case.yaml Felder
</h3>

`case.yaml` ist eine Alternative oder Ergänzung zu `prompt.md`: Sie beschreibt einen Fall in YAML und fügt die Felder hinzu, die auf andere Dateien zeigen. Es benötigt `schema_version: "1.1"` und `name`. Die `prompt.md` Felder `description`, `tags`, `plugins`, `runs` und `expected_outcome` gehen auf die oberste Ebene; `model`, `max_turns`, `timeout_seconds`, `allowed_tools`, `append_system_prompt` und `env` gehen unter `execution:`. Wenn beide Dateien existieren, überschreibt `prompt.md` Frontmatter die passenden `case.yaml` Felder, der `prompt.md` Body ist die Eingabeaufforderung und `graders/*.md` werden nach allen in `case.yaml` aufgelisteten Gradern hinzugefügt.

Diese Felder existieren nur in `case.yaml`:

| Feld                      | Zweck                                                                                                                                                                                                                                         |
| :------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `context.scaffold_script` | Ein Bash-Skript im Fallverzeichnis, das im leeren Arbeitsbereich vor Claude startet, um Fixture-Dateien oder ein Git-Repository zu erstellen. Es wird nur ausgeführt, wenn Sie [`--scaffold`](#add-setup-or-history-with-case-yaml) übergeben |
| `context.history_file`    | Ein `.jsonl` Transkript im Fallverzeichnis zum Fortsetzen. Die Eingabeaufforderung des Falls wird zum nächsten Benutzer-Turn                                                                                                                  |
| `context.add_dirs`        | Verzeichnisse innerhalb des Fallverzeichnisses, die Claude während des Durchlaufs lesen darf, schreibgeschützt gewährt                                                                                                                        |
| `execution.prompt`        | Die Eingabeaufforderung, wenn Sie den ganzen Fall in `case.yaml` behalten und `prompt.md` weglassen                                                                                                                                           |
| `graders`                 | Eine Liste von Gradern, jeder mit einem `name` plus die gleichen Schlüssel, die eine `graders/*.md` Datei in Frontmatter nimmt. Für `llm` Grader, legen Sie die Rubrik in `criteria`                                                          |

<h3 id="grader-frontmatter">
  Grader Frontmatter
</h3>

Jede Grader-Datei unter `graders/` nimmt diese Schlüssel in Frontmatter, plus die Optionen für seinen Typ. Der Name des Graders ist der Dateiname ohne `.md`:

| Schlüssel | Standard      | Zweck                                                                                                                                                                                                                                |
| :-------- | :------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`    | erforderlich  | Einer der [Grader-Typen](#grader-types)                                                                                                                                                                                              |
| `weight`  | `1`           | Relatives Gewicht im Score des Durchlaufs. Jede positive Zahl                                                                                                                                                                        |
| `arm`     | nicht gesetzt | `with-only` schließt den Grader aus der Bewertung in einem [Zwei-Arm-Durchlauf](#compare-against-a-no-plugin-baseline) aus; `both` erzwingt, dass ein Grader, den Claude Code sonst ausschließen würde, in beiden Arms bewertet wird |

<h4 id="what-a-grader-can-look-at">
  Was ein Grader ansehen kann
</h4>

`regex` Grader nehmen ein `target` und `llm` Grader nehmen einen `focus`. Beide akzeptieren die gleichen Werte:

| Wert                             | Was der Grader sieht                                                                                                                                                                                                                                                                                                                                                                      |
| :------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `last_message`                   | Claude's endgültige Antwort-Text. Dies ist der Standard                                                                                                                                                                                                                                                                                                                                   |
| `trace`                          | Die ganze Sitzung als JSON, eine Nachricht pro Zeile. Ein `regex` Grader sieht jede Nachricht; ein `llm` Judge sieht die ersten 12 und die letzten 12. Anführungszeichen und Zeilenumbrüche darin sind JSON-escaped, daher passt ein Regex `\"` anstelle von `"`                                                                                                                          |
| `files`                          | Die Liste der Pfade, die Claude während des Durchlaufs erstellt hat, einer pro Zeile. Nicht ihre Inhalte, und nicht Dateien, die ein Scaffold erstellt hat oder die Claude nur modifiziert hat                                                                                                                                                                                            |
| `{ source: file, path: <path> }` | Der Inhalt einer Datei im Arbeitsbereich nach dem Durchlauf. Verwenden Sie dies, um zu bewerten, was das Plugin produziert hat. Eine PNG-, JPEG-, GIF- oder WebP-Datei wird einem `llm` Judge als Bild angezeigt. Ein `llm` Judge weigert sich, andere Binärdateien wie `.pptx` oder PDF zu lesen; rendern Sie sie zu einem Bild oder schreiben Sie sie als Text aus und bewerten Sie das |
| `mock_calls`                     | Jeder Aufruf, den Claude an ein [gemocktes MCP-Tool](#mock-mcp-servers) machte, mit seiner Eingabe und der Antwort des Mocks                                                                                                                                                                                                                                                              |

<h4 id="grader-types">
  Grader-Typen
</h4>

Jeder Grader-Typ unten listet seine Optionen und wann er bestanden wird:

| Typ           | Optionen                              | Wird bestanden, wenn                                                                                                                                                                                                                                                                 |
| :------------ | :------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `regex`       | `pattern`, `flags`, `match`, `target` | Das JavaScript Regex `pattern` wird im Ziel gefunden. Setzen Sie `match: not_contains`, um Abwesenheit zu erfordern, oder `match: "count:N"`, um genau N Treffer zu erfordern. Legen Sie Groß-/Kleinschreibung-Unempfindlichkeit in `flags: i`; Inline `(?i)` wird nicht unterstützt |
| `tool_used`   | `tool`, `input_match`, `min`, `max`   | Die Anzahl der Aufrufe an `tool`, deren JSON-kodierte Eingabe das optionale `input_match` Regex passt, liegt zwischen `min`, Standard 1, und `max`, Standard unbegrenzt. Um zu bestätigen, dass ein Tool nie aufgerufen wurde, setzen Sie beide `min: 0` und `max: 0`                |
| `tool_order`  | `before`, `after`                     | Beide Tools wurden aufgerufen und der erste passende `before` Aufruf geht dem ersten passenden `after` Aufruf voraus. Jeder ist ein Tool-Name oder `{ tool, input_match }`                                                                                                           |
| `file_exists` | `path`, `exists`                      | Eine Datei, die Claude erstellt hat, passt zum `path` Glob, oder keine mit `exists: false`. Nur Dateien, die während des Durchlaufs erstellt wurden, zählen                                                                                                                          |
| `llm`         | `criteria`, `focus`                   | Ein Judge-Modell stimmt PASS auf die Rubrik in mindestens zwei von drei Stimmen ab. Im `.md` Layout ist der Datei-Body die Kriterien                                                                                                                                                 |
| `baseline`    | `baseline_file`, `criteria`           | Ein Judge findet, dass der Durchlauf die Kriterien mindestens so gut erfüllt wie das Referenz-Transkript bei `baseline_file`, ein `.jsonl` im Fallverzeichnis                                                                                                                        |

<h3 id="mock-files">
  Mock-Dateien
</h3>

Eine `<tool>.md` Datei unter `mocks/<server>/` beantwortet ein Tool. Sein Body ist das Tool-Ergebnis, mit `{{input.<field>}}` und `{{file:fixtures/<name>}}` Substitutionen. Sein Frontmatter akzeptiert diese Schlüssel:

| Schlüssel    | Standard      | Zweck                                                                                                                                                                                                                                                                                                              |
| :----------- | :------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`       | `fixed`       | `fixed` gibt den Body wie geschrieben zurück. `agent` behandelt den Body als Anweisungen für ein kleines Modell, das den Server für den Durchlauf spielt und frühere Aufrufe als Geschichte sieht                                                                                                                  |
| `expect`     | nicht gesetzt | Eine Karte von gepunkteten Eingabe-Pfaden zu einem Typ-Namen wie `string`, `number`, `boolean`, `array` oder `object`, ein `/regex/`, ein Literal oder eine Liste erlaubter Literale. Ein Aufruf, der ihn verletzt, bricht den Durchlauf mit Score 0 ab und wird als `aborted` mit Server, Tool und Grund gemeldet |
| `error`      | `false`       | `fixed` nur. Geben Sie den Body als Tool-Fehler zurück                                                                                                                                                                                                                                                             |
| `abort_when` | nicht gesetzt | `agent` nur. Prosa, die die einzigen Bedingungen auflistet, unter denen der Agent den Durchlauf abbrechen darf                                                                                                                                                                                                     |

Zwei optionale Dateien sitzen neben den Tool-Dateien in einem Server-Verzeichnis:

* **`_server.md`**: ein einzelner `type: agent` Mock, der mehrere Tools beantwortet, aufgelistet in seinem `tools:` Frontmatter-Schlüssel. Ein `<tool>.md` für das gleiche Tool hat Vorrang. Legen Sie einen `expect:` Guard auf das einzelne `<tool>.md`, nicht hier |
* **`_tools.json`**: eine gespeicherte `tools/list` Antwort vom echten Server, daher tragen gemockte Tools ihre echten Beschreibungen und Eingabe-Schemas anstelle eines permissiven Platzhalters |

Ein Falls eigenes `mocks/` Verzeichnis verwendet das gleiche Layout und überschreibt die Suite's Mock-Datei für Datei.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

Dies sind die Probleme, auf die Autoren am häufigsten stoßen, sortiert nach dem, was Sie sehen.

<h3 id="plugin-eval-is-currently-in-early-access">
  „plugin eval is currently in early access"
</h3>

Ihr Build ist älter als die allgemeine Verfügbarkeit des Befehls. Führen Sie `claude update` aus und führen Sie den Befehl dann erneut in einer neuen Sitzung aus.

<h3 id="plugin-eval-is-currently-unavailable">
  „plugin eval is currently unavailable"
</h3>

Anthropic hat den Befehl serverseitig deaktiviert. Nichts auf Ihrem Computer schaltet ihn wieder ein; führen Sie `claude update` aus und versuchen Sie es später in einer neuen Sitzung erneut.

<h3 id="is-not-a-trusted-plugin-directory-and-this-run-cannot-stop-to-ask-you-about-it">
  „is not a trusted plugin directory, and this run cannot stop to ask you about it"
</h3>

Dies ist der erste Durchlauf gegen ein Verzeichnis, dem Claude Code noch nicht vertraut, und es kann Sie nicht fragen, da stdin oder stdout kein Terminal ist, Sie `--json` übergeben haben oder die Umgebungsvariable `CI` auf einen wahren Wert wie `true` gesetzt ist. Führen Sie `claude plugin eval <dir>` einmal in einem Terminal aus und beantworten Sie die Eingabeaufforderung, oder übergeben Sie `--trust-plugin`, wenn Sie dem Code und der Suite des Plugins vertrauen. Siehe [What a run can access](#security).

<h3 id="no-eval-cases-found">
  „No eval cases found"
</h3>

Es existiert kein `<case>/prompt.md` oder `<case>/case.yaml` unter dem geltenden Eval-Verzeichnis, oder Ihre `--case`- und `--tag`-Filter haben keinen Fall gefunden. Führen Sie den Befehl aus dem Plugin-Root aus, oder führen Sie `claude plugin eval init` aus, um eine Suite zu erstellen.

<h3 id="the-baseline-arm-shows-no-plugin-or-delta-is-zero">
  Die Baseline-Arm zeigt kein Plugin, oder Delta ist null
</h3>

Wenn die Zusammenfassung keine `W/OUT`-Spalte hat oder der Fall mit „ablation requested but no plugin resolved" fehlschlägt, wurde kein Plugin für den Fall gefunden. Fügen Sie `plugins: ["../.."]` zum Fall hinzu und geben Sie den Pfad vom Fall-Verzeichnis zum Plugin-Verzeichnis an.

Wenn das Plugin geladen wurde und `Δ` immer noch nahe bei null liegt, während Ihr `tool_used: Skill`-Grader fehlschlägt, ist das normalerweise ein echtes Ergebnis, was bedeutet, dass die `description` des Skills nicht auf die Formulierung der Eingabeaufforderung anspricht. Passen Sie die Beschreibung an und führen Sie die gleiche Suite erneut aus.

<h3 id="agent-type-’-’-not-found-for-one-of-your-plugin’s-agents">
  „Agent type '...' not found" für einen der Agenten Ihres Plugins
</h3>

Standardmäßig wird jeder Fall sowohl mit Ihrem Plugin als auch ohne es ausgeführt, und die Durchläufe ohne es sind die [no-plugin baseline](#the-no-plugin-baseline). Wenn Claude einen der Agenten Ihres Plugins in einem Baseline-Durchlauf versendet, schlägt der Agent-Tool-Aufruf mit `Agent type '<plugin>:<agent-name>' not found. Available agents: ...` fehl. Die Liste benennt nur Agenten, die ohne das Plugin existieren, wie z. B. die [built-in subagents](/docs/de/sub-agents#built-in-subagents).

Der Fehler ist zu erwarten, da `Δ` Ihre Plugin-Durchläufe gegen die Baseline vergleicht. Im JSON-Ergebnis befinden sich die Baseline-Durchläufe unter `cases[].arms.without`.

Bei Durchläufen mit Ihrem geladenen Plugin kann ein Fall, der `Agent` in `allowed_tools` auflistet, einen der Agenten Ihres Plugins anhand seines namensgebundenen Namens versendet, wie z. B. `my-plugin:code-reviewer` für den `code-reviewer`-Agent in einem Plugin namens `my-plugin`. Um die Baseline-Durchläufe zu überspringen, übergeben Sie `--ablation none`.

<h3 id="everything-scores-zero-although-the-right-files-were-produced">
  Alles bewertet null, obwohl die richtigen Dateien erstellt wurden
</h3>

Ihre Grader zielen auf `files`, die Liste der erstellten Pfade, wenn Sie die Inhalte der Datei gemeint haben. Verwenden Sie `{ source: file, path: <path> }` als `target` oder `focus`. Separat zählt `file_exists` nur Dateien, die während des Durchlaufs erstellt wurden, daher ist eine Datei, die das Gerüst erstellt hat oder die Claude nur bearbeitet hat, für sie unsichtbar; bewerten Sie ihren Inhalt, oder verwenden Sie `tool_used` auf `Edit`.

<h3 id="a-regex-over-the-trace-doesn’t-match-text-i-can-see">
  Ein Regex über die Trace stimmt nicht mit Text überein, den ich sehen kann
</h3>

* **Falsches Ziel**: Das Standard-`target` ist `last_message`, nicht die Trace.
* **JSON-Escaping**: Wenn Sie `trace` als Ziel verwenden, ist es JSON pro Zeile, daher erscheinen Anführungszeichen als `\"`.
* **Regex-Syntax**: Regexes verwenden JavaScript-Syntax, daher setzen Sie `i` in `flags`, anstatt `(?i)` zu schreiben.

<h3 id="tools-are-denied-mcp-tools-are-missing-or-bash-won’t-run">
  Tools werden verweigert, MCP-Tools fehlen, oder Bash wird nicht ausgeführt
</h3>

Alles über die schreibgeschützte Menge hinaus benötigt Ihre Genehmigung, wie z. B. `--allow-tools Bash Write`. Ihre persönlichen MCP-Server werden in einem Durchlauf nie geladen. Die eigenen Server des Plugins starten nicht, es sei denn, Sie [aktivieren sie](#mock-mcp-servers), und ihre Tools benötigen dann auch eine `--allow-tools "mcp__plugin_<plugin>_<server>__*"`-Genehmigung; ein simuliertes Tool benötigt keine.

<h3 id="the-run-exits-1-but-the-results-look-fine">
  Der Durchlauf beendet sich mit 1, aber die Ergebnisse sehen gut aus
</h3>

Das Standard-`--threshold` ist 1,0, daher beendet sich der Befehl mit 1, wenn ein Fall unter perfekt bewertet wird. Legen Sie einen Schwellenwert fest, der Ihrem Standard entspricht. Exit 1 deckt auch eine Fall-Datei ab, die nicht geladen werden konnte, was auf stderr über der Tabelle gemeldet wird.

<h3 id="json-output-path-must-end-in-json">
  „--json output path must end in .json"
</h3>

Sie haben das Ziel nach `--json` eingegeben, daher wurde es als Ausgabepfad gelesen. Geben Sie das Ziel zuerst an, wie in `claude plugin eval . --json`, oder geben Sie `--json` einen expliziten `.json`-Pfad.

<h3 id="a-grader-shows-passed-false-under-a-run-that-scored-1-0">
  Ein Grader zeigt passed: false unter einem Durchlauf, der 1,0 bewertet
</h3>

Dieser Grader ist absichtlich von der Bewertung in einem Zwei-Arm-Durchlauf ausgeschlossen, und sein `scored`-Feld ist `false`. Siehe [Score against the no-plugin baseline](#compare-against-a-no-plugin-baseline).

<h3 id="runs-fail-with-a-usage-limit-or-rate-limit-error-partway-through">
  Durchläufe schlagen mit einem Nutzungslimit- oder Rate-Limit-Fehler in der Mitte fehl
</h3>

Wenn Ihr Konto während der Ausführung einer Suite das Nutzungslimit des Plans oder ein API-Rate-Limit erreicht, endet jeder spätere Durchlauf mit diesem Fehler, wird auf das bewertet, was er produziert hat, und bewertet normalerweise 0. Die Suite wird trotzdem beendet und ist nicht als `partial` gekennzeichnet, daher kann das Ergebnis wie eine Regression aussehen. Überprüfen Sie die `NOTES`-Spalte oder `cases[].arms.with[].error` im JSON auf die Limit-Nachricht, bevor Sie den Bewertungen vertrauen, und führen Sie dann erneut aus, nachdem das Limit zurückgesetzt wurde, mit `--runs 1` oder einem `--case`-Filter, wenn Sie darunter bleiben müssen.

<h3 id="runs-time-out-or-hit-the-turn-cap">
  Durchläufe überschreiten das Zeitlimit oder erreichen die Obergrenze für Durchläufe
</h3>

Die Standardwerte sind 10 Durchläufe und 300 Sekunden. Erhöhen Sie `max_turns` und `timeout_seconds` im Fall für Aufgaben, die mehr benötigen, und verwenden Sie `--max-cost-usd` als Kostendeckel anstelle von engen Pro-Durchlauf-Limits.

<h2 id="see-also">
  Siehe auch
</h2>

* [Plugin erstellen](/docs/de/plugins/create): bauen Sie das Plugin, das Sie testen, und laden Sie es mit `--plugin-dir` während der Entwicklung
* [Plugin-Befehle Referenz](/docs/de/plugins/cli-reference#plugin-eval): die `plugin eval` und `plugin eval init` Befehlseinträge. Der Manifest's [`experimental.evals`](/docs/de/plugins/manifest-reference#fields) Schlüssel befindet sich in der Manifest-Referenz
* [Skills](/docs/de/skills): wie die `description` eines Skills entscheidet, wann Claude ihn aufruft, was das ist, was ein Fall, der überprüft, ob der Skill auslöst, misst
* [Sandboxing](/docs/de/sandboxing): die OS-Level Sandbox, die angewendet wird, wenn Sie Bash einem Durchlauf gewähren
* [Plugin veröffentlichen](/docs/de/plugins/publish): veröffentlichen Sie das Plugin, sobald seine Suite bestanden wird
* [Plugin-Kosten und -Nutzung messen](/docs/de/plugins/measure): was das Plugin zu jedem Sitzungskontext hinzufügt und ob Personen es noch verwenden
