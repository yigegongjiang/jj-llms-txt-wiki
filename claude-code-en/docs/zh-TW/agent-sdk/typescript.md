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
  SDK 為您的平台捆綁了一個原生 Claude Code 二進制文件作為可選依賴項，例如 `@anthropic-ai/claude-agent-sdk-darwin-arm64`。大多數安裝不需要單獨安裝 Claude Code。SDK 版本追蹤捆綁的 Claude Code 版本。SDK v0.3.191 捆綁 Claude Code v2.1.191，因此本頁面上需要特定 Claude Code 版本的功能需要具有相同補丁號或更高版本的 SDK 版本。如果您的包管理器跳過可選依賴項，SDK 會拋出 `Native CLI binary for <platform>-<arch> not found`；改為將 [`pathToClaudeCodeExecutable`](#options) 設置為單獨安裝的 `claude` 二進制文件。

  如果您的包管理器不應用 npm 的 `libc` 欄位（如 Yarn 1.x 不應用），您會在 Linux 上同時獲得 glibc 和 musl 平台包，大約使安裝大小翻倍。在 Agent SDK v0.2.141 或更高版本上，SDK 仍會啟動正確的變體。要在容器映像中回收空間，請刪除與您的應用程式運行的 libc 不匹配的平台包；對於 x64 上的 glibc 運行時，即 `rm -rf node_modules/@anthropic-ai/claude-agent-sdk-linux-x64-musl`。在開發機器上，刪除是臨時的，因為 Yarn 會在下一次依賴項更改時重新安裝該包。
</Note>

<h3 id="compile-to-a-single-executable">
  編譯為單個可執行文件
</h3>

當您使用 `bun build --compile` 將應用程式編譯為單個文件可執行文件時，SDK 無法在運行時解析捆綁的 CLI 二進制文件。`require.resolve` 在編譯後可執行文件的 `$bunfs` 虛擬文件系統內不起作用，因此 SDK 會拋出 `Native CLI binary for <platform>-<arch> not found`。

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

`extractFromBunfs()` 將嵌入的二進制文件從編譯後可執行文件的虛擬文件系統複製到每個使用者的臨時目錄，並返回真實路徑。在編譯後的可執行文件外，它返回輸入路徑不變，因此相同的程式碼在開發中無需修改即可運行。

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
| `initializeTimeoutMs` | `number`              | 等待子進程初始化的最大時間（毫秒）。預設為 `60000`。如果初始化未在時間內完成，promise 將以超時錯誤拒絕 |

<h4 id="returns-2">
  返回值
</h4>

返回一個 `Promise<`[`WarmQuery`](#warmquery)`>`，在子進程生成並完成其初始化握手後解析。

<h4 id="example">
  示例
</h4>

早期調用 `startup()`，例如在應用程式啟動時，然後在提示準備好後在返回的句柄上調用 `.query()`。這將子進程生成和初始化移出關鍵路徑。

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

為與 SDK MCP 伺服器一起使用創建類型安全的 MCP 工具定義。

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
  參數
</h4>

| 參數            | 類型                                                                                                     | 描述                                                                                                                                                                |
| :------------ | :----------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | `string`                                                                                               | 工具的名稱                                                                                                                                                             |
| `description` | `string`                                                                                               | 工具功能的描述                                                                                                                                                           |
| `inputSchema` | `Schema extends AnyZodRawShape`                                                                        | 定義工具輸入參數的 Zod 架構（支持 Zod 3 和 Zod 4）                                                                                                                                |
| `handler`     | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>`                                      | 執行工具邏輯的異步函數                                                                                                                                                       |
| `extras`      | `{ annotations?: `[`ToolAnnotations`](#toolannotations)`; searchHint?: string; alwaysLoad?: boolean }` | 可選的額外項。`annotations` 為客戶端提供 MCP 行為提示。`searchHint` 是當 [tool search](/docs/zh-TW/agent-sdk/tool-search) 啟用時在延遲工具列表中顯示的單行功能短語。`alwaysLoad: true` 將此工具的完整架構保留在初始提示中，而不是延遲它 |

<h4 id="toolannotations">
  `ToolAnnotations`
</h4>

從 `@modelcontextprotocol/sdk/types.js` 重新導出。所有字段都是可選提示；客戶端不應依賴它們進行安全決策。

| 字段                | 類型        | 預設值         | 描述                                                             |
| :---------------- | :-------- | :---------- | :------------------------------------------------------------- |
| `title`           | `string`  | `undefined` | 工具的人類可讀標題                                                      |
| `readOnlyHint`    | `boolean` | `false`     | 如果為 `true`，工具不會修改其環境                                           |
| `destructiveHint` | `boolean` | `true`      | 如果為 `true`，工具可能執行破壞性更新（僅在 `readOnlyHint` 為 `false` 時有意義）       |
| `idempotentHint`  | `boolean` | `false`     | 如果為 `true`，使用相同參數的重複調用沒有額外效果（僅在 `readOnlyHint` 為 `false` 時有意義） |
| `openWorldHint`   | `boolean` | `true`      | 如果為 `true`，工具與外部實體交互（例如，網路搜尋）。如果為 `false`，工具的域是封閉的（例如，記憶工具）    |

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

創建在與應用程式相同的程序中運行的 MCP 伺服器實例。

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
  參數
</h4>

| 參數                     | 類型                            | 描述                                                                                                                                                      |
| :--------------------- | :---------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `options.name`         | `string`                      | MCP 伺服器的名稱                                                                                                                                              |
| `options.version`      | `string`                      | 可選版本字符串                                                                                                                                                 |
| `options.instructions` | `string`                      | 可選伺服器指示，從 `initialize` 返回並作為 MCP 指示區塊呈現給模型                                                                                                              |
| `options.tools`        | `Array<SdkMcpToolDefinition>` | 使用 [`tool()`](#tool) 創建的工具定義陣列                                                                                                                          |
| `options.alwaysLoad`   | `boolean`                     | 當為 `true` 時，此伺服器的每個工具都保留在初始提示中，永遠不會延遲到 [tool search](/docs/zh-TW/agent-sdk/tool-search) 後面。與 [`tool()`](#tool) 中的每個工具 `alwaysLoad` 結合                        |
| `options.timeout`      | `number`                      | 此伺服器的工具調用超時時間（毫秒）。Claude Code 將其應用於此伺服器以代替 [`MCP_TOOL_TIMEOUT`](/docs/zh-TW/env-vars)。傳遞至少 1000 的整數。Claude Code 忽略其他值。需要 TypeScript Agent SDK v0.3.248 或更高版本 |

<h3 id="listsessions">
  `listSessions()`
</h3>

發現並列出具有輕量級元資料的過去會話。按項目目錄篩選或列出所有項目中的會話。

```typescript theme={null}
function listSessions(options?: ListSessionsOptions): Promise<SDKSessionInfo[]>;
```

<h4 id="parameters-5">
  參數
</h4>

| 參數                         | 類型        | 預設值         | 描述                                        |
| :------------------------- | :-------- | :---------- | :---------------------------------------- |
| `options.dir`              | `string`  | `undefined` | 列出會話的目錄。省略時，返回所有項目中的會話                    |
| `options.limit`            | `number`  | `undefined` | 返回的最大會話數                                  |
| `options.includeWorktrees` | `boolean` | `true`      | 當 `dir` 在 git 存儲庫內時，包括來自所有 worktree 路徑的會話 |

<h4 id="return-type-sdksessioninfo">
  返回類型：`SDKSessionInfo`
</h4>

| 屬性             | 類型                    | 描述                                          |
| :------------- | :-------------------- | :------------------------------------------ |
| `sessionId`    | `string`              | 唯一會話標識符（UUID）                               |
| `summary`      | `string`              | 顯示標題：自定義標題、自動生成的摘要或第一個提示                    |
| `lastModified` | `number`              | 上次修改時間（自紀元以來的毫秒數）                           |
| `fileSize`     | `number \| undefined` | 會話檔案大小（位元組）。僅針對本地 JSONL 存儲填充                |
| `customTitle`  | `string \| undefined` | 使用者設置的會話標題（通過 `/rename`）                    |
| `firstPrompt`  | `string \| undefined` | 會話中的第一個有意義的使用者提示                            |
| `gitBranch`    | `string \| undefined` | 會話結束時的 Git 分支                               |
| `cwd`          | `string \| undefined` | 會話的工作目錄                                     |
| `tag`          | `string \| undefined` | 使用者設置的會話標籤（見 [`tagSession()`](#tagsession)） |
| `createdAt`    | `number \| undefined` | 創建時間（自紀元以來的毫秒數），來自第一個條目的時間戳                 |

<h4 id="example-2">
  示例
</h4>

打印項目的 10 個最近會話。結果按 `lastModified` 降序排序，因此第一項是最新的。省略 `dir` 以搜尋所有項目。

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

從過去的會話記錄中讀取使用者和助手消息。

```typescript theme={null}
function getSessionMessages(
  sessionId: string,
  options?: GetSessionMessagesOptions
): Promise<SessionMessage[]>;
```

<h4 id="parameters-6">
  參數
</h4>

| 參數               | 類型       | 預設值         | 描述                              |
| :--------------- | :------- | :---------- | :------------------------------ |
| `sessionId`      | `string` | 必需          | 要讀取的會話 UUID（見 `listSessions()`） |
| `options.dir`    | `string` | `undefined` | 查找會話的項目目錄。省略時，搜尋所有項目            |
| `options.limit`  | `number` | `undefined` | 返回的最大消息數                        |
| `options.offset` | `number` | `undefined` | 從開始跳過的消息數                       |

<h4 id="return-type-sessionmessage">
  返回類型：`SessionMessage`
</h4>

| 屬性                   | 類型                      | 描述                                                                                                                                                          |
| :------------------- | :---------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`               | `"user" \| "assistant"` | 消息角色                                                                                                                                                        |
| `uuid`               | `string`                | 唯一消息標識符                                                                                                                                                     |
| `session_id`         | `string`                | 此消息所屬的會話                                                                                                                                                    |
| `message`            | `unknown`               | 來自記錄的原始消息有效負載                                                                                                                                               |
| `parent_tool_use_id` | `string \| null`        | 對於子代理消息，生成 `Agent` 或 `Skill` 工具調用的 `tool_use_id`，該調用啟動了子代理。對於主會話消息和較舊的會話為 `null`                                                                            |
| `parent_agent_id`    | `string \| null`        | 對於來自[嵌套子代理](/docs/zh-TW/sub-agents#let-subagents-spawn-their-own-subagents)的消息，生成它的子代理的 `agentId`。對於主會話消息、來自頂級子代理的消息和較舊的會話為 `null`。需要 Claude Code v2.1.202 或更高版本 |

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

按 ID 讀取單個會話的元資料，無需掃描完整項目目錄。

```typescript theme={null}
function getSessionInfo(
  sessionId: string,
  options?: GetSessionInfoOptions
): Promise<SDKSessionInfo | undefined>;
```

<h4 id="parameters-7">
  參數
</h4>

| 參數            | 類型       | 預設值         | 描述                  |
| :------------ | :------- | :---------- | :------------------ |
| `sessionId`   | `string` | 必需          | 要查找的會話 UUID         |
| `options.dir` | `string` | `undefined` | 項目目錄路徑。省略時，搜尋所有項目目錄 |

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

| 參數            | 類型       | 預設值         | 描述                  |
| :------------ | :------- | :---------- | :------------------ |
| `sessionId`   | `string` | 必需          | 要重命名的會話 UUID        |
| `title`       | `string` | 必需          | 新標題。修剪空格後必須非空       |
| `options.dir` | `string` | `undefined` | 項目目錄路徑。省略時，搜尋所有項目目錄 |

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

| 參數            | 類型               | 預設值         | 描述                  |
| :------------ | :--------------- | :---------- | :------------------ |
| `sessionId`   | `string`         | 必需          | 要標記的會話 UUID         |
| `tag`         | `string \| null` | 必需          | 標籤字符串，或 `null` 以清除  |
| `options.dir` | `string`         | `undefined` | 項目目錄路徑。省略時，搜尋所有項目目錄 |

<h3 id="resolvesettings">
  `resolveSettings()`
</h3>

使用與 CLI 相同的合併引擎為給定目錄解析有效的 Claude Code 設定，無需生成 Claude CLI。在調用 `query()` 之前使用它來檢查 `query()` 調用將看到什麼配置。

<Note>
  此函數處於 alpha 階段，其 API 在穩定之前可能會更改。
</Note>

快照與實時 `query()` 會話應用的內容不同：

* **`policyHelper`**：`resolveSettings()` 讀取 MDM 源，包括 macOS plist 和 Windows HKLM/HKCU，但不執行管理員配置的 `policyHelper` 子程序。
* **伺服器託管設定**：`resolveSettings()` 不會獲取[伺服器託管設定](/docs/zh-TW/server-managed-settings#fetch-and-caching-behavior)。將它們作為 `options.serverManagedSettings` 傳遞以包括它們。
* **`defaultMode`**：快照從每個層級按原樣返回 `permissions.defaultMode`，因此它可以包括項目和本地設定中的 `'auto'` 和 `'bypassPermissions'` 值，[實時會話忽略](/docs/zh-TW/permission-modes#which-mode-a-session-starts-in)。

```typescript theme={null}
function resolveSettings(
  options?: ResolveSettingsOptions
): Promise<ResolvedSettings>;
```

<h4 id="parameters-10">
  參數
</h4>

`resolveSettings()` 接受單個選項對象。所有字段都是可選的。

| 參數                              | 類型                                    | 預設值             | 描述                                                                                                                                                                          |
| :------------------------------ | :------------------------------------ | :-------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.cwd`                   | `string`                              | `process.cwd()` | 用於解析項目和本地設定的相對目錄                                                                                                                                                            |
| `options.settingSources`        | [`SettingSource`](#settingsource)`[]` | 所有源             | 要加載的檔案系統源。傳遞 `[]` 以跳過使用者、項目和本地設定。[端點託管策略](/docs/zh-TW/managed-settings#delivery-mechanisms)在所有情況下都會加載。`resolveSettings()` 僅當您傳遞 `options.serverManagedSettings` 時才包括伺服器託管設定      |
| `options.managedSettings`       | `Settings`                            | `undefined`     | 由嵌入主機提供的策略層設定。遵循與 [`managedSettings` in `Options`](#options) 相同的規則，除了 `resolveSettings()` 不執行配置的 [`policyHelper`](/docs/zh-TW/settings-reference#policyhelper)，因此快照可以包括實時會話丟棄的設定 |
| `options.serverManagedSettings` | `Settings`                            | `undefined`     | 來自 `/api/claude_code/settings` 的伺服器託管設定有效負載。非限制性鍵無過濾地通過                                                                                                                     |

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

下面的示例為項目目錄解析設定並打印控制清理期的源。在沒有設定檔案設置 `cleanupPeriodDays` 的機器上，兩條打印的行都顯示 `undefined` 作為值，這是預期的輸出而不是錯誤。

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

`query()` 函式的設定物件。

| 屬性                                | 類型                                                                                                                                                                                                             | 預設值                           | 說明                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `abortController`                 | `AbortController`                                                                                                                                                                                              | `new AbortController()`       | 用於取消操作的控制器                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `additionalDirectories`           | `string[]`                                                                                                                                                                                                     | `[]`                          | Claude 可以存取的額外目錄。SDK 會將每個項目作為 `--add-dir` 傳遞給 Claude Code，因此使用 `project` 設定來源時，Claude Code 也會[載入目錄的技能、命令和子代理](/docs/zh-TW/permissions#additional-directories-grant-file-access-not-configuration)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `agent`                           | `string`                                                                                                                                                                                                       | `undefined`                   | 主執行緒的代理名稱。代理必須在 `agents` 選項或設定中定義                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `agents`                          | `Record<string, [`AgentDefinition`](#agentdefinition)>`                                                                                                                                                        | `undefined`                   | 以程式設計方式定義子代理                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `agentProgressSummaries`          | `boolean`                                                                                                                                                                                                      | `false`                       | 當為 `true` 時，為子代理產生單行進度摘要，並透過 `summary` 欄位在 [`task_progress`](#sdktaskprogressmessage) 事件上轉發。適用於前景和背景子代理                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `allowDangerouslySkipPermissions` | `boolean`                                                                                                                                                                                                      | `false`                       | 啟用略過權限。使用 `permissionMode: 'bypassPermissions'` 時需要，可在啟動時或稍後透過 `setPermissionMode()` 設定。請參閱[計畫模式](/docs/zh-TW/agent-sdk/permissions#plan-mode-plan)以了解它如何與 `permissionMode: 'plan'` 互動                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `allowedTools`                    | `string[]`                                                                                                                                                                                                     | `[]`                          | 自動核准而不提示的工具。這不會限制 Claude 只能使用這些工具。如果您在此處命名其中一個[任務追蹤工具](/docs/zh-TW/agent-sdk/todo-tracking#model-availability)，Claude Code 也會選擇加入工作階段。其他未列出的工具會根據 `permissionMode` 和 `canUseTool` 進行處理。使用 `disallowedTools` 來封鎖工具。請參閱[權限](/docs/zh-TW/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `betas`                           | [`SdkBeta`](#sdkbeta)`[]`                                                                                                                                                                                      | `[]`                          | 啟用測試版功能                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `canUseTool`                      | [`CanUseTool`](#canusetool)                                                                                                                                                                                    | `undefined`                   | 自訂權限函式，僅在[權限流程](/docs/zh-TW/agent-sdk/permissions#how-permissions-are-evaluated)落實到提示時叫用。不會針對由 `allowedTools`、允許規則或 `permissionMode` 自動核准的呼叫叫用。允許規則不會預先核准[任何模式都不自動核准的動作](/docs/zh-TW/permission-modes#actions-no-mode-auto-approves)。請參閱 [`CanUseTool`](#canusetool) 以取得詳細資訊                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `continue`                        | `boolean`                                                                                                                                                                                                      | `false`                       | 繼續最近的對話                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `cwd`                             | `string`                                                                                                                                                                                                       | `process.cwd()`               | 目前的工作目錄                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `debug`                           | `boolean`                                                                                                                                                                                                      | `false`                       | 為 Claude Code 程序啟用偵錯模式                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `debugFile`                       | `string`                                                                                                                                                                                                       | `undefined`                   | 將偵錯日誌寫入特定檔案路徑。隱含啟用偵錯模式                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `disallowedTools`                 | `string[]`                                                                                                                                                                                                     | `[]`                          | 要拒絕的工具。裸名稱（例如 `"Bash"`）會從 Claude 的內容中移除工具。範圍規則（例如 `"Bash(rm *)"`）會保留工具可用，並在每個權限模式中拒絕符合的呼叫，包括 `bypassPermissions`，針對[如所寫的](/docs/zh-TW/permissions#bash-rule-limits)命令。請參閱[權限](/docs/zh-TW/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `effort`                          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max'`                                                                                                                                                              | `undefined`                   | 控制 Claude 在其回應中投入多少努力。與自適應思考配合使用以引導思考深度。請參閱[調整努力等級](/docs/zh-TW/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `enableFileCheckpointing`         | `boolean`                                                                                                                                                                                                      | `false`                       | 啟用檔案變更追蹤以進行倒帶。請參閱[檔案檢查點](/docs/zh-TW/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `env`                             | `Record<string, string \| undefined>`                                                                                                                                                                          | `process.env`                 | 環境變數。設定時，這會取代子程序環境而不是與 `process.env` 合併，因此請傳遞 `{ ...process.env, YOUR_VAR: 'value' }` 以保留繼承的變數（例如 `PATH`）。請參閱[處理緩慢或停滯的 API 回應](#handle-slow-or-stalled-api-responses)以取得此模式的範例，以及[環境變數](/docs/zh-TW/env-vars)以了解基礎 CLI 讀取的變數。設定 `CLAUDE_AGENT_SDK_CLIENT_APP` 以在 User-Agent 標頭中識別您的應用程式                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `executable`                      | `'bun' \| 'deno' \| 'node'`                                                                                                                                                                                    | 自動偵測                          | 要使用的 JavaScript 執行時間                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `executableArgs`                  | `string[]`                                                                                                                                                                                                     | `[]`                          | 要傳遞給可執行檔的引數                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `extraArgs`                       | `Record<string, string \| null>`                                                                                                                                                                               | `{}`                          | 其他引數                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `fallbackModel`                   | `string`                                                                                                                                                                                                       | `undefined`                   | 主要模型失敗時要使用的模型。接受逗號分隔的清單。如需順序和上限，請參閱[後備模型鏈](/docs/zh-TW/model-config#fallback-model-chains)。如需指導，請參閱[選擇模型](/docs/zh-TW/agent-sdk/configuration#choose-a-model)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `forkSession`                     | `boolean`                                                                                                                                                                                                      | `false`                       | 使用 `resume` 繼續時，分支到新的工作階段 ID 而不是繼續原始工作階段                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `forwardSubagentText`             | `boolean`                                                                                                                                                                                                      | `false`                       | 轉發子代理文字和思考區塊作為助手和使用者訊息，並設定 `parent_tool_use_id`，以便消費者可以呈現巢狀文字記錄。沒有此選項，Claude Code 會發出子代理 `tool_use` 和 `tool_result` 區塊，但不會發出文字或思考。來自每個巢狀深度的子代理的訊息會在 Claude Code v2.1.219 及更新版本上轉發；在 v2.1.219 之前，只有來自深度 1 子代理的訊息出現。子代理衍生的分支技能訊息，以及巢狀分支技能的訊息，需要 v2.1.275 或更新版本                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `hooks`                           | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>`                                                                                                              | `{}`                          | 事件的 Hook 回呼                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `includeHookEvents`               | `boolean`                                                                                                                                                                                                      | `false`                       | 在訊息串流中包含 hook 生命週期事件，作為 [`SDKHookStartedMessage`](#sdkhookstartedmessage)、[`SDKHookProgressMessage`](#sdkhookprogressmessage) 和 [`SDKHookResponseMessage`](#sdkhookresponsemessage)。`SessionStart` 和 `Setup` hooks 的生命週期事件始終包含在內，不需要此選項。某些 hook 事件（例如 `Notification`、`SessionEnd`、`PreCompact` 和 `PostCompact`）永遠不會產生 `SDKHookStartedMessage`，即使使用此選項也是如此。對於這些事件，Claude Code 仍會在執行超過一秒的命令 hook 時發出 `SDKHookProgressMessage` 並產生輸出，並且僅在 hook [在背景執行](/docs/zh-TW/hooks#run-hooks-in-the-background)時發出 `SDKHookResponseMessage`                                                                                                                                                                                                                               |
| `includePartialMessages`          | `boolean`                                                                                                                                                                                                      | `false`                       | 包含部分訊息事件                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `loadTimeoutMs`                   | `number`                                                                                                                                                                                                       | `60000`                       | *Alpha。* 在繼續具體化期間，每個 `sessionStore.load()` 和 `sessionStore.listSubkeys()` 呼叫的逾時（毫秒）。如果配接器未在此視窗內解決，查詢會失敗而不是掛起。未設定 `sessionStore` 時忽略                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `managedSettings`                 | `Settings`                                                                                                                                                                                                     | `undefined`                   | 您的主機程序提供給衍生工作階段的原則層級設定。在具有管理員部署的受管設定的機器上，Claude Code 會忽略這些，除非管理員的最高優先順序受管來源設定 `parentSettingsBehavior: 'merge'`，並且在 [`policyHelper`](/docs/zh-TW/settings-reference#policyhelper) 提供受管設定時永遠不會合併。合併的值會通過限制性篩選器；[限制父設定](/docs/zh-TW/claude-apps-gateway#restrict-parent-settings)涵蓋篩選器允許的內容和 `allowManaged*Only` 鎖定。設定 [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/zh-TW/env-vars) 的主機有三個金鑰直接從此承載讀取：其在 Claude Code v2.1.222 或更新版本上的[模型設定](/docs/zh-TW/model-config#restrict-model-selection)、當沒有受管來源在 v2.1.246 或更新版本上設定時的 [`modelPricing`](/docs/zh-TW/settings-reference#modelpricing)，以及其在 v2.1.247 或更新版本上的 `ENABLE_TOOL_SEARCH` env 項目                                                                                                                                       |
| `maxBudgetUsd`                    | `number`                                                                                                                                                                                                       | `undefined`                   | 當用戶端成本估計達到此美元值時停止查詢。僅計算呼叫自己的支出；從繼續工作階段還原的總計不計算。如需準確性注意事項和重設行為，請參閱[追蹤成本和使用量](/docs/zh-TW/agent-sdk/cost-tracking)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `maxThinkingTokens`               | `number`                                                                                                                                                                                                       | `undefined`                   | *已棄用：* 改用 `thinking`。思考程序的最大權杖數                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `maxTurns`                        | `number`                                                                                                                                                                                                       | `undefined`                   | 最大代理回合（工具使用往返）                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `mcpServers`                      | `Record<string, [`McpServerConfig`](#mcpserverconfig)>`                                                                                                                                                        | `{}`                          | MCP 伺服器設定                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `model`                           | `string`                                                                                                                                                                                                       | CLI 的預設值                      | Claude 模型別名或完整模型名稱。請參閱[接受的值和提供者特定 ID](/docs/zh-TW/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `onElicitation`                   | `(request: ElicitationRequest, options: { signal: AbortSignal }) => Promise<ElicitationResult>`                                                                                                                | `undefined`                   | 用於處理 MCP 引出請求的回呼。當 MCP 伺服器要求使用者輸入且沒有 hook 先處理時呼叫。未提供時，未處理的引出請求會自動被拒絕                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `outputFormat`                    | `{ type: 'json_schema', schema: JSONSchema }`                                                                                                                                                                  | `undefined`                   | 定義代理結果的輸出格式。請參閱[結構化輸出](/docs/zh-TW/agent-sdk/structured-outputs)以取得詳細資訊                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `outputStyle`                     | `string`                                                                                                                                                                                                       | `undefined`                   | 不是 `Options` 欄位。改為在內嵌 [`settings`](/docs/zh-TW/settings) 物件或設定檔中設定 `outputStyle`。請參閱[啟用輸出樣式](/docs/zh-TW/agent-sdk/modifying-system-prompts#activate-an-output-style)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `pathToClaudeCodeExecutable`      | `string`                                                                                                                                                                                                       | 從捆綁的原生二進位檔自動解析                | Claude Code 可執行檔的路徑。只有在安裝期間跳過選用相依性或您的平台不在支援的集合中時才需要                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `permissionMode`                  | [`PermissionMode`](#permissionmode)                                                                                                                                                                            | `'default'`                   | 工作階段的權限模式                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `permissionPromptToolName`        | `string`                                                                                                                                                                                                       | `undefined`                   | 權限提示的 MCP 工具名稱                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `permissionPrompts`               | `'host' \| 'none'`                                                                                                                                                                                             | `'host'`                      | 誰回答權限提示：`'host'` 將它們路由到您的 [`canUseTool`](#canusetool) 回呼或 `permissionPromptToolName` 工具，而 `'none'` [拒絕會提示的呼叫](/docs/zh-TW/agent-sdk/permissions#how-permissions-are-evaluated)。需要 Claude Code v2.1.259 或更新版本                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `persistSession`                  | `boolean`                                                                                                                                                                                                      | `true`                        | 當為 `false` 時，停用工作階段持久化到磁碟。工作階段之後無法繼續                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `planModeInstructions`            | `string`                                                                                                                                                                                                       | `undefined`                   | 計畫模式的自訂工作流程指示。當 `permissionMode` 為 `'plan'` 時，此字串會取代預設計畫模式工作流程主體。CLI 仍會使用唯讀強制前言和 ExitPlanMode 協定頁尾來包裝它                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `plugins`                         | [`SdkPluginConfig`](#sdkpluginconfig)`[]`                                                                                                                                                                      | `[]`                          | 從本機路徑載入自訂外掛程式。請參閱[外掛程式](/docs/zh-TW/agent-sdk/plugins)以取得詳細資訊                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `projectConfigRoot`               | `string`                                                                                                                                                                                                       | `undefined`                   | `cwd` 是其 worktree 的受信任簽出的絕對路徑。Claude Code 從此目錄而不是 `cwd` 讀取專案設定、`.mcp.json` 和專案的 `.claude/` 命令、代理、技能、工作流程、例行程序和輸出樣式，並將 `CLAUDE_PROJECT_DIR` 設定為它。Hooks、協助程式指令碼（例如 `apiKeyHelper`）和 stdio MCP 伺服器以此目錄作為其工作目錄啟動。`CLAUDE.md` 檔案和 `.claude/rules/` 仍從 `cwd` 載入。需要 Claude Code v2.1.275 或更新版本                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `promptSuggestions`               | `boolean`                                                                                                                                                                                                      | `false`                       | 啟用提示建議。在回合後，Claude Code 會發出 `prompt_suggestion` 訊息，其中包含預測的下一個使用者提示。Claude Code 不會為某些回合（例如當您的帳戶接近或達到使用量限制時）產生建議。請參閱[Claude Code 何時跳過建議](/docs/zh-TW/interactive-mode#when-claude-code-skips-suggestions)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `resume`                          | `string`                                                                                                                                                                                                       | `undefined`                   | 要繼續的工作階段 ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `resumeDropsTurn`                 | `string`                                                                                                                                                                                                       | `undefined`                   | 使用 `resumeSessionAt`：截斷繼續打算捨棄的回合的提示 UUID。當捨棄的範圍包含任何不可歸因於該回合的內容（例如吸收的佇列訊息或任務通知）時，Claude Code 會拒絕繼續，並在拒絕訊息中命名 `--resume-drops-turn` 旗標。只有 Agent SDK 和列印模式繼續讀取該對。需要 Claude Code v2.1.223 或更新版本                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `resumeSessionAt`                 | `string`                                                                                                                                                                                                       | `undefined`                   | 在特定訊息 UUID 處繼續工作階段                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `sandbox`                         | [`SandboxSettings`](#sandboxsettings)                                                                                                                                                                          | `undefined`                   | 以程式設計方式設定沙箱行為。請參閱[沙箱設定](#sandboxsettings)以取得詳細資訊                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `sessionId`                       | `string`                                                                                                                                                                                                       | 自動產生                          | 使用特定 UUID 作為工作階段，而不是自動產生一個                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `sessionStore`                    | [`SessionStore`](/docs/zh-TW/agent-sdk/session-storage#the-sessionstore-interface)                                                                                                                                  | `undefined`                   | 將工作階段文字記錄鏡像到外部後端，以便另一個主機可以繼續它們。請參閱[將工作階段持久化到外部儲存體](/docs/zh-TW/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `sessionStoreFlush`               | `'batched' \| 'eager'`                                                                                                                                                                                         | `'batched'`                   | *Alpha。* `sessionStore` 的排清模式。未設定 `sessionStore` 時忽略                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `settings`                        | `string \| Settings`                                                                                                                                                                                           | `undefined`                   | 內嵌[設定](/docs/zh-TW/settings)物件、設定檔路徑或內嵌 JSON 字串。在[優先順序順序](/docs/zh-TW/settings#settings-precedence)中填入旗標設定層。使用 [`applyFlagSettings()`](#applyflagsettings) 在執行時變更                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `settingSources`                  | [`SettingSource`](#settingsource)`[]`                                                                                                                                                                          | CLI 預設值（所有來源）                 | 控制要載入哪些檔案系統設定。傳遞 `[]` 以停用使用者、專案和本機設定。[端點管理的原則](/docs/zh-TW/managed-settings#delivery-mechanisms)無論如何都會載入；當工作階段使用組織認證在[合格設定](/docs/zh-TW/server-managed-settings#platform-availability)上進行驗證時，會擷取伺服器管理的設定。請參閱[使用 Claude Code 功能](/docs/zh-TW/agent-sdk/claude-code-features#what-settingsources-does-not-control)                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `skills`                          | `string[] \| 'all'`                                                                                                                                                                                            | `undefined`                   | 工作階段可用的技能。傳遞 `'all'` 以啟用每個發現的技能，或傳遞技能名稱清單。僅傳遞確切名稱。在 Agent SDK v0.3.221 或更新版本上，SDK 會在啟動 Claude Code 程序之前以錯誤拒絕格式不正確和萬用字元形式的名稱。設定時，SDK 會自動將 Skill 工具新增到 `allowedTools`。如果您也傳遞 `tools`，請在該清單中包含 `'Skill'`。請參閱[技能](/docs/zh-TW/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `spawnClaudeCodeProcess`          | `(options: SpawnOptions) => SpawnedProcess`                                                                                                                                                                    | `undefined`                   | 用於衍生 Claude Code 程序的自訂函式。用於在 VM、容器或遠端環境中執行 Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `stderr`                          | `(data: string) => void`                                                                                                                                                                                       | `undefined`                   | stderr 輸出的回呼                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `strictMcpConfig`                 | `boolean`                                                                                                                                                                                                      | `false`                       | 僅使用在 `mcpServers` 中傳遞的伺服器，並忽略專案 `.mcp.json`、使用者設定、外掛程式提供的 MCP 伺服器和 [claude.ai 連接器](/docs/zh-TW/mcp#use-mcp-servers-from-claude-ai)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `systemPrompt`                    | `string \| string[] \| { type: 'custom'; prompt: string \| string[]; snapshot?: boolean } \| { type: 'preset'; preset: 'claude_code'; append?: string; excludeDynamicSections?: boolean; snapshot?: boolean }` | `undefined`（最小提示）             | 系統提示設定。傳遞字串以取得自訂提示，或傳遞 `{ type: 'preset', preset: 'claude_code' }` 以使用 Claude Code 的系統提示。傳遞字串陣列，在靜態和每個請求部分之間使用匯出的 `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` 常數，以[快取自訂提示的靜態部分](/docs/zh-TW/agent-sdk/modifying-system-prompts#cache-the-static-part-of-a-custom-prompt)。使用預設物件形式時，新增 `append` 以使用其他指示進行擴充，並設定 `excludeDynamicSections: true` 以將每個工作階段內容移至第一個使用者訊息，以[更好地跨機器重複使用提示快取](/docs/zh-TW/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines)。設定 `snapshot: false` 以在每個請求上重建提示，而不是[重複使用工作階段在其第一個請求上記錄的提示](/docs/zh-TW/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session)。若要在自訂提示上設定 `snapshot`，請傳遞 `{ type: 'custom', prompt }` 形式。`{ type: 'custom' }` 形式和 `snapshot` 欄位需要 TypeScript Agent SDK v0.3.257 或更新版本 |
| `taskBudget`                      | `{ total: number }`                                                                                                                                                                                            | `undefined`                   | *Alpha。* API 端任務預算（權杖）。設定時，模型會被告知其剩餘權杖預算，以便它可以調整工具使用速度並在達到限制前完成                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `thinking`                        | [`ThinkingConfig`](#thinkingconfig)                                                                                                                                                                            | 支援的模型為 `{ type: 'adaptive' }` | 控制 Claude 的思考/推理行為。請參閱 [`ThinkingConfig`](#thinkingconfig) 以取得選項                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `title`                           | `string`                                                                                                                                                                                                       | `undefined`                   | 工作階段的顯示標題。使用 `resume` 或 `continue` 繼續時，繼續工作階段的持久化標題優先；使用 [`renameSession()`](#renamesession) 重新標題現有工作階段                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `toolAliases`                     | `Record<string, string>`                                                                                                                                                                                       | `undefined`                   | 將內建工具名稱對應到 MCP 工具名稱，以便 Claude 呼叫您的 MCP 實作而不是內建工具。例如，`{ Bash: 'mcp__workspace__bash' }`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `toolConfig`                      | [`ToolConfig`](#toolconfig)                                                                                                                                                                                    | `undefined`                   | 內建工具行為的設定。請參閱 [`ToolConfig`](#toolconfig) 以取得詳細資訊                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `tools`                           | `string[] \| { type: 'preset'; preset: 'claude_code' }`                                                                                                                                                        | `undefined`                   | 工具設定。傳遞工具名稱陣列或使用預設值以取得 Claude Code 的預設工具                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |

<h4 id="handle-slow-or-stalled-api-responses">
  處理緩慢或停滯的 API 回應
</h4>

CLI 子程序讀取多個環境變數，這些變數控制 API 逾時和停滯偵測。透過 `env` 選項傳遞它們：

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

* `API_TIMEOUT_MS`：Anthropic 用戶端上的每個請求逾時（毫秒）。預設 `600000`。適用於主迴圈和所有子代理。
* `CLAUDE_CODE_MAX_RETRIES`：最大 API 重試次數。預設 `10`，上限為 `15`。每次重試都有自己的 `API_TIMEOUT_MS` 視窗，因此最壞情況下的牆面時間大約是 `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` 加上退避。對於需要等待較長中斷的無人值守執行，設定 [`CLAUDE_CODE_RETRY_WATCHDOG=1`](/docs/zh-TW/errors#tune-retry-behavior)：它無限期重試暫時性容量錯誤，並且在 Claude Code v2.1.199 或更新版本上，將其他暫時性錯誤的預設值提高到 `300` 並移除此變數的上限。
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`：子代理的停滯監視程式。當串流監視程式開啟時，預設值為 `CLAUDE_STREAM_IDLE_TIMEOUT_MS` 加上 5 分鐘，總計 `600000`，除非您提高該變數。關閉串流監視程式時，預設值為 `600000`。在 v2.1.257 之前，預設值始終為 `600000`。

  計時器在每個串流事件上重設。停滯時，Claude Code 會中止子代理並向父代理報告停滯。對於背景子代理，它也會將任務標記為失敗並附加任何部分結果。
* `CLAUDE_ENABLE_STREAM_WATCHDOG` 搭配 `CLAUDE_STREAM_IDLE_TIMEOUT_MS`：串流監視程式，當標頭已到達但回應主體停止串流時中止請求。監視程式預設對所有提供者開啟；設定 `CLAUDE_ENABLE_STREAM_WATCHDOG=0` 以停用它。`CLAUDE_STREAM_IDLE_TIMEOUT_MS` 預設為 `300000` 並固定在該最小值。中止後，[自動重試](/docs/zh-TW/errors#automatic-retries)涵蓋 Claude Code 根據回應進度的程度所執行的操作。

  當監視程式等待 `ANTHROPIC_BASE_URL` 後面的閘道使用保持連線 ping 保持開啟的回應時，設定 `includePartialMessages` 的主機會繼續接收 `ping` [串流事件](#sdkpartialassistantmessage)，因此將這些框架讀取為活躍性而不是在沉默時逾時工作階段。在 v2.1.257 之前，框架在最後一個真實串流事件後 5 分鐘停止。

<h3 id="query-object">
  `Query` 物件
</h3>

`query()` 函式傳回的介面。

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
  方法
</h4>

| 方法                                     | 說明                                                                                                                                                                                                                                                                                                                   |
| :------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt()`                          | 中斷查詢。僅在串流輸入模式中可用。當 CLI 在 [`SDKSystemMessage.capabilities`](#sdksystemmessage) 中公告 `interrupt_receipt_v1` 功能時，使用列出中斷到達時待處理的訊息的 [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) 進行解析。在 v2.1.205 之前的 CLI 上解析 `undefined`                                                                                  |
| `rewindFiles(userMessageId, options?)` | 將檔案還原到指定使用者訊息時的狀態。傳遞 `{ dryRun: true }` 以預覽變更。需要 `enableFileCheckpointing: true`。請參閱[檔案檢查點](/docs/zh-TW/agent-sdk/file-checkpointing)                                                                                                                                                                                     |
| `setPermissionMode()`                  | 變更權限模式（僅在串流輸入模式中可用）                                                                                                                                                                                                                                                                                                  |
| `setModel()`                           | 變更模型（僅在串流輸入模式中可用）。傳遞 `undefined` 或字串 `"default"` 以重設為 [Claude Code 的預設模型](/docs/zh-TW/model-config)                                                                                                                                                                                                                       |
| `setMaxThinkingTokens()`               | *已棄用：* 改用 `thinking` 選項。變更最大思考權杖。傳遞 `null` 以將思考重設為工作階段預設值：清除中期工作階段覆蓋，並且對於已停用思考的工作階段，思考保持關閉                                                                                                                                                                                                                           |
| `applyFlagSettings(settings)`          | 在執行時將設定合併到工作階段的旗標設定層（僅在串流輸入模式中可用）。請參閱 [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                    |
| `updateSettings(source, settings)`     | 將一個允許清單金鑰寫入專案的本機設定檔或您的使用者設定檔，以便值在稍後的工作階段中持久化。請參閱 [`updateSettings()`](#updatesettings)。需要 TypeScript SDK v0.3.257 或更新版本，其捆綁 Claude Code v2.1.257                                                                                                                                                                     |
| `initializationResult()`               | 傳回完整初始化結果，包括支援的命令、模型、帳戶資訊和輸出樣式設定                                                                                                                                                                                                                                                                                     |
| `reinitialize()`                       | 重新傳送 `initialize` 控制請求到執行中的 CLI，並傳回新鮮結果而不是快取的首次連線結果。在傳輸間隙後使用它，例如在中斷後重新附加到工作階段，以便待處理的權限請求再次到達您的 `canUseTool` 回呼。使回呼對每個請求 ID 具有冪等性，因為其回應遺失的請求會再次分派。需要 Claude Code v2.1.195 或更新版本                                                                                                                                       |
| `supportedCommands()`                  | 傳回可用的命令。從 Agent SDK v0.3.216，清單反映中期工作階段命令變更；請參閱 [`SDKCommandsChangedMessage`](#sdkcommandschangedmessage)                                                                                                                                                                                                            |
| `supportedModels()`                    | 傳回具有顯示資訊的可用模型                                                                                                                                                                                                                                                                                                        |
| `supportedAgents()`                    | 傳回可用的子代理作為 [`AgentInfo`](#agentinfo)`[]`                                                                                                                                                                                                                                                                             |
| `mcpServerStatus()`                    | 傳回連線 MCP 伺服器的狀態作為 [`McpServerStatus`](#mcpserverstatus)`[]`                                                                                                                                                                                                                                                          |
| `getContextUsage(opts?)`               | 傳回 [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse)，按類別、技能和工具細分工作階段的內容視窗使用量。使用預設 `detail`，它與 `/context` 在互動式工作階段中顯示的資料相同。[`detail` 選項](#sdkcontrolgetcontextusageresponse)需要 Agent SDK v0.3.257 或更新版本                                                                                       |
| `readFile(path, options?)`             | 從工作階段的檔案系統讀取檔案。Claude Code 根據 `cwd` 解析路徑；[`readFile()` 可以讀取什麼](#what-readfile-can-read)列出它提供的檔案。傳遞 `{ maxBytes }` 以變更讀取上限（預設 1 MB，上限 10 MB）和 `{ encoding: 'base64' }` 以取得二進位檔案（例如影片）。使用 [`SDKControlReadFileResponse`](#sdkcontrolreadfileresponse) 進行解析，或在權限拒絕、遺失檔案或傳輸錯誤時使用 `null`。需要 TypeScript SDK v0.2.121 或更新版本 |
| `reloadSkills()`                       | 從磁碟重新載入技能，以便您在中期工作階段新增或編輯的技能可供執行中的工作階段使用。使用列出重新載入後可用技能的 [`SDKControlReloadSkillsResponse`](#sdkcontrolreloadskillsresponse) 進行解析。需要 Agent SDK v0.3.163 或更新版本                                                                                                                                                         |
| `accountInfo()`                        | 傳回帳戶資訊                                                                                                                                                                                                                                                                                                               |
| `reconnectMcpServer(serverName)`       | 按名稱重新連線 MCP 伺服器。如果名稱也符合設定檔（例如 `.mcp.json` 或 `~/.claude.json`）中的項目，Claude Code 會重新連線您透過 [`mcpServers`](#options) 或 `setMcpServers()` 設定的伺服器，而不是設定檔項目。該解析順序需要 Claude Code v2.1.257 或更新版本                                                                                                                               |
| `toggleMcpServer(serverName, enabled)` | 按名稱啟用或停用 MCP 伺服器，名稱解析與 `reconnectMcpServer()` 相同。停用會中斷伺服器連線                                                                                                                                                                                                                                                          |
| `setMcpServers(servers)`               | 動態取代此工作階段的 MCP 伺服器集合。使用命名已新增和移除的伺服器以及任何錯誤的 [`McpSetServersResult`](#mcpsetserversresult) 進行解析                                                                                                                                                                                                                        |
| `readMcpResource(serverName, uri)`     | *Alpha。* 從連線的 MCP 伺服器讀取一個 MCP Apps `ui://` 資源，以便您的應用程式可以呈現工具的小工具。使用 [`SDKControlMcpReadResourceResponse`](#sdkcontrolmcpreadresourceresponse) 進行解析。需要 TypeScript Agent SDK v0.3.280 或更新版本                                                                                                                            |
| `streamInput(stream)`                  | 將輸入訊息串流到查詢以進行多回合對話                                                                                                                                                                                                                                                                                                   |
| `stopTask(taskId)`                     | 按 ID 停止執行中的背景任務                                                                                                                                                                                                                                                                                                      |
| `close()`                              | 關閉查詢並終止基礎程序。強制結束查詢並清理所有資源                                                                                                                                                                                                                                                                                            |

<h4 id="applyflagsettings">
  `applyFlagSettings()`
</h4>

在執行中的工作階段上變更[設定](/docs/zh-TW/settings)，而不重新啟動查詢。當沒有專用設定器的設定需要在中期工作階段變更時使用它，例如在代理讀取不受信任的輸入後收緊 `permissions`。`setModel()` 和 `setPermissionMode()` 是這兩個金鑰的專用設定器；`applyFlagSettings()` 是接受任何設定金鑰子集的一般形式，在此處傳遞 `model` 的行為與 `setModel()` 相同。

只有某些金鑰在中期工作階段生效：

* **在下一回合套用**：`effortLevel`、`ultracode`、`permissions`、`hooks`、`skillOverrides`、`fastMode`、`agent`。切換 `agent` 也會在下一回合套用該代理的模型覆蓋和 hooks。其系統提示在下一回合套用，或在[重複使用記錄的系統提示](/docs/zh-TW/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session)的工作階段中，一旦工作階段被壓縮。
* **在目前回合期間套用**：`model`。如果您在 Claude 處理回合時切換 `model`，Claude 已在產生的回應會在舊模型上完成，回合的其餘部分（從 Claude Code 對模型進行的下一個呼叫開始）使用新模型。子代理保留自己的模型。在 v2.1.212 之前，中期切換會等待下一回合。
* **中期工作階段無效**：系統提示選項。這些在啟動時解析一次，因此執行中的工作階段保留原始值，即使呼叫成功。若要變更它們，請啟動新工作階段。

`effortLevel` 接受[努力等級](/docs/zh-TW/model-config#adjust-effort-level)名稱。它也接受 `"ultracode"`，這要求 `xhigh` 努力搭配 [ultracode](/docs/zh-TW/workflows#let-claude-decide-with-ultracode)。`applyFlagSettings()` 宣告 `effortLevel` 沒有該值，因此在 TypeScript 中傳遞等效的 `{ ultracode: true }`。`ultracode` 值需要 Claude Code v2.1.203 或更新版本，並且僅由 `applyFlagSettings()` 接受，不由設定檔中的 `effortLevel` 金鑰接受。

值會寫入旗標設定層，與內嵌 `query()` 的 `settings` 選項在啟動時填入的層相同。這與[在頁面優先順序部分](#settings-precedence)稱為程式設計選項的層級相同。

連續呼叫淺層合併頂層金鑰。第二個呼叫搭配 `{ permissions: {...} }` 會取代來自先前呼叫的整個 `permissions` 物件，而不是深層合併到其中。若要從旗標層清除金鑰，請為該金鑰傳遞 `null`。大多數金鑰隨後會回退到較低優先順序的來源。清除的 `model` 會重設為 [Claude Code 的預設模型](/docs/zh-TW/model-config)，即使設定檔設定 `model`。傳遞 `undefined` 沒有效果，因為 JSON 序列化會將其捨棄。

三個金鑰除了 `model` 外會重設工作階段狀態而不是回退：

* `effortLevel: null` 將工作階段返回到模型的預設努力等級，而不是 `query()` 的 `effort` 選項或設定檔中的 `effortLevel`。
* `agent: null` 從下一回合開始以沒有代理的方式執行主執行緒，而不是還原 `query()` 的 `agent` 選項或設定檔中的 `agent`。如果清除的代理已套用自己的模型，工作階段會返回到在啟動時解析的模型。
* `ultracode: null` 關閉 ultracode，如 `false` 一樣，而不是還原設定檔中的 `ultracode` 值。工作階段保留其目前的努力等級，因此在同一呼叫中傳遞 `effortLevel` 以變更它。

僅在串流輸入模式中可用，與 `setModel()` 和 `setPermissionMode()` 的約束相同。

下面的範例在中期工作階段切換作用中模型，然後清除覆蓋，以便模型重設為 [Claude Code 的預設模型](/docs/zh-TW/model-config)。

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const q = query({ prompt: messageStream });

// Override the model for the rest of the session
await q.applyFlagSettings({ model: "claude-opus-4-6" });

// Later: clear the override; the model resets to Claude Code's default
await q.applyFlagSettings({ model: null });
```

<Note>
  `applyFlagSettings()` 僅限 TypeScript。Python SDK 不公開等效方法。
</Note>

<h4 id="updatesettings">
  `updateSettings()`
</h4>

將一個允許清單金鑰寫入磁碟上的設定檔，以便值在稍後載入該來源的工作階段中持久化。每個來源接受一個金鑰，具有字串值：

* **`"localSettings"`**：接受 `outputStyle` 並將其合併到專案的本機設定檔 `.claude/settings.local.json`。新樣式在工作階段的下一個請求時生效。
* **`"userSettings"`**：接受 `effortLevel` 並將其儲存為工作階段目前模型的預設[努力等級](/docs/zh-TW/model-config#adjust-effort-level)，在您的使用者設定檔中的 [`modelSettings`](/docs/zh-TW/settings-reference#modelsettings) 下。傳遞 `max` 不會寫入任何內容，因為 `max` 僅限工作階段。執行中的工作階段無論如何都保留其目前的努力等級，因此當您也想變更該級別時呼叫 [`applyFlagSettings()`](#applyflagsettings)。此來源需要 TypeScript SDK v0.3.277 或更新版本，其捆綁 Claude Code v2.1.277。

當請求攜帶任何其他金鑰、工作階段在遠端傳輸上執行，以及當工作階段的 [`settingSources`](#options) 排除您命名的來源時，呼叫會拒絕。不支援刪除金鑰。

<h3 id="warmquery">
  `WarmQuery`
</h3>

由 [`startup()`](#startup) 傳回的控制代碼。子程序已衍生並初始化，因此在此控制代碼上呼叫 `query()` 會將提示直接寫入準備好的程序，沒有啟動延遲。

```typescript theme={null}
interface WarmQuery extends AsyncDisposable {
  query(prompt: string | AsyncIterable<SDKUserMessage>): Query;
  close(): void;
}
```

<h4 id="methods-2">
  方法
</h4>

| 方法              | 說明                                                            |
| :-------------- | :------------------------------------------------------------ |
| `query(prompt)` | 傳送提示到預熱的子程序並傳回 [`Query`](#query-object)。每個 `WarmQuery` 只能呼叫一次 |
| `close()`       | 關閉子程序而不傳送提示。使用此選項可捨棄不再需要的預熱查詢                                 |

`WarmQuery` 實作 `AsyncDisposable`，因此可以搭配 `await using` 使用以進行自動清理。

<h3 id="sdkcontrolinitializeresponse">
  `SDKControlInitializeResponse`
</h3>

`initializationResult()` 的傳回類型。包含工作階段初始化資料。

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

`hooks_applied` 報告 Claude Code 是否註冊了 `initialize` 請求所攜帶的 `hooks`。SDK 在工作階段啟動時傳送該請求一次，並在每個 [`reinitialize()`](#query-object) 呼叫上再次傳送。該欄位需要 Agent SDK v0.3.238 或更新版本。

當請求未攜帶 hooks 時，Claude Code 會省略該欄位。當請求攜帶 hooks 時，值取決於請求是否是工作階段的首次初始化，以及對於重複的請求，它如何到達工作階段：

* `true`：Claude Code 已註冊 hooks。工作階段的首次初始化傳回此值。透過 CLI 的 stdin 傳送的重複初始化也傳回 `true`。在這種情況下，新請求中的 hooks 會取代先前註冊的 hooks。
* `false`：Claude Code 已忽略 hooks。傳送到遠端工作階段的重複初始化傳回此值，因此加入工作階段的第二個使用者無法取代第一個使用者註冊的 hooks。

在 Agent SDK v0.3.238 之前，回應永遠不會攜帶該欄位，Claude Code 會在每次重複初始化時忽略 `hooks`。

回應始終報告 `fast_mode_state`，當某些東西阻止[快速模式](/docs/zh-TW/fast-mode)時，`fast_mode_disabled_reason` 會隨著原因代碼一起進行，以便您可以解釋阻止的狀態而不是重新衍生可用性。兩種行為都需要 Claude Code v2.1.219 或更新版本。在 v2.1.219 之前，當快速模式不可用時回應會省略 `fast_mode_state`，並且永遠不會攜帶原因。如需原因代碼及其含義，請參閱結果訊息上的 [`fast_mode_disabled_reason`](#sdkresultmessage)。

成功 `initialize` 的控制回應包裝器也攜帶 `pending_permission_requests` 陣列。該欄位在回應包裝器本身上，而不在上面的 `SDKControlInitializeResponse` 承載中。每個項目都是具有相同 `{ type: "control_request", request_id, request }` 形狀的完整 `control_request` 訊息，工作階段在執行時針對權限請求進行串流。

陣列列出此 Claude Code 程序已發出且尚未解決的權限請求。SDK 為您讀取陣列並將每個項目分派到您的 [`canUseTool`](#canusetool) 回呼，與 [`reinitialize()`](#query-object) 在傳輸間隙後觸發的相同重新傳遞。以冪等方式處理重複的請求 ID，因為項目可以重複回呼已接收的請求，然後連線中斷。

陣列在成功的 `initialize` 回應上始終存在，當此程序沒有未解決的權限請求時為空。需要 Claude Code v2.1.268 或更新版本。較早的版本可能會省略該欄位，因此如果您自行解析線路協定，請將遺失的欄位視為較舊的 CLI，而不是沒有待處理內容的證明。

<h3 id="sdkcontrolinterruptresponse">
  `SDKControlInterruptResponse`
</h3>

中斷收據：[`interrupt()`](#query-object) 在公告 [`SDKSystemMessage.capabilities`](#sdksystemmessage) 中的 `interrupt_receipt_v1` 功能的 CLI 上解析的值。需要 Claude Code v2.1.205 或更新版本。較早的 CLI 使用空成功承載回答中斷，因此 `interrupt()` 解析為 `undefined`。

```typescript theme={null}
type SDKControlInterruptResponse = {
  still_queued: string[];
  cancelled?: string[];
};
```

`still_queued` 列出中斷到達時待處理的使用者訊息的 UUID：仍在佇列中的訊息，加上 Claude Code 已從佇列中取出以進行下一回合的任何訊息。一旦工作階段的首次回合開始，Claude Code 會在中斷後處理列出的訊息，除非您先取消它們，並可以將多個訊息合併為一個回合。如果您在首次回合開始前中斷，Claude Code 會在回合開始時立即中止該回合，該回合中列出的訊息不會獲得回應。

使用收據來決定是否重新傳送任何內容。未取消的列出訊息會進入對話，無論是否獲得回應，因此重新傳送它會將其傳遞給 Claude 兩次。

使用這些注意事項解釋清單：

* 只有使用 UUID 加入佇列的訊息才會出現。空陣列並不意味著沒有其他內容會執行。
* 只列出主執行緒訊息。定址到子代理的訊息超出範圍。
* 清單可以包含您的用戶端從未傳送的 UUID，例如[排定的任務](/docs/zh-TW/scheduled-tasks)觸發器。忽略您不認識的 UUID，而不是將其視為錯誤。

直接驅動 CLI 控制協定而不是透過 `interrupt()` 的用戶端可以在 `interrupt` 控制請求上設定 `cancel_queued: true`。Claude Code v2.1.219 及更新版本在 [`SDKSystemMessage.capabilities`](#sdksystemmessage) 中公告支援 `interrupt_cancel_queued_v1` 功能；較舊的 CLI 會忽略該欄位並讓佇列訊息照常執行。這樣的中斷也會取消每個原本會列在 `still_queued` 下的訊息：收據會改為在 `cancelled` 下列出它們，`still_queued` 為空，它們都不會執行。

`cancelled` 清單與 `still_queued` 具有相同的注意事項。`interrupt()` 方法永遠不會傳送 `cancel_queued`，所以它解析的收據不會攜帶 `cancelled`。

收據是在處理中斷時拍攝的快照，在乾淨中斷時，它在中斷回合的 [`SDKResultMessage`](#sdkresultmessage) 之前到達。讀取收據而不是在該結果後檢查佇列：迴圈立即啟動下一個佇列回合，因此您在結果後檢查的佇列已經變更。

<h3 id="sdkcontrolgetcontextusageresponse">
  `SDKControlGetContextUsageResponse`
</h3>

[`getContextUsage()`](#query-object) 的傳回類型。使用預設 `detail`，這是 Claude Code 在互動式工作階段中為 `/context` 命令呈現的相同承載，因此除了權杖計數外，它還攜帶顯示欄位（例如 `color` 和 `gridRows`），Claude Code 使用這些欄位來繪製 `/context` 使用量網格。

方法的選用 `detail` 引數選擇 Claude Code 如何計算每個類別。使用預設值 `'full'`，Claude Code 使用權杖計數 API 請求計算每個類別。傳遞 `{ detail: 'summary' }` 以從最後一個回應的使用量和本機估計中取得答案。沒有權杖計數請求外出，每個類別的數字是近似值。`detail` 引數需要 Agent SDK v0.3.257 或更新版本。

當您傳送 `/context` 作為提示而不是呼叫方法時，Claude Code 會將 [`SDKContextUsage`](#sdkcontextusage) 承載附加到傳遞結果的助手訊息的 `context_usage` 欄位。該欄位需要 Agent SDK v0.3.232 或更新版本。

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

從集合欄位讀取權杖歸因：

* `categories` 保留每個類別的總計。
* `mcpTools` 和 `agents` 將權杖歸因於個別 MCP 工具和子代理。
* `memoryFiles` 列出每個載入的記憶體檔案及其成本。
* `skills.skillFrontmatter` 將技能清單的權杖歸因於每個包含的技能。每個技能的計數測量每個技能的清單項目，因為 Claude Code 實際傳送它，這可能比技能的完整 frontmatter 更短。比較 `skills.totalSkills` 與 `skills.includedSkills` 以查看每個發現的技能是否進入清單。

`totalTokens` 是工作階段的目前內容使用量，`maxTokens` 是針對該使用量測量的視窗。該視窗是模型的內容視窗，或應用自動壓縮視窗時的較低自動壓縮視窗。`rawMaxTokens` 攜帶與 `maxTokens` 相同的值，`percentage` 是 `totalTokens` 作為該視窗的四捨五入百分比。

Claude Code 保留選用的 `deferredBuiltinTools`、`systemTools` 和 `systemPromptSections` 診斷未設定，因此即使類型宣告它們，也應該預期它們不存在。

<h3 id="sdkcontrolreadfileresponse">
  `SDKControlReadFileResponse`
</h3>

[`readFile()`](#query-object) 的傳回類型。

```typescript theme={null}
type SDKControlReadFileResponse = {
  contents: string;
  absPath: string;
  truncated?: boolean;
  encoding?: 'base64';
};
```

`contents` 保留檔案文字，或當您要求 `encoding: 'base64'` 時的 base64 資料；回應的 `encoding` 欄位在該情況下設定為 `'base64'`。`absPath` 是解析的絕對路徑。`truncated` 在檔案長於 `maxBytes` 上限且內容在該限制處被切割時設定。

<h4 id="what-readfile-can-read">
  `readFile()` 可以讀取什麼
</h4>

`readFile()` 提供的檔案集合比 Read 工具更窄：

* 工作階段的工作目錄之一（例如 `cwd` 和 `additionalDirectories`）內的一般檔案
* Claude Code 自己的一些檔案用於工作階段，例如工具結果

Read 拒絕和詢問規則仍會阻止符合的路徑，廣泛的 Read 允許規則不會將檔案系統的其餘部分開啟給 `readFile()`。對於任何其他內容，呼叫會使用 `null` 進行解析。

<h3 id="sdkcontrolreloadskillsresponse">
  `SDKControlReloadSkillsResponse`
</h3>

[`reloadSkills()`](#query-object) 的傳回類型。

```typescript theme={null}
type SDKControlReloadSkillsResponse = {
  skills: SlashCommand[];
};
```

`skills` 列出重新載入後可用的技能，採用 `supportedCommands()` 傳回的相同 [`SlashCommand`](#slashcommand) 形狀。

<h3 id="sdkcontrolmcpreadresourceresponse">
  `SDKControlMcpReadResourceResponse`
</h3>

[`readMcpResource()`](#query-object) 的傳回類型，攜帶 MCP 伺服器的 `resources/read` 結果。需要 TypeScript Agent SDK v0.3.280 或更新版本。

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

將伺服器名稱作為 `mcpServerStatus()` 報告的名稱和 `ui://` URI（例如工具在其 [`_meta`](#mcpserverstatus) 中宣告的 `ui.resourceUri`）傳遞給 `readMcpResource()`。呼叫會針對任何其他 URI 配置、您的應用程式自己託管的 [SDK MCP 伺服器](#createsdkmcpserver) 以及未連線的伺服器而拒絕。當初始化訊息的 [`capabilities`](#sdksystemmessage) 包含 `mcp_read_resource_v1` 時可用。

每個 `contents` 項目都是伺服器傳送的一個內容項目。`blob` 為二進位項目保留 base64 資料，`_meta` 是項目自己的 `_meta`，其中 MCP Apps 伺服器放置資源的 `ui.csp` 和 `ui.permissions`。內容是不受信任的第三方 HTML，因此在沙箱中呈現。

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

以程式設計方式定義的子代理的設定。

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

| 欄位                                    | 必要 | 說明                                                                                                                                                                 |
| :------------------------------------ | :- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`                         | 是  | 何時使用此代理的自然語言說明                                                                                                                                                     |
| `tools`                               | 否  | 允許的工具名稱陣列。如果省略，繼承[子代理可用的每個工具](/docs/zh-TW/sub-agents#available-tools)。若要將技能預先載入到代理的內容中，請使用 `skills` 欄位而不是在此列出 `'Skill'`                                                 |
| `disallowedTools`                     | 否  | 要明確禁止此代理的工具名稱陣列。也接受 MCP 伺服器層級模式：`mcp__server` 或 `mcp__server__*` 移除該伺服器的每個工具，`mcp__*` 移除任何伺服器的每個 MCP 工具                                                            |
| `prompt`                              | 是  | 代理的系統提示                                                                                                                                                            |
| `model`                               | 否  | 此代理的模型覆蓋。接受別名（例如 `'fable'`、`'opus'`、`'sonnet'`、`'haiku'`、`'inherit'`）或完整模型 ID。`'inherit'` 使用主模型。省略時，Claude Code 會在[子代理模型順序](/docs/zh-TW/sub-agents#choose-a-model)中選擇模型 |
| `mcpServers`                          | 否  | 此代理的 MCP 伺服器規格                                                                                                                                                     |
| `skills`                              | 否  | 要預先載入到代理內容中的技能名稱陣列                                                                                                                                                 |
| `initialPrompt`                       | 否  | 當此代理作為主執行緒代理執行時自動提交為首次使用者回合                                                                                                                                        |
| `maxTurns`                            | 否  | 代理回合（API 往返）的最大數量，然後停止                                                                                                                                             |
| `background`                          | 否  | 當叫用時以非阻止背景任務執行此代理                                                                                                                                                  |
| `omitClaudeMd`                        | 否  | 當此代理作為子代理執行時，在沒有使用者、專案和本機 CLAUDE.md 檔案的情況下執行此代理；受管原則檔案仍會載入。將其用於代理，這些代理從 Agent 工具提示中獲取所需的一切。當此代理作為主執行緒代理執行時忽略。需要 TypeScript Agent SDK v0.3.271 或更新版本                |
| `memory`                              | 否  | 此代理的記憶體來源：`'user'`、`'project'` 或 `'local'`                                                                                                                         |
| `effort`                              | 否  | 此代理的推理努力等級。接受命名等級或整數                                                                                                                                               |
| `permissionMode`                      | 否  | 此代理內工具執行的權限模式。[子代理繼承規則](/docs/zh-TW/agent-sdk/permissions#available-modes)決定何時適用。請參閱 [`PermissionMode`](#permissionmode)                                                |
| `criticalSystemReminder_EXPERIMENTAL` | 否  | 實驗性：新增到系統提示的關鍵提醒                                                                                                                                                   |

<h3 id="agentmcpserverspec">
  `AgentMcpServerSpec`
</h3>

指定子代理可用的 MCP 伺服器。可以是伺服器名稱（字串參考父代理 `mcpServers` 設定中的伺服器）或內嵌伺服器設定記錄，將伺服器名稱對應到設定。

```typescript theme={null}
type AgentMcpServerSpec = string | Record<string, McpServerConfigForProcessTransport>;
```

其中 `McpServerConfigForProcessTransport` 是 `McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig`。

<h3 id="settingsource">
  `SettingSource`
</h3>

控制 SDK 從哪些檔案系統設定來源載入設定。

```typescript theme={null}
type SettingSource = "user" | "project" | "local";
```

| 值           | 說明                                         | 位置                            |
| :---------- | :----------------------------------------- | :---------------------------- |
| `'user'`    | 全域使用者設定                                    | `~/.claude/settings.json`     |
| `'project'` | 共享專案設定（版本控制）                               | `.claude/settings.json`       |
| `'local'`   | 本機專案設定，當 Claude Code 將設定儲存到其中時被 gitignored | `.claude/settings.local.json` |

<h4 id="default-behavior">
  預設行為
</h4>

當 `settingSources` 被省略或 `undefined` 時，`query()` 載入與 Claude Code CLI 相同的檔案系統設定：使用者、專案和本機。請參閱[`settingSources` 不控制什麼](/docs/zh-TW/agent-sdk/claude-code-features#what-settingsources-does-not-control)以了解無論此選項如何都會讀取的輸入，以及如何停用它們。

<h4 id="why-use-settingsources">
  為什麼使用 settingSources
</h4>

**停用檔案系統設定：**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Do not load user, project, or local settings from disk
const result = query({
  prompt: "Analyze this code",
  options: { settingSources: [] }
});
```

**僅載入特定設定來源：**

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

若要載入 CLAUDE.md 專案指示，請在 `settingSources` 中包含 `"project"`。請參閱[修改系統提示](/docs/zh-TW/agent-sdk/modifying-system-prompts#claude-md-files-for-project-level-instructions)以了解 CLAUDE.md 載入如何與系統提示選項互動。

<h4 id="settings-precedence">
  設定優先順序
</h4>

載入多個來源時，設定會與此優先順序合併（最高到最低）：

1. 本機設定（`.claude/settings.local.json`）
2. 專案設定（`.claude/settings.json`）
3. 使用者設定（`~/.claude/settings.json`）

程式設計選項（例如 `agents`、`allowedTools` 和 `settings`）覆蓋使用者、專案和本機檔案系統設定。受管原則設定優先於程式設計選項。

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

用於控制工具使用的自訂權限函式類型。

該函式是互動式權限提示的 SDK 替代品：僅當[權限評估流程](/docs/zh-TW/agent-sdk/permissions#how-permissions-are-evaluated)解析為提示時才叫用。由 `allowedTools` 項目、設定允許規則或權限模式（例如 `acceptEdits` 或 `bypassPermissions`）預先核准的工具呼叫永遠不會叫用它。若要限制每個工具呼叫，請改用 [`PreToolUse` hook](/docs/zh-TW/agent-sdk/hooks)。

允許規則不會預先核准[任何模式都不自動核准的動作](/docs/zh-TW/permission-modes#actions-no-mode-auto-approves)；請參閱[權限如何評估](/docs/zh-TW/agent-sdk/permissions#how-permissions-are-evaluated)以了解其中哪些到達回呼，以及在 `dontAsk` 和 `auto` 模式中發生的情況。

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

| 選項               | 類型                                          | 說明                                                                                                                                                                        |
| :--------------- | :------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `signal`         | `AbortSignal`                               | 如果應該中止操作，則發出信號                                                                                                                                                            |
| `suggestions`    | [`PermissionUpdate`](#permissionupdate)`[]` | 建議的權限更新，以便不會再次提示使用者使用此工具。Bash 提示包括具有 `localSettings` [目的地](#permissionupdatedestination)的建議，因此在 `updatedPermissions` 中傳回它會將規則寫入 `.claude/settings.local.json` 並在工作階段間持久化。 |
| `blockedPath`    | `string`                                    | 觸發權限請求的檔案路徑（如果適用）                                                                                                                                                         |
| `mcpServer`      | `{ name: string; source: string }`          | 對於 `mcp__*` 工具，提供該工具的 MCP 伺服器及其定義來自何處，具有 [`McpServerProvenance`](#mcpserverprovenance) 的欄位。對於其他工具不存在。需要 Agent SDK v0.3.274 或更新版本                                          |
| `decisionReason` | `string`                                    | 解釋為什麼觸發此權限請求                                                                                                                                                              |
| `toolUseID`      | `string`                                    | 助手訊息內此特定工具呼叫的唯一識別碼                                                                                                                                                        |
| `agentID`        | `string`                                    | 如果在子代理內執行，子代理的 ID                                                                                                                                                         |
| `requestId`      | `string`                                    | `control_request` 信封的 `request_id`。您的應用程式在其自己的通道上傳送的 `control_response`（例如簽署的 HTTP POST）必須回應此值，以便 Claude Code 程序可以將回覆與請求相符                                                |

回呼通常透過傳回 [`PermissionResult`](#permissionresult) 來解決請求，SDK 會將其寫回其傳輸作為 `control_response`。僅當您的應用程式已透過其自己的通道為此請求傳送 `control_response`（回應 `requestId`）時才傳回 `null`；SDK 隨後會跳過將回應寫入其傳輸。在任何其他情況下傳回 `null` 會使工具呼叫無限期被阻止，因為永遠不會傳送 `control_response` 且權限提示不會逾時。

`requestId` 選項和 `null` 傳回值需要 Claude Code v2.1.199 或更新版本。

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

內建工具行為的設定。

```typescript theme={null}
type ToolConfig = {
  askUserQuestion?: {
    previewFormat?: "markdown" | "html";
  };
};
```

| 欄位                              | 類型                     | 說明                                                                                                                 |
| :------------------------------ | :--------------------- | :----------------------------------------------------------------------------------------------------------------- |
| `askUserQuestion.previewFormat` | `'markdown' \| 'html'` | 選擇加入 [`AskUserQuestion`](/docs/zh-TW/agent-sdk/user-input#question-format) 選項上的 `preview` 欄位並設定其內容格式。未設定時，Claude 不會發出預覽 |

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

MCP 伺服器的設定。

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

在 SDK 中載入外掛程式的設定。

```typescript theme={null}
type SdkPluginConfig = {
  type: "local";
  path: string;
  skipMcpDiscovery?: boolean;
};
```

| 欄位                 | 類型        | 說明                                                                                                      |
| :----------------- | :-------- | :------------------------------------------------------------------------------------------------------ |
| `type`             | `'local'` | 必須是 `'local'`（目前僅支援本機外掛程式）                                                                              |
| `path`             | `string`  | 外掛程式目錄的絕對或相對路徑                                                                                          |
| `skipMcpDiscovery` | `boolean` | 當為 `true` 時，SDK 從此外掛程式載入技能、hooks、代理和命令，但不讀取其 `.mcp.json` 或資訊清單 `mcpServers`。當您的應用程式擁有外掛程式的 MCP 連線時設定此項。 |

**範例：**

```typescript theme={null}
plugins: [
  { type: "local", path: "./my-plugin" },
  { type: "local", path: "/absolute/path/to/plugin" }
];
```

如需建立和使用外掛程式的完整資訊，請參閱[外掛程式](/docs/zh-TW/agent-sdk/plugins)。

<h2 id="message-types">
  訊息類型
</h2>

<h3 id="sdkmessage">
  `SDKMessage`
</h3>

查詢傳回的所有可能訊息的聯合類型。

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

助手回應訊息。

```typescript theme={null}
type SDKAssistantMessage = {
  type: "assistant";
  uuid: UUID;
  session_id: string;
  message: BetaMessage; // From Anthropic SDK
  parent_tool_use_id: string | null;
  error?: SDKAssistantMessageError;
  aborted?: true;
  timestamp?: string;
  context_usage?: SDKContextUsage;
  user_message_uuid?: string;
  user_message_uuids?: string[];
};
```

`message` 欄位是來自 Anthropic SDK 的 [`BetaMessage`](https://platform.claude.com/docs/en/api/messages/create)。它包含 `id`、`content`、`model`、`stop_reason` 和 `usage` 等欄位。

`SDKAssistantMessageError` 是以下其中之一：`'authentication_failed'`、`'oauth_org_not_allowed'`、`'account_on_hold'`、`'billing_error'`、`'rate_limit'`、`'overloaded'`、`'invalid_request'`、`'model_not_found'`、`'server_error'`、`'max_output_tokens'`、`'cloud_credential_error'` 或 `'unknown'`。其中四個值的含義超出其名稱所表示的內容：

* `'model_not_found'`：選定的模型不存在或無法供您的帳戶或部署使用
* `'overloaded'`：API 傳回 529，因為伺服器已滿載，與 `'rate_limit'` 相對，後者是針對您配額的 429
* `'account_on_hold'`：[您的帳戶已被暫停](/docs/zh-TW/errors#your-account-is-on-hold)
* `'cloud_credential_error'`：Claude Code 無法在其執行的機器上取得可用的 AWS 或 Google Cloud 認證，因此沒有請求到達雲端提供者。通常的原因是雲端登入已過期或從未在該機器上完成，但暫時無法連線的認證服務會報告相同的值。請參閱[無法載入 AWS 或 Google Cloud 認證](/docs/zh-TW/errors#could-not-load-aws-or-google-cloud-credentials)。需要 TypeScript Agent SDK v0.3.267 或更新版本，其中包含 Claude Code v2.1.267

當中斷或中止在串流完成前截斷助手訊息時，`aborted` 為 `true`：訊息沒有 `stop_reason`，內容可能在中間詞處結束。該欄位在正常完成的訊息上不存在。它需要 Agent SDK v0.3.214 或更新版本。

Claude Code 在轉換的第一個助手訊息上設定 `user_message_uuid` 和 `user_message_uuids`，條件在 [`user_message_uuid`](#user_message_uuid) 中。

`timestamp` 是訊息內容在產生它的程序上完成生成的 ISO 8601 時間。該值來自該機器的時鐘，因此僅用於顯示，不要按其排序訊息。一個 API 轉換可以產生多個共享 `message.id` 的助手訊息，每個都有自己的 `timestamp`。當欄位不存在時，回退到您收到訊息的時間。

`context_usage` 是 `/context` 報告的結構化副本，類型為 [`SDKContextUsage`](#sdkcontextusage)，需要 Agent SDK v0.3.232 或更新版本。當您以提示形式傳送 `/context` 時，Claude Code 會將報告作為助手訊息傳遞，其 `message.content` 包含 markdown 表格，並將 `context_usage` 附加到同一訊息。Claude Code 不會在任何其他助手訊息上設定該欄位，較早的版本會傳遞 `/context` 表格而不設定它，因此當欄位存在時從欄位讀取明細，當不存在時回退到 markdown 文字。

<h3 id="sdkusermessage">
  `SDKUserMessage`
</h3>

使用者輸入訊息。

```typescript theme={null}
type SDKUserMessage = {
  type: "user";
  uuid?: UUID;
  session_id?: string;
  message: MessageParam; // From Anthropic SDK
  pasted_content?: MessageParam["content"][];
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  shouldQuery?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
  inline_pastes?: string[];
};
```

設定 `pasted_content` 以傳送使用者貼上到您的提示 UI 中而不是輸入的內容，每個貼上一個項目，每個都是字串或內容區塊陣列。Claude Code 按順序在輸入的文字後附加每個項目的文字，並可能將每個貼上內容包裝在 `<pasted_content>` 標籤中。除文字外的區塊會被忽略，因此在 `message.content` 中傳送影像和文件。需要 Agent SDK v0.3.277 或更新版本。

設定 `shouldQuery` 為 `false` 以將訊息附加到文字記錄而不觸發助手轉換。訊息被保留並合併到下一個觸發轉換的使用者訊息中。使用此方法注入內容，例如您在帶外執行的命令的輸出，而不在模型呼叫上花費。

在攜帶 `tool_result` 區塊的訊息上，`tool_use_result` 是工具的結構化輸出物件，而不是傳送給模型的文字。其形狀取決於匹配 `tool_use` 區塊命名的工具，因此欄位的類型為 `unknown`；內建形狀列在[工具輸出類型](#tool-output-types)下。

對於 `Agent` 工具，`tool_use_result` 是 [`AgentOutput`](#agent-2)。在 `completed` 結果上，`content` 包含子代理的報告，不包含 Claude Code 附加到 `tool_result` 文字的代理 ID 和使用情況預告片，因此從 `tool_use_result` 呈現而不是解析該文字。

對於其結果包含 `resource_link` 區塊的 MCP 工具，`tool_use_result` 是一個物件，其中包含 [`SDKMcpResourceLink`](#sdkmcpresourcelink) 項目的 `resourceLinks` 陣列。Claude 將每個連結作為 `tool_result` 區塊中的一行文字接收，因此讀取 `resourceLinks` 以呈現伺服器傳回的檔案，而不是解析該文字。Claude Code 在結果沒有連結時省略 `resourceLinks`，在來自子代理的結果上省略，每個結果最多保留 50 個連結，並在陣列達到 64 KiB 序列化 JSON 後停止新增連結。`resourceLinks` 需要 Agent SDK v0.3.257 或更新版本。

設定 `inline_pastes` 以告訴 Claude Code `message.content` 的哪些部分使用者貼上而不是輸入，每個貼上一個字串。提示文字保留在使用者放置的位置。Claude Code 可能會在其所在位置將每個列出的貼上內容包裝在 `<pasted_content>` 標籤中，以便 Claude 可以區分貼上的材料與使用者自己的話語。只有提示最後一個文字區塊中的貼上內容會被包裝。需要 TypeScript Agent SDK v0.3.280 或更新版本。

<h3 id="sdkusermessagereplay">
  `SDKUserMessageReplay`
</h3>

具有必需 UUID 的重播使用者訊息。

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

從工作階段外部注入的使用者轉換，其 [`origin`](#sdkmessageorigin) 類型為 `peer` 或 `channel` 的轉換，無論是在活躍轉換期間傳遞還是在工作階段閒置時啟動新轉換，都會作為重播到達串流。在 v2.1.207 之前，在工作階段閒置時傳遞的注入轉換在串流上不產生訊息，只在您重新讀取文字記錄時出現。

<h3 id="sdkresultmessage">
  `SDKResultMessage`
</h3>

最終結果訊息。

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

結果上的多個欄位除了 `subtype` 之外還提供診斷詳細資訊：

* `api_error_status`：終止對話的 API 錯誤的 HTTP 狀態碼。當轉換在沒有 API 錯誤的情況下結束時不存在或為 `null`。
* `ttft_ms`：首個令牌的時間（毫秒），在第一個完整助手訊息到達時測量。僅在成功分支上存在。
* `ttft_stream_ms`：直到第一個 `message_start` 串流事件（當回應串流開啟時）的時間（毫秒）。低於 `ttft_ms`；兩者之間的差距是串流第一個訊息所花費的時間。僅在成功分支上存在。
* `user_message_uuid`：您傳送的訊息的 `uuid`，此轉換回答了該訊息。請參閱 [`user_message_uuid`](#user_message_uuid) 以了解哪些結果攜帶它。
* `user_message_uuids`：您傳送的每個訊息的 `uuid`，Claude Code 在此轉換中回答了這些訊息。請參閱 [`user_message_uuids`](#user_message_uuids)。
* `request_sent_wall_ms`：Claude Code 分派 API 請求的紀元毫秒，用於與伺服器端時間戳記的連接。僅與 [`user_message_uuid`](#user_message_uuid) 一起存在，在成功結果上，其中 `is_error` 為 false，其轉換傳送了 API 請求。
* `first_content_frame_ms`：直到第一個 `content_block_start` 或 `content_block_delta` 串流事件的時間（毫秒），將思考區塊計為內容。僅在成功分支上存在，當 `is_error` 為 false 時。需要 Agent SDK v0.3.260 或更新版本。
* `first_stream_post_ms`、`first_stream_post_ack_ms`、`first_stream_post_wall_ms`：上傳轉換第一個串流事件的計時。Claude Code 僅在它串流到 claude.ai 的工作階段中記錄它們，例如[雲端工作階段](/docs/zh-TW/claude-code-on-the-web)，`query()` 產生的結果不攜帶它們。需要 Agent SDK v0.3.260 或更新版本。
* `usage`：僅限主代理迴圈。排除子代理和輔助模型呼叫，在串流輸入工作階段中按轉換計算。優先使用 `modelUsage` 進行令牌/成本會計。
* `modelUsage`：在此 `query()` 呼叫期間通過查詢管道進行的每個模型呼叫的每個模型總計，包括主迴圈、子代理和內部呼叫，例如壓縮和 Workflow 代理。該管道外的輔助呼叫，例如權限分類器和令牌計數請求，被排除。恢復工作階段的呼叫也計算[從工作階段較早呼叫恢復的每個模型總計](/docs/zh-TW/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls)。在串流輸入工作階段中，總計在轉換中是累積的，因此讀取最新結果而不是跨結果求和。請參閱[在串流輸入模式中追蹤成本](/docs/zh-TW/agent-sdk/cost-tracking#track-costs-in-streaming-input-mode)以了解重設，以及[在工作階段崩潰後恢復總計](/docs/zh-TW/agent-sdk/cost-tracking#recover-totals-after-a-session-crash)以了解歸零結果。
* `total_cost_usd`：累積估計成本（美元），涵蓋與 `modelUsage` 相同的呼叫並在相同點重設。恢復工作階段的呼叫也計算[從工作階段較早呼叫恢復的總計](/docs/zh-TW/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls)。這是估計值，不是帳單聲明。請參閱[追蹤成本和使用情況](/docs/zh-TW/agent-sdk/cost-tracking)以了解準確性注意事項。
* `queued_turn_count`：您傳送的帶有 `origin: { kind: "human" }` 的訊息數量，在 Claude Code 產生結果時仍在等待。請參閱 [`queued_turn_count`](#queued_turn_count) 以了解 `0` 和不存在的欄位告訴您什麼。
* `startup_failure_reason`：Claude Code 拒絕啟動的原因，在它在已知啟動失敗時寫入的 `error_during_execution` 結果上。請參閱 [`startup_failure_reason`](#startup_failure_reason) 以了解值以及哪些失敗攜帶它。需要 Agent SDK v0.3.274 或更新版本。
* `terminal_reason`：迴圈結束的原因。`"completed"`、`"max_turns"`、`"tool_deferred"`、`"aborted_streaming"`、`"aborted_tools"`、`"hook_stopped"`、`"stop_hook_prevented"`、`"background_requested"`、`"blocking_limit"`、`"rapid_refill_breaker"`、`"prompt_too_long"`、`"image_error"`、`"model_error"`、`"api_error"`、`"malformed_tool_use_exhausted"`、`"budget_exhausted"`、`"structured_output_retry_exhausted"`、`"tool_deferred_unavailable"` 或 `"turn_setup_failed"` 之一。
* `fast_mode_state`：`"on"`、`"off"` 或 `"cooldown"` 之一。
* `fast_mode_disabled_reason`：為什麼[快速模式](/docs/zh-TW/fast-mode)現在不可用。當沒有任何東西阻止快速模式時不存在，儘管請求仍可能以標準速度執行。在快速模式速率限制後的冷卻期間，Claude Code 報告 `fast_mode_state: "cooldown"` 而沒有原因代碼，並在冷卻期過期時重新啟用快速模式。需要 Claude Code v2.1.219 或更新版本。

使用原因代碼在您自己的 UI 中解釋為什麼快速模式關閉，而不是重新推導可用性。每個代碼命名阻止快速模式的檢查：

| 原因代碼                   | 含義                                                                                                             |
| ---------------------- | -------------------------------------------------------------------------------------------------------------- |
| `free`                 | 帳戶沒有快速模式所需的付費訂閱或使用額度                                                                                           |
| `preference`           | 組織已禁用快速模式                                                                                                      |
| `extra_usage_disabled` | 帳戶已關閉使用額度                                                                                                      |
| `network_error`        | [可用性檢查](/docs/zh-TW/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways)無法連線到 `api.anthropic.com`               |
| `unknown`              | Claude Code 無法確定可用性                                                                                            |
| `not_first_party`      | 工作階段使用 Anthropic API 以外的提供者                                                                                    |
| `disabled_by_env`      | [`CLAUDE_CODE_DISABLE_FAST_MODE`](/docs/zh-TW/env-vars) 已設定                                                         |
| `model_not_allowed`    | 快速模式 Opus 模型不在組織的 [`availableModels`](/docs/zh-TW/model-config#restrict-model-selection) 允許清單中                      |
| `sdk_opt_in_required`  | 工作階段尚未選擇加入快速模式：在 [`settings`](#options) 選項中或通過 [`applyFlagSettings()`](#applyflagsettings) 傳遞 `fastMode: true` |
| `pending`              | 可用性檢查尚未完成                                                                                                      |

相同的欄位對出現在 [`SDKSystemMessage`](#sdksystemmessage) 和 [`SDKControlInitializeResponse`](#sdkcontrolinitializeresponse) 上，因此您可以在第一個轉換之前讀取快速模式狀態。

`origin` 欄位轉發觸發此結果的使用者訊息的 [`SDKMessageOrigin`](#sdkmessageorigin)。當 SDK 注入合成後續轉換（例如針對完成的背景任務）時，生成的 `SDKResultMessage` 攜帶 `origin: { kind: "task-notification" }`。其觸發器觸發且伺服器驗證的例程和來自您其他工作階段的訊息也會到達此類型，每個都帶有[任務通知子類型](#task-notification-subkinds)中描述的 `subkind`。檢查 `kind` 以區分回答您提示的結果與注入的後續內容，然後在路由或抑制它們之前進行區分。如果您的應用程式[宣告排定的執行](#declare-a-scheduled-run)，其結果也攜帶 `kind: "task-notification"`，因此不要僅在 `kind` 上抑制。

當多個背景任務完成一起排隊時，Claude Code 可以在一個轉換中回答它們，而不是每個一個轉換。每個完成仍會產生自己的結果與此來源。Claude Code 一起回答的完成中除最後一個外的所有完成都會按順序產生帶有 `num_turns: 0` 的空結果，最後一個的結果攜帶回答它們全部的轉換。

該欄位對於任何使用者轉換之前發出的結果（例如啟動錯誤）不存在。

當 `PreToolUse` 鉤子傳回 `permissionDecision: "defer"` 時，結果具有 `stop_reason: "tool_deferred"` 和 `deferred_tool_use` 攜帶待處理工具的 `id`、`name` 和 `input`。讀取此欄位以在您自己的 UI 中呈現請求，然後使用相同的 `session_id` 恢復以繼續。請參閱[延遲工具呼叫以供稍後使用](/docs/zh-TW/hooks#defer-a-tool-call-for-later)以了解完整往返。

<h4 id="user_message_uuid">
  `user_message_uuid`
</h4>

轉換回答的 [`SDKUserMessage`](#sdkusermessage) 的 `uuid`，回顯以便您可以將 Claude Code 的回覆與您傳送的訊息相匹配。Claude Code 僅在您在訊息上設定 uuid 時才回顯 `uuid`。該欄位在 `SDKUserMessage` 上是可選的，傳遞給 `query()` 的字串提示不攜帶任何。

轉換回答的訊息取決於轉換如何啟動：

* **您傳送的常規訊息**，意思是沒有 `isSynthetic: true` 的訊息：轉換在其整個執行過程中回答該訊息。當您一起傳送多個訊息時，Claude Code 可以將它們合併為一個轉換，該欄位然後僅攜帶最後一個訊息的 `uuid`。要將回覆與任何合併的訊息相匹配，請使用 [`user_message_uuids`](#user_message_uuids)。
* **您傳送的帶有 `isSynthetic: true` 的訊息**：轉換最初回答該訊息。如果 Claude Code 在工具呼叫之間拾取您的常規訊息，轉換從那時起回答拾取的訊息。回顯合成訊息的 `uuid` 需要 Agent SDK v0.3.265 或更新版本；較早的版本在合成轉換上不回顯任何內容。
* **Claude Code 自己生成的提示**，例如在工作階段重新啟動後繼續中斷工作的轉換：轉換最初不回答您的任何訊息，其框架不攜帶任何回顯。如果 Claude Code 在工具呼叫之間拾取您的常規訊息，轉換從那時起回答該訊息。拾取回顯需要 Agent SDK v0.3.265 或更新版本；較早的版本在這些轉換上不回顯任何內容。

Claude Code 在三種框架上回顯回答的訊息的 `uuid`：

* **結果**：回答您傳送的訊息的轉換的每個結果。在 Agent SDK v0.3.265 或更新版本上，每個此類結果都攜帶它。在 v0.3.265 之前，常規訊息啟動的轉換的成功結果在轉換未傳送 API 請求或以延遲工具呼叫結束時缺少它。在 v0.3.246 之前，錯誤結果也缺少它，在 v0.3.216 之前每個結果都缺少它。
* **轉換的第一個回覆**：第一個[助手訊息](#sdkassistantmessage)，或使用 `includePartialMessages` 的第一個[串流事件](#sdkpartialassistantmessage)，其 `event.type` 不是 `ping`，因此您可以在結果到達之前綁定回覆。當轉換不串流任何內容時，Claude Code 改為在第一個助手訊息上設定它。第一個回覆回顯需要 Agent SDK v0.3.246 或更新版本。當轉換回答的訊息在中途改變時，變更後的第一個回覆也攜帶該欄位，在 Agent SDK v0.3.265 或更新版本上；較早的版本在每個轉換上設定一個回覆框架。
* **轉換的每個 [`thinking_tokens`](#sdkthinkingtokensmessage) 框架**：以便您可以將思考進度歸因於您傳送的訊息，而無需等待轉換的第一個回覆。需要 Agent SDK v0.3.260 或更新版本。

Claude Code 在這些情況下省略該欄位：

* 除了那些第一個回覆之外的回覆框架
* 子代理框架
* 回答沒有 `uuid` 的訊息的轉換：轉換回答了您傳送的沒有 uuid 的訊息，或 Claude Code 啟動了轉換本身並拾取了沒有 uuid 的常規訊息
* 回答您未傳送的訊息的結果，例如崩潰的工作程序後的歸零結果

<h4 id="user_message_uuids">
  `user_message_uuids`
</h4>

您傳送的每個訊息的 `uuid`，Claude Code 在此轉換中回答了這些訊息。當您一起傳送多個訊息時，Claude Code 可以將它們合併為一個轉換，`user_message_uuid` 然後僅命名其中的最後一個。要將回覆與任何合併的訊息相匹配，請在此清單中的任何位置查找該訊息的 `uuid`。需要 Agent SDK v0.3.259 或更新版本。

Claude Code 在每個攜帶該欄位的回覆框架和結果上設定清單以及 `user_message_uuid`。有關攜帶 `user_message_uuid` 的完整框架集以及每個所需的版本，請參閱 [`user_message_uuid`](#user_message_uuid)。清單始終包含 `user_message_uuid` 並最多保留 64 個項目。

當 Claude Code 在轉換執行時拾取您傳送的常規訊息時，它會將該訊息的 `uuid` 新增到結果的清單中。

當第一個回覆或結果攜帶 `user_message_uuid` 而沒有清單時，它來自較早的 Claude Code 版本，因此回退到單個欄位。

<h4 id="queued_turn_count">
  `queued_turn_count`
</h4>

您傳送的帶有 [`origin: { kind: "human" }`](#sdkmessageorigin) 的訊息數量，在 Claude Code 產生結果時仍在命令佇列中等待。需要 Agent SDK v0.3.242 或更新版本。

`0` 和不存在的欄位告訴您什麼：

* **`0`**：Claude Code 不計算您傳送的沒有該 `origin` 的訊息，也不計算任務通知，因此轉換仍可能跟隨。
* **不存在**：Claude Code 在崩潰或致命啟動錯誤後發出的最終結果省略該欄位，並且[可能攜帶歸零的總計](/docs/zh-TW/agent-sdk/cost-tracking#recover-totals-after-a-session-crash)。

<h4 id="startup_failure_reason">
  `startup_failure_reason`
</h4>

Claude Code 拒絕啟動的原因，以便您的應用程式可以提供修復而不是重試。Claude Code 在它在已知啟動失敗時寫入的 `error_during_execution` 結果上設定它。該結果攜帶歸零的總計，其 `errors` 陣列攜帶與 stderr 相同的文字。該欄位在所有其他結果上不存在。需要 Agent SDK v0.3.274 或更新版本。

在 [`env`](#options) 中設定 `CLAUDE_CODE_STARTUP_FAILURE_RESULTS` 為 `1` 以接收每個 `SDKStartupFailureReason` 值的此結果。沒有該變數，Claude Code 僅為這些失敗寫入結果，其餘的以 stderr 輸出、非零退出和無結果訊息結束：

* Claude Code 停止的恢復，因為它[無法將工作階段返回到其工作樹](/docs/zh-TW/worktrees#the-session-resumes-outside-its-worktree)，帶有 `worktree_unverified` 或 `worktree_resume_refused`。該部分說明哪個錯誤攜帶哪個值。
* 拒絕的背景工作階段持有的對話的 [`continue`](#options)，帶有 `session_held_by_background`。對於此類對話的拒絕 [`resume`](#options)，Claude Code 僅在設定變數時寫入結果。

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

每個值命名一個拒絕：

| 值                                      | 什麼停止了工作階段                                                                                                                      |
| :------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------- |
| `org_pin_api_key_conflict`             | 受管設定[需要第一方或 Cloud 閘道登入](/docs/zh-TW/authentication#restrict-login-to-your-organization)，並且已設定 Anthropic API 金鑰、驗證令牌或 `apiKeyHelper` |
| `org_verify_failed`                    | 登入的組織無法針對 pin 進行驗證，例如由於網路故障或已撤銷的令牌                                                                                             |
| `org_pin_mismatch`                     | 登入屬於 pin 不允許的組織                                                                                                                |
| `managed_settings_invalid`             | 無法讀取受管原則設定，或 pin 未命名任何組織                                                                                                       |
| `remote_settings_required_unavailable` | 組織所需的受管設定無法載入                                                                                                                  |
| `gateway_signin_required`              | [Cloud 閘道](/docs/zh-TW/claude-apps-gateway)結束了此登入                                                                                   |
| `gateway_access_denied`                | 對 Cloud 閘道的受管設定請求返回 403，閘道的[疑難排解表](/docs/zh-TW/claude-apps-gateway-deploy#troubleshooting)涵蓋了此情況                                    |
| `proxy_invalid`                        | 代理設定不是完整的 URL                                                                                                                  |
| `temp_dir_unusable`                    | 每個使用者的臨時目錄不安全或無法建立                                                                                                             |
| `cwd_unavailable`                      | 工作目錄已刪除、移動或無法讀取                                                                                                                |
| `shell_tool_missing`                   | 在 Windows 上，沒有可用的 shell 工具：Git Bash 遺失，PowerShell 遺失或使用 `CLAUDE_CODE_USE_POWERSHELL_TOOL` 關閉                                   |
| `session_held_by_background`           | 要恢復或繼續的對話作為[背景工作階段](/docs/zh-TW/agent-view)執行                                                                                       |
| `worktree_resume_refused`              | 工作階段的工作樹未通過其安全檢查，或恢復是從其內部啟動的。`errors` 說明執行相同恢復是否繼續而不使用工作樹                                                                      |
| `worktree_unverified`                  | 工作階段的工作樹現在無法驗證，重試可能會成功                                                                                                         |
| `cli_version_too_old`                  | 此 Claude Code 版本低於 Anthropic 所需的最低版本                                                                                           |
| `bypass_root`                          | 在以 root 身份執行時請求了繞過權限模式                                                                                                         |

<h3 id="sdksystemmessage">
  `SDKSystemMessage`
</h3>

系統初始化訊息。

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

`fast_mode_state` 報告工作階段的[快速模式](/docs/zh-TW/fast-mode)狀態。當某些東西阻止快速模式時，`fast_mode_disabled_reason` 命名阻止它的檢查；該欄位需要 Claude Code v2.1.219 或更新版本。有關原因代碼及其含義，請參閱結果訊息上的 [`fast_mode_disabled_reason`](#sdkresultmessage)。

`terminal_slash_commands` 命名 `slash_commands` 中的項目，其介面綁定到本地終端，例如 `exit`。您可以像 `slash_commands` 中的任何其他項目一樣傳送它們；該欄位的存在是為了遠端或行動用戶端可以從其命令選單中隱藏它們。該欄位僅在非空時存在，需要 Agent SDK v0.3.229 或更新版本。

* 每個 `mcp_servers` 項目上的 `source`：伺服器定義的來源，與 [`McpServerStatus`](#mcpserverstatus) 的 `source` 具有相同的值。需要 Agent SDK v0.3.274 或更新版本。
*

`effort`：[努力級別](/docs/zh-TW/model-config#adjust-effort-level) Claude Code 在工作階段的下一個請求上傳送，或當它不傳送任何時為 `null`。Claude Code 僅在它傳送給[遠端控制](/docs/zh-TW/remote-control)用戶端的初始化訊息上設定該欄位，並從您的應用程式讀取的初始化訊息中省略它。需要 Agent SDK v0.3.234 或更新版本。

`capabilities` 陣列命名此 CLI 實現的協議行為，因此您可以進行功能偵測而不是比較 `claude_code_version` 字串。這是一個開放集合：忽略您不認識的值，並檢查您依賴其行為的特定功能。該欄位需要 Claude Code v2.1.205 或更新版本，在較早的 CLI 上不存在。

| 功能                                                                                                                                                                                          | 含義                                                                                                                   |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| `interrupt_receipt_v1`                                                                                                                                                                      | [`interrupt()`](#query-object) 使用列出中斷到達時待處理的訊息的 [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) 收據進行解析 |
| `interrupt_cancel_queued_v1`                                                                                                                                                                |                                                                                                                      |
| `interrupt` 控制請求尊重 `cancel_queued: true`，取消收據在 `still_queued` 下列出的訊息，並改為在 `cancelled` 下列出它們。請參閱 [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse)。需要 Claude Code v2.1.219 或更新版本 |                                                                                                                      |

<h3 id="sdkpartialassistantmessage">
  `SDKPartialAssistantMessage`
</h3>

串流部分訊息（僅當 `includePartialMessages` 為 true 時）。`parent_tool_use_id` 欄位始終為 `null`：串流事件僅針對主工作階段發出。對於子代理歸因，使用完整訊息，其攜帶 `parent_tool_use_id`，或啟用 [`forwardSubagentText`](#options) 以接收子代理文字和思考作為完整訊息。

```typescript theme={null}
type SDKPartialAssistantMessage = {
  type: "stream_event";
  event: BetaRawMessageStreamEvent; // From Anthropic SDK
  parent_tool_use_id: string | null;
  uuid: UUID;
  session_id: string;
  ttft_ms?: number; // Time to first token in ms, present only on message_start events
  user_message_uuid?: string;
  user_message_uuids?: string[];
};
```

Claude Code 在轉換的第一個非 ping 串流事件上設定 `user_message_uuid` 和 `user_message_uuids`，並在轉換回答的訊息改變時再次設定，條件在 [`user_message_uuid`](#user_message_uuid) 中。

<h3 id="sdkcompactboundarymessage">
  `SDKCompactBoundaryMessage`
</h3>

指示對話壓縮邊界的訊息。

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

迴圈發出的通用文字橫幅。攜帶非錯誤狀態行、鉤子反饋（例如 `UserPromptSubmit` 鉤子的區塊原因）和命令輸出。在 Claude Code v2.1.227 或更新版本上，鉤子的 [`systemMessage`](/docs/zh-TW/hooks#json-output) 可以作為此訊息到達，每行前綴為鉤子的名稱，例如 `PostToolUse:Bash says:`。鉤子的 `systemMessage` 是否作為此訊息到達取決於事件。每個[事件的部分](/docs/zh-TW/hooks#hook-events)在鉤子頁面上說明輸出如何呈現。將 `content` 呈現為給定 `level` 的純文字。

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

在正常工作程序拆卸時發出，以便遠端用戶端可以顯示工作程序退出的原因，而不是等待心跳超時。`reason` 是由主機 CLI 設定的短 snake\_case 字串，例如 `"host_exit"` 或 `"remote_control_disabled"`。僅在即時串流時對此採取行動。恢復的工作階段會重播此訊息的過去實例，因此在這種情況下忽略它們。

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

外掛程式安裝進度事件。當設定 [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/zh-TW/env-vars) 時發出，以便您的 Agent SDK 應用程式可以在第一個轉換之前追蹤市場外掛程式安裝。`started` 和 `completed` 狀態括住整體安裝。`installed` 和 `failed` 狀態報告個別市場並包含 `name`。

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

當權限系統在沒有互動式提示的情況下拒絕工具呼叫時發出的串流事件。使用它在您的 UI 中即時呈現拒絕，而不是僅觀察隨後的 `is_error` 工具結果。它報告的拒絕取決於執行如何處理權限提示：

* **使用 [`canUseTool`](#canusetool) 回呼**和預設 [`permissionPrompts: 'host'`](#options)：權限提示進入您的回呼，此事件報告 Claude Code 自己決定的拒絕，而不呼叫它。
* **都沒有**：裸 `-p` 執行，或 `query()` 既不設定 `canUseTool` 也不設定 `permissionPromptToolName`，拒絕任何會提示的工具呼叫，此事件報告這些拒絕以及 Claude Code 自己決定的拒絕。在 v2.1.223 之前，Claude Code 在沒有回呼的執行中不發出此事件。
* **使用 MCP 提示工具**，使用 `permissionPromptToolName` 或 [`--permission-prompt-tool`](/docs/zh-TW/cli-reference#cli-flags) 旗標設定，以及預設 `permissionPrompts: 'host'`：Claude Code 根本不發出此事件，甚至不發出它自己決定的規則拒絕。
* **使用 [`permissionPrompts: 'none'`](#options)**：Claude Code 拒絕會提示的呼叫，即使也設定了 `canUseTool` 或 MCP 提示工具，此事件報告這些拒絕以及 Claude Code 自己決定的拒絕。需要 Claude Code v2.1.259 或更新版本。

在每個設定中，此事件跳過在 `PreToolUse` 鉤子路徑上決定的任何拒絕，無論鉤子本身拒絕了呼叫還是拒絕規則覆蓋了鉤子的允許或詢問決定。該事件也是盡力而為：偶爾 Claude Code 會記錄拒絕而不發出此事件，因此[結果訊息](#sdkresultmessage)上的 `permission_denials` 是權威記錄。

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

| 欄位                     | 類型       | 描述                                                            |
| ---------------------- | -------- | ------------------------------------------------------------- |
| `tool_name`            | `string` | 被拒絕的工具的名稱                                                     |
| `tool_use_id`          | `string` | 此拒絕回答的 `tool_use` 區塊的 ID                                      |
| `agent_id`             | `string` | 當拒絕的呼叫源自子代理內部時的子代理 ID。鏡像 `can_use_tool` 上的欄位以進行主機端路由          |
| `decision_reason_type` | `string` | 決定組件的判別器，例如 `"rule"`、`"mode"`、`"classifier"` 或 `"asyncAgent"` |
| `decision_reason`      | `string` | 決定組件的人類可讀原因（如果可用）                                             |
| `message`              | `string` | 在 `tool_result` 中傳回給模型的拒絕訊息                                   |

<h3 id="sdkpermissiondenial">
  `SDKPermissionDenial`
</h3>

有關被拒絕的工具使用的資訊。

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

`/context` 報告的結構化形式，作為 `context_usage` 在傳遞 `/context` 結果的 [`SDKAssistantMessage`](#sdkassistantmessage) 上進行。Agent SDK v0.3.232 及更新版本匯出該類型。與 [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse) 不同，它僅攜帶呈現使用情況明細所需的資料，不包含 `color` 和 `gridRows` 等顯示欄位。

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

該表列出 Claude Code 在每個欄位中放置的內容。從 `model` 到 `over_limit` 的欄位描述整個工作階段，集合欄位將令牌歸因於個別項目。

| 欄位               | 類型                                                        | 描述                                                                                                                                                                    |
| ---------------- | --------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `model`          | `string`                                                  | Claude Code 計算使用情況的主迴圈的模型，不是子代理的                                                                                                                                      |
| `total_tokens`   | `number`                                                  | Claude Code 對使用中令牌的估計。未限制在視窗中，因此當工作階段超過限制時可以超過 `raw_max_tokens`                                                                                                       |
| `raw_max_tokens` | `number`                                                  | 模型的內容視窗，或較低的[自動壓縮視窗](/docs/zh-TW/model-config#context-window-and-auto-compaction)（當適用時），例如您設定的或 Claude Code 應用於某些具有 1M 令牌視窗的模型的 200K 邊界。Claude Code 根據此視窗測量 `total_tokens` |
| `percentage`     | `number`                                                  | `total_tokens` 作為 `raw_max_tokens` 的四捨五入百分比，因此當工作階段超過限制時可以超過 100                                                                                                      |
| `over_limit`     | `object`                                                  | 僅當 `total_tokens` 超過 `raw_max_tokens` 時存在。`tokens_over` 是超過的金額，`kind` 說明 Claude Code 如何解決視窗                                                                           |
| `categories`     | [`SDKContextUsageCategory`](#sdkcontextusagecategory)`[]` | 使用情況按類別明細的每一行一個項目                                                                                                                                                     |
| `mcp_tools`      | `object[]`                                                | 歸因於每個 MCP 工具的令牌，帶有其線路名稱（例如 `mcp__linear__create_issue`）和其 `server_name`                                                                                               |
| `memory_files`   | `object[]`                                                | 歸因於每個載入的記憶檔案的令牌，帶有其 `path` 和來源標籤（例如 `Project` 或 `User`）在 `type` 中                                                                                                     |
| `agents`         | `object[]`                                                | 歸因於每個自訂子代理定義的令牌，帶有來源識別碼，例如 `projectSettings`、`userSettings` 或 `plugin`。內建子代理未列出                                                                                       |
| `skills`         | `object[]`                                                | 歸因於技能清單中每個技能的令牌，帶有來源識別碼，對於外掛程式技能，外掛程式的名稱在 `plugin_name` 中。當沒有技能貢獻令牌時不存在                                                                                               |

`over_limit.kind` 記錄 Claude Code 如何解決視窗，而不是 API 是否接受下一個請求：

* `hard_limit`：視窗是 Claude Code 認為是模型自己的限制，超過該限制 API 拒絕請求
* `compaction_window`：視窗是壓縮原則視窗，可能與模型的限制一致，也可能不一致

Claude Code 以加法方式演進該類型，添加新資料作為可選欄位，而不是重新塑造現有欄位。讀取您知道的欄位並忽略您不認識的任何欄位。

<h3 id="sdkcontextusagecategory">
  `SDKContextUsageCategory`
</h3>

`/context` 使用情況按類別明細的一行。

```typescript theme={null}
type SDKContextUsageCategory = {
  name: string;
  tokens: number;
  kind: "used" | "free" | "buffer" | "deferred";
};
```

該表列出 Claude Code 在行的每個欄位中放置的內容。

| 欄位       | 類型       | 描述                                                          |
| -------- | -------- | ----------------------------------------------------------- |
| `name`   | `string` | 行的顯示名稱，如 `/context` 列印的那樣，例如 `Messages`。按 `kind` 分類行，而不是按名稱 |
| `tokens` | `number` | 行的令牌計數。行可以攜帶零令牌                                             |
| `kind`   | `string` | 行代表什麼：`used`、`free`、`buffer` 或 `deferred`                   |

每個 `kind` 值說明行的令牌是什麼：

* `used`：佔據內容視窗的內容
* `free`：剩餘視窗
* `buffer`：壓縮保留
* `deferred`：Claude Code 保留在視窗外的工具架構，從使用情況計算中排除，列出以供了解

<h3 id="sdkmessageorigin">
  `SDKMessageOrigin`
</h3>

使用者角色訊息的來源。這在 [`SDKUserMessage`](#sdkusermessage) 上顯示為 `origin`，並轉發到相應的 [`SDKResultMessage`](#sdkresultmessage)，以便您可以告訴什麼觸發了給定的轉換。

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

| `kind`              | 含義                                                                                                                                                                                                                                                                       |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `human`             | 來自最終使用者的直接輸入。如果您的應用程式將使用者輸入的內容轉發為使用者訊息，請明確將其 `origin` 設定為 `{ kind: "human" }`：Claude Code 將沒有 `origin` 的使用者訊息視為未歸因，並檢查需要人類輸入的提示（例如 [`ultracode` 工作流程關鍵字](/docs/zh-TW/workflows#ask-for-a-workflow-in-your-prompt)）不接受它。在 v2.1.210 之前，Claude Code 將使用者訊息上不存在的 `origin` 視為人類輸入。 |
| `channel`           | 在[頻道](/docs/zh-TW/channels)上到達的訊息。`server` 是來源 MCP 伺服器名稱。                                                                                                                                                                                                                     |
| `peer`              | 來自另一個代理的訊息：進程內[隊友](/docs/zh-TW/agent-teams)或[跨工作階段對等](/docs/zh-TW/cross-session-messaging)，您的另一個 Claude Code 工作階段。請參閱[對等來源欄位](#peer-origin-fields)以了解每個欄位的語義和信任模型。                                                                                                                 |
| `task-notification` | 為沒有新鮮使用者提示的傳遞注入的合成轉換，例如完成的背景任務；請參閱 [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) 以了解該分支。您的應用程式[宣告為排定執行](#declare-a-scheduled-run)的提示也攜帶此類型。可選的 `subkind` 標記引發通知的原因。請參閱[任務通知子類型](#task-notification-subkinds)。                                              |
| `coordinator`       | 來自[代理團隊](/docs/zh-TW/agent-teams)中的團隊協調員的訊息。                                                                                                                                                                                                                                  |
| `auto-continuation` | 當工作階段在沒有新鮮使用者輸入的情況下繼續時注入的合成轉換，例如觸發後續提示的命令結果。                                                                                                                                                                                                                             |
| `unclassified`      | 無法確定來源的注入轉換。當 Claude Code 接收帶有 `isSynthetic: true` 的 [`SDKUserMessage`](#sdkusermessage) 並無法將其分類為任何其他 `kind` 時，它在訊息到達時設定此類型，並將轉換框架化為模型的非使用者來源，而不是將其視為人類輸入。您的應用程式不應設定此值。                                                                                                  |

<h3 id="task-notification-subkinds">
  任務通知子類型
</h3>

當 Claude Code 將任務通知傳遞到工作階段時，如果 Anthropic 伺服器驗證了該通知的來源，它會在通知的 `origin` 上設定 `subkind`。當您的應用程式[自己宣告訊息為排定執行](#declare-a-scheduled-run)時，它也會設定 `subkind`，這需要 TypeScript Agent SDK v0.3.280 或更新版本。`subkind` 需要 Claude Code v2.1.213 或更新版本，它採用以下兩個值之一：

* `scheduled-trigger`：通知是[例程](/docs/zh-TW/routines)的儲存提示，因為例程的觸發器之一觸發而傳遞：其排程、其 [API 觸發器](/docs/zh-TW/routines#add-an-api-trigger)、其 [GitHub 觸發器](/docs/zh-TW/routines#add-a-github-trigger) 或**立即執行**。您的應用程式[宣告為排定執行](#declare-a-scheduled-run)的提示也攜帶此值。Claude Code 將這些框架化為工作階段的指派任務，與[其他任務通知攜帶的通知](#sdktasknotificationmessage)不同。
*

`peer-send-message`：通知是另一個您的工作階段使用[雲端工作階段](/docs/zh-TW/claude-code-on-the-web)使用的伺服器端 `send_message` 工具傳送的訊息，而不是[跨工作階段 `SendMessage` 工具](/docs/zh-TW/cross-session-messaging)，並且 Anthropic 伺服器驗證了兩個工作階段都屬於同一個私人工作階段組。需要 Claude Code v2.1.224 或更新版本。伺服器未以這種方式驗證的 `send_message` 傳遞沒有 subkind。

每個其他任務通知都沒有 `subkind`。這包括[PR 活動](/docs/zh-TW/claude-code-on-the-web#how-claude-responds-to-pr-activity)傳遞到工作階段和背景事件，例如完成的任務。來自[跨工作階段 `SendMessage` 工具](/docs/zh-TW/cross-session-messaging)的訊息根本不是任務通知：無論它們來自同一機器上的工作階段還是通過 Anthropic 伺服器來自另一台機器，Claude Code 都給它們 `kind: "peer"` 和[對等來源欄位](#peer-origin-fields)。

`fireReason` 說明 `scheduled-trigger` 通知觸發的原因，作為短小寫令牌，例如 `scheduled`、`manual`、`retry`、`catch_up` 或 `api`。Anthropic 伺服器在[例程](/docs/zh-TW/routines)的傳遞上設定它，您的應用程式在宣告排定執行時設定它。當都沒有傳送時不存在。需要 TypeScript Agent SDK v0.3.280 或更新版本。

<h4 id="declare-a-scheduled-run">
  宣告排定執行
</h4>

如果您的應用程式按自己的排程執行提示，請宣告每個執行，以便 Claude Code 將轉換框架化為排定任務，而不是來自使用者的即時輸入。使用 [`env`](#options) 中設定為 `1` 的 `CLAUDE_CODE_HOST_SCHEDULED_RUN` 啟動工作階段，然後使用 `origin: { kind: "task-notification", subkind: "scheduled-trigger", fireReason: "scheduled" }` 和不帶 `isSynthetic` 傳送執行的 [`SDKUserMessage`](#sdkusermessage)。Claude Code 在沒有該變數的情況下啟動的程序中忽略宣告。它也在其環境攜帶 [`CLAUDECODE`](/docs/zh-TW/env-vars) 或 `CLAUDE_CODE_CHILD_SESSION` 的程序中忽略它。Claude Code 僅在值為 1 到 32 個小寫字母或底線時保留 `fireReason`。需要 TypeScript Agent SDK v0.3.280 或更新版本。

<h3 id="peer-origin-fields">
  對等來源欄位
</h3>

`peer` 來源識別哪個代理傳送了訊息：進程內[隊友](/docs/zh-TW/agent-teams)使用 `SendMessage` 傳送到 `main`，或[跨工作階段對等](/docs/zh-TW/cross-session-messaging)，您的另一個 Claude Code 工作階段。跨工作階段對等在 macOS 和 Linux 上需要 Claude Code v2.1.224 或更新版本；請參閱[跨工作階段訊息可用性](/docs/zh-TW/cross-session-messaging#availability)以了解原生 Windows 要求。跨工作階段對等可以在同一機器上執行，或在[您的另一台機器](/docs/zh-TW/cross-session-messaging#message-sessions-on-other-machines)或[雲端](/docs/zh-TW/claude-code-on-the-web)中執行，當其訊息通過遠端控制到達時。兩種發送者類型以不同方式填充欄位：

* `from`：隊友的名稱，或跨工作階段對等的發送者地址。對於[單向跨機器訊息](/docs/zh-TW/cross-session-messaging#message-sessions-on-other-machines)，發送者沒有回覆地址，`from` 是 `"unknown"`。該值由發送者編寫；`verifiedPeerPid` 是驗證的身份。
*

`fromMode`：傳送工作階段的權限類別，`bypass` 或 `prompting`，由在您的工作階段之間轉發對等訊息的主機宣告，例如[桌面應用程式](/docs/zh-TW/desktop#work-across-sessions)。Claude Code 在接收工作階段中讀取它，當它應用[入站控制](/docs/zh-TW/cross-session-messaging#control-inbound-messages)時。需要 Agent SDK v0.3.234 或更新版本。

* `senderTaskId`：隊友的任務 ID。對於跨工作階段對等不存在。
*

`name`：發送者的顯示名稱，由 Claude Code 規範化：它去除 Unicode 控制、格式、代理和行或段落分隔符代碼點，然後修剪結果並將其限制為 64 個代碼點，帶有省略號。需要 Claude Code v2.1.205 或更新版本。

*

`body`：解碼的訊息正文，去除對等信封，與模型看到的逐位元組相同。始終存在於隊友訊息中；對於跨工作階段對等，僅當轉換恰好是由 Claude Code 形成的一個對等信封時才存在。呈現 `name` 和 `body` 而不是重新解析訊息文字。需要 Claude Code v2.1.205 或更新版本。

*

`fromSession`：發送者的主機可開啟工作階段 ID，由發送者的主機設定，以便您的 UI 可以連結回傳送工作階段。像 `from` 一樣，它是發送者聲稱的：僅將其用作導航目標，不要將其視為發送者身份的證明。需要 Claude Code v2.1.216 或更新版本。

*

`verifiedPeerPid`：連線到此工作階段的跨工作階段訊息套接字的程序的程序 ID，由核心驗證並從連線本身讀取，絕不從有效負載讀取。使用它，而不是 `from`，來識別發送者：`from` 可由任何同一使用者程序偽造。當 Claude Code 無法驗證它時，該欄位不存在，例如在 Windows 或非套接字入站上，因此不存在的值意味著發送者未驗證。對於轉發的流量，它識別轉發而不是訊息的作者，程序 ID 是可回收的，因此將其視為來源而不是驗證令牌。需要 Claude Code v2.1.216 或更新版本。

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
  mcp_server?: McpServerProvenance;
};
```

當工具來自 MCP 伺服器時，`mcp_server` 會出現；見 [`McpServerProvenance`](#mcpserverprovenance)。`PostToolUse`、`PostToolUseFailure`、`PermissionRequest` 和 `PermissionDenied` 輸入攜帶相同的欄位。此欄位需要 Agent SDK v0.3.274 或更新版本。

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

在請求的模型切換生效之前觸發。`context_tokens` 和其後的欄位估計重新傳送對話到新模型的成本。如需完整的欄位說明和阻止語義，見 [PreModelSwitch](/docs/zh-TW/hooks#premodelswitch)。

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

在工作階段的模型變更後觸發。它攜帶與 `PreModelSwitchHookInput` 相同的欄位，加上兩個額外的 `source` 值。見 [PostModelSwitch](/docs/zh-TW/hooks#postmodelswitch)。

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
  /** @deprecated 自 v2.1.178 起已棄用。攜帶工作階段衍生的團隊名稱；將被移除。 */
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
  /** @deprecated 自 v2.1.178 起已棄用。攜帶工作階段衍生的團隊名稱；將被移除。 */
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
  /** @deprecated 自 v2.1.178 起已棄用。攜帶工作階段衍生的團隊名稱；將被移除。 */
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

`directory` 是被新增目錄的絕對路徑。`source` 是 `"slash_command"`（當 `/add-dir` 新增時）或 `"register_repo_root"`（當 SDK 控制請求執行時）。

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
  /**
   * 終端逸出序列（例如 OSC 9 / OSC 777 桌面通知）
   * 供 Claude Code 代表您發出。僅允許通知/標題 OSCs
   * （0、1、2、9、99、777）和 BEL；包含
   * 其他任何內容的值會被整體忽略。僅互動式 CLI 會發出
   * 它；SDK 會忽略此欄位。
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
        /** 當決定為 "block" 時，從阻止訊息中省略原始提示。 */
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
         * SessionStart hooks 完成後重新掃描 skill 和命令目錄，
         * 以便 hook 安裝的 skills 在同一工作階段中可用。
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
         * 與 PreToolUse 相同的合約："allow" 繼續、"deny" 取消
         * 切換、"ask" 要求使用者確認。僅互動式工作階段中的 /model
         * 顯示該提示；其他所有表面（包括 set_model 請求）
         * 將 "ask" 視為拒絕。
         */
        permissionDecision?: "allow" | "deny" | "ask";
        permissionDecisionReason?: string;
      }
    | {
        hookEventName: "PostModelSwitch";
        /** 透過新模型提供的下一個請求到達模型。 */
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
         * 關於此工具呼叫結果的簡短說明，供自動模式
         * 權限分類器使用。上限為 2000 個字元，在回應
         * 同一呼叫的所有 hooks 中共享；僅在同步
         * hook 回應上受尊重。不要將不受信任的工具輸出複製到其中。
         */
        classifierContext?: string;
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
        /** 用來代替 delta 顯示的文字。省略（或返回 delta 不變）以顯示原始內容。 */
        displayContent?: string;
      };
};
```

<h2 id="tool-input-types">
  工具輸入類型
</h2>

所有內建 Claude Code 工具的輸入架構文件。這些類型從 `@anthropic-ai/claude-agent-sdk` 匯出，可用於類型安全的工具互動。

<h3 id="toolinputschemas">
  `ToolInputSchemas`
</h3>

從 `@anthropic-ai/claude-agent-sdk` 匯出的工具輸入類型的聯合；成員包括：

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

**工具名稱：** `Agent`。先前的名稱 `Task` 仍被接受為別名，[`SDKSystemMessage`](#sdksystemmessage) 初始化訊息中的 `tools` 陣列目前仍將此工具列為 `Task` 以保持向後相容性。

<Note>
  `mode` 欄位在 Claude Code v2.1.212 或更新版本上已棄用且被忽略。子代理在父工作階段的權限模式或其定義的 [`permissionMode`](#agentdefinition) 中執行，[子代理繼承規則](/docs/zh-TW/agent-sdk/permissions#available-modes) 決定使用哪一個。
</Note>

```typescript theme={null}
type AgentInput = {
  description: string;
  prompt: string;
  subagent_type?: string;
  model?: "sonnet" | "opus" | "haiku" | "fable";
  run_in_background?: boolean;
  name?: string;
  team_name?: string; // Deprecated; ignored
  mode?: "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan"; // Deprecated; ignored. The subagent inheritance rules decide a subagent's permission mode
  isolation?: "worktree" | "remote";
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
  answers?: Record<string, string>;
  annotations?: Record<string, { preview?: string; notes?: string }>;
  metadata?: { source?: string };
};
```

在執行期間向使用者提出澄清問題。詳見[處理核准和使用者輸入](/docs/zh-TW/agent-sdk/user-input#handle-clarifying-questions)以了解使用詳情。

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

執行 Bash 命令，支援選擇性逾時和背景執行。工作目錄在命令之間保持不變，包括多輪工作階段後續執行的命令；shell 狀態（例如匯出的環境變數）則不會保持。如需了解哪些目錄變更會保留，詳見[命令之間保持的內容](/docs/zh-TW/tools-reference#what-persists-between-commands)。

<h3 id="monitor">
  Monitor
</h3>

**工具名稱：** `Monitor`

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

執行背景來源並將每個事件傳遞給 Claude，使其能夠做出反應而無需輪詢：`command` 執行指令碼並每行 stdout 發出一個事件，`ws` 開啟 WebSocket 並每個文字框架發出一個事件。提供 `command` 或 `ws` 中的恰好一個。`ws` 來源需要 Claude Code v2.1.195 或更新版本。

`timeout_ms` 是監視的截止時間（以毫秒為單位）。預設為 300000，接受最多 3600000 的值。有效截止時間最多為 1800000，即 30 分鐘，因此較大的接受值會縮短到該值。在截止時間時，監視結束，Claude 收到一個通知，以便在仍需要時啟動新的監視。

匯出的類型將 `timeout_ms` 標記為必需，因為架構填入預設值；省略它的呼叫會驗證通過。

Monitor 執行命令時，遵循與 Bash 相同的權限規則；WebSocket 監視會單獨提示核准。詳見[Monitor 工具參考](/docs/zh-TW/tools-reference#monitor-tool)以了解行為和提供者可用性。

<h3 id="taskoutput">
  TaskOutput
</h3>

在 Claude Code v2.1.277 中移除，連同其 `TaskOutputInput` 類型一起移除。先前從執行中或已完成的背景任務中擷取輸出；Claude 改用 `Read` 讀取背景任務的輸出檔案。

仍命名 `TaskOutput` 的 `disallowedTools` 項目或拒絕規則會被忽略，不會發出警告。

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

在檔案中執行精確的字串替換。

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

從本機檔案系統讀取檔案，包括文字、影片、PDF 和 Jupyter 筆記本。使用 `pages` 指定 PDF 頁面範圍（例如 `"1-5"`）。

對於 PDF，Claude 在 Read 呼叫的 `tool_result` 內容中接收檔案的內容。傳回 `pdf` [輸出](#tool-output-types)的讀取會帶有摘要 `text` 區塊，後面跟著 `document` 區塊。傳回 `parts` 輸出的讀取會帶有摘要 `text` 區塊，後面跟著每個提取頁面的一個區塊：`image` 區塊，或當 Claude Code 無法將其呈現為影片時命名該頁面的 `text` 區塊。在 Agent SDK v0.3.242 之前，Claude Code 在工具結果後作為單獨的 `user` 訊息傳遞檔案的內容。

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

將檔案寫入本機檔案系統，如果存在則覆蓋。

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

快速檔案模式匹配，適用於任何程式碼庫大小。

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
  "-o"?: boolean; // print only the matched parts of each line; requires output_mode: "content"
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

基於 ripgrep 的強大搜尋工具，支援正規表達式。

<h3 id="taskstop">
  TaskStop
</h3>

**工具名稱：** `TaskStop`

```typescript theme={null}
type TaskStopInput = {
  task_id?: string;
  shell_id?: string; // Deprecated: use task_id
};
```

按 ID 停止執行中的背景任務或 shell。自 v2.1.198 起，`task_id` 也接受代理團隊隊友或按代理 ID 或名稱的具名背景代理。

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

編輯 Jupyter 筆記本檔案中的儲存格。

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

從 URL 擷取內容並使用 AI 模型進行處理。

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

搜尋網路並傳回格式化的結果。

<h3 id="workflow">
  Workflow
</h3>

**工具名稱：** `Workflow`

```typescript theme={null}
type WorkflowInput = {
  script?: string;
  name?: string;
  scriptPath?: string;
  args?: unknown; // any JSON value; the published typings render this as an object map
  resumeFromRunId?: string;
  title?: string; // ignored; the script's meta block sets the title
  description?: string; // ignored; the script's meta block sets the description
};
```

執行[動態工作流程](/docs/zh-TW/workflows)：在背景中協調許多子代理並傳回一個統一結果的指令碼。Workflow 工具在 Agent SDK v0.3.149 及更新版本中可用。至少需要 `script`、`name` 或 `scriptPath` 中的一個。

| 欄位                | 類型        | 說明                                                                                                                                                                        |
| ----------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `script`          | `string`  | 內嵌工作流程指令碼。必須以 `export const meta = { name, description }` 作為字面值開始，後面跟著使用 `agent()`、`parallel()`、`pipeline()` 和 `phase()` 的指令碼主體。`meta` 中的選擇性 `phases` 陣列在進度檢視中將代理分組到具名階段下 |
| `name`            | `string`  | 內建工作流程的名稱或儲存在 `.claude/workflows/` 中的工作流程名稱。解析為指令碼                                                                                                                        |
| `scriptPath`      | `string`  | 磁碟上工作流程指令碼檔案的路徑。優先於 `script` 和 `name`。Claude Code 保留每次呼叫的指令碼並在結果中傳回路徑，因此您可以編輯該檔案並使用相同的 `scriptPath` 重新呼叫以進行迭代                                                             |
| `args`            | `unknown` | 輸入值，作為全域 `args` 公開給指令碼，用於參數化的具名工作流程，例如研究問題或檔案路徑清單。將陣列和物件作為實際 JSON 值傳遞，而不是 JSON 編碼的字串                                                                                      |
| `resumeFromRunId` | `string`  | 先前 `Workflow` 呼叫的執行 ID 以繼續。具有未變更輸入的已完成 `agent()` 呼叫通常傳回快取結果；其餘的執行即時。[暫停後繼續](/docs/zh-TW/workflows#resume-after-a-pause)涵蓋哪些已完成的呼叫重新執行。僅限同一工作階段                                 |
| `title`           | `string`  | 已忽略；指令碼的 `meta` 區塊設定標題                                                                                                                                                    |
| `description`     | `string`  | 已忽略；指令碼的 `meta` 區塊設定說明                                                                                                                                                    |

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

建立和管理結構化任務清單以追蹤進度。

<Note>
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  詳見[模型可用性](/docs/zh-TW/agent-sdk/todo-tracking#model-availability)以選擇加入。
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

建立單一任務並傳回其指派的 ID。

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

按 ID 修補一個任務。將 `status` 設定為 `"deleted"` 以移除它。

<h3 id="taskget">
  TaskGet
</h3>

**工具名稱：** `TaskGet`

```typescript theme={null}
type TaskGetInput = {
  taskId: string;
};
```

傳回一個任務的完整詳情，或在找不到 ID 時傳回 `null`。

<h3 id="tasklist">
  TaskList
</h3>

**工具名稱：** `TaskList`

```typescript theme={null}
type TaskListInput = {};
```

傳回目前清單中所有任務的快照。

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**工具名稱：** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeInput = {
  /** Deprecated: no longer used. */
  allowedPrompts?: Array<{
    tool: "Bash";
    prompt: string;
  }>;
  [k: string]: unknown;
};
```

退出 Plan Mode。`allowedPrompts` 欄位已棄用且被忽略；Claude Code 仍接受它以便現有呼叫者和文字記錄驗證。在 v2.1.205 之前，它要求基於提示的 Bash 權限以實施計畫。

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**工具名稱：** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesInput = {
  server?: string;
};
```

列出來自已連接伺服器的可用 MCP 資源。

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

從伺服器讀取特定 MCP 資源。

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

建立並進入臨時 git worktree 以進行隔離工作。傳遞 `path` 以切換到現有 worktree，而不是建立新的。首次進入時，目標必須是目前儲存庫的已註冊 worktree，或在多儲存庫工作區中，位於其中嵌套的儲存庫；從 worktree 工作階段內，它必須位於工作階段儲存庫的 `.claude/worktrees/` 下。`name` 和 `path` 互斥。

<h3 id="exitworktree">
  ExitWorktree
</h3>

**工具名稱：** `ExitWorktree`

```typescript theme={null}
type ExitWorktreeInput = {
  action: "keep" | "remove";
  discard_changes?: boolean;
};
```

退出目前 git worktree 並返回原始工作目錄。`keep` 動作將 worktree 和分支保留在磁碟上，而 `remove` 刪除兩者。當移除具有未提交檔案或未合併提交的 worktree 時，`discard_changes` 必須為 `true`。

<h3 id="enterplanmode">
  EnterPlanMode
</h3>

**工具名稱：** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeInput = {};
```

進入 Plan Mode，其中 Claude 在進行變更前研究並呈現計畫。

<h3 id="croncreate">
  CronCreate
</h3>

**工具名稱：** `CronCreate`

```typescript theme={null}
type CronCreateInput = {
  cron: string;
  prompt: string;
  recurring?: boolean;
  durable?: boolean;
};
```

在本機時間的 5 欄位 cron 排程上排程提示執行。將 `recurring` 設定為 `false` 以在下一個符合時單次觸發。工作預設為工作階段範圍：啟動新對話會清除它們，使用 `--resume` 或 `--continue` 繼續會還原尚未過期的工作。詳見[排程任務](/docs/zh-TW/scheduled-tasks)。

將 `durable` 設定為 `true` 以要求持久化到 `.claude/scheduled_tasks.json`，使工作在重新啟動後存活。持久化排程並非在每個工作階段都可用：當不可用時，Claude Code 接受 `durable: true` 但建立工作階段專用工作。讀取輸出的 `durable` 欄位以查看工作是否已持久化。

<h3 id="crondelete">
  CronDelete
</h3>

**工具名稱：** `CronDelete`

```typescript theme={null}
type CronDeleteInput = {
  id: string;
};
```

按從 `CronCreate` 傳回的 ID 刪除排程的 cron 工作。

<h3 id="cronlist">
  CronList
</h3>

**工具名稱：** `CronList`

```typescript theme={null}
type CronListInput = {};
```

列出排程的 cron 工作：來自 `.claude/scheduled_tasks.json` 的持久化工作和來自目前工作階段的工作階段專用工作。

<h3 id="schedulewakeup">
  ScheduleWakeup
</h3>

**工具名稱：** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupInput = {
  delaySeconds?: number;
  reason?: string;
  prompt?: string;
  noop?: boolean;
  stop?: boolean;
};
```

排程一次性喚醒，在延遲後觸發給定的提示。此工具支援自步調 `/loop` 命令。執行時間將 `delaySeconds` 限制在 60 到 3600 秒之間。除非 `stop` 為 true，否則 `delaySeconds`、`reason`、`prompt` 和 `noop` 欄位為必需。`noop: true` 報告沒有任何變更的喚醒。設定 `stop: true` 以取消待處理的喚醒並結束自步調 `/loop`。`stop` 欄位需要 Claude Code v2.1.202 或更新版本。詳見[工具參考中的 ScheduleWakeup 列](/docs/zh-TW/tools-reference)。

<h3 id="remotetrigger">
  RemoteTrigger
</h3>

**工具名稱：** `RemoteTrigger`

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

管理[例行工作](/docs/zh-TW/routines)，即在雲端託管的排程和觸發 Claude Code 執行。此工具支援 `/schedule` 命令。`trigger_id` 對於 `get`、`update`、`run` 和 `list_runs` 動作為必需。`body` 對於 `create`、`update` 和 `create_webhook_trigger` 為必需，對於 `run` 為選擇性。

`create_webhook_trigger` 將事件來源附加到現有例行工作，例如觸發它的 [GitHub 事件](/docs/zh-TW/routines#add-a-github-trigger)。`body` 命名來源、事件和要觸發的例行工作。需要 Claude Code v2.1.225 或更新版本。

`list_runs` 列出例行工作的最近執行，`get_run_log` 讀取一個執行的日誌。`session_id` 命名要讀取的執行，來自 `list_runs` 結果，`cursor` 透過任一動作的結果進行分頁。兩個動作都需要 Claude Code v2.1.227 或更新版本。

此工具僅在工作階段使用啟用例行工作的計畫的 claude.ai 帳戶進行驗證時可用，當您的組織政策停用[網路上的 Claude Code](/docs/zh-TW/claude-code-on-the-web) 時不存在。在 Claude Code v2.1.227 或更新版本上，當所有者[為組織關閉例行工作](/docs/zh-TW/routines#routines-are-disabled-by-your-organizations-policy)時，該工具也不存在。在 v2.1.227 之前，僅關閉例行工作切換的工作階段仍顯示該工具，伺服器拒絕其呼叫。

<h3 id="pushnotification">
  PushNotification
</h3>

**工具名稱：** `PushNotification`

```typescript theme={null}
type PushNotificationInput = {
  message: string;
  status: "proactive";
};
```

向使用者傳送主動推播通知。將 `message` 保持在 200 個字元以下，因為行動作業系統會截斷較長的文字。詳見[工具參考中的 PushNotification 列](/docs/zh-TW/tools-reference)以了解提供者可用性；推播傳遞透過 Anthropic 託管的基礎設施執行，無法從 Amazon Bedrock、AWS 上的 Claude Platform、Google Cloud 的 Agent Platform 或 Microsoft Foundry 存取。

<h3 id="repl">
  REPL
</h3>

在 v2.1.275 中移除。透過 v2.1.274，實驗性 `REPL` 工具可以透過在 [`env` 選項](#options)中設定 `CLAUDE_CODE_REPL=1` 來開啟。

<h3 id="reportfindings">
  ReportFindings
</h3>

**工具名稱：** `ReportFindings`

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

將程式碼審查發現報告為結構化清單，以便 Claude Code 可以呈現它們而不是將其列印為文字。`level` 是審查執行的工作量級別。發現按最嚴重優先排序，每次呼叫最多 32 個，當沒有倖存時陣列為空。需要 Claude Code v2.1.196 或更新版本。

每個發現包含這些欄位：

* `file`：發現所在的儲存庫相對路徑。選擇性 `line` 是它錨定到的 1 索引行。
* `summary`：缺陷的單句陳述。`failure_scenario` 描述導致錯誤輸出或當機的具體輸入和狀態。
* `short_summary`：選擇性的最多 60 個字元的壓縮標籤，用於緊湊顯示。需要 Claude Code v2.1.212 或更新版本。
* `category`：選擇性的發現類型的短 kebab-case slug，例如 `correctness` 或 `test-coverage`。需要 Claude Code v2.1.199 或更新版本。
* `verdict`：在驗證通過執行時設定；在僅內嵌審查中不存在。
* `outcome`：僅在應用修復後重新報告時設定。

<h3 id="artifact">
  Artifact
</h3>

**工具名稱：** `Artifact`

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

將本機 `.html` 或 `.md` 檔案發佈為託管成品頁面，或列出使用者的已發佈成品。省略 `action` 或傳遞 `"publish"` 以發佈 `file_path`，這對於發佈動作為必需。每個以下欄位適用於發佈：

* `icon`：成品瀏覽器標籤圖示的一個短通用詞，例如 `chart` 或 `map`。Claude 在首次發佈時包含它，在更新時省略它，這會保留成品的儲存圖示。
* `favicon`：已棄用，Claude 會省略它。
* `title`：當 HTML 檔案沒有 `<title>` 標籤時，在瀏覽器標籤和圖庫中命名已發佈頁面。
* `url`：以現有成品為目標以就地更新，而不是建立新的。

`force` 是最後手段的覆蓋，捨棄另一個工作階段發佈的較新版本。發生衝突時，失敗的發佈會傳回較新的內容；Claude 將其變更合併到該內容上，或重新讀取成品，然後再次發佈。僅當使用者明確要求捨棄該版本時才傳遞 `force`。

傳遞 `"list"` 以列舉使用者的已發佈成品；只有 `limit` 和 `scope` 可能伴隨它。`scope` 預設為 `"mine"`，列出使用者擁有的成品；`"shared"` 列出其他人與使用者共享的成品，`"all"` 列出兩者。

* `capabilities`：已發佈頁面使用的執行時功能，按功能名稱鍵入，例如[頁面可能呼叫的連接器](/docs/zh-TW/artifacts#pull-live-data-with-mcp-connectors)。成品服務驗證宣告並拒絕命名帳戶無法使用的功能或給予無效設定的發佈。傳遞 `{}` 以清除儲存的宣告，並在重新部署時省略欄位以保留它。需要 Agent SDK v0.3.235 或更新版本。
* `contract`：已發佈頁面執行的執行時版本。省略它以保留成品的目前版本，傳遞 `"latest"` 以升級，或傳遞特定版本以釘選或回滾。需要 Agent SDK v0.3.235 或更新版本。

類型已匯出，但該工具在 Agent SDK 工作階段中預設關閉。發佈還需要[成品可用性表](/docs/zh-TW/artifacts#availability)中的每個條件，使用 API 金鑰驗證的工作階段不符合。

<h3 id="projects">
  Projects
</h3>

**工具名稱：** `Projects`

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

讀取和寫入附加到工作階段的 claude.ai 專案。按 `method` 分派：

* `project_info`：傳回專案中繼資料和文件清單。
* `project_read`：按 `path` 讀取一個文件。
* `project_search`：使用 `query` 查詢專案的知識庫。`n` 限制點擊數，預設為 5。
* `project_write`：在 `path` 建立或取代文件，來自 `content`（帶有內嵌文字）或 `local_path`（命名工作目錄內的檔案）中的恰好一個。`present_to_user: true` 將寫入的文件標記為使用者需要查看的可交付成果。
* `project_delete`：按 `path` 刪除文件。

<h3 id="readmcpresourcedir">
  ReadMcpResourceDir
</h3>

**工具名稱：** `ReadMcpResourceDirTool`

```typescript theme={null}
type ReadMcpResourceDirInput = {
  server: string;
  uri: string;
};
```

列出 MCP 伺服器上目錄資源的直接子項。僅可用於已宣告支援目錄列表的伺服器；列表不是遞迴的。目錄列表並非在每個工作階段都啟用：當關閉時，呼叫傳回空的 `resources` 清單，`error` 欄位報告目錄列表未啟用。

<h3 id="refreshmcptools">
  RefreshMcpTools
</h3>

**工具名稱：** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsInput = {
  server?: string; // refresh only this server; omit to refresh all connected servers
};
```

重新查詢已連接 MCP 伺服器的工具清單並應用任何變更。類型已匯出，但 Claude Code 僅在您在 [`env` 選項](#options)中設定 `CLAUDE_CODE_ENABLE_REFRESH_MCP_TOOLS=1` 時註冊該工具，並且僅在至少有一個 MCP 伺服器的工作階段中。需要 Claude Code v2.1.211 或更新版本。

<h3 id="showonboardingrolepicker">
  ShowOnboardingRolePicker
</h3>

**工具名稱：** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerInput = {};
```

在 Cowork 上線期間呈現可點擊的角色選擇器晶片列，以便使用者可以選擇其角色並取得相符的外掛程式安裝。不帶任何引數；角色清單由用戶端定義。呼叫會阻止直到使用者回應。

<h3 id="mcpinput">
  McpInput
</h3>

**工具名稱：** `mcp__<server>__<tool>` 形式的動態 MCP 工具名稱

```typescript theme={null}
type McpInput = {
  [k: string]: unknown;
};
```

MCP 工具引數是開放物件：每個伺服器定義自己的參數，因此類型對欄位名稱或值不施加任何限制。請查閱伺服器自己的工具架構以了解特定工具接受的欄位。

<h2 id="tool-output-types">
  工具輸出類型
</h2>

所有內建 Claude Code 工具的輸出架構文件。這些類型從 `@anthropic-ai/claude-agent-sdk` 匯出，代表每個工具傳回的實際回應資料。

<h3 id="tooloutputschemas">
  `ToolOutputSchemas`
</h3>

從 `@anthropic-ai/claude-agent-sdk` 匯出的工具輸出類型聯合；成員包括：

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

**工具名稱：** `Agent`。先前的名稱 `Task` 仍被接受為別名，且 [`SDKSystemMessage`](#sdksystemmessage) 初始化訊息中的 `tools` 陣列目前仍將此工具列為 `Task` 以保持向後相容性。

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

傳回子代理的結果。根據 `status` 欄位進行區分：`"completed"` 表示已完成的工作，`"async_launched"` 表示背景工作，`"remote_launched"` 表示 Claude Code 分派到遠端雲端工作階段的工作，其中 `sessionUrl` 連結到該工作階段，`taskId` 識別它。

在 `completed` 變體上，`resolvedModel` 命名子代理啟動時使用的模型，當套用 [`availableModels`](/docs/zh-TW/model-config#restrict-model-selection) 或其他覆蓋時，可能與要求的 `model` 輸入不同。此欄位需要 Claude Code v2.1.174 或更新版本。在 `async_launched` 上，它命名工作移至背景時使用的模型。

`modelsUsed` 列出子代理使用的模型，按順序排列。此欄位僅在發生中途交換時出現，當執行交換回該模型時，模型會再次出現。在 `async_launched` 上，列表涵蓋背景化前使用的模型。`modelsUsed` 和 `resolvedModel` 的背景化行為都需要 Claude Code v2.1.212 或更新版本。

如果 Claude Code [保留了子代理的隔離 worktree](/docs/zh-TW/worktrees#isolate-subagents-with-worktrees)，`completed` 結果上的 `worktreePath` 是找到它的位置。`worktreeBranch` 是其分支，當 Claude Code 使用 git 建立 worktree 時出現。

Claude Code 從子代理的最終 API 要求填充 `usage` 和 `totalTokens`，而不是從整個執行，所以 `usage.service_tier` 是 API 在該要求上報告的服務層級字串。當存在時，`usage.output_tokens_details.thinking_tokens` 是該要求的輸出令牌中作為思考令牌的數量。`output_tokens_details` 欄位需要 TypeScript SDK v0.3.228 或更新版本，該版本包含 Claude Code v2.1.228。

`usage.output_tokens_details` 在意義上與 [`Usage.output_tokens_details`](#usage) 相符，範圍限於該最終要求，但其每個層級都是選擇性的。保護物件和欄位，例如 `usage.output_tokens_details?.thinking_tokens ?? 0`，而不是直接讀取它。

在 v2.1.207 之前，發佈的類型更窄。它省略了 `worktreePath`、`worktreeBranch`、`citations`、`toolStats.frameCount` 和 `inference_geo`、`speed` 和 `iterations` 使用欄位，並將 `service_tier` 類型化為 `"standard" | "priority" | "batch"`。類型標記為選擇性的欄位可能在較早版本記錄的結果中不存在。

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
  annotations?: Record<string, { preview?: string; notes?: string }>;
  afkTimeoutMs?: number;
};
```

傳回提出的問題和使用者的答案。當使用者輸入自由格式回覆而不是回答結構化問題時，`response` 會被設定；當存在時，Claude 會收到「使用者回應：…」而不是每個問題的答案清單。

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

`stdout`、`stderr` 和 `backgroundTaskId` 欄位攜帶：

| 欄位                 | 它攜帶的內容                                 |
| ------------------ | -------------------------------------- |
| `stdout`           | 命令的 stdout 和 stderr，合併為一個交錯流           |
| `stderr`           | 工具本身新增的通知，例如 shell 工作目錄重設，不是命令的 stderr |
| `backgroundTaskId` | 對於背景命令存在                               |

`timedOutAfterMs` 是逾時（以毫秒為單位），當命令達到其逾時並移至背景而不是明確從那裡開始時設定。`backgroundCwdHint` 在背景化命令包含目錄變更內建函式（例如 `cd`、`pushd`、`popd` 或 `chdir`）時設定，並注意工作階段工作目錄未變更。兩個欄位都需要 Claude Code v2.1.210 或更新版本。

當在前景執行的子代理擁有背景化命令時，Claude Code 會在該子代理給出最終回應時終止命令。Claude Code 在此類命令上將 `backgroundEndsWithFinalResponse` 設定為 `true`，並在命令存活該輪時省略欄位，如主對話或背景子代理啟動的命令一樣。此欄位需要 Claude Code v2.1.227 或更新版本。

Claude Code 將 `gitOperation.commit.branch` 設定為 git 提交摘要行中命名的分支，並對在分離 HEAD 上進行的提交省略它。此欄位需要 Agent SDK v0.3.227 或更新版本。Claude Code 將 `gh pr reopen` 命令報告為 `reopened` PR 動作，需要 Agent SDK v0.3.234 或更新版本。

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

傳回執行中監視器的背景工作 ID。使用此 ID 搭配 `TaskStop` 以提前取消監視。

<h3 id="edit-2">
  Edit
</h3>

**工具名稱：** `Edit`

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

傳回編輯操作的結構化差異。

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
        /** 當整個檔案讀取因超過令牌上限而自動分頁時為真（內容是部分第一頁）。 */
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
      /** 第一個提取頁面的文件頁碼；標記工具結果內容中的頁面影像。 */
      firstPage?: number;
      /** 僅在程序中：頁面影像位元組作為影像區塊在工具結果內容中傳遞，不會保留在發出的 tool_use_result 上，所以此鍵在那裡不存在。 */
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
      /** 當重複資料刪除符合啟動時播種的項目（CLAUDE.md / 巢狀記憶）而不是先前的 Read tool_result 時設定。 */
      source?: "seeded";
    };
```

以適合檔案類型的格式傳回檔案內容。根據 `type` 欄位進行區分。

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
    repository?: string | null;
  };
  userModified?: boolean;
};
```

傳回寫入結果及結構化差異資訊。`originalFile` 和 `structuredPatch` 持有的內容取決於寫入：

* 對於新建立的檔案，`originalFile` 為 null，`structuredPatch` 為空
* 在覆蓋時，`originalFile` 攜帶先前的內容，除非該內容大於約 10 MB：Claude Code 會跳過差異並傳回 `originalFile` null 和 `structuredPatch` 空
* 當寫入未變更任何內容或差異逾時時，`structuredPatch` 也為空

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
  totalMatches?: number;
  countIsComplete?: boolean;
};
```

傳回符合 glob 模式的檔案路徑，按修改時間排序。

`totalMatches` 和 `countIsComplete` 需要 Claude Code v2.1.191 或更新版本。`totalMatches` 報告截斷前的符合檔案數。當 `countIsComplete` 為 false 時，`totalMatches` 是下限，因為基礎搜尋截斷了自己的輸出。

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
  totalFiles?: number;
  totalLines?: number;
  appliedLimit?: number;
  appliedOffset?: number;
};
```

傳回搜尋結果。形狀因 `mode` 而異：檔案清單、包含符合的內容或符合計數。在 `count` 模式中，`numFiles` 和 `numMatches` 是完整結果集上的總計，不是分頁切片。在 v2.1.208 之前，截斷列出項目的 `head_limit` 或 `offset` 也會截斷這些總計。

`totalFiles` 需要 Claude Code v2.1.208 或更新版本，並在 `files_with_matches` 模式中報告 `head_limit` 和 `offset` 分頁前的結果總數。`totalLines` 需要 Claude Code v2.1.210 或更新版本，並在 `content` 模式中報告分頁前的行總數。

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

傳回停止背景工作後的確認。

<h3 id="notebookedit-2">
  NotebookEdit
</h3>

**工具名稱：** `NotebookEdit`

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

傳回筆記本編輯的結果及原始和更新的檔案內容。

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
  artifactRead?: {
    slug: string;
    ver?: string;
    seeded?: false;
  };
};
```

傳回提取的內容及 HTTP 狀態和中繼資料。

`artifactRead` 是 Claude Code 自己的成品讀取記錄，僅當 Claude 提取工作階段可以發佈的成品時出現。Claude Code 在工作階段恢復時讀取它回來，以便稍後發佈基於正確版本；您的程式碼不需要對其採取行動。`slug` 命名成品，`ver` 是讀取記錄的版本，當它未記錄任何內容時不存在，`seeded: false` 標記其完整來源未到達 Claude 的讀取。`seeded` 欄位需要 Agent SDK v0.3.239 或更新版本。

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
  searchCount?: number;
};
```

傳回來自網路的搜尋結果。

<h3 id="workflow-2">
  Workflow
</h3>

**工具名稱：** `Workflow`

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
  sessionUrl?: string; // 當工作流程作為遠端工作階段啟動時設定
  warning?: string;
  error?: string;
};
```

在工具接受呼叫後立即傳回。最終結果稍後作為工作完成到達。在將執行視為已啟動之前檢查 `error`：語法檢查失敗的指令碼傳回 `status: "async_launched"` 並設定 `error`，且永遠不會執行。

| 欄位              | 類型                                      | 描述                                                                                      |
| --------------- | --------------------------------------- | --------------------------------------------------------------------------------------- |
| `status`        | `"async_launched" \| "remote_launched"` | 工具接受了呼叫。`"async_launched"` 用於程序內執行，`"remote_launched"` 用於分派到遠端工作階段而不是在程序內執行的執行          |
| `taskId`        | `string`                                | 執行的背景工作識別碼                                                                              |
| `taskType`      | `"local_workflow" \| "remote_agent"`    | 已註冊背景工作的工作類型，符合 `status` 分支                                                             |
| `workflowName`  | `string`                                | 工作流程指令碼中的 `meta.name`                                                                   |
| `runId`         | `string`                                | 工作流程執行識別碼，在稍後呼叫時作為 `resumeFromRunId` 傳遞。對於 `remote_launched` 執行不存在，其中雲端工作階段 URL 是恢復控制代碼 |
| `summary`       | `string`                                | 工作流程功能的單行描述                                                                             |
| `transcriptDir` | `string`                                | 執行期間寫入子代理文字記錄的目錄                                                                        |
| `scriptPath`    | `string`                                | 此執行的持久化工作流程指令碼路徑。編輯它並作為 `scriptPath` 傳回以重新執行而不重新傳送指令碼                                   |
| `sessionUrl`    | `string`                                | 雲端工作階段 URL，當 `status` 為 `"remote_launched"` 時設定                                         |
| `warning`       | `string`                                | 非阻止性提醒，例如本機 git 狀態與雲端工作階段將複製的推送分支不同                                                     |
| `error`         | `string`                                | 當指令碼語法檢查失敗時設定。當存在時，儘管啟動狀態，執行未啟動                                                         |

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

傳回先前和更新的工作清單。

<Note>
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  請參閱[模型可用性](/docs/zh-TW/agent-sdk/todo-tracking#model-availability)以選擇加入。
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

傳回建立的工作及其指派的 ID。

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

傳回更新結果，包括哪些欄位已變更。

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

傳回完整工作記錄，或當找不到 ID 時傳回 `null`。

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

傳回目前清單中所有工作的快照。

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
  planWasEdited?: boolean;
  awaitingLeaderApproval?: boolean;
  requestId?: string;
};
```

傳回退出計畫模式後的計畫狀態。

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

傳回可用 MCP 資源的陣列。

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
    blobSavedTo?: string;
  }>;
  error?: string;
};
```

傳回要求的 MCP 資源的內容。

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

傳回關於 git worktree 的資訊。

<h3 id="exitworktree-2">
  ExitWorktree
</h3>

**工具名稱：** `ExitWorktree`

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

傳回採取的動作和已退出 worktree 的詳細資訊。

<h3 id="enterplanmode-2">
  EnterPlanMode
</h3>

**工具名稱：** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeOutput = {
  message: string;
};
```

傳回進入計畫模式的確認。

<h3 id="croncreate-2">
  CronCreate
</h3>

**工具名稱：** `CronCreate`

```typescript theme={null}
type CronCreateOutput = {
  id: string;
  humanSchedule: string;
  recurring: boolean;
  durable?: boolean; // 當持久化到 .claude/scheduled_tasks.json 時為真；當僅限工作階段時為假
};
```

傳回工作 ID 和排程的人類可讀描述。

<h3 id="crondelete-2">
  CronDelete
</h3>

**工具名稱：** `CronDelete`

```typescript theme={null}
type CronDeleteOutput = {
  id: string;
};
```

傳回已刪除工作的 ID。

<h3 id="cronlist-2">
  CronList
</h3>

**工具名稱：** `CronList`

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

傳回排程的 cron 工作：來自 `.claude/scheduled_tasks.json` 的持久化工作和來自目前工作階段的僅限工作階段工作。僅限工作階段的工作攜帶 `durable: false`；從磁碟讀取的工作省略欄位。

<h3 id="schedulewakeup-2">
  ScheduleWakeup
</h3>

**工具名稱：** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupOutput = {
  scheduledFor: number;
  clampedDelaySeconds: number;
  wasClamped: boolean;
  stopped?: boolean;
  cancelledWakeups?: number;
};
```

傳回喚醒將觸發的時間（作為紀元毫秒時間戳記）、實際使用的延遲以及要求的延遲是否被限制。`stopped` 欄位在呼叫以 `stop: true` 結束迴圈時為 `true`。它需要 Claude Code v2.1.202 或更新版本。`cancelledWakeups` 欄位計算 `stop: true` 呼叫取消了多少個待處理喚醒。值 0 表示沒有待處理，重複 `/loop` cron 不會被 `stop: true` 取消。它需要 Claude Code v2.1.206 或更新版本。

<h3 id="remotetrigger-2">
  RemoteTrigger
</h3>

**工具名稱：** `RemoteTrigger`

```typescript theme={null}
type RemoteTriggerOutput = {
  status: number;
  json: string;
  summary?: string;
};
```

傳回觸發操作的 API 回應狀態和本體。

<h3 id="pushnotification-2">
  PushNotification
</h3>

**工具名稱：** `PushNotification`

```typescript theme={null}
type PushNotificationOutput = {
  message: string;
  pushSent?: boolean;
  localSent?: boolean;
  disabledReason?: "config_off" | "user_present" | "no_transport";
  sentAt?: string;
};
```

傳回傳遞詳細資訊，包括是否傳送了推送或本機通知以及為什麼跳過傳遞。

<h3 id="reportfindings-2">
  ReportFindings
</h3>

**工具名稱：** `ReportFindings`

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

傳回報告的發現數、審查執行的工作量級別以及為結果本體回顯的發現。需要 Claude Code v2.1.196 或更新版本。回顯的 `short_summary` 欄位需要 Claude Code v2.1.212 或更新版本。

<h3 id="artifact-2">
  Artifact
</h3>

**工具名稱：** `Artifact`

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

傳回已發佈頁面的 `url` 和為發佈動作發佈的本機 `path`，當發佈重新部署現有成品時 `updated` 設定為 true，`warnings` 攜帶任何發佈時間建議。清單動作改為傳回 `artifacts` 列，當存在超過要求限制的成品時 `truncated` 設定。在其範圍不是 `"mine"` 的清單上，每列攜帶 `rel` 標記使用者是否擁有成品或與他們共享，輸出的 `scope` 記錄哪個非預設範圍產生清單；兩者在預設清單上不存在。

<h3 id="projects-2">
  Projects
</h3>

**工具名稱：** `Projects`

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

根據 `method` 欄位進行區分，鏡像輸入。`project_read` 在 `content` 中內聯傳回小文字文件，並改為將較大文件寫入 `local_file` 路徑；`project_search` 當專案的索引可用時傳回 RAG `hits` 並設定 `rag: true`，否則回退到 `docs` 路徑清單。

<h3 id="readmcpresourcedir-2">
  ReadMcpResourceDir
</h3>

**工具名稱：** `ReadMcpResourceDirTool`

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

傳回目錄資源的直接子項。子目錄以 mimeType `"inode/directory"` 出現；`error` 在伺服器無法列出目錄時攜帶人類可讀的訊息。

<h3 id="refreshmcptools-2">
  RefreshMcpTools
</h3>

**工具名稱：** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsOutput = Array<{
  server: string;
  status: "refreshed" | "error" | "not_connected";
  toolCount?: number; // 此伺服器現在可用的工具
  added?: string[]; // 此重新整理新增的工具名稱
  removed?: string[]; // 此重新整理移除的工具名稱
  error?: string; // 重新整理失敗或伺服器無法使用的原因
}>;
```

傳回每個伺服器一個項目：`refreshed` 表示已套用重新查詢的工具清單，`error` 表示重新查詢失敗且保留了先前的工具集，`not_connected` 表示伺服器沒有即時連線可查詢。

<h3 id="showonboardingrolepicker-2">
  ShowOnboardingRolePicker
</h3>

**工具名稱：** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerOutput = {
  role?: string;
  dismissed?: boolean;
};
```

傳回使用者的選擇：當他們選擇角色晶片或輸入一個時為 `role`，當他們關閉選擇器時為 `dismissed: true`。空物件表示使用者批准了呼叫而未選擇角色。

<h3 id="mcpoutput">
  McpOutput
</h3>

**工具名稱：** 形式為 `mcp__<server>__<tool>` 的動態 MCP 工具名稱

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

MCP 工具結果根據伺服器傳回為字串或內容區塊陣列。匯出類型中的尾部純物件分支是架構產生成品：SDK 不傳回裸物件，因為伺服器的結構化輸出在傳回前被序列化為 JSON 字串。在執行時值也可能是 `undefined`，儘管匯出的類型不對此進行建模。

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

工作階段請求的 API 金鑰來源，在 [`SDKSystemMessage`](#sdksystemmessage) 初始化訊息上報告為 `apiKeySource`。

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

Claude Code 報告四個值之一：

| 值                    | 使用中的金鑰                                                                                              |
| -------------------- | --------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_API_KEY`  | `ANTHROPIC_API_KEY` 環境變數中的金鑰                                                                        |
| `apiKeyHelper`       | 您的 [`apiKeyHelper`](/docs/zh-TW/settings-reference#apikeyhelper) 命令傳回的金鑰                                 |
| `/login managed key` | 當您使用 [Claude Console 帳戶](/docs/zh-TW/authentication#claude-console-authentication) 登入時，Claude Code 儲存的金鑰 |
| `none`               | 沒有 API 金鑰。工作階段以其他方式進行驗證，例如 claude.ai 登入、持有人令牌或雲端提供者                                                 |

Agent SDK v0.3.234 及更新版本在類型中列出這四個值。該類型也保留 `user`、`project`、`org`、`temporary` 和 `oauth`，以便舊程式碼仍能編譯，而 Claude Code 不報告它們。

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

可透過 `betas` 選項啟用的可用測試版功能。如需詳細資訊，請參閱 [Beta headers](https://platform.claude.com/docs/en/api/beta-headers)。

```typescript theme={null}
type SdkBeta = "context-1m-2025-08-07";
```

<Warning>
  `context-1m-2025-08-07` 測試版自 2026 年 4 月 30 日起已停用。使用 Claude Sonnet 4.5 或 Sonnet 4 傳遞此值無效，超過標準 200k 令牌內容視窗的請求會傳回錯誤。若要使用 1M 令牌內容視窗，請遷移至 [Claude Opus 5.5、Claude Opus 5、Claude Sonnet 5、Claude Sonnet 4.6、Claude Opus 4.6、Claude Opus 4.7 或 Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview)，這些模型在標準定價下包含 1M 內容，無需測試版標頭。
</Warning>

<h3 id="slashcommand">
  `SlashCommand`
</h3>

有關可用命令的資訊。

```typescript theme={null}
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
  aliases?: string[];
  builtin?: boolean;
};
```

`builtin` 在命令是 Claude Code 自身且輸入 `/name` 執行它的列上為 `true`。它對由使用者、專案、plugin 或 MCP 伺服器定義的命令不存在，以及對其中一個 [按名稱替換](/docs/zh-TW/skills#resolve-skills-that-share-a-name) 的捆綁命令不存在。需要 Agent SDK v0.3.277 或更新版本。

<h3 id="modelinfo">
  `ModelInfo`
</h3>

有關可用模型的資訊。

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

| 欄位                         | 類型                                                                 | 說明                                                                                                                                           |
| :------------------------- | :----------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------- |
| `value`                    | `string`                                                           | 在 API 呼叫中傳遞的模型識別碼                                                                                                                            |
| `resolvedModel`            | `string \| undefined`                                              | 此項目的 `value` 解析為的規範線路模型 ID。別名項目（例如 `sonnet`）解析為明確的模型 ID（例如 `claude-sonnet-5`），因此主機可以將儲存的明確模型 ID 與涵蓋它的別名項目進行比對。需要 Claude Code v2.1.197 或更新版本。 |
| `displayName`              | `string`                                                           | 人類可讀的顯示名稱                                                                                                                                    |
| `description`              | `string`                                                           | 模型功能的說明                                                                                                                                      |
| `supportsEffort`           | `boolean \| undefined`                                             | 此模型是否支援工作量級別                                                                                                                                 |
| `supportedEffortLevels`    | `("low" \| "medium" \| "high" \| "xhigh" \| "max")[] \| undefined` | 此模型接受的工作量級別                                                                                                                                  |
| `supportsAdaptiveThinking` | `boolean \| undefined`                                             | 此模型是否支援自適應思考，其中 Claude 決定何時以及思考多少                                                                                                            |
| `supportsFastMode`         | `boolean \| undefined`                                             | 此模型是否支援快速模式                                                                                                                                  |
| `supportsAutoMode`         | `boolean \| undefined`                                             | 此模型是否支援自動模式                                                                                                                                  |

<h3 id="agentinfo">
  `AgentInfo`
</h3>

有關可透過 Agent 工具叫用的可用子代理的資訊。

```typescript theme={null}
type AgentInfo = {
  name: string;
  description: string;
  model?: string;
};
```

| 欄位            | 類型                    | 說明                                                                                                                        |
| :------------ | :-------------------- | :------------------------------------------------------------------------------------------------------------------------ |
| `name`        | `string`              | 代理類型識別碼（例如 `"Explore"`、`"general-purpose"`）                                                                               |
| `description` | `string`              | 何時使用此代理的說明                                                                                                                |
| `model`       | `string \| undefined` | 此代理使用的模型：別名或模型 ID，或 `'inherit'` 表示父代的模型。當為 `undefined` 時，Claude Code 會在 [子代理模型順序](/docs/zh-TW/sub-agents#choose-a-model) 中選擇模型 |

<h3 id="mcpserverprovenance">
  `McpServerProvenance`
</h3>

提供 `mcp__*` 工具的 MCP 伺服器，以及該伺服器定義的來源。[`PreToolUse`](#pretoolusehookinput)、`PostToolUse`、`PostToolUseFailure`、`PermissionRequest` 和 `PermissionDenied` hook 輸入將其作為 `mcp_server` 攜帶，[`CanUseTool`](#canusetool) 選項將其作為 `mcpServer` 攜帶。對於不來自 MCP 伺服器的工具，兩者都省略它。

```typescript theme={null}
type McpServerProvenance = {
  name: string;
  source: string;
};
```

| 欄位       | 類型       | 說明                                                          |
| :------- | :------- | :---------------------------------------------------------- |
| `name`   | `string` | 伺服器註冊時使用的名稱，與 [`mcpServerStatus()`](#query-object) 為其報告的值相同 |
| `source` | `string` | 伺服器定義的來源：`sdk`、`plugin` 或設定範圍                               |

`source` 採用以下值之一。該集合是開放的，因此將您不認識的值視為已設定的來源，絕不視為 `sdk`：

* **`sdk`**：您的應用程式註冊的進程內伺服器。只有 SDK 主機應用程式可以註冊一個，因此已設定的伺服器絕不報告 `sdk`，無論其名稱如何。
* **`plugin`**：[plugin](/docs/zh-TW/agent-sdk/plugins) 提供的伺服器。其 `name` 是 [plugin 提供的 MCP 伺服器](/docs/zh-TW/mcp#plugin-provided-mcp-servers) 下所述的範圍 `plugin:<plugin-name>:<server-name>` 形式。
* **設定範圍**：`user`、`project`、`local`、`dynamic`、`managed`、`enterprise`、`claudeai` 或 `agent`。`.mcp.json` 伺服器報告 `project`，[MCP 安裝範圍](/docs/zh-TW/mcp#mcp-installation-scopes) 定義 `local`、`project` 和 `user`。您的應用程式在 [`mcpServers` 選項](#options) 中傳遞的伺服器（除了進程內 SDK 伺服器外）報告 `dynamic`。

基於 `source` 而非 `name` 或 `mcp__<server>__` 工具名稱前綴做出信任決定。對於除 `sdk` 以外的任何來源，`name` 是不受信任的文字：在顯示前逸出它。

`McpServerProvenance` 和攜帶它的欄位需要 Agent SDK v0.3.274 或更新版本。

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

已連線 MCP 伺服器的狀態。

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

`source` 說明伺服器定義的來源，具有與 [`McpServerProvenance`](#mcpserverprovenance) 的 `source` 相同的值和信任規則。該欄位需要 Agent SDK v0.3.274 或更新版本，在較早版本上不存在。

`_meta` 在 `tools` 項目上攜帶該工具的 `_meta` 的 MCP Apps 成員，因此您的應用程式可以找到 `ui://` 資源以使用 [`readMcpResource()`](#query-object) 呈現。Claude Code 傳遞 `ui` 物件和已棄用的平面 `ui/resourceUri` 字串，並保留所有其他金鑰。在 `ui` 內，`resourceUri` 是 `ui://` 字串，`visibility` 是當伺服器設定時 `"model"` 和 `"app"` 的陣列，任何其他成員原封不動地傳遞。Claude Code 在值格式不正確時捨棄任一金鑰，並從未宣告任一金鑰的工具中省略 `_meta`。該欄位僅在初始化訊息的 [`capabilities`](#sdksystemmessage) 包含 `mcp_tool_ui_meta_v1` 時存在，並需要 TypeScript Agent SDK v0.3.280 或更新版本。

<h3 id="mcpserverstatusconfig">
  `McpServerStatusConfig`
</h3>

由 `mcpServerStatus()` 報告的 MCP 伺服器的設定。這是所有 MCP 伺服器傳輸類型的聯合。

```typescript theme={null}
type McpServerStatusConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfig
  | McpClaudeAIProxyServerConfig;
```

如需每個傳輸類型的詳細資訊，請參閱 [`McpServerConfig`](#mcpserverconfig)。

<h3 id="accountinfo">
  `AccountInfo`
</h3>

已驗證使用者的帳戶資訊。

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

結果訊息中傳回的每個模型使用統計資訊。`costUSD` 值是用戶端估計。如需計費注意事項，請參閱 [追蹤成本和使用](/docs/zh-TW/agent-sdk/cost-tracking)。

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

`thinkingTokens` 計算此模型產生的思考令牌。`outputTokens` 已包含它們，因此不要將兩者相加。該欄位在輪次在記錄它的 Claude Code 版本上執行之前不存在，因此在較早版本上開始的已恢復工作階段會報告部分計數。`thinkingTokens` 需要 Agent SDK v0.3.257 或更新版本。

`canonicalModel` 和 `provider` 欄位需要 Claude Code v2.1.218 或更新版本。`canonicalModel` 是定價查詢使用的規範模型 ID；它可能與鍵入項目的原始模型字串不同，例如當該字串是提供者特定 ID 或別名時。

`provider` 命名提供模型的 API 後端，例如 `firstParty`、`bedrock`、`vertex`、`foundry`、`anthropicAws`、`mantle` 或 `gateway`。

`costBasis` 命名為模型最新請求定價的價格表：`list` 表示清單價格，`managed` 表示 [`modelPricing`](/docs/zh-TW/settings-reference#modelpricing) 表，或 `unknown` 表示模型 ID 都不符合時。該欄位需要 Claude Code v2.1.246 或更新版本。

<h3 id="configscope">
  `ConfigScope`
</h3>

```typescript theme={null}
type ConfigScope = "local" | "user" | "project";
```

<h3 id="nonnullableusage">
  `NonNullableUsage`
</h3>

[`Usage`](#usage) 的版本，所有可為空的欄位都變為不可為空。

```typescript theme={null}
type NonNullableUsage = {
  [K in keyof Usage]: NonNullable<Usage[K]>;
};
```

<h3 id="usage">
  `Usage`
</h3>

令牌使用統計資訊。這是來自 `@anthropic-ai/sdk` 的 `BetaUsage` 類型。

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

`BetaServerToolUsage`、`BetaIterationsUsage` 和 `BetaOutputTokensDetails` 在 `@anthropic-ai/sdk` 中定義。

`output_tokens_details` 按類別分解計費輸出。它目前攜帶一個欄位 `thinking_tokens: number`，計算模型產生的輸出令牌作為內部推理，包括思考區塊分隔符。`output_tokens_details` 欄位需要 TypeScript SDK v0.3.228 或更新版本，該版本捆綁 Claude Code v2.1.228。

* **計費**：讀取分解以進行可觀測性，而非計費。`output_tokens` 保持為權威總計，`output_tokens - thinking_tokens` 近似非推理輸出。
* **計數涵蓋的內容**：模型產生的原始推理，可能比回應正文中傳回的思考文字更長。API 透過重新令牌化該原始文字來計算它，因此它可能與模型的確切生成計數相差幾個令牌。
* **串流**：在串流助手訊息上，此分解（如 `output_tokens`）是 `message_start` 預留位置，不攜帶實際計數，因此從結果訊息的 `usage` 讀取它，如 [從結果訊息讀取輸出令牌](/docs/zh-TW/agent-sdk/cost-tracking#read-output-tokens-from-the-result-message) 所述。在結果訊息上，當模型或提供者不報告分解時，`thinking_tokens` 讀取 `0`。
* **`null` 情況**：`output_tokens_details` 本身在 Claude Code 合成的助手訊息上為 `null`，例如 API 錯誤訊息。

<h3 id="calltoolresult">
  `CallToolResult`
</h3>

MCP 工具結果類型（來自 `@modelcontextprotocol/sdk/types.js`）。`structuredContent` 是可與 `content` 一起傳回的 JSON 物件，包括影像區塊。請參閱 [傳回結構化資料](/docs/zh-TW/agent-sdk/custom-tools#return-structured-data)。

```typescript theme={null}
type CallToolResult = {
  content: Array<{
    type: "text" | "image" | "audio" | "resource" | "resource_link";
    // 其他欄位因類型而異
  }>;
  structuredContent?: Record<string, unknown>;
  isError?: boolean;
};
```

<h3 id="sdkmcpresourcelink">
  `SDKMcpResourceLink`
</h3>

MCP 工具透過參考傳回的一個檔案。Claude Code 從工具結果中的 `resource_link` 區塊建立每個項目，並將清單作為 [`SDKUserMessage.tool_use_result`](#sdkusermessage) 上的 `resourceLinks` 傳遞，或在呼叫在背景完成時作為 [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) 上的 `resource_links` 傳遞。需要 Agent SDK v0.3.257 或更新版本。

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

Claude Code 會捨棄 `uri` 或 `name` 不是字串的區塊，並省略其值不是列出類型的可選欄位。

| 欄位            | 類型                                     | 說明                   |
| :------------ | :------------------------------------- | :------------------- |
| `uri`         | `string`                               | 資源的 URI，如伺服器傳回的      |
| `name`        | `string`                               | 伺服器給予資源的名稱           |
| `title`       | `string \| undefined`                  | 顯示標題，當伺服器設定時         |
| `description` | `string \| undefined`                  | 說明，當伺服器設定時           |
| `mimeType`    | `string \| undefined`                  | MIME 類型，當伺服器設定時      |
| `size`        | `number \| undefined`                  | 大小（以位元組為單位），當伺服器設定時  |
| `annotations` | `Record<string, unknown> \| undefined` | 區塊的 MCP 註解物件，當伺服器設定時 |

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

控制 Claude 的思考/推理行為。優先於已棄用的 `maxThinkingTokens`。

```typescript theme={null}
type ThinkingDisplay = "summarized" | "omitted";

type ThinkingConfig =
  | { type: "adaptive"; display?: ThinkingDisplay } // 模型決定何時以及思考多少（Opus 4.6+）
  | { type: "enabled"; budgetTokens?: number; display?: ThinkingDisplay } // 固定思考令牌預算
  | { type: "disabled" }; // 無擴展思考
```

可選的 `display` 欄位控制思考文字是否以 `"summarized"` 或 `"omitted"` 傳回。在 Claude Opus 4.7 及更新版本上，API 預設為 `"omitted"`，因此設定 `"summarized"` 以在 `thinking` 區塊中接收思考內容。Claude Code 不會將 `display` 傳送至 Amazon Bedrock 或 Google Cloud 的 Agent Platform，因此在這些提供者上，Opus 4.7 及更新版本即使在您將 `display` 設定為 `"summarized"` 時也會傳回空 `thinking` 區塊。

<h3 id="spawnedprocess">
  `SpawnedProcess`
</h3>

自訂程序生成的介面（與 `spawnClaudeCodeProcess` 選項搭配使用）。`ChildProcess` 已滿足此介面。

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

傳遞至自訂生成函式的選項。

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
  `signal` 欄位告訴您的生成函式何時拆除程序。將其作為 `signal` 選項傳遞至 Node 的 `spawn()`，或將其傳遞至您的 VM 或容器拆除處理程式。

  此信號不會在 [`Options.abortController`](#options) 中止時立即觸發。SDK 首先關閉程序的 stdin 並等待約兩秒，以便 CLI 可以乾淨地關閉，然後中止此信號。若要在呼叫者中止時立即做出反應，請在您自己的 `Options.abortController.signal` 上監聽，您的生成函式可以從其封閉範圍參考。
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

當您呼叫 `setMcpServers()` 時，Claude Code 應用這些規則：

* **呼叫未命名的伺服器**：Claude Code 保持 plugin 提供的伺服器執行。需要 Agent SDK v0.3.210 或更新版本。
* **呼叫命名的伺服器**：除了 CLI 在啟動時啟動的內建伺服器外，Claude Code 只在其設定與您傳遞的設定不同時才替換執行中的伺服器。
* **CLI 在啟動時啟動的內建伺服器**：如果呼叫命名一個，Claude Code 會捨棄該項目並在 `errors` 中報告它。

承諾在新增的 stdio、HTTP 和 SSE 伺服器連線或失敗後解決，因此來自已連線伺服器的工具在下一輪可用。

`added` 列出 Claude Code 新增或替換的伺服器，無論它們是否連線。未能連線的伺服器同時出現在 `added` 和 `errors` 中，失敗文字在 `errors` 下，`failed` 列在 [`mcpServerStatus()`](#methods) 中。在 Claude Code v2.1.257 之前，連線嘗試拋出的伺服器僅在 `errors` 下報告。

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
  skippedLinks?: number;
};
```

`skippedLinks` 計算倒帶拒絕恢復或刪除以確保連結安全的追蹤路徑：追蹤路徑上的符號連結、硬連結或其他非常規檔案，不再解析為檢查點建立時指向的位置的父目錄，或無法安全讀取的備份。該欄位需要 Claude Code v2.1.216 或更新版本。使用 `rewindFiles(userMessageId, { dryRun: true })` 的預覽呼叫永遠不會設定它。

<h3 id="sdkstatusmessage">
  `SDKStatusMessage`
</h3>

狀態更新訊息（例如壓縮）。

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

背景工作完成、失敗或停止時的通知。背景工作包括 `run_in_background` Bash 命令、[Monitor](#monitor) 監視和背景子代理。如需 `ambient` 欄位，請參閱 [`SDKTaskStartedMessage`](#sdktaskstartedmessage)，它定義它及其版本要求。

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

當 Claude Code [將長 MCP 工具呼叫移至背景](/docs/zh-TW/mcp#automatic-backgrounding-of-long-tool-calls) 時，該呼叫的 `tool_result` 區塊僅保留預留位置，呼叫的實際結果在此通知中到達。使用 `tool_use_id` 將通知與呼叫進行比對。在 `completed` 通知上，`resource_links` 列出工具作為 [`SDKMcpResourceLink`](#sdkmcpresourcelink) 項目透過參考傳回的檔案，具有與 [`tool_use_result.resourceLinks`](#sdkusermessage) 相同的 50 連結和 64 KiB 限制。Claude Code 在結果沒有連結時省略 `resource_links`，以及在不是 MCP 工具呼叫的工作通知上。`resource_links` 需要 Agent SDK v0.3.257 或更新版本。

Claude Code 在傳送給模型的每個工作通知前面加上通知，除了帶有 [`scheduled-trigger` 子類型](#task-notification-subkinds) 戳記的傳遞外，它們改為攜帶指派工作框架。通知指出沒有發生人類輸入，因此模型不會將通知視為使用者指令或批准。

若要偵測工作通知輪次，請在 [`SDKUserMessage`](#sdkusermessage) 或 [`SDKResultMessage`](#sdkresultmessage) 上檢查 `origin.kind === "task-notification"`，而不是在通知文字上進行比對。如果您需要知道引發它的內容，請從同一欄位讀取 `subkind`。在 v2.1.205 之前，Claude Code 在工作階段閒置時到達的通知上省略通知。

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

在 hook 開始執行時發出。

Claude Code 將此訊息、[`SDKHookProgressMessage`](#sdkhookprogressmessage) 和 [`SDKHookResponseMessage`](#sdkhookresponsemessage) 立即傳遞至訊息串流，包括在工作階段啟動期間 `SessionStart` 或 `Setup` hook 仍在執行時。Claude Code v2.1.169 至 v2.1.203 在 `SessionStart` 或 `Setup` hook 完成後以一個批次傳遞這些訊息；v2.1.204 恢復了即時傳遞。

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

在 hook 執行時發出，帶有 stdout/stderr 輸出。

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

在 hook 完成執行時發出。

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

在工具執行時定期發出，以指示進度。

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

當工具呼叫在主對話中執行時，Claude Code 每 30 秒發出一個 `tool_progress` 訊息，帶有 `heartbeat: true`。每個心跳攜帶工具名稱和經過的秒數，因此您可以區分長執行呼叫和停滯工作階段。Claude Code 不為子代理內的工具呼叫發出心跳。`heartbeat` 欄位需要 Agent SDK v0.3.214 或更新版本。在 v2.1.257 之前，Claude Code 也不為前景 Agent 工具呼叫發出心跳。

在除心跳外的 Agent 工具的 `tool_progress` 訊息上，`subagent_type` 命名執行中的子代理類型，例如 `general-purpose`。`subagent_retry` 在該子代理等待 API 錯誤退避（例如速率限制或過載）時存在，每次重試嘗試一個訊息。兩個欄位都需要 Agent SDK v0.3.214 或更新版本。

若要從 `subagent_retry` 呈現重試指示器：

* 按 `parent_tool_use_id` 追蹤指示器，它對每個子代理是唯一的。`tool_use_id` 由來自一個助手輪次的平行子代理共享，因此按它追蹤會讓一個子代理的更新清除另一個的指示器。
* 當同一 `parent_tool_use_id` 的稍後 `tool_progress` 到達時清除指示器，既不帶 `subagent_retry` 也不帶 `heartbeat: true`，或當工具的結果訊息到達時。帶 `heartbeat: true` 的框架僅報告活躍性，因此在一個到達時保持指示器。`attempt` 可能在持續重試下超過 `max_retries`，因此不要從計數器衍生清除。
* 將 `error_category` 視為選擇您自己訊息文字的令牌，而非顯示文字。值為 `rate_limit`、`overloaded`、`authentication_failed`、`server_error`、`cloud_credential_error` 和 `unknown`。處理您不認識的值的方式與處理 `unknown` 的方式相同，因為稍後的版本可以新增值。

<h3 id="sdkauthstatusmessage">
  `SDKAuthStatusMessage`
</h3>

在驗證流程期間發出。

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

在工作開始時發出。`task_type` 欄位對 Bash 命令和 [Monitor](#monitor) 監視為 `"local_bash"`，對子代理為 `"local_agent"`，或 `"remote_agent"`。

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

`ambient` 對不是工作階段工作一部分的工作為 `true`，例如 Claude Code 為其自身操作執行的工作。即時更新監視器也是環境的，包括使用者要求的監視器。從活動指示器中排除環境工作。該欄位需要 Agent SDK v0.3.247 或更新版本。

`ambient` 也出現在 [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) 和 [`SDKBackgroundTasksChangedMessage`](#sdkbackgroundtaskschangedmessage) 項目上。

`is_backgrounded` 和 `spawn_depth` 描述 Claude Code 如何啟動工作。兩個欄位都需要 Agent SDK v0.3.238 或更新版本。

* `is_backgrounded`：Claude Code 在 `"local_agent"` 和 `"local_bash"` 工作上設定它。`true` 表示工作在背景執行。`false` 表示工作在前景執行，啟動它的工具呼叫保持阻止，直到工作完成或移至背景。
* `spawn_depth`：Claude Code 僅在 `"local_agent"` 工作上設定它。主執行緒生成的子代理的深度為 `1`。深度 `1` 子代理生成的子代理的深度為 `2`，以此類推。

[已恢復的子代理](/docs/zh-TW/agent-sdk/subagents#resume-subagents) 始終報告 `is_backgrounded: true`，因為 Claude Code 在背景執行每個已恢復的子代理。當前景工作稍後移至背景時，Claude Code 在 [`task_updated`](#sdktaskupdatedmessage) 訊息中報告新的 `is_backgrounded` 值，而不是傳送第二個 `task_started`。

<h3 id="sdktaskprogressmessage">
  `SDKTaskProgressMessage`
</h3>

在子代理或背景工作執行時定期發出。`summary` 欄位僅在啟用 [`agentProgressSummaries`](#options) 時填入。

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

在背景工作的狀態變更時發出，例如當它從 `running` 轉換為 `completed` 時。將 `patch` 合併到按 `task_id` 鍵入的本機工作地圖中。`end_time` 欄位是 Unix 紀元時間戳記（以毫秒為單位），可與 `Date.now()` 比較。

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

每當即時背景工作集變更時發出：工作啟動、完成、被殺死、前景代理被背景化，或工作的 `description` 或 `ambient` 欄位變更。

`tasks` 陣列是完整的即時集。用每個承載替換任何快取集，而不是配對 `task_started` 和 `task_notification` 事件，因此下一個成員資格變更會更正您可能遺漏的任何事件。

相對於這些每個工作事件的順序未指定，因此不要關聯兩個串流。

啟動時不發出任何內容。每當工作階段的 CLI 程序啟動或重新啟動時重設為空集，並讓下一個成員資格變更重新填入它。

當您向執行中的工作階段傳送重複的 `initialize` 控制請求時，例如在傳輸間隙後使用 [`reinitialize()`](#query-object)，Claude Code 在回應後跟著目前即時集的快照，即使它是空的。因此重新連線的主機可以了解執行中的內容，而無需等待下一個成員資格變更。在 Agent SDK v0.3.239 之前，Claude Code 在重複 `initialize` 後沒有傳送快照。

需要 Claude Code v2.1.203 或更新版本。

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

在 Claude 產生思考區塊時發出，包括編輯過的區塊。`estimated_tokens` 是目前區塊中迄今為止產生的思考令牌的執行估計，`estimated_tokens_delta` 是此框架攜帶的增量。使用這些估計進行進度顯示。

當模型或提供者報告分解時，頂級代理迴圈的最終計數是結果訊息的 [`usage.output_tokens_details.thinking_tokens`](#usage)，[不包括子代理令牌](/docs/zh-TW/agent-sdk/cost-tracking#get-the-total-cost-of-a-query)。

需要 Claude Code v2.1.153 或更新版本。

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

在檔案檢查點持久化至磁碟時發出。

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

當工作階段遇到速率限制時發出。

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

當 `errorCode` 為 `"credits_required"` 時，拒絕來自 claude.ai 訂閱，其包含的使用已耗盡，工作階段在使用者購買使用額度之前無法繼續。`canUserPurchaseCredits` 指示已驗證的使用者是否可以為帳戶購買額度，`hasChargeableSavedPaymentMethod` 指示檔案上是否有可計費的儲存付款方式。所有三個欄位在不是額度必需拒絕的速率限制事件上不存在。需要 Claude Code v2.1.181 或更新版本。

<h3 id="sdklocalcommandoutputmessage">
  `SDKLocalCommandOutputMessage`
</h3>

Claude Code 不發出此訊息類型。當您傳送命令（例如 `/context` 或 `/usage`）作為提示時，其輸出作為 [`SDKAssistantMessage`](#sdkassistantmessage) 到達。

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

當可用命令集在工作階段中期變更時發出，例如當 Claude Code 在代理進入子目錄時發現技能時。`commands` 陣列是完整的更新清單，因此用此承載替換任何快取命令清單。在此訊息後呼叫 [`supportedCommands()`](#query-object) 會傳回相同的更新清單，因為該方法追蹤最新推送；這需要 Agent SDK v0.3.216 或更新版本。在較早的 SDK 版本中，`supportedCommands()` 傳回在初始化時擷取的快照，永遠不會反映工作階段中期的變更。

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

在啟用 [`promptSuggestions`](#options) 且 Claude Code 為該輪次產生建議時，輪次後發出。包含預測的下一個使用者提示。對於未取得任何建議的輪次，請參閱 [Claude Code 何時跳過建議](/docs/zh-TW/interactive-mode#when-claude-code-skips-suggestions)。

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

在工作階段的對話被替換而不結束工作階段時發出。在 `query()` 呼叫中，只有 `/clear` 及其別名產生此訊息。在 `new_conversation_id` 下掛載空文字記錄，並捨棄任何快取工作階段標題。

```typescript theme={null}
type SDKConversationResetMessage = {
  type: "conversation_reset";
  new_conversation_id: UUID;
  uuid: UUID;
  session_id: string;
};
```

SDK 的已發佈類型在 Claude Code v2.1.203 及更新版本中宣告 `SDKConversationResetMessage`。在 v2.1.203 之前，`SDKMessage` 參考該類型而不宣告它，因此當 `skipLibCheck` 被停用時，在 `type === "conversation_reset"` 上縮小範圍失敗類型檢查。

<h3 id="aborterror">
  `AbortError`
</h3>

中止操作的自訂錯誤類別。

```typescript theme={null}
class AbortError extends Error {}
```

`AbortError` 是 SDK 類型化 API 中唯一的錯誤類別。其他失敗，例如 Claude Code 程序退出或無法啟動，以沒有 SDK 類別可比對的錯誤拒絕訊息反覆運算。[疑難排解](/docs/zh-TW/agent-sdk/troubleshooting) 按訊息鍵入這些錯誤，每個都有原因和修正。

<h2 id="sandbox-configuration">
  Sandbox 設定
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

Sandbox 行為的設定。使用此設定以程式設計方式啟用命令 sandboxing 並設定網路限制。

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

| 屬性                          | 類型                                                    | 預設值         | 說明                                                                                                                                                            |
| :-------------------------- | :---------------------------------------------------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `enabled`                   | `boolean`                                             | `false`     | 為命令執行啟用 sandbox 模式                                                                                                                                            |
| `failIfUnavailable`         | `boolean`                                             | `true`      | 如果 `enabled` 為 `true` 但 sandbox 無法啟動，則在啟動時停止。設定為 `false` 以回退到未 sandboxed 的執行，並在 stderr 上顯示警告                                                                  |
| `autoAllowBashIfSandboxed`  | `boolean`                                             | `true`      | 當 sandbox 啟用時自動核准 Bash 命令                                                                                                                                     |
| `excludedCommands`          | `string[]`                                            | `[]`        | 繞過 sandbox 限制的命令，例如 `['docker *']`。這些命令會自動以未 sandboxed 的方式執行，無需模型參與；[`sandbox.excludedCommands`](/docs/zh-TW/settings-reference#sandbox-excludedcommands) 涵蓋何時適用項目 |
| `allowUnsandboxedCommands`  | `boolean`                                             | `true`      | 允許模型要求在 sandbox 外執行命令。當為 `true` 時，模型可以在工具輸入中設定 `dangerouslyDisableSandbox`，這會回退到[權限系統](#permissions-fallback-for-unsandboxed-commands)                        |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `undefined` | 網路特定的 sandbox 設定                                                                                                                                              |
| `filesystem`                | [`SandboxFilesystemConfig`](#sandboxfilesystemconfig) | `undefined` | 檔案系統特定的 sandbox 設定，用於讀取/寫入限制                                                                                                                                  |
| `ignoreViolations`          | `Record<string, string[]>`                            | `undefined` | 命令子字串或 `*`（適用於每個命令）對應到要忽略的違規文字子字串的對應，例如 `{ "*": ['/etc/hosts'] }`；請參閱 [`sandbox.ignoreViolations`](/docs/zh-TW/settings-reference#sandbox-ignoreviolations)        |
| `enableWeakerNestedSandbox` | `boolean`                                             | `false`     | 啟用較弱的巢狀 sandbox 以相容性                                                                                                                                          |
| `ripgrep`                   | `{ command: string; args?: string[] }`                | `undefined` | Sandbox 環境的自訂 ripgrep 二進位設定                                                                                                                                   |

<Note>
  Sandbox 取決於平台支援，在 Linux 上，需要 `bubblewrap` 和 `socat` 等工具。當 `enabled` 為 `true` 且 sandbox 無法啟動時，`query()` 會報告一個 `result` 訊息，其中 `subtype: "error_during_execution"` 且原因在 `errors` 中。對於單一訊息 `query()` 呼叫，SDK 會在產生該錯誤結果後拋出，因此請將迴圈包裝在 try 區塊中以繼續執行。請參閱[處理結果](/docs/zh-TW/agent-sdk/agent-loop#handle-the-result)以了解錯誤合約。

  若要改為以未 sandboxed 的方式執行，請設定 `failIfUnavailable: false`。
</Note>

<h4 id="example-usage">
  使用範例
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
  **Unix socket 安全性：** `allowUnixSockets` 選項可以授予存取權限給可以到達 sandbox 外的系統服務。例如，允許 `/var/run/docker.sock` 實際上會透過 Docker API 授予完整的主機系統存取權限，繞過 sandbox 隔離。只允許嚴格必要的 Unix socket，並了解每個 socket 的安全含義。
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Sandbox 模式的網路特定設定。當父 [`SandboxSettings`](#sandboxsettings) 中的 `enabled` 為 `true` 時，這些設定適用於 sandboxed Bash 命令。它們不會限制 WebFetch 工具，該工具改為使用[權限規則](/docs/zh-TW/permissions#webfetch)。

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

| 屬性                        | 類型         | 預設值         | 說明                                                                                                                                                                                         |
| :------------------------ | :--------- | :---------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `string[]` | `[]`        | Sandboxed 程序可以存取的網域名稱                                                                                                                                                                      |
| `deniedDomains`           | `string[]` | `[]`        | Sandboxed 程序無法存取的網域名稱。優先於 `allowedDomains`                                                                                                                                                 |
| `strictAllowlist`         | `boolean`  | `false`     | 拒絕 sandboxed 命令存取[網路允許清單](/docs/zh-TW/sandboxing#network-isolation)外的主機，而不是提示。僅對 sandboxed 命令強制執行；WebFetch 等程序內工具不受其限制。僅從使用者、受管理或 CLI `--settings` 設定中接受；專案設定會被忽略。需要 Claude Code v2.1.219 或更新版本 |
| `allowManagedDomainsOnly` | `boolean`  | `false`     | 僅受管理設定。在[受管理設定](/docs/zh-TW/managed-settings)中設定時，只有 `allowedDomains` 項目和來自受管理設定的 `WebFetch(domain:...)` 允許規則會被接受，來自使用者、專案或本機設定的允許項目會被忽略。透過 SDK 選項設定時無效                                         |
| `allowLocalBinding`       | `boolean`  | `false`     | 允許程序繫結到本機連接埠（例如用於開發伺服器）                                                                                                                                                                    |
| `allowUnixSockets`        | `string[]` | `[]`        | 程序可以存取的 Unix socket 路徑（例如 Docker socket）                                                                                                                                                   |
| `allowAllUnixSockets`     | `boolean`  | `false`     | 允許存取所有 Unix socket                                                                                                                                                                         |
| `httpProxyPort`           | `number`   | `undefined` | 用於網路請求的 HTTP proxy 連接埠                                                                                                                                                                     |
| `socksProxyPort`          | `number`   | `undefined` | 用於網路請求的 SOCKS proxy 連接埠                                                                                                                                                                    |

<Note>
  內建的 sandbox proxy 根據請求的主機名稱強制執行 `allowedDomains`，不會終止或檢查 TLS 流量，因此[網域前置](https://en.wikipedia.org/wiki/Domain_fronting)等技術可能會繞過它。請參閱 [Sandbox 安全限制](/docs/zh-TW/sandboxing#security-limitations)以了解詳細資訊，以及[安全部署](/docs/zh-TW/agent-sdk/secure-deployment#traffic-forwarding)以設定 TLS 終止 proxy。
</Note>

<h3 id="sandboxfilesystemconfig">
  `SandboxFilesystemConfig`
</h3>

Sandbox 模式的檔案系統特定設定。

```typescript theme={null}
type SandboxFilesystemConfig = {
  allowWrite?: string[];
  denyWrite?: string[];
  denyRead?: string[];
};
```

| 屬性           | 類型         | 預設值  | 說明            |
| :----------- | :--------- | :--- | :------------ |
| `allowWrite` | `string[]` | `[]` | 允許寫入存取的檔案路徑模式 |
| `denyWrite`  | `string[]` | `[]` | 拒絕寫入存取的檔案路徑模式 |
| `denyRead`   | `string[]` | `[]` | 拒絕讀取存取的檔案路徑模式 |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  未 Sandboxed 命令的權限回退
</h3>

當 `allowUnsandboxedCommands` 啟用時，模型可以透過在工具輸入中設定 `dangerouslyDisableSandbox: true` 來要求在 sandbox 外執行命令。這些請求會回退到現有的權限系統，這表示您的 `canUseTool` 處理程式會被呼叫，允許您實現自訂授權邏輯。

您的 `excludedCommands` 項目改為自動繞過 sandbox，無需模型參與；[`sandbox.excludedCommands`](/docs/zh-TW/settings-reference#sandbox-excludedcommands) 涵蓋何時適用項目。

在下面的範例中，`isCommandAuthorized` 代表您定義的授權檢查。

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Deploy my application",
  options: {
    sandbox: {
      enabled: true,
      allowUnsandboxedCommands: true // Model can request unsandboxed execution
    },
    permissionMode: "default",
    canUseTool: async (tool, input) => {
      // Check if the model is requesting to bypass the sandbox
      if (tool === "Bash" && input.dangerouslyDisableSandbox) {
        // The model is requesting to run this command outside the sandbox
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
  使用 `dangerouslyDisableSandbox: true` 執行的命令具有完整的系統存取權限。確保您的 `canUseTool` 處理程式仔細驗證這些請求。

  如果 `permissionMode` 設定為 `bypassPermissions` 且 `allowUnsandboxedCommands` 啟用，模型可以自主執行 sandbox 外的命令，無需核准提示，除了[動作無模式自動核准的動作](/docs/zh-TW/permission-modes#actions-no-mode-auto-approves)。此組合實際上允許模型以無聲方式逃脫 sandbox 隔離。
</Warning>

<h2 id="see-also">
  另請參閱
</h2>

* [SDK 概觀](/docs/zh-TW/agent-sdk/overview) - 一般 SDK 概念
* [Python SDK 參考](/docs/zh-TW/agent-sdk/python) - Python SDK 文件
* [CLI 參考](/docs/zh-TW/cli-reference) - 命令列介面
* [常見工作流程](/docs/zh-TW/common-workflows) - 逐步指南
