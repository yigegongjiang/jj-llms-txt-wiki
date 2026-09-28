> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Code Review

> Richten Sie automatisierte PR-Reviews ein, die Logikfehler, Sicherheitslücken und Regressionen durch Multi-Agent-Analyse Ihrer vollständigen Codebasis erkennen

<Note>
  Code Review befindet sich in der Forschungsvorschau und ist für [Teams und Enterprise](https://claude.ai/admin-settings/claude-code) Abonnements verfügbar. Es ist nicht verfügbar für Organisationen mit [Zero Data Retention](/docs/de/zero-data-retention) aktiviert. Bei anderen Plänen können Sie immer noch [einen Diff lokal überprüfen](#review-a-diff-locally) mit dem `/code-review` Befehl.
</Note>

Code Review analysiert Ihre GitHub Pull Requests und veröffentlicht Erkenntnisse als Inline-Kommentare auf den Codezeilen, auf denen Probleme gefunden wurden. Eine Flotte spezialisierter Agenten untersucht die Codeänderungen im Kontext Ihrer vollständigen Codebasis und sucht nach Logikfehlern, Sicherheitslücken, fehlerhaften Grenzfällen und subtilen Regressionen.

Erkenntnisse werden nach Schweregrad gekennzeichnet und genehmigen oder blockieren Ihren PR nicht, sodass bestehende Review-Workflows intakt bleiben. Sie können anpassen, was Claude kennzeichnet, indem Sie eine `CLAUDE.md` oder `REVIEW.md` Datei zu Ihrem Repository hinzufügen.

Um Claude in Ihrer eigenen CI-Infrastruktur statt dieses verwalteten Dienstes auszuführen, siehe [GitHub Actions](/docs/de/github-actions) oder [GitLab CI/CD](/docs/de/gitlab-ci-cd). Für Repositorys auf einer selbst gehosteten GitHub-Instanz siehe [GitHub Enterprise Server](/docs/de/github-enterprise-server).

Diese Seite behandelt:

* [Wie Reviews funktionieren](#how-reviews-work)
* [Setup](#set-up-code-review)
* [Manuelles Auslösen von Reviews](#manually-trigger-reviews) mit `@claude review` und `@claude review always`
* [Anpassung von Reviews](#customize-reviews) mit `CLAUDE.md` und `REVIEW.md`
* [Preisgestaltung](#pricing)
* [Fehlerbehebung](#troubleshooting) fehlgeschlagener Ausführungen und fehlender Kommentare
* [Überprüfung eines Diffs lokal](#review-a-diff-locally) mit dem `/code-review` Befehl

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

Das Antworten auf einen Inline-Kommentar veranlasst Claude nicht, zu antworten oder den PR zu aktualisieren. Um auf eine Erkenntnis zu reagieren, beheben Sie den Code und pushen Sie. Wenn der PR für Push-ausgelöste Reviews abonniert ist, löst die nächste Ausführung den Thread auf, wenn das Problem behoben ist. Um eine neue Überprüfung ohne Pushen anzufordern, kommentieren Sie `@claude review` als [Top-Level-PR-Kommentar](#manually-trigger-reviews).

Um eine Erkenntnis ohne Codeänderung zu verwerfen, lösen Sie ihren Thread auf; das Antworten verwirft sie nicht.

<h3 id="check-run-output">
  Check-Run-Ausgabe
</h3>

Neben den Inline-Review-Kommentaren füllt jedes Review die **Claude Code Review** Check-Run auf, die neben Ihren CI-Checks angezeigt wird. Erweitern Sie ihren **Details**-Link, um eine Zusammenfassung aller Erkenntnisse an einem Ort zu sehen, sortiert nach Schweregrad:

| Schweregrad | Datei:Zeile               | Problem                                                                                   |
| ----------- | ------------------------- | ----------------------------------------------------------------------------------------- |
| 🔴 Wichtig  | `src/auth/session.ts:142` | Token-Aktualisierung läuft parallel mit Logout, wodurch veraltete Sitzungen aktiv bleiben |
| 🟡 Nit      | `src/auth/session.ts:88`  | `parseExpiry` gibt stillschweigend 0 bei fehlerhafter Eingabe zurück                      |

Jede Erkenntnis wird auch als Anmerkung auf der Registerkarte **Files changed** angezeigt, direkt auf den relevanten Diff-Zeilen markiert. Wichtige Erkenntnisse werden mit einem roten Marker gerendert, Nits mit einer gelben Warnung und bereits vorhandene Fehler mit einer grauen Benachrichtigung. Anmerkungen und die Schweregrad-Tabelle werden unabhängig von Inline-Review-Kommentaren in die Check-Run geschrieben, sodass sie verfügbar bleiben, auch wenn GitHub einen Inline-Kommentar auf einer Zeile ablehnt, die sich verschoben hat.

Die Check-Run wird immer mit einer neutralen Schlussfolgerung abgeschlossen, sodass sie das Zusammenführen durch Branch-Schutzregeln niemals blockiert. Wenn Sie Zusammenführungen auf Code Review-Erkenntnisse beschränken möchten, lesen Sie die Schweregrad-Aufschlüsselung aus der Check-Run-Ausgabe in Ihrem eigenen CI. Die letzte Zeile des Details-Texts ist ein maschinenlesbarer Kommentar, den Ihr Workflow mit `gh` und jq analysieren kann. Um die Check-Run-ID zu finden, listen Sie die Check-Runs des Commits mit `gh api repos/OWNER/REPO/commits/<commit-sha>/check-runs --jq '.check_runs[] | {id, name}'` auf und nehmen Sie die `id` der `Claude Code Review`-Ausführung. Ersetzen Sie `OWNER`, `REPO` und `CHECK_RUN_ID` durch Ihren Repository-Besitzer, Repository-Namen und diese ID:

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
    Folgen Sie den Aufforderungen, um die Claude GitHub App zu installieren: Wählen Sie die GitHub-Organisation aus, der die Repositorys gehören, die Sie überprüft haben möchten, wählen Sie aus, auf welche Repositorys die App zugreifen kann, und genehmigen Sie die angeforderten Berechtigungen.

    Um einen Pull Request zu überprüfen, liest Claude die Inhalte Ihres Repositorys über den Lesezugriff der App und veröffentlicht Kommentare und die [Check-Run](#check-run-output) über seinen Schreibzugriff auf Pull Requests und Checks. Während der Installation gewähren Sie einen breiteren Berechtigungssatz, der von anderen Claude-Funktionen wie [GitHub Actions](/docs/de/github-actions) gemeinsam genutzt wird; siehe [GitHub App-Berechtigungen](/docs/de/github-actions#github-app-permissions) für die vollständige Liste.
  </Step>

  <Step title="Wählen Sie Repositorys aus">
    Wählen Sie aus, welche Repositorys für Code Review aktiviert werden sollen. Wenn Sie ein Repository nicht sehen, stellen Sie sicher, dass Sie der Claude GitHub App während der Installation Zugriff darauf gewährt haben. Sie können später weitere Repositorys hinzufügen.
  </Step>

  <Step title="Legen Sie Review-Trigger pro Repo fest">
    Nach Abschluss des Setups zeigt der Code Review Bereich Ihre Repositorys in einer Tabelle an. Verwenden Sie für jedes Repository das Dropdown-Menü **Review Behavior**, um auszuwählen, wann Reviews ausgeführt werden:

    * **Once after PR creation**: Review wird einmal ausgeführt, wenn ein PR geöffnet oder als bereit zur Überprüfung markiert wird
    * **After every push**: Review wird bei jedem Push zum PR-Branch ausgeführt, erkennt neue Probleme, während sich der PR entwickelt, und löst Threads automatisch auf, wenn Sie gekennzeichnete Probleme beheben
    * **Manual**: Das Öffnen oder Pushen zu einem PR startet keine Überprüfung; kommentieren Sie [`@claude review`](#manually-trigger-reviews), um eine anzufordern, oder `@claude review always`, um den PR auch für Überprüfungen bei nachfolgenden Pushes zu abonnieren

    Unabhängig davon, welche Option Sie wählen, überprüft Claude einen [Pull Request von einem Fork](#review-pull-requests-from-forks) nur, wenn jemand `@claude review` darauf kommentiert.

    Das Überprüfen bei jedem Push führt die meisten Reviews durch und kostet am meisten. Der manuelle Modus ist nützlich für Repositorys mit hohem Datenverkehr, bei denen Sie bestimmte PRs in die Überprüfung aufnehmen möchten, oder um nur mit der Überprüfung Ihrer PRs zu beginnen, wenn sie bereit sind.
  </Step>
</Steps>

Die Repositorys-Tabelle zeigt auch die durchschnittlichen Kosten pro Review für jedes Repo basierend auf der letzten Aktivität. Verwenden Sie das Zeilenaktionsmenü, um Code Review pro Repository ein- oder auszuschalten, oder um ein Repository vollständig zu entfernen.

Um das Setup zu überprüfen, öffnen Sie einen Test-PR. Wenn Sie einen automatischen Trigger gewählt haben, wird eine Check-Run namens **Claude Code Review** innerhalb weniger Minuten angezeigt. Wenn Sie Manual gewählt haben, kommentieren Sie `@claude review` auf dem PR, um die erste Überprüfung zu starten. Wenn keine Check-Run angezeigt wird, bestätigen Sie, dass das Repository in Ihren Admin-Einstellungen aufgelistet ist und die Claude GitHub App Zugriff darauf hat.

<h2 id="manually-trigger-reviews">
  Manuelles Auslösen von Reviews
</h2>

Kommentarbefehle starten eine Überprüfung auf Anfrage. Sie funktionieren unabhängig vom konfigurierten Trigger des Repositorys, sodass Sie sie verwenden können, um bestimmte PRs im manuellen Modus in die Überprüfung aufzunehmen oder um eine sofortige Neuüberprüfung in anderen Modi zu erhalten.

| Befehl                  | Was er tut                                                                                  |
| :---------------------- | :------------------------------------------------------------------------------------------ |
| `@claude review`        | Startet eine einzelne Überprüfung, ohne den PR für zukünftige Pushes zu abonnieren          |
| `@claude review always` | Startet eine Überprüfung und abonniert den PR für Push-ausgelöste Reviews in Zukunft        |
| `@claude review once`   | Dasselbe wie `@claude review`: startet eine einzelne Überprüfung, ohne den PR zu abonnieren |

Verwenden Sie `@claude review always`, wenn Sie möchten, dass jeder nachfolgende Push zum PR eine neue Überprüfung startet, z. B. bei einem hochpriorisierten PR in einem Repository, das auf manuellen Modus eingestellt ist. Da der einfache Befehl den PR nicht abonniert, können Sie eine einmalige zweite Meinung anfordern, ohne zu ändern, ob spätere Pushes Überprüfungen auslösen.

<Note>
  Vor einem Update im Juli 2026 hat `@claude review` den PR für Push-ausgelöste Reviews abonniert. Wenn Sie sich auf dieses Verhalten verlassen haben, kommentieren Sie stattdessen `@claude review always`. `@claude review once` funktioniert weiterhin und verhält sich genauso wie der einfache Befehl.
</Note>

Damit einer dieser Befehle eine Überprüfung auslöst:

* Veröffentlichen Sie ihn als Top-Level-PR-Kommentar, nicht als Inline-Kommentar auf einer Diff-Zeile
* Setzen Sie den Befehl an den Anfang des Kommentars, mit `once` oder `always` auf der gleichen Zeile wie der Rest des Befehls
* Sie müssen Schreib-, Verwaltungs- oder Admin-Berechtigung auf dem Repository haben
* Der PR muss offen sein

Wenn das Repository einer Organisation gehört und Ihre Mitgliedschaft in dieser Organisation privat ist, was GitHub standardmäßig ist, identifiziert GitHub Sie Claude gegenüber nicht als Mitglied. Claude kann immer noch auf Ihren Kommentar mit 👀 reagieren, startet aber keine Überprüfung, es sei denn, Sie wurden dem Repository direkt als Mitarbeiter hinzugefügt, auch wenn ein Team oder die Basisberechtigungen der Organisation Ihnen Schreibzugriff geben. Um dies zu beheben, [machen Sie Ihre Organisationsmitgliedschaft öffentlich](https://docs.github.com/en/account-and-profile/setting-up-and-managing-your-personal-account-on-github/managing-your-membership-in-organizations/publicizing-or-hiding-organization-membership) oder bitten Sie einen Repository-Admin, Sie dem Repository als Mitarbeiter hinzuzufügen.

Im Gegensatz zu automatischen Triggern werden manuelle Trigger auf Entwurfs-PRs ausgeführt, da eine explizite Anfrage signalisiert, dass Sie die Überprüfung jetzt möchten, unabhängig vom Entwurfsstatus.

Wenn bereits eine Überprüfung auf diesem PR läuft, wird die Anfrage in die Warteschlange eingereiht, bis die laufende Überprüfung abgeschlossen ist. Sie können den Fortschritt über die Check-Run auf dem PR überwachen.

<h3 id="review-pull-requests-from-forks">
  Review-Pull-Requests von Forks
</h3>

Claude überprüft einen Pull-Request von einem Fork nicht automatisch, unabhängig von der Einstellung **Review-Verhalten** des Repositorys. Um einen zu starten, kommentieren Sie `@claude review` auf dem Pull-Request. Die [Anforderungen für Kommentarbefehle](#manually-trigger-reviews) gelten weiterhin, und der Schreibzugriff, den Sie benötigen, ist auf das Basis-Repository, nicht auf den Fork.

Um eine weitere Überprüfung eines Fork-Pull-Requests zu erhalten, veröffentlichen Sie einen neuen `@claude review`-Kommentar. `@claude review always` funktioniert auch, abonniert den Pull-Request aber nicht für Überprüfungen bei späteren Pushes. Nichts anderes als ein Kommentarbefehl startet eine Überprüfung auf einem Fork-Pull-Request:

* Das Klicken auf **Erneut ausführen** auf der Check-Run startet keine Überprüfung
* Das Pushen neuer Commits startet keine Überprüfung, auch nicht in einem Repository, das auf **Nach jedem Push** eingestellt ist

<h2 id="customize-reviews">
  Anpassung von Reviews
</h2>

Code Review liest zwei Dateien aus Ihrem Repository, um zu steuern, was es kennzeichnet. Sie unterscheiden sich darin, wie stark sie die Überprüfung beeinflussen:

* **`CLAUDE.md`**: gemeinsame Projektanweisungen, die Claude Code für alle Aufgaben verwendet, nicht nur für Reviews. Code Review liest sie als Projektkontext und kennzeichnet neu eingeführte Verstöße als Nits.
* **`REVIEW.md`**: Review-spezifische Anweisungen, die den Agenten gegeben werden, die Erkenntnisse finden und überprüfen, und von den Agenten konsultiert werden, die Erkenntnisse bewerten und melden. Verwenden Sie es, um zu sagen, was Ihr Team gekennzeichnet haben möchte, mit welchem Schweregrad und wie Erkenntnisse gemeldet werden.

<h3 id="claude-md">
  CLAUDE.md
</h3>

Code Review liest Ihre Repository-`CLAUDE.md` Dateien und behandelt neu eingeführte Verstöße als [Nit-Level](#severity-levels) Erkenntnisse. Dies funktioniert bidirektional: Wenn Ihr PR Code auf eine Weise ändert, die eine `CLAUDE.md` Aussage veraltet macht, kennzeichnet Claude, dass die Dokumentation aktualisiert werden muss.

Claude liest `CLAUDE.md` Dateien auf jeder Ebene Ihrer Verzeichnishierarchie, sodass Regeln in einer Unterverzeichnis-`CLAUDE.md` nur auf Dateien unter diesem Pfad angewendet werden. Weitere Informationen zur Funktionsweise von `CLAUDE.md` finden Sie in der [Memory-Dokumentation](/docs/de/memory).

Für Review-spezifische Anleitungen, die Sie nicht auf allgemeine Claude Code Sitzungen angewendet haben möchten, verwenden Sie stattdessen [`REVIEW.md`](#review-md).

<h3 id="review-md">
  REVIEW\.md
</h3>

`REVIEW.md` ist eine Datei in Ihrem Repository-Root, die Code Review auf Ihrem Repo anpasst. Die Agenten in der Review-Pipeline, die Erkenntnisse finden und überprüfen, erhalten ihren Inhalt als Ihre Repository-Review-Anweisungen, zusammen mit Code Reviews Standard-Review-Anleitung, und die Agenten, die Erkenntnisse bewerten und melden, konsultieren sie, bevor sie Schweregrad festlegen und die Überprüfung schreiben.

Setzen Sie die Regeln, die Sie durchgesetzt haben möchten, direkt in `REVIEW.md`.

<h4 id="what-you-can-tune">
  Was Sie optimieren können
</h4>

`REVIEW.md` ist freies Markdown, sodass alles, was Sie als Review-Anweisung ausdrücken können, im Umfang liegt. Die folgenden Muster haben die meiste praktische Auswirkung.

**Schweregrad**: Definieren Sie neu, was 🔴 Wichtig für Ihr Repo bedeutet. Die Standard-Kalibrierung zielt auf Produktionscode ab; ein Docs-Repo, ein Config-Repo oder ein Prototyp möchte möglicherweise eine viel engere Definition. Geben Sie explizit an, welche Klassen von Erkenntnissen Wichtig sind und welche höchstens Nit sind. Sie können auch in die andere Richtung eskalieren, zum Beispiel jeden `CLAUDE.md` Verstoß als Wichtig statt des Standard-Nits behandeln.

**Nit-Volumen**: Begrenzen Sie, wie viele 🟡 Nit-Kommentare eine einzelne Überprüfung veröffentlicht. Prosa- und Config-Dateien können für immer poliert werden. Eine Obergrenze wie 'höchstens fünf Nits melden, den Rest als Zählung in der Zusammenfassung erwähnen" hält Reviews umsetzbar.

**Skip-Regeln**: Listen Sie Pfade, Branch-Muster und Erkenntniskategorien auf, bei denen Claude keine Erkenntnisse veröffentlichen sollte. Häufige Kandidaten sind generierter Code, Lockfiles, vendorte Abhängigkeiten und maschinengeschriebene Branches, zusammen mit allem, das Ihr CI bereits durchsetzt, wie Linting oder Rechtschreibprüfung. Für Pfade, die einige Überprüfung verdienen, aber nicht vollständige Überprüfung, setzen Sie stattdessen eine höhere Messlatte: „in `scripts/`, nur melden, wenn nahezu sicher und schwerwiegend."

**Repo-spezifische Überprüfungen**: Fügen Sie Regeln hinzu, die Sie auf jedem PR gekennzeichnet haben möchten, wie „neue API-Routen müssen einen Integrationtest haben." Da `REVIEW.md` jeden Erkenntnisse-Findungs- und Verifizierungsagenten direkt erreicht, landen diese zuverlässiger als die gleichen Regeln in einem langen `CLAUDE.md`.

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

Dashboard-Kostenzahlen sind Schätzungen zur Überwachung der Aktivität. Für rechnungsgenaue Ausgaben beziehen Sie sich auf Ihre Anthropic-Rechnung.

<h2 id="pricing">
  Preisgestaltung
</h2>

Code Review wird basierend auf der Token-Nutzung abgerechnet. Jede Überprüfung kostet durchschnittlich \$15–25, skalierend mit PR-Größe, Codebasis-Komplexität und wie viele Probleme eine Überprüfung erfordern. Code Review-Nutzung wird separat über [Nutzungsguthaben](https://support.claude.com/de/articles/12429409-extra-usage-for-paid-claude-plans) abgerechnet und zählt nicht gegen die in Ihrem Plan enthaltene Nutzung.

Der Review-Trigger, den Sie wählen, beeinflusst die Gesamtkosten:

* **Once after PR creation**: wird einmal pro PR ausgeführt
* **After every push**: wird bei jedem Push ausgeführt, multipliziert die Kosten mit der Anzahl der Pushes
* **Manual**: keine Reviews bei offenen oder Push-Ereignissen, daher fallen Kosten nur bei Reviews an, die jemand anfordert

Im Modus „Once after PR creation" oder „Manual" führt das Kommentieren von `@claude review always` [den PR in Push-ausgelöste Reviews auf](#manually-trigger-reviews), sodass zusätzliche Kosten pro Push nach diesem Kommentar anfallen. Im Modus „After every push" lösen Pushes bereits Reviews aus, daher ändert sich das Abonnement nicht pro Push-Kosten. Das Kommentieren von `@claude review` führt eine einzelne Überprüfung aus, ohne sich für zukünftige Pushes zu abonnieren. Claude überprüft einen [Pull Request aus einem Fork](#review-pull-requests-from-forks) nur, wenn jemand `@claude review` kommentiert, daher fallen für einen Fork-Pull-Request in keinem Modus Pro-Push-Kosten an.

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

Um die Überprüfung erneut auszuführen, kommentieren Sie `@claude review` auf dem PR. Dies startet eine neue Überprüfung, ohne den PR für zukünftige Pushes zu abonnieren. Wenn der PR nicht [von einem Fork](#review-pull-requests-from-forks) stammt, können Sie stattdessen auf **Re-run** in der **Claude Code Review** Check-Run in Githubs Checks-Registerkarte klicken. Ein erneuter Durchlauf startet auch eine neue Überprüfung, ohne den PR zu abonnieren.

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

Der [`/code-review` Befehl](/docs/de/commands) überprüft einen Diff in Ihrem Terminal ohne Installation der GitHub App. Er meldet Korrektheitsfehler und Wiederverwendung, Vereinfachung und Effizienz-Bereinigungen.

`/review` ist ein Alias von `/code-review`; vor v2.1.223 war es ein separater Befehl, der eine einmalige, schreibgeschützte Überprüfung eines GitHub Pull Request durchführte.

<Steps>
  <Step title="Führen Sie /code-review aus">
    Führen Sie in der Sitzung, in der Sie arbeiten, den Befehl aus:

    ```text theme={null}
    /code-review
    ```

    Er überprüft die Commits Ihres Branches vor seinem Upstream plus alle nicht committeten Änderungen, daher benötigt er Arbeit auf dem Branch oder im Arbeitsbaum, um etwas zu melden. Um etwas anderes zu überprüfen, übergeben Sie ein Ziel: einen Dateipfad, eine PR-Nummer, einen Branch-Namen oder einen Ref-Bereich wie `main...my-feature`.

    Sie können auch Flags hinzufügen:

    * `--fix`: wendet die Erkenntnisse auf Ihren Arbeitsbaum an, nachdem die Überprüfung abgeschlossen ist
    * `--comment`: veröffentlicht die Erkenntnisse als Inline-Kommentare auf einem GitHub Pull Request oder auf einer GitLab Merge Request als einzelne Notiz
    * `--post`: bei einer `ultra` Cloud-Überprüfung eines `github.com` Pull Request wählt es das Veröffentlichen der abgeschlossenen Erkenntnisse zum PR im Startdialog vor; siehe [Erkenntnisse zum Pull Request veröffentlichen](/docs/de/ultrareview#post-findings-to-the-pull-request). Erfordert Claude Code v2.1.227 oder später

    Wenn Sie `--comment` für eine GitLab Merge Request übergeben, veröffentlicht Claude Code die Erkenntnisse über GitLabs `glab` CLI. Erfordert Claude Code v2.1.257 oder später. Wenn `glab` nicht installiert ist, druckt Claude die Erkenntnisse stattdessen im Terminal.

    Übergeben Sie die Merge Request als ihre URL oder eine `!123` Referenz. Claude Code behandelt eine bloße Nummer oder einen Branch-Namen als Merge Request nur, wenn der Checkout-Ursprung auf `gitlab.com` liegt. Auf einer selbstverwalteten GitLab-Instanz übergeben Sie die URL oder das `!123` Format.
  </Step>

  <Step title="Arbeiten Sie weiter">
    Die Überprüfung läuft als Hintergrund-[Subagent](/docs/de/sub-agents) mit seinem eigenen Kontextfenster, daher füllt sie Ihre Konversation nicht. Die Erkenntnisse treffen in Ihrer Konversation ein, wenn die Überprüfung abgeschlossen ist.
  </Step>

  <Step title="Handeln Sie nach den Erkenntnissen">
    Bitten Sie Claude, das zu beheben, das die Überprüfung gefunden hat. Wenn Sie `--fix` oder `--comment` übergeben haben, hat die Überprüfung ihre Erkenntnisse bereits angewendet oder veröffentlicht.
  </Step>
</Steps>

Claude meldet die Erkenntnisse als Text in der Antwort in beiden dieser Läufe, auch wenn eine Host-Anwendung eine Erkenntnisliste anfordert:

* In einer Terminal-Sitzung, in der `/code-review` die Überprüfung als [verzweigten Subagent](/docs/de/skills#run-skills-in-a-subagent) ausführt
* In einem `-p` Lauf mit Text- oder JSON-Ausgabe

In einer Host-Anwendung, die die Erkenntnisliste anfordert, wie die [Desktop-App](/docs/de/desktop), meldet Claude die Überprüfungsergebnisse über das [`ReportFindings` Tool](/docs/de/tools-reference). Claude Code rendert den Bericht als Erkenntnisliste, und jeder Eintrag zeigt den Dateispeicherort, eine einzeilige Zusammenfassung und ein Kategorie-Tag wie `correctness`, wenn die Erkenntnis eines trägt. Eine Host-Anfrage gilt auf jeder Aufwandsebene und erfordert Claude Code v2.1.218 oder später.

Wenn Claude gemeldete Erkenntnisse später in der Sitzung behebt, meldet es sie erneut, und Claude Code markiert jede Erkenntnis in der aktualisierten Erkenntnisliste als behoben, übersprungen oder keine Änderung erforderlich.

<h3 id="what-the-review-reads-and-edits">
  Was die Überprüfung liest und bearbeitet
</h3>

Die Überprüfung folgt Ihrer `CLAUDE.md` wie jede Claude Code Sitzung, aber sie liest nicht [`REVIEW.md`](#review-md). Eine Hintergrund-Überprüfung wendet ihre `--fix` Bearbeitungen außerhalb der [Checkpoints](/docs/de/checkpointing#subagent-edits-not-restored) Ihrer Sitzung an, daher macht `/rewind` sie nicht rückgängig; verwenden Sie git, um sie rückgängig zu machen. Wenn die Überprüfung [im Vordergrund läuft](#run-in-the-foreground), bearbeitet sie Ihren Arbeitsbaum während Ihres eigenen Zuges, daher stellt `/rewind` ihre Bearbeitungen wie gewohnt wieder her.

<h3 id="tune-effort-and-arguments">
  Passen Sie Aufwand und Argumente an
</h3>

Übergeben Sie eine [Aufwandsebene](/docs/de/model-config#adjust-effort-level), um Abdeckung gegen Vertrauen zu tauschen. Bei `low` und `medium` meldet die Überprüfung nur die Erkenntnisse, bei denen sie am sichersten ist, daher sehen Sie weniger falsch positive Ergebnisse; `high` bis `max` erweitern die Abdeckung und können Erkenntnisse einschließen, bei denen die Überprüfung weniger sicher ist.

Wenn Sie keine Ebene eingeben, verwendet die Überprüfung die letzte Ebene von `low` bis `max`, die Sie eingegeben haben, sogar in einer früheren Sitzung, und Claude Code zeigt einen Hinweis wie `Reusing high effort, the level you typed last time`. Geben Sie eine Ebene ein, wie `/code-review high`, um zu ändern, was spätere Läufe wiederverwenden; eine Ebene, die Sie in einem nicht-interaktiven `-p` Lauf übergeben, aktualisiert sie nicht. `ultra` aktualisiert und verwendet die erinnerte Ebene nicht. Wenn Sie noch nie eine Ebene eingegeben haben, verwendet die Überprüfung die aktuelle Aufwandsebene der Sitzung. Vor v2.1.223 verwendete ein `/code-review` ohne Ebene immer die aktuelle Aufwandsebene der Sitzung.

Nach der Aufwandsebene und den Flags liest Claude Code den Rest der Zeile auf eine von zwei Arten:

* **Ohne `ultra`**: alles Verbleibende ist das Überprüfungsziel, auch wenn es mit einem anderen Befehlsnamen beginnt. `/code-review /fix-issue 123` überprüft mit `/fix-issue 123` als Zieltext, anstatt `/fix-issue` als zweite [gestapelte Skill](/docs/de/skills#pass-arguments-to-skills) zu laden. Vor v2.1.218 erweiterte sich ein Befehl, der nach `/code-review` gestapelt wurde, als seine eigene Skill.
* **Mit `ultra`**: Claude Code liest ein einzelnes Wort als Basis-Branch oder PR-Nummer und verwandelt längeren Text, der keinen Branch oder PR benennt, in [eine an die Überprüfung angehängte Notiz](/docs/de/ultrareview#pass-a-request-in-plain-words). `/code-review ultra check my auth changes` überprüft Ihren aktuellen Branch, und Claude bezieht die Erkenntnisse auf Ihre Notiz.

<h3 id="run-in-the-foreground">
  Im Vordergrund ausführen
</h3>

Die Überprüfung läuft standardmäßig im Hintergrund; vor v2.1.218 lief sie in Ihrer Konversation. Sie läuft stattdessen im Vordergrund in Fällen wie diesen:

* Sie führen `/code-review` erneut aus, während eine frühere Überprüfung noch läuft
* Sie führen es im nicht-interaktiven Modus aus, mit dem `-p` Flag oder dem Agent SDK; Claude Code wartet auf die Überprüfung und bezieht die Erkenntnisse in die Antwort ein, außer für `ultra`, das [die Cloud-Überprüfung ohne Warten startet](#escalate-to-ultrareview)
* Sie setzen [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS`](/docs/de/env-vars) auf `1`, was auch jede andere Hintergrund-Task-Funktion ausschaltet

<h3 id="let-claude-start-the-review">
  Lassen Sie Claude die Überprüfung starten
</h3>

Claude kann `/code-review` von selbst starten. Bitten Sie es, Ihre Änderungen in einfacher Sprache zu überprüfen, und es kann die Skill ausführen, ohne dass Sie den Befehl eingeben, und eine [geplante Task](/docs/de/scheduled-tasks) mit `/code-review` als Prompt führt die Überprüfung aus.

Eine geplante Task startet niemals die [Cloud-Überprüfung](#escalate-to-ultrareview), daher planen Sie `/code-review` ohne das `ultra` Argument.

Um sowohl Claude als auch geplante Tasks davon abzuhalten, die Überprüfung zu starten, während Sie `/code-review` zum Eingeben verfügbar halten, fügen Sie einen [`skillOverrides`](/docs/de/skills#override-skill-visibility-from-settings) Eintrag zu einer [Einstellungsdatei](/docs/de/settings#where-settings-live) wie `~/.claude/settings.json` hinzu:

```json theme={null}
{
  "skillOverrides": {
    "code-review": "user-invocable-only"
  }
}
```

Vor v2.1.246 startete Claude `/code-review` von selbst nur dort, wo ein von Anthropic abgerufenes Feature Flag es einschaltete. In [Sitzungen, die keine Feature Flags abrufen](/docs/de/env-vars#features-that-need-feature-flag-fetching), lief `/code-review` nur, wenn Sie es eingegeben haben, und ein geplantes `/code-review` erreichte Claude als einfacher Text.

<h3 id="escalate-to-ultrareview">
  Eskalieren Sie zu Ultrareview
</h3>

`/code-review ultra --fix` führt die tiefere [Ultrareview](/docs/de/ultrareview) in der Cloud aus, dann wendet ihre Erkenntnisse auf Ihren Arbeitsbaum an, wenn sie in Ihrer Sitzung zurückkommen.

Ultrareview verwendet seinen eigenen Umfang: Ihren aktuellen Branch gegen den Standard-Branch des Repositorys, plus nicht committete und gestaged Änderungen im Arbeitsbaum. Für nicht committete Änderungen an Dateien, die wie Anmeldedaten oder Schlüssel benannt sind, wie `.env` und `*.tfvars` Dateien, folgt Claude Code den Regeln für [Hochladen eines lokalen Repositorys zu einer Cloud-Sitzung](/docs/de/claude-code-on-the-web#send-local-repositories-without-github). Übergeben Sie einen Branch-Namen, wie `/code-review ultra develop`, um gegen eine andere Basis zu vergleichen.

Wenn das Ziel ein `github.com` Pull Request ist, können Sie Claude [die abgeschlossenen Erkenntnisse zum PR](/docs/de/ultrareview#post-findings-to-the-pull-request) als Kommentar von Ihrem GitHub-Konto veröffentlichen lassen. Erfordert Claude Code v2.1.227 oder später.

<Note>
  Ultrareview erfordert Authentifizierung mit einem claude.ai Konto und ist nicht auf Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry verfügbar, oder für Organisationen mit Zero Data Retention aktiviert. Wenn Ultrareview nicht verfügbar ist, führt `/code-review ultra` stattdessen eine lokale Überprüfung in Ihrer Sitzung aus.
</Note>

Um eine Cloud-Überprüfung von einem Skript oder CI aus zu starten, führen Sie `claude -p '/code-review ultra'` aus. Claude Code startet die Überprüfung und druckt einen Link zu ihrer Verfolgung. Erfordert Claude Code v2.1.218 oder später.

Wenn die Überprüfung [Nutzungsguthaben](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) abrechnen würde, stoppt Claude Code vorher, da die Abrechnungsbestätigung eine interaktive Sitzung benötigt. Führen Sie stattdessen den [`claude ultrareview` Unterbefehl](/docs/de/ultrareview#run-ultrareview-non-interactively) aus; durch seine Ausführung stimmen Sie der Gebühr zu.

Der Befehl hieß vor v2.1.147 `/simplify`, als er Fixes standardmäßig anwendete. `/simplify` führt eine separate Bereinigung-nur-Überprüfung aus, die Fixes anwendet, ohne nach Fehlern zu suchen. Wenn Sie `/simplify` für die Fehlersuche skriptet haben, wechseln Sie zu `/code-review --fix`.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Befehle](/docs/de/commands): führen Sie `/code-review` in einer lokalen Claude Code Sitzung aus, um einen Diff vor dem Pushen zu überprüfen
* [GitHub Actions](/docs/de/github-actions): führen Sie Claude in Ihren eigenen GitHub Actions Workflows aus für benutzerdefinierte Automatisierung über Code Review hinaus
* [GitLab CI/CD](/docs/de/gitlab-ci-cd): selbst gehostete Claude-Integration für GitLab-Pipelines
* [Memory](/docs/de/memory): wie `CLAUDE.md` Dateien über Claude Code funktionieren
* [Analytics](/docs/de/analytics): verfolgen Sie Claude Code Nutzung über Code Review hinaus
* [How Anthropic secures its AI-native software development lifecycle](https://claude.com/blog/how-anthropic-secures-its-ai-native-software-development-lifecycle): wie automatisierte Reviews als eine Schicht von Anthropics sicherem Entwicklungsprozess funktionieren
