> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugin-Abhängigkeiten

> Deklarieren Sie die Plugins, von denen Ihr Plugin abhängt, mit Versionsbereichen wie ^1.2, und erfahren Sie, wie Claude Code diese installiert, auflöst und bereinigt.

Eine Plugin-Abhängigkeit ist ein anderes Plugin, auf das Ihr Plugin angewiesen ist, z. B. eines, dessen MCP-Server oder Skill es aufruft. Jede Abhängigkeit verfolgt die neueste Version, die sein Marketplace bereitstellt, es sei denn, Sie deklarieren eine Versionsbeschränkung – einen semantischen Versionsbereich wie `^2.0` oder `~2.1.0`, gegen den Sie getestet haben.

Diese Seite ist für Plugin-Autoren, die Abhängigkeiten in `plugin.json` deklarieren, und für Marketplace-Betreuer, die Releases taggen.

<Note>
  Diese Fälle werden auf anderen Seiten behandelt:

  * **Installation eines Plugins mit Abhängigkeiten**: siehe [Installierte Plugins verwalten](/docs/de/plugins/install#manage-installed-plugins)
  * **Lesen eines Abhängigkeitsfehlers**: siehe [Abhängigkeitsfehler](/docs/de/plugins/troubleshooting#dependency-errors)
  * **Deklarieren der npm- und Bun-Pakete, die der eigene Code Ihres Plugins benötigt**: siehe [Node.js-Paketabhängigkeiten](/docs/de/plugins/loading#node-js-package-dependencies)
</Note>

Um eine Beschränkung hinzuzufügen, beginnen Sie bei [Abhängigkeit mit Versionsbeschränkung deklarieren](#declare-a-dependency-with-a-version-constraint). Wenn Sie ein Plugin verwalten, von dem andere abhängen, [taggen Sie Ihre Releases](#tag-plugin-releases-for-version-resolution), damit ihre Beschränkungen aufgelöst werden können.

<h2 id="declare-dependencies">
  Abhängigkeiten deklarieren
</h2>

<span id="decide-whether-to-constrain-dependency-versions" />Ohne Versionsbeschränkung wechselt eine Abhängigkeit zu jedem neuen Release, das sein Marketplace veröffentlicht, wenn Benutzer das nächste Mal aktualisieren. Wenn dieses Release ein MCP-Tool umbenennt, das Ihr Plugin aufruft, bricht Ihr Plugin für alle, die aktualisieren.

Mit einer Beschränkung wie `~2.1.0` auf eine Abhängigkeit aus einer Git-gestützten Quelle erhalten Benutzer, die Ihr Plugin installiert haben, weiterhin `2.1.x`-Patches der Abhängigkeit und wechseln nie zu `2.2`. Um nach Ihrem eigenen Zeitplan zu aktualisieren, testen Sie gegen ein neueres Release und veröffentlichen dann eine neue Version Ihres Plugins mit einer breiteren Beschränkung.

<h3 id="declare-a-dependency-with-a-version-constraint">
  Abhängigkeit mit Versionsbeschränkung deklarieren
</h3>

Listen Sie Abhängigkeiten im `dependencies`-Array der `.claude-plugin/plugin.json` Ihres Plugins auf. Das folgende Manifest deklariert eine unversionierte Abhängigkeit und eine beschränkte Abhängigkeit:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "deploy-kit",
  "version": "3.1.0",
  "dependencies": [
    "audit-logger",
    { "name": "secrets-vault", "version": "~2.1.0" }
  ]
}
```

Ein Eintrag kann ein String sein: nur der Plugin-Name, wie `"audit-logger"` in diesem Manifest, oder `"name@marketplace"`, um ihn in einem anderen Marketplace aufzulösen. Mit einem einfachen String hängt Ihr Plugin von der Version ab, die der Marketplace dieses Plugins bereitstellt.

Um eine Versionsbeschränkung festzulegen, verwenden Sie ein Objekt mit diesen Feldern, jeweils ein String:

| Feld          | Beschreibung                                                                                                                                                                                                                                                                                                               |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | Der Name des Abhängigkeits-Plugins, wie er in seinem Marketplace-Eintrag angezeigt wird. Claude Code schlägt ihn im gleichen Marketplace wie das deklarierte Plugin nach, es sei denn, Sie setzen `marketplace`. Erforderlich.                                                                                             |
| `version`     | Ein [semantischer Versionsbereich](https://github.com/npm/node-semver#ranges) wie `~2.1.0`, `^2.0`, `>=1.4` oder `=2.1.0`. Die Abhängigkeit wird in der höchsten Git-Tag installiert, die diesen Bereich erfüllt, daher muss der Betreuer der Abhängigkeit [Releases taggen](#tag-plugin-releases-for-version-resolution). |
| `marketplace` | Ein anderer Marketplace, um `name` darin aufzulösen. Eine Allowlist steuert Abhängigkeiten zwischen Marketplaces, beschrieben in [Plugin aus einem anderen Marketplace abhängen](#depend-on-a-plugin-from-another-marketplace).                                                                                            |

Ein Bereich stimmt nicht mit Vorabversionen wie `2.0.0-beta.1` überein, es sei denn, Sie entscheiden sich mit einem Vorabversions-Suffix wie `^2.0.0-0` dafür.

<h3 id="bundle-plugins-for-a-team">
  Plugins für ein Team bündeln
</h3>

Um Ingenieuren die Installation eines kuratierten Satzes von Plugins mit einem Befehl zu ermöglichen, veröffentlichen Sie ein Plugin, dessen Manifest einen `name` und ein `dependencies`-Array enthält. Ein Plugin-Manifest benötigt nur `name`, daher ist dies ein gültiges Plugin, und die Installation installiert jede Abhängigkeit.

Beispielsweise kann ein Plattform-Team rollenspezifische Bundles in einem internen Marketplace veröffentlichen, damit Ingenieure einen `claude plugin install` ausführen, anstatt jedes Plugin separat zu installieren:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "backend-standard",
  "version": "1.0.0",
  "description": "Standard plugin set for backend engineers",
  "dependencies": [
    "secrets-vault",
    "deploy-kit",
    { "name": "db-migrate", "version": "^3.0" },
    "oncall-runbook"
  ]
}
```

Um später ein Plugin zum Standard-Set hinzuzufügen, veröffentlichen Sie eine neue `backend-standard`-Version mit der zusätzlichen Abhängigkeit. Wenn der Marketplace nicht [standardmäßig automatisch aktualisiert](/docs/de/plugins/loading#which-marketplaces-and-plugins-auto-update), aktivieren Ingenieure entweder die automatische Aktualisierung für den Marketplace oder aktualisieren manuell:

* **Automatische Aktualisierung für den Marketplace aktivieren**: Die nächste automatische Aktualisierung verschiebt das Bundle zur neuen Version und installiert alle Abhängigkeiten, die es hinzufügt.
* **Manuell aktualisieren**: Führen Sie `claude plugin update backend-standard` in einer Shell aus, dann `/reload-plugins` in einer offenen Sitzung, um die neu hinzugefügten Abhängigkeiten zu installieren.

Für die Schritte auf der Ingenieur-Seite siehe [Plugins aktualisiert halten](/docs/de/plugins/install#keep-plugins-updated).

Um ein Bundle für alle in einer Organisation bereitzustellen, fügt ein Administrator es zu `enabledPlugins` in verwalteten Einstellungen hinzu. Siehe [Plugins vorinstallieren und erforderlich machen](/docs/de/plugins/org#pre-install-and-require-plugins).

<h3 id="depend-on-a-plugin-from-another-marketplace">
  Plugin aus einem anderen Marketplace abhängen
</h3>

Standardmäßig installiert Claude Code eine Abhängigkeit nicht aus einem anderen Marketplace als dem des deklarierenden Plugins selbst, es sei denn, der Benutzer hat diese Abhängigkeit bereits installiert und aktiviert im gleichen Bereich. Dieser Standard verhindert, dass ein Marketplace stillschweigend Plugins aus einer Quelle installiert, die der Benutzer nicht überprüft hat.

Um die Installation zu ermöglichen, fügen Sie den Namen des Ziel-Marketplaces zu `allowCrossMarketplaceDependenciesOn` in der `marketplace.json` des Root-Marketplaces hinzu. Der Root-Marketplace ist derjenige, der das Plugin hostet, das der Benutzer installiert. Nur die Allowlist des Root-Marketplaces gilt.

Die folgende `marketplace.json` ermöglicht `deploy-kit`, von einem Plugin aus `your-shared-marketplace` abhängig zu sein:

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "allowCrossMarketplaceDependenciesOn": ["your-shared-marketplace"],
  "plugins": [
    {
      "name": "deploy-kit",
      "source": "./deploy-kit",
      "dependencies": [
        { "name": "audit-logger", "marketplace": "your-shared-marketplace" }
      ]
    }
  ]
}
```

Wenn `allowCrossMarketplaceDependenciesOn` fehlt oder den Ziel-Marketplace nicht enthält, installiert Claude Code die Abhängigkeit nicht. Wenn die Abhängigkeit im Marketplace-Eintrag deklariert ist, wird die Installation selbst mit einer Nachricht abgelehnt, die mit `Dependency "audit-logger@your-shared-marketplace" (required by deploy-kit@your-marketplace) is in marketplace "your-shared-marketplace", which is not in the allowlist` beginnt und das zu setzende Feld benennt. Wenn sie in `plugin.json` deklariert ist, wird die Installation ohne die Abhängigkeit abgeschlossen und Ihr Plugin kann dann nicht geladen werden.

Die Allowlist-Prüfung gilt nicht für eine Abhängigkeit, die bereits aktiviert ist. Wenn ein Benutzer `audit-logger` aus `your-shared-marketplace` zuerst selbst installiert, im gleichen Bereich, installiert sich `deploy-kit` dann ohne Änderungen an der Allowlist.

<h3 id="test-a-plugin-and-its-dependency-locally">
  Plugin und seine Abhängigkeit lokal testen
</h3>

Wenn Sie ein Plugin und das Plugin, von dem es abhängt, gleichzeitig entwickeln, starten Sie Claude Code aus Ihrer Shell und laden beide mit [`--plugin-dir`](/docs/de/plugins/cli-reference#flags-that-load-a-plugin-for-one-session):

```bash theme={null}
claude --plugin-dir ./my-dependency --plugin-dir ./my-plugin
```

Die lokale Kopie der Abhängigkeit erfüllt den Abhängigkeitseintrag Ihres Plugins, daher müssen Sie die Abhängigkeit nicht aus ihrem Marketplace installieren.

* **Kein `version` erforderlich**: die lokale `plugin.json` benötigt auch keine `version`, da eine [Versionsbeschränkung](#declare-a-dependency-with-a-version-constraint) nicht gegen eine lokale Kopie überprüft wird.
* **Einträge, die einen Marketplace benennen**: Ein Eintrag, der einen Marketplace benennt, stimmt auch mit der lokalen Kopie auf Claude Code v2.1.242 oder später überein.

Bis Sie die Abhängigkeit aus ihrem Marketplace installieren, wird Ihr Plugin nicht mehr geladen, wenn die lokale Kopie deaktiviert oder nicht vorhanden ist:

* **Sie haben die lokale Kopie deaktiviert**: Ihr Plugin wird beim nächsten Plugin-Laden mit einem Fehler deaktiviert, der mit `is disabled — enable it or remove the dependency` endet. Wenn der Fehler die Abhängigkeit als `<name>@inline` benennt, bezieht sich dieser Bezeichner auf die `--plugin-dir`-Kopie.
* **Sie haben eine Sitzung ohne das `--plugin-dir`-Flag der Abhängigkeit gestartet**: Der Fehler meldet, dass die Abhängigkeit nicht installiert ist. Übergeben Sie das Flag erneut, oder installieren Sie die Abhängigkeit aus ihrem Marketplace.

Wenn sich beide Plugins in einem übergeordneten Ordner befinden, können Sie diesen Ordner einmal an `--plugin-dir` übergeben. Wenn der Ordner selbst kein Plugin ist, lädt Claude Code jeden untergeordneten Ordner, der eine `.claude-plugin/plugin.json` hat. Erfordert Claude Code v2.1.265 oder später.

<h2 id="tag-plugin-releases-for-version-resolution">
  Plugin veröffentlichen, von dem andere abhängen
</h2>

Wenn Sie ein Plugin verwalten, von dem andere Plugins mit einer Versionsbeschränkung abhängen, taggen Sie seine Releases, damit diese Beschränkungen aufgelöst werden können. Eine Beschränkung wird gegen Git-Tags im Repository aufgelöst, das das Plugin hostet. Taggen Sie das Repository, auf das die [Plugin-Quelle](/docs/de/plugins/marketplace-reference#plugin-sources) des Plugins in `marketplace.json` verweist:

* **`github`-, `url`- oder `git-subdir`-Quelle**: das eigene Repository des Plugins, daher erstellt der Autor des Plugins die Tags
* **Relativer Pfad wie `./plugins/secrets-vault`**: das Marketplace-Repository, daher erstellt der Marketplace-Betreuer die Tags

<h3 id="create-a-release-tag">
  Release-Tag erstellen
</h3>

Taggen Sie jedes Release als `<plugin-name>--v<version>`, wobei `<version>` dem `version`-Feld in der `plugin.json` dieses Commits entspricht. Das Plugin-Name-Präfix ermöglicht es einem Marketplace-Repository, mehrere Plugins mit unabhängigen Versionshistorien zu hosten.

Erstellen Sie das Tag aus dem Plugin-Verzeichnis mit einem konfigurierten `origin`-Remote zum Empfangen des gepushten Tags, mit [`claude plugin tag`](/docs/de/plugins/cli-reference#plugin-tag):

```bash theme={null}
claude plugin tag --push
```

Der Befehl erstellt den Tag-Namen aus dem Manifest des Plugins. Vor dem Erstellen des Tags führt er diese Prüfungen aus:

* Validiert das Plugin
* Prüft, dass `plugin.json` und der Marketplace-Eintrag sich auf die Version einigen, wenn sich das Plugin-Verzeichnis in einem Marketplace-Checkout befindet
* Erfordert einen sauberen Arbeitsbaum unter dem Plugin-Verzeichnis
* Lehnt ab, wenn das Tag bereits existiert

Ein erfolgreicher Durchlauf gibt `Created tag secrets-vault--v2.1.0` aus. Mit `--push` gibt es auch `Pushed to origin` aus. Ohne `--push` gibt es den `git push`-Befehl aus, den Sie selbst ausführen können.

Übergeben Sie `--dry-run`, um den Plan zu sehen, ohne etwas zu erstellen.

Die [`claude plugin tag`-Referenz](/docs/de/plugins/cli-reference#plugin-tag) listet die verbleibenden Flags auf.

Sie können auch `git tag secrets-vault--v2.1.0` direkt ausführen, solange Sie die `version` in `plugin.json` und im Marketplace-Eintrag selbst synchron halten.

<h3 id="constrain-a-dependency-that-has-a-non-git-source">
  Abhängigkeit mit nicht-Git-Quelle beschränken
</h3>

Tag-basierte Auflösung gilt nur für Git-gestützte Quellen. Für eine Abhängigkeit mit einer `npm`-, `archive`- oder `command`-[Plugin-Quelle](/docs/de/plugins/marketplace-reference#plugin-sources) steuert die Beschränkung nicht, welche Version abgerufen wird. Sie wird immer noch überprüft, wenn das Plugin geladen wird, und das abhängige Plugin wird deaktiviert, wenn die installierte Version sie nicht erfüllt.

Für `npm`-, `archive`- und `command`-Quellen ist die überprüfte Version die `version` in der `plugin.json` der Abhängigkeit. Setzen Sie eine dort, bevor Sie diese Abhängigkeit beschränken, da eine `plugin.json`, die keine Version setzt, keine Beschränkung erfüllt.

Claude Code installiert eine Abhängigkeit mit einer `command`-Quelle nie selbst, daher [installieren Benutzer sie zuerst](/docs/de/plugins/marketplace-reference#command-plugin-source). Es führt auch nie den [`headersHelper`](/docs/de/plugins/host-marketplace#authenticate-archive-downloads) einer Abhängigkeit aus, daher installieren Benutzer auch eine Abhängigkeit, deren Marketplace-Eintrag einen setzt, bevor sie Ihr Plugin installieren.

Neben `claude plugin install` installieren diese Operationen auch alle fehlenden deklarierten Abhängigkeiten, und die `command`- und `headersHelper`-Limits gelten auch für sie:

* `/reload-plugins`
* Automatische Aktualisierung des Marketplaces des abhängigen Plugins
* Erneutes Ausführen von `claude plugin install` auf dem abhängigen Plugin
* `claude plugin marketplace add`

<h2 id="how-dependencies-behave-for-your-users">
  Wie sich Abhängigkeiten für Ihre Benutzer verhalten
</h2>

Diese Abschnitte beschreiben, wie Claude Code die Beschränkungen auflöst, überprüft und kombiniert, die Sie deklarieren, sobald Ihr Plugin neben anderen installiert ist.

<h3 id="how-a-constraint-resolves-against-tags">
  Wie eine Beschränkung gegen Tags aufgelöst wird
</h3>

Wenn ein Benutzer ein Plugin installiert, das `{ "name": "secrets-vault", "version": "~2.1.0" }` deklariert, wird die Abhängigkeit aus dem höchsten `secrets-vault--v`-Tag installiert, der `~2.1.0` im Repository erfüllt, das `secrets-vault` hostet. Wenn kein Tag den Bereich erfüllt, schlägt die Installation fehl oder verwendet die aktuelle Kopie des Marketplaces:

* **Plugin mit eigenem Repository**: Die Installation schlägt mit einer Nachricht fehl, die `Dependency "secrets-vault@your-marketplace" has no git tag satisfying` enthält.
* **Plugin, auf das ein relativer Pfad verweist**: Die Installation verwendet stattdessen die aktuelle Kopie des Marketplaces, und die Beschränkung wird überprüft, wenn das Plugin geladen wird. Wenn diese Kopie außerhalb des Bereichs liegt, bleibt das abhängige Plugin deaktiviert und `claude plugin list` zeigt `Requires "secrets-vault@your-marketplace" ~2.1.0, installed 3.0.0`.

Für ein Plugin, auf das der Marketplace mit einem relativen Pfad verweist, löst ein Marketplace, den Sie als lokalen Ordnerpfad hinzugefügt haben, auch Beschränkungen gegen die Git-Tags dieses Ordners auf, wenn der Ordner ein Git-Repository ist. Dies erfordert Claude Code v2.1.196 oder später. Ein lokaler Ordner, der kein Git-Repository ist, hat keine Tags, daher installiert Claude Code die Abhängigkeit stattdessen aus dem aktuellen Inhalt des Ordners.

<h3 id="confirm-the-resolved-version">
  Aufgelöste Version bestätigen
</h3>

Um zu bestätigen, welche Version eine Beschränkung aufgelöst hat, führen Sie `claude plugin list` in Ihrer Shell aus. Eine Tag-aufgelöste Abhängigkeit zeigt ihre Version mit einem 12-stelligen Commit-Suffix, wie `2.1.0-8713c5b11005`.

Beschränkungsprüfungen verwenden die Version des Tags statt der `version` in `plugin.json`, auch wenn `plugin.json` bei diesem Commit hinterherhinkt.

Wenn Sie ein Tag zu einem anderen Commit verschieben, ruft die nächste Installation den Inhalt dieses Commits ab, anstatt eine veraltete zwischengespeicherte Kopie wiederzuverwenden. Siehe [Versionen und Updates](/docs/de/plugins/loading#versions-and-updates), wie die Version eines Plugins sein Cache-Schlüssel wird.

<h3 id="combine-constraints-from-several-plugins">
  Beschränkungen von mehreren Plugins kombinieren
</h3>

Wenn mehrere installierte Plugins die gleiche Abhängigkeit beschränken, wird die Abhängigkeit zur höchsten Version aufgelöst, die alle ihre Bereiche erfüllt. Häufige Kombinationen werden wie folgt aufgelöst:

| Plugin A erfordert | Plugin B erfordert | Ergebnis                                                                                                                                                    |
| :----------------- | :----------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `^2.0`             | `>=2.1`            | Eine Installation beim höchsten `2.x`-Tag bei oder über `2.1.0`. Beide Plugins werden geladen.                                                              |
| `~2.1`             | `~3.0`             | Die Installation von Plugin B schlägt mit einer `has conflicting version requirements`-Nachricht fehl. Plugin A und die Abhängigkeit bleiben wie sie waren. |
| `=2.1.0`           | keine              | Die Abhängigkeit bleibt bei `2.1.0`. Die automatische Aktualisierung überspringt neuere Versionen, während Plugin A installiert ist.                        |

Die automatische Aktualisierung ruft eine beschränkte Abhängigkeit beim höchsten Git-Tag ab, der jeden Bereich des installierten Plugins erfüllt, anstatt bei der neuesten Version des Marketplaces. Wenn sich die Bereiche der installierten Plugins nicht überlappen, lässt die automatische Aktualisierung diese Abhängigkeit bei ihrer aktuellen Version und die Registerkarte `/plugin` **Errors** zeigt einen Eintrag, der das beschränkende Plugin benennt. Wenn sie sich überlappen, aber kein Tag in den Bereich fällt, ruft die automatische Aktualisierung die aktuelle Kopie des Marketplaces ab und überspringt die Aktualisierung, wenn die `version` dieser Kopie außerhalb des Bereichs eines installierten Plugins liegt.

Wenn ein Benutzer das letzte Plugin deinstalliert, das eine Abhängigkeit beschränkt, wird die Abhängigkeit nicht mehr auf einen Versionsbereich beschränkt und verfolgt bei der nächsten Aktualisierung wieder ihren Marketplace-Eintrag.

<h2 id="see-also">
  Siehe auch
</h2>

* [`claude plugin prune`](/docs/de/plugins/cli-reference#plugin-prune): Entfernen Sie automatisch installierte Abhängigkeiten, die kein Plugin mehr benötigt
* [Marketplace hosten](/docs/de/plugins/host-marketplace): Release-Kanäle und Empfehlung anderer Plugins
