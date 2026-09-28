> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 権限の設定

> 権限モード、hooks、および宣言的な許可/拒否ルールを使用して、エージェントがツールをどのように使用するかを制御します。

Claude Agent SDK は、Claude がツールをどのように使用するかを管理するための権限制御を提供します。権限モードとルールを使用して、自動的に許可されるものを定義し、[`canUseTool` コールバック](/docs/ja/agent-sdk/user-input)を使用して、実行時にそれ以外のすべてを処理します。

<h2 id="how-permissions-are-evaluated">
  権限がどのように評価されるか
</h2>

Claude がツールをリクエストすると、SDK は以下の順序で権限をチェックします。

<Steps>
  <Step title="Hooks">
    最初に [hooks](/docs/ja/agent-sdk/hooks) を実行します。Hook は呼び出しを完全に拒否するか、それを通すことができます。`allow` を返す Hook は、以下の deny および ask ルールをスキップしません。これらは Hook の結果に関係なく評価されます。`PreToolUse` Hook の allow は、[重要なパス](/docs/ja/permission-modes#critical-paths) をターゲットとする `rm` または `rmdir` の削除を承認することもできません。
  </Step>

  <Step title="Deny ルール">
    `deny` ルール（`disallowed_tools` および [settings.json](/docs/ja/settings-reference#permission-settings) から）をチェックします。Deny ルールがマッチした場合、`bypassPermissions` モードでもツールはブロックされます。`Bash` のような裸名の deny ルールは、この評価が始まる前に Claude のコンテキストからツールを削除するため、`Bash(rm *)` のようなスコープ付きルールのみがこのステップでチェックされます。
  </Step>

  <Step title="Ask ルール">
    [settings.json](/docs/ja/settings-reference#permission-settings) から `ask` ルールをチェックします。Ask ルールがマッチした場合、呼び出しは確認のために [`canUseTool` コールバック](/docs/ja/agent-sdk/user-input) にフォールスルーします。`bypassPermissions` モードでも同様です。

    ユーザーインタラクションが必要なツールは同じように動作します。`AskUserQuestion` および [`_meta["anthropic/requiresUserInteraction"]`](/docs/ja/mcp#require-approval-for-a-specific-tool) を設定する MCP ツールサーバーは、allow ルールがマッチした場合でも常にコールバックにフォールスルーします。`dontAsk` モードでは、このモードは決してプロンプトを表示しないため、両方のケースが拒否されます。MCP アノテーションには Claude Code v2.1.199 以降が必要です。

    組織が `ask` に設定した [claude.ai コネクタ](/docs/ja/mcp#organization-controls-on-connector-tools) ツールもこのステップでフローを離れます。`bypassPermissions` モードでも allow ルールがマッチした場合でも、すべての呼び出しはコールバックにフォールスルーします。コールバックは理由 `Your organization requires approval for this tool` を受け取ります。`dontAsk` モードでは、このモードは決してプロンプトを表示しないため、呼び出しは拒否されます。
  </Step>

  <Step title="Permission モード">
    アクティブな [permission モード](#permission-modes) を適用します。

    * `bypassPermissions` モードでは、Claude Code はこのステップに到達したすべてのものを承認します。ただし、[重要なパス](/docs/ja/permission-modes#critical-paths) をターゲットとする `rm` および `rmdir` の削除は除きます。これらはフォールスルーします。
    * `acceptEdits` モードでは、Claude Code は [Accept edits モード](#accept-edits-mode-acceptedits) の下にリストされたファイル操作を承認します。
    * `plan` モードでは、Claude Code は allow ルールに関係なく、ファイル編集およびシェル書き込みツールを `canUseTool` コールバックに送信します。これにより、計画中に書き込み操作を自動承認することはできません。
    * その他のモードでは、リクエストはフォールスルーします。
  </Step>

  <Step title="Allow ルール">
    `allow` ルール（`allowed_tools` および settings.json から）をチェックします。ルールがマッチした場合、ツールは承認されます。ツール自体が承認する呼び出しもこのステップで解決されます。ルールは不要です。例えば、作業ディレクトリ内のファイル読み取りまたは [読み取り専用 Bash コマンド](/docs/ja/permissions#read-only-commands)。`rm` および `rmdir` の削除で [重要なパス](/docs/ja/permission-modes#critical-paths) をターゲットとするものは、allow ルールによって決して承認されません。プロンプトを表示するモードではコールバックに到達し、Claude Code v2.1.218 以降の `auto` モードでは [分類器](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode) に移動し、`dontAsk` モードでは拒否されます。
  </Step>

  <Step title="canUseTool コールバック">
    上記のいずれでも解決されない場合、決定のために [`canUseTool` コールバック](/docs/ja/agent-sdk/user-input) を呼び出します。`dontAsk` モードでは、このステップはスキップされ、ツールは拒否されます。

    TypeScript SDK では、[`permissionPrompts: 'none'`](/docs/ja/agent-sdk/typescript#options) を設定した場合、このステップではコールバックは呼び出されません。[`PermissionRequest` hook](/docs/ja/hooks#permissionrequest) はまだ決定する機会があり、そうしない場合、Claude Code は呼び出しを拒否します。このオプションには Claude Code v2.1.259 以降が必要です。
  </Step>
</Steps>

<img src="https://mintcdn.com/claude-code/jYgs7qigNjO1Badj/images/agent-sdk/permissions-flow.svg?fit=max&auto=format&n=jYgs7qigNjO1Badj&q=85&s=c771ad9085b1277d3708027a49c744bc" className="dark:hidden" alt="上記のステップに対応する 6 ステップの権限評価フロー図。ツールリクエストは hooks、deny ルール、ask ルール、permission モード、allow ルール、canUseTool を通過します。Hooks、deny ルール、canUseTool は Blocked にルーティングでき、permission モード bypass、allow ルール、canUseTool は Execute にルーティングでき、ask ルールは canUseTool にルーティングします。" width="1180" height="260" data-path="images/agent-sdk/permissions-flow.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/permissions-flow-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=e53a91e9059cbf51852b7cedb4dd4251" className="hidden dark:block" alt="上記のステップに対応する 6 ステップの権限評価フロー図。ツールリクエストは hooks、deny ルール、ask ルール、permission モード、allow ルール、canUseTool を通過します。Hooks、deny ルール、canUseTool は Blocked にルーティングでき、permission モード bypass、allow ルール、canUseTool は Execute にルーティングでき、ask ルール は canUseTool にルーティングします。" width="1180" height="260" data-path="images/agent-sdk/permissions-flow-dark.svg" />

TypeScript SDK がコールバックが相談される前に呼び出しを自動承認することを期待する設定で `canUseTool` コールバックを渡す場合、SDK はクエリが構築されるときに Node.js プロセス警告を 1 回発行します。警告のコードは `CLAUDE_SDK_CAN_USE_TOOL_SHADOWED` です。2 つの設定がそれをトリガーします。

* `permissionMode: 'bypassPermissions'`。これは permission モードステップに到達するすべての呼び出しを自動承認します。ただし、[どのモードも自動承認しないアクション](/docs/ja/permission-modes#actions-no-mode-auto-approves) は除きます。
* `"Read"` などの各裸の `allowedTools` エントリ。これはコールバックが相談される前にそのツール全体を自動承認します。ただし、[どのモードも自動承認しないアクション](/docs/ja/permission-modes#actions-no-mode-auto-approves) は除きます。

`Bash(ls *)` などの指定子を持つエントリおよび `acceptEdits` モードはそれをトリガーしません。また、設定ファイルから来る allow ルールはチェックに表示されません。

`process.on('warning', ...)` でリッスンし、コードをマッチさせてログまたは抑制します。モードとルールに関係なくすべてのツール呼び出しをゲートするには、代わりに [`PreToolUse` hook](/docs/ja/agent-sdk/hooks) を使用します。

このページは **allow および deny ルール** および **permission モード** に焦点を当てています。その他のステップについては、以下を参照してください。

* **Hooks：** ツールリクエストを許可、拒否、または変更するカスタムコードを実行します。[実行を Hook で制御する](/docs/ja/agent-sdk/hooks) を参照してください。
* **canUseTool コールバック：** 前のステップで呼び出しが解決されない場合、実行時にユーザーの承認をプロンプトします。[承認とユーザー入力を処理する](/docs/ja/agent-sdk/user-input) を参照してください。

<h2 id="allow-and-deny-rules">
  許可ルールと拒否ルール
</h2>

`allowed_tools` と `disallowed_tools`（TypeScript：`allowedTools` / `disallowedTools`）は、上記の評価フロー内の許可ルールと拒否ルールリストにエントリを追加します。`allowed_tools` に[タスク追跡ツール](/docs/ja/agent-sdk/todo-tracking#model-availability)の 1 つを名前で指定すると、Claude Code もセッションをオプトインします。`allowed_tools` にリストされていない他のツールは、Claude でも利用可能であり、承認が必要なそのツールへの呼び出しは権限モードにフォールスルーします。拒否ルールは、ツール名を指定するか、ツール内のパターンをスコープするかによって動作が異なります。

| オプション                             | 効果                                                                                                                                                                        |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `allowed_tools=["Read", "Grep"]`  | `Read` と `Grep` は自動承認されます。ここにリストされていない他のツールは依然として存在し、承認が必要なそれらへの呼び出しは権限モードと `canUseTool` にフォールスルーします。                                                                     |
| `disallowed_tools=["Bash"]`       | `Bash` ツール定義はリクエストから削除されます。Claude はツールを認識せず、実行を試みることはできません。                                                                                                               |
| `disallowed_tools=["Bash(rm *)"]` | `Bash` は利用可能なままです。`rm *`[に記載されているとおり](/docs/ja/permissions#bash-rule-limits)にマッチする呼び出しは、`bypassPermissions` を含むすべての権限モードで拒否されます。`/bin/rm` を含む他の `Bash` 呼び出しは、権限モードにフォールスルーします。 |
| `disallowed_tools=["*"]`          | すべてのツール定義がリクエストから削除されます。拒否ルールではツール名グロブがサポートされています：`"*"` はすべてのツールにマッチし、`"mcp__*"` はすべてのサーバー全体のすべての MCP ツールにマッチします。                                                         |

許可ルールは、リテラル `mcp__<server>__` プレフィックスの後にのみツール名グロブを受け入れます。サーバーセグメントはグロブフリーである必要があり、設定したサーバーを指定します：`mcp__puppeteer__*` は `puppeteer` サーバーからのすべてのツールにマッチし、`mcp__github__get_*` はその `get_` ツールにマッチします。`allowed_tools=["*"]` や `allowed_tools=["mcp__*"]` のようなアンカーなしエントリは、スタートアップ警告で無視され、何も自動承認しません。

`Read` と `Edit` のスコープ付きルールはパスパターンを取ります。`Edit(path)` ルールは、`Write` と `NotebookEdit` を含む、ファイルを書き込むすべての組み込みツールを管理します。`Write(path)` ルールはファイル権限チェックによってマッチすることはありません。

絶対ファイルシステムパスには `//path` を使用します：`Edit(//secrets/**)` の拒否ルールは、ディスク上の `/secrets` の下のどこでも書き込みをブロックします。単一の先頭スラッシュの場合、`Edit(/secrets/**)` はルールのソースでアンカーします。`allowed_tools` または `disallowed_tools` を通じて渡されるルールの場合、それはセッションの作業ディレクトリを意味するため、ルールはディスク上の `/secrets` をブロックしません。[Read と Edit ルール](/docs/ja/permissions#read-and-edit)で 4 つのアンカー形式と、設定ファイルからのルール解決方法を参照してください。

<Warning>
  **自動承認ツールは `canUseTool` に到達しません。** `acceptEdits` または `bypassPermissions` によって、または許可ルールによって、任意の前のステップで承認されたツール呼び出しは、その `canUseTool` コールバックをスキップするため、そこに配置した権限チェックはそのツールに対して静かにバイパスされます。`AskUserQuestion`、MCP ツール（[`_meta["anthropic/requiresUserInteraction"]`](/docs/ja/mcp#require-approval-for-a-specific-tool)でマークされている）、コネクタツール（[組織が `ask` に設定](/docs/ja/mcp#organization-controls-on-connector-tools)）、および[重要なパス](/docs/ja/permission-modes#critical-paths)をターゲットとする `rm` と `rmdir` 削除は、許可ルールがマッチする場合でも、コールバックに到達します。`auto` モードでは、重要なパス削除は[分類器](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode)に移動し、コールバックには移動しません。一方、ここにリストされている他の呼び出しはそれでも到達します。分類器ルーティングには Claude Code v2.1.218 以降が必要です。`dontAsk` モードでは、これらの呼び出しは代わりに拒否され、コールバックを呼び出しません。

  カバレッジはエントリの形式に依存します：`Read` や `mcp__github__get_issue` のような裸の名前は、上記の例外を除いて、そのツールへのすべての呼び出しを自動承認しますが、`Bash(npm test *)` のようなスコープ付きルールはマッチする呼び出しのみを自動承認し、承認が必要な他の `Bash` 呼び出しはコールバックにフォールスルーします。すべてのツール呼び出しで実行する必要があるチェックの場合は、[`PreToolUse` フック](/docs/ja/agent-sdk/hooks)を使用します：フックはすべての他のステップの前に実行され、フック拒否は `bypassPermissions` モードでも適用されます。
</Warning>

ロックダウンされたエージェントの場合、`allowedTools` を `permissionMode: "dontAsk"` と組み合わせます：

```typescript theme={null}
const options = {
  allowedTools: ["Read", "Glob", "Grep"],
  permissionMode: "dontAsk"
};
```

リストされたツールは承認されます。ただし、[モードが自動承認しないアクション](/docs/ja/permission-modes#actions-no-mode-auto-approves)を除きます。プロンプトを表示する他のすべての呼び出しは代わりに拒否されます。`default` モードで承認が不要な呼び出しは、リストするかどうかに関わらず実行されます。例えば、[読み取り専用 Bash コマンド](/docs/ja/permissions#read-only-commands)、`Agent` のような実行前に尋ねないツール、および作業ディレクトリ内のファイル読み取りなどです。ツールを Claude の到達範囲から完全に外すには、その裸の名前を `disallowedTools` に追加します。

<Warning>
  **`allowed_tools` は `bypassPermissions` を制約しません。** `allowed_tools` はリストしたツールを事前承認します。リストされていない他のツールは、許可ルールによってマッチされず、権限モードにフォールスルーします。ここで `bypassPermissions` はそれらを承認します。`allowed_tools=["Read"]` を `permission_mode="bypassPermissions"` と一緒に設定すると、`Bash`、`Write`、`Edit` を含むすべてのツールが承認されます。`bypassPermissions` が必要だが、特定のツールをブロックしたい場合は、`disallowed_tools` を使用します。
</Warning>

`.claude/settings.json` で許可、拒否、および質問ルールを宣言的に設定することもできます。これらのルールは、`project` 設定ソースが有効な場合に読み込まれます。デフォルト `query()` オプションではこれが有効です。`setting_sources`（TypeScript：`settingSources`）を明示的に設定する場合は、それらを適用するために `"project"` を含めます。[権限設定](/docs/ja/settings-reference#permission-settings)でルール構文を参照してください。

<h2 id="permission-modes">
  権限モード
</h2>

権限モードは、Claude がツールをどのように使用するかについてグローバルコントロールを提供します。`query()` を呼び出すときに権限モードを設定するか、ストリーミングセッション中に動的に変更できます。

<h3 id="available-modes">
  利用可能なモード
</h3>

SDK は以下の権限モードをサポートしています。

| モード                 | 説明           | ツール動作                                                                                                                                                                                                                                                                                                                       |
| :------------------ | :----------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | 標準的な権限動作     | モードベースの自動承認なし。承認が必要で許可ルールに一致しないコールは、`canUseTool` コールバックをトリガーします                                                                                                                                                                                                                                                             |
| `dontAsk`           | プロンプトの代わりに拒否 | それ以外の場合はプロンプトが表示されるコールは拒否されます。`allowed_tools` またはルールで承認されたコール、および `default` モードで承認が不要なコールは実行されます。コネクタツール（[組織が `ask` に設定](/docs/ja/mcp#organization-controls-on-connector-tools)）およびユーザーインタラクションが必要なツール、ならびに [重要なパス](/docs/ja/permission-modes#critical-paths) をターゲットとする `rm` および `rmdir` の削除は、事前に承認していても拒否されます。`canUseTool` は呼び出されません |
| `acceptEdits`       | ファイル編集を自動承認  | ファイル編集および [ファイルシステム操作](#accept-edits-mode-acceptedits)（`mkdir`、`rm`、`mv` など）は自動的に承認されます                                                                                                                                                                                                                                     |
| `bypassPermissions` | 権限チェックをバイパス  | [モードが自動承認しないアクション](/docs/ja/permission-modes#actions-no-mode-auto-approves) を除き、ツールは権限プロンプトなしで実行されます。注意して使用してください                                                                                                                                                                                                                |
| `plan`              | 計画モード        | Claude はソースファイルを編集せずに探索と計画を行います。ファイル編集は自動承認されず、`canUseTool` コールバックを通じてプロンプトが表示されます                                                                                                                                                                                                                                          |
| `auto`              | モデル分類承認      | モデル分類器が権限プロンプトを承認または拒否します。利用可能性については [自動モード](/docs/ja/permission-modes#eliminate-prompts-with-auto-mode) を参照してください                                                                                                                                                                                                               |

<Warning>
  **サブエージェント継承：** サブエージェントは、その [`AgentDefinition`](/docs/ja/agent-sdk/typescript#agentdefinition) で `permissionMode` を設定し、親セッションが `default`、`dontAsk`、または `plan` モードにある場合を除き、親セッションの権限モードで実行されます。その場合でも、Claude Code は `"bypassPermissions"` 値を適用しません。サブエージェントは、親セッション自体が `bypassPermissions` モードにある場合にのみ、`bypassPermissions` モードで実行されます。`bypassPermissions` 例外には Claude Code v2.1.267 以降が必要です。

  サブエージェントは、メインエージェントとは異なるシステムプロンプトを持つ可能性があり、動作がより制約されていないため、`bypassPermissions` を継承すると、完全で自律的なシステムアクセスが付与されます。[モードが自動承認しないアクション](/docs/ja/permission-modes#actions-no-mode-auto-approves) は引き続き適用されます。
</Warning>

<h3 id="set-permission-mode">
  権限モードを設定する
</h3>

クエリを開始するときに権限モードを一度設定するか、セッションがアクティブな間に動的に変更できます。

<Tabs>
  <Tab title="クエリ時">
    クエリを作成するときに `permission_mode`（Python）または `permissionMode`（TypeScript）を渡します。このモードは、動的に変更されない限り、セッション全体に適用されます。

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

  <Tab title="ストリーミング中">
    `set_permission_mode()`（Python）または `setPermissionMode()`（TypeScript）を呼び出して、セッション中盤でモードを変更します。新しいモードは、その後のすべてのツールリクエストに対して直ちに有効になります。これにより、制限的に開始して、信頼が構築されるにつれて権限を緩和できます。たとえば、Claude の初期アプローチを確認した後に `acceptEdits` に切り替えることができます。

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
  モードの詳細
</h3>

<h4 id="accept-edits-mode-acceptedits">
  編集受け入れモード（`acceptEdits`）
</h4>

ファイル操作を自動承認して、Claude がプロンプトなしでコードを編集できるようにします。その他のツール（ファイルシステム操作ではない Bash コマンドなど）は通常の権限が必要です。

**自動承認される操作：**

* ファイル編集（Edit、Write ツール）
* ファイルシステムコマンド：`mkdir`、`touch`、`rm`、`rmdir`、`mv`、`cp`、`sed`

どちらも、作業ディレクトリまたは `additionalDirectories` 内のパスにのみ適用されます。`acceptEdits` モードでは、Claude が以下の場合、Claude Code は要求を自動承認しません。

* そのスコープ外のパスで作業する
* 保護されたパスに書き込む
* `rm` または `rmdir` で [重要なパス](/docs/ja/permission-modes#critical-paths) を削除する

**使用時期：** Claude の編集を信頼し、より高速な反復を望む場合。プロトタイピング中や分離されたディレクトリで作業する場合など。

<h4 id="don’t-ask-mode-dontask">
  質問しないモード（`dontAsk`）
</h4>

`canUseTool` を呼び出さずに、権限プロンプトを拒否に変換します。`allowed_tools`、`settings.json` 許可ルール、またはフックで事前承認されたツール、および `default` モードで承認が不要なコール（作業ディレクトリ内のファイル読み取りや `Agent` への呼び出しなど）は通常どおり実行されます。コネクタツール（[組織が `ask` に設定](/docs/ja/mcp#organization-controls-on-connector-tools)）、ユーザーインタラクションが必要なツール、および [重要なパス](/docs/ja/permission-modes#critical-paths) をターゲットとする `rm` および `rmdir` の削除は、許可ルールが一致する場合でも拒否されます。`PreToolUse` フック許可は、重要なパス削除をクリアしません。

**使用時期：** ヘッドレスエージェント用に固定された明示的なツールサーフェスを望み、`canUseTool` が存在しないことへの暗黙的な依存よりもハード拒否を優先する場合。

<h4 id="bypass-permissions-mode-bypasspermissions">
  権限バイパスモード（`bypassPermissions`）
</h4>

以下に示す場合を除き、プロンプトなしでツール使用を自動承認します。フックは引き続き実行され、必要に応じて操作をブロックできます。Linux および macOS では、Claude Code はこのモードで root として、または [認識されたサンドボックス](/docs/ja/permission-modes#skip-all-checks-with-bypasspermissions-mode) 外の `sudo` の下で起動することを拒否し、クエリは最初のターンの前に失敗します。

<Warning>
  極度の注意を持って使用してください。このモードでは Claude はシステムへの完全なアクセスを持ちます。信頼できるすべての操作が可能な制御された環境でのみ使用してください。

  `allowed_tools` はこのモードを制約しません。リストしたツールだけでなく、すべてのツールが承認されます。これらのコントロールは引き続き適用されます。

  * 拒否ルール、明示的な `ask` ルール、およびフックはモードチェック前に評価され、ツールをブロックできます。
  * コネクタツール（[組織が `ask` に設定](/docs/ja/mcp#organization-controls-on-connector-tools)）、ユーザーインタラクションが必要なツール、および [重要なパス](/docs/ja/permission-modes#critical-paths) をターゲットとする `rm` および `rmdir` の削除は、引き続き `canUseTool` コールバックにフォールスルーします。
  * [クロスセッションメッセージングセーフガード](/docs/ja/permission-modes#skip-all-checks-with-bypasspermissions-mode) は引き続き適用されます。
</Warning>

<h4 id="plan-mode-plan">
  計画モード（`plan`）
</h4>

Claude はソースファイルを編集せずにコードベースを探索し、計画を作成します。読み取り専用ツールは `default` 権限モードと同じように実行されます。

ファイル編集は計画モードで自動承認されません。許可ルールが一致する場合でも、代わりに `canUseTool` コールバックを通じてプロンプトが表示されます。Claude Code v2.1.212 以降では、`touch` や `rm` などのファイルを変更するシェルコマンドは同じ方法で `canUseTool` コールバックに到達します。

`allowDangerouslySkipPermissions: true` を `permissionMode: 'plan'` と一緒に設定した場合、ファイル編集とファイルを変更するシェルコマンドは引き続き `canUseTool` コールバックに到達します。このオプションにより、後で `setPermissionMode()` で `bypassPermissions` に切り替えることができます。

Claude は計画を最終化する前に、`AskUserQuestion` を使用して要件を明確にする場合があります。これらのプロンプトの処理については、[承認とユーザー入力の処理](/docs/ja/agent-sdk/user-input#handle-clarifying-questions) を参照してください。

**使用時期：** Claude に変更を実行せずに提案させたい場合。コードレビュー中や、変更が行われる前に承認する必要がある場合など。

<h2 id="related-resources">
  関連リソース
</h2>

権限評価フローの他のステップについては、以下をご覧ください。

* [承認とユーザー入力の処理](/docs/ja/agent-sdk/user-input)：インタラクティブな承認プロンプトと確認質問
* [Hooks ガイド](/docs/ja/agent-sdk/hooks)：エージェントライフサイクルの重要なポイントでカスタムコードを実行
* [権限ルール](/docs/ja/settings-reference#permission-settings)：`settings.json` の宣言的な許可/拒否ルール
