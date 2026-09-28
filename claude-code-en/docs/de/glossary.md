> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Glossar

> Definitionen für Claude Code-Terminologie. Erfahren Sie, was Agentic Loop, Komprimierung, CLAUDE.md, Hooks, Subagenten, MCP und andere Kernkonzepte bedeuten.

Dieses Glossar definiert Claude Code-Terminologie. Jeder Eintrag verlinkt auf die Seite, auf der das Konzept ausführlich behandelt wird. Für Modell-Konzepte wie Tokens, Temperatur und RAG siehe das [Plattform-Glossar](https://platform.claude.com/docs/de/about-claude/glossary). Für Claude Desktop-Begriffe wie Desktop-Erweiterung, MCPB und DXT siehe das [Claude-Hilfezentrum](https://support.claude.com/).

<h2 id="a">
  A
</h2>

<h3 id="agents-md">
  AGENTS.md
</h3>

Eine Markdown-Datei mit Projektanweisungen, die Sie für KI-Coding-Agenten schreiben. Wenn Ihr Repository eine solche Datei hat und keine [CLAUDE.md](#claude-md), liest Claude sie als Ihre Projektanweisungen, ohne dass Sie eine zweite Datei hinzufügen müssen. Sie können die Einstellung **Projektanweisungen** in `/config` ändern, um Claude beide Dateien oder nur `CLAUDE.md` zu lesen. Das direkte Lesen von `AGENTS.md` erfordert Claude Code v2.1.277 oder später. In einigen Sitzungen kann Claude [`AGENTS.md` nicht lesen](/docs/de/memory#when-agents-md-support-is-unavailable), daher [importieren Sie sie stattdessen aus einer `CLAUDE.md`](/docs/de/memory#share-one-file-with-other-coding-tools).

Weitere Informationen: [AGENTS.md](/docs/de/memory#agents-md)

<h3 id="agent-teams">
  Agent Teams
</h3>

Mehrere unabhängige Claude Code-Sitzungen, die von einem Team-Lead koordiniert werden, mit einer gemeinsamen Aufgabenliste und Peer-to-Peer-Messaging. Im Gegensatz zu [Subagenten](#subagent), die innerhalb einer einzelnen Sitzung ausgeführt werden und nur dem übergeordneten Element berichten, hat jedes Teammate sein eigenes Kontextfenster und Sie können direkt mit jedem von ihnen interagieren. Agent Teams sind experimentell und standardmäßig deaktiviert; siehe [Agent Teams aktivieren](/docs/de/agent-teams#enable-agent-teams).

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

Ein [Berechtigungsmodus](#permission-mode), bei dem ein separates Klassifizierungsmodell Aktionen überprüft, anstatt Sie zu fragen, sodass Claude Code die meisten davon ohne Genehmigung ausführt. Claude Code fragt Sie weiterhin vor Aktionen, die Ihren expliziten Ask-Regeln entsprechen. Auf Pro-, Max- und Team-Plänen ist Auto Mode der [integrierte Standard-Berechtigungsmodus](/docs/de/permission-modes#which-mode-a-session-starts-in) für interaktive Terminal- und VS Code-Sitzungen. Der Klassifizierer blockiert Scope-Eskalation, nicht vertrauenswürdige Infrastruktur und [Prompt Injection](#prompt-injection). Tool-Ergebnisse werden aus dem entfernt, was er sieht, sodass bösartiger Inhalt in einer Datei oder Webseite ihn nicht direkt manipulieren kann.

Weitere Informationen: [Aufforderungen mit Auto Mode eliminieren](/docs/de/permission-modes#eliminate-prompts-with-auto-mode)

<h2 id="b">
  B
</h2>

<h3 id="bare-mode">
  Bare Mode
</h3>

Mit `--bare` startet Claude Code ohne das Laden von Hooks, Skills, benutzerdefinierten Befehlen, Subagenten, installierten Plugins, MCP-Servern, Auto Memory oder CLAUDE.md, mit Ausnahme von Skills in einem Verzeichnis, das Sie mit `--add-dir` übergeben. Empfohlen für CI und Skript-Aufrufe, bei denen Sie auf jeder Maschine das gleiche Ergebnis benötigen.

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

Sie können CLAUDE.md im Projektbereich in `./CLAUDE.md` oder `./.claude/CLAUDE.md`, im Benutzerbereich in `~/.claude/CLAUDE.md` oder als [verwaltete Richtlinie](#managed-settings) für Ihre Organisation platzieren. Alle gefundenen Dateien werden in den Kontext verkettet, anstatt sich gegenseitig zu überschreiben, geordnet vom breitesten Bereich zum spezifischsten. Claude Code kann auch die [AGENTS.md](#agents-md)-Dateien eines Projekts laden, eigenständig oder zusammen mit CLAUDE.md.

Weitere Informationen: [CLAUDE.md-Dateien](/docs/de/memory#claude-md-files)

<h3 id="cloud-session">
  Cloud-Sitzung
</h3>

Eine Claude Code-Sitzung, die weiterläuft, nachdem Sie Ihren Laptop schließen, da sie auf Cloud-Infrastruktur statt auf Ihrem Computer läuft: standardmäßig von Anthropic verwaltet oder eine [selbst gehostete Umgebung](/docs/de/self-hosted-environments), die Ihre Organisation betreibt. Sie starten eine von claude.ai/code, der Claude Mobile App, der Desktop-App mit **Cloud** ausgewählt, `claude --cloud` oder einer [Routine](/docs/de/routines). Eine Sitzung in Ihrem Terminal, IDE oder der Desktop-App mit **Local** ausgewählt ist eine lokale Sitzung; um von einem anderen Gerät auf eine lokale Sitzung zuzugreifen, verwenden Sie [Remote Control](#remote-control).

Weitere Informationen: [Claude Code in der Cloud verwenden](/docs/de/claude-code-on-the-web)

<h3 id="command">
  Command
</h3>

Eine wiederverwendbare Anweisung, die Sie durch Eingabe von `/name` in der Aufforderung aufrufen. Integrierte Befehle wie `/clear`, `/model` und `/compact` steuern die Sitzung. Sie können Ihre eigenen Befehle als Dateien in `.claude/commands/` definieren oder sie aus einem [Plugin](#plugin) installieren. [Skills](#skill) sind die empfohlene Methode zum Verpacken von mehrstufigen Befehlen.

Zwei weitere Verwendungen des Wortes sind nicht verwandt: `claude` CLI-Unterbefehle wie `claude mcp add`, aufgelistet in der [CLI-Referenz](/docs/de/cli-reference#cli-commands), und das `command`-Feld eines stdio [MCP-Servers](#mcp-server)-Eintrags, das die ausführbare Datei angibt, die Claude Code startet, um den Server zu starten.

Weitere Informationen: [Commands](/docs/de/commands) · [Skills](/docs/de/skills)

<h3 id="compaction">
  Compaction
</h3>

Automatische Zusammenfassung Ihrer Konversation, wenn sich das [Kontextfenster](#context-window) seinem Limit nähert. Ältere Tool-Ausgaben werden zuerst gelöscht, dann wird die Konversation zusammengefasst. Projekt-Root CLAUDE.md und Auto Memory überleben die Komprimierung und werden von der Festplatte neu geladen; Anweisungen, die nur in der Konversation gegeben werden, können verloren gehen. Führen Sie `/compact` aus, um manuell auszulösen, optional mit einem Fokus wie `/compact focus on the API changes`.

Weitere Informationen: [Was Komprimierung überlebt](/docs/de/context-window#what-survives-compaction) · [Wenn der Kontext voll wird](/docs/de/how-claude-code-works#when-context-fills-up)

<h3 id="connector">
  Connector
</h3>

Ein [MCP-Server](#mcp-server), der zu Ihrem claude.ai-Konto hinzugefügt wird, anstatt in Claude Code konfiguriert zu werden. Wenn Sie sich bei Claude Code mit diesem Konto anmelden, erscheinen Ihre Connectors in `/mcp` neben den Servern, die Sie lokal hinzugefügt haben. Organisationen können auch Connectors bereitstellen und Pro-Tool-Kontrollen auf ihnen festlegen.

Weitere Informationen: [MCP-Server von claude.ai verwenden](/docs/de/mcp#use-mcp-servers-from-claude-ai)

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

Eine Einstellung, die adaptives Reasoning steuert, wodurch das Modell bei jedem Schritt entscheiden kann, ob und wie viel es denken soll. Höherer Aufwand bedeutet mehr Thinking-Tokens und tieferes Reasoning; niedrigerer Aufwand ist schneller und günstiger. Effort wird auf Fable-Modellen, auf Opus 4.6 und später sowie auf Sonnet 4.6 und später unterstützt.

Weitere Informationen: [Effort Level anpassen](/docs/de/model-config#adjust-effort-level)

<h3 id="extended-thinking">
  Extended Thinking
</h3>

Sichtbares schrittweises Reasoning, das das Modell vor der Antwort durchführt. Sie können es mit dem [Effort Level](#effort-level) anpassen oder Thinking-Tokens mit `MAX_THINKING_TOKENS` auf Modellen mit einem festen Thinking-Budget begrenzen. Thinking erscheint in grauem kursivem Text im Terminal.

Weitere Informationen: [Extended Thinking verwenden](/docs/de/model-config#extended-thinking)

<h2 id="f">
  F
</h2>

<h3 id="frontmatter">
  Frontmatter
</h3>

Ein Block von YAML-Einstellungen ganz oben in einer Markdown-Datei, zwischen einer öffnenden `---`-Zeile und einer schließenden `---`-Zeile. Skills, Subagenten, Ausgabestile und Regeln lesen ihre Konfiguration aus dem Frontmatter, wie die `description` eines Skills oder die `tools` eines Subagenten, und behandeln alles nach der schließenden `---` als die Anweisungen. Die öffnende `---` muss die erste Zeile der Datei sein. Jeder Dateityp akzeptiert seinen eigenen Satz von Feldern.

Weitere Informationen: [Skill-Frontmatter](/docs/de/skills#frontmatter-reference), [Subagent-Frontmatter](/docs/de/sub-agents#supported-frontmatter-fields), [Ausgabestil-Frontmatter](/docs/de/output-styles#frontmatter), [Regel-Frontmatter](/docs/de/memory#rules-frontmatter-reference)

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

Weitere Informationen: [Server-verwaltete Einstellungen](/docs/de/server-managed-settings) · [Einstellungsdateien](/docs/de/settings#where-settings-live)

<h3 id="mcp-model-context-protocol">
  MCP (Model Context Protocol)
</h3>

Ein offener Standard für die Verbindung von KI-Tools mit externen Datenquellen und Diensten. MCP-Server geben Claude neue Tools für Slack, Jira, Datenbanken, Browser und Hunderte anderer Integrationen. Sie verbinden Server über `/mcp` oder durch Hinzufügen zu `.mcp.json`. Für das Protokoll selbst siehe das [Plattform-Glossar](https://platform.claude.com/docs/de/about-claude/glossary#mcp-model-context-protocol).

Weitere Informationen: [Model Context Protocol](/docs/de/mcp)

<h3 id="mcp-server">
  MCP server
</h3>

Ein Programm, das Claude Tools, Prompts oder Ressourcen über [MCP](#mcp-model-context-protocol) bereitstellt. Sie fügen Server mit `claude mcp add` hinzu, in `.mcp.json`, über ein [Plugin](#plugin) oder als claude.ai [Connector](#connector). Ein lokaler stdio-Server wird als Prozess ausgeführt, den Claude Code aus den Feldern `command` und `args` seiner Konfiguration startet, die nichts mit den [Befehlen](#command) zu tun haben, die Sie an der Eingabeaufforderung eingeben.

Weitere Informationen: [Model Context Protocol](/docs/de/mcp)

<h3 id="mcp-tool-search">
  MCP Tool Search
</h3>

Ein Kontextsparmechanismus, der MCP-Tool-Schemas bis zur Notwendigkeit aufschiebt. Nur Tool-Namen und Server-Anweisungen werden beim Start geladen; Claude ruft das vollständige Schema bei Bedarf ab, wenn es sich entscheidet, ein bestimmtes Tool zu verwenden. Dies verhindert, dass untätige MCP-Server viel Kontext verbrauchen.

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

Eine Konfiguration, die die Anweisungen ändert, die Claude Code Claude gibt, um Antwortverhalten, Ton oder Format festzulegen. Im Gegensatz zu [CLAUDE.md](#claude-md), das Projektkontext neben Claudes Standard-Anweisungen hinzufügt, kann ein benutzerdefinierter Output Style die Standard-Softwareentwicklungs-Anweisungen ersetzen.

Weitere Informationen: [Output Styles](/docs/de/output-styles)

<h2 id="p">
  P
</h2>

<h3 id="permission-mode">
  Genehmigungsmodus
</h3>

Das grundlegende Genehmigungsverhalten für die Sitzung. Wechseln Sie mit `Shift+Tab` in der CLI oder verwenden Sie den Moduswahlschalter in VS Code, Desktop und claude.ai. Verfügbare Modi sind `default`, `acceptEdits`, `plan`, `auto`, `dontAsk` und `bypassPermissions`.

Der Modus `default` wird in der CLI, in den VS Code- und JetBrains-Erweiterungen sowie in der Desktop-App als „Manual" bezeichnet, und Claude Code akzeptiert `manual` als Alias für den Wert.

Weitere Informationen: [Wählen Sie einen Genehmigungsmodus](/docs/de/permission-modes)

<h3 id="permission-rule">
  Genehmigungsregel
</h3>

Ein Einstellungseintrag, der eine Werkzeuginvokation basierend auf dem Werkzeugnamen und dem Argumentmuster zulässt, nachfragt oder ablehnt. Regeln werden in der Reihenfolge deny→ask→allow ausgewertet, wobei die erste Übereinstimmung gewinnt. Genehmigungsregeln sind differenzierte Kontrollen, die auf dem breiteren [Genehmigungsmodus](#permission-mode) aufgelagert sind.

Weitere Informationen: [Konfigurieren Sie Berechtigungen](/docs/de/permissions)

<h3 id="plan-mode">
  Plan Mode
</h3>

Ein [Genehmigungsmodus](#permission-mode), bei dem Claude Änderungen recherchiert und vorschlägt, ohne Ihre Quelldateien zu bearbeiten. Es kann lesen, suchen und Explorationskommandos ausführen, dann einen Plan zur Genehmigung präsentieren, bevor etwas berührt wird. Geben Sie Plan Mode mit `/plan` ein oder drücken Sie `Shift+Tab`.

Weitere Informationen: [Analysieren Sie vor dem Bearbeiten mit Plan Mode](/docs/de/permission-modes#analyze-before-you-edit-with-plan-mode)

<h3 id="plugin">
  Plugin
</h3>

Ein Paket aus Skills, Hooks, Subagenten und MCP-Servern, das als eine einzelne installierbare Einheit verpackt ist. Plugin-Skills werden als `plugin-name:skill-name` namensraumbezogen, sodass mehrere Plugins koexistieren können. Verteilen Sie Plugins über Teams über einen [Marketplace](/docs/de/plugins/overview).

Weitere Informationen: [Plugins](/docs/de/plugins/overview)

<h3 id="project-trust">
  Projektvertrauen
</h3>

Ein Dialog, der ein Verzeichnis akzeptiert, bevor Claude Code seine Konfiguration lädt. Die Akzeptanz wird pro Projektverzeichnis gespeichert, außer in Ihrem Home-Verzeichnis, wo das Vertrauen nur für die aktuelle Sitzung gilt und die Eingabeaufforderung bei jedem Start erneut angezeigt wird. Bis Sie ein Verzeichnis vertrauen, hält Claude Code einige der Inhalte zurück, die sein Repository bereitstellt, wie z. B. Projekterlaubnisregeln und Marketplaces aus `.claude/settings.json`. [Was wird ausgeführt, bevor Sie einem Ordner vertrauen](/docs/de/permissions#what-runs-before-you-trust-a-folder) listet jede Art von Inhalt auf, einschließlich dessen, was eine `-p`-Sitzung ohne Dialog ausführt.

Weitere Informationen: [Das `.claude`-Verzeichnis](/docs/de/claude-directory)

<h3 id="prompt-injection">
  Prompt-Injection
</h3>

Feindselige Anweisungen, die in einer Datei, Webseite oder einem Werkzeugergebnis eingebettet sind und versuchen, Claude zu Aktionen umzuleiten, die Sie nie angefordert haben. Die Abwehrmechanismen von Claude Code umfassen das Berechtigungssystem, die Erkennung von Befehlsinjektion und die Vertrauensüberprüfung. Der [Auto-Modus](#auto-mode) fügt eine serverseitige Sonde hinzu, die Werkzeugergebnisse auf verdächtige Inhalte scannt, und einen Klassifizierer, der Aktionen mit entfernten Werkzeugergebnissen überprüft, sodass injizierter Text sie nicht direkt manipulieren kann.

Weitere Informationen: [Schützen Sie sich vor Prompt-Injection](/docs/de/security#protect-against-prompt-injection)

<h2 id="r">
  R
</h2>

<h3 id="remote-control">
  Remote Control
</h3>

Eine Möglichkeit, eine lokale Claude Code-Sitzung von Ihrem Telefon oder Browser über claude.ai fortzusetzen. Ihr Code bleibt auf Ihrem Computer; nur die Benutzeroberfläche ist remote. Unterschiedlich von einer [Cloud-Sitzung](/docs/de/claude-code-on-the-web), die in einer Cloud-Sandbox ausgeführt wird.

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

Die Hierarchie, aus der Claude Code Konfiguration liest, in Vorrangordnung von höchster zu niedrigster: [verwaltete Richtlinie](#managed-settings), Befehlszeilenargumente, lokale Einstellungen unter `.claude/settings.local.json`, Projekteinstellungen unter `.claude/settings.json`, dann Benutzereinstellungen unter `~/.claude/settings.json`. Arrays werden über Schichten hinweg zusammengeführt; Skalare auf einer höheren Schicht überschreiben niedrigere. Siehe [Settings precedence](/docs/de/settings#settings-precedence).

Weitere Informationen: [Einstellungsdateien](/docs/de/settings#where-settings-live)

<h3 id="skill">
  Skill
</h3>

Eine `SKILL.md`-Datei mit Anweisungen, Wissen oder einem Workflow, den Claude zu seinem Toolkit hinzufügt. Claude lädt einen Skill automatisch, wenn er relevant ist, oder Sie rufen ihn direkt mit `/skill-name` auf. Skills folgen dem Agent Skills Open Standard; Claude Code erweitert ihn mit Invokationskontrolle und Subagent-Ausführung.

Skills sind der empfohlene Nachfolger zu benutzerdefinierten Befehlen. Eine Datei unter `.claude/commands/deploy.md` und eine unter `.claude/skills/deploy/SKILL.md` erstellen beide `/deploy` und funktionieren auf die gleiche Weise; vorhandene Befehlsdateien funktionieren weiterhin.

Weitere Informationen: [Erweitern Sie Claude mit Skills](/docs/de/skills)

<h3 id="subagent">
  Subagent
</h3>

Ein spezialisierter KI-Assistent, der in seinem eigenen Kontextfenster mit einem benutzerdefinierten System-Prompt, spezifischem Tool-Zugriff und unabhängigen Berechtigungen ausgeführt wird. Er arbeitet an einer delegierten Aufgabe und gibt eine Zusammenfassung an die Hauptkonversation zurück. Verwenden Sie Subagenten, um große Explorations aus Ihrem primären Kontext zu halten oder um parallele Forschung auszuführen. Ein Subagent bleibt in der Sitzung, die ihn erzeugt hat. Um Erkenntnisse zwischen separaten Sitzungen, die Sie selbst ausführen, zu übergeben, verwenden Sie [Cross-Session-Messaging](/docs/de/cross-session-messaging).

Integrierte Subagenten umfassen Explore, Plan und allgemeinen Zweck.

Weitere Informationen: [Erstellen Sie benutzerdefinierte Subagenten](/docs/de/sub-agents)

<h3 id="surface">
  Surface
</h3>

Jeder Ort, an dem Sie auf Claude Code zugreifen: die CLI, VS Code, JetBrains, Desktop oder claude.ai. Alle Surfaces teilen die gleiche Engine. Sitzungen auf Ihrem Computer lesen Ihre lokale CLAUDE.md, Einstellungen und Skills; [Cloud-Sitzungen](/docs/de/cloud-environments#what-carries-over-from-your-setup) starten von einem frischen Klon Ihres Repositorys und lesen nicht `~/.claude/` auf Ihrem Computer. Slack und die Chrome-Erweiterung sind Integrationen, die sich mit einer Surface verbinden, anstatt Surfaces selbst zu sein.

Weitere Informationen: [Plattformen und Integrationen](/docs/de/platforms)

<h2 id="t">
  T
</h2>

<h3 id="teleport">
  Teleport
</h3>

Ein Befehl, `/teleport`, der eine Cloud Claude Code-Sitzung in Ihr lokales Terminal zieht. Claude ruft den Branch ab, lädt den Konversationsverlauf und setzt die Cloud-Sitzung fort, wo sie zuletzt war. Die umgekehrte Richtung ist `--cloud`, die eine lokale Aufgabe zum Ausführen in der Cloud sendet.

Weitere Informationen: [Vom Cloud zum Terminal](/docs/de/claude-code-on-the-web#from-cloud-to-terminal)

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

| Alter Begriff                                                     | Jetzt genannt                                 | Notizen                                                                            |
| ----------------------------------------------------------------- | --------------------------------------------- | ---------------------------------------------------------------------------------- |
| Headless Mode                                                     | [Non-Interactive Mode](#non-interactive-mode) | Gleiches `-p`-Flag, gleiches Verhalten                                             |
| Web-Sitzung; „Claude Code im Web" als Name für jede Cloud-Sitzung | [Cloud-Sitzung](#cloud-session)               | „Claude Code im Web" benennt jetzt nur die Browser-Oberfläche unter claude.ai/code |
| Custom Commands                                                   | [Skills](#skill)                              | `.claude/commands/`-Dateien funktionieren weiterhin                                |
| Slash Commands                                                    | Commands                                      | „Slash" aus der Produktkopie entfernt                                              |
