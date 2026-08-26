> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code für Ihre Organisation einrichten

> Eine Entscheidungskarte für Administratoren, die Claude Code bereitstellen, mit Abdeckung von API-Anbietern, verwalteten Einstellungen, Richtliniendurchsetzung, Nutzungsüberwachung und Datenbehandlung.

Claude Code erzwingt Organisationsrichtlinien durch verwaltete Einstellungen, die Vorrang vor lokalen Entwicklerkonfigurationen haben. Sie stellen diese Einstellungen über die Claude-Administratorkonsole, Ihr Mobile-Device-Management-System (MDM) oder eine Datei auf der Festplatte bereit. Die Einstellungen steuern, auf welche Tools, Befehle, Server und Netzwerkziele Claude zugreifen kann.

Diese Seite führt Sie durch die Bereitstellungsentscheidungen in der richtigen Reihenfolge. Jede Zeile verlinkt auf den Abschnitt unten und auf die Referenzseite für diesen Bereich.

<Note>
  SSO, SCIM-Bereitstellung und Sitzplatzzuweisung werden auf Claude-Kontoebene konfiguriert. Siehe das [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide) und [Sitzplatzzuweisung](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan) für diese Schritte.
</Note>

| Entscheidung                                                                              | Was Sie wählen                                                  | Referenz                                                                                                                                                                         |
| :---------------------------------------------------------------------------------------- | :-------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [API-Anbieter wählen](#choose-your-api-provider)                                          | Wo Claude Code sich authentifiziert und wie es abgerechnet wird | [Authentifizierung](/docs/de/authentication), [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai), [Microsoft Foundry](/docs/de/microsoft-foundry) |
| [Entscheiden Sie, wie Einstellungen Geräte erreichen](#decide-how-settings-reach-devices) | Wie verwaltete Richtlinien Entwicklermaschinen erreichen        | [Server-verwaltete Einstellungen](/docs/de/server-managed-settings), [Einstellungsdateien](/docs/de/settings#settings-files)                                                               |
| [Entscheiden Sie, was durchgesetzt werden soll](#decide-what-to-enforce)                  | Welche Tools, Befehle und Integrationen zulässig sind           | [Berechtigungen](/docs/de/permissions), [Sandboxing](/docs/de/sandboxing)                                                                                                                  |
| [Nutzungssichtbarkeit einrichten](#set-up-usage-visibility)                               | Wie Sie Ausgaben und Akzeptanz verfolgen                        | [Analytik](/docs/de/analytics), [Überwachung](/docs/de/monitoring-usage), [Kosten](/docs/de/costs)                                                                                              |
| [Datenbehandlung überprüfen](#review-data-handling)                                       | Datenspeicherung und Compliance-Position                        | [Datennutzung](/docs/de/data-usage), [Sicherheit](/docs/de/security)                                                                                                                       |

<h2 id="choose-your-api-provider">
  API-Anbieter wählen
</h2>

Claude Code verbindet sich mit Claude über einen von mehreren API-Anbietern. Ihre Wahl beeinflusst die Abrechnung, die Authentifizierung, welche Compliance-Position Sie erben, und welche Claude Code-Funktionen Ihre Entwickler nutzen können.

| Anbieter                      | Wählen Sie dies, wenn                                                                                                                    |
| :---------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| Claude für Teams / Enterprise | Sie möchten Claude Code und claude.ai unter einem Pro-Sitz-Abonnement ohne Infrastruktur zum Ausführen. Dies ist die Standardempfehlung. |
| Claude Console                | Sie sind API-first oder möchten Pay-as-you-go-Abrechnung                                                                                 |
| Amazon Bedrock                | Sie möchten vorhandene AWS-Compliance-Kontrollen und Abrechnung erben                                                                    |
| Google Cloud's Agent Platform | Sie möchten vorhandene GCP-Compliance-Kontrollen und Abrechnung erben                                                                    |
| Microsoft Foundry             | Sie möchten vorhandene Azure-Compliance-Kontrollen und Abrechnung erben                                                                  |

Einige Claude Code-Funktionen erfordern ein claude.ai-Konto. [Claude Code im Web](/docs/de/claude-code-on-the-web), [Routinen](/docs/de/routines), [Code Review](/docs/de/code-review), [Remote Control](/docs/de/remote-control) und die [Chrome-Erweiterung](/docs/de/chrome) sind nicht über Console API-Schlüssel oder Cloud-Provider-Anmeldedaten allein verfügbar. Wenn Sie über Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry bereitstellen, planen Sie, ob Entwickler auch Claude für Teams oder Enterprise-Sitze benötigen. Jede Funktionsseite listet ihre Plan-Anforderungen auf.

Für den vollständigen Anbietervergleich mit Authentifizierung, Regionen und Funktionsparität siehe [Übersicht zur Enterprise-Bereitstellung](/docs/de/third-party-integrations). Die Auth-Einrichtung für jeden Anbieter finden Sie unter [Authentifizierung](/docs/de/authentication).

Proxy- und Firewall-Anforderungen in [Netzwerkkonfiguration](/docs/de/network-config) gelten unabhängig vom Anbieter. Wenn Sie einen einzelnen Endpunkt vor mehreren Anbietern oder zentralisierte Anforderungsprotokollierung möchten, siehe [LLM-Gateway](/docs/de/llm-gateway).

<h2 id="decide-how-settings-reach-devices">
  Entscheiden Sie, wie Einstellungen Geräte erreichen
</h2>

Verwaltete Einstellungen definieren Richtlinien, die Vorrang vor lokalen Entwicklerkonfigurationen haben. Claude Code sucht an vier Stellen in der folgenden Prioritätsreihenfolge danach und wendet die erste an, die eine nicht leere Konfiguration zurückgibt. Es gibt eine Ausnahme: Ein kleiner Satz von [Sperrtasten über Quellen hinweg](/docs/de/settings#settings-precedence), wie die Sandbox-Allowlist-Sperren, wird berücksichtigt, wenn eine von einem Administrator kontrollierte Quelle diese setzt.

| Mechanismus                 | Lieferung                                                                                                                                                                                             | Priorität  | Plattformen    |
| :-------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------- | :------------- |
| Server-verwaltet            | claude.ai-Administratorkonsole oder ein selbst gehostetes [Claude-Apps-Gateway](/docs/de/claude-apps-gateway) für Gateway-Anmeldungen                                                                      | Höchste    | Alle           |
| plist / Registry-Richtlinie | macOS: `com.anthropic.claudecode` plist<br />Windows: `HKLM\SOFTWARE\Policies\ClaudeCode`                                                                                                             | Hoch       | macOS, Windows |
| Dateibasiert verwaltet      | macOS: `/Library/Application Support/ClaudeCode/managed-settings.json`<br />Linux und WSL: `/etc/claude-code/managed-settings.json`<br />Windows: `C:\Program Files\ClaudeCode\managed-settings.json` | Mittel     | Alle           |
| Windows-Benutzer-Registry   | `HKCU\SOFTWARE\Policies\ClaudeCode`                                                                                                                                                                   | Niedrigste | Nur Windows    |

Ein konfigurierter [`policyHelper`](/docs/de/settings#compute-managed-settings-with-a-policy-helper) setzt sich vor alle vier Quellen durch: Seine Ausgabe wird die einzige verwaltete Konfiguration für den Durchlauf. Siehe [Einstellungspriorität](/docs/de/settings#settings-precedence).

Server-verwaltete Einstellungen erreichen Geräte zum Authentifizierungszeitpunkt und werden während aktiver Sitzungen stündlich aktualisiert, ohne dass eine Endpunkt-Infrastruktur erforderlich ist. Die Lieferung über die claude.ai-Administratorkonsole erfordert einen Claude for Teams oder Enterprise-Plan. Bereitstellungen auf Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry können die gleiche Remote-Lieferung durch Ausführung eines [Claude-Apps-Gateways](/docs/de/claude-apps-gateway) erhalten oder stattdessen einen der dateibasierten oder Betriebssystem-Mechanismen verwenden.

Wenn Ihre Organisation mehrere Anbieter mischt, konfigurieren Sie [server-verwaltete Einstellungen](/docs/de/server-managed-settings) für claude.ai-Benutzer plus ein [dateibasiertes oder plist/Registry-Fallback](/docs/de/settings#settings-files), damit andere Benutzer immer noch verwaltete Richtlinien erhalten.

Die plist- und HKLM-Registry-Speicherorte funktionieren mit jedem Anbieter und widerstehen Manipulationen, da sie Administratorrechte zum Schreiben erfordern. Die Windows-Benutzer-Registry unter HKCU ist ohne Erhöhung beschreibbar, daher sollten Sie sie eher als praktischen Standard als als Durchsetzungskanal behandeln.

Standardmäßig liest WSL nur den Linux-Dateipfad unter `/etc/claude-code`. Um Ihre Windows-Registry und `C:\Program Files\ClaudeCode`-Richtlinie auf WSL auf demselben Computer zu erweitern, setzen Sie [`wslInheritsWindowsSettings: true`](/docs/de/settings#available-settings) in einer dieser nur für Administratoren zugänglichen Windows-Quellen.

Welcher Mechanismus Sie auch wählen, verwaltete Werte haben Vorrang vor Benutzer- und Projekteinstellungen. Array-Einstellungen wie `permissions.allow` und `permissions.deny` führen Einträge aus allen Quellen zusammen, sodass Entwickler verwaltete Listen erweitern, aber nicht daraus entfernen können. Bei [zwei Ausnahmen](/docs/de/settings#settings-precedence), `fallbackModel` und `availableModels`, ersetzt der verwaltete Wert die unteren Ebenen, anstatt sie zusammenzuführen.

Siehe [Server-verwaltete Einstellungen](/docs/de/server-managed-settings) und [Einstellungsdateien und Priorität](/docs/de/settings#settings-files).

<h3 id="wsl-sessions-in-claude-code-desktop">
  WSL-Sitzungen in Claude Code Desktop
</h3>

Unter Windows können [Claude Code Desktop Code-Sitzungen in einer WSL 2-Distribution ausführen](/docs/de/desktop-wsl). Der Claude Code-Prozess der Sitzung wird in der Distribution ausgeführt, daher werden verwaltete Einstellungen über den oben beschriebenen WSL-Erkennungspfad aufgelöst: Windows-spezifische Quellen erreichen ihn nicht, es sei denn, `wslInheritsWindowsSettings: true` wird bereitgestellt.

Auf Geräten, auf denen verwaltete Einstellungen vorhanden sind, sind Desktop-WSL-Sitzungen standardmäßig nicht verfügbar. Wenn Ihre Organisation diese aktivieren möchte, wenden Sie sich an Ihr Anthropic-Kontoteam. Wenn sie aktiviert sind:

* Stellen Sie `wslInheritsWindowsSettings: true` über die HKLM-Registry oder die `C:\Program Files\ClaudeCode`-Datei bereit, damit WSL-Sitzungen die gleiche Richtlinie wie Host-Sitzungen erben.
* Überprüfen Sie dies, indem Sie `/status` in einer WSL-Sitzung ausführen: Die Zeile `Setting sources` sollte `Enterprise managed settings` mit der Windows-Quelle anzeigen, die Sie bereitgestellt haben, `(HKLM)` oder `(file)`.

Prozesse in der WSL 2-Utility-VM sind für Windows-seitige Endpoint-Detection-Sensoren nicht sichtbar. Wenn Sie CrowdStrike Falcon verwenden, aktivieren Sie den Falcon-Sensor für Linux auf WSL 2 mit den zwei Ausschlüssen, die die WSL-Dokumentation von CrowdStrike erfordert, für den WSL-Prozess der virtuellen Maschine und das VM-Disk-Image, damit die Prozess- und Dateiaktivität in der Distribution beobachtbar ist. Claude Code's [OpenTelemetry-Tool-Ausführungs-Telemetrie](/docs/de/monitoring-usage) wird identisch für WSL- und native Sitzungen ausgegeben.

<h2 id="decide-what-to-enforce">
  Entscheiden Sie, was durchgesetzt werden soll
</h2>

Verwaltete Einstellungen können Tools sperren, Sandbox-Ausführung, MCP-Server und Plugin-Quellen einschränken und steuern, welche Hooks ausgeführt werden. Jede Zeile ist eine Kontrollfläche mit den Einstellungsschlüsseln, die sie antreiben.

| Kontrolle                                                                                | Was es tut                                                                                                                                                                                                                                                                                                       | Wichtige Einstellungen                                                                                                   |
| :--------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------- |
| [Berechtigungsregeln](/docs/de/permissions)                                                   | Bestimmte Tools und Befehle zulassen, fragen oder ablehnen                                                                                                                                                                                                                                                       | `permissions.allow`, `permissions.deny`                                                                                  |
| [Berechtigungssperre](/docs/de/permissions#managed-only-settings)                             | Nur verwaltete Berechtigungsregeln gelten; deaktivieren Sie `--dangerously-skip-permissions`                                                                                                                                                                                                                     | `allowManagedPermissionRulesOnly`, `permissions.disableBypassPermissionsMode`                                            |
| [Sandboxing](/docs/de/sandboxing)                                                             | Isolierung auf Betriebssystemebene des Dateisystems und Netzwerks mit Domain-Allowlists                                                                                                                                                                                                                          | `sandbox.enabled`, `sandbox.network.allowedDomains`                                                                      |
| [Verwaltete Richtlinie CLAUDE.md](/docs/de/memory#deploy-organization-wide-claude-md)         | Organisationsweite Anweisungen, die in jeder Sitzung geladen werden, können nicht ausgeschlossen werden                                                                                                                                                                                                          | Datei im verwalteten Richtlinienpfad                                                                                     |
| [MCP-Server-Kontrolle](/docs/de/managed-mcp)                                                  | Einschränken, welche MCP-Server Benutzer hinzufügen oder verbinden können, oder einen festen Satz bereitstellen                                                                                                                                                                                                  | `allowedMcpServers`, `deniedMcpServers`, `allowManagedMcpServersOnly` oder eine bereitgestellte `managed-mcp.json`-Datei |
| [Plugin-Marketplace-Kontrolle](/docs/de/plugin-marketplaces#managed-marketplace-restrictions) | Einschränken, von welchen Marketplace-Quellen Benutzer hinzufügen und installieren können, CLI-Flags ablehnen, die Plugins, Agents und MCP-Server für einen einzelnen Lauf sideloaden, und Allowlisten, welche Plugins von Marketplaces vorgeschlagen werden können                                              | `strictKnownMarketplaces`, `blockedMarketplaces`, `disableSideloadFlags`, `pluginSuggestionMarketplaces`                 |
| [Anpassungssperre](/docs/de/settings#strictpluginonlycustomization)                           | Blockieren Sie Skills, Agents, Hooks und MCP-Server aus Benutzer- und Projektquellen, damit sie nur aus Plugins oder verwalteten Einstellungen stammen können                                                                                                                                                    | `strictPluginOnlyCustomization`                                                                                          |
| [Hook-Einschränkungen](/docs/de/settings#hook-configuration)                                  | Nur verwaltete Hooks werden geladen; HTTP-Hook-URLs einschränken                                                                                                                                                                                                                                                 | `allowManagedHooksOnly`, `allowedHttpHookUrls`                                                                           |
| [Anmeldung erzwingen](/docs/de/settings#available-settings)                                   | Interaktive Anmeldung auf eine bestimmte Methode oder Anthropic-Organisation beschränken. Wenn festgelegt, werden Sitzungen, die durch `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` oder `apiKeyHelper` authentifiziert sind, beim Start blockiert; Cloud-Provider-Sitzungen sind nicht betroffen                 | `forceLoginMethod`, `forceLoginOrgUUID`                                                                                  |
| [Agent-Ansicht deaktivieren](/docs/de/agent-view#how-background-sessions-are-hosted)          | Schalten Sie `claude agents`, `--bg`, `/background` und den On-Demand-Supervisor aus                                                                                                                                                                                                                             | `disableAgentView`                                                                                                       |
| [Modelleinschränkungen](/docs/de/model-config#restrict-model-selection)                       | `availableModels` filtert, welche Modelle in der Auswahl angezeigt werden. Das Hinzufügen von `enforceAvailableModels` beschränkt auch das automatisch ausgewählte Standardmodell. Siehe [Oberflächenabdeckung](/docs/de/model-config#surface-coverage) für die Erreichbarkeit dieser Einstellung in CLI, Web und IDE | `availableModels`, `enforceAvailableModels`                                                                              |
| [Versionsuntergrenze](/docs/de/settings)                                                      | Verhindern Sie, dass Auto-Update unter ein organisationsweites Minimum installiert wird                                                                                                                                                                                                                          | `minimumVersion`                                                                                                         |
| [Erforderlicher Versionsbereich](/docs/de/settings)                                           | Weigern Sie sich, überhaupt zu starten, wenn die laufende Version außerhalb eines von der Organisation genehmigten Bereichs liegt. Stärker als `minimumVersion`, das nur Downgrades blockiert                                                                                                                    | `requiredMinimumVersion`, `requiredMaximumVersion`                                                                       |

Organisationen, deren Mitglieder sich über claude.ai oder die Anthropic API authentifizieren, können Modelle auch ohne Bereitstellung von Einstellungen steuern: [Organisationsmodelleinschränkungen](/docs/de/model-config#organization-model-restrictions) deaktivieren einzelne Modelle, ein [Organisationsstandardmodell](/docs/de/model-config#organization-default-model) legt fest, mit welchem Modell neue Sitzungen beginnen, und [Organisationsaufwandsgrenzen](/docs/de/model-config#organization-effort-limits) begrenzen Aufwandsstufen pro Rolle. Alle drei Kontrollen erfordern einen Claude Enterprise-Plan. Modelleinschränkungen und Aufwandsgrenzen werden serverseitig durchgesetzt; das Standardmodell ist ein Ausgangspunkt, den Benutzer ändern können, es sei denn, die Organisation erzwingt es. Die Durchsetzung ist für eine begrenzte Anzahl von Organisationen verfügbar; fragen Sie Ihr Anthropic-Kontoteam nach der Verfügbarkeit. Keine dieser Kontrollen erreichen Sitzungen auf Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry oder [Claude Platform on AWS](/docs/de/claude-platform-on-aws); verwenden Sie auf diesen Anbietern `availableModels` oben für Einschränkungen und den `model`-Schlüssel in verwalteten Einstellungen für einen Standard.

[Claude Code im Web](/docs/de/claude-code-on-the-web) hat seine eigene Admin-Oberfläche: Auf der Seite „Cloud-Umgebungen" in den Admin-Einstellungen erstellen Besitzer und Administratoren [organisationsweite gemeinsame Umgebungen](/docs/de/claude-code-on-the-web#organization-shared-environments), die die [Netzwerkzugriffsstufe](/docs/de/claude-code-on-the-web#network-access), Umgebungsvariablen und das Setup-Skript für Cloud-Sitzungen der Mitglieder festlegen, und wählen die Standardumgebung der Organisation.

Berechtigungsregeln und Sandboxing decken verschiedene Ebenen ab. Das Ablehnen von WebFetch blockiert Claudes Fetch-Tool, aber wenn Bash zulässig ist, können `curl` und `wget` immer noch jede URL erreichen. Sandboxing schließt diese Lücke mit einer auf Betriebssystemebene durchgesetzten Netzwerk-Domain-Allowlist.

Für das Bedrohungsmodell, das diese Kontrollen verteidigen, siehe [Sicherheit](/docs/de/security).

<h2 id="set-up-usage-visibility">
  Nutzungssichtbarkeit einrichten
</h2>

Wählen Sie die Überwachung basierend auf dem, was Sie melden müssen. Die Dashboards, APIs und Ausgabenkontrollen unterscheiden sich zwischen Claude for Teams oder Enterprise-Plänen und Claude Console-Organisationen. Überprüfen Sie daher die Spalte „Verfügbarkeit", bevor Sie Ihre Berichterstellung um eine Funktion planen.

| Fähigkeit                            | Was Sie erhalten                                                                                                                          | Verfügbarkeit                                                                                                                                                                                                                                                              | Wo Sie anfangen                                        |
| :----------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------- |
| Nutzungsüberwachung                  | OpenTelemetry-Export von Sitzungen, Tools und Tokens                                                                                      | Alle Anbieter                                                                                                                                                                                                                                                              | [Nutzungsüberwachung](/docs/de/monitoring-usage)            |
| Analytik-Dashboard                   | Adoptions- und Beitragskennzahlen mit einem Leaderboard bei Teams / Enterprise; Pro-Benutzer-Nutzungs- und Ausgabenkennzahlen bei Console | Teams / Enterprise unter [claude.ai/analytics](https://claude.ai/analytics/claude-code), Console unter [platform.claude.com/claude-code](https://platform.claude.com/claude-code)                                                                                          | [Analytik](/docs/de/analytics)                              |
| Programmgesteuerte Berichterstellung | Pro-Benutzer-Nutzungs- und Kostendaten über eine API                                                                                      | [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) für Enterprise, [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) für Console                                               | [Kosten](/docs/de/costs#manage-costs-for-your-organization) |
| Ausgabenkontrollen                   | Ausgabenlimits und Ratenlimits                                                                                                            | Admin-Einstellungen für Teams / Enterprise, Workspace-Limits für Console; auf Cloud-Plattformen von Drittanbietern Cloud-Budget-Kontrollen oder ein [Claude-Apps-Gateway](/docs/de/claude-apps-gateway) mit Pro-Benutzer-[Ausgabenlimits](/docs/de/claude-apps-gateway-spend-limits) | [Kosten](/docs/de/costs#manage-costs-for-your-organization) |

Bei Teams und Enterprise stammen Pro-Benutzer-Nutzungs- und Ausgabenzahlen aus dem [Ausgabenbericht](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) in den Analyseinstellungen Ihrer Organisation, nicht aus dem Analytik-Dashboard. Cloud-Anbieter stellen Ausgaben über AWS Cost Explorer, GCP Billing oder Azure Cost Management bereit. Für die Planung von Enterprise-Budgets über Claude Chat, Claude Code und Cowork hinweg siehe den [Claude Enterprise-Verbrauchsleitfaden](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide).

<h2 id="review-data-handling">
  Datenbehandlung überprüfen
</h2>

Bei Team-, Enterprise-, Claude API- und Cloud-Provider-Plänen trainiert Anthropic keine Modelle auf Ihrem Code oder Ihren Prompts. Ihr API-Anbieter bestimmt die Aufbewahrung und Compliance-Position.

| Thema                     | Was Sie wissen sollten                                                                                           | Wo Sie anfangen                                |
| :------------------------ | :--------------------------------------------------------------------------------------------------------------- | :--------------------------------------------- |
| Datennutzungsrichtlinie   | Was Anthropic erfasst, wie lange es aufbewahrt wird, was niemals zum Training verwendet wird                     | [Datennutzung](/docs/de/data-usage)                 |
| Zero Data Retention (ZDR) | Nichts wird nach Abschluss der Anfrage gespeichert. Verfügbar für qualifizierte Konten auf Claude für Enterprise | [Zero Data Retention](/docs/de/zero-data-retention) |
| Sicherheitsarchitektur    | Netzwerkmodell, Verschlüsselung, Authentifizierung, Audit-Trail                                                  | [Sicherheit](/docs/de/security)                     |

Wenn Sie Anfrage-Level-Audit-Protokollierung benötigen oder Datenverkehr nach Datensensibilität weiterleiten möchten, platzieren Sie ein Gateway zwischen Entwicklern und Ihrem Anbieter: Ein selbstgehostetes [Claude Apps Gateway](/docs/de/claude-apps-gateway) protokolliert ein Pro-Anfrage-Audit-Protokoll mit IdP-Identität, oder verwenden Sie ein anderes [LLM-Gateway](/docs/de/llm-gateway). Für behördliche Anforderungen und Zertifizierungen siehe [Rechtliche Angelegenheiten und Compliance](/docs/de/legal-and-compliance).

<h2 id="verify-and-onboard">
  Überprüfen und Onboarding
</h2>

Nach der Konfiguration verwalteter Einstellungen lassen Sie einen Entwickler `/status` in Claude Code ausführen. Auf der Registerkarte **Status** zeigt die Zeile `Setting sources` `Enterprise managed settings` gefolgt von der Quelle in Klammern, eine von `(remote)`, `(plist)`, `(HKLM)`, `(HKCU)` oder `(file)`. Siehe [Aktive Einstellungen überprüfen](/docs/de/settings#verify-active-settings).

Teilen Sie diese Ressourcen, um Entwicklern den Einstieg zu erleichtern:

* [Schnellstart](/docs/de/quickstart): Walkthrough der ersten Sitzung von der Installation bis zur Arbeit mit einem Projekt
* [Häufige Workflows](/docs/de/common-workflows): Muster für alltägliche Aufgaben wie Code-Review, Refactoring und Debugging
* [Claude 101](https://anthropic.skilljar.com/claude-101) und [Claude Code in Action](https://anthropic.skilljar.com/claude-code-in-action): Selbstgesteuerte Anthropic Academy-Kurse

Bei Anmeldeproblemen verweisen Sie Entwickler auf [Authentifizierungs-Fehlerbehebung](/docs/de/troubleshoot-install#login-and-authentication). Die häufigsten Lösungen sind:

* Führen Sie `/logout` und dann `/login` aus, um Konten zu wechseln
* Führen Sie `claude update` aus, wenn die Enterprise-Auth-Option fehlt
* Starten Sie das Terminal nach dem Update neu

Wenn ein Entwickler „You haven't been added to your organization yet" sieht, ist sein Sitzplatz nicht für Claude Code-Zugriff enthalten und muss in der Administratorkonsole aktualisiert werden.

<h2 id="next-steps">
  Nächste Schritte
</h2>

Mit ausgewähltem Anbieter und Liefermechanismus fahren Sie mit der detaillierten Konfiguration fort:

* [Server-verwaltete Einstellungen](/docs/de/server-managed-settings): Liefern Sie verwaltete Richtlinien über die Claude-Administratorkonsole
* [Einstellungsreferenz](/docs/de/settings): Jeder Einstellungsschlüssel, Dateispeicherort und Prioritätsregel
* [Monorepos und große Repositorys](/docs/de/large-codebases): Pro-Verzeichnis-Konfigurationsmuster für Organisationen, die in einem Monorepo bereitstellen
* [Amazon Bedrock](/docs/de/amazon-bedrock), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai), [Microsoft Foundry](/docs/de/microsoft-foundry): Anbieter-spezifische Bereitstellung
* [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide): SSO, SCIM, Sitzplatzverwaltung und Rollout-Playbook
