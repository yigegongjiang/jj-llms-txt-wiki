> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mit Tool-Suche zu vielen Tools skalieren

> Skalieren Sie Ihren Agenten auf Tausende von Tools, indem Sie nur das Nötigste entdecken und bei Bedarf laden.

Die Tool-Suche ermöglicht es Ihrem Agenten, mit Hunderten oder Tausenden von Tools zu arbeiten, indem er sie dynamisch entdeckt und bei Bedarf lädt. Anstatt alle Tool-Definitionen vorab in das Kontextfenster zu laden, durchsucht der Agent Ihren Tool-Katalog und lädt nur die Tools, die er benötigt.

Dieser Ansatz löst zwei Herausforderungen, wenn Tool-Bibliotheken skalieren:

* **Kontexteffizienz:** Tool-Definitionen können große Teile des Kontextfensters verbrauchen (50 Tools können 10–20 K Token verwenden), was weniger Platz für tatsächliche Arbeit lässt.
* **Genauigkeit der Tool-Auswahl:** Die Genauigkeit der Tool-Auswahl verschlechtert sich, wenn mehr als 30–50 Tools gleichzeitig geladen sind.

Die Tool-Suche ist standardmäßig aktiviert.

<h2 id="how-tool-search-works">
  Wie die Tool-Suche funktioniert
</h2>

Wenn die Tool-Suche aktiv ist, werden Tool-Definitionen aus dem Kontextfenster zurückgehalten. Der Agent erhält eine Zusammenfassung der verfügbaren Tools und sucht nach relevanten, wenn die Aufgabe eine Fähigkeit erfordert, die nicht bereits geladen ist. Bis zu fünf der relevantesten Tools werden standardmäßig in den Kontext geladen, wo sie für nachfolgende Durchläufe verfügbar bleiben. Wenn das Gespräch lang genug ist, dass das SDK frühere Nachrichten komprimiert, um Platz freizugeben, können zuvor entdeckte Tools entfernt werden, und der Agent sucht bei Bedarf erneut.

Die Tool-Suche fügt beim ersten Mal, wenn Claude ein Tool entdeckt (der Suchschritt), einen zusätzlichen Roundtrip hinzu, aber bei großen Tool-Sets wird dies durch einen kleineren Kontext bei jedem Durchlauf ausgeglichen. Mit weniger als etwa 10 Tools ist das Laden von allem vorab normalerweise schneller.

Weitere Informationen zum zugrunde liegenden API-Mechanismus finden Sie unter [Tool-Suche in der API](https://platform.claude.com/docs/de/agents-and-tools/tool-use/tool-search-tool).

<Note>
  Die Tool-Suche wird auf Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 und späteren Modellen unterstützt. Weitere Informationen finden Sie unter [Modellkompatibilität in der API-Dokumentation](https://platform.claude.com/docs/de/agents-and-tools/tool-use/tool-search-tool#model-compatibility) für die aktuelle Liste. Auf Googles Agent Platform sind die mindestens unterstützten Modelle Claude Sonnet 4.5 und Claude Opus 4.5.
</Note>

<h2 id="configure-tool-search">
  Tool-Suche konfigurieren
</h2>

Die Tool-Suche ist standardmäßig aktiviert. Sie ist standardmäßig auf Google Cloud's Agent Platform deaktiviert, wo sie für Claude Sonnet 4.5 und später sowie Claude Opus 4.5 und später unterstützt wird. Sie ist auch deaktiviert, wenn `ANTHROPIC_BASE_URL` auf einen Host eines Drittanbieters verweist, da die meisten Proxys `tool_reference`-Blöcke nicht weiterleiten. Sie können jeden Standard mit der Umgebungsvariablen `ENABLE_TOOL_SEARCH` überschreiben:

| Wert            | Verhalten                                                                                                                                                                                                                                                                                          |
| :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| (nicht gesetzt) | Die Tool-Suche ist aktiviert. Tool-Definitionen werden aufgeschoben und bei Bedarf entdeckt. Fällt auf Google Cloud's Agent Platform oder einen `ANTHROPIC_BASE_URL` eines Drittanbieters auf das Laden vorab zurück.                                                                              |
| `true`          | Die Tool-Suche ist immer aktiviert. Das SDK sendet den Beta-Header auch auf Google Cloud's Agent Platform und durch Proxys. Anfragen schlagen auf Google Cloud's Agent Platform-Modellen früher als Sonnet 4.5 oder Opus 4.5 oder auf Proxys fehl, die `tool_reference`-Blöcke nicht unterstützen. |
| `auto`          | Überprüft die kombinierte Token-Anzahl aller Tool-Definitionen gegen das Kontextfenster des Modells. Wenn sie 10 % überschreiten, wird die Tool-Suche aktiviert. Wenn sie unter 10 % liegen, werden alle Tools normal in den Kontext geladen.                                                      |
| `auto:N`        | Wie `auto` mit einem benutzerdefinierten Prozentsatz. `auto:5` wird aktiviert, wenn Tool-Definitionen 5 % des Kontextfensters überschreiten. Niedrigere Werte werden früher aktiviert.                                                                                                             |
| `false`         | Die Tool-Suche ist deaktiviert. Alle Tool-Definitionen werden bei jedem Durchlauf in den Kontext geladen.                                                                                                                                                                                          |

Das Setzen von [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/de/env-vars) hält die Tool-Suche aus, und `ENABLE_TOOL_SEARCH` kann es nicht überschreiben. Die Variable entfernt den Beta-Header, den `defer_loading`-Tool-Definitionen und `tool_reference`-Inhaltsblöcke erfordern.

Die Tool-Suche gilt für alle registrierten Tools, unabhängig davon, ob sie von Remote-MCP-Servern oder [benutzerdefinierten SDK-MCP-Servern](/docs/de/agent-sdk/custom-tools) stammen. Bei Verwendung von `auto` basiert der Schwellenwert auf der kombinierten Größe aller Tool-Definitionen auf allen Servern.

Legen Sie den Wert in der `env`-Option auf `query()` fest. In TypeScript ersetzt `env` die Subprocess-Umgebung, daher sollten Sie `...process.env` verteilen, um vererbte Variablen beizubehalten. In Python wird `env` auf die vererbte Umgebung zusammengeführt. Dieses Beispiel verbindet sich mit einem Remote-MCP-Server, der viele Tools bereitstellt, genehmigt alle vorab mit einem Platzhalter und verwendet `auto:5`, sodass die Tool-Suche aktiviert wird, wenn ihre Definitionen 5 % des Kontextfensters überschreiten:

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
          ENABLE_TOOL_SEARCH: "auto:5" // Activate tool search when tools exceed 5% of context
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
              "ENABLE_TOOL_SEARCH": "auto:5"  # Activate tool search when tools exceed 5% of context
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

Das Setzen von `ENABLE_TOOL_SEARCH` auf `"false"` deaktiviert die Tool-Suche und lädt alle Tool-Definitionen bei jedem Durchlauf in den Kontext. Dies entfernt den Suchrundruf, was schneller sein kann, wenn der Tool-Satz klein ist (weniger als etwa 10 Tools) und die Definitionen bequem in das Kontextfenster passen.

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
* **Modellunterstützung:** Claude Sonnet 4.5, Claude Haiku 4.5, Claude Opus 4.5 und spätere Modelle; siehe [Modellkompatibilität in der API-Dokumentation](https://platform.claude.com/docs/de/agents-and-tools/tool-use/tool-search-tool#model-compatibility) für die aktuelle Liste. Auf Google Clouds Agent Platform: Claude Sonnet 4.5 und spätere sowie Claude Opus 4.5 und spätere.

<h2 id="related-documentation">
  Zugehörige Dokumentation
</h2>

* [Tool-Suche in der API](https://platform.claude.com/docs/de/agents-and-tools/tool-use/tool-search-tool): Vollständige API-Dokumentation für die Tool-Suche, einschließlich benutzerdefinierter Implementierungen
* [MCP-Server verbinden](/docs/de/agent-sdk/mcp): Verbindung zu externen Tools über MCP-Server
* [Benutzerdefinierte Tools](/docs/de/agent-sdk/custom-tools): Erstellen Sie Ihre eigenen Tools mit SDK-MCP-Servern
* [TypeScript SDK-Referenz](/docs/de/agent-sdk/typescript): Vollständige API-Referenz
* [Python SDK-Referenz](/docs/de/agent-sdk/python): Vollständige API-Referenz
