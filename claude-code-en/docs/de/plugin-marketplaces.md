> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Erstellen und Verteilen eines Plugin-Marktplatzes

> Erstellen und hosten Sie Plugin-Marktplätze, um Claude Code-Erweiterungen in Teams und Communities zu verteilen.

Ein **Plugin-Marktplatz** ist ein Katalog, mit dem Sie Plugins an andere verteilen können. Marktplätze bieten zentrale Entdeckung, Versionsverfolgung, automatische Updates und Unterstützung für mehrere Quellentypen, einschließlich Git-Repositories und lokaler Pfade. Diese Anleitung zeigt Ihnen, wie Sie Ihren eigenen Marktplatz erstellen, um Plugins mit Ihrem Team oder Ihrer Community zu teilen.

Möchten Sie Plugins aus einem vorhandenen Marktplatz installieren? Siehe [Entdecken und Installieren vorgefertigter Plugins](/docs/de/discover-plugins).

<h2 id="overview">
  Übersicht
</h2>

Das Erstellen und Verteilen eines Marktplatzes umfasst:

1. **Plugins erstellen**: Erstellen Sie ein oder mehrere Plugins mit skills, Agents, hooks, MCP servers oder LSP servers. Diese Anleitung setzt voraus, dass Sie bereits Plugins zum Verteilen haben; siehe [Plugins erstellen](/docs/de/plugins) für Details zum Erstellen von Plugins.
2. **Marktplatzdatei erstellen**: Definieren Sie eine `marketplace.json`, die Ihre Plugins und deren Speicherorte auflistet. Siehe [Marktplatzdatei erstellen](#create-the-marketplace-file).
3. **Marktplatz hosten**: Pushen Sie zu GitHub, GitLab oder einem anderen Git-Host. Siehe [Marktplätze hosten und verteilen](#host-and-distribute-marketplaces).
4. **Mit Benutzern teilen**: Benutzer fügen Ihren Marktplatz mit `/plugin marketplace add` hinzu und installieren einzelne Plugins. Siehe [Plugins entdecken und installieren](/docs/de/discover-plugins).

Sobald Ihr Marktplatz live ist, können Sie ihn aktualisieren, indem Sie Änderungen in Ihr Repository pushen. Benutzer aktualisieren ihre lokale Kopie mit `/plugin marketplace update`.

<h2 id="walkthrough-create-a-local-marketplace">
  Anleitung: Erstellen Sie einen lokalen Marktplatz
</h2>

Dieses Beispiel erstellt einen Marktplatz mit einem Plugin: ein `quality-review` skill für Code-Reviews. Sie erstellen die Verzeichnisstruktur, fügen ein skill hinzu, erstellen das Plugin-Manifest und den Marktplatzkatalog und installieren und testen ihn dann.

<Steps>
  <Step title="Erstellen Sie die Verzeichnisstruktur">
    ```bash theme={null}
    mkdir -p my-marketplace/.claude-plugin
    mkdir -p my-marketplace/plugins/quality-review-plugin/.claude-plugin
    mkdir -p my-marketplace/plugins/quality-review-plugin/skills/quality-review
    ```
  </Step>

  <Step title="Erstellen Sie das skill">
    Erstellen Sie eine `SKILL.md`-Datei, die definiert, was das `quality-review` skill tut.

    ```markdown my-marketplace/plugins/quality-review-plugin/skills/quality-review/SKILL.md theme={null}
    ---
    description: Review code for bugs, security, and performance
    ---

    Review the code I've selected or the recent changes for:
    - Potential bugs or edge cases
    - Security concerns
    - Performance issues
    - Readability improvements

    Be concise and actionable.
    ```
  </Step>

  <Step title="Erstellen Sie das Plugin-Manifest">
    Erstellen Sie eine `plugin.json`-Datei, die das Plugin beschreibt. Das Manifest befindet sich im `.claude-plugin/`-Verzeichnis.

    ```json my-marketplace/plugins/quality-review-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "quality-review-plugin",
      "description": "Adds a quality-review skill for quick code reviews",
      "version": "1.0.0"
    }
    ```

    <Note>
      Das Festlegen von `version` bedeutet, dass Benutzer nur Updates erhalten, wenn Sie dieses Feld ändern. Erhöhen Sie es daher bei jeder Veröffentlichung. Wenn Sie `version` weglassen und diesen Marktplatz in Git hosten, zählt jeder Commit automatisch als neue Version. Siehe [Versionsauflösung](#version-resolution-and-release-channels), um den richtigen Ansatz zu wählen.
    </Note>
  </Step>

  <Step title="Erstellen Sie die Marktplatzdatei">
    Erstellen Sie den Marktplatzkatalog, der Ihr Plugin auflistet.

    ```json my-marketplace/.claude-plugin/marketplace.json theme={null}
    {
      "name": "my-plugins",
      "owner": {
        "name": "Your Name"
      },
      "plugins": [
        {
          "name": "quality-review-plugin",
          "source": "./plugins/quality-review-plugin",
          "description": "Adds a quality-review skill for quick code reviews"
        }
      ]
    }
    ```
  </Step>

  <Step title="Hinzufügen und Installieren">
    Fügen Sie den Marktplatz hinzu und installieren Sie das Plugin.

    ```shell theme={null}
    /plugin marketplace add ./my-marketplace
    /plugin install quality-review-plugin@my-plugins
    ```
  </Step>

  <Step title="Probieren Sie es aus">
    Wählen Sie etwas Code in Ihrem Editor aus und führen Sie Ihr neues skill aus. Plugin-skills sind mit dem Plugin-Namen namespaced.

    ```shell theme={null}
    /quality-review-plugin:quality-review
    ```
  </Step>
</Steps>

Um mehr über die Möglichkeiten von Plugins zu erfahren, einschließlich hooks, Agents, MCP servers und LSP servers, siehe [Plugins](/docs/de/plugins).

<Note>
  **Wie Plugins installiert werden**: Wenn Benutzer ein Plugin installieren, kopiert Claude Code das Plugin-Verzeichnis an einen Cache-Speicherort. Das bedeutet, dass Plugins keine Dateien außerhalb ihres Verzeichnisses mit Pfaden wie `../shared-utils` referenzieren können, da diese Dateien nicht kopiert werden.

  Wenn Sie Dateien über Plugins hinweg teilen müssen, verwenden Sie Symlinks. Siehe [Plugin-Caching und Dateiauflösung](/docs/de/plugins-reference#plugin-caching-and-file-resolution) für Details.
</Note>

<h2 id="create-the-marketplace-file">
  Marktplatzdatei erstellen
</h2>

Erstellen Sie `.claude-plugin/marketplace.json` im Stammverzeichnis Ihres Repositories. Diese Datei definiert den Namen Ihres Marktplatzes, Eigentümerinformationen und eine Liste von Plugins mit ihren Quellen.

Jeder Plugin-Eintrag benötigt mindestens einen `name` und eine `source`, die Claude Code mitteilt, woher es abgerufen werden soll. Siehe das [vollständige Schema](#marketplace-schema) unten für alle verfügbaren Felder.

```json theme={null}
{
  "name": "company-tools",
  "owner": {
    "name": "DevTools Team",
    "email": "devtools@example.com"
  },
  "plugins": [
    {
      "name": "code-formatter",
      "source": "./plugins/formatter",
      "description": "Automatic code formatting on save",
      "version": "2.1.0",
      "author": {
        "name": "DevTools Team"
      }
    },
    {
      "name": "deployment-tools",
      "source": {
        "source": "github",
        "repo": "company/deploy-plugin"
      },
      "description": "Deployment automation tools"
    }
  ]
}
```

<h2 id="marketplace-schema">
  Marktplatz-Schema
</h2>

<h3 id="required-fields">
  Erforderliche Felder
</h3>

| Feld      | Typ    | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | Beispiel       |
| :-------- | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------- |
| `name`    | string | Marktplatz-Identifier (Kebab-Case, keine Leerzeichen). Dies ist öffentlich sichtbar: Benutzer sehen es beim Installieren von Plugins (z. B. `/plugin install my-tool@your-marketplace`). Jeder Benutzer kann nur einen Marktplatz pro Name registrieren: Das Hinzufügen eines zweiten Marktplatzes mit demselben Namen ersetzt den ersten. Um mehrere Plugins unter einem Marktplatznamen zu veröffentlichen, listen Sie diese alle in einer [einzelnen `marketplace.json`](#create-the-marketplace-file) auf. | `"acme-tools"` |
| `owner`   | object | Informationen zum Marktplatz-Betreuer ([siehe Felder unten](#owner-fields))                                                                                                                                                                                                                                                                                                                                                                                                                                    |                |
| `plugins` | array  | Liste der verfügbaren Plugins                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Siehe unten    |

<Note>
  **Reservierte Namen**: Die folgenden Marktplatznamen sind für die offizielle Nutzung durch Anthropic reserviert und können nicht von Drittanbieter-Marktplätzen verwendet werden: `claude-code-marketplace`, `claude-code-plugins`, `claude-plugins-official`, `claude-plugins-community`, `claude-community`, `anthropic-marketplace`, `anthropic-plugins`, `agent-skills`, `anthropic-agent-skills`, `knowledge-work-plugins`, `life-sciences`, `claude-for-legal`, `claude-for-financial-services`, `financial-services-plugins`, `first-party-plugins`, `healthcare`. Namen, die offizielle Marktplätze imitieren, wie `official-claude-plugins` oder `anthropic-plugins-v2`, sind ebenfalls blockiert. Das Reservieren dieser Namen verhindert, dass sich ein Drittanbieter-Marktplatz als von Anthropic veröffentlichte Quelle darstellt.

  Claude Code überprüft reservierte Namen jedes Mal, wenn es einen Marktplatz lädt, nicht nur wenn Sie einen hinzufügen. Ein Marktplatz, der unter einem dieser Namen registriert wurde, bevor der Name reserviert wurde, wird nicht mehr geladen und meldet, dass er [von einer nicht vertrauenswürdigen Quelle registriert ist](/docs/de/errors#marketplace-is-registered-from-an-untrusted-source). Entfernen Sie diesen Marktplatz und fügen Sie ihn erneut aus der offiziellen Anthropic-Quelle hinzu. Ein Drittanbieter-Marktplatz, der von einem neu reservierten Namen betroffen ist, wird erneut geladen, sobald Sie ihn unter einem anderen Namen erneut hinzufügen. Vor v2.1.205 waren `first-party-plugins` und `healthcare` nicht reserviert, und ein Marktplatz, der bereits unter einem reservierten Namen registriert war, wurde weiterhin geladen.
</Note>

<h3 id="owner-fields">
  Eigentümer-Felder
</h3>

| Feld    | Typ    | Erforderlich | Beschreibung                    |
| :------ | :----- | :----------- | :------------------------------ |
| `name`  | string | Ja           | Name des Betreuers oder Teams   |
| `email` | string | Nein         | Kontakt-E-Mail für den Betreuer |

<h3 id="optional-fields">
  Optionale Felder
</h3>

| Feld                                  | Typ    | Beschreibung                                                                                                                                                                                                                                                                                                                                                         |
| :------------------------------------ | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$schema`                             | string | JSON-Schema-URL für Editor-Autovervollständigung und Validierung. Claude Code ignoriert dieses Feld beim Laden.                                                                                                                                                                                                                                                      |
| `description`                         | string | Kurze Marktplatzbeschreibung                                                                                                                                                                                                                                                                                                                                         |
| `version`                             | string | Marktplatz-Manifest-Version                                                                                                                                                                                                                                                                                                                                          |
| `metadata.pluginRoot`                 | string | Basisverzeichnis, das relativen Plugin-Quellpfaden vorangestellt wird (z. B. `"./plugins"` ermöglicht es Ihnen, `"source": "formatter"` statt `"source": "./plugins/formatter"` zu schreiben)                                                                                                                                                                        |
| `allowCrossMarketplaceDependenciesOn` | array  | Andere Marktplätze, von denen Plugins in diesem Marktplatz abhängen können. Abhängigkeiten von einem Marktplatz, der hier nicht aufgelistet ist, werden bei der Installation blockiert. Siehe [Von einem Plugin aus einem anderen Marktplatz abhängen](/docs/de/plugin-dependencies#depend-on-a-plugin-from-another-marketplace).                                         |
| `renames`                             | object | Zuordnung von einem früheren Plugin-`name` zu seinem aktuellen Namen oder zu `null`, wenn das Plugin entfernt wurde. Ermöglicht es bestehenden Benutzern, automatisch zu migrieren, wenn Sie einen Eintrag in `plugins` umbenennen oder entfernen. Siehe [Plugin umbenennen oder entfernen](#rename-or-remove-a-plugin). Erfordert Claude Code v2.1.193 oder später. |

`description` und `version` werden auch unter `metadata` für Rückwärtskompatibilität akzeptiert.

<h2 id="plugin-entries">
  Plugin-Einträge
</h2>

Jeder Plugin-Eintrag im `plugins`-Array beschreibt ein Plugin und wo man es findet. Sie können jedes Feld aus dem [Plugin-Manifest-Schema](/docs/de/plugins-reference#plugin-manifest-schema) einbeziehen, wie `description`, `version`, `author`, `commands` und `hooks`, plus diese Marktplatz-spezifischen Felder: `source`, `category`, `tags`, `strict` und `relevance`.

<h3 id="required-fields-2">
  Erforderliche Felder
</h3>

| Feld     | Typ            | Beschreibung                                                                                                                                                          |
| :------- | :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`   | string         | Plugin-Identifier (Kebab-Case, keine Leerzeichen). Dies ist öffentlich sichtbar: Benutzer sehen es beim Installieren (z. B. `/plugin install my-plugin@marketplace`). |
| `source` | string\|object | Wo das Plugin abgerufen werden soll (siehe [Plugin-Quellen](#plugin-sources) unten)                                                                                   |

<h3 id="optional-plugin-fields">
  Optionale Plugin-Felder
</h3>

**Standard-Metadatenfelder:**

| Feld             | Typ     | Beschreibung                                                                                                                                                                                                                                                                                                                                                  |
| :--------------- | :------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `displayName`    | string  | Benutzerfreundlicher Name, der in UI-Oberflächen angezeigt wird. Fällt auf `name` zurück, wenn weggelassen. Kann Leerzeichen und beliebige Groß-/Kleinschreibung enthalten. Wird nicht für Namensräume oder Suche verwendet. Erfordert Claude Code v2.1.143 oder später.                                                                                      |
| `description`    | string  | Kurze Plugin-Beschreibung                                                                                                                                                                                                                                                                                                                                     |
| `version`        | string  | Plugin-Version. Falls gesetzt (hier oder in `plugin.json`), wird das Plugin auf diese Zeichenkette festgelegt und Benutzer erhalten Updates nur, wenn sie sich ändert. Weglassen, um auf den Git-Commit-SHA zurückzugreifen. Siehe [Versionsauflösung](#version-resolution-and-release-channels).                                                             |
| `author`         | object  | Plugin-Autoreninformationen (`name` erforderlich, `email` optional)                                                                                                                                                                                                                                                                                           |
| `homepage`       | string  | Plugin-Homepage oder Dokumentations-URL                                                                                                                                                                                                                                                                                                                       |
| `repository`     | string  | Quellcode-Repository-URL                                                                                                                                                                                                                                                                                                                                      |
| `license`        | string  | SPDX-Lizenz-Identifier (z. B. MIT, Apache-2.0)                                                                                                                                                                                                                                                                                                                |
| `keywords`       | array   | Tags für Plugin-Entdeckung und Kategorisierung                                                                                                                                                                                                                                                                                                                |
| `category`       | string  | Plugin-Kategorie zur Organisation                                                                                                                                                                                                                                                                                                                             |
| `tags`           | array   | Tags für Suchbarkeit                                                                                                                                                                                                                                                                                                                                          |
| `strict`         | boolean | Steuert, ob `plugin.json` die Autorität für Komponentendefinitionen ist (Standard: true). Siehe [Strict Mode](#strict-mode) unten.                                                                                                                                                                                                                            |
| `relevance`      | object  | Signale, die Claude Code mitteilen, wann dieses Plugin Benutzern empfohlen werden soll. Wirkt sich nur auf Marktplätze aus, die ein Administrator in verwalteten Einstellungen auf die Whitelist setzt. Siehe [Plugins für Ihre Organisation empfehlen](/docs/de/plugin-relevance). Erfordert Claude Code v2.1.152 oder später.                                    |
| `defaultEnabled` | boolean | Ob das Plugin nach der Installation aktiviert ist (Standard: true). Setzen Sie auf `false`, um das Plugin deaktiviert zu installieren, bis sich der Benutzer anmeldet. Hat Vorrang vor dem gleichen Feld in der `plugin.json` des Plugins. Siehe [Standardaktivierung](/docs/de/plugins-reference#default-enablement). Erfordert Claude Code v2.1.154 oder später. |

**Komponenten-Konfigurationsfelder:**

| Feld         | Typ            | Beschreibung                                                                      |
| :----------- | :------------- | :-------------------------------------------------------------------------------- |
| `skills`     | string\|array  | Benutzerdefinierte Pfade zu Skill-Verzeichnissen, die `<name>/SKILL.md` enthalten |
| `commands`   | string\|array  | Benutzerdefinierte Pfade zu flachen `.md` Skill-Dateien oder Verzeichnissen       |
| `agents`     | string\|array  | Benutzerdefinierte Pfade zu Agent-Dateien                                         |
| `hooks`      | string\|object | Benutzerdefinierte hooks-Konfiguration oder Pfad zu hooks-Datei                   |
| `mcpServers` | string\|object | MCP server-Konfigurationen oder Pfad zu MCP-Konfiguration                         |
| `lspServers` | string\|object | LSP server-Konfigurationen oder Pfad zu LSP-Konfiguration                         |

<h2 id="plugin-sources">
  Plugin-Quellen
</h2>

Plugin-Quellen teilen Claude Code mit, wo jedes einzelne Plugin in Ihrem Marktplatz abgerufen werden soll. Diese werden im `source`-Feld jedes Plugin-Eintrags in `marketplace.json` festgelegt.

Sobald Claude Code ein Plugin klont oder auf den lokalen Computer herunterlädt, wird es in den lokalen versionierten Plugin-Cache unter `~/.claude/plugins/cache` kopiert.

| Quelle         | Typ                              | Felder                             | Notizen                                                                                                                                             |
| -------------- | -------------------------------- | ---------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| Relativer Pfad | `string` (z. B. `"./my-plugin"`) | keine                              | Lokales Verzeichnis im Marktplatz-Repo. Muss mit `./` beginnen. Wird relativ zum Marktplatz-Root aufgelöst, nicht zum `.claude-plugin/`-Verzeichnis |
| `github`       | object                           | `repo`, `ref?`, `sha?`             |                                                                                                                                                     |
| `url`          | object                           | `url`, `ref?`, `sha?`              | Git-URL-Quelle                                                                                                                                      |
| `git-subdir`   | object                           | `url`, `path`, `ref?`, `sha?`      | Unterverzeichnis in einem Git-Repo. Klont sparsam, um die Bandbreite für Monorepos zu minimieren                                                    |
| `npm`          | object                           | `package`, `version?`, `registry?` | Installiert über `npm install`                                                                                                                      |

<Note>
  **Marktplatz-Quellen vs. Plugin-Quellen**: Dies sind unterschiedliche Konzepte, die unterschiedliche Dinge steuern.

  * **Marktplatz-Quelle**: wo der `marketplace.json`-Katalog selbst abgerufen werden soll. Wird festgelegt, wenn Benutzer `/plugin marketplace add` ausführen oder in `extraKnownMarketplaces`-Einstellungen. Unterstützt `ref` (Branch/Tag), aber nicht `sha`.
  * **Plugin-Quelle**: wo ein einzelnes Plugin in der Marktplatz-Liste abgerufen werden soll. Wird im `source`-Feld jedes Plugin-Eintrags in `marketplace.json` festgelegt. Unterstützt sowohl `ref` (Branch/Tag) als auch `sha` (exakter Commit).

  Beispielsweise kann ein Marktplatz, der unter `acme-corp/plugin-catalog` gehostet wird (Marktplatz-Quelle), ein Plugin auflisten, das von `acme-corp/code-formatter` abgerufen wird (Plugin-Quelle). Die Marktplatz-Quelle und die Plugin-Quelle verweisen auf unterschiedliche Repositories und werden unabhängig voneinander angeheftet.
</Note>

Die Git-basierten Quellentypen unten sind `github`, `url` und `git-subdir`. Wenn sowohl `ref` als auch `sha` auf einem von ihnen gesetzt sind, ist `sha` die effektive Anheftung. Claude Code ruft den angehefteten Commit direkt ab und checkt ihn aus.

Auf den meisten Git-Hosts, einschließlich GitHub, GitLab und Bitbucket, bedeutet dies, dass die Installation erfolgreich ist, auch wenn der Branch oder Tag, der durch `ref` benannt wird, inzwischen upstream gelöscht wurde, solange der Commit noch vom Repository aus erreichbar ist. Einige Server, wie AWS CodeCommit, unterstützen das Abrufen von Commits nach SHA nicht. Auf diesen Servern muss `ref` noch vorhanden sein und der angeheftete Commit muss von ihm aus erreichbar sein.

<h3 id="relative-paths">
  Relative Pfade
</h3>

Für Plugins im selben Repository verwenden Sie einen Pfad, der mit `./` beginnt:

```json theme={null}
{
  "name": "my-plugin",
  "source": "./plugins/my-plugin"
}
```

Pfade werden relativ zum Marktplatz-Root aufgelöst, das ist das Verzeichnis, das `.claude-plugin/` enthält. Im obigen Beispiel verweist `./plugins/my-plugin` auf `<repo>/plugins/my-plugin`, obwohl `marketplace.json` unter `<repo>/.claude-plugin/marketplace.json` lebt. Verwenden Sie nicht `../`, um Pfade außerhalb des Marktplatz-Root zu referenzieren.

<Note>
  Relative Pfade werden gegen eine lokale Kopie des Marktplatzes aufgelöst, daher funktionieren sie, wenn Benutzer Ihren Marktplatz aus einer Git-Quelle oder einem lokalen Verzeichnis hinzufügen. Wenn Benutzer Ihren Marktplatz über eine direkte URL zur `marketplace.json`-Datei hinzufügen, werden relative Pfade nicht aufgelöst, da nur diese Datei heruntergeladen wird. Verwenden Sie für URL-basierte Verteilung stattdessen GitHub-, npm- oder Git-URL-Quellen. Siehe [Fehlerbehebung](#plugins-with-relative-paths-fail-in-url-based-marketplaces) für Details.
</Note>

<h3 id="github-repositories">
  GitHub-Repositories
</h3>

```json theme={null}
{
  "name": "github-plugin",
  "source": {
    "source": "github",
    "repo": "owner/plugin-repo"
  }
}
```

Sie können an einen bestimmten Branch, Tag oder Commit anheften:

```json theme={null}
{
  "name": "github-plugin",
  "source": {
    "source": "github",
    "repo": "owner/plugin-repo",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

| Feld   | Typ    | Beschreibung                                                                            |
| :----- | :----- | :-------------------------------------------------------------------------------------- |
| `repo` | string | Erforderlich. GitHub-Repository im Format `owner/repo`                                  |
| `ref`  | string | Optional. Git-Branch oder Tag (Standard: Standard-Branch des Repositories)              |
| `sha`  | string | Optional. Vollständiger 40-stelliger Git-Commit-SHA zum Anheften an eine exakte Version |

<h3 id="git-repositories">
  Git-Repositories
</h3>

```json theme={null}
{
  "name": "git-plugin",
  "source": {
    "source": "url",
    "url": "https://gitlab.com/team/plugin.git"
  }
}
```

Sie können an einen bestimmten Branch, Tag oder Commit anheften:

```json theme={null}
{
  "name": "git-plugin",
  "source": {
    "source": "url",
    "url": "https://gitlab.com/team/plugin.git",
    "ref": "main",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

| Feld  | Typ    | Beschreibung                                                                                                                                                                      |
| :---- | :----- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url` | string | Erforderlich. Vollständige Git-Repository-URL (`https://` oder `git@`). Das `.git`-Suffix ist optional, daher funktionieren Azure DevOps- und AWS CodeCommit-URLs ohne das Suffix |
| `ref` | string | Optional. Git-Branch oder Tag (Standard: Standard-Branch des Repositories)                                                                                                        |
| `sha` | string | Optional. Vollständiger 40-stelliger Git-Commit-SHA zum Anheften an eine exakte Version                                                                                           |

<h3 id="git-subdirectories">
  Git-Unterverzeichnisse
</h3>

Verwenden Sie `git-subdir`, um auf ein Plugin zu verweisen, das sich in einem Unterverzeichnis eines Git-Repositories befindet. Claude Code verwendet einen sparsamen, teilweisen Klon, um nur das Unterverzeichnis abzurufen und die Bandbreite für große Monorepos zu minimieren.

```json theme={null}
{
  "name": "my-plugin",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/acme-corp/monorepo.git",
    "path": "tools/claude-plugin"
  }
}
```

Sie können an einen bestimmten Branch, Tag oder Commit anheften:

```json theme={null}
{
  "name": "my-plugin",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/acme-corp/monorepo.git",
    "path": "tools/claude-plugin",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

Das `url`-Feld akzeptiert auch eine GitHub-Kurzform (`owner/repo`) oder SSH-URLs (`git@github.com:owner/repo.git`).

| Feld   | Typ    | Beschreibung                                                                                       |
| :----- | :----- | :------------------------------------------------------------------------------------------------- |
| `url`  | string | Erforderlich. Git-Repository-URL, GitHub `owner/repo`-Kurzform oder SSH-URL                        |
| `path` | string | Erforderlich. Unterverzeichnispfad im Repo, das das Plugin enthält (z. B. `"tools/claude-plugin"`) |
| `ref`  | string | Optional. Git-Branch oder Tag (Standard: Standard-Branch des Repositories)                         |
| `sha`  | string | Optional. Vollständiger 40-stelliger Git-Commit-SHA zum Anheften an eine exakte Version            |

<h3 id="npm-packages">
  npm-Pakete
</h3>

Plugins, die als npm-Pakete verteilt werden, werden mit `npm install` installiert. Dies funktioniert mit jedem Paket in der öffentlichen npm-Registry oder einer privaten Registry, die Ihr Team hostet.

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin"
  }
}
```

Um an eine bestimmte Version anzuheften, fügen Sie das `version`-Feld hinzu:

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin",
    "version": "2.1.0"
  }
}
```

Um von einer privaten oder internen Registry zu installieren, fügen Sie das `registry`-Feld hinzu:

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin",
    "version": "^2.0.0",
    "registry": "https://npm.example.com"
  }
}
```

| Feld       | Typ    | Beschreibung                                                                                                  |
| :--------- | :----- | :------------------------------------------------------------------------------------------------------------ |
| `package`  | string | Erforderlich. Paketname oder Scoped-Paket (z. B. `@org/plugin`)                                               |
| `version`  | string | Optional. Version oder Versionsspanne (z. B. `2.1.0`, `^2.0.0`, `~1.5.0`)                                     |
| `registry` | string | Optional. Benutzerdefinierte npm-Registry-URL. Standard ist die System-npm-Registry (normalerweise npmjs.org) |

<h3 id="advanced-plugin-entries">
  Erweiterte Plugin-Einträge
</h3>

Dieses Beispiel zeigt einen Plugin-Eintrag mit vielen optionalen Feldern, einschließlich benutzerdefinierter Pfade für Befehle, Agents, hooks und MCP-Server:

```json theme={null}
{
  "name": "enterprise-tools",
  "source": {
    "source": "github",
    "repo": "company/enterprise-plugin"
  },
  "description": "Enterprise workflow automation tools",
  "version": "2.1.0",
  "author": {
    "name": "Enterprise Team",
    "email": "enterprise@example.com"
  },
  "homepage": "https://docs.example.com/plugins/enterprise-tools",
  "repository": "https://github.com/company/enterprise-plugin",
  "license": "MIT",
  "keywords": ["enterprise", "workflow", "automation"],
  "category": "productivity",
  "commands": [
    "./commands/core/",
    "./commands/enterprise/",
    "./commands/experimental/preview.md"
  ],
  "agents": ["./agents/security-reviewer.md", "./agents/compliance-checker.md"],
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PLUGIN_ROOT}/scripts/validate.sh"
          }
        ]
      }
    ]
  },
  "mcpServers": {
    "enterprise-db": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"]
    }
  },
  "strict": false
}
```

Wichtige Dinge zu beachten:

* **`commands` und `agents`**: Sie können mehrere Verzeichnisse oder einzelne Dateien angeben. Pfade sind relativ zum Plugin-Root.
* **`${CLAUDE_PLUGIN_ROOT}`**: Verwenden Sie diese Variable in hooks und MCP-Server-Konfigurationen, um auf Dateien im Installationsverzeichnis des Plugins zu verweisen. Dies ist notwendig, da Plugins beim Installieren an einen Cache-Speicherort kopiert werden.
  * Siehe die [Substitutionstabelle](/docs/de/plugins-reference#environment-variables) für welche Konfigurationsfelder sie pro Servertyp ersetzen
  * Verwenden Sie für Abhängigkeiten oder Status, die Plugin-Updates überstehen sollten, stattdessen [`${CLAUDE_PLUGIN_DATA}`](/docs/de/plugins-reference#persistent-data-directory)
* **`strict: false`**: Da dies auf false gesetzt ist, benötigt das Plugin keine eigene `plugin.json`. Der Marktplatz-Eintrag definiert alles. Siehe [Strict Mode](#strict-mode) unten.

Standardmäßig werden die Skills eines Plugins aus dem `skills/`-Verzeichnis unter seiner `source` geladen. Pfade, die im `skills`-Feld aufgelistet sind, werden zu diesem Scan hinzugefügt:

```json theme={null}
"skills": ["./skills/", "./extra-skills/"]
```

Wenn mehrere Plugin-Einträge einen `skills/`-Ordner im Marktplatz-Root gemeinsam nutzen (`source: "./"`), listen Sie stattdessen bestimmte Unterverzeichnisse auf, damit jeder Eintrag nur seine eigenen Skills lädt:

```json theme={null}
"source": "./",
"skills": ["./skills/code-review", "./skills/docs"]
```

Mit einer Marktplatz-Root-Quelle ist die aufgelistete Pfadliste die vollständige Menge für diesen Eintrag, und andere Verzeichnisse im gemeinsamen `skills/`-Ordner werden nicht geladen. Das Auflisten des `skills/`-Verzeichnisses selbst oder des Plugin-Root behält den vollständigen Scan bei. Wenn keiner der aufgelisteten Pfade existiert, wird stattdessen der Standard-Scan ausgeführt.

<h3 id="strict-mode">
  Strict Mode
</h3>

Das `strict`-Feld steuert, ob `plugin.json` die Autorität für Komponentendefinitionen ist (skills, Agents, hooks, MCP-Server, Ausgabestile).

| Wert              | Verhalten                                                                                                                                                                                  |
| :---------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `true` (Standard) | `plugin.json` ist die Autorität. Der Marktplatz-Eintrag kann es mit zusätzlichen Komponenten ergänzen, und beide Quellen werden zusammengeführt.                                           |
| `false`           | Der Marktplatz-Eintrag ist die gesamte Definition. Wenn das Plugin auch eine `plugin.json` hat, die Komponenten deklariert, ist das ein Konflikt und das Plugin kann nicht geladen werden. |

**Wann jeder Modus verwendet werden sollte:**

* **`strict: true`**: Das Plugin hat seine eigene `plugin.json` und verwaltet seine eigenen Komponenten. Der Marktplatz-Eintrag kann zusätzliche Skills oder hooks hinzufügen. Dies ist der Standard und funktioniert für die meisten Plugins.
* **`strict: false`**: Der Marktplatz-Betreiber möchte vollständige Kontrolle. Das Plugin-Repo stellt Rohdateien bereit, und der Marktplatz-Eintrag definiert, welche dieser Dateien als Skills, Agents, hooks usw. verfügbar gemacht werden. Nützlich, wenn der Marktplatz die Komponenten eines Plugins anders strukturiert oder kuratiert als vom Plugin-Autor beabsichtigt.

<h2 id="host-and-distribute-marketplaces">
  Marktplätze hosten und verteilen
</h2>

<h3 id="host-on-github-recommended">
  Auf GitHub hosten (empfohlen)
</h3>

GitHub ist die empfohlene Methode zum Hosten und Verteilen eines Marktplatzes:

1. **Repository erstellen**: Richten Sie ein neues Repository für Ihren Marktplatz ein
2. **Marktplatzdatei hinzufügen**: Erstellen Sie `.claude-plugin/marketplace.json` mit Ihren Plugin-Definitionen
3. **Mit Teams teilen**: Benutzer fügen Ihren Marktplatz mit `/plugin marketplace add owner/repo` hinzu

**Vorteile**: Integrierte Versionskontrolle, Issue-Tracking und Team-Zusammenarbeitsfunktionen.

<h3 id="host-on-other-git-services">
  Auf anderen Git-Services hosten
</h3>

Jeder Git-Hosting-Service funktioniert, wie GitLab, Bitbucket und selbstgehostete Server. Benutzer fügen mit der vollständigen Repository-URL hinzu:

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git
```

<h3 id="private-repositories">
  Private Repositories
</h3>

Claude Code unterstützt die Installation von Plugins aus privaten Repositories. Für manuelle Installation und Updates verwendet Claude Code Ihre vorhandenen Git-Credential-Helper, daher funktioniert HTTPS-Zugriff über `gh auth login`, macOS Keychain oder `git-credential-store` genauso wie in Ihrem Terminal. SSH-Zugriff funktioniert, solange der Host bereits in Ihrer `known_hosts`-Datei vorhanden ist und der Schlüssel in `ssh-agent` geladen ist, da Claude Code interaktive SSH-Eingabeaufforderungen für den Host-Fingerprint und die Schlüsselpassphrase unterdrückt. GitHub `owner/repo`-Kurzform-Quellen klonen standardmäßig über SSH; legen Sie [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/de/env-vars#variables) fest, um sie stattdessen über HTTPS zu klonen.

Hintergrund-Auto-Updates funktionieren anders. Standardmäßig deaktiviert die Hintergrund-Aktualisierung Git-Credential-Helper für seinen `git pull`, sodass der Pull sich nicht bei privaten Repositories über HTTPS authentifizieren kann, auch wenn ein Helper konfiguriert ist. SSH-Remotes sind nicht betroffen: Ein in `ssh-agent` geladener Schlüssel authentifiziert Hintergrund-Pulls genauso wie manuelle Operationen. Wenn der Hintergrund-Pull fehlschlägt, greift Claude Code auf das erneute Klonen des Marktplatzes von Grund auf zurück. Das erneute Klonen verwendet Ihre gespeicherten Git-Anmeldedaten, kann aber bei großen Repositories [zeitlich überschritten werden](#git-operations-time-out), daher können Auto-Updates für private Marktplätze intermittierend fehlschlagen.

Zwei Einstellungen machen private Marktplätze vorhersehbar:

* Legen Sie `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1` fest, um den vorhandenen Klon beizubehalten, wenn der Hintergrund-Pull fehlschlägt, anstatt zu löschen und erneut zu klonen. Ihre Plugins funktionieren weiterhin aus dem letzten synchronisierten Zustand, und manuelle Updates mit `/plugin marketplace update` ziehen immer noch mit Ihren Anmeldedaten.
* Konfigurieren Sie einen Git-Credential-Helper, beispielsweise mit `gh auth setup-git` für GitHub, damit das erneute Klonen-Fallback sich ohne Eingabeaufforderung authentifizieren kann.

Das Festlegen eines Provider-Tokens wie `GITHUB_TOKEN` in Ihrer Umgebung ermöglicht nicht von selbst die Hintergrund-Authentifizierung. Tokens wirken sich nur durch einen konfigurierten Credential-Helper aus, beispielsweise den Helper der `gh` CLI, der `GH_TOKEN` und `GITHUB_TOKEN` liest.

Um den Hintergrund-Pull selbst über HTTPS zu authentifizieren, konfigurieren Sie ein globales Git-URL-Rewrite. Das Rewrite bettet ein Token in die Remote-URL ein, sodass es wirksam wird, obwohl der Hintergrund-Pull Credential-Helper deaktiviert, und ein erfolgreicher Pull überspringt das erneute Klonen-Fallback. Das folgende Beispiel schreibt die URL des Marktplatz-Repositories um, um ein Zugriffs-Token einzuschließen:

```bash theme={null}
git config --global url."https://x-access-token:YOUR_TOKEN@github.com/acme-corp/plugins".insteadOf "https://github.com/acme-corp/plugins"
```

Beschränken Sie das Rewrite auf den Marktplatz-Repository oder Organisations-Pfad. Ein Rewrite, dessen Basis nur der Host ist, gilt für jeden Abruf und Push zu diesem Host auf der Maschine und überschreibt Ihre normalen Anmeldedaten, einschließlich Pushes zu Ihren eigenen Repositories.

Jeder Provider erwartet einen anderen Benutzernamen in der umgeschriebenen URL, und die gleiche Pfad-Scoping gilt für jeden Provider. Für selbstgehostete Server ersetzen Sie den Hostnamen durch den Hostnamen Ihres Servers:

| Provider  | Umgeschriebene URL-Form                                           |
| :-------- | :---------------------------------------------------------------- |
| GitHub    | `https://x-access-token:YOUR_TOKEN@github.com/acme-corp/plugins`  |
| GitLab    | `https://oauth2:YOUR_TOKEN@gitlab.com/acme-corp/plugins`          |
| Bitbucket | `https://x-token-auth:YOUR_TOKEN@bitbucket.org/acme-corp/plugins` |

Das Rewrite speichert das Token im Klartext in Ihrer Gitconfig, daher verwenden Sie ein Token mit Nur-Lese-Zugriff auf das Marktplatz-Repository.

<Note>
  Konfigurieren Sie in CI/CD-Umgebungen einen Git-Credential-Helper, bevor Sie Plugins aus privaten Repositories installieren. Auf GitHub Actions exportieren Sie ein Token mit Lesezugriff auf das Marktplatz-Repository als `GH_TOKEN`, führen Sie dann `gh auth setup-git` aus. Das Standard-Workflow-Token kann nur auf das eigene Repository des Workflows zugreifen, daher benötigt ein privater Marktplatz in einem anderen Repository ein persönliches Zugriffs-Token oder App-Token. Ein globales URL-Rewrite, das in der Pipeline konfiguriert ist, authentifiziert auch den Hintergrund-Pull direkt.
</Note>

<h3 id="test-locally-before-distribution">
  Lokal vor der Verteilung testen
</h3>

Testen Sie Ihren Marktplatz lokal, bevor Sie ihn teilen:

```shell theme={null}
/plugin marketplace add ./my-marketplace
/plugin install quality-review-plugin@my-plugins
```

Für die vollständige Palette von Add-Befehlen (GitHub, Git-URLs, lokale Pfade, Remote-URLs) siehe [Marktplätze hinzufügen](/docs/de/discover-plugins#add-marketplaces).

<h3 id="require-marketplaces-for-your-team">
  Marktplätze für Ihr Team erforderlich machen
</h3>

Sie können Ihr Repository so konfigurieren, dass Teammitglieder automatisch aufgefordert werden, Ihren Marktplatz zu installieren, wenn sie dem Projektordner vertrauen. Fügen Sie Ihren Marktplatz zu `.claude/settings.json` hinzu:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "company-tools": {
      "source": {
        "source": "github",
        "repo": "your-org/claude-plugins"
      }
    }
  }
}
```

Sie können auch angeben, welche Plugins standardmäßig aktiviert sein sollen:

```json theme={null}
{
  "enabledPlugins": {
    "code-formatter@company-tools": true,
    "deployment-tools@company-tools": true
  }
}
```

Für vollständige Konfigurationsoptionen siehe [Plugin-Einstellungen](/docs/de/settings#plugin-settings).

<Note>
  Wenn Sie eine lokale `directory`- oder `file`-Quelle mit einem relativen Pfad verwenden, wird der Pfad gegen den Haupt-Checkout Ihres Repositories aufgelöst. Wenn Sie Claude Code aus einem Git Worktree ausführen, verweist der Pfad immer noch auf den Haupt-Checkout, sodass alle Worktrees denselben Marktplatz-Speicherort teilen. Der Marktplatz-Status wird einmal pro Benutzer in `~/.claude/plugins/known_marketplaces.json` gespeichert, nicht pro Projekt.
</Note>

<h3 id="pre-populate-plugins-for-containers">
  Plugins für Container vorab ausfüllen
</h3>

Für Container-Images und CI-Umgebungen können Sie ein Plugins-Verzeichnis zur Build-Zeit vorab ausfüllen, damit Claude Code mit bereits verfügbaren Marktplätzen und Plugins startet, ohne zur Laufzeit etwas zu klonen. Legen Sie die Umgebungsvariable `CLAUDE_CODE_PLUGIN_SEED_DIR` fest, um auf dieses Verzeichnis zu verweisen.

Um mehrere Seed-Verzeichnisse zu schichten, trennen Sie Pfade mit `:` auf Unix oder `;` auf Windows. Claude Code durchsucht jedes Verzeichnis in der Reihenfolge und verwendet den ersten Seed, der einen bestimmten Marktplatz oder Plugin-Cache enthält.

Das Seed-Verzeichnis spiegelt die Struktur von `~/.claude/plugins`:

```
$CLAUDE_CODE_PLUGIN_SEED_DIR/
  known_marketplaces.json
  marketplaces/<name>/...
  cache/<marketplace>/<plugin>/<version>/...
```

Um ein Seed-Verzeichnis zu erstellen, führen Sie Claude Code einmal während des Image-Builds aus, installieren Sie die benötigten Plugins, kopieren Sie dann das resultierende `~/.claude/plugins`-Verzeichnis in Ihr Image und verweisen Sie `CLAUDE_CODE_PLUGIN_SEED_DIR` darauf.

Um den Kopierungsschritt zu überspringen, legen Sie `CLAUDE_CODE_PLUGIN_CACHE_DIR` während des Builds auf Ihren Ziel-Seed-Pfad fest, damit Plugins direkt dort installiert werden:

```bash theme={null}
CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin marketplace add your-org/plugins
CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin install my-tool@your-plugins
```

Legen Sie dann `CLAUDE_CODE_PLUGIN_SEED_DIR=/opt/claude-seed` in der Laufzeitumgebung Ihres Containers fest, damit Claude Code beim Start aus dem Seed liest.

Beim Start registriert Claude Code Marktplätze, die in der Seed-Datei `known_marketplaces.json` gefunden werden, in der primären Konfiguration und verwendet Plugin-Caches, die unter `cache/` gefunden werden, ohne erneut zu klonen. Dies funktioniert sowohl im interaktiven Modus als auch im nicht-interaktiven Modus mit dem `-p`-Flag.

Verhaltensdetails:

* **Schreibgeschützt**: Das Seed-Verzeichnis wird nie geschrieben. Auto-Updates sind für Seed-Marktplätze deaktiviert, da git pull auf einem schreibgeschützten Dateisystem fehlschlagen würde.
* **Seed-Einträge haben Vorrang**: Marktplätze, die in der Seed deklariert sind, überschreiben alle übereinstimmenden Einträge in der Benutzerkonfiguration bei jedem Start. Um sich von einem Seed-Plugin abzumelden, verwenden Sie `/plugin disable`, anstatt den Marktplatz zu entfernen.
* **Pfadauflösung**: Claude Code lokalisiert Marktplatz-Inhalte, indem es `$CLAUDE_CODE_PLUGIN_SEED_DIR/marketplaces/<name>/` zur Laufzeit durchsucht, nicht indem es Pfaden vertraut, die in der Seed-JSON gespeichert sind. Dies bedeutet, dass die Seed korrekt funktioniert, auch wenn sie an einem anderen Pfad als dort, wo sie erstellt wurde, bereitgestellt wird.
* **Mutation ist blockiert**: Das Ausführen von `/plugin marketplace remove` oder `/plugin marketplace update` gegen einen Seed-verwalteten Marktplatz schlägt mit Anleitung fehl, um Ihren Administrator zu bitten, das Seed-Image zu aktualisieren.
* **Komponiert mit Einstellungen**: Wenn `extraKnownMarketplaces` oder `enabledPlugins` einen Marktplatz deklarieren, der bereits in der Seed vorhanden ist, verwendet Claude Code die Seed-Kopie, anstatt zu klonen.

<h3 id="managed-marketplace-restrictions">
  Verwaltete Marktplatz-Einschränkungen
</h3>

Für Organisationen, die strikte Kontrolle über Plugin-Quellen benötigen, können Administratoren einschränken, welche Plugin-Marktplätze Benutzer hinzufügen dürfen, indem sie die Einstellung [`strictKnownMarketplaces`](/docs/de/settings#strictknownmarketplaces) in verwalteten Einstellungen verwenden. Um auch die CLI-Flags abzulehnen, die Plugins, Agenten und MCP-Server für einen einzelnen Durchlauf seitenladen, kombinieren Sie es mit [`disableSideloadFlags`](/docs/de/settings#available-settings). Um eine Zulassungsliste zu erstellen, welche Plugins von Marktplätzen als kontextuelle Installationsvorschläge angezeigt werden können, legen Sie [`pluginSuggestionMarketplaces`](/docs/de/settings#available-settings) fest.

Wenn `strictKnownMarketplaces` in verwalteten Einstellungen konfiguriert ist, hängt das Einschränkungsverhalten vom Wert ab:

| Wert                       | Verhalten                                                                                    |
| -------------------------- | -------------------------------------------------------------------------------------------- |
| Nicht definiert (Standard) | Keine Einschränkungen. Benutzer können jeden Marktplatz hinzufügen                           |
| Leeres Array `[]`          | Vollständige Sperrung. Benutzer können keine neuen Marktplätze hinzufügen                    |
| Liste von Quellen          | Benutzer können nur Marktplätze hinzufügen, die genau mit der Zulassungsliste übereinstimmen |

<h4 id="common-configurations">
  Häufige Konfigurationen
</h4>

Deaktivieren Sie alle Marktplatz-Ergänzungen:

```json theme={null}
{
  "strictKnownMarketplaces": []
}
```

Nur bestimmte Marktplätze zulassen:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "github",
      "repo": "acme-corp/approved-plugins"
    },
    {
      "source": "github",
      "repo": "acme-corp/security-tools",
      "ref": "v2.0"
    },
    {
      "source": "url",
      "url": "https://plugins.example.com/marketplace.json"
    }
  ]
}
```

Alle Marktplätze von einem internen Git-Server mit Regex-Musterabgleich auf dem Host zulassen. Dies ist der empfohlene Ansatz für [GitHub Enterprise Server](/docs/de/github-enterprise-server#plugin-marketplaces-on-ghes) oder selbstgehostete GitLab-Instanzen:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "hostPattern",
      "hostPattern": "^github\\.example\\.com$"
    }
  ]
}
```

Dateisystem-basierte Marktplätze aus einem bestimmten Verzeichnis mit Regex-Musterabgleich auf dem Pfad zulassen:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "pathPattern",
      "pathPattern": "^/opt/approved/"
    }
  ]
}
```

Verwenden Sie `".*"` als `pathPattern`, um jeden Dateisystempfad zuzulassen und gleichzeitig Netzwerkquellen mit `hostPattern` zu steuern.

<Note>
  `strictKnownMarketplaces` schränkt ein, was Benutzer hinzufügen können, registriert aber nicht selbst Marktplätze. Um zulässige Marktplätze automatisch verfügbar zu machen, ohne dass Benutzer `/plugin marketplace add` ausführen müssen, kombinieren Sie es mit [`extraKnownMarketplaces`](/docs/de/settings#extraknownmarketplaces) in derselben `managed-settings.json`. Siehe [Beide zusammen verwenden](/docs/de/settings#strictknownmarketplaces).
</Note>

<h4 id="how-restrictions-work">
  Wie Einschränkungen funktionieren
</h4>

Einschränkungen werden überprüft, bevor Netzwerk- oder Dateisystemoperationen durchgeführt werden. Die Überprüfung wird beim Hinzufügen von Marktplätzen und beim Installieren, Aktualisieren, Aktualisieren und Auto-Update von Plugins durchgeführt. Wenn ein Marktplatz hinzugefügt wurde, bevor die Richtlinie konfiguriert wurde, und seine Quelle nicht mehr mit der Zulassungsliste übereinstimmt, weigert sich Claude Code, Plugins daraus zu installieren oder zu aktualisieren. Die gleiche Durchsetzung gilt für `blockedMarketplaces`.

Die Zulassungsliste verwendet exakten Abgleich für die meisten Quellentypen. Damit ein Marktplatz zulässig ist, müssen alle angegebenen Felder genau übereinstimmen:

* Für GitHub-Quellen: `repo` ist erforderlich, und `ref` oder `path` müssen auch übereinstimmen, wenn sie in der Zulassungsliste angegeben sind
* Für URL-Quellen: Die vollständige URL muss genau übereinstimmen
* Für `hostPattern`-Quellen: Der Marktplatz-Host wird gegen das Regex-Muster abgeglichen
* Für `pathPattern`-Quellen: Der Dateisystempfad des Marktplatzes wird gegen das Regex-Muster abgeglichen

Exakter Abgleich normalisiert URLs nicht: Ein nachgestellter Schrägstrich, `.git`-Suffix oder `ssh://` versus `https://`-Form werden als unterschiedliche Werte behandelt. Wenn Ihr Organisations-Marktplatz durch mehr als eine URL-Form geklont werden kann, bevorzugen Sie einen `hostPattern`-Eintrag gegenüber einer literalen URL, damit alle Formen übereinstimmen.

Da `strictKnownMarketplaces` in [verwalteten Einstellungen](/docs/de/settings#settings-files) festgelegt ist, können einzelne Benutzer und Projektkonfigurationen diese Einschränkungen nicht überschreiben.

Für vollständige Konfigurationsdetails einschließlich aller unterstützten Quellentypen und Vergleich mit `extraKnownMarketplaces` siehe die [strictKnownMarketplaces-Referenz](/docs/de/settings#strictknownmarketplaces).

<h3 id="version-resolution-and-release-channels">
  Versionsauflösung und Release-Kanäle
</h3>

Plugin-Versionen bestimmen Cache-Pfade und Update-Erkennung: Wenn die aufgelöste Version mit dem übereinstimmt, was ein Benutzer bereits hat, überspringen `/plugin update` und Auto-Update das Plugin.

Claude Code löst die Version eines Plugins aus dem ersten dieser Punkte auf, der festgelegt ist:

1. `version` in der `plugin.json` des Plugins
2. `version` im Marktplatz-Eintrag des Plugins
3. Der Git-Commit-SHA der Plugin-Quelle

Für die Git-basierten Quellentypen `github`, `url`, `git-subdir` und relative Pfade innerhalb eines Git-gehosteten Marktplatzes können Sie `version` ganz weglassen und jeder neue Commit wird als neue Version behandelt. Dies ist die einfachste Einrichtung für interne oder aktiv entwickelte Plugins.

<Warning>
  Das Festlegen von `version` heftet das Plugin an. Wenn `plugin.json` `"version": "1.0.0"` deklariert, führt das Pushen neuer Commits ohne Änderung dieser Zeichenkette für bestehende Benutzer zu nichts, da Claude Code die gleiche Version sieht und die zwischengespeicherte Kopie behält. Erhöhen Sie das Feld bei jeder Veröffentlichung, oder lassen Sie es weg, um den Commit-SHA zu verwenden.

  Vermeiden Sie das Festlegen von `version` sowohl in `plugin.json` als auch im Marktplatz-Eintrag. Der `plugin.json`-Wert gewinnt immer stillschweigend, daher kann eine veraltete Manifest-Version eine Version maskieren, die Sie in `marketplace.json` festgelegt haben.
</Warning>

<h4 id="set-up-release-channels">
  Richten Sie Release-Kanäle ein
</h4>

Um "stabile" und "neueste" Release-Kanäle für Ihre Plugins zu unterstützen, können Sie zwei Marktplätze einrichten, die auf verschiedene Refs oder SHAs desselben Repos verweisen. Sie können dann die beiden Marktplätze verschiedenen Benutzergruppen über [verwaltete Einstellungen](/docs/de/settings#settings-files) zuweisen.

<Warning>
  Jeder Kanal muss sich zu einer anderen Version auflösen. Wenn Sie explizite Versionen verwenden, muss `plugin.json` eine andere `version` bei jedem angehefteten Ref deklarieren. Wenn Sie `version` weglassen, unterscheiden die unterschiedlichen Commit-SHAs bereits die Kanäle. Wenn zwei Refs sich zu der gleichen Versionskette auflösen, behandelt Claude Code sie als identisch und überspringt das Update.
</Warning>

<h5 id="example">
  Beispiel
</h5>

```json theme={null}
{
  "name": "stable-tools",
  "plugins": [
    {
      "name": "code-formatter",
      "source": {
        "source": "github",
        "repo": "acme-corp/code-formatter",
        "ref": "stable"
      }
    }
  ]
}
```

```json theme={null}
{
  "name": "latest-tools",
  "plugins": [
    {
      "name": "code-formatter",
      "source": {
        "source": "github",
        "repo": "acme-corp/code-formatter",
        "ref": "latest"
      }
    }
  ]
}
```

<h5 id="assign-channels-to-user-groups">
  Kanäle Benutzergruppen zuweisen
</h5>

Weisen Sie jeden Marktplatz der entsprechenden Benutzergruppe über verwaltete Einstellungen zu. Beispielsweise erhält die stabile Gruppe:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "stable-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/stable-tools"
      }
    }
  }
}
```

Die Early-Access-Gruppe erhält stattdessen `latest-tools`:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "latest-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/latest-tools"
      }
    }
  }
}
```

<h4 id="pin-dependency-versions">
  Abhängigkeitsversionen anheften
</h4>

Ein Plugin kann seine Abhängigkeiten auf einen Semver-Bereich beschränken, damit Updates einer Abhängigkeit das abhängige Plugin nicht unterbrechen. Siehe [Plugin-Abhängigkeitsversionen einschränken](/docs/de/plugin-dependencies) für die `{plugin-name}--v{version}` Git-Tag-Konvention, Bereichssyntax und wie mehrere Einschränkungen auf die gleiche Abhängigkeit kombiniert werden.

<h3 id="rename-or-remove-a-plugin">
  Ein Plugin umbenennen oder entfernen
</h3>

Der `name` eines Plugins ist sein stabiler Bezeichner. Benutzer verweisen darauf in `enabledPlugins`, `pluginConfigs` und `/plugin install`-Befehlen, daher bricht das Ändern davon jede vorhandene Installation. Um das in der Benutzeroberfläche angezeigte Label zu ändern, ohne Installationen zu unterbrechen, legen Sie [`displayName`](#optional-plugin-fields) fest und behalten Sie `name` unverändert.

Wenn Sie den `name` eines Plugins ändern müssen oder ein Plugin aus dem `plugins`-Array entfernen, fügen Sie einen Top-Level-`renames`-Eintrag hinzu, damit bestehende Benutzer migrieren, anstatt einen `plugin-not-found`-Fehler zu sehen. Automatische Migration erfordert Claude Code v2.1.193 oder später. Ordnen Sie jeden früheren Namen seinem aktuellen Namen zu, oder zu `null`, wenn das Plugin nicht mehr existiert. Das folgende Beispiel benennt `formatter` in `code-formatter` um und verzeichnet, dass `legacy-linter` entfernt wurde:

```json theme={null}
{
  "name": "acme-tools",
  "owner": { "name": "Acme" },
  "plugins": [
    { "name": "code-formatter", "source": "./plugins/code-formatter" }
  ],
  "renames": {
    "formatter": "code-formatter",
    "legacy-linter": null
  }
}
```

Wenn ein Benutzer Claude Code mit dem alten Namen noch in seinen Einstellungen startet, folgt Claude Code der `renames`-Zuordnung:

* Wenn der Eintrag auf einen neuen Namen verweist, lädt Claude Code das Plugin unter seinem neuen Namen und zeigt eine einzeilige Mitteilung wie `Renamed to "code-formatter" in the "acme-tools" marketplace` an. Es schreibt dann den alten Schlüssel in den neuen Schlüssel in den Benutzer-, Projekt- und lokalen Einstellungsbereichen für sowohl `enabledPlugins` als auch `pluginConfigs` um, sodass die Mitteilung einmal angezeigt wird.
* Für einen `null`-Eintrag löscht Claude Code den alten Schlüssel und die Mitteilung meldet, dass das Plugin aus dem Marktplatz entfernt wurde.
* Wenn das umbenannte Plugin eine Remote-Quelle wie `github` oder `npm` verwendet, meldet Claude Code `plugin-cache-miss` nach der Umbenennung und der Benutzer muss `/plugin install` einmal ausführen, um es unter dem neuen Namen zu holen.

Behandeln Sie `renames` als Nur-Anhängen-Verlauf: Behalten Sie alte Einträge an Ort und Stelle, auch nachdem Sie erwarten, dass jeder Benutzer migriert hat. Claude Code folgt Ketten, daher wenn Sie später `code-formatter` in `formatter-pro` umbenennen, fügen Sie einen zweiten Eintrag hinzu, anstatt den ersten zu bearbeiten. Ein Benutzer, der immer noch das Original `formatter` aktiviert hat, löst sich dann durch beide Einträge zu `formatter-pro` auf.

Führen Sie `claude plugin validate .` nach dem Bearbeiten der Zuordnung aus; es lehnt jeden Eintrag ab, dessen Kette einen Zyklus bildet oder nicht bei `null` oder einem Namen in `plugins` endet.

<Note>
  Verwaltete und Richtlinieneinstellungen sind schreibgeschützt für Claude Code, daher können dort aktivierte Plugins nicht automatisch umgeschrieben werden. Das umbenannte Plugin wird weiterhin jede Sitzung geladen, aber die Umbenennungsmitteilung wiederholt sich, bis ein Administrator `enabledPlugins` in der verwalteten Einstellungsdatei aktualisiert, um den neuen Namen zu verwenden. Das gleiche gilt für Plugins, die über andere schreibgeschützte Quellen wie `--add-dir` aktiviert werden.
</Note>

Frühere Versionen von Claude Code ignorieren das `renames`-Feld und melden `plugin-not-found` für den alten Namen.

<h2 id="validation-and-testing">
  Validierung und Tests
</h2>

Testen Sie Ihren Marktplatz vor dem Teilen.

Validieren Sie Ihre Marktplatz-JSON-Syntax:

```bash theme={null}
claude plugin validate .
```

Oder von innerhalb von Claude Code:

```shell theme={null}
/plugin validate .
```

Fügen Sie den Marktplatz zum Testen hinzu:

```shell theme={null}
/plugin marketplace add ./path/to/marketplace
```

Installieren Sie ein Test-Plugin, um zu überprüfen, ob alles funktioniert:

```shell theme={null}
/plugin install test-plugin@marketplace-name
```

Für vollständige Plugin-Test-Workflows siehe [Testen Sie Ihre Plugins lokal](/docs/de/plugins#test-your-plugins-locally). Für technische Fehlerbehebung siehe [Plugins-Referenz](/docs/de/plugins-reference).

<h2 id="manage-marketplaces-from-the-cli">
  Verwalten Sie Marktplätze über die CLI
</h2>

Claude Code bietet nicht-interaktive `claude plugin marketplace` Unterbefehle zum Scripting und zur Automatisierung. Diese entsprechen den `/plugin marketplace` Befehlen, die in einer interaktiven Sitzung verfügbar sind.

<h3 id="plugin-marketplace-add">
  Plugin marketplace add
</h3>

Fügen Sie einen Marktplatz aus einem GitHub-Repository, einer Git-URL, einer Remote-URL oder einem lokalen Pfad hinzu.

```bash theme={null}
claude plugin marketplace add <source> [options]
```

**Argumente:**

* `<source>`: GitHub `owner/repo` Kurzform, Git-URL, Remote-URL zu einer `marketplace.json`-Datei oder lokaler Verzeichnispfad. Um an einen Branch oder Tag anzuheften, fügen Sie `@ref` zur GitHub-Kurzform oder `#ref` zu einer Git-URL hinzu

Eine URL muss ihr Schema enthalten. Ab Claude Code v2.1.196 wird ein Host, der ohne eines eingegeben wird, wie `gitlab.example.com/team/plugins`, als ungültige `owner/repo` Kurzform abgelehnt und die Fehlermeldung teilt Ihnen mit, dass Sie `https://` hinzufügen oder `./` für einen lokalen Pfad verwenden sollen. Frühere Versionen lasen es als GitHub-Repository-Pfad fehl und schlagen beim Klonen mit einem GitHub-Fehler fehl.

**Optionen:**

| Option                | Beschreibung                                                                                                                                                     | Standard |
| :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------- |
| `--scope <scope>`     | Wo der Marktplatz deklariert werden soll: `user`, `project` oder `local`. Siehe [Plugin-Installationsbereiche](/docs/de/plugins-reference#plugin-installation-scopes) | `user`   |
| `--sparse <paths...>` | Begrenzen Sie den Checkout auf bestimmte Verzeichnisse über git sparse-checkout. Nützlich für Monorepos                                                          |          |

Fügen Sie einen Marktplatz aus GitHub mit `owner/repo` Kurzform hinzu:

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins
```

Heften Sie an einen bestimmten Branch oder Tag mit `@ref` an:

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins@v2.0
```

Fügen Sie von einer Git-URL auf einem nicht-GitHub-Host hinzu:

```bash theme={null}
claude plugin marketplace add https://gitlab.example.com/team/plugins.git
```

Fügen Sie von einer Remote-URL hinzu, die die `marketplace.json`-Datei direkt bereitstellt:

```bash theme={null}
claude plugin marketplace add https://example.com/marketplace.json
```

Fügen Sie von einem lokalen Verzeichnis zum Testen hinzu:

```bash theme={null}
claude plugin marketplace add ./my-marketplace
```

Deklarieren Sie den Marktplatz im Projektbereich, damit er mit Ihrem Team über `.claude/settings.json` geteilt wird:

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins --scope project
```

Für ein Monorepo begrenzen Sie den Checkout auf die Verzeichnisse, die Plugin-Inhalte enthalten:

```bash theme={null}
claude plugin marketplace add acme-corp/monorepo --sparse .claude-plugin plugins
```

<h3 id="plugin-marketplace-list">
  Plugin marketplace list
</h3>

Listet alle konfigurierten Marktplätze auf.

```bash theme={null}
claude plugin marketplace list [options]
```

**Optionen:**

| Option   | Beschreibung     |
| :------- | :--------------- |
| `--json` | Ausgabe als JSON |

Mit `--json` enthält jeder Eintrag `name`, `source` und quellenspezifische Felder: `repo` für GitHub-Quellen, `url` für Git- und URL-Quellen und `path` für lokale Quellen. GitHub- und Git-Quellen enthalten auch ein `ref`-Feld, wenn der Marktplatz mit einem angehefteten Branch oder Tag hinzugefügt wurde.

<h3 id="plugin-marketplace-remove">
  Plugin marketplace remove
</h3>

Entfernen Sie einen konfigurierten Marktplatz. Der Alias `rm` wird auch akzeptiert.

```bash theme={null}
claude plugin marketplace remove <name> [options]
```

**Argumente:**

* `<name>`: Marktplatz-Name zum Entfernen, wie von `claude plugin marketplace list` angezeigt. Dies ist der `name` aus `marketplace.json`, nicht die Quelle, die Sie an `add` übergeben haben

**Optionen:**

| Option            | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Standard        |
| :---------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------- |
| `--scope <scope>` | Beschränken Sie die Entfernung auf einen einzelnen Einstellungsbereich: `user`, `project` oder `local`. Siehe [Plugin-Installationsbereiche](/docs/de/plugins-reference#plugin-installation-scopes). Wenn weggelassen, wird die Deklaration aus jedem bearbeitbaren Bereich entfernt. Wenn angegeben, wird nur die Deklaration dieses Bereichs entfernt; der gemeinsame Status, der Cache und die installierten Plugin-Daten werden beibehalten, wenn der Marktplatz noch in einem anderen Bereich deklariert ist | (alle Bereiche) |

<Warning>
  Das Entfernen eines Marktplatzes aus seinem letzten verbleibenden Bereich deinstalliert auch alle Plugins, die Sie von ihm installiert haben. Um einen Marktplatz zu aktualisieren, ohne installierte Plugins zu verlieren, verwenden Sie stattdessen `claude plugin marketplace update`.
</Warning>

<h3 id="plugin-marketplace-update">
  Plugin marketplace update
</h3>

Aktualisieren Sie Marktplätze von ihren Quellen, um neue Plugins und Versionsänderungen abzurufen. Ein Marktplatz, der mit einem Branch oder Tag `ref` hinzugefügt wurde, wird auf den neuesten Commit dieses Refs aktualisiert, nicht auf den Standard-Branch des Repositorys.

```bash theme={null}
claude plugin marketplace update [name]
```

**Argumente:**

* `[name]`: Marktplatz-Name zum Aktualisieren, wie von `claude plugin marketplace list` angezeigt. Aktualisiert alle Marktplätze, wenn weggelassen

Sowohl `remove` als auch `update` schlagen fehl, wenn sie gegen einen Seed-verwalteten Marktplatz ausgeführt werden, der schreibgeschützt ist. Beim Aktualisieren aller Marktplätze werden Seed-verwaltete Einträge übersprungen und andere Marktplätze werden weiterhin aktualisiert. Um Seed-bereitgestellte Plugins zu ändern, bitten Sie Ihren Administrator, das Seed-Image zu aktualisieren. Siehe [Plugins für Container vorab ausfüllen](#pre-populate-plugins-for-containers).

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="marketplace-not-loading">
  Marktplatz wird nicht geladen
</h3>

**Symptome**: Kann Marktplatz nicht hinzufügen oder Plugins von ihm nicht sehen

**Lösungen**:

* Überprüfen Sie, dass die Marktplatz-URL erreichbar ist
* Überprüfen Sie, dass `.claude-plugin/marketplace.json` im angegebenen Pfad vorhanden ist
* Stellen Sie sicher, dass die JSON-Syntax gültig ist, indem Sie `claude plugin validate` oder `/plugin validate` verwenden. Um Skill-, Agent- und Befehl-Frontmatter zu überprüfen, führen Sie den Befehl für jedes Plugin-Verzeichnis aus
* Bestätigen Sie für private Repositories, dass Sie Zugriffsberechtigung haben

<h3 id="marketplace-validation-errors">
  Marktplatz-Validierungsfehler
</h3>

Führen Sie `claude plugin validate .` oder `/plugin validate .` aus Ihrem Marktplatz-Verzeichnis aus, um auf Probleme zu überprüfen. Wenn der Validator auf ein Marktplatz-Verzeichnis verweist, überprüft er `marketplace.json` auf Schema-Fehler, doppelte Plugin-Namen und Quellpfad-Traversal. Für jeden Eintrag, dessen `source` ein lokaler Pfad ist, validiert er auch die `plugin.json` dieses Plugins und warnt, wenn die `version` des Eintrags nicht mit der in `plugin.json` übereinstimmt. Probleme, die in der `plugin.json` eines Plugins gefunden werden, werden mit dem Eintrag-Index in der Form `plugins[2] plugin.json →` vorangestellt.

Ab Claude Code v2.1.196 umfasst die Pro-Eintrag-Überprüfung auch:

* Plugins, deren `source` `.` ist
* wird ausgeführt, wenn `marketplace.json` außerhalb eines `.claude-plugin`-Verzeichnisses liegt, wobei Quellen gegen das Verzeichnis der Datei selbst aufgelöst werden
* meldet die Probleme jedes Eintrags, auch wenn ein anderer Teil der Datei Schema-Fehler hat

Frühere Versionen überspringen Plugins im Marktplatz-Root und steigen nur von einer `.claude-plugin/marketplace.json` ab.

Um die `plugin.json` eines einzelnen Plugins und seine Skill-, Agent-, Befehl- und Hook-Dateien zu validieren, führen Sie den Befehl für das Plugin-Verzeichnis selbst aus, zum Beispiel `claude plugin validate ./plugins/my-plugin`. Häufige Fehler:

| Fehler                                            | Ursache                                                   | Lösung                                                                                                                                                                             |
| :------------------------------------------------ | :-------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `File not found: .claude-plugin/marketplace.json` | Fehlendes Manifest                                        | Erstellen Sie `.claude-plugin/marketplace.json` mit erforderlichen Feldern                                                                                                         |
| `Invalid JSON syntax: Unexpected token...`        | JSON-Syntaxfehler in marketplace.json                     | Überprüfen Sie auf fehlende Kommas, zusätzliche Kommas oder nicht zitierte Strings                                                                                                 |
| `Duplicate plugin name "x" found in marketplace`  | Zwei Plugins teilen denselben Namen                       | Geben Sie jedem Plugin einen eindeutigen `name`-Wert                                                                                                                               |
| `plugins[0].source: Path contains ".."`           | Quellpfad enthält `..`                                    | Verwenden Sie Pfade relativ zum Marktplatz-Root ohne `..`. Siehe [Relative Pfade](#relative-paths)                                                                                 |
| `YAML frontmatter failed to parse: ...`           | Ungültiges YAML in einer Skill-, Agent- oder Befehlsdatei | Beheben Sie die YAML-Syntax im Frontmatter-Block. Zur Laufzeit wird diese Datei ohne Metadaten geladen. Wird nur bei der Validierung eines Plugin-Verzeichnisses gemeldet          |
| `Invalid JSON syntax: ...` (hooks.json)           | Malformed `hooks/hooks.json`                              | Beheben Sie die JSON-Syntax. Eine malformed `hooks/hooks.json` verhindert, dass das gesamte Plugin geladen wird. Wird nur bei der Validierung eines Plugin-Verzeichnisses gemeldet |

**Warnungen** (nicht blockierend):

* `Marketplace has no plugins defined`: Fügen Sie mindestens ein Plugin zum `plugins`-Array hinzu
* `No marketplace description provided`: Fügen Sie eine Top-Level-`description` hinzu, um Benutzern zu helfen, Ihren Marktplatz zu verstehen
* `Plugin name "x" is not kebab-case`: Der Plugin-Name enthält Großbuchstaben, Leerzeichen oder Sonderzeichen. Benennen Sie in Kleinbuchstaben, Ziffern und Bindestriche um (z. B. `my-plugin`). Claude Code akzeptiert andere Formen, aber die Claude.ai-Marktplatz-Synchronisierung lehnt sie ab.

<h3 id="plugin-installation-failures">
  Plugin-Installationsfehler
</h3>

**Symptome**: Marktplatz wird angezeigt, aber Plugin-Installation schlägt fehl

**Lösungen**:

* Überprüfen Sie, dass Plugin-Quell-URLs erreichbar sind
* Überprüfen Sie, dass Plugin-Verzeichnisse erforderliche Dateien enthalten
* Überprüfen Sie für GitHub-Quellen, dass Repositories öffentlich sind oder Sie Zugriff haben
* Testen Sie Plugin-Quellen manuell durch Klonen/Herunterladen
* Wenn die Quelle sowohl `ref` als auch `sha` festlegt, blockiert ein gelöschter Upstream-Branch oder Tag die Installation nicht auf den meisten Git-Hosts, einschließlich GitHub, GitLab und Bitbucket. Auf Servern, die das Abrufen von Commits nach SHA nicht unterstützen, wie AWS CodeCommit, muss die `ref` immer noch vorhanden sein und der angeheftete Commit muss von ihr erreichbar sein. Wenn die Installation immer noch fehlschlägt, bestätigen Sie, dass der angeheftete Commit immer noch im Repository vorhanden ist

<h3 id="private-repository-authentication-fails">
  Authentifizierung für private Repositories schlägt fehl
</h3>

**Symptome**: Authentifizierungsfehler beim Installieren von Plugins aus privaten Repositories

**Lösungen**:

Für manuelle Installation und Updates:

* Überprüfen Sie, dass Sie bei Ihrem Git-Anbieter authentifiziert sind (führen Sie z. B. `gh auth status` für GitHub aus)
* Überprüfen Sie, dass Ihr Credential-Helper korrekt konfiguriert ist: `git config --global credential.helper`
* Versuchen Sie, das Repository manuell zu klonen, um zu überprüfen, dass Ihre Anmeldedaten funktionieren

Für Hintergrund-Auto-Updates:

* Standardmäßig deaktivieren Hintergrund-Aktualisierungen Git-Credential-Helper für den Pull, sodass der Pull nicht über HTTPS authentifizieren kann. SSH-Remotes mit einem in `ssh-agent` geladenen Schlüssel authentifizieren sich immer noch. Ein fehlgeschlagener Pull löst ein erneutes Klonen von Grund auf aus, das Ihre gespeicherten Anmeldedaten verwendet, aber bei großen Repositories möglicherweise zeitüberschreitet
* Legen Sie `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1` fest, um den vorhandenen Klon beizubehalten, wenn der Hintergrund-Pull fehlschlägt
* Konfigurieren Sie einen Git-Credential-Helper, z. B. `gh auth setup-git`, damit das Fallback-Neuklon authentifizieren kann
* Wenn das Neuklon bei einem großen Repository zeitüberschreitet, erhöhen Sie das Limit mit [`CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`](#git-operations-time-out)
* Konfigurieren Sie ein [Git-URL-Rewrite](#private-repositories) mit Bereich auf das Marktplatz-Repository, damit der Hintergrund-Pull direkt authentifiziert
* Oder aktualisieren Sie private Marktplätze manuell mit `/plugin marketplace update <name>`, das Ihre Anmeldedaten verwendet

<h3 id="marketplace-updates-fail-in-offline-environments">
  Marktplatz-Updates schlagen in Offline-Umgebungen fehl
</h3>

**Symptome**: Marktplatz `git pull` schlägt im Hintergrund fehl und Claude Code versucht wiederholt ein erneutes Klonen, das nicht erfolgreich sein kann.

**Ursache**: Standardmäßig versucht Claude Code ein erneutes Klonen von Grund auf, wenn ein `git pull` fehlschlägt. In Offline- oder Airgapped-Umgebungen schlägt das erneute Klonen auf die gleiche Weise fehl, und die Wiederherstellung des vorherigen Cache danach ist Best-Effort. Die Aktualisierung wird im Hintergrund nach dem Start ausgeführt, sodass sie den Start nicht verzögert, aber jede Sitzung wiederholt die fehlgeschlagenen Versuche und jede Git-Operation kann das [120-Sekunden-Timeout](#git-operations-time-out) abwarten.

**Lösung**: Legen Sie `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1` fest, um den Neuklon-Versuch zu überspringen und den vorhandenen Cache beizubehalten, wenn der Pull fehlschlägt:

```bash theme={null}
export CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1
```

Mit dieser Variable gesetzt behält Claude Code den veralteten Marktplatz-Klon bei `git pull`-Fehler bei und verwendet weiterhin den letzten bekannten guten Status. Verwenden Sie für vollständig Offline-Bereitstellungen, bei denen das Repository nie erreichbar sein wird, stattdessen [`CLAUDE_CODE_PLUGIN_SEED_DIR`](#pre-populate-plugins-for-containers), um das Plugins-Verzeichnis zur Build-Zeit vorab auszufüllen.

<h3 id="git-operations-time-out">
  Git-Operationen zeitüberschreitung
</h3>

**Symptome**: Plugin-Installation oder Marktplatz-Updates schlagen mit einem Timeout-Fehler fehl, wie "Git clone timed out after 120s" oder "Git pull timed out after 120s".

**Ursache**: Claude Code verwendet ein 120-Sekunden-Timeout für alle Git-Operationen, einschließlich Klonen von Plugin-Repositories und Abrufen von Marktplatz-Updates. Große Repositories oder langsame Netzwerkverbindungen können dieses Limit überschreiten.

**Lösung**: Erhöhen Sie das Timeout mit der Umgebungsvariable `CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`. Der Wert ist in Millisekunden:

```bash theme={null}
export CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS=300000  # 5 Minuten
```

<h3 id="plugins-with-relative-paths-fail-in-url-based-marketplaces">
  Plugins mit relativen Pfaden schlagen in URL-basierten Marktplätzen fehl
</h3>

**Symptome**: Einen Marktplatz über URL hinzugefügt (z. B. `https://example.com/marketplace.json`), aber Plugins mit relativen Pfadquellen wie `"./plugins/my-plugin"` schlagen mit "path not found"-Fehlern fehl.

**Ursache**: URL-basierte Marktplätze laden nur die `marketplace.json`-Datei selbst herunter. Sie laden keine Plugin-Dateien vom Server herunter. Relative Pfade im Marktplatz-Eintrag verweisen auf Dateien auf dem Remote-Server, die nicht heruntergeladen wurden.

**Lösungen**:

* **Verwenden Sie externe Quellen**: Ändern Sie Plugin-Einträge, um stattdessen GitHub-, npm- oder Git-URL-Quellen zu verwenden:
  ```json theme={null}
  { "name": "my-plugin", "source": { "source": "github", "repo": "owner/repo" } }
  ```
* **Verwenden Sie einen Git-basierten Marktplatz**: Hosten Sie Ihren Marktplatz in einem Git-Repository und fügen Sie ihn mit der Git-URL hinzu. Git-basierte Marktplätze klonen das gesamte Repository, wodurch relative Pfade funktionieren.

<h3 id="files-not-found-after-installation">
  Dateien nicht gefunden nach Installation
</h3>

**Symptome**: Plugin wird installiert, aber Verweise auf Dateien schlagen fehl, besonders Dateien außerhalb des Plugin-Verzeichnisses

**Ursache**: Plugins werden in ein Cache-Verzeichnis kopiert, anstatt an Ort und Stelle verwendet zu werden. Pfade, die auf Dateien außerhalb des Plugin-Verzeichnisses verweisen (wie `../shared-utils`), funktionieren nicht, da diese Dateien nicht kopiert werden.

**Lösungen**: Siehe [Plugin-Caching und Dateiauflösung](/docs/de/plugins-reference#plugin-caching-and-file-resolution) für Workarounds, einschließlich Symlinks und Verzeichnisumstrukturierung.

Für zusätzliche Debugging-Tools und häufige Probleme siehe [Debugging- und Entwicklungstools](/docs/de/plugins-reference#debugging-and-development-tools).

<h2 id="see-also">
  Siehe auch
</h2>

* [Entdecken und Installieren vorgefertigter Plugins](/docs/de/discover-plugins) - Installieren von Plugins aus vorhandenen Marktplätzen
* [Plugins](/docs/de/plugins) - Erstellen Ihrer eigenen Plugins
* [Plugins-Referenz](/docs/de/plugins-reference) - Vollständige technische Spezifikationen und Schemas
* [Plugin-Einstellungen](/docs/de/settings#plugin-settings) - Plugin-Konfigurationsoptionen
* [strictKnownMarketplaces-Referenz](/docs/de/settings#strictknownmarketplaces) - Verwaltete Marktplatz-Einschränkungen
