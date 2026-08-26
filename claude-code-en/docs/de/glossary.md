> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Glossar

> Definitionen für Claude Code-Terminologie. Erfahren Sie, was Agentic Loop, Komprimierung, CLAUDE.md, Hooks, Subagenten, MCP und andere Kernkonzepte bedeuten.

Dieses Glossar definiert Claude Code-Terminologie. Jeder Eintrag verlinkt auf die Seite, auf der das Konzept ausführlich behandelt wird. Für Modell-Konzepte wie Tokens, Temperatur und RAG siehe das [Plattform-Glossar](https://platform.claude.com/docs/de/about-claude/glossary).

<h2 id="a">
  A
</h2>

<h3 id="agent-teams">
  Agent Teams
</h3>

Mehrere unabhängige Claude Code-Sitzungen, die von einem Team-Lead koordiniert werden, mit einer gemeinsamen Aufgabenliste und Peer-to-Peer-Messaging. Im Gegensatz zu [Subagenten](#subagent), die innerhalb einer einzelnen Sitzung ausgeführt werden und nur dem übergeordneten Element berichten, hat jedes Teammate sein eigenes Kontextfenster und Sie können direkt mit jedem von ihnen interagieren. Agent Teams sind experimentell und müssen durch Setzen von `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` aktiviert werden.

Weitere Informationen: [Agent Teams ausführen](/docs/de/agent-teams)

<h3 id="agentic-coding">
  Agentic Coding
</h3>

Ein Workflow, bei dem die KI Dateien lesen, Befehle ausführen und Änderungen autonom vornehmen kann, während Sie zuschauen, umleiten oder sich entfernen, im Gegensatz zu Chat-basierten Assistenten, die nur Text antworten, den Sie selbst anwenden müssen. Claude Code ist agentic, weil es [Tools](#tool) hat, die es handeln lassen, nicht nur beraten.

Weitere Informationen: [Wie Claude Code funktioniert](/docs/de/how-claude-code-works)

<h3 id="agentic-harness">
  Agentic Harness
</h3>

Die Tools, Kontextverwaltung und Ausführungsumgebung, die ein Sprachmodell in einen fähigen Coding-Agenten verwandeln. Claude Code ist das Harness; Claude ist das Modell darin. Das Harness bietet Dateizugriff, Shell-Ausführung, Berechtigungsverwaltung, Speicherladen und die Schleife, die Aktionen zusammenkettet.

Weitere Informationen: [Wie Claude Code funktioniert](/docs/de/how-claude-code-works)

<h3 id="agentic-loop">
  Agentic Loop
</h3>

Der Zyklus, den Claude für jede Aufgabe durchläuft: Kontext sammeln, Maßnahmen ergreifen, Ergebnisse überprüfen und wiederholen, bis fertig. Jede Tool-Nutzung gibt Informationen zurück, die den nächsten Schritt informieren. Sie können die Schleife jederzeit unterbrechen, um umzuleiten. Die meisten Erweiterungspunkte, einschließlich [Hooks](#hook), [Skills](#skill) und [MCP](#mcp-model-context-protocol), verbinden sich mit spezifischen Phasen dieser Schleife.

Weitere Informationen: [Wie Claude Code funktioniert](/docs/de/how-claude-code-works#the-agentic-loop)

<h3 id="artifact">
  Artifact
</h3>

Eine Live-, interaktive Webseite, die Claude Code aus Ihrer Sitzung auf einer privaten URL auf claude.ai veröffentlicht, damit Sie die Ausgabe visuell sehen oder teilen können, anstatt Terminaltext zu lesen. Die Seite wird aktualisiert, wenn die Sitzung erneut veröffentlicht wird. Artifacts, die Sie aus Claude Code erstellen, erscheinen in derselben Galerie wie Artifacts, die in claude.ai-Gesprächen erstellt wurden. Die Freigabe hängt von Ihrem Plan ab: Bei Pro und Max ein öffentlicher Link, den jeder öffnen kann; bei Team und Enterprise, Freigabe innerhalb Ihrer Organisation, plus öffentliche Links, sobald ein Owner diese aktiviert.

Weitere Informationen: [Sitzungsausgabe als Artifacts teilen](/docs/de/artifacts)

<h3 id="auto-memory">
  Auto Memory
</h3>

Notizen, die Claude für sich selbst basierend auf Ihren Korrektionen und Vorlieben schreibt, gespeichert pro Git-Repository unter `~/.claude/projects/`. Alle Worktrees desselben Repositories teilen sich ein Auto Memory-Verzeichnis. Die ersten 200 Zeilen oder 25 KB des `MEMORY.md`-Index werden zu Beginn jeder Sitzung geladen. Auto Memory ist das von Claude geschriebene Gegenstück zu [CLAUDE.md](#claude-md), das Sie schreiben.

Weitere Informationen: [Auto Memory](/docs/de/memory#auto-memory)

<h3 id="auto-mode">
  Auto Mode
</h3>

Ein [Berechtigungsmodus](#permission-mode), bei dem ein separates Klassifizierungsmodell jede Aktion im Hintergrund überprüft, sodass die meisten ohne Genehmigungsaufforderungen ausgeführt werden; explizite Ask-Regeln werden weiterhin angefordert. Der Klassifizierer blockiert Scope-Eskalation, nicht vertrauenswürdige Infrastruktur und [Prompt Injection](#prompt-injection). Er sieht niemals Tool-Ergebnisse, daher können injizierte Anweisungen seine Entscheidungen nicht beeinflussen.

Weitere Informationen: [Aufforderungen mit Auto Mode eliminieren](/docs/de/permission-modes#eliminate-prompts-with-auto-mode)

<h2 id="b">
  B
</h2>

<h3 id="bare-mode">
  Bare Mode
</h3>

Ein Startup-Flag, `--bare`, das die automatische Erkennung von Hooks, Skills, Plugins, MCP-Servern, Auto Memory und CLAUDE.md überspringt. Nur Flags, die Sie explizit übergeben, haben Auswirkungen. Empfohlen für CI und Skript-Aufrufe, bei denen Sie identisches Verhalten über Maschinen hinweg unabhängig von lokaler Konfiguration benötigen.

Weitere Informationen: [Schneller starten mit Bare Mode](/docs/de/headless#start-faster-with-bare-mode)

<h3 id="bundled-skills">
  Bundled Skills
</h3>

Prompt-basierte Playbooks, die mit Claude Code enthalten sind, wie `/batch`, `/code-review`, `/debug` und `/loop`. Im Gegensatz zu integrierten Befehlen, die feste Logik ausführen, geben Bundled Skills Claude eine detaillierte Aufforderung und lassen es die Arbeit orchestrieren, sodass sie Agenten spawnen, Dateien lesen und sich an Ihre Codebasis anpassen können.

Weitere Informationen: [Bundled Skills](/docs/de/skills#bundled-skills)

<h2 id="c">
  C
</h2>

<h3 id="channel">
  Channel
</h3>

Ein [MCP-Server](#mcp-model-context-protocol), der Ereignisse in Ihre laufende Sitzung pusht, damit Claude auf Dinge reagieren kann, die passieren, während Sie weg vom Terminal sind. Channels können bidirektional sein: Claude liest ein eingehendes Ereignis und antwortet über denselben Channel zurück. Telegram, Discord und iMessage sind in der Forschungsvorschau enthalten.

Weitere Informationen: [Channels](/docs/de/channels)

<h3 id="checkpoint">
  Checkpoint
</h3>

Ein Wiederherstellungspunkt, der bei jedem Prompt erstellt wird, den Sie senden. Claude Code erstellt Snapshots von Dateien vor jeder Bearbeitung, damit ein Checkpoint diese zurücksetzen kann. Drücken Sie `Esc` zweimal oder führen Sie `/rewind` aus, um Code, Konversation oder beides auf einen früheren Punkt zurückzusetzen, oder um einen Teil der Konversation aus einer ausgewählten Nachricht zusammenzufassen. Checkpoints werden mit der Konversation gespeichert, sodass eine fortgesetzte Sitzung immer noch zu ihnen `/rewind` kann. Sie sind getrennt von Git und verfolgen keine Änderungen, die durch das Bash-Tool vorgenommen wurden.

Weitere Informationen: [Checkpointing](/docs/de/checkpointing)

<h3 id="claude-directory">
  `.claude` Verzeichnis
</h3>

Das Verzeichnis, in dem Claude Code projektbezogene Konfiguration liest: Einstellungen, Hooks, Skills, Subagenten, Regeln und Auto Memory. Ein Projekt hat `.claude/` in seiner Wurzel; Ihre Benutzer-Level-Standardwerte befinden sich unter `~/.claude/`.

Weitere Informationen: [Das `.claude` Verzeichnis](/docs/de/claude-directory)

<h3 id="claude-md">
  CLAUDE.md
</h3>

Eine Markdown-Datei mit persistenten Anweisungen, die Sie für Claude schreiben, geladen zu Beginn jeder Sitzung als Benutzernachricht nach dem System-Prompt. Legen Sie Projektkonventionen, Architekturnotizen und „immer X tun"-Regeln hier ab. CLAUDE.md überlebt [Komprimierung](#compaction) und wird danach frisch von der Festplatte neu gelesen.

Sie können CLAUDE.md im Projektbereich in `./CLAUDE.md` oder `./.claude/CLAUDE.md`, im Benutzerbereich in `~/.claude/CLAUDE.md` oder als [verwaltete Richtlinie](#managed-settings) für Ihre Organisation platzieren. Alle gefundenen Dateien werden in den Kontext verkettet, anstatt sich gegenseitig zu überschreiben, geordnet vom breitesten Bereich zum spezifischsten.

Weitere Informationen: [CLAUDE.md-Dateien](/docs/de/memory#claude-md-files)

<h3 id="command">
  Command
</h3>

Eine wiederverwendbare Anweisung, die Sie durch Eingabe von `/name` in der Aufforderung aufrufen. Integrierte Befehle wie `/clear`, `/model` und `/compact` steuern die Sitzung. Sie können Ihre eigenen Befehle als Dateien in `.claude/commands/` definieren oder sie aus einem [Plugin](#plugin) installieren. [Skills](#skill) sind die empfohlene Methode zum Verpacken von mehrstufigen Befehlen.

Weitere Informationen: [Commands](/docs/de/commands) · [Skills](/docs/de/skills)

<h3 id="compaction">
  Compaction
</h3>

Automatische Zusammenfassung Ihrer Konversation, wenn sich das [Kontextfenster](#context-window) seinem Limit nähert. Ältere Tool-Ausgaben werden zuerst gelöscht, dann wird die Konversation zusammengefasst. Projekt-Root CLAUDE.md und Auto Memory überleben die Komprimierung und werden von der Festplatte neu geladen; Anweisungen, die nur in der Konversation gegeben werden, können verloren gehen. Führen Sie `/compact` aus, um manuell auszulösen, optional mit einem Fokus wie `/compact focus on the API changes`.

Weitere Informationen: [Was Komprimierung überlebt](/docs/de/context-window#what-survives-compaction) · [Wenn der Kontext voll wird](/docs/de/how-claude-code-works#when-context-fills-up)

<h3 id="context-window">
  Context Window
</h3>

Das Arbeitsspeicher für eine Sitzung, das Konversationsverlauf, Dateiinhalte, Befehlsausgaben, CLAUDE.md, Auto Memory, geladene Skills und Systeminstruktionen enthält. Während Sie arbeiten, füllt sich der Kontext, bis [Komprimierung](#compaction) ihn zusammenfasst. Führen Sie `/context` aus, um zu sehen, was Platz verwendet. Für das zugrunde liegende Modellkonzept siehe das [Plattform-Glossar](https://platform.claude.com/docs/de/about-claude/glossary#context-window).

Weitere Informationen: [Erkunden Sie das Kontextfenster](/docs/de/context-window)

<h2 id="d">
  D
</h2>

<h3 id="dispatch">
  Dispatch
</h3>

Ein von Telefon initiierter Task-Router, der eine Claude Code-Sitzung in der Desktop-App spawnt, wenn Sie eine Coding-Aufgabe von der Claude Mobile App senden. Ihre Aufforderung leitet automatisch zum richtigen Tool weiter. Verfügbar auf Pro- und Max-Plänen.

Weitere Informationen: [Sitzungen von Dispatch](/docs/de/desktop#sessions-from-dispatch)

<h2 id="e">
  E
</h2>

<h3 id="effort-level">
  Effort Level
</h3>

Eine Einstellung, die steuert, wie viel des adaptiven Reasoning-Thinking-Budgets Claude bei jedem Turn verwendet. Höherer Aufwand bedeutet mehr Thinking-Tokens und tiefere Überlegungen; niedrigerer Aufwand ist schneller und billiger. Effort wird auf Fable 5, auf Opus 4.6 und später sowie auf Sonnet 4.6 und später unterstützt.

Weitere Informationen: [Passen Sie das Effort Level an](/docs/de/model-config#adjust-effort-level)

<h3 id="extended-thinking">
  Extended Thinking
</h3>

Sichtbares schrittweises Reasoning, das das Modell vor der Antwort durchführt. Sie können es mit dem [Effort Level](#effort-level) anpassen oder Thinking-Tokens mit `MAX_THINKING_TOKENS` auf Modellen mit einem festen Thinking-Budget begrenzen. Thinking erscheint in grauem kursivem Text im Terminal.

Weitere Informationen: [Verwenden Sie Extended Thinking](/docs/de/model-config#extended-thinking)

<h2 id="h">
  H
</h2>

<h3 id="hook">
  Hook
</h3>

Ein benutzerdefinierter Handler, der automatisch an einem bestimmten Punkt im Lebenszyklus von Claude Code ausgeführt wird, z. B. bevor ein Tool ausgeführt wird, nach einer Dateibearbeitung oder beim Sitzungsstart. Handler können ein Shell-Befehl, HTTP-Endpunkt, MCP-Tool, LLM-Aufforderung oder Subagent sein. Hooks sind deterministisch: Sie werden an festen Lebenszykluspunkten ausgelöst, nicht nach Ermessen des Modells.

Eine Hook-Konfiguration hat drei Ebenen:

* **Hook Event**: der Lebenszykluspunkt
* **Matcher**: filtert, welche Ereignisse ihn auslösen
* **Hook Handler**: was ausgeführt wird

Weitere Informationen: [Erste Schritte mit Hooks](/docs/de/hooks-guide) · [Hooks-Referenz](/docs/de/hooks)

<h2 id="m">
  M
</h2>

<h3 id="managed-settings">
  Managed Settings
</h3>

Einstellungen, die organisationsweit von IT oder DevOps durchgesetzt werden und von Anthropics Servern über die Admin-Konsole oder auf einem OS-Level-Pfad außerhalb von `~/.claude` bereitgestellt werden. Benutzer- und Projekteinstellungen können verwaltete Einstellungen nicht überschreiben. Die servergesteuerte Bereitstellung gilt für [berechtigte Konfigurationen](/docs/de/server-managed-settings#platform-availability); siehe [Sicherheitsaspekte](/docs/de/server-managed-settings#security-considerations). Verwenden Sie dies für Sicherheitsrichtlinien, Compliance-Anforderungen oder standardisierte Tools über eine Flotte.

Weitere Informationen: [Server-verwaltete Einstellungen](/docs/de/server-managed-settings) · [Einstellungsdateien](/docs/de/settings#settings-files)

<h3 id="mcp-model-context-protocol">
  MCP (Model Context Protocol)
</h3>

Ein offener Standard für die Verbindung von KI-Tools mit externen Datenquellen und Diensten. MCP-Server geben Claude neue Tools für Slack, Jira, Datenbanken, Browser und Hunderte anderer Integrationen. Sie verbinden Server über `/mcp` oder durch Hinzufügen zu `.mcp.json`. Für das Protokoll selbst siehe das [Plattform-Glossar](https://platform.claude.com/docs/de/about-claude/glossary#mcp-model-context-protocol).

Weitere Informationen: [Model Context Protocol](/docs/de/mcp)

<h3 id="mcp-tool-search">
  MCP Tool Search
</h3>

Ein Kontextsparmechanismus, der MCP-Tool-Schemas bis zur Notwendigkeit aufschiebt. Nur Tool-Namen werden beim Start geladen; Claude ruft das vollständige Schema bei Bedarf ab, wenn es sich entscheidet, ein bestimmtes Tool zu verwenden. Dies verhindert, dass untätige MCP-Server viel Kontext verbrauchen.

Weitere Informationen: [Mit MCP Tool Search skalieren](/docs/de/mcp#scale-with-mcp-tool-search)

<h2 id="n">
  N
</h2>

<h3 id="non-interactive-mode">
  Non-interactive mode
</h3>

Ein Modus, der eine einzelne Aufforderung ausführt und ohne eine interaktive Eingabeaufforderung beendet wird, aufgerufen mit `-p` oder `--print`. Wird für CI, Skripte und Piping verwendet. Die Ausführung wird weiterhin als wiederaufnehmbare Sitzung gespeichert, es sei denn, Sie übergeben `--no-session-persistence`. Das [Agent SDK](/docs/de/agent-sdk/overview) ist das Python- und TypeScript-Äquivalent. Früher Headless Mode genannt.

Weitere Informationen: [Claude Code programmgesteuert ausführen](/docs/de/headless)

<h2 id="o">
  O
</h2>

<h3 id="output-style">
  Output Style
</h3>

Eine Konfiguration, die Claudes System-Prompt ändert, um Antwortverhalten, Ton oder Format zu ändern. Output Styles schalten die Software-Engineering-spezifischen Teile des Standard-System-Prompts aus, im Gegensatz zu [CLAUDE.md](#claude-md), das als Benutzernachricht nach dem System-Prompt bereitgestellt wird. Integrierte Styles umfassen Default, Proactive, Explanatory und Learning.

Weitere Informationen: [Output Styles](/docs/de/output-styles)

<h2 id="p">
  P
</h2>

<h3 id="permission-mode">
  Permission Mode
</h3>

Das Baseline-Genehmigungsverhalten für die Sitzung. Wechseln Sie mit `Shift+Tab` in der CLI oder verwenden Sie den Mode-Selector in VS Code, Desktop und claude.ai. Verfügbare Modi sind `default`, `acceptEdits`, `plan`, `auto`, `dontAsk` und `bypassPermissions`.

Der `default` Mode wird in der CLI und in den VS Code- und JetBrains-Erweiterungen als Manual bezeichnet, und Claude Code akzeptiert `manual` als Alias für den Wert.

Weitere Informationen: [Wählen Sie einen Permission Mode](/docs/de/permission-modes)

<h3 id="permission-rule">
  Permission Rule
</h3>

Ein Einstellungseintrag, der eine Tool-Invokation basierend auf dem Tool-Namen und Argument-Muster erlaubt, fragt oder verweigert. Regeln werden in der Reihenfolge deny→ask→allow ausgewertet, der erste Match gewinnt. Permission Rules sind feinkörnige Kontrollen, die auf dem breiteren [Permission Mode](#permission-mode) aufgelagert sind.

Weitere Informationen: [Konfigurieren Sie Berechtigungen](/docs/de/permissions)

<h3 id="plan-mode">
  Plan Mode
</h3>

Ein [Permission Mode](#permission-mode), bei dem Claude Änderungen recherchiert und vorschlägt, ohne Ihre Quelldateien zu bearbeiten. Es kann lesen, suchen und Explorations-Befehle ausführen, dann einen Plan zur Genehmigung präsentieren, bevor etwas berührt wird. Geben Sie Plan Mode mit `/plan` ein oder drücken Sie `Shift+Tab`.

Weitere Informationen: [Analysieren Sie vor der Bearbeitung mit Plan Mode](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode)

<h3 id="plugin">
  Plugin
</h3>

Ein Bundle von Skills, Hooks, Subagenten und MCP-Servern, verpackt als eine einzelne installierbare Einheit. Plugin-Skills werden als `plugin-name:skill-name` namensraum, sodass mehrere Plugins koexistieren. Verteilen Sie Plugins über Teams über einen [Marketplace](/docs/de/plugin-marketplaces).

Weitere Informationen: [Plugins](/docs/de/plugins)

<h3 id="project-trust">
  Project Trust
</h3>

Ein Dialog, der ein Verzeichnis akzeptiert, bevor Claude Code seine Konfiguration lädt. Die Akzeptanz wird pro Projektverzeichnis gespeichert, außer in Ihrem Home-Verzeichnis, wo das Vertrauen nur für die aktuelle Sitzung gilt und die Eingabeaufforderung bei jedem Start erneut angezeigt wird. Trust gates die automatische Installation von Marketplace-Plugins und die Ausführung von projektdefinierten Hooks. Ein Verzeichnis zu vertrauen bedeutet, dass seine `.claude/settings.json`, `.mcp.json` und andere Konfigurationsdateien wirksam werden.

Weitere Informationen: [Das `.claude` Verzeichnis](/docs/de/claude-directory)

<h3 id="prompt-injection">
  Prompt Injection
</h3>

Feindselige Anweisungen, die in einer Datei, Webseite oder Tool-Ergebnis eingebettet sind und versuchen, Claude zu Aktionen umzuleiten, die Sie nie angefordert haben. Die Abwehrmechanismen von Claude Code umfassen das Berechtigungssystem, Befehlsblocklisten und Vertrauensüberprüfung. [Auto Mode](#auto-mode) fügt eine serverseitige Sonde hinzu, die Tool-Ergebnisse auf verdächtige Inhalte scannt, und einen Klassifizierer, der niemals Tool-Ergebnisse sieht, sodass injizierter Text seine Genehmigungsentscheidungen nicht beeinflussen kann.

Weitere Informationen: [Schützen Sie sich vor Prompt Injection](/docs/de/security#protect-against-prompt-injection)

<h2 id="r">
  R
</h2>

<h3 id="remote-control">
  Remote Control
</h3>

Eine Möglichkeit, eine lokale Claude Code-Sitzung von Ihrem Telefon oder Browser über claude.ai fortzusetzen. Ihr Code bleibt auf Ihrem Computer; nur die Benutzeroberfläche ist remote. Unterschiedlich von Claude Code im Web, das in einer Cloud-Sandbox ausgeführt wird.

Weitere Informationen: [Remote Control](/docs/de/remote-control)

<h3 id="rules">
  Rules
</h3>

Modulare Anweisungsdateien in `.claude/rules/`, die zusammen mit CLAUDE.md geladen werden. Eine Regel kann mit YAML `paths:` Frontmatter pfadgebunden sein, sodass sie nur geladen wird, wenn Claude eine übereinstimmende Datei liest, um den Kontext schlank zu halten, bis er relevant ist.

Weitere Informationen: [Organisieren Sie Regeln mit `.claude/rules/`](/docs/de/memory#organize-rules-with-claude/rules/)

<h2 id="s">
  S
</h2>

<h3 id="sandboxing">
  Sandboxing
</h3>

OS-Level-Dateisystem- und Netzwerkisolation für das Bash-Tool. Befehle werden innerhalb einer Grenze ausgeführt, die Sie im Voraus definieren, sodass Claude frei darin arbeiten kann, ohne Genehmigungsaufforderungen pro Befehl. Sandboxing ist eine separate Schicht von [Permission Rules](#permission-rule).

Weitere Informationen: [Sandboxing](/docs/de/sandboxing)

<h3 id="session">
  Session
</h3>

Eine Konversation, die an Ihr aktuelles Verzeichnis gebunden ist, mit ihrem eigenen unabhängigen [Kontextfenster](#context-window). Sitzungen können mit `claude -c` fortgesetzt, mit `--fork-session` geforkt werden, um den Verlauf unter einer neuen Sitzungs-ID zu bewahren, oder parallel über Terminals ausgeführt werden. Das Ausführen von `/clear` startet eine neue Sitzung; die vorherige bleibt gespeichert und ist über `/resume` verfügbar. Das Transkript jeder Sitzung wird unter `~/.claude/projects/` gespeichert.

Weitere Informationen: [Arbeiten Sie mit Sitzungen](/docs/de/how-claude-code-works#work-with-sessions)

<h3 id="settings-layers">
  Settings Layers
</h3>

Die Hierarchie, aus der Claude Code Konfiguration liest, in Vorrangordnung von höchster zu niedrigster: [verwaltete Richtlinie](#managed-settings), Befehlszeilenargumente, lokale Einstellungen unter `.claude/settings.local.json`, Projekteinstellungen unter `.claude/settings.json`, dann Benutzereinstellungen unter `~/.claude/settings.json`. Arrays werden über Schichten hinweg zusammengeführt; Skalare auf einer höheren Schicht überschreiben niedrigere.

Weitere Informationen: [Einstellungsdateien](/docs/de/settings#settings-files)

<h3 id="skill">
  Skill
</h3>

Eine `SKILL.md`-Datei mit Anweisungen, Wissen oder einem Workflow, den Claude zu seinem Toolkit hinzufügt. Claude lädt einen Skill automatisch, wenn er relevant ist, oder Sie rufen ihn direkt mit `/skill-name` auf. Skills folgen dem Agent Skills Open Standard; Claude Code erweitert ihn mit Invokationskontrolle und Subagent-Ausführung.

Skills sind der empfohlene Nachfolger zu benutzerdefinierten Befehlen. Eine Datei unter `.claude/commands/deploy.md` und eine unter `.claude/skills/deploy/SKILL.md` erstellen beide `/deploy` und funktionieren auf die gleiche Weise; vorhandene Befehlsdateien funktionieren weiterhin.

Weitere Informationen: [Erweitern Sie Claude mit Skills](/docs/de/skills)

<h3 id="subagent">
  Subagent
</h3>

Ein spezialisierter KI-Assistent, der in seinem eigenen Kontextfenster mit einem benutzerdefinierten System-Prompt, spezifischem Tool-Zugriff und unabhängigen Berechtigungen ausgeführt wird. Er arbeitet an einer delegierten Aufgabe und gibt eine Zusammenfassung an die Hauptkonversation zurück. Verwenden Sie Subagenten, um große Explorations aus Ihrem primären Kontext zu halten oder um parallele Forschung auszuführen. Unterschiedlich von [Agent Teams](#agent-teams), bei denen jeder Agent eine vollständig unabhängige Sitzung ist, mit der Sie direkt sprechen können.

Integrierte Subagenten umfassen Explore, Plan und allgemeinen Zweck.

Weitere Informationen: [Erstellen Sie benutzerdefinierte Subagenten](/docs/de/sub-agents)

<h3 id="surface">
  Surface
</h3>

Jeder Ort, an dem Sie auf Claude Code zugreifen: die CLI, VS Code, JetBrains, Desktop oder claude.ai. Alle Surfaces teilen die gleiche Engine, sodass Ihre CLAUDE.md, Einstellungen und Skills auf die gleiche Weise über sie hinweg funktionieren. Slack und die Chrome-Erweiterung sind Integrationen, die sich mit einer Surface verbinden, anstatt Surfaces selbst zu sein.

Weitere Informationen: [Plattformen und Integrationen](/docs/de/platforms)

<h2 id="t">
  T
</h2>

<h3 id="teleport">
  Teleport
</h3>

Ein Befehl, `/teleport`, der eine Cloud Claude Code-Sitzung in Ihr lokales Terminal zieht. Claude ruft den Branch ab, lädt den Konversationsverlauf und setzt die Web-Sitzung fort, wo sie zuletzt war. Die umgekehrte Richtung ist `--cloud`, die eine lokale Aufgabe zum Ausführen im Web sendet.

Weitere Informationen: [Vom Web zum Terminal](/docs/de/claude-code-on-the-web#from-web-to-terminal)

<h3 id="tool">
  Tool
</h3>

Eine Aktion, die Claude durchführen kann: eine Datei lesen, Code bearbeiten, einen Shell-Befehl ausführen, das Web durchsuchen, einen Subagenten spawnen. Tools sind das, was Claude Code agentic macht. Ohne sie kann Claude nur mit Text antworten. Jede Tool-Nutzung gibt ein Ergebnis zurück, das Claudes nächste Entscheidung in der [Agentic Loop](#agentic-loop) informiert.

Weitere Informationen: [Tools, die Claude zur Verfügung stehen](/docs/de/tools-reference)

<h3 id="turn">
  Turn
</h3>

Eine vollständige Antwort von Claude innerhalb einer [Sitzung](#session). Ein Turn beginnt, wenn Sie eine Nachricht senden, und endet, wenn Claude die Antwort beendet, mit einer beliebigen Anzahl von [Tool](#tool)-Aufrufen dazwischen. [Stop Hooks](#hook) werden am Ende jedes Turns ausgelöst. Eine Sitzung besteht aus vielen Turns, und die [Agentic Loop](#agentic-loop) beschreibt, was innerhalb eines Turns passiert.

Weitere Informationen: [Wie Claude Code funktioniert](/docs/de/how-claude-code-works#the-agentic-loop)

<h2 id="v">
  V
</h2>

<h3 id="verification-loop">
  Verification Loop
</h3>

Wie eine Sitzung weiß, dass die Arbeit tatsächlich erledigt ist, anstatt nur plausibel zu sein. Sie geben Claude eine Überprüfung, die er ausführen kann, wie eine Test-Suite, einen Build oder einen Screenshot-Vergleich, und Claude iteriert, bis die Überprüfung besteht, anstatt nach einem Versuch zu stoppen. Eine Verification Loop ist die Voraussetzung für [`/goal`](/docs/de/goal), unbeaufsichtigte Läufe und [dynamische Workflows](/docs/de/workflows): ohne eine ist das Einzige, das entscheidet, dass der Agent fertig ist, der Agent selbst.

Weitere Informationen: [Geben Sie Claude eine Möglichkeit, seine Arbeit zu überprüfen](/docs/de/best-practices#give-claude-a-way-to-verify-its-work)

<h2 id="w">
  W
</h2>

<h3 id="worktree-isolation">
  Worktree Isolation
</h3>

Ein Isolationsmodus, der Claude in einem separaten Git-Worktree unter `.claude/worktrees/` ausführt, aktiviert mit dem `-w`-Flag oder `isolation: worktree` in der Subagent-Konfiguration. Änderungen bleiben auf einem separaten Branch in einem separaten Verzeichnis, sodass parallele Agenten die Dateien des anderen nicht überschreiben.

Weitere Informationen: [Führen Sie parallele Sitzungen mit Git Worktrees aus](/docs/de/worktrees)

***

<h2 id="deprecated-and-renamed-terms">
  Veraltete und umbenannte Begriffe
</h2>

Diese Begriffe erscheinen in älteren Dokumentationen, Blog-Posts und Community-Inhalten. Verwenden Sie den aktuellen Namen, wenn Sie diese Website durchsuchen.

| Alter Begriff   | Jetzt genannt                                 | Notizen                                             |
| --------------- | --------------------------------------------- | --------------------------------------------------- |
| Headless Mode   | [Non-Interactive Mode](#non-interactive-mode) | Gleiches `-p`-Flag, gleiches Verhalten              |
| Custom Commands | [Skills](#skill)                              | `.claude/commands/`-Dateien funktionieren weiterhin |
| Slash Commands  | Commands                                      | „Slash" aus der Produktkopie entfernt               |
