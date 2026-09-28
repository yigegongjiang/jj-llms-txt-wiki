> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Beispiel-Einstellungsdateien

> Realistische settings.json-Dateien für einen Entwickler, ein Team und eine Organisation: Kopieren Sie eine, behalten Sie die gewünschten Schlüssel und ändern Sie die Werte.

Diese Seite enthält drei Beispiel-`settings.json`-Dateien, eine für jeden Ort, an dem Sie eine Einstellung speichern:

* Die `~/.claude/settings.json` eines Entwicklers
* Die `.claude/settings.json` eines Teams, die im Repository committed ist
* Die `managed-settings.json` einer Organisation

Jede Datei ist eine plausible Datei für diesen Leser, sodass Sie die Form sehen und die gewünschten Teile kopieren können. Keine davon ist eine empfohlene Grundlage. Jeder Wert stammt aus dem Eintrag des Schlüssels in der [Einstellungsreferenz](/docs/de/settings-reference), die seinen Typ, den Standard und den Ort enthält, an dem er gesetzt werden kann.

Jedes Beispiel hat zwei Registerkarten. **Kopierbare Einstellungsdatei** ist die Datei, wie Sie sie speichern würden. **Was jeder Schlüssel tut** ist dieselbe Datei mit einem Kommentar über jedem Schlüssel; Claude Code akzeptiert keine Kommentare in einer Einstellungsdatei, daher kopieren Sie aus der ersten Registerkarte.

<h2 id="your-own-settings">
  Ihre eigenen Einstellungen
</h2>

Die persönlichen Einstellungen eines Entwicklers. Sie wählt ein Modell und eine Anstrengung aus, passt das Terminal an und genehmigt vorab einen schreibgeschützten Befehl und einen Dateilesevorgang. Alles, was nicht aufgelistet ist, behält seinen Standard. Eine Datei wie diese geht in `~/.claude/settings.json`, wo sie für jedes Projekt gilt, das Sie öffnen.

<Tabs>
  <Tab title="Kopierbare Einstellungsdatei">
    Speichern Sie dies als `~/.claude/settings.json`. Es ist gültiges JSON ohne Kommentare, daher können Sie es einfach einfügen und die Schlüssel löschen, die Sie nicht möchten.

    ```json ~/.claude/settings.json theme={null}
    {
      "model": "claude-sonnet-5",
      "modelSettings": {
        "claude-sonnet-5": { "effortLevel": "xhigh" }
      },
      "editorMode": "vim",
      "theme": "light-daltonized",
      "statusLine": {
        "type": "command",
        "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
        "padding": 2
      },
      "spinnerTipsEnabled": false,
      "preferredNotifChannel": "terminal_bell",
      "permissions": {
        "allow": [
          "Bash(git diff *)",
          "Read(~/.zshrc)"
        ]
      },
      "autoUpdatesChannel": "stable",
      "cleanupPeriodDays": 20
    }
    ```
  </Tab>

  <Tab title="Was jeder Schlüssel tut">
    Dieselbe Datei mit einem Kommentar über jedem Schlüssel. Lesen Sie sie hier; kopieren Sie aus der anderen Registerkarte, da Claude Code keine Kommentare in einer Einstellungsdatei akzeptiert.

    ```jsonc ~/.claude/settings.json theme={null}
    {
      // Starten Sie jede Sitzung mit Sonnet 5
      "model": "claude-sonnet-5",
      // Führen Sie Sonnet 5 über seine Standard-Hochstufe aus; /effort speichert eine Stufe pro Modell, und --effort setzt eine für eine einzelne Sitzung
      "modelSettings": {
        "claude-sonnet-5": { "effortLevel": "xhigh" }
      },
      // Vim-Tastenbindungen in der Eingabeaufforderung
      "editorMode": "vim",
      // Das farbenblindfreundliche helle Design
      "theme": "light-daltonized",
      // Eine Statuszeile unter der Eingabeaufforderung: Modellname und verwendeter Kontext
      "statusLine": {
        "type": "command",
        "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
        "padding": 2
      },
      // Verstecken Sie die Tipps, die sich unter dem Spinner drehen
      "spinnerTipsEnabled": false,
      // Läuten Sie die Terminal-Glocke für Benachrichtigungen, z. B. eine abgeschlossene Aufgabe oder eine wartende Berechtigungsaufforderung
      "preferredNotifChannel": "terminal_bell",
      // Lassen Sie Claude Code git diff ausführen und Ihre .zshrc lesen, ohne zu fragen
      "permissions": {
        "allow": [
          "Bash(git diff *)",
          "Read(~/.zshrc)"
        ]
      },
      // Nehmen Sie Updates aus dem stabilen Kanal
      "autoUpdatesChannel": "stable",
      // Löschen Sie Sitzungstranskripte und andere lokale Sitzungsdaten, die älter als 20 Tage sind
      "cleanupPeriodDays": 20
    }
    ```
  </Tab>
</Tabs>

<h2 id="a-teams-shared-settings">
  Gemeinsame Einstellungen eines Teams
</h2>

Die gemeinsamen Einstellungen eines Teams, die im Repository festgehalten werden, damit jeder, der es klont, die gleichen Berechtigungen, Hooks und das Plugin-Marketplace erhält. Speichern Sie eine Datei wie diese unter `.claude/settings.json` am oberen Ende des Repositories. Das sollten Sie vor dem Commit wissen:

* **Cloud-Sitzungen lesen sie auch.** Eine [Cloud-Sitzung](/docs/de/settings#settings-in-cloud-sessions) startet von einem Klon des Repositories, daher gilt die festgesetzte Datei auch dort.
* **Telemetrie gehört in verwaltete oder persönliche Einstellungen.** Claude Code ignoriert die [OpenTelemetry-Exporter-Variablen](/docs/de/settings-reference#variables-claude-code-ignores-in-env) in den Einstellungsdateien eines Repositories, mit Ausnahme einiger Werte, die Telemetrie deaktivieren. Legen Sie diese in [verwalteten Einstellungen](/docs/de/monitoring-usage#administrator-configuration) für Ihre Organisation fest, oder in der `~/.claude/settings.json` jeder Person.
* **Allow-Regeln warten auf Vertrauen.** Allow-Regeln und `extraKnownMarketplaces`-Einträge werden wirksam, nachdem jede Person [diesem Ordner selbst vertraut](/docs/de/permissions#project-allow-rules-and-workspace-trust), nicht nur einem übergeordneten Ordner; Deny- und Ask-Regeln gelten in jeder Sitzung, vertraut oder nicht.
* **Der Hook ist ein Skript im Repo.** Der Hook dieser Datei führt `.claude/hooks/block-rm.sh` aus; [Wie ein Hook aufgelöst wird](/docs/de/hooks#how-a-hook-resolves) zeigt, wie man ihn schreibt.
* **Regeln entsprechen dem Befehl und Pfad wie geschrieben.** `Bash(git push *)` entspricht nicht [`git -C . push`](/docs/de/permissions#bash-rule-limits). `Read(./.env)` allein stoppt die Datei-Tools und Befehle, die die Datei benennen, wie `cat .env`, aber nicht [`grep -r` über das Verzeichnis ausgeführt](/docs/de/permissions#read-and-edit); der `sandbox`-Block in dieser Datei schließt diese Lücke, da die Sandbox [Ihre `Read`-Deny-Pfade](/docs/de/settings-reference#sandbox-filesystem-denyread) zu dem hinzufügt, was jeder Sandbox-Befehl nicht lesen kann.

<Tabs>
  <Tab title="Kopierbare Einstellungsdatei">
    Speichern Sie diese als `.claude/settings.json` am oberen Ende des Repositories und committen Sie sie. Es ist gültiges JSON ohne Kommentare, daher können Sie es so einfügen und die Schlüssel löschen, die Sie nicht möchten.

    ```json .claude/settings.json theme={null}
    {
      "permissions": {
        "allow": [
          "Bash(npm run *)"
        ],
        "ask": [
          "Bash(git push *)"
        ],
        "deny": [
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ]
      },
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.sh"
              }
            ]
          }
        ]
      },
      "extraKnownMarketplaces": {
        "acme-tools": {
          "source": {
            "source": "github",
            "repo": "acme-corp/claude-plugins"
          }
        }
      },
      "enabledPlugins": {
        "code-formatter@acme-tools": true
      },
      "sandbox": {
        "enabled": true,
        "filesystem": {
          "allowWrite": [
            "/tmp/build"
          ]
        },
        "network": {
          "allowedDomains": [
            "registry.npmjs.org",
            "*.example.com"
          ]
        }
      },
      "plansDirectory": "./plans"
    }
    ```
  </Tab>

  <Tab title="Was jeder Schlüssel bewirkt">
    Die gleiche Datei mit einem Kommentar über jedem Schlüssel. Lesen Sie sie hier; kopieren Sie aus dem anderen Tab, da Claude Code keine Kommentare in einer Einstellungsdatei akzeptiert.

    ```jsonc .claude/settings.json theme={null}
    {
      "permissions": {
        // npm-Skripte ohne Nachfrage ausführen
        "allow": [
          "Bash(npm run *)"
        ],
        // Bestätigung vor git push-Befehlen
        "ask": [
          "Bash(git push *)"
        ],
        // Lesevorgänge von Env-Dateien und dem Secrets-Ordner durch die Datei-Tools und dateilesenden Befehle verweigern
        "deny": [
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ]
      },
      // Vor jedem Bash-Befehl ein Skript im Repo ausführen, das ihn blockieren kann
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.sh"
              }
            ]
          }
        ]
      },
      // Das Plugin-Marketplace des Teams bei jedem Klon registrieren
      "extraKnownMarketplaces": {
        "acme-tools": {
          "source": {
            "source": "github",
            "repo": "acme-corp/claude-plugins"
          }
        }
      },
      // Ein Plugin aus diesem Marketplace aktivieren; ein Plugin aus einer externen Quelle wie einem GitHub-Repository muss von jeder Person einmal installiert werden
      "enabledPlugins": {
        "code-formatter@acme-tools": true
      },
      // Sandbox-Befehle: beschreibbares Build-Verzeichnis; npm und example.com vorab erlaubt, andere Hosts fordern weiterhin auf
      "sandbox": {
        "enabled": true,
        "filesystem": {
          "allowWrite": [
            "/tmp/build"
          ]
        },
        "network": {
          "allowedDomains": [
            "registry.npmjs.org",
            "*.example.com"
          ]
        }
      },
      // Plan-Dateien im Repository behalten
      "plansDirectory": "./plans"
    }
    ```
  </Tab>
</Tabs>

<h2 id="an-organizations-managed-settings">
  Verwaltete Einstellungen einer Organisation
</h2>

Eine `managed-settings.json`-Datei, die die Form der verwalteten Schlüssel zeigt, mit einem plausiblen Wert für jeden. Es ist keine empfohlene Richtlinie: Wählen Sie die Schlüssel aus, die Ihren eigenen Anforderungen entsprechen, und legen Sie Ihre eigenen Werte fest. Das Beispiel setzt diese Schlüssel:

* `forceLoginMethod` und `forceLoginOrgUUID` fixieren die Anmeldemethode und Organisation
* `availableModels` und `enforceAvailableModels` beschränken, welche Modelle Sitzungen verwenden können
* `permissions.deny` blockiert zwei Dateilesevorgang und `curl`-Befehle [wie Claude sie schreibt](/docs/de/permissions#bash-rule-limits), und `disableBypassPermissionsMode` entfernt den Bypass-Berechtigungsmodus
* [`allowManagedPermissionRulesOnly`](/docs/de/settings-reference#allowmanagedpermissionrulesonly) und [`allowManagedMcpServersOnly`](/docs/de/settings-reference#allowmanagedmcpserversonly) machen die verwaltete Berechtigung und MCP-Zulassungslisten zu den einzigen, die gelten
* `allowedMcpServers` fixiert den MCP-Server nach URL
* `strictKnownMarketplaces` erlaubt einen Plugin-Marketplace
* `sandbox` sandboxed Befehle mit einer festen Netzwerk-Zulassungsliste und keinem unsandboxed Retry
* `requiredMinimumVersion` setzt eine Mindestversion für Claude Code
* `cleanupPeriodDays` verkürzt die Aufbewahrung von Sitzungstranskripten und anderen lokalen Daten auf sieben Tage
* `companyAnnouncements` zeigt eine Nachricht beim Start

Administratoren stellen eine Datei wie diese als `managed-settings.json` bereit, oder das gleiche JSON über MDM oder [servergesteuerte Einstellungen](/docs/de/server-managed-settings). Eine bereitgestellte Datei gilt für jeden Computer oder jedes Konto, das sie erreicht. Um einer Gruppe unterschiedliche Werte zu geben, stellen Sie eine andere Datei oder ein anderes Profil für diese Gruppe bereit, da [servergesteuerte Einstellungen noch keine Pro-Gruppen-Richtlinie unterstützen](/docs/de/server-managed-settings#current-limitations).

<Tabs>
  <Tab title="Kopierbare Einstellungsdatei">
    Stellen Sie dies als `managed-settings.json` bereit, oder das gleiche JSON über MDM oder die claude.ai-Konsole. Es ist gültiges JSON ohne Kommentare; ersetzen Sie die Beispiel-Organisations-UUID, Server-URL und den Marketplace durch Ihre eigenen und löschen Sie die Schlüssel, die Sie nicht möchten.

    ```json managed-settings.json theme={null}
    {
      "forceLoginMethod": "claudeai",
      "forceLoginOrgUUID": [
        "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
      ],
      "availableModels": [
        "opus",
        "sonnet"
      ],
      "enforceAvailableModels": true,
      "permissions": {
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true,
      "allowedMcpServers": [
        {
          "serverUrl": "https://api.githubcopilot.com/*"
        }
      ],
      "allowManagedMcpServersOnly": true,
      "strictKnownMarketplaces": [
        {
          "source": "github",
          "repo": "acme-corp/approved-plugins"
        }
      ],
      "sandbox": {
        "enabled": true,
        "failIfUnavailable": true,
        "allowUnsandboxedCommands": false,
        "network": {
          "allowedDomains": [
            "registry.npmjs.org",
            "github.com"
          ],
          "allowManagedDomainsOnly": true
        }
      },
      "requiredMinimumVersion": "2.1.150",
      "cleanupPeriodDays": 7,
      "companyAnnouncements": [
        "Welcome to Acme Corp! Review our code guidelines at docs.example.com"
      ]
    }
    ```
  </Tab>

  <Tab title="Was jeder Schlüssel tut">
    Dieselbe Datei mit einem Kommentar über jedem Schlüssel. Lesen Sie sie hier; kopieren Sie aus der anderen Registerkarte, da Claude Code keine Kommentare in einer Einstellungsdatei akzeptiert.

    ```jsonc managed-settings.json theme={null}
    {
      // Nur claude.ai-Anmeldungen, und nur in dieser Organisation
      "forceLoginMethod": "claudeai",
      "forceLoginOrgUUID": [
        "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
      ],
      // Nur Opus- und Sonnet-Modelle; mit enforceAvailableModels befolgt die Standard-Option auch die Liste
      "availableModels": [
        "opus",
        "sonnet"
      ],
      "enforceAvailableModels": true,
      "permissions": {
        // Blockieren Sie curl-Befehle und Lesevorgänge der .env-Datei des Projekts und des Secrets-Ordners auf jedem Computer
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        // Entfernen Sie den Bypass-Berechtigungsmodus aus jeder Sitzung
        "disableBypassPermissionsMode": "disable"
      },
      // Ignorieren Sie Berechtigungsregeln aus Benutzer-, Projekt- und lokalen Einstellungen
      "allowManagedPermissionRulesOnly": true,
      // Nur der GitHub MCP-Server, abgeglichen nach URL statt nach Name, da ein Benutzer jeden Server
      // "github" nennen kann. Server, die nicht übereinstimmen, werden nicht geladen, einschließlich
      // jeden stdio-Servers, wenn die Liste nur URL-Einträge hat. Der allowManagedMcpServersOnly-Schlüssel
      // unten macht diese verwaltete Liste zur einzigen Zulassungsliste, die zählt
      "allowedMcpServers": [
        {
          "serverUrl": "https://api.githubcopilot.com/*"
        }
      ],
      "allowManagedMcpServersOnly": true,
      // Plugins können nur aus diesem Marketplace stammen
      "strictKnownMarketplaces": [
        {
          "source": "github",
          "repo": "acme-corp/approved-plugins"
        }
      ],
      // Sandboxen Sie jeden Befehl, den Claude ausführt, weigern Sie sich zu starten, wenn die Sandbox nicht
      // eingerichtet werden kann, und lassen Sie einen blockierten Befehl niemals außerhalb der Sandbox erneut versuchen; Netzwerk
      // begrenzt auf npm und GitHub, und Benutzer können keine Domains hinzufügen
      "sandbox": {
        "enabled": true,
        "failIfUnavailable": true,
        "allowUnsandboxedCommands": false,
        "network": {
          "allowedDomains": [
            "registry.npmjs.org",
            "github.com"
          ],
          "allowManagedDomainsOnly": true
        }
      },
      // Weigern Sie sich zu starten auf Versionen älter als 2.1.150
      "requiredMinimumVersion": "2.1.150",
      // Löschen Sie Sitzungstranskripte und andere lokale Sitzungsdaten nach 7 Tagen
      "cleanupPeriodDays": 7,
      // Eine Nachricht, die jeder Benutzer beim Start sieht
      "companyAnnouncements": [
        "Welcome to Acme Corp! Review our code guidelines at docs.example.com"
      ]
    }
    ```
  </Tab>
</Tabs>
