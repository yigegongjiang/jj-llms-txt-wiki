> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# AWS의 Claude Platform에서 Claude Code

> AWS 인증, IAM 액세스 제어 및 AWS Marketplace 청구를 사용하여 Anthropic 운영 Claude API를 사용하도록 Claude Code를 구성합니다.

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

AWS의 Claude Platform은 AWS 인증, IAM 액세스 제어 및 AWS Marketplace 청구를 지원하는 Anthropic 운영 Claude API입니다. 요청은 Anthropic의 API에 직접 도달하므로 동일한 릴리스 일정에 따라 [Claude API](https://platform.claude.com/docs)와 동일한 모델 및 API 기능을 사용할 수 있습니다. Anthropic의 기능 플래그 서비스를 통해 Claude Code가 활성화하는 클라이언트 측 기능(예: [`/loop` 자동 속도 조절](/docs/ko/scheduled-tasks#let-claude-choose-the-interval))은 기본적으로 비활성화되어 있으며, [어드바이저 도구](/docs/ko/advisor)는 사용할 수 없습니다. 전체 목록은 [기능 가용성 매트릭스](/docs/ko/feature-availability#summary-by-provider)를 참조하십시오. AWS 자격 증명 또는 워크스페이스 API 키로 인증하며, AWS Marketplace를 통해 비용을 지불합니다.

이 가이드를 사용하여 Claude Platform on AWS를 통해 이미 프로비저닝한 워크스페이스를 가리키도록 Claude Code를 설정합니다. 이 전에 필요한 AWS 구독 및 워크스페이스 설정은 [Claude Platform on AWS 설명서](https://platform.claude.com/docs/en/build-with-claude/claude-platform-on-aws)를 참조하십시오.

<Note>
  AWS Marketplace를 통해 구독하면 AWS 계정에 연결된 새로운 Anthropic 조직이 프로비저닝됩니다. 이 조직은 Anthropic에서 이미 보유한 모든 조직과 별개이며, 자격 증명은 조직 간에 전송되지 않습니다. AWS 연결 조직의 워크스페이스 ID 및 API 키를 사용하고, 기존 Claude Console 계정의 키는 사용하지 마십시오.
</Note>

<h2 id="prerequisites">
  필수 조건
</h2>

Claude Code를 구성하기 전에 다음이 필요합니다.

* AWS Marketplace를 통한 활성 Claude Platform on AWS 구독
* AWS 연결 Anthropic 조직의 워크스페이스 및 해당 워크스페이스 ID
* Anthropic 서비스를 호출할 수 있는 권한이 있는 IAM 주체 또는 워크스페이스로 범위가 지정된 API 키
* 환경, `~/.aws/credentials` 또는 SigV4 인증을 원하는 경우 연결된 IAM 역할의 AWS 자격 증명. AWS CLI는 SSO 로그인 흐름에만 필요합니다.

<h2 id="setup">
  설정
</h2>

<h3 id="1-configure-aws-credentials">
  1. AWS 자격 증명 구성
</h3>

Claude Code는 Claude Platform on AWS에 대해 두 가지 인증 방법을 지원합니다. 팀이 액세스를 관리하는 방식에 맞는 방법을 선택하십시오.

**옵션 A: SigV4를 사용한 AWS 자격 증명**

Claude Code는 표준 AWS 자격 증명 체인을 사용하여 SigV4로 요청에 서명합니다. 환경 변수, `~/.aws/credentials`의 공유 자격 증명, IAM 역할, AWS SSO 세션 및 AWS SDK가 지원하는 기타 소스입니다.

로컬 사용의 경우 Claude Code를 시작하기 전에 AWS CLI로 로그인하십시오. 아래 예제는 SSO 프로필을 사용하지만 표준 위치에 자격 증명을 생성하는 모든 방법이 작동합니다.

```bash theme={null}
aws sso login --profile my-profile
export AWS_PROFILE=my-profile
```

CI 및 자동화의 경우 Anthropic 서비스를 호출할 수 있는 권한이 있는 IAM 역할을 실행기에 제공하고 `AWS_REGION`을 설정하십시오. 자격 증명 체인이 역할을 자동으로 선택합니다.

SSO 자격 증명이 세션 중에 만료되면 [`awsAuthRefresh`](/docs/ko/amazon-bedrock#advanced-credential-configuration)를 구성하여 Claude Code가 로그인 명령을 다시 실행하고 실패하는 대신 재시도하도록 합니다. Claude Platform on AWS에서 자동 새로 고침을 하려면 Claude Code v2.1.198 이상이 필요합니다. 이전 버전은 `/login`을 실행하라는 프롬프트로 중지되며, AWS 자격 증명을 새로 고칠 수 없습니다. `settings.json`에 명령을 추가하십시오.

```json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile my-profile"
}
```

`awsAuthRefresh`가 구성되면 `/login`은 **Using 3rd-party platforms** 아래에 **Claude Platform on AWS · refresh credentials** 옵션을 표시합니다. 이를 선택하면 구성된 명령을 실행하고 Claude Code를 다시 시작하지 않고 AWS 자격 증명을 다시 읽습니다.

**옵션 B: 워크스페이스 API 키**

워크스페이스 API 키는 장기 보안 비밀이며, 페더레이션된 AWS 자격 증명을 관리하지 않으려는 경우에 유용합니다. AWS Console의 **Claude Platform on AWS → API keys** 아래에서 생성하고 `ANTHROPIC_AWS_API_KEY`로 설정하십시오.

```bash theme={null}
export ANTHROPIC_AWS_API_KEY=sk-ant-xxxxx
```

키는 `x-api-key`로 전송되며 SigV4보다 우선하므로 환경의 모든 AWS 자격 증명이 무시됩니다. 별도의 Claude Console 조직의 API 키는 여기서 작동하지 않습니다.

워크스페이스 API 키를 다른 프로덕션 자격 증명처럼 취급하십시오. [사용자 설정 파일](/docs/ko/settings) `env` 블록은 전역으로 내보내지 않고 키를 컴퓨터로 범위를 지정하는 편리한 방법입니다.

<Note>
  `/login` 및 `/logout` 명령은 Claude Platform on AWS에 대해 Claude.ai 구독에 로그인하지 않습니다. 인증은 AWS 자격 증명 또는 워크스페이스 API 키를 통해 실행됩니다. 예외는 `awsAuthRefresh`가 구성될 때 `/login`이 표시하는 **refresh credentials** 옵션이며, 이는 위에서 설명한 대로 AWS 자격 증명을 다시 읽습니다.
</Note>

<h3 id="2-configure-claude-code">
  2. Claude Code 구성
</h3>

Claude Code를 기본 Anthropic API 대신 Claude Platform on AWS를 통해 라우팅하는 환경 변수를 설정하십시오.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export AWS_REGION=us-east-1
```

`ANTHROPIC_AWS_WORKSPACE_ID`는 필수이며 모든 요청에서 `anthropic-workspace-id` 헤더로 전송됩니다. 기본 URL은 `AWS_REGION`에서 `https://aws-external-anthropic.{region}.api.aws`로 계산됩니다. URL을 직접 재정의하려면 `ANTHROPIC_AWS_BASE_URL`을 설정하십시오.

Claude Platform on AWS는 환경에 AWS 자격 증명이 있어도 선택 사항입니다. Amazon Bedrock 및 Microsoft Foundry는 공급자 라우팅에서 우선하므로 설정된 경우 `CLAUDE_CODE_USE_BEDROCK` 및 `CLAUDE_CODE_USE_FOUNDRY`를 해제하십시오.

<h3 id="3-pin-model-versions">
  3. 모델 버전 고정
</h3>

Claude Platform on AWS는 직접 Claude API와 동일한 모델 ID를 사용합니다.

기본 별칭 `fable`, `opus`, `sonnet` 및 `haiku`는 Claude Platform on AWS에 대한 Claude Code의 기본 제공 기본값으로 확인되며, 이는 최신 릴리스보다 뒤떨어질 수 있습니다. `ANTHROPIC_DEFAULT_OPUS_MODEL` 없이 `opus` 별칭은 Opus 4.8로 확인됩니다. v2.1.207 이전에는 Opus 4.7로 확인되었습니다.

Claude Code를 팀에 배포하는 경우 모델 ID를 명시적으로 고정하여 새 릴리스가 모든 사람을 한 번에 이동하지 않도록 하십시오.

```bash theme={null}
export ANTHROPIC_DEFAULT_FABLE_MODEL=claude-fable-5
export ANTHROPIC_DEFAULT_OPUS_MODEL=claude-opus-4-8
export ANTHROPIC_DEFAULT_SONNET_MODEL=claude-sonnet-5
export ANTHROPIC_DEFAULT_HAIKU_MODEL=claude-haiku-4-5
```

모델 ID 및 별칭의 전체 목록은 [모델 개요](https://platform.claude.com/docs/en/about-claude/models/overview)를 참조하십시오. 기타 모델 관련 변수는 [모델 구성](/docs/ko/model-config)을 참조하십시오.

[프롬프트 캐싱](/docs/ko/prompt-caching)은 자동으로 활성화됩니다. 1시간 캐시 TTL을 5분 기본값 대신 요청하려면 `ENABLE_PROMPT_CACHING_1H=1`을 설정하십시오. API는 1시간 캐시 쓰기를 더 높은 요금으로 청구합니다. 요금은 [프롬프트 캐싱 가격 책정](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing)을 참조하십시오.

<h2 id="use-the-agent-sdk">
  Agent SDK 사용
</h2>

[Agent SDK](/docs/ko/agent-sdk/overview)는 CLI와 동일한 환경 변수를 읽으므로 Claude Code 하위 프로세스를 생성하는 모든 프로그램은 호출 전에 `CLAUDE_CODE_USE_ANTHROPIC_AWS`, `ANTHROPIC_AWS_WORKSPACE_ID` 및 `ANTHROPIC_AWS_API_KEY` 또는 AWS 자격 증명을 내보내 Claude Platform on AWS를 대상으로 할 수 있습니다.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

process.env.CLAUDE_CODE_USE_ANTHROPIC_AWS = "1";
process.env.ANTHROPIC_AWS_WORKSPACE_ID = "wrkspc_01ABCDEFGHIJKLMN";
process.env.AWS_REGION = "us-east-1";

for await (const msg of query({ prompt: "What's in this repo?" })) {
  console.log(msg);
}
```

이 예제는 SigV4에 대한 주변 AWS 자격 증명 체인을 사용합니다. 대신 워크스페이스 API 키로 인증하려면 동일한 방식으로 `ANTHROPIC_AWS_API_KEY`를 설정하십시오. 더 광범위한 Agent SDK 표면은 [Agent SDK 개요](/docs/ko/agent-sdk/overview)를 참조하십시오.

<h2 id="route-through-a-corporate-proxy">
  기업 프록시를 통해 라우팅
</h2>

프록시 또는 [LLM 게이트웨이](/docs/ko/llm-gateway)를 통해 트래픽을 라우팅하려면 `ANTHROPIC_AWS_BASE_URL`을 프록시의 주소로 설정하십시오. Claude Code는 동일한 워크스페이스 및 인증 헤더를 사용하여 해당 URL로 요청을 보내므로 변경되지 않은 상태로 전달하는 모든 게이트웨이가 작동합니다.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export ANTHROPIC_AWS_BASE_URL=https://anthropic-proxy.example.com
```

게이트웨이가 요청 자체에 서명하는 경우 `CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1`을 설정하여 Claude Code가 서명되지 않은 요청을 보내고 게이트웨이가 AWS로 전달하기 전에 SigV4 헤더를 추가하도록 합니다. 게이트웨이가 자체 토큰을 요구하는 경우 `ANTHROPIC_AUTH_TOKEN`에 설정하십시오.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export ANTHROPIC_AWS_BASE_URL=https://anthropic-proxy.example.com
```

<h2 id="troubleshooting">
  문제 해결
</h2>

`/status`를 실행하여 확인된 공급자 및 명시적으로 구성된 워크스페이스 ID, 지역, 기본 URL 재정의 및 인증 건너뛰기 설정을 확인하십시오. 이것이 Claude Code가 Claude Platform on AWS를 대상으로 하는지 확인하는 가장 빠른 방법입니다.

<h3 id="403-forbidden-or-accessdenied-on-every-request">
  모든 요청에서 `403 Forbidden` 또는 `AccessDenied`
</h3>

Claude Code가 확인한 IAM 주체는 워크스페이스에서 Anthropic 서비스를 호출할 수 있는 권한이 없을 가능성이 높습니다. AWS 프로필에 연결된 역할 또는 Claude Code를 시작한 실행기를 확인하고 [IAM 작업 참조](https://platform.claude.com/docs/ko/api/claude-platform-on-aws-iam-actions)에 설명된 `aws-external-anthropic` 작업이 있는지 확인하십시오.

`ANTHROPIC_AWS_API_KEY`를 설정한 경우 키가 SigV4보다 우선하고 오래된 키는 동일한 오류를 생성합니다. AWS Console의 **Claude Platform on AWS → API keys** 아래에서 키를 다시 생성하거나 변수를 해제하여 AWS 자격 증명으로 돌아가십시오.

<h3 id="requests-fail-with-a-missing-workspace-error">
  요청이 누락된 워크스페이스 오류로 실패합니다.
</h3>

`ANTHROPIC_AWS_WORKSPACE_ID`가 설정되지 않았거나 비어 있을 가능성이 높습니다. 모든 Claude Platform on AWS 요청에는 워크스페이스 ID가 포함되어야 합니다. AWS 자격 증명에 의해 암시되지 않습니다. AWS Console 서비스 페이지의 **Workspaces** 아래에서 ID를 찾고 Claude Code를 시작하기 전에 내보내십시오.

<h3 id="requests-still-go-to-api-anthropic-com">
  요청이 여전히 `api.anthropic.com`으로 이동합니다.
</h3>

`CLAUDE_CODE_USE_ANTHROPIC_AWS`가 설정되지 않았거나 truthy로 구문 분석되지 않는 값으로 설정되었을 가능성이 높습니다. `1`로 설정하고 `/status`를 실행하여 확인된 공급자를 확인하십시오. `CLAUDE_CODE_USE_BEDROCK` 또는 `CLAUDE_CODE_USE_FOUNDRY`도 설정된 경우 Claude Platform on AWS보다 우선합니다.

<h2 id="additional-resources">
  추가 리소스
</h2>

Claude Code 구성 전에 필요한 Claude Platform on AWS 구독, 워크스페이스 및 IAM 설정은 플랫폼 설명서에서 다룹니다.

* [Claude Platform on AWS 개요](https://platform.claude.com/docs/ko/build-with-claude/claude-platform-on-aws): 구독, 워크스페이스 설정 및 제품 참조
* [IAM 작업 참조](https://platform.claude.com/docs/ko/api/claude-platform-on-aws-iam-actions): 권한 및 관리형 정책
