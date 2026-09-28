> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mit Tool-Suche zu vielen Tools skalieren

> Skalieren Sie Ihren Agenten auf Tausende von Tools, indem Sie nur das Nötigste entdecken und bei Bedarf laden.

Die Tool-Suche ermöglicht es Ihrem Agenten, mit Hunderten oder Tausenden von Tools zu arbeiten, indem er sie dynamisch entdeckt und bei Bedarf lädt. Anstatt alle Tool-Definitionen vorab in das Kontextfenster zu laden, durchsucht der Agent Ihren Tool-Katalog und lädt nur die Tools, die er benötigt.

Dieser Ansatz löst zwei Herausforderungen, wenn Tool-Bibliotheken skalieren:

* **Kontexteffizienz:** Tool-Definitionen können große Teile des Kontextfensters verbrauchen (50 Tools können 10–20 K Token verwenden), was weniger Platz für tatsächliche Arbeit lässt.
* **Genauigkeit der Tool-Auswahl:** Die Genauigkeit der Tool-Auswahl verschlechtert sich, wenn mehr als 30–50 Tools gleichzeitig geladen sind.

<h2 id="how-tool-search-works">
  Wie die Tool-Suche funktioniert
</h2>

Die Tool-Suche ist standardmäßig aktiviert, mit Ausnahmen, die unter [Tool-Suche konfigurieren](#configure-tool-search) aufgelistet sind.

Wenn sie aktiv ist, werden Tool-Definitionen aus dem Kontextfenster zurückgehalten. Der Agent erhält eine Zusammenfassung der verfügbaren Tools und sucht nach relevanten, wenn die Aufgabe eine Fähigkeit erfordert, die nicht bereits geladen ist. Bis zu fünf der relevantesten Tools werden standardmäßig in den Kontext geladen, wo sie für nachfolgende Durchläufe verfügbar bleiben, bis das SDK die Nachrichten komprimiert, in denen der Agent sie entdeckt hat. Nach dieser Komprimierung sucht der Agent diese Tools erneut, wenn er sie benötigt.

Die Tool-Suche fügt jedes Mal, wenn Claude nach Tools sucht, einen zusätzlichen Roundtrip hinzu, aber bei großen Tool-Sets wird dies durch einen kleineren Kontext bei jedem Durchlauf ausgeglichen. Mit weniger als etwa 10 Tools, deren Definitionen bequem in das Kontextfenster passen, ist das Laden von allem vorab normalerweise schneller.

Weitere Informationen zum zugrunde liegenden API-Mechanismus finden Sie unter [Tool-Suche in der API](https://platform.claude.com/docs/de/agents-and-tools/tool-use/tool-search-tool).

<Note>
  Die Tool-Suche wird auf Microsoft Foundry-[Bereitstellungen, die auf Azure gehostet werden](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options), nicht unterstützt, da diese sie serverseitig ablehnen: Das SDK erkennt die Ablehnung und lädt stattdessen Tool-Definitionen vorab für diese Bereitstellung. [`ENABLE_TOOL_SEARCH`](#configure-tool-search) kann dies nicht überschreiben, da die Ablehnung von der Bereitstellung selbst kommt.
</Note>

<h2 id="configure-tool-search">
  Tool-Suche konfigurieren
</h2>

Die Tool-Suche ist standardmäßig aktiviert. Bei Modellen auf der Liste der nicht unterstützten Modelle des SDK lädt das SDK Tool-Definitionen stattdessen vorab, und kein `ENABLE_TOOL_SEARCH`-Wert überschreibt das. Auf Google Cloud's Agent Platform entscheidet das SDK nach Modellgeneration:

* **Claude Opus 4.5, Sonnet 4.5, Haiku 4.5 und später**: Tool-Suche ist standardmäßig aktiviert.
* **Frühere Agent Platform-Modelle**: Das SDK lädt Tool-Definitionen vorab, da ihre Serving-Stacks den erforderlichen Beta-Header ablehnen. `ENABLE_TOOL_SEARCH` kann dies nicht überschreiben.

Vor Claude Code v2.1.221 deaktivierte das SDK die Tool-Suche für alle Modelle auf Google Cloud's Agent Platform, es sei denn, Sie haben `ENABLE_TOOL_SEARCH` gesetzt.

Das SDK deaktiviert auch die Tool-Suche, wenn `ANTHROPIC_BASE_URL` auf einen Host eines Drittanbieters verweist, da die meisten Proxys `tool_reference`-Blöcke nicht weiterleiten. Sie können diesen Standard mit der Umgebungsvariablen `ENABLE_TOOL_SEARCH` überschreiben:

| Wert            | Verhalten                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| :-------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (nicht gesetzt) | Die Tool-Suche ist aktiviert. Tool-Definitionen werden aufgeschoben und bei Bedarf entdeckt. Fällt auf Google Cloud's Agent Platform-Modellen früher als die Claude 4.5-Generation, einen `ANTHROPIC_BASE_URL` eines Drittanbieters oder eine auf Azure gehostete Microsoft Foundry-Bereitstellung auf das Laden vorab zurück.                                                                                                                             |
| `true`          | Die Tool-Suche ist immer aktiviert, außer bei einer auf Azure gehosteten Microsoft Foundry-Bereitstellung, wo die serverseitige Ablehnung immer noch das Laden vorab erzwingt, und bei Google Cloud's Agent Platform-Modellen früher als die Claude 4.5-Generation, wo das SDK weiterhin Tool-Definitionen vorab lädt. Das SDK sendet den Beta-Header durch Proxys, und Anfragen schlagen auf Proxys fehl, die `tool_reference`-Blöcke nicht unterstützen. |
| `auto`          | Zählt die Token in den Tool-Definitionen, die die Tool-Suche aufschieben kann, und vergleicht die Summe mit dem Kontextfenster des Modells. Wenn die Summe 10 % des Fensters erreicht, wird die Tool-Suche aktiviert. Darunter lädt das SDK jede Tool-Definition vorab in den Kontext.                                                                                                                                                                     |
| `auto:N`        | Wie `auto` mit einem benutzerdefinierten Prozentsatz. `auto:5` wird aktiviert, wenn diese Definitionen 5 % des Kontextfensters erreichen. Niedrigere Werte werden früher aktiviert.                                                                                                                                                                                                                                                                        |
| `false`         | Die Tool-Suche ist deaktiviert. Alle Tool-Definitionen werden bei jedem Durchlauf in den Kontext geladen.                                                                                                                                                                                                                                                                                                                                                  |

Das Setzen von [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/de/env-vars) hält die Tool-Suche aus. Sie können es nicht durch das Setzen von `ENABLE_TOOL_SEARCH` selbst überschreiben. Ihre Organisation kann die Tool-Suche durch [verwaltete Einstellungen](/docs/de/managed-settings) auf Claude Code v2.1.227 oder später aktiviert halten. [Deaktivieren Sie Pre-Release-Funktionen](/docs/de/llm-gateway-protocol#disable-pre-release-capabilities) behandelt, wo die Überschreibung gilt und was die Variable entfernt.

Die Tool-Suche gilt für alle registrierten Tools, unabhängig davon, ob sie von Remote-MCP-Servern oder [benutzerdefinierten SDK-MCP-Servern](/docs/de/agent-sdk/custom-tools) stammen. Wenn Sie `auto` verwenden, zählt das SDK jede Definition, die die Tool-Suche aufschieben kann, gegen einen kombinierten Schwellenwert: jedes MCP-Tool, das nicht als [`alwaysLoad`](/docs/de/mcp#exempt-a-server-from-deferral) markiert ist, von jedem Server, plus die integrierten Tools, die bei Bedarf geladen werden. Das SDK lädt immer Core-Built-in-Tools wie Bash, Read und Edit vorab und zählt sie nicht zum Schwellenwert.

Legen Sie den Wert in der `env`-Option auf `query()` fest. In TypeScript ersetzt `env` die Subprocess-Umgebung, daher sollten Sie `...process.env` verteilen, um vererbte Variablen beizubehalten. In Python wird `env` auf die vererbte Umgebung zusammengeführt. Dieses Beispiel verbindet sich mit einem Remote-MCP-Server, der viele Tools bereitstellt, genehmigt alle vorab mit einem Platzhalter und verwendet `auto:5`, sodass die Tool-Suche aktiviert wird, wenn die Definitionen, die sie aufschieben kann, 5 % des Kontextfensters erreichen:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Find and run the appropriate database query",
      options: {
        mcpServers: {
          "enterprise-tools": {
            // Connect to a remote MCP server
            type: "http",
            url: "https://tools.example.com/mcp"
          }
        },
        allowedTools: ["mcp__enterprise-tools__*"], // Wildcard pre-approves all tools from this server
        env: {
          ...process.env, // env replaces the subprocess environment, so keep inherited variables
          ENABLE_TOOL_SEARCH: "auto:5" // Activate tool search when deferrable definitions reach 5% of context
        }
      }
    })) {
      if (message.type === "result" && message.subtype === "success") {
        console.log(message.result);
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "enterprise-tools": {
                  "type": "http",
                  "url": "https://tools.example.com/mcp",
              }
          },
          allowed_tools=[
              "mcp__enterprise-tools__*"
          ],  # Wildcard pre-approves all tools from this server
          env={
              "ENABLE_TOOL_SEARCH": "auto:5"  # Activate tool search when deferrable definitions reach 5% of context
          },
      )

      try:
          async for message in query(
              prompt="Find and run the appropriate database query",
              options=options,
          ):
              if isinstance(message, ResultMessage) and message.subtype == "success":
                  print(message.result)
      except Exception as error:
          # A single-shot query() raises after yielding an error result
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

Um dieses Beispiel auszuführen, ersetzen Sie `https://tools.example.com/mcp` durch die URL Ihres eigenen MCP-Servers. Bei Erfolg wird der Ergebnistext auf der Konsole ausgegeben.

Da dies ein einmaliger `query()`-Aufruf ist, löst das SDK nach dem Ausgeben eines Fehlerergebnisses eine Ausnahme aus, daher umhüllt das Beispiel die Schleife in einen Try-Block. Um zu sehen, warum eine Ausführung fehlgeschlagen ist, überprüfen Sie den `subtype` der Ergebnismeldung, z. B. `error_during_execution`, innerhalb der Schleife. Weitere Informationen zu Ergebnismeldungen finden Sie unter [Behandeln Sie das Ergebnis](/docs/de/agent-sdk/agent-loop#handle-the-result).

<h2 id="optimize-tool-discovery">
  Tool-Entdeckung optimieren
</h2>

Der Suchmechanismus gleicht Abfragen mit Tool-Namen und Beschreibungen ab. Namen wie `search_slack_messages` erscheinen für eine breitere Palette von Anfragen als `query_slack`. Beschreibungen mit spezifischen Schlüsselwörtern („Slack-Nachrichten nach Schlüsselwort, Kanal oder Datumsbereich durchsuchen") entsprechen mehr Abfragen als generische („Slack abfragen").

Sie können auch einen Systemaufforderungsabschnitt hinzufügen, der verfügbare Tool-Kategorien auflistet. Dies gibt dem Agenten Kontext darüber, welche Arten von Tools verfügbar sind, um danach zu suchen. Übergeben Sie den Text über die Option `systemPrompt` in TypeScript oder `system_prompt` in Python, wobei Sie die Voreinstellung `claude_code` mit `append` verwenden, die Ihren Text zur Voreinstellung hinzufügt, anstatt sie zu ersetzen:

<CodeGroup>
  ```typescript TypeScript theme={null}
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code",
      append: "You can search for tools to interact with Slack, GitHub, and Jira."
    }
  }
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      system_prompt={
          "type": "preset",
          "preset": "claude_code",
          "append": "You can search for tools to interact with Slack, GitHub, and Jira.",
      }
  )
  ```
</CodeGroup>

Den vollständigen Satz von Systemaufforderungsoptionen finden Sie unter [Systemaufforderungen ändern](/docs/de/agent-sdk/modifying-system-prompts).

<h2 id="limits">
  Limits
</h2>

* **Maximale Tools:** 10.000 Tools in Ihrem Katalog
* **Suchergebnisse:** Gibt bis zu fünf relevanteste Tools pro Suche standardmäßig zurück
* **Modellunterstützung:** Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 und spätere Modelle; siehe [Modellkompatibilität in der API-Dokumentation](https://platform.claude.com/docs/de/agents-and-tools/tool-use/tool-search-tool#model-compatibility) für die aktuelle Liste. Dasselbe gilt auf Google Clouds Agent Platform.

<h2 id="related-documentation">
  Zugehörige Dokumentation
</h2>

* [Tool-Suche in der API](https://platform.claude.com/docs/de/agents-and-tools/tool-use/tool-search-tool): Vollständige API-Dokumentation für die Tool-Suche, einschließlich benutzerdefinierter Implementierungen
* [MCP-Server verbinden](/docs/de/agent-sdk/mcp): Verbindung zu externen Tools über MCP-Server
* [Benutzerdefinierte Tools](/docs/de/agent-sdk/custom-tools): Erstellen Sie Ihre eigenen Tools mit SDK-MCP-Servern
* [TypeScript SDK-Referenz](/docs/de/agent-sdk/typescript): Vollständige API-Referenz
* [Python SDK-Referenz](/docs/de/agent-sdk/python): Vollständige API-Referenz
