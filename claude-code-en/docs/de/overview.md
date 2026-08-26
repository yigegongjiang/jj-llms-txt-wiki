> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Übersicht

> Claude Code ist ein agentengestütztes Codierungswerkzeug, das Ihre Codebasis liest, Dateien bearbeitet, Befehle ausführt und sich in Ihre Entwicklungstools integriert. Verfügbar in Ihrem Terminal, IDE, Desktop-App und Browser.

Claude Code ist ein KI-gestützter Codierassistent, der Ihnen hilft, Funktionen zu erstellen, Fehler zu beheben und Entwicklungsaufgaben zu automatisieren. Er versteht Ihre gesamte Codebasis und kann über mehrere Dateien und Tools hinweg arbeiten, um Aufgaben zu erledigen.

<h2 id="get-started">
  Erste Schritte
</h2>

Claude Code läuft auf mehreren Oberflächen: dem Terminal, IDE-Erweiterungen, einer Desktop-App und dem Web. Wählen Sie einen aus den Registerkarten unten aus, um zu beginnen. Die meisten Oberflächen erfordern ein [Claude-Abonnement](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=overview_pricing) oder ein [Anthropic Console](https://console.anthropic.com/)-Konto. Das Terminal CLI und VS Code unterstützen auch [Drittanbieter](/docs/de/third-party-integrations).

<Tabs>
  <Tab title="Terminal">
    Das vollständig ausgestattete CLI für die Arbeit mit Claude Code direkt in Ihrem Terminal. Bearbeiten Sie Dateien, führen Sie Befehle aus und verwalten Sie Ihr gesamtes Projekt über die Befehlszeile.

    To install Claude Code, use one of the following methods:

    <Tabs>
      <Tab title="Native Install (Recommended)">
        **macOS, Linux, WSL:**

        ```bash theme={null}
        curl -fsSL https://claude.ai/install.sh | bash
        ```

        **Windows PowerShell:**

        ```powershell theme={null}
        irm https://claude.ai/install.ps1 | iex
        ```

        **Windows CMD:**

        ```batch theme={null}
        curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
        ```

        If you see `The token '&&' is not a valid statement separator`, you're in PowerShell, not CMD. If you see `'irm' is not recognized as an internal or external command`, you're in CMD, not PowerShell. Your prompt shows `PS C:\` when you're in PowerShell and `C:\` without the `PS` when you're in CMD.

        If the install command fails with `syntax error near unexpected token '<'`, a `403`, or another curl error, see [Troubleshoot installation](/docs/en/troubleshoot-install#find-your-error) to match the error to a fix and for alternative install methods.

        [Git for Windows](https://git-scm.com/downloads/win) is recommended on native Windows so Claude Code can use the Bash tool. If Git for Windows is not installed, Claude Code uses PowerShell as the shell tool instead. WSL setups do not need Git for Windows.

        <Info>
          Native installations automatically update in the background to keep you on the latest version.
        </Info>
      </Tab>

      <Tab title="Homebrew">
        ```bash theme={null}
        brew install --cask claude-code
        ```

        Homebrew offers two casks. `claude-code` tracks the stable release channel, which is typically about a week behind and skips releases with major regressions. `claude-code@latest` tracks the latest channel and receives new versions as soon as they ship.

        <Info>
          Homebrew installations do not auto-update. Run `brew upgrade claude-code` or `brew upgrade claude-code@latest`, depending on which cask you installed, to get the latest features and security fixes.
        </Info>
      </Tab>

      <Tab title="WinGet">
        ```powershell theme={null}
        winget install Anthropic.ClaudeCode
        ```

        <Info>
          WinGet installations do not auto-update. Run `winget upgrade Anthropic.ClaudeCode` periodically to get the latest features and security fixes.
        </Info>
      </Tab>
    </Tabs>

    You can also install with [apt, dnf, or apk](/docs/en/setup#install-with-linux-package-managers) on Debian, Fedora, RHEL, and Alpine.

    Starten Sie dann Claude Code in einem beliebigen Projekt:

    ```bash theme={null}
    cd your-project
    claude
    ```

    Sie werden beim ersten Mal aufgefordert, sich anzumelden. Das ist alles! [Fahren Sie mit dem Quickstart fort →](/docs/de/quickstart)

    <Tip>
      Siehe [Erweiterte Einrichtung](/docs/de/setup) für Installationsoptionen, manuelle Updates oder Deinstallationsanweisungen. Besuchen Sie [Fehlerbehebung bei der Installation](/docs/de/troubleshoot-install), wenn Sie auf Probleme stoßen.
    </Tip>
  </Tab>

  <Tab title="VS Code">
    Die VS Code-Erweiterung bietet Inline-Diffs, @-Erwähnungen, Planüberprüfung und Gesprächsverlauf direkt in Ihrem Editor.

    * [Für VS Code installieren](vscode:extension/anthropic.claude-code)
    * [Für Cursor installieren](cursor:extension/anthropic.claude-code)

    Oder suchen Sie nach „Claude Code" in der Ansicht „Erweiterungen" (`Cmd+Shift+X` auf Mac, `Ctrl+Shift+X` auf Windows/Linux). Nach der Installation öffnen Sie die Befehlspalette (`Cmd+Shift+P` / `Ctrl+Shift+P`), geben Sie „Claude Code" ein und wählen Sie **In neuem Tab öffnen**.

    [Erste Schritte mit VS Code →](/docs/de/vs-code#get-started)
  </Tab>

  <Tab title="Desktop-App">
    Eine eigenständige App für die Ausführung von Claude Code außerhalb Ihrer IDE oder Ihres Terminals. Überprüfen Sie Diffs visuell, führen Sie mehrere Sitzungen nebeneinander aus, planen Sie wiederkehrende Aufgaben und starten Sie Cloud-Sitzungen.

    Herunterladen und installieren:

    * [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs) (Intel und Apple Silicon)
    * [Windows](https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs) (x64)
    * [Windows ARM64](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs)

    Nach der Installation starten Sie Claude, melden Sie sich an und klicken Sie auf die Registerkarte **Code**, um mit dem Codieren zu beginnen. Ein [bezahltes Abonnement](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=overview_desktop_pricing) ist erforderlich.

    [Weitere Informationen zur Desktop-App →](/docs/de/desktop-quickstart)
  </Tab>

  <Tab title="Web">
    Führen Sie Claude Code in Ihrem Browser ohne lokale Einrichtung aus. Starten Sie lang laufende Aufgaben und überprüfen Sie sie später, arbeiten Sie an Repositories, die Sie nicht lokal haben, oder führen Sie mehrere Aufgaben parallel aus. Verfügbar auf Desktop-Browsern und der Claude iOS-App.

    Beginnen Sie mit dem Codieren unter [claude.ai/code](https://claude.ai/code).

    [Erste Schritte im Web →](/docs/de/web-quickstart)
  </Tab>

  <Tab title="JetBrains">
    Ein Plugin für IntelliJ IDEA, PyCharm, WebStorm und andere JetBrains-IDEs mit interaktiver Diff-Anzeige und Auswahlkontext-Freigabe.

    Installieren Sie das [Claude Code-Plugin](https://plugins.jetbrains.com/plugin/27310-claude-code-beta-) aus dem JetBrains Marketplace und starten Sie Ihre IDE neu. Das Plugin erfordert das Claude Code CLI, das separat installiert wird; siehe die [JetBrains-Einrichtungsschritte](/docs/de/jetbrains#installation).

    [Erste Schritte mit JetBrains →](/docs/de/jetbrains)
  </Tab>
</Tabs>

<h2 id="what-you-can-do">
  Was Sie tun können
</h2>

Hier sind einige Möglichkeiten, wie Sie Claude Code nutzen können:

<AccordionGroup>
  <Accordion title="Automatisieren Sie die Arbeit, die Sie immer wieder aufschieben" icon="wand-magic-sparkles">
    Claude Code übernimmt die mühsamen Aufgaben, die Ihren Tag aufzehren: Schreiben von Tests für ungetesteten Code, Beheben von Lint-Fehlern in einem Projekt, Auflösen von Merge-Konflikten, Aktualisieren von Abhängigkeiten und Schreiben von Versionshinweisen.

    ```bash theme={null}
    claude "write tests for the auth module, run them, and fix any failures"
    ```
  </Accordion>

  <Accordion title="Erstellen Sie Funktionen und beheben Sie Fehler" icon="hammer">
    Beschreiben Sie, was Sie möchten, in einfacher Sprache. Claude Code plant den Ansatz, schreibt den Code über mehrere Dateien hinweg und überprüft, ob er funktioniert.

    Bei Fehlern fügen Sie eine Fehlermeldung ein oder beschreiben Sie das Symptom. Claude Code verfolgt das Problem durch Ihre Codebasis, identifiziert die Grundursache und implementiert eine Lösung. Weitere Beispiele finden Sie unter [Häufige Workflows](/docs/de/common-workflows).
  </Accordion>

  <Accordion title="Erstellen Sie Commits und Pull Requests" icon="code-branch">
    Claude Code arbeitet direkt mit git. Es stellt Änderungen bereit, schreibt Commit-Nachrichten, erstellt Branches und öffnet Pull Requests.

    ```bash theme={null}
    claude "commit my changes with a descriptive message"
    ```

    In CI können Sie Code-Reviews und Issue-Triage mit [GitHub Actions](/docs/de/github-actions) oder [GitLab CI/CD](/docs/de/gitlab-ci-cd) automatisieren.
  </Accordion>

  <Accordion title="Verbinden Sie Ihre Tools mit MCP" icon="plug">
    Das [Model Context Protocol (MCP)](/docs/de/mcp) ist ein offener Standard für die Verbindung von KI-Tools mit externen Datenquellen. Mit MCP kann Claude Code Ihre Design-Dokumente in Google Drive lesen, Tickets in Jira aktualisieren, Daten aus Slack abrufen oder Ihre eigenen benutzerdefinierten Tools verwenden. Der [MCP-Schnellstart](/docs/de/mcp-quickstart) verbindet Ihren ersten Server von Anfang bis Ende.
  </Accordion>

  <Accordion title="Passen Sie mit Anweisungen, Skills und Hooks an" icon="sliders">
    [`CLAUDE.md`](/docs/de/memory) ist eine Markdown-Datei, die Sie im Stammverzeichnis Ihres Projekts hinzufügen und die Claude Code zu Beginn jeder Sitzung liest. Verwenden Sie sie, um Codierungsstandards, Architekturentscheidungen, bevorzugte Bibliotheken und Überprüfungschecklisten festzulegen. Claude erstellt auch [automatisches Gedächtnis](/docs/de/memory#auto-memory), während es arbeitet, und speichert Erkenntnisse wie Build-Befehle und Debugging-Einblicke über Sitzungen hinweg, ohne dass Sie etwas schreiben müssen.

    Erstellen Sie [Skills](/docs/de/skills), um wiederholbare Workflows zu verpacken, die Ihr Team teilen kann, wie `/review-pr` oder `/deploy-staging`.

    [Hooks](/docs/de/hooks) ermöglichen es Ihnen, Shell-Befehle vor oder nach Claude Code-Aktionen auszuführen, wie automatische Formatierung nach jeder Dateibearbeitung oder Ausführung von Lint vor einem Commit.
  </Accordion>

  <Accordion title="Führen Sie Agent-Teams aus und erstellen Sie benutzerdefinierte Agents" icon="users">
    Starten Sie [mehrere Claude Code-Agents](/docs/de/sub-agents), die gleichzeitig an verschiedenen Teilen einer Aufgabe arbeiten. Ein Lead-Agent koordiniert die Arbeit, weist Unteraufgaben zu und führt Ergebnisse zusammen.

    Für vollständig benutzerdefinierte Workflows ermöglicht das [Agent SDK](/docs/de/agent-sdk/overview) Ihnen, Ihre eigenen Agents zu erstellen, die von Claude Codes Tools und Funktionen angetrieben werden, mit vollständiger Kontrolle über Orchestrierung, Tool-Zugriff und Berechtigungen.
  </Accordion>

  <Accordion title="Pipen, Skripten und Automatisieren mit der CLI" icon="terminal">
    Claude Code ist zusammensetzbar und folgt der Unix-Philosophie. Pipen Sie Logs hinein, führen Sie es in CI aus oder verketten Sie es mit anderen Tools:

    ```bash theme={null}
    # Analysieren Sie aktuelle Log-Ausgabe
    tail -200 app.log | claude -p "Slack me if you see any anomalies"

    # Automatisieren Sie Übersetzungen in CI
    claude -p "translate new strings into French and raise a PR for review"

    # Massenoperationen über Dateien hinweg
    git diff main --name-only | claude -p "review these changed files for security issues"
    ```

    Siehe die [CLI-Referenz](/docs/de/cli-reference) für den vollständigen Satz von Befehlen und Flags.
  </Accordion>

  <Accordion title="Planen Sie wiederkehrende Aufgaben" icon="clock">
    Führen Sie Claude nach einem Zeitplan aus, um Arbeit zu automatisieren, die sich wiederholt: morgendliche PR-Reviews, nächtliche CI-Fehleranalyse, wöchentliche Abhängigkeitsprüfungen oder Synchronisierung von Dokumenten nach PR-Merges.

    * [Routinen](/docs/de/routines) werden auf von Anthropic verwalteter Infrastruktur ausgeführt, sodass sie weiterhin ausgeführt werden, auch wenn Ihr Computer ausgeschaltet ist. Sie können auch durch API-Aufrufe oder GitHub-Ereignisse ausgelöst werden. Erstellen Sie sie über das Web, die Desktop-App oder durch Ausführung von `/schedule` in der CLI.
    * [Desktop-geplante Aufgaben](/docs/de/desktop-scheduled-tasks) werden auf Ihrem Computer ausgeführt, mit direktem Zugriff auf Ihre lokalen Dateien und Tools
    * [`/loop`](/docs/de/scheduled-tasks) wiederholt eine Eingabeaufforderung innerhalb einer CLI-Sitzung für schnelle Abfragen
  </Accordion>

  <Accordion title="Arbeiten Sie von überall aus" icon="globe">
    Sitzungen sind nicht an eine einzelne Oberfläche gebunden. Verschieben Sie Arbeit zwischen Umgebungen, wenn sich Ihr Kontext ändert:

    * Treten Sie von Ihrem Schreibtisch weg und arbeiten Sie weiter von Ihrem Telefon oder einem beliebigen Browser mit [Remote Control](/docs/de/remote-control)
    * Senden Sie [Dispatch](/docs/de/desktop#sessions-from-dispatch) eine Aufgabe von Ihrem Telefon und öffnen Sie die Desktop-Sitzung, die es erstellt
    * Starten Sie eine lang laufende Aufgabe im [Web](/docs/de/claude-code-on-the-web) oder in der [iOS-App](https://apps.apple.com/app/claude-by-anthropic/id6473753684) und ziehen Sie sie mit `claude --teleport` in Ihr Terminal. Teleport erfordert ein claude.ai-Abonnement.
    * Übergeben Sie eine Terminal-Sitzung an die [Desktop-App](/docs/de/desktop) mit `/desktop` für visuelle Diff-Überprüfung
    * Leiten Sie Aufgaben aus Team-Chat weiter: Erwähnen Sie `@Claude` in [Slack](/docs/de/slack) mit einem Fehlerbericht und erhalten Sie einen Pull Request zurück
  </Accordion>
</AccordionGroup>

<h2 id="use-claude-code-everywhere">
  Verwenden Sie Claude Code überall
</h2>

Jede [Oberfläche](/docs/de/glossary#surface) verbindet sich mit der gleichen zugrunde liegenden Claude Code-Engine, sodass Ihre CLAUDE.md-Dateien, Einstellungen und MCP-Server auf allen Oberflächen funktionieren.

Über die [Terminal](/docs/de/quickstart), [VS Code](/docs/de/vs-code), [JetBrains](/docs/de/jetbrains), [Desktop](/docs/de/desktop) und [Web](/docs/de/claude-code-on-the-web) Oberflächen hinaus integriert sich Claude Code mit CI/CD-, Chat- und Browser-Workflows:

| Ich möchte...                                                                                  | Beste Option                                                                                                         |
| ---------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| Eine lokale Sitzung von meinem Telefon oder einem anderen Gerät fortsetzen                     | [Remote Control](/docs/de/remote-control)                                                                                 |
| Ereignisse von Telegram, Discord, iMessage oder meinen eigenen Webhooks in eine Sitzung pushen | [Channels](/docs/de/channels)                                                                                             |
| Eine Aufgabe lokal starten, auf dem Mobilgerät fortsetzen                                      | [Web](/docs/de/claude-code-on-the-web) oder [Claude iOS-App](https://apps.apple.com/app/claude-by-anthropic/id6473753684) |
| Claude nach einem Zeitplan ausführen                                                           | [Routinen](/docs/de/routines) oder [Desktop-geplante Aufgaben](/docs/de/desktop-scheduled-tasks)                               |
| PR-Reviews und Issue-Triage automatisieren                                                     | [GitHub Actions](/docs/de/github-actions) oder [GitLab CI/CD](/docs/de/gitlab-ci-cd)                                           |
| Automatische Code-Überprüfung bei jedem PR erhalten                                            | [GitHub Code Review](/docs/de/code-review)                                                                                |
| Fehlerberichte von Slack zu Pull Requests weiterleiten                                         | [Slack](/docs/de/slack)                                                                                                   |
| Live-Webanwendungen debuggen                                                                   | [Chrome](/docs/de/chrome)                                                                                                 |
| Benutzerdefinierte Agents für Ihre eigenen Workflows erstellen                                 | [Agent SDK](/docs/de/agent-sdk/overview)                                                                                  |

<h2 id="next-steps">
  Nächste Schritte
</h2>

Nachdem Sie Claude Code installiert haben, helfen Ihnen diese Leitfäden, tiefer einzusteigen.

* [Quickstart](/docs/de/quickstart): Gehen Sie durch Ihre erste echte Aufgabe, vom Erkunden einer Codebasis bis zum Committen einer Lösung
* [Speichern Sie Anweisungen und Erinnerungen](/docs/de/memory): Geben Sie Claude persistente Anweisungen mit CLAUDE.md-Dateien und automatischem Gedächtnis
* [Häufige Workflows](/docs/de/common-workflows) und [Best Practices](/docs/de/best-practices): Muster für optimale Nutzung von Claude Code
* [Ein Harness für jede Aufgabe](https://claude.com/blog/a-harness-for-every-task-dynamic-workflows-in-claude-code): Wie das Claude Code-Team [dynamische Workflows](/docs/de/workflows) nutzt, um Subagenten im großen Maßstab zu orchestrieren
* [Einstellungen](/docs/de/settings): Passen Sie Claude Code an Ihren Workflow an
* [Fehlerbehebung](/docs/de/troubleshooting): Lösungen für häufige Probleme
* [code.claude.com](https://code.claude.com/): Demos, Preise und Produktdetails
