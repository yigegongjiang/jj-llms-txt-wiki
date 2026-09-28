> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude auf ein Ziel hinarbeiten lassen

> Legen Sie mit /goal eine Abschlussbedingung fest und Claude arbeitet über mehrere Turns hinweg daran, bis die Bedingung erfüllt ist, ein Modell sie für unmöglich hält oder ein Fehler, den Sie beheben müssen, das Ziel löscht.

Der Befehl `/goal` legt eine Abschlussbedingung fest und Claude arbeitet darauf hin, ohne dass Sie jeden Schritt eingeben müssen. Nach jedem Turn überprüft ein kleines schnelles Modell, ob die Bedingung erfüllt ist. Falls das Modell urteilt, dass sie noch nicht erfüllt ist, startet Claude einen weiteren Turn, anstatt die Kontrolle an Sie zurückzugeben. Das Ziel wird automatisch gelöscht, sobald die Bedingung erfüllt ist, wenn das Modell urteilt, dass die Bedingung unmöglich zu erfüllen ist, oder wenn ein Turn bei [einem Fehler fehlschlägt, den Sie beheben müssen](#errors-you-have-to-fix-clear-the-goal).

Verwenden Sie ein Ziel für umfangreiche Arbeiten mit einem überprüfbaren Endzustand:

* Migration eines Moduls zu einer neuen API, bis jede Aufrufstelle kompiliert und Tests bestanden sind
* Implementierung eines Design-Dokuments, bis alle Akzeptanzkriterien erfüllt sind
* Aufteilung einer großen Datei in fokussierte Module, bis jedes unter einem Größenlimit liegt
* Durcharbeitung einer gekennzeichneten Issue-Warteschlange, bis die Warteschlange leer ist

<h2 id="compare-ways-to-keep-a-session-running">
  Vergleich zu anderen autonomen Workflows
</h2>

Drei Ansätze halten die aktuelle Sitzung zwischen Eingaben aktiv. Wählen Sie basierend darauf, was den nächsten Turn starten sollte:

| Ansatz                                                              | Nächster Turn startet wenn                                                                                                                                                                                                          | Stoppt wenn                                                                                                                                                                                                                                               |
| :------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/goal`                                                             | Der vorherige Turn beendet ist, oder in einer interaktiven Sitzung eine [Leerlauf-Überprüfung](#background-work-defers-evaluation) oder ein [automatischer Wiederholungsversuch](#other-errors-retry-or-pause-the-goal) fällig wird | Ein Modell bestätigt, dass die Bedingung erfüllt ist, oder beurteilt sie als unmöglich, oder ein Turn schlägt bei [einem Fehler fehl, den Sie beheben müssen](#errors-you-have-to-fix-clear-the-goal), oder Sie führen [`/goal clear`](#clear-a-goal) aus |
| [`/loop`](/docs/de/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) | Ein Zeitintervall verstreicht                                                                                                                                                                                                       | Sie stoppen es, oder Claude entscheidet, dass die Arbeit erledigt ist                                                                                                                                                                                     |
| [Stop Hook](/docs/de/hooks-guide#prompt-based-hooks)                     | Der vorherige Turn beendet ist                                                                                                                                                                                                      | Ihr eigenes Skript oder Ihre Eingabe entscheidet                                                                                                                                                                                                          |

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

Ein Ziel ändert Ihren Berechtigungsmodus nicht. Um Goal-Turns unbeaufsichtigt ausführen zu lassen, führen Sie `/goal` im [Auto-Modus](/docs/de/auto-mode-config) aus. Im [Manuellen Modus](/docs/de/permission-modes) fragt Claude immer noch vor Tool-Aufrufen, die Ihre Einstellungen nicht bereits zulassen, wie z. B. der oben genannte Test-Befehl.

Während das Ziel aktiv ist, zeigt das Transkript jeden Verdikt, das der Evaluator zurückgibt, und Sie können Ctrl+O drücken, um den Grund dahinter zu sehen. Die Statusansicht zeigt auch den neuesten Grund, damit Sie sehen können, worauf Claude als nächstes hinarbeitet.

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

Führen Sie `/goal clear` aus, um ein aktives Ziel zu entfernen, bevor es sich auflöst.

```text theme={null}
/goal clear
```

Claude gibt `Goal cleared:` gefolgt von der Bedingung aus, um zu bestätigen, oder `No goal set`, wenn nichts aktiv war.

`stop`, `off`, `reset`, `none` und `cancel` werden als Aliase für `clear` akzeptiert. Das Ausführen von `/clear` zum Starten eines neuen Gesprächs entfernt auch alle aktiven Ziele.

<h3 id="resume-with-an-active-goal">
  Mit aktivem Ziel fortfahren
</h3>

Wenn Sie eine Sitzung fortsetzen, stellt Claude Code ein Ziel wieder her, das noch aktiv war, als die Sitzung endete. Claude Code stellt es auf jeder Fortsetzungsroute wieder her: `--continue`, `--resume` mit einer Sitzungs-ID, einem Namen oder einem [Transkript-Dateipfad](/docs/de/sessions#resume-a-session), und der [Sitzungsauswahl](/docs/de/sessions#use-the-session-picker). Vor v2.1.239 stellte Claude Code das Ziel auf jeder Route außer der `claude --resume` Auswahl wieder her.

Claude Code übernimmt die Bedingung, setzt aber die Turnanzahl, den Timer und die Token-Ausgabe-Baseline zurück. Es stellt ein Ziel nicht wieder her, das bereits erreicht oder gelöscht wurde.

<h3 id="run-non-interactively">
  Nicht-interaktiv ausführen
</h3>

`/goal` funktioniert im [nicht-interaktiven Modus](/docs/de/headless), in der [Desktop-App](/docs/de/desktop) und über [Remote Control](/docs/de/remote-control). Das Festlegen eines Ziels mit `-p` führt die Schleife in einem einzigen Aufruf bis zur Fertigstellung aus:

```bash theme={null}
claude -p "/goal CHANGELOG.md has an entry for every PR merged this week"
```

Mit der Standard-Textausgabe wird nichts gedruckt, bis die Ausführung endet, daher kann ein Ziel, das viele Turns läuft, stecken bleiben. Fügen Sie `--output-format stream-json --verbose` hinzu, um jede Nachricht auszugeben, während die Schleife läuft.

Unterbrechen Sie den Prozess mit Ctrl+C, um ein nicht-interaktives Ziel zu stoppen, bevor es sich auflöst.

<h2 id="how-evaluation-works">
  Wie Evaluierung funktioniert
</h2>

`/goal` ist ein Wrapper um einen Sitzungs-Bereich [prompt-basierten Stop Hook](/docs/de/hooks#prompt-based-hooks). Jedes Mal, wenn Claude einen Turn beendet, sendet Claude Code die Bedingung und das bisherige Gespräch an Ihr konfiguriertes [kleines schnelles Modell](/docs/de/model-config), das standardmäßig Haiku auf der Claude API ist; bei einem Drittanbieter-Provider überprüfen Sie Ihre [Provider-Seite](/docs/de/third-party-integrations) für den Standard der Plattform. Das Modell gibt eines von drei Urteilen zurück, jeweils mit einem kurzen Grund:

* **Noch nicht erfüllt**: Claude arbeitet weiter und nimmt den Grund als Anleitung für den nächsten Turn.
* **Erfüllt**: Claude Code löscht das Ziel und zeichnet einen erreichten Eintrag im Transkript auf.
* **Unmöglich**: Der Evaluator hat beurteilt, dass die Bedingung niemals erfüllt werden kann. Claude Code löscht das Ziel und zeichnet einen fehlgeschlagenen Eintrag im Transkript zusammen mit dem Grund auf. Sie müssen es nicht selbst löschen.

Wenn Claude weiterhin den Evaluator beantwortet, ohne Fortschritt zu machen (keine Tool-Nutzung für mehrere Turns hintereinander), stoppt Claude Code die Schleife, gibt eine Warnung aus und gibt die Kontrolle an Sie zurück, wobei das Ziel noch gesetzt ist. Die Evaluierung wird nach Ihrer nächsten Eingabeaufforderung fortgesetzt. Der [Hooks-Leitfaden](/docs/de/hooks-guide#stop-hook-hits-the-block-cap) erklärt den zugrunde liegenden Mechanismus.

<h3 id="when-a-turn-fails">
  Wenn ein Turn fehlschlägt
</h3>

Wenn ein Turn fehlschlägt, löscht Claude Code das Ziel, wenn der Fehler einer ist, den Sie beheben müssen. Nach jedem anderen Fehler bleibt das Ziel gesetzt.

<h4 id="errors-you-have-to-fix-clear-the-goal">
  Fehler, die Sie beheben müssen, löschen das Ziel
</h4>

Wenn ein Turn bei einem Fehler fehlschlägt, der nicht gelöscht wird, bis Sie ihn beheben, löscht Claude Code das Ziel und gibt eine Warnung aus, die die Ursache benennt. Die Warnung beginnt mit `Goal cleared after an unrecoverable error` und endet mit `Run /goal again to continue`. Beheben Sie die Ursache, dann [setzen Sie das Ziel erneut](#set-a-goal) mit `/goal <condition>`. Vier Arten von Fehlern löschen das Ziel:

* Ein Authentifizierungsfehler, wenn Claude Code seine eigenen Anmeldedaten verwaltet. Wenn ein Host diese für Sie verwaltet, z. B. die Desktop-App, die VS Code-Erweiterung oder eine [Cloud-Sitzung](/docs/de/claude-code-on-the-web), behält Claude Code das Ziel aktiv, da der Host den Zugriff selbst wiederherstellt.
* Ein erschöpfter Guthabensaldo
* Ein Kontext-Overflow, den [Auto-Komprimierung](/docs/de/model-config#set-the-auto-compact-window) nicht löschen konnte
* Ein Modell, das nicht verfügbar ist

<h4 id="other-errors-retry-or-pause-the-goal">
  Andere Fehler wiederholen oder pausieren das Ziel
</h4>

Nach jedem anderen Fehler bleibt das Ziel gesetzt. In einer interaktiven Sitzung auf Claude Code v2.1.269 oder später gibt Claude Code auch eine Zeile aus, die die Ursache benennt, und wiederholt entweder automatisch oder wartet auf Sie:

* **Wiederholung**: Nach einem Fehler, der dazu neigt, sich selbst zu beheben, z. B. ein überladener Server oder eine unterbrochene Verbindung, zeigt eine Mitteilung, die mit `Goal still active` beginnt, die Wartezeit vor dem nächsten Versuch. Nach drei automatischen Wiederholungen wird das Ziel stattdessen pausiert.
* **Pause**: Nach einem Fehler, den eine Wiederholung nur wiederholen würde, z. B. ein API-Ratenlimit, ein claude.ai [Nutzungslimit](/docs/de/errors#youve-hit-your-session-limit) oder ein Hook, der den Turn beendet hat, zeigt eine Mitteilung, die mit `Goal paused` beginnt, die Ursache. Wenn die Sitzung [wartet, um automatisch fortzufahren, wenn ein Nutzungslimit zurückgesetzt wird](/docs/de/interactive-mode#wait-for-a-usage-limit-to-reset), setzt Claude die Arbeit zum Ziel dann fort.

Senden Sie jederzeit eine Nachricht, um den nächsten Turn sofort zu starten. Um automatische Wiederholungen auszuschalten, setzen Sie [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/de/env-vars) auf `0`, was auch [Überprüfungen](#background-work-defers-evaluation) ausschaltet.

<h3 id="background-work-defers-evaluation">
  Hintergrundarbeit verschiebt die Evaluierung
</h3>

Wenn ein Subagent oder ein Hintergrund-Shell-Befehl noch läuft, wenn ein Turn endet, überspringt Claude Code die Evaluierung für diesen Turn. Es evaluiert am Ende des nächsten Turns, der ohne laufende Hintergrundarbeit endet. Wenn die Hintergrundarbeit endet, liefert Claude Code das Ergebnis an Claude als neuen Turn, sodass Sie nicht auffordern müssen.

Sobald die Hintergrundarbeit das Ziel 30 Minuten lang wartet, ist eine Überprüfung fällig. Bei der Überprüfung listet Claude Code die laufenden Aufgaben auf und fordert Claude auf, ihre Ausgabe zu lesen, zu warten, wenn sie Fortschritt machen, und alle zu beheben oder zu stoppen, die steckengeblieben sind. Nach der ersten Überprüfung wartet Claude Code zweimal so lange vor jeder späteren Überprüfung, bis zu viermal das erste Intervall: mit dem Standard, 1 Stunde nach der ersten Überprüfung, dann alle 2 Stunden. Claude Code liefert eine fällige Überprüfung, die erste eingeschlossen, auf eine von zwei Arten:

* **Wenn ein Turn endet**: Claude Code liefert die Überprüfung am Ende des nächsten Turns, der mit der noch laufenden Arbeit endet. In einer nicht-interaktiven Sitzung, z. B. einer, die mit `-p` gestartet wurde, ist dies die einzige Möglichkeit, wie Claude Code Überprüfungen liefert.
* **Während die Sitzung untätig ist**: In einer interaktiven Sitzung startet Claude Code auch einen Turn selbst, um die Überprüfung zu liefern, anstatt auf Ihre nächste Eingabeaufforderung zu warten. Wenn die Hintergrundarbeit ohne Ergebnis gestoppt hat, fordert Claude Code Claude auf, zum Ziel zu arbeiten. Claude Code startet höchstens drei untätige Überprüfungen pro Ziel zwischen Ihren Eingabeaufforderungen. Bei der dritten untätigen Überprüfung sagt Claude Code, dass untätige Überprüfungen pausiert sind, bis Sie eine weitere Eingabeaufforderung senden. Vor v2.1.246 waren untätige Überprüfungen unbegrenzt. Untätige Überprüfungen erfordern Claude Code v2.1.236 oder später.

Vor v2.1.239 wurden nur untätige Überprüfungen auf diese Weise zurückgefahren; eine Überprüfung, die am Ende eines Turns geliefert wurde, wiederholte sich beim ersten Intervall.

Um das erste Intervall zu ändern, setzen Sie [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/de/env-vars). Claude Code verwendet Ihren Wert anstelle des 30-Minuten-Intervalls und skaliert die späteren Intervalle damit. Setzen Sie es auf `0`, um Überprüfungen und [automatische Wiederholungen](#other-errors-retry-or-pause-the-goal) auszuschalten.

Überprüfungen erfordern Claude Code v2.1.234 oder später.

<h3 id="evaluation-model-and-cost">
  Evaluierungsmodell und Kosten
</h3>

Um auf einem anderen Modell zu evaluieren, setzen Sie [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/de/model-config#environment-variables).

<Warning>
  Claude Code liest `ANTHROPIC_DEFAULT_HAIKU_MODEL` überall dort, wo es das kleine schnelle Modell verwendet, nicht nur für die `/goal`-Evaluierung. Wenn Sie es setzen, löst Claude Code auch den [`haiku`-Alias](/docs/de/model-config#model-aliases) zu diesem Modell auf und führt [Hintergrundfunktionalität](/docs/de/costs#background-token-usage), wie Gesprächszusammenfassung, darauf aus.
</Warning>

Der Evaluator läuft auf dem Provider, für den Ihre Sitzung konfiguriert ist. Er ruft keine Tools auf, daher kann er nur beurteilen, was Claude bereits im Gespräch dargelegt hat.

<Note>
  Evaluierungs-Token werden auf dem kleinen schnellen Modell abgerechnet, das für Ihren Provider konfiguriert ist, und sind normalerweise vernachlässigbar im Vergleich zur Hauptausgabe.
</Note>

<h2 id="requirements">
  Anforderungen
</h2>

Claude Code stellt `/goal` unter der gleichen [Arbeitsbereichs-Vertrauensregel wie Hooks in Einstellungsdateien](/docs/de/permissions#what-runs-before-you-trust-a-folder) zur Verfügung, da der Evaluator Teil des Hooks-Systems ist. `/goal` ist auch nicht verfügbar, wenn [`disableAllHooks`](/docs/de/hooks#disable-or-remove-hooks) nach Anwendung der Einstellungspriorität `true` ist, oder wenn [`allowManagedHooksOnly`](/docs/de/settings-reference#allowmanagedhooksonly) in verwalteten Einstellungen festgelegt ist. In jedem Fall teilt Ihnen der Befehl mit, warum, anstatt stillschweigend nichts zu tun.

<h2 id="see-also">
  Siehe auch
</h2>

* [Eingabe wiederholt mit `/loop` ausführen](/docs/de/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop): auf einem Zeitintervall erneut ausführen, anstatt bis eine Bedingung erfüllt ist
* [Prompt-basierte Hooks](/docs/de/hooks-guide#prompt-based-hooks): schreiben Sie Ihren eigenen Stop Hook, wenn Sie benutzerdefinierte Evaluierungslogik benötigen
* [Auto-Modus](/docs/de/auto-mode-config): genehmigen Sie Tool-Aufrufe automatisch, damit jeder Ziel-Turn unbeaufsichtigt läuft
* [Planungsvergleich](/docs/de/scheduled-tasks#compare-scheduling-options): führen Sie Arbeiten unabhängig von einer offenen Sitzung nach einem Zeitplan aus
