> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK 參考 - TypeScript

> TypeScript Agent SDK 的完整 API 參考，包括所有函數、類型和介面。

<script src="/docs/components/typescript-sdk-type-links.js" defer />

<h2 id="installation">
  安裝
</h2>

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

<Note>
  SDK 為您的平台捆綁了一個原生 Claude Code 二進制文件作為可選依賴項，例如 `@anthropic-ai/claude-agent-sdk-darwin-arm64`。您不需要單獨安裝 Claude Code。如果您的包管理器跳過可選依賴項，SDK 會拋出 `Native CLI binary for <platform> not found`；改為將 [`pathToClaudeCodeExecutable`](#options) 設置為單獨安裝的 `claude` 二進制文件。
</Note>

<h3 id="compile-to-a-single-executable">
  編譯為單個可執行文件
</h3>

當您使用 `bun build --compile` 將應用程序編譯為單個文件可執行文件時，SDK 無法在運行時解析捆綁的 CLI 二進制文件。`require.resolve` 在編譯後可執行文件的 `$bunfs` 虛擬文件系統內不起作用，因此 SDK 會拋出 `Native CLI binary for <platform> not found`。

要解決此問題，請將平台二進制文件嵌入為文件資產，在啟動時使用 `extractFromBunfs()` 將其提取到真實路徑，並將該路徑傳遞給 [`pathToClaudeCodeExecutable`](#options)。

`extractFromBunfs()` 輔助函數需要 `@anthropic-ai/claude-agent-sdk` v0.3.144 或更高版本。下面的示例為 Apple Silicon 上的 macOS 構建：

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

`extractFromBunfs()` 將嵌入的二進制文件從編譯後可執行文件的虛擬文件系統複製到每個用戶的臨時目錄，並返回真實路徑。在編譯後的可執行文件外，它返回輸入路徑不變，因此相同的代碼在開發中無需修改即可運行。

每個編譯後的可執行文件都嵌入單個平台的二進制文件。將導入中的平台包與您的 `--target` 匹配：

* 要進行交叉編譯，請安裝不匹配的平台包，例如 `npm install @anthropic-ai/claude-agent-sdk-linux-x64 --force`。
* 在 Windows 上，二進制子路徑是 `claude.exe`，例如 `@anthropic-ai/claude-agent-sdk-win32-x64/claude.exe`。

<h2 id="functions">
  函數
</h2>

<h3 id="query">
  `query()`
</h3>

與 Claude Code 互動的主要函數。創建一個異步生成器，在消息到達時流式傳輸消息。

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
  參數
</h4>

| 參數        | 類型                                                               | 描述                        |
| :-------- | :--------------------------------------------------------------- | :------------------------ |
| `prompt`  | `string \| AsyncIterable<`[`SDKUserMessage`](#sdkusermessage)`>` | 輸入提示，可以是字符串或異步可迭代對象用於流式模式 |
| `options` | [`Options`](#options)                                            | 可選配置對象（見下面的 Options 類型）   |

<h4 id="returns">
  返回值
</h4>

返回一個 [`Query`](#query-object) 對象，它擴展了 `AsyncGenerator<`[`SDKMessage`](#sdkmessage)`, void>` 並具有額外的方法。

<h3 id="startup">
  `startup()`
</h3>

通過生成 CLI 子進程並在提示可用之前完成初始化握手來預熱 CLI 子進程。返回的 [`WarmQuery`](#warmquery) 句柄稍後接受提示並將其寫入已準備好的進程，因此第一個 `query()` 調用解析時無需支付子進程生成和初始化成本。

```typescript theme={null}
function startup(params?: {
  options?: Options;
  initializeTimeoutMs?: number;
}): Promise<WarmQuery>;
```

<h4 id="parameters-2">
  參數
</h4>

| 參數                    | 類型                    | 描述                                                          |
| :-------------------- | :-------------------- | :---------------------------------------------------------- |
| `options`             | [`Options`](#options) | 可選配置對象。與 `query()` 的 `options` 參數相同                         |
| `initializeTimeoutMs` | `number`              | 等待子進程初始化的最大時間（毫秒）。默認為 `60000`。如果初始化未在時間內完成，promise 將以超時錯誤拒絕 |

<h4 id="returns-2">
  返回值
</h4>

返回一個 `Promise<`[`WarmQuery`](#warmquery)`>`，在子進程生成並完成其初始化握手後解析。

<h4 id="example">
  示例
</h4>

早期調用 `startup()`，例如在應用程序啟動時，然後在提示準備好後在返回的句柄上調用 `.query()`。這將子進程生成和初始化移出關鍵路徑。

```typescript theme={null}
import { startup } from "@anthropic-ai/claude-agent-sdk";

// 提前支付啟動成本
const warm = await startup({ options: { maxTurns: 3 } });

// 稍後，當提示準備好時，這是立即的
for await (const message of warm.query("What files are here?")) {
  console.log(message);
}
```

<h3 id="tool">
  `tool()`
</h3>

為與 SDK MCP 服務器一起使用創建類型安全的 MCP 工具定義。

```typescript theme={null}
function tool<Schema extends AnyZodRawShape>(
  name: string,
  description: string,
  inputSchema: Schema,
  handler: (args: InferShape<Schema>, extra: unknown) => Promise<CallToolResult>,
  extras?: { annotations?: ToolAnnotations }
): SdkMcpToolDefinition<Schema>;
```

<h4 id="parameters-3">
  參數
</h4>

| 參數            | 類型                                                                | 描述                                 |
| :------------ | :---------------------------------------------------------------- | :--------------------------------- |
| `name`        | `string`                                                          | 工具的名稱                              |
| `description` | `string`                                                          | 工具功能的描述                            |
| `inputSchema` | `Schema extends AnyZodRawShape`                                   | 定義工具輸入參數的 Zod 架構（支持 Zod 3 和 Zod 4） |
| `handler`     | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>` | 執行工具邏輯的異步函數                        |
| `extras`      | `{ annotations?: `[`ToolAnnotations`](#toolannotations)` }`       | 可選的 MCP 工具註釋，為客戶端提供行為提示            |

<h4 id="toolannotations">
  `ToolAnnotations`
</h4>

從 `@modelcontextprotocol/sdk/types.js` 重新導出。所有字段都是可選提示；客戶端不應依賴它們進行安全決策。

| 字段                | 類型        | 默認值         | 描述                                                             |
| :---------------- | :-------- | :---------- | :------------------------------------------------------------- |
| `title`           | `string`  | `undefined` | 工具的人類可讀標題                                                      |
| `readOnlyHint`    | `boolean` | `false`     | 如果為 `true`，工具不會修改其環境                                           |
| `destructiveHint` | `boolean` | `true`      | 如果為 `true`，工具可能執行破壞性更新（僅在 `readOnlyHint` 為 `false` 時有意義）       |
| `idempotentHint`  | `boolean` | `false`     | 如果為 `true`，使用相同參數的重複調用沒有額外效果（僅在 `readOnlyHint` 為 `false` 時有意義） |
| `openWorldHint`   | `boolean` | `true`      | 如果為 `true`，工具與外部實體交互（例如，網絡搜索）。如果為 `false`，工具的域是封閉的（例如，記憶工具）    |

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

創建在與應用程序相同的進程中運行的 MCP 服務器實例。

```typescript theme={null}
function createSdkMcpServer(options: {
  name: string;
  version?: string;
  tools?: Array<SdkMcpToolDefinition<any>>;
}): McpSdkServerConfigWithInstance;
```

<h4 id="parameters-4">
  參數
</h4>

| 參數                | 類型                            | 描述                             |
| :---------------- | :---------------------------- | :----------------------------- |
| `options.name`    | `string`                      | MCP 服務器的名稱                     |
| `options.version` | `string`                      | 可選版本字符串                        |
| `options.tools`   | `Array<SdkMcpToolDefinition>` | 使用 [`tool()`](#tool) 創建的工具定義數組 |

<h3 id="listsessions">
  `listSessions()`
</h3>

發現並列出具有輕量級元數據的過去會話。按項目目錄篩選或列出所有項目中的會話。

```typescript theme={null}
function listSessions(options?: ListSessionsOptions): Promise<SDKSessionInfo[]>;
```

<h4 id="parameters-5">
  參數
</h4>

| 參數                         | 類型        | 默認值         | 描述                                        |
| :------------------------- | :-------- | :---------- | :---------------------------------------- |
| `options.dir`              | `string`  | `undefined` | 列出會話的目錄。省略時，返回所有項目中的會話                    |
| `options.limit`            | `number`  | `undefined` | 返回的最大會話數                                  |
| `options.includeWorktrees` | `boolean` | `true`      | 當 `dir` 在 git 存儲庫內時，包括來自所有 worktree 路徑的會話 |

<h4 id="return-type-sdksessioninfo">
  返回類型：`SDKSessionInfo`
</h4>

| 屬性             | 類型                    | 描述                                         |
| :------------- | :-------------------- | :----------------------------------------- |
| `sessionId`    | `string`              | 唯一會話標識符（UUID）                              |
| `summary`      | `string`              | 顯示標題：自定義標題、自動生成的摘要或第一個提示                   |
| `lastModified` | `number`              | 上次修改時間（自紀元以來的毫秒數）                          |
| `fileSize`     | `number \| undefined` | 會話文件大小（字節）。僅針對本地 JSONL 存儲填充                |
| `customTitle`  | `string \| undefined` | 用戶設置的會話標題（通過 `/rename`）                    |
| `firstPrompt`  | `string \| undefined` | 會話中的第一個有意義的用戶提示                            |
| `gitBranch`    | `string \| undefined` | 會話結束時的 Git 分支                              |
| `cwd`          | `string \| undefined` | 會話的工作目錄                                    |
| `tag`          | `string \| undefined` | 用戶設置的會話標籤（見 [`tagSession()`](#tagsession)） |
| `createdAt`    | `number \| undefined` | 創建時間（自紀元以來的毫秒數），來自第一個條目的時間戳                |

<h4 id="example-2">
  示例
</h4>

打印項目的 10 個最近會話。結果按 `lastModified` 降序排序，因此第一項是最新的。省略 `dir` 以搜索所有項目。

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

從過去的會話記錄中讀取用戶和助手消息。

```typescript theme={null}
function getSessionMessages(
  sessionId: string,
  options?: GetSessionMessagesOptions
): Promise<SessionMessage[]>;
```

<h4 id="parameters-6">
  參數
</h4>

| 參數               | 類型       | 默認值         | 描述                              |
| :--------------- | :------- | :---------- | :------------------------------ |
| `sessionId`      | `string` | 必需          | 要讀取的會話 UUID（見 `listSessions()`） |
| `options.dir`    | `string` | `undefined` | 查找會話的項目目錄。省略時，搜索所有項目            |
| `options.limit`  | `number` | `undefined` | 返回的最大消息數                        |
| `options.offset` | `number` | `undefined` | 從開始跳過的消息數                       |

<h4 id="return-type-sessionmessage">
  返回類型：`SessionMessage`
</h4>

| 屬性                   | 類型                      | 描述                                                                                                                                         |
| :------------------- | :---------------------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| `type`               | `"user" \| "assistant"` | 消息角色                                                                                                                                       |
| `uuid`               | `string`                | 唯一消息標識符                                                                                                                                    |
| `session_id`         | `string`                | 此消息所屬的會話                                                                                                                                   |
| `message`            | `unknown`               | 來自記錄的原始消息有效負載                                                                                                                              |
| `parent_tool_use_id` | `string \| null`        | 對於子代理消息，生成 `Agent` 工具調用的 `tool_use_id`。對於主會話消息和較舊的會話為 `null`                                                                               |
| `parent_agent_id`    | `string \| null`        | 對於來自[嵌套子代理](/docs/zh-TW/sub-agents#spawn-nested-subagents)的消息，生成它的子代理的 `agentId`。對於主會話消息、來自頂級子代理的消息和較舊的會話為 `null`。需要 Claude Code v2.1.202 或更高版本 |

<h4 id="example-3">
  示例
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

按 ID 讀取單個會話的元數據，無需掃描完整項目目錄。

```typescript theme={null}
function getSessionInfo(
  sessionId: string,
  options?: GetSessionInfoOptions
): Promise<SDKSessionInfo | undefined>;
```

<h4 id="parameters-7">
  參數
</h4>

| 參數            | 類型       | 默認值         | 描述                  |
| :------------ | :------- | :---------- | :------------------ |
| `sessionId`   | `string` | 必需          | 要查找的會話 UUID         |
| `options.dir` | `string` | `undefined` | 項目目錄路徑。省略時，搜索所有項目目錄 |

返回 [`SDKSessionInfo`](#return-type-sdksessioninfo)，如果找不到會話則返回 `undefined`。

<h3 id="renamesession">
  `renameSession()`
</h3>

通過附加自定義標題條目來重命名會話。重複調用是安全的；最新的標題獲勝。

```typescript theme={null}
function renameSession(
  sessionId: string,
  title: string,
  options?: SessionMutationOptions
): Promise<void>;
```

<h4 id="parameters-8">
  參數
</h4>

| 參數            | 類型       | 默認值         | 描述                  |
| :------------ | :------- | :---------- | :------------------ |
| `sessionId`   | `string` | 必需          | 要重命名的會話 UUID        |
| `title`       | `string` | 必需          | 新標題。修剪空格後必須非空       |
| `options.dir` | `string` | `undefined` | 項目目錄路徑。省略時，搜索所有項目目錄 |

<h3 id="tagsession">
  `tagSession()`
</h3>

標記會話。傳遞 `null` 以清除標籤。重複調用是安全的；最新的標籤獲勝。

```typescript theme={null}
function tagSession(
  sessionId: string,
  tag: string | null,
  options?: SessionMutationOptions
): Promise<void>;
```

<h4 id="parameters-9">
  參數
</h4>

| 參數            | 類型               | 默認值         | 描述                  |
| :------------ | :--------------- | :---------- | :------------------ |
| `sessionId`   | `string`         | 必需          | 要標記的會話 UUID         |
| `tag`         | `string \| null` | 必需          | 標籤字符串，或 `null` 以清除  |
| `options.dir` | `string`         | `undefined` | 項目目錄路徑。省略時，搜索所有項目目錄 |

<h3 id="resolvesettings">
  `resolveSettings()`
</h3>

使用與 CLI 相同的合併引擎為給定目錄解析有效的 Claude Code 設定，無需生成 Claude CLI。在調用 `query()` 之前使用它來檢查 `query()` 調用將看到什麼配置。

<Note>
  此函數處於 alpha 階段，其 API 在穩定之前可能會更改。它讀取 MDM 源，包括 macOS plist 和 Windows HKLM/HKCU，以與 CLI 啟動保持一致，但不執行管理員配置的 `policyHelper` 子進程。`permissions.defaultMode` 字段從所有層級（包括項目設定）按原樣返回。CLI 在遵守升級權限模式之前應用的信任過濾器不被應用。
</Note>

```typescript theme={null}
function resolveSettings(
  options?: ResolveSettingsOptions
): Promise<ResolvedSettings>;
```

<h4 id="parameters-10">
  參數
</h4>

`resolveSettings()` 接受單個選項對象。所有字段都是可選的。

| 參數                              | 類型                                    | 默認值             | 描述                                                                                                                                                                |
| :------------------------------ | :------------------------------------ | :-------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.cwd`                   | `string`                              | `process.cwd()` | 用於解析項目和本地設定的相對目錄                                                                                                                                                  |
| `options.settingSources`        | [`SettingSource`](#settingsource)`[]` | 所有源             | 要加載的文件系統源。傳遞 `[]` 以跳過用戶、項目和本地設定。託管策略設定在所有情況下都會加載。伺服器託管設定取自 `serverManagedSettings`（當主機傳遞時），或從 CLI 的磁盤上緩存讀取；快照不會從網絡獲取它們                                            |
| `options.managedSettings`       | `Settings`                            | `undefined`     | 由嵌入主機提供的限制性策略層設定。當存在管理員部署的託管層時被丟棄；當 [`parentSettingsBehavior`](/docs/zh-TW/settings#available-settings) 為 `"merge"` 時在該層下合併。非限制性鍵（如 `model`）會被靜默丟棄，以便此選項可以加強託管策略但不能放寬它 |
| `options.serverManagedSettings` | `Settings`                            | `undefined`     | 來自 `/api/claude_code/settings` 的服務器託管設定有效負載。非限制性鍵無過濾地通過                                                                                                           |

<h4 id="return-type-resolvedsettings">
  返回類型：`ResolvedSettings`
</h4>

`resolveSettings()` 返回一個對象，描述合併的設定和為每個鍵提供的源。

| 屬性           | 類型                                                  | 描述                              |
| :----------- | :-------------------------------------------------- | :------------------------------ |
| `effective`  | `Settings`                                          | 在優先級順序中應用所有啟用源後的合併設定            |
| `provenance` | `Partial<Record<keyof Settings, ProvenanceEntry>>`  | 對於 `effective` 中的每個頂級鍵，哪個源提供了該值 |
| `sources`    | `Array<{ source, settings, path?, policyOrigin? }>` | 每個源的原始設定，按從最低到最高優先級排序           |

<h4 id="example-4">
  示例
</h4>

下面的示例為項目目錄解析設定並打印控制清理期的源。

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
  類型
</h2>

<h3 id="options">
  `Options`
</h3>

`query()` 函數的配置對象。

| 屬性                                | 類型                                                                                                       | 默認值                           | 描述                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------- | :---------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `abortController`                 | `AbortController`                                                                                        | `new AbortController()`       | 用於取消操作的控制器                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `additionalDirectories`           | `string[]`                                                                                               | `[]`                          | Claude 可以訪問的其他目錄                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `agent`                           | `string`                                                                                                 | `undefined`                   | 主線程的代理名稱。代理必須在 `agents` 選項或設置中定義                                                                                                                                                                                                                                                                                                                                                                                                     |
| `agents`                          | `Record<string, [`AgentDefinition`](#agentdefinition)>`                                                  | `undefined`                   | 以編程方式定義子代理                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `agentProgressSummaries`          | `boolean`                                                                                                | `false`                       | 當為 `true` 時，為子代理生成單行進度摘要，並通過 `summary` 字段在 [`task_progress`](#sdktaskprogressmessage) 事件上轉發它們。適用於前景和背景子代理                                                                                                                                                                                                                                                                                                                            |
| `allowDangerouslySkipPermissions` | `boolean`                                                                                                | `false`                       | 啟用繞過權限。使用 `permissionMode: 'bypassPermissions'` 時需要                                                                                                                                                                                                                                                                                                                                                                                  |
| `allowedTools`                    | `string[]`                                                                                               | `[]`                          | 無需提示即可自動批准的工具。這不會將 Claude 限制為僅這些工具；未列出的工具會進入 `permissionMode` 和 `canUseTool`。使用 `disallowedTools` 來阻止工具。見 [Permissions](/docs/zh-TW/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                           |
| `betas`                           | [`SdkBeta`](#sdkbeta)`[]`                                                                                | `[]`                          | 啟用測試功能                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `canUseTool`                      | [`CanUseTool`](#canusetool)                                                                              | `undefined`                   | 自定義權限函數，僅在 [permission flow](/docs/zh-TW/agent-sdk/permissions#how-permissions-are-evaluated) 進入提示時調用。不會為 `allowedTools`、allow 規則或 `permissionMode` 自動批准的調用調用。`AskUserQuestion`、connector tools [您的組織設置為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools) 和標記為 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP tools 即使您已允許它們也會到達它；在 `dontAsk` 模式下這些會被拒絕。見 [`CanUseTool`](#canusetool) 了解詳情 |
| `continue`                        | `boolean`                                                                                                | `false`                       | 繼續最近的對話                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `cwd`                             | `string`                                                                                                 | `process.cwd()`               | 當前工作目錄                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `debug`                           | `boolean`                                                                                                | `false`                       | 為 Claude Code 進程啟用調試模式                                                                                                                                                                                                                                                                                                                                                                                                               |
| `debugFile`                       | `string`                                                                                                 | `undefined`                   | 將調試日誌寫入特定文件路徑。隱式啟用調試模式                                                                                                                                                                                                                                                                                                                                                                                                               |
| `disallowedTools`                 | `string[]`                                                                                               | `[]`                          | 要拒絕的工具。裸名稱如 `"Bash"` 會從 Claude 的上下文中移除該工具。作用域規則如 `"Bash(rm *)"` 會保留該工具可用，並在每個權限模式中拒絕匹配的調用，包括 `bypassPermissions`。見 [Permissions](/docs/zh-TW/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                  |
| `effort`                          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max'`                                                        | 模型默認值                         | 控制 Claude 在其響應中投入多少努力。與自適應思考一起工作以指導思考深度。見 [adjust the effort level](/docs/zh-TW/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                         |
| `enableFileCheckpointing`         | `boolean`                                                                                                | `false`                       | 啟用文件更改跟蹤以進行回滾。見 [File checkpointing](/docs/zh-TW/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                            |
| `env`                             | `Record<string, string \| undefined>`                                                                    | `process.env`                 | 環境變量。設置此項時，這會替換子進程環境而不是與 `process.env` 合併，因此傳遞 `{ ...process.env, YOUR_VAR: 'value' }` 以保留繼承的變量如 `PATH`。見 [Handle slow or stalled API responses](#handle-slow-or-stalled-api-responses) 了解此模式的示例，以及 [Environment variables](/docs/zh-TW/env-vars) 了解底層 CLI 讀取的變量。設置 `CLAUDE_AGENT_SDK_CLIENT_APP` 以在 User-Agent 標頭中標識您的應用程序                                                                                                               |
| `executable`                      | `'bun' \| 'deno' \| 'node'`                                                                              | 自動檢測                          | 要使用的 JavaScript 運行時                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `executableArgs`                  | `string[]`                                                                                               | `[]`                          | 傳遞給可執行文件的參數                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `extraArgs`                       | `Record<string, string \| null>`                                                                         | `{}`                          | 其他參數                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `fallbackModel`                   | `string`                                                                                                 | `undefined`                   | 主模型失敗時使用的模型                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `forkSession`                     | `boolean`                                                                                                | `false`                       | 使用 `resume` 恢復時，分叉到新會話 ID 而不是繼續原始會話                                                                                                                                                                                                                                                                                                                                                                                                  |
| `forwardSubagentText`             | `boolean`                                                                                                | `false`                       | 轉發子代理文本和思考塊作為助手和用戶消息，並設置 `parent_tool_use_id`，以便消費者可以呈現嵌套記錄。默認情況下，只有來自子代理的 `tool_use` 和 `tool_result` 塊被發出                                                                                                                                                                                                                                                                                                                           |
| `hooks`                           | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>`        | `{}`                          | 事件的 hooks 回調                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `includeHookEvents`               | `boolean`                                                                                                | `false`                       | 將 hooks 生命週期事件包括在消息流中，作為 [`SDKHookStartedMessage`](#sdkhookstartedmessage)、[`SDKHookProgressMessage`](#sdkhookprogressmessage) 和 [`SDKHookResponseMessage`](#sdkhookresponsemessage)。`SessionStart` 和 `Setup` hooks 的生命週期事件始終包括在內，不需要此選項                                                                                                                                                                                             |
| `includePartialMessages`          | `boolean`                                                                                                | `false`                       | 包括部分消息事件                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `loadTimeoutMs`                   | `number`                                                                                                 | `60000`                       | *Alpha.* 在恢復物化期間，每個 `sessionStore.load()` 和 `sessionStore.listSubkeys()` 調用的超時時間（以毫秒為單位）。如果適配器未在此窗口內解決，查詢將失敗而不是掛起。未設置 `sessionStore` 時忽略                                                                                                                                                                                                                                                                                             |
| `managedSettings`                 | `Settings`                                                                                               | `undefined`                   | 由生成父進程提供的策略層設置。當機器上已存在 IT 控制的託管設置層時被丟棄，除非該管理員選擇使用 `parentSettingsBehavior: 'merge'`。無論如何都被過濾為僅限制性鍵                                                                                                                                                                                                                                                                                                                                   |
| `maxBudgetUsd`                    | `number`                                                                                                 | `undefined`                   | 當客戶端成本估計達到此 USD 值時停止查詢。與 `total_cost_usd` 的相同估計進行比較；見 [Track cost and usage](/docs/zh-TW/agent-sdk/cost-tracking) 了解準確性注意事項                                                                                                                                                                                                                                                                                                               |
| `maxThinkingTokens`               | `number`                                                                                                 | `undefined`                   | *已棄用：* 改用 `thinking`。思考過程的最大令牌數                                                                                                                                                                                                                                                                                                                                                                                                      |
| `maxTurns`                        | `number`                                                                                                 | `undefined`                   | 最大代理轉數（工具使用往返）                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `mcpServers`                      | `Record<string, [`McpServerConfig`](#mcpserverconfig)>`                                                  | `{}`                          | MCP 服務器配置                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `model`                           | `string`                                                                                                 | CLI 默認值                       | Claude 模型別名或完整模型名稱。見 [accepted values and provider-specific IDs](/docs/zh-TW/model-config#available-models)                                                                                                                                                                                                                                                                                                                               |
| `onElicitation`                   | `(request: ElicitationRequest, options: { signal: AbortSignal }) => Promise<ElicitationResult>`          | `undefined`                   | 用於處理 MCP elicitation 請求的回調。當 MCP 服務器請求用戶輸入且沒有 hooks 首先處理它時調用。未提供時，未處理的 elicitation 請求會自動被拒絕                                                                                                                                                                                                                                                                                                                                          |
| `outputFormat`                    | `{ type: 'json_schema', schema: JSONSchema }`                                                            | `undefined`                   | 為代理結果定義輸出格式。見 [Structured outputs](/docs/zh-TW/agent-sdk/structured-outputs) 了解詳情                                                                                                                                                                                                                                                                                                                                                         |
| `outputStyle`                     | `string`                                                                                                 | `undefined`                   | 不是 `Options` 字段。改為在內聯 [`settings`](/docs/zh-TW/settings) 對象或設置文件中設置 `outputStyle`。見 [Activate an output style](/docs/zh-TW/agent-sdk/modifying-system-prompts#activate-an-output-style)                                                                                                                                                                                                                                                        |
| `pathToClaudeCodeExecutable`      | `string`                                                                                                 | 從捆綁的原生二進制文件自動解析               | Claude Code 可執行文件的路徑。僅在安裝期間跳過可選依賴項或您的平台不在支持的集合中時需要                                                                                                                                                                                                                                                                                                                                                                                   |
| `permissionMode`                  | [`PermissionMode`](#permissionmode)                                                                      | `'default'`                   | 會話的權限模式                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `permissionPromptToolName`        | `string`                                                                                                 | `undefined`                   | 權限提示的 MCP 工具名稱                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `persistSession`                  | `boolean`                                                                                                | `true`                        | 當為 `false` 時，禁用會話持久化到磁盤。會話之後無法恢復                                                                                                                                                                                                                                                                                                                                                                                                     |
| `planModeInstructions`            | `string`                                                                                                 | `undefined`                   | Plan Mode 的自定義工作流指令。當 `permissionMode` 為 `'plan'` 時，此字符串替換默認 Plan Mode 工作流正文。CLI 仍然使用只讀強制前言和 ExitPlanMode 協議頁腳包裝它                                                                                                                                                                                                                                                                                                                    |
| `plugins`                         | [`SdkPluginConfig`](#sdkpluginconfig)`[]`                                                                | `[]`                          | 從本地路徑加載自定義 plugins。見 [Plugins](/docs/zh-TW/agent-sdk/plugins) 了解詳情                                                                                                                                                                                                                                                                                                                                                                        |
| `promptSuggestions`               | `boolean`                                                                                                | `false`                       | 啟用提示建議。在每個轉數後發出 `prompt_suggestion` 消息，帶有預測的下一個用戶提示                                                                                                                                                                                                                                                                                                                                                                                  |
| `resume`                          | `string`                                                                                                 | `undefined`                   | 要恢復的會話 ID                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `resumeSessionAt`                 | `string`                                                                                                 | `undefined`                   | 在特定消息 UUID 處恢復會話                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `sandbox`                         | [`SandboxSettings`](#sandboxsettings)                                                                    | `undefined`                   | 以編程方式配置 sandbox 行為。見 [Sandbox settings](#sandboxsettings) 了解詳情                                                                                                                                                                                                                                                                                                                                                                       |
| `sessionId`                       | `string`                                                                                                 | 自動生成                          | 使用特定 UUID 作為會話，而不是自動生成一個                                                                                                                                                                                                                                                                                                                                                                                                             |
| `sessionStore`                    | [`SessionStore`](/docs/zh-TW/agent-sdk/session-storage#the-sessionstore-interface)                            | `undefined`                   | 將會話記錄鏡像到外部後端，以便任何主機都可以恢復它們。見 [Persist sessions to external storage](/docs/zh-TW/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                |
| `sessionStoreFlush`               | `'batched' \| 'eager'`                                                                                   | `'batched'`                   | *Alpha.* `sessionStore` 的刷新模式。未設置 `sessionStore` 時忽略                                                                                                                                                                                                                                                                                                                                                                                 |
| `settings`                        | `string \| Settings`                                                                                     | `undefined`                   | 內聯 [settings](/docs/zh-TW/settings) 對象或設置文件的路徑。填充 [precedence order](/docs/zh-TW/settings#settings-precedence) 中的標誌設置層。使用 [`applyFlagSettings()`](#applyflagsettings) 在運行時更改                                                                                                                                                                                                                                                                   |
| `settingSources`                  | [`SettingSource`](#settingsource)`[]`                                                                    | CLI 默認值（所有源）                  | 控制加載哪些文件系統設置。傳遞 `[]` 以禁用用戶、項目和本地設置。無論如何都會加載 [Endpoint-managed policy](/docs/zh-TW/settings#settings-files)；當會話使用組織憑證在 [eligible configuration](/docs/zh-TW/server-managed-settings#platform-availability) 上進行身份驗證時，會獲取服務器管理的設置。見 [Use Claude Code features](/docs/zh-TW/agent-sdk/claude-code-features#what-settingsources-does-not-control)                                                                                                        |
| `skills`                          | `string[] \| 'all'`                                                                                      | `undefined`                   | 會話可用的 skills。傳遞 `'all'` 以啟用每個發現的 skill，或傳遞 skill 名稱列表。設置後，SDK 會自動將 Skill 工具添加到 `allowedTools`。如果您也傳遞 `tools`，請在該列表中包含 `'Skill'`。見 [Skills](/docs/zh-TW/agent-sdk/skills)                                                                                                                                                                                                                                                                  |
| `spawnClaudeCodeProcess`          | `(options: SpawnOptions) => SpawnedProcess`                                                              | `undefined`                   | 用於生成 Claude Code 進程的自定義函數。用於在 VM、容器或遠程環境中運行 Claude Code                                                                                                                                                                                                                                                                                                                                                                              |
| `stderr`                          | `(data: string) => void`                                                                                 | `undefined`                   | stderr 輸出的回調                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `strictMcpConfig`                 | `boolean`                                                                                                | `false`                       | 僅使用在 `mcpServers` 中傳遞的服務器，並忽略項目 `.mcp.json`、用戶設置、plugin 提供的 MCP 服務器和 [claude.ai connectors](/docs/zh-TW/mcp#use-mcp-servers-from-claude-ai)                                                                                                                                                                                                                                                                                               |
| `systemPrompt`                    | `string \| { type: 'preset'; preset: 'claude_code'; append?: string; excludeDynamicSections?: boolean }` | `undefined`（最小提示）             | 系統提示配置。傳遞字符串以獲得自定義提示，或 `{ type: 'preset', preset: 'claude_code' }` 以使用 Claude Code 的系統提示。使用預設對象形式時，添加 `append` 以使用其他指令擴展它，並設置 `excludeDynamicSections: true` 以將每個會話上下文移到第一個用戶消息中以獲得 [better prompt-cache reuse across machines](/docs/zh-TW/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines)                                                                                                          |
| `taskBudget`                      | `{ total: number }`                                                                                      | `undefined`                   | *Alpha.* API 端任務預算（以令牌為單位）。設置後，模型會被告知其剩餘令牌預算，以便它可以調整工具使用速度並在達到限制前完成                                                                                                                                                                                                                                                                                                                                                                  |
| `thinking`                        | [`ThinkingConfig`](#thinkingconfig)                                                                      | 支持的模型為 `{ type: 'adaptive' }` | 控制 Claude 的思考/推理行為。見 [`ThinkingConfig`](#thinkingconfig) 了解選項                                                                                                                                                                                                                                                                                                                                                                        |
| `title`                           | `string`                                                                                                 | `undefined`                   | 會話的顯示標題。通過 `resume` 或 `continue` 恢復時，恢復的會話的持久化標題優先；使用 [`renameSession()`](#renamesession) 重新標題現有會話                                                                                                                                                                                                                                                                                                                                   |
| `toolAliases`                     | `Record<string, string>`                                                                                 | `undefined`                   | 將內置工具名稱映射到 MCP 工具名稱，以便 Claude 調用您的 MCP 實現而不是內置工具。例如，`{ Bash: 'mcp__workspace__bash' }`                                                                                                                                                                                                                                                                                                                                               |
| `toolConfig`                      | [`ToolConfig`](#toolconfig)                                                                              | `undefined`                   | 內置工具行為的配置。見 [`ToolConfig`](#toolconfig) 了解詳情                                                                                                                                                                                                                                                                                                                                                                                         |
| `tools`                           | `string[] \| { type: 'preset'; preset: 'claude_code' }`                                                  | `undefined`                   | 工具配置。傳遞工具名稱數組或使用預設以獲得 Claude Code 的默認工具                                                                                                                                                                                                                                                                                                                                                                                              |

<h4 id="handle-slow-or-stalled-api-responses">
  Handle slow or stalled API responses
</h4>

CLI 子進程讀取多個環境變量，這些變量控制 API 超時和停滯檢測。通過 `env` 選項傳遞它們：

```typescript theme={null}
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

* `API_TIMEOUT_MS`：Anthropic 客戶端上的每個請求超時，以毫秒為單位。默認 `600000`。適用於主循環和所有子代理。
* `CLAUDE_CODE_MAX_RETRIES`：最大 API 重試次數。默認 `10`，上限為 `15`。每次重試都有自己的 `API_TIMEOUT_MS` 窗口，因此最壞情況下的牆時間大約是 `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` 加上退避。對於需要等待更長時間中斷的無人值守運行，設置 `CLAUDE_CODE_RETRY_WATCHDOG=1`：它無限期重試容量錯誤，自 Claude Code v2.1.199 起，為其他瞬時錯誤提高默認值至 `300` 並移除此變量的上限。
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`：使用 `run_in_background` 啟動的子代理的停滯監視程序。默認 `600000`。在每個流事件上重置；在停滯時中止子代理，將任務標記為失敗，並將錯誤與任何部分結果一起呈現給父代理。不適用於同步子代理。
* `CLAUDE_ENABLE_STREAM_WATCHDOG` 與 `CLAUDE_STREAM_IDLE_TIMEOUT_MS`：當標頭已到達但響應正文停止流式傳輸時中止請求。監視程序對所有提供商默認開啟；設置 `CLAUDE_ENABLE_STREAM_WATCHDOG=0` 以禁用它。`CLAUDE_STREAM_IDLE_TIMEOUT_MS` 默認為 `300000` 並被限制為該最小值。中止的請求通過正常重試路徑進行。

<h3 id="query-object">
  `Query` object
</h3>

由 `query()` 函數返回的介面。

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
  applyFlagSettings(settings: { [K in keyof Settings]?: Settings[K] | null }): Promise<void>;
  initializationResult(): Promise<SDKControlInitializeResponse>;
  reinitialize(): Promise<SDKControlInitializeResponse>;
  supportedCommands(): Promise<SlashCommand[]>;
  supportedModels(): Promise<ModelInfo[]>;
  supportedAgents(): Promise<AgentInfo[]>;
  mcpServerStatus(): Promise<McpServerStatus[]>;
  accountInfo(): Promise<AccountInfo>;
  reconnectMcpServer(serverName: string): Promise<void>;
  toggleMcpServer(serverName: string, enabled: boolean): Promise<void>;
  setMcpServers(servers: Record<string, McpServerConfig>): Promise<McpSetServersResult>;
  streamInput(stream: AsyncIterable<SDKUserMessage>): Promise<void>;
  stopTask(taskId: string): Promise<void>;
  close(): void;
}
```

<h4 id="methods">
  Methods
</h4>

| 方法                                     | 描述                                                                                                                                                                                                                                 |
| :------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt()`                          | 中斷查詢。僅在流式輸入模式下可用。當 CLI 在 [`SDKSystemMessage.capabilities`](#sdksystemmessage) 中公告 `interrupt_receipt_v1` 功能時，使用列出存活中斷的排隊消息的 [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) 進行解決。在 v2.1.205 之前的 CLI 上解決為 `undefined` |
| `rewindFiles(userMessageId, options?)` | 將文件恢復到指定用戶消息時的狀態。傳遞 `{ dryRun: true }` 以預覽更改。需要 `enableFileCheckpointing: true`。見 [File checkpointing](/docs/zh-TW/agent-sdk/file-checkpointing)                                                                                        |
| `setPermissionMode()`                  | 更改權限模式（僅在流式輸入模式下可用）                                                                                                                                                                                                                |
| `setModel()`                           | 更改模型（僅在流式輸入模式下可用）                                                                                                                                                                                                                  |
| `setMaxThinkingTokens()`               | *已棄用：* 改用 `thinking` 選項。更改最大思考令牌                                                                                                                                                                                                   |
| `applyFlagSettings(settings)`          | 在運行時將設置合併到會話的標誌設置層中（僅在流式輸入模式下可用）。見 [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                     |
| `initializationResult()`               | 返回完整的初始化結果，包括支持的命令、模型、帳戶信息和輸出樣式配置                                                                                                                                                                                                  |
| `reinitialize()`                       | 重新發送 `initialize` 控制請求到運行的 CLI，並返回新鮮的結果而不是緩存的首次連接結果。在傳輸間隙後使用它，例如在斷開連接後重新附加到會話，以便待處理的權限請求再次到達您的 `canUseTool` 回調。使回調對每個請求 ID 冪等，因為響應丟失的請求會再次被分派。需要 Claude Code v2.1.195 或更高版本                                                        |
| `supportedCommands()`                  | 返回可用的 slash commands                                                                                                                                                                                                               |
| `supportedModels()`                    | 返回具有顯示信息的可用模型                                                                                                                                                                                                                      |
| `supportedAgents()`                    | 返回可用的子代理，作為 [`AgentInfo`](#agentinfo)`[]`                                                                                                                                                                                          |
| `mcpServerStatus()`                    | 返回連接的 MCP 服務器的狀態                                                                                                                                                                                                                   |
| `accountInfo()`                        | 返回帳戶信息                                                                                                                                                                                                                             |
| `reconnectMcpServer(serverName)`       | 按名稱重新連接 MCP 服務器                                                                                                                                                                                                                    |
| `toggleMcpServer(serverName, enabled)` | 按名稱啟用或禁用 MCP 服務器                                                                                                                                                                                                                   |
| `setMcpServers(servers)`               | 動態替換此會話的 MCP 服務器集。返回有關添加、刪除的服務器和任何錯誤的信息                                                                                                                                                                                            |
| `streamInput(stream)`                  | 將輸入消息流式傳輸到查詢以進行多輪對話                                                                                                                                                                                                                |
| `stopTask(taskId)`                     | 按 ID 停止運行的後台任務                                                                                                                                                                                                                     |
| `close()`                              | 關閉查詢並終止底層進程。強制結束查詢並清理所有資源                                                                                                                                                                                                          |

<h4 id="applyflagsettings">
  `applyFlagSettings()`
</h4>

在運行會話上更改任何 [settings](/docs/zh-TW/settings)，無需重新啟動查詢。當沒有專用設置器的設置需要在會話中期更改時使用它，例如在代理讀取不受信任的輸入後收緊 `permissions`。`setModel()` 和 `setPermissionMode()` 是這兩個鍵的專用設置器；`applyFlagSettings()` 是接受任何設置鍵子集的通用形式，在此處傳遞 `model` 的行為與 `setModel()` 相同。

只有某些鍵在會話中期生效：

* **在下一個轉數上應用**：`model`、`effortLevel`、`ultracode`、`permissions`、`hooks`、`skillOverrides`、`fastMode`、`agent`。切換 `agent` 也會在下一個轉數上應用該代理的模型覆蓋、hooks 和系統提示。
* **在會話中期無效**：系統提示選項。這些在啟動時解決一次，因此運行會話保持原始值，即使調用成功。要更改它們，請啟動新會話。

`effortLevel` 接受 [effort level](/docs/zh-TW/model-config#adjust-effort-level) 名稱。它也接受 `"ultracode"`，它以 `xhigh` 努力運行會話並打開 [ultracode](/docs/zh-TW/workflows#let-claude-decide-with-ultracode)。`Settings` 類型聲明 `effortLevel` 沒有該值，因此在 TypeScript 中傳遞等效的 `{ ultracode: true }`。`ultracode` 值需要 Claude Code v2.1.203 或更高版本，並且僅由 `applyFlagSettings()` 接受，不由設置文件中的 `effortLevel` 鍵接受。

這些值被寫入標誌設置層，這是內聯 `query()` 的 `settings` 選項在啟動時填充的同一層。標誌設置位於 [settings precedence order](/docs/zh-TW/settings#settings-precedence) 的頂部附近：它們覆蓋用戶、項目和本地設置，只有託管策略設置可以覆蓋它們。這是 [on-page precedence section](#settings-precedence) 稱為編程選項的同一層。

連續調用淺合併頂級鍵。第二次調用 `{ permissions: {...} }` 會替換先前調用中的整個 `permissions` 對象，而不是深度合併到其中。要從標誌層清除鍵並回退到較低優先級源，請為該鍵傳遞 `null`。傳遞 `undefined` 沒有效果，因為 JSON 序列化會將其刪除。

僅在流式輸入模式下可用，與 `setModel()` 和 `setPermissionMode()` 的約束相同。

下面的示例在會話中期切換活動模型，然後清除覆蓋，以便模型回退到用戶或項目設置指定的任何內容。

```typescript theme={null}
const q = query({ prompt: messageStream });

// 覆蓋會話其餘部分的模型
await q.applyFlagSettings({ model: "claude-opus-4-6" });

// 稍後：清除覆蓋並回退到較低優先級設置
await q.applyFlagSettings({ model: null });
```

<Note>
  `applyFlagSettings()` 僅適用於 TypeScript。Python SDK 不公開等效方法。
</Note>

<h3 id="warmquery">
  `WarmQuery`
</h3>

由 [`startup()`](#startup) 返回的句柄。子進程已生成並初始化，因此在此句柄上調用 `query()` 會直接將提示寫入準備好的進程，無需啟動延遲。

```typescript theme={null}
interface WarmQuery extends AsyncDisposable {
  query(prompt: string | AsyncIterable<SDKUserMessage>): Query;
  close(): void;
}
```

<h4 id="methods-2">
  Methods
</h4>

| 方法              | 描述                                                            |
| :-------------- | :------------------------------------------------------------ |
| `query(prompt)` | 向預熱的子進程發送提示並返回 [`Query`](#query-object)。每個 `WarmQuery` 只能調用一次 |
| `close()`       | 關閉子進程而不發送提示。使用此方法丟棄不再需要的預熱查詢                                  |

`WarmQuery` 實現 `AsyncDisposable`，因此可以與 `await using` 一起使用以進行自動清理。

<h3 id="sdkcontrolinitializeresponse">
  `SDKControlInitializeResponse`
</h3>

`initializationResult()` 的返回類型。包含會話初始化數據。

```typescript theme={null}
type SDKControlInitializeResponse = {
  commands: SlashCommand[];
  agents: AgentInfo[];
  output_style: string;
  available_output_styles: string[];
  models: ModelInfo[];
  account: AccountInfo;
  fast_mode_state?: "off" | "cooldown" | "on";
};
```

當客戶端向已運行的會話發送 `initialize` 時，控制響應包裝器也會帶有可選的 `pending_permission_requests` 數組。該字段位於響應包裝器本身上，而不是上面的 `SDKControlInitializeResponse` 有效負載中。每個條目都是一個完整的 `control_request` 消息，具有與會話在運行時為權限請求流式傳輸的相同 `{ type: "control_request", request_id, request }` 形狀。

這些是在客戶端連接之前發出的請求，仍在等待回复。SDK 為您讀取數組並將每個條目分派到您的 [`canUseTool`](#canusetool) 回調，這是 [`reinitialize()`](#query-object) 在傳輸間隙後觸發的相同重新傳遞。以冪等方式處理重複的請求 ID，因為一個條目可以重複回調已經收到的請求，然後連接斷開。

<h3 id="sdkcontrolinterruptresponse">
  `SDKControlInterruptResponse`
</h3>

中斷收據：[`interrupt()`](#query-object) 在公告 [`SDKSystemMessage.capabilities`](#sdksystemmessage) 中的 `interrupt_receipt_v1` 功能的 CLI 上解決的值。需要 Claude Code v2.1.205 或更高版本。較早的 CLI 使用空成功有效負載回答中斷，因此 `interrupt()` 解決為 `undefined`。

```typescript theme={null}
type SDKControlInterruptResponse = {
  still_queued: string[];
};
```

`still_queued` 列出存活中斷的用戶消息的 UUID：仍在隊列中的消息，加上任何已出隊用於下一個轉數但尚未被中止到達的批次。除非您先取消它，否則每個都作為其自己的轉數在中斷後運行。使用收據決定是否重新發送任何內容；重新發送已列出的消息會產生重複轉數。

使用這些注意事項解釋列表：

* 僅出現已使用 UUID 入隊的消息。空數組並不意味著沒有其他內容會運行。
* 僅列出主線程消息。發送給子代理的消息超出範圍。
* 列表可以包括您的客戶端從未發送的 UUID，例如 [scheduled task](/docs/zh-TW/scheduled-tasks) 觸發器。忽略您不認識的 UUID，而不是將其視為錯誤。

收據是在處理中斷時拍攝的快照，在乾淨中斷時，它在中斷轉數的 [`SDKResultMessage`](#sdkresultmessage) 之前到達。在該結果之後讀取收據而不是檢查隊列：循環立即啟動下一個排隊轉數，因此您在結果後檢查的隊列已經改變。

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

以編程方式定義的子代理的配置。

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
  memory?: "user" | "project" | "local";
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | number;
  permissionMode?: PermissionMode;
  criticalSystemReminder_EXPERIMENTAL?: string;
};
```

| 字段                                    | 必需 | 描述                                                                                                        |
| :------------------------------------ | :- | :-------------------------------------------------------------------------------------------------------- |
| `description`                         | 是  | 何時使用此代理的自然語言描述                                                                                            |
| `tools`                               | 否  | 允許的工具名稱數組。如果省略，從父代理繼承所有工具。要將 Skills 預加載到代理的上下文中，請使用 `skills` 字段而不是在此處列出 `'Skill'`                         |
| `disallowedTools`                     | 否  | 要為此代理明確禁止的工具名稱數組。MCP 服務器級別的模式也被接受：`mcp__server` 或 `mcp__server__*` 移除該服務器的每個工具，`mcp__*` 移除任何服務器的每個 MCP 工具 |
| `prompt`                              | 是  | 代理的系統提示                                                                                                   |
| `model`                               | 否  | 此代理的模型覆蓋。接受別名如 `'fable'`、`'opus'`、`'sonnet'`、`'haiku'`、`'inherit'` 或完整模型 ID。如果省略或 `'inherit'`，使用主模型       |
| `mcpServers`                          | 否  | 此代理可用的 MCP 服務器規範                                                                                          |
| `skills`                              | 否  | 要預加載到代理上下文中的 skill 名稱數組                                                                                   |
| `initialPrompt`                       | 否  | 當此代理作為主線程代理運行時，自動提交為第一個用戶轉數                                                                               |
| `maxTurns`                            | 否  | 最大代理轉數（API 往返），然後停止                                                                                       |
| `background`                          | 否  | 當調用時，將此代理作為非阻塞後台任務運行                                                                                      |
| `memory`                              | 否  | 此代理的內存源：`'user'`、`'project'` 或 `'local'`                                                                  |
| `effort`                              | 否  | 此代理的推理努力級別。接受命名級別或整數                                                                                      |
| `permissionMode`                      | 否  | 此代理內工具執行的權限模式。見 [`PermissionMode`](#permissionmode)                                                       |
| `criticalSystemReminder_EXPERIMENTAL` | 否  | 實驗性：添加到系統提示的關鍵提醒                                                                                          |

<h3 id="agentmcpserverspec">
  `AgentMcpServerSpec`
</h3>

指定子代理可用的 MCP 服務器。可以是服務器名稱（字符串，引用父代理 `mcpServers` 配置中的服務器）或內聯服務器配置記錄，將服務器名稱映射到配置。

```typescript theme={null}
type AgentMcpServerSpec = string | Record<string, McpServerConfigForProcessTransport>;
```

其中 `McpServerConfigForProcessTransport` 是 `McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig`。

<h3 id="settingsource">
  `SettingSource`
</h3>

控制 SDK 從哪些基於文件系統的配置源加載設置。

```typescript theme={null}
type SettingSource = "user" | "project" | "local";
```

| 值           | 描述                 | 位置                            |
| :---------- | :----------------- | :---------------------------- |
| `'user'`    | 全局用戶設置             | `~/.claude/settings.json`     |
| `'project'` | 共享項目設置（版本控制）       | `.claude/settings.json`       |
| `'local'`   | 本地項目設置（gitignored） | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Default behavior
</h4>

當 `settingSources` 被省略或 `undefined` 時，`query()` 加載與 Claude Code CLI 相同的文件系統設置：用戶、項目和本地。託管策略設置在所有情況下都會加載；當會話使用組織憑證在 [eligible configuration](/docs/zh-TW/server-managed-settings#platform-availability) 上進行身份驗證時，會獲取服務器管理的設置。見 [What settingSources does not control](/docs/zh-TW/agent-sdk/claude-code-features#what-settingsources-does-not-control) 了解無論此選項如何都會讀取的輸入，以及如何禁用它們。

<h4 id="why-use-settingsources">
  Why use settingSources
</h4>

**禁用文件系統設置：**

```typescript theme={null}
// 不從磁盤加載用戶、項目或本地設置
const result = query({
  prompt: "Analyze this code",
  options: { settingSources: [] }
});
```

**明確加載所有文件系統設置：**

```typescript theme={null}
const result = query({
  prompt: "Analyze this code",
  options: {
    settingSources: ["user", "project", "local"] // 加載所有設置
  }
});
```

**僅加載特定設置源：**

```typescript theme={null}
// 僅加載項目設置，忽略用戶和本地
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ["project"] // 僅 .claude/settings.json
  }
});
```

**測試和 CI 環境：**

```typescript theme={null}
// 通過排除本地設置確保 CI 中的一致行為
const result = query({
  prompt: "Run tests",
  options: {
    settingSources: ["project"], // 僅團隊共享設置
    permissionMode: "bypassPermissions"
  }
});
```

**僅 SDK 應用程序：**

```typescript theme={null}
// 以編程方式定義所有內容。
// 傳遞 [] 以選擇退出文件系統設置源。
const result = query({
  prompt: "Review this PR",
  options: {
    settingSources: [],
    agents: {
      /* ... */
    },
    mcpServers: {
      /* ... */
    },
    allowedTools: ["Read", "Grep", "Glob"]
  }
});
```

**加載 CLAUDE.md 項目指令：**

```typescript theme={null}
// 加載項目設置以包括 CLAUDE.md 文件
const result = query({
  prompt: "Add a new feature following project conventions",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code" // 使用 Claude Code 的系統提示
    },
    settingSources: ["project"], // 從項目目錄加載 CLAUDE.md
    allowedTools: ["Read", "Write", "Edit"]
  }
});
```

<h4 id="settings-precedence">
  Settings precedence
</h4>

加載多個源時，設置按此優先級（最高到最低）合併：

1. 本地設置（`.claude/settings.local.json`）
2. 項目設置（`.claude/settings.json`）
3. 用戶設置（`~/.claude/settings.json`）

編程選項（如 `agents`、`allowedTools` 和 `settings`）覆蓋用戶、項目和本地文件系統設置。託管策略設置優先於編程選項。

<h3 id="permissionmode">
  `PermissionMode`
</h3>

```typescript theme={null}
type PermissionMode =
  | "default" // 標準權限行為
  | "acceptEdits" // 自動接受文件編輯
  | "bypassPermissions" // 繞過權限檢查；明確要求規則仍然提示
  | "plan" // Plan Mode - 無編輯探索
  | "dontAsk" // 不提示權限，如果未預批准則拒絕
  | "auto"; // 使用模型分類器批准或拒絕每個工具調用
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

用於控制工具使用的自定義權限函數類型。

函數是 SDK 替代交互式權限提示：它僅在 [permission evaluation flow](/docs/zh-TW/agent-sdk/permissions#how-permissions-are-evaluated) 解決為提示時調用。已由 `allowedTools` 條目、設置 allow 規則或權限模式（如 `acceptEdits` 或 `bypassPermissions`）批准的工具調用永遠不會調用它。要限制每個工具調用，改用 [`PreToolUse` hook](/docs/zh-TW/agent-sdk/hooks)。

`AskUserQuestion`、標記為 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP tools 和 connector tools [您的組織設置為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools) 即使 allow 規則匹配也會到達函數。在 `dontAsk` 模式下這些調用會被拒絕，無需調用它。

```typescript theme={null}
type CanUseTool = (
  toolName: string,
  input: Record<string, unknown>,
  options: {
    signal: AbortSignal;
    suggestions?: PermissionUpdate[];
    blockedPath?: string;
    decisionReason?: string;
    toolUseID: string;
    agentID?: string;
    requestId: string;
  }
) => Promise<PermissionResult | null>;
```

| 選項               | 類型                                          | 描述                                                                                                                                                                             |
| :--------------- | :------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`         | `AbortSignal`                               | 如果應中止操作則發出信號                                                                                                                                                                   |
| `suggestions`    | [`PermissionUpdate`](#permissionupdate)`[]` | 建議的權限更新，以便用戶不會再次被提示此工具。Bash 提示包括帶有 `localSettings` [destination](#permissionupdatedestination) 的建議，因此在 `updatedPermissions` 中返回它會將規則寫入 `.claude/settings.local.json` 並在會話中持久化。 |
| `blockedPath`    | `string`                                    | 觸發權限請求的文件路徑（如果適用）                                                                                                                                                              |
| `decisionReason` | `string`                                    | 解釋為什麼觸發此權限請求                                                                                                                                                                   |
| `toolUseID`      | `string`                                    | 此特定工具調用在助手消息中的唯一標識符                                                                                                                                                            |
| `agentID`        | `string`                                    | 如果在子代理中運行，子代理的 ID                                                                                                                                                              |
| `requestId`      | `string`                                    | `control_request` 信封的 `request_id`。您的應用程序在 SDK 外發送的 `control_response`（例如簽名的 HTTP POST）必須回顯此值，以便 Claude Code 進程可以將回复與請求匹配                                                      |

回調通常通過返回 [`PermissionResult`](#permissionresult) 來解決請求，SDK 將其寫回其傳輸作為 `control_response`。僅當您的應用程序已通過其自己的通道為此請求發送 `control_response`（回顯 `requestId`）時才返回 `null`；SDK 然後跳過將響應寫入其傳輸。在任何其他情況下返回 `null` 會使工具調用無限期被阻止，因為永遠不會發送 `control_response` 且權限提示不會超時。

`requestId` 選項和 `null` 返回值需要 Claude Code v2.1.199 或更高版本。

<h3 id="permissionresult">
  `PermissionResult`
</h3>

權限檢查的結果。

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

內置工具行為的配置。

```typescript theme={null}
type ToolConfig = {
  askUserQuestion?: {
    previewFormat?: "markdown" | "html";
  };
};
```

| 字段                              | 類型                     | 描述                                                                                                                |
| :------------------------------ | :--------------------- | :---------------------------------------------------------------------------------------------------------------- |
| `askUserQuestion.previewFormat` | `'markdown' \| 'html'` | 選擇進入 [`AskUserQuestion`](/docs/zh-TW/agent-sdk/user-input#question-format) 選項上的 `preview` 字段並設置其內容格式。未設置時，Claude 不發出預覽 |

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

MCP 服務器的配置。

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

SDK 中加載 plugins 的配置。

```typescript theme={null}
type SdkPluginConfig = {
  type: "local";
  path: string;
  skipMcpDiscovery?: boolean;
};
```

| 字段                 | 類型        | 描述                                                                                                                                    |
| :----------------- | :-------- | :------------------------------------------------------------------------------------------------------------------------------------ |
| `type`             | `'local'` | 必須是 `'local'`（目前僅支持本地 plugins）                                                                                                        |
| `path`             | `string`  | 插件目錄的絕對或相對路徑                                                                                                                          |
| `skipMcpDiscovery` | `boolean` | 當為 `true` 時，SDK 從此 plugin 加載 skills、hooks、agents 和 commands，但不讀取其 `.mcp.json` 或 manifest `mcpServers`。當您的應用程序擁有 plugin 的 MCP 連接時設置此項。 |

**示例：**

```typescript theme={null}
plugins: [
  { type: "local", path: "./my-plugin" },
  { type: "local", path: "/absolute/path/to/plugin" }
];
```

有關創建和使用 plugins 的完整信息，見 [Plugins](/docs/zh-TW/agent-sdk/plugins)。

<h2 id="message-types">
  消息類型
</h2>

<h3 id="sdkmessage">
  `SDKMessage`
</h3>

查詢返回的所有可能消息的聯合類型。

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

助手響應消息。

```typescript theme={null}
type SDKAssistantMessage = {
  type: "assistant";
  uuid: UUID;
  session_id: string;
  message: BetaMessage; // 來自 Anthropic SDK
  parent_tool_use_id: string | null;
  error?: SDKAssistantMessageError;
};
```

`message` 字段是來自 Anthropic SDK 的 [`BetaMessage`](https://platform.claude.com/docs/zh-TW/api/messages/create)。它包括 `id`、`content`、`model`、`stop_reason` 和 `usage` 等字段。

`SDKAssistantMessageError` 是以下之一：`'authentication_failed'`、`'oauth_org_not_allowed'`、`'billing_error'`、`'rate_limit'`、`'overloaded'`、`'invalid_request'`、`'model_not_found'`、`'server_error'`、`'max_output_tokens'` 或 `'unknown'`。`'model_not_found'` 表示選定的模型不存在或對您的帳戶或部署不可用。`'overloaded'` 表示 API 返回了 529，因為伺服器已滿載，與 `'rate_limit'` 相對，後者是針對您配額的 429。

<h3 id="sdkusermessage">
  `SDKUserMessage`
</h3>

用戶輸入消息。

```typescript theme={null}
type SDKUserMessage = {
  type: "user";
  uuid?: UUID;
  session_id?: string;
  message: MessageParam; // 來自 Anthropic SDK
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  shouldQuery?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
};
```

將 `shouldQuery` 設置為 `false` 以將消息附加到記錄而不觸發助手轉數。消息被保留並合併到下一個觸發轉數的用戶消息中。使用此方法注入上下文，例如您在帶外運行的命令的輸出，而無需在其上花費模型調用。

在攜帶 `tool_result` 塊的消息上，`tool_use_result` 是工具的結構化輸出物件，而不是發送給模型的文本。其形狀取決於匹配 `tool_use` 塊命名的工具，因此該字段的類型為 `unknown`；內建形狀列在[工具輸出類型](#tool-output-types)下。

對於 `Agent` 工具，`tool_use_result` 是 [`AgentOutput`](#agent-2)。在 `completed` 結果上，`content` 保存子代理的報告，不包含 Claude Code 附加到 `tool_result` 文本的代理 ID 和使用情況尾部，因此應從 `tool_use_result` 呈現，而不是解析該文本。

<h3 id="sdkusermessagereplay">
  `SDKUserMessageReplay`
</h3>

帶有必需 UUID 的重放用戶消息。

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

從會話外部注入的用戶轉數，其 [`origin`](#sdkmessageorigin) 類型為 `peer` 或 `channel`，無論是在活躍轉數期間傳遞還是在會話閒置時啟動新轉數，都會作為重放到達流。在 v2.1.207 之前，在會話閒置時傳遞的注入轉數在流上不產生任何消息，僅在您重新讀取記錄時出現。

<h3 id="sdkresultmessage">
  `SDKResultMessage`
</h3>

最終結果消息。

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
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      structured_output?: unknown;
      deferred_tool_use?: { id: string; name: string; input: Record<string, unknown> };
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
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
      errors: string[];
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      origin?: SDKMessageOrigin;
    };
```

結果上的多個字段除了 `subtype` 之外還攜帶診斷詳細信息：

* `api_error_status`：終止對話的 API 錯誤的 HTTP 狀態碼。當轉數在沒有 API 錯誤的情況下結束時，不存在或為 `null`。
* `ttft_ms`：首個令牌的時間（毫秒），在第一個完整助手消息到達時測量。僅在成功分支上出現。
* `ttft_stream_ms`：直到第一個 `message_start` 流事件的時間（毫秒），當響應流打開時。低於 `ttft_ms`；兩者之間的差距是流式傳輸第一條消息所花費的時間。僅在成功分支上出現。
* `terminal_reason`：循環結束的原因。為 `"completed"`、`"max_turns"`、`"tool_deferred"`、`"aborted_streaming"`、`"aborted_tools"`、`"hook_stopped"`、`"stop_hook_prevented"`、`"background_requested"`、`"blocking_limit"`、`"rapid_refill_breaker"`、`"prompt_too_long"`、`"image_error"`、`"model_error"`、`"api_error"`、`"malformed_tool_use_exhausted"`、`"budget_exhausted"`、`"structured_output_retry_exhausted"`、`"tool_deferred_unavailable"` 或 `"turn_setup_failed"` 之一。
* `fast_mode_state`：為 `"on"`、`"off"` 或 `"cooldown"` 之一。

`origin` 字段轉發觸發此結果的用戶消息的 [`SDKMessageOrigin`](#sdkmessageorigin)。當後台任務完成且 SDK 注入合成後續轉數時，生成的 `SDKResultMessage` 攜帶 `origin: { kind: "task-notification" }`。檢查此字段以區分回答您的提示的結果與為後台任務後續發出的結果，以便您可以路由或抑制後者。對於在任何用戶轉數之前發出的結果（例如啟動錯誤），該字段不存在。

當 `PreToolUse` hook 返回 `permissionDecision: "defer"` 時，結果具有 `stop_reason: "tool_deferred"` 和 `deferred_tool_use` 攜帶待處理工具的 `id`、`name` 和 `input`。讀取此字段以在您自己的 UI 中顯示請求，然後使用相同的 `session_id` 恢復以繼續。有關完整往返，請參閱[稍後延遲工具調用](/docs/zh-TW/hooks#defer-a-tool-call-for-later)。

<h3 id="sdksystemmessage">
  `SDKSystemMessage`
</h3>

系統初始化消息。

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
  }[];
  model: string;
  permissionMode: PermissionMode;
  slash_commands: string[];
  output_style: string;
  skills: string[];
  plugins: { name: string; path: string }[];
  capabilities?: string[];
};
```

`capabilities` 陣列命名此 CLI 實現的協議行為，因此您可以進行功能檢測而不是比較 `claude_code_version` 字符串。這是一個開放集合：忽略您不認識的值，並檢查您依賴其行為的特定功能。該字段需要 Claude Code v2.1.205 或更高版本，在較早的 CLI 上不存在。

| 功能                     | 含義                                                                                                                 |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `interrupt_receipt_v1` | [`interrupt()`](#query-object) 使用命名存活中斷的排隊消息的 [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) 收據進行解析 |

<h3 id="sdkpartialassistantmessage">
  `SDKPartialAssistantMessage`
</h3>

流式部分消息（僅當 `includePartialMessages` 為 true 時）。`parent_tool_use_id` 字段始終為 `null`：流事件僅針對主會話發出。對於子代理歸因，使用完整消息（攜帶 `parent_tool_use_id`），或啟用 [`forwardSubagentText`](#options) 以接收子代理文本和思考作為完整消息。

```typescript theme={null}
type SDKPartialAssistantMessage = {
  type: "stream_event";
  event: BetaRawMessageStreamEvent; // 來自 Anthropic SDK
  parent_tool_use_id: string | null;
  uuid: UUID;
  session_id: string;
  ttft_ms?: number; // 首個令牌的時間（毫秒），僅在 message_start 事件上出現
};
```

<h3 id="sdkcompactboundarymessage">
  `SDKCompactBoundaryMessage`
</h3>

指示對話壓縮邊界的消息。

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

由循環發出的通用文本橫幅。攜帶非錯誤狀態行、hook 反饋（例如 `UserPromptSubmit` hook 的阻止原因）和命令輸出。將 `content` 呈現為給定 `level` 的純文本。

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

在優雅的 worker 拆卸時發出，以便遠程客戶端可以顯示 worker 消失的原因，而不是等待心跳超時。`reason` 是由主機 CLI 設置的短 snake\_case 字符串，例如 `"host_exit"` 或 `"remote_control_disabled"`。僅在實時流式傳輸時對此採取行動。恢復的會話會重放此消息的過去實例，因此在這種情況下忽略它們。

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

插件安裝進度事件。當設置 [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/zh-TW/env-vars) 時發出，以便您的 Agent SDK 應用程式可以在第一個轉數之前追蹤市場插件安裝。`started` 和 `completed` 狀態括起整體安裝。`installed` 和 `failed` 狀態報告單個市場並包括 `name`。

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

當權限系統自動拒絕工具調用而不進行互動式提示時發出的流事件。使用它在發生時在您的 UI 中呈現拒絕，而不是僅觀察隨後的 `is_error` 工具結果。互動式詢問路徑通過 [`canUseTool`](#canusetool) 回調單獨到達您的應用程式。由 `PreToolUse` hook 發出的拒絕不會通過此事件報告。

此事件需要 Claude Code v2.1.136 或更高版本。

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

| 字段                     | 類型       | 描述                                                            |
| ---------------------- | -------- | ------------------------------------------------------------- |
| `tool_name`            | `string` | 被拒絕的工具的名稱                                                     |
| `tool_use_id`          | `string` | 此拒絕回答的 `tool_use` 塊的 ID                                       |
| `agent_id`             | `string` | 當被拒絕的調用源自子代理內部時的子代理 ID。鏡像 `can_use_tool` 上的字段以進行主機端路由         |
| `decision_reason_type` | `string` | 決定組件的判別器，例如 `"rule"`、`"mode"`、`"classifier"` 或 `"asyncAgent"` |
| `decision_reason`      | `string` | 來自決定組件的人類可讀原因（如果可用）                                           |
| `message`              | `string` | 在 `tool_result` 中返回給模型的拒絕消息                                   |

<h3 id="sdkpermissiondenial">
  `SDKPermissionDenial`
</h3>

有關被拒絕的工具使用的信息。

```typescript theme={null}
type SDKPermissionDenial = {
  tool_name: string;
  tool_use_id: string;
  tool_input: Record<string, unknown>;
};
```

<h3 id="sdkmessageorigin">
  `SDKMessageOrigin`
</h3>

用戶角色消息的來源。這在 [`SDKUserMessage`](#sdkusermessage) 上顯示為 `origin`，並轉發到相應的 [`SDKResultMessage`](#sdkresultmessage)，以便您可以判斷給定轉數的觸發因素。

```typescript theme={null}
type SDKMessageOrigin =
  | { kind: "human" }
  | { kind: "channel"; server: string }
  | {
      kind: "peer";
      from: string;
      name?: string;
      senderTaskId?: string;
      body?: string;
    }
  | { kind: "task-notification" }
  | { kind: "coordinator" }
  | { kind: "auto-continuation" };
```

| `kind`              | 含義                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `human`             | 來自最終用戶的直接輸入。在用戶消息上，缺少的 `origin` 也表示人類輸入。                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `channel`           | 在[頻道](/docs/zh-TW/channels)上到達的消息。`server` 是源 MCP 伺服器名稱。                                                                                                                                                                                                                                                                                                                                                                                                           |
| `peer`              | 來自另一個代理的消息。對於通過 `SendMessage` 發送到 `main` 的進程內[隊友](/docs/zh-TW/agent-teams)，`from` 是隊友的名稱，`senderTaskId` 是其任務 ID。對於跨會話對等體（例如另一個本地 Claude Code 進程），`from` 是發送者地址，`senderTaskId` 不存在。}`name` 和 `body` 需要 Claude Code v2.1.205 或更高版本。`name` 是發送者的顯示名稱，由 Claude Code 規範化：它去除 Unicode 控制、格式、代理和行或段落分隔符代碼點，然後修剪結果並將其限制為 64 個代碼點，並帶有省略號。`body` 是去除對等信封的已解碼消息正文，與模型看到的內容完全相同。對於隊友消息，`body` 始終存在；對於跨會話對等體，僅當轉數恰好是由 Claude Code 形成的一個對等信封時才存在。呈現 `name` 和 `body` 而不是重新解析消息文本。 |
| `task-notification` | 後台任務完成後注入的合成轉數。請參閱 [`SDKTaskNotificationMessage`](#sdktasknotificationmessage)。                                                                                                                                                                                                                                                                                                                                                                               |
| `coordinator`       | 來自[代理團隊](/docs/zh-TW/agent-teams)中的團隊協調員的消息。                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `auto-continuation` | 當會話在沒有新用戶輸入的情況下繼續時注入的合成轉數，例如觸發後續提示的命令結果。                                                                                                                                                                                                                                                                                                                                                                                                                      |

<h2 id="hook-types">
  Hook 類型
</h2>

有關使用 hooks 的綜合指南，包括示例和常見模式，見 [Hooks 指南](/docs/zh-TW/agent-sdk/hooks)。

<h3 id="hookevent">
  `HookEvent`
</h3>

可用的 hook 事件。

```typescript theme={null}
type HookEvent =
  | "PreToolUse"
  | "PostToolUse"
  | "PostToolUseFailure"
  | "PostToolBatch"
  | "Notification"
  | "UserPromptSubmit"
  | "SessionStart"
  | "SessionEnd"
  | "Stop"
  | "SubagentStart"
  | "SubagentStop"
  | "PreCompact"
  | "PermissionRequest"
  | "Setup"
  | "TeammateIdle"
  | "TaskCompleted"
  | "ConfigChange"
  | "WorktreeCreate"
  | "WorktreeRemove"
  | "MessageDisplay";
```

<h3 id="hookcallback">
  `HookCallback`
</h3>

Hook 回調函數類型。

```typescript theme={null}
type HookCallback = (
  input: HookInput, // 所有 hook 輸入類型的聯合
  toolUseID: string | undefined,
  options: { signal: AbortSignal }
) => Promise<HookJSONOutput>;
```

<h3 id="hookcallbackmatcher">
  `HookCallbackMatcher`
</h3>

帶有可選匹配器的 Hook 配置。

```typescript theme={null}
interface HookCallbackMatcher {
  matcher?: string;
  hooks: HookCallback[];
  timeout?: number; // 此匹配器中所有 hooks 的超時時間（秒）
}
```

<h3 id="hookinput">
  `HookInput`
</h3>

所有 hook 輸入類型的聯合類型。

```typescript theme={null}
type HookInput =
  | PreToolUseHookInput
  | PostToolUseHookInput
  | PostToolUseFailureHookInput
  | PostToolBatchHookInput
  | NotificationHookInput
  | UserPromptSubmitHookInput
  | SessionStartHookInput
  | SessionEndHookInput
  | StopHookInput
  | SubagentStartHookInput
  | SubagentStopHookInput
  | PreCompactHookInput
  | PermissionRequestHookInput
  | SetupHookInput
  | TeammateIdleHookInput
  | TaskCompletedHookInput
  | ConfigChangeHookInput
  | WorktreeCreateHookInput
  | WorktreeRemoveHookInput
  | MessageDisplayHookInput;
```

<h3 id="basehookinput">
  `BaseHookInput`
</h3>

所有 hook 輸入類型擴展的基本介面。

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

`prompt_id` 欄位是一個 UUID，用於識別目前正在處理的使用者提示。它與 [OpenTelemetry 事件上的 `prompt.id` 屬性](/docs/zh-TW/monitoring-usage#event-correlation-attributes)相符，在第一個使用者輸入之前不存在。需要 Claude Code v2.1.196 或更新版本。

<h4 id="pretoolusehookinput">
  `PreToolUseHookInput`
</h4>

```typescript theme={null}
type PreToolUseHookInput = BaseHookInput & {
  hook_event_name: "PreToolUse";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
};
```

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
};
```

<h4 id="posttoolbatchhookinput">
  `PostToolBatchHookInput`
</h4>

在批次中的每個工具呼叫都已解決後、下一個模型請求之前觸發一次。`tool_response` 攜帶序列化的 `tool_result` 內容，模型會看到；其形狀與 `PostToolUseHookInput` 的結構化 `Output` 物件不同。

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
};
```

<h4 id="sessionstarthookinput">
  `SessionStartHookInput`
</h4>

```typescript theme={null}
type SessionStartHookInput = BaseHookInput & {
  hook_event_name: "SessionStart";
  source: "startup" | "resume" | "clear" | "compact";
  agent_type?: string;
  model?: string;
};
```

<h4 id="sessionendhookinput">
  `SessionEndHookInput`
</h4>

```typescript theme={null}
type SessionEndHookInput = BaseHookInput & {
  hook_event_name: "SessionEnd";
  reason: ExitReason; // EXIT_REASONS 陣列中的字串
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

<h4 id="permissionrequesthookinput">
  `PermissionRequestHookInput`
</h4>

```typescript theme={null}
type PermissionRequestHookInput = BaseHookInput & {
  hook_event_name: "PermissionRequest";
  tool_name: string;
  tool_input: unknown;
  permission_suggestions?: PermissionUpdate[];
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
  /** @deprecated 自 v2.1.178 起已棄用。攜帶工作階段衍生的團隊名稱；將被移除。 */
  team_name: string;
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
  /** @deprecated 自 v2.1.178 起已棄用。攜帶工作階段衍生的團隊名稱；將被移除。 */
  team_name?: string;
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

Hook 返回值。

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
      }
    | {
        hookEventName: "SessionStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "Setup";
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolUse";
        additionalContext?: string;
        updatedToolOutput?: unknown;
        /** @deprecated 使用 `updatedToolOutput`，適用於所有工具。 */
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
      };
};
```

<h2 id="tool-input-types">
  工具輸入類型
</h2>

所有內置 Claude Code 工具的輸入架構文檔。這些類型從 `@anthropic-ai/claude-agent-sdk` 導出，可用於類型安全的工具交互。

<h3 id="toolinputschemas">
  `ToolInputSchemas`
</h3>

所有工具輸入類型的聯合，從 `@anthropic-ai/claude-agent-sdk` 導出。

```typescript theme={null}
type ToolInputSchemas =
  | AgentInput
  | AskUserQuestionInput
  | BashInput
  | TaskOutputInput
  | EnterWorktreeInput
  | ExitPlanModeInput
  | FileEditInput
  | FileReadInput
  | FileWriteInput
  | GlobInput
  | GrepInput
  | ListMcpResourcesInput
  | McpInput
  | MonitorInput
  | NotebookEditInput
  | ReadMcpResourceInput
  | SubscribeMcpResourceInput
  | SubscribePollingInput
  | TaskCreateInput
  | TaskGetInput
  | TaskListInput
  | TaskStopInput
  | TaskUpdateInput
  | TodoWriteInput
  | UnsubscribeMcpResourceInput
  | UnsubscribePollingInput
  | WebFetchInput
  | WebSearchInput
  | WorkflowInput;
```

<h3 id="agent">
  Agent
</h3>

**工具名稱：** `Agent`（之前是 `Task`，仍然接受作為別名）

```typescript theme={null}
type AgentInput = {
  description: string;
  prompt: string;
  subagent_type?: string;
  model?: "sonnet" | "opus" | "haiku" | "fable";
  run_in_background?: boolean;
  name?: string;
  mode?: "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan";
  isolation?: "worktree";
};
```

啟動新代理以自主處理複雜的多步驟任務。

<h3 id="askuserquestion">
  AskUserQuestion
</h3>

**工具名稱：** `AskUserQuestion`

```typescript theme={null}
type AskUserQuestionInput = {
  questions: Array<{
    question: string;
    header: string;
    options: Array<{ label: string; description: string; preview?: string }>;
    multiSelect: boolean;
  }>;
};
```

在執行期間向用戶提出澄清問題。見 [處理批准和用戶輸入](/docs/zh-TW/agent-sdk/user-input#handle-clarifying-questions) 了解使用詳情。

<h3 id="bash">
  Bash
</h3>

**工具名稱：** `Bash`

```typescript theme={null}
type BashInput = {
  command: string;
  timeout?: number; // milliseconds, max 600000; higher values are clamped to the max
  description?: string;
  run_in_background?: boolean;
  dangerouslyDisableSandbox?: boolean;
};
```

在持久 shell 會話中執行 bash 命令，具有可選的超時和後台執行。

<h3 id="monitor">
  Monitor
</h3>

**工具名稱：** `Monitor`

```typescript theme={null}
type MonitorInput = {
  command?: string;
  ws?: {
    url: string;
    protocols?: string[];
  };
  description: string;
  timeout_ms?: number;
  persistent?: boolean;
};
```

運行後台來源並將每個事件傳遞給 Claude，以便它可以在不輪詢的情況下做出反應：`command` 運行腳本並每個 stdout 行發出一個事件，`ws` 打開 WebSocket 並每個文本幀發出一個事件。提供 `command` 或 `ws` 中的恰好一個。`ws` 來源需要 Claude Code v2.1.195 或更高版本。

為會話長度的監視（如日誌尾部）設置 `persistent: true`。Monitor 運行命令時，遵循與 Bash 相同的權限規則；WebSocket 監視會單獨提示批准。見 [Monitor 工具參考](/docs/zh-TW/tools-reference#monitor-tool) 了解行為和提供商可用性。

<h3 id="taskoutput">
  TaskOutput
</h3>

**工具名稱：** `TaskOutput`

```typescript theme={null}
type TaskOutputInput = {
  task_id: string;
  block: boolean;
  timeout: number;
};
```

從運行或已完成的後台任務檢索輸出。

<h3 id="edit">
  Edit
</h3>

**工具名稱：** `Edit`

```typescript theme={null}
type FileEditInput = {
  file_path: string;
  old_string: string;
  new_string: string;
  replace_all?: boolean;
};
```

在文件中執行精確字符串替換。

<h3 id="read">
  Read
</h3>

**工具名稱：** `Read`

```typescript theme={null}
type FileReadInput = {
  file_path: string;
  offset?: number;
  limit?: number;
  pages?: string;
};
```

從本地文件系統讀取文件，包括文本、圖像、PDF 和 Jupyter 筆記本。對 PDF 頁面範圍使用 `pages`（例如，`"1-5"`）。

<h3 id="write">
  Write
</h3>

**工具名稱：** `Write`

```typescript theme={null}
type FileWriteInput = {
  file_path: string;
  content: string;
};
```

將文件寫入本地文件系統，如果存在則覆蓋。

<h3 id="glob">
  Glob
</h3>

**工具名稱：** `Glob`

```typescript theme={null}
type GlobInput = {
  pattern: string;
  path?: string;
};
```

快速文件模式匹配，適用於任何代碼庫大小。

<h3 id="grep">
  Grep
</h3>

**工具名稱：** `Grep`

```typescript theme={null}
type GrepInput = {
  pattern: string;
  path?: string;
  glob?: string;
  type?: string;
  output_mode?: "content" | "files_with_matches" | "count";
  "-i"?: boolean;
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

基於 ripgrep 的強大搜索工具，支持正則表達式。

<h3 id="taskstop">
  TaskStop
</h3>

**工具名稱：** `TaskStop`

```typescript theme={null}
type TaskStopInput = {
  task_id?: string;
  shell_id?: string; // 已棄用：使用 task_id
};
```

按 ID 停止運行的後台任務或 shell。自 v2.1.198 起，`task_id` 也接受代理團隊隊友或按代理 ID 或名稱的命名後台代理。

<h3 id="notebookedit">
  NotebookEdit
</h3>

**工具名稱：** `NotebookEdit`

```typescript theme={null}
type NotebookEditInput = {
  notebook_path: string;
  cell_id?: string;
  new_source: string;
  cell_type?: "code" | "markdown";
  edit_mode?: "replace" | "insert" | "delete";
};
```

編輯 Jupyter 筆記本文件中的單元格。

<h3 id="webfetch">
  WebFetch
</h3>

**工具名稱：** `WebFetch`

```typescript theme={null}
type WebFetchInput = {
  url: string;
  prompt: string;
};
```

從 URL 獲取內容並使用 AI 模型處理它。

<h3 id="websearch">
  WebSearch
</h3>

**工具名稱：** `WebSearch`

```typescript theme={null}
type WebSearchInput = {
  query: string;
  allowed_domains?: string[];
  blocked_domains?: string[];
};
```

搜索網絡並返回格式化結果。

<h3 id="workflow">
  Workflow
</h3>

**工具名稱：** `Workflow`

```typescript theme={null}
type WorkflowInput = {
  script?: string;
  name?: string;
  scriptPath?: string;
  args?: unknown;
  resumeFromRunId?: string;
};
```

運行 [動態工作流](/docs/zh-TW/workflows)：一個在後台協調許多子代理並返回一個統一結果的腳本。`Workflow` 工具在 Agent SDK v0.3.149 及更高版本中可用。至少需要 `script`、`name` 或 `scriptPath` 之一。

| 字段                | 類型        | 描述                                                                                                                                                                   |
| ----------------- | --------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `script`          | `string`  | 內聯工作流腳本。必須以 `export const meta = { name, description }` 作為字面量開始，然後是使用 `agent()`、`parallel()`、`pipeline()` 和 `phase()` 的腳本主體。`meta` 中的可選 `phases` 陣列在進度檢視中將代理分組到命名階段下 |
| `name`            | `string`  | 內置工作流的名稱或保存在 `.claude/workflows/` 中的工作流名稱。解析為腳本                                                                                                                      |
| `scriptPath`      | `string`  | 磁盤上工作流腳本文件的路徑。優先於 `script` 和 `name`。每次調用都會持久化其腳本並在結果中返回路徑，因此您可以編輯該文件並使用相同的 `scriptPath` 重新調用以進行迭代                                                                    |
| `args`            | `unknown` | 輸入值，作為全局 `args` 暴露給腳本，用於參數化的命名工作流，例如研究問題或文件路徑列表。將數組和對象作為實際 JSON 值傳遞，而不是作為 JSON 編碼的字符串                                                                                |
| `resumeFromRunId` | `string`  | 先前 `Workflow` 調用的運行 ID 以恢復。具有未更改輸入的已完成 `agent()` 調用返回緩存結果；只有更改或新調用才會實時運行。僅限同一會話                                                                                      |

<h3 id="todowrite">
  TodoWrite
</h3>

**工具名稱：** `TodoWrite`

```typescript theme={null}
type TodoWriteInput = {
  todos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
};
```

創建和管理結構化任務列表以跟蹤進度。

<Note>
  自 TypeScript Agent SDK 0.3.142 起，`TodoWrite` 預設為禁用。改用 `TaskCreate`、`TaskGet`、`TaskUpdate` 和 `TaskList`。見 [遷移到 Task 工具](/docs/zh-TW/agent-sdk/todo-tracking#migrate-to-task-tools) 更新您的監視代碼，或設置 `CLAUDE_CODE_ENABLE_TASKS=0` 以恢復為 `TodoWrite`。
</Note>

<h3 id="taskcreate">
  TaskCreate
</h3>

**工具名稱：** `TaskCreate`

```typescript theme={null}
type TaskCreateInput = {
  subject: string;
  description: string;
  activeForm?: string;
  metadata?: Record<string, unknown>;
};
```

創建單個任務並返回其分配的 ID。

<h3 id="taskupdate">
  TaskUpdate
</h3>

**工具名稱：** `TaskUpdate`

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

按 ID 修補一個任務。將 `status` 設置為 `"deleted"` 以移除它。

<h3 id="taskget">
  TaskGet
</h3>

**工具名稱：** `TaskGet`

```typescript theme={null}
type TaskGetInput = {
  taskId: string;
};
```

返回一個任務的完整詳情，或在找不到 ID 時返回 `null`。

<h3 id="tasklist">
  TaskList
</h3>

**工具名稱：** `TaskList`

```typescript theme={null}
type TaskListInput = {};
```

返回當前列表中所有任務的快照。

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**工具名稱：** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeInput = {
  /** 已棄用：不再使用。 */
  allowedPrompts?: Array<{
    tool: "Bash";
    prompt: string;
  }>;
};
```

退出規劃模式。`allowedPrompts` 字段已棄用且被忽略；Claude Code 仍然接受它，以便現有調用者和記錄驗證。在 v2.1.205 之前，它請求基於提示的 Bash 權限以實施計劃。

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**工具名稱：** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesInput = {
  server?: string;
};
```

列出來自連接服務器的可用 MCP 資源。

<h3 id="readmcpresource">
  ReadMcpResource
</h3>

**工具名稱：** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceInput = {
  server: string;
  uri: string;
};
```

從服務器讀取特定的 MCP 資源。

<h3 id="enterworktree">
  EnterWorktree
</h3>

**工具名稱：** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeInput = {
  name?: string;
  path?: string;
};
```

創建並進入臨時 git worktree 以進行隔離工作。傳遞 `path` 以切換到現有 worktree，而不是創建新的。在首次進入時，目標必須是當前存儲庫的已註冊 worktree，或在多存儲庫工作區中，必須是嵌套在其中的存儲庫的已註冊 worktree；從 worktree 會話內進入時，必須在會話存儲庫的 `.claude/worktrees/` 下。`name` 和 `path` 互斥。

<h2 id="tool-output-types">
  工具輸出類型
</h2>

所有內置 Claude Code 工具的輸出架構文檔。這些類型從 `@anthropic-ai/claude-agent-sdk` 導出，代表每個工具返回的實際響應數據。

<h3 id="tooloutputschemas">
  `ToolOutputSchemas`
</h3>

所有工具輸出類型的聯合。

```typescript theme={null}
type ToolOutputSchemas =
  | AgentOutput
  | AskUserQuestionOutput
  | BashOutput
  | EnterWorktreeOutput
  | ExitPlanModeOutput
  | FileEditOutput
  | FileReadOutput
  | FileWriteOutput
  | GlobOutput
  | GrepOutput
  | ListMcpResourcesOutput
  | MonitorOutput
  | NotebookEditOutput
  | ReadMcpResourceOutput
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

**工具名稱：** `Agent`（之前是 `Task`，仍然接受作為別名）

```typescript theme={null}
type AgentOutput =
  | {
      status: "completed";
      agentId: string;
      agentType?: string;
      content: Array<{ type: "text"; text: string; citations?: unknown[] | null }>;
      resolvedModel?: string;
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

返回子代理的結果。在 `status` 字段上區分：`"completed"` 用於已完成的任務，`"async_launched"` 用於後台任務，以及 `"remote_launched"` 用於 Claude Code 分派到遠端雲端工作階段的任務，其中 `sessionUrl` 連結到該工作階段，`taskId` 識別它。

`completed` 和 `async_launched` 變體上的 `resolvedModel` 字段命名子代理實際運行的模型，當應用 [`availableModels`](/docs/zh-TW/model-config#restrict-model-selection) 或其他覆蓋時，該模型可能與請求的 `model` 輸入不同。此字段需要 Claude Code v2.1.174 或更高版本。

在 `completed` 變體上，當子代理在隔離的 git worktree 中運行時，`worktreePath` 被設置，當 Claude Code 創建它時，`worktreeBranch` 命名該 worktree 的分支。`usage.service_tier` 攜帶 API 為子代理的請求報告的服務層字符串。

在 v2.1.207 之前，發佈的類型更窄。它省略了 `worktreePath`、`worktreeBranch`、`citations`、`toolStats.frameCount` 以及 `inference_geo`、`speed` 和 `iterations` 使用字段，並將 `service_tier` 類型化為 `"standard" | "priority" | "batch"`。類型標記為可選的字段可能在由較早版本記錄的結果中不存在。

<h3 id="askuserquestion-2">
  AskUserQuestion
</h3>

**工具名稱：** `AskUserQuestion`

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
};
```

返回提出的問題和用戶的答案。當用戶輸入自由形式的回覆而不是回答結構化問題時，`response` 被設置；當存在時，Claude 會收到「用戶回應：…」而不是每個問題的答案列表。

<h3 id="bash-2">
  Bash
</h3>

**工具名稱：** `Bash`

```typescript theme={null}
type BashOutput = {
  stdout: string;
  stderr: string;
  rawOutputPath?: string;
  interrupted: boolean;
  isImage?: boolean;
  backgroundTaskId?: string;
  backgroundedByUser?: boolean;
  dangerouslyDisableSandbox?: boolean;
  returnCodeInterpretation?: string;
  structuredContent?: unknown[];
  persistedOutputPath?: string;
  persistedOutputSize?: number;
};
```

返回命令輸出，stdout/stderr 分開。後台命令包括 `backgroundTaskId`。

<h3 id="monitor-2">
  Monitor
</h3>

**工具名稱：** `Monitor`

```typescript theme={null}
type MonitorOutput = {
  taskId: string;
  timeoutMs: number;
  persistent?: boolean;
};
```

返回運行監視器的後台任務 ID。使用此 ID 與 `TaskStop` 一起提前取消監視。

<h3 id="edit-2">
  Edit
</h3>

**工具名稱：** `Edit`

```typescript theme={null}
type FileEditOutput = {
  filePath: string;
  oldString: string;
  newString: string;
  originalFile: string;
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
  };
};
```

返回編輯操作的結構化差異。

<h3 id="read-2">
  Read
</h3>

**工具名稱：** `Read`

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
    };
```

返回適合文件類型的文件內容。在 `type` 字段上區分。

<h3 id="write-2">
  Write
</h3>

**工具名稱：** `Write`

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
  };
};
```

返回寫入結果，包含結構化差異信息。

<h3 id="glob-2">
  Glob
</h3>

**工具名稱：** `Glob`

```typescript theme={null}
type GlobOutput = {
  durationMs: number;
  numFiles: number;
  filenames: string[];
  truncated: boolean;
};
```

返回與 Glob 模式匹配的文件路徑，按修改時間排序。

<h3 id="grep-2">
  Grep
</h3>

**工具名稱：** `Grep`

```typescript theme={null}
type GrepOutput = {
  mode?: "content" | "files_with_matches" | "count";
  numFiles: number;
  filenames: string[];
  content?: string;
  numLines?: number;
  numMatches?: number;
  appliedLimit?: number;
  appliedOffset?: number;
};
```

返回搜索結果。形狀因 `mode` 而異：文件列表、帶匹配的內容或匹配計數。

<h3 id="taskstop-2">
  TaskStop
</h3>

**工具名稱：** `TaskStop`

```typescript theme={null}
type TaskStopOutput = {
  message: string;
  task_id: string;
  task_type: string;
  command?: string;
};
```

停止後台任務後返回確認。

<h3 id="notebookedit-2">
  NotebookEdit
</h3>

**工具名稱：** `NotebookEdit`

```typescript theme={null}
type NotebookEditOutput = {
  new_source: string;
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

返回筆記本編輯的結果，包含原始和更新的文件內容。

<h3 id="webfetch-2">
  WebFetch
</h3>

**工具名稱：** `WebFetch`

```typescript theme={null}
type WebFetchOutput = {
  bytes: number;
  code: number;
  codeText: string;
  result: string;
  durationMs: number;
  url: string;
};
```

返回獲取的內容，包含 HTTP 狀態和元數據。

<h3 id="websearch-2">
  WebSearch
</h3>

**工具名稱：** `WebSearch`

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
};
```

返回來自網絡的搜索結果。

<h3 id="workflow-2">
  Workflow
</h3>

**工具名稱：** `Workflow`

```typescript theme={null}
type WorkflowOutput = {
  status: "async_launched";
  taskId: string;
  runId?: string;
  summary?: string;
  transcriptDir?: string;
  scriptPath?: string;
  error?: string;
};
```

在工具接受調用後立即返回。最終結果稍後作為任務完成到達。在將運行視為已啟動之前檢查 `error`：失敗語法檢查的腳本返回 `status: "async_launched"` 並設置 `error`，並且永遠不會運行。

| 字段              | 類型                 | 描述                                                   |
| --------------- | ------------------ | ---------------------------------------------------- |
| `status`        | `"async_launched"` | 工具接受了調用。這是該字段採用的唯一值                                  |
| `taskId`        | `string`           | 運行的後台任務標識符                                           |
| `runId`         | `string`           | 工作流運行標識符，用於在稍後調用時作為 `resumeFromRunId` 傳遞             |
| `summary`       | `string`           | 工作流功能的單行描述                                           |
| `transcriptDir` | `string`           | 執行期間寫入子代理轉錄的目錄                                       |
| `scriptPath`    | `string`           | 此運行的持久化工作流腳本的路徑。編輯它並作為 `scriptPath` 傳回以重新運行而無需重新發送腳本 |
| `error`         | `string`           | 當腳本失敗其語法檢查時設置。存在時，儘管 `async_launched` 狀態，運行未啟動       |

<h3 id="todowrite-2">
  TodoWrite
</h3>

**工具名稱：** `TodoWrite`

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

返回之前和更新的任務列表。

<Note>
  自 TypeScript Agent SDK 0.3.142 起，`TodoWrite` 預設為禁用。改用 `TaskCreate`、`TaskGet`、`TaskUpdate` 和 `TaskList`。請參閱[遷移到 Task 工具](/docs/zh-TW/agent-sdk/todo-tracking#migrate-to-task-tools)以更新您的監視代碼，或設置 `CLAUDE_CODE_ENABLE_TASKS=0` 以恢復為 `TodoWrite`。
</Note>

<h3 id="taskcreate-2">
  TaskCreate
</h3>

**工具名稱：** `TaskCreate`

```typescript theme={null}
type TaskCreateOutput = {
  task: {
    id: string;
    subject: string;
  };
};
```

返回創建的任務及其分配的 ID。

<h3 id="taskupdate-2">
  TaskUpdate
</h3>

**工具名稱：** `TaskUpdate`

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

返回更新結果，包括哪些字段已更改。

<h3 id="taskget-2">
  TaskGet
</h3>

**工具名稱：** `TaskGet`

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

返回完整的任務記錄，或在找不到 ID 時返回 `null`。

<h3 id="tasklist-2">
  TaskList
</h3>

**工具名稱：** `TaskList`

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

返回當前列表中所有任務的快照。

<h3 id="exitplanmode-2">
  ExitPlanMode
</h3>

**工具名稱：** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeOutput = {
  plan: string | null;
  isAgent: boolean;
  filePath?: string;
  hasTaskTool?: boolean;
  awaitingLeaderApproval?: boolean;
  requestId?: string;
};
```

返回退出 Plan Mode 後的計劃狀態。

<h3 id="listmcpresources-2">
  ListMcpResources
</h3>

**工具名稱：** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesOutput = Array<{
  uri: string;
  name: string;
  mimeType?: string;
  description?: string;
  server: string;
}>;
```

返回可用 MCP 資源的數組。

<h3 id="readmcpresource-2">
  ReadMcpResource
</h3>

**工具名稱：** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceOutput = {
  contents: Array<{
    uri: string;
    mimeType?: string;
    text?: string;
  }>;
};
```

返回請求的 MCP 資源的內容。

<h3 id="enterworktree-2">
  EnterWorktree
</h3>

**工具名稱：** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeOutput = {
  worktreePath: string;
  worktreeBranch?: string;
  message: string;
};
```

返回有關 git worktree 的信息。

<h2 id="permission-types">
  權限類型
</h2>

<h3 id="permissionupdate">
  `PermissionUpdate`
</h3>

用於更新權限的操作。

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
  | "userSettings" // 全域使用者設定
  | "projectSettings" // 每個目錄的專案設定
  | "localSettings" // 本機專案設定
  | "session" // 僅限目前工作階段
  | "cliArg"; // CLI 引數
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
  其他類型
</h2>

<h3 id="apikeysource">
  `ApiKeySource`
</h3>

```typescript theme={null}
type ApiKeySource = "user" | "project" | "org" | "temporary" | "oauth";
```

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

可通過 `betas` 選項啟用的可用測試功能。見 [Beta 標頭](https://platform.claude.com/docs/zh-TW/api/beta-headers) 了解更多信息。

```typescript theme={null}
type SdkBeta = "context-1m-2025-08-07";
```

<Warning>
  `context-1m-2025-08-07` beta 自 2026 年 4 月 30 日起已停用。使用 Claude Sonnet 4.5 或 Sonnet 4 傳遞此值無效，超過標準 200k 令牌上下文窗口的請求返回錯誤。要使用 1M 令牌上下文窗口，請遷移到 [Claude Sonnet 5、Claude Sonnet 4.6、Claude Opus 4.6、Claude Opus 4.7 或 Claude Opus 4.8](https://platform.claude.com/docs/zh-TW/about-claude/models/overview)，它們以標準定價包括 1M 上下文，無需 beta 標頭。
</Warning>

<h3 id="slashcommand">
  `SlashCommand`
</h3>

有關可用 slash command 的信息。

```typescript theme={null}
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
  aliases?: string[];
};
```

<h3 id="modelinfo">
  `ModelInfo`
</h3>

有關可用模型的信息。

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

| 字段                         | 類型                                                                 | 描述                                                                                                                                         |
| :------------------------- | :----------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| `value`                    | `string`                                                           | 在 API 呼叫中傳遞的模型標識符                                                                                                                          |
| `resolvedModel`            | `string \| undefined`                                              | 此項目的 `value` 解析為的規範線路模型 ID。別名項目（如 `sonnet`）解析為明確的模型 ID（如 `claude-sonnet-5`），因此主機可以將儲存的明確模型 ID 與涵蓋它的別名項目進行匹配。需要 Claude Code v2.1.197 或更高版本。 |
| `displayName`              | `string`                                                           | 人類可讀的顯示名稱                                                                                                                                  |
| `description`              | `string`                                                           | 模型功能的描述                                                                                                                                    |
| `supportsEffort`           | `boolean \| undefined`                                             | 此模型是否支持努力級別                                                                                                                                |
| `supportedEffortLevels`    | `("low" \| "medium" \| "high" \| "xhigh" \| "max")[] \| undefined` | 此模型接受的努力級別                                                                                                                                 |
| `supportsAdaptiveThinking` | `boolean \| undefined`                                             | 此模型是否支持自適應思考，其中 Claude 決定何時以及多少推理                                                                                                          |
| `supportsFastMode`         | `boolean \| undefined`                                             | 此模型是否支持快速模式                                                                                                                                |
| `supportsAutoMode`         | `boolean \| undefined`                                             | 此模型是否支持自動模式                                                                                                                                |

<h3 id="agentinfo">
  `AgentInfo`
</h3>

有關可通過 Agent 工具調用的可用子代理的信息。

```typescript theme={null}
type AgentInfo = {
  name: string;
  description: string;
  model?: string;
};
```

| 字段            | 類型                    | 描述                                          |
| :------------ | :-------------------- | :------------------------------------------ |
| `name`        | `string`              | 代理類型標識符（例如，`"Explore"`、`"general-purpose"`） |
| `description` | `string`              | 何時使用此代理的描述                                  |
| `model`       | `string \| undefined` | 此代理使用的模型別名。如果省略，繼承父代理的模型                    |

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

連接的 MCP 服務器的狀態。

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
  tools?: {
    name: string;
    description?: string;
    annotations?: {
      readOnly?: boolean;
      destructive?: boolean;
      openWorld?: boolean;
    };
  }[];
};
```

<h3 id="mcpserverstatusconfig">
  `McpServerStatusConfig`
</h3>

MCP 服務器的配置，如 `mcpServerStatus()` 報告的那樣。這是所有 MCP 服務器傳輸類型的聯合。

```typescript theme={null}
type McpServerStatusConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfig
  | McpClaudeAIProxyServerConfig;
```

見 [`McpServerConfig`](#mcpserverconfig) 了解每種傳輸類型的詳情。

<h3 id="accountinfo">
  `AccountInfo`
</h3>

經過身份驗證的用戶的帳戶信息。

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

結果消息中返回的每個模型使用統計。`costUSD` 值是客戶端估計。見 [跟蹤成本和使用情況](/docs/zh-TW/agent-sdk/cost-tracking) 了解計費注意事項。

```typescript theme={null}
type ModelUsage = {
  inputTokens: number;
  outputTokens: number;
  cacheReadInputTokens: number;
  cacheCreationInputTokens: number;
  webSearchRequests: number;
  costUSD: number;
  contextWindow: number;
  maxOutputTokens: number;
};
```

<h3 id="configscope">
  `ConfigScope`
</h3>

```typescript theme={null}
type ConfigScope = "local" | "user" | "project";
```

<h3 id="nonnullableusage">
  `NonNullableUsage`
</h3>

[`Usage`](#usage) 的版本，所有可空字段都變為非可空。

```typescript theme={null}
type NonNullableUsage = {
  [K in keyof Usage]: NonNullable<Usage[K]>;
};
```

<h3 id="usage">
  `Usage`
</h3>

令牌使用統計。這是來自 `@anthropic-ai/sdk` 的 `BetaUsage` 類型。

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
};
```

`BetaServerToolUsage` 和 `BetaIterationsUsage` 在 `@anthropic-ai/sdk` 中定義。

<h3 id="calltoolresult">
  `CallToolResult`
</h3>

MCP 工具結果類型（來自 `@modelcontextprotocol/sdk/types.js`）。`structuredContent` 是一個 JSON 對象，可以與 `content` 一起返回，包括圖像塊。見 [返回結構化數據](/docs/zh-TW/agent-sdk/custom-tools#return-structured-data)。

```typescript theme={null}
type CallToolResult = {
  content: Array<{
    type: "text" | "image" | "audio" | "resource" | "resource_link";
    // 其他字段因類型而異
  }>;
  structuredContent?: Record<string, unknown>;
  isError?: boolean;
};
```

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

控制 Claude 的思考/推理行為。優先於已棄用的 `maxThinkingTokens`。

```typescript theme={null}
type ThinkingDisplay = "summarized" | "omitted";

type ThinkingConfig =
  | { type: "adaptive"; display?: ThinkingDisplay } // 模型確定何時以及多少推理（Opus 4.6+）
  | { type: "enabled"; budgetTokens?: number; display?: ThinkingDisplay } // 固定思考令牌預算
  | { type: "disabled" }; // 無擴展思考
```

可選的 `display` 字段控制思考文本是否返回為 `"summarized"` 或 `"omitted"`。在 Claude Opus 4.7 及更高版本上，API 默認值為 `"omitted"`，因此設置 `"summarized"` 以在 `thinking` 塊中接收思考內容。

<h3 id="spawnedprocess">
  `SpawnedProcess`
</h3>

自定義進程生成的介面（與 `spawnClaudeCodeProcess` 選項一起使用）。`ChildProcess` 已滿足此介面。

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

傳遞給自定義生成函數的選項。

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
  `signal` 字段告訴您的生成函數何時拆除進程。將其作為 `signal` 選項傳遞給 Node 的 `spawn()`，或將其傳遞給您的 VM 或容器拆除處理程序。

  此信號不會在 [`Options.abortController`](#options) 中止的瞬間觸發。SDK 首先關閉進程的 stdin 並等待約兩秒，以便 CLI 可以乾淨地關閉，然後中止此信號。要在調用者中止的瞬間做出反應，請改為監聽您自己的 `Options.abortController.signal`，您的生成函數可以從其封閉範圍引用。
</Note>

<h3 id="mcpsetserversresult">
  `McpSetServersResult`
</h3>

`setMcpServers()` 操作的結果。

```typescript theme={null}
type McpSetServersResult = {
  added: string[];
  removed: string[];
  errors: Record<string, string>;
};
```

<h3 id="rewindfilesresult">
  `RewindFilesResult`
</h3>

`rewindFiles()` 操作的結果。

```typescript theme={null}
type RewindFilesResult = {
  canRewind: boolean;
  error?: string;
  filesChanged?: string[];
  insertions?: number;
  deletions?: number;
};
```

<h3 id="sdkstatusmessage">
  `SDKStatusMessage`
</h3>

狀態更新消息（例如，壓縮）。

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

後台任務完成、失敗或停止時的通知。後台任務包括 `run_in_background` Bash 命令、[Monitor](#monitor) 監視和後台子代理。

```typescript theme={null}
type SDKTaskNotificationMessage = {
  type: "system";
  subtype: "task_notification";
  task_id: string;
  tool_use_id?: string;
  status: "completed" | "failed" | "stopped";
  output_file: string;
  summary: string;
  usage?: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktoolusesummarymessage">
  `SDKToolUseSummaryMessage`
</h3>

對話中工具使用的摘要。

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

Hook 開始執行時發出。

Claude Code 將此消息、[`SDKHookProgressMessage`](#sdkhookprogressmessage) 和 [`SDKHookResponseMessage`](#sdkhookresponsemessage) 立即傳遞到消息流，包括在會話啟動期間 `SessionStart` 或 `Setup` hook 仍在運行時。Claude Code v2.1.169 至 v2.1.203 在 `SessionStart` 或 `Setup` hook 完成後以一個批次傳遞這些消息；v2.1.204 恢復了實時傳遞。

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

Hook 運行時發出，帶有 stdout/stderr 輸出。

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

Hook 完成執行時發出。

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

工具執行時定期發出，以指示進度。

```typescript theme={null}
type SDKToolProgressMessage = {
  type: "tool_progress";
  tool_use_id: string;
  tool_name: string;
  parent_tool_use_id: string | null;
  elapsed_time_seconds: number;
  task_id?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkauthstatusmessage">
  `SDKAuthStatusMessage`
</h3>

在身份驗證流程中發出。

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

後台任務開始時發出。`task_type` 字段是 `"local_bash"` 用於後台 Bash 命令和 [Monitor](#monitor) 監視，`"local_agent"` 用於子代理，或 `"remote_agent"`。

```typescript theme={null}
type SDKTaskStartedMessage = {
  type: "system";
  subtype: "task_started";
  task_id: string;
  tool_use_id?: string;
  description: string;
  task_type?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskprogressmessage">
  `SDKTaskProgressMessage`
</h3>

子代理或後台任務運行時定期發出。`summary` 字段僅在啟用 [`agentProgressSummaries`](#options) 時填充。

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

後台任務的狀態發生變化時發出，例如當它從 `running` 轉換為 `completed` 時。將 `patch` 合併到按 `task_id` 鍵入的本地任務映射中。`end_time` 字段是 Unix 紀元時間戳（以毫秒為單位），可與 `Date.now()` 比較。

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

每當實時後台任務集合發生變化時發出：任務啟動、完成、被殺死，或前台代理被後台化。`tasks` 陣列是完整的實時集合。用每個有效負載替換任何緩存的集合，而不是配對 `task_started` 和 `task_notification` 事件，以便下一個成員資格變化更正您可能錯過的任何事件。

相對於這些每個任務事件的順序是未指定的，因此不要關聯這兩個流。

啟動時不發出任何內容。每當會話的 CLI 進程啟動或重新啟動時重置為空集，並讓下一個成員資格變化重新填充它。

需要 Claude Code v2.1.203 或更高版本。

```typescript theme={null}
type SDKBackgroundTasksChangedMessage = {
  type: "system";
  subtype: "background_tasks_changed";
  tasks: {
    task_id: string;
    task_type: string;
    description: string;
  }[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkthinkingtokensmessage">
  `SDKThinkingTokensMessage`
</h3>

在 Claude 生成思考塊（包括編輯過的思考塊）時發出，帶有迄今為止生成的思考令牌的運行估計。`estimated_tokens` 是當前思考塊的運行總計，`estimated_tokens_delta` 是此幀攜帶的增量。將其用於進度顯示。頂級代理循環的最終計數是結果消息的 `usage.output_tokens`，它[不包括子代理令牌](/docs/zh-TW/agent-sdk/cost-tracking#get-the-total-cost-of-a-query)；使用 [`modelUsage`](#modelusage) 進行整樹會計。

需要 Claude Code v2.1.153 或更高版本。

```typescript theme={null}
type SDKThinkingTokensMessage = {
  type: "system";
  subtype: "thinking_tokens";
  estimated_tokens: number;
  estimated_tokens_delta: number;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkfilespersistedevent">
  `SDKFilesPersistedEvent`
</h3>

文件檢查點持久化到磁盤時發出。

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

會話遇到速率限制時發出。

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

當 `errorCode` 為 `"credits_required"` 時，拒絕來自 claude.ai 訂閱，其包含的使用量已耗盡，會話無法繼續，直到用戶購買使用額度。`canUserPurchaseCredits` 指示經過身份驗證的用戶是否可以為帳戶購買額度，`hasChargeableSavedPaymentMethod` 指示是否有保存的付款方式。這三個字段在非信用額度必需拒絕的速率限制事件上不存在。需要 Claude Code v2.1.181 或更高版本。

<h3 id="sdklocalcommandoutputmessage">
  `SDKLocalCommandOutputMessage`
</h3>

本地 slash command 的輸出（例如，`/voice` 或 `/usage`）。在記錄中顯示為助手風格的文本。

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

可用命令集在會話中期發生變化時發出，例如當代理進入子目錄時發現技能。`commands` 陣列是完整的更新列表，因此用此有效負載替換任何緩存的命令列表。再次調用 `supportedCommands()` 不等同：該方法返回在初始化時捕獲的快照，不反映會話中期的變化。

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

啟用 `promptSuggestions` 時在每個轉數後發出。包含預測的下一個用戶提示。

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

會話的對話被替換而不結束會話時發出，例如在 `/clear` 之後、計劃模式退出時，或當新對話啟動時。在 `new_conversation_id` 下掛載空記錄，並丟棄任何緩存的會話標題。

```typescript theme={null}
type SDKConversationResetMessage = {
  type: "conversation_reset";
  new_conversation_id: UUID;
  uuid: UUID;
  session_id: string;
};
```

SDK 的已發佈類型在 Claude Code v2.1.203 及更高版本中聲明 `SDKConversationResetMessage`。在 v2.1.203 之前，`SDKMessage` 引用該類型而不聲明它，因此當 `skipLibCheck` 被禁用時，在 `type === "conversation_reset"` 上縮小範圍失敗類型檢查。

<h3 id="aborterror">
  `AbortError`
</h3>

中止操作的自定義錯誤類。

```typescript theme={null}
class AbortError extends Error {}
```

<h2 id="sandbox-configuration">
  沙箱配置
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

沙箱行為的配置。使用此選項以編程方式啟用命令沙箱和配置網絡限制。

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

| 屬性                          | 類型                                                    | 預設值         | 描述                                                                                                                              |
| :-------------------------- | :---------------------------------------------------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------ |
| `enabled`                   | `boolean`                                             | `false`     | 為命令執行啟用沙箱模式                                                                                                                     |
| `failIfUnavailable`         | `boolean`                                             | `true`      | 如果 `enabled` 為 `true` 但沙箱無法啟動，則在啟動時停止。設置為 `false` 以回退到無沙箱執行，並在 stderr 上顯示警告                                                     |
| `autoAllowBashIfSandboxed`  | `boolean`                                             | `true`      | 啟用沙箱時自動批准 bash 命令                                                                                                               |
| `excludedCommands`          | `string[]`                                            | `[]`        | 始終繞過沙箱限制的命令（例如，`['docker']`）。這些自動運行在沙箱外，無需模型參與                                                                                  |
| `allowUnsandboxedCommands`  | `boolean`                                             | `true`      | 允許模型請求在沙箱外運行命令。當為 `true` 時，模型可以在工具輸入中設置 `dangerouslyDisableSandbox`，這會回退到[權限系統](#permissions-fallback-for-unsandboxed-commands) |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `undefined` | 網絡特定的沙箱配置                                                                                                                       |
| `filesystem`                | [`SandboxFilesystemConfig`](#sandboxfilesystemconfig) | `undefined` | 檔案系統特定的沙箱配置，用於讀/寫限制                                                                                                             |
| `ignoreViolations`          | `Record<string, string[]>`                            | `undefined` | 違規類別到要忽略的模式的映射（例如，`{ file: ['/tmp/*'], network: ['localhost'] }`）                                                               |
| `enableWeakerNestedSandbox` | `boolean`                                             | `false`     | 為相容性啟用較弱的嵌套沙箱                                                                                                                   |
| `ripgrep`                   | `{ command: string; args?: string[] }`                | `undefined` | 沙箱環境中的自訂 ripgrep 二進制配置                                                                                                          |

<Note>
  沙箱取決於平台支援，在 Linux 上，還需要 `bubblewrap` 和 `socat` 等工具。當 `enabled` 為 `true` 且沙箱無法啟動時，`query()` 會報告一條 `result` 訊息，其中 `subtype: "error_during_execution"`，並在 `errors` 中包含原因。對於單一訊息 `query()` 呼叫，SDK 會在產生該錯誤結果後拋出異常，因此請將迴圈包裝在 try 區塊中以繼續執行。請參閱[處理結果](/docs/zh-TW/agent-sdk/agent-loop#handle-the-result)以了解錯誤合約。

  要改為運行無沙箱，請設置 `failIfUnavailable: false`。
</Note>

<h4 id="example-usage">
  範例用法
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
  // 單一 query() 呼叫會在產生錯誤結果後拋出異常，
  // 例如當沙箱無法啟動時（failIfUnavailable 預設為 true）。
  console.log(`Session ended with an error: ${error}`);
}
```

<Warning>
  **Unix socket 安全性：** `allowUnixSockets` 選項可以授予對強大系統服務的訪問權限。例如，允許 `/var/run/docker.sock` 實際上通過 Docker API 授予對主機系統的完全訪問權限，繞過沙箱隔離。僅允許絕對必要的 Unix sockets 並了解每個的安全含義。
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

沙箱模式的網絡特定配置。這些設置適用於當父級 [`SandboxSettings`](#sandboxsettings) 中的 `enabled` 為 `true` 時的沙箱化 Bash 命令。它們不限制 WebFetch 工具，該工具改用[權限規則](/docs/zh-TW/permissions#webfetch)。

```typescript theme={null}
type SandboxNetworkConfig = {
  allowedDomains?: string[];
  deniedDomains?: string[];
  allowManagedDomainsOnly?: boolean;
  allowLocalBinding?: boolean;
  allowUnixSockets?: string[];
  allowAllUnixSockets?: boolean;
  httpProxyPort?: number;
  socksProxyPort?: number;
};
```

| 屬性                        | 類型         | 預設值         | 描述                                                                                                                       |
| :------------------------ | :--------- | :---------- | :----------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `string[]` | `[]`        | 沙箱進程可以訪問的網域名稱                                                                                                            |
| `deniedDomains`           | `string[]` | `[]`        | 沙箱進程無法訪問的網域名稱。優先於 `allowedDomains`                                                                                       |
| `allowManagedDomainsOnly` | `boolean`  | `false`     | 僅限受管設定。在[受管設定](/docs/zh-TW/permissions#managed-settings)中設置時，僅遵守受管設定中的 `allowedDomains` 條目，而忽略來自使用者、專案或本機設定的條目。通過 SDK 選項設置時無效 |
| `allowLocalBinding`       | `boolean`  | `false`     | 允許進程綁定到本機連接埠（例如，用於開發伺服器）                                                                                                 |
| `allowUnixSockets`        | `string[]` | `[]`        | 進程可以訪問的 Unix socket 路徑（例如，Docker socket）                                                                                 |
| `allowAllUnixSockets`     | `boolean`  | `false`     | 允許訪問所有 Unix sockets                                                                                                      |
| `httpProxyPort`           | `number`   | `undefined` | 網絡請求的 HTTP 代理連接埠                                                                                                         |
| `socksProxyPort`          | `number`   | `undefined` | 網絡請求的 SOCKS 代理連接埠                                                                                                        |

<Note>
  內置沙箱代理根據請求的主機名強制執行 `allowedDomains`，並且不終止或檢查 TLS 流量，因此[網域前置](https://en.wikipedia.org/wiki/Domain_fronting)等技術可能會繞過它。有關詳細資訊，請參閱[沙箱安全限制](/docs/zh-TW/sandboxing#security-limitations)，以及[安全部署](/docs/zh-TW/agent-sdk/secure-deployment#traffic-forwarding)以配置 TLS 終止代理。
</Note>

<h3 id="sandboxfilesystemconfig">
  `SandboxFilesystemConfig`
</h3>

沙箱模式的檔案系統特定配置。

```typescript theme={null}
type SandboxFilesystemConfig = {
  allowWrite?: string[];
  denyWrite?: string[];
  denyRead?: string[];
};
```

| 屬性           | 類型         | 預設值  | 描述            |
| :----------- | :--------- | :--- | :------------ |
| `allowWrite` | `string[]` | `[]` | 允許寫入訪問的檔案路徑模式 |
| `denyWrite`  | `string[]` | `[]` | 拒絕寫入訪問的檔案路徑模式 |
| `denyRead`   | `string[]` | `[]` | 拒絕讀取訪問的檔案路徑模式 |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  無沙箱命令的權限回退
</h3>

啟用 `allowUnsandboxedCommands` 時，模型可以通過在工具輸入中設置 `dangerouslyDisableSandbox: true` 來請求在沙箱外運行命令。這些請求回退到現有的權限系統，意味著您的 `canUseTool` 處理程序被調用，允許您實現自訂授權邏輯。在下面的範例中，`isCommandAuthorized` 代表您定義的授權檢查。

<Note>
  **`excludedCommands` vs `allowUnsandboxedCommands`：**

  * `excludedCommands`：始終自動繞過沙箱的命令的靜態列表（例如，`['docker']`）。模型對此無控制。
  * `allowUnsandboxedCommands`：讓模型在執行時通過在工具輸入中設置 `dangerouslyDisableSandbox: true` 來決定是否請求無沙箱執行。
</Note>

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Deploy my application",
  options: {
    sandbox: {
      enabled: true,
      allowUnsandboxedCommands: true // 模型可以請求無沙箱執行
    },
    permissionMode: "default",
    canUseTool: async (tool, input) => {
      // 檢查模型是否請求繞過沙箱
      if (tool === "Bash" && input.dangerouslyDisableSandbox) {
        // 模型請求在沙箱外運行此命令
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

此模式使您能夠：

* **審計模型請求：** 記錄模型何時請求無沙箱執行
* **實現允許清單：** 僅允許特定命令在沙箱外運行
* **新增批准工作流程：** 需要對特權操作進行明確授權

<Warning>
  使用 `dangerouslyDisableSandbox: true` 運行的命令具有完整的系統訪問權限。確保您的 `canUseTool` 處理程序仔細驗證這些請求。

  如果 `permissionMode` 設置為 `bypassPermissions` 且 `allowUnsandboxedCommands` 啟用，模型可以自主執行沙箱外的命令，無需任何批准提示（明確的[`ask` 規則](/docs/zh-TW/agent-sdk/permissions#how-permissions-are-evaluated)仍會強制執行一個）。此組合實際上允許模型無聲地逃離沙箱隔離。
</Warning>

<h2 id="see-also">
  另見
</h2>

* [SDK 概述](/docs/zh-TW/agent-sdk/overview) - 常規 SDK 概念
* [Python SDK 參考](/docs/zh-TW/agent-sdk/python) - Python SDK 文檔
* [CLI 參考](/docs/zh-TW/cli-reference) - 命令行介面
* [常見工作流](/docs/zh-TW/common-workflows) - 分步指南
