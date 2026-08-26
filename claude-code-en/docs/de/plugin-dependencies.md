> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Versionsbeschränkungen für Plugin-Abhängigkeiten

> Deklarieren Sie Versionsbeschränkungen für Plugin-Abhängigkeiten, und bündeln Sie einen kuratierten Plugin-Satz hinter einer Installation.

Ein Plugin kann von anderen Plugins abhängen, indem es diese in `plugin.json` oder in seinem Marketplace-Eintrag auflistet. Standardmäßig verfolgt eine Abhängigkeit die neueste verfügbare Version, sodass eine vorgelagerte Veröffentlichung die Abhängigkeit Ihres Plugins ohne Warnung ändern kann. Versionsbeschränkungen ermöglichen es Ihnen, eine Abhängigkeit in einem getesteten Versionsbereich zu halten, bis Sie sich entscheiden, sie zu aktualisieren.

Wenn Sie ein Plugin installieren, das Abhängigkeiten deklariert, löst Claude Code diese automatisch auf und installiert sie. Am Ende der Installationsausgabe wird aufgelistet, welche Abhängigkeiten hinzugefügt wurden. Wenn eine Abhängigkeit später fehlt, installieren `/reload-plugins` und die Hintergrund-Plugin-Autoupdate sie neu, sofern ihr Marketplace bereits in Ihren konfigurierten Marketplaces vorhanden ist. Das erneute Ausführen von `claude plugin install` auf dem abhängigen Plugin oder das Hinzufügen eines Marketplace mit `claude plugin marketplace add` löst auch alle ausstehenden fehlenden Abhängigkeiten auf. Abhängigkeiten von einem Marketplace, den Sie nicht hinzugefügt haben, bleiben ungelöst.

Diese Anleitung ist für Plugin-Autoren, die Abhängigkeiten in `plugin.json` deklarieren, und für Marketplace-Verwalter, die Versionen taggen. Um Plugins mit Abhängigkeiten zu installieren, siehe [Plugins entdecken und installieren](/docs/de/discover-plugins). Für das vollständige Manifest-Schema siehe die [Plugins-Referenz](/docs/de/plugins-reference).

<h2 id="why-constrain-dependency-versions">
  Warum Versionsbeschränkungen verwenden
</h2>

Stellen Sie sich einen internen Marketplace vor, auf dem zwei Teams Plugins veröffentlichen. Das Platform-Team verwaltet `secrets-vault`, einen MCP-Server, der ein Secrets-Backend umhüllt. Das Deploy-Team verwaltet `deploy-kit`, das `secrets-vault` aufruft, um während Deployments Anmeldedaten abzurufen.

`deploy-kit` wird gegen `secrets-vault` v2.1.0 getestet. Ohne Versionsbeschränkung führt die nächste Veröffentlichung des Platform-Teams, die ein MCP-Tool umbenennt, dazu, dass die automatische Aktualisierung `secrets-vault` auf die neue Version für jeden Ingenieur aktualisiert und `deploy-kit` bricht.

Mit einer Versionsbeschränkung deklariert `deploy-kit`, dass es `secrets-vault` im Bereich `~2.1.0` benötigt. Ingenieure mit installiertem `deploy-kit` bleiben auf der höchsten passenden `2.1.x`-Patch-Version. Das Deploy-Team aktualisiert nach eigenem Zeitplan, indem es eine neue `deploy-kit`-Version mit einer breiteren Beschränkung veröffentlicht.

<h2 id="declare-a-dependency-with-a-version-constraint">
  Abhängigkeit mit Versionsbeschränkung deklarieren
</h2>

Listet Abhängigkeiten im `dependencies`-Array der `plugin.json` Ihres Plugins auf. Jeder Eintrag ist entweder ein Plugin-Name oder ein Objekt mit einer Versionsbeschränkung.

Das folgende Manifest deklariert eine unversionierte Abhängigkeit und eine beschränkte Abhängigkeit:

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

Ein Eintrag kann ein einfacher String mit nur dem Plugin-Namen sein, wie `"audit-logger"` im obigen Beispiel, das von der Version abhängt, die der Marketplace dieses Plugins bereitstellt. Für mehr Kontrolle verwenden Sie ein Objekt mit diesen Feldern:

| Feld          | Typ    | Beschreibung                                                                                                                                                                                                                                                                                             |
| :------------ | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | string | Plugin-Name. Wird im selben Marketplace wie das deklarierte Plugin aufgelöst. Erforderlich.                                                                                                                                                                                                              |
| `version`     | string | Ein [semver-Bereich](https://github.com/npm/node-semver#ranges) wie `~2.1.0`, `^2.0`, `>=1.4` oder `=2.1.0`. Die Abhängigkeit wird in der höchsten getaggten Version abgerufen, die diesen Bereich erfüllt.                                                                                              |
| `marketplace` | string | Ein anderer Marketplace, um `name` darin aufzulösen. Marketplace-übergreifende Abhängigkeiten sind blockiert, es sei denn, der Ziel-Marketplace ist in [`allowCrossMarketplaceDependenciesOn`](#depend-on-a-plugin-from-another-marketplace) in der `marketplace.json` des Root-Marketplace aufgelistet. |

Das `version`-Feld akzeptiert jeden Ausdruck, der vom `semver`-Paket von Node unterstützt wird, einschließlich Caret-, Tilde-, Bindestrich- und Vergleichsbereiche. Vorabversionen wie `2.0.0-beta.1` sind ausgeschlossen, es sei denn, Ihr Bereich entscheidet sich mit einem Vorabversions-Suffix wie `^2.0.0-0` dafür.

<h2 id="bundle-plugins-for-a-team">
  Plugins für ein Team bündeln
</h2>

Neben dem erforderlichen `name` kann ein Plugin-Manifest nur aus einem `dependencies`-Array bestehen. Die Installation zieht jede Abhängigkeit nach sich, was es zu einer Möglichkeit macht, einen kuratierten Plugin-Satz hinter einer Installation zu verpacken.

Beispielsweise kann ein Plattform-Team rollenspezifische Bundles in einem internen Marketplace veröffentlichen, damit Ingenieure ein `claude plugin install` ausführen, anstatt jedes Tool separat zu installieren:

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

Die Installation von `backend-standard` löst alle vier Abhängigkeiten auf und installiert sie.

Um später ein Tool zum Standard-Set hinzuzufügen, veröffentlichen Sie eine neue `backend-standard`-Version mit der zusätzlichen Abhängigkeit. Auto-Update ist standardmäßig für Nicht-Anthropic-Marketplaces deaktiviert, daher wählen Ingenieure die neue Version auf eine von zwei Arten:

* Aktivieren Sie Auto-Update für den Marketplace in `/plugin`. Das nächste Auto-Update verschiebt das Bundle zur neuen Version und installiert alle Abhängigkeiten, die es hinzufügt.
* Führen Sie `claude plugin update backend-standard` aus, dann `/reload-plugins`, um die neu hinzugefügten Abhängigkeiten zu installieren.

Um Bundles in einer Organisation auszurollen, fügen Sie das Bundle-Plugin zu `enabledPlugins` in [verwalteten Einstellungen](/docs/de/settings#enabledplugins) hinzu.

<h2 id="depend-on-a-plugin-from-another-marketplace">
  Abhängigkeit von einem Plugin aus einem anderen Marketplace
</h2>

Standardmäßig weigert sich Claude Code, eine Abhängigkeit automatisch zu installieren, die sich in einem anderen Marketplace als das Plugin befindet, das sie deklariert. Dies verhindert, dass ein Marketplace stillschweigend Plugins aus einer Quelle abruft, die Sie nicht überprüft haben.

Um dies zu ermöglichen, fügt der Verwalter des Root-Marketplace den Namen des Ziel-Marketplace zu `allowCrossMarketplaceDependenciesOn` in `marketplace.json` hinzu. Der Root-Marketplace ist derjenige, der das Plugin hostet, das der Benutzer installiert. Nur seine Allowlist wird konsultiert, daher vertraut nicht durch zwischengelagerte Marketplaces.

Die folgende `marketplace.json` ermöglicht es `deploy-kit`, von einem Plugin aus `acme-shared` abhängig zu sein:

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "acme-tools",
  "owner": { "name": "Acme" },
  "allowCrossMarketplaceDependenciesOn": ["acme-shared"],
  "plugins": [
    {
      "name": "deploy-kit",
      "source": "./deploy-kit",
      "dependencies": [
        { "name": "audit-logger", "marketplace": "acme-shared" }
      ]
    }
  ]
}
```

Wenn das Feld fehlt oder den Ziel-Marketplace nicht enthält, schlägt die Installation mit einem `cross-marketplace`-Fehler fehl, der das zu setzende Feld benennt. Benutzer können die Abhängigkeit immer noch manuell zuerst installieren, was die Beschränkung erfüllt, ohne die Allowlist zu ändern.

<h2 id="tag-plugin-releases-for-version-resolution">
  Plugin-Versionen für Versionsauflösung taggen
</h2>

Versionsbeschränkungen werden gegen Git-Tags im Marketplace-Repository aufgelöst. Damit Claude Code die verfügbaren Versionen einer Abhängigkeit findet, müssen die Versionen des vorgelagerten Plugins mit einer bestimmten Namenskonvention getaggt werden.

Taggen Sie jede Veröffentlichung als `{plugin-name}--v{version}`, wobei `{version}` dem `version`-Feld in der `plugin.json` dieses Commits entspricht. Führen Sie aus dem Plugin-Verzeichnis aus folgendes aus:

```bash theme={null}
claude plugin tag --push
```

Der Befehl `claude plugin tag` leitet den Tag-Namen aus dem Plugin-Manifest und dem umschließenden Marketplace-Eintrag ab. Vor dem Erstellen des Tags validiert er den Plugin-Inhalt, überprüft, dass `plugin.json` und der Marketplace-Eintrag sich auf die Version einigen, erfordert einen sauberen Arbeitsbaum im Plugin-Verzeichnis und weigert sich, wenn das Tag bereits vorhanden ist. Fügen Sie `--dry-run` hinzu, um zu sehen, was getaggt würde, ohne es zu erstellen. Das direkte Ausführen von `git tag secrets-vault--v2.1.0` ist gleichwertig, wenn Sie `plugin.json` und den Marketplace-Eintrag selbst synchron halten.

Das Plugin-Namen-Präfix ermöglicht es einem Marketplace-Repository, mehrere Plugins mit unabhängigen Versionslinien zu hosten. Der `--v`-Trennzeichen wird als Präfix-Übereinstimmung auf dem vollständigen Plugin-Namen analysiert, sodass Plugin-Namen, die Bindestriche enthalten, korrekt behandelt werden.

Wenn Sie ein Plugin installieren, das `{ "name": "secrets-vault", "version": "~2.1.0" }` deklariert, listet Claude Code die Tags des Marketplace auf, filtert diejenigen, die mit `secrets-vault--v` beginnen, und ruft die höchste Version ab, die `~2.1.0` erfüllt. Wenn kein passender Tag vorhanden ist, wird das abhängige Plugin mit einem Fehler deaktiviert, der die verfügbaren Versionen auflistet.

Ein Marketplace, der als lokaler Ordnerpfad hinzugefügt wird, löst Tags auf die gleiche Weise auf, wenn der Ordner ein Git-Repository ist. Dies erfordert Claude Code v2.1.196 oder später. In zwei Fällen installiert Claude Code die Abhängigkeit stattdessen aus dem aktuellen Inhalt des Ordners:

* Frühere Versionen lesen keine Tags aus einem lokalen Ordner-Marketplace, daher wird eine eingeschränkte Abhängigkeit nur geladen, wenn diese Kopie den Bereich erfüllt.
* Ein lokaler Ordner, der kein Git-Repository ist, hat keine Tags, unabhängig von der Version.

Das aufgelöste Tag's Semver wird separat vom `version`-Feld der `plugin.json` aufgezeichnet, sodass Beschränkungsprüfungen das Tag verwenden, das tatsächlich abgerufen wurde, auch wenn `plugin.json` bei diesem Commit einen veralteten Wert hat. Der Cache-Verzeichnisname für eine Tag-aufgelöste Installation enthält ein 12-stelliges Commit-SHA-Suffix, sodass wenn ein Maintainer ein Tag zu einem anderen Commit verschiebt, die nächste Installation ein frisches Cache-Verzeichnis erhält, anstatt veraltete Inhalte wiederzuverwenden.

<Note>
  Für `npm`-Marketplace-Quellen steuert die Beschränkung nicht, welche Version abgerufen wird, da die Tag-basierte Auflösung nur auf Git-gestützten Quellen gilt. Die Beschränkung wird immer noch zur Ladezeit überprüft, und das abhängige Plugin wird mit `dependency-version-unsatisfied` deaktiviert, wenn die installierte Version sie nicht erfüllt.
</Note>

<h2 id="how-constraints-interact">
  Wie Beschränkungen interagieren
</h2>

Wenn mehrere installierte Plugins dieselbe Abhängigkeit beschränken, schneidet Claude Code ihre Bereiche ab und löst die Abhängigkeit zur höchsten Version auf, die alle erfüllt. Die folgende Tabelle zeigt, wie häufige Kombinationen aufgelöst werden.

| Plugin A erfordert | Plugin B erfordert | Ergebnis                                                                                                                             |
| :----------------- | :----------------- | :----------------------------------------------------------------------------------------------------------------------------------- |
| `^2.0`             | `>=2.1`            | Eine Installation auf dem höchsten `2.x`-Tag bei oder über `2.1.0`. Beide Plugins werden geladen.                                    |
| `~2.1`             | `~3.0`             | Installation von Plugin B schlägt mit `range-conflict` fehl. Plugin A und die Abhängigkeit bleiben wie sie waren.                    |
| `=2.1.0`           | keine              | Die Abhängigkeit bleibt bei `2.1.0`. Die automatische Aktualisierung überspringt neuere Versionen, während Plugin A installiert ist. |

Die automatische Aktualisierung ruft eine beschränkte Abhängigkeit auf dem höchsten Git-Tag ab, der jeden Bereich des installierten Plugins erfüllt, anstatt auf der neuesten Version des Marketplace, sodass die Abhängigkeit weiterhin Updates innerhalb ihres zulässigen Bereichs erhält. Wenn kein Tag alle Bereiche erfüllt, wird die Aktualisierung übersprungen und die Übersprungsmeldung erscheint in `/plugin` Fehler-Registerkarte und benennt das einschränkende Plugin.

Wenn Sie das letzte Plugin deinstallieren, das eine Abhängigkeit beschränkt, wird die Abhängigkeit nicht mehr gehalten und verfolgt bei der nächsten Aktualisierung wieder ihren Marketplace-Eintrag.

<h2 id="enable-or-disable-a-plugin-with-dependencies">
  Plugin mit Abhängigkeiten aktivieren oder deaktivieren
</h2>

Das Aktivieren eines Plugins aktiviert auch die Plugins, von denen es abhängt, und das Deaktivieren eines Plugins wird blockiert, wenn ein anderes aktiviertes Plugin es immer noch benötigt. Beide Verhaltensweisen erfordern Claude Code v2.1.143 oder später. Frühere Versionen aktivieren oder deaktivieren nur das benannte Plugin und zeigen einen `dependency-unsatisfied`-Fehler beim nächsten Laden an.

Wenn Sie ein Plugin aktivieren, aktiviert Claude Code auch seine Abhängigkeiten im selben Bereich. Wenn eine Abhängigkeit ihre eigenen Abhängigkeiten hat, aktiviert Claude Code auch diese. Die Erfolgsmeldung listet auf, was sonst noch zusammen mit dem Plugin, das Sie benannt haben, aktiviert wurde. Wenn eine Abhängigkeit nicht aktiviert werden kann, weigert sich der Befehl und teilt Ihnen mit, was blockiert und wie Sie es beheben können:

| Bedingung                                                                                            | Ergebnis                                                                                                                               |
| :--------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------- |
| Eine Abhängigkeit ist nicht installiert                                                              | Die Aktivierung schlägt fehl und druckt den `claude plugin install`-Befehl für jede fehlende Abhängigkeit.                             |
| Eine Abhängigkeit wird durch die Plugin-Richtlinie Ihrer Organisation blockiert                      | Die Aktivierung schlägt fehl und benennt die blockierte Abhängigkeit.                                                                  |
| Eine Abhängigkeit ist auf `false` in einem Bereich mit höherer Priorität als der Zielbereich gesetzt | Die Aktivierung schlägt fehl. Aktivieren Sie die Abhängigkeit in diesem Bereich, oder übergeben Sie `--scope`, um dort zu schreiben.   |
| Alle Abhängigkeiten sind installiert und zulässig                                                    | Die Aktivierung ist erfolgreich und schreibt `true` für das Plugin und jede Abhängigkeit, die im Zielbereich noch nicht aktiviert war. |

Dies gilt auch, wenn eine Abhängigkeit [`defaultEnabled: false`](/docs/de/plugins-reference#default-enablement) in ihrem Manifest setzt, da Claude Code ein explizites `true` dafür schreibt. Dasselbe gilt bei der Installation: Eine Abhängigkeit, die zur Erfüllung eines aktiven Plugins hinzugezogen wird, wird mit `true` installiert, unabhängig von ihrem eigenen Standard.

Wenn Sie ein Plugin deaktivieren, weigert sich Claude Code, wenn ein anderes aktiviertes Plugin immer noch davon abhängt. Der Fehler benennt die Plugins, die davon abhängen, und gibt Ihnen einen verketteten Befehl, der sie in der richtigen Reihenfolge deaktiviert, endend mit dem, das Sie angefordert haben.

Wenn beispielsweise `deploy-kit` von `secrets-vault` abhängt, schlägt das Deaktivieren von `secrets-vault` allein mit einer Ausgabe ähnlich der folgenden fehl:

```text theme={null}
secrets-vault is still required by deploy-kit. Disable that plugin first, or
disable everything together: claude plugin disable deploy-kit@acme-tools && claude plugin disable secrets-vault@acme-tools
```

Kopieren Sie den verketteten Befehl aus dem Fehler, um den vollständigen Satz in einem Schritt zu deaktivieren.

<h2 id="remove-orphaned-auto-installed-dependencies">
  Verwaiste automatisch installierte Abhängigkeiten entfernen
</h2>

Automatisch installierte Abhängigkeiten bleiben auf der Festplatte, nachdem die Plugins, die sie installiert haben, deinstalliert werden, falls Sie ein abhängiges Plugin neu installieren oder die Abhängigkeit direkt weiterhin verwenden möchten. Um sie zu bereinigen, führen Sie `claude plugin prune` aus, um die automatisch installierten Abhängigkeiten aufzulisten, die kein installiertes Plugin mehr benötigt, und entfernen Sie sie nach einer Bestätigungsaufforderung. Dies erfordert Claude Code v2.1.121 oder später.

```bash theme={null}
claude plugin prune
```

Standardmäßig arbeitet prune im Benutzerbereich. Verwenden Sie `--scope project` oder `--scope local`, um einen anderen Bereich anzuvisieren. Übergeben Sie `--dry-run`, um aufzulisten, was entfernt würde, ohne etwas zu ändern. Übergeben Sie `-y`, um die Bestätigungsaufforderung zu überspringen. Wenn stdin oder stdout kein Terminal ist, listet prune die verwaisten Abhängigkeiten auf und beendet sich, ohne sie zu entfernen, es sei denn, `-y` wird übergeben.

Um als Teil einer Deinstallation zu bereinigen, übergeben Sie `--prune` an `claude plugin uninstall`. Nach dem Entfernen des benannten Plugins scannt Claude Code nach automatisch installierten Abhängigkeiten, die jetzt verwaist sind, und entfernt sie. Plugins, die Sie selbst installiert haben, werden niemals bereinigt, nur diejenigen, die automatisch durch das `dependencies`-Array eines anderen Plugins installiert wurden.

Um beispielsweise `deploy-kit` zu deinstallieren und die Abhängigkeiten zu bereinigen, die es hinterlässt:

```bash theme={null}
claude plugin uninstall deploy-kit --prune
```

<h2 id="resolve-dependency-errors">
  Abhängigkeitsfehler beheben
</h2>

Abhängigkeitsprobleme erscheinen in `claude plugin list` und in der `/plugin`-Schnittstelle. Claude Code deaktiviert das betroffene Plugin, bis Sie den Fehler beheben. Die Tabelle unten listet die häufigsten Fehler und deren Behebung auf.

| Fehler                           | Bedeutung                                                                                                                                                                                                                                                              | Wie zu beheben                                                                                                                                                                                                                                                                                                                           |
| :------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `dependency-unsatisfied`         | Eine deklarierte Abhängigkeit ist nicht installiert, oder sie ist installiert, aber deaktiviert.                                                                                                                                                                       | Führen Sie den `claude plugin install`-Befehl aus, der in der Fehlermeldung angezeigt wird. Wenn der Marketplace der Abhängigkeit noch nicht konfiguriert ist, fügen Sie ihn mit `claude plugin marketplace add` hinzu und Claude Code löst die Abhängigkeit automatisch auf. Wenn die Abhängigkeit deaktiviert ist, aktivieren Sie sie. |
| `range-conflict`                 | Die Versionsanforderungen für eine Abhängigkeit können nicht kombiniert werden. Die Fehlermeldung benennt die Ursache: Keine Version erfüllt alle Bereiche, ein Bereich ist keine gültige Semver-Syntax, oder die kombinierten Bereiche sind zu komplex zum Schneiden. | Deinstallieren oder aktualisieren Sie eines der in Konflikt stehenden Plugins, beheben Sie alle ungültigen `version`-Strings, vereinfachen Sie lange `\|\|`-Ketten, oder bitten Sie den vorgelagerten Autor, seine Beschränkung zu erweitern.                                                                                            |
| `dependency-version-unsatisfied` | Die Version der installierten Abhängigkeit liegt außerhalb des deklarierten Bereichs dieses Plugins.                                                                                                                                                                   | Führen Sie `claude plugin install <dependency>@<marketplace>` aus, um die Abhängigkeit gegen alle aktuellen Beschränkungen neu aufzulösen.                                                                                                                                                                                               |
| `no-matching-tag`                | Das Repository der Abhängigkeit hat kein `{name}--v*`-Tag, das den Bereich erfüllt.                                                                                                                                                                                    | Überprüfen Sie, dass die vorgelagerte Stelle Versionen mit der obigen Konvention getaggt hat, oder lockern Sie Ihren Bereich.                                                                                                                                                                                                            |

Um diese Fehler programmgesteuert zu überprüfen, führen Sie `claude plugin list --json` aus und lesen Sie das `errors`-Feld auf jedem Plugin.

<h2 id="see-also">
  Siehe auch
</h2>

* [Plugins erstellen](/docs/de/plugins): Erstellen Sie Plugins mit Skills, Agents und Hooks
* [Plugin-Marketplace erstellen und verteilen](/docs/de/plugin-marketplaces): Hosten Sie Plugins für Ihr Team
* [Plugins-Referenz](/docs/de/plugins-reference#plugin-manifest-schema): Das vollständige `plugin.json`-Schema
* [Versionsverwaltung](/docs/de/plugins-reference#version-management): Wie die eigene Version eines Plugins aufgelöst wird und als Cache-Schlüssel verwendet wird
