> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Desktop-Anwendung

> Nutzen Sie Claude Code Desktop optimal: parallele Sitzungen mit Git-Isolation, Drag-and-Drop-Pane-Layout, integriertes Terminal und Datei-Editor, Seitenchats, Computernutzung, Dispatch-Sitzungen von Ihrem Telefon, visuelle Diff-Überprüfung, App-Vorschau, PR-Überwachung, Konnektoren und Unternehmenskonfiguration.

Die Claude Desktop-App hat drei Registerkarten: **Chat** für Gespräche, **Cowork** für [Dispatch und längere agentengestützte Arbeiten](https://claude.com/product/cowork) und **Code** für Softwareentwicklung. Diese Seite ist die Referenz für die Registerkarte Code.

<CardGroup cols={3}>
  <Card title="Für macOS herunterladen" icon="apple" href="https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Universeller Build für Intel und Apple Silicon
  </Card>

  <Card title="Für Windows herunterladen" icon="windows" href="https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Für x64-Prozessoren
  </Card>

  <Card title="Claude für Linux abrufen (Beta)" icon="linux" href="/docs/de/desktop-linux">
    apt oder .deb für Ubuntu und Debian
  </Card>
</CardGroup>

Für Windows ARM64 laden Sie das [ARM64-Installationsprogramm](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs) herunter. Unter Linux installieren Sie mit apt; siehe [Claude Desktop unter Linux](/docs/de/desktop-linux).

Nach der Installation starten Sie Claude, melden sich an und klicken auf die Registerkarte **Code**. Eine Anleitung für Ihre erste Sitzung finden Sie im [Leitfaden „Erste Schritte"](/docs/de/desktop-quickstart).

In der Registerkarte Code ist jedes Gespräch eine **Sitzung**: Es hat seinen eigenen Chat-Verlauf und Projektordner, unabhängig von jeder anderen Sitzung. Die Seitenleiste listet Ihre Sitzungen auf und ermöglicht es Ihnen, mehrere parallel auszuführen. Innerhalb einer Sitzung können Sie:

* [Diffs überprüfen und kommentieren](#review-changes-with-diff-view), dann [den resultierenden PR durch CI überwachen](#monitor-pull-request-status)
* [Ihre laufende App](#preview-your-app) in der Browser-Pane in der Vorschau anzeigen, während Claude seine eigenen Änderungen überprüft, und [externe Websites](#browse-external-sites) daneben öffnen
* Claude [Ihre iOS-App im iOS Simulator ausführen und testen](/docs/de/desktop-ios-simulator) in der iOS Simulator-Pane beobachten
* [Panes anordnen](#arrange-your-workspace) für Chat, Diff, Browser, Terminal und Datei-Editor nebeneinander
* Eine [Seitenfrage](#ask-a-side-question-without-derailing-the-session) stellen, die den Kontext der Sitzung nutzt, ohne sie zu beeinträchtigen
* Claude [Ihre anderen Sitzungen überprüfen, Nachrichten senden oder archivieren lassen](#work-across-sessions)
* [Externe Tools verbinden](#connect-external-tools) wie GitHub, Slack und Linear
* Claude [Apps öffnen und Ihren Bildschirm steuern lassen](#let-claude-use-your-computer)
* Auf Ihrem Computer, in der [Cloud](#run-long-running-tasks-in-the-cloud) oder über [SSH](#ssh-sessions) ausführen

Für [geplante wiederkehrende Arbeiten](/docs/de/desktop-scheduled-tasks), [Tastaturkürzel](#keyboard-shortcuts) oder [Aufgaben von Ihrem Telefon senden](#sessions-from-dispatch) siehe die verlinkten Seiten und Abschnitte. Wenn Sie bereits die Terminal-basierte CLI verwenden, siehe den [CLI-Vergleich](#coming-from-the-cli) für das, was übertragen wird.

<h2 id="start-a-session">
  Sitzung starten
</h2>

Bevor Sie Ihre erste Nachricht senden, konfigurieren Sie vier Dinge im Eingabebereich:

* **Umgebung**: Wählen Sie, wo Claude ausgeführt wird. Wählen Sie **Lokal** für Ihren Computer, **Cloud** für eine [Cloud-Sitzung](#cloud-sessions), die nach dem Schließen der App fortgesetzt wird, eine [**SSH-Verbindung**](#ssh-sessions) für einen von Ihnen verwalteten Remote-Computer oder unter Windows eine [**WSL-Distribution**](/docs/de/desktop-wsl). Siehe [Umgebungskonfiguration](#environment-configuration).
* **Projektordner**: Wählen Sie den Ordner oder das Repository aus, in dem Claude arbeitet. Für Cloud-Sitzungen können Sie [mehrere Repositories](#run-long-running-tasks-in-the-cloud) hinzufügen.
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

Berechtigungsmodi kontrollieren, wie viel Autonomie Claude während einer Sitzung hat: ob es vor dem Bearbeiten von Dateien, dem Ausführen von Befehlen oder beidem fragt. Sie können Modi jederzeit mit dem Moduswahlschalter neben der Schaltfläche „Senden" wechseln. Um jede Änderung selbst zu genehmigen, wechseln Sie zu Manual.

Um einen Standardmodus für neue lokale Sitzungen festzulegen, fügen Sie `permissions.defaultMode` zu Ihrer [Einstellungsdatei](/docs/de/settings#where-settings-live) hinzu. Die Desktop-App liest die gleichen Einstellungsdateien wie die CLI. Ein Modus, den Sie im Wahlschalter auswählen, wird pro Ordner gespeichert und hat Vorrang vor `defaultMode` für diesen Ordner, außer Plan, das nur für die aktuelle Sitzung gilt.

| Modus                  | Einstellungsschlüssel | Verhalten                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ---------------------- | --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Manual**             | `default`             | Claude fragt vor dem Bearbeiten von Dateien oder dem Ausführen von Befehlen. Sie sehen einen Diff und können jede Änderung akzeptieren oder ablehnen.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| **Accept edits**       | `acceptEdits`         | Claude akzeptiert Dateibearbeitungen automatisch und häufige Dateisystem-Befehle wie `mkdir`, `touch` und `mv`, fragt aber immer noch vor dem Ausführen anderer Terminal-Befehle. Verwenden Sie dies, wenn Sie Dateiänderungen vertrauen und schnellere Iterationen wünschen.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| **Plan**               | `plan`                | Claude liest Dateien und führt Befehle aus, um zu erkunden, schlägt dann einen Plan vor, ohne Ihren Quellcode zu bearbeiten. Gut für komplexe Aufgaben, bei denen Sie den Ansatz zuerst überprüfen möchten.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| **Auto**               | `auto`                | Claude führt alle Aktionen mit Hintergrund-Sicherheitsprüfungen aus, die die Ausrichtung mit Ihrer Anfrage überprüfen. Reduziert Berechtigungsaufforderungen bei Beibehaltung der Überwachung. Wird angezeigt, wenn [Auto mode verfügbar ist](#auto-mode-availability); es gibt keinen separaten Settings-Umschalter dafür.                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| **Bypass permissions** | `bypassPermissions`   | Claude läuft ohne Berechtigungsaufforderungen, außer für die [Aktionen, die kein Modus automatisch genehmigt](/docs/de/permission-modes#actions-no-mode-auto-approves), Sicherheitsklassifizierer, wenn Claude [auf externen Websites agiert](#browse-external-sites), oder Desktop-Aktionen, bei denen Claude immer zuerst fragt, wie z. B. [Archivieren einer Sitzung](#work-across-sessions). Äquivalent zu `--dangerously-skip-permissions` in der CLI. Aktivieren Sie dies auf Pro- und Max-Plänen in Ihren Einstellungen → Claude Code unter „Allow bypass permissions mode"; auf Team- und Enterprise-Plänen gibt es keinen Settings-Umschalter, und die Organisationsrichtlinie kontrolliert es stattdessen. Verwenden Sie dies nur in sandboxierten Containern oder VMs. |

Frühere Versionen der Code-Registerkarte bezeichneten diese Modi als Ask permissions, Auto accept edits und Plan mode.

Der Berechtigungsmodus `dontAsk` ist nur in der [CLI](/docs/de/permission-modes#allow-only-pre-approved-tools-with-dontask-mode) verfügbar.

<span id="auto-mode-availability" />

Auto mode ist für alle Benutzer auf der Anthropic API verfügbar und erfordert Claude Opus 4.6 oder später, Sonnet 4.6 oder später, oder ein [Fable-Modell](/docs/de/model-config#work-with-fable). Organisationsadministratoren können Auto mode mit dem Schlüssel `disableAutoMode` in [verwalteten Einstellungen](#managed-settings) ausschalten.

Bei Enterprise-Bereitstellungen, die Desktop zu Google Cloud's Agent Platform weiterleiten, ist Auto mode auch standardmäßig verfügbar; siehe [Auto mode on Bedrock, Agent Platform, or Foundry](/docs/de/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry) für die unterstützten Modelle.

<Tip title="Best practice">
  Beginnen Sie komplexe Aufgaben im Plan, damit Claude einen Ansatz abbildet, bevor Änderungen vorgenommen werden. Sobald Sie den Plan genehmigen, wechseln Sie zu Accept edits oder Manual, um ihn auszuführen. Siehe [explore first, then plan, then code](/docs/de/best-practices#explore-first-then-plan-then-code) für mehr zu diesem Workflow.
</Tip>

Cloud-Sitzungen unterstützen Accept edits, Plan und Auto. Accept edits entspricht dem `default`-Modus: Cloud-Sitzungen genehmigen Dateibearbeitungen vorab, daher zeigt der Wahlschalter Accept edits statt Manual an. Bypass permissions ist nicht verfügbar in Cloud-Sitzungen, einschließlich Sitzungen in einer [selbstgehosteten Umgebung](/docs/de/self-hosted-environments).

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

Die Code-Registerkarte ist um Panes aufgebaut, die Sie in jedem Layout anordnen können: Chat, Diff, Browser, Terminal, Datei, Plan, Aufgaben und Subagent, zusammen mit dem [iOS Simulator](/docs/de/desktop-ios-simulator) auf macOS. Ziehen Sie ein Pane an seiner Kopfzeile, um es zu verschieben, oder ziehen Sie eine Pane-Kante, um es zu vergrößern. Drücken Sie **Cmd+\\** auf macOS oder **Ctrl+\\** auf Windows, um das fokussierte Pane zu schließen. Öffnen Sie zusätzliche Panes aus dem Menü **Ansichten** in der Sitzungs-Symbolleiste.

Um über mehrere Bildschirme hinweg zu arbeiten, öffnen Sie ein Pane wie den Diff oder das Terminal in einem eigenen Fenster, und docken Sie es wieder an, wenn Sie fertig sind. Claude arbeitet weiterhin im Hauptfenster.

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

Ansichtsmodi kontrollieren, wie viel Detail im Chat-Transkript angezeigt wird. Wechseln Sie Modi aus dem Dropdown **Transkript-Ansicht** neben der Schaltfläche „Senden", oder drücken Sie **Ctrl+O** auf macOS oder Windows, um durch sie zu zyklisieren. Der Thinking-Modus wird in der Dropdown-Liste nur angezeigt, nachdem Claude Thinking in der Sitzung erzeugt hat, die Sie anzeigen.

| Modus           | Was es anzeigt                                                                                                      |
| --------------- | ------------------------------------------------------------------------------------------------------------------- |
| **Normal**      | Tool-Aufrufe in Zusammenfassungen zusammengefasst, mit vollständigen Text-Antworten                                 |
| **Thinking**    | Tool-Aufrufe in Zusammenfassungen zusammengefasst, plus Claudes Thinking                                            |
| **Ausführlich** | Jeden Tool-Aufruf, jede Datei-Leseoperation und jeden Zwischenschritt, den Claude unternimmt, plus Claudes Thinking |

Verwenden Sie Thinking, um Claudes Überlegungen mit noch zusammengefassten Tool-Aufrufen zu folgen. Verwenden Sie Ausführlich beim Debuggen, warum Claude eine bestimmte Aktion unternommen hat. Claude Desktop-Versionen vor 1.46388.1 listen auch einen Summary-Modus auf, und eine Sitzung, die noch auf Summary eingestellt ist, öffnet sich in Normal, sobald Sie aktualisieren.

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

Computernutzung ermöglicht es Claude, Ihre Apps zu öffnen, Ihren Bildschirm zu steuern und direkt auf Ihrem Computer zu arbeiten, wie Sie es tun würden. Bitten Sie Claude, mit einem Desktop-Tool zu interagieren, das keine CLI hat, oder etwas zu automatisieren, das nur über eine GUI funktioniert. Für das Ausführen und Testen von iOS-Apps öffnet Desktop stattdessen den dedizierten [iOS Simulator-Bereich](/docs/de/desktop-ios-simulator), anstatt Ihren Bildschirm zu steuern; der Bereich funktioniert ohne Aktivierung der Computernutzung.

<Note>
  Computernutzung ist eine Forschungsvorschau auf macOS und Windows, die einen Pro- oder Max-Plan erfordert. Sie ist nicht für Team- oder Enterprise-Pläne verfügbar. Die Claude Desktop-App muss ausgeführt werden.
</Note>

Computernutzung ist standardmäßig deaktiviert. [Aktivieren Sie sie in Einstellungen](#enable-computer-use), bevor Claude Ihren Bildschirm steuern kann. Auf macOS müssen Sie auch Barrierefreiheits- und Bildschirmaufzeichnungsberechtigungen gewähren.

Auf macOS kann Computernutzung auch im Hintergrund ausgeführt werden: Claude arbeitet in den Apps, die Sie genehmigt haben, während Sie weiterarbeiten.

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
* Wenn die Aufgabe das Ausführen oder Testen einer iOS-App ist, verwendet Claude den [iOS Simulator-Bereich](/docs/de/desktop-ios-simulator), der keine Bildschirmsteuerung verwendet.
* Wenn keine dieser Optionen zutrifft, verwendet Claude Computernutzung.

Die [Pro-App-Zugriffsstufen](#app-permissions) verstärken dies: Browser sind auf Nur-Ansicht begrenzt, und Terminals und IDEs auf Nur-Klick, was Claude zum dedizierten Tool lenkt, auch wenn Computernutzung aktiv ist. Bildschirmsteuerung ist für Dinge reserviert, die nichts anderes erreichen kann, wie native Apps, Hardware-Steuerfelder oder proprietäre Tools ohne API.

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
* **Apps anzeigen, wenn Claude fertig ist**: Wenn Computernutzung nicht im Hintergrund ausgeführt wird, blendet Claude Ihre anderen Fenster aus, während es arbeitet, damit es nur mit der genehmigten App interagiert. Wenn Claude fertig ist, werden ausgeblendete Fenster wiederhergestellt, es sei denn, Sie deaktivieren diese Einstellung.

<h2 id="manage-sessions">
  Verwalten Sie Sitzungen
</h2>

Jede Sitzung ist ein unabhängiges Gespräch mit eigenem Kontext und Änderungen. Sie können mehrere Sitzungen parallel ausführen, Seitenchats verzweigen, Claude Ihre anderen Sitzungen überprüfen und Nachrichten senden lassen, Arbeit in die Cloud senden oder Dispatch Sitzungen von Ihrem Telefon aus starten lassen.

<h3 id="work-in-parallel-with-sessions">
  Arbeiten Sie parallel mit Sitzungen
</h3>

Klicken Sie auf **+ Neue Sitzung** in der Seitenleiste, oder drücken Sie **Cmd+N** auf macOS oder **Ctrl+N** auf Windows, um an mehreren Aufgaben parallel zu arbeiten. Drücken Sie **Ctrl+Tab** und **Ctrl+Shift+Tab**, um durch Sitzungen in der Seitenleiste zu zyklisieren. Für Git-Repositories wählen Sie die **worktree**-Option neben dem Branch-Namen, um der Sitzung ihre eigene isolierte Kopie Ihres Projekts mit [Git worktrees](/docs/de/worktrees) zu geben, sodass Änderungen in einer Sitzung andere Sitzungen nicht beeinflussen, bis Sie sie committen.

Um zwei Sitzungen gleichzeitig anzuzeigen, halten Sie **Cmd** auf macOS oder **Ctrl** auf Windows gedrückt und klicken Sie auf eine Sitzung in der Seitenleiste. Die Sitzung wird in einem zweiten Bereich neben dem bereits geöffneten angezeigt. Während die Aufteilung aktiv ist, ersetzt das Klicken auf eine andere Sitzung in der Seitenleiste denjenigen Bereich, der den Fokus hat. Drücken Sie **Cmd+\\** auf macOS oder **Ctrl+\\** auf Windows, um den fokussierten Bereich zu schließen und zu einer einzelnen Sitzung zurückzukehren.

Worktrees werden standardmäßig in `<project-root>/.claude/worktrees/` gespeichert. Sie können dies in Einstellungen → Claude Code unter „Worktree-Speicherort" in ein benutzerdefiniertes Verzeichnis ändern. Sie können auch ein Branch-Präfix festlegen, das jedem Worktree-Branch-Namen vorangestellt wird, was nützlich ist, um von Claude erstellte Branches organisiert zu halten. Um einen Worktree zu entfernen, wenn Sie fertig sind, fahren Sie mit der Maus über die Sitzung in der Seitenleiste und klicken Sie auf das Archiv-Symbol. Um Sitzungen automatisch zu archivieren, wenn ihr Pull Request zusammengeführt oder geschlossen wird, schalten Sie **Auto-Archivieren nach PR-Merge oder -Schließung** in Einstellungen → Claude Code ein. Auto-Archivieren gilt nur für lokale Sitzungen, die beendet wurden.

Um gitignorierte Dateien wie `.env` in neue Worktrees einzubeziehen, erstellen Sie eine [`.worktreeinclude`-Datei](/docs/de/worktrees#copy-gitignored-files-into-worktrees) in Ihrem Projektstammverzeichnis.

<Note>
  Die Sitzungsisolation erfordert [Git](https://git-scm.com/downloads). Die meisten Macs enthalten Git standardmäßig. Führen Sie `git --version` im Terminal aus, um zu überprüfen. Wenn es eine Versionsnummer ausgibt, ist Git installiert. Wenn Sie auf Git-Fehler stoßen, bitten Sie Claude im [Cowork-Tab](https://claude.com/product/cowork), Ihnen bei der Behebung Ihres Setups zu helfen.
</Note>

Verwenden Sie die Steuerelemente oben in der Seitenleiste, um Sitzungen nach Status, Projekt oder Umgebung zu filtern, und um Sitzungen nach Projekt zu gruppieren. Um eine Sitzung umzubenennen, klicken Sie auf den Sitzungstitel in der Symbolleiste oben in der aktiven Sitzung.

Um die Kontext-Nutzung zu überprüfen, siehe [Überprüfen Sie die Nutzung](#check-usage). Wenn der Kontext voll wird, fasst Claude das Gespräch automatisch zusammen und arbeitet weiter. Sie können auch `/compact` eingeben, um die Zusammenfassung früher auszulösen und Kontextraum freizugeben. Siehe [das Kontextfenster](/docs/de/how-claude-code-works#the-context-window) für Details, wie die Komprimierung funktioniert.

Die Desktop-App sendet eine Betriebssystem-Benachrichtigung, wenn eine Code-Sitzung eine Aufgabe abschließt und Sie diese Sitzung gerade nicht anzeigen. Für Sitzungen, die zu einem [Projekt](/docs/de/claude-projects#see-what-needs-you-in-overview) gehören, erhalten Sie stattdessen die Benachrichtigungen des Projekts.

<h3 id="ask-a-side-question-without-derailing-the-session">
  Fragen Sie eine Seitenfrage, ohne die Sitzung zu entgleisen
</h3>

Ein Seitenchat ermöglicht es Ihnen, Claude eine Frage zu stellen, die den Kontext Ihrer Sitzung nutzt, aber nichts zum Hauptgespräch hinzufügt. Verwenden Sie ihn, wenn Sie ein Stück Code verstehen, eine Annahme überprüfen oder eine Idee erkunden möchten, ohne die Sitzung vom Kurs abzubringen.

Drücken Sie **Cmd+;** auf macOS oder **Ctrl+;** auf Windows, um einen Seitenchat zu öffnen, oder geben Sie `/btw` im Eingabefeld ein. Der Seitenchat kann alles im Hauptthread bis zu diesem Punkt lesen. Wenn Sie fertig sind, schließen Sie den Seitenchat und setzen Sie die Hauptsitzung dort fort, wo Sie aufgehört haben.

Seitenchats sind in lokalen, SSH- und WSL-Sitzungen verfügbar. Die Desktop-App speichert Seitenchats nicht auf der Festplatte, daher können Sie nach dem Schließen der App nicht zu einem zurückkehren.

<h3 id="watch-background-tasks">
  Beobachten Sie Hintergrund-Aufgaben
</h3>

Das Aufgaben-Pane zeigt die Hintergrundarbeit, die in der aktuellen Sitzung läuft: Subagents, Hintergrund-Shell-Befehle und [dynamische Workflows](/docs/de/workflows). Öffnen Sie es aus dem Menü **Ansichten** oder ziehen Sie es in Ihr Layout.

Klicken Sie auf einen beliebigen Eintrag, um seine Ausgabe im Subagent-Pane zu sehen oder ihn zu stoppen. Um zu sehen, was andere Sitzungen tun, verwenden Sie die [Seitenleiste](#work-in-parallel-with-sessions), oder bitten Sie Claude, [sie für Sie zu überprüfen](#work-across-sessions).

<h3 id="work-across-sessions">
  Arbeiten Sie sitzungsübergreifend
</h3>

Claude kann Ihre anderen Code-Tab-Sitzungen auflisten, lesen, was jede getan hat, und Nachrichten zwischen ihnen senden. Fragen Sie in natürlicher Sprache: „Welche Sitzung hat die Auth-Umgestaltung berührt?", „Zu welchem Ergebnis ist die API-Sitzung gekommen?" oder „Teilen Sie der Zahlungs-Sitzung mit, dass sich das Schema geändert hat". Sie können Claude auch bitten, eine Sitzung umzubenennen oder zu archivieren. Claude archiviert eine Sitzung auf die gleiche Weise wie das Archiv-Symbol der Seitenleiste, daher bitten Sie es, Sitzungen zu bereinigen, deren PRs zusammengeführt wurden.

Über diese Oberfläche sieht Claude nur die Sitzungen, die die Desktop-App selbst ausführt: lokale, [SSH](#ssh-sessions) und [WSL](/docs/de/desktop-wsl) Sitzungen im Code-Tab. Claude sieht keine Cloud-Sitzungen oder Sitzungen, die Sie vom Terminal CLI oder der VS Code-Erweiterung gestartet haben, auch nicht in Worktrees desselben Projekts. Mit neun Terminal-Worktrees offen und zwei Desktop-Sitzungen meldet Claude, das in einer von ihnen antwortet, die eine andere Desktop-Sitzung. Claude listet niemals die Sitzung auf, von der aus Sie fragen. Standardmäßig sieht es die 20 zuletzt aktiven Sitzungen und überspringt archivierte Sitzungen, es sei denn, Sie fragen danach. [Sitzungsübergreifendes Messaging](/docs/de/cross-session-messaging) ermöglicht Claude separat, [Ihre anderen Claude Code-Sitzungen](/docs/de/cross-session-messaging#see-which-sessions-claude-can-reach) zu kontaktieren, einschließlich Terminal-Sitzungen.

Wenn Claude eine andere Sitzung über diese Oberfläche kontaktiert, zeigt Claude Code sie dort als eine Karte an, die mit dem Titel der sendenden Sitzung gekennzeichnet ist und einen Link zurück enthält, damit Sie immer sehen können, woher eine Nachricht kam. Wenn die empfangende Sitzung gerade eine Aufgabe ausführt, hält Claude Code die Nachricht und Claude liest sie, sobald die aktuelle Arbeit beendet ist. Der empfangende Claude kann antworten, und Claude Code liefert die Antwort über diese Oberfläche zurück. Claude kann nicht an eine archivierte Sitzung liefern und teilt Ihnen mit, wenn eine Nachricht nicht durchkommt.

Claude Code wendet vier Sicherheitsverhalten sitzungsübergreifend an:

* Bevor Claude eine Sitzung archiviert, fragt es Sie zuerst. Sie sehen die Genehmigungskarte in jedem Berechtigungsmodus, einschließlich Auto- und Bypass-Berechtigungen.
* Über diese Oberfläche kann Claude keine sitzungsübergreifenden Nachrichten von einer Sitzung senden, die niemand beobachtet, z. B. eine geplante Aufgabenausführung, und kann nicht in eine liefern.
* Claude Code überprüft jede Nachricht von dieser Oberfläche gegen die [eingehenden Steuerelemente](/docs/de/cross-session-messaging#control-inbound-messages) der empfangenden Sitzung, auch wenn die empfangende Sitzung nicht über [sitzungsübergreifendes Messaging](/docs/de/cross-session-messaging#availability) selbst verfügt. Wenn Sie [`crossSessionInbound`](/docs/de/settings-reference#crosssessioninbound) in der empfangenden Sitzung auf `refuse` setzen, verwirft Claude Code Nachrichten von dieser Oberfläche. Claude Code meldet die Ablehnung der Claude Desktop-App. Vor v2.1.234 verwarf Claude Code jede Nachricht von dieser Oberfläche an eine empfangende Sitzung ohne sitzungsübergreifendes Messaging.
* Claude Code zitiert jede eingehende Nachricht und schreibt sie der Sitzung zu, die sie gesendet hat, und Claude befolgt immer noch die eigenen Berechtigungseinstellungen der empfangenden Sitzung, wenn er auf eine eingeht.

Claude kann auch neue Sitzungen vorschlagen. Wenn es etwas bemerkt, das es wert ist, behoben zu werden und das außerhalb des Umfangs der aktuellen Aufgabe liegt, bietet es die Arbeit als Task-Chip im Chat an. Klicken Sie auf den Chip, um diese Arbeit in einer neuen Sitzung mit eigenem Worktree zu starten. Claude setzt Ihre aktuelle Sitzung ungestört fort.

<h3 id="run-long-running-tasks-in-the-cloud">
  Führen Sie lange laufende Aufgaben in der Cloud aus
</h3>

Für große Umgestaltungen, Test-Suites, Migrationen oder andere lange laufende Aufgaben wählen Sie **Cloud** statt **Lokal**, wenn Sie eine Sitzung starten. Cloud-Sitzungen laufen standardmäßig auf von Anthropic verwalteter Infrastruktur und werden fortgesetzt, auch wenn Sie die App schließen oder Ihren Computer herunterfahren. Überprüfen Sie jederzeit den Fortschritt oder lenken Sie Claude in eine andere Richtung. Sie können Cloud-Sitzungen auch von [claude.ai/code](https://claude.ai/code) oder der [Claude Mobile-App](/docs/de/mobile) aus überwachen.

Cloud-Sitzungen unterstützen auch mehrere Repositories. Nach Auswahl einer Cloud-Umgebung klicken Sie auf die Schaltfläche **+** neben dem ausgewählten Repository, um zusätzliche Repositories zur Sitzung hinzuzufügen. Jedes Repo erhält seinen eigenen Branch-Wahlschalter. Dies ist nützlich für Aufgaben, die mehrere Codebases umfassen, z. B. das Aktualisieren einer gemeinsamen Bibliothek und ihrer Consumer.

Siehe [Claude Code im Web](/docs/de/claude-code-on-the-web) für mehr darüber, wie Cloud-Sitzungen funktionieren. Wenn ein Körper von Arbeit viele Cloud-Sitzungen benötigt, wählen Sie **Projekte** in der Seitenleiste, um ein [Projekt](/docs/de/claude-projects) zu erstellen, in dem Claude die Sitzungen für Sie aus einem Gespräch heraus startet und verfolgt.

<h3 id="continue-in-another-surface">
  Fortsetzen auf einer anderen Oberfläche
</h3>

Das Menü **Fortsetzen in**, das über das VS Code-Symbol unten rechts in der Sitzungs-Symbolleiste zugänglich ist, ermöglicht es Ihnen, Ihre Sitzung auf eine andere Oberfläche zu verschieben:

* **Claude Code im Web**: sendet Ihre lokale Sitzung, um in der Cloud weiter zu laufen. Desktop pusht Ihren Branch, generiert eine Zusammenfassung des Gesprächs und erstellt eine neue Cloud-Sitzung mit dem vollständigen Kontext. Sie können dann wählen, die lokale Sitzung zu archivieren oder zu behalten. Dies erfordert einen sauberen Arbeitsbaum und ist nicht für SSH-Sitzungen verfügbar.
* **Ihre IDE**: öffnet Ihr Projekt in einer unterstützten IDE im aktuellen Arbeitsverzeichnis.

<h3 id="sessions-from-dispatch">
  Sitzungen von Dispatch
</h3>

[Dispatch](https://support.claude.com/en/articles/13947068) ist ein persistentes Gespräch mit Claude, das in der Registerkarte [Cowork](https://claude.com/product/cowork) lebt. Sie senden Dispatch eine Aufgabe, und es entscheidet, wie damit umzugehen ist.

Eine Aufgabe kann auf zwei Wegen als Code-Sitzung enden: Sie fragen direkt danach, z. B. „Öffnen Sie eine Claude Code-Sitzung und beheben Sie den Login-Fehler", oder Dispatch entscheidet, dass die Aufgabe Entwicklungsarbeit ist und startet eine von selbst. Aufgaben, die typischerweise zu Code führen, umfassen das Beheben von Fehlern, das Aktualisieren von Abhängigkeiten, das Ausführen von Tests oder das Öffnen von Pull Requests. Forschung, Dokumentbearbeitung und Tabellenkalkulationsarbeit bleiben in Cowork.

In jedem Fall wird die Code-Sitzung in der Seitenleiste der Registerkarte „Code" mit einem **Dispatch**-Badge angezeigt. Sie erhalten eine Push-Benachrichtigung auf Ihrem Telefon, wenn sie fertig ist oder Ihre Genehmigung benötigt.

Wenn Sie [Computernutzung](#let-claude-use-your-computer) aktiviert haben, können Dispatch-generierte Code-Sitzungen diese auch verwenden. App-Genehmigungen in diesen Sitzungen verfallen nach 30 Minuten und werden erneut angefordert, anstatt die gesamte Sitzung zu dauern wie bei regulären Code-Sitzungen.

Für Setup, Pairing und Dispatch-Einstellungen siehe den [Dispatch-Hilfeartikel](https://support.claude.com/en/articles/13947068). Dispatch erfordert einen Pro- oder Max-Plan und ist nicht für Team- oder Enterprise-Pläne verfügbar.

Dispatch ist eine von mehreren Möglichkeiten, mit Claude zu arbeiten, wenn Sie weg von Ihrem Terminal sind. Für einen Vergleich mit den anderen Optionen siehe [Plattformen und Integrationen](/docs/de/platforms#work-when-you-are-away-from-your-terminal).

<h2 id="extend-claude-code">
  Claude Code erweitern
</h2>

Verbinden Sie externe Dienste, fügen Sie wiederverwendbare Workflows hinzu, passen Sie Claudes Verhalten an und konfigurieren Sie Vorschauserver. Um Connectors, Skills und Plugins an einem Ort zu verwalten, klicken Sie auf **Anpassen** in der Seitenleiste. Die [Cowork](https://claude.com/product/cowork)-Registerkarte in der Desktop-App bezieht ihre Skills, Plugins und Connectors aus dieser Anpassungskonfiguration, die über Ihr claude.ai-Konto synchronisiert wird, nicht aus dem CLI-Verzeichnis `~/.claude`.

Claude Code lädt auch die Skills und Plugins, die für Ihr claude.ai-Konto aktiviert sind, in Terminalsitzungen, in denen Sie sich mit demselben Konto anmelden. Siehe [Skills synchronisiert von claude.ai](/docs/de/skills#how-synced-skills-behave) und [Plugins synchronisiert von claude.ai](/docs/de/plugins/loading#synced-plugins).

<h3 id="connect-external-tools">
  Externe Tools verbinden
</h3>

Für lokale und [SSH](#ssh-sessions)-Sitzungen klicken Sie auf die Schaltfläche **+** neben dem Eingabefeld und wählen Sie **Connectors** aus, um Integrationen wie Google Calendar, Slack, GitHub, Linear, Notion und mehr hinzuzufügen. Sie können Connectors vor oder während einer Sitzung hinzufügen. Die Schaltfläche **+** ist in Cloud- oder WSL-Sitzungen nicht verfügbar, aber [Routinen](/docs/de/routines) konfigurieren Connectors zum Zeitpunkt der Routinenerstellung.

Um Connectors zu verwalten oder zu trennen, gehen Sie zu Einstellungen → Connectors in der Desktop-App oder wählen Sie **Connectors verwalten** aus dem Connectors-Menü im Eingabefeld.

Nach der Verbindung kann Claude Ihren Kalender lesen, Nachrichten senden, Probleme erstellen und direkt mit Ihren Tools interagieren. Sie können Claude fragen, welche Connectors in Ihrer Sitzung konfiguriert sind.

Connectors sind [MCP-Server](/docs/de/mcp) mit einem grafischen Setup-Ablauf. Verwenden Sie sie für eine schnelle Integration mit unterstützten Diensten. Für Integrationen, die nicht in Connectors aufgelistet sind, fügen Sie MCP-Server manuell über [Einstellungsdateien](/docs/de/mcp#installing-mcp-servers) hinzu. Sie können auch [benutzerdefinierte Connectors erstellen](https://support.claude.com/en/articles/11175166-getting-started-with-custom-connectors-using-remote-mcp).

<h3 id="use-skills">
  Skills verwenden
</h3>

[Skills](/docs/de/skills) erweitern, was Claude tun kann. Claude lädt sie automatisch, wenn sie relevant sind, oder Sie können eine direkt aufrufen: Geben Sie `/` im Eingabefeld ein oder klicken Sie auf die Schaltfläche **+** und wählen Sie **Slash-Befehle** aus, um zu sehen, was verfügbar ist. Dies umfasst [integrierte Befehle](/docs/de/commands), Ihre [benutzerdefinierten Skills](/docs/de/skills#create-your-first-skill), Projekt-Skills aus Ihrer Codebasis und Skills aus allen [installierten Plugins](/docs/de/plugins/install). Wählen Sie einen aus und er wird im Eingabefeld hervorgehoben angezeigt. Geben Sie Ihre Aufgabe danach ein und senden Sie wie gewohnt.

Sie können einen Befehl senden, während Claude arbeitet, genauso wie jede andere Nachricht, und die Sitzung kehrt in den Leerlauf zurück, sobald der Zug beendet ist. Vor v2.1.206 konnte ein während des Zugs gesendeter Befehl die Sitzung als laufend anzeigen lassen und Nachrichten, die Sie danach sendeten, wurden nicht zugestellt.

Lokale Sitzungen laden Ihre persönlichen Skills aus `~/.claude/skills/`. Eine [SSH](#ssh-sessions)-Sitzung liest `~/.claude/skills/` aus dem Home-Verzeichnis des Remote-Hosts, nicht von Ihrem Computer.

Lokale und Cloud-Sitzungen laden auch die Skills, die für Ihr claude.ai-Konto aktiviert sind. Cloud-Sitzungen laden sie statt aus `~/.claude/skills/`, wie [Skills in Cowork- und Cloud-Sitzungen](/docs/de/skills#skills-in-cowork-and-cloud-sessions) beschreibt.

<h3 id="install-plugins">
  Plugins installieren
</h3>

[Plugins](/docs/de/plugins/overview) sind wiederverwendbare Pakete, die Skills, Agents, Hooks, MCP-Server und LSP-Konfigurationen zu Claude Code hinzufügen. Sie können Plugins aus der Desktop-App installieren, ohne das Terminal zu verwenden.

Für lokale und [SSH](#ssh-sessions)-Sitzungen klicken Sie auf die Schaltfläche **+** neben dem Eingabefeld und wählen Sie **Plugins** aus, um Ihre installierten Plugins und deren Skills zu sehen. Um ein Plugin hinzuzufügen, wählen Sie **Plugin hinzufügen** aus dem Untermenü, um den Plugin-Browser zu öffnen, der verfügbare Plugins aus Ihren konfigurierten [Marketplaces](/docs/de/plugins/overview) einschließlich des offiziellen Anthropic-Marketplace anzeigt. Wählen Sie **Plugins verwalten** aus, um Plugins zu aktivieren, zu deaktivieren oder zu deinstallieren.

Sie können Plugins auf Ihr Benutzerkonto, ein bestimmtes Projekt oder nur lokal beschränken. Wenn Ihre Organisation Plugins zentral verwaltet, sind diese Plugins in Desktop-Sitzungen auf die gleiche Weise verfügbar wie in der CLI.

Der Plugin-Browser ist in Cloud-Sitzungen nicht verfügbar, und Plugins, die Sie aus der Desktop-App installieren, sind nicht für Cloud-Sitzungen verfügbar. Eine Cloud-Sitzung installiert auch keine Plugins, die das Repository unter `.claude/settings.json` deklariert, wie [Was wird aus Ihrer Einrichtung übernommen](/docs/de/cloud-environments#what-carries-over-from-your-setup) erklärt. Plugins sind in WSL-Sitzungen nicht verfügbar. Für die vollständige Plugin-Referenz einschließlich der Erstellung eigener Plugins siehe [Plugins](/docs/de/plugins/overview).

<h3 id="configure-preview-servers">
  Vorschauserver konfigurieren
</h3>

Claude erkennt automatisch Ihre Dev-Server-Einrichtung und speichert die Konfiguration in `.claude/launch.json` im Stammverzeichnis des Ordners, den Sie beim Starten der Sitzung ausgewählt haben. Die Vorschau verwendet diesen Ordner als Arbeitsverzeichnis. Wenn Sie also einen übergeordneten Ordner ausgewählt haben, werden Unterordner mit ihren eigenen Dev-Servern nicht automatisch erkannt. Um mit dem Server eines Unterordners zu arbeiten, starten Sie entweder eine Sitzung direkt in diesem Ordner oder fügen Sie eine Konfiguration manuell hinzu.

Um anzupassen, wie Ihr Server startet, z. B. um `yarn dev` statt `npm run dev` zu verwenden oder den Port zu ändern, bearbeiten Sie die Datei manuell oder klicken Sie auf **Konfiguration bearbeiten** im Server-Dropdown, um sie in Ihrem Code-Editor zu öffnen. Die Datei unterstützt JSON mit Kommentaren.

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
  Änderungen automatisch überprüfen
</h4>

Wenn `autoVerify` aktiviert ist, überprüft Claude automatisch Code-Änderungen nach dem Bearbeiten von Dateien. Es macht Screenshots, prüft auf Fehler und bestätigt, dass Änderungen funktionieren, bevor es seine Antwort abschließt.

Auto-Verify ist standardmäßig aktiviert. Deaktivieren Sie es pro Projekt, indem Sie `"autoVerify": false` zu `.claude/launch.json` hinzufügen, oder schalten Sie es aus dem Server-Dropdown-Menü um.

```json theme={null}
{
  "version": "0.0.1",
  "autoVerify": false,
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

Wenn deaktiviert, sind Vorschau-Tools weiterhin verfügbar und Sie können Claude jederzeit bitten, zu überprüfen. Auto-Verify macht es automatisch nach jeder Bearbeitung.

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
| `autoPort`          | boolean   | Wie mit Port-Konflikten umgegangen wird. Siehe [Port-Konflikte](#port-conflicts)                                                                                                                                                                                                                                |
| `program`           | string    | Ein Skript, das mit `node` ausgeführt werden soll. Siehe [wann `program` vs `runtimeExecutable` verwendet werden soll](#when-to-use-program-vs-runtimeexecutable)                                                                                                                                               |
| `args`              | string\[] | An `program` übergebene Argumente. Wird nur verwendet, wenn `program` gesetzt ist                                                                                                                                                                                                                               |
| `url`               | string    | Die Adresse, die die Vorschau statt `http://localhost:<port>` öffnet. Siehe [Vorschau unter einer bestimmten URL öffnen](#open-the-preview-at-a-specific-url)                                                                                                                                                   |

<a id="when-to-use-program-vs-runtimeexecutable" />

<h5 id="when-to-use-program-vs-runtimeexecutable">
  Wann `program` vs `runtimeExecutable` verwendet werden soll
</h5>

Verwenden Sie `runtimeExecutable` mit `runtimeArgs`, um einen Dev-Server über einen Package-Manager zu starten. Zum Beispiel führt `"runtimeExecutable": "npm"` mit `"runtimeArgs": ["run", "dev"]` `npm run dev` aus.

Verwenden Sie `program`, wenn Sie ein eigenständiges Skript haben, das Sie direkt mit `node` ausführen möchten. Zum Beispiel führt `"program": "server.js"` `node server.js` aus. Übergeben Sie zusätzliche Flags mit `args`.

<a id="open-the-preview-at-a-specific-url" />

<h5 id="open-the-preview-at-a-specific-url">
  Vorschau unter einer bestimmten URL öffnen
</h5>

Standardmäßig öffnet die Vorschau `http://localhost:<port>`. Legen Sie `url` fest, wenn Ihr Server eine andere Adresse benötigt. Häufige Fälle sind Server, die lokales HTTPS erfordern, Apps, die `*.localhost`-Subdomänen verwenden, und Apps, die Sie durch eine Umleitung anmelden.

```json theme={null}
{
  "version": "0.0.1",
  "configurations": [
    {
      "name": "my-app",
      "runtimeExecutable": "npm",
      "runtimeArgs": ["run", "dev"],
      "port": 8443,
      "url": "https://localhost:8443"
    }
  ]
}
```

Localhost-Adressen öffnen sich direkt, genau wie die Standard-Port-Adresse. Dies umfasst `localhost`, jede `*.localhost`-Subdomain, `127.0.0.1` und `::1`. Aus Sicherheitsgründen muss eine Localhost-`url` nur der Ursprung Ihres Servers sein, ohne Pfad oder Abfrage. Sein Port muss dem Port des Eintrags entsprechen. Um eine bestimmte Seite anzuzeigen, bitten Sie Claude, dorthin zu navigieren, nachdem die Vorschau geöffnet wird. Eine Localhost-`url` mit einem Pfad, einer Abfrage oder einem nicht übereinstimmenden Port wird als Konfigurationsfehler gemeldet, der die URL benennt und die Korrektur anzeigt.

Für jede andere Adresse fragt Desktop beim ersten Öffnen der Vorschau um Ihre Berechtigung, genauso wie wenn Sie in der Vorschau zu einer neuen Website navigieren. Externe Adressen können Pfade enthalten. Wählen Sie **Immer zulassen**, um die Eingabeaufforderung für diese Website in Zukunft zu überspringen. Organisationsrichtlinien, die externe Websites in der Vorschau einschränken, gelten weiterhin.

Um eine Vorschau eines Servers anzuzeigen, den Sie bereits selbst ausführen, legen Sie `url` ohne einen Befehl fest. Claude verbindet die Vorschau mit Ihrem laufenden Server, anstatt einen zu starten:

```json theme={null}
{
  "version": "0.0.1",
  "configurations": [
    {
      "name": "my-app",
      "url": "https://app.localhost:3000"
    }
  ]
}
```

Die `url` muss `http` oder `https` sein und darf keinen Benutzernamen oder Passwort enthalten.

<h4 id="port-conflicts">
  Port-Konflikte
</h4>

Das Feld `autoPort` steuert, was passiert, wenn Ihr bevorzugter Port bereits verwendet wird:

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

  <Tab title="Mehrere Server">
    Für ein Monorepo mit einem Frontend und einem API-Server definieren Sie mehrere Konfigurationen. Das Frontend verwendet `autoPort: true`, damit es einen freien Port wählt, wenn 3000 belegt ist, während der API-Server Port 8080 genau benötigt:

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

  <Tab title="Node.js-Skript">
    Um ein Node.js-Skript direkt auszuführen, anstatt einen Package-Manager-Befehl zu verwenden, verwenden Sie das Feld `program`:

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
* **Cloud**: läuft standardmäßig auf Anthropic-verwalteter Infrastruktur. Sitzungen werden fortgesetzt, auch wenn Sie die App schließen.
* **SSH**: läuft auf einem Remote-Computer, mit dem Sie sich über SSH verbinden, z. B. Ihre eigenen Server, Cloud-VMs oder Dev-Container
* **WSL** (Windows): läuft in einer [WSL 2-Distribution](/docs/de/desktop-wsl) auf Ihrem Computer und verwendet deren Linux-Toolchain und native Pfade

<h3 id="local-sessions">
  Lokale Sitzungen
</h3>

Die Desktop-App erbt nicht immer Ihre vollständige Shell-Umgebung. Auf macOS liest die App beim Starten aus dem Dock oder Finder Ihr Shell-Profil, z. B. `~/.zshrc` oder `~/.bashrc`, um `PATH` und einen festen Satz von Claude Code-Variablen zu extrahieren, aber andere Variablen, die Sie dort exportieren, werden nicht übernommen. Unter Windows erbt die App Benutzer- und Systemumgebungsvariablen, liest aber keine PowerShell-Profile.

Um Umgebungsvariablen für lokale Sitzungen und Dev-Server auf jeder Plattform festzulegen, öffnen Sie das Umgebungs-Dropdown im Eingabefeld, fahren Sie mit der Maus über **Lokal** und klicken Sie auf das Zahnrad-Symbol, um den lokalen Umgebungs-Editor zu öffnen. Variablen, die Sie hier speichern, werden verschlüsselt auf Ihrem Computer gespeichert und gelten für jede lokale Sitzung und jeden Vorschau-Server, den Sie starten. Sie können auch Variablen zum Schlüssel `env` in Ihrer Datei `~/.claude/settings.json` hinzufügen, obwohl diese nur Claude-Sitzungen erreichen und nicht Dev-Server. Siehe [Umgebungsvariablen](/docs/de/env-vars) für die vollständige Liste der unterstützten Variablen.

[Erweitertes Denken](/docs/de/model-config#extended-thinking) ist standardmäßig aktiviert, was die Leistung bei komplexen Denkaufgaben verbessert, aber zusätzliche Token verwendet. Auf der Anthropic API setzen Sie `MAX_THINKING_TOKENS` auf `0` im lokalen Umgebungs-Editor, um das Denken auszuschalten; dies hat keine Auswirkung auf Opus 5.5 oder die Fable-Modelle, die immer erweitertes Denken verwenden. Bei ausgeschaltetem Denken auf der Anthropic API sendet Claude Code stattdessen den Aufwand `high` an Modelle, von denen es weiß, dass sie [diese Kombination nicht akzeptieren](/docs/de/errors#effort-isnt-available-with-thinking-turned-off), wie Opus 5.

Bei Modellen mit [adaptiver Argumentation](/docs/de/model-config#adjust-effort-level) werden `MAX_THINKING_TOKENS`-Werte außer `0` ignoriert, da adaptive Argumentation die Denktiefe steuert. Bei Opus 4.6 und Sonnet 4.6 setzen Sie `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING` auf `1`, um ein festes Denk-Budget zu verwenden; Fable-Modelle, Sonnet 5 und Opus 4.7 und später verwenden immer adaptive Argumentation und haben keinen Modus mit festem Budget.

<h4 id="local-sessions-on-managed-devices">
  Lokale Sitzungen auf verwalteten Geräten
</h4>

Ihr Administrator kann lokale Sitzungen mit der verwalteten Einstellung [`disableDesktopLocalSessions`](#managed-settings) deaktivieren. Wenn dies der Fall ist, bleibt **Lokal** im Umgebungs-Dropdown, ist aber ausgegraut und kann nicht ausgewählt werden, mit einem Tooltip, das besagt, dass Ihre Organisation es deaktiviert hat, und unter Windows ist der [WSL](/docs/de/desktop-wsl)-Eintrag, dessen Verfügbarkeit auf verwalteten Geräten [separat geregelt wird](/docs/de/admin-setup#wsl-sessions-in-claude-code-desktop), auf die gleiche Weise ausgegraut. Neue Sitzungen werden standardmäßig auf die erste SSH-Verbindung gesetzt, wenn eine konfiguriert ist, und Desktop zeigt eine Meldung an, dass lokale Sitzungen auf diesem Gerät nicht verfügbar sind, wenn Sie versuchen, eine vorhandene fortzusetzen. Wählen Sie stattdessen eine [SSH](#ssh-sessions)- oder [Cloud](#cloud-sessions)-Umgebung, oder kontaktieren Sie Ihr IT-Team.

<h3 id="cloud-sessions">
  Cloud-Sitzungen
</h3>

Cloud-Sitzungen werden im Hintergrund fortgesetzt, auch wenn Sie die App schließen. Die Nutzung wird auf Ihre [Abonnementplanlimits](/docs/de/costs) angerechnet, ohne separate Compute-Gebühren.

Sie können benutzerdefinierte Cloud-Umgebungen mit verschiedenen Netzwerkzugriffsstufen und Umgebungsvariablen erstellen. Wenn Sie eine Cloud-Sitzung starten, öffnen Sie das Umgebungs-Dropdown im Eingabefeld, um diese zu verwalten:

* **Eine Umgebung hinzufügen**: wählen Sie **Cloud-Umgebung hinzufügen**
* **Eine Ihrer eigenen Umgebungen bearbeiten oder archivieren**: fahren Sie mit der Maus darüber und klicken Sie auf das Zahnrad-Symbol

Siehe [Cloud-Umgebungen konfigurieren](/docs/de/cloud-environments) für Details zur Konfiguration von Netzwerkzugriff und Umgebungsvariablen.

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

Administratoren können SSH-Verbindungen an Teammitglieder verteilen, indem sie `sshConfigs` zu einer [verwalteten Einstellungsdatei](/docs/de/managed-settings) hinzufügen. Auf diese Weise definierte Verbindungen werden in der Umgebungs-Dropdown-Liste jedes Benutzers automatisch angezeigt und sind als verwaltet gekennzeichnet, sodass Benutzer sie auswählen, aber nicht bearbeiten oder löschen können.

Das folgende Beispiel konfiguriert eine einzelne Verbindung vor:

```json theme={null}
{
  "sshConfigs": [
    {
      "id": "shared-dev-vm",
      "name": "Shared Dev VM",
      "sshHost": "user@dev.example.com",
      "sshPort": 22,
      "sshIdentityFile": "~/.ssh/id_ed25519"
    }
  ]
}
```

Jeder Eintrag erfordert `id`, `name` und `sshHost`. Die Felder `sshPort` und `sshIdentityFile` sind optional. Benutzer können auch `sshConfigs` zu ihrer eigenen `~/.claude/settings.json` hinzufügen, wo Verbindungen, die über den Dialog hinzugefügt werden, gespeichert sind.

<h4 id="restrict-which-ssh-hosts-users-can-connect-to">
  SSH-Hosts einschränken, mit denen Benutzer sich verbinden können
</h4>

Administratoren können Desktop-SSH-Sitzungen auf einen genehmigten Satz von Hosts beschränken, indem sie `sshHostAllowlist` zu einer [verwalteten Einstellungsdatei](/docs/de/managed-settings) hinzufügen. Wenn diese festgelegt ist, können Benutzer sich nur mit Hosts verbinden, deren aufgelöster Hostname einem der Muster entspricht. Setzen Sie es auf ein leeres Array, um SSH-Sitzungen vollständig zu deaktivieren.

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
* **Code im Web**: Aktivieren oder deaktivieren Sie [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) für Ihre Organisation
* **Remote Control**: Aktivieren oder deaktivieren Sie [Remote Control](/docs/de/remote-control) für Ihre Organisation
* **Bypass-Berechtigungsmodus deaktivieren**: Verhindern Sie, dass Benutzer in Ihrer Organisation den Bypass-Berechtigungsmodus aktivieren

<h3 id="managed-settings">
  Verwaltete Einstellungen
</h3>

Verwaltete Einstellungen überschreiben Projekt- und Benutzereinstellungen und gelten für Claude-Code-Sitzungen in Desktop. Sie können diese Schlüssel in der [verwalteten Einstellungsdatei](/docs/de/managed-settings) Ihrer Organisation oder remote über die Admin-Konsole festlegen.

| Schlüssel                                  | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissions.disableBypassPermissionsMode` | auf `"disable"` setzen, um Benutzer daran zu hindern, den Bypass-Berechtigungsmodus zu aktivieren.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `disableAutoMode`                          | auf `"disable"` setzen, um [Auto](/docs/de/permission-modes#eliminate-prompts-with-auto-mode)-Modus aus dem Moduswahlschalter zu entfernen. Auch unter `permissions` akzeptiert.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `autoMode`                                 | passen Sie an, was der Auto-Modus-Klassifizierer über Ihre Organisation vertraut und blockiert. Siehe [Auto-Modus konfigurieren](/docs/de/auto-mode-config).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `browserExternalPageTools`                 | auf `"disabled"` setzen, um zu verhindern, dass Claude Tools verwendet, um externe Seiten im [Browser-Bereich](#browse-external-sites) zu lesen oder zu bearbeiten. Benutzer können externe Websites weiterhin selbst navigieren, und lokale Dev-Server-Vorschauen sind nicht betroffen.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `disableMobileSimulatorTools`              | auf `true` setzen, um Claudes Tools zum Steuern und Erfassen von Geräten im [iOS-Simulator-Bereich](/docs/de/desktop-ios-simulator#turn-off-simulator-access) zu blockieren. Der Bereich bleibt für die eigenen Taps des Benutzers nutzbar; nur Claudes Zugriff wird entfernt. Der Wert muss der JSON-Boolean `true` sein; der String `"true"` wird ignoriert.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `disableBrowserExternalNavigation`         | auf `true` setzen, um externe Browsing im [Browser-Bereich](#browse-external-sites) vollständig auszuschalten. Weder Benutzer noch Claude können zu externen Websites navigieren, und localhost Dev-Server-Vorschauen sind nicht betroffen. Der Wert muss der JSON-Boolean `true` sein; der String `"true"` wird ignoriert.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `sshConfigs`                               | vorkonfigurieren Sie [SSH-Verbindungen](#pre-configure-ssh-connections-for-your-team), die in der Umgebungs-Dropdown angezeigt werden. Benutzer können verwaltete Verbindungen nicht bearbeiten oder löschen.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `sshHostAllowlist`                         | beschränken Sie [SSH-Sitzungen](#restrict-which-ssh-hosts-users-can-connect-to) auf Hosts, deren aufgelöster Hostname einem dieser Muster entspricht. Ein leeres Array deaktiviert SSH-Sitzungen. Wird nur aus verwalteten Einstellungen gelesen.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `disableDesktopLocalSessions`              | auf `true` setzen, um [Code-Sitzungen, die auf dem Gerät ausgeführt werden](#local-sessions-on-managed-devices), auszuschalten und SSH-Sitzungen zu anderen Hosts sowie Cloud-Sitzungen verfügbar zu lassen. Der Wert muss der JSON-Boolean `true` sein. Wird nur aus verwalteten Einstellungen gelesen. Erfordert Claude Desktop v1.37937.0 oder später.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `managedMcpServers`                        | übertragen Sie MCP-Serverkonfigurationen an alle Benutzer. Nur in Drittanbieter-Desktop-Bereitstellungen (3P) verfügbar. Geben Sie in jedem Eintrag einen Transport von `"http"`, `"sse"` oder `"stdio"`, Verbindungsdetails und optional eine `toolPolicy`-Zuordnung an, um einzuschränken, welche Tools dieses Servers Benutzer aufrufen können. Stellen Sie diesen Schlüssel über die verwaltete Einstellungsdatei, MDM oder eine Claude-Apps-Gateway-Richtlinie [`desktop`-Block](/docs/de/claude-apps-gateway-config#claude-desktop-overlay) bereit, da Drittanbieter-Bereitstellungen keine Admin-Konsolen-Einstellungen erhalten. Um diesen Schlüssel über das Gateway bereitzustellen, benötigen Sie Claude Code v2.1.232 oder später auf dem Gateway-Server. Dies ist der eigene Schlüssel der Desktop-App; Claude Code liest eine [gleichnamige verwaltete Einstellung](/docs/de/managed-mcp#provide-servers-through-managed-settings) mit einer anderen Eintragsform. |

Welche verwalteten Einstellungen eine Desktop-Sitzung erreichen, hängt davon ab, wo diese Sitzung ausgeführt wird. Modellbeschränkungen wie [`availableModels`](/docs/de/model-config#restrict-model-selection) werden in Desktop-Claude-Code-Sitzungen auf die gleiche Weise durchgesetzt wie in der Terminal-CLI; siehe [Oberflächenabdeckung](/docs/de/model-config#surface-coverage).

* **Lokale Sitzungen auf diesem Computer**: Eine verwaltete Einstellungsdatei, die auf der Festplatte bereitgestellt wird, gilt. Verwaltete Einstellungen, die remote über die Admin-Konsole hochgeladen werden, erreichen diese Sitzungen auch auf Anthropics API, wenn sich die Sitzung mit einer [berechtigten Anmeldung oder einem Schlüssel](/docs/de/server-managed-settings#platform-availability) authentifiziert, und folgen dabei der gleichen [Einstellungspriorität](/docs/de/settings#settings-precedence) wie die Terminal-CLI.
* **[Cloud-Sitzungen](#cloud-sessions)**: erhalten [Server-verwaltete Einstellungen](/docs/de/server-managed-settings); Gerätedateien erreichen diese nicht, da sie auf von Anthropic verwalteten VMs ausgeführt werden. Sitzungen, die zu einer [selbstgehosteten Umgebung](/docs/de/self-hosted-environments) weitergeleitet werden, lesen auch die verwaltete Einstellungsdatei im Runner-Image. [Wie Claude Code verwaltete Quellen kombiniert](/docs/de/managed-settings#how-claude-code-combines-managed-sources) gibt an, wann diese Datei gilt.
* **[SSH-Sitzungen](#ssh-sessions)**: Die Sitzung liest die verwaltete Einstellungsdatei vom Remote-Host. Desktop selbst liest `sshConfigs`, `sshHostAllowlist` und `disableDesktopLocalSessions` aus den verwalteten Einstellungen des lokalen Computers.
* **[Cowork](https://claude.com/docs/cowork/overview)-Sitzungen**: In einer Cowork-Sitzung auf diesem Computer ruft Claude Code niemals Admin-Konsolen-Einstellungen ab, auch wenn sich der Benutzer mit einem Team- oder Enterprise-Konto anmeldet, und liest Richtlinien, die auf dem Computer bereitgestellt werden, es sei denn, Ihre Claude-Desktop-Konfiguration setzt `requireCoworkFullVmSandbox`. Remote-Cowork-Sitzungen erhalten keine. Siehe [wo und wann eine Richtlinie gilt](/docs/de/managed-settings#where-and-when-a-policy-applies), um zu erfahren, welche Gerätedateien Cowork erreichen, und [MCP-Berechtigungsregeln](/docs/de/permissions#mcp), um zu erfahren, wie `Bash`- und `WebFetch`-Regeln auf Coworks Tools angewendet werden.

In lokalen und SSH-Sitzungen stellt die Desktop-App die verbundenen claude.ai-Konnektoren jedes Benutzers direkt an Claude Code. Keine MCP-Einstellung oder `managed-mcp.json` erreicht diese Konnektoren, unabhängig davon, welche Einstellungsquelle oder Dateispeicherort Sie verwenden. Um die Tools eines Konnektors in diesen Sitzungen zu blockieren, verwenden Sie die [Konnektoren-Tool-Steuerelemente](/docs/de/mcp#organization-controls-on-connector-tools) Ihrer Organisation. [Wie Konnektoren Claude Code erreichen](/docs/de/mcp#how-connectors-reach-claude-code) zeigt, welche Einstellungen Konnektoren in jeder Art von Sitzung steuern.

`permissions.disableBypassPermissionsMode` und `disableAutoMode` funktionieren auch in Benutzer- und Projekteinstellungen, aber das Platzieren in verwalteten Einstellungen verhindert, dass Benutzer sie überschreiben.

Für die Berechtigungs-, Plugin- und Bereitstellungsschlüssel, die nur eine verwaltete Quelle setzen kann, siehe [Schlüssel, die nur verwaltete Einstellungen setzen können](/docs/de/managed-settings#managed-only-settings).

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

Wenn Ihre Organisation [IP-Allowlisting](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting) für Claude aktiviert hat, leiten Sie `bridge.claudeusercontent.com` durch denselben Proxy-Ausgang wie `claude.ai` und `api.anthropic.com`. Wenn Sie dies nicht auf diese Weise leiten können, fügen Sie die Ausgangsadresse, die Ihr Proxy für diesen Host verwendet, zur IP-Allowlist Ihrer Organisation hinzu, aber nur, wenn diese Adresse Ihrer Organisation gewidmet ist: Ein gemeinsamer Proxy-Ausgangsbereich lässt auch andere Kunden des Proxy-Anbieters zu.

Anthropic überprüft Verbindungen zu diesem Host anhand der IP-Allowlist Ihrer Organisation unter Verwendung der Adresse, von der sie ankommen. Wenn Ihr Proxy Datenverkehr dafür über eine Adresse sendet, die nicht auf dieser Allowlist steht, funktionieren Claude in Chrome und andere Funktionen, die sich über die Bridge verbinden, nicht mehr, während der Rest der App weiterhin funktioniert.

Ein [Artefakt](/docs/de/artifacts), das eine Schriftart von [Google Fonts](/docs/de/artifacts#improve-the-visual-design) lädt, fordert auch `fonts.googleapis.com` und `fonts.gstatic.com` an. Beide Hosts sind optional. Wenn Sie diese blockieren, werden Artefakte in Fallback-Schriftarten gerendert. Blockieren Sie mit einer schnellen Ablehnung statt eines stillen Verwerfens, damit die Schriftartanforderung sofort fehlschlägt, anstatt das erste Rendering der Seite zu verzögern.

Artefakte können auch JavaScript-Bibliotheken wie React oder ein Charting-Paket von `cdnjs.cloudflare.com`, `cdn.jsdelivr.net`, `cdn.tailwindcss.com`, `code.jquery.com` und `unpkg.com` laden und von keinem anderen externen Host. Wenn Sie diese Hosts blockieren, funktionieren die Teile eines Artefakts, die von einer Bibliothek abhängen, nicht, und im Gegensatz zu einer blockierten Schriftart hat eine blockierte Bibliothek keinen Fallback. Blockieren Sie auch hier mit einer schnellen Ablehnung, damit eine blockierte Bibliotheksanforderung sofort fehlschlägt, anstatt zu hängen, bis sie abläuft.

<h3 id="authentication-and-sso">
  Authentifizierung und SSO
</h3>

Enterprise-Organisationen können SSO für alle Benutzer verlangen. Siehe [Authentifizierung](/docs/de/authentication) für Plan-Level-Details und [Einrichten von SSO](https://support.claude.com/en/articles/13132885-setting-up-single-sign-on-sso) für SAML-Konfiguration; OIDC-Setup wird im [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide) behandelt.

<h3 id="data-handling">
  Datenbehandlung
</h3>

Claude Code verarbeitet Ihren Code lokal in lokalen Sitzungen oder in Cloud-Sitzungen auf von Anthropic verwalteter Infrastruktur, es sei denn, Ihre Organisation leitet diese zu einer [selbstgehosteten Umgebung](/docs/de/self-hosted-environments) weiter. Cloud-Sitzungen, einschließlich in einer selbstgehosteten Umgebung, senden Gespräche und Code-Kontext an Anthropics API zur Verarbeitung; lokale und SSH-Sitzungen senden diese an den [Modell-Provider](#feature-comparison), den Ihre Bereitstellung konfiguriert, standardmäßig Anthropics API. Siehe [Datenbehandlung](/docs/de/data-usage) für Details zu Datenspeicherung, Datenschutz und Compliance.

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

Wenn Sie bereits die Claude Code CLI verwenden, führt Desktop dieselbe zugrunde liegende Engine mit einer grafischen Benutzeroberfläche aus. Sie können beide gleichzeitig auf demselben Computer ausführen, sogar auf demselben Projekt. Jede behält ihre eigene Sitzungsliste, und Sie können eine CLI-Sitzung in Desktop bringen. Sie teilen Konfiguration und Projektgedächtnis über CLAUDE.md-Dateien.

Um eine CLI-Sitzung in Desktop zu verschieben, führen Sie `/desktop` im Terminal aus. Claude speichert Ihre Sitzung und öffnet sie in der Desktop-App, dann beendet die CLI. Dieser Befehl ist auf macOS und x64 Windows verfügbar, wenn Sie mit einem Claude-Abonnement angemeldet sind. Er ist nicht mit API-Schlüssel-Authentifizierung oder auf Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry verfügbar.

Um eine CLI-Sitzung von innen in Desktop aufzugreifen, geben Sie stattdessen `/resume` in das Eingabefeld ein. Desktop listet die Sitzungen auf, die Sie von der CLI gestartet haben, und Sie können sie nach Titel, Ordner oder Branch durchsuchen und eine Vorschau sehen, wo jede endete. Wählen Sie eine Sitzung aus und sie wird in der App mit ihrer vollständigen Konversation und ihrem Kontext fortgesetzt.

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
| `--resume`, `--continue`                | Klicken Sie auf eine Sitzung in der Seitenleiste, oder geben Sie `/resume` in das Eingabefeld ein, um eine Sitzung aufzugreifen, die Sie von der CLI gestartet haben                                                         |
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
* **Modelle**: die gleichen [Modelle](/docs/de/model-config#available-models) sind in beiden verfügbar. In Desktop wählen Sie das Modell aus dem Dropdown neben der Schaltfläche „Senden". Sie können das Modell während der Sitzung ändern.

<h4 id="mcp-servers-from-the-claude-desktop-chat-app">
  MCP-Server aus der Claude Desktop Chat-App
</h4>

Die Desktop-App lädt MCP-Server aus `claude_desktop_config.json` in lokale Code-Tab-Sitzungen, zusammen mit Servern aus `~/.claude.json` und `.mcp.json`. Ein Server, der in `claude_desktop_config.json` definiert ist, ist sowohl auf der Desktop-Chat-Oberfläche als auch auf lokalen Code-Tab-Sitzungen verfügbar.

Wenn Sie denselben Servernamen in `claude_desktop_config.json` und in `~/.claude.json` oder `.mcp.json` definieren, verbindet sich die Code-Registerkarte in lokalen Sitzungen einmal und verwendet die `claude_desktop_config.json`-Definition.

Die App liefert auch stdio-Server aus `~/.claude.json` an die eingebettete CLI in lokalen Sitzungen erneut. Wenn die oberste Ebene von `~/.claude.json` (Benutzerbereich) und `.mcp.json` denselben stdio-Servernamen definieren, verwendet die Code-Registerkarte die `~/.claude.json`-Definition und weicht von der CLI [Bereichshierarchie](/docs/de/mcp#scope-hierarchy-and-precedence) ab.

<Note>
  Die eigenständige CLI liest `claude_desktop_config.json` nicht. Führen Sie auf macOS und WSL `claude mcp add-from-claude-desktop` aus, um diese Server in `~/.claude.json` zu kopieren. Siehe [MCP-Server aus Claude Desktop importieren](/docs/de/mcp#import-mcp-servers-from-claude-desktop) für den Importablauf und Bereichsoptionen.
</Note>

<h3 id="feature-comparison">
  Funktionsvergleich
</h3>

Diese Tabelle vergleicht Kernfunktionen zwischen CLI und Desktop. Für eine vollständige Liste der CLI-Flags siehe die [CLI-Referenz](/docs/de/cli-reference).

| Funktion                                               | CLI                                                                                      | Desktop                                                                                                                                                                                                                                                                                                                                                                                 |
| ------------------------------------------------------ | ---------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Berechtigungsmodi                                      | Alle Modi einschließlich `dontAsk`                                                       | Manuell, Bearbeitungen akzeptieren, Plan und Auto. Bypass-Berechtigungen erscheinen im Moduswahlschalter, sobald aktiviert: über den Einstellungsschalter auf Pro- und Max-Plänen oder über die Organisationsrichtlinie auf Team- und Enterprise-Plänen                                                                                                                                 |
| [Drittanbieter-Provider](/docs/de/third-party-integrations) | Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry                         | Anthropic's API standardmäßig. Für Gateway-Routing siehe [Desktop-App mit einem Gateway verbinden](/docs/de/llm-gateway-connect#desktop-app). Um die Code-Registerkarte auf Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry oder einem selbstgehosteten LLM-Gateway auszuführen, siehe [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview). |
| [MCP-Server](/docs/de/mcp)                                  | In Einstellungsdateien konfigurieren                                                     | Konnektoren-UI für lokale und SSH-Sitzungen oder Einstellungsdateien                                                                                                                                                                                                                                                                                                                    |
| [Plugins](/docs/de/plugins/overview)                        | `/plugin`-Befehl                                                                         | Plugin-Manager-UI                                                                                                                                                                                                                                                                                                                                                                       |
| @mention-Dateien                                       | Textbasiert                                                                              | Mit Autovervollständigung; lokale und SSH-Sitzungen nur                                                                                                                                                                                                                                                                                                                                 |
| Dateianhänge                                           | Nicht verfügbar                                                                          | Bilder, PDFs                                                                                                                                                                                                                                                                                                                                                                            |
| Sitzungsisolation                                      | [`--worktree`](/docs/de/cli-reference)-Flag                                                   | **worktree**-Option beim Starten einer Sitzung                                                                                                                                                                                                                                                                                                                                          |
| Mehrere Sitzungen                                      | Separate Terminals                                                                       | Seitenleisten-Tabs                                                                                                                                                                                                                                                                                                                                                                      |
| Wiederkehrende Aufgaben                                | Cron-Jobs, CI-Pipelines                                                                  | [Geplante Aufgaben](/docs/de/desktop-scheduled-tasks)                                                                                                                                                                                                                                                                                                                                        |
| Computernutzung                                        | [Aktivieren über `/mcp`](/docs/de/computer-use) auf macOS                                     | [App- und Bildschirmsteuerung](#let-claude-use-your-computer) auf macOS und Windows                                                                                                                                                                                                                                                                                                     |
| iOS-Simulator                                          | Steuern Sie den Simulator über [Computernutzung](/docs/de/computer-use#test-a-simulator-flow) | [iOS-Simulator-Bereich](/docs/de/desktop-ios-simulator) öffnet sich automatisch                                                                                                                                                                                                                                                                                                              |
| Dispatch-Integration                                   | Nicht verfügbar                                                                          | [Dispatch-Sitzungen](#sessions-from-dispatch) in der Seitenleiste                                                                                                                                                                                                                                                                                                                       |
| Scripting und Automatisierung                          | [`--print`](/docs/de/cli-reference), [Agent SDK](/docs/de/headless)                                | Nicht verfügbar                                                                                                                                                                                                                                                                                                                                                                         |

<h3 id="what’s-not-available-in-desktop">
  Was ist nicht in Desktop verfügbar
</h3>

Die folgenden Funktionen sind nicht in Desktop verfügbar, außer wo anders angegeben:

* **Drittanbieter-Provider**: Desktop verbindet sich mit Anthropic's API standardmäßig. Um Desktop durch ein Gateway zu leiten oder die Code-Registerkarte auf Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry oder einem selbstgehosteten LLM-Gateway auszuführen, folgen Sie den Links in der [Drittanbieter-Provider-Zeile](#feature-comparison).
* **Linux (Beta)**: Computernutzung ist noch nicht in der Linux-Desktop-App verfügbar. Siehe [Claude Desktop auf Linux](/docs/de/desktop-linux).
* **Inline-Code-Vorschläge**: Desktop bietet keine Autovervollständigungs-ähnlichen Vorschläge. Es funktioniert durch Gesprächseingaben und explizite Code-Änderungen.
* **Agent-Teams**: Koordinierte Teams, bei denen Claude als Teamleiter Aufgaben an Teamkollegen aus einer gemeinsamen Aufgabenliste zuweist, sind in der [CLI](/docs/de/agent-teams) verfügbar, nicht in Desktop. Für Multi-Agent-Arbeit innerhalb einer Sitzung verwenden Sie [dynamische Workflows](/docs/de/workflows), die in Desktop ausgeführt werden; Claude kann auch [Nachrichten senden und Ihre anderen Sitzungen verwalten](#work-across-sessions) direkt.
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

Sitzungen, die in ihrem eigenen Worktree ausgeführt werden, benötigen Git. Wenn Sie „Git is required" sehen, installieren Sie [Git](https://git-scm.com/downloads) oder [Git for Windows](https://git-scm.com/downloads/win) unter Windows und versuchen Sie es erneut. Unter Windows fragten Claude Desktop-Versionen vor 1.49585.0 nach Git, bevor eine lokale Sitzung gestartet wurde; wenn Sie diese Aufforderung sehen und keine Worktrees verwenden, aktualisieren Sie die App.

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

Wenn Sie einen Fehler melden, geben Sie Ihre Desktop-App-Version, Ihr Betriebssystem, die genaue Fehlermeldung und relevante Protokolle an. Überprüfen Sie auf macOS Console.app. Überprüfen Sie unter Windows Event Viewer → Windows Logs → Application. Überprüfen Sie Protokollauszüge, bevor Sie sie in einem öffentlichen Issue posten; sie können Dateipfade und andere Details aus Ihrer Umgebung enthalten.
