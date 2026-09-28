> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kontrollieren Sie den MCP-Serverzugriff für Ihre Organisation

> Beschränken Sie, welche MCP-Server Benutzer hinzufügen oder verbinden können, oder stellen Sie Server für jeden Benutzer bereit, mit verwalteten Konfigurationsdateien, verwalteten Einstellungen, Zulassungslisten und Ablehnungslisten.

Standardmäßig kann jeder, der Claude Code ausführt, jeden beliebigen [MCP-Server](/docs/de/mcp) verbinden, den er wählt. Anthropic überprüft Konnektoren anhand seiner [Auflistungskriterien](https://claude.com/docs/connectors/building/review-criteria), bevor sie zum [Anthropic-Verzeichnis](https://claude.ai/directory) hinzugefügt werden, führt aber keine Sicherheitsprüfung durch und verwaltet keinen MCP-Server. Als Administrator können Sie einschränken, welche Server in Ihrer Organisation ausgeführt werden, von der Bereitstellung eines festen genehmigten Satzes bis zur vollständigen Deaktivierung von MCP, und Sie können Server für jeden Benutzer bereitstellen.

Diese Einschränkungen gelten für die Server, die Claude Code selbst lädt, einschließlich der Konnektoren, die es von claude.ai abruft. Konnektoren, die die Desktop-App an ihre lokalen und SSH-Sitzungen liefert, kommen prozessinternal an und werden stattdessen von Ihren claude.ai-Organisationseinstellungen aus gesteuert; [Wie Konnektoren Claude Code erreichen](/docs/de/mcp#how-connectors-reach-claude-code) zeigt, welche Kontrollen für Konnektoren in jeder Art von Sitzung gelten, einschließlich Cloud-Sitzungen.

Diese Seite behandelt, wie Sie:

* [Ein Muster wählen](#choose-a-pattern), das dem erforderlichen Kontrollumfang entspricht
* [Einen festen Serversatz mit `managed-mcp.json` bereitstellen](#exclusive-control-with-managed-mcp-json), einschließlich wie Sie [MCP vollständig deaktivieren](#disable-mcp-entirely)
* [Server durch verwaltete Einstellungen bereitstellen](#provide-servers-through-managed-settings), während Benutzer ihre eigenen behalten
* [Server mit Zulassungslisten und Ablehnungslisten kontrollieren](#policy-based-control-with-allowlists-and-denylists)
* [Benutzer informieren, was sie erwarten können](#how-restrictions-appear-to-users), wenn eine Einschränkung einen Server blockiert
* [Überwachen Sie, welche Server Ihre Organisation tatsächlich nutzt](#monitor-mcp-usage)

<Note>
  Die Seite [Sicherheit](/docs/de/security) behandelt das MCP-Bedrohungsmodell und wie Sie einen Server vor der Genehmigung bewerten. [Entscheiden Sie, was Sie durchsetzen möchten](/docs/de/admin-setup#decide-what-to-enforce) behandelt MCP-Einschränkungen zusammen mit den anderen administrativen Kontrollen.
</Note>

<h2 id="choose-a-pattern">
  Wählen Sie ein Muster
</h2>

Claude Code unterstützt eine Reihe von Einschränkungsstufen. Jedes Muster verwendet einen oder mehrere der folgenden Mechanismen: `managed-mcp.json` zum Bereitstellen eines festen Satzes, die verwaltete Einstellung `managedMcpServers` zum Bereitstellen von Servern neben den von Benutzern hinzugefügten und `allowedMcpServers`/`deniedMcpServers` zum Filtern dessen, was Benutzer konfigurieren.

| Muster                     | Funktion                                                                                                                                                                                                                                                        | Konfigurieren                                                                                                  |
| :------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------- |
| **MCP deaktivieren**       | Keine Server werden geladen, außer [In-Process-Servern, die die App registriert, die die Sitzung gestartet hat](#exclusive-control-with-managed-mcp-json) und alle, die Sie [über `managedMcpServers` bereitstellen](#provide-servers-through-managed-settings) | `managed-mcp.json` mit einer leeren Serverzuordnung                                                            |
| **Feste Bereitstellung**   | Jeder Benutzer erhält die gleichen Server und kann keine anderen hinzufügen                                                                                                                                                                                     | `managed-mcp.json` mit den gewünschten Servern                                                                 |
| **Bereitgestellte Server** | Jeder Benutzer erhält die Remote-Server, die Sie auflisten, und behält seine eigenen                                                                                                                                                                            | `managedMcpServers` in verwalteten Einstellungen                                                               |
| **Genehmigter Katalog**    | Veröffentlichen Sie eine Liste genehmigter Server; Benutzer fügen die gewünschten hinzu, alles andere wird blockiert                                                                                                                                            | `allowedMcpServers` + `allowManagedMcpServersOnly: true`                                                       |
| **Nur Plugin-Server**      | Benutzer können keine Server über `~/.claude.json` oder `.mcp.json` hinzufügen; Plugin-Server werden weiterhin geladen                                                                                                                                          | [`strictPluginOnlyCustomization`](/docs/de/settings-reference#strictpluginonlycustomization) mit `mcp` in der Liste |
| **Soft-Allowlist**         | Erzwingen Sie eine Allowlist, die Benutzer in ihren eigenen Einstellungen erweitern können                                                                                                                                                                      | `allowedMcpServers` ohne `allowManagedMcpServersOnly`                                                          |
| **Nur Denylist**           | Blockieren Sie bekannt schlechte Server, erlauben Sie alles andere                                                                                                                                                                                              | `deniedMcpServers`                                                                                             |
| **Keine Einschränkungen**  | Benutzer fügen alles hinzu                                                                                                                                                                                                                                      | Stellen Sie keine verwaltete MCP-Konfiguration bereit                                                          |

<Note>
  Claude Code hat keine integrierte MCP-Server-Registry, die Benutzer durchsuchen und installieren können. Für das Muster des genehmigten Katalogs teilen Sie die genehmigte Liste und ihre `claude mcp add`-Befehle an einem Ort, an dem Ihre Benutzer sie finden, z. B. in einem internen Wiki, oder verteilen Sie die Server als Plugins über einen [verwalteten Plugin-Marketplace](/docs/de/plugins/org#restrict-what-users-can-install), damit Benutzer sie von `/plugin` durchsuchen und installieren können.
</Note>

<h2 id="exclusive-control-with-managed-mcp-json">
  Exklusive Kontrolle mit managed-mcp.json
</h2>

Wenn Sie eine `managed-mcp.json`-Datei bereitstellen, lädt Claude Code nur diese MCP-Server:

* Die Server, die die Datei definiert
* Server, die Sie [über `managedMcpServers` bereitstellen](#provide-servers-through-managed-settings)
* In-Process-Server, die die Anwendung registriert, die die Sitzung gestartet hat, wie beispielsweise der eigene Server der VS Code-Erweiterung oder die [Konnektoren, die die Desktop-Anwendung bereitstellt](/docs/de/mcp#how-connectors-reach-claude-code)

Benutzer können keine anderen MCP-Server hinzufügen, ändern oder verwenden, einschließlich Plugin-bereitgestellter Server und Server, die mit dem [`--mcp-config`-CLI-Flag](/docs/de/cli-reference#cli-flags) übergeben werden. Die Datei unterdrückt auch die claude.ai-Konnektoren, die Claude Code selbst abruft, es sei denn, Sie [erlauben sie neben dem verwalteten Satz](#allow-claude-ai-connectors-alongside-the-managed-set).

<h3 id="deploy-managed-mcp-json">
  managed-mcp.json bereitstellen
</h3>

`managed-mcp.json` ist eine eigenständige Datei und kann daher nicht über [servergesteuerte Einstellungen](/docs/de/server-managed-settings) bereitgestellt werden. Um Server stattdessen über verwaltete Einstellungen bereitzustellen, ohne exklusive Kontrolle, verwenden Sie [`managedMcpServers`](#provide-servers-through-managed-settings).

Jeder Prozess, der in einen Systempfad mit Administratorrechten schreiben kann, kann die Datei bereitstellen. Über eine Flotte hinweg geschieht dies normalerweise über Geräteverwaltungstools wie Jamf oder ein Konfigurationsprofil auf macOS, Gruppenrichtlinie oder Intune unter Windows oder Ihre Flottenverwaltung Ihrer Wahl unter Linux. Claude Code sucht die Datei unter einem dieser Pfade:

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
  Mit benutzerspezifischen Anmeldedaten authentifizieren
</h3>

Jeder Benutzer auf dem Computer kann diese Datei lesen, daher speichern Sie keine API-Schlüssel oder andere Anmeldedaten in `env`-Blöcken. Übergeben Sie stattdessen benutzerspezifische Anmeldedaten mit einer dieser Optionen:

* [`${VAR}`-Erweiterung](/docs/de/mcp#environment-variable-expansion-in-mcp-json) zum Lesen von Geheimnissen aus der Umgebung jedes Benutzers.
* [OAuth oder benutzerspezifische Header](/docs/de/mcp#authenticate-with-remote-mcp-servers), damit sich jeder Benutzer selbst authentifiziert.
* [`headersHelper`](/docs/de/mcp#use-dynamic-headers-for-custom-authentication) zum Generieren von Anmeldedaten zum Verbindungszeitpunkt.

<h3 id="servers-passed-with-mcp-config-or-strict-mcp-config">
  Server, die mit `--mcp-config` oder `--strict-mcp-config` übergeben werden
</h3>

Wenn eine Sitzung Server über `--mcp-config` erhält, während eine `managed-mcp.json`, die Claude Code lesen und analysieren kann, bereitgestellt wird, unterscheidet sich das, was der Benutzer sieht, zwischen einer Workstation und einer Cloud-Sitzung:

* Auf einer Workstation beendet Claude Code beim Start mit `You cannot dynamically configure MCP servers when an enterprise MCP config is present`.
* In [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) auf einem Host, auf dem die Datei bereitgestellt wird, wie beispielsweise einem [selbstgehosteten Runner](/docs/de/self-hosted-environments-configuration#mcp-servers), startet Claude Code nur mit den verwalteten Servern und überspringt die claude.ai-Konnektoren und andere Server, die der Cloud-Host über `--mcp-config` bereitstellt. Nichts in der Sitzung teilt dem Benutzer mit, welche Server ausgelassen wurden. Claude Code benennt sie in einer Warnung auf stderr, die ein selbstgehosteter Runner auf der `debug`-Protokollebene aufzeichnet.

Das Flag `--strict-mcp-config` fordert auf, den verwalteten Satz zu ersetzen. Wenn ein Benutzer es übergibt, während eine solche Datei bereitgestellt wird, beendet Claude Code beim Start sowohl auf einer Workstation als auch in einer Cloud-Sitzung.

<h3 id="how-allowlists-and-denylists-apply-to-the-managed-set">
  Wie Zulassungslisten und Ablehnungslisten auf den verwalteten Satz angewendet werden
</h3>

Die Ablehnungsliste kann die Server in `managed-mcp.json` weiter filtern:

* `deniedMcpServers` gilt auch für verwaltete Server, daher wird ein verwalteter Server, der einem Eintrag entspricht, nicht geladen.
* Die eigene `deniedMcpServers` eines Benutzers wird aus seinen Einstellungen zusammengeführt, daher können Benutzer einen verwalteten Server für sich selbst blockieren.

`allowedMcpServers` gilt nicht für die Server in `managed-mcp.json`, mit einer Ausnahme: Claude Code überprüft immer noch einen Server, dessen Definition [`${VAR}`-Erweiterung](/docs/de/mcp#environment-variable-expansion-in-mcp-json) verwendet, gegen die Zulassungsliste, da die effektive Konfiguration dieses Servers aus der Umgebung jedes Benutzers stammt und nicht nur aus der Datei. Vor v2.1.259 musste jeder verwaltete Server die Zulassungsliste passieren, wenn eine gesetzt war. Siehe [Wie ein Server evaluiert wird](#how-a-server-is-evaluated) für die Felder, die die `${VAR}`-Überprüfung auslösen, und die vollständige Reihenfolge der Überprüfungen.

Wenn Sie `allowedMcpServers` verwendet haben, um zu verhindern, dass einige Ihrer eigenen `managed-mcp.json`-Server geladen werden, beginnen diese Server beim ersten Start jedes Benutzers von v2.1.259 oder später zu laden, es sei denn, sie verwenden `${VAR}`-Erweiterung, ohne Aufforderung oder Benachrichtigung: nur `deniedMcpServers` wird weiterhin von diesen Servern subtrahiert. Fügen Sie Ablehnungslisteneinträge für sie hinzu, oder stellen Sie eine separate `managed-mcp.json` pro Gruppe bereit, bevor Ihre Benutzer ein Upgrade durchführen.

<h3 id="validate-the-configuration">
  Konfiguration validieren
</h3>

Um zu bestätigen, dass die Datei wirksam ist, führen Sie zwei Überprüfungen auf einem verwalteten Computer durch:

1. `claude mcp list` zeigt nur die Server in `managed-mcp.json` plus alle, die Sie über `managedMcpServers` bereitstellen. Zwei weitere Ergebnisse bedeuten, dass etwas nicht stimmt:
   * Wenn die eigenen Server eines Benutzers immer noch angezeigt werden, wird die Datei nicht gelesen; überprüfen Sie den Pfad und die Berechtigungen auf den übergeordneten Verzeichnissen.
   * Wenn die Server der Datei nicht angezeigt werden und der Abschnitt `MCP config diagnostics` die Enterprise-Konfiguration als fehlgeschlagen beim Parsen markiert, kann Claude Code die Datei nicht lesen oder parsen. Beheben Sie den Fehler, den dieser Abschnitt benennt, und lassen Sie den Benutzer Claude Code neu starten.
2. `claude mcp add --transport http test https://example.com/mcp` schlägt mit `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers` fehl. Die URL muss kein echter Server sein, da die Richtlinienüberprüfung den Befehl ablehnt, bevor etwas kontaktiert wird.

<h3 id="disable-mcp-entirely">
  MCP vollständig deaktivieren
</h3>

Stellen Sie eine `managed-mcp.json` mit einer leeren Serverzuordnung bereit, um jeden MCP-Server außer [In-Process-Servern, die die Anwendung registriert, die die Sitzung gestartet hat](#exclusive-control-with-managed-mcp-json), zu blockieren:

```json theme={null}
{
  "mcpServers": {}
}
```

`claude mcp add` schlägt mit dem oben genannten Enterprise-Richtlinienfehler fehl. Server, die Benutzer zuvor konfiguriert hatten, werden beim nächsten Start einer Sitzung nicht mehr geladen, ohne Warnung, dass die Richtlinie der Grund ist. Server, die Sie über `managedMcpServers` bereitstellen, werden weiterhin unter einer leeren Zuordnung geladen, daher lassen Sie diesen Schlüssel auch ungesetzt, um MCP vollständig zu deaktivieren.

<h3 id="allow-claude-ai-connectors-alongside-the-managed-set">
  claude.ai-Konnektoren neben dem verwalteten Satz erlauben
</h3>

Standardmäßig unterdrückt die Bereitstellung von `managed-mcp.json` die [claude.ai-Konnektoren](/docs/de/mcp#use-mcp-servers-from-claude-ai), die Claude Code selbst abruft, einschließlich Konnektoren, die ein Administrator für die Organisation in der claude.ai-Verwaltungskonsole konfiguriert hat. Um diese Konnektoren neben den Servern in `managed-mcp.json` zu laden, setzen Sie `"allowAllClaudeAiMcps": true` in einer [verwalteten Einstellungsquelle](/docs/de/admin-setup#decide-how-settings-reach-devices).

Mit der aktivierten Einstellung lädt Claude Code die gleichen claude.ai-Konnektoren, die es laden würde, wenn `managed-mcp.json` nicht bereitgestellt würde. [Zulassungslisten und Ablehnungslisten](#policy-based-control-with-allowlists-and-denylists) gelten weiterhin für diese Konnektoren, daher können Sie bestimmte mit `deniedMcpServers` blockieren. Die Einstellung betrifft nur die claude.ai-Konnektoren, die Claude Code selbst abruft; Plugin-bereitgestellte Server bleiben unterdrückt.

Cloud-Sitzungen und die lokalen und SSH-Sitzungen der Desktop-Anwendung erhalten Konnektoren auf andere Weise, wie in [Wie Konnektoren Claude Code erreichen](/docs/de/mcp#how-connectors-reach-claude-code) beschrieben. Eine `managed-mcp.json` auf dem Host, der eine Cloud-Sitzung ausführt, wie beispielsweise ein [selbstgehosteter Runner-Host](/docs/de/self-hosted-environments-configuration#mcp-servers), unterdrückt die Konnektoren dieser Sitzung, unabhängig davon, ob Sie `allowAllClaudeAiMcps` setzen oder nicht. Keine `managed-mcp.json` erreicht die Konnektoren, die die Desktop-Anwendung an ihre lokalen und SSH-Sitzungen bereitstellt.

Claude Code liest `allowAllClaudeAiMcps` nur aus von Administratoren kontrollierten Richtlinienebenen: servergesteuerte Einstellungen, ein von MDM bereitgestellter plist- oder HKLM-Registrierungsschlüssel oder eine System-`managed-settings.json`-Datei. Das Platzieren in Benutzer- oder Projekteinstellungen hat keine Auswirkung, daher können Benutzer Konnektoren, die exklusive Kontrolle unterdrückt hat, nicht erneut aktivieren.

<h2 id="provide-servers-through-managed-settings">
  Server über verwaltete Einstellungen bereitstellen
</h2>

Um jedem Benutzer einen Satz von Remote-MCP-Servern zur Verfügung zu stellen, ohne die ausschließliche Kontrolle über MCP zu übernehmen, listen Sie diese unter `managedMcpServers` in einer [verwalteten Einstellungsquelle](/docs/de/admin-setup#decide-how-settings-reach-devices) auf: servergesteuerte Einstellungen, eine [Claude-Apps-Gateway](/docs/de/claude-apps-gateway-config#what-goes-in-cli)-Richtlinie, ein MDM-Profil oder eine Registrierungsrichtlinie oder `managed-settings.json`. Benutzer behalten die Server, die sie selbst hinzufügen, und erhalten Ihre zusätzlich. Erfordert Claude Code v2.1.259 oder später. Frühere Clients ignorieren den Schlüssel.

Der Wert ist ein Objekt, das nach Servername verschlüsselt ist. Jeder Eintrag hat die gleiche Form wie ein HTTP- oder SSE-Server in einer Projekt-[`.mcp.json`](/docs/de/mcp#project-scope)-Datei, einschließlich der optionalen `headers`- und `oauth`-Member, die in [Authentifizierung mit Remote-MCP-Servern](/docs/de/mcp#authenticate-with-remote-mcp-servers) beschrieben sind. Dieses Beispiel stellt einen Suchserver bereit, bei dem sich jeder Benutzer mit OAuth anmeldet, und einen Datensatzserver, der einen Header sendet, den Ihre Organisation ausgibt:

```json theme={null}
{
  "managedMcpServers": {
    "search": {
      "type": "http",
      "url": "https://search.example.com/mcp"
    },
    "records": {
      "type": "http",
      "url": "https://records.example.com/mcp",
      "headers": {
        "X-Records-Key": "key-issued-for-all-claude-code-users"
      }
    }
  }
}
```

Jeder, der die verwalteten Einstellungen auf einem Computer lesen kann, einschließlich des Benutzers, kann einen Header-Wert lesen, den Sie hier festlegen. Verwenden Sie eine Anmeldeinformation, die für diese gesamte Zielgruppe ausgestellt wurde, oder lassen Sie `headers` weg und lassen Sie jeden Benutzer sich mit OAuth anmelden.

<h3 id="what-an-entry-can-contain">
  Was ein Eintrag enthalten kann
</h3>

Claude Code lädt einen Eintrag nur, wenn er alle folgenden Überprüfungen besteht. Es verwirft einen Eintrag, der eine nicht besteht, zeichnet einen Hinweis auf, den Sie mit `/status` lesen können, und lädt trotzdem die anderen Einträge:

* `type` ist `http` oder `sse`. Wie in `.mcp.json` wird `streamable-http` als Alias für `http` akzeptiert.
* `url` ist eine `https://`-URL. Claude Code lehnt eine einfache `http://`-URL ab, einschließlich einer, die auf `localhost` verweist.
* Der Eintrag hat keinen `command`-, `args`-, `env`- oder `headersHelper`-Member, daher nennt ein verwaltetes Einstellungsdokument niemals ein Programm, das auf dem Computer eines Benutzers ausgeführt werden soll.
* Kein Wert enthält einen `${VAR}`-Verweis. Claude Code erweitert Umgebungsvariablen in diesen Einträgen nicht, daher schreiben Sie Literalwerte.
* Der Servername enthält nur Buchstaben, Zahlen, Bindestriche und Unterstriche, und kein Schlüssel oder Wert enthält Steuerzeichen oder unsichtbare Formatierungszeichen.

Claude Desktop hat eine verwaltete Einstellung mit dem gleichen Namen, deren Wert ein Array einer anderen Eintragform ist, daher kopieren Sie nicht eine in die andere. Claude Code akzeptiert die Array-Form nicht und zeichnet stattdessen einen Hinweis auf, anstatt sie zu laden.

Ein Claude-Apps-Gateway führt die gleichen Überprüfungen beim Starten durch; siehe [MCP-Server in einer Richtlinie](/docs/de/claude-apps-gateway-config#mcp-servers-in-a-policy).

<h3 id="how-provided-servers-load">
  Wie bereitgestellte Server geladen werden
</h3>

Diese Regeln entscheiden, was geladen wird, wenn ein bereitgestellter Server mit einer anderen Serverdefinition oder mit einer anderen Einstellung auf dieser Seite überlappt:

* Ein bereitgestellter Server hat Vorrang vor einem Server mit dem gleichen Namen im lokalen, Projekt- oder Benutzerbereich und vor einem Plugin-Server oder claude.ai-Connector, der auf die gleiche URL verweist.
* Wenn Sie auch `managed-mcp.json` bereitstellen, lädt Claude Code seine Server und die bereitgestellten Server zusammen, und der Eintrag der Datei hat Vorrang, wenn beide einen Namen definieren.
* Bereitgestellte Server werden weiterhin geladen, wenn [`strictPluginOnlyCustomization`](/docs/de/settings-reference#strictpluginonlycustomization) die `mcp`-Oberfläche sperrt.
* `deniedMcpServers` gilt für bereitgestellte Server, einschließlich Einträge aus den eigenen Einstellungen eines Benutzers, daher kann ein Benutzer einen für sich selbst blockieren. Bereitgestellte Server benötigen keinen `allowedMcpServers`-Eintrag.

Wenn Sie auch `managed-mcp.json` nicht bereitgestellt haben, behalten die Pro-Lauf-Flags ihre Bedeutung:

* Ein Server, den ein Benutzer mit `--mcp-config` unter dem gleichen Namen übergibt, ersetzt den bereitgestellten für diesen Lauf und wird gegen `allowedMcpServers` überprüft.
* `--strict-mcp-config` lässt bereitgestellte Server zusammen mit jedem anderen konfigurierten Server weg.

Mit bereitgestelltem `managed-mcp.json` verhalten sich beide Flags wie [Ausschließliche Kontrolle mit managed-mcp.json](#exclusive-control-with-managed-mcp-json) beschreibt.

<h3 id="what-users-can-see-and-change">
  Was Benutzer sehen und ändern können
</h3>

Benutzer können einen bereitgestellten Server nicht bearbeiten oder entfernen:

* `claude mcp remove` meldet, dass der Server von der Organisation bereitgestellt wird.
* Wenn Sie auch `managed-mcp.json` nicht bereitgestellt haben, wird ein Eintrag, den ein Benutzer unter dem gleichen Namen hinzufügt, gespeichert, aber nicht verwendet, während Ihrer vorhanden ist.
* Benutzer können einen bereitgestellten Server für sich selbst in [`/mcp`](/docs/de/mcp#disable-a-server-without-removing-it) immer noch ausschalten, das bereitgestellte Server unter **Managed MCPs** auflistet.

`claude mcp get` und `/mcp` zeigen die URL eines bereitgestellten Servers nur als seinen Host an, zum Beispiel `https://mcp.example.com/…`, und `claude mcp get` zeigt seine Header-Namen ohne ihre Werte an.

<h3 id="where-managedmcpservers-applies">
  Wo `managedMcpServers` gilt
</h3>

Claude Code liest `managedMcpServers` aus der verwalteten Quelle, die es unter [Wie Claude Code verwaltete Quellen kombiniert](/docs/de/managed-settings#how-claude-code-combines-managed-sources) auswählt. Wenn diese Quelle [`managedSourcesBehavior`](/docs/de/settings-reference#managedsourcesbehavior) auf `"merge"` setzt, stellt Claude Code stattdessen die Server von jeder Admin-Quelle bereit, und wenn zwei Quellen den gleichen Namen definieren, gilt der Eintrag der höher eingestuften Quelle vollständig. Es liest den Schlüssel niemals aus der benutzergeschriebenen HKCU-Registrierung, aus [übergeordneten Einstellungen, die ein Embedding-Host bereitstellt](/docs/de/managed-settings#parent-settings-from-embedding-hosts), oder aus Benutzer-, Projekt- oder lokalen Einstellungsdateien, wo es den Schlüssel mit einer Warnung verwirft.

Claude Code liest den Schlüssel nicht in der Code-Registerkarte der Claude Desktop-App bei einer Drittanbieter-Bereitstellung oder in den Cowork-Sitzungen der App, da Claude Desktop die MCP-Server dieser Sitzungen selbst bereitstellt und sperrt. `/status` und `claude doctor` sagen so, wenn Ihre verwalteten Einstellungen den Schlüssel dort tragen.

<h3 id="when-provided-servers-connect">
  Wenn bereitgestellte Server verbunden werden
</h3>

Wenn `managedMcpServers` über servergesteuerte Einstellungen ankommt, folgt sein Timing [Abruf- und Caching-Verhalten](/docs/de/server-managed-settings#fetch-and-caching-behavior):

* Auf einem Computer mit zwischengespeicherten Einstellungen hält Claude Code die zwischengespeicherte Kopie dieses Schlüssels zurück, bis der Server die Einstellungen für die Sitzung bestätigt, und wartet auf diese Bestätigung, bevor MCP-Server geladen werden. Wenn die Bestätigung fehlschlägt, wird die Sitzung ohne die bereitgestellten Server fortgesetzt und `/status` sagt, dass sie zurückgehalten werden.
* Beim ersten Start eines Computers, wenn noch nichts zwischengespeichert ist, verbindet eine interaktive Sitzung, die vor dem Eintreffen der Einstellungen startet, die bereitgestellten Server, sobald sie eintreffen, und ein `claude -p`-Lauf, der bereits gestartet hat, kann ohne sie beendet werden.

Mit [Gateway-Anmeldung](/docs/de/claude-apps-gateway-config#precedence-with-other-managed-sources) lädt Claude Code die Richtlinie vor dem Start der Sitzung, daher verzögert oder überspringt keiner der Fälle die bereitgestellten Server.

Interaktive Sitzungen, die bereits ausgeführt werden, wenden Ihre Änderungen am Schlüssel an:

* **Server hinzufügen**: Claude Code verbindet ihn, wenn die aktualisierten Einstellungen eintreffen, ohne einen Neustart.
* **Eintrag eines Servers ändern**: Diese Sitzungen verbinden sich mit der neuen Definition erneut.
* **Server entfernen**: Eine laufende interaktive Sitzung trennt ihn, sobald sie die geänderten Einstellungen liest. Ein nicht-interaktiver (`-p`)-Lauf behält ihn bis zum Ende.

<h2 id="policy-based-control-with-allowlists-and-denylists">
  Richtlinienbasierte Kontrolle mit Zulassungs- und Sperrlisten
</h2>

Zulassungs- und Sperrlisten filtern, welche konfigurierten Server geladen werden dürfen. Sie sind keine Registrierung: Ein Server muss immer noch von einem Benutzer, einem Plugin oder Ihrer Organisation hinzugefügt werden, bevor eine der beiden Listen auf ihn angewendet wird.

Server, die Ihre Organisation über `managedMcpServers` bereitstellt, werden ohne einen Zulassungslisten-Eintrag geladen, und [Wie ein Server bewertet wird](#how-a-server-is-evaluated) behandelt `managed-mcp.json`-Server. Die Sperrliste gilt für jeden Server, unabhängig davon, woher er kommt, mit Ausnahme von In-Process-`type: "sdk"`-Einträgen.

Um Server an Benutzer bereitzustellen, verwenden Sie [`managed-mcp.json`](#exclusive-control-with-managed-mcp-json) oder [`managedMcpServers`](#provide-servers-through-managed-settings). Beide Listen filtern auch Server, die mit dem [`--mcp-config` CLI-Flag](/docs/de/cli-reference#cli-flags) übergeben werden, mit Ausnahme von In-Process-`type: "sdk"`-Einträgen; `--strict-mcp-config` begrenzt, welche Konfigurationsdateien geladen werden, und umgeht keine der beiden Listen.

Um die Zulassungsliste verbindlich zu machen, setzen Sie `allowedMcpServers` und `allowManagedMcpServersOnly: true` zusammen in einer [verwalteten Einstellungsquelle](/docs/de/admin-setup#decide-how-settings-reach-devices), z. B. servergesteuerte Einstellungen oder eine bereitgestellte `managed-settings.json`-Datei.

Die Sperre gilt aus jeder von Administratoren kontrollierten verwalteten Quelle, sodass eine Sperrung in einer bereitgestellten Datei immer noch gilt, wenn auch servergesteuerte Einstellungen verwendet werden, die MCP nicht erwähnen. Während die Sperre aktiv ist, stammt die verwaltete Zulassungsliste aus der höchstrangigen Admin-Quelle, die eine setzt. Das Lesen der Sperre und der Zulassungsliste über Quellen hinweg erfordert Claude Code v2.1.273 oder später.

[Beschränken Sie die Zulassungsliste auf verwaltete Einstellungen nur](#restrict-the-allowlist-to-managed-settings-only) zeigt die Konfiguration.

Ohne `allowManagedMcpServersOnly` werden Zulassungslisten aus jedem Einstellungsbereich zusammengeführt, einschließlich der eigenen `~/.claude/settings.json` eines Benutzers, sodass ein Benutzer das, was Ihre Zulassungsliste erlaubt, erweitern kann. Sperrlisten werden unabhängig davon aus jedem Bereich zusammengeführt.

<Note>
  `allowManagedMcpServersOnly` ist getrennt von `allowManagedPermissionRulesOnly`, das [Berechtigungsregeln](/docs/de/permissions#managed-settings) nur sperrt. Das Setzen dieses Flags erzwingt nicht die MCP-Zulassungsliste.
</Note>

<h3 id="match-servers-by-url-command-or-name">
  Server nach URL, Befehl oder Name abgleichen
</h3>

`allowedMcpServers` und `deniedMcpServers` sind Listen von Einträgen. Jeder Eintrag ist ein Objekt mit einem einzelnen Schlüssel, der Server nach ihrer URL, ihrem Befehl oder ihrem Namen identifiziert:

| Schlüssel       | Gleicht ab                                                                                               | Verwenden für                             |
| :-------------- | :------------------------------------------------------------------------------------------------------- | :---------------------------------------- |
| `serverUrl`     | Eine Remote-Server-URL, exakt oder mit `*`-Platzhaltern                                                  | HTTP- und SSE-Server                      |
| `serverCommand` | Der genaue Befehl und die Argumente, die einen Stdio-Server starten                                      | Stdio-Server                              |
| `serverName`    | Die vom Benutzer zugewiesene Bezeichnung. Nur exakte Übereinstimmung; Platzhalter werden nicht erweitert | Beide Typen, aber siehe die Warnung unten |

Das Belassen von `allowedMcpServers` ungesetzt unterscheidet sich vom Setzen auf ein leeres Array:

| Einstellung         | Ungesetzt (Standard)   | Leeres Array `[]`                                                                      | Gefüllt                                                                                               |
| :------------------ | :--------------------- | :------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------- |
| `allowedMcpServers` | Alle Server erlaubt    | Keine Server erlaubt, außer [den eigenen der Organisation](#how-a-server-is-evaluated) | Nur übereinstimmende Server erlaubt, außer [den eigenen der Organisation](#how-a-server-is-evaluated) |
| `deniedMcpServers`  | Keine Server blockiert | Keine Server blockiert                                                                 | Übereinstimmende Server blockiert                                                                     |

Siehe [Ungültige Einträge in verwalteten Einstellungen](/docs/de/managed-settings#invalid-entries-in-managed-settings) für das, was passiert, wenn ein Eintrag die Schemavalidierung nicht besteht.

<Warning>
  Ein `serverName`-Eintrag in einer der beiden Listen ist keine Sicherheitskontrolle. Der Name ist die Bezeichnung, die ein Benutzer beim Ausführen von `claude mcp add` oder beim Bearbeiten einer Konfigurationsdatei zuweist, nicht der zugrunde liegende Server, sodass ein Benutzer jeden Server `github` nennen kann. Für claude.ai-Konnektoren ist der Name der Anzeigename, der von claude.ai zurückgegeben wird, was sich ändern kann. Um zu erzwingen, welche Server tatsächlich ausgeführt werden, fügen Sie `serverCommand`- oder `serverUrl`-Einträge hinzu.
</Warning>

Die `serverName`-Validierung unterscheidet sich zwischen den beiden Listen:

* In `deniedMcpServers` akzeptiert `serverName` jede nicht leere Zeichenkette ohne führende oder nachfolgende Leerzeichen, sodass Sie [claude.ai-Konnektoren](/docs/de/mcp#use-mcp-servers-from-claude-ai) nach ihrem Anzeigenamen blockieren können. Zum Beispiel blockiert `{ "serverName": "claude.ai Slack" }` den Slack-Konnektor. Bevorzugen Sie einen `serverUrl`-Eintrag, wenn die Sperre robust gegen Umbenennungen sein muss, oder wenn ein Konnektor-Name kollidiert und ein ` (N)`-Suffix erhält.
* In `allowedMcpServers` ist `serverName` auf Buchstaben, Zahlen, Bindestriche und Unterstriche beschränkt. Verwenden Sie `serverUrl`, um einen claude.ai-Konnektor auf die Zulassungsliste zu setzen, den Claude Code selbst abruft; für Konnektoren, die ein Cloud-Host an selbstgehostete Sitzungen liefert, verwenden Sie stattdessen die unter [Konnektor-Datenverkehr verlässt Ihr Netzwerk](/docs/de/self-hosted-environments-deploy#connector-traffic-leaves-your-network) aufgelisteten Einträge.

Um alle claude.ai-Konnektoren auszuschalten, die Claude Code selbst abruft, siehe [`disableClaudeAiConnectors`](/docs/de/mcp#disable-claude-ai-connectors).

<h3 id="how-a-server-is-evaluated">
  Wie ein Server bewertet wird
</h3>

Vor dem Laden eines Servers, einschließlich eines aus `managed-mcp.json`, führt Claude Code die drei folgenden Überprüfungen in Reihenfolge durch. Es führt sie erneut durch, wenn ein Benutzer einen Server erneut verbindet oder einen deaktivierten in `/mcp` wieder aktiviert. In-Process-`type: "sdk"`-Server, die [die App registriert, die die Sitzung gestartet hat](/docs/de/mcp#how-connectors-reach-claude-code), überspringen alle drei.

1. **Listen zusammenführen.** Zulassungs- und Sperrlisten-Einträge aus jedem Einstellungsbereich werden in eine Zulassungsliste und eine Sperrliste kombiniert. Wenn `allowManagedMcpServersOnly` `true` ist, wird nur die verwaltete Zulassungsliste beibehalten; die Sperrliste wird immer aus jedem Bereich zusammengeführt. Wenn mehr als eine verwaltete Quelle vorhanden ist, sagt [Schlüssel, die aus jeder Admin-Quelle gelesen werden](/docs/de/managed-settings#keys-read-from-every-admin-source), welche von ihnen die Listen des verwalteten Bereichs liefern.
2. **Sperrliste überprüfen.** Ein Server, der einem Sperrlisten-Eintrag entspricht, nach URL, Befehl oder Name, wird blockiert. Nichts überschreibt eine Sperrlisten-Übereinstimmung.
3. **Zulassungsliste überprüfen.** Wenn `allowedMcpServers` nirgendwo gesetzt ist, wird jeder Server, der die Sperrliste bestanden hat, geladen. Wenn es gesetzt ist, hängt das, dem der Server entsprechen muss, von seinem Typ ab, wie in der folgenden Tabelle gezeigt.

   Die eigenen Server der Organisation überspringen diese Überprüfung: jeder `managedMcpServers`-Eintrag und jeder `managed-mcp.json`-Eintrag, dessen Werte keine `${VAR}`-Erweiterung verwenden. Integrierte Server überspringen sie auch, z. B. Claude in Chrome, der `ide`-Server, mit dem Claude Code sich in einer laufenden VS Code- oder JetBrains-IDE verbindet, und Server, die die CLI selbst konfiguriert.

   Ein `managed-mcp.json`-Server, der `${VAR}`-Erweiterung in seinem Befehl, seinen Argumenten, `env`, URL oder Headern verwendet, wird immer noch überprüft, ebenso wie jeder Server, den ein Benutzer, ein Plugin, `--mcp-config` oder claude.ai hinzufügt.

| Servertyp              | Erlaubt, wenn es übereinstimmt                                                                                                            |
| :--------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| Remote (HTTP oder SSE) | Ein `serverUrl`-Eintrag. Eine `serverName`-Übereinstimmung zählt nur, wenn die Zulassungsliste keine `serverUrl`-Einträge enthält         |
| Stdio                  | Ein `serverCommand`-Eintrag. Eine `serverName`-Übereinstimmung zählt nur, wenn die Zulassungsliste keine `serverCommand`-Einträge enthält |

Drei Abgleichsregeln gelten innerhalb dieser Überprüfungen:

* **Befehle stimmen genau überein.** Jedes Argument, in Reihenfolge. `["npx", "-y", "server"]` stimmt nicht mit `["npx", "server"]` oder `["npx", "-y", "server", "--flag"]` überein.
* **`serverCommand`- und `serverUrl`-Werte werden vor dem Abgleich erweitert.** Sowohl der Richtlinieneintrag als auch der konfigurierte Wert des Servers durchlaufen [`${VAR}`- und `${VAR:-default}`-Erweiterung](/docs/de/mcp#environment-variable-expansion-in-mcp-json), sodass ein Eintrag, der als `["${HOME}/bin/server"]` geschrieben ist, mit einer Serverkonfiguration übereinstimmt, die entweder die gleiche Referenz oder den erweiterten Pfad verwendet. Unter Windows verweisen Sie auf eine Umgebungsvariable, die dort gesetzt ist, z. B. `${USERPROFILE}` statt `${HOME}`. `serverName`-Werte stimmen wörtlich überein und werden nie erweitert. Die beiden Seiten lesen unterschiedliche Umgebungen; [Wie Richtlinieneinträge erweitert werden](#how-policy-entries-expand) behandelt, welche und wie sich Zulassungs- und Sperrlisten-Einträge unterscheiden.
* **URLs unterstützen `*`-Platzhalter** überall im Muster, einschließlich des Schemas. Der Hostname-Abgleich ist nicht case-sensitiv und ignoriert einen nachgestellten FQDN-Punkt, sodass `https://Mcp.Example.com/*` mit `https://mcp.example.com/api` übereinstimmt. Pfade bleiben case-sensitiv.

| Muster                      | Erlaubt                                                                               |
| :-------------------------- | :------------------------------------------------------------------------------------ |
| `https://mcp.example.com/*` | Alle Pfade auf einer bestimmten Domain                                                |
| `https://mcp.example.com`   | Auch alle Pfade auf dieser Domain. Ein Muster ohne Pfad stimmt mit jedem Pfad überein |
| `https://*.example.com/*`   | Jede Subdomain von `example.com`                                                      |
| `http://localhost:*/*`      | Jeden Port auf localhost                                                              |
| `*://mcp.example.com/*`     | Jedes Schema zu einer bestimmten Domain                                               |

<h4 id="how-policy-entries-expand">
  Wie Richtlinieneinträge erweitert werden
</h4>

Der konfigurierte Wert des Servers wird aus der Live-Prozessumgebung erweitert, wie der Rest von `.mcp.json`. Ein Richtlinieneintrag wird stattdessen aus einer angehefteten Umgebung erweitert, sodass eine Variable, die von einer Projekt- oder Benutzereinstellungsdatei gesetzt wird, nicht ändern kann, was ein Zulassungslisten-Eintrag bedeutet. Da ein Richtlinieneintrag immer noch von der Umgebungsvariable der startenden Shell abhängt, verwenden Sie wörtliche URLs und Befehle für Einträge, auf die Sie sich für die Durchsetzung verlassen.

| Eintragsliste       | Erweitert aus                                                                                                                                                                                                                              | Erweiterung, die das Schema, den Host oder den Pfadbereich eines URL-Eintrags ändern würde |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------ |
| `allowedMcpServers` | Die Umgebung, mit der Claude Code gestartet wurde, plus `env`-Werte aus verwalteten Einstellungen                                                                                                                                          | Claude Code ignoriert den Eintrag                                                          |
| `deniedMcpServers`  | Das gleiche, und eine Variable ohne Startwert und ohne `:-default` wird aus Einstellungsdateien außerhalb des Repositorys gefüllt, z. B. Benutzer- oder verwaltete Einstellungen, die nur jemals das erweitern, dem der Eintrag entspricht | Der Eintrag stimmt immer noch überein                                                      |

Erfordert Claude Code v2.1.219 oder später.

<h3 id="example-configuration">
  Beispielkonfiguration
</h3>

Die folgende Konfiguration richtet eine strikte Zulassungsliste mit einer Sperrliste ein. Die hervorgehobenen Zeilen ändern, wie der Rest der Liste bewertet wird, und die Callouts nach dem Block erklären jeweils eine:

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

* **Zeile 3**: der erste `serverUrl`-Eintrag. Sobald einer existiert, muss jeder Remote-Server einem URL-Muster entsprechen, sodass ein Benutzer keinen nicht aufgelisteten Remote-Server durch Vergabe eines erlaubten Namens erhalten kann.
* **Zeile 5**: der erste `serverCommand`-Eintrag. Gleicher Effekt für Stdio-Server, sodass jeder lokale Server genau einem aufgelisteten Befehl entsprechen muss.
* **Zeile 11**: ein `serverName`-Eintrag in der Sperrliste. Sperrlisten-Einträge gelten immer, sodass jeder Server namens `dangerous-server` unabhängig von seiner URL oder seinem Befehl blockiert wird.

Ein `serverName`-Eintrag in dieser Zulassungsliste würde nie etwas abgleichen, da beide Transporttypen bereits strengere Einträge haben.

Die Akkordeons unten gehen durch, wie ein Server gegen andere Zulassungs- und Sperrlisten-Kombinationen bewertet wird.

<Accordion title="Nur URL-Zulassungsliste">
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
  | HTTP-Server unter `https://mcp.example.com/api`          | Erlaubt: stimmt mit URL-Muster überein                     |
  | HTTP-Server unter `https://api.internal.example.com/mcp` | Erlaubt: stimmt mit Wildcard-Subdomain überein             |
  | HTTP-Server unter `https://external.example.com/mcp`     | Blockiert: stimmt mit keinem URL-Muster überein            |
  | Stdio-Server mit beliebigem Befehl                       | Blockiert: keine Name- oder Befehlseinträge zum Abgleichen |
</Accordion>

<Accordion title="Nur Befehl-Zulassungsliste">
  ```json theme={null}
  {
    "allowedMcpServers": [
      { "serverCommand": ["npx", "-y", "approved-package"] }
    ]
  }
  ```

  | Server                                               | Ergebnis                                      |
  | :--------------------------------------------------- | :-------------------------------------------- |
  | Stdio-Server mit `["npx", "-y", "approved-package"]` | Erlaubt: stimmt mit Befehl überein            |
  | Stdio-Server mit `["node", "server.js"]`             | Blockiert: stimmt nicht mit Befehl überein    |
  | HTTP-Server namens `my-api`                          | Blockiert: keine Name-Einträge zum Abgleichen |
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
  | Stdio-Server namens `local-tool` mit `["npx", "-y", "approved-package"]` | Erlaubt: stimmt mit Befehl überein                                                   |
  | Stdio-Server namens `local-tool` mit `["node", "server.js"]`             | Blockiert: Befehlseinträge existieren, aber stimmt nicht überein                     |
  | Stdio-Server namens `github` mit `["node", "server.js"]`                 | Blockiert: Stdio-Server müssen Befehlen entsprechen, wenn Befehlseinträge existieren |
  | HTTP-Server namens `github`                                              | Erlaubt: stimmt mit Name überein                                                     |
  | HTTP-Server namens `other-api`                                           | Blockiert: Name stimmt nicht überein                                                 |
</Accordion>

<Accordion title="Nur Name-Zulassungsliste">
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
  | HTTP-Server namens `github`                               | Erlaubt: stimmt mit Name überein     |
  | Beliebiger Server namens `other`                          | Blockiert: Name stimmt nicht überein |
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

  | Server                                              | Ergebnis                                                                                   |
  | :-------------------------------------------------- | :----------------------------------------------------------------------------------------- |
  | HTTP-Server unter `https://mcp.example.com/api`     | Erlaubt: stimmt mit Zulassungslisten-URL-Muster überein, keine Sperrlisten-Übereinstimmung |
  | HTTP-Server unter `https://staging.example.com/api` | Blockiert: stimmt mit beiden überein, aber die Sperrliste hat Vorrang                      |
  | HTTP-Server unter `https://other.com/mcp`           | Blockiert: stimmt nicht mit der Zulassungsliste überein                                    |
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

Wenn `allowManagedMcpServersOnly` `true` ist, werden Zulassungslisten aus Benutzer-, Projekt- und lokalen Einstellungen ignoriert. Die Sperrliste wird immer noch aus jedem Einstellungsbereich zusammengeführt, sodass Benutzer Server immer für sich selbst blockieren können.

<h2 id="how-restrictions-appear-to-users">
  Wie Einschränkungen für Benutzer angezeigt werden
</h2>

Informationen dazu, was Benutzer beim Start sehen, wenn `managed-mcp.json` bereitgestellt wird und die Sitzung auch `--mcp-config`-Server hat, finden Sie unter [Exklusive Kontrolle mit managed-mcp.json](#exclusive-control-with-managed-mcp-json). Verwenden Sie diese Tabelle, um die anderen Berichte zu erkennen und Benutzern mitzuteilen, was sie erwarten können, bevor Sie eine Änderung einführen:

| Einschränkung                                                                                                                             | Was der Benutzer sieht                                                                                                       |
| :---------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `managed-mcp.json` ist vorhanden und der Benutzer führt `claude mcp add` aus                                                              | `Cannot add MCP server: enterprise MCP configuration is active and has exclusive control over MCP servers`                   |
| Der Server befindet sich auf einer Ablehnungsliste und der Benutzer führt `claude mcp add` aus                                            | `Cannot add MCP server "<name>": server is explicitly blocked by enterprise policy`                                          |
| Der Server befindet sich nicht auf der Zulassungsliste und der Benutzer führt `claude mcp add` aus                                        | `Cannot add MCP server "<name>": not allowed by enterprise policy`                                                           |
| Der Benutzer führt `claude mcp remove` auf einem Server aus `managedMcpServers` aus                                                       | `MCP server "<name>" is provided by your organization (managed settings) and cannot be removed locally.`                     |
| Ein zuvor konfigurierter Server wird jetzt durch eine Richtlinie blockiert                                                                | Der Server verschwindet aus `/mcp` und `claude mcp list`                                                                     |
| Ein Server wird blockiert, während eine Sitzung ausgeführt wird, und der Benutzer wählt **Reconnect** oder aktiviert ihn in `/mcp` erneut | [`MCP server <name> is blocked by enterprise managed policy`](/docs/de/errors#mcp-server-is-blocked-by-enterprise-managed-policy) |

Wenn ein Server stillschweigend verschwindet, erhält der Benutzer kein Signal dafür, dass die Richtlinie der Grund ist. Teilen Sie betroffenen Benutzern daher mit, welche Server blockiert sind, wenn Sie eine neue Einschränkung einführen.

<h2 id="monitor-mcp-usage">
  Überwachen Sie die MCP-Nutzung
</h2>

Wenn [OpenTelemetry-Export](/docs/de/monitoring-usage) konfiguriert ist, kann Claude Code aufzeichnen, welche MCP-Server und Tools Benutzer aufrufen. Setzen Sie `OTEL_LOG_TOOL_DETAILS=1`, um MCP-Server- und Tool-Namen in Tool-Events einzubeziehen, und aggregieren Sie sie dann in Ihrem Collector, um zu sehen, welche Server Ihre Benutzer tatsächlich verbinden. Siehe [Überwachung](/docs/de/monitoring-usage), um den Exporter einzurichten und das vollständige Event-Schema zu erhalten.

<h2 id="configuration-summary">
  Konfigurationszusammenfassung
</h2>

Jede Datei und Einstellung, die diese Seite behandelt, was sie kontrolliert und wie man sie bereitstellt:

| Oberfläche                   | Was es kontrolliert                                                                                                                                                                                                                                                            | Wo es sich befindet                                                                                                                                                                                                                                                  | Wie man es bereitstellt                                                                                                                                                                                       |
| :--------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `managed-mcp.json`           | Fester Serversatz, exklusive Kontrolle                                                                                                                                                                                                                                         | Systempfad: `/Library/Application Support/ClaudeCode/`, `/etc/claude-code/` oder `C:\Program Files\ClaudeCode\`                                                                                                                                                      | MDM, GPO, Fleet-Verwaltung oder jeder Prozess mit Administratorrechten. Kann nicht über serververwaltete Einstellungen gesetzt werden                                                                         |
| `managedMcpServers`          | Remote-Server, die jedem Benutzer neben seinen eigenen bereitgestellt werden                                                                                                                                                                                                   | Nur verwaltete Einstellungsquellen; die Einstellung hat keine Auswirkung anderswo                                                                                                                                                                                    | Eine [verwaltete Einstellungsquelle](/docs/de/admin-setup#decide-how-settings-reach-devices): serververwaltete Einstellungen, eine Gateway-Richtlinie, `managed-settings.json`, MDM-Profil oder HKLM-Registrierung |
| `allowedMcpServers`          | Zulassungsliste zulässiger Server                                                                                                                                                                                                                                              | Jede [Einstellungsbereich](/docs/de/settings#where-settings-live); [Wie ein Server bewertet wird](#how-a-server-is-evaluated) sagt, wie Listen aus mehreren Bereichen und verwalteten Quellen kombiniert werden                                                           | Zur Durchsetzung eine [verwaltete Einstellungsquelle](/docs/de/admin-setup#decide-how-settings-reach-devices): serververwaltete Einstellungen, `managed-settings.json`, MDM-Profil oder Registrierung              |
| `deniedMcpServers`           | Sperrliste blockierter Server                                                                                                                                                                                                                                                  | Jede Einstellungsbereich; [Wie ein Server bewertet wird](#how-a-server-is-evaluated) sagt, wie Listen aus mehreren Bereichen und verwalteten Quellen kombiniert werden                                                                                               | Gleich wie `allowedMcpServers`                                                                                                                                                                                |
| `allowManagedMcpServersOnly` | Sperrt die Zulassungsliste auf verwaltete Quellen nur                                                                                                                                                                                                                          | Nur verwaltete Einstellungsquellen; [Schlüssel, die aus jeder Admin-Quelle gelesen werden](/docs/de/managed-settings#keys-read-from-every-admin-source) sagt, welche verwalteten Quellen sie aktivieren können. Die Einstellung hat keine Auswirkung in anderen Bereichen | Gleich wie `allowedMcpServers`                                                                                                                                                                                |
| `allowAllClaudeAiMcps`       | Lädt die claude.ai-Konnektoren, die Claude Code selbst abruft, neben `managed-mcp.json`. [Eine `managed-mcp.json` auf dem Host, der eine Cloud-Sitzung ausführt, unterdrückt immer noch die Konnektoren dieser Sitzung](#allow-claude-ai-connectors-alongside-the-managed-set) | Nur verwaltete Einstellungsquellen; die Einstellung hat keine Auswirkung anderswo                                                                                                                                                                                    | Gleich wie `allowedMcpServers`                                                                                                                                                                                |

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Entscheiden Sie, was Sie durchsetzen möchten](/docs/de/admin-setup#decide-what-to-enforce): MCP-Einschränkungen zusammen mit Berechtigungsregeln, Sandboxing und den anderen Admin-Kontrollen
* [Verbinden Sie Claude Code mit Tools über MCP](/docs/de/mcp): die vollständige MCP-Referenz, einschließlich Transporte, Bereiche und Authentifizierung
* [Einstellungen](/docs/de/settings): die Einstellungshierarchie und wie verwaltete Einstellungen Vorrang haben
* [Serververwaltete Einstellungen](/docs/de/server-managed-settings): Stellen Sie `allowedMcpServers` und `deniedMcpServers` aus der Claude.ai-Admin-Konsole bereit
* [Sicherheit](/docs/de/security): das Bedrohungsmodell, das diese Kontrollen schützen
* [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide): SSO, SCIM, Seat-Verwaltung und Rollout-Playbook
