> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# AWS 上的 Claude Platform 上的 Claude Code

> 設定 Claude Code 以使用 Anthropic 營運的 Claude API，搭配 AWS 驗證、IAM 存取控制和 AWS Marketplace 計費。

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

export const Experiment = ({flag, treatment, children}) => {
  const VID_KEY = 'exp_vid';
  const CONSENT_COUNTRIES = new Set(['AT', 'BE', 'BG', 'HR', 'CY', 'CZ', 'DK', 'EE', 'FI', 'FR', 'DE', 'GR', 'HU', 'IE', 'IT', 'LV', 'LT', 'LU', 'MT', 'NL', 'PL', 'PT', 'RO', 'SK', 'SI', 'ES', 'SE', 'RE', 'GP', 'MQ', 'GF', 'YT', 'BL', 'MF', 'PM', 'WF', 'PF', 'NC', 'AW', 'CW', 'SX', 'FO', 'GL', 'AX', 'GB', 'UK', 'AI', 'BM', 'IO', 'VG', 'KY', 'FK', 'GI', 'MS', 'PN', 'SH', 'TC', 'GG', 'JE', 'IM', 'CA', 'BR', 'IN']);
  const fnv1a = s => {
    let h = 0x811c9dc5;
    for (let i = 0; i < s.length; i++) {
      h ^= s.charCodeAt(i);
      h += (h << 1) + (h << 4) + (h << 7) + (h << 8) + (h << 24);
    }
    return h >>> 0;
  };
  const bucket = (seed, vid) => fnv1a(fnv1a(seed + vid) + '') % 10000 < 5000 ? 'control' : 'treatment';
  const [decision] = useState(() => {
    const params = new URLSearchParams(location.search);
    const preBucketed = document.documentElement.dataset['gb_' + flag.replace(/-/g, '_')];
    const force = params.get('gb-force');
    if (force) {
      for (const p of force.split(',')) {
        const [k, v] = p.split(':');
        if (k === flag) return {
          variant: v || 'treatment',
          track: false
        };
      }
    }
    if (navigator.globalPrivacyControl) {
      return {
        variant: 'control',
        track: false
      };
    }
    const prefsMatch = document.cookie.match(/(?:^|; )anthropic-consent-preferences=([^;]+)/);
    if (prefsMatch) {
      try {
        if (JSON.parse(decodeURIComponent(prefsMatch[1])).analytics !== true) {
          return {
            variant: 'control',
            track: false
          };
        }
      } catch {
        return {
          variant: 'control',
          track: false
        };
      }
    } else {
      const country = params.get('country')?.toUpperCase() || (document.cookie.match(/(?:^|; )cf_geo=([A-Z]{2})/) || [])[1];
      if (!country || CONSENT_COUNTRIES.has(country)) {
        return {
          variant: 'control',
          track: false
        };
      }
    }
    let vid;
    try {
      const ajsMatch = document.cookie.match(/(?:^|; )ajs_anonymous_id=([^;]+)/);
      if (ajsMatch) {
        vid = decodeURIComponent(ajsMatch[1]).replace(/^"|"$/g, '');
      } else {
        vid = localStorage.getItem(VID_KEY);
        if (!vid) {
          vid = crypto.randomUUID();
        }
        document.cookie = `ajs_anonymous_id=${vid}; domain=.claude.com; path=/; Secure; SameSite=Lax; max-age=31536000`;
      }
      try {
        localStorage.setItem(VID_KEY, vid);
      } catch {}
    } catch {
      return {
        variant: 'control',
        track: false
      };
    }
    const variant = preBucketed === '1' ? 'treatment' : preBucketed === '0' ? 'control' : bucket(flag, vid);
    return {
      variant,
      track: true,
      vid
    };
  });
  useEffect(() => {
    if (!decision.track) return;
    fetch('https://api.anthropic.com/api/event_logging/v2/batch', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-service-name': 'claude_code_docs'
      },
      body: JSON.stringify({
        events: [{
          event_type: 'GrowthbookExperimentEvent',
          event_data: {
            device_id: decision.vid,
            anonymous_id: decision.vid,
            timestamp: new Date().toISOString(),
            experiment_id: flag,
            variation_id: decision.variant === 'treatment' ? 1 : 0,
            environment: 'production'
          }
        }]
      }),
      keepalive: true
    }).catch(() => {});
  }, []);
  return decision.variant === 'treatment' ? treatment : children;
};

<Experiment flag="docs-contact-sales-cta" treatment={<ContactSalesCard surface="claude_platform_on_aws" />} />

AWS 上的 Claude Platform 是 Anthropic 營運的 Claude API，具有 AWS 驗證、IAM 存取控制和 AWS Marketplace 計費。請求直接到達 Anthropic 的 API，因此您可以獲得與 [Claude API](https://platform.claude.com/docs) 相同的模型和 API 功能，並遵循相同的發佈時程表。Claude Code 透過 Anthropic 的功能旗標服務啟用的用戶端功能，例如 [`/loop` 自我調整步調](/docs/zh-TW/scheduled-tasks#let-claude-choose-the-interval)，預設為關閉，且 [advisor 工具](/docs/zh-TW/advisor) 無法使用。請參閱 [功能可用性矩陣](/docs/zh-TW/feature-availability#summary-by-provider) 以取得完整清單。您使用 AWS 認證或工作區 API 金鑰進行驗證，並透過 AWS Marketplace 付款。

使用本指南將 Claude Code 指向您已透過 AWS 上的 Claude Platform 佈建的工作區。有關在此之前的 AWS 訂閱和工作區設定，請參閱 [AWS 上的 Claude Platform 文件](https://platform.claude.com/docs/en/build-with-claude/claude-platform-on-aws)。

<Note>
  透過 AWS Marketplace 訂閱會佈建一個與您的 AWS 帳戶相關聯的新 Anthropic 組織。此組織與您已有的任何 Anthropic 組織分開，認證不會在它們之間轉移。使用來自 AWS 連結組織的工作區 ID 和 API 金鑰，而不是來自預先存在的 Claude Console 帳戶。
</Note>

<h2 id="prerequisites">
  先決條件
</h2>

在設定 Claude Code 之前，您需要：

* 透過 AWS Marketplace 的有效 AWS 上的 Claude Platform 訂閱
* 您的 AWS 連結 Anthropic 組織中的工作區，及其工作區 ID
* 具有叫用 Anthropic 服務權限的 IAM 主體，或限定於工作區的 API 金鑰
* 您環境中的 AWS 認證、`~/.aws/credentials` 中的認證，或來自附加 IAM 角色的認證（如果您想要 SigV4 驗證）。AWS CLI 僅在 SSO 登入流程中需要。

<h2 id="setup">
  設定
</h2>

<h3 id="1-configure-aws-credentials">
  1. 設定 AWS 認證
</h3>

Claude Code 支援 AWS 上的 Claude Platform 的兩種驗證方法。選擇適合您的團隊如何管理存取的方法。

**選項 A：使用 SigV4 的 AWS 認證**

Claude Code 使用標準 AWS 認證鏈使用 SigV4 簽署請求：環境變數、`~/.aws/credentials` 中的共享認證、IAM 角色、AWS SSO 工作階段，以及 AWS SDK 支援的任何其他來源。

對於本機使用，在啟動 Claude Code 之前使用 AWS CLI 登入。下面的範例使用 SSO 設定檔，但任何在標準位置產生認證的方法都有效。

```bash theme={null}
aws sso login --profile my-profile
export AWS_PROFILE=my-profile
```

對於 CI 和自動化，給予執行器具有叫用 Anthropic 服務權限的 IAM 角色，並設定 `AWS_REGION`。認證鏈會自動選取該角色。

如果您的 SSO 認證在工作階段中途過期，請設定 [`awsAuthRefresh`](/docs/zh-TW/amazon-bedrock#advanced-credential-configuration)，以便 Claude Code 重新執行您的登入命令並重試，而不是失敗。AWS 上的 Claude Platform 上的自動重新整理需要 Claude Code v2.1.198 或更新版本；較早的版本會停止並提示執行 `/login`，這無法重新整理 AWS 認證。將命令新增至您的 `settings.json`：

```json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile my-profile"
}
```

設定 `awsAuthRefresh` 後，`/login` 會在 **使用第三方平台** 下顯示 **Claude Platform on AWS · 重新整理認證** 選項。選取它會執行已設定的命令，並重新讀取您的 AWS 認證，而無需重新啟動 Claude Code。

**選項 B：工作區 API 金鑰**

工作區 API 金鑰是長期有效的祕密，在您不想管理聯合 AWS 認證時很有用。在 AWS Console 中的 **Claude Platform on AWS → API keys** 下產生一個，並將其設定為 `ANTHROPIC_AWS_API_KEY`：

```bash theme={null}
export ANTHROPIC_AWS_API_KEY=sk-ant-xxxxx
```

金鑰以 `x-api-key` 形式傳送，優先於 SigV4，因此您環境中的任何 AWS 認證都會被忽略。來自單獨 Claude Console 組織的 API 金鑰在此不起作用。

將工作區 API 金鑰視為任何其他生產認證。[使用者設定檔](/docs/zh-TW/settings) `env` 區塊是在不全域匯出的情況下將金鑰限定於您的機器的便利方式。

<Note>
  `/login` 和 `/logout` 命令不會變更 AWS 上的 Claude Platform 驗證。驗證透過您的 AWS 認證或工作區 API 金鑰執行，而不是透過 Claude.ai 訂閱。唯一的例外是當設定 `awsAuthRefresh` 時，`/login` 顯示的 **重新整理認證** 選項，它會如上所述重新讀取您的 AWS 認證。
</Note>

<h3 id="2-configure-claude-code">
  2. 設定 Claude Code
</h3>

設定環境變數，將 Claude Code 路由透過 AWS 上的 Claude Platform，而不是預設的 Anthropic API。

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export AWS_REGION=us-east-1
```

`ANTHROPIC_AWS_WORKSPACE_ID` 是必需的，並在每個請求上作為 `anthropic-workspace-id` 標頭傳送。基礎 URL 從 `AWS_REGION` 計算為 `https://aws-external-anthropic.{region}.api.aws`。若要直接覆寫 URL，請設定 `ANTHROPIC_AWS_BASE_URL`。

即使您的環境中存在 AWS 認證，AWS 上的 Claude Platform 也是選擇加入的。Amazon Bedrock 和 Microsoft Foundry 在提供者路由中優先，因此如果設定了 `CLAUDE_CODE_USE_BEDROCK` 和 `CLAUDE_CODE_USE_FOUNDRY`，請取消設定它們。

<h3 id="3-pin-model-versions">
  3. 固定模型版本
</h3>

AWS 上的 Claude Platform 使用與直接 Claude API 相同的模型 ID。

預設別名 `fable`、`opus`、`sonnet` 和 `haiku` 解析為 Claude Code 針對 AWS 上的 Claude Platform 的內建預設值，這些值可能落後於最新版本。沒有 `ANTHROPIC_DEFAULT_OPUS_MODEL`，`opus` 別名解析為 Opus 4.8。在 v2.1.207 之前，它解析為 Opus 4.7。

如果您將 Claude Code 部署到團隊，請明確固定模型 ID，以便新版本不會一次移動所有人：

```bash theme={null}
export ANTHROPIC_DEFAULT_FABLE_MODEL=claude-fable-5
export ANTHROPIC_DEFAULT_OPUS_MODEL=claude-opus-4-8
export ANTHROPIC_DEFAULT_SONNET_MODEL=claude-sonnet-5
export ANTHROPIC_DEFAULT_HAIKU_MODEL=claude-haiku-4-5
```

有關模型 ID 和別名的完整清單，請參閱[模型概述](https://platform.claude.com/docs/en/about-claude/models/overview)。有關其他模型相關變數，請參閱[模型設定](/docs/zh-TW/model-config)。

[Prompt caching](/docs/zh-TW/prompt-caching) 會自動啟用。若要要求 1 小時快取 TTL 而不是 5 分鐘預設值，請設定 `ENABLE_PROMPT_CACHING_1H=1`。API 以更高的費率計費 1 小時快取寫入。有關費率，請參閱 [prompt caching 定價](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing)。

<h2 id="use-the-agent-sdk">
  使用 Agent SDK
</h2>

[Agent SDK](/docs/zh-TW/agent-sdk/overview) 讀取與 CLI 相同的環境變數，因此任何產生 Claude Code 子程序的程式都可以透過在呼叫前匯出 `CLAUDE_CODE_USE_ANTHROPIC_AWS`、`ANTHROPIC_AWS_WORKSPACE_ID` 和 `ANTHROPIC_AWS_API_KEY` 或 AWS 認證來針對 AWS 上的 Claude Platform。

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

process.env.CLAUDE_CODE_USE_ANTHROPIC_AWS = "1";
process.env.ANTHROPIC_AWS_WORKSPACE_ID = "wrkspc_01ABCDEFGHIJKLMN";
process.env.AWS_REGION = "us-east-1";

for await (const msg of query({ prompt: "What's in this repo?" })) {
  console.log(msg);
}
```

此範例依賴環境 AWS 認證鏈進行 SigV4。若要改用工作區 API 金鑰進行驗證，請以相同方式設定 `ANTHROPIC_AWS_API_KEY`。有關更廣泛的 Agent SDK 表面，請參閱 [Agent SDK 概述](/docs/zh-TW/agent-sdk/overview)。

<h2 id="route-through-a-corporate-proxy">
  透過公司代理路由
</h2>

若要透過代理或 [LLM gateway](/docs/zh-TW/llm-gateway) 路由流量，請將 `ANTHROPIC_AWS_BASE_URL` 設定為代理的位址。Claude Code 將請求傳送至該 URL，並使用相同的工作區和驗證標頭，因此任何轉發它們不變的閘道都有效。

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export ANTHROPIC_AWS_BASE_URL=https://anthropic-proxy.example.com
```

如果您的閘道自行簽署請求，請設定 `CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1`，以便 Claude Code 傳送未簽署的請求，並讓閘道在轉發到 AWS 之前新增 SigV4 標頭。如果閘道需要自己的權杖，請在 `ANTHROPIC_AUTH_TOKEN` 中設定它。

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export ANTHROPIC_AWS_BASE_URL=https://anthropic-proxy.example.com
```

<h2 id="troubleshooting">
  Troubleshooting
</h2>

執行 `/status` 以查看已解析的提供者和任何明確設定的工作區 ID、區域、基礎 URL 覆寫和驗證跳過設定。這是確認 Claude Code 是否完全針對 AWS 上的 Claude Platform 的最快方式。

<h3 id="403-forbidden-or-accessdenied-on-every-request">
  `403 Forbidden` 或 `AccessDenied` 在每個請求上
</h3>

Claude Code 解析的 IAM 主體可能缺少在您的工作區中叫用 Anthropic 服務的權限。檢查附加到您的 AWS 設定檔或啟動 Claude Code 的執行器的角色，並驗證它具有 [IAM 動作參考](https://platform.claude.com/docs/zh-TW/api/claude-platform-on-aws-iam-actions)中記錄的 `aws-external-anthropic` 動作。

如果您設定了 `ANTHROPIC_AWS_API_KEY`，金鑰優先於 SigV4，過期的金鑰會產生相同的錯誤。在 AWS Console 中的 **Claude Platform on AWS → API keys** 下重新產生金鑰，或取消設定變數以回退到您的 AWS 認證。

<h3 id="requests-fail-with-a-missing-workspace-error">
  請求失敗，出現遺失工作區錯誤
</h3>

`ANTHROPIC_AWS_WORKSPACE_ID` 可能未設定或為空。每個 AWS 上的 Claude Platform 請求都必須包含工作區 ID。它不是由您的 AWS 認證隱含的。在 AWS Console 服務頁面上的 **Workspaces** 下找到 ID，並在啟動 Claude Code 之前匯出它。

<h3 id="requests-still-go-to-api-anthropic-com">
  請求仍然轉到 `api.anthropic.com`
</h3>

`CLAUDE_CODE_USE_ANTHROPIC_AWS` 可能未設定或設定為不解析為真值的值。將其設定為 `1` 並執行 `/status` 以確認已解析的提供者。如果也設定了 `CLAUDE_CODE_USE_BEDROCK` 或 `CLAUDE_CODE_USE_FOUNDRY`，那些優先於 AWS 上的 Claude Platform。

<h2 id="additional-resources">
  其他資源
</h2>

設定 Claude Code 之前的 AWS 上的 Claude Platform 訂閱、工作區和 IAM 設定涵蓋在平台文件中：

* [AWS 上的 Claude Platform 概述](https://platform.claude.com/docs/zh-TW/build-with-claude/claude-platform-on-aws)：訂閱、工作區設定和產品參考
* [IAM 動作參考](https://platform.claude.com/docs/zh-TW/api/claude-platform-on-aws-iam-actions)：權限和受管原則
