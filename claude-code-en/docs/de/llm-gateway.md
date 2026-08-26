> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Andere LLM-Gateways

> Leiten Sie Claude Code über ein LLM-Gateway weiter, das Ihre Organisation bereits betreibt. Behandelt die Verbindung von Claude Code mit einem Gateway, die Bereitstellung für Ihre Organisation und was Claude Code an ein Gateway sendet.

Dieser Abschnitt behandelt die Verwendung eines Gateway-Produkts, das Ihre Organisation bereits betreibt, anstelle von [Claude Apps Gateway](/docs/de/claude-apps-gateway). Informationen dazu, was ein Gateway ist, wie es zwischen Claude Code und Ihrem Anbieter sitzt und wie Sie zwischen Claude Apps Gateway und einem anderen Produkt wählen, finden Sie in der [Gateway-Übersicht](/docs/de/gateways).

<Note>
  * Wenn Sie ein Entwickler sind, der sich mit einem vorhandenen Gateway verbindet: [Verbinden Sie Claude Code mit Ihrem Gateway](/docs/de/llm-gateway-connect)
  * Wenn Sie ein Administrator sind, der ein Gateway für Ihre Organisation bereitstellt: [Stellen Sie ein Gateway bereit und verteilen Sie es](/docs/de/llm-gateway-rollout)
  * Wenn Sie ein Gateway-Produkt konfigurieren: die [Gateway-Protokoll-Referenz](/docs/de/llm-gateway-protocol)
</Note>

Jedes Gateway, das ein [unterstütztes API-Format](/docs/de/llm-gateway-protocol#api-formats) bereitstellt, funktioniert. Anthropic befürwortet, wartet oder prüft keine Gateway-Produkte von Drittanbietern und unterstützt nicht das Routing von Claude Code zu Nicht-Claude-Modellen über ein Gateway. Stellen Sie das Gateway nach seiner eigenen Dokumentation bereit und schließen Sie dann die Claude Code-Seite mit den [Bereitstellungsschritten unten](#roll-out-a-gateway) ab.

<h2 id="what-a-gateway-provides">
  Was ein Gateway bietet
</h2>

Ein Gateway gibt Ihrer Organisation einen Ort zur Verwaltung von:

* **Anmeldedaten**: Der Anbieter-Schlüssel bleibt serverseitig; Entwickler halten stattdessen Gateway-Anmeldedaten
* **Nutzungsverfolgung**: Attributieren Sie die Nutzung nach Entwickler oder Team, unabhängig davon, welcher Anbieter die Anfrage bedient
* **Kostenkontrollen**: Erzwingen Sie Budgets und Ratenlimits an einem Ort
* **Audit-Protokollierung**: Protokollieren Sie jede Modellanfrage zur Compliance
* **Anbieter-Wechsel**: Ändern Sie den Anbieter in der Gateway-Konfiguration, ohne Entwicklermaschinen zu berühren

Alle diese außer dem Anbieter-Wechsel gelten, ob der Upstream die API von Anthropic oder ein [Cloud-Anbieter](/docs/de/third-party-integrations) ist. Der Anbieter-Wechsel ohne Neukonfiguration von Entwicklermaschinen hängt auch davon ab, dass das Gateway einen einzelnen [Anthropic-Format-Endpunkt](/docs/de/llm-gateway-protocol#api-formats) unabhängig vom Upstream bereitstellt; ein Gateway, das das eigene Format eines Anbieters bereitstellt, bindet die Client-Konfiguration an diesen Anbieter.

Der Kompromiss besteht darin, dass das Gateway zu einer Infrastruktur wird, die Ihre Organisation betreibt. Claude Code fügt mit jeder Version Funktionen hinzu, und ein Gateway, das diese nicht weiterleitet, bricht die entsprechenden Funktionen, daher muss das Gateway-Produkt aktualisiert werden, wenn sich Claude Code entwickelt. Die [Gateway-Protokoll-Referenz](/docs/de/llm-gateway-protocol) behandelt, was weitergeleitet werden soll.

<h2 id="roll-out-a-gateway">
  Ein Gateway bereitstellen
</h2>

Wenn Sie bereit sind, ein LLM-Gateway für Ihre Organisation bereitzustellen, ist die Abfolge gleich, welches Gateway-Produkt Sie auch wählen:

1. Stellen Sie das Gateway bereit und geben Sie ihm Ihre Anbieter-Anmeldedaten, damit es die Anfragen authentifizieren kann, die es weiterleitet.
2. Geben Sie jedem Entwickler ein Gateway-Anmeldedaten aus, damit die Nutzung dem Entwickler zugeordnet wird und das Offboarding ein Anmeldedaten widerruft.
3. Verteilen Sie die Konfiguration über eine [verwaltete Einstellungsdatei](/docs/de/settings#settings-files) und Ihre Secrets-Tools, damit jede Maschine die Basis-URL und ein Anmeldedaten erhält. Wenn beide verteilt werden, konfigurieren Entwickler nichts. Wenn Sie keine Einstellungsverteilung haben, folgen Entwickler der [Verbindungsseite](/docs/de/llm-gateway-connect), um die Variablen selbst zu setzen.
4. Lassen Sie jeden Entwickler [die Konfiguration in Claude Code überprüfen](/docs/de/llm-gateway-connect#check-for-an-existing-configuration), damit Verteilungsprobleme auftauchen, bevor sie vom Gateway abhängig sind.

[Ein LLM-Gateway für Ihre Organisation bereitstellen](/docs/de/llm-gateway-rollout) führt jeden Schritt durch und zeigt die Konfigurationsdateien, die bei jedem verteilt werden sollen. Das Gateway ist ein Teil der Organisationseinrichtung; für Richtliniendurchsetzung, Nutzungssichtbarkeit und Datenbehandlungsentscheidungen siehe [Claude Code für Ihre Organisation einrichten](/docs/de/admin-setup).

<h2 id="subscriptions-and-gateways">
  Abonnements und Gateways
</h2>

Während eine [Gateway-Anmeldedaten-Variable](/docs/de/llm-gateway-connect#set-the-credential-variable) oder `apiKeyHelper` aktiv ist, wird das claude.ai-Abonnement eines Entwicklers nicht verwendet: Das Anmeldedaten ersetzt die Abonnement-Anmeldung für diese Sitzung, und die Nutzungslimits des Abonnements gelten nicht. Dieser Verkehr wird pro Token dem Besitzer des Anmeldedaten, das das Gateway weiterleitet, wie Ihr Anthropic Console-Konto der Organisation oder Ihr Bedrock-, Agent Platform- oder Foundry-Konto, wenn das Gateway dorthin leitet, in Rechnung gestellt.

[`ANTHROPIC_BASE_URL`](/docs/de/llm-gateway-connect#set-the-base-url-and-credential) ist die Variable, die Claude Code auf das Gateway verweist. Das Setzen nur dieser Variable ohne Gateway-Anmeldedaten ersetzt das Abonnement nicht. Anfragen werden immer noch über das Gateway geleitet, aber eine gespeicherte claude.ai-Anmeldung bleibt das aktive Anmeldedaten, daher gelten seine Nutzungslimits und Abrechnung. Gateways, die diesen Verkehr an Anthropic weitergeben, müssen die OAuth-Fähigkeit in `anthropic-beta` weiterleiten; siehe die [Request-Header-Referenz](/docs/de/llm-gateway-protocol#request-headers).

<h2 id="related-pages">
  Verwandte Seiten
</h2>

* [Gateway-Übersicht](/docs/de/gateways): wie ein Gateway funktioniert und wie Sie zwischen Claude Apps Gateway und einem anderen Produkt wählen
* [Claude Apps Gateway](/docs/de/claude-apps-gateway): Anthropics selbstgehostetes Gateway mit SSO-Anmeldung und OTLP-Telemetrie
* [Verbinden Sie Claude Code mit einem LLM-Gateway](/docs/de/llm-gateway-connect): Setzen Sie die Basis-URL und das Anmeldedaten auf Ihrer eigenen Maschine, mit Pro-Surface-Konfiguration und einer Fehlerbehebungstabelle
* [Ein LLM-Gateway für Ihre Organisation bereitstellen](/docs/de/llm-gateway-rollout): Die Admin-Checkliste für die Bereitstellung eines Gateways, die Ausstellung von Entwickler-Anmeldedaten und die Verteilung verwalteter Einstellungen
* [Gateway-Protokoll-Referenz](/docs/de/llm-gateway-protocol): was Claude Code an ein Gateway sendet, für Operatoren, die eines konfigurieren, mit Endpunkten, Headern zum Weiterleiten und Feature-Pass-Through
