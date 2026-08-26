> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Null-Datenspeicherung

> Erfahren Sie mehr über Null-Datenspeicherung (ZDR) für Claude Code, verfügbar für qualifizierte Konten auf Claude for Enterprise, einschließlich Umfang, deaktivierter Funktionen und wie Sie die Aktivierung anfordern.

Null-Datenspeicherung (ZDR) für Claude Code ist für qualifizierte Konten auf Claude for Enterprise verfügbar. Wenn ZDR aktiviert ist, werden Eingabeaufforderungen und Modellreaktionen, die während Claude Code-Sitzungen generiert werden, in Echtzeit verarbeitet und nicht von Anthropic gespeichert, nachdem die Antwort zurückgegeben wurde, außer wenn dies erforderlich ist, um das Gesetz einzuhalten oder Missbrauch zu bekämpfen.

<Note>
  ZDR ist nicht im Standard-Plan von Claude for Enterprise enthalten und kann nicht in Ihren Admin-Einstellungen aktiviert werden. Es ist für qualifizierte Konten verfügbar und erfordert eine separate Aktivierung durch Anthropic. Wenn Ihre Organisation ZDR benötigt, [kontaktieren Sie den Vertrieb](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=zero_data_retention_request) oder Ihr Anthropic-Kontoteam, um die Berechtigung zu bestätigen.
</Note>

ZDR auf Claude for Enterprise gibt Unternehmenskunden die Möglichkeit, Claude Code mit Null-Datenspeicherung zu verwenden und auf Verwaltungsfunktionen zuzugreifen:

* Kostenkontrolle pro Benutzer
* [Analytics](/docs/de/analytics)-Dashboard
* [Serververwaltete Einstellungen](/docs/de/server-managed-settings)
* Audit-Protokolle

ZDR für Claude Code auf Claude for Enterprise gilt nur für die direkte Plattform von Anthropic. Für Claude-Bereitstellungen auf Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry beachten Sie die Datenspeicherungsrichtlinien dieser Plattformen.

<h2 id="zdr-scope">
  ZDR-Umfang
</h2>

ZDR deckt Claude Code-Inferenz auf Claude for Enterprise ab.

<Warning>
  ZDR wird auf Organisationsbasis aktiviert. Jede neue Organisation erfordert, dass ZDR separat von Ihrem Anthropic-Kontoteam aktiviert wird. ZDR wird nicht automatisch auf neue Organisationen angewendet, die unter demselben Konto erstellt werden. Wenden Sie sich an Ihr Kontoteam, um ZDR für neue Organisationen zu aktivieren.
</Warning>

<h3 id="what-zdr-covers">
  Was ZDR abdeckt
</h3>

ZDR deckt Modellrückschluss-Aufrufe ab, die über Claude Code auf Claude for Enterprise durchgeführt werden. Wenn Sie Claude Code in Ihrem Terminal verwenden, werden die Eingabeaufforderungen, die Sie senden, und die Antworten, die Claude generiert, nicht von Anthropic gespeichert. Dies gilt für jedes Modell, das ZDR-Organisationen zur Verfügung steht. Einige Modelle erfordern Datenspeicherung und sind unter ZDR nicht verfügbar; siehe [Modellverfügbarkeit unter ZDR](#model-availability-under-zdr).

<h3 id="what-zdr-does-not-cover">
  Was ZDR nicht abdeckt
</h3>

ZDR erstreckt sich nicht auf die folgenden Funktionen, auch nicht für Organisationen mit aktiviertem ZDR. Diese Funktionen folgen [Standard-Datenspeicherungsrichtlinien](/docs/de/data-usage#data-retention):

| Funktion                         | Details                                                                                                                                                                                                                                                                              |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Chat auf claude.ai               | Chat-Gespräche über die Claude for Enterprise-Weboberfläche werden nicht von ZDR abgedeckt.                                                                                                                                                                                          |
| Cowork                           | Cowork-Sitzungen werden nicht von ZDR abgedeckt.                                                                                                                                                                                                                                     |
| Claude Code Analytics            | Speichert keine Eingabeaufforderungen oder Modellreaktionen, erfasst aber Produktivitätsmetadaten wie Konto-E-Mails und Nutzungsstatistiken. Beitragskennzahlen sind für ZDR-Organisationen nicht verfügbar; das [Analytics-Dashboard](/docs/de/analytics) zeigt nur Nutzungsmetriken an. |
| Benutzer- und Platzverwaltung    | Verwaltungsdaten wie Konto-E-Mails und Platzzuweisungen werden nach Standardrichtlinien beibehalten.                                                                                                                                                                                 |
| Integrationen von Drittanbietern | Daten, die von Drittanbieter-Tools, MCP servers oder anderen externen Integrationen verarbeitet werden, werden nicht von ZDR abgedeckt. Überprüfen Sie die Datenverwaltungspraktiken dieser Dienste unabhängig.                                                                      |

<h2 id="features-disabled-under-zdr">
  Funktionen, die unter ZDR deaktiviert sind
</h2>

Wenn ZDR für eine Claude Code-Organisation auf Claude for Enterprise aktiviert ist, werden bestimmte Funktionen, die das Speichern von Eingabeaufforderungen oder Vervollständigungen erfordern, automatisch auf Backend-Ebene deaktiviert:

| Funktion                                                          | Grund                                                                                                          |
| ----------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| [Claude Code im Web](/docs/de/claude-code-on-the-web)                  | Erfordert serverseitige Speicherung des Gesprächsverlaufs.                                                     |
| [Cloud-Sitzungen](/docs/de/desktop#cloud-sessions) aus der Desktop-App | Erfordert persistente Sitzungsdaten, die Eingabeaufforderungen und Vervollständigungen enthalten.              |
| [Artifacts](/docs/de/artifacts)                                        | Erfordert das Speichern von veröffentlichtem Seiteninhalt auf von Anthropic betriebener Infrastruktur.         |
| Feedback-Übermittlung (`/feedback`)                               | Das Übermitteln von Feedback sendet Gesprächsdaten an Anthropic.                                               |
| [Remote Control](/docs/de/remote-control)                              | Speichert das Sitzungstranskript auf Anthropic-Servern, um das Gespräch über Geräte hinweg zu synchronisieren. |

Diese Funktionen werden im Backend blockiert, unabhängig von der clientseitigen Anzeige. Wenn Sie während des Starts eine deaktivierte Funktion im Claude Code-Terminal sehen, führt der Versuch, sie zu verwenden, zu einem Fehler, der angibt, dass die Richtlinien der Organisation diese Aktion nicht zulassen.

Zukünftige Funktionen können auch deaktiviert werden, wenn sie das Speichern von Eingabeaufforderungen oder Vervollständigungen erfordern.

<h3 id="model-availability-under-zdr">
  Modellverfügbarkeit unter ZDR
</h3>

Claude Fable 5 ist nicht für Organisationen mit aktivierter Null-Datenspeicherung verfügbar. Diese Modellklasse [erfordert Datenspeicherung](https://platform.claude.com/docs/en/manage-claude/api-and-data-retention#model-specific-data-retention-requirements), daher können Anfragen von ZDR-Organisationen nicht von ihr bedient werden. Das Modell fehlt entweder in der `/model`-Auswahl für ZDR-Organisationen oder wird als deaktiviert mit einem Hinweis angezeigt, dass das Deaktivieren von ZDR erforderlich ist, und der Server lehnt Anfragen dafür unabhängig von der Client-Konfiguration ab.

Andere Modelle bleiben unter ZDR verfügbar. Fable 5 ist nicht das Standardmodell, und der `best`-Alias, der sich zu Fable 5 auflöst, wo er verfügbar ist, wird zu Opus für Organisationen, wo er nicht verfügbar ist, einschließlich ZDR-Organisationen.

<h2 id="data-retention-for-policy-violations">
  Datenspeicherung bei Richtlinienverletzungen
</h2>

Auch wenn ZDR aktiviert ist, kann Anthropic Daten speichern, wenn dies gesetzlich erforderlich ist oder um Verstöße gegen die Nutzungsrichtlinie zu beheben. Wenn eine Sitzung wegen eines Richtlinienverstoßes gekennzeichnet wird, kann Anthropic die zugehörigen Ein- und Ausgaben bis zu 2 Jahre lang speichern, in Übereinstimmung mit Anthropics Standard-ZDR-Richtlinie.

<h2 id="request-zdr">
  ZDR anfordern
</h2>

Um ZDR für Claude Code auf Claude for Enterprise anzufordern, [kontaktieren Sie den Vertrieb](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=zero_data_retention_request) oder Ihr Anthropic-Kontoteam. Ihr Kontoteam reicht die Anfrage intern ein, und Anthropic überprüft und aktiviert ZDR in Ihrer Organisation, nachdem die Berechtigung bestätigt wurde. Alle Aktivierungsmaßnahmen werden protokolliert.

Wenn Sie derzeit ZDR für Claude Code über Pay-as-you-go-API-Schlüssel verwenden, können Sie zu Claude for Enterprise wechseln, um Zugriff auf Verwaltungsfunktionen zu erhalten und gleichzeitig ZDR für Claude Code beizubehalten. Wenden Sie sich an Ihr Kontoteam, um die Migration zu koordinieren.
