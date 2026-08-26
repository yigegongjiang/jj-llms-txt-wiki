> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Schnellstart

> Willkommen bei Claude Code!

Diese Schnellstartanleitung ermöglicht es Ihnen, in wenigen Minuten KI-gestützte Codierungshilfe zu nutzen. Am Ende werden Sie verstehen, wie Sie Claude Code für häufige Entwicklungsaufgaben einsetzen.

<h2 id="before-you-begin">
  Bevor Sie beginnen
</h2>

Stellen Sie sicher, dass Sie folgende Voraussetzungen erfüllen:

* Ein offenes Terminal oder eine offene Eingabeaufforderung
  * Wenn Sie das Terminal noch nie verwendet haben, lesen Sie den [Terminal-Leitfaden](/docs/de/terminal-guide)
* Ein Codeprojekt zum Arbeiten
* Ein [Claude-Abonnement](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_prereq) (Pro, Max, Team oder Enterprise), ein [Claude Console](https://console.anthropic.com/)-Konto oder Zugriff über einen [unterstützten Cloud-Anbieter](/docs/de/third-party-integrations)

<Note>
  Diese Anleitung behandelt die Terminal-CLI. Claude Code ist auch im [Web](https://claude.ai/code) verfügbar, als [Desktop-App](/docs/de/desktop), in [VS Code](/docs/de/vs-code) und [JetBrains IDEs](/docs/de/jetbrains), in [Slack](/docs/de/slack) und in CI/CD mit [GitHub Actions](/docs/de/github-actions) und [GitLab](/docs/de/gitlab-ci-cd). Siehe [alle Schnittstellen](/docs/de/overview#use-claude-code-everywhere).
</Note>

<h2 id="step-1-install-claude-code">
  Schritt 1: Claude Code installieren
</h2>

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

<h2 id="step-2-log-in-to-your-account">
  Schritt 2: Melden Sie sich bei Ihrem Konto an
</h2>

Claude Code erfordert ein Konto zur Nutzung. Starten Sie eine interaktive Sitzung mit dem Befehl `claude` und Sie werden beim ersten Gebrauch aufgefordert, sich anzumelden:

```bash theme={null}
claude
```

Für Claude-Abonnement- oder Console-Konten folgen Sie den Aufforderungen, um die Authentifizierung in Ihrem Browser abzuschließen. Um später zu einem anderen Konto zu wechseln oder sich erneut zu authentifizieren, geben Sie `/login` in der laufenden Sitzung ein:

```text theme={null}
/login
```

Sie können sich mit einem dieser Kontotypen anmelden:

* [Claude Pro, Max, Team oder Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_login) (empfohlen)
* [Claude Console](https://console.anthropic.com/) (API-Zugriff mit Prepaid-Guthaben). Bei der ersten Anmeldung wird automatisch ein „Claude Code"-Arbeitsbereich in der Console erstellt, um die Kosten zentral zu verfolgen.
* [Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry](/docs/de/third-party-integrations) (Enterprise-Cloud-Anbieter)
* Ein selbst gehostetes [Claude-Apps-Gateway](/docs/de/claude-apps-gateway), falls Ihre Organisation eines betreibt: Ihr Administrator konfiguriert die Gateway-URL vorab, und `/login` öffnet direkt den Bildschirm **Cloud-Gateway** für Sie, um sich mit Corporate SSO anzumelden

Nach der Anmeldung werden Ihre Anmeldedaten gespeichert und Sie müssen sich nicht erneut anmelden.

<h2 id="step-3-start-your-first-session">
  Schritt 3: Starten Sie Ihre erste Sitzung
</h2>

Öffnen Sie Ihr Terminal in einem beliebigen Projektverzeichnis und starten Sie Claude Code:

```bash theme={null}
cd /path/to/your/project
claude
```

Sie sehen die Claude Code-Eingabeaufforderung mit der Version, dem aktuellen Modell und dem Arbeitsverzeichnis, das oben angezeigt wird. Geben Sie `/help` ein, um verfügbare Befehle anzuzeigen, oder `/resume`, um ein vorheriges Gespräch fortzusetzen.

<Tip>
  Nach der Anmeldung (Schritt 2) werden Ihre Anmeldedaten auf Ihrem System gespeichert. Weitere Informationen finden Sie unter [Verwaltung von Anmeldedaten](/docs/de/authentication#credential-management).
</Tip>

<h2 id="step-4-ask-your-first-question">
  Schritt 4: Stellen Sie Ihre erste Frage
</h2>

Beginnen Sie damit, Ihre Codebasis zu verstehen. Versuchen Sie einen dieser Befehle:

```text theme={null}
what does this project do?
```

Claude wird Ihre Dateien analysieren und eine Zusammenfassung bereitstellen. Sie können auch spezifischere Fragen stellen:

```text theme={null}
what technologies does this project use?
```

```text theme={null}
where is the main entry point?
```

```text theme={null}
explain the folder structure
```

Sie können Claude auch nach seinen eigenen Fähigkeiten fragen:

```text theme={null}
what can Claude Code do?
```

```text theme={null}
how do I create custom skills in Claude Code?
```

```text theme={null}
can Claude Code work with Docker?
```

<Note>
  Claude Code liest Ihre Projektdateien nach Bedarf. Sie müssen den Kontext nicht manuell hinzufügen.
</Note>

<h2 id="step-5-make-your-first-code-change">
  Schritt 5: Nehmen Sie Ihre erste Codeänderung vor
</h2>

Jetzt lassen Sie Claude Code tatsächlich programmieren. Versuchen Sie eine einfache Aufgabe:

```text theme={null}
add a hello world function to the main file
```

Claude Code wird:

1. Die entsprechende Datei finden
2. Die vorgeschlagenen Änderungen anzeigen
3. Um Ihre Genehmigung bitten
4. Die Bearbeitung durchführen

<Note>
  Claude Code fragt immer um Erlaubnis, bevor Dateien geändert werden. Sie können einzelne Änderungen genehmigen oder den Modus „Alle akzeptieren" für eine Sitzung aktivieren.
</Note>

<h2 id="step-6-use-git-with-claude-code">
  Schritt 6: Verwenden Sie Git mit Claude Code
</h2>

Claude Code macht Git-Operationen konversativ:

```text theme={null}
what files have I changed?
```

```text theme={null}
commit my changes with a descriptive message
```

Sie können auch komplexere Git-Operationen anfordern:

```text theme={null}
create a new branch called feature/quickstart
```

```text theme={null}
show me the last 5 commits
```

```text theme={null}
help me resolve merge conflicts
```

<h2 id="step-7-fix-a-bug-or-add-a-feature">
  Schritt 7: Beheben Sie einen Fehler oder fügen Sie eine Funktion hinzu
</h2>

Claude ist versiert im Debuggen und in der Implementierung von Funktionen.

Beschreiben Sie, was Sie möchten, in natürlicher Sprache:

```text theme={null}
add input validation to the user registration form
```

Oder beheben Sie vorhandene Probleme:

```text theme={null}
there's a bug where users can submit empty forms - fix it
```

Claude Code wird:

* Den relevanten Code lokalisieren
* Den Kontext verstehen
* Eine Lösung implementieren
* Tests ausführen, falls verfügbar

<h2 id="step-8-test-out-other-common-workflows">
  Schritt 8: Testen Sie andere häufige Arbeitsabläufe
</h2>

Es gibt verschiedene Möglichkeiten, mit Claude zu arbeiten:

**Code umgestalten**

```text theme={null}
refactor the authentication module to use async/await instead of callbacks
```

**Tests schreiben**

```text theme={null}
write unit tests for the calculator functions
```

**Dokumentation aktualisieren**

```text theme={null}
update the README with installation instructions
```

**Code-Überprüfung**

```text theme={null}
review my changes and suggest improvements
```

<Tip>
  Sprechen Sie mit Claude wie mit einem hilfreichen Kollegen. Beschreiben Sie, was Sie erreichen möchten, und es wird Ihnen helfen, dorthin zu gelangen.
</Tip>

<h2 id="essential-commands">
  Wesentliche Befehle
</h2>

Hier sind die wichtigsten Befehle für die tägliche Nutzung. Shell-Befehle werden von Ihrem Terminal aus ausgeführt, um Claude Code zu starten oder fortzusetzen. Sitzungsbefehle werden in Claude Code ausgeführt, nachdem es gestartet wurde.

**Shell-Befehle**

| Befehl              | Was er tut                                           | Beispiel                            |
| ------------------- | ---------------------------------------------------- | ----------------------------------- |
| `claude`            | Interaktiven Modus starten                           | `claude`                            |
| `claude "task"`     | Eine einmalige Aufgabe ausführen                     | `claude "fix the build error"`      |
| `claude -p "query"` | Einmalige Abfrage ausführen und dann beenden         | `claude -p "explain this function"` |
| `claude -c`         | Letztes Gespräch im aktuellen Verzeichnis fortsetzen | `claude -c`                         |
| `claude -r`         | Ein vorheriges Gespräch fortsetzen                   | `claude -r`                         |

**Sitzungsbefehle**

| Befehl              | Was er tut                  | Beispiel |
| ------------------- | --------------------------- | -------- |
| `/clear`            | Gesprächsverlauf löschen    | `/clear` |
| `/help`             | Verfügbare Befehle anzeigen | `/help`  |
| `/exit` oder Ctrl+D | Claude Code beenden         | `/exit`  |

Siehe die [CLI-Referenz](/docs/de/cli-reference) für die vollständige Liste der Shell-Befehle und die [Befehle-Referenz](/docs/de/commands) für die vollständige Liste der Sitzungsbefehle.

<h2 id="pro-tips-for-beginners">
  Tipps für Anfänger
</h2>

Weitere Informationen finden Sie unter [Best Practices](/docs/de/best-practices) und [häufige Arbeitsabläufe](/docs/de/common-workflows).

<AccordionGroup>
  <Accordion title="Seien Sie spezifisch mit Ihren Anfragen">
    Statt: 'Beheben Sie den Fehler"

    Versuchen Sie: „Beheben Sie den Login-Fehler, bei dem Benutzer einen leeren Bildschirm sehen, nachdem sie falsche Anmeldedaten eingegeben haben"
  </Accordion>

  <Accordion title="Verwenden Sie Schritt-für-Schritt-Anweisungen">
    Unterteilen Sie komplexe Aufgaben in Schritte:

    ```text theme={null}
    1. create a new database table for user profiles
    2. create an API endpoint to get and update user profiles
    3. build a webpage that allows users to see and edit their information
    ```
  </Accordion>

  <Accordion title="Lassen Sie Claude zuerst erkunden">
    Bevor Sie Änderungen vornehmen, lassen Sie Claude Ihren Code verstehen:

    ```text theme={null}
    analyze the database schema
    ```

    ```text theme={null}
    build a dashboard showing products that are most frequently returned by our UK customers
    ```
  </Accordion>

  <Accordion title="Sparen Sie Zeit mit Verknüpfungen">
    * Geben Sie `/` ein, um alle Befehle und Skills anzuzeigen
    * Verwenden Sie Tab für Befehlsvervollständigung
    * Drücken Sie ↑ für Befehlsverlauf
    * Drücken Sie `Shift+Tab`, um zwischen Berechtigungsmodi zu wechseln
  </Accordion>
</AccordionGroup>

<h2 id="what’s-next">
  Was kommt als Nächstes?
</h2>

Nachdem Sie die Grundlagen gelernt haben, erkunden Sie erweiterte Funktionen:

<CardGroup cols={2}>
  <Card title="Wie Claude Code funktioniert" icon="microchip" href="/docs/de/how-claude-code-works">
    Verstehen Sie die agentengestützte Schleife, integrierte Tools und wie Claude Code mit Ihrem Projekt interagiert
  </Card>

  <Card title="Best Practices" icon="star" href="/docs/de/best-practices">
    Erzielen Sie bessere Ergebnisse mit effektiven Prompts und Projektsetup
  </Card>

  <Card title="Häufige Arbeitsabläufe" icon="graduation-cap" href="/docs/de/common-workflows">
    Schritt-für-Schritt-Anleitungen für häufige Aufgaben
  </Card>

  <Card title="Erweitern Sie Claude Code" icon="puzzle-piece" href="/docs/de/features-overview">
    Passen Sie mit CLAUDE.md, skills, hooks, MCP und mehr an
  </Card>
</CardGroup>

<h2 id="getting-help">
  Hilfe erhalten
</h2>

* **In Claude Code**: Geben Sie `/help` ein oder fragen Sie „how do I..."
* **Dokumentation**: Sie sind hier! Durchsuchen Sie andere Leitfäden
* **Community**: Treten Sie unserem [Discord](https://www.anthropic.com/discord) bei, um Tipps und Unterstützung zu erhalten
