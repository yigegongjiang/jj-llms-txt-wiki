> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code GitHub Actions

> @claude 멘션에 응답하고, 작업을 자동화하고, 이슈를 풀 리퀘스트로 변환하기 위해 GitHub Actions 워크플로우에서 Claude Code를 실행합니다

[Claude Code GitHub Actions](https://github.com/anthropics/claude-code-action)는 저장소의 워크플로우 내에서 Claude Code를 실행하는 GitHub Action입니다. 풀 리퀘스트나 이슈 댓글에서 `@claude`를 멘션하여 Claude가 코드를 분석하고, 변경 사항을 구현하고, 커밋을 푸시하도록 합니다. Claude Code GitHub Action에 프롬프트를 제공하여 모든 GitHub 이벤트에서 자동으로 실행되도록 할 수도 있습니다. 이슈를 풀 리퀘스트로 변환하거나, 댓글에서 버그를 수정하거나, 반복되는 작업을 자동화하는 데 사용합니다.

여러 제품이 Claude Code 이름을 공유합니다. 이 페이지는 저장소의 워크플로우 파일로 구성하는 `claude-code-action` 워크플로우 통합을 다룹니다. 관련 제품의 경우 다음을 참조하십시오:

* [Code Review](/docs/ko/code-review): 워크플로우를 작성하지 않고 모든 풀 리퀘스트에서 자동 리뷰
* [클라우드의 Claude Code](/docs/ko/claude-code-on-the-web): 클라우드 인프라에서 실행되는 Claude Code 세션
* [Claude Agent SDK](/docs/ko/agent-sdk/overview): GitHub Actions 외부의 사용자 정의 자동화. Claude Code GitHub Action은 SDK를 기반으로 구축됩니다
* [GitHub Enterprise Server](/docs/ko/github-enterprise-server): 자체 호스팅 GitHub를 사용한 Claude Code

<h2 id="setup">
  설정
</h2>

Claude Code GitHub Action을 두 가지 방법 중 하나로 설정할 수 있습니다:

* **빠른 설정**: Claude Code에서 `/install-github-app`을 실행합니다. Claude Code가 GitHub App을 설치하고, 인증 시크릿을 추가하고, 워크플로우 풀 리퀘스트를 준비합니다
* **수동 설정**: 앱을 설치하고, 시크릿을 추가하고, 워크플로우 파일을 저장소에 직접 복사합니다. 로컬에서 Claude Code를 실행하지 않을 때, 명령이 실패할 때, 또는 워크플로우 파일을 완전히 제어하고 싶을 때 이 경로를 사용합니다.

두 경로 모두 저장소에 대한 관리자 액세스가 필요합니다.

<h3 id="quick-setup">
  빠른 설정
</h3>

`/install-github-app`은 github.com 저장소에서만 작동합니다. 저장소의 git 원격이 gitlab.com이나 bitbucket.org에 있으면 명령이 알림을 출력하고 설정을 시작하지 않고 종료합니다. GitLab 파이프라인에서 Claude Code를 실행하려면 [Claude Code GitLab CI/CD](/docs/ko/gitlab-ci-cd)를 참조하십시오.

시작하기 전에 [GitHub CLI](https://cli.github.com)를 설치하고 `gh auth login`으로 인증합니다. Claude Code가 이를 확인하고 누락되면 경고합니다.

연결하려는 저장소에서 `claude`를 열고 `/install-github-app`을 실행한 후 프롬프트를 따릅니다. Claude Code가 Claude GitHub App을 설치한 후 워크플로우에 대한 인증 시크릿을 설정합니다:

* Claude Code가 이미 API 키를 가지고 있으면 해당 키를 재사용하고, 저장소에 이미 설정된 `ANTHROPIC_API_KEY` 시크릿을 유지할 것을 제안합니다
* 그렇지 않으면 Claude 구독으로 장기 토큰을 생성하거나 API 키를 붙여넣기 중에서 선택합니다

Claude Code가 자격 증명을 저장소 시크릿으로 저장합니다. API 키의 경우 `ANTHROPIC_API_KEY`, 구독 토큰의 경우 `CLAUDE_CODE_OAUTH_TOKEN`이라고 명명됩니다.

Claude Code가 선택한 워크플로우 파일이 포함된 브랜치를 푸시하고, 이미 해당 시크릿을 사용하도록 설정되어 있으며, 풀 리퀘스트를 생성할 준비가 된 상태로 브라우저에서 GitHub를 엽니다. 해당 풀 리퀘스트를 생성하고 병합하면 저장소에서 `@claude`가 작동합니다.

리뷰 워크플로우를 선택하면 Claude가 각 리뷰를 풀 리퀘스트 자체에 게시합니다. 발견한 각 이슈에 대한 인라인 댓글로 또는 발견하지 못한 경우 하나의 요약 댓글로 게시합니다. Claude는 초안과 같은 일부 풀 리퀘스트를 건너뜁니다. [리뷰 워크플로우 예제](#run-a-skill)는 동일한 스킬을 사용하고 이를 나열합니다. v2.1.229 이전에는 Claude가 리뷰를 워크플로우 실행 로그에만 작성했습니다.

이전 버전이 생성한 리뷰 워크플로우를 업데이트하려면 다음 중 하나를 수행합니다:

* `/install-github-app`을 다시 실행합니다. 저장소에 이미 `claude.yml`이 있으면 **최신 버전으로 워크플로우 파일 업데이트**를 선택합니다. Claude Code가 새 브랜치에 워크플로우 파일의 새 복사본을 푸시하고 첫 설치와 동일하게 풀 리퀘스트를 엽니다.
* [리뷰 워크플로우 예제](#run-a-skill)에서 `--comment` 인수와 `claude_args` 줄을 직접 체크인된 파일에 추가합니다. 이렇게 하면 파일에 대해 수행한 다른 편집 사항이 유지됩니다.

GitHub App을 설치한 후 Claude Code가 GitHub Actions 설정을 계속할지 여부를 묻습니다. **지금 건너뛰기**를 선택하여 GitHub App만 설치된 상태로 중지합니다. 나중에 `/install-github-app`을 다시 실행하여 워크플로우 및 시크릿 단계를 완료합니다.

<Note>
  * GitHub App을 설치하면 여러 권한을 부여합니다. 전체 집합은 [GitHub App 권한](#github-app-permissions)을 참조하십시오
  * 빠른 설정은 Claude API 및 Claude 구독과 함께 작동합니다. Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry를 사용하는 경우 [클라우드 공급자와 함께 Claude Code GitHub Actions 사용](/docs/ko/github-actions-cloud-providers)을 참조하십시오
</Note>

<h3 id="manual-setup">
  수동 설정
</h3>

`/install-github-app`을 실행하지 않고 Claude Code GitHub Action을 구성하려면 앱을 설치하고, 시크릿을 추가하고, 워크플로우 파일을 직접 복사합니다:

<Steps>
  <Step title="Claude GitHub App 설치">
    [Claude GitHub App](https://github.com/apps/claude)을 저장소에 설치합니다. Claude Code GitHub Action은 앱의 세 가지 권한에 의존합니다:

    * **Contents**: 읽기 및 쓰기. Claude가 저장소 파일을 수정할 수 있습니다
    * **Issues**: 읽기 및 쓰기. Claude가 이슈에 응답할 수 있습니다
    * **Pull requests**: 읽기 및 쓰기. Claude가 PR을 생성하고 변경 사항을 푸시할 수 있습니다

    설치 중에 다른 Claude 기능이 사용하는 권한도 부여합니다. 전체 집합은 [GitHub App 권한](#github-app-permissions)을 참조하십시오.
  </Step>

  <Step title="인증 시크릿 추가">
    인증 방식에 따라 저장소에 다음 시크릿 중 하나를 추가합니다. GitHub의 [GitHub Actions에서 시크릿 사용](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions) 가이드를 참조하십시오.

    * `ANTHROPIC_API_KEY`: [Claude Console](https://platform.claude.com)의 Claude API 키
    * `CLAUDE_CODE_OAUTH_TOKEN`: Claude 구독으로 인증하는 OAuth 토큰. Pro, Max, Team 및 Enterprise 플랜에서 사용 가능합니다. 로컬에서 `claude setup-token`을 실행하여 생성합니다. [장기 토큰 생성](/docs/ko/authentication#generate-a-long-lived-token)을 참조하십시오

    워크플로우 파일에서 시크릿을 일치하는 입력으로 전달합니다: API 키의 경우 `anthropic_api_key`, OAuth 토큰의 경우 `claude_code_oauth_token`.
  </Step>

  <Step title="워크플로우 파일 복사">
    [examples/claude.yml](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml)을 저장소의 `.github/workflows/` 디렉토리에 복사합니다. 파일은 예제가 아닌 작동하는 워크플로우입니다. 커밋된 상태로 Claude는 누군가 이슈나 풀 리퀘스트에서 `@claude`를 멘션할 때마다 응답합니다. `ANTHROPIC_API_KEY` 시크릿으로 인증합니다. 대신 `CLAUDE_CODE_OAUTH_TOKEN`을 추가한 경우 워크플로우의 `anthropic_api_key` 줄을 `claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}`으로 변경합니다.
  </Step>
</Steps>

<Tip>
  설정 후 이슈나 PR 댓글에서 `@claude`를 태그하여 Claude Code GitHub Action을 테스트합니다.
</Tip>

<h3 id="set-up-for-an-organization">
  조직을 위한 설정
</h3>

빠른 설정이나 수동 설정으로 한 번에 하나의 저장소를 구성합니다. Claude Code GitHub Action을 조직 전체에 배포하려면:

* [Claude GitHub App](https://github.com/apps/claude)을 조직 수준에서 한 번 설치합니다. 모든 저장소 또는 선택된 목록을 선택합니다
* 인증 시크릿을 조직 수준 Actions 시크릿으로 저장합니다. 각 저장소가 자신의 복사본을 가질 필요가 없습니다
* Claude Code GitHub Action을 실행해야 하는 각 저장소에 워크플로우 파일을 추가하거나, 각 저장소가 호출하는 [재사용 가능한 워크플로우](https://docs.github.com/en/actions/using-workflows/reusing-workflows)로 작업을 한 번 정의합니다

저장소 간에 공유되는 시크릿의 경우 OAuth 토큰은 `claude setup-token`을 실행한 사람의 구독에 연결되어 있으므로 [Claude Console](https://platform.claude.com)의 API 키로 인증합니다.

장기 시크릿을 저장하지 않으려면 워크로드 ID 페더레이션을 통해 인증합니다. Claude Code GitHub Action이 워크플로우의 GitHub OpenID Connect (OIDC) 토큰을 Claude Console 서비스 계정을 통해 Claude API 액세스로 교환합니다. 이러한 입력을 설정합니다:

* `anthropic_federation_rule_id`: 페더레이션 규칙 ID, `fdrl_...`
* `anthropic_organization_id`: Anthropic 조직 ID
* `anthropic_service_account_id`: 서비스 계정 ID, `svac_...`. 선택 사항입니다. Console에서 생성한 페더레이션 규칙이 이미 서비스 계정을 대상으로 하기 때문입니다
* `anthropic_workspace_id`: 워크스페이스 ID, `wrkspc_...`. 페더레이션 규칙이 단일 워크스페이스를 대상으로 할 때 선택 사항입니다

워크플로우에 `id-token: write` 권한을 부여합니다. Claude Code GitHub Action이 자신의 `github_token`을 전달할 때도 페더레이션 교환에 필요합니다. [Claude Code GitHub Action의 설정 가이드](https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md)에서 Console 측 구성을 참조하십시오.

보안 검토에서 데이터 처리 및 보존 질문의 경우 [데이터 사용](/docs/ko/data-usage) 및 [보안](/docs/ko/security)을 참조하십시오.

<h3 id="uninstall">
  제거
</h3>

Claude Code GitHub Action을 제거하려면 설치에 적용되는 각 설정 부분을 취소합니다:

* **워크플로우 파일**: `.github/workflows/`에서 `anthropics/claude-code-action`을 사용하는 워크플로우를 삭제합니다. 빠른 설정을 사용한 경우 `claude.yml`을 찾고, 리뷰 워크플로우를 선택한 경우 `claude-code-review.yml`을 찾습니다. 워크플로우가 삭제되면 Claude Code GitHub Action이 더 이상 실행되지 않습니다
* **시크릿**: 저장소에서 `ANTHROPIC_API_KEY` 또는 `CLAUDE_CODE_OAUTH_TOKEN` 시크릿을 삭제합니다. [저장소 간에 공유](#set-up-for-an-organization)한 경우 조직 수준 Actions 시크릿에서도 삭제합니다. 시크릿을 삭제하면 보유한 자격 증명이 유효한 상태로 유지됩니다. API 키를 완전히 폐기하려면 [Claude Console](https://platform.claude.com)에서도 키를 삭제합니다
* **GitHub App**: 저장소 또는 조직 설정의 GitHub Apps에서 Claude GitHub App을 제거합니다. Code Review나 웹 자동 수정과 같은 다른 Claude 기능에 사용하지 않는 경우에만 제거합니다

[클라우드 공급자](/docs/ko/github-actions-cloud-providers)를 구성한 경우 `AWS_ROLE_TO_ASSUME`, `GCP_*` 시크릿 또는 `AZURE_*` 시크릿과 같은 공급자 시크릿도 삭제합니다. `APP_ID` 및 `APP_PRIVATE_KEY` 시크릿과 함께 사용자 정의 GitHub App을 제거합니다.

<h3 id="github-app-permissions">
  GitHub App 권한
</h3>

[Claude GitHub App](https://github.com/apps/claude)은 Claude Code GitHub Action, [Code Review](/docs/ko/code-review) 및 클라우드 세션의 [풀 리퀘스트 자동 수정](/docs/ko/claude-code-on-the-web#auto-fix-pull-requests)을 포함하여 GitHub와 통합되는 모든 Claude 기능에서 공유됩니다. GitHub App은 모든 기능을 포함하는 단일 권한 집합을 가지므로 집합에는 Claude Code GitHub Action이 사용하지 않는 일부 권한이 포함됩니다.

앱을 설치하면 다음 권한을 부여합니다:

| 권한               | 액세스     |
| ---------------- | ------- |
| Actions          | 읽기 및 쓰기 |
| Checks           | 읽기 및 쓰기 |
| Contents         | 읽기 및 쓰기 |
| Discussions      | 읽기 및 쓰기 |
| Issues           | 읽기 및 쓰기 |
| Members          | 읽기      |
| Metadata         | 읽기      |
| Pull requests    | 읽기 및 쓰기 |
| Repository hooks | 읽기 및 쓰기 |
| Statuses         | 읽기      |
| Workflows        | 읽기 및 쓰기 |

권한 집합은 이를 사용하는 기능보다 먼저 변경될 수 있습니다. 앱이 이전에 없던 권한을 요청하면 GitHub가 계정 소유자에게 승인을 요청합니다. 조직 설치의 경우 조직 소유자에게 요청합니다. 설치는 승인할 때까지 이전 권한을 유지합니다. 예를 들어 Actions 액세스가 읽기에서 쓰기로 변경되면 앱이 워크플로우를 다시 실행할 수 있으므로 GitHub가 소유자에게 변경을 승인하도록 요청합니다.

앱을 설치하면 전체 권한 집합을 수락합니다. GitHub는 부분 집합을 수락하도록 허용하지 않습니다. 조직이 Claude Code GitHub Action이 사용하는 권한만 필요로 하는 경우 [Claude Code GitHub Action의 설정 가이드](https://github.com/anthropics/claude-code-action/blob/main/docs/setup.md)에 따라 Contents, Issues 및 Pull requests만 포함하는 사용자 정의 GitHub App을 생성합니다. 사용자 정의 앱은 Claude Code GitHub Action만 포함합니다. Code Review 및 웹 자동 수정에는 여전히 공식 앱이 필요합니다.

Claude Code GitHub Action이 이러한 권한으로 Claude가 할 수 있는 작업을 제한하는 방법에 대한 자세한 내용은 [보안 설명서](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md)를 참조하십시오.

<h2 id="interactive-and-automation-modes">
  대화형 및 자동화 모드
</h2>

Claude Code GitHub Action은 워크플로우 구성에서 실행 방법을 감지합니다:

* **대화형 모드**: 워크플로우가 `prompt` 입력을 제공하지 않으면 Claude는 트리거 구문인 `@claude`(기본값)를 이슈나 풀 리퀘스트 댓글, 풀 리퀨스트 리뷰 또는 새로 열린 이슈의 본문이나 제목에서 기다린 후 해당 요청에 응답합니다. 진행 상황과 결과는 트리거 이슈나 PR의 댓글로 나타납니다.
* **자동화 모드**: 워크플로우가 `prompt` 입력을 제공하면 Claude는 [실행을 트리거할 수 있는 사람](#who-can-trigger-runs)에 대한 확인만 받아 대기 없이 실행됩니다. 기본적으로 결과는 댓글이 아닌 워크플로우 실행 로그에 나타납니다. Claude는 프롬프트가 지시할 때 그리고 [코드 리뷰 예제](#run-a-skill)와 같이 게시할 수 있는 도구가 있을 때 이슈나 풀 리퀘스트에 게시할 수 있습니다.

<h3 id="who-can-trigger-runs">
  실행을 트리거할 수 있는 사람
</h3>

두 모드 모두에서 Claude Code GitHub Action은 Claude가 시작하기 전에 트리거 행위자에 대해 두 가지 확인을 실행합니다. 어느 확인이든 거부하면 실행이 실패합니다:

* **쓰기 액세스**: 이슈 및 풀 리퀘스트 이벤트에서 트리거 사용자는 저장소에 대한 쓰기 액세스 권한이 있어야 합니다. 쓰기 액세스 없이 특정 사용자를 허용하려면 `allowed_non_write_users`를 설정하고 자신의 `github_token` 입력을 전달합니다. `schedule` 트리거와 같이 사용자가 작성하지 않은 이벤트는 이 확인을 건너뜁니다.
* **인간 행위자**: 모든 이벤트에서 Claude Code GitHub Action은 `allowed_bots`에 나열하지 않는 한 봇 행위자를 거부합니다. 이는 봇이 루프에서 Claude를 트리거하는 것을 방지합니다. 이 확인은 GitHub가 저장소 사용자(일반적으로 워크플로우의 `cron` 일정을 마지막으로 변경한 사용자)에 귀속시키는 예약된 실행에도 적용됩니다. 해당 사용자가 봇이면 `allowed_bots`에 나열합니다.

<h2 id="example-use-cases">
  예제 사용 사례
</h2>

[examples 디렉토리](https://github.com/anthropics/claude-code-action/tree/main/examples)에는 다양한 시나리오에 대한 즉시 사용 가능한 워크플로우가 포함되어 있습니다.

이 페이지의 예제는 API 키 인증을 보여줍니다. Claude 구독으로 인증하는 경우 모든 예제의 `anthropic_api_key` 줄을 `claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}`으로 바꿉니다.

<h3 id="respond-to-claude-mentions">
  @claude 멘션에 응답
</h3>

이 워크플로우는 Claude Code GitHub Action을 대화형 모드에서 실행합니다. Claude는 누군가 이슈나 PR 댓글에서 `@claude`를 멘션할 때마다 응답합니다.

```yaml theme={null}
name: Claude Code
on:
  issue_comment:
    types: [created]
  pull_request_review_comment:
    types: [created]
jobs:
  claude:
    if: contains(github.event.comment.body, '@claude')
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: write
      issues: write
      id-token: write
      actions: read
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 1
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
```

이 워크플로우에서 보일러플레이트가 아닌 부분:

* `id-token: write`: Claude Code GitHub Action의 기본 GitHub App 인증에 필요합니다
* `actions: read`: Claude가 PR에서 CI 결과를 읽을 수 있게 합니다
* `actions/checkout`: Claude에게 작업할 저장소의 로컬 복사본을 제공합니다
* `if`: `@claude`를 멘션하지 않는 댓글에서 러너가 시작되지 않도록 합니다. Claude Code GitHub Action도 응답하기 전에 트리거 구문 자체를 확인합니다

워크플로우가 준비되면 이슈나 PR 댓글에서 `@claude`를 멘션하고 요청을 합니다:

```text wrap theme={null}
@claude implement this feature based on the issue description
@claude how should I implement user authentication for this endpoint?
@claude fix the TypeError in the user dashboard component
```

Claude가 동일한 이슈나 PR의 댓글로 응답하고 작업하면서 업데이트합니다.

<h3 id="run-a-skill">
  스킬 실행
</h3>

`prompt` 입력은 일반 텍스트뿐만 아니라 [스킬](/docs/ko/skills) 호출도 허용합니다:

* 저장소의 `.claude/skills/` 디렉토리에 있는 스킬의 경우 `anthropics/claude-code-action` 단계 전에 `actions/checkout`을 실행하여 스킬 파일을 러너에서 사용할 수 있게 한 후 `/skill-name`을 `prompt`로 전달합니다.
* [플러그인](/docs/ko/plugins/overview)에 패키징된 스킬의 경우 `plugin_marketplaces` 및 `plugins` 입력으로 플러그인을 설치한 후 네임스페이스가 지정된 `/plugin-name:skill-name`을 `prompt`로 전달합니다. `plugins` 입력은 `plugin-name@marketplace-name`을 사용합니다. 마켓플레이스 이름은 저장소 URL이 아닌 마켓플레이스 자체의 매니페스트에서 나옵니다.

다음 워크플로우는 `code-review` 플러그인을 설치하고 풀 리퀘스트가 열리거나, 업데이트되거나, 다시 열리거나, 리뷰 준비로 표시될 때 해당 스킬을 실행합니다. 빠른 설정의 리뷰 워크플로우와 동일한 플러그인을 실행합니다. 프롬프트, 모델 및 트리거를 직접 제어하려는 경우 이와 같은 워크플로우를 사용합니다. 워크플로우 파일을 유지하지 않고 자동 리뷰의 경우 [Code Review](/docs/ko/code-review)를 참조하십시오. 공개 저장소에서 GitHub는 포크 풀 리퀘스트로 트리거된 실행에서 시크릿을 보류하므로 리뷰는 동일한 저장소의 브랜치에서 풀 리퀘스트에서만 실행됩니다.

```yaml theme={null}
name: Code Review
on:
  pull_request:
    types: [opened, synchronize, ready_for_review, reopened]
jobs:
  review:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: read
      issues: read
      id-token: write
    steps:
      - uses: actions/checkout@v6
        with:
          fetch-depth: 1
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          plugin_marketplaces: "https://github.com/anthropics/claude-code.git"
          plugins: "code-review@claude-code-plugins"
          prompt: "/code-review:code-review --comment ${{ github.repository }}/pull/${{ github.event.pull_request.number }}"
          claude_args: '--allowedTools "mcp__github_inline_comment__create_inline_comment"'
```

이 워크플로우에서 리뷰가 어디로 가는지 제어하는 두 줄:

* **`--comment`**: Claude가 풀 리퀘스트에 리뷰를 게시합니다. 발견한 각 이슈에 대한 인라인 댓글로 또는 발견하지 못한 경우 하나의 요약 댓글로 게시합니다. 없으면 Claude는 아무것도 게시하지 않으며 워크플로우 실행 로그에서 결과를 읽습니다.
* **`claude_args`**: 스킬 자체의 `allowed-tools` frontmatter가 동일한 도구를 명명하더라도 이 줄을 유지합니다. Claude Code GitHub Action이 `claude_args`의 `--allowedTools`가 이를 명명할 때만 인라인 댓글을 게시하는 MCP 서버를 시작하기 때문입니다.

Claude는 초안 및 닫힌 풀 리퀘스트, 리뷰가 필요하지 않다고 판단하는 풀 리퀘스트(자동화되거나 사소한 것 등), 그리고 이미 Claude의 댓글이 있는 풀 리퀘스트를 건너뜁니다.

<h3 id="run-on-a-schedule">
  일정에 따라 실행
</h3>

`prompt` 입력으로 Claude Code GitHub Action은 cron 일정을 포함한 모든 GitHub 이벤트에서 자동화 모드에서 실행됩니다. 일반 텍스트 프롬프트의 경우 Claude는 프롬프트가 필요로 하는 도구에 대해 `claude_args`의 `--allowedTools` 또는 `settings` 입력의 [`permissions.allow` 규칙](/docs/ko/permissions#permission-rule-syntax)으로 도구를 부여할 때까지 셸이나 GitHub API 액세스 권한이 없습니다. 대신 스킬을 호출하면 Claude는 [`allowed-tools` frontmatter](/docs/ko/skills#pre-approve-tools-for-a-skill)가 부여하는 도구를 사용할 수 있습니다. GitHub는 예약된 워크플로우를 기본 브랜치에서만 실행하며, 공개 저장소에서는 저장소 활동이 60일 없으면 일정을 비활성화합니다.

이 워크플로우는 매일 09:00 UTC에 워크플로우 실행 로그에서 보고서를 생성합니다. 해당 `claude_args` 줄은 [CLI 인수를 전달](#pass-cli-arguments)하여 모델을 선택하고 두 개의 GitHub MCP 도구를 허용합니다. Claude가 해당 도구로 GitHub API를 통해 커밋과 이슈를 읽으므로 체크아웃 단계를 생략할 수 있습니다:

```yaml theme={null}
name: Daily Report
on:
  schedule:
    - cron: "0 9 * * *"
jobs:
  report:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      issues: read
      id-token: write
    steps:
      - uses: anthropics/claude-code-action@v1
        with:
          anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}
          prompt: "Generate a summary of yesterday's commits and open issues"
          claude_args: |
            --model claude-opus-5-5
            --allowedTools "mcp__github__list_commits,mcp__github__list_issues"
```

<h2 id="best-practices">
  모범 사례
</h2>

<h3 id="define-project-standards-in-claude-md">
  CLAUDE.md에서 프로젝트 표준 정의
</h3>

저장소 루트에 `CLAUDE.md` 파일을 생성하여 코드 스타일 지침, 리뷰 기준, 프로젝트별 규칙 및 선호하는 패턴을 정의합니다. Claude는 PR을 생성하고 요청에 응답할 때 이러한 지침을 따릅니다. 자세한 내용은 [메모리 설명서](/docs/ko/memory)를 참조하십시오.

<h3 id="protect-your-credentials">
  자격 증명 보호
</h3>

<Warning>
  API 키나 OAuth 토큰을 저장소에 직접 커밋하지 마십시오. 항상 GitHub Secrets로 저장하고 워크플로우에서 참조합니다. 예: `anthropic_api_key: ${{ secrets.ANTHROPIC_API_KEY }}`.
</Warning>

워크플로우에 필요한 권한만 부여하고 병합하기 전에 Claude의 변경 사항을 검토합니다.

권한 및 인증을 포함한 포괄적인 보안 지침은 [Claude Code Action 보안 설명서](https://github.com/anthropics/claude-code-action/blob/main/docs/security.md)를 참조하십시오.

<h3 id="manage-costs">
  비용 관리
</h3>

각 실행은 두 가지 종류의 리소스를 소비합니다:

* **GitHub Actions 분**: Claude Code GitHub Action은 GitHub 호스팅 러너에서 실행되며, GitHub Actions 분을 소비합니다. 가격 책정 및 분 제한은 [GitHub의 청구 설명서](https://docs.github.com/en/billing/managing-billing-for-your-products/managing-billing-for-github-actions/about-billing-for-github-actions)를 참조하십시오.
* **API 토큰**: 각 상호 작용은 프롬프트 및 응답의 길이, 작업 복잡도 및 코드베이스 크기에 따라 토큰을 소비합니다. 현재 토큰 요금은 [Claude의 가격 책정 페이지](https://claude.com/platform/api)를 참조하십시오. OAuth 토큰으로 인증하면 실행이 API 청구 대신 Claude 구독을 사용합니다.

Claude에게 더 명확한 컨텍스트를 제공하고 각 실행이 수행할 수 있는 작업의 양을 제한하여 두 종류의 비용을 낮출 수 있습니다:

* 특정 `@claude` 요청을 작성하여 Claude가 완료하는 데 더 적은 턴이 필요하도록 합니다
* 이슈 템플릿을 사용하여 미리 컨텍스트를 제공합니다
* `CLAUDE.md`를 간결하게 유지합니다. Claude가 모든 실행에서 이를 읽기 때문입니다
* `claude_args`에서 `--max-turns`를 설정하여 반복을 제한합니다
* 워크플로우 수준 타임아웃을 설정하여 실행 중인 작업을 방지합니다
* GitHub의 동시성 제어를 사용하여 병렬 실행을 제한합니다

조직 전체의 사용 추적은 [분석 대시보드](/docs/ko/analytics) 및 [모니터링](/docs/ko/monitoring-usage)을 참조하십시오. 사용이 측정되고 청구되는 방법은 [비용](/docs/ko/costs)을 참조하십시오.

<h2 id="use-a-cloud-provider">
  클라우드 공급자 사용
</h2>

기본적으로 Claude Code GitHub Action은 API 키나 OAuth 토큰으로 Claude API를 직접 호출합니다. 대신 자신의 클라우드 계정을 통해 추론을 라우팅하려면 공급자에 대한 입력을 설정하고 [클라우드 공급자와 함께 Claude Code GitHub Actions 사용](/docs/ko/github-actions-cloud-providers)을 따릅니다:

* **Amazon Bedrock**: `use_bedrock: "true"`
* **Google Cloud의 Agent Platform**: `use_vertex: "true"`
* **Microsoft Foundry**: `use_foundry: "true"`

세 공급자 모두에서 Claude API 키 대신 OIDC ID 페더레이션을 통해 인증하므로 저장소에 정적 클라우드 자격 증명을 저장하지 않습니다.

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="claude-not-responding-to-claude-commands">
  Claude가 @claude 명령에 응답하지 않음
</h3>

* GitHub App이 저장소에 설치되었는지 확인합니다
* 저장소에 대해 워크플로우가 활성화되었는지 확인합니다
* API 키나 OAuth 토큰이 저장소 시크릿에 설정되었는지 확인합니다
* 댓글에 `/claude`나 `@claude-bot`이 아닌 완전한 단어로 `@claude`가 포함되어 있는지 확인합니다
* 댓글 작성자가 저장소에 대한 쓰기 액세스 권한이 있는지 확인합니다. [실행을 트리거할 수 있는 사람](#who-can-trigger-runs)에서 예외를 참조하십시오

<h3 id="ci-not-running-on-claude’s-commits">
  Claude의 커밋에서 CI가 실행되지 않음
</h3>

* GitHub는 기본 `GITHUB_TOKEN`으로 만든 커밋에서 워크플로우를 트리거하지 않습니다. Claude Code GitHub Action에 `github_token: ${{ secrets.GITHUB_TOKEN }}`을 전달하면 Claude GitHub App으로 인증하도록 제거하거나 대신 사용자 정의 앱 토큰을 전달합니다
* CI 워크플로우의 트리거에 Claude의 푸시가 생성하는 이벤트(예: `push` 또는 `pull_request`)가 포함되어 있는지 확인합니다

<h3 id="authentication-errors">
  인증 오류
</h3>

* 워크플로우를 디버깅하기 전에 로컬에서 `claude`로 테스트하여 API 키나 OAuth 토큰이 유효한지 확인합니다
* Bedrock, Agent Platform 및 Foundry의 경우 클라우드 공급자 페이지의 [문제 해결 섹션](/docs/ko/github-actions-cloud-providers#troubleshooting)을 참조하십시오

더 많은 솔루션은 Claude Code GitHub Action의 [FAQ](https://github.com/anthropics/claude-code-action/blob/main/docs/faq.md)를 참조하십시오.

<h2 id="advanced-configuration">
  고급 구성
</h2>

<h3 id="action-parameters">
  작업 파라미터
</h3>

이는 가장 일반적으로 사용되는 입력입니다. 각각은 `anthropics/claude-code-action` 단계의 `with:` 키에 매핑됩니다.

| 파라미터                      | 설명                                                                                                              | 필수                                                                                                                                                        |
| ------------------------- | --------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`                  | Claude에 대한 지침. 일반 텍스트 또는 [스킬](/docs/ko/skills) 호출. 생략하면 Claude는 [트리거 구문](#interactive-and-automation-modes) 대신 응답합니다 | 아니오                                                                                                                                                       |
| `claude_args`             | Claude Code에 전달된 CLI 인수                                                                                         | 아니오                                                                                                                                                       |
| `anthropic_api_key`       | Claude API 키                                                                                                    | Claude API의 경우 `claude_code_oauth_token` 또는 [워크로드 ID 페더레이션](#set-up-for-an-organization)을 사용하지 않는 한 필수입니다. Bedrock, Agent Platform 또는 Foundry에는 사용되지 않습니다 |
| `claude_code_oauth_token` | Claude 구독으로 인증하기 위한 OAuth 토큰. `claude setup-token`으로 생성됩니다                                                      | 아니오                                                                                                                                                       |
| `github_token`            | GitHub 작업용 토큰. 생략하면 Claude Code GitHub Action이 Claude GitHub App으로 인증합니다                                        | 아니오                                                                                                                                                       |
| `plugin_marketplaces`     | 플러그인 마켓플레이스 Git URL의 줄 바꿈으로 구분된 목록                                                                              | 아니오                                                                                                                                                       |
| `plugins`                 | 실행 전에 설치할 플러그인 이름의 줄 바꿈으로 구분된 목록                                                                                | 아니오                                                                                                                                                       |
| `settings`                | Claude Code 설정. JSON 문자열 또는 설정 JSON 파일의 경로                                                                      | 아니오                                                                                                                                                       |
| `trigger_phrase`          | Claude가 응답하는 트리거 구문. 기본값: `@claude`                                                                             | 아니오                                                                                                                                                       |
| `use_bedrock`             | Claude API 대신 Amazon Bedrock 사용                                                                                 | 아니오                                                                                                                                                       |
| `use_vertex`              | Claude API 대신 Google Cloud의 Agent Platform 사용                                                                   | 아니오                                                                                                                                                       |
| `use_foundry`             | Claude API 대신 Microsoft Foundry 사용                                                                              | 아니오                                                                                                                                                       |

전체 입력 목록은 Claude Code GitHub Action의 [구성 참조](https://github.com/anthropics/claude-code-action/blob/main/docs/usage.md#inputs)를 참조하십시오.

<h3 id="pass-cli-arguments">
  CLI 인수 전달
</h3>

`claude_args` 파라미터는 모든 [Claude Code CLI 인수](/docs/ko/cli-reference)를 허용합니다:

```yaml theme={null}
claude_args: "--max-turns 5 --model claude-sonnet-5 --mcp-config /path/to/config.json"
```

일반적인 인수:

* `--max-turns`: 대화 턴 수 제한
* `--model`: 사용할 모델. 예: `claude-sonnet-5`. 이 인수 없으면 Claude Code GitHub Action이 Claude Code [기본 모델](/docs/ko/model-config)을 사용합니다
* `--mcp-config`: [MCP 구성](/docs/ko/mcp) 경로
* `--allowedTools`: 허용된 도구의 쉼표로 구분된 목록. `--allowed-tools` 별칭도 작동합니다
* `--debug`: 디버그 출력 활성화

<h2 id="upgrade-from-beta">
  베타에서 업그레이드
</h2>

워크플로우가 여전히 `anthropics/claude-code-action@beta`를 참조하면 v1로 업데이트합니다:

1. `uses` 줄에서 `@beta`를 `@v1`로 변경합니다
2. `mode` 입력을 제거합니다. Claude Code GitHub Action이 이제 [모드를 자동으로 감지](#interactive-and-automation-modes)하기 때문입니다
3. `direct_prompt`를 `prompt`로 바꿉니다
4. `max_turns` 및 `model`과 같은 CLI 옵션을 `claude_args`로 이동합니다. `custom_instructions`는 같은 이름의 플래그가 없으며 `--append-system-prompt`가 됩니다

전체 입력 매핑 및 이전/이후 예제는 [마이그레이션 가이드](https://github.com/anthropics/claude-code-action/blob/main/docs/migration-guide.md)를 참조하십시오.

<h2 id="what’s-next">
  다음 단계
</h2>

* [클라우드 공급자와 함께 Claude Code GitHub Actions 사용](/docs/ko/github-actions-cloud-providers): Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry를 통해 추론을 라우팅합니다
* [구성 참조](https://github.com/anthropics/claude-code-action/blob/main/docs/usage.md#inputs): 작업 입력의 전체 목록
* [Examples 디렉토리](https://github.com/anthropics/claude-code-action/tree/main/examples): 더 많은 시나리오에 대한 즉시 사용 가능한 워크플로우
* [Code Review](/docs/ko/code-review): 워크플로우 파일을 유지하지 않고 자동 풀 리퀘스트 리뷰
