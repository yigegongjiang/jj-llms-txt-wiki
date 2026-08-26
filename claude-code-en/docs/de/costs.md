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

Der Session-Block oben in `/usage` zeigt detaillierte Token-Nutzungsstatistiken für Ihre aktuelle Sitzung. Die Dollarzahl ist eine Schätzung, die lokal aus Token-Zählungen berechnet wird, und kann sich von Ihrer tatsächlichen Rechnung unterscheiden. Für verbindliche Abrechnung siehe die Nutzungsseite in der [Claude Console](https://platform.claude.com/usage).

```text theme={null}
Total cost:            $0.55
Total duration (API):  6m 19.7s
Total duration (wall): 6h 33m 10.2s
Total code changes:    0 lines added, 0 lines removed
```

Auf einem Pro-, Max-, Team- oder Enterprise-Plan zeigt `/usage` auch eine Aufschlüsselung dessen, was gegen Ihre Planlimits zählt. Es ordnet die aktuelle Nutzung Skills, Subagenten, Plugins und einzelnen MCP-Servern zu, wobei jeder als Prozentsatz des Gesamtbetrags angezeigt wird. Drücken Sie `d` oder `w`, um zwischen den letzten 24 Stunden und den letzten 7 Tagen zu wechseln. Die Zahlen sind ungefähr und werden aus dem lokalen Sitzungsverlauf auf diesem Computer berechnet, daher ist die Nutzung von anderen Geräten oder claude.ai nicht enthalten.

Wenn die Anfrage für Ihre Planlimits fehlschlägt, meistens weil der Nutzungs-Endpunkt Rate-Limited ist, zeigt `/usage` die letzten Nutzungsbalken an, die auf diesem Computer in den letzten 60 Minuten geladen wurden, zusammen mit einer Notiz `Showing last-known usage`, die angibt, wie lange diese Daten her sind. Drücken Sie `r`, um es erneut zu versuchen; ein erfolgreicher Versuch ersetzt die letzten bekannten Balken durch aktuelle Daten. Ohne einen Snapshot aus den letzten 60 Minuten meldet `/usage`, dass der Nutzungs-Endpunkt Rate-Limited ist, und bietet die gleiche Wiederholungsverknüpfung an. Vor v2.1.208 zeigte eine Rate-Limited-Anfrage in einer Sitzung, die noch keine Nutzung geladen hatte, immer den Fehler ohne Balken an.

In der [VS Code-Erweiterung](/docs/de/vs-code#check-account-and-usage) wird die gleiche Aufschlüsselung im Dialog „Konto & Nutzung" mit einem Tag- und Woche-Umschalter angezeigt. Erfordert Claude Code v2.1.174 oder später.

<h3 id="set-a-spend-limit-on-pro-and-max">
  Ausgabenlimit für Pro und Max festlegen
</h3>

Bei Pro- und Max-Plänen können Sie mit dem Befehl `/usage-credits` ein Dialogfeld in der CLI öffnen, in dem Sie [Nutzungsguthaben](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) verwalten. Aus dem Dialogfeld können Sie:

* Nutzungsguthaben für Ihr Konto aktivieren
* Mehr Nutzungsguthaben kaufen, entweder ein aufgelistetes Paket oder einen benutzerdefinierten Betrag
* Ihr monatliches Ausgabenlimit festlegen, ändern oder entfernen
* Auto-Reload konfigurieren, das automatisch mehr Nutzungsguthaben kauft, wenn Ihr Saldo unter einen von Ihnen festgelegten Schwellenwert fällt

Bei Claude Code-Versionen vor v2.1.207 und bei Konten, bei denen das In-CLI-Dialogfeld nicht verfügbar ist, öffnet `/usage-credits` stattdessen die Seite für Nutzungsguthaben-Abrechnung in Ihrem Browser. Bei Team- und Enterprise-Plänen erhalten Mitglieder mit Abrechnungszugriff die gleiche Browser-Seite, und Mitglieder ohne Abrechnungszugriff senden eine Anfrage aus der CLI, in der sie ihren Administrator bitten, Nutzungsguthaben zu aktivieren oder das Limit zu erhöhen.

Das Ändern des monatlichen Ausgabenlimits erfordert Abrechnungszugriff auf dem Konto. Wenn Sie das Limit erreichen, während Sie noch Nutzungsguthaben verfügbar haben, fordert Claude Code Sie auf, das Limit zu erhöhen oder zu entfernen, damit Sie ohne Unterbrechung der CLI fortfahren können.

Beträge, die Sie in das Dialogfeld eingeben, wie z. B. ein benutzerdefinierter Kaufbetrag, das monatliche Ausgabenlimit oder der Auto-Reload-Schwellenwert und das Ziel, müssen Ziffern sein, optional gefolgt von einem Punkt und einer oder zwei Dezimalstellen, z. B. `20` oder `20.50`. Jede andere Eingabe, einschließlich Kommas, zeigt einen Inline-Fehler an und wird nicht gespeichert. Versionen vor v2.1.207 zeigen das Dialogfeld nicht an und öffnen stattdessen die Abrechnungsseite.

Claude Code fordert Sie auf, `yes` einzugeben, um jeden Kauf und jede Auto-Reload-Änderung zu bestätigen, unabhängig vom Betrag, und die Kaufbestätigung zeigt die Gesamtsumme nach Steuern an, die Sie genehmigen. Das Ändern des monatlichen Ausgabenlimits erfordert die gleiche eingegebene Bestätigung nur über \$1.000 oder über 1.000 Einheiten einer Nicht-US-Dollar-Abrechnungswährung. Vor v2.1.208 verwendeten Käufe und Auto-Reload-Änderungen diesen Schwellenwert ebenfalls, daher gingen kleinere Beträge durch den Standard-Dialogfluss ohne den zusätzlichen eingegebenen `yes`-Schritt.

Betragsfelder öffnen sich mit einem vorgeschlagenen Wert, und die erste Ziffer, die Sie eingeben, ersetzt den Vorschlag, anstatt ihn anzuhängen. Der Bildschirm, der Nutzungsguthaben aktiviert, wird mit „Abbrechen" ausgewählt geöffnet, daher ist das Aktivieren eine bewusste Auswahl statt eines zufälligen Drückens der Eingabetaste. Beide erfordern Claude Code v2.1.208 oder später.

<h2 id="manage-costs-for-your-organization">
  Verwalten Sie Kosten für Ihre Organisation
</h2>

Welche Kontrollen Sie haben, hängt davon ab, wie Ihre Organisation auf Claude Code zugreift: über einen Claude for Teams oder Enterprise-Plan, die Claude Console oder einen Cloud-Anbieter. Bei Teams- und Enterprise-Plänen wird die Nutzung aus der Sitzplatzerlaubnis jedes Mitglieds gezogen. In der Console und bei Cloud-Anbietern wird die Nutzung pro Token an Ihre Organisation abgerechnet. Wenn Ihre Organisation verschiedene Anmeldeverfahren mischt, wird jeder Entwickler nach dem gemessen, mit dem er sich authentifiziert hat.

Die Tabelle ordnet jedes Setup zu, wo Sie Ausgaben sehen, wo Sie sie begrenzen, und wie Sie Pro-Benutzer-Zahlen abrufen.

| Ihr Setup                                                                                | Ausgaben anzeigen                                                                                                                              | Ausgaben begrenzen                    | Pro-Benutzer-Berichterstattung                                                                                                                                                                                                |
| :--------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Claude for Teams oder Enterprise](#claude-for-teams-and-enterprise)                     | [Ausgabenbericht in Organisationsanalysen](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) | Ausgabenlimits in Admin-Einstellungen | [Ausgabenbericht CSV](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans); [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) bei Enterprise |
| [Claude Console (API)](#claude-console)                                                  | [Console-Nutzungsseite](https://platform.claude.com/usage)                                                                                     | Workspace-Ausgabenlimits              | [Console-Dashboard](https://platform.claude.com/claude-code), [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api)                                                    |
| [Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry](#cloud-providers) | Ihre Cloud-Abrechnungskonsole                                                                                                                  | Ihre Cloud-Budgetkontrollen           | [OpenTelemetry](/docs/de/monitoring-usage) oder ein [LLM-Gateway](/docs/de/llm-gateway)                                                                                                                                                 |

[OpenTelemetry-Export](/docs/de/monitoring-usage) funktioniert bei jedem Setup und ist die einzige Option, die Pro-Benutzer-Token- und Kostenmetriken in Echtzeit in Ihren eigenen Observability-Stack streamt.

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

  Für Organisationen mit benutzerdefinierten Ratenlimits zählt Claude Code-Verkehr in diesem Workspace zu den gesamten API-Ratenlimits Ihrer Organisation. Sie können ein [Workspace-Ratenlimit](https://platform.claude.com/docs/de/api/rate-limits#setting-lower-limits-for-workspaces) auf der Limits-Seite dieses Workspace in der Claude Console festlegen, um Claude Code's Anteil zu begrenzen und andere Produktions-Workloads zu schützen.
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

Entwickler bringen Limit-Fragen normalerweise zu ihrem Administrator, daher ist es hilfreich zu wissen, welche Obergrenze sie erreicht haben. Die drei Situationen bedeuten unterschiedliche Dinge:

* **„Sie haben Ihr Sitzungslimit erreicht" oder „Sie haben Ihr wöchentliches Limit erreicht"**: Ein sitzplatzbasiertes Nutzungsfenster bei einem Abonnement-Plan. Diese Fenster werden über alle Modelle hinweg geteilt, daher stellt das Wechseln von Modellen mit `/model` den Zugriff nicht wieder her, obwohl es den Entwickler nach der modellspezifischen Nachricht „Sie haben Ihr Opus-Limit erreicht" weiterarbeiten lässt. Die Nachricht zeigt, wenn sich das Fenster zurückgesetzt, und der Entwickler kann `/usage-credits` ausführen, um Nutzung über die Erlaubnis hinaus anzufordern, wenn Sie [Nutzungsguthaben](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) aktiviert haben. Siehe [Nutzungslimit-Fehler](/docs/de/errors#youve-hit-your-session-limit).
* **Eine Kontext- oder Auto-Compact-Warnung**: Kein Nutzungslimit. Das Gespräch ist der maximalen Eingabegröße des Modells nahe gewachsen, und Claude Code fasst ältere Verlauf zusammen, um Platz freizugeben. Verweisen Sie den Entwickler auf [Token-Nutzung reduzieren](#reduce-token-usage).
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
* **Fügen Sie benutzerdefinierte Compaction-Anweisungen hinzu**: `/compact Focus on code samples and API usage` teilt Claude mit, was während der Zusammenfassung beibehalten werden soll.

Sie können das Compaction-Verhalten auch in Ihrer CLAUDE.md-Datei im Stammverzeichnis Ihres Projekts anpassen:

```markdown theme={null}
# Compact instructions

When you are using compact, please focus on test output and code changes
```

<h3 id="choose-the-right-model">
  Wählen Sie das richtige Modell
</h3>

Sonnet bewältigt die meisten Codierungsaufgaben gut und kostet weniger als Opus. Reservieren Sie Opus für komplexe architektonische Entscheidungen oder mehrstufiges Denken. Verwenden Sie `/model`, um Modelle während einer Sitzung zu wechseln, oder legen Sie einen Standard in `/config` fest. Für einfache Subagent-Aufgaben geben Sie `model: haiku` in Ihrer [Subagent-Konfiguration](/docs/de/sub-agents#choose-a-model) an.

<h3 id="reduce-mcp-server-overhead">
  Reduzieren Sie den MCP-Server-Overhead
</h3>

MCP-Tool-Definitionen werden [standardmäßig aufgeschoben](/docs/de/mcp#scale-with-mcp-tool-search), daher treten nur Tool-Namen in den Kontext ein, bis Claude ein bestimmtes Tool verwendet. Führen Sie `/context` aus, um zu sehen, was Platz verbraucht.

* **Bevorzugen Sie CLI-Tools, wenn verfügbar**: Tools wie `gh`, `aws`, `gcloud` und `sentry-cli` sind immer noch kontexteffektiver als MCP-Server, da sie keine Pro-Tool-Auflistung hinzufügen. Claude kann CLI-Befehle direkt ausführen.
* **Deaktivieren Sie ungenutzte Server**: Führen Sie `/mcp` aus, um konfigurierte Server anzuzeigen und alle zu deaktivieren, die Sie nicht aktiv verwenden.

<h3 id="install-code-intelligence-plugins-for-typed-languages">
  Installieren Sie Code-Intelligence-Plugins für typisierte Sprachen
</h3>

[Code-Intelligence-Plugins](/docs/de/discover-plugins#code-intelligence) geben Claude präzise Symbol-Navigation statt textbasierter Suche, wodurch unnötige Dateileser beim Erkunden unbekannten Codes reduziert werden. Ein einzelner „Gehe zu Definition"-Aufruf ersetzt, was sonst ein Grep gefolgt vom Lesen mehrerer Kandidatendateien sein könnte. Installierte Sprachserver melden auch Typfehler automatisch nach Bearbeitungen, sodass Claude Fehler erkennt, ohne einen Compiler auszuführen.

<h3 id="offload-processing-to-hooks-and-skills">
  Verlagern Sie die Verarbeitung auf Hooks und Skills
</h3>

Benutzerdefinierte [Hooks](/docs/de/hooks) können Daten vorverarbeiten, bevor Claude sie sieht. Anstatt dass Claude eine 10.000-Zeilen-Protokolldatei liest, um Fehler zu finden, kann ein Hook nach `ERROR` suchen und nur übereinstimmende Zeilen zurückgeben, wodurch der Kontext von Zehntausenden Token auf Hunderte reduziert wird.

Ein [Skill](/docs/de/skills) kann Claude Domänenwissen geben, sodass es nicht erkunden muss. Beispielsweise könnte ein „codebase-overview"-Skill die Architektur Ihres Projekts, wichtige Verzeichnisse und Namenskonventionen beschreiben. Wenn Claude den Skill aufruft, erhält es diesen Kontext sofort, anstatt Token zu verschwenden, um mehrere Dateien zu lesen, um die Struktur zu verstehen.

Beispielsweise filtert dieser PreToolUse-Hook die Testausgabe, um nur Fehler anzuzeigen:

<Tabs>
  <Tab title="settings.json">
    Fügen Sie dies zu Ihrer [settings.json](/docs/de/settings#settings-files) hinzu, um den Hook vor jedem Bash-Befehl auszuführen:

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
      echo "{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"allow\",\"updatedInput\":{\"command\":\"$filtered_cmd\"}}}"
    else
      echo "{}"
    fi
    ```
  </Tab>
</Tabs>

<h3 id="move-instructions-from-claude-md-to-skills">
  Verschieben Sie Anweisungen von CLAUDE.md zu Skills
</h3>

Ihre [CLAUDE.md](/docs/de/memory)-Datei wird beim Sitzungsstart in den Kontext geladen. Wenn sie detaillierte Anweisungen für spezifische Workflows enthält (wie PR-Reviews oder Datenbankmigrationen), sind diese Token vorhanden, auch wenn Sie nicht verwandte Arbeit erledigen. [Skills](/docs/de/skills) werden bei Bedarf nur geladen, wenn sie aufgerufen werden, daher hält das Verschieben spezialisierter Anweisungen in Skills Ihren Basis-Kontext kleiner. Streben Sie danach, CLAUDE.md unter 200 Zeilen zu halten, indem Sie nur das Wesentliche einbeziehen.

<h3 id="adjust-extended-thinking">
  Passen Sie das erweiterte Denken an
</h3>

Erweitertes Denken ist standardmäßig aktiviert, da es die Leistung bei komplexen Planungs- und Denkaufgaben erheblich verbessert. Thinking-Token werden als Output-Token abgerechnet, und das Standard-Budget kann je nach Modell Zehntausende Token pro Anfrage betragen. Für einfachere Aufgaben, bei denen tiefes Denken nicht erforderlich ist, können Sie Kosten reduzieren, indem Sie die [Anstrengungsstufe](/docs/de/model-config#adjust-effort-level) mit `/effort` senken oder in `/model`, Denken in `/config` deaktivieren oder auf Modellen mit einem [festen Thinking-Budget](/docs/de/model-config#adaptive-reasoning-and-fixed-thinking-budgets) das Budget durch Setzen der `MAX_THINKING_TOKENS` [Umgebungsvariable](/docs/de/env-vars) senken, beispielsweise `MAX_THINKING_TOKENS=8000`. Adaptive-Reasoning-Modelle ignorieren Budgets ungleich Null, daher verwenden Sie stattdessen Anstrengungsstufen. Das Deaktivieren von Thinking ist auf Fable 5 nicht verfügbar, das immer erweitertes Denken verwendet.

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

<h2 id="understanding-changes-in-claude-code-behavior">
  Verstehen Sie Änderungen im Claude Code-Verhalten
</h2>

Claude Code erhält regelmäßig Updates, die ändern können, wie Funktionen funktionieren, einschließlich Kostenberichterstattung. Führen Sie `claude --version` aus, um Ihre aktuelle Version zu überprüfen. Für spezifische Abrechnungsfragen kontaktieren Sie den Anthropic-Support über Ihr [Console-Konto](https://platform.claude.com/login).
