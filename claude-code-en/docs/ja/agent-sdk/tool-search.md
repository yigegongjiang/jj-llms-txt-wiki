> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 多くのツールにスケーリングするツール検索

> 必要なものだけをオンデマンドで検出して読み込むことで、エージェントを数千のツールにスケーリングします。

ツール検索により、エージェントは数百または数千のツールを動的に検出し、オンデマンドで読み込むことで、それらと連携できます。すべてのツール定義をコンテキストウィンドウに事前に読み込む代わりに、エージェントはツールカタログを検索し、必要なツールのみを読み込みます。

このアプローチは、ツールライブラリがスケーリングするにつれて、2 つの課題を解決します。

* **コンテキスト効率：** ツール定義はコンテキストウィンドウの大部分を消費する可能性があります（50 個のツールは 10～20K トークンを使用できます）。実際の作業用のスペースが減少します。
* **ツール選択精度：** 30～50 個以上のツールが一度に読み込まれると、ツール選択精度が低下します。

<h2 id="how-tool-search-works">
  ツール検索の仕組み
</h2>

ツール検索はデフォルトでオンになっており、[ツール検索の設定](#configure-tool-search)に記載されている例外があります。

ツール検索がアクティブな場合、ツール定義はコンテキストウィンドウから保留されます。エージェントは利用可能なツールの概要を受け取り、タスクが既に読み込まれていない機能を必要とする場合、関連するツールを検索します。最も関連性の高い 5 個までのツールがデフォルトでコンテキストに読み込まれ、その後のターンで利用可能なままになります。SDK がエージェントがツールを検出したメッセージをコンパクト化した後、エージェントは次にそれらのツールが必要になったときに再度検索します。

ツール検索は Claude がツールを検索するたびに 1 つの追加ラウンドトリップを追加しますが、大規模なツールセットの場合、これはすべてのターンでより小さいコンテキストによってオフセットされます。定義がコンテキストウィンドウに快適に収まる約 10 個未満のツールの場合、すべてを事前に読み込む方が通常は高速です。

基盤となる API メカニズムの詳細については、[API のツール検索](https://platform.claude.com/docs/ja/agents-and-tools/tool-use/tool-search-tool)を参照してください。

<Note>
  ツール検索は Microsoft Foundry [Azure でホストされているデプロイメント](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options)ではサポートされていません。これらのデプロイメントはサーバー側でツール検索を拒否します。SDK はこの拒否を検出し、代わりにそのデプロイメント用にツール定義を事前に読み込みます。デプロイメント自体から拒否が来るため、[`ENABLE_TOOL_SEARCH`](#configure-tool-search)はこれをオーバーライドできません。
</Note>

<h2 id="configure-tool-search">
  ツール検索を設定する
</h2>

ツール検索はデフォルトでオンです。SDK のサポートされていないモデルリストに含まれるモデルの場合、SDK はツール定義を事前に読み込み、`ENABLE_TOOL_SEARCH` 値はそれをオーバーライドしません。Google Cloud の Agent Platform では、SDK はモデル世代によって決定します。

* **Claude Opus 4.5、Sonnet 4.5、Haiku 4.5、およびそれ以降**: ツール検索はデフォルトでオンです。
* **以前の Agent Platform モデル**: SDK はツール定義を事前に読み込みます。これは、それらのサービングスタックが必要なベータヘッダーを拒否するためです。`ENABLE_TOOL_SEARCH` はこれをオーバーライドできません。

Claude Code v2.1.221 より前は、`ENABLE_TOOL_SEARCH` を設定しない限り、SDK は Google Cloud の Agent Platform 上のすべてのモデルに対してツール検索を無効にしていました。

SDK は、`ANTHROPIC_BASE_URL` が非ファーストパーティホストを指す場合もツール検索を無効にします。ほとんどのプロキシは `tool_reference` ブロックを転送しないためです。`ENABLE_TOOL_SEARCH` 環境変数でそのデフォルトをオーバーライドできます。

| 値        | 動作                                                                                                                                                                                                                                          |
| :------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| （未設定）    | ツール検索はオンです。ツール定義は遅延され、オンデマンドで検出されます。Google Cloud の Agent Platform の Claude 4.5 世代より前のモデル、非ファーストパーティ `ANTHROPIC_BASE_URL`、または Microsoft Foundry デプロイメント（Azure でホストされている場合）では、事前読み込みにフォールバックします。                                              |
| `true`   | ツール検索は常にオンです。ただし、Microsoft Foundry デプロイメント（Azure でホストされている場合）ではサーバー側の拒否により事前読み込みが強制され、Google Cloud の Agent Platform の Claude 4.5 世代より前のモデルでは SDK がツール定義を事前に読み込み続けます。SDK はプロキシ経由でベータヘッダーを送信し、`tool_reference` ブロックをサポートしないプロキシではリクエストが失敗します。 |
| `auto`   | ツール検索が遅延できるツール定義のトークン数をカウントし、合計をモデルのコンテキストウィンドウと比較します。合計がウィンドウの 10% に達すると、ツール検索がアクティブになります。それ以下の場合、SDK はすべてのツール定義をコンテキストに事前読み込みします。                                                                                                         |
| `auto:N` | カスタム割合を使用した `auto` と同じです。`auto:5` はそれらの定義がコンテキストウィンドウの 5% に達するとアクティブになります。値が低いほど、より早くアクティブになります。                                                                                                                                            |
| `false`  | ツール検索はオフです。すべてのツール定義がすべてのターンでコンテキストに読み込まれます。                                                                                                                                                                                                |

[`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/ja/env-vars) を設定するとツール検索がオフになります。`ENABLE_TOOL_SEARCH` を自分で設定してそれをオーバーライドすることはできません。組織は [管理設定](/docs/ja/managed-settings)を通じてツール検索をオンに保つことができます。Claude Code v2.1.227 以降で可能です。[プレリリース機能を無効にする](/docs/ja/llm-gateway-protocol#disable-pre-release-capabilities)は、オーバーライドが適用される場所と変数が削除する内容をカバーしています。

ツール検索は、リモート MCP サーバーから来るか、[カスタム SDK MCP サーバー](/docs/ja/agent-sdk/custom-tools)から来るかに関わらず、すべての登録ツールに適用されます。`auto` を使用する場合、SDK はツール検索が遅延できるすべての定義をカウントして、1 つの結合された閾値に向かってカウントします。[`alwaysLoad`](/docs/ja/mcp#exempt-a-server-from-deferral)とマークされていない各 MCP ツール（任意のサーバーから）、およびオンデマンドで読み込まれる組み込みツール。SDK は常に Bash、Read、Edit などのコア組み込みツールを事前に読み込み、閾値に向かってカウントしません。

`query()` の `env` オプションで値を設定します。TypeScript では、`env` はサブプロセス環境を置き換えるため、継承された変数を保持するために `...process.env` を展開します。Python では、`env` は継承された環境の上にマージされます。この例は、多くのツールを公開するリモート MCP サーバーに接続し、ワイルドカードですべてのツールを事前承認し、`auto:5` を使用して、ツール検索が遅延できる定義がコンテキストウィンドウの 5% に達するとツール検索をアクティブにします。

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

この例を実行するには、`https://tools.example.com/mcp` を独自の MCP サーバーの URL に置き換えてください。成功時に、結果テキストがコンソールに出力されます。

これは単一ショットの `query()` 呼び出しであるため、SDK はエラー結果を生成した後に発生するため、この例はループを try ブロックでラップします。実行が失敗した理由を確認するには、ループ内の結果メッセージの `subtype`（`error_during_execution` など）を確認してください。結果メッセージの詳細については、[結果を処理する](/docs/ja/agent-sdk/agent-loop#handle-the-result)を参照してください。

<h2 id="optimize-tool-discovery">
  ツール検出を最適化する
</h2>

検索メカニズムは、ツール名と説明に対してクエリを照合します。`search_slack_messages` のような名前は、`query_slack` よりも広い範囲のリクエストに対して表示されます。「キーワード、チャネル、または日付範囲で Slack メッセージを検索」などの具体的なキーワードを含む説明は、「Slack をクエリ」などの一般的な説明よりも多くのクエリに一致します。

利用可能なツールカテゴリをリストするシステムプロンプトセクションを追加することもできます。これにより、エージェントは検索対象のツールの種類に関するコンテキストを取得します。TypeScript では `systemPrompt` オプション、Python では `system_prompt` を使用してテキストを渡します。`claude_code` プリセットで `append` を使用すると、プリセットのプロンプトを置き換えるのではなく、テキストを追加します。

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

システムプロンプトオプションの完全なセットについては、[システムプロンプトの変更](/docs/ja/agent-sdk/modifying-system-prompts)を参照してください。

<h2 id="limits">
  制限
</h2>

* **最大ツール数：** カタログ内の 10,000 個のツール
* **検索結果：** デフォルトでは検索ごとに最も関連性の高い 5 つのツールを返します
* **モデルサポート：** Claude Sonnet 4.5、Claude Haiku 4.5、Claude Opus 4.5、およびそれ以降のモデル。現在のリストについては、[API ドキュメントのモデル互換性](https://platform.claude.com/docs/ja/agents-and-tools/tool-use/tool-search-tool#model-compatibility)を参照してください。Google Cloud の Agent Platform でも同じ最小要件が適用されます。

<h2 id="related-documentation">
  関連ドキュメント
</h2>

* [API のツール検索](https://platform.claude.com/docs/ja/agents-and-tools/tool-use/tool-search-tool)：カスタム実装を含むツール検索の完全な API ドキュメント
* [MCP サーバーを接続する](/docs/ja/agent-sdk/mcp)：MCP サーバー経由で外部ツールに接続する
* [カスタムツール](/docs/ja/agent-sdk/custom-tools)：SDK MCP サーバーで独自のツールを構築する
* [TypeScript SDK リファレンス](/docs/ja/agent-sdk/typescript)：完全な API リファレンス
* [Python SDK リファレンス](/docs/ja/agent-sdk/python)：完全な API リファレンス
