> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugins installieren und verwalten

> Installieren Sie Claude Code-Plugins aus einem Marketplace auf jeder Oberfläche, die Sie verwenden, wählen Sie einen Installationsbereich aus, und aktualisieren oder entfernen Sie sie später.

Das Installieren eines Plugins fügt seine Skills, Agents, Hooks und MCP-Server zu Claude Code auf Ihrem Computer hinzu.

Diese Seite ist für alle gedacht, die Plugins auf ihrem eigenen Computer oder Konto verwenden, ob im Terminal, in der Desktop-App, einer IDE oder einer Cloud-Sitzung: Sie behandelt Installation, Bereichswahl, Hinzufügen von Marketplaces und Aktualisierung von Plugins.

<Note>
  Diese Fälle werden auf anderen Seiten behandelt:

  * **Sie verwenden claude.ai Chat oder Cowork, nicht Claude Code**: siehe [Plugins auf claude.ai und in Cowork](https://claude.com/docs/plugins/overview)
  * **Claude Code hat einen Fehler ausgegeben**: finden Sie ihn unter [Plugins fehlerbeheben](/docs/de/plugins/troubleshooting)
</Note>

Beginnen Sie mit [Plugin installieren](#install-a-plugin). Wenn Ihnen jemand einen Installationsbefehl gesendet hat, dessen `@`-Name nicht `claude-plugins-official` ist, [fügen Sie zuerst diesen Marketplace hinzu](#add-a-marketplace).

<h2 id="install-a-plugin">
  Plugin installieren
</h2>

Als Beispiel installiert dieser Abschnitt [`commit-commands`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/commit-commands) aus [Anthropics offiziellem Marketplace](/docs/de/plugins/anthropic-marketplaces), das Befehle zum Committen, Pushen und Öffnen von Pull Requests hinzufügt.

Die gleichen Schritte installieren jedes andere Plugin: ersetzen Sie seinen Namen und den Namen seines Marketplace überall dort, wo `commit-commands` und `claude-plugins-official` erscheinen. Wenn dieses Plugin aus einem anderen Marketplace stammt, [fügen Sie zuerst den Marketplace hinzu](#add-a-marketplace).

Wählen Sie den Tab für den Ort, an dem Sie Claude Code ausführen.

<Tabs>
  <Tab title="Terminal">
    Starten Sie Claude Code mit `claude` in Ihrem Projekt, dann:

    <Steps>
      <Step title="Öffnen Sie die Details des Plugins mit dem Installationsbefehl">
        Führen Sie `/plugin install` mit dem Namen des Plugins und des Marketplace aus. In einer Sitzung installiert dieser Befehl nicht sofort: Er öffnet das `/plugin`-Panel mit den Details dieses Plugins, damit Sie es überprüfen und zuerst einen Bereich wählen können.

        ```text theme={null}
        /plugin install commit-commands@claude-plugins-official
        ```

        Um stattdessen zu durchsuchen, führen Sie `/plugin` ohne Plugin-Namen aus: Das Panel öffnet sich auf der Registerkarte **Discover**, die Plugins aus jedem hinzugefügten Marketplace auflistet, und Sie können eingeben, um zu suchen, dann **Enter** auf einem Plugin drücken, um seine Details zu öffnen.
      </Step>

      <Step title="Überprüfen Sie, was das Plugin hinzufügt">
        Der Detailbereich zeigt die Beschreibung des Plugins. Es kann auch anzeigen:

        * **Will install**: die Befehle, Agents, Skills, Hooks und MCP- und LSP-Server, die das Plugin hinzufügt.
        * **Last updated**: wird für ein Plugin im offiziellen Marketplace von Anthropic angezeigt.
        * **Context cost**: für ein Plugin im offiziellen Marketplace von Anthropic zwei Token-Schätzungen. **Every turn** ist das, was das Plugin zu jeder Nachricht hinzufügt, die Sie senden, und **When invoked** ist das, was seine Skills und Agents hinzufügen, sobald Claude sie lädt. Die Schätzungen erscheinen, wenn Sie das Plugin öffnen, indem Sie seinen Marketplace benennen, wie der Befehl in Schritt 1, oder von der Registerkarte **Marketplaces**. Der Detailbereich, den Sie aus der Liste **Discover** erreichen, zeigt sie nicht.

        Plugins aus einem lokalen oder benutzerdefinierten Marketplace können stattdessen `Components will be discovered at installation` anzeigen.

        Ein Plugin kann Hooks und MCP-Server ausführen, daher lesen Sie den Bereich, bevor Sie installieren. Siehe [Plugin-Sicherheit und Vertrauen](/docs/de/plugins/security).
      </Step>

      <Step title="Wählen Sie einen Bereich">
        Wählen Sie eine der drei Installationsoptionen:

        * **Install for you (user scope)**: Sie erhalten das Plugin in jedem Projekt auf diesem Computer
        * **Install for all collaborators on this repository (project scope)**: Es ist für alle aktiviert, die in diesem Repository arbeiten
        * **Install for you, in this repo only (local scope)**: Sie erhalten es nur in diesem Repository

        [Wählen Sie einen Installationsbereich](#choose-an-install-scope) sagt, welche Einstellungsdatei jeder schreibt und welche gilt, wenn das gleiche Plugin an mehr als einem Ort gesetzt ist.

        Nachdem Sie einen Bereich ausgewählt haben, installiert Claude Code das Plugin zusammen mit allen Abhängigkeiten, die es deklariert, und druckt dann eine Installationszusammenfassung.
      </Step>

      <Step title="Lesen Sie die Installationszusammenfassung">
        Der letzte Satz der Zusammenfassung sagt Ihnen, ob das Plugin in dieser Sitzung bereits verwendbar ist:

        * **Active now**: `Plugin is now active.` Es ist kein Neuladen erforderlich.
        * **Reload needed**: `Run /reload-plugins to activate.` Das Panel schließt sich und Claude Code führt dieses Neuladen für Sie aus. Wenn das Neuladen den [Prompt-Cache ungültig machen würde](/docs/de/prompt-caching#enabling-or-disabling-a-plugin), warnt es und lässt das Plugin stattdessen ausstehend. Führen Sie `/reload-plugins --force` aus, um es trotzdem zu aktivieren, was eine nicht gecachte Anfrage kostet.
        * **Load failed**: `The plugin couldn't be loaded`. Öffnen Sie die Registerkarte **Errors** in `/plugin`, um den Grund zu erfahren, und siehe dann [Nach Installation: Plugin funktioniert nicht](/docs/de/plugins/troubleshooting#plugin-installed-but-not-working).
      </Step>

      <Step title="Bestätigen Sie, dass das Plugin funktioniert">
        Geben Sie `/` ein und suchen Sie nach den Skills des Plugins unter seinem Namen in der Form `/<plugin>:<skill>`. Für `commit-commands` erscheint `/commit-commands:commit`. Zwei weitere Orte listen das Plugin auch auf:

        * Öffnen Sie die Registerkarte **Installed** in `/plugin`, die das Plugin mit seinem Bereich auflistet.
        * Führen Sie in Ihrer Shell `claude plugin list` aus, das die gleiche Liste mit `Version`-, `Scope`- und `Status`-Zeilen druckt.

        Wenn `/commit-commands:commit` nicht erscheint, siehe [Nach Installation: Plugin funktioniert nicht](/docs/de/plugins/troubleshooting#plugin-installed-but-not-working).
      </Step>
    </Steps>

    Die Installation aus einem anderen Marketplace erfordert zuerst einen zusätzlichen Schritt: [fügen Sie den Marketplace hinzu](#add-a-marketplace). Claude Code fügt Anthropics offiziellen Marketplace für Sie hinzu, wenn Sie zum ersten Mal eine interaktive Terminal-Sitzung starten, weshalb das Beispiel diesen Schritt überspringt. Wenn Sie ein Plugin auf [claude.com/marketplace](https://claude.com/marketplace) gefunden haben, kopiert seine Schaltfläche **Claude Code** den Installationsbefehl in seiner [Shell-Form](#install-from-your-shell), `claude plugin install <name>@claude-plugins-official`.
  </Tab>

  <Tab title="Desktop app">
    In einer lokalen oder SSH-Sitzung in der Registerkarte **Code** der Desktop-App:

    <Steps>
      <Step title="Öffnen Sie den Plugin-Browser">
        Klicken Sie auf die Schaltfläche **+** neben dem Eingabefeld und wählen Sie **Plugins**, dann **Add plugin**. Der Plugin-Browser öffnet sich mit den Plugins aus Ihren Marketplaces.
      </Step>

      <Step title="Wählen Sie das Plugin">
        Finden Sie `commit-commands` und wählen Sie es aus.
      </Step>

      <Step title="Wählen Sie einen Bereich">
        Wählen Sie einen [Bereich](#choose-an-install-scope): Ihr Benutzerkonto, dieses Projekt oder nur lokal.
      </Step>
    </Steps>

    Um später zu aktivieren, zu deaktivieren oder zu deinstallieren, verwenden Sie **+ > Plugins > Manage plugins**. Der Plugin-Browser ist in Cloud-Sitzungen der Desktop-App nicht verfügbar. Siehe [Plugins in der Desktop-App installieren](/docs/de/desktop#install-plugins).
  </Tab>

  <Tab title="VS Code">
    Im Claude Code-Panel in VS Code:

    <Steps>
      <Step title="Öffnen Sie Plugins verwalten">
        Geben Sie `/plugins` im Eingabefeld ein, um **Manage plugins** zu öffnen.
      </Step>

      <Step title="Installieren Sie das Plugin">
        Suchen Sie auf der Registerkarte **Plugins** nach `commit-commands` und klicken Sie auf **Install**. Wenn die Registerkarte keine Plugins auflistet, fügen Sie zuerst `anthropics/claude-plugins-official` auf der Registerkarte **Marketplaces** hinzu.
      </Step>

      <Step title="Wählen Sie einen Bereich">
        Wählen Sie einen [Bereich](#choose-an-install-scope): **Install for you**, **Install for this project** oder **Install locally**.
      </Step>
    </Steps>

    Ihre Änderungen gelten für offene Sitzungen ohne Neustart. Siehe [Plugins in VS Code verwalten](/docs/de/vs-code#manage-plugins).
  </Tab>

  <Tab title="Cloud session">
    Eine [Cloud-Sitzung](/docs/de/cloud-environments), einschließlich [des Browsers unter claude.ai/code](/docs/de/claude-code-on-the-web), hat keinen Plugin-Browser und lädt nicht die Plugins, die Sie auf Ihrem eigenen Computer installiert haben, oder die, die die `.claude/settings.json` Ihres Repositorys aktiviert. Für Plugins, die Ihre Organisation über verwaltete Einstellungen verteilt, siehe [Plugins für Ihre Organisation verwalten](/docs/de/plugins/org).

    Siehe [welche Teile Ihres Setups auch in einer Cloud-Sitzung verfügbar sind](/docs/de/cloud-environments#what-carries-over-from-your-setup) für den Rest Ihres Setups.
  </Tab>
</Tabs>

<h3 id="choose-an-install-scope">
  Wählen Sie einen Installationsbereich
</h3>

Der Installationsbereich eines Plugins entscheidet, wer das Plugin erhält und welche Einstellungsdatei es als aktiviert aufzeichnet:

* **User scope**: Das Plugin ist für Sie in jedem Projekt auf diesem Computer aktiviert. Der Eintrag geht in `enabledPlugins` in `~/.claude/settings.json`.
* **Project scope**: Das Plugin ist für alle aktiviert, die in diesem Repository arbeiten. Der Eintrag geht in `.claude/settings.json`, das Sie committen.
* **Local scope**: Das Plugin ist für Sie nur in diesem Repository aktiviert. Der Eintrag geht in `.claude/settings.local.json`.

Einige Plugins sind vom Autor so eingestellt, dass sie standardmäßig ausgeschaltet sind, über das Feld [`defaultEnabled`](/docs/de/plugins/manifest-reference#defaultenabled). Ein solches Plugin wird installiert, bleibt aber aus, bis Sie es mit `claude plugin enable <name>` in Ihrer Shell oder von der Registerkarte **Installed** von `/plugin` in einer Sitzung aktivieren.

Wenn das gleiche Plugin an mehreren Bereichen gesetzt ist, überschreibt die lokale Einstellung die Projekteinstellung, und die Projekteinstellung überschreibt die Benutzereinstellung. Siehe [Finden Sie, wo ein Plugin aktiviert ist](/docs/de/plugins/loading#find-where-a-plugin-is-enabled) für die vollständige Regel.

Das Terminal, die lokalen Sitzungen der Desktop-App und die VS Code-Erweiterung auf einem Computer lesen die gleichen Einstellungsdateien, daher ist ein Plugin, das Sie in einem von ihnen im Benutzerbereich installieren, auch in den anderen beiden verfügbar.

<h3 id="other-places-you-run-claude-code">
  JetBrains, nicht-interaktive Ausführungen und das Agent SDK
</h3>

Einige Orte, an denen Sie Claude Code ausführen, haben keinen eigenen Plugin-Browser:

* **JetBrains IDEs**: Das JetBrains-Plugin führt Claude Code im Terminal der IDE aus, daher verwenden Sie die Schritte der Registerkarte **Terminal** dort.
* **`claude -p` und andere nicht-interaktive Ausführungen**: `/plugin` wird nicht ausgeführt, und Claude antwortet `/plugin isn't available in this environment.` Plugins, die Sie bereits installiert haben, werden geladen. Installieren und verwalten Sie sie von Ihrer Shell aus mit [`claude plugin`-Befehlen](#install-from-your-shell).
* **Agent SDK**: Laden Sie Plugins über die Plugin-Option des SDK. Siehe [Plugins im Agent SDK laden](/docs/de/agent-sdk/plugins).

Wenn Claude Code meldet, dass ein Plugin, das in der `.claude/settings.json` des Repositorys aktiviert ist, nicht installiert ist, siehe [In Projekteinstellungen aktiviert, aber nicht installiert](/docs/de/plugins/loading#enabled-in-project-settings-but-not-installed).

<Tip>
  Wenn Sie ein Plugin-Autor sind und eine Kopie Ihres Plugins auf der Festplatte testen, starten Sie Claude Code von Ihrer Shell aus mit `--plugin-dir`, um es für eine Sitzung zu laden, anstatt es zu installieren. Siehe [Flags, die ein Plugin für eine Sitzung laden](/docs/de/plugins/cli-reference#flags-that-load-a-plugin-for-one-session).
</Tip>

<h3 id="plugins-from-your-claude-ai-account">
  Plugins aus Ihrem claude.ai-Konto
</h3>

Ihr claude.ai-Konto ist eine separate Quelle von Plugins, neben den Marketplaces, aus denen Sie installieren:

* **What arrives**: Jedes Plugin, das Sie für Ihr claude.ai-Konto aktivieren, und jedes Plugin, das Ihre Organisation für ihre Mitglieder aktiviert. In einer Terminal-Sitzung werden sie im Hintergrund synchronisiert, jedes Mal wenn Sie Claude Code starten, während Sie mit diesem Konto angemeldet sind; in Cowork-Sitzungen werden sie heruntergeladen, wenn die Sitzung startet.
* **Where you see them**: in `/plugin` und `claude plugin list` unter der ID `<name>@synced`. Sie können eines ausschalten, es sei denn, Ihre Organisation verlangt es.
* **What doesn't go the other way**: Plugins, die Sie mit `/plugin` oder `claude plugin install` installieren, bleiben auf diesem Computer und werden nicht zu Ihrem claude.ai-Konto hinzugefügt.

Für Synchronisierungszeitpunkt, Anmeldeanforderungen und Ausschalten der Synchronisierung siehe [Plugins, die von claude.ai synchronisiert werden](/docs/de/plugins/loading#synced-plugins).

<h3 id="install-from-your-shell">
  Von Ihrer Shell installieren
</h3>

Führen Sie `claude plugin install` in Ihrer Shell aus, um ein Plugin zu installieren, ohne eine Claude Code-Sitzung zu starten, zum Beispiel aus einem Setup-Skript.

* **Scope**: Benutzerbereich standardmäßig. Übergeben Sie `--scope project` oder `--scope local`, um es zu ändern.
* **When the plugins load**: Plugins, die es installiert, werden geladen, wenn Sie Claude Code das nächste Mal starten, oder wenn Sie `/reload-plugins` in einer bereits offenen Sitzung ausführen.
* **The marketplace must be added first**: Auf einem Computer, auf dem noch niemand eine interaktive Claude Code-Sitzung geöffnet hat, ist der offizielle Marketplace nicht registriert, daher führt ein Skript, das von ihm installiert, zuerst `claude plugin marketplace add anthropics/claude-plugins-official` aus, bevor die Installation erfolgt.

```bash theme={null}
claude plugin install formatter@your-org --scope project
```

Der Befehl druckt `Successfully installed plugin: formatter@your-org (scope: project)`, wenn er fertig ist.

Einige Plugins installieren, indem sie einen Befehl ausführen, den ihr Marketplace benennt, genannt eine [`command`-Quelle](/docs/de/plugins/marketplace-reference#command-plugin-source). Claude Code zeigt Ihnen diesen Befehl und fragt Sie, ihn zu akzeptieren, bevor er ausgeführt wird. Ein Skript hat niemanden, um diese Aufforderung zu beantworten, daher übergeben Sie dort `--yes`, um es zu akzeptieren.

Für jedes `claude plugin install`-Flag siehe [plugin install](/docs/de/plugins/cli-reference#plugin-install).

<h2 id="add-a-marketplace">
  Marketplace hinzufügen
</h2>

Sie benötigen diesen Abschnitt nur, wenn das Plugin, das Sie möchten, nicht im offiziellen Marketplace von Anthropic ist, zum Beispiel eines, das ein Kollege veröffentlicht hat, oder eines aus dem Community-Marketplace von Anthropic.

Ein Marketplace ist ein Katalog von Plugins, und Claude Code muss einen Marketplace kennen, bevor Sie von ihm installieren können. Sie fügen einen Marketplace einmal hinzu. Danach erscheinen seine Plugins auf der Registerkarte **Discover** und installieren mit `/plugin install <plugin>@<marketplace>` in einer Sitzung oder `claude plugin install <plugin>@<marketplace>` in Ihrer Shell, wobei `<marketplace>` der Name ist, unter dem sich der Marketplace registriert hat. Um beides in einem Schritt zu tun, siehe [Marketplace hinzufügen und in einem Befehl installieren](#add-a-marketplace-and-install-in-one-command).

Führen Sie in einer Claude Code-Sitzung `/plugin marketplace add` gefolgt von der Quelle des Marketplace aus: ein GitHub-Repository, ein Git-Repository auf einem beliebigen Host, ein lokales Verzeichnis oder eine Datei, oder ein gehostetes `marketplace.json`.

| Quelle                                   | Was Sie eingeben                                                                                                                                                                                                                                             | Beispiel                                                                                                                                  |
| :--------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| GitHub-Repository                        | `owner/repo`. Fügen Sie `#ref` hinzu, um einen Branch oder Tag zu fixieren.                                                                                                                                                                                  | `/plugin marketplace add anthropics/claude-code`, oder `/plugin marketplace add your-org/plugins#v1.2.0`, um den Tag `v1.2.0` zu fixieren |
| Git-Repository auf einem beliebigen Host | Die vollständige Clone-URL. Fügen Sie `#ref` hinzu, um einen Branch oder Tag zu fixieren.                                                                                                                                                                    | `/plugin marketplace add https://gitlab.example.com/your-group/your-marketplace.git#v1.0.0`                                               |
| Lokales Verzeichnis oder Datei           | Ein relativer oder absoluter Pfad zu einem Verzeichnis, das `.claude-plugin/marketplace.json` enthält, oder zur JSON-Datei selbst. Beginnen Sie einen relativen Pfad mit `./` oder `../`, da Claude Code ein bloßes `name/name` als GitHub-Repository liest. | `/plugin marketplace add ./my-marketplace`                                                                                                |
| Gehostetes `marketplace.json`            | Seine `https://`-URL                                                                                                                                                                                                                                         | `/plugin marketplace add https://example.com/marketplace.json`                                                                            |

Von Ihrer Shell aus nimmt `claude plugin marketplace add` die gleichen Quellen.

<Tip>
  `/plugin market` funktioniert auch als kürzere Form von `/plugin marketplace`.
</Tip>

Fügen Sie das Präfix `https://` auf jeder URL ein, oder verwenden Sie das Formular `git@host:path` für SSH. Wenn Sie ein bloßes `gitlab.example.com/your-group/your-marketplace.git` eingeben, liest Claude Code es als GitHub `owner/repo`-Kurzform und lehnt es ab.

Wenn der Befehl erfolgreich ist, druckt er `Successfully added marketplace: <name>`, und die Plugins des Marketplace erscheinen auf der Registerkarte **Discover**, wenn Sie `/plugin` das nächste Mal öffnen, ohne dass ein Neuladen erforderlich ist. Wenn es fehlschlägt, stimmen Sie die Fehlermeldung in [Plugins fehlerbeheben](/docs/de/plugins/troubleshooting#add-a-marketplace) ab.

<h3 id="add-a-marketplace-and-install-in-one-command">
  Marketplace hinzufügen und in einem Befehl installieren
</h3>

Um ein Plugin aus einem Marketplace zu installieren, den Sie noch nicht hinzugefügt haben, führen Sie `/plugin install` in einer Claude Code-Sitzung aus und benennen Sie die Marketplace-Quelle mit `--marketplace`. Erfordert Claude Code v2.1.275 oder später.

```text theme={null}
/plugin install deploy-helper --marketplace your-org/plugins
```

Die Quelle nimmt [die gleichen Formen wie `/plugin marketplace add`](#add-a-marketplace) an, wie GitHub `owner/repo`, eine Git-URL oder einen lokalen Pfad, außer dass sie keine Leerzeichen enthalten kann. Geben Sie den Plugin-Namen allein an, ohne ein `@marketplace`-Suffix.

Wenn Sie diesen Marketplace noch nicht hinzugefügt haben, zeigt Claude Code die aufgelöste Quelle an und fragt Sie, bevor Sie sie hinzufügen. Sobald der Marketplace hinzugefügt ist, öffnen sich die Details des Plugins und Sie wählen einen [Installationsbereich](#install-a-plugin). Wenn die Quelle einem Marketplace entspricht, den Sie bereits hinzugefügt haben, überspringt Claude Code die Bestätigung und öffnet die Details des Plugins in diesem Marketplace.

<h3 id="add-a-private-marketplace">
  Privaten Marketplace hinzufügen
</h3>

Ein privater Marketplace ist einer in einem Repository, auf das Sie Anmeldedaten zum Klonen benötigen, auf GitHub oder einem anderen Git-Host. Sie fügen ihn mit dem gleichen `/plugin marketplace add`- oder `claude plugin marketplace add`-Befehl wie einen öffentlichen hinzu. Claude Code klont ihn mit den Git-Anmeldedaten, die bereits auf Ihrem Computer vorhanden sind, und fragt nie, daher hat jede Verbindungsweise eine Anforderung:

* **HTTPS**: Ihre Git-Credential-Helper gelten, daher funktioniert der Zugriff, den Sie mit `gh auth login`, dem macOS-Schlüsselbund oder `git-credential-store` einrichten. Interaktive Aufforderungen werden unterdrückt, daher schlägt ein Host, bei dem Sie sich noch nie authentifiziert haben, statt zu fragen, fehl.
* **SSH**: Der Host muss bereits in Ihrer `known_hosts`-Datei sein und der Schlüssel muss ohne Passphrase-Aufforderung funktionieren, da die Host-Fingerprint- und Passphrase-Aufforderungen auch unterdrückt werden.
* **GitHub `owner/repo`-Kurzform**: Claude Code prüft, ob Ihr SSH-Schlüssel sich bei `github.com` authentifiziert, klont dann über SSH, wenn ja, und über HTTPS, wenn nein. Setzen Sie [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/de/env-vars#variables), um diese Prüfung zu überspringen und immer über HTTPS zu klonen.

Die gleichen Anmeldedaten gelten, wenn Sie `/plugin install`, `/plugin marketplace update` und `claude plugin update` ausführen.

Auf einem GitHub Enterprise Server-Host siehe [Plugin-Marketplaces auf GHES](/docs/de/github-enterprise-server#plugin-marketplaces-on-ghes) für die Anmeldedaten, die jede Operation benötigt.

Wenn Ihre Organisation den Marketplace für Sie über verwaltete Einstellungen registriert, fügen Sie ihn nicht selbst hinzu. Siehe [Plugins vorinstallieren und erforderlich machen](/docs/de/plugins/org#pre-install-and-require-plugins).

<h3 id="add-from-claude-ai">
  Marketplace von claude.ai hinzufügen
</h3>

In Terminal-Sitzungen, in denen [Plugins von Ihrem claude.ai-Konto synchronisiert werden](/docs/de/plugins/loading#synced-plugins), kann claude.ai auch Plugin-Marketplaces für Sie auflisten, wie die Plugin-Bibliothek Ihrer Organisation und Ihre eigenen claude.ai-Uploads. Sie fügen einen davon nach seinem Namen hinzu, anstatt nach einer Quelle. Das Hinzufügen eines Marketplace von claude.ai erfordert Claude Code v2.1.273 oder später.

Fügen Sie einen claude.ai-Marketplace vom `/plugin`-Panel oder von Ihrer Shell aus hinzu:

* **Inside a session**: Führen Sie `/plugin` aus und gehen Sie zur Registerkarte **Marketplaces**, die die Marketplaces von claude.ai auflistet. Wählen Sie einen dort aus, um ihn hinzuzufügen.
* **From your shell**: Führen Sie `claude plugin marketplace list` aus, das sie in einem Abschnitt `From claude.ai:` druckt. Führen Sie dann `claude plugin marketplace add` mit dem Flag `--claudeai` und dem in der Liste angezeigten Namen aus.

Zum Beispiel fügt dieser Befehl einen Marketplace namens `claudeai-organization-library` hinzu:

```bash theme={null}
claude plugin marketplace add --claudeai claudeai-organization-library
```

Claude Code registriert den Marketplace unter einem lokalen Namen, der mit `claudeai-` beginnt, abgeleitet vom Namen, unter dem claude.ai ihn auflistet. Zum Beispiel wird ein Marketplace, der als "Organization library" aufgelistet ist, zu `claudeai-organization-library`. Installieren Sie seine Plugins unter diesem Namen, zum Beispiel mit `claude plugin install <plugin>@claudeai-organization-library`.

Wenn Sie sich abmelden oder sich bei einer anderen claude.ai-Organisation anmelden, bleibt der Marketplace konfiguriert, zeigt aber keine Plugins an, und die Plugins, die Sie bereits von ihm installiert haben, werden weiterhin geladen.

Der Abschnitt `From claude.ai:` kann auch Git-basierte Marketplaces auflisten, die über claude.ai geteilt werden, und druckt eine Quelle für jeden davon. Fügen Sie sie nach dieser Quelle wie in [Marketplace hinzufügen](#add-a-marketplace) hinzu, nicht mit `--claudeai`.

<h2 id="manage-installed-plugins">
  Installierte Plugins verwalten
</h2>

Die Registerkarte **Installed** in `/plugin` listet Ihre Plugins mit Aktionen auf, um jedes zu aktivieren, zu deaktivieren, zu aktualisieren oder zu deinstallieren. Führen Sie in einer Claude Code-Sitzung `/plugin` aus und drücken Sie **Tab**, um es zu erreichen, oder führen Sie `/plugin enable`, `/plugin disable` oder `/plugin uninstall` aus, um das Panel zu öffnen und diese Änderung dort vorzunehmen. Deaktivierte Plugins werden unter einer eingeklappten Kopfzeile am unteren Ende der Liste gruppiert. Verwenden Sie diese Tasten auf der Liste:

* Geben Sie ein, um nach Name oder Beschreibung zu filtern.
* Drücken Sie **Space**, um das ausgewählte Plugin zu aktivieren oder zu deaktivieren, und **f**, um es zu favorisieren.
* Drücken Sie **Enter**, um die Details eines Plugins zu öffnen. Das Menü dort bietet **Disable plugin** oder **Enable plugin**, **Update now** und **Uninstall**. Plugins, die Einstellungen vornehmen, bieten auch **Configure options**.

Die Registerkarte kann auch Plugins im Bereich **Managed** anzeigen. Ihre Organisation hat diese über [verwaltete Einstellungen](/docs/de/settings#settings-files) installiert, und Sie können sie hier nicht aktivieren, deaktivieren oder deinstallieren.

Für ein synchronisiertes Plugin, das Ihre Organisation auf claude.ai erforderlich macht, siehe [Plugins verwalten, die von claude.ai synchronisiert werden](#manage-plugins-synced-from-claude-ai).

Wenn Sie das `/plugin`-Panel mit ausstehenden Änderungen schließen, die Sie darin vorgenommen haben, führt Claude Code `/reload-plugins` für Sie aus, um sie anzuwenden. Wenn das Neuladen den [Prompt-Cache ungültig machen würde](/docs/de/prompt-caching#enabling-or-disabling-a-plugin), warnt es und lässt die Änderungen stattdessen ausstehend. Führen Sie `/reload-plugins --force` aus, um sie trotzdem anzuwenden.

<h3 id="manage-plugins-synced-from-claude-ai">
  Plugins verwalten, die von claude.ai synchronisiert werden
</h3>

Die Registerkarte **Installed** in `/plugin` listet auch die [Plugins, die von Ihrem claude.ai-Konto synchronisiert werden](/docs/de/plugins/loading#synced-plugins), mit `synced` als ihrer Quelle. Synchronisierte Plugins erscheinen in Terminal-Sitzungen auf Claude Code v2.1.273 oder später.

* **Enable or disable**: Verwenden Sie die Registerkarte **Installed**, es sei denn, Ihre Organisation hat das Plugin als erforderlich markiert.
* **Remove**: Schalten Sie das Plugin auf claude.ai aus.

Wenn Claude Code ein hinzugefügtes, aktualisiertes oder entferntes Plugin in eine interaktive Sitzung synchronisiert, sehen Sie `Plugins changed. Run /reload-plugins to activate.` Führen Sie `/reload-plugins` aus, um die Änderung in dieser Sitzung zu laden, oder lassen Sie sie für das nächste Mal, wenn Sie Claude Code starten.

<h3 id="uninstall-a-plugin-the-project-enables">
  Deinstallieren Sie ein Plugin, das das Projekt aktiviert
</h3>

Wenn Sie **Uninstall** für ein Plugin wählen, das die `.claude/settings.json` dieses Repositorys aktiviert, ob von der Registerkarte **Installed** oder mit `/plugin uninstall`, fragt Claude Code, ob es für Sie deaktiviert oder für alle deinstalliert werden soll:

* **Disable for me**: Drücken Sie **y**. Claude Code schreibt `false` für das Plugin in Ihre `.claude/settings.local.json` und lässt es für das Projekt installiert.
* **Uninstall for everyone**: Drücken Sie **u**. Claude Code entfernt das Plugin aus der gemeinsamen `.claude/settings.json`.

<h3 id="see-what-an-installed-plugin-adds-to-your-sessions">
  Sehen Sie, was ein installiertes Plugin zu Ihren Sitzungen hinzufügt
</h3>

Führen Sie in Ihrer Shell `claude plugin details <name>` für ein installiertes Plugin aus. Die Zeile `Always-on` ist die Anzahl der Token, die das Plugin zu jeder Sitzung hinzufügt, in der es aktiviert ist, und die Pro-Komponenten-Zeilen zeigen, welche Skill oder welcher Agent am meisten beiträgt. Für die vollständige Ausgabe und was jede Zahl bedeutet, siehe [Messen Sie, was ein Plugin kostet](/docs/de/plugins/measure#measure-what-a-plugin-costs).

<h3 id="find-plugins-you-no-longer-use">
  Finden Sie Plugins, die Sie nicht mehr verwenden
</h3>

Auf der Registerkarte **Installed** in `/plugin` erscheinen Plugins, die Sie selbst installiert haben und kürzlich nicht verwendet haben, unter einer Kopfzeile **Not used recently**, und die Details jedes Plugins zeigen eine Zeile **Last used**. Verwenden Sie diese Kopfzeile und diese Zeile, um Plugins zu finden, die immer noch Startup- und Kontextkosten hinzufügen, und deaktivieren oder deinstallieren Sie sie dann.

<h3 id="plugins-with-dependencies">
  Plugins mit Abhängigkeiten
</h3>

Ein Plugin kann andere Plugins deklarieren, von denen es abhängt. Wenn Sie ein solches Plugin aus einem Marketplace installieren, deaktivieren oder deinstallieren, handelt Claude Code auch mit diesen Abhängigkeiten:

* **Install**: Claude Code installiert und aktiviert auch die deklarierten Abhängigkeiten des Plugins im gleichen Bereich. Die Erfolgsmeldung listet sie auf.
* **Enable**: Claude Code aktiviert auch die Abhängigkeiten des Plugins, die installiert, aber deaktiviert sind. Wenn eine deklarierte Abhängigkeit nicht installiert ist, schlägt die Aktivierung fehl und die Nachricht sagt Ihnen, sie zuerst zu installieren.
* **Disable**: Wenn ein anderes aktiviertes Plugin das benötigte noch braucht, weigert sich Claude Code und druckt einen verketteten Befehl, der beide in der richtigen Reihenfolge deaktiviert.
* **Uninstall**: Auto-installierte Abhängigkeiten bleiben, bis Sie `claude plugin prune` in Ihrer Shell ausführen; siehe [plugin prune](/docs/de/plugins/cli-reference#plugin-prune).

Wenn Sie das Plugin stattdessen mit `--plugin-dir` geladen haben, siehe [Testen Sie ein Plugin und seine Abhängigkeit lokal](/docs/de/plugins/dependencies#test-a-plugin-and-its-dependency-locally).

<h3 id="manage-plugins-from-your-shell">
  Plugins von Ihrer Shell verwalten
</h3>

Sie können Plugins auch verwalten, ohne eine Claude Code-Sitzung zu starten. Führen Sie in Ihrer Shell `claude plugin install`, `enable`, `disable` oder `uninstall` als gewöhnliche Terminal-Befehle aus; sie ändern die gleichen Einstellungen, die das `/plugin`-Panel tut. Jeder nimmt `--scope`, um einen Bereich anzuvisieren, und verwendet einen Standardbereich, wenn Sie ihn weglassen:

* `enable` und `disable` handeln mit dem spezifischsten Bereich, dessen Einstellungen das Plugin bereits auflisten.
* `install` und `uninstall` handeln mit dem Benutzerbereich.

Zum Beispiel deaktivieren und reaktivieren diese Befehle ein Plugin, dann deinstallieren es im Projektbereich:

```bash theme={null}
claude plugin disable formatter@your-org
claude plugin enable formatter@your-org
claude plugin uninstall formatter@your-org --scope project
```

<h2 id="keep-plugins-updated">
  Halten Sie Plugins aktualisiert
</h2>

Plugins werden automatisch aktualisiert, wenn der Marketplace, aus dem sie stammen, Auto-Update aktiviert hat. Nachdem eine Sitzung startet, aktualisiert Claude Code diese Marketplaces und aktualisiert die On-Disk-Kopien der Plugins, die Sie von ihnen installiert haben.

Die laufende Sitzung behält die Versionen, die sie bereits geladen hat. Nach einer Aktualisierung sehen Sie `Plugin updated: <name> · Run /reload-plugins to apply`, und die nächste Sitzung lädt die neuen Versionen automatisch.

Dies sind die Auto-Update-Standardeinstellungen für jede Art von Marketplace:

* **On by default**: `claude-plugins-official` und die anderen [offiziellen Marketplace-Namen](/docs/de/plugins/security#official-marketplace-names) außer `knowledge-work-plugins` und `first-party-plugins`, plus [Marketplaces, die von claude.ai hinzugefügt werden](#add-from-claude-ai).
* **Off by default**: jeder andere Marketplace, einschließlich des Community-Marketplace, Drittanbieter-Marketplaces und lokaler Entwicklungs-Marketplaces.

Für wann Auto-Update läuft, welche Plugins es überspringt, und die Umgebungsvariablen, die es ausschalten, siehe [Wenn Auto-Update läuft](/docs/de/plugins/loading#when-auto-update-runs).

<h3 id="turn-auto-update-on-or-off-for-a-marketplace">
  Schalten Sie Auto-Update für einen Marketplace ein oder aus
</h3>

Führen Sie in einer Claude Code-Sitzung `/plugin` aus und gehen Sie zur Registerkarte **Marketplaces**. Wählen Sie den Marketplace aus, dann wählen Sie **Enable auto-update** oder **Disable auto-update**.

<h3 id="update-one-plugin-now">
  Aktualisieren Sie ein Plugin jetzt
</h3>

Öffnen Sie in einer Sitzung das Plugin auf der Registerkarte **Installed** in `/plugin` und wählen Sie **Update now**, oder führen Sie in Ihrer Shell `claude plugin update <plugin>@<marketplace>` aus.

<h3 id="auto-update-from-a-private-marketplace">
  Auto-Update von einem privaten Marketplace
</h3>

Für einen privaten Marketplace siehe [Was Background-Auto-Update mit Anmeldedaten tut](/docs/de/plugins/host-marketplace#what-background-auto-update-does-with-credentials) für wie Background-Auto-Updates sich über SSH und HTTPS authentifizieren, und [Plugins fehlerbeheben](/docs/de/plugins/troubleshooting#add-a-marketplace) für die Nachrichten, die Sie sehen, wenn sie fehlschlagen.

<h2 id="manage-marketplaces">
  Marketplaces verwalten
</h2>

Die Registerkarte **Marketplaces** in `/plugin` listet jeden Marketplace auf, den Sie registriert haben, zusammen mit seiner Quelle. Wählen Sie einen aus, um seine Plugins zu durchsuchen, seine Auflistung zu aktualisieren, Auto-Update ein- oder auszuschalten, oder ihn zu entfernen.

Sie können Marketplaces auch mit Befehlen auflisten, aktualisieren und entfernen, von Ihrer Shell oder innerhalb einer Sitzung:

| Aktion                                     | In Ihrer Shell                            | Innerhalb einer Sitzung             |
| :----------------------------------------- | :---------------------------------------- | :---------------------------------- |
| Marketplaces auflisten                     | `claude plugin marketplace list`          | `/plugin marketplace list`          |
| Auflistung eines Marketplace aktualisieren | `claude plugin marketplace update <name>` | `/plugin marketplace update <name>` |
| Marketplace entfernen                      | `claude plugin marketplace remove <name>` | `/plugin marketplace remove <name>` |

Wenn Sie einen Marketplace entfernen, deinstalliert Claude Code jedes Plugin, das Sie von ihm installiert haben, und entfernt ihre `enabledPlugins`-Einträge aus Ihren Einstellungsdateien. Die Registerkarte **Marketplaces** benennt diese Plugins, bevor sie Sie zur Bestätigung auffordert.

<h2 id="next-steps">
  Nächste Schritte
</h2>

* [Anthropics Marketplaces](/docs/de/plugins/anthropic-marketplaces): wie sich der offizielle, Community- und Demo-Marketplace unterscheiden und wo Sie jeden durchsuchen können
* [Plugin-Lade-Referenz](/docs/de/plugins/loading): warum ein Plugin geladen wurde, nicht geladen wurde, oder sich nach einer Aktualisierung nicht änderte
* [Plugin-Sicherheit und Vertrauen](/docs/de/plugins/security): was Sie überprüfen sollten, bevor Sie ein Plugin aus einem Marketplace installieren, den Sie nicht kennen
* [Plugins fehlerbeheben](/docs/de/plugins/troubleshooting): Installations- und Marketplace-Fehlermeldungen mit ihren Fixes
* [Erstellen Sie ein Plugin](/docs/de/plugins/create): bauen Sie Ihr eigenes
