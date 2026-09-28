> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurieren Sie Ihren Agent

> Konfigurieren Sie Agent SDK-Sitzungen: stellen Sie das Optionsobjekt zusammen, legen Sie das Modell, die Umgebung und Limits fest, und finden Sie die Seite jeder Funktionsoption.

Eine Agent SDK-Sitzung liest die Konfiguration aus Einstellungsdateien, Umgebungsvariablen und dem `options`-Objekt, das Sie beim Starten übergeben. Diese Seite zeigt, wie Sie das `options`-Objekt zusammenstellen und welche Einstellungsdateien und Umgebungsvariablen die Kontrolle übernehmen.

Für jeden Optionstyp und Standard siehe die [`Options`](/docs/de/agent-sdk/typescript#options) (TypeScript) und [`ClaudeAgentOptions`](/docs/de/agent-sdk/python#claudeagentoptions) (Python) Referenzen.

<h2 id="pass-options-to-a-session">
  Optionen an eine Sitzung übergeben
</h2>

Jeder `query()`-Aufruf akzeptiert ein Optionsobjekt: `Options` in TypeScript, `ClaudeAgentOptions` in Python. Jedes Feld ist optional, und eine Sitzung, die ohne Optionen gestartet wird, läuft mit den SDK-Standardwerten. Das folgende Beispiel konfiguriert eine schreibgeschützte Sitzung, die die offenen TODOs eines Projekts zusammenfasst. Paare lesen sich als TypeScript / Python, wo sich die Schreibweisen unterscheiden:

* **`model`**: wählt das Modell
* **`allowedTools` / `allowed_tools`**: genehmigt vorab eine schreibgeschützte Werkzeugliste
* **`maxTurns` / `max_turns`**: begrenzt die Anzahl der Züge
* **`cwd`**: legt das Arbeitsverzeichnis fest

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Summarize the open TODOs in this repo",
    options: {
      model: "claude-sonnet-5",
      allowedTools: ["Read", "Glob", "Grep"],
      maxTurns: 8,
      cwd: "/path/to/repo",
    },
  })) {
    if (message.type === "result" && message.subtype === "success" && !message.is_error) {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, ResultMessage, query

  async def main():
      options = ClaudeAgentOptions(
          model="claude-sonnet-5",
          allowed_tools=["Read", "Glob", "Grep"],
          max_turns=8,
          cwd="/path/to/repo",
      )

      async for message in query(
          prompt="Summarize the open TODOs in this repo",
          options=options,
      ):
          if isinstance(message, ResultMessage) and not message.is_error:
              print(message.result)

  asyncio.run(main())
  ```
</CodeGroup>

Zeigen Sie `cwd` auf eines Ihrer eigenen Projekte und führen Sie das Beispiel aus. Die Zusammenfassung der offenen TODOs dieses Projekts wird gedruckt, wenn die Ergebnismeldung ankommt.

`allowedTools` (TypeScript) oder `allowed_tools` (Python) genehmigt die aufgelisteten Werkzeuge vorab, sodass Aufrufe an sie ohne Genehmigung ausgeführt werden. Werkzeuge außerhalb der Liste bleiben verfügbar. Wenn Claude ein nicht aufgelistetes Werkzeug aufruft, entscheidet der Genehmigungsmodus, ob der Aufruf ausgeführt wird. Weitere Informationen finden Sie unter [Allow and deny rules](/docs/de/agent-sdk/permissions#allow-and-deny-rules).

<h2 id="load-settings-files">
  Einstellungsdateien laden
</h2>

Einstellungsdateien liefern Konfigurationen über das Optionsobjekt hinaus. Zwei Optionen steuern, wie sie geladen werden:

* **`settingSources` / `setting_sources`**: steuert, welche Dateisystemquellen geladen werden: Benutzer, Projekt und lokal. Einstellungsdateien und CLAUDE.md-Dateien kommen durch diese Quellen an.
* **`settings`**: lädt einen Einstellungsdateipfad oder eine Inline-JSON-Zeichenkette in beiden Sprachen, und TypeScript akzeptiert auch ein Einstellungsobjekt. Welche Form Sie auch übergeben, sie überschreibt Benutzer-, Projekt- und lokale Dateisystemeinstellungen; nur verwaltete Richtlinieneinstellungen haben einen höheren Rang. Die Referenzen dokumentieren die vollständige Rangfolge unter [Settings precedence](/docs/de/agent-sdk/typescript#settings-precedence) für TypeScript und [Settings precedence](/docs/de/agent-sdk/python#settings-precedence) für Python.

Übergeben Sie `[]`, um Benutzer-, Projekt- und lokale Einstellungen zu deaktivieren. Weitere Informationen finden Sie unter [Use Claude Code features in the SDK](/docs/de/agent-sdk/claude-code-features).

<h2 id="choose-a-model">
  Wählen Sie ein Modell
</h2>

Wenn die `model`-Option, Ihre Einstellungen oder Ihre Umgebung kein Modell auswählen, startet eine neue Sitzung auf [Claude Code's default model](/docs/de/model-config#default-model-setting). Für die Reihenfolge dieser Quellen siehe [Setting your model](/docs/de/model-config#setting-your-model). Setzen Sie `model`, um ein bestimmtes Modell festzulegen, oder wählen Sie ein kleineres für schnellere, günstigere Agenten. Der Wert nimmt einen Modellalias oder einen vollständigen Modellnamen an; Aliase und die Versionen, zu denen sie aufgelöst werden, sind unter [Model aliases](/docs/de/model-config#model-aliases) aufgelistet.

Setzen Sie `fallbackModel` (TypeScript) oder `fallback_model` (Python), um ein Sicherungsmodell zu benennen. Wenn das primäre Modell überlastet oder nicht verfügbar ist, wechselt die Sitzung zum Sicherungsmodell. Das primäre Modell wird zu Beginn jedes Benutzerzugs erneut versucht, sodass die Sitzung zu ihm zurückkehrt, sobald der Ausfall vorbei ist.

In beiden Sprachen akzeptiert die Option ein einzelnes Modell oder eine kommagetrennte Liste von Sicherungen. Für die Reihenfolge und die Kettenbegrenzung siehe [Fallback model chains](/docs/de/model-config#fallback-model-chains). In TypeScript wirft ein Fallback gleich `model` einen Fehler beim Start.

Die folgenden Beispiele zeigen eine Fallback-Liste in TypeScript und ein einzelnes Fallback in Python:

<CodeGroup>
  ```typescript TypeScript theme={null}
  const options = {
    model: "claude-fable-5",
    fallbackModel: "claude-opus-5,claude-sonnet-5",
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      model="claude-fable-5",
      fallback_model="claude-opus-5",
  )
  ```
</CodeGroup>

<span id="sampling-parameters" />

<Note>
  Die [Messages API](https://platform.claude.com/docs/en/api/messages) Anfrageparameter `temperature`, `top_p` und `max_tokens` haben keine Felder auf dem Optionsobjekt in beiden Sprachen. Setzen Sie stattdessen die [effort level](/docs/de/agent-sdk/agent-loop#effort-level) oder ein [spend cap](#limit-turns-and-spend), oder rufen Sie die Messages API auf, wenn Sie diese Parameter direkt benötigen.
</Note>

<h2 id="set-environment-variables">
  Umgebungsvariablen setzen
</h2>

Die `env`-Option setzt Umgebungsvariablen für den Claude Code-Prozess, der Ihre Sitzung ausführt. Ob Ihre Werte die geerbte Umgebung ersetzen oder sich über sie hinweg zusammensetzen, unterscheidet sich je nach Sprache:

* **TypeScript**: `env` ersetzt die Subprozessumgebung
* **Python**: das SDK setzt Ihre Werte über die geerbte Umgebung zusammen, und Ihre Werte überschreiben die geerbten

In TypeScript verteilen Sie `process.env` in `env`, um geerbte Variablen wie `PATH`, `HOME` und `ANTHROPIC_API_KEY` zu behalten. Wenn Sie `env` nicht setzen, erbt der Subprozess Ihre Umgebung in beiden Sprachen.

Das Beispiel leitet API-Verkehr durch ein Gateway, indem es `ANTHROPIC_BASE_URL` setzt.

<CodeGroup>
  ```typescript TypeScript theme={null}
  const options = {
    env: { ...process.env, ANTHROPIC_BASE_URL: "https://gateway.example.com" },
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={"ANTHROPIC_BASE_URL": "https://gateway.example.com"},
  )
  ```
</CodeGroup>

Die Variablen, die Sie übergeben, können auch Claude Code selbst konfigurieren. Für die Variablen, die der Claude Code-Prozess liest, siehe [Environment variables](/docs/de/env-vars). Um API-Timeouts und Stall-Erkennung auf diese Weise zu optimieren, folgen Sie dem Abschnitt Handle slow or stalled API responses in der [TypeScript reference](/docs/de/agent-sdk/typescript#handle-slow-or-stalled-api-responses) oder der [Python reference](/docs/de/agent-sdk/python#handle-slow-or-stalled-api-responses).

<h2 id="set-the-working-directory">
  Legen Sie das Arbeitsverzeichnis fest
</h2>

Setzen Sie `cwd`, um die Sitzung in einem bestimmten Verzeichnis auszuführen. Wenn Sie `cwd` nicht setzen, läuft die Sitzung im Arbeitsverzeichnis Ihres Prozesses. Keines der SDKs hat einen Setter für `cwd`. Um in einem anderen Verzeichnis auszuführen, starten Sie eine weitere Sitzung mit diesem `cwd`.

Claude Code liest das Arbeitsverzeichnis, um Folgendes zu bestimmen:

* **Projekteinstellungen und Hooks**: welche Projekteinstellungen und Hooks geladen werden]\(/de/agent-sdk/claude-code-features)
* **Skills**: wo [session skills are discovered](/docs/de/agent-sdk/skills)
* **Sitzungsspeicher**: zu welchem Projekt eine [stored session belongs to](/docs/de/agent-sdk/session-storage)

Um Werkzeugen den Zugriff auf Dateien außerhalb des Arbeitsverzeichnisses zu ermöglichen, fügen Sie Pfade mit `additionalDirectories` (TypeScript) oder `add_dirs` (Python) hinzu. Für den Umfang dieser Berechtigung siehe [Additional directories grant file access, not configuration](/docs/de/permissions#additional-directories-grant-file-access-not-configuration).

<h2 id="limit-turns-and-spend">
  Begrenzen Sie Züge und Ausgaben
</h2>

Begrenzen Sie Züge und Ausgaben mit `maxTurns` / `max_turns` und `maxBudgetUsd` / `max_budget_usd`. Beide Limits sind deaktiviert, wenn nicht gesetzt. Wenn eine Sitzung ein Limit erreicht, endet der Lauf mit einer Ergebnismeldung, deren Subtyp das Limit benennt, `error_max_turns` oder `error_max_budget_usd`. Was danach passiert, unterscheidet sich je nach Eingabemodus:

* **Single-shot `query()`**: das SDK gibt das Cap-Ergebnis aus und wirft dann, also wickeln Sie die Schleife in einen Try-Block, um über den Fehler hinaus zu gehen
* **Streaming input**: die Sitzung bleibt über ein Cap-Ergebnis hinaus aktiv, und die Max-Turns-Anzahl beginnt für jede eingereihte Nachricht von vorne. Das Budget-Total sammelt sich über Nachrichten an, und sobald die Ausgaben das Limit erreichen, enden spätere Nachrichten in derselben Konversation mit demselben Budget-Ergebnis. Ein [`/clear`](/docs/de/agent-sdk/cost-tracking) startet das Budget neu

Die beiden Limits behandeln `0` unterschiedlich:

* **`maxTurns` / `max_turns`**: `0` führt die Sitzung ohne Zuglimit aus, dasselbe wie das Nicht-Setzen der Option
* **`maxBudgetUsd` / `max_budget_usd`**: die CLI lehnt `0` als ungültigen Betrag beim Start ab, und die Sitzung läuft nie

Weitere Informationen zu beiden Limits, einschließlich Subagent-Ausgaben, finden Sie unter [Turns and budget](/docs/de/agent-sdk/agent-loop#turns-and-budget).

<h2 id="change-configuration-mid-session">
  Ändern Sie die Konfiguration während der Sitzung
</h2>

Wenn Sie eine Sitzung mit [streaming input](/docs/de/agent-sdk/streaming-vs-single-mode) starten, können Sie ihr Modell und ihren Genehmigungsmodus während der Ausführung wechseln. Wo Sie die Setter aufrufen, unterscheidet sich je nach Sprache:

* **TypeScript**: Methoden auf dem Objekt, das `query()` zurückgibt
* **Python**: Methoden auf [`ClaudeSDKClient`](/docs/de/agent-sdk/python#claudesdkclient), da `query()` einen einfachen Iterator ohne Kontrollmethoden zurückgibt

Beide Sprachen haben die gleichen Setter:

* **`setModel()` / `set_model()`**: wechselt das Modell. Rufen Sie es ohne Modell auf, um zu [Claude Code's default model](/docs/de/model-config#default-model-setting) zu wechseln, anstatt zum `model`, das Sie in Optionen übergeben haben.
* **`setPermissionMode()` / `set_permission_mode()`**: wechselt den Genehmigungsmodus

TypeScript hat auch `applyFlagSettings()` und `updateSettings()`:

* **`applyFlagSettings()`**: wendet Einstellungen zur Laufzeit an, wie in `await session.applyFlagSettings({ effortLevel: "high" })`. Die Methode nimmt Einstellungsdateischlüssel anstelle von Optionsfeldern, also überprüfen Sie die [`applyFlagSettings()` reference](/docs/de/agent-sdk/typescript#applyflagsettings) für das Schema und für welche Schlüssel während der Sitzung wirksam werden.
* **`updateSettings()`**: schreibt einen zulassungslisten Schlüssel in eine Einstellungsdatei. Die [`updateSettings()` Referenz](/docs/de/agent-sdk/typescript#updatesettings) benennt den Schlüssel, den jede Quelle akzeptiert, und die Versionsuntergrenzen.
  * Übergeben Sie `"localSettings"`, um die lokale Einstellungsdatei des Projekts zu schreiben, wie in `await session.updateSettings("localSettings", { outputStyle: "Explanatory" })`. Der geschriebene Schlüssel tritt bei der nächsten Anfrage der Sitzung in Kraft und bleibt für spätere Sitzungen bestehen, die `local`-Einstellungen laden.
  * Übergeben Sie `"userSettings"`, um `effortLevel` zu schreiben, den einzigen Schlüssel, den diese Quelle akzeptiert. Claude Code speichert ihn als Standard-Anstrengungsgrad für das aktuelle Modell der Sitzung, und die Anstrengung der laufenden Sitzung ändert sich nicht.

Das folgende Beispiel führt eine zweizügige Sitzung aus, ändert die Konfiguration zwischen den Zügen und druckt das Modell, das jeden Zug beantwortet hat. In TypeScript hält der Prompt-Stream die zweite Nachricht, bis die Setter ausgeführt wurden, und der zweite Zug läuft auf dem neuen Modell.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SDKUserMessage } from "@anthropic-ai/claude-agent-sdk";

  function userMessage(text: string): SDKUserMessage {
    return { type: "user", message: { role: "user", content: text }, parent_tool_use_id: null };
  }

  // Hold the second prompt until the setters have run.
  let startSecondTurn!: () => void;
  const secondTurnReady = new Promise<void>((resolve) => {
    startSecondTurn = resolve;
  });

  async function* turnPrompts(): AsyncGenerator<SDKUserMessage, void> {
    yield userMessage("Reply with exactly: ready");
    await secondTurnReady;
    yield userMessage("Reply with exactly: done");
  }

  const session = query({
    prompt: turnPrompts(),
    options: {
      model: "claude-sonnet-5",
    },
  });

  let turnModel = "";
  let completedTurns = 0;

  for await (const message of session) {
    if (message.type === "assistant") {
      turnModel = message.message.model;
    } else if (message.type === "result") {
      completedTurns += 1;
      if (completedTurns === 1) {
        console.log(`First turn model: ${turnModel}`);
        await session.setModel("claude-opus-5");
        await session.setPermissionMode("acceptEdits");
        startSecondTurn();
      } else {
        console.log(`Second turn model: ${turnModel}`);
        break;
      }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import AssistantMessage, ClaudeAgentOptions, ClaudeSDKClient

  async def main():
      options = ClaudeAgentOptions(model="claude-sonnet-5")

      async with ClaudeSDKClient(options=options) as client:
          await client.query("Reply with exactly: ready")
          first_model = ""
          async for message in client.receive_response():
              if isinstance(message, AssistantMessage):
                  first_model = message.model

          await client.set_model("claude-opus-5")
          await client.set_permission_mode("acceptEdits")

          await client.query("Reply with exactly: done")
          second_model = ""
          async for message in client.receive_response():
              if isinstance(message, AssistantMessage):
                  second_model = message.model

      print(f"First turn model: {first_model}")
      print(f"Second turn model: {second_model}")

  asyncio.run(main())
  ```
</CodeGroup>

Auf der Claude API druckt das Programm `First turn model: claude-sonnet-5`, dann `Second turn model: claude-opus-5` nach dem Wechsel.

<Note>
  Jedes Modell hat seinen eigenen Prompt-Cache, sodass nach einem Wechsel während der Sitzung die nächste Anfrage die vollständige Konversation ungecacht zu den Sätzen des neuen Modells neu berechnet. Weitere Informationen finden Sie unter [Switching models](/docs/de/prompt-caching#switching-models).
</Note>

<h2 id="configure-specific-features">
  Konfigurieren Sie spezifische Funktionen
</h2>

Die folgende Tabelle ordnet jede Option der Funktion zu, die sie konfiguriert. Für Optionen, die diese Seite nicht abdeckt, siehe die [TypeScript](/docs/de/agent-sdk/typescript#options) und [Python](/docs/de/agent-sdk/python#claudeagentoptions) Referenzen. Wenn Sie Ihr Ziel kennen, aber nicht welche Option es erfüllt, beginnen Sie mit [Choose the right feature](/docs/de/agent-sdk/claude-code-features#choose-the-right-feature).

| TypeScript                | Python                      | Steuert                                        | Abgedeckt in                                                                                                                                                                                                         |
| ------------------------- | --------------------------- | ---------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissionMode`          | `permission_mode`           | Was der Agent ohne Genehmigung tun kann        | [Configure permissions](/docs/de/agent-sdk/permissions)                                                                                                                                                                   |
| `allowedTools`            | `allowed_tools`             | Welche Werkzeugaufrufe vorab genehmigt sind    | [Configure permissions](/docs/de/agent-sdk/permissions)                                                                                                                                                                   |
| `canUseTool`              | `can_use_tool`              | Ihr Genehmigungsrückruf für Werkzeugaufrufe    | [Handle tool approval requests](/docs/de/agent-sdk/user-input#handle-tool-approval-requests)                                                                                                                              |
| `systemPrompt`            | `system_prompt`             | Die Anweisungen des Agenten                    | [Modifying system prompts](/docs/de/agent-sdk/modifying-system-prompts)                                                                                                                                                   |
| `settingSources`          | `setting_sources`           | Welche Dateisystemeinstellungen geladen werden | [Use Claude Code features in the SDK](/docs/de/agent-sdk/claude-code-features)                                                                                                                                            |
| `mcpServers`              | `mcp_servers`               | Externe Werkzeugserver                         | [Connect to external tools with MCP](/docs/de/agent-sdk/mcp)                                                                                                                                                              |
| `agents`                  | `agents`                    | Subagent-Definitionen                          | [Subagents](/docs/de/agent-sdk/subagents)                                                                                                                                                                                 |
| `hooks`                   | `hooks`                     | Rückrufe an Lebenszykluspunkten                | [Hooks](/docs/de/agent-sdk/hooks)                                                                                                                                                                                         |
| `skills`                  | `skills`                    | Welche Skills geladen werden                   | [Extend agents with skills](/docs/de/agent-sdk/skills)                                                                                                                                                                    |
| `plugins`                 | `plugins`                   | Welche Plugins geladen werden                  | [Plugins](/docs/de/agent-sdk/plugins)                                                                                                                                                                                     |
| `outputFormat`            | `output_format`             | Strukturierte Ausgabeschemas                   | [Structured outputs](/docs/de/agent-sdk/structured-outputs)                                                                                                                                                               |
| `resume`                  | `resume`                    | Fortsetzen einer gespeicherten Sitzung         | [Sessions](/docs/de/agent-sdk/sessions)                                                                                                                                                                                   |
| `forkSession`             | `fork_session`              | Verzweigung einer Sitzung                      | [Sessions](/docs/de/agent-sdk/sessions)                                                                                                                                                                                   |
| `sessionStore`            | `session_store`             | Externe Sitzungspersistenz                     | [Session storage](/docs/de/agent-sdk/session-storage)                                                                                                                                                                     |
| `enableFileCheckpointing` | `enable_file_checkpointing` | Rückgängig machbare Dateibearbeitungen         | [File checkpointing](/docs/de/agent-sdk/file-checkpointing)                                                                                                                                                               |
| `effort`                  | `effort`                    | Wie viel Arbeit Claude in Antworten investiert | [Effort level](/docs/de/agent-sdk/agent-loop#effort-level)                                                                                                                                                                |
| `sandbox`                 | `sandbox`                   | Sandbox-Verhalten für Werkzeugausführung       | [TypeScript](/docs/de/agent-sdk/typescript#sandbox-configuration) und [Python](/docs/de/agent-sdk/python#sandbox-configuration) Referenzen, mit Bereitstellungskontext in [Secure deployment](/docs/de/agent-sdk/secure-deployment) |

<h2 id="next-steps">
  Nächste Schritte
</h2>

Um Konfiguration in funktionierenden Agenten zusammengesetzt zu sehen:

* **[Quickstart](/docs/de/agent-sdk/quickstart)**: bauen und führen Sie einen ersten Agent von Anfang bis Ende aus
* **[Examples](/docs/de/agent-sdk/examples)**: finden Sie ein vollständiges, ausführbares Projekt oder ein geführtes Claude Cookbook-Rezept, das dem entspricht, was Sie bauen möchten
* **[Multi-tenant isolation](/docs/de/agent-sdk/hosting#multi-tenant-isolation)**: isolieren Sie die Einstellungen und den Speicher jedes Mandanten mit `settingSources` / `setting_sources`, `env` und `cwd`
