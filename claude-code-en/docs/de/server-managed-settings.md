> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Serververwaltete Einstellungen konfigurieren

> Konfigurieren Sie Claude Code zentral für Ihre Organisation durch serververwaltete Einstellungen, ohne dass eine Geräteverwaltungsinfrastruktur erforderlich ist.

Serververwaltete Einstellungen ermöglichen es Organisationsinhabern, Claude Code zentral über [**Admin-Einstellungen > Claude Code > Verwaltete Einstellungen**](https://claude.ai/admin-settings/claude-code) in der claude.ai-Konsole zu konfigurieren. Claude Code-Clients rufen diese Einstellungen automatisch ab, wenn sich Benutzer mit geeigneten Anmeldedaten auf einer Plattform authentifizieren, auf der die serververwaltete Bereitstellung unterstützt wird. Siehe [Plattformverfügbarkeit](#platform-availability) für die Anmeldedaten und Plattformen, die sich qualifizieren.

<Note>
  Serververwaltete Einstellungen sind für [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=server_settings_teams#team-&-enterprise) und [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=server_settings_enterprise) Kunden verfügbar.
</Note>

<h2 id="requirements">
  Anforderungen
</h2>

Um serververwaltete Einstellungen zu verwenden, benötigen Sie:

* Claude for Teams oder Claude for Enterprise Plan
* Die Rolle „Owner" oder „Primary Owner" in Ihrer Claude-Organisation, um die Konfiguration anzuzeigen und zu bearbeiten
* Netzwerkzugriff auf `api.anthropic.com`

<h2 id="choose-between-server-managed-and-endpoint-managed-settings">
  Wählen Sie zwischen serververwalteten und endpunktverwalteten Einstellungen
</h2>

Claude Code unterstützt zwei Ansätze für zentralisierte Konfiguration. Serververwaltete Einstellungen liefern Konfiguration von Anthropics Servern. [Endpunktverwaltete Einstellungen](/docs/de/managed-settings#delivery-mechanisms) werden direkt auf Geräten über native Betriebssystemrichtlinien (macOS verwaltete Einstellungen, Windows-Registrierung) oder verwaltete Einstellungsdateien bereitgestellt.

| Ansatz                                                                           | Am besten geeignet für                                              | Sicherheitsmodell                                                                                                                                  |
| :------------------------------------------------------------------------------- | :------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Serververwaltete Einstellungen**                                               | Organisationen ohne MDM oder Benutzer auf nicht verwalteten Geräten | Einstellungen, die Claude Code beim Start von Anthropics Servern abruft und während der Sitzung stündlich aktualisiert                             |
| **[Endpunktverwaltete Einstellungen](/docs/de/managed-settings#delivery-mechanisms)** | Organisationen mit MDM oder Endpunktverwaltung                      | Einstellungen, die auf Geräten über MDM-Konfigurationsprofile, Registrierungsrichtlinien oder verwaltete Einstellungsdateien bereitgestellt werden |

Wenn Ihre Geräte in einer MDM- oder Endpunktverwaltungslösung registriert sind, bieten endpunktverwaltete Einstellungen stärkere Sicherheitsgarantien, da die Einstellungsdatei auf Betriebssystemebene vor Benutzermodifikationen geschützt werden kann. Endpunktverwaltete Einstellungen erreichen [Cloud-Sitzungen](/docs/de/model-config#surface-coverage) in von Anthropic gehosteten Umgebungen nicht, daher sollten Organisationen, deren Entwickler Cloud-Sitzungen ausführen, auch serververwaltete Einstellungen konfigurieren. Sitzungen in einer [selbstgehosteten Umgebung](/docs/de/self-hosted-environments) lesen auch die verwaltete Einstellungsdatei im Runner-Image. Die [Einstellungspriorität](#settings-precedence) unten gibt an, wann diese Datei gilt.

<h2 id="configure-server-managed-settings">
  Serververwaltete Einstellungen konfigurieren
</h2>

<Steps>
  <Step title="Öffnen Sie die Admin-Konsole">
    Navigieren Sie in der claude.ai-Konsole zu [**Admin-Einstellungen > Claude Code > Verwaltete Einstellungen**](https://claude.ai/admin-settings/claude-code).

    Wenn der Link Sie stattdessen zu einer anderen Admin-Einstellungen-Seite umleitet, anstatt zur Claude Code-Seite, verfügt Ihr Konto nicht über die erforderliche Rolle. Admin und andere Nicht-Eigentümer-Rollen können verwaltete Einstellungen nicht anzeigen oder bearbeiten. Bitten Sie daher einen Eigentümer oder Primären Eigentümer in Ihrer Organisation, die Änderung vorzunehmen. Siehe [Zugriffskontrolle](#access-control).
  </Step>

  <Step title="Definieren Sie Ihre Einstellungen">
    Fügen Sie Ihre Konfiguration als JSON hinzu. Alle [in `settings.json` verfügbaren Einstellungen](/docs/de/settings-reference#all-settings) werden unterstützt, mit Ausnahme derjenigen, die auf die Bereitstellung auf OS-Ebene beschränkt sind. Siehe [Aktuelle Einschränkungen](#current-limitations) für diese kurze Liste. Dies umfasst [hooks](/docs/de/hooks), [Umgebungsvariablen](/docs/de/env-vars) und [nur verwaltete Einstellungen](/docs/de/managed-settings#managed-only-settings) wie `allowManagedPermissionRulesOnly`.

    Dieses Beispiel erzwingt eine Berechtigungsverweigerungsliste, verhindert, dass Benutzer Berechtigungen umgehen, und beschränkt Berechtigungsregeln auf diejenigen, die in verwalteten Einstellungen definiert sind. Die Regel `Bash(curl *)` entspricht `curl` [wie Claude sie schreibt](/docs/de/permissions#bash-rule-limits), nicht `/usr/bin/curl` oder `sh -c 'curl …'`. Für die Netzwerkerzwingung, die nicht vom Befehlstext abhängt, fügen Sie einen [`sandbox`-Block mit `allowManagedDomainsOnly`](/docs/de/sandboxing#configure-the-sandbox-for-your-organization) hinzu.

    ```json theme={null}
    {
      "permissions": {
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true
    }
    ```

    Hooks verwenden das gleiche Format wie in `settings.json`.

    Dieses Beispiel führt ein Audit-Skript nach jeder Dateibearbeitung in der gesamten Organisation aus:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [
              { "type": "command", "command": "/usr/local/bin/audit-edit.sh" }
            ]
          }
        ]
      }
    }
    ```

    Da Hooks Shell-Befehle ausführen, sehen Benutzer in interaktiven Sitzungen einen [Sicherheitsgenehmigungsdialog](#security-approval-dialogs), bevor Claude Code sie anwendet.

    Um den [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) Klassifizierer zu konfigurieren, damit er weiß, welche Repos, Buckets und Domains Ihre Organisation vertraut, stellen Sie einen `autoMode`-Block auf die gleiche Weise bereit. Siehe [Auto-Modus konfigurieren](/docs/de/auto-mode-config), um zu erfahren, wie die `autoMode`-Einträge beeinflussen, was der Klassifizierer blockiert, und wichtige Warnungen zu den Feldern `environment`, `allow`, `soft_deny` und `hard_deny`.
  </Step>

  <Step title="Speichern und bereitstellen">
    Speichern Sie Ihre Änderungen. Claude Code-Clients erhalten die aktualisierten Einstellungen beim nächsten Start oder während des stündlichen Abrufzyklus.
  </Step>
</Steps>

<h3 id="verify-settings-delivery">
  Einstellungsbereitstellung überprüfen
</h3>

Um zu bestätigen, dass Einstellungen angewendet werden, bitten Sie einen Benutzer, Claude Code neu zu starten. Wenn die Konfiguration Einstellungen enthält, die den [Sicherheitsgenehmigungsdialog](#security-approval-dialogs) auslösen, sieht der Benutzer beim nächsten Abruf der Einstellungen durch Claude Code eine Eingabeaufforderung, die die verwalteten Einstellungen beschreibt: beim nächsten Start oder innerhalb einer Stunde in einer laufenden interaktiven Sitzung. Sie können auch überprüfen, dass verwaltete Berechtigungsregeln aktiv sind, indem Sie einen Benutzer `/permissions` ausführen lassen, um seine geltenden Berechtigungsregeln anzuzeigen.

Um das Abrufergebnis auf einem bestimmten Computer zu überprüfen, lassen Sie den Benutzer `claude doctor` ausführen und lesen Sie die Zeile `Managed settings (remote)`. Erfordert Claude Code v2.1.248 oder später. Die Zeile meldet eines von vier Ergebnissen:

* Die bereitgestellten Einstellungen wurden geladen
* Ihre Organisation hat keine serververwalteten Einstellungen konfiguriert
* Der Abruf ist fehlgeschlagen, mit der Ursache und ob eine zwischengespeicherte Richtlinie noch gilt
* Claude Code hat den Abruf übersprungen, mit dem Grund. Siehe [Plattformverfügbarkeit](#platform-availability) für die Anbieter und Konfigurationen, die ihn überspringen

Während der Abruf noch läuft, meldet die Zeile das stattdessen.

In einer laufenden Sitzung zeigt `/status` die gleiche Zeile nach einem fehlgeschlagenen Abruf an, und für einige übersprungene Abrufe, wie z. B. eine Variable eines Drittanbieter-Anbieters oder eine benutzerdefinierte `ANTHROPIC_BASE_URL`, die in der Shell des Benutzers exportiert wird.

<h3 id="access-control">
  Zugriffskontrolle
</h3>

Die folgenden Rollen können serververwaltete Einstellungen verwalten:

* **Primärer Eigentümer**
* **Eigentümer**

Beschränken Sie den Zugriff auf vertrauenswürdiges Personal, da Einstellungsänderungen für alle Benutzer in der Organisation gelten.

<h3 id="managed-only-settings">
  Nur verwaltete Einstellungen
</h3>

Die meisten [Einstellungsschlüssel](/docs/de/settings-reference#all-settings) funktionieren in jedem Bereich. Eine Handvoll Schlüssel werden nur aus verwalteten Einstellungen gelesen und haben keine Auswirkung, wenn sie in Benutzer- oder Projekteinstellungsdateien platziert werden. Siehe [nur verwaltete Einstellungen](/docs/de/managed-settings#managed-only-settings) für die Berechtigung und Plugin-Steuerelemente, oder lesen Sie die Spalte „Bereich" des [Alle Einstellungen](/docs/de/settings-reference#all-settings) Index für den vollständigen Satz.

<h3 id="current-limitations">
  Aktuelle Einschränkungen
</h3>

Serververwaltete Einstellungen haben die folgenden Einschränkungen:

* Einstellungen gelten einheitlich für alle Benutzer in der Organisation. Konfigurationen pro Gruppe werden noch nicht unterstützt.
* Eine [`managed-mcp.json`](/docs/de/managed-mcp) Datei kann nicht über serververwaltete Einstellungen verteilt werden. Stellen Sie stattdessen die Richtlinienschlüssel `allowedMcpServers` und `deniedMcpServers` dort bereit. Ab Claude Code v2.1.259 können Sie auch Remote-Server mit [`managedMcpServers`](/docs/de/managed-mcp#provide-servers-through-managed-settings) bereitstellen, das nur `http`- und `sse`-Server akzeptiert und nicht die exklusive Kontrolle übernimmt, wie die Datei es tut.

  Claude Code liest eine `managed-mcp.json`, die in seinem [Systempfad](/docs/de/managed-mcp#exclusive-control-with-managed-mcp-json) bereitgestellt wird, separat von der Ebene der verwalteten Einstellungen, sodass die Datei weiterhin gilt, wenn serververwaltete Einstellungen in Kraft sind.
* Einstellungen, die auf OS-Ebene-Richtlinienquellen beschränkt sind, wie `policyHelper` und `wslInheritsWindowsSettings`, werden nicht berücksichtigt. Stellen Sie sie stattdessen über MDM oder eine System-Datei `managed-settings.json` bereit. Ein `policyHelper`, der auf diese Weise bereitgestellt wird, wird nur ausgeführt, wenn seine Quelle die unter [Vorrang innerhalb der verwalteten Ebene](/docs/de/managed-settings#precedence-within-the-managed-tier) ausgewählte ist.

<h2 id="settings-delivery">
  Einstellungsbereitstellung
</h2>

<h3 id="settings-precedence">
  Einstellungspriorität
</h3>

Serververwaltete Einstellungen und [endpunktverwaltete Einstellungen](/docs/de/managed-settings#delivery-mechanisms) nehmen beide die höchste Ebene in der Claude Code [Einstellungshierarchie](/docs/de/settings#settings-precedence) ein. Keine andere Einstellungsebene kann sie überschreiben, einschließlich Befehlszeilenargumenten, abgesehen von den [Ausnahmen zur Priorität verwalteter Einstellungen](/docs/de/settings#exceptions-to-managed-settings-precedence).

Innerhalb der verwalteten Ebene verwendet Claude Code standardmäßig die erste Quelle, die mindestens einen Richtlinienschlüssel liefert, wobei zuerst serververwaltete Einstellungen überprüft werden und dann endpunktverwaltete Einstellungen, abgesehen von der [Ausnahmeregelung für Schlüssel, die als nächstes behandelt wird](#per-key-exceptions-across-managed-sources). [Wie Claude Code verwaltete Quellen kombiniert](/docs/de/managed-settings#precedence-within-the-managed-tier) enthält die vollständige Rangfolge, die Ausnahmeregelung für die Steuerschlüssel und die Opt-in-Option, die für jede Quelle gilt.

Wenn die ausgewählte Quelle eine MDM-Richtlinie oder verwaltete Einstellungsdatei ist, deren [`policyHelper`](/docs/de/settings-reference#policyhelper) verwaltete Einstellungen liefert, ersetzt die Ausgabe des Helpers diese Quelle als einzige verwaltete Konfiguration für den Lauf. Claude Code konsultiert einen `policyHelper`, der in MDM oder dateigestützten Einstellungen konfiguriert ist, nicht, während serververwaltete Einstellungen einen Richtlinienschlüssel liefern.

Wenn ein späterer Abruf feststellt, dass die serververwalteten Einstellungen entfernt wurden, führt Claude Code diesen Helper sofort aus, anstatt beim nächsten Start. Der [`policyHelper`](/docs/de/settings-reference#policyhelper) Eintrag behandelt, was passiert, wenn dieser Lauf fehlschlägt.

Wenn Sie Ihre serververwaltete Konfiguration in der Admin-Konsole mit der Absicht löschen, auf eine endpunktverwaltete plist oder Registrierungsrichtlinie zurückzugreifen, beachten Sie, dass [zwischengespeicherte Einstellungen](#fetch-and-caching-behavior) auf Client-Maschinen bestehen bleiben, bis der nächste erfolgreiche Abruf erfolgt, und die Schlüssel, die [nur beim nächsten Start gelten](#fetch-and-caching-behavior), wie `model`, bleiben wirksam, bis jeder Client neu gestartet wird. Führen Sie `/status` aus, um zu sehen, welche verwaltete Quelle aktiv ist.

<h3 id="per-key-exceptions-across-managed-sources">
  Ausnahmen pro Schlüssel über verwaltete Quellen hinweg
</h3>

Drei Arten von Schlüsseln sind Ausnahmen von der Nicht-Zusammenführungsregel:

* **Quellübergreifende Sperr-Schlüssel**: ein kleiner Satz von Schlüsseln, wie die Sandbox-Zulassungslisten-Sperren, [aufgelistet auf der Seite für verwaltete Einstellungen](/docs/de/managed-settings#precedence-within-the-managed-tier). Claude Code berücksichtigt sie, wenn eine beliebige administratorgesteuerte verwaltete Quelle sie setzt; die benutzerbare HKCU-Registrierungsebene ist ausgeschlossen.

  Wenn ein [`policyHelper`](/docs/de/settings-reference#policyhelper) verwaltete Einstellungen liefert, ist seine Ausgabe die einzige Quelle, die diese Überprüfungen lesen, abgesehen von [`forceRemoteSettingsRefresh`](/docs/de/settings-reference#forceremotesettingsrefresh), das Claude Code beim Start direkt aus den administratorgesteuerten Quellen liest.
* **Der `env`-Block**: abgesehen von der Telemetrie-Einheit und Routing-Variablen, die mit einem Anmeldedatenschlüssel gekoppelt sind, beide unten behandelt, wird er pro Schlüssel über die administratorgesteuerten Quellen hinweg zusammengeführt. Für jede Umgebungsvariable gewinnt die höchstpriorisierte Quelle, die sie definiert, und niedrigere administratorgesteuerte Quellen füllen Variablen aus, die die höheren Quellen nicht setzen. Ein endpunktverwalteter `env`-Eintrag gilt daher, wenn die serververwaltete Konfiguration diese Variable nicht setzt, oder während ein zwischengespeicherter Serverwert dafür [zurückgehalten wird, bis der Server ihn bestätigt](#fetch-and-caching-behavior). Erfordert Claude Code v2.1.223 oder später. Vor v2.1.223 wendet Claude Code nur den `env`-Block der ausgewählten Quelle an.
  * **Telemetrie-Einheit**: die `OTEL_EXPORTER_OTLP_*` Exporter-Schlüssel, die `OTEL_LOG_*` Inhaltserfassungs-Umschalter, `OTEL_LOGS_EXPORTER` und die Beta-Tracing-Variablen `ENABLE_BETA_TRACING_DETAILED` und `BETA_TRACING_ENDPOINT` folgen der höchsten Quelle, die eine von ihnen als Einheit setzt. Eine Quelle, die den `otelHeadersHelper` Anmeldedatenschlüssel liefert, beansprucht die Einheit auch, landet diese Variablen aber nur, wenn sie die ausgewählte Quelle ist: Eine Quelle, die nicht ausgewählt ist, aber den Schlüssel liefert, trägt keine von ihnen bei und blockiert immer noch niedrigere Quellen davon, sie auszufüllen. In jedem Fall kann ein Exporter-Endpunkt von einer Quelle niemals mit Anmeldedaten von einer anderen gekoppelt werden.
  * **Anmeldedaten-gekoppeltes Routing**: eine Quelle, die Routing-Variablen mit einem Anmeldedatenschlüssel nur für die ausgewählte Quelle, wie `apiKeyHelper` oder `otelHeadersHelper`, koppelt, trägt diese Routing-Variablen nur bei, wenn sie den Slot gewinnt.
* **Gateway-Anmeldeschlüssel**: Claude Code liest [`forceLoginGatewayUrl`](/docs/de/settings-reference#forcelogingatewayurl), [`gatewayInternalNetworks`](/docs/de/settings-reference#gatewayinternalnetworks) oder den `"gateway"` Wert von [`forceLoginMethod`](/docs/de/settings-reference#forceloginmethod) niemals aus serververwalteten Einstellungen, daher liefert ein Wert dort weder eine Gateway-Anmeldung noch verbirgt eine in einer MDM-Richtlinie oder verwalteten Einstellungsdatei gesetzte. Der [`managedSourcesBehavior` Eintrag](/docs/de/settings-reference#managedsourcesbehavior) sagt, welche administratorgesteuerte Quelle auf der Maschine sie liefert.

<h3 id="fetch-and-caching-behavior">
  Abruf- und Caching-Verhalten
</h3>

Claude Code ruft Einstellungen beim Start von Anthropics Servern ab und fragt stündlich während aktiver Sitzungen nach Updates ab.

Ein Client, der sich über ein [Claude-Apps-Gateway](#platform-availability) angemeldet hat, ruft seine Einstellungen vom Gateway ab und wartet auf diesen Abruf, bevor die Sitzung beginnt, daher gilt der Abruf in den folgenden Listen nicht dafür. [Erzwingen Sie einen Fail-Closed-Start](#enforce-fail-closed-startup) behandelt, was passiert, wenn dieser Abruf fehlschlägt.

**Erster Start ohne zwischengespeicherte Einstellungen:**

* Wenn sich ein Entwickler beim Start anmeldet, z. B. beim ersten Lauf oder nach `/logout`, wartet Claude Code bis zu fünf Sekunden auf den Abruf, bevor die Sitzung geöffnet wird. Wenn die Richtlinie rechtzeitig ankommt, erzwingt Claude Code sie vom ersten Bildschirm an und zeigt Ihre [`companyAnnouncements`](/docs/de/settings-reference#companyannouncements) darauf an. Wenn die Payload [Sicherheitsgenehmigung](#security-approval-dialogs) benötigt, beendet Claude Code das Warten und wendet die Payload an, sobald der Entwickler sie genehmigt
* Bei jedem anderen Start und wenn diese fünf-Sekunden-Wartezeit abläuft, öffnet Claude Code die Sitzung, während der Abruf fortgesetzt wird, sodass ein kurzes Fenster verstreicht, bevor die Einstellungen geladen werden und die Einschränkungen wirksam werden
* Wenn der Abruf fehlschlägt, wird Claude Code ohne serververwaltete Einstellungen fortgesetzt und warnt in interaktiven Sitzungen, dass keine Remote-Richtlinie gilt; endpunktverwaltete Einstellungen gelten immer noch. Wenn eine verwaltete Quelle [`forceRemoteSettingsRefresh`](#enforce-fail-closed-startup) setzt, wird Claude Code stattdessen beendet

**Nachfolgende Starts mit zwischengespeicherten Einstellungen:**

* Zwischengespeicherte Einstellungen werden beim Start sofort angewendet, außer für die zwischengespeicherten `modelPricing` und `managedMcpServers` Werte und die Umgebungsvariablen, die Claude Code zurückhält, bis der Server die Payload bestätigt
* Ein zwischengespeichertes [`modelPricing`](/docs/de/settings-reference#modelpricing) wird nicht angewendet, bis der Abruf der Sitzung die Payload bestätigt. Bis dahin sind die Kostenzahlen, die Entwickler in `/usage` und in der Statuszeile sehen, zum Listenpreis
* Ein zwischengespeicherter [`managedMcpServers`](/docs/de/settings-reference#managedmcpservers) Block wird nicht angewendet, bis der Abruf der Sitzung die Payload bestätigt. Claude Code wartet bis zu 30 Sekunden auf diesen Abruf, bevor MCP-Server verbunden werden. Wenn der Abruf fehlschlägt oder das Zeitlimit überschritten wird, startet die Sitzung ohne die Server der Organisation, `/status` sagt dies, und sie verbinden sich, sobald ein späterer Abruf sie bestätigt. Siehe [Wenn bereitgestellte Server verbunden werden](/docs/de/managed-mcp#when-provided-servers-connect) für das vollständige Verhalten, einschließlich des ersten Starts. Erfordert Claude Code v2.1.259 oder später
* Claude Code ruft frische Einstellungen im Hintergrund ab
* Zwischengespeicherte Einstellungen bleiben bei Netzwerkfehlern erhalten. Wenn der Startup-Abruf fehlschlägt, warnt Claude Code in interaktiven Sitzungen, dass die zwischengespeicherte Richtlinie wirksam ist
* Bis ein Abruf erfolgreich ist, bleiben die beim Start zurückgehaltenen Werte zurückgehalten

Claude Code hält mehrere Kategorien von Variablen im zwischengespeicherten `env`-Block zurück, bis der Server die Payload für die Sitzung bestätigt. Dies verhindert, dass ein zwischengespeicherter Proxy-, Zertifizierungsstellen-, Endpunkt- oder Anmeldedatenwert den Einstellungsabruf umleitet, abfängt oder erneut authentifiziert, der die Payload bestätigt. Die Härtung gilt nur für den vom Server abgerufenen Einstellungs-Cache: [endpunktverwaltete Einstellungen](/docs/de/managed-settings#delivery-mechanisms), die über MDM oder `managed-settings.json` bereitgestellt werden, sind nicht betroffen. Das Zurückhalten erfordert Claude Code v2.1.198 oder später; vor v2.1.198 wird der gesamte zwischengespeicherte `env`-Block beim Start angewendet. Die zurückhaltenden Kategorien umfassen:

* Proxy- und TLS-Konfiguration, wie `HTTPS_PROXY`, `NODE_EXTRA_CA_CERTS` und die mTLS-Client-Zertifikatvariablen `CLAUDE_CODE_CLIENT_CERT` und `CLAUDE_CODE_CLIENT_KEY`
* API-Routing und Anbieterauswahl, einschließlich `ANTHROPIC_BASE_URL`, der Anbieterauswahlvariablen wie `CLAUDE_CODE_USE_BEDROCK` und `CLAUDE_CODE_USE_VERTEX` und der Anbieter-Endpunkt-URLs wie `ANTHROPIC_BEDROCK_BASE_URL`
* Authentifizierungsanmeldedaten, wie `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` und `CLAUDE_CODE_OAUTH_TOKEN`
* Der Konfigurationsverzeichnis-Selektor `CLAUDE_CONFIG_DIR`
* Anmeldedaten-Quellen- und Konfigurationsverzeichnis-Selektoren, in Claude Code v2.1.223 oder später: die Workload Identity Federation-Variablen wie `ANTHROPIC_FEDERATION_RULE_ID` und `ANTHROPIC_IDENTITY_TOKEN`, die Profil- und Konfigurationsverzeichnis-Selektoren `ANTHROPIC_PROFILE` und `ANTHROPIC_CONFIG_DIR` und die Betriebssystem-Verzeichnisvariablen `HOME`, `XDG_CONFIG_HOME`, `APPDATA` und `USERPROFILE`

Claude Code liest die Workload Identity Federation-Variablen und die `ANTHROPIC_PROFILE` und `ANTHROPIC_CONFIG_DIR` Selektoren nur beim Start, daher wechselt ein vom Server bereitgestellter Wert dafür die Anmeldedatenquelle der Sitzung nicht, auch nachdem der Abruf erfolgreich ist. Um diese Selektoren auf Claude Code v2.1.223 oder später bereitzustellen, verwenden Sie [endpunktverwaltete Einstellungen](/docs/de/managed-settings#delivery-mechanisms) wie MDM oder `managed-settings.json`. Für `CLAUDE_CONFIG_DIR` und die Betriebssystem-Verzeichnisvariablen ist das Zurückhalten selbst der Schutz: Der zwischengespeicherte Wert bleibt aus der Umgebung, bis der Server die Payload bestätigt.

Jeder andere Schlüssel im zwischengespeicherten `env`-Block wird beim Start angewendet. Sobald der Server die Payload bestätigt und Sie sie genehmigen, wenn sie [Sicherheitsgenehmigung](#security-approval-dialogs) benötigt, werden die zurückgehaltenen Variablen für den Rest der Sitzung angewendet.

Wenn Ihre Organisation einen Proxy benötigt, um `api.anthropic.com` zu erreichen, betrifft das Zurückhalten nur den vom Server bereitgestellten `env`-Block selbst: Ein Proxy, der in einem [endpunktverwalteten](/docs/de/managed-settings#delivery-mechanisms) `env`-Block über MDM oder `managed-settings.json`, in der Shell-Umgebung oder in [Benutzereinstellungen](/docs/de/settings#where-settings-live) gesetzt ist, erreicht den Einstellungsabruf. Die endpunktverwaltete Quelle erfordert Claude Code v2.1.223 oder später: Der zwischengespeicherte serververwaltete Proxy-Wert wird zurückgehalten, bis der Abruf ihn bestätigt, daher füllt der endpunktverwaltete Wert pro Schlüssel aus und erreicht den Abruf selbst. Vor v2.1.223 verwenden Sie die Shell-Umgebung oder Benutzereinstellungen, damit der Proxy neben einer zwischengespeicherten Server-Payload gilt. Der erste Start hat keinen Cache, daher ist eine endpunktverwaltete Quelle, die Shell-Umgebung oder Benutzereinstellungen immer noch für den anfänglichen Abruf erforderlich.

Claude Code wendet die meisten Einstellungsaktualisierungen auf laufende Sitzungen ohne Neustart an. Einige Aktualisierungen gelten nur beim nächsten Start, einschließlich OpenTelemetry-Exporter-Konfiguration, des `model`-Schlüssels und der Entfernung einer Variablen aus dem `env`-Block.

<h3 id="invalid-entries-in-delivered-settings">
  Ungültige Einträge in bereitgestellten Einstellungen
</h3>

Wenn ein Teil einer Payload die Schemavalidierung nicht besteht, zeigt Claude Code einen Validierungsfehler an und wendet jede verbleibende gültige Einstellung an; [Ungültige Einträge in verwalteten Einstellungen](/docs/de/managed-settings#invalid-entries-in-managed-settings) sagt, was es entfernt und welche Schlüssel auf einen strengeren Wert zurückfallen. Erfordert Claude Code v2.1.169 oder später.

Die serververwaltete Bereitstellung fügt diese Verhaltensweisen hinzu:

* Der Cache unter `~/.claude/remote-settings.json` speichert die gerettete Payload mit entfernten ungültigen Einträgen, abgesehen von ungültigen `cleanupPeriodDays` und `desktopSessionCleanupPeriodDays` Werten, die in der zwischengespeicherten Kopie bleiben und niemals angewendet werden.
* Wenn kein Feld in der Payload gerettet werden kann und die Payload nicht nur diese Aufbewahrungsschlüssel ist, lehnt Claude Code die Payload ab, behält die zuletzt akzeptierten zwischengespeicherten Einstellungen bei und schreibt `Remote settings: Settings validation failed - no fields could be salvaged` in das Debug-Protokoll. Mit `forceRemoteSettingsRefresh` gesetzt, wird die CLI stattdessen beendet.
* Der [Sicherheitsgenehmigungsdialog](#security-approval-dialogs) bewertet die gerettete Payload, daher wird ein entfernter ungültiger Eintrag niemals zur Genehmigung präsentiert und niemals ausgeführt.

Um Bereitstellungsprobleme zu debuggen, führen Sie `claude --debug-file <path>` aus und suchen Sie im Protokoll nach `Remote settings`. Validieren Sie eine Payload-Änderung mit `claude doctor` auf einem Test-Computer, bevor Sie sie in der Organisation bereitstellen.

<h3 id="enforce-fail-closed-startup">
  Erzwingen Sie einen Fail-Closed-Start
</h3>

Standardmäßig wird die CLI, wenn der Abruf der Remote-Einstellungen beim Start fehlschlägt, mit den beim letzten erfolgreichen Abruf zwischengespeicherten Einstellungen fortgesetzt, außer für die [Werte, die Claude Code zurückhält](#fetch-and-caching-behavior), bis ein Abruf gelingt. Auf einer Maschine, die sie noch nie abgerufen hat, wird die CLI ohne serververwaltete Einstellungen fortgesetzt und wendet immer noch alle [endpunktverwalteten Einstellungen](/docs/de/managed-settings#delivery-mechanisms) auf dem Gerät an.

Um Clients daran zu hindern, mit zwischengespeicherten oder fehlenden serververwalteten Einstellungen zu starten, setzen Sie `forceRemoteSettingsRefresh: true` in Ihren verwalteten Einstellungen.

Clients, die sich über ein [Claude-Apps-Gateway](#platform-availability) angemeldet haben, warten auf den Startup-Abruf, unabhängig davon, ob Sie diese Einstellung setzen oder nicht, und handhaben einen fehlgeschlagenen Abruf wie folgt:

* Wenn das Gateway einen beaufsichtigten interaktiven Start mit einem `401` beantwortet und diese Einstellung deaktiviert ist, hat das Gateway diese Anmeldung beendet. Claude Code druckt [`Cloud gateway session expired — run /login to reconnect.`](/docs/de/errors#cloud-gateway-session-expired) und öffnet die Sitzung vom Gateway abgemeldet, bis der Benutzer `/login` ausführt.
* Wenn der Abruf auf andere Weise fehlschlägt, oder bei jeder anderen Art von Start außer einem `claude auth` Unterbefehl, wird der Client mit einem Fehler beendet.

Wenn diese Einstellung in einer Sitzung aktiv ist, die serververwaltete Einstellungen abruft, blockiert die CLI beim Start, bis Remote-Einstellungen neu abgerufen werden. Wenn der Abruf fehlschlägt, wird die CLI beendet, anstatt ohne die Richtlinie fortzufahren. Diese Einstellung perpetuiert sich selbst: Sobald sie vom Server bereitgestellt wird, wird sie auch lokal zwischengespeichert, sodass nachfolgende Starts das gleiche Verhalten erzwingen, auch bevor der erste erfolgreiche Abruf einer neuen Sitzung erfolgt. Eine Sitzung, die [serververwaltete Einstellungen nicht abruft](#platform-availability), startet ohne zu warten.

Um dies zu aktivieren, fügen Sie den Schlüssel zu Ihrer verwalteten Einstellungskonfiguration hinzu:

```json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

Sie können diesen Schlüssel auch in einem [endpunktverwalteten](/docs/de/managed-settings#delivery-mechanisms) MDM-Profil oder einer System-Datei `managed-settings.json` setzen, um Fail-Closed-Verhalten beim ersten Start durchzusetzen, bevor eine Server-Payload bereitgestellt wurde. Dieses Flag ist eine Ausnahme von der [Prioritätsregel](#settings-precedence) oben: Claude Code berücksichtigt es, wenn es in einer beliebigen administratorgesteuerten verwalteten Quelle gesetzt ist, auch wenn eine zwischengespeicherte serververwaltete Payload vorhanden ist, daher wird ein MDM-bereitgestellter Wert nicht ignoriert, wenn serververwaltete Einstellungen vorhanden sind.

Wenn ein [`policyHelper`](/docs/de/settings-reference#policyhelper) verwaltete Einstellungen liefert, ersetzt seine Ausgabe jede andere verwaltete Quelle für die Schlüssel, die Claude Code nach dem Start liest. Für die Quellen, aus denen Claude Code diesen Schlüssel liest, siehe [seinen Einstellungseintrag](/docs/de/settings-reference#forceremotesettingsrefresh). Der `policyHelper` Eintrag sagt, welche Quellen Claude Code den Helper aus liest und wann er ausgeführt wird.

Der Einstellungsabruf sendet auch einen `Cache-Control: no-cache` Header, sodass zwischengeschaltete HTTP-Proxys keine veraltete Antwort bereitstellen.

Bevor Sie diese Einstellung aktivieren, stellen Sie sicher, dass Ihre Netzwerkrichtlinien die Konnektivität zu `api.anthropic.com` ermöglichen. Wenn dieser Endpunkt nicht erreichbar ist, wird die CLI beim Start beendet und Benutzer können Claude Code nicht starten.

Die `claude auth` Unterbefehle wie `claude auth login` sind von dieser Überprüfung und vom Gateway-Startup-Exit ausgenommen, daher können Benutzer sich erneut authentifizieren, wenn abgelaufene Anmeldedaten der Grund für den fehlgeschlagenen Einstellungsabruf sind.

<h3 id="security-approval-dialogs">
  Sicherheitsgenehmigungsdialoge
</h3>

Bestimmte Einstellungen, die Sicherheitsrisiken darstellen könnten, erfordern explizite Benutzergenehmigung, bevor Claude Code sie in einer interaktiven Sitzung anwendet:

* **Shell-Befehlseinstellungen**: Einstellungen, die Shell-Befehle ausführen, wie `apiKeyHelper`, `statusLine` und `otelHeadersHelper`
* **Sandbox-Binär-Einstellungen**: `sandbox.bwrapPath`, `sandbox.socatPath` und `sandbox.ripgrep`. Jede dieser Einstellungen verweist auf eine ausführbare Datei, und Claude Code führt diese ausführbare Datei aus
* **Sandbox-Netzwerk- und Isolationseinstellungen**: [Sandbox](/docs/de/sandboxing) Einstellungen, die dem Sandbox-Proxy ermöglichen, Datenverkehr zu lesen, umzuleiten oder zu authentifizieren, oder die die Isolation der Sandbox schwächen: `sandbox.network.tlsTerminate`, `sandbox.network.httpProxyPort`, `sandbox.network.socksProxyPort`, `sandbox.credentials`, `sandbox.allowAppleEvents`, `sandbox.enableWeakerNestedSandbox`, `sandbox.enableWeakerNetworkIsolation`, `sandbox.filesystem.disabled`, `sandbox.network.allowAllUnixSockets`, `sandbox.network.allowUnixSockets` und `sandbox.network.allowMachLookup`. Ein `sandbox.credentials` Block, der nur `deny` Regeln enthält, benötigt keine Genehmigung, da er die Sandbox einschränkt, ohne dem Proxy Anmeldedaten zu geben. Vor v2.1.251 wendete Claude Code diese Einstellungen ohne Genehmigung an
* **Benutzerdefinierte Umgebungsvariablen**: bereitgestellte `env` Variablen, die die Genehmigung des Benutzers erfordern, wie Proxy- und Base-URL-Variablen; siehe [Umgebungsvariablen und der Genehmigungsdialog](#environment-variables-and-the-approval-dialog)
* **Hook-Konfigurationen**: jede Hook-Definition

Wenn diese Einstellungen vorhanden sind, sehen Benutzer einen Sicherheitsdialog, der erklärt, was konfiguriert wird. Benutzer müssen genehmigen, um fortzufahren. Wenn ein Benutzer die Einstellungen ablehnt, wird Claude Code beendet.

Ein verwalteter CLAUDE.md, der durch den [`claudeMd`](/docs/de/settings-reference#claudemd) Schlüssel bereitgestellt wird, benötigt keine Genehmigung, da es Anweisungstext für Claude ist, anstatt ein Befehl, den Claude Code ausführt. Claude Code überprüft immer noch [Berechtigungen](/docs/de/permissions) für die Tools, die Claude bei der Befolgung dieser Anweisungen verwendet. Vor v2.1.260 erforderte ein `claudeMd` Wert auch Genehmigung.

<h4 id="approval-memory">
  Genehmigungsspeicher
</h4>

Claude Code zeichnet Ihre Genehmigung in Ihrem Konfigurationsverzeichnis auf, `~/.claude` wenn Sie nicht [`CLAUDE_CONFIG_DIR`](/docs/de/env-vars) setzen. Was es aufzeichnet, hängt von der Anmeldedaten ab, die der Einstellungsabruf verwendet:

* **Eine claude.ai-Anmeldung, die durch `/login` oder `claude auth login` gespeichert wurde, oder die [schlüssellose Console-Anmeldung](/docs/de/authentication#sign-in-without-an-api-key)**: eine Genehmigung pro Organisation, gehalten vom Konto, das zuletzt genehmigt hat.
* **Eine [Claude-Apps-Gateway](/docs/de/claude-apps-gateway) Anmeldung**: eine Genehmigung pro Gateway.

  Wenn Sie sich vom gleichen Gateway ab- und wieder anmelden, zeigt Claude Code den Dialog nicht erneut an, während die Einstellungen, die Genehmigung erfordern, unverändert sind. Claude Code zeigt ihn erneut an, wenn sich diese Einstellungen ändern, wenn Sie sich bei einem anderen Gateway anmelden und wenn Sie ein neues Zertifikat für das gleiche Gateway akzeptieren.

  Claude Code speichert keine Genehmigung für ein Loopback-Entwicklungs-Gateway, das über einfaches HTTP erreicht wird, daher wird der Dialog nach jeder Anmeldung erneut angezeigt.
* **Jede andere Anmeldedaten**, wie ein API-Schlüssel oder `CLAUDE_CODE_OAUTH_TOKEN`: eine Genehmigung für die bereitgestellten Einstellungen, die mit der zwischengespeicherten Kopie der Einstellungen in diesem Konfigurationsverzeichnis aufbewahrt werden. Claude Code zeigt den Dialog erneut an, wenn sich die Einstellungen ändern, die Genehmigung erfordern, und nachdem Sie `/logout` oder `claude auth logout` ausführen, von denen jeder die zwischengespeicherte Kopie löscht.

Eine Genehmigung für `sandbox.credentials` oder `sandbox.network.tlsTerminate` deckt auch die [`sandbox.network.allowedDomains`](/docs/de/settings-reference#sandbox-network-alloweddomains) Einträge in denselben bereitgestellten Einstellungen ab, da beide Einstellungen auf diese Zulassungsliste wirken. Der Dialog wird erneut angezeigt, wenn Ihr Administrator einen dieser Einträge hinzufügt oder entfernt, obwohl `sandbox.network.allowedDomains` selbst keine Genehmigung erfordert.

Mit einer gespeicherten claude.ai-Anmeldung:

* Wenn Sie sich ab- und wieder anmelden, oder zu einer anderen Organisation wechseln und später zurückkehren, zeigt Claude Code den Dialog nicht erneut an, während diese Einstellungen unverändert sind, es sei denn, ein anderes Konto hat sie für diese Organisation im gleichen Konfigurationsverzeichnis dazwischen genehmigt.
* Wenn Sie sich bei der gleichen Organisation mit einem anderen Konto anmelden, zeigt Claude Code den Dialog erneut an, auch wenn die Einstellungen unverändert sind. Die Genehmigung dieses Kontos ersetzt die vorherige, daher zeigt Claude Code den Dialog erneut an, wenn Sie zurückwechseln.

Claude Code kann den Dialog nicht immer anzeigen. Jeder Fall unten sagt, welche Einstellungen gelten, wenn es nicht kann, und wann Sie den Dialog nächstes Mal sehen:

* **Eine interaktive Sitzung, die den Dialog nicht anzeigen kann**: Claude Code wendet die bereitgestellten Einstellungen nicht an und behält die zuletzt genehmigten Einstellungen. Der Dialog wird in der nächsten Sitzung angezeigt, die ihn anzeigen kann. Erfordert Claude Code v2.1.211 oder später.
* **`claude install` oder `claude update`**: Claude Code zeigt den Dialog während keinem der beiden Befehle an. Der Befehl wird mit den zuletzt genehmigten Einstellungen ausgeführt, und der Dialog wird in Ihrer nächsten interaktiven Sitzung angezeigt. Wenn Claude Code beim Start auf den Einstellungsabruf wartet, wie mit [`forceRemoteSettingsRefresh`](#enforce-fail-closed-startup) gesetzt oder bei einer [Claude-Apps-Gateway](/docs/de/claude-apps-gateway) Bereitstellung, zeigt es den Dialog stattdessen während des Befehls an, und ein Install-Lauf aus einer Pipe schlägt fehl; siehe [`Raw mode is not supported` während der Installation](/docs/de/troubleshoot-install#raw-mode-is-not-supported-during-install). Vor v2.1.246 versuchte Claude Code, den Dialog auch während dieser Befehle anzuzeigen.
* **Ein Fehler schließt den Dialog, bevor Sie antworten**: Claude Code wendet die bereitgestellten Einstellungen nicht an und behält die zuletzt genehmigten Einstellungen. Es zeigt den Dialog erneut in der nächsten Sitzung an, die ihn anzeigen kann.
* **Ein nicht-interaktiver Lauf**, wie `claude -p` oder eine Agent SDK-Sitzung: Claude Code kann den Dialog nicht anzeigen, daher wendet es sie an, wenn die bereitgestellten Einstellungen Genehmigung erfordern würden, nur für diesen Lauf an. Es zeichnet sie nicht als genehmigt auf oder schreibt sie in den [lokalen Cache](#fetch-and-caching-behavior), und die nächste interaktive Sitzung zeigt den Dialog. Bis ein Benutzer in einer interaktiven Sitzung genehmigt, ruft jeder nicht-interaktive Lauf die Einstellungen beim Start erneut ab. Vor v2.1.207 speicherte ein nicht-interaktiver Lauf die Einstellungen als genehmigt, daher zeigten spätere interaktive Sitzungen den Dialog dafür nie.

<h4 id="environment-variables-and-the-approval-dialog">
  Umgebungsvariablen und der Genehmigungsdialog
</h4>

Claude Code wendet einige bereitgestellte `env` Variablen an, ohne dem Benutzer den Genehmigungsdialog anzuzeigen, einschließlich:

* Funktions- und Befehlsumschalter
* Modellauswahl und Verhaltenseinstellungen, wie `ANTHROPIC_MODEL`, `DISABLE_PROMPT_CACHING` und `CLAUDE_CODE_EFFORT_LEVEL`
* Kontextfenster- und Kompaktierungseinstellungen, wie `DISABLE_AUTO_COMPACT`
* Terminal-UI und Barrierefreiheitsoptionen
* Numerische Grenzen, Budgets und Timeouts

Andere bereitgestellte Variablen können die Genehmigung des Benutzers erfordern, bevor sie wirksam werden; ein nicht leerer Proxy-, Base-URL- oder `OTEL_EXPORTER_OTLP_ENDPOINT` Wert tut dies immer. Wenn eine bereitgestellte Variable Genehmigung benötigt, benennt der Dialog sie, daher sieht der Benutzer genau, was die Richtlinie zu setzen versucht. Vor v2.1.218 wendete Claude Code weniger Variablen an, ohne den Benutzer zu fragen, daher lösten Einstellungen wie `DISABLE_AUTO_COMPACT` den Dialog bei jedem nicht leeren Wert aus.

Claude Code entscheidet, ob vier Datenschutz-Umschalter Genehmigung benötigen, anhand des bereitgestellten Werts statt des Variablennamens: `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`, `DISABLE_ERROR_REPORTING`, `DISABLE_TELEMETRY` und `DO_NOT_TRACK`. Ein wahrheitsgemäßer Wert wie `1` oder `true` schaltet nur Tracking, Berichterstattung oder anderen nicht wesentlichen Datenverkehr aus, daher wendet Claude Code ihn an, ohne den Benutzer zu fragen. Für jeden anderen nicht leeren Wert zeigt Claude Code den Dialog an. Vor v2.1.218 wendeten alle außer `DO_NOT_TRACK` ohne Genehmigung bei jedem Wert an, und `DO_NOT_TRACK` löste den Dialog bei jedem nicht leeren Wert aus.

Claude Code entscheidet auch, ob [`API_FORCE_IDLE_TIMEOUT`](/docs/de/env-vars) Genehmigung benötigt, anhand des bereitgestellten Werts: Ein wahrheitsgemäßer Wert schaltet nur das [Body-Idle-Timeout](/docs/de/network-config#streaming-idle-watchdogs) ein, daher wendet Claude Code ihn an, ohne den Benutzer zu fragen. Für jeden anderen nicht leeren Wert zeigt Claude Code den Dialog an. Vor v2.1.248 löste jeder nicht leere Wert den Dialog aus.

Ob [`ANTHROPIC_CUSTOM_HEADERS`](/docs/de/env-vars#variables) Genehmigung benötigt, hängt auch vom bereitgestellten Wert ab. Header, die nur Anfragen kennzeichnen, wie `Accept-Language`, werden ohne den Dialog angewendet. Eine Zeile, die eine Anmeldedaten, einen Organisations- oder Mandanten-Selektor, einen Routing- oder Host-Override oder einen API-Verhaltens-Header benennt, wie `Authorization`, `X-Api-Key`, `Host`, `anthropic-beta` oder die `X-Amzn-Bedrock-*` Header, benötigt Genehmigung. Ebenso eine Zeile, deren Name kein gültiges HTTP-Header-Token ist, oder deren Wert ein Zeichen enthält, das ein HTTP-Header nicht tragen kann. Die Überprüfung stimmt Wörter im Header-Namen ab, daher benötigt `X-Client-Version`, das `client` und `version` enthält, auch Genehmigung. Vor v2.1.251 wurde jeder `ANTHROPIC_CUSTOM_HEADERS` Wert ohne sie angewendet.

Ein Falsy-Wert wie `0` oder `false` für [`ENABLE_BETA_TRACING_DETAILED`](/docs/de/env-vars#variables) oder [`OTEL_LOG_RAW_API_BODIES`](/docs/de/env-vars#variables) wird ohne den Dialog angewendet, da er nur detailliertes Tracing oder Raw-API-Body-Erfassung ausschaltet. Jeder andere nicht leere Wert für eine der beiden Variablen benötigt Genehmigung.

<h2 id="platform-availability">
  Plattformverfügbarkeit
</h2>

Serververwaltete Einstellungen erfordern eine direkte Verbindung zu `api.anthropic.com`. Die Bereitstellung erfordert auch, dass sich die Sitzung mit einer dieser Anmeldedaten authentifiziert:

* Ein Team- oder Enterprise-OAuth-Login
* Ein OAuth-Token, das über `CLAUDE_CODE_OAUTH_TOKEN` bereitgestellt wird
* Ein direkt konfigurierter API-Schlüssel
* Ein `user_oauth` [Anthropic-Profil](/docs/de/authentication#anthropic-profiles-and-federation-credentials), es sei denn, das Profil setzt eine `base_url` andere als die Anthropic API. Erfordert Claude Code v2.1.257 oder später.

Weder Schlüssel, die von einem [`apiKeyHelper`](/docs/de/settings-reference#apikeyhelper)-Skript zurückgegeben werden, noch [Workload Identity Federation](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation)-Anmeldedaten lösen den Abruf der Einstellungen aus.

In einer [Cowork](https://claude.com/docs/cowork/overview)-Sitzung in der Claude Desktop-App ruft Claude Code keine serververwalteten Einstellungen aus der claude.ai-Admin-Konsole ab, auch wenn sich der Benutzer mit einem Team- oder Enterprise-Konto anmeldet. [Wo und wann eine Richtlinie gilt](/docs/de/managed-settings#where-and-when-a-policy-applies) behandelt, welche Richtlinie Cowork-Sitzungen auf dem Computer des Benutzers und Remote-Cowork-Sitzungen erreicht. claude.ai wendet weiterhin Ihre [`strictKnownMarketplaces`](/docs/de/settings-reference#strictknownmarketplaces)- und [`blockedMarketplaces`](/docs/de/settings-reference#blockedmarketplaces)-Listen selbst an, wenn ein Cowork-Benutzer einen Marketplace aus einem Git-Repository auf claude.ai oder aus **Anpassen** in der Cowork-Registerkarte hinzufügt. [Wie Einschränkungen funktionieren](/docs/de/plugins/org#restrict-what-users-can-install) beschreibt diese Überprüfung.

Wenn Sie eine `CLAUDE_CODE_USE_*`-Providervariable oder eine nicht standardmäßige `ANTHROPIC_BASE_URL` in Ihrer Shell exportieren, überspringt Claude Code den Abruf der Einstellungen für Ihre Sitzungen. [`claude doctor` und `/status` melden den übersprungenen Abruf und dessen Ursache](#verify-settings-delivery).

Sie können den Export nicht mit einem serververwalteten `env`-Block löschen, da der Block durch den Abruf ankommt, den der Export verhindert. Ein [endpunktverwalteter Einstellungen](/docs/de/managed-settings#delivery-mechanisms)-`env`-Block stellt den Abruf auch nicht wieder her: Claude Code prüft die Berechtigung, bevor es verwaltete `env`-Blöcke anwendet, sodass die Überschreibung die Providerauswahl der Sitzung ändert, aber der Abruf bleibt übersprungen.

Um die serververwaltete Bereitstellung wiederherzustellen, entfernen Sie den Export aus Ihrer Shell, oder setzen Sie die Variable auf `""` in Ihrem Benutzereinstellungen-`env`-Block, der vor der Berechtigungsprüfung angewendet wird. Um eine Richtlinie durchzusetzen, ohne sich auf Benutzer zu verlassen, die ihre Shells ändern, stellen Sie die Einstellungen stattdessen über den endpunktverwalteten Kanal bereit.

Für Amazon Bedrock-, Google Cloud's Agent Platform-, Microsoft Foundry- und [Claude Platform on AWS](/docs/de/claude-platform-on-aws)-Bereitstellungen bietet ein selbstgehostetes [Claude-Apps-Gateway](/docs/de/claude-apps-gateway) die entsprechende Remote-Verwaltung von Einstellungen: Mit dem Gateway angemeldete Clients rufen verwaltete Einstellungen vom Gateway statt von `api.anthropic.com` ab. Die Fehlerbehandlung unterscheidet sich beim Start: Ein Gateway-Client, der das Gateway nicht erreichen kann, beendet sich mit einem Fehler, anstatt auf zwischengespeicherte Einstellungen zurückzugreifen, während die stündliche Hintergrundaktualisierung auf beiden Kanälen fehleroffen ist.

<h2 id="audit-logging">
  Audit-Protokollierung
</h2>

Audit-Log-Ereignisse für Einstellungsänderungen sind über die Compliance-API oder den Audit-Log-Export verfügbar. Kontaktieren Sie Ihr Anthropic-Kontoteam für Zugriff.

Audit-Ereignisse enthalten den Typ der durchgeführten Aktion, das Konto und das Gerät, das die Aktion durchgeführt hat, sowie Verweise auf die vorherigen und neuen Werte.

<h2 id="security-considerations">
  Sicherheitsüberlegungen
</h2>

Serververwaltete Einstellungen bieten zentralisierte Richtliniendurchsetzung, funktionieren aber als clientseitige Kontrolle, nicht als Sicherheitsgrenze. Auf nicht verwalteten Geräten benötigt ein Benutzer keinen Admin- oder Sudo-Zugriff, um diese zu umgehen.

| Szenario                                                                           | Verhalten                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :--------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Benutzer bearbeitet die zwischengespeicherte Einstellungsdatei                     | Manipulierte Datei wird beim Start angewendet, außer für die [Werte, die Claude Code bis zur Bestätigung der Nutzlast durch den Server zurückhält](#fetch-and-caching-behavior). Der nächste Serverfetch stellt die korrekten Einstellungen wieder her, außer für die [Schlüssel, die nur beim nächsten Start gelten](#fetch-and-caching-behavior), wie `model` oder eine Variable, die zum `env`-Block hinzugefügt wurde, die bis zum Neustart wirksam bleiben                                                                                                                                                                                                                                                                                                        |
| Benutzer löscht die zwischengespeicherte Einstellungsdatei                         | [Verhalten beim ersten Start](#fetch-and-caching-behavior) tritt auf                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| Benutzer führt eine modifizierte Claude Code-Binärdatei aus                        | Ein Benutzer, der einen modifizierten Client ausführen kann, kann jede clientseitige Kontrolle umgehen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| Benutzer führt eine ältere Claude Code-Version aus                                 | Versionen, die vor serververwalteten Einstellungen entstanden sind, rufen diese nicht ab oder wenden sie nicht an                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| API ist nicht verfügbar                                                            | Zwischengespeicherte Einstellungen werden angewendet, falls verfügbar, außer für die [Werte, die Claude Code bis zu einem erfolgreichen Fetch zurückhält](#fetch-and-caching-behavior). Ohne einen Cache erzwingt Claude Code keine serververwalteten Einstellungen, bis der nächste erfolgreiche Fetch erfolgt, und wendet weiterhin alle [endpunktverwalteten Einstellungen](/docs/de/managed-settings#delivery-mechanisms) auf dem Gerät an. Mit `forceRemoteSettingsRefresh: true` wird die CLI stattdessen beendet, außer für [`claude auth` Unterbefehle](#enforce-fail-closed-startup). Clients, die sich über ein [Claude Apps Gateway](#platform-availability) anmelden, werden beim Start ohne diese Einstellung beendet, mit der gleichen `claude auth`-Ausnahme |
| Benutzer authentifiziert sich mit einer anderen Organisation                       | Einstellungen werden nicht für Konten außerhalb der verwalteten Organisation bereitgestellt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| Benutzer konfiguriert einen [Drittanbieter-Modellprovider](#platform-availability) | Serververwaltete Einstellungen werden umgangen. Dies umfasst das Setzen von `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_MANTLE`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, `CLAUDE_CODE_USE_ANTHROPIC_AWS` oder einer nicht standardmäßigen `ANTHROPIC_BASE_URL`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| Netzwerkverkehr wird abgefangen oder umgeleitet                                    | Deaktivierte TLS-Validierung oder abgefangener Verkehr kann die Einstellungen ändern, die der Client erhält                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |

Um Bearbeitungen lokaler Einstellungsdateien, einschließlich `managed-settings.json`, zu protokollieren, verwenden Sie [`ConfigChange` hooks](/docs/de/hooks#configchange). Claude Code führt diese nicht aus, wenn serververwaltete Einstellungen ankommen oder aktualisiert werden, oder wenn sich ein MDM-Profil oder eine Registrierungsrichtlinie ändert, und ein Hook kann eine `policy_settings`-Änderung nicht blockieren.

Um einzuschränken, auf welche Organisationen Ihre Benutzer mit den vom Client bereitgestellten Anmeldedaten zugreifen können, siehe [Netzwerkzugriffskontrolle mit Tenant Restrictions durchsetzen](https://support.claude.com/en/articles/13198485-enforce-network-level-access-control-with-tenant-restrictions) im Claude Help Center. Für stärkere Durchsetzungsgarantien verwenden Sie [endpunktverwaltete Einstellungen](/docs/de/managed-settings#delivery-mechanisms) auf Geräten, die in einer MDM-Lösung registriert sind.

<h2 id="see-also">
  Siehe auch
</h2>

Verwandte Seiten zur Verwaltung der Claude Code-Konfiguration:

* [Alle Einstellungen](/docs/de/settings-reference): jeder Einstellungsschlüssel
* [Endpunktverwaltete Einstellungen](/docs/de/managed-settings#delivery-mechanisms): verwaltete Einstellungen, die von der IT auf Geräten bereitgestellt werden
* [Authentifizierung](/docs/de/authentication): Einrichtung des Benutzerzugriffs auf Claude Code
* [Sicherheit](/docs/de/security): Sicherheitsvorkehrungen und Best Practices
