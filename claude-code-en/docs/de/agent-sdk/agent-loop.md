> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# So funktioniert die Agent-Schleife

> Verstehen Sie den Nachrichtenlebenszyklus, die Werkzeugausführung, das Kontextfenster und die Architektur, die Ihre SDK-Agenten antreibt.

Das Agent SDK ermöglicht es Ihnen, die autonome Agent-Schleife von Claude Code in Ihre eigenen Anwendungen einzubetten. Das SDK ist ein eigenständiges Paket, das Ihnen programmatische Kontrolle über Werkzeuge, Berechtigungen, Kostenlimits und Ausgabe gibt.

Sowohl die TypeScript- als auch die Python-SDKs bündeln eine native Claude Code-Binärdatei, daher benötigen die meisten Installationen keine separate Claude Code-Installation. Siehe die [Installationsnotiz des Schnellstarts](/docs/de/agent-sdk/quickstart) für die Installationen, die dies tun.

Wenn Sie einen Agent starten, führt das SDK die gleiche [Ausführungsschleife aus, die Claude Code antreibt](/docs/de/how-claude-code-works#the-agentic-loop): Claude bewertet Ihren Prompt, ruft Werkzeuge auf, um Maßnahmen zu ergreifen, erhält die Ergebnisse und wiederholt dies, bis die Aufgabe abgeschlossen ist. Diese Seite erklärt, was in dieser Schleife passiert, damit Sie Ihre Agenten effektiv erstellen, debuggen und optimieren können.

<h2 id="the-loop-at-a-glance">
  Die Schleife auf einen Blick
</h2>

Jede Agent-Sitzung folgt dem gleichen Zyklus:

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-loop-diagram.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=1c6e8f28d80dba14a7287419656f1237" className="dark:hidden" alt="Diagramm der Agent-Schleife: Ihr Prompt wird in die agentengesteuerte Schleife eingegeben, wo Claude bewertet und entweder Werkzeugaufrufe anfordert, deren Ergebnisse in eine weitere Bewertung zurückfließen, oder die endgültige Antwort zurückgibt" width="720" height="212" data-path="images/agent-loop-diagram.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-loop-diagram-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=afe723c52a324d3c61fa72fb02432ab6" className="hidden dark:block" alt="Diagramm der Agent-Schleife: Ihr Prompt wird in die agentengesteuerte Schleife eingegeben, wo Claude bewertet und entweder Werkzeugaufrufe anfordert, deren Ergebnisse in eine weitere Bewertung zurückfließen, oder die endgültige Antwort zurückgibt" width="720" height="212" data-path="images/agent-loop-diagram-dark.svg" />

1. **Prompt empfangen.** Claude empfängt Ihren Prompt zusammen mit dem System-Prompt, Werkzeugdefinitionen und Gesprächsverlauf. Das SDK gibt eine [`SystemMessage`](#message-types) mit dem Subtyp `"init"` aus, die Sitzungsmetadaten enthält.
2. **Bewerten und antworten.** Claude bewertet den aktuellen Status und bestimmt, wie vorzugehen ist. Es kann mit Text antworten, einen oder mehrere Werkzeugaufrufe anfordern oder beides. Das SDK gibt ein oder mehrere [`AssistantMessage`](#message-types)-Objekte aus, eines für jeden Inhaltsblock, wie einen Textblock oder eine Werkzeugaufrufsanforderung.
3. **Werkzeuge ausführen.** Das SDK führt jedes angeforderte Werkzeug aus und sammelt die Ergebnisse. Jeder Satz von Werkzeugergebnissen wird an Claude für die nächste Entscheidung zurückgesendet. Sie können [Hooks](/docs/de/agent-sdk/hooks) verwenden, um Werkzeugaufrufe vor ihrer Ausführung abzufangen, zu ändern oder zu blockieren.
4. **Wiederholen.** Die Schritte 2 und 3 wiederholen sich als Zyklus. Jeder vollständige Zyklus ist eine Runde. Claude ruft weiterhin Werkzeuge auf und verarbeitet Ergebnisse, bis es eine Antwort ohne Werkzeugaufrufe erzeugt.
5. **Ergebnis zurückgeben.** Das SDK gibt eine endgültige [`AssistantMessage`](#message-types) mit der Textantwort (keine Werkzeugaufrufe) aus, gefolgt von einer [`ResultMessage`](#message-types) mit dem endgültigen Text, Token-Nutzung, Kosten und Sitzungs-ID.

Eine schnelle Frage („welche Dateien sind hier?") könnte eine oder zwei Runden dauern, in denen `Glob` aufgerufen und die Ergebnisse beantwortet werden. Eine komplexe Aufgabe („refaktorisieren Sie das Auth-Modul und aktualisieren Sie die Tests") kann Dutzende von Werkzeugaufrufen über viele Runden hinweg verketten, Dateien lesen, Code bearbeiten und Tests ausführen, wobei Claude seinen Ansatz basierend auf jedem Ergebnis anpasst.

<h2 id="turns-and-messages">
  Runden und Nachrichten
</h2>

Eine Runde ist eine Hin- und Rückfahrt in der Schleife: Claude erzeugt eine Ausgabe, die Werkzeugaufrufe enthält, das SDK führt diese Werkzeuge aus, und die Ergebnisse werden automatisch an Claude zurückgesendet. Dies geschieht, ohne die Kontrolle an Ihren Code zurückzugeben. Runden werden fortgesetzt, bis Claude eine Ausgabe ohne Werkzeugaufrufe erzeugt, woraufhin die Schleife endet und das endgültige Ergebnis geliefert wird.

Stellen Sie sich vor, wie eine vollständige Sitzung für den Prompt „Beheben Sie die fehlgeschlagenen Tests in auth.ts" aussehen könnte.

Zunächst sendet das SDK Ihren Prompt an Claude und gibt eine [`SystemMessage`](#message-types) mit den Sitzungsmetadaten aus. Dann beginnt die Schleife:

1. **Runde 1:** Claude ruft `Bash` auf, um `npm test` auszuführen. Das SDK gibt eine [`AssistantMessage`](#message-types) mit dem Werkzeugaufruf aus, führt den Befehl aus und gibt dann eine [`UserMessage`](#message-types) mit der Ausgabe (drei Fehler) aus.
2. **Runde 2:** Claude ruft `Read` auf `auth.ts` und `auth.test.ts` auf. Das SDK gibt eine `AssistantMessage` für jeden Aufruf aus und gibt die Dateiinhalte zurück.
3. **Runde 3:** Claude ruft `Edit` auf, um `auth.ts` zu beheben, und ruft dann `Bash` auf, um `npm test` erneut auszuführen. Alle drei Tests bestehen. Das SDK gibt eine `AssistantMessage` für jeden Aufruf aus.
4. **Letzte Runde:** Claude erzeugt eine nur-Text-Antwort ohne Werkzeugaufrufe: „Behobener Auth-Bug, alle drei Tests bestehen jetzt." Das SDK gibt eine endgültige `AssistantMessage` mit diesem Text aus, gefolgt von einer [`ResultMessage`](#message-types) mit dem gleichen Text plus Kosten und Nutzung.

Das waren vier Runden: drei mit Werkzeugaufrufen, eine endgültige nur-Text-Antwort.

Sie können die Schleife mit `max_turns` / `maxTurns` begrenzen, die nur Werkzeug-Nutzungsrunden zählt. Zum Beispiel würde `max_turns=2` in der obigen Schleife vor dem Bearbeitungsschritt gestoppt haben. Sie können auch `max_budget_usd` / `maxBudgetUsd` verwenden, um Runden basierend auf einem Ausgabenschwellenwert zu begrenzen.

Ohne Limits läuft die Schleife, bis Claude von selbst fertig ist, was für gut definierte Aufgaben in Ordnung ist, aber bei offenen Prompts lange laufen kann („verbessern Sie diese Codebasis"). Das Festlegen eines Budgets ist eine gute Standardeinstellung für Produktionsagenten. Siehe [Runden und Budget](#turns-and-budget) unten für die Optionsreferenz.

<h2 id="message-types">
  Nachrichtentypen
</h2>

Während die Schleife läuft, gibt das SDK einen Stream von Nachrichten aus. Jede Nachricht trägt einen Typ, der Ihnen sagt, aus welcher Phase der Schleife sie stammt. Die fünf Kerntypen sind:

* **`SystemMessage`:** Sitzungslebenszyklus-Ereignisse. Das Feld `subtype` unterscheidet sie:

  * `"init"`: Sitzungsmetadaten für den Durchlauf. Wenn ein `SessionStart`- oder `Setup`-Hook während des Sitzungsstarts ausgeführt wird, kommen seine [Hook-Lebenszyklus-Nachrichten](/docs/de/agent-sdk/typescript#sdkhookstartedmessage) vor der `init`-Nachricht an
  * `"compact_boundary"`: wird nach [Komprimierung](#automatic-compaction) ausgelöst
  * `"informational"`: einfache Textstatusbanner aus der Schleife
  * `"worker_shutting_down"`: der Host wird beendet oder Remote Control wurde getrennt

  In TypeScript ist jeder Subtyp außer `"init"` sein eigener Typ in der [`SDKMessage`-Union](/docs/de/agent-sdk/typescript#sdkmessage) statt eines Subtyps von `SDKSystemMessage`.
* **`AssistantMessage`:** wird für jeden Inhaltsblock in Claudes Antworten ausgegeben, einschließlich des endgültigen nur-Text-Blocks. Jede trägt einen einzelnen Inhaltsblock, wie Text oder einen Werkzeugaufruf, und die Nachrichten aus einer Antwort teilen sich eine Nachrichten-ID.
* **`UserMessage`:** wird nach jeder Werkzeugausführung mit dem Werkzeugergebnis-Inhalt ausgegeben, der an Claude zurückgesendet wird. Wird auch für alle Benutzereingaben ausgegeben, die Sie mid-loop streamen.
* **`StreamEvent`:** wird nur ausgegeben, wenn Teilteilnachrichten aktiviert sind. Enthält rohe API-Streaming-Ereignisse (Text-Deltas, Werkzeug-Input-Chunks). Siehe [Stream-Antworten](/docs/de/agent-sdk/streaming-output).
* **`ResultMessage`:** markiert das Ende der Agent-Schleife. Enthält das endgültige Textergebnis, Token-Nutzung, Kosten und Sitzungs-ID. Überprüfen Sie das Feld `subtype`, um zu bestimmen, ob die Aufgabe erfolgreich war oder ein Limit erreicht hat. Eine kleine Anzahl von nachfolgenden Systemevenementen, wie `prompt_suggestion`, können danach ankommen, daher sollten Sie den Stream bis zum Ende durchlaufen, anstatt beim Ergebnis zu unterbrechen. Siehe [Ergebnis verarbeiten](#handle-the-result).

Diese fünf Typen decken den vollständigen Lebenszyklus der Agent-Schleife ab. Beide SDKs geben auch Observability-Ereignisse wie Ratenlimit-Status und Task-Benachrichtigungen aus, die nicht erforderlich sind, um die Schleife zu steuern. Siehe die [Python-Nachrichtentypen-Referenz](/docs/de/agent-sdk/python#message-types) und [TypeScript-Nachrichtentypen-Referenz](/docs/de/agent-sdk/typescript#message-types) für die vollständigen Listen.

<h3 id="handle-messages">
  Nachrichten verarbeiten
</h3>

Welche Nachrichten Sie verarbeiten, hängt davon ab, was Sie erstellen:

* **Nur endgültige Ergebnisse:** verarbeiten Sie `ResultMessage`, um die Ausgabe, Kosten und ob die Aufgabe erfolgreich war oder ein Limit erreicht hat, zu erhalten.
* **Fortschritts-Updates:** verarbeiten Sie `AssistantMessage`, um zu sehen, was Claude jede Runde tut, einschließlich welche Werkzeuge es aufgerufen hat.
* **Live-Streaming:** aktivieren Sie Teilteilnachrichten (`include_partial_messages` in Python, `includePartialMessages` in TypeScript), um `StreamEvent`-Nachrichten in Echtzeit zu erhalten. Siehe [Stream-Antworten in Echtzeit](/docs/de/agent-sdk/streaming-output).

Wie Sie Nachrichtentypen überprüfen, hängt vom SDK ab:

* **Python:** überprüfen Sie Nachrichtentypen mit `isinstance()` gegen Klassen, die aus `claude_agent_sdk` importiert wurden (zum Beispiel, `isinstance(message, ResultMessage)`).
* **TypeScript:** überprüfen Sie das Feld `type` string (zum Beispiel, `message.type === "result"`). `AssistantMessage` und `UserMessage` umhüllen die rohe API-Nachricht in einem `.message`-Feld, daher befinden sich Inhaltsblöcke bei `message.message.content`, nicht bei `message.content`.

<Accordion title="Beispiel: Nachrichtentypen überprüfen und Ergebnisse verarbeiten">
  <CodeGroup>
    ```python Python theme={null}
    import asyncio
    from claude_agent_sdk import query, AssistantMessage, ResultMessage, TextBlock, ToolUseBlock


    async def main():
        try:
            async for message in query(prompt="Summarize this project"):
                if isinstance(message, AssistantMessage):
                    # Each AssistantMessage carries one content block
                    for block in message.content:
                        if isinstance(block, TextBlock):
                            print(f"Claude: {block.text}")
                        elif isinstance(block, ToolUseBlock):
                            print(f"Tool call: {block.name}")
                if isinstance(message, ResultMessage):
                    if message.subtype == "success":
                        print(message.result)
                    else:
                        print(f"Stopped: {message.subtype}")
        except Exception as error:
            # A single-shot query() raises after yielding an error result. If the
            # failure was an error result, the error subtype branches above have
            # already run; connection or process failures yield no result message.
            print(f"Session ended with an error: {error}")


    asyncio.run(main())
    ```

    ```typescript TypeScript theme={null}
    import { query } from "@anthropic-ai/claude-agent-sdk";

    try {
      for await (const message of query({ prompt: "Summarize this project" })) {
        if (message.type === "assistant") {
          // Each assistant message carries one content block
          for (const block of message.message.content) {
            if (block.type === "text") {
              console.log(`Claude: ${block.text}`);
            } else if (block.type === "tool_use") {
              console.log(`Tool call: ${block.name}`);
            }
          }
        }
        if (message.type === "result") {
          if (message.subtype === "success") {
            console.log(message.result);
          } else {
            console.log(`Stopped: ${message.subtype}`);
          }
        }
      }
    } catch (error) {
      // A single-shot query() throws after yielding an error result. If the
      // failure was an error result, the error subtype branches above have
      // already run; connection or process failures yield no result message.
      console.log(`Session ended with an error: ${error}`);
    }
    ```
  </CodeGroup>
</Accordion>

<h2 id="tool-execution">
  Werkzeugausführung
</h2>

Werkzeuge geben Ihrem Agent die Möglichkeit, Maßnahmen zu ergreifen. Ohne Werkzeuge kann Claude nur mit Text antworten. Mit Werkzeugen kann Claude Dateien lesen, Befehle ausführen, Code durchsuchen und mit externen Diensten interagieren.

<h3 id="built-in-tools">
  Integrierte Werkzeuge
</h3>

Das SDK enthält die gleichen Werkzeuge, die Claude Code antreiben:

| Kategorie          | Werkzeuge                                                       | Was sie tun                                                                       |
| :----------------- | :-------------------------------------------------------------- | :-------------------------------------------------------------------------------- |
| **Dateivorgänge**  | `Read`, `Edit`, `Write`                                         | Dateien lesen, ändern und erstellen                                               |
| **Suche**          | `Glob`, `Grep`                                                  | Dateien nach Muster finden, Inhalte mit Regex durchsuchen                         |
| **Ausführung**     | `Bash`                                                          | Shell-Befehle, Skripte, Git-Vorgänge ausführen                                    |
| **Web**            | `WebSearch`, `WebFetch`                                         | Das Web durchsuchen, Seiten abrufen und analysieren                               |
| **Erkennung**      | `ToolSearch`                                                    | Werkzeuge dynamisch finden und bei Bedarf laden, anstatt alle vorab zu laden      |
| **Orchestrierung** | `Agent`, `Skill`, `AskUserQuestion`, `TaskCreate`, `TaskUpdate` | Subagenten spawnen, Fähigkeiten aufrufen, den Benutzer fragen, Aufgaben verfolgen |

Auf den [Modellen, die die Aufgabenverfolgungswerkzeuge nicht erhalten](/docs/de/agent-sdk/todo-tracking#model-availability), stellt Claude Code `TaskCreate` und `TaskUpdate` nur bereit, wenn Sie sich dafür entscheiden.

Über integrierte Werkzeuge hinaus können Sie:

* **Externe Dienste verbinden** mit [MCP-Servern](/docs/de/agent-sdk/mcp) (Datenbanken, Browser, APIs)
* **Benutzerdefinierte Werkzeuge definieren** mit [benutzerdefinierten Werkzeug-Handlern](/docs/de/agent-sdk/custom-tools)
* **Projekt-Fähigkeiten laden** über [Einstellungsquellen](/docs/de/agent-sdk/claude-code-features) für wiederverwendbare Workflows

<h3 id="tool-permissions">
  Werkzeugberechtigungen
</h3>

Claude bestimmt, welche Werkzeuge aufgerufen werden sollen, basierend auf der Aufgabe, aber Sie kontrollieren, ob diese Aufrufe ausgeführt werden dürfen. Sie können bestimmte Werkzeuge automatisch genehmigen, andere vollständig blockieren oder Genehmigung für alles verlangen. Drei Optionen arbeiten zusammen, um zu bestimmen, was ausgeführt wird:

* **`allowed_tools` / `allowedTools`** genehmigt automatisch aufgelistete Werkzeuge. Ein schreibgeschützter Agent mit `["Read", "Glob", "Grep"]` in seiner Werkzeugliste für zulässige Werkzeuge führt diese Werkzeuge ohne Aufforderung aus. Werkzeuge, die nicht aufgelistet sind, sind immer noch verfügbar, und Aufrufe an sie, die Genehmigung benötigen, fallen durch zum Berechtigungsmodus und `canUseTool`.
* **`disallowed_tools` / `disallowedTools`** blockiert aufgelistete Werkzeuge, unabhängig von anderen Einstellungen. Siehe [Berechtigungen](/docs/de/agent-sdk/permissions) für die Reihenfolge, in der Regeln überprüft werden, bevor ein Werkzeug ausgeführt wird.
* **`permission_mode` / `permissionMode`** kontrolliert, wie viel menschliche Überwachung Sie wünschen. Das SDK wertet den aktiven Modus zusammen mit Ihren Zulassungs- und Ablehnungsregeln in einer festen Reihenfolge aus, die in [Wie Berechtigungen ausgewertet werden](/docs/de/agent-sdk/permissions#how-permissions-are-evaluated) beschrieben ist. Siehe [Berechtigungsmodus](#permission-mode) für verfügbare Modi.

Sie können auch einzelne Werkzeuge mit Regeln wie `"Bash(npm *)"` einschränken, um nur bestimmte Befehle zuzulassen. Siehe [Berechtigungen](/docs/de/agent-sdk/permissions) für die vollständige Regelsyntax.

Wenn ein Werkzeug abgelehnt wird, erhält Claude eine Ablehnungsnachricht als Werkzeugergebnis und versucht normalerweise einen anderen Ansatz oder meldet, dass es nicht fortfahren konnte.

<h3 id="parallel-tool-execution">
  Parallele Werkzeugausführung
</h3>

Wenn Claude mehrere Werkzeugaufrufe in einer einzelnen Runde anfordert, können beide SDKs sie je nach Werkzeug gleichzeitig oder sequenziell ausführen. Schreibgeschützte Werkzeuge (wie `Read`, `Glob`, `Grep` und MCP-Werkzeuge, die als schreibgeschützt gekennzeichnet sind) können gleichzeitig ausgeführt werden. Werkzeuge, die den Status ändern (wie `Edit`, `Write` und `Bash`), werden sequenziell ausgeführt, um Konflikte zu vermeiden.

Benutzerdefinierte Werkzeuge verwenden standardmäßig sequenzielle Ausführung. Um parallele Ausführung für ein benutzerdefiniertes Werkzeug zu aktivieren, setzen Sie `readOnlyHint` in seinen Anmerkungen. Beide [TypeScript](/docs/de/agent-sdk/typescript#tool) und [Python](/docs/de/agent-sdk/python#tool) SDKs verwenden diesen Feldnamen aus dem MCP SDK.

<h2 id="control-how-the-loop-runs">
  Steuern Sie, wie die Schleife ausgeführt wird
</h2>

Sie können begrenzen, wie viele Durchläufe die Schleife durchführt, wie viel sie kostet, wie tiefgreifend Claude denkt, und ob Tools vor der Ausführung genehmigt werden müssen. All diese sind Felder auf [`ClaudeAgentOptions`](/docs/de/agent-sdk/python#claudeagentoptions) (Python) / [`Options`](/docs/de/agent-sdk/typescript#options) (TypeScript).

<h3 id="turns-and-budget">
  Durchläufe und Budget
</h3>

| Option                                          | Was es steuert                | Standard         |
| :---------------------------------------------- | :---------------------------- | :--------------- |
| Max. Durchläufe (`max_turns` / `maxTurns`)      | Maximale Tool-Use-Rundgänge   | Keine Begrenzung |
| Max. Budget (`max_budget_usd` / `maxBudgetUsd`) | Maximale Kosten vor dem Stopp | Keine Begrenzung |

Wenn eine dieser Grenzen erreicht wird, gibt das SDK eine `ResultMessage` mit einem entsprechenden Fehler-Subtyp (`error_max_turns` oder `error_max_budget_usd`) zurück. Siehe [Handle the result](#handle-the-result) für die Überprüfung dieser Subtypen und [`ClaudeAgentOptions`](/docs/de/agent-sdk/python#claudeagentoptions) / [`Options`](/docs/de/agent-sdk/typescript#options) für die Syntax.

Die Budget-Obergrenze deckt [Subagenten](/docs/de/agent-sdk/subagents) ab: ihre Ausgaben zählen zur Gesamtsumme. Sobald die Ausgaben die Obergrenze erreichen, schlägt das Spawnen eines weiteren Subagenten mit `Budget limit reached` fehl, und Claude Code stoppt alle noch laufenden Hintergrund-Subagenten. Die Durchsetzungsverhalten der Obergrenze erfordern Claude Code v2.1.217 oder später.

Mit [Streaming-Eingabe](/docs/de/agent-sdk/streaming-vs-single-mode) bleibt eine Nachricht, die noch in der Warteschlange steht, wenn ein Durchlauf bei der max-turns-Grenze endet, in der Warteschlange. Claude Code fügt sie nicht zum letzten Modellaufruf dieses Durchlaufs hinzu. Es startet einen neuen Durchlauf für die Nachricht, und die max-turns-Zählung beginnt für diesen Durchlauf von vorne. Das Budget-Gesamtbudget wird über Nachrichten hinweg kumuliert, und sobald die Ausgaben `maxBudgetUsd` erreichen, enden spätere Nachrichten in derselben Konversation mit dem `error_max_budget_usd`-Ergebnis. Ein [`/clear`](/docs/de/agent-sdk/cost-tracking) startet das Budget neu.

<h3 id="effort-level">
  Anstrengungsgrad
</h3>

Die `effort`-Option steuert, wie viel Reasoning Claude anwendet. Niedrigere Anstrengungsgrade verwenden weniger Token pro Durchlauf und reduzieren die Kosten. Nicht alle Modelle unterstützen den effort-Parameter. Siehe [Effort](https://platform.claude.com/docs/en/build-with-claude/effort) für die Modelle, die ihn unterstützen.

| Stufe      | Verhalten                               | Geeignet für                                                                                                        |
| :--------- | :-------------------------------------- | :------------------------------------------------------------------------------------------------------------------ |
| `"low"`    | Minimales Reasoning, schnelle Antworten | Datei-Lookups, Verzeichnisse auflisten                                                                              |
| `"medium"` | Ausgewogenes Reasoning                  | Routinemäßige Bearbeitungen, Standardaufgaben                                                                       |
| `"high"`   | Gründliche Analyse                      | Refaktorisierungen, Debugging                                                                                       |
| `"xhigh"`  | Erweiterte Reasoning-Tiefe              | Coding- und agentengestützte Aufgaben auf den [Modellen, die es unterstützen](/docs/de/model-config#adjust-effort-level) |
| `"max"`    | Maximale Reasoning-Tiefe                | Mehrstufige Probleme, die tiefe Analyse erfordern                                                                   |

Wenn Sie `effort` nicht setzen, löst Claude Code den Anstrengungsgrad selbst auf, in der Reihenfolge, die [Adjust effort level](/docs/de/model-config#adjust-effort-level) beschreibt.

<Note>
  `effort` tauscht Latenz und Token-Kosten gegen Reasoning-Tiefe innerhalb jeder Antwort. [Extended thinking](https://platform.claude.com/docs/en/build-with-claude/extended-thinking) ist eine separate Funktion, die `thinking`-Blöcke in der Ausgabe erzeugt, und das `display`-Feld auf `ThinkingConfig` für [Python](/docs/de/agent-sdk/python#thinkingconfig) oder [TypeScript](/docs/de/agent-sdk/typescript#thinkingconfig) steuert, ob Sie deren Text erhalten. Sie sind unabhängig: Sie können `effort: "low"` mit aktiviertem Extended Thinking setzen oder `effort: "max"` ohne es.
</Note>

Verwenden Sie niedrigere Anstrengung für Agenten, die einfache, gut definierte Aufgaben ausführen (wie das Auflisten von Dateien oder das Ausführen eines einzelnen grep), um Kosten und Latenz zu reduzieren. Setzen Sie `effort` in den Top-Level-`query()`-Optionen für die gesamte Sitzung oder pro Subagent mit dem `effort`-Feld auf [`AgentDefinition`](/docs/de/agent-sdk/subagents#agentdefinition-configuration), um die Sitzungsebene zu überschreiben.

<h3 id="permission-mode">
  Genehmigungsmodus
</h3>

Die Genehmigungsmodus-Option (`permission_mode` in Python, `permissionMode` in TypeScript) steuert, ob der Agent vor der Verwendung von Tools um Genehmigung fragt:

| Modus                 | Verhalten                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | Anwendungsfall                                                                                                                                                                      |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `"default"`           | Tool-Aufrufe, die Genehmigung benötigen und nicht durch Allow-Regeln abgedeckt sind, lösen Ihren `canUseTool`-Callback aus; kein Callback bedeutet Ablehnung                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | Interaktive Anwendungen mit einem benutzerdefinierten Genehmigungsrückruf                                                                                                           |
| `"acceptEdits"`       | Genehmigt automatisch Dateibearbeitungen und häufige Dateisystem-Befehle (`mkdir`, `touch`, `mv`, `cp` usw.); andere Bash-Befehle folgen Standardregeln                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | Sie vertrauen Claudes Bearbeitungen und möchten schnellere Iterationen, z. B. während des Prototyping oder beim Arbeiten in einem isolierten Verzeichnis                            |
| `"plan"`              | Claude erkundet und plant, ohne Ihre Quelldateien zu bearbeiten; Dateibearbeitungen werden nie automatisch genehmigt und werden durch Ihren `canUseTool`-Callback angefordert                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Sie möchten, dass Claude Änderungen vorschlägt, ohne sie auszuführen, z. B. während einer Code-Überprüfung oder wenn Sie Änderungen genehmigen müssen, bevor sie vorgenommen werden |
| `"dontAsk"`           | Fragt nie. Tools, die durch [Genehmigungsregeln](/docs/de/settings-reference#permission-settings) vorab genehmigt sind, werden ausgeführt, ebenso wie Aufrufe, die im `default`-Modus keine Genehmigung benötigen, z. B. Datei-Lesevorgänge in Ihren Arbeitsverzeichnissen; jeder Aufruf, der sonst eine Aufforderung auslösen würde, wird abgelehnt. `AskUserQuestion`, Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und MCP-Tools, die als [`requiresUserInteraction`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, werden auch dann abgelehnt, wenn Sie sie erlaubt haben                                                                                                                                                                                                                       | Sie möchten eine feste, explizite Tool-Oberfläche für einen Headless-Agent und bevorzugen eine harte Ablehnung gegenüber stiller Abhängigkeit davon, dass `canUseTool` abwesend ist |
| `"auto"`              | Verwendet einen Modell-Klassifizierer, um Genehmigungsaufforderungen zu genehmigen oder abzulehnen. Siehe [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) für Verfügbarkeit und Verhalten                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Autonome Agenten, die immer noch Sicherheitsvorkehrungen bei der Tool-Nutzung wünschen                                                                                              |
| `"bypassPermissions"` | Führt alle zulässigen Tools aus, ohne zu fragen, außer Tools, die einer expliziten [`ask`-Regel](/docs/de/settings-reference#permission-settings) entsprechen, Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und Tools, die Benutzerinteraktion erfordern. Die [Cross-Session-Messaging-Schutzmaßnahmen](/docs/de/permission-modes#skip-all-checks-with-bypasspermissions-mode) gelten weiterhin. Siehe [How permissions are evaluated](/docs/de/agent-sdk/permissions#how-permissions-are-evaluated) für die Rangfolge. Im TypeScript SDK ist auch `allowDangerouslySkipPermissions: true` in `options` erforderlich. Kann nicht verwendet werden, wenn als Root auf Unix ausgeführt wird. Verwenden Sie nur in isolierten Umgebungen, in denen die Aktionen des Agenten keine Systeme beeinflussen können, die Ihnen wichtig sind | CI, Container oder andere isolierte Umgebungen                                                                                                                                      |

Verwenden Sie für interaktive Anwendungen `"default"` mit einem Tool-Genehmigungsrückruf, um Genehmigungsaufforderungen anzuzeigen. Für autonome Agenten auf einem Dev-Computer genehmigt `"acceptEdits"` automatisch Dateibearbeitungen und häufige Dateisystem-Befehle (`mkdir`, `touch`, `mv`, `cp` usw.), während andere `Bash`-Befehle immer noch durch Allow-Regeln kontrolliert werden. Reservieren Sie `"bypassPermissions"` für CI, Container oder andere isolierte Umgebungen. Siehe [Permissions](/docs/de/agent-sdk/permissions) für vollständige Details.

<h3 id="model">
  Modell
</h3>

Setzen Sie die `model`-Option, um auszuwählen, welches Modell die Sitzung ausführt. Weitere Informationen finden Sie unter [Choose a model](/docs/de/agent-sdk/configuration#choose-a-model).

<h2 id="the-context-window">
  Das Kontextfenster
</h2>

Das Kontextfenster ist die Gesamtmenge an Informationen, die Claude während einer Sitzung zur Verfügung stehen. Es wird nicht zwischen Runden innerhalb einer Sitzung zurückgesetzt. Alles sammelt sich an: der System-Prompt, Werkzeugdefinitionen, Gesprächsverlauf, Werkzeugeingaben und Werkzeugergebnisse. Inhalte, die über Runden hinweg gleich bleiben (System-Prompt, Werkzeugdefinitionen, CLAUDE.md), werden automatisch [prompt-gecacht](https://platform.claude.com/docs/de/build-with-claude/prompt-caching), was Kosten und Latenz für wiederholte Präfixe reduziert. Informationen dazu, wie ein benutzerdefinierter System-Prompt oder `append`-Text die Cache-Wiederverwendung über Sitzungen hinweg beeinflusst, finden Sie unter [Modifying system prompts](/docs/de/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines).

<h3 id="what-consumes-context">
  Was Kontext verbraucht
</h3>

Hier ist, wie jede Komponente den Kontext im SDK beeinflusst:

| Quelle                       | Wann es lädt                                                               | Auswirkung                                                                                                                                                                                                                                                                                                                                                                        |
| :--------------------------- | :------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **System-Prompt**            | Jede Anfrage                                                               | Kleine feste Kosten, immer vorhanden                                                                                                                                                                                                                                                                                                                                              |
| **CLAUDE.md-Dateien**        | Sitzungsstart, über [`settingSources`](/docs/de/agent-sdk/claude-code-features) | Vollständiger Inhalt in jeder Anfrage (aber prompt-gecacht, daher zahlt nur die erste Anfrage die vollständigen Kosten)                                                                                                                                                                                                                                                           |
| **Werkzeugdefinitionen**     | Jede Anfrage; MCP-Schemas standardmäßig aufgeschoben                       | Integrierte Werkzeugschemas werden bei jeder Anfrage geladen. [Werkzeugsuche](/docs/de/agent-sdk/mcp#mcp-tool-search) schiebt MCP-Werkzeugschemas standardmäßig auf und fällt auf vorherige Laden auf nicht unterstützten Modellen und bestimmten Plattformen zurück. Siehe [Werkzeugsuche konfigurieren](/docs/de/agent-sdk/tool-search#configure-tool-search) für die vollständige Matrix |
| **Gesprächsverlauf**         | Sammelt sich über Runden an                                                | Wächst mit jeder Runde: Prompts, Antworten, Werkzeugeingaben, Werkzeugergebnisse                                                                                                                                                                                                                                                                                                  |
| **Fähigkeitsbeschreibungen** | Sitzungsstart, über Einstellungsquellen                                    | Kurze Zusammenfassungen; vollständiger Inhalt lädt nur bei Aufruf                                                                                                                                                                                                                                                                                                                 |

Große Werkzeugergebnisse verbrauchen erheblichen Kontext. Das Lesen einer großen Datei oder das Ausführen eines Befehls mit ausführlicher Ausgabe kann Tausende von Token in einer einzelnen Runde verwenden. Der Kontext sammelt sich über Runden an, daher bauen längere Sitzungen mit vielen Werkzeugaufrufen erheblich mehr Kontext auf als kurze.

<h3 id="automatic-compaction">
  Automatische Komprimierung
</h3>

Wenn sich das Kontextfenster seinem Limit nähert, komprimiert das SDK automatisch das Gespräch: Es fasst ältere Verlauf zusammen, um Platz freizugeben, während Ihre neuesten Austausche und wichtigen Entscheidungen intakt bleiben. Das SDK gibt eine Nachricht mit `type: "system"` und `subtype: "compact_boundary"` im Stream aus, wenn dies geschieht (in Python ist dies eine `SystemMessage`; in TypeScript ist es ein separater `SDKCompactBoundaryMessage`-Typ).

Die Komprimierung ersetzt ältere Nachrichten durch eine Zusammenfassung, daher können spezifische Anweisungen von früh im Gespräch möglicherweise nicht beibehalten werden. Persistente Regeln gehören in CLAUDE.md (geladen über [`settingSources`](/docs/de/agent-sdk/claude-code-features)) statt in den anfänglichen Prompt, da CLAUDE.md-Inhalte bei jeder Anfrage erneut eingespritzt werden.

Sie können das Komprimierungsverhalten auf mehrere Arten anpassen:

* **Zusammenfassungsanweisungen in CLAUDE.md:** Der Kompressor liest Ihre CLAUDE.md wie jeden anderen Kontext, daher können Sie einen Abschnitt einschließen, der ihm sagt, was beim Zusammenfassen zu bewahren ist. Der Kompressor passt auf Absicht an, daher ist der Abschnittskopf frei.
* **`PreCompact`-Hook:** Führen Sie benutzerdefinierte Logik vor der Komprimierung aus, zum Beispiel um das vollständige Transkript zu archivieren. Der Hook erhält ein `trigger`-Feld (`manual` oder `auto`). Siehe [Hooks](/docs/de/agent-sdk/hooks).
* **Manuelle Komprimierung:** Senden Sie `/compact` als Prompt-String, um Komprimierung bei Bedarf auszulösen. Befehle, die auf diese Weise gesendet werden, sind SDK-Eingaben. Siehe [dispatch commands by name](/docs/de/agent-sdk/skills#dispatch-commands-by-name).

<Accordion title="Beispiel: Zusammenfassungsanweisungen in CLAUDE.md">
  Fügen Sie einen Abschnitt zu Ihrer Projekt-CLAUDE.md hinzu, der dem Kompressor sagt, was zu bewahren ist. Der Kopfname ist nicht speziell; verwenden Sie ein beliebiges klares Label.

  ```markdown CLAUDE.md theme={null}
  # Summary instructions

  When summarizing this conversation, always preserve:
  - The current task objective and acceptance criteria
  - File paths that have been read or modified
  - Test results and error messages
  - Decisions made and the reasoning behind them
  ```
</Accordion>

<h3 id="keep-context-efficient">
  Kontext effizient halten
</h3>

Ein paar Strategien für langfristig laufende Agenten:

* **Verwenden Sie Subagenten für Unteraufgaben.** Jeder Subagent startet mit einem frischen Gespräch (kein vorheriger Nachrichtenverlauf, obwohl er seinen eigenen System-Prompt und Projekt-Kontext wie CLAUDE.md lädt). Er sieht nicht die Runden des Elternteils, und nur seine endgültige Antwort kehrt zum Elternteil als Werkzeugergebnis zurück. Der Kontext des Hauptagenten wächst um diese Zusammenfassung, nicht um das vollständige Unteraufgaben-Transkript. Siehe [Was Subagenten erben](/docs/de/agent-sdk/subagents#what-subagents-inherit) für Details.
* **Seien Sie selektiv mit Werkzeugen.** Jede Werkzeugdefinition nimmt Kontextraum ein. Verwenden Sie das Feld `tools` auf [`AgentDefinition`](/docs/de/agent-sdk/subagents#agentdefinition-configuration), um Subagenten auf die minimale Menge zu beschränken, die sie benötigen.
* **Beobachten Sie MCP-Server-Kosten.** [MCP tool search](/docs/de/agent-sdk/mcp#mcp-tool-search) schiebt MCP-Werkzeugschemas standardmäßig auf und lädt sie bei Bedarf. Wenn die Werkzeugsuche ausgeschaltet ist oder auf vorherige Laden auf nicht unterstützten Modellen und bestimmten Plattformen zurückgefallen ist, fügt jeder MCP-Server alle seine Werkzeugschemas zu jeder Anfrage hinzu, daher können ein paar Server mit vielen Werkzeugen erheblichen Kontext verbrauchen, bevor der Agent irgendwelche Arbeiten verrichtet. Siehe [Configure tool search](/docs/de/agent-sdk/tool-search#configure-tool-search) für die Konfigurationen, bei denen das Fallback angewendet wird.
* **Verwenden Sie niedrigere Anstrengung für Routine-Aufgaben.** Setzen Sie [Anstrengung](#effort-level) auf `"low"` für Agenten, die nur Dateien lesen oder Verzeichnisse auflisten müssen. Dies reduziert Token-Nutzung und Kosten.

Für eine detaillierte Aufschlüsselung der Pro-Feature-Kontextkosten siehe [Kontextkosten verstehen](/docs/de/features-overview#understand-context-costs).

<h2 id="sessions-and-continuity">
  Sitzungen und Kontinuität
</h2>

Jede Interaktion mit dem SDK erstellt oder setzt eine Sitzung fort. Erfassen Sie die Sitzungs-ID aus `ResultMessage.session_id` (verfügbar in beiden SDKs), um später fortzufahren. Das TypeScript SDK macht es auch als direktes Feld auf der Init-`SystemMessage` verfügbar; in Python ist es in `SystemMessage.data` verschachtelt.

Wenn Sie fortfahren, wird der vollständige Kontext aus vorherigen Runden wiederhergestellt: Dateien, die gelesen wurden, Analysen, die durchgeführt wurden, und Aktionen, die ergriffen wurden. Sie können auch eine Sitzung forken, um in einen anderen Ansatz zu verzweigen, ohne das Original zu ändern.

Siehe [Sitzungsverwaltung](/docs/de/agent-sdk/sessions) für den vollständigen Leitfaden zu Resume-, Continue- und Fork-Mustern. Um Sitzungen über zustandslose Container oder serverlose Hosts hinweg fortzusetzen, übergeben Sie einen [`session_store` / `sessionStore` Adapter](/docs/de/agent-sdk/session-storage), damit das SDK Transkripte in Ihr eigenes Backend spiegelt und ein anderer Host sie fortsetzen kann. Der Claude Code Unterprozess schreibt weiterhin zuerst auf die lokale Festplatte. Siehe [Dual-Write-Architektur](/docs/de/agent-sdk/session-storage#dual-write-architecture) für die Informationen, welche Kopie eine neue Sitzung überlebt versus einen aus dem Store fortgesetzten Lauf, und wie Sie die lokale Kopie kurzlebig halten.

<Note>
  In Python verwaltet `ClaudeSDKClient` Sitzungs-IDs automatisch über mehrere Aufrufe hinweg. Siehe die [Python SDK-Referenz](/docs/de/agent-sdk/python#choosing-between-query-and-claudesdkclient) für Details.
</Note>

<h2 id="handle-the-result">
  Ergebnis verarbeiten
</h2>

Wenn die Schleife endet, sagt Ihnen die `ResultMessage`, was passiert ist, und gibt Ihnen die Ausgabe. Das Feld `subtype` (verfügbar in beiden SDKs) ist die primäre Methode, um den Beendigungsstatus zu überprüfen.

| Ergebnis-Subtyp                       | Was passiert ist                                                                                                                                                                                                                             | Feld `result` verfügbar? |
| :------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------: |
| `success`                             | Claude hat die Aufgabe normal abgeschlossen                                                                                                                                                                                                  |            Ja            |
| `error_max_turns`                     | Hat das `maxTurns`-Limit erreicht, bevor es fertig wurde                                                                                                                                                                                     |           Nein           |
| `error_max_budget_usd`                | Hat das `maxBudgetUsd`-Limit erreicht, bevor es fertig wurde                                                                                                                                                                                 |           Nein           |
| `error_during_execution`              | Ein Fehler unterbrach die Schleife (zum Beispiel eine abgebrochene Anfrage)                                                                                                                                                                  |           Nein           |
| `error_max_structured_output_retries` | Keine gültige strukturierte Ausgabe wurde innerhalb des konfigurierten Wiederholungslimits erzeugt: jeder Versuch schlug die Validierung fehl, oder ein Modell-Fallback zog die abgeschlossene Ausgabe ohne erfolgreiche Wiederholung zurück |           Nein           |

Das Feld `result` enthält die endgültige Textausgabe und ist nur in der `success`-Variante vorhanden, daher überprüfen Sie immer den Subtyp, bevor Sie es lesen.

Alle Ergebnis-Subtypen enthalten `total_cost_usd`, `usage`, `num_turns` und `session_id`, daher können Sie Kosten verfolgen und fortfahren, auch nach Fehlern. Achten Sie auf diese Fälle:

* Nach einem Sitzungsabsturz ist das endgültige Ergebnis ein `error_during_execution`, dessen Kostenfelder möglicherweise auf Null gesetzt sind und dessen `stop_reason` `null` ist, und der Prozess beendet sich nach dem Ausgeben. Siehe [Gesamtsummen nach einem Sitzungsabsturz wiederherstellen](/docs/de/agent-sdk/cost-tracking#recover-totals-after-a-session-crash).
* In Python sind `total_cost_usd`, `usage` und `model_usage` als optional typisiert, daher überprüfen Sie, dass sie nicht `None` sind, bevor Sie sie lesen.

Das Feld `usage` deckt nur die Hauptagentenschleife ab. Verwenden Sie `modelUsage` oder `model_usage` in Python für die Gesamtbaum-Token- und Kostenabrechnung. Siehe [Kosten und Nutzung verfolgen](/docs/de/agent-sdk/cost-tracking) für Details zur Interpretation der `usage`-Felder.

<Note>
  Wenn eine Abfrage mit einem Fehler-Ergebnis endet:

  * Ein einzelner `query()`-Aufruf liefert die endgültige Ergebnis-Nachricht und löst dann einen Fehler aus, der den Fehlertext enthält, wie z. B. `Reached maximum number of turns`. Das Auslösen ist beabsichtigt. Umhüllen Sie die Schleife mit einem Try-Block, wenn Ihr Code danach fortfahren muss. Der zugrunde liegende Claude Code-Prozess beendet sich auch mit einem Nicht-Null-Code.
  * Eine Streaming-Eingabe-Sitzung bleibt aktiv, und Sie können weiterhin Nachrichten senden, außer nach einem Sitzungsabsturz, der ein endgültiges `error_during_execution`-Ergebnis ausgibt und den Prozess beendet.
</Note>

Das Ergebnis enthält auch ein Feld `stop_reason` (`string | null` in TypeScript, `str | None` in Python), das angibt, warum das Modell bei seiner endgültigen Runde die Generierung gestoppt hat. Häufige Werte sind `end_turn` (Modell fertig normal), `max_tokens` (hat das Ausgabe-Token-Limit erreicht) und `refusal` (das Modell lehnte die Anfrage ab). Bei Fehler-Ergebnissen, die die Schleife erzeugt hat, trägt `stop_reason` den Wert aus der letzten Assistenten-Antwort, bevor die Schleife endete; das Ergebnis, das Claude Code nach einem Sitzungsabsturz synthetisiert, trägt `null`.

Um Ablehnungen zu erkennen, überprüfen Sie `stop_reason === "refusal"` (TypeScript) oder `stop_reason == "refusal"` (Python). Siehe [`SDKResultMessage`](/docs/de/agent-sdk/typescript#sdkresultmessage) (TypeScript) oder [`ResultMessage`](/docs/de/agent-sdk/python#resultmessage) (Python) für den vollständigen Typ.

<h2 id="hooks">
  Hooks
</h2>

[Hooks](/docs/de/agent-sdk/hooks) sind Rückrufe, die an bestimmten Punkten in der Schleife ausgelöst werden: bevor ein Werkzeug läuft, nachdem es zurückkommt, wenn der Agent fertig ist, und so weiter. Einige häufig verwendete Hooks sind:

| Hook                             | Wann es ausgelöst wird                           | Häufige Verwendungen                                             |
| :------------------------------- | :----------------------------------------------- | :--------------------------------------------------------------- |
| `PreToolUse`                     | Bevor ein Werkzeug ausgeführt wird               | Eingaben validieren, gefährliche Befehle blockieren              |
| `PostToolUse`                    | Nachdem ein Werkzeug zurückkommt                 | Ausgaben prüfen, Nebenwirkungen auslösen                         |
| `UserPromptSubmit`               | Wenn ein Prompt gesendet wird                    | Zusätzlichen Kontext in Prompts injizieren                       |
| `Stop`                           | Wenn der Agent fertig ist                        | Ergebnis validieren, Sitzungsstatus speichern                    |
| `SubagentStart` / `SubagentStop` | Wenn ein Subagent spawnt oder abgeschlossen wird | Parallele Task-Ergebnisse verfolgen und aggregieren              |
| `PreCompact`                     | Bevor Kontext-Komprimierung auftritt             | Vollständiges Transkript archivieren, bevor zusammengefasst wird |

Hooks laufen in Ihrem Anwendungsprozess, nicht im Kontext des Agenten, daher verbrauchen sie keinen Kontext. Hooks können auch die Schleife kurzschließen: ein `PreToolUse`-Hook, der einen Werkzeugaufruf ablehnt, verhindert seine Ausführung, und Claude erhält stattdessen die Ablehnungsnachricht.

Beide SDKs unterstützen alle oben genannten Ereignisse. Das TypeScript SDK enthält zusätzliche Ereignisse, die Python noch nicht unterstützt. Siehe [Ausführung mit Hooks kontrollieren](/docs/de/agent-sdk/hooks) für die vollständige Ereignisliste, Pro-SDK-Verfügbarkeit und die vollständige Callback-API.

<h2 id="put-it-all-together">
  Alles zusammenbringen
</h2>

Dieses Beispiel kombiniert die Schlüsselkonzepte von dieser Seite in einen einzelnen Agent, der fehlgeschlagene Tests behebt. Es konfiguriert den Agent mit zulässigen Werkzeugen (automatisch genehmigt, damit der Agent autonom läuft), Projekteinstellungen und Sicherheitslimits für Runden und Anstrengungsgrad. Während die Schleife läuft, erfasst sie die Sitzungs-ID für mögliche Wiederaufnahme, verarbeitet das endgültige Ergebnis und gibt die Gesamtkosten aus.

Da ein einzelner `query()`-Aufruf nach dem Ausgeben eines Fehlerergebnisses eine Ausnahme auslöst, ist die Schleife in einen Try-Block eingewickelt, damit das Skript sauber beendet wird, wenn ein Limit erreicht wird.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def run_agent():
      session_id = None

      try:
          async for message in query(
              prompt="Find and fix the bug causing test failures in the auth module",
              options=ClaudeAgentOptions(
                  allowed_tools=[
                      "Read",
                      "Edit",
                      "Bash",
                      "Glob",
                      "Grep",
                  ],  # Listing tools here auto-approves them (no prompting)
                  setting_sources=[
                      "project"
                  ],  # Load CLAUDE.md, skills, hooks from current directory
                  max_turns=30,  # Prevent runaway sessions
                  effort="high",  # Thorough reasoning for complex debugging
              ),
          ):
              # Handle the final result
              if isinstance(message, ResultMessage):
                  session_id = message.session_id  # Save for potential resumption

                  if message.subtype == "success":
                      print(f"Done: {message.result}")
                  elif message.subtype == "error_max_turns":
                      # Agent ran out of turns. Resume with a higher limit.
                      print(f"Hit turn limit. Resume session {session_id} to continue.")
                  elif message.subtype == "error_max_budget_usd":
                      print("Hit budget limit.")
                  else:
                      print(f"Stopped: {message.subtype}")
                  if message.total_cost_usd is not None:
                      print(f"Cost: ${message.total_cost_usd:.4f}")
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, the error subtype branches above have
          # already run; connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")


  asyncio.run(run_agent())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  let sessionId: string | undefined;

  try {
    for await (const message of query({
      prompt: "Find and fix the bug causing test failures in the auth module",
      options: {
        allowedTools: ["Read", "Edit", "Bash", "Glob", "Grep"], // Listing tools here auto-approves them (no prompting)
        settingSources: ["project"], // Load CLAUDE.md, skills, hooks from current directory
        maxTurns: 30, // Prevent runaway sessions
        effort: "high" // Thorough reasoning for complex debugging
      }
    })) {
      // Save the session ID to resume later if needed
      if (message.type === "system" && message.subtype === "init") {
        sessionId = message.session_id;
      }

      // Handle the final result
      if (message.type === "result") {
        if (message.subtype === "success") {
          console.log(`Done: ${message.result}`);
        } else if (message.subtype === "error_max_turns") {
          // Agent ran out of turns. Resume with a higher limit.
          console.log(`Hit turn limit. Resume session ${sessionId} to continue.`);
        } else if (message.subtype === "error_max_budget_usd") {
          console.log("Hit budget limit.");
        } else {
          console.log(`Stopped: ${message.subtype}`);
        }
        console.log(`Cost: $${message.total_cost_usd.toFixed(4)}`);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, the error subtype branches above have
    // already run; connection or process failures yield no result message.
    console.log(`Session ended with an error: ${error}`);
  }
  ```
</CodeGroup>

Wenn der Agent erfolgreich beendet wird, gibt das Beispiel eine `Done:`-Zeile mit der Zusammenfassung des Agents zur Behebung aus, gefolgt von einer Zeile wie `Cost: $0.0312`.

<h2 id="next-steps">
  Nächste Schritte
</h2>

Jetzt, da Sie die Schleife verstehen, hier ist, wohin Sie gehen sollten, je nachdem, was Sie erstellen:

* **Haben Sie noch keinen Agent ausgeführt?** Beginnen Sie mit dem [Schnellstart](/docs/de/agent-sdk/quickstart), um das SDK installiert zu bekommen und ein vollständiges Beispiel von Anfang bis Ende laufen zu sehen.
* **Bereit, in Ihr Projekt zu integrieren?** [Laden Sie CLAUDE.md, Fähigkeiten und Dateisystem-Hooks](/docs/de/agent-sdk/claude-code-features), damit der Agent automatisch Ihren Projektkonventionen folgt.
* **Erstellen Sie eine interaktive Benutzeroberfläche?** Aktivieren Sie [Streaming](/docs/de/agent-sdk/streaming-output), um Live-Text und Werkzeugaufrufe anzuzeigen, während die Schleife läuft.
* **Benötigen Sie strengere Kontrolle über das, was der Agent tun kann?** Sperren Sie den Werkzeugzugriff mit [Berechtigungen](/docs/de/agent-sdk/permissions) und verwenden Sie [Hooks](/docs/de/agent-sdk/hooks), um Werkzeugaufrufe vor ihrer Ausführung zu prüfen, zu blockieren oder zu transformieren.
* **Führen Sie lange oder teure Aufgaben aus?** Lagern Sie isolierte Arbeiten auf [Subagenten](/docs/de/agent-sdk/subagents) aus, um Ihren Hauptkontext schlank zu halten.
* **Stellen Sie als Service bereit?** Siehe [Hosting the Agent SDK](/docs/de/agent-sdk/hosting) für Container- und Serverless-Anleitung und [Session storage](/docs/de/agent-sdk/session-storage), um Sitzungen in Ihrem eigenen Backend zu speichern.

Für das breitere konzeptionelle Bild der agentengesteuerten Schleife (nicht SDK-spezifisch) siehe [So funktioniert Claude Code](/docs/de/how-claude-code-works). Für einen praktischen Leitfaden zum Entwerfen von Schleifen in Claude Code, von rundenbasierten bis zu zielgesteuerten und proaktiven Schleifen, siehe [Loop engineering: getting started with loops](https://claude.com/blog/getting-started-with-loops) im Blog.
