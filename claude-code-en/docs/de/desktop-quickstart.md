> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Erste Schritte mit der Desktop-App

> Installieren Sie Claude Code auf dem Desktop und starten Sie Ihre erste Coding-Sitzung

Die Desktop-App bietet Ihnen Claude Code mit einer grafischen Benutzeroberfläche, die für die Ausführung mehrerer Sitzungen nebeneinander konzipiert ist: eine Seitenleiste zur Verwaltung paralleler Arbeit, ein Drag-and-Drop-Layout mit integriertem Terminal und Datei-Editor, visuelle Diff-Überprüfung, Live-App-Vorschau, GitHub-PR-Überwachung mit automatischem Merge und geplante Aufgaben. Kein Terminal erforderlich.

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

<Note>
  Claude Code erfordert ein [Pro-, Max-, Team- oder Enterprise-Abonnement](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_pricing).
</Note>

Diese Seite führt Sie durch die Installation der App und den Start Ihrer ersten Sitzung. Wenn Sie bereits eingerichtet sind, siehe [Claude Code Desktop verwenden](/docs/de/desktop) für die vollständige Referenz.

Die Desktop-App hat drei Registerkarten:

* **Chat**: Allgemeine Konversation ohne Dateizugriff, ähnlich wie claude.ai.
* **Cowork**: Ein autonomer Hintergrund-Agent, der an Aufgaben in einer Sandbox-VM mit eigener Umgebung arbeitet und unabhängig läuft, während Sie andere Dinge tun. On-Device-Cowork-Sitzungen führen die VM auf Ihrem Computer aus; Remote-Cowork-Sitzungen führen stattdessen auf einer von Anthropic verwalteten VM aus.
* **Code**: Ein interaktiver Coding-Assistent mit direktem Zugriff auf Ihre lokalen Dateien. Abhängig vom Berechtigungsmodus genehmigen Sie jede Änderung, wenn Claude sie vorschlägt, oder überprüfen die Änderungen, nachdem Claude sie vorgenommen hat.

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

Öffnen Sie die Registerkarte Code, wählen Sie ein Projekt aus und geben Sie Claude eine Aufgabe.

<Steps>
  <Step title="Wählen Sie eine Umgebung und einen Ordner">
    Wählen Sie **Lokal**, um Claude auf Ihrem Computer mit Ihren Dateien direkt auszuführen. Klicken Sie auf **Ordner auswählen** und wählen Sie Ihr Projektverzeichnis.

    <Tip>
      Beginnen Sie mit einem kleinen Projekt, das Sie gut kennen. Das ist der schnellste Weg, um zu sehen, was Claude Code kann.
    </Tip>

    Sie können auch folgende Optionen wählen:

    * **Cloud**: Führen Sie Sitzungen in der Cloud aus, die auch dann fortgesetzt werden, wenn Sie die App schließen. Siehe [Claude Code in der Cloud verwenden](/docs/de/claude-code-on-the-web), um zu erfahren, wie Cloud-Sitzungen funktionieren.
    * **SSH**: Verbinden Sie sich über SSH mit einem Remote-Computer, z. B. mit Ihren eigenen Servern, Cloud-VMs oder Dev-Containern. Desktop installiert Claude Code beim ersten Verbinden automatisch auf dem Remote-Computer.
    * **WSL** (Windows): Führen Sie die Sitzung in einer [WSL 2-Distribution](/docs/de/desktop-wsl) aus; Claude Code, Tools und Git werden auf der Linux-Seite mit nativen Pfaden ausgeführt.
  </Step>

  <Step title="Wählen Sie ein Modell">
    Wählen Sie ein Modell aus dem Dropdown-Menü neben der Schaltfläche zum Senden. Siehe [Modelle](/docs/de/model-config#available-models) für einen Vergleich der verfügbaren Modelle. Sie können das Modell später über das gleiche Dropdown-Menü ändern.
  </Step>

  <Step title="Sagen Sie Claude, was zu tun ist">
    Geben Sie ein, was Claude tun soll:

    * `Find a TODO comment and fix it`
    * `Add tests for the main function`
    * `Create a CLAUDE.md with instructions for this codebase`

    Eine [Sitzung](/docs/de/desktop#work-in-parallel-with-sessions) ist ein Gespräch mit Claude über Ihren Code. Jede Sitzung verfolgt ihren eigenen Kontext und ihre Änderungen.
  </Step>

  <Step title="Überprüfen und akzeptieren Sie Änderungen">
    Was als Nächstes geschieht, hängt vom [Berechtigungsmodus](/docs/de/desktop#choose-a-permission-mode) ab, der in der Auswahl neben der Schaltfläche zum Senden angezeigt wird:

    * **Auto oder Änderungen akzeptieren**: Claude wendet seine Dateiänderungen an, und ein Indikator wie `+12 -1` wird angezeigt, damit Sie diese in der Diff-Ansicht überprüfen können
    * **Manuell**: Claude schlägt jede Änderung vor und wartet auf Ihre Genehmigung, bevor sie angewendet wird. Ihre Dateien werden erst geändert, wenn Sie akzeptieren. Wenn Sie eine Änderung ablehnen, fragt Claude, wie Sie stattdessen vorgehen möchten

    Im Manuellen Modus sehen Sie:

    1. Eine [Diff-Ansicht](/docs/de/desktop#review-changes-with-diff-view), die genau zeigt, was sich in jeder Datei ändert
    2. Schaltflächen zum Akzeptieren/Ablehnen, um jede Änderung zu genehmigen oder abzulehnen
    3. Echtzeit-Updates, während Claude Ihre Anfrage bearbeitet
  </Step>
</Steps>

<h2 id="now-what">
  Und jetzt?
</h2>

Sie haben Ihre erste Bearbeitung durchgeführt. Eine vollständige Referenz zu allem, was Claude Code Desktop kann, finden Sie unter [Claude Code Desktop verwenden](/docs/de/desktop). Hier sind einige Dinge, die Sie als Nächstes ausprobieren können.

**Unterbrechen und lenken.** Sie können Claude jederzeit umleiten. Klicken Sie auf die Stoppschaltfläche, um sofort zu unterbrechen, oder geben Sie eine Korrektur ein und drücken Sie **Eingabe**, um sie zu senden, ohne die laufende Aktion zu stoppen. In beiden Fällen müssen Sie nicht warten, bis sie abgeschlossen ist, oder von vorne beginnen.

**Geben Sie Claude mehr Kontext.** Geben Sie `@filename` in das Eingabefeld ein, um eine bestimmte Datei in das Gespräch zu ziehen, fügen Sie Bilder und PDFs mit der Schaltfläche „Anhang" an, oder ziehen Sie Dateien direkt per Drag-and-Drop in die Eingabe. Je mehr Kontext Claude hat, desto besser sind die Ergebnisse. Siehe [Dateien und Kontext hinzufügen](/docs/de/desktop#add-files-and-context-to-prompts).

**Verwenden Sie Skills für wiederholbare Aufgaben.** Geben Sie `/` ein oder klicken Sie auf **+** → **Slash commands**, um [integrierte Befehle](/docs/de/commands), [benutzerdefinierte Skills](/docs/de/skills) und Plugin-Skills zu durchsuchen. Skills sind wiederverwendbare Eingabeaufforderungen, die Sie jederzeit aufrufen können, z. B. Code-Review-Checklisten oder Bereitstellungsschritte.

**Überprüfen Sie Änderungen vor dem Commit.** Nachdem Claude Dateien bearbeitet hat, wird ein `+12 -1`-Indikator angezeigt. Klicken Sie darauf, um die [Diff-Ansicht](/docs/de/desktop#review-changes-with-diff-view) zu öffnen, überprüfen Sie Änderungen Datei für Datei und kommentieren Sie bestimmte Zeilen. Claude liest Ihre Kommentare und überarbeitet. Klicken Sie auf **Code überprüfen**, um Claude die Diffs selbst auswerten zu lassen und Inline-Vorschläge zu hinterlassen.

**Passen Sie an, wie viel Kontrolle Sie haben.** Ihr [Berechtigungsmodus](/docs/de/desktop#choose-a-permission-mode) bestimmt, wie viel Claude tun kann, ohne um Genehmigung zu fragen:

* **Auto**: Ein Klassifizierer überprüft Aktionen im Hintergrund und blockiert die riskanten, anstatt Sie zu fragen.
* **Manuell**: Claude fragt vor dem Bearbeiten von Dateien oder dem Ausführen von Befehlen.
* **Bearbeitungen akzeptieren**: Claude akzeptiert Dateibearbeitungen automatisch für schnellere Iteration.
* **Plan**: Claude schlägt einen Ansatz vor, ohne Dateien zu bearbeiten, was vor einem großen Refactoring nützlich ist.

**Fügen Sie Plugins für mehr Funktionen hinzu.** Klicken Sie auf die Schaltfläche **+** neben dem Eingabefeld und wählen Sie **Plugins**, um [Plugins](/docs/de/desktop#install-plugins) zu durchsuchen und zu installieren, die Skills, Agents, MCP-Server und mehr hinzufügen.

**Ordnen Sie Ihren Arbeitsbereich an.** Ziehen Sie die Chat-, Diff-, Terminal-, Datei- und Browser-Bereiche in das gewünschte Layout. Öffnen Sie das Terminal mit **Strg+\`**, um Befehle neben Ihrer Sitzung auszuführen, oder klicken Sie auf einen Dateipfad, um ihn im Dateibereich zu öffnen. Siehe [Arbeitsbereich anordnen](/docs/de/desktop#arrange-your-workspace).

**Zeigen Sie eine Vorschau Ihrer App an.** Wenn Sie Ihren Dev-Server auf dem Desktop ausführen, wird Ihre App im Browser-Bereich geöffnet, der auch [externe Websites öffnen](/docs/de/desktop#browse-external-sites) kann. Claude kann die laufende App anzeigen, Endpunkte testen, Protokolle überprüfen und auf das reagieren, was es sieht. Siehe [Zeigen Sie eine Vorschau Ihrer App an](/docs/de/desktop#preview-your-app).

**Verfolgen Sie Ihren Pull Request.** Nach dem Öffnen eines PR überwacht Claude Code die CI-Prüfungsergebnisse und kann Fehler automatisch beheben oder den PR zusammenführen, sobald alle Prüfungen bestanden sind. Siehe [Überwachen Sie den Pull-Request-Status](/docs/de/desktop#monitor-pull-request-status).

**Planen Sie Claude ein.** Richten Sie [geplante Aufgaben](/docs/de/desktop-scheduled-tasks) ein, um Claude automatisch regelmäßig auszuführen: eine tägliche Code-Überprüfung jeden Morgen, eine wöchentliche Abhängigkeitsprüfung oder eine Zusammenfassung, die Daten aus Ihren verbundenen Tools abruft.

**Skalieren Sie auf, wenn Sie bereit sind.** Öffnen Sie [parallele Sitzungen](/docs/de/desktop#work-in-parallel-with-sessions) aus der Seitenleiste, um mehrere Aufgaben gleichzeitig zu bearbeiten, jede in ihrem eigenen Git Worktree, und öffnen Sie den [Aufgabenbereich](/docs/de/desktop#watch-background-tasks), um die Subagents und Hintergrund-Befehle zu beobachten, die eine Sitzung ausführt. Öffnen Sie einen [Side Chat](/docs/de/desktop#ask-a-side-question-without-derailing-the-session), um eine Frage zu stellen, ohne den Hauptthread zu unterbrechen. Senden Sie [langfristige Arbeiten in die Cloud](/docs/de/desktop#run-long-running-tasks-in-the-cloud), damit sie fortgesetzt werden, auch wenn Sie die App schließen, oder [setzen Sie eine Sitzung im Web oder in Ihrer IDE fort](/docs/de/desktop#continue-in-another-surface), wenn eine Aufgabe länger als erwartet dauert. [Verbinden Sie externe Tools](/docs/de/desktop#extend-claude-code) wie GitHub, Slack und Linear, um Ihren Workflow zusammenzubringen.

<h2 id="what’s-next">
  Nächste Schritte
</h2>

* [Claude Code Desktop verwenden](/docs/de/desktop): Berechtigungsmodi, parallele Sitzungen, Diff-Ansicht, Konnektoren und Unternehmenskonfiguration
* [Vom CLI kommend?](/docs/de/desktop#coming-from-the-cli): Führen Sie Desktop und die CLI auf demselben Projekt aus, und vergleichen Sie Funktionen, Flag-Äquivalente und was in Desktop nicht verfügbar ist
* [Fehlerbehebung](/docs/de/desktop#troubleshooting): Lösungen für häufige Fehler und Setup-Probleme
* [Best Practices](/docs/de/best-practices): Tipps zum Schreiben effektiver Prompts und zum Optimieren von Claude Code
* [Häufige Workflows](/docs/de/common-workflows): Tutorials zum Debuggen, Refaktorieren, Testen und mehr
