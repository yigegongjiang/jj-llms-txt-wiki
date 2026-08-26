> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 엔터프라이즈 배포 개요

> Claude Code가 다양한 타사 서비스 및 인프라와 통합되어 엔터프라이즈 배포 요구사항을 충족하는 방법을 알아봅니다.

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

조직은 Anthropic을 통해 직접 또는 클라우드 제공자를 통해 Claude Code를 배포할 수 있습니다. 이 페이지는 올바른 구성을 선택하는 데 도움을 줍니다.

<ContactSalesCard surface="third_party_overview" />

<h2 id="compare-deployment-options">
  배포 옵션 비교
</h2>

대부분의 조직에서는 Claude for Teams 또는 Claude for Enterprise가 최고의 경험을 제공합니다. 팀 멤버는 단일 구독으로 Claude Code와 웹의 Claude에 모두 액세스할 수 있으며, 중앙 집중식 청구 및 인프라 설정이 필요하지 않습니다.

**Claude for Teams**는 셀프 서비스이며 협업 기능, 관리 도구 및 청구 관리를 포함합니다. 빠르게 시작해야 하는 소규모 팀에 최적입니다.

**Claude for Enterprise**는 SSO 및 도메인 캡처, 역할 기반 권한, 규정 준수 API 액세스 및 조직 전체 Claude Code 구성을 배포하기 위한 관리형 정책 설정을 추가합니다. 보안 및 규정 준수 요구사항이 있는 대규모 조직에 최적입니다.

[팀 플랜](https://support.claude.com/ko/articles/9266767-what-is-the-team-plan) 및 [엔터프라이즈 플랜](https://support.claude.com/ko/articles/9797531-what-is-the-enterprise-plan)에 대해 자세히 알아봅니다.

조직에 특정 인프라 요구사항이 있는 경우 아래 옵션을 비교하십시오:

<table>
  <thead>
    <tr>
      <th>기능</th>
      <th>Claude for Teams/Enterprise</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>Claude Platform on AWS</th>
      <th>Google Cloud의 Agent Platform, 이전의 Vertex AI</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>최적 용도</td>
      <td>대부분의 조직 (권장)</td>
      <td>개별 개발자</td>
      <td>AWS 네이티브 배포</td>
      <td>Claude API 기능이 있는 AWS Marketplace 청구</td>
      <td>GCP 네이티브 배포</td>
      <td>Azure 네이티브 배포</td>
    </tr>

    <tr>
      <td>청구</td>
      <td><strong>Teams:</strong> \$150/seat (Premium) PAYG 사용 가능<br /><strong>Enterprise:</strong> <a href="https://claude.com/contact-sales?utm_source=claude_code&utm_medium=docs&utm_content=third_party_enterprise">영업팀에 문의</a></td>
      <td>PAYG</td>
      <td>AWS를 통한 PAYG</td>
      <td>AWS Marketplace를 통한 PAYG</td>
      <td>GCP를 통한 PAYG</td>
      <td>Azure를 통한 PAYG</td>
    </tr>

    <tr>
      <td>지역</td>
      <td>지원되는 [국가](https://www.anthropic.com/supported-countries)</td>
      <td>지원되는 [국가](https://www.anthropic.com/supported-countries)</td>
      <td>여러 AWS [지역](https://docs.aws.amazon.com/bedrock/latest/userguide/models-regions.html)</td>
      <td>여러 AWS 지역</td>
      <td>여러 GCP [지역](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations)</td>
      <td>여러 Azure [지역](https://azure.microsoft.com/en-us/explore/global-infrastructure/products-by-region/)</td>
    </tr>

    <tr>
      <td>Prompt caching</td>
      <td>기본적으로 활성화됨</td>
      <td>기본적으로 활성화됨</td>
      <td>기본적으로 활성화됨</td>
      <td>기본적으로 활성화됨</td>
      <td>기본적으로 활성화됨</td>
      <td>기본적으로 활성화됨</td>
    </tr>

    <tr>
      <td>인증</td>
      <td>Claude.ai SSO 또는 이메일</td>
      <td>API 키</td>
      <td>API 키 또는 AWS 자격증명</td>
      <td>API 키 또는 AWS 자격증명</td>
      <td>GCP 자격증명</td>
      <td>API 키 또는 Microsoft Entra ID</td>
    </tr>

    <tr>
      <td>비용 추적</td>
      <td>사용량 대시보드</td>
      <td>사용량 대시보드</td>
      <td>AWS Cost Explorer</td>
      <td>AWS Cost Explorer</td>
      <td>GCP 청구</td>
      <td>Azure Cost Management</td>
    </tr>

    <tr>
      <td>웹의 Claude 포함</td>
      <td>예</td>
      <td>아니오</td>
      <td>아니오</td>
      <td>아니오</td>
      <td>아니오</td>
      <td>아니오</td>
    </tr>

    <tr>
      <td>엔터프라이즈 기능</td>
      <td>팀 관리, SSO, 사용량 모니터링</td>
      <td>없음</td>
      <td>IAM 정책, CloudTrail</td>
      <td>IAM 정책, CloudTrail</td>
      <td>IAM 역할, Cloud Audit Logs</td>
      <td>RBAC 정책, Azure Monitor</td>
    </tr>
  </tbody>
</table>

각 옵션에서 사용 가능한 기능에 대한 기능별 분석은 [기능 가용성](/docs/ko/feature-availability)을 참조하십시오.

배포 옵션을 선택하여 설정 지침을 확인하십시오:

* [Claude for Teams 또는 Enterprise](/docs/ko/authentication#claude-for-teams-or-enterprise)
* [Anthropic Console](/docs/ko/authentication#claude-console-authentication)
* [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway), Amazon Bedrock, Claude Platform on AWS, Google Cloud의 Agent Platform, Microsoft Foundry 또는 Anthropic API 앞에 IdP 로그인을 추가하는 자체 호스팅 게이트웨이
* [Amazon Bedrock](/docs/ko/amazon-bedrock)
* [Claude Platform on AWS](/docs/ko/claude-platform-on-aws)
* [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai)
* [Microsoft Foundry](/docs/ko/microsoft-foundry)

Amazon Bedrock 및 Google Vertex AI의 경우 `claude`를 실행하고 로그인 프롬프트에서 **3rd-party platform**을 선택하여 대화형 설정 마법사를 시작할 수도 있습니다.

<h2 id="configure-proxies-and-gateways">
  프록시 및 게이트웨이 구성
</h2>

대부분의 조직은 추가 구성 없이 클라우드 제공자를 직접 사용할 수 있습니다. 그러나 조직에 특정 네트워크 또는 관리 요구사항이 있는 경우 회사 프록시 또는 LLM 게이트웨이를 구성해야 할 수 있습니다. 이는 함께 사용할 수 있는 다양한 구성입니다:

* **회사 프록시**: HTTP/HTTPS 프록시를 통해 트래픽을 라우팅합니다. 조직에서 보안 모니터링, 규정 준수 또는 네트워크 정책 적용을 위해 모든 아웃바운드 트래픽이 프록시 서버를 통과해야 하는 경우 이를 사용하십시오. `HTTPS_PROXY` 또는 `HTTP_PROXY` 환경 변수로 구성합니다. [엔터프라이즈 네트워크 구성](/docs/ko/network-config)에서 자세히 알아봅니다.
* **LLM 게이트웨이**: Claude Code와 클라우드 제공자 사이에 위치하여 인증 및 라우팅을 처리하는 서비스입니다. 팀 전체에서 중앙 집중식 사용량 추적, 사용자 정의 속도 제한 또는 예산, 또는 중앙 집중식 인증 관리가 필요한 경우 이를 사용하십시오. `ANTHROPIC_BASE_URL`, `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_AWS_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL`, 또는 `ANTHROPIC_FOUNDRY_BASE_URL` 환경 변수로 구성합니다. [LLM 게이트웨이](/docs/ko/llm-gateway)에서 자세히 알아봅니다.

다음 예제는 셸 또는 셸 프로필(`.bashrc`, `.zshrc`)에서 설정할 환경 변수를 보여줍니다. 다른 구성 방법은 [설정](/docs/ko/settings)을 참조하십시오.

<h3 id="amazon-bedrock">
  Amazon Bedrock
</h3>

<Tabs>
  <Tab title="회사 프록시">
    다음 [환경 변수](/docs/ko/env-vars)를 설정하여 Amazon Bedrock 트래픽을 회사 프록시를 통해 라우팅합니다:

    ```bash theme={null}
    # Bedrock 활성화
    export CLAUDE_CODE_USE_BEDROCK=1
    export AWS_REGION=us-east-1

    # 회사 프록시 구성
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="LLM 게이트웨이">
    다음 [환경 변수](/docs/ko/env-vars)를 설정하여 Amazon Bedrock 트래픽을 LLM 게이트웨이를 통해 라우팅합니다:

    ```bash theme={null}
    # Bedrock 활성화
    export CLAUDE_CODE_USE_BEDROCK=1

    # LLM 게이트웨이 구성
    export ANTHROPIC_BEDROCK_BASE_URL='https://your-llm-gateway.com/bedrock'
    export CLAUDE_CODE_SKIP_BEDROCK_AUTH=1  # 게이트웨이가 AWS 인증을 처리하는 경우
    ```
  </Tab>
</Tabs>

<h3 id="microsoft-foundry">
  Microsoft Foundry
</h3>

<Tabs>
  <Tab title="회사 프록시">
    다음 [환경 변수](/docs/ko/env-vars)를 설정하여 Microsoft Foundry 트래픽을 회사 프록시를 통해 라우팅합니다:

    ```bash theme={null}
    # Microsoft Foundry 활성화
    export CLAUDE_CODE_USE_FOUNDRY=1
    export ANTHROPIC_FOUNDRY_RESOURCE=your-resource
    export ANTHROPIC_FOUNDRY_API_KEY=your-api-key  # 또는 Entra ID 인증의 경우 생략

    # 회사 프록시 구성
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="LLM 게이트웨이">
    다음 [환경 변수](/docs/ko/env-vars)를 설정하여 Microsoft Foundry 트래픽을 LLM 게이트웨이를 통해 라우팅합니다:

    ```bash theme={null}
    # Microsoft Foundry 활성화
    export CLAUDE_CODE_USE_FOUNDRY=1

    # LLM 게이트웨이 구성
    export ANTHROPIC_FOUNDRY_BASE_URL='https://your-llm-gateway.com'
    export ANTHROPIC_FOUNDRY_API_KEY=your-gateway-key  # x-api-key로 전송됨
    ```
  </Tab>
</Tabs>

<h3 id="google-cloud’s-agent-platform">
  Google Cloud's Agent Platform
</h3>

<Tabs>
  <Tab title="회사 프록시">
    다음 [환경 변수](/docs/ko/env-vars)를 설정하여 Google Cloud's Agent Platform 트래픽을 회사 프록시를 통해 라우팅합니다:

    ```bash theme={null}
    # Agent Platform 활성화
    export CLAUDE_CODE_USE_VERTEX=1
    export CLOUD_ML_REGION=us-east5
    export ANTHROPIC_VERTEX_PROJECT_ID=your-project-id

    # 회사 프록시 구성
    export HTTPS_PROXY='https://proxy.example.com:8080'
    ```
  </Tab>

  <Tab title="LLM 게이트웨이">
    다음 [환경 변수](/docs/ko/env-vars)를 설정하여 Google Cloud's Agent Platform 트래픽을 LLM 게이트웨이를 통해 라우팅합니다:

    ```bash theme={null}
    # Agent Platform 활성화
    export CLAUDE_CODE_USE_VERTEX=1

    # LLM 게이트웨이 구성
    export ANTHROPIC_VERTEX_BASE_URL='https://your-llm-gateway.com/vertex'
    export CLAUDE_CODE_SKIP_VERTEX_AUTH=1  # 게이트웨이가 GCP 인증을 처리하는 경우
    export ANTHROPIC_VERTEX_PROJECT_ID=your-gcp-project-id
    export CLOUD_ML_REGION=us-east5
    ```
  </Tab>
</Tabs>

<Tip>
  Claude Code에서 `/status`를 사용하여 프록시 및 게이트웨이 구성이 올바르게 적용되었는지 확인합니다. 예를 들어, 위의 Bedrock 게이트웨이 구성을 사용하면 출력에 다음과 같은 줄이 포함됩니다:

  ```
  API provider: Amazon Bedrock
  Bedrock base URL: https://your-llm-gateway.com/bedrock
  AWS region: us-east-1
  AWS auth skipped
  ```

  회사 프록시를 구성한 경우 `/status`는 프록시 URL이 포함된 `Proxy` 줄도 표시합니다.
</Tip>

<h2 id="best-practices-for-organizations">
  조직을 위한 모범 사례
</h2>

<h3 id="invest-in-documentation-and-memory">
  문서 및 메모리에 투자
</h3>

Claude Code가 코드베이스를 이해할 수 있도록 문서에 투자할 것을 강력히 권장합니다. 조직은 여러 수준에서 CLAUDE.md 파일을 배포할 수 있습니다:

* **조직 전체**: 회사 전체 표준을 위해 `/Library/Application Support/ClaudeCode/CLAUDE.md` (macOS), `/etc/claude-code/CLAUDE.md` (Linux 및 WSL) 또는 `C:\Program Files\ClaudeCode\CLAUDE.md` (Windows)와 같은 시스템 디렉토리에 배포
* **저장소 수준**: 프로젝트 아키텍처, 빌드 명령 및 기여 지침을 포함하는 저장소 루트에 `CLAUDE.md` 파일을 만듭니다. 이를 소스 제어에 체크인하여 모든 사용자가 이점을 얻을 수 있도록 합니다.

[메모리 및 CLAUDE.md 파일](/docs/ko/memory)에서 자세히 알아봅니다.

<h3 id="simplify-deployment">
  배포 단순화
</h3>

사용자 정의 개발 환경이 있는 경우 Claude Code를 설치하는 "원클릭" 방법을 만드는 것이 조직 전체에서 채택을 늘리는 핵심이라는 것을 알았습니다.

<h3 id="start-with-guided-usage">
  안내된 사용으로 시작
</h3>

새 사용자가 코드베이스 Q\&A 또는 더 작은 버그 수정 또는 기능 요청에 Claude Code를 시도하도록 권장합니다. Claude Code에 계획을 세우도록 요청합니다. Claude의 제안을 확인하고 잘못된 경우 피드백을 제공합니다. 시간이 지남에 따라 사용자가 이 새로운 패러다임을 더 잘 이해하게 되면 Claude Code를 더 에이전트적으로 실행하는 데 더 효과적이 될 것입니다.

<h3 id="pin-model-versions-for-cloud-providers">
  클라우드 제공자를 위한 모델 버전 고정
</h3>

[Amazon Bedrock](/docs/ko/amazon-bedrock), [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai), [Microsoft Foundry](/docs/ko/microsoft-foundry) 또는 [Claude Platform on AWS](/docs/ko/claude-platform-on-aws)를 통해 배포하는 경우 `ANTHROPIC_DEFAULT_FABLE_MODEL`, `ANTHROPIC_DEFAULT_OPUS_MODEL`, `ANTHROPIC_DEFAULT_SONNET_MODEL` 및 `ANTHROPIC_DEFAULT_HAIKU_MODEL`을 사용하여 특정 모델 버전을 고정합니다. 고정하지 않으면 모델 별칭이 Claude Code의 해당 제공자에 대한 기본 제공 기본값으로 확인되며, 이는 최신 릴리스보다 뒤떨어질 수 있고 계정에서 아직 활성화되지 않았을 수 있습니다. 고정하면 사용자가 새 모델로 이동하는 시기를 제어할 수 있습니다. 각 제공자가 기본값을 사용할 수 없을 때 수행하는 작업에 대해서는 [모델 구성](/docs/ko/model-config#pin-models-for-third-party-deployments)을 참조하십시오.

<h3 id="configure-security-policies">
  보안 정책 구성
</h3>

보안 팀은 Claude Code가 수행할 수 있고 수행할 수 없는 작업에 대한 관리형 권한을 구성할 수 있으며, 이는 로컬 구성으로 덮어쓸 수 없습니다. [자세히 알아봅니다](/docs/ko/security).

<h3 id="leverage-mcp-for-integrations">
  MCP를 통합에 활용
</h3>

MCP는 Claude Code에 더 많은 정보를 제공하는 좋은 방법입니다. 예를 들어 티켓 관리 시스템 또는 오류 로그에 연결할 수 있습니다. 한 중앙 팀이 MCP 서버를 구성하고 `.mcp.json` 구성을 코드베이스에 체크인하여 모든 사용자가 이점을 얻을 수 있도록 할 것을 권장합니다. [자세히 알아봅니다](/docs/ko/mcp).

Anthropic에서는 Claude Code를 신뢰하여 모든 Anthropic 코드베이스에서 개발을 강화합니다. Claude Code를 우리만큼 즐기시기를 바랍니다.

<h2 id="next-steps">
  다음 단계
</h2>

배포 옵션을 선택하고 팀에 대한 액세스를 구성한 후:

1. **팀에 롤아웃**: 설치 지침을 공유하고 팀 멤버가 [Claude Code를 설치](/docs/ko/setup)하고 자신의 자격증명으로 인증하도록 합니다.
2. **공유 구성 설정**: 저장소에 [CLAUDE.md 파일](/docs/ko/memory)을 만들어 Claude Code가 코드베이스 및 코딩 표준을 이해하도록 도와줍니다.
3. **권한 구성**: [보안 설정](/docs/ko/security)을 검토하여 Claude Code가 환경에서 수행할 수 있고 수행할 수 없는 작업을 정의합니다.
