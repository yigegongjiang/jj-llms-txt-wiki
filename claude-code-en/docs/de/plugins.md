> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins erstellen

> Erstellen Sie benutzerdefinierte Plugins, um Claude Code mit Skills, Agents, Hooks und MCP-Servern zu erweitern.

Plugins ermöglichen es Ihnen, Claude Code mit benutzerdefinierten Funktionen zu erweitern, die projektübergreifend und teamübergreifend freigegeben werden können. Diese Anleitung behandelt die Erstellung eigener Plugins mit Skills, Agents, Hooks und MCP-Servern.

Möchten Sie vorhandene Plugins installieren? Siehe [Plugins entdecken und installieren](/docs/de/discover-plugins). Für vollständige technische Spezifikationen siehe [Plugins-Referenz](/docs/de/plugins-reference).

<h2 id="when-to-use-plugins-vs-standalone-configuration">
  Wann Plugins vs. eigenständige Konfiguration verwenden
</h2>

Claude Code unterstützt zwei Möglichkeiten, um benutzerdefinierte Skills, Agents und Hooks hinzuzufügen:

| Ansatz                                                                                                               | Skill-Namen          | Am besten für                                                                                                        |
| :------------------------------------------------------------------------------------------------------------------- | :------------------- | :------------------------------------------------------------------------------------------------------------------- |
| **Eigenständig** (`.claude/`-Verzeichnis)                                                                            | `/hello`             | Persönliche Workflows, projektspezifische Anpassungen, schnelle Experimente                                          |
| **Plugins** (eigenständige Verzeichnisse mit Skills, Agents, Hooks oder einem `.claude-plugin/plugin.json`-Manifest) | `/plugin-name:hello` | Freigabe für Teamkollegen, Verteilung an die Community, versionierte Releases, wiederverwendbar über Projekte hinweg |

**Verwenden Sie eigenständige Konfiguration, wenn**:

* Sie Claude Code für ein einzelnes Projekt anpassen
* Die Konfiguration persönlich ist und nicht freigegeben werden muss
* Sie mit Skills oder Hooks experimentieren, bevor Sie diese verpacken
* Sie kurze Skill-Namen wie `/hello` oder `/deploy` möchten

**Verwenden Sie Plugins, wenn**:

* Sie Funktionen mit Ihrem Team oder der Community teilen möchten
* Sie die gleichen Skills/Agents über mehrere Projekte hinweg benötigen
* Sie Versionskontrolle und einfache Updates für Ihre Erweiterungen möchten
* Sie über einen Marketplace verteilen
* Sie mit Namespace-Skills wie `/my-plugin:hello` einverstanden sind (Namespacing verhindert Konflikte zwischen Plugins)

<Tip>
  Beginnen Sie mit eigenständiger Konfiguration in `.claude/` für schnelle Iteration, dann [konvertieren Sie zu einem Plugin](#convert-existing-configurations-to-plugins), wenn Sie bereit sind zu teilen.
</Tip>

<h2 id="quickstart">
  Schnellstart
</h2>

Dieser Schnellstart führt Sie durch die Erstellung eines Plugins mit einem benutzerdefinierten Skill. Sie erstellen ein Manifest (die Konfigurationsdatei, die Ihr Plugin definiert), fügen einen Skill hinzu und testen ihn lokal mit dem Flag `--plugin-dir`.

<h3 id="prerequisites">
  Voraussetzungen
</h3>

* Claude Code [installiert und authentifiziert](/docs/de/quickstart#step-1-install-claude-code)

<Note>
  Wenn Sie den Befehl `/plugin` nicht sehen, aktualisieren Sie Claude Code auf die neueste Version. Siehe [Troubleshooting](/docs/de/troubleshooting) für Upgrade-Anweisungen.
</Note>

<h3 id="create-your-first-plugin">
  Erstellen Sie Ihr erstes Plugin
</h3>

<Steps>
  <Step title="Erstellen Sie das Plugin-Verzeichnis">
    Jedes Plugin befindet sich in seinem eigenen Verzeichnis, das Ihre Skills, Agents oder Hooks enthält, optional zusammen mit einem `.claude-plugin/plugin.json`-Manifest. Der Speicherort spielt für diesen Schnellstart keine Rolle, da Sie Claude Code im Testschritt mit `--plugin-dir` auf das Verzeichnis verweisen. Erstellen Sie es überall, wo es praktisch ist, z. B. in einem Scratch-Ordner oder einem Projektverzeichnis:

    ```bash theme={null}
    mkdir my-first-plugin
    ```

    Die verbleibenden Schritte werden vom übergeordneten Verzeichnis aus ausgeführt und referenzieren Pfade wie `my-first-plugin/...` relativ dazu.
  </Step>

  <Step title="Erstellen Sie das Plugin-Manifest">
    Die Manifestdatei unter `.claude-plugin/plugin.json` definiert die Identität Ihres Plugins: seinen Namen, die Beschreibung und die Version. Claude Code verwendet diese Metadaten, um Ihr Plugin im Plugin-Manager anzuzeigen.

    Erstellen Sie das `.claude-plugin`-Verzeichnis in Ihrem Plugin-Ordner:

    ```bash theme={null}
    mkdir my-first-plugin/.claude-plugin
    ```

    Erstellen Sie dann `my-first-plugin/.claude-plugin/plugin.json` mit diesem Inhalt:

    ```json my-first-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-first-plugin",
      "description": "A greeting plugin to learn the basics",
      "version": "1.0.0",
      "author": {
        "name": "Your Name"
      }
    }
    ```

    | Feld          | Zweck                                                                                                                                                                                                                                                                                       |
    | :------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
    | `name`        | Eindeutige Kennung und Skill-Namespace. Skills werden mit diesem Präfix versehen (z. B. `/my-first-plugin:hello`).                                                                                                                                                                          |
    | `description` | Wird im Plugin-Manager angezeigt, wenn Sie Plugins durchsuchen oder installieren.                                                                                                                                                                                                           |
    | `version`     | Optional. Falls gesetzt, erhalten Benutzer nur Updates, wenn Sie dieses Feld erhöhen. Falls weggelassen und Ihr Plugin wird über Git verteilt, wird der Commit-SHA verwendet und jeder Commit zählt als neue Version. Siehe [Versionsverwaltung](/docs/de/plugins-reference#version-management). |
    | `author`      | Optional. Hilfreich für die Zuordnung.                                                                                                                                                                                                                                                      |

    Für zusätzliche Felder wie `homepage`, `repository` und `license` siehe das [vollständige Manifest-Schema](/docs/de/plugins-reference#plugin-manifest-schema).
  </Step>

  <Step title="Fügen Sie einen Skill hinzu">
    Skills befinden sich im Verzeichnis `skills/`. Jeder Skill ist ein Ordner, der eine Datei `SKILL.md` enthält. Der Ordnername wird zum Skill-Namen, mit dem Präfix des Plugin-Namespace (`hello/` in einem Plugin namens `my-first-plugin` erstellt `/my-first-plugin:hello`).

    Erstellen Sie ein Skill-Verzeichnis in Ihrem Plugin-Ordner:

    ```bash theme={null}
    mkdir -p my-first-plugin/skills/hello
    ```

    Erstellen Sie dann `my-first-plugin/skills/hello/SKILL.md` mit diesem Inhalt:

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    description: Greet the user with a friendly message
    disable-model-invocation: true
    ---

    Greet the user warmly and ask how you can help them today.
    ```
  </Step>

  <Step title="Testen Sie Ihr Plugin">
    Führen Sie Claude Code mit dem Flag `--plugin-dir` aus, um Ihr Plugin zu laden:

    ```bash theme={null}
    claude --plugin-dir ./my-first-plugin
    ```

    Sobald Claude Code startet, versuchen Sie Ihren neuen Skill:

    ```shell theme={null}
    /my-first-plugin:hello
    ```

    Sie sehen Claude mit einer Begrüßung antworten. Führen Sie `/help` aus, um Ihren Skill unter dem Plugin-Namespace aufgelistet zu sehen.

    <Note>
      **Warum Namespacing?** Plugin-Skills sind immer mit Namespace versehen (wie `/my-first-plugin:hello`), um Konflikte zu vermeiden, wenn mehrere Plugins Skills mit demselben Namen haben.

      Um das Namespace-Präfix zu ändern, aktualisieren Sie das Feld `name` in `plugin.json`.
    </Note>
  </Step>

  <Step title="Fügen Sie Skill-Argumente hinzu">
    Machen Sie Ihren Skill dynamisch, indem Sie Benutzereingaben akzeptieren. Der Platzhalter `$ARGUMENTS` erfasst jeden Text, den der Benutzer nach dem Skill-Namen bereitstellt.

    Aktualisieren Sie Ihre Datei `SKILL.md`:

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    description: Greet the user with a personalized message
    ---

    # Hello Skill

    Greet the user named "$ARGUMENTS" warmly and ask how you can help them today. Make the greeting personal and encouraging.
    ```

    Führen Sie `/reload-plugins` aus, um die Änderungen zu übernehmen, und versuchen Sie dann den Skill mit Ihrem Namen:

    ```shell theme={null}
    /my-first-plugin:hello Alex
    ```

    Claude wird Sie beim Namen begrüßen. Weitere Informationen zum Übergeben von Argumenten an Skills finden Sie unter [Skills](/docs/de/skills#pass-arguments-to-skills).
  </Step>
</Steps>

Sie haben erfolgreich ein Plugin mit diesen Schlüsselkomponenten erstellt und getestet:

* **Plugin-Manifest** (`.claude-plugin/plugin.json`): beschreibt die Metadaten Ihres Plugins
* **Skills-Verzeichnis** (`skills/`): enthält Ihre benutzerdefinierten Skills
* **Skill-Argumente** (`$ARGUMENTS`): erfasst Benutzereingaben für dynamisches Verhalten

<Tip>
  Das Flag `--plugin-dir` ist nützlich für Entwicklung und Tests. Wenn Sie bereit sind, Ihr Plugin mit anderen zu teilen, siehe [Erstellen und verteilen Sie einen Plugin-Marketplace](/docs/de/plugin-marketplaces).
</Tip>

<h2 id="develop-a-plugin-in-your-skills-directory">
  Entwickeln Sie ein Plugin in Ihrem Skills-Verzeichnis
</h2>

Anstatt `--plugin-dir` bei jedem Start zu übergeben, können Sie ein Plugin in Ihrem Skills-Verzeichnis behalten und Claude Code es automatisch laden lassen. `claude plugin init` erstellt ein Gerüst dafür:

```bash theme={null}
claude plugin init my-tool
```

Dies erstellt `~/.claude/skills/my-tool/` mit einem `.claude-plugin/plugin.json`-Manifest und einer Starter-`SKILL.md`. In der nächsten Sitzung wird es als `my-tool@skills-dir` geladen, ohne dass ein Marketplace oder Installationsschritt erforderlich ist.

Für die Auto-Load-Regeln, persönlicher vs. Projekt-Umfang, die Workspace-Trust-Anforderung und wie man eine aktualisiert oder entfernt, siehe [Skills-Verzeichnis-Plugins](/docs/de/plugins-reference#skills-directory-plugins).

<h2 id="plugin-structure-overview">
  Übersicht über die Plugin-Struktur
</h2>

Sie haben ein Plugin mit einem Skill erstellt, aber Plugins können viel mehr enthalten: benutzerdefinierte Agents, Hooks, MCP-Server, LSP-Server und Hintergrund-Monitore.

<Warning>
  **Häufiger Fehler**: Platzieren Sie `commands/`, `agents/`, `skills/` oder `hooks/` nicht im Verzeichnis `.claude-plugin/`. Nur `plugin.json` gehört in `.claude-plugin/`. Alle anderen Verzeichnisse müssen auf der Plugin-Root-Ebene sein.

  Das Plugin-Root ist das eigene Verzeichnis des einzelnen Plugins: dasjenige, das `.claude-plugin/plugin.json` enthält. Es ist niemals `~/.claude/`. Zum Beispiel liest Claude Code eine `.mcp.json`, die unter `~/.claude/.mcp.json` platziert ist, nicht.
</Warning>

| Verzeichnis       | Speicherort | Zweck                                                                                                   |
| :---------------- | :---------- | :------------------------------------------------------------------------------------------------------ |
| `.claude-plugin/` | Plugin-Root | Enthält `plugin.json`-Manifest (optional, wenn Komponenten Standardspeicherorte verwenden)              |
| `skills/`         | Plugin-Root | Skills als `<name>/SKILL.md`-Verzeichnisse                                                              |
| `commands/`       | Plugin-Root | Skills als flache Markdown-Dateien. Verwenden Sie `skills/` für neue Plugins                            |
| `agents/`         | Plugin-Root | Benutzerdefinierte Agent-Definitionen                                                                   |
| `hooks/`          | Plugin-Root | Event-Handler in `hooks.json`                                                                           |
| `.mcp.json`       | Plugin-Root | MCP-Server-Konfigurationen                                                                              |
| `.lsp.json`       | Plugin-Root | LSP-Server-Konfigurationen für Code-Intelligenz                                                         |
| `monitors/`       | Plugin-Root | Hintergrund-Monitor-Konfigurationen in `monitors.json`                                                  |
| `bin/`            | Plugin-Root | Ausführbare Dateien, die zum `PATH` des Bash-Tools hinzugefügt werden, während das Plugin aktiviert ist |
| `settings.json`   | Plugin-Root | Standard-[Einstellungen](/docs/de/settings), die angewendet werden, wenn das Plugin aktiviert ist            |

Ein Plugin, das genau einen Skill enthält, kann `SKILL.md` direkt im Plugin-Root platzieren, anstatt ein `skills/`-Verzeichnis zu erstellen. Claude Code lädt es als einen einzelnen Skill und verwendet das Frontmatter-Feld `name` für den Aufrufen-Namen. Verwenden Sie das `skills/`-Layout für Plugins, die möglicherweise auf mehr als einen Skill anwachsen.

<Note>
  **Nächste Schritte**: Bereit, weitere Funktionen hinzuzufügen? Springen Sie zu [Entwickeln Sie komplexere Plugins](#develop-more-complex-plugins), um Agents, Hooks, MCP-Server und LSP-Server hinzuzufügen. Für vollständige technische Spezifikationen aller Plugin-Komponenten siehe [Plugins-Referenz](/docs/de/plugins-reference).
</Note>

<h2 id="develop-more-complex-plugins">
  Entwickeln Sie komplexere Plugins
</h2>

Sobald Sie sich mit grundlegenden Plugins vertraut gemacht haben, können Sie anspruchsvollere Erweiterungen erstellen.

<h3 id="add-skills-to-your-plugin">
  Fügen Sie Skills zu Ihrem Plugin hinzu
</h3>

Plugins können [Agent-Skills](/docs/de/skills) enthalten, um die Fähigkeiten von Claude zu erweitern. Skills werden vom Modell aufgerufen: Claude verwendet sie automatisch basierend auf dem Task-Kontext.

Fügen Sie ein Verzeichnis `skills/` auf Ihrer Plugin-Root mit Skill-Ordnern hinzu, die `SKILL.md`-Dateien enthalten:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json
└── skills/
    └── code-review/
        └── SKILL.md
```

Jede `SKILL.md` enthält YAML-Frontmatter und Anweisungen. Fügen Sie eine `description` ein, damit Claude weiß, wann der Skill verwendet werden soll:

```yaml theme={null}
---
description: Reviews code for best practices and potential issues. Use when reviewing code, checking PRs, or analyzing code quality.
---

When reviewing code, check for:
1. Code organization and structure
2. Error handling
3. Security concerns
4. Test coverage
```

Nach der Installation des Plugins führen Sie `/reload-plugins` aus, um die Skills zu laden. Für vollständige Anleitung zur Skill-Erstellung, einschließlich progressiver Offenlegung und Tool-Einschränkungen, siehe [Agent-Skills](/docs/de/skills).

<h3 id="add-lsp-servers-to-your-plugin">
  Fügen Sie LSP-Server zu Ihrem Plugin hinzu
</h3>

<Tip>
  Für gängige Sprachen wie TypeScript, Python und Rust installieren Sie die vorgefertigten LSP-Plugins aus dem offiziellen Marketplace. Erstellen Sie benutzerdefinierte LSP-Plugins nur, wenn Sie Unterstützung für Sprachen benötigen, die noch nicht abgedeckt sind.
</Tip>

LSP-Plugins (Language Server Protocol) geben Claude Echtzeit-Code-Intelligenz. Wenn Sie eine Sprache unterstützen müssen, die kein offizielles LSP-Plugin hat, können Sie ein eigenes erstellen, indem Sie eine `.lsp.json`-Datei zu Ihrem Plugin hinzufügen:

```json .lsp.json theme={null}
{
  "go": {
    "command": "gopls",
    "args": ["serve"],
    "extensionToLanguage": {
      ".go": "go"
    }
  }
}
```

Benutzer, die Ihr Plugin installieren, müssen die Language-Server-Binärdatei auf ihrem Computer installiert haben.

Für vollständige LSP-Konfigurationsoptionen siehe [LSP-Server](/docs/de/plugins-reference#lsp-servers).

<h3 id="add-background-monitors-to-your-plugin">
  Fügen Sie Hintergrund-Monitore zu Ihrem Plugin hinzu
</h3>

Hintergrund-Monitore ermöglichen es Ihrem Plugin, Protokolle, Dateien oder externen Status im Hintergrund zu überwachen und Claude zu benachrichtigen, wenn Ereignisse eintreffen. Claude Code startet jeden Monitor automatisch, wenn das Plugin aktiv ist, sodass Sie Claude nicht anweisen müssen, die Überwachung zu starten.

Fügen Sie eine Datei `monitors/monitors.json` auf der Plugin-Root mit einem Array von Monitor-Einträgen hinzu:

```json monitors/monitors.json theme={null}
[
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log"
  }
]
```

Jede stdout-Zeile aus `command` wird Claude während der Sitzung als Benachrichtigung zugestellt. Für das vollständige Schema, einschließlich des `when`-Triggers und der Variablenersetzung, siehe [Monitore](/docs/de/plugins-reference#monitors).

<h3 id="ship-default-settings-with-your-plugin">
  Versenden Sie Standard-Einstellungen mit Ihrem Plugin
</h3>

Plugins können eine Datei `settings.json` auf der Plugin-Root enthalten, um Standard-Konfiguration anzuwenden, wenn das Plugin aktiviert ist. Derzeit werden nur die Schlüssel `agent` und `subagentStatusLine` unterstützt.

Das Setzen von `agent` aktiviert einen der [benutzerdefinierten Agents](/docs/de/sub-agents) des Plugins als Haupt-Thread und wendet seinen System-Prompt, Tool-Einschränkungen und Modell an. Dies ermöglicht es einem Plugin, das Standardverhalten von Claude Code zu ändern, wenn es aktiviert ist.

```json settings.json theme={null}
{
  "agent": "security-reviewer"
}
```

Dieses Beispiel aktiviert den Agent `security-reviewer`, der im Verzeichnis `agents/` des Plugins definiert ist. Einstellungen aus `settings.json` haben Vorrang vor `settings`, die in `plugin.json` deklariert sind. Unbekannte Schlüssel werden stillschweigend ignoriert.

<h3 id="organize-complex-plugins">
  Organisieren Sie komplexe Plugins
</h3>

Für Plugins mit vielen Komponenten organisieren Sie Ihre Verzeichnisstruktur nach Funktionalität. Für vollständige Verzeichnislayouts und Organisationsmuster siehe [Plugin-Verzeichnisstruktur](/docs/de/plugins-reference#plugin-directory-structure).

<h3 id="test-your-plugins-locally">
  Testen Sie Ihre Plugins lokal
</h3>

Verwenden Sie das Flag `--plugin-dir`, um Plugins während der Entwicklung zu testen. Dies lädt Ihr Plugin direkt, ohne dass eine Installation erforderlich ist.

```bash theme={null}
claude --plugin-dir ./my-plugin
```

Das Flag akzeptiert auch ein `.zip`-Archiv des Plugin-Verzeichnisses, das Claude Code v2.1.128 oder später erfordert.

```bash theme={null}
claude --plugin-dir ./my-plugin.zip
```

Wenn ein `--plugin-dir`-Plugin denselben Namen wie ein installiertes Marketplace-Plugin hat, hat die lokale Kopie in dieser Sitzung Vorrang. Dies ermöglicht es Ihnen, Änderungen an einem Plugin zu testen, das Sie bereits installiert haben, ohne es zuerst zu deinstallieren. Die Ausnahme sind Plugins, die durch verwaltete Einstellungen erzwungen aktiviert oder deaktiviert werden: `--plugin-dir` kann diese nicht überschreiben.

Wenn Sie Änderungen an Ihrem Plugin vornehmen, führen Sie `/reload-plugins` aus, um die Updates zu übernehmen, ohne neu zu starten. Dies lädt Plugins, Skills, Agents, Hooks, Plugin-MCP-Server und Plugin-LSP-Server neu. Testen Sie Ihre Plugin-Komponenten:

* Versuchen Sie Ihre Skills mit `/plugin-name:skill-name`
* Überprüfen Sie, dass Agents in `/context` unter Custom Agents angezeigt werden, oder erwähnen Sie einen mit seinem scoped Namen mit @
* Überprüfen Sie, dass Hooks wie erwartet funktionieren

<Tip>
  Sie können mehrere Plugins gleichzeitig laden, indem Sie das Flag mehrmals angeben:

  ```bash theme={null}
  claude --plugin-dir ./plugin-one --plugin-dir ./plugin-two
  ```
</Tip>

Um ein Plugin zu testen, das bereits als `.zip`-Archiv verpackt und unter einer URL gehostet wird, z. B. ein CI-Build-Artefakt, verwenden Sie stattdessen `--plugin-url`. Claude Code ruft das Archiv beim Start ab und lädt es nur für diese Sitzung. Wenn das Abrufen fehlschlägt oder das Archiv ungültig ist, meldet Claude Code einen Plugin-Ladefehler und startet ohne es. Die gleichen [Vertrauensüberlegungen](/docs/de/discover-plugins#security) gelten wie für jede andere Plugin-Quelle: Verweisen Sie dieses Flag nur auf Archive, die Sie kontrollieren oder denen Sie vertrauen.

Um mehrere Plugins zu laden, wiederholen Sie das Flag für jede URL:

```bash theme={null}
claude --plugin-url https://example.com/my-plugin.zip --plugin-url https://example.com/other.zip
```

Oder übergeben Sie durch Leerzeichen getrennte URLs als ein Argument in Anführungszeichen:

```bash theme={null}
claude --plugin-url "https://example.com/my-plugin.zip https://example.com/other.zip"
```

<h3 id="debug-plugin-issues">
  Debuggen Sie Plugin-Probleme
</h3>

Wenn Ihr Plugin nicht wie erwartet funktioniert:

1. **Überprüfen Sie die Struktur**: Stellen Sie sicher, dass Ihre Verzeichnisse auf der Plugin-Root sind, nicht in `.claude-plugin/`
2. **Testen Sie Komponenten einzeln**: Überprüfen Sie jeden Skill, Agent und Hook separat
3. **Verwenden Sie Validierungs- und Debugging-Tools**: Siehe [Debugging- und Entwicklungstools](/docs/de/plugins-reference#debugging-and-development-tools) für CLI-Befehle und Troubleshooting-Techniken

<h3 id="share-your-plugins">
  Teilen Sie Ihre Plugins
</h3>

Wenn Ihr Plugin bereit zum Teilen ist:

1. **Fügen Sie Dokumentation hinzu**: Fügen Sie eine `README.md` mit Installations- und Verwendungsanweisungen ein
2. **Wählen Sie eine Versionierungsstrategie**: Entscheiden Sie, ob Sie eine explizite `version` setzen oder sich auf den Git-Commit-SHA verlassen. Siehe [Versionsverwaltung](/docs/de/plugins-reference#version-management)
3. **Erstellen oder verwenden Sie einen Marketplace**: Verteilen Sie über [Plugin-Marketplaces](/docs/de/plugin-marketplaces) zur Installation
4. **Testen Sie mit anderen**: Lassen Sie Teamkollegen das Plugin vor einer breiteren Verteilung testen

Sobald Ihr Plugin in einem Marketplace ist, können andere es mit den Anweisungen in [Plugins entdecken und installieren](/docs/de/discover-plugins) installieren. Um ein Plugin intern für Ihr Team zu halten, hosten Sie den Marketplace in einem [privaten Repository](/docs/de/plugin-marketplaces#private-repositories).

<h3 id="submit-your-plugin-to-the-community-marketplace">
  Reichen Sie Ihr Plugin beim Community-Marketplace ein
</h3>

Anthropic verwaltet zwei öffentliche Marketplaces für Claude Code Plugins:

* **`claude-plugins-official`**: ein kuratierter Satz von Plugins, die von Anthropic verwaltet werden. Automatisch beim ersten Mal registriert, wenn Sie Claude Code interaktiv starten. Ein nicht-interaktives Skript, das vor diesem ersten Start ausgeführt wird, muss es explizit mit `claude plugin marketplace add anthropics/claude-plugins-official` hinzufügen.
* **`claude-community`**: der öffentliche Community-Marketplace, auf dem Drittanbieter-Einreichungen nach Überprüfung landen. Benutzer fügen ihn mit `/plugin marketplace add anthropics/claude-plugins-community` hinzu und installieren ihn als `@claude-community`.

Um Ihr Plugin zur Überprüfung im Community-Marketplace einzureichen, verwenden Sie eines der In-App-Formulare:

* **claude.ai**: [claude.ai/admin-settings/directory/submissions/plugins/new](https://claude.ai/admin-settings/directory/submissions/plugins/new)
* **Console**: [platform.claude.com/plugins/submit](https://platform.claude.com/plugins/submit)

Das claude.ai-Formular erfordert eine Team- oder Enterprise-Organisation und Verzeichnisverwaltungszugriff; Organisationsinhaber haben diesen Zugriff standardmäßig. Einzelne Autoren, die nicht Teil einer Team- oder Enterprise-Organisation sind, können stattdessen das Console-Formular verwenden.

Führen Sie `claude plugin validate` lokal aus, bevor Sie einreichen. Die Überprüfungs-Pipeline führt die gleiche Überprüfung bei jeder Einreichung durch, zusammen mit automatisierter Sicherheitsüberprüfung.

Genehmigte Plugins werden auf einen bestimmten Commit-SHA im Katalog [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community) angeheftet, und CI erhöht die Anheftung automatisch, wenn Sie neue Commits in Ihr Repository pushen. Der öffentliche Katalog wird jede Nacht aus der Überprüfungs-Pipeline synchronisiert, daher kann es eine Verzögerung zwischen Genehmigung und dem Erscheinen Ihres Plugins in `marketplace.json` geben. Um zu überprüfen, ob Ihr Plugin bereits installierbar ist, suchen Sie nach seinem Namen im [Community-Katalog](https://github.com/anthropics/claude-plugins-community/blob/main/.claude-plugin/marketplace.json).

Der offizielle Marketplace, `claude-plugins-official`, wird separat kuratiert. Anthropic entscheidet nach eigenem Ermessen, welche Plugins einbezogen werden. Es gibt keinen Bewerbungsprozess, und das Einreichungsformular fügt Plugins nicht zum offiziellen Marketplace hinzu.

Wenn Anthropic Ihr Plugin im offiziellen Marketplace auflistet, kann Ihre CLI Claude Code-Benutzer auffordern, es zu installieren. Siehe [Empfehlen Sie Ihr Plugin von Ihrer CLI](/docs/de/plugin-hints).

<Note>
  Für vollständige technische Spezifikationen, Debugging-Techniken und Verteilungsstrategien siehe [Plugins-Referenz](/docs/de/plugins-reference).
</Note>

<h2 id="convert-existing-configurations-to-plugins">
  Konvertieren Sie vorhandene Konfigurationen in Plugins
</h2>

Wenn Sie bereits Skills oder Hooks in Ihrem Verzeichnis `.claude/` haben, können Sie diese in ein Plugin konvertieren, um die Freigabe und Verteilung zu vereinfachen.

<h3 id="migration-steps">
  Migrationschritte
</h3>

<Steps>
  <Step title="Erstellen Sie die Plugin-Struktur">
    Erstellen Sie ein neues Plugin-Verzeichnis in Ihrem Projektstammverzeichnis neben dem vorhandenen Ordner `.claude/`, damit die relativen `cp`-Pfade im nächsten Schritt aufgelöst werden:

    ```bash theme={null}
    mkdir -p my-plugin/.claude-plugin
    ```

    Erstellen Sie die Manifestdatei unter `my-plugin/.claude-plugin/plugin.json`:

    ```json my-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-plugin",
      "description": "Migrated from standalone configuration",
      "version": "1.0.0"
    }
    ```
  </Step>

  <Step title="Kopieren Sie Ihre vorhandenen Dateien">
    Kopieren Sie Ihre vorhandenen Konfigurationen in das Plugin-Verzeichnis:

    ```bash theme={null}
    # Copy commands
    cp -r .claude/commands my-plugin/

    # Copy agents (if any)
    cp -r .claude/agents my-plugin/

    # Copy skills (if any)
    cp -r .claude/skills my-plugin/
    ```
  </Step>

  <Step title="Migrieren Sie Hooks">
    Wenn Sie Hooks in Ihren Einstellungen haben, erstellen Sie ein Hooks-Verzeichnis:

    ```bash theme={null}
    mkdir my-plugin/hooks
    ```

    Erstellen Sie `my-plugin/hooks/hooks.json` mit Ihrer Hooks-Konfiguration. Kopieren Sie das Objekt `hooks` aus Ihrer `.claude/settings.json` oder `settings.local.json`, da das Format gleich ist. Der Befehl empfängt Hook-Eingaben als JSON auf stdin, verwenden Sie also `jq`, um den Dateipfad zu extrahieren:

    ```json my-plugin/hooks/hooks.json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npm run lint:fix" }]
          }
        ]
      }
    }
    ```
  </Step>

  <Step title="Testen Sie Ihr migriertes Plugin">
    Laden Sie Ihr Plugin, um zu überprüfen, ob alles funktioniert:

    ```bash theme={null}
    claude --plugin-dir ./my-plugin
    ```

    Testen Sie jede Komponente: Führen Sie Ihre Befehle aus, überprüfen Sie, dass Agents in `/context` angezeigt werden, und überprüfen Sie, dass Hooks korrekt ausgelöst werden.
  </Step>
</Steps>

<h3 id="what-changes-when-migrating">
  Was sich bei der Migration ändert
</h3>

| Eigenständig (`.claude/`)                 | Plugin                                    |
| :---------------------------------------- | :---------------------------------------- |
| Nur in einem Projekt verfügbar            | Kann über Marketplaces freigegeben werden |
| Dateien in `.claude/commands/`            | Dateien in `plugin-name/commands/`        |
| Hooks in `settings.json`                  | Hooks in `hooks/hooks.json`               |
| Muss manuell kopiert werden, um zu teilen | Mit `/plugin install` installieren        |

<Note>
  Nach der Migration entfernen Sie die ursprünglichen Dateien aus `.claude/`, um Duplikate zu vermeiden. Projekt- und Benutzer-`.claude/agents/`-Definitionen überschreiben gleichnamige Plugin-Agents, daher wird die Plugin-Version erst wirksam, wenn die ursprünglichen Dateien entfernt werden. Plugin-Skills werden als `/plugin-name:skill-name` namensgebunden, daher bleiben sowohl das Original `/skill-name` als auch die Plugin-Kopie verfügbar, anstatt dass eines das andere überschreibt.
</Note>

<h2 id="next-steps">
  Nächste Schritte
</h2>

Jetzt, da Sie das Plugin-System von Claude Code verstehen, finden Sie hier vorgeschlagene Pfade für verschiedene Ziele:

<h3 id="for-plugin-users">
  Für Plugin-Benutzer
</h3>

* [Plugins entdecken und installieren](/docs/de/discover-plugins): Durchsuchen Sie Marketplaces und installieren Sie Plugins
* [Konfigurieren Sie Team-Marketplaces](/docs/de/discover-plugins#configure-team-marketplaces): Richten Sie Repository-Level-Plugins für Ihr Team ein

<h3 id="for-plugin-developers">
  Für Plugin-Entwickler
</h3>

* [Erstellen und verteilen Sie einen Marketplace](/docs/de/plugin-marketplaces): Verpacken und teilen Sie Ihre Plugins
* [Plugins-Referenz](/docs/de/plugins-reference): Vollständige technische Spezifikationen
* Tauchen Sie tiefer in spezifische Plugin-Komponenten ein:
  * [Skills](/docs/de/skills): Details zur Skill-Entwicklung
  * [Subagents](/docs/de/sub-agents): Agent-Konfiguration und Fähigkeiten
  * [Hooks](/docs/de/hooks): Event-Handling und Automatisierung
  * [MCP](/docs/de/mcp): Integration externer Tools
