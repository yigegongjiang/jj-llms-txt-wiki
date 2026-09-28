> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude にカスタムツールを提供する

> Claude Agent SDK のインプロセス MCP サーバーでカスタムツールを定義し、Claude が関数を呼び出し、API にアクセスし、ドメイン固有の操作を実行できるようにします。

カスタムツールは Agent SDK を拡張し、Claude が会話中に呼び出せる独自の関数を定義できるようにします。SDK のインプロセス MCP サーバーを使用すると、Claude にデータベース、外部 API、ドメイン固有のロジック、またはアプリケーションが必要とするその他の機能へのアクセスを提供できます。

<h2 id="quick-reference">
  クイックリファレンス
</h2>

| 実行したい操作                      | 方法                                                                                                                                                                                  |
| :--------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ツールを定義する                     | Python では [`@tool`](/docs/ja/agent-sdk/python#tool)、TypeScript では [`tool()`](/docs/ja/agent-sdk/typescript#tool) を使用して、名前、説明、スキーマ、ハンドラーを指定します。[カスタムツールを作成する](#create-a-custom-tool)を参照してください。 |
| Claude にツールを登録する             | `create_sdk_mcp_server` / `createSdkMcpServer` でラップし、`query()` の `mcpServers` に渡します。[カスタムツールを呼び出す](#call-a-custom-tool)を参照してください。                                                   |
| ツールを事前承認する                   | 許可されたツールに追加します。[許可されたツールを設定する](#configure-allowed-tools)を参照してください。                                                                                                                  |
| Claude のコンテキストから組み込みツールを削除する | 必要な組み込みのみをリストする `tools` 配列を渡します。[許可されたツールを設定する](#configure-allowed-tools)を参照してください。                                                                                                 |
| Claude がツールを並列で呼び出せるようにする    | 副作用のないツールに `readOnlyHint: true` を設定します。[ツール注釈を追加する](#add-tool-annotations)を参照してください。                                                                                                |
| Claude が読むエラーメッセージを制御する      | `isError: true` を返して、生の例外をサーフェスする代わりにメッセージを作成します。[エラーを処理する](#handle-errors)を参照してください。                                                                                               |
| 画像またはファイルを返す                 | コンテンツ配列で `image` または `resource` ブロックを使用します。[画像とリソースを返す](#return-images-and-resources)を参照してください。                                                                                     |
| マシン可読 JSON 結果を返す             | 結果に `structuredContent` を設定します。[構造化データを返す](#return-structured-data)を参照してください。                                                                                                       |
| 多くのツールにスケーリングする              | [ツール検索](/docs/ja/agent-sdk/tool-search)を使用して、オンデマンドでツールを読み込みます。                                                                                                                          |

<h2 id="create-a-custom-tool">
  カスタムツールを作成する
</h2>

ツールは 4 つの部分で定義され、TypeScript の [`tool()`](/docs/ja/agent-sdk/typescript#tool) ヘルパーまたは Python の [`@tool`](/docs/ja/agent-sdk/python#tool) デコレーターに引数として渡されます。

* **名前：** Claude がツールを呼び出すために使用する一意の識別子。
* **説明：** ツールが何をするか。Claude はこれを読んで、ツールをいつ呼び出すかを決定します。
* **入力スキーマ：** Claude が提供する必要がある引数。TypeScript では常に [Zod スキーマ](https://zod.dev/)であり、ハンドラーの `args` は自動的に型付けされます。Python では、`{"latitude": float}` のような名前から型へのマッピングである dict であり、SDK が JSON Schema に変換します。Python デコレーターは、列挙型、範囲、オプションフィールド、またはネストされたオブジェクトが必要な場合、完全な [JSON Schema](https://json-schema.org/understanding-json-schema/about) dict も直接受け入れます。
* **ハンドラー：** Claude がツールを呼び出すときに実行される非同期関数。検証された引数を受け取り、以下を含むオブジェクトを返す必要があります。
  * `content`（必須）：結果ブロックの配列。各ブロックは `"text"`、`"image"`、`"audio"`、`"resource"`、または `"resource_link"` の `type` を持ちます。テキスト以外のブロックについては、[画像とリソースを返す](#return-images-and-resources)を参照してください。
  * `structuredContent`（オプション）：結果を機械可読データとして保持する JSON オブジェクト。`content` と一緒に返されます。[構造化データを返す](#return-structured-data)を参照してください。
  * `isError`（オプション）：ツール障害を通知するために `true` に設定して、Claude が対応できるようにします。[エラーを処理する](#handle-errors)を参照してください。

ツールを定義した後、[`createSdkMcpServer`](/docs/ja/agent-sdk/typescript#createsdkmcpserver)（TypeScript）または [`create_sdk_mcp_server`](/docs/ja/agent-sdk/python#create_sdk_mcp_server)（Python）でサーバーにラップします。サーバーはアプリケーション内でインプロセスで実行され、別のプロセスとしては実行されません。

<h3 id="weather-tool-example">
  天気ツールの例
</h3>

この例は `get_temperature` ツールを定義し、MCP サーバーにラップします。ツールのセットアップのみを行います。`query` に渡して実行するには、以下の [カスタムツールを呼び出す](#call-a-custom-tool)を参照してください。

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

完全なパラメーター詳細（JSON Schema 入力形式と戻り値の構造を含む）については、[`tool()`](/docs/ja/agent-sdk/typescript#tool) TypeScript リファレンスまたは [`@tool`](/docs/ja/agent-sdk/python#tool) Python リファレンスを参照してください。

<Tip>
  パラメーターをオプションにするには：TypeScript では、Zod フィールドに `.default()` を追加します。Python では、dict スキーマはすべてのキーを必須として扱うため、パラメーターをスキーマから除外し、説明文字列で言及し、ハンドラーで `args.get()` で読み取ります。以下の [`get_precipitation_chance` ツール](#add-more-tools)は両方のパターンを示しています。
</Tip>

<h3 id="call-a-custom-tool">
  カスタムツールを呼び出す
</h3>

`mcpServers` オプション経由で `query` に作成した MCP サーバーを渡します。`mcpServers` のキーは各ツールの完全修飾名 `mcp__{server_name}__{tool_name}` の `{server_name}` セグメントになります。その名前を `allowedTools` にリストして、ツールが権限プロンプトなしで実行されるようにします。

これらのスニペットは、[天気ツールの例](#weather-tool-example)の `weatherServer` を再利用して、特定の場所の天気について Claude に尋ねます。

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

このスニペットを [天気ツールの例](#weather-tool-example)のツールとサーバー定義と 1 つのファイルに組み合わせ、Python の場合は `python weather.py` で、TypeScript の場合は `npx tsx weather.ts` で実行します。Claude は `get_temperature` を呼び出し、スクリプトはサンフランシスコの現在の気温を含む 1 行の回答を出力します。

<h3 id="add-more-tools">
  さらにツールを追加する
</h3>

サーバーは `tools` 配列にリストされているのと同じ数のツールを保持します。サーバーに複数のツールがある場合、`allowedTools` で各ツールを個別にリストするか、ワイルドカード `mcp__weather__*` を使用してサーバーが公開するすべてのツールをカバーできます。

以下の例は 2 番目のツール `get_precipitation_chance` を定義し、[天気ツールの例](#weather-tool-example)の `weatherServer` 定義を、配列内の両方のツールをリストするものに置き換えます。

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

[ツール検索](/docs/ja/agent-sdk/tool-search)はデフォルトで有効になっており、SDK MCP ツールを遅延させます。Claude は各ツールの名前をコンパクトなリストで表示し、必要に応じてその完全なスキーマを読み込みます。ツール検索が無効になっている場合、この配列内のすべてのツールは毎ターン、コンテキストウィンドウスペースを消費します。TypeScript では、[`tool()`](/docs/ja/agent-sdk/typescript#tool) の `extras` 引数または [`createSdkMcpServer()`](/docs/ja/agent-sdk/typescript#createsdkmcpserver) のオプションで `alwaysLoad: true` を渡して、ツールの完全なスキーマを初期プロンプトに保持します。

<h3 id="add-tool-annotations">
  ツール注釈を追加する
</h3>

[ツール注釈](https://modelcontextprotocol.io/docs/concepts/tools#tool-annotations)は、ツールの動作を説明するオプションのメタデータです。TypeScript の `tool()` ヘルパーの 5 番目の引数として、または Python の `@tool` デコレーターの `annotations` キーワード引数経由で渡します。すべてのヒントフィールドはブール値です。

| フィールド             | デフォルト   | 意味                                                |
| :---------------- | :------ | :------------------------------------------------ |
| `readOnlyHint`    | `false` | ツールは環境を変更しません。ツールを他の読み取り専用ツールと並列で呼び出せるかどうかを制御します。 |
| `destructiveHint` | `true`  | ツールは破壊的な更新を実行する可能性があります。情報提供のみ。                   |
| `idempotentHint`  | `false` | 同じ引数での繰り返し呼び出しは追加の効果がありません。情報提供のみ。                |
| `openWorldHint`   | `true`  | ツールはプロセス外のシステムに到達します。情報提供のみ。                      |

注釈はメタデータであり、強制ではありません。`readOnlyHint: true` とマークされたツールでも、ハンドラーがそれを行う場合はディスクに書き込むことができます。注釈をハンドラーに正確に保つようにしてください。

この例は、[天気ツールの例](#weather-tool-example)の `get_temperature` ツールに `readOnlyHint` を追加します。

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
  import { tool } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  tool(
    "get_temperature",
    "Get the current temperature at a location",
    { latitude: z.number(), longitude: z.number() },
    async (args) => ({ content: [{ type: "text", text: `...` }] }),
    { annotations: { readOnlyHint: true } } // Lets Claude batch this with other read-only calls
  );
  ```
</CodeGroup>

[TypeScript](/docs/ja/agent-sdk/typescript#toolannotations) または [Python](/docs/ja/agent-sdk/python#toolannotations) リファレンスで `ToolAnnotations` を参照してください。

<h2 id="control-tool-access">
  ツールアクセスの制御
</h2>

[天気ツールの例](#weather-tool-example)は、サーバーを登録し、`allowedTools` にツールをリストしました。このセクションでは、複数のツールがある場合や組み込みツールを制限したい場合のアクセス範囲の設定方法について説明します。ツール名の構成方法については、[カスタムツールの呼び出し](#call-a-custom-tool)を参照してください。

<h3 id="configure-allowed-tools">
  許可されたツールの設定
</h3>

`tools` オプションと許可/禁止リストは、2 つのレイヤーに影響します。可用性はツールが Claude のコンテキストに表示されるかどうかを制御し、権限は Claude がツールを試みた後に呼び出しが承認されるかどうかを制御します。`tools` と単純名の `disallowedTools` エントリは可用性を変更します。`allowedTools` とスコープ付き `disallowedTools` ルールは権限を変更します。[タスク追跡ツール](/docs/ja/agent-sdk/todo-tracking#model-availability)の 1 つを `allowedTools` に名前を付けた場合、Claude Code もセッションをオプトインします。

| オプション                     | レイヤー | 効果                                                                                                                                                                             |
| :------------------------ | :--- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tools: ["Read", "Grep"]` | 可用性  | リストされた組み込みツールのみが Claude のコンテキストに含まれます。リストされていない組み込みツールは削除されます。MCP ツールは影響を受けません。                                                                                                |
| `tools: []`               | 可用性  | すべての組み込みツールが削除されます。Claude は MCP ツールのみを使用できます。                                                                                                                                  |
| 許可されたツール                  | 権限   | リストされたツールは権限プロンプトなしで実行されます。その他のリストされていないツールは利用可能なままです。呼び出しは[権限フロー](/docs/ja/agent-sdk/permissions)を通じて行われます。                                                                        |
| 禁止されたツール                  | 両方   | `"Bash"` などの単純なツール名は、`tools` から省略するのと同じように、ツールを Claude のコンテキストから削除します。`"Bash(rm *)"` などのスコープ付きルールは、ツールをコンテキストに残し、[記述されたとおり](/docs/ja/permissions#bash-rule-limits)一致する呼び出しのみを拒否します。 |

組み込みツールを完全に削除するには、`tools` から省略するか、`disallowedTools`（Python: `disallowed_tools`）に単純名をリストします。どちらもツールをコンテキストから除外するため、Claude はそれを試みることはありません。スコープ付き `disallowedTools` ルールは一致する呼び出しをブロックしますが、ツールを表示したままにするため、Claude はそれを試みるターンを無駄にする可能性があります。評価順序の詳細については、[権限の設定](/docs/ja/agent-sdk/permissions)を参照してください。

<h2 id="handle-errors">
  エラーを処理する
</h2>

ハンドラーエラーはエージェントループを停止しません。SDK のインプロセス MCP サーバーはキャッチされない例外をキャッチし、エラー結果として返すため、エラーをどのように報告するかが Claude が読む内容を決定します。クエリが失敗するかどうかではありません。

| 何が起こるか                                                              | 結果                                                                            |
| :------------------------------------------------------------------ | :---------------------------------------------------------------------------- |
| ハンドラーがキャッチされない例外をスロー                                                | MCP サーバーはそれをエラー結果に変換し、生の例外メッセージを含めます。Claude はそのメッセージを見て、エージェントループは続行します。      |
| ハンドラーがエラーをキャッチして `isError: true`（TS）/ `"is_error": True`（Python）を返す | Claude はあなたが作成したメッセージを見ます。生の例外が欠いているコンテキスト（どのリクエストが失敗したか、代わりに何を試すかなど）を追加できます。 |

どちらの場合でも Claude は再試行したり、別のツールを試したり、失敗を説明したりできます。生の例外メッセージが Claude が対応するのに十分でない場合は、自分でエラーをキャッチしてください。

以下の例は、ハンドラー内で 2 種類の失敗をキャッチし、Claude が読むエラーメッセージを作成します。200 以外の HTTP ステータスはレスポンスからキャッチされ、エラー結果として返されます。ネットワークエラーまたは無効な JSON は、周囲の `try/except`（Python）または `try/catch`（TypeScript）によってキャッチされ、エラー結果として返されます。どちらの場合でも Claude は、生の例外文字列の代わりに失敗を説明するメッセージを受け取ります。

<CodeGroup>
  ```python Python theme={null}
  import json
  import httpx
  from typing import Any
  from claude_agent_sdk import tool


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
  import { tool } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

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
  画像とリソースを返す
</h2>

ツール結果の `content` 配列は、`text`、`image`、`audio`、`resource`、および `resource_link` ブロックを受け入れます。同じレスポンス内でこれらを混在させることができます。TypeScript では、SDK はオーディオブロックをディスクに保存し、Claude は保存されたファイルパスを含むテキストブロックを受け取ります。Python では、SDK はツール結果からオーディオブロックを削除し、警告をログに記録します。

Claude は各リソースリンクブロックをテキストブロックとして受け取ります。このテキストブロックには、リンクの名前、URI、および説明が含まれます。TypeScript では、アプリケーションはユーザーメッセージの `tool_use_result` 上で [`resourceLinks`](/docs/ja/agent-sdk/typescript#sdkmcpresourcelink) としてリンク自体も受け取ります。Python では、SDK はそれらを CLI が結果を見る前にテキストに平坦化するため、Python の [`resourceLinks` キー](/docs/ja/agent-sdk/python#usermessage) はインプロセスツールに対して生成されることはありません。

<h3 id="images">
  画像
</h3>

画像ブロックは、画像バイトをインラインで、base64 としてエンコードされた形式で保持します。URL フィールドはありません。URL に存在する画像を返すには、ハンドラー内でそれをフェッチし、レスポンスバイトを読み取り、返す前に base64 エンコードしてください。結果は視覚入力として処理されます。

| フィールド      | 型         | 注記                                                                 |
| :--------- | :-------- | :----------------------------------------------------------------- |
| `type`     | `"image"` |                                                                    |
| `data`     | `string`  | Base64 エンコードされたバイト。生の base64 のみ、`data:image/...;base64,` プレフィックスなし |
| `mimeType` | `string`  | 必須。例えば `image/png`、`image/jpeg`、`image/webp`、`image/gif`           |

<CodeGroup>
  ```python Python theme={null}
  import base64
  import httpx
  from claude_agent_sdk import tool


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
  import { tool } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

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
  リソース
</h3>

リソースブロックは、URI で識別されるコンテンツを埋め込みます。URI はコンテンツの参照用ラベルです。実際のコンテンツはブロックの `text` または `blob` フィールドに含まれます。これは、ツールが後で名前で参照することが理にかなったもの（生成されたファイルや外部システムのレコードなど）を生成する場合に使用します。

| フィールド               | 型            | 注記                                                                                     |
| :------------------ | :----------- | :------------------------------------------------------------------------------------- |
| `type`              | `"resource"` |                                                                                        |
| `resource.uri`      | `string`     | コンテンツの識別子。任意の URI スキーム                                                                 |
| `resource.text`     | `string`     | テキストの場合のコンテンツ。これまたは `blob` を提供します。両方ではなく                                               |
| `resource.blob`     | `string`     | バイナリの場合、base64 エンコードされたコンテンツ。TypeScript のみ：Python SDK はバイナリリソースをツール結果から削除し、警告をログに記録します |
| `resource.mimeType` | `string`     | オプション                                                                                  |

この例は、ツールハンドラー内から返されるリソースブロックを示しています。URI `file:///tmp/report.md` は Claude が後で参照できるラベルです。SDK はそのパスから読み取りません。

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

これらのブロック形状は MCP `CallToolResult` 型から来ています。完全な定義については、[MCP 仕様](https://modelcontextprotocol.io/specification/2025-06-18/server/tools#tool-result)を参照してください。

<h2 id="return-structured-data">
  構造化データを返す
</h2>

`structuredContent` は結果のオプションの JSON オブジェクトで、`content` 配列とは別です。テキスト文字列または画像から解析する代わりに、Claude が正確なフィールドとして読み取ることができる生の値を返すために使用します。

`structuredContent` が設定されている場合、Claude は JSON と `content` からのすべての画像またはリソースブロックを受け取ります。`content` のテキストブロックは転送されません。これらは構造化データを複製していると想定されているためです。以下の例は、チャートを画像ブロックとしてレンダリングし、同じハンドラーから `structuredContent` でその背後にあるデータポイントを返します。スニペットでは、`chartPngBuffer` はレンダリングされた PNG バイトを保持する `Buffer` です。

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
  Python の `@tool` デコレーターは、ハンドラーの戻り値の辞書から `content` と `is_error` のみを転送します。Python から `structuredContent` を返すには、in-process SDK サーバーの代わりに[スタンドアロン MCP サーバー](/docs/ja/agent-sdk/mcp)を実行してください。
</Note>

<h2 id="example-unit-converter">
  例：単位変換ツール
</h2>

このツールは、長さ、温度、重さの単位間で値を変換します。ユーザーは「100 キロメートルをマイルに変換して」または「72°F は摂氏温度で何度ですか」と尋ねることができ、Claude はリクエストから正しい単位タイプと単位を選択します。

2 つのパターンを示しています：

* **Enum スキーマ：** `unit_type` は固定値のセットに制限されます。TypeScript では、`z.enum()` を使用します。Python では、dict スキーマは enum をサポートしていないため、完全な JSON Schema dict が必要です。
* **サポートされていない入力の処理：** 変換ペアが見つからない場合、ハンドラーは `isError: true` を返すため、Claude は失敗を通常の結果として扱うのではなく、ユーザーに何が間違ったかを伝えることができます。

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

サーバーが定義されたら、天気の例と同じ方法で `query` に渡します。この例は、同じツールが異なる単位タイプを処理することを示すために、ループで 3 つの異なるプロンプトを送信します。各レスポンスについて、`AssistantMessage` オブジェクト（Claude がそのターン中に行ったツール呼び出しを含む）を検査し、各 `ToolUseBlock` を出力してから最終的な `ResultMessage` テキストを出力します。これにより、Claude がツールを使用している場合と独自の知識から回答している場合を確認できます。

[ツール検索](/docs/ja/agent-sdk/tool-search)はデフォルトで有効になっているため、出力には Claude が遅延ツールスキーマを読み込む際の `ToolSearch` 呼び出しも含まれる場合があります。

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
          try:
              async for message in query(prompt=prompt, options=options):
                  if isinstance(message, AssistantMessage):
                      for block in message.content:
                          if isinstance(block, ToolUseBlock):
                              print(f"[tool call] {block.name}({block.input})")
                  elif isinstance(message, ResultMessage) and message.subtype == "success":
                      print(f"Q: {prompt}\nA: {message.result}\n")
          except Exception as error:
              # A single-shot query() raises after yielding an error result. Only success
              # results are printed above, so handle the failure here and continue with
              # the next prompt.
              print(f"Call failed: {error}")


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
    try {
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
    } catch (error) {
      // A single-shot query() throws after yielding an error result. Only success
      // results are logged above, so handle the failure here and continue with
      // the next prompt.
      console.error(`Call failed: ${error}`);
    }
  }
  ```
</CodeGroup>

<h2 id="next-steps">
  次のステップ
</h2>

このページのパターンを同じサーバー内で組み合わせることができます。単一のサーバーは、データベースツール、API ゲートウェイツール、画像レンダラーを並行して保持できます。

ここから：

* サーバーが数十個のツールに成長する場合は、[ツール検索](/docs/ja/agent-sdk/tool-search)を参照して、Claude がそれらを必要とするまで読み込みを遅延させてください。
* 独自に構築する代わりに外部 MCP サーバー（ファイルシステム、GitHub、Slack）に接続するには、[MCP サーバーを接続](/docs/ja/agent-sdk/mcp)を参照してください。
* どのツールが自動的に実行されるか、または承認が必要かを制御するには、[権限を設定](/docs/ja/agent-sdk/permissions)を参照してください。
