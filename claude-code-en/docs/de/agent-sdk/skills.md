> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent Skills erweitern

> Steuern Sie, welche Skills Claude in Claude Agent SDK-Sitzungen aufrufen kann, versenden Sie Befehle nach Name und erstellen Sie Skills, die Ihre Sitzungen entdecken

Agent Skills erweitern Claude um spezialisierte Fähigkeiten, die Claude aufruft, wenn relevant. Skills werden als `SKILL.md`-Dateien verpackt, die Anweisungen, Beschreibungen und optionale unterstützende Ressourcen enthalten. Diese Seite behandelt auch [Befehle in Agent SDK-Sitzungen](#commands-in-agent-sdk-sessions).

Umfassende Informationen zu Skills, einschließlich Vorteile, Architektur und Authoring-Richtlinien, finden Sie in der [Agent Skills-Übersicht](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview).

<h2 id="how-skills-work-with-the-agent-sdk">
  Wie Skills mit dem Agent SDK funktionieren
</h2>

Bei Verwendung des Claude Agent SDK sind Skills:

* **Als Dateisystem-Artefakte definiert**: Sie erstellen jeden Skill als `SKILL.md`-Datei in seinem eigenen Verzeichnis, z. B. `.claude/skills/<name>/SKILL.md`
* **Aus dem Dateisystem geladen**: Das SDK lädt Skills aus Dateisystem-Speicherorten, die von `settingSources` (TypeScript) oder `setting_sources` (Python) gesteuert werden
* **Automatisch erkannt**: Sobald Dateisystem-Einstellungen geladen sind, erkennt das SDK Skill-Metadaten beim Start aus Benutzer- und Projektverzeichnissen und lädt den vollständigen Inhalt, wenn Claude den Skill aufruft
* **Modell-aufgerufen**: Claude wählt autonom basierend auf dem Kontext, wann sie verwendet werden
* **Benutzer-aufgerufen**: Sie versenden einen Skill direkt, indem Sie `/<name>` in einer Eingabeaufforderung senden. Siehe [Befehle in Agent SDK-Sitzungen](#commands-in-agent-sdk-sessions)
* **Über die `skills`-Option begrenzt**: Erkannte Skills sind standardmäßig aktiviert. Übergeben Sie eine Liste von Skill-Namen, `"all"` oder `[]`, um zu steuern, welche Skills Claude aufrufen kann

Im Gegensatz zu Subagenten, die Sie in der [`agents`-Option](/docs/de/agent-sdk/subagents#programmatic-definition-recommended) definieren können, erstellen Sie Skills als Dateien auf der Festplatte. Das SDK bietet keine programmatische API zum Registrieren von Skills.

<Note>
  Skills werden durch die Dateisystem-Einstellungsquellen erkannt. Mit Standard-`query()`-Optionen lädt das SDK Benutzer- und Projektquellen, sodass Skills in `~/.claude/skills/`, `<cwd>/.claude/skills/` und `.claude/skills/` in jedem übergeordneten Verzeichnis von `<cwd>` bis zur Repository-Root verfügbar sind. Die Projektquelle deckt auch `<dir>/.claude/skills/` in jedem Verzeichnis ab, das Sie über `additionalDirectories` (TypeScript) oder `add_dirs` (Python) übergeben, da das SDK diese Verzeichnisse an Claude Code als [`--add-dir`](/docs/de/skills#skills-from-additional-directories) übergibt. Wenn Sie `settingSources` explizit festlegen, schließen Sie `'project'` ein, um Projekt- und hinzugefügte Verzeichnis-Skills beizubehalten, und `'user'`, um Ihre persönlichen Skills beizubehalten, oder verwenden Sie die [`plugins`-Option](/docs/de/agent-sdk/plugins), um Skills aus einem bestimmten Pfad zu laden.
</Note>

<h2 id="use-skills-with-the-agent-sdk">
  Skills mit dem Agent SDK verwenden
</h2>

Legen Sie die `skills`-Option auf `query()` fest, um zu steuern, welche Skills Claude in der Sitzung aufrufen kann. Wenn weggelassen, sind erkannte Skills aktiviert und das Skill-Tool ist verfügbar, was dem CLI-Verhalten entspricht. Übergeben Sie `"all"`, um Claude jeden erkannten Skill aufrufen zu lassen, eine Liste von Skill-Namen, um nur diese zu erlauben, oder `[]`, um Claude keinen aufrufen zu lassen.

Um Claude beispielsweise nur zwei benannte Skills aufrufen zu lassen:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(skills=["pdf", "docx"])
  ```

  ```typescript TypeScript theme={null}
  const options = { skills: ["pdf", "docx"] };
  ```
</CodeGroup>

<h3 id="set-up-skills-in-a-session">
  Skills in einer Sitzung einrichten
</h3>

Wenn Sie `skills` festlegen, fügt das SDK das Skill-Tool automatisch zu `allowedTools` hinzu. Wenn Sie auch eine explizite `tools`-Liste übergeben, schließen Sie `"Skill"` in diese Liste ein, damit Claude Skills aufrufen kann.

Nach der Konfiguration erkennt Claude automatisch Skills aus dem Dateisystem und ruft sie auf, wenn sie für die Anfrage des Benutzers relevant sind.

Das folgende Beispiel aktiviert jeden erkannten Skill in einer Sitzung und genehmigt die Tools vorab, die Skills häufig benötigen. Das Beispiel setzt `cwd` auf das aktuelle Arbeitsverzeichnis des Prozesses, daher führen Sie es in einem Projekt aus, das ein `.claude/skills/`-Verzeichnis im aktuellen Verzeichnis oder einem übergeordneten Verzeichnis bis zur Repository-Root hat:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import os

  from claude_agent_sdk import query, ClaudeAgentOptions


  async def main():
      options = ClaudeAgentOptions(
          cwd=os.getcwd(),  # .claude/skills/ here or in a parent directory
          setting_sources=["user", "project"],  # Load skills from filesystem
          skills="all",  # Let Claude invoke every discovered skill
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
      cwd: process.cwd(), // .claude/skills/ here or in a parent directory
      settingSources: ["user", "project"], // Load skills from filesystem
      skills: "all", // Let Claude invoke every discovered skill
      allowedTools: ["Read", "Write", "Bash"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

<h3 id="confirm-skills-loaded">
  Bestätigen Sie, dass Skills geladen wurden
</h3>

Nahe am Anfang des Streams gibt das SDK eine Systemmeldung mit dem Subtyp `init` aus. Überprüfen Sie sein `skills`-Array, um zu bestätigen, dass Ihre Skills geladen wurden, bevor Claude mit der Arbeit beginnt. Das Array enthält die benutzer-aufgerufenen Skills, die Sie definiert haben, zusammen mit [gebündelten Skills, die in Claude Code enthalten sind](/docs/de/skills#bundled-skills).

Das Array listet nur benutzer-aufgerufene Skills auf. Ein Skill mit [`user-invocable: false`](/docs/de/skills#control-who-invokes-a-skill) in seinem Frontmatter wird geladen und bleibt für Claude verfügbar, erscheint aber nicht im Array. Das Array listet die gleichen Skills auf, unabhängig davon, ob sie sich in Ihrer `skills`-Liste befinden oder nicht.

<h3 id="allow-only-specific-skills">
  Nur bestimmte Skills erlauben
</h3>

Um Claude nur bestimmte Skills aufrufen zu lassen, übergeben Sie ihre Namen in der `skills`-Liste. Namen entsprechen dem `name`-Feld in `SKILL.md` oder dem Skill-Verzeichnisnamen. Verwenden Sie `plugin:skill` für von Plugins bereitgestellte Skills.

Die Liste akzeptiert nur exakte Skill-Namen. Wenn ein Eintrag nicht als exakter Name funktionieren kann, lehnt `query()` die Liste ab, bevor die Sitzung beginnt. Siehe [Fehler bei ungültigem Skill-Namen](#invalid-skill-name-error) für die Namenregeln und den Fehler, den jedes SDK auslöst.

Das Modell sieht nicht aufgelistete Skills nicht und das Skill-Tool lehnt sie ab, während ihre Dateien auf der Festplatte bleiben und über Read und Bash erreichbar bleiben. Das Einschränken der Liste schränkt nicht [Versand nach Name](#dispatch-commands-by-name) ein.

Um Claude jeden erkannten Skill aufrufen zu lassen, übergeben Sie `skills: "all"` anstelle eines Platzhalters.

<h2 id="commands-in-agent-sdk-sessions">
  Befehle in Agent SDK-Sitzungen
</h2>

Dieser Abschnitt ist die Befehlsdokumentation des SDK. Ein Befehl ist alles, was Sie ausführen, indem Sie `/<name>` in einer Eingabeaufforderung senden. Einträge auf der Befehlsoberfläche unterscheiden sich darin, was sie unterstützt:

* **Integrierte Befehle**: Führen Logik aus, die in den Claude Code-Prozess codiert ist, den das SDK ausführt, zum Beispiel `/compact`
* **Gebündelte Skills**: Eingabeaufforderungs-Artefakte, die mit Claude Code geliefert werden, zum Beispiel `/code-review`
* **Ihre Skills**: Eingabeaufforderungs-Artefakte, die Sie erstellen, jeweils ein Verzeichnis mit einer `SKILL.md`-Datei. Der Name einer vom Benutzer aufzurufenden Skill wird automatisch zur Oberfläche hinzugefügt, sodass das Versenden Ihres eigenen `/security-check` und das Ausführen eines integrierten Befehls auf die gleiche Weise funktionieren
* **Benutzerdefinierte Befehlsdateien**: eine ältere Artefaktform mit dem gleichen Verhalten, flache Markdown-Dateien in `.claude/commands/`, deren Dateinamen zu Befehlsnamen werden. Skills sind ihr empfohlener Nachfolger

Standardmäßig können sowohl Sie als auch Claude jede Skill aufrufen. Sie können jeden Pfad durch die [Frontmatter](/docs/de/skills#control-who-invokes-a-skill) der Skill einschränken. Für eine Definition der beiden Begriffe siehe die Einträge [Befehl](/docs/de/glossary#command) und [Skill](/docs/de/glossary#skill) im Glossar. Siehe [Befehle in Claude Code](/docs/de/commands) für alle integrierten Befehle und [Claude mit Skills erweitern](/docs/de/skills) für den vollständigen Leitfaden zu beiden Artefaktformen.

<h3 id="discover-available-commands">
  Verfügbare Befehle entdecken
</h3>

Sie können Befehle versenden, die ohne ein interaktives Terminal durch das SDK funktionieren. Die `system/init`-Nachricht listet die in Ihrer Sitzung verfügbaren in ihrem `slash_commands`-Feld auf. Befehle, die ein interaktives Terminal benötigen, wie `/theme` und `/terminal-setup`, erscheinen nicht in der Liste. Greifen Sie auf das Feld zu, wenn Ihre Sitzung startet:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Hello Claude",
    options: { maxTurns: 1 }
  })) {
    if (message.type === "system" && message.subtype === "init") {
      console.log("Available commands:", message.slash_commands);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage


  async def main():
      async for message in query(prompt="Hello Claude", options=ClaudeAgentOptions(max_turns=1)):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print("Available commands:", message.data["slash_commands"])


  asyncio.run(main())
  ```
</CodeGroup>

Die gedruckte Liste mischt integrierte Befehle, gebündelte Skills, Ihre vom Benutzer aufzurufenden Skills und `.claude/commands/`-Dateien:

```text theme={null}
Available commands: ["clear", "compact", "context", "usage", "code-review", "verify", "security-check", ...]
```

Eine Skill mit [`user-invocable: false`](/docs/de/skills#control-who-invokes-a-skill) in ihrer Frontmatter erscheint nicht in dieser Liste oder im `skills`-Array von [Bestätigen Sie, dass Skills geladen sind](#confirm-skills-loaded). Sitzungen, die [MCP-Server](/docs/de/agent-sdk/mcp) konfigurieren, können auch [MCP-Eingabeaufforderungen als Befehle](/docs/de/mcp#use-mcp-prompts-as-commands) verfügbar machen.

<h3 id="dispatch-commands-by-name">
  Befehle nach Name versenden
</h3>

Senden Sie einen Befehl, indem Sie ihn in Ihre Eingabeaufforderungszeichenfolge einbeziehen, auf die gleiche Weise wie Sie normalen Text senden. Das Versenden hängt nicht von der `skills`-Option ab. Das Senden von `/<name>` führt eine vom Benutzer aufzurufende Skill aus, auch wenn Ihre `skills`-Liste sie auslässt. Befehle, die auf Gesprächsverlauf wirken, wie `/compact`, benötigen vorherige Nachrichten, um damit zu arbeiten.

Ein `/<name>`, das weder einem Befehl in der Sitzung noch einem integrierten Claude Code-Befehl entspricht, führt nicht zu einem Fehler bei der Abfrage. Claude Code sendet die Eingabeaufforderung als gewöhnliche Nachricht an Claude, mit einem Hinweis, dass der Befehl nicht ausgeführt wurde, sodass die Abfrage einen Modelldurchlauf verbraucht und Claudes Antwort zurückgibt. Vor v2.1.274 gab ein `/<name>`, das nichts entsprach, `Unknown command: /<name>` als Ergebnis ohne einen Modelldurchlauf zurück.

Ein `/<name>`, das einem integrierten Claude Code-Befehl entspricht, der in der Sitzung nicht verfügbar ist, wie `/theme`, gibt `/theme isn't available in this environment.` als Ergebnis ohne einen Modelldurchlauf zurück.

<Note>
  Ein Befehl kann das `maxTurns` / `max_turns`-Limit wie jede andere Eingabeaufforderung treffen und die Abfrage mit einem Fehlerergebnis statt `success` beenden. Für den Fehlerergebnis-Vertrag siehe [Behandeln Sie das Ergebnis](/docs/de/agent-sdk/agent-loop#handle-the-result). Wenn Ihr Befehl das Limit treffen könnte, wickeln Sie die Schleife in einen `try`/`catch` in TypeScript oder `try`/`except` in Python ein, wie in [Einzelne Nachrichteneingabe](/docs/de/agent-sdk/streaming-vs-single-mode#single-message-input) gezeigt, oder setzen Sie `maxTurns` hoch genug, damit die Arbeit abgeschlossen wird.
</Note>

<h3 id="compact-history-with-/compact">
  Verlauf mit `/compact` komprimieren
</h3>

Der `/compact`-Befehl reduziert die Größe Ihres Gesprächsverlaufs, indem er ältere Nachrichten zusammenfasst und dabei wichtigen Kontext bewahrt. Die Komprimierung benötigt ein bestehendes Gespräch mit genug vorherigen Nachrichten zum Zusammenfassen. Dieses Beispiel hat zuerst ein Gespräch, dann komprimiert es und liest die `compact_boundary`-Systemnachricht, die das Ergebnis meldet:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Compaction needs existing history, so have a conversation first
  try {
    for await (const message of query({
      prompt: "Explain what this project does",
      options: { maxTurns: 2 }
    })) {
      if (message.type === "result" && message.subtype === "success") {
        console.log(message.result);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so the follow-up query below still runs.
    console.error(`Session ended with an error: ${error}`);
  }

  // Compact the same conversation
  for await (const message of query({
    prompt: "/compact",
    options: { continue: true, maxTurns: 1 }
  })) {
    if (message.type === "system" && message.subtype === "compact_boundary") {
      console.log("Compaction completed");
      console.log("Pre-compaction tokens:", message.compact_metadata.pre_tokens);
      console.log("Trigger:", message.compact_metadata.trigger);
      // Example output:
      // Compaction completed
      // Pre-compaction tokens: 1842
      // Trigger: manual
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage, SystemMessage


  async def main():
      # Compaction needs existing history, so have a conversation first
      try:
          async for message in query(
              prompt="Explain what this project does",
              options=ClaudeAgentOptions(max_turns=2),
          ):
              if isinstance(message, ResultMessage) and message.subtype == "success":
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so the follow-up query below still runs.
          print(f"Session ended with an error: {error}")

      # Compact the same conversation
      async for message in query(
          prompt="/compact",
          options=ClaudeAgentOptions(continue_conversation=True, max_turns=1),
      ):
          if isinstance(message, SystemMessage) and message.subtype == "compact_boundary":
              print("Compaction completed")
              print("Pre-compaction tokens:", message.data["compact_metadata"]["pre_tokens"])
              print("Trigger:", message.data["compact_metadata"]["trigger"])
              # Example output:
              # Compaction completed
              # Pre-compaction tokens: 1842
              # Trigger: manual


  asyncio.run(main())
  ```
</CodeGroup>

<Note>
  Eine `compact_boundary`-Nachricht kommt nur an, wenn die Komprimierung ausgeführt wurde. Wenn es nichts zu zusammenfassen gibt, meldet `/compact` stattdessen den Grund, ohne einen Fehler auszulösen. Die Ausführung endet immer noch mit einem `success`-Ergebnis und ohne `compact_boundary`-Nachricht, und der Ergebnistext trägt den Grund, zum Beispiel `Not enough messages to compact.` nach einem einzelnen kurzen Austausch. Ein frischer One-Shot-`query()`-Aufruf startet mit leerem Kontext, daher verwenden Sie dieses Muster in einer Sitzung mit vorherigen Durchläufen, zum Beispiel im [Streaming-Eingabemodus](/docs/de/agent-sdk/streaming-vs-single-mode) oder beim Fortsetzen einer Sitzung.
</Note>

<h3 id="reset-context-with-/clear">
  Kontext mit `/clear` zurücksetzen
</h3>

Der `/clear`-Befehl setzt das Gespräch auf einen leeren Kontext zurück, sodass nachfolgende Eingabeaufforderungen ohne vorherigen Gesprächsverlauf starten. Das vorherige Gespräch bleibt auf der Festplatte. Sie können zu diesem Gespräch zurückkehren, indem Sie seine Sitzungs-ID an die [`resume`-Option](/docs/de/agent-sdk/sessions#resume-by-id) übergeben.

`/clear` ist nützlich im [Streaming-Eingabemodus](/docs/de/agent-sdk/streaming-vs-single-mode), wo Sie mehrere Eingabeaufforderungen über eine einzelne Verbindung senden. Für One-Shot-`query()`-Aufrufe startet jeder Aufruf bereits mit leerem Kontext, daher hat das Senden von `/clear` keine praktische Auswirkung. Starten Sie stattdessen eine neue `query()`.

<h2 id="create-skills">
  Skills erstellen
</h2>

Erstellen Sie jeden Skill als Verzeichnis mit einer `SKILL.md`-Datei mit YAML-Frontmatter und Markdown-Inhalt. Das `description`-Feld bestimmt, wann Claude Ihren Skill aufruft.

**Beispiel-Verzeichnisstruktur**:

```text theme={null}
.claude/skills/security-check/
└── SKILL.md
```

<h3 id="choose-a-discovery-level">
  Wählen Sie eine Erkennungsebene
</h3>

Speichern Sie Skills auf einer der beiden häufigsten [Erkennungsebenen](/docs/de/skills#where-skills-live):

* **Projekt-Skills**: `.claude/skills/`, nur im aktuellen Projekt verfügbar
* **Persönliche Skills**: `~/.claude/skills/`, über alle Ihre Projekte hinweg verfügbar

Wenn Sie vorhandene benutzerdefinierte Befehlsdateien in `.claude/commands/` haben, funktionieren sie weiterhin. Eine Befehlsdatei unter `.claude/commands/deploy.md` erstellt `/deploy` und funktioniert auf die gleiche Weise wie ein Skill unter `.claude/skills/deploy/SKILL.md`. Wenn eine Befehlsdatei und ein Skill einen Namen teilen, siehe [Lösen Sie Skills auf, die einen Namen teilen](/docs/de/skills#resolve-skills-that-share-a-name) für welcher ausgeführt wird. Das SDK lädt `.claude/commands/` und `~/.claude/commands/`-Dateien aus den gleichen zwei Bereichen wie Skills. Siehe [Claude mit Skills erweitern](/docs/de/skills) für den vollständigen Leitfaden zu beiden Artefaktformen.

<h3 id="create-and-dispatch-your-first-skill">
  Erstellen und versenden Sie Ihren ersten Skill
</h3>

Um den vollständigen Ablauf zu sehen, erstellen Sie `.claude/skills/security-check/SKILL.md`:

```markdown theme={null}
---
name: security-check
description: Run a security vulnerability scan
---

Analyze the codebase for security vulnerabilities including:
- SQL injection risks
- XSS vulnerabilities
- Exposed credentials
- Insecure configurations
```

Sobald die Datei vorhanden ist, ist der Skill über das SDK verfügbar. Claude ruft ihn auf, wenn eine Anfrage seiner Beschreibung entspricht, und Sie können ihn direkt versenden:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "/security-check",
    options: { maxTurns: 10 }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      async for message in query(
          prompt="/security-check", options=ClaudeAgentOptions(max_turns=10)
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

Ein erfolgreicher Lauf endet mit einem `success`-Ergebnis, dessen Text die Scan-Ergebnisse trägt. Gegen eine kleine Express-App mit eingestreuten Problemen beginnt der Ergebnis-Text:

```text theme={null}
**Security scan of `app.js` — 4 findings (most severe first):**

1. **SQL Injection** (line 8) — `req.query.name` is concatenated directly into the SQL string. Trivially exploitable (`' OR '1'='1`, `'; DROP TABLE users;--`). **Fix:** use parameterized queries, e.g. `db.query("SELECT * FROM users WHERE name = ?", [req.query.name], cb)`.
...
```

Der Name des Skills erscheint auch im `slash_commands`-Array der Init-Meldung.

<Note>
  Claude Code enthält gebündelte `code-review`- und `verify`-Skills. Wenn Sie eine `.claude/commands/`-Datei nach einem von ihnen benennen, z. B. `.claude/commands/code-review.md`, schattet die Datei den gebündelten Skill und `slash_commands` listet den Namen einmal auf.
</Note>

<h2 id="pre-approve-tools-for-skills">
  Tools für Skills vorab genehmigen
</h2>

<Note>
  Für Projekt- und persönliche Skills wendet Claude Code das [`allowed-tools`](/docs/de/skills#pre-approve-tools-for-a-skill)-Frontmatter-Feld in SDK-Sitzungen an. Sie können Tools für diese Skills auch über die `allowedTools`-Option (`allowed_tools` in Python) in Ihrer Abfragekonfiguration vorab genehmigen. Skills [synchronisiert von claude.ai](/docs/de/skills#how-claude-code-handles-the-frontmatter-of-a-synced-skill) folgen ihren eigenen Frontmatter-Regeln.
</Note>

Skills laufen mit den Tools der Sitzung. Das folgende Beispiel genehmigt `Read`, `Grep` und `Glob` mit `allowedTools` (`allowed_tools` in Python) vorab, sodass Claude Dateien inspizieren kann, während der [security-check-Skill](#create-and-dispatch-your-first-skill) ausgeführt wird, ohne auf Genehmigung zu warten:

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import query, ClaudeAgentOptions

  options = ClaudeAgentOptions(
      setting_sources=["user", "project"],  # Load skills from filesystem
      skills="all",
      allowed_tools=["Read", "Grep", "Glob"],
  )


  async def main():
      async for message in query(prompt="Check this project for security issues", options=options):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Check this project for security issues",
    options: {
      settingSources: ["user", "project"], // Load skills from filesystem
      skills: "all",
      allowedTools: ["Read", "Grep", "Glob"]
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Im Stream erscheint der Skill-Aufruf als Skill-Tool-Verwendung, gefolgt von Read-Aufrufen auf den Projektdateien. Der Lauf endet mit einem `success`-Ergebnis, dessen Text die Ergebnisse trägt.

Die Liste genehmigt die benannten Tools vorab, anstatt die anderen einzuschränken. Für den vollständigen Berechtigungsfluss, einschließlich Berechtigungsmodi und des `canUseTool`-Callbacks, siehe [Berechtigungen](/docs/de/agent-sdk/permissions).

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="skills-not-found">
  Skills nicht gefunden
</h3>

**Überprüfen Sie die settingSources-Konfiguration**: Das SDK erkennt Skills durch die `user`- und `project`-Einstellungsquellen. Wenn Sie `settingSources`/`setting_sources` explizit festlegen und diese Quellen auslassen, lädt das SDK Skills nicht:

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
  const optionsWithoutSkills = {
    settingSources: [],
    skills: "all"
  };

  // Skills loaded: user and project sources included
  const optionsWithSkills = {
    settingSources: ["user", "project"],
    skills: "all"
  };
  ```
</CodeGroup>

Welche Skill-Verzeichnisse jede Quelle lädt, siehe die [Dateisystem-Quellen-Tabelle](/docs/de/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources). Weitere Details zu `settingSources`/`setting_sources` finden Sie in der [TypeScript SDK-Referenz](/docs/de/agent-sdk/typescript#settingsource) oder [Python SDK-Referenz](/docs/de/agent-sdk/python#settingsource).

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

Siehe [Skills mit dem Agent SDK verwenden](#use-skills-with-the-agent-sdk) für das vollständige Muster.

**Überprüfen Sie den Dateisystem-Speicherort**:

```bash theme={null}
# Check project skills
ls .claude/skills/*/SKILL.md

# Check personal skills
ls ~/.claude/skills/*/SKILL.md
```

<h3 id="skill-not-being-used">
  Skill wird nicht verwendet
</h3>

**Überprüfen Sie die `skills`-Option**: Wenn Sie eine `skills`-Liste übergeben haben, bestätigen Sie, dass der Name des Skills enthalten ist. Wenn Claude versucht, einen nicht aufgelisteten Skill aufzurufen, gibt das Skill-Tool `Skill <name> is not in this session's skills allowlist` zurück. Fügen Sie den Namen zu Ihrer Liste hinzu, oder versenden Sie den Skill direkt, indem Sie `/<name>` in einer Eingabeaufforderung senden, was ohne Auflistung funktioniert.

**Überprüfen Sie die Beschreibung**: Stellen Sie sicher, dass sie spezifisch ist und relevante Schlüsselwörter enthält. Siehe [Agent Skills Best Practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices#writing-effective-descriptions) für Anleitung zum Schreiben effektiver Beschreibungen.

<h3 id="invalid-skill-name-error">
  Fehler bei ungültigem Skill-Namen
</h3>

Wenn ein Name in Ihrer `skills`-Liste nicht als exakter Skill-Name funktionieren kann, lehnt `query()` die Liste ab, bevor der Claude Code-Prozess gestartet wird. Namen, die die Ablehnung auslösen, umfassen:

* Ein leerer Name
* Ein Name mit Klammern, Kommas oder Steuerzeichen
* Ein Name mit Leerzeichen aufgefüllt
* Eine Platzhalterform wie ein bloßes `*` oder ein `:*`-Suffix

Jedes SDK zeigt die Ablehnung unterschiedlich an:

<Tabs>
  <Tab title="TypeScript">
    Das TypeScript SDK wirft einen `Error`, der die Regel angibt, die der Eintrag brach. Zum Beispiel wirft `skills: ["docs:*"]`:

    ```text theme={null}
    Invalid skill name "docs:*": wildcard-suffix names are not allowed; list each skill by its exact name.
    ```

    Ein leerer Name meldet `Skill names must be non-empty strings.`

    Vor TypeScript Agent SDK 0.3.221 führte das SDK diese Überprüfung nicht durch.
  </Tab>

  <Tab title="Python">
    Das Python SDK wirft `ValueError`, das die Regel angibt, die der Eintrag brach. Zum Beispiel wirft `skills=["docs:*"]`:

    ```text theme={null}
    ValueError: Invalid skill name 'docs:*': wildcard-suffix names are not allowed; list each skill by its exact name.
    ```

    Ein leerer Name meldet `Skill names must be non-empty strings`.

    Vor Python Agent SDK 0.2.129 führte das SDK diese Überprüfung nicht durch.
  </Tab>
</Tabs>

<h3 id="additional-troubleshooting">
  Zusätzliche Fehlerbehebung
</h3>

Für allgemeine Skills-Fehlerbehebung, wie YAML-Syntax-Fehler und Debugging, siehe den [Claude Code Skills-Fehlerbehebungsabschnitt](/docs/de/skills#troubleshooting).

<h2 id="next-steps">
  Nächste Schritte
</h2>

Der [Claude Code Skills-Leitfaden](/docs/de/skills) behandelt das Authoring ausführlich. Seine Anleitung gilt für SDK-Sitzungen. Beginnen Sie mit diesen Abschnitten:

* [Frontmatter-Referenz](/docs/de/skills#frontmatter-reference): jedes unterstützte Feld
* [Übergeben Sie Argumente an Skills](/docs/de/skills#pass-arguments-to-skills): `$ARGUMENTS`, `$0`, `$1` und Skill-Stapelung. Die [vollständige Substitutions-Tabelle](/docs/de/skills#available-string-substitutions) fügt benannte Argumente und die `${CLAUDE_*}`-Variablen hinzu
* [Injizieren Sie dynamischen Kontext](/docs/de/skills#inject-dynamic-context): `` !`command` ``-Zeilen, die ausgeführt werden, bevor Claude den Skill-Inhalt sieht
* [Wählen Sie, wo Skills geladen werden](/docs/de/skills#where-skills-live): jeder Skill-Speicherort, Plugin-Namensraum und welcher Skill ausgeführt wird, wenn zwei einen Namen teilen

<h2 id="related-resources">
  Zugehörige Ressourcen
</h2>

* [Befehle in Claude Code](/docs/de/commands): die vollständige Befehlsoberfläche, einschließlich jedes integrierten
* [Agent Skills-Übersicht](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview): konzeptionelle Übersicht, Vorteile und Architektur
* [Agent Skills Best Practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices): Authoring-Richtlinien für effektive Skills
* [Agent Skills Cookbook](https://platform.claude.com/cookbook/skills-notebooks-01-skills-introduction): Beispiel-Skills und Vorlagen
* [Subagenten im SDK](/docs/de/agent-sdk/subagents): ähnliche dateisystem-basierte Agenten mit programmatischen Optionen
* [SDK-Übersicht](/docs/de/agent-sdk/overview): allgemeine SDK-Konzepte
* [TypeScript SDK-Referenz](/docs/de/agent-sdk/typescript): vollständige API-Dokumentation
* [Python SDK-Referenz](/docs/de/agent-sdk/python): vollständige API-Dokumentation
