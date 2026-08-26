> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude auf ein Ziel hinarbeiten lassen

> Legen Sie mit /goal eine Abschlussbedingung fest und Claude arbeitet über mehrere Turns hinweg daran, bis die Bedingung erfüllt ist.

<Note>
  `/goal` erfordert Claude Code v2.1.139 oder später.
</Note>

Der Befehl `/goal` legt eine Abschlussbedingung fest und Claude arbeitet darauf hin, ohne dass Sie jeden Schritt eingeben müssen. Nach jedem Turn überprüft ein kleines schnelles Modell, ob die Bedingung erfüllt ist. Falls nicht, startet Claude einen weiteren Turn, anstatt die Kontrolle an Sie zurückzugeben. Das Ziel wird automatisch gelöscht, sobald die Bedingung erfüllt ist.

Verwenden Sie ein Ziel für umfangreiche Arbeiten mit einem überprüfbaren Endzustand:

* Migration eines Moduls zu einer neuen API, bis jede Aufrufstelle kompiliert und Tests bestanden sind
* Implementierung eines Design-Dokuments, bis alle Akzeptanzkriterien erfüllt sind
* Aufteilung einer großen Datei in fokussierte Module, bis jedes unter einem Größenlimit liegt
* Durcharbeitung einer gekennzeichneten Issue-Warteschlange, bis die Warteschlange leer ist

<h2 id="compare-ways-to-keep-a-session-running">
  Vergleich zu anderen autonomen Workflows
</h2>

Drei Ansätze halten die aktuelle Sitzung zwischen Eingaben aktiv. Wählen Sie basierend darauf, was den nächsten Turn starten sollte:

| Ansatz                                                              | Nächster Turn startet wenn     | Stoppt wenn                                                           |
| :------------------------------------------------------------------ | :----------------------------- | :-------------------------------------------------------------------- |
| `/goal`                                                             | Der vorherige Turn beendet ist | Ein Modell bestätigt, dass die Bedingung erfüllt ist                  |
| [`/loop`](/docs/de/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) | Ein Zeitintervall verstreicht  | Sie stoppen es, oder Claude entscheidet, dass die Arbeit erledigt ist |
| [Stop Hook](/docs/de/hooks-guide#prompt-based-hooks)                     | Der vorherige Turn beendet ist | Ihr eigenes Skript oder Ihre Eingabe entscheidet                      |

`/goal` und ein Stop Hook werden beide nach jedem Turn ausgelöst. `/goal` ist eine Sitzungs-Verknüpfung: Sie geben eine Bedingung ein und sie ist nur für die aktuelle Sitzung aktiv. Ein Stop Hook befindet sich in Ihrer Einstellungsdatei, gilt für jede Sitzung in seinem Bereich und kann ein Skript für deterministische Überprüfungen oder eine Eingabe für modellbewertete Überprüfungen ausführen.

[Auto-Modus](/docs/de/auto-mode-config) genehmigt von selbst Tool-Aufrufe innerhalb eines einzelnen Turns, startet aber keinen neuen. Claude stoppt, wenn es die Arbeit als erledigt einstuft. `/goal` fügt einen separaten Evaluator hinzu, der Ihre Bedingung nach jedem Turn überprüft, sodass die Fertigstellung von einem frischen Modell entschieden wird, anstatt von dem, das die Arbeit ausführt. Die beiden sind komplementär: Auto-Modus entfernt Pro-Tool-Eingaben und `/goal` entfernt Pro-Turn-Eingaben.

<Tip>
  Die obigen Ansätze halten die aktuelle Sitzung aktiv. Sie können auch Arbeiten planen, die unabhängig von einer offenen Sitzung ausgeführt werden, z. B. nächtliche Tests oder morgendliche Triage. Siehe [Planungsoptionen](/docs/de/scheduled-tasks#compare-scheduling-options) für Cloud-Routinen und Desktop-geplante Aufgaben.
</Tip>

<h2 id="use-/goal">
  `/goal` verwenden
</h2>

Pro Sitzung kann ein Ziel aktiv sein. Der gleiche Befehl legt es fest, überprüft es und löscht es je nach Argument.

<h3 id="set-a-goal">
  Ziel festlegen
</h3>

Führen Sie `/goal` gefolgt von der Bedingung aus, die Sie erfüllt haben möchten. Wenn bereits ein Ziel aktiv ist, ersetzt das neue es.

```text theme={null}
/goal all tests in test/auth pass and the lint step is clean
```

Das Festlegen eines Ziels startet sofort einen Turn mit der Bedingung selbst als Direktive. Sie müssen keine separate Eingabe senden. Während das Ziel aktiv ist, zeigt ein `◎ /goal active` Indikator, wie lange das Ziel bereits läuft.

Ein Ziel ändert keine Berechtigungen. Im Standard-Berechtigungsmodus fragt Claude immer noch vor Tool-Aufrufen, die Ihre Einstellungen nicht bereits zulassen, wie z. B. der oben genannte Test-Befehl. Um Goal-Turns unbeaufsichtigt ausführen zu lassen, kombinieren Sie `/goal` mit [Auto-Modus](/docs/de/auto-mode-config).

Nach jedem Turn gibt der Evaluator einen kurzen Grund zurück, der erklärt, warum die Bedingung erfüllt ist oder nicht. Der neueste Grund wird in der Statusansicht und im Transkript angezeigt, damit Sie sehen können, worauf Claude als nächstes hinarbeitet.

<Note>
  Ein Ziel läuft weiter, bis die Bedingung erfüllt ist oder Sie `/goal clear` ausführen. Führen Sie `/goal` ohne Argument aus, um die bisherigen Turns und Token zu sehen.
</Note>

<h3 id="write-an-effective-condition">
  Effektive Bedingung schreiben
</h3>

Der [Evaluator](#how-evaluation-works) beurteilt Ihre Bedingung anhand dessen, was Claude im Gespräch dargelegt hat. Er führt Befehle nicht aus oder liest Dateien unabhängig, daher schreiben Sie die Bedingung als etwas, das Claudes eigene Ausgabe demonstrieren kann. „Alle Tests in `test/auth` bestanden" funktioniert, weil Claude die Tests ausführt und das Ergebnis im Transkript für den Evaluator zum Lesen landet.

Eine Bedingung, die über viele Turns hinweg hält, hat normalerweise:

* **Einen messbaren Endzustand**: ein Testergebnis, ein Build-Exit-Code, eine Dateianzahl, eine leere Warteschlange
* **Eine angegebene Überprüfung**: wie Claude es beweisen sollte, z. B. „`npm test` beendet mit 0" oder „`git status` ist sauber"
* **Einschränkungen, die wichtig sind**: alles, das sich auf dem Weg dorthin nicht ändern darf, z. B. „keine andere Testdatei wird geändert"

Die Bedingung kann bis zu 4.000 Zeichen lang sein.

Um zu begrenzen, wie lange ein Ziel läuft, fügen Sie eine Turn- oder Zeitklausel in die Bedingung ein, z. B. `or stop after 20 turns`. Claude meldet den Fortschritt gegen diese Klausel jeden Turn und der Evaluator beurteilt sie aus dem Gespräch.

<h3 id="check-status">
  Status überprüfen
</h3>

Führen Sie `/goal` ohne Argumente aus, um den aktuellen Zustand zu sehen.

```text theme={null}
/goal
```

Wenn ein Ziel aktiv ist, zeigt der Status:

* Die Bedingung
* Wie lange es läuft
* Wie viele Turns evaluiert wurden
* Die aktuelle Token-Ausgabe
* Den neuesten Grund des Evaluators

Der Turn-Count und der neueste Grund werden nach der ersten Evaluierung angezeigt.

Wenn kein Ziel aktiv ist, aber eines früher in der Sitzung erreicht wurde, zeigt der Status die erreichte Bedingung zusammen mit ihrer Dauer, Turnanzahl und Token-Ausgabe.

<h3 id="clear-a-goal">
  Ziel löschen
</h3>

Führen Sie `/goal clear` aus, um ein aktives Ziel zu entfernen, bevor seine Bedingung erfüllt ist.

```text theme={null}
/goal clear
```

Claude gibt `Goal cleared:` gefolgt von der Bedingung aus, um zu bestätigen, oder `No goal set`, wenn nichts aktiv war.

`stop`, `off`, `reset`, `none` und `cancel` werden als Aliase für `clear` akzeptiert. Das Ausführen von `/clear` zum Starten eines neuen Gesprächs entfernt auch alle aktiven Ziele.

<h3 id="resume-with-an-active-goal">
  Mit aktivem Ziel fortfahren
</h3>

Ein Ziel, das noch aktiv war, als eine Sitzung endete, wird wiederhergestellt, wenn Sie diese Sitzung mit `--resume` oder `--continue` fortsetzen. Die Bedingung wird übernommen, aber die Turnanzahl, der Timer und die Token-Ausgabe-Baseline werden alle beim Fortsetzen zurückgesetzt. Ein Ziel, das bereits erreicht oder gelöscht wurde, wird nicht wiederhergestellt.

<h3 id="run-non-interactively">
  Nicht-interaktiv ausführen
</h3>

`/goal` funktioniert im [nicht-interaktiven Modus](/docs/de/headless), in der [Desktop-App](/docs/de/desktop) und über [Remote Control](/docs/de/remote-control). Das Festlegen eines Ziels mit `-p` führt die Schleife in einem einzigen Aufruf bis zur Fertigstellung aus:

```bash theme={null}
claude -p "/goal CHANGELOG.md has an entry for every PR merged this week"
```

Mit der Standard-Textausgabe wird nichts gedruckt, bis die Bedingung erfüllt ist, daher kann ein Ziel, das viele Turns läuft, stecken bleiben. Fügen Sie `--output-format stream-json --verbose` hinzu, um jede Nachricht auszugeben, während die Schleife läuft.

Unterbrechen Sie den Prozess mit Ctrl+C, um ein nicht-interaktives Ziel zu stoppen, bevor die Bedingung erfüllt ist.

<h2 id="how-evaluation-works">
  Wie Evaluierung funktioniert
</h2>

`/goal` ist ein Wrapper um einen Sitzungs-Bereich [prompt-basierten Stop Hook](/docs/de/hooks#prompt-based-hooks). Jedes Mal, wenn Claude einen Turn beendet, werden die Bedingung und das bisherige Gespräch an Ihr konfiguriertes [kleines schnelles Modell](/docs/de/model-config) gesendet, das standardmäßig Haiku ist. Das Modell gibt eine Ja-oder-Nein-Entscheidung und einen kurzen Grund zurück. Ein „Nein" teilt Claude mit, dass es weiterarbeiten soll und enthält den Grund als Anleitung für den nächsten Turn. Ein „Ja" löscht das Ziel und zeichnet einen erreichten Eintrag im Transkript auf.

Der Evaluator läuft auf dem Provider, für den Ihre Sitzung konfiguriert ist. Er ruft keine Tools auf, daher kann er nur beurteilen, was Claude bereits im Gespräch dargelegt hat.

<Note>
  Evaluierungs-Token werden auf dem kleinen schnellen Modell abgerechnet, das für Ihren Provider konfiguriert ist, und sind normalerweise vernachlässigbar im Vergleich zur Hauptausgabe.
</Note>

<h2 id="requirements">
  Anforderungen
</h2>

`/goal` läuft nur in Arbeitsbereichen, in denen Sie den Vertrauensdialog akzeptiert haben, da der Evaluator Teil des Hooks-Systems ist. `/goal` ist auch nicht verfügbar, wenn [`disableAllHooks`](/docs/de/hooks#disable-or-remove-hooks) auf einer beliebigen Einstellungsebene festgelegt ist oder wenn [`allowManagedHooksOnly`](/docs/de/settings#hook-configuration) in verwalteten Einstellungen festgelegt ist. In jedem Fall teilt Ihnen der Befehl mit, warum, anstatt stillschweigend nichts zu tun.

<h2 id="see-also">
  Siehe auch
</h2>

* [Eingabe wiederholt mit `/loop` ausführen](/docs/de/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop): auf einem Zeitintervall erneut ausführen, anstatt bis eine Bedingung erfüllt ist
* [Prompt-basierte Hooks](/docs/de/hooks-guide#prompt-based-hooks): schreiben Sie Ihren eigenen Stop Hook, wenn Sie benutzerdefinierte Evaluierungslogik benötigen
* [Auto-Modus](/docs/de/auto-mode-config): genehmigen Sie Tool-Aufrufe automatisch, damit jeder Ziel-Turn unbeaufsichtigt läuft
* [Planungsvergleich](/docs/de/scheduled-tasks#compare-scheduling-options): führen Sie Arbeiten unabhängig von einer offenen Sitzung nach einem Zeitplan aus
