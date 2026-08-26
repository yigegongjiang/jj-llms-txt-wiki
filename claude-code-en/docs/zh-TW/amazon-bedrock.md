> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Amazon Bedrock 上的 Claude Code

> 了解如何透過 Amazon Bedrock 設定 Claude Code，包括設定、IAM 設定和故障排除。

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

<ContactSalesCard surface="bedrock" />

<h2 id="prerequisites">
  先決條件
</h2>

在使用 Amazon Bedrock 設定 Claude Code 之前，請確保您具有：

* 已啟用 Amazon Bedrock 存取的 AWS 帳戶
* 在 Amazon Bedrock 中存取所需的 Claude 模型（例如 Claude Sonnet 4.6）
* 已安裝並設定 AWS CLI（選用 - 僅在您沒有其他取得認證機制時才需要）
* 適當的 IAM 權限

若要使用您自己的 Amazon Bedrock 認證登入，請遵循下面的[使用 Amazon Bedrock 登入](#sign-in-with-bedrock)。若要在整個團隊中部署 Claude Code，請使用[手動設定](#set-up-manually)步驟並在推出前[固定您的模型版本](#4-pin-model-versions)。

<h2 id="sign-in-with-bedrock">
  使用 Bedrock 登入
</h2>

如果您有 AWS 認證並想開始透過 Amazon Bedrock 使用 Claude Code，登入精靈會引導您完成整個過程。您每個帳戶完成一次 AWS 端的先決條件；精靈會處理 Claude Code 端。

<Steps>
  <Step title="在您的 AWS 帳戶中啟用 Anthropic 模型">
    在 [Amazon Bedrock 主控台](https://console.aws.amazon.com/bedrock/)中，開啟模型目錄，選取 Anthropic 模型，然後提交使用案例表單。提交後立即授予存取權限。請參閱[提交使用案例詳細資訊](#1-submit-use-case-details)以了解 AWS Organizations，以及[IAM 設定](#iam-configuration)以了解您的角色所需的權限。
  </Step>

  <Step title="啟動 Claude Code 並選擇 Amazon Bedrock">
    執行 `claude`。在登入提示處，選取**第三方平台**，然後選取 **Amazon Bedrock**。
  </Step>

  <Step title="遵循精靈提示">
    選擇您如何向 AWS 進行驗證：從您的 `~/.aws` 目錄偵測到的 AWS 設定檔、Amazon Bedrock API 金鑰、存取金鑰和密碼，或已在您的環境中的認證。精靈會選取您的區域，驗證您的帳戶可以叫用哪些 Claude 模型，並讓您固定它們。它會將結果儲存到您的[使用者設定檔](/docs/zh-TW/settings)的 `env` 區塊，因此您不需要自己匯出環境變數。
  </Step>
</Steps>

登入後，隨時執行 `/setup-bedrock` 以重新開啟精靈並變更您的認證、區域或模型固定。模型固定步驟從您目前固定的模型開始。精靈會寫入 `~/.claude/settings.json`，或在設定 [`CLAUDE_CONFIG_DIR`](/docs/zh-TW/env-vars#variables) 時寫入 `$CLAUDE_CONFIG_DIR/settings.json`。

<h2 id="set-up-manually">
  手動設定
</h2>

若要透過環境變數而不是精靈來設定 Amazon Bedrock，例如在 CI 或指令碼化企業推出中，請遵循下面的步驟。

<h3 id="1-submit-use-case-details">
  1. 提交使用案例詳細資訊
</h3>

Anthropic 模型的首次使用者必須在叫用模型之前提交使用案例詳細資訊。這是每個 AWS 帳戶執行一次的操作。

1. 確保您具有下面所述的正確 IAM 權限
2. 導覽至 [Amazon Bedrock 主控台](https://console.aws.amazon.com/bedrock/)
3. 從**模型目錄**選取 Anthropic 模型
4. 完成使用案例表單。提交後立即授予存取權限。

如果您使用 AWS Organizations，您可以使用 [`PutUseCaseForModelAccess` API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_PutUseCaseForModelAccess.html) 從管理帳戶提交一次表單。此呼叫需要 `bedrock:PutUseCaseForModelAccess` IAM 權限。核准會自動延伸到子帳戶。

<h3 id="2-configure-aws-credentials">
  2. 設定 AWS 認證
</h3>

Claude Code 使用預設的 AWS SDK 認證鏈。使用以下其中一種方法設定您的認證：

**選項 A：AWS CLI 設定**

```bash theme={null}
aws configure
```

**選項 B：環境變數（存取金鑰）**

```bash theme={null}
export AWS_ACCESS_KEY_ID=your-access-key-id
export AWS_SECRET_ACCESS_KEY=your-secret-access-key
export AWS_SESSION_TOKEN=your-session-token
```

**選項 C：環境變數（SSO 設定檔）**

在執行這些命令之前，將 `your-profile-name` 替換為您的 AWS 設定檔名稱。

```bash theme={null}
aws sso login --profile=your-profile-name

export AWS_PROFILE=your-profile-name
```

Claude Code 從設定檔的 `sso_region` 命名的 IAM Identity Center 區域要求角色認證，這不需要與您執行 Amazon Bedrock 的區域相符。在 v2.1.207 中，Amazon Bedrock 區域覆寫了 `sso_region`，因此 IAM Identity Center 執行個體在不同區域的設定檔無法使用 `Session token not found or invalid` 錯誤進行驗證。

**選項 D：AWS 管理主控台認證**

```bash theme={null}
aws login
```

[深入了解](https://docs.aws.amazon.com/signin/latest/userguide/command-line-sign-in.html) `aws login`。

**選項 E：Amazon Bedrock API 金鑰**

```bash theme={null}
export AWS_BEARER_TOKEN_BEDROCK=your-bedrock-api-key
```

Amazon Bedrock API 金鑰提供了一種更簡單的驗證方法，無需完整的 AWS 認證。[深入了解 Amazon Bedrock API 金鑰](https://aws.amazon.com/blogs/machine-learning/accelerate-ai-development-with-amazon-bedrock-api-keys/)。

<h4 id="credential-caching-and-resolution-timeout">
  認證快取和解析逾時
</h4>

Claude Code 解析 AWS 預設認證提供者鏈一次，並將已解析的認證保留在記憶體中。它會重複使用它們，直到它們過期前五分鐘，或在沒有過期時間時使用一小時，因此 SSO 支援的設定檔大約每個認證生命週期從 IAM Identity Center 要求一次認證。來自 API 的認證錯誤會清除快取，重試會解析新認證。

在 v2.1.207 之前，Claude Code 在每個 API 要求時解析鏈，因此 SSO 支援的設定檔每次都從 IAM Identity Center 要求新認證，在大型部署中可能會被節流。

快取涵蓋上面的每個認證選項，除了 Amazon Bedrock API 金鑰，它不使用提供者鏈。若要改為在每個要求時解析鏈，請設定 [`CLAUDE_CODE_SKIP_AWS_CRED_CACHE=1`](/docs/zh-TW/env-vars)。

鏈的每次解析在 60 秒後逾時。如果鏈中的步驟停滯，例如等待無法接收的輸入的 `credential_process` 協助程式，要求會失敗，並出現 [`AWS default-chain credential resolve timed out`](/docs/zh-TW/errors#aws-default-chain-credential-resolve-timed-out)。如果您的鏈執行合法需要更長時間的互動式登入，例如透過 `aws-vault` 之類的包裝程式進行瀏覽器型 SSO 搭配 MFA，請使用 [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/zh-TW/env-vars) 以毫秒為單位提高限制。在 v2.1.207 之前，停滯的認證解析會使要求無限期等待。

<h4 id="advanced-credential-configuration">
  進階認證設定
</h4>

Claude Code 支援 AWS SSO 和公司身分提供者的自動認證重新整理。將這些設定新增至您的 Claude Code 設定檔（請參閱[設定](/docs/zh-TW/settings)以了解檔案位置）。

這兩個設定有不同的觸發條件：

* **`awsAuthRefresh`**：僅在 Claude Code 偵測到您的 AWS 認證已過期時執行，基於本機時間戳記或當 API 傳回認證錯誤時，然後使用重新整理的認證重試請求。
* **`awsCredentialExport`**：在工作階段開始時和每次認證重新載入時執行，即使您的 AWS 預設認證提供者鏈中的認證仍然有效。當您的 Amazon Bedrock 帳戶需要與預設提供者鏈會解析的認證不同的跨帳戶認證時，請使用此選項。

<h5 id="example-configuration">
  範例設定
</h5>

```json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile myprofile",
  "env": {
    "AWS_PROFILE": "myprofile"
  }
}
```

<h5 id="configuration-settings-explained">
  設定說明
</h5>

**`awsAuthRefresh`**：用於修改 `.aws` 目錄的命令，例如更新認證、SSO 快取或設定檔。命令的輸出會顯示給使用者，但不支援互動式輸入。這適用於瀏覽器型 SSO 流程，其中 CLI 顯示 URL 或代碼，您在瀏覽器中完成驗證。

**`awsCredentialExport`**：僅在您無法修改 `.aws` 且必須直接傳回認證時使用。此命令在每次需要重新整理認證時執行，而不僅在認證過期時執行。輸出會被無聲地擷取，不會顯示給使用者。命令必須以此格式輸出 JSON：

```json theme={null}
{
  "Credentials": {
    "AccessKeyId": "value",
    "SecretAccessKey": "value",
    "SessionToken": "value",
    "Expiration": "2026-01-01T00:00:00Z"
  }
}
```

自 Claude Code v2.1.181 起，`aws configure export-credentials --format process` 的平面輸出也被接受，具有相同的金鑰在頂層而不是巢狀在 `Credentials` 下。

`Expiration` 是選用的。自 Claude Code v2.1.176 起，當命令傳回有效的 ISO 8601 `Expiration` 時，Claude Code 會快取認證直到該時間前五分鐘。沒有它，或在較早版本上，認證會快取一小時。

當您設定 `awsCredentialExport` 而不設定 `awsAuthRefresh` 時，Claude Code 會直接使用匯出的認證，不會在啟動時重新解析 AWS 預設認證提供者鏈。在 v2.1.206 之前，啟動也會重新解析預設提供者鏈，這會在您的代理設定之外進行即時 SSO 或 STS 呼叫，並可能在具有受限出口的網路上阻止第一個提示數分鐘。

<h3 id="3-configure-claude-code">
  3. 設定 Claude Code
</h3>

設定下列環境變數以啟用 Amazon Bedrock：

```bash theme={null}
# 啟用 Bedrock 整合
export CLAUDE_CODE_USE_BEDROCK=1
export AWS_REGION=us-east-1  # 如果您的 AWS 設定檔已設定區域，則為選用；如果您的設定檔沒有區域，則為必需

# 選用：覆寫小型/快速模型 (Bedrock 和 Mantle) 的 AWS 區域。
# 在 Bedrock 上，如果沒有設定 ANTHROPIC_DEFAULT_HAIKU_MODEL
# 或已棄用的 ANTHROPIC_SMALL_FAST_MODEL，則無效。
export ANTHROPIC_SMALL_FAST_MODEL_AWS_REGION=us-west-2

# 選用：覆寫 Bedrock 端點 URL 以用於自訂端點或閘道
# export ANTHROPIC_BEDROCK_BASE_URL=https://bedrock-runtime.us-east-1.amazonaws.com
```

為 Claude Code 啟用 Amazon Bedrock 時，請記住以下事項：

* 自 v2.1.172 起，您只需設定 `AWS_REGION` 以覆寫您的 AWS 設定檔的區域，或在您的設定檔沒有區域時設定。Claude Code 按此順序解析區域：

  * `AWS_REGION`
  * `AWS_DEFAULT_REGION`
  * 在您的作用中 AWS 設定檔上設定的 `region`，首先從 AWS 共用認證檔案讀取，然後從共用設定檔讀取，符合 AWS SDK 優先順序
  * `us-east-1`

  作用中設定檔是 `AWS_PROFILE`（如果已設定），否則為 `default`。設定 `AWS_SHARED_CREDENTIALS_FILE` 或 `AWS_CONFIG_FILE` 以指向非預設檔案路徑。執行 `/status` 以查看已解析的區域。當區域來自您的 AWS 設定檔或預設回退時，`/status` 也會記錄來源。在 v2.1.171 及更早版本上，Claude Code 不會讀取 AWS 設定檔，因此請明確設定 `AWS_REGION`。
* 使用 Amazon Bedrock 時，`/logout` 命令無法使用，因為驗證是透過 AWS 認證處理的。
* WebSearch 工具在 Amazon Bedrock 上無法使用。請參閱 [WebSearch 工具行為](/docs/zh-TW/tools-reference#websearch-tool-behavior)。
* 您可以使用設定檔來設定環境變數，例如 `AWS_PROFILE`，您不想將其洩露給其他程序。請參閱[設定](/docs/zh-TW/settings)以取得更多資訊。

<h3 id="4-pin-model-versions">
  4. 固定模型版本
</h3>

<Warning>
  在部署給多個使用者時固定特定的模型版本。如果不固定，模型別名（例如 `sonnet` 和 `opus`）會解析為 Claude Code 針對 Amazon Bedrock 的內建預設值，該預設值可能落後於最新版本，且可能在您的帳戶中尚不可用。Claude Code 在啟動時會在預設值無法使用時[回退](#startup-model-checks)到先前版本或較低層級模型，但固定可讓您控制使用者何時移至新模型。
</Warning>

將這些環境變數設定為特定的 Amazon Bedrock 模型 ID。

如果沒有 `ANTHROPIC_DEFAULT_OPUS_MODEL`，Amazon Bedrock 上的 `opus` 別名會解析為 Opus 4.8，如果沒有 `ANTHROPIC_DEFAULT_SONNET_MODEL`，`sonnet` 別名會解析為 Sonnet 4.5。此範例將每個別名固定為特定版本：

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='us.anthropic.claude-sonnet-4-6'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='us.anthropic.claude-haiku-4-5-20251001-v1:0'
```

這些變數使用跨區域推論設定檔 ID（帶有 `us.` 前綴）。如果您使用不同的區域前綴或應用程式推論設定檔，請相應調整。在 AWS GovCloud 區域中，使用 `us-gov.` 前綴。如需目前和舊版模型 ID，請參閱[模型概觀](https://platform.claude.com/docs/en/about-claude/models/overview)。請參閱[模型設定](/docs/zh-TW/model-config#pin-models-for-third-party-deployments)以取得完整的環境變數清單。

Claude Code 使用這些預設模型，當未設定固定變數時：

| 模型類型    | 預設值                                            |
| :------ | :--------------------------------------------- |
| 主要模型    | `us.anthropic.claude-opus-4-8`                 |
| 小型/快速模型 | `us.anthropic.claude-sonnet-4-5-20250929-v1:0` |

背景工作（例如工作階段標題產生）使用小型/快速模型，通常是 Haiku 級模型。在 Amazon Bedrock 上，Claude Code 預設為小型/快速模型使用 Sonnet 模型，因為 Haiku 可能不會在每個帳戶或區域中啟用。兩個選項會變更哪個模型執行它們：

* 當您使用 `--model`、`ANTHROPIC_MODEL` 或 `model` 設定選取主要模型時，背景工作使用該模型。設定 `ANTHROPIC_DEFAULT_OPUS_MODEL` 而不設定 `ANTHROPIC_DEFAULT_SONNET_MODEL` 也算作選項，因為內建 Sonnet 模型可能在引導自己的 Opus 的帳戶中無法啟用。
* 若要使用 Haiku 進行背景工作，請將 `ANTHROPIC_DEFAULT_HAIKU_MODEL` 設定為您帳戶中可用的模型 ID。

<Warning>
  Opus 模型的每權杖價格高於 Sonnet 模型，因此未固定主要模型的部署在更新至 v2.1.207 或更新版本後會以 Opus 費率計費。若要將 Sonnet 4.5 保持為主要模型，請將 `ANTHROPIC_MODEL` 設定為其完整模型 ID。使用 `ANTHROPIC_DEFAULT_SONNET_MODEL` 引導預設值且未設定 `ANTHROPIC_DEFAULT_OPUS_MODEL` 的部署會保持其引導的 Sonnet 模型作為預設值。
</Warning>

在 v2.1.207 之前，Amazon Bedrock 上的主要模型預設為 Sonnet 4.5，`opus` 別名解析為 Opus 4.6，背景工作始終使用主要模型。

若要進一步自訂模型，請使用以下其中一種方法：

```bash theme={null}
# 使用推論設定檔 ID
export ANTHROPIC_MODEL='us.anthropic.claude-sonnet-4-6'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='us.anthropic.claude-haiku-4-5-20251001-v1:0'

# 使用應用程式推論設定檔 ARN
export ANTHROPIC_MODEL='arn:aws:bedrock:us-east-2:your-account-id:application-inference-profile/your-model-id'

# 選用：如果需要，停用 prompt caching
export DISABLE_PROMPT_CACHING=1

# 選用：要求 1 小時 prompt cache TTL 而不是 5 分鐘預設值
export ENABLE_PROMPT_CACHING_1H=1
```

1 小時快取 TTL 的計費費率高於 5 分鐘預設值。請參閱[快取生命週期](/docs/zh-TW/prompt-caching#cache-lifetime)。

<Note>Prompt caching 可能不適用於所有 Amazon Bedrock 區域。如果快取權杖計數保持為零，請檢查 Amazon Bedrock 文件中的[支援的模型、區域和限制](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models)。</Note>

<h4 id="map-each-model-version-to-an-inference-profile">
  將每個模型版本對應至推論設定檔
</h4>

`ANTHROPIC_DEFAULT_*_MODEL` 環境變數為每個模型系列設定一個推論設定檔。如果您的組織需要在 `/model` 選擇器中公開同一系列的多個版本，每個版本都路由到其自己的應用程式推論設定檔 ARN，請改用[設定檔](/docs/zh-TW/settings#settings-files)中的 `modelOverrides` 設定。

此範例將四個 Opus 版本對應至不同的 ARN，以便使用者可以在它們之間切換，而無需繞過您組織的推論設定檔：

```json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-7": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-47-prod",
    "claude-opus-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-46-prod",
    "claude-opus-4-5-20251101": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-45-prod",
    "claude-opus-4-1-20250805": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-41-prod"
  }
}
```

當使用者在 `/model` 中選取其中一個版本時，Claude Code 會使用對應的 ARN 呼叫 Amazon Bedrock。當您透過 `--model` 或 `ANTHROPIC_MODEL` 直接傳遞 Anthropic 模型 ID 時，相同的對應也適用。沒有覆寫的版本會回退到內建的 Amazon Bedrock 模型 ID 或在啟動時發現的任何相符推論設定檔。在 v2.1.200 之前，`--model` 和 `ANTHROPIC_MODEL` 值會直接到達 Amazon Bedrock，而不會通過覆寫對應。請參閱[覆寫每個版本的模型 ID](/docs/zh-TW/model-config#override-model-ids-per-version)，以了解覆寫如何與 `availableModels` 和其他模型設定互動的詳細資訊。

<h2 id="startup-model-checks">
  啟動模型檢查
</h2>

當 Claude Code 以 Amazon Bedrock 設定啟動時，它會驗證它打算使用的模型在您的帳戶中是否可存取。

如果您已固定的模型版本比目前 Claude Code 預設值更舊，且您的帳戶可以叫用較新版本，Claude Code 會提示您更新固定。接受會將新模型 ID 寫入您的[使用者設定檔](/docs/zh-TW/settings)並重新啟動 Claude Code。拒絕會被記住，直到下一次預設版本變更。指向[應用程式推論設定檔 ARN](#map-each-model-version-to-an-inference-profile) 的固定會被跳過，因為這些由您的管理員管理。

如果您尚未固定模型且目前預設值在您的帳戶中不可用，Claude Code 會在目前工作階段中回退並顯示通知。它會先嘗試預設模型的較早版本，當預設值是 Opus 模型且沒有可用的 Opus 版本時，會回退到預設 Sonnet 模型。回退不會被保留。在您的 Amazon Bedrock 帳戶中啟用較新模型或[固定版本](#4-pin-model-versions)以使選擇永久化。

<h2 id="iam-configuration">
  IAM 設定
</h2>

建立具有 Claude Code 所需權限的 IAM 政策：

```json theme={null}
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "AllowModelAndInferenceProfileAccess",
      "Effect": "Allow",
      "Action": [
        "bedrock:InvokeModel",
        "bedrock:InvokeModelWithResponseStream",
        "bedrock:ListInferenceProfiles",
        "bedrock:GetInferenceProfile"
      ],
      "Resource": [
        "arn:aws:bedrock:*:*:inference-profile/*",
        "arn:aws:bedrock:*:*:application-inference-profile/*",
        "arn:aws:bedrock:*:*:foundation-model/*"
      ]
    },
    {
      "Sid": "AllowMarketplaceSubscription",
      "Effect": "Allow",
      "Action": [
        "aws-marketplace:ViewSubscriptions",
        "aws-marketplace:Subscribe"
      ],
      "Resource": "*",
      "Condition": {
        "StringEquals": {
          "aws:CalledViaLast": "bedrock.amazonaws.com"
        }
      }
    }
  ]
}
```

如需更嚴格的權限，您可以將資源限制為特定的推論設定檔 ARN。

`bedrock:GetInferenceProfile` 讓 Claude Code 將[應用程式推論設定檔 ARN](#map-each-model-version-to-an-inference-profile) 解析為其支援的基礎模型，用於為該模型選擇正確的請求形狀。

如果權杖缺少此權限，Claude Code 會透過使用替代形狀重試一次來自動復原，因此請求仍會成功，但每個新模型都會增加額外的往返。授予權限可避免重試。這最常適用於 `AWS_BEARER_TOKEN_BEDROCK` 部署，其中權杖的政策通常比完整 IAM 角色更狹隘。

如需詳細資訊，請參閱 [Amazon Bedrock IAM 文件](https://docs.aws.amazon.com/bedrock/latest/userguide/security-iam.html)。

<Note>
  為 Claude Code 建立專用的 AWS 帳戶，以簡化成本追蹤和存取控制。
</Note>

<h2 id="1m-token-context-window">
  1M 權杖內容視窗
</h2>

Claude Sonnet 5、Opus 4.6 及更新版本，以及 Sonnet 4.6，在 Amazon Bedrock 上支援 [1M 權杖內容視窗](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model)。Sonnet 5 透過 [Mantle 端點](#use-the-mantle-endpoint)提供，且始終以 1M 視窗執行，沒有 `[1m]` 變體可選擇。對於其他模型，當您選取 1M 模型變體時，Claude Code 會自動啟用擴展內容視窗。

[設定精靈](#sign-in-with-bedrock)在固定模型時提供 1M 內容選項。若要為手動固定的模型啟用它，請在模型 ID 後附加 `[1m]`。請參閱[為第三方部署固定模型](/docs/zh-TW/model-config#pin-models-for-third-party-deployments)以取得詳細資訊。

<h2 id="service-tiers">
  服務層級
</h2>

[Amazon Bedrock 服務層級](https://docs.aws.amazon.com/bedrock/latest/userguide/service-tiers-inference.html)可讓您在成本和延遲之間進行權衡。將 `ANTHROPIC_BEDROCK_SERVICE_TIER` 設定為 `default`、`flex` 或 `priority`：

```bash theme={null}
export ANTHROPIC_BEDROCK_SERVICE_TIER=priority
```

Claude Code 在每個請求上將此作為 `X-Amzn-Bedrock-Service-Tier` 標頭傳送。層級可用性因模型和區域而異。保留容量使用[佈建輸送量](https://docs.aws.amazon.com/bedrock/latest/userguide/prov-throughput.html) ARN 作為模型 ID，而不是此設定。

<h2 id="aws-guardrails">
  AWS Guardrails
</h2>

[Amazon Bedrock Guardrails](https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails.html) 可讓您為 Claude Code 實施內容篩選。在 [Amazon Bedrock 主控台](https://console.aws.amazon.com/bedrock/)中建立 Guardrail，發佈版本，然後將 Guardrail 標頭新增至您的[設定檔](/docs/zh-TW/settings)。如果您使用跨區域推論設定檔，請在 Guardrail 上啟用跨區域推論。

範例設定：

```json theme={null}
{
  "env": {
    "ANTHROPIC_CUSTOM_HEADERS": "X-Amzn-Bedrock-GuardrailIdentifier: your-guardrail-id\nX-Amzn-Bedrock-GuardrailVersion: 1"
  }
}
```

<h2 id="use-the-mantle-endpoint">
  使用 Mantle 端點
</h2>

Mantle 是一個 Amazon Bedrock 端點，透過原生 Anthropic API 形狀而不是 Amazon Bedrock Invoke API 提供 Claude 模型。它使用相同的 AWS 認證、IAM 權限和本頁面前面所述的 `awsAuthRefresh` 設定。

<h3 id="enable-mantle">
  啟用 Mantle
</h3>

已設定 AWS 認證後，設定 `CLAUDE_CODE_USE_MANTLE` 以將請求路由到 Mantle 端點：

```bash theme={null}
export CLAUDE_CODE_USE_MANTLE=1
export AWS_REGION=us-east-1
```

Claude Code 從 AWS 區域構造端點 URL。自 v2.1.172 起，區域的解析優先順序與[上面的 Amazon Bedrock](#3-configure-claude-code) 相同；較早的版本僅使用 `AWS_REGION`。若要為自訂端點或閘道覆寫 URL，請設定 `ANTHROPIC_BEDROCK_MANTLE_BASE_URL`。

在 Claude Code 內執行 `/status` 以確認。當 Mantle 處於作用中時，提供者行會顯示 `Amazon Bedrock (Mantle)`。

<h3 id="select-a-mantle-model">
  選取 Mantle 模型
</h3>

Mantle 使用以 `anthropic.` 為前綴且沒有版本尾碼的模型 ID，例如 `anthropic.claude-sonnet-5` 或 `anthropic.claude-haiku-4-5`。您的帳戶可用的模型取決於您的組織已被授予的內容；其他模型 ID 列在來自 AWS 的您的上線材料中。請聯絡您的 AWS 帳戶團隊以要求存取允許清單模型。

使用 `--model` 旗標或 Claude Code 內的 `/model` 設定模型：

```bash theme={null}
claude --model anthropic.claude-haiku-4-5
```

<h3 id="run-mantle-alongside-the-invoke-api">
  與 Invoke API 並行執行 Mantle
</h3>

您在 Mantle 上可用的模型可能不包括您今天使用的每個模型。設定 `CLAUDE_CODE_USE_BEDROCK` 和 `CLAUDE_CODE_USE_MANTLE` 可讓 Claude Code 從同一工作階段呼叫兩個端點。符合 Mantle 格式的模型 ID 會路由到 Mantle，所有其他模型 ID 會進入 Amazon Bedrock Invoke API。

```bash theme={null}
export CLAUDE_CODE_USE_BEDROCK=1
export CLAUDE_CODE_USE_MANTLE=1
```

若要在 `/model` 選擇器中顯示 Mantle 模型，請在[設定檔](/docs/zh-TW/settings)中的 `availableModels` 中列出其 ID。此設定也會將選擇器限制為列出的項目。列出 `anthropic.claude-haiku-4-5` 會從選擇器中移除裸 `haiku` 別名，因此也請列出版本前綴或您想保持可選的版本的完整 ID。Mantle ID 和 `haiku` 別名會解析為相同的模型系列，因此合併只會保留更具體的項目。請參閱[合併行為](/docs/zh-TW/model-config#merge-behavior)：

```json theme={null}
{
  "availableModels": ["opus", "sonnet", "claude-haiku-4-5", "anthropic.claude-haiku-4-5"]
}
```

帶有 `anthropic.` 前綴的項目會新增為自訂選擇器選項並路由到 Mantle。將 `anthropic.claude-haiku-4-5` 替換為您的帳戶已被授予的模型 ID。請參閱[限制模型選擇](/docs/zh-TW/model-config#restrict-model-selection)以了解 `availableModels` 如何與其他模型設定互動。

當兩個提供者都處於作用中時，`/status` 會顯示 `Amazon Bedrock + Amazon Bedrock (Mantle)`。

<h3 id="route-mantle-through-a-gateway">
  透過閘道路由 Mantle
</h3>

如果您的組織透過集中式 [LLM 閘道](/docs/zh-TW/llm-gateway)路由模型流量，該閘道在伺服器端注入 AWS 認證，請停用用戶端驗證，以便 Claude Code 傳送沒有 SigV4 簽名或 `x-api-key` 標頭的請求：

```bash theme={null}
export CLAUDE_CODE_USE_MANTLE=1
export CLAUDE_CODE_SKIP_MANTLE_AUTH=1
export ANTHROPIC_BEDROCK_MANTLE_BASE_URL=https://your-gateway.example.com
```

<h3 id="mantle-environment-variables">
  Mantle 環境變數
</h3>

這些變數特定於 Mantle 端點。請參閱[環境變數](/docs/zh-TW/env-vars)以取得完整清單。

| 變數                                      | 目的                                        |
| :-------------------------------------- | :---------------------------------------- |
| `CLAUDE_CODE_USE_MANTLE`                | 啟用 Mantle 端點。設定為 `1` 或 `true`。            |
| `ANTHROPIC_BEDROCK_MANTLE_BASE_URL`     | 覆寫預設 Mantle 端點 URL                        |
| `CLAUDE_CODE_SKIP_MANTLE_AUTH`          | 跳過用戶端驗證以進行代理設定                            |
| `ANTHROPIC_SMALL_FAST_MODEL_AWS_REGION` | 覆寫 Haiku 級模型的 AWS 區域（與 Amazon Bedrock 共用） |

<h2 id="troubleshooting">
  故障排除
</h2>

<h3 id="authentication-loop-with-sso-and-corporate-proxies">
  使用 SSO 和公司代理的驗證迴圈
</h3>

如果在使用 AWS SSO 時瀏覽器標籤頻繁開啟，請從您的[設定檔](/docs/zh-TW/settings)中移除 `awsAuthRefresh` 設定。這可能發生在公司 VPN 或 TLS 檢查代理中斷 SSO 瀏覽器流程時。Claude Code 將中斷的連線視為驗證失敗，重新執行 `awsAuthRefresh`，並無限迴圈。

如果您的網路環境干擾自動瀏覽器型 SSO 流程，請在啟動 Claude Code 之前手動使用 `aws sso login`，而不是依賴 `awsAuthRefresh`。

<h3 id="region-issues">
  區域問題
</h3>

如果您遇到區域問題：

* 檢查模型可用性：`aws bedrock list-inference-profiles --region your-region`
* 切換至支援的區域：`export AWS_REGION=us-east-1`
* 考慮使用推論設定檔進行跨區域存取

如果您收到「不支援隨需輸送量」的錯誤：

* 將模型指定為[推論設定檔](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html) ID

Claude Code 使用 Amazon Bedrock [Invoke API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModelWithResponseStream.html)，不支援 Converse API。

<h3 id="streaming-errors-behind-a-gateway-or-proxy">
  閘道或代理後的串流錯誤
</h3>

如果串流請求失敗，並出現以 `Bedrock streaming response has content-type` 開頭的錯誤，則 Claude Code 和 Amazon Bedrock 之間的閘道或代理正在轉換串流回應。Amazon Bedrock 以二進位事件串流格式串流回應，內容類型為 `application/vnd.amazon.eventstream`，而 Claude Code 會拒絕報告不同內容類型的成功串流回應，而不是解碼它無法讀取的主體。該錯誤會命名它收到的內容類型，通常是來自 Amazon API Gateway 和 Lambda 整合的 `text/event-stream`，該整合會將串流重新發出為伺服器發送事件。

在 v2.1.208 之前，相同的配置錯誤會在整個回應被緩衝後顯示為 `API Error: Truncated event message received`。

若要修復此問題，請配置閘道以不修改地傳遞 `InvokeModelWithResponseStream` 回應主體及其 `Content-Type` 標頭。如果閘道只重寫標頭並完整傳遞二進位主體，請設定 [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1`](/docs/zh-TW/env-vars) 以在修復閘道之前跳過檢查。關閉檢查後，已轉換的回應主體會再次失敗，並顯示 `Truncated event message received`。

<h3 id="zero-token-counts-in-/context">
  /context 中的零權杖計數
</h3>

`/context` 命令透過將工具架構傳送至 Amazon Bedrock count-tokens API 來計算每個工具群組的權杖。在 Claude Code v2.1.196 之前的版本中，Amazon Bedrock 拒絕了該請求，因為架構包含其 count-tokens API 不接受的欄位，因此每個工具群組都顯示 0 個權杖。分解中的其他列（例如訊息和記憶體檔案）不受影響。

更新至 v2.1.196 或更新版本。

<h3 id="mantle-endpoint-errors">
  Mantle 端點錯誤
</h3>

如果在設定 `CLAUDE_CODE_USE_MANTLE` 後 `/status` 未顯示 `Amazon Bedrock (Mantle)`，則該變數未到達程序。確認它已在您啟動 `claude` 的 shell 中匯出，或在[設定檔](/docs/zh-TW/settings)的 `env` 區塊中設定它。

來自 Mantle 端點的 `403`（具有有效認證）表示您的 AWS 帳戶尚未被授予存取您要求的模型的權限。請聯絡您的 AWS 帳戶團隊以要求存取。

命名模型 ID 的 `400` 表示該模型未在 Mantle 上提供。Mantle 有其自己的模型陣容，與標準 Amazon Bedrock 目錄分開，因此推論設定檔 ID（例如 `us.anthropic.claude-sonnet-4-6`）將無法運作。使用 Mantle 格式的 ID，或啟用[兩個端點](#run-mantle-alongside-the-invoke-api)，以便 Claude Code 將每個請求路由到模型可用的端點。

<h2 id="additional-resources">
  其他資源
</h2>

* [Amazon Bedrock 文件](https://docs.aws.amazon.com/bedrock/)
* [Amazon Bedrock 定價](https://aws.amazon.com/bedrock/pricing/)
* [Amazon Bedrock 推論設定檔](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html)
* [Amazon Bedrock 權杖燃盡和配額](https://docs.aws.amazon.com/bedrock/latest/userguide/quotas-token-burndown.html)
* [Amazon Bedrock 上的 Claude Code：快速設定指南](https://community.aws/content/2tXkZKrZzlrlu0KfH8gST5Dkppq/claude-code-on-amazon-bedrock-quick-setup-guide)
* [Claude Code 監控實施 (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md)
