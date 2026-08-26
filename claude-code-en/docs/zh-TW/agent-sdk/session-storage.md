> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 將會話持久化到外部存儲

> 將會話記錄鏡像到 S3、Redis 或您自己的後端，以便任何主機都可以恢復它們。

默認情況下，SDK 將會話記錄寫入本地文件系統上 `~/.claude/projects/` 下的 JSONL 文件。`SessionStore` 適配器允許您將這些記錄鏡像到您自己的後端，例如 S3、Redis 或數據庫，以便在一個主機上創建的會話可以在另一個主機上恢復。

使用會話存儲的常見原因：

* **多主機部署。** 無服務器函數、自動擴展的工作進程和 CI 運行器不共享文件系統。共享存儲允許任何副本恢復任何會話。
* **耐久性。** 本地容器是短暫的。由 S3 或數據庫支持的存儲可以在重新啟動和重新部署後存活。
* **合規性和審計。** 將記錄保存在您已經管理的存儲中，使用您自己的保留規則、加密和訪問控制。

<h2 id="the-sessionstore-interface">
  `SessionStore` 介面
</h2>

`SessionStore` 是一個物件，具有兩個必需的方法 `append` 和 `load`，以及三個可選方法。SDK 呼叫 `append` 在查詢期間寫入記錄項目，呼叫 `load` 讀取它們以進行恢復。

<CodeGroup>
  ```typescript TypeScript theme={null}
  // Exported from @anthropic-ai/claude-agent-sdk as
  // SessionStore, SessionKey, SessionStoreEntry.

  type SessionKey = {
    projectKey: string;
    sessionId: string;
    subpath?: string;
  };

  type SessionStore = {
    // Required
    append(key: SessionKey, entries: SessionStoreEntry[]): Promise<void>;
    load(key: SessionKey): Promise<SessionStoreEntry[] | null>;

    // Optional
    listSessions?(
      projectKey: string,
    ): Promise<Array<{ sessionId: string; mtime: number }>>;
    delete?(key: SessionKey): Promise<void>;
    listSubkeys?(key: {
      projectKey: string;
      sessionId: string;
    }): Promise<string[]>;
  };
  ```

  ```python Python theme={null}
  # Exported from claude_agent_sdk as
  # SessionStore, SessionKey, SessionStoreEntry.

  class SessionKey(TypedDict):
      project_key: str
      session_id: str
      subpath: NotRequired[str]

  class SessionStore(Protocol):
      # Required
      async def append(
          self, key: SessionKey, entries: list[SessionStoreEntry]
      ) -> None: ...
      async def load(self, key: SessionKey) -> list[SessionStoreEntry] | None: ...

      # Optional — omit or raise NotImplementedError
      async def list_sessions(
          self, project_key: str
      ) -> list[SessionStoreListEntry]: ...
      async def delete(self, key: SessionKey) -> None: ...
      async def list_subkeys(self, key: SessionListSubkeysKey) -> list[str]: ...
  ```
</CodeGroup>

`SessionKey` 指向一個記錄。`projectKey` 是工作目錄的穩定、檔案系統安全的編碼，`sessionId` 是工作階段 UUID，`subpath` 在項目屬於子代理記錄或邊車檔案而不是主對話時設定。將 `subpath` 視為不透明的鍵後綴；它遵循磁碟上的配置，例如 `subagents/agent-<id>`。當 `subpath` 未定義時，鍵指向主記錄。

| 方法             | 必需 | 呼叫時機                                                                                              |
| :------------- | :- | :------------------------------------------------------------------------------------------------ |
| `append`       | 是  | 在本機寫入每批記錄項目後。項目是 JSON 安全的物件，本機 JSONL 中每行一個。                                                       |
| `load`         | 是  | 在子程序產生之前一次，當設定 `resume` 時。如果工作階段未知，傳回 `null`。                                                     |
| `listSessions` | 否  | 由 `listSessions({ sessionStore })` 和 `query()`/`startup()` 與 `continue: true` 呼叫。如果未定義，這些呼叫會擲出例外。 |
| `delete`       | 否  | 由 `deleteSession({ sessionStore })` 呼叫。刪除主鍵（無 `subpath`）必須級聯到該工作階段的所有子鍵。如果未定義，刪除是無操作的，適合僅追加後端。    |
| `listSubkeys`  | 否  | 在恢復期間，探索子代理記錄。如果未定義，只有主記錄被恢復。                                                                     |

<h2 id="quick-start">
  快速開始
</h2>

SDK 附帶一個 `InMemorySessionStore` 用於開發和測試。下面的示例使用附加的存儲運行查詢，從結果消息中捕獲會話 ID，然後在第二個 `query()` 呼叫中從存儲恢復。第二個呼叫傳遞相同的存儲實例加上 `resume`，因此 SDK 從存儲而不是本地檔案系統載入記錄：

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, InMemorySessionStore } from "@anthropic-ai/claude-agent-sdk";

  const store = new InMemorySessionStore();

  let sessionId: string | undefined;
  for await (const message of query({
    prompt: "List the TypeScript files under src/",
    options: { sessionStore: store },
  })) {
    if (message.type === "result") {
      sessionId = message.session_id;
    }
  }

  // Resume from the store. The agent has full context from the first call.
  for await (const message of query({
    prompt: "Summarize what those files do",
    options: { sessionStore: store, resume: sessionId },
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      ClaudeAgentOptions,
      InMemorySessionStore,
      ResultMessage,
      query,
  )

  store = InMemorySessionStore()


  async def main():
      session_id = None
      async for message in query(
          prompt="List the Python files under src/",
          options=ClaudeAgentOptions(session_store=store),
      ):
          if isinstance(message, ResultMessage):
              session_id = message.session_id

      # Resume from the store. The agent has full context from the first call.
      async for message in query(
          prompt="Summarize what those files do",
          options=ClaudeAgentOptions(session_store=store, resume=session_id),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

第二個查詢會列印來自第一個查詢的檔案摘要，這表明代理已從存儲恢復並具有完整的上下文。

<h2 id="write-your-own-adapter">
  編寫您自己的適配器
</h2>

針對您的後端實現 `append` 和 `load`。如果您希望 `listSessions()`、`deleteSession()` 和子代理恢復針對存儲工作，請添加 `listSessions`、`delete` 和 `listSubkeys`。

傳遞給 `append` 的條目類型為 `SessionStoreEntry`（一個 `{ type: string; ... }` 對象）。將它們視為不透明的 JSON 安全值：按順序持久化它們，並從 `load` 以相同順序返回它們。`load` 必須返回與追加的條目深度相等的條目；不需要字節相等的序列化，因此像 Postgres `jsonb` 這樣重新排序對象鍵的後端是可以的。

<h2 id="reference-implementations">
  參考實現
</h2>

TypeScript SDK 存儲庫在 [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores) 下包含 S3、Redis 和 Postgres 的可運行參考適配器。它們未發佈到 npm；將您需要的 `src/` 文件複製到您的項目中並安裝相應的後端客戶端。

| 適配器                                                                                                                            | 後端客戶端                | 存儲模型                                           |
| :----------------------------------------------------------------------------------------------------------------------------- | :------------------- | :--------------------------------------------- |
| [`S3SessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/s3)             | `@aws-sdk/client-s3` | 每個 `append()` 一個 JSONL 部分文件；`load()` 列出、排序和連接。 |
| [`RedisSessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/redis)       | `ioredis`            | 每個記錄的 `RPUSH`/`LRANGE` 列表，加上排序集會話索引。           |
| [`PostgresSessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/postgres) | `pg`                 | `jsonb` 表中每個條目一行，按 `BIGSERIAL` 排序。             |

每個適配器都採用預配置的客戶端實例，因此您可以控制憑證、TLS、區域和池。例如，使用 S3：

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";
import { S3Client } from "@aws-sdk/client-s3";
import { S3SessionStore } from "./S3SessionStore"; // copied from examples/session-stores/s3

const store = new S3SessionStore({
  bucket: "my-claude-sessions",
  prefix: "transcripts",
  client: new S3Client({ region: "us-east-1" }),
});

for await (const message of query({
  prompt: "Hello!",
  options: { sessionStore: store },
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}

// Later, possibly on a different host:
for await (const message of query({
  prompt: "Continue where we left off",
  options: { sessionStore: store, resume: "previous-session-id" },
})) {
  // ...
}
```

<h3 id="validate-your-adapter">
  驗證您的適配器
</h3>

兩個 SDK 都附帶一個符合性套件，該套件斷言 `append`、`load` 和可選方法必須滿足的行為契約。當未實現這些方法時，可選方法的測試會自動跳過。

在 TypeScript 中，從示例目錄將 [`shared/conformance.ts`](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/examples/session-stores/shared/conformance.ts) 複製到您的測試套件中。在 Python 中，該套件在包中提供：

```python Python theme={null}
import pytest
from claude_agent_sdk.testing import run_session_store_conformance


@pytest.mark.asyncio
async def test_my_store_conformance():
    await run_session_store_conformance(MyRedisStore)
```

<h2 id="behavior-notes">
  行為說明
</h2>

<h3 id="dual-write-architecture">
  雙寫架構
</h3>

存儲是鏡像，不是替代品。Claude Code 子進程始終首先寫入本地磁盤；SDK 然後將每批轉發到 `append()`。如果您希望本地副本是短暫的，請在 `options.env` 中將 `CLAUDE_CONFIG_DIR` 指向臨時目錄。因為鏡像依賴於本地寫入，`sessionStore` 不能與 `persistSession: false` 結合；如果您同時設置兩者，SDK 會拋出異常。如果與 `enableFileCheckpointing` 結合，它也會拋出異常，因為文件歷史備份 blob 直接寫入本地磁盤，不會鏡像到存儲。

<h3 id="mirror-writes-are-best-effort">
  鏡像寫入是盡力而為
</h3>

如果 `append()` 拒絕，SDK 會以短暫的退避重試該批次最多兩次，總共最多三次嘗試。超時的呼叫不會重試，因為原始呼叫可能仍然會到達。如果批次仍然失敗，錯誤被記錄，一個 `{ type: "system", subtype: "mirror_error" }` 消息被發出到迭代器中，批次被丟棄，查詢繼續。本地記錄已經在磁盤上持久化，因此存儲中斷不會中斷代理或在本地丟失數據。監視 `mirror_error` 如果您需要檢測存儲數據丟失。因為重試的批次可以重新傳遞已經到達的條目，請在您的 `append()` 實現中按 `entry.uuid` 進行去重。

<h3 id="getsessionmessages-returns-the-post-compaction-chain">
  `getSessionMessages` 返回後壓縮鏈
</h3>

`getSessionMessages({ sessionStore })` 返回代理在恢復時會看到的鏈接消息鏈。自動壓縮後，早期的轉向被摘要替換，因此存儲持有 503 個原始條目的會話可能從 `getSessionMessages` 返回 18 條消息。對於完整的原始歷史記錄，包括壓縮前的轉向和元數據條目，直接調用 `store.load(key)`。

<h3 id="forksession-is-not-a-byte-copy">
  `forkSession` 不是字節副本
</h3>

`forkSession({ sessionStore })` 讀取源條目，重寫每個 `sessionId` 字段並重新映射消息 UUID，然後在新鍵下追加轉換後的條目。適配器級別的副本或 `CopyObject` 快捷方式會產生仍然引用舊會話 ID 的記錄，因此 SDK 不使用它。

<h3 id="subagent-transcripts">
  子代理記錄
</h3>

子代理記錄在 `subpath: "subagents/agent-<id>"` 下鏡像。`listSubagents({ sessionStore })` 要求適配器實現 `listSubkeys`；`getSubagentMessages({ sessionStore })` 在可用時使用它，但在未定義時回退到直接子路徑。恢復也調用 `listSubkeys` 來恢復子代理文件；沒有它，只有主記錄被物化。

<h3 id="retention">
  保留
</h3>

SDK 永遠不會自行從您的存儲中刪除。保留是適配器的責任：根據您的合規要求實現 TTL、S3 生命週期策略或計劃清理。`CLAUDE_CONFIG_DIR` 下的本地記錄由 `cleanupPeriodDays` 設置獨立清掃。

<h2 id="supported-on">
  支持於
</h2>

以下 SDK 函數接受 `sessionStore` 選項，當提供時針對存儲而不是本地文件系統運行：

* [`query()`](/docs/zh-TW/agent-sdk/typescript#query)
* [`startup()`](/docs/zh-TW/agent-sdk/typescript#startup)
* [`listSessions()`](/docs/zh-TW/agent-sdk/typescript#listsessions)
* [`getSessionInfo()`](/docs/zh-TW/agent-sdk/typescript#getsessioninfo)
* [`getSessionMessages()`](/docs/zh-TW/agent-sdk/typescript#getsessionmessages)
* [`renameSession()`](/docs/zh-TW/agent-sdk/typescript#renamesession)
* [`tagSession()`](/docs/zh-TW/agent-sdk/typescript#tagsession)
* [`deleteSession()`](/docs/zh-TW/agent-sdk/typescript)
* [`forkSession()`](/docs/zh-TW/agent-sdk/typescript)
* [`listSubagents()`](/docs/zh-TW/agent-sdk/typescript)
* [`getSubagentMessages()`](/docs/zh-TW/agent-sdk/typescript)

<h2 id="related-resources">
  相關資源
</h2>

* [使用會話](/docs/zh-TW/agent-sdk/sessions)：在沒有自定義存儲的情況下繼續、恢復和分叉
* [託管 SDK](/docs/zh-TW/agent-sdk/hosting)：多主機環境的部署模式
* [TypeScript `Options`](/docs/zh-TW/agent-sdk/typescript#options)：完整選項參考
* [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores)：可運行的 S3、Redis 和 Postgres 參考適配器
