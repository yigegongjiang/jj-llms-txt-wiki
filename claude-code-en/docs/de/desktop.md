> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Desktop-Anwendung

> Nutzen Sie Claude Code Desktop optimal: parallele Sitzungen mit Git-Isolation, Drag-and-Drop-Pane-Layout, integriertes Terminal und Datei-Editor, Seitenchats, Computernutzung, Dispatch-Sitzungen von Ihrem Telefon, visuelle Diff-Überprüfung, App-Vorschau, PR-Überwachung, Konnektoren und Unternehmenskonfiguration.

Die Claude Desktop-App hat drei Registerkarten: **Chat** für Gespräche, **Cowork** für [Dispatch und längere agentengestützte Arbeiten](https://claude.com/product/cowork) und **Code** für Softwareentwicklung. Diese Seite ist die Referenz für die Registerkarte Code.

<CardGroup cols={3}>
  <Card title="Download for macOS" icon="apple" href="https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Universal build for Intel and Apple Silicon
  </Card>

  <Card title="Download for Windows" icon="windows" href="https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code&utm_medium=docs">
    For x64 processors
  </Card>

  <Card title="Get Claude for Linux (beta)" icon="linux" href="/docs/en/desktop-linux">
    apt or .deb for Ubuntu and Debian
  </Card>
</CardGroup>

For Windows ARM64, download the [ARM64 installer](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs). On Linux, install with apt; see [Claude Desktop on Linux](/docs/en/desktop-linux).

Nach der Installation starten Sie Claude, melden sich an und klicken auf die Registerkarte **Code**. Beim ersten Öffnen unter Windows benötigen Sie [Git für Windows](https://git-scm.com/downloads/win) installiert; starten Sie die App nach der Installation neu. Eine Anleitung für Ihre erste Sitzung finden Sie im [Leitfaden „Erste Schritte"](/docs/de/desktop-quickstart).

In der Registerkarte Code ist jedes Gespräch eine **Sitzung**: Es hat seinen eigenen Chat-Verlauf, Projektordner und Code-Änderungen, unabhängig von jeder anderen Sitzung. Die Seitenleiste listet Ihre Sitzungen auf und ermöglicht es Ihnen, mehrere parallel auszuführen. Innerhalb einer Sitzung können Sie:

* [Diffs überprüfen und kommentieren](#review-changes-with-diff-view), dann [den resultierenden PR durch CI überwachen](#monitor-pull-request-status)
* [Ihre laufende App](#preview-your-app) in einem eingebetteten Browser in der Vorschau anzeigen, während Claude seine eigenen Änderungen überprüft, und [externe Websites](#browse-external-sites) daneben öffnen
* [Panes anordnen](#arrange-your-workspace) für Chat, Diff, Browser, Terminal und Datei-Editor nebeneinander
* Eine [Seitenfrage](#ask-a-side-question-without-derailing-the-session) stellen, die den Kontext der Sitzung nutzt, ohne sie zu beeinträchtigen
* [Externe Tools verbinden](#connect-external-tools) wie GitHub, Slack und Linear
* Claude [Apps öffnen und Ihren Bildschirm steuern lassen](#let-claude-use-your-computer)
* Auf Ihrem Computer, in der [Cloud](#run-long-running-tasks-remotely) oder über [SSH](#ssh-sessions) ausführen

Für [geplante wiederkehrende Arbeiten](/docs/de/desktop-scheduled-tasks), [Tastaturkürzel](#keyboard-shortcuts) oder [Aufgaben von Ihrem Telefon senden](#sessions-from-dispatch) siehe die verlinkten Seiten und Abschnitte. Wenn Sie bereits die Terminal-basierte CLI verwenden, siehe den [CLI-Vergleich](#coming-from-the-cli) für das, was übertragen wird.

<h2 id="start-a-session">
  Sitzung starten
</h2>

Bevor Sie Ihre erste Nachricht senden, konfigurieren Sie vier Dinge im Eingabebereich:

* **Umgebung**: Wählen Sie, wo Claude ausgeführt wird. Wählen Sie **Lokal** für Ihren Computer, **Remote** für von Anthropic gehostete Cloud-Sitzungen, eine [**SSH-Verbindung**](#ssh-sessions) für einen von Ihnen verwalteten Remote-Computer oder unter Windows eine [**WSL-Distribution**](/docs/de/desktop-wsl). Siehe [Umgebungskonfiguration](#environment-configuration).
* **Projektordner**: Wählen Sie den Ordner oder das Repository aus, in dem Claude arbeitet. Für Cloud-Sitzungen können Sie [mehrere Repositories](#run-long-running-tasks-remotely) hinzufügen.
* **Modell**: Wählen Sie ein [Modell](/docs/de/model-config#available-models) aus dem Dropdown neben der Schaltfläche „Senden". Sie können dies während der Sitzung ändern.
* **Berechtigungsmodus**: Wählen Sie, wie viel Autonomie Claude aus dem [Moduswahlschalter](#choose-a-permission-mode) hat. Sie können dies während der Sitzung ändern.

Geben Sie Ihre Aufgabe ein und drücken Sie **Eingabe**, um zu starten. Jede Sitzung verfolgt ihren eigenen Kontext und Änderungen unabhängig.

<h2 id="work-with-code">
  Arbeiten mit Code
</h2>

Geben Sie Claude den richtigen Kontext, kontrollieren Sie, wie viel es eigenständig tut, und überprüfen Sie, was es geändert hat.

<h3 id="use-the-prompt-box">
  Verwenden Sie das Eingabefeld
</h3>

Geben Sie ein, was Claude tun soll, und drücken Sie **Eingabe**, um zu senden. Claude liest Ihre Projektdateien, nimmt Änderungen vor und führt Befehle basierend auf Ihrem [Berechtigungsmodus](#choose-a-permission-mode) aus. Sie können Claude jederzeit unterbrechen: Klicken Sie auf die Stoppschaltfläche, um sofort zu unterbrechen, oder geben Sie eine Korrektur ein und drücken Sie **Eingabe**, um sie zu senden, ohne die laufende Aktion zu stoppen. Claude liest die Korrektur, sobald die aktuelle Aktion abgeschlossen ist, und passt sich an, bevor der nächste Schritt erfolgt.

Die Schaltfläche **+** neben dem Eingabefeld gibt Ihnen Zugriff auf Dateianhänge, [Skills](#use-skills), [Konnektoren](#connect-external-tools) und [Plugins](#install-plugins).

<h3 id="add-files-and-context-to-prompts">
  Fügen Sie Dateien und Kontext zu Eingaben hinzu
</h3>

Das Eingabefeld unterstützt zwei Möglichkeiten, um externen Kontext einzubinden:

* **@mention-Dateien**: Geben Sie `@` gefolgt von einem Dateinamen ein, um eine Datei zum Gesprächskontext hinzuzufügen. Claude kann diese Datei dann lesen und referenzieren. @mention ist nicht in Cloud-Sitzungen und WSL-Sitzungen verfügbar.
* **Dateien anhängen**: Hängen Sie Bilder, PDFs und andere Dateien an Ihre Eingabe an, indem Sie die Schaltfläche „Anhängen" verwenden, oder ziehen Sie Dateien direkt in die Eingabe. Dies ist nützlich zum Teilen von Screenshots von Fehlern, Design-Mockups oder Referenzdokumenten.

<h3 id="choose-a-permission-mode">
  Wählen Sie einen Berechtigungsmodus
</h3>

Berechtigungsmodi kontrollieren, wie viel Autonomie Claude während einer Sitzung hat: ob es vor dem Bearbeiten von Dateien, dem Ausführen von Befehlen oder beidem fragt. Sie können Modi jederzeit mit dem Moduswahlschalter neben der Schaltfläche „Senden" wechseln. Beginnen Sie mit Manual, um genau zu sehen, was Claude tut, und wechseln Sie dann zu Accept edits oder Plan, wenn Sie sich wohler fühlen.

Um einen Standardmodus für neue lokale Sitzungen festzulegen, fügen Sie `permissions.defaultMode` zu Ihrer [Einstellungsdatei](/docs/de/settings#settings-files) hinzu. Die Desktop-App liest die gleichen Einstellungsdateien wie die CLI. Ein Modus, den Sie im Wahlschalter auswählen, wird pro Ordner gespeichert und hat Vorrang vor `defaultMode` für diesen Ordner, außer Plan, das nur für die aktuelle Sitzung gilt.

| Modus                  | Einstellungsschlüssel | Verhalten                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| ---------------------- | --------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Manual**             | `default`             | Claude fragt vor dem Bearbeiten von Dateien oder dem Ausführen von Befehlen. Sie sehen einen Diff und können jede Änderung akzeptieren oder ablehnen. Empfohlen für neue Benutzer.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| **Accept edits**       | `acceptEdits`         | Claude akzeptiert Dateibearbeitungen automatisch und häufige Dateisystem-Befehle wie `mkdir`, `touch` und `mv`, fragt aber immer noch vor dem Ausführen anderer Terminal-Befehle. Verwenden Sie dies, wenn Sie Dateiänderungen vertrauen und schnellere Iterationen wünschen.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| **Plan**               | `plan`                | Claude liest Dateien und führt Befehle aus, um zu erkunden, schlägt dann einen Plan vor, ohne Ihren Quellcode zu bearbeiten. Gut für komplexe Aufgaben, bei denen Sie den Ansatz zuerst überprüfen möchten.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| **Auto**               | `auto`                | Claude führt alle Aktionen mit Hintergrund-Sicherheitsprüfungen aus, die die Ausrichtung mit Ihrer Anfrage überprüfen. Reduziert Berechtigungsaufforderungen bei Beibehaltung der Überwachung. Wird angezeigt, wenn Ihr Konto die [Verfügbarkeitsanforderungen](#auto-mode-availability) unten erfüllt; es gibt keinen separaten Settings-Umschalter dafür.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| **Bypass permissions** | `bypassPermissions`   | Claude läuft ohne Berechtigungsaufforderungen, außer denen, die durch explizite [ask rules](/docs/de/permissions#manage-permissions) erzwungen werden, Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools), MCP-Tools mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) oder Sicherheitsklassifizierer, wenn Claude [auf externen Websites agiert](#browse-external-sites); äquivalent zu `--dangerously-skip-permissions` in der CLI. Aktivieren Sie dies auf Pro- und Max-Plänen in Ihren Einstellungen → Claude Code unter „Allow bypass permissions mode"; auf Team- und Enterprise-Plänen gibt es keinen Settings-Umschalter, und die Organisationsrichtlinie kontrolliert es stattdessen. Verwenden Sie dies nur in sandboxierten Containern oder VMs. |

Frühere Versionen der Code-Registerkarte bezeichneten diese Modi als Ask permissions, Auto accept edits und Plan mode.

Der Berechtigungsmodus `dontAsk` ist nur in der [CLI](/docs/de/permission-modes#allow-only-pre-approved-tools-with-dontask-mode) verfügbar.

<span id="auto-mode-availability" />

Auto mode ist für alle Benutzer auf der Anthropic API verfügbar und erfordert Claude Opus 4.6 oder später oder Sonnet 4.6 oder später. Organisationsadministratoren können Auto mode mit dem Schlüssel `disableAutoMode` in [verwalteten Einstellungen](#managed-settings) ausschalten.

Bei Enterprise-Bereitstellungen, die Desktop zu Google Cloud's Agent Platform weiterleiten, ist Auto mode [standardmäßig verfügbar](/docs/de/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry), und nur Claude Sonnet 5, Opus 4.7 und Opus 4.8 werden dort unterstützt. Vor Claude Code v2.1.207 mussten Enterprise-Bereitstellungen auf Google Cloud's Agent Platform `CLAUDE_CODE_ENABLE_AUTO_MODE` setzen, um Auto mode zu aktivieren.

<Tip title="Best practice">
  Beginnen Sie komplexe Aufgaben im Plan, damit Claude einen Ansatz abbildet, bevor Änderungen vorgenommen werden. Sobald Sie den Plan genehmigen, wechseln Sie zu Accept edits oder Manual, um ihn auszuführen. Siehe [explore first, then plan, then code](/docs/de/best-practices#explore-first-then-plan-then-code) für mehr zu diesem Workflow.
</Tip>

Cloud-Sitzungen unterstützen Accept edits, Plan und Auto. Accept edits entspricht dem `default`-Modus: Cloud-Sitzungen genehmigen Dateibearbeitungen vorab, daher zeigt der Wahlschalter Accept edits statt Manual an. Bypass permissions ist nicht verfügbar, da die Cloud-Umgebung bereits sandboxed ist.

Enterprise-Administratoren können einschränken, welche Berechtigungsmodi verfügbar sind. Siehe [enterprise configuration](#enterprise-configuration) für Details.

<h3 id="preview-your-app">
  Vorschau Ihrer App
</h3>

Claude kann einen Dev-Server starten und ihn im Browser-Pane öffnen, um seine Änderungen zu überprüfen. Dies funktioniert sowohl für Frontend-Web-Apps als auch für Backend-Server: Claude kann API-Endpunkte testen, Server-Protokolle anzeigen und Probleme, die er findet, iterieren. In den meisten Fällen startet Claude den Server automatisch nach dem Bearbeiten von Projektdateien. Sie können Claude auch jederzeit bitten, eine Vorschau anzuzeigen. Standardmäßig [überprüft Claude automatisch](#auto-verify-changes) Änderungen nach jeder Bearbeitung.

Das Browser-Pane kann auch statische HTML-Dateien, PDFs, Bilder und Videos aus Ihrem Projekt öffnen. Klicken Sie auf einen HTML-, PDF-, Bild- oder Videopfad im Chat, um ihn dort zu öffnen.

Aus dem Browser-Pane können Sie:

* Direkt im Browser-Pane mit Ihrer laufenden App interagieren
* Beobachten, wie Claude seine eigenen Änderungen automatisch überprüft: Es macht Screenshots, inspiziert das DOM, klickt auf Elemente, füllt Formulare aus und behebt Probleme, die es findet
* Server aus dem Server-Dropdown in der Sitzungs-Symbolleiste starten oder stoppen
* Cookies und lokalen Speicher über Server-Neustarts hinweg beibehalten, indem Sie **Persist sessions** im Dropdown auswählen, damit Sie sich während der Entwicklung nicht erneut anmelden müssen
* Die Server-Konfiguration bearbeiten oder alle Server auf einmal stoppen

Claude erstellt die anfängliche Server-Konfiguration basierend auf Ihrem Projekt. Wenn Ihre App einen benutzerdefinierten Dev-Befehl verwendet, bearbeiten Sie `.claude/launch.json`, um Ihr Setup zu entsprechen. Siehe [Configure preview servers](#configure-preview-servers) für die vollständige Referenz.

Um gespeicherte Sitzungsdaten zu löschen oder den Browser vollständig auszuschalten, verwenden Sie die Umschalter in Einstellungen → Claude Code.

<h3 id="browse-external-sites">
  Externe Websites durchsuchen
</h3>

Das Browser-Pane ist ein Browser mit Registerkarten, daher können Sie Dokumentation, Issue-Tracker oder andere Websites neben Ihrer laufenden App öffnen. Um den Browser zu öffnen, drücken Sie **Cmd+Shift+B** auf macOS oder **Ctrl+Shift+B** auf Windows, oder wählen Sie ihn aus dem Menü **Views**. Wenn Sie auf einen externen Link im Chat klicken, bietet ein Wahlschalter **Open in app** an, um das Browser-Pane zu verwenden, oder **Default browser**, um Ihren eigenen Browser zu verwenden; **Cmd**-Klick auf macOS oder **Ctrl**-Klick auf Windows öffnet einen Link direkt in Ihrem Systembrowser. Sie können sich auf Websites im Pane anmelden, einschließlich Popup-Anmeldungsflows wie Google OAuth.

Claude kann externe Seiten lesen und mit ihnen interagieren, indem es die gleichen Tools verwendet, die es zum [Überprüfen Ihrer App](#preview-your-app) nutzt, mit zwei zusätzlichen Sicherheitsprüfungen:

* Sicherheitsklassifizierer überprüfen Claudes Schreibaktionen auf externen Seiten, wie Klicken und Tippen, in jedem Berechtigungsmodus. Dies sind die gleichen Klassifizierer, die [Auto mode](#choose-a-permission-mode) verwendet, und wenn sie eine Aktion kennzeichnen, erhalten Sie eine Berechtigungsaufforderung unabhängig vom Modus.
* In Berechtigungsmodi außer Auto und Bypass permissions wird auch eine Domain-Allowlist-Prüfung angewendet, bevor Claude zu einer neuen Website navigiert.

<h4 id="approve-claude’s-actions-on-a-site">
  Genehmigen Sie Claudes Aktionen auf einer Website
</h4>

Wenn Claude zum ersten Mal auf einer externen Website agiert, wird eine Berechtigungskarte angezeigt und Claude wartet auf Ihre Wahl: **Allow once**, **Always allow** oder **Deny**. **Allow once** genehmigt die Aktion, ohne etwas zu speichern. **Always allow** speichert die Genehmigung für diese Website auf Ihrem Gerät, und Sie können sie in Einstellungen widerrufen. Jede Website benötigt ihre eigene Genehmigung, einschließlich Subdomains. Ihre lokalen Dev-Server und Projektdateien benötigen keine Genehmigung, daher funktioniert [auto-verify](#auto-verify-changes) ohne Aufforderungen.

Auch auf einer genehmigten Website wird Claude keine Artikel kaufen, Konten erstellen oder CAPTCHAs umgehen, ohne Ihre Eingabe. Das Durchsuchen im Browser-Pane verwendet das gleiche Sicherheitsmodell wie die [Claude in Chrome-Erweiterung](/docs/de/chrome). Siehe [Using Claude in Chrome safely](https://support.claude.com/en/articles/12902428-using-claude-in-chrome-safely) für die Behandlung sensibler Websites und riskanter Aktionen durch Claude.

<h4 id="choose-between-the-browser-and-the-chrome-extension">
  Wählen Sie zwischen dem Browser-Pane und der Chrome-Erweiterung
</h4>

Das Browser-Pane verwendet ein sauberes Browser-Profil, getrennt von Ihrem persönlichen Browser, ohne Ihre gespeicherten Anmeldungen oder Verlauf. Verwenden Sie es zum Erstellen und Testen Ihrer App und für Websites, die Ihre Identität nicht benötigen. Wenn Sie möchten, dass Claude als Sie in Ihren angemeldeten Sitzungen agiert, verwenden Sie stattdessen die [Claude in Chrome-Erweiterung](/docs/de/chrome), die den Anmeldestatus Ihres Browsers teilt.

<h4 id="restrict-external-browsing-for-your-organization">
  Beschränken Sie das externe Durchsuchen für Ihre Organisation
</h4>

Der Browser folgt den gleichen [Site-Allowlist- und Blocklist-Kontrollen](https://support.claude.com/en/articles/13065128-claude-in-chrome-admin-controls) wie die Claude in Chrome-Erweiterung. Wenn Ihre Organisation diese Listen bereits für die Erweiterung konfiguriert hat, respektiert der Browser sie automatisch. Administratoren können auch Claudes Tools auf externen Seiten mit der verwalteten Einstellung [`browserExternalPageTools`](#managed-settings) ausschalten. Mit deaktivierten Tools können Benutzer immer noch zu externen Websites navigieren; Claudes Tools können sie nicht lesen oder bearbeiten.

Um das externe Durchsuchen vollständig auszuschalten, setzen Sie die verwaltete Einstellung [`disableBrowserExternalNavigation`](#managed-settings) auf `true`. Dies blockiert alle externe Navigation im Browser, einschließlich Websites auf Ihrer Organisationserlaubnis-Liste; localhost Dev-Server und Datei-Vorschauen funktionieren weiterhin. Verwenden Sie `browserExternalPageTools`, um Benutzern zu ermöglichen, externe Websites weiterhin zu durchsuchen, ohne Claudes Tools, und `disableBrowserExternalNavigation`, um externe Websites für Benutzer und Claude zu blockieren.

<h3 id="review-changes-with-diff-view">
  Überprüfen Sie Änderungen mit der Diff-Ansicht
</h3>

Nachdem Claude Änderungen an Ihrem Code vorgenommen hat, können Sie mit der Diff-Ansicht Änderungen dateiweise überprüfen, bevor Sie einen Pull Request erstellen.

Wenn Claude Dateien ändert, wird ein Diff-Statistik-Indikator angezeigt, der die Anzahl der hinzugefügten und entfernten Zeilen anzeigt, z. B. `+12 -1`. Klicken Sie auf diesen Indikator, um den Diff-Viewer zu öffnen, der eine Dateiliste auf der linken Seite und die Änderungen für jede Datei auf der rechten Seite anzeigt.

Um Kommentare zu bestimmten Zeilen hinzuzufügen, klicken Sie auf eine beliebige Zeile im Diff, um ein Kommentarfeld zu öffnen. Geben Sie Ihr Feedback ein und drücken Sie **Eingabe**, um den Kommentar hinzuzufügen. Nach dem Hinzufügen von Kommentaren zu mehreren Zeilen senden Sie alle Kommentare auf einmal:

* **macOS**: drücken Sie **Cmd+Eingabe**
* **Windows**: drücken Sie **Ctrl+Eingabe**

Claude liest Ihre Kommentare und nimmt die angeforderten Änderungen vor, die als neuer Diff angezeigt werden, den Sie überprüfen können.

<h3 id="review-your-code">
  Überprüfen Sie Ihren Code
</h3>

Klicken Sie in der Diff-Ansicht auf **Review code** in der oberen rechten Symbolleiste, um Claude zu bitten, die Änderungen vor dem Commit zu bewerten. Claude untersucht die aktuellen Diffs und hinterlässt Kommentare direkt in der Diff-Ansicht. Sie können auf jeden Kommentar antworten oder Claude bitten, zu überarbeiten.

Die Überprüfung konzentriert sich auf hochwertige Probleme: Kompilierungsfehler, definitive Logikfehler, Sicherheitslücken und offensichtliche Fehler. Sie kennzeichnet keine Stil-, Formatierungs-, bereits vorhandenen Probleme oder etwas, das ein Linter erfassen würde.

<h3 id="monitor-pull-request-status">
  Überwachen Sie den Pull-Request-Status
</h3>

Nachdem Sie einen Pull Request öffnen, wird eine CI-Statusleiste in der Sitzung angezeigt. Claude Code verwendet die GitHub CLI, um Prüfergebnisse abzurufen und Fehler anzuzeigen.

* **Auto-fix**: Wenn aktiviert, versucht Claude automatisch, fehlgeschlagene CI-Prüfungen zu beheben, indem die Fehlerausgabe gelesen und iteriert wird.
* **Auto-merge**: Wenn aktiviert, führt Claude den PR zusammen, sobald alle Prüfungen bestanden sind. Die Merge-Methode ist Squash. Das Auto-merge muss [in Ihren GitHub-Repository-Einstellungen aktiviert sein](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-auto-merge-for-pull-requests-in-your-repository), damit dies funktioniert.

Verwenden Sie die Umschalter **Auto-fix** und **Auto-merge** in der CI-Statusleiste, um eine der beiden Optionen zu aktivieren. Claude Code sendet auch eine Desktop-Benachrichtigung, wenn CI abgeschlossen ist. Um die Sitzung automatisch zu archivieren, sobald der PR zusammengeführt oder geschlossen wird, schalten Sie [auto-archive](#work-in-parallel-with-sessions) in Einstellungen → Claude Code ein.

<Note>
  Die PR-Überwachung erfordert, dass die [GitHub CLI (`gh`)](https://cli.github.com/) auf Ihrem Computer installiert und authentifiziert ist. Wenn `gh` nicht installiert ist, fordert Desktop Sie auf, es beim ersten Versuch, einen PR zu erstellen, zu installieren.
</Note>

<h2 id="arrange-your-workspace">
  Anordnen Ihres Arbeitsbereichs
</h2>

Die Code-Registerkarte ist um Panes aufgebaut, die Sie in jedem Layout anordnen können: Chat, Diff, Browser, Terminal, Datei, Plan, Aufgaben und Subagent. Ziehen Sie ein Pane an seiner Kopfzeile, um es zu verschieben, oder ziehen Sie eine Pane-Kante, um es zu vergrößern. Drücken Sie **Cmd+\\** auf macOS oder **Ctrl+\\** auf Windows, um das fokussierte Pane zu schließen. Öffnen Sie zusätzliche Panes aus dem Menü **Ansichten** in der Sitzungs-Symbolleiste.

<Note>
  Das Pane-Layout, Terminal, Datei-Editor und Ansichtsmodi in diesem Abschnitt erfordern Claude Desktop v1.2581.0 oder später. Öffnen Sie **Claude → Nach Updates suchen** auf macOS oder **Hilfe → Nach Updates suchen** auf Windows, um zu aktualisieren.
</Note>

<h3 id="run-commands-in-the-terminal">
  Führen Sie Befehle im Terminal aus
</h3>

Das integrierte Terminal ermöglicht es Ihnen, Befehle neben Ihrer Sitzung auszuführen, ohne zu einer anderen App zu wechseln. Öffnen Sie es aus dem Menü **Ansichten** oder drücken Sie **Ctrl+\`** auf macOS oder Windows. Das Terminal öffnet sich im Arbeitsverzeichnis Ihrer Sitzung und teilt die gleiche Umgebung wie Claude, sodass Befehle wie `npm test` oder `git status` die gleichen Dateien sehen, die Claude bearbeitet. Um eine zweite Terminal-Registerkarte zu öffnen, klicken Sie auf **+** in der Terminal-Pane-Kopfzeile oder klicken Sie mit der rechten Maustaste auf einen Ordner im Chat, um **Im Terminal öffnen** zu wählen. Das Terminal ist nur in lokalen Sitzungen verfügbar.

<h3 id="open-and-edit-files">
  Öffnen und bearbeiten Sie Dateien
</h3>

Klicken Sie auf einen Dateipfad im Chat oder Diff-Viewer, um ihn im Datei-Pane zu öffnen. HTML-, PDF-, Bild- und Videopfade öffnen sich stattdessen im [Browser-Pane](#preview-your-app). Nehmen Sie Spot-Bearbeitungen vor und klicken Sie auf **Speichern**, um sie zurückzuschreiben. Wenn sich die Datei auf der Festplatte geändert hat, seit Sie sie geöffnet haben, warnt Sie das Pane und lässt Sie überschreiben oder verwerfen. Klicken Sie auf **Verwerfen**, um Ihre Bearbeitungen rückgängig zu machen, oder klicken Sie auf den Pfad in der Pane-Kopfzeile, um den absoluten Pfad zu kopieren.

Das Datei-Pane ist in lokalen und SSH-Sitzungen verfügbar. Für Cloud-Sitzungen bitten Sie Claude, die Änderung vorzunehmen.

<h3 id="open-files-in-other-apps">
  Öffnen Sie Dateien in anderen Apps
</h3>

Klicken Sie mit der rechten Maustaste auf einen Dateipfad im Chat, Diff-Viewer oder Datei-Pane, um ein Kontextmenü zu öffnen:

* **Als Kontext anhängen**: Fügen Sie die Datei zu Ihrer nächsten Eingabe hinzu
* **Öffnen in**: Öffnen Sie die Datei in einem installierten Editor wie VS Code, Cursor oder Zed
* **Im Finder anzeigen** auf macOS, **Im Explorer anzeigen** auf Windows: Öffnen Sie den enthaltenden Ordner
* **Pfad kopieren**: Kopieren Sie den absoluten Pfad in Ihre Zwischenablage

<h3 id="switch-view-modes">
  Wechseln Sie Ansichtsmodi
</h3>

Ansichtsmodi kontrollieren, wie viel Detail im Chat-Transkript angezeigt wird. Wechseln Sie Modi aus dem Dropdown **Transkript-Ansicht** neben der Schaltfläche „Senden", oder drücken Sie **Ctrl+O** auf macOS oder Windows, um durch sie zu zyklisieren.

| Modus               | Was es anzeigt                                                                               |
| ------------------- | -------------------------------------------------------------------------------------------- |
| **Normal**          | Tool-Aufrufe in Zusammenfassungen zusammengefasst, mit vollständigen Text-Antworten          |
| **Ausführlich**     | Jeden Tool-Aufruf, jede Datei-Leseoperation und jeden Zwischenschritt, den Claude unternimmt |
| **Zusammenfassung** | Nur Claudes endgültige Antworten und die Änderungen, die er vorgenommen hat                  |

Verwenden Sie „Ausführlich", wenn Sie debuggen, warum Claude eine bestimmte Aktion unternommen hat. Verwenden Sie „Zusammenfassung", wenn Sie mehrere Sitzungen ausführen und Ergebnisse schnell scannen möchten.

<h3 id="keyboard-shortcuts">
  Tastaturkürzel
</h3>

Drücken Sie **Cmd+/** auf macOS oder **Ctrl+/** auf Windows, um alle im Code-Tab verfügbaren Kürzel zu sehen. Unter Windows verwenden Sie **Ctrl** anstelle von **Cmd** für die folgenden Kürzel. Sitzungs-Zyklisierung, Terminal-Umschalter und Ansichtsmodus-Umschalter verwenden **Ctrl** auf jeder Plattform.

| Kürzel                                | Aktion                                  |
| ------------------------------------- | --------------------------------------- |
| `Cmd` `/`                             | Tastaturkürzel anzeigen                 |
| `Cmd` `N`                             | Neue Sitzung                            |
| `Cmd` `W`                             | Sitzung schließen                       |
| `Ctrl` `Tab` / `Ctrl` `Shift` `Tab`   | Nächste oder vorherige Sitzung          |
| `Cmd` `Shift` `]` / `Cmd` `Shift` `[` | Nächste oder vorherige Sitzung          |
| `Esc`                                 | Claudes Antwort stoppen                 |
| `Cmd` `Shift` `D`                     | Diff-Pane umschalten                    |
| `Cmd` `Shift` `B`                     | Browser-Pane umschalten                 |
| `Cmd` `Shift` `S`                     | Element im Browser auswählen            |
| `Ctrl` `` ` ``                        | Terminal-Pane umschalten                |
| `Cmd` `\`                             | Fokussiertes Pane schließen             |
| `Cmd` `;`                             | Seitenchat öffnen                       |
| `Ctrl` `O`                            | Ansichtsmodi zyklisieren                |
| `Cmd` `Shift` `M`                     | Berechtigungsmodus-Menü öffnen          |
| `Cmd` `Shift` `I`                     | Modell-Menü öffnen                      |
| `Cmd` `Shift` `E`                     | Aufwand-Menü öffnen                     |
| `1`–`9`                               | Element in einem offenen Menü auswählen |

Diese Kürzel gelten nur für den Code-Tab. Die Terminal-basierten [Kürzel des interaktiven Modus](/docs/de/interactive-mode#keyboard-shortcuts), wie `Shift+Tab` zum Zyklisieren von Modi, gelten nicht in Desktop.

<h3 id="check-usage">
  Überprüfen Sie die Nutzung
</h3>

Klicken Sie auf den Nutzungsring neben dem Modell-Wahlschalter, um Ihre aktuelle Kontextfenster-Nutzung und Ihre Plan-Nutzung für den Zeitraum zu sehen. Die Kontext-Nutzung ist pro Sitzung; die Plan-Nutzung wird über alle Ihre Claude-Code-Oberflächen hinweg geteilt.

<h2 id="let-claude-use-your-computer">
  Lassen Sie Claude Ihren Computer verwenden
</h2>

Computernutzung ermöglicht es Claude, Ihre Apps zu öffnen, Ihren Bildschirm zu steuern und direkt auf Ihrem Computer zu arbeiten, wie Sie es tun würden. Bitten Sie Claude, eine native App im mobilen Simulator zu testen, mit einem Desktop-Tool zu interagieren, das keine CLI hat, oder etwas zu automatisieren, das nur über eine GUI funktioniert.

<Note>
  Computernutzung ist eine Forschungsvorschau auf macOS und Windows, die einen Pro- oder Max-Plan erfordert. Sie ist nicht für Team- oder Enterprise-Pläne verfügbar. Die Claude Desktop-App muss ausgeführt werden.
</Note>

Computernutzung ist standardmäßig deaktiviert. [Aktivieren Sie sie in Einstellungen](#enable-computer-use), bevor Claude Ihren Bildschirm steuern kann. Auf macOS müssen Sie auch Barrierefreiheits- und Bildschirmaufzeichnungsberechtigungen gewähren.

<Warning>
  Im Gegensatz zum [sandboxierten Bash-Tool](/docs/de/sandboxing) läuft Computernutzung auf Ihrem tatsächlichen Desktop mit Zugriff auf das, was Sie genehmigen. Claude überprüft jede Aktion und kennzeichnet potenzielle Prompt-Injection von Bildschirminhalten, aber die Vertrauensgrenze ist unterschiedlich. Siehe den [Sicherheitsleitfaden für Computernutzung](https://support.claude.com/en/articles/14128542) für Best Practices.
</Warning>

<h3 id="when-computer-use-applies">
  Wann Computernutzung anwendbar ist
</h3>

Claude hat mehrere Möglichkeiten, mit einer App oder einem Dienst zu interagieren, und Computernutzung ist die breiteste und langsamste. Es versucht zuerst das präziseste Tool:

* Wenn Sie einen [Konnektor](#connect-external-tools) für einen Dienst haben, verwendet Claude den Konnektor.
* Wenn die Aufgabe ein Shell-Befehl ist, verwendet Claude Bash.
* Wenn die Aufgabe Browser-Arbeit ist und Sie [Claude in Chrome](/docs/de/chrome) eingerichtet haben, verwendet Claude das.
* Wenn keine dieser Optionen zutrifft, verwendet Claude Computernutzung.

Die [Pro-App-Zugriffsstufen](#app-permissions) verstärken dies: Browser sind auf Nur-Ansicht begrenzt, und Terminals und IDEs auf Nur-Klick, was Claude zum dedizierten Tool lenkt, auch wenn Computernutzung aktiv ist. Bildschirmsteuerung ist für Dinge reserviert, die nichts anderes erreichen kann, wie native Apps, Hardware-Steuerfelder, mobile Simulatoren oder proprietäre Tools ohne API.

<h3 id="enable-computer-use">
  Aktivieren Sie Computernutzung
</h3>

Computernutzung ist standardmäßig deaktiviert. Wenn Sie Claude bitten, etwas zu tun, das es benötigt, während es deaktiviert ist, teilt Claude Ihnen mit, dass es die Aufgabe tun könnte, wenn Sie Computernutzung in Einstellungen aktivieren.

<Steps>
  <Step title="Aktualisieren Sie die Desktop-App">
    Stellen Sie sicher, dass Sie die neueste Version von Claude Desktop haben. Auf macOS und Windows laden Sie herunter oder aktualisieren Sie unter [claude.com/download](https://claude.com/download); unter Linux aktualisieren Sie über Ihren Paketmanager ([Anweisungen](/docs/de/desktop-linux)). Starten Sie dann die App neu.
  </Step>

  <Step title="Schalten Sie den Umschalter ein">
    Gehen Sie in der Desktop-App zu **Einstellungen > Allgemein** (unter **Desktop-App**). Suchen Sie den Umschalter **Computernutzung** und schalten Sie ihn ein. Unter Windows wird der Umschalter sofort wirksam und das Setup ist abgeschlossen. Auf macOS fahren Sie mit dem nächsten Schritt fort.

    Wenn Sie den Umschalter nicht sehen, bestätigen Sie, dass Sie macOS oder Windows mit einem Pro- oder Max-Plan verwenden, und aktualisieren und starten Sie die App neu.
  </Step>

  <Step title="Gewähren Sie macOS-Berechtigungen">
    Auf macOS müssen Sie zwei Systemberechtigungen gewähren, bevor der Umschalter wirksam wird:

    * **Barrierefreiheit**: ermöglicht Claude, zu klicken, zu tippen und zu scrollen
    * **Bildschirmaufzeichnung**: ermöglicht Claude, zu sehen, was auf Ihrem Bildschirm ist

    Die Einstellungsseite zeigt den aktuellen Status jeder Berechtigung. Wenn eine verweigert wird, klicken Sie auf das Badge, um den relevanten Systemeinstellungsbereich zu öffnen.
  </Step>
</Steps>

<h3 id="app-permissions">
  App-Berechtigungen
</h3>

Wenn Claude eine App zum ersten Mal verwenden muss, wird eine Eingabeaufforderung in Ihrer Sitzung angezeigt. Klicken Sie auf **Für diese Sitzung zulassen** oder **Ablehnen**. Genehmigungen gelten für die aktuelle Sitzung oder 30 Minuten in [Dispatch-generierten Sitzungen](#sessions-from-dispatch).

Die Eingabeaufforderung zeigt auch, welche Kontrollebene Claude für diese App erhält. Diese Stufen sind nach App-Kategorie festgelegt und können nicht geändert werden:

| Stufe                  | Was Claude tun kann                                                        | Gilt für                    |
| :--------------------- | :------------------------------------------------------------------------- | :-------------------------- |
| Nur Ansicht            | Die App in Screenshots sehen                                               | Browser, Handelsplattformen |
| Nur Klick              | Klicken und scrollen, aber nicht tippen oder Tastenkombinationen verwenden | Terminals, IDEs             |
| Vollständige Kontrolle | Klicken, tippen, ziehen und Tastenkombinationen verwenden                  | Alles andere                |

Apps mit großer Reichweite wie Terminals, Finder oder Datei-Explorer und Systemeinstellungen oder Einstellungen zeigen eine zusätzliche Warnung in der Eingabeaufforderung, damit Sie wissen, was das Genehmigen gewährt.

Sie können zwei Einstellungen in **Einstellungen > Allgemein** (unter **Desktop-App**) konfigurieren:

* **Abgelehnte Apps**: Fügen Sie Apps hier hinzu, um sie ohne Aufforderung abzulehnen. Claude kann eine abgelehnte App indirekt durch Aktionen in einer zulässigen App beeinflussen, kann aber nicht direkt mit der abgelehnen App interagieren.
* **Apps anzeigen, wenn Claude fertig ist**: Während Claude arbeitet, werden Ihre anderen Fenster ausgeblendet, damit es nur mit der genehmigten App interagiert. Wenn Claude fertig ist, werden ausgeblendete Fenster wiederhergestellt, es sei denn, Sie deaktivieren diese Einstellung.

<h2 id="manage-sessions">
  Verwalten Sie Sitzungen
</h2>

Jede Sitzung ist ein unabhängiges Gespräch mit eigenem Kontext und Änderungen. Sie können mehrere Sitzungen parallel ausführen, Arbeit in die Cloud senden oder Dispatch Sitzungen von Ihrem Telefon aus starten lassen.

<h3 id="work-in-parallel-with-sessions">
  Arbeiten Sie parallel mit Sitzungen
</h3>

Klicken Sie auf **+ Neue Sitzung** in der Seitenleiste, oder drücken Sie **Cmd+N** auf macOS oder **Ctrl+N** auf Windows, um an mehreren Aufgaben parallel zu arbeiten. Drücken Sie **Ctrl+Tab** und **Ctrl+Shift+Tab**, um durch Sitzungen in der Seitenleiste zu zyklisieren. Für Git-Repositories erhält jede Sitzung ihre eigene isolierte Kopie Ihres Projekts mit [Git Worktrees](/docs/de/worktrees), sodass Änderungen in einer Sitzung andere Sitzungen nicht beeinflussen, bis Sie sie committen.

Um zwei Sitzungen gleichzeitig anzuzeigen, halten Sie **Cmd** auf macOS oder **Ctrl** auf Windows gedrückt und klicken Sie auf eine Sitzung in der Seitenleiste. Die Sitzung wird in einem zweiten Bereich neben dem bereits geöffneten angezeigt. Während die Aufteilung aktiv ist, ersetzt das Klicken auf eine andere Sitzung in der Seitenleiste denjenigen Bereich, der den Fokus hat. Drücken Sie **Cmd+\\** auf macOS oder **Ctrl+\\** auf Windows, um den fokussierten Bereich zu schließen und zu einer einzelnen Sitzung zurückzukehren.

Worktrees werden standardmäßig in `<project-root>/.claude/worktrees/` gespeichert. Sie können dies in Einstellungen → Claude Code unter 'Worktree-Speicherort" in ein benutzerdefiniertes Verzeichnis ändern. Sie können auch ein Branch-Präfix festlegen, das jedem Worktree-Branch-Namen vorangestellt wird, was nützlich ist, um von Claude erstellte Branches organisiert zu halten. Um einen Worktree zu entfernen, wenn Sie fertig sind, fahren Sie mit der Maus über die Sitzung in der Seitenleiste und klicken Sie auf das Archiv-Symbol. Um Sitzungen automatisch zu archivieren, wenn ihr Pull Request zusammengeführt oder geschlossen wird, schalten Sie **Auto-Archivieren nach PR-Merge oder -Schließung** in Einstellungen → Claude Code ein. Auto-Archivieren gilt nur für lokale Sitzungen, die beendet wurden.

Um gitignorierte Dateien wie `.env` in neue Worktrees einzubeziehen, erstellen Sie eine [`.worktreeinclude`-Datei](/docs/de/worktrees#copy-gitignored-files-into-worktrees) in Ihrem Projektstammverzeichnis.

<Note>
  Die Sitzungsisolation erfordert [Git](https://git-scm.com/downloads). Die meisten Macs enthalten Git standardmäßig. Führen Sie `git --version` im Terminal aus, um zu überprüfen. Unter Windows ist Git erforderlich, damit die Registerkarte „Code" funktioniert: [Laden Sie Git für Windows herunter](https://git-scm.com/downloads/win), installieren Sie es und starten Sie die App neu. Wenn Sie auf Git-Fehler stoßen, bitten Sie Claude im [Cowork-Tab](https://claude.com/product/cowork), Ihnen bei der Behebung Ihres Setups zu helfen.
</Note>

Verwenden Sie die Steuerelemente oben in der Seitenleiste, um Sitzungen nach Status, Projekt oder Umgebung zu filtern, und um Sitzungen nach Projekt zu gruppieren. Um eine Sitzung umzubenennen, klicken Sie auf den Sitzungstitel in der Symbolleiste oben in der aktiven Sitzung. Um die Kontext-Nutzung zu überprüfen, siehe [Überprüfen Sie die Nutzung](#check-usage). Wenn der Kontext voll wird, fasst Claude das Gespräch automatisch zusammen und arbeitet weiter. Sie können auch `/compact` eingeben, um die Zusammenfassung früher auszulösen und Kontextraum freizugeben. Siehe [das Kontextfenster](/docs/de/how-claude-code-works#the-context-window) für Details, wie die Komprimierung funktioniert.

Die Desktop-App sendet eine Betriebssystem-Benachrichtigung, wenn eine Code-Sitzung eine Aufgabe abschließt und Sie diese Sitzung gerade nicht anzeigen.

<h3 id="ask-a-side-question-without-derailing-the-session">
  Fragen Sie eine Seitenfrage, ohne die Sitzung zu entgleisen
</h3>

Ein Seitenchat ermöglicht es Ihnen, Claude eine Frage zu stellen, die den Kontext Ihrer Sitzung nutzt, aber nichts zum Hauptgespräch hinzufügt. Verwenden Sie ihn, wenn Sie ein Stück Code verstehen, eine Annahme überprüfen oder eine Idee erkunden möchten, ohne die Sitzung vom Kurs abzubringen.

Drücken Sie **Cmd+;** auf macOS oder **Ctrl+;** auf Windows, um einen Seitenchat zu öffnen, oder geben Sie `/btw` im Eingabefeld ein. Der Seitenchat kann alles im Hauptthread bis zu diesem Punkt lesen. Wenn Sie fertig sind, schließen Sie den Seitenchat und setzen Sie die Hauptsitzung dort fort, wo Sie aufgehört haben. Seitenchats sind in lokalen, SSH- und WSL-Sitzungen verfügbar.

<h3 id="watch-background-tasks">
  Beobachten Sie Hintergrund-Aufgaben
</h3>

Das Aufgaben-Pane zeigt die Hintergrundarbeit, die in der aktuellen Sitzung läuft: Subagents, Hintergrund-Shell-Befehle und [dynamische Workflows](/docs/de/workflows). Öffnen Sie es aus dem Menü **Ansichten** oder ziehen Sie es in Ihr Layout.

Klicken Sie auf einen beliebigen Eintrag, um seine Ausgabe im Subagent-Pane zu sehen oder ihn zu stoppen. Um zu sehen, was andere Sitzungen tun, verwenden Sie die [Seitenleiste](#work-in-parallel-with-sessions).

<h3 id="run-long-running-tasks-remotely">
  Führen Sie lange laufende Aufgaben remote aus
</h3>

Für große Refaktorierungen, Test-Suites, Migrationen oder andere lange laufende Aufgaben wählen Sie **Remote** statt **Lokal**, wenn Sie eine Sitzung starten. Cloud-Sitzungen laufen auf Anthropics Cloud-Infrastruktur und werden fortgesetzt, auch wenn Sie die App schließen oder Ihren Computer herunterfahren. Überprüfen Sie jederzeit den Fortschritt oder lenken Sie Claude in eine andere Richtung. Sie können Cloud-Sitzungen auch von [claude.ai/code](https://claude.ai/code) oder der Claude iOS-App aus überwachen.

Cloud-Sitzungen unterstützen auch mehrere Repositories. Nach Auswahl einer Cloud-Umgebung klicken Sie auf die Schaltfläche **+** neben dem Repo-Pill, um zusätzliche Repositories zur Sitzung hinzuzufügen. Jedes Repo erhält seinen eigenen Branch-Wahlschalter. Dies ist nützlich für Aufgaben, die mehrere Codebases umfassen, z. B. das Aktualisieren einer gemeinsamen Bibliothek und ihrer Consumer.

Siehe [Claude Code im Web](/docs/de/claude-code-on-the-web) für mehr darüber, wie Cloud-Sitzungen funktionieren.

<h3 id="continue-in-another-surface">
  Fortsetzen auf einer anderen Oberfläche
</h3>

Das Menü **Fortsetzen in**, das über das VS Code-Symbol unten rechts in der Sitzungs-Symbolleiste zugänglich ist, ermöglicht es Ihnen, Ihre Sitzung auf eine andere Oberfläche zu verschieben:

* **Claude Code im Web**: sendet Ihre lokale Sitzung, um remote weiter zu laufen. Desktop pusht Ihren Branch, generiert eine Zusammenfassung des Gesprächs und erstellt eine neue Cloud-Sitzung mit dem vollständigen Kontext. Sie können dann wählen, die lokale Sitzung zu archivieren oder zu behalten. Dies erfordert einen sauberen Arbeitsbaum und ist nicht für SSH-Sitzungen verfügbar.
* **Ihre IDE**: öffnet Ihr Projekt in einer unterstützten IDE im aktuellen Arbeitsverzeichnis.

<h3 id="sessions-from-dispatch">
  Sitzungen von Dispatch
</h3>

[Dispatch](https://support.claude.com/en/articles/13947068) ist ein persistentes Gespräch mit Claude, das in der Registerkarte [Cowork](https://claude.com/product/cowork) lebt. Sie senden Dispatch eine Aufgabe, und es entscheidet, wie damit umzugehen ist.

Eine Aufgabe kann auf zwei Wegen als Code-Sitzung enden: Sie fragen direkt danach, z. B. „Öffnen Sie eine Claude Code-Sitzung und beheben Sie den Login-Fehler", oder Dispatch entscheidet, dass die Aufgabe Entwicklungsarbeit ist und startet eine von selbst. Aufgaben, die typischerweise zu Code führen, umfassen das Beheben von Fehlern, das Aktualisieren von Abhängigkeiten, das Ausführen von Tests oder das Öffnen von Pull Requests. Forschung, Dokumentbearbeitung und Tabellenkalkulationsarbeit bleiben in Cowork.

In jedem Fall wird die Code-Sitzung in der Seitenleiste der Registerkarte „Code" mit einem **Dispatch**-Badge angezeigt. Sie erhalten eine Push-Benachrichtigung auf Ihrem Telefon, wenn sie fertig ist oder Ihre Genehmigung benötigt.

Wenn Sie [Computernutzung](#let-claude-use-your-computer) aktiviert haben, können Dispatch-generierte Code-Sitzungen diese auch verwenden. App-Genehmigungen in diesen Sitzungen verfallen nach 30 Minuten und werden erneut angefordert, anstatt die gesamte Sitzung zu dauern wie bei regulären Code-Sitzungen.

Für Setup, Pairing und Dispatch-Einstellungen siehe den [Dispatch-Hilfeartikel](https://support.claude.com/en/articles/13947068). Dispatch erfordert einen Pro- oder Max-Plan und ist nicht für Team- oder Enterprise-Pläne verfügbar.

Dispatch ist eine von mehreren Möglichkeiten, mit Claude zu arbeiten, wenn Sie weg von Ihrem Terminal sind. Siehe [Plattformen und Integrationen](/docs/de/platforms#work-when-you-are-away-from-your-terminal), um es mit Remote Control, Channels, Slack und geplanten Aufgaben zu vergleichen.

<h2 id="extend-claude-code">
  Erweitern Sie Claude Code
</h2>

Verbinden Sie externe Dienste, fügen Sie wiederverwendbare Workflows hinzu, passen Sie Claudes Verhalten an und konfigurieren Sie Vorschau-Server. Um Konnektoren, Skills und Plugins an einem Ort zu verwalten, klicken Sie auf **Anpassen** in der Seitenleiste.

<h3 id="connect-external-tools">
  Verbinden Sie externe Tools
</h3>

Für lokale und [SSH](#ssh-sessions)-Sitzungen klicken Sie auf die Schaltfläche **+** neben dem Eingabefeld und wählen Sie **Konnektoren**, um Integrationen wie Google Calendar, Slack, GitHub, Linear, Notion und mehr hinzuzufügen. Sie können Konnektoren vor oder während einer Sitzung hinzufügen. Die Schaltfläche **+** ist nicht in Cloud-Sitzungen verfügbar, aber [Routinen](/docs/de/routines) konfigurieren Konnektoren zum Zeitpunkt der Routine-Erstellung.

Um Konnektoren zu verwalten oder zu trennen, gehen Sie zu Einstellungen → Konnektoren in der Desktop-App oder wählen Sie **Konnektoren verwalten** aus dem Konnektoren-Menü im Eingabefeld.

Nach der Verbindung kann Claude Ihren Kalender lesen, Nachrichten senden, Probleme erstellen und direkt mit Ihren Tools interagieren. Sie können Claude fragen, welche Konnektoren in Ihrer Sitzung konfiguriert sind.

Konnektoren sind [MCP-Server](/docs/de/mcp) mit einem grafischen Setup-Ablauf. Verwenden Sie sie für schnelle Integration mit unterstützten Diensten. Für Integrationen, die nicht in Konnektoren aufgelistet sind, fügen Sie MCP-Server manuell über [Einstellungsdateien](/docs/de/mcp#installing-mcp-servers) hinzu. Sie können auch [benutzerdefinierte Konnektoren erstellen](https://support.claude.com/en/articles/11175166-getting-started-with-custom-connectors-using-remote-mcp).

<h3 id="use-skills">
  Verwenden Sie Skills
</h3>

[Skills](/docs/de/skills) erweitern, was Claude tun kann. Claude lädt sie automatisch, wenn relevant, oder Sie können eine direkt aufrufen: Geben Sie `/` im Eingabefeld ein oder klicken Sie auf die Schaltfläche **+** und wählen Sie **Slash-Befehle**, um zu sehen, was verfügbar ist. Dies umfasst [integrierte Befehle](/docs/de/commands), Ihre [benutzerdefinierten Skills](/docs/de/skills#create-your-first-skill), Projekt-Skills aus Ihrer Codebasis und Skills aus allen [installierten Plugins](/docs/de/plugins). Wählen Sie einen aus und er wird im Eingabefeld hervorgehoben angezeigt. Geben Sie Ihre Aufgabe danach ein und senden Sie wie gewohnt.

Sie können einen Befehl senden, während Claude arbeitet, genauso wie jede andere Nachricht, und die Sitzung kehrt zum Leerlauf zurück, sobald der Zug beendet ist. Vor v2.1.206 konnte ein Befehl, der während eines Zuges gesendet wurde, dazu führen, dass die Sitzung weiterhin als laufend angezeigt wurde und Nachrichten, die Sie danach sendeten, nicht zugestellt wurden.

<h3 id="install-plugins">
  Installieren Sie Plugins
</h3>

[Plugins](/docs/de/plugins) sind wiederverwendbare Pakete, die Skills, Agents, hooks, MCP-Server und LSP-Konfigurationen zu Claude Code hinzufügen. Sie können Plugins aus der Desktop-App installieren, ohne das Terminal zu verwenden.

Für lokale und [SSH](#ssh-sessions)-Sitzungen klicken Sie auf die Schaltfläche **+** neben dem Eingabefeld und wählen Sie **Plugins**, um Ihre installierten Plugins und deren Skills zu sehen. Um ein Plugin hinzuzufügen, wählen Sie **Plugin hinzufügen** aus dem Untermenü, um den Plugin-Browser zu öffnen, der verfügbare Plugins aus Ihren konfigurierten [Marketplaces](/docs/de/plugin-marketplaces) einschließlich des offiziellen Anthropic-Marketplace anzeigt. Wählen Sie **Plugins verwalten**, um Plugins zu aktivieren, zu deaktivieren oder zu deinstallieren.

Plugins können auf Ihr Benutzerkonto, ein bestimmtes Projekt oder nur lokal beschränkt sein. Wenn Ihre Organisation Plugins zentral verwaltet, sind diese Plugins in Desktop-Sitzungen auf die gleiche Weise verfügbar wie in der CLI. Plugins sind nicht für Cloud-Sitzungen verfügbar. Für die vollständige Plugin-Referenz einschließlich der Erstellung eigener Plugins siehe [Plugins](/docs/de/plugins).

<h3 id="configure-preview-servers">
  Konfigurieren Sie Vorschau-Server
</h3>

Claude erkennt automatisch Ihr Dev-Server-Setup und speichert die Konfiguration in `.claude/launch.json` im Stammverzeichnis des Ordners, den Sie beim Starten der Sitzung ausgewählt haben. Die Vorschau verwendet diesen Ordner als Arbeitsverzeichnis. Wenn Sie also einen übergeordneten Ordner ausgewählt haben, werden Unterordner mit ihren eigenen Dev-Servern nicht automatisch erkannt. Um mit dem Server eines Unterordners zu arbeiten, starten Sie entweder eine Sitzung direkt in diesem Ordner oder fügen Sie eine Konfiguration manuell hinzu.

Um anzupassen, wie Ihr Server startet, z. B. um `yarn dev` statt `npm run dev` zu verwenden oder den Port zu ändern, bearbeiten Sie die Datei manuell oder klicken Sie auf **Konfiguration bearbeiten** im Dropdown „Vorschau", um sie in Ihrem Code-Editor zu öffnen. Die Datei unterstützt JSON mit Kommentaren.

```json theme={null}
{
  "version": "0.0.1",
  "configurations": [
    {
      "name": "my-app",
      "runtimeExecutable": "npm",
      "runtimeArgs": ["run", "dev"],
      "port": 3000
    }
  ]
}
```

Sie können mehrere Konfigurationen definieren, um verschiedene Server aus demselben Projekt auszuführen, z. B. ein Frontend und eine API. Siehe die [Beispiele](#examples) unten.

<h4 id="auto-verify-changes">
  Automatische Überprüfung von Änderungen
</h4>

Wenn `autoVerify` aktiviert ist, überprüft Claude automatisch Code-Änderungen nach dem Bearbeiten von Dateien. Es macht Screenshots, prüft auf Fehler und bestätigt, dass Änderungen funktionieren, bevor es seine Antwort abschließt.

Die automatische Überprüfung ist standardmäßig aktiviert. Deaktivieren Sie sie pro Projekt, indem Sie `"autoVerify": false` zu `.claude/launch.json` hinzufügen, oder schalten Sie sie aus dem Dropdown „Vorschau" um.

```json theme={null}
{
  "version": "0.0.1",
  "autoVerify": false,
  "configurations": [...]
}
```

Wenn deaktiviert, sind Vorschau-Tools immer noch verfügbar und Sie können Claude jederzeit bitten, zu überprüfen. Die automatische Überprüfung macht es automatisch nach jeder Bearbeitung.

<h4 id="configuration-fields">
  Konfigurationsfelder
</h4>

Jeder Eintrag im Array `configurations` akzeptiert die folgenden Felder:

| Feld                | Typ       | Beschreibung                                                                                                                                                                                                                                                                                                    |
| ------------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`              | string    | Ein eindeutiger Bezeichner für diesen Server                                                                                                                                                                                                                                                                    |
| `runtimeExecutable` | string    | Der auszuführende Befehl, z. B. `npm`, `yarn` oder `node`                                                                                                                                                                                                                                                       |
| `runtimeArgs`       | string\[] | An `runtimeExecutable` übergebene Argumente, z. B. `["run", "dev"]`                                                                                                                                                                                                                                             |
| `port`              | number    | Der Port, auf dem Ihr Server lauscht. Standardmäßig 3000                                                                                                                                                                                                                                                        |
| `cwd`               | string    | Arbeitsverzeichnis relativ zu Ihrem Projektstammverzeichnis. Standardmäßig das Projektstammverzeichnis. Verwenden Sie `${workspaceFolder}`, um das Projektstammverzeichnis explizit zu referenzieren                                                                                                            |
| `env`               | object    | Zusätzliche Umgebungsvariablen als Schlüssel-Wert-Paare, z. B. `{ "NODE_ENV": "development" }`. Legen Sie hier keine Geheimnisse ab, da diese Datei in Ihr Repo committed wird. Um Geheimnisse an Ihren Dev-Server zu übergeben, legen Sie sie stattdessen im [lokalen Umgebungs-Editor](#local-sessions) fest. |
| `autoPort`          | boolean   | Wie Port-Konflikte behandelt werden. Siehe unten                                                                                                                                                                                                                                                                |
| `program`           | string    | Ein mit `node` auszuführendes Skript. Siehe [wann `program` vs `runtimeExecutable` verwendet werden](#when-to-use-program-vs-runtimeexecutable)                                                                                                                                                                 |
| `args`              | string\[] | An `program` übergebene Argumente. Wird nur verwendet, wenn `program` gesetzt ist                                                                                                                                                                                                                               |

<a id="when-to-use-program-vs-runtimeexecutable" />

<h5 id="when-to-use-program-vs-runtimeexecutable">
  Wann `program` vs `runtimeExecutable` verwendet werden
</h5>

Verwenden Sie `runtimeExecutable` mit `runtimeArgs`, um einen Dev-Server über einen Package Manager zu starten. Zum Beispiel `"runtimeExecutable": "npm"` mit `"runtimeArgs": ["run", "dev"]` führt `npm run dev` aus.

Verwenden Sie `program`, wenn Sie ein eigenständiges Skript haben, das Sie direkt mit `node` ausführen möchten. Zum Beispiel `"program": "server.js"` führt `node server.js` aus. Übergeben Sie zusätzliche Flags mit `args`.

<h4 id="port-conflicts">
  Port-Konflikte
</h4>

Das Feld `autoPort` kontrolliert, was passiert, wenn Ihr bevorzugter Port bereits verwendet wird:

* **`true`**: Claude findet und verwendet automatisch einen freien Port. Geeignet für die meisten Dev-Server.
* **`false`**: Claude schlägt mit einem Fehler fehl. Verwenden Sie dies, wenn Ihr Server einen bestimmten Port verwenden muss, z. B. für OAuth-Callbacks oder CORS-Allowlists.
* **Nicht gesetzt (Standard)**: Claude fragt, ob der Server diesen genauen Port benötigt, und speichert dann Ihre Antwort.

Wenn Claude einen anderen Port wählt, übergibt es den zugewiesenen Port an Ihren Server über die Umgebungsvariable `PORT`.

<h4 id="examples">
  Beispiele
</h4>

Diese Konfigurationen zeigen häufige Setups für verschiedene Projekttypen:

<Tabs>
  <Tab title="Next.js">
    Diese Konfiguration führt eine Next.js-App mit Yarn auf Port 3000 aus:

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "web",
          "runtimeExecutable": "yarn",
          "runtimeArgs": ["dev"],
          "port": 3000
        }
      ]
    }
    ```
  </Tab>

  <Tab title="Multiple servers">
    Für ein Monorepo mit einem Frontend und einem API-Server definieren Sie mehrere Konfigurationen. Das Frontend verwendet `autoPort: true`, sodass es einen freien Port wählt, wenn 3000 belegt ist, während der API-Server Port 8080 genau benötigt:

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "frontend",
          "runtimeExecutable": "npm",
          "runtimeArgs": ["run", "dev"],
          "cwd": "apps/web",
          "port": 3000,
          "autoPort": true
        },
        {
          "name": "api",
          "runtimeExecutable": "npm",
          "runtimeArgs": ["run", "start"],
          "cwd": "server",
          "port": 8080,
          "env": { "NODE_ENV": "development" },
          "autoPort": false
        }
      ]
    }
    ```
  </Tab>

  <Tab title="Node.js script">
    Um ein Node.js-Skript direkt auszuführen, statt einen Package-Manager-Befehl zu verwenden, verwenden Sie das Feld `program`:

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "server",
          "program": "server.js",
          "args": ["--verbose"],
          "port": 4000
        }
      ]
    }
    ```
  </Tab>
</Tabs>

<h2 id="environment-configuration">
  Umgebungskonfiguration
</h2>

Die Umgebung, die Sie beim [Starten einer Sitzung](#start-a-session) wählen, bestimmt, wo Claude ausgeführt wird und wie Sie sich verbinden:

* **Lokal**: läuft auf Ihrem Computer mit direktem Zugriff auf Ihre Dateien
* **Remote**: läuft auf Anthropics Cloud-Infrastruktur. Sitzungen werden fortgesetzt, auch wenn Sie die App schließen.
* **SSH**: läuft auf einem Remote-Computer, mit dem Sie sich über SSH verbinden, z. B. Ihre eigenen Server, Cloud-VMs oder Dev-Container
* **WSL** (Windows): läuft in einer [WSL 2-Distribution](/docs/de/desktop-wsl) auf Ihrem Computer und verwendet deren Linux-Toolchain und native Pfade

<h3 id="local-sessions">
  Lokale Sitzungen
</h3>

Die Desktop-App erbt nicht immer Ihre vollständige Shell-Umgebung. Auf macOS liest die App beim Starten aus dem Dock oder Finder Ihr Shell-Profil, z. B. `~/.zshrc` oder `~/.bashrc`, um `PATH` und einen festen Satz von Claude Code-Variablen zu extrahieren, aber andere Variablen, die Sie dort exportieren, werden nicht übernommen. Unter Windows erbt die App Benutzer- und Systemumgebungsvariablen, liest aber keine PowerShell-Profile.

Um Umgebungsvariablen für lokale Sitzungen und Dev-Server auf jeder Plattform festzulegen, öffnen Sie das Umgebungs-Dropdown im Eingabefeld, fahren Sie mit der Maus über **Lokal** und klicken Sie auf das Zahnrad-Symbol, um den lokalen Umgebungs-Editor zu öffnen. Variablen, die Sie hier speichern, werden verschlüsselt auf Ihrem Computer gespeichert und gelten für jede lokale Sitzung und jeden Vorschau-Server, den Sie starten. Sie können auch Variablen zum Schlüssel `env` in Ihrer Datei `~/.claude/settings.json` hinzufügen, obwohl diese nur Claude-Sitzungen erreichen und nicht Dev-Server. Siehe [Umgebungsvariablen](/docs/de/env-vars) für die vollständige Liste der unterstützten Variablen.

[Erweitertes Denken](/docs/de/model-config#extended-thinking) ist standardmäßig aktiviert, was die Leistung bei komplexen Denkaufgaben verbessert, aber zusätzliche Token verwendet. Um das Denken zu deaktivieren, setzen Sie `MAX_THINKING_TOKENS` auf `0` im lokalen Umgebungs-Editor; dies hat keine Auswirkung auf Fable 5, das immer erweitertes Denken verwendet. Bei [Drittanbieter-Providern](/docs/de/third-party-integrations) wird stattdessen der Parameter `thinking` weggelassen, und Adaptive-Reasoning-Modelle können dennoch denken. Bei Modellen mit [adaptiver Argumentation](/docs/de/model-config#adjust-effort-level) wird jeder andere `MAX_THINKING_TOKENS`-Wert ignoriert, da adaptive Argumentation die Denktiefe steuert. Bei Opus 4.6 und Sonnet 4.6 setzen Sie `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING` auf `1`, um ein festes Denk-Budget zu verwenden; Fable 5, Sonnet 5 und Opus 4.7 und später verwenden immer adaptive Argumentation und haben keinen Modus mit festem Budget.

<h3 id="cloud-sessions">
  Remote-Sitzungen
</h3>

Remote-Sitzungen werden im Hintergrund fortgesetzt, auch wenn Sie die App schließen. Die Nutzung wird auf Ihre [Abonnementplanlimits](/docs/de/costs) angerechnet, ohne separate Compute-Gebühren.

Sie können benutzerdefinierte Cloud-Umgebungen mit verschiedenen Netzwerkzugriffsstufen und Umgebungsvariablen erstellen. Wählen Sie das Umgebungs-Dropdown beim Starten einer Cloud-Sitzung und wählen Sie **Umgebung hinzufügen**. Siehe [die Cloud-Umgebung](/docs/de/claude-code-on-the-web#the-cloud-environment) für Details zur Konfiguration von Netzwerkzugriff und Umgebungsvariablen.

<h3 id="ssh-sessions">
  SSH-Sitzungen
</h3>

SSH-Sitzungen ermöglichen es Ihnen, Claude Code auf einem Remote-Computer auszuführen, während Sie die Desktop-App als Ihre Schnittstelle verwenden. Dies ist nützlich für die Arbeit mit Codebases, die auf Cloud-VMs, Dev-Containern oder Servern mit spezifischer Hardware oder Abhängigkeiten vorhanden sind.

Um eine SSH-Verbindung hinzuzufügen, klicken Sie auf das Umgebungs-Dropdown vor dem Starten einer Sitzung und wählen Sie **+ SSH-Verbindung hinzufügen**. Der Dialog fragt nach:

* **Name**: ein freundlicher Bezeichner für diese Verbindung
* **SSH-Host**: `user@hostname` oder ein in `~/.ssh/config` definierter Host
* **SSH-Port**: Standard ist 22, wenn leer gelassen, oder verwendet den Port aus Ihrer SSH-Konfiguration
* **Identity File**: Pfad zu Ihrem privaten Schlüssel, z. B. `~/.ssh/id_rsa`. Lassen Sie leer, um den Standardschlüssel oder Ihre SSH-Konfiguration zu verwenden.

Nach dem Hinzufügen wird die Verbindung im Umgebungs-Dropdown angezeigt. Wählen Sie sie aus, um eine Sitzung auf diesem Computer zu starten. Claude läuft auf dem Remote-Computer mit Zugriff auf seine Dateien und Tools.

Der Remote-Computer muss Linux oder macOS ausführen. Die Desktop-App installiert Claude Code auf dem Remote-Computer automatisch beim ersten Verbindungsaufbau. Nach der Verbindung unterstützen SSH-Sitzungen Berechtigungsmodi, Konnektoren, Plugins und MCP-Server.

<h4 id="pre-configure-ssh-connections-for-your-team">
  SSH-Verbindungen für Ihr Team vorkonfigurieren
</h4>

Administratoren können SSH-Verbindungen an Teammitglieder verteilen, indem sie `sshConfigs` zu einer [verwalteten Einstellungsdatei](/docs/de/settings#settings-precedence) hinzufügen. Auf diese Weise definierte Verbindungen werden in der Umgebungs-Dropdown-Liste jedes Benutzers automatisch angezeigt und sind als verwaltet gekennzeichnet, sodass Benutzer sie auswählen, aber nicht bearbeiten oder löschen können.

Das folgende Beispiel konfiguriert eine einzelne Verbindung vor, die sich in `~/projects` auf dem Remote-Host öffnet:

```json theme={null}
{
  "sshConfigs": [
    {
      "id": "shared-dev-vm",
      "name": "Shared Dev VM",
      "sshHost": "user@dev.example.com",
      "sshPort": 22,
      "sshIdentityFile": "~/.ssh/id_ed25519",
      "startDirectory": "~/projects"
    }
  ]
}
```

Jeder Eintrag erfordert `id`, `name` und `sshHost`. Die Felder `sshPort`, `sshIdentityFile` und `startDirectory` sind optional. Benutzer können auch `sshConfigs` zu ihrer eigenen `~/.claude/settings.json` hinzufügen, wo Verbindungen, die über den Dialog hinzugefügt werden, gespeichert sind.

<h4 id="restrict-which-ssh-hosts-users-can-connect-to">
  SSH-Hosts einschränken, mit denen Benutzer sich verbinden können
</h4>

Administratoren können Desktop-SSH-Sitzungen auf einen genehmigten Satz von Hosts beschränken, indem sie `sshHostAllowlist` zu einer [verwalteten Einstellungsdatei](/docs/de/settings#settings-precedence) hinzufügen. Wenn diese festgelegt ist, können Benutzer sich nur mit Hosts verbinden, deren aufgelöster Hostname einem der Muster entspricht. Setzen Sie es auf ein leeres Array, um SSH-Sitzungen vollständig zu deaktivieren.

Das folgende Beispiel erlaubt Verbindungen zu jedem Host unter `devboxes.example.com` und zu einem einzelnen benannten Bastion-Host:

```json theme={null}
{
  "sshHostAllowlist": ["*.devboxes.example.com", "bastion.example.com"]
}
```

Muster sind nicht case-sensitiv. `*` passt auf jeden Host, und `*.example.com` passt auf `example.com` und jede Subdomain. Alles andere ist eine exakte Übereinstimmung. Die Überprüfung wird gegen den Hostnamen nach `~/.ssh/config`-Auflösung über `ssh -G` durchgeführt, sodass `Host`-Aliase und `ProxyCommand`/`ProxyJump`-Einträge zulässig sind, solange der aufgelöste `HostName` passt.

`sshHostAllowlist` wird nur aus verwalteten Einstellungen gelesen; Werte in Benutzer- oder Projekteinstellungen werden ignoriert. Nur die Claude Desktop-App berücksichtigt diese Einstellung; die Claude Code CLI und IDE-Erweiterungen lesen sie nicht, und sie beschränkt keine `ssh`-Befehle, die über das Bash-Tool ausgeführt werden. Sie regelt, mit welchen Hosts sich die Desktop-App verbindet, nicht den Netzwerk-Egress, daher kombinieren Sie sie mit den Netzwerk- oder Zero-Trust-Kontrollen Ihrer Organisation, wenn Sie eine harte Grenze benötigen.

<h2 id="enterprise-configuration">
  Unternehmenskonfiguration
</h2>

Organisationen in Team- oder Enterprise-Plänen können das Verhalten der Desktop-App durch Admin-Konsolen-Steuerelemente, verwaltete Einstellungsdateien und Geräteverwaltungsrichtlinien verwalten.

<h3 id="admin-console-controls">
  Admin-Konsolen-Steuerelemente
</h3>

Diese Einstellungen werden über die [Admin-Einstellungskonsole](https://claude.ai/admin-settings/claude-code) konfiguriert:

* **Code in der Desktop**: Kontrollieren Sie, ob Benutzer in Ihrer Organisation auf Claude Code in der Desktop-App zugreifen können
* **Code im Web**: Aktivieren oder deaktivieren Sie [Web-Sitzungen](/docs/de/claude-code-on-the-web) für Ihre Organisation
* **Remote Control**: Aktivieren oder deaktivieren Sie [Remote Control](/docs/de/remote-control) für Ihre Organisation
* **Bypass-Berechtigungsmodus deaktivieren**: Verhindern Sie, dass Benutzer in Ihrer Organisation den Bypass-Berechtigungsmodus aktivieren

<h3 id="managed-settings">
  Verwaltete Einstellungen
</h3>

Verwaltete Einstellungen überschreiben Projekt- und Benutzereinstellungen und gelten für Claude-Code-Sitzungen in Desktop. Sie können diese Schlüssel in der [verwalteten Einstellungsdatei](/docs/de/settings#settings-precedence) Ihrer Organisation oder remote über die Admin-Konsole festlegen.

| Schlüssel                                  | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissions.disableBypassPermissionsMode` | auf `"disable"` setzen, um Benutzer daran zu hindern, den Bypass-Berechtigungsmodus zu aktivieren.                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `disableAutoMode`                          | auf `"disable"` setzen, um Benutzer daran zu hindern, den [Auto](/docs/de/permission-modes#eliminate-prompts-with-auto-mode)-Modus zu aktivieren. Entfernt Auto aus dem Moduswahlschalter. Auch unter `permissions` akzeptiert.                                                                                                                                                                                                                                                                                                                          |
| `autoMode`                                 | passen Sie an, was der Auto-Modus-Klassifizierer über Ihre Organisation vertraut und blockiert. Siehe [Auto-Modus konfigurieren](/docs/de/auto-mode-config).                                                                                                                                                                                                                                                                                                                                                                                             |
| `browserExternalPageTools`                 | auf `"disabled"` setzen, um zu verhindern, dass Claude Tools verwendet, um externe Seiten im [Browser-Bereich](#browse-external-sites) zu lesen oder zu bearbeiten. Benutzer können externe Websites weiterhin selbst navigieren, und lokale Dev-Server-Vorschauen sind nicht betroffen.                                                                                                                                                                                                                                                            |
| `disableBrowserExternalNavigation`         | auf `true` setzen, um externe Browsing im [Browser-Bereich](#browse-external-sites) vollständig auszuschalten. Weder Benutzer noch Claude können zu externen Websites navigieren, und localhost Dev-Server-Vorschauen sind nicht betroffen. Der Wert muss der JSON-Boolean `true` sein; der String `"true"` wird ignoriert.                                                                                                                                                                                                                         |
| `sshConfigs`                               | vorkonfigurieren Sie [SSH-Verbindungen](#pre-configure-ssh-connections-for-your-team), die in der Umgebungs-Dropdown angezeigt werden. Benutzer können verwaltete Verbindungen nicht bearbeiten oder löschen.                                                                                                                                                                                                                                                                                                                                       |
| `sshHostAllowlist`                         | beschränken Sie [SSH-Sitzungen](#restrict-which-ssh-hosts-users-can-connect-to) auf Hosts, deren aufgelöster Hostname einem dieser Muster entspricht. Ein leeres Array deaktiviert SSH-Sitzungen. Wird nur aus verwalteten Einstellungen gelesen.                                                                                                                                                                                                                                                                                                   |
| `managedMcpServers`                        | übertragen Sie MCP-Serverkonfigurationen an alle Benutzer in einer Drittanbieter-Bereitstellung. Jeder Eintrag gibt einen Transport von `"http"`, `"sse"` oder `"stdio"`, Verbindungsdetails und optional eine `toolPolicy`-Zuordnung an, die einschränkt, welche Tools in diesem Server Benutzer aufrufen können. Nur in Drittanbieter-Desktop-Bereitstellungen (3P) verfügbar. Stellen Sie diesen Schlüssel über die verwaltete Einstellungsdatei oder MDM bereit, da Drittanbieter-Bereitstellungen keine Admin-Konsolen-Einstellungen erhalten. |

Welche verwalteten Einstellungen eine Desktop-Sitzung erreichen, hängt davon ab, wo diese Sitzung ausgeführt wird. Modellbeschränkungen wie [`availableModels`](/docs/de/model-config#restrict-model-selection) werden in Desktop-Claude-Code-Sitzungen auf die gleiche Weise durchgesetzt wie in der Terminal-CLI; siehe [Oberflächenabdeckung](/docs/de/model-config#surface-coverage).

* **Lokale Sitzungen auf diesem Computer**: Eine verwaltete Einstellungsdatei, die auf der Festplatte bereitgestellt wird, gilt. Verwaltete Einstellungen, die remote über die Admin-Konsole hochgeladen werden, erreichen diese Sitzungen auch auf Anthropics API, wenn sich die Sitzung mit einer Organisationsanmeldung oder einem direkt konfigurierten API-Schlüssel authentifiziert, und folgen dabei der gleichen [Einstellungspriorität](/docs/de/settings#settings-precedence) wie die Terminal-CLI.
* **[Cloud-Sitzungen](#cloud-sessions)**: Werden auf von Anthropic verwalteten VMs ausgeführt und erhalten nur [Server-verwaltete Einstellungen](/docs/de/server-managed-settings).
* **[SSH-Sitzungen](#ssh-sessions)**: Die Sitzung liest die verwaltete Einstellungsdatei vom Remote-Host. Desktop selbst liest `sshConfigs` und `sshHostAllowlist` aus den verwalteten Einstellungen des lokalen Computers beim Erstellen der Verbindung.

`permissions.disableBypassPermissionsMode` und `disableAutoMode` funktionieren auch in Benutzer- und Projekteinstellungen, aber das Platzieren in verwalteten Einstellungen verhindert, dass Benutzer sie überschreiben.

Claude Code liest `autoMode` aus Benutzereinstellungen, dem `--settings`-Flag und verwalteten Einstellungen, aber nicht aus `.claude/settings.json` oder `.claude/settings.local.json`: beide Dateien befinden sich im Repo-Verzeichnis, daher kann ein geklontes Repo oder Build-Schritt seine eigenen Klassifiziererregeln nicht injizieren. Vor v2.1.207 las Claude Code auch `.claude/settings.local.json`.

Für die vollständige Liste der verwalteten Einstellungen einschließlich `allowManagedPermissionRulesOnly` und `allowManagedHooksOnly` siehe [verwaltete Einstellungen](/docs/de/permissions#managed-only-settings).

<h3 id="device-management-policies">
  Geräteverwaltungsrichtlinien
</h3>

IT-Teams können die Desktop-App über MDM auf macOS oder Gruppenrichtlinie unter Windows verwalten. Verfügbare Richtlinien umfassen das Aktivieren oder Deaktivieren der Claude-Code-Funktion, das Steuern von Auto-Updates und das Festlegen einer benutzerdefinierten Bereitstellungs-URL.

* **macOS**: Konfigurieren Sie über die Präferenzdomäne `com.anthropic.claudefordesktop` mit Tools wie Jamf oder Kandji
* **Windows**: Konfigurieren Sie über die Registrierung unter `SOFTWARE\Policies\Claude`

<h3 id="network-access-requirements">
  Netzwerkzugriffsanforderungen
</h3>

Desktop lädt seinen Anwendungscode und Benutzerinhalte von Anthropic-CDN-Hosts.

```text theme={null}
anthropic.com
*.anthropic.com
claude.ai
*.claude.ai
claude.com
*.claude.com
claude.app
*.claude.app
*.claudeusercontent.com
*.claudemcpcontent.com
```

Der Datenverkehr erfolgt über HTTPS auf Port 443, es sei denn, Sie konfigurieren einen benutzerdefinierten Port für [OTLP](/docs/de/monitoring-usage), ein LLM-Gateway oder einen MCP-Server.

Für Proxy-Server, benutzerdefinierte Zertifizierungsstellen, mTLS und die Domänen, die die eigenständige CLI benötigt, siehe [Netzwerkkonfiguration](/docs/de/network-config).

Um die Anzahl der Firewall-Wildcards zu reduzieren, erlauben Sie stattdessen diese Anthropic-Hosts. Bestimmte Subdomänen werden dynamisch generiert und müssen Wildcards bleiben.

```text theme={null}
anthropic.com
api.anthropic.com
a-api.anthropic.com
a-cdn.anthropic.com
s-cdn.anthropic.com
assets-proxy.anthropic.com
claude.ai
a.claude.ai
a-cdn.claude.ai
assets.claude.ai
downloads.claude.ai
*.livepreview.claude.ai
claude.com
platform.claude.com
*.livepreview.claude.app
*.claudeusercontent.com
*.claudemcpcontent.com
```

<h3 id="authentication-and-sso">
  Authentifizierung und SSO
</h3>

Enterprise-Organisationen können SSO für alle Benutzer verlangen. Siehe [Authentifizierung](/docs/de/authentication) für Plan-Level-Details und [Einrichten von SSO](https://support.claude.com/en/articles/13132885-setting-up-single-sign-on-sso) für SAML-Konfiguration; OIDC-Setup wird im [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide) behandelt.

<h3 id="data-handling">
  Datenbehandlung
</h3>

Claude Code verarbeitet Ihren Code lokal in lokalen Sitzungen oder auf Anthropics Cloud-Infrastruktur in Cloud-Sitzungen. Gespräche und Code-Kontext werden an Anthropics API zur Verarbeitung gesendet. Siehe [Datenbehandlung](/docs/de/data-usage) für Details zu Datenspeicherung, Datenschutz und Compliance.

<h3 id="deployment">
  Bereitstellung
</h3>

Desktop kann über Enterprise-Bereitstellungstools verteilt werden:

* **macOS**: Verteilen Sie über MDM wie Jamf oder Kandji mit dem `.dmg`-Installer
* **Windows**: Stellen Sie über das MSIX-Paket bereit. Siehe [Claude Desktop für Windows bereitstellen](https://support.claude.com/en/articles/12622703-deploy-claude-desktop-for-windows) für Enterprise-Bereitstellungsoptionen einschließlich stiller Installation

Für die Domänen, die Sie in Ihrer Firewall auf die Allowlist setzen müssen, siehe [Netzwerkzugriffsanforderungen](#network-access-requirements) oben. Für Proxy-Einstellungen, benutzerdefinierte Zertifizierungsstellen und LLM-Gateways siehe [Netzwerkkonfiguration](/docs/de/network-config).

Für die vollständige Enterprise-Konfigurationsreferenz siehe das [Enterprise-Konfigurationshandbuch](https://support.claude.com/en/articles/12622667-enterprise-configuration).

<h2 id="coming-from-the-cli">
  Kommen Sie von der CLI?
</h2>

Wenn Sie bereits die Claude Code CLI verwenden, führt Desktop dieselbe zugrunde liegende Engine mit einer grafischen Benutzeroberfläche aus. Sie können beide gleichzeitig auf demselben Computer ausführen, sogar auf demselben Projekt. Jede behält separate Sitzungsverlauf, aber sie teilen Konfiguration und Projektgedächtnis über CLAUDE.md-Dateien.

Um eine CLI-Sitzung in Desktop zu verschieben, führen Sie `/desktop` im Terminal aus. Claude speichert Ihre Sitzung und öffnet sie in der Desktop-App, dann beendet die CLI. Dieser Befehl ist auf macOS und Windows verfügbar, wenn Sie mit einem Claude-Abonnement angemeldet sind. Er ist nicht mit API-Schlüssel-Authentifizierung oder auf Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry verfügbar.

<Tip>
  Wann Desktop vs CLI verwendet werden: Verwenden Sie Desktop, wenn Sie parallele Sitzungen in einem Fenster verwalten, Panes nebeneinander anordnen oder Änderungen visuell überprüfen möchten. Verwenden Sie die CLI, wenn Sie Scripting, Automatisierung oder einen Terminal-Workflow bevorzugen.
</Tip>

<h3 id="cli-flag-equivalents">
  CLI-Flag-Äquivalente
</h3>

Diese Tabelle zeigt das Desktop-App-Äquivalent für häufige CLI-Flags. Flags, die nicht aufgelistet sind, haben kein Desktop-Äquivalent, da sie für Scripting oder Automatisierung konzipiert sind.

| CLI                                     | Desktop-Äquivalent                                                                                                                                                                                                           |
| --------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `--model sonnet`                        | Modell-Dropdown neben der Schaltfläche „Senden"                                                                                                                                                                              |
| `--resume`, `--continue`                | Klicken Sie auf eine Sitzung in der Seitenleiste                                                                                                                                                                             |
| `--permission-mode`                     | Moduswahlschalter neben der Schaltfläche „Senden"                                                                                                                                                                            |
| `--dangerously-skip-permissions`        | Bypass-Berechtigungsmodus. Aktivieren Sie auf Pro- und Max-Plänen in Einstellungen → Claude Code → „Bypass-Berechtigungsmodus zulassen"; auf Team- und Enterprise-Plänen wird es durch die Organisationsrichtlinie gesteuert |
| `--add-dir`                             | Fügen Sie mehrere Repos mit der Schaltfläche **+** in Cloud-Sitzungen hinzu                                                                                                                                                  |
| `--allowedTools`, `--disallowedTools`   | Kein Pro-Sitzungs-Äquivalent. Berechtigungsregeln in [Einstellungsdateien](/docs/de/settings) gelten weiterhin.                                                                                                                   |
| `--verbose`                             | [Ausführliche Ansichtsmodus](#switch-view-modes) im Dropdown „Transkript-Ansicht"                                                                                                                                            |
| `--print`, `--output-format`            | Nicht verfügbar. Desktop ist nur interaktiv.                                                                                                                                                                                 |
| `ANTHROPIC_MODEL` Umgebungsvariable     | Modell-Dropdown neben der Schaltfläche „Senden"                                                                                                                                                                              |
| `MAX_THINKING_TOKENS` Umgebungsvariable | Im lokalen Umgebungs-Editor festlegen. Siehe [Umgebungskonfiguration](#environment-configuration).                                                                                                                           |

<h3 id="shared-configuration">
  Gemeinsame Konfiguration
</h3>

Desktop und CLI lesen dieselben Konfigurationsdateien, daher wird Ihr Setup übertragen:

* **[CLAUDE.md](/docs/de/memory)** und `CLAUDE.local.md`-Dateien in Ihrem Projekt werden von beiden verwendet
* **[MCP-Server](/docs/de/mcp)**, die in `~/.claude.json` oder `.mcp.json` konfiguriert sind, funktionieren in beiden
* **[Hooks](/docs/de/hooks)** und **[Skills](/docs/de/skills)**, die in Einstellungen definiert sind, gelten für beide
* **[Einstellungen](/docs/de/settings)** in `~/.claude.json` und `~/.claude/settings.json` werden geteilt. Berechtigungsregeln, erlaubte Tools und andere Einstellungen in `settings.json` gelten für Desktop-Sitzungen.
* **Modelle**: die gleichen [Modelle](/docs/de/model-config#available-models) sind in beiden verfügbar. Wählen Sie in Desktop das Modell aus dem Dropdown neben der Schaltfläche „Senden". Sie können das Modell während der Sitzung ändern.

<Note>
  **MCP-Server aus der Claude Desktop Chat-App**: Die Desktop-App lädt MCP-Server aus `claude_desktop_config.json` in Code-Tab-Sitzungen, zusammen mit Servern aus `~/.claude.json` und `.mcp.json`. Ein Server, der in `claude_desktop_config.json` definiert ist, ist sowohl auf der Desktop-Chat-Oberfläche als auch auf der Registerkarte „Code" verfügbar.

  Die eigenständige CLI liest `claude_desktop_config.json` nicht. Führen Sie auf macOS und WSL `claude mcp add-from-claude-desktop` aus, um diese Server in `~/.claude.json` zu kopieren. Siehe [MCP-Server aus Claude Desktop importieren](/docs/de/mcp#import-mcp-servers-from-claude-desktop) für den Importablauf und Bereichsoptionen.
</Note>

<h3 id="feature-comparison">
  Funktionsvergleich
</h3>

Diese Tabelle vergleicht Kernfunktionen zwischen CLI und Desktop. Für eine vollständige Liste der CLI-Flags siehe die [CLI-Referenz](/docs/de/cli-reference).

| Funktion                                               | CLI                                                              | Desktop                                                                                                                                                                                                                                                                                                                                                                                 |
| ------------------------------------------------------ | ---------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Berechtigungsmodi                                      | Alle Modi einschließlich `dontAsk`                               | Manuell, Bearbeitungen akzeptieren, Plan und Auto. Bypass-Berechtigungen erscheinen im Moduswahlschalter, sobald aktiviert: über den Einstellungsschalter auf Pro- und Max-Plänen oder über die Organisationsrichtlinie auf Team- und Enterprise-Plänen                                                                                                                                 |
| `--dangerously-skip-permissions`                       | CLI-Flag                                                         | Bypass-Berechtigungsmodus. Aktivieren Sie auf Pro- und Max-Plänen in Einstellungen → Claude Code → „Bypass-Berechtigungsmodus zulassen"; auf Team- und Enterprise-Plänen wird es durch die Organisationsrichtlinie gesteuert                                                                                                                                                            |
| [Drittanbieter-Provider](/docs/de/third-party-integrations) | Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry | Anthropic's API standardmäßig. Für Gateway-Routing siehe [Desktop-App mit einem Gateway verbinden](/docs/de/llm-gateway-connect#desktop-app). Um die Code-Registerkarte auf Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry oder einem selbstgehosteten LLM-Gateway auszuführen, siehe [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview). |
| [MCP-Server](/docs/de/mcp)                                  | In Einstellungsdateien konfigurieren                             | Konnektoren-UI für lokale und SSH-Sitzungen oder Einstellungsdateien                                                                                                                                                                                                                                                                                                                    |
| [Plugins](/docs/de/plugins)                                 | `/plugin`-Befehl                                                 | Plugin-Manager-UI                                                                                                                                                                                                                                                                                                                                                                       |
| @mention-Dateien                                       | Textbasiert                                                      | Mit Autovervollständigung; lokale und SSH-Sitzungen nur                                                                                                                                                                                                                                                                                                                                 |
| Dateianhänge                                           | Nicht verfügbar                                                  | Bilder, PDFs                                                                                                                                                                                                                                                                                                                                                                            |
| Sitzungsisolation                                      | [`--worktree`](/docs/de/cli-reference)-Flag                           | Automatische Worktrees                                                                                                                                                                                                                                                                                                                                                                  |
| Mehrere Sitzungen                                      | Separate Terminals                                               | Seitenleisten-Tabs                                                                                                                                                                                                                                                                                                                                                                      |
| Wiederkehrende Aufgaben                                | Cron-Jobs, CI-Pipelines                                          | [Geplante Aufgaben](/docs/de/desktop-scheduled-tasks)                                                                                                                                                                                                                                                                                                                                        |
| Computernutzung                                        | [Aktivieren über `/mcp`](/docs/de/computer-use) auf macOS             | [App- und Bildschirmsteuerung](#let-claude-use-your-computer) auf macOS und Windows                                                                                                                                                                                                                                                                                                     |
| Dispatch-Integration                                   | Nicht verfügbar                                                  | [Dispatch-Sitzungen](#sessions-from-dispatch) in der Seitenleiste                                                                                                                                                                                                                                                                                                                       |
| Scripting und Automatisierung                          | [`--print`](/docs/de/cli-reference), [Agent SDK](/docs/de/headless)        | Nicht verfügbar                                                                                                                                                                                                                                                                                                                                                                         |

<h3 id="what’s-not-available-in-desktop">
  Was ist nicht in Desktop verfügbar
</h3>

Die folgenden Funktionen sind nur in der CLI oder VS Code-Erweiterung verfügbar, außer wo anders angegeben:

* **Drittanbieter-Provider**: Desktop verbindet sich mit Anthropic's API standardmäßig. Um Desktop durch ein Gateway zu leiten, siehe [Desktop-App mit einem Gateway verbinden](/docs/de/llm-gateway-connect#desktop-app). Enterprise-Bereitstellungen können Google Cloud's Agent Platform und Gateway-Provider über [verwaltete Einstellungen](https://claude.com/docs/third-party/claude-desktop/configuration) konfigurieren. Für Amazon Bedrock oder Microsoft Foundry in der CLI siehe [Schnellstart](/docs/de/quickstart). Als Ausnahme zum obigen Abschnitt führt [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview) die Code-Registerkarte auf Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry oder einem selbstgehosteten LLM-Gateway aus.
* **Linux (Beta)**: Computernutzung ist noch nicht in der Linux-Desktop-App verfügbar. Siehe [Claude Desktop auf Linux](/docs/de/desktop-linux).
* **Inline-Code-Vorschläge**: Desktop bietet keine Autovervollständigungs-ähnlichen Vorschläge. Es funktioniert durch Gesprächseingaben und explizite Code-Änderungen.
* **Agent-Teams**: Parallele Claude Code-Sitzungen, die sich gegenseitig Nachrichten senden, sind in der [CLI](/docs/de/agent-teams) verfügbar, nicht in Desktop. Für Multi-Agent-Arbeit innerhalb einer Sitzung verwenden Sie [dynamische Workflows](/docs/de/workflows), die in Desktop ausgeführt werden.
* **Terminal-Dialog-Befehle**: Integrierte Befehle, die ein interaktives Panel im Terminal öffnen, verhalten sich in der Code-Registerkarte anders. Bearbeiten Sie [Einstellungsdateien](/docs/de/settings) direkt, um Berechtigungsregeln und Konfiguration zu verwalten, oder führen Sie die Befehle aus der eigenständigen CLI aus.
  * Befehle ohne Argumentform, wie `/permissions`, antworten mit `isn't available in this environment`.
  * `/config` öffnet Einstellungen → Claude Code. Text nach dem Befehl wird ignoriert, daher setzt `/config theme=dark` das Design nicht.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

Die folgenden Abschnitte behandeln Probleme, die spezifisch für die Desktop-App sind. Für Runtime-API-Fehler, die im Chat angezeigt werden, wie `API Error: 500`, `529 Overloaded`, `429` oder `Prompt is too long`, siehe die [Fehlerreferenz](/docs/de/errors). Diese Fehler und ihre Lösungen sind gleich über CLI, Desktop und Web.

<h3 id="check-your-version">
  Überprüfen Sie Ihre Version
</h3>

Um zu sehen, welche Version der Desktop-App Sie ausführen:

* **macOS**: Klicken Sie auf **Claude** in der Menüleiste und dann auf **Über Claude**
* **Windows**: Klicken Sie auf **Hilfe** und dann auf **Über**

Klicken Sie auf die Versionsnummer, um sie in Ihre Zwischenablage zu kopieren.

<h3 id="403-or-authentication-errors-in-the-code-tab">
  403 oder Authentifizierungsfehler auf der Registerkarte „Code"
</h3>

Wenn Sie `Error 403: Forbidden` oder andere Authentifizierungsfehler bei der Verwendung der Registerkarte „Code" sehen:

1. Melden Sie sich aus dem App-Menü ab und wieder an. Dies ist die häufigste Lösung.
2. Überprüfen Sie, ob Sie ein aktives bezahltes Abonnement haben: Pro, Max, Team oder Enterprise.
3. Wenn die CLI funktioniert, aber Desktop nicht, beenden Sie die Desktop-App vollständig, nicht nur das Fenster schließen, und öffnen Sie sie dann erneut und melden Sie sich an.
4. Überprüfen Sie Ihre Internetverbindung und Proxy-Einstellungen.

<h3 id="blank-or-stuck-screen-on-launch">
  Leerer oder hängender Bildschirm beim Start
</h3>

Wenn die App öffnet, aber einen leeren oder nicht reagierenden Bildschirm anzeigt:

1. Starten Sie die App neu.
2. Überprüfen Sie auf ausstehende Updates. Auf macOS und Windows wird die App beim Start automatisch aktualisiert; unter Linux aktualisieren Sie über apt wie in [Claude Desktop unter Linux](/docs/de/desktop-linux) beschrieben.
3. Überprüfen Sie auf einem verwalteten Netzwerk, dass Ihre Firewall die CDN-Hosts in [Netzwerkzugriffsanforderungen](#network-access-requirements) zulässt.
4. Überprüfen Sie unter Windows den Event Viewer auf Absturzprotokolle unter **Windows Logs → Application**.

<h3 id="failed-to-load-session">
  „Fehler beim Laden der Sitzung"
</h3>

Wenn Sie `Failed to load session` sehen, existiert der ausgewählte Ordner möglicherweise nicht mehr, ein Git-Repository benötigt möglicherweise Git LFS, das nicht installiert ist, oder Dateiberechtigungen verhindern möglicherweise den Zugriff. Versuchen Sie, einen anderen Ordner auszuwählen oder die App neu zu starten.

<h3 id="session-not-finding-installed-tools">
  Sitzung findet installierte Tools nicht
</h3>

Wenn Claude Tools wie `npm`, `node` oder andere CLI-Befehle nicht finden kann, überprüfen Sie, dass die Tools in Ihrem regulären Terminal funktionieren, überprüfen Sie, dass Ihr Shell-Profil PATH richtig einrichtet, und starten Sie die Desktop-App neu, um Umgebungsvariablen neu zu laden.

<h3 id="git-and-git-lfs-errors">
  Git- und Git LFS-Fehler
</h3>

Unter Windows ist Git erforderlich, damit die Registerkarte „Code" lokale Sitzungen startet. Wenn Sie „Git is required" sehen, installieren Sie [Git für Windows](https://git-scm.com/downloads/win) und starten Sie die App neu.

Wenn Sie „Git LFS is required by this repository but is not installed" sehen, installieren Sie Git LFS von [git-lfs.com](https://git-lfs.com/), führen Sie `git lfs install` aus und starten Sie die App neu.

<h3 id="mcp-servers-not-working-on-windows">
  MCP-Server funktionieren nicht unter Windows
</h3>

Wenn MCP-Server-Umschalter nicht reagieren oder Server unter Windows keine Verbindung herstellen, überprüfen Sie, dass der Server in Ihren Einstellungen richtig konfiguriert ist, starten Sie die App neu, überprüfen Sie, dass der Server-Prozess im Task Manager läuft, und überprüfen Sie Server-Protokolle auf Verbindungsfehler.

<h3 id="app-won’t-quit">
  App wird nicht beendet
</h3>

* **macOS**: drücken Sie Cmd+Q. Wenn die App nicht reagiert, verwenden Sie Force Quit mit Cmd+Option+Esc, wählen Sie Claude und klicken Sie auf Force Quit.
* **Windows**: verwenden Sie Task Manager mit Strg+Umschalt+Esc, um den Claude-Prozess zu beenden.

<h3 id="windows-specific-issues">
  Windows-spezifische Probleme
</h3>

* **PATH nicht aktualisiert nach Installation**: Öffnen Sie ein neues Terminal-Fenster. PATH-Updates gelten nur für neue Terminal-Sitzungen.
* **Fehler bei gleichzeitiger Installation**: Wenn Sie einen Fehler über eine andere Installation sehen, die läuft, aber es gibt keine, versuchen Sie, das Installationsprogramm als Administrator auszuführen.

<h3 id="branch-doesn’t-exist-yet-when-opening-in-cli">
  „Branch existiert noch nicht" beim Öffnen in CLI
</h3>

Cloud-Sitzungen können Branches erstellen, die auf Ihrem lokalen Computer nicht existieren. Klicken Sie auf den Branch-Namen in der Sitzungs-Symbolleiste, um ihn zu kopieren, und rufen Sie ihn dann lokal ab:

```bash theme={null}
git fetch origin <branch-name>
git checkout <branch-name>
```

<h3 id="still-stuck">
  Immer noch stecken?
</h3>

* Öffnen Sie Hilfe → Support erhalten in der Desktop-App, oder besuchen Sie das [Claude Support Center](https://support.claude.com/) direkt
* Für Probleme, die auch in der eigenständigen `claude` CLI reproduzierbar sind, suchen Sie oder melden Sie einen Fehler auf [GitHub Issues](https://github.com/anthropics/claude-code/issues)

Wenn Sie einen Fehler melden, geben Sie Ihre Desktop-App-Version, Ihr Betriebssystem, die genaue Fehlermeldung und relevante Protokolle an. Überprüfen Sie auf macOS Console.app. Überprüfen Sie unter Windows Event Viewer → Windows Logs → Application.
