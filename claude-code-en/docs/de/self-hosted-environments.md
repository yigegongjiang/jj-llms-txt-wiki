> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Selbst gehostete Umgebungen

> Führen Sie Claude Code Cloud-Sitzungen auf einer Infrastruktur aus, die Sie kontrollieren: Richten Sie eine selbst gehostete Umgebung ein, stellen Sie Runner bereit und leiten Sie Sitzungen zu Ihrem eigenen Compute weiter.

<Note>
  Selbst gehostete Umgebungen befinden sich in der öffentlichen Beta für Team- und Enterprise-Pläne und sind standardmäßig deaktiviert. Siehe [Verfügbarkeit und Einschränkungen](#availability-and-limitations) für den Aktivierungspfad und was ausgeschlossen ist.
</Note>

Eine selbst gehostete Umgebung führt Claude Code Cloud-Sitzungen auf einer Infrastruktur aus, die Ihre Organisation betreibt. Eine [Cloud-Sitzung](/docs/de/claude-code-on-the-web) ist jede Sitzung, die irgendwo anders als auf dem Rechner des Entwicklers ausgeführt wird: Entwickler starten sie von claude.ai, den mobilen und Desktop-Apps, dem Terminal mit [`claude --cloud`](/docs/de/claude-code-on-the-web#from-terminal-to-cloud) und [geplanten Routinen](/docs/de/routines), und standardmäßig werden sie auf der Infrastruktur von Anthropic ausgeführt. In einer selbst gehosteten Umgebung werden diese Sitzungen in Ihrem Netzwerk ausgeführt, und die Entwicklererfahrung ist ansonsten gleich, abgesehen von den Unterschieden in [Verfügbarkeit und Einschränkungen](#availability-and-limitations) und den [bekannten Problemen](/docs/de/self-hosted-environments-deploy#known-issues-and-limitations) der Bereitstellungsseite.

Wenn Ihr Team keine Cloud-Sitzungen verwendet, gibt es hier nichts zu konfigurieren: Sitzungen in einem Terminal oder einer IDE werden immer auf dem eigenen Rechner des Entwicklers ausgeführt. Wenn Sie Claude Code auf Ihrem eigenen Always-On-Rechner ausführen und es von anderen Geräten aus steuern möchten, verwenden Sie [Remote Control](/docs/de/remote-control), das auch auf Pro- und Max-Plänen verfügbar ist. Wenn Sie bereit sind zu beginnen, gehen Sie direkt zum [Schnellstart](/docs/de/self-hosted-environments-quickstart); um die Sicherheitslage zunächst zu überprüfen, beginnen Sie mit [Bereitstellung in der Produktion](/docs/de/self-hosted-environments-deploy). Der Rest dieser Seite erklärt, wie Self-Hosting funktioniert und wann Sie es wählen sollten.

<h2 id="how-self-hosted-environments-work">
  Wie selbst gehostete Umgebungen funktionieren
</h2>

Self-Hosting hat drei Teile:

* **Umgebung**: ein benanntes Ziel, an das Cloud-Sitzungen gesendet werden können. Ihre Organisation erstellt Umgebungen in den claude.ai Admin-Einstellungen, und jede gruppiert einen Satz von Runnern.
* **Runner**: ein Programm, das auf Hosts in Ihrem Netzwerk ausgeführt wird. Runner führen die Sitzungen aus; die Idee ist die gleiche wie bei einem selbst gehosteten CI-Runner.
* **Sitzung**: eine Claude Code-Aufgabe, die ein Entwickler gestartet hat.

Wenn ein Entwickler eine Cloud-Sitzung startet, zeigt die Sitzungsstart-UI eine Umgebungsauswahl an, die von Anthropic gehostete Umgebungen neben allen von Ihrer Organisation erstellten auflistet. Wenn er Ihre auswählt, platziert die Kontrolleben von Anthropic die Sitzung in der Warteschlange Ihrer Umgebung, wo ein Runner sie beansprucht, das vom Entwickler gewählte Repository klont und einen Claude Code-Prozess auf Ihrem Host startet, um ihn auszuführen. Der Runner authentifiziert sich bei Ihrem Git-Host mit Anmeldedaten, die Sie konfigurieren; [Git konfigurieren](/docs/de/self-hosted-environments-deploy#configure-git) behandelt die Optionen. Sitzungen erreichen Ihre internen Services von innen in Ihrem Netzwerk aus, und Ihren Git-Host auf die gleiche Weise, wenn er intern ist; der Datenverkehr zu Anthropic, das Abfragen der Warteschlange, der Ereignisstrom der Sitzung und die Modellableitung ist ausgehend HTTPS zu `api.anthropic.com`, mit der kurzen Liste weiterer Hosts, die Sitzungen in [Netzwerkanforderungen](/docs/de/self-hosted-environments-deploy#network-requirements) erreichen können. Anthropic verbindet sich niemals mit Ihrem Netzwerk.

<div style={{maxWidth: "640px", margin: "0 auto"}}>
  <Frame>
    <img src="https://mintcdn.com/claude-code/Y0sJ2uDoOVbOVZrQ/images/self-hosted-network-paths.svg?fit=max&auto=format&n=Y0sJ2uDoOVbOVZrQ&q=85&s=8056103fc1c5564c7f0ef219d260b99d" className="dark:hidden" alt="Architekturdiagramm einer selbst gehosteten Umgebung: Ihre Netzwerkgrenze enthält einen Runner, zwei Claude Code-Sitzungsprozesse darin und Ihren Git-Host, mit api.anthropic.com außerhalb, das Warteschlange, Sitzungsstrom und Ableitung hält. Der Runner fragt die Warteschlange ab und erreicht den Git-Host, jeder Sitzungsprozess öffnet seine eigenen Stream-, Ableitungs- und Git-Verbindungen, und jede Verbindung ist ausgehend von Ihrem Netzwerk, ohne eingehende." width="680" height="320" data-path="images/self-hosted-network-paths.svg" />

    <img src="https://mintcdn.com/claude-code/Y0sJ2uDoOVbOVZrQ/images/self-hosted-network-paths-dark.svg?fit=max&auto=format&n=Y0sJ2uDoOVbOVZrQ&q=85&s=fec6aef3b0740d80eaf6d6a7000a2233" className="hidden dark:block" alt="Architekturdiagramm einer selbst gehosteten Umgebung: Ihre Netzwerkgrenze enthält einen Runner, zwei Claude Code-Sitzungsprozesse darin und Ihren Git-Host, mit api.anthropic.com außerhalb, das Warteschlange, Sitzungsstrom und Ableitung hält. Der Runner fragt die Warteschlange ab und erreicht den Git-Host, jeder Sitzungsprozess öffnet seine eigenen Stream-, Ableitungs- und Git-Verbindungen, und jede Verbindung ist ausgehend von Ihrem Netzwerk, ohne eingehende." width="680" height="320" data-path="images/self-hosted-network-paths-dark.svg" />
  </Frame>
</div>

Die zwei Claude Code-Boxen im Diagramm sind Sitzungsprozesse: ein Runner führt zwei Sitzungen gleichzeitig aus, bis zu seiner konfigurierten Kapazität. Ein Runner bedient einen [Besitzer](#key-concepts) auf einmal und sperrt sich auf diesen Besitzer, wenn er seine erste Sitzung beansprucht, sodass ausgecheckter Code sich niemals zwischen Besitzern vermischt; [Runner-Lebenszyklus](#runner-lifecycle) behandelt die Regel.

Sie können Runner selbst starten und am Laufen halten, oder Sie führen den [Autoscaling-Orchestrator](/docs/de/self-hosted-environments-configuration#on-demand-runners) aus, einen zweiten Prozess, den Sie hosten, der Runner startet, wenn Sitzungen in der Warteschlange sind; jeder Runner beendet sich selbst, wenn seine Arbeit abgeschlossen ist. In jedem Fall richten Sie die Umgebung einmal ein, und sie wird in der Auswahl auf jeder unterstützten Oberfläche angezeigt.

<h2 id="availability-and-limitations">
  Verfügbarkeit und Einschränkungen
</h2>

Überprüfen Sie diese vor der Planung eines Rollouts:

* **Pläne**: öffentliche Beta für Team- und Enterprise-Organisationen. Selbst gehostete Umgebungen sind standardmäßig deaktiviert; ein [Besitzer](/docs/de/cloud-environments#organization-shared-environments) aktiviert **Selbst gehostete Umgebungen zulassen** auf der [**Cloud-Umgebungen** Admin-Seite](https://claude.ai/admin-settings/cloud-environments), was erfordert, dass [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) für die Organisation aktiviert sind.
* **Zero Data Retention**: nicht verfügbar für Organisationen mit [Zero Data Retention](/docs/de/zero-data-retention) aktiviert.
* **Modellableitung**: Sitzungen verwenden die Anthropic API, und die Ableitung kann nicht durch [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry](/docs/de/third-party-integrations) oder ein [LLM-Gateway](/docs/de/llm-gateway) geleitet werden.
* **Oberflächen**: Sitzungen, die von [claude.ai/code](https://claude.ai/code), den mobilen und Desktop-Apps, [geplanten Routinen](/docs/de/routines) und dem Terminal mit [`claude --cloud`](/docs/de/claude-code-on-the-web#from-terminal-to-cloud) oder einer [`--environment` Dispatch](/docs/de/self-hosted-environments-testing#run-the-test-loop) gestartet werden, können in selbst gehosteten Umgebungen ausgeführt werden. [Claude Tag](https://claude.com/docs/claude-tag/overview) Sitzungen können auch darin ausgeführt werden, aber Claude kann [Access Bundles](https://claude.com/docs/claude-tag/concepts/glossary#access-bundle) in diesen Sitzungen noch nicht verwenden. [Claude Security](/docs/de/claude-security) und [Code Review](/docs/de/code-review) Sitzungen werden noch nicht zu ihnen weitergeleitet. Die Unterstützung für diese beiden Oberflächen folgt separat.
* **Repositories**: Sitzungen checken Repositories von GitHub aus; siehe [GitHub-Authentifizierungsoptionen](/docs/de/claude-code-on-the-web#github-authentication-options).
* **Abrechnung**: Sitzungen in einer selbst gehosteten Umgebung verbrauchen die Claude Code-Nutzung Ihrer Organisation auf die gleiche Weise wie Sitzungen in von Anthropic gehosteten Umgebungen.

<h2 id="why-self-host">
  Warum Self-Hosting
</h2>

Die meisten Teams werden besser von von Anthropic gehosteten Umgebungen bedient, die keine Infrastruktur zum Ausführen oder Warten benötigen. Self-Hosting ist für Teams, deren Netzwerk-, Tooling- oder Compliance-Anforderungen erfordern, dass die Sitzungsausführung auf einer Infrastruktur erfolgt, die sie kontrollieren. Wenn das auf Sie zutrifft, planen Sie für die operative Verantwortung, die es mit sich bringt: Sie erstellen und warten das Runner-Image, betreiben die Flotte und kontrollieren sein Netzwerk.

Im Gegenzug bietet Self-Hosting Ihnen Netzwerkzugriff, benutzerdefinierte Tools und Compliance-Kontrolle:

* **Netzwerkzugriff**: Sitzungen werden in Ihrem Netzwerk ausgeführt und können interne Services, Datenbanken und Registries erreichen, ohne sie dem öffentlichen Internet auszusetzen
* **Benutzerdefinierte Tools**: Installieren Sie Compiler, SDKs und interne CLIs in Ihrem Runner-Image vor, damit jede Sitzung bereit zum Erstellen startet
* **Compliance**: Repository-Checkouts und Build-Artefakte bleiben auf einer Infrastruktur, die Sie kontrollieren. Der Sitzungsinhalt wird immer noch zu `api.anthropic.com` für die Modellableitung gesendet.

<h2 id="environments-runners-and-sessions">
  Umgebungen, Runner und Sitzungen
</h2>

Umgebungen werden auf der Seite **Cloud-Umgebungen** in den Admin-Einstellungen von claude.ai verwaltet; Runner sind Prozesse, die Sie auf Ihrer eigenen Infrastruktur starten und verwalten.

<h3 id="key-concepts">
  Wichtige Konzepte
</h3>

Diese Begriffe erscheinen auf den Seiten für selbstgehostete Umgebungen:

| Begriff            | Was es ist                                                                                                                                                                                                                                                      |
| :----------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Umgebung           | Eine benannte Gruppe Ihrer Runner, erstellt in den claude.ai-Einstellungen. Sitzungen werden an eine Umgebung weitergeleitet, nicht an einen einzelnen Runner.                                                                                                  |
| Umgebungsgeheimnis | Die einzelne gemeinsame Anmeldeinformation, die Runner zur Authentifizierung und Registrierung bei der Umgebung verwenden. Wird einmal bei der Umgebungserstellung angezeigt und ist in der Admin-Benutzeroberfläche als **Umgebungsschlüssel** gekennzeichnet. |
| Runner             | Der langlebige Prozess, den Sie bereitstellen. Ein Runner registriert sich bei der Umgebung, erhält ein Runner-Token und fragt nach Sitzungen ab.                                                                                                               |
| Sitzung            | Eine Claude Code-Aufgabe, gestartet von claude.ai, der mobilen App oder einer anderen Anthropic-Oberfläche wie einer geplanten Routine oder einem Agent. Jede Sitzung wird als untergeordneter Claude Code-Prozess ausgeführt, den der Runner erzeugt.          |

In API-Feldern, Token-Ansprüchen und Metriknamen wird die Umgebung als `pool` angezeigt, und die Umgebungs-ID ist die `pool_id`. Die [Referenz](/docs/de/self-hosted-environments-reference) ordnet die beiden Schreibweisen zu, einschließlich der veralteten `pool`-Flag-Namen.

Ein Runner bedient jeweils einen Besitzer. Die erste Sitzung, die ein Runner aufgreift, sperrt den Runner für den Besitzer dieser Sitzung, und der Runner führt dann Sitzungen nur für diesen Besitzer bis zu einer konfigurierten Kapazität aus. Wer der Besitzer ist, hängt davon ab, wie die Sitzung gestartet wurde:

* **Sitzungen, die ein Benutzer startet**: Der Besitzer ist das Konto dieses Benutzers.
* **Claude Tag-Kanal-Sitzungen**: Claude führt sie ohne ein Benutzerkonto aus, daher ist der Besitzer der [Claude Tag-Agent](https://claude.com/docs/claude-tag/concepts/glossary#agent-identity), der die Sitzung gestartet hat. Jede Kanal-Sitzung, die dieser Agent startet, hat denselben Besitzer, unabhängig davon, wer die Slack-Nachricht gesendet hat. Ein Runner, der daran gesperrt ist, bedient Sitzungen, die verschiedene Personen gestartet haben, wenn Sie ihn mit einer `--capacity` über eins oder mit einem positiven `--drain-grace-sec` ausführen. Ein Runner, der an einen Benutzer gesperrt ist, greift diese nicht auf, und ein Runner, der an einen Claude Tag-Agent gesperrt ist, greift niemals die Sitzungen eines Benutzers auf.

Die minimale Flottengröße ist daher die Anzahl der Besitzer, die Sie gleichzeitig aktiv erwarten, einschließlich Benutzer und Claude Tag-Agenten.

<h3 id="session-lifecycle">
  Sitzungs-Lebenszyklus
</h3>

Wenn ein Entwickler eine Sitzung startet und Ihre Umgebung auswählt, platziert die Kontrolleben von Anthropic die Sitzung in der Warteschlange der Umgebung. Von dort aus:

1. Ein Runner mit freier Kapazität beansprucht die Sitzung und hält einen Lease darauf.
2. Der Runner klont das Repository in sein Arbeitsverzeichnis und erzeugt einen untergeordneten Claude Code-Prozess.
3. Das untergeordnete Element streamt Ereignisse über HTTPS zurück, während der Runner weiterhin abfragt; jede Abfrage aktualisiert den Lease und dient gleichzeitig als Herzschlag.
4. Wenn der Runner etwa 60 Sekunden lang nicht mehr abfragt, reiht der Server die Sitzung für einen anderen Runner erneut ein.

Der Runner gibt jeder Abfrageanfrage 10 Sekunden. Wenn eine Anfrage abläuft, verloren geht oder eine Antwort erhält, die der Runner nicht analysieren kann, bedient der Runner weiterhin seine aktiven Sitzungen und versucht es nach einer oder zwei Sekunden erneut, anstatt auf die nächste geplante Abfrage zu warten. Beispielsweise antwortet ein abfangender Proxy auf die Abfrage mit seiner eigenen Seite und erzeugt eine Antwort, die der Runner nicht analysieren kann. Jedes Mal, wenn eine weitere Anfrage auf eine dieser Weisen fehlschlägt, verdoppelt der Runner die Lücke vor dem nächsten Versuch, bis zu 20 Sekunden, und verkürzt die Lücke, wenn der Lease kurz vor dem Ablauf steht.

<h3 id="runner-lifecycle">
  Runner-Lebenszyklus
</h3>

Die erste Sitzung, die ein Runner aufgreift, sperrt den Runner für den Besitzer dieser Sitzung, und der Runner führt bis zu `--capacity` gleichzeitige Sitzungen für diesen Besitzer aus. Während der Runner aktive Sitzungen hat und kein Shutdown-Signal erhalten hat oder seine Ruhestandszeit erreicht hat, greift der Runner weiterhin die Arbeit in der Warteschlange des gesperrten Besitzers auf. Was nach Abschluss geschieht, hängt von [`--drain-grace-sec`](/docs/de/self-hosted-environments-reference#runner-cli-flags) ab:

* **Bei der Standardeinstellung von `0`**: Der Runner wird beendet, sobald seine aktiven Sitzungen beendet sind, ohne weitere Abfragen durchzuführen, sodass der Orchestrator, unter dem Sie ihn bereitstellen, wie Kubernetes, ihn mit einer frischen Festplatte neu starten kann, bereit, jeden Besitzer zu bedienen.
* **Bei einem positiven Wert**: Der Runner fragt die Warteschlange des gesperrten Besitzers für diese Anzahl von Sekunden ab, bevor er beendet wird.

Dieser Lebenszyklus isoliert den ausgecheckten Code jedes Besitzers, ohne dass der Runner den Festplattenzustand zwischen Besitzern löschen muss.

Wie Ihre Infrastruktur einen Runner stoppt, entscheidet, ob Sie `--retire-at` benötigen. Ein Kill, der `SIGTERM` liefert, benötigt kein Flag: Der Runner wird wie in [Shutdown-Timing](/docs/de/self-hosted-environments-deploy#shutdown-timing) beschrieben geleert, oder bedient weiterhin die Sitzungen, die er bereits hält, wenn Sie [`--defer-shutdown-max-min`](/docs/de/self-hosted-environments-deploy#defer-the-drain-past-the-first-signal) setzen. Wenn Ihre Infrastruktur stattdessen Hosts zu einer bekannten Wanduhrzeit ohne Signal oder mit einer zu kurzen Kulanzfrist zerstört, wie z. B. eine Sandbox-Lebensdauer-Obergrenze oder Spot-Instance-Rückforderung, übergeben Sie `--retire-at <epoch-seconds>` auf einige Minuten vor dieser Zeit eingestellt. Zur Ruhestandszeit:

1. Der Runner stoppt die Annahme neuer Arbeit.
2. Der Runner gibt jede aktive Sitzung über denselben Freigabepfad frei, den das Flag [`--release-idle-session-min`](/docs/de/self-hosted-environments-reference#runner-cli-flags) verwendet, sodass die Sitzung auf einem frischen Runner fortgesetzt wird, wenn der Benutzer seine nächste Nachricht sendet. Wann der Runner jede Sitzung freigibt, hängt von ihrem Zustand ab:
   * Der Runner gibt eine Sitzung frei, die sich mitten in einem Turn befindet, sobald dieser Turn beendet ist.
   * Wenn ein Turn beendet ist und Hintergrundaufgaben ausgeführt werden, wartet der Runner bis zu 60 Sekunden auf diese, gibt dann die Sitzung frei, auch wenn diese noch ausgeführt werden. Wenn die Aufgaben beendet sind, aber der nachfolgende Turn, der ihre Ergebnisse liest, noch nicht ausgeführt wurde, behält der Runner die Sitzung, bis dieser Turn beendet ist, und wartet nicht länger als [`SELF_HOSTED_RUNNER_BG_RESULT_GRACE_MS`](/docs/de/self-hosted-environments-reference#environment-variable-only-settings) darauf, dass dieser Turn startet.
3. Der Runner wird mit 0 beendet, sobald alle seine Sitzungen freigegeben sind.

Ein Turn, der den Kill überlebt, geht immer noch verloren; [Shutdown-Timing](/docs/de/self-hosted-environments-deploy#shutdown-timing) behandelt die Dimensionierung der Marge. Ohne `--retire-at` ist ein signalloser Host-Kill nicht zu unterscheiden von einem Absturz: Die Kontrolleben zeichnet einen verlorenen Worker auf, anstatt eine saubere Freigabe, und die Sitzung wird an einen anderen Runner erneut eingegeben.

<h3 id="network-paths">
  Netzwerkpfade
</h3>

Der Runner und seine Sitzungen stellen mehrere Arten von ausgehenden Verbindungen her, und es ist keine eingehende Konnektivität von Anthropic erforderlich:

* **Kontrolleben**: Der Runner fragt `api.anthropic.com` nach Arbeit ab und sendet Setup-Fortschritts- und Fehlerereignisse, alle ausgehend HTTPS. Das Abfragen dient auch als Herzschlag des Runners.
* **SCM-Connector**: Der optionale Orchestrator [SCM-Connector](/docs/de/self-hosted-environments-reference#scm-connector-flags) Tunnel ist die einzige WebSocket-Verbindung.
* **Git**: Der Runner klont von und pusht zu Ihrem Git-Host über HTTPS oder SSH, authentifiziert mit Anmeldeinformationen, die Ihre Bereitstellung bereitstellt; [Git konfigurieren](/docs/de/self-hosted-environments-deploy#configure-git) behandelt die Optionen, einschließlich pro-Sitzung geprägte Anmeldeinformationen und der [Anthropic Git-Proxy](/docs/de/self-hosted-environments-deploy#use-the-anthropic-git-proxy), der Git durch `api.anthropic.com` leitet.
* **Sitzungs-Child**: Der untergeordnete Claude Code-Prozess hält den Ereignisstrom der Sitzung zu `api.anthropic.com` und führt seine eigenen ausgehenden Aufrufe für Modell-Inferenz und für Git-Befehle durch, die während der Sitzung ausgeführt werden. Siehe [Netzwerkanforderungen](/docs/de/self-hosted-environments-deploy#network-requirements) für die vollständige Egress-Liste. Das [Diagramm oben](#how-self-hosted-environments-work) zeigt diese Pfade, abgesehen vom optionalen SCM-Connector.

Die Modell-Inferenz verwendet die Anthropic API. Die Kontrolleben liefert den API-Endpunkt an jede Sitzung, und die Sitzung authentifiziert sich mit einem von Anthropic ausgegebenen, sitzungsbezogenen OAuth-Token, sodass Inferenz nicht durch [Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry](/docs/de/third-party-integrations) oder ein [LLM-Gateway](/docs/de/llm-gateway) in selbstgehosteten Umgebungen weitergeleitet werden kann.

Corporate-Egress-Proxys werden unterstützt. Der Runner und der optionale [Autoscaling-Orchestrator](/docs/de/self-hosted-environments-configuration#on-demand-runners) beachten den Proxy und die mTLS-Umgebungsvariablen, die in [Netzwerkkonfiguration](/docs/de/network-config) beschrieben sind, wie `HTTPS_PROXY` und `NO_PROXY`; setzen Sie sie in der Umgebung jedes Prozesses. Die Variablen decken Kontrolleben-Aufrufe, den [SCM-Connector](/docs/de/self-hosted-environments-reference#scm-connector-flags) WebSocket des Orchestrators und den integrierten Klon für HTTPS-Remotes ab, und Sitzungen erben sie vom Runner. Sitzungs-Streaming verwendet Server-Sent Events über HTTPS, daher darf ein Proxy im Pfad Antworten nicht puffern.

Wenn Ihr Proxy auch einen `Proxy-Authorization`-Header benötigt, kann der Runner ihn zu jeder Verbindung hinzufügen, die er zum Proxy öffnet; siehe [Authentifizierung bei einem Egress-Proxy](/docs/de/self-hosted-environments-deploy#authenticate-to-an-egress-proxy).

<h2 id="what-stays-on-your-infrastructure">
  Was auf Ihrer Infrastruktur bleibt
</h2>

Repository-Checkouts, Build-Artefakte, Geheimnisse und alle Dateien, die eine Sitzung erstellt oder ändert, bleiben auf den Maschinen, die Sie bereitstellen. Das Gespräch selbst, einschließlich Eingabeaufforderungen, Antworten und Tool-Ergebnisse, geht zu `api.anthropic.com` für die Modellableitung, und Anthropic speichert das Sitzungstranskript, damit Sie die Sitzung von einer anderen [unterstützten Oberfläche](#availability-and-limitations) aus fortsetzen können.

Eine selbst gehostete Umgebung verschiebt die Sitzungsausführung in Ihr Netzwerk. Die Kontrolleben bleibt von Anthropic gehostet: Sitzungsorchestration, Queueing und die claude.ai Schnittstelle werden weiterhin auf der Infrastruktur von Anthropic ausgeführt.

<h2 id="get-started">
  Erste Schritte
</h2>

Die Self-Hosting-Umgebungsseiten sind nach dem organisiert, was Sie tun:

* [Schnellstart](/docs/de/self-hosted-environments-quickstart): Installieren Sie Claude Code, erstellen Sie eine Umgebung, starten Sie einen Runner und leiten Sie Ihre erste Sitzung weiter
* [Bereitstellung in der Produktion](/docs/de/self-hosted-environments-deploy): Sicherheitshärtung, Netzwerk-Egress, Git-Anmeldedaten, Kubernetes- und Compose-Rezepte, bekannte Probleme und Fehlerbehebung
* [Sitzungen anpassen](/docs/de/self-hosted-environments-configuration): Wrapper-Skripte für pro-Sitzung Anmeldedaten, Lifecycle-Hooks, On-Demand-Runner, MCP-Server und Berechtigungen
* [End-to-End testen](/docs/de/self-hosted-environments-testing): Ein CI-Smoke-Test, der ein Runner-Image überprüft, bevor Sie es hochstufen
* [Referenz](/docs/de/self-hosted-environments-reference): Jedes CLI-Flag, jede Umgebungsvariable, jede Metrik und der Health-Endpunkt
* [Sitzungsidentität überprüfen](/docs/de/self-hosted-environments-identity): Validieren Sie das Sitzungs-Token von Ihren eigenen Services, bevor Sie Zugriff gewähren
