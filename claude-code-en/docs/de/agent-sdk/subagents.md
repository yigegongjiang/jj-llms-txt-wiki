> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Subagenten im SDK

> Definieren und rufen Sie Subagenten auf, um den Kontext zu isolieren, Aufgaben parallel auszuführen und spezialisierte Anweisungen in Ihren Claude Agent SDK-Anwendungen anzuwenden.

Subagenten sind separate Agent-Instanzen, die Ihr Haupt-Agent spawnen kann, um fokussierte Teilaufgaben zu bewältigen.
Verwenden Sie Subagenten, um den Kontext zu isolieren, mehrere Analysen parallel auszuführen und spezialisierte Anweisungen anzuwenden, ohne den Prompt des Haupt-Agenten zu überlasten.

Dieser Leitfaden erklärt, wie Sie Subagenten im SDK mit dem Parameter `agents` definieren und verwenden.

<h2 id="overview">
  Übersicht
</h2>

Sie können Subagenten auf drei Arten erstellen:

* **Programmatisch**: Verwenden Sie den Parameter `agents` in Ihren `query()`-Optionen. Siehe die [TypeScript](/docs/de/agent-sdk/typescript#agentdefinition)- und [Python](/docs/de/agent-sdk/python#agentdefinition)-Referenzen
* **Dateisystem-basiert**: Definieren Sie Agenten als Markdown-Dateien in `.claude/agents/`-Verzeichnissen. Siehe [Subagenten als Dateien definieren](/docs/de/sub-agents)
* **Integriert allgemein einsetzbar**: Claude kann den integrierten `general-purpose`-Subagenten jederzeit über das Agent-Tool aufrufen, ohne dass Sie etwas definieren müssen

Dieser Leitfaden konzentriert sich auf den programmatischen Ansatz, der für SDK-Anwendungen empfohlen wird.

Wenn Sie Subagenten definieren, bestimmt Claude basierend auf dem Feld `description` jedes Subagenten, ob dieser aufgerufen werden soll. Schreiben Sie klare Beschreibungen, die erklären, wann der Subagent verwendet werden sollte, und Claude wird automatisch geeignete Aufgaben delegieren. Sie können einen Subagenten auch explizit nach Name in Ihrem Prompt anfordern, zum Beispiel „Verwenden Sie den Code-Reviewer-Agent, um...".

<h2 id="benefits-of-using-subagents">
  Vorteile der Verwendung von Subagenten
</h2>

<h3 id="context-isolation">
  Kontextisolation
</h3>

Jeder Subagent läuft in seiner eigenen frischen Konversation. Zwischentool-Aufrufe und Ergebnisse bleiben innerhalb des Subagenten; nur seine abschließende Nachricht wird an den übergeordneten Agent zurückgegeben. Siehe [Was Subagenten erben](#what-subagents-inherit) für genau das, was sich im Kontext des Subagenten befindet.

**Beispiel:** Ein `research-assistant`-Subagent kann Dutzende von Dateien durchsuchen, ohne dass dieser Inhalt in der Hauptkonversation angesammelt wird. Der übergeordnete Agent erhält eine prägnante Zusammenfassung, nicht jede Datei, die der Subagent gelesen hat.

<h3 id="parallelization">
  Parallelisierung
</h3>

Mehrere Subagenten können gleichzeitig ausgeführt werden, sodass unabhängige Teilaufgaben in der Zeit des langsamsten statt in der Summe aller abgeschlossen werden.

**Beispiel:** Während einer Code-Überprüfung können Sie die Subagenten `style-checker`, `security-scanner` und `test-coverage` gleichzeitig ausführen, anstatt sie nacheinander auszuführen.

<h3 id="specialized-instructions-and-knowledge">
  Spezialisierte Anweisungen und Wissen
</h3>

Jeder Subagent kann maßgeschneiderte System-Prompts mit spezifischer Expertise, Best Practices und Einschränkungen haben.

**Beispiel:** Ein `database-migration`-Subagent kann detailliertes Wissen über SQL-Best-Practices, Rollback-Strategien und Datenintegritätsprüfungen haben, die in den Anweisungen des Haupt-Agenten unnötiger Lärm wären.

<h3 id="tool-restrictions">
  Tool-Einschränkungen
</h3>

Subagenten können auf bestimmte Tools beschränkt werden, was das Risiko unbeabsichtigter Aktionen verringert.

**Beispiel:** Ein `doc-reviewer`-Subagent könnte nur Zugriff auf Read- und Grep-Tools haben, um sicherzustellen, dass er Ihre Dokumentationsdateien analysieren, aber niemals versehentlich ändern kann.

<h2 id="create-subagents">
  Erstellen von Subagenten
</h2>

<h3 id="programmatic-definition-recommended">
  Programmatische Definition (empfohlen)
</h3>

Definieren Sie Subagenten direkt in Ihrem Code mit dem Parameter `agents`. Claude ruft Subagenten über das `Agent`-Tool auf, daher müssen Sie `Agent` in `allowedTools` einschließen, um Subagenten-Aufrufe automatisch zu genehmigen, ohne eine Berechtigungsaufforderung anzuzeigen.

Die meisten Beispiele auf dieser Seite geben nur das Endergebnis aus. Um zu bestätigen, dass Claude an einen Subagenten delegiert hat, anstatt direkt zu antworten, siehe [Erkennung von Subagenten-Aufrufen](#detect-subagent-invocation).

Dieses Beispiel erstellt zwei Subagenten: einen Code-Reviewer mit Nur-Lese-Zugriff und einen Test-Runner, der Befehle ausführen kann.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Review the authentication module for security issues",
          options=ClaudeAgentOptions(
              # Auto-approve these tools, including Agent for subagent invocation
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  "code-reviewer": AgentDefinition(
                      # description tells Claude when to use this subagent
                      description="Expert code review specialist. Use for quality, security, and maintainability reviews.",
                      # prompt defines the subagent's behavior and expertise
                      prompt="""You are a code review specialist with expertise in security, performance, and best practices.

  When reviewing code:
  - Identify security vulnerabilities
  - Check for performance issues
  - Verify adherence to coding standards
  - Suggest specific improvements

  Be thorough but concise in your feedback.""",
                      # tools restricts what the subagent can do (read-only here)
                      tools=["Read", "Grep", "Glob"],
                      # model overrides the default model for this subagent
                      model="sonnet",
                  ),
                  "test-runner": AgentDefinition(
                      description="Runs and analyzes test suites. Use for test execution and coverage analysis.",
                      prompt="""You are a test execution specialist. Run tests and provide clear analysis of results.

  Focus on:
  - Running test commands
  - Analyzing test output
  - Identifying failing tests
  - Suggesting fixes for failures""",
                      # Bash access lets this subagent run test commands
                      tools=["Bash", "Read", "Grep"],
                  ),
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Review the authentication module for security issues",
    options: {
      // Auto-approve these tools, including Agent for subagent invocation
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        "code-reviewer": {
          // description tells Claude when to use this subagent
          description:
            "Expert code review specialist. Use for quality, security, and maintainability reviews.",
          // prompt defines the subagent's behavior and expertise
          prompt: `You are a code review specialist with expertise in security, performance, and best practices.

  When reviewing code:
  - Identify security vulnerabilities
  - Check for performance issues
  - Verify adherence to coding standards
  - Suggest specific improvements

  Be thorough but concise in your feedback.`,
          // tools restricts what the subagent can do (read-only here)
          tools: ["Read", "Grep", "Glob"],
          // model overrides the default model for this subagent
          model: "sonnet"
        },
        "test-runner": {
          description:
            "Runs and analyzes test suites. Use for test execution and coverage analysis.",
          prompt: `You are a test execution specialist. Run tests and provide clear analysis of results.

  Focus on:
  - Running test commands
  - Analyzing test output
  - Identifying failing tests
  - Suggesting fixes for failures`,
          // Bash access lets this subagent run test commands
          tools: ["Bash", "Read", "Grep"]
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h3 id="agentdefinition-configuration">
  AgentDefinition-Konfiguration
</h3>

| Feld              | Typ                                                         | Erforderlich | Beschreibung                                                                                                                                                                                                                                                         |
| :---------------- | :---------------------------------------------------------- | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`     | `string`                                                    | Ja           | Natürlichsprachige Beschreibung, wann dieser Agent verwendet werden soll                                                                                                                                                                                             |
| `prompt`          | `string`                                                    | Ja           | Der System-Prompt des Agenten, der seine Rolle und sein Verhalten definiert                                                                                                                                                                                          |
| `tools`           | `string[]`                                                  | Nein         | Array von zulässigen Tool-Namen. Falls weggelassen, erbt alle Tools                                                                                                                                                                                                  |
| `disallowedTools` | `string[]`                                                  | Nein         | Array von Tool-Namen, die aus dem Tool-Set des Agenten entfernt werden sollen. MCP-Server-Level-Muster werden ebenfalls akzeptiert: `mcp__server` oder `mcp__server__*` entfernt jedes Tool von diesem Server, und `mcp__*` entfernt jedes MCP-Tool von jedem Server |
| `model`           | `string`                                                    | Nein         | Modell-Override für diesen Agent. Akzeptiert einen Alias wie `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'` oder eine vollständige Modell-ID. Standardmäßig Haupt-Modell, falls weggelassen                                                                 |
| `skills`          | `string[]`                                                  | Nein         | Liste von Skill-Namen, die beim Start in den Kontext des Agenten vorgeladen werden sollen. Nicht aufgelistete Skills bleiben über das Skill-Tool aufrufbar                                                                                                           |
| `memory`          | `'user' \| 'project' \| 'local'`                            | Nein         | Speicherquelle für diesen Agent                                                                                                                                                                                                                                      |
| `mcpServers`      | `(string \| object)[]`                                      | Nein         | MCP-Server, die diesem Agent zur Verfügung stehen, nach Name oder Inline-Konfiguration                                                                                                                                                                               |
| `initialPrompt`   | `string`                                                    | Nein         | Wird automatisch als erster Benutzer-Turn eingereicht, wenn dieser Agent als Haupt-Thread-Agent ausgeführt wird. Wird ignoriert, wenn der Agent als Subagent aufgerufen wird                                                                                         |
| `maxTurns`        | `number`                                                    | Nein         | Maximale Anzahl von Agent-Turns, bevor der Agent stoppt                                                                                                                                                                                                              |
| `background`      | `boolean`                                                   | Nein         | Führen Sie diesen Agent als nicht-blockierende Hintergrund-Aufgabe aus, wenn er aufgerufen wird                                                                                                                                                                      |
| `effort`          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max' \| number` | Nein         | Reasoning-Effort-Level für diesen Agent                                                                                                                                                                                                                              |
| `permissionMode`  | `PermissionMode`                                            | Nein         | Berechtigungsmodus für die Tool-Ausführung innerhalb dieses Agenten                                                                                                                                                                                                  |

Im Python SDK verwenden Feldnamen mit mehreren Wörtern wie `disallowedTools` und `mcpServers` die camelCase-Schreibweise, um das Wire-Format zu entsprechen, anstatt der Python-Konvention snake\_case zu folgen. Siehe die [`AgentDefinition`-Referenz](/docs/de/agent-sdk/python#agentdefinition) für Details.

Zwei Subagenten-Verhaltensweisen haben sich in Claude Code v2.1.198 geändert:

* Subagenten werden standardmäßig im Hintergrund ausgeführt. Ein Agent-Tool-Aufruf, der die [`run_in_background`](/docs/de/agent-sdk/typescript)-Eingabe auslässt, startet einen Hintergrund-Subagenten, und Claude setzt `run_in_background: false`, wenn es das Ergebnis benötigt, bevor es fortfährt. Vor v2.1.198 führte das Auslassen von `run_in_background` den Subagenten synchron aus. Setzen Sie das Feld `background` auf `true`, um die Hintergrund-Ausführung für einen bestimmten Agent zu erzwingen, unabhängig davon, was Claude anfordert.
* Ein Subagent erbt die Extended-Thinking-Konfiguration der Hauptsitzung. In früheren Versionen ist Extended Thinking in Subagenten deaktiviert, unabhängig von der Einstellung der Hauptsitzung.

<Note>
  Ab Claude Code v2.1.172 können Subagenten ihre eigenen Subagenten spawnen. Ein Subagent fünf Ebenen unter dem Haupt-Agent kann keine weiteren Subagenten spawnen, unabhängig davon, ob er im Vordergrund oder Hintergrund ausgeführt wird. Um zu verhindern, dass ein Subagent andere spawnt, lassen Sie `Agent` aus seinem `tools`-Array weg oder fügen Sie es zu `disallowedTools` hinzu. Siehe [verschachtelte Subagenten](/docs/de/sub-agents#spawn-nested-subagents) für die vollständigen Tiefenregeln.
</Note>

<h3 id="filesystem-based-definition-alternative">
  Dateisystem-basierte Definition (Alternative)
</h3>

Sie können Subagenten auch als Markdown-Dateien in `.claude/agents/`-Verzeichnissen definieren. Siehe die [Claude Code Subagenten-Dokumentation](/docs/de/sub-agents) für Details zu diesem Ansatz. Programmatisch definierte Agenten haben Vorrang vor dateisystem-basierten Agenten mit demselben Namen.

<Note>
  Auch ohne benutzerdefinierte Subagenten zu definieren, kann Claude den integrierten `general-purpose`-Subagenten spawnen. Dies ist nützlich, um Recherche- oder Explorationsaufgaben zu delegieren, ohne spezialisierte Agenten zu erstellen. Fügen Sie `Agent` in `allowedTools` ein, damit diese Aufrufe automatisch genehmigt werden, ohne eine Berechtigungsaufforderung anzuzeigen.
</Note>

<h2 id="what-subagents-inherit">
  Was Subagenten erben
</h2>

Das Kontextfenster eines Subagenten startet frisch, ohne übergeordnete Konversation, ist aber nicht leer. Der einzige Inhalt, den Sie vom übergeordneten Agent zum Subagenten übergeben, ist der Prompt-String des Agent-Tools, daher fügen Sie alle Dateipfade, Fehlermeldungen oder Entscheidungen, die der Subagent benötigt, direkt in diesen Prompt ein.

Ein Subagent, der das [`SendMessage`](/docs/de/tools-reference)-Tool hat, startet mit einer Liste der anderen benannten Agenten, die in der Sitzung ausgeführt werden, sodass er weiß, welche Namen er anschreiben kann. Claude Code fügt die Liste automatisch beim ersten Durchgang des Subagenten hinzu. Ein [Fork](/docs/de/sub-agents#fork-the-current-conversation) erhält die Liste nicht, da er stattdessen die übergeordnete Konversation erbt. Die Liste erfordert Claude Code v2.1.206 oder später.

| Der Subagent erhält                                                                                                                     | Der Subagent erhält nicht                                                              |
| :-------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------- |
| Seinen eigenen System-Prompt (`AgentDefinition.prompt`) und den Prompt des Agent-Tools                                                  | Die Konversationshistorie oder Tool-Ergebnisse des übergeordneten Agenten              |
| Projekt CLAUDE.md (geladen über [`settingSources`](/docs/de/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)) | Vorgeladener Skill-Inhalt, es sei denn, er ist in `AgentDefinition.skills` aufgelistet |
| Tool-Definitionen (geerbt vom übergeordneten Agent oder die Teilmenge in `tools`)                                                       | Der System-Prompt des übergeordneten Agenten                                           |

<Note>
  Der übergeordnete Agent erhält die abschließende Nachricht des Subagenten wörtlich als Agent-Tool-Ergebnis, kann sie aber in seiner eigenen Antwort zusammenfassen. Um die Ausgabe des Subagenten wörtlich in der benutzerorientierten Antwort zu bewahren, fügen Sie eine Anweisung dazu in den Prompt oder die `systemPrompt`-Option ein, die Sie an den Haupt-`query()`-Aufruf übergeben.
</Note>

Ein API-Fehler, der den Subagenten vorzeitig beendet, wie z. B. ein Ratenlimit, wird niemals als sein Ergebnis bereitgestellt. Wenn ein Ratenlimit, eine Überlastung oder ein Serverfehler einen Vordergrund-Subagenten unterbricht, der bereits Textausgabe produziert hat, gibt das Agent-Tool diese Teilausgabe mit einem Hinweis zurück, dass der Subagent nicht fertig wurde. Ein Subagent, der nichts produziert hat oder dessen einzige Ausgabe Tool-Aufrufe ohne Text waren, schlägt mit einer Fehlermeldung fehl, `Agent terminated early due to an API error`, gefolgt von der Fehlerdetail. Siehe [API-Fehler in Subagenten](/docs/de/sub-agents#api-errors-in-subagents) für das Vordergrund- und Hintergrundverhalten.

Diese Teilausgabe-Behandlung erfordert Claude Code v2.1.199 oder später. In v2.1.199 hinterließ ein Ratenlimit, eine Überlastung oder ein Serverfehler die Form „nur Tool-Aufrufe" mit einem leeren Teilergebnis, das nur den Abbruchhinweis enthielt.

<h2 id="invoke-subagents">
  Aufrufen von Subagenten
</h2>

<h3 id="automatic-invocation">
  Automatischer Aufruf
</h3>

Claude entscheidet automatisch, wann Subagenten basierend auf der Aufgabe und der `description` jedes Subagenten aufgerufen werden sollen. Wenn Sie beispielsweise einen `performance-optimizer`-Subagenten mit der Beschreibung „Performance-Optimierungsspezialist für Query-Tuning" definieren, wird Claude ihn aufrufen, wenn Ihr Prompt die Optimierung von Queries erwähnt.

Schreiben Sie klare, spezifische Beschreibungen, damit Claude Aufgaben dem richtigen Subagenten zuordnen kann.

<h3 id="explicit-invocation">
  Expliziter Aufruf
</h3>

Um sicherzustellen, dass Claude einen bestimmten Subagenten verwendet, erwähnen Sie ihn nach Name in Ihrem Prompt:

```text theme={null}
"Use the code-reviewer agent to check the authentication module"
```

Dies umgeht das automatische Matching und ruft den benannten Subagenten direkt auf.

<h3 id="dynamic-agent-configuration">
  Dynamische Agent-Konfiguration
</h3>

Sie können Agent-Definitionen dynamisch basierend auf Laufzeitbedingungen erstellen. Dieses Beispiel erstellt einen Security-Reviewer mit verschiedenen Strenge-Leveln und verwendet ein leistungsfähigeres Modell für strenge Überprüfungen.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  # Factory function that returns an AgentDefinition
  # This pattern lets you customize agents based on runtime conditions
  def create_security_agent(security_level: str) -> AgentDefinition:
      is_strict = security_level == "strict"
      return AgentDefinition(
          description="Security code reviewer",
          # Customize the prompt based on strictness level
          prompt=f"You are a {'strict' if is_strict else 'balanced'} security reviewer...",
          tools=["Read", "Grep", "Glob"],
          # Key insight: use a more capable model for high-stakes reviews
          model="opus" if is_strict else "sonnet",
      )


  async def main():
      # The agent is created at query time, so each request can use different settings
      async for message in query(
          prompt="Review this PR for security issues",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  # Call the factory with your desired configuration
                  "security-reviewer": create_security_agent("strict")
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type AgentDefinition } from "@anthropic-ai/claude-agent-sdk";

  // Factory function that returns an AgentDefinition
  // This pattern lets you customize agents based on runtime conditions
  function createSecurityAgent(securityLevel: "basic" | "strict"): AgentDefinition {
    const isStrict = securityLevel === "strict";
    return {
      description: "Security code reviewer",
      // Customize the prompt based on strictness level
      prompt: `You are a ${isStrict ? "strict" : "balanced"} security reviewer...`,
      tools: ["Read", "Grep", "Glob"],
      // Key insight: use a more capable model for high-stakes reviews
      model: isStrict ? "opus" : "sonnet"
    };
  }

  // The agent is created at query time, so each request can use different settings
  for await (const message of query({
    prompt: "Review this PR for security issues",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        // Call the factory with your desired configuration
        "security-reviewer": createSecurityAgent("strict")
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h2 id="detect-subagent-invocation">
  Erkennen von Subagenten-Aufrufen
</h2>

Claude ruft Subagenten über das Agent-Tool auf. Um zu erkennen, wann ein Subagent aufgerufen wird, suchen Sie nach `tool_use`-Blöcken, bei denen `name` `"Agent"` ist. Nachrichten aus dem Kontext eines Subagenten enthalten ein Feld `parent_tool_use_id`.

<Note>
  Der Tool-Name wurde in Claude Code v2.1.63 von `"Task"` zu `"Agent"` umbenannt. Aktuelle SDK-Releases geben `"Agent"` in `tool_use`-Blöcken aus, verwenden aber immer noch `"Task"` in der `system:init`-Tools-Liste und in `result.permission_denials[].tool_name`. Das Überprüfen beider Werte in `block.name` gewährleistet Kompatibilität über SDK-Versionen hinweg.
</Note>

Die Nachrichtenstruktur unterscheidet sich zwischen SDKs. In Python werden Inhaltsblöcke direkt über `message.content` aufgerufen. In TypeScript umhüllt `SDKAssistantMessage` die Claude API-Nachricht, daher wird auf den Inhalt über `message.message.content` zugegriffen.

Dieses Beispiel durchläuft gestreamte Nachrichten und protokolliert, wenn ein Subagent aufgerufen wird und wenn nachfolgende Nachrichten aus dem Ausführungskontext dieses Subagenten stammen.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolUseBlock


  async def main():
      async for message in query(
          prompt="Use the code-reviewer agent to review this codebase",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Glob", "Grep", "Agent"],
              agents={
                  "code-reviewer": AgentDefinition(
                      description="Expert code reviewer.",
                      prompt="Analyze code quality and suggest improvements.",
                      tools=["Read", "Glob", "Grep"],
                  )
              },
          ),
      ):
          # Check for subagent invocation. Match both names: older SDK
          # versions emitted "Task", current versions emit "Agent".
          if hasattr(message, "content") and message.content:
              for block in message.content:
                  if isinstance(block, ToolUseBlock) and block.name in (
                      "Task",
                      "Agent",
                  ):
                      print(f"Subagent invoked: {block.input.get('subagent_type')}")

          # Check if this message is from within a subagent's context
          if hasattr(message, "parent_tool_use_id") and message.parent_tool_use_id:
              print("  (running inside subagent)")

          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Use the code-reviewer agent to review this codebase",
    options: {
      allowedTools: ["Read", "Glob", "Grep", "Agent"],
      agents: {
        "code-reviewer": {
          description: "Expert code reviewer.",
          prompt: "Analyze code quality and suggest improvements.",
          tools: ["Read", "Glob", "Grep"]
        }
      }
    }
  })) {
    const msg = message as any;

    // Check for subagent invocation. Match both names: older SDK versions
    // emitted "Task", current versions emit "Agent".
    for (const block of msg.message?.content ?? []) {
      if (block.type === "tool_use" && (block.name === "Task" || block.name === "Agent")) {
        console.log(`Subagent invoked: ${block.input.subagent_type}`);
      }
    }

    // Check if this message is from within a subagent's context
    if (msg.parent_tool_use_id) {
      console.log("  (running inside subagent)");
    }

    if ("result" in message) {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<h2 id="resume-subagents">
  Fortsetzen von Subagenten
</h2>

Sie können einen Subagenten fortsetzen, um dort fortzufahren, wo er aufgehört hat, anstatt von vorne zu beginnen. Ein fortgesetzter Subagent behält seine vollständige Konversationshistorie, einschließlich aller vorherigen Tool-Aufrufe, Ergebnisse und Reasoning.

Wenn ein Subagent abgeschlossen ist, enthält das Agent-Tool-Ergebnis einen Textblock mit `agentId: <id>`. Die integrierten [`Explore`- und `Plan`-Agenten](/docs/de/sub-agents#built-in-subagents) sind einmalig und geben keine `agentId` zurück, daher verwenden Sie einen benutzerdefinierten Agent oder `general-purpose`, wenn Sie fortsetzen müssen. Um einen Subagenten programmatisch fortzusetzen:

1. **Erfassen Sie die Session-ID**: Extrahieren Sie `session_id` aus Nachrichten während der ersten Abfrage
2. **Extrahieren Sie die Agent-ID**: Analysieren Sie `agentId` aus dem Agent-Tool-Ergebnis-Text
3. **Setzen Sie die Session fort**: Übergeben Sie `resume: sessionId` in den Optionen der zweiten Abfrage und fügen Sie die Agent-ID in Ihren Prompt ein

<Note>
  Sie müssen dieselbe Session fortsetzen, um auf das Transkript des Subagenten zuzugreifen. Jeder `query()`-Aufruf startet standardmäßig eine neue Session, daher übergeben Sie `resume: sessionId`, um in derselben Session fortzufahren.

  Wenn Sie einen benutzerdefinierten Agent verwenden, übergeben Sie dieselbe Agent-Definition im Parameter `agents` für beide Abfragen.
</Note>

Das folgende Beispiel definiert einen benutzerdefinierten `endpoint-finder`-Agent. Die erste Abfrage führt ihn aus und erfasst die Session-ID und Agent-ID aus dem Agent-Tool-Ergebnis, dann setzt die zweite Abfrage die Session fort, um eine Folgefrage zu stellen, die Kontext aus der ersten Analyse erfordert.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import re
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition, ToolResultBlock

  AGENTS = {
      "endpoint-finder": AgentDefinition(
          description="Locates and catalogs API endpoints in a codebase.",
          prompt="You find and document API endpoints. Report each endpoint's path, method, and handler.",
          tools=["Read", "Grep", "Glob"],
      )
  }


  def extract_agent_id(block: ToolResultBlock) -> str | None:
      """Extract agentId from an Agent tool result's text content."""
      parts = block.content if isinstance(block.content, list) else [{"text": block.content}]
      for part in parts:
          if match := re.search(r"agentId:\s*([\w-]+)", part.get("text") or ""):
              return match.group(1)
      return None


  async def main():
      agent_id = None
      session_id = None

      # First invocation - run the endpoint-finder subagent
      try:
          async for message in query(
              prompt="Use the endpoint-finder agent to find all API endpoints in this codebase",
              options=ClaudeAgentOptions(allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS),
          ):
              # Capture session_id from ResultMessage (needed to resume this session)
              if hasattr(message, "session_id"):
                  session_id = message.session_id
              # Search tool results for the agentId trailer
              for block in getattr(message, "content", None) or []:
                  if isinstance(block, ToolResultBlock):
                      agent_id = extract_agent_id(block) or agent_id
              # Print the final result
              if hasattr(message, "result"):
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so session_id and agent_id have already been captured by the loop above.
          print(f"Session ended with an error: {error}")

      # Second invocation - resume and ask follow-up
      if agent_id and session_id:
          async for message in query(
              prompt=f"Resume agent {agent_id} and list the top 3 most complex endpoints",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Grep", "Glob", "Agent"], agents=AGENTS, resume=session_id
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)
      else:
          print("No agentId found in the first query, so there is no subagent to resume.")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, type SDKMessage } from "@anthropic-ai/claude-agent-sdk";

  const agents = {
    "endpoint-finder": {
      description: "Locates and catalogs API endpoints in a codebase.",
      prompt: "You find and document API endpoints. Report each endpoint's path, method, and handler.",
      tools: ["Read", "Grep", "Glob"]
    }
  };

  // Stringify content to search for agentId without traversing nested block types
  function extractAgentId(message: SDKMessage): string | undefined {
    if (message.type !== "assistant" && message.type !== "user") return undefined;
    const content = JSON.stringify(message.message.content);
    const match = content.match(/agentId:\s*([\w-]+)/);
    return match?.[1];
  }

  let agentId: string | undefined;
  let sessionId: string | undefined;

  // First invocation - run the endpoint-finder subagent
  try {
    for await (const message of query({
      prompt: "Use the endpoint-finder agent to find all API endpoints in this codebase",
      options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents }
    })) {
      // Capture session_id from ResultMessage (needed to resume this session)
      if ("session_id" in message) sessionId = message.session_id;
      // Search message content for the agentId (appears in Agent tool results)
      const extractedId = extractAgentId(message);
      if (extractedId) agentId = extractedId;
      // Print the final result
      if ("result" in message) console.log(message.result);
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so sessionId and agentId have already been captured by the loop above.
    console.error(`Session ended with an error: ${error}`);
  }

  // Second invocation - resume and ask follow-up
  if (agentId && sessionId) {
    for await (const message of query({
      prompt: `Resume agent ${agentId} and list the top 3 most complex endpoints`,
      options: { allowedTools: ["Read", "Grep", "Glob", "Agent"], agents, resume: sessionId }
    })) {
      if ("result" in message) console.log(message.result);
    }
  } else {
    console.log("No agentId found in the first query, so there is no subagent to resume.");
  }
  ```
</CodeGroup>

Subagenten-Transkripte bleiben unabhängig von der Hauptkonversation bestehen:

* **Hauptkonversations-Komprimierung**: Wenn die Hauptkonversation komprimiert wird, sind Subagenten-Transkripte nicht betroffen. Sie werden in separaten Dateien gespeichert.
* **Session-Persistenz**: Subagenten-Transkripte bleiben innerhalb ihrer Session bestehen. Sie können einen Subagenten nach dem Neustart von Claude Code fortsetzen, indem Sie dieselbe Session fortsetzen.
* **Automatische Bereinigung**: Transkripte werden basierend auf der Einstellung `cleanupPeriodDays` bereinigt, die standardmäßig auf 30 Tage eingestellt ist.

<h2 id="tool-restrictions-2">
  Tool-Einschränkungen
</h2>

Subagenten können über das Feld `tools` eingeschränkten Tool-Zugriff haben:

* **Feld weglassen**: Agent erbt alle verfügbaren Tools (Standard)
* **Tools angeben**: Agent kann nur aufgelistete Tools verwenden

Dieses Beispiel erstellt einen schreibgeschützten Analyse-Agent, der Code untersuchen, aber keine Dateien ändern oder Befehle ausführen kann.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Analyze the architecture of this codebase",
          options=ClaudeAgentOptions(
              allowed_tools=["Read", "Grep", "Glob", "Agent"],
              agents={
                  "code-analyzer": AgentDefinition(
                      description="Static code analysis and architecture review",
                      prompt="""You are a code architecture analyst. Analyze code structure,
  identify patterns, and suggest improvements without making changes.""",
                      # Read-only tools: no Edit, Write, or Bash access
                      tools=["Read", "Grep", "Glob"],
                  )
              },
          ),
      ):
          if hasattr(message, "result"):
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Analyze the architecture of this codebase",
    options: {
      allowedTools: ["Read", "Grep", "Glob", "Agent"],
      agents: {
        "code-analyzer": {
          description: "Static code analysis and architecture review",
          prompt: `You are a code architecture analyst. Analyze code structure,
  identify patterns, and suggest improvements without making changes.`,
          // Read-only tools: no Edit, Write, or Bash access
          tools: ["Read", "Grep", "Glob"]
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<h3 id="common-tool-combinations">
  Häufige Tool-Kombinationen
</h3>

| Anwendungsfall            | Tools                                   | Beschreibung                                                      |
| :------------------------ | :-------------------------------------- | :---------------------------------------------------------------- |
| Schreibgeschützte Analyse | `Read`, `Grep`, `Glob`                  | Kann Code untersuchen, aber nicht ändern oder ausführen           |
| Test-Ausführung           | `Bash`, `Read`, `Grep`                  | Kann Befehle ausführen und Ausgabe analysieren                    |
| Code-Änderung             | `Read`, `Edit`, `Write`, `Grep`, `Glob` | Vollständiger Lese-/Schreibzugriff ohne Befehlsausführung         |
| Vollständiger Zugriff     | Alle Tools                              | Erbt alle Tools vom übergeordneten Agent (Feld `tools` weglassen) |

<h2 id="scale-up-with-dynamic-workflows">
  Skalierung mit dynamischen Workflows
</h2>

Subagenten funktionieren gut für einige delegierte Aufgaben pro Turn. Für Läufe, die Dutzende bis Hunderte von Agenten koordinieren, verwenden Sie das `Workflow`-Tool, das die Orchestrierung in ein Skript verlagert, das die Laufzeit außerhalb des Konversationskontexts ausführt. Siehe [dynamische Workflows](/docs/de/workflows) für die Unterschiede zwischen Workflows und Turn-für-Turn-Subagenten-Delegation.

Das `Workflow`-Tool ist im TypeScript Agent SDK v0.3.149 und später verfügbar. Fügen Sie `Workflow` in `allowedTools` ein, um Workflow-Läufe automatisch zu genehmigen. Die Tool-Input- und Output-Schemas sind in der [TypeScript-Referenz](/docs/de/agent-sdk/typescript#workflow) aufgelistet.

<h2 id="troubleshooting">
  Fehlerbehebung
</h2>

<h3 id="claude-not-delegating-to-subagents">
  Claude delegiert nicht an Subagenten
</h3>

Wenn Claude Aufgaben direkt abschließt, anstatt an Ihren Subagenten zu delegieren:

* **Überprüfen Sie, dass Agent-Aufrufe genehmigt sind**: Fügen Sie `Agent` in `allowedTools` ein, um Subagenten-Aufrufe automatisch zu genehmigen. Ohne diese Einstellung fallen Agent-Aufrufe auf Ihren `canUseTool`-Callback zurück oder werden im `dontAsk`-Modus abgelehnt
* **Verwenden Sie explizites Prompting**: Erwähnen Sie den Subagenten nach Name in Ihrem Prompt, zum Beispiel „Verwenden Sie den Code-Reviewer-Agent, um..."
* **Schreiben Sie eine klare Beschreibung**: Erklären Sie genau, wann der Subagent verwendet werden sollte, damit Claude Aufgaben angemessen zuordnen kann

<h3 id="filesystem-based-agents-not-loading">
  Dateisystem-basierte Agenten werden nicht geladen
</h3>

Claude Code überwacht `~/.claude/agents/` und `.claude/agents/` und erkennt eine neue oder bearbeitete Agent-Datei innerhalb weniger Sekunden, ohne dass ein Neustart erforderlich ist. Wenn eine Definition nie angezeigt wird, arbeiten Sie diese Ursachen durch:

* **Neues `agents`-Verzeichnis**: Der Watcher deckt nur Verzeichnisse ab, die beim Start der Session vorhanden waren, daher benötigt die erste Datei in einem neuen Verzeichnis einen Session-Neustart. Dies ist die häufigste Ursache.
* **Ungültiges Frontmatter oder doppelter `name`**: Überprüfen Sie die YAML der Datei und ob ein vorhandener Agent bereits den `name` verwendet.
* **`--disable-slash-commands`**: Sessions, die mit diesem Flag gestartet wurden, überwachen diese Verzeichnisse nicht und benötigen immer einen Neustart, um neue Dateien zu laden.
* **Ein programmatischer Agent mit demselben Namen**: `agents`, die an `query()` übergeben werden, überschreiben einen Dateisystem-Agent mit demselben Namen.

Für das Dateiformat siehe [wie man Subagenten-Dateien schreibt](/docs/de/sub-agents#write-subagent-files).

<h3 id="long-prompt-failures-on-windows">
  Fehler bei langen Prompts unter Windows
</h3>

Unter Windows können Subagenten mit sehr langen Prompts aufgrund der Befehlszeilenlängenbeschränkung von 8191 Zeichen fehlschlagen. Halten Sie Prompts prägnant oder verwenden Sie dateisystem-basierte Agenten für komplexe Anweisungen.

<h2 id="related-documentation">
  Verwandte Dokumentation
</h2>

* [Claude Code Subagenten](/docs/de/sub-agents): umfassende Subagenten-Dokumentation einschließlich dateisystem-basierter Definitionen
* [Dynamische Workflows](/docs/de/workflows): Orchestrieren Sie viele Subagenten aus einem Skript für Jobs, die zu groß für eine Konversation sind
* [SDK-Übersicht](/docs/de/agent-sdk/overview): Erste Schritte mit dem Claude Agent SDK
