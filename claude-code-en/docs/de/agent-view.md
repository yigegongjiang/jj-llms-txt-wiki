> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mehrere Agenten mit der Agenten-Ansicht verwalten

> Versenden und verwalten Sie viele Claude Code-Sitzungen von einem Bildschirm aus. Die Agenten-Ansicht zeigt, was jede Sitzung tut und welche Ihre Eingabe benötigen.

Die Agenten-Ansicht, die mit `claude agents` geöffnet wird, ist ein Bildschirm für alle Ihre Hintergrund-Sitzungen: was läuft, was Ihre Eingabe benötigt und was erledigt ist. Versenden Sie neue Sitzungen, beobachten Sie ihren Status auf einen Blick, anstatt durch Transkripte zu scrollen, und greifen Sie nur ein, wenn eine Sitzung Sie benötigt. Jede Hintergrund-Sitzung ist eine vollständige Claude Code-Konversation, die ohne angeschlossenes Terminal weiterläuft, sodass Sie sie jederzeit öffnen, antworten und verlassen können.

<img src="https://mintcdn.com/claude-code/1B48Qz2Z9hac4SLG/images/agent-view-light.png?fit=max&auto=format&n=1B48Qz2Z9hac4SLG&q=85&s=7a186c96ed47d6700d084d77e786be65" className="dark:hidden" alt="Agenten-Ansicht in einem Terminal: Die Kopfzeile zeigt Claude Code v2.1.140, das Modell, das Arbeitsverzeichnis und eine Zusammenfassung der Anzahl. Sitzungen sind unter 'Benötigt Eingabe', 'Wird ausgeführt' und 'Abgeschlossen' gruppiert, mit einer Versand-Eingabe am unteren Rand und einer Fußzeile mit Tastaturhinweisen." width="1772" height="780" data-path="images/agent-view-light.png" />

<img src="https://mintcdn.com/claude-code/1B48Qz2Z9hac4SLG/images/agent-view-dark.png?fit=max&auto=format&n=1B48Qz2Z9hac4SLG&q=85&s=a5bed7434bae368faea3a8f023b52aa2" className="hidden dark:block" alt="Agenten-Ansicht in einem Terminal: Die Kopfzeile zeigt Claude Code v2.1.140, das Modell, das Arbeitsverzeichnis und eine Zusammenfassung der Anzahl. Sitzungen sind unter 'Benötigt Eingabe', 'Wird ausgeführt' und 'Abgeschlossen' gruppiert, mit einer Versand-Eingabe am unteren Rand und einer Fußzeile mit Tastaturhinweisen." width="1772" height="780" data-path="images/agent-view-dark.png" />

Verwenden Sie die Agenten-Ansicht, wenn Sie mehrere unabhängige Aufgaben haben, an denen Claude ohne Ihre ständige Überwachung arbeiten kann. Versenden Sie eine Fehlerbehebung, eine Pull-Request-Überprüfung und eine Untersuchung eines instabilen Tests als drei Zeilen, arbeiten Sie in einem anderen Fenster weiter und überprüfen Sie später, wenn eine Zeile anzeigt, dass sie Sie benötigt oder ein Ergebnis hat.

Wenn Sie direkter in einer Agenten-Sitzung arbeiten möchten, hängen Sie sich an die Zeile an, um die vollständige Konversation zu betreten.

Um die Agenten-Ansicht mit Subagenten, Agent-Teams und Worktrees zu vergleichen, siehe [Agenten parallel ausführen](/docs/de/agents).

<Note>
  Die Agenten-Ansicht ist eine Forschungsvorschau und erfordert Claude Code v2.1.139 oder später. Überprüfen Sie Ihre Version mit `claude --version`. Die Benutzeroberfläche und Tastaturkürzel können sich ändern, wenn sich die Funktion weiterentwickelt.
</Note>

Diese Seite behandelt:

* [Schnellstart](#quick-start): Geben Sie Claude eine Aufgabe, an der sie im Hintergrund arbeiten kann, überprüfen Sie sie und greifen Sie ein, wenn nötig
* [Sitzungen mit der Agenten-Ansicht überwachen](#monitor-sessions-with-agent-view), einschließlich Statussymbole, Vorschau und Antwort, Anhängen, Organisieren und Tastaturkürzel
* [Neue Agenten versenden](#dispatch-new-agents) aus der Agenten-Ansicht, aus einer Sitzung heraus oder aus Ihrer Shell
* [Sitzungen aus der Shell verwalten](#manage-sessions-from-the-shell) mit `claude agents`, `claude attach` und verwandten Befehlen
* [Wie Hintergrund-Sitzungen gehostet werden](#how-background-sessions-are-hosted) durch den Supervisor-Prozess

<h2 id="quick-start">
  Schnellstart
</h2>

Diese Anleitung behandelt die Kern-Agenten-Ansicht-Schleife: versenden Sie eine Aufgabe, beobachten Sie, wie ihre Zeile aktualisiert wird, während Claude arbeitet, schauen Sie nach, um sie zu überprüfen und zu antworten, und hängen Sie sich für das vollständige Gespräch an. Die Sitzung, die Sie versenden, läuft weiter, nachdem Sie die Agenten-Ansicht schließen, sodass Sie sie verlassen und später zurückkehren können.

<Steps>
  <Step title="Agenten-Ansicht öffnen">
    Führen Sie in Ihrer Shell aus:

    ```bash theme={null}
    claude agents
    ```

    Die Agenten-Ansicht öffnet sich mit einer Eingabe am unteren Rand und einer Tabelle, die sich füllt, wenn Sitzungen starten. Drücken Sie jederzeit `Esc`, um zu Ihrer Shell zurückzukehren. Ihre Sitzungen laufen weiter, während Sie weg sind, und erscheinen erneut, wenn Sie die Agenten-Ansicht das nächste Mal öffnen.
  </Step>

  <Step title="Eine Sitzung versenden">
    Geben Sie eine Eingabeaufforderung ein, die eine Aufgabe beschreibt, und drücken Sie `Enter`. Eine neue Hintergrund-Sitzung startet bei dieser Aufgabe und wird als Zeile angezeigt, die zeigt, ob sie funktioniert, auf Sie wartet oder erledigt ist. Die neue Sitzung verwendet das Modell, das in der Agenten-Ansicht-Kopfzeile angezeigt wird, und denselben [Berechtigungsmodus](#permission-mode-model-and-effort), den Sie erhalten würden, wenn Sie `claude` in diesem Verzeichnis ausführen.

    Jede Eingabeaufforderung, die Sie hier eingeben, startet ihre eigene neue Sitzung. Wenn Sie eine weitere Eingabeaufforderung eingeben und `Enter` drücken, wird eine zweite Sitzung neben der ersten gestartet, anstatt eine Folgefrage daran zu senden. Sie können auf diese Weise mehrere parallel ausführen.

    Jede Sitzung nutzt Ihr Abonnementkontingent unabhängig, daher lesen Sie [Einschränkungen](#limitations), bevor Sie viele auf einmal versenden.
  </Step>

  <Step title="Vorschau und Antwort">
    Wählen Sie eine Zeile mit den Pfeiltasten aus und drücken Sie `Space`, um das Vorschau-Panel zu öffnen. Es zeigt die neueste Ausgabe der Sitzung oder die Frage, auf die sie wartet, anstelle des vollständigen Transkripts. Geben Sie eine Antwort ein und drücken Sie `Enter`, um sie zu senden, ohne die Agenten-Ansicht zu verlassen.
  </Step>

  <Step title="Anhängen und Abhängen">
    Drücken Sie `Enter` oder `→` auf einer Zeile, um sich anzuhängen, wenn Sie das vollständige Gespräch möchten. Die Sitzung übernimmt das Terminal als vollständige interaktive Claude Code-Sitzung. Drücken Sie `←` auf einer leeren Eingabeaufforderung, um sich abzuhängen und zur Tabelle zurückzukehren.
  </Step>

  <Step title="Eine vorhandene Sitzung einbringen">
    Dieser Schritt benötigt eine laufende Sitzung. Wenn Sie die früheren Schritte befolgt haben, haben Sie keine offene Sitzung in diesem Terminal, daher öffnen Sie eine reguläre `claude`-Sitzung in einem anderen Terminal und senden Sie ihr zuerst eine Nachricht. Um eine Sitzung, die Sie bereits offen haben, in die Agenten-Ansicht zu verschieben, führen Sie `/bg` darin aus, oder drücken Sie `←` auf einer leeren Eingabeaufforderung, um sie in den Hintergrund zu verschieben und die Agenten-Ansicht in einem Schritt zu öffnen. Die Sitzung läuft weiter und wird als Zeile neben den Sitzungen angezeigt, die Sie versendet haben.
  </Step>
</Steps>

Sie können `claude agents` als Ihren primären Einstiegspunkt anstelle von `claude` verwenden: versenden Sie jede Aufgabe aus der Agenten-Ansicht, hängen Sie sich an, wenn Sie das vollständige Gespräch möchten, und drücken Sie `←`, um zur Tabelle zurückzukehren.

Innerhalb einer regulären `claude`-Sitzung zählt der Hinweis `←` in der Eingabeaufforderungs-Fußzeile die Hintergrund-Agenten, die auf Sie warten, wie z. B. `← 2 agents`, und kehrt zu `← for agents` zurück, wenn keiner Eingaben benötigt. Zählungen über 99 werden als `99+` angezeigt. Die Zählung wird etwa alle zehn Sekunden aktualisiert, während das Terminal fokussiert ist, und sofort, wenn der Fokus zurückkehrt. Sie ändert kurzzeitig die Farbe, wenn sie sich bewegt und wenn ein Agent abgeschlossen wird, es sei denn, die Einstellung [`prefersReducedMotion`](/docs/de/settings#available-settings) ist aktiviert, und sie ist im [Bildschirmlesemodus](/docs/de/accessibility) verborgen. Auf [Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry](/docs/de/third-party-integrations) bleibt der Hinweis in seiner einfachen Form `← for agents` ohne die Zählung. Erfordert Claude Code v2.1.205 oder später.

<h2 id="monitor-sessions-with-agent-view">
  Sitzungen mit der Agenten-Ansicht überwachen
</h2>

Führen Sie `claude agents` aus, um die Agenten-Ansicht zu öffnen. Sie übernimmt das gesamte Terminal und listet jede Sitzung nach Status gruppiert auf, mit angehefteten Sitzungen und denjenigen, die Sie benötigen, oben. Jede Zeile zeigt den Namen der Sitzung, die aktuelle Aktivität und ihr Alter, gezählt von der Erstellung der Sitzung; das Alter einer abgeschlossenen Sitzung friert bei der Dauer der Ausführung ein.

Der Name ist mit der Farbe getönt, die durch [`/color`](/docs/de/commands) in dieser Sitzung festgelegt wurde. Ab v2.1.199 wird die Farbe beibehalten, wenn Sie eine Sitzung [in den Hintergrund verschieben](#from-inside-a-session) mit `←` oder `/background`.

Standardmäßig zeigt die Liste jede Hintergrund-Sitzung, die Sie gestartet haben, über alle Ihre Projekte hinweg. Eine Sitzung, die in einem Repository funktioniert, und eine andere in einem anderen Worktree erscheinen beide hier, unabhängig davon, aus welchem Verzeichnis Sie die Agenten-Ansicht geöffnet haben. Um die Liste auf ein Projekt zu beschränken, übergeben Sie `--cwd`:

```bash theme={null}
claude agents --cwd ~/projects/my-app
```

Dies zeigt nur Sitzungen an, die unter diesem Verzeichnis gestartet wurden. Eine Sitzung, die [in einen Worktree verschoben wurde](#how-file-edits-are-isolated) unter `~/projects/my-app/.claude/worktrees/`, zählt immer noch als zu `~/projects/my-app` gehörend.

Interaktive Sitzungen, die Sie in anderen Terminals offen haben, werden nicht angezeigt, bis Sie sie [in den Hintergrund verschieben](#from-inside-a-session). [Subagenten](/docs/de/sub-agents) und [Teamkollegen](/docs/de/agent-teams), die eine Sitzung startet, werden nicht als separate Zeilen aufgelistet.

```text theme={null}
Angeheftet
  ✽ clawd walk cycle          Drawing the walk-cycle sprite frames          3m

Bereit zur Überprüfung
  ∙ jump physics              Opened PR with collision fix                 #2048  2h

Benötigt Eingabe
  ✻ power-up design           double jump or wall climb?                    1m

Funktioniert
  ✽ collision detection       Adding swept-AABB checks to CollisionSystem   2m
  ✢ playtest level 3          run 12 · all checkpoints cleared           in 4m

Abgeschlossen
  ✻ title screen              result: menu, options, and credits done       9m
  ∙ sound effects             result: 14 SFX exported to assets/audio       4h
  … 6 more
```

<h3 id="read-session-state">
  Sitzungsstatus lesen
</h3>

Jede Zeile beginnt mit einem Symbol, dessen Farbe und Animation den Status der Sitzung anzeigen:

| Status           | Symbol zeigt sich als | Was es bedeutet                                                                |
| :--------------- | :-------------------- | :----------------------------------------------------------------------------- |
| Funktioniert     | Animiert              | Claude führt aktiv Tools aus oder generiert eine Antwort                       |
| Benötigt Eingabe | Gelb                  | Claude wartet auf eine bestimmte Frage oder Genehmigungsentscheidung von Ihnen |
| Untätig          | Gedimmt               | Die Sitzung hat nichts zu tun und ist bereit für Ihren nächsten Prompt         |
| Abgeschlossen    | Grün                  | Die Aufgabe wurde erfolgreich abgeschlossen                                    |
| Fehlgeschlagen   | Rot                   | Die Aufgabe endete mit einem Fehler                                            |
| Gestoppt         | Grau                  | Die Sitzung wurde mit `Ctrl+X` oder `claude stop` gestoppt                     |

Separat zeigt die Form des Symbols, ob der zugrunde liegende Prozess läuft:

| Form                    | Was es bedeutet                                                                                                                                 |
| :---------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------- |
| `✻` oder animiertes `✽` | Der Sitzungsprozess ist aktiv und antwortet sofort                                                                                              |
| `∙`                     | Der Prozess ist beendet. Sie können immer noch Vorschau anzeigen, antworten oder anhängen, und Claude startet von dort neu, wo es aufgehört hat |
| `✢`                     | Eine [`/loop`](/docs/de/scheduled-tasks)-Sitzung, die zwischen Iterationen schläft. Die Zeile zeigt ihre Laufzahl und einen Countdown                |

Das `#N`-Label, das am rechten Rand einer Zeile erscheinen kann, ist ein [Pull Request, mit dem die Sitzung verknüpft ist](#pull-request-status), nicht Teil des Status-Symbols.

Der Terminal-Tab-Titel zeigt die Anzahl der wartenden Eingaben, während die Agenten-Ansicht offen ist: `2 awaiting input · claude agents`, wenn Sitzungen Eingabe benötigen, oder `claude agents`, wenn keine benötigen.

Ab v2.1.198 sendet Claude Code auch eine Benachrichtigung über Ihren konfigurierten [Terminal-Benachrichtigungskanal](/docs/de/terminal-config#get-a-terminal-bell-or-notification), während die Agenten-Ansicht offen ist, wenn eine lokale Hintergrund-Sitzung Ihre Eingabe benötigt, fertig wird oder fehlschlägt. Sitzungen, die nach einem Zeitplan ausgeführt werden, wie z. B. [`/loop`](/docs/de/scheduled-tasks)-Sitzungen, benachrichtigen nur, wenn sie Ihre Eingabe benötigen. Benachrichtigungen verwenden die gleiche [`preferredNotifChannel`-Einstellung](/docs/de/settings#available-settings) wie der Rest von Claude Code und lösen den [`Notification`-Hook](/docs/de/hooks#notification) mit dem Typ `agent_needs_input` oder `agent_completed` aus.

Hintergrund-Sitzungen benötigen kein offenes Terminal, um weiter zu funktionieren. Ein separater [Supervisor-Prozess](#the-supervisor-process) führt sie aus, sodass Sie die Agenten-Ansicht schließen, Ihre Shell schließen oder eine neue interaktive Sitzung starten können und Ihre versendete Arbeit läuft weiter.

Der Sitzungsstatus wird auf der Festplatte durch automatische Updates und Supervisor-Neustarts beibehalten. Sitzungen werden auch beibehalten, wenn Ihr Computer in den Ruhezustand wechselt. Ihre Prozesse werden beim Aufwachen fortgesetzt und der Supervisor verbindet sich wieder mit ihnen, anstatt die Zeitlücke als untätig zu behandeln. Das Herunterfahren stoppt immer noch laufende Sitzungen; siehe [Sitzungen werden nach dem Herunterfahren als fehlgeschlagen angezeigt](#sessions-show-as-failed-after-shutdown), um zu erfahren, wie Sie sie wiederherstellen.

Wenn Sie eine Sitzung öffnen, die nicht mehr reagiert, startet der Supervisor ihren Prozess neu und die Sitzung setzt die unterbrochene Antwort von dort fort, wo sie unterbrochen wurde. Eine Sitzung kann in diesen Zustand geraten, wenn der Computer in den Ruhezustand wechselt, während er sich mitten in einer Antwort befindet. Erfordert Claude Code v2.1.200 oder später.

<h3 id="row-summaries">
  Zeilenzusammenfassungen
</h3>

Die einzeilige Zusammenfassung in jeder Zeile wird von einem [Haiku-Klasse-Modell](/docs/de/model-config) generiert, sodass die Zeile Ihnen zeigen kann, was die Sitzung tut, was sie benötigt oder was sie produziert hat, ohne das Transkript zu öffnen. Während eine Sitzung aktiv funktioniert, wird der Zeilentext höchstens alle 15 Sekunden aus der eigenen aktuellen Ausgabe der Sitzung aktualisiert, ohne eine Modellanfrage zu senden, und das Modell schreibt eine neue Zusammenfassung, wenn jede Runde endet.

Eine arbeitende Zeile zeigt, was die Sitzung sagt, dass sie tut, und eine blockierte Zeile zeigt die Frage, die sie stellt. Während einer langen Runde schreibt das Modell die Zusammenfassung auch etwa einmal pro Minute neu, wartet nach jedem Neuschreiben doppelt so lange, bis zu vier Minuten, sodass eine beschäftigte Zeile nicht weiterhin eine veraltete Zusammenfassung anzeigt. Der Zusammenfassungstext füllt die verbleibende Breite der Zeile und wird nur am rechten Rand des Terminals abgeschnitten; öffnen Sie das [Vorschau-Panel](#peek-and-reply), um einen Satz zu lesen, den der Rand abschneidet. Vor v2.1.205 konnte eine arbeitende Zeile einen rohen Tool-Aufruf anstelle eines Berichts anzeigen, und eine Sitzung, die parallele Arbeitselemente ausführte, zeigte eine `done/total`-Anzahl wie `2/5` vor dem Text.

Der Zusammenfassungstext füllt die verbleibende Breite der Zeile und wird nur am rechten Rand des Terminals abgeschnitten; öffnen Sie das [Vorschau-Panel](#peek-and-reply), um einen Satz zu lesen, den der Rand abschneidet. Vor v2.1.206 wurde der Text bei 64 Spalten abgeschnitten, unabhängig von der Terminal-Breite.

Wenn die Liste [nach Verzeichnis gruppiert ist](#organize-the-list), öffnet sich die Zusammenfassung mit dem Status der Sitzung als farbiges Wort, z. B. `Benötigt Eingabe · double jump or wall climb?`. Bei der Standard-Status-Gruppierung benennt der Gruppenkopf bereits den Status, sodass die Zeile nur die Zusammenfassung anzeigt. Vor v2.1.205 trugen verzeichnis-gruppierte Zeilen kein Status-Wort.

Eine Runde, deren gesamte Ausgabe keine Buchstaben oder Ziffern enthält, wie z. B. eine [`/loop`](/docs/de/scheduled-tasks)-Sitzung, die bei einer ruhigen Iteration ein einzelnes Symbol druckt, behält die vorherige Zusammenfassung und den Status der Zeile bei. Vor v2.1.205 wurde diese Runde neu klassifiziert und konnte eine Sitzung, die auf Ihre Eingabe wartete, zurück zu `Funktioniert` umschalten.

Die End-of-Turn-Zusammenfassung und jedes Mid-Turn-Neuschreiben sind eine kurze Haiku-Klasse-Anfrage durch Ihren normalen Anbieter, abgerechnet und behandelt unter denselben [Datennutzungsbedingungen](/docs/de/data-usage) wie die Sitzung selbst. Die 15-Sekunden-Updates zwischen Modell-Neuschreiben verwenden die eigene Ausgabe der Sitzung wieder und senden keine Anfrage. Bei Drittanbieter-Anbietern wie Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und benutzerdefinierten Gateways wird die Anfrage auf das Hauptmodell der Sitzung zurückgeführt, wenn kein Haiku-Modell konfiguriert ist. Legen Sie [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/de/model-config#environment-variables) fest, um das Modell für diese Zusammenfassungen bei diesen Anbietern auszuwählen.

<h3 id="pull-request-status">
  Pull-Request-Status
</h3>

Wenn eine Sitzung einen Pull Request öffnet, wird ein `#1234`-Label am rechten Rand der Zeile angezeigt, verlinkt zum Pull Request in Terminals, die Hyperlinks unterstützen. Das Label bleibt bestehen, wenn Sie eine Nachverfolgung an die Sitzung senden, sodass der Pull Request sichtbar bleibt, während die Zeile zum Live-Fortschritt zurückkehrt. Hintergrund-Sitzungen, die ihre Änderungen in einem Worktree isoliert haben, öffnen diese Pull Requests selbst; [Wie Datei-Edits isoliert werden](#how-file-edits-are-isolated) behandelt, wann das passiert und was eine Sitzung niemals ohne zu fragen tut.

Eine Sitzung, die an einem bestehenden Pull Request arbeitet, ist auf die gleiche Weise damit verknüpft. Das Bearbeiten, Kommentieren, Schließen oder Markieren eines Pull Requests als bereit mit `gh` verknüpft den Pull Request, den die Ausgabe des Befehls selbst benennt, sodass ein `gh`-Befehl, dessen erfasste Ausgabe keinen Pull Request benennt, keinen Link erstellt; `gh pr merge` ist der häufige Fall, da er sein Ergebnis nur auf einem interaktiven Terminal druckt. Das Auschecken eines Pull Requests mit `gh pr checkout` oder das Pushen zu einem Branch, der einen offenen Pull Request hat, verknüpft ihn, indem stattdessen `gh pr view` nachgeschlagen wird. Vor v2.1.205 waren nur Pull Requests, die die Sitzung erstellt oder ausgecheckt hatte, verknüpft, und ein Push verknüpfte einen nur, wenn der lokale Branch-Name übereinstimmte.

Claude Code liest den Pull Request aus der vollständigen Befehlsausgabe, einschließlich des Teils, der in einer Datei gespeichert wird, wenn die Ausgabe eines Befehls das Inline-Limit überschreitet. Vor v2.1.205 war ein Pull Request, der in einem Bash-Aufruf erstellt wurde, dessen Ausgabe etwa 30.000 Zeichen überschritt, nicht verknüpft.

Wenn eine Sitzung mit mehr als einem Pull Request verknüpft ist, zeigt das Label stattdessen eine Anzahl, z. B. `3 PRs`, farbig gekennzeichnet nach dem offenen Pull Request, der am meisten Aufmerksamkeit benötigt. Öffnen Sie das [Vorschau-Panel](#peek-and-reply), um sie alle zu sehen.

Die Pull-Request-Nummer ist nach ihrem Status farbig gekennzeichnet:

| Farbe | Pull-Request-Status                                                          |
| :---- | :--------------------------------------------------------------------------- |
| Gelb  | Wartet auf Überprüfungen oder Überprüfung, oder Überprüfungen fehlgeschlagen |
| Grün  | Überprüfungen bestanden und keine Überprüfung blockiert                      |
| Lila  | Zusammengeführt                                                              |
| Grau  | Entwurf oder geschlossen                                                     |

Für die meisten Aufgaben ist diese Spalte, wo Sie das Ergebnis aufgreifen: überprüfen und führen Sie den Pull Request zusammen, wenn seine Nummer grün wird.

<h3 id="peek-and-reply">
  Vorschau und Antwort
</h3>

Drücken Sie `Space` auf einer ausgewählten Zeile, um das Vorschau-Panel zu öffnen. Es öffnet sich mit dem Satz, den die Zeile am Terminal-Rand abschneidet, und welcher Satz das ist, hängt vom Status der Sitzung ab:

* Eine Sitzung, die auf Sie wartet: die genaue Frage, die sie stellt, über der Antwort-Eingabe
* Eine abgeschlossene Sitzung: ihr Ergebnis
* Eine arbeitende Sitzung: ihr vollständiger Status-Satz

Alle Pull Requests, die mit der Sitzung verknüpft sind, werden als nächstes aufgelistet. Für eine Sitzung, die auf Sie wartet, zeigt eine Zeile wie `waiting 3m` unter ihnen, wie lange sie wartet, und es ist die einzige Zeit, die im Panel angezeigt wird. Das Alter am rechten Rand der Zeile ist eine andere Zahl: es zählt von der Erstellung der Sitzung.

Meistens reicht das Vorschau-Panel aus und Sie müssen das vollständige Transkript nie öffnen.

Vor v2.1.207 öffnete sich jede Vorschau mit dem Status-Satz und einem bloßen Zeitstempel, und eine blockierte Sitzung's Frage erschien darunter mit dem gleichen Zeitstempel ein zweites Mal vorangestellt.

Geben Sie eine Antwort im Vorschau-Panel ein und drücken Sie `Enter`, um sie an diese Sitzung zu senden. Wenn die Sitzung eine Multiple-Choice-Frage stellt, zeigt das Vorschau-Panel die Optionen an und Sie können eine Zahlentaste drücken, um eine auszuwählen. Für andere blockierte Sitzungen drücken Sie `Tab`, um die Eingabe mit einer vorgeschlagenen Antwort zu füllen, die Sie vor dem Senden bearbeiten können. Präfixieren Sie eine Antwort mit `!`, um stattdessen einen Bash-Befehl zu senden.

Eine Antwort, die nicht zugestellt werden kann, weil der Hintergrund-Service nicht erreichbar ist oder der Versand fehlschlägt, wird gespeichert und an die Sitzung als nächster Prompt gesendet, wenn ihr Prozess wieder startet, und die Fehlermeldung besagt, dass die Antwort gespeichert wurde. Eine Antwort mit dem Präfix `!` wird nicht gespeichert, da der gespeicherte Text die Sitzung als einfacher Prompt erreichen würde, anstatt als Bash-Befehl ausgeführt zu werden.

Mit [Sprachdiktat](/docs/de/voice-dictation) aktiviert können Sie Ihre Push-to-Talk-Taste halten oder tippen, während die Antwort-Eingabe fokussiert ist, um eine Antwort zu diktieren, anstatt sie einzugeben. Das Gleiche funktioniert in der Versand-Eingabe am unteren Rand der Agenten-Ansicht.

Verwenden Sie `↑` und `↓`, um benachbarte Sitzungen in der Vorschau anzuzeigen, ohne das Panel zu schließen, oder `→`, um sich anzuhängen.

<h3 id="attach-to-a-session">
  An eine Sitzung anhängen
</h3>

Drücken Sie `Enter` oder `→` auf einer ausgewählten Zeile, um sich anzuhängen. Die Agenten-Ansicht wird durch die vollständige interaktive Sitzung ersetzt. Wenn Sie sich anhängen, sendet Claude eine kurze Zusammenfassung dessen, was passiert ist, während Sie weg waren.

Während Sie angehängt sind, verhält sich die Sitzung wie jede andere Claude Code-Sitzung: [Befehle](/docs/de/commands), Tastaturkürzel und Funktionen funktionieren alle, mit den folgenden Ausnahmen.

Eine Hintergrund-Sitzung lehnt `/install-github-app` und die [`/mcp`](/docs/de/mcp)-Einstellungsliste, einschließlich ihrer Authentifizierungsaktionen, ab, ob Sie angehängt sind oder vom Vorschau-Panel aus antworten. Die Nachricht leitet Sie zu einer regulären `claude`-Sitzung, und `/mcp reconnect <server>`, `/mcp enable` und `/mcp disable` funktionieren immer noch.

Angehängte Sitzungen werden immer im [Vollbildmodus](/docs/de/fullscreen) gerendert, unabhängig von Ihrer `tui`-Einstellung, da eine Hintergrund-Sitzung keinen Terminal-Scrollback zum Anhängen hat. Scrollen Sie mit `PgUp`, `PgDn` oder dem Mausrad, und drücken Sie `Ctrl+O` für den Transkript-Modus. Ihr natives Terminal-Scroll und tmux-Kopiermodus zeigen nur den aktuellen Viewport, genau wie wenn Sie eine beliebige Vollbildanwendung ausführen.

Drücken Sie `←` auf einer leeren Eingabeaufforderung, oder führen Sie `/exit` aus, um sich abzuhängen und zur Agenten-Ansicht zurückzukehren. Ab v2.1.198 funktioniert dies auf die gleiche Weise, ob Sie die Sitzung von der Agenten-Ansicht aus geöffnet haben oder mit `claude attach <id>` von Ihrer Shell aus.

`Ctrl+Z` hängt auch ab, geht aber stattdessen dorthin zurück, wo Sie angefangen haben: Agenten-Ansicht, wenn Sie sich von dort aus angehängt haben, oder Ihre Shell, wenn Sie `claude attach` ausgeführt haben. Verwenden Sie `Ctrl+Z`, wenn ein Dialog den Fokus hat und nicht auf `←` reagiert.

`Ctrl+C` behält sein Standardunterbrechungsverhalten bei, während es angehängt ist: Es bricht eine laufende Antwort oder einen `!`-Shell-Befehl ab, anstatt sich abzuhängen. Das zweimalige Drücken von `Ctrl+C` auf einer leeren Eingabeaufforderung hängt ab, genau wie in jeder anderen Sitzung.

Das Abhängen stoppt niemals eine Hintergrund-Sitzung: `←`, `Ctrl+Z`, `/exit` und doppeltes `Ctrl+C` oder doppeltes `Ctrl+D` lassen sie alle laufen. Um eine Sitzung von innen zu beenden, führen Sie `/stop` aus.

In einer Sitzung, die im Vordergrund läuft, eine, die Sie im Terminal gestartet haben, anstatt sich von der Agenten-Ansicht anzuhängen, verschiebt das Drücken von `←` auf einer leeren Eingabeaufforderung sie in den Hintergrund und öffnet die Agenten-Ansicht mit dieser Zeile ausgewählt, sodass Sie zwischen Sitzungen wechseln können, ohne das Terminal zu verlassen. Der gleiche einzelne Druck hängt eine angehängte Sitzung ab.

Wenn ein Tool läuft, wenn Sie `←` drücken, wartet Claude Code bis zu etwa zehn Sekunden, bis es fertig ist, bevor es in den Hintergrund geht, und die Antwort wird in der Hintergrund-Sitzung fortgesetzt. Drücken Sie `←` erneut, um sofort in den Hintergrund zu gehen, anstatt zu warten. Wenn laufende Arbeit nicht zur Hintergrund-Sitzung übertragen werden kann, wird zuerst das Dialog `Background this session?` angezeigt, genauso wie mit [`/background`](#from-inside-a-session).

Die Zehn-Sekunden-Grenze gilt nicht, während [Subagenten](/docs/de/sub-agents) laufen. Claude Code wartet weiter, damit ihre Arbeit übertragen wird, und zeigt eine `Still backgrounding after the current tool`-Benachrichtigung an, während es wartet; drücken Sie `←` erneut, um ohne Warten in den Hintergrund zu gehen, was die Subagenten von vorne neu startet. Vor v2.1.203 endete das Warten nach zehn Sekunden und die laufenden Subagenten wurden ohne Warnung von vorne neu gestartet.

Die Zeile wird auch aus einer neuen Sitzung ohne Gesprächsverlauf erstellt, sodass `→` zu ihr zurückkehrt. Vor v2.1.203 zeigte die Agenten-Ansicht einen Onboarding-Hinweis unter dieser Zeile an, wenn sie die einzige war.

Sie können diesen Tastaturkürzel mit der `leftArrowOpensAgents`-Einstellung in `/config` ausschalten.

<h3 id="organize-the-list">
  Die Liste organisieren
</h3>

Die Agenten-Ansicht gruppiert Sitzungen, sodass diejenigen, die Eingabe benötigen, oben sind, mit `Bereit zur Überprüfung` und `Benötigt Eingabe` über `Funktioniert` und `Abgeschlossen`. Diese Gruppennamen entsprechen nicht eins-zu-eins den [Status](#read-session-state) oben: Eine Sitzung wechselt zu `Bereit zur Überprüfung`, wenn sie einen offenen Pull Request hat, und `Abgeschlossen` sammelt beendete, fehlgeschlagene und gestoppte Sitzungen zusammen.

Drücken Sie `Ctrl+S`, um stattdessen nach Verzeichnis zu gruppieren. Ihre Wahl wird über Läufe hinweg gespeichert.

Innerhalb einer Gruppe:

* Drücken Sie `Ctrl+T`, um eine Sitzung oben anzuheften und [ihren Prozess während der Untätigkeit laufen zu lassen](#the-supervisor-process)
* Drücken Sie `Shift+↑` oder `Shift+↓`, um Sitzungen neu anzuordnen
* Drücken Sie `Ctrl+R`, um eine Sitzung umzubenennen
* Drücken Sie `Enter` auf einem Gruppenkopf, um ihn zu reduzieren

Um eine Sitzung aus der Liste zu entfernen, drücken Sie `Ctrl+X`, um sie zu stoppen, und `Ctrl+X` erneut innerhalb von zwei Sekunden, um sie zu löschen. Das Drücken von `Ctrl+X` auf einem Gruppenkopf löscht jede Sitzung in dieser Gruppe nach Bestätigung.

Das Löschen entfernt die Sitzung aus der Agenten-Ansicht. Wenn Claude [einen Worktree erstellt hat](#how-file-edits-are-isolated) für die Sitzung, entfernt das Löschen auch diesen Worktree, einschließlich aller nicht committeten Änderungen darin, sodass Sie Arbeit, die Sie behalten möchten, zuerst committen sollten. Ein Worktree, den Sie selbst erstellt haben und die Sitzung darin gestartet haben, wird an Ort und Stelle gelassen. Das Gesprächstranskript bleibt auf Ihrem lokalen Computer und bleibt über `claude --resume` verfügbar.

Das Löschen entfernt niemals einen Worktree mit Commits, die nirgendwo gepusht werden, oder einen, den eine andere laufende Sitzung beansprucht oder gesperrt hat. Claude Code behält den Worktree und die Sitzung, und die Fußzeile benennt den beibehaltenen Pfad und den Grund. Pushen Sie die Commits, oder schließen Sie die andere Sitzung, dann löschen Sie erneut.

Das Löschen löscht auch die Sitzung aus der [Supervisor's](#the-supervisor-process)-Sitzungsliste, ob Sie mit `Ctrl+X` löschen oder mit [`claude rm`](#manage-sessions-from-the-shell) von der Shell aus, sodass die Entfernung über Supervisor-Neustarts hinweg bestehen bleibt. Vor v2.1.206 ließ das Entfernen einer Sitzung, während der Supervisor neu startete oder nicht erreichbar war, sie in dieser Liste, und der nächste Supervisor startete seinen Prozess neu und zeigte die Zeile erneut.

Abgeschlossene Sitzungen, die nicht auf den Bildschirm passen, werden in eine Zeile `… N more` eingeklappt. Fehler und Sitzungen mit einem offenen Pull Request bleiben immer sichtbar. Die Gruppe `Abgeschlossen` füllt den verbleibenden vertikalen Platz nach den Live-Gruppen, und auf einem kurzen Terminal wird die Kopfzeile auf eine einzelne Zusammenfassungszeile komprimiert, sodass Sitzungen, die funktionieren oder Eingabe benötigen, sichtbar bleiben.

<h3 id="filter-sessions">
  Sitzungen filtern
</h3>

Geben Sie in die Versand-Eingabe ein, um zu filtern, anstatt zu versenden:

| Filter                       | Zeigt                                                                                                         |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------ |
| `a:<name>`                   | Sitzungen, die den benannten Agenten ausführen                                                                |
| `s:<state>`                  | Sitzungen im angegebenen Status, z. B. `s:working`. Akzeptiert auch `s:blocked` für alles, das auf Sie wartet |
| `#<number>` oder eine PR-URL | Die Sitzung, die an diesem Pull Request funktioniert                                                          |
| Jede andere URL              | Die Sitzung, deren erste Eingabeaufforderung diese URL enthielt                                               |

<h3 id="keyboard-shortcuts">
  Tastaturkürzel
</h3>

Drücken Sie `?` in der Agenten-Ansicht, um jedes Kürzel im Kontext zu sehen. Die folgende Tabelle fasst sie zusammen.

| Kürzel                | Aktion                                                                                             |
| :-------------------- | :------------------------------------------------------------------------------------------------- |
| `↑` / `↓`             | Zwischen Zeilen verschieben                                                                        |
| `Enter`               | An die ausgewählte Sitzung anhängen oder versenden, wenn Text in der Eingabe vorhanden ist         |
| `Space`               | Vorschau-Panel für die ausgewählte Sitzung öffnen oder schließen                                   |
| `Shift+Enter`         | Versenden und sofort anhängen                                                                      |
| `→`                   | An die ausgewählte Sitzung anhängen                                                                |
| `Alt+1`..`Alt+9`      | An Sitzung 1–9 im fokussierten Sitzungsverzeichnis anhängen                                        |
| `Tab`                 | Bei leerer Eingabe alle Subagenten durchsuchen. Andernfalls den hervorgehobenen Vorschlag anwenden |
| `Ctrl+S`              | Gruppierung zwischen Status und Verzeichnis wechseln                                               |
| `Ctrl+T`              | Ausgewählte Sitzung anheften oder abheften                                                         |
| `Ctrl+R`              | Ausgewählte Sitzung umbenennen                                                                     |
| `Ctrl+G`              | Versand-Eingabeaufforderung in Ihrem `$VISUAL` oder `$EDITOR` öffnen                               |
| `Ctrl+X`              | Sitzung stoppen; drücken Sie erneut innerhalb von zwei Sekunden, um sie zu löschen                 |
| `Shift+↑` / `Shift+↓` | Ausgewählte Sitzung neu anordnen                                                                   |
| `Esc`                 | Vorschau-Panel schließen, Eingabe löschen oder beenden                                             |
| `Ctrl+C`              | Eingabe löschen; zweimal drücken, um zu beenden                                                    |
| `?`                   | Alle Kürzel anzeigen                                                                               |

<h2 id="dispatch-new-agents">
  Neue Agenten versenden
</h2>

Sie können neue Hintergrund-Sitzungen aus der Agenten-Ansicht versenden, eine vorhandene interaktive Sitzung in den Hintergrund verschieben oder eine direkt aus der Shell starten.

<h3 id="from-agent-view">
  Aus der Agenten-Ansicht
</h3>

Geben Sie eine Eingabeaufforderung in die Eingabe am unteren Rand der Agenten-Ansicht ein und drücken Sie `Enter`, um eine neue Hintergrund-Sitzung zu starten. Die Sitzung wird automatisch aus der Eingabeaufforderung benannt. Sie können sie später mit `Ctrl+R` umbenennen.

Ein Name, den die Sitzung später erhält, wird auch auf ihrer Zeile angezeigt, einschließlich des Namens, den Claude ableitet, wenn Sie [einen Plan akzeptieren](/docs/de/permission-modes#review-and-approve-a-plan) in dieser Sitzung. Vor v2.1.207 zeigte eine Hintergrund-Sitzung, die durch Akzeptieren eines Plans benannt wurde, diesen Namen in `/status` an, aber nicht auf ihrer Agenten-Ansicht-Zeile, bis Sie sie selbst umbenannten.

Fügen Sie ein Bild in die Eingabeaufforderung ein, um einen Screenshot oder ein Diagramm mit der Aufgabe einzubeziehen.

Eingefügter Text, der länger als 800 Zeichen ist oder mehr als zwei Zeilen umfasst, wird zu einem `[Pasted text #N]`-Platzhalter zusammengefasst, sodass die Eingabe auf einer Zeile bleibt; der vollständige Text wird beim Versenden gesendet. Um den zusammengefassten Text vor dem Versenden zu überprüfen oder zu bearbeiten, fügen Sie denselben Text erneut ein und der Platzhalter wird wieder in die Eingabe erweitert. Eine `paste again to expand`-Erinnerung wird für einige Sekunden unter der Eingabe angezeigt, auf Terminals mit mindestens 90 Spalten Breite. Vor v2.1.207 hat das erneute Einfügen desselben Textes einen zweiten Platzhalter hinzugefügt, anstatt den ersten zu erweitern.

Präfixieren oder erwähnen Sie Teile der Eingabeaufforderung, um zu steuern, wie die Sitzung startet:

| Eingabe                                | Effekt                                                                                                                                                                                                 |
| :------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `<agent-name> <prompt>`                | Wenn das erste Wort einem benutzerdefinierten [Subagenten](/docs/de/sub-agents)-Namen entspricht, wird dieser Subagent als Hauptagent der Sitzung mit der Konfiguration aus seinem Frontmatter ausgeführt   |
| `@<agent-name>`                        | Erwähnen Sie einen benutzerdefinierten Subagenten überall in der Eingabeaufforderung, um ihn als Hauptagent auszuführen                                                                                |
| `@<repo>`                              | Erwähnen Sie ein Repository, um die Sitzung dort auszuführen. Siehe [In ein bestimmtes Verzeichnis versenden](#dispatch-to-a-specific-directory), um zu erfahren, welche Repositories aufgelistet sind |
| `/<command>`                           | Schlagen Sie [Skills](/docs/de/skills) und [Befehle](/docs/de/commands) vor, um als Eingabeaufforderung zu versenden                                                                                             |
| `! <command>`                          | Führen Sie einen Shell-Befehl als Hintergrund-Job aus, anstatt eine Claude-Sitzung zu starten. Der Job wird als Zeile angezeigt, die Sie anhängen, beobachten und trennen können                       |
| `#<number>` oder eine Pull Request-URL | Wenn eine Sitzung bereits an diesem PR funktioniert, wählen Sie sie aus, anstatt zu versenden                                                                                                          |
| `Shift+Enter`                          | Versenden und sofort an die neue Sitzung anhängen                                                                                                                                                      |

Ein kleiner Satz von Befehlen wird in der Agenten-Ansicht selbst ausgeführt, anstatt zu versenden:

* `/exit` und `/quit` schließen die Agenten-Ansicht
* `/logout` meldet Sie ab
* `/model` setzt das [Versand-Modell](#set-the-model)
* Ab v2.1.198 öffnet `/login` den Anmeldedialog, damit Sie sich erneut anmelden können, ohne an eine Sitzung anzuhängen

Skills, Ihre eigenen Befehle und Eingabeaufforderungs-erweiternde Built-ins wie `/init` werden als erste Eingabeaufforderung an eine neue Hintergrund-Sitzung gesendet. Andere Built-in-Befehle zeigen stattdessen einen `attach to a session to run it`-Hinweis an. Alles, was Sie eingegeben haben, bleibt in der Eingabe neben dem Hinweis, damit Sie es bearbeiten können. Vor v2.1.203 hat der Hinweis die Eingabe gelöscht und der eingegebene Text ging verloren.

Das Verpacken einer wiederkehrenden Aufgabe als [Skill](/docs/de/skills) ermöglicht es Ihnen, denselben Workflow aus der Agenten-Ansicht wiederholt zu starten, ohne die Eingabeaufforderung erneut einzugeben.

Wenn derselbe `@name` sowohl einem Subagenten als auch einem gleichgeordneten Repository entspricht, hat der Subagent Vorrang. Die Übereinstimmung mit dem ersten Wort gilt auch, sodass eine Eingabeaufforderung, die mit einem Ihrer Subagenten-Namen beginnt, diesen Subagenten versendet, anstatt das Wort als einfachen Text zu behandeln. Verwenden Sie die `@`-Form, wenn Sie explizit sein möchten, oder beginnen Sie die Eingabeaufforderung mit einem anderen Wort, um die Übereinstimmung zu vermeiden.

<h4 id="dispatch-to-a-specific-directory">
  In ein bestimmtes Verzeichnis versenden
</h4>

Eine neue Sitzung läuft in dem Verzeichnis, in dem Sie die Agenten-Ansicht geöffnet haben. Um ein anderes Verzeichnis anzusteuern, verwenden Sie eines dieser Verfahren:

* Öffnen Sie `claude agents` in diesem Verzeichnis.
* Öffnen Sie `claude agents` in einem übergeordneten Verzeichnis und erwähnen Sie ein untergeordnetes Repository mit `@<repo>` in der Eingabeaufforderung. Wenn Sie `@` eingeben, werden diese Ziele aufgelistet:

  * Git-Repositories eine Ebene unter dem Startverzeichnis
  * Die registrierten [Git-Worktrees](/docs/de/worktrees) des Repositories, von dem Sie gestartet haben, die sich in seinem Verzeichnisbaum befinden, wie die, die Claude unter `.claude/worktrees/` erstellt, gekennzeichnet mit ihrem ausgecheckten Branch. Worktrees, die außerhalb des Repositories hinzugefügt wurden, wie mit `git worktree add ../feature`, werden nicht aufgelistet
  * Jedes Verzeichnis, das bereits eine Sitzung in der Liste hat

  Ein Verzeichnis, dessen Name ein Leerzeichen enthält, wird nicht aufgelistet. Vor v2.1.203 wurden registrierte Worktrees nicht aufgelistet, daher bedeutete das Versenden in einen Worktree das Ausführen von `claude --bg` aus dem Verzeichnis dieses Worktrees.
* Führen Sie in der Shell `cd` in das Verzeichnis aus und führen Sie `claude --bg "<prompt>"` aus.

Wenn die Agenten-Ansicht nach Verzeichnis gruppiert ist, wird das Verzeichnis der hervorgehobenen Zeile zum Versand-Ziel, sodass Sie zu einer Gruppe scrollen und darin versenden können, ohne den Pfad erneut einzugeben.

<h3 id="from-inside-a-session">
  Aus einer Sitzung heraus
</h3>

Führen Sie `/background` oder seinen Alias `/bg` aus, um das aktuelle Gespräch in eine Hintergrund-Sitzung zu verschieben. Übergeben Sie eine Eingabeaufforderung wie `/bg run the test suite and fix any failures`, um eine weitere Anweisung zu senden, bevor Sie verschieben. Wenn Claude antwortet, wenn Sie `/bg` ausführen, wird die Antwort in der Hintergrund-Sitzung fortgesetzt.

Das Beenden einer interaktiven Sitzung, die noch Hintergrund-Arbeit ausführt, wie z. B. Subagenten, Hintergrund-Shell-Befehle, Workflows oder [Monitore](/docs/de/tools-reference#monitor-tool), zeigt einen Dialog `Background work is running` an, anstatt sofort zu beenden. Ab v2.1.198 bietet der Dialog `Move to background and exit` neben `Exit anyway` und `Stay` an. Wenn Sie diese Option wählen, wird die Sitzung auf die gleiche Weise wie `/background` in den Hintergrund verschoben und Sie werden dann zu Ihrer Shell zurückgebracht, sodass Arbeit, die fortgesetzt werden kann, weiterläuft und die Sitzung in der Agenten-Ansicht angezeigt wird. Die Option wird nicht angezeigt, wenn die Agenten-Ansicht [ausgeschaltet](#turn-off-agent-view) ist.

Das Verschieben aus einer interaktiven Sitzung startet einen neuen Prozess, der aus dem gespeicherten Gespräch fortgesetzt wird, und laufende Arbeit wird übertragen: Ausführen von Hintergrund-Shell-Befehlen, Hintergrund-Subagenten, dynamische Workflows und geplante Aufgaben, die Sie mit [`/loop`](/docs/de/scheduled-tasks) erstellt haben, werden zur Hintergrund-Sitzung übertragen und laufen dort weiter. Ein Subagent wird zusammen mit allem übertragen, das er gestartet hat, daher wird er nur übertragen, wenn all diese Arbeit auch übertragen werden kann, einschließlich unter Windows. Um laufende Arbeit stattdessen zu stoppen, anstatt sie zu übertragen, setzen Sie die Umgebungsvariable [`CLAUDE_DISABLE_ADOPT=1`](/docs/de/env-vars#variables); Claude Code fordert Sie dann auf, zu bestätigen, bevor Sie verschieben.

Arbeit, die nicht übertragen werden kann, wie z. B. ein laufender [Monitor](/docs/de/tools-reference#monitor-tool), wird gestoppt. Ein Hintergrund-Subagent, der einen Monitor besitzt, wird zusammen mit ihm gestoppt. Wenn eine solche Arbeit läuft, zeigt Claude Code ein Dialog `Background this session?`, damit Sie bestätigen können, bevor es gestoppt wird.

Sobald sich die Sitzung im Hintergrund befindet, kann sie neue Subagenten, Monitore und Hintergrund-Befehle starten, und diese laufen bei späteren Trennungen und Wiederverbindungen weiter.

Konfigurationsflags aus dem ursprünglichen Start werden auf die versendete Sitzung übertragen, sodass ihre MCP-Server, Einstellungen und Fallback-Modell weiterhin wirksam sind:

* `--mcp-config` und `--strict-mcp-config`
* `--settings`
* `--add-dir`
* `--plugin-dir`
* `--fallback-model`
* `--allow-dangerously-skip-permissions`

Verzeichnisse, die Sie während der Sitzung mit [`/add-dir`](/docs/de/permissions#additional-directories-grant-file-access-not-configuration) hinzugefügt haben, werden ebenfalls übertragen.

Das Übertragen von `--allow-dangerously-skip-permissions` hält `bypassPermissions` in der versendeten Sitzung erreichbar, gewährt aber nichts Neues. Der Modus erfordert immer noch die gleiche einmalige interaktive Akzeptanz, die in [Berechtigungsmodus, Modell und Aufwand](#permission-mode-model-and-effort) beschrieben ist, bevor eine Sitzung ihn verwenden kann.

<h3 id="from-your-shell">
  Aus der Shell
</h3>

Übergeben Sie `--bg` oder seine lange Form `--background`, um eine Sitzung zu starten, die direkt in den Hintergrund geht:

```bash theme={null}
claude --bg "investigate the flaky SettingsChangeDetector test"
```

Die Eingabeaufforderung ist das Positionsargument, nicht ein `-p`-Wert. Ab v2.1.198 wird das Kombinieren von `--bg` mit `-p` oder `--print` mit einem Fehler abgelehnt, bevor eine Sitzung erstellt wird, da `--print` niemals die interaktive Sitzung startet, an die `claude agents` anhängt.

Um einen bestimmten Subagenten als Hauptagent der Sitzung auszuführen, kombinieren Sie `--bg` mit `--agent`:

```bash theme={null}
claude --agent code-reviewer --bg "address review comments on PR 1234"
```

Übergeben Sie `--name`, um den Anzeigenamen der Sitzung in der Agenten-Ansicht anstelle des automatisch generierten festzulegen:

```bash theme={null}
claude --bg --name "flaky-test-fix" "investigate the flaky SettingsChangeDetector test"
```

Nach dem Versenden druckt Claude die kurze ID der Sitzung und die Befehle zu ihrer Verwaltung. Wenn der Service, der Hintergrund-Sitzungen hostet, noch nicht läuft, kann `--bg` zuerst `Starting background service…` über dieser Ausgabe drucken. Wenn Sie `--name` übergeben, wird der Name nach der kurzen ID angezeigt:

```text theme={null}
backgrounded · 7c5dcf5d · flaky-test-fix
  claude agents             list sessions
  claude attach 7c5dcf5d    open in this terminal
  claude logs 7c5dcf5d      show recent output
  claude stop 7c5dcf5d      stop this session
```

<h4 id="run-a-shell-command">
  Einen Shell-Befehl ausführen
</h4>

Um einen Shell-Befehl als Hintergrund-Job anstelle einer Claude-Sitzung auszuführen, geben Sie `!` als erstes Zeichen der Agenten-Ansicht-Versand-Eingabe ein. Das `!` wird als Präfix angezeigt und alles, was Sie danach eingeben, ist der Befehl. Das folgende Beispiel versendet `pytest -x` aus dem Agenten-Ansicht-Eingabefeld:

```text theme={null}
! pytest -x
```

Drücken Sie `Enter`, um den Job zu starten. Der gleiche Job kann auch direkt aus Ihrer Shell mit `--exec` gestartet werden:

```bash theme={null}
claude --bg --exec 'pytest -x'
```

Der Befehl wird als PTY-gestützter Job ausgeführt und wird als Zeile in der Agenten-Ansicht angezeigt, mit der neuesten Ausgabezeile als Status. Ein Shell-Job führt den Befehl anstelle von Claude aus, sodass kein Modell aufgerufen wird und die Ausgabe nicht an eine Sitzung gesendet wird.

Um die Ausgabe zu sehen, hängen Sie an die Zeile an, drücken Sie `Space`, um einen Blick zu werfen, ohne anzuhängen, oder führen Sie `claude logs <id>` aus Ihrer Shell aus. Die erfasste Ausgabe bleibt im Speicher und wird nicht auf die Festplatte geschrieben. Die Zeile und ihre Ausgabe werden automatisch etwa fünf Minuten nach dem Beenden des Befehls bereinigt, daher lesen Sie sie vorher, wenn Sie das Ergebnis benötigen.

<h3 id="how-file-edits-are-isolated">
  Wie Dateibearbeitungen isoliert werden
</h3>

Jede Hintergrund-Sitzung, ob aus der Agenten-Ansicht, `/bg` oder `claude --bg` gestartet, beginnt in Ihrem Arbeitsverzeichnis. Bevor Dateien bearbeitet werden, verschiebt Claude die Sitzung in einen isolierten [Git-Worktree](/docs/de/worktrees) unter `.claude/worktrees/`, sodass parallele Sitzungen denselben Checkout lesen können, aber jede in ihren eigenen schreibt.

Claude überspringt den Worktree, wenn:

* Die Sitzung bereits in einem verknüpften Git-Worktree läuft, ob Claude ihn unter `.claude/worktrees/` erstellt hat oder Sie ihn mit `git worktree add` anderswo erstellt haben
* Das Arbeitsverzeichnis kein Git-Repository ist und kein [`WorktreeCreate`-Hook](/docs/de/hooks#worktreecreate) konfiguriert ist
* Der Schreibvorgang außerhalb des Arbeitsverzeichnisses liegt

Um die Worktree-Isolation für ein Repository auszuschalten, in dem Git-Worktrees unpraktisch sind, setzen Sie [`worktree.bgIsolation`](/docs/de/settings#worktree-settings) auf `"none"`. Hintergrund-Sitzungen bearbeiten dann Ihre Arbeitskopie direkt, ohne zuerst in einen Worktree zu wechseln. Fügen Sie die Einstellung zur `.claude/settings.json` des Projekts hinzu:

```json theme={null}
{
  "worktree": {
    "bgIsolation": "none"
  }
}
```

Außerhalb eines Git-Repositories schreiben Sitzungen direkt in das Arbeitsverzeichnis und sind nicht voneinander isoliert, daher vermeiden Sie das Versenden paralleler Sitzungen, die dieselben Dateien bearbeiten. Wenn Sie ein anderes Versionskontrollsystem verwenden, konfigurieren Sie einen [`WorktreeCreate`-Hook](/docs/de/worktrees#non-git-version-control) und Claude isoliert Bearbeitungen auf die gleiche Weise wie für Git.

Wenn der Hook in einem Verzeichnis fehlschlägt, das kein Git-Repository ist, überspringt die Sitzung die Isolation für dieses Verzeichnis und bearbeitet das Arbeitsverzeichnis an Ort und Stelle. Innerhalb eines Git-Repositories bleiben Schreibvorgänge blockiert, bis sich die Sitzung isoliert. Vor v2.1.203 konnte eine Hintergrund-Sitzung in diesem Zustand keine Datei bearbeiten: Jeder Schreibvorgang wurde abgelehnt, bis er sich isolierte, und der Hook konnte dieses Verzeichnis niemals isolieren.

Das Löschen einer Sitzung entfernt oder behält den Worktree, den Claude für sie erstellt hat, je nachdem, wie Sie ihn löschen und was der Worktree enthält:

* Das Löschen in der Agenten-Ansicht mit `Ctrl+X` zweimal entfernt den Worktree, einschließlich aller nicht committeten Änderungen, daher committen Sie die Änderungen, die Sie behalten möchten, zuerst.
* Das Löschen aus der Shell mit [`claude rm`](#manage-sessions-from-the-shell) behält einen Worktree mit nicht committeten Änderungen, zusammen mit seiner Sitzungszeile.
* Keiner der beiden Pfade entfernt einen Worktree mit Commits, die nirgendwo gepusht werden: Der Worktree wird [zusammen mit seiner Sitzung beibehalten](#organize-the-list) und die Ausgabe nennt den beibehaltenen Pfad und den Grund.
* Ein Worktree, den Sie selbst erstellt haben und in dem Sie die Sitzung gestartet haben, wird in jedem Fall beibehalten.

Um den Worktree-Pfad einer Sitzung zu finden, schauen Sie sich die Sitzung an oder hängen Sie an und überprüfen Sie ihr Arbeitsverzeichnis.

Ein [Subagent](/docs/de/sub-agents), den die Hintergrund-Sitzung erzeugt, erbt das Arbeitsverzeichnis der Sitzung, sodass seine Dateibearbeitungen im Worktree der Sitzung landen, anstatt in Ihrer Arbeitskopie. Um einem Subagenten stattdessen seinen eigenen separaten Worktree zu geben, setzen Sie [`isolation: worktree`](/docs/de/sub-agents#supported-frontmatter-fields) in seinem Frontmatter oder übergeben Sie `isolation: "worktree"` beim Erzeugen.

Ab v2.1.198 committed eine Hintergrund-Sitzung, die ihre Code-Änderungen in einem Worktree isoliert hat, auch, pusht ihren eigenen Branch und öffnet einen Entwurf-Pull-Request, ohne zu fragen. Das [`#N`-Label](#pull-request-status) wird auf ihrer Zeile angezeigt, wenn der Pull-Request geöffnet wird. Es pusht niemals zu `main` oder `master`, force-pusht oder merged niemals, und es überspringt den Pull-Request, wenn Sie ihm gesagt haben, keinen zu öffnen, oder das Repository hat kein Remote.

Eine Sitzung, die einen Checkout bearbeitet, den sie nicht selbst isoliert hat, fragt immer noch, bevor sie committed oder Branches wechselt. Dies gilt, wenn Isolation auf `"none"` gesetzt ist, wenn der Worktree-Wechsel fehlgeschlagen ist, oder wenn die Sitzung in einem Worktree gestartet wurde, der bereits existierte.

<h3 id="set-the-model">
  Das Modell festlegen
</h3>

Der im Header der Agenten-Ansicht angezeigte Modellname ist der Versand-Standard. Neue Sitzungen, die Sie aus der Eingabe starten, verwenden dieses Modell, das aus der [`model`-Einstellung](/docs/de/settings#available-settings) in Ihren Benutzereinstellungen stammt. Legen Sie es fest, indem Sie ein Modell in der [`/model`-Auswahl](/docs/de/model-config) auswählen, oder bearbeiten Sie die Einstellung direkt.

Um es für die gesamte Agenten-Ansicht-Sitzung zu überschreiben, übergeben Sie `--model` beim Öffnen der Agenten-Ansicht. Siehe [Berechtigungsmodus, Modell und Aufwand](#permission-mode-model-and-effort).

Um den Versand-Standard aus der Agenten-Ansicht zu ändern, geben Sie `/model` gefolgt von einem Modellnamen in die Versand-Eingabe ein und drücken Sie `Enter`. Der Header wird aktualisiert, um dieses Modell mit einem `(session)`-Marker anzuzeigen, und Sitzungen, die Sie danach versenden, verwenden es. Geben Sie `/model default` ein, um die Überschreibung zu löschen und zum Versand-Standard zurückzukehren. Diese Überschreibung gilt für den Rest des aktuellen `claude agents`-Laufs und schreibt nicht in Ihre Einstellungsdatei. Das folgende Beispiel versendet eine Sitzung auf Opus und die nächste auf Sonnet:

```text theme={null}
/model opus
refactor auth
/model sonnet
run the test suite
```

Jede Hintergrund-Sitzung kann auf einem anderen Modell ausgeführt werden. Um es für eine Sitzung zu überschreiben:

* Übergeben Sie in der Shell `--model` mit `claude --bg`.
* Hängen Sie an eine laufende Sitzung an und führen Sie `/model` aus, um zu wechseln: Eine Auswahl aus der Auswahl oder ein eingegebenes `/model <name>` wird als Standard für neue Sitzungen gespeichert, es sei denn, Sie drücken `s` in der Auswahl für einen Wechsel nur für diese Sitzung. Ein Wechsel nur für diese Sitzung bleibt bestehen, wenn die Sitzung neu gestartet wird.
* Versenden Sie einen [Subagenten](/docs/de/sub-agents), dessen Frontmatter ein `model`-Feld setzt.

<h3 id="permission-mode-model-and-effort">
  Berechtigungsmodus, Modell und Aufwand
</h3>

Eine Hintergrund-Sitzung liest ihre [Einstellungen](/docs/de/settings) aus dem Verzeichnis, in dem sie läuft, genauso wie wenn Sie `claude` dort gestartet hätten. Dies umfasst [`env`-Werte](/docs/de/settings#available-settings) in Projekteinstellungen, sodass ein dort gesetzter `ANTHROPIC_MODEL` oder Provider-Variable auf Hintergrund-Sitzungen in diesem Verzeichnis angewendet wird.

Die Cloud-Provider-Auswahl, wie `CLAUDE_CODE_USE_BEDROCK` oder `CLAUDE_CODE_USE_VERTEX`, und `ANTHROPIC_DEFAULT_*_MODEL`-Aliase folgen der Shell, die die Sitzung versendet hat. Wenn Sie eine [`CLAUDE_CODE_EXTRA_BODY`](/docs/de/env-vars)-Anfragekörper-Überschreibung in dieser Shell exportieren, erreicht sie die Sitzung auf die gleiche Weise. Vor v2.1.206 ignorierten Hintergrund-Worker ein Shell-exportiertes `CLAUDE_CODE_EXTRA_BODY`.

Wenn Sie ein Gateway-`ANTHROPIC_BASE_URL` in der versendenden Shell exportieren, erreicht es die Sitzung auch, zusammen mit `ANTHROPIC_CUSTOM_HEADERS`, wenn der Supervisor mit der gleichen Gateway-Umgebung läuft und die Sitzung in dem Verzeichnis läuft, von dem Sie versenden, oder Ihre eigene Sitzung ist, die mit `←` oder `/background` in den Hintergrund verschoben wurde. Das ist der normale Fall, wenn die erste Shell, die die Agenten-Ansicht öffnet oder eine Hintergrund-Sitzung versendet, die Gateway-Shell ist. Das Versenden in ein anderes Verzeichnis mit `@repo` oder `--cwd` trägt das Gateway der Shell nicht mit sich; die [Einstellungen](/docs/de/settings) dieses Projekts liefern den Endpunkt. Siehe [der Supervisor-Prozess](#the-supervisor-process), um zu erfahren, wie Hintergrund-Sitzungen Provider-Einstellungen und Anmeldedaten beziehen.

Der [Berechtigungsmodus](/docs/de/permissions) hängt davon ab, wie Sie die Sitzung gestartet haben. Das Verschieben einer vorhandenen Sitzung mit `/bg` oder `←` behält den aktuellen Berechtigungsmodus bei, sodass eine Sitzung, die Sie zu `acceptEdits` oder `auto` gewechselt haben, nach dem Trennen in diesem Modus bleibt. Das Versenden aus der Agenten-Ansicht-Eingabe oder das Ausführen von `claude --bg` aus der Shell verwendet den `defaultMode` aus den Einstellungen dieses Verzeichnisses oder den `permissionMode` aus dem Frontmatter des versendeten [Subagenten](/docs/de/sub-agents#supported-frontmatter-fields).

Der Berechtigungsmodus, das Modell und der Aufwand, mit dem eine Hintergrund-Sitzung gestartet wurde, sowie die [Konfigurationsflags, die sie trägt](#from-inside-a-session), bleiben alle bestehen, wenn der Supervisor später [die Sitzung stoppt und neu startet](#the-supervisor-process). Eine Sitzung, die Sie mit `claude --bg --dangerously-skip-permissions` oder `claude --bg --permission-mode bypassPermissions` gestartet haben, bleibt nach diesem Neustart in `bypassPermissions`, anstatt auf den `defaultMode` des Verzeichnisses zurückzufallen, und ein Modell oder Aufwand, den Sie während der Sitzung mit `/model` oder `/effort` geändert haben, wird beibehalten.

Ein Aufwand, den die Sitzung aus der [`effortLevel`-Einstellung](/docs/de/settings#available-settings) statt aus `--effort` oder `/effort` genommen hat, ist nicht beim Versenden festgelegt: Jeder Prozess, der für die Sitzung gestartet wird, liest die Einstellung erneut, sodass das Bearbeiten von `effortLevel` in `settings.json` Sitzungen erreicht, die Sie mit `←` oder `/bg` in den Hintergrund verschieben, und ihre späteren Neustarts. Vor v2.1.203 hat das Verschieben einer Sitzung ihren aus Einstellungen stammenden Aufwand aufgezeichnet, als hätten Sie `--effort` übergeben, sodass spätere `effortLevel`-Bearbeitungen ihn niemals erreichten.

Ein Name, den Sie mit [`/rename`](/docs/de/commands) oder `Ctrl+R` festgelegt haben, bleibt auch über diesen Neustart hinweg bestehen, sodass [`claude --resume <name>`](/docs/de/sessions#name-your-sessions) die Sitzung immer noch auflöst. Vor v2.1.202 hat der Neustart die Sitzung auf den Namen zurückgesetzt, mit dem sie versendet wurde, und der neue Name hat nicht mehr aufgelöst.

Um Standardwerte für jede Sitzung festzulegen, die Sie aus der Agenten-Ansicht versenden, übergeben Sie eines der folgenden Elemente beim Öffnen: `--permission-mode`, `--model`, `--effort` oder `--agent`:

```bash theme={null}
claude agents --permission-mode plan --model opus --effort high
```

`--agent` setzt den [Subagenten](/docs/de/sub-agents), der verwendet wird, wenn eine Versand-Eingabeaufforderung keinen benennt, entweder mit `@name` oder als erstes Wort. Es wird standardmäßig auf die [`agent`-Einstellung](/docs/de/settings#available-settings) gesetzt, wenn eine gesetzt ist, ansonsten auf den integrierten Catch-All-`claude`-Agent. Das Benennen eines Subagenten in der Versand-Eingabe überschreibt beide.

`claude agents` akzeptiert auch `--dangerously-skip-permissions` als Kurzform für `--permission-mode bypassPermissions` und `--allow-dangerously-skip-permissions`, um `bypassPermissions` in jedem versendeten Sitzungs-`Shift+Tab`-Zyklus verfügbar zu machen, ohne in diesem Modus zu starten. Beide entsprechen den [Top-Level-CLI-Flags](/docs/de/cli-reference).

Die aktiven Standardwerte werden in der Fußzeile unter der Versand-Eingabe angezeigt.

Ohne diese Flags verwendet die Sitzung den `defaultMode` aus den Einstellungen dieses Verzeichnisses oder den `permissionMode` aus dem Frontmatter des versendeten [Subagenten](/docs/de/sub-agents#supported-frontmatter-fields), und das im Header der Agenten-Ansicht angezeigte Modell.

Die Verwendung von `bypassPermissions` mit `claude --bg --permission-mode` wird abgelehnt, bis Sie den Bypass-Haftungsausschluss akzeptiert haben, indem Sie `claude --dangerously-skip-permissions` einmal interaktiv ausführen, da dieser Modus einer Sitzung, die Sie nicht beobachten, erlaubt, ohne Genehmigung zu handeln. Das Übergeben von `--dangerously-skip-permissions` oder `--permission-mode bypassPermissions` an `claude agents` zeigt denselben Haftungsausschluss an, wenn Sie ihn noch nicht akzeptiert haben, und das Akzeptieren wendet `bypassPermissions` auf die Sitzungen an, die Sie aus der Ansicht starten. Das Übergeben von `--allow-dangerously-skip-permissions` zeigt denselben Haftungsausschluss auch an, und das Akzeptieren macht `bypassPermissions` im `Shift+Tab`-Zyklus dieser Sitzungen verfügbar, ohne sie darin zu starten.

<h3 id="settings-plugins-and-mcp-servers">
  Einstellungen, Plugins und MCP-Server
</h3>

Die Agenten-Ansicht akzeptiert dieselben Konfigurationsflags wie `claude` zum Laden von Einstellungen, Plugins, MCP-Servern und zusätzlichen Verzeichnissen. Jedes Flag gilt für die Agenten-Ansicht selbst und wird an jede Sitzung weitergeleitet, die Sie daraus versenden, sodass ein Plugin oder MCP-Server, den Sie auf diese Weise laden, auch in diesen Sitzungen verfügbar ist.

| Flag                                                                                             | Effekt                                                                                 |
| :----------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------- |
| [`--settings <file-or-json>`](/docs/de/settings)                                                      | Einstellungen für Agenten-Ansicht und versendete Sitzungen überschreiben               |
| [`--add-dir <path>`](/docs/de/permissions#additional-directories-grant-file-access-not-configuration) | Dateizugriff auf ein zusätzliches Verzeichnis gewähren                                 |
| [`--plugin-dir <path>`](/docs/de/plugins)                                                             | Ein Plugin aus einem lokalen Verzeichnis laden                                         |
| [`--mcp-config <file-or-json>`](/docs/de/mcp)                                                         | MCP-Server aus einer Konfigurationsdatei oder JSON-Zeichenkette laden                  |
| `--strict-mcp-config`                                                                            | Nur die MCP-Server aus `--mcp-config` verwenden, andere MCP-Konfigurationen ignorieren |

Wiederholen Sie `--add-dir`, `--plugin-dir` oder `--mcp-config` einmal pro Wert. Die durch Leerzeichen getrennte Form, wie `--add-dir a b c`, wird mit `claude agents` nicht unterstützt.

Das folgende Beispiel öffnet die Agenten-Ansicht mit einer Einstellungsüberschreibung und einem zusätzlichen Verzeichnis:

```bash theme={null}
claude agents --settings ./ci-settings.json --add-dir ../shared-lib
```

<h2 id="manage-sessions-from-the-shell">
  Sitzungen aus der Shell verwalten
</h2>

Jede Hintergrund-Sitzung hat eine kurze ID, die Sie aus der Shell verwenden können. Die ID wird gedruckt, wenn Sie eine Sitzung mit `claude --bg` starten, und die ID jeder Sitzung ist ihr Verzeichnisname unter `~/.claude/jobs/`. Diese Befehle sind nützlich zum Scripting oder wenn Sie die Agenten-Ansicht nicht öffnen möchten.

| Befehl                       | Zweck                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `claude agents`              | Agenten-Ansicht öffnen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `claude agents --cwd <path>` | Agenten-Ansicht auf Sitzungen beschränken, die unter `<path>` gestartet wurden                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `claude agents --json`       | Aktive Sitzungen als JSON-Array drucken und beenden: jede Live-Sitzung sowie Hintergrund-Sitzungen, die noch arbeiten oder blockiert sind, auch wenn ihr Prozess beendet wurde. Fügen Sie `--all` hinzu, um auch abgeschlossene Hintergrund-Sitzungen einzubeziehen. Jeder Eintrag hat `cwd`, `kind` und `startedAt`. Hintergrund-Einträge haben auch `id`, verwendbar mit `claude attach`/`logs`/`stop`, und `state`: einer von `working`, `blocked`, `done`, `failed` oder `stopped`. `pid` und `status` sind nur vorhanden, während der Prozess aktiv ist, plus `waitingFor`, wenn status `waiting` ist, was angibt, worauf die Sitzung blockiert ist, z. B. `permission prompt` oder `input needed`; `sessionId` und `name` erscheinen, wenn gesetzt. Eine interaktive Sitzung, die Sie nie benannt haben, trägt einen Standard-`name`, der aus dem Namen des Arbeitsverzeichnisses plus einem zweistelligen Suffix wie `my-app-3f` erstellt wird. Mit `--cwd <path>` kombinieren zum Filtern |
| `claude attach <id>`         | An eine Sitzung in diesem Terminal anhängen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `claude logs <id>`           | Neueste Ausgabe der Sitzung drucken                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `claude stop <id>`           | Eine Sitzung stoppen. Akzeptiert auch `claude kill`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `claude respawn <id>`        | Eine Sitzung neu starten (laufend oder gestoppt) mit ihrem Gespräch intakt, z. B. um eine aktualisierte Claude Code-Binärdatei zu verwenden                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `claude respawn --all`       | Jede laufende Sitzung neu starten, z. B. um alle Sitzungen auf einmal auf eine aktualisierte Claude Code-Binärdatei zu verschieben                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `claude rm <id>`             | Eine Sitzung aus der Liste entfernen. Entfernt einen Worktree, den Claude für die Sitzung erstellt hat, wenn er keine nicht committeten Änderungen hat und keine Commits, die nicht irgendwo gepusht wurden; andernfalls wird die Sitzung auch beibehalten, und der Befehl druckt den Worktree-Pfad und den Grund aus, damit Sie ihn beheben und `claude rm` erneut ausführen können. Lässt einen Worktree, den Sie selbst erstellt haben, an Ort und Stelle. Das Gesprächstranskript bleibt auf Ihrem lokalen Computer und bleibt über `claude --resume` verfügbar                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `claude daemon status`       | Den Status des [Supervisors](#the-supervisor-process), die Version, das Socket-Verzeichnis und die Anzahl der Worker drucken                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `claude daemon stop --any`   | Den Supervisor-Prozess und die Hintergrund-Sitzungen, die er hostet, stoppen. Übergeben Sie `--keep-workers`, um Hintergrund-Sitzungen laufen zu lassen, damit der nächste Supervisor sich mit ihnen verbinden kann. Der nächste `claude agents` oder `claude --bg` startet einen neuen Supervisor                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |

<h2 id="how-background-sessions-are-hosted">
  Wie Hintergrund-Sitzungen gehostet werden
</h2>

Jede Sitzung, die in der Agenten-Ansicht aufgelistet ist, wird als Hintergrund-Sitzung betrachtet, unabhängig davon, ob Sie derzeit daran angehängt sind oder nicht. Im Gegensatz dazu ist eine Sitzung, die durch direktes Ausführen von `claude` gestartet wird, an dieses Terminal gebunden und endet, wenn es geschlossen wird, es sei denn, Sie [senden sie in den Hintergrund](#from-inside-a-session).

<h3 id="the-supervisor-process">
  Der Supervisor-Prozess
</h3>

Hintergrund-Sitzungen werden von einem Supervisor-Prozess pro Benutzer gehostet, getrennt von Ihrem Terminal und von der Agenten-Ansicht. Der Supervisor startet automatisch, wenn Sie zum ersten Mal eine Sitzung in den Hintergrund verschieben oder die Agenten-Ansicht öffnen, und Sie verwalten ihn nicht direkt.

Wenn ein Update die Binärdatei ersetzt oder entfernt hat, von der ein laufender Claude Code-Prozess gestartet wurde, startet dieser Prozess den Supervisor von einer anderen installierten Kopie, wie dem installierten `claude`-Launcher oder der neuesten Version auf der Festplatte.

Der Supervisor hält einen vorgewärmten Worker-Prozess bereit, damit ein Dispatch aus der Agenten-Ansicht oder `claude --bg` ohne die Verzögerung eines Cold-Starts beginnt. Wenn Sie dispatchen, weist der Supervisor den vorgewärmten Worker Ihrer Sitzung zu, wendet das Verzeichnis, die Einstellungen und die Anmeldedaten dieser Sitzung darauf an und startet dann einen Ersatz für den nächsten Dispatch. Wenn kein fehlerfreier vorgewärmter Worker verfügbar ist, startet der Supervisor stattdessen einen neuen Prozess.

Der Supervisor und seine Sitzungen authentifizieren sich mit denselben Anmeldedaten wie Ihre interaktiven Sitzungen und stellen keine zusätzlichen Netzwerkverbindungen über die Modell-API hinaus her. Anbieter-Auswahlvariablen wie `CLAUDE_CODE_USE_BEDROCK` und `ANTHROPIC_DEFAULT_*_MODEL`-Aliase werden aus der Shell gelesen, die jede Sitzung dispatcht, und werden auf ihren Worker angewendet.

Der `PATH` der Dispatch-Shell wird auf den Worker auf die gleiche Weise angewendet, daher finden Shell-Befehle, die die Sitzung ausführt, die gleichen Tools wie Ihr Terminal. Vor v2.1.203 behielt eine Hintergrund-Sitzung den `PATH` der Shell, die den Supervisor zuerst gestartet hat, daher konnten Tools, die Sie Ihrem `PATH` hinzugefügt haben, seitdem fehlen, am häufigsten unter Windows.

Eine Hintergrund-Sitzung erbt keine Gateway-Endpunkt-Variablen wie `ANTHROPIC_BASE_URL` oder die entsprechenden Amazon Bedrock-, Google Cloud Agent Platform- und Microsoft Foundry-Basis-URL-Variablen aus der Shell, die den Supervisor gestartet hat. Ohne ein Gateway, das in der Shell exportiert ist, von der Sie dispatchen, verwendet die Sitzung Ihre gespeicherten Anmeldedaten und alle `env`-Werte im [settings](/docs/de/settings) des Projektverzeichnisses. Um jede Sitzung in einem Projekt auf ein [LLM-Gateway](/docs/de/llm-gateway) zu verweisen, setzen Sie `ANTHROPIC_BASE_URL` im `env`-Block der `.claude/settings.json` dieses Projekts.

Wenn Sie ein Gateway `ANTHROPIC_BASE_URL` in der Shell exportieren, von der Sie dispatchen, erreicht es den Worker dieser Sitzung. `ANTHROPIC_CUSTOM_HEADERS` und die Anmeldedaten, die neben ihnen exportiert sind, werden damit weitergeleitet. Dies geschieht, wenn der Supervisor aus einer Umgebung mit dem gleichen Gateway gestartet wurde. Der Supervisor erfasst seine Umgebung aus der ersten Shell, die die Agenten-Ansicht öffnet oder eine Hintergrund-Sitzung dispatcht, daher gibt das Starten aus der Gateway-Shell ihm diese Umgebung. Die Weiterleitung gilt auch nur für Sitzungen, die in das Verzeichnis dispatcht werden, von dem aus Sie dispatchen, oder die mit `←` oder `/background` in den Hintergrund verschoben werden: Das Dispatchen in ein anderes Verzeichnis mit `@repo` oder `--cwd` trägt das Gateway der Shell nicht mit sich, und der `settings.json` `env`-Block dieses Projekts liefert stattdessen den Endpunkt. Wenn die Umgebung des Supervisors ein anderes Gateway oder keines trägt, behält der Worker Ihre gespeicherten Anmeldedaten gegen den Standard-Endpunkt, anstatt die Anmeldedaten einer Umgebung mit dem Endpunkt einer anderen zu vermischen. Vor v2.1.203 wurde der `ANTHROPIC_BASE_URL` der Dispatch-Shell gelöscht, während der `ANTHROPIC_API_KEY`, der neben ihm exportiert wurde, beibehalten wurde, daher wurde der Schlüssel des Gateways an den Standard-Endpunkt gesendet und jede Anfrage schlug mit einem 401 fehl.

Der weitergeleitete Endpunkt gilt nur für diesen Live-Prozess und wird niemals auf die Festplatte geschrieben. Wenn der Supervisor eine untätige Sitzung stoppt und sie später neu startet, liest der neu gestartete Prozess seinen Endpunkt erneut aus Ihren Einstellungen: Mit einem Gateway `ANTHROPIC_AUTH_TOKEN` fällt er auf Ihre gespeicherten Anmeldedaten zurück, und mit einem von einem Gateway ausgegebenen `ANTHROPIC_API_KEY` kann die Authentifizierung fehlschlagen, bis das Gateway in den Einstellungen gesetzt wird.

Jede Hintergrund-Sitzung ist ihr eigener Claude Code-Prozess, der vom Supervisor verwaltet wird, anstatt an Ihr Terminal gebunden zu sein. Eine Sitzung, die aktiv funktioniert, auf Ihre Eingabe wartet oder ein Terminal angehängt hat, behält ihren Prozess am Laufen. Ein laufender Hintergrund-Shell-Befehl, Subagent, dynamischer Workflow oder Monitor zählt als aktive Arbeit, daher hält ein langfristiger Prozess wie ein Dev-Server die Sitzung am Leben.

Sobald eine Sitzung fertig ist und etwa eine Stunde lang unverbunden sitzt, stoppt der Supervisor ihren Prozess, um Ressourcen freizugeben. Eine Sitzung, die Sie mit `Ctrl+T` [angeheftet](#organize-the-list) haben, ist ausgenommen und behält ihren Prozess im Leerlauf bei. Das Transkript und der Status bleiben auf der Festplatte, und das nächste Mal, wenn Sie anhängen, Vorschau anzeigen oder antworten, startet der Supervisor einen frischen Prozess von dort, wo er aufgehört hat. Wenn jede Sitzung fertig ist und kein Terminal verbunden ist, beendet sich der Supervisor selbst und startet erneut, wenn Sie das nächste Mal eine Sitzung benötigen.

Hintergrund-Arbeit, die die Sitzung selbst auf der obersten Ebene gestartet hat, wird übergeben, wenn ihr Prozess gestoppt, neu gestartet oder aktualisiert wird, einschließlich unter Windows. Der nächste Prozess, der für diese Sitzung gestartet wird, greift die Arbeit wieder auf:

* Ein Hintergrund-Shell-Befehl, der inzwischen fertig ist, wird als abgeschlossen mit seiner Ausgabe gemeldet
* Ein dynamischer Workflow wird von dort fortgesetzt, wo er aufgehört hat
* Ein [Hintergrund-Subagent](/docs/de/sub-agents#run-subagents-in-foreground-or-background) wird von seinem eigenen Transkript fortgesetzt

Ab v2.1.198 deckt die Übergabe alle drei ab. Vor v2.1.198 deckte sie nur Shell-Befehle und Workflows ab, daher wurde ein Hintergrund-Subagent mit dem Prozess gestoppt und beim nächsten Aufwachen als fehlgeschlagen gemeldet.

Arbeit, deren Status nur innerhalb des Prozesses selbst lebt, stoppt damit, anstatt übergeben zu werden. Das sind Shell-Befehle, die ein Subagent gestartet hat, die der fortgesetzte Subagent erneut starten kann, und laufende [Monitore](/docs/de/tools-reference#monitor-tool), deren Ereignisstrom nicht auf einen anderen Prozess verschoben werden kann.

Das Löschen der Sitzung stoppt alles, das sie übergeben hat. Um alle Hintergrund-Arbeiten der Sitzung mit dem Prozess zu stoppen, anstatt sie zu übergeben, setzen Sie die Umgebungsvariable [`CLAUDE_CODE_DISABLE_BG_EXIT_HANDOFF`](/docs/de/env-vars#variables) auf `1`.

Ein neu gestarteter Prozess findet die Konversation einer Sitzung, die [während einer Aufgabe in einen Worktree verschoben wurde](#how-file-edits-are-isolated): Wenn sich das Transkript nicht dort befindet, wo die Sitzung gestartet wurde, sucht Claude Code auch unter den registrierten Worktrees des Repositorys. Vor v2.1.207 konnte das erneute Öffnen dieser Sitzung aus der Agenten-Ansicht, nachdem ihr Prozess gestoppt worden war, eine leere Konversation mit nur ihrem ursprünglichen Prompt anzeigen, wobei das Transkript noch intakt auf der Festplatte war; das erneute Öffnen der Sitzung auf v2.1.207 oder später stellt sie wieder her.

Wenn eine neu gestartete Sitzung zurückkommt und nur ihren ursprünglichen Prompt zeigt, weil Claude Code ihr Transkript als leer missverstanden hat, wird das Gesprächstranskript stattdessen mit einem `.orphaned-`-Suffix umbenannt, anstatt gelöscht zu werden, sodass es auf Ihrem Computer bleibt.

Eine leere Zeile, die von Drücken von `←` übrig bleibt und der nie eine Eingabeaufforderung gegeben wurde, wird nach etwa fünf Minuten vollständig entfernt, damit sich die Liste von selbst löscht. Sitzungen, die mit `claude --bg` gestartet wurden, und Sitzungen, die auf eine Setup-Eingabeaufforderung wie einen Vertrauensdialog warten, werden auf diese Weise nicht entfernt.

Wenn der Host wenig Speicher hat, stoppt der Supervisor zuerst untätige nicht angeheftete Sitzungen und stoppt untätige angeheftete Sitzungen nur, wenn dies nichts freigegeben hat.

Der Supervisor beobachtet die installierte Claude Code-Binärdatei auf der Festplatte und startet in die neue Version neu, nachdem der reguläre [Auto-Updater](/docs/de/setup#auto-updates) sie ersetzt. Dies ist eine lokale Dateiüberwachung, keine Netzwerkprüfung. Hintergrund-Sitzungen sind abgelöste Prozesse, daher laufen sie während des Neustarts weiter und der neue Supervisor verbindet sich wieder mit ihnen. Eine untätige angeheftete Sitzung wird auch an Ort und Stelle in die neue Version neu gestartet, damit sie das Update aufgreift, ohne dass Sie sie erneut anhängen müssen.

Sobald der neue Supervisor die Kontrolle übernimmt, startet er auch die verbleibenden untätigen Sitzungen auf die neue Version neu, einige auf einmal im Hintergrund, nach einer kurzen Verzögerung, die es Terminals ermöglicht, die über den Neustart hinweg angehängt sind, sich zuerst erneut zu verbinden. Eine Sitzung, die funktioniert, auf Ihre Eingabe wartet oder ein Terminal angehängt hat, wird nicht unterbrochen; sie wird auf die neue Version verschoben, wenn ihr Prozess das nächste Mal neu gestartet wird. Vor v2.1.206 verschob der Supervisor nur wenige untätige Sitzungen pro Minute auf eine neue Version, daher konnten Sitzungen die alte Version eine Weile nach einem Update weiter ausführen.

Diese Neustarts verschieben eine Sitzung nur auf eine neuere Version. Ein Supervisor, der eine ältere Claude Code-Version ausführt als die, mit der der Prozess einer Sitzung gestartet wurde, lässt diesen Prozess in Ruhe; die Sitzung führt die neuere Version weiter aus, bis ein neuerer Supervisor die Kontrolle übernimmt.

Das Ausführen von `claude attach`, während der Supervisor eine Sitzung neu startet, sei es für ein Update, einen Stall oder eine Migration, wartet auf den Ersatzprozess, anstatt fehlzuschlagen. Eine Statuszeile wie `Agent is updating to the new Claude Code…` benennt, worauf es wartet, und zählt die verstrichenen Sekunden, und der Befehl verbindet sich, sobald die Sitzung bereit ist. Nach etwa 60 Sekunden stoppt es das Warten und meldet einen Fehler. Vor v2.1.205 stoppte `claude attach` das Wiederholen nach wenigen Sekunden und druckte einen Fehler, während die Sitzung noch neu gestartet wurde.

<h3 id="where-state-is-stored">
  Wo der Status gespeichert ist
</h3>

Der Sitzungsstatus wird unter Ihrem Claude Code-Konfigurationsverzeichnis gespeichert. Wenn Sie [`CLAUDE_CONFIG_DIR`](/docs/de/env-vars) setzen, verwendet der Supervisor stattdessen dieses Verzeichnis und läuft als separate Instanz mit ihren eigenen Sitzungen.

| Pfad                             | Inhalt                                                                                                                             |
| :------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| `~/.claude/daemon.log`           | Supervisor-Protokoll                                                                                                               |
| `~/.claude/daemon/roster.json`   | Liste der laufenden Hintergrund-Sitzungen, verwendet zum Wiederherstellen der Verbindung nach einem Neustart                       |
| `~/.claude/jobs/<id>/state.json` | Pro-Sitzungs-Status in der Agenten-Ansicht angezeigt                                                                               |
| `~/.claude/jobs/<id>/tmp/`       | Pro-Sitzungs-Scratch-Verzeichnis. Schreibvorgänge hier erfordern keine Berechtigung. Wird entfernt, wenn die Sitzung gelöscht wird |

Jede Hintergrund-Sitzung hat die Umgebungsvariable `CLAUDE_JOB_DIR` auf ihr `~/.claude/jobs/<id>`-Verzeichnis gesetzt, daher können Shell-Befehle, die die Sitzung ausführt, temporäre Dateien in `$CLAUDE_JOB_DIR/tmp` schreiben, ohne mit parallelen Sitzungen zu kollidieren.

Um diesen Status zu überprüfen, ohne die Dateien direkt zu lesen, führen Sie `claude daemon status` aus. Es meldet, ob der Supervisor erreichbar ist, seine Prozess-ID und Version, das Socket-Verzeichnis und wie viele Hintergrund-Sitzungen aktiv sind.

Der Befehl warnt auch, wenn der laufende Supervisor eine andere Version hat als der `claude`, den Sie aufgerufen haben, was nach einem Update geschieht, das der Supervisor noch nicht neu gestartet hat. Die Warnung zeigt beide Versionen an und teilt Ihnen mit, dass Sie `claude daemon stop --any` ausführen sollen, um die neue Version zu übernehmen. Wenn Claude Code als Betriebssystem-Dienst installiert ist, ist der vorgeschlagene Befehl `claude daemon stop` ohne das Flag.

Sitzungen bleiben bei diesem Versionskonflikt intakt: Eine ältere Claude Code-Version, die die `state.json` einer Sitzung aktualisiert, behält Felder, die sie nicht erkennt, und behält die Sitzung in der Liste. Die Sitzungsliste in `roster.json` folgt derselben Regel: Eine ältere Version, die sie umschreibt, behält Felder, die eine neuere Version geschrieben hat, daher bleiben Sitzungen, die von der neueren Version gestartet wurden, erreichbar und akzeptieren weiterhin Eingaben, nachdem der Supervisor neu gestartet wird. Vor v2.1.200 konnten ältere Versionen diese Felder beim Umschreiben löschen.

Unter Windows zeigt `claude daemon status` den zugrunde liegenden Dateifehler an, wenn die Pipe-Schlüsseldatei des Daemons gesperrt oder nicht lesbar ist, anstatt einen generischen Verbindungsfehler zu melden.

<h3 id="turn-off-agent-view">
  Agenten-Ansicht ausschalten
</h3>

Um Hintergrund-Agenten und die Agenten-Ansicht vollständig auszuschalten, setzen Sie die Einstellung `disableAgentView` [setting](/docs/de/settings) auf `true` oder setzen Sie die Umgebungsvariable `CLAUDE_CODE_DISABLE_AGENT_VIEW`. Administratoren können dies durch [verwaltete Einstellungen](/docs/de/permissions#managed-settings) erzwingen.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="claude-agents-lists-subagents-instead-of-opening-agent-view">
  `claude agents` listet Subagenten auf, anstatt die Agenten-Ansicht zu öffnen
</h3>

Wenn `claude agents` eine Anzahl gefolgt von Ihren konfigurierten Subagenten ausgibt und dann beendet wird, ist die Agenten-Ansicht in Ihrer Umgebung nicht verfügbar. Führen Sie `claude update` aus, um die neueste Version zu installieren.

Wenn die Agenten-Ansicht nach dem Update immer noch nicht geöffnet wird, überprüfen Sie, ob sie durch eine Einstellung oder Umgebungsvariable [deaktiviert](#turn-off-agent-view) wurde.

<h3 id="agent-view-opens-with-no-sessions">
  Agenten-Ansicht öffnet sich ohne Sitzungen
</h3>

Bevor Sie Ihre erste Sitzung versenden, zeigt die Agenten-Ansicht die leeren Abschnittskopfzeilen mit einer Beschreibung unter jedem sowie eine einzeilige Erklärung über der Eingabe anstelle der Sitzungsliste. Geben Sie eine Eingabeaufforderung in die Eingabe am unteren Rand ein und drücken Sie `Enter`, um Ihre erste Sitzung zu versenden.

<h3 id="backgrounding-shows-a-background-this-session-dialog">
  Backgrounding zeigt einen `Background this session?`-Dialog
</h3>

Wenn das Drücken von `←` zum Hintergrund der aktuellen Sitzung einen `Background this session?`-Dialog zeigt, hat die Sitzung laufende Arbeit, die nicht zur Hintergrund-Sitzung übertragen werden kann, wie z. B. ein laufender [Monitor](/docs/de/tools-reference#monitor-tool), und Claude Code wird sie nicht stillschweigend stoppen. Das Dialog benennt die Arbeit, die gestoppt wird, und zählt separat die Aufgaben, die übertragen werden. Führen Sie `/tasks` aus, um zu sehen, was läuft, dann bestätigen Sie, um trotzdem in den Hintergrund zu gehen, oder wählen Sie `Stay`, um die Arbeit zuerst fertig zu stellen. Siehe [Aus einer Sitzung heraus](#from-inside-a-session), um zu sehen, welche Aufgabentypen übertragen werden und welche gestoppt werden.

<h3 id="prompt-rejected-as-too-short">
  Eingabeaufforderung als zu kurz abgelehnt
</h3>

Die Versand-Eingabe erwartet eine Aufgabenbeschreibung, keine Gesprächseröffnung. Eine Eingabeaufforderung, die kürzer als vier Zeichen ist, wird mit einem `Too short`-Hinweis abgelehnt, damit ein versehentlicher Tastendruck keine Sitzung startet. Beschreiben Sie, was die Sitzung tun soll, z. B. `investigate the flaky checkout test`.

<h3 id="sessions-show-as-failed-after-shutdown">
  Sitzungen werden nach dem Herunterfahren als fehlgeschlagen angezeigt
</h3>

Das Herunterfahren oder Neustarten Ihres Computers stoppt laufende Hintergrund-Sitzungen, sodass sie beim nächsten Öffnen der Agenten-Ansicht als fehlgeschlagen angezeigt werden. Hängen Sie sich an, zeigen Sie Vorschau an oder antworten Sie auf eine beliebige Sitzung und die Sitzung startet von dort neu, wo sie aufgehört hat.

Der Ruhezustand allein verursacht dies nicht. Sitzungen werden über den Ruhezustand hinweg beibehalten und der Supervisor verbindet sich beim Aufwachen wieder mit ihnen.

<h3 id="opening-a-session-says-the-conversation-is-already-open">
  Öffnen einer Sitzung besagt, dass die Konversation bereits offen ist
</h3>

Das Öffnen einer gestoppten Zeile, deren Konversation auch von einem anderen laufenden nicht-interaktiven Claude Code-Prozess geöffnet wird, z. B. ein Hintergrund-Worker für dieselbe Konversation, der noch heruntergefahren wird, zeigt `This conversation is already open in another running Claude session` anstatt den Prozess der Zeile zu starten, da zwei Prozesse nicht in dasselbe Transkript schreiben können. Antworten Sie in der Sitzung, die die Konversation bereits offen hat, oder beenden Sie sie und öffnen Sie die Zeile erneut. Eine Antwort, die Sie mit dem abgelehnten Versuch eingegeben haben, geht nicht verloren; sie wird gesendet, wenn die Sitzung das nächste Mal startet.

Vor v2.1.203 startete dieser Zustand trotzdem einen zweiten Prozess. Dieser Prozess beendete sich mit einem `currently running as a background agent`-Fehler und die Zeile zeigte als fehlgeschlagen.

<h3 id="a-session-fails-before-starting-with-a-possibly-low-memory-note">
  Eine Sitzung schlägt vor dem Start mit einer `possibly low memory`-Notiz fehl
</h3>

Ab v2.1.199 zeigt die Zeile den Exit und fügt `possibly low memory — free some up and retry` hinzu, wenn der Prozess einer Hintergrund-Sitzung beendet wird, bevor er fertig startet und der Host wenig Speicher hat. Frühere Versionen zeigten nur den bloßen Exit-Grund für diesen Fehler.

Die Notiz ist eine Hypothese, keine bestätigte Ursache. Claude Code fügt sie nur hinzu, wenn der Prozess stillschweigend beendet wurde, ohne einen Fehler zu schreiben und ohne durch ein Signal gestoppt zu werden, und der Host meldete zu diesem Zeitpunkt wenig Speicher. Wenn der Prozess vor dem Exit einen Fehler geschrieben hat, zeigt die Zeile stattdessen diesen Fehler.

Geben Sie Speicher auf dem Computer frei, dann hängen Sie sich an, zeigen Sie Vorschau an oder antworten Sie auf die Zeile und der Supervisor startet einen neuen Prozess für die Sitzung. Wenn der Speicher niedrig bleibt, stoppt der Supervisor auch [untätige Sitzungen](#the-supervisor-process) von selbst, um Ressourcen freizugeben.

<h3 id="agent-view-says-the-background-service-did-not-respond">
  Agenten-Ansicht sagt, dass der Hintergrunddienst nicht geantwortet hat
</h3>

Wenn das Anhängen, Anschauen oder `claude logs` meldet, dass der Hintergrunddienst nicht geantwortet hat, ist der Supervisor-Prozess wahrscheinlich steckengeblieben. Stoppen Sie ihn und lassen Sie den nächsten `claude agents` einen neuen starten. Um Ihre Hintergrund-Sitzungen während des Neustarts am Laufen zu halten, übergeben Sie `--keep-workers`:

```bash theme={null}
claude daemon stop --any --keep-workers
```

Der neue Supervisor verbindet sich wieder mit den laufenden Sitzungen. Ohne `--keep-workers` beendet der Befehl auch die Hintergrund-Sitzungen. Das Flag `--any` bestätigt, dass Sie einen Supervisor stoppen möchten, der bei Bedarf gestartet wurde, anstatt als installierter Dienst, was die Standardeinstellung ist.

Ein Supervisor, der startet, aber keine Verbindungen akzeptieren kann, beendet sich selbst und gibt seine Sperre frei, sodass der nächste `claude agents` ohne diesen manuellen Stop einen neuen startet. Die obigen Schritte gelten, wenn ein laufender Supervisor steckenbleibt.

Unter Windows, wenn der Supervisor nicht auf die Stoppanforderung antwortet, gibt der Befehl seine Prozess-ID aus. Beenden Sie diesen Prozess mit `taskkill /PID <pid>`, um die Wiederherstellung abzuschließen. Hintergrund-Sitzungen werden immer noch beibehalten, wenn Sie `--keep-workers` übergeben haben.

<h3 id="dispatch-fails-with-could-not-resolve-authentication-method">
  Versand schlägt mit `Could not resolve authentication method` fehl
</h3>

Wenn ein Hintergrund-Versand mit `Could not resolve authentication method` fehlschlägt, während interaktive Sitzungen sich normal authentifizieren, hat der Worker, der den Versand erhalten hat, keine Anmeldedaten aufgegriffen. Der Supervisor stellt einen frischen Snapshot der Anmeldedaten bereit, wenn er einen [vorgewärmten Worker](#the-supervisor-process) zuweist, daher bedeutet dieser Fehler, dass dem Supervisor-Prozess selbst keine gespeicherten Anmeldedaten zur Verfügung standen. Bestätigen Sie, dass Sie `/login` ausgeführt oder einen API-Schlüssel konfiguriert haben, dann stoppen Sie den Supervisor:

```bash theme={null}
claude daemon stop --any --keep-workers
```

Der nächste `claude agents` oder `claude --bg` startet einen neuen Supervisor, der Ihre gespeicherten Anmeldedaten liest. Wenn Sie sich mit einer Umgebungsvariable wie `ANTHROPIC_API_KEY` authentifizieren, anstatt `/login` zu verwenden, führen Sie diesen nächsten Befehl aus einer Shell aus, in der die Variable gesetzt ist.

Siehe die [Fehlerreferenz](/docs/de/errors#could-not-resolve-authentication-method) für die vollständige Liste der Ursachen und Behebungen.

<h3 id="background-sessions-can’t-read-desktop-documents-or-downloads-on-macos">
  Hintergrund-Sitzungen können Desktop, Dokumente oder Downloads auf macOS nicht lesen
</h3>

Unter macOS wird der Hintergrund-Sitzungs-Host als eigener Prozess ausgeführt und fordert Zugriff auf geschützte Ordner separat von Ihrem Terminal an. Wenn eine Hintergrund-Sitzung `Operation not permitted` meldet, wenn sie `~/Desktop`, `~/Documents`, `~/Downloads` oder einen anderen geschützten Ort liest, gewähren Sie Zugriff in den Systemeinstellungen unter Datenschutz & Sicherheit > Dateien und Ordner, oder aktivieren Sie Vollständigen Festplattenzugriff für den Eintrag.

Mit dem nativen Installer wird der Eintrag als Claude Code angezeigt und die Berechtigung bleibt über Updates hinweg erhalten. Bei anderen Installationsmethoden wie Homebrew oder npm zeigt der Eintrag den Binärpfad an und muss möglicherweise nach dem Update erneut gewährt werden.

<h3 id="background-sessions-can’t-reach-local-network-hosts-on-macos">
  Hintergrund-Sitzungen können auf macOS keine lokalen Netzwerk-Hosts erreichen
</h3>

Unter macOS 15 und später blockiert das System einen Prozess daran, Geräte in Ihrem lokalen Netzwerk zu erreichen, bis Sie die Berechtigung für lokales Netzwerk gewähren. Vor v2.1.198 forderte der Hintergrund-Sitzungs-Host diese Berechtigung nie an, daher schlugen Befehle, die auf eine LAN-Adresse abzielten, mit `connect: no route to host` fehl, obwohl derselbe Befehl in einem Vordergrund-Terminal funktionierte. Ab v2.1.198 löst der erste Befehl in einer Hintergrund-Sitzung, der sich mit einer lokalen Netzwerk-Adresse verbindet, die macOS-Berechtigung für lokales Netzwerk für Claude Code aus. Gewähren Sie sie einmal und diese Befehle erreichen LAN-Hosts auf die gleiche Weise wie in einem Vordergrund-Terminal.

<h3 id="a-session-is-slow-to-respond-after-attaching">
  Eine Sitzung reagiert langsam nach dem Anhängen
</h3>

Sobald eine Sitzung fertig ist und etwa eine Stunde lang unverbunden sitzt, stoppt der Supervisor seinen Prozess, um Ressourcen freizugeben. Das Anhängen startet einen frischen Prozess von dort, wo er aufgehört hat, und wechselt sofort zur Sitzung, während der Prozess neu startet. Sitzungen, die funktionieren, auf Sie warten oder [angeheftet](#organize-the-list) sind, werden auf diese Weise nicht gestoppt, daher heften Sie eine Sitzung mit `Ctrl+T` an, um sie reaktionsschnell zu halten.

Während der Prozess startet, wird der letzte Bildschirm der Sitzungstranskription mit einer `Session is starting`-Notiz darunter angezeigt, und die Live-Sitzung ersetzt ihn, sobald sie bereit ist.

<h3 id="claude/worktrees/-is-filling-up">
  `.claude/worktrees/` füllt sich auf
</h3>

Das Löschen einer Sitzung in der Agenten-Ansicht entfernt den Worktree, den Claude dafür erstellt hat, und ein Worktree, der nicht sicher entfernt werden kann, [behält seine Sitzungszeile](#organize-the-list), damit er nicht verwaist wird. `claude rm` behält einen Worktree mit nicht committeten Änderungen und gibt seinen Pfad aus. Listen Sie verbleibende Einträge mit `git worktree list` im Projektverzeichnis auf und entfernen Sie jeden mit `git worktree remove <path>`. Siehe [Worktrees bereinigen](/docs/de/worktrees#clean-up-worktrees).

<h2 id="limitations">
  Einschränkungen
</h2>

Die Agenten-Ansicht ist eine Forschungsvorschau mit den folgenden Einschränkungen:

* **Ratenlimits gelten**: Hintergrund-Sitzungen verbrauchen Ihre Abonnementnutzung genauso wie interaktive Sitzungen, daher verwendet das Ausführen von zehn Agenten parallel die Quote ungefähr zehnmal schneller.
* **Sitzungen sind lokal**: Hintergrund-Sitzungen laufen auf Ihrem Computer. Sie werden über den Ruhezustand hinweg beibehalten, stoppen aber, wenn der Computer heruntergefahren wird.
* **Von Claude erstellte Worktrees werden mit der Sitzung in der Agenten-Ansicht gelöscht**: Führen Sie Änderungen zusammen, bevor Sie eine Sitzung löschen, die Dateien in ihrem eigenen Worktree bearbeitet hat. Ein Worktree mit Commits, die nirgendwo gepusht werden, wird zusammen mit der Sitzung beibehalten. `claude rm` behält auch einen Worktree bei, der nicht committete Änderungen hat, und ein Worktree, den Sie selbst erstellt haben, wird an Ort und Stelle belassen.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

Weitere Möglichkeiten zum parallelen Ausführen von Claude finden Sie unter:

* [Agenten parallel ausführen](/docs/de/agents): Vergleichen Sie die Agenten-Ansicht mit Subagenten, Agenten-Teams und Worktrees
* [Agenten-Teams](/docs/de/agent-teams): Koordinieren Sie mehrere Sitzungen, die sich gegenseitig Nachrichten senden
* [Claude Code im Web](/docs/de/claude-code-on-the-web): Führen Sie Sitzungen in einer verwalteten Cloud-Umgebung aus, anstatt lokal

<h2 id="version-history">
  Versionsverlauf
</h2>

Die Agenten-Ansicht hat sich während der Forschungsvorschau schnell entwickelt. Wenn Sie eine ältere Claude Code-Version verwenden, kann sich einiges auf dieser Seite unterscheiden; insbesondere lehnt `claude agents` Flags ab, die es noch nicht unterstützt, mit einem `unknown option`-Fehler. Die folgende Tabelle listet auf, wann jedes Flag und Verhalten hinzugefügt wurde.

| Version  | Änderung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| v2.1.208 | Das Anhängen an eine Sitzung, deren Prozess gestoppt wurde, zeigt den letzten Bildschirmvoll ihres Transkripts an, während der Prozess startet, anstatt nur eine `Session is starting`-Notiz. Eine Antwort, die nicht zugestellt werden kann, weil der Hintergrunddienst nicht erreichbar ist oder das Senden fehlschlägt, wird gespeichert und als nächste Eingabeaufforderung der Sitzung gesendet, wenn ihr Prozess wieder startet; vor dieser Version ging eine Antwort, die verloren ging, während der Hintergrunddienst nicht erreichbar war, verloren. Ein Prozess, dessen eigene Binärdatei durch ein Update ersetzt wurde, kann den Supervisor immer noch starten, vom installierten `claude`-Launcher oder der neuesten Version auf der Festplatte, anstatt fehlzuschlagen, bis Claude Code neu gestartet wurde. Ein Supervisor, der eine ältere Version ausführt, startet eine untätige Sitzung, die von einer neueren Version gestartet wurde, niemals auf seiner eigenen älteren Binärdatei neu. Das Löschen einer Sitzung entfernt ihren Worktree auch nachdem die Sitzung den Worktree auf einen anderen Branch verschoben hat, und behält den Worktree zusammen mit der Sitzungszeile, wenn der Worktree Commits hat, die nirgendwo gepusht werden, oder eine andere Sitzung ihn beansprucht, anstatt die Commits zu zerstören oder den Worktree verwaist zu lassen. `/install-github-app` und die `/mcp`-Einstellungsliste und ihre Authentifizierungsaktionen werden in einer Hintergrund-Sitzung mit einer Nachricht abgelehnt, die die Alternative benennt; in v2.1.208 nur wurde die `/model`-Auswahl auf die gleiche Weise abgelehnt und ein eingegebenes `/model <name>` schaltete nur diese Sitzung um, anstatt auch Ihr Standard-Modell zu speichern. |
| v2.1.207 | Das Vorschau-Panel öffnet sich mit dem Satz, den die Zeile abschneidet, wie z. B. die genaue Frage für eine Sitzung, die auf Sie wartet, und zeigt, wie lange eine blockierte Sitzung wartet, als eine einzelne `waiting 3m`-Zeile, anstatt denselben Zeitstempel dem Statussatz und der Frage voranzustellen. Das Einfügen desselben Textes erneut in die Versand-Eingabe erweitert den zusammengeklappten `[Pasted text #N]`-Platzhalter, anstatt einen zweiten hinzuzufügen. Eine Hintergrund-Sitzung, die durch Akzeptieren eines Plans benannt wird, zeigt diesen Namen auf ihrer Zeile an. Eine Hintergrund-Sitzung, die in einen Worktree verschoben wurde, behält ihre Unterhaltung, wenn ihr Prozess aus der Agenten-Ansicht neu gestartet wird.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| v2.1.206 | Zeilenzusammenfassungen füllen die verbleibende Breite der Zeile und schneiden nur am rechten Rand des Terminals ab, anstatt bei 64 Spalten. Nachdem der Supervisor in eine neue Claude Code-Version neu startet, startet er die verbleibenden untätigen Hintergrund-Sitzungen im Hintergrund auf diese Version neu, anstatt einige pro Minute. Das Löschen einer Sitzung mit `Ctrl+X` oder `claude rm` löscht sie auch aus der Sitzungsliste des Supervisors, sodass die Zeile nach einem Supervisor-Neustart nicht mehr erneut angezeigt wird.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| v2.1.205 | Zeilenzusammenfassungen zeigen den eigenen einzeiligen Bericht der Sitzung, abgeschnitten bei 64 Spalten, anstelle einer rohen Tool-Aufrufe oder einer `done/total`-Anzahl; verzeichnisgruppierten Zeilen öffnen sich mit einem farbigen Statuswort. Das Vorschau-Panel öffnet sich mit dem vollständigen Statussatz und für eine Sitzung, die auf Sie wartet, ihrer genauen Frage über der Antwort-Eingabe. Sitzungen, die einen Pull Request mit `gh` bearbeiten, kommentieren, schließen oder als bereit markieren, sind damit verknüpft, nicht nur solche, die einen Pull Request erstellen oder auschecken, ein Push verknüpft einen Pull Request auch wenn der lokale Branch-Name nicht übereinstimmt, und ein Pull Request, dessen Erstellungsbefehl-Ausgabe das Inline-Limit überschritten hat, ist auch verknüpft. Eine Wendung ohne lesbaren Text behält den vorherigen Zustand der Sitzung bei, anstatt ihn zurück auf `Working` zu setzen. `claude attach` wartet bis zu etwa 60 Sekunden auf eine Sitzung, die neu startet, mit einer Statuszeile, die angibt warum, anstatt fehlzuschlagen.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| v2.1.203 | Ein Gateway `ANTHROPIC_BASE_URL`, das in der Versand-Shell exportiert wird, erreicht die von ihr versendeten Sitzungen in demselben Verzeichnis, wenn der Supervisor diese Gateway-Umgebung teilt, anstatt gelöscht zu werden, während der daneben exportierte API-Schlüssel beibehalten wird. Der `PATH` der Versand-Shell wird auf den Worker jeder Sitzung angewendet. Das Drücken von `←` während Subagenten laufen, wartet auf sie, anstatt sie nach zehn Sekunden neu zu starten. Die leere Liste zeigt immer die Abschnittskopfzeilen mit einer Beschreibung unter jedem an. Das Eingeben von `@` in der Versand-Eingabe listet auch die registrierten Git-Worktrees des Start-Repositorys auf, die sich in seinem Verzeichnisbaum befinden. Ein Aufwand, der von der `effortLevel`-Einstellung geerbt wird, folgt späteren Änderungen dieser Einstellung, anstatt beim Versand festgelegt zu werden. Das Öffnen einer gestoppten Sitzung, deren Unterhaltung bereits in einer anderen laufenden Sitzung offen ist, wird mit einer Nachricht abgelehnt, anstatt die Zeile fehlschlagen zu lassen. Ein Befehl, der in der Agenten-Ansicht nicht verfügbar ist, lässt den eingegebenen Text in der Eingabe. Ein `WorktreeCreate`-Hook, der außerhalb eines Git-Repositorys fehlschlägt, blockiert die Sitzung nicht mehr beim Bearbeiten von Dateien.                                                                                                                                                                                                                                                                                                                                                                                                                     |
| v2.1.202 | Ein Name, der mit `/rename` oder `Ctrl+R` in einer Hintergrund-Sitzung festgelegt wird, bleibt erhalten, wenn der Supervisor seinen Prozess stoppt und neu startet, anstatt auf den Namen zurückzusetzen, mit dem die Sitzung versendet wurde.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| v2.1.200 | Eine ältere Claude Code-Version, die die Sitzungsliste in `roster.json` umschreibt, behält Felder bei, die von einer neueren Version geschrieben wurden, was der bestehenden `state.json`-Garantie entspricht, sodass Sitzungen, die von der neueren Version gestartet wurden, nach dem Neustart des Supervisors weiterhin Eingaben akzeptieren. Wenn Sie eine Sitzung öffnen, die nicht mehr reagiert, startet der Supervisor ihren Prozess neu und die Sitzung setzt die unterbrochene Antwort von dort fort, wo sie stehen geblieben ist.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| v2.1.199 | Eine Hintergrund-Sitzung, deren Prozess beendet wird, bevor sie auf einem Host mit wenig Speicher vollständig startet, zeigt `possibly low memory — free some up and retry` in ihrem Zeilenstatus an, anstatt nur die bloße Beendigungsursache anzuzeigen. Das Verschieben einer Sitzung in den Hintergrund mit `←` oder `/background` überträgt ihre `/color`-Einstellung auf die neue Zeile.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| v2.1.198 | Die Agenten-Ansicht sendet eine Benachrichtigung über `preferredNotifChannel`, wenn eine Hintergrund-Sitzung Eingabe benötigt, beendet wird oder fehlschlägt, und löst den `Notification`-Hook mit dem Typ `agent_needs_input` oder `agent_completed` aus. `←` und `/exit` innerhalb von `claude attach <id>` kehren zur Agenten-Ansicht zurück, anstatt zur Shell zu beenden; `Ctrl+Z` kehrt zur Shell zurück. Eine Hintergrund-Sitzung, die ihre Arbeit in einem Worktree isoliert, committed und pusht ihren eigenen isolierten Branch, niemals `main` oder `master`, und öffnet einen Entwurf eines Pull Requests, wenn sie beendet wird, anstatt zuerst zu fragen. `/login` wird in der Agenten-Ansicht ausgeführt und öffnet den Anmeldedialog. Der Beendigungsdialog `Background work is running` bietet `Move to background and exit` an. Die Beendigungsübergabe deckt auch Hintergrund-Subagenten ab, die beim nächsten Aufwachen aus ihrem Transkript fortgesetzt werden, anstatt als fehlgeschlagen gemeldet zu werden. `claude --bg` kombiniert mit `-p` oder `--print` wird mit einem Fehler abgelehnt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| v2.1.196 | Ein einzelner `←`-Druck verschiebt eine Vordergrund-Sitzung in den Hintergrund; frühere Versionen erforderten zwei Drücke, mit einem Fußzeilen-Hinweis und einer Bestätigung. `--dangerously-skip-permissions`, das an `claude agents` übergeben wird, zeigt den Bypass-Haftungsausschluss an, anstatt stillschweigend gelöscht zu werden. Interaktive Sitzungen, die Sie nie benannt haben, tragen einen Standard-Namen wie `my-app-3f` in Sitzungsauflistungen und `claude agents --json`. Hintergrund-Shell-Befehle und dynamische Workflows überleben das Stoppen, Neustarten oder Aktualisieren des Sitzungsprozesses, einschließlich unter Windows; setzen Sie `CLAUDE_CODE_DISABLE_BG_EXIT_HANDOFF=1`, um die Übergabe auszuschalten. Ein Transkript, das beim Neustart als leer missverstanden wird, wird mit einem `.orphaned-`-Suffix umbenannt, anstatt gelöscht zu werden.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| v2.1.195 | Laufende Arbeit wird auch unter Windows übertragen, wenn Sie eine Sitzung in den Hintergrund verschieben; setzen Sie `CLAUDE_DISABLE_ADOPT=1`, um sie stattdessen zu stoppen. Die Gruppe `Abgeschlossen` füllt den verbleibenden vertikalen Platz und die Kopfzeile wird auf kurzen Terminals komprimiert. Eine ältere Claude Code-Version löscht nicht mehr neuere Sitzungs-`state.json`-Felder oder versteckt diese Sitzungen nicht vor `claude agents`. Das Anhängen an eine gestoppte Sitzung wechselt sofort, anstatt bis zu fünf Sekunden einen leeren Bildschirm zu zeigen. Ein Supervisor, der keine Verbindungen akzeptieren kann, beendet sich selbst und gibt seine Sperre frei.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| v2.1.174 | Hintergrund-Sitzungen erben keine Gateway-Endpunkt-Variablen wie `ANTHROPIC_BASE_URL` aus der Supervisor-Start-Shell mehr; der Supervisor stellt einen frischen Snapshot der Anmeldedaten für vorgewärmte Worker bereit, was spurlose `Could not resolve authentication method`-Fehler behebt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| v2.1.172 | `/model` in der Versand-Eingabe setzt eine Sitzungs-Bereichs-Versand-Modell-Überschreibung.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| v2.1.161 | Zeilenzusammenfassungen zeigen eine `done/total`-Anzahl für parallele Arbeitselemente; das Vorschau-Panel benennt das am längsten laufende parallele Arbeitselement.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| v2.1.157 | `claude agents` akzeptiert `--agent`; versendete Sitzungen beachten die `agent`-Einstellung.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| v2.1.145 | Sprachdiktat wird in der Vorschau-Panel-Antwort-Eingabe und der Versand-Eingabe unterstützt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| v2.1.143 | `worktree.bgIsolation`-Einstellung hinzugefügt; `claude agents` akzeptiert `--allow-dangerously-skip-permissions`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| v2.1.142 | `claude agents` akzeptiert `--permission-mode`, `--model`, `--effort`, `--dangerously-skip-permissions`, `--settings`, `--add-dir`, `--plugin-dir`, `--mcp-config` und `--strict-mcp-config`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| v2.1.141 | `claude agents` akzeptiert `--cwd`, um die Liste auf ein Projekt zu beschränken.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| v2.1.139 | Agenten-Ansicht als Forschungsvorschau eingeführt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
