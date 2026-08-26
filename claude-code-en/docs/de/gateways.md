> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code über ein Gateway ausführen

> Leiten Sie Claude Code über ein selbstgehostetes Gateway für zentralisierte Anmeldedaten, Nutzungsverfolgung und Kostenkontrolle weiter. Behandelt die Architektur, Anthropics Claude-Apps-Gateway und die Verwendung anderer Gateway-Produkte.

Ein Gateway ist ein Proxy, den Ihre Organisation zwischen Claude Code und einem Modellanbietern betreibt. Claude Code sendet API-Traffic an das Gateway statt direkt an den Anbieter, und das Gateway leitet ihn mit einem Anmeldedatum weiter, das Ihre Organisation hält. Entwickler authentifizieren sich beim Gateway statt Anbieter-Anmeldedaten zu halten, sodass Authentifizierung, Nutzungsverfolgung, Budgets und Audit-Protokollierung an einem Ort stattfinden, den Sie kontrollieren.

Claude Code enthält ein selbstgehostetes Gateway, [Claude-Apps-Gateway](/docs/de/claude-apps-gateway), in der `claude`-Binärdatei, sodass Sie kein separates Gateway-Produkt einführen müssen, um eines auszuführen. Wenn Ihre Organisation bereits ein [LLM-Gateway](/docs/de/llm-gateway) betreibt, funktioniert Claude Code auch damit.

Diese Seite behandelt:

* [Wie ein Gateway zwischen Claude Code und Ihrem Anbieter sitzt](#how-a-gateway-works)
* [Wahl zwischen Claude-Apps-Gateway und einem Gateway, das Sie bereits betreiben](#choose-a-gateway)
* [Wie Gateways mit claude.ai-Abonnements interagieren](#subscriptions-and-gateways)
* [Was separat vom Gateway konfiguriert wird](#configure-separately-from-the-gateway)

<h2 id="how-a-gateway-works">
  Wie ein Gateway funktioniert
</h2>

Jedes Claude Code eines Entwicklers wird auf die Adresse des Gateways verwiesen und authentifiziert sich mit einem vom Gateway ausgegebenen Anmeldedatum.

Das Gateway authentifiziert den Entwickler, wendet alle Zugriffs- und Budgetregeln an, die Sie konfigurieren, und leitet die Anfrage mit dem Anmeldedatum Ihrer Organisation an Ihren Anbieter weiter. Der Anbieter kann die API von Anthropic oder ein [Cloud-Anbieter](/docs/de/third-party-integrations) wie Amazon Bedrock, Google Clouds Agent Platform oder Microsoft Foundry sein; die Konfiguration des Gateways entscheidet. Mit Claude-Apps-Gateway oder einem anderen Gateway, das einen einzelnen Anthropic-Format-Endpunkt verfügbar macht, erfordert ein Anbieterwechsel keine Änderungen an Entwicklermaschinen.

<Frame>
  <img src="https://mintcdn.com/claude-code/-uq-4JE0W_JO5Er5/images/llm-gateway-flow.svg?fit=max&auto=format&n=-uq-4JE0W_JO5Er5&q=85&s=1c1a8dcc0cfcc3a58652cc8e28cd3e20" alt="Diagramm, das zeigt, wie Claude Code über ein Gateway weitergeleitet wird. In einer Zone für Entwicklermaschinen senden die Claude Code CLI und die VS Code-Erweiterung Anfragen an die Gateway-Adresse mit einem Pro-Entwickler-Anmeldedatum. In einer Zone mit der Bezeichnung Ihre Infrastruktur verarbeitet das Gateway Authentifizierung, Nutzungsverfolgung, Budgets und Routing und leitet Anfragen mit dem Anmeldedatum Ihrer Organisation weiter. In einer Zone für Modellanbietern führt ein durchgehender Pfeil zum konfigurierten Anbieter, dargestellt als Anthropic API, und gestrichelte Pfeile führen zu anderen Anbieteroptionen, dargestellt mit Amazon Bedrock, Google Cloud und Microsoft Foundry als Beispiele." width="780" height="322" data-path="images/llm-gateway-flow.svg" />
</Frame>

Zwei Arten von Anmeldedaten sind beteiligt:

* **Entwickler-Anmeldedatum**: Jeder Entwickler hält sein eigenes, das vom Gateway ausgestellt wird. Es authentifiziert ihn beim Gateway und identifiziert ihn in der Nutzungsverfolgung
* **Anbieter-Anmeldedatum**: Das Gateway hält ein Anmeldedatum für Ihr Anbieterkonto, das von allem weitergeleitetem Traffic gemeinsam genutzt wird

<h2 id="choose-a-gateway">
  Wählen Sie ein Gateway
</h2>

Claude Code funktioniert mit Anthropics eigenem Gateway oder mit einem Gateway, das Ihre Organisation bereits betreibt.

<h3 id="claude-apps-gateway">
  Claude-Apps-Gateway
</h3>

Claude-Apps-Gateway ist Anthropics selbstgehostetes Gateway, das in der `claude`-Binärdatei enthalten ist. Es leitet zu Amazon Bedrock, Claude Platform auf AWS, Google Cloud, Microsoft Foundry oder der Anthropic API als Upstream weiter. Entwickler melden sich über `/login` mit Ihrem Unternehmensidentitätsanbieter an, das Gateway erzwingt Modellzugriff und [verwaltete Einstellungen](/docs/de/permissions#managed-settings) nach IdP-Gruppe, und es gibt [OpenTelemetry Protocol (OTLP)](/docs/de/monitoring-usage)-Nutzungsmetriken an Ihren eigenen Observability-Stack aus.

Da es zusammen mit jeder Claude Code-Version erstellt und getestet wird, leitet es die Header und Anfragfelder weiter, die Claude Code sendet. Ein separat verwaltetes Gateway muss seine [Weiterleitungsregeln aktualisieren](/docs/de/llm-gateway-protocol#forward-as-open-lists), wenn sich diese Header und Felder mit jeder Version ändern; Claude-Apps-Gateway wird mit der CLI veröffentlicht, sodass es keine Liste gibt, die aktuell gehalten werden muss. Siehe [Verfügbarkeit und Einschränkungen](/docs/de/claude-apps-gateway#availability-and-limitations) für die kleine Menge von Funktionen, die sich in einer Gateway-Sitzung unterschiedlich verhalten.

Die Gateway-Anmeldung ist ein Browser-SSO-Schritt, und es gibt keinen Service-Token-Flow, sodass eine CI-Pipeline ohne einen Entwickler, der die Anmeldung genehmigt, sich nicht über sie authentifizieren kann; konfigurieren Sie diese direkt gegen Ihren Anbieter. Agent SDK-Sitzungen und `claude -p`-Läufe auf einer Maschine, auf der sich ein Entwickler angemeldet hat, verwenden die Gateway-Sitzung dieser Maschine und werden durch ihre Richtlinien geregelt. Siehe [CI-Pipelines und Remote-Maschinen](/docs/de/claude-apps-gateway#ci-pipelines-and-remote-machines).

Siehe [Claude-Apps-Gateway](/docs/de/claude-apps-gateway), um es bereitzustellen.

<h3 id="other-gateways">
  Andere Gateways
</h3>

Wenn Ihre Organisation bereits ein LLM-Gateway oder API-Gateway betreibt, können Sie es stattdessen verwenden. Anthropic unterstützt, verwaltet oder prüft keine anderen Gateway-Produkte und unterstützt nicht das Routing von Claude Code zu Nicht-Claude-Modellen über ein Gateway. Siehe [Andere LLM-Gateways](/docs/de/llm-gateway) für die Admin-Rollout-Checkliste, was ein Gateway implementieren muss, und wie Sie Claude Code darauf verweisen.

<h2 id="subscriptions-and-gateways">
  Abonnements und Gateways
</h2>

Wenn Entwickler sich über ein Gateway mit einem Gateway-Anmeldedatum verbinden, wird die Nutzung zu API-Raten auf das Anbieterkonto Ihrer Organisation abgerechnet, und ihre claude.ai-Abonnements werden nicht verwendet oder berechnet. Das Setzen von [`ANTHROPIC_AUTH_TOKEN`](/docs/de/env-vars) für ein Gateway, das Sie betreiben, oder die Anmeldung bei einem Claude-Apps-Gateway mit `/login` deaktiviert die Abonnement-Anmeldung für diese Sitzung. Jede Anfrage, die unter diesem Anmeldedatum weitergeleitet wird, wird dem Konto hinter dem Anmeldedatum des Gateway-Providers berechnet.

Die Ausnahme ist das Setzen von nur `ANTHROPIC_BASE_URL`, ohne Gateway-Anmeldedatum. Anfragen werden immer noch über das Gateway weitergeleitet, aber eine gespeicherte claude.ai-Anmeldung bleibt das aktive Anmeldedatum, sodass die Nutzungslimits und Abrechnung des Abonnements gelten. [Andere LLM-Gateways](/docs/de/llm-gateway#subscriptions-and-gateways) behandelt diese Konfiguration und was das Gateway weiterleiten muss, damit es funktioniert.

<h2 id="configure-separately-from-the-gateway">
  Separat vom Gateway konfigurieren
</h2>

Ein Gateway leitet Modell-API-Anfragen weiter. Ein paar Dinge, die Sie erwarten könnten, dass es sie verarbeitet, werden anderswo konfiguriert:

* **Welches Modell antwortet**: Wählen Sie das Modell mit dem `/model`-Befehl oder [Modell-Umgebungsvariablen](/docs/de/model-config#setting-your-model). Das Gateway entscheidet, wohin Anfragen gehen, nicht welches Modell der Entwickler auswählt. Claude-Apps-Gateway kann die Auswahl mit einer Pro-Gruppen-`availableModels`-Zulassungsliste begrenzen, aber der Entwickler wählt immer noch innerhalb davon.
* **Anderer Netzwerk-Traffic**: Claude Code selbst sendet Versionsüberprüfungen und Downloads direkt an Anthropic, getrennt vom Gateway-Pfad. Ob der optionale Client-Telemetrie-Stream auch davon abhängt, hängt von Ihrem Anbieter ab; die [Telemetrie-Standardtabelle](/docs/de/data-usage#telemetry-services) behandelt jeden Fall. In einer angemeldeten Claude-Apps-Gateway-Sitzung deaktiviert das Gateway-Anmeldedatum die Anthropic-gebundene Analytik und, wenn [Telemetrie-Weiterleitung](/docs/de/claude-apps-gateway-config#telemetry) konfiguriert ist, heftet OTLP-Export an das Gateway. Ihr Netzwerk benötigt immer noch Ausgang zu den [erforderlichen Domänen](/docs/de/network-config), oder setzen Sie [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/de/env-vars), um die optionalen Streams auszuschalten.
* **Unternehmens-HTTP-Proxies**: Ein `HTTPS_PROXY` sitzt zwischen Claude Code und jedem Server, mit dem es spricht, einschließlich des Gateways. Wenn Ihr Netzwerk einen benötigt, [konfigurieren Sie den Proxy](/docs/de/network-config) zusätzlich zum Gateway. Für ein Claude apps Gateway, das Sie hosten, [überprüft die Anmeldung, dass der Proxy-Host auch in einem privaten Netzwerk ist](/docs/de/claude-apps-gateway#prerequisites); wenn nicht, fügen Sie den Gateway-Host zu `NO_PROXY` hinzu, damit die CLI sich direkt mit ihm verbindet.

<h2 id="next-steps">
  Nächste Schritte
</h2>

Die nächste Seite hängt davon ab, wer das Gateway betreibt. Anthropics Gateway wird aus der `claude`-Binärdatei ausgeführt und hat seinen eigenen Setup-Leitfaden; ein Gateway, das Ihre Organisation bereits betreibt, hat ein Protokoll zum Implementieren und eine Admin-Rollout-Checkliste.

* [Claude-Apps-Gateway](/docs/de/claude-apps-gateway), um Anthropics selbstgehostetes Gateway mit SSO-Anmeldung und OTLP-Telemetrie bereitzustellen
* [Andere LLM-Gateways](/docs/de/llm-gateway) für das, was ein Gateway, das Ihre Organisation bereits betreibt, implementieren muss, und wie Sie Claude Code darauf verweisen
* [Richten Sie Claude Code für Ihre Organisation ein](/docs/de/admin-setup) für die breiteren Rollout-Entscheidungen, von denen ein Gateway ein Teil ist
