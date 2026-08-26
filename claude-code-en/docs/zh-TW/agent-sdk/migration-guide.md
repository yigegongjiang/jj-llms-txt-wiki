> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 遷移至 Claude Agent SDK

> 將 Claude Code TypeScript 和 Python SDK 遷移至 Claude Agent SDK 的指南

<h2 id="overview">
  概述
</h2>

Claude Code SDK 已重新命名為 **Claude Agent SDK**，其文檔已重新組織。此變更反映了該 SDK 在構建超越編碼任務的 AI 代理方面的更廣泛功能。

<h2 id="what’s-changed">
  變更內容
</h2>

| 方面               | 舊版                          | 新版                               |
| :--------------- | :-------------------------- | :------------------------------- |
| **套件名稱 (TS/JS)** | `@anthropic-ai/claude-code` | `@anthropic-ai/claude-agent-sdk` |
| **Python 套件**    | `claude-code-sdk`           | `claude-agent-sdk`               |
| **文檔位置**         | Claude Code 文檔              | API 指南 → Agent SDK 部分            |

<Note>
  **文檔變更：** Agent SDK 文檔已從 Claude Code 文檔移至 API 指南下的專用 [Agent SDK](/docs/zh-TW/agent-sdk/overview) 部分。Claude Code 文檔現在專注於 CLI 工具和自動化功能。
</Note>

<h2 id="migration-steps">
  遷移步驟
</h2>

<h3 id="for-typescript/javascript-projects">
  針對 TypeScript/JavaScript 專案
</h3>

**1. 卸載舊套件：**

```bash theme={null}
npm uninstall @anthropic-ai/claude-code
```

**2. 安裝新套件：**

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

**3. 更新您的匯入：**

將所有匯入從 `@anthropic-ai/claude-code` 變更為 `@anthropic-ai/claude-agent-sdk`：

```typescript theme={null}
// 之前
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-code";

// 之後
import { query, tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
```

**4. 更新 package.json 依賴項：**

如果您在 `package.json` 中列出了該套件，請更新它：

之前：

```json theme={null}
{
  "dependencies": {
    "@anthropic-ai/claude-code": "^0.0.42"
  }
}
```

之後：

```json theme={null}
{
  "dependencies": {
    "@anthropic-ai/claude-agent-sdk": "^0.2.0"
  }
}
```

**5. 檢查[重大變更](#breaking-changes)**

進行完成遷移所需的任何代碼變更。

<h3 id="for-python-projects">
  針對 Python 專案
</h3>

**1. 卸載舊套件：**

```bash theme={null}
pip uninstall claude-code-sdk
```

**2. 安裝新套件：**

```bash theme={null}
pip install claude-agent-sdk
```

**3. 更新您的匯入：**

將所有匯入從 `claude_code_sdk` 變更為 `claude_agent_sdk`：

```python theme={null}
# 之前
from claude_code_sdk import query, ClaudeCodeOptions

# 之後
from claude_agent_sdk import query, ClaudeAgentOptions
```

**4. 更新類型名稱：**

將 `ClaudeCodeOptions` 變更為 `ClaudeAgentOptions`：

```python theme={null}
# 之前
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7")

# 之後
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7")
```

**5. 檢查[重大變更](#breaking-changes)**

進行完成遷移所需的任何代碼變更。

<h2 id="breaking-changes">
  重大變更
</h2>

<Warning>
  為了改進隔離和明確配置，Claude Agent SDK v0.1.0 為從 Claude Code SDK 遷移的用戶引入了重大變更。在遷移前請仔細檢查本部分。
</Warning>

<h3 id="python-claudecodeoptions-renamed-to-claudeagentoptions">
  Python：ClaudeCodeOptions 重新命名為 ClaudeAgentOptions
</h3>

**變更內容：** Python SDK 類型 `ClaudeCodeOptions` 已重新命名為 `ClaudeAgentOptions`。

**遷移：**

```python theme={null}
# 之前 (claude-code-sdk)
from claude_code_sdk import query, ClaudeCodeOptions

options = ClaudeCodeOptions(model="claude-opus-4-7", permission_mode="acceptEdits")

# 之後 (claude-agent-sdk)
from claude_agent_sdk import query, ClaudeAgentOptions

options = ClaudeAgentOptions(model="claude-opus-4-7", permission_mode="acceptEdits")
```

**為什麼變更：** 類型名稱現在與「Claude Agent SDK」品牌相符，並在 SDK 的命名約定中提供一致性。

<h3 id="system-prompt-no-longer-default">
  系統提示不再是預設值
</h3>

**變更內容：** SDK 不再預設使用 Claude Code 的系統提示。

**遷移：**

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // 之前 (v0.0.x) - 預設使用 Claude Code 的系統提示
  const before = query({ prompt: "Hello" });

  // 之後 (v0.1.0) - 預設使用最小系統提示
  // 要獲得舊行為，明確請求 Claude Code 的預設值：
  const presetResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: { type: "preset", preset: "claude_code" }
    }
  });

  // 或使用自訂系統提示：
  const customResult = query({
    prompt: "Hello",
    options: {
      systemPrompt: "You are a helpful coding assistant"
    }
  });
  ```

  ```python Python theme={null}
  # 之前 (v0.0.x) - 預設使用 Claude Code 的系統提示
  async for message in query(prompt="Hello"):
      print(message)

  # 之後 (v0.1.0) - 預設使用最小系統提示
  # 要獲得舊行為，明確請求 Claude Code 的預設值：
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(
          system_prompt={"type": "preset", "preset": "claude_code"}  # 使用預設值
      ),
  ):
      print(message)

  # 或使用自訂系統提示：
  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(system_prompt="You are a helpful coding assistant"),
  ):
      print(message)
  ```
</CodeGroup>

**為什麼變更：** 為 SDK 應用程式提供更好的控制和隔離。您現在可以構建具有自訂行為的代理，而無需繼承 Claude Code 的 CLI 焦點指令。

<h3 id="settings-sources-default">
  設定來源預設值
</h3>

此預設值在 v0.1.0 中曾短暫變更，然後被還原，因此無需進行遷移操作。

**目前行為：** 在 `query()` 上省略 `settingSources` 會載入用戶、專案和本地檔案系統設定，與 CLI 相符。這包括 `~/.claude/settings.json`、`.claude/settings.json`、`.claude/settings.local.json`、CLAUDE.md 檔案和自訂命令。

要從檔案系統設定中隔離運行，請傳遞空陣列：

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const isolatedResult = query({
    prompt: "Hello",
    options: {
      settingSources: [] // 未載入檔案系統設定
    }
  });

  // 或僅載入特定來源：
  const projectOnlyResult = query({
    prompt: "Hello",
    options: {
      settingSources: ["project"] // 僅專案設定
    }
  });
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(setting_sources=[]),  # 未載入檔案系統設定
  ):
      print(message)

  # 或僅載入特定來源：
  async for message in query(
      prompt="Hello",
      options=ClaudeAgentOptions(
          setting_sources=["project"]  # 僅專案設定
      ),
  ):
      print(message)
  ```
</CodeGroup>

隔離對於 CI/CD 管道、已部署的應用程式、測試環境和多租戶系統特別重要，其中本地自訂不應洩露。

<Note>
  SDK v0.1.0 曾短暫預設為未載入任何設定；這在後續版本中被還原。Python SDK 0.1.59 及更早版本將空清單視為與省略選項相同，因此在依賴 `setting_sources=[]` 之前請升級。請參閱 [settingSources 不控制的內容](/docs/zh-TW/agent-sdk/claude-code-features#what-settingsources-does-not-control) 以了解即使 `settingSources` 為 `[]` 時也會讀取的輸入。
</Note>

<h2 id="why-the-rename">
  為什麼重新命名？
</h2>

Claude Code SDK 最初是為編碼任務設計的，但它已發展成為構建所有類型 AI 代理的強大框架。新名稱「Claude Agent SDK」更好地反映了其功能：

* 構建業務代理（法律助手、財務顧問、客戶支援）
* 建立專門的編碼代理（SRE 機器人、安全審查員、代碼審查代理）
* 為任何領域開發自訂代理，具有工具使用、MCP 整合等功能

<h2 id="getting-help">
  獲取幫助
</h2>

如果您在遷移過程中遇到任何問題：

**針對 TypeScript/JavaScript：**

1. 檢查所有匯入是否已更新為使用 `@anthropic-ai/claude-agent-sdk`
2. 驗證您的 package.json 具有新套件名稱
3. 執行 `npm install` 以確保依賴項已更新

**針對 Python：**

1. 檢查所有匯入是否已更新為使用 `claude_agent_sdk`
2. 驗證您的 requirements.txt 或 pyproject.toml 具有新套件名稱
3. 執行 `pip install claude-agent-sdk` 以確保套件已安裝

<h2 id="next-steps">
  後續步驟
</h2>

* 探索 [Agent SDK 概述](/docs/zh-TW/agent-sdk/overview) 以了解可用功能
* 查看 [TypeScript SDK 參考](/docs/zh-TW/agent-sdk/typescript) 以取得詳細的 API 文檔
* 檢查 [Python SDK 參考](/docs/zh-TW/agent-sdk/python) 以取得 Python 特定文檔
* 了解 [自訂工具](/docs/zh-TW/agent-sdk/custom-tools) 和 [MCP 整合](/docs/zh-TW/agent-sdk/mcp)
