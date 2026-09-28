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
| [Web](/docs/de/claude-code-on-the-web) | Langfristige Aufgaben, die nicht viel Steuerung benötigen, oder Arbeiten, die offline fortgesetzt werden sollen | Cloud, standardmäßig von Anthropic verwaltet; wird nach dem Trennen fortgesetzt                                                                                                              |
| [Mobile](/docs/de/mobile)              | Starten und Überwachen von Aufgaben, wenn Sie weg von Ihrem Computer sind                                       | Cloud-Sitzungen aus der Claude-App für iOS und Android, [Remote Control](/docs/de/remote-control) für lokale Sitzungen, [Dispatch](/docs/de/desktop#sessions-from-dispatch) zu Desktop auf Pro und Max |

Die CLI ist die vollständigste Oberfläche für Terminal-native Arbeiten: Scripting und das Agent SDK sind nur in der CLI verfügbar. Drittanbieter-Provider funktionieren auch in [VS Code](/docs/de/vs-code#use-third-party-providers) und in [JetBrains](/docs/de/feature-availability#features-available-on-every-provider), das die CLI im Terminal Ihrer IDE ausführt. Enterprise-[Desktop](/docs/de/desktop)-Bereitstellungen unterstützen Google Cloud's Agent Platform, und Desktop unterstützt [Gateway-Provider](/docs/de/llm-gateway-connect#desktop-app); für Amazon Bedrock oder Microsoft Foundry verwenden Sie die CLI oder eine IDE-Erweiterung, oder [Claude Desktop auf 3P](https://claude.com/docs/third-party/claude-desktop/overview), das die Code-Registerkarte auf diesen Providern ausführt. Desktop und die IDE-Erweiterungen verzichten auf einige CLI-exklusive Funktionen zugunsten visueller Überprüfung und engerer Editor-Integration. Das Web läuft in der Cloud, sodass Aufgaben nach dem Trennen weitergehen. Mobile ist ein einfacher Client für diese gleichen Cloud-Sitzungen oder für eine lokale Sitzung über Remote Control und kann Aufgaben mit Dispatch zu Desktop senden.

Sie können mehrere Oberflächen im gleichen Projekt verwenden. Konfiguration, Projektgedächtnis und MCP-Server werden über die lokalen Oberflächen hinweg gemeinsam genutzt.

<h2 id="connect-your-tools">
  Verbinden Sie Ihre Tools
</h2>

Integrationen ermöglichen es Claude, mit Services außerhalb Ihrer Codebasis zu arbeiten.

| Integration                                      | Was es tut                                                                                                           | Verwenden Sie es für                                                                                |
| :----------------------------------------------- | :------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------- |
| [Chrome](/docs/de/chrome)                             | Steuert Ihren Browser mit Ihren angemeldeten Sitzungen                                                               | Testen von Web-Apps, Ausfüllen von Formularen, Automatisierung von Websites ohne API                |
| [GitHub Actions](/docs/de/github-actions)             | Führt Claude in Ihrer CI-Pipeline aus                                                                                | Automatisierte PR-Überprüfungen, Issue-Triage, geplante Wartung                                     |
| [GitLab CI/CD](/docs/de/gitlab-ci-cd)                 | Dasselbe wie GitHub Actions für GitLab                                                                               | CI-gesteuerte Automatisierung auf GitLab                                                            |
| [Code Review](/docs/de/code-review)                   | Überprüft jeden PR automatisch                                                                                       | Fehler vor der menschlichen Überprüfung erkennen                                                    |
| [Slack](/docs/de/slack)                               | Antwortet auf `@Claude`-Erwähnungen in Ihren Kanälen                                                                 | Umwandlung von Fehlerberichten in Pull Requests aus Team-Chat                                       |
| [Claude Tag](https://claude.com/docs/claude-tag) | Führt `@Claude` als gemeinsame Identität Ihrer Organisation mit vom Administrator konfigurierten Zugriffsrechten aus | Gemeinsamer Team-Zugriff auf Team- und Enterprise-Plänen, anstelle von Slack-Sitzungen pro Benutzer |

Für Integrationen, die hier nicht aufgeführt sind, ermöglichen [MCP-Server](/docs/de/mcp) und [Konnektoren](/docs/de/desktop#connect-external-tools) die Verbindung mit fast allem: Linear, Notion, Google Drive oder Ihren eigenen internen APIs.

<h2 id="work-when-you-are-away-from-your-terminal">
  Arbeiten Sie, wenn Sie weg von Ihrem Terminal sind
</h2>

Claude Code bietet mehrere Möglichkeiten, um zu arbeiten, wenn Sie nicht an Ihrem Terminal sind. Sie unterscheiden sich darin, was die Arbeit auslöst, wo Claude ausgeführt wird und wie viel Setup Sie benötigen.

|                                                          | Auslöser                                                                                                    | Claude wird ausgeführt auf                                                                    | Setup                                                                                                                                      | Am besten geeignet für                                               |
| :------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------- |
| [Dispatch](/docs/de/desktop#sessions-from-dispatch)           | Senden Sie eine Aufgabe aus der Claude Mobile-App                                                           | Ihr Computer (Desktop)                                                                        | [Koppeln Sie die Mobile-App mit Desktop](https://support.claude.com/en/articles/13947068)                                                  | Delegieren von Arbeit, wenn Sie weg sind, minimales Setup            |
| [Remote Control](/docs/de/remote-control)                     | Steuern Sie eine laufende Sitzung von [claude.ai/code](https://claude.ai/code) oder der Claude Mobile-App   | Ihr Computer (CLI oder VS Code)                                                               | Führen Sie `claude remote-control` aus                                                                                                     | Steuerung laufender Arbeiten von einem anderen Gerät                 |
| [Channels](/docs/de/channels)                                 | Pushen Sie Ereignisse aus einer Chat-App wie Telegram oder Discord oder Ihrem eigenen Server                | Ihr Computer (CLI)                                                                            | [Installieren Sie ein Channel-Plugin](/docs/de/channels#quickstart) oder [erstellen Sie Ihr eigenes](/docs/de/channels-reference)                    | Reagieren auf externe Ereignisse wie CI-Fehler oder Chat-Nachrichten |
| [Slack](/docs/de/slack)                                       | Erwähnen Sie `@Claude` in einem Team-Kanal                                                                  | Anthropic Cloud                                                                               | [Installieren Sie die Slack-App](/docs/de/slack#setting-up-claude-code-in-slack) mit [Claude Code im Web](/docs/de/claude-code-on-the-web) aktiviert | PRs und Reviews aus Team-Chat                                        |
| [Self-hosted environments](/docs/de/self-hosted-environments) | Starten Sie eine [Cloud-Sitzung](/docs/de/claude-code-on-the-web) und wählen Sie die Umgebung Ihrer Organisation | Infrastruktur Ihrer Organisation                                                              | [Stellen Sie Runner bereit](/docs/de/self-hosted-environments-quickstart), in Team- und Enterprise-Plänen                                       | Cloud-Sitzungen, die in Ihrem Netzwerk ausgeführt werden müssen      |
| [Scheduled tasks](/docs/de/scheduled-tasks)                   | Legen Sie einen Zeitplan fest                                                                               | [CLI](/docs/de/scheduled-tasks), [Desktop](/docs/de/desktop-scheduled-tasks) oder [Cloud](/docs/de/routines) | Wählen Sie eine Häufigkeit                                                                                                                 | Wiederkehrende Automatisierung wie tägliche Reviews                  |

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
* [Web](/docs/de/claude-code-on-the-web): Cloud-Sitzungen aus Ihrem Browser unter claude.ai/code, die weiterlaufen, wenn Sie sich trennen
* [Projekte](/docs/de/claude-projects): eine Konversation, in der Claude viele Cloud-Sitzungen für ein Projekt koordiniert und Bericht erstattet
* [Mobile](/docs/de/mobile): die Claude-App für [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) und [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) zum Starten und Überwachen von Aufgaben, wenn Sie weg von Ihrem Computer sind

<h3 id="integrations">
  Integrationen
</h3>

* [Chrome](/docs/de/chrome): Automatisieren Sie Browser-Aufgaben mit Ihren angemeldeten Sitzungen
* [Computernutzung](/docs/de/computer-use): Lassen Sie Claude Apps öffnen und Ihren Bildschirm auf macOS steuern
* [GitHub Actions](/docs/de/github-actions): Führen Sie Claude in Ihrer CI-Pipeline aus
* [GitLab CI/CD](/docs/de/gitlab-ci-cd): dasselbe für GitLab
* [Code Review](/docs/de/code-review): automatische Überprüfung bei jedem Pull Request
* [Slack](/docs/de/slack): Senden Sie Aufgaben aus Team-Chat, erhalten Sie PRs zurück
* [Claude Tag](https://claude.com/docs/claude-tag): Führen Sie `@Claude` als gemeinsame Identität Ihrer Organisation in Team- und Enterprise-Plänen aus

<h3 id="remote-access">
  Remote-Zugriff
</h3>

* [Dispatch](/docs/de/desktop#sessions-from-dispatch): Senden Sie eine Aufgabe von Ihrem Telefon aus, und es kann eine Desktop-Sitzung starten
* [Remote Control](/docs/de/remote-control): Steuern Sie eine laufende Sitzung von Ihrem Telefon oder Browser aus
* [Channels](/docs/de/channels): Schieben Sie Ereignisse von Chat-Apps oder Ihren eigenen Servern in eine Sitzung
* [Geplante Aufgaben](/docs/de/scheduled-tasks): Führen Sie Prompts nach einem wiederkehrenden Zeitplan aus
