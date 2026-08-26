> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Google Cloud의 Agent Platform에서 Claude Code 사용하기

> Google Cloud의 Agent Platform(이전 Vertex AI)을 통해 Claude Code를 구성하는 방법을 알아봅니다. 설정, IAM 구성 및 문제 해결을 포함합니다.

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
  필수 요구사항
</h2>

Google Cloud의 Agent Platform(이전의 Vertex AI)을 사용하여 Claude Code를 구성하기 전에 다음을 확인하십시오:

* 청구가 활성화된 Google Cloud Platform(GCP) 계정
* Google Cloud의 Agent Platform API가 활성화된 GCP 프로젝트
* 원하는 Claude 모델에 대한 액세스(예: Claude Sonnet 4.6)
* Google Cloud SDK(`gcloud`) 설치 및 구성
* 원하는 GCP 지역에 할당된 할당량

자신의 Google Cloud의 Agent Platform 자격증명으로 로그인하려면 아래의 [Google Cloud의 Agent Platform으로 로그인](#sign-in-with-agent-platform)을 따르십시오. 팀 전체에 Claude Code를 배포하려면 [수동 설정](#set-up-manually) 단계를 사용하고 롤아웃 전에 [모델 버전을 고정](#5-pin-model-versions)하십시오.

<h2 id="sign-in-with-agent-platform">
  Agent Platform으로 로그인
</h2>

Google Cloud 자격증명이 있고 Google Cloud의 Agent Platform을 통해 Claude Code 사용을 시작하려면 로그인 마법사가 이를 안내합니다. GCP 측 필수 요구사항을 프로젝트당 한 번 완료하면 마법사가 Claude Code 측을 처리합니다.

<Steps>
  <Step title="GCP 프로젝트에서 Claude 모델 활성화">
    프로젝트에 대해 [Google Cloud의 Agent Platform API 활성화](#1-enable-agent-platform-api)한 다음 [Google Cloud의 Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden)에서 원하는 Claude 모델에 대한 액세스를 요청합니다. 계정에 필요한 권한은 [IAM 구성](#iam-configuration)을 참조하십시오.
  </Step>

  <Step title="Claude Code를 시작하고 Google Cloud의 Agent Platform 선택">
    `claude`를 실행합니다. 로그인 프롬프트에서 **3rd-party platform**을 선택한 다음 **Google Vertex AI**를 선택합니다. 이는 로그인 프롬프트가 Google Cloud의 Agent Platform에 대해 여전히 사용하는 레이블입니다.
  </Step>

  <Step title="마법사 프롬프트 따르기">
    Google Cloud에 인증하는 방법을 선택합니다: `gcloud`의 Application Default Credentials, 서비스 계정 키 파일 또는 환경에 이미 있는 자격증명. 마법사는 프로젝트와 지역을 감지하고, 프로젝트가 호출할 수 있는 Claude 모델을 확인하며, 이를 고정할 수 있게 합니다. 결과를 [사용자 설정 파일](/docs/ko/settings)의 `env` 블록에 저장하므로 환경 변수를 직접 내보낼 필요가 없습니다.
  </Step>
</Steps>

로그인한 후 언제든지 `/setup-vertex`를 실행하여 마법사를 다시 열고 자격증명, 프로젝트, 지역 또는 모델 고정을 변경할 수 있습니다. 모델 고정 단계는 현재 고정된 모델에서 시작됩니다. 마법사는 `~/.claude/settings.json`에 쓰거나, [`CLAUDE_CONFIG_DIR`](/docs/ko/env-vars#variables)이 설정되어 있을 때 `$CLAUDE_CONFIG_DIR/settings.json`에 씁니다.

<h2 id="region-configuration">
  지역 구성
</h2>

Claude Code는 Google Cloud의 Agent Platform [전역](https://cloud.google.com/blog/products/ai-machine-learning/global-endpoint-for-claude-models-generally-available-on-vertex-ai), 다중 지역 및 지역 엔드포인트를 지원합니다. `CLOUD_ML_REGION`을 `global`, `eu` 또는 `us`와 같은 다중 지역 위치 또는 `us-east5`와 같은 특정 지역으로 설정합니다. Claude Code는 `aiplatform.eu.rep.googleapis.com` 및 `aiplatform.us.rep.googleapis.com` 호스트를 포함한 다중 지역 위치에 대해 각 형식에 맞는 올바른 Google Cloud의 Agent Platform 호스트명을 선택합니다.

<Note>
  Google Cloud의 Agent Platform은 모든 엔드포인트 유형에서 Claude Code 기본 모델을 지원하지 않을 수 있습니다. 모델 가용성은 [특정 지역](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations#genai-partner-models), 다중 지역 위치 및 [전역 엔드포인트](https://cloud.google.com/vertex-ai/generative-ai/docs/partner-models/use-partner-models#supported_models)에 따라 다릅니다. 지원되는 위치로 전환하거나 지원되는 모델을 지정해야 할 수 있습니다.
</Note>

<h2 id="set-up-manually">
  수동 설정
</h2>

Google Cloud의 Agent Platform을 마법사 대신 환경 변수를 통해 구성하려면(예: CI 또는 스크립트된 엔터프라이즈 롤아웃의 경우) 아래 단계를 따르십시오.

<h3 id="1-enable-agent-platform-api">
  1. Agent Platform API 활성화
</h3>

GCP 프로젝트에서 Google Cloud의 Agent Platform API를 활성화합니다:

```bash theme={null}
# 프로젝트 ID 설정
gcloud config set project YOUR-PROJECT-ID

# Agent Platform API 활성화
gcloud services enable aiplatform.googleapis.com
```

<h3 id="2-request-model-access">
  2. 모델 액세스 요청
</h3>

Google Cloud의 Agent Platform에서 Claude 모델에 대한 액세스를 요청합니다:

1. [Google Cloud의 Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden)으로 이동합니다
2. "Claude" 모델을 검색합니다
3. 원하는 Claude 모델에 대한 액세스를 요청합니다(예: Claude Sonnet 4.6)
4. 승인을 기다립니다(24-48시간이 소요될 수 있습니다)

<h3 id="3-configure-gcp-credentials">
  3) GCP 자격증명 구성
</h3>

Claude Code는 표준 Google Cloud 인증을 사용합니다.

자세한 내용은 [Google Cloud 인증 설명서](https://cloud.google.com/docs/authentication)를 참조하십시오.

Claude Code v2.1.121 이상은 동일한 Application Default Credentials 체인을 통해 [X.509 인증서 기반 Workload Identity Federation](https://cloud.google.com/iam/docs/workload-identity-federation-with-x509-certificates)을 지원합니다. `GOOGLE_APPLICATION_CREDENTIALS`를 자격증명 구성 파일의 경로로 설정합니다.

<Note>
  Claude Code는 Google Cloud의 Agent Platform 요청에 대해 `ANTHROPIC_VERTEX_PROJECT_ID`를 프로젝트 ID로 사용합니다. `GCLOUD_PROJECT` 및 `GOOGLE_CLOUD_PROJECT` 환경 변수와 `GOOGLE_APPLICATION_CREDENTIALS`에서 참조하는 자격증명 파일이 이를 우선합니다. 이 중 어느 것도 설정되지 않으면 프로젝트 ID는 `gcloud` 구성 또는 연결된 서비스 계정에서 확인됩니다.
</Note>

<h4 id="advanced-credential-configuration">
  고급 자격증명 구성
</h4>

Claude Code는 `gcpAuthRefresh` 설정을 통해 GCP에 대한 자동 자격증명 새로 고침을 지원합니다. Claude Code가 GCP 자격증명이 만료되었거나 로드할 수 없음을 감지하면 요청을 다시 시도하기 전에 구성된 명령을 실행하여 새 자격증명을 얻습니다.

```json theme={null}
{
  "gcpAuthRefresh": "gcloud auth application-default login",
  "env": {
    "ANTHROPIC_VERTEX_PROJECT_ID": "your-project-id"
  }
}
```

명령의 출력은 사용자에게 표시되지만 대화형 입력은 지원되지 않습니다. 이는 CLI가 URL을 표시하고 브라우저에서 인증을 완료하는 브라우저 기반 인증 흐름에 적합합니다. 인증이 완료되지 않으면 새로 고침 명령은 3분 후에 시간 초과됩니다. `.claude/settings.json`과 같은 프로젝트 설정에서 `gcpAuthRefresh`를 설정하면 워크스페이스 신뢰 프롬프트를 수락한 후에만 명령이 실행됩니다.

<h3 id="4-configure-claude-code">
  4. Claude Code 구성
</h3>

다음 환경 변수를 설정합니다:

```bash theme={null}
# Agent Platform 통합 활성화
export CLAUDE_CODE_USE_VERTEX=1
export CLOUD_ML_REGION=global
export ANTHROPIC_VERTEX_PROJECT_ID=YOUR-PROJECT-ID

# 선택사항: 사용자 정의 엔드포인트 또는 게이트웨이를 위해 Agent Platform 엔드포인트 URL 재정의
# export ANTHROPIC_VERTEX_BASE_URL=https://aiplatform.googleapis.com

# 선택사항: 필요한 경우 prompt caching 비활성화
export DISABLE_PROMPT_CACHING=1

# 선택사항: 기본 5분 대신 1시간 prompt cache TTL 요청
export ENABLE_PROMPT_CACHING_1H=1

# CLOUD_ML_REGION=global일 때, 전역 엔드포인트를 지원하지 않는 모델의 지역 재정의
export VERTEX_REGION_CLAUDE_HAIKU_4_5=us-east5
export VERTEX_REGION_CLAUDE_4_6_SONNET=europe-west1
```

대부분의 모델 버전에는 해당하는 `VERTEX_REGION_CLAUDE_*` 변수가 있습니다. 전체 목록은 [환경 변수 참조](/docs/ko/env-vars)를 참조하십시오. [Google Cloud의 Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden)에서 어떤 모델이 전역 엔드포인트를 지원하는지 또는 지역 전용인지 확인하십시오.

[Prompt caching](/docs/ko/prompt-caching)은 자동으로 활성화됩니다. 이를 비활성화하려면 `DISABLE_PROMPT_CACHING=1`을 설정하십시오. 기본 5분 대신 1시간 캐시 TTL을 요청하려면 `ENABLE_PROMPT_CACHING_1H=1`을 설정하십시오. 1시간 TTL을 사용한 캐시 쓰기는 더 높은 요금으로 청구됩니다. 높은 속도 제한을 위해 Google Cloud 지원팀에 문의하십시오. Google Cloud의 Agent Platform을 사용할 때 `/logout` 명령은 Google Cloud 자격증명을 통해 인증이 처리되므로 사용할 수 없습니다.

Claude Code는 [MCP tool search](/docs/ko/mcp#scale-with-mcp-tool-search)를 Google Cloud의 Agent Platform에서 기본적으로 비활성화합니다. 따라서 모든 MCP 도구 정의는 미리 로드됩니다. Google Cloud의 Agent Platform은 Claude Sonnet 4.5 이상 및 Claude Opus 4.5 이상에 대해 도구 검색을 지원합니다. 이러한 모델에서 도구 검색을 활성화하려면 `ENABLE_TOOL_SEARCH=true`를 설정하십시오. Google Cloud의 Agent Platform의 이전 모델은 필요한 베타 헤더를 허용하지 않으며, 이러한 모델에서 도구 검색을 활성화하면 요청이 실패합니다.

<h3 id="5-pin-model-versions">
  5. 모델 버전 고정
</h3>

<Warning>
  여러 사용자에게 배포할 때 특정 모델 버전을 고정합니다. 고정하지 않으면 `sonnet` 및 `opus`와 같은 모델 별칭이 Claude Code의 Google Cloud의 Agent Platform용 기본 제공 기본값으로 확인되며, 이는 최신 릴리스보다 뒤떨어질 수 있고 프로젝트에서 아직 활성화되지 않았을 수 있습니다. Claude Code는 기본값을 사용할 수 없을 때 시작 시 [이전 버전으로 폴백](#startup-model-checks)하지만, 고정하면 사용자가 새 모델로 이동하는 시기를 제어할 수 있습니다.
</Warning>

이러한 환경 변수를 특정 Google Cloud의 Agent Platform 모델 ID로 설정합니다.

`ANTHROPIC_DEFAULT_OPUS_MODEL`이 없으면 Google Cloud의 Agent Platform의 `opus` 별칭이 Opus 4.8로 확인되고, `ANTHROPIC_DEFAULT_SONNET_MODEL`이 없으면 `sonnet` 별칭이 Sonnet 4.5로 확인됩니다. 이 예제는 각 별칭을 특정 버전으로 고정합니다:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='claude-sonnet-5'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

현재 및 레거시 모델 ID는 [모델 개요](https://platform.claude.com/docs/en/about-claude/models/overview)를 참조하십시오. 환경 변수의 전체 목록은 [모델 구성](/docs/ko/model-config#pin-models-for-third-party-deployments)을 참조하십시오.

Claude Code는 고정 변수가 설정되지 않았을 때 이러한 기본 모델을 사용합니다:

| 모델 유형    | 기본값                          |
| :------- | :--------------------------- |
| 주 모델     | `claude-opus-4-8`            |
| 소형/빠른 모델 | `claude-sonnet-4-5@20250929` |

백그라운드 작업(예: 세션 제목 생성)은 소형/빠른 모델(일반적으로 Haiku 클래스 모델)을 사용합니다. Google Cloud의 Agent Platform에서 Claude Code는 모든 프로젝트 또는 지역에서 Haiku가 활성화되지 않을 수 있으므로 백그라운드 작업에 기본 Sonnet 모델을 사용합니다. 다음 두 가지 선택이 어떤 모델이 이를 수행하는지 변경합니다:

* `--model`, `ANTHROPIC_MODEL` 또는 `model` 설정으로 주 모델을 선택하면 백그라운드 작업이 해당 모델을 사용합니다. `ANTHROPIC_DEFAULT_SONNET_MODEL` 없이 `ANTHROPIC_DEFAULT_OPUS_MODEL`을 설정하는 것도 선택으로 간주됩니다. 왜냐하면 기본 제공 Sonnet 모델이 자체 Opus를 조정하는 프로젝트에서 활성화되지 않을 수 있기 때문입니다.
* 백그라운드 작업에 Haiku를 사용하려면 `ANTHROPIC_DEFAULT_HAIKU_MODEL`을 프로젝트에서 사용 가능한 모델 ID로 설정합니다.

<Warning>
  Opus 모델은 Sonnet 모델보다 토큰당 가격이 높으므로, 주 모델을 고정하지 않는 배포는 v2.1.207 이상으로 업데이트되면 Opus 요금으로 청구됩니다. Sonnet 4.5를 주 모델로 유지하려면 `ANTHROPIC_MODEL`을 전체 모델 ID로 설정합니다. `ANTHROPIC_DEFAULT_SONNET_MODEL`으로 기본값을 조정하고 `ANTHROPIC_DEFAULT_OPUS_MODEL`을 설정하지 않는 배포는 조정된 Sonnet 모델을 기본값으로 유지합니다.
</Warning>

v2.1.207 이전에는 Google Cloud의 Agent Platform의 주 모델이 기본적으로 Sonnet 4.5였고, `opus` 별칭이 Opus 4.6으로 확인되었으며, 백그라운드 작업은 항상 주 모델을 사용했습니다.

모델을 추가로 사용자 정의하려면:

```bash theme={null}
export ANTHROPIC_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

<h2 id="startup-model-checks">
  시작 모델 확인
</h2>

Claude Code가 Google Cloud의 Agent Platform으로 구성되어 시작할 때 사용하려는 모델이 프로젝트에서 액세스 가능한지 확인합니다.

현재 Claude Code 기본값보다 오래된 모델 버전을 고정했고 프로젝트가 최신 버전을 호출할 수 있으면 Claude Code는 고정을 업데이트하라는 메시지를 표시합니다. 수락하면 새 모델 ID를 [사용자 설정 파일](/docs/ko/settings)에 쓰고 Claude Code를 다시 시작합니다. 거절하면 다음 기본 버전 변경까지 기억됩니다.

모델을 고정하지 않았고 현재 기본값을 프로젝트에서 사용할 수 없으면 Claude Code는 현재 세션에 대해 이전 버전으로 폴백하고 알림을 표시합니다. 기본값이 Opus 모델이고 사용 가능한 Opus 버전이 없으면 기본 Sonnet 모델로 폴백합니다. 폴백은 유지되지 않습니다. [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden)에서 최신 모델을 활성화하거나 [버전을 고정](#5-pin-model-versions)하여 선택을 영구적으로 만듭니다.

<h2 id="iam-configuration">
  IAM 구성
</h2>

필요한 IAM 권한을 할당합니다:

`roles/aiplatform.user` 역할에는 필요한 권한이 포함됩니다:

* `aiplatform.endpoints.predict` - 모델 호출 및 토큰 계산에 필요

더 제한적인 권한의 경우 위의 권한만 포함하는 사용자 정의 역할을 만듭니다.

자세한 내용은 [Google Cloud의 Agent Platform IAM 설명서](https://cloud.google.com/vertex-ai/docs/general/access-control)를 참조하십시오.

<Note>
  비용 추적 및 액세스 제어를 단순화하기 위해 Claude Code용 전용 GCP 프로젝트를 만듭니다.
</Note>

<h2 id="1m-token-context-window">
  1M 토큰 context window
</h2>

Claude Sonnet 5, Opus 4.6 이상 및 Sonnet 4.6은 Google Cloud의 Agent Platform에서 [1M 토큰 context window](https://platform.claude.com/docs/ko/build-with-claude/context-windows#context-window-sizes-by-model)를 지원합니다. Sonnet 5는 항상 1M 윈도우로 실행되며, 선택할 `[1m]` 변형이 없습니다. 다른 모델의 경우, Claude Code는 1M 모델 변형을 선택할 때 확장된 context window를 자동으로 활성화합니다.

[설정 마법사](#sign-in-with-agent-platform)는 모델을 고정할 때 1M context 옵션을 제공합니다. 수동으로 고정된 모델에 대해 대신 활성화하려면 모델 ID에 `[1m]`을 추가합니다. 자세한 내용은 [타사 배포를 위한 모델 고정](/docs/ko/model-config#pin-models-for-third-party-deployments)을 참조하십시오.

<h2 id="troubleshooting">
  문제 해결
</h2>

"기본 자격증명을 로드할 수 없음" 오류가 발생하는 경우:

* `gcloud auth application-default login`을 실행하여 Application Default Credentials를 설정합니다
* `GOOGLE_APPLICATION_CREDENTIALS`를 서비스 계정 키 파일 경로로 설정합니다
* 모든 옵션은 [GCP 자격증명 구성](#3-configure-gcp-credentials)을 참조하세요

할당량 문제가 발생하는 경우:

* [Cloud Console](https://cloud.google.com/docs/quotas/view-manage)을 통해 현재 할당량을 확인하거나 할당량 증가를 요청합니다

"모델을 찾을 수 없음" 404 오류가 발생하는 경우:

* [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden)에서 모델이 활성화되어 있는지 확인합니다
* 지정된 위치에서 모델을 사용할 수 있는지 확인합니다. 일부 모델은 특정 지역이 아닌 `global` 또는 `eu` 및 `us`와 같은 다중 지역 위치에서만 제공됩니다
* `CLOUD_ML_REGION=global`을 사용하는 경우 [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden)의 "지원되는 기능" 아래에서 모델이 전역 엔드포인트를 지원하는지 확인합니다. 전역 엔드포인트를 지원하지 않는 모델의 경우:
  * `ANTHROPIC_MODEL` 또는 `ANTHROPIC_DEFAULT_HAIKU_MODEL`을 통해 지원되는 모델을 지정하거나,
  * `VERTEX_REGION_<MODEL_NAME>` 환경 변수를 사용하여 지역 또는 다중 지역 위치를 설정합니다

429 오류가 발생하는 경우:

* 지역 엔드포인트의 경우 주 모델과 소형/빠른 모델이 선택한 지역에서 지원되는지 확인합니다
* `CLOUD_ML_REGION=global`로 전환하여 더 나은 가용성을 고려합니다

<h2 id="additional-resources">
  추가 리소스
</h2>

* [Google Cloud의 Agent Platform 설명서](https://cloud.google.com/vertex-ai/docs)
* [Google Cloud의 Agent Platform 가격](https://cloud.google.com/vertex-ai/pricing)
* [Google Cloud의 Agent Platform 할당량 및 제한](https://cloud.google.com/vertex-ai/docs/quotas)
