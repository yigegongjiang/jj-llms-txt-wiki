> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Bugs mit Ultrareview finden

> Führen Sie eine tiefe, Multi-Agent-Code-Review in der Cloud mit /code-review ultra durch, um Bugs vor dem Merge zu finden und zu verifizieren.

<Note>
  Ultrareview ist eine Research-Preview-Funktion. Die Funktion, Preisgestaltung und Verfügbarkeit können sich basierend auf Feedback ändern. Der Befehl wird jetzt als `/code-review ultra` aufgerufen, und `/ultrareview` bleibt als Alias erhalten.
</Note>

Ultrareview ist eine tiefe Code-Review, die auf Claude Code in der Web-Infrastruktur ausgeführt wird. Wenn Sie `/code-review ultra` ausführen, startet Claude Code eine Flotte von Reviewer-Agenten in einer Remote-Sandbox, um Bugs in Ihrem Branch oder Pull Request zu finden.

Im Vergleich zu einer lokalen `/code-review` oder `/review` bietet Ultrareview:

* **Höhere Signalqualität**: Jeder gemeldete Fund wird unabhängig reproduziert und verifiziert, sodass sich die Ergebnisse auf echte Bugs konzentrieren und nicht auf Stilvorschläge
* **Breitere Abdeckung**: Eine größere Flotte von Reviewer-Agenten erkundet die Änderung parallel, was Probleme aufdeckt, die eine lokale Review übersehen könnte
* **Keine lokale Ressourcennutzung**: Die Review läuft vollständig in einer Remote-Sandbox, sodass Ihr Terminal für andere Arbeiten frei bleibt, während sie läuft

Ultrareview erfordert eine Authentifizierung mit einem Claude.ai-Konto, da es auf Claude Code in der Web-Infrastruktur ausgeführt wird. Wenn Sie nur mit einem API-Schlüssel angemeldet sind, führen Sie `/login` aus und authentifizieren Sie sich zuerst mit Claude.ai. Ultrareview ist nicht verfügbar, wenn Sie Claude Code mit Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry verwenden, und es ist nicht für Organisationen verfügbar, die Zero Data Retention aktiviert haben.

<h2 id="run-ultrareview-from-the-cli">
  Ultrareview von der CLI ausführen
</h2>

Starten Sie eine Review aus einem beliebigen Git-Repository in der Claude Code CLI.

```text theme={null}
/code-review ultra
```

Ohne Argumente überprüft Ultrareview den Diff zwischen Ihrem aktuellen Branch und dem Standard-Branch, einschließlich aller nicht committeter und gestaged Changes in Ihrem Working Tree. Claude Code bündelt den Repository-Status und lädt ihn in eine Remote-Sandbox für die Review hoch.

Um stattdessen einen GitHub Pull Request zu überprüfen, übergeben Sie die PR-Nummer.

```text theme={null}
/code-review ultra 1234
```

Im PR-Modus klont die Remote-Sandbox den Pull Request direkt vom Host, anstatt Ihren lokalen Working Tree zu bündeln. Der PR-Modus funktioniert mit Repositories auf `github.com` und auf [GitHub Enterprise Server](/docs/de/github-enterprise-server)-Instanzen, die ein Administrator mit Claude Code verbunden hat.

<Tip>
  Wenn Ihr Repository zu groß zum Bündeln ist, fordert Claude Code Sie auf, stattdessen den PR-Modus zu verwenden. Pushen Sie Ihren Branch und öffnen Sie einen Draft PR, führen Sie dann `/code-review ultra <PR-number>` aus.

  Wenn der Diff des Pull Request zu groß ist, lehnt Claude Code die Review mit einem Scoping-Hinweis ab, bevor irgendwelche Review-Arbeiten ausgeführt werden.
</Tip>

Vor dem Start zeigt Claude Code einen Bestätigungsdialog mit dem Review-Umfang (einschließlich der Datei- und Zeilenanzahl bei der Überprüfung eines Branches), Ihren verbleibenden kostenlosen Durchläufen und den geschätzten Kosten an. Nach der Bestätigung läuft die Review im Hintergrund weiter und Sie können Ihre Sitzung weiterhin nutzen. Der Befehl wird nur ausgeführt, wenn Sie ihn mit `/code-review ultra` aufrufen; Claude startet nicht automatisch eine Ultrareview.

<h2 id="pricing-and-free-runs">
  Preisgestaltung und kostenlose Durchläufe
</h2>

Ultrareview ist eine Premium-Funktion, die gegen Nutzungsguthaben statt gegen die in Ihrem Plan enthaltene Nutzung abgerechnet wird.

| Plan                | Kostenlose Durchläufe enthalten | Nach kostenlosen Durchläufen                                                                                          |
| ------------------- | ------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| Pro                 | 3 kostenlose Durchläufe         | abgerechnet als [Nutzungsguthaben](https://support.claude.com/de/articles/12429409-extra-usage-for-paid-claude-plans) |
| Max                 | 3 kostenlose Durchläufe         | abgerechnet als [Nutzungsguthaben](https://support.claude.com/de/articles/12429409-extra-usage-for-paid-claude-plans) |
| Team und Enterprise | keine                           | abgerechnet als [Nutzungsguthaben](https://support.claude.com/de/articles/12429409-extra-usage-for-paid-claude-plans) |

Pro- und Max-Abonnenten erhalten drei kostenlose Ultrareview-Durchläufe, um die Funktion zu testen. Diese drei Durchläufe sind eine einmalige Zuteilung pro Konto und werden nicht erneuert. Nachdem Sie alle drei verwendet haben oder nachdem der Zeitraum der kostenlosen Durchläufe endet, wird jede Review als Nutzungsguthaben abgerechnet und kostet typischerweise 5 bis 20 Dollar, je nach Größe der Änderung. Ein Durchlauf zählt, sobald die Cloud-Sitzung startet, daher verbraucht eine Review, die Sie frühzeitig beenden oder die nicht vollständig abgeschlossen wird, immer noch einen kostenlosen Durchlauf. Bei einer kostenpflichtigen Review werden Nutzungsguthaben nur für den Teil abgerechnet, der ausgeführt wurde.

Da Ultrareview außerhalb der kostenlosen Durchläufe immer als Nutzungsguthaben abgerechnet wird, muss Ihr Konto oder Ihre Organisation Nutzungsguthaben aktiviert haben, bevor Sie eine kostenpflichtige Review starten können. Wenn Nutzungsguthaben nicht aktiviert sind, blockiert Claude Code den Start und verlinkt Sie zu den Abrechnungseinstellungen, wo Sie sie aktivieren können. Sie können auch `/usage-credits` ausführen, um Ihre aktuelle Einstellung zu überprüfen oder zu ändern.

<h2 id="track-a-running-review">
  Eine laufende Review verfolgen
</h2>

Eine Review dauert normalerweise 5 bis 10 Minuten. Die Review läuft als Hintergrundaufgabe, sodass Sie in Ihrer Sitzung weiterarbeiten, andere Befehle starten oder das Terminal vollständig schließen können.

Verwenden Sie `/tasks`, um laufende und abgeschlossene Reviews anzuzeigen, die Detailansicht für eine Review zu öffnen oder eine laufende Review zu stoppen. Das Stoppen einer Review archiviert die Cloud-Sitzung, und teilweise Ergebnisse werden nicht zurückgegeben. Wenn die Review abgeschlossen ist, erscheinen die verifizierten Ergebnisse als Benachrichtigung in Ihrer Sitzung. Jedes Ergebnis enthält den Dateispeicherort und eine Erklärung des Problems, sodass Sie Claude direkt bitten können, es zu beheben.

<h2 id="run-ultrareview-non-interactively">
  Ultrareview nicht-interaktiv ausführen
</h2>

Verwenden Sie den Unterbefehl `claude ultrareview`, um eine Ultrareview von CI oder einem Skript ohne eine interaktive Sitzung zu starten. Der Unterbefehl startet die gleiche Review wie `/code-review ultra`, blockiert, bis die Remote-Review abgeschlossen ist, gibt die Ergebnisse auf stdout aus und beendet sich mit Code 0 bei Erfolg oder 1 bei Fehler.

```bash theme={null}
claude ultrareview
claude ultrareview 1234
claude ultrareview origin/main
```

Ohne Argumente überprüft der Unterbefehl den Diff zwischen Ihrem aktuellen Branch und dem Standard-Branch. Übergeben Sie eine PR-Nummer, um einen Pull Request zu überprüfen, oder übergeben Sie einen Base-Branch, um stattdessen den Diff gegen diesen Branch zu überprüfen. Das Aufrufen des Unterbefehls gilt als Zustimmung zu der Abrechnungs- und Bedingungseingabeaufforderung, die der interaktive Befehl anzeigt.

Fortschrittsmeldungen und die Live-Sitzungs-URL gehen zu stderr, sodass stdout analysierbar bleibt. Verwenden Sie diese Flags, um die Ausgabe und das Timeout zu steuern:

| Flag                  | Beschreibung                                                                  |
| --------------------- | ----------------------------------------------------------------------------- |
| `--json`              | Geben Sie die rohe `bugs.json`-Nutzlast statt der formatierten Ergebnisse aus |
| `--timeout <minutes>` | Maximale Minuten zum Warten auf den Abschluss der Review. Standard ist 30     |

Das Ausführen von `claude ultrareview` erfordert die gleiche Authentifizierung und Nutzungsguthaben-Konfiguration wie `/code-review ultra`. Der Unterbefehl beendet sich mit Code 0, wenn die Review mit oder ohne Ergebnisse abgeschlossen ist, Code 1, wenn die Review nicht gestartet werden kann, die Remote-Sitzung fehlschlägt oder das Timeout abläuft, und Code 130, wenn sie mit Strg+C unterbrochen wird. Die Remote-Review läuft weiter, wenn Sie den Unterbefehl unterbrechen; folgen Sie der auf stderr gedruckten Sitzungs-URL, um sie im Browser zu beobachten.

Für automatische Reviews bei GitHub Pull Requests integriert sich [Code Review](/docs/de/code-review) direkt mit Ihrem Repository und veröffentlicht Ergebnisse als Inline-PR-Kommentare ohne einen CLI-Schritt.

<h2 id="how-ultrareview-compares-to-/code-review-and-/review">
  Wie Ultrareview mit /code-review und /review verglichen wird
</h2>

Alle drei Befehle überprüfen Code, zielen aber auf verschiedene Phasen Ihres Workflows ab.

|               | `/code-review`                           | `/review <pr>`                                                         | `/code-review ultra`                                                                 |
| ------------- | ---------------------------------------- | ---------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| Ziel          | Ihr Working Diff                         | einen GitHub Pull Request                                              | Ihr Working Diff oder einen Pull Request                                             |
| Läuft         | lokal in Ihrer Sitzung                   | lokal in Ihrer Sitzung                                                 | remote in einer Cloud-Sandbox                                                        |
| Tiefe         | skaliert mit dem Effort-Argument         | eine einzelne Review auf dem Effort der Sitzung                        | Multi-Agent-Flotte mit unabhängiger Verifizierung                                    |
| Dauer         | Sekunden bis wenige Minuten              | Sekunden bis wenige Minuten                                            | ungefähr 5 bis 10 Minuten                                                            |
| Kosten        | zählt zur normalen Nutzung               | zählt zur normalen Nutzung                                             | kostenlose Durchläufe, dann ungefähr 5 bis 20 Dollar pro Review als Nutzungsguthaben |
| Am besten für | schnelles Feedback während der Iteration | Überprüfung eines Pull Requests eines Teamkollegen vor der Genehmigung | Pre-Merge-Sicherheit bei wesentlichen Änderungen                                     |

Verwenden Sie `/code-review` für schnelles Feedback während der Arbeit. Verwenden Sie `/review <pr>` zur Überprüfung eines Pull Requests auf die gleiche Weise wie vor der Genehmigung. Verwenden Sie `/code-review ultra` vor dem Merge einer wesentlichen Änderung, wenn Sie einen tieferen Durchgang wünschen, der Probleme erfasst, die eine lokale Review übersehen könnte.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Claude Code im Web](/docs/de/claude-code-on-the-web): Erfahren Sie, wie Cloud-Sitzungen und Cloud-Sandboxes funktionieren
* [Planen Sie komplexe Änderungen mit Ultraplan](/docs/de/ultraplan): das Planungs-Gegenstück zu Ultrareview für vorausschauende Designarbeiten
* [Verwalten Sie Kosten effektiv](/docs/de/costs): Verfolgen Sie die Nutzung und legen Sie Ausgabenlimits fest
