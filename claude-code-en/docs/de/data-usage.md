> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Datennutzung

> Erfahren Sie mehr über die Datennutzungsrichtlinien von Anthropic für Claude

<h2 id="data-policies">
  Datenrichtlinien
</h2>

<h3 id="data-training-policy">
  Datentrainingsrichtlinie
</h3>

**Verbrauchernutzer (Free-, Pro- und Max-Pläne)**:
Wir geben Ihnen die Möglichkeit, Ihre Daten zur Verbesserung zukünftiger Claude-Modelle zu nutzen. Wir trainieren neue Modelle mit Daten aus Free-, Pro- und Max-Konten, wenn diese Einstellung aktiviert ist (auch wenn Sie Claude Code aus diesen Konten verwenden).

**Kommerzielle Nutzer**: (Team- und Enterprise-Pläne, API, Plattformen von Drittanbietern und Claude Gov) behalten bestehende Richtlinien bei: Anthropic trainiert keine generativen Modelle mit Code oder Eingabeaufforderungen, die unter kommerziellen Bedingungen an Claude Code gesendet werden, es sei denn, der Kunde hat sich dafür entschieden, seine Daten für die Modellverbesserung bereitzustellen (zum Beispiel das [Developer Partner Program](https://support.claude.com/de/articles/11174108-about-the-development-partner-program)).

<h3 id="development-partner-program">
  Development Partner Program
</h3>

Wenn Sie sich explizit dafür entscheiden, uns Materialien zum Trainieren bereitzustellen, z. B. über das [Development Partner Program](https://support.claude.com/de/articles/11174108-about-the-development-partner-program), können wir diese Materialien zum Trainieren unserer Modelle verwenden. Ein Organisationsadministrator kann sich explizit für das Development Partner Program für seine Organisation anmelden. Beachten Sie, dass dieses Programm nur für die Anthropic-API von Erstanbietern verfügbar ist und nicht für Amazon Bedrock oder Google Cloud's Agent Platform-Nutzer.

<h3 id="feedback-using-the-/feedback-command">
  Feedback mit dem `/feedback`-Befehl
</h3>

Wenn Sie uns Feedback zu Claude Code mit dem `/feedback`-Befehl senden, können wir Ihr Feedback zur Verbesserung unserer Produkte und Dienstleistungen nutzen. Transkripte, die über `/feedback` freigegeben werden, werden 5 Jahre lang aufbewahrt.

<h3 id="session-quality-surveys">
  Sitzungsqualitätsumfragen
</h3>

Wenn Sie in Claude Code die Eingabeaufforderung „Wie macht Claude das in dieser Sitzung?" sehen, wird bei der Beantwortung dieser Umfrage (einschließlich der Auswahl von „Verwerfen") nur Ihre Bewertung aufgezeichnet. Wir erfassen oder speichern keine Gesprächstranskripte, Eingaben, Ausgaben oder andere Sitzungsdaten als Teil der Bewertungsaufforderung selbst. Im Gegensatz zu Daumen-hoch/runter-Feedback oder `/feedback`-Berichten ist diese Sitzungsqualitätsumfrage eine einfache Produktzufriedenheitsmetrik.

Nach der Bewertungsaufforderung sehen Sie möglicherweise eine separate Folgefrage: „Kann Anthropic Ihr Sitzungstranskript ansehen, um uns bei der Verbesserung von Claude Code zu helfen?". Dies ist ein optionaler zweiter Schritt, der sich von der Bewertung unterscheidet:

* **Ja**: lädt Ihr Gesprächstranskript, alle Subagenten-Transkripte und die Raw-Sitzungsprotokoll-Datei von der Festplatte zu Anthropic hoch. Bekannte API-Schlüssel- und Token-Muster werden vor dem Hochladen redigiert. Quellcode, Dateiinhalte und andere Gesprächsinhalte werden unverändert hochgeladen. Freigegebene Transkripte werden bis zu 6 Monate lang aufbewahrt. Auf Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry und angemeldeten [Claude Apps Gateway](/docs/de/claude-apps-gateway)-Sitzungen schreibt „Ja" dieselbe Nutzlast stattdessen in ein lokales Archiv unter `~/.claude/feedback-bundles/`; nichts verlässt Ihren Computer, bis Sie diese Datei weiterleiten.
* **Nein**: lehnt ab, ohne etwas zu senden
* **Nicht erneut fragen**: lehnt ab und verhindert, dass diese Folgefrage in zukünftigen Sitzungen angezeigt wird

Nichts wird hochgeladen, es sei denn, Sie wählen explizit **Ja**. Organisationen mit [Zero Data Retention](/docs/de/zero-data-retention) oder bei denen Produktfeedback durch Organisationsrichtlinie deaktiviert ist, oder bei denen `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` gesetzt ist, sehen diese Folgefrage nie. Ihre Antworten auf diese Umfrage, einschließlich Sitzungstranskripte, die nach der Bewertungsaufforderung eingereicht werden, beeinflussen nicht Ihre Datentrainingseinstellungen und können nicht zum Trainieren unserer KI-Modelle verwendet werden.

Um diese Umfragen zu deaktivieren, setzen Sie `CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1`. Die Umfrage wird auch deaktiviert, wenn `DISABLE_TELEMETRY`, `DO_NOT_TRACK` oder `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` gesetzt ist. Organisationen, die nicht wesentlichen Datenverkehr blockieren, aber Umfrageantworten über ihren eigenen [OpenTelemetry-Collector](/docs/de/monitoring-usage) erfassen, können die Umfrage wieder aktivieren, indem sie `CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL=1` setzen. Die Umfrage protokolliert dann Bewertungen nur zum konfigurierten Collector. Die Transkript-Freigabe-Folgefrage und der gesamte andere Anthropic-gebundene Feedback-Datenverkehr bleiben deaktiviert. Um die Häufigkeit zu steuern, anstatt zu deaktivieren, setzen Sie [`feedbackSurveyRate`](/docs/de/settings#available-settings) in Ihrer Einstellungsdatei auf eine Wahrscheinlichkeit zwischen `0` und `1`.

<h3 id="data-retention">
  Datenspeicherung
</h3>

Anthropic speichert Claude Code-Daten basierend auf Ihrem Kontotyp und Ihren Einstellungen.

**Verbrauchernutzer (Free-, Pro- und Max-Pläne)**:

* Nutzer, die die Datennutzung für die Modellverbesserung zulassen: 5-jährige Aufbewahrungsfrist zur Unterstützung der Modellentwicklung und Sicherheitsverbesserungen
* Nutzer, die die Datennutzung für die Modellverbesserung nicht zulassen: 30-tägige Aufbewahrungsfrist
* Datenschutzeinstellungen können jederzeit unter [claude.ai/settings/data-privacy-controls](https://claude.ai/settings/data-privacy-controls) geändert werden.

**Kommerzielle Nutzer (Team, Enterprise und API)**:

* Standard: 30-tägige Aufbewahrungsfrist
* [Zero Data Retention](/docs/de/zero-data-retention): verfügbar für Claude Code auf Claude for Enterprise. ZDR ist nicht in dem Standard-Enterprise-Plan enthalten; es wird auf Organisationsbasis von Ihrem Account-Team aktiviert, nachdem die Berechtigung bestätigt wurde
* Lokales Caching: Claude Code-Clients speichern Sitzungstranskripte lokal im Klartext unter `~/.claude/projects/` für standardmäßig 30 Tage, um die Sitzungswiederaufnahme zu ermöglichen. Passen Sie den Zeitraum mit `cleanupPeriodDays` an. Siehe [Anwendungsdaten](/docs/de/claude-directory#application-data) für das, was gespeichert wird und wie man es löscht.

Sie können einzelne Claude Code-Websitzungen jederzeit löschen. Das Löschen einer Sitzung entfernt die Ereignisdaten der Sitzung dauerhaft. Anweisungen zum Löschen von Sitzungen finden Sie unter [Sitzungen löschen](/docs/de/claude-code-on-the-web#delete-sessions).

Erfahren Sie mehr über Datenspeicherungspraktiken in unserem [Privacy Center](https://privacy.anthropic.com/).

Vollständige Details finden Sie in unseren [Commercial Terms of Service](https://www.anthropic.com/legal/commercial-terms) (für Team-, Enterprise- und API-Nutzer) oder [Consumer Terms](https://www.anthropic.com/legal/consumer-terms) (für Free-, Pro- und Max-Nutzer) und [Privacy Policy](https://www.anthropic.com/legal/privacy).

<h2 id="data-access">
  Datenzugriff
</h2>

Für alle Nutzer von Erstanbietern können Sie mehr über die protokollierten Daten für [lokales Claude Code](#local-claude-code-data-flow-and-dependencies) und [Remote Claude Code](#cloud-execution-data-flow-and-dependencies) erfahren. [Remote Control](/docs/de/remote-control)-Sitzungen folgen dem lokalen Datenfluss, da die gesamte Ausführung auf Ihrem Computer stattfindet; während der Verbindung wird das Sitzungstranskript auch auf Anthropic-Servern gespeichert, um die Konversation über Geräte hinweg zu synchronisieren, wie in [Verbindung und Sicherheit](/docs/de/remote-control#connection-and-security) beschrieben. Beachten Sie, dass Claude bei Remote Claude Code auf das Repository zugreift, in dem Sie Ihre Claude Code-Sitzung starten. Claude greift nicht auf Repositories zu, die Sie verbunden haben, aber in denen Sie keine Sitzung gestartet haben.

<h2 id="local-claude-code-data-flow-and-dependencies">
  Local Claude Code: Datenfluss und Abhängigkeiten
</h2>

Das folgende Diagramm zeigt, wie Claude Code während der Installation und des normalen Betriebs eine Verbindung zu externen Diensten herstellt. Durchgehende Linien zeigen erforderliche Verbindungen an, während gestrichelte Linien optionale oder vom Benutzer initiierte Datenflüsse darstellen.

<img src="https://mintcdn.com/claude-code/YR4DRZyI3CdsXkiT/images/claude-code-data-flow.svg?fit=max&auto=format&n=YR4DRZyI3CdsXkiT&q=85&s=2846ea92cfc2297b8620c31c82b482ad" alt="Diagramm, das die externen Verbindungen von Claude Code zeigt: Installation/Update verbindet sich mit dem Verteilungsserver, und Benutzeranfragen verbinden sich mit Anthropic-Konsole-Authentifizierung und öffentlicher API, mit optionalen Telemetrie-Flüssen, die Metriken und Fehlerberichte an Anthropic und Drittanbieterdienste übertragen. Feedback, das mit /feedback gesendet wird, geht an Google Cloud Storage und erstellt optional ein GitHub-Problem" width="720" height="520" data-path="images/claude-code-data-flow.svg" />

Claude Code wird lokal ausgeführt. Um mit dem LLM zu interagieren, sendet Claude Code Daten über das Netzwerk. Diese Daten umfassen alle Benutzereingabeaufforderungen und Modellausgaben, verschlüsselt während der Übertragung über TLS 1.2+. Claude Code ist mit den meisten gängigen VPNs und LLM-Proxys kompatibel.

Die Verschlüsselung im Ruhezustand hängt von Ihrem Modelldienstanbieter ab:

| Anbieter                      | Verschlüsselung im Ruhezustand                                                                                                                         |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Anthropic API                 | Verschlüsselung auf Infrastrukturebene (AES-256). Aktivieren Sie [Zero Data Retention](/docs/de/zero-data-retention) für keine serverseitige Persistierung. |
| Amazon Bedrock                | AES-256 mit von AWS verwalteten Schlüsseln. Von Kunden verwaltete Schlüssel verfügbar über AWS KMS.                                                    |
| Google Cloud's Agent Platform | Von Google verwaltete Verschlüsselungsschlüssel. CMEK verfügbar.                                                                                       |
| Microsoft Foundry             | Anfragen werden an die Anthropic-Infrastruktur mit AES-256-Festplattenverschlüsselung weitergeleitet.                                                  |

Claude Code basiert auf den APIs von Anthropic. Weitere Informationen zu den Sicherheitskontrollen unserer API, einschließlich unserer API-Protokollierungsverfahren, finden Sie in den Compliance-Artefakten im [Anthropic Trust Center](https://trust.anthropic.com).

<h3 id="cloud-execution-data-flow-and-dependencies">
  Cloud-Ausführung: Datenfluss und Abhängigkeiten
</h3>

Bei Verwendung von [Claude Code on the web](/docs/de/claude-code-on-the-web) werden Sitzungen in von Anthropic verwalteten virtuellen Maschinen statt lokal ausgeführt. In Cloud-Umgebungen:

* **Code- und Datenspeicherung:** Ihr Repository wird auf eine isolierte VM geklont. Code und Sitzungsdaten unterliegen den Aufbewahrungs- und Nutzungsrichtlinien für Ihren Kontotyp (siehe Abschnitt Datenspeicherung oben)
* **Anmeldedaten:** Die GitHub-Authentifizierung wird über einen sicheren Proxy durchgeführt; Ihre GitHub-Anmeldedaten gelangen niemals in die Sandbox
* **Netzwerkverkehr:** Der gesamte ausgehende Datenverkehr wird über einen Sicherheitsproxy für Audit-Protokollierung und Missbrauchsprävention geleitet
* **Sitzungsdaten:** Eingabeaufforderungen, Codeänderungen und Ausgaben folgen den gleichen Datenrichtlinien wie die lokale Claude Code-Nutzung

Sicherheitsdetails zur Cloud-Ausführung finden Sie unter [Security](/docs/de/security#cloud-execution-security).

<h2 id="telemetry-services">
  Telemetrie-Dienste
</h2>

Claude Code sendet zwei Arten von operativen Telemetriedaten: Nutzungsmetriken und Fehlerberichte. Sie können jede einzeln mit den folgenden Umgebungsvariablen deaktivieren oder den gesamten nicht wesentlichen Datenverkehr auf einmal deaktivieren, indem Sie `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` setzen.

**Metriken**: Latenz, Zuverlässigkeit und Nutzungsmuster, die an Anthropic und an Drittanbieter-Protokollierungsinfrastruktur über TLS gesendet werden. Metriken enthalten niemals Ihren Code, Ihre Eingabeaufforderungen oder Dateipfade. Setzen Sie `DISABLE_TELEMETRY=1`, um sich abzumelden.

**Fehlerberichte**: Fehlermeldungen und Stack-Traces aus den eigenen Interna von Claude Code, die an einen Drittanbieter-Fehler-Tracking-Dienst über TLS gesendet werden. Claude Code redigiert bekannte Muster von Geheimnissen, Dateipfaden, E-Mail-Adressen und anderen persönlichen Informationen, bevor etwas Ihren Computer verlässt. Setzen Sie `DISABLE_ERROR_REPORTING=1`, um sich abzumelden.

Die Fehlerberichterstattung ist nur aktiviert, wenn alle diese Bedingungen erfüllt sind:

* Sie melden sich mit einem Claude Pro- oder Max-Abonnement an
* Sie führen Claude Code v2.1.198 oder später aus
* Sie verbinden sich direkt mit der Claude API
* Ihre Organisation hat keine Vereinbarung zur Nulldatenspeicherung oder HIPAA

Wenn Sie den `/feedback`-Befehl ausführen, wird eine Kopie Ihres Gesprächsverlaufs einschließlich Code an Anthropic gesendet. Vor dem Absenden wählen Sie, wie viel Verlauf Sie einbeziehen möchten: nur die aktuelle Sitzung, was die Standardeinstellung ist, oder auch andere Sitzungen aus demselben Projekt der letzten 24 Stunden oder 7 Tage. Die Daten werden während der Übertragung über TLS verschlüsselt und in Google Cloud Storage gespeichert, das gespeicherte Daten im Ruhezustand standardmäßig verschlüsselt. Optional wird ein GitHub-Problem im öffentlichen Repository erstellt. Um sich abzumelden, setzen Sie die Umgebungsvariable `DISABLE_FEEDBACK_COMMAND` auf `1`.

Wenn Sie einen Drittanbieter wie Amazon Bedrock oder Google Cloud's Agent Platform verwenden oder keine Anthropic-Anmeldedaten konfiguriert haben, schreibt `/feedback` den Bericht stattdessen in ein lokales Archiv unter `~/.claude/feedback-bundles/`, anstatt ihn an Anthropic zu senden. Bekannte API-Schlüssel- und Token-Muster werden vor dem Schreiben des Archivs redigiert. Nichts verlässt Ihren Computer, bis Sie diese Datei an Ihren Anthropic-Kontorepräsentanten senden oder sie an eine Supportanfrage anhängen.

<h2 id="default-behaviors-by-api-provider">
  Standardverhalten nach API-Anbieter
</h2>

Standardmäßig sind Fehlerberichterstattung, Telemetrie und Bug-Berichterstattung deaktiviert, wenn Sie Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry oder Claude Platform auf AWS verwenden. Sitzungsqualitätsumfragen und die WebFetch-Domänensicherheitsprüfung sind Ausnahmen und werden unabhängig vom Anbieter ausgeführt. Auf einer angemeldeten [Claude Apps Gateway](/docs/de/claude-apps-gateway)-Sitzung sind Nutzungsanalysen, Fehlerberichterstattung und Umfragebewertungen an Anthropic durch die Gateway-Anmeldedaten selbst deaktiviert, ohne dass eine Einstellung zum erneuten Aktivieren vorhanden ist. Sie können sich auf einmal von all dem nicht wesentlichen Datenverkehr, einschließlich Umfragen, abmelden, indem Sie `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` setzen. Diese Variable beeinträchtigt die WebFetch-Prüfung nicht, die ihre eigene Abmeldeoption hat. Hier sind die vollständigen Standardverhalten:

| Dienst                                 | Claude API                                                                                                                      | Google Cloud's Agent Platform API                                                                              | Amazon Bedrock API                                                                                             | Microsoft Foundry API                                                                                          | Claude Platform auf AWS                                                                                        |
| -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| **Metriken**                           | Standardmäßig aktiviert.<br />`DISABLE_TELEMETRY=1` zum Deaktivieren.                                                           | Standardmäßig deaktiviert.<br />`CLAUDE_CODE_USE_VERTEX` muss 1 sein.                                          | Standardmäßig deaktiviert.<br />`CLAUDE_CODE_USE_BEDROCK` muss 1 sein.                                         | Standardmäßig deaktiviert.<br />`CLAUDE_CODE_USE_FOUNDRY` muss 1 sein.                                         | Standardmäßig deaktiviert.<br />`CLAUDE_CODE_USE_ANTHROPIC_AWS` muss 1 sein.                                   |
| **Fehlerberichte**                     | Aktiviert für Pro- und Max-Anmeldungen auf v2.1.198+, ansonsten deaktiviert.<br />`DISABLE_ERROR_REPORTING=1` zum Deaktivieren. | Standardmäßig deaktiviert.<br />`CLAUDE_CODE_USE_VERTEX` muss 1 sein.                                          | Standardmäßig deaktiviert.<br />`CLAUDE_CODE_USE_BEDROCK` muss 1 sein.                                         | Standardmäßig deaktiviert.<br />`CLAUDE_CODE_USE_FOUNDRY` muss 1 sein.                                         | Standardmäßig deaktiviert.<br />`CLAUDE_CODE_USE_ANTHROPIC_AWS` muss 1 sein.                                   |
| **Claude API (`/feedback`-Berichte)**  | Standardmäßig aktiviert.<br />`DISABLE_FEEDBACK_COMMAND=1` zum Deaktivieren.                                                    | Standardmäßig deaktiviert.<br />`CLAUDE_CODE_USE_VERTEX` muss 1 sein.                                          | Standardmäßig deaktiviert.<br />`CLAUDE_CODE_USE_BEDROCK` muss 1 sein.                                         | Standardmäßig deaktiviert.<br />`CLAUDE_CODE_USE_FOUNDRY` muss 1 sein.                                         | Standardmäßig deaktiviert.<br />`CLAUDE_CODE_USE_ANTHROPIC_AWS` muss 1 sein.                                   |
| **Sitzungsqualitätsumfragen**          | Standardmäßig aktiviert.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` zum Deaktivieren.                                         | Standardmäßig aktiviert.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` zum Deaktivieren.                        | Standardmäßig aktiviert.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` zum Deaktivieren.                        | Standardmäßig aktiviert.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` zum Deaktivieren.                        | Standardmäßig aktiviert.<br />`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY=1` zum Deaktivieren.                        |
| **WebFetch-Domänensicherheitsprüfung** | Standardmäßig aktiviert.<br />`skipWebFetchPreflight: true` in [Einstellungen](/docs/de/settings) zum Deaktivieren.                  | Standardmäßig aktiviert.<br />`skipWebFetchPreflight: true` in [Einstellungen](/docs/de/settings) zum Deaktivieren. | Standardmäßig aktiviert.<br />`skipWebFetchPreflight: true` in [Einstellungen](/docs/de/settings) zum Deaktivieren. | Standardmäßig aktiviert.<br />`skipWebFetchPreflight: true` in [Einstellungen](/docs/de/settings) zum Deaktivieren. | Standardmäßig aktiviert.<br />`skipWebFetchPreflight: true` in [Einstellungen](/docs/de/settings) zum Deaktivieren. |

Alle Umgebungsvariablen können in `settings.json` eingecheckt werden (siehe [Einstellungsreferenz](/docs/de/settings)).

Ab v2.1.126 werden Metriken standardmäßig aktiviert für Google Cloud's Agent Platform, Amazon Bedrock und Microsoft Foundry, wenn eine Host-Plattform `CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST` setzt, und folgen der Standard-Abmeldeoption `DISABLE_TELEMETRY`. Fehlerberichterstattung und `/feedback`-Berichte bleiben standardmäßig auf diesen Anbietern deaktiviert.

<h3 id="webfetch-domain-safety-check">
  WebFetch-Domänensicherheitsprüfung
</h3>

Bevor eine URL abgerufen wird, sendet das WebFetch-Tool den angeforderten Hostnamen an `api.anthropic.com`, um ihn gegen eine von Anthropic verwaltete Sicherheitsblockierungsliste zu überprüfen. Es wird nur der Hostname gesendet, nicht die vollständige URL, der Pfad oder der Seiteninhalt. Ergebnisse werden pro Hostname für fünf Minuten zwischengespeichert.

Diese Prüfung wird unabhängig davon ausgeführt, welchen Modellanbieter Sie verwenden, und wird nicht durch `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` beeinträchtigt. Wenn Ihr Netzwerk `api.anthropic.com` blockiert, schlagen WebFetch-Anfragen fehl, bis Sie entweder die Domäne auf die Whitelist setzen oder `skipWebFetchPreflight: true` in [Einstellungen](/docs/de/settings) setzen. Das Deaktivieren der Prüfung bedeutet, dass WebFetch versucht, jede URL abzurufen, ohne die Blockierungsliste zu konsultieren. Kombinieren Sie dies daher mit [`WebFetch`-Berechtigungsregeln](/docs/de/permissions#webfetch), wenn Sie einschränken müssen, auf welche Domänen Claude zugreifen kann.
