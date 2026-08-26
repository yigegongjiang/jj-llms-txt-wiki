> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitHub Actions

> Claude Code를 GitHub 워크플로우에 통합하는 방법에 대해 알아봅니다

Claude Code GitHub Actions는 GitHub 워크플로우에 AI 기반 자동화를 제공합니다. PR이나 이슈에서 간단한 `@claude` 멘션으로 Claude가 코드를 분석하고, 풀 리퀘스트를 생성하고, 기능을 구현하고, 버그를 수정할 수 있습니다. 모두 프로젝트의 표준을 따르면서 말입니다. 트리거 없이 모든 PR에 자동으로 게시되는 리뷰의 경우 [GitHub Code Review](/docs/ko/code-review)를 참조하십시오.

<Note>
  Claude Code GitHub Actions는 [Claude Agent SDK](/docs/ko/agent-sdk/overview)를 기반으로 구축되어 있으며, 이를 통해 Claude Code를 애플리케이션에 프로그래밍 방식으로 통합할 수 있습니다. SDK를 사용하여 GitHub Actions를 넘어서는 사용자 정의 자동화 워크플로우를 구축할 수 있습니다.
</Note>

<h2 id="why-use-claude-code-github-actions">
  Claude Code GitHub Actions를 사용하는 이유는 무엇입니까?
</h2>

* **즉시 PR 생성**: 필요한 사항을 설명하면 Claude가 모든 필요한 변경 사항이 포함된 완전한 PR을 생성합니다
* **자동화된 코드 구현**: 이슈를 단일 명령으로 작동하는 코드로 변환합니다
* **표준 준수**: Claude는 `CLAUDE.md` 지침과 기존 코드 패턴을 존중합니다
* **간단한 설정**: 설치 프로그램과 API 키로 몇 분 안에 시작할 수 있습니다
* **기본적으로 안전**: 코드는 Github의 러너에 유지됩니다

<h2 id="what-can-claude-do">
  Claude가 할 수 있는 것은 무엇입니까?
</h2>

Claude Code는 코드 작업 방식을 변환하는 강력한 GitHub Action을 제공합니다:

<h3 id="claude-code-action">
  Claude Code Action
</h3>

이 GitHub Action을 사용하면 GitHub Actions 워크플로우 내에서 Claude Code를 실행할 수 있습니다. 이를 사용하여 Claude Code 위에 사용자 정의 워크플로우를 구축할 수 있습니다.

[저장소 보기 →](https://github.com/anthropics/claude-code-action)

<h2 id="setup">
  설정
</h2>

<h2 id="quick-setup">
  빠른 설정
</h2>

Claude Code 터미널에서 `/install-github-app`을 실행하여 통합을 대화형으로 설정합니다. 이 명령은 Claude GitHub 앱을 저장소에 설치한 다음 GitHub Actions 워크플로우 및 API 키 시크릿 추가 과정을 안내합니다.

GitHub 앱이 설치된 후 명령은 GitHub Actions 설정을 계속할지 여부를 묻습니다. Claude Code v2.1.187 이상에서는 **지금 건너뛰기**를 선택하여 앱만 설치된 상태로 중지하고 `/install-github-app`을 다시 실행하여 워크플로우 및 시크릿 단계로 돌아갈 수 있습니다. 이전 버전은 워크플로우 선택으로 바로 진행합니다.

<Note>
  * GitHub 앱을 설치하고 시크릿을 추가하려면 저장소 관리자여야 합니다
  * GitHub 앱은 Contents, Issues 및 Pull requests에 대한 읽기 및 쓰기 권한을 요청합니다
  * 이 빠른 시작 방법은 직접 Claude API 사용자만 사용할 수 있습니다. Amazon Bedrock 또는 Google Cloud의 Agent Platform을 사용 중인 경우 [Amazon Bedrock 및 Google Cloud 사용](#using-with-amazon-bedrock-and-google-cloud) 섹션을 참조하십시오.
</Note>

<h2 id="manual-setup">
  수동 설정
</h2>

`/install-github-app` 명령이 실패하거나 수동 설정을 선호하는 경우 다음 수동 설정 지침을 따르십시오:

1. **Claude GitHub 앱을 저장소에 설치합니다**: [https://github.com/apps/claude](https://github.com/apps/claude)

   Claude GitHub 앱에는 다음 저장소 권한이 필요합니다:

   * **Contents**: 읽기 및 쓰기 (저장소 파일 수정)
   * **Issues**: 읽기 및 쓰기 (이슈에 응답)
   * **Pull requests**: 읽기 및 쓰기 (PR 생성 및 변경 사항 푸시)

   보안 및 권한에 대한 자세한 내용은 [보안 설명서](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md)를 참조하십시오.
2. **ANTHROPIC\_API\_KEY를 저장소 시크릿에 추가합니다** ([GitHub Actions에서 시크릿을 사용하는 방법 알아보기](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions))
3. **워크플로우 파일을 복사합니다** [examples/claude.yml](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml)에서 저장소의 `.github/workflows/`로

<Tip>
  빠른 시작 또는 수동 설정을 완료한 후 이슈 또는 PR 댓글에서 `@claude`를 태그하여 작업을 테스트합니다.
</Tip>

<h2 id="upgrading-from-beta">
  베타에서 업그레이드
</h2>

<Warning>
  Claude Code GitHub Actions v1.0은 베타 버전에서 v1.0으로 업그레이드하기 위해 워크플로우 파일을 업데이트해야 하는 주요 변경 사항을 도입합니다.
</Warning>

현재 Claude Code GitHub Actions의 베타 버전을 사용 중인 경우 워크플로우를 GA 버전을 사용하도록 업데이트하는 것이 좋습니다. 새 버전은 자동 모드 감지와 같은 강력한 새 기능을 추가하면서 구성을 단순화합니다.

<h3 id="essential-changes">
  필수 변경 사항
</h3>

모든 베타 사용자는 업그레이드하기 위해 워크플로우 파일에서 다음 변경 사항을 수행해야 합니다:

1. **작업 버전 업데이트**: `@beta`를 `@v1`로 변경합니다
2. **모드 구성 제거**: `mode: "tag"` 또는 `mode: "agent"` 삭제 (이제 자동 감지됨)
3. **프롬프트 입력 업데이트**: `direct_prompt`를 `prompt`로 바꿉니다
4. **CLI 옵션 이동**: `max_turns`, `model`, `custom_instructions` 등을 `claude_args`로 변환합니다

<h3 id="breaking-changes-reference">
  주요 변경 사항 참조
</h3>

| 이전 베타 입력              | 새 v1.0 입력                             |
| --------------------- | ------------------------------------- |
| `mode`                | *(제거됨 - 자동 감지됨)*                      |
| `direct_prompt`       | `prompt`                              |
| `override_prompt`     | GitHub 변수가 있는 `prompt`                |
| `custom_instructions` | `claude_args: --append-system-prompt` |
| `max_turns`           | `claude_args: --max-turns`            |
| `model`               | `claude_args: --model`                |
| `allowed_tools`       | `claude_args: --allowedTools`         |
| `disallowed_tools`    | `claude_args: --disallowedTools`      |
| `claude_env`          | `settings` JSON 형식                    |

<h3 id="before-and-after-example">
  이전 및 이후 예제
</h3>

**베타 버전:**

```yaml theme={null}
- uses: anthropics/claude-code-action@beta
  with:
    mode: "tag"
    direct_prompt: "Review this PR for security issues"
    anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
    custom_instructions: "Follow our coding standards"
    max_turns: "10"
    model: "claude-sonnet-5"
```

**GA 버전 (v1.0):**

```yaml theme={null}
- uses: anthropics/claude-code-action@v1
  with:
    prompt: "Review this PR for security issues"
    anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
    claude_args: |
      --append-system-prompt "Follow our coding standards"
      --max-turns 10
      --model claude-sonnet-5
```

<Tip>
  작업은 이제 구성에 따라 대화형 모드(`@claude` 멘션에 응답) 또는 자동화 모드(프롬프트로 즉시 실행)에서 실행할지 여부를 자동으로 감지합니다.
</Tip>

<h2 id="example-use-cases">
  예제 사용 사례
</h2>

Claude Code GitHub Actions는 다양한 작업에 도움이 될 수 있습니다. [examples 디렉토리](https://github.com/anthropics/claude-code-action/tree/main/examples)에는 다양한 시나리오에 대한 즉시 사용 가능한 워크플로우가 포함되어 있습니다.

<h3 id="basic-workflow">
  기본 워크플로우
</h3>

```yaml theme={null}
name: Claude Code
on:
  issue_comment:
    types: [created]
  pull_request_review_comment:
    types: [created]
jobs:
  claude:
    runs-on: ubuntu-latest
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          # Responds to @claude mentions in comments
```

<h3 id="using-skills">
  skills 사용
</h3>

`prompt` 입력은 [skill](/docs/ko/skills) 호출뿐만 아니라 일반 텍스트도 허용합니다:

* 저장소의 `.claude/skills/` 디렉토리에 있는 skill의 경우, 작업 단계 전에 `actions/checkout`을 실행하고 `/skill-name`을 전달합니다.
* 플러그인에 패키징된 skill의 경우, `plugin_marketplaces` 및 `plugins` 입력으로 플러그인을 설치하고 네임스페이스가 지정된 `/plugin-name:skill-name`을 전달합니다.

다음 워크플로우는 `code-review` 플러그인을 설치하고 각 새로운 또는 업데이트된 pull request에서 해당 skill을 실행합니다:

```yaml theme={null}
name: Code Review
on:
  pull_request:
    types: [opened, synchronize]
jobs:
  review:
    runs-on: ubuntu-latest
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          plugin_marketplaces: "https://github.com/anthropics/claude-code.git"
          plugins: "code-review@claude-code-plugins"
          prompt: "/code-review:code-review ${{ github.repository }}/pull/${{ github.event.pull_request.number }}"
```

<h3 id="custom-automation-with-prompts">
  프롬프트를 사용한 사용자 정의 자동화
</h3>

```yaml theme={null}
name: Daily Report
on:
  schedule:
    - cron: "0 9 * * *"
jobs:
  report:
    runs-on: ubuntu-latest
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          prompt: "Generate a summary of yesterday's commits and open issues"
          claude_args: "--model opus"
```

<h3 id="common-use-cases">
  일반적인 사용 사례
</h3>

이슈 또는 PR 댓글에서:

```text wrap theme={null}
@claude implement this feature based on the issue description
@claude how should I implement user authentication for this endpoint?
@claude fix the TypeError in the user dashboard component
```

Claude는 자동으로 컨텍스트를 분석하고 적절하게 응답합니다.

<h2 id="best-practices">
  모범 사례
</h2>

<h3 id="claude-md-configuration">
  CLAUDE.md 구성
</h3>

저장소 루트에 `CLAUDE.md` 파일을 생성하여 코드 스타일 지침, 리뷰 기준, 프로젝트별 규칙 및 선호하는 패턴을 정의합니다. 이 파일은 Claude의 프로젝트 표준 이해를 안내합니다.

<h3 id="security-considerations">
  보안 고려 사항
</h3>

<Warning>API 키를 저장소에 직접 커밋하지 마십시오.</Warning>

권한, 인증 및 모범 사례를 포함한 포괄적인 보안 지침은 [Claude Code Action 보안 설명서](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md)를 참조하십시오.

항상 GitHub Secrets를 API 키에 사용합니다:

* API 키를 `ANTHROPIC_API_KEY`라는 저장소 시크릿으로 추가합니다
* 워크플로우에서 참조합니다: `anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}`
* 작업 권한을 필요한 것으로만 제한합니다
* 병합하기 전에 Claude의 제안을 검토합니다

API 키를 워크플로우 파일에 직접 하드코딩하는 대신 항상 GitHub Secrets(예: `${{ secrets.ANTHROPIC_API_KEY }}`)를 사용합니다.

<h3 id="optimizing-performance">
  성능 최적화
</h3>

이슈 템플릿을 사용하여 컨텍스트를 제공하고, `CLAUDE.md`를 간결하고 집중적으로 유지하고, 워크플로우에 적절한 타임아웃을 구성합니다.

<h3 id="ci-costs">
  CI 비용
</h3>

Claude Code GitHub Actions를 사용할 때 관련 비용을 인식합니다:

**GitHub Actions 비용:**

* Claude Code는 GitHub 호스팅 러너에서 실행되며, 이는 GitHub Actions 분을 소비합니다
* 자세한 가격 책정 및 분 제한은 [GitHub의 청구 설명서](https://docs.github.com/en/billing/managing-billing-for-your-products/managing-billing-for-github-actions/about-billing-for-github-actions)를 참조하십시오

**API 비용:**

* 각 Claude 상호 작용은 프롬프트 및 응답의 길이에 따라 API 토큰을 소비합니다
* 토큰 사용량은 작업 복잡도 및 코드베이스 크기에 따라 다릅니다
* 현재 토큰 요금은 [Claude의 가격 책정 페이지](https://claude.com/platform/api)를 참조하십시오

**비용 최적화 팁:**

* 특정 `@claude` 명령을 사용하여 불필요한 API 호출을 줄입니다
* `claude_args`에서 적절한 `--max-turns`를 구성하여 과도한 반복을 방지합니다
* 워크플로우 수준 타임아웃을 설정하여 실행 중인 작업을 방지합니다
* GitHub의 동시성 제어를 사용하여 병렬 실행을 제한하는 것을 고려합니다

<h2 id="configuration-examples">
  구성 예제
</h2>

Claude Code Action v1은 통합 파라미터로 구성을 단순화합니다:

```yaml theme={null}
- uses: anthropics/claude-code-action@v1
  with:
    anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
    prompt: "Your instructions here" # Optional
    claude_args: "--max-turns 5" # Optional CLI arguments
```

주요 기능:

* **통합 프롬프트 인터페이스** - 모든 지침에 `prompt` 사용
* **skills** - 프롬프트에서 설치된 [skills](/docs/ko/skills)를 직접 호출합니다
* **CLI 통과** - `claude_args`를 통한 모든 Claude Code CLI 인수
* **유연한 트리거** - 모든 GitHub 이벤트와 함께 작동합니다

완전한 워크플로우 파일은 [examples 디렉토리](https://github.com/anthropics/claude-code-action/tree/main/examples)를 방문하십시오.

<Tip>
  이슈 또는 PR 댓글에 응답할 때 Claude는 자동으로 @claude 멘션에 응답합니다. 다른 이벤트의 경우 `prompt` 파라미터를 사용하여 지침을 제공합니다.
</Tip>

<h2 id="using-with-amazon-bedrock-and-google-cloud">
  Amazon Bedrock 및 Google Cloud 사용
</h2>

엔터프라이즈 환경의 경우 자신의 클라우드 인프라와 함께 Claude Code GitHub Actions를 사용할 수 있습니다. 이 접근 방식은 동일한 기능을 유지하면서 데이터 거주지 및 청구에 대한 제어를 제공합니다.

<h3 id="prerequisites">
  필수 조건
</h3>

클라우드 공급자와 함께 Claude Code GitHub Actions를 설정하기 전에 다음이 필요합니다:

<h4 id="for-google-cloud’s-agent-platform">
  Google Cloud의 Agent Platform의 경우:
</h4>

1. Google Cloud의 Agent Platform이 활성화된 Google Cloud 프로젝트
2. GitHub Actions에 대해 구성된 Workload Identity Federation
3. 필요한 권한이 있는 서비스 계정
4. GitHub 앱(권장) 또는 기본 GITHUB\_TOKEN 사용

<h4 id="for-amazon-bedrock">
  Amazon Bedrock의 경우:
</h4>

1. Amazon Bedrock이 활성화된 AWS 계정
2. AWS에서 구성된 GitHub OIDC Identity Provider
3. Amazon Bedrock 권한이 있는 IAM 역할
4. GitHub 앱(권장) 또는 기본 GITHUB\_TOKEN 사용

<Steps>
  <Step title="사용자 정의 GitHub 앱 생성 (3P 공급자에 권장)">
    Google Cloud의 Agent Platform 또는 Amazon Bedrock과 같은 3P 공급자를 사용할 때 최상의 제어 및 보안을 위해 자신의 GitHub 앱을 생성하는 것이 좋습니다:

    1. [https://github.com/settings/apps/new로](https://github.com/settings/apps/new로) 이동합니다
    2. 기본 정보를 입력합니다:
       * **GitHub 앱 이름**: 고유한 이름을 선택합니다 (예: "YourOrg Claude Assistant")
       * **홈페이지 URL**: 조직의 웹사이트 또는 저장소 URL
    3. 앱 설정을 구성합니다:
       * **Webhooks**: "Active" 선택 해제 (이 통합에는 필요하지 않음)
    4. 필수 권한을 설정합니다:
       * **저장소 권한**:
         * Contents: 읽기 및 쓰기
         * Issues: 읽기 및 쓰기
         * Pull requests: 읽기 및 쓰기
    5. "GitHub 앱 생성"을 클릭합니다
    6. 생성 후 "개인 키 생성"을 클릭하고 다운로드한 `.pem` 파일을 저장합니다
    7. 앱 설정 페이지에서 앱 ID를 기록합니다
    8. 저장소에 앱을 설치합니다:
       * 앱의 설정 페이지에서 왼쪽 사이드바의 "앱 설치"를 클릭합니다
       * 계정 또는 조직을 선택합니다
       * "선택한 저장소만"을 선택하고 특정 저장소를 선택합니다
       * "설치"를 클릭합니다
    9. 개인 키를 저장소 시크릿으로 추가합니다:
       * 저장소의 설정 → 시크릿 및 변수 → Actions로 이동합니다
       * `.pem` 파일의 내용으로 `APP_PRIVATE_KEY`라는 새 시크릿을 생성합니다
    10. 앱 ID를 시크릿으로 추가합니다:

    * GitHub 앱의 ID로 `APP_ID`라는 새 시크릿을 생성합니다

    <Note>
      이 앱은 [actions/create-github-app-token](https://github.com/actions/create-github-app-token) 작업과 함께 사용되어 워크플로우에서 인증 토큰을 생성합니다.
    </Note>

    **Claude API의 경우 또는 자신의 Github 앱을 설정하지 않으려는 경우 대안**: 공식 Anthropic 앱을 사용합니다:

    1. 다음에서 설치합니다: [https://github.com/apps/claude](https://github.com/apps/claude)
    2. 인증을 위한 추가 구성이 필요하지 않습니다
  </Step>

  <Step title="클라우드 공급자 인증 구성">
    클라우드 공급자를 선택하고 안전한 인증을 설정합니다:

    <AccordionGroup>
      <Accordion title="Amazon Bedrock">
        **자격 증명을 저장하지 않고 GitHub Actions가 안전하게 인증할 수 있도록 AWS를 구성합니다.**

        > **보안 참고**: 저장소별 구성을 사용하고 최소 필요 권한만 부여합니다.

        **필수 설정**:

        1. **Amazon Bedrock 활성화**:
           * Amazon Bedrock에서 Claude 모델에 대한 액세스 요청
           * 교차 지역 모델의 경우 모든 필요한 지역에서 액세스 요청

        2. **GitHub OIDC Identity Provider 설정**:
           * 공급자 URL: `https://token.actions.githubusercontent.com`
           * 대상: `sts.amazonaws.com`

        3. **GitHub Actions용 IAM 역할 생성**:
           * 신뢰할 수 있는 엔티티 유형: 웹 ID
           * ID 공급자: `token.actions.githubusercontent.com`
           * 권한: `AmazonBedrockFullAccess` 정책
           * 특정 저장소에 대한 신뢰 정책 구성

        **필수 값**:

        설정 후 다음이 필요합니다:

        * **AWS\_ROLE\_TO\_ASSUME**: 생성한 IAM 역할의 ARN

        <Tip>
          OIDC는 자격 증명이 임시이고 자동으로 회전되기 때문에 정적 AWS 액세스 키를 사용하는 것보다 더 안전합니다.
        </Tip>

        자세한 OIDC 설정 지침은 [AWS 설명서](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_create_oidc.html)를 참조하십시오.
      </Accordion>

      <Accordion title="Google Cloud의 Agent Platform">
        **자격 증명을 저장하지 않고 GitHub Actions가 안전하게 인증할 수 있도록 Google Cloud를 구성합니다.**

        > **보안 참고**: 저장소별 구성을 사용하고 최소 필요 권한만 부여합니다.

        **필수 설정**:

        1. **Google Cloud 프로젝트에서 API 활성화**:
           * IAM Credentials API
           * Security Token Service (STS) API
           * Google Cloud의 Agent Platform API

        2. **Workload Identity Federation 리소스 생성**:
           * Workload Identity Pool 생성
           * 다음을 사용하여 GitHub OIDC 공급자 추가:
             * 발급자: `https://token.actions.githubusercontent.com`
             * 저장소 및 소유자에 대한 속성 매핑
             * **보안 권장**: 저장소별 속성 조건 사용

        3. **서비스 계정 생성**:
           * `Vertex AI User` 역할만 부여
           * **보안 권장**: 저장소당 전용 서비스 계정 생성

        4. **IAM 바인딩 구성**:
           * Workload Identity Pool이 서비스 계정을 가장하도록 허용
           * **보안 권장**: 저장소별 주체 집합 사용

        **필수 값**:

        설정 후 다음이 필요합니다:

        * **GCP\_WORKLOAD\_IDENTITY\_PROVIDER**: 전체 공급자 리소스 이름
        * **GCP\_SERVICE\_ACCOUNT**: 서비스 계정 이메일 주소

        <Tip>
          Workload Identity Federation은 다운로드 가능한 서비스 계정 키의 필요성을 제거하여 보안을 개선합니다.
        </Tip>

        자세한 설정 지침은 [Google Cloud Workload Identity Federation 설명서](https://cloud.google.com/iam/docs/workload-identity-federation)를 참조하십시오.
      </Accordion>
    </AccordionGroup>
  </Step>

  <Step title="필수 시크릿 추가">
    저장소에 다음 시크릿을 추가합니다 (설정 → 시크릿 및 변수 → Actions):

    #### Claude API의 경우 (직접):

    1. **API 인증의 경우**:
       * `ANTHROPIC_API_KEY`: [console.anthropic.com](https://console.anthropic.com)의 Claude API 키

    2. **GitHub 앱의 경우 (자신의 앱을 사용하는 경우)**:
       * `APP_ID`: GitHub 앱의 ID
       * `APP_PRIVATE_KEY`: 개인 키 (.pem) 내용

    #### Google Cloud의 Agent Platform의 경우

    1. **GCP 인증의 경우**:
       * `GCP_WORKLOAD_IDENTITY_PROVIDER`
       * `GCP_SERVICE_ACCOUNT`

    2. **GitHub 앱의 경우 (자신의 앱을 사용하는 경우)**:
       * `APP_ID`: GitHub 앱의 ID
       * `APP_PRIVATE_KEY`: 개인 키 (.pem) 내용

    #### Amazon Bedrock의 경우

    1. **AWS 인증의 경우**:
       * `AWS_ROLE_TO_ASSUME`

    2. **GitHub 앱의 경우 (자신의 앱을 사용하는 경우)**:
       * `APP_ID`: GitHub 앱의 ID
       * `APP_PRIVATE_KEY`: 개인 키 (.pem) 내용
  </Step>

  <Step title="워크플로우 파일 생성">
    클라우드 공급자와 통합되는 GitHub Actions 워크플로우 파일을 생성합니다. 아래 예제는 Amazon Bedrock 및 Google Cloud의 Agent Platform 모두에 대한 완전한 구성을 보여줍니다:

    <AccordionGroup>
      <Accordion title="Amazon Bedrock 워크플로우">
        **필수 조건:**

        * Amazon Bedrock 액세스가 Claude 모델 권한으로 활성화됨
        * GitHub가 AWS에서 OIDC ID 공급자로 구성됨
        * GitHub Actions를 신뢰하는 Amazon Bedrock 권한이 있는 IAM 역할

        **필수 GitHub 시크릿:**

        | 시크릿 이름               | 설명                              |
        | -------------------- | ------------------------------- |
        | `AWS_ROLE_TO_ASSUME` | Amazon Bedrock 액세스용 IAM 역할의 ARN |
        | `APP_ID`             | GitHub 앱 ID (앱 설정에서)            |
        | `APP_PRIVATE_KEY`    | GitHub 앱에 대해 생성한 개인 키           |

        ```yaml theme={null}
        name: Claude PR Action

        permissions:
          contents: write
          pull-requests: write
          issues: write
          id-token: write

        on:
          issue_comment:
            types: [created]
          pull_request_review_comment:
            types: [created]
          issues:
            types: [opened, assigned]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && contains(github.event.issue.body, '@claude'))
            runs-on: ubuntu-latest
            env:
              AWS_REGION: us-west-2
            steps:
              - name: Checkout repository
                uses: actions/checkout@v4

              - name: Generate GitHub App token
                id: app-token
                uses: actions/create-github-app-token@v2
                with:
                  app-id: ${{ secrets.APP_ID }}
                  private-key: ${{ secrets.APP_PRIVATE_KEY }}

              - name: Configure AWS Credentials (OIDC)
                uses: aws-actions/configure-aws-credentials@v4
                with:
                  role-to-assume: ${{ secrets.AWS_ROLE_TO_ASSUME }}
                  aws-region: us-west-2

              - uses: anthropics/claude-code-action@v1
                with:
                  github_token: ${{ steps.app-token.outputs.token }}
                  use_bedrock: "true"
                  claude_args: '--model us.anthropic.claude-sonnet-4-6 --max-turns 10'
        ```

        <Tip>
          Amazon Bedrock의 모델 ID 형식에는 지역 접두사가 포함됩니다 (예: `us.anthropic.claude-sonnet-4-6`).
        </Tip>
      </Accordion>

      <Accordion title="Google Cloud의 Agent Platform 워크플로우">
        **필수 조건:**

        * GCP 프로젝트에서 Google Cloud의 Agent Platform API 활성화됨
        * GitHub에 대해 구성된 Workload Identity Federation
        * Google Cloud의 Agent Platform 권한이 있는 서비스 계정

        **필수 GitHub 시크릿:**

        | 시크릿 이름                           | 설명                                                 |
        | -------------------------------- | -------------------------------------------------- |
        | `GCP_WORKLOAD_IDENTITY_PROVIDER` | Workload Identity Provider 리소스 이름                  |
        | `GCP_SERVICE_ACCOUNT`            | Google Cloud의 Agent Platform 액세스 권한이 있는 서비스 계정 이메일 |
        | `APP_ID`                         | GitHub 앱 ID (앱 설정에서)                               |
        | `APP_PRIVATE_KEY`                | GitHub 앱에 대해 생성한 개인 키                              |

        ```yaml theme={null}
        name: Claude PR Action

        permissions:
          contents: write
          pull-requests: write
          issues: write
          id-token: write

        on:
          issue_comment:
            types: [created]
          pull_request_review_comment:
            types: [created]
          issues:
            types: [opened, assigned]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && contains(github.event.issue.body, '@claude'))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v4

              - name: Generate GitHub App token
                id: app-token
                uses: actions/create-github-app-token@v2
                with:
                  app-id: ${{ secrets.APP_ID }}
                  private-key: ${{ secrets.APP_PRIVATE_KEY }}

              - name: Authenticate to Google Cloud
                id: auth
                uses: google-github-actions/auth@v2
                with:
                  workload_identity_provider: ${{ secrets.GCP_WORKLOAD_IDENTITY_PROVIDER }}
                  service_account: ${{ secrets.GCP_SERVICE_ACCOUNT }}

              - uses: anthropics/claude-code-action@v1
                with:
                  github_token: ${{ steps.app-token.outputs.token }}
                  trigger_phrase: "@claude"
                  use_vertex: "true"
                  claude_args: '--model claude-sonnet-4-5@20250929 --max-turns 10'
                env:
                  ANTHROPIC_VERTEX_PROJECT_ID: ${{ steps.auth.outputs.project_id }}
                  CLOUD_ML_REGION: us-east5
                  VERTEX_REGION_CLAUDE_4_5_SONNET: us-east5
        ```

        <Tip>
          프로젝트 ID는 Google Cloud 인증 단계에서 자동으로 검색되므로 하드코딩할 필요가 없습니다.
        </Tip>
      </Accordion>
    </AccordionGroup>
  </Step>
</Steps>

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude가 @claude 명령에 응답하지 않음
</h3>

GitHub 앱이 올바르게 설치되었는지 확인하고, 워크플로우가 활성화되었는지 확인하고, API 키가 저장소 시크릿에 설정되었는지 확인하고, 댓글에 `@claude`가 포함되어 있는지 확인합니다 (`/claude` 아님).

<h3 id="ci-not-running-on-claude’s-commits">
  Claude의 커밋에서 CI가 실행되지 않음
</h3>

GitHub 앱 또는 사용자 정의 앱을 사용 중인지 확인합니다 (Actions 사용자 아님), 워크플로우 트리거에 필요한 이벤트가 포함되어 있는지 확인하고, 앱 권한에 CI 트리거가 포함되어 있는지 확인합니다.

<h3 id="authentication-errors">
  인증 오류
</h3>

API 키가 유효하고 충분한 권한이 있는지 확인합니다. Amazon Bedrock 또는 Google Cloud의 Agent Platform의 경우 자격 증명 구성을 확인하고 시크릿이 워크플로우에서 올바르게 명명되었는지 확인합니다.

<h2 id="advanced-configuration">
  고급 구성
</h2>

<h3 id="action-parameters">
  작업 파라미터
</h3>

Claude Code Action v1은 단순화된 구성을 사용합니다:

| 파라미터                  | 설명                                               | 필수    |
| --------------------- | ------------------------------------------------ | ----- |
| `prompt`              | Claude에 대한 지침 (일반 텍스트 또는 [skill](/docs/ko/skills) 이름) | 아니오\* |
| `claude_args`         | Claude Code에 전달된 CLI 인수                          | 아니오   |
| `plugin_marketplaces` | 플러그인 마켓플레이스 Git URL의 줄 바꿈으로 구분된 목록               | 아니오   |
| `plugins`             | 실행 전에 설치할 플러그인 이름의 줄 바꿈으로 구분된 목록                 | 아니오   |
| `anthropic_api_key`   | Claude API 키                                     | 예\*\* |
| `github_token`        | API 액세스용 GitHub 토큰                               | 아니오   |
| `trigger_phrase`      | 사용자 정의 트리거 구문 (기본값: "@claude")                   | 아니오   |
| `use_bedrock`         | Claude API 대신 Amazon Bedrock 사용                  | 아니오   |
| `use_vertex`          | Claude API 대신 Google Cloud의 Agent Platform 사용    | 아니오   |

\*프롬프트는 선택 사항입니다. 이슈/PR 댓글에서 생략하면 Claude는 트리거 구문에 응답합니다\
\*\*직접 Claude API에 필수이며, Amazon Bedrock 또는 Google Cloud의 Agent Platform에는 필수가 아닙니다

<h4 id="pass-cli-arguments">
  CLI 인수 전달
</h4>

`claude_args` 파라미터는 모든 Claude Code CLI 인수를 허용합니다:

```yaml theme={null}
claude_args: "--max-turns 5 --model claude-sonnet-5 --mcp-config /path/to/config.json"
```

일반적인 인수:

* `--max-turns`: 최대 대화 턴 (기본값: 10)
* `--model`: 사용할 모델 (예: `claude-sonnet-5`)
* `--mcp-config`: MCP 구성 경로
* `--allowedTools`: 허용된 도구의 쉼표로 구분된 목록. `--allowed-tools` 별칭도 작동합니다.
* `--debug`: 디버그 출력 활성화

<h3 id="alternative-integration-methods">
  대체 통합 방법
</h3>

`/install-github-app` 명령이 권장되는 접근 방식이지만 다음을 수행할 수도 있습니다:

* **사용자 정의 GitHub 앱**: 브랜드 사용자 이름 또는 사용자 정의 인증 흐름이 필요한 조직의 경우. 필요한 권한(contents, issues, pull requests)으로 자신의 GitHub 앱을 생성하고 actions/create-github-app-token 작업을 사용하여 워크플로우에서 토큰을 생성합니다.
* **수동 GitHub Actions**: 최대 유연성을 위한 직접 워크플로우 구성
* **MCP 구성**: Model Context Protocol 서버의 동적 로딩

자세한 인증, 보안 및 고급 구성 가이드는 [Claude Code Action 설명서](https://github.com/anthropics/claude-code-action/blob/main/docs)를 참조하십시오.

<h3 id="customizing-claude’s-behavior">
  Claude의 동작 사용자 정의
</h3>

두 가지 방법으로 Claude의 동작을 구성할 수 있습니다:

1. **CLAUDE.md**: 저장소의 루트에 `CLAUDE.md` 파일에서 코딩 표준, 리뷰 기준 및 프로젝트별 규칙을 정의합니다. Claude는 PR을 생성하고 요청에 응답할 때 이러한 지침을 따릅니다. 자세한 내용은 [Memory 설명서](/docs/ko/memory)를 확인하십시오.
2. **사용자 정의 프롬프트**: 워크플로우 파일의 `prompt` 파라미터를 사용하여 워크플로우별 지침을 제공합니다. 이를 통해 다양한 워크플로우 또는 작업에 대해 Claude의 동작을 사용자 정의할 수 있습니다.

Claude는 PR을 생성하고 요청에 응답할 때 이러한 지침을 따릅니다.
