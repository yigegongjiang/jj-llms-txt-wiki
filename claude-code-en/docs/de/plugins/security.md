> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugin-Sicherheit und Vertrauen

> Entscheiden Sie, ob Sie einem Plugin vertrauen, bevor Sie es installieren – von dem, was ein Plugin auf Ihrem Computer tun kann, bis hin zu dessen Überprüfung und Deinstallation.

Ein Claude Code-Plugin, das Sie installieren, kann beliebigen Code auf Ihrem Computer mit Ihren Benutzerrechten ausführen.

Sie installieren ein Plugin aus einem Marketplace, das ist der Katalog, den Claude Code abruft. Einige Marketplace-Namen sind [für Anthropics eigene Marketplaces reserviert](#marketplace-tiers), und alle anderen Marketplaces sind von Drittanbietern. Der Name eines Marketplace zeigt Ihnen, wer den Katalog veröffentlicht, nicht was jedes Plugin darin tut. Daher [überprüfen Sie ein Plugin vor der Installation](#review-a-plugin-before-you-install), unabhängig davon, von welchem Marketplace es stammt.

Lesen Sie diese Seite, wenn Sie entscheiden, ob Sie ein Plugin installieren möchten, oder wenn Sie Tools überprüfen, bevor Ihr Team sie verwenden kann.

<Note>
  Diese Fälle werden auf anderen Seiten behandelt:

  * **Claude Codes eigenes Sicherheitsmodell**: siehe [Sicherheit](/docs/de/security)
  * **Einschränkung oder Anforderung von Plugins für eine Organisation**: siehe [Plugins für Ihre Organisation verwalten](/docs/de/plugins/org)
  * **Die Plugins `security-guidance` oder `claude-security`**: diese Seite handelt nicht von ihnen. Siehe [`security-guidance`](/docs/de/security-guidance) und [`claude-security`](/docs/de/claude-security)
</Note>

Beginnen Sie mit [was ein Plugin tun kann](#understand-what-a-plugin-can-do) und [welche Marketplaces Anthropics gehören](#marketplace-tiers), dann [überprüfen Sie das Plugin vor der Installation](#review-a-plugin-before-you-install).

<h2 id="understand-what-a-plugin-can-do">
  Verstehen Sie, was ein Plugin tun kann
</h2>

Ein Plugin kann Inhalte enthalten, die Code auf Ihrem Computer mit Ihren Benutzerrechten ausführen, und Inhalte, die als Anweisungen in Claudes Kontext eingehen. Daher [überprüfen Sie ein Plugin vor der Installation](#review-a-plugin-before-you-install). Hier ist, was ein installiertes Plugin tun kann:

* **Hooks**: die [Hooks](/docs/de/hooks) eines Plugins werden als Shell-Befehle an Punkten im Lebenszyklus von Claude Code ausgeführt, z. B. vor oder nach einem Tool-Aufruf.
* **MCP- und LSP-Server**: Claude Code verbindet sich mit den [MCP-Servern](/docs/de/mcp), die ein aktiviertes Plugin deklariert, und gibt Claude deren Tools. Ein stdio-MCP-Server wird als Prozess ausgeführt, den Claude Code auf Ihrem Computer startet. Claude Code startet auch die Sprachserver, die das Plugin deklariert.
* **`bin/`-Verzeichnis**: Claude Code fügt das `bin/`-Verzeichnis jedes aktivierten Plugins zum `PATH` der Shell des Bash-Tools hinzu, sodass Claudes Bash-Befehle jede dort vorhandene ausführbare Datei ausführen können.
* **Skills, Befehle und Agenten**: diese gehen als Anweisungen in Claudes Kontext ein und beeinflussen daher, was Claude mit den Tools tut, die es bereits hat.
* **Updates**: wenn Auto-Update für den Marketplace aktiviert ist, von dem Sie ein Plugin installiert haben, aktualisiert Claude Code dieses Plugin im Hintergrund, sodass sich die Dateien, die Sie überprüft haben, auf der Festplatte ändern können. [Wann Auto-Update ausgeführt wird](/docs/de/plugins/loading#when-auto-update-runs) enthält die zeitliche Planung. Um Auto-Update pro Marketplace ein- oder auszuschalten, siehe [Plugins aktuell halten](/docs/de/plugins/install#keep-plugins-updated).

Claude Codes [Berechtigungsregeln](/docs/de/permissions) und [Sandbox](/docs/de/sandboxing) decken die Tool-Aufrufe ab, die Claude macht, nicht den Code, den ein Plugin selbst ausführt:

* **Hooks und Server-Prozesse**: Befehls-Hooks führen Shell-Befehle mit Ihren vollständigen Benutzerberechtigungen aus. Claude Code führt Hooks und MCP-Server außerhalb der Sandbox aus.
* **Claudes Tool-Aufrufe**: ein Aufruf eines der MCP-Tools des Plugins und ein Bash-Befehl, der eine ausführbare Datei aus dem `bin/`-Verzeichnis des Plugins ausführt, sind Tool-Aufrufe, daher gelten Ihre Berechtigungsregeln für sie.

Die Installation eines Plugins aktiviert es auch, es sei denn, sein Manifest oder der Marketplace-Eintrag setzt [`defaultEnabled: false`](/docs/de/plugins/install#choose-an-install-scope) und Sie haben es nicht selbst aktiviert.

Um ein Plugin zu entfernen, dem Sie nicht mehr vertrauen, siehe [Entfernen Sie ein Plugin, dem Sie nicht mehr vertrauen](#remove-a-plugin-you-no-longer-trust).

<h2 id="marketplace-tiers">
  Identifizieren Sie Anthropics Marketplaces nach Name
</h2>

Der Name eines Marketplace ordnet ihn in eine von drei Ebenen ein: offiziell, Community oder Drittanbieter. Claude Code akzeptiert die offiziellen und Community-Namen nur für Marketplaces, die aus `github.com/anthropics/`-Repositories stammen, daher kann sich ein Marketplace von Drittanbietern nicht als Anthropic-Marketplace ausgeben. Ein Marketplace, den ein Kollege oder Ihre Organisation veröffentlicht, ist ein Drittanbieter-Marketplace.

Die Tabelle zeigt, welche Namen in jede Ebene fallen:

| Ebene         | Welche Marketplaces                                                                             |
| :------------ | :---------------------------------------------------------------------------------------------- |
| Offiziell     | Die [offiziellen Marketplace-Namen](#official-marketplace-names), wie `claude-plugins-official` |
| Community     | `claude-community`, `claude-plugins-community` und `healthcare`                                 |
| Drittanbieter | Alle anderen Marketplaces                                                                       |

Wenn der `claude-community`-Katalog ein Plugin auf einen Commit-SHA festlegt, was er für fast jeden Eintrag tut, weigert sich Claude Code, einen anderen Commit zu installieren.

<h3 id="official-marketplace-names">
  Offizielle Marketplace-Namen
</h3>

Diese Marketplace-Namen bilden die offizielle Ebene:

* `claude-plugins-official`
* `claude-code-marketplace`
* `claude-code-plugins`
* `anthropic-marketplace`
* `anthropic-plugins`
* `agent-skills`
* `anthropic-agent-skills`
* `life-sciences`
* `knowledge-work-plugins`
* `claude-for-legal`
* `claude-for-financial-services`
* `financial-services-plugins`
* `first-party-plugins`
* `claude-tag-plugins`

Wie sich die offiziellen, Community- und Demo-Marketplaces unterscheiden und wo Sie sehen können, was jeder auflistet, siehe [Anthropics Marketplaces](/docs/de/plugins/anthropic-marketplaces).

<h2 id="review-a-plugin-before-you-install">
  Überprüfen Sie ein Plugin vor der Installation
</h2>

Bevor Sie ein Plugin installieren, schauen Sie sich an, was es hinzufügt und woher es kommt.

<Steps>
  <Step title="Überprüfen Sie die Quelle des Marketplace">
    Führen Sie in Ihrer Shell `claude plugin marketplace list` aus, um die Quelle zu drucken, von der jeder Marketplace hinzugefügt wurde, z. B. ein GitHub-Repository oder ein Verzeichnis.
  </Step>

  <Step title="Lesen Sie den Detailbereich">
    Führen Sie in einer Claude Code-Sitzung `/plugin` aus und wählen Sie das Plugin aus. Der Detailbereich zeigt einen Abschnitt **Will install** (Wird installiert), der die Befehle, Agenten, Skills, Hooks und MCP- und LSP-Server des Plugins auflistet. Für ein Plugin, für das Anthropic keine veröffentlichten Komponentendaten hat, zeigt der Abschnitt, was der Marketplace-Eintrag deklariert, oder eine Notiz: `Components will be discovered at installation` (Komponenten werden bei der Installation erkannt) für ein Plugin, das im Marketplace gespeichert ist, oder `Component summary not available for remote plugin` (Komponentenzusammenfassung nicht verfügbar für Remote-Plugin) für eines, das von anderswo abgerufen wird.
  </Step>

  <Step title="Lesen Sie die Quelle des Plugins">
    Wählen Sie im Detailbereich **Open homepage** (Homepage öffnen) oder **View on GitHub** (Auf GitHub anzeigen) unter den Installationsoptionen. Wenn der Bereich keines von beiden anbietet, öffnen Sie das Marketplace-Repository, das Sie im ersten Schritt gefunden haben. Finden Sie das Verzeichnis des Plugins dort. Der Abschnitt **Will install** zeigt, dass ein Hook vorhanden ist, aber nicht, was er ausführt. Lesen Sie daher diese Dateien im Verzeichnis des Plugins:

    * **`hooks/hooks.json`**: der Befehl, den jeder Hook ausführt
    * **`.mcp.json`**: der Befehl oder die URL jedes Servers
    * **`bin/`**: jede Datei im Verzeichnis
  </Step>

  <Step title="Listet auf, was das Plugin enthält">
    Klonen Sie das Repository, das das Verzeichnis des Plugins enthält, und führen Sie dann `claude --plugin-dir <plugin directory> plugin details <plugin name>` in Ihrer Shell aus, um zu sehen, was Claude Code darin findet. Der Befehl liest die Dateien des Plugins, ohne eine Sitzung zu starten, und druckt eine `Component inventory` (Komponenteninventar), die die Skills und Befehle des Plugins, Agenten, Hooks mit dem Ereignis jedes Hooks und MCP- und LSP-Server auflistet.
  </Step>
</Steps>

Nach der Installation eines Plugins führen Sie `claude plugin details <plugin name>` in Ihrer Shell aus, um das gleiche `Component inventory` für die installierte Kopie unter `~/.claude/plugins/cache/<marketplace>/<plugin>/<version>/` zu drucken.

<h3 id="remove-a-plugin-you-no-longer-trust">
  Entfernen Sie ein Plugin, dem Sie nicht mehr vertrauen
</h3>

Führen Sie in Ihrer Shell [`claude plugin uninstall <plugin>`](/docs/de/plugins/cli-reference#plugin-uninstall) mit dem `--scope` aus, in dem Sie es installiert haben. Überprüfen Sie dann, was die Deinstallation entfernt hat und was sie hinterlassen hat:

* **Persistente Daten**: wenn dies der letzte Scope war, in dem das Plugin installiert war, löscht die Deinstallation auch das Verzeichnis mit persistenten Daten des Plugins, es sei denn, Sie übergeben `--keep-data`.
* **Zwischengespeicherte Dateien**: die Dateien des Plugins bleiben auf der Festplatte unter `~/.claude/plugins/cache/` für 14 Tage, bevor ein [Hintergrund-Sweep sie entfernt](/docs/de/plugins/loading#cleanup-of-previous-versions). Nachdem Sie Ihr letztes Plugin deinstalliert haben, bleiben verwaiste Verzeichnisse bestehen, bis Sie ein anderes installieren. Um die Dateien jetzt zu löschen, entfernen Sie das Verzeichnis des Plugins unter `~/.claude/plugins/cache/<marketplace>/<plugin>/` selbst.
* **Der Marketplace**: wenn Sie dem Besitzer des Marketplace auch nicht vertrauen, [entfernen Sie auch den Marketplace](/docs/de/plugins/install#manage-marketplaces), was jedes Plugin deinstalliert, das Sie von ihm installiert haben.

<h2 id="recognize-when-claude-code-refuses-or-warns">
  Erkennen Sie, wann Claude Code ablehnt oder warnt
</h2>

Der Detailbereich, den Sie auf der Registerkarte **Discover** (Entdecken) oder **Marketplaces** in `/plugin` öffnen, zeigt die gleiche Vertrauenswarnung für jedes Plugin. Claude Code lehnt statt zu warnen in Fällen wie denen unter [Nicht vertrauenswürdige Marketplace-Quellen und fehlgeschlagene Integritätsprüfungen](#untrusted-marketplace-sources-and-failed-integrity-checks) ab.

<h3 id="trust-warning-before-you-install">
  Vertrauenswarnung vor der Installation
</h3>

Die Warnung liest sich gleich, unabhängig davon, von welchem Marketplace das Plugin kommt:

```text theme={null}
Make sure you trust a plugin before installing, updating, or using it. Anthropic does not control what MCP servers, files, or other software are included in plugins and cannot verify that they will work as intended or that they won't change. See each plugin's homepage for more information.
```

Wenn Ihre Organisation `pluginTrustMessage` in [verwalteten Einstellungen](/docs/de/plugins/org) setzt, hängt Claude Code diesen Text an die Warnung an.

<h3 id="untrusted-marketplace-sources-and-failed-integrity-checks">
  Nicht vertrauenswürdige Marketplace-Quellen und fehlgeschlagene Integritätsprüfungen
</h3>

Claude Code weigert sich, einen Marketplace zu laden oder ein Plugin in diesen Fällen zu installieren, jeder mit seiner eigenen Fehlermeldung:

* **Nicht vertrauenswürdige Marketplace-Quelle**: wenn ein Marketplace einen offiziellen oder Community-Namen verwendet, aber seine Quelle außerhalb von `github.com/anthropics/` liegt, stoppt Claude Code das Laden des Marketplace und der Plugins, die Sie von ihm installiert haben. Der Fehler ist [Marketplace is registered from an untrusted source](/docs/de/errors#marketplace-is-registered-from-an-untrusted-source).
* **Archiv-Integrität**: wenn ein Marketplace-Eintrag eine [`archive`-Quelle](/docs/de/plugins/marketplace-reference#archive-plugin-source) auf einen `sha256`-Digest festlegt und der Digest der heruntergeladenen Datei nicht übereinstimmt, weigert sich Claude Code die Installation. Der Fehler ist [Plugin archive integrity check failed](/docs/de/errors#plugin-archive-integrity-check-failed).

Der `sha256`-Pin ist getrennt vom Commit-SHA-Pin des Community-Katalogs, der den Git-Commit auswählt, der ausgecheckt werden soll.

<h2 id="enforce-plugin-controls-for-your-organization">
  Erzwingen Sie Plugin-Kontrollen für Ihre Organisation
</h2>

Mit [verwalteten Einstellungen](/docs/de/plugins/org) kann ein Administrator diese Plugin-Kontrollen erzwingen:

* Whitelist oder Blacklist von Marketplace-Quellen
* Erzwungenes Aktivieren von Plugins
* Deaktivieren Sie die Flags `--plugin-dir` und `--plugin-url` und die Variable `CLAUDE_CODE_PLUGIN_DIRS`
* Begrenzen Sie Hooks auf diejenigen aus verwalteten Einstellungen und erzwungenen Plugins
* Verhindern Sie, dass Plugins aus den claude.ai-Konten von Mitgliedern in Claude Code geladen werden, mit [`syncClaudeAiPlugins`](/docs/de/plugins/org#control-matrix)

Die [Kontrollmatrix](/docs/de/plugins/org#control-matrix) sagt, was jeder Schlüssel tut und nicht abdeckt.

<h2 id="find-plugins-in-telemetry">
  Finden Sie Plugins in der Telemetrie
</h2>

Wenn Ihre Organisation Claude Codes [OpenTelemetry-Ereignisse](/docs/de/monitoring-usage) in sein eigenes Backend exportiert, entscheiden die [Marketplace-Ebenen](#marketplace-tiers), welche Plugin-Namen dort erscheinen:

* **[Plugin loaded event](/docs/de/monitoring-usage#plugin-loaded-event)**: das Ereignis meldet offizielle Ebenen-Plugin- und Marketplace-Namen wie sie sind. Für die Community- und Drittanbieter-Ebenen sind `plugin.name` und `marketplace.name` die Literalzeichenfolge `third-party`, es sei denn, Sie setzen `OTEL_LOG_TOOL_DETAILS=1`.
* **Plugin-Scope**: der `plugin.scope` des geladenen Ereignisses meldet immer noch, woher das Plugin kam, z. B. `org` für ein Plugin, das Ihre verwalteten Einstellungen aktivieren, oder `user-local` für alle anderen Drittanbieter-Plugins. Das [Plugin loaded event](/docs/de/monitoring-usage#plugin-loaded-event) listet jeden Wert auf.
* **[Plugin installed event](/docs/de/monitoring-usage#plugin-installed-event)**: es sei denn, Sie setzen `OTEL_LOG_TOOL_DETAILS=1`, lässt das Ereignis die Namenfelder für Nicht-Offizielle-Plugins weg, anstatt `third-party` zu melden.
* **[Claude Code Analytics API](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list)**: Claude Code meldet Plugins aus den offiziellen und Community-Ebenen nach Name und meldet alle anderen Plugins als `third-party`.

<h2 id="next-steps">
  Nächste Schritte
</h2>

* [Plugins für Ihre Organisation verwalten](/docs/de/plugins/org): beschränken Sie, von welchen Marktplätzen Benutzer installieren können, und erzwingen Sie die, denen Sie vertrauen
* [Plugins installieren und verwalten](/docs/de/plugins/install): überprüfen Sie den Detailbereich eines Plugins, bevor Sie einen Umfang wählen
* [Anthropic-Marktplätze](/docs/de/plugins/anthropic-marketplaces): welche Marktplatznamen gehören Anthropic
* [Sicherheit](/docs/de/security): Claude Code's eigenes Sicherheitsmodell
