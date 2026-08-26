> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code 與 GitHub Enterprise Server

> 將 Claude Code 連接到您自託管的 GitHub Enterprise Server 實例，以進行網頁會話、代碼審查和插件市場。

<Note>
  GitHub Enterprise Server 支持適用於 Team 和 Enterprise 計劃。
</Note>

GitHub Enterprise Server (GHES) 支持讓您的組織使用 Claude Code 與託管在自管理 GitHub 實例上的存儲庫，而不是 github.com。一旦 Owner 連接您的 GHES 實例，開發人員可以運行網頁會話和獲得自動化代碼審查，無需任何按存儲庫的配置。您實例上託管的插件市場也受支持；憑證要求因表面而異，如 [GHES 上的插件市場](#plugin-marketplaces-on-ghes) 中所述。

對於 github.com 上的存儲庫，請參閱 [Claude Code on the web](/docs/zh-TW/claude-code-on-the-web) 和 [Code Review](/docs/zh-TW/code-review)。要在您自己的 CI 基礎設施中運行 Claude，請參閱 [GitHub Actions](/docs/zh-TW/github-actions)。

<h2 id="what-works-with-github-enterprise-server">
  GitHub Enterprise Server 支持的功能
</h2>

下表顯示了 Claude Code 的哪些功能支持 GHES，以及與 github.com 行為的任何差異。

| 功能                     | GHES 支持 | 備註                                                                                      |
| :--------------------- | :------ | :-------------------------------------------------------------------------------------- |
| Claude Code on the web | ✅ 支持    | 管理員連接 GHES 實例一次；開發人員像往常一樣使用 `claude --cloud` 或 [claude.ai/code](https://claude.ai/code) |
| Code Review            | ✅ 支持    | 與 github.com 相同的自動化 PR 審查                                                               |
| Claude Security        | ✅ 支持    | 在 Enterprise 計劃的公開測試版中提供，位於 [claude.ai/security](https://claude.ai/security)            |
| Teleport sessions      | ✅ 支持    | 使用 `--teleport` 在網頁和終端之間移動會話                                                            |
| Plugin marketplaces    | ✅ 支持    | 認證要求因介面而異。請參閱 [GHES 上的 Plugin marketplaces](#plugin-marketplaces-on-ghes)               |
| Contribution metrics   | ✅ 支持    | 通過 webhooks 傳遞到 [analytics dashboard](/docs/zh-TW/analytics)                                 |
| GitHub Actions         | ✅ 支持    | 需要手動工作流設置；`/install-github-app` 僅適用於 github.com                                         |
| GitHub MCP server      | ❌ 不支持   | GitHub MCP server 不適用於 GHES 實例                                                          |

<h2 id="admin-setup">
  管理員設置
</h2>

一位擁有者將您的 GHES 實例連接到 Claude Code 一次。之後，您組織中的開發人員可以使用 GHES 存儲庫，無需任何額外配置。您需要在 Claude 組織中具有擁有者或主要擁有者角色，以及在 GHES 實例上創建 GitHub Apps 的權限。

引導式設置生成 GitHub App 清單，並將您重定向到 GHES 實例以一鍵創建應用。如果您的環境阻止重定向流，可以使用 [替代手動設置](#manual-setup)。

<Steps>
  <Step title="打開 Claude Code 管理員設置">
    轉到 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) 並找到 GitHub Enterprise Server 部分。
  </Step>

  <Step title="開始引導式設置">
    點擊 **Connect**。輸入連接的顯示名稱和您的 GHES 主機名，例如 `github.example.com`。如果您的 GHES 實例使用自簽名或私有證書頒發機構，請將 CA 證書粘貼到可選字段中。
  </Step>

  <Step title="創建 GitHub App">
    點擊 **Continue to GitHub Enterprise**。您的瀏覽器重定向到您的 GHES 實例，並預填充應用清單。檢查配置並點擊 **Create GitHub App**。GHES 將您重定向回 Claude，應用憑據自動存儲。
  </Step>

  <Step title="在您的存儲庫上安裝應用">
    從 GHES 實例上的 GitHub App 頁面，在您希望 Claude 訪問的存儲庫或組織上安裝應用。您可以從一個子集開始，稍後添加更多。
  </Step>

  <Step title="啟用功能">
    返回 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) 並為您的 GHES 存儲庫啟用 [Code Review](/docs/zh-TW/code-review#set-up-code-review)、Claude Security 和 [contribution metrics](/docs/zh-TW/analytics#enable-contribution-metrics)，使用與 github.com 相同的配置。
  </Step>
</Steps>

<h3 id="github-app-permissions">
  GitHub App 權限
</h3>

清單使用 Claude 在網頁會話、Code Review、Claude Security 和 contribution metrics 中需要的權限和 webhook 事件配置 GitHub App：

| 權限               | 訪問 | 用途                                 |
| :--------------- | :- | :--------------------------------- |
| Contents         | 讀寫 | 克隆存儲庫和推送分支                         |
| Pull requests    | 讀寫 | 創建 PR 和發佈審查評論                      |
| Issues           | 讀寫 | 響應問題提及                             |
| Checks           | 讀寫 | 發佈 Code Review 檢查運行                |
| Actions          | 讀  | 讀取 CI 狀態以進行自動修復                    |
| Repository hooks | 讀寫 | 接收 contribution metrics 的 webhooks |
| Metadata         | 讀  | GitHub 對所有應用都需要                    |

應用訂閱 `pull_request`、`issue_comment`、`pull_request_review_comment`、`pull_request_review` 和 `check_run` 事件。

<h3 id="manual-setup">
  手動設置
</h3>

如果引導式重定向流被您的網絡配置阻止，請點擊 **Add manually** 而不是 Connect。在您的 GHES 實例上使用 [上述權限和事件](#github-app-permissions) 創建 GitHub App，然後在表單中輸入應用憑據：主機名、OAuth 客戶端 ID 和密鑰、GitHub App ID、客戶端 ID、客戶端密鑰、webhook 密鑰和私鑰。

<h3 id="network-requirements">
  網絡要求
</h3>

您的 GHES 實例必須可從 Anthropic 基礎設施訪問，以便 Claude 可以克隆存儲庫並發佈審查評論。如果您的 GHES 實例在防火牆後面，請將 [Anthropic API IP 地址](https://platform.claude.com/docs/en/api/ip-addresses) 列入白名單。

<h2 id="developer-workflow">
  開發人員工作流
</h2>

一旦您的管理員連接了 GHES 實例，就不需要開發人員端的配置。Claude Code 從您工作目錄中的 git 遠程自動檢測您的 GHES 主機名。

像往常一樣從您的 GHES 實例克隆存儲庫：

```bash theme={null}
git clone git@github.example.com:platform/api-service.git
cd api-service
```

然後開始網頁會話。Claude 從您的 git 遠程檢測 GHES 主機，並通過您組織的配置實例路由會話：

```bash theme={null}
claude --cloud "Add retry logic to the payment webhook handler"
```

會話在 Anthropic 基礎設施上運行，從 GHES 克隆您的存儲庫，並將更改推送回分支。使用 `/tasks` 或在 [claude.ai/code](https://claude.ai/code) 監控進度。有關完整的遠程會話工作流（包括 diff 審查、自動修復和例程），請參閱 [Claude Code on the web](/docs/zh-TW/claude-code-on-the-web)。

<h3 id="teleport-sessions-to-your-terminal">
  Teleport 會話到您的終端
</h3>

使用 `claude --teleport` 將網頁會話拉入您的本地終端。Teleport 在獲取分支和加載會話歷史之前驗證您在同一 GHES 存儲庫的簽出中。有關詳細信息，請參閱 [teleport 要求](/docs/zh-TW/claude-code-on-the-web#teleport-requirements)。

<h2 id="plugin-marketplaces-on-ghes">
  GHES 上的插件市場
</h2>

在您的 GHES 實例上託管插件市場，以在您的組織中分發內部工具。市場結構與 github.com 託管的市場相同，但安裝方式取決於您在何處新增市場，且認證在不同介面上有所不同：

| 介面                              | 安裝方式                                                                            | 每個使用者需要什麼                                                                         |
| :------------------------------ | :------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------- |
| Claude Code CLI 和桌面應用           | Claude Code 使用機器現有的 git 認證複製市場存儲庫                                               | 從其機器對您的 GHES 主機的 Git 存取權                                                          |
| 託管設定 (`extraKnownMarketplaces`) | Claude Code 註冊該項目並使用機器現有的 git 認證複製存儲庫                                           | 從其機器對您的 GHES 主機的 Git 存取權                                                          |
| claude.ai 組織插件設定                | 擁有者選擇 GHES 實例作為來源；Anthropic 的後端使用來自 [管理員設定](#admin-setup) 的 GitHub App 擷取並同步存儲庫 | 新增後每個使用者無需任何操作。新增它的擁有者需要連接自己的 GitHub Enterprise 帳戶作為存取檢查，且 GitHub App 必須安裝在市場存儲庫上 |
| claude.ai 使用者設定                 | Anthropic 的後端使用提交使用者的 GitHub Enterprise 連接擷取存儲庫                                 | 連接到 Claude 的自己的 GitHub Enterprise 帳戶                                              |
| Claude Code 網頁版                 | 雲端工作階段在工作階段沙箱內複製市場。沙箱只有在工作階段的存儲庫位於同一實例上時，才能到達您的 GHES 實例，且其 git 認證的範圍限於工作階段的存儲庫  | 對於 GHES 託管的市場不可靠：與工作階段存儲庫不同的主機無法到達，即使是同一實例的安裝也可能失敗。請改用 CLI、託管設定或 claude.ai        |

<Warning>
  當從使用者設定新增市場時，claude.ai 上的 GitHub Enterprise 連接是按使用者的。[管理員設定](#admin-setup) 將您的 GHES 實例連接到您的組織，但它不連接個別使用者帳戶：每個從自己的設定新增 GHES 市場的使用者必須先連接自己的 GitHub Enterprise 帳戶，且一個使用者的連接（包括擁有者的）不涵蓋任何其他人。由擁有者在組織插件設定中新增的市場不會對使用者施加此要求，因為持續的擷取使用組織的 GitHub App。新增市場的擁有者仍然需要在新增時連接自己的 GitHub Enterprise 帳戶。
</Warning>

<h3 id="add-a-ghes-marketplace">
  添加 GHES 市場
</h3>

`owner/repo` 簡寫始終解析為 github.com。對於 GHES 託管的市場，請使用完整的 git URL。建議使用 HTTPS URL：

```bash theme={null}
/plugin marketplace add https://github.example.com/platform/claude-plugins.git
```

如果機器已經信任您的 GHES 主機，SSH URL 也可以工作：

```bash theme={null}
/plugin marketplace add git@github.example.com:platform/claude-plugins.git
```

Claude Code 以非互動方式執行 git，並拒絕連接到不在機器 `known_hosts` 檔案中的主機的 SSH 連接。帶有 git 認證幫助程式的 HTTPS URL 可以避免 `known_hosts` 要求。

有關構建市場的完整指南，請參閱 [Create and distribute a plugin marketplace](/docs/zh-TW/plugin-marketplaces)。

<h3 id="pre-register-ghes-marketplaces-with-managed-settings">
  使用託管設定預先註冊 GHES 市場
</h3>

`extraKnownMarketplaces` 設定預先註冊市場，以便開發人員無需手動設定即可獲得它。它可以從 [任何設定檔案](/docs/zh-TW/settings#extraknownmarketplaces) 工作，包括存儲庫的 `.claude/settings.json`；託管設定可以組織範圍內傳遞它：

```json theme={null}
{
  "extraKnownMarketplaces": {
    "internal-tools": {
      "source": {
        "source": "git",
        "url": "https://github.example.com/platform/claude-plugins.git"
      }
    }
  }
}
```

Claude Code 在本地安裝這些市場：它註冊每個項目並使用機器現有的 git 認證複製存儲庫。此路徑不經過 claude.ai，因此不需要按使用者的 GitHub Enterprise 連接。為了成功推出：

* **使用完整的 git URL。** `owner/repo` 簡寫始終解析為 github.com，無法參考 GHES 主機。
* **偏好 HTTPS URL。** SSH 複製在不已信任您的 GHES 主機金鑰的機器上失敗。帶有您組織標準 git 認證幫助程式的 HTTPS URL 可在任何配置了認證的機器上工作。
* **確認每台機器都可以從您的 GHES 主機複製。** 如果機器缺少認證，市場已註冊但永遠不會安裝，其插件報告為未找到而不是提示輸入認證。
* **確認設定到達每台機器。** 託管設定檔案只對部署到的機器生效，例如透過您的裝置管理系統。有關檔案位置，請參閱 [託管設定](/docs/zh-TW/settings#settings-files)。

<h3 id="allowlist-ghes-marketplaces-in-managed-settings">
  在託管設定中將 GHES 市場列入白名單
</h3>

如果您的組織使用 [託管設定](/docs/zh-TW/settings) 來限制開發人員可以添加的市場，請使用 `hostPattern` 源類型來允許來自您的 GHES 實例的所有市場，而無需列舉每個存儲庫：

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

有關完整的架構，請參閱 [strictKnownMarketplaces](/docs/zh-TW/settings#strictknownmarketplaces) 和 [extraKnownMarketplaces](/docs/zh-TW/settings#extraknownmarketplaces) 設定參考。

<h2 id="limitations">
  限制
</h2>

一些功能在 GHES 上的行為與 github.com 上不同。[功能表](#what-works-with-github-enterprise-server) 總結了支持；本部分涵蓋了解決方案。

* **`/install-github-app` 命令**：改為遵循 claude.ai 上的 [管理員設置](#admin-setup) 流程。如果您還想在 GHES 上使用 GitHub Actions 工作流，請手動調整 [示例工作流](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml)。
* **GitHub MCP server**：改為使用為您的 GHES 主機配置的 `gh` CLI。運行 `gh auth login --hostname github.example.com` 進行身份驗證，然後 Claude 可以在會話中使用 `gh` 命令。

<h2 id="troubleshooting">
  故障排除
</h2>

<h3 id="web-session-fails-to-clone-repository">
  網頁會話無法克隆存儲庫
</h3>

如果 `claude --cloud` 因克隆錯誤而失敗，請驗證 Owner 已完成您的 GHES 實例的設置，並且 GitHub App 已安裝在您正在使用的存儲庫上。與連接該實例的 Owner 確認在 Claude 設置中註冊的主機名與您的 git 遠端中的主機名匹配。

<h3 id="marketplace-add-fails-with-a-policy-error">
  市場添加因策略錯誤而失敗
</h3>

如果 `/plugin marketplace add` 因您的 GHES URL 而被阻止，您的組織已限制市場源。要求您的管理員在 [託管設置](#allowlist-ghes-marketplaces-in-managed-settings) 中為您的 GHES 主機名添加 `hostPattern` 條目。

<h3 id="marketplace-add-on-claude-ai-fails-with-a-github-access-error">
  claude.ai 上的市場添加因 GitHub 存取錯誤而失敗
</h3>

如果從您的使用者設置添加 GHES 市場失敗並出現通用錯誤（例如「無法添加市場」），請先檢查您的 GitHub Enterprise 連接。這是當您自己的 GitHub Enterprise 帳戶未連接到 Claude 時出現的情況，即使您的組織的 GHES 實例已配置且其他使用者已連接。該對話框不會指向 GitHub Enterprise 連接流程，而「瀏覽」標籤上的「連接到 GitHub」選項會登入 github.com，這不會授予對 GHES 存儲庫的存取權限。

要連接您的 GitHub Enterprise 帳戶：[claude.ai/code](https://claude.ai/code) 上的存儲庫選擇器為每個已配置的 GHES 實例提供連接選項，Owner 也可以從 [Claude Code 管理員設置](https://claude.ai/admin-settings/claude-code) 的 GitHub Enterprise 部分進行連接。然後再次添加市場。或者，要求 Owner 在組織外掛程式設置中添加市場，這樣可以消除每個使用者的連接要求。

在其他 claude.ai 表面上，GHES 市場上的「找不到存儲庫。如果是私有的，需要 GitHub 存取」錯誤通常表示相同的缺失連接。通過上述路徑之一連接您的 GitHub Enterprise 帳戶，然後重試。

<h3 id="ghes-instance-not-reachable">
  GHES 實例無法訪問
</h3>

如果審查或網頁會話超時，您的 GHES 實例可能無法從 Anthropic 基礎設施訪問。確認您的防火牆允許來自 [Anthropic API IP 地址](https://platform.claude.com/docs/zh-TW/api/ip-addresses) 的入站連接。

<h2 id="related-resources">
  相關資源
</h2>

這些頁面更深入地涵蓋了本指南中引用的功能：

* [Claude Code on the web](/docs/zh-TW/claude-code-on-the-web)：在雲基礎設施上運行 Claude Code 會話
* [Code Review](/docs/zh-TW/code-review)：自動化 PR 審查
* [Plugin marketplaces](/docs/zh-TW/plugin-marketplaces)：構建和分發插件目錄
* [Analytics](/docs/zh-TW/analytics)：跟踪使用情況和貢獻指標
* [Managed settings](/docs/zh-TW/settings)：組織範圍的策略配置
* [Network configuration](/docs/zh-TW/network-config)：防火牆和 IP 白名單要求
