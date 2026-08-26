> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Google Cloud 的 Agent Platform 上的 Claude Code

> 了解如何透過 Google Cloud 的 Agent Platform（前身為 Vertex AI）設定 Claude Code，包括設定、IAM 設定和故障排除。

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

<ContactSalesCard surface="vertex" />

<h2 id="prerequisites">
  先決條件
</h2>

在使用 Google Cloud 的 Agent Platform（前身為 Vertex AI）設定 Claude Code 之前，請確保您具有：

* 已啟用計費的 Google Cloud Platform (GCP) 帳戶
* 已啟用 Google Cloud 的 Agent Platform API 的 GCP 專案
* 存取所需的 Claude 模型（例如 Claude Sonnet 4.6）
* 已安裝並設定 Google Cloud SDK (`gcloud`)
* 在所需的 GCP 區域中分配的配額

若要使用您自己的 Google Cloud 的 Agent Platform 認證登入，請遵循下方的[使用 Google Cloud 的 Agent Platform 登入](#sign-in-with-agent-platform)。若要在整個團隊中部署 Claude Code，請使用[手動設定](#set-up-manually)步驟並在推出前[固定您的模型版本](#5-pin-model-versions)。

<h2 id="sign-in-with-agent-platform">
  使用 Agent Platform 登入
</h2>

如果您有 Google Cloud 認證並想開始透過 Google Cloud 的 Agent Platform 使用 Claude Code，登入精靈會引導您完成整個過程。您只需在每個專案中完成一次 GCP 端的先決條件；精靈會處理 Claude Code 端的設定。

<Steps>
  <Step title="在您的 GCP 專案中啟用 Claude 模型">
    為您的專案[啟用 Google Cloud 的 Agent Platform API](#1-enable-agent-platform-api)，然後在 [Google Cloud 的 Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) 中要求存取您想要的 Claude 模型。請參閱 [IAM 設定](#iam-configuration)以了解您的帳戶需要的權限。
  </Step>

  <Step title="啟動 Claude Code 並選擇 Google Cloud 的 Agent Platform">
    執行 `claude`。在登入提示處，選擇**第三方平台**，然後選擇 **Google Vertex AI**，這是登入提示仍然用於 Google Cloud 的 Agent Platform 的標籤。
  </Step>

  <Step title="遵循精靈提示">
    選擇您如何向 Google Cloud 進行驗證：來自 `gcloud` 的應用程式預設認證、服務帳戶金鑰檔案，或已在您的環境中的認證。精靈會偵測您的專案和區域，驗證您的專案可以呼叫哪些 Claude 模型，並讓您固定它們。它會將結果儲存到您的[使用者設定檔](/docs/zh-TW/settings)的 `env` 區塊，因此您不需要自己匯出環境變數。
  </Step>
</Steps>

登入後，您可以隨時執行 `/setup-vertex` 以重新開啟精靈並變更您的認證、專案、區域或模型固定。模型固定步驟會從您目前固定的模型開始。精靈會寫入 `~/.claude/settings.json`，或在設定 [`CLAUDE_CONFIG_DIR`](/docs/zh-TW/env-vars#variables) 時寫入 `$CLAUDE_CONFIG_DIR/settings.json`。

<h2 id="region-configuration">
  區域設定
</h2>

Claude Code 支援 Google Cloud 的 Agent Platform [全球](https://cloud.google.com/blog/products/ai-machine-learning/global-endpoint-for-claude-models-generally-available-on-vertex-ai)、多區域和區域端點。將 `CLOUD_ML_REGION` 設定為 `global`、多區域位置（例如 `eu` 或 `us`）或特定區域（例如 `us-east5`）。Claude Code 為每種形式選擇正確的 Google Cloud 的 Agent Platform 主機名稱，包括多區域位置的 `aiplatform.eu.rep.googleapis.com` 和 `aiplatform.us.rep.googleapis.com` 主機。

<Note>
  Google Cloud 的 Agent Platform 可能不支援每個端點類型上的 Claude Code 預設模型。模型可用性在[特定區域](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations#genai-partner-models)、多區域位置和[全球端點](https://cloud.google.com/vertex-ai/generative-ai/docs/partner-models/use-partner-models#supported_models)之間有所不同。您可能需要切換到支援的位置或指定支援的模型。
</Note>

<h2 id="set-up-manually">
  手動設定
</h2>

若要透過環境變數而不是精靈來設定 Google Cloud 的 Agent Platform，例如在 CI 或指令碼化企業推出中，請遵循下列步驟。

<h3 id="1-enable-agent-platform-api">
  1. 啟用 Agent Platform API
</h3>

在您的 GCP 專案中啟用 Google Cloud 的 Agent Platform API：

```bash theme={null}
# 設定您的專案 ID
gcloud config set project YOUR-PROJECT-ID

# 啟用 Agent Platform API
gcloud services enable aiplatform.googleapis.com
```

<h3 id="2-request-model-access">
  2. 要求模型存取
</h3>

在 Google Cloud 的 Agent Platform 中要求存取 Claude 模型：

1. 導覽至 [Google Cloud 的 Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden)
2. 搜尋'Claude'模型
3. 要求存取所需的 Claude 模型（例如 Claude Sonnet 4.6）
4. 等待核准（可能需要 24-48 小時）

<h3 id="3-configure-gcp-credentials">
  3) 設定 GCP 認證
</h3>

Claude Code 使用標準的 Google Cloud 驗證。

如需詳細資訊，請參閱 [Google Cloud 驗證文件](https://cloud.google.com/docs/authentication)。

Claude Code v2.1.121 或更新版本透過相同的 Application Default Credentials 鏈支援 [X.509 憑證型 Workload Identity Federation](https://cloud.google.com/iam/docs/workload-identity-federation-with-x509-certificates)。將 `GOOGLE_APPLICATION_CREDENTIALS` 設定為您的認證設定檔案路徑。

<Note>
  Claude Code 使用 `ANTHROPIC_VERTEX_PROJECT_ID` 作為 Google Cloud 的 Agent Platform 要求的專案 ID。`GCLOUD_PROJECT` 和 `GOOGLE_CLOUD_PROJECT` 環境變數以及 `GOOGLE_APPLICATION_CREDENTIALS` 參考的認證檔案優先於它。如果這些都未設定，專案 ID 會從您的 `gcloud` 設定或附加的服務帳戶解析。
</Note>

<h4 id="advanced-credential-configuration">
  進階認證設定
</h4>

Claude Code 透過 `gcpAuthRefresh` 設定支援 GCP 的自動認證重新整理。當 Claude Code 偵測到您的 GCP 認證已過期或無法載入時，它會執行設定的命令以在重試要求之前取得新認證。

```json theme={null}
{
  "gcpAuthRefresh": "gcloud auth application-default login",
  "env": {
    "ANTHROPIC_VERTEX_PROJECT_ID": "your-project-id"
  }
}
```

命令的輸出會顯示給使用者，但不支援互動式輸入。這適用於瀏覽器型驗證流程，其中 CLI 顯示 URL，您在瀏覽器中完成驗證。如果驗證未在三分鐘內完成，重新整理命令會逾時。如果您在專案設定（例如 `.claude/settings.json`）中設定 `gcpAuthRefresh`，命令只會在您接受工作區信任提示後執行。

<h3 id="4-configure-claude-code">
  4. 設定 Claude Code
</h3>

設定下列環境變數：

```bash theme={null}
# 啟用 Agent Platform 整合
export CLAUDE_CODE_USE_VERTEX=1
export CLOUD_ML_REGION=global
export ANTHROPIC_VERTEX_PROJECT_ID=YOUR-PROJECT-ID

# 選用：覆寫 Agent Platform 端點 URL 以用於自訂端點或閘道
# export ANTHROPIC_VERTEX_BASE_URL=https://aiplatform.googleapis.com

# 選用：如需要，停用 prompt caching
export DISABLE_PROMPT_CACHING=1

# 選用：要求 1 小時 prompt cache TTL 而不是 5 分鐘預設值
export ENABLE_PROMPT_CACHING_1H=1

# 當 CLOUD_ML_REGION=global 時，覆寫不支援全球端點的模型的區域
export VERTEX_REGION_CLAUDE_HAIKU_4_5=us-east5
export VERTEX_REGION_CLAUDE_4_6_SONNET=europe-west1
```

大多數模型版本都有對應的 `VERTEX_REGION_CLAUDE_*` 變數。如需完整清單，請參閱[環境變數參考](/docs/zh-TW/env-vars)。檢查 [Google Cloud 的 Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) 以確定哪些模型支援全球端點與僅限區域端點。

[Prompt caching](/docs/zh-TW/prompt-caching) 會自動啟用。若要停用它，請設定 `DISABLE_PROMPT_CACHING=1`。若要要求 1 小時 cache TTL 而不是 5 分鐘預設值，請設定 `ENABLE_PROMPT_CACHING_1H=1`；具有 1 小時 TTL 的 cache 寫入會以更高費率計費。如需提高速率限制，請聯絡 Google Cloud 支援。使用 Google Cloud 的 Agent Platform 時，`/logout` 命令會被停用，因為驗證是透過 Google Cloud 認證處理的。

Claude Code 在 Google Cloud 的 Agent Platform 上預設停用 [MCP tool search](/docs/zh-TW/mcp#scale-with-mcp-tool-search)，因此 MCP 工具定義會預先載入。Google Cloud 的 Agent Platform 支援 Claude Sonnet 4.5 及更新版本以及 Claude Opus 4.5 及更新版本的工具搜尋。設定 `ENABLE_TOOL_SEARCH=true` 以在這些模型上啟用它。Google Cloud 的 Agent Platform 上的較早模型不接受所需的 beta 標頭，如果您使用它們啟用工具搜尋，要求會失敗。

<h3 id="5-pin-model-versions">
  5. 固定模型版本
</h3>

<Warning>
  在部署到多個使用者時固定特定模型版本。如果不固定，模型別名（例如 `sonnet` 和 `opus`）會解析為 Claude Code 針對 Google Cloud 的 Agent Platform 的內建預設值，該預設值可能落後於最新版本，且可能尚未在您的專案中啟用。Claude Code 在啟動時會在預設值無法使用時[回退](#startup-model-checks)到先前版本或較低階層模型，但固定可讓您控制使用者何時移至新模型。
</Warning>

將這些環境變數設定為特定的 Google Cloud 的 Agent Platform 模型 ID。

如果沒有 `ANTHROPIC_DEFAULT_OPUS_MODEL`，Google Cloud 的 Agent Platform 上的 `opus` 別名會解析為 Opus 4.8，如果沒有 `ANTHROPIC_DEFAULT_SONNET_MODEL`，`sonnet` 別名會解析為 Sonnet 4.5。此範例將每個別名固定為特定版本：

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='claude-sonnet-5'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

如需目前和舊版模型 ID，請參閱[模型概覽](https://platform.claude.com/docs/en/about-claude/models/overview)。如需完整的環境變數清單，請參閱[模型設定](/docs/zh-TW/model-config#pin-models-for-third-party-deployments)。

Claude Code 使用這些預設模型，當未設定固定變數時：

| 模型類型    | 預設值                          |
| :------ | :--------------------------- |
| 主要模型    | `claude-opus-4-8`            |
| 小型/快速模型 | `claude-sonnet-4-5@20250929` |

背景工作（例如工作階段標題產生）使用小型/快速模型，通常是 Haiku 級模型。在 Google Cloud 的 Agent Platform 上，Claude Code 預設為主要 Sonnet 模型進行背景工作，因為 Haiku 可能不會在每個專案或區域中啟用。有兩個選項會變更哪個模型執行這些工作：

* 當您使用 `--model`、`ANTHROPIC_MODEL` 或 `model` 設定選擇主要模型時，背景工作會使用該模型。設定 `ANTHROPIC_DEFAULT_OPUS_MODEL` 而不設定 `ANTHROPIC_DEFAULT_SONNET_MODEL` 也算是一個選擇，因為內建 Sonnet 模型可能在引導自己的 Opus 的專案中未啟用。
* 若要使用 Haiku 進行背景工作，請將 `ANTHROPIC_DEFAULT_HAIKU_MODEL` 設定為您專案中可用的模型 ID。

<Warning>
  Opus 模型的每個 token 價格高於 Sonnet 模型，因此不固定主要模型的部署在更新到 v2.1.207 或更新版本後會以 Opus 費率計費。若要保持 Sonnet 4.5 作為主要模型，請將 `ANTHROPIC_MODEL` 設定為其完整模型 ID。使用 `ANTHROPIC_DEFAULT_SONNET_MODEL` 引導預設值且未設定 `ANTHROPIC_DEFAULT_OPUS_MODEL` 的部署會保持其引導的 Sonnet 模型作為預設值。
</Warning>

在 v2.1.207 之前，Google Cloud 的 Agent Platform 上的主要模型預設為 Sonnet 4.5，`opus` 別名解析為 Opus 4.6，背景工作始終使用主要模型。

若要進一步自訂模型：

```bash theme={null}
export ANTHROPIC_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

<h2 id="startup-model-checks">
  啟動模型檢查
</h2>

當 Claude Code 以 Google Cloud 的 Agent Platform 設定啟動時，它會驗證它打算使用的模型在您的專案中是否可存取。

如果您已固定的模型版本比目前 Claude Code 預設值更舊，且您的專案可以呼叫較新版本，Claude Code 會提示您更新固定。接受會將新模型 ID 寫入您的[使用者設定檔](/docs/zh-TW/settings)並重新啟動 Claude Code。拒絕會被記住，直到下一次預設版本變更。

如果您尚未固定模型，且目前預設值在您的專案中無法使用，Claude Code 會在目前工作階段中回退並顯示通知。它會先嘗試預設模型的較早版本，當預設為 Opus 模型且沒有可用的 Opus 版本時，會回退到預設 Sonnet 模型。回退不會被保留。在 [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) 中啟用較新模型或[固定版本](#5-pin-model-versions)以使選擇永久化。

<h2 id="iam-configuration">
  IAM 設定
</h2>

指派所需的 IAM 權限：

`roles/aiplatform.user` 角色包含所需的權限：

* `aiplatform.endpoints.predict` - 模型呼叫和權杖計數所需

如需更嚴格的權限，請建立只包含上述權限的自訂角色。

如需詳細資訊，請參閱 [Google Cloud 的 Agent Platform IAM 文件](https://cloud.google.com/vertex-ai/docs/general/access-control)。

<Note>
  為 Claude Code 建立專用的 GCP 專案，以簡化成本追蹤和存取控制。
</Note>

<h2 id="1m-token-context-window">
  1M token context window
</h2>

Claude Sonnet 5、Opus 4.6 及更新版本，以及 Sonnet 4.6，在 Google Cloud 的 Agent Platform 上支援 [1M token context window](https://platform.claude.com/docs/zh-TW/build-with-claude/context-windows#context-window-sizes-by-model)。Sonnet 5 始終以 1M 視窗執行，沒有 `[1m]` 變體可選擇。對於其他模型，Claude Code 會在您選擇 1M 模型變體時自動啟用擴展 context window。

[設定精靈](#sign-in-with-agent-platform)在固定模型時提供 1M context 選項。若要為手動固定的模型啟用它，請在模型 ID 後附加 `[1m]`。如需詳細資訊，請參閱[為第三方部署固定模型](/docs/zh-TW/model-config#pin-models-for-third-party-deployments)。

<h2 id="troubleshooting">
  故障排除
</h2>

如果您遇到「無法載入預設認證」錯誤：

* 執行 `gcloud auth application-default login` 以設定應用程式預設認證
* 將 `GOOGLE_APPLICATION_CREDENTIALS` 設定為服務帳戶金鑰檔案路徑
* 請參閱 [設定 GCP 認證](#3-configure-gcp-credentials) 以了解所有選項

如果您遇到配額問題：

* 透過 [Cloud Console](https://cloud.google.com/docs/quotas/view-manage) 檢查目前配額或要求增加配額

如果您遇到「找不到模型」404 錯誤：

* 確認模型在 [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) 中已啟用
* 驗證模型在您指定的位置中可用。某些模型僅在 `global` 或多區域位置（例如 `eu` 和 `us`）上提供，不在特定區域中
* 如果使用 `CLOUD_ML_REGION=global`，請檢查您的模型是否在 [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) 中的「支援的功能」下支援全球端點。對於不支援全球端點的模型，請執行下列其中一項：
  * 透過 `ANTHROPIC_MODEL` 或 `ANTHROPIC_DEFAULT_HAIKU_MODEL` 指定支援的模型，或
  * 使用 `VERTEX_REGION_<MODEL_NAME>` 環境變數設定區域或多區域位置

如果您遇到 429 錯誤：

* 對於區域端點，請確保主要模型和小型/快速模型在您選擇的區域中受支援
* 考慮切換到 `CLOUD_ML_REGION=global` 以獲得更好的可用性

<h2 id="additional-resources">
  其他資源
</h2>

* [Google Cloud 的 Agent Platform 文件](https://cloud.google.com/vertex-ai/docs)
* [Google Cloud 的 Agent Platform 定價](https://cloud.google.com/vertex-ai/pricing)
* [Google Cloud 的 Agent Platform 配額和限制](https://cloud.google.com/vertex-ai/docs/quotas)
