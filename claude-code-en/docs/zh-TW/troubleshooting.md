> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 故障排除

> 修復 Claude Code 中的高 CPU 或記憶體使用、掛起、auto-compact 抖動和搜尋問題，並找到其他問題的正確頁面。

本頁涵蓋 Claude Code 執行後的效能、穩定性和搜尋問題。如需其他問題，請從符合您遇到問題位置的頁面開始：

| 症狀                                                                                                        | 前往                                                                        |
| :-------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------ |
| `command not found`、安裝失敗、PATH 問題、`EACCES`、TLS 錯誤                                                          | [故障排除安裝和登入](/docs/zh-TW/troubleshoot-install)                                  |
| 更新或安裝下載失敗，出現 `The connection dropped while downloading the update` 或 `aborted`                            | [錯誤參考](/docs/zh-TW/errors#the-connection-dropped-while-downloading-the-update) |
| 登入迴圈、OAuth 錯誤、`403 Forbidden`、「組織已停用」、Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 認證 | [故障排除安裝和登入](/docs/zh-TW/troubleshoot-install#login-and-authentication)         |
| 設定未套用、hooks 未觸發、MCP 伺服器未載入                                                                                | [偵錯您的設定](/docs/zh-TW/debug-your-config)                                        |
| `API Error: 5xx`、`529 Overloaded`、`429`、請求驗證錯誤                                                            | [錯誤參考](/docs/zh-TW/errors)                                                     |
| `model not found` 或 `you may not have access to it`                                                       | [錯誤參考](/docs/zh-TW/errors#theres-an-issue-with-the-selected-model)             |
| VS Code 擴充功能未連接或未偵測到 Claude                                                                               | [VS Code 整合](/docs/zh-TW/vs-code#fix-common-issues)                            |
| JetBrains 外掛程式或 IDE 未偵測到                                                                                  | [JetBrains 整合](/docs/zh-TW/jetbrains#troubleshooting)                          |
| 高 CPU 或記憶體、回應緩慢、掛起、搜尋找不到檔案                                                                                | [效能和穩定性](#performance-and-stability)下方                                    |

如果您不確定哪個適用，請在 Claude Code 內執行 `/doctor` 以自動檢查您的安裝、設定、擴充功能和上下文使用情況；它會提議可以在您確認後套用的修復。如果 `claude` 根本無法啟動，請改為從您的 shell 執行 `claude doctor`。執行 `/mcp` 以檢查 MCP 伺服器狀態。

<h2 id="performance-and-stability">
  效能和穩定性
</h2>

這些部分涵蓋與資源使用、回應性和搜尋行為相關的問題。

<h3 id="high-cpu-or-memory-usage">
  高 CPU 或記憶體使用
</h3>

Claude Code 設計用於與大多數開發環境配合使用，但在處理大型程式碼庫時可能消耗大量資源。如果您遇到效能問題：

1. 定期使用 `/compact` 減少上下文大小
2. 在主要任務之間關閉並重新啟動 Claude Code
3. 考慮將大型構建目錄新增到您的 `.gitignore` 檔案
4. 使用 [`claude --safe-mode`](/docs/zh-TW/cli-reference#cli-flags) 重新啟動以檢查外掛程式、MCP 伺服器或 hook 是否為來源。它會停用該工作階段的所有自訂；如果使用量下降，請參閱[偵錯您的設定](/docs/zh-TW/debug-your-config#test-against-a-clean-configuration)以找出是哪一個

如果在這些步驟後記憶體使用仍然很高，請執行 `/heapdump` 以將 JavaScript 堆快照和記憶體分解寫入 `~/Desktop`。在沒有 Desktop 資料夾的 Linux 上，檔案會寫入您的主目錄。

分解會顯示常駐集合大小、JS 堆積、陣列緩衝區和未計算的原生記憶體，這有助於識別成長是在 JavaScript 物件還是在原生程式碼中。若要檢查保留者，請在 Chrome DevTools 中的 Memory → Load 下開啟 `.heapsnapshot` 檔案；分解是以 `-diagnostics.json` 結尾的檔案。

<Warning>
  `.heapsnapshot` 檔案包含程序中的每個字串。請勿將其附加到公開問題或分享。在 [GitHub](https://github.com/anthropics/claude-code/issues) 上報告記憶體問題時，只附加 `-diagnostics.json` 檔案。該檔案包含記憶體統計資訊，不包含任何對話內容或認證。
</Warning>

<h3 id="large-tables-are-cut-off-in-the-terminal">
  大型表格在終端中被截斷
</h3>

超過 200 列的 Markdown 表格會呈現其前 200 列，後面跟著 `… N more rows not shown` 行。只有顯示受到限制：完整表格保留在對話中，[`/copy`](/docs/zh-TW/commands) 會複製每一列。對於在終端中太大而無法閱讀的表格，請要求 Claude 改為將其寫入檔案。在 v2.1.208 之前，Claude Code 呈現每一列，因此繼續包含非常大型表格的工作階段可能會在重新呈現時停滯。

<h3 id="auto-compaction-stops-with-a-thrashing-error">
  Auto-compaction 停止並出現 thrashing 錯誤
</h3>

如果您看到 `Autocompact is thrashing: the context refilled to the limit...`，自動 compaction 成功，但檔案或工具輸出立即多次重新填充上下文視窗。Claude Code 停止重試以避免在沒有進展的迴圈上浪費 API 呼叫。

若要復原：

1. 要求 Claude 以較小的塊讀取超大檔案，例如特定行範圍或函式，而不是整個檔案
2. 執行 `/compact` 並關注丟棄大輸出，例如 `/compact keep only the plan and the diff`
3. 將大檔案工作移動到 [subagent](/docs/zh-TW/sub-agents)，以便它在單獨的上下文視窗中執行
4. 如果早期對話不再需要，執行 `/clear`

<h3 id="command-hangs-or-freezes">
  命令掛起或凍結
</h3>

如果 Claude Code 似乎無回應：

1. 按 Ctrl+C 嘗試取消目前操作
2. 如果無回應，您可能需要關閉終端並重新啟動

重新啟動不會遺失您的對話。在同一目錄中執行 `claude --resume` 以繼續會話。

<h3 id="garbled-or-corrupted-text-in-an-editor’s-integrated-terminal">
  編輯器整合終端中的文字亂碼或損毀
</h3>

如果在 VS Code、Cursor 或 Devin Desktop 整合終端中執行 Claude Code 時字元呈現為方塊、塗抹或錯誤的字形，終端的 GPU 渲染器可能是原因。在 Claude Code 中執行 `/terminal-setup` 以將 `terminal.integrated.gpuAcceleration` 設定為 `"off"`，或在您的編輯器設定中手動設定並重新載入視窗。請參閱[終端配置](/docs/zh-TW/terminal-config)以了解 `/terminal-setup` 寫入的其他設定。

<h3 id="search-and-discovery-issues">
  搜尋和發現問題
</h3>

如果搜尋工具、`@file` 提及、自訂代理或自訂 skills 找不到檔案，捆綁的 `ripgrep` 二進位檔可能無法在您的系統上執行。安裝您平台的 `ripgrep` 套件並告訴 Claude Code 改用它：

<Tabs>
  <Tab title="macOS">
    ```bash theme={null}
    brew install ripgrep
    ```
  </Tab>

  <Tab title="Ubuntu/Debian">
    ```bash theme={null}
    sudo apt install ripgrep
    ```
  </Tab>

  <Tab title="Alpine">
    ```bash theme={null}
    apk add ripgrep
    ```
  </Tab>

  <Tab title="Arch">
    ```bash theme={null}
    pacman -S ripgrep
    ```
  </Tab>

  <Tab title="Windows">
    ```powershell theme={null}
    winget install BurntSushi.ripgrep.MSVC
    ```
  </Tab>
</Tabs>

然後在您的[環境](/docs/zh-TW/env-vars)中設定 `USE_BUILTIN_RIPGREP=0`。

<h3 id="slow-or-incomplete-search-results-on-wsl">
  WSL 上的搜尋速度緩慢或結果不完整
</h3>

在 WSL 上[跨檔案系統工作](https://learn.microsoft.com/en-us/windows/wsl/filesystems)時的磁碟讀取效能損失可能導致在 WSL 上使用 Claude Code 時匹配數少於預期。搜尋仍然有效，但在原生檔案系統上返回的結果較少。

<Note>
  `claude doctor` 在這種情況下將搜尋顯示為正常。
</Note>

**解決方案：**

1. **提交更具體的搜尋**：透過指定目錄或檔案類型來減少搜尋的檔案數量：「Search for JWT validation logic in the auth-service package」或「Find use of md5 hash in JS files」。

2. **將專案移動到 Linux 檔案系統**：如果可能，確保您的專案位於 Linux 檔案系統（`/home/`）而不是 Windows 檔案系統（`/mnt/c/`）。

3. **改用原生 Windows**：考慮在 Windows 上原生執行 Claude Code 而不是透過 WSL，以獲得更好的檔案系統效能。

<h2 id="get-more-help">
  取得更多幫助
</h2>

如果您遇到此處未涵蓋的問題：

1. 執行 `/doctor` 進行設定檢查，並執行 `/mcp` 以檢查 MCP 伺服器狀態
2. 在 Claude Code 內使用 `/feedback` 命令直接向 Anthropic 報告問題
3. 檢查 [GitHub 存儲庫](https://github.com/anthropics/claude-code)以了解已知問題
4. 直接向 Claude 詢問其功能和特性。Claude 內置訪問其文檔。
