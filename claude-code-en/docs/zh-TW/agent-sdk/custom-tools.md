> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 為 Claude 提供自訂工具

> 使用 Claude Agent SDK 的同程序 MCP 伺服器定義自訂工具，讓 Claude 可以呼叫您的函數、存取您的 API，並執行特定領域的操作。

自訂工具透過讓您定義 Claude 在對話期間可以呼叫的自己的函數來擴展 Agent SDK。使用 SDK 的同程序 MCP 伺服器，您可以讓 Claude 存取資料庫、外部 API、特定領域邏輯或應用程式需要的任何其他功能。

本指南涵蓋如何使用輸入結構描述和處理程式定義工具、將它們組合到 MCP 伺服器中、將它們傳遞給 `query`，以及控制 Claude 可以存取哪些工具。它也涵蓋錯誤處理、工具註解，以及傳回非文字內容（如影像）。

<h2 id="quick-reference">
  快速參考
</h2>

| 如果您想要...            | 執行此操作                                                                                                                                                              |
| :------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 定義工具                | 使用 [`@tool`](/docs/zh-TW/agent-sdk/python#tool)（Python）或 [`tool()`](/docs/zh-TW/agent-sdk/typescript#tool)（TypeScript），搭配名稱、描述、結構描述和處理程式。請參閱[建立自訂工具](#create-a-custom-tool)。 |
| 向 Claude 註冊工具       | 在 `create_sdk_mcp_server` / `createSdkMcpServer` 中包裝，並傳遞至 `query()` 中的 `mcpServers`。請參閱[呼叫自訂工具](#call-a-custom-tool)。                                              |
| 預先核准工具              | 新增至您允許的工具。請參閱[設定允許的工具](#configure-allowed-tools)。                                                                                                                  |
| 從 Claude 的內容中移除內建工具 | 傳遞 `tools` 陣列，僅列出您想要的內建工具。請參閱[設定允許的工具](#configure-allowed-tools)。                                                                                                  |
| 讓 Claude 平行呼叫工具     | 在沒有副作用的工具上設定 `readOnlyHint: true`。請參閱[新增工具註解](#add-tool-annotations)。                                                                                              |
| 控制 Claude 讀取的錯誤訊息   | 傳回 `isError: true` 以組成訊息，而不是顯示原始例外狀況。請參閱[處理錯誤](#handle-errors)。                                                                                                    |
| 傳回影像或檔案             | 在內容陣列中使用 `image` 或 `resource` 區塊。請參閱[傳回影像和資源](#return-images-and-resources)。                                                                                       |
| 傳回機器可讀的 JSON 結果     | 在結果上設定 `structuredContent`。請參閱[傳回結構化資料](#return-structured-data)。                                                                                                  |
| 擴展到許多工具             | 使用[工具搜尋](/docs/zh-TW/agent-sdk/tool-search)按需載入工具。                                                                                                                      |

<h2 id="create-a-custom-tool">
  建立自訂工具
</h2>

工具由四個部分定義，作為 TypeScript 中 [`tool()`](/docs/zh-TW/agent-sdk/typescript#tool) 協助程式或 Python 中 [`@tool`](/docs/zh-TW/agent-sdk/python#tool) 裝飾器的引數傳遞：

* **名稱：** Claude 用來呼叫工具的唯一識別碼。
* **描述：** 工具的功能。Claude 讀取此項以決定何時呼叫它。
* **輸入結構描述：** Claude 必須提供的引數。在 TypeScript 中，這始終是 [Zod 結構描述](https://zod.dev/)，處理程式的 `args` 會自動從中輸入。在 Python 中，這是將名稱對應到類型的字典，例如 `{"latitude": float}`，SDK 會為您轉換為 JSON Schema。Python 裝飾器也接受完整的 [JSON Schema](https://json-schema.org/understanding-json-schema/about) 字典，當您需要列舉、範圍、選用欄位或巢狀物件時。
* **處理程式：** 當 Claude 呼叫工具時執行的非同步函數。它接收驗證的引數，並必須傳回具有以下內容的物件：
  * `content`（必需）：結果區塊的陣列，每個區塊的 `type` 為 `"text"`、`"image"`、`"audio"`、`"resource"` 或 `"resource_link"`。請參閱[傳回影像和資源](#return-images-and-resources)以取得非文字區塊。
  * `structuredContent`（選用）：保存結果作為機器可讀資料的 JSON 物件，與 `content` 一起傳回。請參閱[傳回結構化資料](#return-structured-data)。
  * `isError`（選用）：設定為 `true` 以表示工具失敗，讓 Claude 可以對其做出反應。請參閱[處理錯誤](#handle-errors)。

定義工具後，使用 [`createSdkMcpServer`](/docs/zh-TW/agent-sdk/typescript#createsdkmcpserver)（TypeScript）或 [`create_sdk_mcp_server`](/docs/zh-TW/agent-sdk/python#create_sdk_mcp_server)（Python）將其包裝在伺服器中。伺服器在應用程式內同程序執行，而不是作為單獨的程序。

<h3 id="weather-tool-example">
  天氣工具範例
</h3>

此範例定義 `get_temperature` 工具並將其包裝在 MCP 伺服器中。它只設定工具；若要將其傳遞至 `query` 並執行它，請參閱下面的[呼叫自訂工具](#call-a-custom-tool)。

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

請參閱 [`tool()`](/docs/zh-TW/agent-sdk/typescript#tool) TypeScript 參考或 [`@tool`](/docs/zh-TW/agent-sdk/python#tool) Python 參考，以取得完整的參數詳細資訊，包括 JSON Schema 輸入格式和傳回值結構。

<Tip>
  若要使參數成為選用：在 TypeScript 中，將 `.default()` 新增至 Zod 欄位。在 Python 中，字典結構描述將每個鍵視為必需，因此請將參數留出結構描述，在描述字串中提及它，並在處理程式中使用 `args.get()` 讀取它。下面的 [`get_precipitation_chance` 工具](#add-more-tools)顯示兩種模式。
</Tip>

<h3 id="call-a-custom-tool">
  呼叫自訂工具
</h3>

透過 `mcpServers` 選項將您建立的 MCP 伺服器傳遞至 `query`。`mcpServers` 中的鍵成為每個工具的完全限定名稱中的 `{server_name}` 區段：`mcp__{server_name}__{tool_name}`。在 `allowedTools` 中列出該名稱，以便工具執行而不會出現權限提示。

這些程式碼片段重複使用上面[範例](#weather-tool-example)中的 `weatherServer`，以詢問 Claude 特定位置的天氣。

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
  新增更多工具
</h3>

伺服器在其 `tools` 陣列中列出的工具一樣多。如果有多個工具在伺服器上，您可以在 `allowedTools` 中個別列出每個工具，或使用萬用字元 `mcp__weather__*` 來涵蓋伺服器公開的每個工具。

下面的範例將第二個工具 `get_precipitation_chance` 新增至[天氣工具範例](#weather-tool-example)中的 `weatherServer`，並使用陣列中的兩個工具重建它。

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

此陣列中的每個工具在每個回合都會消耗內容視窗空間。如果您定義了數十個工具，請參閱[工具搜尋](/docs/zh-TW/agent-sdk/tool-search)以改為按需載入它們。

<h3 id="add-tool-annotations">
  新增工具註解
</h3>

[工具註解](https://modelcontextprotocol.io/docs/concepts/tools#tool-annotations)是描述工具行為方式的選用中繼資料。在 TypeScript 中將它們作為 `tool()` 協助程式的第五個引數傳遞，或在 Python 中透過 `@tool` 裝飾器的 `annotations` 關鍵字引數傳遞。所有提示欄位都是布林值。

| 欄位                | 預設值     | 意義                             |
| :---------------- | :------ | :----------------------------- |
| `readOnlyHint`    | `false` | 工具不會修改其環境。控制工具是否可以與其他唯讀工具平行呼叫。 |
| `destructiveHint` | `true`  | 工具可能執行破壞性更新。僅供參考。              |
| `idempotentHint`  | `false` | 使用相同引數重複呼叫沒有額外效果。僅供參考。         |
| `openWorldHint`   | `true`  | 工具到達程序外的系統。僅供參考。               |

註解是中繼資料，不是強制執行。標記為 `readOnlyHint: true` 的工具如果處理程式執行該操作，仍然可以寫入磁碟。保持註解準確反映處理程式。

此範例將 `readOnlyHint` 新增至[天氣工具範例](#weather-tool-example)中的 `get_temperature` 工具。

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

請參閱 [TypeScript](/docs/zh-TW/agent-sdk/typescript#toolannotations) 或 [Python](/docs/zh-TW/agent-sdk/python#toolannotations) 參考中的 `ToolAnnotations`。

<h2 id="control-tool-access">
  控制工具存取
</h2>

[天氣工具範例](#weather-tool-example)註冊了伺服器並在 `allowedTools` 中列出了工具。本節涵蓋工具名稱的構造方式，以及當您有多個工具或想要限制內建工具時如何限制存取。

<h3 id="tool-name-format">
  工具名稱格式
</h3>

當 MCP 工具公開給 Claude 時，它們的名稱遵循特定格式：

* 模式：`mcp__{server_name}__{tool_name}`
* 範例：伺服器 `weather` 中名為 `get_temperature` 的工具變成 `mcp__weather__get_temperature`

<h3 id="configure-allowed-tools">
  設定允許的工具
</h3>

`tools` 選項和允許/不允許清單影響兩個層級：可用性控制工具是否出現在 Claude 的內容中，權限控制 Claude 嘗試呼叫後是否核准呼叫。`tools` 和裸名稱 `disallowedTools` 項目改變可用性。`allowedTools` 和限定範圍的 `disallowedTools` 規則只改變權限。

| 選項                        | 層級  | 效果                                                                                                      |
| :------------------------ | :-- | :------------------------------------------------------------------------------------------------------ |
| `tools: ["Read", "Grep"]` | 可用性 | 只有列出的內建工具在 Claude 的內容中。未列出的內建工具會被移除。MCP 工具不受影響。                                                         |
| `tools: []`               | 可用性 | 所有內建工具都被移除。Claude 只能使用您的 MCP 工具。                                                                        |
| 允許的工具                     | 權限  | 列出的工具執行時不會出現權限提示。未列出的工具保持可用；呼叫會通過[權限流程](/docs/zh-TW/agent-sdk/permissions)。                                  |
| 不允許的工具                    | 兩者  | 裸工具名稱（例如 `"Bash"`）會將工具從 Claude 的內容中移除，與從 `tools` 中省略它相同。限定範圍的規則（例如 `"Bash(rm *)"`）會將工具保留在內容中，並僅拒絕符合的呼叫。 |

若要完全移除內建工具，請從 `tools` 中省略它或在 `disallowedTools` 中列出其裸名稱（Python：`disallowed_tools`）；兩者都會將工具保留在內容之外，以便 Claude 永遠不會嘗試它。限定範圍的 `disallowedTools` 規則會阻止符合的呼叫，但會將工具保留為可見，因此 Claude 可能會浪費一個回合嘗試它。請參閱[設定權限](/docs/zh-TW/agent-sdk/permissions)以取得完整的評估順序。

<h2 id="handle-errors">
  處理錯誤
</h2>

處理程式錯誤不會停止代理迴圈。SDK 的同處理程序 MCP 伺服器會捕獲未捕獲的例外，並將其作為錯誤結果傳回，因此您報告錯誤的方式決定了 Claude 讀取的內容，而不是查詢是否失敗：

| 發生的情況                                                       | 結果                                             |
| :---------------------------------------------------------- | :--------------------------------------------- |
| 處理程式擲出未捕獲的例外                                                | MCP 伺服器將其轉換為錯誤結果，攜帶原始例外訊息。Claude 看到該訊息，代理迴圈繼續。 |
| 處理程式捕獲錯誤並傳回 `isError: true`（TS）/ `"is_error": True`（Python） | Claude 看到您撰寫的訊息。您可以新增原始例外缺乏的內容，例如哪個請求失敗或要嘗試什麼。 |

在兩種情況下，Claude 都可以重試、嘗試不同的工具或解釋失敗。當原始例外訊息不足以讓 Claude 採取行動時，請自行捕獲錯誤。

下面的範例在處理程式內捕獲兩種失敗，並撰寫 Claude 讀取的錯誤訊息。非 200 HTTP 狀態從回應中捕獲並作為錯誤結果傳回。網路錯誤或無效 JSON 由周圍的 `try/except`（Python）或 `try/catch`（TypeScript）捕獲，也作為錯誤結果傳回。在兩種情況下，Claude 都會收到描述失敗的訊息，而不是裸露的例外字串。

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
  傳回影像和資源
</h2>

工具結果中的 `content` 陣列接受 `text`、`image`、`audio`、`resource` 和 `resource_link` 區塊。您可以在同一回應中混合它們。在 TypeScript 中，音訊區塊會儲存到磁碟，Claude 會收到一個包含已儲存檔案路徑的文字區塊；在 Python 中，SDK 會從工具結果中捨棄音訊區塊並記錄警告。資源連結區塊會轉換為包含連結名稱、URI 和描述的文字區塊。

<h3 id="images">
  影像
</h3>

影像區塊以 base64 編碼的方式內聯攜帶影像位元組。沒有 URL 欄位。若要傳回位於 URL 的影像，請在處理程式中擷取它、讀取回應位元組，並在傳回前進行 base64 編碼。結果作為視覺輸入進行處理。

| 欄位         | 類型        | 備註                                                      |
| :--------- | :-------- | :------------------------------------------------------ |
| `type`     | `"image"` |                                                         |
| `data`     | `string`  | Base64 編碼的位元組。僅原始 base64，沒有 `data:image/...;base64,` 前綴 |
| `mimeType` | `string`  | 必需。例如 `image/png`、`image/jpeg`、`image/webp`、`image/gif` |

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
  資源
</h3>

資源區塊嵌入由 URI 識別的內容片段。URI 是 Claude 參考的標籤；實際內容位於區塊的 `text` 或 `blob` 欄位中。當您的工具產生稍後按名稱尋址有意義的內容時使用此項，例如生成的檔案或來自外部系統的記錄。

| 欄位                  | 類型           | 備註                                                               |
| :------------------ | :----------- | :--------------------------------------------------------------- |
| `type`              | `"resource"` |                                                                  |
| `resource.uri`      | `string`     | 內容的識別碼。任何 URI 配置                                                 |
| `resource.text`     | `string`     | 內容，如果是文字。提供此項或 `blob`，不要同時提供兩者                                   |
| `resource.blob`     | `string`     | 內容 base64 編碼，如果是二進位。僅限 TypeScript：Python SDK 會從工具結果中捨棄二進位資源並記錄警告 |
| `resource.mimeType` | `string`     | 選用                                                               |

此範例顯示從工具處理程式內傳回的資源區塊。URI `file:///tmp/report.md` 是 Claude 稍後可以參考的標籤；SDK 不會從該路徑讀取。

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

這些區塊形狀來自 MCP `CallToolResult` 類型。請參閱 [MCP 規格](https://modelcontextprotocol.io/specification/2025-06-18/server/tools#tool-result)以取得完整定義。

<h2 id="return-structured-data">
  傳回結構化資料
</h2>

`structuredContent` 是結果上的選用 JSON 物件，與 `content` 陣列分開。使用它傳回原始值，Claude 可以將其讀取為確切欄位，而不是從文字字串或影像中解析它們。

設定 `structuredContent` 時，Claude 接收 JSON 加上 `content` 中的任何影像或資源區塊。`content` 中的文字區塊不會轉發，因為假設它們複製結構化資料。下面的範例將圖表呈現為影像區塊，並從同一處理程式在 `structuredContent` 中傳回其背後的資料點。

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
  Python `@tool` 裝飾器僅從處理程式的傳回字典轉發 `content` 和 `is_error`。若要從 Python 傳回 `structuredContent`，請改為執行[獨立 MCP 伺服器](/docs/zh-TW/agent-sdk/mcp)。
</Note>

<h2 id="example-unit-converter">
  範例：單位轉換器
</h2>

此工具在長度、溫度和重量的單位之間轉換值。使用者可以詢問「將 100 公里轉換為英里」或「72°F 是多少攝氏度」，Claude 從請求中選擇正確的單位類型和單位。

它演示了兩種模式：

* **列舉結構描述：** `unit_type` 受限於一組固定值。在 TypeScript 中，使用 `z.enum()`。在 Python 中，字典結構描述不支援列舉，因此需要完整的 JSON Schema 字典。
* **不支援的輸入處理：** 當找不到轉換對時，處理程式傳回 `isError: true`，以便 Claude 可以告訴使用者出了什麼問題，而不是將失敗視為正常結果。

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

定義伺服器後，以與天氣範例相同的方式將其傳遞至 `query`。此範例在迴圈中傳送三個不同的提示，以顯示同一工具處理不同的單位類型。對於每個回應，它檢查 `AssistantMessage` 物件（包含 Claude 在該回合期間進行的工具呼叫）並在列印最終 `ResultMessage` 文字之前列印每個 `ToolUseBlock`。這讓您看到 Claude 何時使用工具與從自己的知識回答。

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
  後續步驟
</h2>

自訂工具在標準介面中包裝非同步函數。您可以在同一伺服器中混合本頁上的模式：單一伺服器可以並排保存資料庫工具、API 閘道工具和影像呈現器。

從這裡：

* 如果您的伺服器增長到數十個工具，請參閱[工具搜尋](/docs/zh-TW/agent-sdk/tool-search)以延遲載入它們，直到 Claude 需要它們。
* 若要連接到外部 MCP 伺服器（檔案系統、GitHub、Slack）而不是建立自己的，請參閱[連接 MCP 伺服器](/docs/zh-TW/agent-sdk/mcp)。
* 若要控制哪些工具自動執行與需要核准，請參閱[設定權限](/docs/zh-TW/agent-sdk/permissions)。

<h2 id="related-documentation">
  相關文件
</h2>

* [TypeScript SDK 參考](/docs/zh-TW/agent-sdk/typescript)
* [Python SDK 參考](/docs/zh-TW/agent-sdk/python)
* [MCP 文件](https://modelcontextprotocol.io)
* [SDK 概觀](/docs/zh-TW/agent-sdk/overview)
