> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Berechtigungen konfigurieren

> Kontrollieren Sie, wie Ihr Agent Tools mit Berechtigungsmodi, Hooks und deklarativen Allow/Deny-Regeln verwendet.

Das Claude Agent SDK bietet Berechtigungskontrollen zur Verwaltung der Tool-Nutzung durch Claude. Verwenden Sie Berechtigungsmodi und Regeln, um automatisch zu definieren, was zulässig ist, und den [`canUseTool`-Callback](/docs/de/agent-sdk/user-input), um alles andere zur Laufzeit zu handhaben.

<h2 id="how-permissions-are-evaluated">
  Wie Berechtigungen ausgewertet werden
</h2>

Wenn Claude ein Tool anfordert, prüft das SDK Berechtigungen in dieser Reihenfolge:

<Steps>
  <Step title="Hooks">
    Führen Sie [Hooks](/docs/de/agent-sdk/hooks) zuerst aus. Ein Hook kann den Aufruf direkt ablehnen oder ihn weitergeben. Ein Hook, der `allow` zurückgibt, überspringt nicht die Deny- und Ask-Regeln unten; diese werden unabhängig vom Hook-Ergebnis ausgewertet. Ein `PreToolUse` Hook allow kann auch keine `rm` oder `rmdir` Entfernung genehmigen, die auf einen [kritischen Pfad](/docs/de/permission-modes#critical-paths) abzielt.
  </Step>

  <Step title="Deny-Regeln">
    Prüfen Sie `deny` Regeln (aus `disallowed_tools` und [settings.json](/docs/de/settings-reference#permission-settings)). Wenn eine Deny-Regel zutrifft, wird das Tool blockiert, auch im `bypassPermissions` Modus. Bare-Name Deny-Regeln wie `Bash` entfernen das Tool aus Claudes Kontext, bevor diese Auswertung beginnt, daher werden nur scoped Regeln wie `Bash(rm *)` in diesem Schritt geprüft.
  </Step>

  <Step title="Ask-Regeln">
    Prüfen Sie `ask` Regeln aus [settings.json](/docs/de/settings-reference#permission-settings). Wenn eine Ask-Regel zutrifft, fällt der Aufruf in Ihren [`canUseTool` Callback](/docs/de/agent-sdk/user-input) zur Bestätigung, auch im `bypassPermissions` Modus.

    Tools, die Benutzerinteraktion erfordern, verhalten sich auf die gleiche Weise: `AskUserQuestion` und MCP-Tools, deren Server [`_meta["anthropic/requiresUserInteraction"]`](/docs/de/mcp#require-approval-for-a-specific-tool) setzt, fallen immer in den Callback, auch wenn eine Allow-Regel zutrifft. Im `dontAsk` Modus werden beide Fälle stattdessen abgelehnt, da dieser Modus niemals auffordert. Die MCP-Anmerkung erfordert Claude Code v2.1.199 oder später.

    [claude.ai Connector](/docs/de/mcp#organization-controls-on-connector-tools) Tools, die Ihre Organisation auf `ask` gesetzt hat, verlassen den Fluss auch in diesem Schritt. Jeder Aufruf fällt in den Callback, auch im `bypassPermissions` Modus und auch wenn eine Allow-Regel zutrifft. Der Callback erhält den Grund `Your organization requires approval for this tool`. Im `dontAsk` Modus wird der Aufruf stattdessen abgelehnt, da dieser Modus niemals auffordert.
  </Step>

  <Step title="Berechtigungsmodus">
    Wenden Sie den aktiven [Berechtigungsmodus](#permission-modes) an:

    * Im `bypassPermissions` Modus genehmigt Claude Code alles, das diesen Schritt erreicht, außer `rm` und `rmdir` Entfernungen, die auf einen [kritischen Pfad](/docs/de/permission-modes#critical-paths) abzielen, die stattdessen weitergeleitet werden.
    * Im `acceptEdits` Modus genehmigt Claude Code die unter [Accept edits mode](#accept-edits-mode-acceptedits) aufgelisteten Dateivorgänge.
    * Im `plan` Modus sendet Claude Code Datei-Edit- und Shell-Write-Tools an Ihren `canUseTool` Callback unabhängig von Allow-Regeln, damit Schreibvorgänge während der Planung nicht automatisch genehmigt werden können.
    * In anderen Modi fällt die Anfrage durch.
  </Step>

  <Step title="Allow-Regeln">
    Prüfen Sie `allow` Regeln (aus `allowed_tools` und settings.json). Wenn eine Regel zutrifft, wird das Tool genehmigt. Ein Aufruf, den das Tool selbst genehmigt, wird auch in diesem Schritt aufgelöst, ohne dass eine Regel erforderlich ist: zum Beispiel ein Dateilesezugriff in Ihren Arbeitsverzeichnissen oder ein [schreibgeschützter Bash-Befehl](/docs/de/permissions#read-only-commands). `rm` und `rmdir` Entfernungen, die auf einen [kritischen Pfad](/docs/de/permission-modes#critical-paths) abzielen, werden niemals durch eine Allow-Regel genehmigt: sie erreichen Ihren Callback in den Modi, die auffordern, gehen zum [Klassifizierer](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) im `auto` Modus auf Claude Code v2.1.218 oder später, und werden im `dontAsk` Modus abgelehnt.
  </Step>

  <Step title="canUseTool Callback">
    Wenn nicht durch einen der oben genannten Punkte aufgelöst, rufen Sie Ihren [`canUseTool` Callback](/docs/de/agent-sdk/user-input) zur Entscheidung auf. Im `dontAsk` Modus wird dieser Schritt übersprungen und das Tool wird abgelehnt.

    Im TypeScript SDK, wenn Sie [`permissionPrompts: 'none'`](/docs/de/agent-sdk/typescript#options) setzen, wird Ihr Callback in diesem Schritt nicht aufgerufen. Ein [`PermissionRequest` Hook](/docs/de/hooks#permissionrequest) erhält immer noch eine Chance zu entscheiden, und wenn nicht, lehnt Claude Code den Aufruf ab. Die Option erfordert Claude Code v2.1.259 oder später.
  </Step>
</Steps>

<img src="https://mintcdn.com/claude-code/jYgs7qigNjO1Badj/images/agent-sdk/permissions-flow.svg?fit=max&auto=format&n=jYgs7qigNjO1Badj&q=85&s=c771ad9085b1277d3708027a49c744bc" className="dark:hidden" alt="Diagramm des sechsstufigen Berechtigungsauswertungsflusses, das den obigen Schritten entspricht: Eine Tool-Anfrage durchläuft Hooks, Deny-Regeln, Ask-Regeln, Berechtigungsmodus, Allow-Regeln und canUseTool. Hooks, Deny-Regeln und canUseTool können zu Blocked weiterleiten; Berechtigungsmodus Bypass, Allow-Regeln und canUseTool können zu Execute weiterleiten; Ask-Regeln leiten zu canUseTool weiter." width="1180" height="260" data-path="images/agent-sdk/permissions-flow.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/permissions-flow-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=e53a91e9059cbf51852b7cedb4dd4251" className="hidden dark:block" alt="Diagramm des sechsstufigen Berechtigungsauswertungsflusses, das den obigen Schritten entspricht: Eine Tool-Anfrage durchläuft Hooks, Deny-Regeln, Ask-Regeln, Berechtigungsmodus, Allow-Regeln und canUseTool. Hooks, Deny-Regeln und canUseTool können zu Blocked weiterleiten; Berechtigungsmodus Bypass, Allow-Regeln und canUseTool können zu Execute weiterleiten; Ask-Regeln leiten zu canUseTool weiter." width="1180" height="260" data-path="images/agent-sdk/permissions-flow-dark.svg" />

Wenn Sie einen `canUseTool` Callback in einer Konfiguration übergeben, in der das TypeScript SDK erwartet, dass die Auswertungsreihenfolge Aufrufe automatisch genehmigt, bevor der Callback konsultiert wird, gibt das SDK eine Node.js-Prozesswarnung aus, wenn die Abfrage konstruiert wird. Der Code der Warnung ist `CLAUDE_SDK_CAN_USE_TOOL_SHADOWED`. Zwei Konfigurationen lösen sie aus:

* `permissionMode: 'bypassPermissions'`, das jeden Aufruf automatisch genehmigt, der den Berechtigungsmodus-Schritt erreicht, außer den [Aktionen, die kein Modus automatisch genehmigt](/docs/de/permission-modes#actions-no-mode-auto-approves)
* Jeder bare `allowedTools` Eintrag wie `"Read"`, der dieses ganze Tool automatisch genehmigt, bevor der Callback konsultiert wird, außer den [Aktionen, die kein Modus automatisch genehmigt](/docs/de/permission-modes#actions-no-mode-auto-approves)

Einträge mit einem Spezifizierer wie `Bash(ls *)` und der `acceptEdits` Modus lösen es nicht aus, und Allow-Regeln aus Einstellungsdateien sind für die Prüfung nicht sichtbar.

Hören Sie mit `process.on('warning', ...)` zu und gleichen Sie den Code ab, um ihn zu protokollieren oder zu unterdrücken. Um jeden Tool-Aufruf unabhängig von Modus und Regeln zu steuern, verwenden Sie stattdessen einen [`PreToolUse` Hook](/docs/de/agent-sdk/hooks).

Diese Seite konzentriert sich auf **Allow- und Deny-Regeln** und **Berechtigungsmodi**. Für die anderen Schritte:

* **Hooks:** Führen Sie benutzerdefinierten Code aus, um Tool-Anfragen zu erlauben, zu verweigern oder zu ändern. Siehe [Ausführung mit Hooks steuern](/docs/de/agent-sdk/hooks).
* **canUseTool Callback:** Fordern Sie Benutzer zur Laufzeit zur Genehmigung auf, wenn kein früherer Schritt den Aufruf auflöst. Siehe [Genehmigungen und Benutzereingaben verarbeiten](/docs/de/agent-sdk/user-input).

<h2 id="allow-and-deny-rules">
  Allow- und Deny-Regeln
</h2>

`allowed_tools` und `disallowed_tools` (TypeScript: `allowedTools` / `disallowedTools`) fügen Einträge zu den Allow- und Deny-Regellisten im obigen Evaluierungsfluss hinzu. Wenn Sie eines der [Task-Tracking-Tools](/docs/de/agent-sdk/todo-tracking#model-availability) in `allowed_tools` benennen, aktiviert Claude Code auch die Sitzung dafür. Jedes andere Tool, das nicht in `allowed_tools` aufgelistet ist, ist weiterhin für Claude verfügbar, und ein Aufruf, der Genehmigung benötigt, fällt durch zum Genehmigungsmodus. Deny-Regeln verhalten sich unterschiedlich, je nachdem, ob sie ein Tool benennen oder ein Muster innerhalb eines Tools eingrenzen.

| Option                            | Auswirkung                                                                                                                                                                                                                                                                          |
| :-------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowed_tools=["Read", "Grep"]`  | `Read` und `Grep` werden automatisch genehmigt. Andere Tools, die hier nicht aufgelistet sind, existieren weiterhin, und Aufrufe, die Genehmigung benötigen, fallen durch zum Genehmigungsmodus und `canUseTool`.                                                                   |
| `disallowed_tools=["Bash"]`       | Die `Bash`-Tool-Definition wird aus der Anfrage entfernt. Claude sieht das Tool nicht und kann es nicht versuchen.                                                                                                                                                                  |
| `disallowed_tools=["Bash(rm *)"]` | `Bash` bleibt verfügbar. Aufrufe, die `rm *` [wie geschrieben](/docs/de/permissions#bash-rule-limits) entsprechen, werden in jedem Genehmigungsmodus abgelehnt, einschließlich `bypassPermissions`. Andere `Bash`-Aufrufe, einschließlich `/bin/rm`, fallen durch zum Genehmigungsmodus. |
| `disallowed_tools=["*"]`          | Jede Tool-Definition wird aus der Anfrage entfernt. Tool-Name-Globs werden in Deny-Regeln unterstützt: `"*"` entspricht jedem Tool und `"mcp__*"` entspricht jedem MCP-Tool über alle Server hinweg.                                                                                |

Allow-Regeln akzeptieren Tool-Name-Globs nur nach einem literalen `mcp__<server>__`-Präfix. Das Server-Segment muss glob-frei sein, damit die Regel einen bestimmten Server benennt, den Sie konfiguriert haben: `mcp__puppeteer__*` entspricht jedem Tool vom `puppeteer`-Server, und `mcp__github__get_*` entspricht seinen `get_`-Tools. Ein unverankter Eintrag wie `allowed_tools=["*"]` oder `allowed_tools=["mcp__*"]` wird mit einer Startwarnmeldung ignoriert und genehmigt nichts automatisch.

Begrenzte Regeln für `Read` und `Edit` verwenden ein Pfadmuster. `Edit(path)`-Regeln regeln alle integrierten Tools, die Dateien schreiben, einschließlich `Write` und `NotebookEdit`; eine `Write(path)`-Regel wird niemals durch die Dateiberechtigungsprüfungen abgeglichen.

Verwenden Sie `//path` für einen absoluten Dateisystempfad: eine Deny-Regel von `Edit(//secrets/**)` blockiert Schreibvorgänge überall unter `/secrets` auf der Festplatte. Mit einem einzelnen führenden Schrägstrich verankert `Edit(/secrets/**)` stattdessen an der Quelle der Regel. Für Regeln, die durch `allowed_tools` oder `disallowed_tools` übergeben werden, bedeutet das das Arbeitsverzeichnis der Sitzung, daher blockiert die Regel nicht `/secrets` auf der Festplatte. Siehe [Read- und Edit-Regeln](/docs/de/permissions#read-and-edit) für die vier Ankerformen und wie Regeln aus Einstellungsdateien aufgelöst werden.

<Warning>
  **Automatisch genehmigte Tools erreichen `canUseTool` niemals.** Ein Tool-Aufruf, der in einem früheren Schritt genehmigt wird, durch `acceptEdits` oder `bypassPermissions` oder durch eine Allow-Regel, überspringt Ihren `canUseTool`-Callback, daher werden Berechtigungsprüfungen, die Sie dort durchführen, für dieses Tool stillschweigend umgangen. `AskUserQuestion`, MCP-Tools, die mit [`_meta["anthropic/requiresUserInteraction"]`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools), und `rm`- und `rmdir`-Entfernungen, die auf einen [kritischen Pfad](/docs/de/permission-modes#critical-paths) abzielen, erreichen immer noch den Callback, auch wenn eine Allow-Regel passt. Im `auto`-Modus gehen kritische-Pfad-Entfernungen zum [Klassifizierer](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) statt zum Callback, während die anderen hier aufgelisteten Aufrufe ihn immer noch erreichen; das Klassifizierer-Routing erfordert Claude Code v2.1.218 oder später. Im `dontAsk`-Modus werden diese Aufrufe stattdessen abgelehnt, ohne den Callback aufzurufen.

  Die Abdeckung hängt von der Form des Eintrags ab: ein bloßer Name wie `Read` oder `mcp__github__get_issue` genehmigt automatisch jeden Aufruf dieses Tools außer den oben genannten Ausnahmen, während eine begrenzte Regel wie `Bash(npm test *)` nur übereinstimmende Aufrufe automatisch genehmigt, und andere `Bash`-Aufrufe, die Genehmigung benötigen, fallen immer noch durch zum Callback. Für Prüfungen, die bei jedem Tool-Aufruf ausgeführt werden müssen, verwenden Sie einen [`PreToolUse`-Hook](/docs/de/agent-sdk/hooks): Hooks werden vor jedem anderen Schritt ausgeführt, und eine Hook-Ablehnung gilt auch im `bypassPermissions`-Modus.
</Warning>

Für einen abgesperrten Agent kombinieren Sie `allowedTools` mit `permissionMode: "dontAsk"`:

```typescript theme={null}
const options = {
  allowedTools: ["Read", "Glob", "Grep"],
  permissionMode: "dontAsk"
};
```

Aufgelistete Tools werden genehmigt, außer den [Aktionen, die kein Modus automatisch genehmigt](/docs/de/permission-modes#actions-no-mode-auto-approves), und jeder andere Aufruf, der sonst eine Aufforderung auslösen würde, wird stattdessen abgelehnt. Aufrufe, die im `default`-Modus keine Genehmigung benötigen, werden ausgeführt, unabhängig davon, ob Sie sie aufgelistet haben oder nicht, wie z. B. [schreibgeschützte Bash-Befehle](/docs/de/permissions#read-only-commands), Tools wie `Agent`, die vor dem Ausführen nicht fragen, und Dateileser in Ihren Arbeitsverzeichnissen. Um ein Tool vollständig außerhalb von Claudes Reichweite zu platzieren, fügen Sie seinen bloßen Namen zu `disallowedTools` hinzu.

<Warning>
  **`allowed_tools` schränkt `bypassPermissions` nicht ein.** `allowed_tools` genehmigt die Tools, die Sie aufgelistet haben, im Voraus. Andere nicht aufgelistete Tools werden nicht durch eine Allow-Regel abgeglichen und fallen durch zum Genehmigungsmodus, wo `bypassPermissions` sie genehmigt. Das Setzen von `allowed_tools=["Read"]` zusammen mit `permission_mode="bypassPermissions"` genehmigt immer noch jedes Tool, einschließlich `Bash`, `Write` und `Edit`. Wenn Sie `bypassPermissions` benötigen, aber bestimmte Tools blockiert haben möchten, verwenden Sie `disallowed_tools`.
</Warning>

Sie können auch Allow-, Deny- und Ask-Regeln deklarativ in `.claude/settings.json` konfigurieren. Diese Regeln werden gelesen, wenn die `project`-Einstellungsquelle aktiviert ist, was sie für Standard-`query()`-Optionen ist. Wenn Sie `setting_sources` (TypeScript: `settingSources`) explizit setzen, schließen Sie `"project"` ein, damit sie angewendet werden. Siehe [Berechtigungseinstellungen](/docs/de/settings-reference#permission-settings) für die Regelsyntax.

<h2 id="permission-modes">
  Berechtigungsmodi
</h2>

Berechtigungsmodi bieten globale Kontrolle darüber, wie Claude Tools nutzt. Sie können den Berechtigungsmodus beim Aufrufen von `query()` festlegen oder ihn während Streaming-Sitzungen dynamisch ändern.

<h3 id="available-modes">
  Verfügbare Modi
</h3>

Das SDK unterstützt diese Berechtigungsmodi:

| Modus               | Beschreibung                               | Tool-Verhalten                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :------------------ | :----------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Standardberechtigungsverhalten             | Keine modusbasierten automatischen Genehmigungen; Aufrufe, die eine Genehmigung benötigen und keine Allow-Regel erfüllen, lösen Ihren `canUseTool`-Callback aus                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `dontAsk`           | Ablehnung statt Nachfrage                  | Jeder Aufruf, der sonst eine Nachfrage auslösen würde, wird abgelehnt. Aufrufe, die von `allowed_tools` oder Regeln genehmigt werden, und Aufrufe, die im `default`-Modus keine Genehmigung benötigen, werden ausgeführt; Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und Tools, die Benutzerinteraktion erfordern, werden abgelehnt, auch wenn Sie diese vorab genehmigt haben, ebenso wie `rm` und `rmdir` Löschungen, die auf einen [kritischen Pfad](/docs/de/permission-modes#critical-paths) abzielen. `canUseTool` wird nie aufgerufen |
| `acceptEdits`       | Dateibearbeitungen automatisch akzeptieren | Dateibearbeitungen und [Dateisystemoperationen](#accept-edits-mode-acceptedits) (`mkdir`, `rm`, `mv` usw.) werden automatisch genehmigt                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `bypassPermissions` | Berechtigungsprüfungen umgehen             | Tools werden ohne Berechtigungsaufforderungen ausgeführt, außer für die [Aktionen, die kein Modus automatisch genehmigt](/docs/de/permission-modes#actions-no-mode-auto-approves). Mit Vorsicht verwenden                                                                                                                                                                                                                                                                                                                                                                                                         |
| `plan`              | Planungsmodus                              | Claude erkundet und plant, ohne Ihre Quelldateien zu bearbeiten; Dateibearbeitungen werden nie automatisch genehmigt und werden durch Ihren `canUseTool`-Callback abgefragt                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `auto`              | Modellklassifizierte Genehmigungen         | Ein Modellklassifizierer genehmigt oder lehnt Berechtigungsaufforderungen ab. Siehe [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) für Verfügbarkeit                                                                                                                                                                                                                                                                                                                                                                                                                                    |

<Warning>
  **Subagent-Vererbung:** Ein Subagent wird im Berechtigungsmodus der übergeordneten Sitzung ausgeführt, es sei denn, Sie legen `permissionMode` auf seiner [`AgentDefinition`](/docs/de/agent-sdk/typescript#agentdefinition) fest und die übergeordnete Sitzung befindet sich im `default`-, `dontAsk`- oder `plan`-Modus. Auch dann wendet Claude Code nie einen `"bypassPermissions"`-Wert an. Ein Subagent wird nur dann im `bypassPermissions`-Modus ausgeführt, wenn die übergeordnete Sitzung selbst dies tut. Die `bypassPermissions`-Ausnahme erfordert Claude Code v2.1.267 oder später.

  Subagents können unterschiedliche Systemaufforderungen und weniger eingeschränktes Verhalten als Ihr Hauptagent haben, daher gewährt die Vererbung von `bypassPermissions` ihnen vollständigen, autonomen Systemzugriff. Die [Aktionen, die kein Modus automatisch genehmigt](/docs/de/permission-modes#actions-no-mode-auto-approves), gelten weiterhin.
</Warning>

<h3 id="set-permission-mode">
  Berechtigungsmodus festlegen
</h3>

Sie können den Berechtigungsmodus einmalig beim Starten einer Abfrage festlegen oder ihn dynamisch ändern, während die Sitzung aktiv ist.

<Tabs>
  <Tab title="Zum Abfragezeitpunkt">
    Übergeben Sie `permission_mode` (Python) oder `permissionMode` (TypeScript) beim Erstellen einer Abfrage. Dieser Modus gilt für die gesamte Sitzung, es sei denn, er wird dynamisch geändert.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Help me refactor this code",
              options=ClaudeAgentOptions(
                  permission_mode="default",  # Set the mode here
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      async function main() {
        for await (const message of query({
          prompt: "Help me refactor this code",
          options: {
            permissionMode: "default" // Set the mode here
          }
        })) {
          if ("result" in message) {
            console.log(message.result);
          }
        }
      }

      main();
      ```
    </CodeGroup>
  </Tab>

  <Tab title="Während des Streaming">
    Rufen Sie `set_permission_mode()` (Python) oder `setPermissionMode()` (TypeScript) auf, um den Modus während der Sitzung zu ändern. Der neue Modus wird sofort für alle nachfolgenden Tool-Anfragen wirksam. Dies ermöglicht es Ihnen, restriktiv zu beginnen und die Berechtigungen zu lockern, wenn Vertrauen aufgebaut wird, z. B. durch Wechsel zu `acceptEdits` nach Überprüfung von Claudes anfänglichem Ansatz.

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions


      async def main():
          async with ClaudeSDKClient(
              options=ClaudeAgentOptions(
                  permission_mode="default",  # Start in default mode
              )
          ) as client:
              await client.query("Help me refactor this code")

              # Change mode dynamically mid-session
              await client.set_permission_mode("acceptEdits")

              # Process messages with the new permission mode
              async for message in client.receive_response():
                  if hasattr(message, "result"):
                      print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      async function main() {
        const q = query({
          prompt: "Help me refactor this code",
          options: {
            permissionMode: "default" // Start in default mode
          }
        });

        // Change mode dynamically mid-session
        await q.setPermissionMode("acceptEdits");

        // Process messages with the new permission mode
        for await (const message of q) {
          if ("result" in message) {
            console.log(message.result);
          }
        }
      }

      main();
      ```
    </CodeGroup>
  </Tab>
</Tabs>

<h3 id="mode-details">
  Modusdetails
</h3>

<h4 id="accept-edits-mode-acceptedits">
  Bearbeitungsmodus akzeptieren (`acceptEdits`)
</h4>

Genehmigt Dateivorgänge automatisch, damit Claude Code bearbeiten kann, ohne zu fragen. Andere Tools (wie Bash-Befehle, die keine Dateisystemvorgänge sind) erfordern weiterhin normale Berechtigungen.

**Automatisch genehmigte Vorgänge:**

* Dateibearbeitungen (Edit-, Write-Tools)
* Dateisystembefehle: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, `sed`

Beide gelten nur für Pfade innerhalb des Arbeitsverzeichnisses oder `additionalDirectories`. Im `acceptEdits`-Modus genehmigt Claude Code die Anfrage nicht automatisch, wenn Claude:

* An einem Pfad außerhalb dieses Bereichs arbeitet
* In einen geschützten Pfad schreibt
* Einen [kritischen Pfad](/docs/de/permission-modes#critical-paths) mit `rm` oder `rmdir` löscht

**Verwenden Sie, wenn:** Sie Claudes Bearbeitungen vertrauen und schnellere Iteration wünschen, z. B. während der Prototypenerstellung oder beim Arbeiten in einem isolierten Verzeichnis.

<h4 id="don’t-ask-mode-dontask">
  Nicht-Fragen-Modus (`dontAsk`)
</h4>

Konvertiert jede Berechtigungsaufforderung in eine Ablehnung, ohne `canUseTool` aufzurufen. Tools, die von `allowed_tools`, `settings.json` Allow-Regeln oder einem Hook vorab genehmigt wurden, werden normal ausgeführt, ebenso wie Aufrufe, die im `default`-Modus keine Genehmigung benötigen, wie Dateilesevorgänge in Ihren Arbeitsverzeichnissen und Aufrufe an `Agent`. Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools), Tools, die Benutzerinteraktion erfordern, und `rm` und `rmdir` Löschungen, die auf einen [kritischen Pfad](/docs/de/permission-modes#critical-paths) abzielen, werden abgelehnt, auch wenn eine Allow-Regel übereinstimmt. Ein `PreToolUse`-Hook-Allow hebt auch eine kritische Pfad-Löschung nicht auf.

**Verwenden Sie, wenn:** Sie eine feste, explizite Tool-Oberfläche für einen Headless-Agent wünschen und eine harte Ablehnung gegenüber stiller Abhängigkeit von fehlender `canUseTool` bevorzugen.

<h4 id="bypass-permissions-mode-bypasspermissions">
  Berechtigungen umgehen-Modus (`bypassPermissions`)
</h4>

Genehmigt Tool-Nutzungen automatisch ohne Aufforderung, außer in den unten aufgeführten Fällen. Hooks werden weiterhin ausgeführt und können Vorgänge bei Bedarf blockieren. Auf Linux und macOS weigert sich Claude Code, in diesem Modus als Root oder unter `sudo` außerhalb einer [erkannten Sandbox](/docs/de/permission-modes#skip-all-checks-with-bypasspermissions-mode) zu starten, und die Abfrage schlägt vor dem ersten Turn fehl.

<Warning>
  Mit äußerster Vorsicht verwenden. Claude hat in diesem Modus vollständigen Systemzugriff. Verwenden Sie nur in kontrollierten Umgebungen, in denen Sie allen möglichen Vorgängen vertrauen.

  `allowed_tools` beschränkt diesen Modus nicht. Jedes Tool wird genehmigt, nicht nur die, die Sie aufgelistet haben. Diese Kontrollen gelten weiterhin:

  * Deny-Regeln, explizite `ask`-Regeln und Hooks werden vor der Modusüberprüfung ausgewertet und können ein Tool weiterhin blockieren.
  * Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools), Tools, die Benutzerinteraktion erfordern, und `rm` und `rmdir` Löschungen, die auf einen [kritischen Pfad](/docs/de/permission-modes#critical-paths) abzielen, fallen weiterhin in Ihren `canUseTool`-Callback.
  * Die [Sitzungsübergreifenden Messaging-Schutzmaßnahmen](/docs/de/permission-modes#skip-all-checks-with-bypasspermissions-mode) gelten weiterhin.
</Warning>

<h4 id="plan-mode-plan">
  Planungsmodus (`plan`)
</h4>

Claude erkundet die Codebasis und erstellt einen Plan, ohne Ihre Quelldateien zu bearbeiten. Read-only-Tools werden wie im `default`-Berechtigungsmodus ausgeführt.

Dateibearbeitungen werden im Planungsmodus nie automatisch genehmigt, auch wenn eine Allow-Regel übereinstimmt. Sie werden stattdessen durch Ihren `canUseTool`-Callback abgefragt. In Claude Code v2.1.212 oder später erreichen Shell-Befehle, die Dateien ändern, wie `touch` und `rm`, Ihren `canUseTool`-Callback auf die gleiche Weise.

Wenn Sie `allowDangerouslySkipPermissions: true` zusammen mit `permissionMode: 'plan'` festlegen, erreichen Dateibearbeitungen und Shell-Befehle, die Dateien ändern, weiterhin Ihren `canUseTool`-Callback. Die Option ermöglicht es Ihnen, später mit `setPermissionMode()` zu `bypassPermissions` zu wechseln.

Claude kann `AskUserQuestion` verwenden, um Anforderungen zu klären, bevor der Plan abgeschlossen wird. Siehe [Genehmigungen und Benutzereingaben verarbeiten](/docs/de/agent-sdk/user-input#handle-clarifying-questions) für die Verarbeitung dieser Aufforderungen.

**Verwenden Sie, wenn:** Sie möchten, dass Claude Änderungen vorschlägt, ohne sie auszuführen, z. B. während einer Code-Überprüfung oder wenn Sie Änderungen genehmigen müssen, bevor sie vorgenommen werden.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

Für die anderen Schritte im Genehmigungsbewertungsfluss:

* [Genehmigungen und Benutzereingaben verarbeiten](/docs/de/agent-sdk/user-input): interaktive Genehmigungsaufforderungen und Klärungsfragen
* [Hooks-Anleitung](/docs/de/agent-sdk/hooks): Ausführung von benutzerdefiniertem Code an wichtigen Punkten im Agent-Lebenszyklus
* [Berechtigungsregeln](/docs/de/settings-reference#permission-settings): deklarative Allow/Deny-Regeln in `settings.json`
