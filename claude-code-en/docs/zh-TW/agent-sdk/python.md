> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK 參考 - Python

> Python Agent SDK 的完整 API 參考，包括所有函數、類型和類別。

<h2 id="installation">
  安裝
</h2>

在虛擬環境中安裝套件。在最近的 Debian、Ubuntu 和 Homebrew Python 安裝上，針對系統 Python 執行 `pip install` 會失敗，並出現 `error: externally-managed-environment` 錯誤。

```bash theme={null}
python3 -m venv .venv
source .venv/bin/activate
pip install claude-agent-sdk
```

如需 uv、Windows PowerShell 和 API 金鑰設定，請參閱 [Agent SDK 概觀中的開始使用](/docs/zh-TW/agent-sdk/overview#get-started)。

<h2 id="choosing-between-query-and-claudesdkclient">
  在 `query()` 和 `ClaudeSDKClient` 之間選擇
</h2>

Python SDK 提供了兩種與 Claude Code 互動的方式：

<h3 id="quick-comparison">
  快速比較
</h3>

| 功能                  | `query()`                                  | `ClaudeSDKClient` |
| :------------------ | :----------------------------------------- | :---------------- |
| **Session**         | 預設情況下建立新 session                           | 重複使用相同 session    |
| **Conversation**    | 單一交換                                       | 同一上下文中的多個交換       |
| **Connection**      | 自動管理                                       | 手動控制              |
| **Streaming Input** | ✅ 支援                                       | ✅ 支援              |
| **Interrupts**      | ❌ 不支援                                      | ✅ 支援              |
| **Hooks**           | ✅ 支援                                       | ✅ 支援              |
| **Custom Tools**    | ✅ 支援                                       | ✅ 支援              |
| **Continue Chat**   | 透過 `continue_conversation` 或 `resume` 手動進行 | ✅ 自動進行            |
| **Use Case**        | 一次性任務                                      | 持續對話              |

<h3 id="when-to-use-query-one-off-tasks">
  何時使用 `query()`（一次性任務）
</h3>

**最適合：**

* 一次性問題，不需要對話歷史
* 不需要先前交換上下文的獨立任務
* 簡單的自動化腳本
* 當您想每次都重新開始時

<h3 id="when-to-use-claudesdkclient-continuous-conversation">
  何時使用 `ClaudeSDKClient`（持續對話）
</h3>

**最適合：**

* **繼續對話** - 當您需要 Claude 記住上下文時
* **後續問題** - 基於先前回應進行構建
* **互動式應用程式** - 聊天介面、REPL
* **回應驅動邏輯** - 當下一個動作取決於 Claude 的回應時
* **Session 控制** - 明確管理對話生命週期

<h2 id="functions">
  函數
</h2>

<h3 id="query">
  `query()`
</h3>

為每次與 Claude Code 的互動建立新 session。返回一個非同步迭代器，在消息到達時產生消息。每次呼叫 `query()` 都會重新開始，不記得先前的互動，除非您傳遞 `continue_conversation=True` 或在 [`ClaudeAgentOptions`](#claudeagentoptions) 中傳遞 `resume`。請參閱 [Sessions](/docs/zh-TW/agent-sdk/sessions)。

```python theme={null}
async def query(
    *,
    prompt: str | AsyncIterable[dict[str, Any]],
    options: ClaudeAgentOptions | None = None,
    transport: Transport | None = None
) -> AsyncIterator[Message]
```

<h4 id="parameters">
  參數
</h4>

| 參數          | 類型                           | 描述                                          |
| :---------- | :--------------------------- | :------------------------------------------ |
| `prompt`    | `str \| AsyncIterable[dict]` | 輸入提示，可以是字串或非同步可迭代物件（用於串流模式）                 |
| `options`   | `ClaudeAgentOptions \| None` | 可選配置物件（如果為 None，預設為 `ClaudeAgentOptions()`） |
| `transport` | `Transport \| None`          | 用於與 CLI 程序通訊的可選自訂傳輸                         |

<h4 id="returns">
  返回
</h4>

返回 `AsyncIterator[Message]`，從對話中產生消息。

<h4 id="example-with-options">
  範例 - 使用選項
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions


async def main():
    options = ClaudeAgentOptions(
        system_prompt="You are an expert Python developer",
        permission_mode="acceptEdits",
        cwd="/home/user/project",
    )

    async for message in query(prompt="Create a Python web server", options=options):
        print(message)


asyncio.run(main())
```

<h3 id="tool">
  `tool()`
</h3>

用於定義具有類型安全的 MCP tools 的裝飾器。

```python theme={null}
def tool(
    name: str,
    description: str,
    input_schema: type | dict[str, Any],
    annotations: ToolAnnotations | None = None
) -> Callable[[Callable[[Any], Awaitable[dict[str, Any]]]], SdkMcpTool[Any]]
```

<h4 id="parameters-2">
  參數
</h4>

| 參數             | 類型                                              | 描述                         |
| :------------- | :---------------------------------------------- | :------------------------- |
| `name`         | `str`                                           | tool 的唯一識別碼                |
| `description`  | `str`                                           | tool 功能的人類可讀描述             |
| `input_schema` | `type \| dict[str, Any]`                        | 定義 tool 輸入參數的架構（見下文）       |
| `annotations`  | [`ToolAnnotations`](#toolannotations)` \| None` | 可選的 MCP tool 註解，為客戶端提供行為提示 |

<h4 id="input-schema-options">
  輸入架構選項
</h4>

1. **簡單類型對應**（推薦）：

   ```python theme={null}
   {"text": str, "count": int, "enabled": bool}
   ```

2. **JSON Schema 格式**（用於複雜驗證）：
   ```python theme={null}
   {
       "type": "object",
       "properties": {
           "text": {"type": "string"},
           "count": {"type": "integer", "minimum": 0},
       },
       "required": ["text"],
   }
   ```

<h4 id="returns-2">
  返回
</h4>

一個裝飾器函數，包裝 tool 實現並返回 `SdkMcpTool` 實例。

<h4 id="example">
  範例
</h4>

```python theme={null}
from claude_agent_sdk import tool
from typing import Any


@tool("greet", "Greet a user", {"name": str})
async def greet(args: dict[str, Any]) -> dict[str, Any]:
    return {"content": [{"type": "text", "text": f"Hello, {args['name']}!"}]}
```

<h4 id="toolannotations">
  `ToolAnnotations`
</h4>

從 `mcp.types` 重新匯出（也可以從 `claude_agent_sdk` 匯入）。所有欄位都是可選提示；客戶端不應依賴它們進行安全決策。

| 欄位                | 類型             | 預設      | 描述                                                                  |
| :---------------- | :------------- | :------ | :------------------------------------------------------------------ |
| `title`           | `str \| None`  | `None`  | tool 的人類可讀標題                                                        |
| `readOnlyHint`    | `bool \| None` | `False` | 如果為 `True`，tool 不會修改其環境                                             |
| `destructiveHint` | `bool \| None` | `True`  | 如果為 `True`，tool 可能執行破壞性更新（僅在 `readOnlyHint` 為 `False` 時有意義）         |
| `idempotentHint`  | `bool \| None` | `False` | 如果為 `True`，使用相同參數的重複呼叫沒有額外效果（僅在 `readOnlyHint` 為 `False` 時有意義）      |
| `openWorldHint`   | `bool \| None` | `True`  | 如果為 `True`，tool 與外部實體互動（例如網路搜尋）。如果為 `False`，tool 的域是封閉的（例如記憶體 tool） |

```python theme={null}
from claude_agent_sdk import tool, ToolAnnotations
from typing import Any


@tool(
    "search",
    "Search the web",
    {"query": str},
    annotations=ToolAnnotations(readOnlyHint=True, openWorldHint=True),
)
async def search(args: dict[str, Any]) -> dict[str, Any]:
    return {"content": [{"type": "text", "text": f"Results for: {args['query']}"}]}
```

<h3 id="create_sdk_mcp_server">
  `create_sdk_mcp_server()`
</h3>

建立在 Python 應用程式內執行的進程內 MCP 伺服器。

```python theme={null}
def create_sdk_mcp_server(
    name: str,
    version: str = "1.0.0",
    tools: list[SdkMcpTool[Any]] | None = None
) -> McpSdkServerConfig
```

<h4 id="parameters-3">
  參數
</h4>

| 參數        | 類型                              | 預設        | 描述                          |
| :-------- | :------------------------------ | :-------- | :-------------------------- |
| `name`    | `str`                           | -         | 伺服器的唯一識別碼                   |
| `version` | `str`                           | `"1.0.0"` | 伺服器版本字串                     |
| `tools`   | `list[SdkMcpTool[Any]] \| None` | `None`    | 使用 `@tool` 裝飾器建立的 tool 函數清單 |

<h4 id="returns-3">
  返回
</h4>

返回 `McpSdkServerConfig` 物件，可以傳遞給 `ClaudeAgentOptions.mcp_servers`。

<h4 id="example-2">
  範例
</h4>

```python theme={null}
from claude_agent_sdk import tool, create_sdk_mcp_server


@tool("add", "Add two numbers", {"a": float, "b": float})
async def add(args):
    return {"content": [{"type": "text", "text": f"Sum: {args['a'] + args['b']}"}]}


@tool("multiply", "Multiply two numbers", {"a": float, "b": float})
async def multiply(args):
    return {"content": [{"type": "text", "text": f"Product: {args['a'] * args['b']}"}]}


calculator = create_sdk_mcp_server(
    name="calculator",
    version="2.0.0",
    tools=[add, multiply],  # Pass decorated functions
)

# Use with Claude
options = ClaudeAgentOptions(
    mcp_servers={"calc": calculator},
    allowed_tools=["mcp__calc__add", "mcp__calc__multiply"],
)
```

<h3 id="list_sessions">
  `list_sessions()`
</h3>

列出過去的 sessions 及其中繼資料。按專案目錄篩選或列出所有專案中的 sessions。同步；立即返回。

```python theme={null}
def list_sessions(
    directory: str | None = None,
    limit: int | None = None,
    offset: int = 0,
    include_worktrees: bool = True
) -> list[SDKSessionInfo]
```

<h4 id="parameters-4">
  參數
</h4>

| 參數                  | 類型            | 預設     | 描述                                                     |
| :------------------ | :------------ | :----- | :----------------------------------------------------- |
| `directory`         | `str \| None` | `None` | 列出 sessions 的目錄。省略時，返回所有專案中的 sessions                  |
| `limit`             | `int \| None` | `None` | 返回的最大 sessions 數                                       |
| `offset`            | `int`         | `0`    | 從排序結果開始跳過的 sessions 數。與 `limit` 搭配使用以進行分頁              |
| `include_worktrees` | `bool`        | `True` | 當 `directory` 在 git 儲存庫內時，包括所有 worktrees 路徑中的 sessions |

<h4 id="return-type-sdksessioninfo">
  返回類型：`SDKSessionInfo`
</h4>

| 屬性              | 類型            | 描述                                                   |
| :-------------- | :------------ | :--------------------------------------------------- |
| `session_id`    | `str`         | 唯一 session 識別碼                                       |
| `summary`       | `str`         | 顯示標題：自訂標題、自動生成的摘要或第一個提示                              |
| `last_modified` | `int`         | 上次修改時間（自紀元以來的毫秒數）                                    |
| `file_size`     | `int \| None` | Session 檔案大小（以位元組為單位）（遠端儲存後端為 `None`）                |
| `custom_title`  | `str \| None` | 使用者設定的 session 標題                                    |
| `first_prompt`  | `str \| None` | session 中的第一個有意義的使用者提示                               |
| `git_branch`    | `str \| None` | session 結束時的 Git 分支                                  |
| `cwd`           | `str \| None` | session 的工作目錄                                        |
| `tag`           | `str \| None` | 使用者設定的 session 標籤（見 [`tag_session()`](#tag_session)） |
| `created_at`    | `int \| None` | session 建立時間（自紀元以來的毫秒數）                              |

<h4 id="example-3">
  範例
</h4>

列印專案的 10 個最近 sessions。結果按 `last_modified` 降序排序，因此第一項是最新的。省略 `directory` 以搜尋所有專案。

```python theme={null}
from claude_agent_sdk import list_sessions

for session in list_sessions(directory="/path/to/project", limit=10):
    print(f"{session.summary} ({session.session_id})")
```

<h3 id="get_session_messages">
  `get_session_messages()`
</h3>

從過去的 session 中檢索消息。同步；立即返回。

```python theme={null}
def get_session_messages(
    session_id: str,
    directory: str | None = None,
    limit: int | None = None,
    offset: int = 0
) -> list[SessionMessage]
```

<h4 id="parameters-5">
  參數
</h4>

| 參數           | 類型            | 預設     | 描述                  |
| :----------- | :------------ | :----- | :------------------ |
| `session_id` | `str`         | 必需     | 要檢索消息的 session ID   |
| `directory`  | `str \| None` | `None` | 要查看的專案目錄。省略時，搜尋所有專案 |
| `limit`      | `int \| None` | `None` | 返回的最大消息數            |
| `offset`     | `int`         | `0`    | 從開始跳過的消息數           |

<h4 id="return-type-sessionmessage">
  返回類型：`SessionMessage`
</h4>

| 屬性                   | 類型                             | 描述          |
| :------------------- | :----------------------------- | :---------- |
| `type`               | `Literal["user", "assistant"]` | 消息角色        |
| `uuid`               | `str`                          | 唯一消息識別碼     |
| `session_id`         | `str`                          | session 識別碼 |
| `message`            | `Any`                          | 原始消息內容      |
| `parent_tool_use_id` | `None`                         | 保留供未來使用     |

<h4 id="example-4">
  範例
</h4>

```python theme={null}
from claude_agent_sdk import list_sessions, get_session_messages

sessions = list_sessions(limit=1)
if sessions:
    messages = get_session_messages(sessions[0].session_id)
    for msg in messages:
        print(f"[{msg.type}] {msg.uuid}")
```

<h3 id="get_session_info">
  `get_session_info()`
</h3>

按 ID 讀取單個 session 的中繼資料，無需掃描完整專案目錄。同步；立即返回。

```python theme={null}
def get_session_info(
    session_id: str,
    directory: str | None = None,
) -> SDKSessionInfo | None
```

<h4 id="parameters-6">
  參數
</h4>

| 參數           | 類型            | 預設     | 描述                  |
| :----------- | :------------ | :----- | :------------------ |
| `session_id` | `str`         | 必需     | 要查詢的 session 的 UUID |
| `directory`  | `str \| None` | `None` | 專案目錄路徑。省略時，搜尋所有專案目錄 |

返回 [`SDKSessionInfo`](#return-type-sdksessioninfo)，如果找不到 session，則返回 `None`。

<h4 id="example-5">
  範例
</h4>

查詢單個 session 的中繼資料，無需掃描專案目錄。當您已經從先前的執行中獲得 session ID 時很有用。

```python theme={null}
from claude_agent_sdk import get_session_info

info = get_session_info("550e8400-e29b-41d4-a716-446655440000")
if info:
    print(f"{info.summary} (branch: {info.git_branch}, tag: {info.tag})")
```

<h3 id="rename_session">
  `rename_session()`
</h3>

通過附加自訂標題項來重新命名 session。重複呼叫是安全的；最新的標題獲勝。同步。

```python theme={null}
def rename_session(
    session_id: str,
    title: str,
    directory: str | None = None,
) -> None
```

<h4 id="parameters-7">
  參數
</h4>

| 參數           | 類型            | 預設     | 描述                    |
| :----------- | :------------ | :----- | :-------------------- |
| `session_id` | `str`         | 必需     | 要重新命名的 session 的 UUID |
| `title`      | `str`         | 必需     | 新標題。去除空格後必須非空         |
| `directory`  | `str \| None` | `None` | 專案目錄路徑。省略時，搜尋所有專案目錄   |

如果 `session_id` 不是有效的 UUID 或 `title` 為空，則引發 `ValueError`；如果找不到 session，則引發 `FileNotFoundError`。

<h4 id="example-6">
  範例
</h4>

重新命名最近的 session，以便稍後更容易找到。新標題在後續讀取時出現在 [`SDKSessionInfo.custom_title`](#return-type-sdksessioninfo) 中。

```python theme={null}
from claude_agent_sdk import list_sessions, rename_session

sessions = list_sessions(directory="/path/to/project", limit=1)
if sessions:
    rename_session(sessions[0].session_id, "Refactor auth module")
```

<h3 id="tag_session">
  `tag_session()`
</h3>

標記 session。傳遞 `None` 以清除標籤。重複呼叫是安全的；最新的標籤獲勝。同步。

```python theme={null}
def tag_session(
    session_id: str,
    tag: str | None,
    directory: str | None = None,
) -> None
```

<h4 id="parameters-8">
  參數
</h4>

| 參數           | 類型            | 預設     | 描述                                 |
| :----------- | :------------ | :----- | :--------------------------------- |
| `session_id` | `str`         | 必需     | 要標記的 session 的 UUID                |
| `tag`        | `str \| None` | 必需     | 標籤字串，或 `None` 以清除。儲存前進行 Unicode 清理 |
| `directory`  | `str \| None` | `None` | 專案目錄路徑。省略時，搜尋所有專案目錄                |

如果 `session_id` 不是有效的 UUID 或 `tag` 在清理後為空，則引發 `ValueError`；如果找不到 session，則引發 `FileNotFoundError`。

<h4 id="example-7">
  範例
</h4>

標記 session，然後在稍後的讀取中按該標籤篩選。傳遞 `None` 以清除現有標籤。

```python theme={null}
from claude_agent_sdk import list_sessions, tag_session

# Tag a session
tag_session("550e8400-e29b-41d4-a716-446655440000", "needs-review")

# Later: find all sessions with that tag
for session in list_sessions(directory="/path/to/project"):
    if session.tag == "needs-review":
        print(session.summary)
```

<h2 id="classes">
  類別
</h2>

<h3 id="claudesdkclient">
  `ClaudeSDKClient`
</h3>

**在多個交換中維持對話 session。** 這是 TypeScript SDK 的 `query()` 函數內部工作方式的 Python 等效物 - 它建立一個可以繼續對話的客戶端物件。

<h4 id="key-features">
  主要功能
</h4>

* **Session 連續性**：在多個 `query()` 呼叫中維持對話上下文
* **相同對話**：session 保留先前的消息
* **中斷支援**：可以在任務中途停止執行
* **明確生命週期**：您控制 session 何時開始和結束
* **回應驅動流程**：可以對回應做出反應並發送後續消息
* **自訂 tools 和 hooks**：支援自訂 tools（使用 `@tool` 裝飾器建立）和 hooks

```python theme={null}
class ClaudeSDKClient:
    def __init__(self, options: ClaudeAgentOptions | None = None, transport: Transport | None = None)
    async def connect(self, prompt: str | AsyncIterable[dict] | None = None) -> None
    async def query(self, prompt: str | AsyncIterable[dict], session_id: str = "default") -> None
    async def receive_messages(self) -> AsyncIterator[Message]
    async def receive_response(self) -> AsyncIterator[Message]
    async def interrupt(self) -> None
    async def set_permission_mode(self, mode: str) -> None
    async def set_model(self, model: str | None = None) -> None
    async def rewind_files(self, user_message_id: str) -> None
    async def get_mcp_status(self) -> McpStatusResponse
    async def reconnect_mcp_server(self, server_name: str) -> None
    async def toggle_mcp_server(self, server_name: str, enabled: bool) -> None
    async def stop_task(self, task_id: str) -> None
    async def get_server_info(self) -> dict[str, Any] | None
    async def disconnect(self) -> None
```

<h4 id="methods">
  方法
</h4>

| 方法                                        | 描述                                                                                                              |
| :---------------------------------------- | :-------------------------------------------------------------------------------------------------------------- |
| `__init__(options)`                       | 使用可選配置初始化客戶端                                                                                                    |
| `connect(prompt)`                         | 使用可選初始提示或消息流連接到 Claude                                                                                          |
| `query(prompt, session_id)`               | 以串流模式發送新請求                                                                                                      |
| `receive_messages()`                      | 以非同步迭代器接收來自 Claude 的所有消息                                                                                        |
| `receive_response()`                      | 接收消息直到並包括 ResultMessage                                                                                         |
| `interrupt()`                             | 發送中斷信號（僅在串流模式下工作）                                                                                               |
| `set_permission_mode(mode)`               | 變更目前 session 的權限模式                                                                                              |
| `set_model(model)`                        | 變更目前 session 的模型。傳遞 `None` 以重設為預設值                                                                              |
| `rewind_files(user_message_id)`           | 將檔案還原到指定使用者消息時的狀態。需要 `enable_file_checkpointing=True`。見 [檔案 checkpointing](/docs/zh-TW/agent-sdk/file-checkpointing) |
| `get_mcp_status()`                        | 取得所有已配置 MCP 伺服器的狀態。返回 [`McpStatusResponse`](#mcpstatusresponse)                                                 |
| `reconnect_mcp_server(server_name)`       | 重試連接到失敗或斷開連接的 MCP 伺服器                                                                                           |
| `toggle_mcp_server(server_name, enabled)` | 在 session 中途啟用或停用 MCP 伺服器。停用會移除其 tools                                                                          |
| `stop_task(task_id)`                      | 停止執行中的背景任務。[`TaskNotificationMessage`](#tasknotificationmessage) 的狀態為 `"stopped"` 在消息流中跟隨                       |
| `get_server_info()`                       | 取得伺服器資訊，包括 session ID 和功能                                                                                       |
| `disconnect()`                            | 從 Claude 斷開連接                                                                                                   |

<h4 id="context-manager-support">
  上下文管理器支援
</h4>

客戶端可以用作非同步上下文管理器以進行自動連接管理：

```python theme={null}
async with ClaudeSDKClient() as client:
    await client.query("Hello Claude")
    async for message in client.receive_response():
        print(message)
```

> **重要：** 在迭代消息時，避免使用 `break` 提前退出，因為這可能導致 asyncio 清理問題。相反，讓迭代自然完成或使用標誌來追蹤何時找到所需內容。

<h4 id="example-continuing-a-conversation">
  範例 - 繼續對話
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient, AssistantMessage, TextBlock, ResultMessage


async def main():
    async with ClaudeSDKClient() as client:
        # First question
        await client.query("What's the capital of France?")

        # Process response
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")

        # Follow-up question - the session retains the previous context
        await client.query("What's the population of that city?")

        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")

        # Another follow-up - still in the same conversation
        await client.query("What are some famous landmarks there?")

        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")


asyncio.run(main())
```

<h4 id="example-streaming-input-with-claudesdkclient">
  範例 - 使用 ClaudeSDKClient 進行串流輸入
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient


async def message_stream():
    """Generate messages dynamically."""
    yield {
        "type": "user",
        "message": {"role": "user", "content": "Analyze the following data:"},
    }
    await asyncio.sleep(0.5)
    yield {
        "type": "user",
        "message": {"role": "user", "content": "Temperature: 25°C, Humidity: 60%"},
    }
    await asyncio.sleep(0.5)
    yield {
        "type": "user",
        "message": {"role": "user", "content": "What patterns do you see?"},
    }


async def main():
    async with ClaudeSDKClient() as client:
        # Stream input to Claude
        await client.query(message_stream())

        # Process response
        async for message in client.receive_response():
            print(message)

        # Follow-up in same session
        await client.query("Should we be concerned about these readings?")

        async for message in client.receive_response():
            print(message)


asyncio.run(main())
```

<h4 id="example-using-interrupts">
  範例 - 使用中斷
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, ResultMessage


async def interruptible_task():
    options = ClaudeAgentOptions(allowed_tools=["Bash"], permission_mode="acceptEdits")

    async with ClaudeSDKClient(options=options) as client:
        # Start a long-running task
        await client.query("Count from 1 to 100 slowly, using the bash sleep command")

        # Let it run for a bit
        await asyncio.sleep(2)

        # Interrupt the task
        await client.interrupt()
        print("Task interrupted!")

        # Drain the interrupted task's messages (including its ResultMessage)
        async for message in client.receive_response():
            if isinstance(message, ResultMessage):
                print(f"Interrupted task finished with subtype={message.subtype!r}")
                # subtype is "error_during_execution" for interrupted tasks

        # Send a new command
        await client.query("Just say hello instead")

        # Now receive the new response
        async for message in client.receive_response():
            if isinstance(message, ResultMessage) and message.subtype == "success":
                print(f"New result: {message.result}")


asyncio.run(interruptible_task())
```

<Note>
  **中斷後的緩衝區行為：** `interrupt()` 發送停止信號但不清除消息緩衝區。已由中斷任務產生的消息，包括其 `ResultMessage`（帶有 `subtype="error_during_execution"`），保留在流中。您必須在讀取新查詢的回應之前使用 `receive_response()` 清空它們。如果您在 `interrupt()` 之後立即發送新查詢並僅呼叫一次 `receive_response()`，您將收到中斷任務的消息，而不是新查詢的回應。
</Note>

<h4 id="example-advanced-permission-control">
  範例 - 進階權限控制
</h4>

```python theme={null}
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions
from claude_agent_sdk.types import (
    PermissionResultAllow,
    PermissionResultDeny,
    ToolPermissionContext,
)


async def custom_permission_handler(
    tool_name: str, input_data: dict, context: ToolPermissionContext
) -> PermissionResultAllow | PermissionResultDeny:
    """Custom logic for tool permissions."""

    # Block writes to system directories
    if tool_name == "Write" and input_data.get("file_path", "").startswith("/system/"):
        return PermissionResultDeny(
            message="System directory write not allowed", interrupt=True
        )

    # Redirect sensitive file operations
    if tool_name in ["Write", "Edit"] and "config" in input_data.get("file_path", ""):
        safe_path = f"./sandbox/{input_data['file_path']}"
        return PermissionResultAllow(
            updated_input={**input_data, "file_path": safe_path}
        )

    # Allow everything else
    return PermissionResultAllow(updated_input=input_data)


async def main():
    # 不要同時在 allowed_tools 中列出受限 tools：allow 規則在 can_use_tool 執行前批准呼叫
    options = ClaudeAgentOptions(can_use_tool=custom_permission_handler)

    async with ClaudeSDKClient(options=options) as client:
        await client.query("Update the system config file")

        async for message in client.receive_response():
            # 將使用 sandbox 路徑代替
            print(message)


asyncio.run(main())
```

<h2 id="types">
  類型
</h2>

<Note>
  **`@dataclass` vs `TypedDict`：** 此 SDK 使用兩種類型。用 `@dataclass` 裝飾的類別（例如 `ResultMessage`、`AgentDefinition`、`TextBlock`）在執行時是物件實例，支援屬性存取：`msg.result`。用 `TypedDict` 定義的類別（例如 `ThinkingConfigEnabled`、`McpStdioServerConfig`、`SyncHookJSONOutput`）在執行時是**純字典**，需要鍵存取：`config["budget_tokens"]`，而不是 `config.budget_tokens`。`ClassName(field=value)` 呼叫語法對兩者都有效，但只有 dataclasses 產生具有屬性的物件。
</Note>

<h3 id="sdkmcptool">
  `SdkMcpTool`
</h3>

使用 `@tool` 裝飾器建立的 SDK MCP tool 的定義。

```python theme={null}
@dataclass
class SdkMcpTool(Generic[T]):
    name: str
    description: str
    input_schema: type[T] | dict[str, Any]
    handler: Callable[[T], Awaitable[dict[str, Any]]]
    annotations: ToolAnnotations | None = None
```

| 屬性             | 類型                                         | 描述                                                                                  |
| :------------- | :----------------------------------------- | :---------------------------------------------------------------------------------- |
| `name`         | `str`                                      | tool 的唯一識別碼                                                                         |
| `description`  | `str`                                      | 人類可讀描述                                                                              |
| `input_schema` | `type[T] \| dict[str, Any]`                | 輸入驗證的架構                                                                             |
| `handler`      | `Callable[[T], Awaitable[dict[str, Any]]]` | 處理 tool 執行的非同步函數                                                                    |
| `annotations`  | `ToolAnnotations \| None`                  | 可選的 MCP tool 註解（例如 `readOnlyHint`、`destructiveHint`、`openWorldHint`）。來自 `mcp.types` |

<h3 id="transport">
  `Transport`
</h3>

自訂傳輸實現的抽象基類。使用此來透過自訂通道與 Claude 程序通訊（例如，遠端連接而不是本地子程序）。

<Warning>
  這是一個低級內部 API。介面可能在未來版本中變更。自訂實現必須更新以符合任何介面變更。
</Warning>

```python theme={null}
from abc import ABC, abstractmethod
from collections.abc import AsyncIterator
from typing import Any


class Transport(ABC):
    @abstractmethod
    async def connect(self) -> None: ...

    @abstractmethod
    async def write(self, data: str) -> None: ...

    @abstractmethod
    def read_messages(self) -> AsyncIterator[dict[str, Any]]: ...

    @abstractmethod
    async def close(self) -> None: ...

    @abstractmethod
    def is_ready(self) -> bool: ...

    @abstractmethod
    async def end_input(self) -> None: ...
```

| 方法                | 描述                       |
| :---------------- | :----------------------- |
| `connect()`       | 連接傳輸並準備通訊                |
| `write(data)`     | 將原始資料（JSON + 換行符）寫入傳輸    |
| `read_messages()` | 非同步迭代器，產生解析的 JSON 消息     |
| `close()`         | 關閉連接並清理資源                |
| `is_ready()`      | 如果傳輸可以發送和接收，返回 `True`    |
| `end_input()`     | 關閉輸入流（例如，為子程序傳輸關閉 stdin） |

匯入：`from claude_agent_sdk import Transport`

<h3 id="claudeagentoptions">
  `ClaudeAgentOptions`
</h3>

Claude Code 查詢的配置 dataclass。

```python theme={null}
@dataclass
class ClaudeAgentOptions:
    tools: list[str] | ToolsPreset | None = None
    allowed_tools: list[str] = field(default_factory=list)
    system_prompt: str | SystemPromptPreset | SystemPromptFile | None = None
    mcp_servers: dict[str, McpServerConfig] | str | Path = field(default_factory=dict)
    strict_mcp_config: bool = False
    permission_mode: PermissionMode | None = None
    continue_conversation: bool = False
    resume: str | None = None
    max_turns: int | None = None
    max_budget_usd: float | None = None
    disallowed_tools: list[str] = field(default_factory=list)
    model: str | None = None
    fallback_model: str | None = None
    betas: list[SdkBeta] = field(default_factory=list)
    output_format: dict[str, Any] | None = None
    permission_prompt_tool_name: str | None = None
    cwd: str | Path | None = None
    cli_path: str | Path | None = None
    settings: str | None = None
    add_dirs: list[str | Path] = field(default_factory=list)
    env: dict[str, str] = field(default_factory=dict)
    extra_args: dict[str, str | None] = field(default_factory=dict)
    max_buffer_size: int | None = None
    debug_stderr: Any = sys.stderr  # Deprecated
    stderr: Callable[[str], None] | None = None
    can_use_tool: CanUseTool | None = None
    hooks: dict[HookEvent, list[HookMatcher]] | None = None
    user: str | None = None
    include_partial_messages: bool = False
    include_hook_events: bool = False
    fork_session: bool = False
    agents: dict[str, AgentDefinition] | None = None
    setting_sources: list[SettingSource] | None = None
    skills: list[str] | Literal["all"] | None = None
    sandbox: SandboxSettings | None = None
    plugins: list[SdkPluginConfig] = field(default_factory=list)
    max_thinking_tokens: int | None = None  # Deprecated: use thinking instead
    thinking: ThinkingConfig | None = None
    effort: EffortLevel | None = None
    enable_file_checkpointing: bool = False
    session_store: SessionStore | None = None
    session_store_flush: SessionStoreFlushMode = "batched"
```

| 屬性                            | 類型                                                                                       | 預設                   | 描述                                                                                                                                                                                                                                                       |
| :---------------------------- | :--------------------------------------------------------------------------------------- | :------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tools`                       | `list[str] \| ToolsPreset \| None`                                                       | `None`               | Tools 配置。使用 `{"type": "preset", "preset": "claude_code"}` 以取得 Claude Code 的預設 tools                                                                                                                                                                      |
| `allowed_tools`               | `list[str]`                                                                              | `[]`                 | 自動批准的 tools，無需提示。這不會限制 Claude 僅使用這些 tools；未列出的 tools 會進入 `permission_mode` 和 `can_use_tool`。使用 `disallowed_tools` 來阻止 tools。見 [權限](/docs/zh-TW/agent-sdk/permissions#allow-and-deny-rules)                                                                    |
| `system_prompt`               | `str \| SystemPromptPreset \| SystemPromptFile \| None`                                  | `None`               | 系統提示配置。傳遞字串以取得自訂提示，`{"type": "preset", "preset": "claude_code"}` 以取得 Claude Code 的系統提示（可選 `"append"`），或 `{"type": "file", "path": "..."}` 以從磁碟載入大型提示。見 [`SystemPromptPreset`](#systempromptpreset) 和 [`SystemPromptFile`](#systempromptfile)             |
| `mcp_servers`                 | `dict[str, McpServerConfig] \| str \| Path`                                              | `{}`                 | MCP 伺服器配置或配置檔案路徑                                                                                                                                                                                                                                         |
| `strict_mcp_config`           | `bool`                                                                                   | `False`              | 當為 `True` 時，僅使用在 `mcp_servers` 中傳遞的伺服器，並忽略專案 `.mcp.json`、使用者設定、外掛程式提供的 MCP 伺服器和 [claude.ai 連接器](/docs/zh-TW/mcp#use-mcp-servers-from-claude-ai)。對應到 CLI `--strict-mcp-config` 旗標                                                                              |
| `permission_mode`             | `PermissionMode \| None`                                                                 | `None`               | tool 使用的權限模式                                                                                                                                                                                                                                             |
| `continue_conversation`       | `bool`                                                                                   | `False`              | 繼續最近的對話                                                                                                                                                                                                                                                  |
| `resume`                      | `str \| None`                                                                            | `None`               | 要繼續的 session ID                                                                                                                                                                                                                                          |
| `max_turns`                   | `int \| None`                                                                            | `None`               | 最大代理轉數（tool 使用往返）                                                                                                                                                                                                                                        |
| `max_budget_usd`              | `float \| None`                                                                          | `None`               | 當客戶端成本估計達到此 USD 值時停止查詢。與 `total_cost_usd` 的相同估計進行比較；見 [追蹤成本和使用情況](/docs/zh-TW/agent-sdk/cost-tracking) 以了解準確性注意事項                                                                                                                                             |
| `disallowed_tools`            | `list[str]`                                                                              | `[]`                 | 要拒絕的 tools。裸名稱（例如 `"Bash"`）會從 Claude 的上下文中移除該 tool。範圍規則（例如 `"Bash(rm *)"`）會保留該 tool 可用，並在每個權限模式（包括 `bypassPermissions`）中拒絕匹配的呼叫。見 [權限](/docs/zh-TW/agent-sdk/permissions#allow-and-deny-rules)                                                                |
| `enable_file_checkpointing`   | `bool`                                                                                   | `False`              | 啟用檔案變更追蹤以進行倒帶。見 [檔案 checkpointing](/docs/zh-TW/agent-sdk/file-checkpointing)                                                                                                                                                                                  |
| `model`                       | `str \| None`                                                                            | `None`               | Claude 模型別名或完整模型名稱。見 [接受的值和提供者特定 ID](/docs/zh-TW/model-config#available-models)                                                                                                                                                                               |
| `fallback_model`              | `str \| None`                                                                            | `None`               | 如果主模型失敗，使用的備用模型                                                                                                                                                                                                                                          |
| `betas`                       | `list[SdkBeta]`                                                                          | `[]`                 | 要啟用的測試版功能。見 [`SdkBeta`](#sdkbeta) 以了解可用選項                                                                                                                                                                                                                |
| `output_format`               | `dict[str, Any] \| None`                                                                 | `None`               | 結構化回應的輸出格式（例如 `{"type": "json_schema", "schema": {...}}`）。見 [結構化輸出](/docs/zh-TW/agent-sdk/structured-outputs) 以了解詳情                                                                                                                                           |
| `permission_prompt_tool_name` | `str \| None`                                                                            | `None`               | 權限提示的 MCP tool 名稱                                                                                                                                                                                                                                        |
| `cwd`                         | `str \| Path \| None`                                                                    | `None`               | 目前工作目錄                                                                                                                                                                                                                                                   |
| `cli_path`                    | `str \| Path \| None`                                                                    | `None`               | Claude Code CLI 可執行檔的自訂路徑                                                                                                                                                                                                                                |
| `settings`                    | `str \| None`                                                                            | `None`               | 設定檔案的路徑                                                                                                                                                                                                                                                  |
| `add_dirs`                    | `list[str \| Path]`                                                                      | `[]`                 | Claude 可以存取的其他目錄                                                                                                                                                                                                                                         |
| `env`                         | `dict[str, str]`                                                                         | `{}`                 | 環境變數合併到繼承的程序環境之上。見 [環境變數](/docs/zh-TW/env-vars) 以了解底層 CLI 讀取的變數，以及 [處理緩慢或停滯的 API 回應](#handle-slow-or-stalled-api-responses) 以了解逾時相關變數                                                                                                                         |
| `extra_args`                  | `dict[str, str \| None]`                                                                 | `{}`                 | 直接傳遞給 CLI 的其他 CLI 參數                                                                                                                                                                                                                                     |
| `max_buffer_size`             | `int \| None`                                                                            | `None`               | 緩衝 CLI stdout 時的最大位元組數                                                                                                                                                                                                                                   |
| `debug_stderr`                | `Any`                                                                                    | `sys.stderr`         | *已棄用* - 用於偵錯輸出的類似檔案的物件。改用 `stderr` 回呼                                                                                                                                                                                                                    |
| `stderr`                      | `Callable[[str], None] \| None`                                                          | `None`               | 用於 CLI stderr 輸出的回呼函數                                                                                                                                                                                                                                    |
| `can_use_tool`                | [`CanUseTool`](#canusetool) ` \| None`                                                   | `None`               | tool 權限回呼函數，僅在[權限流程](/docs/zh-TW/agent-sdk/permissions#how-permissions-are-evaluated)進入提示時呼叫。不會為由 `allowed_tools`、允許規則或 `permission_mode` 自動批准的呼叫呼叫。見 [`CanUseTool`](#canusetool) 以了解詳情                                                                       |
| `hooks`                       | `dict[HookEvent, list[HookMatcher]] \| None`                                             | `None`               | 用於攔截事件的 hooks 配置                                                                                                                                                                                                                                         |
| `user`                        | `str \| None`                                                                            | `None`               | 使用者識別碼                                                                                                                                                                                                                                                   |
| `include_partial_messages`    | `bool`                                                                                   | `False`              | 包括部分消息串流事件。啟用時，[`StreamEvent`](#streamevent) 消息會被產生                                                                                                                                                                                                      |
| `include_hook_events`         | `bool`                                                                                   | `False`              | 在消息流中包括 hook 生命週期事件作為 `HookEventMessage` 物件                                                                                                                                                                                                              |
| `fork_session`                | `bool`                                                                                   | `False`              | 使用 `resume` 繼續時，分叉到新 session ID 而不是繼續原始 session                                                                                                                                                                                                          |
| `agents`                      | `dict[str, AgentDefinition] \| None`                                                     | `None`               | 以程式設計方式定義的子代理                                                                                                                                                                                                                                            |
| `plugins`                     | `list[SdkPluginConfig]`                                                                  | `[]`                 | 從本地路徑載入自訂外掛程式。見 [外掛程式](/docs/zh-TW/agent-sdk/plugins) 以了解詳情                                                                                                                                                                                                   |
| `sandbox`                     | [`SandboxSettings`](#sandboxsettings) ` \| None`                                         | `None`               | 以程式設計方式配置沙箱行為。見 [沙箱設定](#sandboxsettings) 以了解詳情                                                                                                                                                                                                           |
| `setting_sources`             | `list[SettingSource] \| None`                                                            | `None`（CLI 預設值：所有來源） | 控制要載入哪些檔案系統設定。傳遞 `[]` 以停用使用者、專案和本地設定。無論如何都會載入受管原則設定；當 session 使用組織認證在[符合條件的配置](/docs/zh-TW/server-managed-settings#platform-availability)上進行驗證時，會擷取伺服器管理的設定。見 [使用 Claude Code 功能](/docs/zh-TW/agent-sdk/claude-code-features#what-settingsources-does-not-control) |
| `skills`                      | `list[str] \| Literal["all"] \| None`                                                    | `None`               | 可供 session 使用的 skills。傳遞 `"all"` 以啟用每個發現的 skill，或傳遞 skill 名稱清單。設定時，SDK 會自動將 Skill tool 新增到 `allowed_tools`。如果您也傳遞 `tools`，請在該清單中包括 `"Skill"`。見 [Skills](/docs/zh-TW/agent-sdk/skills)                                                                         |
| `max_thinking_tokens`         | `int \| None`                                                                            | `None`               | *已棄用* - 思考區塊的最大令牌數。改用 `thinking`                                                                                                                                                                                                                         |
| `thinking`                    | [`ThinkingConfig`](#thinkingconfig) ` \| None`                                           | `None`               | 控制擴展思考行為。優先於 `max_thinking_tokens`                                                                                                                                                                                                                       |
| `effort`                      | [`EffortLevel`](#effortlevel) ` \| None`                                                 | `None`               | 思考深度的努力級別。見 [調整努力級別](/docs/zh-TW/model-config#adjust-effort-level)                                                                                                                                                                                            |
| `session_store`               | [`SessionStore`](/docs/zh-TW/agent-sdk/session-storage#the-sessionstore-interface) ` \| None` | `None`               | 將 session 記錄鏡像到外部後端，以便任何主機都可以繼續它們。見 [將 sessions 持久化到外部儲存](/docs/zh-TW/agent-sdk/session-storage)                                                                                                                                                              |
| `session_store_flush`         | `Literal["batched", "eager"]`                                                            | `"batched"`          | 何時將鏡像的記錄項目刷新到 `session_store`。`"batched"` 每轉一次或當緩衝區填滿時刷新；`"eager"` 在每個框架後觸發背景刷新。當 `session_store` 為 `None` 時忽略                                                                                                                                           |

<h4 id="handle-slow-or-stalled-api-responses">
  處理緩慢或停滯的 API 回應
</h4>

CLI 子程序讀取多個環境變數，控制 API 逾時和停滯偵測。透過 `ClaudeAgentOptions.env` 傳遞它們：

```python theme={null}
options = ClaudeAgentOptions(
    env={
        "API_TIMEOUT_MS": "120000",
        "CLAUDE_CODE_MAX_RETRIES": "2",
        "CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS": "120000",
    },
)
```

* `API_TIMEOUT_MS`：Anthropic 客戶端上的每個請求逾時，以毫秒為單位。預設 `600000`。適用於主迴圈和所有子代理。
* `CLAUDE_CODE_MAX_RETRIES`：最大 API 重試次數。預設 `10`，上限為 `15`。每次重試都有自己的 `API_TIMEOUT_MS` 視窗，因此最壞情況下的牆時間大約是 `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` 加上退避。對於需要等待更長中斷的無人值守執行，設定 `CLAUDE_CODE_RETRY_WATCHDOG=1`：它無限期重試容量錯誤，自 Claude Code v2.1.199 起，會將其他暫時性錯誤的預設值提高到 `300` 並移除此變數的上限。
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`：使用 `run_in_background` 啟動的子代理的停滯監視程式。預設 `600000`。在每個串流事件上重置；停滯時中止子代理，將任務標記為失敗，並將錯誤呈現給父代理，包含任何部分結果。不適用於同步子代理。
* `CLAUDE_ENABLE_STREAM_WATCHDOG` 搭配 `CLAUDE_STREAM_IDLE_TIMEOUT_MS`：當標頭已到達但回應本體停止串流時中止請求。監視程式預設在所有提供者上啟用；設定 `CLAUDE_ENABLE_STREAM_WATCHDOG=0` 以停用它。`CLAUDE_STREAM_IDLE_TIMEOUT_MS` 預設為 `300000` 並限制在該最小值。中止的請求會經過正常重試路徑。

<h3 id="outputformat">
  `OutputFormat`
</h3>

結構化輸出驗證的配置。將此作為 `dict` 傳遞給 `ClaudeAgentOptions` 上的 `output_format` 欄位：

```python theme={null}
# Expected dict shape for output_format
{
    "type": "json_schema",
    "schema": {...},  # Your JSON Schema definition
}
```

| 欄位       | 必需 | 描述                                     |
| :------- | :- | :------------------------------------- |
| `type`   | 是  | 必須是 `"json_schema"` 以進行 JSON Schema 驗證 |
| `schema` | 是  | 用於輸出驗證的 JSON Schema 定義                 |

<h3 id="systempromptpreset">
  `SystemPromptPreset`
</h3>

使用 Claude Code 的預設系統提示配置，可選新增。

```python theme={null}
class SystemPromptPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
    append: NotRequired[str]
    exclude_dynamic_sections: NotRequired[bool]
```

| 欄位                         | 必需 | 描述                                                                                                                                                                            |
| :------------------------- | :- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`                     | 是  | 必須是 `"preset"` 以使用預設系統提示                                                                                                                                                      |
| `preset`                   | 是  | 必須是 `"claude_code"` 以使用 Claude Code 的系統提示                                                                                                                                     |
| `append`                   | 否  | 要附加到預設系統提示的其他指示                                                                                                                                                               |
| `exclude_dynamic_sections` | 否  | 將每個 session 上下文（例如工作目錄、git 狀態和記憶體路徑）從系統提示移到第一個使用者消息。改進跨使用者和機器的提示快取重複使用。見 [修改系統提示](/docs/zh-TW/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) |

<h3 id="systempromptfile">
  `SystemPromptFile`
</h3>

用於從檔案而不是作為字串傳遞自訂系統提示的配置。SDK 將此對應到 CLI [`--system-prompt-file`](/docs/zh-TW/cli-reference#system-prompt-flags) 旗標。當提示很大時使用檔案形式：SDK 在 CLI 子程序 argv 上傳遞字串 `system_prompt`，受限於 OS 命令列長度限制，在 SDK 發送任何 API 請求之前。在 Linux 上，單個參數長於大約 128 KB 會在程序生成時失敗，出現 `Argument list too long`。在 Windows 上，整個命令列上限為大約 32 KB，因此字串形式在較低閾值失敗。

```python theme={null}
class SystemPromptFile(TypedDict):
    type: Literal["file"]
    path: str
```

| 欄位     | 必需 | 描述                    |
| :----- | :- | :-------------------- |
| `type` | 是  | 必須是 `"file"` 以從磁碟載入提示 |
| `path` | 是  | 包含系統提示的檔案路徑           |

<h3 id="settingsource">
  `SettingSource`
</h3>

控制 SDK 從哪些檔案系統配置來源載入設定。

```python theme={null}
SettingSource = Literal["user", "project", "local"]
```

| 值           | 描述            | 位置                            |
| :---------- | :------------ | :---------------------------- |
| `"user"`    | 全域使用者設定       | `~/.claude/settings.json`     |
| `"project"` | 共享專案設定（版本控制）  | `.claude/settings.json`       |
| `"local"`   | 本地專案設定（未版本控制） | `.claude/settings.local.json` |

<h4 id="default-behavior">
  預設行為
</h4>

當 `setting_sources` 被省略或為 `None` 時，`query()` 載入與 Claude Code CLI 相同的檔案系統設定：使用者、專案和本地。無論如何都會載入受管原則設定；當 session 使用組織認證在[符合條件的配置](/docs/zh-TW/server-managed-settings#platform-availability)上進行驗證時，會擷取伺服器管理的設定。見 [settingSources 不控制的內容](/docs/zh-TW/agent-sdk/claude-code-features#what-settingsources-does-not-control) 以了解無論此選項如何都會讀取的輸入，以及如何停用它們。

<h4 id="why-use-setting_sources">
  為什麼使用 setting\_sources
</h4>

**停用檔案系統設定：**

```python theme={null}
# Do not load user, project, or local settings from disk
from claude_agent_sdk import query, ClaudeAgentOptions

async for message in query(
    prompt="Analyze this code",
    options=ClaudeAgentOptions(
        setting_sources=[]
    ),
):
    print(message)
```

<Note>
  在 Python SDK 0.1.59 及更早版本中，空清單的處理方式與省略選項相同，因此 `setting_sources=[]` 沒有停用檔案系統設定。如果您需要空清單生效，請升級到較新版本。TypeScript SDK 不受影響。
</Note>

**明確載入所有檔案系統設定：**

```python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions

async for message in query(
    prompt="Analyze this code",
    options=ClaudeAgentOptions(
        setting_sources=["user", "project", "local"]
    ),
):
    print(message)
```

**僅載入特定設定來源：**

```python theme={null}
# Load only project settings, ignore user and local
async for message in query(
    prompt="Run CI checks",
    options=ClaudeAgentOptions(
        setting_sources=["project"]  # Only .claude/settings.json
    ),
):
    print(message)
```

**測試和 CI 環境：**

```python theme={null}
# Ensure consistent behavior in CI by excluding local settings
async for message in query(
    prompt="Run tests",
    options=ClaudeAgentOptions(
        setting_sources=["project"],  # Only team-shared settings
        permission_mode="bypassPermissions",
    ),
):
    print(message)
```

**僅 SDK 應用程式：**

```python theme={null}
# Define everything programmatically.
# Pass [] to opt out of filesystem setting sources.
async for message in query(
    prompt="Review this PR",
    options=ClaudeAgentOptions(
        setting_sources=[],
        agents={...},
        mcp_servers={...},
        allowed_tools=["Read", "Grep", "Glob"],
    ),
):
    print(message)
```

**載入 CLAUDE.md 專案指示：**

```python theme={null}
# Load project settings to include CLAUDE.md files
async for message in query(
    prompt="Add a new feature following project conventions",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",  # Use Claude Code's system prompt
        },
        setting_sources=["project"],  # Loads CLAUDE.md from project
        allowed_tools=["Read", "Write", "Edit"],
    ),
):
    print(message)
```

<h4 id="settings-precedence">
  設定優先順序
</h4>

當載入多個來源時，設定會以此優先順序合併（最高到最低）：

1. 本地設定（`.claude/settings.local.json`）
2. 專案設定（`.claude/settings.json`）
3. 使用者設定（`~/.claude/settings.json`）

程式設計選項（例如 `agents` 和 `allowed_tools`）會覆蓋使用者、專案和本地檔案系統設定。受管原則設定優先於程式設計選項。

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

以程式設計方式定義的子代理的配置。

```python theme={null}
@dataclass
class AgentDefinition:
    description: str
    prompt: str
    tools: list[str] | None = None
    disallowedTools: list[str] | None = None
    model: str | None = None
    skills: list[str] | None = None
    memory: Literal["user", "project", "local"] | None = None
    mcpServers: list[str | dict[str, Any]] | None = None
    initialPrompt: str | None = None
    maxTurns: int | None = None
    background: bool | None = None
    effort: EffortLevel | int | None = None
    permissionMode: PermissionMode | None = None
```

| 欄位                | 必需 | 描述                                                                                                                       |
| :---------------- | :- | :----------------------------------------------------------------------------------------------------------------------- |
| `description`     | 是  | 何時使用此代理的自然語言描述                                                                                                           |
| `prompt`          | 是  | 代理的系統提示                                                                                                                  |
| `tools`           | 否  | 允許的 tool 名稱陣列。如果省略，繼承所有 tools                                                                                            |
| `disallowedTools` | 否  | 要從代理的 tool 集中移除的 tool 名稱陣列。也接受 MCP 伺服器級別的模式：`mcp__server` 或 `mcp__server__*` 移除該伺服器的每個 tool，`mcp__*` 移除任何伺服器的每個 MCP tool |
| `model`           | 否  | 此代理的模型覆蓋。接受別名，例如 `"sonnet"`、`"opus"`、`"haiku"` 或 `"inherit"`，或完整模型 ID。如果省略，使用主模型                                         |
| `skills`          | 否  | 此代理可用的 skill 名稱清單                                                                                                        |
| `memory`          | 否  | 此代理的記憶體來源：`"user"`、`"project"` 或 `"local"`                                                                               |
| `mcpServers`      | 否  | 此代理可用的 MCP 伺服器。每個項目是伺服器名稱或內聯 `{name: config}` 字典                                                                         |
| `initialPrompt`   | 否  | 當此代理作為主執行緒代理執行時自動提交為第一個使用者轉                                                                                              |
| `maxTurns`        | 否  | 代理停止前的最大代理轉數                                                                                                             |
| `background`      | 否  | 當呼叫時將此代理作為非阻塞背景任務執行                                                                                                      |
| `effort`          | 否  | 此代理的推理努力級別。接受命名級別或整數。見 [`EffortLevel`](#effortlevel)                                                                     |
| `permissionMode`  | 否  | 此代理內 tool 執行的權限模式。見 [`PermissionMode`](#permissionmode)                                                                  |

<Note>
  `AgentDefinition` 欄位名稱使用 camelCase，例如 `disallowedTools`、`permissionMode` 和 `maxTurns`。這些名稱直接對應到與 TypeScript SDK 共享的線路格式。這與 `ClaudeAgentOptions` 不同，後者對等頂級欄位（例如 `disallowed_tools` 和 `permission_mode`）使用 Python snake\_case。因為 `AgentDefinition` 是 dataclass，傳遞 snake\_case 關鍵字在構造時會引發 `TypeError`。
</Note>

<h3 id="permissionmode">
  `PermissionMode`
</h3>

用於控制 tool 執行的權限模式。

```python theme={null}
PermissionMode = Literal[
    "default",  # Standard permission behavior
    "acceptEdits",  # Auto-accept file edits
    "plan",  # Planning mode - explore without editing
    "dontAsk",  # Deny anything not pre-approved instead of prompting
    "bypassPermissions",  # Bypass permission checks; explicit ask rules still prompt (use with caution)
    "auto",  # A model classifier approves or denies each tool call
]
```

<h3 id="effortlevel">
  `EffortLevel`
</h3>

用於指導思考深度的努力級別。

```python theme={null}
EffortLevel = Literal[
    "low",  # Minimal thinking, fastest responses
    "medium",  # Moderate thinking
    "high",  # Deep reasoning
    "xhigh",  # Extended reasoning; falls back to "high" on models that don't support it
    "max",  # Maximum effort
]
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

tool 權限回呼函數的類型別名。

```python theme={null}
CanUseTool = Callable[
    [str, dict[str, Any], ToolPermissionContext], Awaitable[PermissionResult]
]
```

回呼接收：

* `tool_name`：被呼叫的 tool 名稱
* `input_data`：tool 的輸入參數
* `context`：具有其他資訊的 `ToolPermissionContext`

返回 `PermissionResult`（`PermissionResultAllow` 或 `PermissionResultDeny`）。

回呼是互動式權限提示的 SDK 替代品：它僅在[權限評估流程](/docs/zh-TW/agent-sdk/permissions#how-permissions-are-evaluated)解決為提示時呼叫。由 `allowed_tools` 項目、設定允許規則或權限模式（例如 `acceptEdits` 或 `bypassPermissions`）已批准的 tool 呼叫永遠不會呼叫它。要限制每個 tool 呼叫，改用 [`PreToolUse` hook](/docs/zh-TW/agent-sdk/hooks)。

`AskUserQuestion`、標記為 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP tools，以及您的組織設定為 [`ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools) 的連接器 tools 即使允許規則相符也會到達回呼。在 `dontAsk` 模式下，這些呼叫會被拒絕，而不呼叫回呼。

<h3 id="toolpermissioncontext">
  `ToolPermissionContext`
</h3>

傳遞給 tool 權限回呼的上下文資訊。

```python theme={null}
@dataclass
class ToolPermissionContext:
    signal: Any | None = None  # Future: abort signal support
    suggestions: list[PermissionUpdate] = field(default_factory=list)
    blocked_path: str | None = None
    decision_reason: str | None = None
    title: str | None = None
    display_name: str | None = None
    description: str | None = None
```

| 欄位                | 類型                       | 描述                                                                                                                                     |
| :---------------- | :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`          | `Any \| None`            | 保留供未來中止信號支援                                                                                                                            |
| `suggestions`     | `list[PermissionUpdate]` | 來自 CLI 的權限更新建議。Bash 提示包括具有 `localSettings` 目的地的建議，因此在 `updated_permissions` 中返回它會將規則寫入 `.claude/settings.local.json` 並在 sessions 中持久化。 |
| `blocked_path`    | `str \| None`            | 觸發權限請求的檔案路徑（如適用）。例如，當 Bash 命令嘗試存取允許目錄外的路徑時                                                                                             |
| `decision_reason` | `str \| None`            | 觸發此權限請求的原因。從 PreToolUse hook 的 `permissionDecisionReason` 轉發，當 hook 返回 `"ask"` 時                                                       |
| `title`           | `str \| None`            | 完整權限提示句子，例如 `Claude wants to read foo.txt`。當存在時用作主要提示文本                                                                                |
| `display_name`    | `str \| None`            | tool 操作的簡短名詞短語，例如 `Read file`，適合按鈕標籤                                                                                                   |
| `description`     | `str \| None`            | 權限 UI 的人類可讀副標題                                                                                                                         |

<h3 id="permissionresult">
  `PermissionResult`
</h3>

權限回呼結果的聯合類型。

```python theme={null}
PermissionResult = PermissionResultAllow | PermissionResultDeny
```

<h3 id="permissionresultallow">
  `PermissionResultAllow`
</h3>

指示應允許 tool 呼叫的結果。

```python theme={null}
@dataclass
class PermissionResultAllow:
    behavior: Literal["allow"] = "allow"
    updated_input: dict[str, Any] | None = None
    updated_permissions: list[PermissionUpdate] | None = None
```

| 欄位                    | 類型                               | 預設        | 描述              |
| :-------------------- | :------------------------------- | :-------- | :-------------- |
| `behavior`            | `Literal["allow"]`               | `"allow"` | 必須是 "allow"     |
| `updated_input`       | `dict[str, Any] \| None`         | `None`    | 要使用的修改輸入而不是原始輸入 |
| `updated_permissions` | `list[PermissionUpdate] \| None` | `None`    | 要應用的權限更新        |

<h3 id="permissionresultdeny">
  `PermissionResultDeny`
</h3>

指示應拒絕 tool 呼叫的結果。

```python theme={null}
@dataclass
class PermissionResultDeny:
    behavior: Literal["deny"] = "deny"
    message: str = ""
    interrupt: bool = False
```

| 欄位          | 類型                | 預設       | 描述               |
| :---------- | :---------------- | :------- | :--------------- |
| `behavior`  | `Literal["deny"]` | `"deny"` | 必須是 "deny"       |
| `message`   | `str`             | `""`     | 解釋為什麼拒絕 tool 的消息 |
| `interrupt` | `bool`            | `False`  | 是否中斷目前執行         |

<h3 id="permissionupdate">
  `PermissionUpdate`
</h3>

用於以程式設計方式更新權限的配置。

```python theme={null}
@dataclass
class PermissionUpdate:
    type: Literal[
        "addRules",
        "replaceRules",
        "removeRules",
        "setMode",
        "addDirectories",
        "removeDirectories",
    ]
    rules: list[PermissionRuleValue] | None = None
    behavior: Literal["allow", "deny", "ask"] | None = None
    mode: PermissionMode | None = None
    directories: list[str] | None = None
    destination: (
        Literal["userSettings", "projectSettings", "localSettings", "session"] | None
    ) = None
```

| 欄位            | 類型                                        | 描述              |
| :------------ | :---------------------------------------- | :-------------- |
| `type`        | `Literal[...]`                            | 權限更新操作的類型       |
| `rules`       | `list[PermissionRuleValue] \| None`       | 用於新增/取代/移除操作的規則 |
| `behavior`    | `Literal["allow", "deny", "ask"] \| None` | 基於規則的操作的行為      |
| `mode`        | `PermissionMode \| None`                  | setMode 操作的模式   |
| `directories` | `list[str] \| None`                       | 用於新增/移除目錄操作的目錄  |
| `destination` | `Literal[...] \| None`                    | 應用權限更新的位置       |

<h3 id="permissionrulevalue">
  `PermissionRuleValue`
</h3>

要在權限更新中新增、取代或移除的規則。

```python theme={null}
@dataclass
class PermissionRuleValue:
    tool_name: str
    rule_content: str | None = None
```

<h3 id="toolspreset">
  `ToolsPreset`
</h3>

使用 Claude Code 預設 tool 集的預設 tools 配置。

```python theme={null}
class ToolsPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
```

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

控制擴展思考行為。三個配置的聯合：

```python theme={null}
ThinkingDisplay = Literal["summarized", "omitted"]


class ThinkingConfigAdaptive(TypedDict):
    type: Literal["adaptive"]
    display: NotRequired[ThinkingDisplay]


class ThinkingConfigEnabled(TypedDict):
    type: Literal["enabled"]
    budget_tokens: int
    display: NotRequired[ThinkingDisplay]


class ThinkingConfigDisabled(TypedDict):
    type: Literal["disabled"]


ThinkingConfig = ThinkingConfigAdaptive | ThinkingConfigEnabled | ThinkingConfigDisabled
```

| 變體         | 欄位                                 | 描述               |
| :--------- | :--------------------------------- | :--------------- |
| `adaptive` | `type`, `display`                  | Claude 自適應決定何時思考 |
| `enabled`  | `type`, `budget_tokens`, `display` | 啟用具有特定令牌預算的思考    |
| `disabled` | `type`                             | 停用思考             |

可選的 `display` 欄位控制思考文本是否返回為 `"summarized"` 或 `"omitted"`。在 Claude Opus 4.7 及更新版本上，API 預設為 `"omitted"`，因此設定 `"summarized"` 以在 [`ThinkingBlock`](#thinkingblock) 輸出中接收思考內容。

因為這些是 `TypedDict` 類別，它們在執行時是純字典。要麼將它們構造為字典字面量，要麼呼叫類別作為構造函數；兩者都產生 `dict`。使用 `config["budget_tokens"]` 存取欄位，而不是 `config.budget_tokens`：

```python theme={null}
from claude_agent_sdk import ClaudeAgentOptions, ThinkingConfigEnabled

# Option 1: dict literal (recommended, no import needed)
options = ClaudeAgentOptions(thinking={"type": "enabled", "budget_tokens": 20000})

# Option 2: constructor-style (returns a plain dict)
config = ThinkingConfigEnabled(type="enabled", budget_tokens=20000)
print(config["budget_tokens"])  # 20000
# config.budget_tokens would raise AttributeError
```

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

SDK 測試版功能的字面類型。

```python theme={null}
SdkBeta = Literal["context-1m-2025-08-07"]
```

與 `ClaudeAgentOptions` 中的 `betas` 欄位一起使用以啟用測試版功能。

<Warning>
  `context-1m-2025-08-07` 測試版自 2026 年 4 月 30 日起已停用。使用 Claude Sonnet 4.5 或 Sonnet 4 傳遞此標頭沒有效果，超過標準 200k 令牌上下文視窗的請求會返回錯誤。要使用 1M 令牌上下文視窗，請遷移到 [Claude Sonnet 5、Claude Sonnet 4.6、Claude Opus 4.6、Claude Opus 4.7 或 Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview)，它們以標準定價包括 1M 上下文，無需測試版標頭。
</Warning>

<h3 id="mcpsdkserverconfig">
  `McpSdkServerConfig`
</h3>

使用 `create_sdk_mcp_server()` 建立的 SDK MCP 伺服器的配置。

```python theme={null}
class McpSdkServerConfig(TypedDict):
    type: Literal["sdk"]
    name: str
    instance: Any  # MCP Server instance
```

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

MCP 伺服器配置的聯合類型。

```python theme={null}
McpServerConfig = (
    McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig
)
```

<h4 id="mcpstdioserverconfig">
  `McpStdioServerConfig`
</h4>

```python theme={null}
class McpStdioServerConfig(TypedDict):
    type: NotRequired[Literal["stdio"]]  # Optional for backwards compatibility
    command: str
    args: NotRequired[list[str]]
    env: NotRequired[dict[str, str]]
```

<h4 id="mcpsseserverconfig">
  `McpSSEServerConfig`
</h4>

```python theme={null}
class McpSSEServerConfig(TypedDict):
    type: Literal["sse"]
    url: str
    headers: NotRequired[dict[str, str]]
```

<h4 id="mcphttpserverconfig">
  `McpHttpServerConfig`
</h4>

```python theme={null}
class McpHttpServerConfig(TypedDict):
    type: Literal["http"]
    url: str
    headers: NotRequired[dict[str, str]]
```

<h3 id="mcpserverstatusconfig">
  `McpServerStatusConfig`
</h3>

MCP 伺服器的配置，如 [`get_mcp_status()`](#methods) 所報告。這是所有 [`McpServerConfig`](#mcpserverconfig) 傳輸變體加上用於透過 claude.ai 代理的伺服器的僅輸出 `claudeai-proxy` 變體的聯合。

```python theme={null}
McpServerStatusConfig = (
    McpStdioServerConfig
    | McpSSEServerConfig
    | McpHttpServerConfig
    | McpSdkServerConfigStatus
    | McpClaudeAIProxyServerConfig
)
```

`McpSdkServerConfigStatus` 是 [`McpSdkServerConfig`](#mcpsdkserverconfig) 的可序列化形式，僅具有 `type`（`"sdk"`）和 `name`（`str`）欄位；進程內 `instance` 被省略。`McpClaudeAIProxyServerConfig` 具有 `type`（`"claudeai-proxy"`）、`url`（`str`）和 `id`（`str`）欄位。

<h3 id="mcpstatusresponse">
  `McpStatusResponse`
</h3>

來自 [`ClaudeSDKClient.get_mcp_status()`](#methods) 的回應。在 `mcpServers` 鍵下包裝伺服器狀態清單。

```python theme={null}
class McpStatusResponse(TypedDict):
    mcpServers: list[McpServerStatus]
```

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

連接的 MCP 伺服器的狀態，包含在 [`McpStatusResponse`](#mcpstatusresponse) 中。

```python theme={null}
class McpServerStatus(TypedDict):
    name: str
    status: McpServerConnectionStatus  # "connected" | "failed" | "needs-auth" | "pending" | "disabled"
    serverInfo: NotRequired[McpServerInfo]
    error: NotRequired[str]
    config: NotRequired[McpServerStatusConfig]
    scope: NotRequired[str]
    tools: NotRequired[list[McpToolInfo]]
```

| 欄位           | 類型                                                    | 描述                                                                                                                     |
| :----------- | :---------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------- |
| `name`       | `str`                                                 | 伺服器名稱                                                                                                                  |
| `status`     | `str`                                                 | `"connected"`、`"failed"`、`"needs-auth"`、`"pending"` 或 `"disabled"` 之一                                                  |
| `serverInfo` | `dict`（可選）                                            | 伺服器名稱和版本（`{"name": str, "version": str}`）                                                                              |
| `error`      | `str`（可選）                                             | 伺服器連接失敗時的錯誤消息                                                                                                          |
| `config`     | [`McpServerStatusConfig`](#mcpserverstatusconfig)（可選） | 伺服器配置。與 [`McpServerConfig`](#mcpserverconfig) 相同的形狀（stdio、SSE、HTTP 或 SDK），加上用於透過 claude.ai 連接的伺服器的 `claudeai-proxy` 變體 |
| `scope`      | `str`（可選）                                             | 配置範圍                                                                                                                   |
| `tools`      | `list`（可選）                                            | 此伺服器提供的 tools，每個都具有 `name`、`description` 和 `annotations` 欄位                                                            |

<h3 id="sdkpluginconfig">
  `SdkPluginConfig`
</h3>

在 SDK 中載入外掛程式的配置。

```python theme={null}
class SdkPluginConfig(TypedDict):
    type: Literal["local"]
    path: str
```

| 欄位     | 類型                 | 描述                         |
| :----- | :----------------- | :------------------------- |
| `type` | `Literal["local"]` | 必須是 `"local"`（目前僅支援本地外掛程式） |
| `path` | `str`              | 外掛程式目錄的絕對或相對路徑             |

**範例：**

```python theme={null}
plugins = [
    {"type": "local", "path": "./my-plugin"},
    {"type": "local", "path": "/absolute/path/to/plugin"},
]
```

如需建立和使用外掛程式的完整資訊，見 [外掛程式](/docs/zh-TW/agent-sdk/plugins)。

<h2 id="message-types">
  消息類型
</h2>

<h3 id="message">
  `Message`
</h3>

所有可能消息的聯合類型。

```python theme={null}
Message = (
    UserMessage
    | AssistantMessage
    | SystemMessage
    | ResultMessage
    | StreamEvent
    | RateLimitEvent
)
```

<h3 id="usermessage">
  `UserMessage`
</h3>

使用者輸入消息。

```python theme={null}
@dataclass
class UserMessage:
    content: str | list[ContentBlock]
    uuid: str | None = None
    parent_tool_use_id: str | None = None
    tool_use_result: dict[str, Any] | None = None
```

| 欄位                   | 類型                          | 描述                             |
| :------------------- | :-------------------------- | :----------------------------- |
| `content`            | `str \| list[ContentBlock]` | 消息內容為文字或內容區塊                   |
| `uuid`               | `str \| None`               | 唯一消息識別碼                        |
| `parent_tool_use_id` | `str \| None`               | 如果此消息是 tool 結果回應，則為 tool 使用 ID |
| `tool_use_result`    | `dict[str, Any] \| None`    | tool 結果資料（如適用）                 |

<h3 id="assistantmessage">
  `AssistantMessage`
</h3>

具有內容區塊的助手回應消息。

```python theme={null}
@dataclass
class AssistantMessage:
    content: list[ContentBlock]
    model: str
    parent_tool_use_id: str | None = None
    error: AssistantMessageError | None = None
    usage: dict[str, Any] | None = None
    message_id: str | None = None
```

| 欄位                   | 類型                                                           | 描述                                                        |
| :------------------- | :----------------------------------------------------------- | :-------------------------------------------------------- |
| `content`            | `list[ContentBlock]`                                         | 回應中的內容區塊清單                                                |
| `model`              | `str`                                                        | 產生回應的模型                                                   |
| `parent_tool_use_id` | `str \| None`                                                | 如果這是嵌套回應，則為 tool 使用 ID                                    |
| `error`              | [`AssistantMessageError`](#assistantmessageerror) ` \| None` | 如果回應遇到錯誤，則為錯誤類型                                           |
| `usage`              | `dict[str, Any] \| None`                                     | 每消息令牌使用情況（與 [`ResultMessage.usage`](#resultmessage) 相同的鍵） |
| `message_id`         | `str \| None`                                                | API 消息 ID。來自一個轉的多個消息共享相同的 ID                              |

<h3 id="assistantmessageerror">
  `AssistantMessageError`
</h3>

助手消息的可能錯誤類型。

```python theme={null}
AssistantMessageError = Literal[
    "authentication_failed",
    "billing_error",
    "rate_limit",
    "invalid_request",
    "server_error",
    "max_output_tokens",
    "unknown",
]
```

<h3 id="systemmessage">
  `SystemMessage`
</h3>

具有中繼資料的系統消息。

```python theme={null}
@dataclass
class SystemMessage:
    subtype: str
    data: dict[str, Any]
```

<h3 id="resultmessage">
  `ResultMessage`
</h3>

具有成本和使用情況資訊的最終結果消息。

```python theme={null}
@dataclass
class ResultMessage:
    subtype: str
    duration_ms: int
    duration_api_ms: int
    is_error: bool
    num_turns: int
    session_id: str
    stop_reason: str | None = None
    total_cost_usd: float | None = None
    usage: dict[str, Any] | None = None
    result: str | None = None
    structured_output: Any = None
    model_usage: dict[str, Any] | None = None
    permission_denials: list[Any] | None = None
    deferred_tool_use: DeferredToolUse | None = None
    errors: list[str] | None = None
    api_error_status: int | None = None
    uuid: str | None = None
```

`subtype` 欄位決定了其他哪些欄位會被填入。它是 `"success"`、`"error_during_execution"`、`"error_max_turns"`、`"error_max_budget_usd"` 或 `"error_max_structured_output_retries"` 之一。Python dataclass 將所有變體扁平化為一個形狀，因此不適用於返回的 subtype 的欄位為 `None`。

當對話在錯誤時結束時，多個欄位會帶有診斷詳細資訊：

* `is_error`：當對話以錯誤狀態結束時為 `True`。在 `error_*` subtypes 上始終為 `True`。在 `subtype="success"` 上，當最終模型請求失敗時為 `True`，表示代理迴圈已完成但最後一個 API 呼叫返回了錯誤。
* `api_error_status`：終止 API 錯誤的 HTTP 狀態碼。當轉在沒有錯誤的情況下結束時為 `None`。僅在 `subtype="success"` 上填入。
* `result`：在 `subtype="success"` 上為最終助手消息的文字，或在 `error_*` subtypes 上為 `None`。當 `subtype="success"` 且 `is_error=True` 時，如果可用，此欄位會保存 API 錯誤字串，但可能為空，因此請檢查 `api_error_status` 和前面的 `AssistantMessage` 內容以獲取詳細資訊。
* `errors`：迴圈級別的錯誤字串，例如最大轉數消息。僅在 `error_*` subtypes 上填入。

`usage` 字典在出現時包含以下鍵：

| 鍵                             | 類型    | 描述                                                                                                                  |
| ----------------------------- | ----- | ------------------------------------------------------------------------------------------------------------------- |
| `input_tokens`                | `int` | 頂層代理迴圈消耗的輸入令牌。[子代理令牌不包括在內](/docs/zh-TW/agent-sdk/cost-tracking#get-the-total-cost-of-a-query)；使用 `model_usage` 進行整個樹的計算。 |
| `output_tokens`               | `int` | 頂層代理迴圈產生的輸出令牌。子代理令牌不包括在內。                                                                                           |
| `cache_creation_input_tokens` | `int` | 用於建立新快取項目的令牌。                                                                                                       |
| `cache_read_input_tokens`     | `int` | 從現有快取項目讀取的令牌。                                                                                                       |

`model_usage` 字典將模型名稱對應到每個模型的使用情況。內部字典鍵使用 camelCase，因為該值從基礎 CLI 程序未修改地傳遞，符合 TypeScript [`ModelUsage`](/docs/zh-TW/agent-sdk/typescript#modelusage) 類型：

| 鍵                          | 類型      | 描述                                                                                  |
| -------------------------- | ------- | ----------------------------------------------------------------------------------- |
| `inputTokens`              | `int`   | 此模型的輸入令牌。                                                                           |
| `outputTokens`             | `int`   | 此模型的輸出令牌。                                                                           |
| `cacheReadInputTokens`     | `int`   | 此模型的快取讀取令牌。                                                                         |
| `cacheCreationInputTokens` | `int`   | 此模型的快取建立令牌。                                                                         |
| `webSearchRequests`        | `int`   | 此模型進行的網路搜尋請求。                                                                       |
| `costUSD`                  | `float` | 此模型的估計成本（以 USD 為單位），在客戶端計算。見 [追蹤成本和使用情況](/docs/zh-TW/agent-sdk/cost-tracking) 以了解計費注意事項。 |
| `contextWindow`            | `int`   | 此模型的上下文視窗大小。                                                                        |
| `maxOutputTokens`          | `int`   | 此模型的最大輸出令牌限制。                                                                       |

<h3 id="streamevent">
  `StreamEvent`
</h3>

用於在串流期間進行部分消息更新的串流事件。僅在 `ClaudeAgentOptions` 中 `include_partial_messages=True` 時接收。透過 `from claude_agent_sdk.types import StreamEvent` 匯入。

```python theme={null}
@dataclass
class StreamEvent:
    uuid: str
    session_id: str
    event: dict[str, Any]  # The raw Claude API stream event
    parent_tool_use_id: str | None = None
```

| 欄位                   | 類型               | 描述                                                                                        |
| :------------------- | :--------------- | :---------------------------------------------------------------------------------------- |
| `uuid`               | `str`            | 此事件的唯一識別碼                                                                                 |
| `session_id`         | `str`            | session 識別碼                                                                               |
| `event`              | `dict[str, Any]` | 原始 Claude API 串流事件資料                                                                      |
| `parent_tool_use_id` | `str \| None`    | 始終為 `None`。串流事件僅針對主 session 發出。如需子代理歸屬，請使用完整消息，例如 [`AssistantMessage`](#assistantmessage) |

<h3 id="ratelimitevent">
  `RateLimitEvent`
</h3>

當速率限制狀態變更時發出（例如，從 `"allowed"` 到 `"allowed_warning"`）。使用此來在使用者達到硬限制之前警告他們，或在狀態為 `"rejected"` 時退避。

```python theme={null}
@dataclass
class RateLimitEvent:
    rate_limit_info: RateLimitInfo
    uuid: str
    session_id: str
```

| 欄位                | 類型                                | 描述          |
| :---------------- | :-------------------------------- | :---------- |
| `rate_limit_info` | [`RateLimitInfo`](#ratelimitinfo) | 目前速率限制狀態    |
| `uuid`            | `str`                             | 唯一事件識別碼     |
| `session_id`      | `str`                             | session 識別碼 |

<h3 id="ratelimitinfo">
  `RateLimitInfo`
</h3>

由 [`RateLimitEvent`](#ratelimitevent) 攜帶的速率限制狀態。

```python theme={null}
RateLimitStatus = Literal["allowed", "allowed_warning", "rejected"]
RateLimitType = Literal[
    "five_hour", "seven_day", "seven_day_opus", "seven_day_sonnet", "overage"
]


@dataclass
class RateLimitInfo:
    status: RateLimitStatus
    resets_at: int | None = None
    rate_limit_type: RateLimitType | None = None
    utilization: float | None = None
    overage_status: RateLimitStatus | None = None
    overage_resets_at: int | None = None
    overage_disabled_reason: str | None = None
    raw: dict[str, Any] = field(default_factory=dict)
```

| 欄位                        | 類型                        | 描述                                                  |
| :------------------------ | :------------------------ | :-------------------------------------------------- |
| `status`                  | `RateLimitStatus`         | 目前狀態。`"allowed_warning"` 表示接近限制；`"rejected"` 表示達到限制 |
| `resets_at`               | `int \| None`             | 速率限制視窗重設時的 Unix 時間戳                                 |
| `rate_limit_type`         | `RateLimitType \| None`   | 適用的速率限制視窗                                           |
| `utilization`             | `float \| None`           | 消耗的速率限制分數（0.0 到 1.0）                                |
| `overage_status`          | `RateLimitStatus \| None` | 按使用量付費超額使用的狀態（如適用）                                  |
| `overage_resets_at`       | `int \| None`             | 超額視窗重設時的 Unix 時間戳                                   |
| `overage_disabled_reason` | `str \| None`             | 如果狀態為 `"rejected"`，為什麼超額不可用                         |
| `raw`                     | `dict[str, Any]`          | 來自 CLI 的完整原始字典，包括上面未建模的欄位                           |

<h3 id="taskstartedmessage">
  `TaskStartedMessage`
</h3>

在背景任務啟動時發出。背景任務是在主轉之外追蹤的任何內容：背景 Bash 命令、[Monitor](#monitor) 監視、透過 Agent tool 生成的子代理或遠端代理。`task_type` 欄位告訴您是哪一個。此命名與 `Task` 到 `Agent` tool 重新命名無關。

```python theme={null}
@dataclass
class TaskStartedMessage(SystemMessage):
    task_id: str
    description: str
    uuid: str
    session_id: str
    tool_use_id: str | None = None
    task_type: str | None = None
```

| 欄位            | 類型            | 描述                                                                               |
| :------------ | :------------ | :------------------------------------------------------------------------------- |
| `task_id`     | `str`         | 任務的唯一識別碼                                                                         |
| `description` | `str`         | 任務的描述                                                                            |
| `uuid`        | `str`         | 唯一消息識別碼                                                                          |
| `session_id`  | `str`         | session 識別碼                                                                      |
| `tool_use_id` | `str \| None` | 相關聯的 tool 使用 ID                                                                  |
| `task_type`   | `str \| None` | 背景任務的類型：`"local_bash"` 用於背景 Bash 和 Monitor 監視，`"local_agent"` 或 `"remote_agent"` |

<h3 id="taskusage">
  `TaskUsage`
</h3>

背景任務的令牌和計時資料。

```python theme={null}
class TaskUsage(TypedDict):
    total_tokens: int
    tool_uses: int
    duration_ms: int
```

<h3 id="taskprogressmessage">
  `TaskProgressMessage`
</h3>

定期為執行中的背景任務發出進度更新。

```python theme={null}
@dataclass
class TaskProgressMessage(SystemMessage):
    task_id: str
    description: str
    usage: TaskUsage
    uuid: str
    session_id: str
    tool_use_id: str | None = None
    last_tool_name: str | None = None
```

| 欄位               | 類型            | 描述              |
| :--------------- | :------------ | :-------------- |
| `task_id`        | `str`         | 任務的唯一識別碼        |
| `description`    | `str`         | 目前狀態描述          |
| `usage`          | `TaskUsage`   | 此任務迄今為止的令牌使用情況  |
| `uuid`           | `str`         | 唯一消息識別碼         |
| `session_id`     | `str`         | session 識別碼     |
| `tool_use_id`    | `str \| None` | 相關聯的 tool 使用 ID |
| `last_tool_name` | `str \| None` | 任務最後使用的 tool 名稱 |

<h3 id="tasknotificationmessage">
  `TaskNotificationMessage`
</h3>

在背景任務完成、失敗或停止時發出。背景任務包括 `run_in_background` Bash 命令、Monitor 監視和背景子代理。

```python theme={null}
@dataclass
class TaskNotificationMessage(SystemMessage):
    task_id: str
    status: TaskNotificationStatus  # "completed" | "failed" | "stopped"
    output_file: str
    summary: str
    uuid: str
    session_id: str
    tool_use_id: str | None = None
    usage: TaskUsage | None = None
```

| 欄位            | 類型                       | 描述                                        |
| :------------ | :----------------------- | :---------------------------------------- |
| `task_id`     | `str`                    | 任務的唯一識別碼                                  |
| `status`      | `TaskNotificationStatus` | `"completed"`、`"failed"` 或 `"stopped"` 之一 |
| `output_file` | `str`                    | 任務輸出檔案的路徑                                 |
| `summary`     | `str`                    | 任務結果的摘要                                   |
| `uuid`        | `str`                    | 唯一消息識別碼                                   |
| `session_id`  | `str`                    | session 識別碼                               |
| `tool_use_id` | `str \| None`            | 相關聯的 tool 使用 ID                           |
| `usage`       | `TaskUsage \| None`      | 任務的最終令牌使用情況                               |

<h2 id="content-block-types">
  內容區塊類型
</h2>

<h3 id="contentblock">
  `ContentBlock`
</h3>

所有內容區塊的聯合類型。

```python theme={null}
ContentBlock = TextBlock | ThinkingBlock | ToolUseBlock | ToolResultBlock
```

<h3 id="textblock">
  `TextBlock`
</h3>

文字內容區塊。

```python theme={null}
@dataclass
class TextBlock:
    text: str
```

<h3 id="thinkingblock">
  `ThinkingBlock`
</h3>

思考內容區塊（用於具有思考能力的模型）。

```python theme={null}
@dataclass
class ThinkingBlock:
    thinking: str
    signature: str
```

<h3 id="tooluseblock">
  `ToolUseBlock`
</h3>

工具使用請求區塊。

```python theme={null}
@dataclass
class ToolUseBlock:
    id: str
    name: str
    input: dict[str, Any]
```

<h3 id="toolresultblock">
  `ToolResultBlock`
</h3>

工具執行結果區塊。

```python theme={null}
@dataclass
class ToolResultBlock:
    tool_use_id: str
    content: str | list[dict[str, Any]] | None = None
    is_error: bool | None = None
```

<h2 id="error-types">
  錯誤類型
</h2>

<h3 id="claudesdkerror">
  `ClaudeSDKError`
</h3>

所有 SDK 錯誤的基礎例外類別。

```python theme={null}
class ClaudeSDKError(Exception):
    """Base error for Claude SDK."""
```

<h3 id="clinotfounderror">
  `CLINotFoundError`
</h3>

當 Claude Code CLI 未安裝或找不到時引發。

```python theme={null}
class CLINotFoundError(CLIConnectionError):
    def __init__(
        self, message: str = "Claude Code not found", cli_path: str | None = None
    ):
        """
        Args:
            message: Error message (default: "Claude Code not found")
            cli_path: Optional path to the CLI that was not found
        """
```

<h3 id="cliconnectionerror">
  `CLIConnectionError`
</h3>

當連接到 Claude Code 失敗時引發。

```python theme={null}
class CLIConnectionError(ClaudeSDKError):
    """Failed to connect to Claude Code."""
```

<h3 id="processerror">
  `ProcessError`
</h3>

當 Claude Code 程序失敗時引發。

```python theme={null}
class ProcessError(ClaudeSDKError):
    def __init__(
        self, message: str, exit_code: int | None = None, stderr: str | None = None
    ):
        self.exit_code = exit_code
        self.stderr = stderr
```

<h3 id="clijsondecodeerror">
  `CLIJSONDecodeError`
</h3>

當 JSON 解析失敗時引發。

```python theme={null}
class CLIJSONDecodeError(ClaudeSDKError):
    def __init__(self, line: str, original_error: Exception):
        """
        Args:
            line: The line that failed to parse
            original_error: The original JSON decode exception
        """
        self.line = line
        self.original_error = original_error
```

<h2 id="hook-types">
  Hook 類型
</h2>

如需使用 hooks 的綜合指南，包括範例和常見模式，見 [Hooks 指南](/docs/zh-TW/agent-sdk/hooks)。

<h3 id="hookevent">
  `HookEvent`
</h3>

支援的 hook 事件類型。

```python theme={null}
HookEvent = Literal[
    "PreToolUse",  # Called before tool execution
    "PostToolUse",  # Called after tool execution
    "PostToolUseFailure",  # Called when a tool execution fails
    "UserPromptSubmit",  # Called when user submits a prompt
    "Stop",  # Called when stopping execution
    "SubagentStop",  # Called when a subagent stops
    "PreCompact",  # Called before message compaction
    "Notification",  # Called for notification events
    "SubagentStart",  # Called when a subagent starts
    "PermissionRequest",  # Called when a permission decision is needed
]
```

<Note>
  TypeScript SDK 支援 Python 中尚未提供的其他 hook 事件：`SessionStart`、`SessionEnd`、`Setup`、`TeammateIdle`、`TaskCompleted`、`ConfigChange`、`WorktreeCreate`、`WorktreeRemove`、`PostToolBatch` 和 `MessageDisplay`。
</Note>

<h3 id="hookcallback">
  `HookCallback`
</h3>

hook 回呼函數的類型定義。

```python theme={null}
HookCallback = Callable[[HookInput, str | None, HookContext], Awaitable[HookJSONOutput]]
```

參數：

* `input`：強類型 hook 輸入，具有基於 `hook_event_name` 的判別聯合（見 [`HookInput`](#hookinput)）
* `tool_use_id`：可選 tool 使用識別碼（用於 tool 相關 hooks）
* `context`：具有其他資訊的 hook 上下文

返回可能包含以下內容的 [`HookJSONOutput`](#hookjsonoutput)：

* `decision`：`"block"` 以阻止動作
* `systemMessage`：警告消息，顯示給使用者
* `hookSpecificOutput`：hook 特定輸出資料

<h3 id="hookcontext">
  `HookContext`
</h3>

傳遞給 hook 回呼的上下文資訊。

```python theme={null}
class HookContext(TypedDict):
    signal: Any | None  # Future: abort signal support
```

<h3 id="hookmatcher">
  `HookMatcher`
</h3>

用於將 hooks 符合到特定事件或 tools 的配置。

```python theme={null}
@dataclass
class HookMatcher:
    matcher: str | None = (
        None  # Tool name or pattern to match (e.g., "Bash", "Write|Edit")
    )
    hooks: list[HookCallback] = field(
        default_factory=list
    )  # List of callbacks to execute
    timeout: float | None = (
        None  # Timeout in seconds for all hooks in this matcher (default: 60)
    )
```

<h3 id="hookinput">
  `HookInput`
</h3>

所有 hook 輸入類型的聯合類型。實際類型取決於 `hook_event_name` 欄位。

```python theme={null}
HookInput = (
    PreToolUseHookInput
    | PostToolUseHookInput
    | PostToolUseFailureHookInput
    | UserPromptSubmitHookInput
    | StopHookInput
    | SubagentStopHookInput
    | PreCompactHookInput
    | NotificationHookInput
    | SubagentStartHookInput
    | PermissionRequestHookInput
)
```

<h3 id="basehookinput">
  `BaseHookInput`
</h3>

所有 hook 輸入類型中存在的基礎欄位。

```python theme={null}
class BaseHookInput(TypedDict):
    session_id: str
    transcript_path: str
    cwd: str
    permission_mode: NotRequired[str]
```

| 欄位                | 類型        | 描述              |
| :---------------- | :-------- | :-------------- |
| `session_id`      | `str`     | 目前 session 識別碼  |
| `transcript_path` | `str`     | session 記錄檔案的路徑 |
| `cwd`             | `str`     | 目前工作目錄          |
| `permission_mode` | `str`（可選） | 目前權限模式          |

<h3 id="pretoolusehookinput">
  `PreToolUseHookInput`
</h3>

`PreToolUse` hook 事件的輸入資料。

```python theme={null}
class PreToolUseHookInput(BaseHookInput):
    hook_event_name: Literal["PreToolUse"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_use_id: str
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| 欄位                | 類型                      | 描述                       |
| :---------------- | :---------------------- | :----------------------- |
| `hook_event_name` | `Literal["PreToolUse"]` | 始終為 "PreToolUse"         |
| `tool_name`       | `str`                   | 即將執行的 tool 名稱            |
| `tool_input`      | `dict[str, Any]`        | tool 的輸入參數               |
| `tool_use_id`     | `str`                   | 此 tool 使用的唯一識別碼          |
| `agent_id`        | `str`（可選）               | 子代理識別碼，當 hook 在子代理內觸發時出現 |
| `agent_type`      | `str`（可選）               | 子代理類型，當 hook 在子代理內觸發時出現  |

<h3 id="posttoolusehookinput">
  `PostToolUseHookInput`
</h3>

`PostToolUse` hook 事件的輸入資料。

```python theme={null}
class PostToolUseHookInput(BaseHookInput):
    hook_event_name: Literal["PostToolUse"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_response: Any
    tool_use_id: str
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| 欄位                | 類型                       | 描述                       |
| :---------------- | :----------------------- | :----------------------- |
| `hook_event_name` | `Literal["PostToolUse"]` | 始終為 "PostToolUse"        |
| `tool_name`       | `str`                    | 已執行的 tool 名稱             |
| `tool_input`      | `dict[str, Any]`         | 使用的輸入參數                  |
| `tool_response`   | `Any`                    | tool 執行的回應               |
| `tool_use_id`     | `str`                    | 此 tool 使用的唯一識別碼          |
| `agent_id`        | `str`（可選）                | 子代理識別碼，當 hook 在子代理內觸發時出現 |
| `agent_type`      | `str`（可選）                | 子代理類型，當 hook 在子代理內觸發時出現  |

<h3 id="posttoolusefailurehookinput">
  `PostToolUseFailureHookInput`
</h3>

`PostToolUseFailure` hook 事件的輸入資料。在 tool 執行失敗時呼叫。

```python theme={null}
class PostToolUseFailureHookInput(BaseHookInput):
    hook_event_name: Literal["PostToolUseFailure"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_use_id: str
    error: str
    is_interrupt: NotRequired[bool]
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| 欄位                | 類型                              | 描述                       |
| :---------------- | :------------------------------ | :----------------------- |
| `hook_event_name` | `Literal["PostToolUseFailure"]` | 始終為 "PostToolUseFailure" |
| `tool_name`       | `str`                           | 失敗的 tool 名稱              |
| `tool_input`      | `dict[str, Any]`                | 使用的輸入參數                  |
| `tool_use_id`     | `str`                           | 此 tool 使用的唯一識別碼          |
| `error`           | `str`                           | 失敗執行的錯誤消息                |
| `is_interrupt`    | `bool`（可選）                      | 失敗是否由中斷引起                |
| `agent_id`        | `str`（可選）                       | 子代理識別碼，當 hook 在子代理內觸發時出現 |
| `agent_type`      | `str`（可選）                       | 子代理類型，當 hook 在子代理內觸發時出現  |

<h3 id="userpromptsubmithookinput">
  `UserPromptSubmitHookInput`
</h3>

`UserPromptSubmit` hook 事件的輸入資料。

```python theme={null}
class UserPromptSubmitHookInput(BaseHookInput):
    hook_event_name: Literal["UserPromptSubmit"]
    prompt: str
```

| 欄位                | 類型                            | 描述                     |
| :---------------- | :---------------------------- | :--------------------- |
| `hook_event_name` | `Literal["UserPromptSubmit"]` | 始終為 "UserPromptSubmit" |
| `prompt`          | `str`                         | 使用者提交的提示               |

<h3 id="stophookinput">
  `StopHookInput`
</h3>

`Stop` hook 事件的輸入資料。

```python theme={null}
class StopHookInput(BaseHookInput):
    hook_event_name: Literal["Stop"]
    stop_hook_active: bool
```

| 欄位                 | 類型                | 描述             |
| :----------------- | :---------------- | :------------- |
| `hook_event_name`  | `Literal["Stop"]` | 始終為 "Stop"     |
| `stop_hook_active` | `bool`            | stop hook 是否活躍 |

<h3 id="subagentstophookinput">
  `SubagentStopHookInput`
</h3>

`SubagentStop` hook 事件的輸入資料。

```python theme={null}
class SubagentStopHookInput(BaseHookInput):
    hook_event_name: Literal["SubagentStop"]
    stop_hook_active: bool
    agent_id: str
    agent_transcript_path: str
    agent_type: str
```

| 欄位                      | 類型                        | 描述                 |
| :---------------------- | :------------------------ | :----------------- |
| `hook_event_name`       | `Literal["SubagentStop"]` | 始終為 "SubagentStop" |
| `stop_hook_active`      | `bool`                    | stop hook 是否活躍     |
| `agent_id`              | `str`                     | 子代理的唯一識別碼          |
| `agent_transcript_path` | `str`                     | 子代理記錄檔案的路徑         |
| `agent_type`            | `str`                     | 子代理的類型             |

<h3 id="precompacthookinput">
  `PreCompactHookInput`
</h3>

`PreCompact` hook 事件的輸入資料。

```python theme={null}
class PreCompactHookInput(BaseHookInput):
    hook_event_name: Literal["PreCompact"]
    trigger: Literal["manual", "auto"]
    custom_instructions: str | None
```

| 欄位                    | 類型                          | 描述               |
| :-------------------- | :-------------------------- | :--------------- |
| `hook_event_name`     | `Literal["PreCompact"]`     | 始終為 "PreCompact" |
| `trigger`             | `Literal["manual", "auto"]` | 觸發壓縮的原因          |
| `custom_instructions` | `str \| None`               | 壓縮的自訂指示          |

<h3 id="notificationhookinput">
  `NotificationHookInput`
</h3>

`Notification` hook 事件的輸入資料。

```python theme={null}
class NotificationHookInput(BaseHookInput):
    hook_event_name: Literal["Notification"]
    message: str
    title: NotRequired[str]
    notification_type: str
```

| 欄位                  | 類型                        | 描述                 |
| :------------------ | :------------------------ | :----------------- |
| `hook_event_name`   | `Literal["Notification"]` | 始終為 "Notification" |
| `message`           | `str`                     | 通知消息內容             |
| `title`             | `str`（可選）                 | 通知標題               |
| `notification_type` | `str`                     | 通知類型               |

<h3 id="subagentstarthookinput">
  `SubagentStartHookInput`
</h3>

`SubagentStart` hook 事件的輸入資料。

```python theme={null}
class SubagentStartHookInput(BaseHookInput):
    hook_event_name: Literal["SubagentStart"]
    agent_id: str
    agent_type: str
```

| 欄位                | 類型                         | 描述                  |
| :---------------- | :------------------------- | :------------------ |
| `hook_event_name` | `Literal["SubagentStart"]` | 始終為 "SubagentStart" |
| `agent_id`        | `str`                      | 子代理的唯一識別碼           |
| `agent_type`      | `str`                      | 子代理的類型              |

<h3 id="permissionrequesthookinput">
  `PermissionRequestHookInput`
</h3>

`PermissionRequest` hook 事件的輸入資料。允許 hooks 以程式設計方式處理權限決策。

```python theme={null}
class PermissionRequestHookInput(BaseHookInput):
    hook_event_name: Literal["PermissionRequest"]
    tool_name: str
    tool_input: dict[str, Any]
    permission_suggestions: NotRequired[list[Any]]
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| 欄位                       | 類型                             | 描述                       |
| :----------------------- | :----------------------------- | :----------------------- |
| `hook_event_name`        | `Literal["PermissionRequest"]` | 始終為 "PermissionRequest"  |
| `tool_name`              | `str`                          | 請求權限的 tool 名稱            |
| `tool_input`             | `dict[str, Any]`               | tool 的輸入參數               |
| `permission_suggestions` | `list[Any]`（可選）                | 來自 CLI 的建議權限更新           |
| `agent_id`               | `str`（可選）                      | 子代理識別碼，當 hook 在子代理內觸發時出現 |
| `agent_type`             | `str`（可選）                      | 子代理類型，當 hook 在子代理內觸發時出現  |

<h3 id="hookjsonoutput">
  `HookJSONOutput`
</h3>

hook 回呼返回值的聯合類型。

```python theme={null}
HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput
```

<h4 id="synchookjsonoutput">
  `SyncHookJSONOutput`
</h4>

具有控制和決策欄位的同步 hook 輸出。

```python theme={null}
class SyncHookJSONOutput(TypedDict):
    # Control fields
    continue_: NotRequired[bool]  # Whether to proceed (default: True)
    suppressOutput: NotRequired[bool]  # Hide stdout from transcript
    stopReason: NotRequired[str]  # Message when continue is False

    # Decision fields
    decision: NotRequired[Literal["block"]]
    systemMessage: NotRequired[str]  # Warning message for user
    reason: NotRequired[str]  # Feedback for Claude

    # Hook-specific output
    hookSpecificOutput: NotRequired[HookSpecificOutput]
```

<Note>
  在 Python 程式碼中使用 `continue_`（帶下劃線）。發送到 CLI 時會自動轉換為 `continue`。
</Note>

<h4 id="hookspecificoutput">
  `HookSpecificOutput`
</h4>

包含 hook 事件名稱和事件特定欄位的 `TypedDict`。形狀取決於 `hookEventName` 值。如需每個 hook 事件的可用欄位的完整詳情，見 [使用 hooks 控制執行](/docs/zh-TW/agent-sdk/hooks#outputs)。

事件特定輸出類型的判別聯合。`hookEventName` 欄位決定哪些欄位有效。

```python theme={null}
class PreToolUseHookSpecificOutput(TypedDict):
    hookEventName: Literal["PreToolUse"]
    permissionDecision: NotRequired[Literal["allow", "deny", "ask", "defer"]]
    permissionDecisionReason: NotRequired[str]
    updatedInput: NotRequired[dict[str, Any]]
    additionalContext: NotRequired[str]


class PostToolUseHookSpecificOutput(TypedDict):
    hookEventName: Literal["PostToolUse"]
    additionalContext: NotRequired[str]
    updatedToolOutput: NotRequired[Any]
    updatedMCPToolOutput: NotRequired[Any]  # Deprecated: use updatedToolOutput, which works for all tools


class PostToolUseFailureHookSpecificOutput(TypedDict):
    hookEventName: Literal["PostToolUseFailure"]
    additionalContext: NotRequired[str]


class UserPromptSubmitHookSpecificOutput(TypedDict):
    hookEventName: Literal["UserPromptSubmit"]
    additionalContext: NotRequired[str]


class NotificationHookSpecificOutput(TypedDict):
    hookEventName: Literal["Notification"]
    additionalContext: NotRequired[str]


class SubagentStartHookSpecificOutput(TypedDict):
    hookEventName: Literal["SubagentStart"]
    additionalContext: NotRequired[str]


class PermissionRequestHookSpecificOutput(TypedDict):
    hookEventName: Literal["PermissionRequest"]
    decision: dict[str, Any]


HookSpecificOutput = (
    PreToolUseHookSpecificOutput
    | PostToolUseHookSpecificOutput
    | PostToolUseFailureHookSpecificOutput
    | UserPromptSubmitHookSpecificOutput
    | NotificationHookSpecificOutput
    | SubagentStartHookSpecificOutput
    | PermissionRequestHookSpecificOutput
)
```

<h4 id="asynchookjsonoutput">
  `AsyncHookJSONOutput`
</h4>

延遲 hook 執行的非同步 hook 輸出。

```python theme={null}
class AsyncHookJSONOutput(TypedDict):
    async_: Literal[True]  # Set to True to defer execution
    asyncTimeout: NotRequired[int]  # Timeout in milliseconds
```

<Note>
  在 Python 程式碼中使用 `async_`（帶下劃線）。發送到 CLI 時會自動轉換為 `async`。
</Note>

<h3 id="hook-usage-example">
  Hook 使用範例
</h3>

此範例註冊兩個 hooks：一個阻止危險的 bash 命令（如 `rm -rf /`），另一個記錄所有 tool 使用情況以進行審計。安全 hook 僅在 Bash 命令上執行（透過 `matcher`），而記錄 hook 在所有 tools 上執行。

```python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher, HookContext
from typing import Any


async def validate_bash_command(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Validate and potentially block dangerous bash commands."""
    if input_data["tool_name"] == "Bash":
        command = input_data["tool_input"].get("command", "")
        if "rm -rf /" in command:
            return {
                "hookSpecificOutput": {
                    "hookEventName": "PreToolUse",
                    "permissionDecision": "deny",
                    "permissionDecisionReason": "Dangerous command blocked",
                }
            }
    return {}


async def log_tool_use(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Log all tool usage for auditing."""
    print(f"Tool used: {input_data.get('tool_name')}")
    return {}


options = ClaudeAgentOptions(
    hooks={
        "PreToolUse": [
            HookMatcher(
                matcher="Bash", hooks=[validate_bash_command], timeout=120
            ),  # 2 min for validation
            HookMatcher(
                hooks=[log_tool_use]
            ),  # Applies to all tools (default 60s timeout)
        ],
        "PostToolUse": [HookMatcher(hooks=[log_tool_use])],
    }
)

async for message in query(prompt="Analyze this codebase", options=options):
    print(message)
```

<h2 id="tool-input/output-types">
  Tool 輸入/輸出類型
</h2>

所有內建 Claude Code tools 的輸入/輸出架構文件。雖然 Python SDK 不將這些匯出為類型，但它們代表消息中 tool 輸入和輸出的結構。

<h3 id="agent">
  Agent
</h3>

**Tool 名稱：** `Agent`（先前為 `Task`，仍接受作為別名）

**輸入：**

```python theme={null}
{
    "description": str,  # 任務的簡短描述（3-5 個單詞）
    "prompt": str,  # agent 要執行的任務
    "subagent_type": str,  # 要使用的專門 agent 類型
}
```

**輸出：**

```python theme={null}
{
    "result": str,  # 來自 subagent 的最終結果
    "usage": dict | None,  # Token 使用統計
    "total_cost_usd": float | None,  # 估計的美元總成本
    "duration_ms": int | None,  # 執行持續時間（毫秒）
}
```

<h3 id="askuserquestion">
  AskUserQuestion
</h3>

**Tool 名稱：** `AskUserQuestion`

在執行期間詢問使用者澄清問題。見 [處理批准和使用者輸入](/docs/zh-TW/agent-sdk/user-input#handle-clarifying-questions) 以了解使用詳情。

**輸入：**

```python theme={null}
{
    "questions": [  # 要詢問使用者的問題（1-4 個問題）
        {
            "question": str,  # 要詢問使用者的完整問題
            "header": str,  # 顯示為晶片/標籤的非常簡短標籤（最多 12 個字元）
            "options": [  # 可用的選擇（2-4 個選項）
                {
                    "label": str,  # 此選項的顯示文字（1-5 個單詞）
                    "description": str,  # 此選項含義的說明
                }
            ],
            "multiSelect": bool,  # 設定為 true 以允許多個選擇
        }
    ],
    "answers": dict[str, str | list[str]] | None,
    # 由權限系統填入的使用者答案。多選
    # 答案可能是標籤列表或逗號連接的字串
}
```

**輸出：**

```python theme={null}
{
    "questions": [  # 被詢問的問題
        {
            "question": str,
            "header": str,
            "options": [{"label": str, "description": str}],
            "multiSelect": bool,
        }
    ],
    "answers": dict[str, str],  # 將問題文字對應到答案字串
    # 多選答案以逗號分隔
}
```

<h3 id="bash">
  Bash
</h3>

**Tool 名稱：** `Bash`

**輸入：**

```python theme={null}
{
    "command": str,  # 要執行的命令
    "timeout": int | None,  # 可選的逾時時間（毫秒）（最多 600000；更高的值會被限制為最大值）
    "description": str | None,  # 清晰、簡潔的描述（5-10 個單詞）
    "run_in_background": bool | None,  # 設定為 true 以在背景執行
}
```

**輸出：**

```python theme={null}
{
    "output": str,  # 合併的 stdout 和 stderr 輸出
    "exitCode": int,  # 命令的結束代碼
    "killed": bool | None,  # 命令是否因逾時而被終止
    "shellId": str | None,  # 背景程序的 Shell ID
}
```

<h3 id="monitor">
  Monitor
</h3>

**Tool 名稱：** `Monitor`

執行背景來源並將每個事件傳遞給 Claude，以便它可以做出反應而無需輪詢：`command` 執行指令碼並每個 stdout 行發出一個事件，`ws` 開啟 WebSocket 並每個文字框架發出一個事件。請提供 `command` 或 `ws` 中的恰好一個。

當 Monitor 執行命令時，它遵循與 Bash 相同的權限規則；WebSocket 監視會單獨提示批准。`ws` 來源需要 Claude Code v2.1.195 或更新版本。見 [Monitor tool 參考](/docs/zh-TW/tools-reference#monitor-tool) 以了解行為和提供者可用性。

**輸入：**

```python theme={null}
{
    "command": str | None,  # Shell 指令碼；每個 stdout 行是一個事件，結束會停止監視
    "ws": dict | None,  # WebSocket 來源：{"url": str, "protocols": list[str] | None}；每個文字框架是一個事件
    "description": str,  # 在通知中顯示的簡短描述
    "timeout_ms": int | None,  # 在此期限後終止（預設 300000，最多 3600000）
    "persistent": bool | None,  # 在工作階段的生命週期內執行；使用 TaskStop 停止
}
```

**輸出：**

```python theme={null}
{
    "taskId": str,  # 背景監視任務的 ID
    "timeoutMs": int,  # 逾時期限（毫秒）（持續時為 0）
    "persistent": bool | None,  # 當執行到 TaskStop 或工作階段結束時為 True
}
```

<h3 id="edit">
  Edit
</h3>

**Tool 名稱：** `Edit`

**輸入：**

```python theme={null}
{
    "file_path": str,  # 要修改的檔案的絕對路徑
    "old_string": str,  # 要替換的文字
    "new_string": str,  # 用來替換的文字
    "replace_all": bool | None,  # 替換所有出現次數（預設 False）
}
```

**輸出：**

```python theme={null}
{
    "message": str,  # 確認訊息
    "replacements": int,  # 進行的替換次數
    "file_path": str,  # 被編輯的檔案路徑
}
```

<h3 id="read">
  Read
</h3>

**Tool 名稱：** `Read`

**輸入：**

```python theme={null}
{
    "file_path": str,  # 要讀取的檔案的絕對路徑
    "offset": int | None,  # 開始讀取的行號
    "limit": int | None,  # 要讀取的行數
}
```

**輸出（文字檔案）：**

```python theme={null}
{
    "content": str,  # 包含行號的檔案內容
    "total_lines": int,  # 檔案中的總行數
    "lines_returned": int,  # 實際返回的行數
}
```

**輸出（影像）：**

```python theme={null}
{
    "image": str,  # Base64 編碼的影像資料
    "mime_type": str,  # 影像 MIME 類型
    "file_size": int,  # 檔案大小（位元組）
}
```

<h3 id="write">
  Write
</h3>

**Tool 名稱：** `Write`

**輸入：**

```python theme={null}
{
    "file_path": str,  # 要寫入的檔案的絕對路徑
    "content": str,  # 要寫入檔案的內容
}
```

**輸出：**

```python theme={null}
{
    "message": str,  # 成功訊息
    "bytes_written": int,  # 寫入的位元組數
    "file_path": str,  # 被寫入的檔案路徑
}
```

<h3 id="glob">
  Glob
</h3>

**Tool 名稱：** `Glob`

**輸入：**

```python theme={null}
{
    "pattern": str,  # 要與檔案匹配的 glob 模式
    "path": str | None,  # 要搜尋的目錄（預設為 cwd）
}
```

**輸出：**

```python theme={null}
{
    "matches": list[str],  # 匹配的檔案路徑陣列
    "count": int,  # 找到的匹配數
    "search_path": str,  # 使用的搜尋目錄
}
```

<h3 id="grep">
  Grep
</h3>

**Tool 名稱：** `Grep`

**輸入：**

```python theme={null}
{
    "pattern": str,  # 正規表達式模式
    "path": str | None,  # 要搜尋的檔案或目錄
    "glob": str | None,  # 用於篩選檔案的 glob 模式
    "type": str | None,  # 要搜尋的檔案類型
    "output_mode": str | None,  # "content"、"files_with_matches" 或 "count"
    "-i": bool | None,  # 不區分大小寫搜尋
    "-n": bool | None,  # 顯示行號
    "-B": int | None,  # 每個匹配前顯示的行數
    "-A": int | None,  # 每個匹配後顯示的行數
    "-C": int | None,  # 匹配前後顯示的行數
    "head_limit": int | None,  # 將輸出限制為前 N 行/項目
    "multiline": bool | None,  # 啟用多行模式
}
```

**輸出（content 模式）：**

```python theme={null}
{
    "matches": [
        {
            "file": str,
            "line_number": int | None,
            "line": str,
            "before_context": list[str] | None,
            "after_context": list[str] | None,
        }
    ],
    "total_matches": int,
}
```

**輸出（files\_with\_matches 模式）：**

```python theme={null}
{
    "files": list[str],  # 包含匹配的檔案
    "count": int,  # 包含匹配的檔案數
}
```

<h3 id="notebookedit">
  NotebookEdit
</h3>

**Tool 名稱：** `NotebookEdit`

**輸入：**

```python theme={null}
{
    "notebook_path": str,  # Jupyter notebook 的絕對路徑
    "cell_id": str | None,  # 要編輯的儲存格的 ID
    "new_source": str,  # 儲存格的新來源
    "cell_type": "code" | "markdown" | None,  # 儲存格的類型
    "edit_mode": "replace" | "insert" | "delete" | None,  # 編輯操作類型
}
```

**輸出：**

```python theme={null}
{
    "message": str,  # 成功訊息
    "edit_type": "replaced" | "inserted" | "deleted",  # 執行的編輯類型
    "cell_id": str | None,  # 受影響的儲存格 ID
    "total_cells": int,  # 編輯後 notebook 中的總儲存格數
}
```

<h3 id="webfetch">
  WebFetch
</h3>

**Tool 名稱：** `WebFetch`

**輸入：**

```python theme={null}
{
    "url": str,  # 要從中擷取內容的 URL
    "prompt": str,  # 在擷取的內容上執行的提示
}
```

**輸出：**

```python theme={null}
{
    "bytes": int,  # 擷取內容的大小（位元組）
    "code": int,  # HTTP 回應代碼
    "codeText": str,  # HTTP 回應代碼文字
    "result": str,  # 將提示應用於內容的處理結果
    "durationMs": int,  # 擷取和處理內容的時間（毫秒）
    "url": str,  # 被擷取的 URL
}
```

<h3 id="websearch">
  WebSearch
</h3>

**Tool 名稱：** `WebSearch`

**輸入：**

```python theme={null}
{
    "query": str,  # 要使用的搜尋查詢
    "allowed_domains": list[str] | None,  # 僅包含來自這些網域的結果
    "blocked_domains": list[str] | None,  # 永遠不包含來自這些網域的結果
}
```

**輸出：**

```python theme={null}
{
    "query": str,  # 搜尋查詢
    "results": list[str | {"tool_use_id": str, "content": list[{"title": str, "url": str}]}],
    "durationSeconds": float,  # 搜尋持續時間（秒）
}
```

<h3 id="todowrite">
  TodoWrite
</h3>

**Tool 名稱：** `TodoWrite`

<Note>
  自 Claude Code v2.1.142 起，`TodoWrite` 預設為停用。改用 `TaskCreate`、`TaskGet`、`TaskUpdate` 和 `TaskList`。見 [遷移到 Task tools](/docs/zh-TW/agent-sdk/todo-tracking#migrate-to-task-tools) 以更新您的監視程式碼，或設定 `CLAUDE_CODE_ENABLE_TASKS=0` 以還原為 `TodoWrite`。
</Note>

**輸入：**

```python theme={null}
{
    "todos": [
        {
            "content": str,  # 任務描述
            "status": "pending" | "in_progress" | "completed",  # 任務狀態
            "activeForm": str,  # 描述的主動形式
        }
    ]
}
```

**輸出：**

```python theme={null}
{
    "message": str,  # 成功訊息
    "stats": {"total": int, "pending": int, "in_progress": int, "completed": int},
}
```

<h3 id="taskcreate">
  TaskCreate
</h3>

**Tool 名稱：** `TaskCreate`

**輸入：**

```python theme={null}
{
    "subject": str,  # 簡短的任務標題
    "description": str,  # 詳細的任務內容
    "activeForm": str | None,  # 進行中時顯示的現在式標籤
    "metadata": dict | None,  # 任意呼叫者中繼資料
}
```

**輸出：**

```python theme={null}
{
    "task": {"id": str, "subject": str},  # 建立的任務及指派的 ID
}
```

<h3 id="taskupdate">
  TaskUpdate
</h3>

**Tool 名稱：** `TaskUpdate`

**輸入：**

```python theme={null}
{
    "taskId": str,  # 要修補的任務的 ID
    "status": Literal["pending", "in_progress", "completed", "deleted"] | None,
    "subject": str | None,
    "description": str | None,
    "activeForm": str | None,
    "addBlocks": list[str] | None,  # 此任務現在阻止的任務 ID
    "addBlockedBy": list[str] | None,  # 現在阻止此任務的任務 ID
    "owner": str | None,
    "metadata": dict | None,
}
```

**輸出：**

```python theme={null}
{
    "success": bool,
    "taskId": str,
    "updatedFields": list[str],  # 變更的欄位名稱
    "error": str | None,
    "statusChange": {"from": str, "to": str} | None,
}
```

<h3 id="taskget">
  TaskGet
</h3>

**Tool 名稱：** `TaskGet`

**輸入：**

```python theme={null}
{
    "taskId": str,  # 要讀取的任務的 ID
}
```

**輸出：**

```python theme={null}
{
    "task": {
        "id": str,
        "subject": str,
        "description": str,
        "status": Literal["pending", "in_progress", "completed"],
        "blocks": list[str],
        "blockedBy": list[str],
    } | None,  # 當找不到 ID 時為 None
}
```

<h3 id="tasklist">
  TaskList
</h3>

**Tool 名稱：** `TaskList`

**輸入：**

```python theme={null}
{}
```

**輸出：**

```python theme={null}
{
    "tasks": [
        {
            "id": str,
            "subject": str,
            "status": Literal["pending", "in_progress", "completed"],
            "owner": str | None,
            "blockedBy": list[str],
        }
    ],
}
```

<h3 id="bashoutput">
  BashOutput
</h3>

**Tool 名稱：** `BashOutput`

**輸入：**

```python theme={null}
{
    "bash_id": str,  # 背景 shell 的 ID
    "filter": str | None,  # 用於篩選輸出行的可選正規表達式
}
```

**輸出：**

```python theme={null}
{
    "output": str,  # 自上次檢查以來的新輸出
    "status": "running" | "completed" | "failed",  # 目前 shell 狀態
    "exitCode": int | None,  # 完成時的結束代碼
}
```

<h3 id="killbash">
  KillBash
</h3>

**Tool 名稱：** `KillBash`

**輸入：**

```python theme={null}
{
    "shell_id": str  # 要終止的背景 shell 的 ID
}
```

**輸出：**

```python theme={null}
{
    "message": str,  # 成功訊息
    "shell_id": str,  # 被終止的 shell 的 ID
}
```

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**Tool 名稱：** `ExitPlanMode`

**輸入：**

```python theme={null}
{
    "plan": str  # 使用者要執行以供批准的計畫
}
```

**輸出：**

```python theme={null}
{
    "message": str,  # 確認訊息
    "approved": bool | None,  # 使用者是否批准計畫
}
```

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**Tool 名稱：** `ListMcpResourcesTool`

**輸入：**

```python theme={null}
{
    "server": str | None  # 可選的伺服器名稱以篩選資源
}
```

**輸出：**

```python theme={null}
{
    "resources": [
        {
            "uri": str,
            "name": str,
            "description": str | None,
            "mimeType": str | None,
            "server": str,
        }
    ],
    "total": int,
}
```

<h3 id="readmcpresource">
  ReadMcpResource
</h3>

**Tool 名稱：** `ReadMcpResourceTool`

**輸入：**

```python theme={null}
{
    "server": str,  # MCP 伺服器名稱
    "uri": str,  # 要讀取的資源 URI
}
```

**輸出：**

```python theme={null}
{
    "contents": [
        {"uri": str, "mimeType": str | None, "text": str | None, "blob": str | None}
    ],
    "server": str,
}
```

<h2 id="advanced-features-with-claudesdkclient">
  使用 ClaudeSDKClient 的進階功能
</h2>

<h3 id="building-a-continuous-conversation-interface">
  建立持續對話介面
</h3>

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    AssistantMessage,
    TextBlock,
)
import asyncio


class ConversationSession:
    """Maintains a single conversation session with Claude."""

    def __init__(self, options: ClaudeAgentOptions | None = None):
        self.client = ClaudeSDKClient(options)
        self.turn_count = 0

    async def start(self):
        await self.client.connect()
        print("Starting conversation session. Claude will remember context.")
        print(
            "Commands: 'exit' to quit, 'interrupt' to stop current task, 'new' for new session"
        )

        while True:
            user_input = input(f"\n[Turn {self.turn_count + 1}] You: ")

            if user_input.lower() == "exit":
                break
            elif user_input.lower() == "interrupt":
                await self.client.interrupt()
                print("Task interrupted!")
                continue
            elif user_input.lower() == "new":
                # Disconnect and reconnect for a fresh session
                await self.client.disconnect()
                await self.client.connect()
                self.turn_count = 0
                print("Started new conversation session (previous context cleared)")
                continue

            # Send message - the session retains all previous messages
            await self.client.query(user_input)
            self.turn_count += 1

            # Process response
            print(f"[Turn {self.turn_count}] Claude: ", end="")
            async for message in self.client.receive_response():
                if isinstance(message, AssistantMessage):
                    for block in message.content:
                        if isinstance(block, TextBlock):
                            print(block.text, end="")
            print()  # New line after response

        await self.client.disconnect()
        print(f"Conversation ended after {self.turn_count} turns.")


async def main():
    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Write", "Bash"], permission_mode="acceptEdits"
    )
    session = ConversationSession(options)
    await session.start()


# Example conversation:
# Turn 1 - You: "Create a file called hello.py"
# Turn 1 - Claude: "I'll create a hello.py file for you..."
# Turn 2 - You: "What's in that file?"
# Turn 2 - Claude: "The hello.py file I just created contains..." (remembers!)
# Turn 3 - You: "Add a main function to it"
# Turn 3 - Claude: "I'll add a main function to hello.py..." (knows which file!)

asyncio.run(main())
```

<h3 id="using-hooks-for-behavior-modification">
  使用 Hooks 進行行為修改
</h3>

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    HookMatcher,
    HookContext,
)
import asyncio
from typing import Any


async def pre_tool_logger(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Log all tool usage before execution."""
    tool_name = input_data.get("tool_name", "unknown")
    print(f"[PRE-TOOL] About to use: {tool_name}")

    # You can modify or block the tool execution here
    if tool_name == "Bash" and "rm -rf" in str(input_data.get("tool_input", {})):
        return {
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "deny",
                "permissionDecisionReason": "Dangerous command blocked",
            }
        }
    return {}


async def post_tool_logger(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Log results after tool execution."""
    tool_name = input_data.get("tool_name", "unknown")
    print(f"[POST-TOOL] Completed: {tool_name}")
    return {}


async def user_prompt_modifier(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Add context to user prompts."""
    original_prompt = input_data.get("prompt", "")

    # Add a timestamp as additional context for Claude to see
    from datetime import datetime

    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")

    return {
        "hookSpecificOutput": {
            "hookEventName": "UserPromptSubmit",
            "additionalContext": f"[Submitted at {timestamp}] Original prompt: {original_prompt}",
        }
    }


async def main():
    options = ClaudeAgentOptions(
        hooks={
            "PreToolUse": [
                HookMatcher(hooks=[pre_tool_logger]),
                HookMatcher(matcher="Bash", hooks=[pre_tool_logger]),
            ],
            "PostToolUse": [HookMatcher(hooks=[post_tool_logger])],
            "UserPromptSubmit": [HookMatcher(hooks=[user_prompt_modifier])],
        },
        allowed_tools=["Read", "Write", "Bash"],
    )

    async with ClaudeSDKClient(options=options) as client:
        await client.query("List files in current directory")

        async for message in client.receive_response():
            # Hooks will automatically log tool usage
            pass


asyncio.run(main())
```

<h3 id="real-time-progress-monitoring">
  即時進度監控
</h3>

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    AssistantMessage,
    ToolUseBlock,
    ToolResultBlock,
    TextBlock,
)
import asyncio


async def monitor_progress():
    options = ClaudeAgentOptions(
        allowed_tools=["Write", "Bash"], permission_mode="acceptEdits"
    )

    async with ClaudeSDKClient(options=options) as client:
        await client.query("Create 5 Python files with different sorting algorithms")

        # Monitor progress in real-time
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, ToolUseBlock):
                        if block.name == "Write":
                            file_path = block.input.get("file_path", "")
                            print(f"Creating: {file_path}")
                    elif isinstance(block, ToolResultBlock):
                        print("Completed tool execution")
                    elif isinstance(block, TextBlock):
                        print(f"Claude says: {block.text[:100]}...")

        print("Task completed!")


asyncio.run(monitor_progress())
```

<h2 id="example-usage">
  範例使用
</h2>

<h3 id="basic-file-operations-using-query">
  基本檔案操作（使用 query）
</h3>

```python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock
import asyncio


async def create_project():
    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Write", "Bash"],
        permission_mode="acceptEdits",
        cwd="/home/user/project",
    )

    async for message in query(
        prompt="Create a Python project structure with setup.py", options=options
    ):
        if isinstance(message, AssistantMessage):
            for block in message.content:
                if isinstance(block, ToolUseBlock):
                    print(f"Using tool: {block.name}")


asyncio.run(create_project())
```

<h3 id="error-handling">
  錯誤處理
</h3>

```python theme={null}
from claude_agent_sdk import query, CLINotFoundError, ProcessError, CLIJSONDecodeError

try:
    async for message in query(prompt="Hello"):
        print(message)
except CLINotFoundError:
    print(
        "Claude Code CLI not found. Try reinstalling: pip install --force-reinstall claude-agent-sdk"
    )
except ProcessError as e:
    print(f"Process failed with exit code: {e.exit_code}")
except CLIJSONDecodeError as e:
    print(f"Failed to parse response: {e}")
```

<h3 id="streaming-mode-with-client">
  使用客戶端的串流模式
</h3>

```python theme={null}
from claude_agent_sdk import ClaudeSDKClient
import asyncio


async def interactive_session():
    async with ClaudeSDKClient() as client:
        # Send initial message
        await client.query("What's the weather like?")

        # Process responses
        async for msg in client.receive_response():
            print(msg)

        # Send follow-up
        await client.query("Tell me more about that")

        # Process follow-up response
        async for msg in client.receive_response():
            print(msg)


asyncio.run(interactive_session())
```

<h3 id="using-custom-tools-with-claudesdkclient">
  使用 ClaudeSDKClient 的自訂 tools
</h3>

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    tool,
    create_sdk_mcp_server,
    AssistantMessage,
    TextBlock,
)
import asyncio
from typing import Any


# Define custom tools with @tool decorator
@tool("calculate", "Perform mathematical calculations", {"expression": str})
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    try:
        result = eval(args["expression"], {"__builtins__": {}})
        return {"content": [{"type": "text", "text": f"Result: {result}"}]}
    except Exception as e:
        return {
            "content": [{"type": "text", "text": f"Error: {str(e)}"}],
            "is_error": True,
        }


@tool("get_time", "Get current time", {})
async def get_time(args: dict[str, Any]) -> dict[str, Any]:
    from datetime import datetime

    current_time = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    return {"content": [{"type": "text", "text": f"Current time: {current_time}"}]}


async def main():
    # Create SDK MCP server with custom tools
    my_server = create_sdk_mcp_server(
        name="utilities", version="1.0.0", tools=[calculate, get_time]
    )

    # Configure options with the server
    options = ClaudeAgentOptions(
        mcp_servers={"utils": my_server},
        allowed_tools=["mcp__utils__calculate", "mcp__utils__get_time"],
    )

    # Use ClaudeSDKClient for interactive tool usage
    async with ClaudeSDKClient(options=options) as client:
        await client.query("What's 123 * 456?")

        # Process calculation response
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Calculation: {block.text}")

        # Follow up with time query
        await client.query("What time is it now?")

        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Time: {block.text}")


asyncio.run(main())
```

<h2 id="sandbox-configuration">
  沙箱配置
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

沙箱行為的配置。使用此來啟用命令沙箱並以程式設計方式配置網路限制。

```python theme={null}
class SandboxSettings(TypedDict, total=False):
    enabled: bool
    autoAllowBashIfSandboxed: bool
    excludedCommands: list[str]
    allowUnsandboxedCommands: bool
    network: SandboxNetworkConfig
    ignoreViolations: SandboxIgnoreViolations
    enableWeakerNestedSandbox: bool
```

| 屬性                          | 類型                                                    | 預設      | 描述                                                                                                                                  |
| :-------------------------- | :---------------------------------------------------- | :------ | :---------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `bool`                                                | `False` | 為命令執行啟用沙箱模式                                                                                                                         |
| `autoAllowBashIfSandboxed`  | `bool`                                                | `True`  | 啟用沙箱時自動批准 bash 命令                                                                                                                   |
| `excludedCommands`          | `list[str]`                                           | `[]`    | 始終繞過沙箱限制的命令（例如 `["docker"]`）。這些自動執行沙箱外，無需模型參與                                                                                       |
| `allowUnsandboxedCommands`  | `bool`                                                | `True`  | 允許模型請求在沙箱外執行命令。當為 `True` 時，模型可以在 tool 輸入中設定 `dangerouslyDisableSandbox`，這會回退到[權限系統](#permissions-fallback-for-unsandboxed-commands) |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `None`  | 網路特定的沙箱配置                                                                                                                           |
| `ignoreViolations`          | [`SandboxIgnoreViolations`](#sandboxignoreviolations) | `None`  | 配置要忽略的沙箱違規                                                                                                                          |
| `enableWeakerNestedSandbox` | `bool`                                                | `False` | 啟用較弱的嵌套沙箱以相容性                                                                                                                       |

<Note>
  沙箱取決於平台支援，在 Linux 上，需要 `bubblewrap` 和 `socat` 等工具。預設情況下，當 `enabled` 為 `True` 但沙箱無法啟動時，命令會在沙箱外執行，並在 stderr 上顯示警告。此預設與 TypeScript SDK 不同，其中 `failIfUnavailable` 預設為 `true`。

  在沙箱設定中設定 `"failIfUnavailable": True` 以改為停止。該鍵尚未在 `SandboxSettings` 上宣告，但 SDK 會將其轉發給 Claude Code，後者會遵守它。`query()` 然後報告 `ResultMessage`，其中 `subtype="error_during_execution"` 且原因在 `errors` 中。監視該子類型，而不是期望 `query()` 在產生訊息之前引發。
</Note>

<h4 id="example-usage-2">
  範例使用
</h4>

```python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions, SandboxSettings

sandbox_settings: SandboxSettings = {
    "enabled": True,
    "autoAllowBashIfSandboxed": True,
    "network": {"allowLocalBinding": True},
}

async for message in query(
    prompt="Build and test my project",
    options=ClaudeAgentOptions(sandbox=sandbox_settings),
):
    print(message)
```

<Warning>
  **Unix socket 安全性**：`allowUnixSockets` 選項可以授予對強大系統服務的存取權限。例如，允許 `/var/run/docker.sock` 實際上透過 Docker API 授予完整主機系統存取權限，繞過沙箱隔離。僅允許嚴格必要的 Unix sockets，並了解每個的安全含義。
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

沙箱模式的網路特定配置。這些設定適用於當父 [`SandboxSettings`](#sandboxsettings) 中的 `enabled` 為 `True` 時的沙箱化 Bash 命令。它們不會限制 WebFetch 工具，該工具改用[權限規則](/docs/zh-TW/permissions#webfetch)。

```python theme={null}
class SandboxNetworkConfig(TypedDict, total=False):
    allowedDomains: list[str]
    deniedDomains: list[str]
    allowManagedDomainsOnly: bool
    allowUnixSockets: list[str]
    allowAllUnixSockets: bool
    allowLocalBinding: bool
    allowMachLookup: list[str]
    httpProxyPort: int
    socksProxyPort: int
```

| 屬性                        | 類型          | 預設      | 描述                                                            |
| :------------------------ | :---------- | :------ | :------------------------------------------------------------ |
| `allowedDomains`          | `list[str]` | `[]`    | 沙箱化程序可以存取的網域名稱                                                |
| `deniedDomains`           | `list[str]` | `[]`    | 沙箱化程序無法存取的網域名稱。優先於 `allowedDomains`                           |
| `allowManagedDomainsOnly` | `bool`      | `False` | 僅限受管設定：在受管設定中設定時，忽略來自非受管設定來源的 `allowedDomains`。透過 SDK 選項設定時無效 |
| `allowUnixSockets`        | `list[str]` | `[]`    | 程序可以存取的 Unix socket 路徑（例如 Docker socket）                      |
| `allowAllUnixSockets`     | `bool`      | `False` | 允許存取所有 Unix sockets                                           |
| `allowLocalBinding`       | `bool`      | `False` | 允許程序繫結到本地連接埠（例如開發伺服器）                                         |
| `allowMachLookup`         | `list[str]` | `[]`    | 僅限 macOS：允許的 XPC/Mach 服務名稱。支援尾部萬用字元                           |
| `httpProxyPort`           | `int`       | `None`  | 網路請求的 HTTP proxy 連接埠                                          |
| `socksProxyPort`          | `int`       | `None`  | 網路請求的 SOCKS proxy 連接埠                                         |

<Note>
  內建沙箱 proxy 根據請求的主機名稱強制執行網路允許清單，不會終止或檢查 TLS 流量，因此[網域前置](https://en.wikipedia.org/wiki/Domain_fronting)等技術可能會繞過它。有關詳細資訊，請參閱[沙箱安全限制](/docs/zh-TW/sandboxing#security-limitations)，以及[安全部署](/docs/zh-TW/agent-sdk/secure-deployment#traffic-forwarding)以配置 TLS 終止 proxy。
</Note>

<h3 id="sandboxignoreviolations">
  `SandboxIgnoreViolations`
</h3>

用於忽略特定沙箱違規的配置。

```python theme={null}
class SandboxIgnoreViolations(TypedDict, total=False):
    file: list[str]
    network: list[str]
```

| 屬性        | 類型          | 預設   | 描述           |
| :-------- | :---------- | :--- | :----------- |
| `file`    | `list[str]` | `[]` | 要忽略違規的檔案路徑模式 |
| `network` | `list[str]` | `[]` | 要忽略違規的網路模式   |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  未沙箱化命令的權限回退
</h3>

當 `allowUnsandboxedCommands` 啟用時，模型可以透過在 tool 輸入中設定 `dangerouslyDisableSandbox: True` 來請求在沙箱外執行命令。這些請求回退到現有權限系統，意味著您的 `can_use_tool` 處理程序將被呼叫，允許您實現自訂授權邏輯。

<Note>
  **`excludedCommands` vs `allowUnsandboxedCommands`：**

  * `excludedCommands`：始終自動繞過沙箱的靜態命令清單（例如 `["docker"]`）。模型無法控制此。
  * `allowUnsandboxedCommands`：讓模型在執行時透過在 tool 輸入中設定 `dangerouslyDisableSandbox: True` 來決定是否請求未沙箱化執行。
</Note>

```python theme={null}
from claude_agent_sdk import (
    query,
    ClaudeAgentOptions,
    HookMatcher,
    PermissionResultAllow,
    PermissionResultDeny,
    ToolPermissionContext,
)


async def can_use_tool(
    tool: str, input: dict, context: ToolPermissionContext
) -> PermissionResultAllow | PermissionResultDeny:
    # Check if the model is requesting to bypass the sandbox
    if tool == "Bash" and input.get("dangerouslyDisableSandbox"):
        # The model is requesting to run this command outside the sandbox
        print(f"Unsandboxed command requested: {input.get('command')}")

        if is_command_authorized(input.get("command")):
            return PermissionResultAllow()
        return PermissionResultDeny(
            message="Command not authorized for unsandboxed execution"
        )
    return PermissionResultAllow()


# Required: dummy hook keeps the stream open for can_use_tool
async def dummy_hook(input_data, tool_use_id, context):
    return {"continue_": True}


async def prompt_stream():
    yield {
        "type": "user",
        "message": {"role": "user", "content": "Deploy my application"},
    }


async def main():
    async for message in query(
        prompt=prompt_stream(),
        options=ClaudeAgentOptions(
            sandbox={
                "enabled": True,
                "allowUnsandboxedCommands": True,  # Model can request unsandboxed execution
            },
            permission_mode="default",
            can_use_tool=can_use_tool,
            hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
        ),
    ):
        print(message)
```

此模式使您能夠：

* **審計模型請求**：記錄模型何時請求未沙箱化執行
* **實現允許清單**：僅允許特定命令在沙箱外執行
* **新增批准工作流程**：需要明確授權以進行特權操作

<Warning>
  使用 `dangerouslyDisableSandbox: True` 執行的命令具有完整系統存取權限。確保您的 `can_use_tool` 處理程序仔細驗證這些請求。

  如果 `permission_mode` 設定為 `bypassPermissions` 且 `allow_unsandboxed_commands` 啟用，模型可以自主執行沙箱外的命令，無需任何批准提示。此組合實際上允許模型無聲地逃脫沙箱隔離。
</Warning>

<h2 id="see-also">
  另見
</h2>

* [SDK 概述](/docs/zh-TW/agent-sdk/overview) - 一般 SDK 概念
* [TypeScript SDK 參考](/docs/zh-TW/agent-sdk/typescript) - TypeScript SDK 文件
* [CLI 參考](/docs/zh-TW/cli-reference) - 命令列介面
* [常見工作流程](/docs/zh-TW/common-workflows) - 逐步指南
