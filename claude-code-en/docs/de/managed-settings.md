> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Verwaltete Einstellungen bereitstellen

> Stellen Sie verwaltete Einstellungen auf jedem Entwicklerrechner bereit: Bereitstellungsmechanismen pro Betriebssystem, wie Claude Code verwaltete Quellen kombiniert und wie Sie die Durchsetzung überprüfen.

Verwaltete Einstellungen sind die Einstellungen, die Ihre Organisation auf jedem Entwicklerrechner bereitstellt. Claude Code wendet sie über alle anderen Ebenen an, sodass kein Benutzer-, Projekt-, lokaler oder `--settings`-Wert sie außer Kraft setzt, mit Ausnahme einiger weniger [sicherheitsrelevanter Ausnahmen](/docs/de/settings#exceptions-to-managed-settings-precedence), bei denen ein strengerer Wert von einer niedrigeren Ebene dennoch zählt.

Diese Seite ist für den Administrator, der verwaltete Einstellungen bereitstellt oder debuggt, warum eine nicht angewendet wird. Um zu entscheiden, was durchgesetzt werden soll, beginnen Sie mit der Tabelle [Entscheiden Sie, was durchgesetzt werden soll](/docs/de/admin-setup#decide-what-to-enforce). Für den claude.ai-Konsolenpfad siehe [Serververwaltete Einstellungen](/docs/de/server-managed-settings). Für die Datei, in die die eigenen Werte eines Entwicklers gehen, siehe [Einstellungen](/docs/de/settings).

<h2 id="deploy-a-managed-settings-file">
  Stellen Sie eine verwaltete Einstellungsdatei bereit
</h2>

Dies ist die schnellste Möglichkeit, eine Richtlinie auf jedem Rechner einzuführen: eine `managed-settings.json`-Datei. Wenn Sie noch nicht entschieden haben, wie Sie verwaltete Einstellungen bereitstellen möchten, oder Ihre Geräte unter MDM verwaltet werden oder Entwickler Cloud-Sitzungen ausführen, lesen Sie zuerst [Wählen Sie einen Bereitstellungsmechanismus](#choose-a-delivery-mechanism).

<Steps>
  <Step title="Schreiben Sie managed-settings.json">
    Schreiben Sie eine `managed-settings.json`, die die Schlüssel enthält, die Sie durchsetzen möchten, in der gleichen JSON-Form wie `settings.json`. Die Tabelle [Entscheiden Sie, was durchgesetzt werden soll](/docs/de/admin-setup#decide-what-to-enforce) listet die Schlüssel hinter jedem Steuerelement auf, und jeder Eintrag in der [Einstellungsreferenz](/docs/de/settings-reference) sagt, ob eine verwaltete Quelle ihn setzen kann. Diese Datei blockiert zwei Dateileseoperationen, deaktiviert den Bypass-Modus und lässt Claude Code Berechtigungsregeln aus Benutzer-, Projekt- und lokalen Dateien sowie aus `--allowedTools` ignorieren:

    ```json managed-settings.json theme={null}
    {
      "permissions": {
        "deny": [
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true
    }
    ```

    Ein vollständigeres Beispiel, das die Form weiterer verwalteter Schlüssel zeigt, einschließlich der Anmeldemethode, Modelle, MCP-Server und Marktplätze, finden Sie unter [Die verwalteten Einstellungen einer Organisation](/docs/de/settings-example#an-organizations-managed-settings).
  </Step>

  <Step title="Platzieren Sie die Datei auf jedem Rechner">
    Speichern Sie die Datei als `managed-settings.json` im Systemverzeichnis für das Betriebssystem, indem Sie beliebige Tools verwenden, die bereits Dateien auf Ihrer Flotte platzieren:

    * **macOS**: `/Library/Application Support/ClaudeCode/managed-settings.json`
    * **Linux und WSL**: `/etc/claude-code/managed-settings.json`
    * **Windows**: `C:\Program Files\ClaudeCode\managed-settings.json`
  </Step>

  <Step title="Bestätigen Sie, dass die Richtlinie angewendet wurde">
    Führen Sie auf einem Rechner `/status` in Claude Code aus. Die Zeile `Setting sources` zeigt `Enterprise managed settings (file)`. Führen Sie dann einen Rollout für den Rest der Flotte durch; [Überprüfen Sie, dass eine Richtlinie in Kraft ist](#check-that-a-policy-is-in-force) behandelt, worauf Sie achten sollten, wenn die Zeile fehlt.
  </Step>
</Steps>

<span id="managed-settings-delivery" />

<span id="delivery-mechanisms" />

<h2 id="choose-a-delivery-mechanism">
  Wählen Sie einen Bereitstellungsmechanismus
</h2>

Die Datei in den obigen Schritten ist eine von vier Möglichkeiten, verwaltete Einstellungen auf einen Rechner zu bringen. Jeder Mechanismus trägt die gleichen Richtlinienschlüssel wie eine `settings.json`-Datei, daher gilt die [Einstellungsreferenz](/docs/de/settings-reference) für alle. Einige Schlüssel sind an bestimmte Quellen gebunden, und die Zeile „Scope" jedes Eintrags sagt, welche:

* **Bereitstellungssteuerelemente**: [`policyHelper`](/docs/de/settings-reference#policyhelper), [`wslInheritsWindowsSettings`](/docs/de/settings-reference#wslinheritswindowssettings) und [`managedSourcesBehavior`](/docs/de/settings-reference#managedsourcesbehavior)
* **Gateway-Anmeldeschlüssel**: [`forceLoginGatewayUrl`](/docs/de/settings-reference#forcelogingatewayurl), [`gatewayInternalNetworks`](/docs/de/settings-reference#gatewayinternalnetworks) und der `"gateway"`-Wert von [`forceLoginMethod`](/docs/de/settings-reference#forceloginmethod)

Eine verwaltete Einstellungsdatei, ein MDM-Profil oder die claude.ai-Konsole wendet eine Richtlinie auf alle an, die sie erreicht. Um einer Gruppe von Entwicklern eine andere Richtlinie zu geben, stellen Sie eine andere Datei oder ein anderes Profil für diese Gruppe bereit; die claude.ai-Konsole [kann noch keine Gruppe als Ziel festlegen](/docs/de/server-managed-settings#current-limitations), während ein selbstgehostetes [Claude-Apps-Gateway](/docs/de/claude-apps-gateway) verwaltete Einstellungen pro IdP-Gruppe bereitstellt.

Wenn mehr als ein Mechanismus eine Richtlinie auf dem gleichen Rechner bereitstellt, verwendet Claude Code standardmäßig einen und ignoriert die anderen. [Wie Claude Code verwaltete Quellen kombiniert](#how-claude-code-combines-managed-sources) gibt die Reihenfolge und das Opt-in an, das jede Quelle anwendet.

Die MDM- und Dateireihen werden zusammen als endpunktverwaltete Einstellungen bezeichnet, da die Richtlinie auf dem Gerät des Entwicklers gespeichert ist, im Gegensatz zur serververwalteten Reihe, wo Claude Code sie abruft.

Wählen Sie einen Mechanismus danach aus, wie Sie bereits Geräte verwalten, indem Sie die folgende Tabelle verwenden.

| Mechanismus                                                   | Wie Sie ihn bereitstellen                                                                                                                                                                                                           | Wann Claude Code ihn liest                                                                                                                                                                                                                                                                                             | Verwenden Sie ihn, wenn                                                                                            |
| :------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------- |
| [Serververwaltete Einstellungen](/docs/de/server-managed-settings) | In der claude.ai-Verwaltungskonsole oder auf einem selbstgehosteten [Claude-Apps-Gateway](/docs/de/claude-apps-gateway)                                                                                                                  | Beim Start abgerufen und stündlich abgefragt; siehe [Änderungen, die Genehmigung benötigen](#where-and-when-a-policy-applies)                                                                                                                                                                                          | Sie möchten einen Ort, um die Richtlinie für eine claude.ai-Organisation zu ändern, ohne jeden Rechner zu berühren |
| MDM oder Richtlinie auf Betriebssystemebene                   | Als macOS-Konfigurationsprofil oder Windows-`HKLM`-Registrierungswert, über Jamf, Intune, Gruppenrichtlinie oder ein ähnliches Tool; siehe [wo jeder Mechanismus die Richtlinie speichert](#where-each-mechanism-stores-the-policy) | Beim Start gelesen und alle 30 Minuten auf Änderungen überprüft                                                                                                                                                                                                                                                        | Sie verwalten bereits Geräte mit MDM oder Gruppenrichtlinie                                                        |
| Dateibasiert                                                  | Als `managed-settings.json` in einem Systemverzeichnis auf jedem Rechner; siehe [wo jeder Mechanismus die Richtlinie speichert](#where-each-mechanism-stores-the-policy)                                                            | Beim Start gelesen und neu geladen, wenn sich eine Datei ändert                                                                                                                                                                                                                                                        | Rechner ohne MDM, Linux-Hosts oder Images, die Sie selbst erstellen                                                |
| HKCU-Registrierung, Windows und WSL                           | Als Windows-`HKCU`-Registrierungswert; siehe [wo jeder Mechanismus die Richtlinie speichert](#where-each-mechanism-stores-the-policy)                                                                                               | Beim Start gelesen und alle 30 Minuten auf Änderungen überprüft; Claude Code verwendet ihn nur, wenn keine andere verwaltete Quelle einen Richtlinienschlüssel bereitstellt und keine [vom Host bereitgestellten übergeordneten Einstellungen](#let-an-embedding-host-add-policy) einen restriktiven Schlüssel liefern | Sie können den Schlüssel auf Maschinenebene `HKLM` nicht schreiben                                                 |

Starter-Vorlagen für Jamf, Iru, Intune und Gruppenrichtlinie befinden sich im [MDM-Beispiel-Repository](https://github.com/anthropics/claude-code/tree/main/examples/mdm).

Für verwaltete MCP-Server, die Sie neben einem dieser über `managed-mcp.json` bereitstellen oder durch den Schlüssel [`managedMcpServers`](/docs/de/settings-reference#managedmcpservers) bereitstellen, siehe [Verwaltete MCP-Konfiguration](/docs/de/managed-mcp).

<h3 id="where-and-when-a-policy-applies">
  Wo und wann eine Richtlinie angewendet wird
</h3>

Eine bereitgestellte Richtlinie erreicht die Sitzungen des Entwicklers wie folgt:

* **Oberflächen**: Auf dem Rechner des Entwicklers lesen das Terminal, die VS Code- und JetBrains-Erweiterungen, die Registerkarte „Code" der Desktop-App und [Agent SDK](/docs/de/agent-sdk/typescript)-Sitzungen alle diese Quellen. Agent SDK-Sitzungen laden verwaltete Einstellungen auch dann, wenn `settingSources` die Benutzer-, Projekt- und lokalen Dateien ausschließt.
* **Cloud-Sitzungen**: Eine Sitzung in einer von Anthropic gehosteten Umgebung liest kein MDM-Profil oder keine Datei eines Geräts, daher muss die Richtlinie dafür aus serververwalteten Einstellungen stammen. Eine Sitzung in einer [selbstgehosteten Umgebung](/docs/de/self-hosted-environments) liest auch die verwaltete Einstellungsdatei in ihrem Runner-Image, standardmäßig nur wenn serververwaltete Einstellungen keinen Richtlinienschlüssel bereitstellen, mit Ausnahme der [Schlüssel, die Claude Code aus jeder Admin-Quelle liest](#keys-read-from-every-admin-source). [Wie Claude Code verwaltete Quellen kombiniert](#how-claude-code-combines-managed-sources) behandelt das Opt-in, das beide anwendet.
* **Cowork-Sitzungen**: [Cowork](https://claude.com/docs/cowork/overview) in der Claude Desktop-App führt ihre Sitzungen auf Claude Code aus. In einer Cowork-Sitzung ruft Claude Code niemals serververwaltete Einstellungen aus der claude.ai-Verwaltungskonsole ab, auch wenn sich der Benutzer mit einem Team- oder Enterprise-Konto anmeldet, daher hängt die angewendete Richtlinie davon ab, wo die Sitzung ausgeführt wird:

  * **Auf dem Rechner des Benutzers**: Standardmäßig liest Claude Code in einer Cowork-Sitzung die MDM- oder Richtlinie auf Betriebssystemebene und die verwaltete Einstellungsdatei auf diesem Gerät, daher stellen Sie die Richtlinie dort bereit.
  * **In einer vollständigen VM-Sandbox**: Wenn Ihre Claude Desktop-verwaltete Konfiguration [`requireCoworkFullVmSandbox`](https://claude.com/docs/third-party/claude-desktop/configuration#requirecoworkfullvmsandbox) setzt, wird Claude Code in einer virtuellen Maschine ausgeführt, in der die MDM-Richtlinie und die verwaltete Einstellungsdatei des Geräts nicht vorhanden sind.
  * **Remote-Cowork-Sitzungen**: Diese werden auf von Anthropic verwalteten VMs ausgeführt, wo Claude Code keine Geräterichtlinie zum Lesen hat.

  Überall dort, wo die Sitzung ausgeführt wird, wendet claude.ai die Listen [`strictKnownMarketplaces`](/docs/de/settings-reference#strictknownmarketplaces) und [`blockedMarketplaces`](/docs/de/settings-reference#blockedmarketplaces) der Verwaltungskonsole selbst an, wenn jemand einen Marketplace aus einem Git-Repository auf claude.ai oder aus **Anpassen** in der Cowork-Registerkarte hinzufügt. [Wie Einschränkungen funktionieren](/docs/de/plugins/org#restrict-what-users-can-install) beschreibt diese Überprüfung. Die [Oberflächenabdeckung](/docs/de/model-config#surface-coverage) vergleicht Cowork mit den anderen Oberflächen.
* **Laufende Sitzungen**: Die meisten Änderungen erreichen eine laufende Sitzung nach dem Zeitplan in der [Bereitstellungsmechanismus-Tabelle](#choose-a-delivery-mechanism), ohne einen Neustart.
  * Änderungen an [`forceRemoteSettingsRefresh`](/docs/de/settings-reference#forceremotesettingsrefresh), [`requiredMinimumVersion`](/docs/de/settings-reference#requiredminimumversion) und [einigen benutzerbearbeitbaren Schlüsseln](/docs/de/settings#when-edits-take-effect) treten beim nächsten Sitzungsstart in Kraft.
  * Ein neuer oder geänderter [`policyHelper`](/docs/de/settings-reference#policyhelper)-Eintrag tritt beim nächsten Start in Kraft. Wenn serververwaltete Einstellungen den Helper bei diesem Start überschatten, wird der Helper ausgeführt, sobald ein Abruf meldet, dass diese Einstellungen entfernt wurden.
* **Änderungen, die Genehmigung benötigen**: Abgesehen von den [Updates, die auf den nächsten Start warten](/docs/de/server-managed-settings#fetch-and-caching-behavior), wartet eine serververwaltete Änderung an einer Einstellung, die [Genehmigung benötigt](/docs/de/server-managed-settings#security-approval-dialogs), wie ein Hook oder eine `env`-Variable, darauf, dass der Entwickler den Dialog in einer interaktiven Sitzung akzeptiert, und wird für den aktuellen Lauf in einer Sitzung angewendet, die eine IDE-Erweiterung oder das Agent SDK hostet. Andere serververwaltete Änderungen werden bei der nächsten Abfrage angewendet.
* **Langlebige Sitzungen**: Eine Sitzung, die wochenlang offen bleibt, kann immer noch hinter einem Rollout zurückbleiben. [`requiredMinimumVersion`](/docs/de/settings-reference#requiredminimumversion) blockiert den Start einer veralteten Binärdatei und beendet keine bereits laufende Sitzung.

<span id="format-the-policy-for-each-platform" />

<h3 id="where-each-mechanism-stores-the-policy">
  Wo jeder Mechanismus die Richtlinie speichert
</h3>

Die Schlüssel sind überall gleich, aber jeder Mechanismus speichert sie an einem anderen Ort und in einer anderen Form:

* **Serververwaltete**: Anthropics Server oder Ihr Gateway halten die Richtlinie. Claude Code behält einen lokalen Cache, den es beim Start anwendet und [bei jedem erfolgreichen Abruf ersetzt](/docs/de/server-managed-settings#security-considerations).
* **macOS-Konfigurationsprofil**: die verwaltete Präferenzdomäne `com.anthropic.claudecode`. Verwenden Sie die gleichen Top-Level-Schlüssel wie `managed-settings.json`, mit verschachtelten Einstellungen als Wörterbücher und Listen als plist-Arrays.
* **Windows HKLM-Registrierung**: das JSON als `REG_SZ`- oder `REG_EXPAND_SZ`-Wert namens `Settings` unter `HKLM\SOFTWARE\Policies\ClaudeCode`.
* **Dateibasiert**: `managed-settings.json`, ein optionales `managed-settings.d/`-Verzeichnis und `managed-mcp.json` im Systemverzeichnis: `/Library/Application Support/ClaudeCode/` auf macOS, `/etc/claude-code/` auf Linux und WSL und `C:\Program Files\ClaudeCode\` auf Windows. Claude Code liest den Legacy-Windows-Pfad `C:\ProgramData\ClaudeCode\managed-settings.json` nicht.
* **Windows HKCU-Registrierung**: der gleiche `Settings`-Wert unter `HKCU\SOFTWARE\Policies\ClaudeCode`.

<h3 id="split-a-file-based-policy-across-teams">
  Teilen Sie eine dateibasierte Richtlinie auf Teams auf
</h3>

Wenn mehrere Teams Teile einer Richtlinie besitzen, platzieren Sie jeden Teil in seiner eigenen Datei in `managed-settings.d/`, neben `managed-settings.json` im gleichen Systemverzeichnis, anstatt eine gemeinsame Datei zu bearbeiten.

Claude Code führt `managed-settings.json` zuerst zusammen, dann jede `*.json`-Datei im Verzeichnis in alphabetischer Reihenfolge. Benennen Sie die Dateien mit numerischen Präfixen, um die Reihenfolge zu steuern, wie `10-telemetry.json` und `20-security.json`. Claude Code ignoriert versteckte Dateien und Dateien, die nicht auf `.json` enden.

Wenn zwei Dateien den gleichen Schlüssel setzen, kombiniert Claude Code sie nach diesen Regeln:

* **Einzelne Werte**, wie `"model": "opus"` oder `"cleanupPeriodDays": 7`: Der Wert der späteren Datei ersetzt den früheren
* **Listen**, wie `permissions.deny` oder `sandbox.network.allowedDomains`: Die beiden Listen kombinieren, mit entfernten Duplikaten
* **Verschachtelte Blöcke**, wie `env` oder `sandbox`: Die beiden Blöcke führen Schlüssel für Schlüssel zusammen, und jeder Schlüssel darin folgt den gleichen Regeln
* **`fallbackModel`**: Die spätere Kette ersetzt die frühere ganz
* **[`extraKnownMarketplaces`](/docs/de/settings-reference#extraknownmarketplaces) und [`managedMcpServers`](/docs/de/settings-reference#managedmcpservers)**: Ein späterer Eintrag mit dem gleichen Namen ersetzt den früheren ganz
* **[`modelPicker`](/docs/de/settings-reference#modelpicker)**: Die spätere Aufstellung ersetzt die frühere ganz

<span id="precedence-within-the-managed-tier" />

<span id="which-managed-source-claude-code-uses" />

<h2 id="how-claude-code-combines-managed-sources">
  Wie Claude Code verwaltete Quellen kombiniert
</h2>

Wenn Ihre Organisation mehr als eine verwaltete Quelle auf demselben Computer bereitstellt, entscheidet der Schlüssel [`managedSourcesBehavior`](/docs/de/settings-reference#managedsourcesbehavior), was Claude Code mit den anderen tut:

* **`"first-wins"`, die Standardeinstellung**: Claude Code verwendet die höchstrangige Quelle, die mindestens einen Richtlinienschlüssel bereitstellt, und ignoriert die übrigen, anstatt sie zu zusammenzuführen, mit Ausnahme der Schlüssel in [Schlüssel, die von jeder Admin-Quelle gelesen werden](#keys-read-from-every-admin-source). Claude Code zeigt keine Warnung für die Quellen an, die es überspringt; `/status` [nennt die Quelle, die es verwendet hat, und die, die es übersprungen hat](#read-the-source-in-/status).
* **`"merge"`**: Claude Code wendet jede Admin-Quelle an, die einen Richtlinienschlüssel bereitstellt, und kombiniert sie nach Art des Schlüssels: Bei den meisten Schlüsseln gilt der Wert der höherrangigen Quelle, Listen werden vereinigt, und Sperren nehmen den strengsten Wert. [Zusammensetzen jeder verwalteten Quelle](#compose-every-managed-source) sagt, wo der Schlüssel gesetzt wird und wie jede Art von Schlüssel kombiniert wird. Erfordert Claude Code v2.1.242 oder später.

Beide Einstellungen ordnen die Quellen auf die gleiche Weise. Diese Begriffe treten in diesem Abschnitt wiederholt auf:

* **Richtlinienschlüssel**: jeder Einstellungsschlüssel außer den zwei Steuerschlüsseln, [`wslInheritsWindowsSettings`](/docs/de/settings-reference#wslinheritswindowssettings) und [`managedSourcesBehavior`](/docs/de/settings-reference#managedsourcesbehavior). Eine verwaltete Einstellungsdatei oder MDM-Richtlinie, die nur diese enthält, zählt nicht, und Claude Code geht zur nächsten Quelle über.
* **Admin-Quelle**: eine der ersten drei Quellen unten. Die HKCU-Registrierung ist vom Benutzer beschreibbar und ist keine.

Claude Code prüft die Quellen in dieser Reihenfolge, höchste Priorität zuerst:

1. Remote-Einstellungen, bereitgestellt von claude.ai als [servergesteuerte Einstellungen](/docs/de/server-managed-settings) oder durch ein [Claude-Apps-Gateway](/docs/de/claude-apps-gateway). Claude Code ruft diese Quelle nur ab, wenn die Sitzung sich direkt mit einem [berechtigten Login oder Schlüssel](/docs/de/server-managed-settings#platform-availability) bei der API von Anthropic authentifiziert oder sich mit `/login` bei einem Gateway anmeldet. Bei anderen Anbietern oder wenn `ANTHROPIC_BASE_URL` auf etwas anderes als die API von Anthropic verweist, beginnt es bei der nächsten Quelle
2. MDM- oder Betriebssystem-Richtlinien: der macOS-plist oder der HKLM-Registrierungsschlüssel
3. Verwaltete Einstellungsdateien, `managed-settings.d/*.json` und `managed-settings.json` zusammengeführt
4. Die HKCU-Registrierung unter Windows und unter WSL, sobald der HKLM-Registrierungsschlüssel oder die Windows-Verwaltungsdatei [`wslInheritsWindowsSettings`](/docs/de/settings-reference#wslinheritswindowssettings) aktiviert und der HKCU-Wert es auch setzt. Claude Code liest sie nur, wenn keine Quelle darüber einen Richtlinienschlüssel bereitstellt und keine [vom Host bereitgestellten übergeordneten Einstellungen](#let-an-embedding-host-add-policy) einen restriktiven Schlüssel liefern

Dieses Diagramm zeigt die Rangfolge mit Beispielen der Schlüssel zwischen Quellen, die Claude Code aus den ersten drei Quellen unter einer der beiden Einstellungen liest:

<img src="https://mintcdn.com/claude-code/zuWID2B-Rxm8DEC8/images/managed-source-precedence.svg?fit=max&auto=format&n=zuWID2B-Rxm8DEC8&q=85&s=53f6be49f06eff48e01422c8ae1bc2e6" className="dark:hidden" alt="Diagramm, das die vier verwalteten Einstellungsquellen zeigt, die von Remote-Einstellungen oben über MDM, verwaltete Einstellungsdateien und die HKCU-Registrierung unten rangiert sind. Standardmäßig liefert die erste Quelle mit einem Richtlinienschlüssel die Richtlinie und die übrigen werden übersprungen; mit managedSourcesBehavior auf merge gesetzt, trägt jede Admin-Quelle mit einem Richtlinienschlüssel bei, kombiniert nach Art des Schlüssels, und die HKCU-Registrierung bleibt außen vor. Ein Seitenpanel zeigt, dass Schlüssel zwischen Quellen wie die Sandbox-Sperren, forceRemoteSettingsRefresh und die pro-Variable env-Zusammenführung von jeder Admin-Quelle gelesen werden, was die HKCU-Registrierung ausschließt." width="680" height="330" data-path="images/managed-source-precedence.svg" />

<img src="https://mintcdn.com/claude-code/zuWID2B-Rxm8DEC8/images/managed-source-precedence-dark.svg?fit=max&auto=format&n=zuWID2B-Rxm8DEC8&q=85&s=ae407a9a08a3d680e80cf1a2af845d71" className="hidden dark:block" alt="Diagramm, das die vier verwalteten Einstellungsquellen zeigt, die von Remote-Einstellungen oben über MDM, verwaltete Einstellungsdateien und die HKCU-Registrierung unten rangiert sind. Standardmäßig liefert die erste Quelle mit einem Richtlinienschlüssel die Richtlinie und die übrigen werden übersprungen; mit managedSourcesBehavior auf merge gesetzt, trägt jede Admin-Quelle mit einem Richtlinienschlüssel bei, kombiniert nach Art des Schlüssels, und die HKCU-Registrierung bleibt außen vor. Ein Seitenpanel zeigt, dass Schlüssel zwischen Quellen wie die Sandbox-Sperren, forceRemoteSettingsRefresh und die pro-Variable env-Zusammenführung von jeder Admin-Quelle gelesen werden, was die HKCU-Registrierung ausschließt." width="680" height="330" data-path="images/managed-source-precedence-dark.svg" />

<h3 id="keys-read-from-every-admin-source">
  Schlüssel, die von jeder Admin-Quelle gelesen werden
</h3>

Unter der Standardeinstellung `"first-wins"` liest Claude Code die meisten Schlüssel nur aus der [Quelle, die es ausgewählt hat](#how-claude-code-combines-managed-sources), und ignoriert einen Wert in einer niedrigerrangigen Quelle, selbst wenn die ausgewählte Quelle diesen Schlüssel nicht setzt.

Einige Schlüssel funktionieren anders. Claude Code liest sie aus jeder Admin-Quelle, sodass eine niedrigerrangige MDM-Richtlinie oder verwaltete Einstellungsdatei sie immer noch setzen kann, wenn die ausgewählte Quelle dies nicht tut. Claude Code lässt die vom Benutzer beschreibbare HKCU-Registrierung aus dieser Überprüfung aus; wenn HKCU die einzige Quelle ist und kein Host übergeordnete Einstellungen liefert, gilt HKCU wie jede ausgewählte Quelle.

Die Schlüssel zwischen Quellen umfassen:

* `sandbox.network.allowManagedDomainsOnly` und `sandbox.filesystem.allowManagedReadPathsOnly`: ein `true` in einer beliebigen Admin-Quelle aktiviert die Sperre. Während eine Sperre aktiv ist, vereinigt Claude Code die Zulassungsliste, die sie sperrt, `sandbox.network.allowedDomains` zusammen mit `WebFetch(domain:...)` Zulassungsregeln oder `sandbox.filesystem.allowRead` über jede Admin-Quelle hinweg. Ohne die Sperre behandelt Claude Code die Zulassungsliste wie jeden anderen Schlüssel, sodass unter `"first-wins"` die Zulassungsliste einer nicht ausgewählten Admin-Quelle ignoriert wird
* `allowAllClaudeAiMcps`
* `allowManagedMcpServersOnly`: ein `true` in einer beliebigen Admin-Quelle aktiviert die MCP-Zulassungslisten-Sperre. Während die Sperre aktiv ist, kommt die verwaltete `allowedMcpServers` Liste aus der höchstrangigen Admin-Quelle, die eine setzt. Eine servergesteuerte Liste ersetzt die Liste einer niedrigeren Quelle, anstatt sich mit ihr zu kombinieren.

  Wenn keine Admin-Quelle eine Liste setzt, wird jeder Server geladen, der die Ablehnungsliste passiert, es sei denn, [übergeordnete Einstellungen](#let-an-embedding-host-add-policy) liefern eine Liste.

  Ohne die Sperre liest Claude Code `allowedMcpServers` aus der verwalteten Quelle, die es anwendet, sodass unter `"first-wins"` die Liste einer nicht ausgewählten Admin-Quelle ignoriert wird. Erfordert Claude Code v2.1.273 oder später
* `deniedMcpServers` und [`disableClaudeAiConnectors`](/docs/de/settings-reference#disableclaudeaiconnectors): ein Eintrag oder ein `true` in einer beliebigen Admin-Quelle gilt. Erfordert Claude Code v2.1.273 oder später
* Die Sandbox-Binärpfade `sandbox.bwrapPath` und `sandbox.socatPath`
* Die Sandbox `ripgrep` Binärdatei, [`sandbox.ripgrep`](/docs/de/settings-reference#sandbox-ripgrep)
* `sandbox.filesystem.disabled` und `sandbox.network.strictAllowlist`
* [`useAutoModeDuringPlan`](/docs/de/settings-reference#useautomodeduringplan), [`syncClaudeAiSkills`](/docs/de/settings-reference#syncclaudeaiskills) und [`syncClaudeAiPlugins`](/docs/de/settings-reference#syncclaudeaiplugins), wobei ein `false` aus einer beliebigen Admin-Quelle das Verhalten ausschaltet. Ein `false` in den Benutzer- oder lokalen Einstellungen des Entwicklers schaltet es auch aus; jeder Schlüssel kann nur verweigern
* [`enableArtifact`](/docs/de/settings-reference#enableartifact), wobei ein `false` aus einer beliebigen Admin-Quelle das [Artifact-Tool](/docs/de/artifacts) ausschaltet. Ein `false` in den Benutzer-, Projekt- oder lokalen Einstellungen des Entwicklers schaltet es auch aus, und keine Quelle schaltet es wieder ein; siehe [welche niedrigeren Werte immer noch zählen](/docs/de/settings#exceptions-to-managed-settings-precedence). Erfordert Claude Code v2.1.242 oder später
* [`maxEffortLevel`](/docs/de/settings-reference#maxeffortlevel), wobei die niedrigste Obergrenze in einer beliebigen Admin-Quelle gilt. Wenn ein Entwickler eine niedrigere Obergrenze in seinen eigenen Einstellungen oder mit `--settings` setzt, wendet Claude Code diese an; keine Quelle kann die Obergrenze erhöhen. Erfordert Claude Code v2.1.267 oder später
* Ein Commit-Trailer-Opt-out in `attribution` oder im veralteten `includeCoAuthoredBy` aus einer beliebigen Ebene
* [`forceRemoteSettingsRefresh`](/docs/de/server-managed-settings)
* `env`, pro Variable über die Admin-Quellen zusammengeführt: jede Variable kommt aus der höchstpriorisierten Quelle, die sie definiert, sodass niedrigere Quellen Variablen ausfüllen, die höhere nicht setzen. Einige Variablen folgen ihren eigenen Regeln; [Pro-Schlüssel-Ausnahmen über verwaltete Quellen](/docs/de/server-managed-settings#per-key-exceptions-across-managed-sources) nennt jede. Erfordert Claude Code v2.1.223 oder später. Vor v2.1.223 wendete Claude Code nur den gesamten `env`-Block der ausgewählten Quelle an

Die [Gateway-Login-Schlüssel](#choose-a-delivery-mechanism) folgen einer separaten Regel. Claude Code liest sie niemals aus servergesteuerten Einstellungen, sodass während servergesteuerte Einstellungen die ausgewählte Quelle sind, die höchstrangige Admin-Quelle auf dem Computer, die einen Richtlinienschlüssel trägt, sie immer noch liefert. Ein Wert in einer Admin-Quelle, die unter dieser rangiert, oder in der HKCU-Registrierung wird ignoriert.

Wenn eine Admin-Quelle `allowManagedMcpServersOnly` setzt oder eine `allowedMcpServers` Liste setzt und dieser Wert nicht der ist, der gerade gültig ist, nennen `/status` und `claude doctor` diese Quelle und diesen Schlüssel.

<h3 id="compose-every-managed-source">
  Zusammensetzen jeder verwalteten Quelle
</h3>

Um Claude Code jede Admin-Quelle anwenden zu lassen, die Ihre Organisation bereitstellt, setzen Sie [`managedSourcesBehavior`](/docs/de/settings-reference#managedsourcesbehavior) auf `"merge"` in der höchstrangigen Quelle, die Sie bereitstellen. Claude Code liest den Schlüssel nur aus der höchstrangigen Quelle, die entweder den Schlüssel oder einen Richtlinienschlüssel trägt, sodass eine niedrigere Quelle sich nicht selbst zum Zusammenführen mit der Quelle darüber anmelden kann, und ein Computer, der niemals servergesteuerte Einstellungen erhält, benötigt den Schlüssel auch in seinem MDM-Profil. Die vom Benutzer beschreibbare HKCU-Registrierung wird niemals mit einer anderen Quelle zusammengeführt. Erfordert Claude Code v2.1.242 oder später.

Unter `"merge"` fügt Claude Code die Listeneinträge einer niedrigeren Quelle, wie `permissions.allow` Regeln und Hooks, zur Richtlinie hinzu, sodass aktivieren Sie es nur, wenn jede Quelle, die unter Ihrer höchsten rangiert, unter der Kontrolle eines Administrators steht.

Diese Tabelle zeigt, wie Claude Code jede Art von Schlüssel unter `"merge"` kombiniert. Der [`managedSourcesBehavior` Eintrag](/docs/de/settings-reference#managedsourcesbehavior) nennt jeden Schlüssel in drei der Zeilen: Restriktions-Zulassungslisten, Werte, die ganz genommen werden, und Schlüssel, die nur aus der höchstrangigen Quelle gelesen werden.

| Art des Schlüssels                                              | Wie Claude Code es kombiniert                                                                                                                                                                  | Beispiele                                                                                                                |
| :-------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------- |
| Listen                                                          | Kombiniert die Einträge aus jeder Quelle                                                                                                                                                       | `permissions.allow`, `hooks`, `sandbox.network.allowedDomains`, `deniedMcpServers`                                       |
| Sperren                                                         | Wendet den strengsten Wert an, den eine Quelle setzt; ein lockererer Wert gilt nur aus der höchstrangigen Quelle                                                                               | `allowManagedHooksOnly`, `permissions.disableBypassPermissionsMode`, `crossSessionInbound`                               |
| Restriktions-Zulassungslisten                                   | Nimmt die Liste ganz aus der höchstrangigen Quelle, die sie setzt, ohne Einträge aus niedrigeren Quellen hinzuzufügen                                                                          | `availableModels`, `allowedMcpServers`, `strictKnownMarketplaces`, `allowedChannelPlugins` und die `fallbackModel` Kette |
| Werte, die ganz genommen werden                                 | Nimmt den Wert ganz aus der höchstrangigen Quelle, die ihn setzt, ohne Einträge oder Felder aus niedrigeren Quellen zu kombinieren                                                             | `sandbox.credentials.awsPairs`, `sandbox.ripgrep`                                                                        |
| Bereitgestellte MCP-Server                                      | Kombiniert die Servernamen aus jeder Quelle; wenn zwei Quellen denselben Namen setzen, wendet den gesamten Eintrag der höherrangigen Quelle an                                                 | `managedMcpServers`                                                                                                      |
| Schlüssel, die nur aus der höchstrangigen Quelle gelesen werden | Ignoriert den Schlüssel in jeder niedrigeren Quelle, selbst wenn die höchstrangige Quelle ihn nicht setzt                                                                                      | Credential-Helfer wie `apiKeyHelper`, Login-Pins wie `forceLoginOrgUUID`, `modelPicker`, `permissions.defaultMode`       |
| `env`                                                           | Führt pro Variable über Admin-Quellen unter einer der beiden Einstellungen zusammen, wie [Schlüssel, die von jeder Admin-Quelle gelesen werden](#keys-read-from-every-admin-source) beschreibt |                                                                                                                          |
| Jeder andere Schlüssel                                          | Nimmt den Wert aus der höchstrangigen Quelle, die ihn setzt                                                                                                                                    | `model`, `cleanupPeriodDays`                                                                                             |

Um zu bestätigen, welche Quellen auf einem Computer kombiniert wurden, [lesen Sie die Zeile `Setting sources` in `/status`](#read-the-source-in-/status); dieser Abschnitt sagt, was jedes Label bedeutet.

<h3 id="compute-the-policy-with-a-helper-program">
  Berechnen Sie die Richtlinie mit einem Hilfsprogramm
</h3>

Ein [`policyHelper`](/docs/de/settings-reference#policyhelper) ist eine ausführbare Datei, die Ihre MDM-Richtlinie oder verwaltete Einstellungsdatei benennt, und Claude Code führt sie aus, um verwaltete Einstellungen beim Start zu berechnen. Wenn die ausgewählte Quelle einen konfiguriert und der Helper ein `managedSettings` Objekt ausgibt, ändert diese Ausgabe, was Claude Code liest:

* **Das ausgegebene `managedSettings` Objekt ist die einzige verwaltete Einstellung für die Sitzung**, einschließlich für die [Schlüssel, die es sonst aus jeder Admin-Quelle liest](#keys-read-from-every-admin-source), mit Ausnahme von [`forceRemoteSettingsRefresh`, das seine eigene Startregel hat](/docs/de/settings-reference#forceremotesettingsrefresh)

Für welche Helper-Läufe fehlschlagen und was Claude Code tut, wenn einer fehlschlägt, siehe [Helper-Fehler](/docs/de/settings-reference#helper-failures).

<span id="parent-settings-from-embedding-hosts" />

<span id="control-policy-from-an-embedding-host" />

<span id="merge-policy-from-an-embedding-host" />

<h3 id="let-an-embedding-host-add-policy">
  Lassen Sie einen Embedding-Host eine Richtlinie hinzufügen
</h3>

Wenn eine andere Anwendung Claude Code startet, wie Claude Desktop, eine IDE-Erweiterung oder eine Agent SDK-App, kann dieser Host seine eigenen verwalteten Einstellungen durch die SDK `managedSettings` Option übergeben. Claude Code nennt diese übergeordneten Einstellungen.

Standardmäßig ignoriert Claude Code übergeordnete Einstellungen, wenn eine Admin-Quelle vorhanden ist: servergesteuerte Einstellungen, eine MDM- oder Betriebssystem-Richtlinie oder eine verwaltete Einstellungsdatei.

Um Claude Code übergeordnete Einstellungen neben einer Admin-Quelle zusammenzuführen, setzen Sie [`parentSettingsBehavior`](/docs/de/settings-reference#parentsettingsbehavior) auf `"merge"` in der höchstpriorisierten verwalteten Quelle; Claude Code liest den Schlüssel nur aus dieser Quelle.

Claude Code behält dann nur die Werte des Hosts, die einschränken, was Claude tun kann, mit einer Lücke, die man kennen sollte: Wenn Sie nicht auch die `allowManaged*Only` Sperren setzen, gelten die Zulassungsregeln und Sandbox-Zulassungslisten des Hosts immer noch. Siehe [Beschränken Sie übergeordnete Einstellungen](/docs/de/claude-apps-gateway#restrict-parent-settings) für die Sperren.

Ein [`policyHelper`](/docs/de/settings-reference#policyhelper) kann die übergeordnete Zusammenführung unabhängig von diesem Schlüssel ausschalten; sein Eintrag sagt, wann.

Claude Code wendet auch diese Überprüfungen auf vom Host bereitgestellte Werte auf ihre eigene an:

* Wenn eine Admin-Quelle `allowManagedPermissionRulesOnly` setzt, lässt Claude Code [vom Host bereitgestellte](/docs/de/claude-apps-gateway#restrict-parent-settings) Zulassungsregeln und `additionalDirectories` fallen, während es sie liest, selbst wenn eine höherpriorisierte Quelle den Schlüssel nicht setzt. Die Auswirkung des Schlüssels auf Ihre eigenen Zulassungsregeln kommt aus den verwalteten Einstellungen, die Claude Code anwendet, oder aus übergeordneten Einstellungen, die Sie zusammenführen möchten
* Claude Code erzwingt den `forceLoginOrgUUID` oder `allowedMcpServers` Wert in den verwalteten Einstellungen, die es anwendet, und blockiert einen vom Host bereitgestellten. Ein Wert in einer niedrigeren Admin-Quelle, die Claude Code nicht anwendet, wendet weder an noch blockiert den Wert des Hosts.

  Auf Claude Code v2.1.273 oder später, während `allowManagedMcpServersOnly` aktiv ist, gilt die `allowedMcpServers` Liste aus der höchstrangigen Admin-Quelle, die eine setzt, und blockiert die des Hosts, als ein [Schlüssel zwischen Quellen](#keys-read-from-every-admin-source). Die Liste des Hosts gilt nur, wenn keine Admin-Quelle eine setzt. Der [`managedSourcesBehavior`](/docs/de/settings-reference#managedsourcesbehavior) Eintrag sagt, welche Quelle jeden Schlüssel unter `"merge"` liefert. Vor v2.1.223 blockierte ein Wert in einer beliebigen Admin-Quelle den Wert des Hosts
* Für `availableModels` erzwingt Claude Code den Wert in den verwalteten Einstellungen, die es anwendet, und blockiert eine vom Host bereitgestellte Liste
* Für `strictKnownMarketplaces` erzwingt Claude Code ebenso die Liste in den verwalteten Einstellungen, die es anwendet, und blockiert eine vom Host bereitgestellte. Die Liste des Hosts gilt nur, wenn keine angewendete verwaltete Quelle eine setzt. Erfordert Claude Code v2.1.282 oder später
* Eine vom Host bereitgestellte `blockedMarketplaces` gilt zusätzlich zu jeder Blockliste, die eine verwaltete Quelle setzt. Erfordert Claude Code v2.1.282 oder später

<h4 id="keep-cowork-folder-access-when-only-managed-rules-apply">
  Behalten Sie den Zugriff auf den Cowork-Ordner, wenn nur verwaltete Regeln gelten
</h4>

[Cowork](https://claude.com/docs/cowork/overview) in der Claude Desktop-App führt seine Sitzungen auf Claude Code aus und gewährt jeder Sitzung Zugriff auf ihre Arbeitsordner, wie den Ordner, den der Benutzer verbindet, durch Zulassungsregeln, die es liefert, wenn es die Sitzung startet. Wenn Ihre verwaltete Richtlinie [`allowManagedPermissionRulesOnly`](/docs/de/settings-reference#allowmanagedpermissionrulesonly) setzt, behält Claude Code nur die Zulassungsregeln in der verwalteten Richtlinie: es lässt Zulassungsregeln fallen, die ein Host als übergeordnete Einstellungen, als `--allowedTools` oder in einer Einstellungsdatei liefert, sodass Schreibvorgänge in diese Ordner ihre Vorabgenehmigung verlieren. In einer Cowork-Sitzung, die vor Bearbeitungen fragt, kann Cowork die Eingabeaufforderung nicht anzeigen, und Claude meldet jeden Schreibvorgang als blockiert, weil der Pfad zu einem geschützten Ort oder einem Pfad außerhalb des verbundenen Ordners aufgelöst wird.

Um die Schreibvorgänge wiederherzustellen, fügen Sie Zulassungsregeln für diese Ordner zur verwalteten Quelle hinzu, die Claude Code [auswählt](#precedence-within-the-managed-tier) auf diesen Computern: auf einer MDM-verwalteten Flotte ist das die MDM-Richtlinie anstelle einer separaten verwalteten Einstellungsdatei. Dieses Beispiel verwendet die Dateiform, und eine MDM-Richtlinie nimmt die gleichen Schlüssel. Es behält `allowManagedPermissionRulesOnly` gesetzt und erlaubt Bearbeitungen unter einem `CoworkProjects` Ordner im Basisverzeichnis jedes Benutzers; ersetzen Sie den Pfad durch die Ordner, die Ihre Benutzer verbinden:

```json managed-settings.json theme={null}
{
  "allowManagedPermissionRulesOnly": true,
  "permissions": {
    "allow": [
      "Edit(~/CoworkProjects/**)"
    ]
  }
}
```

Nachdem Sie die Richtlinie bereitgestellt haben, kann Claude Dateien unter diesem Ordner in einer neuen Cowork-Sitzung speichern. [Lese- und Bearbeitungsregeln](/docs/de/permissions#read-and-edit) behandeln die Pfadsyntax, einschließlich der `//` Form für absolute Pfade.

<h3 id="what-a-developer-can-change">
  Was ein Entwickler ändern kann
</h3>

Die eigenen Einstellungsdateien eines Entwicklers, `--settings` Werte und Projektdateien überschreiben niemals einen verwalteten Wert; die [Ausnahmen](/docs/de/settings#exceptions-to-managed-settings-precedence) lassen nur einen strengeren niedrigeren Wert zählen. Diese Fälle liegen außerhalb dieser Regel:

* **Das Modell für eine Sitzung**: ein verwaltetes `model` ist ein Standard, keine Sperre. `--model` und `ANTHROPIC_MODEL` wählen immer noch das Modell für diese Sitzung, also stellen Sie [`availableModels`](/docs/de/settings-reference#availablemodels) bereit, um die Auswahl einzuschränken.
* **Lokale Admin-Rechte**: ein Entwickler, der ein Administrator auf dem Computer ist, kann die verwaltete Quelle selbst bearbeiten, weshalb MDM-Tools das Profil oder die Datei nach einem Zeitplan erneut bereitstellen können und weshalb die HKLM-Registrierung und die macOS-Verwaltungseinstellungsdomäne existieren.
* **Der servergesteuerte Cache**: servergesteuerte Einstellungen kommen von Anthropic-Servern, und eine Bearbeitung des lokalen Cache [dauert nur bis zum nächsten erfolgreichen Abruf](/docs/de/server-managed-settings#security-considerations).
* **Andere Tools**: verwaltete Einstellungen binden nur Claude Code. Ein Entwickler, der die API von einem anderen Tool aufruft, unterliegt ihnen nicht.

<span id="verify-enforcement" />

<span id="verify-that-a-policy-is-in-force" />

<h2 id="check-that-a-policy-is-in-force">
  Überprüfen Sie, ob eine Richtlinie in Kraft ist
</h2>

Ein Entwickler meldet, dass eine Richtlinie nicht angewendet wird, oder Sie möchten bestätigen, dass ein Rollout abgeschlossen wurde, bevor Sie es in der gesamten Flotte bereitstellen. Zwei Befehle auf diesem Computer beantworten dies: `/status` zeigt, welche verwaltete Quelle Claude Code ausgewählt hat, und `claude doctor` listet auf, was es verworfen hat.

<h3 id="read-the-source-in-/status">
  Lesen Sie die Quelle in /status
</h3>

Führen Sie auf dem Computer des Entwicklers `/status` in Claude Code aus und lesen Sie die Zeile `Setting sources`. Wenn eine verwaltete Quelle in Kraft ist, listet die Zeile `Enterprise managed settings` mit der von Claude Code ausgewählten Quelle in Klammern auf:

* `(remote)`: servergesteuerte Einstellungen von claude.ai oder einem Gateway
* `(plist)` oder `(HKLM)`: eine MDM- oder Betriebssystemrichtlinie
* `(file)`, `(drop-ins)` oder `(file + drop-ins)`: `managed-settings.json`, das Drop-in-Verzeichnis oder beides
* `(remote + file, merged)` oder eine andere Liste, die mit `, merged` endet: Ihre Organisation [setzt jede verwaltete Quelle zusammen](#compose-every-managed-source), und Claude Code hat die aufgelisteten Quellen in die Richtlinie zusammengeführt. Eine niedrigere Quelle kann weiterhin `env`-Variablen bereitstellen, ohne in der Liste zu erscheinen. Erfordert Claude Code v2.1.242 oder später
* `(HKCU)`: das benutzergeschriebene Registrierungs-Fallback
* `(parent process)`: ein [Embedding-Host](#let-an-embedding-host-add-policy) hat restriktive Einstellungen bereitgestellt
* `(helper)`: ein [`policyHelper`](/docs/de/settings-reference#policyhelper), der von der ausgewählten MDM- oder Dateiquelle konfiguriert wird

Wenn Claude Code eine verwaltete Quelle auf dem Computer gefunden hat und diese nicht ausgewählt hat, benennt eine zweite Zeile, `Skipped sources`, jede solche Quelle. Lesen Sie sie, um eine Richtlinie, die den Computer nie erreicht hat, von einer zu unterscheiden, die ihn erreicht hat und die eine höher priorisierte Quelle überschrieben hat. Erfordert Claude Code v2.1.242 oder später.

Wenn die Richtlinie nicht angewendet wird, teilt Ihnen die Zeile `Setting sources` mit, welches von zwei Problemen Sie haben:

* **Die Zeile fehlt**: Claude Code hat keine verwaltete Quelle gefunden, die einen Richtlinienschlüssel bereitstellt.

  Wenn Sie eine verwaltete Einstellungsdatei bereitgestellt haben, überprüfen Sie, dass sie sich im Pfad für das Betriebssystem befindet und dass sie einen [Richtlinienschlüssel](#how-claude-code-combines-managed-sources) enthält, nicht nur die Steuerschlüssel. Eine Datei, die kein gültiges JSON ist, führt nicht zu diesem Zustand; Claude Code [weigert sich zu starten](#find-entries-claude-code-dropped) stattdessen.

  Wenn Sie stattdessen über servergesteuerte Einstellungen bereitgestellt haben, führen Sie `claude doctor` aus, das das [Abrufergebnis](/docs/de/server-managed-settings#verify-settings-delivery) meldet.
* **Die Zeile benennt eine Quelle, die nicht die ist, die Sie bereitgestellt haben**: eine höher priorisierte Quelle ist vorhanden und Claude Code hat die Ihre ignoriert, und `Skipped sources` listet sie auf. [Wie Claude Code verwaltete Quellen kombiniert](#how-claude-code-combines-managed-sources) gibt die Reihenfolge an.

<span id="invalid-entries-in-managed-settings" />

<h3 id="find-entries-claude-code-dropped">
  Finden Sie Einträge, die Claude Code verworfen hat
</h3>

Wenn eine verwaltete Einstellungsdatei, ein MDM-Profil, ein Registrierungswert oder eine servergesteuerte Nutzlast die Schemavalidierung nicht besteht, überspringt Claude Code zunächst die einzelnen Einträge, die es reparieren kann, wie z. B. eine ungültige Berechtigungsregel, mit einer Warnung für jeden, und verwirft dann jeden Top-Level-Schlüssel, dessen Wert immer noch fehlschlägt, und setzt die Durchsetzung aller verbleibenden gültigen Schlüssel fort.

Claude Code ist strenger mit den `managedSettings`, die ein [`policyHelper`](/docs/de/settings-reference#policyhelper) ausgibt: Es führt die gleichen Eintragsreparaturen durch, aber jede Schemaverletzung, die überlebt, schlägt den gesamten Helper-Lauf fehl, und beim Start weigert sich Claude Code zu starten, das gleiche wie für einen Helper, der mit einem Nicht-Null-Wert beendet wird.

Wenn eine verwaltete Einstellungsdatei, eine Drop-in-Datei, ein MDM-plist oder ein HKLM-Registrierungswert vorhanden ist, aber nicht als JSON-Objekt analysiert werden kann, weigert sich Claude Code zu starten und gibt [einen Fehler aus, der die Quelle benennt](/docs/de/errors#managed-settings-document-could-not-be-parsed), auch wenn eine andere Admin-Quelle eine gültige Richtlinie bereitstellt. Jede Quelle schlägt auf diese Weise fehl, wenn:

* **Verwaltete Einstellungsdatei oder Drop-in-Datei**: Die Datei ist kein gültiges JSON, oder ihre oberste Ebene ist kein Objekt
* **MDM plist**: macOS's `plutil` meldet die plist als fehlerhaft, oder ihr konvertierter Inhalt ist kein JSON-Objekt
* **HKLM-Registrierungswert**: Der `Settings`-Wert ist keine Zeichenkette, ist leer oder enthält kein JSON-Objekt

Drei Quellzustände verursachen diese Weigerung nicht:

* Eine fehlende Datei, ein Profil oder ein Registrierungswert ist kein Fehler; Claude Code läuft ohne diese Quelle.
* Eine leere verwaltete Einstellungsdatei zählt als `{}`.
* Ein fehlerhafter Wert im benutzergeschriebenen HKCU-Registrierungsschlüssel blockiert niemals den Start. Claude Code meldet ihn stattdessen als Hinweis in `/status` und `claude doctor`.

Wenn eine verwaltete Einstellungsdatei, eine Drop-in-Datei oder ein `managed-settings.d/`-Verzeichnis nicht gelesen werden kann und keine Admin-Quelle eine Richtlinie bereitstellt, beenden Sitzungen, die mit claude.ai- oder Claude Console-Anmeldedaten angemeldet sind, beim Start mit einer Nachricht, um einen Administrator zu kontaktieren.

Um einen verworfenen Eintrag zu finden, schauen Sie an einer von drei Stellen nach:

* Interaktive Sitzungen zeigen beim Start ein Dialogfeld an, das die ungültigen Einträge auflistet.
* Nicht-interaktive Läufe mit `-p` geben eine Zusammenfassung auf stderr aus.
* [`claude doctor`](/docs/de/debug-your-config) listet jeden ungültigen Eintrag mit seiner Quelle und seinem Feld auf.

<h4 id="keys-that-fail-closed">
  Schlüssel, die geschlossen fehlschlagen
</h4>

Einige Durchsetzungsschlüssel werden nicht verworfen, wenn sie ungültig sind. Claude Code setzt einen strengeren Fallback durch, bis der Wert behoben ist; die Tabelle zeigt, was es für jeden Schlüssel durchsetzt:

| Feld                          | Verhalten bei Vorhandensein, aber ungültig                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :---------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedMcpServers`           | Wird als leere Zulassungsliste durchgesetzt, bis der Wert behoben ist, sodass keine MCP-Server, die Benutzer hinzufügen, zugelassen werden. Server, die Ihre Organisation über [`managedMcpServers`](/docs/de/settings-reference#managedmcpservers) bereitstellt, werden weiterhin geladen, und `managed-mcp.json`-Server werden gemäß [Wie ein Server bewertet wird](/docs/de/managed-mcp#how-a-server-is-evaluated) geladen. Ein einzelner ungültiger Eintrag wird entfernt und die gültige Teilmenge wird durchgesetzt.                                                        |
| `allowedHttpHookUrls`         | Claude Code setzt eine leere verwaltete [Zulassungsliste](/docs/de/settings-reference#allowedhttphookurls) durch, bis Sie den Wert beheben, sodass ein HTTP-Hook nur ausgeführt wird, wenn eine andere Einstellungsdatei seine URL auflistet. Wenn nur ein einzelner Eintrag ungültig ist, entfernt Claude Code diesen Eintrag und setzt den Rest durch.                                                                                                                                                                                                                     |
| `httpHookAllowedEnvVars`      | Claude Code setzt eine leere verwaltete [Zulassungsliste](/docs/de/settings-reference#httphookallowedenvvars) durch, bis Sie den Wert beheben, sodass eine Header-Variable nur interpoliert wird, wenn eine andere Einstellungsdatei sie benennt. Wenn nur ein einzelner Eintrag ungültig ist, entfernt Claude Code diesen Eintrag und setzt den Rest durch.                                                                                                                                                                                                                 |
| `allowedChannelPlugins`       | Claude Code setzt eine leere Zulassungsliste durch, bis Sie den Wert beheben, sodass kein an `--channels` übergebenes Channel-Plugin zugelassen wird. Wenn nur ein einzelner Eintrag ungültig ist, entfernt es diesen Eintrag und setzt den Rest durch.                                                                                                                                                                                                                                                                                                                 |
| `strictKnownMarketplaces`     | Wird als leere Zulassungsliste durchgesetzt, bis der Wert behoben ist, sodass keine [Marketplace-Quelle](/docs/de/plugins/org#restrict-what-users-can-install) zugelassen wird. Ein einzelner Eintrag, der ungültig ist oder nicht durchgesetzt werden kann, wie z. B. ein `hostPattern`-Regex, der nicht kompiliert wird, wird entfernt und die gültige Teilmenge wird durchgesetzt.                                                                                                                                                                                        |
| `allowManagedHooksOnly`       | Wird als `true` behandelt, bis behoben: Die [Hook-Einschränkungen](/docs/de/settings-reference#allowmanagedhooksonly) gelten und, sofern `disableCommandPluginSources` nicht explizit `false` ist, sind befehlsgesteuerte Plugins deaktiviert.                                                                                                                                                                                                                                                                                                                               |
| `allowManagedMcpServersOnly`  | Wird als `true` behandelt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `disableCommandPluginSources` | Wird als `true` behandelt, sodass befehlsgesteuerte Plugins deaktiviert bleiben, bis der Wert behoben ist.                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `disableSideloadFlags`        | Wird als `true` behandelt, bis der Wert behoben ist, mit den für [`disableSideloadFlags`](/docs/de/settings-reference#disablesideloadflags) aufgelisteten Auswirkungen.                                                                                                                                                                                                                                                                                                                                                                                                      |
| `availableModels`             | Wird als leere Zulassungsliste durchgesetzt, bis behoben, sodass nur das Standardmodell verfügbar ist; ein Nicht-String-Eintrag wird entfernt und die gültige Teilmenge wird durchgesetzt.                                                                                                                                                                                                                                                                                                                                                                              |
| `enforceAvailableModels`      | Wird als `true` behandelt.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `syncClaudeAiPlugins`         | Wird als `false` behandelt, sodass die Synchronisierung von [claude.ai-Plugins](/docs/de/settings-reference#syncclaudeaiplugins) deaktiviert ist, bis der Wert behoben ist.                                                                                                                                                                                                                                                                                                                                                                                                  |
| `forceLoginOrgUUID`           | Keine Organisation darf sich anmelden, bis der Wert behoben ist.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `gatewayInternalNetworks`     | Wenn der ungültige Wert von der höchsten verwalteten Quelle auf dem Computer stammt, weigert sich `/login`, jede neue [Cloud-Gateway](/docs/de/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own)-Anmeldung auf diesem Computer, bis der Wert behoben ist.                                                                                                                                                                                                                                                                                                 |
| `crossSessionInbound`         | Wird als `refuse`, der restriktivste Wert, behandelt, sodass eingehende [Cross-Session-Nachrichten](/docs/de/cross-session-messaging#control-inbound-messages) abgelehnt werden, bis der Wert behoben ist. Der Entwickler sieht [eine Warnung](/docs/de/errors#crosssessioninbound-must-be-one-of-accept-hold-refuse).                                                                                                                                                                                                                                                            |
| `deniedMcpServers`            | Ein einzelner ungültiger Eintrag wird entfernt und die gültige Teilmenge wird durchgesetzt. Ein vollständig ungültiger Wert wird mit einer Warnung verworfen, da das Ablehnen aller Server Server blockieren würde, die die Richtlinie nie benannt hat.                                                                                                                                                                                                                                                                                                                 |
| `blockedMarketplaces`         | Ein einzelner ungültiger Eintrag wird entfernt und die gültige Teilmenge wird durchgesetzt. Ein Eintrag, der analysiert wird, aber nie übereinstimmen kann, wie z. B. ein `hostPattern`-Regex, der nicht kompiliert wird, wird mit einer Warnung beibehalten. Es blockiert nichts, bis behoben, aber [Marketplace-Einschränkungen](/docs/de/plugins/org#restrict-what-users-can-install) bleiben aktiv. Ein vollständig ungültiger Wert wird mit einer Warnung verworfen, da das Blockieren aller Marketplaces Quellen blockieren würde, die die Richtlinie nie benannt hat. |
| `sandbox.credentials`         | Ein behebbarer ungültiger Eintrag wird zu `mode: "deny"` mit einer Warnung herabgestuft; ein nicht behebbarer wird entfernt; gültige Einträge bleiben durchgesetzt. Siehe [ungültige Credential-Einträge](/docs/de/settings-reference#invalid-credential-entries-in-managed-settings)                                                                                                                                                                                                                                                                                        |

`allowedHttpHookUrls` und `httpHookAllowedEnvVars` werden über Einstellungsdateien zusammengeführt, sodass Einträge in Ihren Benutzer-, Projekt- oder lokalen Einstellungen weiterhin gelten, während die verwaltete Liste leer ist.

Die Fallbacks für diese beiden Schlüssel und für `allowedChannelPlugins` erfordern Claude Code v2.1.267 oder später; frühere Versionen verwerfen den gesamten Schlüssel, wenn sein Wert oder ein Eintrag ungültig ist. Die Fallbacks für `strictKnownMarketplaces`, `blockedMarketplaces` und `disableSideloadFlags` erfordern Claude Code v2.1.277 oder später; frühere Versionen verwerfen den gesamten Schlüssel, wenn sein Wert oder ein Eintrag ungültig ist.

`requiredMinimumVersion` und `requiredMaximumVersion` schlagen absichtlich offen fehl: Ein ungültiger Wert wird verworfen, anstatt durchgesetzt zu werden.

Diese Toleranz gilt nur für verwaltete Einstellungen. Benutzer-, Projekt- und lokale Einstellungsdateien bleiben streng: Eine Datei, deren JSON oder Top-Level-Form die Validierung nicht besteht, wird als Ganzes abgelehnt und gemeldet, und ein einzelner Eintrag, der fehlschlägt, wie z. B. eine fehlerhafte Berechtigungsregel, wird mit einer Warnung übersprungen, während der Rest der Datei angewendet wird.

<span id="managed-only-settings" />

<h2 id="keys-only-a-managed-source-can-set">
  Schlüssel, die nur eine verwaltete Quelle setzen kann
</h2>

Claude Code liest die folgenden Schlüssel nur aus einer verwalteten Quelle; das Platzieren in Benutzer- oder Projekteinstellungsdateien hat keine Auswirkung.

Die meisten sind Sperren: Der Wert, den eine Sperre regelt, wie Berechtigungsregeln oder `sandbox.network.allowedDomains`, ist ein gewöhnlicher Schlüssel, den jede Ebene setzen kann, und die Sperre sagt Claude Code, nur den verwalteten Wert zu beachten.

Die Tabelle behandelt die Berechtigungs-, Plugin- und Liefersteuerelemente. Für jeden Schlüssel, der hier nicht aufgelistet ist, sagt die Spalte „Scope" der [Einstellungsreferenz](/docs/de/settings-reference#all-settings) Index, ob er nur verwaltet ist; die verbleibenden nur verwalteten Schlüssel dort umfassen die Gateway-Anmelde-URL, Version, Browser, Mobile-Simulator, SSH-Host, Desktop-Lokalsitzung, Sandbox-Binärpfad, Modellpreisgestaltung und CLAUDE.md-Steuerelemente.

| Einstellung                                                                                                           | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :-------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`allowAllClaudeAiMcps`](/docs/de/settings-reference#allowallclaudeaimcps)                                                 | Laden Sie die claude.ai-Konnektoren, die Claude Code selbst abruft, neben einer bereitgestellten `managed-mcp.json`, anstatt sie zu unterdrücken                                                                                                                                                                                                                                                                                                                                                                                                    |
| [`allowedChannelPlugins`](/docs/de/settings-reference#allowedchannelplugins)                                               | Zulassungsliste von Channel-Plugins, die Nachrichten schieben dürfen. Ersetzt die Standard-Anthropic-Zulassungsliste, wenn gesetzt. Erfordert `channelsEnabled: true`. Siehe [Beschränken Sie, welche Channel-Plugins ausgeführt werden können](/docs/de/channels#restrict-which-channel-plugins-can-run)                                                                                                                                                                                                                                                |
| [`allowManagedHooksOnly`](/docs/de/settings-reference#allowmanagedhooksonly)                                               | Wenn `true`, beschränkt, welche Hooks ausgeführt werden; siehe [was unter `allowManagedHooksOnly` ausgeführt wird](/docs/de/settings-reference#what-runs-under-allowmanagedhooksonly) für die vollständige Effektliste                                                                                                                                                                                                                                                                                                                                   |
| [`allowManagedMcpServersOnly`](/docs/de/settings-reference#allowmanagedmcpserversonly)                                     | Wenn `true`, werden nur `allowedMcpServers` aus verwalteten Einstellungen beachtet. `deniedMcpServers` führt immer noch aus allen Quellen zusammen. Siehe [Schlüssel, die von jeder Admin-Quelle gelesen werden](#keys-read-from-every-admin-source) für welche verwalteten Quellen es setzen können, und [Verwaltete MCP-Konfiguration](/docs/de/managed-mcp)                                                                                                                                                                                           |
| [`allowManagedPermissionRulesOnly`](/docs/de/settings-reference#allowmanagedpermissionrulesonly)                           | Macht verwaltete Einstellungen zur einzigen Einstellungsquelle von Berechtigungsregeln. Der Eintrag listet jede Quelle auf, die er ignoriert                                                                                                                                                                                                                                                                                                                                                                                                        |
| [`blockedMarketplaces`](/docs/de/settings-reference#blockedmarketplaces)                                                   | Blockliste von Marktplatzquellen. Blockierte Quellen werden vor dem Download überprüft, daher berühren sie niemals das Dateisystem. Siehe [verwaltete Marktplatz-Einschränkungen](/docs/de/plugins/org#restrict-what-users-can-install)                                                                                                                                                                                                                                                                                                                  |
| [`channelsEnabled`](/docs/de/settings-reference#channelsenabled)                                                           | Erlauben Sie [Kanäle](/docs/de/channels) für die Organisation. Siehe [Enterprise-Steuerelemente](/docs/de/channels#enterprise-controls) für die Standardeinstellung auf jedem Plan                                                                                                                                                                                                                                                                                                                                                                            |
| [`disableCommandPluginSources`](/docs/de/settings-reference#disablecommandpluginsources)                                   | Wenn `true`, blockiert [`command`-Plugin-Quellen](/docs/de/plugins/marketplace-reference#command-plugin-source) vollständig, daher wird der vom Marktplatz deklarierte Befehl nie ausgeführt. Blockiert auch Marktplatz-[`headersHelper`-Befehle](/docs/de/plugins/host-marketplace#authenticate-archive-downloads), außer für einen Marktplatz, den verwaltete Einstellungen selbst deklarieren. Wenn nicht gesetzt, folgt `allowManagedHooksOnly`. Erfordert Claude Code v2.1.229 oder später, und der `headersHelper`-Block erfordert v2.1.238 oder später |
| [`disableSideloadFlags`](/docs/de/settings-reference#disablesideloadflags)                                                 | Lehnen Sie die Flags `--plugin-dir`, `--plugin-url`, `--agents` und `--mcp-config` beim Start ab. In Cloud-Sitzungen löscht Claude Code die MCP-Server, die der Server durch `--mcp-config` bereitgestellt hat, mit Ausnahme von In-Process-`type: "sdk"`-Einträgen, und startet die Sitzung. Erfordert Claude Code v2.1.193 oder später                                                                                                                                                                                                            |
| [`forceRemoteSettingsRefresh`](/docs/de/settings-reference#forceremotesettingsrefresh)                                     | Wenn `true`, blockiert CLI-Start, bis remote verwaltete Einstellungen frisch abgerufen werden, und beendet, wenn der Abruf fehlschlägt. Siehe [Fail-Closed-Durchsetzung](/docs/de/server-managed-settings#enforce-fail-closed-startup)                                                                                                                                                                                                                                                                                                                   |
| [`managedMcpServers`](/docs/de/settings-reference#managedmcpservers)                                                       | Remote-MCP-Server, die jedem Benutzer neben ihren eigenen bereitgestellt werden. Es stellt Server bereit, anstatt etwas zu sperren. Siehe [Stellen Sie Server durch verwaltete Einstellungen bereit](/docs/de/managed-mcp#provide-servers-through-managed-settings). Erfordert Claude Code v2.1.259 oder später                                                                                                                                                                                                                                          |
| [`managedSourcesBehavior`](/docs/de/settings-reference#managedsourcesbehavior)                                             | Ob Claude Code nur die höchstpriorität verwaltete Quelle anwendet oder [komponiert jede von ihnen](#compose-every-managed-source)                                                                                                                                                                                                                                                                                                                                                                                                                   |
| [`parentSettingsBehavior`](/docs/de/settings-reference#parentsettingsbehavior)                                             | Ob vom Host bereitgestellte übergeordnete Einstellungen unter der verwalteten Richtlinie zusammengeführt werden                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| [`pluginSuggestionMarketplaces`](/docs/de/settings-reference#pluginsuggestionmarketplaces)                                 | Marktplätze, deren Plugins Claude Code Benutzern vorschlagen darf                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| [`pluginTrustMessage`](/docs/de/settings-reference#plugintrustmessage)                                                     | Benutzerdefinierte Nachricht, die der vor der Installation angezeigten Plugin-Vertrauenswarnung angehängt wird                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [`policyHelper`](/docs/de/settings-reference#policyhelper)                                                                 | Ausführbare Datei, die verwaltete Einstellungen beim Start berechnet; siehe [Berechnen Sie verwaltete Einstellungen mit einem Policy-Helper](/docs/de/settings-reference#policyhelper)                                                                                                                                                                                                                                                                                                                                                                   |
| [`sandbox.filesystem.allowManagedReadPathsOnly`](/docs/de/settings-reference#sandbox-filesystem-allowmanagedreadpathsonly) | Wenn `true`, werden nur `filesystem.allowRead`-Pfade aus verwalteten Einstellungen beachtet. `denyRead` führt immer noch aus allen Quellen zusammen                                                                                                                                                                                                                                                                                                                                                                                                 |
| [`sandbox.network.allowManagedDomainsOnly`](/docs/de/settings-reference#sandbox-network-allowmanageddomainsonly)           | Beachten Sie nur verwaltete `allowedDomains` und `WebFetch(domain:...)`-Zulassungsregeln; blockieren Sie andere Domänen ohne Eingabeaufforderung                                                                                                                                                                                                                                                                                                                                                                                                    |
| [`strictKnownMarketplaces`](/docs/de/settings-reference#strictknownmarketplaces)                                           | Steuert, welche Plugin-Marktplatzquellen Benutzer hinzufügen und Plugins installieren können. Siehe [verwaltete Marktplatz-Einschränkungen](/docs/de/plugins/org#restrict-what-users-can-install)                                                                                                                                                                                                                                                                                                                                                        |
| [`strictPluginOnlyCustomization`](/docs/de/settings-reference#strictpluginonlycustomization)                               | Blockieren Sie Skills, Agents, Hooks und MCP-Server aus Benutzer- und Projektquellen; `true` sperrt alle vier, ein Array nennt, welche                                                                                                                                                                                                                                                                                                                                                                                                              |
| [`wslInheritsWindowsSettings`](/docs/de/settings-reference#wslinheritswindowssettings)                                     | Wenn im HKLM-Registrierungsschlüssel oder einer Datei unter `C:\Program Files\ClaudeCode` gesetzt, lassen Sie WSL die Windows-Richtlinienkette lesen und lesen Sie `/etc/claude-code` nur, wenn keine verwaltete Einstellungsdatei oder Drop-in unter diesem Verzeichnis einen [Richtlinienschlüssel](#how-claude-code-combines-managed-sources) bereitstellt; der Eintrag gibt die Reihenfolge                                                                                                                                                     |

<Note>
  Auf Team- und Enterprise-Plänen aktiviert oder deaktiviert ein Owner [Remote Control](/docs/de/remote-control) und [Web-Sitzungen](/docs/de/claude-code-on-the-web) organisationsweit in [Claude Code-Verwaltungseinstellungen](https://claude.ai/admin-settings/claude-code). Remote Control kann zusätzlich pro Gerät mit der Einstellung [`disableRemoteControl`](/docs/de/settings-reference#disableremotecontrol) deaktiviert werden. Web-Sitzungen haben keinen verwalteten Einstellungsschlüssel pro Gerät.

  Um zu überprüfen, ob diese Organisationseinstellungen einen bestimmten Rechner erreicht haben, führen Sie dort `claude doctor` aus und lesen Sie die Zeile `Organization policy`, die sagt, wo Claude Code die Richtlinie geladen hat oder warum es nicht geladen wurde. Erfordert Claude Code v2.1.261 oder später. In einer laufenden Sitzung zeigt `/status` die gleiche Zeile, wenn die Richtlinie nicht geladen wurde.
</Note>

<h2 id="turn-telemetry-off-for-your-organization">
  Schalten Sie Telemetrie für Ihre Organisation aus
</h2>

Claude Code sendet Anthropic-Betriebstelemetrie]\(/de/data-usage#telemetry-services) standardmäßig auf Sitzungen, die die Anthropic API verwenden, ob direkt, über ein LLM-Gateway oder über ein benutzerdefiniertes `ANTHROPIC_BASE_URL`; [Standardverhalten nach API-Anbieter](/docs/de/data-usage#default-behaviors-by-api-provider) sagt, welche Anbieter es senden. Um es für jeden Entwickler auszuschalten, ohne sich auf die Shell jeder Person zu verlassen, liefern Sie `DISABLE_TELEMETRY` über den `env`-Block Ihrer verwalteten Einstellungen. Dieses Beispiel setzt `DISABLE_TELEMETRY` für jeden, den die Richtlinie erreicht:

```json theme={null}
{
  "env": {
    "DISABLE_TELEMETRY": "1"
  }
}
```

Claude Code wendet einen Wert von `1` an, ohne dem Benutzer den [Genehmigungsdialog](/docs/de/server-managed-settings#environment-variables-and-the-approval-dialog) zu zeigen.

Wenn Sie Telemetrie ausschalten, stoppt Claude Code das Senden der Nutzungsdaten, die Ihr Organisations-[Analyse-Dashboard](/docs/de/analytics) für die Entwickler speist, die die Richtlinie erreicht. Die Variable schaltet auch das Abrufen von Feature-Flags aus, was [Features, die Feature-Flag-Abrufen benötigen](/docs/de/env-vars#features-that-need-feature-flag-fetching), für diese Entwickler nicht verfügbar macht.

[Wo und wann eine Richtlinie angewendet wird](#where-and-when-a-policy-applies) sagt, welcher Bereitstellungsmechanismus jede Oberfläche erreicht, und [Plattformverfügbarkeit](/docs/de/server-managed-settings#platform-availability) sagt, welche Sitzungen den Abruf von serververwalteten Einstellungen überspringen.

Wenn Ihre Organisation kundenverwaltete Verschlüsselungsschlüssel verwendet und Claude Code über ein Gateway leitet, sagt [Konfigurieren Sie Proxys und Gateways](/docs/de/third-party-integrations#configure-proxies-and-gateways), warum diese Sitzungen diese Variable benötigen.

<h2 id="see-also">
  Siehe auch
</h2>

* [Richten Sie Claude Code für Ihre Organisation ein](/docs/de/admin-setup): Entscheiden Sie, was durchgesetzt werden soll und wie
* [Serververwaltete Einstellungen](/docs/de/server-managed-settings): Liefern Sie Richtlinie aus der claude.ai-Konsole oder einem Gateway
* [Verwaltete MCP-Konfiguration](/docs/de/managed-mcp): Steuern Sie, welche MCP-Server Entwickler verwenden können
* [Alle Einstellungen](/docs/de/settings-reference): Jeder Schlüssel, mit ob eine verwaltete Quelle ihn setzen kann
* [Beispiel-Einstellungsdateien](/docs/de/settings-example#an-organizations-managed-settings): Eine vollständige `managed-settings.json`, die die Form der verwalteten Schlüssel zeigt
