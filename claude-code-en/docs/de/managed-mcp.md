> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kontrollieren Sie den MCP-Serverzugriff für Ihre Organisation

> Beschränken Sie, welche MCP-Server Benutzer hinzufügen oder mit verwalteten Konfigurationsdateien, Zulassungslisten und Sperrlisten verbinden können.

Standardmäßig kann jeder, der Claude Code ausführt, jeden beliebigen [MCP-Server](/docs/de/mcp) verbinden. Anthropic überprüft Konnektoren anhand seiner [Auflistungskriterien](https://claude.com/docs/connectors/building/review-criteria), bevor sie zum [Anthropic-Verzeichnis](https://claude.ai/directory) hinzugefügt werden, führt aber keine Sicherheitsprüfung durch und verwaltet keinen MCP-Server. Als Administrator können Sie einschränken, welche Server in Ihrer Organisation ausgeführt werden – von der Bereitstellung eines festen genehmigten Satzes bis zur vollständigen Deaktivierung von MCP.

Diese Seite behandelt folgende Themen:

* [Wählen Sie ein Muster](#choose-a-pattern), das dem erforderlichen Kontrollumfang entspricht
* [Stellen Sie einen festen Serversatz mit `managed-mcp.json` bereit](#exclusive-control-with-managed-mcp-json), einschließlich [Deaktivierung von MCP vollständig](#disable-mcp-entirely)
* [Kontrollieren Sie Server mit Zulassungslisten und Sperrlisten](#policy-based-control-with-allowlists-and-denylists)
* [Teilen Sie Benutzern mit, was sie erwarten können](#how-restrictions-appear-to-users), wenn eine Einschränkung einen Server blockiert
* [Überwachen Sie, welche Server Ihre Organisation tatsächlich nutzt](#monitor-mcp-usage)

<Note>
  Die Seite [Sicherheit](/docs/de/security) behandelt das MCP-Bedrohungsmodell und wie Sie einen Server vor der Genehmigung bewerten. [Entscheiden Sie, was Sie durchsetzen möchten](/docs/de/admin-setup#decide-what-to-enforce) behandelt MCP-Einschränkungen zusammen mit den anderen administrativen Kontrollen.
</Note>

<h2 id="choose-a-pattern">
  Wählen Sie ein Muster
</h2>

Claude Code unterstützt eine Reihe von Einschränkungsstufen. Jedes Muster verwendet einen oder beide der unten behandelten Mechanismen: `managed-mcp.json` zur Bereitstellung eines festen Satzes und `allowedMcpServers`/`deniedMcpServers` zum Filtern der Benutzerkonfiguration.

| Muster                    | Was es tut                                                                                                           | Konfigurieren                                                                                        |
| :------------------------ | :------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------- |
| **MCP deaktivieren**      | Keine Server werden irgendwo geladen                                                                                 | `managed-mcp.json` mit einer leeren Serverzuordnung                                                  |
| **Feste Bereitstellung**  | Jeder Benutzer erhält die gleichen Server und kann keine anderen hinzufügen                                          | `managed-mcp.json` mit den gewünschten Servern                                                       |
| **Genehmigter Katalog**   | Veröffentlichen Sie eine Liste genehmigter Server; Benutzer fügen die gewünschten hinzu, alles andere wird blockiert | `allowedMcpServers` + `allowManagedMcpServersOnly: true`                                             |
| **Nur Plugin-Server**     | Server können nur aus Plugins stammen; Benutzer können keine eigenen hinzufügen                                      | [`strictPluginOnlyCustomization`](/docs/de/settings#strictpluginonlycustomization) mit `mcp` in der Liste |
| **Soft-Zulassungsliste**  | Erzwingen Sie eine Zulassungsliste, die Benutzer in ihren eigenen Einstellungen erweitern können                     | `allowedMcpServers` ohne `allowManagedMcpServersOnly`                                                |
| **Nur Sperrliste**        | Blockieren Sie bekannt schlechte Server, erlauben Sie alles andere                                                   | `deniedMcpServers`                                                                                   |
| **Keine Einschränkungen** | Benutzer fügen alles hinzu                                                                                           | Stellen Sie keine verwaltete MCP-Konfiguration bereit                                                |

<Note>
  Claude Code hat keine integrierte MCP-Serverregistrierung, die Benutzer durchsuchen und installieren können. Für das Muster des genehmigten Katalogs teilen Sie die genehmigte Liste und ihre `claude mcp add`-Befehle an einem Ort, den Ihre Benutzer finden, z. B. einem internen Wiki, oder verteilen Sie die Server als Plugins über einen [verwalteten Plugin-Marketplace](/docs/de/plugin-marketplaces#managed-marketplace-restrictions), damit Benutzer sie von `/plugin` durchsuchen und installieren können.
</Note>

<h2 id="exclusive-control-with-managed-mcp-json">
  Exklusive Kontrolle mit managed-mcp.json
</h2>

Wenn Sie eine `managed-mcp.json`-Datei bereitstellen, lädt Claude Code nur die Server, die diese Datei definiert. Benutzer können keine anderen MCP-Server hinzufügen, ändern oder verwenden, einschließlich Plugin-bereitgestellter Server. Die Datei unterdrückt auch claude.ai-Konnektoren, es sei denn, Sie [erlauben sie neben dem verwalteten Satz](#allow-claude-ai-connectors-alongside-the-managed-set).

Zwei weitere Einstellungen können den verwalteten Satz weiter filtern:

* `allowedMcpServers` und `deniedMcpServers` gelten auch für verwaltete Server, daher wird ein verwalteter Server, der diese nicht erfüllt, nicht geladen.
* Die eigenen `deniedMcpServers` eines Benutzers werden aus seinen Einstellungen zusammengeführt, daher können Benutzer einen verwalteten Server für sich selbst blockieren.

Siehe [Wie ein Server bewertet wird](#how-a-server-is-evaluated) für die vollständige Reihenfolge der Überprüfungen.

`managed-mcp.json` ist eine eigenständige Datei und kann nicht über [serververwaltete Einstellungen](/docs/de/server-managed-settings) bereitgestellt werden. Jeder Prozess, der in einen Systempfad mit Administratorrechten schreiben kann, kann ihn bereitstellen. In großem Maßstab geschieht dies normalerweise über Geräteverwaltungstools wie Jamf oder ein Konfigurationsprofil auf macOS, Gruppenrichtlinie oder Intune unter Windows oder Ihre Fleet-Verwaltung Ihrer Wahl unter Linux. Claude Code sucht die Datei unter einem dieser Pfade:

| Plattform     | Pfad                                                       |
| :------------ | :--------------------------------------------------------- |
| macOS         | `/Library/Application Support/ClaudeCode/managed-mcp.json` |
| Linux und WSL | `/etc/claude-code/managed-mcp.json`                        |
| Windows       | `C:\Program Files\ClaudeCode\managed-mcp.json`             |

Die Datei verwendet das gleiche Format wie eine Projekt-[`.mcp.json`](/docs/de/mcp#project-scope)-Datei:

```json theme={null}
{
  "mcpServers": {
    "github": {
      "type": "http",
      "url": "https://api.githubcopilot.com/mcp/"
    },
    "sentry": {
      "type": "http",
      "url": "https://mcp.sentry.dev/mcp"
    },
    "company-internal": {
      "type": "stdio",
      "command": "/usr/local/bin/company-mcp-server",
      "args": ["--config", "/etc/company/mcp-config.json"],
      "env": {
        "COMPANY_API_URL": "https://internal.example.com"
      }
    }
  }
}
```

<h3 id="authenticate-with-per-user-credentials">
  Authentifizieren Sie sich mit benutzerspezifischen Anmeldedaten
</h3>

Jeder Benutzer auf dem Computer kann diese Datei lesen, daher speichern Sie keine API-Schlüssel oder andere Anmeldedaten in `env`-Blöcken. Übergeben Sie benutzerspezifische Anmeldedaten stattdessen mit einem dieser:

* [`${VAR}`-Erweiterung](/docs/de/mcp#environment-variable-expansion-in-mcp-json) zum Lesen von Geheimnissen aus der Umgebung jedes Benutzers.
* [OAuth oder benutzerspezifische Header](/docs/de/mcp#authenticate-with-remote-mcp-servers), damit sich jeder Benutzer selbst authentifiziert.
* [`headersHelper`](/docs/de/mcp#use-dynamic-headers-for-custom-authentication) zum Generieren von Anmeldedaten zum Verbindungszeitpunkt.

<h3 id="validate-the-configuration">
  Validieren Sie die Konfiguration
</h3>

Um zu bestätigen, dass die Datei wirksam ist, führen Sie zwei Überprüfungen auf einem verwalteten Computer durch:

1. `claude mcp list` zeigt nur die Server in `managed-mcp.json`. Wenn die eigenen Server eines Benutzers immer noch angezeigt werden, wird die Datei nicht gelesen; überprüfen Sie den Pfad und die Berechtigungen.
2. `claude mcp add --transport http test https://example.com/mcp` schlägt mit `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers` fehl. Die URL muss kein echter Server sein, da die Richtlinienprüfung den Befehl ablehnt, bevor etwas kontaktiert wird.

<h3 id="disable-mcp-entirely">
  Deaktivieren Sie MCP vollständig
</h3>

Stellen Sie eine `managed-mcp.json` mit einer leeren Serverzuordnung bereit, um jeden MCP-Server zu blockieren:

```json theme={null}
{
  "mcpServers": {}
}
```

Benutzer sehen keine MCP-Server in `/mcp`, und `claude mcp add` schlägt mit dem oben genannten Enterprise-Richtlinienfehler fehl. Server, die Benutzer zuvor konfiguriert hatten, werden beim nächsten Starten einer Sitzung nicht mehr geladen, ohne dass eine Warnung angezeigt wird, dass die Richtlinie der Grund ist.

<h3 id="allow-claude-ai-connectors-alongside-the-managed-set">
  Erlauben Sie claude.ai-Konnektoren neben dem verwalteten Satz
</h3>

Die Bereitstellung von `managed-mcp.json` unterdrückt [claude.ai-Konnektoren](/docs/de/mcp#use-mcp-servers-from-claude-ai) standardmäßig, einschließlich Konnektoren, die ein Administrator für die Organisation in der claude.ai-Verwaltungskonsole konfiguriert hat. Um diese Konnektoren neben den Servern in `managed-mcp.json` zu laden, setzen Sie `"allowAllClaudeAiMcps": true` in einer [verwalteten Einstellungsquelle](/docs/de/admin-setup#decide-how-settings-reach-devices). Erfordert Claude Code v2.1.149 oder später.

Mit der aktivierten Einstellung lädt Claude Code die gleichen claude.ai-Konnektoren, die es laden würde, wenn `managed-mcp.json` nicht bereitgestellt würde. [Zulassungslisten und Ablehnungslisten](#policy-based-control-with-allowlists-and-denylists) gelten weiterhin für diese Konnektoren, daher können Sie bestimmte mit `deniedMcpServers` blockieren. Die Einstellung betrifft nur claude.ai-Konnektoren; Plugin-bereitgestellte Server bleiben unterdrückt.

Claude Code liest diese Einstellung nur aus von Administratoren kontrollierten Richtlinien-Ebenen: serververwaltete Einstellungen, ein von MDM bereitgestellter plist- oder HKLM-Registrierungsschlüssel oder eine System-`managed-settings.json`-Datei. Das Platzieren in Benutzer- oder Projekteinstellungen hat keine Auswirkung, daher können Benutzer Konnektoren, die exklusive Kontrolle unterdrückt hat, nicht erneut aktivieren.

<h2 id="policy-based-control-with-allowlists-and-denylists">
  Richtlinienbasierte Kontrolle mit Zulassungslisten und Sperrlisten
</h2>

Zulassungslisten und Sperrlisten filtern, welche konfigurierten Server geladen werden dürfen. Sie sind keine Registrierung: Ein Server muss immer noch von einem Benutzer, einem Plugin oder `managed-mcp.json` hinzugefügt werden, bevor die Zulassungsliste oder Sperrliste darauf angewendet wird. Um Server für Benutzer bereitzustellen, verwenden Sie [`managed-mcp.json`](#exclusive-control-with-managed-mcp-json). Beide Listen filtern auch Server, die mit dem [`--mcp-config` CLI-Flag](/docs/de/cli-reference#cli-flags) übergeben werden; `--strict-mcp-config` begrenzt, welche Konfigurationsdateien geladen werden, und umgeht keine der beiden Listen.

Um die Zulassungsliste verbindlich zu machen, setzen Sie `allowedMcpServers` und `allowManagedMcpServersOnly: true` zusammen in einer [verwalteten Einstellungsquelle](/docs/de/admin-setup#decide-how-settings-reach-devices), z. B. serververwaltete Einstellungen oder eine bereitgestellte `managed-settings.json`-Datei. [Beschränken Sie die Zulassungsliste auf verwaltete Einstellungen nur](#restrict-the-allowlist-to-managed-settings-only) zeigt die Konfiguration. Ohne `allowManagedMcpServersOnly` werden Zulassungslisten aus jeder Einstellungsquelle zusammengeführt, einschließlich der eigenen `~/.claude/settings.json` eines Benutzers, daher kann ein Benutzer erweitern, was Ihre Zulassungsliste erlaubt. Sperrlisten werden unabhängig davon aus jeder Quelle zusammengeführt.

<Note>
  `allowManagedMcpServersOnly` ist getrennt von `allowManagedPermissionRulesOnly`, das [Berechtigungsregeln](/docs/de/permissions#managed-settings) nur sperrt. Das Setzen dieses Flags erzwingt nicht die MCP-Zulassungsliste.
</Note>

<h3 id="match-servers-by-url-command-or-name">
  Passen Sie Server nach URL, Befehl oder Name an
</h3>

`allowedMcpServers` und `deniedMcpServers` sind Listen von Einträgen. Jeder Eintrag ist ein Objekt mit einem einzelnen Schlüssel, der Server nach ihrer URL, ihrem Befehl oder ihrem Namen identifiziert:

| Schlüssel       | Passt zu                                                                                           | Verwenden Sie für                         |
| :-------------- | :------------------------------------------------------------------------------------------------- | :---------------------------------------- |
| `serverUrl`     | Eine Remote-Server-URL, exakt oder mit `*`-Platzhaltern                                            | HTTP- und SSE-Server                      |
| `serverCommand` | Der genaue Befehl und die Argumente, die einen Stdio-Server starten                                | Stdio-Server                              |
| `serverName`    | Das vom Benutzer zugewiesene Label. Nur exakte Übereinstimmung; Platzhalter werden nicht erweitert | Beide Typen, aber siehe die Warnung unten |

Das Nicht-Setzen von `allowedMcpServers` unterscheidet sich vom Setzen auf ein leeres Array:

| Einstellung         | Nicht gesetzt (Standard) | Leeres Array `[]`      | Gefüllt                             |
| :------------------ | :----------------------- | :--------------------- | :---------------------------------- |
| `allowedMcpServers` | Alle Server erlaubt      | Keine Server erlaubt   | Nur übereinstimmende Server erlaubt |
| `deniedMcpServers`  | Keine Server blockiert   | Keine Server blockiert | Übereinstimmende Server blockiert   |

Siehe [Ungültige Einträge in verwalteten Einstellungen](/docs/de/settings#invalid-entries-in-managed-settings) für das, was passiert, wenn ein Eintrag die Schemavalidierung nicht besteht.

<Warning>
  Ein `serverName`-Eintrag in einer der beiden Listen ist keine Sicherheitskontrolle. Der Name ist das Label, das ein Benutzer beim Ausführen von `claude mcp add` oder beim Bearbeiten einer Konfigurationsdatei zuweist, nicht der zugrunde liegende Server, daher kann ein Benutzer jeden Server `github` nennen. Für claude.ai-Konnektoren ist der Name der von claude.ai zurückgegebene Anzeigename, der sich ändern kann. Um zu erzwingen, welche Server tatsächlich ausgeführt werden, fügen Sie `serverCommand`- oder `serverUrl`-Einträge hinzu.
</Warning>

Die `serverName`-Validierung unterscheidet sich zwischen den beiden Listen:

* In `deniedMcpServers` akzeptiert `serverName` jede nicht leere Zeichenkette, daher können Sie [claude.ai-Konnektoren](/docs/de/mcp#use-mcp-servers-from-claude-ai) nach ihrem Anzeigenamen blockieren. Beispielsweise blockiert `{ "serverName": "claude.ai Slack" }` den Slack-Konnektor. Bevorzugen Sie einen `serverUrl`-Eintrag, wenn die Sperrung robust gegen Umbenennungen sein muss, oder wenn ein Konnektor-Name kollidiert und ein ` (N)`-Suffix erhält.
* In `allowedMcpServers` ist `serverName` auf Buchstaben, Zahlen, Bindestriche und Unterstriche beschränkt. Verwenden Sie `serverUrl`, um einen claude.ai-Konnektor in die Zulassungsliste aufzunehmen.

Um alle claude.ai-Konnektoren auszuschalten, siehe [`disableClaudeAiConnectors`](/docs/de/mcp#disable-claude-ai-connectors).

<h3 id="how-a-server-is-evaluated">
  Wie ein Server bewertet wird
</h3>

Vor dem Laden eines Servers, einschließlich eines aus `managed-mcp.json`, führt Claude Code drei Überprüfungen in Reihenfolge durch:

1. **Zusammenführen der Listen.** Zulassungslisten- und Sperrlisten-Einträge aus jeder Einstellungsquelle werden in eine Zulassungsliste und eine Sperrliste kombiniert. Wenn `allowManagedMcpServersOnly` `true` ist, wird nur die verwaltete Zulassungsliste beibehalten; die Sperrliste wird immer aus jeder Quelle zusammengeführt.
2. **Überprüfen Sie die Sperrliste.** Ein Server, der einem Sperrlisten-Eintrag entspricht, nach URL, Befehl oder Name, wird blockiert. Nichts überschreibt eine Sperrlisten-Übereinstimmung.
3. **Überprüfen Sie die Zulassungsliste.** Wenn `allowedMcpServers` nirgendwo gesetzt ist, wird jeder Server, der die Sperrliste bestanden hat, geladen. Wenn es gesetzt ist, hängt das, dem der Server entsprechen muss, von seinem Typ ab, wie in der Tabelle unten gezeigt.

| Servertyp              | Erlaubt, wenn es passt zu                                                                                                                 |
| :--------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| Remote (HTTP oder SSE) | Ein `serverUrl`-Eintrag. Eine `serverName`-Übereinstimmung zählt nur, wenn die Zulassungsliste keine `serverUrl`-Einträge enthält         |
| Stdio                  | Ein `serverCommand`-Eintrag. Eine `serverName`-Übereinstimmung zählt nur, wenn die Zulassungsliste keine `serverCommand`-Einträge enthält |

Drei Matching-Regeln gelten innerhalb dieser Überprüfungen:

* **Befehle stimmen genau überein.** Jedes Argument, in Reihenfolge. `["npx", "-y", "server"]` stimmt nicht mit `["npx", "server"]` oder `["npx", "-y", "server", "--flag"]` überein.
* **`serverCommand`- und `serverUrl`-Werte werden vor dem Matching erweitert.** Sowohl der Richtlinieneintrag als auch der konfigurierte Wert des Servers durchlaufen die gleiche [`${VAR}`- und `${VAR:-default}`-Erweiterung](/docs/de/mcp#environment-variable-expansion-in-mcp-json) wie `.mcp.json`, daher passt ein Eintrag, der als `["${HOME}/bin/server"]` geschrieben ist, zu einer Serverkonfiguration, die entweder die gleiche Referenz oder den erweiterten Pfad verwendet. Unter Windows verweisen Sie auf eine Umgebungsvariable, die dort gesetzt ist, z. B. `${USERPROFILE}` statt `${HOME}`. `serverName`-Werte stimmen wörtlich überein und werden nie erweitert.
* **URLs unterstützen `*`-Platzhalter** überall im Muster, einschließlich des Schemas. Hostname-Matching ist case-insensitiv und ignoriert einen nachgestellten FQDN-Punkt, daher passt `https://Mcp.Example.com/*` zu `https://mcp.example.com/api`. Pfade bleiben case-sensitiv.

| Muster                      | Erlaubt                                                                     |
| :-------------------------- | :-------------------------------------------------------------------------- |
| `https://mcp.example.com/*` | Alle Pfade auf einer bestimmten Domain                                      |
| `https://mcp.example.com`   | Auch alle Pfade auf dieser Domain. Ein Muster ohne Pfad passt zu jedem Pfad |
| `https://*.example.com/*`   | Jede Subdomain von `example.com`                                            |
| `http://localhost:*/*`      | Jeder Port auf localhost                                                    |
| `*://mcp.example.com/*`     | Jedes Schema zu einer bestimmten Domain                                     |

Da `${VAR}`-Erweiterung die Prozessumgebung von Claude Code selbst liest, wird ein `serverCommand`- oder `serverUrl`-Richtlinieneintrag, der auf eine Variable verweist, zu dem Wert erweitert, den ein Benutzer setzt. Verwenden Sie wörtliche URLs und Befehle für Einträge, auf die Sie sich für die Durchsetzung verlassen.

<h3 id="example-configuration">
  Beispielkonfiguration
</h3>

Die folgende Konfiguration richtet eine harte Zulassungsliste mit einer Sperrliste ein. Die hervorgehobenen Zeilen ändern, wie der Rest der Liste bewertet wird, und die Callouts nach dem Block erklären jeweils:

```json {3,5,11} theme={null}
{
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://mcp.sentry.dev/*" },
    { "serverCommand": ["npx", "-y", "@modelcontextprotocol/server-filesystem", "."] },
    { "serverCommand": ["python", "/usr/local/bin/approved-server.py"] },
    { "serverUrl": "https://mcp.example.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ],
  "deniedMcpServers": [
    { "serverName": "dangerous-server" },
    { "serverCommand": ["npx", "-y", "unapproved-package"] },
    { "serverUrl": "https://*.untrusted.example.com/*" }
  ]
}
```

* **Zeile 3**: der erste `serverUrl`-Eintrag. Sobald einer existiert, muss jeder Remote-Server einem URL-Muster entsprechen, daher kann ein Benutzer keinen nicht aufgelisteten Remote-Server erhalten, indem er ihm einen zulässigen Namen gibt.
* **Zeile 5**: der erste `serverCommand`-Eintrag. Gleicher Effekt für Stdio-Server, daher muss jeder lokale Server genau einem aufgelisteten Befehl entsprechen.
* **Zeile 11**: ein `serverName`-Eintrag in der Sperrliste. Sperrlisten-Einträge gelten immer, daher wird jeder Server namens `dangerous-server` blockiert, unabhängig von seiner URL oder seinem Befehl.

Ein `serverName`-Eintrag in dieser Zulassungsliste würde niemals etwas entsprechen, da beide Transporttypen bereits strengere Einträge haben.

Die Akkordeons unten zeigen, wie ein Server gegen andere Zulassungslisten- und Sperrlisten-Kombinationen bewertet wird.

<Accordion title="Nur-URL-Zulassungsliste">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverUrl": "https://mcp.example.com/*" },
      { "serverUrl": "https://*.internal.example.com/*" }
    ]
  }
  ```

  | Server                                                   | Ergebnis                                                   |
  | :------------------------------------------------------- | :--------------------------------------------------------- |
  | HTTP-Server unter `https://mcp.example.com/api`          | Erlaubt: passt zu URL-Muster                               |
  | HTTP-Server unter `https://api.internal.example.com/mcp` | Erlaubt: passt zu Wildcard-Subdomain                       |
  | HTTP-Server unter `https://external.example.com/mcp`     | Blockiert: passt zu keinem URL-Muster                      |
  | Stdio-Server mit beliebigem Befehl                       | Blockiert: keine Name- oder Befehlseinträge zum Abgleichen |
</Accordion>

<Accordion title="Nur-Befehl-Zulassungsliste">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | Server                                               | Ergebnis                                      |
  | :--------------------------------------------------- | :-------------------------------------------- |
  | Stdio-Server mit `["npx", "-y", "approved-package"]` | Erlaubt: passt zu Befehl                      |
  | Stdio-Server mit `["node", "server.js"]`             | Blockiert: passt nicht zu Befehl              |
  | HTTP-Server namens `my-api`                          | Blockiert: keine Nameneinträge zum Abgleichen |
</Accordion>

<Accordion title="Gemischte Name- und Befehl-Zulassungsliste">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverName": "github" },
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | Server                                                                   | Ergebnis                                                                             |
  | :----------------------------------------------------------------------- | :----------------------------------------------------------------------------------- |
  | Stdio-Server namens `local-tool` mit `["npx", "-y", "approved-package"]` | Erlaubt: passt zu Befehl                                                             |
  | Stdio-Server namens `local-tool` mit `["node", "server.js"]`             | Blockiert: Befehlseinträge existieren, aber passt nicht                              |
  | Stdio-Server namens `github` mit `["node", "server.js"]`                 | Blockiert: Stdio-Server müssen Befehlen entsprechen, wenn Befehlseinträge existieren |
  | HTTP-Server namens `github`                                              | Erlaubt: passt zu Name                                                               |
  | HTTP-Server namens `other-api`                                           | Blockiert: Name passt nicht                                                          |
</Accordion>

<Accordion title="Nur-Name-Zulassungsliste">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverName": "github" },
      { "serverName": "internal-tool" }
    ]
  }
  ```

  | Server                                                    | Ergebnis                             |
  | :-------------------------------------------------------- | :----------------------------------- |
  | Stdio-Server namens `github` mit beliebigem Befehl        | Erlaubt: keine Befehlsbeschränkungen |
  | Stdio-Server namens `internal-tool` mit beliebigem Befehl | Erlaubt: keine Befehlsbeschränkungen |
  | HTTP-Server namens `github`                               | Erlaubt: passt zu Name               |
  | Jeder Server namens `other`                               | Blockiert: Name passt nicht          |
</Accordion>

<Accordion title="Zulassungsliste mit Sperrlisten-Überschreibung">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverUrl": "https://*.example.com/*" }
    ],
    "deniedMcpServers": [
      { "serverUrl": "https://staging.example.com/*" }
    ]
  }
  ```

  | Server                                              | Ergebnis                                                                         |
  | :-------------------------------------------------- | :------------------------------------------------------------------------------- |
  | HTTP-Server unter `https://mcp.example.com/api`     | Erlaubt: passt zu Zulassungslisten-URL-Muster, keine Sperrlisten-Übereinstimmung |
  | HTTP-Server unter `https://staging.example.com/api` | Blockiert: passt zu beiden, aber die Sperrliste hat Vorrang                      |
  | HTTP-Server unter `https://other.com/mcp`           | Blockiert: passt nicht zu Zulassungsliste                                        |
</Accordion>

<h3 id="restrict-the-allowlist-to-managed-settings-only">
  Beschränken Sie die Zulassungsliste auf verwaltete Einstellungen nur
</h3>

Um die verwaltete Zulassungsliste zur einzigen anzuwenden, setzen Sie `allowManagedMcpServersOnly` in der verwalteten Einstellungsdatei:

```json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverUrl": "https://api.githubcopilot.com/*" },
    { "serverUrl": "https://*.internal.example.com/*" }
  ]
}
```

Wenn `allowManagedMcpServersOnly` `true` ist, werden Zulassungslisten aus Benutzer-, Projekt- und lokalen Einstellungen ignoriert. Die Sperrliste wird immer noch aus allen Quellen zusammengeführt, daher können Benutzer Server immer für sich selbst blockieren.

<h2 id="how-restrictions-appear-to-users">
  Wie Einschränkungen für Benutzer angezeigt werden
</h2>

Wenn eine Einschränkung einen Server blockiert, sieht der Benutzer entweder einen Fehler von `claude mcp add` oder der Server wird stillschweigend nicht mehr geladen. Verwenden Sie diese Tabelle, um diese Berichte zu erkennen und um Benutzern mitzuteilen, was sie erwarten können, bevor Sie eine Änderung einführen:

| Einschränkung                                                                            | Was der Benutzer sieht                                                                                     |
| :--------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json` ist vorhanden und der Benutzer führt `claude mcp add` aus             | `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers` |
| Der Server ist auf einer Sperrliste und der Benutzer führt `claude mcp add` aus          | `Cannot add MCP server "<name>": server is explicitly blocked by enterprise policy`                        |
| Der Server ist nicht auf der Zulassungsliste und der Benutzer führt `claude mcp add` aus | `Cannot add MCP server "<name>": not allowed by enterprise policy`                                         |
| Ein zuvor konfigurierter Server wird jetzt durch Richtlinie blockiert                    | Der Server verschwindet stillschweigend aus `/mcp` und `claude mcp list` ohne Warnung                      |

Im letzten Fall erhält der Benutzer kein Signal, dass die Richtlinie der Grund ist, warum sein Server verschwunden ist, daher teilen Sie betroffenen Benutzern mit, welche Server blockiert werden, wenn Sie eine neue Einschränkung einführen.

<h2 id="monitor-mcp-usage">
  Überwachen Sie die MCP-Nutzung
</h2>

Wenn [OpenTelemetry-Export](/docs/de/monitoring-usage) konfiguriert ist, kann Claude Code aufzeichnen, welche MCP-Server und Tools Benutzer aufrufen. Setzen Sie `OTEL_LOG_TOOL_DETAILS=1`, um MCP-Server- und Tool-Namen in Tool-Events einzubeziehen, und aggregieren Sie sie dann in Ihrem Collector, um zu sehen, welche Server Ihre Benutzer tatsächlich verbinden. Siehe [Überwachung](/docs/de/monitoring-usage), um den Exporter einzurichten und das vollständige Event-Schema zu erhalten.

<h2 id="configuration-summary">
  Konfigurationszusammenfassung
</h2>

Jede Datei und Einstellung, die diese Seite behandelt, was sie kontrolliert und wie man sie bereitstellt:

| Oberfläche                   | Was es kontrolliert                                                           | Wo es sich befindet                                                                                                                                            | Wie man es bereitstellt                                                                                                                                                                          |
| :--------------------------- | :---------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json`           | Fester Serversatz, exklusive Kontrolle                                        | Systempfad: `/Library/Application Support/ClaudeCode/`, `/etc/claude-code/` oder `C:\Program Files\ClaudeCode\`                                                | MDM, GPO, Fleet-Verwaltung oder jeder Prozess mit Administratorrechten. Kann nicht über serververwaltete Einstellungen gesetzt werden                                                            |
| `allowedMcpServers`          | Zulassungsliste zulässiger Server                                             | Jede [Einstellungsdatei](/docs/de/settings#settings-files); Einträge aus jeder Quelle werden zusammengeführt, es sei denn, `allowManagedMcpServersOnly` ist gesetzt | Zur Durchsetzung eine [verwaltete Einstellungsquelle](/docs/de/admin-setup#decide-how-settings-reach-devices): serververwaltete Einstellungen, `managed-settings.json`, MDM-Profil oder Registrierung |
| `deniedMcpServers`           | Sperrliste blockierter Server                                                 | Jede Einstellungsdatei; Einträge aus jeder Quelle werden zusammengeführt                                                                                       | Gleich wie `allowedMcpServers`                                                                                                                                                                   |
| `allowManagedMcpServersOnly` | Sperrt die Zulassungsliste auf verwaltete Quellen nur                         | Nur verwaltete Einstellungsquellen; die Einstellung hat keine Auswirkung anderswo                                                                              | Gleich wie `allowedMcpServers`                                                                                                                                                                   |
| `allowAllClaudeAiMcps`       | Lädt claude.ai-Konnektoren neben `managed-mcp.json` statt sie zu unterdrücken | Nur verwaltete Einstellungsquellen; die Einstellung hat keine Auswirkung anderswo                                                                              | Gleich wie `allowedMcpServers`                                                                                                                                                                   |

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Entscheiden Sie, was Sie durchsetzen möchten](/docs/de/admin-setup#decide-what-to-enforce): MCP-Einschränkungen zusammen mit Berechtigungsregeln, Sandboxing und den anderen Admin-Kontrollen
* [Verbinden Sie Claude Code mit Tools über MCP](/docs/de/mcp): die vollständige MCP-Referenz, einschließlich Transporte, Bereiche und Authentifizierung
* [Einstellungen](/docs/de/settings): die Einstellungshierarchie und wie verwaltete Einstellungen Vorrang haben
* [Serververwaltete Einstellungen](/docs/de/server-managed-settings): Stellen Sie `allowedMcpServers` und `deniedMcpServers` aus der Claude.ai-Admin-Konsole bereit
* [Sicherheit](/docs/de/security): das Bedrohungsmodell, das diese Kontrollen schützen
* [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide): SSO, SCIM, Seat-Verwaltung und Rollout-Playbook
