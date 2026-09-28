> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Bugs mit Ultrareview finden

> Führen Sie eine tiefe, Multi-Agent-Code-Review in der Cloud mit /code-review ultra durch, um Bugs vor dem Merge zu finden und zu verifizieren.

<Note>
  Ultrareview ist eine Research-Preview-Funktion. Die Funktion, Preisgestaltung und Verfügbarkeit können sich basierend auf Feedback ändern. Der Befehl ist `/code-review ultra`. Wenn Ultrareview für Ihr Konto verfügbar ist, ist `/ultrareview` ein Alias.
</Note>

Ultrareview ist eine tiefe Code-Review, die als [Cloud-Sitzung](/docs/de/claude-code-on-the-web) auf der Infrastruktur von Anthropic ausgeführt wird. Wenn Sie `/code-review ultra` ausführen, startet Claude Code eine Flotte von Reviewer-Agenten in einer Cloud-Sandbox, um Bugs in Ihrem Branch oder Pull Request zu finden.

Im Vergleich zu einer lokalen `/code-review` bietet Ultrareview:

* **Höhere Signalqualität**: Jeder gemeldete Fund wird unabhängig reproduziert und verifiziert, sodass sich die Ergebnisse auf echte Bugs konzentrieren und nicht auf Stilvorschläge
* **Breitere Abdeckung**: Eine größere Flotte von Reviewer-Agenten erkundet die Änderung parallel, was Probleme aufdeckt, die eine lokale Review übersehen könnte
* **Keine lokale Ressourcennutzung**: Die Review läuft vollständig in einer Cloud-Sandbox, sodass Ihr Terminal für andere Arbeiten frei bleibt, während sie läuft

Ultrareview erfordert eine Authentifizierung mit einem claude.ai-Konto, da es als Cloud-Sitzung auf der Infrastruktur von Anthropic ausgeführt wird. Wenn Sie nur mit einem API-Schlüssel angemeldet sind, führen Sie `/login` aus und authentifizieren Sie sich zuerst mit claude.ai. Ultrareview ist nicht verfügbar, wenn Sie Claude Code mit Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry verwenden, und es ist nicht für Organisationen verfügbar, die Zero Data Retention aktiviert haben. Wenn Ultrareview nicht verfügbar ist, führt `/code-review ultra` stattdessen eine lokale Review in Ihrer Sitzung aus.

<h2 id="run-ultrareview-from-the-cli">
  Ultrareview von der CLI ausführen
</h2>

Starten Sie eine Review aus einem beliebigen Git-Repository:

```text theme={null}
/code-review ultra
```

Ohne Argumente überprüft Ultrareview den Diff zwischen Ihrem aktuellen Branch und dem Standard-Branch, einschließlich aller nicht committeter und gestaged Changes. Für nicht committete Änderungen an Dateien mit Namen wie Credentials oder Keys, wie `.env` und `*.tfvars` Dateien, folgt Claude Code den Regeln für [Hochladen eines lokalen Repositories in eine Cloud-Sitzung](/docs/de/claude-code-on-the-web#send-local-repositories-without-github).

Für eine Branch-Review bündelt Claude Code den Repository-Status und lädt ihn in eine Remote-Sandbox hoch; wenn Sie [einen Pull Request überprüfen](#review-a-pull-request), lädt Claude Code nichts von Ihrem Computer hoch.

Vor dem Start zeigt Claude Code einen Bestätigungsdialog mit dem Review-Umfang, Ihren verbleibenden kostenlosen Durchläufen und den geschätzten Kosten an; für eine Branch-Review umfasst der Umfang die Datei- und Zeilenanzahl. Nach der Bestätigung läuft die Review im Hintergrund weiter, während Sie Ihre Sitzung weiterhin nutzen.

Der Befehl wird nur ausgeführt, wenn Sie ihn mit `/code-review ultra` aufrufen; Claude startet nicht automatisch eine Ultrareview.

<h3 id="review-against-a-different-base">
  Review gegen eine andere Basis
</h3>

Um gegen eine andere Basis als den Standard-Branch zu vergleichen, übergeben Sie den Branch-Namen. Dieses Beispiel überprüft Ihren aktuellen Branch gegen `develop` statt gegen den Standard-Branch:

```text theme={null}
/code-review ultra develop
```

Der Basis-Branch muss nicht in Ihrem lokalen Clone vorhanden sein; Claude Code ruft ihn von `origin` ab. Wenn der Name einen Tippfehler enthält, schlägt Claude Code den nächstgelegenen Branch-Namen im Fehler vor.

Ein Commit-ID oder Tag funktioniert auch als Basis, und die Review deckt dann die Änderungen auf Ihrem Branch seit diesem Commit ab.

<h3 id="review-a-pull-request">
  Review eines Pull Requests
</h3>

Um einen GitHub Pull Request statt eines lokalen Branches zu überprüfen, übergeben Sie die PR-Nummer:

```text theme={null}
/code-review ultra 1234
```

Der Befehl akzeptiert auch `#1234`, `PR 1234` und eingefügte PR-URLs; eine eingefügte URL muss auf das Repository in Ihrem aktuellen Verzeichnis verweisen.

Im PR-Modus klont die Remote-Sandbox den Pull Request direkt vom Host, anstatt Ihren lokalen Working Tree zu bündeln. Der PR-Modus funktioniert mit Repositories auf `github.com` und auf [GitHub Enterprise Server](/docs/de/github-enterprise-server)-Instanzen, die ein Administrator mit Claude Code verbunden hat.

Für Repositories auf `github.com` klont die Sandbox mit dem GitHub-Konto, das mit Ihrem Claude-Konto verbunden ist, daher muss das Konto den PR-Repository lesen können. Claude Code überprüft dies vor dem Erstellen der Cloud-Sitzung, es sei denn, Sie haben [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/de/env-vars#variables) gesetzt, und lehnt den Start ab, wenn [kein Konto verbunden ist](/docs/de/errors#no-github-account-is-connected-to-your-claude-account) oder [das Konto das Repository nicht sehen kann](/docs/de/errors#your-connected-github-account-cant-see-the-repository); die Ablehnung nennt die Lösung. Vor v2.1.248 überprüfte Claude Code dies nicht vor dem Start.

Führen Sie [`/web-setup`](/docs/de/web-quickstart#connect-from-your-terminal) aus, um Ihren GitHub CLI-Login mit Ihrem Claude-Konto zu verbinden.

<h3 id="post-findings-to-the-pull-request">
  Ergebnisse im Pull Request posten
</h3>

In Claude Code v2.1.227 oder später können Sie, wenn Sie einen Pull Request auf `github.com` überprüfen, Claude die fertigen Ergebnisse als einzelnen einfachen Kommentar von Ihrem eigenen GitHub-Konto im PR posten lassen. Der Kommentar ist keine Review oder Genehmigung und endet mit einer Notiz „Generated by Claude Code". Wenn Sie einen Branch oder einen GitHub Enterprise Server Pull Request überprüfen, zeigt Claude Code die Ergebnisse nur in Ihrer Sitzung an.

Claude Code postet niemals, es sei denn, Sie wählen dies für diesen Durchlauf, und `--no-post` ist die Standardeinstellung. Das Posten ist eine Wahl, die Sie für jeden Durchlauf treffen:

* **Interaktiv**: Wählen Sie im Startdialog **Run and post the findings to the PR as me** aus. Wenn Sie `--post` zum Befehl hinzufügen, wie in `/code-review ultra 1234 --post`, wählt Claude Code diese Option vor und fragt trotzdem vor dem Start.
* **Nicht-interaktiv**: Führen Sie den [`claude ultrareview` Subbefehl](#run-ultrareview-non-interactively) mit `--post` aus. Sie stimmen dem Posten zu, indem Sie den Subbefehl mit dem Flag ausführen, daher postet Claude Code ohne zu fragen. In einem `claude -p '/code-review ultra'` Durchlauf beendet Claude Code sich, bevor die Ergebnisse ankommen, daher postet es nichts; verwenden Sie stattdessen den Subbefehl.

Claude Code postet nicht von Ihrem Computer. Es sendet die Review-Sitzungs-ID an die Anthropic API, die die gespeicherten Ergebnisse der Review als Kommentar über das GitHub-Konto postet, das Sie mit Claude verbunden haben. Das Posten erfordert die gleiche claude.ai-Anmeldung wie die Review selbst, und es ist nicht auf Drittanbieter verfügbar oder wenn Sie [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/de/env-vars) gesetzt haben.

In einer interaktiven Sitzung startet Claude Code das Posten, wenn die Ergebnisse ankommen, daher halten Sie die Sitzung offen, bis die Review endet. Claude Code behält die Posten-Wahl nur in dieser Sitzung. Wenn die Sitzung endet, bevor die Review endet, postet Claude Code nichts, auch wenn Sie das Gespräch später fortsetzen.

Wenn das Posten endet, teilt Claude Ihnen das Ergebnis mit:

* **Gepostet**: Claude gibt Ihnen einen Link zum Kommentar.
* **Bereits gepostet**: Ein früheres Posten der gleichen Review hat den Kommentar bereits auf dem PR platziert, daher verlinkt Claude Sie stattdessen zum Pull Request, anstatt erneut zu posten.
* **Fehlgeschlagen**: Claude teilt Ihnen mit, warum, und die Ergebnisse bleiben in Ihrem Terminal, damit Sie sie manuell posten können.

<h3 id="pass-a-request-in-plain-words">
  Eine Anfrage in einfachen Worten übergeben
</h3>

In Claude Code v2.1.218 oder später können Sie auch beschreiben, woran Sie arbeiten, in einfachen Worten:

```text theme={null}
/code-review ultra check my auth changes
```

Die Review deckt immer noch Ihren aktuellen Branch ab, den gleichen Umfang wie das Ausführen ohne Argument. Claude behält Ihren Text als Notiz, die im Startdialog angezeigt wird, und bezieht die Ergebnisse darauf, wenn sie ankommen.

Claude Code behandelt Ihren Text nur als Notiz, wenn er mehr als ein Wort hat und kein Branch-Name oder PR-Verweis ist. Es liest ein einzelnes Wort als Branch-Namen oder PR-Verweis, daher erhält ein fehlerhafter Branch-Name den Fehler des nächstgelegenen Branches von [Review gegen eine andere Basis](#review-against-a-different-base) statt mit einer Notiz zu starten. Wenn Ihr Text einen PR-Verweis mit anderen Worten kombiniert, wie `check PR 123 again`, startet Claude Code auch nicht; es fordert Sie auf, mit nur der PR-Nummer erneut auszuführen, um diesen PR zu überprüfen, oder ohne den Verweis, um Ihren aktuellen Branch zu überprüfen.

<Tip>
  Wenn Ihr Repository zu groß zum Bündeln ist, fordert Claude Code Sie auf, stattdessen den PR-Modus zu verwenden. Pushen Sie Ihren Branch und öffnen Sie einen Draft PR, führen Sie dann `/code-review ultra <PR-number>` aus.
</Tip>

<h3 id="diff-limits-and-fallbacks">
  Diff-Limits und Fallbacks
</h3>

Ultrareview überprüft den Diff, bevor irgendwelche Review-Arbeiten ausgeführt werden, und teilt Ihnen mit, wenn es ihn nicht wie vorhanden überprüfen kann:

* **Diff zu groß**: Eine Branch-Review kann standardmäßig bis zu 500 geänderte Dateien und 8.000 geänderte Zeilen enthalten. Die genauen Werte können sich ändern, und die [Ablehnung](/docs/de/errors#diff-is-too-large-for-ultrareview) nennt die geltenden Werte, die Größe Ihres Diffs und die Dateien mit den meisten geänderten Zeilen. Claude Code lehnt einen zu großen Pull Request auf die gleiche Weise ab und nennt seine Datei- und Zeilenanzahl, aber nicht die Aufschlüsselung pro Datei
* **Nichts zu überprüfen**: Wenn der Diff gegen die Basis leer ist, weigert sich Ultrareview und nennt den Branch oder Commit, gegen den es verglichen wurde, und den Fall, in dem Sie sich befinden, wie z. B. auf dem Basis-Branch selbst mit nichts Uncommitted, oder ein Branch, dessen Commits alle bereits Teil der Basis sind. Es schlägt auch den Weg aus diesem Fall vor, wie z. B. zum Branch mit Ihrer Arbeit wechseln, lokale Änderungen stagen oder committen, oder eine andere Basis übergeben
* **Erster Commit**: Der erste Commit eines Repositories hat nichts Früheres zum Vergleichen, daher überprüft Ultrareview jede Datei darin, nachdem Sie im Startdialog bestätigt haben. Wenn Sie unverfolgbare Dateien haben, weigert es sich stattdessen und teilt Ihnen mit, die Dateien zu `git add`, die Sie überprüft haben möchten. Die gleichen Größenlimits gelten.

  Ein erster Commit wird nur nach dieser Bestätigung ganz überprüft, daher weigern sich der `claude ultrareview` Subbefehl und `claude -p` und verweisen Sie auf eine interaktive Sitzung stattdessen. Erfordert Claude Code v2.1.277 oder später
* **Keine Merge-Basis**: Wenn Ihr Branch keine Historie mit dem Basis-Branch teilt, oder das Repository keinen Basis-Branch zum Vergleichen hat, überprüft Ultrareview stattdessen jede verfolgbare Datei im Repository. Das Fallback erfordert einen vollständigen Clone und wendet die gleichen Größenlimits an. Es wird nur gestartet, wenn Sie im Startdialog bestätigen oder den `claude ultrareview` Subbefehl selbst ausführen. In `claude -p` und überall sonst, wo keines davon passiert, weigert sich Ultrareview, sagt, die Review würde jede Datei abdecken, und verweist Sie auf eine interaktive Sitzung.

  Bei einem Checkout ohne Branches oder andere Refs, wie ein detached HEAD, der durch Auschecken von `FETCH_HEAD` nach dem Abrufen einer URL erstellt wurde, [lehnt Claude Code die Review ab](/docs/de/errors#your-checkout-has-no-branches) und schlägt vor, zuerst einen Branch zu erstellen

<h2 id="pricing-and-free-runs">
  Preisgestaltung und kostenlose Durchläufe
</h2>

Ultrareview ist eine Premium-Funktion, die gegen Nutzungsguthaben statt gegen die in Ihrem Plan enthaltene Nutzung abgerechnet wird.

| Plan                | Kostenlose Durchläufe enthalten | Nach kostenlosen Durchläufen                                                                                          |
| ------------------- | ------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| Pro                 | 3 kostenlose Durchläufe         | abgerechnet als [Nutzungsguthaben](https://support.claude.com/de/articles/12429409-extra-usage-for-paid-claude-plans) |
| Max                 | 3 kostenlose Durchläufe         | abgerechnet als [Nutzungsguthaben](https://support.claude.com/de/articles/12429409-extra-usage-for-paid-claude-plans) |
| Team und Enterprise | keine                           | abgerechnet als [Nutzungsguthaben](https://support.claude.com/de/articles/12429409-extra-usage-for-paid-claude-plans) |

* **Kostenlose Durchläufe**: Die drei Pro- und Max-Durchläufe sind eine einmalige Zuteilung pro Konto und werden nicht erneuert.
* **Kosten pro Review**: Nach Verwendung der kostenlosen Durchläufe typischerweise 5 bis 25 Dollar in Nutzungsguthaben, je nach Größe der Änderung, entsprechend der Schätzung, die der Startdialog vor jedem Durchlauf anzeigt.
* **Wann ein Durchlauf zählt**: sobald die Cloud-Sitzung startet. Eine Review, die Sie frühzeitig beenden oder die nicht vollständig abgeschlossen wird, verbraucht immer noch einen kostenlosen Durchlauf; eine kostenpflichtige Review wird nur für den Teil abgerechnet, der ausgeführt wurde.

Da Ultrareview außerhalb der kostenlosen Durchläufe immer als Nutzungsguthaben abgerechnet wird, muss Ihr Konto oder Ihre Organisation Nutzungsguthaben aktiviert haben, bevor Sie eine kostenpflichtige Review starten können. Wenn Nutzungsguthaben nicht aktiviert sind, blockiert Claude Code den Start, und wie Sie diese aktivieren, hängt von Ihrem Abrechnungszugriff ab:

* Wenn Sie die Abrechnung für Ihr Konto verwalten können, verlinkt Claude Code Sie zu den Abrechnungseinstellungen, wo Sie Nutzungsguthaben aktivieren können.
* Bei Team- und Enterprise-Plänen senden Mitglieder ohne Abrechnungszugriff eine Anfrage über die CLI, in der sie ihren Administrator bitten, Nutzungsguthaben zu aktivieren.

Sie können auch `/usage-credits` ausführen, um Ihre Nutzungsguthaben-Einstellung zu überprüfen oder zu ändern.

Claude Code fordert Sie einmal pro Konversation auf, die Abrechnung von Nutzungsguthaben zu bestätigen: Wenn Sie eine neue Konversation starten, beispielsweise mit `/clear`, zeigt Claude Code die Bestätigung erneut für die nächste kostenpflichtige Review an.

<h2 id="track-a-running-review">
  Eine laufende Review verfolgen
</h2>

Eine Review dauert normalerweise 5 bis 10 Minuten. Die Review läuft als Hintergrundaufgabe, sodass Sie in Ihrer Sitzung weiterarbeiten, andere Befehle starten oder das Terminal vollständig schließen können. Wenn Sie sich dafür entschieden haben, [die Ergebnisse in der Pull-Anfrage zu veröffentlichen](#post-findings-to-the-pull-request), halten Sie die Sitzung offen, bis die Review abgeschlossen ist. Wenn die Sitzung zuerst endet, veröffentlicht Claude Code nichts.

Verwenden Sie `/tasks`, um laufende und abgeschlossene Reviews anzuzeigen, die Detailansicht für eine Review zu öffnen oder eine laufende Review zu stoppen. Wenn Sie eine Review stoppen, archiviert Claude Code die Cloud-Sitzung und gibt keine teilweisen Ergebnisse zurück.

Claude kann Ihnen auch mitteilen, dass eine Review gestoppt wurde oder dass ihre Sitzung nicht gefunden wurde:

* Wenn die Cloud-Sitzung der Review gestoppt oder [archiviert](/docs/de/claude-code-on-the-web#archive-sessions) auf claude.ai wird, bevor die Review abgeschlossen ist, teilt Claude Ihnen mit, dass sie gestoppt wurde.
* Wenn die Cloud-Sitzung der Review gelöscht wurde oder Sie sich seit dem Start bei einem anderen Claude-Konto oder einer anderen Organisation angemeldet haben, teilt Claude Ihnen mit, dass die Sitzung nicht gefunden wurde.
* Wenn Sie die Konten gewechselt haben, kann die Review möglicherweise immer noch unter dem Konto abgeschlossen werden, das sie gestartet hat. Wenn die Review noch läuft, melden Sie sich erneut als dieses Konto an und setzen Sie das Gespräch mit `claude --resume` fort, um es erneut anzuhängen.

Wenn die Review abgeschlossen ist, zeigt Claude Code die verifizierten Ergebnisse als Benachrichtigung in Ihrer Sitzung an. Jedes Ergebnis enthält den Dateispeicherort und eine Erklärung des Problems, sodass Sie Claude direkt bitten können, es zu beheben.

<h2 id="run-ultrareview-non-interactively">
  Ultrareview nicht-interaktiv ausführen
</h2>

Verwenden Sie den Unterbefehl `claude ultrareview`, um eine Ultrareview von CI oder einem Skript ohne eine interaktive Sitzung zu starten. Der Unterbefehl startet die gleiche Review wie `/code-review ultra`, blockiert, bis die Remote-Review abgeschlossen ist, und gibt die Ergebnisse auf stdout aus.

```bash theme={null}
claude ultrareview
claude ultrareview 1234
claude ultrareview origin/main
```

Ohne Argumente überprüft der Unterbefehl den Diff zwischen Ihrem aktuellen Branch und dem Standard-Branch, mit dem gleichen [Fallback für das gesamte Repository](#diff-limits-and-fallbacks) wie `/code-review ultra`, wenn keine Merge-Basis vorhanden ist. Übergeben Sie eine PR-Nummer, um einen Pull Request zu überprüfen, oder einen Base-Branch, um dagegen zu überprüfen; die [Behandlung des Base-Branch](#review-against-a-different-base) entspricht dem interaktiven Befehl.

Sie stimmen dem Fallback für das gesamte Repository und der Abrechnungs- und Bedingungseingabeaufforderung zu, wenn Sie den Unterbefehl ausführen, sodass die Ausführung ohne Warten auf Eingaben startet. Das Ausführen selbst ist das, was als Zustimmung zählt. Wenn Claude den Unterbefehl stattdessen für Sie ausführt, beispielsweise über das Bash-Tool, lehnt Claude Code die Review des gesamten Repositorys ab.

In Claude Code v2.1.218 oder später können Sie die Cloud-Review auch starten, indem Sie `/code-review ultra` in einer nicht-interaktiven Sitzung ausführen, beispielsweise `claude -p '/code-review ultra'`. Claude Code startet die Review und gibt einen Tracking-Link aus, ohne auf die Ergebnisse zu warten, im Gegensatz zu `claude ultrareview`, das blockiert, bis diese ankommen. Wenn die Review Nutzungsguthaben abrechnen würde, stoppt Claude Code vor dem Start und verweist Sie auf `claude ultrareview`, da die Abrechnungsbestätigung eine interaktive Sitzung benötigt. Vor v2.1.218 führte `/code-review ultra` in einer nicht-interaktiven Sitzung eine lokale Review aus.

Fortschrittsmeldungen und die Live-Sitzungs-URL gehen zu stderr, sodass stdout analysierbar bleibt. Verwenden Sie diese Flags, um die Ausgabe, das Timeout und ob die Ergebnisse gepostet werden sollen, zu steuern:

| Flag                  | Beschreibung                                                                                                                                                                                                                                                                                                        |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--json`              | Geben Sie die rohe `bugs.json`-Nutzlast statt der formatierten Ergebnisse aus                                                                                                                                                                                                                                       |
| `--timeout <minutes>` | Maximale Minuten zum Warten auf den Abschluss der Review. Standard ist 45                                                                                                                                                                                                                                           |
| `--post`              | [Posten Sie die fertigen Ergebnisse](#post-findings-to-the-pull-request) als einen einfachen Kommentar aus Ihrem GitHub-Konto zum Pull Request. Funktioniert bei `github.com` Pull-Request-Zielen; bei anderen Zielen ignoriert Claude Code das Flag und teilt dies mit. Erfordert Claude Code v2.1.227 oder später |
| `--no-post`           | Posten Sie die Ergebnisse nicht. Dies ist die Standardeinstellung, und wenn Sie beide Flags übergeben, postet Claude Code nicht. Erfordert Claude Code v2.1.227 oder später                                                                                                                                         |

Das Ausführen von `claude ultrareview` erfordert die gleiche Authentifizierung und Nutzungsguthaben-Konfiguration wie `/code-review ultra`.

Der Unterbefehl beendet sich mit einem von drei Codes:

* **0**: Die Review wurde abgeschlossen, mit oder ohne Ergebnisse
* **1**: Die Review konnte nicht gestartet werden, die Cloud-Sitzung ist fehlgeschlagen, oder das Timeout ist abgelaufen
* **130**: Sie haben den Unterbefehl mit Strg+C unterbrochen

Wenn Sie den Unterbefehl unterbrechen, läuft die Remote-Review weiter; folgen Sie der auf stderr gedruckten Sitzungs-URL, um sie im Browser zu beobachten.

Mit `--post` startet der Unterbefehl das Posten direkt nach dem Ausdrucken der Ergebnisse und gibt den Link auf stderr aus.

* Wenn die Ausführung fehlschlägt, das Timeout abläuft oder Sie sie unterbrechen, postet der Unterbefehl nichts.
* Wenn die Review abgeschlossen ist, aber der Kommentar nicht gepostet wird, gibt Claude Code den Grund auf stderr aus, und die Ergebnisse bleiben auf stdout, sodass Sie sie manuell posten können.

Für automatische Reviews bei GitHub Pull Requests integriert sich [Code Review](/docs/de/code-review) direkt mit Ihrem Repository und veröffentlicht Ergebnisse als Inline-PR-Kommentare ohne einen CLI-Schritt.

<h2 id="how-ultrareview-compares-to-/code-review">
  Wie Ultrareview mit /code-review verglichen wird
</h2>

Beide Reviews überprüfen Code, aber Sie verwenden sie in verschiedenen Phasen Ihres Workflows.

|               | `/code-review`                                               | `/code-review ultra`                                                                 |
| ------------- | ------------------------------------------------------------ | ------------------------------------------------------------------------------------ |
| Ziel          | Ihr Working Diff, ein Pull Request, ein Branch oder ein Pfad | Ihr Working Diff oder ein Pull Request                                               |
| Läuft         | lokal in Ihrer Sitzung                                       | in einer Cloud-Sandbox                                                               |
| Tiefe         | skaliert mit dem Effort-Argument                             | Multi-Agent-Flotte mit unabhängiger Verifizierung                                    |
| Dauer         | Sekunden bis wenige Minuten                                  | ungefähr 5 bis 10 Minuten                                                            |
| Kosten        | zählt zur normalen Nutzung                                   | kostenlose Durchläufe, dann ungefähr 5 bis 25 Dollar pro Review als Nutzungsguthaben |
| Am besten für | schnelles Feedback während der Iteration                     | Pre-Merge-Sicherheit bei wesentlichen Änderungen                                     |

Verwenden Sie `/code-review` für schnelles Feedback während der Arbeit, oder übergeben Sie eine PR-Nummer, um einen Pull Request eines Teamkollegen vor der Genehmigung zu überprüfen. Verwenden Sie `/code-review ultra` vor dem Merge einer wesentlichen Änderung, wenn Sie einen tieferen Durchgang wünschen, der Probleme erfasst, die eine lokale Review übersehen könnte.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Claude Code im Web](/docs/de/claude-code-on-the-web): Erfahren Sie, wie Cloud-Sitzungen und Cloud-Sandboxes funktionieren
* [Verwalten Sie Kosten effektiv](/docs/de/costs): Verfolgen Sie die Nutzung und legen Sie Ausgabenlimits fest
