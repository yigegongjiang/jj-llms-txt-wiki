> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitLab CI/CD

> Claude Code를 GitLab CI/CD와 함께 개발 워크플로우에 통합하는 방법을 알아봅니다

<Info>
  Claude Code for GitLab CI/CD는 현재 베타 버전입니다. 경험을 개선하면서 기능과 기능성이 진화할 수 있습니다.

  이 통합은 GitLab에서 유지 관리합니다. 지원을 받으려면 다음 [GitLab 이슈](https://gitlab.com/gitlab-org/gitlab/-/issues/573776)를 참조하세요.
</Info>

<Note>
  이 통합은 [Claude Code CLI and Agent SDK](/docs/ko/agent-sdk/overview) 위에 구축되어 있으며, CI/CD 작업 및 사용자 정의 자동화 워크플로우에서 Claude를 프로그래밍 방식으로 사용할 수 있습니다.
</Note>

<h2 id="why-use-claude-code-with-gitlab">
  GitLab에서 Claude Code를 사용하는 이유
</h2>

* **즉시 MR 생성**: 필요한 사항을 설명하면 Claude가 변경 사항과 설명이 포함된 완전한 MR을 제안합니다
* **자동화된 구현**: 단일 명령 또는 언급으로 이슈를 작동하는 코드로 변환합니다
* **프로젝트 인식**: Claude는 `CLAUDE.md` 지침과 기존 코드 패턴을 따릅니다
* **간단한 설정**: `.gitlab-ci.yml`에 하나의 작업과 마스킹된 CI/CD 변수를 추가합니다
* **엔터프라이즈 준비**: Claude API, Amazon Bedrock 또는 Google Cloud의 Agent Platform을 선택하여 데이터 거주지 및 조달 요구 사항을 충족합니다
* **기본적으로 안전**: GitLab 러너에서 실행되며 브랜치 보호 및 승인이 적용됩니다

<h2 id="how-it-works">
  작동 방식
</h2>

Claude Code는 GitLab CI/CD를 사용하여 격리된 작업에서 AI 작업을 실행하고 MR을 통해 결과를 다시 커밋합니다:

1. **이벤트 기반 오케스트레이션**: GitLab은 선택한 트리거(예: 이슈, MR 또는 검토 스레드에서 `@claude`를 언급하는 댓글)를 수신합니다. 작업은 스레드 및 저장소에서 컨텍스트를 수집하고, 해당 입력에서 프롬프트를 작성하고, Claude Code를 실행합니다.

2. **공급자 추상화**: 환경에 맞는 공급자를 사용합니다:
   * Claude API (SaaS)
   * Amazon Bedrock (IAM 기반 액세스, 교차 지역 옵션)
   * Google Cloud의 Agent Platform (GCP 네이티브, Workload Identity Federation)

3. **샌드박스 실행**: 각 상호 작용은 엄격한 네트워크 및 파일 시스템 규칙이 있는 컨테이너에서 실행됩니다. Claude Code는 쓰기를 제한하기 위해 작업 공간 범위 권한을 적용합니다. 모든 변경 사항은 MR을 통해 흐르므로 검토자가 diff를 보고 승인이 여전히 적용됩니다.

지역 엔드포인트를 선택하여 지연 시간을 줄이고 기존 클라우드 계약을 사용하면서 데이터 주권 요구 사항을 충족합니다.

<h2 id="what-can-claude-do">
  Claude가 할 수 있는 것
</h2>

Claude Code는 코드 작업 방식을 변환하는 강력한 CI/CD 워크플로우를 활성화합니다:

* 이슈 설명 또는 댓글에서 MR 생성 및 업데이트
* 성능 회귀 분석 및 최적화 제안
* 브랜치에 직접 기능 구현 후 MR 열기
* 테스트 또는 댓글로 식별된 버그 및 회귀 수정
* 후속 댓글에 응답하여 요청된 변경 사항에 대해 반복

<h2 id="setup">
  설정
</h2>

<h3 id="quick-setup">
  빠른 설정
</h3>

가장 빠른 시작 방법은 `.gitlab-ci.yml`에 최소 작업을 추가하고 API 키를 마스킹된 변수로 설정하는 것입니다.

1. **마스킹된 CI/CD 변수 추가**
   * **설정** → **CI/CD** → **변수**로 이동합니다
   * `ANTHROPIC_API_KEY` 추가 (마스킹됨, 필요에 따라 보호됨)

2. **`.gitlab-ci.yml`에 Claude 작업 추가**

```yaml theme={null}
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  # 작업을 트리거하는 방법에 맞게 규칙을 조정합니다:
  # - 수동 실행
  # - 병합 요청 이벤트
  # - '@claude'를 포함하는 댓글이 있을 때 웹/API 트리거
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - curl -fsSL https://claude.ai/install.sh | bash
  script:
    # 선택 사항: 설정에서 제공하는 경우 GitLab MCP 서버 시작
    - /bin/gitlab-mcp-server || true
    # 웹/API 트리거를 통해 컨텍스트 페이로드로 호출할 때 AI_FLOW_* 변수 사용
    - echo "$AI_FLOW_INPUT for $AI_FLOW_CONTEXT on $AI_FLOW_EVENT"
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Review this MR and implement the requested changes'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
```

작업과 `ANTHROPIC_API_KEY` 변수를 추가한 후 **CI/CD** → **파이프라인**에서 작업을 수동으로 실행하여 테스트하거나, MR에서 트리거하여 Claude가 브랜치에서 업데이트를 제안하고 필요한 경우 MR을 열도록 합니다.

<Note>
  Claude API 대신 Amazon Bedrock 또는 Google Cloud의 Agent Platform에서 실행하려면 아래의 [Amazon Bedrock 및 Google Cloud 사용](#using-with-amazon-bedrock-and-google-cloud) 섹션을 참조하여 인증 및 환경 설정을 확인하세요.
</Note>

<h3 id="manual-setup-recommended-for-production">
  수동 설정 (프로덕션에 권장)
</h3>

더 제어된 설정을 선호하거나 엔터프라이즈 공급자가 필요한 경우:

1. **공급자 액세스 구성**:
   * **Claude API**: `ANTHROPIC_API_KEY`를 생성하고 마스킹된 CI/CD 변수로 저장합니다
   * **Amazon Bedrock**: **GitLab 구성** → **AWS OIDC**를 구성하고 Amazon Bedrock용 IAM 역할을 생성합니다
   * **Google Cloud의 Agent Platform**: **GitLab용 Workload Identity Federation 구성** → **GCP**

2. **GitLab API 작업을 위한 프로젝트 자격 증명 추가**:
   * 기본적으로 `CI_JOB_TOKEN`을 사용하거나 `api` 범위가 있는 프로젝트 액세스 토큰을 생성합니다
   * PAT를 사용하는 경우 `GITLAB_ACCESS_TOKEN` (마스킹됨)으로 저장합니다

3. **Claude 작업을 `.gitlab-ci.yml`에 추가** (아래 예제 참조)

4. **(선택 사항) 언급 기반 트리거 활성화**:
   * 이벤트 리스너에 "댓글 (노트)"에 대한 프로젝트 웹훅을 추가합니다 (사용하는 경우)
   * 댓글에 `@claude`가 포함될 때 `AI_FLOW_INPUT` 및 `AI_FLOW_CONTEXT`와 같은 변수로 파이프라인 트리거 API를 호출하도록 리스너를 설정합니다

<h2 id="example-use-cases">
  예제 사용 사례
</h2>

<h3 id="turn-issues-into-mrs">
  이슈를 MR로 변환
</h3>

이슈 댓글에서:

```text theme={null}
@claude implement this feature based on the issue description
```

Claude는 이슈 및 코드베이스를 분석하고, 브랜치에서 변경 사항을 작성하고, 검토를 위해 MR을 엽니다.

<h3 id="get-implementation-help">
  구현 도움 받기
</h3>

MR 토론에서:

```text theme={null}
@claude suggest a concrete approach to cache the results of this API call
```

Claude는 변경 사항을 제안하고, 적절한 캐싱으로 코드를 추가하고, MR을 업데이트합니다.

<h3 id="fix-bugs-quickly">
  버그 빠르게 수정
</h3>

이슈 또는 MR 댓글에서:

```text theme={null}
@claude fix the TypeError in the user dashboard component
```

Claude는 버그를 찾고, 수정을 구현하고, 브랜치를 업데이트하거나 새 MR을 엽니다.

<h2 id="using-with-amazon-bedrock-and-google-cloud">
  Amazon Bedrock 및 Google Cloud 사용
</h2>

엔터프라이즈 환경의 경우 동일한 개발자 경험으로 클라우드 인프라에서 완전히 Claude Code를 실행할 수 있습니다.

<Tabs>
  <Tab title="Amazon Bedrock">
    ### 필수 조건

    Amazon Bedrock으로 Claude Code를 설정하기 전에 다음이 필요합니다:

    1. 원하는 Claude 모델에 대한 Amazon Bedrock 액세스가 있는 AWS 계정
    2. AWS IAM에서 OIDC 자격 증명 공급자로 구성된 GitLab
    3. Amazon Bedrock 권한이 있는 IAM 역할 및 GitLab 프로젝트/참조로 제한된 신뢰 정책
    4. 역할 가정을 위한 GitLab CI/CD 변수:
       * `AWS_ROLE_TO_ASSUME` (역할 ARN)
       * `AWS_REGION` (Amazon Bedrock 지역)

    ### 설정 지침

    OIDC를 통해 GitLab CI 작업이 IAM 역할을 가정하도록 AWS를 구성합니다 (정적 키 없음).

    **필수 설정:**

    1. Amazon Bedrock을 활성화하고 대상 Claude 모델에 대한 액세스를 요청합니다
    2. 아직 없는 경우 GitLab용 IAM OIDC 공급자를 생성합니다
    3. GitLab OIDC 공급자를 신뢰하고 프로젝트 및 보호된 참조로 제한된 IAM 역할을 생성합니다
    4. Amazon Bedrock 호출 API에 대한 최소 권한 권한을 연결합니다

    **CI/CD 변수에 저장할 필수 값:**

    * `AWS_ROLE_TO_ASSUME`
    * `AWS_REGION`

    설정 → CI/CD → 변수에서 변수를 추가합니다:

    ```yaml theme={null}
    # Amazon Bedrock의 경우:
    - AWS_ROLE_TO_ASSUME
    - AWS_REGION
    ```

    위의 Amazon Bedrock 작업 예제를 사용하여 런타임에 GitLab 작업 토큰을 임시 AWS 자격 증명으로 교환합니다.
  </Tab>

  <Tab title="Google Cloud의 Agent Platform">
    ### 필수 조건

    Google Cloud의 Agent Platform으로 Claude Code를 설정하기 전에 다음이 필요합니다:

    1. 다음이 포함된 Google Cloud 프로젝트:
       * Google Cloud의 Agent Platform API 활성화됨
       * GitLab OIDC를 신뢰하도록 구성된 Workload Identity Federation
    2. 필요한 Google Cloud의 Agent Platform 역할만 있는 전용 서비스 계정
    3. WIF용 GitLab CI/CD 변수:
       * `GCP_WORKLOAD_IDENTITY_PROVIDER` (전체 리소스 이름)
       * `GCP_SERVICE_ACCOUNT` (서비스 계정 이메일)

    ### 설정 지침

    Workload Identity Federation을 통해 GitLab CI 작업이 서비스 계정을 가장하도록 Google Cloud를 구성합니다.

    **필수 설정:**

    1. IAM Credentials API, STS API 및 Google Cloud의 Agent Platform API 활성화
    2. GitLab OIDC용 Workload Identity Pool 및 공급자 생성
    3. Google Cloud의 Agent Platform 역할이 있는 전용 서비스 계정 생성
    4. WIF 주체에 서비스 계정을 가장할 수 있는 권한 부여

    **CI/CD 변수에 저장할 필수 값:**

    * `GCP_WORKLOAD_IDENTITY_PROVIDER`
    * `GCP_SERVICE_ACCOUNT`

    설정 → CI/CD → 변수에서 변수를 추가합니다:

    ```yaml theme={null}
    # Google Cloud의 Agent Platform의 경우:
    - GCP_WORKLOAD_IDENTITY_PROVIDER
    - GCP_SERVICE_ACCOUNT
    - CLOUD_ML_REGION (예: us-east5)
    ```

    위의 Google Cloud의 Agent Platform 작업 예제를 사용하여 키를 저장하지 않고 인증합니다.
  </Tab>
</Tabs>

<h2 id="configuration-examples">
  구성 예제
</h2>

파이프라인에 맞게 조정할 수 있는 즉시 사용 가능한 스니펫입니다.

<h3 id="basic-gitlab-ci-yml-claude-api">
  기본 .gitlab-ci.yml (Claude API)
</h3>

```yaml theme={null}
stages:
  - ai

claude:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  variables:
    GIT_STRATEGY: fetch
  before_script:
    - apk update
    - apk add --no-cache git curl bash
    - curl -fsSL https://claude.ai/install.sh | bash
  script:
    - /bin/gitlab-mcp-server || true
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Summarize recent changes and suggest improvements'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  # Claude Code는 CI/CD 변수에서 ANTHROPIC_API_KEY를 사용합니다
```

<h3 id="amazon-bedrock-job-example-oidc">
  Amazon Bedrock 작업 예제 (OIDC)
</h3>

**필수 조건:**

* Amazon Bedrock이 활성화되고 선택한 Claude 모델에 액세스 가능
* GitLab OIDC가 AWS에 구성되고 GitLab 프로젝트 및 참조를 신뢰하는 역할
* Amazon Bedrock 권한이 있는 IAM 역할 (최소 권한 권장)

**필수 CI/CD 변수:**

* `AWS_ROLE_TO_ASSUME`: Amazon Bedrock 액세스용 IAM 역할의 ARN
* `AWS_REGION`: Amazon Bedrock 지역 (예: `us-west-2`)

```yaml theme={null}
claude-bedrock:
  stage: ai
  image: node:24-alpine3.21
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  before_script:
    - apk add --no-cache bash curl jq git python3 py3-pip
    - pip install --no-cache-dir awscli
    - curl -fsSL https://claude.ai/install.sh | bash
    # GitLab OIDC 토큰을 AWS 자격 증명으로 교환
    - export AWS_WEB_IDENTITY_TOKEN_FILE="${CI_JOB_JWT_FILE:-/tmp/oidc_token}"
    - if [ -n "${CI_JOB_JWT_V2}" ]; then printf "%s" "$CI_JOB_JWT_V2" > "$AWS_WEB_IDENTITY_TOKEN_FILE"; fi
    - >
      aws sts assume-role-with-web-identity
      --role-arn "$AWS_ROLE_TO_ASSUME"
      --role-session-name "gitlab-claude-$(date +%s)"
      --web-identity-token "file://$AWS_WEB_IDENTITY_TOKEN_FILE"
      --duration-seconds 3600 > /tmp/aws_creds.json
    - export AWS_ACCESS_KEY_ID="$(jq -r .Credentials.AccessKeyId /tmp/aws_creds.json)"
    - export AWS_SECRET_ACCESS_KEY="$(jq -r .Credentials.SecretAccessKey /tmp/aws_creds.json)"
    - export AWS_SESSION_TOKEN="$(jq -r .Credentials.SessionToken /tmp/aws_creds.json)"
  script:
    - /bin/gitlab-mcp-server || true
    - >
      claude
      -p "${AI_FLOW_INPUT:-'Implement the requested changes and open an MR'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  variables:
    AWS_REGION: "us-west-2"
```

<Note>
  Amazon Bedrock의 모델 ID에는 지역별 접두사가 포함됩니다 (예: `us.anthropic.claude-sonnet-4-6`). 워크플로우에서 지원하는 경우 작업 구성 또는 프롬프트를 통해 원하는 모델을 전달합니다.
</Note>

<h3 id="agent-platform-job-example-workload-identity-federation">
  Agent Platform 작업 예제 (Workload Identity Federation)
</h3>

**필수 조건:**

* GCP 프로젝트에서 Google Cloud의 Agent Platform API 활성화됨
* GitLab OIDC를 신뢰하도록 구성된 Workload Identity Federation
* Google Cloud의 Agent Platform 권한이 있는 서비스 계정

**필수 CI/CD 변수:**

* `GCP_WORKLOAD_IDENTITY_PROVIDER`: 전체 공급자 리소스 이름
* `GCP_SERVICE_ACCOUNT`: 서비스 계정 이메일
* `CLOUD_ML_REGION`: Google Cloud의 Agent Platform 지역 (예: `us-east5`)

```yaml theme={null}
claude-vertex:
  stage: ai
  image: gcr.io/google.com/cloudsdktool/google-cloud-cli:slim
  rules:
    - if: '$CI_PIPELINE_SOURCE == "web"'
  before_script:
    - apt-get update && apt-get install -y git && apt-get clean
    - curl -fsSL https://claude.ai/install.sh | bash
    # WIF를 통해 Google Cloud에 인증 (다운로드된 키 없음)
    - >
      gcloud auth login --cred-file=<(cat <<EOF
      {
        "type": "external_account",
        "audience": "${GCP_WORKLOAD_IDENTITY_PROVIDER}",
        "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
        "service_account_impersonation_url": "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/${GCP_SERVICE_ACCOUNT}:generateAccessToken",
        "token_url": "https://sts.googleapis.com/v1/token"
      }
      EOF
      )
    - gcloud config set project "$(gcloud projects list --format='value(projectId)' --filter="name:${CI_PROJECT_NAMESPACE}" | head -n1)" || true
  script:
    - /bin/gitlab-mcp-server || true
    - >
      CLOUD_ML_REGION="${CLOUD_ML_REGION:-us-east5}"
      claude
      -p "${AI_FLOW_INPUT:-'Review and update code as requested'}"
      --permission-mode acceptEdits
      --allowedTools "Bash Read Edit Write mcp__gitlab"
      --debug
  variables:
    CLOUD_ML_REGION: "us-east5"
```

<Note>
  Workload Identity Federation을 사용하면 서비스 계정 키를 저장할 필요가 없습니다. 저장소별 신뢰 조건 및 최소 권한 서비스 계정을 사용합니다.
</Note>

<h2 id="best-practices">
  모범 사례
</h2>

<h3 id="claude-md-configuration">
  CLAUDE.md 구성
</h3>

저장소 루트에 `CLAUDE.md` 파일을 생성하여 코딩 표준, 검토 기준 및 프로젝트별 규칙을 정의합니다. Claude는 실행 중에 이 파일을 읽고 변경 사항을 제안할 때 규칙을 따릅니다.

<h3 id="security-considerations">
  보안 고려 사항
</h3>

**API 키 또는 클라우드 자격 증명을 저장소에 커밋하지 마세요**. 항상 GitLab CI/CD 변수를 사용합니다:

* `ANTHROPIC_API_KEY`를 마스킹된 변수로 추가합니다 (필요한 경우 보호)
* 가능한 경우 공급자별 OIDC를 사용합니다 (장기 키 없음)
* 작업 권한 및 네트워크 송신 제한
* 다른 기여자처럼 Claude의 MR을 검토합니다

<h3 id="optimizing-performance">
  성능 최적화
</h3>

* `CLAUDE.md`를 집중적이고 간결하게 유지합니다
* 명확한 이슈/MR 설명을 제공하여 반복을 줄입니다
* 작업 시간 초과를 구성하여 실행 중단을 방지합니다
* 가능한 경우 러너에서 npm 및 패키지 설치를 캐시합니다

<h3 id="ci-costs">
  CI 비용
</h3>

GitLab CI/CD와 함께 Claude Code를 사용할 때 관련 비용을 인식합니다:

* **GitLab Runner 시간**:
  * Claude는 GitLab 러너에서 실행되고 컴퓨팅 분을 소비합니다
  * GitLab 플랜의 러너 청구 세부 정보를 참조하세요

* **API 비용**:
  * 각 Claude 상호 작용은 프롬프트 및 응답 크기에 따라 토큰을 소비합니다
  * 토큰 사용량은 작업 복잡도 및 코드베이스 크기에 따라 다릅니다
  * [Anthropic 가격 책정](https://platform.claude.com/docs/ko/about-claude/pricing) 세부 정보를 참조하세요

* **비용 최적화 팁**:
  * 특정 `@claude` 명령을 사용하여 불필요한 턴을 줄입니다
  * 적절한 `max_turns` 및 작업 시간 초과 값을 설정합니다
  * 동시성을 제한하여 병렬 실행을 제어합니다

<h2 id="security-and-governance">
  보안 및 거버넌스
</h2>

* 각 작업은 제한된 네트워크 액세스가 있는 격리된 컨테이너에서 실행됩니다
* Claude의 변경 사항은 MR을 통해 흐르므로 검토자가 모든 diff를 봅니다
* 브랜치 보호 및 승인 규칙이 AI 생성 코드에 적용됩니다
* Claude Code는 쓰기를 제한하기 위해 작업 공간 범위 권한을 사용합니다
* 자신의 공급자 자격 증명을 가져오기 때문에 비용이 제어됩니다

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude가 @claude 명령에 응답하지 않음
</h3>

* 파이프라인이 트리거되고 있는지 확인합니다 (수동, MR 이벤트 또는 노트 이벤트 리스너/웹훅을 통해)
* CI/CD 변수 (`ANTHROPIC_API_KEY` 또는 클라우드 공급자 설정)가 있고 마스킹 해제되어 있는지 확인합니다
* 댓글에 `@claude` (not `/claude`)가 포함되어 있고 언급 트리거가 구성되어 있는지 확인합니다

<h3 id="job-can’t-write-comments-or-open-mrs">
  작업이 댓글을 쓰거나 MR을 열 수 없음
</h3>

* `CI_JOB_TOKEN`이 프로젝트에 대한 충분한 권한이 있거나 `api` 범위가 있는 프로젝트 액세스 토큰을 사용하는지 확인합니다
* `mcp__gitlab` 도구가 `--allowedTools`에서 활성화되어 있는지 확인합니다
* 작업이 MR의 컨텍스트에서 실행되거나 `AI_FLOW_*` 변수를 통해 충분한 컨텍스트가 있는지 확인합니다

<h3 id="authentication-errors">
  인증 오류
</h3>

* **Claude API의 경우**: `ANTHROPIC_API_KEY`가 유효하고 만료되지 않았는지 확인합니다
* **Amazon Bedrock 또는 Google Cloud의 Agent Platform의 경우**: OIDC/WIF 구성, 역할 가장 및 비밀 이름을 확인합니다. 지역 및 모델 가용성을 확인합니다

<h2 id="advanced-configuration">
  고급 구성
</h2>

<h3 id="common-parameters-and-variables">
  일반적인 매개변수 및 변수
</h3>

Claude Code는 다음과 같이 일반적으로 사용되는 입력을 지원합니다:

* `prompt` / `prompt_file`: 인라인 (`-p`) 또는 파일을 통해 지침을 제공합니다
* `max_turns`: 왕복 반복 횟수를 제한합니다
* `timeout_minutes`: 총 실행 시간을 제한합니다
* `ANTHROPIC_API_KEY`: Claude API에 필요합니다 (Amazon Bedrock 또는 Google Cloud의 Agent Platform에는 사용되지 않음)
* 공급자별 환경: `AWS_REGION`, Google Cloud의 Agent Platform용 프로젝트/지역 변수

<Note>
  정확한 플래그 및 매개변수는 `@anthropic-ai/claude-code` 버전에 따라 다를 수 있습니다. 작업에서 `claude --help`를 실행하여 지원되는 옵션을 확인합니다.
</Note>

<h3 id="customizing-claude’s-behavior">
  Claude의 동작 사용자 정의
</h3>

두 가지 주요 방법으로 Claude를 안내할 수 있습니다:

1. **CLAUDE.md**: 코딩 표준, 보안 요구 사항 및 프로젝트 규칙을 정의합니다. Claude는 실행 중에 이를 읽고 규칙을 따릅니다.
2. **사용자 정의 프롬프트**: 작업에서 `prompt`/`prompt_file`을 통해 작업별 지침을 전달합니다. 다양한 작업에 다양한 프롬프트를 사용합니다 (예: 검토, 구현, 리팩토링).
