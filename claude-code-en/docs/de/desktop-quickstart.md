> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Erste Schritte mit der Desktop-App

> Installieren Sie Claude Code auf dem Desktop und starten Sie Ihre erste Coding-Sitzung

Die Desktop-App bietet Ihnen Claude Code mit einer grafischen Benutzeroberfläche, die für die Ausführung mehrerer Sitzungen nebeneinander konzipiert ist: eine Seitenleiste zur Verwaltung paralleler Arbeit, ein Drag-and-Drop-Layout mit integriertem Terminal und Datei-Editor, visuelle Diff-Überprüfung, Live-App-Vorschau, GitHub-PR-Überwachung mit automatischem Merge und geplante Aufgaben. Kein Terminal erforderlich.

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

<Note>
  Claude Code erfordert ein [Pro-, Max-, Team- oder Enterprise-Abonnement](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_pricing).
</Note>

Diese Seite führt Sie durch die Installation der App und den Start Ihrer ersten Sitzung. Wenn Sie bereits eingerichtet sind, siehe [Claude Code Desktop verwenden](/docs/de/desktop) für die vollständige Referenz.

Die Desktop-App hat drei Registerkarten:

* **Chat**: Allgemeine Konversation ohne Dateizugriff, ähnlich wie claude.ai.
* **Cowork**: Ein autonomer Hintergrund-Agent, der an Aufgaben in einer Sandbox-VM mit eigener Umgebung arbeitet und unabhängig läuft, während Sie andere Dinge tun. On-Device-Cowork-Sitzungen führen die VM auf Ihrem Computer aus; Remote-Cowork-Sitzungen führen stattdessen auf einer von Anthropic verwalteten VM aus.
* **Code**: Ein interaktiver Coding-Assistent mit direktem Zugriff auf Ihre lokalen Dateien. Sie überprüfen und genehmigen jede Änderung in Echtzeit.

Chat und Cowork werden im [Claude Help Center](https://support.claude.com/) behandelt; die Installation und Bereitstellung der Desktop-App wird in den [Claude Desktop-Supportartikeln](https://support.claude.com/en/collections/16163169-claude-desktop) behandelt. Diese Seite konzentriert sich auf die Registerkarte **Code**.

<h2 id="install">
  Installieren
</h2>

<Steps>
  <Step title="Installieren und anmelden">
    Laden Sie das Installationsprogramm unter macOS und Windows über die obigen Links herunter und führen Sie es aus. Unter Linux folgen Sie den Installationsschritten in [Claude Desktop unter Linux](/docs/de/desktop-linux). Starten Sie Claude aus Ihrem Anwendungsordner unter macOS, dem Startmenü unter Windows oder Ihrem Anwendungsstarter unter Linux und melden Sie sich mit Ihrem Anthropic-Konto an.
  </Step>

  <Step title="Öffnen Sie die Registerkarte Code">
    Klicken Sie auf die Registerkarte **Code** oben in der Mitte. Wenn Sie beim Klicken auf „Code" aufgefordert werden, ein Upgrade durchzuführen, müssen Sie zunächst [ein bezahltes Abonnement abschließen](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_upgrade). Wenn Sie aufgefordert werden, sich online anzumelden, schließen Sie die Anmeldung ab und starten Sie die App neu. Wenn Sie einen 403-Fehler sehen, siehe [Authentifizierungsfehlersuche](/docs/de/desktop#403-or-authentication-errors-in-the-code-tab).
  </Step>
</Steps>

Die Desktop-App enthält Claude Code. Sie müssen Node.js oder die CLI nicht separat installieren. Um `claude` vom Terminal aus zu verwenden, installieren Sie die CLI separat. Siehe [Erste Schritte mit der CLI](/docs/de/quickstart).

<h2 id="start-your-first-session">
  Starten Sie Ihre erste Sitzung
</h2>

Wählen Sie mit der geöffneten Registerkarte „Code" ein Projekt aus und geben Sie Claude etwas zu tun.

<Steps>
  <Step title="Wählen Sie eine Umgebung und einen Ordner">
    Wählen Sie **Lokal**, um Claude auf Ihrem Computer mit Ihren Dateien direkt auszuführen. Klicken Sie auf **Ordner auswählen** und wählen Sie Ihr Projektverzeichnis.

    <Tip>
      Beginnen Sie mit einem kleinen Projekt, das Sie gut kennen. Es ist die schnellste Möglichkeit zu sehen, was Claude Code kann. Unter Windows muss [Git](https://git-scm.com/downloads/win) für lokale Sitzungen installiert sein. Die meisten Macs enthalten Git standardmäßig.
    </Tip>

    Sie können auch auswählen:

    * **Remote**: Führen Sie Sitzungen auf der Cloud-Infrastruktur von Anthropic aus, die auch dann fortgesetzt werden, wenn Sie die App schließen. Cloud-Sitzungen verwenden die gleiche Infrastruktur wie [Claude Code im Web](/docs/de/claude-code-on-the-web).
    * **SSH**: Verbinden Sie sich über SSH mit einem Remote-Computer, z. B. Ihren eigenen Servern, Cloud-VMs oder Dev-Containern. Desktop installiert Claude Code beim ersten Verbindungsaufbau automatisch auf dem Remote-Computer.
    * **WSL** (Windows): Führen Sie die Sitzung in einer [WSL 2-Distribution](/docs/de/desktop-wsl) aus; Claude Code, Tools und Git werden auf der Linux-Seite mit nativen Pfaden ausgeführt.
  </Step>

  <Step title="Wählen Sie ein Modell">
    Wählen Sie ein Modell aus der Dropdown-Liste neben der Schaltfläche „Senden". Siehe [Modelle](/docs/de/model-config#available-models) für einen Vergleich der verfügbaren Modelle. Sie können das Modell später aus der gleichen Dropdown-Liste ändern.
  </Step>

  <Step title="Sagen Sie Claude, was zu tun ist">
    Geben Sie ein, was Claude tun soll:

    * `Find a TODO comment and fix it`
    * `Add tests for the main function`
    * `Create a CLAUDE.md with instructions for this codebase`

    Eine [Sitzung](/docs/de/desktop#work-in-parallel-with-sessions) ist eine Konversation mit Claude über Ihren Code. Jede Sitzung verfolgt ihren eigenen Kontext und ihre Änderungen, sodass Sie an mehreren Aufgaben arbeiten können, ohne dass sie sich gegenseitig beeinflussen.
  </Step>

  <Step title="Überprüfen und akzeptieren Sie Änderungen">
    Standardmäßig startet die Registerkarte „Code" im [Modus „Berechtigungen erfragen"](/docs/de/desktop#choose-a-permission-mode), in dem Claude Änderungen vorschlägt und auf Ihre Genehmigung wartet, bevor er sie anwendet. Sie sehen:

    1. Eine [Diff-Ansicht](/docs/de/desktop#review-changes-with-diff-view), die genau zeigt, was sich in jeder Datei ändern wird
    2. Schaltflächen „Akzeptieren"/„Ablehnen", um jede Änderung zu genehmigen oder abzulehnen
    3. Echtzeit-Updates, während Claude Ihre Anfrage bearbeitet

    Wenn Sie eine Änderung ablehnen, fragt Claude, wie Sie anders vorgehen möchten. Ihre Dateien werden erst geändert, wenn Sie sie akzeptieren.
  </Step>
</Steps>

<h2 id="now-what">
  Was nun?
</h2>

Sie haben Ihre erste Bearbeitung vorgenommen. Für die vollständige Referenz zu allem, was Desktop kann, siehe [Claude Code Desktop verwenden](/docs/de/desktop). Hier sind einige Dinge, die Sie als Nächstes versuchen können.

**Unterbrechen und lenken.** Sie können Claude jederzeit unterbrechen. Klicken Sie auf die Stoppschaltfläche, um sofort zu unterbrechen, oder geben Sie eine Korrektur ein und drücken Sie **Eingabe**, um sie zu senden, ohne die laufende Aktion zu stoppen. In jedem Fall müssen Sie nicht warten, bis sie fertig ist, oder von vorne anfangen.

**Geben Sie Claude mehr Kontext.** Geben Sie `@filename` im Eingabefeld ein, um eine bestimmte Datei in die Konversation zu ziehen, fügen Sie Bilder und PDFs mit der Schaltfläche „Anhang" an, oder ziehen Sie Dateien direkt in das Eingabefeld. Je mehr Kontext Claude hat, desto besser sind die Ergebnisse. Siehe [Dateien und Kontext zu Eingaben hinzufügen](/docs/de/desktop#add-files-and-context-to-prompts).

**Verwenden Sie Skills für wiederholbare Aufgaben.** Geben Sie `/` ein oder klicken Sie auf **+** → **Slash commands**, um [integrierte Befehle](/docs/de/commands), [benutzerdefinierte Skills](/docs/de/skills) und Plugin-Skills zu durchsuchen. Skills sind wiederverwendbare Eingaben, die Sie aufrufen können, wenn Sie sie benötigen, wie Code-Review-Checklisten oder Bereitstellungsschritte.

**Überprüfen Sie Änderungen vor dem Commit.** Nachdem Claude Dateien bearbeitet hat, wird ein `+12 -1`-Indikator angezeigt. Klicken Sie darauf, um die [Diff-Ansicht](/docs/de/desktop#review-changes-with-diff-view) zu öffnen, überprüfen Sie Änderungen Datei für Datei und kommentieren Sie bestimmte Zeilen. Claude liest Ihre Kommentare und überarbeitet. Klicken Sie auf **Code überprüfen**, um Claude die Diffs selbst auswerten zu lassen und Inline-Vorschläge zu hinterlassen.

**Passen Sie an, wie viel Kontrolle Sie haben.** Ihr [Berechtigungsmodus](/docs/de/desktop#choose-a-permission-mode) steuert, wie viel Claude ohne Genehmigung tun kann:

* **Manuell**: die Standardeinstellung. Claude fragt vor dem Bearbeiten von Dateien oder dem Ausführen von Befehlen.
* **Bearbeitungen akzeptieren**: Claude akzeptiert Dateibearbeitungen automatisch für schnellere Iteration.
* **Plan**: Claude schlägt einen Ansatz vor, ohne Dateien zu bearbeiten, was vor einem großen Refactoring nützlich ist.

**Fügen Sie Plugins für mehr Funktionen hinzu.** Klicken Sie auf die Schaltfläche **+** neben dem Eingabefeld und wählen Sie **Plugins**, um [Plugins](/docs/de/desktop#install-plugins) zu durchsuchen und zu installieren, die Skills, Agents, MCP servers und mehr hinzufügen.

**Arrangieren Sie Ihren Arbeitsbereich.** Ziehen Sie die Chat-, Diff-, Terminal-, Datei- und Browser-Bereiche in das Layout, das Sie möchten. Öffnen Sie das Terminal mit **Strg+\`**, um Befehle neben Ihrer Sitzung auszuführen, oder klicken Sie auf einen Dateipfad, um ihn im Datei-Bereich zu öffnen. Siehe [Arrangieren Sie Ihren Arbeitsbereich](/docs/de/desktop#arrange-your-workspace).

**Zeigen Sie eine Vorschau Ihrer App an.** Wenn Sie Ihren Dev-Server im Desktop ausführen, öffnet sich Ihre App im Browser-Bereich, der auch [externe Websites öffnen](/docs/de/desktop#browse-external-sites) kann. Claude kann die laufende App anzeigen, Endpunkte testen, Protokolle überprüfen und auf das, was es sieht, iterieren. Siehe [Zeigen Sie eine Vorschau Ihrer App an](/docs/de/desktop#preview-your-app).

**Verfolgen Sie Ihren Pull Request.** Nachdem Sie einen PR geöffnet haben, überwacht Claude Code die CI-Prüfungsergebnisse und kann Fehler automatisch beheben oder den PR zusammenführen, sobald alle Prüfungen bestanden sind. Siehe [Überwachen Sie den Pull-Request-Status](/docs/de/desktop#monitor-pull-request-status).

**Setzen Sie Claude auf einen Zeitplan.** Richten Sie [geplante Aufgaben](/docs/de/desktop-scheduled-tasks) ein, um Claude automatisch regelmäßig auszuführen: eine tägliche Code-Überprüfung jeden Morgen, eine wöchentliche Abhängigkeitsprüfung oder eine Zusammenfassung, die von Ihren verbundenen Tools abruft.

**Skalieren Sie auf, wenn Sie bereit sind.** Öffnen Sie [parallele Sitzungen](/docs/de/desktop#work-in-parallel-with-sessions) aus der Seitenleiste, um an mehreren Aufgaben gleichzeitig zu arbeiten, jede in ihrem eigenen Git worktree, und öffnen Sie den [Aufgaben-Bereich](/docs/de/desktop#watch-background-tasks), um die Subagents und Hintergrund-Befehle zu beobachten, die eine Sitzung ausführt. Öffnen Sie einen [Side Chat](/docs/de/desktop#ask-a-side-question-without-derailing-the-session), um eine Frage zu stellen, ohne den Hauptthread zu unterbrechen. Senden Sie [langfristige Arbeit in die Cloud](/docs/de/desktop#run-long-running-tasks-remotely), damit sie auch dann fortgesetzt wird, wenn Sie die App schließen, oder [setzen Sie eine Sitzung im Web oder in Ihrer IDE fort](/docs/de/desktop#continue-in-another-surface), wenn eine Aufgabe länger als erwartet dauert. [Verbinden Sie externe Tools](/docs/de/desktop#extend-claude-code) wie GitHub, Slack und Linear, um Ihren Workflow zusammenzubringen.

<h2 id="coming-from-the-cli">
  Kommen Sie von der CLI?
</h2>

Desktop führt die gleiche Engine wie die CLI mit einer grafischen Benutzeroberfläche aus. Sie können beide gleichzeitig auf dem gleichen Projekt ausführen, und sie teilen die Konfiguration (CLAUDE.md-Dateien, MCP servers, hooks, Skills und Einstellungen). Für einen vollständigen Vergleich von Funktionen, Flag-Äquivalenten und was in Desktop nicht verfügbar ist, siehe [CLI-Vergleich](/docs/de/desktop#coming-from-the-cli).

<h2 id="what’s-next">
  Was kommt als Nächstes
</h2>

* [Claude Code Desktop verwenden](/docs/de/desktop): Berechtigungsmodi, parallele Sitzungen, Diff-Ansicht, Konnektoren und Enterprise-Konfiguration
* [Fehlerbehebung](/docs/de/desktop#troubleshooting): Lösungen für häufige Fehler und Setup-Probleme
* [Best Practices](/docs/de/best-practices): Tipps zum Schreiben effektiver Eingaben und zum Herausholen des Besten aus Claude Code
* [Häufige Workflows](/docs/de/common-workflows): Tutorials zum Debuggen, Refactoring, Testen und mehr
