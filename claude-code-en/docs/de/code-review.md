> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Code Review

> Richten Sie automatisierte PR-Reviews ein, die Logikfehler, Sicherheitslücken und Regressionen durch Multi-Agent-Analyse Ihrer vollständigen Codebasis erkennen

<Note>
  Code Review befindet sich in der Forschungsvorschau und ist für [Teams und Enterprise](https://claude.ai/admin-settings/claude-code) Abonnements verfügbar. Es ist nicht verfügbar für Organisationen mit [Zero Data Retention](/docs/de/zero-data-retention) aktiviert.
</Note>

Code Review analysiert Ihre GitHub Pull Requests und veröffentlicht Erkenntnisse als Inline-Kommentare auf den Codezeilen, auf denen Probleme gefunden wurden. Eine Flotte spezialisierter Agenten untersucht die Codeänderungen im Kontext Ihrer vollständigen Codebasis und sucht nach Logikfehlern, Sicherheitslücken, fehlerhaften Grenzfällen und subtilen Regressionen.

Erkenntnisse werden nach Schweregrad gekennzeichnet und genehmigen oder blockieren Ihren PR nicht, sodass bestehende Review-Workflows intakt bleiben. Sie können anpassen, was Claude kennzeichnet, indem Sie eine `CLAUDE.md` oder `REVIEW.md` Datei zu Ihrem Repository hinzufügen.

Um Claude in Ihrer eigenen CI-Infrastruktur statt dieses verwalteten Dienstes auszuführen, siehe [GitHub Actions](/docs/de/github-actions) oder [GitLab CI/CD](/docs/de/gitlab-ci-cd). Für Repositorys auf einer selbst gehosteten GitHub-Instanz siehe [GitHub Enterprise Server](/docs/de/github-enterprise-server).

Diese Seite behandelt:

* [Wie Reviews funktionieren](#how-reviews-work)
* [Setup](#set-up-code-review)
* [Manuelles Auslösen von Reviews](#manually-trigger-reviews) mit `@claude review` und `@claude review once`
* [Anpassung von Reviews](#customize-reviews) mit `CLAUDE.md` und `REVIEW.md`
* [Preisgestaltung](#pricing)
* [Fehlerbehebung](#troubleshooting) fehlgeschlagener Ausführungen und fehlender Kommentare
* [Überprüfung eines Diffs lokal](#review-a-diff-locally) mit dem `/code-review` Befehl

<Note>
  Um einen Diff lokal in Ihrem Terminal ohne Installation der GitHub App zu überprüfen, führen Sie den `/code-review` Befehl in einer beliebigen Claude Code Sitzung aus. Siehe [Überprüfung eines Diffs lokal](#review-a-diff-locally).
</Note>

<h2 id="how-reviews-work">
  Wie Reviews funktionieren
</h2>

Sobald ein Administrator [Code Review aktiviert](#set-up-code-review) für Ihre Organisation, werden Reviews ausgelöst, wenn ein PR geöffnet wird, bei jedem Push oder auf manuelle Anfrage, je nach konfiguriertem Verhalten des Repositorys. Das Kommentieren von `@claude review` [startet Reviews auf einem PR](#manually-trigger-reviews) in jedem Modus.

Wenn ein Review ausgeführt wird, analysieren mehrere Agenten parallel den Diff und den umgebenden Code auf Anthropic-Infrastruktur. Jeder Agent sucht nach einer anderen Klasse von Problemen, dann überprüft ein Verifizierungsschritt Kandidaten gegen das tatsächliche Codeverhalten, um falsch positive Ergebnisse zu filtern. Die Ergebnisse werden dedupliziert, nach Schweregrad eingestuft und als Inline-Kommentare auf den spezifischen Zeilen veröffentlicht, auf denen Probleme gefunden wurden, mit einer Zusammenfassung im Review-Text. Wenn keine Probleme gefunden werden, aktualisiert Code Review die GitHub-Check-Run, um anzuzeigen, dass keine Probleme erkannt wurden. Claude kann auch einen kurzen Bestätigungskommentar auf dem PR veröffentlichen.

Reviews skalieren in den Kosten mit PR-Größe und Komplexität und werden im Durchschnitt in 20 Minuten abgeschlossen. Administratoren können Review-Aktivität und Ausgaben über das [Analytics-Dashboard](#view-usage) überwachen.

<h3 id="severity-levels">
  Schweregrad-Stufen
</h3>

Jede Erkenntnis wird mit einer Schweregrad-Stufe gekennzeichnet:

| Marker | Schweregrad       | Bedeutung                                                                                   |
| :----- | :---------------- | :------------------------------------------------------------------------------------------ |
| 🔴     | Wichtig           | Ein Fehler, der vor dem Zusammenführen behoben werden sollte                                |
| 🟡     | Nit               | Ein kleineres Problem, das behoben werden sollte, aber nicht blockierend ist                |
| 🟣     | Bereits vorhanden | Ein Fehler, der in der Codebasis vorhanden ist, aber nicht durch diesen PR eingeführt wurde |

Erkenntnisse enthalten einen ausklappbaren erweiterten Reasoning-Bereich, den Sie erweitern können, um zu verstehen, warum Claude das Problem gekennzeichnet hat und wie es das Problem überprüft hat.

<h3 id="rate-and-reply-to-findings">
  Bewertung und Antwort auf Erkenntnisse
</h3>

Jeder Review-Kommentar von Claude kommt bereits mit 👍 und 👎 angehängt, sodass beide Schaltflächen in der GitHub-Benutzeroberfläche für Ein-Klick-Bewertung angezeigt werden. Klicken Sie auf 👍, wenn die Erkenntnis nützlich war, oder auf 👎, wenn sie falsch oder störend war. Anthropic sammelt Reaktionszählungen nach dem Zusammenführen des PR und verwendet sie, um den Reviewer zu optimieren. Reaktionen lösen keine Neuüberprüfung aus oder ändern etwas auf dem PR.

Das Antworten auf einen Inline-Kommentar veranlasst Claude nicht, zu antworten oder den PR zu aktualisieren. Um auf eine Erkenntnis zu reagieren, beheben Sie den Code und pushen Sie. Wenn der PR für Push-ausgelöste Reviews abonniert ist, löst die nächste Ausführung den Thread auf, wenn das Problem behoben ist. Um eine neue Überprüfung ohne Pushen anzufordern, kommentieren Sie `@claude review once` als [Top-Level-PR-Kommentar](#manually-trigger-reviews).

<h3 id="check-run-output">
  Check-Run-Ausgabe
</h3>

Neben den Inline-Review-Kommentaren füllt jedes Review die **Claude Code Review** Check-Run auf, die neben Ihren CI-Checks angezeigt wird. Erweitern Sie ihren **Details**-Link, um eine Zusammenfassung aller Erkenntnisse an einem Ort zu sehen, sortiert nach Schweregrad:

| Schweregrad | Datei:Zeile               | Problem                                                                                   |
| ----------- | ------------------------- | ----------------------------------------------------------------------------------------- |
| 🔴 Wichtig  | `src/auth/session.ts:142` | Token-Aktualisierung läuft parallel mit Logout, wodurch veraltete Sitzungen aktiv bleiben |
| 🟡 Nit      | `src/auth/session.ts:88`  | `parseExpiry` gibt stillschweigend 0 bei fehlerhafter Eingabe zurück                      |

Jede Erkenntnis wird auch als Anmerkung auf der Registerkarte **Files changed** angezeigt, direkt auf den relevanten Diff-Zeilen markiert. Wichtige Erkenntnisse werden mit einem roten Marker gerendert, Nits mit einer gelben Warnung und bereits vorhandene Fehler mit einer grauen Benachrichtigung. Anmerkungen und die Schweregrad-Tabelle werden unabhängig von Inline-Review-Kommentaren in die Check-Run geschrieben, sodass sie verfügbar bleiben, auch wenn GitHub einen Inline-Kommentar auf einer Zeile ablehnt, die sich verschoben hat.

Die Check-Run wird immer mit einer neutralen Schlussfolgerung abgeschlossen, sodass sie das Zusammenführen durch Branch-Schutzregeln niemals blockiert. Wenn Sie Zusammenführungen auf Code Review-Erkenntnisse beschränken möchten, lesen Sie die Schweregrad-Aufschlüsselung aus der Check-Run-Ausgabe in Ihrem eigenen CI. Die letzte Zeile des Details-Texts ist ein maschinenlesbarer Kommentar, den Ihr Workflow mit `gh` und jq analysieren kann:

```bash theme={null}
gh api repos/OWNER/REPO/check-runs/CHECK_RUN_ID \
  --jq '.output.text | split("bughunter-severity: ")[1] | split(" -->")[0] | fromjson'
```

Dies gibt ein JSON-Objekt mit Zählungen pro Schweregrad zurück, zum Beispiel `{"normal": 2, "nit": 1, "pre_existing": 0}`. Der `normal`-Schlüssel enthält die Anzahl der Wichtig-Erkenntnisse; ein Wert ungleich Null bedeutet, dass Claude mindestens einen Fehler gefunden hat, der vor dem Zusammenführen behoben werden sollte.

<h3 id="what-code-review-checks">
  Was Code Review überprüft
</h3>

Standardmäßig konzentriert sich Code Review auf Korrektheit: Fehler, die die Produktion unterbrechen würden, nicht auf Formatierungspräferenzen oder fehlende Testabdeckung. Sie können erweitern, was es überprüft, indem Sie [Anleitungsdateien hinzufügen](#customize-reviews) zu Ihrem Repository.

<h2 id="set-up-code-review">
  Code Review einrichten
</h2>

Ein Owner aktiviert Code Review einmal für die Organisation und wählt aus, welche Repositorys einbezogen werden sollen.

<Steps>
  <Step title="Öffnen Sie die Claude Code Admin-Einstellungen">
    Gehen Sie zu [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) und finden Sie den Code Review Bereich. Sie benötigen die Owner- oder Primary Owner-Rolle in Ihrer Claude-Organisation und die Berechtigung, GitHub Apps in Ihrer GitHub-Organisation zu installieren.
  </Step>

  <Step title="Setup starten">
    Klicken Sie auf **Setup**. Dies startet den GitHub App-Installationsablauf.
  </Step>

  <Step title="Installieren Sie die Claude GitHub App">
    Folgen Sie den Aufforderungen, um die Claude GitHub App in Ihrer GitHub-Organisation zu installieren. Die App fordert diese Repository-Berechtigungen an:

    * **Contents**: Lesen und Schreiben
    * **Issues**: Lesen und Schreiben
    * **Pull requests**: Lesen und Schreiben

    Code Review verwendet Lesezugriff auf Inhalte und Schreibzugriff auf Pull Requests. Der breitere Berechtigungssatz unterstützt auch [GitHub Actions](/docs/de/github-actions), wenn Sie diese später aktivieren.
  </Step>

  <Step title="Wählen Sie Repositorys aus">
    Wählen Sie aus, welche Repositorys für Code Review aktiviert werden sollen. Wenn Sie ein Repository nicht sehen, stellen Sie sicher, dass Sie der Claude GitHub App während der Installation Zugriff darauf gewährt haben. Sie können später weitere Repositorys hinzufügen.
  </Step>

  <Step title="Legen Sie Review-Trigger pro Repo fest">
    Nach Abschluss des Setups zeigt der Code Review Bereich Ihre Repositorys in einer Tabelle an. Verwenden Sie für jedes Repository das Dropdown-Menü **Review Behavior**, um auszuwählen, wann Reviews ausgeführt werden:

    * **Once after PR creation**: Review wird einmal ausgeführt, wenn ein PR geöffnet oder als bereit zur Überprüfung markiert wird
    * **After every push**: Review wird bei jedem Push zum PR-Branch ausgeführt, erkennt neue Probleme, während sich der PR entwickelt, und löst Threads automatisch auf, wenn Sie gekennzeichnete Probleme beheben
    * **Manual**: Reviews werden nur gestartet, wenn jemand [kommentiert `@claude review` oder `@claude review once` auf einem PR](#manually-trigger-reviews); `@claude review` abonniert den PR auch für Reviews bei nachfolgenden Pushes

    Das Überprüfen bei jedem Push führt die meisten Reviews durch und kostet am meisten. Der manuelle Modus ist nützlich für Repositorys mit hohem Datenverkehr, bei denen Sie bestimmte PRs in die Überprüfung aufnehmen möchten, oder um nur mit der Überprüfung Ihrer PRs zu beginnen, wenn sie bereit sind.
  </Step>
</Steps>

Die Repositorys-Tabelle zeigt auch die durchschnittlichen Kosten pro Review für jedes Repo basierend auf der letzten Aktivität. Verwenden Sie das Zeilenaktionsmenü, um Code Review pro Repository ein- oder auszuschalten, oder um ein Repository vollständig zu entfernen.

Um das Setup zu überprüfen, öffnen Sie einen Test-PR. Wenn Sie einen automatischen Trigger gewählt haben, wird eine Check-Run namens **Claude Code Review** innerhalb weniger Minuten angezeigt. Wenn Sie Manual gewählt haben, kommentieren Sie `@claude review` auf dem PR, um die erste Überprüfung zu starten. Wenn keine Check-Run angezeigt wird, bestätigen Sie, dass das Repository in Ihren Admin-Einstellungen aufgelistet ist und die Claude GitHub App Zugriff darauf hat.

<h2 id="manually-trigger-reviews">
  Manuelles Auslösen von Reviews
</h2>

Zwei Kommentarbefehle starten eine Überprüfung auf Anfrage. Beide funktionieren unabhängig vom konfigurierten Trigger des Repositorys, sodass Sie sie verwenden können, um bestimmte PRs im manuellen Modus in die Überprüfung aufzunehmen oder um eine sofortige Neuüberprüfung in anderen Modi zu erhalten.

| Befehl                | Was er tut                                                                           |
| :-------------------- | :----------------------------------------------------------------------------------- |
| `@claude review`      | Startet eine Überprüfung und abonniert den PR für Push-ausgelöste Reviews in Zukunft |
| `@claude review once` | Startet eine einzelne Überprüfung, ohne den PR für zukünftige Pushes zu abonnieren   |

Verwenden Sie `@claude review once`, wenn Sie Feedback zum aktuellen Zustand eines PR möchten, aber nicht möchten, dass jeder nachfolgende Push eine Überprüfung verursacht. Dies ist nützlich für langfristige PRs mit häufigen Pushes oder wenn Sie eine einmalige zweite Meinung möchten, ohne das Review-Verhalten des PR zu ändern.

Damit einer der beiden Befehle eine Überprüfung auslöst:

* Veröffentlichen Sie ihn als Top-Level-PR-Kommentar, nicht als Inline-Kommentar auf einer Diff-Zeile
* Setzen Sie den Befehl an den Anfang des Kommentars, mit `once` auf der gleichen Zeile, wenn Sie die One-Shot-Form verwenden
* Sie müssen Owner-, Member- oder Collaborator-Zugriff auf das Repository haben
* Der PR muss offen sein

Im Gegensatz zu automatischen Triggern werden manuelle Trigger auf Entwurfs-PRs ausgeführt, da eine explizite Anfrage signalisiert, dass Sie die Überprüfung jetzt möchten, unabhängig vom Entwurfsstatus.

Wenn bereits eine Überprüfung auf diesem PR läuft, wird die Anfrage in die Warteschlange eingereiht, bis die laufende Überprüfung abgeschlossen ist. Sie können den Fortschritt über die Check-Run auf dem PR überwachen.

<h2 id="customize-reviews">
  Anpassung von Reviews
</h2>

Code Review liest zwei Dateien aus Ihrem Repository, um zu steuern, was es kennzeichnet. Sie unterscheiden sich darin, wie stark sie die Überprüfung beeinflussen:

* **`CLAUDE.md`**: gemeinsame Projektanweisungen, die Claude Code für alle Aufgaben verwendet, nicht nur für Reviews. Code Review liest sie als Projektkontext und kennzeichnet neu eingeführte Verstöße als Nits.
* **`REVIEW.md`**: Review-spezifische Anweisungen, die direkt in jeden Agent in der Review-Pipeline als höchste Priorität eingefügt werden. Verwenden Sie es, um zu ändern, was gekennzeichnet wird, mit welchem Schweregrad und wie Erkenntnisse gemeldet werden.

<h3 id="claude-md">
  CLAUDE.md
</h3>

Code Review liest Ihre Repository-`CLAUDE.md` Dateien und behandelt neu eingeführte Verstöße als [Nit-Level](#severity-levels) Erkenntnisse. Dies funktioniert bidirektional: Wenn Ihr PR Code auf eine Weise ändert, die eine `CLAUDE.md` Aussage veraltet macht, kennzeichnet Claude, dass die Dokumentation aktualisiert werden muss.

Claude liest `CLAUDE.md` Dateien auf jeder Ebene Ihrer Verzeichnishierarchie, sodass Regeln in einer Unterverzeichnis-`CLAUDE.md` nur auf Dateien unter diesem Pfad angewendet werden. Weitere Informationen zur Funktionsweise von `CLAUDE.md` finden Sie in der [Memory-Dokumentation](/docs/de/memory).

Für Review-spezifische Anleitungen, die Sie nicht auf allgemeine Claude Code Sitzungen angewendet haben möchten, verwenden Sie stattdessen [`REVIEW.md`](#review-md).

<h3 id="review-md">
  REVIEW\.md
</h3>

`REVIEW.md` ist eine Datei in Ihrem Repository-Root, die überschreibt, wie Code Review auf Ihrem Repo verhält. Sein Inhalt wird in den System-Prompt jedes Agenten in der Review-Pipeline als höchste Prioritäts-Anweisungsblock eingefügt und hat Vorrang vor der Standard-Review-Anleitung.

Da es wörtlich eingefügt wird, ist `REVIEW.md` einfache Anweisungen: [`@` Import-Syntax](/docs/de/memory#import-additional-files) wird nicht erweitert, und referenzierte Dateien werden nicht in den Prompt gelesen. Setzen Sie die Regeln, die Sie durchgesetzt haben möchten, direkt in die Datei.

<h4 id="what-you-can-tune">
  Was Sie optimieren können
</h4>

`REVIEW.md` ist freies Markdown, sodass alles, was Sie als Review-Anweisung ausdrücken können, im Umfang liegt. Die folgenden Muster haben die meiste praktische Auswirkung.

**Schweregrad**: Definieren Sie neu, was 🔴 Wichtig für Ihr Repo bedeutet. Die Standard-Kalibrierung zielt auf Produktionscode ab; ein Docs-Repo, ein Config-Repo oder ein Prototyp möchte möglicherweise eine viel engere Definition. Geben Sie explizit an, welche Klassen von Erkenntnissen Wichtig sind und welche höchstens Nit sind. Sie können auch in die andere Richtung eskalieren, zum Beispiel jeden `CLAUDE.md` Verstoß als Wichtig statt des Standard-Nits behandeln.

**Nit-Volumen**: Begrenzen Sie, wie viele 🟡 Nit-Kommentare eine einzelne Überprüfung veröffentlicht. Prosa- und Config-Dateien können für immer poliert werden. Eine Obergrenze wie 'höchstens fünf Nits melden, den Rest als Zählung in der Zusammenfassung erwähnen" hält Reviews umsetzbar.

**Skip-Regeln**: Listen Sie Pfade, Branch-Muster und Erkenntniskategorien auf, bei denen Claude keine Erkenntnisse veröffentlichen sollte. Häufige Kandidaten sind generierter Code, Lockfiles, vendorte Abhängigkeiten und maschinengeschriebene Branches, zusammen mit allem, das Ihr CI bereits durchsetzt, wie Linting oder Rechtschreibprüfung. Für Pfade, die einige Überprüfung verdienen, aber nicht vollständige Überprüfung, setzen Sie stattdessen eine höhere Messlatte: „in `scripts/`, nur melden, wenn nahezu sicher und schwerwiegend."

**Repo-spezifische Überprüfungen**: Fügen Sie Regeln hinzu, die Sie auf jedem PR gekennzeichnet haben möchten, wie „neue API-Routen müssen einen Integrationtest haben." Da `REVIEW.md` als höchste Priorität eingefügt wird, landen diese zuverlässiger als die gleichen Regeln in einem langen `CLAUDE.md`.

**Verifizierungsbalken**: Fordern Sie Beweise an, bevor eine Erkenntnisklasse veröffentlicht wird. Zum Beispiel, „Verhaltensansprüche benötigen eine `file:line` Zitierung in der Quelle, nicht eine Inferenz aus Benennung" reduziert falsch positive Ergebnisse, die sonst den Autor eine Runde kosten würden.

**Re-Review-Konvergenz**: Sagen Sie Claude, wie er sich verhalten soll, wenn ein PR bereits überprüft wurde. Eine Regel wie „nach der ersten Überprüfung, neue Nits unterdrücken und nur Wichtig-Erkenntnisse veröffentlichen" stoppt eine einzeilige Korrektur von Runde sieben allein auf Stil.

**Zusammenfassungsform**: Bitten Sie darum, dass der Review-Text mit einer einzeiligen Tally wie `2 faktisch, 4 Stil` beginnt, und führen Sie mit „keine faktischen Probleme" an, wenn das der Fall ist. Der Autor möchte die Form der Arbeit vor den Details wissen.

<h4 id="example">
  Beispiel
</h4>

Dieses `REVIEW.md` kalibriert den Schweregrad für einen Backend-Service neu, begrenzt Nits, überspringt generierte Dateien und fügt Repo-spezifische Überprüfungen hinzu.

```markdown theme={null}
# Review-Anweisungen

## Was Wichtig hier bedeutet

Reservieren Sie Wichtig für Erkenntnisse, die Verhalten unterbrechen würden, Daten lecken würden,
oder einen Rollback blockieren würden: falsche Logik, unscoped Datenbankabfragen, PII
in Logs oder Fehlermeldungen, und Migrationen, die nicht rückwärtskompatibel sind. Stil, Benennung und Refactoring-Vorschläge sind höchstens Nit.

## Begrenzen Sie die Nits

Melden Sie höchstens fünf Nits pro Überprüfung. Wenn Sie mehr gefunden haben, sagen Sie „plus N
ähnliche Elemente" in der Zusammenfassung statt sie inline zu veröffentlichen. Wenn
alles, was Sie gefunden haben, ein Nit ist, führen Sie die Zusammenfassung mit „Keine blockierenden
Probleme" an.

## Nicht melden

- Alles, das CI bereits durchsetzt: Lint, Formatierung, Typfehler
- Generierte Dateien unter `src/gen/` und jede `*.lock` Datei
- Nur-Test-Code, der absichtlich Produktionsregeln verletzt

## Immer überprüfen

- Neue API-Routen haben einen Integrationtest
- Log-Zeilen enthalten keine E-Mail-Adressen, Benutzer-IDs oder Request-Bodies
- Datenbankabfragen sind auf den Aufrufer des Mandanten beschränkt
```

<h4 id="keep-it-focused">
  Halten Sie es fokussiert
</h4>

Länge hat einen Preis: Ein langer `REVIEW.md` verwässert die Regeln, die am meisten zählen. Halten Sie es auf Anweisungen, die Review-Verhalten ändern, und lassen Sie allgemeinen Projektkontext in `CLAUDE.md`.

<h2 id="view-usage">
  Nutzung anzeigen
</h2>

Gehen Sie zu [claude.ai/analytics/code-review](https://claude.ai/analytics/code-review), um Code Review Aktivität in Ihrer Organisation zu sehen. Das Dashboard zeigt:

| Bereich              | Was es zeigt                                                                                                |
| :------------------- | :---------------------------------------------------------------------------------------------------------- |
| PRs reviewed         | Tägliche Anzahl der überprüften Pull Requests über den ausgewählten Zeitraum                                |
| Cost weekly          | Wöchentliche Ausgaben für Code Review                                                                       |
| Feedback             | Anzahl der Review-Kommentare, die automatisch aufgelöst wurden, weil ein Entwickler das Problem behoben hat |
| Repository breakdown | Pro-Repo-Anzahl der überprüften PRs und aufgelösten Kommentare                                              |

Die Repositorys-Tabelle in den Admin-Einstellungen zeigt auch die durchschnittlichen Kosten pro Review für jedes Repo. Dashboard-Kostenzahlen sind Schätzungen zur Überwachung der Aktivität; für rechnungsgenaue Ausgaben beziehen Sie sich auf Ihre Anthropic-Rechnung.

<h2 id="pricing">
  Preisgestaltung
</h2>

Code Review wird basierend auf der Token-Nutzung abgerechnet. Jede Überprüfung kostet durchschnittlich \$15–25, skalierend mit PR-Größe, Codebasis-Komplexität und wie viele Probleme eine Überprüfung erfordern. Code Review-Nutzung wird separat über [Nutzungsguthaben](https://support.claude.com/de/articles/12429409-extra-usage-for-paid-claude-plans) abgerechnet und zählt nicht gegen die in Ihrem Plan enthaltene Nutzung.

Der Review-Trigger, den Sie wählen, beeinflusst die Gesamtkosten:

* **Once after PR creation**: wird einmal pro PR ausgeführt
* **After every push**: wird bei jedem Push ausgeführt, multipliziert die Kosten mit der Anzahl der Pushes
* **Manual**: keine Reviews, bis jemand `@claude review` auf einem PR kommentiert

In jedem Modus führt das Kommentieren von `@claude review` [den PR in Push-ausgelöste Reviews auf](#manually-trigger-reviews), sodass zusätzliche Kosten pro Push nach diesem Kommentar anfallen. Um eine einzelne Überprüfung auszuführen, ohne sich für zukünftige Pushes zu abonnieren, kommentieren Sie stattdessen `@claude review once`.

Kosten erscheinen auf Ihrer Anthropic-Rechnung, unabhängig davon, ob Ihre Organisation Amazon Bedrock oder Google Cloud's Agent Platform für andere Claude Code-Funktionen verwendet. Um eine monatliche Ausgabenbegrenzung für Code Review festzulegen, gehen Sie zu [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage) und konfigurieren Sie das Limit für den Claude Code Review-Service.

Überwachen Sie die Ausgaben über das wöchentliche Kostendiagramm in [analytics](#view-usage) oder die durchschnittliche Kostenspalte pro Repository in den Admin-Einstellungen.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

Review-Ausführungen sind Best-Effort. Eine fehlgeschlagene Ausführung blockiert Ihren PR niemals, aber sie wird auch nicht automatisch erneut versucht. Dieser Abschnitt behandelt, wie Sie sich von einer fehlgeschlagenen Ausführung erholen und wo Sie nachschauen können, wenn die Check-Run Probleme meldet, die Sie nicht finden können.

<h3 id="retrigger-a-failed-or-timed-out-review">
  Auslösen einer fehlgeschlagenen oder abgelaufenen Überprüfung erneut
</h3>

Wenn die Review-Infrastruktur auf einen internen Fehler trifft oder ihr Zeitlimit überschreitet, wird die Check-Run mit einem Titel von **Code review encountered an error** oder **Code review timed out** abgeschlossen. Die Schlussfolgerung ist immer noch neutral, sodass nichts Ihre Zusammenführung blockiert, aber keine Erkenntnisse werden veröffentlicht.

Um die Überprüfung erneut auszuführen, kommentieren Sie `@claude review once` auf dem PR. Dies startet eine neue Überprüfung, ohne den PR für zukünftige Pushes zu abonnieren. Wenn der PR bereits für Push-ausgelöste Reviews abonniert ist, startet das Pushen eines neuen Commits auch eine neue Überprüfung.

Die Schaltfläche **Re-run** in Githubs Checks-Registerkarte löst Code Review nicht erneut aus. Verwenden Sie stattdessen den Kommentarbefehl oder einen neuen Push.

<h3 id="review-didn’t-run-and-the-pr-shows-a-spend-cap-message">
  Überprüfung wurde nicht ausgeführt und der PR zeigt eine Ausgabenbegrenzungs-Nachricht
</h3>

Wenn die monatliche Ausgabenbegrenzung Ihrer Organisation erreicht ist, veröffentlicht Code Review einen einzelnen Kommentar auf dem PR, der erklärt, dass die Überprüfung übersprungen wurde. Reviews werden automatisch am Anfang des nächsten Abrechnungszeitraums fortgesetzt, oder sofort, wenn ein Administrator die Obergrenze bei [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage) erhöht.

<h3 id="find-issues-that-aren’t-showing-as-inline-comments">
  Finden Sie Probleme, die nicht als Inline-Kommentare angezeigt werden
</h3>

Wenn der Check-Run-Titel besagt, dass Probleme gefunden wurden, aber Sie keine Inline-Review-Kommentare auf dem Diff sehen, schauen Sie an diesen anderen Stellen, wo Erkenntnisse angezeigt werden:

* **Check-Run Details**: Klicken Sie auf **Details** neben der Claude Code Review Check-Run auf der Registerkarte Checks. Die Schweregrad-Tabelle listet jede Erkenntnis mit ihrer Datei, Zeile und Zusammenfassung auf, unabhängig davon, ob der Inline-Kommentar akzeptiert wurde.
* **Files changed Anmerkungen**: Öffnen Sie die Registerkarte **Files changed** auf dem PR. Erkenntnisse werden als Anmerkungen gerendert, die direkt an den Diff-Zeilen angebracht sind, getrennt von Review-Kommentaren.
* **Review-Text**: Wenn Sie zum PR gepusht haben, während eine Überprüfung lief, können einige Erkenntnisse auf Zeilen verweisen, die nicht mehr im aktuellen Diff vorhanden sind. Diese werden unter einer **Additional findings** Überschrift im Review-Text angezeigt, anstatt als Inline-Kommentare.

<h2 id="review-a-diff-locally">
  Überprüfung eines Diffs lokal
</h2>

Der [`/code-review` Befehl](/docs/de/commands) überprüft einen Diff in Ihrem Terminal ohne Installation der GitHub App. Führen Sie ihn in einer beliebigen Claude Code Sitzung aus: er meldet Korrektheitsfehler und Wiederverwendung, Vereinfachung und Effizienz-Bereinigungen. Standardmäßig deckt die lokale Überprüfung die Commits Ihres Branches vor seinem Upstream plus alle nicht committeten Änderungen im Arbeitsbaum ab. Übergeben Sie `--comment`, um Erkenntnisse als Inline-PR-Kommentare zu veröffentlichen, oder `--fix`, um die Erkenntnisse auf Ihren Arbeitsbaum anzuwenden, nachdem die Überprüfung abgeschlossen ist.

Niedrigere [Aufwandsebenen](/docs/de/model-config#adjust-effort-level) geben weniger, höher vertrauenswürdige Erkenntnisse zurück, während `high` bis `max` breitere Abdeckung geben und unsichere Erkenntnisse einschließen können. Ohne ein Aufwandsargument verwendet die Überprüfung die aktuelle Aufwandsebene der Sitzung. Um etwas anderes als den Standard-Diff zu überprüfen, übergeben Sie ein Ziel: einen Dateipfad, eine PR-Nummer, einen Branch-Namen oder einen Ref-Bereich wie `main...my-feature`. Die Ref-Bereichsform überprüft den committeten Diff, den ein Pull Request von `my-feature` in `main` enthalten würde, unabhängig davon, wie der Upstream des Branches konfiguriert ist.

`/code-review ultra --fix` führt die tiefere [ultrareview](/docs/de/ultrareview) in der Cloud aus, dann wendet ihre Erkenntnisse auf Ihren Arbeitsbaum an, wenn sie in Ihrer Sitzung zurückkommen. Ultrareview verwendet seinen eigenen Umfang: Ihren aktuellen Branch gegen den Standard-Branch des Repositorys, plus alle nicht committeten und gestaged Änderungen im Arbeitsbaum.

Der Befehl hieß vor v2.1.147 `/simplify`, als er Fixes standardmäßig anwendete. Ab v2.1.154 führt `/simplify` eine separate Bereinigung-nur-Überprüfung durch, die Fixes anwendet, ohne nach Fehlern zu suchen. Wenn Sie `/simplify` für die Fehlersuche skriptet haben, wechseln Sie zu `/code-review --fix`, das unverändert ist.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

Code Review ist so konzipiert, dass es neben dem Rest von Claude Code funktioniert. Wenn Sie Reviews lokal ausführen möchten, bevor Sie einen PR öffnen, eine selbst gehostete Einrichtung benötigen oder tiefer verstehen möchten, wie `CLAUDE.md` Claudes Verhalten über Tools hinweg prägt, sind diese Seiten gute nächste Schritte:

* [Befehle](/docs/de/commands): führen Sie `/code-review` in einer lokalen Claude Code Sitzung aus, um einen Diff vor dem Pushen zu überprüfen
* [GitHub Actions](/docs/de/github-actions): führen Sie Claude in Ihren eigenen GitHub Actions Workflows aus für benutzerdefinierte Automatisierung über Code Review hinaus
* [GitLab CI/CD](/docs/de/gitlab-ci-cd): selbst gehostete Claude-Integration für GitLab-Pipelines
* [Memory](/docs/de/memory): wie `CLAUDE.md` Dateien über Claude Code funktionieren
* [Analytics](/docs/de/analytics): verfolgen Sie Claude Code Nutzung über Code Review hinaus
