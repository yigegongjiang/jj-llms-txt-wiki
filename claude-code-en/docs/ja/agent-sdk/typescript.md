> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK リファレンス - TypeScript

> TypeScript Agent SDK の完全な API リファレンス。すべての関数、型、インターフェースを含みます。

<script src="/docs/components/typescript-sdk-type-links.js" defer />

<h2 id="installation">
  インストール
</h2>

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

<Note>
  SDK は、`@anthropic-ai/claude-agent-sdk-darwin-arm64` などのオプション依存関係として、プラットフォーム用のネイティブ Claude Code バイナリをバンドルしています。ほとんどのインストールでは Claude Code を別途インストールする必要はありません。SDK バージョンはバンドルされた Claude Code バージョンを追跡します。SDK v0.3.191 は Claude Code v2.1.191 をバンドルしているため、このページの Claude Code バージョンが必要な機能には、同じパッチ番号以降の SDK リリースが必要です。パッケージマネージャーがオプション依存関係をスキップする場合、SDK は `Native CLI binary for <platform>-<arch> not found` をスローします。代わりに、別途インストールされた `claude` バイナリに [`pathToClaudeCodeExecutable`](#options) を設定してください。

  パッケージマネージャーが npm の `libc` フィールドを適用しない場合（Yarn 1.x のように）、Linux では glibc と musl プラットフォームパッケージの両方が取得され、インストールサイズがおよそ 2 倍になります。Agent SDK v0.2.141 以降では、SDK は正しいバリアントを起動します。コンテナイメージ内のスペースを回復するには、アプリが実行される libc と一致しないプラットフォームパッケージを削除します。glibc ランタイムで x64 の場合、`rm -rf node_modules/@anthropic-ai/claude-agent-sdk-linux-x64-musl` です。開発マシンでは、Yarn が次の依存関係の変更時にパッケージを再インストールするため、削除は一時的です。
</Note>

<h3 id="compile-to-a-single-executable">
  単一の実行可能ファイルにコンパイルする
</h3>

`bun build --compile` を使用してアプリケーションを単一ファイルの実行可能ファイルにコンパイルする場合、SDK は実行時にバンドルされた CLI バイナリを解決できません。`require.resolve` はコンパイルされた実行可能ファイルの `$bunfs` 仮想ファイルシステム内では機能しないため、SDK は `Native CLI binary for <platform>-<arch> not found` をスローします。

この問題を回避するには、プラットフォームバイナリをファイルアセットとして埋め込み、起動時に `extractFromBunfs()` を使用して実際のパスに抽出し、そのパスを [`pathToClaudeCodeExecutable`](#options) に渡します。

`extractFromBunfs()` ヘルパーには `@anthropic-ai/claude-agent-sdk` v0.3.144 以降が必要です。以下の例は macOS on Apple Silicon 向けにビルドします。

```typescript theme={null}
import binPath from "@anthropic-ai/claude-agent-sdk-darwin-arm64/claude" with { type: "file" };
import { extractFromBunfs } from "@anthropic-ai/claude-agent-sdk/extract";
import { query } from "@anthropic-ai/claude-agent-sdk";

const cliPath = extractFromBunfs(binPath);

for await (const message of query({
  prompt: "Hello",
  options: { pathToClaudeCodeExecutable: cliPath },
})) {
  console.log(message);
}
```

`extractFromBunfs()` は、コンパイルされた実行可能ファイルの仮想ファイルシステムから埋め込まれたバイナリをユーザーごとの一時ディレクトリにコピーし、実際のパスを返します。コンパイルされた実行可能ファイルの外では、入力パスを変更せずに返すため、同じコードは開発環境で変更なしに実行されます。

各コンパイルされた実行可能ファイルは、単一プラットフォームのバイナリを埋め込みます。インポート内のプラットフォームパッケージを `--target` と一致させます。

* クロスコンパイルするには、一致しないプラットフォームパッケージをインストールします。例えば `npm install @anthropic-ai/claude-agent-sdk-linux-x64 --force`。
* Windows では、バイナリサブパスは `claude.exe` です。例えば `@anthropic-ai/claude-agent-sdk-win32-x64/claude.exe`。

<h2 id="functions">
  関数
</h2>

<h3 id="query">
  `query()`
</h3>

Claude Code と相互作用するための主要な関数です。メッセージが到着するにつれてストリーミングする非同期ジェネレータを作成します。

```typescript theme={null}
function query({
  prompt,
  options
}: {
  prompt: string | AsyncIterable<SDKUserMessage>;
  options?: Options;
}): Query;
```

<h4 id="parameters">
  パラメータ
</h4>

| パラメータ     | 型                                                                | 説明                                       |
| :-------- | :--------------------------------------------------------------- | :--------------------------------------- |
| `prompt`  | `string \| AsyncIterable<`[`SDKUserMessage`](#sdkusermessage)`>` | 入力プロンプト（文字列またはストリーミングモード用の非同期反復可能オブジェクト） |
| `options` | [`Options`](#options)                                            | オプションの設定オブジェクト（以下の Options 型を参照）         |

<h4 id="returns">
  戻り値
</h4>

[`Query`](#query-object) オブジェクトを返します。このオブジェクトは `AsyncGenerator<`[`SDKMessage`](#sdkmessage)`, void>` を拡張し、追加のメソッドを備えています。

<h3 id="startup">
  `startup()`
</h3>

CLI サブプロセスをプリウォームします。プロセスをスポーンし、プロンプトが利用可能になる前に初期化ハンドシェイクを完了します。返された [`WarmQuery`](#warmquery) ハンドルは後でプロンプトを受け入れ、既に準備ができているプロセスに書き込むため、最初の `query()` 呼び出しはサブプロセスのスポーンと初期化のコストを支払わずに解決します。

```typescript theme={null}
function startup(params?: {
  options?: Options;
  initializeTimeoutMs?: number;
}): Promise<WarmQuery>;
```

<h4 id="parameters-2">
  パラメータ
</h4>

| パラメータ                 | 型                     | 説明                                                                              |
| :-------------------- | :-------------------- | :------------------------------------------------------------------------------ |
| `options`             | [`Options`](#options) | オプションの設定オブジェクト。`query()` の `options` パラメータと同じです                                 |
| `initializeTimeoutMs` | `number`              | サブプロセス初期化の待機時間の最大値（ミリ秒）。デフォルトは `60000` です。初期化が時間内に完了しない場合、プロミスはタイムアウトエラーで拒否されます |

<h4 id="returns-2">
  戻り値
</h4>

サブプロセスがスポーンされ、初期化ハンドシェイクを完了したら解決する `Promise<`[`WarmQuery`](#warmquery)`>` を返します。

<h4 id="example">
  例
</h4>

`startup()` を早期に呼び出します。例えば、アプリケーション起動時に呼び出し、プロンプトが準備できたら返されたハンドルで `.query()` を呼び出します。これにより、サブプロセスのスポーンと初期化をクリティカルパスから外します。

```typescript theme={null}
import { startup } from "@anthropic-ai/claude-agent-sdk";

// スタートアップコストを事前に支払う
const warm = await startup({ options: { maxTurns: 3 } });

// 後で、プロンプトが準備できたら、これは即座です
for await (const message of warm.query("What files are here?")) {
  console.log(message);
}
```

<h3 id="tool">
  `tool()`
</h3>

SDK MCP サーバーで使用するためのタイプセーフな MCP ツール定義を作成します。

```typescript theme={null}
function tool<Schema extends AnyZodRawShape>(
  name: string,
  description: string,
  inputSchema: Schema,
  handler: (args: InferShape<Schema>, extra: unknown) => Promise<CallToolResult>,
  extras?: { annotations?: ToolAnnotations; searchHint?: string; alwaysLoad?: boolean }
): SdkMcpToolDefinition<Schema>;
```

<h4 id="parameters-3">
  パラメータ
</h4>

| パラメータ         | 型                                                                                                      | 説明                                                                                                                                                                                               |
| :------------ | :----------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | `string`                                                                                               | ツールの名前                                                                                                                                                                                           |
| `description` | `string`                                                                                               | ツールが何をするかの説明                                                                                                                                                                                     |
| `inputSchema` | `Schema extends AnyZodRawShape`                                                                        | ツールの入力パラメータを定義する Zod スキーマ（Zod 3 と Zod 4 の両方をサポート）                                                                                                                                                |
| `handler`     | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>`                                      | ツールロジックを実行する非同期関数                                                                                                                                                                                |
| `extras`      | `{ annotations?: `[`ToolAnnotations`](#toolannotations)`; searchHint?: string; alwaysLoad?: boolean }` | オプションの追加情報。`annotations` は MCP の動作ヒントをクライアントに提供します。`searchHint` は [ツール検索](/docs/ja/agent-sdk/tool-search) がアクティブな場合に遅延ツールリストに表示される 1 行の機能フレーズです。`alwaysLoad: true` はこのツールの完全なスキーマを初期プロンプトに保持し、遅延させません |

<h4 id="toolannotations">
  `ToolAnnotations`
</h4>

`@modelcontextprotocol/sdk/types.js` から再エクスポートされます。すべてのフィールドはオプションのヒントです。クライアントはセキュリティの決定にこれらに依存すべきではありません。

| フィールド             | 型         | デフォルト       | 説明                                                                                    |
| :---------------- | :-------- | :---------- | :------------------------------------------------------------------------------------ |
| `title`           | `string`  | `undefined` | ツールの人間が読める形のタイトル                                                                      |
| `readOnlyHint`    | `boolean` | `false`     | `true` の場合、ツールはその環境を変更しません                                                            |
| `destructiveHint` | `boolean` | `true`      | `true` の場合、ツールは破壊的な更新を実行する可能性があります（`readOnlyHint` が `false` の場合のみ意味があります）             |
| `idempotentHint`  | `boolean` | `false`     | `true` の場合、同じ引数での繰り返し呼び出しは追加の効果がありません（`readOnlyHint` が `false` の場合のみ意味があります）          |
| `openWorldHint`   | `boolean` | `true`      | `true` の場合、ツールは外部エンティティと相互作用します（例えば、Web 検索）。`false` の場合、ツールのドメインは閉じられています（例えば、メモリツール） |

```typescript theme={null}
import { tool } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

const searchTool = tool(
  "search",
  "Search the web",
  { query: z.string() },
  async ({ query }) => {
    return { content: [{ type: "text", text: `Results for: ${query}` }] };
  },
  { annotations: { readOnlyHint: true, openWorldHint: true } }
);
```

<h3 id="createsdkmcpserver">
  `createSdkMcpServer()`
</h3>

アプリケーションと同じプロセスで実行される MCP サーバーインスタンスを作成します。

```typescript theme={null}
function createSdkMcpServer(options: {
  name: string;
  version?: string;
  instructions?: string;
  tools?: Array<SdkMcpToolDefinition<any>>;
  alwaysLoad?: boolean;
  timeout?: number;
}): McpSdkServerConfigWithInstance;
```

<h4 id="parameters-4">
  パラメータ
</h4>

| パラメータ                  | 型                             | 説明                                                                                                                                                                               |
| :--------------------- | :---------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.name`         | `string`                      | MCP サーバーの名前                                                                                                                                                                      |
| `options.version`      | `string`                      | オプションのバージョン文字列                                                                                                                                                                   |
| `options.instructions` | `string`                      | オプションのサーバー指示。`initialize` から返され、MCP 指示ブロックとしてモデルに表示されます                                                                                                                          |
| `options.tools`        | `Array<SdkMcpToolDefinition>` | [`tool()`](#tool) で作成されたツール定義の配列                                                                                                                                                 |
| `options.alwaysLoad`   | `boolean`                     | `true` の場合、このサーバーのすべてのツールは初期プロンプトに留まり、[ツール検索](/docs/ja/agent-sdk/tool-search) の背後で遅延されることはありません。[`tool()`](#tool) のツール単位の `alwaysLoad` と組み合わせます                                       |
| `options.timeout`      | `number`                      | このサーバーのツール呼び出しのタイムアウト（ミリ秒）。Claude Code はこれを [`MCP_TOOL_TIMEOUT`](/docs/ja/env-vars) の代わりにこのサーバーに適用します。1000 以上の整数を渡してください。Claude Code は他の値を無視します。TypeScript Agent SDK v0.3.248 以降が必要です |

<h3 id="listsessions">
  `listSessions()`
</h3>

軽いメタデータを含む過去のセッションを検出してリストします。プロジェクトディレクトリでフィルタリングするか、すべてのプロジェクト全体でセッションをリストします。

```typescript theme={null}
function listSessions(options?: ListSessionsOptions): Promise<SDKSessionInfo[]>;
```

<h4 id="parameters-5">
  パラメータ
</h4>

| パラメータ                      | 型         | デフォルト       | 説明                                                   |
| :------------------------- | :-------- | :---------- | :--------------------------------------------------- |
| `options.dir`              | `string`  | `undefined` | セッションをリストするディレクトリ。省略した場合、すべてのプロジェクト全体のセッションを返します     |
| `options.limit`            | `number`  | `undefined` | 返すセッションの最大数                                          |
| `options.includeWorktrees` | `boolean` | `true`      | `dir` が git リポジトリ内にある場合、すべての worktree パスからセッションを含めます |

<h4 id="return-type-sdksessioninfo">
  戻り値の型：`SDKSessionInfo`
</h4>

| プロパティ          | 型                     | 説明                                                  |
| :------------- | :-------------------- | :-------------------------------------------------- |
| `sessionId`    | `string`              | 一意のセッション識別子（UUID）                                   |
| `summary`      | `string`              | 表示タイトル：カスタムタイトル、自動生成されたサマリー、または最初のプロンプト             |
| `lastModified` | `number`              | 最後に変更された時刻（エポック以降のミリ秒）                              |
| `fileSize`     | `number \| undefined` | セッションファイルサイズ（バイト）。ローカル JSONL ストレージの場合のみ入力されます       |
| `customTitle`  | `string \| undefined` | ユーザーが設定したセッションタイトル（`/rename` 経由）                    |
| `firstPrompt`  | `string \| undefined` | セッション内の最初の意味のあるユーザープロンプト                            |
| `gitBranch`    | `string \| undefined` | セッション終了時の git ブランチ                                  |
| `cwd`          | `string \| undefined` | セッションの作業ディレクトリ                                      |
| `tag`          | `string \| undefined` | ユーザーが設定したセッションタグ（[`tagSession()`](#tagsession) を参照） |
| `createdAt`    | `number \| undefined` | 作成時刻（エポック以降のミリ秒）。最初のエントリのタイムスタンプから                  |

<h4 id="example-2">
  例
</h4>

プロジェクトの 10 個の最新セッションを出力します。結果は `lastModified` で降順にソートされるため、最初の項目が最新です。`dir` を省略すると、すべてのプロジェクト全体を検索します。

```typescript theme={null}
import { listSessions } from "@anthropic-ai/claude-agent-sdk";

const sessions = await listSessions({ dir: "/path/to/project", limit: 10 });

for (const session of sessions) {
  console.log(`${session.summary} (${session.sessionId})`);
}
```

<h3 id="getsessionmessages">
  `getSessionMessages()`
</h3>

過去のセッショントランスクリプトからユーザーとアシスタントのメッセージを読み取ります。

```typescript theme={null}
function getSessionMessages(
  sessionId: string,
  options?: GetSessionMessagesOptions
): Promise<SessionMessage[]>;
```

<h4 id="parameters-6">
  パラメータ
</h4>

| パラメータ            | 型        | デフォルト       | 説明                                             |
| :--------------- | :------- | :---------- | :--------------------------------------------- |
| `sessionId`      | `string` | 必須          | 読み取るセッション UUID（`listSessions()` を参照）           |
| `options.dir`    | `string` | `undefined` | セッションを検索するプロジェクトディレクトリ。省略した場合、すべてのプロジェクトを検索します |
| `options.limit`  | `number` | `undefined` | 返すメッセージの最大数                                    |
| `options.offset` | `number` | `undefined` | 開始からスキップするメッセージ数                               |

<h4 id="return-type-sessionmessage">
  戻り値の型：`SessionMessage`
</h4>

| プロパティ                | 型                       | 説明                                                                                                                                                                                                        |
| :------------------- | :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`               | `"user" \| "assistant"` | メッセージロール                                                                                                                                                                                                  |
| `uuid`               | `string`                | 一意のメッセージ識別子                                                                                                                                                                                               |
| `session_id`         | `string`                | このメッセージが属するセッション                                                                                                                                                                                          |
| `message`            | `unknown`               | トランスクリプトからの生のメッセージペイロード                                                                                                                                                                                   |
| `parent_tool_use_id` | `string \| null`        | サブエージェントメッセージの場合、サブエージェントを開始した `Agent` または `Skill` ツール呼び出しの `tool_use_id`。メインセッションメッセージと古いセッションの場合は `null`                                                                                                |
| `parent_agent_id`    | `string \| null`        | [ネストされたサブエージェント](/docs/ja/sub-agents#let-subagents-spawn-their-own-subagents) からのメッセージの場合、それをスポーンしたサブエージェントの `agentId`。メインセッションメッセージ、トップレベルサブエージェントからのメッセージ、および古いセッションの場合は `null`。Claude Code v2.1.202 以降が必要です |

<h4 id="example-3">
  例
</h4>

```typescript theme={null}
import { listSessions, getSessionMessages } from "@anthropic-ai/claude-agent-sdk";

const [latest] = await listSessions({ dir: "/path/to/project", limit: 1 });

if (latest) {
  const messages = await getSessionMessages(latest.sessionId, {
    dir: "/path/to/project",
    limit: 20
  });

  for (const msg of messages) {
    console.log(`[${msg.type}] ${msg.uuid}`);
  }
}
```

<h3 id="getsessioninfo">
  `getSessionInfo()`
</h3>

フルプロジェクトディレクトリスキャンなしで ID でシングルセッションのメタデータを読み取ります。

```typescript theme={null}
function getSessionInfo(
  sessionId: string,
  options?: GetSessionInfoOptions
): Promise<SDKSessionInfo | undefined>;
```

<h4 id="parameters-7">
  パラメータ
</h4>

| パラメータ         | 型        | デフォルト       | 説明                                           |
| :------------ | :------- | :---------- | :------------------------------------------- |
| `sessionId`   | `string` | 必須          | ルックアップするセッションの UUID                          |
| `options.dir` | `string` | `undefined` | プロジェクトディレクトリパス。省略した場合、すべてのプロジェクトディレクトリを検索します |

[`SDKSessionInfo`](#return-type-sdksessioninfo) を返すか、セッションが見つからない場合は `undefined` を返します。

<h3 id="renamesession">
  `renameSession()`
</h3>

カスタムタイトルエントリを追加してセッションの名前を変更します。繰り返し呼び出しは安全です。最新のタイトルが優先されます。

```typescript theme={null}
function renameSession(
  sessionId: string,
  title: string,
  options?: SessionMutationOptions
): Promise<void>;
```

<h4 id="parameters-8">
  パラメータ
</h4>

| パラメータ         | 型        | デフォルト       | 説明                                           |
| :------------ | :------- | :---------- | :------------------------------------------- |
| `sessionId`   | `string` | 必須          | 名前を変更するセッションの UUID                           |
| `title`       | `string` | 必須          | 新しいタイトル。空白をトリミングした後、空でない必要があります              |
| `options.dir` | `string` | `undefined` | プロジェクトディレクトリパス。省略した場合、すべてのプロジェクトディレクトリを検索します |

<h3 id="tagsession">
  `tagSession()`
</h3>

セッションにタグを付けます。`null` を渡してタグをクリアします。繰り返し呼び出しは安全です。最新のタグが優先されます。

```typescript theme={null}
function tagSession(
  sessionId: string,
  tag: string | null,
  options?: SessionMutationOptions
): Promise<void>;
```

<h4 id="parameters-9">
  パラメータ
</h4>

| パラメータ         | 型                | デフォルト       | 説明                                           |
| :------------ | :--------------- | :---------- | :------------------------------------------- |
| `sessionId`   | `string`         | 必須          | タグを付けるセッションの UUID                            |
| `tag`         | `string \| null` | 必須          | タグ文字列、またはクリアする場合は `null`                     |
| `options.dir` | `string`         | `undefined` | プロジェクトディレクトリパス。省略した場合、すべてのプロジェクトディレクトリを検索します |

<h3 id="resolvesettings">
  `resolveSettings()`
</h3>

CLI をスポーンせずに、CLI と同じマージエンジンを使用して、指定されたディレクトリの有効な Claude Code 設定を解決します。`query()` 呼び出しを呼び出す前に、その呼び出しが何を見るかを検査するために使用します。

<Note>
  この関数はアルファ版であり、安定化前に API が変更される可能性があります。
</Note>

スナップショットはライブ `query()` セッションが適用するものと異なります：

* **`policyHelper`**：`resolveSettings()` は macOS plist と Windows HKLM/HKCU を含む MDM ソースを読み取りますが、管理者が設定した `policyHelper` サブプロセスを実行しません。
* **サーバー管理設定**：`resolveSettings()` は [サーバー管理設定](/docs/ja/server-managed-settings#fetch-and-caching-behavior) をフェッチしません。それらを含めるには `options.serverManagedSettings` に渡してください。
* **`defaultMode`**：スナップショットは `permissions.defaultMode` をすべてのティアから現状のまま返すため、プロジェクトおよびローカル設定からの `'auto'` および `'bypassPermissions'` 値を含めることができます。これは [ライブセッションが無視する](/docs/ja/permission-modes#which-mode-a-session-starts-in) ものです。

```typescript theme={null}
function resolveSettings(
  options?: ResolveSettingsOptions
): Promise<ResolvedSettings>;
```

<h4 id="parameters-10">
  パラメータ
</h4>

`resolveSettings()` は単一のオプションオブジェクトを受け入れます。すべてのフィールドはオプションです。

| パラメータ                           | 型                                     | デフォルト           | 説明                                                                                                                                                                                                                             |
| :------------------------------ | :------------------------------------ | :-------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.cwd`                   | `string`                              | `process.cwd()` | プロジェクトおよびローカル設定を相対的に解決するディレクトリ                                                                                                                                                                                                 |
| `options.settingSources`        | [`SettingSource`](#settingsource)`[]` | すべてのソース         | どのファイルシステムソースをロードするか。`[]` を渡してユーザー、プロジェクト、およびローカル設定をスキップします。[エンドポイント管理ポリシー](/docs/ja/managed-settings#delivery-mechanisms) はすべての場合にロードされます。`resolveSettings()` は `options.serverManagedSettings` を渡す場合のみサーバー管理設定を含めます               |
| `options.managedSettings`       | `Settings`                            | `undefined`     | 埋め込みホストによって提供されるポリシーティア設定。[`Options`](#options) の [`managedSettings`](/docs/ja/settings-reference) と同じルールに従います。ただし、`resolveSettings()` は設定された [`policyHelper`](/docs/ja/settings-reference) を実行しないため、スナップショットはライブセッションが削除する設定を含めることができます |
| `options.serverManagedSettings` | `Settings`                            | `undefined`     | `/api/claude_code/settings` からのサーバー管理設定ペイロード。制限のないキーはフィルタリングなしで通過します                                                                                                                                                           |

<h4 id="return-type-resolvedsettings">
  戻り値の型：`ResolvedSettings`
</h4>

`resolveSettings()` はマージされた設定と各キーに貢献したソースを説明するオブジェクトを返します。

| プロパティ        | 型                                                   | 説明                                       |
| :----------- | :-------------------------------------------------- | :--------------------------------------- |
| `effective`  | `Settings`                                          | すべての有効なソースを優先順位順に適用した後のマージされた設定          |
| `provenance` | `Partial<Record<keyof Settings, ProvenanceEntry>>`  | `effective` の各トップレベルキーについて、値を提供したソース     |
| `sources`    | `Array<{ source, settings, path?, policyOrigin? }>` | ソースごとの生の設定。最も低い優先順位から最も高い優先順位の順に並べられています |

<h4 id="example-4">
  例
</h4>

以下の例は、プロジェクトディレクトリの設定を解決し、クリーンアップ期間を制御するソースを出力します。設定ファイルが `cleanupPeriodDays` を設定していないマシンでは、両方の出力行は値に対して `undefined` を表示します。これはエラーではなく、予想される出力です。

```typescript theme={null}
import { resolveSettings } from "@anthropic-ai/claude-agent-sdk";

const { effective, provenance } = await resolveSettings({
  cwd: "/path/to/project",
  settingSources: ["user", "project", "local"],
});

console.log(`Cleanup period: ${effective.cleanupPeriodDays} days`);
console.log(`Set by: ${provenance.cleanupPeriodDays?.source}`);
```

<h2 id="types">
  型
</h2>

<h3 id="options">
  `Options`
</h3>

`query()` 関数の設定オブジェクトです。

| プロパティ                             | 型                                                                                                                                                                                                              | デフォルト                                  | 説明                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `abortController`                 | `AbortController`                                                                                                                                                                                              | `new AbortController()`                | 操作をキャンセルするためのコントローラー                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `additionalDirectories`           | `string[]`                                                                                                                                                                                                     | `[]`                                   | Claude がアクセスできる追加ディレクトリ。SDK は各エントリを Claude Code に `--add-dir` として渡すため、`project` 設定ソースを使用すると Claude Code は[ディレクトリのスキル、コマンド、サブエージェントも読み込みます](/docs/ja/permissions#additional-directories-grant-file-access-not-configuration)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `agent`                           | `string`                                                                                                                                                                                                       | `undefined`                            | メインスレッドのエージェント名。エージェントは `agents` オプションまたは設定で定義する必要があります                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `agents`                          | `Record<string, [`AgentDefinition`](#agentdefinition)>`                                                                                                                                                        | `undefined`                            | プログラムでサブエージェントを定義します                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `agentProgressSummaries`          | `boolean`                                                                                                                                                                                                      | `false`                                | `true` の場合、サブエージェントの 1 行の進捗サマリーを生成し、`summary` フィールド経由で [`task_progress`](#sdktaskprogressmessage) イベントで転送します。フォアグラウンドおよびバックグラウンドサブエージェントに適用されます                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `allowDangerouslySkipPermissions` | `boolean`                                                                                                                                                                                                      | `false`                                | 権限をバイパスできるようにします。`permissionMode: 'bypassPermissions'` を使用する場合に必須です。スタートアップ時または後で `setPermissionMode()` を通じて設定できます。[プランモード](/docs/ja/agent-sdk/permissions#plan-mode-plan)を参照して、`permissionMode: 'plan'` との相互作用を確認してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `allowedTools`                    | `string[]`                                                                                                                                                                                                     | `[]`                                   | プロンプトなしで自動承認するツール。これは Claude をこれらのツールのみに制限しません。[タスク追跡ツール](/docs/ja/agent-sdk/todo-tracking#model-availability)の 1 つをここで指定すると、Claude Code もセッションをオプトインします。リストされていない他のツールは `permissionMode` と `canUseTool` にフォールスルーします。`disallowedTools` を使用してツールをブロックします。[権限](/docs/ja/agent-sdk/permissions#allow-and-deny-rules)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `betas`                           | [`SdkBeta`](#sdkbeta)`[]`                                                                                                                                                                                      | `[]`                                   | ベータ機能を有効にします                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `canUseTool`                      | [`CanUseTool`](#canusetool)                                                                                                                                                                                    | `undefined`                            | カスタム権限関数。[権限フロー](/docs/ja/agent-sdk/permissions#how-permissions-are-evaluated)がプロンプトにフォールスルーする場合にのみ呼び出されます。`allowedTools`、許可ルール、または `permissionMode` で自動承認された呼び出しには呼び出されません。許可ルールは[どのモードも自動承認しないアクション](/docs/ja/permission-modes#actions-no-mode-auto-approves)を事前承認しません。詳細は [`CanUseTool`](#canusetool) を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `continue`                        | `boolean`                                                                                                                                                                                                      | `false`                                | 最新の会話を続行します                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `cwd`                             | `string`                                                                                                                                                                                                       | `process.cwd()`                        | 現在の作業ディレクトリ                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `debug`                           | `boolean`                                                                                                                                                                                                      | `false`                                | Claude Code プロセスのデバッグモードを有効にします                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `debugFile`                       | `string`                                                                                                                                                                                                       | `undefined`                            | デバッグログを特定のファイルパスに書き込みます。暗黙的にデバッグモードを有効にします                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `disallowedTools`                 | `string[]`                                                                                                                                                                                                     | `[]`                                   | 拒否するツール。`"Bash"` のような単純な名前はツールを Claude のコンテキストから削除します。`"Bash(rm *)"` のようなスコープ付きルールはツールを利用可能なままにし、すべての権限モード（`bypassPermissions` を含む）で[書かれたとおりの](/docs/ja/permissions#bash-rule-limits)コマンドの一致する呼び出しを拒否します。[権限](/docs/ja/agent-sdk/permissions#allow-and-deny-rules)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `effort`                          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max'`                                                                                                                                                              | `undefined`                            | Claude がレスポンスに費やす努力の量を制御します。適応的思考と連携して思考の深さをガイドします。[努力レベルを調整](/docs/ja/model-config#adjust-effort-level)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `enableFileCheckpointing`         | `boolean`                                                                                                                                                                                                      | `false`                                | ファイル変更追跡を有効にして巻き戻しを可能にします。[ファイルチェックポイント](/docs/ja/agent-sdk/file-checkpointing)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `env`                             | `Record<string, string \| undefined>`                                                                                                                                                                          | `process.env`                          | 環境変数。設定すると、このサブプロセス環境を `process.env` とマージする代わりに置き換えるため、`PATH` のような継承変数を保つために `{ ...process.env, YOUR_VAR: 'value' }` を渡してください。このパターンの例は[遅いまたは停止した API レスポンスを処理](#handle-slow-or-stalled-api-responses)を参照し、基盤となる CLI が読み込む変数については[環境変数](/docs/ja/env-vars)を参照してください。`CLAUDE_AGENT_SDK_CLIENT_APP` を設定して User-Agent ヘッダーでアプリを識別します                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `executable`                      | `'bun' \| 'deno' \| 'node'`                                                                                                                                                                                    | 自動検出                                   | 使用する JavaScript ランタイム                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `executableArgs`                  | `string[]`                                                                                                                                                                                                     | `[]`                                   | 実行可能ファイルに渡す引数                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `extraArgs`                       | `Record<string, string \| null>`                                                                                                                                                                               | `{}`                                   | 追加引数                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `fallbackModel`                   | `string`                                                                                                                                                                                                       | `undefined`                            | プライマリモデルが失敗した場合に使用するモデル。カンマ区切りリストを受け入れます。順序と上限については[フォールバックモデルチェーン](/docs/ja/model-config#fallback-model-chains)を参照してください。ガイダンスについては[モデルを選択](/docs/ja/agent-sdk/configuration#choose-a-model)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `forkSession`                     | `boolean`                                                                                                                                                                                                      | `false`                                | `resume` で再開する場合、元のセッション ID を続行する代わりに新しいセッション ID にフォークします                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `forwardSubagentText`             | `boolean`                                                                                                                                                                                                      | `false`                                | サブエージェントのテキストと思考ブロックをアシスタントおよびユーザーメッセージとして `parent_tool_use_id` を設定して転送し、コンシューマーがネストされたトランスクリプトをレンダリングできるようにします。このオプションがない場合、Claude Code はサブエージェント `tool_use` および `tool_result` ブロックを出力しますが、テキストや思考は出力しません。すべてのネストの深さのサブエージェントからのメッセージは Claude Code v2.1.219 以降で転送されます。v2.1.219 より前では、深さ 1 のサブエージェントからのメッセージのみが表示されました。フォークされたスキルがスポーンするサブエージェントのメッセージ、およびネストされたフォークされたスキルのメッセージには v2.1.275 以降が必要です                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `hooks`                           | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>`                                                                                                              | `{}`                                   | イベントのフックコールバック                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `includeHookEvents`               | `boolean`                                                                                                                                                                                                      | `false`                                | フックライフサイクルイベントをメッセージストリームに [`SDKHookStartedMessage`](#sdkhookstartedmessage)、[`SDKHookProgressMessage`](#sdkhookprogressmessage)、[`SDKHookResponseMessage`](#sdkhookresponsemessage) として含めます。`SessionStart` および `Setup` フックのライフサイクルイベントは常に含まれ、このオプションは不要です。`Notification`、`SessionEnd`、`PreCompact`、`PostCompact` などの一部のフックイベントは、このオプションを使用しても `SDKHookStartedMessage` を生成しません。これらのイベントについては、Claude Code は 1 秒以上実行されるコマンドフックが出力を生成している間 `SDKHookProgressMessage` を出力し、[バックグラウンドで実行](/docs/ja/hooks#run-hooks-in-the-background)するフックが完了したときのみ `SDKHookResponseMessage` を出力します                                                                                                                                                                                                                                                                                                     |
| `includePartialMessages`          | `boolean`                                                                                                                                                                                                      | `false`                                | 部分メッセージイベントを含めます                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `loadTimeoutMs`                   | `number`                                                                                                                                                                                                       | `60000`                                | *アルファ版。* 再開の具体化中に各 `sessionStore.load()` および `sessionStore.listSubkeys()` 呼び出しのタイムアウト（ミリ秒）。アダプターがこのウィンドウ内で解決しない場合、クエリはハングする代わりに失敗します。`sessionStore` が設定されていない場合は無視されます                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `managedSettings`                 | `Settings`                                                                                                                                                                                                     | `undefined`                            | ホストプロセスがスポーンされたセッションに提供するポリシー層設定。管理者がデプロイした管理設定を持つマシンでは、管理者の最優先管理ソースが `parentSettingsBehavior: 'merge'` を設定しない限り、Claude Code はこれらを無視し、[`policyHelper`](/docs/ja/settings-reference#policyhelper) が管理設定を提供している間はマージしません。マージされた値は制限のみのフィルターを通過します。[親設定を制限](/docs/ja/claude-apps-gateway#restrict-parent-settings)はフィルターが許可するもの、および `allowManaged*Only` ロックをカバーします。[`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/ja/env-vars) を設定するホストは、このペイロードから直接 3 つのキーを読み込みます。Claude Code v2.1.222 以降の[モデル設定](/docs/ja/model-config#restrict-model-selection)、管理ソースが設定していない場合の v2.1.246 以降の [`modelPricing`](/docs/ja/settings-reference#modelpricing)、および v2.1.247 以降の `ENABLE_TOOL_SEARCH` env エントリ                                                                                                                                                                                                               |
| `maxBudgetUsd`                    | `number`                                                                                                                                                                                                       | `undefined`                            | クライアント側のコスト推定がこの USD 値に達したときにクエリを停止します。呼び出し自体の支出のみをカウントします。再開されたセッションから復元された合計はカウントされません。精度の注意事項とリセット動作については、[コストと使用状況を追跡](/docs/ja/agent-sdk/cost-tracking)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `maxThinkingTokens`               | `number`                                                                                                                                                                                                       | `undefined`                            | *非推奨:* 代わりに `thinking` を使用してください。思考プロセスの最大トークン数                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `maxTurns`                        | `number`                                                                                                                                                                                                       | `undefined`                            | 最大エージェンティックターン数（ツール使用ラウンドトリップ）                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `mcpServers`                      | `Record<string, [`McpServerConfig`](#mcpserverconfig)>`                                                                                                                                                        | `{}`                                   | MCP サーバー設定                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `model`                           | `string`                                                                                                                                                                                                       | CLI からのデフォルト                           | Claude モデルエイリアスまたは完全なモデル名。[受け入れられた値とプロバイダー固有の ID](/docs/ja/model-config#available-models) を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `onElicitation`                   | `(request: ElicitationRequest, options: { signal: AbortSignal }) => Promise<ElicitationResult>`                                                                                                                | `undefined`                            | MCP エリシテーションリクエストを処理するためのコールバック。MCP サーバーがユーザー入力をリクエストし、フックが最初に処理しない場合に呼び出されます。提供されない場合、未処理のエリシテーションリクエストは自動的に拒否されます                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `outputFormat`                    | `{ type: 'json_schema', schema: JSONSchema }`                                                                                                                                                                  | `undefined`                            | エージェント結果の出力形式を定義します。詳細は[構造化出力](/docs/ja/agent-sdk/structured-outputs)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `outputStyle`                     | `string`                                                                                                                                                                                                       | `undefined`                            | `Options` フィールドではありません。インライン [`settings`](/docs/ja/settings) オブジェクトまたは設定ファイルで `outputStyle` を設定してください。[出力スタイルを有効化](/docs/ja/agent-sdk/modifying-system-prompts#activate-an-output-style)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `pathToClaudeCodeExecutable`      | `string`                                                                                                                                                                                                       | バンドルされたネイティブバイナリから自動解決                 | Claude Code 実行可能ファイルへのパス。インストール中にオプション依存関係がスキップされた場合、またはプラットフォームがサポートセットにない場合にのみ必要です                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `permissionMode`                  | [`PermissionMode`](#permissionmode)                                                                                                                                                                            | `'default'`                            | セッションの権限モード                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `permissionPromptToolName`        | `string`                                                                                                                                                                                                       | `undefined`                            | 権限プロンプト用の MCP ツール名                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `permissionPrompts`               | `'host' \| 'none'`                                                                                                                                                                                             | `'host'`                               | 権限プロンプトに応答するのは誰か: `'host'` はそれらを [`canUseTool`](#canusetool) コールバックまたは `permissionPromptToolName` ツールにルーティングし、`'none'` は[プロンプトが表示されるはずだった呼び出しを拒否](/docs/ja/agent-sdk/permissions#how-permissions-are-evaluated)します。Claude Code v2.1.259 以降が必要です                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `persistSession`                  | `boolean`                                                                                                                                                                                                      | `true`                                 | `false` の場合、ディスクへのセッション永続化を無効にします。セッションは後で再開できません                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `planModeInstructions`            | `string`                                                                                                                                                                                                       | `undefined`                            | プランモードのカスタムワークフロー指示。`permissionMode` が `'plan'` の場合、この文字列はデフォルトのプランモードワークフロー本体を置き換えます。CLI は引き続き読み取り専用強制プリアンブルと ExitPlanMode プロトコルフッターでラップします                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `plugins`                         | [`SdkPluginConfig`](#sdkpluginconfig)`[]`                                                                                                                                                                      | `[]`                                   | ローカルパスからカスタムプラグインを読み込みます。詳細は[プラグイン](/docs/ja/agent-sdk/plugins)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `projectConfigRoot`               | `string`                                                                                                                                                                                                       | `undefined`                            | `cwd` がワークツリーである信頼できるチェックアウトの絶対パス。Claude Code はプロジェクト設定、`.mcp.json`、およびプロジェクトの `.claude/` コマンド、エージェント、スキル、ワークフロー、ルーチン、出力スタイルを `cwd` ではなくこのディレクトリから読み込み、`CLAUDE_PROJECT_DIR` をそれに設定します。フック、`apiKeyHelper` などのヘルパースクリプト、および stdio MCP サーバーはこのディレクトリを作業ディレクトリとして開始します。`CLAUDE.md` ファイルと `.claude/rules/` は引き続き `cwd` から読み込まれます。Claude Code v2.1.275 以降が必要です                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `promptSuggestions`               | `boolean`                                                                                                                                                                                                      | `false`                                | プロンプト提案を有効にします。ターン後、Claude Code は予測される次のユーザープロンプトを含む `prompt_suggestion` メッセージを出力します。アカウントが使用制限に近い、または達している場合など、一部のターンでは Claude Code は提案を生成しません。[Claude Code が提案をスキップする場合](/docs/ja/interactive-mode#when-claude-code-skips-suggestions)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `resume`                          | `string`                                                                                                                                                                                                       | `undefined`                            | 再開するセッション ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `resumeDropsTurn`                 | `string`                                                                                                                                                                                                       | `undefined`                            | `resumeSessionAt` を使用: 切り詰め再開が破棄するつもりのターンのプロンプト UUID。破棄された範囲に、そのターンに帰属しないもの（吸収されたキューに入ったメッセージやタスク通知など）が含まれている場合、Claude Code は再開を拒否し、拒否メッセージで `--resume-drops-turn` フラグを指定します。Agent SDK とプリントモード再開のみがペアを読み込みます。Claude Code v2.1.223 以降が必要です                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `resumeSessionAt`                 | `string`                                                                                                                                                                                                       | `undefined`                            | 特定のメッセージ UUID でセッションを再開します                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `sandbox`                         | [`SandboxSettings`](#sandboxsettings)                                                                                                                                                                          | `undefined`                            | プログラムでサンドボックス動作を設定します。詳細は[サンドボックス設定](#sandboxsettings)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `sessionId`                       | `string`                                                                                                                                                                                                       | 自動生成                                   | 自動生成する代わりに特定の UUID をセッションに使用します                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `sessionStore`                    | [`SessionStore`](/docs/ja/agent-sdk/session-storage#the-sessionstore-interface)                                                                                                                                     | `undefined`                            | セッショントランスクリプトを外部バックエンドにミラーリングして、別のホストがそれらを再開できるようにします。[セッションを外部ストレージに永続化](/docs/ja/agent-sdk/session-storage)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `sessionStoreFlush`               | `'batched' \| 'eager'`                                                                                                                                                                                         | `'batched'`                            | *アルファ版。* `sessionStore` のフラッシュモード。`sessionStore` が設定されていない場合は無視されます                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `settings`                        | `string \| Settings`                                                                                                                                                                                           | `undefined`                            | インライン [settings](/docs/ja/settings) オブジェクト、設定ファイルパス、またはインライン JSON 文字列。[優先順位](/docs/ja/settings#settings-precedence)のフラグ設定層を入力します。[`applyFlagSettings()`](#applyflagsettings) で実行時に変更します                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `settingSources`                  | [`SettingSource`](#settingsource)`[]`                                                                                                                                                                          | CLI デフォルト（すべてのソース）                     | 読み込むファイルシステム設定を制御します。`[]` を渡してユーザー、プロジェクト、ローカル設定を無効にします。[エンドポイント管理ポリシー](/docs/ja/managed-settings#delivery-mechanisms)は関係なく読み込まれます。サーバー管理設定は、セッションが[適格な設定](/docs/ja/server-managed-settings#platform-availability)で組織認証情報を使用して認証するときにフェッチされます。[Claude Code 機能を使用](/docs/ja/agent-sdk/claude-code-features#what-settingsources-does-not-control)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `skills`                          | `string[] \| 'all'`                                                                                                                                                                                            | `undefined`                            | セッションで利用可能なスキル。すべての検出されたスキルを有効にするには `'all'` を渡すか、スキル名のリストを渡します。正確な名前のみを渡してください。Agent SDK v0.3.221 以降では、SDK は形式が正しくないワイルドカード形式の名前を Claude Code プロセスを開始する前にエラーで拒否します。設定すると、SDK は Skill ツールを `allowedTools` に自動的に追加します。`tools` も渡す場合は、そのリストに `'Skill'` を含めてください。[スキル](/docs/ja/agent-sdk/skills)を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `spawnClaudeCodeProcess`          | `(options: SpawnOptions) => SpawnedProcess`                                                                                                                                                                    | `undefined`                            | Claude Code プロセスをスポーンするカスタム関数。VM、コンテナ、またはリモート環境で Claude Code を実行するために使用します                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `stderr`                          | `(data: string) => void`                                                                                                                                                                                       | `undefined`                            | stderr 出力のコールバック                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `strictMcpConfig`                 | `boolean`                                                                                                                                                                                                      | `false`                                | `mcpServers` で渡されたサーバーのみを使用し、プロジェクト `.mcp.json`、ユーザー設定、プラグイン提供の MCP サーバー、および [claude.ai コネクタ](/docs/ja/mcp#use-mcp-servers-from-claude-ai)を無視します                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `systemPrompt`                    | `string \| string[] \| { type: 'custom'; prompt: string \| string[]; snapshot?: boolean } \| { type: 'preset'; preset: 'claude_code'; append?: string; excludeDynamicSections?: boolean; snapshot?: boolean }` | `undefined`（最小プロンプト）                   | システムプロンプト設定。カスタムプロンプトの場合は文字列を渡すか、Claude Code のシステムプロンプトを使用するには `{ type: 'preset', preset: 'claude_code' }` を渡します。エクスポートされた `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` 定数を静的部分とリクエストごとの部分の間に含む文字列の配列を渡して、[カスタムプロンプトの静的部分をキャッシュ](/docs/ja/agent-sdk/modifying-system-prompts#cache-the-static-part-of-a-custom-prompt)します。プリセットオブジェクト形式を使用する場合、`append` を追加して追加の指示で拡張し、`excludeDynamicSections: true` を設定して[マシン間でのプロンプトキャッシュの再利用を改善](/docs/ja/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines)するためにセッションごとのコンテキストを最初のユーザーメッセージに移動します。`snapshot: false` を設定して、[セッションが最初のリクエストで記録したプロンプトを再利用](/docs/ja/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session)する代わりに、すべてのリクエストでプロンプトを再構築します。カスタムプロンプトで `snapshot` を設定するには、`{ type: 'custom', prompt }` 形式を渡します。`{ type: 'custom' }` 形式と `snapshot` フィールドには TypeScript Agent SDK v0.3.257 以降が必要です |
| `taskBudget`                      | `{ total: number }`                                                                                                                                                                                            | `undefined`                            | *アルファ版。* API 側のタスク予算（トークン）。設定すると、モデルに残りのトークン予算が通知されるため、ツール使用のペースを調整し、制限前にラップアップできます                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `thinking`                        | [`ThinkingConfig`](#thinkingconfig)                                                                                                                                                                            | サポートされているモデルの場合 `{ type: 'adaptive' }` | Claude の思考/推論動作を制御します。オプションについては [`ThinkingConfig`](#thinkingconfig) を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `title`                           | `string`                                                                                                                                                                                                       | `undefined`                            | セッションの表示タイトル。`resume` または `continue` で再開する場合、再開されたセッションの永続化されたタイトルが優先されます。既存のセッションを再タイトルするには [`renameSession()`](#renamesession) を使用してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `toolAliases`                     | `Record<string, string>`                                                                                                                                                                                       | `undefined`                            | 組み込みツール名を MCP ツール名にマップして、Claude が組み込みの代わりに MCP 実装を呼び出すようにします。例えば、`{ Bash: 'mcp__workspace__bash' }`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `toolConfig`                      | [`ToolConfig`](#toolconfig)                                                                                                                                                                                    | `undefined`                            | 組み込みツール動作の設定。詳細は [`ToolConfig`](#toolconfig) を参照してください                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `tools`                           | `string[] \| { type: 'preset'; preset: 'claude_code' }`                                                                                                                                                        | `undefined`                            | ツール設定。ツール名の配列を渡すか、プリセットを使用して Claude Code のデフォルトツールを取得します                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |

<h4 id="handle-slow-or-stalled-api-responses">
  遅いまたは停止した API レスポンスを処理
</h4>

CLI サブプロセスは、API タイムアウトとスタール検出を制御するいくつかの環境変数を読み込みます。`env` オプションを通じてそれらを渡します:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const result = query({
  prompt: "Analyze this code",
  options: {
    env: {
      ...process.env,
      API_TIMEOUT_MS: "120000",
      CLAUDE_CODE_MAX_RETRIES: "2",
      CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS: "120000",
    },
  },
});
```

* `API_TIMEOUT_MS`: Anthropic クライアントのリクエストごとのタイムアウト（ミリ秒）。デフォルト `600000`。メインループとすべてのサブエージェントに適用されます。
* `CLAUDE_CODE_MAX_RETRIES`: 最大 API リトライ数。デフォルト `10`、上限 `15`。各リトライは独自の `API_TIMEOUT_MS` ウィンドウを取得するため、最悪の場合の実時間は大約 `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` にバックオフを加えたものです。長い停止を待つ必要がある無人実行の場合は、[`CLAUDE_CODE_RETRY_WATCHDOG=1`](/docs/ja/errors#tune-retry-behavior) を設定してください。一時的な容量エラーを無限に再試行し、Claude Code v2.1.199 以降では、他の一時的なエラーのデフォルトを `300` に引き上げ、この変数の上限を削除します。
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: サブエージェント用のスタールウォッチドッグ。ストリームウォッチドッグがオンの場合、デフォルトは `CLAUDE_STREAM_IDLE_TIMEOUT_MS` プラス 5 分で、その変数を引き上げない限り `600000` になります。ストリームウォッチドッグがオフの場合、デフォルトは `600000` です。v2.1.257 より前では、デフォルトは常に `600000` でした。

  タイマーは各ストリームイベントでリセットされます。スタール時、Claude Code はサブエージェントを中止し、スタールを親に報告します。バックグラウンドサブエージェントの場合、タスクも失敗とマークし、部分的な結果を添付します。
* `CLAUDE_ENABLE_STREAM_WATCHDOG` と `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: ヘッダーが到着したがレスポンス本体がストリーミングを停止したときにリクエストを中止するストリームウォッチドッグ。ウォッチドッグはすべてのプロバイダーでデフォルトでオンです。無効にするには `CLAUDE_ENABLE_STREAM_WATCHDOG=0` を設定してください。`CLAUDE_STREAM_IDLE_TIMEOUT_MS` はデフォルト `300000` で、その最小値にクランプされます。中止後、[自動リトライ](/docs/ja/errors#automatic-retries)はレスポンスがどこまで進んだかに基づいて Claude Code が何をするかをカバーします。

  `ANTHROPIC_BASE_URL` の背後にあるゲートウェイがキープアライブピングで開いたままにしているレスポンスをウォッチドッグが待機している間、`includePartialMessages` を設定するホストは `ping` [ストリームイベント](#sdkpartialassistantmessage)を受け取り続けるため、これらのフレームを沈黙でセッションをタイムアウトするのではなく活性度として読み込んでください。v2.1.257 より前では、フレームは最後の実際のストリームイベントから 5 分後に停止しました。

<h3 id="query-object">
  `Query` オブジェクト
</h3>

`query()` 関数によって返されるインターフェース。

```typescript theme={null}
interface Query extends AsyncGenerator<SDKMessage, void> {
  interrupt(): Promise<SDKControlInterruptResponse | undefined>;
  rewindFiles(
    userMessageId: string,
    options?: { dryRun?: boolean }
  ): Promise<RewindFilesResult>;
  setPermissionMode(mode: PermissionMode): Promise<void>;
  setModel(model?: string): Promise<void>;
  setMaxThinkingTokens(maxThinkingTokens: number | null): Promise<void>;
  applyFlagSettings(settings: {
    [K in keyof Settings]?: K extends 'effortLevel'
      ? 'low' | 'medium' | 'high' | 'xhigh' | 'max' | null
      : Settings[K] | null;
  }): Promise<void>;
  updateSettings(
    source: 'localSettings' | 'userSettings',
    settings: Record<string, unknown>,
  ): Promise<void>;
  initializationResult(): Promise<SDKControlInitializeResponse>;
  reinitialize(): Promise<SDKControlInitializeResponse>;
  supportedCommands(): Promise<SlashCommand[]>;
  supportedModels(): Promise<ModelInfo[]>;
  supportedAgents(): Promise<AgentInfo[]>;
  mcpServerStatus(): Promise<McpServerStatus[]>;
  getContextUsage(opts?: {
    detail?: 'summary' | 'full';
  }): Promise<SDKControlGetContextUsageResponse>;
  readFile(
    path: string,
    options?: { maxBytes?: number; encoding?: 'utf-8' | 'base64' }
  ): Promise<SDKControlReadFileResponse | null>;
  reloadSkills(): Promise<SDKControlReloadSkillsResponse>;
  accountInfo(): Promise<AccountInfo>;
  reconnectMcpServer(serverName: string): Promise<void>;
  toggleMcpServer(serverName: string, enabled: boolean): Promise<void>;
  setMcpServers(servers: Record<string, McpServerConfig>): Promise<McpSetServersResult>;
  readMcpResource(serverName: string, uri: string): Promise<SDKControlMcpReadResourceResponse>;
  streamInput(stream: AsyncIterable<SDKUserMessage>): Promise<void>;
  stopTask(taskId: string): Promise<void>;
  close(): void;
}
```

<h4 id="methods">
  メソッド
</h4>

| メソッド                                   | 説明                                                                                                                                                                                                                                                                                                                                                                                   |
| :------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt()`                          | クエリを中断します。ストリーミング入力モードでのみ利用可能です。CLI が [`SDKSystemMessage.capabilities`](#sdksystemmessage) で `interrupt_receipt_v1` 機能をアドバタイズする場合、中断が到着したときに保留中だったメッセージをリストする [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) で解決します。v2.1.205 より前の CLI では `undefined` で解決します                                                                                                           |
| `rewindFiles(userMessageId, options?)` | ファイルを指定されたユーザーメッセージの状態に復元します。変更をプレビューするには `{ dryRun: true }` を渡します。`enableFileCheckpointing: true` が必要です。[ファイルチェックポイント](/docs/ja/agent-sdk/file-checkpointing)を参照してください                                                                                                                                                                                                                   |
| `setPermissionMode()`                  | 権限モードを変更します（ストリーミング入力モードでのみ利用可能）                                                                                                                                                                                                                                                                                                                                                     |
| `setModel()`                           | モデルを変更します（ストリーミング入力モードでのみ利用可能）。`undefined` または文字列 `"default"` を渡すと、[Claude Code のデフォルトモデル](/docs/ja/model-config)にリセットされます                                                                                                                                                                                                                                                                |
| `setMaxThinkingTokens()`               | *非推奨:* 代わりに `thinking` オプションを使用してください。最大思考トークン数を変更します。`null` を渡すと、思考をセッションデフォルトにリセットします。中セッション上書きはクリアされ、思考が無効なセッションでは思考は無効のままです                                                                                                                                                                                                                                                      |
| `applyFlagSettings(settings)`          | 実行時にセッションのフラグ設定層に設定をマージします（ストリーミング入力モードでのみ利用可能）。[`applyFlagSettings()`](#applyflagsettings) を参照してください                                                                                                                                                                                                                                                                                |
| `updateSettings(source, settings)`     | プロジェクトのローカル設定ファイルまたはユーザー設定ファイルにホワイトリストキーを書き込み、値が後のセッションで永続化されるようにします。[`updateSettings()`](#updatesettings) を参照してください。TypeScript SDK v0.3.257 以降が必要で、Claude Code v2.1.257 をバンドルしています                                                                                                                                                                                                  |
| `initializationResult()`               | サポートされているコマンド、モデル、アカウント情報、出力スタイル設定を含む完全な初期化結果を返します                                                                                                                                                                                                                                                                                                                                   |
| `reinitialize()`                       | 実行中の CLI に `initialize` 制御リクエストを再送信し、キャッシュされた最初の接続結果の代わりに新しい結果を返します。トランスポートギャップ後（切断後のセッションへの再接続など）に使用して、保留中の権限リクエストが `canUseTool` コールバックに再度到達するようにします。コールバックをリクエスト ID ごとにべき等にしてください。応答が失われたリクエストは再度ディスパッチされるためです。Claude Code v2.1.195 以降が必要です                                                                                                                                        |
| `supportedCommands()`                  | 利用可能なコマンドを返します。Agent SDK v0.3.216 からリストは中セッションコマンド変更を反映します。[`SDKCommandsChangedMessage`](#sdkcommandschangedmessage) を参照してください                                                                                                                                                                                                                                                       |
| `supportedModels()`                    | 表示情報を含む利用可能なモデルを返します                                                                                                                                                                                                                                                                                                                                                                 |
| `supportedAgents()`                    | [`AgentInfo`](#agentinfo)`[]` として利用可能なサブエージェントを返します                                                                                                                                                                                                                                                                                                                                  |
| `mcpServerStatus()`                    | 接続された MCP サーバーのステータスを [`McpServerStatus`](#mcpserverstatus)`[]` として返します                                                                                                                                                                                                                                                                                                              |
| `getContextUsage(opts?)`               | セッションのコンテキストウィンドウ使用状況をカテゴリ、スキル、ツール別に分類する [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse) を返します。デフォルト `detail` では、インタラクティブセッションで `/context` が表示するのと同じデータです。[`detail` オプション](#sdkcontrolgetcontextusageresponse)には Agent SDK v0.3.257 以降が必要です                                                                                                                |
| `readFile(path, options?)`             | セッションのファイルシステムからファイルを読み込みます。Claude Code は `cwd` に対してパスを解決します。[`readFile()` が読み込める内容](#what-readfile-can-read)は提供するファイルをリストします。読み込みキャップを変更するには `{ maxBytes }` を渡します（デフォルト 1 MB、上限 10 MB）。画像などのバイナリファイルの場合は `{ encoding: 'base64' }` を渡します。[`SDKControlReadFileResponse`](#sdkcontrolreadfileresponse) で解決するか、権限拒否、ファイル欠落、またはトランスポートエラーで `null` で解決します。TypeScript SDK v0.2.121 以降が必要です |
| `reloadSkills()`                       | ディスクからスキルを再読み込みして、中セッションで追加または編集したスキルが実行中のセッションで利用可能になるようにします。再読み込み後に利用可能なスキルをリストする [`SDKControlReloadSkillsResponse`](#sdkcontrolreloadskillsresponse) で解決します。Agent SDK v0.3.163 以降が必要です                                                                                                                                                                                            |
| `accountInfo()`                        | アカウント情報を返します                                                                                                                                                                                                                                                                                                                                                                         |
| `reconnectMcpServer(serverName)`       | MCP サーバーを名前で再接続します。名前が `.mcp.json` または `~/.claude.json` などの設定ファイルのエントリとも一致する場合、Claude Code は設定ファイルエントリではなく、[`mcpServers`](#options) または `setMcpServers()` を通じて設定したサーバーを再接続します。その解決順序には Claude Code v2.1.257 以降が必要です                                                                                                                                                                  |
| `toggleMcpServer(serverName, enabled)` | `reconnectMcpServer()` と同じ名前解決で、MCP サーバーを名前で有効または無効にします。無効にするとサーバーが切断されます                                                                                                                                                                                                                                                                                                            |
| `setMcpServers(servers)`               | このセッションの MCP サーバーセットを動的に置き換えます。追加および削除されたサーバーと任意のエラーを指定する [`McpSetServersResult`](#mcpsetserversresult) で解決します                                                                                                                                                                                                                                                                       |
| `readMcpResource(serverName, uri)`     | *アルファ版。* 接続された MCP サーバーから 1 つの MCP Apps `ui://` リソースを読み込み、アプリケーションがツールのウィジェットをレンダリングできるようにします。[`SDKControlMcpReadResourceResponse`](#sdkcontrolmcpreadresourceresponse) で解決します。TypeScript Agent SDK v0.3.280 以降が必要です                                                                                                                                                                 |
| `streamInput(stream)`                  | マルチターン会話のためにクエリに入力メッセージをストリーミングします                                                                                                                                                                                                                                                                                                                                                   |
| `stopTask(taskId)`                     | ID でバックグラウンドタスクを実行中に停止します                                                                                                                                                                                                                                                                                                                                                            |
| `close()`                              | クエリを閉じて基盤となるプロセスを終了します。クエリを強制的に終了し、すべてのリソースをクリーンアップします                                                                                                                                                                                                                                                                                                                               |

<h4 id="applyflagsettings">
  `applyFlagSettings()`
</h4>

クエリを再開せずに実行中のセッションで [settings](/docs/ja/settings) を変更します。信頼できない入力を読み込んだ後に権限を厳しくするなど、専用セッターがない設定を中セッションで変更する必要がある場合に使用します。`setModel()` と `setPermissionMode()` はこれら 2 つのキーの専用セッターです。`applyFlagSettings()` は設定キーの任意のサブセットを受け入れる一般的な形式で、ここで `model` を渡すことは `setModel()` と同じように動作します。

一部のキーのみが中セッションで有効になります:

* **次のターンで適用**: `effortLevel`、`ultracode`、`permissions`、`hooks`、`skillOverrides`、`fastMode`、`agent`。`agent` を切り替えると、そのエージェントのモデル上書きとフックも次のターンで適用されます。そのシステムプロンプトは次のターンで、または[記録されたシステムプロンプトを再利用](/docs/ja/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session)するセッションではセッションがコンパクト化されると適用されます。
* **現在のターン中に適用**: `model`。Claude がターンで作業している間に `model` を切り替える場合、Claude が既に生成しているレスポンスは古いモデルで完了し、ターンの残り（Claude Code がモデルに対して行う次の呼び出しから始まる）は新しいモデルを使用します。サブエージェントは独自のモデルを保持します。v2.1.212 より前では、中ターン切り替えは次のターンを待機していました。
* **中セッションで効果なし**: システムプロンプトオプション。これらはスタートアップで 1 回解決されるため、実行中のセッションは呼び出しが成功しても元の値を保持します。それらを変更するには、新しいセッションを開始してください。

`effortLevel` は[努力レベル](/docs/ja/model-config#adjust-effort-level)名を受け入れます。また、`"ultracode"` も受け入れます。これは [ultracode](/docs/ja/workflows#let-claude-decide-with-ultracode) を使用した `xhigh` 努力をリクエストします。`applyFlagSettings()` は `effortLevel` をその値なしで宣言するため、TypeScript で同等の `{ ultracode: true }` を渡してください。`ultracode` 値には Claude Code v2.1.203 以降が必要で、`applyFlagSettings()` でのみ受け入れられ、設定ファイルの `effortLevel` キーでは受け入れられません。

値はフラグ設定層に書き込まれます。これは `query()` のインライン `settings` オプションがスタートアップで入力するのと同じ層です。これは[オンページ優先順位セクション](#settings-precedence)がプログラムオプションと呼ぶのと同じ層です。

連続した呼び出しは最上位レベルキーをシャローマージします。`{ permissions: {...} }` を含む 2 番目の呼び出しは、前の呼び出しからの `permissions` オブジェクト全体を深くマージするのではなく置き換えます。フラグ層からキーをクリアするには、そのキーに `null` を渡します。ほとんどのキーはその後、低優先度ソースにフォールバックします。クリアされた `model` は、設定ファイルが `model` を設定している場合でも、[Claude Code のデフォルトモデル](/docs/ja/model-config)にリセットされます。`undefined` を渡すと JSON シリアル化がそれをドロップするため効果がありません。

`model` 以外の 3 つのキーはセッション状態をリセットする代わりにフォールバックします:

* `effortLevel: null` はセッションをモデルのデフォルト努力レベルに返します。`query()` の `effort` オプションまたは設定ファイルの `effortLevel` ではなく。
* `agent: null` は次のターンから、メインスレッドをエージェントなしで実行します。`query()` の `agent` オプションまたは設定ファイルの `agent` を復元するのではなく。クリアされたエージェントが独自のモデルを適用していた場合、セッションはスタートアップで解決したモデルに戻ります。
* `ultracode: null` は `false` と同様に ultracode をオフにします。設定ファイルから `ultracode` 値を復元するのではなく。セッションは現在の努力レベルを保持するため、同じ呼び出しで `effortLevel` を渡してそれを変更します。

ストリーミング入力モードでのみ利用可能です。これは `setModel()` と `setPermissionMode()` と同じ制約です。

以下の例は中セッションでアクティブなモデルを切り替え、その後上書きをクリアしてモデルを [Claude Code のデフォルトモデル](/docs/ja/model-config)にリセットします。

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const q = query({ prompt: messageStream });

// Override the model for the rest of the session
await q.applyFlagSettings({ model: "claude-opus-4-6" });

// Later: clear the override; the model resets to Claude Code's default
await q.applyFlagSettings({ model: null });
```

<Note>
  `applyFlagSettings()` は TypeScript のみです。Python SDK は同等のメソッドを公開していません。
</Note>

<h4 id="updatesettings">
  `updateSettings()`
</h4>

設定ファイルにホワイトリストキーを書き込み、値が後のセッションで永続化されるようにします。各ソースは 1 つのキーを受け入れます。文字列値を持ちます:

* **`"localSettings"`**: `outputStyle` を受け入れ、プロジェクトのローカル設定ファイル `.claude/settings.local.json` にマージします。新しいスタイルはセッションの次のリクエストで有効になります。
* **`"userSettings"`**: `effortLevel` を受け入れ、セッションの現在のモデルのデフォルト[努力レベル](/docs/ja/model-config#adjust-effort-level)としてユーザー設定ファイルの [`modelSettings`](/docs/ja/settings-reference#modelsettings) の下に保存します。`max` を渡すと、`max` はセッションのみであるため何も書き込みません。実行中のセッションは現在の努力レベルをどちらにしても保持するため、変更したい場合は [`applyFlagSettings()`](#applyflagsettings) を呼び出します。このソースには TypeScript SDK v0.3.277 以降が必要で、Claude Code v2.1.277 をバンドルしています。

呼び出しは、リクエストが他のキーを運ぶ場合、セッションがリモートトランスポートを通じて実行される場合、およびセッションの [`settingSources`](#options) が指定したソースを除外する場合に拒否されます。キーの削除はサポートされていません。

<h3 id="warmquery">
  `WarmQuery`
</h3>

[`startup()`](#startup) によって返されるハンドル。サブプロセスは既にスポーンおよび初期化されているため、このハンドルで `query()` を呼び出すと、スタートアップレイテンシーなしで準備完了プロセスにプロンプトを直接書き込みます。

```typescript theme={null}
interface WarmQuery extends AsyncDisposable {
  query(prompt: string | AsyncIterable<SDKUserMessage>): Query;
  close(): void;
}
```

<h4 id="methods-2">
  メソッド
</h4>

| メソッド            | 説明                                                                                          |
| :-------------- | :------------------------------------------------------------------------------------------ |
| `query(prompt)` | 事前ウォーミングされたサブプロセスにプロンプトを送信し、[`Query`](#query-object) を返します。`WarmQuery` ごとに 1 回のみ呼び出すことができます |
| `close()`       | プロンプトを送信せずにサブプロセスを閉じます。不要になった暖かいクエリを破棄するために使用します                                            |

`WarmQuery` は `AsyncDisposable` を実装するため、自動クリーンアップのために `await using` で使用できます。

<h3 id="sdkcontrolinitializeresponse">
  `SDKControlInitializeResponse`
</h3>

`initializationResult()` の戻り値の型。セッション初期化データを含みます。

```typescript theme={null}
type SDKControlInitializeResponse = {
  commands: SlashCommand[];
  agents: AgentInfo[];
  output_style: string;
  available_output_styles: string[];
  models: ModelInfo[];
  account: AccountInfo;
  fast_mode_state?: "off" | "cooldown" | "on";
  fast_mode_disabled_reason?: FastModeDisabledReason;
  hooks_applied?: boolean;
};
```

`hooks_applied` は Claude Code が `initialize` リクエストが運んだ `hooks` を登録したかどうかを報告します。SDK はセッション開始時にそのリクエストを 1 回送信し、各 [`reinitialize()`](#query-object) 呼び出しで再度送信します。フィールドには Agent SDK v0.3.238 以降が必要です。

Claude Code はリクエストがフックを運ばなかった場合、フィールドを省略します。リクエストがフックを運んだ場合、値はリクエストがセッションの最初の初期化かどうか、および繰り返されたものの場合、セッションに到達した方法に依存します:

* `true`: Claude Code はフックを登録しました。セッションの最初の初期化はこの値を返します。CLI の stdin を通じて送信された繰り返し初期化も `true` を返します。その場合、新しいリクエストのフックは以前に登録されたフックを置き換えます。
* `false`: Claude Code はフックを無視しました。リモートセッションに送信された繰り返し初期化はこの値を返すため、セッションに参加する 2 番目のクライアントは最初のクライアントが登録したフックを置き換えることはできません。

Agent SDK v0.3.238 より前では、レスポンスはフィールドを運ばず、Claude Code はすべての繰り返し初期化で `hooks` を無視していました。

レスポンスは常に `fast_mode_state` を報告し、何かが[高速モード](/docs/ja/fast-mode)をブロックする場合、`fast_mode_disabled_reason` はブロックされた状態を説明できるように理由コードを運びます。両方の動作には Claude Code v2.1.219 以降が必要です。v2.1.219 より前では、高速モードが利用可能でない場合、レスポンスは `fast_mode_state` を省略し、理由を運びませんでした。理由コードとその意味については、結果メッセージの [`fast_mode_disabled_reason`](#sdkresultmessage) を参照してください。

成功した `initialize` の制御レスポンスラッパーは `pending_permission_requests` 配列も運びます。フィールドは上記の `SDKControlInitializeResponse` ペイロードではなく、レスポンスラッパー自体にあります。各エントリは、セッションが実行中に権限リクエストのためにストリーミングするのと同じ `{ type: "control_request", request_id, request }` 形状を持つ完全な `control_request` メッセージです。

配列は、この Claude Code プロセスが発行し、まだ解決していない権限リクエストをリストします。SDK はあなたのために配列を読み込み、各エントリを [`canUseTool`](#canusetool) コールバック（接続が切断される前にコールバックが既に受け取ったリクエストの再配信と同じ）にディスパッチします。繰り返されたリクエスト ID をべき等に処理してください。エントリはコールバックが既に受け取ったリクエストを繰り返すことができるためです。

配列は成功した `initialize` レスポンスで常に存在し、このプロセスに未解決の権限リクエストがない場合は空です。Claude Code v2.1.268 以降が必要です。以前のバージョンはフィールドを省略する可能性があるため、ワイヤープロトコルを自分で解析する場合は、欠落しているフィールドを古い CLI として扱い、何も保留中でないという証拠ではなく扱ってください。

<h3 id="sdkcontrolinterruptresponse">
  `SDKControlInterruptResponse`
</h3>

割り込み受信: [`interrupt()`](#query-object) が [`SDKSystemMessage.capabilities`](#sdksystemmessage) で `interrupt_receipt_v1` 機能をアドバタイズする CLI で解決する値。Claude Code v2.1.205 以降が必要です。以前の CLI は割り込みに空の成功ペイロードで応答するため、`interrupt()` は `undefined` で解決します。

```typescript theme={null}
type SDKControlInterruptResponse = {
  still_queued: string[];
  cancelled?: string[];
};
```

`still_queued` は割り込みが到着したときに保留中だったユーザーメッセージの UUID をリストします。キューに入ったままのメッセージ、および Claude Code が既にキューから次のターンのために取り出したメッセージです。セッションの最初のターンが開始した後、Claude Code は最初に割り込みをキャンセルしない限り、リストされたメッセージを処理します。複数を 1 つのターンにマージできます。最初のターンが開始する前に割り込みする場合、Claude Code はそのターンが開始するとすぐに中止し、そのターンのリストされたメッセージはレスポンスを取得しません。

リストを使用して、何を再送信するかを決定します。リストされたメッセージを再送信しない場合、レスポンスを取得するかどうかに関わらず、会話に入り、Claude に 2 回配信されます。

これらの注意事項でリストを解釈します:

* UUID でエンキューされたメッセージのみが表示されます。空の配列は他に何も実行されないことを意味しません。
* メインスレッドメッセージのみがリストされます。サブエージェントにアドレス指定されたメッセージはスコープ外です。
* リストには、クライアントが送信しなかった UUID（[スケジュール済みタスク](/docs/ja/scheduled-tasks)トリガーなど）を含めることができます。エラーとして扱う代わりに、認識しない UUID を無視してください。

CLI の制御プロトコルを `interrupt()` を通じてではなく直接駆動するクライアントは、`interrupt` 制御リクエストで `cancel_queued: true` を設定できます。Claude Code v2.1.219 以降は [`SDKSystemMessage.capabilities`](#sdksystemmessage) で `interrupt_cancel_queued_v1` 機能でサポートをアドバタイズします。古い CLI はフィールドを無視し、キューに入ったメッセージを通常どおり実行させます。そのような割り込みは、`still_queued` の下にリストされるはずのすべてのメッセージもキャンセルします。それらは代わりに `cancelled` の下にリストされ、`still_queued` は空で、それらのどれも実行されません。

`cancelled` リストは `still_queued` と同じ注意事項を運びます。`interrupt()` メソッドは `cancel_queued` を送信しないため、それが解決するレシートは `cancelled` を運びません。

レシートは割り込みが処理される瞬間に撮られたスナップショットで、クリーン割り込みでは中断されたターンの [`SDKResultMessage`](#sdkresultmessage) の前に到着します。そのレシートの後のキューを検査する代わりにレシートを読み込んでください。ループは次のキューに入ったターンをすぐに開始するため、結果の後に検査するキューは既に変更されています。

<h3 id="sdkcontrolgetcontextusageresponse">
  `SDKControlGetContextUsageResponse`
</h3>

[`getContextUsage()`](#query-object) の戻り値の型。デフォルト `detail` では、これはインタラクティブセッションで `/context` コマンドが Claude Code がレンダリングするのと同じペイロードで、トークンカウントと共に `color` や `gridRows` などの表示フィールドを運びます。これは Claude Code が `/context` 使用グリッドを描画するために使用します。

メソッドのオプション `detail` 引数は Claude Code が各カテゴリをカウントする方法を選択します。デフォルト `'full'` では、Claude Code はトークンカウント API リクエストで各カテゴリをカウントします。`{ detail: 'summary' }` を渡して、最後のレスポンスの使用状況とローカル推定から答えを取得します。トークンカウントリクエストは出ていかず、カテゴリごとの数値は概算です。`detail` 引数には Agent SDK v0.3.257 以降が必要です。

メソッドを呼び出す代わりに `/context` をプロンプトとして送信する場合、Claude Code は結果を配信するアシスタントメッセージの `context_usage` フィールドに [`SDKContextUsage`](#sdkcontextusage) ペイロードを添付します。そのフィールドには Agent SDK v0.3.232 以降が必要です。

```typescript theme={null}
type SDKControlGetContextUsageResponse = {
  categories: {
    name: string;
    tokens: number;
    color: string;
    isDeferred?: boolean;
  }[];
  totalTokens: number;
  maxTokens: number;
  rawMaxTokens: number;
  percentage: number;
  gridRows: {
    color: string;
    isFilled: boolean;
    categoryName: string;
    tokens: number;
    percentage: number;
    squareFullness: number;
  }[][];
  model: string;
  memoryFiles: {
    path: string;
    type: string;
    tokens: number;
  }[];
  mcpTools: {
    name: string;
    serverName: string;
    tokens: number;
    isLoaded?: boolean;
  }[];
  deferredBuiltinTools?: {
    name: string;
    tokens: number;
    isLoaded: boolean;
  }[];
  systemTools?: {
    name: string;
    tokens: number;
  }[];
  systemPromptSections?: {
    name: string;
    tokens: number;
  }[];
  agents: {
    agentType: string;
    source: string;
    tokens: number;
  }[];
  slashCommands?: {
    totalCommands: number;
    includedCommands: number;
    tokens: number;
  };
  skills?: {
    totalSkills: number;
    includedSkills: number;
    tokens: number;
    skillFrontmatter: {
      name: string;
      source: string;
      tokens: number;
    }[];
  };
  autoCompactThreshold?: number;
  isAutoCompactEnabled: boolean;
  messageBreakdown?: {
    toolCallTokens: number;
    toolResultTokens: number;
    attachmentTokens: number;
    assistantMessageTokens: number;
    userMessageTokens: number;
    redirectedContextTokens: number;
    unattributedTokens: number;
    toolCallsByType: {
      name: string;
      callTokens: number;
      resultTokens: number;
    }[];
    attachmentsByType: {
      name: string;
      tokens: number;
    }[];
  };
  apiUsage: {
    input_tokens: number;
    output_tokens: number;
    cache_creation_input_tokens: number;
    cache_read_input_tokens: number;
  } | null;
};
```

コレクションフィールドからトークン帰属を読み込みます:

* `categories` はカテゴリごとの合計を保持します。
* `mcpTools` と `agents` はトークンを個別の MCP ツールとサブエージェントに帰属させます。
* `memoryFiles` は各読み込まれたメモリファイルをそのコストと共にリストします。
* `skills.skillFrontmatter` はスキルリストのトークンを含まれた各スキルに帰属させます。スキルごとのカウントは各スキルのリストエントリを Claude Code が実際に送信するときに測定し、スキルの完全なフロントマターより短くなる可能性があります。`skills.totalSkills` を `skills.includedSkills` と比較して、すべての検出されたスキルがリストに含まれたかどうかを確認します。

`totalTokens` はセッションの現在のコンテキスト使用状況で、`maxTokens` はその使用状況が測定されるウィンドウです。そのウィンドウはモデルのコンテキストウィンドウ、または 1 つが適用される場合は低い自動コンパクション ウィンドウです。`rawMaxTokens` は `maxTokens` と同じ値を運び、`percentage` はそのウィンドウの丸められたパーセンテージとしての `totalTokens` です。

Claude Code はオプション `deferredBuiltinTools`、`systemTools`、`systemPromptSections` 診断を設定しないため、フィールドが存在しない場合でも期待してください。

<h3 id="sdkcontrolreadfileresponse">
  `SDKControlReadFileResponse`
</h3>

[`readFile()`](#query-object) の戻り値の型。

```typescript theme={null}
type SDKControlReadFileResponse = {
  contents: string;
  absPath: string;
  truncated?: boolean;
  encoding?: 'base64';
};
```

`contents` はファイルテキストを保持するか、`encoding: 'base64'` をリクエストした場合は base64 データを保持します。レスポンスの `encoding` フィールドはその場合 `'base64'` に設定されます。`absPath` は解決された絶対パスです。`truncated` はファイルが `maxBytes` キャップより長く、コンテンツがその制限で切り詰められた場合に設定されます。

<h4 id="what-readfile-can-read">
  `readFile()` が読み込める内容
</h4>

`readFile()` は Read ツールより狭いファイルセットを提供します:

* `cwd` や `additionalDirectories` などのセッションの作業ディレクトリの 1 つ内の通常ファイル
* セッションのツール結果などの Claude Code 独自のいくつかのファイル

Read 拒否および質問ルールは引き続き一致するパスをブロックし、広い Read 許可ルールは `readFile()` にファイルシステムの残りを開きません。他のすべてについて、呼び出しは `null` で解決します。

<h3 id="sdkcontrolreloadskillsresponse">
  `SDKControlReloadSkillsResponse`
</h3>

[`reloadSkills()`](#query-object) の戻り値の型。

```typescript theme={null}
type SDKControlReloadSkillsResponse = {
  skills: SlashCommand[];
};
```

`skills` は再読み込み後に利用可能なスキルをリストします。`supportedCommands()` が返すのと同じ [`SlashCommand`](#slashcommand) 形状です。

<h3 id="sdkcontrolmcpreadresourceresponse">
  `SDKControlMcpReadResourceResponse`
</h3>

[`readMcpResource()`](#query-object) の戻り値の型。MCP サーバーの `resources/read` 結果を運びます。TypeScript Agent SDK v0.3.280 以降が必要です。

```typescript theme={null}
type SDKControlMcpReadResourceResponse = {
  contents: {
    uri: string;
    mimeType?: string;
    text?: string;
    blob?: string;
    _meta?: Record<string, unknown>;
  }[];
};
```

`readMcpResource()` にサーバー名を `mcpServerStatus()` が報告するのと同じように、および `ui://` URI（ツールが [`_meta`](#mcpserverstatus) で宣言する `ui.resourceUri` など）を渡します。呼び出しは他の URI スキーム、アプリケーションが自分でホストする [SDK MCP サーバー](#createsdkmcpserver)、および接続されていないサーバーに対して拒否されます。初期化メッセージの [`capabilities`](#sdksystemmessage) に `mcp_read_resource_v1` が含まれている場合に利用可能です。

各 `contents` エントリは、サーバーが送信した 1 つのコンテンツアイテムです。`blob` はバイナリアイテムの base64 データを保持し、`_meta` はアイテム自体の `_meta` で、MCP Apps サーバーはリソースの `ui.csp` と `ui.permissions` を配置します。コンテンツは信頼できない第三者の HTML であるため、サンドボックスでレンダリングしてください。

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

プログラムで定義されたサブエージェントの設定。

```typescript theme={null}
type AgentDefinition = {
  description: string;
  tools?: string[];
  disallowedTools?: string[];
  prompt: string;
  model?: string;
  mcpServers?: AgentMcpServerSpec[];
  skills?: string[];
  initialPrompt?: string;
  maxTurns?: number;
  background?: boolean;
  omitClaudeMd?: boolean;
  memory?: "user" | "project" | "local";
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | number;
  permissionMode?: PermissionMode;
  criticalSystemReminder_EXPERIMENTAL?: string;
};
```

| フィールド                                 | 必須  | 説明                                                                                                                                                                                                                        |
| :------------------------------------ | :-- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `description`                         | はい  | このエージェントを使用する場合の自然言語説明                                                                                                                                                                                                    |
| `tools`                               | いいえ | 許可されたツール名の配列。省略した場合、[サブエージェントで利用可能なすべてのツール](/docs/ja/sub-agents#available-tools)を継承します。スキルをエージェントのコンテキストにプリロードするには、ここで `'Skill'` をリストするのではなく `skills` フィールドを使用してください                                                           |
| `disallowedTools`                     | いいえ | このエージェントで明示的に許可しないツール名の配列。MCP サーバーレベルのパターンも受け入れられます: `mcp__server` または `mcp__server__*` はそのサーバーからすべてのツールを削除し、`mcp__*` はすべての MCP ツールを任意のサーバーから削除します                                                                        |
| `prompt`                              | はい  | エージェントのシステムプロンプト                                                                                                                                                                                                          |
| `model`                               | いいえ | このエージェントのモデル上書き。`'fable'`、`'opus'`、`'sonnet'`、`'haiku'`、`'inherit'` などのエイリアスまたは完全なモデル ID を受け入れます。`'inherit'` はメインモデルを使用します。省略した場合、Claude Code は[サブエージェントモデル順序](/docs/ja/sub-agents#choose-a-model)でモデルを選択します                   |
| `mcpServers`                          | いいえ | このエージェント用の MCP サーバー仕様                                                                                                                                                                                                     |
| `skills`                              | いいえ | エージェントコンテキストにプリロードするスキル名の配列                                                                                                                                                                                               |
| `initialPrompt`                       | いいえ | このエージェントがメインスレッドエージェントとして実行される場合、最初のユーザーターンとして自動送信されます                                                                                                                                                                    |
| `maxTurns`                            | いいえ | 停止する前のエージェンティックターン数（API ラウンドトリップ）の最大数                                                                                                                                                                                     |
| `background`                          | いいえ | 呼び出されたときにこのエージェントをノンブロッキングバックグラウンドタスクとして実行します                                                                                                                                                                             |
| `omitClaudeMd`                        | いいえ | このエージェントがサブエージェントとして実行される場合、ユーザー、プロジェクト、ローカル CLAUDE.md ファイルなしでこのエージェントを実行します。管理ポリシーファイルは引き続き読み込まれます。Agent ツールプロンプトから必要なすべてを取得するエージェントに使用します。このエージェントがメインスレッドエージェントとして実行される場合は無視されます。TypeScript Agent SDK v0.3.271 以降が必要です |
| `memory`                              | いいえ | このエージェントのメモリソース: `'user'`、`'project'`、または `'local'`                                                                                                                                                                       |
| `effort`                              | いいえ | このエージェントの推論努力レベル。名前付きレベルまたは整数を受け入れます                                                                                                                                                                                      |
| `permissionMode`                      | いいえ | このエージェント内のツール実行の権限モード。[サブエージェント継承ルール](/docs/ja/agent-sdk/permissions#available-modes)は、それが適用される場合を決定します。[`PermissionMode`](#permissionmode) を参照してください                                                                          |
| `criticalSystemReminder_EXPERIMENTAL` | いいえ | 実験的: システムプロンプトに追加された重要なリマインダー                                                                                                                                                                                             |

<h3 id="agentmcpserverspec">
  `AgentMcpServerSpec`
</h3>

サブエージェントで利用可能な MCP サーバーを指定します。サーバー名（親の `mcpServers` 設定からサーバーを参照する文字列）またはインラインサーバー設定レコード（サーバー名を設定にマップ）です。

```typescript theme={null}
type AgentMcpServerSpec = string | Record<string, McpServerConfigForProcessTransport>;
```

ここで `McpServerConfigForProcessTransport` は `McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig` です。

<h3 id="settingsource">
  `SettingSource`
</h3>

SDK が設定を読み込むファイルシステムベースの設定ソースを制御します。

```typescript theme={null}
type SettingSource = "user" | "project" | "local";
```

| 値           | 説明                                                 | 場所                            |
| :---------- | :------------------------------------------------- | :---------------------------- |
| `'user'`    | グローバルユーザー設定                                        | `~/.claude/settings.json`     |
| `'project'` | 共有プロジェクト設定（バージョン管理）                                | `.claude/settings.json`       |
| `'local'`   | ローカルプロジェクト設定、Claude Code が設定をそれに保存するときに gitignored | `.claude/settings.local.json` |

<h4 id="default-behavior">
  デフォルト動作
</h4>

`settingSources` が省略されるか `undefined` の場合、`query()` は Claude Code CLI と同じファイルシステム設定を読み込みます。ユーザー、プロジェクト、ローカル。[settingSources が制御しないもの](/docs/ja/agent-sdk/claude-code-features#what-settingsources-does-not-control)を参照して、それらを読み込む入力と、それらを無効にする方法を確認してください。

<h4 id="why-use-settingsources">
  settingSources を使用する理由
</h4>

**ファイルシステム設定を無効にします:**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Do not load user, project, or local settings from disk
const result = query({
  prompt: "Analyze this code",
  options: { settingSources: [] }
});
```

**特定の設定ソースのみを読み込みます:**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Load only project settings, ignore user and local
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ["project"] // Only .claude/settings.json
  }
});
```

CLAUDE.md プロジェクト指示を読み込むには、`settingSources` に `"project"` を含めます。CLAUDE.md 読み込みがシステムプロンプトオプションとどのように相互作用するかについては、[システムプロンプトを変更](/docs/ja/agent-sdk/modifying-system-prompts#claude-md-files-for-project-level-instructions)を参照してください。

<h4 id="settings-precedence">
  設定優先順位
</h4>

複数のソースが読み込まれる場合、設定はこの優先順位（最高から最低）でマージされます:

1. ローカル設定（`.claude/settings.local.json`）
2. プロジェクト設定（`.claude/settings.json`）
3. ユーザー設定（`~/.claude/settings.json`）

`agents`、`allowedTools`、`settings` などのプログラムオプションは、ユーザー、プロジェクト、ローカルファイルシステム設定をオーバーライドします。管理ポリシー設定はプログラムオプションより優先されます。

<h3 id="permissionmode">
  `PermissionMode`
</h3>

```typescript theme={null}
type PermissionMode =
  | "default" // Standard permission behavior
  | "acceptEdits" // Auto-accept file edits
  | "bypassPermissions" // Bypass permission checks; explicit ask rules still prompt
  | "plan" // Planning mode - explore without editing
  | "dontAsk" // Don't prompt for permissions, deny if not pre-approved
  | "auto"; // Model classifier approves or denies permission prompts
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

ツール使用を制御するためのカスタム権限関数型。

関数は対話的権限プロンプトの SDK 置き換えです。[権限評価フロー](/docs/ja/agent-sdk/permissions#how-permissions-are-evaluated)がプロンプトに解決する場合にのみ呼び出されます。`allowedTools` エントリ、設定許可ルール、または `acceptEdits` や `bypassPermissions` などの権限モードで既に承認されたツール呼び出しは、それを呼び出しません。すべてのツール呼び出しをゲートするには、代わりに [`PreToolUse` フック](/docs/ja/agent-sdk/hooks)を使用してください。

許可ルールは[どのモードも自動承認しないアクション](/docs/ja/permission-modes#actions-no-mode-auto-approves)を事前承認しません。[権限がどのように評価されるか](/docs/ja/agent-sdk/permissions#how-permissions-are-evaluated)を参照して、どれがコールバックに到達し、`dontAsk` および `auto` モードで何が起こるかを確認してください。

```typescript theme={null}
type CanUseTool = (
  toolName: string,
  input: Record<string, unknown>,
  options: {
    signal: AbortSignal;
    suggestions?: PermissionUpdate[];
    blockedPath?: string;
    mcpServer?: { name: string; source: string };
    decisionReason?: string;
    toolUseID: string;
    agentID?: string;
    requestId: string;
  }
) => Promise<PermissionResult | null>;
```

| オプション            | 型                                           | 説明                                                                                                                                                                                                 |
| :--------------- | :------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`         | `AbortSignal`                               | 操作を中止する必要がある場合にシグナルされます                                                                                                                                                                            |
| `suggestions`    | [`PermissionUpdate`](#permissionupdate)`[]` | 提案された権限更新。ユーザーがこのツールに再度プロンプトされないようにします。Bash プロンプトは `localSettings` [宛先](#permissionupdatedestination)を含む提案を含むため、`updatedPermissions` で返すと、ルールを `.claude/settings.local.json` に書き込み、セッション全体で永続化します。 |
| `blockedPath`    | `string`                                    | 該当する場合、権限リクエストをトリガーしたファイルパス                                                                                                                                                                        |
| `mcpServer`      | `{ name: string; source: string }`          | `mcp__*` ツールの場合、そのツールを提供する MCP サーバーと、そのサーバーの定義がどこから来たかを示す [`McpServerProvenance`](#mcpserverprovenance) のフィールド。他のツールの場合は不在。Agent SDK v0.3.274 以降が必要です                                              |
| `decisionReason` | `string`                                    | この権限リクエストがトリガーされた理由を説明します                                                                                                                                                                          |
| `toolUseID`      | `string`                                    | アシスタントメッセージ内のこの特定のツール呼び出しの一意の識別子                                                                                                                                                                   |
| `agentID`        | `string`                                    | サブエージェント内で実行している場合、サブエージェントの ID                                                                                                                                                                    |
| `requestId`      | `string`                                    | `control_request` エンベロープの `request_id`。アプリケーションが SDK の外で送信する `control_response`（署名付き HTTP POST など）は、Claude Code プロセスが返信をリクエストと一致させることができるようにこの値をエコーする必要があります                                       |

コールバックは通常、[`PermissionResult`](#permissionresult) を返すことでリクエストを解決します。これは SDK がそのトランスポートを通じて `control_response` として書き込みます。アプリケーションが既にこのリクエストの `control_response` を独自のチャネルを通じて送信した場合にのみ `null` を返します。`requestId` をエコーします。SDK はその後、トランスポートへのレスポンス書き込みをスキップします。他の場合に `null` を返すと、`control_response` が送信されず、権限プロンプトはタイムアウトしないため、ツール呼び出しは無期限にブロックされたままになります。

`requestId` オプションと `null` 戻り値には Claude Code v2.1.199 以降が必要です。

<h3 id="permissionresult">
  `PermissionResult`
</h3>

権限チェックの結果。

```typescript theme={null}
type PermissionResult =
  | {
      behavior: "allow";
      updatedInput?: Record<string, unknown>;
      updatedPermissions?: PermissionUpdate[];
      toolUseID?: string;
    }
  | {
      behavior: "deny";
      message: string;
      interrupt?: boolean;
      toolUseID?: string;
    };
```

<h3 id="toolconfig">
  `ToolConfig`
</h3>

組み込みツール動作の設定。

```typescript theme={null}
type ToolConfig = {
  askUserQuestion?: {
    previewFormat?: "markdown" | "html";
  };
};
```

| フィールド                           | 型                      | 説明                                                                                                                                          |
| :------------------------------ | :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| `askUserQuestion.previewFormat` | `'markdown' \| 'html'` | [`AskUserQuestion`](/docs/ja/agent-sdk/user-input#question-format) オプションの `preview` フィールドをオプトインし、そのコンテンツ形式を設定します。設定されていない場合、Claude はプレビューを出力しません |

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

MCP サーバーの設定。

```typescript theme={null}
type McpServerConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfigWithInstance;
```

<h4 id="mcpstdioserverconfig">
  `McpStdioServerConfig`
</h4>

```typescript theme={null}
type McpStdioServerConfig = {
  type?: "stdio";
  command: string;
  args?: string[];
  env?: Record<string, string>;
};
```

<h4 id="mcpsseserverconfig">
  `McpSSEServerConfig`
</h4>

```typescript theme={null}
type McpSSEServerConfig = {
  type: "sse";
  url: string;
  headers?: Record<string, string>;
};
```

<h4 id="mcphttpserverconfig">
  `McpHttpServerConfig`
</h4>

```typescript theme={null}
type McpHttpServerConfig = {
  type: "http";
  url: string;
  headers?: Record<string, string>;
};
```

<h4 id="mcpsdkserverconfigwithinstance">
  `McpSdkServerConfigWithInstance`
</h4>

```typescript theme={null}
type McpSdkServerConfigWithInstance = {
  type: "sdk";
  name: string;
  timeout?: number;
  instance: McpServer;
};
```

<h4 id="mcpclaudeaiproxyserverconfig">
  `McpClaudeAIProxyServerConfig`
</h4>

```typescript theme={null}
type McpClaudeAIProxyServerConfig = {
  type: "claudeai-proxy";
  url: string;
  id: string;
};
```

<h3 id="sdkpluginconfig">
  `SdkPluginConfig`
</h3>

SDK でプラグインを読み込むための設定。

```typescript theme={null}
type SdkPluginConfig = {
  type: "local";
  path: string;
  skipMcpDiscovery?: boolean;
};
```

| フィールド              | 型         | 説明                                                                                                                                         |
| :----------------- | :-------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| `type`             | `'local'` | `'local'` である必要があります（現在ローカルプラグインのみがサポートされています）                                                                                             |
| `path`             | `string`  | プラグインディレクトリへの絶対パスまたは相対パス                                                                                                                   |
| `skipMcpDiscovery` | `boolean` | `true` の場合、SDK はこのプラグインからスキル、フック、エージェント、コマンドを読み込みますが、その `.mcp.json` またはマニフェスト `mcpServers` は読み込みません。アプリケーションがプラグインの MCP 接続を所有している場合に設定します。 |

**例:**

```typescript theme={null}
plugins: [
  { type: "local", path: "./my-plugin" },
  { type: "local", path: "/absolute/path/to/plugin" }
];
```

プラグインの作成と使用に関する完全な情報については、[プラグイン](/docs/ja/agent-sdk/plugins)を参照してください。

<h2 id="message-types">
  メッセージ型
</h2>

<h3 id="sdkmessage">
  `SDKMessage`
</h3>

クエリによって返されるすべての可能なメッセージの共用体型。

```typescript theme={null}
type SDKMessage =
  | SDKAssistantMessage
  | SDKUserMessage
  | SDKUserMessageReplay
  | SDKResultMessage
  | SDKSystemMessage
  | SDKPartialAssistantMessage
  | SDKCompactBoundaryMessage
  | SDKStatusMessage
  | SDKLocalCommandOutputMessage
  | SDKHookStartedMessage
  | SDKHookProgressMessage
  | SDKHookResponseMessage
  | SDKPluginInstallMessage
  | SDKToolProgressMessage
  | SDKAuthStatusMessage
  | SDKTaskNotificationMessage
  | SDKTaskStartedMessage
  | SDKTaskProgressMessage
  | SDKTaskUpdatedMessage
  | SDKBackgroundTasksChangedMessage
  | SDKThinkingTokensMessage
  | SDKSessionStateChangedMessage
  | SDKWorkerShuttingDownMessage
  | SDKCommandsChangedMessage
  | SDKNotificationMessage
  | SDKFilesPersistedEvent
  | SDKToolUseSummaryMessage
  | SDKMemoryRecallMessage
  | SDKRateLimitEvent
  | SDKElicitationCompleteMessage
  | SDKPermissionDeniedMessage
  | SDKPromptSuggestionMessage
  | SDKAPIRetryMessage
  | SDKMirrorErrorMessage
  | SDKInformationalMessage
  | SDKConversationResetMessage;
```

<h3 id="sdkassistantmessage">
  `SDKAssistantMessage`
</h3>

アシスタント応答メッセージ。

```typescript theme={null}
type SDKAssistantMessage = {
  type: "assistant";
  uuid: UUID;
  session_id: string;
  message: BetaMessage; // Anthropic SDK から
  parent_tool_use_id: string | null;
  error?: SDKAssistantMessageError;
  aborted?: true;
  timestamp?: string;
  context_usage?: SDKContextUsage;
  user_message_uuid?: string;
  user_message_uuids?: string[];
};
```

`message` フィールドは Anthropic SDK の [`BetaMessage`](https://platform.claude.com/docs/ja/api/messages/create) です。`id`、`content`、`model`、`stop_reason`、`usage` などのフィールドを含みます。

`SDKAssistantMessageError` は以下のいずれかです：`'authentication_failed'`、`'oauth_org_not_allowed'`、`'account_on_hold'`、`'billing_error'`、`'rate_limit'`、`'overloaded'`、`'invalid_request'`、`'model_not_found'`、`'server_error'`、`'max_output_tokens'`、`'cloud_credential_error'`、または `'unknown'`。これらの値のうち 4 つは、その名前が示すもの以上の意味を持ちます：

* `'model_not_found'`：選択されたモデルが存在しないか、アカウントまたはデプロイメントで利用できません
* `'overloaded'`：API がサーバーが容量に達しているため 529 を返しました。これは `'rate_limit'` とは異なり、クォータに対する 429 です
* `'account_on_hold'`：[アカウントが保留中です](/docs/ja/errors#your-account-is-on-hold)
* `'cloud_credential_error'`：Claude Code が実行されているマシンで使用可能な AWS または Google Cloud 認証情報を取得できなかったため、クラウドプロバイダーにリクエストが到達しませんでした。通常の原因は、そのマシンで期限切れまたは完了していないクラウドサインインですが、一時的に到達不可能な認証情報サービスも同じ値を報告します。[AWS または Google Cloud 認証情報を読み込めませんでした](/docs/ja/errors#could-not-load-aws-or-google-cloud-credentials)を参照してください。TypeScript Agent SDK v0.3.267 以降が必要です。これは Claude Code v2.1.267 をバンドルしています

`aborted` は、割り込みまたは中止がアシスタントメッセージをストリーム完了前に切り詰めたときに `true` です：メッセージに `stop_reason` がなく、コンテンツは単語の途中で終わる可能性があります。このフィールドは通常完了したメッセージには存在しません。Agent SDK v0.3.214 以降が必要です。

Claude Code は、[`user_message_uuid`](#user_message_uuid) の条件下で、ターンの最初のアシスタントメッセージに `user_message_uuid` と `user_message_uuids` を設定します。

`timestamp` は、メッセージのコンテンツがそれを生成したプロセスで生成を完了した ISO 8601 時刻です。値はそのマシンのクロックから取得されるため、表示用にのみ使用し、メッセージを順序付けないでください。1 つの API ターンは、同じ `message.id` を共有する複数のアシスタントメッセージを生成でき、それぞれが独自の `timestamp` を持ちます。フィールドが存在しない場合は、メッセージを受け取った時刻にフォールバックします。

`context_usage` は `/context` レポートの構造化コピーで、[`SDKContextUsage`](#sdkcontextusage) として型付けされ、Agent SDK v0.3.232 以降が必要です。プロンプトとして `/context` を送信すると、Claude Code はレポートを `message.content` がマークダウンテーブルを保持するアシスタントメッセージとして配信し、`context_usage` をその同じメッセージに添付します。Claude Code はこのフィールドを他のアシスタントメッセージに設定せず、以前のバージョンは `/context` テーブルをそれなしで配信するため、フィールドが存在するときは分析から読み取り、存在しないときはマークダウンテキストにフォールバックします。

<h3 id="sdkusermessage">
  `SDKUserMessage`
</h3>

ユーザー入力メッセージ。

```typescript theme={null}
type SDKUserMessage = {
  type: "user";
  uuid?: UUID;
  session_id?: string;
  message: MessageParam; // Anthropic SDK から
  pasted_content?: MessageParam["content"][];
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  shouldQuery?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
  inline_pastes?: string[];
};
```

ユーザーが入力したのではなく、プロンプト UI に貼り付けたコンテンツを送信するには `pasted_content` を設定します。貼り付けごとに 1 つのエントリ。各エントリは文字列またはコンテンツブロックの配列です。Claude Code は各エントリのテキストを入力されたテキストの後に追加し、各貼り付けを `<pasted_content>` タグで囲む可能性があります。テキスト以外のブロックは無視されるため、画像とドキュメントを `message.content` で送信します。Agent SDK v0.3.277 以降が必要です。

`shouldQuery` を `false` に設定して、アシスタントターンをトリガーせずにメッセージをトランスクリプトに追加します。メッセージは保持され、ターンをトリガーする次のユーザーメッセージにマージされます。これを使用して、バンド外で実行したコマンドの出力など、モデル呼び出しを費やさずにコンテキストを注入します。

`tool_result` ブロックを持つメッセージでは、`tool_use_result` はモデルに送信されたテキストではなく、ツールの構造化出力オブジェクトです。その形状は、対応する `tool_use` ブロックで指定されたツールに依存するため、フィールドは `unknown` として型付けされます。組み込みの形状は [ツール出力型](#tool-output-types) の下にリストされています。

`Agent` ツールの場合、`tool_use_result` は [`AgentOutput`](#agent-2) です。`completed` 結果では、`content` はサブエージェントのレポートを保持し、Claude Code が `tool_result` テキストに追加するエージェント ID と使用状況トレーラーは含まれません。そのため、そのテキストを解析する代わりに `tool_use_result` からレンダリングします。

結果に `resource_link` ブロックを含む MCP ツールの場合、`tool_use_result` は [`SDKMcpResourceLink`](#sdkmcpresourcelink) エントリの `resourceLinks` 配列を持つオブジェクトです。Claude は各リンクを `tool_result` ブロック内のテキスト行として受け取るため、そのテキストを解析する代わりに `resourceLinks` を読んでサーバーが返したファイルをレンダリングします。Claude Code は結果にリンクがない場合、サブエージェントからの結果で `resourceLinks` を省略し、結果ごとに最大 50 リンクを保持し、配列が 64 KiB のシリアル化 JSON に達するとリンクの追加を停止します。`resourceLinks` には Agent SDK v0.3.257 以降が必要です。

ユーザーが貼り付けたのではなく入力した `message.content` のどの部分かを Claude Code に伝えるには `inline_pastes` を設定します。貼り付けごとに 1 つの文字列。プロンプトテキストはユーザーが配置した場所に留まります。Claude Code は各リストされた貼り付けを `<pasted_content>` タグで囲む可能性があります。そこに立つため、Claude は貼り付けられた素材をユーザー自身の言葉から区別できます。プロンプトの最後のテキストブロック内の貼り付けのみがラップされます。TypeScript Agent SDK v0.3.280 以降が必要です。

<h3 id="sdkusermessagereplay">
  `SDKUserMessageReplay`
</h3>

必須 UUID を含む再生されたユーザーメッセージ。

```typescript theme={null}
type SDKUserMessageReplay = {
  type: "user";
  uuid: UUID;
  session_id: string;
  message: MessageParam;
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
  isReplay: true;
};
```

セッション外から注入されたユーザーターン（[`origin`](#sdkmessageorigin) の種類が `peer` または `channel` であるもの）は、アクティブなターン中に配信されたか、セッションがアイドル状態の間に新しいターンを開始したかに関わらず、ストリームに再生として到達します。v2.1.207 より前では、セッションがアイドル状態の間に配信された注入されたターンはストリーム上にメッセージを生成せず、トランスクリプトを再読み込みするときにのみ表示されました。

<h3 id="sdkresultmessage">
  `SDKResultMessage`
</h3>

最終結果メッセージ。

```typescript theme={null}
type SDKResultMessage =
  | {
      type: "result";
      subtype: "success";
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      api_error_status?: number | null;
      num_turns: number;
      result: string;
      stop_reason: string | null;
      ttft_ms?: number;
      ttft_stream_ms?: number;
      user_message_uuid?: string;
      user_message_uuids?: string[];
      request_sent_wall_ms?: number;
      first_content_frame_ms?: number;
      first_stream_post_ms?: number;
      first_stream_post_ack_ms?: number;
      first_stream_post_wall_ms?: number;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      queued_turn_count?: number;
      structured_output?: unknown;
      deferred_tool_use?: { id: string; name: string; input: Record<string, unknown> };
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      fast_mode_disabled_reason?: FastModeDisabledReason;
      origin?: SDKMessageOrigin;
    }
  | {
      type: "result";
      subtype:
        | "error_max_turns"
        | "error_during_execution"
        | "error_max_budget_usd"
        | "error_max_structured_output_retries";
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      num_turns: number;
      stop_reason: string | null;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      queued_turn_count?: number;
      errors: string[];
      startup_failure_reason?: SDKStartupFailureReason;
      user_message_uuid?: string;
      user_message_uuids?: string[];
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      fast_mode_disabled_reason?: FastModeDisabledReason;
      origin?: SDKMessageOrigin;
    };
```

結果の複数のフィールドは、`subtype` を超えた診断詳細を提供します：

* `api_error_status`：会話を終了させた API エラーの HTTP ステータスコード。ターンが API エラーなしで終了した場合、存在しないか `null` です。
* `ttft_ms`：最初のトークンまでの時間（ミリ秒）。最初の完全なアシスタントメッセージが到着したときに測定されます。成功の場合のみ存在します。
* `ttft_stream_ms`：最初の `message_start` ストリームイベントまでの時間（ミリ秒）。レスポンスストリームが開くときです。`ttft_ms` より低く、2 つの間のギャップは最初のメッセージをストリーミングするのに費やされた時間です。成功の場合のみ存在します。
* `user_message_uuid`：このターンが答えたメッセージの `uuid`。[`user_message_uuid`](#user_message_uuid) を参照して、どの結果がそれを持つかを確認してください。
* `user_message_uuids`：Claude Code がこのターンで答えたすべてのメッセージの `uuid`。[`user_message_uuids`](#user_message_uuids) を参照してください。
* `request_sent_wall_ms`：Claude Code が API リクエストをディスパッチしたエポックミリ秒。サーバー側タイムスタンプとの結合用です。[`user_message_uuid`](#user_message_uuid) と一緒にのみ存在し、`is_error` が false で API リクエストを送信したターンの成功結果です。
* `first_content_frame_ms`：最初の `content_block_start` または `content_block_delta` ストリームイベントまでの時間（ミリ秒）。思考ブロックをコンテンツとしてカウントします。成功の場合のみ存在し、`is_error` が false のときです。Agent SDK v0.3.260 以降が必要です。
* `first_stream_post_ms`、`first_stream_post_ack_ms`、`first_stream_post_wall_ms`：ターンの最初のストリームイベントをアップロードするためのタイミング。Claude Code はそれを [クラウドセッション](/docs/ja/claude-code-on-the-web) などの claude.ai にストリーミングするセッションでのみ記録し、`query()` が生成する結果はそれらを持ちません。Agent SDK v0.3.260 以降が必要です。
* `usage`：メインエージェントループのみ。サブエージェントと補助モデル呼び出しを除外し、ストリーミング入力セッションではターンごとです。トークン/コスト会計には `modelUsage` を優先してください。
* `modelUsage`：この `query()` 呼び出し中にクエリパイプラインを通じて行われたすべてのモデル呼び出しのモデルごとの合計。メインループ、サブエージェント、圧縮や Workflow エージェントなどの内部呼び出しを含みます。権限分類器やトークンカウントリクエストなど、そのパイプラインの外側のヘルパー呼び出しは除外されます。セッションを再開する呼び出しは、[セッションの以前の呼び出しから復元されたモデルごとの合計](/docs/ja/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls) もカウントします。ストリーミング入力セッションでは、合計はターン全体で累積されるため、結果全体で合計するのではなく、最新の結果を読んでください。[ストリーミング入力モードでコストを追跡する](/docs/ja/agent-sdk/cost-tracking#track-costs-in-streaming-input-mode) でリセットを参照し、[セッションクラッシュ後に合計を復旧する](/docs/ja/agent-sdk/cost-tracking#recover-totals-after-a-session-crash) でゼロ化された結果を参照してください。
* `total_cost_usd`：この `query()` 呼び出しの累積推定コスト（USD）。`modelUsage` と同じ呼び出しをカバーし、同じポイントでリセットされます。セッションを再開する呼び出しは、[セッションの以前の呼び出しから復元された合計](/docs/ja/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls) もカウントします。これは推定値であり、請求書ではありません。[コストと使用状況を追跡する](/docs/ja/agent-sdk/cost-tracking) で精度に関する注意事項を参照してください。
* `queued_turn_count`：Claude Code が結果を生成したときに、`origin: { kind: "human" }` で送信したメッセージのうち、まだ待機中のメッセージの数。[`queued_turn_count`](#queued_turn_count) を参照して、`0` と不在フィールドが何を示すかを確認してください。
* `startup_failure_reason`：Claude Code が起動を拒否した理由。セッションが開始する前に結果を生成します。[`startup_failure_reason`](#startup_failure_reason) を参照して、値と、どの失敗がそれを持つかを確認してください。Agent SDK v0.3.274 以降が必要です。
* `terminal_reason`：ループが終了した理由。`"completed"`、`"max_turns"`、`"tool_deferred"`、`"aborted_streaming"`、`"aborted_tools"`、`"hook_stopped"`、`"stop_hook_prevented"`、`"background_requested"`、`"blocking_limit"`、`"rapid_refill_breaker"`、`"prompt_too_long"`、`"image_error"`、`"model_error"`、`"api_error"`、`"malformed_tool_use_exhausted"`、`"budget_exhausted"`、`"structured_output_retry_exhausted"`、`"tool_deferred_unavailable"`、または `"turn_setup_failed"` のいずれかです。
* `fast_mode_state`：`"on"`、`"off"`、または `"cooldown"` のいずれかです。
* `fast_mode_disabled_reason`：[高速モード](/docs/ja/fast-mode) が今利用できない理由。高速モードをブロックするものがない場合は不在ですが、リクエストは標準速度で実行される可能性があります。高速モードレート制限後のクールダウン中、Claude Code は `fast_mode_state: "cooldown"` を理由コードなしで報告し、クールダウンが期限切れになると高速モードを再度有効にします。Claude Code v2.1.219 以降が必要です。

理由コードを使用して、独自の UI で高速モードがオフになっている理由を説明し、可用性を再導出する代わりに説明します。各コードは高速モードをブロックしたチェックに名前を付けます：

| 理由コード                  | 意味                                                                                                                               |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `free`                 | アカウントが高速モードに必要な有料サブスクリプションまたは使用クレジットを持っていません                                                                                     |
| `preference`           | 組織が高速モードを無効にしました                                                                                                                 |
| `extra_usage_disabled` | 使用クレジットがアカウントに対してオフになっています                                                                                                       |
| `network_error`        | [可用性チェック](/docs/ja/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) が `api.anthropic.com` に到達できませんでした                         |
| `unknown`              | Claude Code は可用性を判断できませんでした                                                                                                      |
| `not_first_party`      | セッションは Anthropic API 以外のプロバイダーを使用しています                                                                                           |
| `disabled_by_env`      | [`CLAUDE_CODE_DISABLE_FAST_MODE`](/docs/ja/env-vars) が設定されています                                                                        |
| `model_not_allowed`    | 高速モード Opus モデルが組織の [`availableModels`](/docs/ja/model-config#restrict-model-selection) 許可リストにありません                                    |
| `sdk_opt_in_required`  | セッションが高速モードにオプトインしていません：[`settings`](#options) オプションで `fastMode: true` を渡すか、[`applyFlagSettings()`](#applyflagsettings) を通じて渡します |
| `pending`              | 可用性チェックがまだ完了していません                                                                                                               |

同じフィールドペアが [`SDKSystemMessage`](#sdksystemmessage) と [`SDKControlInitializeResponse`](#sdkcontrolinitializeresponse) に表示されるため、最初のターンの前に高速モード状態を読むことができます。

`origin` フィールドは、この結果をトリガーしたユーザーメッセージの [`SDKMessageOrigin`](#sdkmessageorigin) を転送します。SDK が完了したバックグラウンドタスクなどの合成フォローアップターンを注入する場合、結果の `SDKResultMessage` は `origin: { kind: "task-notification" }` を持ちます。トリガーが発火し、サーバーが検証したメッセージが他のセッションから到着するルーチンも、[タスク通知サブキンド](#task-notification-subkinds) で説明されている `subkind` を持つこのキンドで到着します。`kind` をチェックして、プロンプトに答える結果と注入されたフォローアップを区別してから、ルーティングまたは抑制します。アプリケーションが [スケジュール済み実行を宣言](#declare-a-scheduled-run) する場合、それらの結果も `kind: "task-notification"` を持つため、`kind` だけで抑制しないでください。

複数のバックグラウンドタスク完了が一緒にキューに入れられている場合、Claude Code はそれらを 1 つのターンで答えることができます。各完了は依然として独自の結果を生成します。Claude Code が一緒に答える完了のうち、最後のもの以外はすべて、`num_turns: 0` の空の結果を順番に生成し、最後のものの結果はそれらすべてに答えるターンを持ちます。

フィールドは、スタートアップエラーなど、ユーザーターンの前に発行される結果には存在しません。

`PreToolUse` フックが `permissionDecision: "defer"` を返すと、結果は `stop_reason: "tool_deferred"` を持ち、`deferred_tool_use` は保留中のツールの `id`、`name`、`input` を持ちます。このフィールドを読んで、独自の UI でリクエストをサーフェスし、同じ `session_id` で再開して続行します。[ツール呼び出しを後で延期する](/docs/ja/hooks#defer-a-tool-call-for-later) で完全なラウンドトリップを参照してください。

<h4 id="user_message_uuid">
  `user_message_uuid`
</h4>

このターンが答えている [`SDKUserMessage`](#sdkusermessage) の `uuid`。Claude Code の返信をメッセージに一致させるために、メッセージに設定した場合にのみ、Claude Code は `uuid` をエコーバックします。フィールドは `SDKUserMessage` では省略可能で、`query()` に渡された文字列プロンプトは何も持ちません。

ターンが答えるメッセージは、ターンの開始方法によって異なります：

* **送信した通常のメッセージ**（`isSynthetic: true` なし）：ターンはその実行全体でそのメッセージに答えます。複数のメッセージを一緒に送信すると、Claude Code はそれらを 1 つのターンにマージでき、フィールドはそれらの最後のメッセージの `uuid` のみを持ちます。マージされたメッセージのいずれかへの返信を一致させるには、[`user_message_uuids`](#user_message_uuids) を使用してください。
* **`isSynthetic: true` で送信したメッセージ**：ターンは最初そのメッセージに答えます。Claude Code がツール呼び出し間でメッセージを拾う場合、ターンはそれ以降、拾われたメッセージに答えます。合成メッセージの `uuid` をエコーバックするには Agent SDK v0.3.265 以降が必要です。以前のバージョンは合成ターンで何もエコーバックしません。
* **Claude Code が自身で生成したプロンプト**（セッション再開後に中断された作業を続行するターンなど）：ターンは最初メッセージに答えず、フレームはエコーを持ちません。Claude Code がツール呼び出し間でメッセージを拾う場合、ターンはそれ以降、そのメッセージに答えます。ピックアップエコーには Agent SDK v0.3.265 以降が必要です。以前のバージョンはこれらのターンで何もエコーバックしません。

Claude Code は、3 種類のフレームで答えられたメッセージの `uuid` をエコーバックします：

* **結果**：メッセージに答えたターンのすべての結果。Agent SDK v0.3.265 以降ではすべてのそのような結果がそれを持ちます。v0.3.265 より前では、通常のメッセージが開始したターンの成功結果は、ターンが API リクエストを送信しなかったか、延期されたツール呼び出しで終了したときにそれを欠いていました。v0.3.246 より前では、エラー結果もそれを欠いていました。v0.3.216 より前ではすべての結果がそれを欠いていました。
* **ターンの最初の返信**：最初の [アシスタントメッセージ](#sdkassistantmessage)、または `includePartialMessages` を使用して、`event.type` が `ping` ではない最初の [ストリームイベント](#sdkpartialassistantmessage)。結果が到着する前に返信をバインドできます。ターンが何もストリーミングしない場合、Claude Code は代わりに最初のアシスタントメッセージに設定します。最初の返信エコーには Agent SDK v0.3.246 以降が必要です。ターンが答えているメッセージが途中で変わる場合、変更後の最初の返信もフィールドを持ちます。Agent SDK v0.3.265 以降です。以前のバージョンはターンごとに 1 つの返信フレームに設定します。
* **ターンのすべての [`thinking_tokens`](#sdkthinkingtokensmessage) フレーム**：ターンの最初の返信を待たずに、送信したメッセージに思考進捗を属性付けできます。Agent SDK v0.3.260 以降が必要です。

Claude Code はこれらの場合にフィールドを省略します：

* 最初の返信以外の返信フレーム
* サブエージェントフレーム
* `uuid` を持つメッセージに答えないターン：ターンは `uuid` なしで送信したメッセージに答えたか、Claude Code がターンを開始し、`uuid` を持つ通常のメッセージを拾いませんでした
* 送信したメッセージに答えない結果（クラッシュしたワーカープロセス後のゼロ化された結果など）

<h4 id="user_message_uuids">
  `user_message_uuids`
</h4>

Claude Code がこのターンで答えたすべてのメッセージの `uuid`。複数のメッセージを一緒に送信すると、Claude Code はそれらを 1 つのターンにマージでき、`user_message_uuid` はそれらの最後のメッセージのみに名前を付けます。マージされたメッセージのいずれかへの返信を一致させるには、このリストのどこかでそのメッセージの `uuid` を探してください。Agent SDK v0.3.259 以降が必要です。

Claude Code は、そのフィールドを持つ各返信フレームと結果で、`user_message_uuid` と一緒にリストを設定します。`user_message_uuid` を持つすべてのフレームについては、[`user_message_uuid`](#user_message_uuid) を参照してください。各フレームが必要とするバージョンについても参照してください。リストは常に `user_message_uuid` を含み、最大 64 エントリを保持します。

Claude Code がターン実行中に送信した通常のメッセージを拾う場合、そのメッセージの `uuid` を結果のリストに追加します。

最初の返信または結果が `user_message_uuid` をリストなしで持つ場合、それは以前の Claude Code バージョンから来たため、単一フィールドにフォールバックしてください。

<h4 id="queued_turn_count">
  `queued_turn_count`
</h4>

Claude Code が結果を生成したときに、[`origin: { kind: "human" }`](#sdkmessageorigin) で送信したメッセージのうち、コマンドキューで待機中のメッセージの数。Agent SDK v0.3.242 以降が必要です。

`0` と不在フィールドが示すもの：

* **`0`**：Claude Code はその `origin` なしで送信したメッセージをカウントせず、タスク通知もカウントしないため、ターンは依然として続く可能性があります。
* **不在**：Claude Code がクラッシュまたは致命的なスタートアップエラーの後に発行する最終結果はフィールドを省略し、[ゼロ化された合計を持つ可能性があります](/docs/ja/agent-sdk/cost-tracking#recover-totals-after-a-session-crash)。

<h4 id="startup_failure_reason">
  `startup_failure_reason`
</h4>

Claude Code が起動を拒否した理由。アプリケーションが再試行の代わりに修正を提供できるようにします。Claude Code は既知のスタートアップ失敗で終了する前に書く `error_during_execution` 結果に設定します。その結果はゼロ化された合計を持ち、その `errors` 配列は stderr と同じテキストを持ちます。フィールドは他のすべての結果に存在しません。Agent SDK v0.3.274 以降が必要です。

[`env`](#options) で `CLAUDE_CODE_STARTUP_FAILURE_RESULTS` を `1` に設定して、すべての `SDKStartupFailureReason` 値に対してこの結果を受け取ります。その変数がない場合、Claude Code はこれらの失敗に対してのみ結果を書き、残りは stderr 出力、ゼロ以外の終了、および結果メッセージなしで終了します：

* Claude Code が [ワークツリーにセッションを返すことができない](/docs/ja/worktrees#the-session-resumes-outside-its-worktree) ため停止する再開。`worktree_unverified` または `worktree_resume_refused`。そのセクションはどのエラーがどの値を持つかを示します。
* バックグラウンドセッションが保持する会話の拒否された [`continue`](#options)。`session_held_by_background`。そのような会話の拒否された [`resume`](#options) については、Claude Code は変数が設定されている場合にのみ結果を書きます。

```typescript theme={null}
type SDKStartupFailureReason =
  | "org_pin_api_key_conflict"
  | "org_verify_failed"
  | "org_pin_mismatch"
  | "managed_settings_invalid"
  | "remote_settings_required_unavailable"
  | "gateway_signin_required"
  | "gateway_access_denied"
  | "proxy_invalid"
  | "temp_dir_unusable"
  | "cwd_unavailable"
  | "shell_tool_missing"
  | "session_held_by_background"
  | "worktree_resume_refused"
  | "worktree_unverified"
  | "cli_version_too_old"
  | "bypass_root";
```

各値は 1 つの拒否に名前を付けます：

| 値                                      | セッションを停止したもの                                                                                                                                               |
| :------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `org_pin_api_key_conflict`             | 管理設定が [ファーストパーティまたはクラウドゲートウェイサインイン](/docs/ja/authentication#restrict-login-to-your-organization) を必要とし、Anthropic API キー、認証トークン、または `apiKeyHelper` が代わりに設定されています |
| `org_verify_failed`                    | サインインの組織をピンに対して検証できませんでした。例えば、ネットワーク障害または失効したトークンのため                                                                                                       |
| `org_pin_mismatch`                     | サインインはピンが許可しない組織に属しています                                                                                                                                    |
| `managed_settings_invalid`             | 管理ポリシー設定を読み込めなかったか、ピンが組織に名前を付けていません                                                                                                                        |
| `remote_settings_required_unavailable` | 組織が必要とする管理設定を読み込めませんでした                                                                                                                                    |
| `gateway_signin_required`              | [クラウドゲートウェイ](/docs/ja/claude-apps-gateway) がこのサインインを終了しました                                                                                                      |
| `gateway_access_denied`                | クラウドゲートウェイへの管理設定リクエストが 403 で返されました。ゲートウェイの [トラブルシューティングテーブル](/docs/ja/claude-apps-gateway-deploy#troubleshooting) がカバーしています                                     |
| `proxy_invalid`                        | プロキシ設定が完全な URL ではありません                                                                                                                                     |
| `temp_dir_unusable`                    | ユーザーごとの一時ディレクトリが安全でないか、作成できませんでした                                                                                                                          |
| `cwd_unavailable`                      | 作業ディレクトリが削除、移動、または読み込めません                                                                                                                                  |
| `shell_tool_missing`                   | Windows では、シェルツールが利用できません：Git Bash が見つからず、PowerShell が見つからないか `CLAUDE_CODE_USE_POWERSHELL_TOOL` でオフになっています                                                 |
| `session_held_by_background`           | 再開または続行する会話が [バックグラウンドセッション](/docs/ja/agent-view) として実行されています                                                                                                   |
| `worktree_resume_refused`              | セッションのワークツリーが安全チェックに失敗したか、再開がその内部から起動されました。`errors` は同じ再開を再度実行するとワークツリーなしで続行するかどうかを示します                                                                    |
| `worktree_unverified`                  | セッションのワークツリーを今検証できず、再試行が成功する可能性があります                                                                                                                       |
| `cli_version_too_old`                  | この Claude Code バージョンは Anthropic が必要とする最小値より下です                                                                                                             |
| `bypass_root`                          | バイパス権限モードが root として実行中にリクエストされました                                                                                                                          |

<h3 id="sdksystemmessage">
  `SDKSystemMessage`
</h3>

システム初期化メッセージ。

```typescript theme={null}
type SDKSystemMessage = {
  type: "system";
  subtype: "init";
  uuid: UUID;
  session_id: string;
  agents?: string[];
  apiKeySource: ApiKeySource;
  betas?: string[];
  claude_code_version: string;
  cwd: string;
  tools: string[];
  mcp_servers: {
    name: string;
    status: string;
    source?: string;
  }[];
  model: string;
  permissionMode: PermissionMode;
  slash_commands: string[];
  terminal_slash_commands?: string[];
  output_style: string;
  skills: string[];
  plugins: { name: string; path: string }[];
  fast_mode_state?: FastModeState;
  fast_mode_disabled_reason?: FastModeDisabledReason;
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | null;
  capabilities?: string[];
};
```

`fast_mode_state` はセッションの [高速モード](/docs/ja/fast-mode) 状態を報告します。何かが高速モードをブロックする場合、`fast_mode_disabled_reason` はそれをブロックしたチェックに名前を付けます。フィールドには Claude Code v2.1.219 以降が必要です。理由コードとその意味については、結果メッセージの [`fast_mode_disabled_reason`](#sdkresultmessage) を参照してください。

`terminal_slash_commands` は、`exit` などのローカルターミナルにバインドされたインターフェースを持つ `slash_commands` のエントリに名前を付けます。他のエントリと同じように送信できます。フィールドは、リモートまたはモバイルクライアントがコマンドメニューから非表示にできるように存在します。フィールドは空でない場合にのみ存在し、Agent SDK v0.3.229 以降が必要です。

* 各 `mcp_servers` エントリの `source`：サーバー定義がどこから来たか。[`McpServerStatus`](#mcpserverstatus) の `source` と同じ値を持ちます。Agent SDK v0.3.274 以降が必要です。
* `effort`：[努力レベル](/docs/ja/model-config#adjust-effort-level) Claude Code がセッションの次のリクエストで送信するか、何も送信しない場合は `null`。Claude Code は [Remote Control](/docs/ja/remote-control) クライアントに送信する初期化メッセージでのみフィールドを設定し、アプリケーションが読む初期化メッセージから省略します。Agent SDK v0.3.234 以降が必要です。

`capabilities` 配列は、この CLI が実装するプロトコル動作に名前を付けるため、`claude_code_version` 文字列を比較する代わりに機能検出を行うことができます。これはオープンセットです：認識しない値は無視し、依存する特定の動作の機能をチェックしてください。フィールドには Claude Code v2.1.205 以降が必要で、以前の CLI では存在しません。

| 機能                           | 意味                                                                                                                                                                                                                        |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt_receipt_v1`       | [`interrupt()`](#query-object) は、割り込みが到着したときに保留中だったメッセージをリストする [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) レシートで解決します                                                                                   |
| `interrupt_cancel_queued_v1` | `interrupt` 制御リクエストは `cancel_queued: true` を尊重し、レシートが `still_queued` の下にリストするメッセージをキャンセルし、代わりに `cancelled` の下にリストします。[`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) を参照してください。Claude Code v2.1.219 以降が必要です |

<h3 id="sdkpartialassistantmessage">
  `SDKPartialAssistantMessage`
</h3>

ストリーミング部分メッセージ（`includePartialMessages` が true の場合のみ）。`parent_tool_use_id` フィールドは常に `null` です：ストリームイベントはメインセッションのみに対して発行されます。サブエージェント属性については、`parent_tool_use_id` を持つ完全なメッセージを使用するか、[`forwardSubagentText`](#options) を有効にして、サブエージェントテキストと思考を完全なメッセージとして受け取ります。

```typescript theme={null}
type SDKPartialAssistantMessage = {
  type: "stream_event";
  event: BetaRawMessageStreamEvent; // Anthropic SDK から
  parent_tool_use_id: string | null;
  uuid: UUID;
  session_id: string;
  ttft_ms?: number; // 最初のトークンまでの時間（ミリ秒）。message_start イベントにのみ存在
  user_message_uuid?: string;
  user_message_uuids?: string[];
};
```

Claude Code は、ターンの最初の非 ping ストリームイベントで `user_message_uuid` と `user_message_uuids` を設定し、[`user_message_uuid`](#user_message_uuid) の条件下でターンが答えているメッセージが変わるときに再度設定します。

<h3 id="sdkcompactboundarymessage">
  `SDKCompactBoundaryMessage`
</h3>

会話圧縮境界を示すメッセージ。

```typescript theme={null}
type SDKCompactBoundaryMessage = {
  type: "system";
  subtype: "compact_boundary";
  uuid: UUID;
  session_id: string;
  compact_metadata: {
    trigger: "manual" | "auto";
    pre_tokens: number;
  };
};
```

<h3 id="sdkinformationalmessage">
  `SDKInformationalMessage`
</h3>

ループによって発行される汎用テキストバナー。エラーではないステータス行、`UserPromptSubmit` フックのブロック理由などのフックフィードバック、およびコマンド出力を含みます。Claude Code v2.1.227 以降では、フックの [`systemMessage`](/docs/ja/hooks#json-output) はこのメッセージとして到着でき、各行にはフックの名前が接頭辞として付けられます（`PostToolUse:Bash says:` など）。フックの `systemMessage` がこのメッセージとして到着するかどうかはイベントに依存します。各 [イベントのセクション](/docs/ja/hooks#hook-events) はフックページで出力がどのようにサーフェスするかを説明しています。`content` を指定された `level` でプレーンテキストとしてレンダリングします。

```typescript theme={null}
type SDKInformationalMessage = {
  type: "system";
  subtype: "informational";
  content: string;
  level: "info" | "notice" | "suggestion" | "warning";
  tool_use_id?: string;
  prevent_continuation?: boolean;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkworkershuttingdownmessage">
  `SDKWorkerShuttingDownMessage`
</h3>

グレースフルワーカーティアダウン時に発行されるため、リモートクライアントはハートビートタイムアウトを待つ代わりにワーカーが消えた理由を表示できます。`reason` はホスト CLI によって設定される短い snake\_case 文字列です（`"host_exit"` や `"remote_control_disabled"` など）。ライブストリーミング時にのみこれに対応します。再開されたセッションはこのメッセージの過去のインスタンスを再生するため、その場合は無視してください。

```typescript theme={null}
type SDKWorkerShuttingDownMessage = {
  type: "system";
  subtype: "worker_shutting_down";
  reason: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkplugininstallmessage">
  `SDKPluginInstallMessage`
</h3>

プラグインインストール進捗イベント。[`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/ja/env-vars) が設定されている場合に発行されるため、Agent SDK アプリケーションは最初のターンの前にマーケットプレイスプラグインのインストールを追跡できます。`started` と `completed` ステータスは全体的なインストールをブラケットします。`installed` と `failed` ステータスは個別のマーケットプレイスをレポートし、`name` を含みます。

```typescript theme={null}
type SDKPluginInstallMessage = {
  type: "system";
  subtype: "plugin_install";
  status: "started" | "installed" | "failed" | "completed";
  name?: string;
  error?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkpermissiondeniedmessage">
  `SDKPermissionDeniedMessage`
</h3>

権限システムがインタラクティブプロンプトなしでツール呼び出しを拒否するときに発行されるストリームイベント。これを使用して、その後に続く `is_error` ツール結果のみを観察するのではなく、拒否を UI にリアルタイムでレンダリングします。実行がどのように権限プロンプトを処理するかに応じて、どの拒否をレポートするかが異なります：

* **[`canUseTool`](#canusetool) コールバックとデフォルトの [`permissionPrompts: 'host'`](#options) を使用**：権限プロンプトはコールバックに送信され、このイベントは Claude Code がコールバックを呼び出さずに独自に決定した拒否をレポートします。
* **どちらでもない**：ベアの `-p` 実行、または `canUseTool` も `permissionPromptToolName` も設定しない `query()`。プロンプトが表示されるツール呼び出しを拒否し、このイベントはそれらの拒否と Claude Code が独自に決定した拒否の両方をレポートします。v2.1.223 より前では、Claude Code はコールバックなしの実行でこのイベントを発行しませんでした。
* **MCP プロンプトツール**（`permissionPromptToolName` または [`--permission-prompt-tool`](/docs/ja/cli-reference#cli-flags) フラグで設定）とデフォルトの `permissionPrompts: 'host'`：Claude Code はこのイベントをまったく発行しません。ルール拒否を含め、独自に決定した拒否についても発行しません。
* **[`permissionPrompts: 'none'`](#options) を使用**：Claude Code はプロンプトが表示されるツール呼び出しを拒否します。`canUseTool` または MCP プロンプトツールも設定されている場合でも、このイベントはそれらの拒否と Claude Code が独自に決定した拒否の両方をレポートします。Claude Code v2.1.259 以降が必要です。

すべての構成で、このイベントは `PreToolUse` フックパスで決定された拒否をスキップします。フックが呼び出し自体を拒否したか、拒否ルールがフックの許可または質問決定をオーバーライドしたかに関わらず。イベントはベストエフォートでもあります：時々 Claude Code は拒否を記録してもこのイベントを発行しないため、[結果メッセージ](#sdkresultmessage) の `permission_denials` が権限のある記録です。

```typescript theme={null}
type SDKPermissionDeniedMessage = {
  type: "system";
  subtype: "permission_denied";
  tool_name: string;
  tool_use_id: string;
  agent_id?: string;
  decision_reason_type?: string;
  decision_reason?: string;
  message: string;
  uuid: UUID;
  session_id: string;
};
```

| フィールド                  | 型        | 説明                                                                                  |
| ---------------------- | -------- | ----------------------------------------------------------------------------------- |
| `tool_name`            | `string` | 拒否されたツールの名前                                                                         |
| `tool_use_id`          | `string` | この拒否が答える `tool_use` ブロックの ID                                                        |
| `agent_id`             | `string` | 拒否された呼び出しがサブエージェント内で発生した場合のサブエージェント ID。ホスト側ルーティング用に `can_use_tool` のフィールドをミラーリングします |
| `decision_reason_type` | `string` | 決定したコンポーネントの判別式（`"rule"`、`"mode"`、`"classifier"`、`"asyncAgent"` など）                 |
| `decision_reason`      | `string` | 利用可能な場合、決定したコンポーネントからの人間が読める理由                                                      |
| `message`              | `string` | `tool_result` でモデルに返される拒否メッセージ                                                      |

<h3 id="sdkpermissiondenial">
  `SDKPermissionDenial`
</h3>

拒否されたツール使用に関する情報。

```typescript theme={null}
type SDKPermissionDenial = {
  tool_name: string;
  tool_use_id: string;
  tool_input: Record<string, unknown>;
};
```

<h3 id="sdkcontextusage">
  `SDKContextUsage`
</h3>

`/context` レポートの構造化形式。[`SDKAssistantMessage`](#sdkassistantmessage) で `context_usage` として実行される `/context` 結果を配信します。Agent SDK v0.3.232 以降はこの型をエクスポートします。[`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse) とは異なり、使用状況分析をレンダリングするために必要なデータのみを持ち、`color` と `gridRows` などの表示フィールドは持ちません。

```typescript theme={null}
type SDKContextUsage = {
  model: string;
  total_tokens: number;
  raw_max_tokens: number;
  percentage: number;
  over_limit?: {
    tokens_over: number;
    kind: "hard_limit" | "compaction_window";
  };
  categories: SDKContextUsageCategory[];
  mcp_tools: {
    name: string;
    server_name: string;
    tokens: number;
  }[];
  memory_files: {
    path: string;
    type: string;
    tokens: number;
  }[];
  agents: {
    agent_type: string;
    source: string;
    tokens: number;
  }[];
  skills?: {
    name: string;
    source: string;
    plugin_name?: string;
    tokens: number;
  }[];
};
```

テーブルは、Claude Code が各フィールドに何を入れるかをリストします。`model` から `over_limit` までのフィールドはセッション全体を説明し、コレクションフィールドはトークンを個別のアイテムに属性付けします。

| フィールド            | 型                                                         | 説明                                                                                                                                                                                                  |
| ---------------- | --------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `model`          | `string`                                                  | Claude Code が使用状況を計算したメインループのモデル。サブエージェントではありません                                                                                                                                                    |
| `total_tokens`   | `number`                                                  | Claude Code の使用中のトークンの推定値。ウィンドウにクランプされていないため、セッションが制限を超えている場合は `raw_max_tokens` を超える可能性があります                                                                                                        |
| `raw_max_tokens` | `number`                                                  | モデルのコンテキストウィンドウ、または設定したものなど、より低い [自動圧縮ウィンドウ](/docs/ja/model-config#context-window-and-auto-compaction)。1M トークンウィンドウを持つ一部のモデルに Claude Code が適用する 200K 境界など。Claude Code は `total_tokens` をこのウィンドウに対して測定します |
| `percentage`     | `number`                                                  | `total_tokens` を `raw_max_tokens` の丸められたパーセンテージとして。セッションが制限を超えている場合は 100 を超える可能性があります                                                                                                               |
| `over_limit`     | `object`                                                  | `total_tokens` が `raw_max_tokens` を超える場合にのみ存在します。`tokens_over` は超過量で、`kind` は Claude Code がウィンドウをどのように解決したかを示します                                                                                    |
| `categories`     | [`SDKContextUsageCategory`](#sdkcontextusagecategory)`[]` | 使用状況別カテゴリ分析の各行に 1 つのエントリ                                                                                                                                                                            |
| `mcp_tools`      | `object[]`                                                | 各 MCP ツールに属性付けされたトークン。ワイヤー名（`mcp__linear__create_issue` など）と `server_name` を持ちます                                                                                                                    |
| `memory_files`   | `object[]`                                                | 各読み込まれたメモリファイルに属性付けされたトークン。`path` と `Project` または `User` などのソースラベルを `type` で持ちます                                                                                                                    |
| `agents`         | `object[]`                                                | 各カスタムサブエージェント定義に属性付けされたトークン。`projectSettings`、`userSettings`、`plugin` などのソース識別子を持ちます。組み込みサブエージェントはリストされていません                                                                                        |
| `skills`         | `object[]`                                                | スキルリスト内の各スキルに属性付けされたトークン。ソース識別子と、プラグインスキルの場合はプラグインの名前を `plugin_name` で持ちます。スキルがトークンに貢献しない場合は不在です                                                                                                    |

`over_limit.kind` はウィンドウをどのように解決したかを記録し、API が次のリクエストを受け入れるかどうかではありません：

* `hard_limit`：ウィンドウは Claude Code がモデル自身の制限と信じるもので、API がリクエストを拒否する過去です
* `compaction_window`：ウィンドウは圧縮ポリシーウィンドウで、モデルの制限と一致する場合もしない場合もあります

Claude Code は型を加算的に進化させ、既存のものを再形成する代わりに新しいデータをオプションフィールドとして追加します。知っているフィールドを読み、認識しないものは無視してください。

<h3 id="sdkcontextusagecategory">
  `SDKContextUsageCategory`
</h3>

`/context` 使用状況別カテゴリ分析の 1 行。

```typescript theme={null}
type SDKContextUsageCategory = {
  name: string;
  tokens: number;
  kind: "used" | "free" | "buffer" | "deferred";
};
```

テーブルは、Claude Code が行の各フィールドに何を入れるかをリストします。

| フィールド    | 型        | 説明                                                             |
| -------- | -------- | -------------------------------------------------------------- |
| `name`   | `string` | `/context` が印刷するような行の表示名（`Messages` など）。名前ではなく `kind` で行を分類します |
| `tokens` | `number` | 行のトークンカウント。行はゼロトークンを持つことができます                                  |
| `kind`   | `string` | 行が表すもの：`used`、`free`、`buffer`、または `deferred`                   |

各 `kind` 値は行のトークンが何であるかを示します：

* `used`：コンテキストウィンドウを占有するコンテンツ
* `free`：残りのウィンドウ
* `buffer`：圧縮リザーブ
* `deferred`：Claude Code がウィンドウの外に保持し、使用状況計算から除外するツールスキーマ。認識用にリストされています

<h3 id="sdkmessageorigin">
  `SDKMessageOrigin`
</h3>

ユーザーロールメッセージの出所。これは [`SDKUserMessage`](#sdkusermessage) の `origin` として表示され、対応する [`SDKResultMessage`](#sdkresultmessage) に転送されるため、特定のターンをトリガーしたものを判断できます。

```typescript theme={null}
type SDKMessageOrigin =
  | { kind: "human" }
  | { kind: "channel"; server: string }
  | {
      kind: "peer";
      from: string;
      fromMode?: "bypass" | "prompting";
      name?: string;
      fromSession?: string;
      senderTaskId?: string;
      body?: string;
      verifiedPeerPid?: number;
    }
  | {
      kind: "task-notification";
      subkind?: "scheduled-trigger" | "peer-send-message";
      fireReason?: string;
    }
  | { kind: "coordinator" }
  | { kind: "auto-continuation" }
  | { kind: "unclassified" };
```

| `kind`              | 意味                                                                                                                                                                                                                                                                                                                                        |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `human`             | エンドユーザーからの直接入力。アプリケーションがユーザーが入力したものをユーザーメッセージとして転送する場合、その `origin` を明示的に `{ kind: "human" }` に設定します：Claude Code はユーザーメッセージに `origin` がないものを属性なしとして扱い、[`ultracode` ワークフローキーワード](/docs/ja/workflows#ask-for-a-workflow-in-your-prompt) などの人間が入力したプロンプトを必要とするチェックはそれを受け入れません。v2.1.210 より前では、Claude Code はユーザーメッセージの不在の `origin` をヒューマン入力として扱いました。 |
| `channel`           | [チャネル](/docs/ja/channels) に到着するメッセージ。`server` はソース MCP サーバー名です。                                                                                                                                                                                                                                                                                |
| `peer`              | 別のエージェントからのメッセージ：プロセス内の [teammate](/docs/ja/agent-teams) が `SendMessage` で送信するか、[クロスセッションピア](/docs/ja/cross-session-messaging)。別の Claude Code セッション。[ピア出所フィールド](#peer-origin-fields) を参照してください。                                                                                                                                                     |
| `task-notification` | 新しいユーザープロンプトなしで配信が到着するときに注入される合成ターン（完了したバックグラウンドタスクなど）。[`SDKTaskNotificationMessage`](#sdktasknotificationmessage) をそのアームについて参照してください。オプションの `subkind` は通知を発生させたものをマークします。[タスク通知サブキンド](#task-notification-subkinds) を参照してください。                                                                                                            |
| `coordinator`       | [エージェントチーム](/docs/ja/agent-teams) のチームコーディネーターからのメッセージ。                                                                                                                                                                                                                                                                                        |
| `auto-continuation` | セッションが新しいユーザー入力なしで続行するときに注入される合成ターン（コマンド結果がフォローアッププロンプトをトリガーするなど）。                                                                                                                                                                                                                                                                        |
| `unclassified`      | 出所を判断できなかった注入されたターン。Claude Code が [`SDKUserMessage`](#sdkusermessage) を `isSynthetic: true` で受け取り、他の `kind` として分類できない場合、メッセージが到着するときにこのキンドを設定し、ターンをモデルに非ユーザーソースとしてフレーミングします。ヒューマン入力として扱う代わりに。アプリケーションはこの値を設定しないでください。                                                                                                                     |

<h3 id="task-notification-subkinds">
  タスク通知サブキンド
</h3>

Claude Code がタスク通知をセッションに配信するとき、その通知の `origin` に `subkind` を設定するのは、Anthropic サーバーがその通知がどこから来たかを検証した場合のみです。また、アプリケーションが [スケジュール済み実行を宣言](#declare-a-scheduled-run) する場合にも `subkind` を設定します。これには TypeScript Agent SDK v0.3.280 以降が必要です。`subkind` には Claude Code v2.1.213 以降が必要で、2 つの値のいずれかを取ります：

* `scheduled-trigger`：通知は [ルーチン](/docs/ja/routines) の保存されたプロンプトで、ルーチンのトリガーの 1 つが発火したため配信されました：スケジュール、[API トリガー](/docs/ja/routines#add-an-api-trigger)、[GitHub トリガー](/docs/ja/routines#add-a-github-trigger)、または **今すぐ実行**。アプリケーションが [スケジュール済み実行を宣言](#declare-a-scheduled-run) する場合も、この値を持ちます。Claude Code はこれらをモデルにセッションの割り当てられたタスクとしてフレーミングし、[他のタスク通知が持つ通知](#sdktasknotificationmessage) とは異なる通知を持ちます。
* `peer-send-message`：通知は別のセッションが [クラウドセッション](/docs/ja/claude-code-on-the-web) が互いにメッセージするために使用するサーバー側 `send_message` ツールで送信したメッセージで、[クロスセッション `SendMessage` ツール](/docs/ja/cross-session-messaging) ではなく、Anthropic サーバーが両方のセッションが同じプライベートセッショングループに属することを検証しました。Claude Code v2.1.224 以降が必要です。サーバーが検証しなかった `send_message` 配信は `subkind` を取得しません。

他のすべてのタスク通知には `subkind` がありません。これには、マシンで発火する [スケジュール済みタスク](/docs/ja/scheduled-tasks)、[PR アクティビティ](/docs/ja/claude-code-on-the-web#how-claude-responds-to-pr-activity) がセッションに配信される、バックグラウンドイベント（完了したタスクなど）が含まれます。[クロスセッション `SendMessage` ツール](/docs/ja/cross-session-messaging) からのメッセージはタスク通知ではありません：同じマシンのセッションから来るか、別のマシンから Anthropic サーバーを通じて来るかに関わらず、Claude Code は `kind: "peer"` と [ピア出所フィールド](#peer-origin-fields) を与えます。

`fireReason` は `scheduled-trigger` 通知が発火した理由を示します。`scheduled`、`manual`、`retry`、`catch_up`、`api` などの短い小文字トークンとして。Anthropic サーバーは [ルーチン](/docs/ja/routines) の配信に設定し、アプリケーションはスケジュール済み実行を宣言するときに設定します。どちらも送信しなかった場合は不在です。TypeScript Agent SDK v0.3.280 以降が必要です。

<h4 id="declare-a-scheduled-run">
  スケジュール済み実行を宣言する
</h4>

アプリケーションが独自のスケジュールでプロンプトを実行する場合、各実行を宣言して、Claude Code がターンをモデルにスケジュール済みタスクとしてフレーミングするようにします。ライブユーザー入力ではなく。[`env`](#options) で `CLAUDE_CODE_HOST_SCHEDULED_RUN` を `1` に設定してセッションを開始し、実行の [`SDKUserMessage`](#sdkusermessage) を `origin: { kind: "task-notification", subkind: "scheduled-trigger", fireReason: "scheduled" }` で送信し、`isSynthetic` なしで送信します。Claude Code はその変数なしで開始されたプロセスで宣言を無視します。また、プロセスの環境が [`CLAUDECODE`](/docs/ja/env-vars) または `CLAUDE_CODE_CHILD_SESSION` を持つプロセスでも無視します。Claude Code は `fireReason` を値が 1 ～ 32 の小文字の文字または下線である場合にのみ保持します。TypeScript Agent SDK v0.3.280 以降が必要です。

<h3 id="peer-origin-fields">
  ピア出所フィールド
</h3>

`peer` 出所は、メッセージを送信したエージェントを識別します：`SendMessage` で `main` に送信するプロセス内の [teammate](/docs/ja/agent-teams)、または [クロスセッションピア](/docs/ja/cross-session-messaging)。別の Claude Code セッション。クロスセッションピアには macOS と Linux で Claude Code v2.1.224 以降が必要です。[クロスセッションメッセージング可用性](/docs/ja/cross-session-messaging#availability) でネイティブ Windows 要件を参照してください。クロスセッションピアは同じマシンで実行でき、[別のマシン](/docs/ja/cross-session-messaging#message-sessions-on-other-machines) または [クラウド](/docs/ja/claude-code-on-the-web) で実行でき、Remote Control を通じてメッセージが到着するときです。2 種類の送信者はフィールドを異なる方法で埋めます：

* `from`：teammate の名前、またはクロスセッションピアの送信者アドレス。[一方向クロスマシンメッセージ](/docs/ja/cross-session-messaging#message-sessions-on-other-machines) の場合、送信者は返信アドレスを持たず、`from` は `"unknown"` です。値は送信者が作成したもので、`verifiedPeerPid` が検証された ID です。
* `fromMode`：送信セッションの権限クラス。`bypass` または `prompting`。セッション間でピアメッセージをリレーするホスト（[デスクトップアプリ](/docs/ja/desktop#work-across-sessions) など）によって宣言されます。Claude Code は [インバウンドコントロール](/docs/ja/cross-session-messaging#control-inbound-messages) を適用するときに受信セッションで読みます。Agent SDK v0.3.234 以降が必要です。
* `senderTaskId`：teammate のタスク ID。クロスセッションピアの場合は不在です。
* `name`：送信者の表示名。Claude Code によって正規化されます：Unicode 制御、形式、サロゲート、および行または段落区切り文字コードポイントを削除し、結果をトリミングして 64 コードポイントで上限に達し、省略記号を付けます。Claude Code v2.1.205 以降が必要です。
* `body`：ピアエンベロープを削除したデコードされたメッセージ本体。モデルが見るものとバイト単位で正確です。teammate メッセージの場合は常に存在します。クロスセッションピアの場合、ターンが Claude Code によって形成された正確に 1 つのピアエンベロープである場合にのみ存在します。メッセージテキストを再解析する代わりに、`name` と `body` をレンダリングします。Claude Code v2.1.205 以降が必要です。
* `fromSession`：送信者のホストが開くことができるセッション ID。送信者のホストによって設定されるため、UI が送信セッションにリンクバックできます。`from` と同様に、送信者が主張したものです：ナビゲーションターゲットとしてのみ使用し、送信者の ID の証明として扱わないでください。Claude Code v2.1.216 以降が必要です。
* `verifiedPeerPid`：このセッションのクロスセッションメッセージングソケットに接続したプロセスのプロセス ID。カーネルによって検証され、ペイロードからではなく接続自体から読み取られます。送信者を識別するには `from` ではなくこれを使用してください：`from` は同じユーザープロセスによって偽造可能です。フィールドは Claude Code がそれを検証できない場合（Windows やソケット以外の入力など）に不在です。不在の値は送信者が未検証であることを意味します。リレーされたトラフィックの場合、メッセージの作成者ではなくリレーを識別し、プロセス ID は再利用可能なため、認証トークンではなく出所として扱ってください。Claude Code v2.1.216 以降が必要です。

<h2 id="hook-types">
  フック型
</h2>

フックの使用に関する包括的なガイド、例、一般的なパターンについては、[フックガイド](/docs/ja/agent-sdk/hooks) を参照してください。

<h3 id="hookevent">
  `HookEvent`
</h3>

利用可能なフックイベント。

```typescript theme={null}
type HookEvent =
  | "PreToolUse"
  | "PostToolUse"
  | "PostToolUseFailure"
  | "PostToolBatch"
  | "Notification"
  | "UserPromptSubmit"
  | "UserPromptExpansion"
  | "SessionStart"
  | "SessionEnd"
  | "Stop"
  | "StopFailure"
  | "SubagentStart"
  | "SubagentStop"
  | "PreCompact"
  | "PostCompact"
  | "PreModelSwitch"
  | "PostModelSwitch"
  | "PermissionRequest"
  | "PermissionDenied"
  | "Setup"
  | "TeammateIdle"
  | "TaskCreated"
  | "TaskCompleted"
  | "Elicitation"
  | "ElicitationResult"
  | "ConfigChange"
  | "DirectoryAdded"
  | "WorktreeCreate"
  | "WorktreeRemove"
  | "InstructionsLoaded"
  | "CwdChanged"
  | "FileChanged"
  | "MessageDisplay";
```

<h3 id="hookcallback">
  `HookCallback`
</h3>

フックコールバック関数型。

```typescript theme={null}
type HookCallback = (
  input: HookInput, // すべてのフック入力型の共用体
  toolUseID: string | undefined,
  options: { signal: AbortSignal }
) => Promise<HookJSONOutput>;
```

<h3 id="hookcallbackmatcher">
  `HookCallbackMatcher`
</h3>

オプションのマッチャーを含むフック設定。

```typescript theme={null}
interface HookCallbackMatcher {
  matcher?: string;
  hooks: HookCallback[];
  timeout?: number; // このマッチャーのすべてのフックのタイムアウト（秒）
}
```

<h3 id="hookinput">
  `HookInput`
</h3>

すべてのフック入力型の共用体型。

```typescript theme={null}
type HookInput =
  | PreToolUseHookInput
  | PostToolUseHookInput
  | PostToolUseFailureHookInput
  | PostToolBatchHookInput
  | PermissionDeniedHookInput
  | NotificationHookInput
  | UserPromptSubmitHookInput
  | UserPromptExpansionHookInput
  | SessionStartHookInput
  | SessionEndHookInput
  | StopHookInput
  | StopFailureHookInput
  | SubagentStartHookInput
  | SubagentStopHookInput
  | PreCompactHookInput
  | PostCompactHookInput
  | PreModelSwitchHookInput
  | PostModelSwitchHookInput
  | PermissionRequestHookInput
  | SetupHookInput
  | TeammateIdleHookInput
  | TaskCreatedHookInput
  | TaskCompletedHookInput
  | ElicitationHookInput
  | ElicitationResultHookInput
  | ConfigChangeHookInput
  | InstructionsLoadedHookInput
  | DirectoryAddedHookInput
  | WorktreeCreateHookInput
  | WorktreeRemoveHookInput
  | CwdChangedHookInput
  | FileChangedHookInput
  | MessageDisplayHookInput;
```

<h3 id="basehookinput">
  `BaseHookInput`
</h3>

すべてのフック入力型が拡張する基本インターフェース。

```typescript theme={null}
type BaseHookInput = {
  session_id: string;
  transcript_path: string;
  cwd: string;
  prompt_id?: string;
  permission_mode?: string;
  effort?: { level: string };
  agent_id?: string;
  agent_type?: string;
};
```

`prompt_id` フィールドは、現在処理中のユーザープロンプトを識別する UUID です。[OpenTelemetry イベントの `prompt.id` 属性](/docs/ja/monitoring-usage#event-correlation-attributes) と一致し、最初のユーザー入力まで存在しません。Claude Code v2.1.196 以降が必要です。

<h4 id="pretoolusehookinput">
  `PreToolUseHookInput`
</h4>

```typescript theme={null}
type PreToolUseHookInput = BaseHookInput & {
  hook_event_name: "PreToolUse";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  mcp_server?: McpServerProvenance;
};
```

`mcp_server` は、ツールが MCP サーバーから来た場合に存在します。[`McpServerProvenance`](#mcpserverprovenance) を参照してください。`PostToolUse`、`PostToolUseFailure`、`PermissionRequest`、および `PermissionDenied` 入力は同じフィールドを保持します。このフィールドには Agent SDK v0.3.274 以降が必要です。

<h4 id="posttoolusehookinput">
  `PostToolUseHookInput`
</h4>

```typescript theme={null}
type PostToolUseHookInput = BaseHookInput & {
  hook_event_name: "PostToolUse";
  tool_name: string;
  tool_input: unknown;
  tool_response: unknown;
  tool_use_id: string;
  duration_ms?: number;
  mcp_server?: McpServerProvenance;
};
```

<h4 id="posttoolusefailurehookinput">
  `PostToolUseFailureHookInput`
</h4>

```typescript theme={null}
type PostToolUseFailureHookInput = BaseHookInput & {
  hook_event_name: "PostToolUseFailure";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  error: string;
  is_interrupt?: boolean;
  duration_ms?: number;
  mcp_server?: McpServerProvenance;
};
```

<h4 id="posttoolbatchhookinput">
  `PostToolBatchHookInput`
</h4>

バッチ内のすべてのツール呼び出しが解決された後、次のモデルリクエストの前に 1 回発火します。`tool_response` はモデルが見るシリアル化された `tool_result` コンテンツを保持します。形状は `PostToolUseHookInput` の構造化された `Output` オブジェクトとは異なります。

```typescript theme={null}
type PostToolBatchHookInput = BaseHookInput & {
  hook_event_name: "PostToolBatch";
  tool_calls: PostToolBatchToolCall[];
};

type PostToolBatchToolCall = {
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  tool_response?: unknown;
};
```

<h4 id="permissiondeniedhookinput">
  `PermissionDeniedHookInput`
</h4>

```typescript theme={null}
type PermissionDeniedHookInput = BaseHookInput & {
  hook_event_name: "PermissionDenied";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  reason: string;
  mcp_server?: McpServerProvenance;
};
```

<h4 id="notificationhookinput">
  `NotificationHookInput`
</h4>

```typescript theme={null}
type NotificationHookInput = BaseHookInput & {
  hook_event_name: "Notification";
  message: string;
  title?: string;
  notification_type: string;
};
```

<h4 id="userpromptsubmithookinput">
  `UserPromptSubmitHookInput`
</h4>

```typescript theme={null}
type UserPromptSubmitHookInput = BaseHookInput & {
  hook_event_name: "UserPromptSubmit";
  prompt: string;
  session_title?: string;
};
```

<h4 id="userpromptexpansionhookinput">
  `UserPromptExpansionHookInput`
</h4>

```typescript theme={null}
type UserPromptExpansionHookInput = BaseHookInput & {
  hook_event_name: "UserPromptExpansion";
  expansion_type: "slash_command" | "mcp_prompt";
  command_name: string;
  command_args: string;
  command_source?: string;
  prompt: string;
};
```

<h4 id="sessionstarthookinput">
  `SessionStartHookInput`
</h4>

```typescript theme={null}
type SessionStartHookInput = BaseHookInput & {
  hook_event_name: "SessionStart";
  source: "startup" | "resume" | "clear" | "compact" | "fork";
  agent_type?: string;
  model?: string;
  session_title?: string;
};
```

<h4 id="sessionendhookinput">
  `SessionEndHookInput`
</h4>

```typescript theme={null}
type SessionEndHookInput = BaseHookInput & {
  hook_event_name: "SessionEnd";
  reason: ExitReason; // EXIT_REASONS 配列からの文字列
};
```

<h4 id="stophookinput">
  `StopHookInput`
</h4>

```typescript theme={null}
type StopHookInput = BaseHookInput & {
  hook_event_name: "Stop";
  stop_hook_active: boolean;
  last_assistant_message?: string;
  background_tasks?: BackgroundTaskSummary[];
  session_crons?: SessionCronSummary[];
};
```

<h4 id="stopfailurehookinput">
  `StopFailureHookInput`
</h4>

```typescript theme={null}
type StopFailureHookInput = BaseHookInput & {
  hook_event_name: "StopFailure";
  error: SDKAssistantMessageError;
  error_details?: string;
  last_assistant_message?: string;
};
```

<h4 id="subagentstarthookinput">
  `SubagentStartHookInput`
</h4>

```typescript theme={null}
type SubagentStartHookInput = BaseHookInput & {
  hook_event_name: "SubagentStart";
  agent_id: string;
  agent_type: string;
};
```

<h4 id="subagentstophookinput">
  `SubagentStopHookInput`
</h4>

```typescript theme={null}
type SubagentStopHookInput = BaseHookInput & {
  hook_event_name: "SubagentStop";
  stop_hook_active: boolean;
  agent_id: string;
  agent_transcript_path: string;
  agent_type: string;
  last_assistant_message?: string;
  background_tasks?: BackgroundTaskSummary[];
  session_crons?: SessionCronSummary[];
};

type BackgroundTaskSummary = {
  id: string;
  type: string;
  status: string;
  description: string;
  command?: string;
  agent_type?: string;
  server?: string;
  tool?: string;
  name?: string;
};

type SessionCronSummary = {
  id: string;
  schedule: string;
  recurring: boolean;
  prompt: string;
};
```

<h4 id="precompacthookinput">
  `PreCompactHookInput`
</h4>

```typescript theme={null}
type PreCompactHookInput = BaseHookInput & {
  hook_event_name: "PreCompact";
  trigger: "manual" | "auto";
  custom_instructions: string | null;
};
```

<h4 id="postcompacthookinput">
  `PostCompactHookInput`
</h4>

```typescript theme={null}
type PostCompactHookInput = BaseHookInput & {
  hook_event_name: "PostCompact";
  trigger: "manual" | "auto";
  compact_summary: string;
};
```

<h4 id="premodelswitchhookinput">
  `PreModelSwitchHookInput`
</h4>

リクエストされたモデルスイッチが有効になる前に発火します。`context_tokens` とそれ以降のフィールドは、新しいモデルに会話を再送信するコストを推定します。完全なフィールド説明とブロッキングセマンティクスについては、[PreModelSwitch](/docs/ja/hooks#premodelswitch) を参照してください。

```typescript theme={null}
type PreModelSwitchHookInput = BaseHookInput & {
  hook_event_name: "PreModelSwitch";
  from_model: string;
  to_model: string;
  requested_model: string | null;
  source: "command" | "picker" | "sdk";
  context_tokens: number;
  prompt_cache_warm: boolean;
  cache_ttl: "5m" | "1h";
  estimated_cache_write_usd: number;
  pricing: "configured" | "catalog" | "default";
};
```

<h4 id="postmodelswitchhookinput">
  `PostModelSwitchHookInput`
</h4>

セッションのモデルが変更された後に発火します。`PreModelSwitchHookInput` と同じフィールドを保持し、2 つの追加の `source` 値があります。[PostModelSwitch](/docs/ja/hooks#postmodelswitch) を参照してください。

```typescript theme={null}
type PostModelSwitchHookInput = BaseHookInput & {
  hook_event_name: "PostModelSwitch";
  from_model: string;
  to_model: string;
  requested_model: string | null;
  source: "command" | "picker" | "sdk" | "auto" | "resume";
  context_tokens: number;
  prompt_cache_warm: boolean;
  cache_ttl: "5m" | "1h";
  estimated_cache_write_usd: number;
  pricing: "configured" | "catalog" | "default";
};
```

<h4 id="permissionrequesthookinput">
  `PermissionRequestHookInput`
</h4>

```typescript theme={null}
type PermissionRequestHookInput = BaseHookInput & {
  hook_event_name: "PermissionRequest";
  tool_name: string;
  tool_input: unknown;
  permission_suggestions?: PermissionUpdate[];
  mcp_server?: McpServerProvenance;
};
```

<h4 id="setuphookinput">
  `SetupHookInput`
</h4>

```typescript theme={null}
type SetupHookInput = BaseHookInput & {
  hook_event_name: "Setup";
  trigger: "init" | "maintenance";
};
```

<h4 id="teammateidlehookinput">
  `TeammateIdleHookInput`
</h4>

```typescript theme={null}
type TeammateIdleHookInput = BaseHookInput & {
  hook_event_name: "TeammateIdle";
  teammate_name: string;
  /** @deprecated v2.1.178 以降は非推奨。セッション派生のチーム名を保持します。削除予定です。 */
  team_name: string;
};
```

<h4 id="taskcreatedhookinput">
  `TaskCreatedHookInput`
</h4>

```typescript theme={null}
type TaskCreatedHookInput = BaseHookInput & {
  hook_event_name: "TaskCreated";
  task_id: string;
  task_subject: string;
  task_description?: string;
  teammate_name?: string;
  /** @deprecated v2.1.178 以降は非推奨。セッション派生のチーム名を保持します。削除予定です。 */
  team_name?: string;
};
```

<h4 id="taskcompletedhookinput">
  `TaskCompletedHookInput`
</h4>

```typescript theme={null}
type TaskCompletedHookInput = BaseHookInput & {
  hook_event_name: "TaskCompleted";
  task_id: string;
  task_subject: string;
  task_description?: string;
  teammate_name?: string;
  /** @deprecated v2.1.178 以降は非推奨。セッション派生のチーム名を保持します。削除予定です。 */
  team_name?: string;
};
```

<h4 id="elicitationhookinput">
  `ElicitationHookInput`
</h4>

```typescript theme={null}
type ElicitationHookInput = BaseHookInput & {
  hook_event_name: "Elicitation";
  mcp_server_name: string;
  message: string;
  mode?: "form" | "url";
  url?: string;
  elicitation_id?: string;
  requested_schema?: Record<string, unknown>;
};
```

<h4 id="elicitationresulthookinput">
  `ElicitationResultHookInput`
</h4>

```typescript theme={null}
type ElicitationResultHookInput = BaseHookInput & {
  hook_event_name: "ElicitationResult";
  mcp_server_name: string;
  elicitation_id?: string;
  mode?: "form" | "url";
  action: "accept" | "decline" | "cancel";
  content?: Record<string, unknown>;
};
```

<h4 id="configchangehookinput">
  `ConfigChangeHookInput`
</h4>

```typescript theme={null}
type ConfigChangeHookInput = BaseHookInput & {
  hook_event_name: "ConfigChange";
  source:
    | "user_settings"
    | "project_settings"
    | "local_settings"
    | "policy_settings"
    | "skills";
  file_path?: string;
};
```

<h4 id="instructionsloadedhookinput">
  `InstructionsLoadedHookInput`
</h4>

```typescript theme={null}
type InstructionsLoadedHookInput = BaseHookInput & {
  hook_event_name: "InstructionsLoaded";
  file_path: string;
  memory_type: "User" | "Project" | "Local" | "Managed";
  load_reason:
    | "session_start"
    | "nested_traversal"
    | "path_glob_match"
    | "include"
    | "compact";
  globs?: string[];
  trigger_file_path?: string;
  parent_file_path?: string;
};
```

<h4 id="directoryaddedhookinput">
  `DirectoryAddedHookInput`
</h4>

```typescript theme={null}
type DirectoryAddedHookInput = BaseHookInput & {
  hook_event_name: "DirectoryAdded";
  directory: string;
  source: "slash_command" | "register_repo_root";
};
```

`directory` は追加されたディレクトリの絶対パスです。`source` は `/add-dir` で追加された場合は `"slash_command"` で、SDK 制御リクエストで追加された場合は `"register_repo_root"` です。

<h4 id="worktreecreatehookinput">
  `WorktreeCreateHookInput`
</h4>

```typescript theme={null}
type WorktreeCreateHookInput = BaseHookInput & {
  hook_event_name: "WorktreeCreate";
  name: string;
};
```

<h4 id="worktreeremovehookinput">
  `WorktreeRemoveHookInput`
</h4>

```typescript theme={null}
type WorktreeRemoveHookInput = BaseHookInput & {
  hook_event_name: "WorktreeRemove";
  worktree_path: string;
};
```

<h4 id="cwdchangedhookinput">
  `CwdChangedHookInput`
</h4>

```typescript theme={null}
type CwdChangedHookInput = BaseHookInput & {
  hook_event_name: "CwdChanged";
  old_cwd: string;
  new_cwd: string;
};
```

<h4 id="filechangedhookinput">
  `FileChangedHookInput`
</h4>

```typescript theme={null}
type FileChangedHookInput = BaseHookInput & {
  hook_event_name: "FileChanged";
  file_path: string;
  event: "change" | "add" | "unlink";
};
```

<h4 id="messagedisplayhookinput">
  `MessageDisplayHookInput`
</h4>

```typescript theme={null}
type MessageDisplayHookInput = BaseHookInput & {
  hook_event_name: "MessageDisplay";
  turn_id: string;
  message_id: string;
  index: number;
  final: boolean;
  delta: string;
};
```

<h3 id="hookjsonoutput">
  `HookJSONOutput`
</h3>

フック戻り値。

```typescript theme={null}
type HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput;
```

<h4 id="asynchookjsonoutput">
  `AsyncHookJSONOutput`
</h4>

```typescript theme={null}
type AsyncHookJSONOutput = {
  async: true;
  asyncTimeout?: number;
};
```

<h4 id="synchookjsonoutput">
  `SyncHookJSONOutput`
</h4>

```typescript theme={null}
type SyncHookJSONOutput = {
  continue?: boolean;
  suppressOutput?: boolean;
  stopReason?: string;
  decision?: "approve" | "block";
  systemMessage?: string;
  /**
   * Claude Code があなたの代わりに発行するターミナルエスケープシーケンス（例：OSC 9 / OSC 777 デスクトップ通知）。
   * 通知/タイトル OSC（0、1、2、9、99、777）と BEL のみが許可されます。
   * 他のものを含む値は全体として無視されます。インタラクティブ CLI のみが
   * 発行します。SDK はこのフィールドを無視します。
   */
  terminalSequence?: string;
  reason?: string;
  hookSpecificOutput?:
    | {
        hookEventName: "PreToolUse";
        permissionDecision?: "allow" | "deny" | "ask" | "defer";
        permissionDecisionReason?: string;
        updatedInput?: Record<string, unknown>;
        additionalContext?: string;
      }
    | {
        hookEventName: "UserPromptSubmit";
        additionalContext?: string;
        sessionTitle?: string;
        /** decision が "block" の場合、ブロックメッセージから元のプロンプトを省略します。 */
        suppressOriginalPrompt?: boolean;
      }
    | {
        hookEventName: "UserPromptExpansion";
        additionalContext?: string;
      }
    | {
        hookEventName: "SessionStart";
        additionalContext?: string;
        initialUserMessage?: string;
        sessionTitle?: string;
        watchPaths?: string[];
        /**
         * SessionStart フックが完了した後、スキルとコマンドディレクトリを再スキャンします。
         * これにより、フックによってインストールされたスキルが同じセッションで利用可能になります。
         */
        reloadSkills?: boolean;
      }
    | {
        hookEventName: "Setup";
        additionalContext?: string;
      }
    | {
        hookEventName: "PreModelSwitch";
        /**
         * PreToolUse と同じ契約：「allow」は進行、「deny」はスイッチをキャンセル、
         * 「ask」はユーザーに確認を求めます。インタラクティブセッションの /model のみが
         * そのプロンプトを表示します。他のすべてのサーフェス（set_model リクエストを含む）は、
         * 「ask」を拒否として扱います。
         */
        permissionDecision?: "allow" | "deny" | "ask";
        permissionDecisionReason?: string;
      }
    | {
        hookEventName: "PostModelSwitch";
        /** 新しいモデルが提供する次のリクエストでモデルに到達します。 */
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolUse";
        additionalContext?: string;
        /**
         * このツール呼び出しの結果に関する短いメモで、自動モード
         * 権限分類器用です。2000 文字に制限され、同じ呼び出しに応答する
         * すべてのフック間で共有されます。同期フックレスポンスでのみ尊重されます。
         * 信頼できないツール出力をコピーしないでください。
         */
        classifierContext?: string;
        updatedToolOutput?: unknown;
        /** @deprecated `updatedToolOutput` を使用してください。これはすべてのツールで機能します。 */
        updatedMCPToolOutput?: unknown;
      }
    | {
        hookEventName: "PostToolUseFailure";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolBatch";
        additionalContext?: string;
      }
    | {
        hookEventName: "Stop";
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStop";
        additionalContext?: string;
      }
    | {
        hookEventName: "PermissionDenied";
        retry?: boolean;
      }
    | {
        hookEventName: "Notification";
        additionalContext?: string;
      }
    | {
        hookEventName: "PermissionRequest";
        decision:
          | {
              behavior: "allow";
              updatedInput?: Record<string, unknown>;
              updatedPermissions?: PermissionUpdate[];
            }
          | {
              behavior: "deny";
              message?: string;
              interrupt?: boolean;
            };
      }
    | {
        hookEventName: "Elicitation";
        action?: "accept" | "decline" | "cancel";
        content?: Record<string, unknown>;
      }
    | {
        hookEventName: "ElicitationResult";
        action?: "accept" | "decline" | "cancel";
        content?: Record<string, unknown>;
      }
    | {
        hookEventName: "CwdChanged";
        watchPaths?: string[];
      }
    | {
        hookEventName: "FileChanged";
        watchPaths?: string[];
      }
    | {
        hookEventName: "WorktreeCreate";
        worktreePath: string;
      }
    | {
        hookEventName: "MessageDisplay";
        /** デルタの代わりに表示されるテキスト。元のテキストを表示するには、省略するか（またはデルタを変更せずに返す）。 */
        displayContent?: string;
      };
};
```

<h2 id="tool-input-types">
  ツール入力型
</h2>

すべての組み込み Claude Code ツールの入力スキーマのドキュメント。これらの型は `@anthropic-ai/claude-agent-sdk` からエクスポートされ、タイプセーフなツール相互作用に使用できます。

<h3 id="toolinputschemas">
  `ToolInputSchemas`
</h3>

`@anthropic-ai/claude-agent-sdk` からエクスポートされたツール入力型の共用体。メンバーには以下が含まれます：

```typescript theme={null}
type ToolInputSchemas =
  | AgentInput
  | ArtifactInput
  | AskUserQuestionInput
  | BashInput
  | CronCreateInput
  | CronDeleteInput
  | CronListInput
  | EnterPlanModeInput
  | EnterWorktreeInput
  | ExitPlanModeInput
  | ExitWorktreeInput
  | FileEditInput
  | FileReadInput
  | FileWriteInput
  | GlobInput
  | GrepInput
  | ListMcpResourcesInput
  | McpInput
  | MonitorInput
  | NotebookEditInput
  | ProjectsInput
  | PushNotificationInput
  | ReadMcpResourceDirInput
  | ReadMcpResourceInput
  | RefreshMcpToolsInput
  | RemoteTriggerInput
  | ReportFindingsInput
  | ScheduleWakeupInput
  | ShowOnboardingRolePickerInput
  | TaskCreateInput
  | TaskGetInput
  | TaskListInput
  | TaskStopInput
  | TaskUpdateInput
  | TodoWriteInput
  | WebFetchInput
  | WebSearchInput
  | WorkflowInput;
```

<h3 id="agent">
  Agent
</h3>

**ツール名：** `Agent`。以前の名前 `Task` はまだエイリアスとして受け入れられ、[`SDKSystemMessage`](#sdksystemmessage) 初期化メッセージの `tools` 配列は、後方互換性のためこのツールを `Task` として一覧表示しています。

<Note>
  `mode` フィールドは Claude Code v2.1.212 以降では非推奨で無視されます。サブエージェントは親セッションの権限モードまたはその定義の [`permissionMode`](#agentdefinition) のいずれかで実行され、[サブエージェント継承ルール](/docs/ja/agent-sdk/permissions#available-modes) がどちらであるかを決定します。
</Note>

```typescript theme={null}
type AgentInput = {
  description: string;
  prompt: string;
  subagent_type?: string;
  model?: "sonnet" | "opus" | "haiku" | "fable";
  run_in_background?: boolean;
  name?: string;
  team_name?: string; // 非推奨；無視されます
  mode?: "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan"; // 非推奨；無視されます。サブエージェント継承ルールがサブエージェントの権限モードを決定します
  isolation?: "worktree" | "remote";
};
```

複雑なマルチステップタスクを自律的に処理する新しいエージェントを起動します。

<h3 id="askuserquestion">
  AskUserQuestion
</h3>

**ツール名：** `AskUserQuestion`

```typescript theme={null}
type AskUserQuestionInput = {
  questions: Array<{
    question: string;
    header: string;
    options: Array<{ label: string; description: string; preview?: string }>;
    multiSelect: boolean;
  }>;
  answers?: Record<string, string>;
  annotations?: Record<string, { preview?: string; notes?: string }>;
  metadata?: { source?: string };
};
```

実行中にユーザーに明確化の質問をします。使用方法の詳細については、[承認とユーザー入力を処理](/docs/ja/agent-sdk/user-input#handle-clarifying-questions) を参照してください。

<h3 id="bash">
  Bash
</h3>

**ツール名：** `Bash`

```typescript theme={null}
type BashInput = {
  command: string;
  timeout?: number; // ミリ秒、最大 600000；より高い値は最大値にクランプされます
  description?: string;
  run_in_background?: boolean;
  dangerouslyDisableSandbox?: boolean;
};
```

オプションのタイムアウトとバックグラウンド実行を備えた Bash コマンドを実行します。作業ディレクトリはコマンド間で永続化されます。マルチターンセッションの後のターンで実行されるコマンドも含まれます。エクスポートされた環境変数などのシェル状態は永続化されません。ディレクトリ変更がどの程度引き継がれるかの制限については、[コマンド間で永続化されるもの](/docs/ja/tools-reference#what-persists-between-commands) を参照してください。

<h3 id="monitor">
  Monitor
</h3>

**ツール名：** `Monitor`

```typescript theme={null}
type MonitorInput = {
  description: string;
  timeout_ms: number;
  command?: string;
  ws?: {
    url: string;
    protocols?: string[];
  };
};
```

バックグラウンドソースを実行し、各イベントを Claude に配信するため、ポーリングなしで反応できます。`command` はスクリプトを実行し、stdout 行ごとに 1 つのイベントを発行し、`ws` は WebSocket を開き、テキストフレームごとに 1 つのイベントを発行します。`command` または `ws` のいずれか正確に 1 つを指定してください。`ws` ソースには Claude Code v2.1.195 以降が必要です。

`timeout_ms` はウォッチの期限（ミリ秒）です。デフォルトは 300000 で、最大 3600000 の値を受け入れます。有効な期限は最大 1800000（30 分）です。より大きい受け入れられた値はその値に短縮されます。期限に達するとウォッチが終了し、Claude は 1 つの通知を受け取るため、必要に応じて新しいウォッチを開始できます。

エクスポートされた型は `timeout_ms` を必須としてマークしています。スキーマがデフォルトを入力するため、それを省略する呼び出しは検証されます。

Monitor がコマンドを実行する場合、Bash と同じパーミッションルールに従います。WebSocket ウォッチは別途承認を求めます。動作とプロバイダーの可用性については、[Monitor ツールリファレンス](/docs/ja/tools-reference#monitor-tool) を参照してください。

<h3 id="taskoutput">
  TaskOutput
</h3>

Claude Code v2.1.277 で削除されました。その `TaskOutputInput` 型と一緒に削除されました。以前は実行中または完了したバックグラウンドタスクから出力を取得していました。Claude はバックグラウンドタスクの出力ファイルを `Read` で読み取ります。

`disallowedTools` エントリまたは `TaskOutput` という名前の deny ルールは警告なしに無視されます。

<h3 id="edit">
  Edit
</h3>

**ツール名：** `Edit`

```typescript theme={null}
type FileEditInput = {
  file_path: string;
  old_string: string;
  new_string: string;
  replace_all?: boolean;
};
```

ファイル内で正確な文字列置換を実行します。

<h3 id="read">
  Read
</h3>

**ツール名：** `Read`

```typescript theme={null}
type FileReadInput = {
  file_path: string;
  offset?: number;
  limit?: number;
  pages?: string;
};
```

テキスト、画像、PDF、Jupyter ノートブックを含むローカルファイルシステムからファイルを読み取ります。PDF ページ範囲には `pages` を使用します（例：`"1-5"`）。

PDF の場合、Claude は Read 呼び出しの `tool_result` コンテンツ内のファイルの内容を受け取ります。`pdf` [出力](#tool-output-types) を返す読み取りは、サマリー `text` ブロックの後に `document` ブロックを含みます。`parts` 出力を返す読み取りは、サマリー `text` ブロックの後に抽出されたページごとに 1 つのブロックを含みます。`image` ブロック、または Claude Code がそれを画像としてレンダリングできなかった場合はページを名前付けする `text` ブロック。Agent SDK v0.3.242 より前は、Claude Code はファイルの内容をツール結果の後の別の `user` メッセージとして配信していました。

<h3 id="write">
  Write
</h3>

**ツール名：** `Write`

```typescript theme={null}
type FileWriteInput = {
  file_path: string;
  content: string;
};
```

ローカルファイルシステムにファイルを書き込み、存在する場合は上書きします。

<h3 id="glob">
  Glob
</h3>

**ツール名：** `Glob`

```typescript theme={null}
type GlobInput = {
  pattern: string;
  path?: string;
};
```

任意のコードベースサイズで機能する高速ファイルパターンマッチング。

<h3 id="grep">
  Grep
</h3>

**ツール名：** `Grep`

```typescript theme={null}
type GrepInput = {
  pattern: string;
  path?: string;
  glob?: string;
  type?: string;
  output_mode?: "content" | "files_with_matches" | "count";
  "-i"?: boolean;
  "-o"?: boolean; // マッチした部分のみを各行から出力；output_mode: "content" が必要
  "-n"?: boolean;
  "-B"?: number;
  "-A"?: number;
  "-C"?: number;
  context?: number;
  head_limit?: number;
  offset?: number;
  multiline?: boolean;
};
```

ripgrep に基づいた正規表現サポート付きの強力な検索ツール。

<h3 id="taskstop">
  TaskStop
</h3>

**ツール名：** `TaskStop`

```typescript theme={null}
type TaskStopInput = {
  task_id?: string;
  shell_id?: string; // 非推奨：task_id を使用
};
```

ID でバックグラウンドタスクまたはシェルを停止します。v2.1.198 以降、`task_id` はエージェントチームのチームメイト、またはエージェント ID または名前で名前付きバックグラウンドエージェントも受け入れます。

<h3 id="notebookedit">
  NotebookEdit
</h3>

**ツール名：** `NotebookEdit`

```typescript theme={null}
type NotebookEditInput = {
  notebook_path: string;
  cell_id?: string;
  new_source: string;
  cell_type?: "code" | "markdown";
  edit_mode?: "replace" | "insert" | "delete";
};
```

Jupyter ノートブックファイルのセルを編集します。

<h3 id="webfetch">
  WebFetch
</h3>

**ツール名：** `WebFetch`

```typescript theme={null}
type WebFetchInput = {
  url: string;
  prompt: string;
};
```

URL からコンテンツを取得し、AI モデルで処理します。

<h3 id="websearch">
  WebSearch
</h3>

**ツール名：** `WebSearch`

```typescript theme={null}
type WebSearchInput = {
  query: string;
  allowed_domains?: string[];
  blocked_domains?: string[];
};
```

ウェブを検索し、フォーマットされた結果を返します。

<h3 id="workflow">
  Workflow
</h3>

**ツール名：** `Workflow`

```typescript theme={null}
type WorkflowInput = {
  script?: string;
  name?: string;
  scriptPath?: string;
  args?: unknown; // 任意の JSON 値；公開された型付けはこれをオブジェクトマップとしてレンダリングします
  resumeFromRunId?: string;
  title?: string; // 無視されます；スクリプトのメタブロックがタイトルを設定します
  description?: string; // 無視されます；スクリプトのメタブロックが説明を設定します
};
```

[動的ワークフロー](/docs/ja/workflows) を実行します。これは多くのサブエージェントをバックグラウンドで調整し、1 つの統合結果を返すスクリプトです。`Workflow` ツールは Agent SDK v0.3.149 以降で利用可能です。`script`、`name`、または `scriptPath` の少なくとも 1 つが必要です。

| フィールド             | 型         | 説明                                                                                                                                                                                                                    |
| ----------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `script`          | `string`  | インラインワークフロースクリプト。リテラルとして `export const meta = { name, description }` で始まり、その後に `agent()`、`parallel()`、`pipeline()`、および `phase()` を使用するスクリプト本体が続く必要があります。`meta` 内のオプションの `phases` 配列は、進捗ビューで名前付きステージの下にエージェントをグループ化します |
| `name`            | `string`  | 組み込みワークフローまたは `.claude/workflows/` に保存されたワークフローの名前。スクリプトに解決されます                                                                                                                                                       |
| `scriptPath`      | `string`  | ディスク上のワークフロースクリプトファイルへのパス。`script` と `name` より優先されます。Claude Code はすべての呼び出しのスクリプトを永続化し、結果でパスを返すため、そのファイルを編集して同じ `scriptPath` で再度呼び出して反復処理できます                                                                          |
| `args`            | `unknown` | スクリプトにグローバル `args` として公開される入力値。研究質問またはファイルパスのリストなど、パラメータ化された名前付きワークフロー用です。配列とオブジェクトを JSON エンコード文字列ではなく実際の JSON 値として渡します                                                                                               |
| `resumeFromRunId` | `string`  | 再開する前の `Workflow` 呼び出しの実行 ID。変更されていない入力を持つ完了した `agent()` 呼び出しは通常キャッシュされた結果を返します。残りは実行されます。[一時停止後に再開](/docs/ja/workflows#resume-after-a-pause) は、どの完了した呼び出しが再実行されるかをカバーしています。同じセッションのみ                                      |
| `title`           | `string`  | 無視されます；スクリプトの `meta` ブロックがタイトルを設定します                                                                                                                                                                                  |
| `description`     | `string`  | 無視されます；スクリプトの `meta` ブロックが説明を設定します                                                                                                                                                                                    |

<h3 id="todowrite">
  TodoWrite
</h3>

**ツール名：** `TodoWrite`

```typescript theme={null}
type TodoWriteInput = {
  todos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
};
```

進捗を追跡するための構造化タスクリストを作成および管理します。

<Note>
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  [モデルの可用性](/docs/ja/agent-sdk/todo-tracking#model-availability) を参照してオプトインしてください。
</Note>

<h3 id="taskcreate">
  TaskCreate
</h3>

**ツール名：** `TaskCreate`

```typescript theme={null}
type TaskCreateInput = {
  subject: string;
  description: string;
  activeForm?: string;
  metadata?: Record<string, unknown>;
};
```

単一のタスクを作成し、割り当てられた ID を返します。

<h3 id="taskupdate">
  TaskUpdate
</h3>

**ツール名：** `TaskUpdate`

```typescript theme={null}
type TaskUpdateInput = {
  taskId: string;
  status?: "pending" | "in_progress" | "completed" | "deleted";
  subject?: string;
  description?: string;
  activeForm?: string;
  addBlocks?: string[];
  addBlockedBy?: string[];
  owner?: string;
  metadata?: Record<string, unknown>;
};
```

ID でタスクを 1 つパッチします。`status` を `"deleted"` に設定して削除します。

<h3 id="taskget">
  TaskGet
</h3>

**ツール名：** `TaskGet`

```typescript theme={null}
type TaskGetInput = {
  taskId: string;
};
```

1 つのタスクの完全な詳細を返すか、ID が見つからない場合は `null` を返します。

<h3 id="tasklist">
  TaskList
</h3>

**ツール名：** `TaskList`

```typescript theme={null}
type TaskListInput = {};
```

現在のリストのすべてのタスクのスナップショットを返します。

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**ツール名：** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeInput = {
  /** 非推奨：使用されなくなりました。 */
  allowedPrompts?: Array<{
    tool: "Bash";
    prompt: string;
  }>;
  [k: string]: unknown;
};
```

計画モードを終了します。`allowedPrompts` フィールドは非推奨で無視されます。Claude Code は既存の呼び出し元とトランスクリプトが検証されるようにそれでも受け入れます。v2.1.205 より前は、計画を実装するためのプロンプトベースの Bash パーミッションをリクエストしていました。

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**ツール名：** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesInput = {
  server?: string;
};
```

接続されたサーバーから利用可能な MCP リソースをリストします。

<h3 id="readmcpresource">
  ReadMcpResource
</h3>

**ツール名：** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceInput = {
  server: string;
  uri: string;
};
```

サーバーから特定の MCP リソースを読み取ります。

<h3 id="enterworktree">
  EnterWorktree
</h3>

**ツール名：** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeInput = {
  name?: string;
  path?: string;
};
```

分離された作業用の一時的な git worktree を作成して入力します。新しい worktree を作成する代わりに、既存の worktree に切り替えるには `path` を渡します。最初の入力時、ターゲットは現在のリポジトリの登録済み worktree、またはマルチリポジトリワークスペースの場合はその中にネストされたリポジトリの worktree である必要があります。worktree セッション内からは、セッションのリポジトリの `.claude/worktrees/` の下にある必要があります。`name` と `path` は相互に排他的です。

<h3 id="exitworktree">
  ExitWorktree
</h3>

**ツール名：** `ExitWorktree`

```typescript theme={null}
type ExitWorktreeInput = {
  action: "keep" | "remove";
  discard_changes?: boolean;
};
```

現在の git worktree を終了し、元の作業ディレクトリに戻ります。`keep` アクションは worktree とブランチをディスク上に残し、`remove` は両方を削除します。`discard_changes` は、コミットされていないファイルまたはマージされていないコミットを持つ worktree を削除する場合は `true` である必要があります。

<h3 id="enterplanmode">
  EnterPlanMode
</h3>

**ツール名：** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeInput = {};
```

計画モードに入ります。Claude は変更を加える前に計画を研究して提示します。

<h3 id="croncreate">
  CronCreate
</h3>

**ツール名：** `CronCreate`

```typescript theme={null}
type CronCreateInput = {
  cron: string;
  prompt: string;
  recurring?: boolean;
  durable?: boolean;
};
```

ローカル時間の 5 フィールド cron スケジュールでプロンプトを実行するようにスケジュールします。`recurring` を `false` に設定して、次のマッチで 1 回だけ実行します。ジョブはデフォルトではセッションスコープです。新しい会話を開始するとジョブがクリアされ、`--resume` または `--continue` で再開するとまだ期限切れになっていないジョブが復元されます。[スケジュール済みタスク](/docs/ja/scheduled-tasks) を参照してください。

`durable` を `true` に設定すると、`.claude/scheduled_tasks.json` への永続化をリクエストするため、ジョブは再起動後も存続します。耐久性のあるスケジューリングはすべてのセッションで利用できるわけではありません。利用できない場合、Claude Code は `durable: true` を受け入れますがジョブをセッションのみで作成します。出力の `durable` フィールドを読んで、ジョブが永続化されたかどうかを確認してください。

<h3 id="crondelete">
  CronDelete
</h3>

**ツール名：** `CronDelete`

```typescript theme={null}
type CronDeleteInput = {
  id: string;
};
```

`CronCreate` から返された ID でスケジュール済み cron ジョブを削除します。

<h3 id="cronlist">
  CronList
</h3>

**ツール名：** `CronList`

```typescript theme={null}
type CronListInput = {};
```

スケジュール済み cron ジョブをリストします。`.claude/scheduled_tasks.json` からの耐久性のあるジョブと現在のセッションからのセッションのみのジョブ。

<h3 id="schedulewakeup">
  ScheduleWakeup
</h3>

**ツール名：** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupInput = {
  delaySeconds?: number;
  reason?: string;
  prompt?: string;
  noop?: boolean;
  stop?: boolean;
};
```

遅延後に指定されたプロンプトを実行する 1 回限りのウェイクアップをスケジュールします。このツールは自分のペースで進める `/loop` コマンドをサポートしています。ランタイムは `delaySeconds` を 60 ～ 3600 秒の間にクランプします。`delaySeconds`、`reason`、`prompt`、および `noop` フィールドは、`stop` が true でない限り必須です。`noop: true` は何も変わらなかったウェイクアップを報告します。`stop: true` を設定すると、保留中のウェイクアップをキャンセルし、自分のペースで進める `/loop` を終了します。`stop` フィールドには Claude Code v2.1.202 以降が必要です。[ツールリファレンスの ScheduleWakeup 行](/docs/ja/tools-reference) を参照してください。

<h3 id="remotetrigger">
  RemoteTrigger
</h3>

**ツール名：** `RemoteTrigger`

```typescript theme={null}
type RemoteTriggerInput = {
  action:
    | "list"
    | "get"
    | "create"
    | "update"
    | "run"
    | "create_webhook_trigger"
    | "list_runs"
    | "get_run_log";
  trigger_id?: string;
  session_id?: string;
  cursor?: string;
  body?: {
    [k: string]: unknown;
  };
};
```

クラウドでホストされているスケジュール済みおよびトリガーされた Claude Code 実行である [Routines](/docs/ja/routines) を管理します。このツールは `/schedule` コマンドをサポートしています。`trigger_id` は `get`、`update`、`run`、および `list_runs` アクションに必須です。`body` は `create`、`update`、および `create_webhook_trigger` に必須で、`run` ではオプションです。

`create_webhook_trigger` は、[GitHub イベント](/docs/ja/routines#add-a-github-trigger) など、既存のルーチンにイベントソースをアタッチして実行します。`body` はソース、イベント、および実行するルーチンに名前を付けます。Claude Code v2.1.225 以降が必要です。

`list_runs` はルーチンの最近の実行をリストし、`get_run_log` は 1 つの実行のログを読みます。`session_id` は `list_runs` 結果から読むべき実行に名前を付け、`cursor` はいずれかのアクションの結果をページングします。両方のアクションには Claude Code v2.1.227 以降が必要です。

このツールは、セッションが Routines が有効なプランで claude.ai アカウントで認証されている場合にのみ利用可能で、組織のポリシーが [クラウドセッション](/docs/ja/claude-code-on-the-web) を無効にしている場合は存在しません。Claude Code v2.1.227 以降では、Owner が [組織のルーチンをオフにした](/docs/ja/routines#routines-are-disabled-by-your-organizations-policy) 場合、ツールも存在しません。v2.1.227 より前は、ルーチンの切り替えのみがオフになっているセッションでもツールが表示され、サーバーはその呼び出しを拒否していました。

<h3 id="pushnotification">
  PushNotification
</h3>

**ツール名：** `PushNotification`

```typescript theme={null}
type PushNotificationInput = {
  message: string;
  status: "proactive";
};
```

ユーザーにプロアクティブなプッシュ通知を送信します。モバイルオペレーティングシステムがより長いテキストを切り詰めるため、`message` を 200 文字以下に保ちます。[ツールリファレンスの PushNotification 行](/docs/ja/tools-reference) でプロバイダーの可用性を参照してください。プッシュ配信は Anthropic でホストされたインフラストラクチャを通じて実行されます。Amazon Bedrock、Claude Platform on AWS、Google Cloud の Agent Platform、または Microsoft Foundry からはアクセスできません。

<h3 id="repl">
  REPL
</h3>

v2.1.275 で削除されました。v2.1.274 までは、実験的な `REPL` ツールを [`env` オプション](#options) で `CLAUDE_CODE_REPL=1` を設定することでオンにできました。

<h3 id="reportfindings">
  ReportFindings
</h3>

**ツール名：** `ReportFindings`

```typescript theme={null}
type ReportFindingsInput = {
  level?: "low" | "medium" | "high" | "xhigh" | "max";
  findings: Array<{
    file: string;
    line?: number;
    summary: string;
    failure_scenario: string;
    short_summary?: string;
    category?: string;
    verdict?: "CONFIRMED" | "PLAUSIBLE";
    outcome?: "fixed" | "skipped" | "no_change_needed";
  }>;
};
```

コードレビューの検出結果を構造化リストとしてレポートするため、Claude Code はテキストとして出力する代わりにレンダリングできます。`level` はレビューが実行された努力レベルです。検出結果は最も重大度の高い順に並べられ、呼び出しごとに最大 32 個で、配列は何も生き残らなかった場合は空です。Claude Code v2.1.196 以降が必要です。

各検出結果には以下のフィールドが含まれます：

* `file`：検出結果が含まれるリポジトリ相対パス。オプションの `line` はそれがアンカーされている 1 インデックス付きの行です。
* `summary`：欠陥の 1 文の説明。`failure_scenario` は、間違った出力またはクラッシュにつながる具体的な入力と状態を説明します。
* `short_summary`：コンパクト表示用の最大 60 文字の圧縮ラベル（オプション）。Claude Code v2.1.212 以降が必要です。
* `category`：`correctness` または `test-coverage` などの検出結果タイプの短いケバブケーススラッグ（オプション）。Claude Code v2.1.199 以降が必要です。
* `verdict`：検証パスが実行されたときに設定；インラインのみのレビューでは存在しません。
* `outcome`：修正を適用した後に再度レポートする場合にのみ設定されます。

<h3 id="artifact">
  Artifact
</h3>

**ツール名：** `Artifact`

```typescript theme={null}
type ArtifactInput = {
  action?: "publish" | "list";
  file_path?: string;
  favicon?: string;
  icon?: string;
  limit?: number;
  scope?: "mine" | "shared" | "all";
  title?: string;
  description?: string;
  label?: string;
  url?: string;
  force?: boolean;
  capabilities?: Record<string, unknown>;
  contract?: "latest" | string;
};
```

ローカル `.html` または `.md` ファイルをホストされたアーティファクトページとして公開するか、ユーザーの公開されたアーティファクトをリストします。`action` を省略するか `"publish"` を渡して `file_path` を公開します。これは公開アクションに必須です。以下の各フィールドは公開に適用されます：

* `icon`：アーティファクトのブラウザタブアイコンの 1 つの短い一般的な単語（`chart` や `map` など）。Claude は最初の公開時にそれを含め、更新時に省略します。これはアーティファクトの保存されたアイコンを保持します。
* `favicon`：非推奨で、Claude は省略します。
* `title`：HTML ファイルに `<title>` タグがない場合、ブラウザタブとギャラリーで公開されたページに名前を付けます。
* `url`：新しいページを作成する代わりに、既存のアーティファクトをその場で更新するターゲット。

`force` は、別のセッションが公開した新しいバージョンを破棄する最後の手段の上書きです。競合時に、失敗した公開はより新しいコンテンツを返します。Claude はその変更をそのコンテンツにマージするか、アーティファクトを再度読み取り、再度公開します。ユーザーがそのバージョンを明示的に破棄するよう求めた場合にのみ `force` を渡します。

`"list"` を渡してユーザーの公開されたアーティファクトを列挙します。`limit` と `scope` のみがそれに付随する場合があります。`scope` はデフォルトで `"mine"` で、ユーザーが所有するアーティファクトをリストします。`"shared"` は他の人がユーザーと共有したアーティファクトをリストし、`"all"` は両方をリストします。

* `capabilities`：公開されたページが使用するランタイム機能。機能名でキー付けされます。例えば、[ページが呼び出す可能性があるコネクタ](/docs/ja/artifacts#pull-live-data-with-mcp-connectors)。アーティファクトサービスは宣言を検証し、アカウントが使用できない機能に名前を付けるか、無効な設定を与える公開を拒否します。`{}` を渡して保存された宣言をクリアし、再デプロイ時にフィールドを省略して保持します。Agent SDK v0.3.235 以降が必要です。
* `contract`：公開されたページが実行されるランタイムバージョン。省略してアーティファクトの現在のバージョンを保持し、`"latest"` を渡してアップグレードするか、特定のバージョンを渡してピン留めまたはロールバックします。Agent SDK v0.3.235 以降が必要です。

型はエクスポートされていますが、ツールは Agent SDK セッションではデフォルトでオフになっています。公開には、[アーティファクト可用性テーブル](/docs/ja/artifacts#availability) のすべての条件も必要です。API キーで認証されたセッションはこれらを満たしていません。

<h3 id="projects">
  Projects
</h3>

**ツール名：** `Projects`

```typescript theme={null}
type ProjectsInput = {
  method:
    | "project_info"
    | "project_read"
    | "project_search"
    | "project_write"
    | "project_delete";
  path?: string;
  content?: string;
  local_path?: string;
  present_to_user?: boolean;
  query?: string;
  n?: number;
};
```

セッションにアタッチされた claude.ai Project を読み書きします。`method` でディスパッチします：

* `project_info`：プロジェクトメタデータとドキュメントリストを返します。
* `project_read`：`path` で 1 つのドキュメントを読みます。
* `project_search`：`query` でプロジェクトのナレッジベースをクエリします。`n` はヒット数をキャップし、デフォルトは `5` です。
* `project_write`：`content`（インラインテキストを含む）または `local_path`（作業ディレクトリ内のファイルに名前を付ける）のいずれか正確に 1 つから `path` でドキュメントを作成または置換します。`present_to_user: true` は、書き込まれたドキュメントをユーザーが見る必要がある成果物としてマークします。
* `project_delete`：`path` でドキュメントを削除します。

<h3 id="readmcpresourcedir">
  ReadMcpResourceDir
</h3>

**ツール名：** `ReadMcpResourceDirTool`

```typescript theme={null}
type ReadMcpResourceDirInput = {
  server: string;
  uri: string;
};
```

MCP サーバー上のディレクトリリソースの直接の子をリストします。ディレクトリリスティングのサポートを宣言したサーバーに対してのみ使用可能です。リスティングは再帰的ではありません。ディレクトリリスティングはすべてのセッションで有効になっているわけではありません。オフの場合、呼び出しは空の `resources` リストを返し、`error` フィールドはディレクトリリスティングが有効になっていないことを報告します。

<h3 id="refreshmcptools">
  RefreshMcpTools
</h3>

**ツール名：** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsInput = {
  server?: string; // このサーバーのみをリフレッシュ；すべての接続されたサーバーをリフレッシュするには省略
};
```

接続されたMCPサーバーのツールリストを再度クエリし、変更を適用します。型はエクスポートされていますが、Claude Code は [`env` オプション](#options) で `CLAUDE_CODE_ENABLE_REFRESH_MCP_TOOLS=1` を設定し、少なくとも 1 つの MCP サーバーがあるセッションでのみツールを登録します。Claude Code v2.1.211 以降が必要です。

<h3 id="showonboardingrolepicker">
  ShowOnboardingRolePicker
</h3>

**ツール名：** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerInput = {};
```

Cowork オンボーディング中にクリック可能なロールピッカーチップ行をレンダリングするため、ユーザーはロールを選択して一致するプラグインをインストールできます。引数は不要です。ロールリストはクライアントで定義されます。呼び出しはユーザーが応答するまでブロックされます。

<h3 id="mcpinput">
  McpInput
</h3>

**ツール名：** `mcp__<server>__<tool>` の形式の動的 MCP ツール名

```typescript theme={null}
type McpInput = {
  [k: string]: unknown;
};
```

MCP ツール引数はオープンオブジェクトです。各サーバーは独自のパラメーターを定義するため、型はフィールド名または値に制約を課しません。特定のツールが受け入れるフィールドについては、サーバー独自のツールスキーマを参照してください。

<h2 id="tool-output-types">
  ツール出力型
</h2>

すべての組み込み Claude Code ツールの出力スキーマのドキュメント。これらの型は `@anthropic-ai/claude-agent-sdk` からエクスポートされ、各ツールによって返される実際の応答データを表します。

<h3 id="tooloutputschemas">
  `ToolOutputSchemas`
</h3>

`@anthropic-ai/claude-agent-sdk` からエクスポートされたツール出力型の共用体。メンバーには以下が含まれます：

```typescript theme={null}
type ToolOutputSchemas =
  | AgentOutput
  | ArtifactOutput
  | AskUserQuestionOutput
  | BashOutput
  | CronCreateOutput
  | CronDeleteOutput
  | CronListOutput
  | EnterPlanModeOutput
  | EnterWorktreeOutput
  | ExitPlanModeOutput
  | ExitWorktreeOutput
  | FileEditOutput
  | FileReadOutput
  | FileWriteOutput
  | GlobOutput
  | GrepOutput
  | ListMcpResourcesOutput
  | McpOutput
  | MonitorOutput
  | NotebookEditOutput
  | ProjectsOutput
  | PushNotificationOutput
  | ReadMcpResourceDirOutput
  | ReadMcpResourceOutput
  | RefreshMcpToolsOutput
  | RemoteTriggerOutput
  | ReportFindingsOutput
  | ScheduleWakeupOutput
  | ShowOnboardingRolePickerOutput
  | TaskCreateOutput
  | TaskGetOutput
  | TaskListOutput
  | TaskStopOutput
  | TaskUpdateOutput
  | TodoWriteOutput
  | WebFetchOutput
  | WebSearchOutput
  | WorkflowOutput;
```

<h3 id="agent-2">
  Agent
</h3>

**ツール名：** `Agent`。以前の名前 `Task` はまだエイリアスとして受け入れられており、[`SDKSystemMessage`](#sdksystemmessage) 初期化メッセージの `tools` 配列は、後方互換性のためこのツールを `Task` として現在リストしています。

```typescript theme={null}
type AgentOutput =
  | {
      status: "completed";
      agentId: string;
      agentType?: string;
      content: Array<{ type: "text"; text: string; citations?: unknown[] | null }>;
      resolvedModel?: string;
      modelsUsed?: string[];
      totalToolUseCount: number;
      totalDurationMs: number;
      totalTokens: number;
      usage: {
        input_tokens: number;
        output_tokens: number;
        cache_creation_input_tokens: number | null;
        cache_read_input_tokens: number | null;
        server_tool_use: {
          web_search_requests: number;
          web_fetch_requests: number;
        } | null;
        service_tier: string | null;
        cache_creation: {
          ephemeral_1h_input_tokens: number;
          ephemeral_5m_input_tokens: number;
        } | null;
        inference_geo?: string | null;
        speed?: string | null;
        iterations?: unknown;
        output_tokens_details?: {
          thinking_tokens?: number | null;
        } | null;
      };
      toolStats?: {
        readCount: number;
        searchCount: number;
        bashCount: number;
        editFileCount: number;
        linesAdded: number;
        linesRemoved: number;
        otherToolCount: number;
        frameCount?: number;
      };
      prompt: string;
      worktreePath?: string;
      worktreeBranch?: string;
    }
  | {
      status: "async_launched";
      isAsync?: true;
      agentId: string;
      description: string;
      resolvedModel?: string;
      modelsUsed?: string[];
      prompt: string;
      outputFile: string;
      canReadOutputFile?: boolean;
    }
  | {
      status: "remote_launched";
      taskId: string;
      sessionUrl: string;
      description: string;
      prompt: string;
      outputFile: string;
    };
```

サブエージェントからの結果を返します。`status` フィールドで判別されます：完了したタスクの場合は `"completed"`、バックグラウンドタスクの場合は `"async_launched"`、Claude Code がクラウドセッションにディスパッチしたタスクの場合は `"remote_launched"`。`sessionUrl` はそのセッションにリンクし、`taskId` はそれを識別します。

`completed` バリアントでは、`resolvedModel` はサブエージェントが開始したモデルに名前を付けます。これは、[`availableModels`](/docs/ja/model-config#restrict-model-selection) または別のオーバーライドが適用される場合、要求された `model` 入力と異なる場合があります。このフィールドには Claude Code v2.1.174 以降が必要です。`async_launched` では、タスクがバックグラウンドに移動したときに使用中のモデルに名前を付けます。

`modelsUsed` はサブエージェントが使用したモデルを順序で列挙します。このフィールドは、実行中のスワップが発生した場合にのみ存在し、実行がそれに戻った場合、モデルが再度表示されます。`async_launched` では、リストはバックグラウンド化前に使用されたモデルをカバーします。`modelsUsed` と `resolvedModel` のバックグラウンド化動作の両方には Claude Code v2.1.212 以降が必要です。

Claude Code が[サブエージェントの分離された worktree を保持](/docs/ja/worktrees#isolate-subagents-with-worktrees)した場合、`completed` 結果の `worktreePath` はそれを見つける場所です。`worktreeBranch` はそのブランチで、Claude Code が git で worktree を作成した場合に存在します。

Claude Code は `usage` と `totalTokens` をサブエージェントの最終 API リクエストから入力します。実行全体からではないため、`usage.service_tier` は API がそのリクエストで報告したサービスティア文字列です。存在する場合、`usage.output_tokens_details.thinking_tokens` はそのリクエストの出力トークンのうち思考トークンであった数です。`output_tokens_details` フィールドには TypeScript SDK v0.3.228 以降が必要です。これは Claude Code v2.1.228 をバンドルしています。

`usage.output_tokens_details` は意味において [`Usage.output_tokens_details`](#usage) と一致し、その最終リクエストにスコープされていますが、ここではそのすべてのレベルがオプションです。例えば `usage.output_tokens_details?.thinking_tokens ?? 0` のように、オブジェクトとフィールドの両方をガードしてください。直接読み取るのではなく。

v2.1.207 より前では、公開された型はより狭いものでした。`worktreePath`、`worktreeBranch`、`citations`、`toolStats.frameCount`、および `inference_geo`、`speed`、`iterations` 使用フィールドを省略し、`service_tier` を `"standard" | "priority" | "batch"` として型付けしていました。型がオプションとしてマークするフィールドは、以前のバージョンで記録された結果に存在しない場合があります。

<h3 id="askuserquestion-2">
  AskUserQuestion
</h3>

**ツール名：** `AskUserQuestion`

```typescript theme={null}
type AskUserQuestionOutput = {
  questions: Array<{
    question: string;
    header: string;
    options: Array<{ label: string; description: string; preview?: string }>;
    multiSelect: boolean;
  }>;
  answers: Record<string, string>;
  response?: string;
  annotations?: Record<string, { preview?: string; notes?: string }>;
  afkTimeoutMs?: number;
};
```

質問とユーザーの回答を返します。`response` はユーザーが構造化された質問に答える代わりに自由形式の返信を入力した場合に設定されます。存在する場合、Claude は質問ごとの回答リストの代わりに「ユーザーが応答しました：…」を受け取ります。

<h3 id="bash-2">
  Bash
</h3>

**ツール名：** `Bash`

```typescript theme={null}
type BashOutput = {
  stdout: string;
  stderr: string;
  rawOutputPath?: string;
  interrupted: boolean;
  isImage?: boolean;
  backgroundTaskId?: string;
  backgroundedByUser?: boolean;
  timedOutAfterMs?: number;
  backgroundCwdHint?: string;
  backgroundEndsWithFinalResponse?: true;
  dangerouslyDisableSandbox?: boolean;
  returnCodeInterpretation?: string;
  noOutputExpected?: boolean;
  structuredContent?: unknown[];
  persistedOutputPath?: string;
  persistedOutputSize?: number;
  staleReadFileStateHint?: string;
  ghRateLimitHint?: string;
  gitOperation?: {
    commit?: { sha: string; kind: "committed" | "amended" | "cherry-picked"; branch?: string };
    push?: { branch: string };
    branch?: { ref: string; action: "merged" | "rebased" };
    pr?: {
      number: number;
      url?: string;
      action: "created" | "edited" | "merged" | "commented" | "closed" | "reopened" | "ready" | "draft" | "auto-merge-enabled" | "auto-merge-disabled";
    };
  };
};
```

`stdout`、`stderr`、`backgroundTaskId` フィールドは以下を保持します：

| フィールド              | 保持内容                                                 |
| ------------------ | ---------------------------------------------------- |
| `stdout`           | コマンドの stdout と stderr。1 つのインターリーブされたストリームにマージされます    |
| `stderr`           | シェルの作業ディレクトリリセットなど、ツール自体が追加する通知。コマンドの stderr ではありません |
| `backgroundTaskId` | バックグラウンドコマンドの場合に存在します                                |

`timedOutAfterMs` はタイムアウト（ミリ秒）で、コマンドがタイムアウトに達し、明示的に開始するのではなくバックグラウンドに移動した場合に設定されます。`backgroundCwdHint` はバックグラウンド化されたコマンドに `cd`、`pushd`、`popd`、`chdir` などのディレクトリ変更組み込みが含まれていた場合に設定され、セッション作業ディレクトリが変更されなかったことに注意します。両方のフィールドには Claude Code v2.1.210 以降が必要です。

フォアグラウンドで実行されているサブエージェントがバックグラウンド化されたコマンドを所有している場合、Claude Code はそのサブエージェントが最終応答を与えるときにコマンドを終了します。Claude Code はそのようなコマンドに `backgroundEndsWithFinalResponse` を `true` に設定し、メインの会話またはバックグラウンドサブエージェントによって開始されたコマンドのようにコマンドが有効期間を超える場合、フィールドを省略します。このフィールドには Claude Code v2.1.227 以降が必要です。

Claude Code は `gitOperation.commit.branch` を git のコミット概要行で名前が付けられたブランチに設定し、デタッチされた HEAD でコミットされたコミットの場合は省略します。このフィールドには Agent SDK v0.3.227 以降が必要です。Claude Code は `gh pr reopen` コマンドを `reopened` PR アクションとして報告します。これには Agent SDK v0.3.234 以降が必要です。

<h3 id="monitor-2">
  Monitor
</h3>

**ツール名：** `Monitor`

```typescript theme={null}
type MonitorOutput = {
  taskId: string;
  timeoutMs: number;
  persistent?: boolean;
};
```

実行中のモニターのバックグラウンドタスク ID を返します。この ID を `TaskStop` で使用して、ウォッチを早期にキャンセルします。

<h3 id="edit-2">
  Edit
</h3>

**ツール名：** `Edit`

```typescript theme={null}
type FileEditOutput = {
  filePath: string;
  oldString: string;
  newString: string;
  originalFile: string | null;
  structuredPatch: Array<{
    oldStart: number;
    oldLines: number;
    newStart: number;
    newLines: number;
    lines: string[];
  }>;
  userModified: boolean;
  replaceAll: boolean;
  gitDiff?: {
    filename: string;
    status: "modified" | "added";
    additions: number;
    deletions: number;
    changes: number;
    patch: string;
    repository?: string | null;
  };
};
```

編集操作の構造化された diff を返します。

<h3 id="read-2">
  Read
</h3>

**ツール名：** `Read`

```typescript theme={null}
type FileReadOutput =
  | {
      type: "text";
      file: {
        filePath: string;
        content: string;
        numLines: number;
        startLine: number;
        totalLines: number;
        /** True when a whole-file read was auto-paginated because it exceeded the token cap (the content is a partial first page). */
        truncatedByTokenCap?: boolean;
      };
    }
  | {
      type: "image";
      file: {
        base64: string;
        type: "image/jpeg" | "image/png" | "image/gif" | "image/webp";
        originalSize: number;
        dimensions?: {
          originalWidth?: number;
          originalHeight?: number;
          displayWidth?: number;
          displayHeight?: number;
        };
      };
    }
  | {
      type: "notebook";
      file: {
        filePath: string;
        cells: unknown[];
      };
    }
  | {
      type: "pdf";
      file: {
        filePath: string;
        base64: string;
        originalSize: number;
      };
    }
  | {
      type: "parts";
      file: {
        filePath: string;
        originalSize: number;
        count: number;
        outputDir: string;
      };
      /** Document page number of the first extracted page; labels the page images in the tool_result content. */
      firstPage?: number;
      /** In-process only: the page-image bytes are delivered as image blocks in the tool_result content and aren't retained on the emitted tool_use_result, so this key is absent there. */
      pages?: {
        base64: string;
        mediaType: "image/jpeg" | "image/png" | "image/gif" | "image/webp";
        error?: string;
      }[];
    }
  | {
      type: "file_unchanged";
      file: {
        filePath: string;
      };
      /** Set when the dedup matched a startup-seeded entry (CLAUDE.md / nested memory) rather than a prior Read tool_result. */
      source?: "seeded";
    };
```

ファイルタイプに適切な形式でファイルコンテンツを返します。`type` フィールドで判別されます。

<h3 id="write-2">
  Write
</h3>

**ツール名：** `Write`

```typescript theme={null}
type FileWriteOutput = {
  type: "create" | "update";
  filePath: string;
  content: string;
  structuredPatch: Array<{
    oldStart: number;
    oldLines: number;
    newStart: number;
    newLines: number;
    lines: string[];
  }>;
  originalFile: string | null;
  gitDiff?: {
    filename: string;
    status: "modified" | "added";
    additions: number;
    deletions: number;
    changes: number;
    patch: string;
    repository?: string | null;
  };
  userModified?: boolean;
};
```

構造化された diff 情報を含む書き込み結果を返します。`originalFile` と `structuredPatch` が保持する内容は書き込みによって異なります：

* 新しく作成されたファイルの場合、`originalFile` は null で `structuredPatch` は空です
* 上書きの場合、`originalFile` は前のコンテンツを保持します。ただし、そのコンテンツが約 10 MB より大きい場合は除きます。Claude Code は diff をスキップし、`originalFile` null と `structuredPatch` 空を返します
* 書き込みが何も変更しなかった場合、または diff がタイムアウトした場合、`structuredPatch` も空です

<h3 id="glob-2">
  Glob
</h3>

**ツール名：** `Glob`

```typescript theme={null}
type GlobOutput = {
  durationMs: number;
  numFiles: number;
  filenames: string[];
  truncated: boolean;
  totalMatches?: number;
  countIsComplete?: boolean;
};
```

glob パターンに一致するファイルパスを返します。変更時刻でソートされます。

`totalMatches` と `countIsComplete` には Claude Code v2.1.191 以降が必要です。`totalMatches` は切り詰め前のマッチングファイル数を報告します。`countIsComplete` が false の場合、基になる検索が独自の出力を切り詰めたため、`totalMatches` は下限です。

<h3 id="grep-2">
  Grep
</h3>

**ツール名：** `Grep`

```typescript theme={null}
type GrepOutput = {
  mode?: "content" | "files_with_matches" | "count";
  numFiles: number;
  filenames: string[];
  content?: string;
  numLines?: number;
  numMatches?: number;
  totalFiles?: number;
  totalLines?: number;
  appliedLimit?: number;
  appliedOffset?: number;
};
```

検索結果を返します。形状は `mode` によって異なります：ファイルリスト、マッチを含むコンテンツ、またはマッチ数。`count` モードでは、`numFiles` と `numMatches` はページネーションされたスライスではなく、完全な結果セット全体の合計です。v2.1.208 より前では、リストされたエントリを切り詰めた `head_limit` または `offset` もそれらの合計を切り詰めていました。

`totalFiles` には Claude Code v2.1.208 以降が必要で、`files_with_matches` モードで `head_limit` と `offset` ページネーション前の結果の総数を報告します。`totalLines` には Claude Code v2.1.210 以降が必要で、`content` モードでページネーション前の行の総数を報告します。

<h3 id="taskstop-2">
  TaskStop
</h3>

**ツール名：** `TaskStop`

```typescript theme={null}
type TaskStopOutput = {
  message: string;
  task_id: string;
  task_type: string;
  command?: string;
};
```

バックグラウンドタスクを停止した後の確認を返します。

<h3 id="notebookedit-2">
  NotebookEdit
</h3>

**ツール名：** `NotebookEdit`

```typescript theme={null}
type NotebookEditOutput = {
  new_source: string;
  old_source?: string;
  cell_id?: string;
  cell_type: "code" | "markdown";
  language: string;
  edit_mode: string;
  error?: string;
  notebook_path: string;
  original_file: string;
  updated_file: string;
};
```

元のファイルと更新されたファイルコンテンツを含むノートブック編集の結果を返します。

<h3 id="webfetch-2">
  WebFetch
</h3>

**ツール名：** `WebFetch`

```typescript theme={null}
type WebFetchOutput = {
  bytes: number;
  code: number;
  codeText: string;
  result: string;
  durationMs: number;
  url: string;
  artifactRead?: {
    slug: string;
    ver?: string;
    seeded?: false;
  };
};
```

HTTP ステータスとメタデータを含む取得されたコンテンツを返します。

`artifactRead` は Claude Code 独自のアーティファクト読み取りの記録で、セッションが公開できるアーティファクトを Claude が取得した場合にのみ存在します。Claude Code はセッションが再開されるときにそれを読み戻すため、後の公開は正しいバージョンに基づいて構築されます。コードはそれに対して行動する必要はありません。`slug` はアーティファクトに名前を付け、`ver` は読み取りが記録したバージョンで、何も記録しなかった場合は存在しません。`seeded: false` は完全なソースが Claude に到達しなかった読み取りをマークします。`seeded` フィールドには Agent SDK v0.3.239 以降が必要です。

<h3 id="websearch-2">
  WebSearch
</h3>

**ツール名：** `WebSearch`

```typescript theme={null}
type WebSearchOutput = {
  query: string;
  results: Array<
    | {
        tool_use_id: string;
        content: Array<{ title: string; url: string }>;
      }
    | string
  >;
  durationSeconds: number;
  searchCount?: number;
};
```

ウェブからの検索結果を返します。

<h3 id="workflow-2">
  Workflow
</h3>

**ツール名：** `Workflow`

```typescript theme={null}
type WorkflowOutput = {
  status: "async_launched" | "remote_launched";
  taskId: string;
  taskType?: "local_workflow" | "remote_agent";
  workflowName?: string;
  runId?: string;
  summary?: string;
  transcriptDir?: string;
  scriptPath?: string;
  sessionUrl?: string; // set when the workflow launched as a cloud session
  warning?: string;
  error?: string;
};
```

ツールが呼び出しを受け入れた直後に返します。最終的な結果は後でタスク完了として到着します。実行が開始されたと見なす前に `error` を確認してください：構文チェックに失敗したスクリプトは `status: "async_launched"` と `error` セットで返され、実行されません。

| フィールド           | 型                                       | 説明                                                                                                |
| --------------- | --------------------------------------- | ------------------------------------------------------------------------------------------------- |
| `status`        | `"async_launched" \| "remote_launched"` | ツールが呼び出しを受け入れました。インプロセス実行の場合は `"async_launched"`、クラウドセッションにディスパッチされた実行の場合は `"remote_launched"`    |
| `taskId`        | `string`                                | 実行のバックグラウンドタスク識別子                                                                                 |
| `taskType`      | `"local_workflow" \| "remote_agent"`    | 登録されたバックグラウンドタスクのタスク型。`status` アームと一致します                                                          |
| `workflowName`  | `string`                                | ワークフロースクリプトの `meta.name`                                                                          |
| `runId`         | `string`                                | 後の呼び出しで `resumeFromRunId` として渡すワークフロー実行識別子。`remote_launched` 実行の場合は存在しません。クラウドセッション URL が再開ハンドルです |
| `summary`       | `string`                                | ワークフローが何をするかの 1 行の説明                                                                              |
| `transcriptDir` | `string`                                | 実行中にサブエージェントトランスクリプトが書き込まれるディレクトリ                                                                 |
| `scriptPath`    | `string`                                | この実行のために永続化されたワークフロースクリプトへのパス。編集して `scriptPath` として戻すことで、スクリプトを再送信せずに再実行できます                      |
| `sessionUrl`    | `string`                                | クラウドセッション URL。`status` が `"remote_launched"` の場合に設定されます                                           |
| `warning`       | `string`                                | ローカル git 状態がクラウドセッションがクローンするプッシュされたブランチから分岐するなど、ブロッキングされない注意                                      |
| `error`         | `string`                                | スクリプトが構文チェックに失敗した場合に設定されます。存在する場合、起動されたステータスにもかかわらず実行は開始されませんでした                                  |

<h3 id="todowrite-2">
  TodoWrite
</h3>

**ツール名：** `TodoWrite`

```typescript theme={null}
type TodoWriteOutput = {
  oldTodos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
  newTodos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
};
```

前のタスクリストと更新されたタスクリストを返します。

<Note>
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  [モデルの可用性](/docs/ja/agent-sdk/todo-tracking#model-availability)を参照してオプトインしてください。
</Note>

<h3 id="taskcreate-2">
  TaskCreate
</h3>

**ツール名：** `TaskCreate`

```typescript theme={null}
type TaskCreateOutput = {
  task: {
    id: string;
    subject: string;
  };
};
```

割り当てられた ID を持つ作成されたタスクを返します。

<h3 id="taskupdate-2">
  TaskUpdate
</h3>

**ツール名：** `TaskUpdate`

```typescript theme={null}
type TaskUpdateOutput = {
  success: boolean;
  taskId: string;
  updatedFields: string[];
  error?: string;
  statusChange?: {
    from: string;
    to: string;
  };
};
```

更新結果を返します。どのフィールドが変更されたかを含みます。

<h3 id="taskget-2">
  TaskGet
</h3>

**ツール名：** `TaskGet`

```typescript theme={null}
type TaskGetOutput = {
  task: {
    id: string;
    subject: string;
    description: string;
    status: "pending" | "in_progress" | "completed";
    blocks: string[];
    blockedBy: string[];
  } | null;
};
```

完全なタスクレコードを返します。ID が見つからない場合は `null` を返します。

<h3 id="tasklist-2">
  TaskList
</h3>

**ツール名：** `TaskList`

```typescript theme={null}
type TaskListOutput = {
  tasks: Array<{
    id: string;
    subject: string;
    status: "pending" | "in_progress" | "completed";
    owner?: string;
    blockedBy: string[];
  }>;
};
```

現在のリスト内のすべてのタスクのスナップショットを返します。

<h3 id="exitplanmode-2">
  ExitPlanMode
</h3>

**ツール名：** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeOutput = {
  plan: string | null;
  isAgent: boolean;
  filePath?: string;
  hasTaskTool?: boolean;
  planWasEdited?: boolean;
  awaitingLeaderApproval?: boolean;
  requestId?: string;
};
```

計画モード終了後の計画状態を返します。

<h3 id="listmcpresources-2">
  ListMcpResources
</h3>

**ツール名：** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesOutput = Array<{
  uri: string;
  name: string;
  mimeType?: string;
  description?: string;
  server: string;
}>;
```

利用可能な MCP リソースの配列を返します。

<h3 id="readmcpresource-2">
  ReadMcpResource
</h3>

**ツール名：** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceOutput = {
  contents: Array<{
    uri: string;
    mimeType?: string;
    text?: string;
    blobSavedTo?: string;
  }>;
  error?: string;
};
```

要求された MCP リソースのコンテンツを返します。

<h3 id="enterworktree-2">
  EnterWorktree
</h3>

**ツール名：** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeOutput = {
  worktreePath: string;
  worktreeBranch?: string;
  message: string;
};
```

git worktree に関する情報を返します。

<h3 id="exitworktree-2">
  ExitWorktree
</h3>

**ツール名：** `ExitWorktree`

```typescript theme={null}
type ExitWorktreeOutput = {
  action: "keep" | "remove";
  originalCwd: string;
  worktreePath: string;
  worktreeBranch?: string;
  tmuxSessionName?: string;
  discardedFiles?: number;
  discardedCommits?: number;
  message: string;
};
```

実行されたアクションと終了した worktree に関する詳細を返します。

<h3 id="enterplanmode-2">
  EnterPlanMode
</h3>

**ツール名：** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeOutput = {
  message: string;
};
```

計画モードが入力されたことの確認を返します。

<h3 id="croncreate-2">
  CronCreate
</h3>

**ツール名：** `CronCreate`

```typescript theme={null}
type CronCreateOutput = {
  id: string;
  humanSchedule: string;
  recurring: boolean;
  durable?: boolean; // true when persisted to .claude/scheduled_tasks.json; false when session-only
};
```

ジョブ ID とスケジュールの人間が読める説明を返します。

<h3 id="crondelete-2">
  CronDelete
</h3>

**ツール名：** `CronDelete`

```typescript theme={null}
type CronDeleteOutput = {
  id: string;
};
```

削除されたジョブの ID を返します。

<h3 id="cronlist-2">
  CronList
</h3>

**ツール名：** `CronList`

```typescript theme={null}
type CronListOutput = {
  jobs: {
    id: string;
    cron: string;
    humanSchedule: string;
    prompt: string;
    recurring?: boolean;
    durable?: boolean;
  }[];
};
```

スケジュール済み cron ジョブを返します：`.claude/scheduled_tasks.json` からの永続的なジョブと現在のセッションからのセッションのみのジョブ。セッションのみのジョブは `durable: false` を保持します。ディスクから読み取られたジョブはフィールドを省略します。

<h3 id="schedulewakeup-2">
  ScheduleWakeup
</h3>

**ツール名：** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupOutput = {
  scheduledFor: number;
  clampedDelaySeconds: number;
  wasClamped: boolean;
  stopped?: boolean;
  cancelledWakeups?: number;
};
```

ウェイクアップが発火する時刻をエポックミリ秒タイムスタンプとして、実際に使用された遅延、および要求された遅延がクランプされたかどうかを返します。`stopped` フィールドは `stop: true` でループを終了した呼び出しの場合は `true` です。Claude Code v2.1.202 以降が必要です。`cancelledWakeups` フィールドは `stop: true` 呼び出しがキャンセルした保留中のウェイクアップの数をカウントします。0 の値は何も保留されていないことを意味し、繰り返し `/loop` cron は `stop: true` でキャンセルされません。Claude Code v2.1.206 以降が必要です。

<h3 id="remotetrigger-2">
  RemoteTrigger
</h3>

**ツール名：** `RemoteTrigger`

```typescript theme={null}
type RemoteTriggerOutput = {
  status: number;
  json: string;
  summary?: string;
};
```

トリガー操作の API レスポンスステータスと本体を返します。

<h3 id="pushnotification-2">
  PushNotification
</h3>

**ツール名：** `PushNotification`

```typescript theme={null}
type PushNotificationOutput = {
  message: string;
  pushSent?: boolean;
  localSent?: boolean;
  disabledReason?: "config_off" | "user_present" | "no_transport";
  sentAt?: string;
};
```

配信の詳細を返します。プッシュまたはローカル通知が送信されたかどうか、および配信がスキップされた理由を含みます。

<h3 id="reportfindings-2">
  ReportFindings
</h3>

**ツール名：** `ReportFindings`

```typescript theme={null}
type ReportFindingsOutput = {
  count: number;
  level?: "low" | "medium" | "high" | "xhigh" | "max";
  findings: Array<{
    file: string;
    line?: number;
    summary: string;
    failure_scenario: string;
    short_summary?: string;
    category?: string;
    verdict?: "CONFIRMED" | "PLAUSIBLE";
    outcome?: "fixed" | "skipped" | "no_change_needed";
  }>;
};
```

報告された検出結果の数、レビューが実行された努力レベル、および結果本体のためにエコーバックされた検出結果を返します。Claude Code v2.1.196 以降が必要です。エコーバックされた `short_summary` フィールドには Claude Code v2.1.212 以降が必要です。

<h3 id="artifact-2">
  Artifact
</h3>

**ツール名：** `Artifact`

```typescript theme={null}
type ArtifactOutput =
  | {
      url: string;
      path: string;
      title?: string;
      version?: string;
      capabilities?: unknown;
      stored?: {
        contract: string;
        capabilities?: Record<string, unknown>;
      };
      warnings?: string[];
      contract?: string;
      updated?: boolean;
      liveSubscription?: string;
    }
  | {
      artifacts: Array<{
        title: string;
        url: string;
        updatedAt?: string;
        rel?: "mine" | "shared";
      }>;
      truncated?: boolean;
      scope?: "shared" | "all";
    };
```

公開されたページの `url` と公開されたローカル `path` を返します。公開が既存のアーティファクトを再デプロイした場合は `updated` が true に設定され、`warnings` は公開時の勧告を保持します。リストアクションは代わりに `artifacts` 行を返し、より多くのアーティファクトが要求された制限より存在する場合は `truncated` が設定されます。スコープが `"mine"` でないリストでは、各行は `rel` を保持し、ユーザーがアーティファクトを所有しているか、それが共有されているかをマークします。出力の `scope` はどの非デフォルトスコープがリストを生成したかを記録します。両方ともデフォルトリストに存在しません。

<h3 id="projects-2">
  Projects
</h3>

**ツール名：** `Projects`

```typescript theme={null}
type ProjectsOutput =
  | {
      method: "project_info";
      notice?: string;
      name: string;
      description: string;
      instructions: string;
      docs: Array<{ path: string; created_at: string | null }>;
      files?: Array<{
        path: string;
        file_kind: string;
        created_at: string | null;
      }>;
      sync_sources?: Array<{
        type: string | null;
        config: Record<string, unknown>;
      }>;
      knowledge: {
        knowledge_size: number;
        max_knowledge_size: number;
      };
    }
  | {
      method: "project_read";
      notice?: string;
      path: string;
      file_kind?: string;
      content?: string;
      local_file?: string;
      created_at: string | null;
    }
  | {
      method: "project_search";
      notice?: string;
      rag: boolean;
      hits?: Array<{ name?: string; doc_uuid?: string; text?: string }>;
      docs?: string[];
    }
  | {
      method: "project_write";
      notice?: string;
      path: string;
      doc_uuid: string;
      replaced: boolean;
      present_to_user?: boolean;
      local_path?: string;
    }
  | {
      method: "project_delete";
      notice?: string;
      path: string;
      deleted: boolean;
    };
```

`method` フィールドで判別され、入力をミラーリングします。`project_read` は小さいテキストドキュメントを `content` にインラインで返し、より大きなドキュメントを代わりに `local_file` パスに書き込みます。`project_search` はプロジェクトのインデックスが利用可能な場合は RAG `hits` を `rag: true` で返し、そうでない場合は `docs` パスリストにフォールバックします。

<h3 id="readmcpresourcedir-2">
  ReadMcpResourceDir
</h3>

**ツール名：** `ReadMcpResourceDirTool`

```typescript theme={null}
type ReadMcpResourceDirOutput = {
  resources: Array<{
    uri: string;
    name: string;
    mimeType?: string;
  }>;
  error?: string;
};
```

ディレクトリリソースの直接の子を返します。サブディレクトリは mimeType `"inode/directory"` で表示されます。`error` はサーバーがディレクトリをリストできなかった場合に人間が読める メッセージを保持します。

<h3 id="refreshmcptools-2">
  RefreshMcpTools
</h3>

**ツール名：** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsOutput = Array<{
  server: string;
  status: "refreshed" | "error" | "not_connected";
  toolCount?: number; // tools now available from this server
  added?: string[]; // tool names this refresh added
  removed?: string[]; // tool names this refresh removed
  error?: string; // why the refresh failed or the server was unavailable
}>;
```

サーバーごとに 1 つのエントリを返します：`refreshed` は再クエリされたツールリストが適用されたことを意味し、`error` は再クエリが失敗し、前のツールセットが保持されたことを意味し、`not_connected` はサーバーがクエリするライブ接続を持たないことを意味します。

<h3 id="showonboardingrolepicker-2">
  ShowOnboardingRolePicker
</h3>

**ツール名：** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerOutput = {
  role?: string;
  dismissed?: boolean;
};
```

ユーザーの選択を返します：ロールチップを選択したか入力した場合は `role`、ピッカーを閉じた場合は `dismissed: true`。空のオブジェクトはユーザーがロールを選択せずに呼び出しを承認したことを意味します。

<h3 id="mcpoutput">
  McpOutput
</h3>

**ツール名：** `mcp__<server>__<tool>` の形式の動的 MCP ツール名

```typescript theme={null}
type McpOutput =
  | string
  | {
      type: string;
      [k: string]: unknown;
    }[]
  | {
      [k: string]: unknown;
    };
```

MCP ツール結果はサーバーに応じて文字列またはコンテンツブロックの配列として返されます。エクスポートされた型の末尾のプレーンオブジェクトブランチはスキーマ生成アーティファクトです：SDK はベアオブジェクトを返しません。サーバーの構造化出力は返される前に JSON 文字列にシリアル化されるためです。実行時に値は `undefined` の場合もありますが、エクスポートされた型はこれをモデル化しません。

<h2 id="permission-types">
  パーミッション型
</h2>

<h3 id="permissionupdate">
  `PermissionUpdate`
</h3>

パーミッション更新の操作。

```typescript theme={null}
type PermissionUpdate =
  | {
      type: "addRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "replaceRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "removeRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "setMode";
      mode: PermissionMode;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "addDirectories";
      directories: string[];
      destination: PermissionUpdateDestination;
    }
  | {
      type: "removeDirectories";
      directories: string[];
      destination: PermissionUpdateDestination;
    };
```

<h3 id="permissionbehavior">
  `PermissionBehavior`
</h3>

```typescript theme={null}
type PermissionBehavior = "allow" | "deny" | "ask";
```

<h3 id="permissionupdatedestination">
  `PermissionUpdateDestination`
</h3>

```typescript theme={null}
type PermissionUpdateDestination =
  | "userSettings" // グローバルユーザー設定
  | "projectSettings" // ディレクトリごとのプロジェクト設定
  | "localSettings" // ローカルプロジェクト設定
  | "session" // 現在のセッションのみ
  | "cliArg"; // CLI 引数
```

<h3 id="permissionrulevalue">
  `PermissionRuleValue`
</h3>

```typescript theme={null}
type PermissionRuleValue = {
  toolName: string;
  ruleContent?: string;
};
```

<h2 id="other-types">
  その他のタイプ
</h2>

<h3 id="apikeysource">
  `ApiKeySource`
</h3>

セッションのリクエストに使用される API キーの出所です。[`SDKSystemMessage`](#sdksystemmessage) 初期化メッセージの `apiKeySource` として報告されます。

```typescript theme={null}
type ApiKeySource =
  | "ANTHROPIC_API_KEY"
  | "apiKeyHelper"
  | "/login managed key"
  | "none"
  | "user"
  | "project"
  | "org"
  | "temporary"
  | "oauth";
```

Claude Code は 4 つの値のいずれかを報告します。

| 値                    | 使用中のキー                                                                                                 |
| -------------------- | ------------------------------------------------------------------------------------------------------ |
| `ANTHROPIC_API_KEY`  | `ANTHROPIC_API_KEY` 環境変数内のキー                                                                           |
| `apiKeyHelper`       | [`apiKeyHelper`](/docs/ja/settings-reference#apikeyhelper) コマンドが返すキー                                        |
| `/login managed key` | [Claude Console アカウント](/docs/ja/authentication#claude-console-authentication)でログインしたときに Claude Code が保存したキー |
| `none`               | API キーなし。セッションは claude.ai ログイン、ベアラートークン、またはクラウドプロバイダーなど別の方法で認証します                                      |

Agent SDK v0.3.234 以降では、タイプ内にこれら 4 つの値がリストされます。タイプは `user`、`project`、`org`、`temporary`、`oauth` も保持しているため、古いコードは引き続きコンパイルされ、Claude Code はそれらを報告しません。

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

`betas` オプションで有効にできる利用可能なベータ機能です。詳細については、[Beta headers](https://platform.claude.com/docs/en/api/beta-headers) を参照してください。

```typescript theme={null}
type SdkBeta = "context-1m-2025-08-07";
```

<Warning>
  `context-1m-2025-08-07` ベータは 2026 年 4 月 30 日時点で廃止されました。Claude Sonnet 4.5 または Sonnet 4 でこの値を渡すと効果がなく、標準の 200k トークンコンテキストウィンドウを超えるリクエストはエラーを返します。1M トークンコンテキストウィンドウを使用するには、[Claude Opus 5.5、Claude Opus 5、Claude Sonnet 5、Claude Sonnet 4.6、Claude Opus 4.6、Claude Opus 4.7、または Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview) に移行してください。これらは標準価格で 1M コンテキストを含み、ベータヘッダーは不要です。
</Warning>

<h3 id="slashcommand">
  `SlashCommand`
</h3>

利用可能なコマンドに関する情報です。

```typescript theme={null}
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
  aliases?: string[];
  builtin?: boolean;
};
```

`builtin` は、コマンドが Claude Code 自体のものであり、`/name` を入力するとそれが実行される場合に `true` です。ユーザー、プロジェクト、プラグイン、または MCP サーバーによって定義されたコマンド、および [名前で置き換える](/docs/ja/skills#resolve-skills-that-share-a-name) バンドルされたコマンドの場合は存在しません。Agent SDK v0.3.277 以降が必要です。

<h3 id="modelinfo">
  `ModelInfo`
</h3>

利用可能なモデルに関する情報です。

```typescript theme={null}
type ModelInfo = {
  value: string;
  resolvedModel?: string;
  displayName: string;
  description: string;
  supportsEffort?: boolean;
  supportedEffortLevels?: ("low" | "medium" | "high" | "xhigh" | "max")[];
  supportsAdaptiveThinking?: boolean;
  supportsFastMode?: boolean;
  supportsAutoMode?: boolean;
};
```

| フィールド                      | タイプ                                                                | 説明                                                                                                                                                                             |
| :------------------------- | :----------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `value`                    | `string`                                                           | API 呼び出しで渡すモデル識別子                                                                                                                                                              |
| `resolvedModel`            | `string \| undefined`                                              | このエントリの `value` が解決される正規のワイヤモデル ID。`sonnet` などのエイリアスエントリは `claude-sonnet-5` などの明示的なモデル ID に解決されるため、ホストは保存された明示的なモデル ID をそれをカバーするエイリアスエントリと照合できます。Claude Code v2.1.197 以降が必要です。 |
| `displayName`              | `string`                                                           | 人間が読める表示名                                                                                                                                                                      |
| `description`              | `string`                                                           | モデルの機能の説明                                                                                                                                                                      |
| `supportsEffort`           | `boolean \| undefined`                                             | このモデルが努力レベルをサポートするかどうか                                                                                                                                                         |
| `supportedEffortLevels`    | `("low" \| "medium" \| "high" \| "xhigh" \| "max")[] \| undefined` | このモデルが受け入れる努力レベル                                                                                                                                                               |
| `supportsAdaptiveThinking` | `boolean \| undefined`                                             | このモデルが適応的思考をサポートするかどうか。Claude が何をどの程度考えるかを決定します                                                                                                                                |
| `supportsFastMode`         | `boolean \| undefined`                                             | このモデルが高速モードをサポートするかどうか                                                                                                                                                         |
| `supportsAutoMode`         | `boolean \| undefined`                                             | このモデルが自動モードをサポートするかどうか                                                                                                                                                         |

<h3 id="agentinfo">
  `AgentInfo`
</h3>

Agent ツールを介して呼び出すことができる利用可能なサブエージェントに関する情報です。

```typescript theme={null}
type AgentInfo = {
  name: string;
  description: string;
  model?: string;
};
```

| フィールド         | タイプ                   | 説明                                                                                                                                               |
| :------------ | :-------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | `string`              | エージェントタイプ識別子（例：`"Explore"`、`"general-purpose"`）                                                                                                  |
| `description` | `string`              | このエージェントをいつ使用するかの説明                                                                                                                              |
| `model`       | `string \| undefined` | このエージェントが使用するモデル：エイリアスまたはモデル ID、または親のモデルの場合は `'inherit'`。`undefined` の場合、Claude Code は [サブエージェントモデル順序](/docs/ja/sub-agents#choose-a-model) でモデルを選択します |

<h3 id="mcpserverprovenance">
  `McpServerProvenance`
</h3>

`mcp__*` ツールを提供する MCP サーバーと、そのサーバーの定義がどこから来たかです。[`PreToolUse`](#pretoolusehookinput)、`PostToolUse`、`PostToolUseFailure`、`PermissionRequest`、`PermissionDenied` フックの入力は `mcp_server` として、[`CanUseTool`](#canusetool) オプションは `mcpServer` として持ちます。MCP サーバーから来ないツールの場合は両方とも省略されます。

```typescript theme={null}
type McpServerProvenance = {
  name: string;
  source: string;
};
```

| フィールド    | タイプ      | 説明                                                             |
| :------- | :------- | :------------------------------------------------------------- |
| `name`   | `string` | サーバーが登録されている名前。[`mcpServerStatus()`](#query-object) が報告するのと同じ値 |
| `source` | `string` | サーバーの定義がどこから来たか：`sdk`、`plugin`、または設定スコープ                       |

`source` は以下の値のいずれかを取ります。セットはオープンなため、認識しない値を設定されたソースとして扱い、`sdk` としては扱いません。

* **`sdk`**: アプリケーションが登録したインプロセスサーバー。SDK ホストアプリケーションのみが登録できるため、設定されたサーバーは名前に関係なく `sdk` を報告しません。
* **`plugin`**: [プラグイン](/docs/ja/agent-sdk/plugins) が提供するサーバー。その `name` は [プラグイン提供 MCP サーバー](/docs/ja/mcp#plugin-provided-mcp-servers) で説明されているスコープ付き `plugin:<plugin-name>:<server-name>` 形式です。
* **設定スコープ**: `user`、`project`、`local`、`dynamic`、`managed`、`enterprise`、`claudeai`、または `agent`。`.mcp.json` サーバーは `project` を報告し、[MCP インストールスコープ](/docs/ja/mcp#mcp-installation-scopes) は `local`、`project`、`user` を定義します。アプリケーションが [`mcpServers` オプション](#options) で渡すサーバー（インプロセス SDK サーバー以外）は `dynamic` を報告します。

信頼決定は `name` または `mcp__<server>__` ツール名プレフィックスではなく `source` に基づいてください。`sdk` 以外のソースの場合、`name` は信頼されていないテキストです。表示する前にエスケープしてください。

`McpServerProvenance` とそれを持つフィールドには Agent SDK v0.3.274 以降が必要です。

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

接続された MCP サーバーのステータスです。

```typescript theme={null}
type McpServerStatus = {
  name: string;
  status: "connected" | "failed" | "needs-auth" | "pending" | "disabled";
  serverInfo?: {
    name: string;
    version: string;
  };
  error?: string;
  config?: McpServerStatusConfig;
  scope?: string;
  source?: string;
  tools?: {
    name: string;
    description?: string;
    annotations?: {
      readOnly?: boolean;
      destructive?: boolean;
      openWorld?: boolean;
    };
    _meta?: Record<string, unknown>;
  }[];
};
```

`source` はサーバーの定義がどこから来たかを示し、[`McpServerProvenance`](#mcpserverprovenance) の `source` と同じ値と信頼ルールを持ちます。フィールドには Agent SDK v0.3.274 以降が必要で、以前のバージョンでは存在しません。

`tools` エントリの `_meta` は、そのツールの `_meta` の MCP Apps メンバーを持ち、アプリケーションが [`readMcpResource()`](#query-object) でレンダリングする `ui://` リソースを見つけることができます。Claude Code は `ui` オブジェクトと非推奨のフラット `ui/resourceUri` 文字列をパススルーし、他のすべてのキーを保留します。`ui` 内では、`resourceUri` は `ui://` 文字列で、`visibility` はサーバーが設定した場合に `"model"` と `"app"` の配列で、他のメンバーは変更されずにパススルーされます。Claude Code は値が不正な形式の場合、どちらかのキーをドロップし、ツールが宣言しない場合は `_meta` を省略します。フィールドは初期化メッセージの [`capabilities`](#sdksystemmessage) に `mcp_tool_ui_meta_v1` が含まれる場合にのみ存在し、TypeScript Agent SDK v0.3.280 以降が必要です。

<h3 id="mcpserverstatusconfig">
  `McpServerStatusConfig`
</h3>

`mcpServerStatus()` によって報告される MCP サーバーの設定です。これはすべての MCP サーバートランスポートタイプの和集合です。

```typescript theme={null}
type McpServerStatusConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfig
  | McpClaudeAIProxyServerConfig;
```

各トランスポートタイプの詳細については、[`McpServerConfig`](#mcpserverconfig) を参照してください。

<h3 id="accountinfo">
  `AccountInfo`
</h3>

認証されたユーザーのアカウント情報です。

```typescript theme={null}
type AccountInfo = {
  email?: string;
  organization?: string;
  subscriptionType?: string;
  tokenSource?: string;
  apiKeySource?: string;
};
```

<h3 id="modelusage">
  `ModelUsage`
</h3>

結果メッセージで返されるモデルごとの使用統計です。`costUSD` 値はクライアント側の推定値です。請求に関する注意事項については、[コストと使用状況の追跡](/docs/ja/agent-sdk/cost-tracking) を参照してください。

```typescript theme={null}
type ModelUsage = {
  inputTokens: number;
  outputTokens: number;
  thinkingTokens?: number;
  cacheReadInputTokens: number;
  cacheCreationInputTokens: number;
  webSearchRequests: number;
  costUSD: number;
  contextWindow: number;
  maxOutputTokens: number;
  canonicalModel?: string;
  provider?: string;
  costBasis?: 'list' | 'managed' | 'unknown';
};
```

`thinkingTokens` はこのモデルが生成した思考トークンをカウントします。`outputTokens` はすでにそれらを含んでいるため、2 つを合計しないでください。フィールドは、ターンが記録するバージョンの Claude Code で実行されるまで存在しません。そのため、以前のバージョンで開始された再開されたセッションは部分的なカウントを報告します。`thinkingTokens` には Agent SDK v0.3.257 以降が必要です。

`canonicalModel` および `provider` フィールドには Claude Code v2.1.218 以降が必要です。`canonicalModel` は価格ルックアップが使用する正規モデル ID です。例えば、その文字列がプロバイダー固有の ID またはエイリアスの場合、生のモデル文字列と異なる場合があります。

`provider` は、`firstParty`、`bedrock`、`vertex`、`foundry`、`anthropicAws`、`mantle`、`gateway` など、モデルを提供した API バックエンドに名前を付けます。

`costBasis` はモデルの最新リクエストに価格を付けた価格表に名前を付けます：リスト価格の場合は `list`、[`modelPricing`](/docs/ja/settings-reference#modelpricing) テーブルの場合は `managed`、モデル ID がどちらにも一致しない場合は `unknown`。フィールドには Claude Code v2.1.246 以降が必要です。

<h3 id="configscope">
  `ConfigScope`
</h3>

```typescript theme={null}
type ConfigScope = "local" | "user" | "project";
```

<h3 id="nonnullableusage">
  `NonNullableUsage`
</h3>

すべてのヌル許容フィールドがヌル許容でない [`Usage`](#usage) のバージョンです。

```typescript theme={null}
type NonNullableUsage = {
  [K in keyof Usage]: NonNullable<Usage[K]>;
};
```

<h3 id="usage">
  `Usage`
</h3>

トークン使用統計です。これは `@anthropic-ai/sdk` の `BetaUsage` タイプです。

```typescript theme={null}
type Usage = {
  input_tokens: number;
  output_tokens: number;
  cache_creation_input_tokens: number | null;
  cache_read_input_tokens: number | null;
  cache_creation: {
    ephemeral_5m_input_tokens: number;
    ephemeral_1h_input_tokens: number;
  } | null;
  server_tool_use: BetaServerToolUsage | null;
  service_tier: "standard" | "priority" | "batch" | null;
  speed: "standard" | "fast" | null;
  inference_geo: string | null;
  iterations: BetaIterationsUsage | null;
  output_tokens_details: BetaOutputTokensDetails | null;
};
```

`BetaServerToolUsage`、`BetaIterationsUsage`、`BetaOutputTokensDetails` は `@anthropic-ai/sdk` で定義されています。

`output_tokens_details` は請求された出力をカテゴリ別に分類します。現在、`thinking_tokens: number` という 1 つのフィールドを持ち、モデルが内部推論として生成した出力トークンをカウントします。これには思考ブロック区切り文字が含まれます。`output_tokens_details` フィールドには TypeScript SDK v0.3.228 以降が必要で、これは Claude Code v2.1.228 をバンドルしています。

* **請求**: 観測可能性のために分類を読み、請求のためではありません。`output_tokens` は権限のある合計のままで、`output_tokens - thinking_tokens` は非推論出力を近似します。
* **カウントが対象とするもの**: モデルが生成した生の推論。これは応答本文で返された思考テキストより長い場合があります。API はその生テキストを再トークン化することで計算するため、モデルの正確な生成カウントから数トークン異なる場合があります。
* **ストリーミング**: ストリーミングされたアシスタントメッセージでは、この分類は `output_tokens` と同様に `message_start` プレースホルダーで、実際のカウントを持たないため、[結果メッセージから出力トークンを読む](/docs/ja/agent-sdk/cost-tracking#read-output-tokens-from-the-result-message) で説明されているように結果メッセージから読んでください。結果メッセージでは、モデルまたはプロバイダーが分類を報告しない場合、`thinking_tokens` は `0` を読みます。
* **`null` ケース**: `output_tokens_details` 自体は、Claude Code が合成するアシスタントメッセージ（API エラーメッセージなど）では `null` です。

<h3 id="calltoolresult">
  `CallToolResult`
</h3>

MCP ツール結果タイプ（`@modelcontextprotocol/sdk/types.js` から）。`structuredContent` は `content` と一緒に返すことができる JSON オブジェクトで、画像ブロックを含みます。[構造化データを返す](/docs/ja/agent-sdk/custom-tools#return-structured-data) を参照してください。

```typescript theme={null}
type CallToolResult = {
  content: Array<{
    type: "text" | "image" | "audio" | "resource" | "resource_link";
    // Additional fields vary by type
  }>;
  structuredContent?: Record<string, unknown>;
  isError?: boolean;
};
```

<h3 id="sdkmcpresourcelink">
  `SDKMcpResourceLink`
</h3>

MCP ツールが参照によって返した 1 つのファイルです。Claude Code は各エントリをツール結果の `resource_link` ブロックから構築し、[`SDKUserMessage.tool_use_result`](#sdkusermessage) の `resourceLinks` として、または [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) の `resource_links` として呼び出しがバックグラウンドで完了したときにリストを配信します。Agent SDK v0.3.257 以降が必要です。

```typescript theme={null}
type SDKMcpResourceLink = {
  uri: string;
  name: string;
  title?: string;
  description?: string;
  mimeType?: string;
  size?: number;
  annotations?: Record<string, unknown>;
};
```

Claude Code は `uri` または `name` が文字列でないブロックをドロップし、値がリストされたタイプでないオプションフィールドを除外します。

| フィールド         | タイプ                                    | 説明                                       |
| :------------ | :------------------------------------- | :--------------------------------------- |
| `uri`         | `string`                               | リソースの URI。サーバーが返したもの                     |
| `name`        | `string`                               | サーバーがリソースに付けた名前                          |
| `title`       | `string \| undefined`                  | 表示タイトル。サーバーが設定した場合                       |
| `description` | `string \| undefined`                  | 説明。サーバーが設定した場合                           |
| `mimeType`    | `string \| undefined`                  | MIME タイプ。サーバーが設定した場合                     |
| `size`        | `number \| undefined`                  | サイズ（バイト単位）。サーバーが設定した場合                   |
| `annotations` | `Record<string, unknown> \| undefined` | ブロックの MCP annotations オブジェクト。サーバーが設定した場合 |

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

Claude の思考/推論動作を制御します。非推奨の `maxThinkingTokens` より優先されます。

```typescript theme={null}
type ThinkingDisplay = "summarized" | "omitted";

type ThinkingConfig =
  | { type: "adaptive"; display?: ThinkingDisplay } // The model determines when and how much to reason (Opus 4.6+)
  | { type: "enabled"; budgetTokens?: number; display?: ThinkingDisplay } // Fixed thinking token budget
  | { type: "disabled" }; // No extended thinking
```

オプションの `display` フィールドは、思考テキストが `"summarized"` または `"omitted"` で返されるかどうかを制御します。Claude Opus 4.7 以降では、API デフォルトは `"omitted"` なため、思考コンテンツを `thinking` ブロックで受け取るには `"summarized"` を設定してください。Claude Code は Amazon Bedrock または Google Cloud の Agent Platform に `display` を送信しないため、これらのプロバイダーでは Opus 4.7 以降は `display` を `"summarized"` に設定した場合でも空の `thinking` ブロックを返します。

<h3 id="spawnedprocess">
  `SpawnedProcess`
</h3>

カスタムプロセス生成用のインターフェース（`spawnClaudeCodeProcess` オプションで使用）。`ChildProcess` はすでにこのインターフェースを満たしています。

```typescript theme={null}
interface SpawnedProcess {
  stdin: Writable;
  stdout: Readable;
  readonly killed: boolean;
  readonly exitCode: number | null;
  kill(signal: NodeJS.Signals): boolean;
  on(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  on(event: "error", listener: (error: Error) => void): void;
  once(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  once(event: "error", listener: (error: Error) => void): void;
  off(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  off(event: "error", listener: (error: Error) => void): void;
}
```

<h3 id="spawnoptions">
  `SpawnOptions`
</h3>

カスタム spawn 関数に渡されるオプションです。

```typescript theme={null}
interface SpawnOptions {
  command: string;
  args: string[];
  cwd?: string;
  env: Record<string, string | undefined>;
  signal: AbortSignal;
}
```

<Note>
  `signal` フィールドは、プロセスをティアダウンするタイミングを spawn 関数に伝えます。Node の `spawn()` に `signal` オプションとして渡すか、VM またはコンテナティアダウンハンドラーに渡してください。

  このシグナルは [`Options.abortController`](#options) が中止する瞬間に発火しません。SDK は最初にプロセスの stdin を閉じ、CLI がクリーンにシャットダウンできるように約 2 秒待機してから、このシグナルを中止します。呼び出し元が中止する瞬間に反応するには、spawn 関数がそのエンクロージングスコープから参照できる独自の `Options.abortController.signal` をリッスンしてください。
</Note>

<h3 id="mcpsetserversresult">
  `McpSetServersResult`
</h3>

`setMcpServers()` 操作の結果です。

```typescript theme={null}
type McpSetServersResult = {
  added: string[];
  removed: string[];
  errors: Record<string, string>;
};
```

`setMcpServers()` を呼び出すと、Claude Code は以下のルールを適用します。

* **呼び出しが名前を付けないサーバー**: Claude Code はプラグイン提供サーバーを実行し続けます。Agent SDK v0.3.210 以降が必要です。
* **呼び出しが名前を付けるサーバー**: CLI が起動時に開始した組み込みサーバーを除き、Claude Code は実行中のサーバーを、渡した設定と異なる場合にのみ置き換えます。
* **CLI が起動時に開始した組み込みサーバー**: 呼び出しが 1 つを名前付けする場合、Claude Code はそのエントリをドロップし、`errors` で報告します。

新しく追加された stdio、HTTP、SSE サーバーが接続または失敗した後、プロミスが解決されるため、接続されたサーバーからのツールは次のターンで利用可能です。

`added` は Claude Code が追加または置き換えたサーバーをリストします。接続したかどうかに関わらず。接続に失敗したサーバーは `added` と `errors` の両方に表示され、失敗テキストは `errors` の下にあり、[`mcpServerStatus()`](#methods) に `failed` 行があります。Claude Code v2.1.257 より前では、接続試行がスローされたサーバーは `errors` の下にのみ報告されました。

<h3 id="rewindfilesresult">
  `RewindFilesResult`
</h3>

`rewindFiles()` 操作の結果です。

```typescript theme={null}
type RewindFilesResult = {
  canRewind: boolean;
  error?: string;
  filesChanged?: string[];
  insertions?: number;
  deletions?: number;
  skippedLinks?: number;
};
```

`skippedLinks` は、リンク安全性のためにリワインドが復元または削除を拒否した追跡パスをカウントします。追跡パスのシンボリックリンク、ハードリンク、またはその他の非通常ファイル、チェックポイント取得時に指していた場所に解決されなくなった親ディレクトリ、または安全に読み取ることができなかったバックアップ。フィールドには Claude Code v2.1.216 以降が必要です。`rewindFiles(userMessageId, { dryRun: true })` でのプレビュー呼び出しは設定しません。

<h3 id="sdkstatusmessage">
  `SDKStatusMessage`
</h3>

ステータス更新メッセージ（例：コンパクト化）。

```typescript theme={null}
type SDKStatusMessage = {
  type: "system";
  subtype: "status";
  status: "compacting" | null;
  permissionMode?: PermissionMode;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktasknotificationmessage">
  `SDKTaskNotificationMessage`
</h3>

バックグラウンドタスクが完了、失敗、または停止したときの通知です。バックグラウンドタスクには `run_in_background` Bash コマンド、[Monitor](#monitor) ウォッチ、バックグラウンドサブエージェントが含まれます。`ambient` フィールドについては、[`SDKTaskStartedMessage`](#sdktaskstartedmessage) を参照してください。これはそれを定義し、そのバージョン要件を定義します。

```typescript theme={null}
type SDKTaskNotificationMessage = {
  type: "system";
  subtype: "task_notification";
  task_id: string;
  tool_use_id?: string;
  status: "completed" | "failed" | "stopped";
  output_file: string;
  summary: string;
  ambient?: boolean;
  usage?: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  resource_links?: SDKMcpResourceLink[];
  uuid: UUID;
  session_id: string;
};
```

Claude Code が [長い MCP ツール呼び出しをバックグラウンドに移動](/docs/ja/mcp#automatic-backgrounding-of-long-tool-calls) する場合、その呼び出しの `tool_result` ブロックはプレースホルダーのみを保持し、呼び出しの実際の結果はこの通知で到着します。`tool_use_id` で通知を呼び出しと照合してください。
`completed` 通知では、`resource_links` はツールが [`SDKMcpResourceLink`](#sdkmcpresourcelink) エントリとして参照によって返したファイルをリストし、[`tool_use_result.resourceLinks`](#sdkusermessage) と同じ 50 リンクおよび 64 KiB 制限があります。Claude Code は結果にリンクがなく、MCP ツール呼び出しではないタスクの通知では `resource_links` を省略します。`resource_links` には Agent SDK v0.3.257 以降が必要です。

Claude Code は、[`scheduled-trigger` サブカインド](#task-notification-subkinds) でスタンプされた配信を除き、送信するすべてのタスク通知にモデルへの通知を前置します。これらは代わりに割り当てられたタスクフレーミングを持ちます。通知は、人間の入力が発生していないため、モデルが通知をユーザー指示または承認として扱わないことを述べています。

タスク通知ターンを検出するには、[`SDKUserMessage`](#sdkusermessage) または [`SDKResultMessage`](#sdkresultmessage) で `origin.kind === "task-notification"` をチェックしてください。通知テキストの一致ではなく。必要に応じて同じフィールドから `subkind` を読んで、何がそれを発生させたかを知ってください。v2.1.205 より前では、Claude Code はセッションがアイドル状態の間に到着した通知から通知を除外しました。

<h3 id="sdktoolusesummarymessage">
  `SDKToolUseSummaryMessage`
</h3>

会話でのツール使用の概要です。

```typescript theme={null}
type SDKToolUseSummaryMessage = {
  type: "tool_use_summary";
  summary: string;
  preceding_tool_use_ids: string[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookstartedmessage">
  `SDKHookStartedMessage`
</h3>

フックが実行を開始するときに発行されます。

Claude Code はこのメッセージ、[`SDKHookProgressMessage`](#sdkhookprogressmessage)、[`SDKHookResponseMessage`](#sdkhookresponsemessage) をメッセージストリームに直ちに配信します。セッション起動中に `SessionStart` または `Setup` フックがまだ実行中の場合も含みます。Claude Code v2.1.169 から v2.1.203 はこれらのメッセージを `SessionStart` または `Setup` フックが完了した後に 1 つのバッチで配信しました。v2.1.204 はライブ配信を復元しました。

```typescript theme={null}
type SDKHookStartedMessage = {
  type: "system";
  subtype: "hook_started";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookprogressmessage">
  `SDKHookProgressMessage`
</h3>

フックが実行中で、stdout/stderr 出力があるときに発行されます。

```typescript theme={null}
type SDKHookProgressMessage = {
  type: "system";
  subtype: "hook_progress";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  stdout: string;
  stderr: string;
  output: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookresponsemessage">
  `SDKHookResponseMessage`
</h3>

フックが実行を完了するときに発行されます。

```typescript theme={null}
type SDKHookResponseMessage = {
  type: "system";
  subtype: "hook_response";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  output: string;
  stdout: string;
  stderr: string;
  exit_code?: number;
  outcome: "success" | "error" | "cancelled";
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktoolprogressmessage">
  `SDKToolProgressMessage`
</h3>

ツールが実行中に定期的に発行され、進捗を示します。

```typescript theme={null}
type SDKToolProgressMessage = {
  type: "tool_progress";
  tool_use_id: string;
  tool_name: string;
  parent_tool_use_id: string | null;
  elapsed_time_seconds: number;
  task_id?: string;
  heartbeat?: boolean;
  subagent_type?: string;
  subagent_retry?: {
    agent_id: string;
    attempt: number;
    max_retries: number;
    retry_delay_ms: number;
    error_status: number | null;
    error_category: string;
  };
  uuid: UUID;
  session_id: string;
};
```

ツール呼び出しがメイン会話で実行されている間、Claude Code は `heartbeat: true` で 30 秒ごとに `tool_progress` メッセージを発行します。各ハートビートはツール名と経過秒数を持ち、長時間実行される呼び出しを停止したセッションと区別できます。Claude Code はサブエージェント内のツール呼び出しのハートビートを発行しません。`heartbeat` フィールドには Agent SDK v0.3.214 以降が必要です。v2.1.257 より前では、Claude Code はフォアグラウンド Agent ツール呼び出しのハートビートも発行しませんでした。

ハートビート以外の Agent ツールの `tool_progress` メッセージでは、`subagent_type` は実行中のサブエージェントタイプ（`general-purpose` など）に名前を付けます。`subagent_retry` はそのサブエージェントが API エラーバックオフ（レート制限やオーバーロードなど）を待機している間に存在し、再試行試行ごとに 1 つのメッセージがあります。両方のフィールドには Agent SDK v0.3.214 以降が必要です。

`subagent_retry` から再試行インジケーターをレンダリングするには：

* `parent_tool_use_id` でインジケーターを追跡します。これはサブエージェントごとに一意です。`tool_use_id` は 1 つのアシスタントターンから並列サブエージェントで共有されるため、それで追跡するとあるサブエージェントの更新が別のサブエージェントのインジケーターをクリアします。
* 同じ `parent_tool_use_id` の後の `tool_progress` が `subagent_retry` も `heartbeat: true` も持たない場合、またはツールの結果メッセージが到着したときにインジケーターをクリアします。`heartbeat: true` のフレームはライブネスのみを報告するため、1 つが到着したときはインジケーターを保持してください。`attempt` は永続的な再試行の下で `max_retries` を超える可能性があるため、カウンターからクリアを導出しないでください。
* `error_category` を表示テキストではなく、独自のメッセージテキストを選択するためのトークンとして扱います。値は `rate_limit`、`overloaded`、`authentication_failed`、`server_error`、`cloud_credential_error`、`unknown` です。認識しない値を `unknown` と同じ方法で処理してください。後のリリースは値を追加できるためです。

<h3 id="sdkauthstatusmessage">
  `SDKAuthStatusMessage`
</h3>

認証フロー中に発行されます。

```typescript theme={null}
type SDKAuthStatusMessage = {
  type: "auth_status";
  isAuthenticating: boolean;
  output: string[];
  error?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskstartedmessage">
  `SDKTaskStartedMessage`
</h3>

タスクが開始するときに発行されます。`task_type` フィールドは Bash コマンドと [Monitor](#monitor) ウォッチの場合は `"local_bash"`、サブエージェントの場合は `"local_agent"`、または `"remote_agent"` です。

```typescript theme={null}
type SDKTaskStartedMessage = {
  type: "system";
  subtype: "task_started";
  task_id: string;
  tool_use_id?: string;
  description: string;
  task_type?: string;
  is_backgrounded?: boolean;
  spawn_depth?: number;
  ambient?: boolean;
  uuid: UUID;
  session_id: string;
};
```

`ambient` は、セッションの作業の一部ではないタスク（Claude Code が独自の操作のために実行するタスクなど）の場合は `true` です。ライブアップデートウォッチャーもアンビエントで、ユーザーが要求したウォッチャーを含みます。アクティビティインジケーターからアンビエントタスクを除外してください。フィールドには Agent SDK v0.3.247 以降が必要です。

`ambient` は [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) および [`SDKBackgroundTasksChangedMessage`](#sdkbackgroundtaskschangedmessage) エントリにも表示されます。

`is_backgrounded` および `spawn_depth` は Claude Code がタスクをどのように開始したかを説明します。両方のフィールドには Agent SDK v0.3.238 以降が必要です。

* `is_backgrounded`: Claude Code は `"local_agent"` および `"local_bash"` タスクで設定します。`true` はタスクがバックグラウンドで実行されることを意味します。`false` はタスクがフォアグラウンドで実行され、それを開始したツール呼び出しはタスクが完了するかバックグラウンドに移動するまでブロックされたままであることを意味します。
* `spawn_depth`: Claude Code は `"local_agent"` タスクのみで設定します。メインスレッドが生成したサブエージェントの深さは `1` です。深さ `1` サブエージェントが生成したサブエージェントの深さは `2` などです。

[再開されたサブエージェント](/docs/ja/agent-sdk/subagents#resume-subagents) は常に `is_backgrounded: true` を報告します。Claude Code はすべての再開されたサブエージェントをバックグラウンドで実行するためです。フォアグラウンドタスクが後でバックグラウンドに移動する場合、Claude Code は新しい `is_backgrounded` 値を [`task_updated`](#sdktaskupdatedmessage) メッセージで報告し、2 番目の `task_started` を送信しません。

<h3 id="sdktaskprogressmessage">
  `SDKTaskProgressMessage`
</h3>

サブエージェントまたはバックグラウンドタスクが実行中に定期的に発行されます。`summary` フィールドは [`agentProgressSummaries`](#options) が有効な場合にのみ入力されます。

```typescript theme={null}
type SDKTaskProgressMessage = {
  type: "system";
  subtype: "task_progress";
  task_id: string;
  tool_use_id?: string;
  description: string;
  subagent_type?: string;
  usage: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  last_tool_name?: string;
  summary?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskupdatedmessage">
  `SDKTaskUpdatedMessage`
</h3>

バックグラウンドタスクの状態が変わるときに発行されます。例えば、`running` から `completed` に遷移するときなど。`patch` をローカルタスクマップ（`task_id` でキー付け）にマージしてください。`end_time` フィールドは Unix エポックタイムスタンプ（ミリ秒単位）で、`Date.now()` と比較可能です。

```typescript theme={null}
type SDKTaskUpdatedMessage = {
  type: "system";
  subtype: "task_updated";
  task_id: string;
  patch: {
    status?: "pending" | "running" | "completed" | "failed" | "killed";
    description?: string;
    end_time?: number;
    total_paused_ms?: number;
    error?: string;
    is_backgrounded?: boolean;
  };
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkbackgroundtaskschangedmessage">
  `SDKBackgroundTasksChangedMessage`
</h3>

ライブバックグラウンドタスクのセットが変わるときに発行されます。タスクが開始、完了、キル、フォアグラウンドエージェントがバックグラウンド化、またはタスクの `description` または `ambient` フィールドが変わるときなど。

`tasks` 配列はライブセット全体です。`task_started` および `task_notification` イベントをペアリングするのではなく、各ペイロードでキャッシュされたセットを置き換えてください。次のメンバーシップ変更は逃したイベントを修正します。

これらのタスクごとのイベントに対する順序付けは指定されていないため、2 つのストリームを相関させないでください。

起動時には何も発行されません。セッションの CLI プロセスが開始または再開されるたびに空のセットにリセットし、次のメンバーシップ変更でそれを再入力させてください。

実行中のセッションに繰り返される `initialize` 制御リクエストを送信する場合（例えば、トランスポートギャップ後の [`reinitialize()`](#query-object)）、Claude Code は応答に続いて現在のライブセットのスナップショットを送信します。空の場合でも。再接続ホストは次のメンバーシップ変更を待つことなく何が実行されているかを学習します。Agent SDK v0.3.239 より前では、Claude Code は繰り返された `initialize` の後にスナップショットを送信しませんでした。

Claude Code v2.1.203 以降が必要です。

```typescript theme={null}
type SDKBackgroundTasksChangedMessage = {
  type: "system";
  subtype: "background_tasks_changed";
  tasks: {
    task_id: string;
    task_type: string;
    description: string;
    ambient?: boolean;
  }[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkthinkingtokensmessage">
  `SDKThinkingTokensMessage`
</h3>

Claude が思考ブロック（編集されたものを含む）を生成している間に発行されます。`estimated_tokens` は現在のブロックで生成された思考トークンの実行推定値で、`estimated_tokens_delta` はこのフレームで持ち込まれた増分です。進捗表示にこれらの推定値を使用してください。

モデルまたはプロバイダーが分類を報告する場合、トップレベルエージェントループの最終カウントは結果メッセージの [`usage.output_tokens_details.thinking_tokens`](#usage) です。これは [サブエージェントトークンを含みません](/docs/ja/agent-sdk/cost-tracking#get-the-total-cost-of-a-query)。

Claude Code v2.1.153 以降が必要です。

```typescript theme={null}
type SDKThinkingTokensMessage = {
  type: "system";
  subtype: "thinking_tokens";
  estimated_tokens: number;
  estimated_tokens_delta: number;
  user_message_uuid?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkfilespersistedevent">
  `SDKFilesPersistedEvent`
</h3>

ファイルチェックポイントがディスクに永続化されるときに発行されます。

```typescript theme={null}
type SDKFilesPersistedEvent = {
  type: "system";
  subtype: "files_persisted";
  files: { filename: string; file_id: string }[];
  failed: { filename: string; error: string }[];
  processed_at: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkratelimitevent">
  `SDKRateLimitEvent`
</h3>

セッションがレート制限に遭遇するときに発行されます。

```typescript theme={null}
type SDKRateLimitEvent = {
  type: "rate_limit_event";
  rate_limit_info: {
    status: "allowed" | "allowed_warning" | "rejected";
    resetsAt?: number;
    utilization?: number;
    errorCode?: "credits_required";
    canUserPurchaseCredits?: boolean;
    hasChargeableSavedPaymentMethod?: boolean;
  };
  uuid: UUID;
  session_id: string;
};
```

`errorCode` が `"credits_required"` の場合、拒否は含まれた使用量が枯渇した claude.ai サブスクリプションからのもので、ユーザーが使用クレジットを購入するまでセッションは続行できません。`canUserPurchaseCredits` はアカウントのクレジットを購入できるかどうかを示し、`hasChargeableSavedPaymentMethod` は保存された支払い方法がファイルにあるかどうかを示します。3 つのフィールドすべてはクレジット必須拒否ではないレート制限イベントでは存在しません。Claude Code v2.1.181 以降が必要です。

<h3 id="sdklocalcommandoutputmessage">
  `SDKLocalCommandOutputMessage`
</h3>

Claude Code はこのメッセージタイプを発行しません。`/context` または `/usage` などのコマンドをプロンプトとして送信する場合、その出力は [`SDKAssistantMessage`](#sdkassistantmessage) として到着します。

```typescript theme={null}
type SDKLocalCommandOutputMessage = {
  type: "system";
  subtype: "local_command_output";
  content: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkcommandschangedmessage">
  `SDKCommandsChangedMessage`
</h3>

利用可能なコマンドのセットがセッション中に変わるときに発行されます。例えば、Claude Code がエージェントがサブディレクトリに入るときにスキルを発見するときなど。`commands` 配列は完全に更新されたリストなため、このペイロードでキャッシュされたコマンドリストを置き換えてください。
このメッセージの後に [`supportedCommands()`](#query-object) を呼び出すと、メソッドが最新のプッシュを追跡するため、同じ更新されたリストが返されます。これには Agent SDK v0.3.216 以降が必要です。以前の SDK バージョンでは、`supportedCommands()` は初期化時にキャプチャされたスナップショットを返し、セッション中の変更を反映しません。

```typescript theme={null}
type SDKCommandsChangedMessage = {
  type: "system";
  subtype: "commands_changed";
  commands: SlashCommand[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkpromptsuggestionmessage">
  `SDKPromptSuggestionMessage`
</h3>

[`promptSuggestions`](#options) が有効で Claude Code がそのターンの提案を生成した後、ターンの後に発行されます。予測される次のユーザープロンプトが含まれます。提案を取得しないターンについては、[Claude Code が提案をスキップする場合](/docs/ja/interactive-mode#when-claude-code-skips-suggestions) を参照してください。

```typescript theme={null}
type SDKPromptSuggestionMessage = {
  type: "prompt_suggestion";
  suggestion: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkconversationresetmessage">
  `SDKConversationResetMessage`
</h3>

セッションを終了せずにセッションの会話が置き換えられるときに発行されます。`query()` 呼び出しでは、`/clear` とそのエイリアスのみがこのメッセージを生成します。`new_conversation_id` の下に空のトランスクリプトをマウントし、キャッシュされたセッションタイトルを破棄してください。

```typescript theme={null}
type SDKConversationResetMessage = {
  type: "conversation_reset";
  new_conversation_id: UUID;
  uuid: UUID;
  session_id: string;
};
```

SDK の公開された型付けは Claude Code v2.1.203 以降で `SDKConversationResetMessage` を宣言します。v2.1.203 より前では、`SDKMessage` は型を宣言せずに参照したため、`skipLibCheck` が無効な場合、`type === "conversation_reset"` での絞り込みは型チェックに失敗しました。

<h3 id="aborterror">
  `AbortError`
</h3>

中止操作のカスタムエラークラスです。

```typescript theme={null}
class AbortError extends Error {}
```

`AbortError` は SDK の型付き API の唯一のエラークラスです。Claude Code プロセスの終了や起動失敗など、その他の失敗は、SDK クラスと照合するメッセージイテレーションを拒否します。[トラブルシューティング](/docs/ja/agent-sdk/troubleshooting) はそれらのエラーをメッセージでキー付けし、各エラーの原因と修正を示します。

<h2 id="sandbox-configuration">
  サンドボックス設定
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

サンドボックス動作の設定。これを使用して、コマンドサンドボックスを有効にし、ネットワーク制限をプログラムで設定します。

```typescript theme={null}
type SandboxSettings = {
  enabled?: boolean;
  failIfUnavailable?: boolean;
  autoAllowBashIfSandboxed?: boolean;
  excludedCommands?: string[];
  allowUnsandboxedCommands?: boolean;
  network?: SandboxNetworkConfig;
  filesystem?: SandboxFilesystemConfig;
  ignoreViolations?: Record<string, string[]>;
  enableWeakerNestedSandbox?: boolean;
  ripgrep?: { command: string; args?: string[] };
};
```

| プロパティ                       | 型                                                     | デフォルト       | 説明                                                                                                                                                                                |
| :-------------------------- | :---------------------------------------------------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `boolean`                                             | `false`     | コマンド実行のサンドボックスモードを有効にします                                                                                                                                                          |
| `failIfUnavailable`         | `boolean`                                             | `true`      | `enabled` が `true` でもサンドボックスが起動できない場合、起動時に停止します。`false` に設定すると、stderr に警告を表示してサンドボックス外実行にフォールバックします                                                                               |
| `autoAllowBashIfSandboxed`  | `boolean`                                             | `true`      | サンドボックスが有効な場合、Bash コマンドを自動承認します                                                                                                                                                   |
| `excludedCommands`          | `string[]`                                            | `[]`        | サンドボックス制限をバイパスするコマンド（例：`['docker *']`）。これらはモデルの関与なしに自動的にサンドボックス外で実行されます。[`sandbox.excludedCommands`](/docs/ja/settings-reference#sandbox-excludedcommands) を参照して、エントリが適用される場合を確認してください |
| `allowUnsandboxedCommands`  | `boolean`                                             | `true`      | モデルがサンドボックス外でコマンドを実行するようにリクエストすることを許可します。`true` の場合、モデルはツール入力で `dangerouslyDisableSandbox` を設定でき、[パーミッションシステム](#permissions-fallback-for-unsandboxed-commands) にフォールバックします        |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `undefined` | ネットワーク固有のサンドボックス設定                                                                                                                                                                |
| `filesystem`                | [`SandboxFilesystemConfig`](#sandboxfilesystemconfig) | `undefined` | 読み取り/書き込み制限のためのファイルシステム固有のサンドボックス設定                                                                                                                                               |
| `ignoreViolations`          | `Record<string, string[]>`                            | `undefined` | コマンド部分文字列のマップ、またはすべてのコマンドに対して `*`、無視する違反テキストの部分文字列（例：`{ "*": ['/etc/hosts'] }`）。[`sandbox.ignoreViolations`](/docs/ja/settings-reference#sandbox-ignoreviolations) を参照してください           |
| `enableWeakerNestedSandbox` | `boolean`                                             | `false`     | 互換性のための弱いネストされたサンドボックスを有効にします                                                                                                                                                     |
| `ripgrep`                   | `{ command: string; args?: string[] }`                | `undefined` | サンドボックス環境のカスタム ripgrep バイナリ設定                                                                                                                                                     |

<Note>
  サンドボックスはプラットフォームサポートに依存し、Linux では `bubblewrap` や `socat` などのツールが必要です。`enabled` が `true` でサンドボックスが起動できない場合、`query()` は `subtype: "error_during_execution"` の `result` メッセージを報告し、理由を `errors` に含めます。単一メッセージの `query()` 呼び出しの場合、SDK はそのエラー結果を生成した後にスローするため、ループを try ブロックでラップして、それを超えて続行してください。エラーコントラクトについては [結果を処理する](/docs/ja/agent-sdk/agent-loop#handle-the-result) を参照してください。

  代わりにサンドボックス外で実行するには、`failIfUnavailable: false` を設定してください。
</Note>

<h4 id="example-usage">
  使用例
</h4>

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

try {
  for await (const message of query({
    prompt: "Build and test my project",
    options: {
      sandbox: {
        enabled: true,
        autoAllowBashIfSandboxed: true,
        network: {
          allowLocalBinding: true
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
} catch (error) {
  // A single-shot query() throws after yielding an error result,
  // such as when the sandbox can't start (failIfUnavailable defaults to true).
  console.log(`Session ended with an error: ${error}`);
}
```

<Warning>
  **Unix ソケットセキュリティ：** `allowUnixSockets` オプションは、サンドボックスの外に到達するシステムサービスへのアクセスを許可できます。例えば、`/var/run/docker.sock` を許可すると、Docker API 経由でホストシステムへの完全なアクセスが効果的に許可され、サンドボックス分離がバイパスされます。厳密に必要な Unix ソケットのみを許可し、各ソケットのセキュリティへの影響を理解してください。
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

サンドボックスモードのネットワーク固有の設定。これらの設定は、親の [`SandboxSettings`](#sandboxsettings) で `enabled` が `true` の場合、サンドボックス化された Bash コマンドに適用されます。WebFetch ツールには適用されず、代わりに [パーミッションルール](/docs/ja/permissions#webfetch) を使用します。

```typescript theme={null}
type SandboxNetworkConfig = {
  allowedDomains?: string[];
  deniedDomains?: string[];
  strictAllowlist?: boolean;
  allowManagedDomainsOnly?: boolean;
  allowLocalBinding?: boolean;
  allowUnixSockets?: string[];
  allowAllUnixSockets?: boolean;
  httpProxyPort?: number;
  socksProxyPort?: number;
};
```

| プロパティ                     | 型          | デフォルト       | 説明                                                                                                                                                                                                                                                       |
| :------------------------ | :--------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `string[]` | `[]`        | サンドボックス化されたプロセスがアクセスできるドメイン名                                                                                                                                                                                                                             |
| `deniedDomains`           | `string[]` | `[]`        | サンドボックス化されたプロセスがアクセスできないドメイン名。`allowedDomains` より優先されます                                                                                                                                                                                                  |
| `strictAllowlist`         | `boolean`  | `false`     | [ネットワークアローリスト](/docs/ja/sandboxing#network-isolation) の外のホストへのサンドボックス化されたコマンドアクセスを拒否します。プロンプトの代わりに強制されます。サンドボックス化されたコマンドのみに強制されます。WebFetch などのインプロセスツールはそれによってゲートされません。ユーザー、管理、または CLI `--settings` 設定からのみ尊重されます。プロジェクト設定は無視されます。Claude Code v2.1.219 以降が必要です |
| `allowManagedDomainsOnly` | `boolean`  | `false`     | [管理設定](/docs/ja/managed-settings) で設定された場合、管理設定からの `allowedDomains` エントリと管理設定からの `WebFetch(domain:...)` 許可ルールのみが尊重され、ユーザー、プロジェクト、またはローカル設定からの許可エントリは無視されます。SDK オプション経由で設定された場合は効果がありません                                                                       |
| `allowLocalBinding`       | `boolean`  | `false`     | プロセスがローカルポートにバインドすることを許可します（例：開発サーバー）                                                                                                                                                                                                                    |
| `allowUnixSockets`        | `string[]` | `[]`        | プロセスがアクセスできる Unix ソケットパス（例：Docker ソケット）                                                                                                                                                                                                                  |
| `allowAllUnixSockets`     | `boolean`  | `false`     | すべての Unix ソケットへのアクセスを許可します                                                                                                                                                                                                                               |
| `httpProxyPort`           | `number`   | `undefined` | ネットワークリクエスト用の HTTP プロキシポート                                                                                                                                                                                                                               |
| `socksProxyPort`          | `number`   | `undefined` | ネットワークリクエスト用の SOCKS プロキシポート                                                                                                                                                                                                                              |

<Note>
  組み込みサンドボックスプロキシは、リクエストされたホスト名に基づいて `allowedDomains` を強制し、TLS トラフィックを終了または検査しないため、[ドメインフロンティング](https://en.wikipedia.org/wiki/Domain_fronting) などの技術がそれをバイパスする可能性があります。詳細は [サンドボックスセキュリティの制限](/docs/ja/sandboxing#security-limitations) を参照し、TLS 終了プロキシの設定については [セキュアなデプロイ](/docs/ja/agent-sdk/secure-deployment#traffic-forwarding) を参照してください。
</Note>

<h3 id="sandboxfilesystemconfig">
  `SandboxFilesystemConfig`
</h3>

サンドボックスモードのファイルシステム固有の設定。

```typescript theme={null}
type SandboxFilesystemConfig = {
  allowWrite?: string[];
  denyWrite?: string[];
  denyRead?: string[];
};
```

| プロパティ        | 型          | デフォルト | 説明                    |
| :----------- | :--------- | :---- | :-------------------- |
| `allowWrite` | `string[]` | `[]`  | 書き込みアクセスを許可するファイルパターン |
| `denyWrite`  | `string[]` | `[]`  | 書き込みアクセスを拒否するファイルパターン |
| `denyRead`   | `string[]` | `[]`  | 読み取りアクセスを拒否するファイルパターン |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  サンドボックス外コマンドのパーミッションフォールバック
</h3>

`allowUnsandboxedCommands` が有効な場合、モデルはツール入力で `dangerouslyDisableSandbox: true` を設定することで、サンドボックス外でコマンドを実行するようにリクエストできます。これらのリクエストは既存のパーミッションシステムにフォールバックします。つまり、`canUseTool` ハンドラーが呼び出され、カスタム認可ロジックを実装できます。

`excludedCommands` エントリは、代わりにサンドボックスを自動的にバイパスし、モデルの関与はありません。[`sandbox.excludedCommands`](/docs/ja/settings-reference#sandbox-excludedcommands) を参照して、エントリが適用される場合を確認してください。

下の例では、`isCommandAuthorized` は定義する認可チェックの代わりです。

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Deploy my application",
  options: {
    sandbox: {
      enabled: true,
      allowUnsandboxedCommands: true // モデルはサンドボックス外実行をリクエストできます
    },
    permissionMode: "default",
    canUseTool: async (tool, input) => {
      // モデルがサンドボックスをバイパスするようにリクエストしているかチェック
      if (tool === "Bash" && input.dangerouslyDisableSandbox) {
        // モデルはこのコマンドをサンドボックス外で実行するようにリクエストしています
        console.log(`Unsandboxed command requested: ${input.command}`);

        if (isCommandAuthorized(input.command)) {
          return { behavior: "allow" as const, updatedInput: input };
        }
        return {
          behavior: "deny" as const,
          message: "Command not authorized for unsandboxed execution"
        };
      }
      return { behavior: "allow" as const, updatedInput: input };
    }
  }
})) {
  if ("result" in message) console.log(message.result);
}
```

<Warning>
  `dangerouslyDisableSandbox: true` で実行されるコマンドはシステムへの完全なアクセスを持ちます。`canUseTool` ハンドラーがこれらのリクエストを慎重に検証することを確認してください。

  `permissionMode` が `bypassPermissions` に設定され、`allowUnsandboxedCommands` が有効な場合、モデルは [アクションなしモードが自動承認しないアクション](/docs/ja/permission-modes#actions-no-mode-auto-approves) を除き、承認プロンプトなしにサンドボックス外でコマンドを自律的に実行できます。この組み合わせにより、モデルはサンドボックス分離を静かにエスケープできます。
</Warning>

<h2 id="see-also">
  関連項目
</h2>

* [SDK 概要](/docs/ja/agent-sdk/overview) - 一般的な SDK 概念
* [Python SDK リファレンス](/docs/ja/agent-sdk/python) - Python SDK ドキュメント
* [CLI リファレンス](/docs/ja/cli-reference) - コマンドラインインターフェース
* [一般的なワークフロー](/docs/ja/common-workflows) - ステップバイステップガイド
