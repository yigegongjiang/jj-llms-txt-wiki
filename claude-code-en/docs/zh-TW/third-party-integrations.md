> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 企業部署概述

> 了解 Claude Code 如何與各種第三方服務和基礎設施整合，以滿足企業部署需求。

export const ContactSalesCard = ({surface}) => {
  const utm = content => `utm_source=claude_code&utm_medium=docs&utm_content=${surface}_${content}`;
  const iconArrowRight = (size = 13) => <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
      <line x1="5" y1="12" x2="19" y2="12" />
      <polyline points="12 5 19 12 12 19" />
    </svg>;
  const STYLES = `
.cc-cs {
  --cs-slate: #141413;
  --cs-clay: #d97757;
  --cs-clay-deep: #c6613f;
  --cs-gray-000: #ffffff;
  --cs-gray-700: #3d3d3a;
  --cs-border-default: rgba(31, 30, 29, 0.15);
  font-family: inherit;
}
.dark .cc-cs {
  --cs-slate: #f0eee6;
  --cs-gray-000: #262624;
  --cs-gray-700: #bfbdb4;
  --cs-border-default: rgba(240, 238, 230, 0.14);
}
.cc-cs-card {
  display: flex; align-items: center; justify-content: space-between;
  gap: 16px; padding: 14px 16px; margin: 0;
  background: var(--cs-gray-000); border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; flex-wrap: wrap;
}
.cc-cs-text { font-size: 13px; color: var(--cs-gray-700); line-height: 1.5; flex: 1; min-width: 240px; }
.cc-cs-text strong { font-weight: 550; color: var(--cs-slate); }
.cc-cs-actions { display: flex; align-items: center; gap: 8px; flex-shrink: 0; }
.cc-cs-btn-clay {
  display: inline-flex; align-items: center; gap: 8px;
  background: var(--cs-clay-deep); color: #fff; border: none;
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
  transition: background-color 0.15s; white-space: nowrap;
}
.cc-cs-btn-clay:hover { background: var(--cs-clay); }
.cc-cs-btn-ghost {
  display: inline-flex; align-items: center; gap: 8px;
  background: transparent; color: var(--cs-gray-700);
  border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
}
.cc-cs-btn-ghost:hover { background: rgba(0, 0, 0, 0.04); }
.dark .cc-cs-btn-ghost:hover { background: rgba(255, 255, 255, 0.04); }
@media (max-width: 720px) {
  .cc-cs-actions { width: 100%; }
}
`;
  return <div className="cc-cs not-prose">
      <style>{STYLES}</style>
      <div className="cc-cs-card">
        <div className="cc-cs-text">
          <strong>Deploying Claude Code across your organization?</strong> Talk to sales about enterprise plans, SSO, and centralized billing.
        </div>
        <div className="cc-cs-actions">
          <a href={`https://claude.com/pricing?${utm('view_plans')}#plans-business`} className="cc-cs-btn-ghost">
            View plans
          </a>
          <a href={`https://claude.com/contact-sales?${utm('contact_sales')}`} className="cc-cs-btn-clay">
            Contact sales {iconArrowRight()}
          </a>
        </div>
      </div>
    </div>;
};

組織可以直接通過 Anthropic 或通過雲端提供商部署 Claude Code。本頁面幫助您選擇正確的配置。

<ContactSalesCard surface="third_party_overview" />

<h2 id="compare-deployment-options">
  比較部署選項
</h2>

對於大多數組織，Claude for Teams 或 Claude for Enterprise 提供最佳體驗。團隊成員可以通過單一訂閱同時存取 Claude Code 和網頁版 Claude，具有集中計費和無需基礎設施設置的優勢。

**Claude for Teams** 是自助服務，包括協作功能、管理工具、SSO、計費管理和[伺服器管理的設定](/docs/zh-TW/server-managed-settings)，用於組織範圍的 Claude Code 配置。最適合需要快速開始的較小團隊。

**Claude for Enterprise** 增加了網域擷取、角色型權限和合規性 API 存取。最適合具有安全和合規性要求的大型組織。

了解更多關於 [Team 計劃](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) 和 [Enterprise 計劃](https://support.claude.com/en/articles/9797531-what-is-the-enterprise-plan)。

部署選項的比較涵蓋模型推理執行的位置。若要在您的組織運營的計算上執行 [Claude Code 網頁版](/docs/zh-TW/claude-code-on-the-web) 工作階段，請參閱 [自託管環境](/docs/zh-TW/self-hosted-environments)。

如果您的組織有特定的基礎設施要求，請比較以下選項：

<table>
  <thead>
    <tr>
      <th>功能</th>
      <th>Claude for Teams/Enterprise</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform on AWS</th>
      <th>Google Cloud's Agent Platform，前身為 Vertex AI</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>最適合</td>
      <td>大多數組織（推薦）</td>
      <td>個人開發者</td>
      <td>AWS 原生部署</td>
      <td>AWS Marketplace 計費搭配 Claude API 功能</td>
      <td>GCP 原生部署</td>
      <td>Azure 原生部署</td>
    </tr>

    <tr>
      <td>計費</td>
      <td><strong>Teams：</strong> \$150/座位（Premium）提供 PAYG<br /><strong>Enterprise：</strong> <a href="https://claude.com/contact-sales?utm_source=claude_code&utm_medium=docs&utm_content=third_party_enterprise">聯絡銷售</a></td>
      <td>PAYG</td>
      <td>通過 AWS 的 PAYG</td>
      <td>通過 AWS Marketplace 的 PAYG</td>
      <td>通過 GCP 的 PAYG</td>
      <td>通過 Azure 的 PAYG</td>
    </tr>

    <tr>
      <td>地區</td>
      <td>支援的 [國家/地區](https://www.anthropic.com/supported-countries)</td>
      <td>支援的 [國家/地區](https://www.anthropic.com/supported-countries)</td>
      <td>多個 AWS [地區](https://docs.aws.amazon.com/bedrock/latest/userguide/models-regions.html)</td>
      <td>多個 AWS 地區</td>
      <td>多個 GCP [地區](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations)</td>
      <td>多個 Azure [地區](https://azure.microsoft.com/en-us/explore/global-infrastructure/products-by-region/)</td>
    </tr>

    <tr>
      <td>Prompt caching</td>
      <td>預設啟用</td>
      <td>預設啟用</td>
      <td>預設啟用</td>
      <td>預設啟用</td>
      <td>預設啟用</td>
      <td>預設啟用</td>
    </tr>

    <tr>
      <td>身份驗證</td>
      <td>claude.ai SSO 或電子郵件</td>
      <td>API 金鑰或 [Console 登入（無需 API 金鑰）](/docs/zh-TW/authentication#sign-in-without-an-api-key)</td>
      <td>API 金鑰或 AWS 認證</td>
      <td>API 金鑰或 AWS 認證</td>
      <td>GCP 認證</td>
      <td>API 金鑰或 Microsoft Entra ID</td>
    </tr>

    <tr>
      <td>成本追蹤</td>
      <td>使用儀表板</td>
      <td>使用儀表板</td>
      <td>AWS Cost Explorer</td>
      <td>AWS Cost Explorer</td>
      <td>GCP Billing</td>
      <td>Azure Cost Management</td>
    </tr>

    <tr>
      <td>包括網頁版 Claude</td>
      <td>是</td>
      <td>否</td>
      <td>否</td>
      <td>否</td>
      <td>否</td>
      <td>否</td>
    </tr>

    <tr>
      <td>企業功能</td>
      <td>團隊管理、SSO、使用監控</td>
      <td>無</td>
      <td>IAM 策略、CloudTrail</td>
      <td>IAM 策略、CloudTrail</td>
      <td>IAM 角色、Cloud Audit Logs</td>
      <td>RBAC 策略、Azure Monitor</td>
    </tr>
  </tbody>
</table>

如需了解每個選項上可用功能的逐項細目，請參閱 [功能可用性](/docs/zh-TW/feature-availability)。

選擇部署選項以查看設置說明：

* [Claude for Teams 或 Enterprise](/docs/zh-TW/authentication#claude-for-teams-or-enterprise)
* [Anthropic Console](/docs/zh-TW/authentication#claude-console-authentication)
* [Claude 應用程式閘道](/docs/zh-TW/claude-apps-gateway)，一個自託管閘道，在 Amazon Bedrock、Claude Platform on AWS、Google Cloud's Agent Platform、Microsoft Foundry 或 Anthropic API 前面添加 IdP 登入
* [Amazon Bedrock](/docs/zh-TW/amazon-bedrock)
* [Claude Platform on AWS](/docs/zh-TW/claude-platform-on-aws)
* [Google Cloud's Agent Platform](/docs/zh-TW/google-vertex-ai)
* [Microsoft Foundry](/docs/zh-TW/microsoft-foundry)

對於 Amazon Bedrock 和 Google Vertex AI，您也可以執行 `claude` 並在登入提示時選擇 **3rd-party platform** 以啟動互動式設置精靈。

<h2 id="configure-proxies-and-gateways">
  配置代理和網關
</h2>

大多數組織可以直接使用雲端提供商，無需額外配置。但是，如果您的組織有特定的網路或管理要求，您可能需要配置公司代理或 LLM 網關。這些是可以一起使用的不同配置：

* **公司代理**：通過 HTTP/HTTPS 代理路由流量。如果您的組織要求所有出站流量都通過代理伺服器以進行安全監控、合規性或網路策略執行，請使用此選項。使用 `HTTPS_PROXY` 或 `HTTP_PROXY` 環境變數進行配置。在 [企業網路配置](/docs/zh-TW/network-config) 中了解更多。
* **LLM 網關**：位於 Claude Code 和雲端提供商之間的服務，用於處理身份驗證和路由。如果您需要跨團隊的集中使用追蹤、自訂速率限制或預算，或集中身份驗證管理，請使用此選項。使用 `ANTHROPIC_BASE_URL`、`ANTHROPIC_BEDROCK_BASE_URL`、`ANTHROPIC_AWS_BASE_URL`、`ANTHROPIC_VERTEX_BASE_URL` 或 `ANTHROPIC_FOUNDRY_BASE_URL` 環境變數進行配置。在 [LLM 網關](/docs/zh-TW/llm-gateway) 中了解更多。

如需針對每個提供商的環境變數，以透過 LLM 網關路由 Amazon Bedrock、Microsoft Foundry 或 Google Cloud 的 Agent Platform，請參閱 [透過網關路由到雲端提供商](/docs/zh-TW/llm-gateway-connect#route-to-a-cloud-provider-through-a-gateway)。在 Claude Code 中執行 `/status` 以驗證工作階段使用的提供商、基礎 URL 和代理。

如果您的組織使用 [客戶管理的加密金鑰](https://platform.claude.com/docs/en/manage-claude/cmek) (CMEK)，並透過 LLM 網關或自訂 `ANTHROPIC_BASE_URL` 路由 Claude Code，CMEK 不適用於這些工作階段上 Claude Code 的操作遙測。若要為每位開發人員關閉遙測，請透過受管設定傳遞 `DISABLE_TELEMETRY`，如 [為您的組織關閉遙測](/docs/zh-TW/managed-settings#turn-telemetry-off-for-your-organization) 中所示。

<h2 id="best-practices-for-organizations">
  組織的最佳實踐
</h2>

<h3 id="invest-in-documentation-and-memory">
  投資於文件和記憶
</h3>

我們強烈建議投資於文件，以便 Claude Code 能夠理解您的程式碼庫。組織可以在多個層級部署 CLAUDE.md 檔案。請參閱[CLAUDE.md 檔案可以存放的位置](/docs/zh-TW/memory#choose-where-to-put-claude-md-files)和[如何部署組織範圍的 CLAUDE.md](/docs/zh-TW/memory#deploy-organization-wide-claude-md)。

<h3 id="simplify-deployment">
  簡化部署
</h3>

如果您有自訂開發環境，我們發現建立「一鍵」安裝 Claude Code 的方式是在組織中推動採用的關鍵。

<h3 id="start-with-guided-usage">
  從引導式使用開始
</h3>

鼓勵新使用者嘗試使用 Claude Code 進行程式碼庫問答，或用於較小的錯誤修復或功能請求。要求 Claude Code 制定計畫。檢查 Claude 的建議，如果偏離軌道，請提供回饋。隨著時間推移，當使用者更好地理解這種新範例時，他們將更有效地讓 Claude Code 以更多代理方式運行。

<h3 id="pin-model-versions-for-cloud-providers">
  為雲端提供商固定模型版本
</h3>

如果您透過 [Amazon Bedrock](/docs/zh-TW/amazon-bedrock)、[Google Cloud 的 Agent Platform](/docs/zh-TW/google-vertex-ai)、[Microsoft Foundry](/docs/zh-TW/microsoft-foundry) 或 [Claude Platform on AWS](/docs/zh-TW/claude-platform-on-aws) 進行部署，請使用 `ANTHROPIC_DEFAULT_FABLE_MODEL`、`ANTHROPIC_DEFAULT_OPUS_MODEL`、`ANTHROPIC_DEFAULT_SONNET_MODEL` 和 `ANTHROPIC_DEFAULT_HAIKU_MODEL` 固定特定模型版本。如果不固定，模型別名會解析為 Claude Code 針對該提供商的內建預設值，這可能會滯後於最新版本，且可能尚未在您的帳戶中啟用。固定版本可讓您控制使用者何時移至新模型。請參閱[模型設定](/docs/zh-TW/model-config#pin-models-for-third-party-deployments)，瞭解當預設值不可用時每個提供商的做法。

<h3 id="configure-security-policies">
  設定安全性原則
</h3>

安全團隊可以設定受管權限，以決定 Claude Code 允許和不允許執行的操作，這些權限無法被本機設定覆寫。[瞭解更多](/docs/zh-TW/security)。

<h3 id="leverage-mcp-for-integrations">
  使用 MCP 進行整合
</h3>

MCP 是為 Claude Code 提供更多資訊的絕佳方式，例如連接到票證管理系統或錯誤日誌。我們建議由一個中央團隊設定 MCP 伺服器，並將 `.mcp.json` 設定檔簽入程式碼庫，以便所有使用者受益。[瞭解更多](/docs/zh-TW/mcp)。

<h2 id="next-steps">
  後續步驟
</h2>

選擇部署選項並為您的團隊配置存取權限後：

1. **向您的團隊推出**：分享安裝說明，並讓團隊成員 [安裝 Claude Code](/docs/zh-TW/setup) 並使用其認證進行身份驗證。
2. **設置共享配置**：在您的存儲庫中建立 [CLAUDE.md 文件](/docs/zh-TW/memory)，以幫助 Claude Code 理解您的程式碼庫和編碼標準。
3. **配置權限**：查看 [安全設置](/docs/zh-TW/security)，以定義 Claude Code 在您的環境中可以和不能執行的操作。
