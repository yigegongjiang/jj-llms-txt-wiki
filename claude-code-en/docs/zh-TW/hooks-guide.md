> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用 hooks 自動化工作流程

> 當 Claude Code 編輯檔案、完成任務或需要輸入時，自動執行 shell 命令。格式化程式碼、發送通知、驗證命令並強制執行專案規則。

Hooks 是使用者定義的 shell 命令，在 Claude Code 生命週期的特定時間點執行。它們提供對 Claude Code 行為的確定性控制，確保某些操作始終發生，而不是依賴 LLM 選擇執行它們。使用 hooks 來強制執行專案規則、自動化重複性任務，並將 Claude Code 與您現有的工具整合。

對於需要判斷而不是確定性規則的決策，您也可以使用[基於提示的 hooks](#prompt-based-hooks) 或[基於代理的 hooks](#agent-based-hooks)，它們使用 Claude 模型來評估條件。

有關擴展 Claude Code 的其他方式，請參閱[skills](/docs/zh-TW/skills)以提供 Claude 額外的指令和可執行命令、[subagents](/docs/zh-TW/sub-agents)以在隔離的上下文中執行任務，以及[plugins](/docs/zh-TW/plugins)以打包要在專案間共享的擴展。

<Tip>
  本指南涵蓋常見用例和入門方式。有關完整的事件架構、JSON 輸入/輸出格式和非同步 hooks 和 MCP 工具 hooks 等進階功能，請參閱 [Hooks 參考](/docs/zh-TW/hooks)。
</Tip>

<h2 id="set-up-your-first-hook">
  設定您的第一個 hook
</h2>

若要建立 hook，請將 `hooks` 區塊新增到[設定檔](#configure-hook-location)。本逐步解說建立一個桌面通知 hook，因此每當 Claude 等待您的輸入而不是監視終端時，您都會收到警報。

<Steps>
  <Step title="將 hook 新增到您的設定">
    開啟 `~/.claude/settings.json` 並新增 `Notification` hook。下面的範例使用 `osascript` 進行 macOS；有關 Linux 和 Windows 命令，請參閱[當 Claude 需要輸入時收到通知](#get-notified-when-claude-needs-input)。

    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'"
              }
            ]
          }
        ]
      }
    }
    ```

    如果您的設定檔已經有 `hooks` 鍵，請將 `Notification` 作為現有事件鍵的同級項目新增，而不是替換整個物件。每個事件名稱是單個 `hooks` 物件內的鍵：

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write" }]
          }
        ],
        "Notification": [
          {
            "matcher": "",
            "hooks": [{ "type": "command", "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'" }]
          }
        ]
      }
    }
    ```

    您也可以透過在 CLI 中描述您想要的內容，要求 Claude 為您編寫 hook。
  </Step>

  <Step title="驗證配置">
    輸入 `/hooks` 以開啟 hooks 瀏覽器。您將看到所有可用 hook 事件的列表，每個配置了 hooks 的事件旁邊都有一個計數。選擇 `Notification` 以確認您的新 hook 出現在列表中。選擇 hook 會顯示其詳細資訊：事件、匹配器、類型、來源檔案和命令。
  </Step>

  <Step title="測試 hook">
    按 `Esc` 返回 CLI。要求 Claude 執行需要權限的操作，然後切換離開終端。您應該會收到桌面通知。
  </Step>
</Steps>

<Tip>
  `/hooks` 選單是唯讀的。若要新增、修改或移除 hooks，請直接編輯您的設定 JSON 或要求 Claude 進行變更。
</Tip>

<h2 id="what-you-can-automate">
  您可以自動化的內容
</h2>

Hooks 讓您在 Claude Code 生命週期的關鍵點執行程式碼：編輯後格式化檔案、在執行前阻止命令、當 Claude 需要輸入時發送通知、在工作階段開始時注入上下文等。有關 hook 事件的完整列表，請參閱 [Hooks 參考](/docs/zh-TW/hooks#hook-lifecycle)。

每個範例都包含一個現成可用的配置區塊，您可以將其新增到[設定檔](#configure-hook-location)。

有關 hooks 執行單獨模型審查並將發現結果反饋到工作階段中的生產範例，請參閱 [`security-guidance` plugin 如何與 Claude Code 整合](/docs/zh-TW/security-guidance#how-the-plugin-integrates-with-claude-code)。

<h3 id="get-notified-when-claude-needs-input">
  當 Claude 需要輸入時收到通知
</h3>

每當 Claude 完成工作並需要您的輸入時收到桌面通知，這樣您可以切換到其他任務而無需檢查終端。

此 hook 使用 `Notification` 事件，當 Claude 等待輸入或權限時觸發。下面的每個標籤使用平台的原生通知命令。將此新增到 `~/.claude/settings.json`：

<Tabs>
  <Tab title="macOS">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "osascript -e 'display notification \"Claude Code needs your attention\" with title \"Claude Code\"'"
              }
            ]
          }
        ]
      }
    }
    ```

    <Accordion title="如果沒有出現通知">
      `osascript` 透過內建的 Script Editor 應用程式路由通知。如果 Script Editor 沒有通知權限，命令會無聲地失敗，macOS 不會提示您授予它。在終端中執行一次以使 Script Editor 出現在您的通知設定中：

      ```bash theme={null}
      osascript -e 'display notification "test"'
      ```

      目前不會出現任何內容。開啟**系統設定 > 通知**，在列表中找到 **Script Editor**，並開啟**允許通知**。再次執行命令以確認測試通知出現。
    </Accordion>
  </Tab>

  <Tab title="Linux">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "notify-send 'Claude Code' 'Claude Code needs your attention'"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="Windows (PowerShell)">
    ```json theme={null}
    {
      "hooks": {
        "Notification": [
          {
            "matcher": "",
            "hooks": [
              {
                "type": "command",
                "command": "powershell.exe -Command \"[System.Reflection.Assembly]::LoadWithPartialName('System.Windows.Forms'); [System.Windows.Forms.MessageBox]::Show('Claude Code needs your attention', 'Claude Code')\""
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>
</Tabs>

空的 `matcher` 會在所有通知類型上觸發。若要僅在特定事件上觸發，請將其設定為以下其中一個值：

| Matcher                | 觸發時機                                                    |
| :--------------------- | :------------------------------------------------------ |
| `permission_prompt`    | Claude 需要您批准工具使用                                        |
| `idle_prompt`          | Claude 完成並等待您的下一個提示                                     |
| `auth_success`         | 驗證完成                                                    |
| `elicitation_dialog`   | MCP 伺服器開啟引導表單                                           |
| `elicitation_complete` | MCP 引導表單被提交或關閉                                          |
| `elicitation_response` | MCP 引導回應被發送回伺服器                                         |
| `agent_needs_input`    | 背景工作階段開始等待您的輸入。僅在 [agent view](/docs/zh-TW/agent-view) 開啟時觸發 |
| `agent_completed`      | 背景工作階段完成或失敗。僅在 [agent view](/docs/zh-TW/agent-view) 開啟時觸發    |

`agent_needs_input` 和 `agent_completed` 匹配器需要 Claude Code v2.1.198 或更新版本。

輸入 `/hooks` 並選擇 `Notification` 以確認 hook 已註冊。有關完整的事件架構，請參閱 [Notification 參考](/docs/zh-TW/hooks#notification)。

<h3 id="auto-format-code-after-edits">
  編輯後自動格式化程式碼
</h3>

在 Claude 編輯的每個檔案上自動執行 [Prettier](https://prettier.io/)，以便格式保持一致而無需手動干預。

此 hook 使用 `PostToolUse` 事件搭配 `Edit|Write` 匹配器，因此它只在檔案編輯工具之後執行。該命令使用 [`jq`](https://jqlang.github.io/jq/) 提取編輯的檔案路徑並將其傳遞給 Prettier。將此新增到您的專案根目錄中的 `.claude/settings.json`：

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r '.tool_input.file_path' | xargs npx prettier --write"
          }
        ]
      }
    ]
  }
}
```

在 Claude Code v2.1.191 或更新版本上，您也可以將匹配器寫成 `Edit,Write`，因為在這些版本上 `|` 和 `,` 是工具名稱匹配器的可互換清單分隔符。

<Note>
  本頁上的 Bash 範例使用 `jq` 進行 JSON 解析。使用 `brew install jq`（macOS）、`apt-get install jq`（Debian 和 Ubuntu）安裝它，或參閱 [`jq` 下載](https://jqlang.github.io/jq/download/)。
</Note>

<h3 id="block-edits-to-protected-files">
  阻止編輯受保護的檔案
</h3>

防止 Claude 修改敏感檔案，如 `.env`、`package-lock.json` 或 `.git/` 中的任何內容。Claude 會收到解釋編輯被阻止原因的回饋，因此它可以調整其方法。

此範例使用 hook 呼叫的單獨指令檔。該指令檢查目標檔案路徑是否與受保護的模式列表相符，並以代碼 2 退出以阻止編輯。

<Steps>
  <Step title="建立 hook 指令">
    將此儲存到 `.claude/hooks/protect-files.sh`：

    ```bash theme={null}
    #!/bin/bash
    # protect-files.sh

    INPUT=$(cat)
    FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

    PROTECTED_PATTERNS=(".env" "package-lock.json" ".git/")

    for pattern in "${PROTECTED_PATTERNS[@]}"; do
      if [[ "$FILE_PATH" == *"$pattern"* ]]; then
        echo "Blocked: $FILE_PATH matches protected pattern '$pattern'" >&2
        exit 2
      fi
    done

    exit 0
    ```
  </Step>

  <Step title="在 macOS 和 Linux 上使指令可執行">
    Hook 指令必須可執行，Claude Code 才能執行它們：

    ```bash theme={null}
    chmod +x .claude/hooks/protect-files.sh
    ```
  </Step>

  <Step title="註冊 hook">
    將 `PreToolUse` hook 新增到 `.claude/settings.json`，在任何 `Edit` 或 `Write` 工具呼叫之前執行指令：

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [
              {
                "type": "command",
                "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/protect-files.sh"
              }
            ]
          }
        ]
      }
    }
    ```
  </Step>
</Steps>

<h3 id="re-inject-context-after-compaction">
  壓縮後重新注入上下文
</h3>

當 Claude 的上下文視窗填滿時，壓縮會總結對話以釋放空間。這可能會遺失重要細節。使用帶有 `compact` 匹配器的 `SessionStart` hook 在每次壓縮後重新注入關鍵上下文。

您的命令寫入 stdout 的任何文字都會新增到 Claude 的上下文中。此範例提醒 Claude 專案慣例和最近的工作。將此新增到您的專案根目錄中的 `.claude/settings.json`：

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "compact",
        "hooks": [
          {
            "type": "command",
            "command": "echo 'Reminder: use Bun, not npm. Run bun test before committing. Current sprint: auth refactor.'"
          }
        ]
      }
    ]
  }
}
```

您可以將 `echo` 替換為任何產生動態輸出的命令，如 `git log --oneline -5` 以顯示最近的提交。有關在每個工作階段開始時注入上下文，請考慮改用 [CLAUDE.md](/docs/zh-TW/memory)。有關環境變數，請參閱參考中的 [`CLAUDE_ENV_FILE`](/docs/zh-TW/hooks#persist-environment-variables)。

<h3 id="audit-configuration-changes">
  審計配置變更
</h3>

追蹤工作階段期間設定或 skills 檔案何時變更。`ConfigChange` 事件在外部程序或編輯器修改配置檔案時觸發，因此您可以記錄變更以進行合規性檢查或阻止未授權的修改。

此範例將每個變更附加到審計日誌。將此新增到 `~/.claude/settings.json`：

```json theme={null}
{
  "hooks": {
    "ConfigChange": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "jq -c '{timestamp: now | todate, source: .source, file: .file_path}' >> ~/claude-config-audit.log"
          }
        ]
      }
    ]
  }
}
```

匹配器按配置類型篩選：`user_settings`、`project_settings`、`local_settings`、`policy_settings` 或 `skills`。要阻止變更生效，以代碼 2 退出或傳回 `{"decision": "block"}`。有關完整的輸入架構，請參閱 [ConfigChange 參考](/docs/zh-TW/hooks#configchange)。

<h3 id="reload-environment-when-directory-or-files-change">
  當目錄或檔案變更時重新載入環境
</h3>

某些專案根據您所在的目錄設定不同的環境變數。[direnv](https://direnv.net/) 之類的工具在您的 shell 中自動執行此操作，但 Claude 的 Bash 工具不會自行選取這些變更。

配對 `SessionStart` hook 與 `CwdChanged` hook 可以修復此問題。`SessionStart` 載入您啟動時所在目錄的變數，`CwdChanged` 在 Claude 每次變更目錄時重新載入它們。兩者都寫入 `CLAUDE_ENV_FILE`，Claude Code 在每個 Bash 命令之前執行為指令碼前置。將此新增到 `~/.claude/settings.json`：

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ],
    "CwdChanged": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ]
  }
}
```

在每個具有 `.envrc` 的目錄中執行一次 `direnv allow`，以便允許 direnv 載入它。如果您使用 devbox 或 nix 而不是 direnv，相同的模式適用於 `devbox shellenv` 或 `devbox global shellenv` 代替 `direnv export bash`。

若要對特定檔案而不是每次目錄變更做出反應，請使用 `FileChanged` 搭配 `matcher` 列出要監視的檔案名稱（以 `|` 分隔）。建立監視清單時，Claude Code 會將此值分割為字面檔案名稱，而不是作為正規表達式進行評估。有關輸入架構、`watchPaths` 輸出和 `CLAUDE_ENV_FILE` 詳細資訊，請參閱 [FileChanged](/docs/zh-TW/hooks#filechanged)。此範例監視工作目錄中 `.envrc` 和 `.env` 的變更：

```json theme={null}
{
  "hooks": {
    "FileChanged": [
      {
        "matcher": ".envrc|.env",
        "hooks": [
          {
            "type": "command",
            "command": "direnv export bash > \"$CLAUDE_ENV_FILE\""
          }
        ]
      }
    ]
  }
}
```

有關輸入架構、`watchPaths` 輸出和 `CLAUDE_ENV_FILE` 詳細資訊，請參閱 [CwdChanged](/docs/zh-TW/hooks#cwdchanged) 和 [FileChanged](/docs/zh-TW/hooks#filechanged) 參考項目。

<h3 id="auto-approve-specific-permission-prompts">
  自動批准特定權限提示
</h3>

跳過您始終允許的工具呼叫的批准對話。此範例自動批准 `ExitPlanMode`，這是 Claude 在完成呈現計畫並要求繼續時呼叫的工具，因此您不會在每次計畫準備好時被提示。

與上面的退出代碼範例不同，自動批准要求您的 hook 將 JSON 決策寫入 stdout。`PermissionRequest` hook 在 Claude Code 即將顯示權限對話時觸發，傳回 `"behavior": "allow"` 會代表您回答它。

匹配器將 hook 的範圍限制為僅 `ExitPlanMode`，因此不會影響其他提示。將此新增到 `~/.claude/settings.json`：

```json theme={null}
{
  "hooks": {
    "PermissionRequest": [
      {
        "matcher": "ExitPlanMode",
        "hooks": [
          {
            "type": "command",
            "command": "echo '{\"hookSpecificOutput\": {\"hookEventName\": \"PermissionRequest\", \"decision\": {\"behavior\": \"allow\"}}}'"
          }
        ]
      }
    ]
  }
}
```

當 hook 批准時，Claude Code 退出計畫模式並恢復進入計畫模式之前處於活動狀態的任何權限模式。文字記錄顯示「由 PermissionRequest hook 允許」，其中對話會出現。hook 路徑始終保持當前對話：它無法清除上下文並以對話可以執行的方式啟動新的實現工作階段。

若要改為設定特定的權限模式，您的 hook 的輸出可以包含帶有 `setMode` 項目的 `updatedPermissions` 陣列。`mode` 值是任何權限模式，如 `default`、`acceptEdits` 或 `bypassPermissions`，`destination: "session"` 僅將其應用於當前工作階段。

<Note>
  `bypassPermissions` 只有在工作階段已經啟用了繞過模式時才適用：`--dangerously-skip-permissions`、`--permission-mode bypassPermissions`、`--allow-dangerously-skip-permissions` 或設定中的 `permissions.defaultMode: "bypassPermissions"`，且未被 [`permissions.disableBypassPermissionsMode`](/docs/zh-TW/permissions#managed-settings) 禁用。它永遠不會被持久化為 `defaultMode`。
</Note>

若要將工作階段切換到 `acceptEdits`，您的 hook 會將此 JSON 寫入 stdout：

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PermissionRequest",
    "decision": {
      "behavior": "allow",
      "updatedPermissions": [
        { "type": "setMode", "mode": "acceptEdits", "destination": "session" }
      ]
    }
  }
}
```

保持匹配器盡可能狹窄。在 `.*` 上進行匹配或留空匹配器會自動批准每個權限提示，包括檔案寫入和 shell 命令。有關決策欄位的完整集合，請參閱 [PermissionRequest 參考](/docs/zh-TW/hooks#permissionrequest-decision-control)。

<h2 id="how-hooks-work">
  Hooks 如何工作
</h2>

Hook 事件在 Claude Code 的特定生命週期點觸發。當事件觸發時，所有匹配的 hooks 並行執行，相同的 hook 命令會自動去重。下表顯示每個事件及其觸發時間：

| Event                 | When it fires                                                                                                                                                                                                                                         |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SessionStart`        | When a session begins or resumes                                                                                                                                                                                                                      |
| `Setup`               | When you start Claude Code with `--init-only`, or with `--init` or `--maintenance` in `-p` mode. For one-time preparation in CI or scripts                                                                                                            |
| `UserPromptSubmit`    | When you submit a prompt, before Claude processes it                                                                                                                                                                                                  |
| `UserPromptExpansion` | When a user-typed command expands into a prompt, before it reaches Claude. Can block the expansion                                                                                                                                                    |
| `PreToolUse`          | Before a tool call executes. Can block it                                                                                                                                                                                                             |
| `PermissionRequest`   | When a tool call needs a permission decision                                                                                                                                                                                                          |
| `PermissionDenied`    | When auto mode denies a tool call, including denials without a classifier verdict. Use JSON `hookSpecificOutput.retry: true` to tell the model it may retry the denied tool call. Claude Code ignores `retry` when the classifier produced no verdict |
| `PostToolUse`         | After a tool call succeeds                                                                                                                                                                                                                            |
| `PostToolUseFailure`  | After a tool call fails                                                                                                                                                                                                                               |
| `PostToolBatch`       | After a full batch of parallel tool calls resolves, before the next model call                                                                                                                                                                        |
| `Notification`        | When Claude Code sends a notification                                                                                                                                                                                                                 |
| `MessageDisplay`      | While assistant message text is displayed                                                                                                                                                                                                             |
| `SubagentStart`       | When a subagent is spawned                                                                                                                                                                                                                            |
| `SubagentStop`        | When a subagent finishes                                                                                                                                                                                                                              |
| `TaskCreated`         | When a task is being created via `TaskCreate`                                                                                                                                                                                                         |
| `TaskCompleted`       | When a task is being marked as completed                                                                                                                                                                                                              |
| `Stop`                | When Claude finishes responding                                                                                                                                                                                                                       |
| `StopFailure`         | When the turn ends due to an API error                                                                                                                                                                                                                |
| `TeammateIdle`        | When an [agent team](/docs/en/agent-teams) teammate is about to go idle                                                                                                                                                                                    |
| `InstructionsLoaded`  | When a CLAUDE.md or `.claude/rules/*.md` file is loaded into context. Fires at session start and when files are lazily loaded during a session                                                                                                        |
| `ConfigChange`        | When a configuration file changes during a session                                                                                                                                                                                                    |
| `CwdChanged`          | When the working directory changes, for example when Claude executes a `cd` command. Useful for reactive environment management with tools like direnv                                                                                                |
| `DirectoryAdded`      | When a working directory is added mid-session via `/add-dir` or the SDK `register_repo_root` control request                                                                                                                                          |
| `FileChanged`         | When a watched file changes on disk. The `matcher` field specifies which filenames to watch                                                                                                                                                           |
| `WorktreeCreate`      | When a worktree is being created via `--worktree`, `isolation: "worktree"`, or for a background session. Replaces default git behavior                                                                                                                |
| `WorktreeRemove`      | When a worktree is being removed at session exit, when a subagent finishes, or when you delete a background session                                                                                                                                   |
| `PreCompact`          | Before context compaction                                                                                                                                                                                                                             |
| `PostCompact`         | After context compaction completes                                                                                                                                                                                                                    |
| `PreModelSwitch`      | Before Claude Code applies a model switch that you or a client requested. Can block the switch                                                                                                                                                        |
| `PostModelSwitch`     | After the session's model changes, including changes Claude Code makes on its own, such as restoring the model when you resume a session                                                                                                              |
| `Elicitation`         | When an MCP server requests user input during a tool call                                                                                                                                                                                             |
| `ElicitationResult`   | After a user responds to an MCP elicitation, before the response is sent back to the server                                                                                                                                                           |
| `SessionEnd`          | When a session terminates                                                                                                                                                                                                                             |

每個 hook 都有一個 `type` 來決定它如何執行。大多數 hooks 使用 `"type": "command"`，它執行 shell 命令。還有四種其他類型可用：

* `"type": "http"`：POST 事件資料到 URL。請參閱 [HTTP hooks](#http-hooks)。
* `"type": "mcp_tool"`：在已連接的 MCP 伺服器上呼叫工具。請參閱 [MCP tool hooks](/docs/zh-TW/hooks#mcp-tool-hook-fields)。
* `"type": "prompt"`：單輪 LLM 評估。請參閱[基於提示的 hooks](#prompt-based-hooks)。
* `"type": "agent"`：具有工具存取的多輪驗證。Agent hooks 是實驗性的，可能會改變。請參閱[基於 Agent 的 hooks](#agent-based-hooks)。

<h3 id="combine-results-from-multiple-hooks">
  合併來自多個 hooks 的結果
</h3>

當多個 hooks 相符同一事件時，每個 hook 的命令都會執行到完成，然後 Claude Code 合併結果。一個 hook 傳回 `deny` 不會阻止同級 hooks 執行。不要依賴一個 hook 的 `deny` 來抑制另一個 hook 中的副作用。

所有匹配的 hooks 完成後，Claude Code 合併它們的輸出。對於 `PreToolUse` 權限決策，最具限制性的答案獲勝，順序為 `deny`、`defer`、`ask`、`allow`。來自 `additionalContext` 的文字會從每個 hook 保留並一起傳遞給 Claude。

下面的範例在 `Bash` 上註冊了兩個 `PreToolUse` hooks。第一個將每個命令附加到日誌檔案並退出 0。第二個執行一個指令碼，當命令包含 `rm -rf` 時退出 2 以拒絕：

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "jq -r .tool_input.command >> ~/.claude/bash.log"
          },
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/block-rm-rf.sh"
          }
        ]
      }
    ]
  }
}
```

當 Claude 嘗試執行 `rm -rf /tmp/build` 時，兩個 hooks 並行執行。日誌記錄 hook 將命令寫入 `~/.claude/bash.log` 並退出 0，這表示沒有決策。防護欄 hook 退出 2，這拒絕了工具呼叫。拒絕獲勝，所以 Claude Code 阻止命令並向 Claude 顯示防護欄的 stderr。日誌項仍然被寫入，因為日誌記錄 hook 已經執行。

<h3 id="read-input-and-return-output">
  讀取輸入並傳回輸出
</h3>

Hooks 透過 stdin、stdout、stderr 和退出代碼與 Claude Code 通訊。當事件觸發時，Claude Code 將事件特定的資料作為 JSON 傳遞到您的指令的 stdin。您的指令讀取該資料、執行其工作，並透過退出代碼告訴 Claude Code 接下來要做什麼。

<h4 id="hook-input">
  Hook 輸入
</h4>

每個事件都包含常見欄位，如 `session_id` 和 `cwd`，但每個事件類型都新增不同的資料。例如，當 Claude 執行 Bash 命令時，`PreToolUse` hook 在 stdin 上接收類似以下內容：

```json theme={null}
{
  "session_id": "abc123",          // 此工作階段的唯一 ID
  "cwd": "/Users/sarah/myproject", // 事件觸發時的工作目錄
  "hook_event_name": "PreToolUse", // 哪個事件觸發了此 hook
  "tool_name": "Bash",             // Claude 即將使用的工具
  "tool_input": {                  // Claude 傳遞給工具的引數
    "command": "npm test"          // 對於 Bash，這是 shell 命令
  }
}
```

您的指令可以解析該 JSON 並對任何這些欄位採取行動。`UserPromptSubmit` hooks 改為取得 `prompt` 文字，`SessionStart` hooks 取得 `source`（startup、resume、clear、compact），等等。有關共享欄位，請參閱參考中的[常見輸入欄位](/docs/zh-TW/hooks#common-input-fields)，以及每個事件的部分以了解事件特定的架構。

<h4 id="hook-output">
  Hook 輸出
</h4>

您的指令透過寫入 stdout 或 stderr 並以特定代碼退出來告訴 Claude Code 接下來要做什麼。以下 `PreToolUse` hook 阻止命令：

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command')

if echo "$COMMAND" | grep -q "drop table"; then
  echo "Blocked: dropping tables is not allowed" >&2  # stderr 變成 Claude 的回饋
  exit 2 # exit 2 = 阻止操作
fi

exit 0  # exit 0 = 沒有決策；正常的權限流程適用
```

退出代碼決定接下來會發生什麼：

* **Exit 0**：hook 報告沒有異議，操作正常進行。對於 `PreToolUse` hook，這不會批准工具呼叫：正常的[權限流程](/docs/zh-TW/permissions)仍然適用。對於 `UserPromptSubmit`、`UserPromptExpansion` 和 `SessionStart` hooks，您寫入 stdout 的任何內容都會新增到 Claude 的上下文中。
* **Exit 2**：操作被阻止。寫入原因到 stderr，Claude 會收到它作為回饋，以便它可以調整。某些事件無法被阻止：對於 `SessionStart`、`Setup`、`Notification` 和其他事件，exit 2 會向使用者顯示 stderr，執行繼續。有關完整清單，請參閱[每個事件的 exit code 2 行為](/docs/zh-TW/hooks#exit-code-2-behavior-per-event)。
* **任何其他退出代碼**：操作繼續。文字記錄顯示 `<hook name> hook error` 通知，後面跟著 stderr 的第一行；完整的 stderr 進入[除錯日誌](/docs/zh-TW/hooks#debug-hooks)。

<h4 id="structured-json-output">
  結構化 JSON 輸出
</h4>

退出代碼只讓您阻止或保持沉默。為了獲得更多控制，退出 0 並改為將 JSON 物件列印到 stdout。

<Note>
  使用 exit 2 以 stderr 訊息阻止，或使用 exit 0 和 JSON 進行結構化控制。不要混合它們：Claude Code 在您退出 2 時忽略 JSON。
</Note>

例如，`PreToolUse` hook 可以拒絕工具呼叫並告訴 Claude 為什麼，或將其升級給使用者以獲得批准：

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "PreToolUse",
    "permissionDecision": "deny",
    "permissionDecisionReason": "Use rg instead of grep for better performance"
  }
}
```

使用 `"deny"`，Claude Code 會取消工具呼叫並將 `permissionDecisionReason` 回饋給 Claude。這些 `permissionDecision` 值特定於 `PreToolUse`：

* `"allow"`：跳過互動式權限提示。拒絕和詢問規則，包括企業受管拒絕清單，仍然適用，以及您的組織設定為 `ask` 的連接器工具提示和標記為 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具
* `"deny"`：取消工具呼叫並將原因傳送給 Claude
* `"ask"`：照常向使用者顯示權限提示

第四個值 `"defer"` 在[非互動模式](/docs/zh-TW/headless)中使用 `-p` 旗標時可用。它以保留的工具呼叫退出程序，以便 Agent SDK 包裝器可以收集輸入並繼續。有關詳細資訊，請參閱參考中的[延遲工具呼叫以供稍後使用](/docs/zh-TW/hooks#defer-a-tool-call-for-later)。

傳回 `"allow"` 會跳過互動式提示，但不會覆蓋[權限規則](/docs/zh-TW/permissions#manage-permissions)。如果拒絕規則與工具呼叫相符，即使您的 hook 傳回 `"allow"`，呼叫也會被阻止。如果詢問規則相符，使用者仍會被提示，以及您的組織設定為 `ask` 的連接器工具和標記為 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具。這意味著來自任何設定範圍（包括[受管理的設定](/docs/zh-TW/settings#settings-files)）的拒絕規則始終優先於 hook 批准。

其他事件使用不同的決策模式。例如，`PostToolUse` 和 `Stop` hooks 使用頂級 `decision: "block"` 欄位，而 `PermissionRequest` 使用 `hookSpecificOutput.decision.behavior`。有關按事件的完整分解，請參閱參考中的[摘要表](/docs/zh-TW/hooks#decision-control)。

對於 `UserPromptSubmit` hooks，改用 `hookSpecificOutput.additionalContext` 將文字注入到 Claude 的上下文中。將 `additionalContext` 嵌套在 `hookSpecificOutput` 內；如果您將其放在 JSON 的頂級，Claude Code 會無聲地忽略它。例如，此輸出將目前分支狀態新增到每個提示：

```json theme={null}
{
  "hookSpecificOutput": {
    "hookEventName": "UserPromptSubmit",
    "additionalContext": "Current branch: release-42. Deploy freeze until Friday."
  }
}
```

有關完整的輸出形狀（包括阻止提示和設定工作階段標題），請參閱 [UserPromptSubmit 決策控制](/docs/zh-TW/hooks#userpromptsubmit-decision-control)。

具有 `type: "prompt"` 的 Hooks 以不同方式處理輸出：請參閱[基於提示的 hooks](#prompt-based-hooks)。

<h3 id="filter-hooks-with-matchers">
  使用匹配器篩選 hooks
</h3>

沒有匹配器，hook 會在其事件的每次出現時觸發。匹配器讓您縮小範圍。例如，如果您只想在檔案編輯後執行格式化程式（而不是在每次工具呼叫後），請將匹配器新增到您的 `PostToolUse` hook：

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          { "type": "command", "command": "prettier --write ..." }
        ]
      }
    ]
  }
}
```

`"Edit|Write"` 匹配器只在 Claude 使用 `Edit` 或 `Write` 工具時觸發，而不是在它使用 `Bash`、`Read` 或任何其他工具時。在 Claude Code v2.1.191 或更新版本上，逗號以相同方式分隔替代項，因此 `"Edit, Write"` 是等效的。請參閱[匹配器模式](/docs/zh-TW/hooks#matcher-patterns)以了解純名稱和正規表達式如何被評估。

<Note>
  Claude 也可以透過 `Bash` 工具執行 shell 命令來建立或修改檔案。如果您的 hook 必須看到每個檔案變更，例如用於合規掃描或稽核日誌，請新增一個[`Stop`](/docs/zh-TW/hooks#stop) hook，它每輪掃描一次工作樹。為了獲得每次呼叫的覆蓋範圍，也請匹配 `Bash` 並讓您的指令使用 `git status --porcelain` 列出修改和未追蹤的檔案。
</Note>

每個事件類型都在特定欄位上進行匹配：

| 事件                                                                                                                                                     | 匹配器篩選的內容                                               | 範例匹配器值                                                                                                                                                                     |
| :----------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PreToolUse`、`PostToolUse`、`PostToolUseFailure`、`PermissionRequest`、`PermissionDenied`                                                                 | 工具名稱                                                   | `Bash`、`Edit\|Write`、`mcp__.*`                                                                                                                                             |
| `SessionStart`                                                                                                                                         | 工作階段如何開始                                               | `startup`、`resume`、`clear`、`compact`                                                                                                                                       |
| `Setup`                                                                                                                                                | 哪個 CLI 旗標觸發了設定                                         | `init`、`maintenance`                                                                                                                                                       |
| `SessionEnd`                                                                                                                                           | 工作階段為什麼結束                                              | `clear`、`resume`、`logout`、`prompt_input_exit`、`bypass_permissions_disabled`、`other`                                                                                        |
| `Notification`                                                                                                                                         | 通知類型                                                   | `permission_prompt`、`idle_prompt`、`auth_success`、`elicitation_dialog`、`elicitation_complete`、`elicitation_response`、`agent_needs_input`、`agent_completed`                  |
| `SubagentStart`                                                                                                                                        | Agent 類型                                               | `general-purpose`、`Explore`、`Plan` 或自訂 Agent 名稱                                                                                                                            |
| `PreCompact`、`PostCompact`                                                                                                                             | 什麼觸發了壓縮                                                | `manual`、`auto`                                                                                                                                                            |
| `SubagentStop`                                                                                                                                         | Agent 類型                                               | 與 `SubagentStart` 相同的值                                                                                                                                                     |
| `ConfigChange`                                                                                                                                         | 配置來源                                                   | `user_settings`、`project_settings`、`local_settings`、`policy_settings`、`skills`                                                                                             |
| `StopFailure`                                                                                                                                          | 錯誤類型                                                   | `rate_limit`、`overloaded`、`authentication_failed`、`oauth_org_not_allowed`、`billing_error`、`invalid_request`、`model_not_found`、`server_error`、`max_output_tokens`、`unknown` |
| `InstructionsLoaded`                                                                                                                                   | 載入原因                                                   | `session_start`、`nested_traversal`、`path_glob_match`、`include`、`compact`                                                                                                   |
| `Elicitation`                                                                                                                                          | MCP 伺服器名稱                                              | 您配置的 MCP 伺服器名稱                                                                                                                                                             |
| `ElicitationResult`                                                                                                                                    | MCP 伺服器名稱                                              | 與 `Elicitation` 相同的值                                                                                                                                                       |
| `FileChanged`                                                                                                                                          | 字面檔案名稱以監視（請參閱 [FileChanged](/docs/zh-TW/hooks#filechanged)） | `.envrc\|.env`                                                                                                                                                             |
| `UserPromptExpansion`                                                                                                                                  | 命令名稱                                                   | 您的 skill 或命令名稱                                                                                                                                                             |
| `UserPromptSubmit`、`PostToolBatch`、`Stop`、`TeammateIdle`、`TaskCreated`、`TaskCompleted`、`WorktreeCreate`、`WorktreeRemove`、`CwdChanged`、`MessageDisplay` | 不支援匹配器                                                 | 始終在每次出現時觸發                                                                                                                                                                 |

下面的標籤頁顯示不同事件類型上匹配器的更多範例。

<Tabs>
  <Tab title="記錄每個 Bash 命令">
    只匹配 `Bash` 工具呼叫並將每個命令記錄到檔案。`PostToolUse` 事件在命令完成後觸發，因此 `tool_input.command` 包含執行的內容。hook 在 stdin 上接收事件資料作為 JSON，`jq -r '.tool_input.command'` 只提取命令字串，`>>` 將其附加到日誌檔案：

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "jq -r '.tool_input.command' >> ~/.claude/command-log.txt"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="匹配 MCP 工具">
    MCP 工具使用與內建工具不同的命名慣例：`mcp__<server>__<tool>`，其中 `<server>` 是 MCP 伺服器名稱，`<tool>` 是它提供的工具。例如，`mcp__github__search_repositories` 或 `mcp__filesystem__read_file`。來自[外掛提供的 MCP 伺服器](/docs/zh-TW/mcp#plugin-provided-mcp-servers)的工具使用範圍伺服器段，例如 `mcp__plugin_my-plugin_db__query`。使用正規表達式匹配器來針對來自特定伺服器的所有工具，或使用 `mcp__.*__write.*` 之類的模式跨伺服器進行匹配。有關完整的範例列表，請參閱參考中的[匹配 MCP 工具](/docs/zh-TW/hooks#match-mcp-tools)。

    下面的命令使用 `jq` 從 hook 的 JSON 輸入中提取工具名稱，並將其寫入 stderr。寫入 stderr 會保持 stdout 乾淨以用於 JSON 輸出，並將訊息發送到[除錯日誌](/docs/zh-TW/hooks#debug-hooks)：

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "mcp__github__.*",
            "hooks": [
              {
                "type": "command",
                "command": "echo \"GitHub tool called: $(jq -r '.tool_name')\" >&2"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="在工作階段結束時清理">
    `SessionEnd` 事件支援工作階段結束原因的匹配器。此 hook 只在 `clear` 時觸發（當您執行 `/clear` 時），而不是在正常退出時：

    ```json theme={null}
    {
      "hooks": {
        "SessionEnd": [
          {
            "matcher": "clear",
            "hooks": [
              {
                "type": "command",
                "command": "rm -f /tmp/claude-scratch-*.txt"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>
</Tabs>

有關完整的匹配器語法，請參閱 [Hooks 參考](/docs/zh-TW/hooks#configuration)。

<h4 id="filter-by-tool-name-and-arguments-with-the-if-field">
  使用 `if` 欄位按工具名稱和引數篩選
</h4>

`if` 欄位使用[權限規則語法](/docs/zh-TW/permissions)按工具名稱和引數一起篩選 hooks，因此 hook 程序只在工具呼叫相符時生成。這超越了 `matcher`，它只在工具名稱級別篩選。

例如，這個配置只在 Claude 使用 `git` 命令而不是所有 Bash 命令時執行 hook：

```json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "if": "Bash(git *)",
            "command": "\"$CLAUDE_PROJECT_DIR\"/.claude/hooks/check-git-policy.sh"
          }
        ]
      }
    ]
  }
}
```

您的 hook 命令是否執行取決於您的 `if` 模式的形狀和 Claude 正在呼叫的 Bash 命令：

| `if` 模式            | Bash 命令                | Hook 執行？ | 為什麼                                     |
| :----------------- | :--------------------- | :------- | :-------------------------------------- |
| `Bash(git *)`      | `git push`             | 是        | 命令名稱相符                                  |
| `Bash(git *)`      | `npm test && git push` | 是        | 每個子命令都被檢查；`git push` 相符                 |
| `Bash(git *)`      | `echo $(git log)`      | 是        | `$()` 和反引號內的命令被檢查；`git log` 相符          |
| `Bash(git *)`      | `echo $(date)`         | 否        | 沒有子命令相符 `git *`                         |
| `Bash(git push *)` | `echo $(date)`         | 是        | 指定超過命令名稱的模式在 `$()`、反引號或 `$VAR` 上執行 hook |

篩選器也會失敗開放，當 Bash 命令無法解析時執行您的 hook。因為篩選器是盡力而為，請使用[權限系統](/docs/zh-TW/permissions)而不是 hook 來強制執行硬允許或拒絕。

`if` 欄位接受與權限規則相同的模式：`"Bash(git *)"`、`"Edit(*.ts)"` 等。若要匹配多個工具名稱，請使用每個都有自己的 `if` 值的單獨處理程式，或在 `matcher` 級別進行匹配，其中支援管道交替。

`if` 只適用於工具事件：`PreToolUse`、`PostToolUse`、`PostToolUseFailure`、`PermissionRequest` 和 `PermissionDenied`。將其新增到任何其他事件會防止 hook 執行。

<h3 id="configure-hook-location">
  配置 hook 位置
</h3>

您新增 hook 的位置決定了其範圍：

| 位置                                                             | 範圍                      | 可共享                             |
| :------------------------------------------------------------- | :---------------------- | :------------------------------ |
| `~/.claude/settings.json`                                      | 您的所有專案                  | 否，本機到您的機器                       |
| `.claude/settings.json`                                        | 單個專案                    | 是，可以提交到儲存庫                      |
| `.claude/settings.local.json`                                  | 單個專案                    | 否，gitignored 當 Claude Code 建立它時 |
| 受管理的原則設定                                                       | 組織範圍                    | 是，由管理員控制                        |
| [Plugin](/docs/zh-TW/plugins) `hooks/hooks.json`                    | 啟用外掛時                   | 是，與外掛捆綁                         |
| [Skill](/docs/zh-TW/skills) 或[agent](/docs/zh-TW/sub-agents) frontmatter | 當 skill 或 agent 處於活動狀態時 | 是，在元件檔案中定義                      |

在 Claude Code 中執行 [`/hooks`](/docs/zh-TW/hooks#the-%2Fhooks-menu) 以瀏覽按事件分組的所有配置的 hooks。

若要禁用 hooks，請在設定檔中設定 `"disableAllHooks": true`。受管理的原則設定中配置的 Hooks 仍會執行，除非 `disableAllHooks` 也在那裡設定。

如果您在 Claude Code 執行時直接編輯設定檔，檔案監視程式通常會自動選取 hook 變更。

<h2 id="prompt-based-hooks">
  基於提示的 hooks
</h2>

對於需要判斷而不是確定性規則的決策，使用 `type: "prompt"` hooks。Claude Code 不執行 shell 命令，而是將您的提示和 hook 的輸入資料傳送到 Claude 模型（預設為 Haiku）以做出決策。如果您需要更多功能，可以使用 `model` 欄位指定不同的模型。

模型的唯一工作是傳回 yes/no 決策作為 JSON：

* `"ok": true`：操作繼續
* `"ok": false`：發生的情況取決於事件：
  * `Stop` 和 `SubagentStop`：`reason` 被回饋給 Claude，以便它繼續工作
  * `PreToolUse`：工具呼叫被拒絕，`reason` 作為工具錯誤傳回給 Claude，以便它可以調整並繼續
  * `PostToolUse`、`PostToolBatch`、`UserPromptSubmit` 和 `UserPromptExpansion`：回合結束，`reason` 在聊天中顯示為警告行

此範例使用 `Stop` hook 詢問模型是否所有請求的任務都已完成。如果模型傳回 `"ok": false`，Claude 會繼續工作並使用 `reason` 作為其下一個指令：

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "prompt",
            "prompt": "Check if all tasks are complete. If not, respond with {\"ok\": false, \"reason\": \"what remains to be done\"}."
          }
        ]
      }
    ]
  }
}
```

有關完整的配置選項，請參閱參考中的[基於提示的 hooks](/docs/zh-TW/hooks#prompt-based-hooks)。

<h2 id="agent-based-hooks">
  基於代理的 hooks
</h2>

<Warning>
  Agent hooks 是實驗性的。行為和配置可能在未來版本中改變。對於生產工作流程，優先使用[命令 hooks](/docs/zh-TW/hooks#command-hook-fields)。
</Warning>

當驗證需要檢查檔案或執行命令時，使用 `type: "agent"` hooks。與只進行單個 LLM 呼叫的提示 hooks 不同，代理 hooks 生成一個 subagent，可以讀取檔案、搜尋程式碼和使用其他工具在傳回決策之前驗證條件。

代理 hooks 使用與提示 hooks 相同的 `"ok"` / `"reason"` 回應格式，但預設超時時間更長（60 秒）且最多 50 個工具使用輪次。

此範例驗證在允許 Claude 停止之前測試通過：

```json theme={null}
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "agent",
            "prompt": "Verify that all unit tests pass. Run the test suite and check the results. $ARGUMENTS",
            "timeout": 120
          }
        ]
      }
    ]
  }
}
```

當 hook 輸入資料本身足以做出決策時，使用提示 hooks。當您需要根據程式碼庫的實際狀態驗證某些內容時，使用代理 hooks。

有關完整的配置選項，請參閱參考中的[基於代理的 hooks](/docs/zh-TW/hooks#agent-based-hooks)。

<h2 id="http-hooks">
  HTTP hooks
</h2>

使用 `type: "http"` hooks 將事件資料 POST 到 HTTP 端點，而不是執行 shell 命令。端點接收命令 hook 在 stdin 上接收的相同 JSON，並使用相同的 JSON 格式透過 HTTP 回應主體傳回結果。

HTTP hooks 在您希望 Web 伺服器、雲端函數或外部服務處理 hook 邏輯時很有用：例如，一個共享的審計服務，在整個團隊中記錄工具使用事件。

此範例將每個工具使用 POST 到本機記錄服務：

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "http",
            "url": "http://localhost:8080/hooks/tool-use",
            "headers": {
              "Authorization": "Bearer $MY_TOKEN"
            },
            "allowedEnvVars": ["MY_TOKEN"]
          }
        ]
      }
    ]
  }
}
```

端點應使用與命令 hooks 相同的[輸出格式](/docs/zh-TW/hooks#json-output)傳回 JSON 回應主體。要阻止工具呼叫，傳回 2xx 回應並包含適當的 `hookSpecificOutput` 欄位。HTTP 狀態代碼本身無法阻止操作。

標頭值支援使用 `$VAR_NAME` 或 `${VAR_NAME}` 語法的環境變數插值。只有在 `allowedEnvVars` 陣列中列出的變數才會被解析；所有其他 `$VAR` 參考保持為空。

有關完整的配置選項和回應處理，請參閱參考中的 [HTTP hooks](/docs/zh-TW/hooks#http-hook-fields)。

<h2 id="limitations-and-troubleshooting">
  限制和故障排除
</h2>

<h3 id="limitations">
  限制
</h3>

設計 hooks 時請記住這些限制：

* 命令 hooks 只透過 stdout、stderr 和退出代碼通訊。它們無法觸發 `/` 命令或工具呼叫。透過 `additionalContext` 傳回的文字會作為系統提醒注入，Claude 將其讀取為純文字。HTTP hooks 改為透過回應主體通訊。
* Hook 超時因類型而異。透過 `timeout` 欄位（以秒為單位）按 hook 覆寫。
  * `command`、`http`、`mcp_tool`：10 分鐘。`UserPromptSubmit` 將這些降低至 30 秒，`MessageDisplay` 將這些降低至 10 秒。
  * `prompt`：30 秒。
  * `agent`：60 秒。
* `PostToolUse` hooks 無法撤銷操作，因為工具已經執行。
* `PermissionRequest` hooks 在[非互動模式](/docs/zh-TW/headless)（`-p` 旗標）中不觸發。對於自動化權限決策，使用 `PreToolUse` hooks。
* `Stop` hooks 在 Claude 完成回應時觸發，而不僅在任務完成時。它們在使用者中斷時不觸發。API 錯誤觸發 [StopFailure](/docs/zh-TW/hooks#stopfailure) 代替。
* 當多個 `PreToolUse` hooks 傳回 [`updatedInput`](/docs/zh-TW/hooks#pretooluse) 以重寫工具的引數時，最後完成的會獲勝。由於 hooks 並行執行，順序是非確定性的。避免有多個 hook 修改同一工具的輸入。

<h3 id="hooks-and-permission-modes">
  Hooks 和權限模式
</h3>

`PreToolUse` hooks 在任何權限模式檢查之前觸發。傳回 `permissionDecision: "deny"` 的 hook 會阻止工具，即使在 `bypassPermissions` 模式或使用 `--dangerously-skip-permissions`。這讓您強制執行使用者無法透過變更其權限模式來繞過的原則。

反面不成立：傳回 `"allow"` 的 hook 不會繞過來自設定的拒絕規則，它也無法抑制您的組織設定為 `ask` 的連接器工具的提示或標記為 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具。Hooks 可以加強限制，但不能放寬超過權限規則允許的限制。

<h3 id="hook-not-firing">
  Hook 未觸發
</h3>

Hook 已配置但從不執行。

* 執行 `/hooks` 並確認 hook 出現在正確的事件下
* 檢查匹配器模式是否與工具名稱完全相符。匹配器區分大小寫
* 驗證您觸發的是正確的事件類型：`PreToolUse` 在工具執行前觸發，`PostToolUse` 在之後觸發
* 如果在非互動模式（`-p` 旗標）中使用 `PermissionRequest` hooks，改用 `PreToolUse`

<h3 id="hook-error-in-output">
  Hook 輸出中的錯誤
</h3>

您在文字記錄中看到類似「PreToolUse hook error: ...」的訊息。

* 您的指令意外以非零代碼退出。透過管道傳輸範例 JSON 來手動測試它：
  ```bash theme={null}
  echo '{"tool_name":"Bash","tool_input":{"command":"ls"}}' | ./my-hook.sh
  echo $?  # 檢查退出代碼
  ```
* 如果您看到「command not found」，使用絕對路徑或 `${CLAUDE_PROJECT_DIR}` 來參考指令。為了完全避免 shell 引用，添加 `"args": []` 以切換到 [exec 形式](/docs/zh-TW/hooks#exec-form-and-shell-form)，它直接生成指令而不使用 shell
* 如果您看到「jq: command not found」，安裝 `jq` 或使用 Python/Node.js 進行 JSON 解析
* 如果指令根本沒有執行，使其可執行：`chmod +x ./my-hook.sh`

<h3 id="/hooks-shows-no-hooks-configured">
  `/hooks` 顯示未配置任何 hooks
</h3>

您編輯了設定檔但 hooks 未出現在選單中。

* 檔案編輯通常會自動選取。如果在幾秒鐘後仍未出現，檔案監視程式可能已錯過變更：重新啟動您的工作階段以強制重新載入。
* 驗證您的 JSON 有效：不允許尾隨逗號和註解
* 確認設定檔在正確的位置：`.claude/settings.json` 用於專案 hooks，`~/.claude/settings.json` 用於全域 hooks

<h3 id="stop-hook-hits-the-block-cap">
  Stop hook 觸發區塊上限
</h3>

Claude 繼續工作而不是停止，然後以警告結束回合，表示 Stop hook 連續阻止了太多次。

Claude Code 在 Stop hook 連續阻止 8 次而沒有進展後會覆寫它。您的 hook 指令需要檢查它是否已經觸發了延續。從 JSON 輸入解析 `stop_hook_active` 欄位，如果為 `true` 則提前退出：

```bash theme={null}
#!/bin/bash
INPUT=$(cat)
if [ "$(echo "$INPUT" | jq -r '.stop_hook_active')" = "true" ]; then
  exit 0  # 允許 Claude 停止
fi
# ... 您的 hook 邏輯的其餘部分
```

如果您的 hook 合理地需要超過八次迭代才能收斂，使用 [`CLAUDE_CODE_STOP_HOOK_BLOCK_CAP`](/docs/zh-TW/env-vars) 提高上限。

<h3 id="json-validation-failed">
  JSON 驗證失敗
</h3>

Claude Code 顯示 JSON 解析錯誤，即使您的 hook 指令輸出有效的 JSON。

當 Claude Code 執行 shell 形式的命令 hook（沒有 `args` 的）時，它在 macOS 和 Linux 上生成 `sh -c`，或在 Windows 上生成 Git Bash。此 shell 是非互動式的，但 Git Bash 和某些配置（例如 `BASH_ENV` 指向 `~/.bashrc`）仍然會來源您的設定檔。如果該設定檔包含無條件的 `echo` 陳述式，輸出會被前置到您的 hook 的 JSON：

```text theme={null}
Shell ready on arm64
{"decision": "block", "reason": "Not allowed"}
```

Claude Code 嘗試將其解析為 JSON 並失敗。要修復此問題，在您的 shell 設定檔中包裝 echo 陳述式，使其只在互動式 shell 中執行：

```bash theme={null}
# 在 ~/.zshrc 或 ~/.bashrc 中
if [[ $- == *i* ]]; then
  echo "Shell ready"
fi
```

`$-` 變數包含 shell 旗標，`i` 表示互動式。Hooks 在非互動式 shell 中執行，因此 echo 被跳過。

<h3 id="debug-techniques">
  除錯技術
</h3>

文字記錄檢視（使用 `Ctrl+O` 切換）為每個觸發的 hook 顯示一行摘要：成功是無聲的，阻止錯誤顯示 stderr，非阻止錯誤顯示 `<hook name> hook error` 通知，後面跟著 stderr 的第一行。

有關完整的執行詳細資訊，包括哪些 hooks 相符、它們的退出代碼、stdout 和 stderr，請閱讀除錯日誌。使用 `claude --debug-file /tmp/claude.log` 啟動 Claude Code 以寫入已知路徑，然後在另一個終端中執行 `tail -f /tmp/claude.log`。如果您啟動時沒有該旗標，在工作階段中執行 `/debug` 以啟用記錄並找到日誌路徑。

<h2 id="learn-more">
  深入瞭解
</h2>

* [Hooks 參考](/docs/zh-TW/hooks)：完整的事件架構、JSON 輸出格式、非同步 hooks 和 MCP 工具 hooks
* [安全考量](/docs/zh-TW/hooks#security-considerations)：在共享或生產環境中部署 hooks 之前進行檢查
* [Bash 命令驗證器範例](https://github.com/anthropics/claude-code/blob/main/examples/hooks/bash_command_validator_example.py)：完整的參考實現
