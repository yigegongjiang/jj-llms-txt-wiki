> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Serververwaltete Einstellungen konfigurieren

> Konfigurieren Sie Claude Code zentral für Ihre Organisation durch serververwaltete Einstellungen, ohne dass eine Geräteverwaltungsinfrastruktur erforderlich ist.

Serververwaltete Einstellungen ermöglichen es Administratoren, Claude Code zentral über [**Admin-Einstellungen > Claude Code > Verwaltete Einstellungen**](https://claude.ai/admin-settings/claude-code) in der claude.ai-Konsole zu konfigurieren. Claude Code-Clients rufen diese Einstellungen automatisch ab, wenn sich Benutzer mit einer Organisations-OAuth-Anmeldung oder einem direkt konfigurierten API-Schlüssel authentifizieren, auf Plattformen, auf denen die serververwaltete Bereitstellung unterstützt wird. Siehe [Plattformverfügbarkeit](#platform-availability).

Dieser Ansatz ist für Organisationen konzipiert, die keine Geräteverwaltungsinfrastruktur haben oder Einstellungen für Benutzer auf nicht verwalteten Geräten verwalten müssen.

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

Claude Code unterstützt zwei Ansätze für zentralisierte Konfiguration. Serververwaltete Einstellungen liefern Konfiguration von Anthropics Servern. [Endpunktverwaltete Einstellungen](/docs/de/settings#settings-files) werden direkt auf Geräten über native Betriebssystemrichtlinien (macOS verwaltete Einstellungen, Windows-Registrierung) oder verwaltete Einstellungsdateien bereitgestellt.

| Ansatz                                                              | Am besten geeignet für                                              | Sicherheitsmodell                                                                                                                                  |
| :------------------------------------------------------------------ | :------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Serververwaltete Einstellungen**                                  | Organisationen ohne MDM oder Benutzer auf nicht verwalteten Geräten | Einstellungen, die von Anthropics Servern zum Authentifizierungszeitpunkt bereitgestellt werden                                                    |
| **[Endpunktverwaltete Einstellungen](/docs/de/settings#settings-files)** | Organisationen mit MDM oder Endpunktverwaltung                      | Einstellungen, die auf Geräten über MDM-Konfigurationsprofile, Registrierungsrichtlinien oder verwaltete Einstellungsdateien bereitgestellt werden |

Wenn Ihre Geräte in einer MDM- oder Endpunktverwaltungslösung registriert sind, bieten endpunktverwaltete Einstellungen stärkere Sicherheitsgarantien, da die Einstellungsdatei auf Betriebssystemebene vor Benutzermodifikationen geschützt werden kann. Endpunktverwaltete Einstellungen erreichen [Cloud-Sitzungen](/docs/de/model-config#surface-coverage) nicht, daher sollten Organisationen, die Claude Code im Web verwenden, auch serververwaltete Einstellungen konfigurieren.

<h2 id="configure-server-managed-settings">
  Serververwaltete Einstellungen konfigurieren
</h2>

<Steps>
  <Step title="Öffnen Sie die Admin-Konsole">
    Navigieren Sie in der claude.ai-Konsole zu [**Admin-Einstellungen > Claude Code > Verwaltete Einstellungen**](https://claude.ai/admin-settings/claude-code).

    Wenn der Link Sie stattdessen zu einer anderen Admin-Einstellungen-Seite umleitet, anstatt zur Claude Code-Seite, verfügt Ihr Konto nicht über die erforderliche Rolle. Admin und andere Nicht-Eigentümer-Rollen können verwaltete Einstellungen nicht anzeigen oder bearbeiten. Bitten Sie daher einen Eigentümer oder Primären Eigentümer in Ihrer Organisation, die Änderung vorzunehmen. Siehe [Zugriffskontrolle](#access-control).
  </Step>

  <Step title="Definieren Sie Ihre Einstellungen">
    Fügen Sie Ihre Konfiguration als JSON hinzu. Alle [in `settings.json` verfügbaren Einstellungen](/docs/de/settings#available-settings) werden unterstützt, mit Ausnahme derjenigen, die auf die Bereitstellung auf OS-Ebene beschränkt sind. Siehe [Aktuelle Einschränkungen](#current-limitations) für diese kurze Liste. Dies umfasst [hooks](/docs/de/hooks), [Umgebungsvariablen](/docs/de/env-vars) und [nur verwaltete Einstellungen](/docs/de/permissions#managed-only-settings) wie `allowManagedPermissionRulesOnly`.

    Dieses Beispiel erzwingt eine Berechtigungsverweigerungsliste, verhindert, dass Benutzer Berechtigungen umgehen, und beschränkt Berechtigungsregeln auf diejenigen, die in verwalteten Einstellungen definiert sind:

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

    Um den [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) Klassifizierer zu konfigurieren, damit er weiß, welche Repos, Buckets und Domains Ihre Organisation vertraut:

    ```json theme={null}
    {
      "autoMode": {
        "environment": [
          "Source control: github.example.com/acme-corp and all repos under it",
          "Trusted cloud buckets: s3://acme-build-artifacts, gs://acme-ml-datasets",
          "Trusted internal domains: *.corp.example.com"
        ]
      }
    }
    ```

    Da Hooks Shell-Befehle ausführen, sehen Benutzer einen [Sicherheitsgenehmigungsdialog](#security-approval-dialogs), bevor sie angewendet werden. Siehe [Auto-Modus konfigurieren](/docs/de/auto-mode-config), um zu erfahren, wie die `autoMode` Einträge beeinflussen, was der Klassifizierer blockiert, und wichtige Warnungen zu den Feldern `environment`, `allow`, `soft_deny` und `hard_deny`.
  </Step>

  <Step title="Speichern und bereitstellen">
    Speichern Sie Ihre Änderungen. Claude Code-Clients erhalten die aktualisierten Einstellungen beim nächsten Start oder während des stündlichen Abrufzyklus.
  </Step>
</Steps>

<h3 id="verify-settings-delivery">
  Einstellungsbereitstellung überprüfen
</h3>

Um zu bestätigen, dass Einstellungen angewendet werden, bitten Sie einen Benutzer, Claude Code neu zu starten. Wenn die Konfiguration Einstellungen enthält, die den [Sicherheitsgenehmigungsdialog](#security-approval-dialogs) auslösen, sieht der Benutzer beim Start eine Eingabeaufforderung, die die verwalteten Einstellungen beschreibt. Sie können auch überprüfen, dass verwaltete Berechtigungsregeln aktiv sind, indem Sie einen Benutzer `/permissions` ausführen lassen, um seine geltenden Berechtigungsregeln anzuzeigen.

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

Die meisten [Einstellungsschlüssel](/docs/de/settings#available-settings) funktionieren in jedem Bereich. Eine Handvoll Schlüssel werden nur aus verwalteten Einstellungen gelesen und haben keine Auswirkung, wenn sie in Benutzer- oder Projekteinstellungsdateien platziert werden. Siehe [nur verwaltete Einstellungen](/docs/de/permissions#managed-only-settings) für die vollständige Liste. Jede Einstellung, die nicht auf dieser Liste steht, kann immer noch in verwalteten Einstellungen platziert werden und hat die höchste Priorität.

<h3 id="current-limitations">
  Aktuelle Einschränkungen
</h3>

Serververwaltete Einstellungen haben die folgenden Einschränkungen:

* Einstellungen gelten einheitlich für alle Benutzer in der Organisation. Konfigurationen pro Gruppe werden noch nicht unterstützt.
* Eine [`managed-mcp.json`](/docs/de/managed-mcp) Datei kann nicht über serververwaltete Einstellungen verteilt werden. Stellen Sie stattdessen die Richtlinienschlüssel `allowedMcpServers` und `deniedMcpServers` dort bereit.
* Einstellungen, die auf OS-Ebene-Richtlinienquellen beschränkt sind, wie `policyHelper` und `wslInheritsWindowsSettings`, werden nicht berücksichtigt. Stellen Sie sie stattdessen über MDM oder eine System-Datei `managed-settings.json` bereit.

<h2 id="settings-delivery">
  Einstellungsbereitstellung
</h2>

<h3 id="settings-precedence">
  Einstellungspriorität
</h3>

Serververwaltete Einstellungen und [endpunktverwaltete Einstellungen](/docs/de/settings#settings-files) nehmen beide die höchste Ebene in der Claude Code [Einstellungshierarchie](/docs/de/settings#settings-precedence) ein. Keine andere Einstellungsebene kann sie überschreiben, einschließlich Befehlszeilenargumenten.

Innerhalb der verwalteten Ebene gewinnt eine konfigurierte [`policyHelper`](/docs/de/settings#compute-managed-settings-with-a-policy-helper) vor jeder anderen verwalteten Quelle, einschließlich serververwalteter Einstellungen: Ihre Ausgabe wird die einzige verwaltete Konfiguration für den Lauf.

Andernfalls verwendet Claude Code die erste Quelle, die eine nicht leere Konfiguration liefert. Serververwaltete Einstellungen werden zuerst überprüft, dann endpunktverwaltete Einstellungen. Quellen werden nicht zusammengeführt: Wenn serververwaltete Einstellungen überhaupt Schlüssel liefern, werden andere endpunktverwaltete Einstellungen ignoriert. Wenn serververwaltete Einstellungen nichts liefern, gelten endpunktverwaltete Einstellungen.

Eine Ausnahme gilt: Ein kleiner Satz von [quellübergreifenden Sperr-Schlüsseln](/docs/de/settings#settings-precedence), wie die Sandbox-Zulassungslisten-Sperren, wird berücksichtigt, wenn eine beliebige administratorgesteuerte verwaltete Quelle sie setzt; die benutzerbare HKCU-Registrierungsebene ist ausgeschlossen.

Wenn Sie Ihre serververwaltete Konfiguration in der Admin-Konsole mit der Absicht löschen, auf eine endpunktverwaltete plist oder Registrierungsrichtlinie zurückzugreifen, beachten Sie, dass [zwischengespeicherte Einstellungen](#fetch-and-caching-behavior) auf Client-Maschinen bestehen bleiben, bis der nächste erfolgreiche Abruf erfolgt. Führen Sie `/status` aus, um zu sehen, welche verwaltete Quelle aktiv ist.

<h3 id="fetch-and-caching-behavior">
  Abruf- und Caching-Verhalten
</h3>

Claude Code ruft Einstellungen beim Start von Anthropics Servern ab und fragt stündlich während aktiver Sitzungen nach Updates ab.

**Erster Start ohne zwischengespeicherte Einstellungen:**

* Claude Code ruft Einstellungen asynchron ab
* Wenn der Abruf fehlschlägt, wird Claude Code ohne verwaltete Einstellungen fortgesetzt
* Es gibt ein kurzes Fenster, bevor Einstellungen geladen werden, in dem Einschränkungen noch nicht erzwungen werden

**Nachfolgende Starts mit zwischengespeicherten Einstellungen:**

* Zwischengespeicherte Einstellungen werden beim Start sofort angewendet, außer für die unten beschriebenen Transport-, Routing- und Authentifizierungs-Umgebungsvariablen
* Claude Code ruft frische Einstellungen im Hintergrund ab
* Zwischengespeicherte Einstellungen bleiben bei Netzwerkfehlern erhalten. Die zurückhaltenden Umgebungsvariablen bleiben zurückhaltend, bis ein Abruf erfolgreich ist

Ab v2.1.198 hält Claude Code drei Kategorien von Variablen im zwischengespeicherten `env`-Block zurück, bis der Server die Payload für die Sitzung bestätigt. Dies verhindert, dass ein zwischengespeicherter Proxy-, Zertifizierungsstellen-, Endpunkt- oder Anmeldedatenwert den Einstellungsabruf umleitet, abfängt oder erneut authentifiziert, der die Payload bestätigt. Die Härtung gilt nur für den vom Server abgerufenen Einstellungs-Cache: [endpunktverwaltete Einstellungen](/docs/de/settings#settings-files), die über MDM oder `managed-settings.json` bereitgestellt werden, sind nicht betroffen. Die zurückhaltenden Kategorien sind:

* Proxy- und TLS-Konfiguration, wie `HTTPS_PROXY`, `NODE_EXTRA_CA_CERTS` und die mTLS-Client-Zertifikatvariablen `CLAUDE_CODE_CLIENT_CERT` und `CLAUDE_CODE_CLIENT_KEY`
* API-Routing und Anbieterauswahl, einschließlich `ANTHROPIC_BASE_URL`, der Anbieterauswahlvariablen wie `CLAUDE_CODE_USE_BEDROCK` und `CLAUDE_CODE_USE_VERTEX` und der Anbieter-Endpunkt-URLs wie `ANTHROPIC_BEDROCK_BASE_URL`
* Authentifizierungsanmeldedaten, wie `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` und `CLAUDE_CODE_OAUTH_TOKEN`

Jeder andere Schlüssel im zwischengespeicherten `env`-Block, wie Telemetrie- und OpenTelemetry-Konfiguration, wird beim Start wie zuvor angewendet. Sobald der Abruf erfolgreich ist, werden die zurückhaltenden Variablen für den Rest der Sitzung angewendet.

Wenn Ihre Organisation einen Proxy benötigt, um `api.anthropic.com` zu erreichen, setzen Sie ihn in der Shell-Umgebung oder in [Benutzereinstellungen](/docs/de/settings#settings-files) anstatt nur im verwalteten `env`-Block. Der erste Start hat keinen Cache, daher waren diese Quellen bereits für den anfänglichen Abruf erforderlich.

Claude Code wendet Einstellungsaktualisierungen automatisch ohne Neustart an, außer für erweiterte Einstellungen wie OpenTelemetry-Konfiguration, die einen vollständigen Neustart erfordern, um wirksam zu werden.

<h3 id="invalid-entries-in-delivered-settings">
  Ungültige Einträge in bereitgestellten Einstellungen
</h3>

Bereitgestellte Payloads werden tolerant mit den gleichen Regeln wie die anderen verwalteten Quellen analysiert. Wenn eine Payload einen Eintrag enthält, der die Schemavalidierung nicht besteht, entfernt Claude Code diesen Eintrag, zeigt einen Validierungsfehler an und wendet alle verbleibenden gültigen Einstellungen an. Siehe [Ungültige Einträge in verwalteten Einstellungen](/docs/de/settings#invalid-entries-in-managed-settings) für das Verhalten auf Feldebene, einschließlich der Behandlung von Sicherheitserzwingungsfeldern. Erfordert Claude Code v2.1.169 oder später.

Die serververwaltete Bereitstellung fügt diese Verhaltensweisen hinzu:

* Der Cache unter `~/.claude/remote-settings.json` speichert die gerettete Payload mit entfernten ungültigen Einträgen. Die rohe ungültige Payload wird niemals beibehalten.
* Wenn kein Feld in der Payload gerettet werden kann, behält Claude Code die zuletzt akzeptierten zwischengespeicherten Einstellungen bei und zeichnet einen schwerwiegenden Fehler auf.
* Der [Sicherheitsgenehmigungsdialog](#security-approval-dialogs) bewertet die gerettete Payload, sodass ein entfernter ungültiger Eintrag niemals zur Genehmigung präsentiert wird und niemals ausgeführt wird.

Um Bereitstellungsprobleme zu debuggen, führen Sie `claude --debug-file <path>` aus und suchen Sie im Protokoll nach `Remote settings`. Validieren Sie eine Payload-Änderung mit `claude doctor` auf einem Test-Computer, bevor Sie sie in der Organisation bereitstellen.

<h3 id="enforce-fail-closed-startup">
  Erzwingen Sie einen Fail-Closed-Start
</h3>

Standardmäßig wird die CLI ohne verwaltete Einstellungen fortgesetzt, wenn der Abruf der Remote-Einstellungen beim Start fehlschlägt. Für Umgebungen, in denen dieses kurze nicht erzwungene Fenster nicht akzeptabel ist, setzen Sie `forceRemoteSettingsRefresh: true` in Ihren verwalteten Einstellungen.

Wenn diese Einstellung aktiv ist, blockiert die CLI beim Start, bis Remote-Einstellungen neu abgerufen werden. Wenn der Abruf fehlschlägt, wird die CLI beendet, anstatt ohne die Richtlinie fortzufahren. Diese Einstellung perpetuiert sich selbst: Sobald sie vom Server bereitgestellt wird, wird sie auch lokal zwischengespeichert, sodass nachfolgende Starts das gleiche Verhalten erzwingen, auch bevor der erste erfolgreiche Abruf einer neuen Sitzung erfolgt.

Um dies zu aktivieren, fügen Sie den Schlüssel zu Ihrer verwalteten Einstellungskonfiguration hinzu:

```json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

Sie können diesen Schlüssel auch in einem [endpunktverwalteten](/docs/de/settings#settings-files) MDM-Profil oder einer System-Datei `managed-settings.json` setzen, um Fail-Closed-Verhalten beim ersten Start durchzusetzen, bevor eine Server-Payload bereitgestellt wurde. Ab v2.1.191 ist dieses Flag eine Ausnahme von der [Prioritätsregel](#settings-precedence) oben: Es wird berücksichtigt, wenn es in einer beliebigen verwalteten Quelle gesetzt ist, auch wenn eine zwischengespeicherte Server-verwaltete Payload vorhanden ist, sodass ein MDM-bereitgestellter Wert nicht ignoriert wird, wenn serververwaltete Einstellungen vorhanden sind.

Der Einstellungsabruf sendet auch einen `Cache-Control: no-cache` Header, sodass zwischengeschaltete HTTP-Proxys keine veraltete Antwort bereitstellen.

Bevor Sie diese Einstellung aktivieren, stellen Sie sicher, dass Ihre Netzwerkrichtlinien die Konnektivität zu `api.anthropic.com` ermöglichen. Wenn dieser Endpunkt nicht erreichbar ist, wird die CLI beim Start beendet und Benutzer können Claude Code nicht starten.

Ab v2.1.139 sind die `claude auth` Unterbefehle wie `claude auth login` von dieser Überprüfung ausgenommen, sodass Benutzer sich erneut authentifizieren können, wenn abgelaufene Anmeldedaten der Grund für den fehlgeschlagenen Einstellungsabruf sind.

<h3 id="security-approval-dialogs">
  Sicherheitsgenehmigungsdialoge
</h3>

Bestimmte Einstellungen, die Sicherheitsrisiken darstellen könnten, erfordern explizite Benutzergenehmigung, bevor Claude Code sie anwendet:

* **Shell-Befehlseinstellungen**: Einstellungen, die Shell-Befehle ausführen
* **Benutzerdefinierte Umgebungsvariablen**: Variablen, die nicht in der bekannten sicheren Zulassungsliste enthalten sind
* **Hook-Konfigurationen**: jede Hook-Definition
* **Verwalteter CLAUDE.md-Inhalt**: ein `claudeMd`-Wert, der durch verwaltete Einstellungen bereitgestellt wird

Wenn diese Einstellungen vorhanden sind, sehen Benutzer einen Sicherheitsdialog, der erklärt, was konfiguriert wird. Benutzer müssen genehmigen, um fortzufahren. Wenn ein Benutzer die Einstellungen ablehnt, wird Claude Code beendet.

<Note>
  Ein nicht-interaktiver Lauf, wie `claude -p` oder eine Agent SDK-Sitzung, kann den Dialog nicht anzeigen. Wenn die bereitgestellten Einstellungen eine Genehmigung erfordern würden, wendet Claude Code sie nur für diesen Lauf an: Es zeichnet sie nicht als genehmigt auf und schreibt sie nicht in den [lokalen Cache](#fetch-and-caching-behavior), und die nächste interaktive Sitzung zeigt den Dialog. Bis ein Benutzer in einer interaktiven Sitzung genehmigt, ruft jeder nicht-interaktive Lauf die Einstellungen beim Start erneut ab. Vor v2.1.207 speicherte ein nicht-interaktiver Lauf die Einstellungen als genehmigt, sodass spätere interaktive Sitzungen den Dialog dafür nie zeigten.
</Note>

<h2 id="platform-availability">
  Plattformverfügbarkeit
</h2>

Serververwaltete Einstellungen erfordern eine direkte Verbindung zu `api.anthropic.com`, und die Bereitstellung erfordert, dass sich die Sitzung mit einem Organisations-OAuth-Login oder einem direkt konfigurierten API-Schlüssel authentifiziert. Schlüssel, die von einem [`apiKeyHelper`](/docs/de/settings#available-settings)-Skript zurückgegeben werden, lösen den Abruf der Einstellungen nicht aus.

Serververwaltete Einstellungen sind nicht verfügbar, wenn Drittanbieter-Modellprovider verwendet werden:

* Amazon Bedrock
* Google Cloud's Agent Platform
* Microsoft Foundry
* [Claude Platform on AWS](/docs/de/claude-platform-on-aws)
* Benutzerdefinierte API-Endpunkte über `ANTHROPIC_BASE_URL` oder Drittanbieter-[LLM-Gateways](/docs/de/llm-gateway)

Wenn Sie eine `CLAUDE_CODE_USE_*`-Providervariable oder eine nicht standardmäßige `ANTHROPIC_BASE_URL` in Ihrer Shell exportieren, überspringt Claude Code den Abruf der Einstellungen für Ihre Sitzungen. Sie können den Export nicht mit einem serververwalteten `env`-Block löschen, da der Block durch den Abruf ankommt, den der Export verhindert. Ein [endpunktverwalteter Einstellungen](/docs/de/settings#settings-files)-`env`-Block stellt den Abruf auch nicht wieder her: Claude Code prüft die Berechtigung, bevor es verwaltete `env`-Blöcke anwendet, sodass die Überschreibung die Providerauswahl der Sitzung ändert, aber der Abruf bleibt übersprungen.

Um die serververwaltete Bereitstellung wiederherzustellen, entfernen Sie den Export aus Ihrer Shell, oder setzen Sie die Variable auf `""` in Ihrem Benutzereinstellungen-`env`-Block, der vor der Berechtigungsprüfung angewendet wird. Um eine Richtlinie durchzusetzen, ohne sich auf Benutzer zu verlassen, die ihre Shells ändern, stellen Sie die Einstellungen stattdessen über den endpunktverwalteten Kanal bereit.

Für Amazon Bedrock-, Google Cloud's Agent Platform- und Microsoft Foundry-Bereitstellungen bietet ein selbstgehostetes [Claude-Apps-Gateway](/docs/de/claude-apps-gateway) die entsprechende Remote-Verwaltung von Einstellungen: Mit dem Gateway angemeldete Clients rufen verwaltete Einstellungen vom Gateway statt von `api.anthropic.com` ab. Die Fehlerbehandlung unterscheidet sich beim Start: Ein Gateway-Client, der das Gateway nicht erreichen kann, beendet sich mit einem Fehler, anstatt auf zwischengespeicherte Einstellungen zurückzugreifen, während die stündliche Hintergrundaktualisierung auf beiden Kanälen fehleroffen ist.

<h2 id="audit-logging">
  Audit-Protokollierung
</h2>

Audit-Log-Ereignisse für Einstellungsänderungen sind über die Compliance-API oder den Audit-Log-Export verfügbar. Kontaktieren Sie Ihr Anthropic-Kontoteam für Zugriff.

Audit-Ereignisse enthalten den Typ der durchgeführten Aktion, das Konto und das Gerät, das die Aktion durchgeführt hat, sowie Verweise auf die vorherigen und neuen Werte.

<h2 id="security-considerations">
  Sicherheitsüberlegungen
</h2>

Serververwaltete Einstellungen bieten zentralisierte Richtliniendurchsetzung, funktionieren aber als clientseitige Kontrolle, nicht als Sicherheitsgrenze. Auf nicht verwalteten Geräten benötigt ein Benutzer keinen Admin- oder Sudo-Zugriff, um diese zu umgehen.

| Szenario                                                                           | Verhalten                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| :--------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Benutzer bearbeitet die zwischengespeicherte Einstellungsdatei                     | Manipulierte Datei wird beim Start angewendet, aber korrekte Einstellungen werden beim nächsten Serverfetch wiederhergestellt. Ab v2.1.198 werden die Transport-, API-Routing- und Authentifizierungs-Umgebungsvariablen im `env`-Block [zurückgehalten, bis der Server die Nutzlast bestätigt](#fetch-and-caching-behavior)                                                                                                                                                                                                                                               |
| Benutzer löscht die zwischengespeicherte Einstellungsdatei                         | Verhalten beim ersten Start tritt auf: Einstellungen werden asynchron abgerufen mit einem kurzen nicht erzwungenen Fenster                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| Benutzer führt eine modifizierte Claude Code-Binärdatei aus                        | Ein Benutzer, der einen modifizierten Client ausführen kann, kann jede clientseitige Kontrolle umgehen                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| Benutzer führt eine ältere Claude Code-Version aus                                 | Versionen, die vor serververwalteten Einstellungen entstanden sind, rufen diese nicht ab oder wenden sie nicht an                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| API ist nicht verfügbar                                                            | Zwischengespeicherte Einstellungen werden angewendet, falls verfügbar, andernfalls werden verwaltete Einstellungen nicht erzwungen, bis der nächste erfolgreiche Abruf erfolgt. Ab v2.1.198 werden die Transport-, API-Routing- und Authentifizierungs-Umgebungsvariablen im zwischengespeicherten `env`-Block [bei Abruffehler zurückgehalten](#fetch-and-caching-behavior); der Rest des Caches wird weiterhin angewendet. Mit `forceRemoteSettingsRefresh: true` wird die CLI stattdessen beendet, außer für [`claude auth` Unterbefehle](#enforce-fail-closed-startup) |
| Benutzer authentifiziert sich mit einer anderen Organisation                       | Einstellungen werden nicht für Konten außerhalb der verwalteten Organisation bereitgestellt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| Benutzer konfiguriert einen [Drittanbieter-Modellprovider](#platform-availability) | Serververwaltete Einstellungen werden umgangen. Dies umfasst das Setzen von `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_MANTLE`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, `CLAUDE_CODE_USE_ANTHROPIC_AWS` oder einer nicht standardmäßigen `ANTHROPIC_BASE_URL`                                                                                                                                                                                                                                                                                                |
| Netzwerkverkehr wird abgefangen oder umgeleitet                                    | Deaktivierte TLS-Validierung oder abgefangener Verkehr kann die Einstellungen ändern, die der Client erhält                                                                                                                                                                                                                                                                                                                                                                                                                                                                |

Um Laufzeitkonfigurationsänderungen zu erkennen, verwenden Sie [`ConfigChange` hooks](/docs/de/hooks#configchange), um Änderungen zu protokollieren oder nicht autorisierte Änderungen zu blockieren, bevor sie wirksam werden.

Um einzuschränken, auf welche Organisationen Ihre Benutzer mit den vom Client bereitgestellten Anmeldedaten zugreifen können, siehe [Netzwerkzugriffskontrolle mit Tenant Restrictions durchsetzen](https://support.claude.com/en/articles/13198485-enforce-network-level-access-control-with-tenant-restrictions) im Claude Help Center. Für stärkere Durchsetzungsgarantien verwenden Sie [endpunktverwaltete Einstellungen](/docs/de/settings#settings-files) auf Geräten, die in einer MDM-Lösung registriert sind.

<h2 id="see-also">
  Siehe auch
</h2>

Verwandte Seiten zur Verwaltung der Claude Code-Konfiguration:

* [Einstellungen](/docs/de/settings): vollständige Konfigurationsreferenz einschließlich aller verfügbaren Einstellungen
* [Endpunktverwaltete Einstellungen](/docs/de/settings#settings-files): verwaltete Einstellungen, die von der IT auf Geräten bereitgestellt werden
* [Authentifizierung](/docs/de/authentication): Einrichtung des Benutzerzugriffs auf Claude Code
* [Sicherheit](/docs/de/security): Sicherheitsvorkehrungen und Best Practices
