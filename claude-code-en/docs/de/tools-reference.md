> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Tools-Referenz

> Vollständige Referenz für die Tools, die Claude Code verwenden kann, einschließlich Berechtigungsanforderungen und Verhalten pro Tool.

Claude Code hat Zugriff auf eine Reihe von integrierten Tools, die ihm helfen, Ihre Codebasis zu verstehen und zu ändern. Die Tool-Namen sind die exakten Zeichenketten, die Sie in [Berechtigungsregeln](/docs/de/permissions#tool-specific-permission-rules), [Subagent-Tool-Listen](/docs/de/sub-agents) und [Hook-Matchern](/docs/de/hooks) verwenden.

Um zu steuern, welche Tools Claude verwenden kann und wann er zuerst fragt, konfigurieren Sie [Berechtigungsregeln](/docs/de/permissions#tool-specific-permission-rules) in Ihren Einstellungen, [Hooks](/docs/de/hooks) oder einer [Tool-Liste eines Subagenten](/docs/de/sub-agents#supported-frontmatter-fields). Siehe [Tools mit Berechtigungsregeln und Hooks konfigurieren](#configure-tools-with-permission-rules-and-hooks) für jeden Ort, der einen Tool-Namen akzeptiert.

Um benutzerdefinierte Tools hinzuzufügen, verbinden Sie einen [MCP-Server](/docs/de/mcp). Um Claude mit wiederverwendbaren Prompt-basierten Workflows zu erweitern, schreiben Sie einen [Skill](/docs/de/skills), der über das vorhandene `Skill`-Tool ausgeführt wird, anstatt einen neuen Tool-Eintrag hinzuzufügen.

<Info>
  Bei Pro-, Max- und Team-Plänen startet Claude Code Sitzungen im [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode), wobei ein Klassifizierer die meisten dieser Aufforderungen entscheidet, anstatt Sie. Die Spalte `Berechtigung erforderlich` zeigt, ob das Tool im [Manuellen Modus](/docs/de/permission-modes) für Pfade im Arbeitsverzeichnis auffordert. Datei-Zugriffs-Tools, die mit Nein gekennzeichnet sind, einschließlich `Read`, `Grep` und `Glob`, fordern weiterhin für Pfade außerhalb des [Arbeitsverzeichnisses und zusätzlicher Verzeichnisse](/docs/de/permissions#working-directories) auf. `Bash` ist mit Ja gekennzeichnet, führt aber einen integrierten Satz von [schreibgeschützten Befehlen](/docs/de/permissions#read-only-commands) ohne Aufforderung aus.
</Info>

| Tool                   | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Berechtigung erforderlich |
| :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------ |
| `Agent`                | Erzeugt einen [Subagenten](/docs/de/sub-agents) mit seinem eigenen Kontextfenster, um eine Aufgabe zu bewältigen. Mit [Agent-Teams](/docs/de/agent-teams) aktiviert, kann ein Aufruf, der einen `name` trägt, einen [Teamkollegen](/docs/de/agent-teams#how-claude-starts-agent-teams) starten. Siehe [Agent-Tool-Verhalten](#agent-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Nein                      |
| `Artifact`             | Veröffentlicht eine HTML- oder Markdown-Datei als [Artefakt](/docs/de/artifacts): eine private, interaktive Seite auf claude.ai. Sie können sie mit einem öffentlichen Link teilen oder in Ihrer Organisation auf Team- und Enterprise-Plänen, wo öffentliches Teilen erfordert, dass ein Besitzer es [aktiviert](/docs/de/artifacts#control-public-sharing). Erfordert einen Pro-, Max-, Team- oder Enterprise-Plan und `/login`-Authentifizierung; siehe [Verfügbarkeit](/docs/de/artifacts#availability)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Ja                        |
| `AskUserQuestion`      | Stellt Multiple-Choice-Fragen, um Anforderungen zu sammeln oder Mehrdeutigkeit zu klären. Fragen bleiben standardmäßig offen, bis Sie sie beantworten. Siehe [AskUserQuestion-Tool-Verhalten](#askuserquestion-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Nein                      |
| `Bash`                 | Führt Shell-Befehle in Ihrer Umgebung aus. Siehe [Bash-Tool-Verhalten](#bash-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | Ja                        |
| `CronCreate`           | Plant eine wiederkehrende oder einmalige Aufforderung innerhalb der aktuellen Sitzung. Aufgaben sind sitzungsbegrenzt und werden bei `--resume` oder `--continue` wiederhergestellt, wenn sie nicht abgelaufen sind. Siehe [geplante Aufgaben](/docs/de/scheduled-tasks)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Nein                      |
| `CronDelete`           | Bricht eine geplante Aufgabe nach ID ab                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Nein                      |
| `CronList`             | Listet alle geplanten Aufgaben in der Sitzung auf                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Nein                      |
| `Edit`                 | Nimmt gezielte Änderungen an bestimmten Dateien vor. Siehe [Edit-Tool-Verhalten](#edit-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Ja                        |
| `EndConversation`      | Beendet die Sitzung in seltenen Fällen von anhaltendem missbräuchlichem Input oder wenn Sie Claude auffordern, das Tool zu demonstrieren. Erfordert Claude Code v2.1.213 oder später. Siehe [EndConversation-Tool-Verhalten](#endconversation-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Nein                      |
| `EnterPlanMode`        | Wechselt in den Plan-Modus, um einen Ansatz vor dem Codieren zu entwerfen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Nein                      |
| `EnterWorktree`        | Erstellt einen isolierten [Git-Worktree](/docs/de/worktrees) und wechselt hinein. Übergeben Sie einen `path`, um stattdessen in einen vorhandenen Worktree zu wechseln. Beim ersten Eintritt kann das Ziel ein Worktree des aktuellen Repositorys oder in einem Multi-Repo-Workspace ein darin verschachteltes Repositorys sein. Vor v2.1.203 wurde ein Worktree eines verschachtelten Repositorys abgelehnt. Ein `path` außerhalb von `.claude/worktrees/` fordert Ihre Genehmigung auf, bevor Sie eintreten, da er das Arbeitsverzeichnis und den Schreibzugriff der Sitzung an diesen Ort verschiebt. Die Erstellung neuer Worktrees und Pfade unter `.claude/worktrees/` fordern nicht auf. Vor v2.1.206 trat Claude Pfaden außerhalb von `.claude/worktrees/` ohne Aufforderung bei. Aus einer Worktree-Sitzung oder von einem Subagenten mit einem angehefteten Arbeitsverzeichnis wie [`isolation: worktree`](/docs/de/sub-agents#supported-frontmatter-fields) ist nur die `path`-Form verfügbar und das Ziel muss unter `.claude/worktrees/` des Repositorys der Sitzung liegen                                                                                                         | Ja                        |
| `ExitPlanMode`         | Präsentiert einen Plan zur Genehmigung und beendet den Plan-Modus                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Ja                        |
| `ExitWorktree`         | Beendet eine Worktree-Sitzung und kehrt zum ursprünglichen Verzeichnis zurück. Nicht verfügbar für Subagenten, die bereits in ihrem eigenen Arbeitsverzeichnis ausgeführt werden, z. B. mit [`isolation: worktree`](/docs/de/sub-agents#supported-frontmatter-fields)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Nein                      |
| `Glob`                 | Findet Dateien basierend auf Musterabgleich. Siehe [Glob-Tool-Verhalten](#glob-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Nein                      |
| `Grep`                 | Sucht nach Mustern in Dateiinhalten. Siehe [Grep-Tool-Verhalten](#grep-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Nein                      |
| `ListAgents`           | Listet die Agenten auf, die Claude mit `SendMessage` kontaktieren kann: Subagenten in der Sitzung, [Agent-Team](/docs/de/agent-teams)-Teamkollegen, Ihre anderen lokalen Claude-Code-Sitzungen und, während diese Sitzung mit [Remote Control](/docs/de/remote-control) verbunden ist, Ihre [Claude Code im Web](/docs/de/claude-code-on-the-web)-Sitzungen und Ihre Remote-Control-Sitzungen auf anderen Maschinen. Unterstützt den `/list-agents`-Befehl. Siehe [sitzungsübergreifendes Messaging](/docs/de/cross-session-messaging). Erfordert Claude Code v2.1.224 oder später und wird nur in Sitzungen angezeigt, in denen [sitzungsübergreifendes Messaging aktiviert ist](/docs/de/cross-session-messaging#availability). Teamkollegen-Zeilen und die erste Zeile, die den Namen dieser Sitzung zeigt, erfordern v2.1.239 oder später                                                                                                                                                                                                                                                                                                                                                                   | Nein                      |
| `ListMcpResourcesTool` | Listet Ressourcen auf, die von verbundenen [MCP-Servern](/docs/de/mcp) bereitgestellt werden                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Nein                      |
| `LSP`                  | Code-Intelligenz über Sprachserver: Sprung zu Definitionen, Suche nach Referenzen, Meldung von Typfehlern und Warnungen. Siehe [LSP-Tool-Verhalten](#lsp-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Nein                      |
| `Monitor`              | Führt einen Befehl im Hintergrund aus und speist jede Ausgabezeile an Claude zurück, damit er auf Protokolleinträge, Dateiänderungen oder abgerufene Status in der Mitte des Gesprächs reagieren kann. Kann auch einen WebSocket öffnen und jede eingehende Nachricht als Ereignis behandeln. Siehe [Monitor-Tool](#monitor-tool)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Ja                        |
| `NotebookEdit`         | Ändert Jupyter-Notebook-Zellen. Siehe [NotebookEdit-Tool-Verhalten](#notebookedit-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Ja                        |
| `PowerShell`           | Führt PowerShell-Befehle nativ aus. Siehe [PowerShell-Tool](#powershell-tool) für Verfügbarkeit                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | Ja                        |
| `PushNotification`     | Sendet eine Desktop-Benachrichtigung und eine Telefon-Push-Benachrichtigung, wenn [Remote Control](/docs/de/remote-control) verbunden ist, damit eine lange laufende Aufgabe oder [geplante Aufgabe](/docs/de/scheduled-tasks) Sie erreichen kann, wenn Sie weggehen. Push-Zustellung läuft durch von Anthropic gehostete Infrastruktur, auf die nicht von Amazon Bedrock, Claude Platform auf AWS, Google Clouds Agent Platform oder Microsoft Foundry zugegriffen werden kann                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Nein                      |
| `Read`                 | Liest den Inhalt von Dateien. Siehe [Read-Tool-Verhalten](#read-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Nein                      |
| `ReadMcpResourceTool`  | Liest eine bestimmte MCP-Ressource nach URI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | Nein                      |
| `RemoteTrigger`        | Erstellt, aktualisiert, führt aus und listet [Routinen](/docs/de/routines) auf claude.ai auf. Unterstützt den `/schedule`-Befehl. Die [`RemoteTrigger`-Eingabereferenz](/docs/de/agent-sdk/typescript#remotetrigger) dokumentiert jede Aktion und die Organisationsrichtlinien, die das Tool entfernen. Routinen befinden sich auf claude.ai und erfordern einen Pro-, Max-, Team- oder Enterprise-Plan, daher ist dieses Tool nicht von Amazon Bedrock, Claude Platform auf AWS, Google Clouds Agent Platform oder Microsoft Foundry zugänglich                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Nein                      |
| `ReportFindings`       | Meldet Code-Review-Ergebnisse als strukturierte Liste mit einer Datei, Zusammenfassung und Fehlerfall pro Ergebnis, damit Claude Code sie rendern kann, anstatt sie als Text auszudrucken. Claude ruft es auf, wenn aktive Code-Review-Anweisungen dies anweisen. Erfordert Claude Code v2.1.196 oder später. Ab v2.1.199 kann ein Ergebnis auch einen optionalen `category`-Slug tragen, wie `correctness` oder `test-coverage`, der neben dem Dateispeicherort in der gerenderten Liste angezeigt wird                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Nein                      |
| `ScheduleWakeup`       | Plant die nächste Iteration einer [selbstgesteuerten `/loop`](/docs/de/scheduled-tasks#let-claude-choose-the-interval) neu. Claude ruft dies am Ende jeder Iteration auf, um auszuwählen, wann die nächste ausgeführt wird, zwischen einer Minute und einer Stunde; Sie rufen es nicht direkt auf. Um die Schleife stattdessen zu beenden, ruft Claude sie mit `stop: true` auf, was das ausstehende Wakeup abbricht. Das `stop`-Feld erfordert Claude Code v2.1.202 oder später. Das ausstehende Wakeup wird in `session_crons` in [Stop-Hook-Eingabe](/docs/de/hooks#stop-input) angezeigt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Nein                      |
| `SendFeedback`         | Entwirft einen Feedbackbericht über Claude Code, der ein Produktproblem oder Claudes eigenes Verhalten in der Sitzung abdeckt, und stellt ihn in die Warteschlange auf Ihrer Maschine, damit Sie ihn überprüfen können. Claude Code sendet nichts, bis Sie sich entscheiden, den Entwurf zu senden. Siehe [SendFeedback-Tool-Verhalten](#sendfeedback-tool-behavior). Erfordert Claude Code v2.1.238 oder später                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Nein                      |
| `SendMessage`          | Sendet eine Nachricht an einen anderen Agenten: einen [Agent-Team](/docs/de/agent-teams)-Teamkollegen, einen [Subagenten, den es nach Agent-ID oder Name fortsetzen kann](/docs/de/sub-agents#resume-subagents), oder eine Ihrer anderen Claude-Code-Sitzungen, auf dieser Maschine oder darüber hinaus. Das Messaging anderer Sitzungen erfordert Claude Code v2.1.224 oder später. [Sitzungsübergreifendes Messaging](/docs/de/cross-session-messaging) behandelt, welche Sitzungen Claude erreichen kann, [wie eine Nachricht aussieht, wenn sie ankommt](/docs/de/cross-session-messaging#what-a-message-looks-like), und [wie Claude eine Benachrichtigung erhält, wenn eine andere Sitzung untätig wird](/docs/de/cross-session-messaging#get-a-notice-when-another-session-goes-idle). Claude kann eine optionale `summary`-Eingabe einschließen, typischerweise 5-10 Wörter, die Claude Code als Einzeilenvorschau anzeigt. Wenn Claude sie bei einer [Nur-Text-Nachricht](/docs/de/cross-session-messaging#limitations) auslässt, verwendet Claude Code die erste Zeile der Nachricht als Zusammenfassung. Claude Code schneidet eine Zusammenfassung, die länger als 200 Zeichen ist, mit einer Ellipse ab | Nein                      |
| `SendUserFile`         | Sendet Dateien aus der Sitzung an Sie mit einer optionalen Beschriftung, damit ein generierter Bericht, Diagramm, Screenshot oder erstelltes Artefakt Ihr Gerät erreicht, anstatt nur im Transkript erwähnt zu werden. Ab v2.1.196 steuert die optionale `display`-Eingabe die Präsentation: `render` öffnet die Datei inline im Client, `attach` zeigt nur eine Download-Karte an, und wenn nicht gesetzt, entscheidet der Client nach Dateityp. Verfügbar, wenn ein [Remote Control](/docs/de/remote-control)-Client verbunden ist oder in einer [Cloud-Sitzung](/docs/de/claude-code-on-the-web). Die Zustellung läuft durch von Anthropic gehostete Infrastruktur, daher ist das Tool nicht auf Amazon Bedrock, Google Clouds Agent Platform oder Microsoft Foundry verfügbar                                                                                                                                                                                                                                                                                                                                                                                                                | Nein                      |
| `ShareOnboardingGuide` | Lädt `ONBOARDING.md` hoch und gibt einen Share-Link zurück, den Teamkollegen in Claude Code öffnen können. Wird von `/team-onboarding` aufgerufen, nachdem der Leitfaden geschrieben wurde. Verfügbar für claude.ai-Abonnenten auf Pro-, Max-, Team- und Enterprise-Plänen                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Ja                        |
| `Skill`                | Führt einen [Skill](/docs/de/skills#control-who-invokes-a-skill) innerhalb des Hauptgesprächs aus                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Ja                        |
| `SubagentHandback`     | Liefert den abschließenden Bericht eines Subagenten an das Gespräch, das das Ergebnis dieses Subagenten erhält. Wird nur im [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) bereitgestellt, für Subagenten, die das Agent-Tool lokal ausführt, außer [Forks](/docs/de/sub-agents#fork-the-current-conversation), und ist in der Terminal-CLI, IDE-Erweiterungen, Cloud-Sitzungen und dem Agent SDK verfügbar; der Klassifizierer überprüft den Bericht, bevor er zugestellt wird. Erfordert Claude Code v2.1.271 oder später                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | Nein                      |
| `TaskCreate`           | Erstellt eine neue Aufgabe in der Aufgabenliste. Wird standardmäßig nur auf den Modellen bereitgestellt, die unter [Task-Tool-Verfügbarkeit](#task-tool-availability) aufgelistet sind, und auf anderen Modellen, wenn Sie sich dafür entscheiden                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Nein                      |
| `TaskGet`              | Ruft vollständige Details für eine bestimmte Aufgabe ab. Wird standardmäßig nur auf den Modellen bereitgestellt, die unter [Task-Tool-Verfügbarkeit](#task-tool-availability) aufgelistet sind, und auf anderen Modellen, wenn Sie sich dafür entscheiden                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Nein                      |
| `TaskList`             | Listet alle Aufgaben mit ihrem aktuellen Status auf. Wird standardmäßig nur auf den Modellen bereitgestellt, die unter [Task-Tool-Verfügbarkeit](#task-tool-availability) aufgelistet sind, und auf anderen Modellen, wenn Sie sich dafür entscheiden                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | Nein                      |
| `TaskOutput`           | Ruft Ausgabe von einer Hintergrundaufgabe ab. Veraltet zugunsten von `Read` auf dem Ausgabedateipfad der Aufgabe. Wenn keine Aufgabe der ID entspricht, listet der Fehler die laufenden Hintergrund-Agenten nach ID und Beschreibung auf. Vor v2.1.203 benannte der Fehler nur die fehlende ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Nein                      |
| `TaskStop`             | Stoppt eine laufende Hintergrundaufgabe nach ID. Es akzeptiert auch einen [Agent-Team-Teamkollegen](/docs/de/agent-teams) oder einen benannten Hintergrund-Agenten nach Agent-ID oder Name. Vor v2.1.198 akzeptierte es nur eine Hintergrundaufgaben-ID. Wenn keine Aufgabe der ID entspricht, listet der Fehler die laufenden Hintergrund-Agenten nach ID und Beschreibung auf, einschließlich Agenten, die ein anderer Agent erzeugt hat. Vor v2.1.203 listete der Fehler laufende Teamkollegen und benannte Agenten auf, aber nicht Hintergrund-Agenten, die ein anderer Agent erzeugt hat, daher konnten diese nicht identifiziert oder vom Hauptgespräch aus gestoppt werden                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Nein                      |
| `TaskUpdate`           | Aktualisiert Aufgabenstatus, Abhängigkeiten, Details oder löscht Aufgaben. Wird standardmäßig nur auf den Modellen bereitgestellt, die unter [Task-Tool-Verfügbarkeit](#task-tool-availability) aufgelistet sind, und auf anderen Modellen, wenn Sie sich dafür entscheiden                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | Nein                      |
| `TodoWrite`            | Verwaltet die Sitzungs-Aufgabencheckliste. Standardmäßig deaktiviert zugunsten von `TaskCreate`, `TaskGet`, `TaskList` und `TaskUpdate`. Setzen Sie `CLAUDE_CODE_ENABLE_TASKS=0`, um es in [Sitzungen, die die Task-Tracking-Tools haben](#task-tool-availability) erneut zu aktivieren                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Nein                      |
| `ToolSearch`           | Sucht nach und lädt aufgeschobene Tools, wenn [Tool-Suche](/docs/de/mcp#scale-with-mcp-tool-search) aktiviert ist                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Nein                      |
| `WaitForMcpServers`    | Wartet auf einen oder mehrere [MCP-Server](/docs/de/mcp), die noch im Hintergrund verbunden werden, damit eine Anfrage ihre Tools verwenden kann, ohne die Sitzung neu zu starten. Claude ruft es auf, wenn ein benötigter Server noch nicht verbunden ist. Wird nur angezeigt, wenn [Tool-Suche](/docs/de/mcp#scale-with-mcp-tool-search) deaktiviert ist, da `ToolSearch` das Warten handhabt, wenn es aktiviert ist                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Nein                      |
| `WebFetch`             | Ruft Inhalte von einer angegebenen URL ab. Siehe [WebFetch-Tool-Verhalten](#webfetch-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | Ja                        |
| `WebSearch`            | Führt Web-Suchen durch. Siehe [WebSearch-Tool-Verhalten](#websearch-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Ja                        |
| `Workflow`             | Führt einen [dynamischen Workflow](/docs/de/workflows) aus: ein Skript, das viele Subagenten im Hintergrund orchestriert und ein konsolidiertes Ergebnis zurückgibt                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | Ja                        |
| `Write`                | Erstellt oder überschreibt Dateien. Siehe [Write-Tool-Verhalten](#write-tool-behavior)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Ja                        |

<h2 id="configure-tools-with-permission-rules-and-hooks">
  Tools mit Berechtigungsregeln und Hooks konfigurieren
</h2>

In den meisten Fällen entscheidet Claude selbst, wann diese Tools verwendet werden sollen, und Sie müssen sie nicht selbst benennen, wenn Sie mit Claude interagieren. Sie verweisen auf Tool-Namen direkt, wenn Sie Berechtigungen und andere Konfigurationen definieren:

* in [`permissions.allow`](/docs/de/settings-reference#permissions-allow) und [`permissions.deny`](/docs/de/settings-reference#permissions-deny) in den Einstellungen und in der `/permissions`-Schnittstelle
* in den [CLI-Flags](/docs/de/cli-reference) `--allowedTools` und `--disallowedTools`
* in den Optionen [`allowedTools` und `disallowedTools`](/docs/de/agent-sdk/permissions#allow-and-deny-rules) des Agent SDK
* in der [Frontmatter `allowed-tools`](/docs/de/skills#frontmatter-reference) eines Skills
* in der [`if`-Bedingung](/docs/de/hooks-guide#filter-by-tool-name-and-arguments-with-the-if-field) eines Hooks

Alle diese akzeptieren das gleiche Regelformat: `ToolName(specifier)`. Der Specifier hängt vom Tool ab, und mehrere Tools verwenden ein gemeinsames Format:

| Regelformat                    | Gilt für                  | Details                                                                   |
| :----------------------------- | :------------------------ | :------------------------------------------------------------------------ |
| `Bash(npm run *)`              | Bash, Monitor             | [Befehlsmuster-Matching](/docs/de/permissions#bash)                            |
| `PowerShell(Get-ChildItem *)`  | PowerShell                | [Befehlsmuster-Matching](/docs/de/permissions#powershell)                      |
| `Read(~/secrets/**)`           | Read, Grep, Glob, LSP     | [Pfadmuster-Matching](/docs/de/permissions#read-and-edit)                      |
| `Edit(/src/**)`                | Edit, Write, NotebookEdit | [Pfadmuster-Matching](/docs/de/permissions#read-and-edit)                      |
| `Skill(deploy *)`              | Skill                     | [Skill-Namen-Matching](/docs/de/skills#restrict-claude%E2%80%99s-skill-access) |
| `Agent(Explore)`               | Agent                     | [Subagent-Typ-Matching](/docs/de/permissions#agent-subagents)                  |
| `WebFetch(domain:example.com)` | WebFetch                  | [Domain-Matching](/docs/de/permissions#webfetch)                               |
| `WebSearch`                    | WebSearch                 | Kein Specifier; Tool als Ganzes erlauben oder verweigern                  |

Tools, die hier nicht aufgeführt sind, wie `ExitPlanMode` oder `ShareOnboardingGuide`, akzeptieren nur den bloßen Tool-Namen ohne Specifier.

Eine `Edit(...)`-Erlaubnisregel gewährt auch Lesezugriff auf denselben Pfad, sodass Sie keine entsprechende `Read(...)`-Regel benötigen. Eine `Read(...)`-Verweigerungsregel blockiert auch die Tools Edit und Write auf demselben Pfad, einschließlich der Erstellung einer neuen Datei dort, da beide Tools Inhalte ändern, die Claude zurücklesen können muss. Die `Read`-Verweigerungsprüfung erfordert Claude Code v2.1.208 oder später bei Bearbeitungen, und v2.1.228 oder später bei Schreibvorgängen.

Hook-`matcher`-Felder verwenden bloße Tool-Namen, nicht das geklammerte Regelformat. Siehe [Matcher-Muster](/docs/de/hooks#matcher-patterns) für die Matching-Regeln. Für die Feldnamen, die jedes Tool an `tool_input` in Hooks übergibt, siehe die [PreToolUse-Eingabereferenz](/docs/de/hooks#pretooluse-input).

<h2 id="agent-tool-behavior">
  Verhalten des Agent-Tools
</h2>

Das Agent-Tool startet einen Subagenten in einem separaten Kontextfenster. Der Subagent arbeitet seine Aufgabe autonom ab und gibt dann sein Ergebnis an das übergeordnete Gespräch zurück. Das übergeordnete Gespräch sieht nicht die Zwischentool-Aufrufe oder Ausgaben des Subagenten, sondern nur dieses Endergebnis. Mit aktivierten [Agent-Teams](/docs/de/agent-teams) kann ein Aufruf mit einem `name` einen [Teamkollegen](/docs/de/agent-teams#how-claude-starts-agent-teams) starten, der stattdessen über Team-Nachrichten berichtet, anstatt ein Ergebnis zurückzugeben.

Um die Anzahl der Durchläufe eines Subagenten zu begrenzen, setzen Sie `maxTurns` in der [Subagenten-Definition](/docs/de/sub-agents#supported-frontmatter-fields). Wenn der Subagent das Limit erreicht, markiert Claude Code das zurückgegebene Ergebnis als Teilausgabe, und Claude kann [den Subagenten fortsetzen](/docs/de/sub-agents#resume-subagents), um fortzufahren.

Das gleiche Agent-Tool startet auch [abgespaltene Subagenten](/docs/de/sub-agents#fork-the-current-conversation) überall dort, wo [Fork-Modus](/docs/de/sub-agents#turn-fork-mode-on-or-off) aktiviert ist. Ein Fork erbt das vollständige übergeordnete Gespräch, anstatt neu zu beginnen, läuft im Hintergrund außer in den [Fällen, die im Vordergrund bleiben](/docs/de/sub-agents#run-subagents-in-foreground-or-background), und zeigt dennoch Berechtigungsaufforderungen in Ihrem Terminal an. Der Rest dieses Abschnitts beschreibt Nicht-Fork-Subagenten.

Welche Tools ein Nicht-Fork-Subagent verwenden kann, hängt von den Feldern `tools` und `disallowedTools` in der [Subagenten-Definition](/docs/de/sub-agents) ab:

* **Kein Feld gesetzt**: Der Subagent erbt jedes [für Subagenten verfügbare Tool](/docs/de/sub-agents#available-tools).
* **Nur `tools`**: Der Subagent erhält nur die aufgelisteten Tools.
* **Nur `disallowedTools`**: Der Subagent erhält jedes übergeordnete Tool außer den aufgelisteten.
* **Beide gesetzt**: `disallowedTools` hat Vorrang. Ein Tool, das in beiden aufgelistet ist, wird entfernt.

In jedem Fall ist die aufgelöste Menge auf die [für Subagenten verfügbaren Tools](/docs/de/sub-agents#available-tools) beschränkt: Ein Tool, das für Subagenten nicht verfügbar ist, wird niemals gewährt, auch wenn es in `tools` aufgelistet ist. Wenn die Bedingungen im `SubagentHandback`-Tools-Tabelleneintrag erfüllt sind, gibt Claude Code dem Subagenten auch dieses Tool, auch wenn Sie es aus `tools` auslassen oder in `disallowedTools` aufgelistet haben.

Wenn jeder Eintrag in der `tools`-Liste eines Subagenten nicht mit einem verwendbaren Tool übereinstimmt, gibt das Agent-Tool normalerweise einen Fehler zurück, der die Einträge benennt, anstatt den Subagenten zu starten; siehe [Agent würde mit null Tools gestartet](/docs/de/errors#agent-would-be-spawned-with-zero-tools) für die Nachricht und wie man jeden Eintrag behebt.

Das Starten des Subagenten selbst fordert nicht zur Berechtigung auf. Claude Code überprüft die eigenen Tool-Aufrufe des Subagenten gegen Ihre Berechtigungsregeln, während er läuft.

Wo Sie die Berechtigungsaufforderungen eines Subagenten sehen, hängt davon ab, ob er im Vordergrund oder im Hintergrund läuft. Claude Code führt Subagenten standardmäßig im Hintergrund aus, außer in den [Fällen, die im Vordergrund laufen](/docs/de/sub-agents#run-subagents-in-foreground-or-background).

* **Vordergrund-Subagenten** zeigen die gleichen Berechtigungsaufforderungen, die Sie im Hauptgespräch sehen würden, in dem Moment, in dem jeder Tool-Aufruf stattfindet.
* **Hintergrund-Subagenten** zeigen Berechtigungsaufforderungen in Ihrer Hauptsitzung ab v2.1.186 an. Die Aufforderung benennt, welcher Subagent fragt, und das Drücken von Esc verweigert diesen einen Tool-Aufruf, ohne den Subagenten zu stoppen. Vor v2.1.186 verweigerten Hintergrund-Subagenten automatisch jeden Tool-Aufruf, der sonst eine Aufforderung auslösen würde, und setzten fort, ohne dieses Tool zu verwenden.

Um [zu begrenzen, was ein Subagent erreichen kann](/docs/de/sub-agents#control-subagent-capabilities), verengen Sie zunächst sein `tools`-Feld, beispielsweise indem Sie Bash von der Liste ausschließen, oder setzen Sie Ablehnungsregeln in Ihren Einstellungen.

<h2 id="askuserquestion-tool-behavior">
  Verhalten des AskUserQuestion-Tools
</h2>

Claude verwendet `AskUserQuestion`, um Ihnen Multiple-Choice-Fragen zu stellen, wenn eine Entscheidung oder Klärung erforderlich ist. Antworten Sie, indem Sie eine Option auswählen, oder geben Sie Ihren eigenen Text über die Zeile `Other` oder das Notizfeld ein.

Wenn Sie durch Eingabe Ihres eigenen Textes antworten, leitet Claude Code die Antwort mit neutraler Formulierung weiter, damit Claude das befolgt, was Sie geschrieben haben, einschließlich einer Aufforderung zu warten oder zunächst zu erklären.

<h3 id="question-auto-continue-timeout">
  Timeout für automatische Fortsetzung von Fragen
</h3>

Fragen bleiben offen, bis Sie sie beantworten. Wenn Sie möchten, dass eine unbeantwortete Frage irgendwann von selbst geschlossen wird und Claude ohne Sie fortfahren kann, stellen Sie die Einstellung [`askUserQuestionTimeout`](/docs/de/settings-reference#askuserquestiontimeout) auf `60s`, `5m` oder `10m` ein, entweder in Ihrer Benutzerdatei `settings.json` oder in der Zeile **Question auto-continue timeout** in `/config`.

Nachdem eine Frage so lange ohne Eingabe offen war, schließt sich der Dialog von selbst: Er sendet alle Optionen, die Sie bereits ausgewählt haben, und teilt Claude mit, dass Sie möglicherweise weg von Ihrer Tastatur sind, sodass Claude nach eigenem Ermessen fortfahren kann und später erneut fragen kann. Sie sehen einen Countdown für die letzten 20 Sekunden. Drücken Sie eine beliebige Taste, um den Timer neu zu starten; auf Terminals, die den Fokus melden, wird der Timer auch durch Wechsel zum Fenster neu gestartet.

Das Timeout gilt nur für die Multiple-Choice-Fragen von `AskUserQuestion`; Berechtigungsaufforderungen, einschließlich Planbestätigung, werden bei Untätigkeit nie automatisch aufgelöst.

<h2 id="bash-tool-behavior">
  Bash-Tool-Verhalten
</h2>

Das Bash-Tool führt jeden Befehl in einem separaten Prozess aus.

<h3 id="what-persists-between-commands">
  Was zwischen Befehlen erhalten bleibt
</h3>

* Wenn Claude `cd` in der Hauptsitzung ausführt, wird das neue Arbeitsverzeichnis zu späteren Bash-Befehlen übernommen, solange es sich im Projektverzeichnis oder einem [zusätzlichen Arbeitsverzeichnis](/docs/de/permissions#working-directories) befindet, das Sie mit `--add-dir`, `/add-dir` oder `additionalDirectories` in den Einstellungen hinzugefügt haben. Dies umfasst Befehle, die Claude in Reaktion auf Ihre späteren Nachrichten ausführt.
  * Subagent-Sitzungen übernehmen niemals Änderungen des Arbeitsverzeichnisses.
  * Wenn `cd` außerhalb dieser Verzeichnisse landet, setzt Claude Code auf das Projektverzeichnis zurück und fügt `Shell cwd was reset to <dir>` zum Tool-Ergebnis hinzu.
  * Um diese Übernahme zu deaktivieren, damit jeder Bash-Befehl im Projektverzeichnis startet, setzen Sie `CLAUDE_BASH_MAINTAIN_PROJECT_WORKING_DIR=1`.
* Umgebungsvariablen bleiben nicht erhalten. Ein `export` in einem Befehl ist im nächsten nicht verfügbar.
* Aliase und Shell-Funktionen, die in Ihrer Shell-Startdatei definiert sind, sind verfügbar. Beim Sitzungsstart sourced Claude Code `~/.zshrc`, `~/.bashrc` oder `~/.profile` je nach Ihrer Shell, erfasst die resultierenden Aliase, Funktionen und Shell-Optionen und wendet sie auf jeden Bash-Befehl an.

Aktivieren Sie Ihre virtualenv oder conda-Umgebung, bevor Sie Claude Code starten. Um Umgebungsvariablen über Bash-Befehle hinweg persistent zu machen, setzen Sie [`CLAUDE_ENV_FILE`](/docs/de/env-vars) auf ein Shell-Skript, bevor Sie Claude Code starten, oder verwenden Sie einen [SessionStart-Hook](/docs/de/hooks#persist-environment-variables), um es dynamisch zu füllen.

<h3 id="timeout-and-output-limits">
  Timeout- und Ausgabegrenzen
</h3>

Jeder Befehl läuft unter einem Timeout, und Claude verwaltet ihn: Wenn er länger als die Standardzeit für einen Befehl benötigt, übergibt er den `timeout`-Parameter mit diesem Aufruf — Sie setzen niemals ein Pro-Befehl-Timeout. Zwei [Umgebungsvariablen](/docs/de/env-vars) begrenzen, was Claude erhält:

* `BASH_DEFAULT_TIMEOUT_MS` — die Standardeinstellung, wenn Claude kein Timeout übergibt; standardmäßig zwei Minuten
* `BASH_MAX_TIMEOUT_MS` — mit der Standardeinstellung wird die Obergrenze gesetzt, die das begrenzt, was Claude anfordert: die effektive Obergrenze ist die größere der beiden, standardmäßig zehn Minuten

<h4 id="output-limits">
  Ausgabegrenzen
</h4>

Claude Code streamt die Ausgabe eines Befehls während der Ausführung in eine Arbeitsdatei; ein Befehl, dessen Ausgabe 5 GB überschreitet, wird beendet. Wenn der Befehl beendet ist, liest Claude Code die Ausgabe aus dieser Datei zurück, bis zum unten beschriebenen Lesefenster. Wie viel der Ausgabe Claude inline erreicht, hängt davon ab, ob Claude Code das Ergebnis als Fehler behandelt:

| Ergebnis | Was Claude erhält                                                                                                                                                                                                                                                                       |
| :------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Gültig   | Inline bis zu etwa 30.000 Zeichen standardmäßig; darüber hinaus der Pfad einer Datei, die im Sitzungsverzeichnis gespeichert und über 64 MiB gekürzt ist, plus eine Vorschau von bis zu den ersten 2.000 Zeichen, und Claude liest oder durchsucht die Datei, wenn er den Rest benötigt |
| Fehler   | Inline bis zu etwa 10.000 Zeichen; darüber hinaus ein Kopf-und-Schwanz-Auszug dieser Größe aus dem Lesefenster, ohne Dateipfad                                                                                                                                                          |

Ein Befehl, der mit 1 beendet wird, zählt als gültiges Ergebnis für das Bash-Tool nur, wenn Claude Code den Exit-Code 1 als gutartiges Ergebnis für diesen Befehl erkennt: `grep`, `rg`, `egrep`, `fgrep`, `find`, `diff`, `test` und `[`, plus `git diff` und `git grep`. Jeder andere Befehl, der mit 1 beendet wird, zählt als Fehler, auch wenn 1 ein gutartiges Informationsergebnis ist: keine Übereinstimmungen für `pgrep` und `jq -e`, Dateien, die sich für `cmp` unterscheiden.

[`BASH_MAX_OUTPUT_LENGTH`](/docs/de/env-vars) setzt, wie viele Zeichen der Ausgabe Claude Code aus der Arbeitsdatei in das Ergebnis eines Befehls zurückliest: standardmäßig 30.000, bis zu einer harten Obergrenze von 150.000. Erhöhen Sie dies, wenn Ihre Befehle routinemäßig dieses Fenster überlasten, z. B. bei einem ausführlichen Build oder einem vollständigen Test-Suite-Protokoll. Das Erhöhen vergrößert das Lesefenster, das auch das Fenster ist, aus dem der Auszug eines fehlgeschlagenen Befehls geschnitten wird. Es erhöht nicht die Inline-Obergrenzen: ein gültiges Ergebnis über der Inline-Obergrenze kommt als Dateipfad plus Vorschau an, unabhängig von dieser Variablen.

Um zu ändern, wie viel eines gültigen Ergebnisses Claude inline erhält, setzen Sie stattdessen die Einstellung [`bashOutputMaxChars`](/docs/de/settings-reference#bashoutputmaxchars), bis zu 128.000 Zeichen. Sie dimensioniert die Inline-Obergrenze und das Lesefenster zusammen, und Claude Code ignoriert dann `BASH_MAX_OUTPUT_LENGTH`. Erfordert Claude Code v2.1.261 oder später.

<h3 id="background-commands">
  Hintergrund-Befehle
</h3>

Für lang laufende Prozesse wie Dev-Server oder Watch-Builds kann Claude `run_in_background: true` setzen, um den Befehl als Hintergrund-Task zu starten und weiter zu arbeiten, während er läuft. Listen Sie Hintergrund-Tasks auf und stoppen Sie sie mit `/tasks`. Nachdem Sie einen dort stoppen oder von einem verbundenen Client wie der Desktop-App, geht Claude weiter, anstatt zu warten. Wenn ein Subagent den Befehl gestartet hat, ist es dieser Subagent, der weitergeht.

Ein Befehl, den ein [Vordergrund-Subagent](/docs/de/sub-agents#run-subagents-in-foreground-or-background) gestartet hat, stoppt, wenn dieser Subagent seine endgültige Antwort gibt. Ein Befehl, den die Hauptkonversation oder ein Hintergrund-Subagent gestartet hat, läuft nach einer endgültigen Antwort weiter. Im nicht-interaktiven Modus mit dem `-p`-Flag [enden Hintergrund-Befehle kurz nach dem endgültigen Ergebnis des Laufs](/docs/de/headless#background-tasks-at-exit).

Wenn ein Befehl sein Timeout erreicht, ohne zu beenden, verschiebt Claude Code ihn in den Hintergrund, anstatt ihn zu stoppen, es sei denn, der Befehl beginnt mit `sleep`. Claude arbeitet weiter, während der Befehl fortgesetzt wird. Claude Code wendet die gleichen Lebenszeitregeln auf einen verschobenen Befehl an wie auf jeden anderen Hintergrund-Befehl, daher endet er immer noch einen Vordergrund-Subagent-Befehl bei der endgültigen Antwort dieses Subagenten. Das Setzen von [`CLAUDE_CODE_DISABLE_BACKGROUND_TASKS=1`](/docs/de/env-vars#variables) deaktiviert Auto-Backgrounding zusammen mit dem Rest der Hintergrund-Task-Funktionalität.

Das Ergebnis eines Befehls, der in den Hintergrund verschoben wird, gibt an, was passiert ist:

* Wenn das Timeout die Verschiebung auslöst, meldet das Ergebnis dies explizit: `Command did not complete within its 120s timeout and was moved to the background`, wobei die Sekunden dem angewendeten Timeout entsprechen, gefolgt von der Task-ID und dem Pfad der Datei, in die die Ausgabe geschrieben wird.
* Ein `cd`, `pushd`, `popd` oder `chdir` in einem Befehl, der in den Hintergrund verschoben wird, wird niemals übernommen: das Ergebnis besagt `Session cwd remains <dir>; directory changes made by the backgrounded command do not apply to subsequent commands.`, daher handelt Claude nicht auf eine Verzeichnisänderung, die nicht stattgefunden hat.

<h3 id="memory-limit-on-linux-and-wsl">
  Speicherlimit unter Linux und WSL
</h3>

Unter Linux und WSL setzen Sie [`CLAUDE_CODE_TOOL_MEMORY_LIMIT`](/docs/de/env-vars#variables) auf eine Größe wie `4G`, um den Speicher zu begrenzen, den Bash-, PowerShell- und [Monitor](#monitor-tool)-Tool-Befehle verwenden können, damit ein durchgehender Build nicht den Speicher benötigt, den der Rest der Sitzung benötigt. Erfordert Claude Code v2.1.233 oder später. Vor v2.1.246 liefen Monitor-Tool-Befehle außerhalb der Obergrenze.

* Schreiben Sie die Größe als Anzahl von Bytes oder mit einem `K`-, `M`-, `G`- oder `T`-Suffix. Setzen Sie `0`, `off`, `false`, `no` oder `none`, um die Obergrenze auszuschalten. Claude Code ignoriert jeden anderen Wert, den es nicht als Größe lesen kann, z. B. `4e9`.
* Claude Code zählt alle Bash-, PowerShell- und Monitor-Befehle einer Sitzung gegen die eine Obergrenze, nicht jeden Befehl für sich.
* Claude Code wendet die Obergrenze mit einer Memory-Cgroup an. Wenn es die Cgroup nicht einrichten kann, laufen Befehle ohne Obergrenze, und das Debug-Protokoll von `claude --debug` sagt warum.
* Nachdem der erste Prozess, den Claude Code startet, die Obergrenze eingeschaltet hat, oder sie ausgeschaltet hat, weil ein Off-Wert oder ein fehlgeschlagenes Cgroup-Setup, hält Claude Code dieses Ergebnis, bis Sie neu starten. Um einen geänderten oder entfernten Wert oder ein festes Setup anzuwenden, starten Sie `claude` erneut.
* Wenn Befehle nicht unter der Obergrenze bleiben können, tötet der Kernel einen Befehl, und nichts in seinem Ergebnis nennt die Obergrenze.

Claude Code kann auch andere Arten von Prozessen, die es startet, gegen die gleiche Grenze zählen. Setzen Sie [`CLAUDE_CODE_TOOL_MEMORY_CGROUP_EXCLUDE`](/docs/de/env-vars#variables) auf eine kommagetrennte Liste der Arten, die von der Obergrenze ausgenommen werden sollen; Claude Code wendet die Obergrenze auf jede Art an, die nicht auf Ihrer Liste steht. Setzen Sie es auf `none`, um jede Art zu begrenzen, oder auf `all-new`, um nur Bash-, PowerShell- und Monitor-Tool-Befehle zu begrenzen. Erfordert Claude Code v2.1.246 oder später. Die Arten, die Sie benennen können:

* `mcp`: lokale [MCP-Server](/docs/de/mcp)
* `lsp`: [Sprachserver](#lsp-tool-behavior)
* `hooks`: [Hook](/docs/de/hooks)-Befehle
* `plugin`: Befehle, die [Plugins](/docs/de/plugins/overview) ausführen
* `helper`: Claude Codes eigene Helper-Befehle, wie `git`
* `agent`: untergeordnete Claude-Code-Prozesse, wie [Agent-Teamkollegen](/docs/de/agent-teams)

Was auch immer Sie auflisten, diese Regeln gelten:

* **Unbekannte Namen**: Claude Code ignoriert Namen, die es nicht erkennt
* **Bash, PowerShell und Monitor**: Claude Code hält Bash-, PowerShell- und Monitor-Tool-Befehle unter der Obergrenze, was auch immer Sie auflisten
* **Variable nicht gesetzt**: Claude Code nimmt die Menge der anderen begrenzten Arten aus der Konfiguration, die Anthropic vom Server liefert, und diese Menge kann sich im Laufe der Zeit ändern, daher setzen Sie die Variable, wenn Sie eine Menge benötigen, die sich nicht ändert
* **Permission-Gating-Hooks**: Auch wenn jede Art begrenzt ist, schließt Claude Code von der Obergrenze einen Hook aus, der das Ergebnis einer Aktion blockieren oder ändern kann, und jeden MCP-Server, den ein solcher Hook aufruft, daher kann das Töten eines Permission-Gating-Hooks durch den Kernel die Aktion, die er blockiert, nicht zulassen

<h2 id="edit-tool-behavior">
  Verhalten des Edit-Tools
</h2>

Das Edit-Tool führt exakte Zeichenkettenersetzung durch. Es nimmt einen `old_string` und einen `new_string` und ersetzt den ersten durch den zweiten. Es verwendet weder Regex noch Fuzzy Matching.

Drei Überprüfungen müssen bestanden werden, damit eine Bearbeitung angewendet wird. Bevor eine davon durchgeführt wird, wird ein Pfad, der einer [`Read`-Ablehnungsregel](/docs/de/permissions#tool-specific-permission-rules) entspricht, abgelehnt, einschließlich der Erstellung einer neuen Datei dort. Die Ablehnung erfordert Claude Code v2.1.208 oder später.

* **Read-before-edit**: Claude liest die Datei in der aktuellen Konversation, bevor er sie bearbeitet, und ein Lesezugriff, der mit einer [`PARTIAL view`-Benachrichtigung](#read-tool-behavior) unterbrochen wird, zählt nicht. Claude Opus 4.6, Claude Haiku 4.5 und ältere Modelle erfordern immer das Lesen. Neuere Modelle können eine ungelesene Datei bearbeiten, wenn das Lesen keine Genehmigungsaufforderung erfordern würde und das Read-Tool verfügbar ist.
* **Match**: `old_string` muss genau wie geschrieben in der Datei vorkommen. Ein einzelnes Zeichen Unterschied in Leerzeichen oder Einrückung reicht aus, um nicht zu treffen.
* **Eindeutigkeit**: `old_string` muss genau einmal vorkommen. Wenn es mehr als einmal vorkommt, stellt Claude entweder eine längere Zeichenkette mit ausreichend umgebendem Kontext bereit, um ein Vorkommen festzulegen, oder setzt `replace_all: true`, um alle zu ersetzen.

Eine Datei, die sich auf der Festplatte geändert hat, nachdem Claude sie zuletzt gelesen hat, kann immer noch bearbeitet werden, wenn `old_string` genau und eindeutig dem aktuellen Inhalt entspricht und Claude Code die Datei ohne Aufforderung lesen kann. Das Abgleichen mit dem aktuellen Inhalt der Datei hält dies sicher, und das Ergebnis vermerkt, dass die Datei weitere Änderungen enthält, damit Claude sie vor Bearbeitungen, die auf umgebendem Inhalt basieren, erneut liest. In jedem anderen Fall, z. B. bei einem veralteten `old_string` oder einem, der mehr als einmal ohne `replace_all` übereinstimmt, liest Claude die Datei vor der Bearbeitung erneut. Die gelockerte Behandlung von ungelesenen und geänderten Dateien erfordert Claude Code v2.1.208 oder später; davor lehnte Claude Code jede Bearbeitung einer Datei ab, die es in der Konversation nicht gelesen hatte oder die sich auf der Festplatte nach dem Lesen geändert hatte.

Das Anzeigen einer Datei mit Bash erfüllt auch die Read-before-edit-Anforderung, wenn der Befehl `cat`, `nl`, `bat`, `batcat`, `head`, `tail`, `sed -n 'X,Yp'`, `grep`, `egrep`, `fgrep` oder `rg` auf einer einzelnen Datei ohne Pipes oder Umleitungen ist. Weitergeleitete Ausgabe und andere Bash-Befehle zählen nicht zur Read-before-edit-Überprüfung.

Das Anzeigen einer Datei mit Bash beeinflusst nur die Bearbeitungsberechtigung, nicht die Berechtigungen. Siehe [Read- und Edit-Berechtigungsregeln](/docs/de/permissions#read-and-edit) für die Bash-Befehle, die Ihre `Read`- und `Edit`-Ablehnungsregeln abdecken.

<h2 id="endconversation-tool-behavior">
  Verhalten des EndConversation-Tools
</h2>

Das EndConversation-Tool beendet die aktuelle Sitzung. Claude verwendet es nur in zwei Situationen:

* als letztes Mittel gegen anhaltende missbräuchliche Eingaben, nachdem Versuche, das Gespräch umzuleiten, fehlgeschlagen sind und nachdem eine klare Warnung in einer früheren Nachricht erfolgt ist
* wenn Sie explizit darum bitten, das Tool demonstriert zu sehen, und bestätigen, dass Sie die Sitzung beenden möchten

Allgemeine Frustration, Fluchen oder eine schlecht laufende Aufgabe qualifizieren sich nicht, und ebenso wenig Anfragen nach schädlichen Inhalten, die Claude ablehnt, anstatt die Sitzung zu beenden. Claude Code folgt dem gleichen Ansatz wie claude.ai, das [eine seltene Teilmenge von Chats beenden kann](https://www.anthropic.com/research/end-subset-conversations).

Nachdem Claude eine interaktive Sitzung beendet hat, wird die Sitzung gesperrt. Neue Eingaben und die meisten Befehle geben `Claude ended this conversation. Start a new session (or /clear) to continue.` zurück, und nur `/clear`, `/resume`, `/help`, `/exit` und `/feedback` werden weiterhin ausgeführt. Claude Code verzeichnet das Ende in der Sitzungstranskription, sodass das Fortsetzen einer beendeten Sitzung die Sperre wiederherstellt; die Sitzungshistorie wird nicht gelöscht.

Das Fortsetzen einer beendeten Sitzung im [nicht-interaktiven Modus](/docs/de/headless) mit dem Flag `-p` führt zu einem Fehler und beendet mit Code 1, sodass ein Skript den beendeten Lauf nicht als Erfolg interpretiert.

Das Tool fordert niemals eine Berechtigung an, und [PreToolUse-Hooks](/docs/de/hooks#pretooluse) werden nicht dafür ausgeführt. Solange ein anderes Tool vorhanden ist, können Sie es auch nicht blockieren: [Deny- und Ask-Regeln](/docs/de/permissions#tool-specific-permission-rules), die `EndConversation` benennen, haben keine Auswirkung, und weder `--disallowedTools` noch eine `--tools`-Liste können es entfernen. Die Ausnahme ist absichtlich: Das Tool tut nichts außer das Gespräch zu beenden, liest oder ändert niemals Dateien oder Daten, und eine Schutzmaßnahme dieser Art funktioniert nur, wenn die Sitzung, auf die sie sich bezieht, sie nicht deaktivieren kann. Wenn Ihre Deny-Regeln alle anderen Tools entfernen und auch `EndConversation` abgleichen, wie es `"*"` tut, entfernt Claude Code es ebenfalls, anstatt es als einziges Tool zu belassen, es sei denn, eine Allow-Regel benennt `EndConversation` explizit. Eine Deny-Liste, die alle anderen Tools entfernt, ohne `EndConversation` abzugleichen, behält es bei.

[Subagents](/docs/de/sub-agents) erhalten das Tool niemals. Hintergrundaufgaben, die die Werkzeugliste der Hauptkonversation teilen, sehen es, aber das Aufrufen dort beendet nichts.

Das Tool wird nur angezeigt, wenn alle folgenden Bedingungen erfüllt sind:

* **Version**: Claude Code v2.1.213 oder später.
* **Modell**: Das Modell der Sitzung ist Claude Opus 4.8, Claude Sonnet 5, Claude Fable 5 oder eine neuere Version einer dieser Familien.
* **Oberfläche**: eine interaktive Terminalsitzung, einschließlich einer `claude`-Sitzung im integrierten Terminal einer IDE, wie der [JetBrains-Plugin](/docs/de/jetbrains) sie ausführt. Andere Oberflächen enthalten das Tool nicht, wie zum Beispiel:
  * nicht-interaktive `-p`-Läufe
  * Sitzungen über die [Agent SDK](/docs/de/agent-sdk/overview) TypeScript- und Python-Pakete
  * das [VS Code-Erweiterungs](/docs/de/vs-code)-Panel, das seine eigene CLI bündelt
  * [GitHub Actions](/docs/de/github-actions)
  * [Claude Code im Web](/docs/de/claude-code-on-the-web)
* **Startmodus**: keine [`--bare`](/docs/de/headless#start-faster-with-bare-mode)-Sitzung. Der Bare-Modus lädt nur Shell- und Datei-Tools, sodass das Tool dort nie registriert wird.
* **Anbieter**: nicht verfügbar auf [Amazon Bedrock](/docs/de/amazon-bedrock), [Claude Platform on AWS](/docs/de/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai) oder [Microsoft Foundry](/docs/de/microsoft-foundry), oder auf Sitzungen, die sich über ein [Cloud-Gateway](/docs/de/claude-apps-gateway) anmelden.

<h2 id="glob-tool-behavior">
  Glob-Werkzeugverhalten
</h2>

Das Glob-Werkzeug findet Dateien nach Namensmuster. Unter Windows ist es Teil des Standard-Werkzeugsatzes. Auf macOS, Linux und WSL lässt Claude Code Glob und [Grep](#grep-tool-behavior) aus dem Standard-Werkzeugsatz aus, und Claude sucht mit `find` und `grep` über das Bash-Werkzeug. In Claudes Shell führen diese beiden Befehle eingebettete Versionen von `bfs` und `ugrep` aus, und die Suchen erreichen Ihre Hooks und Berechtigungsregeln als `Bash`-Aufrufe.

Auf macOS, Linux und WSL erhalten Sie die Glob- und Grep-Werkzeuge in diesen Fällen zurück:

* Sie benennen `Glob` oder `Grep` in [`--tools` oder `--allowedTools`](/docs/de/cli-reference#cli-flags), wenn Sie die Sitzung starten, oder in den entsprechenden [Agent SDK](/docs/de/agent-sdk/overview)-Optionen. Mit `--tools` erhalten Sie die, die Sie auflisten, und das Benennen eines der beiden Werkzeuge in `--allowedTools` stellt beide wieder her. Eine Allow-Regel in einer Einstellungsdatei hat diese Auswirkung nicht.
* Eine Berechtigungen-[Deny-Regel](/docs/de/permissions#match-all-uses-of-a-tool), das Flag `--disallowedTools` oder [`--restricted`](/docs/de/cli-reference#cli-flags) entfernt `Bash` aus der Sitzung.
* Ein [Subagent](/docs/de/sub-agents#available-tools) listet `Glob` oder `Grep` in seinem `tools`-Feld auf und lässt `Bash` aus. Die aufgelisteten Werkzeuge kommen für diesen Subagenten nur zurück, oder für die gesamte Sitzung, wenn er als Hauptsitzungs-Agent über [`--agent`](/docs/de/sub-agents#invoke-subagents-explicitly) oder die `agent`-Einstellung ausgeführt wird.

Glob unterstützt Standard-Glob-Syntax einschließlich `**` für rekursives Verzeichnis-Matching:

* `**/*.js` passt alle `.js`-Dateien in beliebiger Tiefe
* `src/**/*.ts` passt alle `.ts`-Dateien unter `src/`
* `*.{json,yaml}` passt `.json`- und `.yaml`-Dateien im aktuellen Verzeichnis

Ergebnisse werden nach Änderungszeit sortiert und auf 100 Dateien begrenzt. Wenn die Obergrenze erreicht wird, sieht Claude ein Kürzungsflag im Ergebnis und kann das Muster verengen.

Glob respektiert `.gitignore` standardmäßig nicht, daher findet es gitignorierte Dateien neben verfolgten. Dies unterscheidet sich von [Grep](#grep-tool-behavior), das gitignorierte Dateien überspringt. Um Glob `.gitignore` respektieren zu lassen, setzen Sie `CLAUDE_CODE_GLOB_NO_IGNORE=false`, bevor Sie Claude Code starten.

Claude Code entscheidet über die Berechtigung für einen Glob-Aufruf, bevor es überprüft, ob das Suchverzeichnis existiert. Es führt immer noch die Leseberechtigung-Überprüfung für einen fehlenden `path` außerhalb der [Arbeitsverzeichnisse](/docs/de/permissions#working-directories) durch, daher bedeutet eine Berechtigungsaufforderung für einen Pfad nicht, dass der Pfad existiert.

Ein `pattern`- oder `path`-Wert, der ein Null-Byte enthält, gibt einen Fehler zurück, der Claude auffordert, ihn zu entfernen.&#x20;

<h2 id="grep-tool-behavior">
  Grep-Werkzeugverhalten
</h2>

Das Grep-Werkzeug durchsucht Dateiinhalte nach Mustern. Während [Glob](#glob-tool-behavior) Dateien nach Namen findet, findet Grep Zeilen in ihnen. Auf macOS, Linux und WSL ist Grep standardmäßig unter denselben Bedingungen nicht vorhanden wie Glob. Siehe [Glob-Werkzeugverhalten](#glob-tool-behavior) für den Fall, dass beide Werkzeuge verfügbar sind.

Grep basiert auf [ripgrep](https://github.com/BurntSushi/ripgrep) und verwendet ripgreps Regex-Syntax, nicht POSIX-Grep. Muster, die Regex-Metazeichen enthalten, müssen maskiert werden. Zum Beispiel erfordert das Finden von `interface{}` in Go-Code das Muster `interface\{\}`.

Ein Muster, Glob oder Dateityp, den ripgrep ablehnt, gibt einen Fehler zurück, der ripgreps Diagnose enthält, damit Claude die Eingabe korrigieren und erneut suchen kann. Vor v2.1.208 meldete Claude Code eine abgelehnte Eingabe als `No files found` statt als Fehler, selbst wenn der gesuchte Text in den Zieldateien vorhanden war.

Drei Ausgabemodi steuern, was zurückkommt:

* `files_with_matches`: nur Dateipfade, kein Zeileninhalt. Dies ist die Standardeinstellung.
* `content`: übereinstimmende Zeilen mit Datei und Zeilennummer. Wenn der `offset`-Parameter des Werkzeugs über den letzten Treffer für ein Muster hinausweist, das Treffer hat, gibt Grep `No entries at this offset` zurück, sodass Claude den Offset vergrößert oder zurückgesetzt wird, anstatt zu dem Ergebnis zu kommen, dass das Muster nicht passt.
* `count`: Übereinstimmungsanzahl pro Datei, gefolgt von einer Gesamtsumme über alle übereinstimmenden Dateien. Die Gesamtsumme umfasst jeden Treffer, auch wenn die Parameter `head_limit` oder `offset` des Werkzeugs die aufgelisteten Pro-Datei-Einträge kürzen. Vor v2.1.208 summierte die Gesamtsumme nur die aufgelisteten Einträge.

Claude kann Ergebnisse nach Datei mit dem `glob`-Parameter wie `**/*.tsx` oder nach Sprache mit dem `type`-Parameter wie `py` oder `rust` eingrenzen. Standardmäßig passen Muster innerhalb einer einzelnen Zeile. Claude kann `multiline: true` setzen, um über Zeilengrenzen hinweg zu passen.

Grep respektiert `.gitignore`, daher werden gitignorierte Dateien übersprungen. Um eine gitignorierte Datei zu durchsuchen, übergibt Claude ihren Pfad direkt.

Claude Code entscheidet über die Berechtigung für einen Grep-Aufruf, bevor es überprüft, ob der Such-`path` existiert. Es führt immer noch die Leseberechtigung-Überprüfung für einen fehlenden `path` außerhalb der [Arbeitsverzeichnisse](/docs/de/permissions#working-directories) durch, sodass eine Berechtigungsaufforderung für einen Pfad nicht bedeutet, dass der Pfad existiert.

<h2 id="lsp-tool-behavior">
  LSP-Werkzeugverhalten
</h2>

Das LSP-Werkzeug gibt Claude Code-Intelligenz von einem laufenden Sprachserver. Nach jeder Dateiänderung meldet es automatisch Typfehler und Warnungen, damit Claude Probleme ohne einen separaten Build-Schritt beheben kann. Claude kann es auch direkt aufrufen, um Code zu navigieren:

* Sprung zu einer Symbol-Definition
* Suche nach allen Referenzen zu einem Symbol
* Typinformationen an einer Position abrufen
* Symbole in einer Datei auflisten
* Suche nach einem Symbol nach Name im gesamten Workspace
* Implementierungen einer Schnittstelle finden
* Aufrufen-Hierarchien verfolgen

Claude Code hält das Werkzeug inaktiv, bis Sie ein [Code-Intelligence-Plugin](/docs/de/plugins/code-intelligence) für Ihre Sprache installieren. In [Cloud-Sitzungen](/docs/de/claude-code-on-the-web) startet Claude Code keine Plugin-Sprachserver, daher bleibt das LSP-Werkzeug dort inaktiv. Claude Code übernimmt die Sprachserver-Konfiguration vom Plugin, und Sie installieren die Server-Binärdatei selbst.

Claude Code gibt ein Fehler-Ergebnis für jeden LSP-Aufruf auf einer Datei zurück, deren Sprachserver nicht gestartet werden kann.

<h2 id="monitor-tool">
  Monitor-Werkzeug
</h2>

Das Monitor-Werkzeug ermöglicht es Claude, etwas im Hintergrund zu beobachten und zu reagieren, wenn es sich ändert, ohne das Gespräch zu unterbrechen. Bitten Sie Claude:

* Eine Protokolldatei zu verfolgen und Fehler zu kennzeichnen, wenn sie erscheinen
* Eine PR oder CI-Job abzufragen und zu melden, wenn sich ihr Status ändert
* Ein Verzeichnis auf Dateiänderungen zu überwachen
* Ausgabe von einem beliebigen langfristigen Skript zu verfolgen, auf das Sie es hinweisen
* Eine Verbindung zu einem WebSocket-Feed herzustellen und jede Nachricht zu melden, wenn sie ankommt

Für die meisten Überwachungen schreibt Claude ein kleines Skript, führt es im Hintergrund aus und empfängt jede Ausgabezeile, wenn sie ankommt. Für einen Server, der bereits Ereignisse pusht, kann Claude stattdessen eine [WebSocket](#websocket-source) öffnen, anstatt ein Skript auszuführen.

Sie arbeiten weiter in der gleichen Sitzung und Claude interveniert, wenn ein Ereignis eintritt.

Jede Überwachung, die Claude startet, hat eine Frist: standardmäßig 5 Minuten, maximal 30 Minuten und maximal 10 Minuten in einem [nicht-interaktiven](/docs/de/headless) Lauf mit einem einzelnen Prompt mit `-p`.

Bei der Frist endet die Überwachung. Claude erhält eine Benachrichtigung, damit es die Überwachung erneut starten kann, falls sie noch benötigt wird.

Beenden Sie eine Überwachung, indem Sie Claude auffordern, sie zu stornieren, oder indem Sie die Sitzung beenden. Wenn Sie einen [Subagenten](/docs/de/sub-agents) beenden, der Monitore gestartet hat, beispielsweise von `/tasks`, werden diese Monitore mit ihm beendet.

Wenn Monitor einen Befehl ausführt, verwendet es die gleichen [Berechtigungsregeln wie Bash](/docs/de/permissions#tool-specific-permission-rules), daher gelten `allow`- und `deny`-Muster, die Sie für Bash festgelegt haben, auch hier. Während der [automatische Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) aktiv ist, setzt Claude Code Zulassungsregeln beiseite, die `Monitor` selbst benennen, zusammen mit den anderen [breiten Zulassungsregeln, die es verwirft](/docs/de/permission-modes#how-the-classifier-evaluates-actions), sodass der Klassifizierer Monitor-Befehle auf die gleiche Weise überprüft wie Bash-Befehle.

Die [WebSocket-Quelle](#websocket-source) hat ihre eigene Genehmigungsaufforderung, die der Klassifizierer auch im automatischen Modus entscheidet.

Das Werkzeug ist nicht auf Amazon Bedrock, Google Cloud's Agent Platform oder Microsoft Foundry verfügbar. Es ist auch nicht verfügbar, wenn `DISABLE_TELEMETRY` oder `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` gesetzt ist.

Plugins können Monitore deklarieren, die automatisch starten, wenn das Plugin aktiv ist, anstatt Claude zu bitten, sie zu starten. Siehe [Plugin-Monitore](/docs/de/plugins/components#monitors).

<h3 id="websocket-source">
  WebSocket-Quelle
</h3>

<Note>
  Die WebSocket-Quelle erfordert Claude Code v2.1.195 oder später.
</Note>

Wenn ein Server bereits Ereignisse über eine WebSocket pusht, kann Claude sich direkt damit verbinden, anstatt ein Abfrageskript zu schreiben. Jede Art von Socket-Aktivität wird entweder zu einem Ereignis oder beendet die Überwachung:

* **Textnachrichten**: jede wird zu einem Ereignis, auch wenn die Nachricht mehrere Zeilen umfasst.
* **Binärnachrichten**: werden nicht weitergeleitet. Claude empfängt stattdessen eine Platzhalterzeile wie `[binary frame, 512 bytes]`.
* **Nachrichten größer als 1 MiB**: die Überwachung endet, daher abonnieren Sie einen gefilterten Feed, falls vorhanden.
* **Socket-Schließung**: die Überwachung endet und Claude empfängt den Schließungscode.

Eine WebSocket-Überwachung nimmt eine `ws`-Eingabe anstelle von `command` und ein einzelner Monitor-Aufruf kann die beiden nicht kombinieren. Die `ws`-Eingabe hat zwei Felder:

| Feld        | Erforderlich | Beschreibung                                                                                                                                                                   |
| :---------- | :----------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url`       | Ja           | Der Endpunkt zum Verbinden. Muss eine `ws://`- oder `wss://`-URL ohne eingebettete Anmeldedaten oder Leerzeichen sein, nur ASCII-Zeichen verwenden                             |
| `protocols` | Nein         | WebSocket-Subprotokollnamen, die während des Handshake angeboten werden. Jeder Eintrag muss ein gültiges Subprotokoll-Token sein, und die Liste darf keine Duplikate enthalten |

Die `timeout_ms`-Frist gilt auch für eine WebSocket-Überwachung: die Überwachung endet bei der Frist, und `TaskStop` bricht sie früh ab.

Das Öffnen einer WebSocket fordert zur Genehmigung auf; im [automatischen Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) entscheidet der Klassifizierer stattdessen. Die Aufforderung bietet keine Option, zukünftige Aufforderungen für denselben Host zu überspringen.

Claude Code lehnt URLs ab, die auf eine private, Link-Local- oder Cloud-Metadaten-Adresse verweisen, einschließlich Hostnamen, die zu einer aufgelöst werden. Es lehnt auch Hosts in `sandbox.network.deniedDomains` ab, und wenn [`allowManagedDomainsOnly`](/docs/de/settings-reference#sandbox-network-allowmanageddomainsonly) in verwalteten Einstellungen gesetzt ist, jeden Host außerhalb der verwalteten Zulassungsliste.

<h2 id="notebookedit-tool-behavior">
  NotebookEdit-Werkzeugverhalten
</h2>

NotebookEdit ändert ein Jupyter-Notebook eine Zelle nach der anderen, indem es Zellen nach ihrer `cell_id` anvisiert. Es führt keine Zeichenkettenersetzung über das Notebook durch wie [Edit](#edit-tool-behavior) auf einfachen Dateien.

Drei Bearbeitungsmodi steuern, was mit der Zielzelle geschieht:

* `replace`: Überschreiben Sie die Quelle der Zelle. Dies ist die Standardeinstellung.
* `insert`: Fügen Sie eine neue Zelle nach der Zielzelle hinzu. Ohne `cell_id` geht die neue Zelle an den Anfang des Notebooks. Erfordert `cell_type` auf `code` oder `markdown` gesetzt.
* `delete`: Entfernen Sie die Zielzelle.

Berechtigungsregeln verwenden das `Edit(...)`-Pfadformat. Eine Regel wie `Edit(notebooks/**)` deckt NotebookEdit-Aufrufe auf Dateien in diesem Verzeichnis ab.

<h2 id="powershell-tool">
  PowerShell-Tool
</h2>

Das PowerShell-Tool ermöglicht es Claude, PowerShell-Befehle nativ auszuführen. Unter Windows bedeutet dies, dass Befehle in PowerShell ausgeführt werden, anstatt sie über Git Bash weiterzuleiten. Die Verfügbarkeit des Tools hängt von Ihrer Plattform ab:

* **Windows ohne Git Bash**: Das Tool ist automatisch aktiviert.
* **Windows mit installiertem Git Bash**: Das Tool ist standardmäßig für claude.ai und Console-Konten aktiviert; setzen Sie `CLAUDE_CODE_USE_POWERSHELL_TOOL=1`, um es in Amazon Bedrock, Google Cloud's Agent Platform und Microsoft Foundry-Sitzungen zu aktivieren, oder `0`, um es auszuschalten.
* **Linux, macOS und WSL**: Das Tool ist optional.

Ihre [PreToolUse-Hooks](/docs/de/hooks#powershell) erhalten die Befehlszeichenfolge des Tools in `tool_input.command`, mit denselben Feldern wie das Bash-Tool.

Verwenden Sie `Bash|PowerShell` in Hooks, die Shell-Befehle überprüfen; der [PowerShell-Hook-Eingabeabschnitt](/docs/de/hooks#powershell) erklärt, warum das Abgleichen von nur `Bash` nicht ausreichend ist.

<h3 id="enable-the-powershell-tool">
  PowerShell-Tool aktivieren
</h3>

Setzen Sie `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` in Ihrer Umgebung oder in `settings.json`:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_USE_POWERSHELL_TOOL": "1"
  }
}
```

Unter Windows setzen Sie die Variable auf `0`, um das Tool auszuschalten. Unter Linux, macOS und WSL erfordert das Tool PowerShell 7 oder später: Installieren Sie `pwsh` und stellen Sie sicher, dass es sich in Ihrem `PATH` befindet.

Unter Windows erkennt Claude Code automatisch `pwsh.exe` für PowerShell 7+ mit einem Fallback auf `powershell.exe` für PowerShell 5.1. Wenn das Tool aktiviert ist, behandelt Claude PowerShell als die primäre Shell. Das Bash-Tool bleibt für POSIX-Skripte verfügbar, wenn Git Bash installiert ist.

Claude Code startet PowerShell mit `-ExecutionPolicy Bypass` nur im Prozessbereich, sodass `.ps1`-Skripte und Modulimporte auf Standard-Windows-Installationen funktionieren, ohne die Richtlinie des Computers zu ändern. Der Prozessbereich-Bypass überschreibt nicht die Gruppenrichtlinie `MachinePolicy` oder `UserPolicy`, daher gelten Unternehmensrichtlinien weiterhin. Um stattdessen die effektive Ausführungsrichtlinie des Computers zu respektieren, setzen Sie `CLAUDE_CODE_POWERSHELL_RESPECT_EXECUTION_POLICY=1`.

<h3 id="shell-selection-in-settings-hooks-and-skills">
  Shell-Auswahl in Einstellungen, Hooks und Skills
</h3>

Drei zusätzliche Einstellungen steuern, wo PowerShell verwendet wird:

* `"defaultShell": "powershell"` in [`settings.json`](/docs/de/settings-reference#all-settings): leitet interaktive `!`-Befehle durch PowerShell weiter. Erfordert, dass das PowerShell-Tool aktiviert ist.
* `"shell": "powershell"` auf einzelnen [Command-Hooks](/docs/de/hooks#command-hook-fields): führt diesen Hook in PowerShell aus. Hooks starten PowerShell direkt, daher funktioniert dies unabhängig von `CLAUDE_CODE_USE_POWERSHELL_TOOL`.
* `shell: powershell` in [Skill-Frontmatter](/docs/de/skills#frontmatter-reference): führt `` !`command` ``-Blöcke in PowerShell aus. Erfordert, dass das PowerShell-Tool aktiviert ist.

Das gleiche Verhalten zum Zurücksetzen des Arbeitsverzeichnisses der Hauptsitzung, das im Abschnitt zum Bash-Tool beschrieben ist, gilt für PowerShell-Befehle, einschließlich der Umgebungsvariablen `CLAUDE_BASH_MAINTAIN_PROJECT_WORKING_DIR`.

Ab v2.1.196 bedeutet Exitcode 1 von `grep`, `rg`, `egrep`, `fgrep`, `findstr` und `git grep` keine Übereinstimmungen. Exitcode 1 von `git diff` bedeutet, dass Unterschiede vorhanden sind. Keines dieser Ergebnisse wird Claude als Befehlsfehler gemeldet. Für `robocopy` sind Exitcodes 0 bis 7 Informationsergebnisse, wie kopierte Dateien oder erkannte zusätzliche Dateien. Exitcodes von 8 oder höher zählen als Fehler.

<h3 id="windows-encoding-and-exit-codes">
  Windows-Codierung und Exitcodes
</h3>

Unter Windows erfordern die folgenden PowerShell-Codierungs- und Exitcode-Verhaltensweisen Claude Code v2.1.214 oder später:

* Umleitung mit `>` und `>>` schreibt UTF-8-Dateien auf PowerShell 5.1
* Claude Code codiert Text, der an die Standardeingabe eines nativen Befehls weitergeleitet wird, als UTF-8
* Claude Code erfasst Fehlerausgabe ohne ANSI-Escape-Sequenzen
* Ein Befehl, dessen untergeordneter Prozess auf die Standardeingabe wartet, erhält das Dateiende statt zu hängen
* Exitcode 1 von `where.exe` bedeutet keine Übereinstimmung, und von `fc.exe` und `diff.exe` bedeutet es, dass sich die Dateien unterscheiden. Wenn der Befehl eine Ausgabe erzeugt, behandelt Claude Code diesen Exitcode als eine gültige negative Antwort statt als Befehlsfehler. Claude Code meldet weiterhin eine stille Form, wie `where.exe /Q` oder eine Umleitung zu `$null`, als Fehler bei Exitcode 1

Vor v2.1.214 schrieb `>` auf PowerShell 5.1 UTF-16LE-Dateien, nicht-ASCII-weitergeleitete Eingaben kamen als `?` an, und Python-Skripte konnten mit einem `UnicodeEncodeError` abstürzen, wenn nicht-ASCII-Zeichen gedruckt wurden.

<h3 id="preview-limitations">
  Vorschaubeschränkungen
</h3>

Das PowerShell-Tool hat die folgenden bekannten Einschränkungen während der Vorschau:

* PowerShell-Profile werden nicht geladen
* Unter Windows wird Sandboxing nicht unterstützt

<h2 id="read-tool-behavior">
  Verhalten des Read-Tools
</h2>

Das Read-Tool nimmt einen Dateipfad und gibt den Inhalt mit Zeilennummern zurück. Claude ist angewiesen, immer absolute Pfade zu übergeben.

Standardmäßig gibt Read die Datei vom Anfang zurück. Wenn ein vollständiges Datei-Lesen das Token-Limit überschreitet, gibt Read die erste Seite mit einer `PARTIAL view`-Benachrichtigung zurück, die Claude mitteilt, wie viel der Datei er erhalten hat und wie er mit `offset` und `limit` mehr lesen kann. Ein Lesen, das einen expliziten `offset` oder `limit` übergibt und immer noch das Token-Limit überschreitet, gibt einen Fehler zurück.

Ein Lesen mit einem expliziten `limit` stoppt, sobald die ausgewählten Zeilen das überschreiten, was das Token-Limit jemals aufnehmen könnte, und gibt einen Fehler zurück, ohne den Rest des Bereichs zu laden. Der Fehler teilt Claude mit, einen kleineren `limit` zu verwenden, oder stattdessen mit [Grep](#grep-tool-behavior) nach spezifischem Inhalt zu suchen, wenn eine einzelne Zeile so groß ist. Vor v2.1.208 lud Claude Code den gesamten Bereich in den Speicher, bevor er ihn ablehnte, sodass das Lesen einer Datei mit einer extrem langen einzelnen Zeile zu Speichermangel führen konnte.

Das Lesen einer leeren Datei gibt eine Benachrichtigung zurück, dass die Datei existiert, aber ihr Inhalt leer ist, und ein `offset` nach der letzten Zeile gibt eine Benachrichtigung zurück, die die Zeilenanzahl der Datei angibt. Vor v2.1.208 gab das Lesen einer leeren Datei stattdessen die Past-the-End-Benachrichtigung zurück.

Read verarbeitet mehrere Dateitypen über reinen Text hinaus:

* **Bilder**: PNG, JPG und andere Bildformate werden als visueller Inhalt zurückgegeben, den Claude sehen kann, nicht als Rohdaten. Claude Code ändert die Größe großer Bilder und komprimiert sie neu, um die Bildgrößenlimits des Modells einzuhalten, bevor sie gesendet werden, sodass Claude möglicherweise eine herunterskalierte Version eines großen Screenshots sieht. Ab v2.1.196 wird ein Bild, das nach dieser Größenänderung immer noch größer als 500 KB ist, als JPEG mit reduzierter Qualität mit seinen unveränderten Pixeldimensionen neu codiert. Wenn Claude feine Pixel-Details in einem großen Bild übersieht, bitten Sie es, zuerst die Region von Interesse zuzuschneiden, beispielsweise mit ImageMagick über Bash.
* **PDFs**: Claude liest kurze `.pdf`-Dateien vollständig. Für PDFs länger als 10 Seiten liest es in Bereichen mit einem `pages`-Parameter, wie `"1-5"`, bis zu 20 Seiten auf einmal.
* **Jupyter-Notizbücher**: `.ipynb`-Dateien geben alle Zellen mit ihren Ausgaben zurück, einschließlich Code, Markdown und Visualisierungen. Claude Code weigert sich, eine Notizbuchdatei über 100 MB zu lesen; der Fehler teilt Claude mit, wie er stattdessen einen Teil des Notizbuchs liest, wie z. B. einen Ausschnitt von Zellen, mit einem Shell-Befehl.

Read liest nur Dateien, keine Verzeichnisse. Claude listet Verzeichnisinhalte mit einem Shell-Befehl wie `ls` auf.

<h2 id="sendfeedback-tool-behavior">
  Verhalten des SendFeedback-Tools
</h2>

Von Claude verfasstes Feedback ist ein Feedbackbericht über Claude Code, den Claude für Sie schreibt. Es erfordert Claude Code v2.1.238 oder später. Claude Code speichert jeden Entwurf auf Ihrem Computer unter `~/.claude/feedback/drafts/`, und nichts erreicht Anthropic, bis Sie ihn senden. Claude erstellt einen mit dem SendFeedback-Tool, wenn:

* Ein Tool oder Befehl immer wieder fehlschlägt
* Es kann Ihnen bei etwas nicht helfen, das Sie gefragt haben
* Sie weisen auf einen Fehler hin, den es gemacht hat, oder es bemerkt einen
* Sie bitten es, Feedback einzureichen

<h3 id="what-you-see-when-claude-drafts">
  Was Sie sehen, wenn Claude einen Entwurf erstellt
</h3>

Nachdem Claude einen Entwurf in die Warteschlange eingereiht hat, sehen Sie eine Karte über Ihrer Eingabeaufforderung mit dem Titel des Entwurfs. Drücken Sie `1`, um den Entwurf zu überprüfen, drücken Sie `2` zweimal, um ihn wie geschrieben zu senden, oder drücken Sie `0`, um ihn zu verwerfen. Ein verworfener Entwurf bleibt in Ihrer Warteschlange. Nachdem Sie eine Karte verworfen haben, fragt Claude Code, ob von Claude verfasstes Feedback deaktiviert werden soll. Es stellt die Frage nicht mehr, nachdem Sie zweimal abgelehnt haben.

Standardmäßig sehen Sie in einer Sitzung höchstens drei Karten; Anthropic kann dieses Limit vom Server aus ohne Veröffentlichung anpassen. Nach dem Limit und immer wenn Sie [`feedbackDrafts`](/docs/de/settings-reference#feedbackdrafts) auf `quiet` setzen, sehen Sie nur eine Anzahl der in der Warteschlange befindlichen Entwürfe in der Eingabeaufforderungsfußzeile.

<h3 id="review-and-edit-a-draft">
  Überprüfen und bearbeiten Sie einen Entwurf
</h3>

Führen Sie `/feedback` ohne Argument aus, um Ihre Warteschlange zu öffnen. Es listet jeden in der Warteschlange befindlichen Entwurf aus allen Ihren Sitzungen auf, einschließlich Entwürfe, deren Karten Sie verworfen oder nie gesehen haben. Wählen Sie einen Entwurf aus, um ihn zur Überprüfung zu öffnen, wo Sie:

* Den Titel, Bereich und Details bearbeiten können
* **Transkript senden** auf `yes` oder `no` setzen. Wenn das Transkript aus der Sitzung, in der Claude den Entwurf in die Warteschlange eingereiht hat, noch verfügbar ist, beginnt es mit `yes`, das diese Konversation an Anthropic sendet; `no` sendet nur den Bericht
* Den Entwurf senden, verwerfen oder in der Warteschlange für später belassen

Um stattdessen selbst einen Bericht zu schreiben, drücken Sie `w` für den Standard-Feedbackdialog. `/feedback` mit Text danach und `/bug` öffnen diesen Dialog direkt.

<h3 id="send-a-draft">
  Senden Sie einen Entwurf
</h3>

Wenn Sie einen Entwurf senden, reicht Claude Code ihn auf die gleiche Weise wie einen `/feedback`-Bericht ein, mit der gleichen [Aufbewahrung](/docs/de/data-usage#feedback-using-the-%2Ffeedback-command), und löscht den Entwurf von Ihrem Computer. Wenn Sie von der Karte aus senden, wird `✓ Sent` angezeigt; wenn Sie aus der Warteschlange senden, wird sie mit einer Quittungs-ID geschlossen.

Der Bericht enthält:

* Ihren Titel, Bereich und Details
* Umgebungsinformationen, wie Ihre Claude Code-Version, Betriebssystem und Modell
* Die IDs der letzten API-Anfragen
* Das Gesprächstranskript, wenn Sie **Transkript senden** auf dem Überprüfungsbildschirm auf `yes` belassen haben. Das Senden von der Karte aus enthält niemals das Transkript

Claude Code behält Ihr Arbeitsverzeichnis im lokalen Entwurf, um das Transkript zu finden, und sendet das Verzeichnis nicht.

In [Organisationen mit Null-Datenspeicherung](/docs/de/zero-data-retention#features-disabled-under-zdr) lässt Claude Code das Tool weg, wie es auch bei `/feedback` der Fall ist. Wenn eine Sitzung in einer solchen Organisation das Tool immer noch anbietet, bleiben Entwürfe auf Ihrem Computer, und das Senden schlägt mit `Feedback collection is not available for organizations with custom data retention policies.` fehl.

<h3 id="discard-or-keep-a-draft">
  Verwerfen oder behalten Sie einen Entwurf
</h3>

Wenn Sie einen Entwurf verwerfen, löscht Claude Code ihn von Ihrem Computer. Ein Entwurf, den Sie in der Warteschlange belassen, läuft nach 30 Tagen ab oder nach [`cleanupPeriodDays`](/docs/de/settings-reference#cleanupperioddays), wenn dieser Wert kürzer ist. Die Warteschlange enthält 10 Entwürfe aus allen Ihren Sitzungen, und wenn Claude einen elften in die Warteschlange einreiht, löscht Claude Code den ältesten. Wenn Sie `/exit` mit Entwürfen aus der Sitzung ausführen, die sich noch in der Warteschlange befinden, fragt Claude Code, ob Sie diese überprüfen oder verwerfen möchten, bevor Sie beenden.

<h3 id="turn-claude-drafted-feedback-off">
  Deaktivieren Sie von Claude verfasstes Feedback
</h3>

Setzen Sie **Claude-drafted feedback** in `/config` auf `off`, das die [`feedbackDrafts`](/docs/de/settings-reference#feedbackdrafts)-Einstellung schreibt, oder setzen Sie [`CLAUDE_CODE_SEND_FEEDBACK=0`](/docs/de/env-vars) für eine Sitzung. Mit beiden kann Claude keine Entwürfe in die Warteschlange einreihen. Um das Verfassen ohne Karten fortzusetzen, setzen Sie stattdessen `feedbackDrafts` auf `quiet`. Administratoren können `feedbackDrafts` in [verwalteten Einstellungen](/docs/de/managed-settings) setzen, was Vorrang vor Ihrer eigenen Einstellung hat.

<h3 id="sessions-without-claude-drafted-feedback">
  Sitzungen ohne von Claude verfasstes Feedback
</h3>

Claude Code enthält das Tool in interaktiven Terminalsitzungen auf Ihrem eigenen Computer, die die Claude API anstelle eines Cloud-Anbieters verwenden. Es lässt das Tool weg aus:

* Nicht-interaktiven `-p`-Läufen und [Agent SDK](/docs/de/agent-sdk/overview)-Sitzungen, die keinen Bildschirm zum Überprüfen der Warteschlange haben
* Cloud-Sitzungen wie [Claude Code im Web](/docs/de/claude-code-on-the-web), die nicht in die Warteschlange auf Ihrem Computer schreiben können
* Sitzungen auf [Amazon Bedrock](/docs/de/amazon-bedrock), [Claude Platform on AWS](/docs/de/claude-platform-on-aws), [Google Cloud's Agent Platform](/docs/de/google-vertex-ai) oder [Microsoft Foundry](/docs/de/microsoft-foundry)
* Sitzungen, in denen Sie [`CLAUDE_CODE_SEND_FEEDBACK=0`](/docs/de/env-vars) oder [`DISABLE_FEEDBACK_COMMAND=1`](/docs/de/env-vars) setzen, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` auf einen beliebigen nicht-leeren Wert setzen oder [Feature-Flag-Abruf](/docs/de/env-vars#features-that-need-feature-flag-fetching) deaktiviert haben
* Organisationen, die Produktfeedback deaktiviert haben, und [Organisationen mit Null-Datenspeicherung](/docs/de/zero-data-retention#features-disabled-under-zdr)

<h2 id="task-tool-availability">
  Verfügbarkeit des Task-Tools
</h2>

Die Task-Tracking-Tools, `TaskCreate`, `TaskGet`, `TaskUpdate`, `TaskList` und `TodoWrite`, sind standardmäßig nur auf Claude 3.x-Modellen, Opus 4 bis 4.7, Sonnet 4 bis 4.6 und Haiku 4.5 verfügbar. Überall dort, wo die Tools verfügbar sind, erhalten Sie die vier Task-Tools oder `TodoWrite`, wenn Sie [`CLAUDE_CODE_ENABLE_TASKS=0`](/docs/de/env-vars) setzen.

Bei jedem anderen Modell lässt Claude Code die Tools weg, es sei denn, Sie aktivieren sie. Das Gleiche gilt für eine Modell-ID, die Claude Code nicht erkennt, z. B. einen benutzerdefinierten Modellnamen, der über ein [LLM-Gateway](/docs/de/llm-gateway) bereitgestellt wird. Bei neueren Modellen verfolgt Claude mehrstufige Arbeiten ohne eine schriftliche Checkliste, und die Definitionen und Erinnerungen der Tools beanspruchen Kontext. Ohne die Tools fügt Claude nichts zur [Aufgabenliste](/docs/de/interactive-mode#task-list) hinzu, während es arbeitet.

Wenn Sie diese Tools auf einem Modell verwenden möchten, das sie standardmäßig nicht hat, führen Sie eines der folgenden Verfahren durch:

* Exportieren Sie [`CLAUDE_CODE_ENABLE_TODO_TOOLS=1`](/docs/de/env-vars), bevor Sie Claude Code starten, zum Beispiel `CLAUDE_CODE_ENABLE_TODO_TOOLS=1 claude`. Claude Code stellt dann die gleichen Tools auf jedem Modell und jedem Anbieter bereit
* Nennen Sie eines der Tools in [`--allowedTools`](/docs/de/cli-reference#cli-flags), zum Beispiel `claude --allowedTools TaskCreate`
* Listen Sie die Tools in [`--tools`](/docs/de/cli-reference#cli-flags) auf, was die integrierten Tools der Sitzung auf die genannten beschränkt. Fügen Sie die gewünschten Tools zusammen mit den anderen integrierten Tools ein, die Sie verwenden
* Im Agent SDK funktionieren die [`allowedTools` und `tools` Optionen](/docs/de/agent-sdk/todo-tracking#model-availability) genauso wie die beiden Flags

In [Hintergrund-Sitzungen](/docs/de/agent-view) und in [Claude Code im Web](/docs/de/claude-code-on-the-web) stellt Claude Code die gleichen Tools auf jedem Modell bereit, unabhängig davon, ob es aufgelistet ist oder nicht.

Claude Code gibt einem Subagenten die Tools nur, wenn Ihre Sitzung sie hat, auch wenn der Subagent ein anderes Modell ausführt. Ein In-Process-[Agent-Team](/docs/de/agent-teams)-Teamkollege folgt Ihrer Sitzung auf die gleiche Weise, während ein Teamkollege in seinem eigenen [geteilten Bereich](/docs/de/agent-teams#choose-a-display-mode) als separater Claude Code-Prozess ausgeführt wird, daher entscheidet sein eigenes Modell. Ohne die Task-Tools koordiniert sich ein Agent mit seinem Team durch Nachrichten statt über die [gemeinsame Aufgabenliste](/docs/de/agent-teams#assign-and-claim-tasks).

Die hier beschriebene Standardeinstellung gilt in Claude Code v2.1.268 und später.

<h2 id="webfetch-tool-behavior">
  WebFetch-Tool-Verhalten
</h2>

WebFetch nimmt eine URL und eine Eingabeaufforderung entgegen, die beschreibt, was extrahiert werden soll. Es ruft die Seite ab, konvertiert die Antwort in Markdown, wenn der Server HTML zurückgibt, und führt die Eingabeaufforderung gegen den Inhalt mit einem kleinen, schnellen Modell aus. Bei den meisten Abrufen erhält Claude die Antwort dieses Modells, nicht die rohe Seite. Der Konvertierungsschritt ist nicht konfigurierbar.

Dies macht WebFetch absichtlich verlustbehaftet. Die Extraktions-Eingabeaufforderung bestimmt, was Claude erreicht, daher kann ein Ergebnis, das besagt, dass eine Seite etwas nicht erwähnt, nur bedeuten, dass die Eingabeaufforderung nicht danach gefragt hat. Bitten Sie Claude, die Seite erneut mit einer spezifischeren Eingabeaufforderung abzurufen, oder verwenden Sie `curl` über Bash für die unverarbeitete Seite.

Einige Verhaltensweisen prägen die Antwort, die Claude erhält:

* WebFetch lehnt `localhost` und jeden anderen Hostnamen ohne einen Punkt ab, z. B. einen bloßen Intranet-Namen, bevor eine Anfrage gestellt wird. Der [Fehler, den es zurückgibt](/docs/de/errors#webfetch-cannot-fetch-localhost), teilt Claude mit, dass lokale Server mit `curl` über Bash erreicht werden sollen.
* HTTP-URLs werden automatisch auf HTTPS aktualisiert.
* Große Seiten werden vor der Verarbeitung auf ein festes Zeichenlimit gekürzt.
* WebFetch speichert jede Antwort standardmäßig 15 Minuten lang zwischen, sodass wiederholte Abrufe derselben URL schnell zurückgegeben werden. Auf Claude Code v2.1.233 oder später können Sie [`CLAUDE_CODE_WEBFETCH_CACHE_TTL_MS`](/docs/de/env-vars#variables) setzen, um zu ändern, wie lange WebFetch jede Antwort speichert.
* Eine Seite, die nicht innerhalb von fünf Minuten heruntergeladen wird, einschließlich aller Umleitungen, denen WebFetch folgt, schlägt mit einem Deadline-Fehler fehl. Auf Claude Code v2.1.268 oder später können Sie [`CLAUDE_CODE_WEBFETCH_DEADLINE_MS`](/docs/de/env-vars#variables) setzen, um das Limit zu ändern, oder auf `0`, um es zu entfernen.
* Wenn eine URL zu einem anderen Host umleitet, gibt WebFetch ein Textergebnis zurück, das die ursprüngliche URL und das Umleitungsziel benennt, anstatt ihr zu folgen. Claude ruft dann die neue URL mit einem zweiten WebFetch-Aufruf ab.
* Wenn der Extraktionsschritt eine überladene API trifft, versucht Claude Code es mit Backoff erneut; ein Abruf, der immer noch fehlschlägt, gibt ein Fehlerergebnis zurück. Vor v2.1.212 konnte der API-Fehlertext Claude erreichen, als wäre er der extrahierte Seiteninhalt.

In Manual und `acceptEdits` [Berechtigungsmodi](/docs/de/permission-modes) fordert WebFetch vor dem Abruf auf, außer für Domänen, die Ihre [Berechtigungsregeln](/docs/de/permissions#manage-permissions) bereits zulassen oder ablehnen, und eine integrierte Reihe von genehmigten Dokumentationsdomänen, die ohne Eingabeaufforderung abgerufen werden. Unabhängig davon, was Ihre Regeln zulassen, muss ein Abruf auch zuerst die [WebFetch-Domänensicherheitsprüfung](/docs/de/data-usage#webfetch-domain-safety-check) bestehen; dieser Abschnitt behandelt, was die Prüfung sendet und die Einstellung, die sie überspringt. Die Eingabeaufforderung bietet drei Optionen:

* **Ja**: genehmigt diesen Abruf nur. Der nächste WebFetch-Aufruf fordert erneut auf, auch für dieselbe Domäne.
* **Ja, und frag mich nicht mehr für `<domain>`**: genehmigt den Abruf und speichert eine `WebFetch(domain:...)`-Zulassungsregel für diese Domäne in `.claude/settings.local.json` für dieses Repository. Siehe [wie gespeicherte Genehmigungen bestehen bleiben](/docs/de/permissions#permission-system). Wenn Ihre Organisation [`allowManagedPermissionRulesOnly`](/docs/de/permissions#managed-only-settings) setzt, blendet Claude Code diese Option aus.
* **Nein, und sag Claude, was er stattdessen tun soll**: lehnt den Abruf ab.

Um eine Domäne im Voraus ohne Eingabeaufforderung zuzulassen, fügen Sie eine Zulassungsregel wie `WebFetch(domain:example.com)` hinzu; `WebFetch(domain:*)` lässt jede Domäne zu. Die `auto` und `bypassPermissions` [Berechtigungsmodi](/docs/de/permissions#permission-modes) überspringen die Eingabeaufforderung, außer für eine Domäne, die eine explizite `ask`-Regel erfüllt.

Eine explizite `WebFetch(domain:...)`-Regel in `deny`, `ask` oder `allow` hat Vorrang vor der genehmigten Reihe, sodass Sie eine genehmigte Domäne blockieren oder eine Eingabeaufforderung dafür verlangen können.

WebFetch setzt einen `User-Agent`-Header, der mit `Claude-User` beginnt, und einen `Accept`-Header, der Markdown gegenüber HTML bevorzugt, sodass Server, die Inhaltsaushandlung unterstützen, Markdown direkt zurückgeben können.

Sandbox-Befehle erben nicht die integrierte Reihe von genehmigten Dokumentationsdomänen von WebFetch. Um einem Sandbox-Befehl den Zugriff auf eine Domäne ohne Eingabeaufforderung zu ermöglichen, fügen Sie die Domäne zu [`allowedDomains`](/docs/de/settings-reference#sandbox-network-alloweddomains) hinzu oder lassen Sie sie mit einer `WebFetch(domain:...)`-Regel zu, die die [Sandbox auch berücksichtigt](/docs/de/sandboxing#network-isolation). WebFetch liest die Sandbox-Zulassungsliste im Gegenzug nie, daher verhindert das Hinzufügen einer Domäne zu einer Sandbox- oder Organisations-Netzwerk-Zulassungsliste nicht, dass WebFetch danach fragt.

<h2 id="websearch-tool-behavior">
  WebSearch-Tool-Verhalten
</h2>

WebSearch führt eine Abfrage gegen Anthropics [Web-Suche](https://platform.claude.com/docs/en/agents-and-tools/tool-use/web-search-tool)-Backend durch und gibt Ergebnistitel und URLs zurück. Es ruft die Ergebnisseiten nicht ab. Um eine Seite zu lesen, die Claude in Suchergebnissen findet, folgt es mit [WebFetch](#webfetch-tool-behavior) nach.

Das Tool kann bis zu acht Backend-Suchen pro Aufruf durchführen und die Suche intern verfeinern, bevor es Ergebnisse zurückgibt. Claude kann Ergebnisse mit `allowed_domains` auf bestimmte Hosts beschränken oder mit `blocked_domains` ausschließen. Die beiden Listen können nicht in einem einzigen Aufruf kombiniert werden.

Wenn die Suchanfrage ein überladenes API trifft, versucht Claude Code es mit Backoff erneut; ein Aufruf, der immer noch fehlschlägt, gibt ein Fehlerergebnis zurück. Vor v2.1.212 konnte der API-Fehlertext Claude erreichen, als wäre er ein Suchergebnis.

WebSearch-Berechtigungsregeln benötigen keinen Spezifizierer. Ein einfacher `WebSearch`-Eintrag in `allow` oder `deny` ist die einzige Form.

Das Such-Backend ist nicht konfigurierbar. Um mit einem anderen Anbieter zu suchen, fügen Sie einen [MCP-Server](/docs/de/mcp) hinzu, der ein Such-Tool bereitstellt.

<Note>
  WebSearch ist auf der Claude API und [Claude Platform on AWS](/docs/de/claude-platform-on-aws) verfügbar. Auf Microsoft Foundry erfordert es eine [auf Anthropic gehostete Bereitstellung](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options): auf Azure gehostete Bereitstellungen unterstützen keine serverseitigen Tools, daher schlägt der WebSearch-Aufruf fehl. Auf Googles Agent Platform funktioniert es mit Claude 4 und späteren Modellen, einschließlich Opus, Sonnet und Haiku. Amazon Bedrock stellt das serverseitige Web-Such-Tool nicht bereit.
</Note>

<h3 id="session-search-limit">
  Suchgrenze für Sitzungen
</h3>

Eine Sitzung kann maximal 200 WebSearch-Aufrufe durchführen, gezählt über die Hauptkonversation und jeden [Subagenten](/docs/de/sub-agents), den sie erzeugt, sodass Suchen, die von parallelen Forschungs-Fan-Outs durchgeführt werden, gegen das gleiche Limit zählen. Das Limit erfordert Claude Code v2.1.212 oder später. Wenn Claude das Limit erreicht, geben weitere Aufrufe einen Hinweis zurück, der Claude auffordert, mit den bereits gesammelten Informationen fortzufahren, anstatt einen Fehler zurückzugeben, der einen erneuten Versuch einladen würde. Sie sehen den Hinweis nicht: ein begrenzter Aufruf erscheint in der Konversation als eine Suche, die nichts getan hat, und wenn Claude weitere Suchen benötigt, teilt der Hinweis ihm mit, Sie zu bitten, das Limit zu erhöhen.

Setzen Sie die Umgebungsvariable [`CLAUDE_CODE_MAX_WEB_SEARCHES_PER_SESSION`](/docs/de/env-vars), um die Obergrenze zu ändern; sie akzeptiert eine positive ganze Zahl, sodass die Obergrenze erhöht, aber nicht deaktiviert werden kann. Das Ausführen von [`/clear`](/docs/de/commands#all-commands) setzt den Zähler zurück. Wenn Arbeiten, die immer noch [Subagenten](/docs/de/sub-agents) erzeugen können, das Löschen überstehen, wie z. B. ein laufender Workflow, wird der Zähler stattdessen übertragen.

<h2 id="write-tool-behavior">
  Verhalten des Write-Tools
</h2>

Das Write-Tool erstellt eine neue Datei oder überschreibt eine vorhandene Datei mit dem vollständigen bereitgestellten Inhalt. Es fügt nicht an und führt nicht zusammen.

Ob Claude eine vorhandene Datei in der aktuellen Konversation lesen muss, bevor er sie überschreibt, hängt vom Modell und der Datei ab:

* Claude Opus 4.6, Claude Haiku 4.5 und ältere Modelle erfordern immer das Lesen, daher schlägt ein Write zu einer ungelesenen vorhandenen Datei mit einem Fehler fehl.
* Neuere Modelle können eine Datei überschreiben, die sie in dieser Sitzung nie gelesen haben, unter den gleichen Bedingungen wie [read-before-edit](#edit-tool-behavior): Das Lesen würde keine Berechtigungsaufforderung erfordern und das Read-Tool ist verfügbar.
* Jupyter-Notebooks und Dateien, die Claude nur teilweise mit einer [`PARTIAL view`-Benachrichtigung](#read-tool-behavior) gelesen hat, erfordern das Lesen auf jedem Modell.

Diese Einschränkung gilt nicht für neue Dateien. Vor v2.1.228 erforderten alle Modelle das Lesen vor dem Überschreiben einer vorhandenen Datei.

Das Anzeigen der Datei mit Bash erfüllt diese Anforderung auch unter den gleichen Regeln, die unter [Edit tool behavior](#edit-tool-behavior) beschrieben sind.

Für teilweise Änderungen an einer vorhandenen Datei verwendet Claude Edit statt Write.

<h2 id="check-which-tools-are-available">
  Überprüfen Sie, welche Werkzeuge verfügbar sind
</h2>

Ihr genaues Werkzeugsatz hängt von Ihrem Anbieter, Ihrer Plattform und Ihren Einstellungen ab. Um zu überprüfen, was in einer laufenden Sitzung geladen ist, fragen Sie Claude direkt:

```text theme={null}
What tools do you have access to?
```

Claude gibt eine Zusammenfassung im Gesprächsstil. Für genaue MCP-Werkzeugnamen führen Sie `/mcp` aus.

<Note>
  Das [Advisor-Tool](/docs/de/advisor) ist ein [Server-Tool](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool), das die API ausführt, anstatt ein Tool, das Claude Code implementiert. Es hat keinen Namen, auf den Sie in Berechtigungsregeln oder Hook-Matchern verweisen können.
</Note>

<h2 id="see-also">
  Siehe auch
</h2>

* [MCP-Server](/docs/de/mcp): Fügen Sie benutzerdefinierte Werkzeuge durch Verbindung externer Server hinzu
* [Berechtigungen](/docs/de/permissions): Berechtigungssystem, Regelsyntax und werkzeugspezifische Muster
* [Subagents](/docs/de/sub-agents): Konfigurieren Sie Werkzeugzugriff für Subagents
* [Hooks](/docs/de/hooks-guide): Führen Sie benutzerdefinierte Befehle vor oder nach der Werkzeugausführung aus
