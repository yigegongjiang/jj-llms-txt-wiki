> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 為您的組織設定 Claude Code

> 管理員部署 Claude Code 的決策地圖，涵蓋 API 提供者、受管設定、政策執行、使用情況監控和資料處理。

Claude Code 透過受管設定來執行組織政策，這些設定優先於本地開發人員配置。您可以從 Claude 管理員控制台、行動裝置管理 (MDM) 系統或磁碟上的檔案傳遞這些設定。這些設定控制 Claude 可以存取的工具、命令、伺服器和網路目的地。

本頁按順序介紹部署決策。每一行都連結到下面的部分和該區域的參考頁面。

<Note>
  SSO、SCIM 佈建和座位分配在 Claude 帳戶級別進行配置。有關這些步驟，請參閱 [Claude 企業管理員指南](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide) 和 [座位分配](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan)。
</Note>

| 決策                                               | 您正在選擇什麼                 | 參考                                                                                                                                                                                     |
| :----------------------------------------------- | :---------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [選擇您的 API 提供者](#choose-your-api-provider)        | Claude Code 驗證的位置以及如何計費 | [Authentication](/docs/zh-TW/authentication)、[Amazon Bedrock](/docs/zh-TW/amazon-bedrock)、[Google Cloud's Agent Platform](/docs/zh-TW/google-vertex-ai)、[Microsoft Foundry](/docs/zh-TW/microsoft-foundry) |
| [決定設定如何到達裝置](#decide-how-settings-reach-devices) | 受管政策如何到達開發人員機器          | [Server-managed settings](/docs/zh-TW/server-managed-settings)、[Settings files](/docs/zh-TW/settings#settings-files)                                                                             |
| [決定要執行什麼](#decide-what-to-enforce)               | 允許哪些工具、命令和整合            | [Permissions](/docs/zh-TW/permissions)、[Sandboxing](/docs/zh-TW/sandboxing)                                                                                                                      |
| [設定使用情況可見性](#set-up-usage-visibility)            | 您如何追蹤支出和採用情況            | [Analytics](/docs/zh-TW/analytics)、[Monitoring](/docs/zh-TW/monitoring-usage)、[Costs](/docs/zh-TW/costs)                                                                                              |
| [檢查資料處理](#review-data-handling)                  | 資料保留和合規狀況               | [Data usage](/docs/zh-TW/data-usage)、[Security](/docs/zh-TW/security)                                                                                                                            |

<h2 id="choose-your-api-provider">
  選擇您的 API 提供者
</h2>

Claude Code 透過多個 API 提供者之一連接到 Claude。您的選擇會影響計費、驗證、您繼承的合規狀況，以及您的開發人員可以使用的 Claude Code 功能。

| 提供者                           | 在以下情況下選擇此選項                                            |
| :---------------------------- | :----------------------------------------------------- |
| Claude for Teams / Enterprise | 您希望 Claude Code 和 claude.ai 在一個按座位訂閱下，無需執行基礎設施。這是預設建議。 |
| Claude Console                | 您是 API 優先或希望按使用量付費計費                                   |
| Amazon Bedrock                | 您希望繼承現有的 AWS 合規控制和計費                                   |
| Google Cloud's Agent Platform | 您希望繼承現有的 GCP 合規控制和計費                                   |
| Microsoft Foundry             | 您希望繼承現有的 Azure 合規控制和計費                                 |

某些 Claude Code 功能需要 claude.ai 帳戶。[Claude Code on the web](/docs/zh-TW/claude-code-on-the-web)、[Routines](/docs/zh-TW/routines)、[Code Review](/docs/zh-TW/code-review)、[Remote Control](/docs/zh-TW/remote-control) 和 [Chrome extension](/docs/zh-TW/chrome) 無法透過 Console API 金鑰或雲端提供者認證單獨使用。如果您透過 Amazon Bedrock、Google Cloud's Agent Platform 或 Microsoft Foundry 部署，請規劃開發人員是否也需要 Claude for Teams 或 Enterprise 座位。每個功能頁面都列出其計畫要求。

有關涵蓋驗證、區域和功能奇偶性的完整提供者比較，請參閱 [企業部署概述](/docs/zh-TW/third-party-integrations)。每個提供者的驗證設定位於 [Authentication](/docs/zh-TW/authentication)。

無論提供者如何，[網路配置](/docs/zh-TW/network-config) 中的代理和防火牆要求都適用。如果您想要在多個提供者前面有單一端點或集中式請求日誌記錄，請參閱 [LLM gateway](/docs/zh-TW/llm-gateway)。

<h2 id="decide-how-settings-reach-devices">
  決定設定如何到達裝置
</h2>

受管設定定義優先於本地開發人員配置的政策。Claude Code 按優先順序檢查以下四個來源，並應用第一個傳回非空配置的來源，但有一個例外：當任何管理員控制的來源設定時，會遵守一小組[跨來源鎖定鍵](/docs/zh-TW/settings#settings-precedence)，例如沙箱允許清單鎖定。

| 機制                      | 傳遞                                                                                                                                                                                               | 優先級 | 平台            |
| :---------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-- | :------------ |
| Server-managed          | claude.ai 管理員控制台，或用於閘道登入的自託管 [Claude apps gateway](/docs/zh-TW/claude-apps-gateway)                                                                                                                   | 最高  | 全部            |
| plist / registry policy | macOS：`com.anthropic.claudecode` plist<br />Windows：`HKLM\SOFTWARE\Policies\ClaudeCode`                                                                                                          | 高   | macOS、Windows |
| File-based managed      | macOS：`/Library/Application Support/ClaudeCode/managed-settings.json`<br />Linux 和 WSL：`/etc/claude-code/managed-settings.json`<br />Windows：`C:\Program Files\ClaudeCode\managed-settings.json` | 中   | 全部            |
| Windows user registry   | `HKCU\SOFTWARE\Policies\ClaudeCode`                                                                                                                                                              | 最低  | 僅 Windows     |

已配置的 [`policyHelper`](/docs/zh-TW/settings#compute-managed-settings-with-a-policy-helper) 會優先於所有四個來源：其輸出成為該執行的唯一受管配置。請參閱[設定優先順序](/docs/zh-TW/settings#settings-precedence)。

Server-managed 設定在驗證時到達裝置，並在活動會話期間每小時刷新一次，無需端點基礎設施。透過 claude.ai 管理員控制台傳遞需要 Claude for Teams 或 Enterprise 計畫。在 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 上的部署可以透過執行 [Claude apps gateway](/docs/zh-TW/claude-apps-gateway) 獲得相同的遠端傳遞，或改用其中一個基於檔案或作業系統級別的機制。

如果您的組織混合使用提供者，請為 claude.ai 使用者配置 [server-managed settings](/docs/zh-TW/server-managed-settings) 加上 [基於檔案或 plist/registry 備用](/docs/zh-TW/settings#settings-files)，以便其他使用者仍然接收受管政策。

plist 和 HKLM 登錄位置適用於任何提供者，並且由於需要管理員權限才能寫入，因此可以抵抗篡改。Windows 使用者登錄中的 HKCU 無需提升即可寫入，因此將其視為便利預設值而不是執行通道。

根據預設，WSL 僅讀取 `/etc/claude-code` 的 Linux 檔案路徑。若要將您的 Windows 登錄和 `C:\Program Files\ClaudeCode` 政策擴展到同一機器上的 WSL，請在這些僅限管理員的 Windows 來源之一中設定 [`wslInheritsWindowsSettings: true`](/docs/zh-TW/settings#available-settings)。

無論您選擇哪種機制，受管值都優先於使用者和專案設定。陣列設定（例如 `permissions.allow` 和 `permissions.deny`）會合併來自所有來源的項目，因此開發人員可以擴展受管清單但無法從中移除。對於[兩個例外](/docs/zh-TW/settings#settings-precedence)，`fallbackModel` 和 `availableModels`，受管值會取代較低層級而不是合併。

請參閱 [Server-managed settings](/docs/zh-TW/server-managed-settings) 和 [Settings files and precedence](/docs/zh-TW/settings#settings-files)。

<h3 id="wsl-sessions-in-claude-code-desktop">
  Claude Code Desktop 中的 WSL 會話
</h3>

在 Windows 上，[Claude Code Desktop 可以在 WSL 2 發行版內執行 Code 會話](/docs/zh-TW/desktop-wsl)。會話的 Claude Code 程序在發行版內執行，因此它透過上述 WSL 探索路徑解析受管設定：除非部署了 `wslInheritsWindowsSettings: true`，否則僅限 Windows 的來源無法到達它。

在存在受管設定的裝置上，Desktop WSL 會話預設不可用。如果您的組織想要啟用它們，請聯絡您的 Anthropic 帳戶團隊。啟用後：

* 透過 HKLM 登錄或 `C:\Program Files\ClaudeCode` 檔案部署 `wslInheritsWindowsSettings: true`，以便 WSL 會話繼承與主機會話相同的政策。
* 透過在 WSL 會話內執行 `/status` 進行驗證：`Setting sources` 行應顯示 `Enterprise managed settings` 以及您部署的 Windows 來源 `(HKLM)` 或 `(file)`。

WSL 2 公用程式 VM 內的程序對 Windows 端端點偵測感應器不可見。如果您使用 CrowdStrike Falcon，請在 WSL 2 上啟用適用於 Linux 的 Falcon 感應器，並使用 CrowdStrike 的 WSL 文件所需的兩個排除項目（適用於 WSL 虛擬機器程序和 VM 磁碟映像），以便可以觀察到發行版內的程序和檔案活動。Claude Code 的 [OpenTelemetry 工具執行遙測](/docs/zh-TW/monitoring-usage)對 WSL 和原生會話的發出方式相同。

<h2 id="decide-what-to-enforce">
  決定要執行什麼
</h2>

受管設定可以鎖定工具、沙箱執行、限制 MCP 伺服器和外掛程式來源，以及控制哪些 hooks 執行。每一行都是一個控制表面，具有驅動它的設定鍵。

| 控制                                                                                        | 它的作用                                                                                                                                              | 關鍵設定                                                                                                  |
| :---------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------- |
| [Permission rules](/docs/zh-TW/permissions)                                                    | 允許、詢問或拒絕特定工具和命令                                                                                                                                   | `permissions.allow`、`permissions.deny`                                                                |
| [Permission lockdown](/docs/zh-TW/permissions#managed-only-settings)                           | 僅受管權限規則適用；禁用 `--dangerously-skip-permissions`                                                                                                     | `allowManagedPermissionRulesOnly`、`permissions.disableBypassPermissionsMode`                          |
| [Sandboxing](/docs/zh-TW/sandboxing)                                                           | 作業系統級別的檔案系統和網路隔離，具有網域允許清單                                                                                                                         | `sandbox.enabled`、`sandbox.network.allowedDomains`                                                    |
| [Managed policy CLAUDE.md](/docs/zh-TW/memory#deploy-organization-wide-claude-md)              | 在每個會話中載入的組織範圍指令，無法排除                                                                                                                              | 受管政策路徑中的檔案                                                                                            |
| [MCP server control](/docs/zh-TW/managed-mcp)                                                  | 限制使用者可以新增或連接的 MCP 伺服器，或部署固定集合                                                                                                                     | `allowedMcpServers`、`deniedMcpServers`、`allowManagedMcpServersOnly`，或已部署的 `managed-mcp.json` 檔案       |
| [Plugin marketplace control](/docs/zh-TW/plugin-marketplaces#managed-marketplace-restrictions) | 限制使用者可以新增和安裝的市場來源、拒絕為單次執行側載外掛程式、agents 和 MCP 伺服器的 CLI 旗標，以及允許清單哪些市場的外掛程式可以被建議                                                                     | `strictKnownMarketplaces`、`blockedMarketplaces`、`disableSideloadFlags`、`pluginSuggestionMarketplaces` |
| [Customization lockdown](/docs/zh-TW/settings#strictpluginonlycustomization)                   | 阻止 skills、agents、hooks 和 MCP 伺服器來自使用者和專案來源，使其只能來自外掛程式或受管設定                                                                                        | `strictPluginOnlyCustomization`                                                                       |
| [Hook restrictions](/docs/zh-TW/settings#hook-configuration)                                   | 僅受管 hooks 載入；限制 HTTP hook URL                                                                                                                     | `allowManagedHooksOnly`、`allowedHttpHookUrls`                                                         |
| [Login enforcement](/docs/zh-TW/settings#available-settings)                                   | 限制互動式登入為特定方法或 Anthropic 組織。設定時，由 `ANTHROPIC_API_KEY`、`ANTHROPIC_AUTH_TOKEN` 或 `apiKeyHelper` 驗證的會話在啟動時被阻止；雲端提供者會話不受影響                             | `forceLoginMethod`、`forceLoginOrgUUID`                                                                |
| [Disable agent view](/docs/zh-TW/agent-view#how-background-sessions-are-hosted)                | 關閉 `claude agents`、`--bg`、`/background` 和隨選監督員                                                                                                    | `disableAgentView`                                                                                    |
| [Model restrictions](/docs/zh-TW/model-config#restrict-model-selection)                        | `availableModels` 篩選模型選擇器中出現的模型。新增 `enforceAvailableModels` 也會限制自動選擇的預設模型。請參閱[表面涵蓋範圍](/docs/zh-TW/model-config#surface-coverage)以了解此設定如何到達 CLI、網頁和 IDE | `availableModels`、`enforceAvailableModels`                                                            |
| [Version floor](/docs/zh-TW/settings)                                                          | 防止自動更新安裝低於組織範圍最小值的版本                                                                                                                              | `minimumVersion`                                                                                      |
| [Required version range](/docs/zh-TW/settings)                                                 | 當執行版本超出組織核准範圍時，完全拒絕啟動。比 `minimumVersion` 更強大，後者只會阻止降級                                                                                             | `requiredMinimumVersion`、`requiredMaximumVersion`                                                     |

透過 claude.ai 或 Anthropic API 進行身份驗證的組織成員也可以在不部署設定的情況下管理模型：[組織模型限制](/docs/zh-TW/model-config#organization-model-restrictions)停用個別模型、[組織預設模型](/docs/zh-TW/model-config#organization-default-model)設定新會話啟動時使用的模型，以及[組織工作量限制](/docs/zh-TW/model-config#organization-effort-limits)限制每個角色的工作量級別。所有三個控制都需要 Claude Enterprise 方案。模型限制和工作量限制在伺服器端執行；預設模型是一個起點，使用者可以變更，除非組織強制執行。強制執行適用於有限的組織集合；請向您的 Anthropic 帳戶團隊詢問可用性。這些控制都不會到達 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 或 [Claude Platform on AWS](/docs/zh-TW/claude-platform-on-aws) 上的會話；在這些提供者上，使用上面的 `availableModels` 進行限制，以及受管設定中的 `model` 鍵作為預設值。

[Claude Code on the web](/docs/zh-TW/claude-code-on-the-web) 有其自己的管理表面：在管理設定中的 Cloud environments 頁面上，擁有者和管理員建立[組織共享環境](/docs/zh-TW/claude-code-on-the-web#organization-shared-environments)，設定成員雲端會話的[網路存取級別](/docs/zh-TW/claude-code-on-the-web#network-access)、環境變數和設定指令碼，並選擇組織的預設環境。

權限規則和沙箱涵蓋不同的層。拒絕 WebFetch 會阻止 Claude 的 fetch 工具，但如果允許 Bash，`curl` 和 `wget` 仍然可以到達任何 URL。沙箱透過在作業系統級別執行的網路網域允許清單來彌補這一差距。

有關這些控制防禦的威脅模型，請參閱 [Security](/docs/zh-TW/security)。

<h2 id="set-up-usage-visibility">
  設定使用情況可見性
</h2>

根據您需要報告的內容選擇監控。儀表板、API 和支出控制在 Claude for Teams 或 Enterprise 計畫與 Claude Console 組織之間有所不同，因此在根據功能規劃報告之前，請檢查「可用性」欄。

| 功能                     | 您獲得什麼                                                      | 可用性                                                                                                                                                                                                                     | 從哪裡開始                                                    |
| :--------------------- | :--------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------- |
| Usage monitoring       | 會話、工具和令牌的 OpenTelemetry 匯出                                 | 所有提供者                                                                                                                                                                                                                   | [Monitoring usage](/docs/zh-TW/monitoring-usage)              |
| Analytics dashboard    | Teams / Enterprise 上具有排行榜的採用和貢獻指標；Console 上的每個使用者使用情況和支出指標 | Teams / Enterprise 在 [claude.ai/analytics](https://claude.ai/analytics/claude-code)，Console 在 [platform.claude.com/claude-code](https://platform.claude.com/claude-code)                                                | [Analytics](/docs/zh-TW/analytics)                            |
| Programmatic reporting | 透過 API 的每個使用者使用情況和成本資料                                     | Enterprise 的 [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics)，Console 的 [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api) | [Costs](/docs/zh-TW/costs#manage-costs-for-your-organization) |
| Spend controls         | 支出限制和速率限制                                                  | Teams / Enterprise 的管理員設定、Console 的工作區限制；在第三方雲端上，雲端預算控制或具有每個使用者[支出限制](/docs/zh-TW/claude-apps-gateway-spend-limits)的 [Claude 應用程式閘道](/docs/zh-TW/claude-apps-gateway)                                                             | [Costs](/docs/zh-TW/costs#manage-costs-for-your-organization) |

在 Teams 和 Enterprise 上，每個使用者的使用情況和支出數字來自您組織分析設定中的[支出報告](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans)，而不是分析儀表板。雲端提供者透過 AWS Cost Explorer、GCP Billing 或 Azure Cost Management 公開支出。如需規劃跨 Claude chat、Claude Code 和 Cowork 的企業預算，請參閱 [Claude Enterprise 消費指南](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide)。

<h2 id="review-data-handling">
  檢查資料處理
</h2>

在 Team、Enterprise、Claude API 和雲端提供者計畫上，Anthropic 不會在您的程式碼或提示上訓練模型。您的 API 提供者決定保留和合規狀況。

| 主題                        | 需要了解的內容                                  | 從哪裡開始                                             |
| :------------------------ | :--------------------------------------- | :------------------------------------------------ |
| Data usage policy         | Anthropic 收集什麼、保留多長時間、永遠不會用於訓練的內容        | [Data usage](/docs/zh-TW/data-usage)                   |
| Zero Data Retention (ZDR) | 請求完成後不存儲任何內容。在 Claude for Enterprise 上可用 | [Zero data retention](/docs/zh-TW/zero-data-retention) |
| Security architecture     | 網路模型、加密、驗證、稽核追蹤                          | [Security](/docs/zh-TW/security)                       |

如果您需要請求級別的稽核日誌記錄或按資料敏感性路由流量，請在開發人員和您的提供者之間放置自託管的 [Claude apps gateway](/docs/zh-TW/claude-apps-gateway)，它會記錄具有 IdP 身分的每個請求稽核日誌，或使用另一個 [LLM gateway](/docs/zh-TW/llm-gateway)。有關法規要求和認證，請參閱 [Legal and compliance](/docs/zh-TW/legal-and-compliance)。

<h2 id="verify-and-onboard">
  驗證和上線
</h2>

配置受管設定後，讓開發人員在 Claude Code 內執行 `/status`。在 **Status** 標籤上，`Setting sources` 行顯示 `Enterprise managed settings` 後面跟著括號中的來源，其中之一為 `(remote)`、`(plist)`、`(HKLM)`、`(HKCU)` 或 `(file)`。請參閱 [驗證作用中的設定](/docs/zh-TW/settings#verify-active-settings)。

分享這些資源以幫助開發人員入門：

* [快速入門](/docs/zh-TW/quickstart)：從安裝到使用專案的首次會話逐步說明
* [常見工作流程](/docs/zh-TW/common-workflows)：日常任務的模式，例如程式碼審查、重構和除錯
* [Claude 101](https://anthropic.skilljar.com/claude-101) 和 [Claude Code in Action](https://anthropic.skilljar.com/claude-code-in-action)：自進度 Anthropic Academy 課程

對於登入問題，請將開發人員指向 [驗證疑難排解](/docs/zh-TW/troubleshoot-install#login-and-authentication)。最常見的修復是：

* 執行 `/logout` 然後 `/login` 以切換帳戶
* 如果缺少企業驗證選項，執行 `claude update`
* 更新後重新啟動終端

如果開發人員看到「您尚未被新增到您的組織」，他們的座位不包括 Claude Code 存取權限，需要在管理員控制台中更新。

<h2 id="next-steps">
  後續步驟
</h2>

選擇提供者和傳遞機制後，繼續進行詳細配置：

* [Server-managed settings](/docs/zh-TW/server-managed-settings)：從 Claude 管理員控制台傳遞受管政策
* [Settings reference](/docs/zh-TW/settings)：每個設定鍵、檔案位置和優先級規則
* [Monorepos and large repos](/docs/zh-TW/large-codebases)：為部署到 monorepo 的組織提供的每個目錄配置模式
* [Amazon Bedrock](/docs/zh-TW/amazon-bedrock)、[Google Cloud's Agent Platform](/docs/zh-TW/google-vertex-ai)、[Microsoft Foundry](/docs/zh-TW/microsoft-foundry)：提供者特定部署
* [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide)：SSO、SCIM、座位管理和推出劇本
