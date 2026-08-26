> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code 設定

> 使用全域和專案層級設定以及環境變數來設定 Claude Code。

Claude Code 提供多種設定選項，可根據您的需求配置其行為。您可以執行 `/config` 命令來設定 Claude Code，這會開啟一個標籤式設定介面，您可以在其中查看狀態資訊並修改設定選項。從 v2.1.181 版本開始，您可以透過將 `key=value` 傳遞給 `/config` 來變更單一選項，而無需開啟介面，例如 `/config verbose=true`。

<h2 id="configuration-scopes">
  設定範圍
</h2>

Claude Code 使用範圍系統來決定設定的適用位置和共享對象。了解範圍可幫助您決定如何為個人使用、團隊協作或企業部署設定 Claude Code。

<h3 id="available-scopes">
  可用的範圍
</h3>

| 範圍          | 位置                                               | 影響對象                                                        | 與團隊共享？        |
| :---------- | :----------------------------------------------- | :---------------------------------------------------------- | :------------ |
| **Managed** | 伺服器管理的設定、plist / 登錄或系統層級 `managed-settings.json` | 伺服器管理交付的所有組織成員；plist、HKLM 登錄和檔案交付的機器上的所有使用者；HKCU 登錄交付的目前使用者 | 是（由 IT 部署）    |
| **User**    | `~/.claude/` 目錄                                  | 您，跨所有專案                                                     | 否             |
| **Project** | 儲存庫中的 `.claude/`                                 | 此儲存庫的所有協作者                                                  | 是（提交到 git）    |
| **Local**   | `.claude/settings.local.json`                    | 您，僅在此儲存庫中                                                   | 否（gitignored） |

<h3 id="when-to-use-each-scope">
  何時使用各個範圍
</h3>

**Managed 範圍**用於：

* 必須在整個組織範圍內強制執行的安全政策
* 無法覆蓋的合規要求
* 由 IT/DevOps 部署的標準化設定

**User 範圍**最適合：

* 您想在任何地方使用的個人偏好設定（主題、編輯器設定）
* 您在所有專案中使用的工具和 plugins
* API 金鑰和身份驗證（安全儲存）

**Project 範圍**最適合：

* 團隊共享的設定（權限、hooks、MCP servers）
* 整個團隊應該擁有的 plugins
* 跨協作者標準化工具

**Local 範圍**最適合：

* 特定專案的個人覆蓋
* 在與團隊共享之前測試設定
* 對其他人不適用的機器特定設定

<h3 id="how-scopes-interact">
  範圍如何互動
</h3>

當相同的設定在多個範圍中出現時，Claude Code 會按優先順序應用它們：

1. **Managed**（最高）- 無法被任何東西覆蓋
2. **命令列引數** - 臨時工作階段覆蓋
3. **Local** - 覆蓋專案和使用者設定
4. **Project** - 覆蓋使用者設定
5. **User**（最低）- 當沒有其他東西指定設定時適用

例如，如果您的使用者設定將 `spinnerTipsEnabled` 設定為 `true`，而專案設定將其設定為 `false`，則專案值適用。權限規則的行為不同，因為它們跨範圍合併而不是覆蓋。請參閱[設定優先順序](#settings-precedence)。

<h3 id="what-uses-scopes">
  哪些功能使用範圍
</h3>

範圍適用於許多 Claude Code 功能：

| 功能              | 使用者位置                     | 專案位置                              | 本機位置                          |
| :-------------- | :------------------------ | :-------------------------------- | :---------------------------- |
| **Settings**    | `~/.claude/settings.json` | `.claude/settings.json`           | `.claude/settings.local.json` |
| **Subagents**   | `~/.claude/agents/`       | `.claude/agents/`                 | 無                             |
| **MCP servers** | `~/.claude.json`          | `.mcp.json`                       | `~/.claude.json`（每個專案）        |
| **Plugins**     | `~/.claude/settings.json` | `.claude/settings.json`           | `.claude/settings.local.json` |
| **CLAUDE.md**   | `~/.claude/CLAUDE.md`     | `CLAUDE.md` 或 `.claude/CLAUDE.md` | `CLAUDE.local.md`             |

在 Windows 上，顯示為 `~/.claude` 的路徑會解析為 `%USERPROFILE%\.claude`。

***

<h2 id="settings-files">
  設定檔案
</h2>

`settings.json` 檔案是透過分層設定來設定 Claude Code 的官方機制：

* **使用者設定**在 `~/.claude/settings.json` 中定義，適用於所有專案。
* **專案設定**儲存在您的專案目錄中：
  * `.claude/settings.json` 用於簽入原始碼控制並與您的團隊共享的設定
  * `.claude/settings.local.json` 用於未簽入的設定，適用於個人偏好和實驗。Claude Code 建立 `.claude/settings.local.json` 時，會設定 git 以忽略該檔案。如果您自己建立該檔案，請手動將其新增到 gitignore。

    因為此檔案是您的而不是儲存庫的，其權限 `allow` 規則會生效，無需 `.claude/settings.json` allow 規則所需的[工作區信任](/docs/zh-TW/permissions#project-allow-rules-and-workspace-trust)步驟。如果儲存庫提供該檔案，例如透過提交它，工作區信任仍然適用。
* **Managed 設定**：對於需要集中控制的組織，Claude Code 支援多種 managed 設定的傳遞機制。所有機制都使用相同的 JSON 格式，無法被使用者或專案設定覆蓋：

  * **伺服器管理的設定**：透過 Anthropic 的伺服器或自託管的 [Claude apps gateway](/docs/zh-TW/claude-apps-gateway) 在登入時遠端傳遞，可從 claude.ai 管理員主控台或自託管 Claude apps gateway 傳遞。請參閱[伺服器管理的設定](/docs/zh-TW/server-managed-settings)。
  * **MDM/OS 層級政策**：透過 macOS 和 Windows 上的原生裝置管理傳遞：
    * macOS：`com.anthropic.claudecode` managed preferences 網域。plist 的頂層金鑰鏡像 `managed-settings.json`，巢狀設定為字典，陣列為 plist 陣列。透過 Jamf、Iru (Kandji) 或類似 MDM 工具中的設定檔案部署。
    * Windows：`HKLM\SOFTWARE\Policies\ClaudeCode` 登錄機碼，其中包含 `Settings` 值（REG\_SZ 或 REG\_EXPAND\_SZ）包含 JSON（透過群組原則或 Intune 部署）
    * Windows（使用者層級）：`HKCU\SOFTWARE\Policies\ClaudeCode`（最低政策優先順序，僅在沒有管理員層級來源時使用）
  * **檔案型**：`managed-settings.json` 和 `managed-mcp.json` 部署到系統目錄：

    * macOS：`/Library/Application Support/ClaudeCode/`
    * Linux 和 WSL：`/etc/claude-code/`
    * Windows：`C:\Program Files\ClaudeCode\`

    <Warning>
      自 v2.1.75 起，舊版 Windows 路徑 `C:\ProgramData\ClaudeCode\managed-settings.json` 不再受支援。已將設定部署到該位置的管理員必須將檔案遷移到 `C:\Program Files\ClaudeCode\managed-settings.json`。
    </Warning>

    檔案型 managed 設定也支援在與 `managed-settings.json` 相同的系統目錄中的 `managed-settings.d/` 放入目錄。這讓不同的團隊可以部署獨立的政策片段，而無需協調對單一檔案的編輯。

    遵循 systemd 慣例，`managed-settings.json` 首先作為基礎合併，然後放入目錄中的所有 `*.json` 檔案按字母順序排序並合併在頂部。對於純量值，後面的檔案會覆蓋前面的檔案；陣列會連接並去重；物件會深度合併。以 `.` 開頭的隱藏檔案會被忽略。

    使用數字前綴來控制合併順序，例如 `10-telemetry.json` 和 `20-security.json`。

  請參閱 [managed 設定](/docs/zh-TW/permissions#managed-only-settings) 和 [Managed MCP 設定](/docs/zh-TW/managed-mcp) 以取得詳細資訊。

  此[儲存庫](https://github.com/anthropics/claude-code/tree/main/examples/mdm)包含 Jamf、Iru (Kandji)、Intune 和群組原則的入門部署範本。使用這些作為起點，並根據您的需求進行調整。

  <Note>
    Managed 部署也可以使用 `strictKnownMarketplaces` 限制 **plugin marketplace 新增**。如需詳細資訊，請參閱 [Managed marketplace 限制](/docs/zh-TW/plugin-marketplaces#managed-marketplace-restrictions)。
  </Note>
* **其他設定**儲存在 `~/.claude.json` 中。此檔案包含您的 OAuth 工作階段、[MCP server](/docs/zh-TW/mcp) 設定（用於使用者和本機範圍）、每個專案的狀態（允許的工具、信任設定）和各種快取。專案範圍的 MCP servers 分別儲存在 `.mcp.json` 中。

<Note>
  Claude Code 會自動建立設定檔案的時間戳記備份，並保留最近五個備份以防止資料遺失。
</Note>

```JSON 設定檔案範例 theme={null}
{
  "$schema": "https://json.schemastore.org/claude-code-settings.json",
  "permissions": {
    "allow": [
      "Bash(npm run lint)",
      "Bash(npm run test *)",
      "Read(~/.zshrc)"
    ],
    "deny": [
      "Bash(curl *)",
      "Read(./.env)",
      "Read(./.env.*)",
      "Read(./secrets/**)"
    ]
  },
  "env": {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    "OTEL_METRICS_EXPORTER": "otlp"
  },
  "companyAnnouncements": [
    "Welcome to Acme Corp! Review our code guidelines at docs.acme.com",
    "Reminder: Code reviews required for all PRs",
    "New security policy in effect"
  ]
}
```

上面範例中的 `$schema` 行指向 Claude Code 設定的[官方 JSON 架構](https://json.schemastore.org/claude-code-settings.json)。將其新增到您的 `settings.json` 可在 VS Code、Cursor 和任何其他支援 JSON 架構驗證的編輯器中啟用自動完成和內嵌驗證。

已發佈的架構會定期更新，可能不包含最新 CLI 版本中新增的設定，因此最近記錄的欄位上的驗證警告不一定表示您的設定無效。

<h3 id="when-edits-take-effect">
  編輯何時生效
</h3>

Claude Code 會監視您的設定檔案，並在它們變更時重新載入它們，因此對大多數金鑰的編輯會在執行中的工作階段中應用，無需重新啟動。這包括 `permissions`、`hooks` 和認證協助程式（如 `apiKeyHelper`）。重新載入涵蓋使用者、專案、本機和 managed 設定，並且 [`ConfigChange` hook](/docs/zh-TW/hooks#configchange) 會針對每個偵測到的變更觸發。

少數金鑰在工作階段啟動時讀取一次，並在下次重新啟動時應用：

* `model`：使用 [`/model`](/docs/zh-TW/model-config#setting-your-model) 在工作階段中切換
* [`outputStyle`](/docs/zh-TW/output-styles)：系統提示的一部分，在 `/clear` 或重新啟動時重建

<h3 id="invalid-entries-in-managed-settings">
  Managed 設定中的無效項目
</h3>

Managed 設定會寬容地解析。當 managed 設定包含驗證架構失敗的項目時，Claude Code 會移除該項目、記錄警告，並強制執行每個剩餘的有效政策。單一拼寫錯誤無法停用組織政策的其餘部分。執行 [`/doctor`](/docs/zh-TW/debug-your-config#check-resolved-settings) 以列出被移除的項目及其來源檔案和欄位。

此行為在所有三種傳遞機制中一致：[伺服器管理的設定](/docs/zh-TW/server-managed-settings)、透過 MDM 部署的 plist 和登錄政策，以及 `managed-settings.json` 檔案。需要 Claude Code v2.1.169 或更新版本。

安全強制欄位按欄位處理，而不是在存在但無效時被整體移除：

| 欄位                           | 存在但無效時的行為                                                                                                                   |
| :--------------------------- | :-------------------------------------------------------------------------------------------------------------------------- |
| `allowedMcpServers`          | 作為空白名單強制執行，因此在修復值之前不允許任何 MCP servers。個別無效項目會被移除，有效子集會被強制執行。                                                                 |
| `allowManagedMcpServersOnly` | 視為 `true`。                                                                                                                  |
| `availableModels`            | 作為空白名單強制執行，因此在修復值之前僅預設模型可用。個別非字串項目會被移除，有效子集會被強制執行。適用於 v2.1.175 及更新版本。                                                       |
| `enforceAvailableModels`     | 視為 `true`。適用於 v2.1.175 及更新版本。                                                                                               |
| `forceLoginOrgUUID`          | 在修復值之前，不允許任何組織登入。                                                                                                           |
| `deniedMcpServers`           | 個別無效項目會被移除，有效子集會被強制執行。完全無效的值會被丟棄並出現警告，因為拒絕每個 server 會阻止政策從未命名的 servers。                                                     |
| `sandbox.credentials`        | 在 `files` 或 `envVars` 中的個別無效項目會被移除並出現警告，有效子集會被強制執行。完全無效的 `credentials` 值會被丟棄並出現警告，而 `sandbox` 的其餘部分仍然適用。適用於 v2.1.191 及更新版本。 |

`requiredMinimumVersion` 和 `requiredMaximumVersion` 設計上會失敗開放：無效的值會被移除而不是強制執行，因此不良的政策推送無法防止 Claude Code 啟動。

驗證錯誤會在三個地方出現：

* 互動式工作階段在啟動時顯示列出無效項目的對話框。
* 使用 `-p` 的無頭執行會將摘要列印到 stderr。
* [`claude doctor`](/docs/zh-TW/debug-your-config) 列出每個無效項目及其來源和欄位。

在整個機隊部署政策變更之前，在測試機器上執行 `claude doctor` 以驗證政策變更。

此寬容性僅適用於 managed 設定。使用者、專案和本機設定檔案保持嚴格：驗證失敗的檔案會被整體拒絕並報告。

<h3 id="available-settings">
  可用的設定
</h3>

`settings.json` 支援多個選項：

| 金鑰                                 | 說明                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | 範例                                                                                                                              |
| :--------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------ |
| `advisorModel`                     | 伺服器端 [advisor tool](/docs/zh-TW/advisor) 的模型。接受模型別名，例如 `"opus"`、`"sonnet"` 或 `"fable"`（v2.1.170+），或完整模型 ID。當您執行 `/advisor` 時自動寫入。取消設定以停用 advisor                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | `"opus"`                                                                                                                        |
| `agent`                            | 將主執行緒作為命名 subagent 執行，並為從 `claude agents` 分派的工作階段設定預設 agent。應用該 subagent 的系統提示、工具限制和模型。請參閱[明確叫用 subagents](/docs/zh-TW/sub-agents#invoke-subagents-explicitly)                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | `"code-reviewer"`                                                                                                               |
| `agentPushNotifEnabled`            | **預設**：`false`。當[遠端控制](/docs/zh-TW/remote-control)已連線時，允許 Claude 主動傳送推播通知到您的手機，例如當長時間工作完成時。在 `/config` 中顯示為**Claude 決定時推播**。請參閱[行動推播通知](/docs/zh-TW/remote-control#mobile-push-notifications)。需要 Claude Code v2.1.119 或更新版本                                                                                                                                                                                                                                                                                                                                                                                                                  | `true`                                                                                                                          |
| `allowAllClaudeAiMcps`             | （Managed 設定僅限）載入 claude.ai connectors 以及部署的 `managed-mcp.json`，否則會取得獨佔控制並抑制它們。請參閱 [Managed MCP 設定](/docs/zh-TW/managed-mcp)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | `true`                                                                                                                          |
| `allowedChannelPlugins`            | （Managed 設定僅限）可能推送訊息的頻道 plugins 白名單。在設定時替換預設 Anthropic 白名單。未定義 = 回退到預設值，空陣列 = 阻止所有頻道 plugins。需要 `channelsEnabled: true`。請參閱[限制哪些頻道 plugins 可以執行](/docs/zh-TW/channels#restrict-which-channel-plugins-can-run)                                                                                                                                                                                                                                                                                                                                                                                                                           | `[{ "marketplace": "claude-plugins-official", "plugin": "telegram" }]`                                                          |
| `allowedHttpHookUrls`              | HTTP hooks 可能針對的 URL 模式白名單。支援 `*` 作為萬用字元。設定時，具有不匹配 URL 的 hooks 會被阻止。未定義 = 無限制，空陣列 = 阻止所有 HTTP hooks。陣列跨設定來源合併。請參閱 [Hook 設定](#hook-configuration)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `["https://hooks.example.com/*"]`                                                                                               |
| `allowedMcpServers`                | 在 managed-settings.json 中設定時，使用者可以設定的 MCP servers 白名單。未定義 = 無限制，空陣列 = 鎖定。適用於所有範圍。拒絕清單優先。請參閱 [Managed MCP 設定](/docs/zh-TW/managed-mcp)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `[{ "serverName": "github" }]`                                                                                                  |
| `allowManagedHooksOnly`            | （Managed 設定僅限）僅載入 managed hooks、SDK hooks 和在 managed 設定 `enabledPlugins` 中強制啟用的 plugins 中的 hooks。使用者、專案和所有其他 plugin hooks 被阻止。請參閱 [Hook 設定](#hook-configuration)                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `true`                                                                                                                          |
| `allowManagedMcpServersOnly`       | （Managed 設定僅限）僅尊重 managed 設定中的 `allowedMcpServers`。`deniedMcpServers` 仍從所有來源合併。使用者仍可新增 MCP servers，但僅適用管理員定義的白名單。請參閱 [Managed MCP 設定](/docs/zh-TW/managed-mcp)                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | `true`                                                                                                                          |
| `allowManagedPermissionRulesOnly`  | （Managed 設定僅限）防止使用者和專案設定定義 `allow`、`ask` 或 `deny` 權限規則。僅適用 managed 設定中的規則。請參閱 [Managed 專用設定](/docs/zh-TW/permissions#managed-only-settings)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | `true`                                                                                                                          |
| `alwaysThinkingEnabled`            | 為所有工作階段預設啟用[擴展思考](/docs/zh-TW/model-config#extended-thinking)。通常透過 `/config` 命令而不是直接編輯來設定。若要強制思考關閉，無論此設定如何，請在 `env` 中設定 [`MAX_THINKING_TOKENS=0`](/docs/zh-TW/env-vars)，這會停用 Anthropic API 上的思考，除了 Fable 5，無法關閉思考。在[第三方提供者](/docs/zh-TW/third-party-integrations)上，這會改為省略 `thinking` 參數，自適應推理模型仍可能思考                                                                                                                                                                                                                                                                                                                                              | `true`                                                                                                                          |
| `apiKeyHelper`                     | 自訂指令碼，在系統 shell（macOS 和 Linux 上為 `/bin/sh`，Windows 上為 `cmd`）中執行，以產生驗證值。此值將作為 `X-Api-Key` 和 `Authorization: Bearer` 標頭傳送以進行模型請求。使用 [`CLAUDE_CODE_API_KEY_HELPER_TTL_MS`](/docs/zh-TW/env-vars) 設定重新整理間隔                                                                                                                                                                                                                                                                                                                                                                                                                                  | `/bin/generate_temp_api_key.sh`                                                                                                 |
| `askUserQuestionTimeout`           | **預設**：`"never"`。未回答的 [`AskUserQuestion`](/docs/zh-TW/tools-reference) 對話框在自動繼續前的閒置時間，使用您已選擇的任何選項。接受 `"60s"`、`"5m"`、`"10m"` 或 `"never"`。使用預設值，問題會等待您回答。在 `/config` 中顯示為**問題自動繼續逾時**，會將此金鑰寫入使用者設定。不從專案或本機設定讀取。需要 Claude Code v2.1.200 或更新版本                                                                                                                                                                                                                                                                                                                                                                                              | `"5m"`                                                                                                                          |
| `attribution`                      | 自訂 git 提交和拉取請求的歸屬。請參閱[歸屬設定](#attribution-settings)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `{"commit": "🤖 Generated with Claude Code", "pr": ""}`                                                                         |
| `autoCompactEnabled`               | **預設**：`true`。當內容接近限制時自動壓縮對話。在 `/config` 中顯示為**自動壓縮**。若要透過環境變數停用，請在 `env` 中設定 [`DISABLE_AUTO_COMPACT`](/docs/zh-TW/env-vars)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | `false`                                                                                                                         |
| `autoMemoryDirectory`              | [自動記憶](/docs/zh-TW/memory#storage-location)儲存的自訂目錄。接受絕對路徑或 `~/` 前綴的路徑。從專案或本機設定接受，此設定在您接受工作區信任對話後才受尊重，因為複製的儲存庫可能提供此檔案                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `"~/my-memory-dir"`                                                                                                             |
| `autoMemoryEnabled`                | **預設**：`true`。啟用[自動記憶](/docs/zh-TW/memory#enable-or-disable-auto-memory)。當為 `false` 時，Claude 不會從自動記憶目錄讀取或寫入。您也可以在工作階段期間使用 `/memory` 切換此設定。若要透過環境變數停用，請在 `env` 中設定 [`CLAUDE_CODE_DISABLE_AUTO_MEMORY`](/docs/zh-TW/env-vars)                                                                                                                                                                                                                                                                                                                                                                                                                  | `false`                                                                                                                         |
| `autoMode`                         | 自訂[自動模式](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode)分類器阻止和允許的內容。包含 `environment`、`allow`、`soft_deny` 和 `hard_deny` 陣列的散文規則。在陣列中包含字面字串 `"$defaults"` 以在該位置繼承內建規則。請參閱[設定自動模式](/docs/zh-TW/auto-mode-config)。僅從使用者設定、`--settings` 旗標和 managed 設定讀取。在專案 `.claude/settings.json` 和本機 `.claude/settings.local.json` 中被忽略。在 v2.1.207 之前，`.claude/settings.local.json` 也被讀取                                                                                                                                                                                                                                                       | `{"soft_deny": ["$defaults", "Never run terraform apply"]}`                                                                     |
| `autoMode.classifyAllShell`        | **預設**：`false`。當為 `true` 時，在自動模式使用中時暫停每個 Bash 和 PowerShell 允許規則，以便所有 shell 命令透過分類器路由，而不僅僅是符合任意程式碼執行模式的規則。請參閱[透過分類器路由所有 shell 命令](/docs/zh-TW/auto-mode-config#route-all-shell-commands-through-the-classifier)。需要 Claude Code v2.1.193 或更新版本                                                                                                                                                                                                                                                                                                                                                                                            | `true`                                                                                                                          |
| `autoScrollEnabled`                | **預設**：`true`。在[全螢幕渲染](/docs/zh-TW/fullscreen)中，跟隨新輸出到對話的底部。在 `/config` 中顯示為**自動捲軸**。當此設定關閉時，權限提示仍會捲軸進入檢視                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | `false`                                                                                                                         |
| `autoUpdatesChannel`               | **預設**：`"latest"`。遵循更新的發行頻道。使用 `"stable"` 以取得通常約一週舊的版本並跳過有重大迴歸的版本，或 `"latest"` 以取得最新版本。若要完全停用自動更新，請在 `env` 中設定 [`DISABLE_AUTOUPDATER`](/docs/zh-TW/setup#disable-auto-updates)                                                                                                                                                                                                                                                                                                                                                                                                                                                          | `"stable"`                                                                                                                      |
| `availableModels`                  | 限制使用者可以為主工作階段、[subagents](/docs/zh-TW/sub-agents)、[skills](/docs/zh-TW/skills) 和 [advisor](/docs/zh-TW/advisor) 選擇的模型。不影響預設選項，除非 `enforceAvailableModels` 也設定。請參閱[限制模型選擇](/docs/zh-TW/model-config#restrict-model-selection)                                                                                                                                                                                                                                                                                                                                                                                                                           | `["sonnet", "haiku"]`                                                                                                           |
| `awaySummaryEnabled`               | 在您離開終端機幾分鐘後返回時顯示單行工作階段摘要。設定為 `false` 或在 `/config` 中關閉工作階段摘要以停用。與 [`CLAUDE_CODE_ENABLE_AWAY_SUMMARY`](/docs/zh-TW/env-vars) 相同                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | `true`                                                                                                                          |
| `awsAuthRefresh`                   | 修改 `.aws` 目錄的自訂指令碼（請參閱[進階認證設定](/docs/zh-TW/amazon-bedrock#advanced-credential-configuration)）                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | `aws sso login --profile myprofile`                                                                                             |
| `awsCredentialExport`              | 輸出包含 AWS 認證的 JSON 的自訂指令碼（請參閱[進階認證設定](/docs/zh-TW/amazon-bedrock#advanced-credential-configuration)）                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | `/bin/generate_aws_grant.sh`                                                                                                    |
| `axScreenReader`                   | 渲染螢幕閱讀器友善的輸出：沒有裝飾邊框或動畫的平面文字。螢幕閱讀器模式使用經典渲染器，因此在其使用中時 `tui` 設定無效；附加的[背景工作階段](/docs/zh-TW/agent-view)仍會全螢幕渲染。[`CLAUDE_AX_SCREEN_READER`](/docs/zh-TW/env-vars) 環境變數和 [`--ax-screen-reader`](/docs/zh-TW/cli-reference#cli-flags) 旗標優先。需要 Claude Code v2.1.181 或更新版本                                                                                                                                                                                                                                                                                                                                                                                  | `true`                                                                                                                          |
| `blockedMarketplaces`              | （Managed 設定僅限）marketplace 來源的黑名單。在 marketplace 新增和 plugin 安裝、更新、重新整理和自動更新時強制執行，因此在設定政策之前新增的 marketplace 無法用於擷取 plugins。在下載前檢查被阻止的來源，因此它們永遠不會接觸檔案系統。請參閱 [Managed marketplace 限制](/docs/zh-TW/plugin-marketplaces#managed-marketplace-restrictions)                                                                                                                                                                                                                                                                                                                                                                                       | `[{ "source": "github", "repo": "untrusted/plugins" }]`                                                                         |
| `browserExternalPageTools`         | （Managed 設定僅限）設定為 `"disabled"` 以防止 Claude 使用工具讀取或作用於桌面應用程式[瀏覽器窗格](/docs/zh-TW/desktop#browse-external-sites)中的外部頁面。使用者仍可自行導覽到外部網站，本機開發伺服器預覽不受影響                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | `"disabled"`                                                                                                                    |
| `channelsEnabled`                  | （Managed 設定僅限）允許組織使用[頻道](/docs/zh-TW/channels)。在 claude.ai Team 和 Enterprise 方案上，當此設定未設定或為 `false` 時，頻道會被阻止。對於使用 API 金鑰驗證的 [Anthropic Console](/docs/zh-TW/authentication#claude-console-authentication) 帳戶，除非您的組織部署 managed 設定（在這種情況下此金鑰必須設定為 `true`），否則預設允許頻道                                                                                                                                                                                                                                                                                                                                                                              | `true`                                                                                                                          |
| `claudeMd`                         | （Managed 設定僅限）CLAUDE.md 樣式的指示，作為組織管理的記憶注入。僅在 managed 或政策設定中設定時受尊重，在使用者、專案和本機設定中被忽略。請參閱[組織範圍的 CLAUDE.md](/docs/zh-TW/memory#deploy-organization-wide-claude-md)                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | `"Always run make lint before committing."`                                                                                     |
| `claudeMdExcludes`                 | 載入[記憶](/docs/zh-TW/memory)時要跳過的 `CLAUDE.md` 檔案的 Glob 模式或絕對路徑。模式與絕對檔案路徑相符。僅適用於使用者、專案和本機記憶；managed 政策檔案無法排除                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | `["**/vendor/**/CLAUDE.md"]`                                                                                                    |
| `cleanupPeriodDays`                | **預設**：`30` 天，最少 `1`。Claude Code 刪除[工作階段檔案和其他應用程式資料](/docs/zh-TW/claude-directory#cleaned-up-automatically)超過此期間的檔案在啟動時。設定 `0` 會失敗並出現驗證錯誤。相同的年齡截止也適用於[孤立 worktrees](/docs/zh-TW/worktrees#clean-up-worktrees) 在啟動時的自動移除。如果 Claude Code 無法讀取或解析設定檔案，它會暫停保留清理掃描並在 `/status` 中顯示警告，直到您修復檔案，除非 [managed 設定](/docs/zh-TW/server-managed-settings)提供 `cleanupPeriodDays`，在這種情況下掃描以 managed 值執行。在 v2.1.203 之前，清理以 30 天預設執行，並可能刪除較長 `cleanupPeriodDays` 打算保留的文字記錄；新於 30 天的檔案永遠不會被移除。若要完全停用文字記錄寫入，請設定 [`CLAUDE_CODE_SKIP_PROMPT_HISTORY`](/docs/zh-TW/env-vars) 環境變數。在非互動模式中，使用 `--no-session-persistence` 與 `-p` 一起傳遞或在 Agent SDK 中設定 `persistSession: false`。 | `20`                                                                                                                            |
| `companyAnnouncements`             | 在啟動時向使用者顯示的公告。如果提供多個公告，它們將隨機循環。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `["Welcome to Acme Corp! Review our code guidelines at docs.acme.com"]`                                                         |
| `defaultShell`                     | **預設**：`"bash"`，或在 Bash 不可用時 Windows 上為 `"powershell"`。輸入框 `!` 命令的預設 shell。接受 `"bash"` 或 `"powershell"`。設定 `"powershell"` 會在 Windows 上透過 PowerShell 路由互動式 `!` 命令。需要 `CLAUDE_CODE_USE_POWERSHELL_TOOL=1`。請參閱 [PowerShell tool](/docs/zh-TW/tools-reference#powershell-tool)                                                                                                                                                                                                                                                                                                                                                              | `"powershell"`                                                                                                                  |
| `deniedMcpServers`                 | 在 managed-settings.json 中設定時，明確阻止的 MCP servers 拒絕清單。適用於所有範圍，包括 managed servers。拒絕清單優先於白名單。請參閱 [Managed MCP 設定](/docs/zh-TW/managed-mcp)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | `[{ "serverName": "filesystem" }]`                                                                                              |
| `disableAgentView`                 | 設定為 `true` 以關閉[背景代理和代理檢視](/docs/zh-TW/agent-view)：`claude agents`、`--bg`、`/background` 和隨選主管。通常在 [managed 設定](/docs/zh-TW/permissions#managed-settings) 中設定。等同於將 `CLAUDE_CODE_DISABLE_AGENT_VIEW` 設定為 `1`                                                                                                                                                                                                                                                                                                                                                                                                                                    | `true`                                                                                                                          |
| `disableAllHooks`                  | 停用所有 [hooks](/docs/zh-TW/hooks) 和任何自訂[狀態行](/docs/zh-TW/statusline)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | `true`                                                                                                                          |
| `disableArtifact`                  | 設定為 `true` 以停用 [Artifact](/docs/zh-TW/artifacts) 工具，該工具將工作階段輸出發佈為 claude.ai 上的私人網頁。等同於將 `CLAUDE_CODE_DISABLE_ARTIFACT` 設定為 `1`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | `true`                                                                                                                          |
| `disableAutoMode`                  | 設定為 `"disable"` 以防止[自動模式](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode)被啟用。從 `Shift+Tab` 循環中移除 `auto` 並在啟動時拒絕 `--permission-mode auto`。在[managed 設定](/docs/zh-TW/permissions#managed-settings)中最有用，使用者無法覆蓋它                                                                                                                                                                                                                                                                                                                                                                                                                 | `"disable"`                                                                                                                     |
| `disableBrowserExternalNavigation` | （Managed 設定僅限）設定為 `true` 以關閉桌面應用程式[瀏覽器窗格](/docs/zh-TW/desktop#browse-external-sites)中的外部瀏覽。使用者和 Claude 都無法導覽到外部網站，localhost 開發伺服器預覽不受影響。值必須是 JSON 布林值 `true`；字串 `"true"` 會被忽略                                                                                                                                                                                                                                                                                                                                                                                                                                                           | `true`                                                                                                                          |
| `disableBundledSkills`             | 設定為 `true` 以停用隨 Claude Code 一起提供的 [skills](/docs/zh-TW/skills) 和工作流程：bundled skills 和工作流程會被完全移除，而內建斜線命令（如 `/init`）保持可輸入但對模型隱藏。`/doctor` 保持可輸入，如內建命令；改用 [`DISABLE_DOCTOR_COMMAND`](/docs/zh-TW/env-vars) 隱藏它。來自 plugins、`.claude/skills/` 和 `.claude/commands/` 的 Skills 不受影響。等同於將 `CLAUDE_CODE_DISABLE_BUNDLED_SKILLS` 設定為 `1`                                                                                                                                                                                                                                                                                                               | `true`                                                                                                                          |
| `disableClaudeAiConnectors`        | 停用 [claude.ai MCP connectors](/docs/zh-TW/mcp#use-mcp-servers-from-claude-ai)，使其不會自動擷取或連線。在任何設定範圍中設定。任何來源中的 `true` 優先，因此簽入的專案 `.claude/settings.json` 可以選擇退出雲端 connectors，但專案層級的 `false` 無法覆蓋使用者或政策層級的 `true`。透過 `--mcp-config` 明確傳遞的 Servers 不受影響。若要拒絕個別 connectors 而不是所有 connectors，請改用 [`deniedMcpServers`](/docs/zh-TW/managed-mcp)。需要 Claude Code v2.1.182 或更新版本                                                                                                                                                                                                                                                                        | `true`                                                                                                                          |
| `disableDeepLinkRegistration`      | 設定為 `"disable"` 以防止 Claude Code 在啟動時向作業系統註冊 `claude-cli://` 協議處理程式。[深層連結](/docs/zh-TW/deep-links)讓外部工具透過預先填入的提示開啟 Claude Code 工作階段。在協議處理程式註冊受限或單獨管理的環境中很有用                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | `"disable"`                                                                                                                     |
| `disabledMcpjsonServers`           | 要拒絕的 `.mcp.json` 檔案中特定 MCP servers 的清單                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             | `["filesystem"]`                                                                                                                |
| `disableRemoteControl`             | 停用[遠端控制](/docs/zh-TW/remote-control)：阻止 `claude remote-control`、`--remote-control` 旗標、自動啟動和工作階段內切換。通常放在[managed 設定](/docs/zh-TW/permissions#managed-settings)中以進行每個裝置的 MDM 強制執行，但適用於任何範圍。需要 Claude Code v2.1.128 或更新版本                                                                                                                                                                                                                                                                                                                                                                                                                       | `true`                                                                                                                          |
| `disableSideloadFlags`             | （Managed 設定僅限）在啟動時拒絕 `--plugin-dir`、`--plugin-url`、`--agents` 和 `--mcp-config` CLI 旗標，使用者可能會傳遞這些旗標以繞過 [`strictKnownMarketplaces`](#strictknownmarketplaces) 進行單一執行。也拒絕從任何內部產生 CLI 的表面傳遞這些旗標，目前[Cowork](/docs/zh-TW/desktop) 桌面應用程式中的本機工作階段。其 servers 全部為進程內 `type: "sdk"` 項目的 `--mcp-config` 仍被接受，因此 Agent SDK 和 VS Code 擴充功能保持工作。不阻止 `claude mcp add`、`.mcp.json` 或 SDK `setMcpServers()`；與 [`allowedMcpServers`](/docs/zh-TW/managed-mcp) 配對以進行每個 server 的 MCP 控制。需要 Claude Code v2.1.193 或更新版本                                                                                                                                            | `true`                                                                                                                          |
| `disableSkillShellExecution`       | 停用 [skills](/docs/zh-TW/skills) 和來自使用者、專案、plugin 或其他目錄來源的自訂命令中的內嵌 shell 執行（`` !`...` `` 和 ` ```! ` 區塊）。命令會被替換為 `[shell command execution disabled by policy]` 而不是被執行。Bundled 和 managed skills 不受影響。在[managed 設定](/zh-TW/permissions#managed-settings)中最有用，使用者無法覆蓋它                                                                                                                                                                                                                                                                                                                                                                      | `true`                                                                                                                          |
| `disableWorkflows`                 | **預設**：`false`。停用[動態工作流程](/docs/zh-TW/workflows#turn-workflows-off)和 bundled workflow 命令。等同於將 `CLAUDE_CODE_DISABLE_WORKFLOWS` 設定為 `1`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `true`                                                                                                                          |
| `editorMode`                       | **預設**：`"normal"`。輸入提示的快捷鍵模式：`"normal"` 或 `"vim"`。在 `/config` 中顯示為**編輯器模式**                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | `"vim"`                                                                                                                         |
| `effortLevel`                      | 跨工作階段持久化[努力等級](/docs/zh-TW/model-config#adjust-effort-level)。接受 `"low"`、`"medium"`、`"high"` 或 `"xhigh"`。當您執行 `/effort` 時自動寫入，其中包含其中一個值。`--effort` 和 [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/zh-TW/env-vars) 會覆蓋此設定以進行一個工作階段。請參閱[調整努力等級](/docs/zh-TW/model-config#adjust-effort-level)以了解支援的模型                                                                                                                                                                                                                                                                                                                                                          | `"xhigh"`                                                                                                                       |
| `enableAllProjectMcpServers`       | 自動批准專案 `.mcp.json` 檔案中定義的所有 MCP servers。自 v2.1.196 起，`claude mcp list` 和 `claude mcp get` 在不受信任的資料夾中僅從[未簽入儲存庫的設定檔案](/docs/zh-TW/mcp#managing-your-servers)中尊重此金鑰                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | `true`                                                                                                                          |
| `enableArtifact`                   | 為此使用者啟用或停用 [Artifact](/docs/zh-TW/artifacts) 工具。未設定時，預設遵循功能的[可用性](/docs/zh-TW/artifacts#availability)以取得您的帳戶。`/config` 中的 **Artifacts** 列寫入此金鑰。Managed `disableArtifact` 和您組織的[管理員設定](/docs/zh-TW/artifacts#manage-artifacts-for-your-organization)優先，該金鑰在專案和本機設定（`.claude/settings.json`、`.claude/settings.local.json`）中被忽略，儲存庫可能會簽入。需要 Claude Code v2.1.196 或更新版本                                                                                                                                                                                                                                                                               | `true`                                                                                                                          |
| `enabledMcpjsonServers`            | 要批准的 `.mcp.json` 檔案中特定 MCP servers 的清單。自 v2.1.196 起，`claude mcp list` 和 `claude mcp get` 在不受信任的資料夾中僅從[未簽入儲存庫的設定檔案](/docs/zh-TW/mcp#managing-your-servers)中尊重此金鑰                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | `["memory", "github"]`                                                                                                          |
| `enforceAvailableModels`           | 將 `availableModels` 白名單擴展到預設模型。當在 managed 設定中為 `true` 且 `availableModels` 是非空陣列時，預設選項會回退到第一個可用的白名單項目，但僅當使用者帳戶類型的預設模型不在白名單中時；白名單預設會保持原樣。當 `availableModels` 未設定或為空時無效。請參閱[為預設模型強制執行白名單](/docs/zh-TW/model-config#enforce-the-allowlist-for-the-default-model)。需要 Claude Code v2.1.175 或更新版本                                                                                                                                                                                                                                                                                                                                              | `true`                                                                                                                          |
| `env`                              | 應用於每個工作階段和 Claude Code 從中產生的子流程的環境變數。將變數設定為 `""` 以使用空字串覆蓋 shell 匯出，Claude Code 將其視為未設定以進行提供者選擇。子流程仍會繼承空值。`NO_COLOR` 和 `FORCE_COLOR` 在此處設定時僅到達子流程；若要變更 Claude Code 自己的介面顏色，請在啟動 `claude` 前在您的 shell 中設定它們。自 v2.1.195 起，Claude Code 的託管環境設定的身份變數（例如 `CLAUDE_CODE_REMOTE` 和 `CLAUDE_CODE_ACCOUNT_UUID`）在此處設定時會被忽略                                                                                                                                                                                                                                                                                                                     | `{"FOO": "bar"}`                                                                                                                |
| `fallbackModel`                    | 當主模型過載或不可用時按順序嘗試的備用模型。Claude Code 會為該輪的其餘部分切換到鏈中的下一個可用模型並顯示通知。`"default"` 擴展為預設模型。鏈限制為三個模型；額外項目會被忽略。與大多數陣列設定不同，此金鑰不跨設定檔案合併：定義它的最高優先順序檔案提供整個鏈。[`--fallback-model`](/docs/zh-TW/cli-reference#cli-flags) 旗標會覆蓋此設定以進行一個工作階段。請參閱[備用模型鏈](/docs/zh-TW/model-config#fallback-model-chains)                                                                                                                                                                                                                                                                                                                                                          | `["claude-sonnet-5", "claude-haiku-4-5"]`                                                                                       |
| `fastMode`                         | 為可用的工作階段開啟[快速模式](/docs/zh-TW/fast-mode)。使用 `/fast` 切換會在使用者設定中寫入 `true`，當您關閉快速模式時移除金鑰                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `true`                                                                                                                          |
| `fastModePerSessionOptIn`          | 當為 `true` 時，快速模式不會跨工作階段持久化。每個工作階段都以快速模式關閉開始，需要使用者使用 `/fast` 啟用它。使用者的快速模式偏好仍會儲存。請參閱[需要每個工作階段的選擇加入](/docs/zh-TW/fast-mode#require-per-session-opt-in)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | `true`                                                                                                                          |
| `feedbackSurveyRate`               | [工作階段品質調查](/docs/zh-TW/data-usage#session-quality-surveys)出現時符合條件的機率（0–1）。設定為 `0` 以完全抑制，或設定 [`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY`](/docs/zh-TW/env-vars) 在 `env` 中。在使用 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 時很有用，其中預設樣本率不適用                                                                                                                                                                                                                                                                                                                                                                                     | `0.05`                                                                                                                          |
| `fileCheckpointingEnabled`         | **預設**：`true`。在每次編輯前快照檔案，以便 [`/rewind`](/docs/zh-TW/checkpointing) 可以還原它們。在 `/config` 中顯示為**倒帶程式碼（檢查點）**。若要透過環境變數停用，請在 `env` 中設定 [`CLAUDE_CODE_DISABLE_FILE_CHECKPOINTING`](/docs/zh-TW/env-vars)                                                                                                                                                                                                                                                                                                                                                                                                                                            | `false`                                                                                                                         |
| `fileSuggestion`                   | 為 `@` 檔案自動完成設定自訂指令碼。請參閱[檔案建議設定](#file-suggestion-settings)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         | `{"type": "command", "command": "~/.claude/file-suggestion.sh"}`                                                                |
| `footerLinksRegexes`               | 當 regex 符合輪次輸出時在頁尾中渲染額外的可點擊徽章。每個項目都有一個 `pattern`、一個包含 `{name}` 佔位符的 URL 範本（從命名擷取群組填入），以及一個選用的 `label`。僅從使用者設定、`--settings` 旗標和 managed 設定讀取。請參閱[頁尾連結徽章](#footer-link-badges)以了解 URL 限制、方案白名單和限制。需要 Claude Code v2.1.176 或更新版本                                                                                                                                                                                                                                                                                                                                                                                                      | `[{"type": "regex", "pattern": "\\b(?<key>PROJ-\\d+)\\b", "url": "https://issues.example.com/browse/{key}", "label": "{key}"}]` |
| `forceLoginMethod`                 | 使用 `claudeai` 限制登入到 Claude.ai 帳戶，`console` 限制登入到 Claude Console 帳戶，或 `gateway` 限制登入到雲端閘道；請參閱 [Claude apps gateway](/docs/zh-TW/claude-apps-gateway)。在 managed 設定中設定為任何值時，由 `ANTHROPIC_API_KEY`、`ANTHROPIC_AUTH_TOKEN` 或 `apiKeyHelper` 驗證的工作階段在啟動時被阻止，因為環境認證無法滿足所需的登入方法。第三方提供者工作階段（例如 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry）不被阻止：它們針對您的雲端提供者而不是 Anthropic 進行驗證                                                                                                                                                                                                                                               | `claudeai`                                                                                                                      |
| `forceLoginGatewayUrl`             | 在 `/login` 雲端閘道畫面上預先填入並鎖定閘道 URL。此金鑰或 `forceLoginMethod: "gateway"` 會顯示該畫面；同時設定兩者以便 URL 被填入。僅在 managed 政策層級受尊重；在使用者和專案設定中被忽略。請參閱 [Claude apps gateway](/docs/zh-TW/claude-apps-gateway#set-the-gateway-url)                                                                                                                                                                                                                                                                                                                                                                                                                              | `"https://claude-gateway.example.com"`                                                                                          |
| `forceLoginOrgUUID`                | 要求登入屬於特定 Anthropic 組織。接受單一 UUID 字串（也會在登入期間預先選擇該組織），或 UUID 陣列，其中接受任何列出的組織而不預先選擇。在 managed 設定中設定時，如果驗證帳戶不屬於列出的組織，登入會失敗；由 `ANTHROPIC_API_KEY`、`ANTHROPIC_AUTH_TOKEN` 或 `apiKeyHelper` 驗證的工作階段在啟動時被阻止，因為無法驗證它們的組織成員資格。第三方提供者工作階段（例如 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry）不被阻止：使用您的雲端 IAM 限制可以使用哪些雲端帳戶。空陣列會失敗關閉並使用誤設定訊息阻止登入                                                                                                                                                                                                                                                                                 | `"xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"` 或 `["xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx", "yyyyyyyy-yyyy-yyyy-yyyy-yyyyyyyyyyyy"]`   |
| `forceRemoteSettingsRefresh`       | （Managed 設定僅限）阻止 CLI 啟動，直到從伺服器新鮮擷取遠端 managed 設定。如果擷取失敗，CLI 會結束而不是繼續使用快取或無設定。未設定時，啟動會繼續而不等待遠端設定。請參閱[失敗關閉強制執行](/docs/zh-TW/server-managed-settings#enforce-fail-closed-startup)                                                                                                                                                                                                                                                                                                                                                                                                                                                           | `true`                                                                                                                          |
| `gcpAuthRefresh`                   | 當 GCP Application Default Credentials 過期或無法載入時重新整理它們的自訂指令碼。請參閱[進階認證設定](/docs/zh-TW/google-vertex-ai#advanced-credential-configuration)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | `gcloud auth application-default login`                                                                                         |
| `hooks`                            | 設定自訂命令以在生命週期事件執行。請參閱 [hooks 文件](/docs/zh-TW/hooks)以了解格式                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | 請參閱 [hooks](/docs/zh-TW/hooks)                                                                                                       |
| `httpHookAllowedEnvVars`           | HTTP hooks 可能插入到標頭中的環境變數名稱白名單。設定時，每個 hook 的有效 `allowedEnvVars` 是與此清單的交集。未定義 = 無限制。陣列跨設定來源合併。請參閱 [Hook 設定](#hook-configuration)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | `["MY_TOKEN", "HOOK_SECRET"]`                                                                                                   |
| `includeGitInstructions`           | **預設**：`true`。在 Claude 的系統提示中包含內建提交和 PR 工作流程指示和 git 狀態快照。設定為 `false` 以移除兩者，例如在使用您自己的 git 工作流程 skills 時。`CLAUDE_CODE_DISABLE_GIT_INSTRUCTIONS` 環境變數在設定時優先於此設定                                                                                                                                                                                                                                                                                                                                                                                                                                                                       | `false`                                                                                                                         |
| `inputNeededNotifEnabled`          | **預設**：`false`。當[遠端控制](/docs/zh-TW/remote-control)已連線時，當權限提示或問題等待您的輸入時傳送推播通知到您的手機。在 `/config` 中顯示為**需要操作時推播**。請參閱[行動推播通知](/docs/zh-TW/remote-control#mobile-push-notifications)。需要 Claude Code v2.1.119 或更新版本                                                                                                                                                                                                                                                                                                                                                                                                                                | `true`                                                                                                                          |
| `language`                         | 設定 Claude 的首選回應語言（例如 `"japanese"`、`"spanish"`、`"french"`）。Claude 預設會以此語言回應。也設定[語音聽寫](/docs/zh-TW/voice-dictation#change-the-dictation-language)語言和自動產生的工作階段標題。自 v2.1.176 起，未設定時，工作階段標題符合您對話的語言                                                                                                                                                                                                                                                                                                                                                                                                                                          | `"japanese"`                                                                                                                    |
| `minimumVersion`                   | 防止背景自動更新和 `claude update` 安裝低於此版本的版本。當從 `"latest"` 頻道切換到 `"stable"` 時透過 `/config` 提示您保持在目前版本或允許降級。選擇保持設定此值。也適用於[managed 設定](/docs/zh-TW/permissions#managed-settings)以釘選組織範圍的最小值。如需完全阻止啟動的硬底線，請參閱 `requiredMinimumVersion`                                                                                                                                                                                                                                                                                                                                                                                                              | `"2.1.100"`                                                                                                                     |
| `model`                            | 覆蓋 Claude Code 使用的預設模型。`--model` 和 [`ANTHROPIC_MODEL`](/docs/zh-TW/model-config#environment-variables) 會覆蓋此設定以進行一個工作階段                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | `"claude-sonnet-5"`                                                                                                             |
| `modelOverrides`                   | 將 Anthropic 模型 ID 對應到提供者特定的模型 ID，例如 Amazon Bedrock 推論設定檔 ARN。每個模型選擇器項目在呼叫提供者 API 時使用其對應的值。請參閱[按版本覆蓋模型 ID](/docs/zh-TW/model-config#override-model-ids-per-version)                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | `{"claude-opus-4-6": "arn:aws:bedrock:..."}`                                                                                    |
| `otelHeadersHelper`                | 產生動態 OpenTelemetry 標頭的指令碼。在啟動時和定期執行。使用 [`CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`](/docs/zh-TW/env-vars) 設定重新整理間隔。請參閱[動態標頭](/docs/zh-TW/monitoring-usage#dynamic-headers)                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `/bin/generate_otel_headers.sh`                                                                                                 |
| `outputStyle`                      | 設定輸出樣式以調整系統提示。請參閱[輸出樣式文件](/docs/zh-TW/output-styles)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `"Explanatory"`                                                                                                                 |
| `parentSettingsBehavior`           | （Managed 設定僅限）**預設**：`"first-wins"`。控制由嵌入主機流程（例如 Agent SDK 或 IDE 擴充功能）以程式設計方式提供的 managed 設定在同時存在管理員部署的 managed 層級時是否適用。`"first-wins"`：父級提供的設定被丟棄，僅適用管理員層級。`"merge"`：父級提供的設定適用於管理員層級下方，經過篩選以便它們可以收緊 managed 政策但不能放鬆政策。當未部署管理員層級時無效。需要 Claude Code v2.1.133 或更新版本                                                                                                                                                                                                                                                                                                                                                                    | `"merge"`                                                                                                                       |
| `permissions`                      | 請參閱下表以了解權限的結構。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |                                                                                                                                 |
| `plansDirectory`                   | **預設**：`~/.claude/plans`。自訂 Plan Mode 檔案的儲存位置。路徑相對於專案根目錄。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | `"./plans"`                                                                                                                     |
| `pluginSuggestionMarketplaces`     | （Managed 設定僅限）其 plugins 可以作為內容相關安裝建議出現的 marketplace 名稱。沒有 marketplace 宣告的建議會出現而不需要此白名單；內建第一方前端設計提示不受影響。建議來自每個 plugin 在其 marketplace 項目中的 `relevance` 宣告。名稱僅在 marketplace 在機器上註冊且其註冊來源也在 managed 設定中宣告時才生效，作為該名稱的 `extraKnownMarketplaces` 項目或 `strictKnownMarketplaces` 的項目。從不同來源在白名單名稱下註冊的 marketplace 會被忽略。官方 marketplace 豁免於來源要求：白名單其名稱就足夠了，因為該名稱只能從官方 Anthropic 來源註冊。                                                                                                                                                                                                                                                          | `["acme-corp-plugins"]`                                                                                                         |
| `pluginTrustMessage`               | （Managed 設定僅限）在安裝前顯示的 plugin 信任警告中附加的自訂訊息。使用此選項新增組織特定的內容，例如確認來自您內部 marketplace 的 plugins 已經過審查。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `"All plugins from our marketplace are approved by IT"`                                                                         |
| `policyHelper`                     | 管理員部署的可執行檔，在啟動時動態計算 managed 設定。僅從 MDM 或系統 `managed-settings.json` 檔案受尊重。請參閱[使用政策協助程式計算 managed 設定](#compute-managed-settings-with-a-policy-helper)。需要 Claude Code v2.1.136 或更新版本                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `{"path": "/usr/local/bin/claude-policy"}`                                                                                      |
| `preferredNotifChannel`            | **預設**：`"auto"`。工作完成和權限提示通知的方法：`"auto"`、`"terminal_bell"`、`"iterm2"`、`"iterm2_with_bell"`、`"kitty"`、`"ghostty"` 或 `"notifications_disabled"`。`"auto"` 在 iTerm2、Ghostty 和 Kitty 中傳送桌面通知，在其他終端機中不執行任何操作。設定 `"terminal_bell"` 以在任何終端機中響鈴字元。在 `/config` 中顯示為**通知**。請參閱[取得終端機鈴聲或通知](/docs/zh-TW/terminal-config#get-a-terminal-bell-or-notification)                                                                                                                                                                                                                                                                                         | `"terminal_bell"`                                                                                                               |
| `prefersReducedMotion`             | 減少或停用 UI 動畫（微調器、閃爍、閃光效果）以提高可訪問性                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `true`                                                                                                                          |
| `prUrlTemplate`                    | PR 徽章的 URL 範本，顯示在頁尾和工具結果摘要中。替換 `gh` 報告的 PR URL 中的 `{host}`、`{owner}`、`{repo}`、`{number}` 和 `{url}`。使用以指向內部程式碼審查工具而不是 `github.com`。不影響 Claude 散文中的 `#123` 自動連結                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | `"https://reviews.example.com/{owner}/{repo}/pull/{number}"`                                                                    |
| `remoteControlAtStartup`           | 當每個互動式工作階段啟動時自動連線[遠端控制](/docs/zh-TW/remote-control)，而不是等待 `/remote-control`。設定為 `true` 以始終自動連線，`false` 以永不自動連線，或保留未設定以遵循您組織的預設。在 `/config` 中顯示為**為所有工作階段啟用遠端控制**。請參閱[為所有工作階段啟用遠端控制](/docs/zh-TW/remote-control#enable-remote-control-for-all-sessions)                                                                                                                                                                                                                                                                                                                                                                                       | `false`                                                                                                                         |
| `requiredMaximumVersion`           | Managed 設定僅限。允許啟動的最大 Claude Code 版本。如果執行中的版本較新，Claude Code 會在啟動時結束並指示使用者透過組織的核准方法安裝核准的版本；`claude install <version>` 也可能有效。背景自動更新和 `claude update` 會跳過高於上限的版本，因此在範圍內的安裝保持在範圍內。`claude update`、`claude install` 和 `claude doctor` 在上限以上保持工作，以便使用者可以恢復。早於此設定的版本會忽略它                                                                                                                                                                                                                                                                                                                                                                   | `"2.1.150"`                                                                                                                     |
| `requiredMinimumVersion`           | Managed 設定僅限。啟動所需的最小 Claude Code 版本。如果執行中的版本較舊，Claude Code 會在啟動時結束並指示使用者透過組織的核准方法更新。`claude update`、`claude install` 和 `claude doctor` 在底線以下保持工作，以便使用者可以恢復。與 `minimumVersion` 不同，後者防止降級但永遠不會阻止啟動。早於此設定的版本會忽略它                                                                                                                                                                                                                                                                                                                                                                                                                      | `"2.1.150"`                                                                                                                     |
| `respectGitignore`                 | **預設**：`true`。控制 `@` 檔案選擇器是否尊重 `.gitignore` 模式。當為 `true` 時，符合 `.gitignore` 模式的檔案會從建議中排除                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            | `false`                                                                                                                         |
| `respondToBashCommands`            | **預設**：`true`。輸入框 `!` shell 命令執行後 Claude 是否回應。設定為 `false` 以將命令輸出新增到內容而不回應。請參閱[使用 `!` 前綴的 Shell 模式](/docs/zh-TW/interactive-mode#shell-mode-with-prefix)。需要 Claude Code v2.1.186 或更新版本                                                                                                                                                                                                                                                                                                                                                                                                                                                   | `false`                                                                                                                         |
| `showClearContextOnPlanAccept`     | **預設**：`false`。在 Plan Mode 接受畫面上顯示「清除內容」選項。設定為 `true` 以還原選項                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | `true`                                                                                                                          |
| `showThinkingSummaries`            | **預設**：`false`。在互動式工作階段中顯示[擴展思考](/docs/zh-TW/model-config#extended-thinking)摘要。未設定或 `false` 時，思考區塊由 API 編輯並顯示為摺疊的存根。編輯只會改變您看到的內容，而不是模型生成的內容：若要減少思考支出，請[降低預算或停用思考](/docs/zh-TW/model-config#extended-thinking)。此設定在非互動模式（`-p`）、Agent SDK 或 IDE 擴充功能（例如 VS Code）中無效                                                                                                                                                                                                                                                                                                                                                                            | `true`                                                                                                                          |
| `showTurnDuration`                 | **預設**：`true`。在回應後顯示輪次持續時間訊息，例如「Cooked for 1m 6s」。在 `/config` 中顯示為**顯示輪次持續時間**                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | `false`                                                                                                                         |
| `skillListingBudgetFraction`       | **預設**：`0.01`。為 Claude 每輪看到的[skill 清單](/docs/zh-TW/skills#skill-descriptions-are-cut-short)保留的模型內容視窗分數。當清單超過預算時，最少使用的 skills 的描述會摺疊為裸名稱，以便 Claude 仍可叫用它們，但不會看到原因。提高以保持更多描述可見，代價是每輪更多內容。`/doctor` 估計清單成本對預算                                                                                                                                                                                                                                                                                                                                                                                                                              | `0.02`                                                                                                                          |
| `skillListingMaxDescChars`         | **預設**：`1536`。[skill 清單](/docs/zh-TW/skills#skill-descriptions-are-cut-short)中每個 skill 的字元上限，Claude 每輪看到的 `description` 和 `when_to_use` 文字的組合。超過此長度的文字會被截斷。提高以保持長描述完整，代價是每輪更多內容；降低以在 [`skillListingBudgetFraction`](#available-settings) 下適應更多 skills                                                                                                                                                                                                                                                                                                                                                                                   | `2048`                                                                                                                          |
| `skillOverrides`                   | 按 skill 名稱鍵入的每個 skill 可見性覆蓋。值為 `"on"`、`"name-only"`、`"user-invocable-only"` 或 `"off"`。讓您隱藏或摺疊 skill 而無需編輯其 SKILL.md。不適用於 plugin skills，這些由 `/plugin` 管理。`/skills` 功能表將這些寫入 `.claude/settings.local.json`。請參閱[從設定覆蓋 skill 可見性](/docs/zh-TW/skills#override-skill-visibility-from-settings)。需要 Claude Code v2.1.129 或更新版本                                                                                                                                                                                                                                                                                                                 | `{"legacy-context": "name-only", "deploy": "off"}`                                                                              |
| `skipWebFetchPreflight`            | 跳過[WebFetch 網域安全檢查](/docs/zh-TW/data-usage#webfetch-domain-safety-check)，該檢查在擷取前將每個請求的主機名稱傳送到 `api.anthropic.com`。在阻止流量到 Anthropic 的環境中設定為 `true`，例如 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 部署，具有限制性的出站。跳過時，WebFetch 嘗試任何 URL 而不諮詢黑名單                                                                                                                                                                                                                                                                                                                                                                         | `true`                                                                                                                          |
| `spinnerTipsEnabled`               | **預設**：`true`。在 Claude 工作時在微調器中顯示提示。設定為 `false` 以停用提示                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | `false`                                                                                                                         |
| `spinnerTipsOverride`              | 使用自訂字串覆蓋微調器提示。`tips`：提示字串陣列。`excludeDefault`：如果為 `true`，僅顯示自訂提示；如果為 `false` 或不存在，自訂提示會與內建提示合併                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | `{ "excludeDefault": true, "tips": ["Use our internal tool X"] }`                                                               |
| `spinnerVerbs`                     | 自訂在微調器中顯示的動作動詞。將 `mode` 設定為 `"replace"` 以僅使用您的動詞，或 `"append"` 以將它們新增到預設值                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           | `{"mode": "append", "verbs": ["Pondering", "Crafting"]}`                                                                        |
| `sshConfigs`                       | 要在[桌面](/docs/zh-TW/desktop#pre-configure-ssh-connections-for-your-team)環境下拉式清單中顯示的 SSH 連線。每個項目需要 `id`、`name` 和 `sshHost`；`sshPort`、`sshIdentityFile` 和 `startDirectory` 是選用的。在 managed 設定中設定時，連線對使用者是唯讀的。僅從 managed 和使用者設定讀取                                                                                                                                                                                                                                                                                                                                                                                                            | `[{"id": "dev-vm", "name": "Dev VM", "sshHost": "user@dev.example.com"}]`                                                       |
| `statusLine`                       | 設定自訂狀態行以顯示內容。物件的選用 `padding`、`refreshInterval` 和 `hideVimModeIndicator` 欄位控制間距、定期重新執行和是否隱藏提示下方的內建 vim 模式指示器。請參閱 [`statusLine` 文件](/docs/zh-TW/statusline#manually-configure-a-status-line)                                                                                                                                                                                                                                                                                                                                                                                                                                              | `{"type": "command", "command": "~/.claude/statusline.sh"}`                                                                     |
| `strictKnownMarketplaces`          | （Managed 設定僅限）plugin marketplaces 白名單。未定義 = 無限制，空陣列 = 鎖定。在 marketplace 新增和 plugin 安裝、更新、重新整理和自動更新時強制執行，因此在設定政策之前新增的 marketplace 無法用於擷取 plugins。請參閱 [Managed marketplace 限制](/docs/zh-TW/plugin-marketplaces#managed-marketplace-restrictions)                                                                                                                                                                                                                                                                                                                                                                                           | `[{ "source": "github", "repo": "acme-corp/plugins" }]`                                                                         |
| `strictPluginOnlyCustomization`    | （Managed 設定僅限）阻止 skills、agents、hooks 和 MCP servers 來自使用者和專案來源，因此它們只能來自 plugins 或 managed 設定。`true` 鎖定所有四個表面；陣列僅鎖定命名的表面。請參閱 [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)                                                                                                                                                                                                                                                                                                                                                                                                                                       | `["skills", "hooks"]`                                                                                                           |
| `syntaxHighlightingDisabled`       | 停用 diffs、程式碼區塊和檔案預覽中的語法醒目提示                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        | `true`                                                                                                                          |
| `teammateMode`                     | **預設**：`in-process`。[agent team](/docs/zh-TW/agent-teams) 隊友的顯示方式：`in-process`、`auto`（在 tmux 或 iTerm2 中選擇分割窗格，否則為進程內）、`tmux`（使用 tmux 或 iTerm2 選擇分割窗格，從您的終端機偵測）或 }`iterm2`（iTerm2 原生分割窗格透過 `it2` CLI，在 v2.1.186 中新增）。預設在 v2.1.179 中從 `auto` 變更。`--teammate-mode` 會覆蓋此設定以進行一個工作階段。請參閱[選擇顯示模式](/docs/zh-TW/agent-teams#choose-a-display-mode)                                                                                                                                                                                                                                                                                                   | `"auto"`                                                                                                                        |
| `terminalProgressBarEnabled`       | **預設**：`true`。在支援的終端機中顯示終端機進度條：ConEmu、Ghostty 1.2.0+ 和 iTerm2 3.6.6+。在 `/config` 中顯示為**終端機進度條**                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    | `false`                                                                                                                         |
| `theme`                            | **預設**：`"dark"`。介面的色彩主題：`"auto"`、`"dark"`、`"light"`、`"dark-daltonized"`、`"light-daltonized"`、`"dark-ansi"`、`"light-ansi"` 或自訂主題參考，例如 `"custom:<slug>"` 或 `"custom:<plugin-name>:<slug>"`。請參閱[建立自訂主題](/docs/zh-TW/terminal-config#create-a-custom-theme)。在 `/config` 中顯示為**主題**                                                                                                                                                                                                                                                                                                                                                          | `"dark"`                                                                                                                        |
| `tui`                              | 終端機 UI 渲染器。使用 `"fullscreen"` 以取得無閃爍[替代螢幕渲染器](/docs/zh-TW/fullscreen)，具有虛擬化捲軸。使用 `"default"` 以取得經典主螢幕渲染器。透過 `/tui` 設定。您也可以設定 [`CLAUDE_CODE_NO_FLICKER`](/docs/zh-TW/env-vars) 環境變數。背景工作階段從[代理檢視](/docs/zh-TW/agent-view)開啟時，無論此設定如何，始終使用全螢幕渲染器                                                                                                                                                                                                                                                                                                                                                                                                     | `"fullscreen"`                                                                                                                  |
| `ultracode`                        | 為工作階段開啟 [ultracode](/docs/zh-TW/workflows#let-claude-decide-with-ultracode)。此金鑰不從 `settings.json` 讀取。透過 `/effort ultracode`、`--settings` 或 Agent SDK 控制請求設定。若要以 ultracode 已開啟的狀態啟動工作階段，請使用 `claude --effort ultracode` 啟動，需要 Claude Code v2.1.203 或更新版本                                                                                                                                                                                                                                                                                                                                                                                 | `true`                                                                                                                          |
| `useAutoModeDuringPlan`            | **預設**：`true`。Plan Mode 在自動模式可用時是否使用自動模式語義。不從共享專案設定讀取。在 `/config` 中顯示為「在計畫期間使用自動模式」                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `false`                                                                                                                         |
| `verbose`                          | **預設**：`false`。顯示完整工具輸出而不是截斷摘要。在 `/config` 中顯示為**詳細輸出**。`--verbose` 旗標會覆蓋此設定以進行一個工作階段                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | `true`                                                                                                                          |
| `viewMode`                         | 啟動時的預設文字記錄檢視模式：`"default"`、`"verbose"` 或 `"focus"`。設定時覆蓋粘性 `/focus` 選擇。`--verbose` 旗標會覆蓋此設定以進行一個工作階段                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | `"verbose"`                                                                                                                     |
| `vimInsertModeRemaps`              | 將兩個按鍵 INSERT 模式序列對應到 Escape 在[vim 編輯器模式](/docs/zh-TW/interactive-mode#vim-editor-mode)中。每個金鑰恰好是兩個按順序輸入的可列印字元，`"<Esc>"` 是唯一支援的目標；其他項目被忽略。僅從使用者、`--settings` 旗標和 managed 設定讀取，因此儲存庫的簽入設定無法重新對應您的按鍵。除非 `editorMode` 為 `"vim"`，否則無效。請參閱[重新對應 INSERT 模式按鍵序列](/docs/zh-TW/interactive-mode#remap-insert-mode-key-sequences)。需要 Claude Code v2.1.208 或更新版本                                                                                                                                                                                                                                                                                          | `{"jj": "<Esc>"}`                                                                                                               |
| `voice`                            | [語音聽寫](/docs/zh-TW/voice-dictation)設定：`enabled` 開啟聽寫，`mode` 選擇 `"hold"` 或 `"tap"`，`autoSubmit` 在保持模式中按鍵釋放時傳送提示。當您執行 `/voice` 時自動寫入。需要 Claude.ai 帳戶                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | `{ "enabled": true, "mode": "tap" }`                                                                                            |
| `voiceEnabled`                     | `voice.enabled` 的舊版別名。偏好 `voice` 物件                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                | `true`                                                                                                                          |
| `wheelScrollAccelerationEnabled`   | **預設**：`true`。在[全螢幕渲染](/docs/zh-TW/fullscreen#mouse-wheel-scrolling)中，加速滑鼠滾輪捲軸速度在快速捲軸期間。設定為 `false` 以取得每個滾輪缺口的恆定捲軸速率。需要 Claude Code v2.1.174 或更新版本                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      | `false`                                                                                                                         |
| `workflowKeywordTriggerEnabled`    | **預設**：`true`。提示中的單詞 `ultracode` 是否觸發[動態工作流程](/docs/zh-TW/workflows#ask-for-a-workflow-in-your-prompt)。設定為 `false` 以輸入單詞而不觸發一個。Ultracode 努力設定、`/workflows` 和儲存的工作流程命令不受影響。在 `/config` 中顯示為**Ultracode 關鍵字觸發**。在 v2.1.160 之前，觸發關鍵字是 `workflow`                                                                                                                                                                                                                                                                                                                                                                                           | `false`                                                                                                                         |
| `wslInheritsWindowsSettings`       | （Windows managed 設定僅限）當為 `true` 時，WSL 上的 Claude Code 除了 `/etc/claude-code` 外還會從 Windows 政策鏈讀取 managed 設定，Windows 來源優先。僅在 HKLM 登錄機碼或 `C:\Program Files\ClaudeCode\managed-settings.json` 中設定時受尊重，兩者都需要 Windows 管理員才能寫入。為了讓 HKCU 政策也在 WSL 上適用，旗標必須另外在 HKCU 本身中設定。對原生 Windows 無效                                                                                                                                                                                                                                                                                                                                                      | `true`                                                                                                                          |

<h3 id="global-config-settings">
  全域設定設定
</h3>

這些設定儲存在 `~/.claude.json` 中，而不是 `settings.json`。將它們新增到 `settings.json` 將觸發架構驗證錯誤。

<Note>
  v2.1.119 之前的版本也會在此處儲存許多 `/config` 偏好金鑰，而不是在 `settings.json` 中，包括 `theme`、`verbose`、`editorMode`、`autoCompactEnabled` 和 `preferredNotifChannel`。
</Note>

| 金鑰                           | 說明                                                                                                                                                                                                                                                                                                                                                                                               | 範例         |
| :--------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------- |
| `autoConnectIde`             | **預設**：`false`。當 Claude Code 從外部終端機啟動時自動連線到執行中的 IDE。在 VS Code 或 JetBrains 終端機外執行時在 `/config` 中顯示為**自動連線到 IDE（外部終端機）**。[`CLAUDE_CODE_AUTO_CONNECT_IDE`](/docs/zh-TW/env-vars) 環境變數在設定時會覆蓋此設定                                                                                                                                                                                                           | `true`     |
| `autoInstallIdeExtension`    | **預設**：`true`。從 VS Code 終端機執行時自動安裝 Claude Code IDE 擴充功能。在 VS Code 或 JetBrains 終端機內執行時在 `/config` 中顯示為**自動安裝 IDE 擴充功能**。您也可以設定 [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/zh-TW/env-vars) 環境變數                                                                                                                                                                                                        | `false`    |
| `externalEditorContext`      | **預設**：`false`。當您使用 `Ctrl+G` 開啟外部編輯器時，將 Claude 的前一個回應作為 `#` 註解內容前置。在 `/config` 中顯示為**在外部編輯器中顯示最後回應**                                                                                                                                                                                                                                                                                             | `true`     |
| `permissionExplainerEnabled` | **預設**：`true`。當您在 Bash 或 PowerShell 權限提示上按 `Ctrl+E` 時顯示模型產生的[命令說明](/docs/zh-TW/permissions#permission-system)。設定為 `false` 以關閉快捷鍵                                                                                                                                                                                                                                                                      | `false`    |
| `teammateDefaultModel`       | [agent team](/docs/zh-TW/agent-teams) 隊友在生成提示未指定時的預設模型。設定為模型別名（例如 `"sonnet"`），或 `null` 以繼承主管的目前 `/model` 選擇。在 `/config` 中顯示為**預設隊友模型**                                                                                                                                                                                                                                                                | `"sonnet"` |
| `workflowSizeGuideline`      | **預設**：`unrestricted`，不傳送任何指南。設定 Claude 在其撰寫的動態工作流程中目標的[代理計數](/docs/zh-TW/workflows#set-a-size-guideline)。Claude Code 將值作為建議而不是強制上限傳送給 Claude。接受 `unrestricted`、`small`、`medium` 或 `large`。在 `/config` 中顯示為**動態工作流程大小**。您也可以使用 `/config workflowSizeGuideline=small` 直接設定它。需要 Claude Code v2.1.202 或更新版本。指南的代理計數也替換[`Large workflow` 警告](/docs/zh-TW/workflows#cost)的預設閾值；該行為需要 Claude Code v2.1.203 或更新版本 | `"small"`  |

<h3 id="worktree-settings">
  Worktree 設定
</h3>

設定 `--worktree` 如何建立和管理 git worktrees。

| 金鑰                            | 說明                                                                                                                                                                                                                                                                                                                   | 範例                                    |
| :---------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------ |
| `worktree.baseRef`            | 新 worktrees 分支的來源 ref。`"fresh"`（預設）從 `origin/<default-branch>` 分支以取得與遠端相符的乾淨樹。`"head"` 從您目前的本機 `HEAD` 分支，所以未推送的提交和功能分支狀態存在於 worktree 中。在連結的 worktree 內，`"head"` 解析為該 worktree 的 `HEAD`，而不是主簽出的。適用於 `--worktree`、`EnterWorktree` 工具和 subagent 隔離                                                                      | `"head"`                              |
| `worktree.symlinkDirectories` | 要從主儲存庫符號連結到每個 worktree 的目錄，以避免在磁碟上複製大型目錄。預設不符號連結任何目錄                                                                                                                                                                                                                                                                 | `["node_modules", ".cache"]`          |
| `worktree.sparsePaths`        | 要在每個 worktree 中透過 git sparse-checkout 簽出的目錄。僅將列出的目錄加上根層級檔案寫入磁碟，在大型 monorepos 中速度更快。當稀疏 worktree 存在時，git 在儲存庫的共享 `.git/config` 中啟用 `extensions.worktreeConfig`；請參閱[僅簽出您需要的目錄](/docs/zh-TW/large-codebases#check-out-only-the-directories-you-need)                                                                         | `["packages/my-app", "shared/utils"]` |
| `worktree.bgIsolation`        | [背景工作階段](/docs/zh-TW/agent-view#how-file-edits-are-isolated)的隔離模式。`"worktree"`（預設）在呼叫 `EnterWorktree` 之前阻止主簽出中的 `Edit`/`Write`。在 git 儲存庫外，失敗的 [`WorktreeCreate` hook](/docs/zh-TW/worktrees#non-git-version-control) 會釋放區塊，以便工作階段可以就地編輯工作目錄；需要 Claude Code v2.1.203 或更新版本。`"none"` 讓背景工作直接編輯工作副本。需要 Claude Code v2.1.143 或更新版本 | `"none"`                              |

若要將 gitignored 檔案（如 `.env`）複製到新的 worktrees，請改用專案根目錄中的 [`.worktreeinclude` 檔案](/docs/zh-TW/worktrees#copy-gitignored-files-into-worktrees)，而不是設定。

<h3 id="permission-settings">
  權限設定
</h3>

| 金鑰                                  | 說明                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | 範例                                                                     |
| :---------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------- |
| `allow`                             | 允許工具使用的權限規則陣列。工具名稱 globs 僅在字面 `mcp__<server>__` 前綴之後的工具位置支援，例如 `mcp__github__get_*`；server 段必須無 glob。請參閱下面的[權限規則語法](#permission-rule-syntax)以了解模式匹配詳細資訊                                                                                                                                                                                                                                                                                                                             | `[ "Bash(git diff *)" ]`                                               |
| `ask`                               | 要求在工具使用時確認的權限規則陣列。請參閱下面的[權限規則語法](#permission-rule-syntax)                                                                                                                                                                                                                                                                                                                                                                                                                           | `[ "Bash(git push *)" ]`                                               |
| `deny`                              | 拒絕工具使用的權限規則陣列。使用此選項從 Claude Code 存取中排除敏感檔案。工具名稱接受 glob 模式：`"*"` 拒絕每個工具，`"mcp__*"` 拒絕所有 MCP 工具。請參閱[權限規則語法](#permission-rule-syntax)和 [Bash 權限限制](/docs/zh-TW/permissions#tool-specific-permission-rules)                                                                                                                                                                                                                                                                                  | `[ "WebFetch", "Bash(curl *)", "Read(./.env)", "Read(./secrets/**)" ]` |
| `additionalDirectories`             | Claude 有權存取的其他[工作目錄](/docs/zh-TW/permissions#working-directories)。大多數 `.claude/` 設定[未從這些目錄發現](/docs/zh-TW/permissions#additional-directories-grant-file-access-not-configuration)                                                                                                                                                                                                                                                                                                             | `[ "../docs/" ]`                                                       |
| `defaultMode`                       | 開啟 Claude Code 時的預設[權限模式](/docs/zh-TW/permission-modes)。有效值：`default`、`acceptEdits`、`plan`、`auto`、`dontAsk`、`bypassPermissions` 和 }`manual` 作為 `default` 的別名，CLI 和 VS Code 和 JetBrains 擴充功能中標記為 Manual 的模式。`manual` 別名需要 Claude Code v2.1.200 或更新版本。自 Claude Code v2.1.142 起，當在專案或本機設定（`.claude/settings.json`、`.claude/settings.local.json`）中設定時，`auto` 會被忽略，因此儲存庫無法授予自己自動模式。改為在 `~/.claude/settings.json` 中設定它。在 v2.1.142 之前，專案設定可以設定 `auto`。`--permission-mode` CLI 旗標會覆蓋此設定以進行單一工作階段 | `"acceptEdits"`                                                        |
| `disableBypassPermissionsMode`      | 設定為 `"disable"` 以防止啟用 `bypassPermissions` 模式。這會停用 `--dangerously-skip-permissions` 旗標。在[managed 設定](/docs/zh-TW/permissions#managed-settings)中最有用，使用者無法覆蓋它                                                                                                                                                                                                                                                                                                                               | `"disable"`                                                            |
| `skipDangerousModePermissionPrompt` | 跳過透過 `--dangerously-skip-permissions` 或 `defaultMode: "bypassPermissions"` 進入 bypass permissions 模式之前顯示的確認提示。在專案設定（`.claude/settings.json`）中設定時被忽略，以防止不受信任的儲存庫自動繞過提示                                                                                                                                                                                                                                                                                                                | `true`                                                                 |

<h3 id="permission-rule-syntax">
  權限規則語法
</h3>

權限規則遵循 `Tool` 或 `Tool(specifier)` 的格式。規則按順序評估：首先是拒絕規則，然後是詢問，最後是允許。第一個匹配的規則決定結果，無論規則特異性如何。請參閱[權限規則評估順序](/docs/zh-TW/permissions#manage-permissions)以了解詳細資訊。

快速範例：

| 規則                             | 效果                    |
| :----------------------------- | :-------------------- |
| `Bash`                         | 符合所有 Bash 命令          |
| `Bash(npm run *)`              | 符合以 `npm run` 開頭的命令   |
| `Read(./.env)`                 | 符合讀取 `.env` 檔案        |
| `WebFetch(domain:example.com)` | 符合對 example.com 的擷取請求 |

如需完整的規則語法參考，包括萬用字元行為、Read、Edit、WebFetch、MCP 和 Agent 規則的工具特定模式，以及 Bash 模式的安全限制，請參閱[權限規則語法](/docs/zh-TW/permissions#permission-rule-syntax)。

<h3 id="sandbox-settings">
  Sandbox 設定
</h3>

設定進階 sandboxing 行為。Sandboxing 將 bash 命令與您的檔案系統和網路隔離。請參閱 [Sandboxing](/docs/zh-TW/sandboxing) 以了解詳細資訊。

| 金鑰                                     | 說明                                                                                                                                                                                                                                                                                                                                                                                                                                                                              | 範例                                                   |
| :------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------- |
| `enabled`                              | 啟用 bash sandboxing（macOS、Linux 和 WSL2）。預設：false                                                                                                                                                                                                                                                                                                                                                                                                                                 | `true`                                               |
| `failIfUnavailable`                    | 如果 `sandbox.enabled` 為 true 但 sandbox 無法啟動（遺失相依性、不支援的平台），則在啟動時以錯誤結束。當為 false（預設）時，會顯示警告，命令會以 unsandboxed 方式執行。適用於需要 sandboxing 作為硬閘門的 managed 設定部署                                                                                                                                                                                                                                                                                                                              | `true`                                               |
| `autoAllowBashIfSandboxed`             | 在 sandboxed 時自動批准 bash 命令。預設：true                                                                                                                                                                                                                                                                                                                                                                                                                                               | `true`                                               |
| `excludedCommands`                     | 應在 sandbox 外執行的命令                                                                                                                                                                                                                                                                                                                                                                                                                                                               | `["docker *"]`                                       |
| `allowUnsandboxedCommands`             | 允許命令透過 `dangerouslyDisableSandbox` 參數在 sandbox 外執行。當設定為 `false` 時，`dangerouslyDisableSandbox` 逃脫艙口完全停用，所有命令必須 sandboxed（或在 `excludedCommands` 中）。適用於需要嚴格 sandboxing 的企業政策。預設：true                                                                                                                                                                                                                                                                                               | `false`                                              |
| `filesystem.allowWrite`                | sandboxed 命令可以寫入的其他路徑。陣列跨所有設定範圍合併：使用者、專案和 managed 路徑合併，不替換。也與 `Edit(...)` 允許權限規則中的路徑合併。請參閱下面的[路徑前綴](#sandbox-path-prefixes)。                                                                                                                                                                                                                                                                                                                                                    | `["/tmp/build", "~/.kube"]`                          |
| `filesystem.denyWrite`                 | sandboxed 命令無法寫入的路徑。陣列跨所有設定範圍合併。也與 `Edit(...)` 拒絕權限規則中的路徑合併。                                                                                                                                                                                                                                                                                                                                                                                                                    | `["/etc", "/usr/local/bin"]`                         |
| `filesystem.denyRead`                  | sandboxed 命令無法讀取的路徑。陣列跨所有設定範圍合併。也與 `Read(...)` 拒絕權限規則中的路徑合併。                                                                                                                                                                                                                                                                                                                                                                                                                    | `["~/.aws/credentials"]`                             |
| `filesystem.allowRead`                 | 在 `denyRead` 區域內重新允許讀取的路徑。`allowRead` 路徑在更廣泛的 `denyRead` 區域內重新開啟讀取，`denyRead` 中的精確路徑在更廣泛的 `allowRead` 內保持被阻止；請參閱[重疊表](/docs/zh-TW/sandboxing#configure-sandboxing)以了解範例。陣列跨所有設定範圍合併。使用此選項建立僅工作區讀取存取模式。                                                                                                                                                                                                                                                                               | `["."]`                                              |
| `filesystem.allowManagedReadPathsOnly` | （Managed 設定僅限）僅尊重 managed 設定中的 `filesystem.allowRead` 路徑。`denyRead` 仍從所有來源合併。預設：false                                                                                                                                                                                                                                                                                                                                                                                           | `true`                                               |
| `credentials.files`                    | Sandboxed 命令無法讀取的認證檔案或目錄。應用與 `filesystem.denyRead` 相同的讀取區塊；單獨的金鑰將認證路徑與 `credentials.envVars` 分組，並與一般檔案系統規則分開。每個項目為 `{ "path": "...", "mode": "deny" }`，僅支援 `deny`。路徑使用與 `filesystem.*` 設定相同的[前綴](#sandbox-path-prefixes)。陣列跨所有設定範圍合併。需要 Claude Code v2.1.187 或更新版本。                                                                                                                                                                                                             | `[{ "path": "~/.aws/credentials", "mode": "deny" }]` |
| `credentials.envVars`                  | 環境變數以[保護免受 sandboxed 命令](/docs/zh-TW/sandboxing#protect-credentials)。每個項目有一個 `name` 和一個 `mode`；名稱必須以字母或底線開頭，並僅包含字母、數字和底線。`deny` 從 sandboxed 命令的環境中移除變數。需要 Claude Code v2.1.187 或更新版本。}`mask` 在 sandbox 內用每個工作階段的 sentinel 值替換變數，而 sandbox 代理在該項目的 `injectHosts` 的出站請求上替換真實值；它需要 `network.tlsTerminate` 和 Claude Code v2.1.199 或更新版本。`mask` 項目僅從使用者、managed 或 CLI `--settings` 設定受尊重，不從 `.claude/settings.json` 或 `.claude/settings.local.json`。陣列跨所有設定範圍合併，當相同變數同時出現兩種模式時 `deny` 優先。 | `[{ "name": "GITHUB_TOKEN", "mode": "deny" }]`       |
| `credentials.envVars[].injectHosts`    | Sandbox 代理替換 `mask` 項目真實值的主機。每個主機也必須由 `network.allowedDomains` 涵蓋，無論是精確還是通過萬用字元。未設定時，代理在 `network.allowedDomains` 中的每個主機上替換值。當 `mode` 為 `deny` 時被接受但忽略。需要 Claude Code v2.1.199 或更新版本。}                                                                                                                                                                                                                                                                                          | `["api.github.com"]`                                 |
| `credentials.allowPlaintextInject`     | 允許 `mask` 替換在純 HTTP 請求以及 TLS 終止的 HTTPS 上。在純 HTTP 上，上游身份未驗證，認證以明文形式傳輸，因此在受信任的測試網路外保持此設定關閉。僅從使用者、managed 或 CLI `--settings` 設定受尊重，不從 `.claude/settings.json` 或 `.claude/settings.local.json`。預設：false。需要 Claude Code v2.1.199 或更新版本。}                                                                                                                                                                                                                                             | `true`                                               |
| `network.allowUnixSockets`             | （macOS 僅限）sandbox 中可存取的 Unix socket 路徑。在 Linux 和 WSL2 上被忽略，其中 seccomp 篩選器無法檢查 socket 路徑；改用 `allowAllUnixSockets`。                                                                                                                                                                                                                                                                                                                                                               | `["~/.ssh/agent-socket"]`                            |
| `network.allowAllUnixSockets`          | 允許 sandbox 中的所有 Unix socket 連線。在 Linux 和 WSL2 上，這是允許 Unix sockets 的唯一方式，因為它跳過了 seccomp 篩選器，否則會阻止 `socket(AF_UNIX, ...)` 呼叫。預設：false                                                                                                                                                                                                                                                                                                                                             | `true`                                               |
| `network.allowLocalBinding`            | 允許繫結到 localhost 連接埠（macOS 僅限）。預設：false                                                                                                                                                                                                                                                                                                                                                                                                                                          | `true`                                               |
| `network.allowMachLookup`              | sandbox 可能查詢的其他 XPC/Mach 服務名稱（macOS 僅限）。支援單一尾部 `*` 用於前綴匹配。iOS 模擬器或 Playwright 等透過 XPC 通訊的工具需要。                                                                                                                                                                                                                                                                                                                                                                                  | `["com.apple.coresimulator.*"]`                      |
| `network.allowedDomains`               | 允許出站網路流量的網域陣列。支援萬用字元（例如 `*.example.com`）。                                                                                                                                                                                                                                                                                                                                                                                                                                       | `["github.com", "*.npmjs.org"]`                      |
| `network.deniedDomains`                | 允許阻止出站網路流量的網域陣列。支援與 `allowedDomains` 相同的萬用字元語法。當兩者都符合時優先於 `allowedDomains`。無論 `allowManagedDomainsOnly` 如何，都從所有設定來源合併。                                                                                                                                                                                                                                                                                                                                                          | `["sensitive.cloud.example.com"]`                    |
| `network.allowManagedDomainsOnly`      | （Managed 設定僅限）僅尊重 managed 設定中的 `allowedDomains` 和 `WebFetch(domain:...)` 允許規則。來自使用者、專案和本機設定的網域會被忽略。非允許的網域會自動阻止，不會提示使用者。拒絕的網域仍從所有來源受尊重。預設：false                                                                                                                                                                                                                                                                                                                                  | `true`                                               |
| `network.httpProxyPort`                | 如果您想帶上自己的代理，使用的 HTTP 代理連接埠。如果未指定，Claude 將執行自己的代理。                                                                                                                                                                                                                                                                                                                                                                                                                               | `8080`                                               |
| `network.socksProxyPort`               | 如果您想帶上自己的代理，使用的 SOCKS5 代理連接埠。如果未指定，Claude 將執行自己的代理。                                                                                                                                                                                                                                                                                                                                                                                                                             | `8081`                                               |
| `network.tlsTerminate`                 | 實驗性。在 sandbox 代理內終止 TLS，以便它可以讀取 HTTPS 請求的內容。[credential substitution](/docs/zh-TW/sandboxing#protect-credentials) 的 `mask` 需要。設定 `{}` 以為工作階段產生臨時憑證授權單位，或設定 `caCertPath` 和 `caKeyPath` 以使用您自己的。僅從使用者、managed 或 CLI `--settings` 設定受尊重，不從 `.claude/settings.json` 或 `.claude/settings.local.json`。需要 Claude Code v2.1.199 或更新版本。}                                                                                                                                                      | `{}`                                                 |
| `enableWeakerNestedSandbox`            | 為無特權 Docker 環境啟用較弱的 sandbox（Linux 和 WSL2 僅限）。**降低安全性。** 預設：false                                                                                                                                                                                                                                                                                                                                                                                                                | `true`                                               |
| `enableWeakerNetworkIsolation`         | （macOS 僅限）允許在 sandbox 中存取系統 TLS 信任服務（`com.apple.trustd.agent`）。使用 `httpProxyPort` 和自訂 CA 的 MITM 代理時，Go 型工具（如 `gh`、`gcloud` 和 `terraform`）需要驗證 TLS 憑證。**透過開啟潛在的資料外洩路徑降低安全性**。預設：false                                                                                                                                                                                                                                                                                            | `true`                                               |
| `allowAppleEvents`                     | （macOS 僅限）允許 sandboxed 命令傳送 Apple Events。`open`、`osascript` 和在瀏覽器中開啟 URL 的工具需要，否則會失敗並出現錯誤 `-600`。**移除程式碼執行隔離。** Sandboxed 命令可以啟動其他應用程式 unsandboxed，無需使用者提示；它們也可以傳送 AppleScript 命令到執行中的應用程式（例如終端機），受限於每個應用程式的 macOS 自動化同意提示 (TCC)。僅從使用者、managed 或 CLI 設定受尊重，不從專案設定。預設：false                                                                                                                                                                                                      | `true`                                               |
| `bwrapPath`                            | （Managed 設定僅限，Linux/WSL2）bubblewrap (`bwrap`) 二進位檔的絕對路徑。覆蓋透過 `PATH` 的自動偵測。僅從 [managed 設定](/docs/zh-TW/settings#settings-files)受尊重，不從使用者或專案設定。在 managed 環境中 `bwrap` 安裝在非標準位置時很有用。                                                                                                                                                                                                                                                                                                     | `/opt/admin/bwrap`                                   |
| `socatPath`                            | （Managed 設定僅限，Linux/WSL2）用於 sandbox 網路代理的 `socat` 二進位檔的絕對路徑。覆蓋透過 `PATH` 的自動偵測。僅從 managed 設定受尊重。                                                                                                                                                                                                                                                                                                                                                                                 | `/opt/admin/socat`                                   |

<h4 id="sandbox-path-prefixes">
  Sandbox 路徑前綴
</h4>

`filesystem.allowWrite`、`filesystem.denyWrite`、`filesystem.denyRead`、`filesystem.allowRead` 和 `credentials.files` 中的路徑支援這些前綴：

| 前綴        | 含義                                        | 範例                                                                |
| :-------- | :---------------------------------------- | :---------------------------------------------------------------- |
| `/`       | 從檔案系統根目錄的絕對路徑                             | `/tmp/build` 保持 `/tmp/build`                                      |
| `~/`      | 相對於主目錄                                    | `~/.kube` 變成 `$HOME/.kube`                                        |
| `./` 或無前綴 | 相對於專案根目錄（用於專案設定）或相對於 `~/.claude`（用於使用者設定） | `./output` 在 `.claude/settings.json` 中解析為 `<project-root>/output` |

較舊的 `//path` 前綴用於絕對路徑仍然有效。如果您之前使用單斜線 `/path` 期望專案相對解析，請切換到 `./path`。此語法與[讀取和編輯權限規則](/docs/zh-TW/permissions#read-and-edit)不同，後者使用 `//path` 用於絕對和 `/path` 用於專案相對。Sandbox 檔案系統路徑使用標準慣例：`/tmp/build` 是絕對路徑。

**設定範例：**

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "autoAllowBashIfSandboxed": true,
    "excludedCommands": ["docker *"],
    "filesystem": {
      "allowWrite": ["/tmp/build", "~/.kube"],
      "denyRead": ["~/.aws/credentials"]
    },
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org", "registry.yarnpkg.com"],
      "deniedDomains": ["uploads.github.com"],
      "allowUnixSockets": [
        "/var/run/docker.sock"
      ],
      "allowLocalBinding": true
    }
  }
}
```

**檔案系統和網路限制**可以透過兩種合併在一起的方式設定：

* **`sandbox.filesystem` 設定**（如上所示）：在 OS 層級 sandbox 邊界控制路徑。這些限制適用於所有子流程命令（例如 `kubectl`、`terraform`、`npm`），而不僅僅是 Claude 的檔案工具。
* **權限規則**：使用 `Edit` 允許/拒絕規則控制 Claude 的檔案工具存取，`Read` 拒絕規則阻止讀取，`WebFetch` 允許/拒絕規則控制網路網域。這些規則中的路徑也會合併到 sandbox 設定中。

<h3 id="attribution-settings">
  歸屬設定
</h3>

Claude Code 將歸屬新增到 git 提交和拉取請求。這些分別設定：

* 提交預設使用 [git trailers](https://git-scm.com/docs/git-interpret-trailers)（如 `Co-Authored-By`），可以自訂或停用
* 拉取請求說明是純文字

| 金鑰           | 說明                                                                                                             |
| :----------- | :------------------------------------------------------------------------------------------------------------- |
| `commit`     | git 提交的歸屬，包括任何 trailers。空字串隱藏提交歸屬                                                                              |
| `pr`         | 拉取請求說明的歸屬。空字串隱藏拉取請求歸屬                                                                                          |
| `sessionUrl` | 當從網頁或遠端控制工作階段執行時，是否將 claude.ai 工作階段連結附加為提交上的 `Claude-Session` trailer 和拉取請求說明中的連結。預設為 `true`。設定為 `false` 以省略連結 |

**預設提交歸屬：**

```text theme={null}
Co-Authored-By: Claude Sonnet 5 <noreply@anthropic.com>
```

模型名稱在 trailer 中反映工作階段的使用中模型。

**預設拉取請求歸屬：**

```text theme={null}
🤖 Generated with [Claude Code](https://claude.com/claude-code)
```

**範例：**

```json theme={null}
{
  "attribution": {
    "commit": "Generated with AI\n\nCo-Authored-By: AI <ai@example.com>",
    "pr": ""
  }
}
```

<Note>
  `attribution` 設定優先於已棄用的 `includeCoAuthoredBy` 設定。若要隱藏所有歸屬，請將 `commit` 和 `pr` 設定為空字串，並將 `sessionUrl` 設定為 `false`。
</Note>

<h3 id="file-suggestion-settings">
  檔案建議設定
</h3>

為 `@` 檔案路徑自動完成設定自訂命令。內建檔案建議使用快速檔案系統遍歷，但大型 monorepos 可能受益於專案特定的索引，例如預先建立的檔案索引或自訂工具。

```json theme={null}
{
  "fileSuggestion": {
    "type": "command",
    "command": "~/.claude/file-suggestion.sh"
  }
}
```

該命令使用與 [hooks](/docs/zh-TW/hooks) 相同的環境變數執行，包括 `CLAUDE_PROJECT_DIR`。它透過 stdin 接收包含 `query` 欄位的 JSON：

```json theme={null}
{"query": "src/comp"}
```

將換行符分隔的檔案路徑輸出到 stdout（目前限制為 15）：

```text theme={null}
src/components/Button.tsx
src/components/Modal.tsx
src/components/Form.tsx
```

**範例：**

```bash theme={null}
#!/bin/bash
query=$(cat | jq -r '.query')
# 用您自己的檔案搜尋命令替換 your-repo-file-index
your-repo-file-index --query "$query" | head -20
```

<h3 id="footer-link-badges">
  頁尾連結徽章
</h3>

`footerLinksRegexes` 設定在輸入框下方的頁尾中渲染額外的可點擊徽章。使用它將專案 CLI 列印的 ID（例如審查工具和問題追蹤器）轉換為工作階段連結。

每個項目的 `pattern` regex 與輪次輸出相符：工具結果，包括檔案內容和擷取的頁面，以及 Claude 自己的回應。`url` 和 `label` 中的 `{name}` 佔位符從模式中的命名擷取群組填入。

以下範例在問題金鑰（如 `PROJ-1234`）出現在輪次輸出中時渲染徽章。`(?<key>...)` 命名群組擷取金鑰，`{key}` 將其替換到 URL 和標籤中：

```json ~/.claude/settings.json theme={null}
{
  "footerLinksRegexes": [
    {
      "type": "regex",
      "pattern": "\\b(?<key>PROJ-\\d+)\\b",
      "url": "https://issues.example.com/browse/{key}",
      "label": "{key}"
    }
  ]
}
```

設定此項後，當 `PROJ-1234` 出現在工具結果或 Claude 的回覆中時，`PROJ-1234` 晶片會出現在頁尾中，連結到 `https://issues.example.com/browse/PROJ-1234`。

以下限制適用於每個項目：

| 限制     | 行為                                                                                                                                             |
| :----- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| URL 來源 | 擷取的值是 URL 編碼的，構造的 URL 必須共享範本的字面來源。擷取可以填入路徑段或查詢值，但無法改變連結指向的位置                                                                                   |
| URL 長度 | 超過 2048 個字元的構造 URL 會被丟棄                                                                                                                        |
| URL 方案 | 必須是 `https`、`http` 或公認的編輯器或工作區深層連結方案：`vscode`、`vscode-insiders`、`cursor`、`windsurf`、`zed`、`jetbrains`、`idea`、`slack`、`linear`、`notion`、`figma` |
| 標籤     | 預設為符合的文字，截斷為 28 個顯示欄                                                                                                                           |
| 徽章計數   | 最多 5 個徽章渲染。最舊的被較新的符合所取代，`/clear` 會移除它們                                                                                                         |
| 設定範圍   | 僅從使用者設定、`--settings` 旗標和 managed 設定讀取。在專案 `.claude/settings.json` 和本機 `.claude/settings.local.json` 中被忽略                                       |

輪次完成時，Claude Code 在主執行緒上將每個項目的 `pattern` regex 與輪次輸出相符，因此緩慢的 regex 會阻止 UI，直到完成。嵌套量詞（例如 `(a+)+$`）可能針對某些輸入花費指數級長時間並凍結工作階段，因此保持每個 `pattern` 線性並避免嵌套 `+` 或 `*`。

頁尾徽章與[自訂狀態行](/docs/zh-TW/statusline)並排渲染（當設定一個時）；兩者都不替換另一個。使用狀態行用於從工作階段資料計算自己內容的指令碼驅動列，使用頁尾徽章將對話中的 ID 轉換為連結，而無需指令碼。

<h3 id="hook-configuration">
  Hook 設定
</h3>

這些設定控制允許執行哪些 hooks 以及 HTTP hooks 可以存取的內容。`allowManagedHooksOnly` 設定只能在 [managed 設定](#settings-files)中設定。URL 和環境變數白名單可以在任何設定層級設定，並跨來源合併。

**當 `allowManagedHooksOnly` 為 `true` 時的行為：**

* 載入 Managed hooks 和 SDK hooks
* 從在 managed 設定 `enabledPlugins` 中強制啟用的 plugins 載入 Hooks。這讓管理員透過組織 marketplace 分發經過審查的 hooks，同時阻止其他所有內容。信任由完整 `plugin@marketplace` ID 授予，因此來自不同 marketplace 的同名 plugin 保持被阻止
* 使用者 hooks、專案 hooks 和所有其他 plugin hooks 被阻止

**限制 HTTP hook URL：**

限制 HTTP hooks 可以針對的 URL。支援 `*` 作為匹配的萬用字元。定義陣列時，針對不匹配 URL 的 HTTP hooks 會被無聲地阻止。主機名稱匹配不區分大小寫，並忽略尾部 FQDN 點，符合 DNS 語義。

```json theme={null}
{
  "allowedHttpHookUrls": ["https://hooks.example.com/*", "http://localhost:*"]
}
```

**限制 HTTP hook 環境變數：**

限制 HTTP hooks 可以插入到標頭值中的環境變數名稱。每個 hook 的有效 `allowedEnvVars` 是其自己清單與此設定的交集。

```json theme={null}
{
  "httpHookAllowedEnvVars": ["MY_TOKEN", "HOOK_SECRET"]
}
```

<h3 id="compute-managed-settings-with-a-policy-helper">
  使用政策協助程式計算 managed 設定
</h3>

`policyHelper` 設定指向在啟動時計算 managed 設定的可執行檔，因此管理員可以從裝置狀態、身份或遠端服務衍生政策，而不是靜態檔案。從 MDM 或系統 `managed-settings.json` 檔案設定它。Claude Code 在任何其他範圍中出現 `policyHelper` 時會忽略它，包括使用者設定、專案設定、HKCU 登錄 hive 和[伺服器管理的設定](/docs/zh-TW/server-managed-settings)。

該設定接受這些金鑰：

| 金鑰                  | 類型     | 說明                                          |
| ------------------- | ------ | ------------------------------------------- |
| `path`              | string | 協助程式可執行檔的絕對路徑                               |
| `timeoutMs`         | number | 在將執行視為失敗之前等待協助程式多長時間                        |
| `refreshIntervalMs` | number | 在背景中重新執行協助程式的頻率。設定為 `0` 以停用重新整理，或至少 `60000` |

協助程式將 JSON 信封寫入 stdout。將設定放在 `managedSettings` 金鑰下，而不是在頂層，因為裸設定物件會以 `managedSettings` 未定義的方式解析並應用任何內容：

```json theme={null}
{
  "managedSettings": {
    "permissions": { "deny": ["Read(//etc/secrets/**)"] }
  },
  "claudeMd": "# Organization context\n...",
  "appendSystemPrompt": "Always cite the internal style guide."
}
```

當協助程式發出 `managedSettings` 時，該物件會替換該執行的檔案型 managed 設定。當協助程式在啟動時以非零狀態結束時，Claude Code 會列印錯誤並拒絕啟動，因此需要中斷恢復能力的協助程式應從自己的快取提供並以 `0` 結束。

<h3 id="settings-precedence">
  設定優先順序
</h3>

設定按優先順序順序應用。從最高到最低：

1. **Managed 設定**（[伺服器管理](/docs/zh-TW/server-managed-settings)、[MDM/OS 層級政策](#configuration-scopes)或 [managed 設定](#settings-files)）
   * 由 IT 透過伺服器傳遞、MDM 設定檔案、登錄政策或 managed 設定檔案部署的政策
   * 無法被任何其他層級覆蓋，包括命令列引數
   * 在 managed 層級內，僅使用一個 managed 來源，其他來源被忽略而不是合併。優先順序，最高優先：
     * [`policyHelper`](#compute-managed-settings-with-a-policy-helper) 輸出：當設定時，這是唯一使用的 managed 來源
     * 遠端（claude.ai [伺服器管理](/docs/zh-TW/server-managed-settings)或 [Claude apps gateway](/docs/zh-TW/claude-apps-gateway)傳遞）
     * MDM/OS 層級政策
     * 檔案型（`managed-settings.d/*.json` 和 `managed-settings.json`，合併在一起）
     * HKCU 登錄（僅限 Windows）
   * 少數金鑰是例外，在任何管理員控制的 managed 來源設定它們時受尊重，而不是僅由獲勝的來源。使用者可寫的 HKCU 登錄來源被排除。例外金鑰為：
     * sandbox 鎖定金鑰 `sandbox.network.allowManagedDomainsOnly` 和 `sandbox.filesystem.allowManagedReadPathsOnly`，以及其相關聯的白名單
     * `allowAllClaudeAiMcps`
     * sandbox 二進位路徑 `sandbox.bwrapPath` 和 `sandbox.socatPath`
     * [`forceRemoteSettingsRefresh`](/docs/zh-TW/server-managed-settings)
   * 嵌入主機（例如 Claude Desktop）可以透過 SDK `managedSettings` 選項提供政策。預設情況下，當任何 admin 部署的 managed 來源存在時，此會被忽略：伺服器管理的設定、MDM 或 OS 層級政策，或 managed 設定檔案。使用者可寫的 HKCU 登錄回退不計為 admin 部署的來源。管理員可以透過設定 [`parentSettingsBehavior`](#available-settings) 為 `"merge"` 來選擇加入。嵌入器的值會被篩選，以便它們可以收緊 managed 政策但不能放鬆政策。

2. **命令列引數**
   * 特定工作階段的臨時覆蓋。JSON 透過 `--settings <file-or-json>` 傳遞會與檔案型設定合併，使用與其他層級相同的規則：此處設定的金鑰會覆蓋本機、專案或使用者設定中的相同金鑰，省略金鑰會保留較低層級的值

3. **本機專案設定**（`.claude/settings.local.json`）
   * 個人專案特定設定

4. **共享專案設定**（`.claude/settings.json`）
   * 原始碼控制中的團隊共享專案設定

5. **使用者設定**（`~/.claude/settings.json`）
   * 個人全域設定

此階層確保組織政策始終被強制執行，同時仍允許團隊和個人自訂其體驗。無論您從 CLI、[VS Code 擴充功能](/docs/zh-TW/vs-code)或 [JetBrains IDE](/docs/zh-TW/jetbrains) 執行 Claude Code，相同的優先順序都適用。

例如，如果您的使用者設定將 `permissions.defaultMode` 設定為 `acceptEdits`，而專案的共享設定將其設定為 `default`，則專案值適用。下面的範例涵蓋陣列值設定（如權限規則）如何組合的方式。

<Note>
  **陣列設定跨範圍合併。** 當相同的陣列值設定（例如 `sandbox.filesystem.allowWrite` 或 `permissions.allow`）出現在多個範圍中時，陣列會**連接和去重**，而不是替換。這意味著較低優先順序的範圍可以新增項目而不覆蓋由較高優先順序範圍設定的項目，反之亦然。例如，如果 managed 設定將 `allowWrite` 設定為 `["/opt/company-tools"]`，使用者新增 `["~/.kube"]`，則最終設定中包含兩個路徑。

  兩個陣列設定不以此方式合併：

  * [`fallbackModel`](#available-settings) 是一個有序鏈，其中位置具有意義：定義它的最高優先順序檔案提供整個值。
  * [`availableModels`](#available-settings)：當[最高優先順序 managed 來源](/docs/zh-TW/server-managed-settings#settings-precedence)定義它時，該清單按原樣應用，使用者、專案和本機項目無法擴展它。跨非 managed 範圍，陣列會照常合併。請參閱[合併行為](/docs/zh-TW/model-config#merge-behavior)。
</Note>

<h3 id="verify-active-settings">
  驗證使用中的設定
</h3>

在 Claude Code 內執行 `/status` 以查看哪些設定來源是使用中的。在功能表內，**Status** 標籤包含 `Setting sources` 行，列出 Claude Code 為目前工作階段載入的每一層，例如 `User settings` 或 `Project local settings`。當[managed 設定](/docs/zh-TW/admin-setup#decide-how-settings-reach-devices)生效時，項目會在括號中顯示傳遞頻道，例如 `Enterprise managed settings (remote)`、`(plist)`、`(HKLM)`、`(HKCU)` 或 `(file)`。`remote` 頻道涵蓋 claude.ai 伺服器管理的設定和 [Claude apps gateway](/docs/zh-TW/claude-apps-gateway)傳遞的政策。層級僅在該來源以至少一個金鑰載入時才出現在清單中，因此空清單表示未找到任何設定來源。

`Setting sources` 行確認正在讀取哪些來源。它不顯示哪一層提供了每個個別金鑰。同一對話框中的 **Config** 標籤是固定切換集（例如主題和詳細輸出）的編輯器，而不是您 `settings.json` 內容的檢視。

如果設定檔案包含錯誤（例如無效的 JSON 或驗證失敗的值），`/status` 會列出受影響的檔案。執行 `/doctor` 以查看每個錯誤的詳細資訊。

<h3 id="key-points-about-the-configuration-system">
  設定系統的關鍵要點
</h3>

* **記憶檔案（`CLAUDE.md`）**：包含 Claude 在啟動時載入的指示和內容
* **設定檔案（JSON）**：設定權限、環境變數和工具行為
* **Skills**：可以使用 `/skill-name` 叫用或由 Claude 自動載入的自訂提示
* **MCP servers**：使用其他工具和整合擴展 Claude Code
* **優先順序**：較高層級的設定（Managed）覆蓋較低層級的設定（User/Project）
* **繼承**：設定會合併跨範圍；較高優先順序範圍中的純量值覆蓋，陣列連接，有兩個例外如[陣列合併注意](#settings-precedence)中所述

<h3 id="system-prompt">
  系統提示
</h3>

Claude Code 的內部系統提示未發佈。若要新增自訂指示，請使用 `CLAUDE.md` 檔案或 `--append-system-prompt` 旗標。

<h3 id="exclude-sensitive-files">
  排除敏感檔案
</h3>

若要防止 Claude Code 存取包含敏感資訊（如 API 金鑰、機密和環境檔案）的檔案，請在您的 `.claude/settings.json` 檔案中使用 `permissions.deny` 設定：

```json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./.env)",
      "Read(./.env.*)",
      "Read(./secrets/**)",
      "Read(./config/credentials.json)",
      "Read(./build)"
    ]
  }
}
```

這取代了已棄用的 `ignorePatterns` 設定。符合這些模式的檔案會從檔案發現和搜尋結果中排除，並拒絕對這些檔案的讀取操作。

<h2 id="subagent-configuration">
  Subagent 設定
</h2>

Claude Code 支援可在使用者和專案層級設定的自訂 AI subagents。這些 subagents 儲存為具有 YAML frontmatter 的 Markdown 檔案：

* **使用者 subagents**：`~/.claude/agents/`，在所有專案中可用
* **專案 subagents**：`.claude/agents/`，特定於您的專案，可與您的團隊共享

Subagent 檔案定義具有自訂提示和工具權限的專門 AI 助手。在 [subagents 文件](/docs/zh-TW/sub-agents)中深入了解建立和使用 subagents。

<h2 id="plugin-configuration">
  Plugin 配置
</h2>

Claude Code 支援 plugin 系統，可讓您使用 skills、agents、hooks 和 MCP servers 擴展功能。Plugins 透過 marketplaces 分發，可以在使用者和儲存庫層級設定。

<h3 id="plugin-settings">
  Plugin 設定
</h3>

`settings.json` 中的 plugin 相關設定：

```json theme={null}
{
  "enabledPlugins": {
    "formatter@acme-tools": true,
    "deployer@acme-tools": true,
    "analyzer@security-plugins": false
  },
  "extraKnownMarketplaces": {
    "acme-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/claude-plugins"
      }
    }
  }
}
```

<h4 id="enabledplugins">
  `enabledPlugins`
</h4>

控制啟用哪些 plugins。格式：`"plugin-name@marketplace-name": true/false`。沒有在任何範圍中有項目的 plugin 會回退到其 [`defaultEnabled`](/docs/zh-TW/plugins-reference#default-enablement) 值。

**範圍**：

* **使用者設定**（`~/.claude/settings.json`）：個人 plugin 偏好設定
* **專案設定**（`.claude/settings.json`）：與團隊共享的專案特定 plugins
* **本機設定**（`.claude/settings.local.json`）：每台機器的覆蓋，Claude Code 建立時會被 gitignored
* **Managed 設定**（`managed-settings.json`）：組織範圍的政策覆蓋，在所有範圍阻止安裝並從 marketplace 隱藏 plugin

<Note>
  專案設定優先於使用者設定，因此在 `~/.claude/settings.json` 中將 plugin 設定為 `false` 不會停用專案的 `.claude/settings.json` 啟用的 plugin。若要在您的機器上選擇退出專案啟用的 plugin，請改在 `.claude/settings.local.json` 中將其設定為 `false`。

  由 managed 設定強制啟用的 plugins 無法以此方式停用，因為 managed 設定會覆蓋本機設定。

  自 Claude Code v2.1.195 起，在專案的 `.claude/settings.json` 中啟用來自外部來源（例如 GitHub 儲存庫或 npm 套件）的 plugin 不會為其他人安裝它。每個載入 plugins 的路徑都會要求每個使用者在執行前[安裝並信任 plugin](/docs/zh-TW/discover-plugins#configure-team-marketplaces)。
</Note>

**範例**：

```json theme={null}
{
  "enabledPlugins": {
    "code-formatter@team-tools": true,
    "deployment-tools@team-tools": true,
    "experimental-features@personal": false
  }
}
```

<h4 id="pluginconfigs">
  `pluginConfigs`
</h4>

儲存 plugin 的 [`userConfig`](/docs/zh-TW/plugins-reference#user-configuration) 提示收集的非敏感選項值，按 plugin ID 鍵入。Claude Code 在您填入 plugin 的設定對話框時會將此鍵寫入使用者設定，因此您無需手動編輯它。敏感選項改為儲存在 macOS Keychain 中，或在沒有支援 keychain 的平台上儲存在 `~/.claude/.credentials.json` 中。

此範例儲存從 `acme-tools` marketplace 安裝的 plugin 的一個選項：

```json theme={null}
{
  "pluginConfigs": {
    "deployer@acme-tools": {
      "options": {
        "api_endpoint": "https://api.example.com"
      }
    }
  }
}
```

`pluginConfigs` 僅從使用者設定、`--settings` 旗標和 managed 設定讀取。專案的 `.claude/settings.json` 或 `.claude/settings.local.json` 中的項目會被忽略，因為這些值會被替換到 plugin hook、MCP 和 LSP 配置中，而複製的儲存庫不得能夠提供它們。在 v2.1.207 之前，專案和本機設定也會被讀取。

<h4 id="extraknownmarketplaces">
  `extraKnownMarketplaces`
</h4>

定義應為儲存庫提供的其他 marketplaces。通常在儲存庫層級設定中使用，以確保團隊成員有權存取所需的 plugin 來源。

**當儲存庫包含 `extraKnownMarketplaces` 時**：

1. 當團隊成員信任資料夾時，系統會提示他們安裝 marketplace
2. 然後提示團隊成員從該 marketplace 安裝 plugins
3. 使用者可以跳過不需要的 marketplaces 或 plugins（儲存在使用者設定中）
4. 安裝尊重信任邊界並需要明確同意

**範例**：

```json theme={null}
{
  "extraKnownMarketplaces": {
    "acme-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/claude-plugins"
      }
    },
    "security-plugins": {
      "source": {
        "source": "git",
        "url": "https://git.example.com/security/plugins.git"
      }
    }
  }
}
```

**Marketplace 來源類型**：

* `github`：GitHub 儲存庫（使用 `repo`）
* `git`：任何 git URL（使用 `url`）
* `directory`：本機檔案系統路徑（使用 `path`，僅用於開發）
* `hostPattern`：正規表達式模式以符合 marketplace 主機（使用 `hostPattern`）
* `settings`：直接在 settings.json 中宣告的內嵌 marketplace，無需單獨的託管儲存庫（使用 `name` 和 `plugins`）

`git` 來源類型適用於任何 git 託管服務，包括自託管 GitLab 和 Bitbucket。Claude Code 使用與該機器上 `git clone` 相同的驗證來複製儲存庫：已設定的認證助手或 SSH 金鑰。提供者 token（例如 `GITHUB_TOKEN`）只有透過讀取它的認證助手才會生效。請參閱[私有儲存庫](/docs/zh-TW/plugin-marketplaces#private-repositories)以了解設定詳細資訊。

對於 `github` 和 `git` 來源，在 `source` 物件內設定 `"skipLfs": true`（與 `repo` 或 `url` 並列）以在 Claude Code 複製或更新 marketplace 儲存庫時跳過 Git LFS 下載。LFS 指標檔案保持為指標而不是下載其內容。當儲存庫包含與 plugin 內容無關的大型 LFS 物件時，請使用此選項。需要 Claude Code v2.1.153 或更新版本。

每個 marketplace 項目也接受選用的 `autoUpdate` 布林值。在 `source` 旁邊設定 `"autoUpdate": true`，使 Claude Code 在啟動時重新整理該 marketplace 並更新其已安裝的 plugins。省略時，官方 Anthropic marketplaces 預設為 `true`，所有其他 marketplaces 預設為 `false`。請參閱[設定自動更新](/docs/zh-TW/discover-plugins#configure-auto-updates)。

使用 `source: 'settings'` 宣告一小組 plugins，無需設定託管 marketplace 儲存庫。此處列出的 Plugins 必須參考外部來源，例如 GitHub 或 npm。您仍需要在 `enabledPlugins` 中分別啟用每個 plugin。

```json theme={null}
{
  "extraKnownMarketplaces": {
    "team-tools": {
      "source": {
        "source": "settings",
        "name": "team-tools",
        "plugins": [
          {
            "name": "code-formatter",
            "source": {
              "source": "github",
              "repo": "acme-corp/code-formatter"
            }
          }
        ]
      }
    }
  }
}
```

<h4 id="strictknownmarketplaces">
  `strictKnownMarketplaces`
</h4>

**Managed 設定僅限**：控制使用者可以新增和安裝 plugins 的 plugin marketplaces。此設定只能在 [managed 設定](/docs/zh-TW/settings#settings-files)中設定，並為管理員提供對 marketplace 來源的嚴格控制。

**Managed 設定檔案位置**：

* **macOS**：`/Library/Application Support/ClaudeCode/managed-settings.json`
* **Linux 和 WSL**：`/etc/claude-code/managed-settings.json`
* **Windows**：`C:\Program Files\ClaudeCode\managed-settings.json`

**關鍵特性**：

* 僅在 managed 設定（`managed-settings.json`）中可用
* 無法被使用者或專案設定覆蓋（最高優先順序）
* 在網路/檔案系統操作之前強制執行（被阻止的來源永遠不會執行）
* 對來源規格使用精確匹配（包括 git 來源的 `ref`、`path`），除了 `hostPattern` 和 `pathPattern`，它們使用正規表達式匹配

**白名單行為**：

* `undefined`（預設）：無限制 - 使用者可以新增任何 marketplace
* 空陣列 `[]`：完全鎖定 - 使用者無法新增任何新 marketplaces
* 來源清單：使用者只能新增完全符合的 marketplaces

**所有支援的來源類型**：

白名單支援多種 marketplace 來源類型。大多數來源使用精確匹配，而 `hostPattern` 和 `pathPattern` 分別使用正規表達式匹配 marketplace 主機和檔案系統路徑。

1. **GitHub 儲存庫**：

```json theme={null}
{ "source": "github", "repo": "acme-corp/approved-plugins" }
{ "source": "github", "repo": "acme-corp/security-tools", "ref": "v2.0" }
{ "source": "github", "repo": "acme-corp/plugins", "ref": "main", "path": "marketplace" }
```

欄位：`repo`（必需）、`ref`（選用：分支或標籤）、`path`（選用：子目錄）

2. **Git 儲存庫**：

```json theme={null}
{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git" }
{ "source": "git", "url": "https://bitbucket.org/acme-corp/plugins.git", "ref": "production" }
{ "source": "git", "url": "ssh://git@git.example.com/plugins.git", "ref": "v3.1", "path": "approved" }
```

欄位：`url`（必需）、`ref`（選用：分支或標籤）、`path`（選用：子目錄）

3. **基於 URL 的 marketplaces**：

```json theme={null}
{ "source": "url", "url": "https://plugins.example.com/marketplace.json" }
{ "source": "url", "url": "https://cdn.example.com/marketplace.json", "headers": { "Authorization": "Bearer ${TOKEN}" } }
```

欄位：`url`（必需）、`headers`（選用：用於驗證存取的 HTTP 標頭）

<Note>
  基於 URL 的 marketplaces 僅下載 `marketplace.json` 檔案。它們不從伺服器下載 plugin 檔案。基於 URL 的 marketplaces 中的 Plugins 必須使用外部來源（GitHub、npm 或 git URL），而不是相對路徑。對於具有相對路徑的 plugins，請改用基於 Git 的 marketplace。請參閱[疑難排解](/docs/zh-TW/plugin-marketplaces#plugins-with-relative-paths-fail-in-url-based-marketplaces)以了解詳細資訊。
</Note>

4. **NPM 套件**：

```json theme={null}
{ "source": "npm", "package": "@acme-corp/claude-plugins" }
{ "source": "npm", "package": "@acme-corp/approved-marketplace" }
```

欄位：`package`（必需，支援範圍套件）

5. **檔案路徑**：

```json theme={null}
{ "source": "file", "path": "/usr/local/share/claude/acme-marketplace.json" }
{ "source": "file", "path": "/opt/acme-corp/plugins/marketplace.json" }
```

欄位：`path`（必需：marketplace.json 檔案的絕對路徑）

6. **目錄路徑**：

```json theme={null}
{ "source": "directory", "path": "/usr/local/share/claude/acme-plugins" }
{ "source": "directory", "path": "/opt/acme-corp/approved-marketplaces" }
```

欄位：`path`（必需：包含 `.claude-plugin/marketplace.json` 的目錄的絕對路徑）

7. **主機模式匹配**：

```json theme={null}
{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }
{ "source": "hostPattern", "hostPattern": "^gitlab\\.internal\\.example\\.com$" }
```

欄位：`hostPattern`（必需：用於符合 marketplace 主機的正規表達式模式）

當您想允許來自特定主機的所有 marketplaces 而不列舉每個儲存庫時，請使用主機模式匹配。這對於具有內部 GitHub Enterprise 或 GitLab 伺服器的組織很有用，開發人員可以在其中建立自己的 marketplaces。

按來源類型的主機提取：

* `github`：始終符合 `github.com`
* `git`：從 URL 提取主機名稱（支援 HTTPS 和 SSH 格式）
* `url`：從 URL 提取主機名稱
* `npm`、`file`、`directory`：不支援主機模式匹配

8. **路徑模式匹配**：

```json theme={null}
{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }
{ "source": "pathPattern", "pathPattern": ".*" }
```

欄位：`pathPattern`（必需：與 `file` 和 `directory` 來源的 `path` 欄位相符的正規表達式模式）

使用路徑模式匹配來允許檔案系統型 marketplaces 與網路來源的 `hostPattern` 限制並行。設定 `".*"` 以允許所有本機路徑，或設定更窄的模式以限制到特定目錄。

**設定範例**：

範例：僅允許特定 marketplaces：

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "github",
      "repo": "acme-corp/approved-plugins"
    },
    {
      "source": "github",
      "repo": "acme-corp/security-tools",
      "ref": "v2.0"
    },
    {
      "source": "url",
      "url": "https://plugins.example.com/marketplace.json"
    },
    {
      "source": "npm",
      "package": "@acme-corp/compliance-plugins"
    }
  ]
}
```

範例：停用所有 marketplace 新增：

```json theme={null}
{
  "strictKnownMarketplaces": []
}
```

範例：允許來自內部 git 伺服器的所有 marketplaces：

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "hostPattern",
      "hostPattern": "^github\\.example\\.com$"
    }
  ]
}
```

**精確匹配要求**：

Marketplace 來源必須完全符合才能允許使用者的新增。對於基於 git 的來源（`github` 和 `git`），這包括所有選用欄位：

* `repo` 或 `url` 必須完全符合
* `ref` 欄位必須完全符合（或兩者都未定義）
* `path` 欄位必須完全符合（或兩者都未定義）

**不符合**的來源範例：

```json theme={null}
// 這些是不同的來源：
{ "source": "github", "repo": "acme-corp/plugins" }
{ "source": "github", "repo": "acme-corp/plugins", "ref": "main" }

// 這些也不同：
{ "source": "github", "repo": "acme-corp/plugins", "path": "marketplace" }
{ "source": "github", "repo": "acme-corp/plugins" }
```

**與 `extraKnownMarketplaces` 的比較**：

| 方面         | `strictKnownMarketplaces` | `extraKnownMarketplaces` |
| ---------- | ------------------------- | ------------------------ |
| **目的**     | 組織政策強制執行                  | 團隊便利                     |
| **設定檔案**   | 僅 `managed-settings.json` | 任何設定檔案                   |
| **行為**     | 阻止非白名單新增                  | 自動安裝遺失的 marketplaces     |
| **何時強制執行** | 在網路/檔案系統操作之前              | 在使用者信任提示之後               |
| **可以被覆蓋**  | 否（最高優先順序）                 | 是（由較高優先順序設定）             |
| **來源格式**   | 直接來源物件                    | 具有巢狀來源的命名 marketplace    |
| **使用案例**   | 合規、安全限制                   | 上線、標準化                   |

**格式差異**：

`strictKnownMarketplaces` 使用直接來源物件：

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/plugins" }
  ]
}
```

`extraKnownMarketplaces` 需要命名 marketplaces：

```json theme={null}
{
  "extraKnownMarketplaces": {
    "acme-tools": {
      "source": { "source": "github", "repo": "acme-corp/plugins" }
    }
  }
}
```

**同時使用兩者**：

`strictKnownMarketplaces` 是政策閘門：它控制使用者可能新增的內容，但不註冊任何 marketplaces。若要同時限制和為所有使用者預先註冊 marketplace，請在 `managed-settings.json` 中設定兩者：

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/plugins" }
  ],
  "extraKnownMarketplaces": {
    "acme-tools": {
      "source": { "source": "github", "repo": "acme-corp/plugins" }
    }
  }
}
```

僅設定 `strictKnownMarketplaces` 時，使用者仍可透過 `/plugin marketplace add` 手動新增允許的 marketplace，但它不會自動提供。

**重要注意事項**：

* 限制在任何網路請求或檔案系統操作之前檢查
* 被阻止時，使用者會看到清晰的錯誤訊息，指示來源被 managed 政策阻止
* 限制在 marketplace 新增和 plugin 安裝、更新、重新整理和自動更新時強制執行。在設定政策之前新增的 marketplace 一旦其來源不再符合白名單，就無法用於安裝或更新 plugins
* Managed 設定具有最高優先順序，無法被覆蓋

請參閱 [Managed marketplace 限制](/docs/zh-TW/plugin-marketplaces#managed-marketplace-restrictions)以了解面向使用者的文件。

<h4 id="strictpluginonlycustomization">
  `strictPluginOnlyCustomization`
</h4>

**Managed 設定僅限**：阻止 skills、agents、hooks 和 MCP servers 來自使用者和專案來源，因此它們只能來自 plugins 或 managed 設定。將其與 `strictKnownMarketplaces` 結合以控制完整的自訂供應鏈：marketplace 白名單控制使用者可以安裝哪些 plugins，此設定阻止所有不來自 plugin 或 managed 設定的內容。

該值要麼是 `true` 以鎖定所有四個表面，要麼是命名要鎖定的表面的陣列：

```json theme={null}
{
  "strictPluginOnlyCustomization": ["skills", "hooks"]
}
```

對於每個鎖定的表面，Claude Code 會跳過使用者層級和專案層級的來源，並僅載入 plugin 提供的和 managed 來源：

| 表面       | 鎖定時被阻止                                    | 仍然載入                                                                |
| :------- | :---------------------------------------- | :------------------------------------------------------------------ |
| `skills` | `~/.claude/skills/`、`.claude/skills/`     | Plugin skills、bundled skills、managed 政策目錄中的 skills                  |
| `agents` | `~/.claude/agents/`、`.claude/agents/`     | Plugin agents、內建 agents、managed 政策目錄中的 agents                       |
| `hooks`  | 使用者、專案和本機 `settings.json` 中的 Hooks        | Plugin hooks、managed 設定中的 hooks                                     |
| `mcp`    | `~/.claude.json` 和 `.mcp.json` 中的 Servers | Plugin MCP servers、[`managed-mcp.json`](/docs/zh-TW/managed-mcp) servers |

Claude Code 版本不識別的表面名稱會被忽略而不是導致設定檔案失敗，因此您可以在所有用戶端更新之前新增新的表面名稱。

<h3 id="manage-plugins">
  管理 plugins
</h3>

使用 `/plugin` 命令以互動方式管理 plugins：

* 瀏覽 marketplaces 中的可用 plugins
* 安裝/解除安裝 plugins
* 啟用/停用 plugins
* 檢視 plugin 詳細資訊（提供的 skills、agents、hooks）
* 新增/移除 marketplaces

在 [plugins 文件](/docs/zh-TW/plugins)中深入了解 plugin 系統。

<h2 id="environment-variables">
  環境變數
</h2>

環境變數可讓您控制 Claude Code 行為，而無需編輯設定檔案。任何變數也可以在 [`settings.json`](#available-settings) 中的 `env` 金鑰下設定，以將其應用於每個工作階段或推出到您的團隊。

請參閱[環境變數參考](/docs/zh-TW/env-vars)以了解完整清單。

<h2 id="tools-available-to-claude">
  Claude 可用的工具
</h2>

Claude Code 可以存取一組工具，用於讀取、編輯、搜尋、執行命令和協調 subagents。工具名稱是您在權限規則和 hook 匹配器中使用的確切字串。

請參閱[工具參考](/docs/zh-TW/tools-reference)以了解完整清單和 Bash 工具行為詳細資訊。

<h2 id="see-also">
  另請參閱
</h2>

* [Permissions](/docs/zh-TW/permissions)：權限系統、規則語法、工具特定模式和 managed 政策
* [Authentication](/docs/zh-TW/authentication)：設定使用者對 Claude Code 的存取
* [Debug your configuration](/docs/zh-TW/debug-your-config)：診斷為什麼設定、hook 或 MCP server 未生效
* [Troubleshoot installation and login](/docs/zh-TW/troubleshoot-install)：安裝、authentication 和平台問題
