> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins fehlerbeheben

> Beheben Sie Plugin-Fehler in Claude Code. Finden Sie die genaue Meldung, die Sie gesehen haben, gruppiert nach Phase von der Ausführung von /plugin über die Installation bis zur Organisationsrichtlinie.

Diese Seite listet Fehlermeldungen und Symptome für Claude Code-Plugins und für Marktplätze auf – die Kataloge, aus denen Claude Code Plugins installiert. Jeder Eintrag gibt die Ursache, eine Lösung und das, was Sie sehen, sobald die Lösung funktioniert.

Wenn eine Meldung ein Plugin oder einen Marktplatz benennt, zeigt der Eintrag einen Platzhalter wie `<name>` statt dessen an.

Verwenden Sie diese Seite, ob Sie Plugins installieren, sie erstellen, einen Marktplatz hosten oder Plugins für eine Organisation verwalten.

<Note>
  Diese Fälle werden auf anderen Seiten behandelt:

  * **Warum Bereiche, der Cache und die Priorität sich so verhalten**: lesen Sie [Plugin-Ladungsreferenz](/docs/de/plugins/loading)
  * **Nachschlagen eines Flags, Feldes oder Befehls**: verwenden Sie die [Plugin-Befehle-Referenz](/docs/de/plugins/cli-reference), die [Manifest-Referenz](/docs/de/plugins/manifest-reference) oder die [Marktplatz-Referenz](/docs/de/plugins/marketplace-reference)
</Note>

Suchen Sie nach der genauen Meldung, die Sie gesehen haben. Jede Meldung wird unter der Phase aufgelistet, die sie erzeugt, was nicht immer der Befehl ist, den Sie ausgeführt haben. Zum Beispiel kann eine Installation fehlschlagen, weil ein Marktplatz fehlt, daher wird diese Meldung unter [Einen Marktplatz hinzufügen](#add-a-marketplace) aufgelistet.

<h2 id="find-where-/plugin-runs">
  Finden Sie, wo `/plugin` ausgeführt wird
</h2>

`/plugin` ist ein Befehl, den Sie in einer laufenden Claude Code-Terminalsitzung eingeben, und er öffnet ein interaktives Panel. Die Einträge in diesem Abschnitt behandeln die Orte, an denen Sie ihn eingeben können, aber er kann nicht ausgeführt werden, und die Befehlsschreibweisen, die nicht vorhanden sind.

<h3 id="plugin-isnt-available-in-this-environment">
  `/plugin isn't available in this environment`
</h3>

Sie haben `/plugin` irgendwo anders als in einer Claude Code-Terminalsitzung eingegeben, und Claude hat stattdessen diese Zeile geantwortet, ohne etwas zu öffnen.

Sie erhalten diese Antwort in einer Sitzung, die kein Terminal zum Zeichnen des `/plugin`-Panels hat: [nicht-interaktiver Modus](/docs/de/headless) mit `claude -p`, das Agent SDK, die Code-Registerkarte der Claude-Desktop-App, das VS Code-Erweiterungspanel und der Browser unter claude.ai/code.

Im VS Code-Erweiterungspanel erhält nur eine `/plugin`-Zeile mit etwas danach, wie `/plugin install <plugin>@<marketplace>`, diese Antwort. `/plugin` oder `/plugins` allein eingegeben öffnet den Dialog **Plugins verwalten**.

Installieren Sie das Plugin von der Oberfläche, auf der Sie sich befinden:

* **Claude-Desktop-App, lokale oder SSH-Sitzung**: klicken Sie auf die Schaltfläche **+** neben der Eingabeaufforderung, dann **Plugins**, dann **Plugin hinzufügen**, um den [Plugin-Browser](/docs/de/desktop#install-plugins) zu öffnen
* **VS Code-Erweiterung**: verwenden Sie die Registerkarte **VS Code** unter [Ein Plugin installieren](/docs/de/plugins/install#install-a-plugin)
* **Claude Code im Web oder eine Desktop-Cloud-Sitzung**: eine Cloud-Sitzung hat keinen Plugin-Browser. Siehe die Registerkarte **Cloud-Sitzung** unter [Ein Plugin installieren](/docs/de/plugins/install#install-a-plugin), um zu sehen, was eine Cloud-Sitzung lädt
* **Ein Terminal, auf das Sie Zugriff haben**: führen Sie `claude` aus und geben Sie dort `/plugin` ein, oder führen Sie `claude plugin install <plugin>@<marketplace>` in Ihrer Shell aus, ohne eine Sitzung zu starten

Wenn eine Terminal-Installation funktioniert, druckt `/plugin` eine Installationszusammenfassung, die mit `✓ Installed <plugin>.` beginnt, und `claude plugin install` druckt `Successfully installed plugin: <plugin>@<marketplace>`.

<h3 id="zsh-no-such-file-or-directory-plugin">
  `zsh: no such file or directory: /plugin`
</h3>

Sie haben `/plugin ...` an einer Shell-Eingabeaufforderung eingegeben, und die Shell hat gemeldet, dass keine Datei namens `/plugin` vorhanden ist. Bash meldet `bash: /plugin: No such file or directory`.

`/plugin` ist ein Befehl, den Sie in einer Claude Code-Sitzung eingeben, nicht an der Shell-Eingabeaufforderung. Starten Sie eine Sitzung und geben Sie dort denselben Befehl ein:

```shell theme={null}
claude
```

Dann an der Claude Code-Eingabeaufforderung:

```text theme={null}
/plugin install <plugin>@<marketplace>
```

Eine erfolgreiche Installation druckt eine Zusammenfassung, die mit `✓ Installed <plugin>.` beginnt. Wenn die Installation selbst dann fehlschlägt, befindet sich ihre Meldung unter [Einen Marktplatz hinzufügen](#add-a-marketplace) oder [Ein Plugin installieren](#install-a-plugin).

Um von der Shell aus ohne Starten einer Sitzung zu installieren, führen Sie stattdessen `claude plugin install <plugin>@<marketplace>` aus.

<h3 id="the-term-plugin-is-not-recognized-as-the-name-of-a-cmdlet">
  `The term '/plugin' is not recognized as the name of a cmdlet`
</h3>

Sie haben `/plugin ...` an einer PowerShell-Eingabeaufforderung eingegeben, und `/plugin` ist ein Claude Code-Befehl, kein Programm. Bash und Zsh melden [ihre eigene Form dieses Fehlers](#zsh-no-such-file-or-directory-plugin).

Verwenden Sie stattdessen eine dieser Optionen:

* Führen Sie `claude` aus, geben Sie dann `/plugin` an der Claude Code-Eingabeaufforderung ein
* Führen Sie `claude plugin install <plugin>@<marketplace>` in PowerShell aus, ohne eine Sitzung zu starten

<h3 id="claude-command-not-found-after-claude-plugin">
  `claude: command not found` nach `claude plugin ...`
</h3>

Sie haben `claude plugin install ...` in Ihrer Shell ausgeführt, und die Shell konnte `claude` überhaupt nicht finden. Unter Windows lautet die Meldung `'claude' is not recognized as the name of a cmdlet` oder `'claude' is not recognized as an internal or external command`.

Die Ursache liegt nicht am Plugin-Befehl. Entweder ist Claude Code nicht installiert, oder sein Installationsverzeichnis befindet sich nicht auf Ihrem `PATH` in dieser Shell. Folgen Sie [`command not found: claude` nach der Installation](/docs/de/troubleshoot-install#command-not-found-claude-after-installation), und versuchen Sie dann den Plugin-Befehl erneut.

<h3 id="unknown-command-and-command-spellings-that-dont-exist">
  `Unknown command` und Befehlsschreibweisen, die nicht vorhanden sind
</h3>

Sie haben einen Plugin-Befehl eingegeben, den Sie irgendwo gesehen haben, und erhielten `Unknown command: /<name>` in einer Sitzung oder `error: unknown command '<name>'` oder `error: unknown option '<flag>'` vom `claude`-Binär in Ihrer Shell.

Es gibt mehrere Befehlsschreibweisen, die Claude Code nicht hat. Die folgende Tabelle ordnet jede einer echten Befehl zu. Die [Plugin-Befehlsreferenz](/docs/de/plugins/cli-reference) listet jeden Unterbefehl und jedes Flag auf.

| Sie haben eingegeben                       | Was Claude Code sagt                                                         | Verwenden Sie stattdessen                                                                                                                                      |
| :----------------------------------------- | :--------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `claude plugin add <source>`               | `error: unknown command 'add'`                                               | `claude plugin marketplace add <source>`, um einen Marktplatz hinzuzufügen, oder `claude plugin install <plugin>@<marketplace>`, um ein Plugin zu installieren |
| `claude plugin install <plugin> --project` | `error: unknown option '--project'`                                          | `claude plugin install <plugin>@<marketplace> --scope project`                                                                                                 |
| `/install <plugin>`                        | `Unknown command: /install`                                                  | `/plugin install <plugin>@<marketplace>`                                                                                                                       |
| `/plugin add <source>`                     | Das `/plugin`-Panel öffnet sich auf der Registerkarte **Discover**           | `/plugin marketplace add <source>`                                                                                                                             |
| `marketplace.anthropic.com` als Quelle     | `Invalid marketplace source format. Try: owner/repo, https://..., or ./path` | `anthropics/claude-plugins-official` für den offiziellen Marktplatz                                                                                            |

Diese Schreibweisen sehen falsch aus, funktionieren aber:

* `claude plugins` ist ein Alias von `claude plugin`
* `claude plugin remove` ist ein Alias von `claude plugin uninstall`
* `/plugins` und `/marketplace` in einer Sitzung öffnen dasselbe Panel wie `/plugin`

<h2 id="add-a-marketplace">
  Einen Marktplatz hinzufügen
</h2>

Ein Marktplatz ist ein Katalog, den Sie Claude Code aus einem Git-Repository, einer URL oder einem lokalen Pfad hinzufügen. Diese Einträge behandeln die Meldungen, die Sie erhalten, wenn das Hinzufügen fehlschlägt oder eine spätere Aktualisierung fehlschlägt.

<h3 id="marketplace-claude-plugins-official-not-found">
  `Marketplace "claude-plugins-official" not found`
</h3>

Sie haben `/plugin install <plugin>@claude-plugins-official` in einer Sitzung ausgeführt, und Claude Code hat gemeldet, dass es keinen Marktplatz mit diesem Namen hat.

Der offizielle Marktplatz ist auf diesem Computer noch nicht registriert. Claude Code registriert ihn normalerweise selbst beim ersten Mal, wenn Sie eine interaktive Terminalsitzung starten. Er ist noch nicht ausgeführt worden, wenn Sie Claude Code nur über die VS Code-Erweiterung verwendet haben, und er überspringt oder verschiebt diesen Schritt:

* Wenn eine Richtlinie die Quelle blockiert
* Wenn `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL` gesetzt ist
* Nach einem fehlgeschlagenen Versuch, der auf eine Wiederholung wartet

Die `claude plugin`-Shell-Befehle registrieren ihn nie für Sie.

Fügen Sie ihn hinzu, und versuchen Sie dann die Installation erneut:

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code druckt `Successfully added marketplace: claude-plugins-official`, und `/plugin marketplace list` zeigt den Marktplatz mit seiner Quelle.

Für jeden anderen Marktplatznamen in dieser Meldung siehe [`Marketplace "<name>" not found`](#marketplace-not-found).

Dieselbe Zeichenkette erscheint auch in der Registerkarte `/plugin` **Errors**, der Liste der Ladefehler des Panels, wenn ein Plugin in Ihren Einstellungen einen Marktplatz benennt, den Sie nicht hinzugefügt haben.

<h3 id="marketplace-not-found">
  `Marketplace "<name>" not found`
</h3>

Sie haben `/plugin install <plugin>@<name>` in einer Sitzung ausgeführt, oft aus einer Installationszeile, die jemand Ihnen gesendet hat, und Claude Code hat gemeldet, dass es keinen Marktplatz mit diesem Namen hat.

Wenn der Name mit `claudeai-` beginnt, wird der Marktplatz auf claude.ai gehostet, und Sie fügen ihn nach Name aus Ihrer Shell mit `claude plugin marketplace add --claudeai <name>` hinzu. Siehe [Einen Marktplatz von claude.ai hinzufügen](/docs/de/plugins/install#add-from-claude-ai).

Für jeden anderen Namen benennt eine Installationszeile einen Marktplatz, sagt aber nicht, wo der Marktplatz gehostet wird, und Claude Code hat keinen Index, um einen Marktplatznamen darin nachzuschlagen. Fragen Sie denjenigen, der die Zeile gesendet hat, nach der Quelle des Marktplatzes, die ein GitHub `owner/repo`, eine Git-URL oder ein Pfad ist. Dann [fügen Sie den Marktplatz hinzu](/docs/de/plugins/install#add-a-marketplace) und führen Sie die Installationszeile erneut aus.

Ein Marktplatz, den jemand Ihnen sendet, ist von Drittanbietern, daher [überprüfen Sie das Plugin, bevor Sie es installieren](/docs/de/plugins/security#review-a-plugin-before-you-install).

Wenn Sie den Marktplatz bereits hinzugefügt haben, überprüfen Sie die Schreibweise gegen `/plugin marketplace list`.

<h3 id="invalid-marketplace-source-format">
  `Invalid marketplace source format`
</h3>

Sie haben `/plugin marketplace add <source>` oder `claude plugin marketplace add <source>` ausgeführt, und Claude Code hat `Invalid marketplace source format. Try: owner/repo, https://..., or ./path` geantwortet.

Claude Code akzeptiert eine Quelle in einer dieser Formen:

* Ein GitHub `owner/repo`-Kurzbefehl
* Eine `https://`- oder `http://`-URL
* Eine `user@host:path`-SSH-URL
* Ein lokaler Pfad, der mit `./`, `../`, `/` oder `~` beginnt

Ein bloßer Name wie `claude-plugins-official` passt zu keinem davon. Auch nicht ein bloßer Hostname wie `marketplace.anthropic.com`.

Geben Sie die Quelle in einer der akzeptierten Formen erneut ein:

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code druckt `Successfully added marketplace: <name>`, wenn das Hinzufügen funktioniert.

<h3 id="is-not-a-valid-github-owner-repo-shorthand">
  `'<source>' is not a valid GitHub owner/repo shorthand`
</h3>

Sie haben eine Quelle mit einem Schrägstrich übergeben, die nicht `owner/repo` ist, wie `github.com/owner/repo` oder ein `gitlab.example.com/group/project`-Pfad. Claude Code hat sie mit dieser Meldung und einer Liste akzeptierter Formen abgelehnt.

Der `owner/repo`-Kurzbefehl ist nur für GitHub und muss GitHubs Benennungsregeln befolgen, daher schlägt ein Hostname oder ein zusätzliches Pfadsegment fehl. Übergeben Sie die Quelle in der Form, die dem Ort entspricht, an dem der Marktplatz gehostet wird:

* **Ein Repository auf einem beliebigen Host**: die vollständige Clone-URL
* **Eine gehostete `marketplace.json`**: ihre `https://`-URL
* **Ein lokaler Checkout**: `./path` oder ein absoluter Pfad

Um beispielsweise den offiziellen Marktplatz durch seine Clone-URL hinzuzufügen, in einer Sitzung:

```text theme={null}
/plugin marketplace add https://github.com/anthropics/claude-plugins-official.git
```

Ein erfolgreiches Hinzufügen druckt `Successfully added marketplace: <name>`.

<h3 id="path-does-not-exist">
  `Path does not exist: <path>`
</h3>

Sie haben einen lokalen Pfad an `marketplace add` übergeben, und es existiert nichts an diesem Pfad. Ein relativer Pfad wird gegen Ihr aktuelles Verzeichnis aufgelöst.

Überprüfen Sie den aufgelösten Pfad in der Meldung. Führen Sie dann den Befehl aus dem Verzeichnis aus, von dem der relative Pfad beginnt, oder übergeben Sie einen absoluten Pfad zum Marktplatzverzeichnis. Ein erfolgreiches Hinzufügen druckt `Successfully added marketplace: <name>`.

Claude Code akzeptiert ein Verzeichnis, das `.claude-plugin/marketplace.json` enthält, oder einen Pfad zu einer `.json`-Datei. Ein Pfad zu einer anderen Datei schlägt mit `File path must point to a .json file (marketplace.json)` fehl.

<h3 id="marketplace-file-not-found-at-claude-plugin-marketplace-json">
  `Marketplace file not found at <path>/.claude-plugin/marketplace.json`
</h3>

Claude Code hat den Marktplatz geklont oder heruntergeladen, aber keine `marketplace.json` am erwarteten Pfad darin gefunden. Der Befehl zum Hinzufügen meldet ihn als `Failed to add marketplace: Marketplace file not found at ...`.

Der Standardort ist `.claude-plugin/marketplace.json` im Repository-Root, und die [Marktplatz-Referenz](/docs/de/plugins/marketplace-reference) listet die akzeptierten Orte auf.

Die Lösung unterscheidet sich für den Besitzer und für alle anderen:

* **Sie besitzen den Marktplatz**: legen Sie die Datei an diesem Ort ab und fügen Sie den Marktplatz erneut hinzu
* **Jemand anderes hostet ihn**: fragen Sie den Besitzer nach der genauen Quelle, die er veröffentlicht

<h3 id="ssh-authentication-failed-or-https-authentication-failed">
  `SSH authentication failed` oder `HTTPS authentication failed`
</h3>

Sie haben einen Marktplatz aus einem Git-Repository hinzugefügt oder aktualisiert, und der Clone ist mit `Failed to clone marketplace repository:` gefolgt von einer dieser Zeilen fehlgeschlagen.

Überprüfen Sie zunächst das Repository selbst: ein falsch geschriebenes `owner/repo`, ein Repository, das nicht vorhanden ist, oder ein privates Repository, das Sie nicht sehen können, endet auch in dieser Meldung. Öffnen Sie die Repository-URL in Ihrem Browser, oder führen Sie `git ls-remote <url>` in Ihrem Terminal aus, um zu bestätigen, dass es vorhanden ist und Sie Zugriff haben.

Wenn das Repository richtig ist, liegt die Ursache bei den Anmeldedaten. Claude Code führt Git mit deaktivierten interaktiven Eingabeaufforderungen aus, daher kann es Sie nicht um ein Passwort, eine Schlüsselpassphrase oder eine Anmeldedaten fragen, wie Ihr Terminal es würde. Wenn Git eine Eingabeaufforderung benötigt, sehen Sie `fatal: Cannot prompt because user interactivity has been disabled` oder `terminal prompts disabled` im ursprünglichen Fehler. Nur Anmeldedaten, die bereits nicht-interaktiv funktionieren, sind erfolgreich:

* **SSH**: `ssh -T git@<host>` muss ohne Aufforderung für eine Passphrase erfolgreich sein, und der Host muss bereits in `known_hosts` sein
* **HTTPS**: Ihr Credential Helper muss ein Token für den Host enthalten. Für GitHub führen Sie `gh auth login` und `gh auth setup-git` aus. Für einen anderen Host speichern Sie ein persönliches Zugriffstoken in Ihrem Git-Credential Helper. Testen Sie mit `git ls-remote <url>`

Sobald `git ls-remote` in Ihrem Terminal ohne Eingabeaufforderung erfolgreich ist, führen Sie das Hinzufügen oder die Aktualisierung erneut aus. Ein erfolgreiches Hinzufügen druckt `Successfully added marketplace: <name>`. Eine erfolgreiche Aktualisierung druckt `Successfully updated marketplace: <name>` aus Ihrer Shell oder `✔ Updated 1 marketplace` in einer Sitzung.

Um Claude Code zu veranlassen, SSH für GitHub `owner/repo`-Quellen zu überspringen, setzen Sie `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`. Ohne sie klont Claude Code diese Quellen über SSH, wenn ein SSH-Schlüssel für `github.com` konfiguriert aussieht, und fällt auf HTTPS zurück, wenn der SSH-Clone fehlschlägt.

Für das, was Hintergrund-Auto-Updates mit Ihren Anmeldedaten können und nicht können, siehe [Was Hintergrund-Auto-Update mit Anmeldedaten macht](/docs/de/plugins/host-marketplace#what-background-auto-update-does-with-credentials).

<h3 id="ssh-host-key-is-not-in-your-known-hosts-file">
  `SSH host key is not in your known_hosts file`
</h3>

Sie haben einen Marktplatz über SSH von einem Host hinzugefügt, zu dem Sie sich noch nie verbunden haben, und der Clone ist mit dieser Zeile und einem `ssh -T git@<host>`-Hinweis fehlgeschlagen. Für einen Host, dessen Schlüssel sich geändert hat, lautet die Meldung `SSH host key has changed` mit einem `ssh-keygen -R <host>`-Hinweis statt dessen.

Claude Code klont mit `StrictHostKeyChecking=yes`, daher weigert es sich, einen Host zu akzeptieren, dessen Schlüssel Sie noch nicht akzeptiert haben, anstatt den Schlüssel automatisch zu akzeptieren. Verbinden Sie sich einmal von Ihrem Terminal aus, um den Fingerabdruck zu akzeptieren, und versuchen Sie es dann erneut:

```shell theme={null}
ssh -T git@github.com
```

Für ein öffentliches Repository fügen Sie den Marktplatz stattdessen durch seine `https://`-URL hinzu, um SSH ganz zu vermeiden.

<h3 id="command-git-not-found-or-is-in-an-unsafe-location">
  `Command 'git' not found or is in an unsafe location`
</h3>

Unter Windows haben Sie einen Marktplatz hinzugefügt und Claude Code hat `Failed to clone marketplace repository: Command 'git' not found or is in an unsafe location (current directory)` gemeldet.

Claude Code sucht nach `git` auf Ihrem `PATH` und weigert sich, einen zu führen, der nur im aktuellen Verzeichnis gefunden wird. Um es zu beheben, installieren Sie Git und versuchen Sie es erneut:

<Steps>
  <Step title="Installieren Sie Git für Windows">
    Installieren Sie Git für Windows, damit `git` auf Ihrem `PATH` ist.
  </Step>

  <Step title="Öffnen Sie ein neues Terminal">
    Öffnen Sie ein neues Terminal, damit der aktualisierte `PATH` angewendet wird.
  </Step>

  <Step title="Bestätigen Sie, dass Git ausgeführt wird">
    Bestätigen Sie, dass `git --version` eine Version druckt.
  </Step>

  <Step title="Versuchen Sie das Hinzufügen erneut">
    Führen Sie den Befehl `marketplace add` erneut aus.
  </Step>
</Steps>

<h3 id="git-clone-timed-out-after-120s">
  `Git clone timed out after 120s`
</h3>

Sie haben einen Marktplatz hinzugefügt oder aktualisiert, und es ist mit `Git clone timed out after 120s` fehlgeschlagen, gefolgt von einem Hinweis zum Setzen von `CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`.

Das Klonen eines Marktplatzes und das erneute Klonen zum Aktualisieren erhält standardmäßig 120 Sekunden. Für ein großes Repository oder eine langsame Verbindung erhöhen Sie das Limit. Der Wert ist in Millisekunden:

<Tabs>
  <Tab title="Bash oder Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS=300000
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS = "300000"
    ```
  </Tab>
</Tabs>

Versuchen Sie es dann in derselben Shell erneut.

Wenn das Repository ein Monorepo ist, begrenzen Sie den Checkout auf die Verzeichnisse, die Sie mit `claude plugin marketplace add <source> --sparse <paths>` benennen.

<h3 id="marketplace-updates-keep-failing-offline">
  Marktplatz-Updates schlagen offline immer fehl
</h3>

Sie arbeiten in einer Umgebung, in der der Git-Host des Marktplatzes nicht erreichbar ist, und jede Sitzung wiederholt eine fehlgeschlagene Aktualisierung im Hintergrund. Ihr vorhandener Checkout des Marktplatzes bleibt an Ort und Stelle und der Start wird nicht verzögert.

Jede Sitzung überprüft Claude Code für einen Marktplatz mit [Auto-Update an](/docs/de/plugins/loading#which-marketplaces-and-plugins-auto-update) den Git-Host des Marktplatzes im Hintergrund auf neue Commits. Wenn diese Überprüfung den Host nicht erreichen kann, versucht sie, den Marktplatz erneut zu klonen, und offline schlägt dieser Clone auch fehl.

Setzen Sie diese Variable, um den Re-Clone-Versuch zu überspringen und den vorhandenen Checkout zu verwenden, wenn die Überprüfung den Host nicht erreichen kann:

<Tabs>
  <Tab title="Bash oder Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE = "1"
    ```
  </Tab>
</Tabs>

Mit der gesetzten Variable überspringt Claude Code den Re-Clone nur für einen Checkout, der bereits `.claude-plugin/marketplace.json` enthält. Ein Marktplatz, der nie geklont wurde oder dessen Clone auf halbem Weg stoppte, erhält immer noch den Clone-Versuch, daher fügen Sie ihn einmal online hinzu.

Für eine vollständig offline-Bereitstellung füllen Sie stattdessen das Plugins-Verzeichnis zur Image-Build-Zeit mit `CLAUDE_CODE_PLUGIN_SEED_DIR` vor, nach [Seed-Container und CI](/docs/de/plugins/org#seed-containers-and-ci).

<h3 id="marketplace-add-fails-on-a-github-enterprise-server-host">
  Marktplatz-Hinzufügen schlägt auf einem GitHub Enterprise Server-Host fehl
</h3>

Sie haben einen Marktplatz von einer GitHub Enterprise Server (GHES)-URL hinzugefügt und erhielten einen Richtlinienfehler, oder Sie haben ihn von claude.ai hinzugefügt und erhielten einen GitHub-Zugriffsfehler.

Beide Fälle befinden sich auf der GHES-Seite:

* [Ein Richtlinienfehler](/docs/de/github-enterprise-server#marketplace-add-fails-with-a-policy-error) bedeutet, dass Ihre Organisation Marktplatzquellen eingeschränkt hat und ein Administrator ein `hostPattern` für den Host hinzufügen muss
* [Ein GitHub-Zugriffsfehler auf claude.ai](/docs/de/github-enterprise-server#marketplace-add-on-claude-ai-fails-with-a-github-access-error) bedeutet, dass Ihr eigenes GitHub Enterprise-Konto noch nicht verbunden ist

<h2 id="install-a-plugin">
  Ein Plugin installieren
</h2>

Sie haben einen Marktplatz hinzugefügt und eine Installation ausgeführt, und die Installation hat mit einer Meldung statt mit der Installation von etwas gestoppt. Diese Einträge behandeln diese Meldungen. Sie behandeln auch die zugehörigen Meldungen, die später in der Registerkarte `/plugin` **Errors** oder als leere Registerkarte **Discover** erscheinen, wenn ein Plugin oder sein Marktplatz nicht gefunden, gelesen oder vertraut werden kann.

<h3 id="plugin-not-found-in-marketplace">
  `Plugin "<name>" not found in marketplace "<marketplace>"`
</h3>

Sie haben `/plugin install <name>@<marketplace>` oder `claude plugin install <name>@<marketplace>` ausgeführt, und der Plugin-Name befindet sich nicht in der Kopie des Katalogs dieses Marktplatzes auf Ihrem Computer.

`claude plugin install` in Ihrer Shell druckt dieselbe Meldung, wenn Sie den Marktplatz überhaupt nicht hinzugefügt haben. Wenn `claude plugin marketplace update <marketplace>` dann `Marketplace '<marketplace>' not found` antwortet, [fügen Sie den Marktplatz zuerst hinzu](#add-a-marketplace).

<h4 id="the-message-ends-with-a-refresh-hint">
  `not found in marketplace` mit einem Aktualisierungshinweis
</h4>

Der Hinweis lautet `Your local copy may be out of date — try claude plugin marketplace update <marketplace>` oder `The marketplace couldn't be refreshed (...)`. Claude Code hat den Marktplatz vor der Suche nicht aktualisiert, z. B. wenn Sie offline sind, daher kann Ihre Kopie des Katalogs veraltet sein. Aktualisieren Sie mit dem Namen des Marktplatzes und installieren Sie dann erneut:

```text theme={null}
/plugin marketplace update <marketplace>
```

`claude plugin marketplace update` druckt `Successfully updated marketplace: <name>`, und `/plugin marketplace update` zeigt `✔ Updated 1 marketplace`. Wenn die wiederholte Installation dieselbe Meldung druckt, überprüfen Sie den Namen wie [`not found in marketplace` ohne Hinweis](#the-message-has-no-hint) beschreibt. [Wenn Claude Code einen Marktplatz vor einer Installation aktualisiert](/docs/de/plugins/loading#when-claude-code-refreshes-a-marketplace-before-an-install) listet die anderen Fälle auf, in denen die Aktualisierung nicht ausgeführt wird.

<h4 id="the-message-has-no-hint">
  `not found in marketplace` ohne Hinweis
</h4>

Der Name ist das wahrscheinlichste Problem. Öffnen Sie `/plugin`, gehen Sie zu **Discover**, und kopieren Sie den Namen aus der Liste.

Vor v2.1.232 aktualisierte Claude Code den benannten Marktplatz nur nach dem Lookup-Fehler und nur, wenn Auto-Update dafür aktiviert war.

<h3 id="plugin-not-found-in-any-marketplace">
  `Plugin "<name>" not found in any marketplace`
</h3>

Sie haben `/plugin install <name>` ohne `@marketplace` ausgeführt, und kein registrierter Marktplatz hat dieses Plugin. `claude plugin install <name>` meldet `Plugin "<name>" not found in any configured marketplace`.

Ohne einen Marktplatznamen durchsucht `claude plugin install` die Kataloge, die es bereits hat, und aktualisiert sie nicht zuerst, und `/plugin install` aktualisiert nur Marktplätze, die Auto-Update aktiviert haben. Benennen Sie den Marktplatz, und Claude Code aktualisiert ihn vor der Suche nach dem Plugin:

```text theme={null}
/plugin install <name>@<marketplace>
```

Wenn die Installation funktioniert, sehen Sie `✓ Installed <plugin>.` in einer Sitzung oder `Successfully installed plugin: <plugin>@<marketplace>` von `claude plugin install`.

Wenn Sie nicht wissen, welcher Marktplatz das Plugin auflistet, führen Sie `/plugin marketplace list` für die Marktplätze aus, die Sie haben, und durchsuchen Sie **Discover** in `/plugin` nach dem Plugin-Namen.

<h3 id="plugin-is-already-installed-globally">
  `Plugin '<name>@<marketplace>' is already installed globally`
</h3>

Sie haben `/plugin install` für ein Plugin ausgeführt, das bereits im Benutzerbereich oder durch verwaltete Einstellungen installiert ist, und Claude Code hat mit `Use '/plugin' to manage existing plugins.` abgelehnt. Wenn Sie den Plugin-Namen ohne `@<marketplace>` eingegeben haben, lässt die Meldung `globally` weg.

Das Plugin ist bereits in jedem Projekt verfügbar, daher gibt es nichts hinzuzufügen. Um seinen [Bereich](/docs/de/plugins/install) zu ändern, ihn zu aktivieren oder zu deaktivieren oder ihn zu konfigurieren, öffnen Sie `/plugin` und gehen Sie zu **Installed**.

Ein Plugin, das nur im Projekt- oder lokalen Bereich installiert ist, löst diese Meldung nicht aus. Claude Code lässt Sie es auch im Benutzerbereich installieren, daher ist es in anderen Projekten verfügbar.

`claude plugin install` in Ihrer Shell druckt eine andere Meldung. Für ein Plugin, das bereits im Zielbereich installiert ist, druckt es `Plugin "<name>@<marketplace>" is already installed (scope: user)` und beendet mit 0. Wenn sein Cache-Verzeichnis fehlt, lädt derselbe Befehl es erneut herunter.

<h3 id="this-plugin-uses-a-source-type-your-claude-code-version-does-not-suppo">
  `This plugin uses a source type your Claude Code version does not support`
</h3>

Sie haben ein Plugin installiert, dessen Marktplatz-Eintrag einen Quellentyp verwendet, den diese Version von Claude Code nicht abrufen kann, und Claude Code hat mit dieser Meldung und `Update Claude Code and try again.` gestoppt.

Aktualisieren Sie Claude Code, und versuchen Sie dann die Installation erneut. Quellentypen befinden sich in der [Marktplatz-Referenz](/docs/de/plugins/marketplace-reference).

<h3 id="plugin-archive-integrity-check-failed">
  `Plugin archive integrity check failed`
</h3>

Sie haben ein Plugin installiert, das als ZIP-Archiv verteilt wird, und Claude Code hat es mit dieser Zeile und `The archive was not installed.` abgelehnt. Der Marktplatz-Eintrag des Plugins verwendet eine [`archive`-Quelle](/docs/de/plugins/marketplace-reference) mit einem `sha256`-Pin, und der Digest der heruntergeladenen Datei stimmt nicht mit dem Pin überein.

Die vollständige Meldung sieht so aus:

```text theme={null}
Plugin archive integrity check failed for https://artifacts.example.com/claude-plugins/my-plugin.zip: expected sha256 6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1, got ac52220c0914ef8ca6a602e4a7362f88d30fb021110f72a6d15b68c3fe7df2b7. The archive was not installed. Verify the sha256 in the marketplace entry, or that the URL serves the intended file.
```

Die Lösung unterscheidet sich für den Herausgeber und den Installer:

* **Sie veröffentlichen das Plugin**: berechnen Sie den Digest der genauen Datei, die die URL bereitstellt, neu und aktualisieren Sie den `sha256` im Marktplatz-Eintrag. Verwenden Sie `shasum -a 256 my-plugin.zip` oder `Get-FileHash -Algorithm SHA256 my-plugin.zip` in PowerShell
* **Sie installieren das Plugin**: führen Sie `/plugin marketplace update <name>` in einer Sitzung aus, um den Katalog zu aktualisieren, falls der Eintrag korrigiert wurde, und versuchen Sie dann die Installation erneut. Wenn die Digests nach der Aktualisierung immer noch nicht übereinstimmen, fragen Sie den Marktplatz-Besitzer, welche Datei er vor der Installation gepinnt hat

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  `Marketplace "<name>" is registered from an untrusted source`
</h3>

Ein Marktplatz, den Sie früher hinzugefügt haben, hat das Laden gestoppt, und so auch seine Plugins. Diese Zeile erscheint in der Registerkarte `/plugin` **Errors** oder bei der nächsten Aktualisierung.

Der Marktplatz ist unter einem Namen registriert, der [für offizielle Anthropic-Marktplätze reserviert ist](/docs/de/plugins/marketplace-reference), aber seine registrierte Quelle ist kein `anthropics`-GitHub-Repository. Reservierte Namen werden jedes Mal neu überprüft, wenn ein Marktplatz geladen oder aktualisiert wird, daher laden der Marktplatz und die von ihm installierten Plugins nicht mehr.

Die vollständige Meldung benennt den reservierten Namen und die Lösung:

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

Die Lösung unterscheidet sich für Benutzer und Herausgeber:

* **Sie verwenden den Marktplatz**: führen Sie in Ihrer Shell `claude plugin marketplace remove <name>` aus, fügen Sie dann den Marktplatz erneut aus dem offiziellen `github.com/anthropics`-Repository hinzu
* **Sie veröffentlichen einen Drittanbieter-Marktplatz, der den Namen vor seiner Reservierung verwendet hat**: benennen Sie ihn um und bitten Sie Benutzer, ihn von Ihrer Quelle erneut hinzuzufügen

Vor v2.1.205 überprüfte Claude Code den Namen nur, wenn Sie den Marktplatz hinzufügten, daher behielt ein Eintrag, der vor seiner Reservierung registriert wurde, das Laden bei.

<h3 id="plugin-has-a-corrupt-manifest-file-or-has-an-invalid-manifest-file">
  `Plugin <name> has a corrupt manifest file` oder `has an invalid manifest file`
</h3>

Claude Code hat das Plugin abgerufen, konnte dann aber seine `.claude-plugin/plugin.json` nicht lesen. In der Shell kann der `<name>` in dieser Zeile ein temporärer Verzeichnisname sein; das Präfix `Failed to install plugin "<name>@<marketplace>"` trägt den echten Namen des Plugins. Die Formulierung sagt, welche Überprüfung fehlgeschlagen ist:

* **`corrupt manifest file`, gefolgt von `JSON parse error:`**: die Datei ist kein gültiges JSON
* **`invalid manifest file`, gefolgt von `Validation errors:`**: die Datei wird geparst, schlägt aber das Schema fehl, wie `name: Invalid input` für ein fehlendes erforderliches Feld

`claude plugin install` meldet entweder als `Failed to install plugin "<name>@<marketplace>":` und beendet mit Code 1.

Der Autor des Plugins muss die Datei beheben, und das Plugin kann nicht installiert werden, bis dies geschieht:

* **Wenn das Sie sind**: führen Sie `claude plugin validate <plugin-directory>` in Ihrer Shell aus, um denselben Fehler mit dem fehlerhaften Pfad zu sehen, und beheben Sie dann die Datei
* **Wenn das nicht Sie sind**: melden Sie die Meldung dem Marktplatz-Besitzer

<h3 id="plugin-directory-not-found-at-path">
  `Plugin directory not found at path: <path>`
</h3>

Die Registerkarte **Errors** in `/plugin` zeigt dies für ein aktiviertes Plugin, das sein Marktplatz durch einen relativen Pfad auflistet, wie `./plugins/my-plugin`, wenn kein Verzeichnis an diesem Pfad im Marktplatz vorhanden ist. Wenn Sie den Marktplatz verwalten, korrigieren Sie den `source`-Pfad des Eintrags oder stellen Sie den Ordner wieder her. Andernfalls melden Sie die Meldung dem Marktplatz-Besitzer.

`Marketplace directory not found at path: <path>` bedeutet stattdessen, dass das eigene Verzeichnis des Marktplatzes fehlt. Für einen Marktplatz, den Sie von einem lokalen Pfad hinzugefügt haben, wurde dieses Verzeichnis verschoben oder gelöscht. Stellen Sie es wieder her, oder entfernen Sie den Marktplatz und fügen Sie ihn von seinem neuen Ort erneut hinzu.

<h3 id="no-plugins-available-or-no-marketplaces-configured">
  `No plugins available` oder `No marketplaces configured`
</h3>

Sie haben `/plugin` geöffnet und die Registerkarte **Discover** ist leer, oder `claude plugin marketplace list` hat `No marketplaces configured` gedruckt.

Kein Marktplatz ist registriert, daher gibt es keinen Katalog zum Anzeigen. Fügen Sie in einer Sitzung den offiziellen Marktplatz `anthropics/claude-plugins-official` hinzu:

```text theme={null}
/plugin marketplace add anthropics/claude-plugins-official
```

Claude Code druckt `Successfully added marketplace: claude-plugins-official`, und **Discover** listet seine Plugins auf. Die Seite [Anthropic-Marktplätze](/docs/de/plugins/anthropic-marketplaces) listet die anderen Marktplätze auf, die Sie hinzufügen können.

<h3 id="marketplace-is-already-added-from-a-different-source">
  `Marketplace "<name>" is already added from a different source`
</h3>

Sie haben das Hinzufügen eines Marktplatzes durch [`/plugin install <plugin> --marketplace <source>`](/docs/de/plugins/install#add-a-marketplace-and-install-in-one-command) bestätigt, und der Katalog, den Claude Code von dieser Quelle abgerufen hat, hat denselben Namen wie ein Marktplatz, den Sie bereits von einer anderen Quelle hinzugefügt haben. Claude Code behält den vorhandenen Marktplatz bei, anstatt ihn zu ersetzen, und das Plugin wird nicht installiert.

Die vollständige Meldung sieht so aus:

```text theme={null}
Marketplace "acme-tools" is already added from a different source (github:acme/plugins). To use this source instead, remove that marketplace first with /plugin marketplace remove acme-tools.
```

Wählen Sie, welche Quelle Sie möchten:

* **Der Marktplatz, den Sie bereits hinzugefügt haben**: installieren Sie von ihm nach Name mit `/plugin install <plugin>@<name>`
* **Die neue Quelle**: führen Sie `/plugin marketplace remove <name>` aus, und versuchen Sie dann die Installation erneut

<h3 id="cannot-add-marketplace-its-network-source-differs">
  `Cannot add marketplace "<name>": its network source differs from the one declared for it in settings`
</h3>

Sie haben `marketplace add` ausgeführt, und der Katalog an dieser Quelle hat denselben Namen wie ein Marktplatz, den eine Einstellungsdatei bereits unter [`extraKnownMarketplaces`](/docs/de/settings-reference#extraknownmarketplaces) mit einer anderen Quelle deklariert. Claude Code weigert das Hinzufügen und registriert nichts.

Die Meldung endet mit der Lösung: Die Quelle muss mit der übereinstimmen, die die Einstellungen für diesen Namen deklarieren, oder Sie ändern die Deklaration. Vergleichen Sie die Quelle, die Sie übergeben haben, mit dem `extraKnownMarketplaces`-Eintrag für diesen Namen, einschließlich seines `ref`, `path` und `headers`, und führen Sie dann eines dieser aus:

* **Verwenden Sie die deklarierte Quelle**: fügen Sie den Marktplatz von der Quelle hinzu, die der Einstellungseintrag benennt
* **Verwenden Sie die neue Quelle**: bearbeiten Sie oder entfernen Sie den `extraKnownMarketplaces`-Eintrag, und fügen Sie den Marktplatz erneut hinzu. Wenn verwaltete Einstellungen ihn deklarieren, fragen Sie Ihren Administrator

<h3 id="failed-to-install-from-the-plugin-menu">
  `Failed to install: <plugin> (<reason>)`
</h3>

Sie haben Plugins zum Installieren im Menü `/plugin` ausgewählt, keines von ihnen wurde installiert, und das Menü hat sich mit dieser Zusammenfassung dessen geschlossen, was fehlgeschlagen ist.

Einige Gründe, wie die Ausgabe von Git nach einem fehlgeschlagenen Clone, zeigen nur ihre erste Zeile. Wenn ein solcher Grund gekürzt wurde, endet die Zusammenfassung mit `Installing a plugin from its details (Enter) in /plugin shows its full error.`

Was zu tun ist, hängt davon ab, ob die Zusammenfassung den Grund gekürzt hat:

* Beheben Sie, was der Grund in Klammern benennt
* Wenn der Grund gekürzt wurde, führen Sie `/plugin` aus, wählen Sie das Plugin auf der Registerkarte **Discover** aus, und drücken Sie **Enter**, um es von seinen Details aus zu installieren. Wenn die Installation dort fehlschlägt, zeigt die Detailansicht den ganzen Fehler

<h3 id="could-not-move-the-new-copy-of-this-plugin-version">
  `Could not move the new copy of this plugin version into <path>`
</h3>

Wenn Sie ein Plugin installieren, lädt Claude Code eine frische Kopie seiner Dateien herunter und verschiebt sie in den Ordner dieser Version im [Plugin-Cache](/docs/de/plugins/loading#find-plugins-on-disk). Diese Meldung bedeutet, dass die Verschiebung fehlgeschlagen ist, normalerweise weil ein anderes Programm den Ordner verwendete, während die Installation ausgeführt wurde. Der Dateisystem-Code erscheint in Klammern:

```text theme={null}
Could not move the new copy of this plugin version into /home/user/.claude/plugins/cache/acme-tools/formatter/1.2.0: the new copy or the version folder stayed busy while the install ran (ENOTEMPTY) — usually a scanner still reading the freshly downloaded files, another program using that folder, or another process re-creating it. The previously installed copy was moved back. Run the install again once other Claude Code sessions or programs using that folder have finished.
```

Die Meldung sagt, was mit der Kopie geschah, die vor der Installation installiert wurde, was Ihnen sagt, ob das Plugin immer noch funktioniert:

* `The previously installed copy was moved back`: die Version, die Sie hatten, ist immer noch installiert
* `had to be removed first`, `was not moved back` oder `could not be moved back`: diese Plugin-Version ist nicht installiert, bis eine Installation erfolgreich ist
* Kein solcher Satz: es gab keine frühere Kopie, daher ist die Version noch nicht installiert

Unter Windows, wenn ein anderes Programm die installierte Kopie selbst hält, sagt die Meldung stattdessen, dass diese Kopie `could not be replaced` ist und dass `It was not replaced and the new copy was discarded`, daher ist die Version, die Sie hatten, immer noch installiert.

Eine `Left on disk`-Liste benennt beiseite gelegte Ordner im Cache. Eine spätere Installation dieser Version oder eine Plugin-Cache-Bereinigung entfernt sie, daher müssen Sie sie nicht löschen.

Um die Installation zu beheben:

* Schließen Sie andere Claude Code-Sitzungen, Editoren und Terminals, die den Plugin-Ordner unter `~/.claude/plugins/cache` verwenden, und führen Sie die Installation erneut aus
* Wenn die Meldung sagt, die Berechtigungen des Plugin-Cache-Ordners zu überprüfen, stellen Sie Ihre Schreibberechtigung auf dem Ordner wieder her, den sie benennt, und geben Sie Speicherplatz frei, und führen Sie die Installation erneut aus

<h3 id="dependency-errors">
  Abhängigkeitsfehler
</h3>

Ein Plugin, das Abhängigkeiten deklariert, kann nicht installiert werden oder installiert und bleibt deaktiviert, wenn eine Abhängigkeit nicht erfüllt werden kann. Die Meldung erreicht Sie zur Installationszeit oder zur Ladezeit:

* **Während der Installation**: die Ablehnung kommt als Fehlermeldung der Installation zurück
* **Wenn das Plugin geladen wird**: das Problem erscheint in `claude plugin list` und der Registerkarte `/plugin` **Errors**, und Claude Code hält das betroffene Plugin deaktiviert, bis Sie es beheben

Die Tabelle listet jede Meldung und ihre Lösung auf. Um Abhängigkeiten als Autor zu deklarieren, siehe [Plugin-Abhängigkeiten](/docs/de/plugins/dependencies).

| Meldung                                                                                           | Bedeutung                                                                                                                            | Wie zu beheben                                                                                                                                                                                                                                                                                                                           |
| :------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Dependency "<dep>" is not installed`                                                             | Eine deklarierte Abhängigkeit ist nicht installiert.                                                                                 | Installieren Sie sie in Ihrer Shell mit `claude plugin install <dep>@<marketplace>`, oder deinstallieren Sie das Plugin. Wenn der Marktplatz der Abhängigkeit noch nicht registriert ist, fügen Sie ihn hinzu und führen Sie `/reload-plugins` in Ihrer Sitzung aus, was die fehlenden Abhängigkeiten installiert, die es auflösen kann. |
| `Dependency "<dep>" is disabled`                                                                  | Die Abhängigkeit ist installiert, aber ausgeschaltet.                                                                                | Aktivieren Sie die Abhängigkeit, oder deinstallieren Sie das Plugin, das sie benötigt.                                                                                                                                                                                                                                                   |
| `Requires "<dep>" <range>, installed <version>`                                                   | Die Version der installierten Abhängigkeit liegt außerhalb des deklarierten Bereichs des Plugins.                                    | Aktualisieren Sie die Abhängigkeit auf eine Version im Bereich, oder deinstallieren Sie das Plugin.                                                                                                                                                                                                                                      |
| `<Plugin or Dependency> "<name>" has conflicting version requirements`                            | Keine Version erfüllt jeden Bereich, der sie pinnt. Die Meldung listet die Bereiche auf.                                             | Deinstallieren oder aktualisieren Sie eines der in Konflikt stehenden Plugins, oder bitten Sie den Upstream-Autor, seine Einschränkung zu lockern.                                                                                                                                                                                       |
| `... has version requirements too complex to intersect` oder `has an invalid version requirement` | Ein Bereich ist kein gültiges Semver, oder die kombinierten Bereiche können nicht geschnitten werden.                                | Beheben Sie den ungültigen Bereich oder vereinfachen Sie lange `\|\|`-Ketten.                                                                                                                                                                                                                                                            |
| `... has no git tag satisfying <range>`                                                           | Das Repository der Abhängigkeit hat kein `<name>--v*`-Tag im Bereich.                                                                | Überprüfen Sie, dass der Upstream-Releases mit dieser Konvention taggt, oder lockern Sie den Bereich.                                                                                                                                                                                                                                    |
| `Dependency "<dep>" (required by <plugin>) is in <marketplace>, which is not in the allowlist`    | Die Abhängigkeit befindet sich in einem anderen Marktplatz, und die marktplatzübergreifende Auflösung ist standardmäßig deaktiviert. | Installieren Sie die Abhängigkeit selbst im gleichen Bereich, in Ihrer Shell mit `claude plugin install <dep>@<marketplace>` plus dem `--scope`, in dem Sie das Plugin installieren, und versuchen Sie es dann erneut.                                                                                                                   |

Um diese programmgesteuert zu sehen, führen Sie `claude plugin list --json` in Ihrer Shell aus. Plugins mit Problemen tragen ein `errors`-Feld mit den Meldungen und ein `errorDetails`-Feld mit einem `type` für jede: die ersten zwei Zeilen sind `dependency-unsatisfied` und die dritte ist `dependency-version-unsatisfied`.

<h2 id="plugin-installed-but-not-working">
  Plugin installiert, aber funktioniert nicht
</h2>

Die Installation war erfolgreich, aber die Skills, Hooks oder Server des Plugins tun nichts. Beginnen Sie mit [Plugin wird nicht angezeigt oder seine Skills werden nicht angezeigt](#plugin-doesnt-appear-or-its-skills-dont-show-up), was Ihnen sagt, wo Claude Code das meldet, was es geladen hat, und passen Sie dann die Meldung an.

<h3 id="plugin-doesnt-appear-or-its-skills-dont-show-up">
  Plugin wird nicht angezeigt oder seine Skills werden nicht angezeigt
</h3>

Sie haben ein Plugin installiert und `/` eingegeben, um seine Skills zu erwarten, oder Claude gebeten, es zu verwenden, und nichts ist passiert.

Überprüfen Sie den Status des Plugins, bevor Sie etwas ändern:

<Steps>
  <Step title="Bestätigen Sie, dass das Plugin installiert und aktiviert ist">
    Führen Sie `/plugin` aus und öffnen Sie **Installed**. Bestätigen Sie, dass das Plugin aufgelistet und aktiviert ist. `claude plugin list` in Ihrer Shell druckt dieselbe Liste mit der Version, dem Bereich und dem `Status: ✔ enabled` jedes Plugins.
  </Step>

  <Step title="Lesen Sie die Registerkarte Errors">
    Öffnen Sie die Registerkarte **Errors** im gleichen Panel. Jeder Eintrag paart eine Meldung mit einer Anleitung. Die meisten Meldungen im Rest dieses Abschnitts stammen aus dieser Registerkarte.
  </Step>

  <Step title="Laden Sie erneut, wenn Sie während dieser Sitzung installiert haben">
    Wenn das Plugin installiert und fehlerfrei ist, aber Sie es während dieser Sitzung installiert haben, führen Sie `/reload-plugins` aus. Es druckt `Reloaded:` mit Zählungen von Plugins, Skills, Agents, Hooks und Servern. Wenn etwas fehlgeschlagen ist, fügt es `N errors during load. Run /plugin for details.` hinzu.
  </Step>
</Steps>

Wenn das Plugin ohne Fehler geladen wird und seine Skills immer noch nicht angezeigt werden, unterscheidet sich der nächste Schritt für Ihr eigenes Plugin und für das eines anderen:

* **Ein Plugin, das Sie erstellen**: siehe [Plugin lädt, aber seine Skills fehlen](#plugin-loads-but-its-skills-are-missing)
* **Ein Plugin, das jemand anderes veröffentlicht hat**: öffnen Sie **Installed** in `/plugin` und öffnen Sie den Detailbereich des Plugins, der auflistet, was das Plugin enthält. Ein Plugin, das dort keine Skills auflistet, hat keine anzubieten, wenn Sie `/` eingeben

<h3 id="run-reload-plugins-to-activate">
  `Run /reload-plugins to activate.`
</h3>

Die Installationszusammenfassung in `/plugin` endete mit `Run /reload-plugins to activate.` statt `Plugin is now active.`

Claude Code hat das Plugin während der Installation nicht aktiviert, entweder weil die Aktivierung den [Prompt-Cache ungültig machen würde](/docs/de/prompt-caching#enabling-or-disabling-a-plugin) oder weil der Aktivierungsversuch fehlgeschlagen ist.

Sie müssen den Befehl nicht eingeben. Das Panel schließt sich und Claude Code führt `/reload-plugins` für Sie aus, oder reiht es in die Warteschlange, bis die Antwort, die gerade gestreamt wird, endet.

Lesen Sie, was dieses Reload druckt:

* **`Reloaded:` mit Zählungen von Plugins, Skills, Agents, Hooks und Servern**: das Plugin ist jetzt aktiv. Wenn etwas nicht geladen werden konnte, fügt die Zeile `N errors during load. Run /plugin for details.` hinzu.
* **`This reload changes MCP tools (...) — your next message will re-read the whole conversation instead of using the cache. Run /reload-plugins --force to apply.`**: das Reload würde einen Plugin-MCP-Server hinzufügen oder entfernen, oder das `LSP`-Tool, und Ihren Prompt-Cache ungültig machen. Für den LSP-Fall beginnt die Zeile mit `This reload adds the LSP tool` oder `This reload removes the LSP tool`. Führen Sie es mit `--force` aus, um das Plugin zu aktivieren, oder starten Sie eine neue Sitzung

Vor v2.1.268 blieb eine Installation, die während der Installation nicht aktiviert wurde, ausstehend, bis Sie `/reload-plugins` selbst ausgeführt haben.

Vor v2.1.246 umfasste die Skills-Zählung in dieser Zusammenfassung nur die `commands/`-Einträge eines Plugins, daher konnte ein Reload die `SKILL.md`-Skills eines Plugins laden und immer noch `0 skills` melden.

<h3 id="plugin-not-cached-at">
  `Plugin "<name>" not cached at <path>`
</h3>

Die Registerkarte **Errors** zeigt diese Zeile mit der Anleitung `Run /plugin to refresh the plugin cache`. Claude Code hat einen Installationsdatensatz für das Plugin, aber das Verzeichnis, auf das der Datensatz zeigt, fehlt, zum Beispiel nachdem Sie den Cache gelöscht haben.

Installieren Sie das Plugin erneut von Ihrer Shell. `claude plugin install <name>@<marketplace>` lädt ein Plugin erneut herunter, dessen Installationsverzeichnis fehlt, obwohl sein Datensatz vorhanden ist:

```shell theme={null}
claude plugin install <name>@<marketplace>
```

Führen Sie dann `/reload-plugins` in Ihrer Sitzung aus. Der Eintrag in der Registerkarte **Errors** verschwindet und das Plugin ist wieder unter **Installed**.

<h3 id="a-plugin-you-disabled-still-loads">
  `Disabled in ~/.claude/settings.json but still loads`
</h3>

Sie haben ein Plugin in `~/.claude/settings.json` auf `false` gesetzt, und seine Zeile in `claude plugin list` oder `/plugin` zeigt diese Meldung gefolgt von der Quelle, die es aktiviert, wie `— project settings enable it, which overrides your user setting`. Ein `true` in dieser höheren Prioritätsquelle überschreibt Ihre Benutzereinstellung.

Um sich von einem projektaktivierten Plugin auf Ihrem Computer abzumelden, setzen Sie die ID in `.claude/settings.local.json` auf `false`, die eine höhere Priorität als die Projektdatei hat. Für die anderen Quellen, die die Meldung benennen kann, siehe [Deaktiviert in Benutzereinstellungen, aber lädt immer noch](/docs/de/plugins/loading#disabled-in-user-settings-but-still-loads).

Wenn `claude plugin list` das Plugin stattdessen als `required by your org` markiert, ist keine Einstellungsdatei beteiligt: Ihre Organisation markiert dieses synchronisierte Plugin auf claude.ai als erforderlich, und es lädt, auch wenn Sie es früher deaktiviert haben. Siehe [Plugins, die von claude.ai synchronisiert werden](/docs/de/plugins/loading#synced-plugins).

<h3 id="plugin-is-enabled-in-project-settings-but-isnt-installed-here">
  `Plugin "<name>" is enabled in project settings but isn't installed here`
</h3>

Die Registerkarte **Errors** zeigt diese Zeile für ein Plugin, das die `.claude/settings.json` Ihres Projekts aktiviert, mit der Anleitung `Run claude plugin install <name>@<marketplace> --scope project to install it for this project`.

Die Einstellungen eines Repositories können ein Plugin für jeden aktivieren, der es öffnet, aber sie installieren es nicht. Wenn das Plugin aus einer externen Quelle wie einem GitHub-Repository oder einem npm-Paket stammt, lädt Claude Code es nicht herunter, bis Sie es selbst installieren. Führen Sie den Befehl aus der Anleitung in Ihrer Shell aus, und laden Sie dann erneut:

```shell theme={null}
claude plugin install <name>@<marketplace> --scope project
```

Nachdem Sie `/reload-plugins` in Ihrer Sitzung ausgeführt haben, ist der Eintrag in der Registerkarte **Errors** weg und das Plugin ist unter **Installed** aufgelistet.

Wenn Ihre Organisation Plugins für Sie vorinstalliert, tut sie dies stattdessen durch verwaltete Einstellungen. Siehe [Plugins vorinstallieren und erforderlich machen](/docs/de/plugins/org#pre-install-and-require-plugins).

<h3 id="failed-to-load-hooks-from-and-hooks-that-dont-fire">
  `Failed to load hooks from <path>` und Hooks, die nicht ausgelöst werden
</h3>

Die Hooks eines Plugins werden nicht ausgeführt. Entweder zeigt die Registerkarte **Errors** einen Ladefehler dafür, die Hooks laden und Sie sehen `<Event> hook error`-Hinweise im Transkript, oder ein Hook lädt ohne Fehler und wird nie ausgelöst.

<h4 id="hooks-fail-to-load">
  Hooks können nicht geladen werden
</h4>

Die Registerkarte **Errors** zeigt eine dieser Meldungen:

* **`Failed to load hooks from <path>: <reason>`**: `hooks/hooks.json` ist kein gültiges JSON oder schlägt das Hooks-Schema fehl. Der Grund benennt den Parse- oder Validierungsfehler. Beheben Sie die Datei. Um ein JSON-Syntaxproblem in `hooks/hooks.json` zu fangen, bevor Sie das Plugin veröffentlichen, führen Sie `claude plugin validate <plugin-directory>` in Ihrer Shell aus
* **`hooks path not found: <path>`**: das Feld `hooks` des Manifests benennt eine Datei, die nicht an diesem Pfad relativ zum Plugin-Root vorhanden ist. Beheben Sie den Pfad oder fügen Sie die Datei hinzu

<h4 id="hook-error-notices-in-the-transcript">
  `hook error`-Hinweise im Transkript
</h4>

Ein Hinweis der Form `... hook error: Failed with non-blocking status code: <stderr>` bedeutet, dass der Hook ausgeführt wurde und sein Befehl fehlgeschlagen ist. Beispielsweise bedeutet `Stop hook error: Failed with non-blocking status code: /bin/sh: node: command not found`, dass die Shell, die Claude Code erzeugt hat, `node` nicht finden konnte. Installieren Sie es, oder stellen Sie sicher, dass es sich auf dem `PATH` des Terminals befindet, von dem Sie `claude` starten.

Für jeden anderen Fehler führen Sie den Befehl des Hooks selbst aus dem Plugin-Verzeichnis aus, um die vollständige Ausgabe zu sehen, oder erfassen Sie den vollständigen stderr mit [Debug-Protokollierung](/docs/de/hooks#debug-hooks).

<h4 id="hook-loads-but-never-fires">
  Hook lädt, aber wird nie ausgelöst
</h4>

Wenn ein Hook ohne Fehler geladen wird, aber nie ausgelöst wird, überprüfen Sie seine Definition und beobachten Sie dann seine Ausführung:

<Steps>
  <Step title="Überprüfen Sie den Ereignisnamen">
    Ereignisnamen sind Groß-/Kleinschreibung-empfindlich, daher bestätigen Sie, dass Ihrer genau übereinstimmt, zum Beispiel `PostToolUse`.
  </Step>

  <Step title="Überprüfen Sie den Matcher">
    Bestätigen Sie, dass der `matcher` des Hooks dem Tool-Namen entspricht.
  </Step>

  <Step title="Lösen Sie das Ereignis absichtlich aus">
    Für einen `PostToolUse`-Hook bitten Sie Claude, eine Datei zu bearbeiten.
  </Step>

  <Step title="Lesen Sie das Debug-Protokoll">
    Öffnen Sie das [Debug-Protokoll](/docs/de/hooks#debug-hooks), das aufzeichnet, welche Hooks übereinstimmten. Ein Hook, der ausgeführt wurde, erscheint dort mit seinem Exit-Code.
  </Step>
</Steps>

<h3 id="invalid-mcp-server-config-for-and-mcp-servers-that-dont-start">
  `Invalid MCP server config for "<server>"` und MCP-Server, die nicht starten
</h3>

Ein Plugin bündelt einen MCP-Server, und die Registerkarte **Errors** zeigt `Invalid MCP server config for "<server>": <error>`, oder der Server ist aufgelistet, aber `/mcp` zeigt ihn nie verbunden.

<h4 id="invalid-mcp-server-config-for-server-error">
  `Invalid MCP server config for "<server>": <error>`
</h4>

Die Konfiguration des Servers besteht die Schema-Überprüfung, aber Claude Code kann sie für diese Sitzung nicht auflösen. Der Text nach dem Doppelpunkt benennt die Ursache und entscheidet die Lösung:

* **`Missing environment variables: <names>`**: setzen Sie diese Variablen in der Shell, von der Sie Claude Code starten, und starten Sie dann eine neue Sitzung
* **`URL is unset or invalid`**: eine `${user_config.*}`-Option, die die URL verwendet, ist nicht gesetzt. Führen Sie `/plugin configure <plugin>` aus, um sie zu setzen
* **`has an invalid MCP url`** oder **`headersHelper for MCP server '<server>' references ${user_config.*}`**: die Konfiguration des Plugins selbst ist schuld. Beheben Sie die `url` oder `headersHelper` in der MCP-Konfiguration Ihres Plugins, oder melden Sie es dem Autor des Plugins, wenn das Plugin nicht Ihres ist. Der `headersHelper`-Fall hat seinen eigenen Eintrag unter [Plugin-Befehl referenziert user\_config](/docs/de/errors#plugin-command-references-user-config)

<h4 id="server-is-configured-but-never-connects">
  Server ist konfiguriert, verbindet sich aber nie
</h4>

Führen Sie `/mcp` aus, um den Status des Servers zu sehen. Wenn der Server gesund ist, listet `/mcp` ihn als verbunden auf.

Um den Fehler zu lesen, den der Server beim Starten gedruckt hat, führen Sie `claude --debug` aus und öffnen Sie das Protokoll unter `~/.claude/debug/<session-id>.txt`. Das Flag `--debug` druckt nicht zum Terminal.

Ein Server-Eintrag in `.mcp.json`, der das Schema nicht besteht, erscheint nicht in der Registerkarte **Errors**. Claude Code löscht diesen Server und zeichnet `Invalid MCP server config for <server> in <path>` nur in diesem Debug-Protokoll auf. Um den Eintrag zu finden, ohne das Plugin zu laden, führen Sie `claude plugin validate` in Ihrer Shell im Plugin-Verzeichnis aus, das es als Fehler meldet.

Vor v2.1.281 überprüfte `claude plugin validate` nicht `.mcp.json`.

<h4 id="server-works-with-plugin-dir-but-fails-after-install">
  Server funktioniert mit `--plugin-dir`, schlägt aber nach der Installation fehl
</h4>

Sie sind der Autor des Plugins, und der Server startet, wenn Sie das Plugin aus seinem Quellverzeichnis mit `--plugin-dir` laden, schlägt aber fehl, sobald das Plugin installiert ist.

Claude Code kopiert ein installiertes Plugin in seinen Cache, daher bricht ein Pfad, der nur aus dem Quellverzeichnis funktioniert. Schreiben Sie Pfade im Plugin mit `${CLAUDE_PLUGIN_ROOT}`.

Für Pfade, die außerhalb des Plugin-Verzeichnisses reichen, siehe [Dateien, die das Plugin außerhalb seines Verzeichnisses referenziert, werden nicht gefunden](#files-the-plugin-references-outside-its-directory-arent-found).

<h3 id="language-server-doesnt-start">
  Sprachserver startet nicht, verwendet zu viel Speicher oder meldet falsche Diagnosen
</h3>

Sie haben ein [Code-Intelligence-Plugin](/docs/de/plugins/code-intelligence) installiert und Claude sieht keine Diagnosen, oder der Sprachserver verwendet zu viel Speicher oder meldet Fehler, die nicht real sind.

<h4 id="language-server-doesn’t-start">
  Sprachserver startet nicht
</h4>

Das Plugin verbindet sich mit einem Sprachserver-Binär, das Sie separat installieren, und Claude Code erzeugt es nach Befehlsname von Ihrem `PATH`.

Die Registerkarte `/plugin` **Errors** zeigt den Fehler mit seinem Grund, wie `Executable not found in $PATH: "<binary>"`, und `claude --debug` protokolliert ihn als `LSP server <name> failed to start: <reason>`.

Installieren Sie das Binär und bestätigen Sie, dass es sich auf dem `PATH` des Terminals befindet, von dem Sie `claude` starten, zum Beispiel mit `which typescript-language-server`. Starten Sie dann eine neue Sitzung.

<h4 id="language-server-uses-too-much-memory">
  Sprachserver verwendet zu viel Speicher
</h4>

Sprachserver wie `rust-analyzer` und `pyright` indizieren das ganze Projekt. Deaktivieren Sie das Plugin mit `/plugin disable <plugin>` in einer Sitzung und verlassen Sie sich stattdessen auf Claudes integrierte Such-Tools.

<h4 id="false-positive-diagnostics-in-a-monorepo">
  Falsch-positive Diagnosen in einem Monorepo
</h4>

Ein Sprachserver, der nicht für den Arbeitsbereich konfiguriert ist, kann ungelöste Importe für interne Pakete melden. Es gibt nichts auf der Claude Code-Seite zu beheben, und die Diagnosen hindern Claude nicht daran, Code zu bearbeiten.

<h2 id="build-a-plugin">
  Ein Plugin erstellen
</h2>

Sie entwickeln ein Plugin und laden es mit `--plugin-dir` oder installieren es von einem lokalen Marktplatz. Diese Einträge behandeln die Fehler, auf die Sie bei der Entwicklung eines Plugins stoßen. Für die Überprüfungen, die nach jeder Änderung ausgeführt werden, siehe [Testen und Debuggen](/docs/de/plugins/create#test-and-debug).

Zwei Fehler, die auch die Benutzer eines Plugins erreichen, haben ihre Einträge unter [Plugin installiert, aber funktioniert nicht](#plugin-installed-but-not-working):

* **Ein Hook, der nicht ausgelöst wird**: siehe [Hooks, die nicht ausgelöst werden](#failed-to-load-hooks-from-and-hooks-that-dont-fire)
* **Ein MCP-Server, der nicht startet**: siehe [MCP-Server, die nicht starten](#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start)

<h3 id="commands-path-not-found">
  `commands path not found: <path>`
</h3>

Die Registerkarte **Errors** zeigt `commands path not found: <absolute path>` mit der Anleitung `Check that the path in your manifest or marketplace config is correct`. Dieselbe Meldung erscheint für `skills`, `agents` und `hooks`.

Claude Code hat einen Pfad aus Ihrer `plugin.json` oder Ihrem Marktplatz-Eintrag gegen den Plugin-Root aufgelöst und dort nichts gefunden. Der Pfad in der Meldung ist der absolute Pfad, den es überprüft hat, daher vergleichen Sie ihn mit dem, was auf der Festplatte vorhanden ist. Beheben Sie den Pfad oder erstellen Sie das Verzeichnis, und führen Sie dann `/reload-plugins` aus.

Pfade im Manifest sind relativ zum Plugin-Root und beginnen mit `./`. Ein Pfad, der außerhalb des Plugin-Root aufgelöst wird, wird stattdessen als `<component> path escapes plugin directory` gemeldet und wird gelöscht.

<h3 id="plugin-dir-loads-a-plugin-with-no-components">
  `--plugin-dir` in einem Marktplatz-Root lädt die Plugins unter `plugins/` nicht
</h3>

Sie haben `claude --plugin-dir <path>` gestartet und sehen keinen Fehler, aber die Skills, Agents und Hooks des Plugins sind nicht da.

`--plugin-dir` nimmt das Plugin-Root-Verzeichnis, das, das `.claude-plugin/plugin.json` und die Komponentenverzeichnisse wie `skills/` enthält. Wenn Sie es stattdessen auf einen Marktplatz-Root zeigen, liest Claude Code nicht `marketplace.json`, daher lädt ein Plugin unter `plugins/` nicht, und Sie sehen keinen Fehler. Vor v2.1.281 lud Claude Code einen Marktplatz-Root als ein leeres Plugin, das nach diesem Verzeichnis benannt ist. Zeigen Sie das Flag auf das Plugin-Verzeichnis selbst:

```shell theme={null}
claude --plugin-dir ./my-marketplace/plugins/my-plugin
```

Öffnen Sie dann **Installed** in `/plugin`, wo der Detailbereich des Plugins seine Komponenten auflistet.

<h3 id="files-the-plugin-references-outside-its-directory-arent-found">
  Dateien, die das Plugin außerhalb seines Verzeichnisses referenziert, werden nicht gefunden
</h3>

Ein Plugin funktioniert aus seinem Quellverzeichnis mit `--plugin-dir`, schlägt aber nach der Installation fehl, mit Fehlern über einen Pfad wie `../shared-utils`.

Claude Code kopiert ein installiertes Plugin in seinen Cache und lädt es von dort, daher zeigt ein Pfad, der außerhalb des eigenen Verzeichnisses des Plugins reicht, auf nichts im Cache. Verschieben Sie die gemeinsamen Dateien in das Plugin-Verzeichnis, oder referenzieren Sie sie durch einen Symlink darin. Für den Ort des Caches und wie Pfade aufgelöst werden, siehe [Plugins auf der Festplatte finden](/docs/de/plugins/loading#find-plugins-on-disk).

<h3 id="claude-plugin-root-shows-forward-slashes-on-windows">
  `${CLAUDE_PLUGIN_ROOT}` zeigt Schrägstriche unter Windows
</h3>

Unter Windows erhält ein Plugin-Hook `${CLAUDE_PLUGIN_ROOT}` als `C:/Users/you/...` statt `C:\Users\you\...`, und ein Skript, das Backslashes erwartet, bricht.

Claude Code führt Shell-Form-Hooks durch Git Bash unter Windows aus und ersetzt den Plugin-Root absichtlich in der Forward-Slash-Win32-Form. Bash-Builtins, MSYS-Tools und native Windows-Binärdateien akzeptieren alle diese Form.

Wenn Ihr Skript Backslashes benötigt, wechseln Sie den Hook zu einer der Formen, die native Pfade behalten, beschrieben unter [Exec-Form und Shell-Form](/docs/de/hooks#exec-form-and-shell-form):

* Ein Exec-Form-Hook, der den Prozess direkt mit einem `args`-Array erzeugt
* Ein Hook mit `"shell": "powershell"`

<h3 id="plugin-loads-but-its-skills-are-missing">
  Plugin lädt, aber seine Skills fehlen
</h3>

Ihr Plugin ist unter **Installed** ohne Fehler aufgelistet, aber seine Skills werden nicht angeboten, wenn Sie `/` eingeben.

Skills laden aus `skills/` im Plugin-Root und Befehle aus `commands/` im Plugin-Root. Nur `plugin.json` gehört in `.claude-plugin/`, und ein `skills/`-Verzeichnis in `.claude-plugin/` wird nicht gescannt. Verschieben Sie die Verzeichnisse zum Plugin-Root und führen Sie `/reload-plugins` aus. Danach listet der Detailbereich des Plugins in `/plugin` die Skills auf, und das Eingeben von `/` bietet sie an.

Jeder Skill ist ein Verzeichnis, das `SKILL.md` enthält. Ein `skills`-Eintrag im Manifest, der auf eine `SKILL.md`-Datei statt auf sein Verzeichnis zeigt, wird als `path is a file; skills entries must be directories containing SKILL.md` gemeldet.

<h3 id="skill-loads-but-claude-never-invokes-the-skill">
  Skill lädt, aber Claude ruft den Skill nie auf
</h3>

Der Skill Ihres Plugins wird ausgeführt, wenn Sie seinen `/<plugin>:<skill>`-Befehl eingeben, aber Claude ruft ihn nie als Reaktion auf eine einfache Anfrage auf.

Überprüfen Sie diese Ursachen in Reihenfolge:

* **Der Skill setzt `disable-model-invocation: true`**: mit diesem Feld gesetzt, können nur Sie den Skill aufrufen. Der Template-Skill in [Erstellen Sie Ihr erstes Plugin](/docs/de/plugins/create#create-your-first-plugin) setzt ihn. Entfernen Sie die Zeile aus einem Skill, den Claude von selbst aufrufen soll. [Kontrollieren Sie, wer einen Skill aufruft](/docs/de/skills#control-who-invokes-a-skill) behandelt das Feld
* **Die Beschreibung passt nicht, wie Menschen fragen**: arbeiten Sie die Überprüfungen in [Skill wird nicht ausgelöst](/docs/de/skills#skill-not-triggering) durch
* **Die Beschreibung ist gekürzt**: wenn viele Skills installiert sind, kürzt Claude Code Beschreibungen, um in das Zeichenlimit der Auflistung zu passen, was die Schlüsselwörter entfernen kann, die Claude benötigt, um eine Anfrage zu passen. Siehe [Skill-Beschreibungen werden gekürzt](/docs/de/skills#skill-descriptions-are-cut-short)

Um zu messen, wie oft der Skill über realistische Eingabeaufforderungen ausgelöst wird, statt jeden einzeln zu überprüfen, schreiben Sie einen Eval-Fall mit einem [`tool_used: Skill`-Grader](/docs/de/plugin-evals#create-your-first-eval-suite) und führen Sie ihn mit `claude plugin eval` nach jeder Beschreibungsänderung aus.

<h3 id="is-not-a-plugin-or-skill-folder">
  `<directory> is not a plugin or skill folder` von `claude plugin eval init`
</h3>

Sie haben `claude plugin eval init` aus einem Verzeichnis ausgeführt, das kein Plugin-Root ist, wie Ihr Home-Verzeichnis oder der Root eines Repositories, das das Plugin in einem Unterverzeichnis hält. `init` schreibt die Suite unter dem Arbeitsverzeichnis, daher stoppt es, statt ein `evals/`-Verzeichnis zu erstellen, das das Plugin nie sehen würde.

Wechseln Sie zum Plugin-Root, dem Verzeichnis, das `.claude-plugin/plugin.json` oder die `SKILL.md` des Skills hält, und führen Sie den Befehl erneut aus. Um die Suite absichtlich woanders zu gerüsten, übergeben Sie `--eval-dir`. Siehe [Testen Sie Plugins mit Evals](/docs/de/plugin-evals).

<h3 id="the-userconfig-dialog-never-appears">
  Der `userConfig`-Dialog erscheint nie
</h3>

Ihr Plugin deklariert `userConfig`-Optionen, aber kein Konfigurationsdialog erscheint, wenn Sie es installieren.

Die interaktive Installation zeigt den Dialog, und der Shell-Befehl nimmt die Werte stattdessen als Flags:

* **`/plugin install` in einer Sitzung, oder die Registerkarte Discover in `/plugin`**: der Dialog ist Teil dieser interaktiven Installation
* **`claude plugin install` in Ihrer Shell**: fordert nie `userConfig`-Werte auf. Es speichert alle `--config KEY=VALUE`-Werte, die Sie übergeben, und wenn Optionen ungesetzt bleiben, druckt es `N userConfig options not yet set — run /plugin configure <plugin>@<marketplace> in Claude Code, or pass --config KEY=VALUE.` Wenn eine der ungesetzten Optionen erforderlich ist, folgt `(M required)` auf `not yet set`.

Wenn Sie von der Shell aus installiert haben, übergeben Sie die Werte mit `--config`, ein Flag pro Option:

```shell theme={null}
claude plugin install my-plugin@my-marketplace --config api_url=https://example.com
```

Wenn jede Option gesetzt ist, trägt die Installationsausgabe keine `not yet set`-Zeile. Um den Dialog danach stattdessen zu öffnen, führen Sie `/plugin configure my-plugin@my-marketplace` in einer Sitzung aus.

Wenn Sie einen `--config`-Schlüssel übergeben, den das Manifest nicht deklariert, wird das Plugin immer noch installiert, und der Befehl druckt `⚠ Installed, but --config not applied: --config key "<key>" isn't declared in this plugin's userConfig.` gefolgt von den Schlüsseln, die das Plugin deklariert.

<h3 id="claude-plugin-validate-reports-errors">
  `claude plugin validate` meldet Fehler
</h3>

Sie haben `claude plugin validate <path>` ausgeführt, oder `/plugin validate <path>` in einer Sitzung, und es hat `Found N errors` und `Validation failed` gedruckt, dann mit Code 1 beendet.

Der Validator liest das Manifest am Pfad, den Sie ihm geben: `.claude-plugin/plugin.json` für ein Plugin-Verzeichnis, oder `.claude-plugin/marketplace.json` für ein Marktplatz-Verzeichnis. Für einen Marktplatz stellt er Problemen im eigenen Manifest eines Eintrags das Eintragsindex voran, wie `plugins[1] plugin.json → json: ...`.

Die Tabelle behandelt die Meldungen, die die Validierung stoppen, und zwei Warnungen, `No frontmatter block found` und `Unknown field '<key>'`, die sie nur stoppen, wenn Sie `--strict` übergeben. Andere Warnungen, wie eine fehlende Beschreibung, sind nicht aufgelistet.

| Meldung                                                                                                  | Ursache                                                                             | Beheben                                                                                                                            |
| :------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| `File not found: <path>`                                                                                 | Der Pfad hat kein Manifest, oder existiert nicht.                                   | Führen Sie den Befehl gegen das Plugin- oder Marktplatz-Root aus, das Verzeichnis, das `.claude-plugin/` enthält.                  |
| `No manifest found in directory. Expected .claude-plugin/marketplace.json or .claude-plugin/plugin.json` | Das Verzeichnis hat kein `.claude-plugin/`-Manifest.                                | Erstellen Sie das Manifest, oder zeigen Sie auf das richtige Verzeichnis.                                                          |
| `Invalid JSON syntax: <parse error>`                                                                     | Das Manifest, oder `hooks/hooks.json`, ist kein gültiges JSON.                      | Beheben Sie das JSON. Bis Sie `hooks/hooks.json` beheben, lädt eine Sitzung das Plugin ohne die Hooks in dieser Datei.             |
| `Path not found: <path>. The runtime loader will report this as a load failure.`                         | Ein Komponentenpfad im Manifest existiert nicht.                                    | Beheben Sie den Pfad oder erstellen Sie das Verzeichnis.                                                                           |
| `Path contains ".." which could be a path traversal attempt: <path>`                                     | Ein Komponentenpfad entweicht dem Plugin-Verzeichnis.                               | Verwenden Sie Pfade im Plugin-Root.                                                                                                |
| `Path is a file; skills entries must be directories containing SKILL.md`                                 | Ein `skills`-Eintrag zeigt auf `SKILL.md` statt auf sein Verzeichnis.               | Zeigen Sie auf das übergeordnete Verzeichnis, oder `.` für eine Root-Level-`SKILL.md`.                                             |
| `No frontmatter block found` oder `YAML frontmatter failed to parse: <error>`                            | Eine Skill-, Agent- oder Befehlsdatei hat fehlende oder ungültige YAML-Frontmatter. | Fügen Sie oder beheben Sie die Frontmatter zwischen `---`-Trennzeichen. Wird beim Validieren eines Plugin-Verzeichnisses gemeldet. |
| `Unknown field '<key>'`                                                                                  | Das Manifest hat ein Feld, das das Schema nicht definiert.                          | Entfernen Sie es, oder verwenden Sie den Namen, den die Meldung vorschlägt. Claude Code ignoriert unbekannte Felder zur Ladezeit.  |

Führen Sie den Befehl erneut aus, nachdem Sie jeden Fehler behoben haben, bis er keine Fehler druckt.

`plugin.json`-Felder befinden sich in der [Manifest-Referenz](/docs/de/plugins/manifest-reference), und Marktplatz-Level-Meldungen befinden sich unter [Marktplatz-Validierungsfehler](#marketplace-validation-errors).

<h3 id="plugin-has-conflicting-manifests">
  `Plugin <name> has conflicting manifests`
</h3>

Das Plugin kann nicht geladen werden mit `Plugin <name> has conflicting manifests: both plugin.json and marketplace entry specify components.`

Das Plugin hat seine eigene `plugin.json`, und sein Marktplatz-Eintrag setzt `strict: false`, während er auch eines von `commands`, `agents`, `skills`, `hooks`, `outputStyles` oder `themes` deklariert. Entfernen Sie diese Felder aus dem Eintrag, oder setzen Sie `strict: true` im Eintrag, damit Claude Code sie an `plugin.json` anhängt. Siehe [Strict-Modus](/docs/de/plugins/marketplace-reference#strict-mode).

<h3 id="warning-no-commands-found-in-plugin-custom-directory">
  `Warning: No commands found in plugin <name> custom directory`
</h3>

Wenn das Plugin geladen wird, zeichnet das `claude --debug`-Protokoll unter `~/.claude/debug/<session-id>.txt` `Warning: No commands found in plugin <name> custom directory: <path>. Expected .md files or SKILL.md in subdirectories.` auf. Nichts erscheint in der Sitzung oder der Registerkarte **Errors**.

Der `commands`-Pfad im Manifest existiert, hält aber keine `.md`-Dateien und keine `SKILL.md` in einem Unterverzeichnis. Fügen Sie die Befehlsdateien hinzu, oder entfernen Sie den Pfad aus dem Manifest.

<h2 id="host-a-marketplace">
  Einen Marktplatz hosten
</h2>

Sie veröffentlichen einen Marktplatz und ein Benutzer meldet einen Fehler, oder Ihre eigene Validierung schlägt fehl. Diese Einträge sind für den Marktplatz-Besitzer.

<h3 id="plugins-with-relative-paths-fail-in-url-based-marketplaces">
  Plugins mit relativen Pfaden schlagen in URL-basierten Marktplätzen fehl
</h3>

Benutzer haben Ihren Marktplatz mit einer `https://example.com/marketplace.json`-URL hinzugefügt. Installationen von Plugins, deren `source` ein relativer Pfad ist, wie `./plugins/my-plugin`, schlagen mit `its marketplace entry path does not stay inside the marketplace directory` fehl. Bereits installierte Plugins können nicht geladen werden mit `Plugin source path refused`. Beide Meldungen haben einen [Fehler-Referenzeintrag](/docs/de/errors#marketplace-entry-path-does-not-stay-inside-the-marketplace-directory).

Wenn ein Benutzer einen URL-basierten Marktplatz hinzufügt, lädt Claude Code nur die `marketplace.json`-Datei selbst herunter. Es lädt Plugin-Dateien nicht durch relativen Pfad von diesem Server herunter, daher zeigt ein relativer Pfad in einem Eintrag auf ein Verzeichnis, das nie heruntergeladen wurde. Geben Sie jedem Eintrag eine Quelle, die Claude Code selbst abrufen kann, wie ein GitHub-Repository:

```json theme={null}
{ "name": "my-plugin", "source": { "source": "github", "repo": "owner/repo" } }
```

Alternativ hosten Sie den Marktplatz in einem Git-Repository und sagen Sie Benutzern, dass sie ihn mit der Repository-URL hinzufügen. Für eine Git-Quelle klont Claude Code das ganze Repository, daher werden relative Pfade aufgelöst. Quellentypen befinden sich in der [Marktplatz-Referenz](/docs/de/plugins/marketplace-reference).

<h3 id="marketplace-validation-errors">
  Marktplatz-Validierungsfehler
</h3>

Sie haben `claude plugin validate .` aus Ihrem Marktplatz-Verzeichnis ausgeführt und es hat Fehler oder Warnungen in der Marktplatz-Datei selbst gemeldet.

`claude plugin validate` validiert auch jeden Eintrag, dessen `source` ein lokaler Pfad ist, und warnt, wenn die `version` des Eintrags mit der des eigenen Manifests des Plugins nicht übereinstimmt.

Die Tabelle listet die Marktplatz-Level-Meldungen auf. Eintrag-Level-Meldungen sind die Plugin-Meldungen unter [`claude plugin validate` meldet Fehler](#claude-plugin-validate-reports-errors), mit dem Präfix `plugins[N] plugin.json →`.

| Meldung                                                                                                                     | Art     | Beheben                                                                                                                                                              |
| :-------------------------------------------------------------------------------------------------------------------------- | :------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Duplicate plugin name "<name>" found in marketplace`                                                                       | Fehler  | Geben Sie jedem Plugin einen eindeutigen `name`.                                                                                                                     |
| `Path contains "..": <path>` unter `plugins[N].source`                                                                      | Fehler  | Verwenden Sie Pfade relativ zum Marktplatz-Root ohne `..`-Segmente.                                                                                                  |
| `Marketplace name cannot contain control or bidirectional-formatting characters`                                            | Fehler  | Entfernen Sie das Zeichen aus dem Namen, wie ein Escape oder eine Newline.                                                                                           |
| `Plugin name cannot contain control or bidirectional-formatting characters`                                                 | Fehler  | Entfernen Sie das Zeichen aus dem Plugin-`name`.                                                                                                                     |
| `Marketplace has no plugins defined`                                                                                        | Warnung | Fügen Sie mindestens einen Eintrag zu `plugins` hinzu.                                                                                                               |
| `No marketplace description provided`                                                                                       | Warnung | Fügen Sie eine Top-Level-`description` hinzu.                                                                                                                        |
| `Plugin name "<name>" is not kebab-case` unter `plugins[N] plugin.json → name`                                              | Warnung | Benennen Sie in Kleinbuchstaben, Ziffern und Bindestriche um. Claude Code akzeptiert andere Formen, aber die claude.ai-Marktplatz-Synchronisierung lehnt sie ab.     |
| `Entry declares version "<a>" but <path>/plugin.json says "<b>"`                                                            | Warnung | Aktualisieren Sie den Eintrag, um `plugin.json` zu entsprechen, das zur Installationszeit maßgeblich ist.                                                            |
| `Marketplace name "<name>" is reserved in Claude Desktop`                                                                   | Warnung | Benennen Sie den Marktplatz um. Die verwaltete Marktplatz-Synchronisierung von Claude Desktop lehnt `org`, `org-provisioned` und `unknown` in jeder Schreibweise ab. |
| `Marketplace name "<name>" is not accepted by Claude Desktop` oder `Plugin name "<name>" is not accepted by Claude Desktop` | Warnung | Benennen Sie in höchstens 128 Zeichen aus Buchstaben, Ziffern, `.`, `_` und `-` um, beginnend mit einem Buchstaben oder einer Ziffer.                                |

Vor v2.1.247 wurde ein Marktplatz-Name, der Kontroll- oder bidirektionale Formatierungszeichen enthielt, nur als `Marketplace name impersonates an official Anthropic/Claude marketplace` gemeldet.

<h2 id="blocked-by-your-organization">
  Blockiert durch Ihre Organisation
</h2>

Ihre Organisation stellt verwaltete Einstellungen bereit, die Plugins einschränken, und ein Befehl wurde mit einer Richtlinienmeldung abgelehnt. Diese Einträge benennen die Einstellung hinter jeder Ablehnung, damit Sie wissen, was Sie Ihren Administrator fragen müssen. Für die Admin-Seite siehe [Verwalten Sie Plugins für Ihre Organisation](/docs/de/plugins/org).

<h3 id="marketplace-source-is-blocked-by-enterprise-policy">
  `Marketplace source '<source>' is blocked by enterprise policy`
</h3>

Sie haben `/plugin marketplace add`, `update` oder eine Installation ausgeführt, und Claude Code hat mit dieser Zeile abgelehnt. Für eine GitHub- oder Git-Quelle folgt der Host der Quelle in Klammern, wie in `'github:owner/repo' (github.com)`.

Ihr Administrator hat `blockedMarketplaces` oder `strictKnownMarketplaces` in verwalteten Einstellungen gesetzt, und diese Quelle ist nicht zulässig. Bitten Sie Ihren Administrator, die Quelle zu erlauben, oder fügen Sie eine der zulässigen Quellen hinzu, die die Meldung auflistet.

Passen Sie den Rest der Meldung an, um zu sehen, welche Art von Richtlinie die Quelle blockiert hat:

* **`Allowed sources: <list>`**: der Block kommt von der `strictKnownMarketplaces`-Zulassungsliste statt von der `blockedMarketplaces`-Blockliste
* **`No external marketplaces are allowed.`**: die `strictKnownMarketplaces`-Zulassungsliste ist leer
* **Ein `Tip:`, dass der Kurzbefehl github.com annimmt**: die Zulassungsliste erlaubt einen Git-Host nach Hostname, und der `owner/repo`-Kurzbefehl, den Sie übergeben haben, zeigt auf github.com. Wenn sich das Repository auf Ihrem internen Host befindet, fügen Sie es erneut mit seiner vollständigen URL hinzu, wie `git@your-git-host.com:owner/repo.git`

Ein Marktplatz, den Sie vor der Richtlinie hinzugefügt haben, wird restriktiver, stoppt auch das Aktualisieren, weil die Richtlinie bei jeder Aktualisierung angewendet wird.

<h3 id="marketplace-is-not-in-the-allowed-marketplace-list">
  `Marketplace "<name>" is not in the allowed marketplace list`
</h3>

Die Registerkarte **Errors** zeigt diese Zeile, oder `Marketplace "<name>" is blocked by enterprise policy`, für einen Marktplatz, den Sie bereits registriert haben.

Die gleichen verwalteten Einstellungen, die eine [Marktplatzquelle](#marketplace-source-is-blocked-by-enterprise-policy) blockieren, gelten zur Ladezeit. `strictKnownMarketplaces` umfasst diesen Marktplatz nicht, oder `blockedMarketplaces` benennt ihn, daher stoppt Claude Code das Laden und seine Plugins. Für die Zulassungslisten-Variante zeigt die Anleitung die zulässigen Quellen oder `Contact your administrator to configure allowed marketplace sources`. Für die Blocklisten-Variante lautet sie `This marketplace source is explicitly blocked by your administrator`.

<h3 id="plugin-is-blocked-by-your-organizations-policy-and-cannot-be-installed">
  `Plugin "<name>" is blocked by your organization's policy and cannot be installed`
</h3>

Eine Installation wurde mit dieser Zeile abgelehnt, eine Aktivierung mit derselben Zeile, die mit `cannot be enabled` endet, oder eine Installation oder Aktualisierung mit einer, die den Grund benennt: `Plugin "<name>" is from marketplace "<marketplace>", which is blocked by your organization's policy`, oder `Plugin "<name>" depends on "<dep>", which is blocked by your organization's policy`.

Verwaltete Einstellungen blockieren dieses Plugin, seinen Marktplatz oder eine Abhängigkeit, die es benötigt. Fragen Sie Ihren Administrator, welcher Eintrag zutrifft. Ein blockierter Abhängigkeit bedeutet, dass das Plugin nicht installiert werden kann, bis der Marktplatz der Abhängigkeit zulässig ist.

<h3 id="plugin-dir-is-disabled-by-your-organizations-managed-settings-disables">
  `--plugin-dir is disabled by your organization's managed settings (disableSideloadFlags)`
</h3>

Sie haben `claude` mit `--plugin-dir`, `--plugin-url`, `--agents` oder `--mcp-config` gestartet. Claude Code hat mit dieser Meldung beendet und `Plugins, custom agents, and MCP servers can only be loaded from sources your administrator has approved.`

Ihr Administrator hat `disableSideloadFlags` in verwalteten Einstellungen gesetzt, was die Flags ausschaltet, die Plugins, Agents und Server aus beliebigen Pfaden laden. Laden Sie das Plugin stattdessen von einem genehmigten Marktplatz, oder bitten Sie Ihren Administrator, die Einstellung zu entfernen.

Eine zugehörige Meldung in der Registerkarte `/plugin` **Errors** ist `--plugin-dir copy of "<name>" ignored: plugin is locked by managed settings`. Verwaltete Einstellungen aktivieren oder deaktivieren dieses Plugin nach Name, und Claude Code ignoriert Ihre `--plugin-dir`-Kopie davon, damit das Flag die Richtlinie nicht überschreiben kann.

<h3 id="plugins-from-claude-skills-are-blocked-by-your-organizations-managed-s">
  `Plugins from ~/.claude/skills/ are blocked by your organization's managed settings`
</h3>

Sie haben `claude plugin init` oder `claude plugin enable` ausgeführt, und es hat mit dieser Zeile gestoppt. Die Meldung benennt `strictKnownMarketplaces or blockedMarketplaces` und bittet Ihren Administrator, `{"source":"skills-dir"}` zu `strictKnownMarketplaces` hinzuzufügen oder es aus `blockedMarketplaces` zu entfernen.

Die `skills-dir`-Quelle steht für Plugins, die Claude Code aus Ihrem `~/.claude/skills/`-Verzeichnis lädt. Bitten Sie Ihren Administrator, die Änderung vorzunehmen, die die Meldung benennt.

<h3 id="command-sourced-plugins-are-disabled-by-your-organizations-managed-set">
  `Command-sourced plugins are disabled by your organization's managed settings`
</h3>

Sie haben ein Plugin mit einer `command`-Quelle installiert oder aktualisiert, und es hat mit dieser Zeile gestoppt und `The plugin was not installed or updated and its command was not run.`

Ihr Administrator hat `disableCommandPluginSources` gesetzt, daher weigert sich Claude Code, den Marktplatz-deklarierten Befehl auszuführen, der das Plugin erzeugt. Das Setzen von `allowManagedHooksOnly` allein hat denselben Effekt, wenn `disableCommandPluginSources` ungesetzt ist. Fragen Sie Ihren Administrator, ob das Plugin von einem Quellentyp veröffentlicht werden kann, den die Richtlinie erlaubt.

<h3 id="marketplace-is-seed-managed">
  `Marketplace '<name>' is seed-managed`
</h3>

Sie haben `claude plugin marketplace update <name>` ausgeführt, und es ist mit `Marketplace '<name>' is seed-managed (<dir>)` fehlgeschlagen und einem Hinweis, Ihren Admin zu fragen.

Ein Operator hat diesen Marktplatz durch `CLAUDE_CODE_PLUGIN_SEED_DIR` vorpopuliert, und Claude Code behandelt einen Seed-verwalteten Marktplatz als schreibgeschützt. Eine Massen-`marketplace update` überspringt ihn und aktualisiert die anderen.

Um den Inhalt des Marktplatzes zu ändern, fragen Sie die Person, die das Seed-Image verwaltet, um es zu aktualisieren. Für das Verfahren siehe [Seed-Container und CI](/docs/de/plugins/org#seed-containers-and-ci).

<h2 id="next-steps">
  Nächste Schritte
</h2>

* [Plugin-Ladeverzeichnis](/docs/de/plugins/loading): warum sich Bereiche, der Cache und die Priorität so verhalten
* [Plugin-Befehlsreferenz](/docs/de/plugins/cli-reference): Flags, Standardwerte, Ausgabe und Exit-Codes für die `claude plugin`-Befehle
* [Installieren und verwalten Sie Plugins](/docs/de/plugins/install): die Installationsschritte von Anfang an
* [Verwalten Sie Plugins für Ihre Organisation](/docs/de/plugins/org#troubleshoot-policy): Richtlinien-seitige Fehlerbehebung für Administratoren
