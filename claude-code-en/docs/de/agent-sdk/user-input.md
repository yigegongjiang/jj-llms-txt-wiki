> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Genehmigungen und Benutzereingaben verarbeiten

> Zeigen Sie Claudes Genehmigungsanfragen und Klärungsfragen den Benutzern an und geben Sie deren Entscheidungen an das SDK zurück.

Während Claude an einer Aufgabe arbeitet, muss er manchmal mit Benutzern Rücksprache halten. Er könnte eine Genehmigung benötigen, bevor er Dateien löscht, oder fragen müssen, welche Datenbank für ein neues Projekt verwendet werden soll. Ihre Anwendung muss diese Anfragen den Benutzern anzeigen, damit Claude mit deren Eingabe fortfahren kann.

Claude fordert Benutzereingaben in zwei Situationen an: wenn er **Genehmigung zur Verwendung eines Tools benötigt** (wie das Löschen von Dateien oder das Ausführen von Befehlen) und wenn er **Klärungsfragen hat** (über das `AskUserQuestion`-Tool). Beide lösen Ihren `canUseTool`-Callback aus, der die Ausführung pausiert, bis Sie eine Antwort zurückgeben. Dies unterscheidet sich von normalen Gesprächsrunden, bei denen Claude fertig ist und auf Ihre nächste Nachricht wartet.

Bei Klärungsfragen generiert Claude die Fragen und Optionen. Ihre Aufgabe besteht darin, sie den Benutzern zu präsentieren und ihre Auswahl zurückzugeben. Sie können diesem Ablauf keine eigenen Fragen hinzufügen; wenn Sie Benutzer selbst etwas fragen müssen, tun Sie dies separat in Ihrer Anwendungslogik.

Der Callback kann unbegrenzt ausstehend bleiben. Die Ausführung bleibt pausiert, bis Ihr Callback zurückkommt, und das SDK bricht das Warten nur ab, wenn die Abfrage selbst abgebrochen wird. Wenn ein Benutzer länger braucht, um zu antworten, als Ihr Prozess vernünftigerweise laufen kann, geben Sie die [`defer`-Hook-Entscheidung](/docs/de/hooks#defer-a-tool-call-for-later) zurück, mit der der Prozess beendet und später aus der persistierten Sitzung fortgesetzt werden kann.

Diese Anleitung zeigt Ihnen, wie Sie jeden Anforderungstyp erkennen und angemessen reagieren.

<h2 id="detect-when-claude-needs-input">
  Erkennen Sie, wenn Claude Eingaben benötigt
</h2>

Übergeben Sie einen `canUseTool`-Callback in Ihren Abfrageoptionen. Der Callback wird ausgelöst, wenn Claude Benutzereingaben benötigt, und erhält den Tool-Namen und die Eingabe als Argumente:

<CodeGroup>
  ```python Python theme={null}
  async def handle_tool_request(tool_name, input_data, context):
      # Benutzer auffordern und Zulassung oder Ablehnung zurückgeben
      ...


  options = ClaudeAgentOptions(can_use_tool=handle_tool_request)
  ```

  ```typescript TypeScript theme={null}
  async function handleToolRequest(toolName, input, options) {
    // options includes { signal: AbortSignal, suggestions?: PermissionUpdate[] }
    // Benutzer auffordern und Zulassung oder Ablehnung zurückgeben
  }

  const options = { canUseTool: handleToolRequest };
  ```
</CodeGroup>

Der Callback wird in zwei Fällen ausgelöst:

1. **Tool benötigt Genehmigung**: Claude möchte ein Tool verwenden, das nicht durch eine [Berechtigungsregel](/docs/de/agent-sdk/permissions) oder einen Berechtigungsmodus automatisch genehmigt wird. Überprüfen Sie `tool_name` auf das Tool (z. B. `"Bash"`, `"Write"`).
2. **Claude stellt eine Frage**: Claude ruft das `AskUserQuestion`-Tool auf. Überprüfen Sie, ob `tool_name == "AskUserQuestion"`, um es anders zu behandeln. Wenn Sie ein `tools`-Array angeben, fügen Sie `AskUserQuestion` ein, damit dies funktioniert. Siehe [Klärungsfragen verarbeiten](#handle-clarifying-questions) für Details.

<Warning>
  **Der Callback wird nie für automatisch genehmigte Tools ausgelöst.** Jede Genehmigung früher im [Berechtigungsevaluierungsfluss](/docs/de/agent-sdk/permissions#how-permissions-are-evaluated), eine Zulassungsregel oder ein Modus wie `acceptEdits` oder `bypassPermissions`, löst den Aufruf auf, bevor `canUseTool` konsultiert wird. Wenn Sie ein Tool einfach in `allowed_tools` auflisten, wird eine `canUseTool`-Überprüfung für dieses Tool nie ausgeführt, es sei denn, eine Abfrage-Regel oder der `plan`-Modus leitet den Aufruf zurück zu einer Eingabeaufforderung. Für Logik, die auf jeden Tool-Aufruf angewendet werden muss, verwenden Sie einen [`PreToolUse`-Hook](/docs/de/agent-sdk/hooks), der vor dem Rest des Flusses ausgeführt wird und Anfragen zulassen, ablehnen oder ändern kann.

  `AskUserQuestion`, MCP-Tools, die mit [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, und Connector-Tools, [die Ihre Organisation auf `ask`](/docs/de/mcp#organization-controls-on-connector-tools) gesetzt hat, erreichen den Callback auch dann, wenn eine Zulassungsregel zutrifft. Im `dontAsk`-Modus werden diese Aufrufe stattdessen abgelehnt, ohne den Callback aufzurufen.
</Warning>

Sie können auch den [`PermissionRequest`-Hook](/docs/de/agent-sdk/hooks#available-hooks) verwenden, um externe Benachrichtigungen (Slack, E-Mail, Push) zu senden, wenn Claude auf Genehmigung wartet.

<h2 id="handle-tool-approval-requests">
  Tool-Genehmigungsanfragen verarbeiten
</h2>

Nachdem Sie einen `canUseTool`-Callback in Ihren Abfrageoptionen übergeben haben, wird er ausgelöst, wenn Claude ein Tool verwenden möchte, das nicht automatisch genehmigt ist. Ihr Callback erhält drei Argumente:

| Argument                            | Beschreibung                                                                                                                                                                                                                                                                                                                                                                          |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `toolName`                          | Der Name des Tools, das Claude verwenden möchte (z. B. `"Bash"`, `"Write"`, `"Edit"`)                                                                                                                                                                                                                                                                                                 |
| `input`                             | Die Parameter, die Claude an das Tool übergibt. Der Inhalt variiert je nach Tool.                                                                                                                                                                                                                                                                                                     |
| `options` (TS) / `context` (Python) | Zusätzlicher Kontext, einschließlich optionaler `suggestions` (vorgeschlagene `PermissionUpdate`-Einträge, um erneute Aufforderungen zu vermeiden) und eines Abbruchsignals. In TypeScript ist `signal` ein `AbortSignal`; in Python ist das Signalfeld für zukünftige Verwendung reserviert. Siehe [`ToolPermissionContext`](/docs/de/agent-sdk/python#toolpermissioncontext) für Python. |

Das `input`-Objekt enthält Tool-spezifische Parameter. Häufige Beispiele:

| Tool    | Eingabefelder                           |
| ------- | --------------------------------------- |
| `Bash`  | `command`, `description`, `timeout`     |
| `Write` | `file_path`, `content`                  |
| `Edit`  | `file_path`, `old_string`, `new_string` |
| `Read`  | `file_path`, `offset`, `limit`          |

Siehe die SDK-Referenz für vollständige Eingabeschemas: [Python](/docs/de/agent-sdk/python#tool-input%2Foutput-types) | [TypeScript](/docs/de/agent-sdk/typescript#tool-input-types).

Sie können diese Informationen dem Benutzer anzeigen, damit er entscheiden kann, ob er die Aktion zulässt oder ablehnt, und dann die entsprechende Antwort zurückgeben.

Das folgende Beispiel fordert Claude auf, eine Testdatei zu erstellen und zu löschen. Wenn Claude jeden Vorgang versucht, druckt der Callback die Tool-Anfrage auf dem Terminal aus und fordert zur y/n-Genehmigung auf.

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query
  from claude_agent_sdk.types import (
      HookMatcher,
      PermissionResultAllow,
      PermissionResultDeny,
      ToolPermissionContext,
  )


  async def can_use_tool(
      tool_name: str, input_data: dict, context: ToolPermissionContext
  ) -> PermissionResultAllow | PermissionResultDeny:
      # Tool-Anfrage anzeigen
      print(f"\nTool: {tool_name}")
      if tool_name == "Bash":
          print(f"Command: {input_data.get('command')}")
          if input_data.get("description"):
              print(f"Description: {input_data.get('description')}")
      else:
          print(f"Input: {input_data}")

      # Benutzergenehmigung abrufen
      response = input("Allow this action? (y/n): ")

      # Zulassung oder Ablehnung basierend auf der Antwort des Benutzers zurückgeben
      if response.lower() == "y":
          # Zulassen: Tool wird mit der ursprünglichen (oder geänderten) Eingabe ausgeführt
          return PermissionResultAllow(updated_input=input_data)
      else:
          # Ablehnen: Tool wird nicht ausgeführt, Claude sieht die Nachricht
          return PermissionResultDeny(message="User denied this action")


  # Erforderliche Umgehung: Dummy-Hook hält den Stream für can_use_tool offen
  async def dummy_hook(input_data, tool_use_id, context):
      return {"continue_": True}


  async def prompt_stream():
      yield {
          "type": "user",
          "message": {
              "role": "user",
              "content": "Create a test file in /tmp and then delete it",
          },
      }


  async def main():
      async for message in query(
          prompt=prompt_stream(),
          options=ClaudeAgentOptions(
              can_use_tool=can_use_tool,
              hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
          ),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";
  import * as readline from "readline";

  // Hilfsfunktion zum Auffordern von Benutzereingaben im Terminal
  function prompt(question: string): Promise<string> {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    });
    return new Promise((resolve) =>
      rl.question(question, (answer) => {
        rl.close();
        resolve(answer);
      })
    );
  }

  for await (const message of query({
    prompt: "Create a test file in /tmp and then delete it",
    options: {
      canUseTool: async (toolName, input) => {
        // Tool-Anfrage anzeigen
        console.log(`\nTool: ${toolName}`);
        if (toolName === "Bash") {
          console.log(`Command: ${input.command}`);
          if (input.description) console.log(`Description: ${input.description}`);
        } else {
          console.log(`Input: ${JSON.stringify(input, null, 2)}`);
        }

        // Benutzergenehmigung abrufen
        const response = await prompt("Allow this action? (y/n): ");

        // Zulassung oder Ablehnung basierend auf der Antwort des Benutzers zurückgeben
        if (response.toLowerCase() === "y") {
          // Zulassen: Tool wird mit der ursprünglichen (oder geänderten) Eingabe ausgeführt
          return { behavior: "allow", updatedInput: input };
        } else {
          // Ablehnen: Tool wird nicht ausgeführt, Claude sieht die Nachricht
          return { behavior: "deny", message: "User denied this action" };
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
  ```
</CodeGroup>

<Note>
  In Python erfordert `can_use_tool` den [Streaming-Modus](/docs/de/agent-sdk/streaming-vs-single-mode). Wenn Sie einen endlichen Nachrichtenstrom durch `query(prompt=generator)` oder `ClaudeSDKClient.connect(prompt=async_iterable)` übergeben, schließt das SDK den Eingabestrom nach der letzten Nachricht, bevor der Berechtigungscallback aufgerufen werden kann, es sei denn, ein registrierter Hook oder ein In-Process-MCP-Server hält ihn offen. Das obige Beispiel hält ihn mit einem `PreToolUse`-Hook offen, der `{"continue_": True}` zurückgibt. Das Verbinden ohne Eingabeaufforderung und das Senden von Nachrichten über `ClaudeSDKClient.query()` hält den Stream von selbst offen und benötigt keinen Hook.
</Note>

Dieses Beispiel verwendet einen `y/n`-Ablauf, bei dem jede Eingabe außer `y` als Ablehnung behandelt wird. In der Praxis könnten Sie eine umfangreichere Benutzeroberfläche erstellen, die es Benutzern ermöglicht, die Anfrage zu ändern, Feedback zu geben oder Claude vollständig umzuleiten. Siehe [Auf Tool-Anfragen reagieren](#respond-to-tool-requests) für alle Möglichkeiten, wie Sie reagieren können.

<h3 id="respond-to-tool-requests">
  Auf Tool-Anfragen reagieren
</h3>

Ihr Callback gibt einen von zwei Antworttypen zurück:

| Antwort      | Python                                     | TypeScript                            |
| ------------ | ------------------------------------------ | ------------------------------------- |
| **Zulassen** | `PermissionResultAllow(updated_input=...)` | `{ behavior: "allow", updatedInput }` |
| **Ablehnen** | `PermissionResultDeny(message=...)`        | `{ behavior: "deny", message }`       |

Beim Zulassen wird das Tool mit der von Claude angeforderten Eingabe ausgeführt, es sei denn, Sie geben eine geänderte Eingabe zurück, `updatedInput` in TypeScript oder `updated_input` in Python. Vor v2.1.207 lehnte Claude Code ein Zulassungsergebnis ab, das `updatedInput` auslies, und lehnte den Tool-Aufruf mit einem Validierungsfehler ab.

Beim Ablehnen geben Sie eine Nachricht an, die erklärt, warum. Claude sieht diese Nachricht und kann seinen Ansatz anpassen.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk.types import PermissionResultAllow, PermissionResultDeny

  # Tool-Ausführung zulassen
  return PermissionResultAllow(updated_input=input_data)

  # Tool blockieren
  return PermissionResultDeny(message="User rejected this action")
  ```

  ```typescript TypeScript theme={null}
  // Tool-Ausführung zulassen
  return { behavior: "allow", updatedInput: input };

  // Tool blockieren
  return { behavior: "deny", message: "User rejected this action" };
  ```
</CodeGroup>

Über das Zulassen oder Ablehnen hinaus können Sie die Eingabe des Tools ändern oder Kontext bereitstellen, der Claude hilft, seinen Ansatz anzupassen:

* **Genehmigen**: Lassen Sie das Tool genau wie von Claude angefordert ausführen
* **Mit Änderungen genehmigen**: Ändern Sie die Eingabe vor der Ausführung (z. B. Pfade bereinigen, Einschränkungen hinzufügen)
* **Genehmigen und merken**: Geben Sie eine vorgeschlagene Berechtigungsregel zurück, damit übereinstimmende Aufrufe das nächste Mal die Aufforderung überspringen
* **Ablehnen**: Blockieren Sie das Tool und teilen Sie Claude mit, warum
* **Alternative vorschlagen**: Blockieren Sie, aber leiten Sie Claude zu dem hin, was der Benutzer stattdessen möchte
* **Vollständig umleiten**: Verwenden Sie [Streaming-Eingabe](/docs/de/agent-sdk/streaming-vs-single-mode), um Claude eine völlig neue Anweisung zu senden

<Tabs>
  <Tab title="Genehmigen">
    Der Benutzer genehmigt die Aktion unverändert. Geben Sie die `input` aus Ihrem Callback unverändert durch und das Tool wird genau wie von Claude angefordert ausgeführt.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          print(f"Claude wants to use {tool_name}")
          approved = await ask_user("Allow this action?")

          if approved:
              return PermissionResultAllow(updated_input=input_data)
          return PermissionResultDeny(message="User declined")
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        console.log(`Claude wants to use ${toolName}`);
        const approved = await askUser("Allow this action?");

        if (approved) {
          return { behavior: "allow", updatedInput: input };
        }
        return { behavior: "deny", message: "User declined" };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Mit Änderungen genehmigen">
    Der Benutzer genehmigt, möchte aber die Anfrage zuerst ändern. Sie können die Eingabe vor der Tool-Ausführung ändern. Claude sieht das Ergebnis, wird aber nicht darüber informiert, dass Sie etwas geändert haben. Nützlich zum Bereinigen von Parametern, zum Hinzufügen von Einschränkungen oder zum Einschränken des Zugriffs.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          if tool_name == "Bash":
              # Benutzer genehmigt, aber alle Befehle auf Sandbox beschränken
              sandboxed_input = {**input_data}
              sandboxed_input["command"] = input_data["command"].replace(
                  "/tmp", "/tmp/sandbox"
              )
              return PermissionResultAllow(updated_input=sandboxed_input)
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "Bash") {
          // Benutzer genehmigt, aber alle Befehle auf Sandbox beschränken
          const sandboxedInput = {
            ...input,
            command: input.command.replace("/tmp", "/tmp/sandbox")
          };
          return { behavior: "allow", updatedInput: sandboxedInput };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Genehmigen und merken">
    Der Benutzer genehmigt und möchte nicht erneut gefragt werden für diese Art von Aufruf. Das dritte Callback-Argument enthält `suggestions`, ein Array von vorgefertigten [`PermissionUpdate`](/docs/de/agent-sdk/typescript#permissionupdate)-Einträgen. Geben Sie einen in `updatedPermissions` zurück, um ihn anzuwenden. Ein Vorschlag mit dem `localSettings`-Ziel schreibt die Regel in `.claude/settings.local.json`, damit zukünftige Sitzungen die Aufforderung für übereinstimmende Aufrufe überspringen.

    Das Python-Beispiel erfordert `claude-agent-sdk` 0.1.80 oder später.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          choice = await ask_user(f"Allow {tool_name}?", ["once", "always", "no"])

          if choice == "always":
              persist = [
                  s for s in context.suggestions if s.destination == "localSettings"
              ]
              return PermissionResultAllow(
                  updated_input=input_data, updated_permissions=persist
              )
          if choice == "once":
              return PermissionResultAllow(updated_input=input_data)
          return PermissionResultDeny(message="User declined")
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input, { suggestions = [] }) => {
        const choice = await askUser(`Allow ${toolName}?`, ["once", "always", "no"]);

        if (choice === "always") {
          const persist = suggestions.filter(
            (s) => s.destination === "localSettings"
          );
          return {
            behavior: "allow",
            updatedInput: input,
            updatedPermissions: persist
          };
        }
        if (choice === "once") {
          return { behavior: "allow", updatedInput: input };
        }
        return { behavior: "deny", message: "User declined" };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Ablehnen">
    Der Benutzer möchte nicht, dass diese Aktion stattfindet. Blockieren Sie das Tool und geben Sie eine Nachricht an, die erklärt, warum. Claude sieht diese Nachricht und kann einen anderen Ansatz versuchen.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          approved = await ask_user(f"Allow {tool_name}?")

          if not approved:
              return PermissionResultDeny(message="User rejected this action")
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        const approved = await askUser(`Allow ${toolName}?`);

        if (!approved) {
          return {
            behavior: "deny",
            message: "User rejected this action"
          };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Alternative vorschlagen">
    Der Benutzer möchte diese spezifische Aktion nicht, hat aber eine andere Idee. Blockieren Sie das Tool und fügen Sie Anleitung in Ihre Nachricht ein. Claude wird dies lesen und basierend auf Ihrem Feedback entscheiden, wie er vorgehen soll.

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name, input_data, context):
          if tool_name == "Bash" and "rm" in input_data.get("command", ""):
              # Benutzer möchte nicht löschen, schlagen Sie stattdessen Archivierung vor
              return PermissionResultDeny(
                  message="User doesn't want to delete files. They asked if you could compress them into an archive instead."
              )
          return PermissionResultAllow(updated_input=input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "Bash" && input.command.includes("rm")) {
          // Benutzer möchte nicht löschen, schlagen Sie stattdessen Archivierung vor
          return {
            behavior: "deny",
            message:
              "User doesn't want to delete files. They asked if you could compress them into an archive instead."
          };
        }
        return { behavior: "allow", updatedInput: input };
      };
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Vollständig umleiten">
    Für eine vollständige Richtungsänderung (nicht nur einen Anstoß) verwenden Sie [Streaming-Eingabe](/docs/de/agent-sdk/streaming-vs-single-mode), um Claude eine neue Anweisung direkt zu senden. Dies umgeht die aktuelle Tool-Anfrage und gibt Claude völlig neue Anweisungen zum Befolgen.
  </Tab>
</Tabs>

<h2 id="handle-clarifying-questions">
  Klärungsfragen verarbeiten
</h2>

Wenn Claude mehr Anleitung zu einer Aufgabe mit mehreren gültigen Ansätzen benötigt, ruft es das `AskUserQuestion`-Tool auf. Dies löst Ihren `canUseTool`-Callback mit `toolName` auf `AskUserQuestion` aus. Die Eingabe enthält Claudes Fragen als Multiple-Choice-Optionen, die Sie dem Benutzer anzeigen und deren Auswahl zurückgeben.

<Tip>
  Klärungsfragen sind besonders häufig im [`plan`-Modus](/docs/de/agent-sdk/permissions#plan-mode-plan), in dem Claude die Codebasis erkundet und Fragen stellt, bevor er einen Plan vorschlägt. Dies macht den Plan-Modus ideal für interaktive Workflows, bei denen Claude Anforderungen sammeln soll, bevor Änderungen vorgenommen werden.
</Tip>

Die folgenden Schritte zeigen, wie Sie Klärungsfragen verarbeiten:

<Steps>
  <Step title="Übergeben Sie einen canUseTool-Callback">
    Übergeben Sie einen `canUseTool`-Callback in Ihren Abfrageoptionen. Standardmäßig ist `AskUserQuestion` verfügbar. Wenn Sie ein `tools`-Array angeben, um Claudes Funktionen einzuschränken (z. B. einen schreibgeschützten Agent mit nur `Read`, `Glob` und `Grep`), fügen Sie `AskUserQuestion` in dieses Array ein. Andernfalls kann Claude keine Klärungsfragen stellen:

    <CodeGroup>
      ```python Python theme={null}
      async for message in query(
          prompt="Analyze this codebase",
          options=ClaudeAgentOptions(
              # Fügen Sie AskUserQuestion in Ihre Tools-Liste ein
              tools=["Read", "Glob", "Grep", "AskUserQuestion"],
              can_use_tool=can_use_tool,
          ),
      ):
          print(message)
      ```

      ```typescript TypeScript theme={null}
      for await (const message of query({
        prompt: "Analyze this codebase",
        options: {
          // Fügen Sie AskUserQuestion in Ihre Tools-Liste ein
          tools: ["Read", "Glob", "Grep", "AskUserQuestion"],
          canUseTool: async (toolName, input) => {
            // Klärungsfragen hier verarbeiten
          }
        }
      })) {
        console.log(message);
      }
      ```
    </CodeGroup>
  </Step>

  <Step title="Erkennen Sie AskUserQuestion">
    Überprüfen Sie in Ihrem Callback, ob `toolName` gleich `AskUserQuestion` ist, um es anders als andere Tools zu behandeln:

    <CodeGroup>
      ```python Python theme={null}
      async def can_use_tool(tool_name: str, input_data: dict, context):
          if tool_name == "AskUserQuestion":
              # Ihre Implementierung zum Sammeln von Antworten vom Benutzer
              return await handle_clarifying_questions(input_data)
          # Andere Tools normal verarbeiten
          return await prompt_for_approval(tool_name, input_data)
      ```

      ```typescript TypeScript theme={null}
      canUseTool: async (toolName, input) => {
        if (toolName === "AskUserQuestion") {
          // Ihre Implementierung zum Sammeln von Antworten vom Benutzer
          return handleClarifyingQuestions(input);
        }
        // Andere Tools normal verarbeiten
        return promptForApproval(toolName, input);
      };
      ```
    </CodeGroup>
  </Step>

  <Step title="Analysieren Sie die Frageneingabe">
    Die Eingabe enthält Claudes Fragen in einem `questions`-Array. Jede Frage hat eine `question` (der anzuzeigende Text), `options` (die Auswahlmöglichkeiten) und `multiSelect` (ob mehrere Auswahlen zulässig sind):

    ```json theme={null}
    {
      "questions": [
        {
          "question": "How should I format the output?",
          "header": "Format",
          "options": [
            { "label": "Summary", "description": "Brief overview" },
            { "label": "Detailed", "description": "Full explanation" }
          ],
          "multiSelect": false
        },
        {
          "question": "Which sections should I include?",
          "header": "Sections",
          "options": [
            { "label": "Introduction", "description": "Opening context" },
            { "label": "Conclusion", "description": "Final summary" }
          ],
          "multiSelect": true
        }
      ]
    }
    ```

    Siehe [Frageformat](#question-format) für vollständige Feldbeschreibungen.
  </Step>

  <Step title="Sammeln Sie Antworten vom Benutzer">
    Präsentieren Sie die Fragen dem Benutzer und sammeln Sie deren Auswahl. Wie Sie dies tun, hängt von Ihrer Anwendung ab: ein Terminal-Prompt, ein Web-Formular, ein mobiler Dialog usw.
  </Step>

  <Step title="Geben Sie Antworten an Claude zurück">
    Erstellen Sie das `answers`-Objekt als Datensatz, wobei jeder Schlüssel der `question`-Text ist und jeder Wert das `label` der ausgewählten Option ist:

    | Aus dem Frageobjekt                                         | Verwenden Sie als |
    | ----------------------------------------------------------- | ----------------- |
    | `question`-Feld (z. B. `"How should I format the output?"`) | Schlüssel         |
    | `label`-Feld der ausgewählten Option (z. B. `"Summary"`)    | Wert              |

    Für Multi-Select-Fragen übergeben Sie ein Array von Labels oder verbinden Sie sie mit `", "`. Wenn Sie [freie Texteingabe unterstützen](#support-free-text-input), verwenden Sie den benutzerdefinierten Text des Benutzers als Wert.

    <CodeGroup>
      ```python Python theme={null}
      return PermissionResultAllow(
          updated_input={
              "questions": input_data.get("questions", []),
              "answers": {
                  "How should I format the output?": "Summary",
                  "Which sections should I include?": ["Introduction", "Conclusion"],
              },
          }
      )
      ```

      ```typescript TypeScript theme={null}
      return {
        behavior: "allow",
        updatedInput: {
          questions: input.questions,
          answers: {
            "How should I format the output?": "Summary",
            "Which sections should I include?": "Introduction, Conclusion"
          }
        }
      };
      ```
    </CodeGroup>
  </Step>
</Steps>

<h3 id="question-format">
  Frageformat
</h3>

Die Eingabe enthält Claudes generierte Fragen in einem `questions`-Array. Jede Frage hat diese Felder:

| Feld          | Beschreibung                                                                                                                                           |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `question`    | Der vollständige Fragetext zum Anzeigen                                                                                                                |
| `header`      | Kurzes Label für die Frage (max. 12 Zeichen)                                                                                                           |
| `options`     | Array von 2-4 Auswahlmöglichkeiten, jeweils mit `label` und `description`. TypeScript: optional `preview` (siehe [unten](#option-previews-typescript)) |
| `multiSelect` | Wenn `true`, können Benutzer mehrere Optionen auswählen                                                                                                |

Die Struktur, die Ihr Callback erhält:

```json theme={null}
{
  "questions": [
    {
      "question": "How should I format the output?",
      "header": "Format",
      "options": [
        { "label": "Summary", "description": "Brief overview of key points" },
        { "label": "Detailed", "description": "Full explanation with examples" }
      ],
      "multiSelect": false
    }
  ]
}
```

<h4 id="option-previews-typescript">
  Optionsvorschau (TypeScript)
</h4>

`toolConfig.askUserQuestion.previewFormat` fügt jedem Option ein `preview`-Feld hinzu, damit Ihre App ein visuelles Mockup neben dem Label anzeigen kann. Ohne diese Einstellung generiert Claude keine Vorschau und das Feld ist nicht vorhanden.

| `previewFormat`          | `preview` enthält                                                                                                            |
| :----------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| nicht gesetzt (Standard) | Feld ist nicht vorhanden. Claude generiert keine Vorschau.                                                                   |
| `"markdown"`             | ASCII-Art und eingezäunte Code-Blöcke                                                                                        |
| `"html"`                 | Ein gestyltes `<div>`-Fragment (das SDK lehnt `<script>`, `<style>` und `<!DOCTYPE>` ab, bevor Ihr Callback ausgeführt wird) |

Das Format gilt für alle Fragen in der Sitzung. Claude fügt `preview` bei Optionen ein, bei denen ein visueller Vergleich hilfreich ist (Layout-Auswahlmöglichkeiten, Farbschemas) und lässt es weg, wo nicht (Ja/Nein-Bestätigungen, nur Text-Auswahlmöglichkeiten). Überprüfen Sie auf `undefined`, bevor Sie rendern.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Help me choose a card layout",
  options: {
    toolConfig: {
      askUserQuestion: { previewFormat: "html" }
    },
    canUseTool: async (toolName, input) => {
      // input.questions[].options[].preview ist ein HTML-String oder undefined
      return { behavior: "allow", updatedInput: input };
    }
  }
})) {
  // ...
}
```

Eine Option mit HTML-Vorschau:

```json theme={null}
{
  "label": "Compact",
  "description": "Title and metric value only",
  "preview": "<div style=\"padding:12px;border:1px solid #ddd;border-radius:8px\"><div style=\"font-size:12px;color:#666\">Active users</div><div style=\"font-size:28px;font-weight:600\">1,284</div></div>"
}
```

<h3 id="response-format">
  Antwortformat
</h3>

Geben Sie ein `answers`-Objekt zurück, das jedes `question`-Feld der Frage dem `label` der ausgewählten Option zuordnet:

| Feld        | Beschreibung                                                                                               |
| ----------- | ---------------------------------------------------------------------------------------------------------- |
| `questions` | Geben Sie das ursprüngliche Questions-Array durch (erforderlich für die Tool-Verarbeitung)                 |
| `answers`   | Objekt, bei dem Schlüssel Fragetext und Werte ausgewählte Labels sind                                      |
| `response`  | Optionale freie Antwort, die der Benutzer eingegeben hat, anstatt die strukturierten Fragen zu beantworten |

Für Multi-Select-Fragen übergeben Sie ein Array von Labels oder verbinden Sie sie mit `", "`. Für benutzerdefinierte freie Texteingaben wie eine „Other"-Option geben Sie den Text des Benutzers in `answers[question]` ein, wie in [Freie Texteingabe unterstützen](#support-free-text-input) gezeigt. Setzen Sie `response` nur, wenn Ihre Benutzeroberfläche dem Benutzer ermöglicht, die Fragenkarte zu schließen und eine allgemeine Antwort einzugeben, die keine Antwort auf eine bestimmte Frage ist. Wenn `response` gesetzt ist, erhält Claude „Der Benutzer hat geantwortet: …" anstelle der Liste der Antworten pro Frage.

```json theme={null}
{
  "questions": [
    // ...
  ],
  "answers": {
    "How should I format the output?": "Summary",
    "Which sections should I include?": ["Introduction", "Conclusion"]
  }
}
```

<h4 id="support-free-text-input">
  Unterstützen Sie freie Texteingabe
</h4>

Claudes vordefinierte Optionen decken nicht immer ab, was Benutzer möchten. Um Benutzern zu ermöglichen, ihre eigene Antwort einzugeben:

* Zeigen Sie nach Claudes Optionen eine zusätzliche „Other"-Auswahlmöglichkeit an, die Texteingabe akzeptiert
* Verwenden Sie den benutzerdefinierten Text des Benutzers als Antwortwert (nicht das Wort „Other")

Siehe das [vollständige Beispiel](#complete-example) unten für eine vollständige Implementierung.

<h3 id="complete-example">
  Vollständiges Beispiel
</h3>

Claude stellt Klärungsfragen, wenn er Benutzereingaben benötigt, um fortzufahren. Wenn Claude beispielsweise aufgefordert wird, bei der Entscheidung über einen Tech-Stack für eine mobile App zu helfen, könnte Claude Fragen zu Cross-Platform vs. Native, Backend-Vorlieben oder Zielplattformen stellen. Diese Fragen helfen Claude, Entscheidungen zu treffen, die den Vorlieben des Benutzers entsprechen, anstatt zu raten.

Dieses Beispiel verarbeitet diese Fragen in einer Terminal-Anwendung. Hier ist, was bei jedem Schritt passiert:

1. **Leiten Sie die Anfrage weiter**: Der `canUseTool`-Callback überprüft, ob der Tool-Name `"AskUserQuestion"` ist, und leitet zu einem dedizierten Handler weiter
2. **Zeigen Sie Fragen an**: Der Handler durchläuft das `questions`-Array und druckt jede Frage mit nummerierten Optionen
3. **Sammeln Sie Eingaben**: Der Benutzer kann eine Nummer eingeben, um eine Option auszuwählen, oder direkt freien Text eingeben (z. B. „jquery", „i don't know")
4. **Ordnen Sie Antworten zu**: Der Code überprüft, ob die Eingabe numerisch ist (verwendet das Label der Option) oder freier Text (verwendet den Text direkt)
5. **Geben Sie an Claude zurück**: Die Antwort enthält sowohl das ursprüngliche `questions`-Array als auch die `answers`-Zuordnung

Speichern Sie die TypeScript-Version als `ask.ts` und führen Sie sie mit `npx tsx ask.ts` aus, oder speichern Sie die Python-Version als `ask.py` und führen Sie sie mit `python ask.py` aus.

<CodeGroup>
  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query
  from claude_agent_sdk.types import HookMatcher, PermissionResultAllow


  def parse_response(response: str, options: list) -> str:
      """Analysieren Sie Benutzereingaben als Optionsnummer(n) oder freien Text."""
      try:
          indices = [int(s.strip()) - 1 for s in response.split(",")]
          labels = [options[i]["label"] for i in indices if 0 <= i < len(options)]
          return ", ".join(labels) if labels else response
      except ValueError:
          return response


  async def handle_ask_user_question(input_data: dict) -> PermissionResultAllow:
      """Zeigen Sie Claudes Fragen an und sammeln Sie Benutzerantworten."""
      answers = {}

      for q in input_data.get("questions", []):
          print(f"\n{q['header']}: {q['question']}")

          options = q["options"]
          for i, opt in enumerate(options):
              print(f"  {i + 1}. {opt['label']} - {opt['description']}")
          if q.get("multiSelect"):
              print("  (Enter numbers separated by commas, or type your own answer)")
          else:
              print("  (Enter a number, or type your own answer)")

          response = input("Your choice: ").strip()
          answers[q["question"]] = parse_response(response, options)

      return PermissionResultAllow(
          updated_input={
              "questions": input_data.get("questions", []),
              "answers": answers,
          }
      )


  async def can_use_tool(
      tool_name: str, input_data: dict, context
  ) -> PermissionResultAllow:
      # Leiten Sie AskUserQuestion zu unserem Frage-Handler weiter
      if tool_name == "AskUserQuestion":
          return await handle_ask_user_question(input_data)
      # Auto-Genehmigung anderer Tools für dieses Beispiel
      return PermissionResultAllow(updated_input=input_data)


  async def prompt_stream():
      yield {
          "type": "user",
          "message": {
              "role": "user",
              "content": "Help me decide on the tech stack for a new mobile app",
          },
      }


  # Erforderliche Umgehung: Dummy-Hook hält den Stream für can_use_tool offen
  async def dummy_hook(input_data, tool_use_id, context):
      return {"continue_": True}


  async def main():
      async for message in query(
          prompt=prompt_stream(),
          options=ClaudeAgentOptions(
              can_use_tool=can_use_tool,
              hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
          ),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";
  import * as readline from "readline/promises";

  // Hilfsfunktion zum Auffordern von Benutzereingaben im Terminal
  async function prompt(question: string): Promise<string> {
    const rl = readline.createInterface({ input: process.stdin, output: process.stdout });
    const answer = await rl.question(question);
    rl.close();
    return answer;
  }

  // Analysieren Sie Benutzereingaben als Optionsnummer(n) oder freien Text
  function parseResponse(response: string, options: any[]): string {
    const indices = response.split(",").map((s) => parseInt(s.trim()) - 1);
    const labels = indices
      .filter((i) => !isNaN(i) && i >= 0 && i < options.length)
      .map((i) => options[i].label);
    return labels.length > 0 ? labels.join(", ") : response;
  }

  // Zeigen Sie Claudes Fragen an und sammeln Sie Benutzerantworten
  async function handleAskUserQuestion(input: any) {
    const answers: Record<string, string> = {};

    for (const q of input.questions) {
      console.log(`\n${q.header}: ${q.question}`);

      const options = q.options;
      options.forEach((opt: any, i: number) => {
        console.log(`  ${i + 1}. ${opt.label} - ${opt.description}`);
      });
      if (q.multiSelect) {
        console.log("  (Enter numbers separated by commas, or type your own answer)");
      } else {
        console.log("  (Enter a number, or type your own answer)");
      }

      const response = (await prompt("Your choice: ")).trim();
      answers[q.question] = parseResponse(response, options);
    }

    // Geben Sie die Antworten an Claude zurück (muss ursprüngliche Fragen enthalten)
    return {
      behavior: "allow",
      updatedInput: { questions: input.questions, answers }
    };
  }

  async function main() {
    for await (const message of query({
      prompt: "Help me decide on the tech stack for a new mobile app",
      options: {
        canUseTool: async (toolName, input) => {
          // Leiten Sie AskUserQuestion zu unserem Frage-Handler weiter
          if (toolName === "AskUserQuestion") {
            return handleAskUserQuestion(input);
          }
          // Auto-Genehmigung anderer Tools für dieses Beispiel
          return { behavior: "allow", updatedInput: input };
        }
      }
    })) {
      if ("result" in message) console.log(message.result);
    }
  }

  main();
  ```
</CodeGroup>

<h2 id="limitations">
  Einschränkungen
</h2>

* **Subagenten**: `AskUserQuestion` ist derzeit nicht in Subagenten verfügbar, die über das Agent-Tool erzeugt werden
* **Fragenlimits**: Jeder `AskUserQuestion`-Aufruf unterstützt 1-4 Fragen mit jeweils 2-4 Optionen

<h2 id="other-ways-to-get-user-input">
  Andere Möglichkeiten, Benutzereingaben zu erhalten
</h2>

Der `canUseTool`-Callback und das `AskUserQuestion`-Tool decken die meisten Genehmigungs- und Klärungsszenarien ab, aber das SDK bietet andere Möglichkeiten, Eingaben von Benutzern zu erhalten:

<h3 id="streaming-input">
  Streaming-Eingabe
</h3>

Verwenden Sie [Streaming-Eingabe](/docs/de/agent-sdk/streaming-vs-single-mode), wenn Sie:

* **Den Agent mitten in der Aufgabe unterbrechen**: Senden Sie ein Abbruchsignal oder ändern Sie die Richtung, während Claude arbeitet
* **Zusätzlichen Kontext bereitstellen**: Fügen Sie Informationen hinzu, die Claude benötigt, ohne darauf zu warten, dass es fragt
* **Chat-Schnittstellen erstellen**: Lassen Sie Benutzer Folgenachrichten während langwieriger Operationen senden

Streaming-Eingabe ist ideal für Konversations-UIs, bei denen Benutzer während der Ausführung mit dem Agent interagieren, nicht nur bei Genehmigungsprüfpunkten.

<h3 id="custom-tools">
  Benutzerdefinierte Tools
</h3>

Verwenden Sie [benutzerdefinierte Tools](/docs/de/agent-sdk/custom-tools), wenn Sie:

* **Strukturierte Eingaben sammeln**: Erstellen Sie Formulare, Assistenten oder mehrstufige Workflows, die über das Multiple-Choice-Format von `AskUserQuestion` hinausgehen
* **Externe Genehmigungssysteme integrieren**: Verbinden Sie sich mit bestehenden Ticketing-, Workflow- oder Genehmigungsplattformen
* **Domänenspezifische Interaktionen implementieren**: Erstellen Sie Tools, die auf die Anforderungen Ihrer Anwendung zugeschnitten sind, wie Code-Review-Schnittstellen oder Bereitstellungs-Checklisten

Benutzerdefinierte Tools geben Ihnen vollständige Kontrolle über die Interaktion, erfordern aber mehr Implementierungsarbeit als die Verwendung des integrierten `canUseTool`-Callbacks.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Berechtigungen konfigurieren](/docs/de/agent-sdk/permissions): Richten Sie Berechtigungsmodi und -regeln ein
* [Ausführung mit Hooks steuern](/docs/de/agent-sdk/hooks): Führen Sie benutzerdefinierten Code an Schlüsselpunkten im Agent-Lebenszyklus aus
* [TypeScript SDK-Referenz](/docs/de/agent-sdk/typescript#canusetool): Vollständige canUseTool-API-Dokumentation
