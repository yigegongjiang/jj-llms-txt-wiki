> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Ein Plugin veröffentlichen und verteilen

> Veröffentlichen Sie ein Claude Code Plugin über Ihren eigenen Marketplace oder Anthropics Community Marketplace, mit einer Pre-Release-Checkliste und wie Benutzer Updates erhalten.

Ein Claude Code Plugin zu veröffentlichen bedeutet, es in einem Marketplace aufzulisten – einem JSON-Katalog, der Plugins auflistet und angibt, wo jedes abgerufen werden kann – damit andere Personen es nach Name installieren können und Ihre Updates erhalten. Sie können Ihren eigenen Marketplace betreiben oder Ihr Plugin bei Anthropics Community Marketplace einreichen. Um ein Plugin ohne Veröffentlichung zu teilen, senden Sie Personen das Plugin-Verzeichnis oder eine `.zip` davon zum selbst Laden.

Diese Seite ist für den Autor eines funktionierenden Plugins, der bereit ist, es zu teilen.

<Note>
  Diese Fälle werden auf anderen Seiten behandelt:

  * **Ihr Plugin ist noch nicht fertig**: Beginnen Sie mit [Ein Plugin erstellen](/docs/de/plugins/create)
  * **Sie verwalten eine CLI oder SDK mit einem Plugin in einem offiziellen Marketplace**: siehe [Empfehlen Sie Ihr Plugin von Ihrer CLI](/docs/de/plugins/cli-hints)
</Note>

Beginnen Sie mit [Wählen Sie, wie Sie verteilen](#choose-how-to-distribute), um die Verteilungsoptionen zu vergleichen. Wenn Sie Ihre Route bereits kennen, gehen Sie zu [Bereiten Sie Ihr Plugin für die Veröffentlichung vor](#prepare-your-plugin-for-release), und folgen Sie dann dem Abschnitt Ihrer Route für das, was Sie Ihren Benutzern mitteilen und wie sie Ihre Updates erhalten.

<h2 id="choose-how-to-distribute">
  Wählen Sie, wie Sie verteilen
</h2>

Wählen Sie eine Verteilungsoption basierend darauf, wer das Plugin installieren muss:

| Route                                                                    | Wer kann installieren                                                                   | Was Sie benötigen                                                                                              | Erhalten Benutzer Ihre Updates automatisch?       |
| :----------------------------------------------------------------------- | :-------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------- | :------------------------------------------------ |
| [Kein Marketplace](#share-a-plugin-without-a-marketplace)                | Die Personen, denen Sie den Plugin-Ordner oder eine `.zip` davon senden                 | Der Plugin-Ordner                                                                                              | Nein. Sie laden die Kopie, die Sie gesendet haben |
| [Ihr eigener Marketplace](#publish-through-your-own-marketplace)         | Jeder, der das Repository erreichen kann, das privat sein kann und Ihr Team klonen kann | Ein Git-Repository oder ein anderer Host mit einer `.claude-plugin/marketplace.json`, die Ihr Plugin auflistet | Aus                                               |
| [Anthropics Community Marketplace](#submit-to-the-community-marketplace) | Jeder, der `anthropics/claude-plugins-community` hinzufügt                              | Eine Einreichung über das Plugin-Verzeichnis-Einreichungsformular                                              | Aus                                               |

Auto-Update ist eine Einstellung pro Marketplace auf der Benutzerseite, die neue Versionen im Hintergrund abruft.

<h2 id="prepare-your-plugin-for-release">
  Bereiten Sie Ihr Plugin für die Veröffentlichung vor
</h2>

Der Name, die Version, die Validierung und eine Installation von einem Marketplace entscheiden, ob eine Veröffentlichung für die Personen funktioniert, die sie installieren. Überprüfen Sie diese vor der ersten Veröffentlichung und erneut vor jeder späteren.

<Steps>
  <Step title="Wählen Sie einen permanenten Namen">
    Benutzer installieren, aktivieren und konfigurieren Ihr Plugin nach `name@marketplace`, daher ist ein umbenanntes Plugin für jede vorhandene Installation ein anderes Plugin. Wählen Sie einen Kebab-Case-Namen wie `deploy-helper`, da `claude plugin validate` vor anderen Formen warnt, und behandeln Sie ihn als permanent. Setzen Sie `displayName` in `plugin.json` für das Label, das Benutzer sehen.
  </Step>

  <Step title="Entscheiden Sie, wie Sie versionieren">
    Wenn Sie `version` in `plugin.json` setzen und später Commits pushen, ohne es zu ändern, druckt `claude plugin update` `<name> is already at the latest version (1.0.0).` und Benutzer behalten die alte Kopie. Erhöhen Sie entweder `version` bei jeder Veröffentlichung, oder lassen Sie sie in einem Git-gehosteten Marketplace weg, damit Claude Code stattdessen den Commit SHA verwendet. Siehe [Versionen und Updates](/docs/de/plugins/loading#versions-and-updates).
  </Step>

  <Step title="Validieren">
    Führen Sie in Ihrer Shell `claude plugin validate --strict ./your-plugin` aus. Ein sauberer Durchlauf druckt `✔ Validation passed`.

    * **In CI**: Behalten Sie `--strict` bei, das auch den Durchlauf mit Exit-Code 1 bei Warnungen wie einem unbekannten Manifest-Feld oder einer fehlenden `version` fehlschlagen lässt. Lassen Sie `--strict` weg, wenn Sie sich im vorherigen Schritt entschieden haben, `version` wegzulassen.
    * **Pfade**: Die Validierung meldet Komponentenpfade, die nicht mit `./` beginnen. Beziehen Sie sich in Hook-Befehlen und MCP-Server-Konfigurationen auf Dateien als `${CLAUDE_PLUGIN_ROOT}/...`. Siehe [Pfadregeln](/docs/de/plugins/manifest-reference#path-rules).
  </Step>

  <Step title="Installieren Sie es von einem lokalen Marketplace">
    Fügen Sie in Ihrer Shell einen lokalen Marketplace hinzu, der das Plugin mit `claude plugin marketplace add ./path-to-marketplace` auflistet, installieren Sie das Plugin davon, und starten Sie eine Sitzung, um zu bestätigen, dass es geladen wird.

    * Für den kleinsten funktionierenden Marketplace siehe [Erstellen Sie einen Marketplace](/docs/de/plugins/create-marketplace).
    * Um zu wissen, ob eine Installation Ihr Quellverzeichnis oder eine zwischengespeicherte Kopie lädt, siehe [In-Place- und kopierte Plugins](/docs/de/plugins/loading#in-place-and-copied-plugins).
  </Step>

  <Step title="Füllen Sie die Metadaten aus, die Benutzer sehen">
    Setzen Sie `description`, `author`, `homepage` und `repository` in `plugin.json`, und fügen Sie eine `README.md` im Plugin-Root hinzu. `homepage` muss als URL analysierbar sein. Die [Manifest-Referenz](/docs/de/plugins/manifest-reference#fields) listet jedes Feld auf.
  </Step>

  <Step title="Führen Sie Ihre Eval-Suite aus">
    Wenn Sie eine Eval-Suite haben, führen Sie `claude plugin eval` in Ihrer Shell aus. Sie führt die Testfälle des Plugins aus und bewertet die Ergebnisse, was Regressionen erfasst, wenn Sie das Plugin ändern. Siehe [Testen Sie Plugins mit Evals](/docs/de/plugin-evals).
  </Step>
</Steps>

<h2 id="share-a-plugin-without-a-marketplace">
  Teilen Sie ein Plugin ohne einen Marketplace
</h2>

Wenn sich das Plugin in einem Git-Repository befindet, können Personen es klonen und den Checkout laden, oder Claude Code von ihrer Shell mit `--plugin-url` starten, das auf eine `.zip` zeigt, die Sie an eine Veröffentlichung anhängen. Um Ihre nächste Version zu erhalten, pullen oder laden sie erneut herunter. Wenn es sich nicht in einem Repository befindet, senden Sie ihnen das Verzeichnis oder eine `.zip` davon. Sie laden es auf eine von zwei Arten:

* **Für eine Sitzung**: Sie starten Claude Code von ihrer Shell mit `claude --plugin-dir ./deploy-helper`, wobei der Pfad der Klon, der entpackte Ordner oder die `.zip` selbst ist. Siehe [Flags, die ein Plugin für eine Sitzung laden](/docs/de/plugins/cli-reference#flags-that-load-a-plugin-for-one-session).
* **Für jede Sitzung**: Sie verschieben das Plugin-Verzeichnis mit seiner `.claude-plugin/plugin.json` unter `~/.claude/skills/`, damit Claude Code es [in jeder Sitzung lädt](/docs/de/plugins/loading#find-where-a-plugin-came-from).

Das Hinzufügen einer `.claude-plugin/marketplace.json` zu demselben Repository ist das, was Personen ermöglicht, nach Name zu installieren und mit einem Befehl zu aktualisieren; siehe [Veröffentlichen Sie über Ihren eigenen Marketplace](#publish-through-your-own-marketplace).

<h3 id="ship-a-plugin-with-your-own-tool">
  Versenden Sie ein Plugin mit Ihrem eigenen Tool
</h3>

Wenn Sie eine CLI oder SDK verwalten, veröffentlichen Sie das Plugin in einem Marketplace und lassen Sie Ihren Installer oder die Post-Install-Nachricht die zwei Befehle ausführen oder drucken, die ein Benutzer benötigt: `claude plugin marketplace add <source>`, dann `claude plugin install <name>@<marketplace>`. Für die In-Session-Erkennung, wenn jemand Ihr Tool verwendet, siehe [Empfehlen Sie Ihr Plugin von Ihrer CLI](/docs/de/plugins/cli-hints).

<h2 id="publish-through-your-own-marketplace">
  Veröffentlichen Sie über Ihren eigenen Marketplace
</h2>

Ihr eigener Marketplace ist eine `.claude-plugin/marketplace.json`-Datei, die Ihr Plugin auflistet und zu einem Git-Repository hinzugefügt wird. Sobald sich die Datei im Repository befindet, wird das Plugin veröffentlicht, ohne Einreichungsformular. Sie können die Datei im eigenen Repository des Plugins oder in einem separaten aufbewahren.

<h3 id="add-the-marketplace-file-to-your-repository">
  Fügen Sie die Marketplace-Datei zu Ihrem Repository hinzu
</h3>

Um vom eigenen Repository des Plugins aus zu veröffentlichen, speichern Sie die Marketplace-Datei neben `plugin.json` in `.claude-plugin/`, mit einem Eintrag, dessen `source` `"./"` ist, das Repository-Root. Geben Sie dem Eintrag denselben `name` wie `plugin.json`, gemäß [Halten Sie den Eintragnamen und den Manifest-Namen gleich](/docs/de/plugins/create-marketplace#keep-the-entry-name-and-the-manifest-name-the-same):

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Name" },
  "plugins": [
    { "name": "deploy-helper", "source": "./" }
  ]
}
```

Führen Sie in Ihrer Shell `claude plugin validate .` im Repository aus, um die Datei vor dem Push zu überprüfen.

[Erstellen Sie einen Marketplace](/docs/de/plugins/create-marketplace) behandelt das Layout mit mehreren Plugins in einem Repository.

<h3 id="control-who-can-install">
  Kontrollieren Sie, wer installieren kann
</h3>

Jeder, der das Repository klonen kann, kann davon installieren, daher ist der Marketplace auch privat, wenn das Repository privat ist. Für Hosts außer einem Git-Repository siehe [Hosten Sie einen Marketplace](/docs/de/plugins/host-marketplace). Um alle in einem Unternehmen zu erreichen, einschließlich Personen, die kein Git verwenden, siehe [Rollout für ein ganzes Unternehmen](/docs/de/plugins/host-marketplace#roll-out-to-a-whole-company).

<h3 id="tell-users-how-to-install">
  Teilen Sie Benutzern mit, wie sie installieren
</h3>

Teilen Sie Ihren Benutzern mit, den Marketplace hinzuzufügen und dann das Plugin von ihrer Shell zu installieren, wobei Sie die Quelle und Namen durch Ihre ersetzen:

* Fügen Sie den Marketplace einmal hinzu: `claude plugin marketplace add your-org/your-marketplace`, wobei das Argument eine GitHub `owner/repo`-Kurzform, eine URL oder ein Pfad ist
* Installieren Sie das Plugin: `claude plugin install deploy-helper@your-marketplace`
* Oder tun Sie beides von innerhalb einer Sitzung: `/plugin install deploy-helper --marketplace your-org/your-marketplace`. Erfordert Claude Code v2.1.275 oder später. Siehe [Fügen Sie einen Marketplace hinzu und installieren Sie in einem Befehl](/docs/de/plugins/install#add-a-marketplace-and-install-in-one-command)

<h3 id="ship-updates-to-users">
  Versenden Sie Updates an Benutzer
</h3>

Benutzer erhalten eine Veröffentlichung, wenn sie danach fragen oder wenn Auto-Update für Ihren Marketplace aktiviert ist:

* **Auf Anfrage**: `claude plugin update deploy-helper@your-marketplace` in der Shell des Benutzers aktualisiert den Marketplace und installiert die neue Kopie, wenn sich die Version Ihres Plugins geändert hat
* **Auto-Update**: standardmäßig für Ihren Marketplace deaktiviert. Siehe [Aktivieren Sie Auto-Update](/docs/de/plugins/host-marketplace#turn-on-auto-update). Sobald aktiviert, tut es dasselbe wie `claude plugin update` mit einer Verzögerung nach dem Sitzungsstart

[Installieren Sie Plugins](/docs/de/plugins/install) behandelt die Befehle auf der Benutzerseite, und [wenn Auto-Update ausgeführt wird](/docs/de/plugins/loading#when-auto-update-runs) behandelt das Timing.

<h2 id="submit-to-the-community-marketplace">
  Reichen Sie beim Community Marketplace ein
</h2>

Anthropics Community Marketplace, `claude-community`, ist der öffentliche Marketplace, der Plugins auflistet, die über das Plugin-Verzeichnis-Einreichungsformular eingereicht wurden.

Benutzer fügen den Community Marketplace in einer Claude Code-Sitzung mit `/plugin marketplace add anthropics/claude-plugins-community` hinzu und installieren davon als `@claude-community`.

Für die Unterschiede zwischen dem Community Marketplace und dem offiziellen Marketplace siehe [Anthropics Marketplaces](/docs/de/plugins/anthropic-marketplaces).

Um Ihr Plugin beim Community Marketplace einzureichen, verwenden Sie eines der In-App-Formulare:

* **claude.ai**: [claude.ai/admin-settings/directory/submissions/plugins/new](https://claude.ai/admin-settings/directory/submissions/plugins/new)
* **Console**: [platform.claude.com/plugins/submit](https://platform.claude.com/plugins/submit)

Das claude.ai-Formular erfordert eine Team- oder Enterprise-Organisation und die Directory-Berechtigung, die Eigentümer standardmäßig haben. Einzelne Autoren, die nicht Teil einer Team- oder Enterprise-Organisation sind, können stattdessen das Console-Formular verwenden.

Führen Sie in Ihrer Shell `claude plugin validate ./your-plugin` lokal aus, bevor Sie einreichen, wobei Sie `./your-plugin` durch den Pfad zu Ihrem Plugin-Verzeichnis ersetzen. Wenn die Validierung erfolgreich ist, druckt Claude Code `✔ Validation passed`, oder `✔ Validation passed with warnings`, wenn es Warnungen gibt. Warnungen führen nicht zu Validierungsfehlern; fügen Sie `--strict` hinzu, um sie als Fehler zu behandeln.

Aufgelistete Plugins erscheinen im [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community)-Katalog, in fast jedem Fall an einen bestimmten Commit SHA angeheftet.

Es kann eine Verzögerung zwischen der Einreichung und dem Erscheinen Ihres Plugins in `marketplace.json` geben. Um zu überprüfen, ob Ihr Plugin bereits installierbar ist, suchen Sie nach seinem Namen im [Community-Katalog](https://github.com/anthropics/claude-plugins-community/blob/main/.claude-plugin/marketplace.json).

Der offizielle Marketplace, `claude-plugins-official`, akzeptiert keine Einreichungen über diese Formulare. Wenn Sie mit einem Anthropic-Partner-Kontakt zusammenarbeiten, fragen Sie ihn nach einer Auflistung im offiziellen Marketplace.

<h2 id="ship-updates-renames-and-removals">
  Versenden Sie Updates, Umbenennungen und Entfernungen
</h2>

<h3 id="release-a-new-version">
  Veröffentlichen Sie eine neue Version
</h3>

Wenn Sie über Ihren eigenen Marketplace veröffentlichen und Ihre `plugin.json` `version` setzt, erhöhen Sie sie und pushen Sie. Benutzer, die `claude plugin update` ausführen oder Auto-Update aktiviert haben, erhalten dann die neue Version, wie unter [Versenden Sie Updates an Benutzer](#ship-updates-to-users) beschrieben.

<h3 id="tag-a-release">
  Markieren Sie eine Veröffentlichung
</h3>

Markieren Sie die Veröffentlichung in Git, wenn andere Plugins einen Versionsbereich auf Ihrem deklarieren, da diese Bereiche gegen Tags aufgelöst werden. Andernfalls benötigen Sie keinen Tag.

Um zu markieren, führen Sie `claude plugin tag` in Ihrer Shell aus dem Plugin-Verzeichnis aus. Es erstellt einen `{name}--v{version}`-Tag. Fügen Sie `--push` hinzu, um den Tag an `origin` zu senden. Die [`plugin tag`-Referenz](/docs/de/plugins/cli-reference#plugin-tag) listet ihre Flags auf.

<h3 id="rename-or-remove-a-plugin">
  Benennen Sie ein Plugin um oder entfernen Sie es
</h3>

Ändern Sie niemals den `name` eines veröffentlichten Plugins. Nach einer Umbenennung verlieren Benutzer, die es bereits installiert haben, das Plugin, da ihre Installation unter dem alten Namen aufgezeichnet ist. Ein `renames`-Eintrag in Ihrer Marketplace-Datei migriert sie stattdessen. Ändern Sie `displayName`, wenn Sie ein anderes Label möchten.

Wenn eine Umbenennung unvermeidlich ist, verwenden Sie die `renames`-Zuordnung der Marketplace-Datei, damit vorhandene Installationen migrieren, anstatt mit [`Plugin "<name>" not found in marketplace`](/docs/de/plugins/troubleshooting#plugin-not-found-in-marketplace) fehlzuschlagen. Um ein Plugin aus dem Marketplace zu entfernen, oder für die vollständigen `renames`-Details, siehe [Benennen Sie ein Plugin um oder entfernen Sie es](/docs/de/plugins/host-marketplace#rename-or-remove-a-plugin) auf der Hosting-Seite. Die [Marketplace-Referenz](/docs/de/plugins/marketplace-reference#top-level-fields) hat das Feld.

<h2 id="declare-dependencies">
  Deklarieren Sie Abhängigkeiten
</h2>

Wenn Ihr Plugin ein anderes Plugin aus demselben Marketplace benötigt, das aktiviert sein soll, listen Sie es im `dependencies`-Array von `plugin.json` auf. Jeder Eintrag ist ein einfacher Name oder ein Objekt mit einem Semver-`version`-Bereich. Wenn ein Benutzer Ihr Plugin installiert, installiert und aktiviert Claude Code auch die Abhängigkeit.

[Plugin-Abhängigkeiten](/docs/de/plugins/dependencies) behandelt die Bereichssyntax, Marketplace-übergreifende Abhängigkeiten und wie Benutzer Abhängigkeiten, die sie nicht mehr benötigen, bereinigen.

<h2 id="next-steps">
  Nächste Schritte
</h2>

* [Hosten und verwalten Sie einen Marketplace](/docs/de/plugins/host-marketplace): Veröffentlichen Sie neue Versionen und halten Sie Benutzer auf dem Laufenden
* [Plugin-Abhängigkeiten](/docs/de/plugins/dependencies): Deklarieren und versionieren Sie die Plugins, auf die Ihr Plugin angewiesen ist
* [Empfehlen Sie Ihr Plugin von Ihrer CLI](/docs/de/plugins/cli-hints): Fordern Sie Claude Code-Benutzer Ihrer CLI auf, das Plugin zu installieren
* [Messen Sie Plugin-Kosten und -Nutzung](/docs/de/plugins/measure): Sehen Sie, was Ihr Plugin im Kontext kostet und ob Personen es verwenden
