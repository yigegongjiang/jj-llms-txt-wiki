> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitHub Actions를 클라우드 제공자와 함께 사용하기

> Claude API 대신 Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry를 통해 Claude Code GitHub Actions 실행하기

[Claude Code GitHub Actions](/docs/ko/github-actions)는 기본적으로 Claude API를 호출합니다. 대신 자신의 클라우드 계정을 통해 추론을 라우팅하려면 Claude Code GitHub Action의 provider 입력을 설정하고 워크플로우의 OpenID Connect(OIDC) 토큰을 신뢰하도록 클라우드를 구성합니다. 워크플로우는 해당 토큰으로 인증되므로 저장소에 장기 클라우드 자격 증명을 저장할 필요가 없습니다.

<Info>
  이 페이지는 [GitHub Actions 설정](/docs/ko/github-actions#setup)을 기반으로 합니다. 이미 워크플로우 파일과 `anthropics/claude-code-action` 단계를 알고 있다고 가정하며, 클라우드 제공자가 변경하는 부분만 다룹니다.
</Info>

<h2 id="choose-your-provider">
  제공자 선택하기
</h2>

Claude Code GitHub Action은 세 가지 제공자를 지원하며, 아래의 설정 단계는 클라우드 측 구성에서만 다릅니다. 조직에서 이미 Claude 모델 액세스 권한이 있는 제공자를 사용합니다. `anthropics/claude-code-action` 단계의 `with:` 블록에서 하나의 입력으로 Claude Code GitHub Action에 사용할 제공자를 알려줍니다:

* **Amazon Bedrock**: `use_bedrock: "true"`
* **Google Cloud의 Agent Platform**: `use_vertex: "true"`
* **Microsoft Foundry**: `use_foundry: "true"`

[통합 설정](#set-up-the-integration) 아래의 완전한 워크플로우 예제에는 각 제공자에 대한 입력이 이미 포함되어 있습니다.

<h2 id="prerequisites">
  필수 조건
</h2>

시작하기 전에 다음이 필요합니다:

* Claude Code GitHub Action이 실행되는 저장소에 대한 관리자 액세스 권한(GitHub App 설치 및 비밀 추가)
* 클라우드 계정에서 ID 리소스를 생성할 권한: AWS의 IAM 역할 및 OIDC ID 제공자, Google Cloud의 Workload Identity Federation 리소스 및 서비스 계정, 또는 Azure의 Microsoft Entra 애플리케이션
* 제공자의 Claude 모델 액세스:
  * **Amazon Bedrock**: Claude 모델에 대한 액세스 권한 부여. 이 페이지의 예제에서 `us.` 모델 ID와 같은 교차 지역 추론 프로필은 해당 지역 그룹의 모든 지역에서 액세스 권한이 부여되어야 합니다. [Amazon Bedrock의 Claude Code](/docs/ko/amazon-bedrock) 참조
  * **Google Cloud의 Agent Platform**: Agent Platform API가 활성화되고 Claude 모델에 액세스할 수 있는 프로젝트. [Google Cloud의 Agent Platform의 Claude Code](/docs/ko/google-vertex-ai) 참조
  * **Microsoft Foundry**: Claude 모델 배포가 있는 Foundry 리소스. [Microsoft Foundry의 Claude Code](/docs/ko/microsoft-foundry) 참조

<h2 id="set-up-the-integration">
  통합 설정하기
</h2>

필수 조건 외에도 Claude Code GitHub Action을 위한 GitHub ID, 클라우드 측 신뢰 구성, 저장소 비밀, 그리고 워크플로우 파일을 생성합니다. 아래 단계는 각각을 안내합니다.

<Steps>
  <Step title="GitHub ID 선택하기">
    Claude Code GitHub Action은 GitHub ID를 통해 커밋을 푸시하고 댓글을 게시합니다. [빠른 설정](/docs/ko/github-actions#quick-setup)은 이를 위해 공식 Claude GitHub App을 설치합니다. 클라우드 제공자를 사용할 때는 ID를 직접 선택합니다:

    * **공식 [Claude GitHub App](https://github.com/apps/claude)**: 저장소에 설치하거나, 이미 설치되어 있으면 다음 단계로 건너뜁니다
    * **사용자 정의 GitHub App**: Claude Code GitHub Action이 사용하는 세 가지 권한만 원할 때 자신의 앱을 생성합니다([공식 앱의 전체 세트](/docs/ko/github-actions#github-app-permissions) 대신)
    * **GitHub의 자동 `GITHUB_TOKEN`**: 생성하거나 설치할 앱이 없지만, GitHub는 이를 사용하여 만든 커밋에서 CI 워크플로우를 트리거하지 않습니다

    네 번째 단계의 워크플로우 예제는 사용자 정의 앱으로 인증합니다. 해당 단계에서는 다른 두 옵션에 대해 변경할 사항도 설명합니다.

    사용자 정의 앱을 생성하려면 [새 GitHub App을 등록](https://docs.github.com/en/apps/creating-github-apps/registering-a-github-app/registering-a-github-app)하되 이 통합에서 사용하지 않으므로 웹훅을 비활성화합니다. 세 가지 저장소 권한을 부여합니다:

    * **Contents**: 읽기 및 쓰기
    * **Issues**: 읽기 및 쓰기
    * **Pull requests**: 읽기 및 쓰기

    앱을 등록한 후 개인 키를 생성하고 다운로드한 `.pem` 파일을 보관하고, 앱의 설정 페이지에서 App ID를 기록하고, Claude Code GitHub Action이 실행되는 저장소에 [앱을 설치](https://docs.github.com/en/apps/using-github-apps/installing-your-own-github-app)합니다. 세 번째 단계에서 키와 ID를 비밀로 추가합니다.
  </Step>

  <Step title="클라우드 인증 구성하기">
    GitHub가 워크플로우에 발급하는 OIDC 토큰을 신뢰하도록 클라우드를 구성하여 각 워크플로우 실행이 단기 클라우드 자격증명을 얻도록 합니다. 각 탭의 글머리 기호는 생성할 항목을 요약하고, 각 탭은 콘솔 수준 단계에 대한 클라우드 공급업체의 가이드로 연결됩니다.

    <Tabs>
      <Tab title="Amazon Bedrock">
        [OIDC ID 제공자 생성에 대한 AWS 가이드](https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_create_oidc.html)에 따라 AWS 계정에서 신뢰 구성을 생성합니다:

        * 제공자 URL `https://token.actions.githubusercontent.com` 및 대상 `sts.amazonaws.com`을 사용하여 GitHub OIDC ID 제공자 추가
        * 해당 제공자를 웹 ID로 신뢰하는 IAM 역할을 생성하고 [IAM 구성](/docs/ko/amazon-bedrock#iam-configuration)의 범위가 지정된 호출 정책을 연결합니다. 이 정책은 `bedrock:InvokeModel`, `bedrock:InvokeModelWithResponseStream`, `bedrock:ListInferenceProfiles`, `bedrock:GetInferenceProfile` 및 두 가지 `aws-marketplace` 구독 작업을 부여합니다
        * 역할의 신뢰 정책을 저장소로 제한합니다. `repo:your-org/your-repo:*`와 같은 주체 조건을 사용합니다. 청구 형식은 [GitHub의 OIDC 강화 가이드](https://docs.github.com/en/actions/deployment/security-hardening-your-deployments/about-security-hardening-with-openid-connect)를 참조합니다

        역할의 ARN을 기록합니다. 다음 단계에서 비밀로 추가합니다.
      </Tab>

      <Tab title="Google Cloud의 Agent Platform">
        [Workload Identity Federation 문서](https://cloud.google.com/iam/docs/workload-identity-federation)에 따라 Google Cloud 프로젝트에서 페더레이션 리소스를 생성합니다:

        * 세 가지 API 활성화: IAM Credentials, Security Token Service(STS), 및 Agent Platform API(서비스 이름은 `aiplatform.googleapis.com`)
        * 발급자가 `https://token.actions.githubusercontent.com`인 GitHub OIDC 제공자를 사용하여 Workload Identity Pool을 생성하고, 풀을 저장소로 제한하는 속성 조건을 추가합니다
        * `Vertex AI User` 역할(즉, `roles/aiplatform.user`)만 있는 전용 서비스 계정을 생성하고 풀이 이를 가장하도록 허용합니다

        제공자의 전체 리소스 이름과 서비스 계정의 이메일 주소를 기록합니다. 다음 단계에서 비밀로 추가합니다.
      </Tab>

      <Tab title="Microsoft Foundry">
        [GitHub Actions에서 인증하기에 대한 Microsoft 가이드](https://learn.microsoft.com/en-us/azure/developer/github/connect-from-azure-openid-connect)에 따라 저장소에 대한 페더레이션 자격증명을 사용하여 Microsoft Entra 애플리케이션을 생성합니다:

        * Microsoft Entra 애플리케이션을 등록하고 저장소에 GitHub가 발급하는 토큰을 신뢰하는 페더레이션 ID 자격증명을 추가합니다. 사용자 할당 관리 ID는 애플리케이션 대신 사용할 수 있습니다. 둘 다 아래에서 기록하는 클라이언트 ID를 가집니다
        * 애플리케이션에 Foundry 리소스에 대한 `Azure AI User` 역할을 할당합니다. 더 좁은 사용자 정의 역할은 [Azure RBAC 구성](/docs/ko/microsoft-foundry#azure-rbac-configuration)을 참조합니다

        애플리케이션의 클라이언트 ID, 테넌트 ID, 구독 ID를 기록합니다. 다음 단계에서 비밀로 추가합니다.
      </Tab>
    </Tabs>
  </Step>

  <Step title="저장소 비밀 추가하기">
    Claude Code GitHub Action이 실행되는 저장소에서 제공자에 대한 비밀을 추가하고, 첫 번째 단계에서 사용자 정의 GitHub App을 생성한 경우 두 가지 앱 비밀을 추가합니다. GitHub의 [GitHub Actions에서 비밀 사용하기](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions) 가이드를 참조합니다.

    | 비밀                               | 필요한 대상                       | 값                      |
    | -------------------------------- | ---------------------------- | ---------------------- |
    | `AWS_ROLE_TO_ASSUME`             | Amazon Bedrock               | IAM 역할의 ARN            |
    | `GCP_WORKLOAD_IDENTITY_PROVIDER` | Google Cloud의 Agent Platform | 제공자의 전체 리소스 이름         |
    | `GCP_SERVICE_ACCOUNT`            | Google Cloud의 Agent Platform | 서비스 계정의 이메일 주소         |
    | `AZURE_CLIENT_ID`                | Microsoft Foundry            | Entra 애플리케이션의 클라이언트 ID |
    | `AZURE_TENANT_ID`                | Microsoft Foundry            | Microsoft Entra 테넌트 ID |
    | `AZURE_SUBSCRIPTION_ID`          | Microsoft Foundry            | Azure 구독 ID            |
    | `APP_ID`                         | 사용자 정의 GitHub App            | GitHub App의 ID         |
    | `APP_PRIVATE_KEY`                | 사용자 정의 GitHub App            | `.pem` 개인 키 파일의 내용     |
  </Step>

  <Step title="워크플로우 파일 생성하기">
    제공자에 대한 워크플로우 파일(예: `.github/workflows/claude.yml`)을 생성합니다. 각 예제는 `@claude` 언급에 응답하고, 사용자 정의 앱으로 GitHub에 인증하며, `id-token: write` 권한을 포함합니다. GitHub는 클라우드 제공자가 자격증명으로 교환하는 OIDC 토큰을 발급하기 위해 이 권한이 필요합니다.

    첫 번째 단계에서 다른 GitHub ID를 선택한 경우 예제를 조정합니다:

    * **공식 Claude GitHub App**: GitHub App 토큰 생성 단계와 `github_token` 줄 삭제
    * **GitHub의 자동 토큰**: 토큰 생성 단계 삭제 및 `github_token` 줄을 `github_token: ${{ secrets.GITHUB_TOKEN }}`으로 변경

    <Warning>
      공개 저장소에서 모든 사용자의 트리거 구문이 포함된 댓글이 이 워크플로우를 시작합니다. 자격증명 단계는 Claude Code GitHub Action이 댓글 작성자의 쓰기 액세스를 확인하기 전에 실행되므로, 작업은 권한이 없는 사용자를 거부하지만 워크플로우가 App 토큰을 생성하고 클라우드 제공자에 로그인한 후에만 거부합니다. 이는 감사 로그 항목을 남기고 Actions 분을 소비합니다. 이러한 실행을 피하려면 자격증명 단계 전에 댓글 작성자의 쓰기 액세스를 확인하는 단계를 추가합니다.
    </Warning>

    <Tabs>
      <Tab title="Amazon Bedrock">
        `aws-region` 값을 자신의 값으로 바꿉니다. 자격증명 단계는 이를 `AWS_REGION`으로 내보내므로 작업의 나머지 부분에서 사용할 수 있습니다.

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
            types: [opened]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v6

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
                  claude_args: '--model us.anthropic.claude-sonnet-4-6'
        ```

        <Tip>
          Bedrock 모델 ID에는 `us.`와 같은 교차 지역 추론 프로필 접두사가 포함됩니다. 모델 액세스 권한을 부여한 지역 그룹의 접두사를 사용합니다.
        </Tip>
      </Tab>

      <Tab title="Google Cloud의 Agent Platform">
        `CLOUD_ML_REGION` 값을 자신의 값으로 바꿉니다. 워크플로우가 `auth` 단계의 출력에서 프로젝트 ID를 읽으므로 프로젝트 ID를 하드코딩할 필요가 없습니다.

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
            types: [opened]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v6

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
                  use_vertex: "true"
                  claude_args: '--model claude-sonnet-5'
                env:
                  ANTHROPIC_VERTEX_PROJECT_ID: ${{ steps.auth.outputs.project_id }}
                  CLOUD_ML_REGION: us-east5
        ```
      </Tab>

      <Tab title="Microsoft Foundry">
        `your-resource-name`을 Foundry 리소스 이름으로 바꿉니다. Claude Code는 이를 통해 엔드포인트 URL을 구축합니다. `azure/login` 단계는 워크플로우의 OIDC 토큰으로 로그인하고, Claude Code는 Azure [기본 자격증명 체인](https://learn.microsoft.com/en-us/azure/developer/javascript/sdk/authentication/credential-chains#defaultazurecredential-overview)을 통해 자격증명을 선택합니다.

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
            types: [opened]

        jobs:
          claude-pr:
            if: |
              (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
              (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || contains(github.event.issue.title, '@claude')))
            runs-on: ubuntu-latest
            steps:
              - name: Checkout repository
                uses: actions/checkout@v6

              - name: Generate GitHub App token
                id: app-token
                uses: actions/create-github-app-token@v2
                with:
                  app-id: ${{ secrets.APP_ID }}
                  private-key: ${{ secrets.APP_PRIVATE_KEY }}

              - name: Authenticate to Azure
                uses: azure/login@v2
                with:
                  client-id: ${{ secrets.AZURE_CLIENT_ID }}
                  tenant-id: ${{ secrets.AZURE_TENANT_ID }}
                  subscription-id: ${{ secrets.AZURE_SUBSCRIPTION_ID }}

              - uses: anthropics/claude-code-action@v1
                with:
                  github_token: ${{ steps.app-token.outputs.token }}
                  use_foundry: "true"
                  claude_args: '--model claude-sonnet-5'
                env:
                  ANTHROPIC_FOUNDRY_RESOURCE: your-resource-name
        ```

        <Tip>
          Foundry 리소스의 Claude 배포와 일치하는 모델 ID를 사용합니다. 모델 구성 및 버전 고정은 [Microsoft Foundry의 Claude Code](/docs/ko/microsoft-foundry)를 참조합니다.
        </Tip>
      </Tab>
    </Tabs>

    모든 제공자에서 `claude_args`에 `--max-turns`를 추가하여 실행 길이와 비용을 제한할 수 있습니다. [비용 관리](/docs/ko/github-actions#manage-costs)를 참조합니다.
  </Step>

  <Step title="설정 테스트하기">
    이슈 또는 PR 댓글에서 `@claude`를 언급한 다음 저장소의 Actions 탭에서 실행을 확인합니다. Claude는 동일한 이슈 또는 PR의 댓글로 응답합니다.
  </Step>
</Steps>

<h2 id="troubleshooting">
  문제 해결
</h2>

실패한 실행은 일반적으로 두 가지 위치 중 하나에서 중단됩니다:

* **인증 오류**: 일반적으로 OIDC 구성 오류입니다. 워크플로우에 `id-token: write` 권한이 포함되어 있는지, 신뢰 구성의 저장소 조건이 저장소와 정확히 일치하는지, 워크플로우의 비밀 이름이 추가한 이름과 일치하는지 확인합니다
* **트리거 및 CI 문제**: Claude Code GitHub Action이 Claude API를 호출할 때와 동일하게 동작합니다. 메인 페이지의 [문제 해결 섹션](/docs/ko/github-actions#troubleshooting) 및 Claude Code GitHub Action의 [FAQ](https://github.com/anthropics/claude-code-action/blob/main/docs/faq.md)를 참조합니다

<h2 id="what’s-next">
  다음 단계
</h2>

* [Claude Code GitHub Actions](/docs/ko/github-actions) - 예제, 매개변수, 모범 사례
* [Amazon Bedrock의 Claude Code](/docs/ko/amazon-bedrock) - Bedrock 모델 ID 및 지역
* [Google Cloud의 Agent Platform의 Claude Code](/docs/ko/google-vertex-ai) - Agent Platform 모델 ID 및 지역
* [Microsoft Foundry의 Claude Code](/docs/ko/microsoft-foundry) - Foundry 모델 및 엔드포인트 구성
