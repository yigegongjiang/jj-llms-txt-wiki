> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 驗證

> 登入 Claude Code 並為個人、團隊和組織配置驗證。

Claude Code 支援多種驗證方法，具體取決於您的設定。個人使用者可以使用 Claude.ai 帳戶登入，而團隊可以使用 Claude for Teams 或 Enterprise、Claude Console 或雲端提供商（如 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry）。

<h2 id="log-in-to-claude-code">
  登入 Claude Code
</h2>

[安裝 Claude Code](/docs/zh-TW/setup#install-claude-code) 後，在您的終端機中執行 `claude`。首次啟動時，Claude Code 會為您開啟瀏覽器視窗以供登入。

如果瀏覽器未自動開啟，請按 `c` 將登入 URL 複製到您的剪貼簿，然後將其貼到您的瀏覽器中。

如果您的瀏覽器在您登入後顯示登入代碼而不是重新導向回來，請將其貼到終端機的 `Paste code here if prompted` 提示符處。這在瀏覽器無法連接到 Claude Code 的本機回呼伺服器時發生，這在 WSL2、SSH 工作階段和容器中很常見。

登入完成時，終端機會顯示 `Login successful`，並提示您按 `Enter` 繼續。

您可以使用以下任何帳戶類型進行驗證：

* **Claude Pro 或 Max 訂閱**：使用您的 Claude.ai 帳戶登入。在 [claude.com/pricing](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_pro_max) 訂閱。
* **Claude for Teams 或 Enterprise**：使用您的團隊管理員邀請您的 Claude.ai 帳戶登入。
* **Claude Console**：使用您的 Console 認證登入。您的管理員必須先 [邀請您](#claude-console-authentication)。
* **雲端提供商**：如果您的組織使用 [Amazon Bedrock](/docs/zh-TW/amazon-bedrock)、[Google Cloud 的 Agent Platform](/docs/zh-TW/google-vertex-ai) 或 [Microsoft Foundry](/docs/zh-TW/microsoft-foundry)，請在執行 `claude` 之前設定所需的環境變數，或在登入提示符處選擇 **3rd-party platform**，這會為 Bedrock 和 Vertex AI 啟動互動式設定精靈。不需要瀏覽器登入。
* **雲端閘道**：如果您的組織執行自託管的 [Claude 應用程式閘道](/docs/zh-TW/claude-apps-gateway)，請透過 `/login` 使用公司 SSO 登入。閘道簽發的權杖是工作階段的唯一認證。

管理員可以使用 [`forceLoginMethod` 和 `forceLoginOrgUUID`](/docs/zh-TW/settings#available-settings) 受管設定來限制互動式登入。當設定其中任一項時，由 `ANTHROPIC_API_KEY`、`ANTHROPIC_AUTH_TOKEN` 或 `apiKeyHelper` 驗證的工作階段在啟動時會被封鎖；雲端提供商工作階段不受影響。

若要登出並重新驗證，請在 Claude Code 提示符處輸入 `/logout`。登出也會重設您的首次啟動設定狀態，因此下次您執行 `claude` 時，它會再次引導您完成登入和設定。

如果您在登入時遇到問題，請參閱 [驗證疑難排解](/docs/zh-TW/troubleshoot-install#login-and-authentication)。

<h2 id="set-up-team-authentication">
  設定團隊驗證
</h2>

對於團隊和組織，您可以透過以下方式之一配置 Claude Code 存取：

* [Claude for Teams 或 Enterprise](#claude-for-teams-or-enterprise)，建議用於大多數團隊
* [Claude Console](#claude-console-authentication)
* [Claude apps gateway](/docs/zh-TW/claude-apps-gateway)，一個自託管閘道，使用您的 IdP 簽署開發人員，並將推論路由到您配置的雲端提供商
* [Amazon Bedrock](/docs/zh-TW/amazon-bedrock)
* [Google Cloud's Agent Platform](/docs/zh-TW/google-vertex-ai)
* [Microsoft Foundry](/docs/zh-TW/microsoft-foundry)

<h3 id="claude-for-teams-or-enterprise">
  Claude for Teams 或 Enterprise
</h3>

[Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams#team-&-enterprise) 和 [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise) 為使用 Claude Code 的組織提供最佳體驗。團隊成員可以存取 Claude Code 和網頁版 Claude，並具有集中式帳單和團隊管理。

* **Claude for Teams**：自助服務方案，具有協作功能、管理工具和帳單管理。最適合較小的團隊。
* **Claude for Enterprise**：新增 SSO、網域擷取、角色型權限、合規性 API 和受管原則設定，用於組織範圍的 Claude Code 配置。最適合具有安全性和合規性要求的大型組織。

<Steps>
  <Step title="訂閱">
    訂閱 [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams_step#team-&-enterprise) 或聯絡銷售部門以取得 [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise_step)。
  </Step>

  <Step title="邀請團隊成員">
    從管理儀表板邀請團隊成員。
  </Step>

  <Step title="安裝並登入">
    團隊成員安裝 Claude Code 並使用其 Claude.ai 帳戶登入。
  </Step>
</Steps>

<h3 id="claude-console-authentication">
  Claude Console 驗證
</h3>

對於偏好基於 API 的帳單的組織，您可以透過 Claude Console 設定存取。

<Steps>
  <Step title="建立或使用 Console 帳戶">
    使用您現有的 Claude Console 帳戶或建立新帳戶。
  </Step>

  <Step title="新增使用者">
    您可以透過以下任一方法新增使用者：

    * 從 Console 內大量邀請使用者：Settings -> Members -> Invite
    * [設定 SSO](https://support.claude.com/en/articles/13132885-setting-up-single-sign-on-sso)
  </Step>

  <Step title="指派角色">
    邀請使用者時，指派以下其中一個角色：

    * **Claude Code** 角色：使用者只能建立 Claude Code API 金鑰
    * **Developer** 角色：使用者可以建立任何類型的 API 金鑰
  </Step>

  <Step title="使用者完成設定">
    每個受邀使用者需要：

    * 接受 Console 邀請
    * [檢查系統要求](/docs/zh-TW/setup#system-requirements)
    * [安裝 Claude Code](/docs/zh-TW/setup#install-claude-code)
    * 使用 Console 帳戶認證登入
  </Step>
</Steps>

<h3 id="cloud-provider-authentication">
  雲端提供商驗證
</h3>

對於使用 Amazon Bedrock、Google Cloud's Agent Platform 或 Microsoft Foundry 的團隊：

<Steps>
  <Step title="遵循提供商設定">
    遵循 [Amazon Bedrock 文件](/docs/zh-TW/amazon-bedrock)、[Google Cloud's Agent Platform 文件](/docs/zh-TW/google-vertex-ai) 或 [Microsoft Foundry 文件](/docs/zh-TW/microsoft-foundry)。
  </Step>

  <Step title="分發配置">
    將環境變數和產生雲端認證的說明分發給您的使用者。深入瞭解如何 [在此管理配置](/docs/zh-TW/settings)。
  </Step>

  <Step title="安裝 Claude Code">
    使用者可以 [安裝 Claude Code](/docs/zh-TW/setup#install-claude-code)。
  </Step>
</Steps>

<h2 id="credential-management">
  認證管理
</h2>

Claude Code 安全地管理您的驗證認證：

* **儲存位置**：
  * 在 macOS 上，認證儲存在加密的 macOS Keychain 中。
  * 在 Linux 上，認證儲存在 `~/.claude/.credentials.json` 中，檔案模式為 `0600`。
  * 在 Windows 上，認證儲存在 `%USERPROFILE%\.claude\.credentials.json` 中，並繼承您的使用者設定檔目錄的存取控制，預設情況下將檔案限制為您的使用者帳戶。
  * 如果您在 Linux 或 Windows 上設定了 `CLAUDE_CONFIG_DIR` 環境變數，`.credentials.json` 檔案將位於該目錄下。
  * Claude Code 透過 `/login` 和 `/logout` 管理 `.credentials.json`。若要透過自訂 API 端點路由請求，請改為設定 [`ANTHROPIC_BASE_URL`](/docs/zh-TW/env-vars) 環境變數。
* **支援的驗證類型**：Claude.ai 認證、Claude API 認證、Microsoft Foundry Auth、Bedrock Auth、Vertex Auth 和 [Claude apps gateway](/docs/zh-TW/claude-apps-gateway) 工作階段令牌。
* **自訂認證指令碼**：[`apiKeyHelper`](/docs/zh-TW/settings#available-settings) 設定可以配置為執行傳回 API 金鑰的 shell 指令碼。
* **重新整理間隔**：根據預設，`apiKeyHelper` 在 5 分鐘後或在 HTTP 401 回應時呼叫。設定 `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` 環境變數以自訂重新整理間隔。
* **緩慢協助程式通知**：如果 `apiKeyHelper` 花費超過 10 秒的時間傳回金鑰，Claude Code 會在提示符列中顯示警告通知，顯示經過的時間。如果您經常看到此通知，請檢查您的認證指令碼是否可以最佳化。
* **協助程式失敗**：當指令碼以錯誤結束、逾時或不列印任何內容時，請求在三次嘗試內失敗，並顯示 [`Your apiKeyHelper script is failing`](/docs/zh-TW/errors#your-apikeyhelper-script-is-failing)。在 v2.1.208 之前，協助程式失敗會在大約十次無聲重試後顯示為通用 401。

`apiKeyHelper`、`ANTHROPIC_API_KEY` 和 `ANTHROPIC_AUTH_TOKEN` 適用於 CLI 和包裝它的介面，包括 VS Code 擴充功能、Agent SDK 和 GitHub Actions。Claude Desktop 和雲端工作階段不會呼叫 `apiKeyHelper` 或讀取這些環境變數：它們使用 OAuth，除了執行[第三方推論配置](/docs/zh-TW/llm-gateway-connect#desktop-app)的桌面工作階段外，該工作階段使用該配置的認證進行驗證。

<h3 id="renew-an-expiring-login">
  續約即將過期的登入
</h3>

當您使用 `/login` 建立的登入在五天內即將過期時，Claude Code 會在啟動時顯示警告：`您的登入將在 3 天後過期 · 執行 /login 以續約`。需要 Claude Code v2.1.203 或更新版本。

執行 `/login` 以續約。警告僅供參考，永遠不會阻止請求：驗證會持續運作，直到登入實際過期。登入生命週期本身保持不變；提前警告是 v2.1.203 新增的功能。

一旦儲存的登入過期且無法重新整理，每個請求都會失敗，並顯示 [`Login expired · Please run /login`](/docs/zh-TW/errors#login-expired)，直到您再次登入。在 v2.1.206 之前，過期的登入會顯示為模型錯誤。

警告僅在 claude.ai 或 Claude Console 登入是有效認證時出現，而不是在雲端提供商、`ANTHROPIC_API_KEY`、`ANTHROPIC_AUTH_TOKEN` 或 `apiKeyHelper` 提供認證時出現。

對於執行無人值守的工作階段，提前續約最為重要。在[代理檢視中的背景工作階段](/docs/zh-TW/agent-view)或[遠端控制](/docs/zh-TW/remote-control)工作階段一旦超過登入生命週期，一旦認證過期就會停止進行，在您再次登入之前無法恢復。

<h3 id="authentication-precedence">
  驗證優先順序
</h3>

當存在多個認證時，Claude Code 按此順序選擇一個：

1. 雲端提供商認證，當設定了 `CLAUDE_CODE_USE_BEDROCK`、`CLAUDE_CODE_USE_VERTEX` 或 `CLAUDE_CODE_USE_FOUNDRY` 時。請參閱[第三方整合](/docs/zh-TW/third-party-integrations)以取得設定。
2. `ANTHROPIC_AUTH_TOKEN` 環境變數。作為 `Authorization: Bearer` 標頭傳送。當透過[LLM 閘道或代理](/docs/zh-TW/llm-gateway)路由時使用此選項，該閘道或代理使用持有人令牌而不是 Anthropic API 金鑰進行驗證。
3. `ANTHROPIC_API_KEY` 環境變數。作為 `X-Api-Key` 標頭傳送。用於直接 Anthropic API 存取，使用來自 [Claude Console](https://platform.claude.com) 的金鑰。在互動模式下，系統會提示您一次以核准或拒絕金鑰，您的選擇會被記住。若要稍後變更，請使用 `/config` 中的「使用自訂 API 金鑰」切換。在非互動模式 (`-p`) 中，當金鑰存在時始終使用該金鑰。
4. [`apiKeyHelper`](/docs/zh-TW/settings#available-settings) 指令碼輸出。用於動態或輪換認證，例如從保管庫擷取的短期令牌。
5. `CLAUDE_CODE_OAUTH_TOKEN` 環境變數。由 [`claude setup-token`](#generate-a-long-lived-token) 產生的長期 OAuth 令牌。用於 CI 管道和指令碼，其中瀏覽器登入不可用。
6. 來自 `/login` 的訂閱 OAuth 認證。這是 Claude Pro、Max、Team 和 Enterprise 使用者的預設值。

已簽署的 [Claude apps gateway](/docs/zh-TW/claude-apps-gateway) 工作階段位於此清單之外：它是一個提供商選擇，如 Amazon Bedrock 或 Google Cloud 的 Agent Platform，並且優先於它們。當閘道工作階段存在時，CLI 使用閘道令牌進行驗證，即使設定了 `CLAUDE_CODE_USE_BEDROCK`、`CLAUDE_CODE_USE_VERTEX` 或 `CLAUDE_CODE_USE_FOUNDRY`，上面的持有人令牌、API 金鑰和 `apiKeyHelper` 項目也不會被使用。

如果您有有效的 Claude 訂閱，但您的環境中也設定了 `ANTHROPIC_API_KEY`，則 API 金鑰在核准後優先。如果金鑰屬於已停用或過期的組織，這可能會導致驗證失敗。執行 `unset ANTHROPIC_API_KEY` 以回退到您的訂閱，並檢查 `/status` 以確認哪種方法處於活動狀態。`Login method` 列會顯示您的訂閱帳戶，當 API 金鑰在使用中時會出現 `API key` 列。

[網頁版 Claude Code](/docs/zh-TW/claude-code-on-the-web) 始終使用您的訂閱認證。如果您在沙箱環境中設定 `ANTHROPIC_API_KEY` 或 `ANTHROPIC_AUTH_TOKEN`，它不會覆蓋您的訂閱認證。

<h3 id="generate-a-long-lived-token">
  產生長期令牌
</h3>

對於 CI 管道、指令碼或其他互動式瀏覽器登入不可用的環境，使用 `claude setup-token` 產生一年期 OAuth 令牌：

```bash theme={null}
claude setup-token
```

該命令會引導您完成 OAuth 授權並將令牌列印到終端機。它不會將令牌儲存在任何地方；複製它並將其設定為您想要驗證的任何地方的 `CLAUDE_CODE_OAUTH_TOKEN` 環境變數：

```bash theme={null}
export CLAUDE_CODE_OAUTH_TOKEN=your-token
```

此令牌使用您的 Claude 訂閱進行驗證，需要 Pro、Max、Team 或 Enterprise 方案。它的範圍僅限於推論，無法建立 [Remote Control](/docs/zh-TW/remote-control) 工作階段。

[Bare mode](/docs/zh-TW/headless#start-faster-with-bare-mode) 不讀取 `CLAUDE_CODE_OAUTH_TOKEN`。如果您的指令碼傳遞 `--bare`，請改用 `ANTHROPIC_API_KEY` 或 `apiKeyHelper` 進行驗證。
