> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Subagents im SDK

> Definieren und rufen Sie Subagents auf, um den Kontext zu isolieren, Aufgaben parallel auszuführen und spezialisierte Anweisungen in Ihren Claude Agent SDK-Anwendungen anzuwenden.

Subagents sind separate Agent-Instanzen, die Ihr Hauptagent spawnen kann, um fokussierte Teilaufgaben zu bewältigen.
Verwenden Sie sie, um den Kontext zu isolieren, mehrere Analysen parallel auszuführen und spezialisierte Anweisungen anzuwenden, ohne das Prompt des Hauptagents zu erweitern.

<h2 id="overview">
  Übersicht
</h2>

Sie können Subagenten auf drei Arten erstellen:

* **Programmgesteuert**: Verwenden Sie den Parameter `agents` in Ihren `query()`-Optionen. Siehe die Referenzen für [TypeScript](/docs/de/agent-sdk/typescript#agentdefinition) und [Python](/docs/de/agent-sdk/python#agentdefinition)
* **Dateisystem-basiert**: Definieren Sie Agenten als Markdown-Dateien in `.claude/agents/`-Verzeichnissen. Siehe [Subagenten als Dateien definieren](/docs/de/sub-agents)
* **Integriert allgemein einsetzbar**: Claude kann den integrierten `general-purpose`-Subagenten jederzeit über das Agent-Tool aufrufen, ohne dass Sie etwas definieren müssen

Dieser Leitfaden konzentriert sich auf den programmgesteuerten Ansatz, der für SDK-Anwendungen empfohlen wird.

<h2 id="benefits-of-using-subagents">
  Vorteile der Verwendung von Subagenten
</h2>

Da Subagenten separate Agenten-Instanzen sind, bietet die Delegierung von Aufgaben an sie vier Vorteile:

* **Kontext-Isolation**: Jeder Subagent läuft in seiner eigenen Konversation, die neu beginnt, es sei denn, der Subagent ist ein [Fork](/docs/de/sub-agents#fork-the-current-conversation). In jedem Fall bleiben zwischenzeitliche Tool-Aufrufe und Ergebnisse innerhalb des Subagenten; nur seine abschließende Nachricht kehrt zum übergeordneten Agenten zurück. Ein `research-assistant`-Subagent kann Dutzende von Dateien durchsuchen, ohne dass sich dieser Inhalt in der Hauptkonversation ansammelt. Der übergeordnete Agent erhält eine prägnante Zusammenfassung, nicht jede Datei, die der Subagent gelesen hat. Siehe [What subagents inherit](#what-subagents-inherit) für genau das, was sich im Kontext des Subagenten befindet.
* **Parallelisierung**: Mehrere Subagenten können gleichzeitig ausgeführt werden, sodass unabhängige Teilaufgaben in der Zeit des langsamsten statt in der Summe aller abgeschlossen werden. Während einer Code-Überprüfung können Sie `style-checker`-, `security-scanner`- und `test-coverage`-Subagenten gleichzeitig statt nacheinander ausführen.
* **Spezialisierte Anweisungen und Wissen**: Jeder Subagent kann eine maßgeschneiderte System-Eingabeaufforderung mit spezifischer Expertise, Best Practices und Einschränkungen haben. Ein `database-migration`-Subagent kann detailliertes Wissen über SQL-Best-Practices, Rollback-Strategien und Datenintegritätsprüfungen haben, die in den Anweisungen des Hauptagenten unnötiges Rauschen wären.
* **Tool-Einschränkungen**: Subagenten können auf bestimmte Tools beschränkt werden, was das Risiko unbeabsichtigter Aktionen verringert. Ein `doc-reviewer`-Subagent könnte nur Zugriff auf Read- und Grep-Tools haben, um sicherzustellen, dass er analysieren, aber niemals versehentlich Ihre Dokumentationsdateien ändern kann.

<h2 id="create-subagents">
  Subagenten erstellen
</h2>

<h3 id="programmatic-definition-recommended">
  Programmatische Definition (empfohlen)
</h3>

Definieren Sie Subagenten direkt in Ihrem Code mit dem Parameter `agents`. Claude ruft Subagenten über das Tool `Agent` auf.

Die meisten Beispiele auf dieser Seite geben nur das Endergebnis aus. Um zu bestätigen, dass Claude an einen Subagenten delegiert hat, anstatt direkt zu antworten, siehe [Subagenten-Aufruf erkennen](#detect-subagent-invocation).

Dieses Beispiel erstellt zwei Subagenten: einen Code-Reviewer mit Lesezugriff und einen Test-Runner, der Befehle ausführen kann.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition


  async def main():
      async for message in query(
          prompt="Review the authentication module for security issues",
          options=ClaudeAgentOptions(
              # Auto-approve these tools
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
      // Auto-approve these tools
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

| Feld              | Typ                                                         | Erforderlich | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                 |
| :---------------- | :---------------------------------------------------------- | :----------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`     | `string`                                                    | Ja           | Natürlichsprachige Beschreibung, wann dieser Agent verwendet werden soll                                                                                                                                                                                                                                                                                                                     |
| `prompt`          | `string`                                                    | Ja           | Der System-Prompt des Agenten, der seine Rolle und sein Verhalten definiert                                                                                                                                                                                                                                                                                                                  |
| `tools`           | `string[]`                                                  | Nein         | Array von zulässigen Tool-Namen. Falls weggelassen, erbt jeden [für Subagenten verfügbaren Tool](/docs/de/sub-agents#available-tools)                                                                                                                                                                                                                                                             |
| `disallowedTools` | `string[]`                                                  | Nein         | Array von Tool-Namen, die aus dem Tool-Set des Agenten entfernt werden sollen. MCP-Server-Level-Muster werden ebenfalls akzeptiert: `mcp__server` oder `mcp__server__*` entfernt jeden Tool von diesem Server, und `mcp__*` entfernt jeden MCP-Tool von jedem Server                                                                                                                         |
| `model`           | `string`                                                    | Nein         | Modell-Override für diesen Agent. Akzeptiert einen Alias wie `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, oder eine vollständige Modell-ID. `'inherit'` verwendet das Hauptmodell. Wenn Sie es weglassen, wählt Claude Code das Modell in der [Subagenten-Modellreihenfolge](/docs/de/sub-agents#choose-a-model)                                                                      |
| `skills`          | `string[]`                                                  | Nein         | Liste von Skill-Namen, die beim Start in den Kontext des Agenten vorgeladen werden sollen. Nicht aufgelistete Skills bleiben über das Skill-Tool aufrufbar                                                                                                                                                                                                                                   |
| `memory`          | `'user' \| 'project' \| 'local'`                            | Nein         | Speicherquelle für diesen Agent                                                                                                                                                                                                                                                                                                                                                              |
| `mcpServers`      | `(string \| object)[]`                                      | Nein         | MCP-Server, die diesem Agent zur Verfügung stehen, nach Name oder Inline-Konfiguration                                                                                                                                                                                                                                                                                                       |
| `initialPrompt`   | `string`                                                    | Nein         | Wird automatisch als erste Benutzer-Runde eingereicht, wenn dieser Agent als Haupt-Thread-Agent läuft. Wird ignoriert, wenn der Agent als Subagent aufgerufen wird                                                                                                                                                                                                                           |
| `maxTurns`        | `number`                                                    | Nein         | Maximale Anzahl von Agent-Runden, bevor der Agent stoppt. Wenn der Agent das Limit erreicht, gibt Claude Code seine Ausgabe als teilweise markiert zurück, und Sie können [den Agent fortsetzen](#resume-subagents), um fortzufahren. Die teilweise Markierung erfordert Claude Code v2.1.246 oder später                                                                                    |
| `background`      | `boolean`                                                   | Nein         | Führen Sie diesen Agent als nicht-blockierende Hintergrund-Aufgabe aus, wenn er aufgerufen wird                                                                                                                                                                                                                                                                                              |
| `omitClaudeMd`    | `boolean`                                                   | Nein         | Führen Sie diesen Agent ohne die Benutzer-, Projekt- und lokalen CLAUDE.md-Dateien aus, wenn er als Subagent läuft; verwaltete Richtliniendateien werden weiterhin geladen. Wird ignoriert, wenn der Agent als Haupt-Thread-Agent läuft. Erfordert TypeScript Agent SDK v0.3.271 oder später. Das Python SDK [`AgentDefinition`](/docs/de/agent-sdk/python#agentdefinition) hat dieses Feld nicht |
| `effort`          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max' \| number` | Nein         | Reasoning-Aufwandsstufe für diesen Agent                                                                                                                                                                                                                                                                                                                                                     |
| `permissionMode`  | `PermissionMode`                                            | Nein         | Berechtigungsmodus für die Tool-Ausführung innerhalb dieses Agenten. Die [Subagenten-Vererbungsregeln](/docs/de/agent-sdk/permissions#available-modes) entscheiden, wann er angewendet wird                                                                                                                                                                                                       |

Im Python SDK behalten mehrteilige Feldnamen wie `disallowedTools` und `mcpServers` ihre camelCase-Schreibweise, um dem Wire-Format zu entsprechen, anstatt Pythons snake\_case-Konvention zu folgen. Siehe die [`AgentDefinition`-Referenz](/docs/de/agent-sdk/python#agentdefinition) für Details.

Subagenten laufen standardmäßig im Hintergrund. Ein Agent-Tool-Aufruf, der die [`run_in_background`](/docs/de/sub-agents#run-subagents-in-foreground-or-background)-Eingabe weglässt, startet einen Hintergrund-Subagenten, und Claude setzt `run_in_background: false`, wenn es das Ergebnis benötigt, bevor es fortfährt. Setzen Sie das Feld `background` auf `true`, um die Hintergrund-Ausführung für einen bestimmten Agent zu erzwingen, unabhängig davon, was Claude anfordert. Vor Claude Code v2.1.198 wurde der Hintergrund-Standard schrittweise eingeführt, und ein Agent-Tool-Aufruf, der `run_in_background` wegließ, konnte den Subagenten synchron ausführen.

Subagenten können auch ihre eigenen Subagenten spawnen. Um zu begrenzen, wie tief diese Verschachtelung geht, wie viele Subagenten gleichzeitig laufen und wie viel eine Abfrage ausgibt, siehe [Subagenten-Tiefe, Parallelität und Ausgaben begrenzen](#cap-subagent-depth-concurrency-and-spend).

<h3 id="filesystem-based-definition-alternative">
  Dateisystem-basierte Definition (Alternative)
</h3>

Sie können Subagenten auch als Markdown-Dateien in `.claude/agents/`-Verzeichnissen definieren. Siehe die [Claude Code Subagenten-Dokumentation](/docs/de/sub-agents) für Details zu diesem Ansatz. Programmatisch definierte Agenten haben Vorrang vor dateisystem-basierten Agenten mit demselben Namen.

<Note>
  Wenn Claude das Agent-Tool ohne einen `subagent_type` aufruft, erhält es den integrierten `general-purpose`-Subagenten, den Claude auch spawnen kann, wenn Sie keine Agenten selbst definieren. Das Setzen von [`CLAUDE_AGENT_SDK_DISABLE_BUILTIN_AGENTS=1`](/docs/de/env-vars) entfernt diesen Standard, und ein solcher Aufruf schlägt mit [`subagent_type is required`](/docs/de/errors#subagent-type-is-required) fehl.
</Note>

<h2 id="what-subagents-inherit">
  Was Subagenten erben
</h2>

Sofern der Subagent kein [Fork](/docs/de/sub-agents#fork-the-current-conversation) ist, startet sein Kontextfenster neu, ohne übergeordnete Konversation, ist aber nicht leer. Der einzige Inhalt, den Sie vom übergeordneten Agent zum Subagenten übergeben, ist die Prompt-Zeichenkette des Agent-Tools. Fügen Sie daher alle Dateipfade, Fehlermeldungen oder Entscheidungen, die der Subagent benötigt, direkt in diese Prompt ein.

Ein Subagent, der das [`SendMessage`](/docs/de/tools-reference)-Tool hat, startet mit einer Liste der anderen benannten Agenten, die in der Sitzung ausgeführt werden, sodass er weiß, an welche Namen er Nachrichten senden kann. Claude Code fügt die Liste automatisch beim ersten Durchgang des Subagenten hinzu. Ein [Fork](/docs/de/sub-agents#fork-the-current-conversation) erhält die Liste nicht, da er stattdessen die übergeordnete Konversation erbt.

Ein Subagent erbt auch die Konfiguration des erweiterten Denkens der Hauptsitzung.

Die folgende Tabelle zeigt, welche Inhalte der Kontext eines Nicht-Fork-Subagenten enthält und was er nicht enthält.

| Der Subagent erhält                                                                                                                                                                                                    | Der Subagent erhält nicht                                                       |
| :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------ |
| Seinen eigenen System-Prompt (`AgentDefinition.prompt`) und den Prompt des Agent-Tools                                                                                                                                 | Die Konversationshistorie oder Tool-Ergebnisse des übergeordneten Agenten       |
| Projekt CLAUDE.md (geladen über [`settingSources`](/docs/de/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)), sofern der Agent nicht [`omitClaudeMd`](#agentdefinition-configuration) setzt | Vorgeladene Skill-Inhalte, sofern nicht in `AgentDefinition.skills` aufgelistet |
| Tool-Definitionen (geerbt vom übergeordneten Agent oder die Teilmenge in `tools`, [gefiltert für Hintergrund-Läufe](/docs/de/sub-agents#available-tools))                                                                   | Der System-Prompt des übergeordneten Agenten                                    |

<Note>
  Der übergeordnete Agent erhält die letzte Nachricht des Subagenten als Agent-Tool-Ergebnis, kann sie aber in seiner eigenen Antwort zusammenfassen. Um die Ausgabe des Subagenten wörtlich in der benutzerorientierten Antwort zu bewahren, fügen Sie eine Anweisung dazu in den Prompt oder die `systemPrompt`-Option ein, die Sie an den Hauptaufruf `query()` übergeben.

  In v2.1.210 und später [scannt Claude Code die letzte Nachricht auf anweisungsähnliche Muster](/docs/de/sub-agents#subagent-output-scanning), bevor der übergeordnete Agent sie liest. Der Scan behandelt drei Arten von Mustern unterschiedlich:

  * **Imitation von Steuertags**: Claude Code neutralisiert ein Tag, das nur das Harness ausgibt, wie einen `<system-reminder>`-Block, an Ort und Stelle. Es fügt einen Backslash nach der öffnenden spitzen Klammer ein und löscht nichts.
  * **Erwähnungen der Berechtigungskonfiguration**: Claude Code behält Verweise auf die Berechtigungskonfiguration, wie `.claude/settings.json`, `bypassPermissions` oder `--dangerously-skip-permissions`, wie geschrieben bei.
  * **Durchsatzmarker**: Eine Zeile, die mit `Human:` oder `Assistant:` beginnt, erhält einen Backslash vor dem Doppelpunkt, sodass die Nachricht keine Konversationsdurchsatzgrenze imitieren kann.

  Bei einer Steuertag- oder Berechtigungskonfigurationsübereinstimmung stellt Claude Code eine `[harness: ...]`-Markerzeile voran, die die übereinstimmenden Muster benennt; eine Durchsatzmarker-Übereinstimmung fügt die Markerzeile nicht hinzu. Dies sind die einzigen Änderungen, die der Scan vornimmt: Er entfernt oder umformuliert niemals den Text des Subagenten.
</Note>

Ein API-Fehler, der den Subagenten vorzeitig beendet, wie z. B. eine Ratenbegrenzung, wird niemals als sein Ergebnis bereitgestellt. Siehe [API-Fehler in Subagenten](/docs/de/sub-agents#api-errors-in-subagents) für das Vordergrund- und Hintergrundverhalten.

<h2 id="invoke-subagents">
  Subagenten aufrufen
</h2>

<h3 id="automatic-invocation">
  Automatischer Aufruf
</h3>

Claude entscheidet automatisch, wann Subagenten basierend auf der Aufgabe und der `description` jedes Subagenten aufgerufen werden. Wenn Sie beispielsweise einen `performance-optimizer`-Subagenten mit der Beschreibung „Performance-Optimierungsspezialist für Query-Tuning" definieren, wird Claude ihn aufrufen, wenn Ihr Prompt die Optimierung von Queries erwähnt.

Schreiben Sie klare, spezifische Beschreibungen, damit Claude Aufgaben dem richtigen Subagenten zuordnen kann.

<h3 id="explicit-invocation">
  Expliziter Aufruf
</h3>

Um sicherzustellen, dass Claude einen bestimmten Subagenten verwendet, erwähnen Sie ihn namentlich in Ihrem Prompt:

```text theme={null}
"Use the code-reviewer agent to check the authentication module"
```

Dies umgeht das automatische Matching und ruft den benannten Subagenten direkt auf.

<h3 id="dynamic-agent-configuration">
  Dynamische Agent-Konfiguration
</h3>

Sie können Agent-Definitionen dynamisch basierend auf Laufzeitbedingungen erstellen. Dieses Beispiel erstellt einen Security-Reviewer mit verschiedenen Strenge-Ebenen und verwendet ein leistungsfähigeres Modell für strenge Überprüfungen.

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
  Subagent-Aufrufe erkennen
</h2>

Claude ruft Subagents über das Agent-Tool auf. Um zu erkennen, wenn ein Subagent aufgerufen wird, suchen Sie nach `tool_use`-Blöcken, bei denen `name` gleich `"Agent"` ist. Nachrichten aus dem Kontext eines Subagents enthalten ein `parent_tool_use_id`-Feld.

<Note>
  Das Tool wird in `tool_use`-Blöcken als `"Agent"` angezeigt, aber in der `system:init`-Toolsliste als `"Task"`. Vor Claude Code v2.1.63 benannten `tool_use`-Blöcke es ebenfalls `"Task"`. Um die Erkennung über SDK-Versionen hinweg funktionsfähig zu halten, stimmen Sie beide Werte in `block.name` ab.
</Note>

Die Nachrichtenstruktur unterscheidet sich zwischen SDKs. In Python greifen Sie direkt über `message.content` auf Inhaltsblöcke zu. In TypeScript umhüllt `SDKAssistantMessage` die Claude-API-Nachricht, daher greifen Sie über `message.message.content` auf Inhalte zu.

Dieses Beispiel durchläuft gestreamte Nachrichten und protokolliert, wenn ein Subagent aufgerufen wird und wenn nachfolgende Nachrichten aus dem Ausführungskontext dieses Subagents stammen.

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
  Subagenten fortsetzen
</h2>

Sie können einen Subagenten fortsetzen, um dort weiterzumachen, wo er aufgehört hat, anstatt von vorne zu beginnen. Ein fortgesetzter Subagent behält seine vollständige Gesprächshistorie bei, einschließlich aller vorherigen Tool-Aufrufe, Ergebnisse und Überlegungen.

Wenn ein Subagent sein [`maxTurns`](#agentdefinition-configuration)-Limit erreicht, markiert Claude Code die Ausgabe im Agent-Tool-Ergebnis als teilweise, damit Claude weiß, dass die Ausführung unvollständig ist.

Wenn ein Subagent abgeschlossen ist, enthält das Agent-Tool-Ergebnis einen Textblock mit `agentId: <id>`. Die integrierten [`Explore`- und `Plan`-Agenten](/docs/de/sub-agents#built-in-subagents) sind einmalig und geben keine `agentId` zurück. Verwenden Sie daher einen benutzerdefinierten Agenten oder `general-purpose`, wenn Sie fortsetzen müssen. Um einen Subagenten programmgesteuert fortzusetzen:

1. **Erfassen Sie die Sitzungs-ID**: Extrahieren Sie `session_id` aus Nachrichten während der ersten Abfrage
2. **Extrahieren Sie die Agent-ID**: Analysieren Sie `agentId` aus dem Agent-Tool-Ergebnis-Text
3. **Setzen Sie die Sitzung fort**: Übergeben Sie `resume: sessionId` in den Optionen der zweiten Abfrage, und fügen Sie die Agent-ID in Ihrem Prompt ein. Jeder `query()`-Aufruf startet standardmäßig eine neue Sitzung, und Sie müssen dieselbe Sitzung fortsetzen, um auf das Transkript des Subagenten zuzugreifen.

<Note>
  Wenn Sie einen benutzerdefinierten Agenten verwenden, übergeben Sie dieselbe Agent-Definition im `agents`-Parameter für beide Abfragen.
</Note>

Das folgende Beispiel definiert einen benutzerdefinierten `endpoint-finder`-Agenten. Die erste Abfrage führt ihn aus und erfasst die Sitzungs-ID und Agent-ID aus dem Agent-Tool-Ergebnis. Dann setzt die zweite Abfrage die Sitzung fort, um eine Folgefrage zu stellen, die Kontext aus der ersten Analyse erfordert.

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

Subagenten-Transkripte werden in separaten Dateien gespeichert und bleiben unabhängig vom Hauptgespräch erhalten. Siehe [Subagenten in Claude Code fortsetzen](/docs/de/sub-agents#resume-subagents) für das Komprimierungsverhalten und den `cleanupPeriodDays`-Bereinigungszeitraum.

<h2 id="tool-restrictions">
  Werkzeugbeschränkungen
</h2>

Verwenden Sie das Feld `tools`, um zu begrenzen, was ein Subagent tun kann:

* **`tools` weglassen**: Der Subagent erhält jedes [Werkzeug, das für Subagenten verfügbar ist](/docs/de/sub-agents#available-tools)
* **Werkzeuge auflisten**: Der Subagent erhält nur diese. Ein Code-Reviewer, der niemals Dateien bearbeiten sollte, erhält beispielsweise `["Read", "Grep", "Glob"]`

Ein Werkzeug, das Sie weglassen, ist überhaupt nicht in der Sitzung des Subagenten vorhanden: Claude arbeitet ohne es, ohne Berechtigungsaufforderung oder Fehler.

Dieses Beispiel erstellt einen schreibgeschützten Analyse-Agent, der Code untersuchen kann, aber keine Dateien ändern oder Befehle ausführen kann.

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
  Häufige Werkzeugkombinationen
</h3>

| Anwendungsfall            | Werkzeuge                               | Beschreibung                                                           |
| :------------------------ | :-------------------------------------- | :--------------------------------------------------------------------- |
| Schreibgeschützte Analyse | `Read`, `Grep`, `Glob`                  | Kann Code untersuchen, aber nicht ändern oder ausführen                |
| Testausführung            | `Bash`, `Read`, `Grep`                  | Kann Befehle ausführen und Ausgabe analysieren                         |
| Code-Änderung             | `Read`, `Edit`, `Write`, `Grep`, `Glob` | Vollständiger Lese-/Schreibzugriff ohne Befehlsausführung              |
| Vollständiger Zugriff     | Alle Werkzeuge                          | Erbt die für Subagenten verfügbaren Werkzeuge (Feld `tools` weglassen) |

<h2 id="cap-subagent-depth-concurrency-and-spend">
  Tiefe, Parallelität und Ausgaben von Subagenten begrenzen
</h2>

<Note>
  Dieser Abschnitt beschreibt TypeScript SDK v0.3.219 und Python SDK v0.2.127 und später, die Versionen, die Claude Code v2.1.219 oder später enthalten. Bei früheren Versionen fehlen einige dieser Limits oder haben andere Standardwerte, daher sollten Sie ein Upgrade durchführen, bevor Sie sich auf diese Limits verlassen. Die [Umgebungsvariablenreferenz](/docs/de/env-vars) und [Durchläufe und Budget](/docs/de/agent-sdk/agent-loop#turns-and-budget) dokumentieren die Claude Code-Version, die jede Variable hinzugefügt hat, und die Durchsetzung der Ausgabenbegrenzung für Subagenten.
</Note>

Claude entscheidet selbst, wann ein Subagent erzeugt werden soll und wie viele erzeugt werden sollen. Jeder Subagent stellt seine eigenen API-Anfragen, die zur `total_cost_usd` der Abfrage zählen, und ein Subagent kann seine eigenen Subagenten erzeugen, sodass ein Prompt zu einem Baum von Agenten wachsen kann.

Sie können dieses Wachstum auf drei Arten begrenzen: wie tief Subagenten verschachtelt sind, wie viele gleichzeitig ausgeführt werden und wie viel die gesamte Abfrage kostet. Legen Sie die Tiefe- und Parallelitätslimits als Umgebungsvariablen über die [`env`](/docs/de/agent-sdk/typescript#options)-Option fest, und das Ausgabenlimit als Abfrageoption:

| Limit        | Festlegen mit                                            | Standard                                                                                                            | Was Claude Code beim Limit tut                                                                                                                                                                                                                                                                                                                                                                             |
| :----------- | :------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Tiefe        | [`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH`](/docs/de/env-vars)   | `3` Ebenen von Subagenten unter Ihrem Hauptagenten. `1` verhindert, dass Ihre Subagenten ihre eigenen erzeugen      | Lässt einen Subagenten in der untersten Ebene unfähig sein zu erzeugen, sodass er seine delegierte Arbeit selbst erledigt. Siehe [verschachtelte Subagenten](/docs/de/sub-agents#let-subagents-spawn-their-own-subagents)                                                                                                                                                                                       |
| Parallelität | [`CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS`](/docs/de/env-vars)   | `20` Subagenten, die gleichzeitig ausgeführt werden, zählen jeden Subagenten, den Claude mit dem Agent-Tool erzeugt | Weigert sich, einen weiteren Subagenten zu erzeugen, gibt `Concurrent subagent limit reached` zurück, bis die laufende Anzahl unter das Limit fällt. Sitzungen mit aktivem [ultracode](/docs/de/model-config#adjust-effort-level) werden nie abgelehnt. Siehe das [Parallelitätslimit für Subagenten](/docs/de/sub-agents#concurrent-subagent-limit)                                                                 |
| Ausgaben     | `maxBudgetUsd` in TypeScript, `max_budget_usd` in Python | Kein Limit. Zählt die Ausgaben des Aufrufs selbst, einschließlich Subagenten-Anfragen                               | Erzwingt die Obergrenze auf drei Arten: weigert sich, mehr Subagenten zu erzeugen, gibt `Budget limit reached` zurück, stoppt Hintergrund-Subagenten, die noch ausgeführt werden, und beendet die Abfrage mit dem Ergebnis-Subtyp `error_max_budget_usd`. Wie sich die Obergrenzen über eine Sitzung hinweg verhalten, finden Sie unter [Durchläufe und Budget](/docs/de/agent-sdk/agent-loop#turns-and-budget) |

Die beiden SDKs behandeln die `env`-Option unterschiedlich: Das TypeScript SDK ersetzt die Subprocess-Umgebung damit, daher verteilen Sie `process.env` darin, um Variablen wie `PATH` zu behalten, während das Python SDK es in die geerbte Umgebung zusammenführt. Dieses Beispiel deaktiviert Verschachtelung, erlaubt höchstens fünf Subagenten gleichzeitig und stoppt die Abfrage, sobald die geschätzten Ausgaben \$5 erreichen:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      try:
          async for message in query(
              prompt="Audit every service in this repo for unhandled promise rejections",
              options=ClaudeAgentOptions(
                  allowed_tools=["Read", "Grep", "Glob", "Agent"],
                  # env is merged on top of the inherited environment
                  env={
                      "CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH": "1",
                      "CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS": "5",
                  },
                  max_budget_usd=5.0,
              ),
          ):
              if isinstance(message, ResultMessage):
                  print(f"{message.subtype}: ${message.total_cost_usd}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result,
          # so the budget-capped result has already been printed above.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Audit every service in this repo for unhandled promise rejections",
      options: {
        allowedTools: ["Read", "Grep", "Glob", "Agent"],
        // env replaces the subprocess environment, so spread process.env to keep PATH
        env: {
          ...process.env,
          CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH: "1",
          CLAUDE_CODE_MAX_CONCURRENT_SUBAGENTS: "5",
        },
        maxBudgetUsd: 5,
      },
    })) {
      if (message.type === "result") {
        console.log(`${message.subtype}: $${message.total_cost_usd}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result,
    // so the budget-capped result has already been logged above.
    console.error(`Session ended with an error: ${error}`);
  }
  ```
</CodeGroup>

Was Sie sehen, hängt davon ab, welches Limit, falls vorhanden, die Abfrage erreicht:

* **Unter der Ausgabenbegrenzung**: Sie sehen `success` und die geschätzten Kosten.
* **Bei der Ausgabenbegrenzung**: Sie sehen `error_max_budget_usd` mit Kosten bei oder über `5`, und dann wird Ihr Fehlerhandler ausgeführt.
* **Bei der Parallelitätsbegrenzung**: Sie sehen einen `tool_result`-Block im Nachrichtenstrom mit `Concurrent subagent limit reached`. Claude erhält denselben Block als Ergebnis des Agent-Tools.

<h3 id="run-opus-5-with-subagents">
  Opus 5 mit Subagenten ausführen
</h3>

Claude Opus 5 delegiert bereitwilliger an Subagenten als frühere Modelle, daher sind die [Tiefe-, Parallelitäts- und Ausgabenlimits](#cap-subagent-depth-concurrency-and-spend) am wichtigsten bei Abfragen, die Opus 5 ausführen. Der [Opus 5-Prompting-Leitfaden](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-opus-5#controlling-subagent-spawning) enthält eine Delegationsanweisung, die Sie zu jedem Prompt hinzufügen können. Ob Claude Code eine eigene Anweisung hinzufügt, hängt davon ab, welche [System-Prompt](/docs/de/agent-sdk/modifying-system-prompts#how-system-prompts-work) Sie verwenden:

* **`claude_code`-Voreinstellung**: Wenn das Modell Opus 5 ist, fügt Claude Code eine Zeile zu seinem System-Prompt hinzu, die Claude mitteilt, das Agent-Tool nicht aufzurufen, es sei denn, es wird danach gefragt. Das Agent-Tool bleibt verfügbar.
* **Ein benutzerdefinierter Prompt oder kein `systemPrompt`**: Claude Code erstellt seinen System-Prompt nicht, daher fehlt diese Zeile. Fügen Sie die Delegationsanweisung aus dem Prompting-Leitfaden zu Ihrem eigenen Prompt hinzu.

Jede Anweisung lenkt nur Claude, daher legen Sie auch die Limits fest. Claude Code erzwingt sie, wie auch immer Claude delegiert.

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

* **Verwenden Sie explizites Prompting**: Erwähnen Sie den Subagenten nach Name in Ihrem Prompt, zum Beispiel „Verwenden Sie den Code-Reviewer-Agent, um das Authentifizierungsmodul zu überprüfen"
* **Schreiben Sie eine klare Beschreibung**: Erklären Sie genau, wann der Subagent verwendet werden sollte, damit Claude Aufgaben angemessen zuordnen kann

<h3 id="filesystem-based-agents-not-loading">
  Dateisystem-basierte Agenten werden nicht geladen
</h3>

Claude Code überwacht `~/.claude/agents/` und `.claude/agents/` und erkennt eine neue oder bearbeitete Agent-Datei innerhalb weniger Sekunden, ohne dass ein Neustart erforderlich ist. Wenn eine Definition nie angezeigt wird, arbeiten Sie diese Ursachen durch:

* **Neues `agents`-Verzeichnis**: Der Watcher deckt nur Verzeichnisse ab, die beim Start der Session vorhanden waren, daher benötigt die erste Datei in einem neuen Verzeichnis einen Session-Neustart. Dies ist die häufigste Ursache.
* **Ungültiges Frontmatter oder doppelter `name`**: Überprüfen Sie die YAML der Datei und ob ein vorhandener Agent bereits den `name` verwendet.
* **`--disable-slash-commands`**: Sessions, die mit diesem Flag gestartet wurden, überwachen diese Verzeichnisse nicht und benötigen immer einen Neustart, um neue Dateien zu laden.
* **Eine Datei unter einem hinzugefügten Verzeichnis**: Claude Code lädt `.claude/agents/` aus Verzeichnissen, die mit der `add_dirs`-Option (Python) oder `additionalDirectories`-Option (TypeScript) oder der CLI-Option `--add-dir` oder `/add-dir` hinzugefügt wurden, überwacht sie aber nicht, daher benötigt eine neue oder bearbeitete Datei dort einen Session-Neustart.
* **Ein programmatischer Agent mit demselben Namen**: `agents`, die an `query()` übergeben werden, überschreiben einen Dateisystem-Agent mit demselben Namen.

Für das Dateiformat siehe [wie man Subagenten-Dateien schreibt](/docs/de/sub-agents#write-subagent-files).

<h2 id="related-documentation">
  Verwandte Dokumentation
</h2>

* [Claude Code Subagenten](/docs/de/sub-agents): umfassende Subagenten-Dokumentation einschließlich dateisystem-basierter Definitionen
* [Dynamische Workflows](/docs/de/workflows): Orchestrieren Sie viele Subagenten aus einem Skript für Jobs, die zu groß für eine Konversation sind
* [SDK-Übersicht](/docs/de/agent-sdk/overview): Erste Schritte mit dem Claude Agent SDK
