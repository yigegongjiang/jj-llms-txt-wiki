> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# MCP を使用して外部ツールに接続する

> MCP サーバーを設定してエージェントを外部ツールで拡張します。トランスポートタイプ、大規模なツールセット向けのツール検索、認証、エラーハンドリングについて説明します。

[Model Context Protocol（MCP）](https://modelcontextprotocol.io/docs/getting-started/intro)は、AI エージェントを外部ツールおよびデータソースに接続するためのオープンスタンダードです。MCP を使用すると、エージェントはデータベースをクエリし、Slack や GitHub などの API と統合し、カスタムツール実装を記述することなく他のサービスに接続できます。

MCP サーバーはローカルプロセスとして実行したり、HTTP 経由で接続したり、SDK アプリケーション内で直接実行したりできます。

<Note>
  このページは Agent SDK の MCP 設定について説明しています。Claude Code CLI に MCP サーバーを追加してすべてのプロジェクトで読み込むには、[MCP インストールスコープ](/docs/ja/mcp#mcp-installation-scopes)を参照してください。
</Note>

<h2 id="quickstart">
  クイックスタート
</h2>

この例は、[HTTP トランスポート](#http%2Fsse-servers)を使用して [Claude Code ドキュメンテーション](https://code.claude.com/docs)MCP サーバーに接続し、[`allowedTools`](#allow-mcp-tools)とワイルドカードを使用してサーバーからすべてのツールを許可します。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Use the docs MCP server to explain what hooks are in Claude Code",
    options: {
      mcpServers: {
        "claude-code-docs": {
          type: "http",
          url: "https://code.claude.com/docs/mcp"
        }
      },
      allowedTools: ["mcp__claude-code-docs__*"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "claude-code-docs": {
                  "type": "http",
                  "url": "https://code.claude.com/docs/mcp",
              }
          },
          allowed_tools=["mcp__claude-code-docs__*"],
      )

      async for message in query(
          prompt="Use the docs MCP server to explain what hooks are in Claude Code",
          options=options,
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

エージェントはドキュメンテーションサーバーに接続し、hooks に関する情報を検索して、結果を返します。

<h2 id="add-an-mcp-server">
  MCP サーバーを追加する
</h2>

`query()` を呼び出す際にコード内で MCP サーバーを設定するか、[`settingSources`](#from-a-config-file) 経由で読み込まれる `.mcp.json` ファイルで設定できます。

<h3 id="in-code">
  コード内での設定
</h3>

`mcpServers` オプションで MCP サーバーを直接渡します。この例では `/Users/me/projects` 用のローカルファイルシステム MCP サーバーを起動します。そのパスをマシン上のディレクトリに置き換えてください。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List files in my project",
    options: {
      mcpServers: {
        filesystem: {
          command: "npx",
          args: ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me/projects"]
        }
      },
      allowedTools: ["mcp__filesystem__*"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "filesystem": {
                  "command": "npx",
                  "args": [
                      "-y",
                      "@modelcontextprotocol/server-filesystem",
                      "/Users/me/projects",
                  ],
              }
          },
          allowed_tools=["mcp__filesystem__*"],
      )

      async for message in query(prompt="List files in my project", options=options):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="from-a-config-file">
  設定ファイルから読み込む
</h3>

プロジェクトルートに `.mcp.json` ファイルを作成します。このファイルは `project` 設定ソースが有効な場合に読み込まれます。デフォルトの `query()` オプションでは有効になっています。`settingSources` を明示的に設定する場合は、このファイルを読み込むために `"project"` を含めてください。`/Users/me/projects` をマシン上のディレクトリに置き換えてください。

```json theme={null}
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me/projects"]
    }
  }
}
```

<h2 id="connection-timing">
  接続タイミング
</h2>

Claude Code は `options.mcpServers` で渡すサーバーをスタートアップ時に登録し、最初のターンの待機（存在する場合）が解決されると [init メッセージ](#error-handling) を発行します。各 `options.mcpServers` サーバーが最初のターンを遅延させるかどうか、および接続するタイミングは、そのタイプによって異なります。

| サーバータイプ                                           | 最初のターンを遅延させるか                | 最初のターン待機タイムアウト                                          |
| :------------------------------------------------ | :--------------------------- | :------------------------------------------------------ |
| stdio サーバー、またはキャッシュされたツールリストのない HTTP/SSE サーバー     | はい、接続されるまで                   | [`MCP_TIMEOUT`](/docs/ja/env-vars)、デフォルトは 30 秒。接続はその期限で失敗します |
| キャッシュされたツールリストを持つリモートサーバー（Claude Code が以前の接続から保存） | いいえ。キャッシュされたツールは最初のターンから利用可能 | なし。最初のツール呼び出しで接続し、その遅延接続には独自のタイムアウトがあります                |
| インプロセス [SDK サーバー](#sdk-mcp-servers)               | はい、接続してツールをリストするまで           | なし。接続とツールリスティングリクエストはそれぞれ独自のタイムアウトを持ちます                 |

[設定ファイル](#from-a-config-file)（`.mcp.json` など）またはプラグインから読み込まれたサーバーは、通常、init メッセージで `pending` と表示されます。`options.mcpServers` が stdio、HTTP、または SSE サーバーを保持している場合、最初のターンはこれらの保留中のサーバーも待機し、`MCP_TIMEOUT` までです。`options.mcpServers` が空であるか、SDK サーバーのみを保持している場合、最初のターンは代わりに最大 2 秒間待機します。

* **[ツール検索](/docs/ja/agent-sdk/tool-search)（デフォルト）を使用する場合**：待機は [`alwaysLoad: true`](/docs/ja/mcp#exempt-a-server-from-deferral) で設定された保留中のサーバーをカバーし、その他はカバーしません。その他は引き続きバックグラウンドで接続します。[ツール可用性](/docs/ja/mcp#tool-availability) は、Claude がそれらのツールに接続した後、どのようにしてそれらのツールに到達するかについて説明しています。
* **ツール検索なし**：待機はすべての保留中のサーバーをカバーします。[ツール検索を設定](/docs/ja/agent-sdk/tool-search#configure-tool-search) は、ツール検索をオフにする方法をカバーしています。たとえば `disallowedTools` を通じてセッションから `ToolSearch` ツールを除外する場合、セッションはツール検索なしで実行されます。

`permissionPromptToolName` を設定した場合、最初のターンはすべての場合においてそのツールのサーバーも待機し、`MCP_TIMEOUT` までです。

最初のターン待機を自分で設定するには、[`env` オプション](/docs/ja/agent-sdk/configuration#set-environment-variables) に `CLAUDE_CODE_MCP_STARTUP_WAIT_MS` を追加します。たとえば `CLAUDE_CODE_MCP_STARTUP_WAIT_MS: "5000"` です。最初のターンはその後、ツール検索が利用可能かどうかに関わらず、すべての保留中のサーバーをその多くのミリ秒間待機します。この期限は、`options.mcpServers` の stdio、HTTP、および SSE サーバーの `MCP_TIMEOUT` 最初のターン待機も置き換えます。`CLAUDE_CODE_MCP_STARTUP_WAIT_MS` には Claude Code v2.1.274 以降が必要です。

待機が終了したときに保留中のままのサーバーは、バックグラウンドで接続し続けます。変数を `0` に設定して待機をスキップします。`permissionPromptToolName` サーバーは、値に関わらず、独自の `MCP_TIMEOUT` 待機を保持します。

init メッセージが送信される前に、最初のターン待機とは別の、より早い段階でスタートアップ自体をブロックするには：

* [`MCP_CONNECTION_NONBLOCKING`](/docs/ja/env-vars) を `0` に設定して、接続バッチ全体をブロックします。Claude Code はそのデフォルトで 5 秒でその待機をキャップします。[`MCP_CONNECT_TIMEOUT_MS`](/docs/ja/env-vars) 環境変数でキャップをミリ秒単位で調整します。その期限で保留中のサーバーはバックグラウンドで接続し続けます。
* サーバーの設定で `alwaysLoad: true` を設定して、そのツールを最初のターンで完全なスキーマで利用可能にします。[ツール検索遅延から除外](/docs/ja/mcp#exempt-a-server-from-deferral)。Claude Code はそのサーバーのツールをスタートアップで待機し、同じ期限でキャップされます。一方、他のサーバーはバックグラウンドで接続し続けます。キャッシュされたツールリストを持つリモートサーバーは、上記の表に従って、接続せずにそれらを提供します。

サブタイプ `init` の `system` メッセージは、発行された時点での各サーバーのステータスを報告します。これらのステータスを読むには [エラーハンドリング](#error-handling) を参照してください。

<h2 id="allow-mcp-tools">
  MCP ツールを許可する
</h2>

MCP ツールは Claude が使用する前に明示的な権限が必要です。権限がない場合、Claude はツールが利用可能であることは認識しますが、呼び出すことはできません。

<h3 id="tool-naming-convention">
  ツール命名規則
</h3>

MCP ツールは `mcp__<server-name>__<tool-name>` という命名パターンに従います。例えば、`"github"` という名前の GitHub サーバーに `list_issues` ツールがある場合、`mcp__github__list_issues` になります。

<h3 id="auto-approve-with-allowedtools">
  allowedTools で自動承認
</h3>

`allowedTools` を使用して特定の MCP ツールを事前承認し、Claude が権限プロンプトなしで使用できるようにします。

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      mcpServers: {
        // your servers
      },
      allowedTools: [
        "mcp__github__*", // All tools from the github server
        "mcp__db__query", // Only the query tool from db server
        "mcp__slack__send_message" // Only send_message from slack server
      ]
    }
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      mcp_servers={
          # your servers
      },
      allowed_tools=[
          "mcp__github__*",  # All tools from the github server
          "mcp__db__query",  # Only the query tool from db server
          "mcp__slack__send_message",  # Only send_message from slack server
      ],
  )
  ```
</CodeGroup>

ワイルドカード（`*`）を使用すると、各ツールを個別にリストアップすることなく、サーバーのすべてのツールを許可できます。

<Note>
  **MCP アクセスには権限モードより `allowedTools` を優先してください。** `permissionMode: "acceptEdits"` は MCP ツールを自動承認しません（ファイル編集とファイルシステム Bash コマンドのみ）。`permissionMode: "bypassPermissions"` は MCP ツールを自動承認しますが、他のほとんどのセーフティプロンプトも無効にするため、必要以上に広範です。残存するプロンプトについては [権限の評価方法](/docs/ja/agent-sdk/permissions#how-permissions-are-evaluated) を参照してください。`allowedTools` のワイルドカードは、必要な MCP サーバーのみに権限を付与し、それ以上のものは付与しません。完全な比較については [権限モード](/docs/ja/agent-sdk/permissions#permission-modes) を参照してください。
</Note>

<h3 id="discover-available-tools">
  利用可能なツールを検出
</h3>

MCP サーバーが提供するツールを確認するには、サーバーのドキュメントを確認するか、`system` init メッセージの `tools` 配列を検査します。MCP ツール名は `mcp__` で始まります。

Claude Code は `options.mcpServers` で渡されたサーバーの [最初のターン接続待機](#connection-timing) 後に init メッセージを出力するため、`tools` 配列には、その時点で接続されている各サーバーの `mcp__` ツール、および [キャッシュされたツールリスト](#connection-timing) を持つサーバーのツール（初回使用時に接続）がリストされます。接続されていない他のサーバーのツールは存在しません。各サーバーのステータスを読み取る方法については [エラーハンドリング](#error-handling) を参照してください。

このフィルターは MCP ツール名を出力します。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const options = {
    mcpServers: {
      // your servers
    },
  };

  for await (const message of query({ prompt: "...", options })) {
    if (message.type === "system" && message.subtype === "init") {
      const mcpTools = message.tools.filter((name) => name.startsWith("mcp__"));
      console.log("Available MCP tools:", mcpTools);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              # your servers
          },
      )
      async for message in query(prompt="...", options=options):
          if isinstance(message, SystemMessage) and message.subtype == "init":
              mcp_tools = [t for t in message.data.get("tools", []) if t.startswith("mcp__")]
              print("Available MCP tools:", mcp_tools)


  asyncio.run(main())
  ```
</CodeGroup>

Claude にサーバーから利用可能なツールをリストアップするよう依頼することもできます。

<h2 id="transport-types">
  トランスポートタイプ
</h2>

MCP サーバーはさまざまなトランスポートプロトコルを使用してエージェントと通信します。サーバーのドキュメントを確認して、どのトランスポートがサポートされているかを確認してください。

* ドキュメントに**実行するコマンド**（`npx @modelcontextprotocol/server-filesystem` など）が記載されている場合は、stdio を使用します
* ドキュメントに**URL** が記載されている場合は、HTTP または SSE を使用します
* コード内で独自のツールを構築している場合は、SDK MCP サーバーを使用します

<h3 id="stdio-servers">
  stdio サーバー
</h3>

stdin/stdout を介して通信するローカルプロセスです。同じマシン上で実行する MCP サーバーに使用します。`.mcp.json` 形式の場合は、[設定ファイルから](#from-a-config-file)に示されているのと同じフィールドを使用します。コード内では、コマンドとその引数を渡します。`/Users/me/projects` をマシン上のディレクトリに置き換えます。

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      mcpServers: {
        filesystem: {
          command: "npx",
          args: ["-y", "@modelcontextprotocol/server-filesystem", "/Users/me/projects"]
        }
      },
      allowedTools: ["mcp__filesystem__read_file", "mcp__filesystem__list_directory"]
    }
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      mcp_servers={
          "filesystem": {
              "command": "npx",
              "args": [
                  "-y",
                  "@modelcontextprotocol/server-filesystem",
                  "/Users/me/projects",
              ],
          }
      },
      allowed_tools=["mcp__filesystem__read_file", "mcp__filesystem__list_directory"],
  )
  ```
</CodeGroup>

<h3 id="http/sse-servers">
  HTTP/SSE サーバー
</h3>

クラウドホストされた MCP サーバーとリモート API には HTTP または SSE を使用します。`.mcp.json` 形式の場合は、[リモートサーバーの HTTP ヘッダー](#http-headers-for-remote-servers)の例と同じフィールドを使用し、SSE サーバーの場合は `"type": "sse"` を使用します。コード内では、サーバーの URL を渡します。

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      mcpServers: {
        "remote-api": {
          type: "sse",
          url: "https://api.example.com/mcp/sse",
          headers: {
            Authorization: `Bearer ${process.env.API_TOKEN}`
          }
        }
      },
      allowedTools: ["mcp__remote-api__*"]
    }
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      mcp_servers={
          "remote-api": {
              "type": "sse",
              "url": "https://api.example.com/mcp/sse",
              "headers": {"Authorization": f"Bearer {os.environ['API_TOKEN']}"},
          }
      },
      allowed_tools=["mcp__remote-api__*"],
  )
  ```
</CodeGroup>

ストリーム可能な HTTP トランスポートの場合は、代わりに `"type": "http"` を使用します。`.mcp.json` およびその他の JSON 設定ファイルでは、`"streamable-http"` は `"http"` のエイリアスとして受け入れられます。SDK の `McpHttpServerConfig` タイプは `"http"` のみを宣言しているため、コード内で渡すサーバーには `"http"` を使用します。

<h3 id="sdk-mcp-servers">
  SDK MCP サーバー
</h3>

別のサーバープロセスを実行する代わりに、アプリケーションコード内でカスタムツールを直接定義します。実装の詳細については、[カスタムツールガイド](/docs/ja/agent-sdk/custom-tools)を参照してください。

[`initialize` 制御リクエスト](/docs/ja/agent-sdk/typescript#sdkcontrolinitializeresponse)によって登録された SDK MCP サーバーは、Claude Code がリクエストを処理するとすぐに接続を開始します。

<h2 id="mcp-tool-search">
  MCP ツール検索
</h2>

多くの MCP ツールを設定している場合、ツール定義はコンテキストウィンドウの大部分を消費する可能性があります。ツール検索はコンテキストからツール定義を保留し、各ターンで Claude が必要とするツールのみを読み込むことでこの問題を解決します。

ツール検索はデフォルトで有効になっています。設定オプション、ベストプラクティス、およびカスタム SDK ツールでのツール検索の使用については、[ツール検索](/docs/ja/agent-sdk/tool-search)を参照してください。

<h2 id="authentication">
  認証
</h2>

ほとんどの MCP サーバーは、外部サービスにアクセスするために認証が必要です。サーバー設定で環境変数を通じて認証情報を渡します。

<h3 id="pass-credentials-via-environment-variables">
  環境変数を通じて認証情報を渡す
</h3>

`env` フィールドを使用して、API キー、トークン、およびその他の認証情報を MCP サーバーに渡します。

<Tabs>
  <Tab title="In code">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            "api-server": {
              command: "npx",
              args: ["-y", "@your-org/api-mcp-server"],
              env: {
                API_KEY: process.env.API_KEY
              }
            }
          },
          allowedTools: ["mcp__api-server__*"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "api-server": {
                  "command": "npx",
                  "args": ["-y", "@your-org/api-mcp-server"],
                  "env": {"API_KEY": os.environ["API_KEY"]},
              }
          },
          allowed_tools=["mcp__api-server__*"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "api-server": {
          "command": "npx",
          "args": ["-y", "@your-org/api-mcp-server"],
          "env": {
            "API_KEY": "${API_KEY}"
          }
        }
      }
    }
    ```

    `${API_KEY}` 構文は、実行時に環境変数を展開します。
  </Tab>
</Tabs>

<h3 id="http-headers-for-remote-servers">
  リモートサーバーの HTTP ヘッダー
</h3>

HTTP および SSE サーバーの場合、サーバー設定で認証ヘッダーを直接渡します。

<Tabs>
  <Tab title="In code">
    <CodeGroup>
      ```typescript TypeScript hidelines={1,-1} theme={null}
      const _ = {
        options: {
          mcpServers: {
            "secure-api": {
              type: "http",
              url: "https://api.example.com/mcp",
              headers: {
                Authorization: `Bearer ${process.env.API_TOKEN}`
              }
            }
          },
          allowedTools: ["mcp__secure-api__*"]
        }
      };
      ```

      ```python Python theme={null}
      options = ClaudeAgentOptions(
          mcp_servers={
              "secure-api": {
                  "type": "http",
                  "url": "https://api.example.com/mcp",
                  "headers": {"Authorization": f"Bearer {os.environ['API_TOKEN']}"},
              }
          },
          allowed_tools=["mcp__secure-api__*"],
      )
      ```
    </CodeGroup>
  </Tab>

  <Tab title=".mcp.json">
    ```json theme={null}
    {
      "mcpServers": {
        "secure-api": {
          "type": "http",
          "url": "https://api.example.com/mcp",
          "headers": {
            "Authorization": "Bearer ${API_TOKEN}"
          }
        }
      }
    }
    ```

    `${API_TOKEN}` 構文は、実行時に環境変数を展開します。
  </Tab>
</Tabs>

リモートサーバーの完全な動作例（ヘッダーで認証されたもの）については、[リポジトリから問題を一覧表示する](#list-issues-from-a-repository)を参照してください。

<h3 id="oauth2-authentication">
  OAuth2 認証
</h3>

[MCP 仕様は OAuth 2.1 をサポートしています](https://modelcontextprotocol.io/specification/2025-03-26/basic/authorization)。SDK はブラウザを開いたり、対話的な OAuth フローを実行したりしません。設定されたサーバーが認可チャレンジを返し、保存されたトークンが利用できない場合、エージェント実行はそのサーバーのツールなしで続行され、サーバーはステータス `needs-auth` を報告します。[システム初期化メッセージ](/docs/ja/agent-sdk/typescript#sdksystemmessage)の `mcp_servers` 配列は、発行されたときにそのサーバーに対して `pending` を表示する場合があります。サーバーが認証情報を必要とするかどうかを確認するには、TypeScript SDK で `mcpServerStatus()` をポーリングするか、Python で [`get_mcp_status()`](/docs/ja/agent-sdk/python#methods) をポーリングします。

認証情報を提供するには、アプリケーション内で OAuth フローを完了し、結果のアクセストークンをサーバーの `headers` に渡します。

<CodeGroup>
  ```typescript TypeScript theme={null}
  // After completing OAuth flow in your app.
  // Implement getAccessTokenFromOAuthFlow for your OAuth provider.
  const accessToken = await getAccessTokenFromOAuthFlow();

  const options = {
    mcpServers: {
      "oauth-api": {
        type: "http",
        url: "https://api.example.com/mcp",
        headers: {
          Authorization: `Bearer ${accessToken}`
        }
      }
    },
    allowedTools: ["mcp__oauth-api__*"]
  };
  ```

  ```python Python theme={null}
  # After completing OAuth flow in your app.
  # Implement get_access_token_from_oauth_flow for your OAuth provider.
  access_token = await get_access_token_from_oauth_flow()

  options = ClaudeAgentOptions(
      mcp_servers={
          "oauth-api": {
              "type": "http",
              "url": "https://api.example.com/mcp",
              "headers": {"Authorization": f"Bearer {access_token}"},
          }
      },
      allowed_tools=["mcp__oauth-api__*"],
  )
  ```
</CodeGroup>

<h2 id="examples">
  例
</h2>

<h3 id="list-issues-from-a-repository">
  リポジトリから問題を一覧表示する
</h3>

この例は、リモート [GitHub MCP サーバー](https://github.com/github/github-mcp-server) に接続して、最近の問題を一覧表示します。この例には、MCP 接続とツール呼び出しを検証するためのデバッグログが含まれています。

実行する前に、クエリしたいリポジトリへの読み取りアクセス権を持つ [GitHub 個人用アクセストークン](https://github.com/settings/personal-access-tokens) を作成し、環境変数として設定してください。

```bash theme={null}
export GITHUB_TOKEN=YOUR_GITHUB_PAT
```

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "List the 3 most recent issues in anthropics/claude-code",
    options: {
      mcpServers: {
        github: {
          type: "http",
          url: "https://api.githubcopilot.com/mcp/",
          headers: {
            Authorization: `Bearer ${process.env.GITHUB_TOKEN}`
          }
        }
      },
      allowedTools: ["mcp__github__list_issues"]
    }
  })) {
    // Verify MCP server connected successfully
    if (message.type === "system" && message.subtype === "init") {
      console.log("MCP servers:", message.mcp_servers);
    }

    // Log when Claude calls an MCP tool
    if (message.type === "assistant") {
      for (const block of message.message.content) {
        if (block.type === "tool_use" && block.name.startsWith("mcp__")) {
          console.log("MCP tool called:", block.name);
        }
      }
    }

    // Print the final result
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  import os
  from claude_agent_sdk import (
      query,
      ClaudeAgentOptions,
      ResultMessage,
      SystemMessage,
      AssistantMessage,
  )


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "github": {
                  "type": "http",
                  "url": "https://api.githubcopilot.com/mcp/",
                  "headers": {"Authorization": f"Bearer {os.environ['GITHUB_TOKEN']}"},
              }
          },
          allowed_tools=["mcp__github__list_issues"],
      )

      async for message in query(
          prompt="List the 3 most recent issues in anthropics/claude-code",
          options=options,
      ):
          # Verify MCP server connected successfully
          if isinstance(message, SystemMessage) and message.subtype == "init":
              print("MCP servers:", message.data.get("mcp_servers"))

          # Log when Claude calls an MCP tool
          if isinstance(message, AssistantMessage):
              for block in message.content:
                  if hasattr(block, "name") and block.name.startswith("mcp__"):
                      print("MCP tool called:", block.name)

          # Print the final result
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

`MCP servers:` の行で、`github` の `status` が `connected` であることは、トークンが機能していることを確認します。Claude Code がサーバーの [キャッシュされたツールリスト](#connection-timing) を持っている場合、ステータスは代わりに `pending` と表示され、サーバーは最初のツール呼び出しで接続します。ステータスが `failed` または `needs-auth` の場合は、サーバーが利用できないときに Claude がビルトインツールにフォールバックする可能性があるため、結果を信頼する前に [エラーハンドリング](#error-handling) を参照してください。

<h3 id="query-a-database">
  データベースをクエリする
</h3>

この例は [DBHub](https://github.com/bytebase/dbhub) を使用して Postgres データベースをクエリします。エージェントはデータベーススキーマを自動的に検出し、SQL クエリを作成して、結果を返します。

DBHub の `execute_sql` ツールは、エージェントが発行する SQL（書き込みを含む）を実行します。ただし、[DBHub 設定ファイル](https://dbhub.ai/config/toml) で `readonly = true` を設定すると、DBHub は `INSERT`、`UPDATE`、`DELETE`、および DDL ステートメントを拒否するため、エージェントが書き込みを発行した場合でも、この例はデータを変更できません。DBHub は設定を読み込むときにプロセス環境から `${DATABASE_URL}` を解決するため、接続文字列はファイルの外に留まります。スクリプトの隣に次の `dbhub.toml` を作成してください。

```toml dbhub.toml theme={null}
[[sources]]
id = "production"
dsn = "${DATABASE_URL}"

[[tools]]
name = "execute_sql"
source = "production"
readonly = true
```

スクリプトは、接続文字列を直接渡す代わりに、DBHub が設定ファイルを指すようにします。実行する前に、`DATABASE_URL` 環境変数を接続文字列に設定してください。プレースホルダー値を独自のデータベース詳細に置き換えてください。

```bash theme={null}
export DATABASE_URL=postgresql://user:password@localhost:5432/mydb
```

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    // Natural language query - Claude writes the SQL
    prompt: "How many users signed up last week? Break it down by day.",
    options: {
      mcpServers: {
        postgres: {
          command: "npx",
          // dbhub.toml sets readonly = true, so execute_sql rejects writes
          args: ["-y", "@bytebase/dbhub", "--config", "dbhub.toml"]
        }
      },
      allowedTools: ["mcp__postgres__execute_sql"]
    }
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={
              "postgres": {
                  "command": "npx",
                  # dbhub.toml sets readonly = true, so execute_sql rejects writes
                  "args": [
                      "-y",
                      "@bytebase/dbhub",
                      "--config",
                      "dbhub.toml",
                  ],
              }
          },
          allowed_tools=["mcp__postgres__execute_sql"],
      )

      # Natural language query - Claude writes the SQL
      async for message in query(
          prompt="How many users signed up last week? Break it down by day.",
          options=options,
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="error-handling">
  エラーハンドリング
</h2>

MCP サーバーは様々な理由で接続に失敗する可能性があります。サーバープロセスがインストールされていない、認証情報が無効である、またはリモートサーバーに到達できない場合があります。

Claude Code は各クエリの開始時に、サブタイプ `init` の `system` メッセージを発行します。このメッセージには各 MCP サーバーの接続ステータスが含まれます。`status` フィールドは `"pending"`、`"connected"`、`"failed"`、`"needs-auth"`、または `"disabled"` のいずれかです。Claude Code は `options.mcpServers` で渡されたサーバーの [最初のターン接続待機](#connection-timing) の後に init メッセージを発行するため、待機時間内に接続したサーバーは `"connected"` を表示します。

init メッセージでは、`"pending"` をそれ自体で失敗として扱わないでください。これは以下のいずれかを意味する可能性があります。

* サーバーはまだ接続されていません。[Claude Code がそれを待つ時間](#connection-timing) を参照してください
* サーバーのツールリストは [キャッシュから提供](#connection-timing) され、最初の使用時に接続が確立されました
* 接続期限が切れました。このようなサーバーはタイミングに応じて `"pending"` または `"failed"` を報告します

使用できないサーバーを検出するには、`"failed"` または `"needs-auth"` をチェックしてください。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  try {
    for await (const message of query({
      prompt: "Process data",
      options: {
        mcpServers: {
          // Replace dataServer with your server configuration
          "data-processor": dataServer
        }
      }
    })) {
      if (message.type === "system" && message.subtype === "init") {
        const unavailableServers = message.mcp_servers.filter(
          (s) => s.status === "failed" || s.status === "needs-auth"
        );

        if (unavailableServers.length > 0) {
          console.warn("Unavailable MCP servers:", unavailableServers);
        }
      }

      if (message.type === "result" && message.subtype === "error_during_execution") {
        console.error("Execution failed");
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, the error subtype branch above has
    // already run; a failure to start or reach the Claude Code process
    // yields no result message. MCP servers that fail to connect don't
    // throw: use the status check above, and note that servers still
    // "pending" at init need a later status check.
    console.log(`Session ended with an error: ${error}`);
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, SystemMessage, ResultMessage


  async def main():
      # Replace data_server with your server configuration
      options = ClaudeAgentOptions(mcp_servers={"data-processor": data_server})

      try:
          async for message in query(prompt="Process data", options=options):
              if isinstance(message, SystemMessage) and message.subtype == "init":
                  unavailable_servers = [
                      s
                      for s in message.data.get("mcp_servers", [])
                      if s.get("status") in ("failed", "needs-auth")
                  ]

                  if unavailable_servers:
                      print(f"Unavailable MCP servers: {unavailable_servers}")

              if (
                  isinstance(message, ResultMessage)
                  and message.subtype == "error_during_execution"
              ):
                  print("Execution failed")
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, the error subtype branch above has
          # already run; a failure to start or reach the Claude Code process
          # yields no result message. MCP servers that fail to connect don't
          # raise: use the status check above, and note that servers still
          # "pending" at init need a later status check.
          print(f"Session ended with an error: {error}")


  asyncio.run(main())
  ```
</CodeGroup>

リモートサーバーのステータスは、`"connected"` を報告した後でも変わる可能性があります。セッション中に接続が切れると、Claude Code はサーバーを `"pending"` に戻して [再接続](/docs/ja/mcp#automatic-reconnection) します。その後、TypeScript で `mcpServerStatus()` を呼び出すか、Python で [`ClaudeSDKClient.get_mcp_status()`](/docs/ja/agent-sdk/python#methods) を呼び出すと、あなた側で設定変更がなくても、以前に接続されていたサーバーに対して `"pending"` を報告できます。

5 回の再接続試行が失敗した後、サーバーは `"failed"` を報告するか、再度認可が必要な場合は `"needs-auth"` を報告します。手動で再試行するには、TypeScript で [`reconnectMcpServer()`](/docs/ja/agent-sdk/typescript#methods) を呼び出すか、Python で [`ClaudeSDKClient.reconnect_mcp_server()`](/docs/ja/agent-sdk/python#methods) を呼び出してください。

<h2 id="troubleshooting">
  トラブルシューティング
</h2>

<h3 id="server-shows-failed-status">
  サーバーが「failed」ステータスを表示する
</h3>

`init` メッセージをチェックして、どのサーバーが接続に失敗したかを確認します。

<CodeGroup>
  ```typescript TypeScript theme={null}
  if (message.type === "system" && message.subtype === "init") {
    for (const server of message.mcp_servers) {
      if (server.status === "failed") {
        console.error(`Server ${server.name} failed to connect`);
      }
    }
  }
  ```

  ```python Python theme={null}
  if isinstance(message, SystemMessage) and message.subtype == "init":
      for server in message.data.get("mcp_servers", []):
          if server.get("status") == "failed":
              print(f"Server {server['name']} failed to connect")
  ```
</CodeGroup>

`"pending"` ステータスはサーバーが失敗したことを意味しません。初期化時にカバーされるケースについては、[エラーハンドリング](#error-handling)を参照してください。セッションの後半で更新されたステータスを取得するには、TypeScript SDK でクエリの `mcpServerStatus()` メソッドを呼び出すか、Python で [`ClaudeSDKClient.get_mcp_status()`](/docs/ja/agent-sdk/python#methods) を呼び出します。

一般的な原因：

* **環境変数の欠落**：必要なトークンと認証情報が設定されていることを確認します。stdio サーバーの場合、`env` フィールドがサーバーが期待する内容と一致していることを確認します。
* **サーバーがインストールされていない**：`npx` コマンドの場合、パッケージが存在し、Node.js が PATH に含まれていることを確認します。
* **無効な接続文字列**：データベースサーバーの場合、接続文字列の形式を確認し、データベースがアクセス可能であることを確認します。
* **ネットワークの問題**：リモート HTTP/SSE サーバーの場合、URL に到達可能であり、ファイアウォールが接続を許可していることを確認します。

<h3 id="tools-not-being-called">
  ツールが呼び出されていない
</h3>

Claude がツールを認識しているが使用していない場合、`allowedTools` で権限を付与していることを確認します。

<CodeGroup>
  ```typescript TypeScript hidelines={1,-1} theme={null}
  const _ = {
    options: {
      mcpServers: {
        // your servers
      },
      allowedTools: ["mcp__servername__*"] // Auto-approve calls from this server
    }
  };
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      mcp_servers={
          # your servers
      },
      allowed_tools=["mcp__servername__*"],  # Auto-approve calls from this server
  )
  ```
</CodeGroup>

<h3 id="connection-timeouts">
  接続タイムアウト
</h3>

MCP サーバー接続はデフォルトで 30 秒後にタイムアウトします。実行中のツール呼び出しにかかる時間を変更するには、[`MCP_TOOL_TIMEOUT`](/docs/ja/env-vars) を設定します。サーバーの起動に時間がかかる場合、接続は失敗します。[`MCP_TIMEOUT`](/docs/ja/env-vars) 環境変数（ミリ秒単位）で接続制限を引き上げます。より多くの起動時間が必要なサーバーの場合、以下も検討してください。

* 利用可能な場合は、より軽量なサーバーを使用する
* エージェントを開始する前にサーバーをプリウォーミングする
* 遅い初期化の原因についてサーバーログを確認する

TypeScript では、[`timeout` を `createSdkMcpServer()` に渡す](/docs/ja/agent-sdk/typescript#createsdkmcpserver)ことで、単一の [SDK MCP サーバー](#sdk-mcp-servers) のツール呼び出し制限を設定できます。

<h3 id="tool-output-exceeds-maximum-allowed-tokens">
  ツール出力が最大許容トークン数を超える
</h3>

SDK は Claude Code と同じ MCP 出力制限を適用します。画像コンテンツのないツール結果が 25,000 トークンより大きい場合、Claude Code は出力をファイルに保存し、ツール結果をファイルパスを名前とするエラーメッセージに置き換えるため、エージェントは出力を部分的に読み戻すことができます。

[`MAX_MCP_OUTPUT_TOKENS`](/docs/ja/env-vars) 環境変数で制限を引き上げます。完全な動作については [MCP 出力制限と警告](/docs/ja/mcp#mcp-output-limits-and-warnings) を参照してください。これには、サーバーが `anthropic/maxResultSizeChars` アノテーションでツールごとの高い制限を宣言する方法も含まれます。

<h2 id="related-resources">
  関連リソース
</h2>

* **[カスタムツールガイド](/docs/ja/agent-sdk/custom-tools)**: SDK アプリケーションでインプロセスで実行される独自の MCP サーバーを構築します
* **[権限](/docs/ja/agent-sdk/permissions)**: `allowedTools` と `disallowedTools` を使用してエージェントが使用できる MCP ツールを制御します
* **[TypeScript SDK リファレンス](/docs/ja/agent-sdk/typescript)**: MCP 設定オプションを含む完全な API リファレンス
* **[Python SDK リファレンス](/docs/ja/agent-sdk/python)**: MCP 設定オプションを含む完全な API リファレンス
* **[MCP サーバーディレクトリ](https://github.com/modelcontextprotocol/servers)**: データベース、API など、利用可能な MCP サーバーを参照します
