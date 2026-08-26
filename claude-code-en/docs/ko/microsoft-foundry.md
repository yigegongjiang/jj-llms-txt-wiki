> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Microsoft Foundry의 Claude Code

> 설정, 구성 및 문제 해결을 포함하여 Microsoft Foundry를 통해 Claude Code를 구성하는 방법을 알아봅니다.

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

<ContactSalesCard surface="foundry" />

<h2 id="prerequisites">
  필수 조건
</h2>

Microsoft Foundry로 Claude Code를 구성하기 전에 다음을 확인하세요:

* Microsoft Foundry에 액세스할 수 있는 Azure 구독
* Microsoft Foundry 리소스 및 배포를 만들 수 있는 RBAC 권한
* Azure CLI 설치 및 구성(선택 사항 - 자격 증명을 얻을 다른 메커니즘이 없는 경우에만 필요)

<Note>
  Claude Code를 여러 사용자에게 배포하는 경우 [모델 버전을 고정](#4-pin-model-versions)하기 전에 롤아웃하세요.
</Note>

<h2 id="setup">
  설정
</h2>

<h3 id="1-provision-microsoft-foundry-resource">
  1. Microsoft Foundry 리소스 프로비저닝
</h3>

먼저 Azure에서 Claude 리소스를 만듭니다:

1. [Microsoft Foundry 포털](https://ai.azure.com/)로 이동합니다
2. 새 리소스를 만들고 리소스 이름을 기록합니다
3. Claude 모델에 대한 배포를 만들고 각 배포에 지정한 배포 이름을 기록합니다. 4단계에서 이 이름들을 모델 변수로 설정합니다:
   * Claude Opus
   * Claude Sonnet
   * Claude Haiku

<h3 id="2-configure-azure-credentials">
  2) Azure 자격 증명 구성
</h3>

Claude Code는 Microsoft Foundry에 대해 세 가지 인증 방법을 지원합니다. 보안 요구 사항에 가장 적합한 방법을 선택하세요.

**옵션 A: API 키 인증**

1. Microsoft Foundry 포털에서 리소스로 이동합니다
2. **엔드포인트 및 키** 섹션으로 이동합니다
3. **API 키**를 복사합니다
4. 환경 변수를 설정합니다. `your-azure-api-key`를 복사한 키로 바꾸세요:

```bash theme={null}
export ANTHROPIC_FOUNDRY_API_KEY=your-azure-api-key
```

**옵션 B: Microsoft Entra ID 인증**

`ANTHROPIC_FOUNDRY_API_KEY`와 `ANTHROPIC_FOUNDRY_AUTH_TOKEN`이 모두 설정되지 않으면 Claude Code는 자동으로 Azure SDK [기본 자격 증명 체인](https://learn.microsoft.com/en-us/azure/developer/javascript/sdk/authentication/credential-chains#defaultazurecredential-overview)을 사용합니다.
이는 로컬 및 원격 워크로드를 인증하기 위한 다양한 방법을 지원합니다.

로컬 환경에서는 일반적으로 Azure CLI를 사용할 수 있습니다:

```bash theme={null}
az login
```

**옵션 C: Bearer 토큰 인증**

Claude Code는 모든 요청에서 `ANTHROPIC_FOUNDRY_AUTH_TOKEN`의 값을 `Authorization: Bearer` 헤더로 전송합니다. 호스트 애플리케이션이나 로그인 스크립트와 같은 다른 프로세스가 이미 액세스 토큰을 얻은 경우 이 옵션을 사용합니다. Claude Code v2.1.203 이상이 필요합니다.

변수를 Microsoft Entra ID가 리소스에 대해 발급한 bearer 토큰으로 설정합니다:

```bash theme={null}
export ANTHROPIC_FOUNDRY_AUTH_TOKEN=your-entra-access-token
```

`ANTHROPIC_FOUNDRY_AUTH_TOKEN`은 `ANTHROPIC_FOUNDRY_API_KEY`보다 우선하며 기본 자격 증명 체인보다 우선합니다.

<Note>
  Microsoft Foundry를 사용할 때 `/logout` 명령은 Azure 자격 증명을 통해 인증이 처리되므로 사용할 수 없습니다.
</Note>

<h3 id="3-configure-claude-code">
  3. Claude Code 구성
</h3>

Microsoft Foundry를 활성화하려면 다음 환경 변수를 설정합니다:

```bash theme={null}
# Microsoft Foundry 통합 활성화
export CLAUDE_CODE_USE_FOUNDRY=1

# Azure 리소스 이름 ({resource}를 리소스 이름으로 바꾸기)
export ANTHROPIC_FOUNDRY_RESOURCE={resource}
# 또는 전체 기본 URL 제공:
# export ANTHROPIC_FOUNDRY_BASE_URL=https://{resource}.services.ai.azure.com/anthropic
```

<h3 id="4-pin-model-versions">
  4. 모델 버전 고정
</h3>

<Warning>
  모든 배포에 대해 특정 모델 버전을 고정합니다. 고정하지 않으면 `sonnet` 및 `opus`와 같은 모델 별칭이 Claude Code의 Microsoft Foundry용 기본 제공 기본값으로 확인되며, 이는 최신 릴리스보다 뒤떨어질 수 있고 계정에서 아직 사용할 수 없을 수 있습니다. Microsoft Foundry는 시작 모델 확인이 없으므로 기본값을 사용할 수 없을 때 요청이 실패합니다. Azure 배포를 만들 때 "최신으로 자동 업데이트" 대신 특정 모델 버전을 선택합니다.
</Warning>

모델 변수를 1단계에서 만든 배포 이름과 일치하도록 설정합니다.

`ANTHROPIC_DEFAULT_OPUS_MODEL`이 없으면 Microsoft Foundry의 `opus` 별칭은 Opus 4.6으로 확인됩니다. 최신 모델을 사용하려면 Opus 4.8 ID로 설정합니다:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='claude-sonnet-5'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5'
```

세션 제목 생성과 같은 백그라운드 작업은 일반적으로 Haiku 클래스 모델인 소형/빠른 모델을 사용합니다. Microsoft Foundry에서 Claude Code는 모든 계정이 Haiku 배포를 가지고 있지 않기 때문에 기본 모델로 기본 설정됩니다. 백그라운드 작업에 Haiku를 사용하려면 위에 표시된 대로 `ANTHROPIC_DEFAULT_HAIKU_MODEL`을 계정에서 사용 가능한 Haiku 배포로 설정합니다.

현재 및 레거시 모델 ID는 [모델 개요](https://platform.claude.com/docs/en/about-claude/models/overview)를 참조하세요. 전체 환경 변수 목록은 [모델 구성](/docs/ko/model-config#pin-models-for-third-party-deployments)을 참조하세요.

[Prompt caching](/docs/ko/prompt-caching)은 자동으로 활성화됩니다. 기본 5분 대신 1시간 캐시 TTL을 요청하려면 다음 변수를 설정합니다. 1시간 TTL로 캐시 쓰기는 더 높은 요금으로 청구됩니다:

```bash theme={null}
export ENABLE_PROMPT_CACHING_1H=1
```

<h3 id="5-run-claude-code">
  5. Claude Code 실행
</h3>

환경 변수가 설정되면 프로젝트 디렉터리에서 Claude Code를 시작합니다:

```bash theme={null}
claude
```

Claude Code는 환경에서 `CLAUDE_CODE_USE_FOUNDRY` 및 기타 Microsoft Foundry 변수를 읽고 첫 번째 프롬프트에서 Azure 리소스에 연결합니다. Amazon Bedrock 및 Google Cloud의 Agent Platform과 달리 Microsoft Foundry는 대화형 설정 마법사가 없으므로 3단계와 4단계의 환경 변수가 유일한 구성 경로입니다.

설정을 확인하려면 Claude Code 내에서 `/status`를 실행합니다. API 제공자 라인은 구성한 리소스 이름 또는 기본 URL과 함께 `Microsoft Foundry`를 표시합니다.

<h2 id="azure-rbac-configuration">
  Azure RBAC 구성
</h2>

`Azure AI User` 및 `Cognitive Services User` 기본 역할에는 Claude 모델을 호출하는 데 필요한 모든 권한이 포함됩니다.

더 제한적인 권한의 경우 다음을 포함하는 사용자 지정 역할을 만듭니다:

```json theme={null}
{
  "permissions": [
    {
      "dataActions": [
        "Microsoft.CognitiveServices/accounts/providers/*"
      ]
    }
  ]
}
```

자세한 내용은 [Microsoft Foundry RBAC 설명서](https://learn.microsoft.com/en-us/azure/ai-foundry/concepts/rbac-azure-ai-foundry)를 참조하세요.

<h2 id="troubleshooting">
  문제 해결
</h2>

"Failed to get token from azureADTokenProvider: ChainedTokenCredential authentication failed" 오류가 발생하면:

* 환경에서 Entra ID를 구성하거나 `ANTHROPIC_FOUNDRY_API_KEY`를 설정합니다.

첫 번째 프롬프트에서 반복되는 연결 오류로 요청이 실패하면:

* `ANTHROPIC_FOUNDRY_RESOURCE`가 자리 표시자가 아닌 실제 리소스 이름으로 설정되어 있는지 확인합니다. Claude Code는 이 값에서 엔드포인트 URL을 구성하므로 잘못된 이름은 존재하지 않는 호스트를 가리킵니다.

<h2 id="additional-resources">
  추가 리소스
</h2>

* [Microsoft Foundry 설명서](https://learn.microsoft.com/en-us/azure/ai-foundry/what-is-azure-ai-foundry)
* [Microsoft Foundry 모델](https://ai.azure.com/explore/models)
* [Microsoft Foundry 가격](https://azure.microsoft.com/en-us/pricing/details/ai-foundry/)
