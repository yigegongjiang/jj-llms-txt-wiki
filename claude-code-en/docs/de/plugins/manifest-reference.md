> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugin-Manifest-Referenz

> Vollständige Referenz für plugin.json: jedes Feld mit seinem Typ und Standard, akzeptierte Pfadformen und die userConfig- und Umgebungsvariablenschemas.

Ein Plugin-Manifest ist die Datei `plugin.json` im Verzeichnis `.claude-plugin/` eines Plugins. Sie enthält die Metadaten des Plugins und die [`userConfig`](#user-configuration)-Werte, die Claude Code den Benutzer auffordert einzugeben. Sie deklariert auch alle Komponenten, die Sie inline definieren oder außerhalb ihres [Standardorts](#standard-layout) speichern.

Diese Referenz ist für Plugin-Ersteller und für Marketplace-Besitzer, die Komponentenfelder in einen Marketplace-Eintrag einfügen.

<Note>
  Diese Fälle werden auf anderen Seiten behandelt:

  * **Erlernen des Plugin-Aufbaus**: Beginnen Sie mit [Plugin erstellen](/docs/de/plugins/create)
  * **Was jede Komponente zur Laufzeit tut**: siehe [Plugin-Komponenten](/docs/de/plugins/components)
</Note>

Beginnen Sie mit dem Abschnitt, der dem entspricht, was Sie nachschlagen:

* Ein Feld: die [Feldtabelle](#fields) gibt den Typ jedes Feldes, ob es erforderlich ist, seinen Standard und was es akzeptiert. [Pfadregeln](#path-rules) behandelt das `./`-Präfix und die Eindämmung für jeden Komponentenpfad
* Eine `userConfig`-Option oder einen `channels`-Eintrag: die [Benutzerkonfiguration](#user-configuration) und [Kanäle](#channels)-Schemas
* `${CLAUDE_PLUGIN_ROOT}` oder eine andere Variable, auf die ein Plugin verweisen kann: [Umgebungsvariablen](#environment-variables)
* Wo die Dateien jeder Komponente hingehen: [Standardlayout](#standard-layout)
* Eine Nachricht von `claude plugin validate`: die [Seite zur Fehlerbehebung](/docs/de/plugins/troubleshooting) listet jede Nachricht mit ihrer Lösung und Links zu den relevanten Abschnitten auf dieser Seite auf

<h2 id="manifest-file">
  Manifest-Datei
</h2>

Das Manifest ist optional. Ohne es lädt Claude Code die Komponenten, die es im [Standardlayout](#standard-layout) findet. Der Plugin-Name kommt dann aus dem Marketplace-Eintrag oder aus dem Verzeichnisnamen, wenn Sie das Plugin mit `--plugin-dir` laden.

Schreiben Sie ein Manifest, wenn Sie Metadaten, eine Komponente außerhalb ihres Standardverzeichnisses, `userConfig` oder eine inline Komponentendefinition möchten.

Speichern Sie das Manifest unter `.claude-plugin/plugin.json` im Plugin-Root. Legen Sie jede andere Plugin-Datei im Plugin-Root ab, nicht im `.claude-plugin/`-Verzeichnis. Das umfasst `skills/`, `commands/` und `hooks/`.

Das folgende Beispiel setzt die meisten Schlüssel in der [Feldtabelle](#fields). Es besteht die Validierung in einem Plugin-Verzeichnis, das jeden referenzierten Pfad enthält.

```json theme={null}
{
  "name": "deploy-tools",
  "displayName": "Deploy Tools",
  "version": "1.2.0",
  "description": "Deployment commands, a review agent, and a status monitor",
  "author": {
    "name": "Example Team",
    "email": "dev@example.com",
    "url": "https://example.com"
  },
  "homepage": "https://example.com/docs/deploy-tools",
  "repository": "https://github.com/example/deploy-tools",
  "license": "MIT",
  "keywords": ["deployment", "ci"],
  "defaultEnabled": true,
  "dependencies": ["secrets-vault"],
  "metadata": { "catalogId": "cat-123" },
  "skills": ["./extra-skills/"],
  "commands": {
    "status": {
      "source": "./commands/status.md",
      "description": "Show the current deployment status"
    },
    "about": {
      "content": "Explain what the deploy-tools plugin provides.",
      "description": "Describe this plugin"
    }
  },
  "agents": ["./agents/reviewer.md"],
  "hooks": "./config/extra-hooks.json",
  "mcpServers": {
    "deploy-api": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"]
    }
  },
  "lspServers": "./.lsp.json",
  "outputStyles": "./styles/",
  "experimental": {
    "themes": "./themes/",
    "monitors": "./config/monitors.json"
  },
  "userConfig": {
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "Token for the deployment API",
      "sensitive": true
    }
  }
}
```

<h3 id="unrecognized-fields">
  Nicht erkannte Felder
</h3>

Ein nicht erkannter Top-Level-Schlüssel wird entfernt, und ein nicht erkannter Schlüssel in einer `userConfig`-Option, einem `channels`-Eintrag, einer `lspServers`-Konfiguration oder einem `monitors`-Eintrag wird abgelehnt:

* **Top-Level-Felder**: Das Feld wird entfernt und das Plugin wird geladen. `claude plugin validate` meldet jedes nicht erkannte Top-Level-Feld als Warnung
* **Strikte Objekte**: `userConfig`-Optionen, `channels`-Einträge, `lspServers`-Konfigurationen und `monitors`-Einträge sind streng. Ein unbekannter Schlüssel in einem ist ein Fehler, und das Plugin wird nicht geladen

<h3 id="validate-the-manifest">
  Manifest validieren
</h3>

`claude plugin validate` ist die maßgebliche Überprüfung für ein Manifest. Führen Sie es von Ihrer Shell aus gegen das Plugin-Verzeichnis aus:

```bash theme={null}
claude plugin validate ./my-plugin
```

Der Befehl meldet eines dieser Ergebnisse:

* **`Validation passed`**: Das Manifest wird geladen
* **`Validation passed with warnings`**: Das Manifest wird geladen, aber der Validator hat etwas gefunden, das zu beheben ist, z. B. ein unbekanntes Top-Level-Feld, das Claude Code entfernt, ein `name`, der nicht in Kebab-Case ist, oder ein fehlender `version`, `description` oder `author`. Übergeben Sie `--strict`, um Warnungen in CI in Fehler umzuwandeln
* **`Validation failed`**: Das Manifest hat einen Typ-Mismatch, einen Pfad, der fehlt oder den Plugin-Root verlässt, oder einen unbekannten Schlüssel in einer `userConfig`-Option, einem `channels`-Eintrag, einer `lspServers`-Konfiguration oder einem `monitors`-Eintrag. Claude Code meldet das gleiche Problem, wenn es das Plugin lädt

<h2 id="fields">
  Felder
</h2>

Die Tabelle listet die Top-Level-Schlüssel in `plugin.json` auf. `name` ist der einzige erforderliche Schlüssel. Wenn ein Feldname ein Link ist, hat der verlinkte Abschnitt seine vollständigen Regeln.

Für Komponentenschlüssel wie `commands` und `hooks` zeigt [Komponentenpfadformen](#component-path-forms) jede akzeptierte Form mit einem Beispiel, und jeder Pfad folgt den [Pfadregeln](#path-rules) für das `./`-Präfix, Erweiterungen und Eindämmung.

| Feld                                 | Typ                                | Beschreibung                                                                                                                                                                                                                                                                                                                                     |
| :----------------------------------- | :--------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$schema`                            | String                             | JSON-Schema-URL für Editor-Autovervollständigung. Claude Code ignoriert es beim Laden                                                                                                                                                                                                                                                            |
| [`name`](#name)                      | String                             | Plugin-Identifier, erforderlich. Verwenden Sie Kebab-Case. Jede Komponente wird darunter namespaced                                                                                                                                                                                                                                              |
| [`displayName`](#displayname)        | String                             | Name, der in der Benutzeroberfläche anstelle von `name` angezeigt wird                                                                                                                                                                                                                                                                           |
| [`version`](#version)                | String                             | Versionszeichenfolge. Das Setzen hält Benutzer auf dieser Version, bis Sie sie ändern                                                                                                                                                                                                                                                            |
| `description`                        | String                             | Kurze Erklärung, was das Plugin bietet                                                                                                                                                                                                                                                                                                           |
| `author`                             | Object                             | `name`, das erforderlich ist, plus optionale `email` und `url`                                                                                                                                                                                                                                                                                   |
| `homepage`                           | String                             | Dokumentations-URL. Muss als URL analysierbar sein, oder das Plugin wird nicht geladen                                                                                                                                                                                                                                                           |
| `repository`                         | String                             | Quell-Repository-URL. Nicht validiert                                                                                                                                                                                                                                                                                                            |
| `license`                            | String                             | SPDX-Identifier wie `MIT` oder `Apache-2.0`                                                                                                                                                                                                                                                                                                      |
| `keywords`                           | Array von Strings                  | Discovery-Tags                                                                                                                                                                                                                                                                                                                                   |
| [`metadata`](#metadata)              | Object                             | Freiformobjekt für Ihre eigenen Daten. Claude Code liest es nicht                                                                                                                                                                                                                                                                                |
| [`defaultEnabled`](#defaultenabled)  | Boolean                            | Ob das Plugin aktiviert startet, wenn der Benutzer es nicht gesetzt hat. Standard ist `true`                                                                                                                                                                                                                                                     |
| [`dependencies`](#dependencies)      | Array von Strings oder Objekten    | Plugins, die aktiviert sein müssen, damit dieses funktioniert                                                                                                                                                                                                                                                                                    |
| [`settings`](#settings)              | Object                             | Einstellungen, die Claude Code anwendet, während das Plugin aktiviert ist. Nur `agent` und `subagentStatusLine` wirken sich aus                                                                                                                                                                                                                  |
| [`userConfig`](#user-configuration)  | Object                             | Werte, die Claude Code den Benutzer auffordert einzugeben, wenn das Plugin aktiviert ist                                                                                                                                                                                                                                                         |
| [`channels`](#channels)              | Array von Objekten                 | Nachrichtenkanäle, die das Plugin bereitstellt, jeweils an einen seiner MCP-Server gebunden                                                                                                                                                                                                                                                      |
| `skills`                             | Pfad oder Array von Pfaden         | Verzeichnisse zum Scannen nach Skills, jeweils ein Verzeichnis von `<name>/SKILL.md`-Ordnern oder ein Ordner mit `SKILL.md` direkt. `"."` benennt den Plugin-Root. Fügt zum Standard-`skills/`-Scan hinzu                                                                                                                                        |
| [`commands`](#commands)              | Pfad, Array von Pfaden oder Objekt | Flache `.md`-Befehlsdateien, Verzeichnisse davon oder eine Objektzuordnung von Befehlsname zu `source` oder `content`. Ersetzt den Standard-`commands/`-Scan                                                                                                                                                                                     |
| `agents`                             | Pfad oder Array von Pfaden         | Agent-`.md`-Dateien. Verzeichnisse werden nicht akzeptiert. Ersetzt den Standard-`agents/`-Scan                                                                                                                                                                                                                                                  |
| [`hooks`](#hooks)                    | Pfad, Objekt oder Array von beiden | `.json`-Hook-Dateien oder inline Hook-Konfiguration. Zusammen mit `hooks/hooks.json` geladen                                                                                                                                                                                                                                                     |
| [`mcpServers`](#mcpservers)          | Pfad, Objekt oder Array von beiden | `.json`-MCP-Konfigurationsdateien, `.mcpb`- oder `.dxt`-Bundles oder inline Server-Konfigurationen mit Namen als Schlüssel. Zusammen mit `.mcp.json` geladen; ein später deklarierter Server-Name ersetzt einen früheren                                                                                                                         |
| [`lspServers`](#lspservers)          | Pfad, Objekt oder Array von beiden | `.json`-LSP-Konfigurationsdateien oder inline Server-Konfigurationen mit Namen als Schlüssel. Zusammen mit `.lsp.json` geladen                                                                                                                                                                                                                   |
| `outputStyles`                       | Pfad oder Array von Pfaden         | Ausgabestil-Dateien oder Verzeichnisse. Ersetzt den Standard-`output-styles/`-Scan                                                                                                                                                                                                                                                               |
| `workflows`                          | Pfad oder Array von Pfaden         | [Workflow](/docs/de/workflows#distribute-a-workflow-in-a-plugin)-`.js`-Dateien oder Verzeichnisse. Ersetzt den Standard-`workflows/`-Scan                                                                                                                                                                                                             |
| `experimental`                       | Object                             | Container für `themes`, `monitors` und `evals`, deren Manifest-Form sich möglicherweise noch ändert                                                                                                                                                                                                                                              |
| `experimental.themes`                | Pfad oder Array von Pfaden         | Theme-Dateien oder Verzeichnisse. Ersetzt den Standard-`themes/`-Scan. Ein Top-Level-`themes`-Schlüssel wird immer noch geladen, mit einer `claude plugin validate`-Warnung                                                                                                                                                                      |
| [`experimental.monitors`](#monitors) | Pfad oder inline Array             | Eine `.json`-Datei mit dem Monitors-Array oder das Array selbst. Standard ist `monitors/monitors.json`. Ein Top-Level-`monitors`-Schlüssel wird immer noch geladen, mit einer `claude plugin validate`-Warnung. Monitore laufen nur in interaktiven Sitzungen und nicht auf Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry |
| `experimental.evals`                 | Pfad oder Array von Pfaden         | Verzeichnis, das die [Eval-Fälle](/docs/de/plugin-evals#use-a-different-eval-directory) des Plugins enthält, wenn es nicht das Standard-`evals/`-Verzeichnis ist. `claude plugin eval --eval-dir` überschreibt es                                                                                                                                     |

In der Spalte Typ ist ein Pfad ein String relativ zum Plugin-Root, z. B. `"./custom/commands"`.

<h3 id="name">
  `name`
</h3>

Der Plugin-Identifier. Er muss nicht leer sein, ohne Leerzeichen, `@`, `:`, Pfadtrennzeichen, Steuerzeichen oder bidirektionale Formatierungszeichen; verwenden Sie Kebab-Case.

Claude Code namespaced jede Komponente darunter, daher erscheint ein Agent `reviewer` im Plugin `deploy-tools` als `deploy-tools:reviewer`.

<h3 id="displayname">
  `displayName`
</h3>

Der Name, der in der Benutzeroberfläche anstelle von `name` angezeigt wird. Er kann Leerzeichen und beliebige Groß-/Kleinschreibung enthalten und wird nicht für Namespacing oder Lookup verwendet.

Für ein Marketplace-installiertes Plugin hat ein `displayName` im [Marketplace-Eintrag](/docs/de/plugins/marketplace-reference#plugin-entries) Vorrang vor diesem Wert.

<h3 id="version">
  `version`
</h3>

Eine Versionszeichenfolge, nicht gegen Semver überprüft. Das Setzen fixiert das Plugin auf diese Version, bis Sie es ändern; siehe [Versionen und Updates](/docs/de/plugins/loading#versions-and-updates). Ein Plugin mit einer [`command`-Quelle](/docs/de/plugins/marketplace-reference), ein Plugin aus einem [Marketplace, das auf claude.ai gehostet wird](/docs/de/plugins/install#add-from-claude-ai), und ein Plugin, das [an Ort und Stelle geladen wird](/docs/de/plugins/loading#find-plugins-on-disk) aus einem Marketplace, der als lokales Verzeichnis hinzugefügt wurde, werden nicht durch dieses Feld fixiert.

<h3 id="metadata">
  `metadata`
</h3>

Ein Freiformobjekt für Ihre eigenen Daten, z. B. Katalog- oder Berechtigungsfelder. Claude Code liest es nicht. Erfordert Claude Code v2.1.222 oder später.

<h3 id="defaultenabled">
  `defaultEnabled`
</h3>

Ob das Plugin aktiviert startet, wenn der Benutzer es nicht in [`enabledPlugins`](/docs/de/settings-reference#enabledplugins) gesetzt hat. Standard ist `true`. Ein Plugin, von dem ein aktiviertes Plugin abhängt, startet unabhängig aktiviert. Das gleiche Feld im Marketplace-Eintrag überschreibt dieses.

Sobald der `enabledPlugins`-Eintrag eines Benutzers geschrieben wird, bleibt er über Plugin-Updates hinweg bestehen, daher ändert das Ändern von `defaultEnabled` in einer späteren Version die Einstellung für einen bestehenden Benutzer nicht.

<h3 id="dependencies">
  `dependencies`
</h3>

Plugins, die aktiviert sein müssen, damit dieses funktioniert. Jeder Eintrag ist `"name"`, `"name@marketplace"` oder `{ "name": "...", "marketplace": "...", "version": "..." }`. Bare Namen werden gegen diesen Plugin-eigenen Marketplace aufgelöst. Siehe [Abhängigkeitsbeschränkungen](/docs/de/plugins/dependencies).

<h3 id="settings">
  `settings`
</h3>

Einstellungen, die Claude Code anwendet, während das Plugin aktiviert ist. Nur `agent` und `subagentStatusLine` wirken sich aus; andere Schlüssel werden beim Laden gelöscht. Eine `settings.json` im Plugin-Root hat Vorrang vor diesem Schlüssel. Siehe [Standardeinstellungen](/docs/de/plugins/components#default-settings).

<h2 id="component-path-forms">
  Komponentenpfadformen
</h2>

Jeder Komponentenschlüssel akzeptiert einen Pfad relativ zum Plugin-Root. `hooks`, `mcpServers`, `lspServers` und `experimental.monitors` akzeptieren auch inline Konfiguration, `commands` akzeptiert auch eine Objektzuordnung, und `mcpServers` akzeptiert auch MCP-Bundle-Pfade und URLs. Die folgenden Beispiele zeigen jede akzeptierte Form einmal. Für das, was jede Komponente zur Laufzeit tut, siehe [Plugin-Komponenten](/docs/de/plugins/components).

<h3 id="path-only-fields">
  Nur-Pfad-Felder
</h3>

`agents`, `skills`, `outputStyles`, `workflows` und `experimental.themes` nehmen einen Pfad oder ein Array von Pfaden. `agents`-Einträge müssen `.md`-Dateien sein, und `skills`-Einträge müssen Verzeichnisse sein. Die anderen drei akzeptieren ein Verzeichnis oder eine Datei.

```json theme={null}
{
  "agents": ["./custom-agents/reviewer.md", "./custom-agents/tester.md"],
  "skills": ["./extra-skills/", "."],
  "outputStyles": "./styles/"
}
```

<h3 id="commands">
  `commands`
</h3>

`commands` nimmt einen Pfad, ein Array von Pfaden oder eine Objektzuordnung. Ein Pfad benennt eine flache `.md`-Befehlsdatei oder ein Verzeichnis. In der Objektzuordnung wird jeder Schlüssel zum Befehlsnamen nach dem Plugin-Präfix. Zum Beispiel wird `"about"` im Plugin `deploy-tools` als `/deploy-tools:about` ausgeführt.

Jeder Wert setzt genau einen von `source` oder `content`, und ein Eintrag, der beide oder keinen setzt, schlägt die Validierung fehl. Die anderen Felder in dieser Tabelle sind optional:

| Feld           | Typ              | Beschreibung                                                               |
| :------------- | :--------------- | :------------------------------------------------------------------------- |
| `source`       | string           | Pfad zur Markdown-Datei des Befehls, relativ zum Plugin-Root               |
| `content`      | string           | Inline-Markdown für den Befehlstext, anstelle von `source`                 |
| `description`  | string           | Beschreibung, die für den Befehl angezeigt wird                            |
| `argumentHint` | string           | Argument-Hinweis, der nach dem Befehlsnamen angezeigt wird, z. B. `[file]` |
| `model`        | string           | Standardmodell für den Befehl                                              |
| `allowedTools` | array of strings | Tools, die der Befehl ohne Aufforderung verwenden darf                     |

Diese Zuordnung deklariert einen Befehl aus einer Datei und einen aus inline Inhalt:

```json theme={null}
{
  "commands": {
    "status": { "source": "./commands/status.md", "argumentHint": "[env]" },
    "about": { "content": "Explain what this plugin provides." }
  }
}
```

<h3 id="hooks">
  `hooks`
</h3>

`hooks` nimmt einen `.json`-Dateipfad, ein inline Hooks-Objekt in der gleichen Form wie [`hooks` in `settings.json`](/docs/de/hooks#configuration), oder ein Array, das beide mischt. Für Hook-Ereignisse und Handler-Felder siehe die [Hooks-Referenz](/docs/de/hooks#hook-events).

Claude Code führt zusammen, was Sie mit `hooks/hooks.json` deklarieren, wenn diese Datei existiert.

```json theme={null}
{
  "hooks": [
    "./config/extra-hooks.json",
    {
      "PostToolUse": [
        {
          "matcher": "Write|Edit",
          "hooks": [
            { "type": "command", "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/format.sh" }
          ]
        }
      ]
    }
  ]
}
```

<h3 id="mcpservers">
  `mcpServers`
</h3>

`mcpServers` nimmt einen `.json`-Dateipfad, einen MCP-Bundle-Pfad oder eine URL, eine inline Zuordnung oder ein Array, das diese mischt. Für Server-Konfigurationsfelder siehe [Plugin-bereitgestellte MCP-Server](/docs/de/mcp#plugin-provided-mcp-servers).

Claude Code lädt zuerst `.mcp.json` im Plugin-Root, dann jede deklarierte Form in Reihenfolge. Ein später deklarierter Server-Name ersetzt einen früheren.

Ein `mcpServers`-Wert nimmt eine dieser Formen:

| Form              | Beispielwert                                                                           | Was Claude Code tut                                                                                              |
| :---------------- | :------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| `.json`-Dateipfad | `"./mcp/servers.json"`                                                                 | Liest die Datei als `mcpServers`-Zuordnung                                                                       |
| MCP-Bundle-Pfad   | `"./bundle.mcpb"`                                                                      | Extrahiert das `.mcpb`- oder `.dxt`-Bundle in `.mcpb-cache/` im Plugin-Root und liest seine Server-Konfiguration |
| MCP-Bundle-URL    | `"https://example.com/server.mcpb"`                                                    | Lädt das Bundle in `.mcpb-cache/` herunter, dann liest es                                                        |
| Inline-Zuordnung  | `{ "deploy-api": { "command": "node", "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"] } }` | Verwendet die Zuordnung als Server-Konfigurationen mit Namen als Schlüssel                                       |

Ein Bundle-Pfad oder eine URL muss mit `.mcpb` oder `.dxt` enden. Jede andere Erweiterung schlägt die Validierung fehl.

<h3 id="lspservers">
  `lspServers`
</h3>

`lspServers` nimmt einen `.json`-Dateipfad, eine inline Zuordnung von Server-Name zu Konfiguration oder ein Array von beiden.

Claude Code lädt zuerst `.lsp.json` im Plugin-Root, dann jede deklarierte Konfiguration in Reihenfolge. Ein später deklarierter Server-Name ersetzt einen früheren.

Jede Server-Konfiguration ist ein striktes Objekt mit diesen Feldern. Ein unbekannter Schlüssel schlägt die Validierung fehl.

| Feld                    | Erforderlich | Beschreibung                                                                                                                                                                                     |
| :---------------------- | :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`               | Ja           | Language-Server-Binärdatei. Keine Leerzeichen, es sei denn, der Wert beginnt mit `/`; Argumente in `args` einfügen                                                                               |
| `extensionToLanguage`   | Ja           | Zuordnung von Dateierweiterung zu LSP-Sprach-ID, mindestens ein Eintrag. Schlüssel beginnen mit einem Punkt, z. B. `".go"`                                                                       |
| `args`                  | Nein         | Argumente, die an den Server übergeben werden                                                                                                                                                    |
| `transport`             | Nein         | Kommunikationstransport: `stdio` (Standard) oder `socket`. Claude Code akzeptiert `socket`, führt aber jeden Server über stdio aus, daher gelten die stdout-Protokollregeln für alle Server      |
| `env`                   | Nein         | Umgebungsvariablen für den Server-Prozess                                                                                                                                                        |
| `initializationOptions` | Nein         | Optionen, die in der Initialize-Anfrage gesendet werden                                                                                                                                          |
| `settings`              | Nein         | Einstellungen, die von `workspace/didChangeConfiguration` gesendet werden                                                                                                                        |
| `workspaceFolder`       | Nein         | Workspace-Ordnerpfad für den Server                                                                                                                                                              |
| `startupTimeout`        | Nein         | Millisekunden zum Warten auf den Start, eine positive Ganzzahl                                                                                                                                   |
| `shutdownTimeout`       | Nein         | Millisekunden zum Warten auf ein ordnungsgemäßes Herunterfahren, eine positive Ganzzahl. Wenn das Timeout abläuft, beendet Claude Code den Server-Prozess. Wenn nicht gesetzt, gilt kein Timeout |
| `restartOnCrash`        | Nein         | Ob der Server nach einem Absturz neu gestartet werden soll. Standard ist `true`. Setzen Sie auf `false`, um einen abgestürzten Server gestoppt zu lassen, anstatt ihn neu zu starten             |
| `maxRestarts`           | Nein         | Neustartversuche, bevor aufgegeben wird, null oder mehr                                                                                                                                          |
| `diagnostics`           | Nein         | Ob Diagnosen nach Bearbeitungen in den Kontext gepusht werden sollen. Standard ist `true`                                                                                                        |

Diese inline Konfiguration führt `gopls` für `.go`-Dateien aus:

```json theme={null}
{
  "lspServers": {
    "go": {
      "command": "gopls",
      "args": ["serve"],
      "extensionToLanguage": { ".go": "go" }
    }
  }
}
```

Für die Language-Server, die Anthropic als Plugins veröffentlicht, und wie sich die Server zur Laufzeit verhalten, siehe [Code-Intelligenz](/docs/de/plugins/code-intelligence).

<h3 id="monitors">
  `monitors`
</h3>

`experimental.monitors` nimmt einen `.json`-Dateipfad oder das inline Array. Wenn Sie den Schlüssel weglassen, lädt Claude Code `monitors/monitors.json`, falls vorhanden.

Jeder Eintrag ist ein striktes Objekt mit diesen Feldern.

| Feld          | Erforderlich | Beschreibung                                                                                                                                                                             |
| :------------ | :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | Ja           | Identifier, eindeutig im Plugin                                                                                                                                                          |
| `command`     | Ja           | Shell-Befehl, den Claude Code als persistenten Hintergrundprozess im Sitzungsarbeitsverzeichnis ausführt                                                                                 |
| `description` | Ja           | Kurze Zusammenfassung, die im Task-Panel und in Benachrichtigungszusammenfassungen angezeigt wird                                                                                        |
| `when`        | Nein         | Mit `"always"`, dem Standard, startet der Monitor beim Sitzungsstart und beim Plugin-Reload. Mit `"on-skill-invoke:<skill>"` startet er das erste Mal, wenn dieser Skill ausgeführt wird |

Dieses inline Array deklariert einen Monitor, der das erste Mal startet, wenn der `deploy`-Skill ausgeführt wird:

```json theme={null}
{
  "experimental": {
    "monitors": [
      {
        "name": "deploy-status",
        "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/poll-deploy.sh",
        "description": "Deployment status changes",
        "when": "on-skill-invoke:deploy"
      }
    ]
  }
}
```

Ein Monitor-`command` kann nicht auf `${user_config.*}` verweisen. Siehe [Felder, die durch eine Shell laufen](#fields-that-run-through-a-shell).

<h2 id="path-rules">
  Pfadregeln
</h2>

Jeder Komponentenpfad in einem Manifest ist relativ zum Plugin-Root und muss mit `./` beginnen. Ein Pfad wie `commands/foo.md` schlägt die Validierung fehl. `skills` und `mcpServers` akzeptieren jeweils eine Form außerhalb dieser Regel:

* **`skills`**: akzeptiert auch `"."`. Sowohl `"."` als auch `"./"` bezeichnen den Plugin-Root. Vor v2.1.221 schlugen `"."` die Manifest-Validierung fehl, daher verwenden Sie `"./"`, wenn das Plugin auf früheren Versionen geladen werden muss
* **`mcpServers`**: akzeptiert auch eine `https://`-Bundle-URL

<h3 id="containment-and-existence">
  Eindämmung und Existenz
</h3>

Jeder Komponentenpfad muss sich im Plugin-Root auflösen und muss existieren. `claude plugin validate` überprüft nicht die `outputStyles`-, `lspServers`-, `monitors`- oder `themes`-Pfade, daher schlägt ein schlechter Pfad in diesen Feldern nur fehl, wenn das Plugin geladen wird:

* **Eindämmung**: Ein Pfad, der sich außerhalb des Plugin-Root auflöst, wird nicht geladen, und die `/plugin`-Registerkarte **Errors** zeigt `<component> path escapes plugin directory: <path>`. Ein Pfad mit `..` ist der übliche Fall, und `claude plugin validate` meldet ihn als `Path contains ".." which could be a path traversal attempt`
* **Existenz**: Ein Pfad, der nicht existiert, wird nicht geladen, und die `/plugin`-Registerkarte **Errors** zeigt `<component> path not found: <path>`. `claude plugin validate` meldet ihn als `Path not found`

<h3 id="how-each-key-combines-with-its-default-location">
  Wie jeder Schlüssel mit seinem Standardort kombiniert wird
</h3>

Jeder Komponentenschlüssel ersetzt seinen Standardort, fügt zu ihm hinzu oder führt ihn zusammen:

* **Ersetzt den Standard**: `commands`, `agents`, `outputStyles`, `workflows`, `experimental.themes`, `experimental.monitors`. Wenn Sie `commands` setzen, wird das Standard-`commands/`-Verzeichnis nicht gescannt. Um den Standard zu behalten und mehr hinzuzufügen, listen Sie ihn explizit auf: `"commands": ["./commands/", "./extras/"]`
* **Fügt zum Standard hinzu**: `skills`. Das `skills/`-Verzeichnis wird immer noch gescannt, und die aufgelisteten Verzeichnisse werden zusammen mit ihm geladen
* **Führt zusammen**: `hooks`, `mcpServers`, `lspServers`. Die Standarddatei wird zuerst geladen, und was das Manifest deklariert, wird zusammengeführt, wie unter [Komponentenpfadformen](#component-path-forms) beschrieben

Wenn ein Plugin einen Standardordner wie `commands/` hat und auch den Manifest-Schlüssel setzt, der ihn ersetzt, lädt Claude Code die Manifest-Pfade und nicht den Ordner. `claude plugin list` und die `/plugin`-Schnittstelle zeigen dann die Warnung `Default <folder>/ folder is ignored because the manifest sets "<key>"`.

Um die Warnung zu vermeiden, setzen Sie den Schlüssel auf einen Pfad in diesem Ordner: `"commands": ["./commands/deploy.md"]` benennt eine Datei im Standardordner und erzeugt keine Warnung.

<h2 id="user-configuration">
  Benutzerkonfiguration
</h2>

`userConfig` deklariert Werte, die Claude Code den Benutzer auffordert einzugeben, wenn das Plugin aktiviert ist, damit Benutzer `settings.json` nicht selbst bearbeiten.

Schlüssel sind Identifier aus Buchstaben, Ziffern und Unterstrichen und können nicht mit einer Ziffer beginnen.

Jeder Wert ist ein striktes Objekt mit diesen Feldern. Ein unbekannter Schlüssel schlägt die Validierung fehl.

| Feld          | Erforderlich | Beschreibung                                                                                                                                                                                                  |
| :------------ | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `type`        | Ja           | Einer von `string`, `number`, `boolean`, `directory` oder `file`                                                                                                                                              |
| `title`       | Ja           | Label, das im Konfigurationsdialog angezeigt wird                                                                                                                                                             |
| `description` | Ja           | Hilfetext, der unter dem Feld angezeigt wird                                                                                                                                                                  |
| `required`    | Nein         | Wenn `true`, akzeptiert der Konfigurationsdialog keinen leeren Wert                                                                                                                                           |
| `default`     | Nein         | Wert, der verwendet wird, wenn der Benutzer nichts bereitstellt: ein String, eine Zahl, ein Boolean oder ein Array von Strings                                                                                |
| `options`     | Nein         | Für `string`, die Werte, die das Feld akzeptiert, angezeigt als Picker in `/config`. Siehe [Feld auf feste Optionen beschränken](#limit-a-field-to-fixed-options). Erfordert Claude Code v2.1.271 oder später |
| `multiple`    | Nein         | Für `string`, erlaubt ein Array von Strings                                                                                                                                                                   |
| `sensitive`   | Nein         | Wenn `true`, maskiert die Eingabe und speichert den Wert in sicherer Speicherung anstelle von `settings.json`                                                                                                 |
| `min` / `max` | Nein         | Grenzen für `number`                                                                                                                                                                                          |

Jede Option jedes aktivierten Plugins erscheint auch als Zeile im `/config`-Panel, außer `sensitive`-Optionen und `multiple`-Listen. Die `/config`-Zeilen erfordern Claude Code v2.1.269 oder später.

Diese `userConfig` deklariert einen Endpunkt und ein maskiertes Token:

```json theme={null}
{
  "userConfig": {
    "api_endpoint": {
      "type": "string",
      "title": "API endpoint",
      "description": "Your team's API endpoint"
    },
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "API authentication token",
      "sensitive": true
    }
  }
}
```

<h3 id="limit-a-field-to-fixed-options">
  Feld auf feste Optionen beschränken
</h3>

Setzen Sie `options` auf ein `userConfig`-Feld, um Benutzer seinen Wert aus einer festen Liste auszuwählen.

Um ein `tone`-Feld auf drei Optionen zu beschränken, listen Sie sie in `options` auf und setzen Sie `default` auf eine davon:

```json theme={null}
{
  "userConfig": {
    "tone": {
      "type": "string",
      "title": "Tone",
      "description": "Voice for generated replies",
      "options": ["neutral", "warm", "formal"],
      "default": "neutral"
    }
  }
}
```

Wenn Sie `options` auf einem beliebigen Feld deklarieren, können Benutzer auf Claude Code-Versionen vor v2.1.271 das Plugin nicht laden.

`options` gilt für ein `string`-Feld, das nicht `multiple` oder `sensitive` ist. Setzen Sie `default` auf einen der aufgelisteten Werte, oder setzen Sie `required: true`, damit der Benutzer einen auswählen muss. Jede Option ist ein einfaches Label von 1 bis 64 Zeichen, und `claude plugin validate`, das Sie in Ihrer Shell ausführen, meldet alles andere, das es ablehnt. Ein Plugin, dessen `options` diese Regeln brechen, wird nicht geladen.

<h3 id="where-values-are-stored">
  Wo Werte gespeichert werden
</h3>

Nicht-sensitive Werte werden unter [`pluginConfigs`](/docs/de/settings-reference#pluginconfigs) in der `settings.json` des Benutzers gespeichert. Sensitive Werte gehen stattdessen in den sicheren Credential-Store der Plattform. Die [Einstellungsseite](/docs/de/settings-reference#pluginconfigs) listet auf, welche Einstellungsdateien `pluginConfigs` gelesen werden.

<h3 id="reference-a-saved-value">
  Einen gespeicherten Wert referenzieren
</h3>

Referenzieren Sie einen gespeicherten Wert, wo das Plugin ihn benötigt, in einer von zwei Formen:

* **`${user_config.KEY}`**: ersetzt in MCP-Server-Konfiguration, LSP-Server-Konfiguration, [Exec-Form](/docs/de/hooks#exec-form-and-shell-form)-Hook-`args` und Skill- und Agent-Inhalt. In Skill- und Agent-Inhalt werden nur nicht-sensitive Werte ersetzt, und ein sensitive Wert dort wird zu einem Platzhalter
* **`CLAUDE_PLUGIN_OPTION_<KEY>`**: exportiert zu Hook-Prozessen für jede Option, mit `<KEY>` in Großbuchstaben. Ein Shell-Form-Hook liest `$CLAUDE_PLUGIN_OPTION_API_TOKEN` für `api_token`

<h3 id="fields-that-run-through-a-shell">
  Felder, die durch eine Shell laufen
</h3>

Shell-Form-Hook-Befehle, Monitor-Befehle und MCP-[`headersHelper`](/docs/de/mcp#use-dynamic-headers-for-custom-authentication) lehnen `${user_config.*}` ab. Eine Komponente, die darauf in einem dieser Felder verweist, schlägt mit einem [Fehler](/docs/de/errors#plugin-command-references-user-config) fehl, anstatt zu laufen, weil der Feldwert an eine Shell übergeben wird, die den ersetzten Wert neu analysieren würde.

Die Tabelle zeigt, wie der Wert stattdessen jedes dieser Felder erreichen kann.

| Feld                    | Wie der Wert es erreichen kann                                                                                                                                                                                         |
| :---------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Shell-Form-Hook-Befehle | Verwenden Sie [Exec-Form](/docs/de/hooks#exec-form-and-shell-form) mit `args`, oder lesen Sie `CLAUDE_PLUGIN_OPTION_<KEY>` aus der Hook-Umgebung                                                                            |
| Monitor-Befehle         | Nicht durch Claude Code. Monitor-Prozesse erhalten `CLAUDE_PLUGIN_OPTION_<KEY>` nicht, daher muss das Monitor-Skript den Wert selbst abrufen                                                                           |
| MCP-`headersHelper`     | Nicht durch Claude Code. Die Helper-Umgebung trägt `CLAUDE_PLUGIN_ROOT`, `CLAUDE_CODE_MCP_SERVER_NAME` und `CLAUDE_CODE_MCP_SERVER_URL`, aber keine Optionswerte, daher muss das Helper-Skript den Wert selbst abrufen |

<h2 id="channels">
  Kanäle
</h2>

`channels` deklariert die Nachrichtenkanäle, die ein Plugin bereitstellt, z. B. eine Brücke zu einer Chat-App. Wenn Sie einen deklarieren, kann Claude Code auffordern, die Kanalkonfiguration zu konfigurieren, wenn das Plugin aktiviert ist. Für wie der Server Nachrichten injiziert, siehe die [Kanäle-Referenz](/docs/de/channels-reference#package-as-a-plugin).

Jeder Eintrag ist ein striktes Objekt, das an einen der MCP-Server des Plugins gebunden ist, mit diesen Feldern:

| Feld          | Erforderlich | Beschreibung                                                                                                                                                                     |
| :------------ | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `server`      | Ja           | Schlüssel des MCP-Servers in diesem Plugin's `mcpServers`, an den der Kanal gebunden ist                                                                                         |
| `displayName` | Nein         | Name, der im Konfigurationsdialog-Titel angezeigt wird. Standard ist der Server-Name                                                                                             |
| `userConfig`  | Nein         | Optionen zum Auffordern, in der gleichen Form wie [Top-Level-`userConfig`](#user-configuration). Gespeicherte Werte ersetzen `${user_config.KEY}`-Referenzen in der Server-`env` |

Dieses Manifest bindet einen Kanal an den `telegram`-MCP-Server des Plugins und fordert ein Bot-Token auf, das sich in die Server-`env` ersetzt:

```json theme={null}
{
  "mcpServers": {
    "telegram": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"],
      "env": { "BOT_TOKEN": "${user_config.bot_token}" }
    }
  },
  "channels": [
    {
      "server": "telegram",
      "displayName": "Telegram",
      "userConfig": {
        "bot_token": {
          "type": "string",
          "title": "Bot token",
          "description": "Telegram bot token",
          "sensitive": true
        }
      }
    }
  ]
}
```

<h2 id="environment-variables">
  Umgebungsvariablen
</h2>

Claude Code stellt drei Pfadvariablen für Plugin-Komponenten bereit. Referenzieren Sie sie als `${NAME}` in den Feldern, die unter [Wo jede Variable sich auflöst](#where-each-variable-resolves) aufgelistet sind, und lesen Sie sie als Umgebungsvariablen in den Prozessen, die sie erhalten.

| Variable                | Löst sich auf zu                                                                                                                                                                                                                 | Verwenden Sie sie für                                                              |
| :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------- |
| `${CLAUDE_PLUGIN_ROOT}` | Absoluter Pfad der installierten Version des Plugins                                                                                                                                                                             | Skripte, Binärdateien und Konfigurationsdateien, die mit dem Plugin gebündelt sind |
| `${CLAUDE_PLUGIN_DATA}` | `~/.claude/plugins/data/<id>/`, erstellt bei erster Referenz und über Plugin-Updates hinweg beibehalten. `<id>` ist der Plugin-Identifier mit jedem Zeichen außer einem Buchstaben, einer Ziffer, `_` oder `-` ersetzt durch `-` | Installierte Abhängigkeiten wie `node_modules`, generierter Code und Caches        |
| `${CLAUDE_PROJECT_DIR}` | Das Projekt-Root                                                                                                                                                                                                                 | Projekt-lokale Skripte und Konfigurationsdateien                                   |

`${CLAUDE_PLUGIN_ROOT}` ändert sich, wenn das Plugin aktualisiert wird, daher schreiben Sie keinen Zustand dort. Für wo sich der Root bewegt und wann das alte Verzeichnis bereinigt wird, siehe die [Ladenseite](/docs/de/plugins/loading).

Wenn Sie das Plugin von der letzten Stelle, an der es installiert ist, deinstallieren, wird das `${CLAUDE_PLUGIN_DATA}`-Verzeichnis gelöscht, es sei denn, Sie übergeben [`--keep-data`](/docs/de/plugins/cli-reference).

<h3 id="where-each-variable-resolves">
  Wo jede Variable sich auflöst
</h3>

In jeder Plugin-Komponente lösen sich `${...}`-Referenzen inline in spezifischen Feldern auf, und einige Komponenten erhalten die Variablen auch in ihrer Prozessumgebung:

| Plugin-Komponente                 | Felder, wo `${...}` sich auflöst            | Exportiert zum Prozess                                                                            |
| :-------------------------------- | :------------------------------------------ | :------------------------------------------------------------------------------------------------ |
| Hook-Befehle                      | Überall in `command` und `args`             | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PROJECT_DIR` und `CLAUDE_PLUGIN_OPTION_<KEY>` |
| Monitor-Befehle                   | Überall in `command`                        | Nicht exportiert                                                                                  |
| MCP-`stdio`-Server                | `command`, `args`, `env`                    | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`                                                        |
| MCP-`http`-, `sse`-, `ws`-Server  | `url`, `headers`, `headersHelper`           | Nicht anwendbar                                                                                   |
| LSP-Server                        | `command`, `args`, `env`, `workspaceFolder` | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PROJECT_DIR`                                  |
| Skill-, Befehls- und Agent-Inhalt | Überall im Markdown-Text                    | Nicht anwendbar                                                                                   |

Die Variablen sind nicht in der Umgebung von Befehlen vorhanden, die Claude durch das Bash-Tool ausführt, in der Hauptsitzung oder in einem Subagent. In Skill-, Befehls- und Agent-Inhalt schreiben Sie die `${...}`-Referenz stattdessen im Markdown-Text, und Claude Code ersetzt den Pfad inline, wenn es den Inhalt lädt.

<h3 id="quoting-and-path-separators">
  Anführungszeichen und Pfadtrennzeichen
</h3>

Halten Sie jeden ersetzten Pfad ein einzelnes Argument:

* **Hook-Befehle**: Verwenden Sie [Exec-Form](/docs/de/hooks#exec-form-and-shell-form) mit `args`, damit jeder Pfad ein Argument ohne Anführungszeichen ist
* **Shell-Form-Hooks und Monitor-Befehle**: Wickeln Sie die Variable in doppelte Anführungszeichen ein, damit ein Pfad mit Leerzeichen ein Wort bleibt

Dieser Shell-Form-Hook führt ein Skript aus, das mit dem Plugin gebündelt ist:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/process.sh"
          }
        ]
      }
    ]
  }
}
```

Auf Windows verwenden die ersetzten Pfade Schrägstriche, damit eine Shell Backslashes nicht als Escapes liest.

<h2 id="standard-layout">
  Standardlayout
</h2>

Jeder Komponententyp hat einen Standardort unter dem Plugin-Root, der verwendet wird, wenn das Manifest nicht anderswo verweist.

| Komponente          | Standardort                  | Inhalt                                                                                                                                                                                                                                                                                                                                                               |
| :------------------ | :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Manifest            | `.claude-plugin/plugin.json` | Plugin-Metadaten und Konfiguration. Optional                                                                                                                                                                                                                                                                                                                         |
| Skills              | `skills/`                    | Ein `<name>/SKILL.md` pro Skill. Ein Plugin mit `SKILL.md` im Root, kein `skills/` und kein `skills`-Schlüssel wird als einzelner Skill geladen                                                                                                                                                                                                                      |
| Befehle             | `commands/`                  | Flache Markdown-Befehlsdateien. Bevorzugen Sie `skills/` für neue Plugins                                                                                                                                                                                                                                                                                            |
| Agenten             | `agents/`                    | Agent-Markdown-Dateien. Unterordner sind Teil des [Agent-Namens](/docs/de/plugins/components#agents)                                                                                                                                                                                                                                                                      |
| Hooks               | `hooks/hooks.json`           | Hook-Konfiguration                                                                                                                                                                                                                                                                                                                                                   |
| MCP-Server          | `.mcp.json`                  | MCP-Server-Definitionen                                                                                                                                                                                                                                                                                                                                              |
| LSP-Server          | `.lsp.json`                  | LSP-Server-Konfigurationen                                                                                                                                                                                                                                                                                                                                           |
| Ausgabestile        | `output-styles/`             | Ausgabestil-Markdown-Dateien                                                                                                                                                                                                                                                                                                                                         |
| Workflows           | `workflows/`                 | Workflow-`.js`-Dateien                                                                                                                                                                                                                                                                                                                                               |
| Themes              | `themes/`                    | Theme-JSON-Dateien                                                                                                                                                                                                                                                                                                                                                   |
| Monitore            | `monitors/monitors.json`     | Das Monitors-Array                                                                                                                                                                                                                                                                                                                                                   |
| Ausführbare Dateien | `bin/`                       | Dateien hier sind auf dem Bash-Tool's `PATH`, während das Plugin aktiviert ist, daher führt Claude sie als bloße Befehle aus. claude.ai und Cowork installieren kein Plugin, das dieses Verzeichnis hat, einschließlich eines, das Sie [durch claude.ai-Organisationseinstellungen verteilen](/docs/de/plugins/host-marketplace#distribute-through-organization-settings) |
| Einstellungen       | `settings.json`              | `agent`- und `subagentStatusLine`-Standards, die angewendet werden, während das Plugin aktiviert ist                                                                                                                                                                                                                                                                 |

Ein Plugin, das jeden Standardort verwendet, plus einen `scripts/`-Ordner, den seine Hooks aufrufen, ist wie folgt angeordnet:

```text theme={null}
deploy-tools/
├── .claude-plugin/
│   └── plugin.json
├── skills/
│   └── deploy/
│       └── SKILL.md
├── commands/
│   └── status.md
├── agents/
│   └── reviewer.md
├── hooks/
│   └── hooks.json
├── monitors/
│   └── monitors.json
├── output-styles/
│   └── terse.md
├── themes/
│   └── dracula.json
├── workflows/
│   └── release-audit.js
├── bin/
│   └── deploy-tool
├── scripts/
│   └── format.sh
├── settings.json
├── .mcp.json
└── .lsp.json
```

Um durch dieses Layout zu klicken und zu lesen, was jede Datei tut, öffnen Sie den [Plugin-Explorer](/docs/de/plugins/components#explore-the-plugin-directory).

Eine `CLAUDE.md` im Plugin-Root wird nicht als Kontext geladen, und `claude plugin validate` warnt, wenn es eine findet. Um Anweisungen einzuschließen, die in Claude's Kontext geladen werden, legen Sie sie in einen Skill.

<h2 id="marketplace-entries-and-the-manifest">
  Marketplace-Einträge und das Manifest
</h2>

Ein [Marketplace-Eintrag](/docs/de/plugins/marketplace-reference) akzeptiert jedes Feld auf dieser Seite neben [seinen eigenen Feldern](/docs/de/plugins/marketplace-reference#plugin-entries), einschließlich `strict`.

Das `strict`-Feld entscheidet, ob der Eintrag Komponenten zu einem Plugin hinzufügen darf, das sein eigenes `plugin.json` hat. Es ist standardmäßig `true`.

<h3 id="how-entry-fields-combine-with-plugin-json">
  Wie Eintragsfelder mit `plugin.json` kombiniert werden
</h3>

Der Eintrag dient entweder als Manifest, fügt Komponenten hinzu oder steht in Konflikt damit:

* **Kein `plugin.json`**: Der Eintrag ist das Manifest, unabhängig von `strict`. Eintrag `hooks` wird nur in der inline Objektform geladen. Für einen Dateipfad oder ein Array dort zeigt die `/plugin`-Registerkarte **Errors** einen `not yet supported in a marketplace entry`-Fehler
* **`plugin.json` vorhanden, `strict` nicht gesetzt oder `true`**: Claude Code lädt das Manifest und hängt die `commands`, `agents`, `skills`, `outputStyles` und `themes` des Eintrags daran an. Für `hooks` ersetzen die Matcher des Eintrags für ein Ereignis die des Manifests für das gleiche Ereignis, und Ereignisse, die nur das Manifest deklariert, behalten ihre
* **`plugin.json` vorhanden, `strict: false`**: Ein Eintrag, der `commands`, `agents`, `skills`, `hooks`, `outputStyles` oder `themes` deklariert, ist ein Konflikt, und das Plugin wird nicht geladen mit `Plugin <name> has conflicting manifests`

Wenn ein [Marketplace-Eintrag, dessen `source` das Marketplace-Root ist](/docs/de/plugins/marketplace-reference), spezifische `skills`-Unterverzeichnisse auflistet, werden nur diese Unterverzeichnisse geladen, und das Standard-`skills/`-Verzeichnis des Plugins wird nicht gescannt. Ein `skills`-Schlüssel im Manifest [fügt stattdessen zum Standard hinzu](#how-each-key-combines-with-its-default-location).

<h3 id="metadata-precedence">
  Metadaten-Vorrang
</h3>

Einige Metadatenfelder haben einen festen Vorrang unabhängig von `strict`:

* **`defaultEnabled` und Anzeigefelder**: Die `defaultEnabled` des Eintrags und seine [Anzeigefelder](/docs/de/plugins/marketplace-reference#entry-and-plugin-json) wie `displayName` überschreiben die des Manifests
* **`version`**: Die `version` des Manifests überschreibt die des Eintrags
* **`name`**: Wenn der Eintrag das Plugin unter einem anderen `name` als das Manifest auflistet, verwendet `enabledPlugins` den Eintragnamen, und Komponenten werden unter dem Manifest-Namen namespaced

Für die vollständige Vorrangtabelle siehe [Strict-Modus](/docs/de/plugins/marketplace-reference).

<h2 id="next-steps">
  Nächste Schritte
</h2>

* [Komponenten zu einem Plugin hinzufügen](/docs/de/plugins/components): Was jede Komponente zur Laufzeit tut, mit einem Beispiel, das validiert
* [Marketplace-Referenz](/docs/de/plugins/marketplace-reference): Die Eintragsfelder, die ein Marketplace für Ihr Plugin setzen kann
* [Plugin-Befehls-Referenz](/docs/de/plugins/cli-reference#plugin-validate): `claude plugin validate`-Flags und Ausgabe
* [Plugins fehlerbeheben](/docs/de/plugins/troubleshooting#claude-plugin-validate-reports-errors): Jede Validierungsnachricht mit ihrer Lösung
