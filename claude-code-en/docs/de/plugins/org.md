> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Verwalten Sie Claude Code-Plugins für Ihre Organisation

> Kontrollieren Sie, welche Plugins Claude Code auf jedem Computer in Ihrer Organisation installiert und zulässt, durch verwaltete Einstellungen.

Mit verwalteten Einstellungen können Sie entscheiden, welche Plugins Claude Code auf jedem Computer in Ihrer Organisation installiert und zulässt. Benutzer können diese nicht überschreiben. Sie stellen sie entweder als [serverseitig verwaltete Einstellungen](/docs/de/server-managed-settings) aus der claude.ai-Administratorkonsole oder als endpunktseitig verwaltete Einstellungen über MDM oder eine `managed-settings.json`-Datei bereit. Die meisten Steuerelemente auf dieser Seite wirken sich nur auf verwaltete Einstellungen aus.

Diese Seite ist für Administratoren bestimmt, und die Einstellungen hier regeln Claude Code.

<Note>
  Diese Fälle werden auf anderen Seiten behandelt:

  * **Installieren von Plugins für sich selbst**: Beginnen Sie bei [Plugins installieren](/docs/de/plugins/install)
  * **Kontrollieren, welche Plugins Mitglieder in claude.ai und Cowork verwenden können**: siehe [Verwalten Sie Plugins für Ihre Organisation](https://support.claude.com/en/articles/13837433) im Hilfecenter
  * **Die Plugins-Seite in den Administratoreinstellungen von claude.ai**: [**Organisationseinstellungen > Plugins & Skills**](https://claude.ai/admin-settings/skills?tab=inventory) aktiviert Plugins für die claude.ai-Konten der Mitglieder, und diese erreichen Claude Code als [synchronisierte Plugins](/docs/de/plugins/loading#synced-plugins). Es werden keine der Schlüssel auf dieser Seite gesetzt
</Note>

Die Abschnitte folgen der Reihenfolge, die die meisten Rollouts durchlaufen: [Plugins erforderlich machen](#pre-install-and-require-plugins) für alle oder pro Repository, [Container und CI seeding](#seed-containers-and-ci), [einschränken](#restrict-what-users-can-install), was Benutzer selbst hinzufügen können, [Aktualisierungsrichtlinie festlegen](#set-update-policy), dann [überprüfen](#audit-and-review), was installiert ist. Um jede Richtlinienschlüssel an einem Ort zu überprüfen, siehe die [Kontrollmatrix](#control-matrix).

<h2 id="pre-install-and-require-plugins">
  Plugins vorinstallieren und erforderlich machen
</h2>

Ein Marketplace ist ein Katalog von Plugins, die Claude Code aus einem Git-Repository, einer URL oder einem lokalen Pfad abruft. Sobald Sie einen Marketplace auf einem Computer registrieren, kann Claude Code Plugins daraus installieren.

Um Plugins für eine Flotte zu installieren, setzen Sie zwei Schlüssel zusammen in [verwalteten Einstellungen](/docs/de/managed-settings), die Richtliniendatei oder die serverseitig bereitgestellte Richtlinie, die jeder Computer in Ihrer Organisation liest: `extraKnownMarketplaces` registriert einen Marketplace auf jedem Computer, und `enabledPlugins` benennt die zu installierenden und aktivierenden Plugins. [Wählen Sie einen Bereitstellungsmechanismus](#choose-a-delivery-mechanism) behandelt, wie verwaltete Einstellungen jeden Computer erreichen.

<h3 id="choose-a-delivery-mechanism">
  Wählen Sie einen Bereitstellungsmechanismus
</h3>

Verwaltete Einstellungen erreichen einen Computer über einen von drei Bereitstellungsmechanismen:

* **Serverseitig verwaltete Einstellungen**: Setzen Sie die Plugin-Schlüssel als JSON unter [**Organisationseinstellungen > Claude Code > Verwaltete Einstellungen**](https://claude.ai/admin-settings/claude-code). Erfordert eine [Eigentümerrolle](/docs/de/server-managed-settings#access-control) in Ihrer Claude-Organisation. Eine Cloud-Sitzung ruft diese Einstellungen ab, bevor sie Plugins installiert.
* **MDM-Richtlinien**: Unter macOS stellen Sie eine plist bereit, deren oberste Schlüssel die Einstellungsschlüssel sind. Unter Windows speichern Sie das gesamte JSON-Dokument als Zeichenkette in einem Registrierungswert. Die plist-Domäne und der Registrierungsschlüssel befinden sich unter [Wo jeder Mechanismus die Richtlinie speichert](/docs/de/managed-settings#where-each-mechanism-stores-the-policy).
* **Verwaltete Einstellungsdatei**: Platzieren Sie eine `managed-settings.json` im Systempfad der Plattform. Sie können auch Dateien zum `managed-settings.d/`-Drop-in-Verzeichnis daneben hinzufügen. Die Dateipfade pro Plattform befinden sich unter [Wo jeder Mechanismus die Richtlinie speichert](/docs/de/managed-settings#where-each-mechanism-stores-the-policy), und die Drop-in-Zusammenführungsregeln befinden sich unter [Teilen Sie eine dateibasierte Richtlinie auf Teams auf](/docs/de/managed-settings#split-a-file-based-policy-across-teams).

Verwenden Sie serverseitig verwaltete Einstellungen, wenn Sie eine Claude for Teams- oder Enterprise-Organisation auf claude.ai haben und Ihre Geräte nicht alle unter MDM stehen. Verwenden Sie andernfalls eine MDM-Richtlinie oder die verwaltete Einstellungsdatei. Für den Kompromiss siehe [Wählen Sie zwischen serverseitig verwalteten und endpunktseitig verwalteten Einstellungen](/docs/de/server-managed-settings#choose-between-server-managed-and-endpoint-managed-settings).

<h4 id="which-managed-source-applies-on-a-machine">
  Welche verwaltete Quelle gilt auf einem Computer
</h4>

Standardmäßig gilt nur eine dieser drei Quellen auf einem Computer. Claude Code verwendet die erste, die einen Richtlinienschlüssel bereitstellt, und prüft zuerst serverseitig verwaltete Einstellungen, dann MDM-Richtlinien, dann die verwaltete Einstellungsdatei. Wenn serverseitig verwaltete Einstellungen auch nur einen unabhängigen Richtlinienschlüssel bereitstellen, ignoriert Claude Code die Plugin-Schlüssel in einer MDM-Richtlinie oder verwalteten Einstellungsdatei auf diesem Computer, mit Ausnahme der [Schlüssel, die es aus jeder Quelle liest](/docs/de/managed-settings#keys-read-from-every-admin-source).

Um stattdessen jede Quelle anzuwenden, setzen Sie [`managedSourcesBehavior`](/docs/de/managed-settings#compose-every-managed-source) auf `"merge"`.

[Wie Claude Code verwaltete Quellen kombiniert](/docs/de/managed-settings#how-claude-code-combines-managed-sources) listet auch die Schlüssel auf, die Claude Code in beiden Modi aus jeder Quelle liest.

<h3 id="require-a-marketplace-and-its-plugins">
  Erforderlich machen eines Marketplace und seiner Plugins
</h3>

Fügen Sie den Marketplace unter `extraKnownMarketplaces` hinzu, gekennzeichnet durch den eigenen `name` des Marketplace aus seiner `marketplace.json`. Fügen Sie dann jedes Plugin unter `enabledPlugins` als `plugin-name@marketplace-name` hinzu. Jeder Marketplace-Eintrag enthält ein `source`-Objekt mit einem `source`-Feld, das den Typ benennt, z. B. `github`. Dieses verwaltete Einstellungsbeispiel registriert einen Organisations-Marketplace und erzwingt zwei Plugins daraus:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": { "source": "github", "repo": "your-org/your-marketplace" },
      "autoUpdate": true
    }
  },
  "enabledPlugins": {
    "code-formatter@your-marketplace": true,
    "deploy-helper@your-marketplace": true
  }
}
```

Nachdem die Einstellungen einen Computer erreichen, registriert Claude Code den Marketplace und installiert die beiden Plugins zu Beginn der nächsten Sitzung des Benutzers. Benutzer sehen sie in `/plugin`, und das Deaktivieren eines Plugins in ihrem eigenen Bereich stoppt nicht das Laden, da verwaltete Einstellungen Vorrang vor jedem anderen Bereich haben.

Um ein Plugin in jedem Bereich zu blockieren und es aus der Marketplace-Auflistung auszublenden, setzen Sie es stattdessen auf `false` in den verwalteten `enabledPlugins`.

Passen Sie die Felder `autoUpdate` und `source` für Ihren Marketplace an:

* **`autoUpdate`**: `true` hält den Marketplace und seine Plugins im Hintergrund aktualisiert, und `false` schaltet das aus. Siehe [Aktualisierungsrichtlinie festlegen](#set-update-policy).
* **`source`**: `github` ist einer von mehreren Quellentypen. Eine `git`-Quelle benötigt eine `url` für GitLab oder einen internen Host, und eine `url`-Quelle benötigt die Adresse einer gehosteten `marketplace.json`. Jede Quellenform befindet sich in der [Marketplace-Referenz](/docs/de/plugins/marketplace-reference).

Wenn der Marketplace ein privates Git-Repository ist, benötigt jeder Benutzer Lesezugriff darauf. Das Klonen eines Git-basierten Marketplace wird mit Git auf dem Computer des Benutzers ausgeführt, wobei gespeicherte Anmeldedaten verwendet werden und keine Eingabeaufforderungen angezeigt werden. Für Benutzer ohne Git-Host-Konten verwenden Sie stattdessen einen [Seed](#seed-containers-and-ci).

Ein verwalteter Eintrag überschreibt auch einen gleichnamigen Marketplace-Eintrag oder eine `--plugin-dir`-Kopie aus einer anderen Quelle:

* **Marketplaces**: Ein verwalteter Marketplace-Eintrag ersetzt einen Eintrag mit niedrigerer Priorität mit demselben Namen, und die Felder der beiden Einträge werden nicht zusammengeführt.
* **`--plugin-dir`-Kopien**: `--plugin-dir` lädt ein Plugin aus einem lokalen Verzeichnis für eine Sitzung. Für das, was passiert, wenn der Name dieser Kopie mit einem Plugin übereinstimmt, das Ihre verwalteten `enabledPlugins` benennen, siehe [Namenskonflikte](/docs/de/plugins/loading#name-conflicts).

Der offizielle Marketplace von Anthropic `claude-plugins-official` benötigt keinen `extraKnownMarketplaces`-Eintrag, wenn `enabledPlugins` eines seiner Plugins auf `true` setzt. Dieser `name@claude-plugins-official`-Eintrag deklariert den Marketplace selbst, überall wo diese Schlüssel gelten. Wenn Sie keines seiner Plugins aktivieren und es trotzdem auf jedem Computer registriert haben möchten, geben Sie ihm einen expliziten Eintrag, wie [Erlauben Sie den offiziellen Marketplace und Ihren eigenen](#allow-the-official-marketplace-and-your-own) zeigt.

<h3 id="require-plugins-per-repository">
  Erforderlich machen von Plugins pro Repository
</h3>

Um die Mitwirkenden eines Repositories statt Ihrer ganzen Flotte abzudecken, setzen Sie `extraKnownMarketplaces` und `enabledPlugins` in der `.claude/settings.json` dieses Repositories. Die `extraKnownMarketplaces`-Einträge gelten nur in einem Ordner, den der Mitwirkende vertraut hat, und in einem nicht vertrauenswürdigen Ordner ignoriert Claude Code sie ohne Meldung:

* **Interaktive Sitzungen**: Claude Code registriert den Marketplace nur, nachdem der Mitwirkende den [Workspace-Vertrauensdialog](/docs/de/permissions#what-runs-before-you-trust-a-folder) für diesen Ordner akzeptiert hat.
* **[Nicht-interaktive `-p`-Läufe](/docs/de/headless)**: Die Einträge gelten nur in einem Ordner, dessen Vertrauen der Benutzer bereits interaktiv akzeptiert hat, oder dessen `hasTrustDialogAccepted`-Flag Sie in `~/.claude.json` setzen.

Ein Plugin, das der Marketplace durch einen relativen Pfad auflistet, wird aus der Marketplace-Kopie geladen, sobald die `extraKnownMarketplaces`-Einträge des Repositories gelten. Ein Plugin, dessen Marketplace-Eintrag stattdessen auf eine externe Quelle verweist, z. B. das eigene GitHub-Repository des Plugins, wird nicht allein aus den Repository-Einstellungen installiert. Jeder Mitwirkende sieht `Plugin "<name>" is enabled in project settings but isn't installed`, bis er `claude plugin install <name>@<marketplace> --scope project` ausführt, wie [Plugins installieren](/docs/de/plugins/install) beschreibt.

Wenn Sie eine lokale `directory`- oder `file`-Quelle mit einem relativen Pfad verwenden, wird der Pfad gegen den Haupt-Checkout Ihres Repositories aufgelöst. Wenn Sie Claude Code aus einem Git-Worktree ausführen, verweist der Pfad immer noch auf den Haupt-Checkout, sodass alle Worktrees denselben Marketplace-Speicherort gemeinsam nutzen.

Um ein Bundle von Plugins mit Abhängigkeiten bereitzustellen, setzen Sie das Bundle-Plugin in `enabledPlugins`, wie [Plugin-Abhängigkeiten](/docs/de/plugins/dependencies) beschreibt.

<h3 id="when-each-surface-applies-the-plugin-keys">
  Wenn jede Oberfläche die Plugin-Schlüssel anwendet
</h3>

Die Tabelle zeigt, wann jede Art von Claude Code-Sitzung `extraKnownMarketplaces` und `enabledPlugins` anwendet, aus verwalteten Einstellungen und aus der `.claude/settings.json` eines Repositories. Für die Desktop-App und die IDE-Erweiterungen siehe [Ein Plugin installieren](/docs/de/plugins/install#install-a-plugin).

| Oberfläche           | Verwaltete `extraKnownMarketplaces` und `enabledPlugins`                                                                                                                                                                                                                                                                                                                                   | Repository `.claude/settings.json`                                                                              |
| :------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- |
| Terminal, interaktiv | Angewendet beim Sitzungsstart auf jedem Computer, der die Einstellungen erhält                                                                                                                                                                                                                                                                                                             | `extraKnownMarketplaces` angewendet nach Vertrauen; `enabledPlugins` angewendet beim Sitzungsstart              |
| `-p` und CI          | Angewendet beim Sitzungsstart, mit Installationen im Hintergrund                                                                                                                                                                                                                                                                                                                           | `extraKnownMarketplaces` nur in vertrauenswürdigen Ordnern; `enabledPlugins` angewendet                         |
| Cloud-Sitzungen      | In einer von Anthropic gehosteten Umgebung erreichen nur serverseitig verwaltete Einstellungen die Sitzung, die auf sie wartet, bevor sie Plugins installiert. MDM-Richtlinien und verwaltete Einstellungsdateien bleiben auf dem Computer des Benutzers. Für eine selbstgehostete Umgebung siehe [Wo und wann eine Richtlinie gilt](/docs/de/managed-settings#where-and-when-a-policy-applies) | Siehe die **Cloud-Sitzung**-Registerkarte unter [Ein Plugin installieren](/docs/de/plugins/install#install-a-plugin) |

In einem `-p`- oder CI-Lauf werden Marketplaces und Plugins im Hintergrund installiert, sodass ein Plugin in der ersten Runde fehlen kann. Setzen Sie `CLAUDE_CODE_SYNC_PLUGIN_INSTALL=1`, um den Lauf warten zu lassen, bis die Installation vor seiner ersten Abfrage abgeschlossen ist.

<h3 id="confirm-the-rollout">
  Bestätigen Sie den Rollout
</h3>

Überprüfen Sie, dass der Marketplace und die Plugins auf einem Computer oder in einem CI-Lauf angekommen sind:

* **Auf einem Computer**: Starten Sie Claude Code und führen Sie `/plugin` aus. Der Marketplace und die Plugins sind aufgelistet.
* **In CI**: Führen Sie `claude -p` mit `--output-format stream-json --verbose` aus. Das `init`-Ereignis listet die geladenen Plugins unter `plugins` auf.

<h2 id="seed-containers-and-ci">
  Container und CI seeding
</h2>

Für Container-Images und CI-Runner, die zur Laufzeit nicht klonen können, füllen Sie ein Plugin-Verzeichnis zur Build-Zeit vor und verweisen Sie `CLAUDE_CODE_PLUGIN_SEED_DIR` darauf. Claude Code registriert die Marketplaces des Seeds beim Start und lädt Plugin-Caches aus dem Seed an Ort und Stelle, ohne zu klonen.

Ein Seed dient auch Benutzern, die kein Git-Host-Konto haben.

<Note>
  In CI/CD-Umgebungen konfigurieren Sie einen Git-Anmeldedaten-Helper, bevor Sie Plugins aus privaten Repositories installieren. Auf GitHub Actions exportieren Sie ein Token mit Lesezugriff auf das Marketplace-Repository als `GH_TOKEN`, führen dann `gh auth setup-git` aus. Das Standard-Workflow-Token kann nur auf das eigene Repository des Workflows zugreifen, daher benötigt ein privater Marketplace in einem anderen Repository ein persönliches Zugriffstoken oder App-Token.
</Note>

<Steps>
  <Step title="Installieren Sie in den Seed zur Build-Zeit">
    Setzen Sie `CLAUDE_CODE_PLUGIN_CACHE_DIR` auf den Seed-Pfad, damit der Marketplace und die Plugins dort statt in `~/.claude/plugins` installiert werden:

    ```bash theme={null}
    CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin marketplace add your-org/your-marketplace
    CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin install code-formatter@your-marketplace
    ```

    Der Seed hat das gleiche Layout wie `~/.claude/plugins`: `known_marketplaces.json`, `marketplaces/<name>/` und `cache/<marketplace>/<plugin>/<version>/`. Sie können den Seed an einem anderen Pfad als dem, an dem Sie ihn erstellt haben, bereitstellen.
  </Step>

  <Step title="Verweisen Sie die Laufzeit auf den Seed">
    Setzen Sie `CLAUDE_CODE_PLUGIN_SEED_DIR=/opt/claude-seed` in der Umgebung des Containers. Um mehrere Seeds zu verwenden, trennen Sie ihre Pfade mit `:` auf Unix oder `;` unter Windows. Claude Code verwendet den ersten Seed, der einen bestimmten Marketplace oder Plugin-Cache enthält.
  </Step>

  <Step title="Aktivieren Sie die Plugins">
    Die Plugins in einem Seed sind nicht von selbst aktiviert. Setzen Sie `enabledPlugins` für jedes Seed-Plugin, das Sie geladen haben möchten, in verwalteten Einstellungen oder in der `.claude/settings.json` des Repositories.
  </Step>
</Steps>

Um einen Seed zu überprüfen, führen Sie `claude -p` mit `--output-format stream-json --verbose` im Image aus. In der `plugins`-Liste des `init`-Ereignisses befindet sich der `path` jedes geladenen Plugins unter dem Seed, z. B. `/opt/claude-seed/cache/your-marketplace/code-formatter/1.0.0`.

Seed-Marketplaces folgen diesen Regeln:

* **Schreibgeschützt**: Claude Code schreibt niemals in den Seed und erzwingt `autoUpdate` aus für Seed-Marketplaces.
* **Seed-Einträge haben Vorrang**: Beim Start überschreibt ein im Seed deklarierter Marketplace den Eintrag des Benutzers mit demselben Namen. Benutzer deaktivieren ein Seed-Plugin mit `claude plugin disable`, nicht durch Entfernen des Marketplace.
* **Aktualisierung und Entfernung schlagen fehl**: `claude plugin marketplace update <name>` und `remove` ohne `--scope` auf einem Seed-Marketplace schlagen mit einer Meldung fehl, die das Seed-Verzeichnis benennt.
* **Richtlinie gilt immer noch**: Die [Zulassungsliste und Blockliste](#restrict-what-users-can-install) überprüfen auch die aufgezeichnete Quelle eines Seed-Marketplace. Erlauben Sie die Quelle, aus der Sie den Seed erstellt haben.

Für Flotten ohne ausgehenden Git-Zugriff kombinieren Sie einen Seed mit `directory`- oder `file`-Marketplace-Quellen auf einer gemeinsamen Bereitstellung. Setzen Sie auch `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1`, was auch [Plugin-Auto-Update](/docs/de/plugins/loading#when-auto-update-runs) ausschaltet. Wenn ein Proxy verfügbar ist, siehe [Proxy-Konfiguration](/docs/de/network-config#proxy-configuration) für die zu setzenden Variablen.

<h2 id="restrict-what-users-can-install">
  Einschränken, was Benutzer installieren können
</h2>

Die verwaltete `strictKnownMarketplaces`-Zulassungsliste und die `blockedMarketplaces`-Blockliste bestimmen, aus welchen Marketplace-Quellen Plugins stammen dürfen. Die Quelle eines Marketplace ist das Git-Repository, die URL oder der lokale Pfad, von dem Claude Code es abruft. Beide Listen entsprechen der Quelle des Marketplace, aus dem ein Plugin stammt, nicht dem eigenen Eintrag des Plugins in diesem Marketplace.

Für die häufige Sperrung, die den offiziellen Marketplace und Ihren eigenen zulässt, siehe [Den offiziellen Marketplace und Ihren eigenen zulassen](#allow-the-official-marketplace-and-your-own). Kombinieren Sie dies mit [`disableSideloadFlags`](#control-matrix), damit Benutzer Plugins nicht aus einem lokalen Verzeichnis oder einer URL laden können.

Beide Listen gelten vor dem Download und erneut beim Sitzungsstart:

* **Vor einem Download**: Die Listen gelten, wenn ein Benutzer einen Marketplace hinzufügt und bei jedem Install, Update, Refresh und Auto-Update.
* **Beim Sitzungsstart**: Die Listen gelten erneut für bereits installierte Plugins, sodass ein installiertes Plugin, dessen Marketplace-Quelle nicht mehr übereinstimmt, nicht geladen wird. `/plugin` listet es mit `Marketplace "<name>" is not in the allowed marketplace list` oder `Marketplace "<name>" is blocked by enterprise policy` auf.

Wo die beiden Listen durchgesetzt werden, hängt davon ab, wo Sie sie festlegen:

* **Die claude.ai-Admin-Konsole**: Claude Code erzwingt beide Listen in den Sitzungen, die [server-verwaltete Einstellungen lesen](/docs/de/managed-settings#where-and-when-a-policy-applies). claude.ai überprüft sie auch, wenn jemand in Ihrer Organisation einen neuen Marketplace aus einem Git-Repository auf claude.ai oder aus **Anpassen** in der Claude Desktop-App außerhalb der Code-Registerkarte hinzufügt. Dies umfasst einen Marketplace, den ein Mitglied für sein eigenes Konto hinzufügt, und einen, der für die gesamte Organisation unter [**Organisationseinstellungen > Plugins**](https://claude.ai/admin-settings/plugins) hinzugefügt wird. claude.ai lehnt ein Repository ab, das die Zulassungsliste nicht zulässt oder das die Blockliste nennt. Es überprüft einen Marketplace nicht erneut, der an beiden Stellen hinzugefügt wurde, bevor Sie die Listen festlegten, und es überprüft hochgeladene Plugins nicht.
* **Eine verwaltete Einstellungsdatei, eine Richtlinie auf Betriebssystemebene oder eine andere verwaltete Quelle**: Claude Code erzwingt beide Listen, wo es diese Quelle liest. claude.ai liest sie nicht.

Während eine Zulassungsliste festgelegt ist oder eine Blockliste eine andere Quelle als [`skills-dir`](#blocklist-with-blockedmarketplaces) nennt, wird ein Plugin, dessen Marketplace Claude Code nicht finden kann, nicht geladen. `/plugin` zeigt den Richtlinienfehler dafür an, anstatt einen Fehler „nicht gefunden" anzuzeigen. Der häufige Fall ist ein veralteter `enabledPlugins`-Eintrag für einen Marketplace, den niemand registriert hat.

<h3 id="control-matrix">
  Kontrollmatrix
</h3>

Die Tabelle listet jeden Plugin-Richtlinienschlüssel auf, was er erzwingt und was er nicht kann.

| Schlüssel                                                                | Was er erzwingt                                                                                                                                                                                                                                                                                                     | Was er nicht kann                                                                                                                                                                                                                                                                  |
| :----------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `strictKnownMarketplaces`                                                | Zulassungsliste von Marketplace-Quellen. `[]` blockiert jede Quelle, einschließlich des offiziellen Marketplace. Alias: `allowedMarketplaces`                                                                                                                                                                       | Registriert keinen Marketplace, schränkt Einträge in einem zulassenen Marketplace nicht ein oder blockiert `--plugin-dir` nicht                                                                                                                                                    |
| `blockedMarketplaces`                                                    | Blockliste von Marketplace-Quellen, überprüft vor der Zulassungsliste                                                                                                                                                                                                                                               | Blockiert keinen Marketplace, der bereits aus einer Quelle registriert ist, die nicht übereinstimmt                                                                                                                                                                                |
| `syncClaudeAiPlugins`                                                    | Setzen Sie auf `false`, um zu verhindern, dass Claude Code die Plugins herunterlädt und lädt, die [von claude.ai synchronisiert werden](/docs/de/plugins/loading#synced-plugins) für das Konto jedes Benutzers. Erfordert Claude Code v2.1.273 oder später                                                               | Schaltet kein synchronisiertes Plugin aus. Dafür setzen Sie `"<name>@synced": false` in [`enabledPlugins`](/docs/de/settings-reference#enabledplugins)                                                                                                                                  |
| `enabledPlugins`                                                         | `true` erzwingt das Aktivieren, `false` blockiert in jedem Bereich und verbirgt das Plugin                                                                                                                                                                                                                          | Installiert kein Plugin, dessen Marketplace nicht registriert oder zulässig ist                                                                                                                                                                                                    |
| `disableSideloadFlags`                                                   | Lehnt `--plugin-dir`, `--plugin-url`, `--agents`, die Agent SDK `plugins`-Option und nicht-SDK `--mcp-config` beim Start ab und lehnt Ordner ab, die in der [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/de/env-vars#variables)-Variable benannt sind, auf die gleiche Weise                                                        | Schränkt `.mcp.json`, `claude mcp add` oder SDK-bereitgestellte Server nicht ein. Kombinieren Sie mit [`allowedMcpServers`](/docs/de/managed-mcp)                                                                                                                                       |
| `disableCommandPluginSources`                                            | Blockiert Plugins mit einer `command`-Quelle vor Installation, Update oder Laden. Eine `command`-Quelle ist eine, deren Plugin-Verzeichnis durch Ausführung eines Befehls auf dem Computer erzeugt wird. Wenn nicht gesetzt, nimmt es den Wert von `allowManagedHooksOnly` an                                       | Beeinflusst andere Quellentypen nicht                                                                                                                                                                                                                                              |
| `allowManagedHooksOnly`                                                  | Schränkt ein, welche Hooks ausgeführt werden. Siehe [`allowManagedHooksOnly`](/docs/de/settings-reference#allowmanagedhooksonly)                                                                                                                                                                                         | Vertraut Hooks von Plugins nicht, die Benutzer selbst aktivieren                                                                                                                                                                                                                   |
| `strictPluginOnlyCustomization`                                          | Blockiert Skills, Agents, Hooks und MCP-Server, die nicht von einem Plugin, verwalteten Einstellungen oder Claude Code-Built-ins stammen. Setzen Sie auf `true`, um alle vier Typen abzudecken, oder auf ein Array von `skills`, `agents`, `hooks` und `mcp`-Werten wie `["skills", "hooks"]`, um einige abzudecken | Schränkt nicht ein, welche Plugins Benutzer installieren. Kombinieren Sie mit `strictKnownMarketplaces`                                                                                                                                                                            |
| `pluginSuggestionMarketplaces`                                           | Marketplaces, deren Plugins als Installationsvorschläge angezeigt werden können. Siehe [Plugins empfehlen](#recommend-plugins)                                                                                                                                                                                      | Beeinflusst die integrierten Tipps nicht                                                                                                                                                                                                                                           |
| `pluginTrustMessage`                                                     | Fügt Ihren Text zur Vertrauenswarnung an, die `/plugin` vor der Installation eines Plugins anzeigt                                                                                                                                                                                                                  | Ändert den Text der Warnung selbst nicht                                                                                                                                                                                                                                           |
| `allowedChannelPlugins`                                                  | Ersetzt die Standardliste von Plugins, die Kanalnachrichten pushen dürfen. Erfordert `channelsEnabled: true`                                                                                                                                                                                                        | Siehe [Einschränken, welche Channel-Plugins ausgeführt werden können](/docs/de/channels#restrict-which-channel-plugins-can-run)                                                                                                                                                         |
| [`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL=1`](/docs/de/env-vars) | Stoppt interaktive Terminal-Sitzungen von der automatischen Registrierung des offiziellen Marketplace                                                                                                                                                                                                               | Entfernt keinen bereits registrierten Marketplace. Die Zulassungsliste und Blockliste geben die gleiche automatische Registrierung ohne sie ein. Ein Computer, der einmal damit gestartet wurde, setzt die automatische Registrierung nicht fort, nachdem Sie es deaktiviert haben |

Jeder Schlüssel in der Tabelle ist eine verwaltete Einstellung, außer `enabledPlugins`, `syncClaudeAiPlugins` und `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL`:

* **`enabledPlugins`**: Sie können es in jedem Bereich festlegen, und verwaltete Einstellungen sperren es.
* **`syncClaudeAiPlugins`**: Jeder Benutzer kann es auch in seinen eigenen Benutzer- oder lokalen Einstellungen festlegen. Siehe seinen [Bereich in der Einstellungsreferenz](/docs/de/settings-reference#syncclaudeaiplugins).
* **`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL`**: Dies ist eine Umgebungsvariable, die Sie über den verwalteten `env`-Block bereitstellen, der unter [Updates für die gesamte Flotte ausschalten](#turn-updates-off-for-the-whole-fleet) angezeigt wird.

Jeder Einstellungsschlüssel hier hat einen Eintrag in der [Einstellungsreferenz](/docs/de/settings-reference).

<h4 id="aliases-for-the-marketplace-keys">
  Aliase für die Marketplace-Schlüssel
</h4>

`strictKnownMarketplaces` kann auch als `allowedMarketplaces` geschrieben werden, und `extraKnownMarketplaces` kann auch als `additionalMarketplaces` geschrieben werden.

* **Version**: Die Aliase erfordern Claude Code v2.1.232 oder später, und ältere Clients ignorieren sie. Behalten Sie in einer Datei, die eine gemischte Flotte liest, die kanonischen Namen.
* **Beide Schreibweisen gesetzt**: Wenn eine Datei beide Schreibweisen setzt, gilt der Wert des kanonischen Schlüssels.

<h3 id="allowlist-with-strictknownmarketplaces">
  Zulassungsliste mit `strictKnownMarketplaces`
</h3>

Setzen Sie die Zulassungsliste auf eine Liste dieser Quellobjekte. Die meisten Einträge stimmen genau überein, `hostPattern`- und `pathPattern`-Einträge stimmen als reguläre Ausdrücke überein, und `github`-Besitzer-Wildcards stimmen nach Besitzer überein:

* **`github`**: `{ "source": "github", "repo": "your-org/approved-plugins" }`, mit optionalem `ref` und `path`.
* **`github`-Besitzer-Wildcard**: `{ "source": "github", "repo": "your-org/*" }` stimmt mit jedem Repository unter diesem Besitzer überein. Das `*` muss für den gesamten Repository-Namen stehen. Claude Code ignoriert Einträge wie `*/plugins` und `your-org/tools-*` als ungültig, sodass sie nichts abgleichen. Erfordert Claude Code v2.1.223 oder später.
* **`git`**: `{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git" }`, mit optionalem `ref` und `path`.
* **`url`**: `{ "source": "url", "url": "https://plugins.example.com/marketplace.json" }`, mit optionalen `headers`.
* **`file` und `directory`**: `{ "source": "file", "path": "/opt/marketplace/marketplace.json" }` oder `{ "source": "directory", "path": "/opt/marketplace/plugins" }`, mit absoluten Pfaden.
* **`hostPattern`**: `{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }`, abgeglichen gegen den Host von `github`, `git` und `url`-Quellen. Das Muster stimmt überall im Hostnamen überein, daher verankern Sie es mit `^` und `$` wie gezeigt, um den gesamten Host abzugleichen. Eine `github`-Quelle zählt immer als `github.com`. Verwenden Sie einen `hostPattern`-Eintrag für einen GitHub Enterprise Server oder GitLab-Host, auf dem Entwickler ihre eigenen Marketplaces erstellen. Die [GHES-Seite](/docs/de/github-enterprise-server#allowlist-ghes-marketplaces-in-managed-settings) hat das durchgearbeitete Beispiel.
* **`pathPattern`**: `{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }`, abgeglichen gegen den `path` von `file`- und `directory`-Quellen. Das Muster stimmt überall im Pfad überein, daher beginnen Sie es mit `^`, um ein Verzeichnispräfix zu fixieren. `".*"` erlaubt jeden lokalen Pfad.
* **`skills-dir`**: `{ "source": "skills-dir" }` hält [Skills-Verzeichnis-Plugins](#keep-skills-directory-plugins-loading) beim Laden unter einer Zulassungsliste und stimmt mit keinem Marketplace überein.

<h4 id="how-entries-match">
  Wie Einträge abgleichen
</h4>

Ein `url`-Eintrag stimmt mit seinem `url`-Wert überein; `headers` werden nicht verglichen. Für `github`- und `git`-Einträge müssen `repo` oder `url`, `ref` und `path` alle übereinstimmen oder auf beiden Seiten fehlen:

* Ein Eintrag ohne `ref` deckt keine Quelle mit `ref: "main"` ab.
* Ein Eintrag für `your-org/your-marketplace` deckt keine `git`-URL ab, die das gleiche Repository klont.
* Ein nachgestellter Schrägstrich, ein `.git`-Suffix oder `ssh://` anstelle von `https://` ist ein anderer Wert. Wenn ein Marketplace von mehr als einer URL geklont werden kann, bevorzugen Sie einen `hostPattern`-Eintrag.

Besitzer-Wildcard-Einträge folgen den genauen Regeln für `ref` und stimmen mit jedem `path` im Repository überein, es sei denn, der Eintrag fixiert einen. Wildcard-Matching ist auf der Zulassungsliste case-sensitiv.

<h4 id="keep-skills-directory-plugins-loading">
  Skills-Verzeichnis-Plugins laden halten
</h4>

Skills-Verzeichnis-Plugins sind die Plugins, die Benutzer unter `~/.claude/skills/` oder in einem Projekt `.claude/skills/` in Ordnern behalten, die ein `.claude-plugin/plugin.json` tragen. Wenn Sie eine Zulassungsliste ohne einen `{ "source": "skills-dir" }`-Eintrag festlegen, werden sie nicht geladen. Einfache [Skills](/docs/de/skills), d. h. ein `SKILL.md` ohne dieses Manifest, werden weiterhin geladen.

<h4 id="marketplaces-hosted-on-claude-ai">
  Auf claude.ai gehostete Marketplaces
</h4>

Die Zulassungsliste und Blockliste stimmen mit einem [auf claude.ai gehosteten Marketplace](/docs/de/plugins/install#add-from-claude-ai) nach seinem Host überein. Um einen zuzulassen oder zu blockieren, fügen Sie einen `hostPattern`-Eintrag hinzu, der `claude.ai` zu `strictKnownMarketplaces` oder `blockedMarketplaces` abgleicht. Auf der Zulassungsliste lässt ein solcher Eintrag die Marketplaces Ihrer Organisation auf claude.ai und die claude.ai-Standard-Marketplaces zu, aber nicht einen Marketplace aus den eigenen claude.ai-Uploads eines Mitglieds oder einen, dessen Bereich claude.ai nicht angegeben hat. Erfordert Claude Code v2.1.273 oder später.

<h4 id="lock-every-source-out">
  Jede Quelle sperren
</h4>

Eine leere Zulassungsliste, `[]`, sperrt jede Marketplace-Quelle, einschließlich des offiziellen Marketplace.

Diese Sperrung deckt nicht die Plugins ab, die [von claude.ai synchronisiert werden](/docs/de/plugins/loading#synced-plugins), die Claude Code aus dem Konto jedes Benutzers herunterlädt, anstatt aus einem Marketplace. Um diese auch zu stoppen, setzen Sie [`syncClaudeAiPlugins`](/docs/de/settings-reference#syncclaudeaiplugins) in verwalteten Einstellungen auf `false` oder schalten Sie Skills für Ihre Organisation auf claude.ai aus.

<h3 id="blocklist-with-blockedmarketplaces">
  Blockliste mit `blockedMarketplaces`
</h3>

`blockedMarketplaces` nimmt die gleichen Quellobjekte wie [`strictKnownMarketplaces`](#allowlist-with-strictknownmarketplaces) und wird zuerst überprüft, sodass eine Quelle auf beiden Listen blockiert wird. Blocklisten-Matching ist breiter als Zulassungslisten-Matching:

* Git-URLs werden kanonisiert, sodass die `git@`- und `https://`-Formen, `.git`-Suffixe und nachgestellte Schrägstriche eines `github.com`-Repositorys alle den gleichen Eintrag abgleichen.
* Ein `github`-Eintrag blockiert auch die äquivalente `git`-URL und umgekehrt.
* Für einen `owner/*`-Eintrag ist der Besitzervergleich case-insensitiv.
* Ein Eintrag ohne `ref` oder `path` blockiert jeden ref und path der Repositorys, die er abgleicht.

Dieser Eintrag blockiert jedes Repository unter einem GitHub-Besitzer:

```json theme={null}
{
  "blockedMarketplaces": [
    { "source": "github", "repo": "untrusted-org/*" }
  ]
}
```

Die `url`-Einträge in `blockedMarketplaces` gelten auch, wenn ein Benutzer eine `https://`-Repository-URL hinzufügt, die Claude Code [klont, anstatt zu fetchen](/docs/de/plugins/cli-reference#plugin-marketplace-add), wie eine einfache `github.com`- oder `gitlab.com`-Repository-URL. Der Benutzer kann diese URL nicht hinzufügen, wenn ein Eintrag sie nennt. Der Abgleich ignoriert das `.git`-Suffix und jeden ref, den der Benutzer nach `#` anhängt. Erfordert Claude Code v2.1.232 oder später.

Ein `{ "source": "skills-dir" }`-Eintrag hier stoppt [Skills-Verzeichnis-Plugins](#keep-skills-directory-plugins-loading) vom Laden, sowohl von `~/.claude/skills/` als auch von einem Projekt `.claude/skills/`.

Eine Blockliste, die nur diesen Eintrag nennt, zählt nicht als aktive Einschränkung, sodass sie nicht [Plugins, deren Marketplace Claude Code nicht finden kann](#restrict-what-users-can-install), vom Laden stoppt.

<h3 id="allow-the-official-marketplace-and-your-own">
  Den offiziellen Marketplace und Ihren eigenen zulassen
</h3>

Die meisten Organisationen lassen den offiziellen Marketplace und ihren eigenen zu und registrieren beide, sodass jeder Computer sie hat. Diese verwaltete Einstellungsrichtlinie lässt beide Marketplaces zu, registriert beide, erzwingt zwei Plugins und lehnt `--plugin-dir` ab:

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "anthropics/claude-plugins-official" },
    { "source": "github", "repo": "your-org/*" },
    { "source": "skills-dir" }
  ],
  "extraKnownMarketplaces": {
    "claude-plugins-official": {
      "source": { "source": "github", "repo": "anthropics/claude-plugins-official" }
    },
    "your-marketplace": {
      "source": { "source": "github", "repo": "your-org/your-marketplace" }
    }
  },
  "enabledPlugins": {
    "code-formatter@your-marketplace": true,
    "deploy-helper@your-marketplace": true
  },
  "disableSideloadFlags": true
}
```

Auf einem Computer mit dieser Richtlinie schlägt das Hinzufügen einer Quelle außerhalb der Liste fehl, z. B. `/plugin marketplace add https://example.com/other-marketplace.git`, mit einer Nachricht, die `is blocked by enterprise policy` gefolgt von den zulassenen Quellen enthält. `claude --plugin-dir ./x` beendet sich mit einer Nachricht, die `disableSideloadFlags` nennt.

Der `{ "source": "skills-dir" }`-Eintrag hält [Skills-Verzeichnis-Plugins](#keep-skills-directory-plugins-loading) unter dieser Zulassungsliste laden. Entfernen Sie diesen Eintrag und sie werden nicht geladen.

Registrieren Sie beide Marketplaces mit expliziten `extraKnownMarketplaces`-Einträgen, wie diese Richtlinie es tut, anstatt sich auf die Zulassungsliste oder auf die Selbstregistrierung des offiziellen Marketplace zu verlassen:

* **Die Zulassungsliste registriert nichts**: Ein `extraKnownMarketplaces`-Eintrag tut es, und er muss selbst die Zulassungsliste bestehen. Claude Code weigert sich, einen verwalteten Marketplace zu registrieren, dessen Quelle die Zulassungsliste nicht abgleicht.
* **Der offizielle Marketplace registriert sich nur in einer interaktiven Terminal-Sitzung**: Auch dort registriert er sich nur, wenn die Zulassungsliste es zulässt. Ein `-p`-Lauf oder ein Terminal, das an eine Cloud-Sitzung angehängt ist, registriert es nie.
* **Ein blockierter Versuch wird erinnert**: Wenn ein Computer jemals unter einer Richtlinie lief, die den offiziellen Marketplace blockierte, zeichnet Claude Code den blockierten Versuch auf und versucht nicht erneut, nachdem die Richtlinie geändert wird. Eine `[]`-Sperrung ist eine solche Richtlinie. Dieser Computer registriert ihn wieder nur durch einen `extraKnownMarketplaces`-Eintrag wie den in dieser Richtlinie, einen `enabledPlugins`-Eintrag für eines seiner Plugins oder ein manuelles `/plugin marketplace add`.

<h2 id="set-update-policy">
  Aktualisierungsrichtlinie festlegen
</h2>

Sie können die Aktualisierungsrichtlinie pro Marketplace, für die ganze Flotte oder pro Benutzergruppe durch Release-Kanäle festlegen.

<h3 id="turn-auto-update-on-or-off-per-marketplace">
  Schalten Sie Auto-Update pro Marketplace ein oder aus
</h3>

Plugin-Auto-Update läuft im Hintergrund nach dem Start für Marketplaces, die es eingeschaltet haben. Für welche Marketplaces es standardmäßig eingeschaltet ist, siehe [Wenn Auto-Update läuft](/docs/de/plugins/loading#when-auto-update-runs). Um für die Flotte zu entscheiden, setzen Sie `"autoUpdate": true` oder `false` auf einem verwalteten `extraKnownMarketplaces`-Eintrag:

* Wenn der verwaltete Eintrag das Feld setzt, lehnt Claude Code den `/plugin`-Toggle des Benutzers mit einem Fehler ab, der mit `Auto-update for '<name>' is set by` beginnt.
* Wenn der verwaltete Eintrag das Feld nicht setzt, bleibt der Toggle des Benutzers erhalten.

<h3 id="turn-updates-off-for-the-whole-fleet">
  Schalten Sie Updates für die ganze Flotte aus
</h3>

Um Plugin-Auto-Update für jeden Marketplace auszuschalten, setzen Sie `DISABLE_AUTOUPDATER` im verwalteten `env`-Block, wie dieses Beispiel zeigt. Die gleiche Variable stoppt auch Claude Code's eigene Updates:

```json theme={null}
{
  "env": {
    "DISABLE_AUTOUPDATER": "1"
  }
}
```

Um Claude Code's eigene Updates zu stoppen, aber Plugin-Auto-Update zu behalten, fügen Sie `"FORCE_AUTOUPDATE_PLUGINS": "1"` zum gleichen Block hinzu. Die anderen [Umgebungsvariablen, die Plugin-Auto-Update stoppen](/docs/de/plugins/loading#when-auto-update-runs), funktionieren auf die gleiche Weise.

`DISABLE_AUTOUPDATER` deckt keine Plugins mit einer [`command`-Quelle](/docs/de/plugins/marketplace-reference#command-plugin-source) ab. Claude Code führt den Befehl jedes aktivierten Plugins jede Sitzung erneut aus und installiert die Ausgabe, wenn sie sich geändert hat. Für das, was diese Läufe stoppt, siehe [Wenn eine Command-Quelle erneut läuft](/docs/de/plugins/loading#when-a-command-source-re-runs).

<h3 id="assign-release-channels-to-user-groups">
  Weisen Sie Release-Kanäle Benutzergruppen zu
</h3>

Um stabile und Early-Access-Kanäle zu betreiben, hosten Sie zwei Marketplaces, die auf verschiedene refs der gleichen Plugins verweisen. Geben Sie dann jeder Benutzergruppe ihren eigenen Marketplace durch entweder separate endpunktseitig verwaltete Einstellungen oder eine Gateway-Richtlinie. Serverseitig verwaltete Einstellungen aus der Administratorkonsole [gelten für jeden Benutzer in Ihrer Organisation](/docs/de/server-managed-settings#current-limitations), daher können sie nicht verschiedene Einstellungen verschiedenen Gruppen zuweisen.

* Stellen Sie separate [endpunktseitig verwaltete Einstellungen](/docs/de/managed-settings#delivery-mechanisms), z. B. eine verwaltete Einstellungsdatei oder ein MDM-Profil, für die Geräte jeder Gruppe bereit. Um zu überprüfen, ob die Datei oder das Profil pro Gruppe auf einem Gerät gilt, das auch eine organisationsweite Quelle hat, siehe [Wie Claude Code verwaltete Quellen kombiniert](/docs/de/managed-settings#precedence-within-the-managed-tier).
* Definieren Sie eine [Claude Apps Gateway-Richtlinie](/docs/de/claude-apps-gateway-config#managed) pro Gruppe. Das Gateway wendet die erste Richtlinie an, deren Abgleichsregel zu einem Benutzer passt, daher ordnen Sie die Richtlinien so, dass jeder Benutzer die Richtlinie seiner Gruppe erreicht. Die `extraKnownMarketplaces`-Karte dieser Richtlinie wird nicht mit einer anderen Richtlinie zusammengeführt, daher listen Sie jeden Marketplace auf, den die Gruppe benötigt, nicht nur ihren Kanal-Marketplace.

Mit beiden Mechanismen erhält die stabile Gruppe diese Konfiguration:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "stable-tools": {
      "source": { "source": "github", "repo": "your-org/stable-tools" }
    }
  }
}
```

Die Early-Access-Gruppe erhält stattdessen `latest-tools`. Um die beiden Marketplaces einzurichten, siehe [Betreiben Sie Release-Kanäle](/docs/de/plugins/host-marketplace#run-release-channels).

<h2 id="recommend-plugins">
  Plugins empfehlen
</h2>

Marketplace-Besitzer können `relevance`-Signale an Einträge anhängen, damit Claude Code das Plugin vorschlägt, wenn ein Projekt übereinstimmt.

Vorschläge aus einem Marketplace erscheinen nur, wenn er auf dem Computer des Benutzers registriert ist, Sie seinen Namen in `pluginSuggestionMarketplaces` in verwalteten Einstellungen auflistet, und Sie seine Quelle in der gleichen Richtlinie deklarieren. Deklarieren Sie die Quelle entweder als den `extraKnownMarketplaces`-Eintrag des Marketplace oder als einen Zulassungslisten-Eintrag. Der offizielle Marketplace benötigt nur den Namen. Siehe [Aktivieren Sie Vorschläge in verwalteten Einstellungen](/docs/de/plugins/relevance#enable-suggestions-in-managed-settings).

<h2 id="audit-and-review">
  Überprüfen und überprüfen
</h2>

OpenTelemetry-Ereignisse und die Analytics-API sagen Ihnen, was Ihre Flotte installiert und ausführt.

Für das, was ein Plugin auf einem Computer ausführen kann und was jede Vertrauensstufe erlaubt, lesen Sie [Plugin-Sicherheit](/docs/de/plugins/security), bevor Sie einen Marketplace genehmigen.

<h3 id="opentelemetry-events">
  OpenTelemetry-Ereignisse
</h3>

`claude_code.plugin_installed` zeichnet jede Installation auf, und `claude_code.plugin_loaded` zeichnet jedes aktivierte Plugin beim Sitzungsstart auf. Beide Ereignisse redigieren oder weglassen Namen von Drittanbieter-Plugins und Marketplaces, es sei denn, Sie setzen `OTEL_LOG_TOOL_DETAILS=1`, wie [Redigierte Plugin-Namen in Ihrem Backend](/docs/de/plugins/measure#redacted-plugin-names-in-your-backend) zeigt. Feldlisten befinden sich unter [Plugin-Installationsereignis](/docs/de/monitoring-usage#plugin-installed-event) und [Plugin-Ladeereignis](/docs/de/monitoring-usage#plugin-loaded-event).

<h3 id="analytics-api">
  Analytics-API
</h3>

Im Enterprise-Plan gibt `GET /v1/organizations/analytics/plugins` pro Plugin, pro Tag Install- und Aufrufraten über Claude Code und Cowork zurück. Sie können die Raten nach Benutzer oder RBAC-Gruppe gruppieren. Plugin-Aktivität, die Anthropic ohne einen Plugin-Namen erreicht, erscheint in einer aggregierten `third-party`-Zeile. Siehe die [Endpunkt-Referenz](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) und [Greifen Sie programmgesteuert auf Daten zu](/docs/de/analytics#access-data-programmatically) für den Schlüssel, den sie benötigt.

<h2 id="plan-for-what-managed-settings-can’t-enforce">
  Planen Sie, was verwaltete Einstellungen nicht erzwingen können
</h2>

Diese Anfragen aus Sicherheitsüberprüfungen haben keinen dedizierten Schlüssel im aktuellen Einstellungsschema. Die nächsten vorhandenen Steuerelemente sind:

* **Pro-Benutzer- oder Pro-Gruppen-Targeting**: Jeder Plugin-Schlüssel gilt für jeden Benutzer, der die Einstellungen erhält. Serverseitig verwaltete Einstellungen stellen eine Konfiguration pro Organisation bereit. Für Pro-Gruppen-Richtlinie verwenden Sie separate endpunktseitig verwaltete Einstellungen oder Gateway-Richtlinien, wie unter [Weisen Sie Release-Kanäle Benutzergruppen zu](#assign-release-channels-to-user-groups).
* **Einschränken von Einträgen in einem zulässigen Marketplace**: Die Zulassungsliste erfüllt Marketplace-Quellen. Um ein Plugin aus einem zulässigen Marketplace zu blockieren, setzen Sie es auf `false` in verwalteten `enabledPlugins`.
* **Verstecken Sie `/plugin`**: Kein Schlüssel deaktiviert den Befehl. Das nächste Äquivalent kombiniert eine Zulassungsliste, die nur Ihren Marketplace benennt, verwaltete `enabledPlugins`-Einträge für die Plugins, die Sie bereitstellen, und `disableSideloadFlags`.
* **Gating `--plugin-dir` durch die Zulassungsliste**: Die Zulassungsliste deckt `--plugin-dir` nicht ab. `disableSideloadFlags` tut es.
* **Erzwingen Sie die claude.ai-Plugin-Toggles durch diese Schlüssel**: [**Organisationseinstellungen > Plugins & Skills**](https://claude.ai/admin-settings/skills?tab=inventory) setzt nicht die Schlüssel auf dieser Seite. Was Mitglieder und Ihre Organisation dort einschalten, erreicht die CLI als [synchronisierte Plugins](/docs/de/plugins/loading#synced-plugins), die ihre eigenen Steuerelemente haben.

<h2 id="troubleshoot-policy">
  Richtlinie fehlerbeheben
</h2>

Wenn sich die Plugin-Richtlinie auf einem Computer nicht wie erwartet verhält, überprüfen Sie zuerst diese Symptome:

* **Die verwaltete Datei wurde nicht geparst**: Wenn eine `managed-settings.json` kein gültiges JSON ist, weigert sich Claude Code zu starten und druckt [einen Fehler, der die Datei benennt](/docs/de/errors#managed-settings-document-could-not-be-parsed). Eine Datei, die geparst wird, aber einen ungültigen Eintrag hat, behält den Rest ihrer Richtlinie. Siehe [Ungültige Einträge in verwalteten Einstellungen](/docs/de/managed-settings#invalid-entries-in-managed-settings).
* **Die verwaltete Quelle wurde nicht geladen**: Führen Sie `/status` aus und suchen Sie nach `Enterprise managed settings` in der `Setting sources`-Zeile. Wenn es fehlt, wurde die Quelle nicht geladen.
* **Ein Benutzer meldet `blocked by enterprise policy`**: Die Meldung benennt den Marketplace oder seine Quelle. Für eine Zulassungsliste listet sie auch die zulässigen Quellen auf. Die benutzerorientierten Einträge befinden sich unter [Fehlerbehebung für Plugins](/docs/de/plugins/troubleshooting).
* **Ein Plugin, das der Benutzer in `~/.claude/settings.json` deaktiviert hat, lädt immer noch**: Eine andere Einstellungsquelle hat es erneut aktiviert, z. B. ein verwalteter `enabledPlugins`-Eintrag, der es erzwingt. `/plugin` und `claude plugin list` zeigen `Disabled in ~/.claude/settings.json but still loads` mit dieser Einstellungsquelle.

<h2 id="next-steps">
  Nächste Schritte
</h2>

* [Marketplace-Referenz](/docs/de/plugins/marketplace-reference#marketplace-sources): Die `source`-Werte, die `extraKnownMarketplaces`, `strictKnownMarketplaces` und `blockedMarketplaces` akzeptieren
* [Hosten und verwalten Sie einen Marketplace](/docs/de/plugins/host-marketplace): Betreiben Sie den Marketplace, auf den Ihre Richtlinie verweist
* [Plugin-Sicherheit und Vertrauen](/docs/de/plugins/security): Was ein Plugin auf einem Computer tun kann und wie man eines vor der Installation überprüft
* [Serverseitig verwaltete Einstellungen](/docs/de/server-managed-settings): Stellen Sie diese Schlüssel aus der claude.ai-Administratorkonsole bereit
* [Fehlerbehebung für Plugins](/docs/de/plugins/troubleshooting#blocked-by-your-organization): Die Meldungen, die Benutzer sehen, wenn die Richtlinie sie blockiert
