> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kosten effektiv verwalten

> Verfolgen Sie die Token-Nutzung, legen Sie Ausgabenlimits für Teams fest und reduzieren Sie Claude Code-Kosten durch Kontextverwaltung, Modellauswahl, Einstellungen für erweitertes Denken und Preprocessing-Hooks.

Claude Code wird nach API-Token-Verbrauch berechnet. Für Abonnementplan-Preise (Pro, Max, Team, Enterprise) siehe [claude.com/pricing](https://claude.com/pricing). Die Kosten pro Entwickler variieren stark je nach Modellauswahl, Codebasis-Größe und Nutzungsmustern wie dem Ausführen mehrerer Instanzen oder Automatisierung.

In unternehmensweiten Bereitstellungen betragen die durchschnittlichen Kosten etwa 13 USD pro Entwickler pro aktivem Tag und 150–250 USD pro Entwickler pro Monat, wobei die Kosten für 90 % der Benutzer unter 30 USD pro aktivem Tag bleiben. Um die Ausgaben für Ihr eigenes Team zu schätzen, beginnen Sie mit einer kleinen Pilotgruppe und verwenden Sie die Tracking-Tools unten, um eine Baseline zu etablieren, bevor Sie einen breiteren Rollout durchführen.

Diese Seite behandelt, wie Sie [Ihre Kosten verfolgen](#track-your-costs), [Kosten für Ihre Organisation verwalten](#manage-costs-for-your-organization) und [Token-Nutzung reduzieren](#reduce-token-usage).

<h2 id="track-your-costs">
  Verfolgen Sie Ihre Kosten
</h2>

<h3 id="using-the-/usage-command">
  Verwenden des `/usage`-Befehls
</h3>

<Note>
  Der Session-Block in `/usage` zeigt die API-Token-Nutzung an und ist für API-Benutzer vorgesehen. Claude Max und Pro-Abonnenten haben die Nutzung in ihrem Abonnement enthalten, daher ist die Session-Kostenzahl nicht relevant für Abrechnungszwecke. Abonnenten sehen Plannutzungsbalken, Aktivitätsstatistiken und eine Nutzungsaufschlüsselung auf demselben Bildschirm.
</Note>

Der Session-Block oben in `/usage` zeigt detaillierte Token-Nutzungsstatistiken für Ihre aktuelle Sitzung. Claude Code berechnet die Dollarzahl lokal aus Token-Zählungen zum Listenpreis, es sei denn, eine [`modelPricing`](/docs/de/settings-reference#modelpricing)-Tabelle ist in Kraft. Ein Administrator legt eine in den verwalteten Einstellungen Ihrer Organisation fest, damit die Zahl Ihre vertraglich vereinbarten Sätze verwendet, und während eine Tabelle in Kraft ist, trägt die Zeile `Total cost` die Notiz `at your organization's configured rates`. Die Zahl ist eine Schätzung, daher siehe für verbindliche Abrechnung die Nutzungsseite in der [Claude Console](https://platform.claude.com/usage).

```text theme={null}
Total cost:            $0.55
Total duration (API):  6m 20s
Total duration (wall): 6h 33m 10s
Total code changes:    0 lines added, 0 lines removed
Usage by model:
   claude-sonnet-4-6:  1.2k input, 5.3k output, 940.0k cache read, 50.0k cache write ($0.55)
```

Diese Summen werden zurückgesetzt, wenn `/clear` eine neue Sitzung startet, daher beginnt die Gesamtkostenzahl der nächsten Sitzung bei \$0. Vor v2.1.211 wurden sie über `/clear` für die Lebensdauer des Claude Code-Prozesses weiter addiert.

Für eine Antwort von der Claude API, die zum 1,1×-[Datenresidenz-Satz](https://platform.claude.com/docs/en/about-claude/pricing#data-residency-pricing) abgerechnet wird, multipliziert Claude Code den Listenpreis der Token dieser Antwort mit 1,1 in der Session-Kostenzahl. Die gleiche Summe erscheint in der [Kostenfeld der Statuszeile](/docs/de/statusline#cost-and-duration-tracking), und die multiplizierte Zahl zählt auch gegen [`--max-budget-usd`](/docs/de/cli-reference#cli-flags). Vor v2.1.239 wendete Claude Code die 1,1× nicht auf diese Antworten an, daher war die Session-Kostenzahl niedriger als die Rechnung.

<h4 id="prompt-cache-statistics">
  Prompt-Cache-Statistiken
</h4>

Nach der ersten API-Antwort der Hauptkonversation fügt Claude Code dem Session-Block auch eine Zeile `Prompt cache (main)` hinzu, die die [Prompt-Cache](/docs/de/prompt-caching)-Nutzung der Sitzung zusammenfasst: die Anfragezahl, der Anteil der Eingabe-Token, die aus dem Cache bereitgestellt werden, Cache-Misses und ob der Cache gerade warm ist. Erfordert Claude Code v2.1.251 oder später.

```text theme={null}
Prompt cache (main):   14 requests · 91% of input tokens from cache · 2 misses (last 6m 10s ago, 310.2k tokens re-cached) · 1 expected rebuild (compaction or tool-result clearing) · warm (1h TTL, last activity 40s ago)
```

Die Misses, erwarteten Neuerstellungen und warmen oder kalten Teile der Zeile bedeuten Folgendes:

* **Misses**: Anfragen, die Inhalte erneut verarbeitet haben, die der Cache bereits enthielt, mit dem Zeitpunkt des letzten Miss und wie viele Token diese Anfragen zurück in den Cache geschrieben haben. Claude Code zählt eine Anfrage als Miss, wenn die Anfrage mehr als 5% und mindestens 2.000 Token von dem erneut verarbeitet hat, was sie aus dem Cache hätte lesen können. [Aktionen, die den Cache ungültig machen](/docs/de/prompt-caching#actions-that-invalidate-the-cache) listet die üblichen Ursachen auf. Wenn Claude Code eine wahrscheinliche Ursache für den letzten Miss identifizieren kann, benennt die Zeile sie auch, zum Beispiel `likely cause: tool definitions changed`. Der Text der wahrscheinlichen Ursache erfordert Claude Code v2.1.260 oder später.
* **Erwartete Neuerstellungen**: Wenn Claude Code die Konversation selbst gerade umgeschrieben hat, durch [Komprimierung](/docs/de/prompt-caching#compacting-the-conversation) oder durch Löschen alter Tool-Ergebnisse aus dem Kontext, zählt es die gleiche Art von Miss als erwartete Neuerstellung statt. Dieser Teil erscheint nur, nachdem mindestens eine erwartete Neuerstellung stattgefunden hat.
* **Warm oder kalt**: ob das zwischengespeicherte Präfix noch innerhalb seiner [Cache-Lebensdauer](/docs/de/prompt-caching#cache-lifetime) liegt, mit dem geltenden TTL. Wenn der Cache kalt ist, zeigt die Zeile, wie lange die Sitzung untätig war. Wenn keine Antwort Cache-Token gemeldet hat, endet die Zeile stattdessen mit `no prompt caching reported by the API`.

Die Zählungen stammen aus den Cache-Token-Feldern in den API-Antworten, daher funktioniert die Zeile bei jedem Anbieter und Gateway. Sie deckt nur die Hauptkonversation ab, nicht Subagenten. `/clear` setzt sie mit dem Rest des Session-Blocks zurück.

Statuszeilen-Skripte können die gleichen Zahlen aus dem [`prompt_cache`-Objekt](/docs/de/statusline#prompt-cache-fields) lesen.

<h4 id="plan-usage-breakdown">
  Plannutzungs-Aufschlüsselung
</h4>

Bei einem Pro-, Max-, Team- oder Enterprise-Plan zeigt `/usage` auch eine Aufschlüsselung dessen, was gegen Ihre Planlimits zählt:

* **Attribution**: kürzliche Nutzung, die Skills, Subagenten, Plugins und einzelnen MCP-Servern zugeordnet ist, jeweils als Prozentsatz des Gesamtbetrags angezeigt. Der Anteil eines MCP-Servers zählt nur die Anfragen, die eines seiner Tool-Ergebnisse verbraucht haben. Vor v2.1.222 ordnete Claude Code nach einem Aufruf eines MCP-Servers jede nachfolgende Anfrage diesem Server zu, was seinen Anteil überzeichnete.
* **Verhaltensflags**: Verhaltensweisen wie langer Kontext oder Cache-Misses, gekennzeichnet, wenn eine 10% oder mehr der kürzlichen Nutzung ausmacht.
* **Schleifen**: eine Zeile für jede der schwersten [`/loop` oder anderen geplanten Aufgaben](/docs/de/scheduled-tasks), die kürzlich ausgeführt wurden, geordnet nach Gesamttoken, mit einer Zählung der übrigen. Claude Code meldet, wie oft jede Aufgabe ausgelöst wird, wie oft sie ausgeführt wurde, ihre Gesamt- und Pro-Lauf-Token und wann sie zuletzt ausgeführt wurde. Claude Code schlüsselt eine Zeile nach der Eingabeaufforderung der Aufgabe auf, daher bleibt eine Schleife, die Sie stoppen und neu erstellen, eine Zeile. Erfordert Claude Code v2.1.242 oder später.

Drücken Sie `d` oder `w`, um zwischen den letzten 24 Stunden und den letzten 7 Tagen zu wechseln. Die Zahlen sind ungefähr und werden aus dem lokalen Sitzungsverlauf auf diesem Computer berechnet, daher ist die Nutzung von anderen Geräten oder claude.ai nicht enthalten.

In der [VS Code-Erweiterung](/docs/de/vs-code#check-account-and-usage) erscheinen die Attributionsanteile und Verhaltensflags im Dialog „Konto & Nutzung" mit einem Tag- und Woche-Umschalter, ohne die Zeilen „Schleifen".

<h4 id="check-your-usage-credits-spend">
  Überprüfen Sie Ihre Ausgaben für Nutzungsguthaben
</h4>

`/usage` zeigt auch eine Nutzungsguthaben-Zeile an, während [Nutzungsguthaben](#add-usage-credits-to-your-subscription) aktiviert sind. Was die Zeile anzeigt, hängt von Ihrem Plan ab:

* **Pro und Max**: Ihre Ausgaben für den aktuellen Monat, gemessen an Ihrem monatlichen Ausgabenlimit, wenn Sie eines festgelegt haben. Wenn Sie kein Limit festgelegt haben, zeigt die Zeile `Unlimited` und keine Ausgabenzahl.
* **Team und Enterprise**: Ihre eigenen Ausgaben für den aktuellen Monat, gemessen an jedem [Limit, das Ihre Organisation festgelegt hat](#claude-for-teams-and-enterprise), das für Sie gilt. Ein Limit, das die gesamte Organisation abdeckt, erscheint nicht in der Zeile. Wenn Sie kein eigenes Limit haben, zeigt die Zeile Ihre Ausgaben ohne Limit daneben. Während Nutzungsguthaben für Sie ausgeschaltet sind, zeigt `/usage` keine Nutzungsguthaben-Zeile.

Wenn Sie ein Ausgabenlimit haben, erscheint die Zeile, sobald Nutzungsguthaben aktiviert sind, und zeigt 0%, bis Sie zum ersten Mal Nutzungsguthaben ausgeben. Vor v2.1.236 zeigte `/usage` die Zeile nur bei Pro- und Max-Plänen an, und eine Zeile mit einem Ausgabenlimit blieb verborgen, bis Sie etwas ausgegeben hatten.

<h4 id="when-the-usage-request-fails">
  Wenn die Nutzungsanfrage fehlschlägt
</h4>

Wenn die Anfrage für Ihre Planlimits fehlschlägt, meistens weil der Nutzungs-Endpunkt Rate-Limited ist, zeigt `/usage` die letzten Nutzungsbalken an, die auf diesem Computer in den letzten 60 Minuten geladen wurden, zusammen mit einer Notiz `Showing last-known usage`, die angibt, wie lange diese Daten her sind. Drücken Sie `r`, um es erneut zu versuchen; ein erfolgreicher Versuch ersetzt die letzten bekannten Balken durch aktuelle Daten. Ohne einen Snapshot aus den letzten 60 Minuten meldet `/usage`, dass der Nutzungs-Endpunkt Rate-Limited ist, und bietet die gleiche Wiederholungsverknüpfung an. Vor v2.1.208 zeigte eine Rate-Limited-Anfrage in einer Sitzung, die noch keine Nutzung geladen hatte, immer den Fehler ohne Balken an.

<h3 id="analyze-your-usage-patterns">
  Analysieren Sie Ihre Nutzungsmuster
</h3>

Führen Sie [`/insights`](/docs/de/commands#all-commands) aus, um einen Bericht darüber zu erhalten, wie Sie arbeiten, anstatt wie viele Token Sie verwendet haben. Es analysiert Ihre kürzlichen Sitzungen auf diesem Computer und schreibt einen HTML-Bericht, der abdeckt, woran Sie arbeiten, Reibungspunkte wie missverstandene Anfragen oder fehlerhafter Code und Vorschläge zur effektiveren Nutzung von Claude Code. Ein einzelner Lauf analysiert bis zu 200 Sitzungen, die es noch nicht gesehen hat, und überspringt sehr kurze. Wenn Sitzungen ausgelassen werden, zeigt der Berichtkopf die analysierte Anzahl mit der Gesamtzahl in Klammern an, zum Beispiel `200 sessions (412 total)`.

Claude Code schreibt den neuesten Bericht in `~/.claude/usage-data/report.html` und speichert eine zeitgestempelte Kopie jedes Laufs im gleichen Verzeichnis, daher werden frühere Berichte nicht überschrieben. Claude Code löscht Berichte nach dem gleichen Zeitplan wie der Rest Ihrer Sitzungsdaten: beim Start entfernt es Dateien, die älter als [`cleanupPeriodDays`](/docs/de/claude-directory#cleaned-up-automatically) sind, standardmäßig 30 Tage.

Sie können `/insights` bei jedem Plan und mit jedem Anbieter ausführen. Die Analyse läuft durch den gleichen Anbieter und das gleiche Konto wie Ihre regulären Sitzungen, und die Token zählen gegen Ihren Plan oder Ihre API-Nutzung. Sitzungen von anderen Geräten und claude.ai sind nicht enthalten.

<h3 id="add-usage-credits-to-your-subscription">
  Fügen Sie Nutzungsguthaben zu Ihrem Abonnement hinzu
</h3>

[Nutzungsguthaben](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) ermöglichen es Ihnen, über das Nutzungslimit Ihres Plans hinaus zu arbeiten. Um sie zu verwalten, führen Sie `/usage-credits` aus, nachdem Sie sich mit Ihrem claude.ai-Abonnement über `/login` angemeldet haben; der Befehl ist nicht mit API-Schlüssel-Authentifizierung verfügbar. In Self-Service-Enterprise-Organisationen, Enterprise-Testversionen und Enterprise-Organisationen, die über AWS Marketplace abgerechnet werden, erfordert der Befehl Claude Code v2.1.248 oder später; frühere Versionen lehnen ihn mit [`Unknown command: /usage-credits`](/docs/de/errors#unknown-command) ab. Was er öffnet, hängt von Ihrer Rolle ab:

| Ihre Rolle                                             | Was `/usage-credits` tut                                                                                                                                                                                                                                                                            |
| :----------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Pro- oder Max-Abonnent                                 | Öffnet [**Einstellungen > Nutzung**](https://claude.ai/settings/usage) auf claude.ai im Browser. In seinem Abschnitt **Nutzungsguthaben** können Sie Nutzungsguthaben aktivieren oder deaktivieren und Ihren Guthabensaldo, die Ausgaben dieses Monats und Ihr monatliches Ausgabenlimit überprüfen |
| Team- oder Enterprise-Mitglied mit Abrechnungszugriff  | Öffnet Ihre Organisationsnutzungseinstellungen, [**Admin-Einstellungen > Nutzung**](https://claude.ai/admin-settings/usage), im Browser                                                                                                                                                             |
| Team- oder Enterprise-Mitglied ohne Abrechnungszugriff | Fordert Sie auf zu bestätigen, sendet dann eine Anfrage an die Administratoren Ihrer Organisation. Vor v2.1.211 sendete Claude Code die Anfrage ohne einen Bestätigungsschritt                                                                                                                      |

Für Team- und Enterprise-Mitglieder ohne Abrechnungszugriff erscheint die Bestätigung nur in interaktiven Sitzungen: im nicht-interaktiven Modus mit dem `-p`-Flag und von [Remote Control](/docs/de/remote-control) aus sendet der Befehl keine Anfrage und teilt Ihnen mit, dass Sie ihn stattdessen in einer interaktiven Sitzung ausführen sollen.

Wenn Sie `/usage-credits` erneut ausführen, während Ihre frühere Anfrage auf einen Administrator wartet, teilt Claude Code Ihnen mit, dass bereits eine Anfrage gesendet wurde, anstatt ein Duplikat zu senden. Nachdem ein Administrator Ihre Anfrage ablehnt, sendet das erneute Ausführen des Befehls eine neue. Vor v2.1.222 blockierte eine abgelehnte Anfrage auch neue Anfragen.

Bei Pro- und Max-Plänen fordert Claude Code Sie auf, Ihr Limit zu erhöhen oder zu entfernen, ohne die CLI zu verlassen, wenn Sie Ihr Ausgabenlimit erreichen, während noch Nutzungsguthaben verfügbar sind. Wenn der Server die Änderung ablehnt, siehe [Could not update your spend limit](/docs/de/errors#could-not-update-your-spend-limit).

<h2 id="manage-costs-for-your-organization">
  Verwalten Sie Kosten für Ihre Organisation
</h2>

Welche Kontrollen Sie haben, hängt davon ab, wie Ihre Organisation auf Claude Code zugreift: über einen Claude for Teams oder Enterprise-Plan, die Claude Console oder einen Cloud-Anbieter. Bei Teams- und Enterprise-Plänen wird die Nutzung aus der Sitzplatzerlaubnis jedes Mitglieds gezogen. In der Console und bei Cloud-Anbietern wird die Nutzung pro Token an Ihre Organisation abgerechnet. Wenn Ihre Organisation verschiedene Anmeldeverfahren mischt, wird jeder Entwickler nach dem gemessen, mit dem er sich authentifiziert hat.

Die Tabelle ordnet jedes Setup zu, wo Sie Ausgaben sehen, wo Sie sie begrenzen, und wie Sie Pro-Benutzer-Zahlen abrufen. Bei einem einzelnen Pro- oder Max-Plan haben Sie keine Organisation zu verwalten, daher verfolgen Sie Ihre eigenen Ausgaben für Nutzungsguthaben, einschließlich [Schnellmodus](/docs/de/fast-mode#see-where-fast-mode-spend-appears), unter [Nutzungsguthaben zu Ihrem Abonnement hinzufügen](#add-usage-credits-to-your-subscription).

| Ihr Setup                                                                                | Ausgaben anzeigen                                                                                                                              | Ausgaben begrenzen                    | Pro-Benutzer-Berichterstattung                                                                                                                                                                                                |
| :--------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Claude for Teams oder Enterprise](#claude-for-teams-and-enterprise)                     | [Ausgabenbericht in Organisationsanalysen](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) | Ausgabenlimits in Admin-Einstellungen | [Ausgabenbericht CSV](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans); [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) bei Enterprise |
| [Claude Console (API)](#claude-console)                                                  | [Console-Nutzungsseite](https://platform.claude.com/usage)                                                                                     | Workspace-Ausgabenlimits              | [Console-Dashboard](https://platform.claude.com/claude-code), [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api)                                                    |
| [Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry](#cloud-providers) | Ihre Cloud-Abrechnungskonsole                                                                                                                  | Ihre Cloud-Budgetkontrollen           | [OpenTelemetry](/docs/de/monitoring-usage) oder ein [LLM-Gateway](/docs/de/llm-gateway)                                                                                                                                                 |

[OpenTelemetry-Export](/docs/de/monitoring-usage) funktioniert bei jedem Setup und ist die einzige Option, die Pro-Benutzer-Token- und Kostenmetriken in Echtzeit in Ihren eigenen Observability-Stack streamt.

<h3 id="report-spend-at-your-contracted-rates">
  Ausgaben zu Ihren vertraglich vereinbarten Sätzen melden
</h3>

Standardmäßig berechnet Claude Code jede Kostenzahl, die es Entwicklern anzeigt, zum Listenpreis, daher stimmen die Zahlen in `/usage`, der Statuszeile und OpenTelemetry nicht mit Ihrer Rechnung überein, wenn Ihre Organisation vertraglich vereinbarte Sätze zahlt. Um sie abzustimmen, setzen Sie die verwaltete Einstellung [`modelPricing`](/docs/de/settings-reference#modelpricing) auf Ihre Sätze. Die Einstellung ändert, was Claude Code meldet, nicht was Anthropic berechnet. Erfordert Claude Code v2.1.242 oder später.

<Steps>
  <Step title="Nehmen Sie die Sätze aus Ihrem Vertrag">
    Geben Sie die Pro-Million-Token-Sätze aus Ihrem Vertrag ein. Claude Code ruft sie nicht aus der Claude Console ab, daher aktualisieren Sie die Einstellung, wenn sich der Vertrag ändert.
  </Step>

  <Step title="Schreiben Sie die Einstellung">
    Setzen Sie `multiplier` unter 1 für einen pauschalen Rabatt oder über 1 für einen Aufschlag, listen Sie die vier Pro-Token-Sätze jedes Modells unter `overrides` auf, oder tun Sie beides. Ein Aufschlag erfordert Claude Code v2.1.271 oder später. Der [`modelPricing`-Eintrag](/docs/de/settings-reference#modelpricing) hat die Form und ein einsatzbereites Beispiel.
  </Step>

  <Step title="Stellen Sie es über verwaltete Einstellungen bereit">
    Liefern Sie es als [verwaltete Einstellungen](/docs/de/managed-settings): servergesteuerte Einstellungen, eine MDM-Richtlinie, `managed-settings.json` oder ein [Richtlinien-Hilfsprogramm](/docs/de/managed-settings#compute-the-policy-with-a-helper-program). Claude Code ignoriert den Schlüssel in Benutzer-, Projekt- und lokalen Einstellungen und in `--settings`.
  </Step>
</Steps>

Um zu bestätigen, dass die Sätze wirksam sind, führen Sie `/usage` in einer Sitzung aus, die [die verwalteten Einstellungen erhalten hat](/docs/de/managed-settings#read-the-source-in-%2Fstatus): Die Zeile `Total cost` des Sitzungsblocks trägt die Notiz `at your organization's configured rates`. Die Zahlen sind immer noch Schätzungen, keine Rechnung. Die Pro-Million-Token-Preise in der `/model`-Auswahl bleiben zum Listenpreis.

<h3 id="claude-for-teams-and-enterprise">
  Claude for Teams und Enterprise
</h3>

Bei Claude for Teams und Enterprise-Plänen wird die Claude Code-Nutzung jedes Mitglieds aus einer Pro-Sitzplatz-Erlaubnis gezogen, die sich in einem rollierenden Fünf-Stunden-Fenster und einem wöchentlichen Fenster zurückgesetzt. Die Erlaubnis wird mit Claude Chat und Cowork geteilt, und ihre Größe hängt vom [Sitzplatz-Tier](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan) (Standard oder Premium) ab. Ihre Kontrollen befinden sich in der claude.ai Admin-Konsole, nicht in der Claude Console.

* **Ausgaben anzeigen**: Der [Ausgabenbericht in Organisationsanalysen](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) zeigt geschätzte Ausgaben pro Benutzer und pro Modell mit CSV-Export, täglich aktualisiert. Der Bericht deckt Nutzungsguthaben-Ausgaben ab und wird angezeigt, sobald Nutzungsguthaben aktiviert sind. Die Nutzung innerhalb der Sitzplatzerlaubnis wird nicht in Dollar gemessen.
* **Adoption anzeigen**: Das [Analytics-Dashboard](https://claude.ai/analytics/claude-code) zeigt täglich aktive Benutzer, Sitzungen und Beitragskennzahlen mit CSV-Export von Beitragsdaten. Siehe [Team-Nutzung mit Analytics verfolgen](/docs/de/analytics).
* **Ausgaben begrenzen**: Die Sitzplatzerlaubnis ist die Standard-Obergrenze. Um Mitgliedern zu ermöglichen, darüber hinauszugehen, aktivieren Sie [Nutzungsguthaben](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) und legen Sie Ausgabenlimits auf Organisations-, Gruppen- oder einzelner Mitgliedsebene fest.
* **Pro-Benutzer-Zahlen abrufen**: Im Enterprise-Plan gibt die [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) Pro-Benutzer-Nutzungs- und Kostenberichte über Claude-Oberflächen hinweg zurück, einschließlich Claude Code. Ein Primary Owner erstellt einen Schlüssel mit dem `read:analytics`-Bereich bei [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys). Im Teams-Plan exportieren Sie den [Ausgabenbericht CSV](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans), der Token-Nutzung und geschätzte Ausgaben pro Benutzer und pro Modell auflistet.

Der [Claude Enterprise-Verbrauchsleitfaden](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide) ist die Planungsreferenz für Administratoren. Er erklärt, wie sich der Verbrauch über Claude Chat, Claude Code und Cowork unterscheidet, und gibt Pro-Benutzer-Dollar-Ausgangspunkte für die Budgetierung. Budgetieren Sie mehr für einen Coding-Sitzplatz als für einen Chat-Sitzplatz: Jeder Claude Code-Zug enthält Dateiinhalte, Tool-Aufrufe und mehrstufiges Denken, daher kann eine Debugging-Sitzung mehr verbrauchen als ein Tag Chat.

<h3 id="claude-console">
  Claude Console
</h3>

API-Organisationen verwalten Claude Code-Ausgaben über [Workspaces](https://platform.claude.com/docs/en/build-with-claude/workspaces). Sie können [Workspace-Ausgabenlimits festlegen](https://platform.claude.com/docs/en/build-with-claude/workspaces#workspace-limits) für die gesamten Claude Code-Ausgaben und [Kosten- und Nutzungsberichte anzeigen](https://platform.claude.com/docs/en/build-with-claude/workspaces#usage-and-cost-tracking) in der Console.

<Note>
  Wenn Sie Claude Code zum ersten Mal mit Ihrem Claude Console-Konto authentifizieren, wird automatisch ein Workspace namens „Claude Code" für Sie erstellt. Dieser Workspace bietet zentrale Kostenverfolgung und Verwaltung für alle Claude Code-Nutzung in Ihrer Organisation. Sie können keine API-Schlüssel für diesen Workspace erstellen; er ist ausschließlich für Claude Code-Authentifizierung und -Nutzung.

  Für Organisationen mit benutzerdefinierten Ratenlimits zählt Claude Code-Verkehr in diesem Workspace zu den gesamten API-Ratenlimits Ihrer Organisation. Sie können ein [Workspace-Ratenlimit](https://platform.claude.com/docs/en/api/rate-limits#setting-lower-limits-for-workspaces) auf der Limits-Seite dieses Workspace in der Claude Console festlegen, um Claude Code's Anteil zu begrenzen und andere Produktions-Workloads zu schützen.
</Note>

Für Pro-Benutzer-Berichterstattung zeigt das [Console-Dashboard](https://platform.claude.com/claude-code) Ausgaben und akzeptierte Zeilen pro Mitglied, und die [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) gibt die gleichen täglichen Pro-Benutzer-Metriken programmgesteuert mit einem [Admin API-Schlüssel](https://platform.claude.com/settings/admin-keys) zurück. Siehe [Analytics für API-Kunden](/docs/de/analytics#access-analytics-for-api-customers).

<h4 id="rate-limit-recommendations">
  Empfehlungen für Ratenlimits
</h4>

Beim Einrichten von Claude Code für Teams sollten Sie diese Token Pro Minute (TPM) und Anfragen Pro Minute (RPM) pro Benutzer-Empfehlungen basierend auf Ihrer Organisationsgröße berücksichtigen:

| Team-Größe       | TPM pro Benutzer | RPM pro Benutzer |
| ---------------- | ---------------- | ---------------- |
| 1–5 Benutzer     | 200.000–300.000  | 5–7              |
| 5–20 Benutzer    | 100.000–150.000  | 2,5–3,5          |
| 20–50 Benutzer   | 50.000–75.000    | 1,25–1,75        |
| 50–100 Benutzer  | 25.000–35.000    | 0,62–0,87        |
| 100–500 Benutzer | 15.000–20.000    | 0,37–0,47        |
| 500+ Benutzer    | 10.000–15.000    | 0,25–0,35        |

Wenn Sie beispielsweise 200 Benutzer haben, könnten Sie 20.000 TPM für jeden Benutzer anfordern, oder insgesamt 4 Millionen TPM (200\*20.000 = 4 Millionen).

Die TPM pro Benutzer sinkt mit zunehmender Team-Größe, da in größeren Organisationen weniger Benutzer Claude Code gleichzeitig verwenden. Diese Ratenlimits gelten auf Organisationsebene, nicht pro einzelnem Benutzer, was bedeutet, dass einzelne Benutzer vorübergehend mehr als ihren berechneten Anteil verbrauchen können, wenn andere den Service nicht aktiv nutzen.

<Note>
  Wenn Sie Szenarien mit ungewöhnlich hoher gleichzeitiger Nutzung erwarten (z. B. Live-Schulungssitzungen mit großen Gruppen), benötigen Sie möglicherweise höhere TPM-Zuordnungen pro Benutzer.
</Note>

<h3 id="cloud-providers">
  Cloud-Anbieter
</h3>

Bei Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry wird Claude Code pro Token an Ihr Cloud-Konto abgerechnet, und Ausgabenkontrollen befinden sich in der Abrechnungskonsole Ihres Cloud-Anbieters. Claude Code sendet keine Metriken aus Ihrer Cloud zurück an Anthropic, daher decken die [Analytics-Dashboards](/docs/de/analytics) und die Claude Code Analytics API diese Nutzung nicht ab.

Für Pro-Benutzer-Kostenzuordnung haben Sie drei Optionen:

* **OpenTelemetry**: [Exportieren Sie Metriken](/docs/de/monitoring-usage) von der Maschine jedes Entwicklers in Ihren eigenen Observability-Stack. Dies gibt Ihnen Pro-Benutzer-Token-Zählungen, Kosten und Tool-Aktivität unabhängig vom Anbieter.
* **Ein Claude Apps Gateway**: Ein selbst gehostetes [Claude Apps Gateway](/docs/de/claude-apps-gateway) bietet Pro-Benutzer-Nutzungszuordnung, OTLP-Metriken mit Token-Zählungen und [Pro-Benutzer-Ausgabenlimits](/docs/de/claude-apps-gateway-spend-limits) auf diesen Anbietern.
* **Ein LLM-Gateway**: Leiten Sie den gesamten Claude Code-Verkehr durch einen Proxy, der Ausgaben pro Schlüssel verfolgt. Mehrere große Unternehmen berichteten über die Verwendung von [LiteLLM](/docs/de/llm-gateway), einem Open-Source-Tool, das [Ausgaben nach Schlüssel verfolgt](https://docs.litellm.ai/docs/proxy/virtual_keys#tracking-spend). Dieses Projekt ist nicht mit Anthropic verbunden und wurde nicht auf Sicherheit überprüft.

<h3 id="when-a-developer-asks-about-a-limit">
  Wenn ein Entwickler eine Frage zu einem Limit stellt
</h3>

Entwickler bringen Limit-Fragen normalerweise zu ihrem Administrator, daher ist es hilfreich zu wissen, welche Obergrenze sie erreicht haben. Diese Situationen bedeuten unterschiedliche Dinge:

* **„Sie haben Ihr Sitzungslimit erreicht" oder „Sie haben Ihr wöchentliches Limit erreicht"**: Ein sitzplatzbasiertes Nutzungsfenster bei einem Abonnement-Plan, das über alle Modelle hinweg geteilt wird, daher kann der Entwickler den Zugriff nicht durch Wechsel von Modellen mit `/model` wiederherstellen. Die Nachricht zeigt, wenn sich das Fenster zurückgesetzt. Nach der modellspezifischen Nachricht „Sie haben Ihr Opus-Limit erreicht" oder „Sie haben Ihr Sonnet-Limit erreicht" ermöglicht das Wechseln zu einem Modell außerhalb dieser Familie mit `/model` dem Entwickler, weiterarbeiten zu können. Siehe [Nutzungslimit-Fehler](/docs/de/errors#youve-hit-your-session-limit). Was der Entwickler in der Zwischenzeit tun kann:
  * Führen Sie `/usage-credits` aus, um Nutzung über die Erlaubnis hinaus anzufordern, wenn Sie [Nutzungsguthaben](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) aktiviert haben.
  * Bei Claude Code v2.1.234 oder später [warten und fahren Sie die unterbrochene Aufgabe nach dem Zurücksetzen automatisch fort](/docs/de/interactive-mode#wait-for-a-usage-limit-to-reset); dieser Abschnitt listet auf, wann Claude Code das Warten von selbst startet und wann der Entwickler es aus `/rate-limit-options` auswählt. Um für Ihre Flotte zu steuern, ob Claude Code das Warten von selbst startet, setzen Sie [`autoContinueAtUsageLimit`](/docs/de/settings-reference#autocontinueatusagelimit) in [verwalteten Einstellungen](/docs/de/settings#settings-precedence).
* **„Sie haben Ihr individuelles Ausgabenlimit erreicht", „Ausgabenlimit der Organisation" oder „gemeinsames Budget des Teams"**: Die Anfrage des Entwicklers würde an Nutzungsguthaben abgerechnet, und diese Guthaben haben ein Ausgabenlimit erreicht, das Sie festgelegt haben. Um dem Entwickler zu ermöglichen, fortzufahren, gehen Sie zu [**Admin-Einstellungen > Nutzung**](https://claude.ai/admin-settings/usage) und erhöhen Sie das Limit, das die Nachricht nennt. Wenn die Nachricht auch eine Plan-Zurücksetztzeit nennt, kann der Entwickler stattdessen bis dahin warten. Siehe [die Fehlerreferenz](/docs/de/errors#youve-hit-your-monthly-spend-limit) für jede Variante.
* **Eine Ausgabenlimit-Nachricht von einem [Claude Apps Gateway](/docs/de/claude-apps-gateway)**: Der Entwickler hat eine Ausgabenbegrenzung überschritten, die Sie auf Ihrem selbst gehosteten Gateway festgelegt haben, und das Gateway blockiert seine Anfragen, bis sich der Zeitraum zurückgesetzt oder Sie die Begrenzung erhöhen. Siehe [Gateway-Ausgabenlimits](/docs/de/claude-apps-gateway-spend-limits) für Begrenzungen, Zurücksetzzeitpläne und die Nachricht, die der Entwickler sieht.
* **Eine Kontext- oder Auto-Compact-Warnung**: Kein Nutzungslimit. Das Gespräch ist der [Auto-Compact-Fenster](/docs/de/model-config#set-the-auto-compact-window) der Sitzung nahe gewachsen, der Schwellenwert, bei dem Claude Code ältere Verlauf zusammenfasst, um Platz freizugeben. Verweisen Sie den Entwickler auf [Token-Nutzung reduzieren](#reduce-token-usage).
* **Unerwartet hohe Ausgaben bei einem API- oder Cloud-Anbieter-Plan**: Normalerweise zurückzuführen auf lange Sitzungen, die nie gelöscht wurden, oder auf Opus, das als Standard-Modell belassen wurde. Die Gewohnheiten mit der höchsten Auswirkung zum Teilen sind das Löschen zwischen nicht verwandten Aufgaben und das Anpassen des Modells an die Aufgabe, beide in [Token-Nutzung reduzieren](#reduce-token-usage) behandelt.

<h3 id="agent-team-token-costs">
  Token-Kosten für Agent-Teams
</h3>

[Agent-Teams](/docs/de/agent-teams) starten mehrere Claude Code-Instanzen, jede mit ihrem eigenen Kontextfenster. Die Token-Nutzung skaliert mit der Anzahl der aktiven Teammates und wie lange jeder läuft.

Um Agent-Team-Kosten überschaubar zu halten:

* Verwenden Sie Sonnet für Teammates. Es bietet ein Gleichgewicht zwischen Fähigkeit und Kosten für Koordinationsaufgaben.
* Halten Sie Teams klein. Jeder Teammate führt sein eigenes Kontextfenster aus, daher ist die Token-Nutzung ungefähr proportional zur Team-Größe.
* Halten Sie Spawn-Prompts fokussiert. Teammates laden CLAUDE.md, MCP-Server und Skills automatisch, aber alles im Spawn-Prompt trägt von Anfang an zu ihrem Kontext bei.
* Fahren Sie Teammates herunter, wenn ihre Arbeit erledigt ist. Jeder aktive Teammate verbraucht weiterhin Token, bis er beendet wird oder die Sitzung endet.
* Agent-Teams sind standardmäßig deaktiviert. Setzen Sie `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` in Ihrer [settings.json](/docs/de/settings) oder Umgebung, um sie zu aktivieren. Siehe [Agent-Teams aktivieren](/docs/de/agent-teams#enable-agent-teams).

<h2 id="reduce-token-usage">
  Reduzieren Sie die Token-Nutzung
</h2>

Token-Kosten skalieren mit der Kontextgröße: Je mehr Kontext Claude verarbeitet, desto mehr Token verwenden Sie. Claude Code optimiert Kosten automatisch durch [Prompt Caching](/docs/de/prompt-caching), das Kosten für wiederholte Inhalte wie Systemprompts reduziert, und Auto-Compaction, das Gesprächsverlauf zusammenfasst, wenn sich dem Kontextlimit genähert wird.

Die folgenden Strategien helfen Ihnen, den Kontext klein zu halten und die Kosten pro Nachricht zu reduzieren.

<h3 id="manage-context-proactively">
  Verwalten Sie den Kontext proaktiv
</h3>

Verwenden Sie `/usage`, um Ihre aktuelle Token-Nutzung zu überprüfen, oder [konfigurieren Sie Ihre Statuszeile](/docs/de/statusline#context-window-usage), um sie kontinuierlich anzuzeigen.

* **Zwischen Aufgaben löschen**: Verwenden Sie `/clear`, um neu zu beginnen, wenn Sie zu nicht verwandter Arbeit wechseln. Veralteter Kontext verschwendet Token bei jeder nachfolgenden Nachricht. Verwenden Sie `/rename` vor dem Löschen, damit Sie die Sitzung später leicht finden können, dann `/resume`, um zu ihr zurückzukehren.
* **Fügen Sie benutzerdefinierte Compaction-Anweisungen hinzu**: `/compact Focus on code samples and API usage` teilt Claude mit, was während der Zusammenfassung beibehalten werden soll. In einer neuen Sitzung gibt `/compact` `Not enough messages to compact.` aus, da noch kein Gesprächsverlauf zum Zusammenfassen vorhanden ist.

Sie können das Compaction-Verhalten auch in Ihrer CLAUDE.md-Datei im Stammverzeichnis Ihres Projekts anpassen:

```markdown theme={null}
# Compact instructions

When you are using compact, please focus on test output and code changes
```

<h3 id="choose-the-right-model">
  Wählen Sie das richtige Modell
</h3>

Sonnet bewältigt die meisten Codierungsaufgaben gut und kostet weniger als Opus. Reservieren Sie Opus für komplexe architektonische Entscheidungen oder mehrstufiges Denken. Verwenden Sie `/model`, um Modelle während einer Sitzung zu wechseln, oder legen Sie einen Standard in `/config` fest. Ein Wechsel zu Opus gilt auch für die [Subagents, die das Modell Ihrer Sitzung erben](/docs/de/model-config#setting-your-model). Für einfache Subagent-Aufgaben geben Sie `model: haiku` in Ihrer [Subagent-Konfiguration](/docs/de/sub-agents#choose-a-model) an.

<h3 id="reduce-mcp-server-overhead">
  Reduzieren Sie den MCP-Server-Overhead
</h3>

MCP-Tool-Definitionen werden [standardmäßig aufgeschoben](/docs/de/mcp#scale-with-mcp-tool-search), daher treten nur Tool-Namen und Server-Anweisungen in den Kontext ein, bis Claude ein bestimmtes Tool verwendet. Führen Sie `/context` aus, um zu sehen, was Platz verbraucht.

* **Bevorzugen Sie CLI-Tools, wenn verfügbar**: Tools wie `gh`, `aws`, `gcloud` und `sentry-cli` sind immer noch kontexteffektiver als MCP-Server, da sie keine Pro-Tool-Auflistung hinzufügen. Claude kann CLI-Befehle direkt ausführen.
* **Deaktivieren Sie ungenutzte Server**: Führen Sie `/mcp` aus, um konfigurierte Server anzuzeigen und alle zu deaktivieren, die Sie nicht aktiv verwenden.

<h3 id="install-code-intelligence-plugins-for-typed-languages">
  Installieren Sie Code-Intelligence-Plugins für typisierte Sprachen
</h3>

[Code-Intelligence-Plugins](/docs/de/plugins/code-intelligence) geben Claude präzise Symbol-Navigation statt textbasierter Suche, wodurch unnötige Dateileser beim Erkunden unbekannten Codes reduziert werden. Ein einzelner „Gehe zu Definition"-Aufruf ersetzt, was sonst ein Grep gefolgt vom Lesen mehrerer Kandidatendateien sein könnte. Installierte Sprachserver melden auch Typfehler automatisch nach Bearbeitungen, sodass Claude Fehler erkennt, ohne einen Compiler auszuführen.

<h3 id="offload-processing-to-hooks-and-skills">
  Verlagern Sie die Verarbeitung auf Hooks und Skills
</h3>

Benutzerdefinierte [Hooks](/docs/de/hooks) können Daten vorverarbeiten, bevor Claude sie sieht. Anstatt dass Claude eine 10.000-Zeilen-Protokolldatei liest, um Fehler zu finden, kann ein Hook nach `ERROR` suchen und nur übereinstimmende Zeilen zurückgeben, wodurch der Kontext von Zehntausenden Token auf Hunderte reduziert wird.

Ein [Skill](/docs/de/skills) kann Claude Domänenwissen geben, sodass es nicht erkunden muss. Beispielsweise könnte ein „codebase-overview"-Skill die Architektur Ihres Projekts, wichtige Verzeichnisse und Namenskonventionen beschreiben. Wenn Claude den Skill aufruft, erhält es diesen Kontext sofort, anstatt Token zu verschwenden, um mehrere Dateien zu lesen, um die Struktur zu verstehen.

Beispielsweise filtert dieser PreToolUse-Hook die Testausgabe, um nur Fehler anzuzeigen:

<Tabs>
  <Tab title="settings.json">
    Fügen Sie dies zu Ihrer [settings.json](/docs/de/settings#where-settings-live) hinzu, um den Hook vor jedem Bash-Befehl auszuführen:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "~/.claude/hooks/filter-test-output.sh"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="filter-test-output.sh">
    Der Hook ruft dieses Skript auf. Erstellen Sie den Ordner mit `mkdir -p ~/.claude/hooks`, speichern Sie das Skript unten als `~/.claude/hooks/filter-test-output.sh` und machen Sie es ausführbar mit `chmod +x ~/.claude/hooks/filter-test-output.sh`. Es überprüft, ob der Befehl ein Test-Runner ist, und ändert ihn, um nur Fehler anzuzeigen:

    ```bash theme={null}
    #!/bin/bash
    input=$(cat)
    cmd=$(echo "$input" | jq -r '.tool_input.command')

    # If running tests, filter to show only failures
    if [[ "$cmd" =~ ^(npm test|pytest|go test) ]]; then
      filtered_cmd="$cmd 2>&1 | grep -A 5 -E '(FAIL|ERROR|error:)' | head -100"
      echo "$input" | jq --arg filtered "$filtered_cmd" \
        '{hookSpecificOutput: {hookEventName: "PreToolUse", permissionDecision: "allow", updatedInput: (.tool_input + {command: $filtered})}}'
    else
      echo "{}"
    fi
    ```
  </Tab>
</Tabs>

Um die Einrichtung zu überprüfen, führen Sie `/hooks` aus und überprüfen Sie, dass der Hook unter PreToolUse angezeigt wird. Sie können Claude Code auch mit `claude --debug-file ./claude-debug.txt` starten und Claude bitten, `npm test` auszuführen. Wenn der Hook den Befehl umschreibt, enthält diese Protokolldatei eine Zeile `modified tool input keys`, die `command` und die anderen Bash-Eingabefelder auflistet.

<h3 id="move-instructions-from-claude-md-to-skills">
  Verschieben Sie Anweisungen von CLAUDE.md zu Skills
</h3>

Ihre [CLAUDE.md](/docs/de/memory)-Datei wird beim Sitzungsstart in den Kontext geladen. Wenn sie detaillierte Anweisungen für spezifische Workflows enthält (wie PR-Reviews oder Datenbankmigrationen), sind diese Token vorhanden, auch wenn Sie nicht verwandte Arbeit erledigen. [Skills](/docs/de/skills) werden bei Bedarf nur geladen, wenn sie aufgerufen werden, daher hält das Verschieben spezialisierter Anweisungen in Skills Ihren Basis-Kontext kleiner. Streben Sie danach, CLAUDE.md unter 200 Zeilen zu halten, indem Sie nur das Wesentliche einbeziehen.

<h3 id="adjust-extended-thinking">
  Passen Sie das erweiterte Denken an
</h3>

Erweitertes Denken ist standardmäßig aktiviert, da es die Leistung bei komplexen Planungs- und Denkaufgaben erheblich verbessert. Thinking-Token werden als Output-Token abgerechnet, und das Standard-Budget kann je nach Modell Zehntausende Token pro Anfrage betragen.

Für einfachere Aufgaben, bei denen tiefes Denken nicht erforderlich ist, können Sie Kosten reduzieren, indem Sie die [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level) mit `/effort` senken oder in `/model`, oder indem Sie Denken in `/config` deaktivieren. Sie können Denken auf Opus 5.5 oder den Fable-Modellen nicht ausschalten, die immer erweitertes Denken verwenden.

Auf Modellen mit einem [festen Thinking-Budget](/docs/de/model-config#adaptive-reasoning-and-fixed-thinking-budgets) können Sie das Budget auch senken, indem Sie die `MAX_THINKING_TOKENS` [Umgebungsvariable](/docs/de/env-vars) setzen, beispielsweise `MAX_THINKING_TOKENS=8000`. Adaptive-Reasoning-Modelle ignorieren Budgets ungleich Null, daher verwenden Sie stattdessen Anstrengungsstufen.

<h3 id="delegate-verbose-operations-to-subagents">
  Delegieren Sie ausführliche Operationen an Subagents
</h3>

Das Ausführen von Tests, das Abrufen von Dokumentation oder das Verarbeiten von Protokolldateien kann erheblichen Kontext verbrauchen. Delegieren Sie diese an [Subagents](/docs/de/sub-agents#isolate-high-volume-operations), sodass die ausführliche Ausgabe im Kontext des Subagent bleibt, während nur eine Zusammenfassung zu Ihrem Hauptgespräch zurückkehrt.

<h3 id="manage-agent-team-costs">
  Verwalten Sie Agent-Team-Kosten
</h3>

Agent-Teams verwenden ungefähr 7-mal mehr Token als Standard-Sitzungen, wenn Teammates im Plan Mode laufen, da jeder Teammate sein eigenes Kontextfenster verwaltet und als separate Claude-Instanz läuft. Halten Sie Team-Aufgaben klein und in sich geschlossen, um die Token-Nutzung pro Teammate zu begrenzen. Siehe [Agent-Teams](/docs/de/agent-teams) für Details.

<h3 id="write-specific-prompts">
  Schreiben Sie spezifische Prompts
</h3>

Vage Anfragen wie „Verbessern Sie diese Codebasis" lösen breites Scannen aus. Spezifische Anfragen wie „Fügen Sie Eingabevalidierung zur Login-Funktion in auth.ts hinzu" ermöglichen es Claude, effizient mit minimalen Dateileser zu arbeiten.

<h3 id="work-efficiently-on-complex-tasks">
  Arbeiten Sie effizient an komplexen Aufgaben
</h3>

Für längere oder komplexere Arbeiten helfen diese Gewohnheiten, verschwendete Token durch das Gehen des falschen Weges zu vermeiden:

* **Verwenden Sie Plan Mode für komplexe Aufgaben**: Drücken Sie Shift+Tab, um [Plan Mode](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode) vor der Implementierung zu betreten. Claude erkundet die Codebasis und schlägt einen Ansatz zur Genehmigung vor, was teure Überarbeitungen verhindert, wenn die anfängliche Richtung falsch ist.
* **Korrigieren Sie den Kurs früh**: Wenn Claude in die falsche Richtung geht, drücken Sie Escape, um sofort zu stoppen. Verwenden Sie `/rewind` oder doppeltippen Sie Escape, um das Gespräch und den Code zu einem vorherigen Checkpoint wiederherzustellen.
* **Geben Sie Verifizierungsziele an**: Fügen Sie Testfälle ein, fügen Sie Screenshots ein oder definieren Sie erwartete Ausgabe in Ihrem Prompt. Wenn Claude seine eigene Arbeit verifizieren kann, erkennt es Probleme, bevor Sie Korrektionen anfordern müssen.
* **Testen Sie schrittweise**: Schreiben Sie eine Datei, testen Sie sie, dann fahren Sie fort. Dies erkennt Probleme früh, wenn sie billig zu beheben sind.

<h2 id="background-token-usage">
  Hintergrund-Token-Nutzung
</h2>

Claude Code verwendet Token für einige Hintergrund-Funktionalität, auch wenn untätig:

* **Gesprächszusammenfassung**: Hintergrund-Jobs, die vorherige Gespräche für die `claude --resume`-Funktion zusammenfassen
* **Befehlsverarbeitung**: Einige Befehle wie `/usage` können Anfragen generieren, um den Status zu überprüfen

Diese Hintergrund-Prozesse verbrauchen eine kleine Menge Token (typischerweise unter 0,04 USD pro Sitzung), auch ohne aktive Interaktion.

Wenn Prompt-Vorschläge aktiviert sind, sendet Claude Code auch eine kurze Anfrage an das Modell, das Ihre Sitzung verwendet, nachdem Claude antwortet, um [Ihren nächsten Prompt vorzuschlagen](/docs/de/interactive-mode#prompt-suggestions). Diese Anfrage nutzt den Prompt-Cache des Gesprächs wieder, daher besteht sie hauptsächlich aus Cache-Lesevorgängen plus einigen Ausgabe-Token. Claude Code [überspringt dies, wenn Ihr Konto sich dem Nutzungslimit nähert oder dieses erreicht hat](/docs/de/interactive-mode#when-claude-code-skips-suggestions). Um diese Anfragen zu stoppen, [deaktivieren Sie Prompt-Vorschläge](/docs/de/interactive-mode#turn-prompt-suggestions-off).

<h2 id="why-usage-climbs-in-a-long-session">
  Warum die Nutzung in einer langen Sitzung ansteigt
</h2>

Eine Sitzung, die stundenlang offen war, kann viel mehr von Ihren Planlimits verbrauchen als Ihre Aktivität vermuten lässt, normalerweise aus einem dieser Gründe:

* **Langer Kontext**: Claude Code sendet Ihre vollständige Konversation mit jeder Anfrage, und jedes Mal, wenn Claude Tools verwendet, sendet es eine weitere Anfrage mit diesem Batch von Tool-Ergebnissen. Mit [Prompt Caching](/docs/de/prompt-caching) liest Claude Code diese Historie mit der [zwischengespeicherten Token-Rate](https://platform.claude.com/docs/en/about-claude/pricing), sodass eine einzeilige Frage in einer Sitzung, die den ganzen Tag offen war, immer noch Nutzung für die gesamte Konversation verursacht. Siehe [Kontext proaktiv verwalten](#manage-context-proactively) für Möglichkeiten, Ihren Kontext klein zu halten
* **Cache-Misses**: Ihre erste Nachricht nach einer Pause, die länger als die [Cache-Lebensdauer](/docs/de/prompt-caching#cache-lifetime) ist, verfehlt den Cache und verarbeitet Ihren vollständigen Kontext neu. Die Lebensdauer beträgt eine Stunde bei einem Abonnement und sinkt auf fünf Minuten, sobald Sie [Nutzungsguthaben](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) in Anspruch nehmen; bei einem API-Schlüssel oder Cloud-Provider beträgt sie standardmäßig fünf Minuten. Um die einstündige Lebensdauer beizubehalten, während Sie Nutzungsguthaben in Anspruch nehmen, [wählen Sie die TTL selbst](/docs/de/prompt-caching#choose-the-ttl-yourself). Bei Pro- und Max-Plänen bietet Claude Code beim Fortsetzen einer großen Sitzung nach einer langen Pause an, [von einer Zusammenfassung fortzufahren](/docs/de/sessions#resume-from-a-summary), sodass spätere Anfragen nicht die vollständige Historie tragen
* **Geplante Aufgaben**: Eine [geplante Aufgabe](/docs/de/scheduled-tasks) wird in ihrem Intervall ausgelöst, auch während die Sitzung untätig ist, und sendet jedes Mal Ihren vollständigen Kontext
* **Sitzungsübergreifende Nachrichten**: Claude Code liefert eine [Nachricht aus einer anderen Ihrer Sitzungen](/docs/de/cross-session-messaging) als neuen Zug, wenn diese Sitzung untätig ist, und sendet jedes Mal Ihren vollständigen Kontext. Um eingehende Nachrichten zu halten, anstatt sie zu liefern, setzen Sie [`crossSessionInbound`](/docs/de/settings-reference#crosssessioninbound) auf `hold`
* **Zielüberprüfungen**: Während Hintergrundarbeit ein aktives [Ziel](/docs/de/goal) wartet, fragt Claude Code [Claude, um diese Arbeit zu überprüfen](/docs/de/goal#background-work-defers-evaluation), auch wenn die Sitzung untätig ist, und startet einen neuen Zug, der Ihren vollständigen Kontext sendet. Claude Code startet höchstens drei untätige Überprüfungen pro Ziel zwischen Ihren Eingaben. Vor v2.1.246 waren untätige Überprüfungen unbegrenzt. Um Überprüfungen auszuschalten, setzen Sie [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/de/env-vars) auf `0`. Untätige Überprüfungen erfordern Claude Code v2.1.236 oder später
* **Agent-Teamkollegen**: Jeder aktive [Teamkollege](#agent-team-token-costs) verbraucht weiterhin Token, bis er beendet wird
* **Komprimierung**: `/compact` liest die Konversation, die es zusammenfasst, sodass [das Komprimieren eines großen Kontexts](/docs/de/prompt-caching#compacting-the-conversation) selbst eine große Anfrage ist. Wenn Sie einen Neuanfang statt Kontinuität möchten, kostet `/clear` nichts

Bei einem Pro-, Max-, Team- oder Enterprise-Plan kennzeichnet die `/usage`-Aufschlüsselung Verhaltensweisen, die 10 % oder mehr Ihrer kürzlichen Nutzung ausmachen, wie langer Kontext oder Cache-Misses, jeweils mit einem Tipp, um es zu reduzieren.

<h2 id="understanding-changes-in-claude-code-behavior">
  Änderungen im Verhalten von Claude Code verstehen
</h2>

Claude Code erhält regelmäßig Updates, die ändern können, wie Funktionen funktionieren, einschließlich der Kostenberechnung. Führen Sie `claude --version` aus, um Ihre aktuelle Version zu überprüfen.

Bei Fragen zur Abrechnung für Ihr spezifisches Konto wenden Sie sich bitte an den Anthropic-Support über den In-Product-Messenger:

* **Abonnementpläne** (Pro, Max, Team, Enterprise): Melden Sie sich unter [claude.ai](https://claude.ai) an, klicken Sie auf Ihre Initialen in der unteren linken Ecke und wählen Sie **Hilfe erhalten**
* **Console (API) Abrechnung**: Melden Sie sich unter [platform.claude.com](https://platform.claude.com) an, klicken Sie auf Ihre Initialen und wählen Sie **Hilfe erhalten**

Siehe [So erhalten Sie Support](https://support.claude.com/en/articles/9015913-how-to-get-support) für den vollständigen Ablauf, einschließlich wer auf jedem Plan einen menschlichen Agenten erreichen kann.
