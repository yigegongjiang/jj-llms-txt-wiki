> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plattformen und Integrationen

> Wählen Sie, wo Sie Claude Code ausführen möchten, und was Sie damit verbinden. Vergleichen Sie die CLI, Desktop, VS Code, JetBrains, Web und Integrationen wie Chrome, Slack und CI/CD.

Claude Code führt überall die gleiche zugrunde liegende Engine aus, aber jede Oberfläche ist für eine andere Arbeitsweise optimiert. Diese Seite hilft Ihnen, die richtige Plattform für Ihren Arbeitsablauf auszuwählen und die Tools zu verbinden, die Sie bereits verwenden.

<h2 id="where-to-run-claude-code">
  Wo Sie Claude Code ausführen
</h2>

Wählen Sie eine Plattform basierend auf Ihrer bevorzugten Arbeitsweise und dem Ort Ihres Projekts.

| Plattform                         | Am besten für                                                                                                   | Was Sie erhalten                                                                                                                                                                             |
| :-------------------------------- | :-------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [CLI](/docs/de/quickstart)             | Terminal-Arbeitsabläufe, Scripting, Remote-Server                                                               | Vollständiger Funktionsumfang, [Agent SDK](/docs/de/headless), [Computernutzung](/docs/de/computer-use) auf macOS (Pro und Max), Drittanbieter-Provider                                                |
| [Desktop](/docs/de/desktop)            | Visuelle Überprüfung, parallele Sitzungen, verwaltetes Setup                                                    | Diff-Viewer, App-Vorschau, [Computernutzung](/docs/de/desktop#let-claude-use-your-computer) und [Dispatch](/docs/de/desktop#sessions-from-dispatch) auf Pro und Max                                    |
| [VS Code](/docs/de/vs-code)            | Arbeiten in VS Code ohne Wechsel zu einem Terminal                                                              | Inline-Diffs, integriertes Terminal, Dateikontext                                                                                                                                            |
| [JetBrains](/docs/de/jetbrains)        | Arbeiten in IntelliJ, PyCharm, WebStorm oder anderen JetBrains-IDEs                                             | Diff-Viewer, Auswahlfreigabe, Terminal-Sitzung                                                                                                                                               |
| [Web](/docs/de/claude-code-on-the-web) | Langfristige Aufgaben, die nicht viel Steuerung benötigen, oder Arbeiten, die offline fortgesetzt werden sollen | Von Anthropic verwaltete Cloud, wird nach dem Trennen fortgesetzt                                                                                                                            |
| Mobile                            | Starten und Überwachen von Aufgaben, wenn Sie weg von Ihrem Computer sind                                       | Cloud-Sitzungen aus der Claude-App für iOS und Android, [Remote Control](/docs/de/remote-control) für lokale Sitzungen, [Dispatch](/docs/de/desktop#sessions-from-dispatch) zu Desktop auf Pro und Max |

Die CLI ist die vollständigste Oberfläche für Terminal-native Arbeiten: Scripting und das Agent SDK sind nur in der CLI verfügbar. Drittanbieter-Provider funktionieren auch in [VS Code](/docs/de/vs-code#use-third-party-providers). Enterprise-[Desktop](/docs/de/desktop)-Bereitstellungen unterstützen Google Cloud's Agent Platform, und Desktop unterstützt [Gateway-Provider](/docs/de/llm-gateway-connect#desktop-app); für Amazon Bedrock oder Microsoft Foundry verwenden Sie die CLI oder VS Code oder [Claude Desktop auf 3P](https://claude.com/docs/third-party/claude-desktop/overview), das die Code-Registerkarte auf diesen Providern ausführt. Desktop und die IDE-Erweiterungen verzichten auf einige CLI-exklusive Funktionen zugunsten visueller Überprüfung und engerer Editor-Integration. Das Web läuft in Anthropics Cloud, sodass Aufgaben nach dem Trennen weitergehen. Mobile ist ein einfacher Client für diese gleichen Cloud-Sitzungen oder für eine lokale Sitzung über Remote Control und kann Aufgaben mit Dispatch zu Desktop senden.

Sie können mehrere Oberflächen im gleichen Projekt verwenden. Konfiguration, Projektgedächtnis und MCP-Server werden über die lokalen Oberflächen hinweg gemeinsam genutzt.

<h2 id="connect-your-tools">
  Verbinden Sie Ihre Tools
</h2>

Integrationen ermöglichen es Claude, mit Services außerhalb Ihrer Codebasis zu arbeiten.

| Integration                          | Was es tut                                             | Verwenden Sie es für                                                                 |
| :----------------------------------- | :----------------------------------------------------- | :----------------------------------------------------------------------------------- |
| [Chrome](/docs/de/chrome)                 | Steuert Ihren Browser mit Ihren angemeldeten Sitzungen | Testen von Web-Apps, Ausfüllen von Formularen, Automatisierung von Websites ohne API |
| [GitHub Actions](/docs/de/github-actions) | Führt Claude in Ihrer CI-Pipeline aus                  | Automatisierte PR-Überprüfungen, Issue-Triage, geplante Wartung                      |
| [GitLab CI/CD](/docs/de/gitlab-ci-cd)     | Dasselbe wie GitHub Actions für GitLab                 | CI-gesteuerte Automatisierung auf GitLab                                             |
| [Code Review](/docs/de/code-review)       | Überprüft jeden PR automatisch                         | Fehler vor der menschlichen Überprüfung erkennen                                     |
| [Slack](/docs/de/slack)                   | Antwortet auf `@Claude`-Erwähnungen in Ihren Kanälen   | Umwandlung von Fehlerberichten in Pull Requests aus Team-Chat                        |

Für Integrationen, die hier nicht aufgeführt sind, ermöglichen [MCP-Server](/docs/de/mcp) und [Konnektoren](/docs/de/desktop#connect-external-tools) die Verbindung mit fast allem: Linear, Notion, Google Drive oder Ihren eigenen internen APIs.

<h2 id="work-when-you-are-away-from-your-terminal">
  Arbeiten Sie, wenn Sie weg von Ihrem Terminal sind
</h2>

Claude Code offers several ways to work when you're not at your terminal. They differ in what triggers the work, where Claude runs, and how much you need to set up.

|                                                          | Trigger                                                                                        | Claude runs on                                                                               | Setup                                                                                                                                | Best for                                                      |
| :------------------------------------------------------- | :--------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------ |
| [Dispatch](/docs/en/desktop#sessions-from-dispatch)           | Message a task from the Claude mobile app                                                      | Your machine (Desktop)                                                                       | [Pair the mobile app with Desktop](https://support.claude.com/en/articles/13947068)                                                  | Delegating work while you're away, minimal setup              |
| [Remote Control](/docs/en/remote-control)                     | Drive a running session from [claude.ai/code](https://claude.ai/code) or the Claude mobile app | Your machine (CLI or VS Code)                                                                | Run `claude remote-control`                                                                                                          | Steering in-progress work from another device                 |
| [Channels](/docs/en/channels)                                 | Push events from a chat app like Telegram or Discord, or your own server                       | Your machine (CLI)                                                                           | [Install a channel plugin](/docs/en/channels#quickstart) or [build your own](/docs/en/channels-reference)                                      | Reacting to external events like CI failures or chat messages |
| [Slack](/docs/en/slack)                                       | Mention `@Claude` in a team channel                                                            | Anthropic cloud                                                                              | [Install the Slack app](/docs/en/slack#setting-up-claude-code-in-slack) with [Claude Code on the web](/docs/en/claude-code-on-the-web) enabled | PRs and reviews from team chat                                |
| [Self-hosted environments](/docs/en/self-hosted-environments) | Start a [cloud session](/docs/en/claude-code-on-the-web) and pick your organization's environment   | Your organization's infrastructure                                                           | [Deploy runners](/docs/en/self-hosted-environments-quickstart), on Team and Enterprise plans                                              | Cloud sessions that must run inside your network              |
| [Scheduled tasks](/docs/en/scheduled-tasks)                   | Set a schedule                                                                                 | [CLI](/docs/en/scheduled-tasks), [Desktop](/docs/en/desktop-scheduled-tasks), or [cloud](/docs/en/routines) | Pick a frequency                                                                                                                     | Recurring automation like daily reviews                       |

Wenn Sie nicht sicher sind, wo Sie anfangen sollen, [installieren Sie die CLI](/docs/de/quickstart) und führen Sie sie in einem Projektverzeichnis aus. Wenn Sie lieber kein Terminal verwenden möchten, bietet [Desktop](/docs/de/desktop-quickstart) Ihnen die gleiche Engine mit einer grafischen Benutzeroberfläche.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

<h3 id="platforms">
  Plattformen
</h3>

* [CLI-Schnellstart](/docs/de/quickstart): Installation und Ausführung Ihres ersten Befehls im Terminal
* [Desktop](/docs/de/desktop): visuelle Diff-Überprüfung, parallele Sitzungen, Computernutzung und Dispatch
* [VS Code](/docs/de/vs-code): die Claude Code-Erweiterung in Ihrem Editor
* [JetBrains](/docs/de/jetbrains): die Erweiterung für IntelliJ, PyCharm und andere JetBrains-IDEs
* [Claude Code im Web](/docs/de/claude-code-on-the-web): Cloud-Sitzungen, die weiterlaufen, wenn Sie sich trennen
* Mobile: die Claude-App für [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) und [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) zum Starten und Überwachen von Aufgaben, wenn Sie weg von Ihrem Computer sind

<h3 id="integrations">
  Integrationen
</h3>

* [Chrome](/docs/de/chrome): Automatisieren Sie Browser-Aufgaben mit Ihren angemeldeten Sitzungen
* [Computernutzung](/docs/de/computer-use): Lassen Sie Claude Apps öffnen und Ihren Bildschirm auf macOS steuern
* [GitHub Actions](/docs/de/github-actions): Führen Sie Claude in Ihrer CI-Pipeline aus
* [GitLab CI/CD](/docs/de/gitlab-ci-cd): dasselbe für GitLab
* [Code Review](/docs/de/code-review): automatische Überprüfung bei jedem Pull Request
* [Slack](/docs/de/slack): Senden Sie Aufgaben aus Team-Chat, erhalten Sie PRs zurück

<h3 id="remote-access">
  Remote-Zugriff
</h3>

* [Dispatch](/docs/de/desktop#sessions-from-dispatch): Senden Sie eine Aufgabe von Ihrem Telefon aus, und es kann eine Desktop-Sitzung starten
* [Remote Control](/docs/de/remote-control): Steuern Sie eine laufende Sitzung von Ihrem Telefon oder Browser aus
* [Channels](/docs/de/channels): Schieben Sie Ereignisse von Chat-Apps oder Ihren eigenen Servern in eine Sitzung
* [Geplante Aufgaben](/docs/de/scheduled-tasks): Führen Sie Prompts nach einem wiederkehrenden Zeitplan aus
