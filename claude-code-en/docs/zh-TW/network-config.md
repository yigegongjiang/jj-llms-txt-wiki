> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 企業網路設定

> 為企業環境設定 Claude Code，包括代理伺服器、自訂憑證授權單位 (CA) 和相互傳輸層安全性 (mTLS) 驗證。

Claude Code 透過環境變數支援各種企業網路和安全設定。這包括透過公司代理伺服器路由流量、信任自訂憑證授權單位 (CA)，以及使用相互傳輸層安全性 (mTLS) 憑證進行驗證以增強安全性。

<Note>
  本頁面顯示的所有環境變數也可以在 [`settings.json`](/docs/zh-TW/settings) 中設定。
</Note>

<h2 id="proxy-configuration">
  代理設定
</h2>

<h3 id="environment-variables">
  環境變數
</h3>

Claude Code 遵守標準代理環境變數：

```bash theme={null}
# HTTPS 代理（建議）
export HTTPS_PROXY=https://proxy.example.com:8080

# HTTP 代理（如果 HTTPS 不可用）
export HTTP_PROXY=http://proxy.example.com:8080

# 略過特定請求的代理 - 空格分隔格式
export NO_PROXY="localhost 192.168.1.1 example.com .example.com"
# 略過特定請求的代理 - 逗號分隔格式
export NO_PROXY="localhost,192.168.1.1,example.com,.example.com"
# 略過所有請求的代理
export NO_PROXY="*"
```

<Note>
  Claude Code 不支援 SOCKS 代理。
</Note>

<h3 id="basic-authentication">
  基本驗證
</h3>

如果您的代理需要基本驗證，請在代理 URL 中包含認證資訊：

```bash theme={null}
export HTTPS_PROXY=http://username:password@proxy.example.com:8080
```

<Warning>
  避免在指令碼中硬編碼密碼。改用環境變數或安全認證儲存。
</Warning>

<Tip>
  對於需要進階驗證（NTLM、Kerberos 等）的代理，請考慮使用支援您驗證方法的 LLM Gateway 服務。
</Tip>

<h2 id="ca-certificate-store">
  CA 憑證存放區
</h2>

根據預設，Claude Code 信任其捆綁的 Mozilla CA 憑證和您作業系統的憑證存放區。讀取作業系統存放區需要具有 `tls.getCACertificates` 的執行時環境：原生安裝程式始終具有它，npm 安裝需要 Node 22.15 或更新版本。在較舊的 Node 版本上，只有捆綁的集合和 `NODE_EXTRA_CA_CERTS` 適用。企業 TLS 檢查代理（例如 CrowdStrike Falcon 和 Zscaler）在其根憑證安裝在作業系統信任存放區中且執行時可以讀取它時，無需額外設定即可運作。

`CLAUDE_CODE_CERT_STORE` 接受以逗號分隔的來源清單。認可的值為 `bundled`（Claude Code 隨附的 Mozilla CA 集合）和 `system`（作業系統信任存放區）。預設值為 `bundled,system`。

若只信任捆綁的 Mozilla CA 集合：

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=bundled
```

若只信任作業系統憑證存放區：

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=system
```

<Note>
  `CLAUDE_CODE_CERT_STORE` 在 `settings.json` 中沒有專用的架構金鑰。透過 `~/.claude/settings.json` 中的 `env` 區塊或直接在程序環境中設定。
</Note>

<h2 id="custom-ca-certificates">
  自訂 CA 憑證
</h2>

如果您的企業環境使用自訂 CA，請設定 Claude Code 以直接信任它：

```bash theme={null}
export NODE_EXTRA_CA_CERTS=/path/to/ca-cert.pem
```

<h2 id="mtls-authentication">
  mTLS 驗證
</h2>

對於需要用戶端憑證驗證的企業環境：

```bash theme={null}
# 用於驗證的用戶端憑證
export CLAUDE_CODE_CLIENT_CERT=/path/to/client-cert.pem

# 用戶端私密金鑰
export CLAUDE_CODE_CLIENT_KEY=/path/to/client-key.pem

# 選用：加密私密金鑰的密碼
export CLAUDE_CODE_CLIENT_KEY_PASSPHRASE="your-passphrase"
```

Claude Code 在啟動時讀取憑證和金鑰檔案，並在每次套用設定時重新讀取它們，包括在工作階段期間設定變更時。若要輪換憑證和金鑰，請取代相同路徑上的檔案。

<h2 id="network-access-requirements">
  網路存取需求
</h2>

Claude Code 需要存取以下 URL。請在您的代理設定和防火牆規則中將這些 URL 列入允許清單，特別是在容器化或受限網路環境中。

| URL                            | 用途                                                                                                                                                                                                                                                                  |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api.anthropic.com`            | Claude API 請求                                                                                                                                                                                                                                                       |
| `claude.ai`                    | claude.ai 帳戶驗證                                                                                                                                                                                                                                                      |
| `platform.claude.com`          | Anthropic Console 帳戶驗證                                                                                                                                                                                                                                              |
| `mcp-proxy.anthropic.com`      | [來自 claude.ai 的 MCP 連接器](/docs/zh-TW/mcp#use-mcp-servers-from-claude-ai)，包括組織管理員設定的連接器。連接器流量會透過此代理路由；對於 claude.ai 驗證的使用者，連接器預設為啟用。若要停用，請設定 [`ENABLE_CLAUDEAI_MCP_SERVERS=false`](/docs/zh-TW/env-vars) 或 [`disableClaudeAiConnectors`](/docs/zh-TW/settings#available-settings) 設定 |
| `downloads.claude.ai`          | 外掛程式可執行檔下載；原生安裝程式和原生自動更新程式                                                                                                                                                                                                                                          |
| `storage.googleapis.com`       | 在 `/plugin` 中顯示的安裝計數和外掛程式中繼資料。已簽署的[成品](/docs/zh-TW/artifacts)上傳會先嘗試此主機；當 `api.anthropic.com` 被阻止時，發佈會回退到 `api.anthropic.com`                                                                                                                                             |
| `storage.googleapis.com`       | 2.1.116 版本之前的原生安裝程式和原生自動更新程式                                                                                                                                                                                                                                        |
| `bridge.claudeusercontent.com` | [Chrome 中的 Claude](/docs/zh-TW/chrome) 擴充功能 WebSocket 橋接器                                                                                                                                                                                                                |
| `*.claudeusercontent.com`      | 在 claude.ai 上檢視[成品](/docs/zh-TW/artifacts)。檢視器會從此來源的沙箱子網域載入每個成品的內容。檢視器的瀏覽器需要此項，CLI 本身不需要                                                                                                                                                                                 |
| `raw.githubusercontent.com`    | [`/release-notes`](/docs/zh-TW/commands) 的變更日誌摘要和更新後顯示的版本資訊                                                                                                                                                                                                              |

如果您透過 npm 安裝 Claude Code 或管理自己的二進位分發，終端使用者不需要原生安裝程式，自動更新程式不需要使用 `downloads.claude.ai`。表格中的其他用途無論安裝方法為何都適用。

Claude Code 預設也會傳送選用的操作遙測，您可以使用環境變數停用此功能。請參閱[遙測服務](/docs/zh-TW/data-usage#telemetry-services)以了解如何在完成允許清單之前停用它。

使用 [Amazon Bedrock](/docs/zh-TW/amazon-bedrock)、[Google Cloud 的 Agent Platform](/docs/zh-TW/google-vertex-ai)、[Microsoft Foundry](/docs/zh-TW/microsoft-foundry) 或已登入的 [Claude 應用程式閘道](/docs/zh-TW/claude-apps-gateway)工作階段時，模型流量和驗證會傳送到您的提供者或閘道，而不是 `api.anthropic.com`、`claude.ai` 或 `platform.claude.com`。WebFetch 工具仍會呼叫 `api.anthropic.com` 進行其[網域安全檢查](/docs/zh-TW/data-usage#webfetch-domain-safety-check)，除非您在[設定](/docs/zh-TW/settings)中設定 `skipWebFetchPreflight: true`。

[Claude Code on the web](/docs/zh-TW/claude-code-on-the-web) 和 [Code Review](/docs/zh-TW/code-review) 從 Anthropic 管理的基礎設施連線到您的儲存庫。如果您的 GitHub Enterprise Cloud 組織按 IP 位址限制存取，請啟用[已安裝 GitHub Apps 的 IP 允許清單繼承](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#allowing-access-by-github-apps)。Claude GitHub App 會註冊其 IP 範圍，因此啟用此設定可允許存取而無需手動設定。若要[手動將範圍新增到允許清單](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#adding-an-allowed-ip-address)，或設定其他防火牆，請參閱 [Anthropic API IP 位址](https://platform.claude.com/docs/en/api/ip-addresses)。

對於防火牆後的自託管 [GitHub Enterprise Server](/docs/zh-TW/github-enterprise-server) 執行個體，請將相同的 [Anthropic API IP 位址](https://platform.claude.com/docs/en/api/ip-addresses) 列入允許清單，以便 Anthropic 基礎設施可以連線到您的 GHES 主機以複製儲存庫並發佈審查評論。

<h3 id="desktop-and-claude-ai">
  Desktop 和 claude.ai
</h3>

前面的表格主要涵蓋獨立 CLI。Claude Desktop 應用程式和瀏覽器中的 claude.ai 會從其他 Anthropic CDN 主機載入其應用程式代碼，包括 `assets-proxy.anthropic.com`。允許 `claude.ai` 但阻止這些主機會產生空白頁面而不是錯誤。請參閱 Desktop 頁面上的[網路存取需求](/docs/zh-TW/desktop#network-access-requirements)。

<h2 id="additional-resources">
  其他資源
</h2>

* [Claude Code 設定](/docs/zh-TW/settings)
* [環境變數參考](/docs/zh-TW/env-vars)
* [疑難排解指南](/docs/zh-TW/troubleshooting)
