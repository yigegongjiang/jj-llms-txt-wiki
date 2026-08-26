> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent Skills im SDK

> Erweitern Sie Claude mit spezialisierten Fähigkeiten mithilfe von Agent Skills im Claude Agent SDK

<h2 id="overview">
  Übersicht
</h2>

Agent Skills erweitern Claude um spezialisierte Fähigkeiten, die Claude autonom aufruft, wenn relevant. Skills werden als `SKILL.md`-Dateien verpackt, die Anweisungen, Beschreibungen und optionale unterstützende Ressourcen enthalten.

Umfassende Informationen zu Skills, einschließlich Vorteile, Architektur und Authoring-Richtlinien, finden Sie in der [Agent Skills-Übersicht](https://platform.claude.com/docs/de/agents-and-tools/agent-skills/overview).

<h2 id="how-skills-work-with-the-sdk">
  Wie Skills mit dem SDK funktionieren
</h2>

Bei Verwendung des Claude Agent SDK sind Skills:

1. **Als Dateisystem-Artefakte definiert**: Erstellt als `SKILL.md`-Dateien in spezifischen Verzeichnissen (`.claude/skills/`)
2. **Aus dem Dateisystem geladen**: Skills werden aus Dateisystem-Speicherorten geladen, die von `settingSources` (TypeScript) oder `setting_sources` (Python) gesteuert werden
3. **Automatisch erkannt**: Sobald Dateisystem-Einstellungen geladen sind, werden Skill-Metadaten beim Start aus Benutzer- und Projektverzeichnissen erkannt; vollständiger Inhalt wird geladen, wenn ausgelöst
4. **Modell-aufgerufen**: Claude wählt autonom basierend auf dem Kontext, wann sie verwendet werden
5. **Gefiltert über die `skills`-Option**: Erkannte Skills sind standardmäßig aktiviert. Übergeben Sie eine Liste von Skill-Namen, `"all"` oder `[]`, um zu steuern, welche in der Sitzung verfügbar sind

Im Gegensatz zu Subagenten (die programmatisch definiert werden können) müssen Skills als Dateisystem-Artefakte erstellt werden. Das SDK bietet keine programmatische API zum Registrieren von Skills.

<Note>
  Skills werden durch die Dateisystem-Einstellungsquellen erkannt. Mit Standard-`query()`-Optionen lädt das SDK Benutzer- und Projektquellen, sodass Skills in `~/.claude/skills/`, `<cwd>/.claude/skills/` und `.claude/skills/` in jedem übergeordneten Verzeichnis von `<cwd>` bis zur Repository-Root verfügbar sind. Wenn Sie `settingSources` explizit festlegen, schließen Sie `'user'` oder `'project'` ein, um die Skill-Erkennung beizubehalten, oder verwenden Sie die [`plugins`-Option](/docs/de/agent-sdk/plugins), um Skills aus einem bestimmten Pfad zu laden.
</Note>

<h2 id="using-skills-with-the-sdk">
  Verwendung von Skills mit dem SDK
</h2>

Legen Sie die `skills`-Option auf `query()` fest, um zu steuern, welche Skills der Sitzung zur Verfügung stehen. Wenn weggelassen, sind erkannte Skills aktiviert und das Skill-Tool ist verfügbar, was dem CLI-Verhalten entspricht. Übergeben Sie `"all"`, um jeden erkannten Skill zu aktivieren, eine Liste von Skill-Namen, um nur diese zu aktivieren, oder `[]`, um alle zu deaktivieren. Wenn Sie `skills` festlegen, fügt das SDK das Skill-Tool automatisch zu `allowedTools` hinzu. Wenn Sie auch eine explizite `tools`-Liste übergeben, nehmen Sie `"Skill"` in diese Liste auf, damit Claude Skills aufrufen kann.

Nach der Konfiguration erkennt Claude automatisch Skills aus dem Dateisystem und ruft sie auf, wenn sie für die Anfrage des Benutzers relevant sind.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      options = ClaudeAgentOptions(
          cwd="/path/to/project",  # Project with .claude/skills/
          setting_sources=["user", "project"],  # Load Skills from filesystem
          skills="all",  # Enable every discovered Skill
          allowed_tools=["Read", "Write", "Bash"],
      )

      async for message in query(
          prompt="Help me process this PDF document", options=options
      ):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Help me process this PDF document",
    options: {
      cwd: "/path/to/project", // Project with .claude/skills/
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all", // Enable every discovered Skill
      allowedTools: ["Read", "Write", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Um nur bestimmte Skills zu aktivieren, übergeben Sie ihre Namen. Namen entsprechen dem `name`-Feld in `SKILL.md` oder dem Skill-Verzeichnisnamen. Verwenden Sie `plugin:skill` für von Plugins bereitgestellte Skills.

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(skills=["pdf", "docx"])
  ```

  ```typescript TypeScript theme={null}
  const options = { skills: ["pdf", "docx"] };
  ```
</CodeGroup>

Die `skills`-Option ist ein Kontextfilter, kein Sandbox. Nicht aufgelistete Skills sind für das Modell verborgen und werden vom Skill-Tool abgelehnt, aber ihre Dateien bleiben auf der Festplatte und sind über Read und Bash erreichbar.

<h2 id="skill-locations">
  Skill-Speicherorte
</h2>

Skills werden aus Dateisystem-Verzeichnissen basierend auf Ihrer `settingSources`/`setting_sources`-Konfiguration geladen:

* **Projekt-Skills** (`.claude/skills/`): Mit Ihrem Team über Git geteilt - geladen, wenn `setting_sources` `"project"` enthält
* **Benutzer-Skills** (`~/.claude/skills/`): Persönliche Skills über alle Projekte hinweg - geladen, wenn `setting_sources` `"user"` enthält
* **Plugin-Skills**: Mit installierten Claude Code-Plugins gebündelt

<h2 id="creating-skills">
  Erstellen von Skills
</h2>

Skills werden als Verzeichnisse definiert, die eine `SKILL.md`-Datei mit YAML-Frontmatter und Markdown-Inhalt enthalten. Das `description`-Feld bestimmt, wann Claude Ihren Skill aufruft.

**Beispiel-Verzeichnisstruktur**:

```bash theme={null}
.claude/skills/processing-pdfs/
└── SKILL.md
```

Vollständige Anleitung zum Erstellen von Skills, einschließlich SKILL.md-Struktur, mehrdatei-Skills und Beispiele, finden Sie unter:

* [Agent Skills in Claude Code](/docs/de/skills): Vollständige Anleitung mit Beispielen
* [Agent Skills Best Practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices): Authoring-Richtlinien und Namenskonventionen

<h2 id="tool-restrictions">
  Tool-Einschränkungen
</h2>

<Note>
  Das `allowed-tools`-Frontmatter-Feld in SKILL.md wird nur unterstützt, wenn Sie Claude Code CLI direkt verwenden. **Es gilt nicht bei Verwendung von Skills über das SDK**.

  Bei Verwendung des SDK steuern Sie den Tool-Zugriff über die Hauptoption `allowedTools` in Ihrer Query-Konfiguration.
</Note>

Um den Tool-Zugriff für Skills in SDK-Anwendungen zu steuern, verwenden Sie `allowedTools`, um bestimmte Tools vorab zu genehmigen. Ohne einen `canUseTool`-Callback wird alles, was nicht in der Liste enthalten ist, verweigert:

<Note>
  Import-Anweisungen aus dem ersten Beispiel werden in den folgenden Code-Snippets angenommen.
</Note>

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
      allowed_tools=["Read", "Grep", "Glob"],
  )

  async for message in query(prompt="Analyze the codebase structure", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Analyze the codebase structure",
    options: {
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"],
      permissionMode: "dontAsk" // Deny anything not in allowedTools
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h2 id="discovering-available-skills">
  Verfügbare Skills entdecken
</h2>

Um zu sehen, welche Skills in Ihrer SDK-Anwendung verfügbar sind, fragen Sie einfach Claude:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
  )

  async for message in query(prompt="What Skills are available?", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "What Skills are available?",
    options: {
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all"
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Claude listet die verfügbaren Skills basierend auf Ihrem aktuellen Arbeitsverzeichnis und installierten Plugins auf.

<h2 id="testing-skills">
  Testen von Skills
</h2>

Testen Sie Skills, indem Sie Fragen stellen, die ihren Beschreibungen entsprechen:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      cwd="/path/to/project",
      setting_sources=["user", "project"],  # Load Skills from filesystem
      skills="all",
      allowed_tools=["Read", "Bash"],
  )

  async for message in query(prompt="Extract text from invoice.pdf", options=options):
      print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Extract text from invoice.pdf",
    options: {
      cwd: "/path/to/project",
      settingSources: ["user", "project"], // Load Skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Claude ruft automatisch den relevanten Skill auf, wenn die Beschreibung Ihrer Anfrage entspricht.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="skills-not-found">
  Skills nicht gefunden
</h3>

**Überprüfen Sie die settingSources-Konfiguration**: Skills werden durch die `user`- und `project`-Einstellungsquellen erkannt. Wenn Sie `settingSources`/`setting_sources` explizit festlegen und diese Quellen weglassen, werden Skills nicht geladen:

<CodeGroup>
  ```python Python theme={null}
  # Skills not loaded: setting_sources excludes user and project
  options = ClaudeAgentOptions(setting_sources=[], skills="all")

  # Skills loaded: user and project sources included
  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],
      skills="all",
  )
  ```

  ```typescript TypeScript theme={null}
  // Skills not loaded: settingSources excludes user and project
  const options = {
    settingSources: [],
    skills: "all"
  };

  // Skills loaded: user and project sources included
  const options = {
    settingSources: ["user", "project"],
    skills: "all"
  };
  ```
</CodeGroup>

Weitere Details zu `settingSources`/`setting_sources` finden Sie in der [TypeScript SDK-Referenz](/docs/de/agent-sdk/typescript#settingsource) oder [Python SDK-Referenz](/docs/de/agent-sdk/python#settingsource).

**Überprüfen Sie das Arbeitsverzeichnis**: Das SDK lädt Skills aus `.claude/skills/` in der `cwd`-Option und in jedem übergeordneten Verzeichnis bis zur Repository-Root. Stellen Sie sicher, dass `cwd` auf ein Verzeichnis verweist, das `.claude/skills/` enthält oder darunter liegt, innerhalb desselben Repositorys:

<CodeGroup>
  ```python Python theme={null}
  # Ensure your cwd points to the directory containing .claude/skills/
  options = ClaudeAgentOptions(
      cwd="/path/to/project",  # .claude/skills/ here or in a parent directory
      setting_sources=["user", "project"],  # Loads skills from these sources
      skills="all",
  )
  ```

  ```typescript TypeScript theme={null}
  // Ensure your cwd points to the directory containing .claude/skills/
  const options = {
    cwd: "/path/to/project", // .claude/skills/ here or in a parent directory
    settingSources: ["user", "project"], // Loads skills from these sources
    skills: "all"
  };
  ```
</CodeGroup>

Siehe den Abschnitt „Verwendung von Skills mit dem SDK" oben für das vollständige Muster.

**Überprüfen Sie den Dateisystem-Speicherort**:

```bash theme={null}
# Check project Skills
ls .claude/skills/*/SKILL.md

# Check personal Skills
ls ~/.claude/skills/*/SKILL.md
```

<h3 id="skill-not-being-used">
  Skill wird nicht verwendet
</h3>

**Überprüfen Sie die `skills`-Option**: Wenn Sie eine `skills`-Liste übergeben haben, bestätigen Sie, dass der Name des Skills enthalten ist. Das Übergeben von `[]` deaktiviert alle Skills.

**Überprüfen Sie die Beschreibung**: Stellen Sie sicher, dass sie spezifisch ist und relevante Schlüsselwörter enthält. Siehe [Agent Skills Best Practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions) für Anleitung zum Schreiben effektiver Beschreibungen.

<h3 id="additional-troubleshooting">
  Zusätzliche Fehlerbehebung
</h3>

Für allgemeine Skills-Fehlerbehebung (YAML-Syntax, Debugging usw.) siehe den [Claude Code Skills-Fehlerbehebungsabschnitt](/docs/de/skills#troubleshooting).

<h2 id="related-documentation">
  Zugehörige Dokumentation
</h2>

<h3 id="skills-guides">
  Skills-Leitfäden
</h3>

* [Agent Skills in Claude Code](/docs/de/skills): Vollständiger Skills-Leitfaden mit Erstellung, Beispielen und Fehlerbehebung
* [Agent Skills Overview](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview): Konzeptionelle Übersicht, Vorteile und Architektur
* [Agent Skills Best Practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices): Authoring-Richtlinien für effektive Skills
* [Agent Skills Cookbook](https://platform.claude.com/cookbook/skills-notebooks-01-skills-introduction): Beispiel-Skills und Vorlagen

<h3 id="sdk-resources">
  SDK-Ressourcen
</h3>

* [Subagents im SDK](/docs/de/agent-sdk/subagents): Ähnliche dateisystem-basierte Agenten mit programmatischen Optionen
* [Slash Commands im SDK](/docs/de/agent-sdk/slash-commands): Benutzer-aufgerufene Befehle
* [SDK-Übersicht](/docs/de/agent-sdk/overview): Allgemeine SDK-Konzepte
* [TypeScript SDK-Referenz](/docs/de/agent-sdk/typescript): Vollständige API-Dokumentation
* [Python SDK-Referenz](/docs/de/agent-sdk/python): Vollständige API-Dokumentation
