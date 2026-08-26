> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 以程式方式執行 Claude Code

> 使用 Agent SDK 從 CLI、Python 或 TypeScript 以程式方式執行 Claude Code。

[Agent SDK](/docs/zh-TW/agent-sdk/overview) 提供與 Claude Code 相同的工具、agent 迴圈和上下文管理。它可作為 CLI 用於指令碼和 CI/CD，或作為 [Python](/docs/zh-TW/agent-sdk/python) 和 [TypeScript](/docs/zh-TW/agent-sdk/typescript) 套件供完整的程式控制。

若要以非互動模式執行 Claude Code，請傳遞 `-p` 和您的提示以及任何 [CLI 選項](/docs/zh-TW/cli-reference)：

```bash theme={null}
claude -p "Find and fix the bug in auth.py" --allowedTools "Read,Edit,Bash"
```

本頁涵蓋透過 CLI (`claude -p`) 使用 Agent SDK。如需具有結構化輸出、工具核准回呼和原生訊息物件的 Python 和 TypeScript SDK 套件，請參閱 [完整 Agent SDK 文件](/docs/zh-TW/agent-sdk/overview)。

<h2 id="basic-usage">
  基本用法
</h2>

將 `-p`（或 `--print`）旗標新增至任何 `claude` 命令以非互動方式執行它。所有 [CLI 選項](/docs/zh-TW/cli-reference) 都適用於 `-p`，包括：

* `--continue` 用於 [繼續對話](#continue-conversations)
* `--allowedTools` 用於 [自動核准工具](#auto-approve-tools)
* `--output-format` 用於 [結構化輸出](#get-structured-output)

此範例詢問 Claude 關於您的程式碼庫的問題並列印回應：

```bash theme={null}
claude -p "What does the auth module do?"
```

<h3 id="start-faster-with-bare-mode">
  使用裸機模式加快速度
</h3>

新增 `--bare` 以跳過 hooks、skills、plugins、MCP 伺服器、自動記憶體和 CLAUDE.md 的自動探索來減少啟動時間。沒有它，`claude -p` 會載入互動式工作階段會載入的相同 [上下文](/docs/zh-TW/how-claude-code-works#the-context-window)，包括在工作目錄或 `~/.claude` 中設定的任何內容。

裸機模式對於 CI 和指令碼很有用，您需要在每台機器上獲得相同的結果。隊友 `~/.claude` 中的 hook 或專案的 `.mcp.json` 中的 MCP 伺服器不會執行，因為裸機模式永遠不會讀取它們。只有您明確傳遞的旗標才會生效。

此範例在裸機模式下執行一次性摘要任務，並預先核准 Read 工具，以便呼叫完成而無需許可提示：

```bash theme={null}
claude --bare -p "Summarize this file" --allowedTools "Read"
```

在裸機模式下，Claude 可以存取 Bash、檔案讀取和檔案編輯工具。使用旗標傳遞您需要的任何上下文：

| 要載入       | 使用                                                      |
| --------- | ------------------------------------------------------- |
| 系統提示新增    | `--append-system-prompt`, `--append-system-prompt-file` |
| 設定        | `--settings <file-or-json>`                             |
| MCP 伺服器   | `--mcp-config <file-or-json>`                           |
| 自訂 agents | `--agents <json>`                                       |
| 外掛程式      | `--plugin-dir <path>`, `--plugin-url <url>`             |

裸機模式跳過 OAuth 和鑰匙圈讀取。Anthropic 驗證必須來自 `ANTHROPIC_API_KEY` 或傳遞給 `--settings` 的 JSON 中的 `apiKeyHelper`。Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 使用其通常的提供者認證。

<Note>
  `--bare` 是指令碼和 SDK 呼叫的建議模式，將在未來版本中成為 `-p` 的預設值。
</Note>

<h3 id="background-tasks-at-exit">
  結束時的背景工作
</h3>

如果 Claude 在 `claude -p` 執行期間啟動 [背景 Bash 工作](/docs/zh-TW/tools-reference#bash-tool-behavior)，例如開發伺服器或監視組建，該工作將在 Claude 傳回其最終結果且 stdin 已關閉後約五秒鐘終止。寬限期允許在結果之後立即完成的工作仍然傳遞其輸出。在 v2.1.163 之前，永不退出的背景程序會無限期地保持 `claude -p` 呼叫開啟。

背景 [subagents](/docs/zh-TW/sub-agents) 和工作流程不受五秒寬限期的限制，因為它們的結果是最終輸出的一部分，所以 `claude -p` 會等待它們完成。從 v2.1.182 開始，該等待預設上限為十分鐘，因此卡住的背景 agent 無法無限期地保持程序開啟。使用 [`CLAUDE_CODE_PRINT_BG_WAIT_CEILING_MS`](/docs/zh-TW/env-vars) 調整上限，或將其設定為 `0` 以無限制地等待。

<h2 id="examples">
  範例
</h2>

這些範例突出顯示常見的 CLI 模式。對於 CI 和其他指令碼呼叫，新增 [`--bare`](#start-faster-with-bare-mode) 以便它們不會選擇本地設定的任何內容。

<h3 id="pipe-data-through-claude">
  透過 Claude 管道傳送資料
</h3>

非互動模式讀取 stdin，因此您可以像任何其他命令列工具一樣管道傳送資料並重新導向回應。

此範例將建置日誌管道傳送至 Claude 並將說明寫入檔案：

```bash theme={null}
cat build-error.txt | claude -p 'concisely explain the root cause of this build error' > output.txt
```

使用 `--output-format json`，回應承載包括 `total_cost_usd` 和每個模型的成本明細，因此指令碼呼叫者可以追蹤每次叫用的支出，而無需查詢 [使用儀表板](/docs/zh-TW/costs)。

<Note>
  自 Claude Code v2.1.128 起，管道傳送的 stdin 上限為 10MB。如果超過上限，Claude Code 會以清晰的錯誤和非零狀態代碼退出。若要處理更大的輸入，請將內容寫入檔案，並在提示中參考檔案路徑，而不是管道傳送它。
</Note>

<h3 id="add-claude-to-a-build-script">
  將 Claude 新增至建置指令碼
</h3>

您可以在指令碼中包裝非互動呼叫，以將 Claude 用作專案特定的 linter 或審查者。

此 `package.json` 指令碼將針對 `main` 的差異管道傳送至 Claude，並要求它報告拼寫錯誤。管道傳送差異意味著 Claude 不需要 Bash 權限來讀取它，而逸出的雙引號使指令碼可移植到 Windows：

```json theme={null}
{
  "scripts": {
    "lint:claude": "git diff main | claude -p \"you are a typo linter. for each typo in this diff, report filename:line on one line and the issue on the next. return nothing else.\""
  }
}
```

<h3 id="get-structured-output">
  取得結構化輸出
</h3>

使用 `--output-format` 控制回應的傳回方式：

* `text`（預設）：純文字輸出
* `json`：包含結果、工作階段 ID 和中繼資料的結構化 JSON
* `stream-json`：用於即時串流的換行分隔 JSON

此範例以 JSON 格式傳回專案摘要及工作階段中繼資料，文字結果在 `result` 欄位中：

```bash theme={null}
claude -p "Summarize this project" --output-format json
```

若要取得符合特定結構描述的輸出，請使用 `--output-format json` 搭配 `--json-schema` 和 [JSON Schema](https://json-schema.org/) 定義。回應包含關於請求的中繼資料（工作階段 ID、使用情況等），結構化輸出在 `structured_output` 欄位中。

此範例從 auth.py 提取函式名稱並將其作為字串陣列傳回：

```bash theme={null}
claude -p "Extract the main function names from auth.py" \
  --output-format json \
  --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}'
```

如果值不是有效的 JSON Schema，`claude` 會以 `Error: --json-schema is not a valid JSON Schema` 退出，後面跟著驗證器的診斷。Claude Code 接受使用 `format` 關鍵字的結構描述，例如 `"format": "email"`，但將 `format` 視為註解，不強制執行它。在 v2.1.205 之前，Claude Code 會以無聲方式忽略無效的結構描述並傳回非結構化文字，並將任何包含 `format` 的結構描述視為無效。

<Tip>
  使用 [jq](https://jqlang.github.io/jq/) 之類的工具來解析回應並提取特定欄位：

  ```bash theme={null}
  # Extract the text result
  claude -p "Summarize this project" --output-format json | jq -r '.result'

  # Extract structured output
  claude -p "Extract function names from auth.py" \
    --output-format json \
    --json-schema '{"type":"object","properties":{"functions":{"type":"array","items":{"type":"string"}}},"required":["functions"]}' \
    | jq '.structured_output'
  ```
</Tip>

<h3 id="stream-responses">
  串流回應
</h3>

使用 `--output-format stream-json` 搭配 `--verbose` 和 `--include-partial-messages` 以在產生令牌時接收它們。每一行都是代表事件的 JSON 物件：

```bash theme={null}
claude -p "Explain recursion" --output-format stream-json --verbose --include-partial-messages
```

串流的最後一行是包含最終回應文字、成本和工作階段中繼資料的 `result` 訊息。在 v2.1.208 之前，管道傳送大型回應可能會截斷最後一行並省略 `result` 訊息。

下列範例使用 [jq](https://jqlang.github.io/jq/) 篩選文字差異並僅顯示串流文字。`-r` 旗標輸出原始字串（無引號），`-j` 不帶換行符號的聯結，因此令牌會連續串流：

```bash theme={null}
claude -p "Write a poem" --output-format stream-json --verbose --include-partial-messages | \
  jq -rj 'select(.type == "stream_event" and .event.delta.type? == "text_delta") | .event.delta.text'
```

當 API 請求因可重試錯誤而失敗時，Claude Code 會在重試前發出 `system/api_retry` 事件。您可以使用此來顯示重試進度或實施自訂退避邏輯。

| 欄位               | 類型            | 描述                                                                                                                                                                                |
| ---------------- | ------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`           | `"system"`    | 訊息類型                                                                                                                                                                              |
| `subtype`        | `"api_retry"` | 將此識別為重試事件                                                                                                                                                                         |
| `attempt`        | 整數            | 目前嘗試次數，從 1 開始                                                                                                                                                                     |
| `max_retries`    | 整數            | 允許的總重試次數                                                                                                                                                                          |
| `retry_delay_ms` | 整數            | 毫秒直到下一次嘗試                                                                                                                                                                         |
| `error_status`   | 整數或 null      | HTTP 狀態碼，或 `null` 表示沒有 HTTP 回應的連線錯誤                                                                                                                                               |
| `error`          | 字串            | 錯誤類別：`authentication_failed`、`oauth_org_not_allowed`、`billing_error`、`rate_limit`、`overloaded`、`invalid_request`、`model_not_found`、`server_error`、`max_output_tokens` 或 `unknown` |
| `uuid`           | 字串            | 唯一事件識別碼                                                                                                                                                                           |
| `session_id`     | 字串            | 事件所屬的工作階段                                                                                                                                                                         |

`system/init` 事件報告工作階段中繼資料，包括模型、工具、MCP 伺服器和載入的外掛程式。除非設定了啟動事件，否則它是串流中的第一個事件：

* `plugin_install` 事件，當設定了 [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/zh-TW/env-vars) 時。
* [`hook_started`、`hook_progress` 和 `hook_response` 事件](/docs/zh-TW/agent-sdk/typescript#sdkhookstartedmessage)，當設定的 [`SessionStart`](/docs/zh-TW/hooks#sessionstart) 或 [`Setup`](/docs/zh-TW/hooks#setup) hook 執行時。這些會在 hook 產生時串流。Claude Code v2.1.169 至 v2.1.203 在 hook 完成後以一個批次傳遞它們，仍在 `system/init` 之前；v2.1.204 恢復了即時傳遞。

該事件也包含一個選用的 `capabilities` 字串陣列，命名此 Claude Code 版本實施的協定行為，例如 `interrupt_receipt_v1`。檢查它以進行功能偵測，而不是比較版本字串，並忽略您不認識的值。該欄位需要 Claude Code v2.1.205 或更新版本，在較早版本中不存在。請參閱 [`SDKSystemMessage`](/docs/zh-TW/agent-sdk/typescript#sdksystemmessage) 以取得功能清單。

使用外掛程式欄位在外掛程式未載入時使 CI 失敗：

| 欄位              | 類型 | 描述                                                                                                                                  |
| --------------- | -- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `plugins`       | 陣列 | 成功載入的外掛程式，每個都有 `name` 和 `path`                                                                                                      |
| `plugin_errors` | 陣列 | 外掛程式載入時間錯誤，每個都有 `plugin`、`type` 和 `message`。包括不滿足的相依性版本和 `--plugin-dir` 載入失敗，例如遺失的路徑或無效的封存。受影響的外掛程式被降級並從 `plugins` 中缺失。當沒有錯誤時，金鑰被省略 |

當設定了 [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/zh-TW/env-vars) 時，Claude Code 在第一次轉換前發出 `system/plugin_install` 事件，同時市場外掛程式安裝。使用這些在您自己的 UI 中顯示安裝進度。

| 欄位           | 類型                                                   | 描述                                                           |
| ------------ | ---------------------------------------------------- | ------------------------------------------------------------ |
| `type`       | `"system"`                                           | 訊息類型                                                         |
| `subtype`    | `"plugin_install"`                                   | 將此識別為外掛程式安裝事件                                                |
| `status`     | `"started"`、`"installed"`、`"failed"` 或 `"completed"` | `started` 和 `completed` 括住整體安裝；`installed` 和 `failed` 報告個別市場 |
| `name`       | 字串，選用                                                | 市場名稱，在 `installed` 和 `failed` 上出現                            |
| `error`      | 字串，選用                                                | 失敗訊息，在 `failed` 上出現                                          |
| `uuid`       | 字串                                                   | 唯一事件識別碼                                                      |
| `session_id` | 字串                                                   | 事件所屬的工作階段                                                    |

如需具有回呼和訊息物件的程式化串流，請參閱 Agent SDK 文件中的 [即時串流回應](/docs/zh-TW/agent-sdk/streaming-output)。

<h3 id="auto-approve-tools">
  自動核准工具
</h3>

使用 `--allowedTools` 讓 Claude 使用某些工具而無需提示。此範例執行測試套件並修復失敗，允許 Claude 執行 Bash 命令和讀取/編輯檔案而無需請求許可：

```bash theme={null}
claude -p "Run the test suite and fix any failures" \
  --allowedTools "Bash,Read,Edit"
```

若要為整個工作階段設定基準而不是列出個別工具，請傳遞 [權限模式](/docs/zh-TW/permission-modes)。`dontAsk` 拒絕 `permissions.allow` 規則或 [唯讀命令集](/docs/zh-TW/permissions#read-only-commands) 中未包含的任何內容，這對於鎖定的 CI 執行很有用。`AskUserQuestion`、連接器工具 [您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools) 和標記為 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具即使當允許規則符合時也被拒絕。

`acceptEdits` 讓 Claude 寫入檔案而無需提示，也自動核准常見的檔案系統命令，例如 `mkdir`、`touch`、`mv` 和 `cp`。其他 shell 命令和網路請求仍然需要 `--allowedTools` 項目或 `permissions.allow` 規則，否則當嘗試執行時執行會中止：

```bash theme={null}
claude -p "Apply the lint fixes" --permission-mode acceptEdits
```

<h3 id="create-a-commit">
  建立提交
</h3>

此範例檢查暫存的變更並建立具有適當訊息的提交：

```bash theme={null}
claude -p "Look at my staged changes and create an appropriate commit" \
  --allowedTools "Bash(git diff *),Bash(git log *),Bash(git status *),Bash(git commit *)"
```

`--allowedTools` 旗標使用 [權限規則語法](/docs/zh-TW/settings#permission-rule-syntax)。尾部的 ` *` 啟用前綴匹配，因此 `Bash(git diff *)` 允許任何以 `git diff` 開頭的命令。空格在 `*` 之前很重要：沒有它，`Bash(git diff*)` 也會符合 `git diff-index`。

<Note>
  使用者叫用的 [skills](/docs/zh-TW/skills) 和自訂命令在 `-p` 模式中運作：在提示字串中包含 `/skill-name`，Claude Code 會在執行前展開它。開啟互動對話的內建命令，例如 `/login`，在 `-p` 模式中不可用。`/model`、`/effort`、`/fast`、`/color` 和 `/rename` 接受值作為引數，例如 `/model sonnet`，`/mcp` 不帶引數會列印伺服器狀態的文字摘要；這些形式需要 Claude Code v2.1.205 或更新版本，並遵循每個命令的 [可用性注意事項](/docs/zh-TW/commands#all-commands)。若要從 `-p` 叫用變更設定，請將 `key=value` 傳遞至 `/config`，例如 `/config thinking=false`。
</Note>

<h3 id="customize-the-system-prompt">
  自訂系統提示
</h3>

使用 `--append-system-prompt` 新增指示同時保持 Claude Code 的預設行為。此範例將 PR 差異管道傳送至 Claude 並指示它檢查安全漏洞：

```bash theme={null}
gh pr diff "$1" | claude -p \
  --append-system-prompt "You are a security engineer. Review for vulnerabilities." \
  --output-format json
```

請參閱 [系統提示旗標](/docs/zh-TW/cli-reference#system-prompt-flags) 以取得更多選項，包括 `--system-prompt` 以完全取代預設提示。

<h3 id="continue-conversations">
  繼續對話
</h3>

使用 `--continue` 繼續最近的對話，或使用 `--resume` 搭配工作階段 ID 以繼續特定對話。此範例執行檢查，然後傳送後續提示：

```bash theme={null}
# First request
claude -p "Review this codebase for performance issues"

# Continue the most recent conversation
claude -p "Now focus on the database queries" --continue
claude -p "Generate a summary of all issues found" --continue
```

如果您執行多個對話，請擷取工作階段 ID 以繼續特定對話：

```bash theme={null}
session_id=$(claude -p "Start a review" --output-format json | jq -r '.session_id')
claude -p "Continue that review" --resume "$session_id"
```

從同一目錄執行兩個命令：工作階段 ID 查詢的範圍限於目前專案目錄及其 git worktrees。請參閱 [繼續工作階段](/docs/zh-TW/sessions#resume-a-session) 以取得完整的範圍規則。

<h2 id="next-steps">
  後續步驟
</h2>

* [Agent SDK 快速入門](/docs/zh-TW/agent-sdk/quickstart)：使用 Python 或 TypeScript 建立您的第一個 agent
* [CLI 參考](/docs/zh-TW/cli-reference)：所有 CLI 旗標和選項
* [GitHub Actions](/docs/zh-TW/github-actions)：在 GitHub 工作流程中使用 Agent SDK
* [GitLab CI/CD](/docs/zh-TW/gitlab-ci-cd)：在 GitLab 管道中使用 Agent SDK
