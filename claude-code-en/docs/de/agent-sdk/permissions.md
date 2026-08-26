> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Berechtigungen konfigurieren

> Kontrollieren Sie, wie Ihr Agent Tools mit Berechtigungsmodi, Hooks und deklarativen Allow/Deny-Regeln verwendet.

Das Claude Agent SDK bietet Berechtigungskontrollen zur Verwaltung der Tool-Nutzung durch Claude. Verwenden Sie Berechtigungsmodi und Regeln, um zu definieren, was automatisch zulässig ist, und den [`canUseTool`-Callback](/docs/de/agent-sdk/user-input), um alles andere zur Laufzeit zu handhaben.

<Note>
  Diese Seite behandelt Berechtigungsmodi und Regeln. Um interaktive Genehmigungsabläufe zu erstellen, bei denen Benutzer Tool-Anfragen zur Laufzeit genehmigen oder ablehnen, siehe [Genehmigungen und Benutzereingaben handhaben](/docs/de/agent-sdk/user-input).
</Note>

<h2 id="how-permissions-are-evaluated">
  Wie Berechtigungen ausgewertet werden
</h2>

Wenn Claude ein Tool anfordert, prüft das SDK die Berechtigungen in dieser Reihenfolge:

<Steps>
  <Step title="Hooks">
    Führen Sie [Hooks](/docs/de/agent-sdk/hooks) zuerst aus. Ein Hook kann den Aufruf direkt ablehnen oder ihn weitergeben. Ein Hook, der `allow` zurückgibt, überspringt nicht die Deny- und Ask-Regeln unten; diese werden unabhängig vom Hook-Ergebnis ausgewertet.
  </Step>

  <Step title="Deny-Regeln">
    Prüfen Sie `deny`-Regeln (aus `disallowed_tools` und [settings.json](/docs/de/settings#permission-settings)). Wenn eine Deny-Regel zutrifft, wird das Tool blockiert, auch im `bypassPermissions`-Modus. Bare-Name-Deny-Regeln wie `Bash` entfernen das Tool aus Claudes Kontext, bevor diese Auswertung beginnt, daher werden nur scoped-Regeln wie `Bash(rm *)` in diesem Schritt geprüft.
  </Step>

  <Step title="Ask-Regeln">
    Prüfen Sie `ask`-Regeln aus [settings.json](/docs/de/settings#permission-settings). Wenn eine Ask-Regel zutrifft, fällt der Aufruf zu Ihrem [`canUseTool`-Callback](/docs/de/agent-sdk/user-input) zur Bestätigung durch, auch im `bypassPermissions`-Modus.

    Tools, die Benutzerinteraktion erfordern, verhalten sich auf die gleiche Weise: `AskUserQuestion` und MCP-Tools, deren Server [`_meta["anthropic/requiresUserInteraction"]`](/docs/de/mcp#require-approval-for-a-specific-tool) setzt, fallen immer zum Callback durch, auch wenn eine Allow-Regel zutrifft. Im `dontAsk`-Modus werden beide Fälle stattdessen abgelehnt, da dieser Modus niemals eine Aufforderung anzeigt. Die MCP-Anmerkung erfordert Claude Code v2.1.199 oder später.

    [claude.ai-Connector](/docs/de/mcp#organization-controls-on-connector-tools)-Tools, die Ihre Organisation auf `ask` gesetzt hat, verlassen den Fluss auch in diesem Schritt. Jeder Aufruf fällt zum Callback durch, auch im `bypassPermissions`-Modus und auch wenn eine Allow-Regel zutrifft. Der Callback erhält den Grund `Your organization requires approval for this tool`. Im `dontAsk`-Modus wird der Aufruf stattdessen abgelehnt, da dieser Modus niemals eine Aufforderung anzeigt.
  </Step>

  <Step title="Berechtigungsmodus">
    Wenden Sie den aktiven [Berechtigungsmodus](#permission-modes) an. `bypassPermissions` genehmigt alles, das diesen Schritt erreicht. `acceptEdits` genehmigt Dateivorgänge. `plan` leitet Datei-Edit- und Shell-Write-Tools zu Ihrem `canUseTool`-Callback weiter, unabhängig von Allow-Regeln, sodass Schreibvorgänge während der Planung nicht automatisch genehmigt werden können. Andere Modi fallen durch.
  </Step>

  <Step title="Allow-Regeln">
    Prüfen Sie `allow`-Regeln (aus `allowed_tools` und settings.json). Wenn eine Regel zutrifft, wird das Tool genehmigt.
  </Step>

  <Step title="canUseTool-Callback">
    Wenn nicht durch eines der oben genannten Verfahren gelöst, rufen Sie Ihren [`canUseTool`-Callback](/docs/de/agent-sdk/user-input) für eine Entscheidung auf. Im `dontAsk`-Modus wird dieser Schritt übersprungen und das Tool wird abgelehnt.
  </Step>
</Steps>

<img src="https://mintcdn.com/claude-code/jYgs7qigNjO1Badj/images/agent-sdk/permissions-flow.svg?fit=max&auto=format&n=jYgs7qigNjO1Badj&q=85&s=c771ad9085b1277d3708027a49c744bc" alt="Diagramm des sechsstufigen Berechtigungsauswertungsflusses, das den obigen Schritten entspricht: Eine Tool-Anfrage durchläuft Hooks, Deny-Regeln, Ask-Regeln, Berechtigungsmodus, Allow-Regeln und canUseTool. Hooks, Deny-Regeln und canUseTool können zu Blockiert weiterleiten; Berechtigungsmodus-Bypass, Allow-Regeln und canUseTool können zu Ausführen weiterleiten; Ask-Regeln leiten zu canUseTool weiter." width="1180" height="260" data-path="images/agent-sdk/permissions-flow.svg" />

Ab v2.1.198 gibt das TypeScript SDK eine Node.js-Prozesswarnung aus, wenn Sie einen `canUseTool`-Callback übergeben, den diese Auswertungsreihenfolge niemals erreichen kann. Der Warnungscode ist `CLAUDE_SDK_CAN_USE_TOOL_SHADOWED`. Zwei Konfigurationen lösen ihn aus:

* `permissionMode: 'bypassPermissions'`, das jeden Aufruf, der den Berechtigungsmodus-Schritt erreicht, automatisch genehmigt
* Jeder bare `allowedTools`-Eintrag wie `"Read"`, der dieses gesamte Tool automatisch genehmigt, bevor der Callback konsultiert wird

Einträge mit einem Spezifizierer wie `Bash(ls *)` und der `acceptEdits`-Modus lösen ihn nicht aus, und Allow-Regeln aus Einstellungsdateien sind für die Prüfung nicht sichtbar.

Hören Sie mit `process.on('warning', ...)` zu und gleichen Sie den Code ab, um ihn zu protokollieren oder zu unterdrücken. Um jeden Tool-Aufruf unabhängig von Modus und Regeln zu steuern, verwenden Sie stattdessen einen [`PreToolUse`-Hook](/docs/de/agent-sdk/hooks).

Diese Seite konzentriert sich auf **Allow- und Deny-Regeln** sowie **Berechtigungsmodi**. Für die anderen Schritte:

* **Hooks:** Führen Sie benutzerdefinierten Code aus, um Tool-Anfragen zu genehmigen, abzulehnen oder zu ändern. Siehe [Ausführung mit Hooks steuern](/docs/de/agent-sdk/hooks).
* **canUseTool-Callback:** Fordern Sie Benutzer zur Laufzeit zur Genehmigung auf, wenn kein früherer Schritt den Aufruf löst. Siehe [Genehmigungen und Benutzereingaben handhaben](/docs/de/agent-sdk/user-input).

<h2 id="allow-and-deny-rules">
  Allow- und Deny-Regeln
</h2>

`allowed_tools` und `disallowed_tools` (TypeScript: `allowedTools` / `disallowedTools`) fügen Einträge zu den Allow- und Deny-Regellisten im obigen Auswertungsfluss hinzu. Allow-Regeln beeinflussen nur die Genehmigung: Ein Tool, das nicht in `allowed_tools` aufgelistet ist, ist immer noch für Claude verfügbar und fällt durch zum Berechtigungsmodus. Deny-Regeln verhalten sich unterschiedlich, je nachdem, ob sie ein Tool benennen oder ein Muster innerhalb eines Tools eingrenzen.

| Option                            | Auswirkung                                                                                                                                                                                             |
| :-------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowed_tools=["Read", "Grep"]`  | `Read` und `Grep` werden automatisch genehmigt. Tools, die hier nicht aufgelistet sind, existieren immer noch und fallen durch zum Berechtigungsmodus und `canUseTool`.                                |
| `disallowed_tools=["Bash"]`       | Die `Bash`-Tool-Definition wird aus der Anfrage entfernt. Claude sieht das Tool nicht und kann es nicht versuchen.                                                                                     |
| `disallowed_tools=["Bash(rm *)"]` | `Bash` bleibt verfügbar. Aufrufe, die `rm *` entsprechen, werden in jedem Berechtigungsmodus abgelehnt, einschließlich `bypassPermissions`. Andere `Bash`-Aufrufe fallen durch zum Berechtigungsmodus. |
| `disallowed_tools=["*"]`          | Jede Tool-Definition wird aus der Anfrage entfernt. Tool-Name-Globs werden in Deny-Regeln unterstützt: `"*"` entspricht jedem Tool und `"mcp__*"` entspricht jedem MCP-Tool über alle Server hinweg.   |

Allow-Regeln akzeptieren Tool-Name-Globs nur nach einem literalen `mcp__<server>__`-Präfix. Das Server-Segment muss glob-frei sein, damit die Regel einen bestimmten Server benennt, den Sie konfiguriert haben: `mcp__puppeteer__*` entspricht jedem Tool vom `puppeteer`-Server, und `mcp__github__get_*` entspricht seinen `get_`-Tools. Ein unverankter Eintrag wie `allowed_tools=["*"]` oder `allowed_tools=["mcp__*"]` wird mit einer Startwarnmeldung ignoriert und genehmigt nichts automatisch.

Begrenzte Regeln für `Read` und `Edit` verwenden ein Pfadmuster. `Edit(path)`-Regeln regeln alle integrierten Tools, die Dateien schreiben, einschließlich `Write` und `NotebookEdit`; eine `Write(path)`-Regel wird nie von den Dateiberechtigungsprüfungen erfasst.

Verwenden Sie `//path` für einen absoluten Dateisystempfad: Eine Deny-Regel von `Edit(//secrets/**)` blockiert Schreibvorgänge überall unter `/secrets` auf der Festplatte. Mit einem einzelnen führenden Schrägstrich verankert `Edit(/secrets/**)` stattdessen an der Quelle der Regel. Für Regeln, die durch `allowed_tools` oder `disallowed_tools` übergeben werden, bedeutet das das Arbeitsverzeichnis der Sitzung, sodass die Regel `/secrets` auf der Festplatte nicht blockiert. Siehe [Read- und Edit-Regeln](/docs/de/permissions#read-and-edit) für die vier Ankerformen und wie Regeln aus Einstellungsdateien aufgelöst werden.

<Warning>
  **Auto-genehmigte Tools erreichen `canUseTool` nie.** Ein Tool-Aufruf, der in einem früheren Schritt genehmigt wurde, durch `acceptEdits` oder `bypassPermissions`, oder durch eine Allow-Regel, überspringt Ihren `canUseTool`-Callback, sodass Berechtigungsprüfungen, die Sie dort durchführen, für dieses Tool stillschweigend umgangen werden. `AskUserQuestion`, MCP-Tools, die mit [`_meta["anthropic/requiresUserInteraction"]`](/docs/de/mcp#require-approval-for-a-specific-tool) gekennzeichnet sind, und Connector-Tools, [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools), erreichen den Callback immer noch, auch wenn eine Allow-Regel passt.

  Die Abdeckung hängt von der Form des Eintrags ab: Ein einfacher Name wie `Read` oder `mcp__github__get_issue` genehmigt jeden Aufruf dieses Tools automatisch, während eine begrenzte Regel wie `Bash(ls *)` nur übereinstimmende Aufrufe automatisch genehmigt und andere `Bash`-Aufrufe immer noch zum Callback fallen. Für Prüfungen, die bei jedem Tool-Aufruf ausgeführt werden müssen, verwenden Sie einen [`PreToolUse`-Hook](/docs/de/agent-sdk/hooks): Hooks werden vor jedem anderen Schritt ausgeführt, und eine Hook-Ablehnung gilt auch im `bypassPermissions`-Modus.
</Warning>

Für einen gesperrten Agent kombinieren Sie `allowedTools` mit `permissionMode: "dontAsk"`. Aufgelistete Tools werden genehmigt, abgesehen von den immer-Prompt-Tools in der obigen Warnung; alles andere wird direkt abgelehnt, anstatt zu fragen:

```typescript theme={null}
const options = {
  allowedTools: ["Read", "Glob", "Grep"],
  permissionMode: "dontAsk"
};
```

<Warning>
  **`allowed_tools` beschränkt `bypassPermissions` nicht.** `allowed_tools` genehmigt nur die Tools, die Sie aufgelistet haben. Nicht aufgelistete Tools werden von keiner Allow-Regel erfasst und fallen durch zum Berechtigungsmodus, wo `bypassPermissions` sie genehmigt. Das Setzen von `allowed_tools=["Read"]` zusammen mit `permission_mode="bypassPermissions"` genehmigt immer noch jedes Tool, einschließlich `Bash`, `Write` und `Edit`. Wenn Sie `bypassPermissions` benötigen, aber bestimmte Tools blockieren möchten, verwenden Sie `disallowed_tools`.
</Warning>

Sie können Allow-, Deny- und Ask-Regeln auch deklarativ in `.claude/settings.json` konfigurieren. Diese Regeln werden gelesen, wenn die `project`-Einstellungsquelle aktiviert ist, was sie für Standard-`query()`-Optionen ist. Wenn Sie `setting_sources` (TypeScript: `settingSources`) explizit setzen, fügen Sie `"project"` ein, damit sie angewendet werden. Siehe [Berechtigungseinstellungen](/docs/de/settings#permission-settings) für die Regelsyntax.

<h2 id="permission-modes">
  Berechtigungsmodi
</h2>

Berechtigungsmodi bieten globale Kontrolle über die Tool-Nutzung durch Claude. Sie können den Berechtigungsmodus beim Aufrufen von `query()` setzen oder ihn dynamisch während Streaming-Sitzungen ändern.

<h3 id="available-modes">
  Verfügbare Modi
</h3>

Das SDK unterstützt diese Berechtigungsmodi:

| Modus               | Beschreibung                               | Tool-Verhalten                                                                                                                                                                                                                                                                                                                                          |
| :------------------ | :----------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `default`           | Standardberechtigungsverhalten             | Keine automatischen Genehmigungen; nicht übereinstimmende Tools lösen Ihren `canUseTool`-Callback aus                                                                                                                                                                                                                                                   |
| `dontAsk`           | Ablehnung statt Nachfrage                  | Alles, das nicht von `allowed_tools` oder Regeln vorab genehmigt ist, wird abgelehnt; Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und Tools, die Benutzerinteraktion erfordern, werden abgelehnt, auch wenn Sie sie vorab genehmigt haben. `canUseTool` wird nie aufgerufen         |
| `acceptEdits`       | Dateibearbeitungen automatisch akzeptieren | Dateibearbeitungen und [Dateisystemvorgänge](#accept-edits-mode-acceptedits) (`mkdir`, `rm`, `mv` usw.) werden automatisch genehmigt                                                                                                                                                                                                                    |
| `bypassPermissions` | Berechtigungsprüfungen umgehen             | Tools werden ohne Berechtigungsaufforderungen ausgeführt, mit Ausnahme von Tools, die einer expliziten [`ask`-Regel](#how-permissions-are-evaluated) entsprechen, Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und Tools, die Benutzerinteraktion erfordern (mit Vorsicht verwenden) |
| `plan`              | Planungsmodus                              | Claude erkundet und plant, ohne Ihre Quelldateien zu bearbeiten; Dateibearbeitungen werden nie automatisch genehmigt und werden durch Ihren `canUseTool`-Callback angefordert                                                                                                                                                                           |
| `auto`              | Modellklassifizierte Genehmigungen         | Ein Modellklassifizierer genehmigt oder lehnt jeden Tool-Aufruf ab. Siehe [Auto-Modus](/docs/de/permission-modes#eliminate-prompts-with-auto-mode) für Verfügbarkeit                                                                                                                                                                                         |

<Warning>
  **Subagent-Vererbung:** Wenn der übergeordnete Agent `bypassPermissions`, `acceptEdits` oder `auto` verwendet, erben alle Subagents diesen Modus und er kann nicht pro Subagent überschrieben werden. Subagents können unterschiedliche Systemaufforderungen und weniger eingeschränktes Verhalten als Ihr Hauptagent haben, daher erbt `bypassPermissions` ihnen vollständigen, autonomen Systemzugriff. Explizite [`ask`-Regeln](#how-permissions-are-evaluated), Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und Tools, die Benutzerinteraktion erfordern, erzwingen weiterhin eine Aufforderung.
</Warning>

<h3 id="set-permission-mode">
  Berechtigungsmodus setzen
</h3>

Sie können den Berechtigungsmodus einmal beim Starten einer Abfrage setzen oder ihn dynamisch ändern, während die Sitzung aktiv ist.

<Tabs>
  <Tab title="Zur Abfragezeit">
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
    Rufen Sie `set_permission_mode()` (Python) oder `setPermissionMode()` (TypeScript) auf, um den Modus während der Sitzung zu ändern. Der neue Modus wird sofort für alle nachfolgenden Tool-Anfragen wirksam. Dies ermöglicht es Ihnen, restriktiv zu beginnen und Berechtigungen zu lockern, wenn Vertrauen aufgebaut wird, z. B. zum Wechsel zu `acceptEdits` nach Überprüfung von Claudes anfänglichem Ansatz.

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
  Accept Edits-Modus (`acceptEdits`)
</h4>

Genehmigt automatisch Dateivorgänge, damit Claude Code ohne Aufforderung bearbeiten kann. Andere Tools (wie Bash-Befehle, die keine Dateisystemvorgänge sind) erfordern weiterhin normale Berechtigungen.

**Automatisch genehmigte Vorgänge:**

* Dateibearbeitungen (Edit-, Write-Tools)
* Dateisystembefehle: `mkdir`, `touch`, `rm`, `rmdir`, `mv`, `cp`, `sed`

Beide gelten nur für Pfade innerhalb des Arbeitsverzeichnisses oder `additionalDirectories`. Pfade außerhalb dieses Bereichs und Schreibvorgänge auf geschützte Pfade werden weiterhin angefordert.

**Verwenden Sie, wenn:** Sie Claudes Bearbeitungen vertrauen und schnellere Iteration wünschen, z. B. während der Prototypenerstellung oder beim Arbeiten in einem isolierten Verzeichnis.

<h4 id="don’t-ask-mode-dontask">
  Don't Ask-Modus (`dontAsk`)
</h4>

Konvertiert jede Berechtigungsaufforderung in eine Ablehnung. Tools, die von `allowed_tools`, `settings.json`-Allow-Regeln oder einem Hook vorab genehmigt sind, werden normal ausgeführt. Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und Tools, die Benutzerinteraktion erfordern, werden abgelehnt, auch wenn eine Allow-Regel stimmt. Alles andere wird abgelehnt, ohne `canUseTool` aufzurufen.

**Verwenden Sie, wenn:** Sie eine feste, explizite Tool-Oberfläche für einen Headless-Agent wünschen und eine harte Ablehnung gegenüber stiller Abhängigkeit von fehlender `canUseTool` bevorzugen.

<h4 id="bypass-permissions-mode-bypasspermissions">
  Bypass Permissions-Modus (`bypassPermissions`)
</h4>

Genehmigt automatisch alle Tool-Nutzungen ohne Aufforderungen. Hooks werden weiterhin ausgeführt und können Vorgänge bei Bedarf blockieren.

<Warning>
  Mit äußerster Vorsicht verwenden. Claude hat in diesem Modus vollständigen Systemzugriff. Verwenden Sie nur in kontrollierten Umgebungen, in denen Sie allen möglichen Vorgängen vertrauen.

  `allowed_tools` beschränkt diesen Modus nicht. Jedes Tool wird genehmigt, nicht nur die, die Sie aufgelistet haben. Deny-Regeln (`disallowed_tools`), explizite `ask`-Regeln und Hooks werden vor der Modusüberprüfung ausgewertet und können ein Tool immer noch blockieren. Connector-Tools [die Ihre Organisation auf `ask` gesetzt hat](/docs/de/mcp#organization-controls-on-connector-tools) und Tools, die Benutzerinteraktion erfordern, fallen weiterhin in Ihren `canUseTool`-Callback.
</Warning>

<h4 id="plan-mode-plan">
  Plan-Modus (`plan`)
</h4>

Claude erkundet die Codebasis und erstellt einen Plan, ohne Ihre Quelldateien zu bearbeiten. Schreibgeschützte Tools werden wie im Standard-Modus ausgeführt. Dateibearbeitungen werden im Plan-Modus nie automatisch genehmigt, auch wenn eine Allow-Regel stimmt. Sie werden stattdessen durch Ihren `canUseTool`-Callback angefordert. Claude kann `AskUserQuestion` verwenden, um Anforderungen zu klären, bevor der Plan abgeschlossen wird. Siehe [Genehmigungen und Benutzereingaben handhaben](/docs/de/agent-sdk/user-input#handle-clarifying-questions) für die Behandlung dieser Aufforderungen.

**Verwenden Sie, wenn:** Sie möchten, dass Claude Änderungen vorschlägt, ohne sie auszuführen, z. B. während der Code-Überprüfung oder wenn Sie Änderungen genehmigen müssen, bevor sie vorgenommen werden.

<h2 id="related-resources">
  Verwandte Ressourcen
</h2>

Für die anderen Schritte im Berechtigungsauswertungsfluss:

* [Genehmigungen und Benutzereingaben handhaben](/docs/de/agent-sdk/user-input): interaktive Genehmigungsaufforderungen und Klärungsfragen
* [Hooks-Anleitung](/docs/de/agent-sdk/hooks): Führen Sie benutzerdefinierten Code an Schlüsselpunkten im Agent-Lebenszyklus aus
* [Berechtigungsregeln](/docs/de/settings#permission-settings): deklarative Allow/Deny-Regeln in `settings.json`
