> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Teamnutzung mit Analysen verfolgen

> Zeigen Sie Claude Code-Nutzungsmetriken an, verfolgen Sie die Einführung und messen Sie die Engineering-Geschwindigkeit im Analytics-Dashboard.

Claude Code bietet Analytics-Dashboards, um Organisationen dabei zu helfen, Entwicklernutzungsmuster zu verstehen, Beitragskennzahlen zu verfolgen und zu messen, wie Claude Code die Engineering-Geschwindigkeit beeinflusst. Greifen Sie auf das Dashboard für Ihren Plan zu:

| Plan                          | Dashboard-URL                                                              | Enthält                                                                             | Weitere Informationen                                |
| ----------------------------- | -------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- | ---------------------------------------------------- |
| Claude for Teams / Enterprise | [claude.ai/analytics/claude-code](https://claude.ai/analytics/claude-code) | Nutzungsmetriken, Beitragskennzahlen mit GitHub-Integration, Rangliste, Datenexport | [Details](#access-analytics-for-team-and-enterprise) |
| API (Claude Console)          | [platform.claude.com/claude-code](https://platform.claude.com/claude-code) | Nutzungsmetriken, Ausgabenverfolgung, Team-Insights                                 | [Details](#access-analytics-for-api-customers)       |

<h2 id="access-analytics-for-team-and-enterprise">
  Analytics für Teams und Enterprise aufrufen
</h2>

Navigieren Sie zu [claude.ai/analytics/claude-code](https://claude.ai/analytics/claude-code). Admins und Owners können das Dashboard anzeigen.

Das Teams- und Enterprise-Dashboard enthält:

* **Nutzungsmetriken**: akzeptierte Codezeilen, Akzeptanzrate für Vorschläge, täglich aktive Benutzer und Sitzungen
* **Beitragskennzahlen**: PRs und Codezeilen, die mit Claude Code-Unterstützung versendet wurden, mit [GitHub-Integration](#enable-contribution-metrics)
* **Rangliste**: Top-Beitragsteller, sortiert nach Claude Code-Nutzung
* **Datenexport**: Beitragsdaten als CSV für benutzerdefinierte Berichte herunterladen

Für Pro-Benutzer-Token-Zählungen und Kostenschätzungen konfigurieren Sie [OpenTelemetry-Export](/docs/de/monitoring-usage), oder exportieren Sie den [Ausgabenbericht](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) aus den Analytics-Einstellungen Ihrer Organisation, der die Token-Nutzung und geschätzte Ausgabenguthaben-Ausgaben pro Benutzer und pro Modell auflistet.

<h3 id="enable-contribution-metrics">
  Beitragskennzahlen aktivieren
</h3>

<Note>
  Beitragskennzahlen befinden sich in der öffentlichen Beta und sind in Claude for Teams und Claude for Enterprise-Plänen verfügbar. Diese Metriken decken nur Benutzer innerhalb Ihrer claude.ai-Organisation ab. Die Nutzung über die Claude Console API oder Integrationen von Drittanbietern ist nicht enthalten.
</Note>

Nutzungs- und Einführungsdaten sind für alle Claude for Teams und Claude for Enterprise-Konten verfügbar. Beitragskennzahlen erfordern zusätzliche Einrichtung, um Ihre GitHub-Organisation zu verbinden.

Sie benötigen die Owner-Rolle, um Analytics-Einstellungen zu konfigurieren. Ein GitHub-Admin muss die GitHub-App installieren.

<Warning>
  Beitragskennzahlen sind nicht für Organisationen mit aktiviertem [Zero Data Retention](/docs/de/zero-data-retention) verfügbar. Das Analytics-Dashboard zeigt nur Nutzungsmetriken an.
</Warning>

<Steps>
  <Step title="GitHub-App installieren">
    Ein GitHub-Admin installiert die Claude GitHub-App auf dem GitHub-Konto Ihrer Organisation unter [github.com/apps/claude](https://github.com/apps/claude).
  </Step>

  <Step title="Claude Code-Analysen aktivieren">
    Ein Claude Owner navigiert zu [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) und aktiviert die Claude Code-Analytics-Funktion.
  </Step>

  <Step title="GitHub-Analysen aktivieren">
    Aktivieren Sie auf der gleichen Seite den Schalter 'GitHub analytics".
  </Step>

  <Step title="Mit GitHub authentifizieren">
    Schließen Sie den GitHub-Authentifizierungsfluss ab und wählen Sie aus, welche GitHub-Organisationen in die Analyse einbezogen werden sollen.
  </Step>
</Steps>

Daten werden normalerweise innerhalb von 24 Stunden nach der Aktivierung angezeigt, mit täglichen Updates. Wenn keine Daten angezeigt werden, wird möglicherweise eine dieser Meldungen angezeigt:

* **'GitHub-App erforderlich"**: Installieren Sie die GitHub-App, um Beitragskennzahlen anzuzeigen
* **„Datenverarbeitung läuft"**: Überprüfen Sie in einigen Tagen erneut und bestätigen Sie, dass die GitHub-App installiert ist, falls keine Daten angezeigt werden

Beitragskennzahlen unterstützen GitHub Cloud und GitHub Enterprise Server.

<h3 id="review-summary-metrics">
  Zusammenfassende Metriken überprüfen
</h3>

<Note>
  Diese Metriken sind absichtlich konservativ und stellen eine Unterschätzung der tatsächlichen Auswirkungen von Claude Code dar. Nur Zeilen und PRs, bei denen hohes Vertrauen in die Beteiligung von Claude Code besteht, werden gezählt.
</Note>

Das Dashboard zeigt diese zusammenfassenden Metriken oben an:

* **PRs mit CC**: Gesamtanzahl der zusammengeführten Pull Requests, die mindestens eine Codezeile enthalten, die mit Claude Code geschrieben wurde
* **Codezeilen mit CC**: Gesamtzahl der Codezeilen in allen zusammengeführten PRs, die mit Claude Code-Unterstützung geschrieben wurden. Nur „effektive Zeilen" werden gezählt: Zeilen mit mehr als 3 Zeichen nach Normalisierung, ohne leere Zeilen und Zeilen mit nur Klammern oder trivialer Interpunktion.
* **PRs mit Claude Code (%)**: Prozentsatz aller zusammengeführten PRs, die Claude Code-unterstützten Code enthalten
* **Akzeptanzrate für Vorschläge**: Prozentsatz der Fälle, in denen Benutzer Claude Code-Codebearbeitungsvorschläge akzeptieren, einschließlich Edit, Write und NotebookEdit-Tool-Nutzung
* **Akzeptierte Codezeilen**: Gesamtzahl der Codezeilen, die von Claude Code geschrieben wurden und die Benutzer in ihren Sitzungen akzeptiert haben. Dies schließt abgelehnte Vorschläge aus und verfolgt keine nachfolgenden Löschungen.

<h3 id="explore-the-charts">
  Diagramme erkunden
</h3>

Das Dashboard enthält mehrere Diagramme zur Visualisierung von Trends im Zeitverlauf.

<h4 id="track-adoption">
  Einführung verfolgen
</h4>

Das Adoption-Diagramm zeigt tägliche Nutzungstrends:

* **users**: täglich aktive Benutzer
* **sessions**: Anzahl der aktiven Claude Code-Sitzungen pro Tag

<h4 id="measure-prs-per-user">
  PRs pro Benutzer messen
</h4>

Dieses Diagramm zeigt die individuelle Entwickleraktivität im Zeitverlauf:

* **PRs per user**: Gesamtzahl der pro Tag zusammengeführten PRs geteilt durch täglich aktive Benutzer
* **users**: täglich aktive Benutzer

Verwenden Sie dies, um zu verstehen, wie sich die individuelle Produktivität mit zunehmender Claude Code-Einführung ändert.

<h4 id="view-pull-requests-breakdown">
  Aufschlüsselung der Pull Requests anzeigen
</h4>

Das Pull requests-Diagramm zeigt eine tägliche Aufschlüsselung der zusammengeführten PRs:

* **PRs with CC**: Pull Requests mit Claude Code-unterstütztem Code
* **PRs without CC**: Pull Requests ohne Claude Code-unterstützten Code

Wechseln Sie zur Ansicht **Lines of code**, um die gleiche Aufschlüsselung nach Codezeilen statt nach PR-Anzahl zu sehen.

<h4 id="find-top-contributors">
  Top-Beitragsteller finden
</h4>

Die Rangliste zeigt die Top 10-Benutzer, sortiert nach Beitragsmenge. Wechseln Sie zwischen:

* **Pull requests**: zeigt PRs mit Claude Code vs. alle PRs für jeden Benutzer
* **Lines of code**: zeigt Zeilen mit Claude Code vs. alle Zeilen für jeden Benutzer

Klicken Sie auf **Export all users**, um vollständige Beitragsdaten für alle Benutzer als CSV-Datei herunterzuladen. Der Export enthält alle Benutzer, nicht nur die angezeigten Top 10.

<h3 id="pr-attribution">
  PR-Zuordnung
</h3>

Wenn Beitragskennzahlen aktiviert sind, analysiert Claude Code zusammengeführte Pull Requests, um zu bestimmen, welcher Code mit Claude Code-Unterstützung geschrieben wurde. Dies geschieht durch Abgleich der Claude Code-Sitzungsaktivität mit dem Code in jedem PR.

<h4 id="tagging-criteria">
  Tagging-Kriterien
</h4>

PRs werden als „with Claude Code" gekennzeichnet, wenn sie mindestens eine Codezeile enthalten, die während einer Claude Code-Sitzung geschrieben wurde. Das System verwendet konservatives Matching: Nur Code, bei dem hohes Vertrauen in die Beteiligung von Claude Code besteht, wird als unterstützt gezählt.

<h4 id="attribution-process">
  Zuordnungsprozess
</h4>

Wenn ein Pull Request zusammengeführt wird:

1. Hinzugefügte Zeilen werden aus dem PR-Diff extrahiert
2. Claude Code-Sitzungen, die übereinstimmende Dateien innerhalb eines Zeitfensters bearbeitet haben, werden identifiziert
3. PR-Zeilen werden mit Claude Code-Ausgabe unter Verwendung mehrerer Strategien abgeglichen
4. Metriken werden für KI-unterstützte Zeilen und Gesamtzeilen berechnet

Vor dem Vergleich werden Zeilen normalisiert: Leerzeichen werden gekürzt, mehrere Leerzeichen werden zusammengefasst, Anführungszeichen werden standardisiert und Text wird in Kleinbuchstaben konvertiert.

Zusammengeführte Pull Requests mit Claude Code-unterstützten Zeilen werden in GitHub mit `claude-code-assisted` gekennzeichnet.

<h4 id="time-window">
  Zeitfenster
</h4>

Sitzungen von 21 Tagen vor bis 2 Tage nach dem PR-Zusammenführungsdatum werden für den Zuordnungsabgleich berücksichtigt.

<h4 id="excluded-files">
  Ausgeschlossene Dateien
</h4>

Bestimmte Dateien werden automatisch von der Analyse ausgeschlossen, da sie automatisch generiert werden:

* Lock-Dateien: package-lock.json, yarn.lock, Cargo.lock und ähnliche
* Generierter Code: Protobuf-Ausgaben, Build-Artefakte, minifizierte Dateien
* Build-Verzeichnisse: dist/, build/, node\_modules/, target/
* Test-Fixtures: Snapshots, Cassetten, Mock-Daten
* Zeilen über 1.000 Zeichen, die wahrscheinlich minifiziert oder generiert sind

<h4 id="attribution-notes">
  Zuordnungshinweise
</h4>

Beachten Sie diese zusätzlichen Details bei der Interpretation von Zuordnungsdaten:

* Code, der von Entwicklern erheblich umgeschrieben wurde, mit mehr als 20% Unterschied, wird nicht Claude Code zugeordnet
* Sitzungen außerhalb des 21-Tage-Fensters werden nicht berücksichtigt
* Der Algorithmus berücksichtigt nicht den PR-Quell- oder Zielzweig bei der Durchführung der Zuordnung

<h3 id="get-the-most-from-analytics">
  Nutzen Sie Analytics optimal
</h3>

Verwenden Sie Beitragskennzahlen, um ROI zu demonstrieren, Einführungsmuster zu identifizieren und Teammitglieder zu finden, die anderen beim Einstieg helfen können.

<h4 id="monitor-adoption">
  Einführung überwachen
</h4>

Verfolgen Sie das Adoption-Diagramm und Benutzerzahlen, um Folgendes zu identifizieren:

* Aktive Benutzer, die Best Practices teilen können
* Gesamte Einführungstrends in Ihrer Organisation
* Nutzungsrückgänge, die auf Reibung oder Probleme hindeuten können

<h4 id="measure-roi">
  ROI messen
</h4>

Beitragskennzahlen helfen bei der Beantwortung der Frage „Lohnt sich dieses Tool für die Investition?" mit Daten aus Ihrer eigenen Codebasis:

* Verfolgen Sie Änderungen bei PRs pro Benutzer im Zeitverlauf, wenn die Einführung zunimmt
* Vergleichen Sie PRs und Codezeilen, die mit und ohne Claude Code versendet wurden
* Verwenden Sie zusammen mit [DORA-Metriken](https://dora.dev/), Sprint-Geschwindigkeit oder anderen Engineering-KPIs, um Änderungen durch die Einführung von Claude Code zu verstehen

<h4 id="identify-power-users">
  Power-User identifizieren
</h4>

Die Rangliste hilft Ihnen, Teammitglieder mit hoher Claude Code-Einführung zu finden, die:

* Prompting-Techniken und Workflows mit dem Team teilen können
* Feedback geben können, was gut funktioniert
* Neue Benutzer onboarden können

<h4 id="access-data-programmatically">
  Auf Daten programmgesteuert zugreifen
</h4>

Im Enterprise-Plan gibt die [Claude Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) Pro-Benutzer-Engagement-, Nutzungs- und Kostenberichte für Ihre Organisation über Claude-Oberflächen hinweg zurück, einschließlich Claude Code. Ein Primary Owner erstellt einen Schlüssel mit dem Umfang `read:analytics` unter [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys). Die API ist im Teams-Plan nicht verfügbar.

Um diese Daten über GitHub abzufragen, suchen Sie nach PRs mit dem Label `claude-code-assisted`.

<h2 id="access-analytics-for-api-customers">
  Analytics für API-Kunden aufrufen
</h2>

API-Kunden, die die Claude Console verwenden, können auf Analytics unter [platform.claude.com/claude-code](https://platform.claude.com/claude-code) zugreifen. Sie benötigen die UsageView-Berechtigung, um auf das Dashboard zuzugreifen, die den Rollen Developer, Billing, Admin, Owner und Primary Owner gewährt wird. Um die gleichen täglichen Metriken pro Benutzer programmgesteuert abzurufen, verwenden Sie die [Claude Code Analytics API](https://platform.claude.com/docs/de/build-with-claude/claude-code-analytics-api) mit einem Admin-API-Schlüssel.

<Note>
  Beitragskennzahlen mit GitHub-Integration sind derzeit nicht für API-Kunden verfügbar. Das Console-Dashboard zeigt nur Nutzungs- und Ausgabemetriken an.
</Note>

Das Console-Dashboard zeigt:

* **Lines of code accepted**: Gesamtzahl der Codezeilen, die von Claude Code geschrieben wurden und die Benutzer in ihren Sitzungen akzeptiert haben. Dies schließt abgelehnte Vorschläge aus und verfolgt keine nachfolgenden Löschungen.
* **Suggestion accept rate**: Prozentsatz der Fälle, in denen Benutzer die Nutzung von Code-Bearbeitungstools akzeptieren, einschließlich Edit, Write und NotebookEdit-Tools.
* **Activity**: täglich aktive Benutzer und Sitzungen, die in einem Diagramm angezeigt werden.
* **Spend**: tägliche API-Kosten in Dollar neben der Benutzerzahl.

<h3 id="view-team-insights">
  Team-Insights anzeigen
</h3>

Die Team-Insights-Tabelle zeigt Metriken pro Benutzer:

* **Members**: alle Benutzer, die sich bei Claude Code authentifiziert haben. API-Schlüsselbenutzer werden nach Schlüsselkennung angezeigt, OAuth-Benutzer werden nach E-Mail-Adresse angezeigt.
* **Spend this month**: Gesamtkosten der API pro Benutzer für den aktuellen Monat.
* **Lines this month**: Gesamtzahl der akzeptierten Codezeilen pro Benutzer für den aktuellen Monat.

<Note>
  Die Ausgabenzahlen im Console-Dashboard sind Schätzungen für Analytics-Zwecke. Für tatsächliche Kosten beziehen Sie sich auf Ihre Abrechnungsseite.
</Note>

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Monitoring mit OpenTelemetry](/docs/de/monitoring-usage): Exportieren Sie Echtzeit-Metriken und Ereignisse in Ihren Observability-Stack
* [Kosten effektiv verwalten](/docs/de/costs): Legen Sie Ausgabenlimits fest und optimieren Sie die Token-Nutzung
* [Berechtigungen](/docs/de/permissions): Konfigurieren Sie Rollen und Berechtigungen
