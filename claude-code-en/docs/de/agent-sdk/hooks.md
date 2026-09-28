> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agentverhalten mit Hooks abfangen und steuern

> Fangen Sie Agentverhalten an wichtigen Ausführungspunkten mit Hooks ab und passen Sie es an

Hooks sind Callback-Funktionen, die Ihren Code als Reaktion auf Agent-Ereignisse ausführen, z. B. wenn ein Tool aufgerufen wird, eine Sitzung startet oder die Ausführung stoppt. Mit Hooks können Sie:

* **Gefährliche Operationen blockieren**, bevor sie ausgeführt werden, z. B. destruktive Shell-Befehle oder nicht autorisierter Dateizugriff
* **Alle Tool-Aufrufe protokollieren und überprüfen** für Compliance, Debugging oder Analytik
* **Eingaben und Ausgaben transformieren**, um Daten zu bereinigen, Anmeldedaten einzufügen oder Dateipfade umzuleiten
* **Menschliche Genehmigung anfordern** für sensible Aktionen wie Datenbankschreibvorgänge oder API-Aufrufe
* **Sitzungslebenszyklus verfolgen**, um den Status zu verwalten, Ressourcen freizugeben oder Benachrichtigungen zu senden

<h2 id="how-hooks-work">
  Funktionsweise von Hooks
</h2>

<Steps>
  <Step title="Ein Ereignis wird ausgelöst">
    Während der Agent-Ausführung passiert etwas und das SDK löst ein Ereignis aus: Ein Tool wird aufgerufen (`PreToolUse`), ein Tool gibt ein Ergebnis zurück (`PostToolUse`), ein Subagent startet oder stoppt, der Agent ist untätig oder die Ausführung ist beendet. Siehe die [vollständige Liste der Ereignisse](#available-hooks).
  </Step>

  <Step title="Das SDK sammelt registrierte Hooks">
    Das SDK prüft auf Hooks, die für diesen Ereignistyp registriert sind. Dies umfasst Callback-Hooks, die Sie in `options.hooks` übergeben, und Shell-Befehls-Hooks aus Einstellungsdateien, wenn der entsprechende [`settingSources`](/docs/de/agent-sdk/typescript#settingsource) oder [`setting_sources`](/docs/de/agent-sdk/python#settingsource) Eintrag aktiviert ist, was für Standard-`query()`-Optionen der Fall ist.
  </Step>

  <Step title="Matcher filtern, welche Hooks ausgeführt werden">
    Wenn ein Hook ein [`matcher`](#matchers) Muster hat (z. B. `"Write|Edit"`), testet das SDK es gegen das Ziel des Ereignisses (z. B. den Tool-Namen). Hooks ohne Matcher werden für jedes Ereignis dieses Typs ausgeführt.
  </Step>

  <Step title="Callback-Funktionen werden ausgeführt">
    Jede übereinstimmende Hook-[Callback-Funktion](#callback-functions) erhält Eingaben über das, was passiert: den Tool-Namen, seine Argumente, die Sitzungs-ID und andere ereignisspezifische Details.
  </Step>

  <Step title="Ihr Callback gibt eine Entscheidung zurück">
    Nach dem Ausführen von Operationen (Protokollierung, API-Aufrufe, Validierung) gibt Ihr Callback ein [Ausgabeobjekt](#outputs) zurück, das dem Agent mitteilt, was zu tun ist: die Operation zulassen, blockieren, die Eingabe ändern oder Kontext in das Gespräch einfügen.
  </Step>
</Steps>

Das folgende Beispiel bringt diese Schritte zusammen. Es registriert einen `PreToolUse` Hook (Schritt 1) mit einem `"Write|Edit"` Matcher (Schritt 3), sodass der Callback nur für Datei-Schreib-Tools ausgelöst wird. Wenn ausgelöst, erhält der Callback die Eingabe des Tools (Schritt 4), prüft, ob der Dateipfad auf eine `.env`-Datei abzielt, und gibt `permissionDecision: "deny"` zurück, um die Operation zu blockieren (Schritt 5):

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      AssistantMessage,
      ClaudeSDKClient,
      ClaudeAgentOptions,
      HookMatcher,
      ResultMessage,
  )


  # Define a hook callback that receives tool call details
  async def protect_env_files(input_data, tool_use_id, context):
      # Extract the file path from the tool's input arguments
      file_path = input_data["tool_input"].get("file_path", "")
      file_name = file_path.split("/")[-1]

      # Block the operation if targeting a .env file
      if file_name == ".env":
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Cannot modify .env files",
              }
          }

      # Return empty object to allow the operation
      return {}


  async def main():
      options = ClaudeAgentOptions(
          hooks={
              # Register the hook for PreToolUse events
              # The matcher filters to only Write and Edit tool calls
              "PreToolUse": [HookMatcher(matcher="Write|Edit", hooks=[protect_env_files])]
          }
      )

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Create a .env file with the standard local development database configuration")
          async for message in client.receive_response():
              # Filter for assistant and result messages
              if isinstance(message, (AssistantMessage, ResultMessage)):
                  print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, PreToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

  // Define a hook callback with the HookCallback type
  const protectEnvFiles: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast input to the specific hook type for type safety
    const preInput = input as PreToolUseHookInput;

    // Cast tool_input to access its properties (typed as unknown in the SDK)
    const toolInput = preInput.tool_input as Record<string, unknown>;
    const filePath = toolInput?.file_path as string;
    const fileName = filePath?.split("/").pop();

    // Block the operation if targeting a .env file
    if (fileName === ".env") {
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "deny",
          permissionDecisionReason: "Cannot modify .env files"
        }
      };
    }

    // Return empty object to allow the operation
    return {};
  };

  for await (const message of query({
    prompt: "Create a .env file with the standard local development database configuration",
    options: {
      hooks: {
        // Register the hook for PreToolUse events
        // The matcher filters to only Write and Edit tool calls
        PreToolUse: [{ matcher: "Write|Edit", hooks: [protectEnvFiles] }]
      }
    }
  })) {
    // Filter for assistant and result messages
    if (message.type === "assistant" || message.type === "result") {
      console.log(message);
    }
  }
  ```
</CodeGroup>

Wenn Sie eines der Skripte ausführen, versucht Claude, die `.env`-Datei zu erstellen, der Hook verweigert den Tool-Aufruf, und Claudes endgültige Antwort erklärt, dass es keine `.env`-Dateien erstellen kann.

<h2 id="available-hooks">
  Verfügbare Hooks
</h2>

Das SDK bietet Hooks für verschiedene Phasen der Agent-Ausführung. Einige Hooks sind in beiden SDKs verfügbar, während andere nur für TypeScript verfügbar sind.

| Hook-Ereignis                                          | Python SDK | TypeScript SDK | Was löst es aus                                                                                                                                                                 | Beispiel-Anwendungsfall                                                                                                                                                                                                         |
| ------------------------------------------------------ | ---------- | -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`                                           | Ja         | Ja             | Tool-Aufrufanforderung (kann blockiert oder geändert werden)                                                                                                                    | Gefährliche Shell-Befehle blockieren                                                                                                                                                                                            |
| `PostToolUse`                                          | Ja         | Ja             | Tool-Ausführungsergebnis                                                                                                                                                        | Alle Dateiänderungen im Audit-Trail protokollieren                                                                                                                                                                              |
| `PostToolUseFailure`                                   | Ja         | Ja             | Tool-Ausführungsfehler                                                                                                                                                          | Tool-Fehler behandeln oder protokollieren                                                                                                                                                                                       |
| `PostToolBatch`                                        | Nein       | Ja             | Ein vollständiger Batch von Tool-Aufrufen wird aufgelöst, einmal pro Batch vor dem nächsten Modellaufruf                                                                        | Konventionen einmal für den gesamten Batch einfügen                                                                                                                                                                             |
| `UserPromptSubmit`                                     | Ja         | Ja             | Benutzer-Prompt-Übermittlung                                                                                                                                                    | Zusätzlichen Kontext in Prompts einfügen                                                                                                                                                                                        |
| [`UserPromptExpansion`](/docs/de/hooks#userpromptexpansion) | Nein       | Ja             | Ein von Benutzern eingegebener Befehl oder ein MCP-Prompt wird zu einem Prompt erweitert, bevor er Claude erreicht. Wird nicht ausgelöst, wenn Claude selbst eine Skill aufruft | Einen Befehl von direkter Aufrufe blockieren oder Kontext hinzufügen, wenn eine Skill eingegeben wird                                                                                                                           |
| `MessageDisplay`                                       | Nein       | Ja             | Eine Assistenten-Nachricht mit Text wird abgeschlossen, einmal pro Nachricht mit dem vollständigen Nachrichtentext                                                              | Angezeigten Text redigieren oder neu formatieren, ohne das Transkript zu ändern                                                                                                                                                 |
| `Stop`                                                 | Ja         | Ja             | Agent-Ausführung stoppt                                                                                                                                                         | Sitzungsstatus vor dem Beenden speichern                                                                                                                                                                                        |
| `StopFailure`                                          | Nein       | Ja             | Die Runde endet mit einem API-Fehler statt mit einem normalen Stopp                                                                                                             | Fehler protokollieren oder Benachrichtigungen senden                                                                                                                                                                            |
| `SubagentStart`                                        | Ja         | Ja             | Subagent-Initialisierung                                                                                                                                                        | Parallele Task-Spawning verfolgen                                                                                                                                                                                               |
| `SubagentStop`                                         | Ja         | Ja             | Subagent-Fertigstellung                                                                                                                                                         | Ergebnisse aus parallelen Tasks aggregieren                                                                                                                                                                                     |
| `PreCompact`                                           | Ja         | Ja             | Anforderung zur Gesprächskomprimierung                                                                                                                                          | Vollständiges Transkript vor der Zusammenfassung archivieren                                                                                                                                                                    |
| `PostCompact`                                          | Nein       | Ja             | Gesprächskomprimierung ist abgeschlossen                                                                                                                                        | Die generierte Zusammenfassung protokollieren                                                                                                                                                                                   |
| [`PreModelSwitch`](/docs/de/hooks#premodelswitch)           | Nein       | Ja             | Ein angefordeter Modellwechsel, bevor er stattfindet (kann blockiert werden)                                                                                                    | Wechsel zu einem bestimmten Modell blockieren                                                                                                                                                                                   |
| [`PostModelSwitch`](/docs/de/hooks#postmodelswitch)         | Nein       | Ja             | Das Modell der Sitzung ändert sich, einschließlich eines automatischen Fallbacks                                                                                                | Claude modellspezifische Anleitung für das neue Modell geben                                                                                                                                                                    |
| `PermissionRequest`                                    | Ja         | Ja             | Ein Tool-Aufruf benötigt eine Berechtigungsentscheidung                                                                                                                         | Benutzerdefinierte Berechtigungsbehandlung                                                                                                                                                                                      |
| `PermissionDenied`                                     | Nein       | Ja             | Auto-Modus verweigert einen Tool-Aufruf, einschließlich Verweigerungen ohne Klassifizierer-Urteil                                                                               | Verweigerungen protokollieren oder dem Modell mitteilen, dass es möglicherweise erneut versuchen kann; Claude Code ignoriert `retry: true` für Verweigerungen ohne Urteil. Siehe [PermissionDenied](/docs/de/hooks#permissiondenied) |
| `SessionStart`                                         | Nein       | Ja             | Sitzungsinitialisierung                                                                                                                                                         | Protokollierung und Telemetrie initialisieren                                                                                                                                                                                   |
| `SessionEnd`                                           | Nein       | Ja             | Sitzungsbeendigung                                                                                                                                                              | Temporäre Ressourcen bereinigen                                                                                                                                                                                                 |
| `Notification`                                         | Ja         | Ja             | Agent-Statusmeldungen                                                                                                                                                           | Agent-Status-Updates an Slack oder PagerDuty senden                                                                                                                                                                             |
| `Setup`                                                | Nein       | Ja             | Sitzungssetup/Wartung                                                                                                                                                           | Initialisierungsaufgaben ausführen                                                                                                                                                                                              |
| `TeammateIdle`                                         | Nein       | Ja             | Teammate wird untätig                                                                                                                                                           | Arbeit neu zuweisen oder benachrichtigen                                                                                                                                                                                        |
| `TaskCreated`                                          | Nein       | Ja             | Eine Task wird über das `TaskCreate`-Tool erstellt                                                                                                                              | Task-Benennungskonventionen durchsetzen                                                                                                                                                                                         |
| [`TaskCompleted`](/docs/de/hooks#taskcompleted)             | Nein       | Ja             | Eine Task wird als abgeschlossen markiert                                                                                                                                       | Bestandene Tests vor dem Schließen einer Task erforderlich                                                                                                                                                                      |
| `Elicitation`                                          | Nein       | Ja             | Ein MCP-Server fordert Benutzereingaben während einer Task an                                                                                                                   | Auf MCP-Eingabeanforderungen programmgesteuert reagieren                                                                                                                                                                        |
| `ElicitationResult`                                    | Nein       | Ja             | Ein Benutzer antwortet auf eine MCP-Elicitation                                                                                                                                 | Die Antwort ändern oder blockieren, bevor sie an den Server zurückgeht                                                                                                                                                          |
| `ConfigChange`                                         | Nein       | Ja             | Konfigurationsdatei ändert sich                                                                                                                                                 | Einstellungen dynamisch neu laden                                                                                                                                                                                               |
| `InstructionsLoaded`                                   | Nein       | Ja             | Eine `CLAUDE.md`- oder Regeldatei wird in den Kontext geladen                                                                                                                   | Überprüfen, welche Anweisungsdateien geladen werden                                                                                                                                                                             |
| `WorktreeCreate`                                       | Nein       | Ja             | Git Worktree erstellt                                                                                                                                                           | Isolierte Workspaces verfolgen                                                                                                                                                                                                  |
| `WorktreeRemove`                                       | Nein       | Ja             | Git Worktree entfernt                                                                                                                                                           | Workspace-Ressourcen bereinigen                                                                                                                                                                                                 |
| `CwdChanged`                                           | Nein       | Ja             | Das Arbeitsverzeichnis ändert sich während einer Sitzung                                                                                                                        | Umgebungsvariablen pro Verzeichnis neu laden                                                                                                                                                                                    |
| `FileChanged`                                          | Nein       | Ja             | Eine überwachte Datei wird geändert, erstellt oder gelöscht                                                                                                                     | Konfiguration neu laden, wenn sich Projektdateien ändern                                                                                                                                                                        |
| `DirectoryAdded`                                       | Nein       | Ja             | Ein Arbeitsverzeichnis wird während einer Sitzung hinzugefügt                                                                                                                   | Abhängigkeiten für ein während der Sitzung hinzugefügtes Repository installieren                                                                                                                                                |

<h2 id="configure-hooks">
  Hooks konfigurieren
</h2>

Um einen Hook zu konfigurieren, übergeben Sie ihn im `hooks` Feld Ihrer Agent-Optionen (`ClaudeAgentOptions` in Python, das `options` Objekt in TypeScript). Dieser Ausschnitt setzt voraus, dass Sie bereits einen Hook-Callback definiert haben, wie `protect_env_files` in Python oder `protectEnvFiles` in TypeScript aus dem obigen Beispiel:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={"PreToolUse": [HookMatcher(matcher="Bash", hooks=[my_callback])]}
  )

  async with ClaudeSDKClient(options=options) as client:
      await client.query("Your prompt")
      async for message in client.receive_response():
          print(message)
  ```

  ```typescript TypeScript theme={null}
  for await (const message of query({
    prompt: "Your prompt",
    options: {
      hooks: {
        PreToolUse: [{ matcher: "Bash", hooks: [myCallback] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Die `hooks` Option ist ein Wörterbuch in Python oder ein Objekt in TypeScript, wobei:

* **Schlüssel**: [Hook-Ereignisnamen](#available-hooks) wie `'PreToolUse'`, `'PostToolUse'` und `'Stop'`
* **Werte**: Arrays von [Matchern](#matchers), die jeweils ein optionales Filtermuster und Ihre [Callback-Funktionen](#callback-functions) enthalten

<h3 id="matchers">
  Matcher
</h3>

Verwenden Sie Matcher, um zu filtern, wann Ihre Callbacks ausgelöst werden. Das `matcher` Feld wird gegen einen anderen Wert abgeglichen, je nach Hook-Ereignistyp. Beispielsweise werden Tool-basierte Hooks gegen den Tool-Namen abgeglichen, während `Notification` Hooks gegen den Benachrichtigungstyp abgeglichen werden.

SDK-Matcher folgen den gleichen Regeln wie [Matcher in Einstellungsdateien](/docs/de/hooks#matcher-patterns). Dieser Abschnitt dokumentiert die Pfade für exakte Zeichenketten- und reguläre Ausdrucksbewertung, ihre Versionsanforderungen und die Matcher-Werte für jeden Ereignistyp.

| Option    | Typ              | Standard    | Beschreibung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| --------- | ---------------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `matcher` | `string`         | `undefined` | Muster, das gegen das Filterfeld des Ereignisses abgeglichen wird, nach den [Regeln für Matcher in Einstellungsdateien](/docs/de/hooks#matcher-patterns). Für Tool-Hooks ist dies der Tool-Name. Integrierte Tools umfassen `Bash`, `Read`, `Write`, `Edit`, `Glob`, `Grep`, `WebFetch`, `Agent` und andere (siehe [Tool-Eingabetypen](/docs/de/agent-sdk/typescript#tool-input-types) für die vollständige Liste). MCP-Tools verwenden das Muster `mcp__<server>__<action>`, wobei `<server>` der Schlüssel ist, den Sie in der `mcpServers` Konfiguration verwenden. |
| `hooks`   | `HookCallback[]` | -           | Erforderlich. Array von Callback-Funktionen, die ausgeführt werden, wenn das Muster übereinstimmt                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `timeout` | `number`         | `undefined` | Timeout in Sekunden. Wenn weggelassen, wendet Claude Code das [Standard-Timeout des Ereignisses](#hook-timeout) an. Ihre SDK-Callbacks folgen den `command` Hook-Standards                                                                                                                                                                                                                                                                                                                                                                                   |

Verwenden Sie das `matcher` Muster, um nach Möglichkeit spezifische Tools anzusteuern. Ein Matcher mit `'Bash'` wird nur für Bash-Befehle ausgeführt, während das Weglassen des Musters Ihre Callbacks für jedes Vorkommen des Ereignisses ausführt. Lassen Sie es absichtlich weg, um jeden Tool-Aufruf Ihrer Sitzung zu protokollieren.

<h3 id="callback-functions">
  Callback-Funktionen
</h3>

<h4 id="inputs">
  Eingaben
</h4>

Jeder Hook-Callback erhält drei Argumente:

* **Eingabedaten:** ein typisiertes Objekt mit Ereignisdetails. Jeder Hook-Typ hat seine eigene Eingabeform. Beispielsweise enthält `PreToolUseHookInput` `tool_name` und `tool_input`, während `NotificationHookInput` `message` enthält. Siehe die vollständigen Typdefinitionen in den [TypeScript](/docs/de/agent-sdk/typescript#hookinput) und [Python](/docs/de/agent-sdk/python#hookinput) SDK-Referenzen.
  * Alle Hook-Eingaben teilen `session_id`, `cwd` und `hook_event_name`.
  * `agent_id` und `agent_type` werden ausgefüllt, wenn der Hook in einem Subagent ausgelöst wird. In TypeScript befinden sich diese in der Basis-Hook-Eingabe und sind für alle Hook-Typen verfügbar. In Python sind sie optionale Felder auf `PreToolUse`, `PostToolUse`, `PostToolUseFailure` und `PermissionRequest`, und erforderliche Felder auf `SubagentStart` und `SubagentStop`.
* **Tool-Verwendungs-ID** (`str | None` / `string | undefined`): korreliert `PreToolUse` und `PostToolUse` Ereignisse für denselben Tool-Aufruf.
* **Kontext:** In TypeScript enthält eine `signal` Eigenschaft (`AbortSignal`) für Abbruch. In Python ist dieses Argument für zukünftige Verwendung reserviert.

<h4 id="outputs">
  Ausgaben
</h4>

Ihr Callback gibt ein Objekt mit zwei Kategorien von Feldern zurück:

* **Top-Level-Felder** werden bei jedem Ereignis akzeptiert: `systemMessage` zeigt eine Nachricht für den Benutzer an, und `continue` (`continue_` in Python) bestimmt, ob der Agent nach diesem Hook weiterläuft. Einige Ereignisse verwerfen sie oder liefern sie an anderer Stelle. Jeder [Abschnitt des Ereignisses](/docs/de/hooks#hook-events) auf der Hooks-Seite sagt, wo sie landen.
* **`hookSpecificOutput`** steuert die aktuelle Operation. Die Felder, die Sie darin setzen, hängen vom Hook-Ereignistyp ab:
  * Für `PreToolUse` Hooks ist dies der Ort, an dem Sie `permissionDecision` (`"allow"`, `"deny"`, `"ask"` oder `"defer"`), `permissionDecisionReason` und `updatedInput` setzen. Wenn Sie `"defer"` zurückgeben, endet die Abfrage, damit Sie sie [später fortsetzen](/docs/de/hooks#defer-a-tool-call-for-later) können.
  * Für `PostToolUse` Hooks können Sie `additionalContext` setzen, um Informationen zum Tool-Ergebnis anzuhängen. Um die Ausgabe des Tools vor Claude zu ersetzen, setzen Sie `updatedToolOutput`, das für jedes Tool in beiden SDKs funktioniert. Das ältere `updatedMCPToolOutput` Feld ersetzt nur MCP-Tool-Ausgabe und ist veraltet.
  * Im TypeScript SDK kann ein `PostToolUse` Callback auch `classifierContext` zurückgeben, eine kurze Notiz über das Ergebnis des Tool-Aufrufs für den [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) Berechtigungsklassifizierer. Da Ihr Callback in Ihrem eigenen Anwendungsprozess ausgeführt wird, kann der Klassifizierer eine Benutzeraussage, die Sie in der Notiz weitergeben, als Benutzerabsicht gewichten. Das Feld erfordert TypeScript Agent SDK v0.3.236 oder später. [Annotieren Sie ein Ergebnis für den Auto-Modus Klassifizierer](/docs/de/hooks#annotate-a-result-for-the-auto-mode-classifier) behandelt die Längenbegrenzung, die Nur-Synchron-Regel und was nicht in die Notiz gehört.

Geben Sie `{}` zurück, um die Operation ohne Änderungen zuzulassen. SDK-Callback-Hooks verwenden das gleiche JSON-Ausgabeformat wie [Claude Code Shell-Befehls-Hooks](/docs/de/hooks#json-output), das jedes Feld und ereignisspezifische Option dokumentiert. Für die SDK-Typdefinitionen siehe die [TypeScript](/docs/de/agent-sdk/typescript#synchookjsonoutput) und [Python](/docs/de/agent-sdk/python#synchookjsonoutput) SDK-Referenzen.

<Note>
  Wenn mehrere Hooks oder Berechtigungsregeln gelten, hat `deny` Vorrang vor `defer`, was Vorrang vor `ask` hat, was Vorrang vor `allow` hat. Wenn ein Hook `deny` zurückgibt, wird die Operation blockiert, unabhängig von anderen Hooks.
</Note>

<h4 id="asynchronous-output">
  Asynchrone Ausgabe
</h4>

Standardmäßig wartet der Agent darauf, dass Ihr Hook zurückkommt, bevor er fortfährt. Wenn Ihr Hook einen Nebeneffekt ausführt, wie Protokollierung oder Webhook-Versand, und das Verhalten des Agenten nicht beeinflussen muss, können Sie stattdessen eine asynchrone Ausgabe zurückgeben. Dies teilt dem Agent mit, dass er sofort fortfahren soll, ohne auf die Fertigstellung des Hooks zu warten. In diesem Ausschnitt stehen `send_to_logging_service` in Python und `sendToLoggingService` in TypeScript für jede Protokollierungsfunktion, die Sie definieren:

<CodeGroup>
  ```python Python theme={null}
  async def async_hook(input_data, tool_use_id, context):
      # Start a background task, then return immediately
      asyncio.create_task(send_to_logging_service(input_data))
      return {"async_": True, "asyncTimeout": 30000}
  ```

  ```typescript TypeScript theme={null}
  const asyncHook: HookCallback = async (input, toolUseID, { signal }) => {
    // Start a background task, then return immediately
    sendToLoggingService(input).catch(console.error);
    return { async: true, asyncTimeout: 30000 };
  };
  ```
</CodeGroup>

| Feld           | Typ      | Beschreibung                                                                                                                                     |
| -------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| `async`        | `true`   | Signalisiert Async-Modus. Der Agent fährt fort, ohne zu warten. In Python verwenden Sie `async_`, um das reservierte Schlüsselwort zu vermeiden. |
| `asyncTimeout` | `number` | Optionales Timeout in Millisekunden für die Hintergrund-Operation                                                                                |

<Note>
  Asynchrone Ausgaben können nicht blockieren, ändern oder Kontext in die Operation einfügen, da der Agent bereits weitergegangen ist. Verwenden Sie sie nur für Nebeneffekte wie Protokollierung, Metriken oder Benachrichtigungen.
</Note>

<h2 id="examples">
  Beispiele
</h2>

Mehrere Beispiele in diesem Abschnitt zeigen nur die Callback-Funktion. Um eines auszuführen, registrieren Sie den Callback unter dem entsprechenden Ereignis im `hooks` Feld Ihrer Optionen, wie in [Hooks konfigurieren](#configure-hooks) gezeigt.

<h3 id="modify-tool-input">
  Tool-Eingabe ändern
</h3>

Dieses Beispiel fängt Write-Tool-Aufrufe ab und schreibt das `file_path` Argument um, um `/sandbox` voranzustellen, wodurch alle Datei-Schreibvorgänge in ein Sandbox-Verzeichnis umgeleitet werden. Der Callback gibt `updatedInput` mit dem geänderten Pfad und `permissionDecision: 'allow'` zurück, um die umgeschriebene Operation automatisch zu genehmigen:

<CodeGroup>
  ```python Python theme={null}
  async def redirect_to_sandbox(input_data, tool_use_id, context):
      if input_data["hook_event_name"] != "PreToolUse":
          return {}

      if input_data["tool_name"] == "Write":
          original_path = input_data["tool_input"].get("file_path", "")
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "allow",
                  "updatedInput": {
                      **input_data["tool_input"],
                      "file_path": f"/sandbox{original_path}",
                  },
              }
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const redirectToSandbox: HookCallback = async (input, toolUseID, { signal }) => {
    if (input.hook_event_name !== "PreToolUse") return {};

    const preInput = input as PreToolUseHookInput;
    const toolInput = preInput.tool_input as Record<string, unknown>;
    if (preInput.tool_name === "Write") {
      const originalPath = toolInput.file_path as string;
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "allow",
          updatedInput: {
            ...toolInput,
            file_path: `/sandbox${originalPath}`
          }
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<Note>
  Kombinieren Sie `updatedInput` mit `permissionDecision: 'allow'`, um die geänderte Eingabe automatisch zu genehmigen, oder `permissionDecision: 'ask'`, um sie dem Benutzer anzuzeigen. Wenn Sie `permissionDecision` weglassen, wird die geänderte Eingabe trotzdem angewendet und durchläuft die normale Berechtigungsprüfung. Mit `'defer'` wird `updatedInput` ignoriert. Geben Sie immer ein neues Objekt zurück, anstatt das ursprüngliche `tool_input` zu mutieren.
</Note>

Um die Umleitung zu bestätigen, setzen Sie das Präfix auf einen Pfad, in den Sie schreiben können, z. B. `./sandbox` oder `/tmp/sandbox` (macOS erlaubt nicht das Erstellen eines Root-Level `/sandbox` Verzeichnisses), und bitten Sie dann den Agent, eine Datei zu schreiben: Das Ergebnis des Write-Tools im Nachrichtenstrom nennt den Pfad mit Ihrem Sandbox-Präfix anstelle des von Claude angeforderten.

<h3 id="add-context-and-block-a-tool">
  Kontext hinzufügen und ein Tool blockieren
</h3>

Dieses Beispiel blockiert Schreibvorgänge in das `/etc` Verzeichnis und erklärt den Grund sowohl dem Modell als auch dem Benutzer:

* `permissionDecision: 'deny'` stoppt den Tool-Aufruf.
* `permissionDecisionReason` teilt dem Modell mit, warum, damit es nicht erneut versucht.
* `systemMessage` zeigt dem Benutzer, was passiert ist.

<CodeGroup>
  ```python Python theme={null}
  async def block_etc_writes(input_data, tool_use_id, context):
      file_path = input_data["tool_input"].get("file_path", "")

      if file_path.startswith("/etc"):
          return {
              # Top-level field: message shown to the user
              "systemMessage": "Remember: system directories like /etc are protected.",
              # hookSpecificOutput: block the operation
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "deny",
                  "permissionDecisionReason": "Writing to /etc is not allowed",
              },
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const blockEtcWrites: HookCallback = async (input, toolUseID, { signal }) => {
    const preInput = input as PreToolUseHookInput;
    const toolInput = preInput.tool_input as Record<string, unknown>;
    const filePath = toolInput?.file_path as string;

    if (filePath?.startsWith("/etc")) {
      return {
        // Top-level field: message shown to the user
        systemMessage: "Remember: system directories like /etc are protected.",
        // hookSpecificOutput: block the operation
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "deny",
          permissionDecisionReason: "Writing to /etc is not allowed"
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<h3 id="auto-approve-specific-tools">
  Spezifische Tools automatisch genehmigen
</h3>

Standardmäßig kann der Agent vor der Verwendung bestimmter Tools um Genehmigung bitten. Dieses Beispiel genehmigt schreibgeschützte Dateisystem-Tools (Read, Glob, Grep) automatisch, indem `permissionDecision: 'allow'` zurückgegeben wird, sodass sie ohne Benutzerbestätigung ausgeführt werden, während alle anderen Tools normalen Berechtigungsprüfungen unterliegen:

<CodeGroup>
  ```python Python theme={null}
  async def auto_approve_read_only(input_data, tool_use_id, context):
      if input_data["hook_event_name"] != "PreToolUse":
          return {}

      read_only_tools = ["Read", "Glob", "Grep"]
      if input_data["tool_name"] in read_only_tools:
          return {
              "hookSpecificOutput": {
                  "hookEventName": input_data["hook_event_name"],
                  "permissionDecision": "allow",
                  "permissionDecisionReason": "Read-only tool auto-approved",
              }
          }
      return {}
  ```

  ```typescript TypeScript theme={null}
  const autoApproveReadOnly: HookCallback = async (input, toolUseID, { signal }) => {
    if (input.hook_event_name !== "PreToolUse") return {};

    const preInput = input as PreToolUseHookInput;
    const readOnlyTools = ["Read", "Glob", "Grep"];
    if (readOnlyTools.includes(preInput.tool_name)) {
      return {
        hookSpecificOutput: {
          hookEventName: preInput.hook_event_name,
          permissionDecision: "allow",
          permissionDecisionReason: "Read-only tool auto-approved"
        }
      };
    }
    return {};
  };
  ```
</CodeGroup>

<h3 id="register-multiple-hooks">
  Mehrere Hooks registrieren
</h3>

Wenn ein Ereignis ausgelöst wird, werden alle übereinstimmenden Hooks parallel ausgeführt. Bei Berechtigungsentscheidungen gewinnt das restriktivste Ergebnis: Ein einzelnes `deny` blockiert den Tool-Aufruf, unabhängig davon, was die anderen Hooks zurückgeben. Da die Abschlussreihenfolge nicht deterministisch ist, schreiben Sie jeden Hook so, dass er unabhängig agiert, anstatt sich darauf zu verlassen, dass ein anderer Hook zuerst ausgeführt wurde.

Das folgende Beispiel registriert drei unabhängige Prüfungen für jeden Tool-Aufruf:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={
          "PreToolUse": [
              HookMatcher(hooks=[authorization_check]),
              HookMatcher(hooks=[input_validator]),
              HookMatcher(hooks=[audit_logger]),
          ]
      }
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    hooks: {
      PreToolUse: [
        { hooks: [authorizationCheck] },
        { hooks: [inputValidator] },
        { hooks: [auditLogger] }
      ]
    }
  };
  ```
</CodeGroup>

<h3 id="filter-with-multi-tool-matchers">
  Mit Multi-Tool-Matchern filtern
</h3>

Verwenden Sie Multi-Tool-Matcher, um einen Callback über verwandte Tools hinweg zu teilen. Dieses Beispiel registriert drei Matcher mit unterschiedlichen Bereichen:

* Eine durch Pipe getrennte exakte Liste (`Write|Edit|NotebookEdit`) löst `file_security_hook` nur für Datei-Änderungs-Tools aus.
* Ein Regex (`^mcp__`) löst `mcp_audit_hook` für alle MCP-Tools aus, deren Namen mit `mcp__` beginnen.
* Ein weggelassener Matcher löst `global_logger` für jeden Tool-Aufruf unabhängig vom Namen aus.

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      hooks={
          "PreToolUse": [
              # Match file modification tools
              HookMatcher(matcher="Write|Edit|NotebookEdit", hooks=[file_security_hook]),
              # Match all MCP tools
              HookMatcher(matcher="^mcp__", hooks=[mcp_audit_hook]),
              # Match everything (no matcher)
              HookMatcher(hooks=[global_logger]),
          ]
      }
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    hooks: {
      PreToolUse: [
        // Match file modification tools
        { matcher: "Write|Edit|NotebookEdit", hooks: [fileSecurityHook] },

        // Match all MCP tools
        { matcher: "^mcp__", hooks: [mcpAuditHook] },

        // Match everything (no matcher)
        { hooks: [globalLogger] }
      ]
    }
  };
  ```
</CodeGroup>

<h3 id="track-subagent-activity">
  Subagent-Aktivität verfolgen
</h3>

Verwenden Sie `SubagentStop` Hooks, um zu überwachen, wenn Subagents ihre Arbeit beenden. Siehe den vollständigen Eingabetyp in den [TypeScript](/docs/de/agent-sdk/typescript#hookinput) und [Python](/docs/de/agent-sdk/python#hookinput) SDK-Referenzen. Dieses Beispiel protokolliert eine Zusammenfassung jedes Mal, wenn ein Subagent abgeschlossen wird:

<CodeGroup>
  ```python Python theme={null}
  async def subagent_tracker(input_data, tool_use_id, context):
      # Log subagent details when it finishes
      print(f"[SUBAGENT] Completed: {input_data['agent_id']}")
      print(f"  Transcript: {input_data['agent_transcript_path']}")
      print(f"  Tool use ID: {tool_use_id}")
      print(f"  Stop hook active: {input_data.get('stop_hook_active')}")
      return {}


  options = ClaudeAgentOptions(
      hooks={"SubagentStop": [HookMatcher(hooks=[subagent_tracker])]}
  )
  ```

  ```typescript TypeScript theme={null}
  import { HookCallback, SubagentStopHookInput } from "@anthropic-ai/claude-agent-sdk";

  const subagentTracker: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast to SubagentStopHookInput to access subagent-specific fields
    const subInput = input as SubagentStopHookInput;

    // Log subagent details when it finishes
    console.log(`[SUBAGENT] Completed: ${subInput.agent_id}`);
    console.log(`  Transcript: ${subInput.agent_transcript_path}`);
    console.log(`  Tool use ID: ${toolUseID}`);
    console.log(`  Stop hook active: ${subInput.stop_hook_active}`);
    return {};
  };

  const options = {
    hooks: {
      SubagentStop: [{ hooks: [subagentTracker] }]
    }
  };
  ```
</CodeGroup>

<h3 id="make-http-requests-from-hooks">
  HTTP-Anfragen von Hooks aus stellen
</h3>

Hooks können asynchrone Operationen wie HTTP-Anfragen ausführen. Fangen Sie Fehler in Ihrem Hook ab, anstatt sie zu propagieren.

Dieses Beispiel sendet einen Webhook nach jeder Tool-Fertigstellung und protokolliert, welches Tool ausgeführt wurde und wann. Der Hook fängt Fehler von einem fehlgeschlagenen Webhook ab:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import json
  import urllib.request
  from datetime import datetime


  def _send_webhook(tool_name):
      """Synchronous helper that POSTs tool usage data to an external webhook."""
      data = json.dumps(
          {
              "tool": tool_name,
              "timestamp": datetime.now().isoformat(),
          }
      ).encode()
      req = urllib.request.Request(
          "https://api.example.com/webhook",
          data=data,
          headers={"Content-Type": "application/json"},
          method="POST",
      )
      urllib.request.urlopen(req)


  async def webhook_notifier(input_data, tool_use_id, context):
      # Only fire after a tool completes (PostToolUse), not before
      if input_data["hook_event_name"] != "PostToolUse":
          return {}

      try:
          # Run the blocking HTTP call in a thread to avoid blocking the event loop
          await asyncio.to_thread(_send_webhook, input_data["tool_name"])
      except Exception as e:
          # Log the error but don't raise
          print(f"Webhook request failed: {e}")

      return {}
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, PostToolUseHookInput } from "@anthropic-ai/claude-agent-sdk";

  const webhookNotifier: HookCallback = async (input, toolUseID, { signal }) => {
    // Only fire after a tool completes (PostToolUse), not before
    if (input.hook_event_name !== "PostToolUse") return {};

    try {
      await fetch("https://api.example.com/webhook", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          tool: (input as PostToolUseHookInput).tool_name,
          timestamp: new Date().toISOString()
        }),
        // Pass signal so the request cancels if the hook times out
        signal
      });
    } catch (error) {
      // Handle cancellation separately from other errors
      if (error instanceof Error && error.name === "AbortError") {
        console.log("Webhook request cancelled");
      }
      // Don't re-throw
    }

    return {};
  };

  // Register as a PostToolUse hook
  for await (const message of query({
    prompt: "Refactor the auth module",
    options: {
      hooks: {
        PostToolUse: [{ hooks: [webhookNotifier] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Um zu bestätigen, dass der Hook ausgelöst wird, verweisen Sie die Webhook-URL auf einen Endpunkt, den Sie überwachen können, und senden Sie eine Eingabeaufforderung, die ein Tool verwendet: Der Hook sendet einen POST mit dem Tool-Namen und dem Zeitstempel nach jeder Tool-Fertigstellung.

<h3 id="forward-notifications-to-slack">
  Benachrichtigungen an Slack weiterleiten
</h3>

Verwenden Sie `Notification` Hooks, um Systembenachrichtigungen vom Agent zu empfangen und sie an externe Dienste weiterzuleiten. In SDK-Sitzungen führt Claude Code diesen Hook für die folgenden Benachrichtigungstypen aus:

* [`permission_prompt`](/docs/de/hooks#notification) sobald eine Berechtigungsanfrage etwa sechs Sekunden auf Ihrem [`canUseTool` Callback](/docs/de/agent-sdk/user-input) gewartet hat. Erfordert TypeScript Agent SDK v0.3.233 oder später oder Python Agent SDK v0.2.139 oder später
* `elicitation_complete` und `elicitation_response` für Benutzer-Abfrage-Flows

Claude Code gibt die anderen Typen aus, wie `idle_prompt`, `auth_success` und `elicitation_dialog`, aus interaktiver Benutzeroberfläche, die SDK-Sitzungen nicht ausführen.

Jede Benachrichtigung enthält ein `message` Feld mit einer für Menschen lesbaren Beschreibung und optional einen `title`.

Dieses Beispiel leitet jede Benachrichtigung an einen Slack-Kanal weiter. Es erfordert eine [Slack Incoming Webhook URL](https://docs.slack.dev/messaging/sending-messages-using-incoming-webhooks/), die Sie erstellen, indem Sie eine App zu Ihrem Slack-Workspace hinzufügen und Incoming Webhooks aktivieren:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  import json
  import urllib.request

  from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, HookMatcher


  def _send_slack_notification(message):
      """Synchronous helper that sends a message to Slack via incoming webhook."""
      data = json.dumps({"text": f"Agent status: {message}"}).encode()
      req = urllib.request.Request(
          "https://hooks.slack.com/services/YOUR/WEBHOOK/URL",
          data=data,
          headers={"Content-Type": "application/json"},
          method="POST",
      )
      urllib.request.urlopen(req)


  async def notification_handler(input_data, tool_use_id, context):
      try:
          # Run the blocking HTTP call in a thread to avoid blocking the event loop
          await asyncio.to_thread(_send_slack_notification, input_data.get("message", ""))
      except Exception as e:
          print(f"Failed to send notification: {e}")

      # Return empty object. Notification hooks don't modify agent behavior
      return {}


  async def main():
      options = ClaudeAgentOptions(
          hooks={
              # Register the hook for Notification events (no matcher needed)
              "Notification": [HookMatcher(hooks=[notification_handler])],
          },
      )

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Analyze this codebase")
          async for message in client.receive_response():
              print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query, HookCallback, NotificationHookInput } from "@anthropic-ai/claude-agent-sdk";

  // Define a hook callback that sends notifications to Slack
  const notificationHandler: HookCallback = async (input, toolUseID, { signal }) => {
    // Cast to NotificationHookInput to access the message field
    const notification = input as NotificationHookInput;

    try {
      // POST the notification message to a Slack incoming webhook
      await fetch("https://hooks.slack.com/services/YOUR/WEBHOOK/URL", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          text: `Agent status: ${notification.message}`
        }),
        // Pass signal so the request cancels if the hook times out
        signal
      });
    } catch (error) {
      if (error instanceof Error && error.name === "AbortError") {
        console.log("Notification cancelled");
      } else {
        console.error("Failed to send notification:", error);
      }
    }

    // Return empty object. Notification hooks don't modify agent behavior
    return {};
  };

  // Register the hook for Notification events (no matcher needed)
  for await (const message of query({
    prompt: "Analyze this codebase",
    options: {
      hooks: {
        Notification: [{ hooks: [notificationHandler] }]
      }
    }
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

Wenn ein `Notification` Ereignis ausgelöst wird, sendet der Hook die `message` der Benachrichtigung, mit dem Präfix `Agent status:`, an den Kanal, auf den Ihr Webhook abzielt.

<h2 id="fix-common-issues">
  Häufige Probleme beheben
</h2>

<h3 id="hook-not-firing">
  Hook wird nicht ausgelöst
</h3>

* Überprüfen Sie, ob der Hook-Ereignisname korrekt und case-sensitiv ist (`PreToolUse`, nicht `preToolUse`)
* Überprüfen Sie, ob Ihr Matcher-Muster den Tool-Namen genau abgleicht
* Stellen Sie sicher, dass der Hook unter dem richtigen Ereignistyp in `options.hooks` ist
* Für Nicht-Tool-Hooks, die Matcher unterstützen, wie `Notification` und `SubagentStop`, gleichen Matcher gegen verschiedene Felder ab, und `Stop` ignoriert Matcher vollständig (siehe [Matcher-Muster](/docs/de/hooks#matcher-patterns))
* Hooks werden möglicherweise nicht ausgelöst, wenn der Agent das [`max_turns`](/docs/de/agent-sdk/python#claudeagentoptions) Limit erreicht, da die Sitzung endet, bevor Hooks ausgeführt werden können

<h3 id="matcher-not-filtering-as-expected">
  Matcher filtert nicht wie erwartet
</h3>

Matcher gleichen nur Tool-Namen ab, nicht Dateipfade oder andere Argumente. Um nach Dateipfad zu filtern, prüfen Sie `tool_input.file_path` in Ihrem Hook:

```typescript theme={null}
const myHook: HookCallback = async (input, toolUseID, { signal }) => {
  const preInput = input as PreToolUseHookInput;
  const toolInput = preInput.tool_input as Record<string, unknown>;
  const filePath = toolInput?.file_path as string;
  if (!filePath?.endsWith(".md")) return {}; // Skip non-markdown files
  // Process markdown files...
  return {};
};
```

<h3 id="hook-timeout">
  Hook-Timeout
</h3>

Claude Code führt jeden Callback mit einem Timeout aus, den Sie in Sekunden mit dem Feld `timeout` auf seinem `HookMatcher` festlegen. Wenn Sie keinen festlegen, verwendet Claude Code den Standard des Ereignisses: 600 Sekunden für die meisten Ereignisse, 30 Sekunden für `UserPromptSubmit`, `PreModelSwitch` und `PostModelSwitch`, und 10 Sekunden für `MessageDisplay`. Claude Code führt `SessionEnd` Callbacks während des Herunterfahrens unter dem kürzeren [SessionEnd-Timeout-Budget](/docs/de/hooks#sessionend-input) aus, standardmäßig 1,5 Sekunden.

Wenn ein Callback sein Timeout überschreitet, bricht Claude Code es ab und verwirft seine Ausgabe, und die Sitzung wird fortgesetzt, anstatt zu hängen. Was danach passiert, hängt vom Ereignis ab:

* `PreToolUse`: Claude Code führt den Tool-Aufruf nicht aus, Claude erhält ein Tool-Ergebnis, das besagt, dass der Hook nicht vor seinem Timeout geantwortet hat, und der Turn wird fortgesetzt. Wenn ein anderer `PreToolUse` Hook eine explizite Ablehnung zurückgegeben hat, erhält Claude stattdessen diese Ablehnung. Vor v2.1.210 meldete Claude Code das Timeout an Claude als Benutzerabweisung, was unbeaufsichtigte Sitzungen zum Stoppen und Warten auf Eingabe führte.
* `PostToolUse` und `PostToolUseFailure`: Claude Code behält das Tool-Ergebnis bei und der Turn wird fortgesetzt.
* `UserPromptSubmit` und [`UserPromptExpansion`](/docs/de/hooks#userpromptexpansion): Claude Code blockiert die Aufforderung mit einer Nachricht, die den Hook und das Timeout benennt, und die Sitzung wird fortgesetzt. Da ein Callback bei diesen Ereignissen als Richtlinien-Gate fungieren kann, lässt Claude Code niemals eine abgelaufene Aufforderung ungeprüft durch. Vor v2.1.208 endete Claude Code die Abfrage mit `error_during_execution`, wenn ein Callback bei diesen Ereignissen abgelaufen ist.
* `Stop` und `SubagentStop`: der abgelaufene Callback zählt als Rückgabe ohne Entscheidung. Der Agent oder Subagent stoppt, als hätte dieser Callback es erlaubt, und eine Entscheidung von Ihren anderen Hooks bei dem Ereignis wird immer noch angewendet. Vor Claude Code v2.1.273 zählte ein abgelaufener `Stop` oder `SubagentStop` Callback als fehlgeschlagener Hook-Lauf, und Claude Code verwarf die Entscheidungen Ihrer anderen Hooks bei dem Ereignis.
* `SessionStart`: der abgelaufene Callback zählt als Rückgabe ohne Ausgabe, und die Sitzung wird mit der Ausgabe Ihrer anderen `SessionStart` Hooks fortgesetzt.
* `PreModelSwitch`: Claude Code blockiert den Modellwechsel. Ein Hook, der nicht antwortet, hat den Wechsel nicht genehmigt.
* Andere Ereignisse, wie `Notification`, `PreCompact` und `PostModelSwitch`: Claude Code protokolliert den Fehler und wird fortgesetzt.

Das erste Mal, wenn ein `Stop` oder `SessionStart` Callback in der Hauptsitzung abläuft, fügt Claude Code auch eine [`SDKInformationalMessage`](/docs/de/agent-sdk/typescript#sdkinformationalmessage) zum Nachrichtenstrom hinzu, die besagt, dass die App, die die Sitzung steuert, nicht geantwortet hat. Spätere Timeouts wiederholen diese Nachricht nicht, während Ihre App nicht reagiert.

Wenn Sie die Abfrage unterbrechen, während ein Callback ausstehend ist, bricht Claude Code den ausstehenden Tool-Aufruf ab. Vor v2.1.208 konnte der Tool-Aufruf noch fortfahren, wenn Sie während eines ausstehenden `PreToolUse` Callbacks unterbrochen haben.

Wenn Ihr Callback mehr Zeit benötigt, legen Sie einen höheren `timeout` auf seinem `HookMatcher` fest. In TypeScript verwenden Sie das `AbortSignal` aus dem dritten Callback-Argument, um Abbruch elegant zu behandeln, wenn das Timeout abläuft.

<h3 id="tool-blocked-unexpectedly">
  Tool wird unerwartet blockiert
</h3>

* Überprüfen Sie alle `PreToolUse` Hooks auf `permissionDecision: 'deny'` Rückgaben
* Fügen Sie Protokollierung zu Ihren Hooks hinzu, um zu sehen, welche `permissionDecisionReason` sie zurückgeben
* Überprüfen Sie, ob Matcher-Muster nicht zu breit sind: ein leerer Matcher gleicht alle Tools ab

<h3 id="modified-input-not-applied">
  Geänderte Eingabe wird nicht angewendet
</h3>

* Stellen Sie sicher, dass `updatedInput` in `hookSpecificOutput` ist, nicht auf der obersten Ebene:

  ```typescript theme={null}
  return {
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "allow",
      updatedInput: { command: "new command" }
    }
  };
  ```

* Kombinieren Sie `updatedInput` nicht mit `permissionDecision: 'defer'`, das die geänderte Eingabe verwirft. Das Weglassen von `permissionDecision` ist in Ordnung: die geänderte Eingabe wird immer noch durch die normale Berechtigungsevaluierung angewendet. Sie können auch `'allow'` zurückgeben, um die geänderte Eingabe automatisch zu genehmigen, oder `'ask'`, um sie dem Benutzer zur Genehmigung anzuzeigen

* Schließen Sie `hookEventName` in `hookSpecificOutput` ein, um zu identifizieren, für welchen Hook-Typ die Ausgabe bestimmt ist

<h3 id="session-hooks-not-available-in-python">
  Sitzungs-Hooks nicht in Python verfügbar
</h3>

`SessionStart` und `SessionEnd` können als SDK-Callback-Hooks in TypeScript registriert werden, sind aber im Python SDK nicht verfügbar, da sein `HookEvent` Typ sie auslässt. In Python sind sie nur als [Shell-Befehls-Hooks](/docs/de/hooks#hook-events) verfügbar, die in Einstellungsdateien wie `.claude/settings.json` definiert sind. Um Shell-Befehls-Hooks aus Ihrer SDK-Anwendung zu laden, schließen Sie die entsprechende Einstellungsquelle mit [`setting_sources`](/docs/de/agent-sdk/python#settingsource) oder [`settingSources`](/docs/de/agent-sdk/typescript#settingsource) ein:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      setting_sources=["project"],  # Loads .claude/settings.json including hooks
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    settingSources: ["project"] // Loads .claude/settings.json including hooks
  };
  ```
</CodeGroup>

Um stattdessen Initialisierungslogik als Python SDK-Callback auszuführen, verwenden Sie die erste Nachricht von `client.receive_response()` als Auslöser.

<h3 id="subagent-permission-prompts-multiplying">
  Subagent-Berechtigungsaufforderungen vervielfachen sich
</h3>

Beim Spawnen mehrerer Subagents kann jeder einzelne Berechtigungen separat anfordern. Um wiederholte Aufforderungen zu vermeiden, verwenden Sie `PreToolUse` Hooks, um spezifische Tools automatisch zu genehmigen, oder konfigurieren Sie Berechtigungsregeln, die Subagents [vom übergeordneten Gespräch erben](/docs/de/sub-agents#permission-modes).

<h3 id="recursive-hook-loops-with-subagents">
  Rekursive Hook-Schleifen mit Subagents
</h3>

Ein `UserPromptSubmit` Hook, der Subagents spawnt, kann unendliche Schleifen erzeugen, wenn diese Subagents denselben Hook auslösen. Um dies zu verhindern:

* Verwenden Sie eine gemeinsame Variable oder Sitzungsstatus, um zu verfolgen, ob Sie bereits in einem Subagent sind
* Beschränken Sie Hooks so, dass sie nur für die Top-Level-Agent-Sitzung ausgeführt werden

<h3 id="systemmessage-not-appearing-in-output">
  systemMessage wird nicht in der Ausgabe angezeigt
</h3>

Das `systemMessage` Feld zeigt eine Nachricht für den Benutzer an, nicht für das Modell. Auf Claude Code v2.1.227 oder später kann die `systemMessage` eines Hooks in dem Nachrichtenstrom als [`SDKInformationalMessage`](/docs/de/agent-sdk/typescript#sdkinformationalmessage) auftauchen. Ob dies der Fall ist, hängt vom Ereignis ab. Der Abschnitt jedes [Ereignisses](/docs/de/hooks#hook-events) auf der Hooks-Seite sagt, wie die Ausgabe auftaucht. Um stattdessen Kontext an das Modell zu übergeben, geben Sie [`additionalContext`](/docs/de/hooks#add-context-for-claude) zurück.

Vor v2.1.227 gab das SDK Hook-Ausgaben im Nachrichtenstrom nur für `SessionStart` und `Setup` Hooks aus. Für jedes andere Ereignis erschien die Ausgabe nur in den Lebenszyklusereignissen, die [`includeHookEvents`](/docs/de/agent-sdk/typescript#options) (`include_hook_events` in Python) hinzufügt. Der Eintrag dieser Option behandelt, welche Lebenszyklusereignisse jedes Hook-Ereignis erzeugt.

Wenn Sie Hook-Entscheidungen für Ihre Anwendung zuverlässig sichtbar machen müssen, protokollieren Sie sie separat oder verwenden Sie einen dedizierten Ausgabekanal.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

* [Claude Code Hooks-Referenz](/docs/de/hooks): vollständige JSON-Eingabe-/Ausgabeschemas, Ereignisdokumentation und Matcher-Muster
* [Claude Code Hooks-Leitfaden](/docs/de/hooks-guide): Shell-Befehls-Hook-Beispiele und Walkthroughs
* [TypeScript SDK-Referenz](/docs/de/agent-sdk/typescript): Hook-Typen, Eingabe-/Ausgabedefinitionen und Konfigurationsoptionen
* [Python SDK-Referenz](/docs/de/agent-sdk/python): Hook-Typen, Eingabe-/Ausgabedefinitionen und Konfigurationsoptionen
* [Berechtigungen](/docs/de/agent-sdk/permissions): Steuern Sie, was Ihr Agent tun kann
* [Benutzerdefinierte Tools](/docs/de/agent-sdk/custom-tools): Erstellen Sie Tools, um Agent-Funktionen zu erweitern
