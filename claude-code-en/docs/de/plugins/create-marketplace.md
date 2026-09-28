> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Einen Marketplace erstellen

> Erstellen Sie einen Plugin-Marketplace aus einer marketplace.json-Datei und testen Sie ihn lokal, bevor Sie ihn hosten.

Ein Plugin-Marketplace ist ein Verzeichnis oder Repository mit einer `.claude-plugin/marketplace.json`-Datei, die Ihre Plugins auflistet und angibt, wo jedes abgerufen werden kann. Sie pushen das Verzeichnis zu einem Git-Host, und jeder mit Zugriff registriert es in Claude Code mit einem Befehl und installiert Ihre Plugins daraus.

Erstellen Sie Ihren eigenen Marketplace, wenn Sie möchten, dass eine von Ihnen gewählte Gruppe, z. B. Ihr Team oder Ihre Organisation, Ihre Plugins installiert und weiterhin Updates aus einem von Ihnen kontrollierten Katalog erhält. Das Repository kann privat sein, es kann so viele Plugins auflisten, wie Sie möchten, und ein Administrator kann [es auf jedem Computer erforderlich machen](/docs/de/plugins/org).

<Note>
  Diese Fälle werden auf anderen Seiten behandelt:

  * **Ein Plugin mit wenigen Personen teilen**: Senden Sie ihnen das Plugin-Verzeichnis oder eine `.zip`-Datei davon. Siehe [Ein Plugin ohne Marketplace teilen](/docs/de/plugins/publish#share-a-plugin-without-a-marketplace).
  * **Ein Plugin für alle anbieten**: Reichen Sie es im Community-Marketplace von Anthropic ein. Siehe [Im Community-Marketplace einreichen](/docs/de/plugins/publish#submit-to-the-community-marketplace).
  * **Ein Plugin selbst verwenden**: Laden Sie es mit `--plugin-dir` oder speichern Sie es in Ihrem Skills-Verzeichnis. Siehe [Entwickeln ohne Marketplace](/docs/de/plugins/create#develop-without-a-marketplace).
</Note>

Beginnen Sie mit [Einen Marketplace erstellen](#create-a-marketplace), um einen auf Ihrem eigenen Computer zu erstellen und ein Plugin daraus zu installieren, dann [fügen Sie weitere Plugin-Einträge hinzu](#add-plugin-entries).

<h2 id="create-a-marketplace">
  Erstellen Sie einen Marketplace
</h2>

Die folgenden Schritte erstellen einen Marketplace auf Ihrem Computer, fügen ein Plugin hinzu, registrieren ihn in Claude Code und installieren das Plugin daraus. Das ist die gesamte Schleife, und es ist die gleiche Schleife, die Ihre Benutzer durchlaufen, sobald Sie den Marketplace irgendwo hosten, wo sie ihn erreichen können. Führen Sie jeden Befehl in Ihrer Shell aus, aus dem Verzeichnis, in dem Sie `my-marketplace/` erstellen möchten.

Sie benötigen ein Plugin zum Auflisten. Das Beispiel verwendet `my-first-plugin` aus [Create your first plugin](/docs/de/plugins/create#create-your-first-plugin), ein Plugin mit einer Skill, die Sie als `/my-first-plugin:hello` ausführen; erstellen Sie es zuerst, wenn Sie noch kein Plugin haben. Um stattdessen ein eigenes Plugin zu verwenden, ersetzen Sie sein Verzeichnis und seinen `name` überall dort, wo die Schritte `my-first-plugin` sagen. Für das, was ein Plugin-Verzeichnis enthalten kann, siehe den [plugin directory explorer](/docs/de/plugins/components#explore-the-plugin-directory).

<Steps>
  <Step title="Richten Sie das Marketplace-Verzeichnis ein">
    Ein Marketplace ist ein Verzeichnis mit einer `.claude-plugin/marketplace.json`-Datei plus den Plugins, die es auflistet. Erstellen Sie das Marketplace-Verzeichnis und seinen `.claude-plugin/`-Ordner, kopieren Sie dann Ihr Plugin unter `plugins/`:

    ```bash theme={null}
    mkdir -p my-marketplace/.claude-plugin my-marketplace/plugins
    cp -r my-first-plugin my-marketplace/plugins/
    ```

    Überprüfen Sie, dass das Plugin an seinem neuen Ort gültig ist, damit jeder spätere Fehler über den Marketplace und nicht über das Plugin ist:

    ```bash theme={null}
    claude plugin validate ./my-marketplace/plugins/my-first-plugin
    ```

    Die letzte Zeile der Ausgabe lautet `✔ Validation passed`.
  </Step>

  <Step title="Erstellen Sie die Marketplace-Datei">
    Speichern Sie `marketplace.json` unter `my-marketplace/.claude-plugin/marketplace.json`. Die Datei erfordert einen `name`, einen `owner` und ein `plugins`-Array.

    Jedes Objekt in `plugins` ist ein Plugin-Eintrag und benötigt einen `name` und eine `source`. Schreiben Sie die `source` des Eintrags als Pfad vom Marketplace-Root. Der Root ist `my-marketplace/`, das Verzeichnis, das `.claude-plugin/` enthält.

    ```json my-marketplace/.claude-plugin/marketplace.json theme={null}
    {
      "name": "my-marketplace",
      "description": "Plugins for my team",
      "owner": {
        "name": "Your Name"
      },
      "plugins": [
        {
          "name": "my-first-plugin",
          "source": "./plugins/my-first-plugin",
          "description": "A greeting plugin to learn the basics"
        }
      ]
    }
    ```
  </Step>

  <Step title="Validieren Sie den Marketplace">
    Führen Sie `claude plugin validate` im Marketplace-Verzeichnis aus, um die JSON-Syntax, die erforderlichen Felder und jeden Plugin-Eintrag in seiner `.claude-plugin/marketplace.json` zu überprüfen.

    ```bash theme={null}
    claude plugin validate ./my-marketplace
    ```

    Für die Datei wie in Schritt 2 geschrieben, lautet die letzte Zeile der Ausgabe `✔ Validation passed`.
  </Step>

  <Step title="Fügen Sie den Marketplace hinzu und installieren Sie das Plugin">
    Registrieren Sie das Verzeichnis als Marketplace.

    ```bash theme={null}
    claude plugin marketplace add ./my-marketplace
    ```

    Der Befehl gibt `✔ Successfully added marketplace: my-marketplace (declared in user settings)` aus, was bedeutet, dass der Marketplace in Ihrer Benutzereinstellungsdatei aufgezeichnet ist.

    Installieren Sie das Plugin. Die Installations-ID ist der `name` des Eintrags, ein `@` und der Marketplace-`name`.

    ```bash theme={null}
    claude plugin install my-first-plugin@my-marketplace
    ```

    Der Befehl gibt `✔ Successfully installed plugin: my-first-plugin@my-marketplace (scope: user)` aus.

    Innerhalb einer Sitzung registriert `/plugin marketplace add ./my-marketplace` den Marketplace auf die gleiche Weise. `/plugin install my-first-plugin@my-marketplace` öffnet die Details des Plugins im `/plugin`-Panel, wo Sie es installieren. Für diesen Ablauf siehe [Install and manage plugins](/docs/de/plugins/install).
  </Step>

  <Step title="Bestätigen Sie, dass das Plugin geladen wurde">
    Installierte Plugins auflisten.

    ```bash theme={null}
    claude plugin list
    ```

    Die Ausgabe listet `my-first-plugin@my-marketplace` mit `Status: ✔ enabled` auf.

    Um zu sehen, was das Plugin geladen hat, zeigen Sie seine Details an.

    ```bash theme={null}
    claude plugin details my-first-plugin
    ```

    Der Abschnitt `Component inventory` lautet `Skills (1)  hello`.

    Um die Skill auszuführen, starten Sie eine Sitzung und geben Sie `/my-first-plugin:hello` ein. Claude grüßt Sie. Der Befehl hat den Namen des Plugins als Präfix, wie es jeder Skill-Name eines Plugins tut.
  </Step>
</Steps>

<h2 id="add-plugin-entries">
  Plugin-Einträge hinzufügen
</h2>

Jedes Plugin, das Sie verteilen, ist ein Objekt im `plugins`-Array von `marketplace.json`. Um ein zweites Plugin hinzuzufügen, fügen Sie ein zweites Objekt hinzu. Diese Felder decken die meisten Einträge ab:

* `name`: der Bezeichner, den Personen vor `@` eingeben, wenn sie installieren. Er kann keine Leerzeichen enthalten.
* `source`: wo Claude Code das Plugin abruft. Schreiben Sie einen relativen Pfad-String für ein Plugin im Marketplace-Verzeichnis, wie in [der Anleitung](#create-a-marketplace), oder ein Quellobjekt für ein Plugin außerhalb davon. Siehe [Wählen Sie eine Plugin-Quelle](#choose-a-plugin-source).
* `description`: die Zeile, die Personen neben dem Plugin sehen, wenn sie Ihren Marketplace in `/plugin` durchsuchen.

Für die vollständige Feldliste siehe [Plugin-Einträge](/docs/de/plugins/marketplace-reference#plugin-entries).

Ein Eintrag kann auch jedes [`plugin.json`](/docs/de/plugins/manifest-reference)-Feld setzen. Für den Fall, dass die `plugin.json`-Felder eines Eintrags für ein Plugin gelten, das sein eigenes `plugin.json` hat, siehe [Eintrag und plugin.json](/docs/de/plugins/marketplace-reference#entry-and-plugin-json).

<h2 id="rules-for-plugin-entries">
  Regeln für Plugin-Einträge
</h2>

Die meisten fehlgeschlagenen Installationen von einem neuen Marketplace stammen von einem relativen Pfad, der aus dem falschen Verzeichnis geschrieben wurde, oder von einem Eintrags-Namen, der sich vom `name` in der `plugin.json` des Plugins unterscheidet.

<h3 id="write-relative-paths-from-the-marketplace-root">
  Schreiben Sie relative Pfade vom Marketplace-Root
</h3>

Der Marketplace-Root ist das Verzeichnis, das `.claude-plugin/` enthält. In [der Anleitung](#create-a-marketplace) ist das `my-marketplace/`, also ist die `source` des Eintrags `"./plugins/my-first-plugin"`. Der Pfad beginnt nicht innerhalb von `.claude-plugin/`, verwenden Sie also nicht `..`, um ihn zu verlassen.

Ein Pfad mit `..` und ein Pfad zu einem fehlenden Verzeichnis schlagen bei verschiedenen Befehlen fehl:

* **Ein Pfad mit `..`**: `claude plugin validate` meldet den Eintrag als ungültig. Die Nachricht beginnt mit `Path contains "..": ./../plugins/my-first-plugin`.
* **Ein Pfad zu einem Verzeichnis, das nicht existiert**: `claude plugin validate` besteht. `claude plugin install` schlägt mit `Source path does not exist: <path>` fehl, und `<path>` ist der absolute Ort, den Claude Code überprüft hat.

<h3 id="keep-the-entry-name-and-the-manifest-name-the-same">
  Halten Sie den Eintrags-Namen und den Manifest-Namen gleich
</h3>

Ein Marketplace-Plugin hat einen Eintrags-`name` in `marketplace.json` und einen `name` in seiner eigenen `plugin.json`, genannt der Manifest-Name. Jeder Name erscheint an verschiedenen Stellen:

* **Eintrags-Name**: die Installations-ID, `<entry-name>@<marketplace>`. Es ist das, was Personen eingeben, um zu installieren, was `claude plugin list` anzeigt, und der Schlüssel, den Claude Code unter [`enabledPlugins`](/docs/de/settings-reference#enabledplugins) in ihrer Einstellungsdatei schreibt.
* **Manifest-Name**: das Präfix auf den Skills des Plugins und der Name, den `claude plugin details` nimmt.

Wenn sich die beiden Namen unterscheiden und jemand nach dem Manifest-Namen installiert, meldet Claude Code `Plugin "<manifest-name>" not found in marketplace "<marketplace>"`. Halten Sie die beiden Namen gleich. Für mehr darüber, wie Claude Code die beiden Namen verwendet, siehe [Plugin-Lade-Referenz](/docs/de/plugins/loading#find-where-a-plugin-came-from).

<h2 id="choose-a-plugin-source">
  Wählen Sie eine Plugin-Quelle
</h2>

Jeder Plugin-Eintrag in `marketplace.json` hat eine `source`, die Claude Code mitteilt, wo dieses eine Plugin abgerufen werden kann. Wählen Sie die Quelle danach aus, wo die Plugin-Dateien gespeichert sind. Die Tabelle listet die Quellen auf, die die meisten Marketplace-Besitzer verwenden.

| Quelle         | Verwenden Sie es, wenn                                                               | Minimaler `source`-Wert                                                                   |
| :------------- | :----------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------- |
| Relativer Pfad | Die Plugin-Dateien befinden sich im Marketplace-Verzeichnis selbst                   | `"./plugins/my-first-plugin"`                                                             |
| `github`       | Das Plugin ist sein eigenes GitHub-Repository                                        | `{ "source": "github", "repo": "your-org/my-first-plugin" }`                              |
| `git-subdir`   | Das Plugin ist ein Unterverzeichnis eines anderen Repositorys, z. B. eines Monorepos | `{ "source": "git-subdir", "url": "your-org/monorepo", "path": "tools/my-first-plugin" }` |

In einer `git-subdir`-Quelle nimmt `url` eine Git-URL oder eine `owner/repo`-GitHub-Kurzform.

Ein Plugin kann auch aus einem dieser Quellentypen stammen:

* `url`: ein Git-Repository nach URL, auf jedem Host
* `archive`: eine Zip-Datei, die über HTTPS heruntergeladen wird
* `npm`: ein npm-Paket
* `command`: ein Verzeichnis, das durch Ausführen eines Befehls auf dem Computer erstellt wird, auf dem das Plugin installiert ist

Für die Felder jedes Quellentyps und zum Anheften einer Git-basierten Quelle an einen `ref` oder `sha` siehe [Plugin-Quellen](/docs/de/plugins/marketplace-reference#plugin-sources).

<h2 id="validate-and-test">
  Validieren und testen
</h2>

Wenn Sie Plugins hinzufügen, führen Sie `claude plugin validate ./my-marketplace` in Ihrer Shell nach jeder Bearbeitung aus und installieren Sie aus dem Marketplace auf Ihrem eigenen Computer, bevor Sie ihn teilen. Validierung und Installation fangen verschiedene Probleme auf.

<h3 id="problems-that-validation-reports">
  Probleme, die die Validierung meldet
</h3>

`claude plugin validate` liest nur Dateien im Marketplace-Verzeichnis. Es meldet:

* JSON-Syntaxfehler, als `json: Invalid JSON syntax: <reason>`
* Fehlende erforderliche Felder, z. B. `owner: Invalid input`
* Ein Marketplace-Name mit Leerzeichen, Nicht-ASCII-Zeichen oder eine Form, die einen offiziellen Anthropic-Marketplace imitiert, z. B. `claude-official`
* Ein relativer `source`, der `..` enthält
* Unbekannte Felder auf der obersten Ebene oder in einem Plugin-Eintrag, als Warnungen
* Probleme in der `plugin.json` jedes Plugins mit relativem Pfad, als `plugins[N] plugin.json → <field>: <message>`

Für jede Nachricht, die `validate` drucken kann, siehe [Validierungsmeldungen](/docs/de/plugins/marketplace-reference#validation-messages). Für seine Flags und Exit-Codes siehe [`plugin validate`](/docs/de/plugins/cli-reference#plugin-validate).

<h3 id="problems-that-surface-when-you-add-or-install">
  Probleme, die beim Hinzufügen oder Installieren auftauchen
</h3>

Probleme, die `claude plugin validate` nicht meldet, erscheinen, wenn Sie den Marketplace hinzufügen oder daraus installieren:

* **Wenn Sie den Marketplace hinzufügen**: die genauen [offiziellen Marketplace-Namen](/docs/de/plugins/marketplace-reference#reserved-names), z. B. `claude-plugins-official`, bestehen die Validierung. Wenn Sie einen Marketplace mit einem dieser Namen hinzufügen, weigert sich Claude Code mit einer Nachricht, die mit `The name '<name>' is reserved for official Anthropic marketplaces` beginnt.
* **Wenn Sie ein Plugin installieren**:
  * Claude Code ruft zuerst eine `github`-, `git-subdir`- oder andere Remote-Quelle ab, wenn Sie das Plugin installieren, also erscheint ein falscher `repo` oder `path` dann.
  * Ein relativer `source`, dessen Verzeichnis nicht existiert, schlägt auch bei der Installation fehl, mit `Source path does not exist: <path>`.

<h3 id="test-an-edit-to-a-plugin">
  Testen Sie eine Bearbeitung eines Plugins
</h3>

In [der Anleitung](#create-a-marketplace) haben Sie `my-marketplace` aus einem lokalen Verzeichnis mit einer relativen Pfad-`source` hinzugefügt. Mit diesem Setup liest Claude Code die Plugin-Dateien direkt aus `my-marketplace/plugins/`. Ihre Bearbeitungen treten beim nächsten Sitzungsstart oder wenn Sie `/reload-plugins` in einer Sitzung ausführen, ohne Änderung der Plugin-`version` in Kraft.

Personen, die aus Ihrem gehosteten Marketplace installieren, erhalten stattdessen eine Kopie im Plugin-Cache. Für wie sie eine neue Version erhalten, siehe [Halten Sie Benutzer auf dem Laufenden](/docs/de/plugins/host-marketplace#keep-users-up-to-date).

<h3 id="remove-the-marketplace-to-start-over">
  Entfernen Sie den Marketplace, um von vorne zu beginnen
</h3>

Um alles zu entfernen und von vorne zu beginnen, führen Sie `claude plugin marketplace remove my-marketplace` in Ihrer Shell aus. Der Befehl entfernt den Marketplace und deinstalliert seine Plugins.

<h2 id="host-your-marketplace">
  Hosten Sie Ihren Marketplace
</h2>

Sobald Sie ein Plugin aus dem Marketplace auf Ihrem eigenen Computer installieren können, wie in [Einen Marketplace erstellen](#create-a-marketplace), pushen Sie das Marketplace-Verzeichnis zu einem Git-Host.

Ihre Teamkollegen führen dann `claude plugin marketplace add <owner>/<repo>` in ihrer Shell für ein GitHub-Repository aus, oder den gleichen Befehl mit der Repository-URL. Sie installieren dann ein Plugin nach Name wie in [der Anleitung](#create-a-marketplace).

Für privaten Repository-Zugriff, Updates, Versionierung und Umbenennung oder Entfernung von Einträgen siehe [Hosten und Verwalten eines Marketplaces](/docs/de/plugins/host-marketplace).

<h2 id="next-steps">
  Nächste Schritte
</h2>

* [Hosten und Verwalten eines Marketplaces](/docs/de/plugins/host-marketplace): Wählen Sie einen Host, halten Sie Benutzer auf dem Laufenden und benennen Sie Plugins sicher um oder entfernen Sie sie
* [Marketplace-Referenz](/docs/de/plugins/marketplace-reference): `marketplace.json`-Felder und Quellentypen
* [Verwalten Sie Plugins für Ihre Organisation](/docs/de/plugins/org): Erforderlich machen Sie Ihren Marketplace und seine Plugins auf jedem Computer
* [Schlagen Sie Plugins nach Relevanz vor](/docs/de/plugins/relevance): Lassen Sie Claude Code ein Plugin aus Ihrem Marketplace vorschlagen, wenn eine Sitzung übereinstimmt
