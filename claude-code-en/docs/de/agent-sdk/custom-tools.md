> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Geben Sie Claude benutzerdefinierte Tools

> Definieren Sie benutzerdefinierte Tools mit dem In-Process-MCP-Server des Claude Agent SDK, damit Claude Ihre Funktionen aufrufen, Ihre APIs treffen und domänenspezifische Operationen ausführen kann.

Benutzerdefinierte Tools erweitern das Agent SDK, indem Sie Ihre eigenen Funktionen definieren können, die Claude während einer Konversation aufrufen kann. Mit dem In-Process-MCP-Server des SDK können Sie Claude Zugriff auf Datenbanken, externe APIs, domänenspezifische Logik oder jede andere Funktionalität geben, die Ihre Anwendung benötigt.

Dieser Leitfaden behandelt, wie Sie Tools mit Eingabeschemas und Handlern definieren, sie in einen MCP-Server bündeln, sie an `query` übergeben und kontrollieren, auf welche Tools Claude zugreifen kann. Er behandelt auch Fehlerbehandlung, Tool-Annotationen und die Rückgabe von Nicht-Text-Inhalten wie Bildern.

<h2 id="quick-reference">
  Schnellreferenz
</h2>

| Wenn Sie möchten...                                     | Tun Sie dies                                                                                                                                                                                                                                                            |
| :------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Ein Tool definieren                                     | Verwenden Sie [`@tool`](/docs/de/agent-sdk/python#tool) (Python) oder [`tool()`](/docs/de/agent-sdk/typescript#tool) (TypeScript) mit einem Namen, einer Beschreibung, einem Schema und einem Handler. Siehe [Erstellen Sie ein benutzerdefiniertes Tool](#create-a-custom-tool). |
| Ein Tool bei Claude registrieren                        | Wickeln Sie in `create_sdk_mcp_server` / `createSdkMcpServer` ein und übergeben Sie es an `mcpServers` in `query()`. Siehe [Rufen Sie ein benutzerdefiniertes Tool auf](#call-a-custom-tool).                                                                           |
| Ein Tool vorab genehmigen                               | Fügen Sie es zu Ihren zulässigen Tools hinzu. Siehe [Konfigurieren Sie zulässige Tools](#configure-allowed-tools).                                                                                                                                                      |
| Entfernen Sie ein integriertes Tool aus Claudes Kontext | Übergeben Sie ein `tools`-Array, das nur die gewünschten integrierten Tools auflistet. Siehe [Konfigurieren Sie zulässige Tools](#configure-allowed-tools).                                                                                                             |
| Lassen Sie Claude Tools parallel aufrufen               | Setzen Sie `readOnlyHint: true` auf Tools ohne Nebenwirkungen. Siehe [Fügen Sie Tool-Annotationen hinzu](#add-tool-annotations).                                                                                                                                        |
| Kontrollieren Sie die Fehlermeldung, die Claude liest   | Geben Sie `isError: true` zurück, um die Nachricht zu verfassen, anstatt die rohe Ausnahme zu zeigen. Siehe [Fehler behandeln](#handle-errors).                                                                                                                         |
| Geben Sie Bilder oder Dateien zurück                    | Verwenden Sie `image`- oder `resource`-Blöcke im Content-Array. Siehe [Geben Sie Bilder und Ressourcen zurück](#return-images-and-resources).                                                                                                                           |
| Geben Sie ein maschinenlesbares JSON-Ergebnis zurück    | Setzen Sie `structuredContent` auf das Ergebnis. Siehe [Geben Sie strukturierte Daten zurück](#return-structured-data).                                                                                                                                                 |
| Skalieren Sie auf viele Tools                           | Verwenden Sie [Tool-Suche](/docs/de/agent-sdk/tool-search), um Tools bei Bedarf zu laden.                                                                                                                                                                                    |

<h2 id="create-a-custom-tool">
  Erstellen Sie ein benutzerdefiniertes Tool
</h2>

Ein Tool wird durch vier Teile definiert, die als Argumente an den [`tool()`](/docs/de/agent-sdk/typescript#tool)-Helper in TypeScript oder den [`@tool`](/docs/de/agent-sdk/python#tool)-Dekorator in Python übergeben werden:

* **Name:** ein eindeutiger Bezeichner, den Claude verwendet, um das Tool aufzurufen.
* **Beschreibung:** was das Tool tut. Claude liest dies, um zu entscheiden, wann es aufgerufen werden soll.
* **Eingabeschema:** die Argumente, die Claude bereitstellen muss. In TypeScript ist dies immer ein [Zod-Schema](https://zod.dev/), und die `args` des Handlers werden automatisch davon typisiert. In Python ist dies ein Dict, das Namen auf Typen abbildet, wie `{"latitude": float}`, das das SDK für Sie in JSON Schema konvertiert. Der Python-Dekorator akzeptiert auch direkt ein vollständiges [JSON Schema](https://json-schema.org/understanding-json-schema/about)-Dict, wenn Sie Enums, Bereiche, optionale Felder oder verschachtelte Objekte benötigen.
* **Handler:** die asynchrone Funktion, die ausgeführt wird, wenn Claude das Tool aufruft. Sie empfängt die validierten Argumente und muss ein Objekt mit folgenden Eigenschaften zurückgeben:
  * `content` (erforderlich): ein Array von Ergebnisblöcken, jeder mit einem `type` von `"text"`, `"image"`, `"audio"`, `"resource"` oder `"resource_link"`. Siehe [Geben Sie Bilder und Ressourcen zurück](#return-images-and-resources) für Nicht-Text-Blöcke.
  * `structuredContent` (optional): ein JSON-Objekt, das das Ergebnis als maschinenlesbare Daten enthält, das zusammen mit `content` zurückgegeben wird. Siehe [Geben Sie strukturierte Daten zurück](#return-structured-data).
  * `isError` (optional): setzen Sie auf `true`, um einen Tool-Fehler zu signalisieren, damit Claude darauf reagieren kann. Siehe [Fehler behandeln](#handle-errors).

Nach dem Definieren eines Tools wickeln Sie es mit [`createSdkMcpServer`](/docs/de/agent-sdk/typescript#createsdkmcpserver) (TypeScript) oder [`create_sdk_mcp_server`](/docs/de/agent-sdk/python#create_sdk_mcp_server) (Python) in einen Server ein. Der Server läuft im Prozess in Ihrer Anwendung, nicht als separater Prozess.

<h3 id="weather-tool-example">
  Beispiel für ein Wetter-Tool
</h3>

Dieses Beispiel definiert ein `get_temperature`-Tool und wickelt es in einen MCP-Server ein. Es richtet nur das Tool ein; um es an `query` zu übergeben und auszuführen, siehe [Rufen Sie ein benutzerdefiniertes Tool auf](#call-a-custom-tool) unten.

<CodeGroup>
  ```python Python theme={null}
  from typing import Any
  import httpx
  from claude_agent_sdk import tool, create_sdk_mcp_server


  # Define a tool: name, description, input schema, handler
  @tool(
      "get_temperature",
      "Get the current temperature at a location",
      {"latitude": float, "longitude": float},
  )
  async def get_temperature(args: dict[str, Any]) -> dict[str, Any]:
      async with httpx.AsyncClient() as client:
          response = await client.get(
              "https://api.open-meteo.com/v1/forecast",
              params={
                  "latitude": args["latitude"],
                  "longitude": args["longitude"],
                  "current": "temperature_2m",
                  "temperature_unit": "fahrenheit",
              },
          )
          data = response.json()

      # Return a content array - Claude sees this as the tool result
      return {
          "content": [
              {
                  "type": "text",
                  "text": f"Temperature: {data['current']['temperature_2m']}°F",
              }
          ]
      }


  # Wrap the tool in an in-process MCP server
  weather_server = create_sdk_mcp_server(
      name="weather",
      version="1.0.0",
      tools=[get_temperature],
  )
  ```

  ```typescript TypeScript theme={null}
  import { tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  // Define a tool: name, description, input schema, handler
  const getTemperature = tool(
    "get_temperature",
    "Get the current temperature at a location",
    {
      latitude: z.number().describe("Latitude coordinate"), // .describe() adds a field description Claude sees
      longitude: z.number().describe("Longitude coordinate")
    },
    async (args) => {
      // args is typed from the schema: { latitude: number; longitude: number }
      const response = await fetch(
        `https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&current=temperature_2m&temperature_unit=fahrenheit`
      );
      const data: any = await response.json();

      // Return a content array - Claude sees this as the tool result
      return {
        content: [{ type: "text", text: `Temperature: ${data.current.temperature_2m}°F` }]
      };
    }
  );

  // Wrap the tool in an in-process MCP server
  const weatherServer = createSdkMcpServer({
    name: "weather",
    version: "1.0.0",
    tools: [getTemperature]
  });
  ```
</CodeGroup>

Siehe die [`tool()`](/docs/de/agent-sdk/typescript#tool)-TypeScript-Referenz oder die [`@tool`](/docs/de/agent-sdk/python#tool)-Python-Referenz für vollständige Parameterdetails, einschließlich JSON-Schema-Eingabeformate und Rückgabewertstruktur.

<Tip>
  Um einen Parameter optional zu machen: Fügen Sie in TypeScript `.default()` zum Zod-Feld hinzu. In Python behandelt das Dict-Schema jeden Schlüssel als erforderlich, also lassen Sie den Parameter aus dem Schema weg, erwähnen Sie ihn in der Beschreibungszeichenkette und lesen Sie ihn mit `args.get()` im Handler. Das [`get_precipitation_chance`-Tool unten](#add-more-tools) zeigt beide Muster.
</Tip>

<h3 id="call-a-custom-tool">
  Rufen Sie ein benutzerdefiniertes Tool auf
</h3>

Übergeben Sie den MCP-Server, den Sie erstellt haben, an `query` über die `mcpServers`-Option. Der Schlüssel in `mcpServers` wird zum `{server_name}`-Segment im vollständig qualifizierten Namen jedes Tools: `mcp__{server_name}__{tool_name}`. Listen Sie diesen Namen in `allowedTools` auf, damit das Tool ohne Genehmigungsaufforderung ausgeführt wird.

Diese Snippets verwenden den `weatherServer` aus dem [Beispiel oben](#weather-tool-example) wieder, um Claude zu fragen, wie das Wetter an einem bestimmten Ort ist.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={"weather": weather_server},
          allowed_tools=["mcp__weather__get_temperature"],
      )

      async for message in query(
          prompt="What's the temperature in San Francisco?",
          options=options,
      ):
          # ResultMessage is the final message after all tool calls complete
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "What's the temperature in San Francisco?",
    options: {
      mcpServers: { weather: weatherServer },
      allowedTools: ["mcp__weather__get_temperature"]
    }
  })) {
    // "result" is the final message after all tool calls complete
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

<h3 id="add-more-tools">
  Fügen Sie weitere Tools hinzu
</h3>

Ein Server enthält so viele Tools, wie Sie in seinem `tools`-Array auflisten. Mit mehr als einem Tool auf einem Server können Sie jedes einzelne in `allowedTools` auflisten oder das Wildcard `mcp__weather__*` verwenden, um alle Tools abzudecken, die der Server verfügbar macht.

Das Beispiel unten fügt ein zweites Tool, `get_precipitation_chance`, zum `weatherServer` aus dem [Wetter-Tool-Beispiel](#weather-tool-example) hinzu und erstellt ihn mit beiden Tools im Array neu.

<CodeGroup>
  ```python Python theme={null}
  # Define a second tool for the same server
  @tool(
      "get_precipitation_chance",
      "Get the hourly precipitation probability for a location. "
      "Optionally pass 'hours' (1-24) to control how many hours to return.",
      {"latitude": float, "longitude": float},
  )
  async def get_precipitation_chance(args: dict[str, Any]) -> dict[str, Any]:
      # 'hours' isn't in the schema - read it with .get() to make it optional
      hours = args.get("hours", 12)
      async with httpx.AsyncClient() as client:
          response = await client.get(
              "https://api.open-meteo.com/v1/forecast",
              params={
                  "latitude": args["latitude"],
                  "longitude": args["longitude"],
                  "hourly": "precipitation_probability",
                  "forecast_days": 1,
              },
          )
          data = response.json()
      chances = data["hourly"]["precipitation_probability"][:hours]

      return {
          "content": [
              {
                  "type": "text",
                  "text": f"Next {hours} hours: {'%, '.join(map(str, chances))}%",
              }
          ]
      }


  # Rebuild the server with both tools in the array
  weather_server = create_sdk_mcp_server(
      name="weather",
      version="1.0.0",
      tools=[get_temperature, get_precipitation_chance],
  )
  ```

  ```typescript TypeScript theme={null}
  // Define a second tool for the same server
  const getPrecipitationChance = tool(
    "get_precipitation_chance",
    "Get the hourly precipitation probability for a location",
    {
      latitude: z.number(),
      longitude: z.number(),
      hours: z
        .number()
        .int()
        .min(1)
        .max(24)
        .default(12) // .default() makes the parameter optional
        .describe("How many hours of forecast to return")
    },
    async (args) => {
      const response = await fetch(
        `https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&hourly=precipitation_probability&forecast_days=1`
      );
      const data: any = await response.json();
      const chances = data.hourly.precipitation_probability.slice(0, args.hours);

      return {
        content: [{ type: "text", text: `Next ${args.hours} hours: ${chances.join("%, ")}%` }]
      };
    }
  );

  // Rebuild the server with both tools in the array
  const weatherServer = createSdkMcpServer({
    name: "weather",
    version: "1.0.0",
    tools: [getTemperature, getPrecipitationChance]
  });
  ```
</CodeGroup>

Jedes Tool in diesem Array verbraucht Kontextfensterplatz bei jedem Durchgang. Wenn Sie Dutzende von Tools definieren, siehe [Tool-Suche](/docs/de/agent-sdk/tool-search), um sie stattdessen bei Bedarf zu laden.

<h3 id="add-tool-annotations">
  Fügen Sie Tool-Annotationen hinzu
</h3>

[Tool-Annotationen](https://modelcontextprotocol.io/docs/concepts/tools#tool-annotations) sind optionale Metadaten, die beschreiben, wie sich ein Tool verhält. Übergeben Sie sie als fünftes Argument an den `tool()`-Helper in TypeScript oder über das `annotations`-Schlüsselwortargument für den `@tool`-Dekorator in Python. Alle Hint-Felder sind Boolesche Werte.

| Feld              | Standard | Bedeutung                                                                                                                    |
| :---------------- | :------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `readOnlyHint`    | `false`  | Tool ändert seine Umgebung nicht. Steuert, ob das Tool parallel mit anderen schreibgeschützten Tools aufgerufen werden kann. |
| `destructiveHint` | `true`   | Tool kann destruktive Updates durchführen. Nur informativ.                                                                   |
| `idempotentHint`  | `false`  | Wiederholte Aufrufe mit denselben Argumenten haben keine zusätzliche Auswirkung. Nur informativ.                             |
| `openWorldHint`   | `true`   | Tool erreicht Systeme außerhalb Ihres Prozesses. Nur informativ.                                                             |

Annotationen sind Metadaten, keine Durchsetzung. Ein Tool, das mit `readOnlyHint: true` markiert ist, kann immer noch auf die Festplatte schreiben, wenn das der Handler tut. Halten Sie die Annotation genau zum Handler.

Dieses Beispiel fügt `readOnlyHint` zum `get_temperature`-Tool aus dem [Wetter-Tool-Beispiel](#weather-tool-example) hinzu.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import tool, ToolAnnotations


  @tool(
      "get_temperature",
      "Get the current temperature at a location",
      {"latitude": float, "longitude": float},
      annotations=ToolAnnotations(
          readOnlyHint=True
      ),  # Lets Claude batch this with other read-only calls
  )
  async def get_temperature(args):
      return {"content": [{"type": "text", "text": "..."}]}
  ```

  ```typescript TypeScript theme={null}
  tool(
    "get_temperature",
    "Get the current temperature at a location",
    { latitude: z.number(), longitude: z.number() },
    async (args) => ({ content: [{ type: "text", text: `...` }] }),
    { annotations: { readOnlyHint: true } } // Lets Claude batch this with other read-only calls
  );
  ```
</CodeGroup>

Siehe `ToolAnnotations` in der [TypeScript](/docs/de/agent-sdk/typescript#toolannotations)- oder [Python](/docs/de/agent-sdk/python#toolannotations)-Referenz.

<h2 id="control-tool-access">
  Tool-Zugriff kontrollieren
</h2>

Das [Wetter-Tool-Beispiel](#weather-tool-example) registrierte einen Server und listete Tools in `allowedTools` auf. Dieser Abschnitt behandelt, wie Tool-Namen konstruiert werden und wie Sie den Zugriff scoped, wenn Sie mehrere Tools haben oder integrierte Tools einschränken möchten.

<h3 id="tool-name-format">
  Tool-Namensformat
</h3>

Wenn MCP-Tools Claude verfügbar gemacht werden, folgen ihre Namen einem bestimmten Format:

* Muster: `mcp__{server_name}__{tool_name}`
* Beispiel: Ein Tool namens `get_temperature` im Server `weather` wird zu `mcp__weather__get_temperature`

<h3 id="configure-allowed-tools">
  Zulässige Tools konfigurieren
</h3>

Die `tools`-Option und die zulässigen/nicht zulässigen Listen beeinflussen zwei Ebenen: Verfügbarkeit, die steuert, ob ein Tool in Claudes Kontext angezeigt wird, und Berechtigung, die steuert, ob ein Aufruf genehmigt wird, sobald Claude ihn versucht. `tools` und einfache `disallowedTools`-Einträge ändern die Verfügbarkeit. `allowedTools` und scoped `disallowedTools`-Regeln ändern nur die Berechtigung.

| Option                    | Ebene         | Auswirkung                                                                                                                                                                                                                   |
| :------------------------ | :------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tools: ["Read", "Grep"]` | Verfügbarkeit | Nur die aufgelisteten integrierten Tools sind in Claudes Kontext. Nicht aufgelistete integrierte Tools werden entfernt. MCP-Tools sind nicht betroffen.                                                                      |
| `tools: []`               | Verfügbarkeit | Alle integrierten Tools werden entfernt. Claude kann nur Ihre MCP-Tools verwenden.                                                                                                                                           |
| zulässige Tools           | Berechtigung  | Aufgelistete Tools werden ohne Genehmigungsaufforderung ausgeführt. Nicht aufgelistete Tools bleiben verfügbar; Aufrufe gehen durch den [Genehmigungsfluss](/docs/de/agent-sdk/permissions).                                      |
| nicht zulässige Tools     | Beide         | Ein einfacher Tool-Name wie `"Bash"` entfernt das Tool aus Claudes Kontext, genauso wie das Weglassen aus `tools`. Eine scoped-Regel wie `"Bash(rm *)"` lässt das Tool im Kontext und lehnt nur übereinstimmende Aufrufe ab. |

Um ein integriertes Tool vollständig zu entfernen, lassen Sie es aus `tools` weg oder listen Sie seinen einfachen Namen in `disallowedTools` (Python: `disallowed_tools`) auf; beide halten das Tool aus dem Kontext, damit Claude es nie versucht. Eine scoped `disallowedTools`-Regel blockiert übereinstimmende Aufrufe, lässt das Tool aber sichtbar, daher kann Claude möglicherweise einen Durchgang damit verschwenden. Siehe [Berechtigungen konfigurieren](/docs/de/agent-sdk/permissions) für die vollständige Evaluierungsreihenfolge.

<h2 id="handle-errors">
  Fehler behandeln
</h2>

Ein Handler-Fehler stoppt die Agent-Schleife nicht. Der In-Process-MCP-Server des SDK fängt nicht abgefangene Ausnahmen ab und gibt sie als Fehler-Ergebnisse zurück. Daher bestimmt, wie Sie einen Fehler melden, was Claude liest, nicht ob die Abfrage fehlschlägt:

| Was passiert                                                                                   | Ergebnis                                                                                                                                                                                    |
| :--------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Handler wirft eine nicht abgefangene Ausnahme                                                  | Der MCP-Server konvertiert sie in ein Fehler-Ergebnis mit der rohen Ausnahmemeldung. Claude sieht diese Meldung, und die Agent-Schleife setzt sich fort.                                    |
| Handler fängt den Fehler ab und gibt `isError: true` (TS) / `"is_error": True` (Python) zurück | Claude sieht die Meldung, die Sie verfassen. Sie können Kontext hinzufügen, den die rohe Ausnahme nicht hat, z. B. welche Anfrage fehlgeschlagen ist oder was stattdessen zu versuchen ist. |

In beiden Fällen kann Claude erneut versuchen, ein anderes Tool versuchen oder den Fehler erklären. Fangen Sie Fehler selbst ab, wenn die rohe Ausnahmemeldung nicht ausreicht, damit Claude handeln kann.

Das Beispiel unten fängt zwei Arten von Fehlern im Handler ab und verfasst die Fehlermeldung, die Claude liest. Ein Nicht-200-HTTP-Status wird aus der Antwort abgefangen und als Fehler-Ergebnis zurückgegeben. Ein Netzwerkfehler oder ungültiges JSON wird durch das umgebende `try/except` (Python) oder `try/catch` (TypeScript) abgefangen und auch als Fehler-Ergebnis zurückgegeben. In beiden Fällen erhält Claude eine Meldung, die den Fehler beschreibt, anstatt einer bloßen Ausnahmemeldung.

<CodeGroup>
  ```python Python theme={null}
  import json
  import httpx
  from typing import Any


  @tool(
      "fetch_data",
      "Fetch data from an API",
      {"endpoint": str},  # Simple schema
  )
  async def fetch_data(args: dict[str, Any]) -> dict[str, Any]:
      try:
          async with httpx.AsyncClient() as client:
              response = await client.get(args["endpoint"])
              if response.status_code != 200:
                  # Return the failure as a tool result so Claude can react to it.
                  # is_error marks this as a failed call rather than odd-looking data.
                  return {
                      "content": [
                          {
                              "type": "text",
                              "text": f"API error: {response.status_code} {response.reason_phrase}",
                          }
                      ],
                      "is_error": True,
                  }

              data = response.json()
              return {"content": [{"type": "text", "text": json.dumps(data, indent=2)}]}
      except Exception as e:
          # Composes the message Claude reads. An uncaught exception would
          # reach Claude as the raw str(e) with no context.
          return {
              "content": [{"type": "text", "text": f"Failed to fetch data: {str(e)}"}],
              "is_error": True,
          }
  ```

  ```typescript TypeScript theme={null}
  tool(
    "fetch_data",
    "Fetch data from an API",
    {
      endpoint: z.string().url().describe("API endpoint URL")
    },
    async (args) => {
      try {
        const response = await fetch(args.endpoint);

        if (!response.ok) {
          // Return the failure as a tool result so Claude can react to it.
          // isError marks this as a failed call rather than odd-looking data.
          return {
            content: [
              {
                type: "text",
                text: `API error: ${response.status} ${response.statusText}`
              }
            ],
            isError: true
          };
        }

        const data = await response.json();
        return {
          content: [
            {
              type: "text",
              text: JSON.stringify(data, null, 2)
            }
          ]
        };
      } catch (error) {
        // Composes the message Claude reads. An uncaught throw would
        // reach Claude as the raw error message with no context.
        return {
          content: [
            {
              type: "text",
              text: `Failed to fetch data: ${error instanceof Error ? error.message : String(error)}`
            }
          ],
          isError: true
        };
      }
    }
  );
  ```
</CodeGroup>

<h2 id="return-images-and-resources">
  Geben Sie Bilder und Ressourcen zurück
</h2>

Das `content`-Array in einem Tool-Ergebnis akzeptiert `text`-, `image`-, `audio`-, `resource`- und `resource_link`-Blöcke. Sie können sie in derselben Antwort mischen. In TypeScript werden Audio-Blöcke auf der Festplatte gespeichert und Claude erhält einen Text-Block mit dem gespeicherten Dateipfad; in Python löscht das SDK Audio-Blöcke aus dem Tool-Ergebnis und protokolliert eine Warnung. Resource-Link-Blöcke werden in einen Text-Block konvertiert, der den Namen, die URI und die Beschreibung des Links enthält.

<h3 id="images">
  Bilder
</h3>

Ein Bildblock trägt die Bildbytes inline, kodiert als Base64. Es gibt kein URL-Feld. Um ein Bild zurückzugeben, das sich unter einer URL befindet, rufen Sie es im Handler ab, lesen Sie die Antwortbytes und kodieren Sie sie Base64, bevor Sie sie zurückgeben. Das Ergebnis wird als visueller Input verarbeitet.

| Feld       | Typ       | Notizen                                                                         |
| :--------- | :-------- | :------------------------------------------------------------------------------ |
| `type`     | `"image"` |                                                                                 |
| `data`     | `string`  | Base64-kodierte Bytes. Nur rohes Base64, kein `data:image/...;base64,`-Präfix   |
| `mimeType` | `string`  | Erforderlich. Zum Beispiel `image/png`, `image/jpeg`, `image/webp`, `image/gif` |

<CodeGroup>
  ```python Python theme={null}
  import base64
  import httpx


  # Define a tool that fetches an image from a URL and returns it to Claude
  @tool("fetch_image", "Fetch an image from a URL and return it to Claude", {"url": str})
  async def fetch_image(args):
      async with httpx.AsyncClient() as client:  # Fetch the image bytes
          response = await client.get(args["url"])

      return {
          "content": [
              {
                  "type": "image",
                  "data": base64.b64encode(response.content).decode(
                      "ascii"
                  ),  # Base64-encode the raw bytes
                  "mimeType": response.headers.get(
                      "content-type", "image/png"
                  ),  # Read MIME type from the response
              }
          ]
      }
  ```

  ```typescript TypeScript theme={null}
  tool(
    "fetch_image",
    "Fetch an image from a URL and return it to Claude",
    {
      url: z.string().url()
    },
    async (args) => {
      const response = await fetch(args.url); // Fetch the image bytes
      const buffer = Buffer.from(await response.arrayBuffer()); // Read into a Buffer for base64 encoding
      const mimeType = response.headers.get("content-type") ?? "image/png";

      return {
        content: [
          {
            type: "image",
            data: buffer.toString("base64"), // Base64-encode the raw bytes
            mimeType
          }
        ]
      };
    }
  );
  ```
</CodeGroup>

<h3 id="resources">
  Ressourcen
</h3>

Ein Ressourcenblock bettet ein Stück Inhalt ein, das durch einen URI identifiziert wird. Der URI ist ein Label für Claude, um darauf zu verweisen; der tatsächliche Inhalt befindet sich im `text`- oder `blob`-Feld des Blocks. Verwenden Sie dies, wenn Ihr Tool etwas produziert, das sinnvoll ist, um später nach Name adressiert zu werden, wie eine generierte Datei oder ein Datensatz aus einem externen System.

| Feld                | Typ          | Notizen                                                                                                                                                    |
| :------------------ | :----------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`              | `"resource"` |                                                                                                                                                            |
| `resource.uri`      | `string`     | Bezeichner für den Inhalt. Beliebiges URI-Schema                                                                                                           |
| `resource.text`     | `string`     | Der Inhalt, wenn er Text ist. Geben Sie dies oder `blob` an, nicht beide                                                                                   |
| `resource.blob`     | `string`     | Der Inhalt Base64-kodiert, wenn er binär ist. Nur TypeScript: das Python SDK löscht binäre Ressourcen aus dem Tool-Ergebnis und protokolliert eine Warnung |
| `resource.mimeType` | `string`     | Optional                                                                                                                                                   |

Dieses Beispiel zeigt einen Ressourcenblock, der von innen aus einem Tool-Handler zurückgegeben wird. Der URI `file:///tmp/report.md` ist ein Label, das Claude später referenzieren kann; das SDK liest nicht aus diesem Pfad.

<CodeGroup>
  ```typescript TypeScript theme={null}
  return {
    content: [
      {
        type: "resource",
        resource: {
          uri: "file:///tmp/report.md", // Label for Claude to reference, not a path the SDK reads
          mimeType: "text/markdown",
          text: "# Report\n..." // The actual content, inline
        }
      }
    ]
  };
  ```

  ```python Python theme={null}
  return {
      "content": [
          {
              "type": "resource",
              "resource": {
                  "uri": "file:///tmp/report.md",  # Label for Claude to reference, not a path the SDK reads
                  "mimeType": "text/markdown",
                  "text": "# Report\n...",  # The actual content, inline
              },
          }
      ]
  }
  ```
</CodeGroup>

Diese Block-Formen stammen aus dem MCP-`CallToolResult`-Typ. Siehe die [MCP-Spezifikation](https://modelcontextprotocol.io/specification/2025-06-18/server/tools#tool-result) für die vollständige Definition.

<h2 id="return-structured-data">
  Geben Sie strukturierte Daten zurück
</h2>

`structuredContent` ist ein optionales JSON-Objekt auf dem Ergebnis, getrennt vom `content`-Array. Verwenden Sie es, um Rohwerte zurückzugeben, die Claude als exakte Felder lesen kann, anstatt sie aus einer Textzeichenkette oder einem Bild zu analysieren.

Wenn `structuredContent` gesetzt ist, empfängt Claude das JSON plus alle Bild- oder Ressourcenblöcke aus `content`. Textblöcke in `content` werden nicht weitergeleitet, da angenommen wird, dass sie die strukturierten Daten duplizieren. Das Beispiel unten rendert ein Diagramm als Bildblock und gibt die Datenpunkte dahinter in `structuredContent` vom selben Handler zurück.

```typescript TypeScript theme={null}
return {
  content: [
    {
      type: "image",
      data: chartPngBuffer.toString("base64"),
      mimeType: "image/png"
    }
  ],
  structuredContent: {
    series: "temperature_2m",
    unit: "fahrenheit",
    points: [62.1, 63.4, 65.0, 64.2]
  }
};
```

<Note>
  Der Python-`@tool`-Dekorator leitet nur `content` und `is_error` aus dem Rückgabe-Dict des Handlers weiter. Um `structuredContent` von Python zurückzugeben, führen Sie stattdessen einen [eigenständigen MCP-Server](/docs/de/agent-sdk/mcp) aus.
</Note>

<h2 id="example-unit-converter">
  Beispiel: Einheitenkonverter
</h2>

Dieses Tool konvertiert Werte zwischen Einheiten der Länge, Temperatur und des Gewichts. Ein Benutzer kann fragen „100 Kilometer in Meilen konvertieren" oder „Was ist 72°F in Celsius", und Claude wählt den richtigen Einheitstyp und die Einheiten aus der Anfrage.

Es demonstriert zwei Muster:

* **Enum-Schemas:** `unit_type` ist auf einen festen Satz von Werten beschränkt. In TypeScript verwenden Sie `z.enum()`. In Python unterstützt das Dict-Schema keine Enums, daher ist das vollständige JSON-Schema-Dict erforderlich.
* **Behandlung nicht unterstützter Eingaben:** Wenn ein Konvertierungspaar nicht gefunden wird, gibt der Handler `isError: true` zurück, damit Claude dem Benutzer sagen kann, was schief gelaufen ist, anstatt einen Fehler als normales Ergebnis zu behandeln.

<CodeGroup>
  ```python Python theme={null}
  from typing import Any
  from claude_agent_sdk import tool, create_sdk_mcp_server


  # z.enum() in TypeScript becomes an "enum" constraint in JSON Schema.
  # The dict schema has no equivalent, so full JSON Schema is required.
  @tool(
      "convert_units",
      "Convert a value from one unit to another",
      {
          "type": "object",
          "properties": {
              "unit_type": {
                  "type": "string",
                  "enum": ["length", "temperature", "weight"],
                  "description": "Category of unit",
              },
              "from_unit": {
                  "type": "string",
                  "description": "Unit to convert from, e.g. kilometers, fahrenheit, pounds",
              },
              "to_unit": {"type": "string", "description": "Unit to convert to"},
              "value": {"type": "number", "description": "Value to convert"},
          },
          "required": ["unit_type", "from_unit", "to_unit", "value"],
      },
  )
  async def convert_units(args: dict[str, Any]) -> dict[str, Any]:
      conversions = {
          "length": {
              "kilometers_to_miles": lambda v: v * 0.621371,
              "miles_to_kilometers": lambda v: v * 1.60934,
              "meters_to_feet": lambda v: v * 3.28084,
              "feet_to_meters": lambda v: v * 0.3048,
          },
          "temperature": {
              "celsius_to_fahrenheit": lambda v: (v * 9) / 5 + 32,
              "fahrenheit_to_celsius": lambda v: (v - 32) * 5 / 9,
              "celsius_to_kelvin": lambda v: v + 273.15,
              "kelvin_to_celsius": lambda v: v - 273.15,
          },
          "weight": {
              "kilograms_to_pounds": lambda v: v * 2.20462,
              "pounds_to_kilograms": lambda v: v * 0.453592,
              "grams_to_ounces": lambda v: v * 0.035274,
              "ounces_to_grams": lambda v: v * 28.3495,
          },
      }

      key = f"{args['from_unit']}_to_{args['to_unit']}"
      fn = conversions.get(args["unit_type"], {}).get(key)

      if not fn:
          return {
              "content": [
                  {
                      "type": "text",
                      "text": f"Unsupported conversion: {args['from_unit']} to {args['to_unit']}",
                  }
              ],
              "is_error": True,
          }

      result = fn(args["value"])
      return {
          "content": [
              {
                  "type": "text",
                  "text": f"{args['value']} {args['from_unit']} = {result:.4f} {args['to_unit']}",
              }
          ]
      }


  converter_server = create_sdk_mcp_server(
      name="converter",
      version="1.0.0",
      tools=[convert_units],
  )
  ```

  ```typescript TypeScript theme={null}
  import { tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  const convert = tool(
    "convert_units",
    "Convert a value from one unit to another",
    {
      unit_type: z.enum(["length", "temperature", "weight"]).describe("Category of unit"),
      from_unit: z
        .string()
        .describe("Unit to convert from, e.g. kilometers, fahrenheit, pounds"),
      to_unit: z.string().describe("Unit to convert to"),
      value: z.number().describe("Value to convert")
    },
    async (args) => {
      type Conversions = Record<string, Record<string, (v: number) => number>>;

      const conversions: Conversions = {
        length: {
          kilometers_to_miles: (v) => v * 0.621371,
          miles_to_kilometers: (v) => v * 1.60934,
          meters_to_feet: (v) => v * 3.28084,
          feet_to_meters: (v) => v * 0.3048
        },
        temperature: {
          celsius_to_fahrenheit: (v) => (v * 9) / 5 + 32,
          fahrenheit_to_celsius: (v) => ((v - 32) * 5) / 9,
          celsius_to_kelvin: (v) => v + 273.15,
          kelvin_to_celsius: (v) => v - 273.15
        },
        weight: {
          kilograms_to_pounds: (v) => v * 2.20462,
          pounds_to_kilograms: (v) => v * 0.453592,
          grams_to_ounces: (v) => v * 0.035274,
          ounces_to_grams: (v) => v * 28.3495
        }
      };

      const key = `${args.from_unit}_to_${args.to_unit}`;
      const fn = conversions[args.unit_type]?.[key];

      if (!fn) {
        return {
          content: [
            {
              type: "text",
              text: `Unsupported conversion: ${args.from_unit} to ${args.to_unit}`
            }
          ],
          isError: true
        };
      }

      const result = fn(args.value);
      return {
        content: [
          {
            type: "text",
            text: `${args.value} ${args.from_unit} = ${result.toFixed(4)} ${args.to_unit}`
          }
        ]
      };
    }
  );

  const converterServer = createSdkMcpServer({
    name: "converter",
    version: "1.0.0",
    tools: [convert]
  });
  ```
</CodeGroup>

Sobald der Server definiert ist, übergeben Sie ihn an `query` auf die gleiche Weise wie das Wetter-Beispiel. Dieses Beispiel sendet drei verschiedene Prompts in einer Schleife, um zu zeigen, wie dasselbe Tool verschiedene Einheitstypen handhabt. Für jede Antwort inspiziert es `AssistantMessage`-Objekte (die die Tool-Aufrufe enthalten, die Claude während dieses Durchgangs gemacht hat) und gibt jeden `ToolUseBlock` aus, bevor es den endgültigen `ResultMessage`-Text ausgibt. Dies lässt Sie sehen, wann Claude das Tool verwendet, im Gegensatz zu Antworten aus seinem eigenen Wissen.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      query,
      ClaudeAgentOptions,
      ResultMessage,
      AssistantMessage,
      ToolUseBlock,
  )


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={"converter": converter_server},
          allowed_tools=["mcp__converter__convert_units"],
      )

      prompts = [
          "Convert 100 kilometers to miles.",
          "What is 72°F in Celsius?",
          "How many pounds is 5 kilograms?",
      ]

      for prompt in prompts:
          async for message in query(prompt=prompt, options=options):
              if isinstance(message, AssistantMessage):
                  for block in message.content:
                      if isinstance(block, ToolUseBlock):
                          print(f"[tool call] {block.name}({block.input})")
              elif isinstance(message, ResultMessage) and message.subtype == "success":
                  print(f"Q: {prompt}\nA: {message.result}\n")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const prompts = [
    "Convert 100 kilometers to miles.",
    "What is 72°F in Celsius?",
    "How many pounds is 5 kilograms?"
  ];

  for (const prompt of prompts) {
    for await (const message of query({
      prompt,
      options: {
        mcpServers: { converter: converterServer },
        allowedTools: ["mcp__converter__convert_units"]
      }
    })) {
      if (message.type === "assistant") {
        for (const block of message.message.content) {
          if (block.type === "tool_use") {
            console.log(`[tool call] ${block.name}`, block.input);
          }
        }
      } else if (message.type === "result" && message.subtype === "success") {
        console.log(`Q: ${prompt}\nA: ${message.result}\n`);
      }
    }
  }
  ```
</CodeGroup>

<h2 id="next-steps">
  Nächste Schritte
</h2>

Benutzerdefinierte Tools wickeln asynchrone Funktionen in einer Standardschnittstelle ein. Sie können die Muster auf dieser Seite im selben Server mischen: Ein einzelner Server kann ein Datenbank-Tool, ein API-Gateway-Tool und einen Bild-Renderer nebeneinander halten.

Von hier aus:

* Wenn Ihr Server auf Dutzende von Tools wächst, siehe [Tool-Suche](/docs/de/agent-sdk/tool-search), um das Laden zu verschieben, bis Claude sie benötigt.
* Um sich mit externen MCP-Servern (Dateisystem, GitHub, Slack) zu verbinden, anstatt Ihre eigenen zu erstellen, siehe [Verbinden Sie MCP-Server](/docs/de/agent-sdk/mcp).
* Um zu kontrollieren, welche Tools automatisch ausgeführt werden, im Gegensatz zu denen, die Genehmigung erfordern, siehe [Konfigurieren Sie Berechtigungen](/docs/de/agent-sdk/permissions).

<h2 id="related-documentation">
  Verwandte Dokumentation
</h2>

* [TypeScript SDK-Referenz](/docs/de/agent-sdk/typescript)
* [Python SDK-Referenz](/docs/de/agent-sdk/python)
* [MCP-Dokumentation](https://modelcontextprotocol.io)
* [SDK-Übersicht](/docs/de/agent-sdk/overview)
