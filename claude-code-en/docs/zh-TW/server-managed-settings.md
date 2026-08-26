> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 設定伺服器管理的設定

> 透過伺服器傳遞的設定在 Claude.ai 上為您的組織集中設定 Claude Code，無需裝置管理基礎設施。

伺服器管理的設定允許組織擁有者透過 claude.ai 主控台中的 [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code) 集中設定 Claude Code。Claude Code 用戶端在使用者使用其組織 OAuth 登入或直接設定的 API 金鑰進行身份驗證時會自動接收這些設定，在支援伺服器管理傳遞的平台上。請參閱[平台可用性](#platform-availability)。

此方法適用於沒有裝置管理基礎設施的組織，或需要為非受管裝置上的使用者管理設定的組織。

<Note>
  伺服器管理的設定適用於 [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=server_settings_teams#team-&-enterprise) 和 [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=server_settings_enterprise) 客戶。
</Note>

<h2 id="requirements">
  需求
</h2>

若要使用伺服器管理的設定，您需要：

* Claude for Teams 或 Claude for Enterprise 方案
* 您的 Claude 組織中的擁有者或主要擁有者角色，以檢視和編輯配置
* 對 `api.anthropic.com` 的網路存取

<h2 id="choose-between-server-managed-and-endpoint-managed-settings">
  在伺服器管理和端點管理的設定之間選擇
</h2>

Claude Code 支援兩種集中設定方法。伺服器管理的設定從 Anthropic 的伺服器傳遞設定。[端點管理的設定](/docs/zh-TW/settings#settings-files) 透過原生作業系統原則 (macOS 受管偏好設定、Windows 登錄) 或受管設定檔直接部署到裝置。

| 方法                                            | 最適合                    | 安全模型                          |
| :-------------------------------------------- | :--------------------- | :---------------------------- |
| **伺服器管理的設定**                                  | 沒有 MDM 的組織，或非受管裝置上的使用者 | 在身份驗證時從 Anthropic 伺服器傳遞的設定    |
| **[端點管理的設定](/docs/zh-TW/settings#settings-files)** | 具有 MDM 或端點管理的組織        | 透過 MDM 設定檔、登錄原則或受管設定檔部署到裝置的設定 |

如果您的裝置已在 MDM 或端點管理解決方案中註冊，端點管理的設定提供更強的安全保證，因為設定檔可以在作業系統層級受到保護，防止使用者修改。端點管理的設定不會到達 [雲端工作階段](/docs/zh-TW/model-config#surface-coverage)，因此在網路上使用 Claude Code 的組織也應該設定伺服器管理的設定。

<h2 id="configure-server-managed-settings">
  設定伺服器管理的設定
</h2>

<Steps>
  <Step title="開啟管理員主控台">
    在 claude.ai 主控台中，前往 [**Admin Settings > Claude Code > Managed settings**](https://claude.ai/admin-settings/claude-code)。

    如果連結將您重新導向至不同的 Admin Settings 頁面，而不是 Claude Code 頁面，表示您的帳戶沒有所需的角色。管理員和其他非擁有者角色無法檢視或編輯受管設定，因此請要求您組織中的擁有者或主要擁有者進行變更。請參閱[存取控制](#access-control)。
  </Step>

  <Step title="定義您的設定">
    將您的設定新增為 JSON。支援 [`settings.json` 中提供的所有設定](/docs/zh-TW/settings#available-settings)，除了限制於作業系統層級原則傳遞的設定外；請參閱[目前的限制](#current-limitations)以取得該簡短清單。這包括 [hooks](/docs/zh-TW/hooks)、[環境變數](/docs/zh-TW/env-vars) 和[僅限受管的設定](/docs/zh-TW/permissions#managed-only-settings)，例如 `allowManagedPermissionRulesOnly`。

    此範例強制執行權限拒絕清單，防止使用者繞過權限，並將權限規則限制為在受管設定中定義的規則：

    ```json theme={null}
    {
      "permissions": {
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true
    }
    ```

    Hooks 使用與 `settings.json` 中相同的格式。

    此範例在整個組織中的每次檔案編輯後執行稽核指令碼：

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Edit|Write",
            "hooks": [
              { "type": "command", "command": "/usr/local/bin/audit-edit.sh" }
            ]
          }
        ]
      }
    }
    ```

    若要設定 [auto mode](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode) 分類器，使其知道您的組織信任哪些儲存庫、儲存桶和網域：

    ```json theme={null}
    {
      "autoMode": {
        "environment": [
          "Source control: github.example.com/acme-corp and all repos under it",
          "Trusted cloud buckets: s3://acme-build-artifacts, gs://acme-ml-datasets",
          "Trusted internal domains: *.corp.example.com"
        ]
      }
    }
    ```

    因為 hooks 執行 shell 命令，使用者在套用前會看到[安全核准對話方塊](#security-approval-dialogs)。請參閱[設定 auto mode](/docs/zh-TW/auto-mode-config)，了解 `autoMode` 項目如何影響分類器阻止的內容，以及關於 `environment`、`allow`、`soft_deny` 和 `hard_deny` 欄位的重要警告。
  </Step>

  <Step title="儲存並部署">
    儲存您的變更。Claude Code 用戶端在下次啟動或每小時輪詢週期時會接收更新的設定。
  </Step>
</Steps>

<h3 id="verify-settings-delivery">
  驗證設定傳遞
</h3>

若要確認設定正在套用，請要求使用者重新啟動 Claude Code。如果設定包含觸發[安全核准對話方塊](#security-approval-dialogs)的設定，使用者會在啟動時看到描述受管設定的提示。您也可以透過讓使用者執行 `/permissions` 來檢視其有效的權限規則，以驗證受管權限規則是否處於作用中。

<h3 id="access-control">
  存取控制
</h3>

以下角色可以管理伺服器管理的設定：

* **主要擁有者**
* **擁有者**

限制對受信任人員的存取，因為設定變更會套用到組織中的所有使用者。

<h3 id="managed-only-settings">
  僅限受管的設定
</h3>

大多數[設定金鑰](/docs/zh-TW/settings#available-settings)可在任何範圍中運作。少數金鑰只能從受管設定中讀取，在放置於使用者或專案設定檔中時無效。請參閱[僅限受管的設定](/docs/zh-TW/permissions#managed-only-settings)以取得完整清單。任何不在該清單上的設定仍然可以放置在受管設定中，並具有最高優先順序。

<h3 id="current-limitations">
  目前的限制
</h3>

伺服器管理的設定有以下限制：

* 設定統一套用到組織中的所有使用者。尚不支援每個群組的設定。
* [`managed-mcp.json`](/docs/zh-TW/managed-mcp) 檔案無法透過伺服器管理的設定分發。改為在該處傳遞 `allowedMcpServers` 和 `deniedMcpServers` 原則金鑰。
* 限制於作業系統層級原則來源的設定，例如 `policyHelper` 和 `wslInheritsWindowsSettings`，不會被接受。改為透過 MDM 或系統 `managed-settings.json` 檔案部署它們。

<h2 id="settings-delivery">
  設定傳遞
</h2>

<h3 id="settings-precedence">
  設定優先順序
</h3>

伺服器管理的設定和[端點管理的設定](/docs/zh-TW/settings#settings-files)都佔據 Claude Code [設定階層](/docs/zh-TW/settings#settings-precedence)中的最高層級。沒有其他設定層級可以覆蓋它們，包括命令列引數。

在受管層級內，已設定的 [`policyHelper`](/docs/zh-TW/settings#compute-managed-settings-with-a-policy-helper) 會優先於其他所有受管來源，包括伺服器管理的設定：其輸出成為該執行的唯一受管設定。

否則，Claude Code 會使用第一個傳遞非空設定的來源。伺服器管理的設定會先檢查，然後是端點管理的設定。來源不會合併：如果伺服器管理的設定傳遞任何金鑰，其他端點管理的設定會被完全忽略。如果伺服器管理的設定不傳遞任何內容，端點管理的設定會套用。

有一個例外適用：當任何管理員控制的受管來源設定時，會遵守一小組[跨來源鎖定金鑰](/docs/zh-TW/settings#settings-precedence)（例如沙箱允許清單鎖定）；使用者可寫入的 HKCU 登錄層級被排除。

如果您在管理員主控台中清除伺服器管理的設定，意圖回退到端點管理的 plist 或登錄原則，請注意[快取的設定](#fetch-and-caching-behavior)會在用戶端機器上持續存在，直到下次成功擷取。執行 `/status` 以查看哪個受管來源處於作用中。

<h3 id="fetch-and-caching-behavior">
  擷取和快取行為
</h3>

Claude Code 在啟動時從 Anthropic 的伺服器擷取設定，並在作用中的工作階段期間每小時輪詢一次更新。

**首次啟動而無快取設定：**

* Claude Code 非同步擷取設定
* 如果擷取失敗，Claude Code 會在沒有受管設定的情況下繼續
* 在設定載入之前有一個簡短的視窗，其中限制尚未強制執行

**後續啟動並有快取設定：**

* 快取設定在啟動時立即套用，除了下面描述的傳輸、路由和驗證環境變數外
* Claude Code 在背景擷取新鮮設定
* 快取設定透過網路故障持續存在。被保留的環境變數會保持被保留，直到擷取成功

自 v2.1.198 起，Claude Code 會在快取的 `env` 區塊中保留三個環境變數類別，直到伺服器確認該工作階段的承載。這可防止快取的 Proxy、憑證授權單位、端點或認證值重新導向、攔截或重新驗證確認承載的設定擷取。強化只適用於伺服器擷取的設定快取：透過 MDM 或 `managed-settings.json` 部署的[端點管理的設定](/docs/zh-TW/settings#settings-files)不受影響。被保留的類別為：

* Proxy 和 TLS 設定，例如 `HTTPS_PROXY`、`NODE_EXTRA_CA_CERTS` 和 mTLS 用戶端憑證變數 `CLAUDE_CODE_CLIENT_CERT` 和 `CLAUDE_CODE_CLIENT_KEY`
* API 路由和提供者選擇，包括 `ANTHROPIC_BASE_URL`、提供者選擇變數（例如 `CLAUDE_CODE_USE_BEDROCK` 和 `CLAUDE_CODE_USE_VERTEX`）以及提供者端點 URL（例如 `ANTHROPIC_BEDROCK_BASE_URL`）
* 驗證認證，例如 `ANTHROPIC_API_KEY`、`ANTHROPIC_AUTH_TOKEN` 和 `CLAUDE_CODE_OAUTH_TOKEN`

快取 `env` 區塊中的所有其他金鑰（例如遙測和 OpenTelemetry 設定）會如之前一樣在啟動時套用。擷取成功後，被保留的變數會在工作階段的其餘時間套用。

如果您的組織需要 Proxy 才能到達 `api.anthropic.com`，請在殼層環境或[使用者設定](/docs/zh-TW/settings#settings-files)中設定它，而不是只在受管 `env` 區塊中設定。首次啟動沒有快取，因此這些來源已經是初始擷取的必要條件。

Claude Code 自動套用設定更新而無需重新啟動，除了進階設定（例如 OpenTelemetry 設定）需要完整重新啟動才能生效。

<h3 id="invalid-entries-in-delivered-settings">
  傳遞設定中的無效項目
</h3>

傳遞的承載會以與其他受管來源相同的規則寬容地解析。當承載包含無法通過結構描述驗證的項目時，Claude Code 會移除該項目、顯示驗證錯誤，並套用每個剩餘的有效設定。請參閱[受管設定中的無效項目](/docs/zh-TW/settings#invalid-entries-in-managed-settings)以了解欄位層級的行為，包括如何處理安全強制欄位。需要 Claude Code v2.1.169 或更新版本。

伺服器管理的傳遞新增這些行為：

* `~/.claude/remote-settings.json` 中的快取會儲存已移除無效項目的已修復承載。原始無效承載永遠不會被持續保存。
* 當承載中沒有欄位可以被修復時，Claude Code 會保留最後接受的快取設定並記錄致命錯誤。
* [安全核准對話方塊](#security-approval-dialogs)會評估已修復的承載，因此被移除的無效項目永遠不會被呈現以供核准，也永遠不會執行。

若要偵錯傳遞問題，請執行 `claude --debug-file <path>` 並在日誌中搜尋 `Remote settings`。在將承載變更推出到組織之前，請在測試機器上使用 `claude doctor` 驗證承載變更。

<h3 id="enforce-fail-closed-startup">
  強制執行失敗關閉啟動
</h3>

根據預設，如果遠端設定擷取在啟動時失敗，CLI 會在沒有受管設定的情況下繼續。對於這個簡短的未強制執行視窗無法接受的環境，請在您的受管設定中設定 `forceRemoteSettingsRefresh: true`。

當此設定處於作用中時，CLI 會在啟動時阻止，直到遠端設定被新鮮擷取。如果擷取失敗，CLI 會結束而不是在沒有原則的情況下繼續。此設定會自我延續：一旦從伺服器傳遞，它也會在本機快取，以便後續啟動即使在新工作階段的第一次成功擷取之前也會強制執行相同的行為。

若要啟用此功能，請將金鑰新增到您的受管設定設定：

```json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

您也可以在[端點管理的](/docs/zh-TW/settings#settings-files) MDM 設定檔或系統 `managed-settings.json` 檔案中設定此金鑰，以在首次啟動時強制執行失敗關閉行為，在任何伺服器承載被傳遞之前。自 v2.1.191 起，此旗標是上述[優先順序規則](#settings-precedence)的例外：當在任何受管來源中設定時，即使快取的伺服器管理承載也存在，它也會被接受，因此當伺服器管理的設定存在時，MDM 傳遞的值不會被忽略。

設定擷取也會傳送 `Cache-Control: no-cache` 標頭，以便中間 HTTP Proxy 不會提供過時的回應。

在啟用此設定之前，請確保您的網路原則允許連線到 `api.anthropic.com`。如果該端點無法到達，CLI 會在啟動時結束，使用者無法啟動 Claude Code。

自 v2.1.139 起，`claude auth` 子命令（例如 `claude auth login`）不受此檢查限制，因此使用者可以在過期認證是設定擷取失敗原因時重新驗證。

<h3 id="security-approval-dialogs">
  安全核准對話方塊
</h3>

某些可能造成安全風險的設定需要明確的使用者核准才能套用：

* **Shell 命令設定**：執行 shell 命令的設定
* **自訂環境變數**：不在已知安全允許清單中的變數
* **Hook 設定**：任何 hook 定義
* **受管 CLAUDE.md 內容**：透過受管設定傳遞的 `claudeMd` 值

當這些設定存在時，使用者會看到安全對話方塊，說明正在設定的內容。使用者必須核准才能繼續。如果使用者拒絕設定，Claude Code 會結束。

<Note>
  非互動執行（例如 `claude -p` 或 Agent SDK 工作階段）無法顯示對話方塊。當傳遞的設定需要核准時，Claude Code 會僅針對該執行套用它們：它不會將它們記錄為已核准或寫入[本機快取](#fetch-and-caching-behavior)，下一個互動工作階段會顯示對話方塊。在使用者在互動工作階段中核准之前，每個非互動執行都會在啟動時再次擷取設定。在 v2.1.207 之前，非互動執行會將設定儲存為已核准，因此後來的互動工作階段永遠不會為它們顯示對話方塊。
</Note>

<h2 id="platform-availability">
  平台可用性
</h2>

伺服器管理的設定需要直接連線到 `api.anthropic.com`，並且傳遞需要工作階段使用組織 OAuth 登入或直接配置的 API 金鑰進行驗證。由 [`apiKeyHelper`](/docs/zh-TW/settings#available-settings) 指令碼傳回的金鑰不會觸發設定擷取。

在使用第三方模型提供者時，伺服器管理的設定無法使用：

* Amazon Bedrock
* Google Cloud 的 Agent Platform
* Microsoft Foundry
* [Claude Platform on AWS](/docs/zh-TW/claude-platform-on-aws)
* 透過 `ANTHROPIC_BASE_URL` 或第三方 [LLM 閘道](/docs/zh-TW/llm-gateway) 的自訂 API 端點

如果您在殼層中匯出 `CLAUDE_CODE_USE_*` 提供者變數或非預設的 `ANTHROPIC_BASE_URL`，Claude Code 會略過您工作階段的設定擷取。您無法使用伺服器管理的 `env` 區塊清除匯出，因為該區塊是透過匯出所防止的擷取來傳遞的。[端點管理的設定](/docs/zh-TW/settings#settings-files) `env` 區塊也不會還原擷取：Claude Code 在套用管理的 `env` 區塊之前會檢查合格性，因此覆寫會變更工作階段的提供者選擇，但擷取仍會被略過。

若要還原伺服器管理的傳遞，請從殼層移除匯出，或在您的使用者設定 `env` 區塊中將變數設定為 `""`，這會在合格性檢查之前套用。若要在不依賴使用者變更其殼層的情況下強制執行原則，請改為透過端點管理的通道傳遞設定。

對於 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 部署，自託管的 [Claude 應用程式閘道](/docs/zh-TW/claude-apps-gateway) 提供等效的遠端管理設定傳遞：閘道登入的用戶端從閘道而不是 `api.anthropic.com` 擷取管理設定。啟動時的失敗語義不同：無法到達閘道的閘道用戶端會以錯誤結束，而不是回退到快取的設定，而每小時的背景重新整理在兩個通道上都是開放失敗的。

<h2 id="audit-logging">
  稽核記錄
</h2>

設定變更的稽核記錄事件可透過合規性 API 或稽核記錄匯出取得。請聯絡您的 Anthropic 帳戶團隊以取得存取權。

稽核事件包括執行的動作類型、執行動作的帳戶和裝置，以及對先前和新值的參考。

<h2 id="security-considerations">
  安全考量
</h2>

伺服器管理的設定提供集中式原則強制執行，但它們作為用戶端控制運作，而非安全邊界。在非受管裝置上，使用者不需要管理員或 sudo 存取權就能略過它們。

| 情況                                      | 行為                                                                                                                                                                                                                                           |
| :-------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 使用者編輯快取的設定檔                             | 篡改的檔案在啟動時套用，但正確的設定會在下次伺服器擷取時還原。自 v2.1.198 起，`env` 區塊中的傳輸、API 路由和驗證環境變數會[在伺服器確認承載後才提供](#fetch-and-caching-behavior)                                                                                                                           |
| 使用者刪除快取的設定檔                             | 首次啟動行為發生：設定非同步擷取，有一個簡短的未強制執行視窗                                                                                                                                                                                                               |
| 使用者執行修改過的 Claude Code 二進位檔              | 能夠執行修改過用戶端的使用者可以略過任何用戶端控制                                                                                                                                                                                                                    |
| 使用者執行較舊的 Claude Code 版本                 | 早於伺服器管理設定的版本不會擷取或套用它們                                                                                                                                                                                                                        |
| API 無法使用                                | 如果可用，快取設定會套用，否則受管設定在下次成功擷取之前不會強制執行。自 v2.1.198 起，快取 `env` 區塊中的傳輸、API 路由和驗證環境變數會[在擷取失敗時被保留](#fetch-and-caching-behavior)；快取的其餘部分仍然適用。使用 `forceRemoteSettingsRefresh: true` 時，CLI 會結束而不是繼續，除了 [`claude auth` 子命令](#enforce-fail-closed-startup) |
| 使用者使用不同的組織進行身份驗證                        | 不會為受管組織外的帳戶傳遞設定                                                                                                                                                                                                                              |
| 使用者設定[第三方模型提供者](#platform-availability) | 伺服器管理的設定會被略過。這包括設定 `CLAUDE_CODE_USE_BEDROCK`、`CLAUDE_CODE_USE_MANTLE`、`CLAUDE_CODE_USE_VERTEX`、`CLAUDE_CODE_USE_FOUNDRY`、`CLAUDE_CODE_USE_ANTHROPIC_AWS` 或非預設的 `ANTHROPIC_BASE_URL`                                                          |
| 網路流量被攔截或重新導向                            | 停用的 TLS 驗證或攔截的流量可以改變用戶端接收的設定                                                                                                                                                                                                                 |

若要偵測執行時期設定變更，請使用 [`ConfigChange` hooks](/docs/zh-TW/hooks#configchange) 來記錄修改或在未授權的變更生效前阻止它們。

若要限制使用者可以使用用戶端提供的認證存取的組織，請參閱 Claude 說明中心中的[使用租戶限制強制執行網路層級存取控制](https://support.claude.com/en/articles/13198485-enforce-network-level-access-control-with-tenant-restrictions)。如需更強的強制執行保證，請在已在 MDM 解決方案中註冊的裝置上使用[端點管理的設定](/docs/zh-TW/settings#settings-files)。

<h2 id="see-also">
  另請參閱
</h2>

用於管理 Claude Code 設定的相關頁面：

* [Settings](/docs/zh-TW/settings)：完整的設定參考，包括所有可用的設定
* [Endpoint-managed settings](/docs/zh-TW/settings#settings-files)：由 IT 部門部署到裝置的受管設定
* [Authentication](/docs/zh-TW/authentication)：設定使用者對 Claude Code 的存取
* [Security](/docs/zh-TW/security)：安全保護措施和最佳實踐
