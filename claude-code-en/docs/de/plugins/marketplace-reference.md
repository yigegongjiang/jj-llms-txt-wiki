> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Marketplace-Referenz

> Vollständige Referenz für marketplace.json-Felder, Plugin-Einträge und die Plugin- und Marketplace-Quellobjekte mit Angabe ihrer Gültigkeitsbereiche.

`marketplace.json` ist die Datei, die einen Plugin-Marketplace definiert. Sie enthält den Namen des Marketplace, seinen Besitzer und einen Eintrag pro Plugin. Die Plugin-Quelle jedes Eintrags gibt an, woher Claude Code dieses Plugin abruft.

Eine Marketplace-Quelle ist ein separates Objekt, das angibt, woher Claude Code die Marketplace-Datei selbst abruft. Sie schreiben eine in den Einstellungen, oder Claude Code erstellt eine, wenn Sie `claude plugin marketplace add` ausführen.

Diese Referenz ist für Marketplace-Betreuer, die einen genauen Feldnamen oder -wert benötigen, und für Administratoren, die wissen müssen, welche `source`-Werte in [`extraKnownMarketplaces`](/docs/de/settings-reference#extraknownmarketplaces), [`strictKnownMarketplaces`](/docs/de/settings-reference#strictknownmarketplaces) und [`blockedMarketplaces`](/docs/de/plugins/org#restrict-what-users-can-install) gültig sind.

<Note>
  Diese Fälle werden auf anderen Seiten behandelt:

  * **Erstellen oder Hosten eines Marketplace**: siehe [Marketplace erstellen](/docs/de/plugins/create-marketplace) und [Marketplace hosten und verwalten](/docs/de/plugins/host-marketplace)
  * **Allowlist- und Blocklist-Rezepte**: siehe [Plugins für Ihre Organisation verwalten](/docs/de/plugins/org)
</Note>

Finden Sie den Abschnitt für das, was Sie schreiben oder lesen:

* **Die Marketplace-Datei**: [Top-Level-Felder](#top-level-fields) und [Plugin-Einträge](#plugin-entries)
* **Die `source` eines Eintrags**: [Plugin-Quellen](#plugin-sources)
* **Ein `source`-Objekt in Einstellungen**: [Marketplace-Quellen](#marketplace-sources)
* **Ausgabe von [`claude plugin validate <path>`](/docs/de/plugins/cli-reference)**: [Validierungsmeldungen](#validation-messages), die jede Meldung dem Feld zuordnet, das sie benennt

<h2 id="marketplace-file">
  Marketplace-Datei
</h2>

Speichern Sie die Marketplace-Datei unter `.claude-plugin/marketplace.json` im Verzeichnis Ihres Marketplace. Wenn Sie die Datei an einer anderen Stelle im Repository speichern, müssen Benutzer den Marketplace in [`extraKnownMarketplaces`](/docs/de/settings-reference#extraknownmarketplaces) mit `path` in seiner Quelle deklarieren, da `claude plugin marketplace add` keine Option dafür hat.

Das Verzeichnis, das `.claude-plugin/` enthält, wird als Marketplace-Root bezeichnet, und jede relative Plugin-Quelle wird von dort aus aufgelöst, nicht von `.claude-plugin/`.

Jeder Benutzer registriert einen Marketplace pro `name`, daher kann ein Benutzer nicht zwei Marketplaces mit demselben Namen gleichzeitig registriert haben.

Claude Code ignoriert einen unbekannten Top-Level-Schlüssel oder Plugin-Eintrag-Schlüssel, anstatt ihn abzulehnen, daher wird ein Tippfehler stillschweigend geladen. `claude plugin validate` meldet jeden unbekannten Schlüssel als Warnung.

<h3 id="reserved-names">
  Reservierte Namen
</h3>

Sie können Ihrem Marketplace keinen der folgenden Namen geben:

* **Offizielle Marketplace-Namen**: `claude-code-marketplace`, `claude-code-plugins`, `claude-plugins-official`, `anthropic-marketplace`, `anthropic-plugins`, `agent-skills`, `anthropic-agent-skills`, `life-sciences`, `knowledge-work-plugins`, `claude-for-legal`, `claude-for-financial-services`, `financial-services-plugins`, `first-party-plugins` und `claude-tag-plugins`. Reserviert, es sei denn, der Marketplace stammt von einer `github`- oder `git`-[Marketplace-Quelle](#marketplace-sources) unter `github.com/anthropics/`.
* **Community-Marketplace-Namen**: `claude-community`, `claude-plugins-community` und `healthcare`. Reserviert nach der gleichen Regel wie die offiziellen Namen.
* **Plugin-Verzeichnisnamen**: `anthropic-plugin-directory` und `claude-plugin-directory`. Reserviert nach der gleichen Regel wie die offiziellen Namen.
* **Namen, die einen offiziellen Marketplace imitieren**: Namen wie `official-claude-plugins` oder `claude-plugins-v2` und jeder Name, der ein Nicht-ASCII-Zeichen enthält. Der Fehler ist `Marketplace name impersonates an official Anthropic/Claude marketplace`. Ein Steuerzeichen oder bidirektionales Formatierungszeichen in einem Namen meldet auch `Marketplace name cannot contain control or bidirectional-formatting characters`.
* <span id="reserved-name-spellings" />**Eine andere Schreibweise eines reservierten Namens**: ein Name, der sich von einem reservierten Namen nur durch einen nachgestellten Punkt oder durch ein Symbol anstelle eines Bindestrichs unterscheidet, daher zählt `claude.code.plugins` als `claude-code-plugins`. `claude plugin validate` akzeptiert einen solchen Namen; das Hinzufügen des Marketplace schlägt mit [`is another spelling of "<reserved>", a reserved marketplace name`](/docs/de/errors#marketplace-name-is-another-spelling-of-a-reserved-name) fehl, und ein bereits unter einem registrierter Marketplace wird nicht mehr geladen. Diese Überprüfung erfordert Claude Code v2.1.280 oder später.
* **Namen, die Claude Code für Plugins verwendet, die nicht von einem Marketplace stammen**: `inline` für Plugins, die mit [`--plugin-dir`](/docs/de/cli-reference) geladen werden, `builtin` für integrierte Plugins, `skills-dir` für Plugins, die automatisch von [`.claude/skills/`](/docs/de/skills) geladen werden, und `synced` für Plugins, die von Ihrem claude.ai-Konto synchronisiert werden. `claude-plugin-test` ist ebenfalls reserviert. `skills-dir` erscheint auch als `{"source": "skills-dir"}` in `strictKnownMarketplaces` und `blockedMarketplaces`, beschrieben unter [Quellwerte, die nur in Richtlinienlisten gültig sind](#source-values-valid-only-in-policy-lists).
* **`npm`, `pip`, `uv`, `cargo`, `github` und `gh`**: reserviert in jeder Schreibweise. Diese Überprüfung erfordert Claude Code v2.1.275 oder später.
* **Namen, die mit `claudeai-` beginnen**: reserviert für Marketplaces, die auf claude.ai gehostet werden. `claude plugin marketplace add` lehnt jeden anderen Marketplace ab, der einen mit `Cannot add marketplace "<name>": names starting with "claudeai-" are reserved for marketplaces hosted on claude.ai` verwendet.

<h2 id="top-level-fields">
  Top-Level-Felder
</h2>

Die Tabelle listet jeden Schlüssel auf, den Claude Code aus `marketplace.json` liest. `name`, `owner` und `plugins` sind erforderlich.

| Feld                                       | Typ              | Beschreibung                                                                                                                                                                                                                                                                                           |
| :----------------------------------------- | :--------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                                     | string           | Marketplace-Bezeichner. Keine Leerzeichen, Steuerzeichen oder bidirektionalen Formatierungszeichen, kein `/` oder `\`, kein `..` und nicht `.`. Siehe [Reservierte Namen](#reserved-names). Benutzer geben ihn nach `@` ein, wenn sie ein Plugin installieren                                          |
| `owner`                                    | object           | Informationen zum Betreuer. `name` ist erforderlich; `email` und `url` sind optional                                                                                                                                                                                                                   |
| `plugins`                                  | array            | [Plugin-Einträge](#plugin-entries). Jeder Eintrag wird einzeln validiert, daher führt ein ungültiger Eintrag nicht zum Fehlschlag des Marketplace                                                                                                                                                      |
| `$schema`                                  | string           | JSON-Schema-URL für Editor-Autovervollständigung. Wird beim Laden ignoriert                                                                                                                                                                                                                            |
| `description`                              | string           | Marketplace-Beschreibung, die Benutzern angezeigt wird. `claude plugin validate` warnt, wenn sie fehlt                                                                                                                                                                                                 |
| `version`                                  | string           | Marketplace-Manifest-Version                                                                                                                                                                                                                                                                           |
| `metadata.description`, `metadata.version` | string           | Alternativer Speicherort für `description` und `version`                                                                                                                                                                                                                                               |
| `metadata.pluginRoot`                      | string           | Verzeichnis, unter dem sich bare Plugin-Quellnamen auflösen. Siehe [Relative Pfad-Plugin-Quelle](#relative-path-plugin-source). Erfordert Claude Code v2.1.239 oder später                                                                                                                             |
| `forceRemoveDeletedPlugins`                | boolean          | Wenn `true`, wird ein Plugin, das Sie aus `plugins` entfernen, auf den Maschinen der Benutzer deinstalliert. Siehe [Marketplace hosten und verwalten](/docs/de/plugins/host-marketplace)                                                                                                                    |
| `allowCrossMarketplaceDependenciesOn`      | array of strings | Marketplace-Namen, deren Plugins als Abhängigkeiten von Plugins dieses Marketplace installiert werden dürfen. Wenn Sie ein Plugin installieren, gilt nur die Liste im eigenen Marketplace dieses Plugins für seine gesamte Abhängigkeitskette. Siehe [Plugin-Abhängigkeiten](/docs/de/plugins/dependencies) |
| `renames`                                  | object           | Zuordnung von einem früheren Plugin-`name` zu seinem aktuellen Namen oder zu `null` für ein Plugin, das Sie entfernt haben. Erfordert Claude Code v2.1.193 oder später. Siehe [Marketplace hosten und verwalten](/docs/de/plugins/host-marketplace)                                                         |

<h2 id="plugin-entries">
  Plugin-Einträge
</h2>

Jedes Objekt im Top-Level-Array `plugins` von `marketplace.json` benennt ein Plugin und gibt an, woher es abgerufen werden soll. `name` und `source` sind erforderlich.

Ein Eintrag akzeptiert auch jedes [`plugin.json`-Feld](/docs/de/plugins/manifest-reference), wie `description`, `version`, `author`, `commands` und `hooks`. Für den Fall, dass diese Felder gelten, siehe [Wie ein Eintrag mit plugin.json kombiniert wird](#entry-and-plugin-json).

Die Tabelle listet die eigenen Felder des Eintrags und die Manifest-Felder auf, deren Bedeutung sich in einem Eintrag ändert.

| Feld             | Typ              | Beschreibung                                                                                                                                                                                                                                                                                                                                     |
| :--------------- | :--------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`           | string           | Plugin-Bezeichner ohne Leerzeichen, Steuerzeichen oder bidirektionale Formatierungszeichen. Benutzer geben ihn vor `@` ein, wenn sie installieren, auch wenn die eigene `plugin.json` des Plugins einen anderen `name` setzt                                                                                                                     |
| `source`         | string or object | Woher das Plugin abgerufen werden soll. Siehe [Plugin-Quellen](#plugin-sources)                                                                                                                                                                                                                                                                  |
| `description`    | string           | Wird in [`/plugin`](/docs/de/plugins/install)-Auflistungen und Details angezeigt                                                                                                                                                                                                                                                                      |
| `version`        | string           | Versionszeichenfolge für das Plugin. Wenn `plugin.json` auch `version` setzt, hat `plugin.json` Vorrang und `claude plugin validate` warnt. Siehe [Plugin-Lade-Referenz](/docs/de/plugins/loading)                                                                                                                                                    |
| `category`       | string           | Freie Kategorie zur Organisation des Katalogs                                                                                                                                                                                                                                                                                                    |
| `tags`           | array of strings | Freie Tags für die Suche                                                                                                                                                                                                                                                                                                                         |
| `strict`         | boolean          | Standard `true`. Ob `plugin.json` die definitive Quelle für die Komponenten des Plugins ist. Siehe [Strict-Modus](#strict-mode)                                                                                                                                                                                                                  |
| `relevance`      | object           | Signale, die Claude Code mitteilen, wann das Plugin vorgeschlagen werden soll. Siehe [Plugins für Ihre Organisation empfehlen](/docs/de/plugins/relevance)                                                                                                                                                                                            |
| `dependencies`   | array            | Plugins, die für dieses aktiviert sein müssen. Jedes Element ist `"name"`, `"name@marketplace"` oder ein Objekt. Siehe [Plugin-Abhängigkeiten](/docs/de/plugins/dependencies)                                                                                                                                                                         |
| `defaultEnabled` | boolean          | Standard `true`. Ob das Plugin aktiviert startet, wenn der Benutzer es nicht in [`enabledPlugins`](/docs/de/settings-reference#enabledplugins) gesetzt hat. Der Eintragswert hat Vorrang vor `plugin.json`                                                                                                                                            |
| `displayName`    | string           | Benutzerfreundlicher Name, der in der Benutzeroberfläche angezeigt wird. Wenn weder der Eintrag noch die `plugin.json` des Plugins einen setzt, sehen Benutzer den `name` des Plugins                                                                                                                                                            |
| `metadata`       | object           | Freies Objekt für Ihre eigenen Felder. Claude Code liest es nicht. Erfordert Claude Code v2.1.222 oder später                                                                                                                                                                                                                                    |
| `headers`        | object           | HTTP-Header, die Claude Code sendet, wenn es diesen Eintrag [Archiv](#archive-plugin-source) herunterlädt. Ein hier gesetzter Header ersetzt einen Header mit demselben Namen aus der [`headers`](#fields-by-type) der Marketplace-Quelle. Erfordert Claude Code v2.1.238 oder später                                                            |
| `headersHelper`  | string           | Befehl, der die Archive-Download-Header dieses Eintrags als ein JSON-Objekt ausgibt, für eine Anmeldedaten, die abläuft. Der Eintrag muss auch [`"strict": false`](#strict-mode) setzen. Erfordert Claude Code v2.1.238 oder später. Siehe [Authentifizieren Sie Archive-Downloads](/docs/de/plugins/host-marketplace#authenticate-archive-downloads) |

<h3 id="entry-and-plugin-json">
  Wie ein Eintrag mit plugin.json kombiniert wird
</h3>

Die Felder des Eintrags gelten unterschiedlich für ein abgerufenes Plugin, das seine eigene `.claude-plugin/plugin.json` hat, und für eines, das nicht:

* **Keine `plugin.json`**: Der Eintrag ist das Manifest unabhängig von `strict`. Jedes Manifest-Feld im Eintrag gilt, einschließlich [`mcpServers`, `lspServers`, `userConfig` und `channels`](/docs/de/plugins/manifest-reference).
* **`plugin.json` vorhanden**: `plugin.json` ist das Manifest. Der [Strict-Modus](#strict-mode) entscheidet, ob die sechs Komponentenfelder des Eintrags, `commands`, `agents`, `skills`, `hooks`, `outputStyles` und `themes`, damit kombiniert oder als Konflikt abgelehnt werden. Eintrag `mcpServers`, `lspServers`, `userConfig` und `channels` gelten nicht. Deklarieren Sie sie in `plugin.json`.

<h4 id="hooks-in-an-entry">
  Hooks in einem Eintrag
</h4>

Schreiben Sie Eintrag `hooks` als ein Inline-Objekt, das Hook-Ereignisnamen auf Matcher-Arrays abbildet. Wenn Sie einen Dateipfad oder ein Array schreiben, besteht `claude plugin validate`. Diese Hooks werden nie ausgeführt, und Claude Code meldet einen `not yet supported in a marketplace entry`-Fehler für das Plugin. Legen Sie dateibasierte Hooks in die [`hooks/hooks.json`](/docs/de/plugins/components) oder `plugin.json` des Plugins selbst.

<h4 id="display-fields">
  Anzeigefelder
</h4>

Sowohl der Eintrag als auch die eigene `plugin.json` des Plugins können die Anzeigefelder `displayName`, `description`, `author`, `homepage`, `repository`, `license` und `keywords` setzen. Benutzer sehen diese Werte in Plugin-Auflistungen und Details vor und nach der Installation:

* Für ein Feld, das Sie auf dem Eintrag setzen, sehen Benutzer den Wert des Eintrags, auch wenn `plugin.json` einen anderen setzt.
* Für ein Feld, das der Eintrag nicht setzt, sehen Benutzer den `plugin.json`-Wert.

Vor der Installation kann Claude Code `plugin.json` nur für Einträge mit einer [relativen Pfad-Quelle](#relative-path-plugin-source) lesen, deren Plugin-Dateien sich im Marketplace selbst befinden. Für einen Eintrag mit einem anderen Quelltyp sehen Benutzer nur die eigenen Felder des Eintrags, bis sie das Plugin installieren.

<h3 id="strict-mode">
  Strict-Modus
</h3>

`strict` entscheidet, was passiert, wenn das abgerufene Plugin seine eigene `plugin.json` hat und der Eintrag auch eines der [Komponentenfelder](#entry-and-plugin-json) deklariert: `commands`, `agents`, `skills`, `hooks`, `outputStyles` oder `themes`. Mit `strict: true`, dem Standard, hängt Claude Code die Komponentenfelder des Eintrags an `plugin.json` an, außer `hooks`, dessen Matcher die des Manifest pro Ereignis ersetzen. Mit `strict: false` ist ein Eintrag, der ein Komponentenfeld deklariert, ein Konflikt, und das Plugin wird nicht geladen. Die Tabelle zeigt jede Kombination von `strict`, `plugin.json` und den Komponentenfeldern des Eintrags.

| `strict`             | `plugin.json` | Komponentenfelder des Eintrags | Ergebnis                                                                                                                                                                                                                                        |
| :------------------- | :------------ | :----------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| any                  | absent        | any                            | Der Eintrag ist das Manifest                                                                                                                                                                                                                    |
| `true`, der Standard | present       | any                            | `plugin.json` ist die Autorität. Claude Code hängt die Komponentenfelder des Eintrags daran an, außer `hooks`, deren Matcher [die des Manifest pro Ereignis ersetzen](/docs/de/plugins/manifest-reference#how-entry-fields-combine-with-plugin-json) |
| `false`              | present       | none                           | `plugin.json` ist das Manifest, wie mit `true`                                                                                                                                                                                                  |
| `false`              | present       | one or more                    | Konflikt. Das Plugin wird nicht geladen mit `Plugin <name> has conflicting manifests: both plugin.json and marketplace entry specify components`                                                                                                |

<h2 id="plugin-sources">
  Plugin-Quellen
</h2>

Die `source` eines Plugin-Eintrags gibt an, woher Claude Code dieses eine Plugin abruft. Es ist entweder eine relative Pfad-Zeichenfolge oder ein Objekt, dessen eigener `source`-Schlüssel den Typ benennt, daher sieht ein Eintrag wie `"source": { "source": "github", "repo": "your-org/formatter" }` aus.

Die Tabelle listet jeden Plugin-Quelltyp und seine Felder auf.

| Typ            | Felder                           | Notizen                                                                                                                                                                                                                               |
| :------------- | :------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Relativer Pfad | die Zeichenfolge selbst          | Ein Verzeichnis im Marketplace, aufgelöst vom Marketplace-Root. Muss mit `./` beginnen, es sei denn, Sie schreiben einen [bare Name unter `metadata.pluginRoot`](#relative-path-plugin-source). `"."` allein bedeutet den Root selbst |
| `github`       | `repo`, `ref`, `sha`             | GitHub-Repository in `owner/repo`-Form                                                                                                                                                                                                |
| `url`          | `url`, `ref`, `sha`              | Jedes Git-Repository nach URL                                                                                                                                                                                                         |
| `git-subdir`   | `url`, `path`, `ref`, `sha`      | Ein Unterverzeichnis eines Git-Repository, abgerufen mit einem spärlichen partiellen Klon                                                                                                                                             |
| `npm`          | `package`, `version`, `registry` | npm-Paket, abgerufen mit Ihrem npm-Client und entpackt ohne Ausführung von Installationsskripten                                                                                                                                      |
| `archive`      | `url`, `sha256`                  | Zip-Archiv über HTTPS. Erfordert Claude Code v2.1.224 oder später                                                                                                                                                                     |
| `command`      | `command`, `timeout`, `mode`     | Verzeichnis, das von einem Befehl gedruckt wird, den Claude Code auf der Maschine des Benutzers ausführt. Erfordert Claude Code v2.1.229 oder später                                                                                  |

Die Namen `url` und `github` sind auch [Marketplace-Quell](#marketplace-sources)-Typen, wobei `url` einen direkten Link zu einer `marketplace.json`-Datei anstelle eines Git-Repository bedeutet. `git` existiert nur als Marketplace-Quelle, und `npm` existiert als beides. `git-subdir`, `archive` und `command` existieren nur als Plugin-Quellen.

Verwenden Sie einen relativen Pfad für ein Plugin in einem Unterverzeichnis des Marketplace-Repository selbst. Verwenden Sie `git-subdir` für ein Unterverzeichnis eines anderen Repository.

`github`, `url` und `git-subdir`-Quellen teilen die Felder `ref` und `sha`:

* **`ref`**: ein Branch oder Tag. Standardmäßig der Standard-Branch des Repository.
* **`sha`**: eine vollständige 40-stellige Kleinbuchstaben-Commit-SHA. Wenn Sie sowohl `ref` als auch `sha` setzen, checkt Claude Code `sha` aus. Auf den meisten Git-Hosts, einschließlich GitHub, GitLab und Bitbucket, bedeutet dies, dass die Installation erfolgreich ist, auch wenn der Branch oder Tag, der von `ref` benannt wird, seitdem upstream gelöscht wurde, solange der Commit noch vom Repository erreichbar ist. Einige Server, wie AWS CodeCommit, unterstützen das Abrufen von Commits nach SHA nicht. Auf diesen Servern muss `ref` noch existieren und der angeheftete Commit muss von ihm erreichbar sein.

Für die Art, wie jeder Typ abgerufen, zwischengespeichert und versioniert wird, siehe [Plugin-Lade-Referenz](/docs/de/plugins/loading).

<h3 id="relative-path-plugin-source">
  Relative Pfad-Plugin-Quelle
</h3>

Der Pfad wird vom Marketplace-Root aufgelöst. `./plugins/formatter` ist `<root>/plugins/formatter`, obwohl sich die Marketplace-Datei in `<root>/.claude-plugin/` befindet.

Ein Pfad, der `..` enthält, schlägt die Validierung fehl. Auf macOS und Linux lehnt Claude Code einen Eintragspfad ab, der irgendwo nach dem führenden `./` einen Backslash enthält, daher schreiben Sie den Pfad mit Schrägstrichen.

```json theme={null}
{ "name": "formatter", "source": "./plugins/formatter" }
```

Ein relativer Pfad wird nur aufgelöst, wenn Claude Code die Dateien des Marketplace hat, daher überprüfen Sie den [Marketplace-Quell](#marketplace-sources)-Typ:

* **`github`, `git`, `file` und `directory`**: Claude Code hat die Dateien des Marketplace.
* **`url`**: Claude Code ruft nur `marketplace.json` ab, daher können sich relative Pfade nicht auflösen. Geben Sie jedem Plugin stattdessen eine Objekt-Quelle, wie `github` oder `git-subdir`.
* **`settings`**: relative Pfade werden sofort abgelehnt.

<h4 id="bare-names-under-pluginroot">
  Bare Names unter pluginRoot
</h4>

Ein bare Name ist ein einzelner Verzeichnisname ohne `/`, wie `"formatter"`. Um bare Names anstelle von `./`-Pfaden zu schreiben, setzen Sie [`metadata.pluginRoot`](#top-level-fields) auf das Verzeichnis, unter dem sie sich auflösen. Mit `"pluginRoot": "./plugins"` wird `"source": "formatter"` zu `./plugins/formatter` aufgelöst. Erfordert Claude Code v2.1.239 oder später.

`metadata.pluginRoot` hat diese Grenzen:

* Es muss selbst ein relativer Pfad im Marketplace sein.
* Es hat keine Auswirkung auf eine Quelle, die bereits mit `./` beginnt.
* Eine Quelle, die ein `/` enthält, wie `team-a/formatter`, ist kein bare Name und benötigt immer noch das `./`-Präfix, auch wenn `metadata.pluginRoot` gesetzt ist.

<h3 id="github-plugin-source">
  github Plugin-Quelle
</h3>

`repo` nimmt `owner/repo`. `ref` und `sha` sind optional.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "github",
    "repo": "your-org/formatter",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

<h3 id="url-plugin-source">
  url Plugin-Quelle
</h3>

`url` ist eine vollständige Git-URL: `https://`, `http://`, `file://` oder `git@`. Ein `.git`-Suffix ist nicht erforderlich, daher funktionieren Azure DevOps- und AWS CodeCommit-URLs wie geschrieben. Dieser Typ nimmt keine `owner/repo`-Kurzform.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "url",
    "url": "https://gitlab.example.com/your-group/formatter.git",
    "ref": "main"
  }
}
```

<h3 id="git-subdir-plugin-source">
  git-subdir Plugin-Quelle
</h3>

`url` akzeptiert eine vollständige Git-URL oder GitHub `owner/repo`-Kurzform. `path` ist das Unterverzeichnis, das das Plugin enthält, und Claude Code lädt nur dieses Unterverzeichnis herunter.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/your-org/monorepo.git",
    "path": "tools/formatter"
  }
}
```

<h3 id="npm-plugin-source">
  npm Plugin-Quelle
</h3>

Eine `npm`-Quelle nimmt diese Felder:

* `package`: ein Paketname oder ein scoped Name wie `@your-org/formatter`
* `version`: eine Version oder ein Bereich
* `registry`: eine Registry-URL für ein Paket, das nicht in der Standard-Registry ist

Claude Code ruft das Paket mit Ihrem npm-Client ab. Die Installationsskripte des Pakets, wie `preinstall` oder `postinstall`, werden nie ausgeführt, und seine Abhängigkeiten werden während des Abrufs nicht installiert. Wenn das Paket eine unterstützte Lockdatei neben seiner `package.json` hat, installiert Claude Code diese [Node.js-Paketabhängigkeiten](/docs/de/plugins/loading#node-js-package-dependencies) in einem separaten Schritt, auch mit deaktivierten Skripten.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "npm",
    "package": "@your-org/formatter",
    "version": "^2.0.0",
    "registry": "https://npm.example.com"
  }
}
```

<h3 id="archive-plugin-source">
  archive Plugin-Quelle
</h3>

`url` muss `https://` verwenden und kann nicht auf einen Loopback-, Link-Local- oder Cloud-Metadaten-Host verweisen.

Der Plugin-Root kann sich oben im Zip oder ein Verzeichnis darunter befinden.

`sha256` ist der Digest des Archivs als 64 Hex-Zeichen, Großbuchstaben oder Kleinbuchstaben. Wenn Sie ihn setzen, lehnt Claude Code einen Download ab, der nicht übereinstimmt.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "archive",
    "url": "https://artifacts.example.com/formatter-2.0.0.zip",
    "sha256": "6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1"
  }
}
```

<h3 id="command-plugin-source">
  command Plugin-Quelle
</h3>

Verwenden Sie eine `command`-Quelle, wenn ein auf der Maschine des Benutzers installiertes Tool das Plugin-Verzeichnis erzeugt, wie eine IDE, die ihr Plugin für die Toolchain rendert, die der Benutzer ausgewählt hat. Claude Code führt den Befehl aus, wenn der Benutzer das Plugin installiert oder aktualisiert, und [erneut einmal pro Sitzung](/docs/de/plugins/loading#when-a-command-source-re-runs), daher erhalten Benutzer die geänderte Ausgabe des Tools ohne Neuinstallation.

Eine `command`-Quelle nimmt diese Felder:

* `command`: ein Shell-Befehl, der den absoluten Pfad des Plugin-Verzeichnisses als eine Zeile druckt und mit 0 beendet. Claude Code zeigt Benutzern die ganze Zeichenfolge zur Überprüfung an, bevor sie ausgeführt wird. Schreiben Sie sie als druckbares ASCII, höchstens 500 Zeichen, ohne vier oder mehr Leerzeichen hintereinander.
* `timeout`: eine ganze Zahl von Sekunden von 1 bis 600. Standardmäßig 60.
* `mode`: `copy`, der Standard, oder `link`. Siehe [Copy-Modus und Link-Modus](#copy-mode-and-link-mode).

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "command",
    "command": "my-tool claude-plugin-path",
    "timeout": 120
  }
}
```

Für die Art, wie Benutzer den Befehl akzeptieren, siehe [Aus Ihrer Shell installieren](/docs/de/plugins/install#install-from-your-shell). Für das, was Benutzer sehen, nachdem Sie ihn ändern, siehe [Ändern Sie den Befehl einer Befehlsquelle](/docs/de/plugins/host-marketplace#change-the-command-of-a-command-source). Administratoren schalten Befehlsquellen mit [`disableCommandPluginSources`](/docs/de/settings-reference#disablecommandpluginsources) aus.

<h4 id="what-the-command-must-do">
  Was der Befehl tun muss
</h4>

Schreiben Sie den Befehl, um diese Anforderungen zu erfüllen:

* **Shell und Arbeitsverzeichnis**: Claude Code führt den Befehl durch `sh` oder auf Windows durch `cmd.exe` aus dem Home-Verzeichnis des Benutzers aus. Geben Sie einen absoluten Pfad oder einen Befehl auf `PATH` an.
* **Ausgabe**: drucken Sie genau eine Zeile auf stdout, den absoluten Pfad des Plugin-Verzeichnisses, und beenden Sie mit 0 innerhalb von `timeout` Sekunden.
* **Verzeichnisinhalte**: das Verzeichnis enthält das vollständige Plugin, wenn der Befehl beendet wird. Der Pfad kann sich von Lauf zu Lauf unterscheiden.

<h4 id="output-that-fails-the-install-or-update">
  Ausgabe, die die Installation oder Aktualisierung fehlschlagen lässt
</h4>

Die Installation oder Aktualisierung schlägt fehl, wenn der Befehl mit Nicht-Null beendet wird, länger als `timeout` läuft oder etwas anderes als einen absoluten Pfad druckt. Es schlägt auch fehl, wenn das gedruckte Verzeichnis eines dieser ist:

* **Kein Plugin-Inhalt**: das gedruckte Verzeichnis hat keinen Plugin-Inhalt auf seiner obersten Ebene, wie ein `.claude-plugin/`-Verzeichnis oder ein `skills/`-, `commands/`-, `agents/`- oder `hooks/`-Verzeichnis.
* **Das Verzeichnis der Sitzung selbst**: das gedruckte Verzeichnis ist das, in dem Claude Code gestartet wurde, oder eines seiner übergeordneten Verzeichnisse.
* **Ein Netzwerkpfad**: auf Windows ist der gedruckte Pfad ein UNC-Pfad.
* **Zu groß zum Kopieren**: im Copy-Modus ist das Verzeichnis größer als 256 MiB oder hat mehr als 20.000 Einträge.

<h4 id="copy-mode-and-link-mode">
  Copy-Modus und Link-Modus
</h4>

`mode` entscheidet, ob Claude Code das gedruckte Verzeichnis kopiert oder es an Ort und Stelle verwendet:

* **`copy`**: Claude Code kopiert das Verzeichnis in den Plugin-Cache und leitet die [Plugin-Version](/docs/de/plugins/loading#how-claude-code-computes-the-version) von einem Hash der kopierten Dateien ab. Ihr Tool kann das Verzeichnis nach dem Beenden des Befehls löschen oder umschreiben. Ein erneuter Lauf, der identische Dateien erzeugt, zählt als aktuell.
* **`link`**: Claude Code füllt den Cache-Eintrag des Plugins mit einem Link zu jedem Top-Level-Eintrag des gedruckten Verzeichnisses und lädt die Dateien an Ort und Stelle. Nichts wird kopiert, Dateiinhalte werden nicht gehasht, und die Größenlimits gelten nicht. Verwenden Sie es für ein Verzeichnis, das zu groß zum Kopieren ist, wie einen gerenderten SDK-Export.

Ein Link-Modus-Plugin hat diese Anforderungen:

* **Halten Sie das Verzeichnis an Ort und Stelle**: Claude Code lädt das Plugin bei jedem Start durch die Links, daher muss das gedruckte Verzeichnis bleiben, wo es ist, solange das Plugin installiert bleibt.
* **Drucken Sie einen anderen Pfad, um neuen Inhalt zu signalisieren**: die Version kommt vom echten Pfad des gedruckten Verzeichnisses und seinen Top-Level-Einträgen, nicht von den Dateien darin.
* **Halten Sie Top-Level-Symlinks im Verzeichnis**: die Installation schlägt fehl, wenn ein Top-Level-Eintrag ein Symlink ist, der außerhalb des gedruckten Verzeichnisses verweist.
* **Schließen Sie `node_modules` ein**: Claude Code überspringt die [Node.js-Paketabhängigkeitsinstallation](/docs/de/plugins/loading#node-js-package-dependencies) für ein Link-Modus-Plugin, daher drucken Sie ein Verzeichnis, das bereits die Pakete enthält, die das Plugin benötigt.
* **Sitzungen, die im Verzeichnis gestartet werden**: eine Sitzung, die im gedruckten Verzeichnis oder irgendwo darunter gestartet wird, lädt das Plugin nicht.
* **Nicht auf Windows**: Claude Code lehnt die Installation eines Link-Modus-Plugins auf Windows ab. Deklarieren Sie `"mode": "copy"` dort.

<h2 id="marketplace-sources">
  Marketplace-Quellen
</h2>

Eine Marketplace-Quelle gibt an, woher Claude Code eine `marketplace.json` abruft. Die CLI erstellt eine für Sie, wenn Sie einen Marketplace hinzufügen, und Sie schreiben eine selbst in Einstellungen:

* **[`claude plugin marketplace add`](/docs/de/plugins/cli-reference)**: Claude Code erstellt die Quelle aus der Zeichenfolge, die Sie übergeben.
* **[`extraKnownMarketplaces`](/docs/de/settings-reference#extraknownmarketplaces)**: Sie schreiben die Quelle selbst als das `source`-Objekt.
* **[`strictKnownMarketplaces`](/docs/de/settings-reference#strictknownmarketplaces) und [`blockedMarketplaces`](/docs/de/plugins/org#restrict-what-users-can-install)**: Administratoren schreiben Quellen in diese zwei Richtlinienlisten. `strictKnownMarketplaces` ist die Allowlist und `blockedMarketplaces` ist die Blocklist.

Die Typnamen `url`, `git` und `github` bedeuten etwas anderes in einer Marketplace-Quelle als in einer [Plugin-Quelle](#plugin-sources):

| Typname  | Als Marketplace-Quelle                                                                               | Als Plugin-Quelle                                                         |
| :------- | :--------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------ |
| `url`    | Ein direkter Link zu einer `marketplace.json`-Datei mit Feldern `url`, `headers` und `headersHelper` | Ein Git-Repository zum Klonen mit Feldern `url`, `ref` und `sha`          |
| `git`    | Ein Git-Repository zum Klonen mit Feldern `url`, `ref`, `path` und `sparsePaths`                     | Existiert nicht                                                           |
| `github` | Ein GitHub-Repository mit Feldern `repo`, `ref`, `path` und `sparsePaths`                            | Ein GitHub-Repository mit Feldern `repo`, `ref` und `sha` und kein `path` |

Die Tabelle listet jeden Marketplace-Quelltyp mit seinen Feldern, der `claude plugin marketplace add`-Eingabe, die ihn erzeugt, und was er in jedem der drei Einstellungsschlüssel tut.

| Typ           | Felder                               | `marketplace add`-Eingabe                                                                                                                                                      | `extraKnownMarketplaces`                                             | `strictKnownMarketplaces`                                                                                                                                                                                                                        | `blockedMarketplaces`                                                             |
| :------------ | :----------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------- |
| `url`         | `url`, `headers`, `headersHelper`    | Eine `http://`- oder `https://`-URL, die keine Git-Form entspricht                                                                                                             | Lädt                                                                 | Erlaubt die gleiche URL                                                                                                                                                                                                                          | Blockiert die gleiche URL                                                         |
| `github`      | `repo`, `ref`, `path`, `sparsePaths` | `owner/repo`, `owner/repo@ref` oder `owner/repo#ref`                                                                                                                           | Lädt                                                                 | Erlaubt das gleiche `repo`, `ref` und `path`. `repo` kann `owner/*` sein                                                                                                                                                                         | Blockiert das gleiche und eine `git`-URL zum gleichen Repository                  |
| `git`         | `url`, `ref`, `path`, `sparsePaths`  | Eine `user@host:path`-URL oder eine `https://`-URL, die mit `.git` endet, `/_git/` enthält oder ein github.com- oder gitlab.com-Repository benennt. `#ref` heftet einen ref an | Lädt                                                                 | Erlaubt die gleiche URL, `ref` und `path`                                                                                                                                                                                                        | Blockiert das gleiche und andere Schreibweisen des gleichen github.com-Repository |
| `npm`         | `package`                            | Nicht erzeugt                                                                                                                                                                  | Schlägt zu laden fehl: `NPM marketplace sources not yet implemented` | Parst aber passt nichts, weil sich nichts als `npm`-Marketplace registriert                                                                                                                                                                      | Parst aber passt nichts                                                           |
| `file`        | `path`                               | Ein Pfad zu einer `.json`-Datei                                                                                                                                                | Lädt                                                                 | Erlaubt den gleichen Pfad                                                                                                                                                                                                                        | Blockiert den gleichen Pfad                                                       |
| `directory`   | `path`                               | Ein Pfad zu einem Verzeichnis                                                                                                                                                  | Lädt                                                                 | Erlaubt den gleichen Pfad                                                                                                                                                                                                                        | Blockiert den gleichen Pfad                                                       |
| `settings`    | `name`, `plugins`, `owner`           | Nicht erzeugt                                                                                                                                                                  | Lädt                                                                 | Erlaubt einen Eintrag mit dem gleichen `name` und identischen `plugins`                                                                                                                                                                          | Blockiert den gleichen `name`                                                     |
| `skills-dir`  | none                                 | Nicht erzeugt                                                                                                                                                                  | Schlägt zu laden fehl: `Unsupported marketplace source type`         | Hält [Skills-Verzeichnis-Plugins](/docs/de/plugins/org#keep-skills-directory-plugins-loading) beim Laden, während eine Allowlist gesetzt ist. Siehe [Quellwerte, die nur in Richtlinienlisten gültig sind](#source-values-valid-only-in-policy-lists) | Stoppt Skills-Verzeichnis-Plugins vom Laden                                       |
| `hostPattern` | `hostPattern`                        | Nicht erzeugt                                                                                                                                                                  | Schlägt zu laden fehl: `Unsupported marketplace source type`         | Erlaubt `github`-, `git`- und `url`-Quellen, deren Host passt                                                                                                                                                                                    | Blockiert diese Quellen                                                           |
| `pathPattern` | `pathPattern`                        | Nicht erzeugt                                                                                                                                                                  | Schlägt zu laden fehl: `Unsupported marketplace source type`         | Erlaubt `file`- und `directory`-Quellen, deren `path` passt                                                                                                                                                                                      | Blockiert diese Quellen                                                           |

<h3 id="fields-by-type">
  Felder nach Typ
</h3>

Die Tabelle listet jedes Marketplace-Quellfeld auf, das einen Standard, eine Einschränkung oder eine typspezifische Bedeutung hat.

| Feld            | Typen           | Beschreibung                                                                                                                                                                                                                                                                    |
| :-------------- | :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `url`           | `url`           | Link zur `marketplace.json`-Datei. Claude Code lädt nur diese Datei herunter, daher können die Plugins des Marketplace keine [relativen Pfad-Quellen](#relative-path-plugin-source) verwenden                                                                                   |
| `url`           | `git`           | Das Git-Repository zum Klonen                                                                                                                                                                                                                                                   |
| `headers`       | `url`           | Zuordnung von HTTP-Headern, die Claude Code mit dem Abruf sendet, für authentifizierte Hosts                                                                                                                                                                                    |
| `headersHelper` | `url`           | Befehl, der Header druckt, deren Werte zu kurzlebig sind, um sie in `headers` aufzulisten. Erfordert Claude Code v2.1.238 oder später. Siehe [Authentifizieren Sie Archive-Downloads](/docs/de/plugins/host-marketplace#authenticate-archive-downloads)                              |
| `repo`          | `github`        | In `marketplace add` und `extraKnownMarketplaces` muss `repo` ein Repository benennen. `marketplace add` lehnt `owner/*` als keine gültige `owner/repo`-Kurzform ab; in `extraKnownMarketplaces` nimmt Claude Code es wörtlich und der Klon schlägt fehl                        |
| `ref`           | `github`, `git` | Branch oder Tag. Standardmäßig der Standard-Branch des Repository                                                                                                                                                                                                               |
| `path`          | `github`, `git` | Der Pfad der Marketplace-Datei im Repository. Standardmäßig `.claude-plugin/marketplace.json`                                                                                                                                                                                   |
| `path`          | `file`          | Die Marketplace-Datei selbst. Claude Code liest sie an Ort und Stelle und nimmt das Verzeichnis zwei Ebenen oben als Marketplace-Root, daher halten Sie die Datei unter `<root>/.claude-plugin/marketplace.json`                                                                |
| `path`          | `directory`     | Der Marketplace-Root, das Verzeichnis, das `.claude-plugin/marketplace.json` enthält                                                                                                                                                                                            |
| `sparsePaths`   | `github`, `git` | Array von Verzeichnissen für einen spärlichen Checkout, wie `[".claude-plugin", "plugins"]`. `claude plugin marketplace add --sparse` setzt es                                                                                                                                  |
| `skipLfs`       | `github`, `git` | Akzeptiert und hat keine Auswirkung. Siehe [Halten Sie Plugin-Dateien aus Git LFS](/docs/de/plugins/host-marketplace#keep-plugin-files-out-of-git-lfs)                                                                                                                               |
| `name`          | `settings`      | Muss dem `extraKnownMarketplaces`-Schlüssel entsprechen und kann kein [reservierter Name](#reserved-names) sein                                                                                                                                                                 |
| `plugins`       | `settings`      | Der Inline-Katalog ohne gehostete Datei. Jedes Element nimmt `name`, `source`, `description`, `version`, `strict`, `headers` und `headersHelper`. Schreiben Sie die `source` jedes Elements als einen Objekttyp, weil ein relativer Pfad kein Repository zum Auflösen gegen hat |

<h3 id="source-values-valid-only-in-policy-lists">
  Quellwerte, die nur in Richtlinienlisten gültig sind
</h3>

`hostPattern`, `pathPattern`, `skills-dir` und die `owner/*`-Form von `repo` sind nur in den zwei Richtlinienlisten gültig, `strictKnownMarketplaces` und `blockedMarketplaces`:

* **`hostPattern` und `pathPattern`**: reguläre Ausdrücke, die Claude Code gegen eine Quelle testet, bevor er von ihr abruft.
* **`skills-dir`**: keine Quelle. Wenn Sie `strictKnownMarketplaces` überhaupt setzen, [Skills-Verzeichnis-Plugins](/docs/de/plugins/org#keep-skills-directory-plugins-loading) stoppen das Laden, bis Sie `{"source": "skills-dir"}` zu dieser Liste hinzufügen.
* **`owner/*`**: als ein `github` `repo`-Wert passt jedes Repository unter genau diesem GitHub-Besitzer. Erfordert Claude Code v2.1.223 oder später.

Für die Reihenfolge der Übereinstimmung, exakte `ref`-Semantik und Rezepte, siehe [Plugins für Ihre Organisation verwalten](/docs/de/plugins/org).

<h3 id="source-objects-in-settings">
  Quellobjekte in Einstellungen
</h3>

Ein `extraKnownMarketplaces`-Wert ist eine Zuordnung vom Marketplace-Namen zu einem Objekt mit `source`. Dieser Eintrag registriert einen Marketplace aus einem Git-Repository bei seinem `main`-Branch:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": {
        "source": "git",
        "url": "https://git.example.com/your-org/your-marketplace.git",
        "ref": "main"
      }
    }
  }
}
```

`strictKnownMarketplaces` und `blockedMarketplaces` sind Arrays von Quellobjekten. Diese Allowlist lässt einen GitHub-Besitzer und einen internen Host zu:

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "your-org/*" },
    { "source": "hostPattern", "hostPattern": "^git\\.example\\.com$" }
  ]
}
```

<h2 id="validation-messages">
  Validierungsmeldungen
</h2>

`claude plugin validate <path>` nimmt den Marketplace-Root oder die Marketplace-Datei selbst. Es druckt Fehler und Warnungen. Für Exit-Codes und `--strict`, siehe [plugin validate](/docs/de/plugins/cli-reference#plugin-validate).

Eine Meldung benennt einen Plugin-Eintrag nach seinem Index, geschrieben als `plugins.1.source` oder `plugins[1].source`.

Eine Meldung mit dem Präfix eines Eintrag-Index und `plugin.json →`, wie `plugins[2] plugin.json →`, ist über die eigenen Dateien dieses Plugins. [`claude plugin validate` meldet Fehler](/docs/de/plugins/troubleshooting#claude-plugin-validate-reports-errors) listet diese Meldungen mit ihren Fixes auf.

Warnungen, die Claude Desktop-Flaggennamen erwähnen, die Claude Code akzeptiert, aber Claude Desktop ablehnt, weil Claude Desktop strengere Namenregeln hat.

Die Tabelle ordnet Marketplace-Level-Meldungen dem Feld zu, das jede betrifft.

| Meldung                                                                                                                                                                                                      | Ebene   | Feld                                                                                                                            |
| :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------ | :------------------------------------------------------------------------------------------------------------------------------ |
| `Marketplace must have a name`                                                                                                                                                                               | Error   | `name` ist leer                                                                                                                 |
| `Marketplace name cannot contain spaces. Use kebab-case (e.g., "my-marketplace")`                                                                                                                            | Error   | `name`                                                                                                                          |
| `Marketplace name cannot contain path separators (/ or \), ".." sequences, or be "."`                                                                                                                        | Error   | `name`                                                                                                                          |
| `Marketplace name impersonates an official Anthropic/Claude marketplace`                                                                                                                                     | Error   | `name`. Siehe [Reservierte Namen](#reserved-names)                                                                              |
| `Marketplace name cannot contain control or bidirectional-formatting characters`                                                                                                                             | Error   | `name` enthält ein Steuerzeichen, wie ein Escape oder eine Zeilenumbruch, oder ein Unicode-Bidirektionales-Formatierungszeichen |
| `Marketplace name "inline" is reserved for --plugin-dir session plugins`, und die `builtin`-, `skills-dir`-, `synced`-, `claude-plugin-test`-, `npm`-, `pip`-, `uv`-, `cargo`-, `github`- und `gh`-Varianten | Error   | `name`                                                                                                                          |
| `Author name cannot be empty`                                                                                                                                                                                | Error   | `owner.name`                                                                                                                    |
| `Plugin name cannot contain spaces. Use kebab-case (e.g., "my-plugin")`                                                                                                                                      | Error   | `plugins[i].name`                                                                                                               |
| `Plugin name cannot contain control or bidirectional-formatting characters`                                                                                                                                  | Error   | `plugins[i].name`                                                                                                               |
| `Duplicate plugin name "x" found in marketplace`                                                                                                                                                             | Error   | Zwei Einträge teilen einen `name`                                                                                               |
| `plugins.i.source: Invalid input`                                                                                                                                                                            | Error   | Die `source` des Eintrags passt zu keinem Typ. Siehe [Ungültige Eingabe auf einer Quelle](#invalid-input-on-a-source)           |
| `plugins[i].source: Path contains "..": <path>`                                                                                                                                                              | Error   | Ein relativer `source`, der dem Marketplace-Root entkommt                                                                       |
| `source.source: 'unsupported' is a parse-time placeholder and cannot be authored`                                                                                                                            | Error   | `plugins[i].source`                                                                                                             |
| `Plugin "x" sets headersHelper but is not "strict": false`                                                                                                                                                   | Error   | `plugins[i].headersHelper`, auf einem `archive`-Eintrag                                                                         |
| `chain does not resolve (<reason>) — target must be a name in plugins[], a key in renames, or null`                                                                                                          | Error   | `renames.<old>`                                                                                                                 |
| `target "x" is not a valid plugin name (PluginIdSchema)`                                                                                                                                                     | Error   | `renames.<old>`                                                                                                                 |
| `Unknown field 'x'. Claude Code ignores it at load time.`                                                                                                                                                    | Warning | Der benannte Schlüssel auf der obersten Ebene, unter `metadata`, in einem Eintrag oder unter der `relevance` eines Eintrags     |
| `Marketplace has no plugins defined`                                                                                                                                                                         | Warning | `plugins` ist leer                                                                                                              |
| `Plugin "x" sets headers/headersHelper, which only apply to "archive" sources; they have no effect on this entry.`                                                                                           | Warning | `plugins[i].headers` oder `plugins[i].headersHelper`, auf einem Eintrag, dessen `source` nicht `archive` ist                    |
| `Plugin "x" fetches its archive with a headersHelper but sets no sha256 pin`                                                                                                                                 | Warning | `plugins[i].source.sha256`                                                                                                      |
| `Header "x" is a request-routing/identity header that catalog entries may not set; Claude Code drops it at download time.`                                                                                   | Warning | `plugins[i].headers.<name>`                                                                                                     |
| `Local source "x" is or traverses a symlink, so <path> was not read`                                                                                                                                         | Warning | `plugins[i].source`                                                                                                             |
| `No marketplace description provided. Adding a description helps users understand what this marketplace offers`                                                                                              | Warning | `description`                                                                                                                   |
| `Entry declares version "x" but <path>/plugin.json says "y". At install time, plugin.json wins`                                                                                                              | Warning | `plugins[i].version`, auf einem relativen Pfad-Eintrag                                                                          |
| `'relevance' must be an object containing topic and signals; got <type>. It will be ignored at load time.`                                                                                                   | Warning | `plugins[i].relevance`                                                                                                          |
| `'metadata' must be a free-form object; got <type>. It will be ignored at load time.`                                                                                                                        | Warning | `plugins[i].metadata`                                                                                                           |
| `'experimental' must be an object containing component declarations; got <type>. It will be ignored at load time.`                                                                                           | Warning | `plugins[i].experimental`                                                                                                       |
| `Marketplace name "x" is reserved in Claude Desktop`                                                                                                                                                         | Warning | `name` ist `org`, `org-provisioned` oder `unknown`. Claude Desktop lehnt den Marketplace ab                                     |
| `Marketplace name "x" is not accepted by Claude Desktop (letters, digits, ".", "_", "-"; must start alphanumeric; max 128 chars)`                                                                            | Warning | `name`. Claude Desktop lehnt den Marketplace ab                                                                                 |
| `Plugin name "x" is not accepted by Claude Desktop (letters, digits, ".", "_", "-"; must start alphanumeric; max 128 chars)`                                                                                 | Warning | `plugins[i].name`. Claude Desktop lässt den Eintrag fallen                                                                      |

<h3 id="invalid-input-on-a-source">
  Ungültige Eingabe auf einer Quelle
</h3>

`Invalid input` auf einer `source` bedeutet, dass das Objekt keinem Quelltyp entsprach. Überprüfen Sie diese Ursachen:

* Ein relativer Pfad, der nicht mit `./` beginnt, außer `"."` oder einem [bare Name unter `metadata.pluginRoot`](#relative-path-plugin-source)
* Ein `npm` `package`, das `..` enthält
* Ein `source`-Typ, der nicht einer der [Plugin-Quellen](#plugin-sources) ist
* Ein bekannter Typ mit einem erforderlichen Feld, das fehlt oder den falschen Typ hat, wie `github` ohne `repo`

<h3 id="failures-that-validation-doesn’t-catch">
  Fehler, die die Validierung nicht erfasst
</h3>

`claude plugin validate` meldet nicht jeden Fehler. Ein Eintrag `hooks`, der als Dateipfad oder Array geschrieben wird, besteht die Validierung, und der Fehler erscheint nur, wenn das Plugin lädt, wie [Hooks in einem Eintrag](#hooks-in-an-entry) beschreibt. Fehler beim Abrufen einer `source` erscheinen auch nur nach der Installation, nicht in der Validierung.

[`claude plugin list`](/docs/de/plugins/cli-reference) zeigt ein Plugin, das nicht geladen wurde, mit seinem Fehler, und [Plugins beheben](/docs/de/plugins/troubleshooting) behandelt die Lade-Zeit-Zeichenfolgen.

<h2 id="next-steps">
  Nächste Schritte
</h2>

* [Marketplace erstellen](/docs/de/plugins/create-marketplace): Erstellen Sie einen Marketplace aus diesen Feldern und installieren Sie ihn lokal
* [Marketplace hosten und verwalten](/docs/de/plugins/host-marketplace): Wo Sie die Datei ablegen und wie Benutzer Änderungen erhalten
* [Plugin-Manifest-Referenz](/docs/de/plugins/manifest-reference): Die `plugin.json`-Felder, die ein Eintrag überschreiben kann
* [Plugins für Ihre Organisation verwalten](/docs/de/plugins/org): Allowlist- und Blocklist-Rezepte, die diese Quellwerte verwenden
