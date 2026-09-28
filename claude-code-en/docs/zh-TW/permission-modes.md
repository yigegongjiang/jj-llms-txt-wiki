> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 選擇權限模式

> 控制 Claude 在採取動作前是否詢問。在 CLI 中使用 Shift+Tab、在 VS Code 中使用模式指示器，或在 Desktop 中使用模式選擇器來切換權限模式。

權限模式設定 Claude 在工作階段中可以執行哪些操作而無需先詢問您。在 Manual 模式中，Claude Code 會在大多數編輯檔案、執行 shell 命令或存取網路的操作前停止並詢問您。在[自動模式](#eliminate-prompts-with-auto-mode)中，第二個模型（分類器）會審查操作而不是您；[分類器如何評估操作](#how-the-classifier-evaluates-actions)列出它審查的操作以及跳過的操作。

在 Pro、Max 和 Team 方案上，內建的起始權限模式是自動模式。[工作階段在哪個模式中啟動](#which-mode-a-session-starts-in)涵蓋改變起始權限模式的表面和設定。您也可以隨時改變執行中工作階段的權限模式。

<h2 id="available-modes">
  可用的模式
</h2>

每種模式在便利性和監督之間做出不同的權衡。下表顯示在每種模式中 Claude 無需權限提示即可執行的操作。Manual 模式出現在其設定值 `default` 下。

| 模式                                                                  | 無需詢問即可執行                                                   | 最適合           |
| :------------------------------------------------------------------ | :--------------------------------------------------------- | :------------ |
| `default`                                                           | 僅讀取                                                        | 自己審查每個操作、敏感工作 |
| [`acceptEdits`](#auto-approve-file-edits-with-acceptedits-mode)     | 讀取、檔案編輯和常見的檔案系統命令（`mkdir`、`touch`、`mv`、`cp` 等）             | 迭代您正在審查的程式碼   |
| [`plan`](#analyze-before-you-edit-with-plan-mode)                   | 讀取，加上當[自動模式](#eliminate-prompts-with-auto-mode)可用時分類器批准的命令 | 在變更程式碼前探索程式碼庫 |
| [`auto`](#eliminate-prompts-with-auto-mode)                         | 所有操作，具有背景安全檢查                                              | 長期任務、減少提示疲勞   |
| [`dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode)       | 讀取和預先批准的工具；任何會提示的操作都被拒絕                                    | 鎖定的 CI 和指令碼   |
| [`bypassPermissions`](#skip-all-checks-with-bypasspermissions-mode) | 所有操作                                                       | 僅限隔離的容器和虛擬機器  |

審查每個操作的模式在 CLI 中、`claude --help` 中、VS Code 和 JetBrains 擴充功能中以及桌面應用程式中名為 **Manual**。其設定值為 `default`，這是 hooks 和 SDK 整合使用的值。CLI 接受 `manual` 作為別名，無論您在何處輸入該值，例如 `claude --permission-mode manual` 或 `"defaultMode": "manual"`。Manual 標籤和 `manual` 別名需要 Claude Code v2.1.200 或更新版本。桌面應用程式的標籤不取決於您的 CLI 版本。

寫入[受保護的路徑](#protected-paths)永遠不會自動批准，唯一的例外是 `bypassPermissions` 模式，以及可使用略過權限的 Plan Mode 工作階段，也就是以[將 `bypassPermissions` 放入模式循環](#switch-permission-modes)的方式啟動的工作階段。

模式設定基準。在頂部分層[權限規則](/docs/zh-TW/permissions#manage-permissions)以預先批准或阻止特定工具。拒絕規則在每種模式中都會阻止，包括 `bypassPermissions`。拒絕和詢問規則不適用於 [`EndConversation`](/docs/zh-TW/tools-reference#endconversation-tool-behavior)，只要 Claude 仍有至少一個其他工具可以呼叫。允許規則在 `bypassPermissions` 中無效。

<h3 id="actions-no-mode-auto-approves">
  任何模式都不會自動批准的操作
</h3>

Claude Code 在任何模式中都不會自動批准以下操作，包括 `bypassPermissions`。每個項目都連結到說明在每種模式中會發生什麼的部分：

* 符合明確[詢問規則](/docs/zh-TW/permissions#manage-permissions)的工具
* 您的組織[設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools) 的連接器工具，在該設定到達 Claude Code 的工作階段中
* 需要使用者互動的工具：內建的 `AskUserQuestion` 工具和標記為 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具
* `rm` 和 `rmdir` 移除針對[關鍵路徑](#critical-paths)，沒有允許規則或 `PreToolUse` hook `"allow"` 批准
* [跨工作階段訊息保護措施](#skip-all-checks-with-bypasspermissions-mode)
* 當 [`permissions.blockReadsOutsideWorkingDirectories`](/docs/zh-TW/settings-reference#permissions-blockreadsoutsideworkingdirectories) 開啟時，在工作目錄外讀取：已識別的檔案讀取 Bash 命令即使在自動模式和 `bypassPermissions` 模式中也會提示，任何[未沙箱化的重試](/docs/zh-TW/sandboxing#the-unsandboxed-retry-escape-hatch)需要批准才能在沙箱外執行也是如此。需要 Claude Code v2.1.257 或更新版本。

  殼層解析器無法追蹤的命令，例如變更目錄超過一次或執行子殼層的命令，即使在命名沒有外部路徑時也會以相同方式提示。當命令在[沙箱](/docs/zh-TW/sandboxing)中執行且沙箱強制執行該區塊時，此提示不適用。

<h2 id="common-setups">
  常見設定
</h2>

權限模式決定 Claude 是否在操作前詢問，而 [Bash 沙箱](/docs/zh-TW/sandboxing)和外部[隔離邊界](/docs/zh-TW/sandbox-environments)決定操作執行後可以到達什麼。下表中的每一行將目標與讓您到達該目標的旗標或設定以及所需的隔離配對，作為起點。[可用的模式](#available-modes)列出在每種模式中無需提示即可執行的操作。

| 您想要              | 開始使用                                                                                                                      | 所需的隔離                                                                                                                                          | 注意                                                                                                                                     |
| :--------------- | :------------------------------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------- |
| 自己審查每個操作         | Manual 模式：`claude --permission-mode default`                                                                              | 無                                                                                                                                              | 敏感工作、不熟悉的程式碼                                                                                                                           |
| 在本地迭代，提示更少，無需分類器 | Manual 模式加上 Bash 沙箱在[自動允許模式](/docs/zh-TW/sandboxing#sandbox-modes)中：`claude --permission-mode default`，然後執行 `/sandbox` 並選擇自動允許 | 內建 Bash 沙箱，在 macOS、Linux 和 WSL2 上                                                                                                              | 拒絕規則仍然適用，詢問規則命名命令（例如 `Bash(git push *)`）仍然提示。若要改為從設定檔開啟沙箱，請將 [`sandbox.enabled`](/docs/zh-TW/settings-reference#sandbox-enabled) 設定為 `true` |
| 在變更任何內容前探索       | `claude --permission-mode plan`                                                                                           | 無                                                                                                                                              | Claude Code 會阻止編輯，直到您[批准計畫](#review-and-approve-a-plan)                                                                                |
| 在自動模式中無人值守工作     | `claude --permission-mode auto`、Pro、Max 和 Team 上的[內建起始權限模式](#which-mode-a-session-starts-in)                              | 無；沙箱或容器增加深度防禦                                                                                                                                  | 需要[支援的模型](#eliminate-prompts-with-auto-mode)，您的組織可以[關閉自動模式](#eliminate-prompts-with-auto-mode)                                         |
| 在 CI 中使用精確允許清單執行 | `claude -p "run the test suite" --permission-mode dontAsk --allowedTools "Bash(npm test)" "Read"`                         | 無，超出您的 CI 執行器提供的                                                                                                                               | [Claude Code on the web](/docs/zh-TW/claude-code-on-the-web) 忽略設定檔中的 `dontAsk`                                                              |
| 在容器內完全無人值守執行     | `claude -p "<prompt>" --dangerously-skip-permissions`                                                                     | 必需：容器、虛擬機或[沙箱執行時](/docs/zh-TW/sandbox-environments#sandbox-runtime)；在 Linux 和 macOS 上，以[非 root 使用者](#skip-all-checks-with-bypasspermissions-mode)執行 | Claude Code on the web 忽略設定檔中的此模式。在此 `-p` 執行中，[仍會提示的少數呼叫](#skip-all-checks-with-bypasspermissions-mode)會被拒絕                            |

Bash 沙箱和自動模式獨立工作並結合，除了[沙箱模式](/docs/zh-TW/sandboxing#sandbox-modes)下列出的例外。如需完整互動，請參閱[沙箱化如何與權限和權限模式相關](/docs/zh-TW/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes)和[隔離如何與權限模式相關](/docs/zh-TW/sandbox-environments#how-isolation-relates-to-permission-modes)。

<h2 id="which-mode-a-session-starts-in">
  工作階段在哪個模式中啟動
</h2>

當您在終端中啟動新工作階段時，Claude Code 會從適用的第一個中取得權限模式：

1. `--permission-mode` 旗標或 `--dangerously-skip-permissions`

2. [設定檔](/docs/zh-TW/settings#where-settings-live)中的 `permissions.defaultMode`

   如果您在 `.claude/settings.json` 或 `.claude/settings.local.json` 中設定 `"auto"`，該值不會生效，Claude Code 會改為使用內建預設值而不是來自 `~/.claude/settings.json` 的 `defaultMode`。如果您在這兩個檔案中設定 `"bypassPermissions"`，它也不會生效，工作階段會以 Manual 模式啟動。其他值適用於任何設定檔。

3. 內建預設值

VS Code 擴充功能啟動的對話遵循[切換權限模式](#switch-permission-modes)中的擴充功能自身清單。如需 Claude Code 在恢復工作階段時啟動的權限模式，請參閱[恢復時的權限模式](/docs/zh-TW/sessions#permission-mode-on-resume)。

內建 `auto` 預設在 macOS、Linux 和 WSL 上需要 Claude Code v2.1.228 或更新版本，在原生 Windows 上需要 v2.1.233 或更新版本。在較早的版本上，內建預設是 Manual。

內建預設取決於您如何執行 Claude Code、您的方案以及 Claude Code 是否可以擷取其功能旗標。符合的第一行適用。該表涵蓋您在終端或透過 VS Code 擴充功能啟動的工作階段；對於桌面應用程式和 claude.ai，請參閱[切換權限模式](#switch-permission-modes)中的 Desktop 和 Web 標籤。

| 您如何執行 Claude Code                                                                                                                                                            | 內建起始權限模式  |
| :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------- |
| 任何設定檔將 `disableAutoMode` 設定為 `"disable"`                                                                                                                                     | `default` |
| [功能旗標擷取](/docs/zh-TW/env-vars#features-that-need-feature-flag-fetching)已關閉                                                                                                        | `default` |
| 您的[安裝 Claude Code 或升級後的第一個工作階段](/docs/zh-TW/env-vars#first-session-after-an-install-or-upgrade)到新增此預設的版本，除非在全新安裝後，Claude Code 及時擷取旗標                                              | `default` |
| `claude -p` 或 [Agent SDK](/docs/zh-TW/agent-sdk/permissions)                                                                                                                      | `default` |
| Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry、[Claude Platform on AWS](/docs/zh-TW/claude-platform-on-aws) 或已登入的 [Claude 應用程式閘道](/docs/zh-TW/claude-apps-gateway)工作階段 | `default` |
| Pro、Max 或 Team 方案，在終端或透過 [VS Code 擴充功能](/docs/zh-TW/vs-code)                                                                                                                      | `auto`    |
| Enterprise 方案或 Claude Console API 金鑰                                                                                                                                         | `default` |

當功能旗標擷取已關閉或在[安裝或升級後的第一個工作階段](/docs/zh-TW/env-vars#first-session-after-an-install-or-upgrade)中旗標尚未到達時，VS Code 擴充功能在選擇起始權限模式時會忽略每個設定檔。

當旗標、設定檔或內建預設選擇 `auto` 但自動模式對工作階段不可用時，Claude Code 會改為以 Manual 啟動工作階段。當工作階段不符合[可用性要求](#eliminate-prompts-with-auto-mode)時，自動模式不可用，例如設定檔關閉它或不支援它的模型，或當 Anthropic 已在伺服器端暫時關閉它時。

內建預設第一次在自動模式中啟動您的工作階段時，Claude Code 會顯示連結到此頁面的通知：

* 在終端中，一次，在工作階段頂部
* 在 VS Code 擴充功能中，作為新對話螢幕上的卡片，直到您關閉它

在 Pro、Max 和 Team 方案上，如果您的 `~/.claude/settings.json` 將 `defaultMode` 設定為 `auto` 以外的值，且沒有其他設定檔設定它，您的工作階段會繼續以該模式啟動。Claude Code 會在終端或 VS Code 擴充功能中詢問一次，是否將設定變更為自動模式。如果您拒絕，您的設定會保持原樣。

<h3 id="start-in-a-different-mode">
  以不同的權限模式啟動
</h3>

您可以為一個工作階段、或作為機器、專案或組織中每個工作階段的預設值設定起始權限模式。當多個設定檔設定 `permissions.defaultMode` 時，[設定優先順序](/docs/zh-TW/settings#settings-precedence)決定，因此專案或受管值優先於 `~/.claude/settings.json`。若要變更已執行工作階段的權限模式，請參閱[切換權限模式](#switch-permission-modes)。

| 若要為以下設定起始權限模式      | 執行此操作                                                                                                                                                                                                                     |
| :----------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 您即將啟動的一個工作階段       | 將權限模式作為旗標傳遞，例如 `claude --permission-mode default`                                                                                                                                                                         |
| 您在此機器上啟動的每個終端工作階段  | 在 `~/.claude/settings.json` 中設定 `permissions.defaultMode`。如需 VS Code 擴充功能讀取的內容，請參閱[切換權限模式](#switch-permission-modes)                                                                                                      |
| 您在一個專案中啟動的每個終端工作階段 | 在專案的 `.claude/settings.json` 中設定 `permissions.defaultMode`。您在終端中啟動的工作階段遵守除 `auto` 和 `bypassPermissions` 外的每個值；VS Code 擴充功能啟動的工作階段不讀取專案設定以取得起始權限模式                                                                         |
| 您組織中的每個終端工作階段      | 在[受管設定](/docs/zh-TW/managed-settings)中設定 `permissions.defaultMode`。終端工作階段以該模式啟動，人們仍然可以切換到自動模式；如需 VS Code 擴充功能讀取的內容，請參閱[切換權限模式](#switch-permission-modes)。若要移除自動模式以便沒有人可以選擇它，請改為將 `permissions.disableAutoMode` 設定為 `"disable"` |

此範例使您機器上的每個終端工作階段以 Manual 模式啟動，其設定值為 `default`。將其儲存在 `~/.claude/settings.json` 中：

```json theme={null}
{
  "permissions": {
    "defaultMode": "default"
  }
}
```

您啟動的下一個工作階段會在狀態列中顯示 `⏸ manual mode on`。

<h2 id="switch-permission-modes">
  切換權限模式
</h2>

每個介面都有自己的控制項用於在工作階段期間切換權限模式，以及自己的方式來選擇新工作階段啟動的權限模式。選擇您的介面以查看其控制項。

<Tabs>
  <Tab title="CLI">
    **在工作階段期間**：按 `Shift+Tab` 循環切換權限模式。從 `auto`，第一次按下切換到 `default`，循環然後執行 `default` → `acceptEdits` → `plan` → 回到 `default`。可選模式（如下所述）在 `plan` 之後插入。狀態列將活動模式顯示為 `default` 的灰色 `⏸ manual mode on`，或作為 `⏵⏵ accept edits on`、`⏸ plan mode on`、`⏵⏵ auto mode on`、`⏵⏵ don't ask on` 或 `⏵⏵ bypass permissions on`。

    並非每個模式都在預設循環中：

    * `auto`：當[自動模式可用](#eliminate-prompts-with-auto-mode)時出現；循環切換到它會在不需要確認提示的情況下切換模式
    * `bypassPermissions`：在您使用 `--permission-mode bypassPermissions`、`--dangerously-skip-permissions`、`--allow-dangerously-skip-permissions` 或[使用者、`--settings` 或受管設定](/docs/zh-TW/settings-reference#permissions-defaultmode)中的 `permissions.defaultMode: "bypassPermissions"` 啟動後出現。`--allow-` 變體將權限模式新增到循環中而不啟動它
    * `dontAsk`：永遠不會在循環中出現；使用 `--permission-mode dontAsk` 設定它

    啟用的可選模式在 `plan` 之後插入，`bypassPermissions` 優先，`auto` 最後。如果您同時啟用了兩者，您將在循環到 `auto` 的途中循環通過 `bypassPermissions`。

    **從 Bash 權限提示**：在 Manual 和 `acceptEdits` 權限模式中，當[自動模式](#eliminate-prompts-with-auto-mode)可用時，Claude Code 會將**是的，並切換到自動模式**新增到 Bash 命令的權限提示。選擇它以批准命令並將工作階段切換到自動模式。[PowerShell 工具](/docs/zh-TW/tools-reference#powershell-tool)提示不提供該選項。需要 Claude Code v2.1.247 或更新版本。

    Claude Code 不會將該選項新增到由您的[`ask` 規則](/docs/zh-TW/permissions#manage-permissions)之一或[hook](/docs/zh-TW/hooks#pretooluse-decision-control)強制的提示，因為自動模式仍會向您顯示這些提示，因此切換不會移除它們。

    **在啟動時**：將權限模式作為旗標傳遞。

    ```bash theme={null}
    claude --permission-mode plan
    ```

    **作為預設值**：在您想要的範圍設定 `permissions.defaultMode`，如[以不同的權限模式啟動](#start-in-a-different-mode)中所述。

    相同的 `--permission-mode` 旗標適用於 `-p` 用於[非互動式執行](/docs/zh-TW/headless)。
  </Tab>

  <Tab title="VS Code">
    **在工作階段期間**：點擊提示框底部的模式指示器。它對此頁面上的模式使用這些標籤：

    | UI 標籤              | 模式                  |
    | :----------------- | :------------------ |
    | Manual             | `default`           |
    | Edit automatically | `acceptEdits`       |
    | Plan               | `plan`              |
    | Auto               | `auto`              |
    | Bypass permissions | `bypassPermissions` |

    **作為預設值**：若要固定對話啟動的權限模式，請在 VS Code 使用者設定中將 `claudeCode.initialPermissionMode` 設定為 `default`、`manual`、`acceptEdits`、`plan` 或 `bypassPermissions`。該設定不接受 `auto`；若要以 Auto 啟動，請將其保留未設定，並從模式指示器中選擇**Auto**一次，如下面第 2 項所述。擴充功能在適用的第一個中啟動每個新對話：

    1. `claudeCode.initialPermissionMode`
    2. 您上次從模式指示器選擇的模式，如果它是 Manual、Edit automatically 或 Auto。選擇 Plan 或 Bypass permissions 僅適用於該對話
    3. 來自[受管設定](/docs/zh-TW/managed-settings)或 `~/.claude/settings.json` 的 `permissions.defaultMode`，在 Pro、Max 和 Team 方案上具有[功能旗標擷取](#which-mode-a-session-starts-in)可用
    4. 您的方案、提供者和組織設定的[內建預設](#which-mode-a-session-starts-in)

    擴充功能永遠不會從專案的 `.claude/settings.json` 或 `.claude/settings.local.json` 讀取起始權限模式，在不符合第 3 項條件的對話中根本不讀取任何設定檔。當設定 `claudeCode.claudeProcessWrapper` 時，第 3 和 4 項也不適用：這些對話以 Manual 啟動，除非第 1 或 2 項設定權限模式。

    當[自動模式可用](#eliminate-prompts-with-auto-mode)時，Auto 會在模式指示器中出現。

    Bypass permissions 需要擴充功能設定中的 **Allow dangerously skip permissions** 切換。沒有它，權限模式不會在指示器中出現，來自第 1 或 3 項的 `bypassPermissions` 值會改為以 Manual 啟動對話。當自動模式不可用時，來自任何項的 Auto 同樣會以 Manual 啟動對話。

    請參閱 [VS Code 指南](/docs/zh-TW/vs-code)以取得擴充功能特定的詳細資訊。
  </Tab>

  <Tab title="JetBrains">
    JetBrains 外掛程式在 IDE 終端中執行 Claude Code，因此切換權限模式的方式與 CLI 中相同：按 `Shift+Tab` 循環切換，或在啟動時傳遞 `--permission-mode`。
  </Tab>

  <Tab title="Desktop">
    **在工作階段期間**：在 Code 標籤中，使用傳送按鈕旁邊的模式選擇器。並非每個模式都會在選擇器中出現：

    * **Auto**：當[自動模式可用](#eliminate-prompts-with-auto-mode)時出現
    * **Bypass permissions**：在 Pro 和 Max 方案上需要 Desktop 設定中的 **Allow bypass permissions mode** 切換；在 Team 和 Enterprise 方案上，組織政策改為控制它

    Cowork 標籤不使用這些模式。Cowork 有自己的權限模式，單獨啟用，Cowork 標籤在為您的帳戶啟用超出其預設值的模式之前根本不顯示模式選擇器。請參閱 [Cowork 文件](https://claude.com/docs/cowork/overview)。

    如需桌面特定的詳細資訊，請參閱 Desktop 指南中的[選擇權限模式](/docs/zh-TW/desktop#choose-a-permission-mode)。

    **作為預設值**：在[設定](/docs/zh-TW/settings#where-settings-live)中設定 `defaultMode`。桌面應用程式讀取與 CLI 相同的設定檔，並將權限模式套用到新的本機工作階段。

    您在模式選擇器中選擇的模式會按資料夾記住，並優先於該資料夾的 `defaultMode`。Plan 是例外：選擇它僅適用於目前工作階段。

    如需 `defaultMode` 在設定檔中的位置，請參閱[以不同的權限模式啟動](#start-in-a-different-mode)下的範例。
  </Tab>

  <Tab title="Web and mobile">
    在 [claude.ai/code](https://claude.ai/code) 或行動應用程式中使用提示框旁邊的模式下拉式選單。權限提示會在 claude.ai 中出現以供批准。出現的模式取決於工作階段在何處執行：

    * **Cloud sessions**：在 [Claude Code on the web](/docs/zh-TW/claude-code-on-the-web) 上：接受編輯、Plan 和 Auto。接受編輯對應於 `default` 模式：雲端工作階段預先批准檔案編輯，無論模式為何，因此下拉式選單會顯示接受編輯而不是 Manual。雲端工作階段仍然遵守設定中的 `defaultMode: "acceptEdits"`。Auto 模式僅在您的組織允許且選定的模型支援時出現。Bypass permissions 不可用。
    * **[Remote Control](/docs/zh-TW/remote-control) sessions** 在您的本機機器上：Manual、接受編輯和 Plan。您無法從應用程式選擇 Auto 或 Bypass permissions。
      * 除了 Bypass permissions，下拉式選單顯示本機工作階段所在的權限模式，包括從終端設定的模式。它在應用程式或終端中權限模式變更時更新。工作階段永遠不會向 claude.ai 報告 Bypass permissions，因此從終端切換到它不會變更下拉式選單顯示的內容。
      * 由[桌面應用程式](/docs/zh-TW/desktop)或 [VS Code 擴充功能](/docs/zh-TW/vs-code)託管的工作階段在權限模式變更時向 claude.ai 報告，與在終端中託管的工作階段相同。
      * 在 v2.1.202 之前，使用 `/remote-control` 或 `claude --remote-control` 連線的工作階段根本不報告其權限模式，因此 claude.ai 和行動應用程式可能會顯示工作階段不在的權限模式。不匹配僅影響標籤。Claude Code 從工作階段的實際權限模式產生權限提示，它們仍然在應用程式中出現以供批准。

    對於 Remote Control，執行工作階段的本機機器必須使用您的 claude.ai 帳戶登入；不支援 API 金鑰。您也可以在啟動該本機工作階段時設定起始權限模式：

    ```bash theme={null}
    claude remote-control --permission-mode acceptEdits
    ```
  </Tab>
</Tabs>

<h2 id="auto-approve-file-edits-with-acceptedits-mode">
  使用 acceptEdits 模式自動批准檔案編輯
</h2>

`acceptEdits` 模式讓 Claude 在您的工作目錄中建立和編輯檔案，無需提示。當此模式處於活動狀態時，狀態列會顯示 `⏵⏵ accept edits on`。

除了檔案編輯外，`acceptEdits` 模式還會自動批准常見的檔案系統 Bash 命令：`mkdir`、`touch`、`rm`、`rmdir`、`mv`、`cp` 和 `sed`。當這些命令以安全環境變數（例如 `LANG=C` 或 `NO_COLOR=1`）或程序包裝器（例如 `timeout`、`nice` 或 `nohup`）作為前綴時，也會自動批准。與檔案編輯一樣，自動批准僅適用於工作目錄或 `additionalDirectories` 內的路徑。超出該範圍的路徑、寫入[受保護路徑](#protected-paths)、`rm` 和 `rmdir` 移除針對[關鍵路徑](#critical-paths)以及所有其他 Bash 命令（除了[內建唯讀集合](/docs/zh-TW/permissions#read-only-commands)）仍會提示。

當[PowerShell 工具](/docs/zh-TW/tools-reference#powershell-tool)啟用時，`acceptEdits` 模式也會自動批准 `Set-Content`、`Add-Content`、`Clear-Content` 和 `Remove-Item` 在範圍內的路徑上，以及它們的常見別名。相同的範圍和受保護路徑規則適用，`Remove-Item` 有[自己的檢查](#remove-item-in-powershell)。包含引號字元的位置引數（例如 `Set-Content .\notes.txt "It's done"` 中的撇號）仍會在範圍內路徑上提示，因為 Claude Code 無法靜態驗證其引用和未引用讀數不同的引數。透過命名參數（例如 `-Value`）傳遞內容以避免提示。

當您想在編輯器中或透過 `git diff` 事後檢查變更，而不是逐個批准每個編輯時，請使用 `acceptEdits`。

從 Manual 模式按一次 `Shift+Tab` 進入它，或直接啟動它：

```bash theme={null}
claude --permission-mode acceptEdits
```

<h2 id="analyze-before-you-edit-with-plan-mode">
  使用 Plan Mode 在編輯前進行分析
</h2>

Plan Mode 告訴 Claude 在進行變更前先研究並提出建議。Claude 會讀取檔案、執行 shell 命令進行探索，並撰寫計畫，但不會編輯您的原始碼。除了在具有[略過權限可用](#skip-all-checks-with-bypasspermissions-mode)的互動式終端機工作階段中，編輯會保持被阻止，直到您核准計畫。

當[自動模式](/docs/zh-TW/auto-mode-config)可用且 `useAutoModeDuringPlan` 設定已開啟（預設為開啟）時，分類器會在規劃期間檢查 shell 命令，而不是提示您。已核准的命令會執行，被拒絕的命令會被阻止。否則，[內建唯讀集合](/docs/zh-TW/permissions#read-only-commands)之外的命令會提示您核准，包括當沙箱的[自動允許模式](/docs/zh-TW/sandboxing#sandbox-modes)已啟用時。在具有可用略過權限的互動式終端機工作階段中，分類器和提示都不適用於規劃命令；[使用 bypassPermissions Mode 略過所有檢查](#skip-all-checks-with-bypasspermissions-mode)涵蓋了仍在該處提示的少數事項。在 v2.1.212 至 v2.1.217 中，沒有略過權限的工作階段會針對唯讀集合之外的每個命令提示，無論自動模式是否可用。

按 `Shift+Tab` 或在單一提示前加上 `/plan` 來進入 Plan Mode。您也可以從 CLI 開始使用 Plan Mode：

```bash theme={null}
claude --permission-mode plan
```

再次按 `Shift+Tab` 以離開 Plan Mode，而不核准計畫。

<h3 id="review-and-approve-a-plan">
  檢查並核准計畫
</h3>

當計畫準備好時，Claude 會呈現它並詢問如何進行。從該提示中，您可以選擇：

* **是的，並使用自動模式**：核准並開始使用[自動模式](#eliminate-prompts-with-auto-mode)。如果自動模式對您的工作階段[不可用](#eliminate-prompts-with-auto-mode)，例如因為您的組織關閉了它，此選項會顯示為**是的，自動接受編輯**。如果您使用啟用的略過權限開始工作階段，該選項會改為顯示**是的，並為此工作階段切換到 BYPASS PERMISSIONS（無進一步提示）**。
* **是的，手動核准編輯**：核准並逐個檢查每個編輯。
* **否，繼續規劃**：保持在 Plan Mode 並告訴 Claude 要變更什麼。

核准計畫會退出 Plan Mode 並將工作階段切換到每個核准選項描述的權限模式，因此 Claude 開始編輯。若要再次規劃，使用 `Shift+Tab` 循環回到 Plan Mode，或在下一個提示前加上 `/plan`。

按 `Ctrl+G` 在您的預設文字編輯器中開啟提議的計畫並在 Claude 繼續前直接編輯它。當[`showClearContextOnPlanAccept`](/docs/zh-TW/settings-reference#showclearcontextonplanaccept)已啟用時，清單會獲得第一個選項，該選項核准計畫並清除規劃內容。

接受計畫也會根據計畫為工作階段提供[產生的標題](/docs/zh-TW/sessions#name-your-sessions)，除非您已經命名了工作階段。

<h3 id="set-plan-mode-as-the-default">
  將 Plan Mode 設定為預設值
</h3>

若要讓 Plan Mode 成為專案終端機工作階段的預設值，請在 `.claude/settings.json` 中將 `defaultMode` 設定為 `plan`，放置如[以不同權限模式開始](#start-in-a-different-mode)下的範例所示。[VS Code 擴充功能](/docs/zh-TW/vs-code)啟動的對話不會讀取專案設定以取得起始權限模式。在那裡，改為在您的 VS Code 使用者設定中將 `claudeCode.initialPermissionMode` 設定為 `plan`。

<h2 id="eliminate-prompts-with-auto-mode">
  使用自動模式消除權限提示
</h2>

自動模式讓 Claude 無需例行權限提示即可執行。一個獨立的分類器模型在操作執行前進行審查，阻止任何超出您請求範圍、針對無法識別的基礎設施或似乎由 Claude 讀取的惡意內容驅動的操作。明確的[詢問規則](/docs/zh-TW/permissions#manage-permissions)仍會強制提示。

在 Pro、Max 和 Team 計畫上，自動模式是[會話開始時的內建預設權限模式](#which-mode-a-session-starts-in)。

分類器也會審查 Claude 使用 [`SendMessage`](/docs/zh-TW/tools-reference) 發送給另一個代理的每條訊息，無論是純文字還是結構化的[代理團隊](/docs/zh-TW/agent-teams)訊息，在 Claude Code 傳遞之前，無論是在自動模式還是在[計畫模式中分類器審查命令](#analyze-before-you-edit-with-plan-mode)時；發送審查需要 Claude Code v2.1.222 或更新版本。

分類器也會審查並批准或阻止針對[關鍵路徑](#critical-paths)的 `rm` 和 `rmdir` 移除，例如 `rm -rf /` 和 `rm -rf ~`，包括當移除位於命令或程序替換內時。

自動模式也會促使 Claude 繼續工作而不停下來提出澄清問題，儘管當您的提示或技能明確依賴它時 Claude 仍會詢問。如需在仍會提示您的模式中獲得更強的自主行為，請改為設定[主動輸出風格](/docs/zh-TW/output-styles)。

<Warning>
  自動模式減少了權限提示，但不保證安全性。將其用於您信任一般方向的任務，而不是作為敏感操作審查的替代品。
</Warning>

自動模式僅在您的帳戶符合以下所有要求時才可用：

* **計畫**：所有計畫。
* **組織**：在 Team 和 Enterprise 上，自動模式預設可用。管理員可以通過在[受管設定](/docs/zh-TW/managed-settings)中將 `permissions.disableAutoMode` 設定為 `"disable"` 來為組織關閉它。
* **模型**：在 Anthropic API 和 [AWS 上的 Claude Platform](/docs/zh-TW/claude-platform-on-aws) 上，Claude Opus 4.6 或更新版本、Sonnet 4.6 或更新版本，或[Fable 模型](/docs/zh-TW/model-config#work-with-fable)。在 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 和已登入的[Claude 應用程式閘道](/docs/zh-TW/claude-apps-gateway)會話上，僅限 Claude Sonnet 5、Opus 4.7 或更新版本以及 Fable 模型。較舊的模型，包括 Sonnet 4.5、Opus 4.5、Haiku 和 claude-3 模型，在任何提供者上都不受支援。
* **提供者**：在 Anthropic API、AWS 上的 Claude Platform、Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 和已登入的 Claude 應用程式閘道會話上預設可用。

如果 Claude Code 報告自動模式不可用，首先檢查這些要求以及任何設定檔是否設定了 [`disableAutoMode`](/docs/zh-TW/settings-reference#disableautomode)。Anthropic 也可能已在伺服器端關閉自動模式，或伺服器可能已為您的帳戶拒絕自動模式。收到任一答案的會話會保持自動模式關閉直到會話結束，因此稍後啟動新會話。

一條單獨的訊息，命名一個模型並說自動模式「無法確定」操作的安全性，意味著分類器請求失敗。該失敗通常是暫時的，但在 Amazon Bedrock 上，它可能會重複出現，直到您的帳戶可以調用命名的模型。請參閱[錯誤參考](/docs/zh-TW/errors#auto-mode-cannot-determine-the-safety-of-an-action)以了解原因和應對方法。

如果您在[設定](/docs/zh-TW/settings-reference#all-settings)中設定 `defaultMode: "auto"`，而終端會話在沒有錯誤的情況下以手動模式啟動，該設定可能位於 `.claude/settings.json` 或 `.claude/settings.local.json` 中。`auto` 不會從這些檔案生效。將其移至 `~/.claude/settings.json`。對於 VS Code 擴充功能啟動的對話，請改為檢查擴充功能自己的列表[切換權限模式](#switch-permission-modes)。

<h3 id="enable-auto-mode-on-bedrock-agent-platform-or-foundry">
  Bedrock、Agent Platform 或 Foundry 上的自動模式
</h3>

在 [Amazon Bedrock](/docs/zh-TW/amazon-bedrock)、[Google Cloud 的 Agent Platform](/docs/zh-TW/google-vertex-ai)、[Microsoft Foundry](/docs/zh-TW/microsoft-foundry) 和已登入的 [Claude 應用程式閘道](/docs/zh-TW/claude-apps-gateway)會話上，自動模式預設出現在 `Shift+Tab` 循環中。出現在循環中不會改變會話開始時的權限模式：在這些提供者上，終端會話以您的 [`defaultMode`](/docs/zh-TW/settings-reference#permissions-defaultmode) 開始，除非您更改它，否則為手動模式，而 [VS Code 擴充功能](/docs/zh-TW/vs-code)中的對話除非 `claudeCode.initialPermissionMode` 或您在擴充功能中選擇的模式設定了一個，否則以手動模式開始。這些提供者上僅支援 Claude Sonnet 5、Opus 4.7 或更新版本以及 Fable 模型。

要使自動模式成為預設啟動權限模式，請在使用者或受管設定中設定 `"permissions": {"defaultMode": "auto"}`。在 VS Code 擴充功能啟動的會話中，改為從模式指示器選擇 **Auto**。[切換權限模式](#switch-permission-modes)涵蓋了什麼優先於該選擇。

[`/doctor`](/docs/zh-TW/commands#all-commands)檢查在這些提供者上提議此使用者設定預設，就像在 Anthropic API 上一樣。

要防止開發人員使用自動模式，請在[受管設定](/docs/zh-TW/managed-settings)中將 `disableAutoMode` 設定為 `"disable"`。這會從 `Shift+Tab` 循環中移除 `auto`，並且以 `--permission-mode auto` 啟動的會話以手動模式啟動。已在自動模式中執行的會話在設定從[管理員部署的來源](/docs/zh-TW/managed-settings#which-managed-source-claude-code-uses)到達該會話時會離開它，並顯示 `auto mode disabled by settings`。在 v2.1.251 之前，執行中的會話會保持自動模式直到它結束。

在 v2.1.158 到 v2.1.206 中，自動模式在這些提供者上是關閉的，直到您設定 `CLAUDE_CODE_ENABLE_AUTO_MODE=1`，並且 Claude Code 在這些提供者上忽略 `defaultMode: "auto"`，除非也設定了該變數。該變數仍被接受以保持相容性，從 v2.1.207 開始沒有效果。

<h3 id="server-side-classifier-review">
  伺服器端分類器審查
</h3>

在自動模式中，Claude Code 可以要求伺服器檢查[決策順序](#how-the-classifier-evaluates-actions)發送進行審查的操作，作為會話模型請求的一部分，而不是發送自己的分類器請求。這些會話詢問：

* **直接連接到 Anthropic API**：在互動式終端會話中，在每個 claude.ai 計畫和使用 Claude API 的帳戶上，隨著 Anthropic 推出。在 Pro、Max 和 Team 計畫上需要 Claude Code v2.1.271 或更新版本，在 Enterprise 計畫和 Claude API 帳戶上需要 v2.1.278 或更新版本。從 v2.1.282 開始，[不提取功能標誌](/docs/zh-TW/env-vars#features-that-need-feature-flag-fetching)的會話，例如因為您關閉了遙測，預設在任何類型的會話中詢問伺服器。
* **雲端提供者、LLM 閘道或代理**：在 [AWS 上的 Claude Platform](/docs/zh-TW/claude-platform-on-aws)、Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上，以及每當您將 `ANTHROPIC_BASE_URL` 指向[LLM 閘道或代理](/docs/zh-TW/llm-gateway)時，無論您的計畫如何。預設詢問伺服器需要 Claude Code v2.1.278 或更新版本。
* **已登入的 [Claude 應用程式閘道](/docs/zh-TW/claude-apps-gateway)會話**：需要 Claude Code v2.1.280 或更新版本

伺服器審查操作的地方，其判決決定了它們。另外兩個結果是可能的：

* **伺服器不審查會話**：回應完成時沒有審查結果，或伺服器回答它不審查此會話。最常見的原因是 LLM 閘道或代理丟棄了審查請求或結果，以及平台、區域或認證還沒有伺服器端檢查。Claude Code 回退到自己的分類器請求。一旦該回退在會話的其餘部分保持，它會在那些請求被計費的帳戶上顯示[關於分類器請求費用的通知](/docs/zh-TW/auto-mode-classifier-billing)。
* **伺服器對操作沒有給出判決**：Claude Code 拒絕該操作而不是執行它未審查。在任何連接上，當回應在審查結果到達之前結束或結果以 Claude Code 無法讀取的形式到達時，會發生這種情況。LLM 閘道或代理可能會導致任一情況。在直接連接到 Anthropic API 時，當伺服器對操作的檢查失敗時，例如超時，也會發生這種情況。[伺服器沒有返回安全判決](/docs/zh-TW/errors#the-server-returned-no-safety-verdict)涵蓋拒絕訊息、拒絕重複時會發生什麼以及應對方法。

要跳過詢問伺服器並始終使用 Claude Code 自己的分類器請求，請設定 [`CLAUDE_CODE_AUTO_MODE_SERVER=0`](/docs/zh-TW/env-vars)。在直接連接到 Anthropic API 時，該變數需要 Claude Code v2.1.281 或更新版本。將其設定為 `1` 會在沒有伺服器審查的會話中打開伺服器審查，例如 `-p` 或 Agent SDK 會話，除非您也設定了 `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`。如果您設定 `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1` 並保持 `CLAUDE_CODE_AUTO_MODE_SERVER` 未設定，Claude Code 也會停止詢問伺服器。

<h3 id="what-the-classifier-blocks-by-default">
  分類器預設阻止的內容
</h3>

分類器信任您的工作目錄和會話啟動時為其配置的遠端。在會話期間使用 `git remote add` 或 `git remote set-url` 添加或重新指向的遠端不受信任，其他所有內容都被視為外部，直到您[配置受信任的基礎設施](/docs/zh-TW/auto-mode-config)。在 v2.1.200 之前，會話中期添加的遠端也受信任。

**預設阻止**：

* 下載並執行程式碼，例如 `curl | bash`
* 將敏感資料發送到外部端點
* 生產部署和遷移
* 雲端儲存上的大量刪除
* 授予 IAM 或儲存庫權限
* 修改共享基礎設施
* 不可逆地銷毀會話前存在的檔案
* 強制推送
* 提交或推送會將秘密或敏感資料發送到儲存庫外的更改，或擴大部署公開的內容。這涵蓋將秘密傳遞給不已接收它的目的地的 CI 工作流程或部署配置、讀取秘密存儲並發送資料的指令碼或設定步驟，以及擴大部署發佈內容的配置更改，例如登錄、可見性、工件或來源地圖設定。檢查適用於任何分支，即使儲存庫是公開的也適用，並在提交或推送時觸發，無論該提交或推送是否觸發管道；清除它需要命名執行效果，而不僅僅是提交或推送。在 v2.1.211 之前，此檢查的範圍限於預設分支：推送到那裡時，如果它攜帶敏感內容、相對於您要求的隱藏或誤述的更改、從儲存庫外部移植的內容或繞過您要求的審查的內容，則被阻止
* `git reset --hard`、`git checkout -- .`、`git restore .`、`git clean -fd`、`git stash drop` 或 `git stash clear`，分類器推測會丟棄未提交的更改
* `git commit --amend` 當 HEAD 的提交不是在此會話中建立的
* 從 v2.1.198 開始，`git commit --amend` 當 HEAD 的提交已經被推送。僅訊息重述不被阻止：`--amend -m` 沒有新暫存的內容，在 Claude 在此會話期間建立的提交上
* `terraform destroy`、`pulumi destroy`、`cdk destroy` 或 `terragrunt destroy`，以及應用銷毀資源的計畫

Claude Code v2.1.195 及更新版本預設阻止更多類別。其中幾個取決於[環境](/docs/zh-TW/auto-mode-config#define-trusted-infrastructure)條目，例如敏感遠端目標和受保護的 IaC 範圍，您可以將其縮小到具體名稱。

* 寫入秘密管理器，或更改 DNS 記錄或 TLS 憑證
* 合併沒有人類批准的拉取請求、批准 Claude 自己的拉取請求或禁用 CI 檢查
* 發佈本身是自動化命令的評論，例如 `atlantis apply` 或機器人的 `/deploy` 或 `/merge`
* 切換、調整或刪除生產功能標誌
* 將基礎設施更改應用於受保護的 IaC 範圍，或排空並移除叢集節點
* 寫入超出您命名的資源的共享計算叢集，例如標籤選擇器或 `--all` 捕獲其他使用者的工作
* 建立在每個節點上執行或攔截叢集流量的 Kubernetes 資源，例如 DaemonSets 和准入 webhooks
* 互動式 shell 或連接埠轉發到敏感遠端目標
* 開啟隧道或反向 shell，使本地服務可從公開網際網路訪問
* 將即時認證或令牌列印到文字記錄或檔案中
* 訪問在您的[環境](/docs/zh-TW/auto-mode-config#define-trusted-infrastructure)中列為敏感資料位置的位置，或從其中複製資料。從 v2.1.198 開始，這也會阻止從一個發送資料到該條目排除的受眾
* 將套件安裝繞過您的內部套件登錄路由到公開登錄。從 v2.1.198 開始，這也適用於您在對話中告訴 Claude 內部登錄或鏡像存在的情況，而不僅僅是在您的環境中列出的情況
* 使用禁用安全防護的標誌執行命令，例如 `--insecure`
* 啟動在沒有人類批准或沙箱的情況下執行的自主代理迴圈，例如使用 `--dangerously-skip-permissions` 或 `--no-sandbox` 啟動的迴圈。從 v2.1.198 開始，這也涵蓋執行第三方代理或評估工具，隔離和按操作批准禁用，例如使用 `--yes-always` 啟動的執行器
* [Chrome 中的 Claude](/docs/zh-TW/chrome)瀏覽器操作，可能會將頁面內容、Cookie 或認證發送到跨來源

Claude Code v2.1.198 及更新版本也預設阻止這些：

* 按萬用字元、glob 或年齡篩選器而不是按特定命名路徑刪除 `/tmp`、`$TMPDIR` 或另一個共享暫存或快取目錄中的檔案
* 在內容發送、上傳、發佈或寫入其他人或共享系統時包含敏感詳細資訊，當您自己的訊息沒有授權這些詳細資訊給該收件人時。PR 和問題正文、提交訊息和評論在儲存庫在信任邊界外或公開時計為此類出站內容，包括您組織自己的公開儲存庫；內部檔案路徑、代碼名稱、即時 API 回應資料（例如電子郵件或帳戶識別碼）和基礎設施識別碼計為敏感詳細資訊。PR、問題和提交訊息範圍需要 Claude Code v2.1.200 或更新版本。PR 或問題正文中的即時個人資料（例如電子郵件地址、帳戶或組織識別碼或使用指標）需要您命名這些詳細資訊和收件人，無論儲存庫的可見性或信任邊界如何。該檢查需要 Claude Code v2.1.203 或更新版本
* 將按鍵發送到 Claude Code 自己的 tmux 窗格以驅動其自己的介面，分類器將其視為 Claude 更改自己的權限或監督

Claude Code v2.1.200 及更新版本也預設阻止這些：

* 註解掉、刪除或強制通過保護安全行為的測試或斷言，例如身份驗證、存取控制、輸入驗證或沙箱
* 刪除或拆除有狀態資源 Claude 在會話中沒有建立的資源，當沒有更具體的刪除規則適用且您沒有命名該資源時
* 在不適合任務的第三方主機上重新指向 API 基礎 URL、代理端點、webhook 接收器或登錄鏡像，包括在 `.env.example` 等範例檔案中
* 使用 `git remote set-url` 或 `git remote add` 更改推送去向，除非您命名了新遠端
* 推送秘密或個人或受信任的資料到已知為公開的儲存庫，或推送不是該儲存庫自己工作一部分的機密材料到那裡。dotfiles 儲存庫自己的主題是個人或受信任資料的唯一例外，來自私有儲存庫到任何公開表面的內容以相同方式被阻止；兩項改進都需要 Claude Code v2.1.203 或更新版本。在 v2.1.203 之前，個人資料與機密材料分組，僅當它不是該儲存庫自己工作的一部分時才被阻止。當儲存庫的可見性未確定時，分類器不會單獨基於此進行阻止；它改為根據其他規則判斷內容
* 針對不同儲存庫或組織開啟拉取請求、使用 `gh repo fork` 進行分叉或推送到第三方儲存庫，除非您命名了該外部目標

Claude Code v2.1.203 及更新版本也預設阻止這些：

* 來自敏感本地存儲或其名稱、路徑或類型將其標記為敏感的檔案的內容進入提交、推送、PR 或問題文字、gist 或貼上或套件發佈，除非您命名了來源和目的地。會話文字記錄和對話日誌、認證和配置點資料夾（例如 SSH 金鑰、雲端認證、瀏覽器設定檔和 shell 歷史記錄）以及使用者資料匯出都計為此類，儲存庫是私有的不會清除它

Claude Code v2.1.205 及更新版本也預設阻止這些：

* 寫入 Claude Code 會話文字記錄、`~/.claude/projects/` 下的 `.jsonl` 歷史檔案或您配置的配置目錄，無論是直接還是通過 shell 命令。該規則也涵蓋 Claude Code 為其自己的檢查附加到每個文字記錄條目的中繼資料行。讀取文字記錄不被阻止
* 遞迴強制刪除，例如 `rm -rf "$VAR"` 或 `Remove-Item -Recurse -Force $dir`，其目標是 shell 變數或以其為根的 glob，在對話中分類器看到的任何地方都沒有指派。該值僅來自較早的命令輸出，分類器永遠不會收到，因此分類器無法根據其他刪除規則驗證刪除目標。當您命名正在刪除的確切路徑或 Claude 使用寫入命令的已解析文字路徑重新執行刪除時，該塊會清除。分類器可以解析其目標的刪除不受影響。`Remove-Item` 目標是裸 `*` 或以 `/*` 或 `\*` 結尾的目標永遠不會到達分類器：Claude Code [直接拒絕它們](#remove-item-in-powershell)

Claude Code v2.1.257 及更新版本也預設阻止這些：

* 從雲端實例中繼資料端點（例如 `169.254.169.254`）請求認證，或使用機器自己的服務帳戶或節點身份明確驗證雲端、叢集或登錄呼叫
* 通過直接請求以外的路由到達公開主機，例如隧道、反向 shell 或重寫為指向外部的解析器或代理配置
* 讀取屬於主機而不是您的任務的認證，例如節點憑證或節點的容器登錄身份驗證
* 連接到或掃描 Claude 沒有啟動的同級容器、pod 或 VM，或容器下的節點

如果 Claude Code 在允許其中之一的地方執行，請在 `autoMode.environment` 中的[主機包含條目](/docs/zh-TW/auto-mode-config#define-trusted-infrastructure)中描述該設定。

Claude Code v2.1.261 及更新版本也預設阻止這些：

* 在訊息、PR 或問題文字、文件或連結將被開啟或提取的任何其他地方發佈或寫入公開貼上、圖表或資料共享服務的連結，當 URL 本身攜帶正在共享的內容時，除非您命名了該服務

**預設允許**：

* 您工作目錄中的本地檔案操作
* 安裝在您的鎖定檔案或清單中聲明的依賴項
* 讀取 `.env` 並將認證發送到其匹配的 API
* 唯讀 HTTP 請求
* 推送到您正在處理的儲存庫的任何分支，包括預設分支。其名稱將其標記為部署或發佈目標的非預設分支，例如 `production` 或 `gh-pages`，不涵蓋：分類器根據其自己的條款判斷推送到那裡。推送的內容仍根據其他規則進行檢查，[`permissions.deny` 規則](/docs/zh-TW/permissions#manage-permissions)仍可以在每種模式中[按書寫](/docs/zh-TW/permissions#bash-rule-limits)阻止推送命令，遠端自己的分支保護仍然適用。在 v2.1.211 之前，僅允許推送到您啟動的分支、Claude 建立的分支和到預設分支的例行推送，在 v2.1.203 之前任何直接推送到預設分支都被阻止

Claude Code v2.1.195 及更新版本也預設允許這些：

* 刪除 Claude 在同一會話中較早建立的確切工作
* 作為您的任務的一部分讀取、審查或寫入安全相關的程式碼、配置和威脅模型
* 在同一多代理會話中一起工作的代理之間的訊息
* 將資料發送到您在 [`environment`](/docs/zh-TW/auto-mode-config#define-trusted-infrastructure) 中列出的受信任域、儲存桶和服務。這僅涵蓋資料流，而不是同一基礎設施上的破壞性或認證操作
* [Chrome 中的 Claude](/docs/zh-TW/chrome)導航到受信任的內部域、localhost 或您命名的 URL

沙箱命令預設沒有網路存取。Claude 在命令本身上命名命令需要的主機，分類器與命令一起審查它們，批准的列表僅為該一個命令開啟這些主機。[按命令允許的域](/docs/zh-TW/sandboxing#per-command-allowed-domains-in-auto-mode)涵蓋列表可以和不能開啟什麼以及當命令到達未列出的主機時會發生什麼。

執行 `claude auto-mode defaults` 以將完整規則列表列印為 JSON。如果例行操作被阻止，管理員可以通過 `autoMode.environment` 設定添加受信任的儲存庫、儲存桶和服務：請參閱[配置自動模式](/docs/zh-TW/auto-mode-config)。

推送到您正在處理的儲存庫的任何分支並建立與您的請求相符的拉取請求無需提示即可執行，除非推送或拉取請求屬於[阻止列表](#what-the-classifier-blocks-by-default)，例如秘密或敏感資料離開儲存庫，或針對不同儲存庫或組織的拉取請求。要在保持自動模式的同時要求這些命令前的人類檢查點，請添加 `permissions.ask` 規則，這些規則與命令[按書寫](/docs/zh-TW/permissions#bash-rule-limits)相符：請參閱[常見邊界](/docs/zh-TW/auto-mode-config#common-boundaries)。

<h3 id="first-read-outside-the-working-directories">
  工作目錄外的第一次讀取
</h3>

當 [`permissions.blockReadsOutsideWorkingDirectories`](/docs/zh-TW/settings-reference#permissions-blockreadsoutsideworkingdirectories) 關閉時，檔案讀取在自動模式中無需提示即可執行，包括在[工作目錄](/docs/zh-TW/permissions#working-directories)外的讀取。Claude 第一次在工作目錄外的路徑上使用 Read、Grep 或 Glob 工具時，Claude Code 會詢問您是否繼續允許這些讀取。

該提示不會出現在非互動式 `-p` 執行或背景會話中；那裡的讀取照常執行。

無論您的答案如何，Claude 都會繼續工作：

* **繼續允許**：讀取執行，稍後對工作目錄外的讀取照常執行，Claude Code 記錄您的答案，以便提示不會再次出現
* **從現在開始阻止**：讀取被拒絕，Claude Code 在您的使用者設定中將 [`permissions.blockReadsOutsideWorkingDirectories`](/docs/zh-TW/settings-reference#permissions-blockreadsoutsideworkingdirectories) 設定為 `true`，這使檔案工具在每個稍後的會話和每種權限模式中拒絕此類讀取。要稍後讓 Claude 讀取此類路徑，請使用 `/add-dir` 添加其目錄或移除該設定。
* **下次再詢問**：讀取被拒絕，下一次對工作目錄外的讀取會再次提示

<h3 id="boundaries-you-state-in-conversation">
  您在對話中陳述的邊界
</h3>

分類器將您在對話中陳述的邊界視為阻止信號。如果您告訴 Claude「不要推送」或「在我審查後再部署」，分類器會阻止匹配的操作，即使預設規則會允許它們。邊界保持有效，直到您在稍後的訊息中解除它。Claude 自己的判斷認為條件已滿足不會解除它。

邊界不作為規則儲存。分類器在每次檢查時從文字記錄重新讀取它們，因此如果[上下文壓縮](/docs/zh-TW/costs#reduce-token-usage)移除陳述邊界的訊息，邊界可能會丟失。為了硬保證，請改為添加[拒絕規則](/docs/zh-TW/permissions#permission-rule-syntax)。

<h3 id="approvals-you-state-in-conversation">
  您在對話中陳述的批准
</h3>

如果您告訴 Claude 被阻止的操作是允許的，分類器會將其讀取為您的批准，並可以清除該塊。您如何措辭決定了操作是否執行以及批准的範圍有多遠：

* **命名操作及其細節**：您的訊息必須命名操作和使其危險的具體事物，例如強制推送的分支。僅命名動詞不會清除任何內容，因此「您可以強制推送」會使塊保持原位。
* **期望它涵蓋一個操作**：批准涵蓋您命名的破壞性操作，因此稍後的操作會再次被阻止，除非您授予批准作為常設。要停止一次批准一個例行模式，請將其添加到 [`autoMode.allow`](/docs/zh-TW/auto-mode-config#override-the-block-and-allow-rules)。
* **某些塊保持原位**：[分類器的優先順序](/docs/zh-TW/auto-mode-config#override-the-block-and-allow-rules)列出了您的批准可以到達的塊。要執行它不會清除的步驟，[離開自動模式](#switch-permission-modes)並回答權限提示。

<h3 id="when-auto-mode-falls-back">
  當自動模式回退時
</h3>

當自動模式無法批准您的會話操作時，會發生什麼取決於情況：

* **被阻止的操作**：Claude Code 顯示通知並在 `/permissions` 下的 **Recently denied** 標籤中列出操作，您可以按 `r` 使用手動批准重試它。當分類器對操作[沒有產生判決](/docs/zh-TW/errors#auto-mode-cannot-determine-the-safety-of-an-action)時，因為自動模式以外的安全檢查拒絕了分類器自己的請求或其回應沒有解析，Claude Code 拒絕該操作而沒有通知或 **Recently denied** 條目。
* **重複塊**：如果分類器連續 3 次或總共 20 次阻止操作，自動模式暫停，Claude Code 恢復提示。批准提示的操作會恢復自動模式。這些閾值不可配置。任何允許的操作重置連續計數器，而總計數器在會話期間持續，僅在其自己的限制觸發回退時重置。當[自動模式以外的安全檢查拒絕分類器的請求](/docs/zh-TW/errors#auto-mode-cannot-determine-the-safety-of-an-action)時，Claude Code 不會計算拒絕以達到任一閾值；連結的條目涵蓋 Claude Code 如何處理這些拒絕。
* **無法提示的會話**：沒有 [`--permission-prompt-tool`](/docs/zh-TW/cli-reference#cli-flags) 的[非互動式](/docs/zh-TW/headless) `-p` 執行沒有回退提示。當重複塊達到閾值時，操作不執行，Claude 繼續工作。當[自動模式以外的安全檢查拒絕分類器的請求](/docs/zh-TW/errors#auto-mode-cannot-determine-the-safety-of-an-action)時也適用相同情況。Claude Code 在任一情況下都不會停止執行。
* **伺服器沒有返回安全判決**：在[伺服器端分類器審查](#server-side-classifier-review)下，Claude Code 拒絕伺服器沒有給出判決的操作，並在一行中十個回應沒有判決後停止轉向。請參閱[伺服器沒有返回安全判決](/docs/zh-TW/errors#the-server-returned-no-safety-verdict)。
* **檢查期間的模式切換**：如果您在分類器檢查待處理時切換權限模式，Claude Code 會丟棄新模式不會請求的判決，而不是應用它：您會被提示進行批准，或在 [`dontAsk` 模式](#allow-only-pre-approved-tools-with-dontask-mode)中自動拒絕操作。

重複塊通常意味著分類器缺少關於您的基礎設施的上下文。使用 `/feedback` 報告誤報，或讓管理員[配置受信任的基礎設施](/docs/zh-TW/auto-mode-config)。

<span id="how-the-classifier-evaluates-actions" />

<AccordionGroup>
  <Accordion title="分類器如何評估操作">
    每個操作都經過固定的決策順序。第一個匹配的步驟獲勝：

    1. 與您的[允許、詢問或拒絕規則](/docs/zh-TW/permissions#manage-permissions)相符的操作立即解決，但以下例外：
       * 寫入[受保護路徑](#protected-paths)的操作會路由到分類器，即使允許規則相符，`rm` 和 `rmdir` 移除針對 Claude Code v2.1.218 及更新版本中的[關鍵路徑](#critical-paths)也是如此
       * 標記為 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具即使允許規則相符也會直接提示您，您的組織設定為 [`ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools) 的連接器工具在該設定到達 Claude Code 的會話中也是如此
       * 攜帶[按命令允許的域](/docs/zh-TW/sandboxing#per-command-allowed-domains-in-auto-mode)的 shell 命令也會路由到分類器，即使允許規則相符，因為規則批准命令，而不是其主機
       * 在命令內容上相符的詢問規則，例如 `Bash(git push *)`，回退到權限提示
    2. 唯讀操作和您工作目錄中的檔案編輯會自動批准，除了寫入[受保護路徑](#protected-paths)和[工作目錄外的第一次讀取](#first-read-outside-the-working-directories)，這會提示您
       * 在具有[伺服器端分類器審查](#server-side-classifier-review)的會話中，唯讀和[沙箱](/docs/zh-TW/sandboxing#sandbox-modes) shell 命令等待該審查，如果它標記它們則被阻止
    3. 其他所有內容都進入分類器。在步驟 1 中直接提示您的連接器工具和 `requiresUserInteraction` MCP 工具永遠不會到達分類器，因此既不是組織要求的批准也不是同意步驟會自動批准
    4. 如果分類器阻止，Claude 收到原因並嘗試替代方案。在大多數會話中，原因命名分類器相符的規則，例如 `[Data Exfiltration]`，而不是給出書面解釋；請參閱[審查拒絕](/docs/zh-TW/auto-mode-config#review-denials)

    進入自動模式時，授予任意程式碼執行的廣泛允許規則會被丟棄：

    * 無條件 `Bash(*)` 或 `PowerShell(*)`
    * 萬用字元解釋器，例如 `Bash(python*)`
    * 套件管理器執行命令
    * `Agent` 允許規則
    * [`Monitor`](/docs/zh-TW/tools-reference#monitor-tool) 允許規則，因為 Claude Code 通過 shell 執行 Monitor 命令

    狹窄的規則，例如 `Bash(npm test)` 保持有效。Claude Code 在您離開自動模式時恢復丟棄的規則。在 v2.1.236 之前，Claude Code 在自動模式中保持 `Monitor` 允許規則有效，因此與整個工具相符的規則批准 Monitor 命令而無需分類器審查。

    Claude Code 也在會丟棄未提交工作的命令（例如 `git reset --hard` 或 `rm -rf`）之前自己執行 `git status`，並向分類器顯示是否存在暫存、修改或未追蹤的工作。Claude Code 在該檢查中報告未追蹤的檔案，即使儲存庫的 git 配置設定 `status.showUntrackedFiles=no`。

    在 Claude Code 本身發送的分類器請求中，分類器看到使用者訊息、除了唯讀查詢（例如檔案讀取和搜尋）之外的工具呼叫以及您的 CLAUDE.md 內容。工具結果從這些請求中被剝離，因此檔案或網頁中的惡意內容無法直接操縱分類器。

    您可以使用 [PostToolUse hook 的 `classifierContext` 欄位](/docs/zh-TW/hooks#annotate-a-result-for-the-auto-mode-classifier)註解呼叫的結果，分類器將其讀取為應用程式提供的上下文。該欄位需要 Claude Code v2.1.236 或更新版本。

    一個單獨的伺服器端探針掃描傳入的工具結果並在 Claude 讀取之前標記可疑內容。有關這些層如何協同工作的更多資訊，請參閱[自動模式公告](https://claude.com/blog/auto-mode)和[工程深入探討](https://www.anthropic.com/engineering/claude-code-auto-mode)。
  </Accordion>

  <Accordion title="自動模式如何處理子代理">
    分類器在三個點檢查[子代理](/docs/zh-TW/sub-agents)工作：

    1. 在子代理啟動之前，委派的任務描述被評估，因此看起來危險的任務在生成時被阻止。
    2. 當子代理執行時，其每個操作都通過分類器，使用與父會話相同的規則，子代理前言中的任何 `permissionMode` 都被忽略。
    3. 當子代理完成時，分類器審查其工作和最終報告，然後父代讀取報告。當分類器標記子代理的工作或報告，或單獨的 API 安全檢查拒絕審查時，報告仍被傳遞，前面加上安全警告。當分類器對審查不可用時，報告到達時帶有驗證子代理工作的說明，然後再根據它採取行動。
  </Accordion>

  <Accordion title="成本和延遲">
    分類器預設在 Claude Sonnet 5 上執行，而不是在您的 `/model` 選擇上。Anthropic 配置伺服器端的分類器模型優先於該預設。當您的會話模型是 Claude Sonnet 4.6，或當 [`availableModels`](/docs/zh-TW/model-config#restrict-model-selection) 排除 Sonnet 5 時，分類器改為在會話模型上執行，或在會話在[Fable 模型](/docs/zh-TW/model-config#work-with-fable)上執行時在 Opus 模型上執行；在 Anthropic API 以外的提供者上，該 Opus 回退是提供者的預設 Opus 模型。

    會話的第一個自動模式請求驗證 Sonnet 5 預設：如果請求成功，Sonnet 5 保持會話的分類器模型，如果它失敗因為模型不可用，會話改為使用回退。在該驗證解決後，分類器的模型在會話期間不會改變。

    在 Enterprise 計畫和使用 Claude API 的帳戶上，[AWS 上的 Claude Platform](/docs/zh-TW/claude-platform-on-aws)、Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry，分類器呼叫計入您的令牌使用量。每次檢查發送文字記錄的一部分加上待處理操作，在執行前添加往返。讀取和受保護路徑外的工作目錄編輯跳過分類器，因此開銷主要來自 shell 命令和網路操作。伺服器審查它們作為會話模型請求的一部分的地方，沒有單獨的分類器呼叫要計數；請參閱[伺服器端分類器審查](#server-side-classifier-review)。

    沙箱網路存取不添加按連接分類器請求。分類器判斷[命令命名的主機](/docs/zh-TW/sandboxing#per-command-allowed-domains-in-auto-mode)與命令一起在一次審查中，Claude Code 檢查每個連接對批准列表而無需再次呼叫分類器。
  </Accordion>
</AccordionGroup>

<h2 id="allow-only-pre-approved-tools-with-dontask-mode">
  使用 dontAsk 模式僅允許預先批准的工具
</h2>

如果您設定 `dontAsk` 模式，Claude Code 會自動拒絕所有原本會提示的工具呼叫。Claude 仍會執行在 Manual 模式中不需要批准的操作，例如您工作目錄內的檔案讀取和[唯讀 Bash 命令](/docs/zh-TW/permissions#read-only-commands)，加上符合您 `permissions.allow` 規則的操作和由 [PreToolUse hook](/docs/zh-TW/permissions#extend-permissions-with-hooks) 批准的呼叫。在您預先定義 Claude 可以執行的確切操作的 CI 管道或受限環境中使用此模式；工作階段永遠不會等待輸入。此模式啟用時，狀態列會顯示 `⏵⏵ don't ask on`。

Claude Code 會拒絕符合您明確 [`ask` 規則](/docs/zh-TW/permissions#manage-permissions)的呼叫，而不是提示。它也會拒絕內建的 `AskUserQuestion` 工具，即使您的允許規則符合它，以及您的組織[設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools)的連接器工具在該設定到達 Claude Code 的工作階段中。它以相同方式拒絕標記為 [`_meta["anthropic/requiresUserInteraction"]`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具，因為它們的批准卡需要此模式永遠不會收集的答案；這需要 Claude Code v2.1.199 或更新版本。

`rm` 和 `rmdir` 移除針對[關鍵路徑](#critical-paths)，例如 `rm -rf /` 和 `rm -rf ~`，即使允許規則或 `PreToolUse` hook 允許它們也被拒絕。

[Claude Code on the web](/docs/zh-TW/claude-code-on-the-web) 上的雲端工作階段會忽略 `defaultMode: "dontAsk"`；詳見 [bypassPermissions](#skip-all-checks-with-bypasspermissions-mode) 以了解詳情。

在啟動時使用旗標設定：

```bash theme={null}
claude --permission-mode dontAsk
```

<h2 id="skip-all-checks-with-bypasspermissions-mode">
  使用 bypassPermissions 模式跳過所有檢查
</h2>

`bypassPermissions` 模式會停用權限提示和安全檢查，使工具呼叫立即執行，包括寫入[受保護路徑](#protected-paths)。

[任何模式都不會自動批准的操作](#actions-no-mode-auto-approves)在此模式中仍會提示。

兩個[跨工作階段訊息傳遞](/docs/zh-TW/cross-session-messaging)保護措施在此模式中仍然適用，以及在有可用的略過權限的計畫模式工作階段中：

* 針對超出此機器的工作階段訊息的 [`isolatePeerMachines`](/docs/zh-TW/settings-reference#isolatepeermachines) 核准提示仍會出現。
* 當沒有 [`crossSessionInbound`](/docs/zh-TW/cross-session-messaging#control-inbound-messages) 值適用時，Claude Code 會保留來自您另一個工作階段的入站訊息以供您核准，只有當傳送工作階段識別自己也在略過權限提示時才會無需詢問即傳遞。如果您在保留訊息時離開權限模式，Claude Code 會重新套用入站規則，並傳遞任何現在接受的保留訊息。

在有可用略過權限的互動式終端工作階段中，Claude Code 也不會強制執行[計畫模式的](#analyze-before-you-edit-with-plan-mode)區塊。Claude 仍被指示在計畫時不進行編輯，但它在計畫期間嘗試的任何檔案編輯或 shell 命令都會無需提示即執行。明確的[詢問規則](/docs/zh-TW/permissions#manage-permissions)和針對[關鍵路徑](#critical-paths)的 `rm` 和 `rmdir` 移除仍會提示。

計畫模式在 Claude Code 執行時沒有互動式終端的任何地方都保持其區塊，包括使用 `-p` 的[非互動式執行](/docs/zh-TW/headless)、[Agent SDK](/docs/zh-TW/agent-sdk/permissions#plan-mode-plan) 工作階段，以及 [VS Code 擴充功能](/docs/zh-TW/vs-code)的聊天面板中的對話。在那裡，`--allow-dangerously-skip-permissions` 使 `bypassPermissions` 稍後可選。

<Warning>
  只在隔離環境（如容器、虛擬機或沒有網際網路存取的開發容器）中使用此模式，其中 Claude Code 無法損害您的主機系統。
</Warning>

您無法從未啟用此模式的工作階段進入 `bypassPermissions`。在啟動時使用 [`permissions.defaultMode: "bypassPermissions"`](/docs/zh-TW/settings-reference#permissions-defaultmode) 或使用啟用旗標來啟用它：

```bash theme={null}
claude --permission-mode bypassPermissions
```

`--dangerously-skip-permissions` 旗標是等效的。

Claude Code 拒絕在您使用 [`--restricted`](/docs/zh-TW/cli-reference#cli-flags) 啟動的工作階段中使用 `bypassPermissions`。`--restricted` 需要 Claude Code v2.1.248 或更新版本。

第一次使用此模式啟動互動式工作階段時，Claude Code 會顯示警告對話框，要求您接受對無權限檢查執行的動作負責。Claude Code 會將您的接受儲存到使用者設定，因此對話框只會出現一次。如果您拒絕，Claude Code 會結束。在[非互動模式](/docs/zh-TW/headless)中不會顯示對話框，使用 `--bg` 啟動的[背景工作階段](/docs/zh-TW/agent-view)會被拒絕，直到您在互動式工作階段中接受對話框。

在 Linux 和 macOS 上，當以 root 或 `sudo` 身份執行時，Claude Code 拒絕以此模式啟動：

```text theme={null}
--dangerously-skip-permissions cannot be used with root/sudo privileges for security reasons
```

在識別的沙箱內會自動跳過檢查。若要在容器中自主執行，請使用[開發容器](/docs/zh-TW/devcontainer)設定，它以非 root 使用者身份執行 Claude Code。

[網路上的 Claude Code](/docs/zh-TW/claude-code-on-the-web) 不會遵守您設定檔中的 `defaultMode: "bypassPermissions"` 或 `"dontAsk"`，因此儲存庫的簽入設定無法在略過權限模式下啟動雲端工作階段。該設定會被無聲地忽略，工作階段會改為以模式下拉式選單中顯示的權限模式啟動。請參閱[切換權限模式](#switch-permission-modes)以了解雲端工作階段提供哪些模式。

<Warning>
  `bypassPermissions` 不提供針對提示注入或意外動作的保護。若要進行背景安全檢查且權限提示大幅減少，請改用[自動模式](#eliminate-prompts-with-auto-mode)。管理員可以透過在[受管設定](/docs/zh-TW/managed-settings)中將 `permissions.disableBypassPermissionsMode` 設定為 `"disable"` 來封鎖此模式。
</Warning>

<h2 id="protected-paths">
  受保護的路徑
</h2>

對於一小組路徑的寫入操作永遠不會自動批准，唯一的例外是 `bypassPermissions` 模式，以及可使用[略過權限](#skip-all-checks-with-bypasspermissions-mode)的計畫模式互動式終端工作階段。這可以防止意外損壞儲存庫狀態和 Claude 自身的設定。

| 模式                      | 受保護路徑寫入                                                                                                                                   |
| :---------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| `default`、`acceptEdits` | 提示                                                                                                                                        |
| `plan`                  | 在[略過權限](#skip-all-checks-with-bypasspermissions-mode)可用的互動式終端工作階段中允許。否則，當[自動模式](#eliminate-prompts-with-auto-mode)在規劃期間可用時路由到分類器，當它不可用時提示 |
| `auto`                  | 路由至分類器                                                                                                                                    |
| `dontAsk`               | 拒絕                                                                                                                                        |
| `bypassPermissions`     | 允許                                                                                                                                        |

在使用 [`--restricted`](/docs/zh-TW/cli-reference#cli-flags) 啟動的工作階段中，需要 Claude Code v2.1.248 或更新版本，分類器無法批准受保護路徑寫入。

設定檔案中的 [`permissions.allow`](/docs/zh-TW/permissions#manage-permissions) 規則不會預先批准受保護路徑的寫入。安全檢查在 Claude Code 評估設定中的允許規則之前執行，因此在 `~/.claude/settings.json` 或 `.claude/settings.json` 中的 `Edit(.claude/**)` 之類的項目不會改變上表中的每個模式結果。在提示的模式中，`.claude/` 寫入的提示會提供**是的，並允許 Claude 在此工作階段編輯其自身設定**，這會在該工作階段中批准後續的 `.claude/` 寫入而無需再次提示。

受保護的目錄：

* `.git`
* `.config/git`
* `.vscode`
* `.idea`
* `.husky`
* `.cargo`
* `.devcontainer`
* `.yarn`
* `.mvn`
* `.claude`，除了 `.claude/worktrees` 其中 Claude 儲存其自身的 git worktrees

受保護的檔案：

* `.gitconfig`、`.gitmodules`
* `.bashrc`、`.bash_profile`、`.bash_login`、`.bash_aliases`、`.bash_logout`、`.zshrc`、`.zprofile`、`.zshenv`、`.zlogin`、`.zlogout`、`.profile`、`.envrc`
* `.npmrc`、`.yarnrc`、`.yarnrc.yml`、`.pnp.cjs`、`.pnp.loader.mjs`、`.pnpmfile.cjs`、`bunfig.toml`、`.bunfig.toml`
* `.bazelrc`、`.bazelversion`、`.bazeliskrc`
* `.pre-commit-config.yaml`、`lefthook.yml`、`lefthook.yaml`、`.lefthook.yml`、`.lefthook.yaml`
* `gradle-wrapper.properties`、`maven-wrapper.properties`
* `.devcontainer.json`
* `.ripgreprc`、`pyrightconfig.json`
* `.mcp.json`、`.claude.json`

<h2 id="critical-paths">
  關鍵路徑
</h2>

Claude Code 永遠不會讓 [`permissions.allow`](/docs/zh-TW/permissions#manage-permissions) 規則或返回 `"allow"` 的 [`PreToolUse` hook](/docs/zh-TW/permissions#extend-permissions-with-hooks) 批准針對關鍵路徑的 `rm` 或 `rmdir` 命令，即使在跳過其他提示的模式中。此斷路器防止模型錯誤。符合的拒絕規則仍會完全阻止命令。

會發生什麼取決於您的權限模式：

| 模式                      | Claude Code 對關鍵路徑移除的操作                                                            |
| :---------------------- | :-------------------------------------------------------------------------------- |
| `default`、`acceptEdits` | 要求您批准它                                                                            |
| `plan`                  | 要求您批准它。當[自動模式在規劃期間可用](#analyze-before-you-edit-with-plan-mode)且沒有略過權限可用時，改為傳送到分類器 |
| `auto`                  | 傳送到[分類器](#eliminate-prompts-with-auto-mode)                                       |
| `dontAsk`               | 拒絕它                                                                               |
| `bypassPermissions`     | 要求您批准它                                                                            |

如果明確的[詢問規則](/docs/zh-TW/permissions#manage-permissions)符合命令，Claude Code 即使在 `auto` 模式中也會詢問您。在詢問的模式中，[`PermissionRequest` hook](/docs/zh-TW/hooks#permissionrequest) 可以像回答任何其他提示一樣回答提示。

Claude Code 將 `rm` 或 `rmdir` 目標視為關鍵路徑，當它是以下任何一個時：

* 檔案系統根目錄
* 頂級目錄，意思是根目錄的任何直接子目錄，例如 `/usr`、`/etc` 或 `/data`
* 您的主目錄
* Windows 磁碟機根目錄及其頂級目錄，例如 `C:\` 和 `C:\Windows`
* 您的工作目錄及其父目錄
* 您的其他工作目錄及其父目錄，但僅當移除是其中一個下的 glob 時，例如 `rm -rf <dir>/*`。`rm -rf <dir>` 在目錄本身上不會觸發此檢查

Claude Code 也將直接在 shell 變數下的 glob 或尾部斜線視為關鍵路徑移除，例如 `rm -rf "$DIR"/*`，因為當變數為空時命令變成從檔案系統根目錄的移除。

此變數情況的提示會命名被標記的 `rm` 並說明如何重寫它以便檢查通過：

* 對於像 `$DIR` 這樣的變數，保護每個擴展，使得當變數未設定或為空時 shell 會停止並出現錯誤，如 `rm -rf "${DIR:?}"/*`，或使用字面路徑
* 對於通常已設定的變數，例如 `$HOME`，使用字面路徑

其擴展都以這種方式保護的移除不是關鍵路徑移除，因此在 `bypassPermissions` 模式中它會在沒有提示的情況下執行。

使用 `(...)` 的子殼層、使用 `{ ...; }` 的大括號群組、使用 `$(...)` 或反引號的命令替換，或使用 `<(...)` 的程序替換隱藏移除，不會跳過檢查。Claude Code 找到關鍵路徑移除，無論它位於巢狀形式內部（如 `(rm -rf ~)` 或 `echo "$(rm -rf ~)"`），還是位於同一命令中的其他地方。

<h3 id="remove-item-in-powershell">
  PowerShell 中的 Remove-Item
</h3>

當您啟用 [PowerShell 工具](/docs/zh-TW/tools-reference#powershell-tool)時，Claude Code 為 `Remove-Item` 提供自己的檢查，與 `rm` 關鍵路徑清單分開。結果取決於目標，第一個匹配的情況適用：

* **系統路徑**：檔案系統根目錄及其頂級目錄、磁碟機根目錄及其頂級目錄以及您的主目錄。Claude Code 在每種模式中拒絕命令，無需詢問您。
* **萬用字元**：裸 `*` 或任何以 `/*` 或 `\*` 結尾的目標，包括 shell 變數下的 glob，例如 `$dir/*`。Claude Code 在每種模式中拒絕命令，無需詢問您，在[分類器](#eliminate-prompts-with-auto-mode)看到它之前。
* **您的工作目錄或其中一個父目錄，使用 `-Recurse`**：Claude Code 將命令視為任何其他在您的權限模式中需要批准的命令，因此它在詢問的模式中詢問您，在 `auto` 模式中傳送到分類器，在 `dontAsk` 模式中拒絕它。`bypassPermissions` 模式跳過此檢查。

<h2 id="see-also">
  另請參閱
</h2>

* [Permissions](/docs/zh-TW/permissions)：allow、ask 和 deny 規則；受管理的原則
* [Configure auto mode](/docs/zh-TW/auto-mode-config)：告訴分類器您的組織信任哪些基礎設施
* [Hooks](/docs/zh-TW/hooks)：透過 `PreToolUse` 和 `PermissionRequest` hooks 的自訂權限邏輯
* [Security](/docs/zh-TW/security)：保護措施和最佳實踐
* [Sandboxing](/docs/zh-TW/sandboxing)：Bash 命令的檔案系統和網路隔離
* [Non-interactive mode](/docs/zh-TW/headless)：使用 `-p` 旗標執行 Claude Code
