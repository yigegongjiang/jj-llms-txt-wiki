> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# GitHub Enterprise Server와 Claude Code

> 자체 호스팅되는 GitHub Enterprise Server 인스턴스에 Claude Code를 연결하여 웹 세션, 코드 리뷰 및 플러그인 마켓플레이스를 사용합니다.

<Note>
  GitHub Enterprise Server 지원은 Team 및 Enterprise 플랜에서 사용 가능합니다.
</Note>

GitHub Enterprise Server(GHES) 지원을 통해 조직은 github.com 대신 자체 관리되는 GitHub 인스턴스에 호스팅된 저장소와 함께 Claude Code를 사용할 수 있습니다. 관리자가 GHES 인스턴스를 연결하면 개발자는 저장소별 구성 없이 웹 세션을 실행하고 자동화된 코드 리뷰를 받을 수 있습니다. 인스턴스에 호스팅된 플러그인 마켓플레이스도 지원되며, 자격 증명 요구 사항은 [GHES의 플러그인 마켓플레이스](#plugin-marketplaces-on-ghes)에 설명된 대로 표면에 따라 다릅니다.

github.com의 저장소의 경우 [웹에서 Claude Code](/docs/ko/claude-code-on-the-web) 및 [코드 리뷰](/docs/ko/code-review)를 참조하십시오. 자신의 CI 인프라에서 Claude를 실행하려면 [GitHub Actions](/docs/ko/github-actions)를 참조하십시오.

<h2 id="what-works-with-github-enterprise-server">
  GitHub Enterprise Server에서 작동하는 기능
</h2>

아래 표는 Claude Code의 어떤 기능이 GHES를 지원하는지 및 github.com 동작과의 차이점을 보여줍니다.

| 기능              | GHES 지원   | 참고                                                                                                     |
| :-------------- | :-------- | :----------------------------------------------------------------------------------------------------- |
| 웹에서 Claude Code | ✅ 지원됨     | 관리자가 GHES 인스턴스를 한 번 연결하면 개발자는 `claude --cloud` 또는 [claude.ai/code](https://claude.ai/code)를 평소처럼 사용합니다 |
| 코드 리뷰           | ✅ 지원됨     | github.com과 동일한 자동화된 PR 리뷰                                                                             |
| Claude Security | ✅ 지원됨     | Enterprise 플랜의 공개 베타에서 [claude.ai/security](https://claude.ai/security)에서 사용 가능                        |
| Teleport 세션     | ✅ 지원됨     | `--teleport`를 사용하여 웹과 터미널 간에 세션 이동                                                                     |
| 플러그인 마켓플레이스     | ✅ 지원됨     | 표면별로 자격증명 요구사항이 다릅니다. [GHES의 플러그인 마켓플레이스](#plugin-marketplaces-on-ghes)를 참조하세요                         |
| 기여도 메트릭         | ✅ 지원됨     | [분석 대시보드](/docs/ko/analytics)로 웹훅을 통해 전달됨                                                                   |
| GitHub Actions  | ✅ 지원됨     | 수동 워크플로우 설정 필요; `/install-github-app`은 github.com 전용                                                   |
| GitHub MCP 서버   | ❌ 지원되지 않음 | GitHub MCP 서버는 GHES 인스턴스와 작동하지 않습니다                                                                    |

<h2 id="admin-setup">
  관리자 설정
</h2>

관리자가 GHES 인스턴스를 Claude Code에 한 번 연결합니다. 그 후 조직의 개발자는 추가 구성 없이 GHES 저장소를 사용할 수 있습니다. Claude 조직에 대한 관리자 또는 주 관리자 역할과 GHES 인스턴스에서 GitHub App을 만들 수 있는 권한이 필요합니다.

안내식 설정은 GitHub App 매니페스트를 생성하고 한 번의 클릭으로 앱을 만들기 위해 GHES 인스턴스로 리디렉션합니다. 환경이 리디렉션 흐름을 차단하는 경우 [대체 수동 설정](#manual-setup)을 사용할 수 있습니다.

<Steps>
  <Step title="Claude Code 관리자 설정 열기">
    [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)로 이동하여 GitHub Enterprise Server 섹션을 찾습니다.
  </Step>

  <Step title="안내식 설정 시작">
    **연결**을 클릭합니다. 연결의 표시 이름과 GHES 호스트명(예: `github.example.com`)을 입력합니다. GHES 인스턴스가 자체 서명 또는 개인 인증 기관 인증서를 사용하는 경우 선택적 필드에 CA 인증서를 붙여넣습니다.
  </Step>

  <Step title="GitHub App 만들기">
    **GitHub Enterprise로 계속**을 클릭합니다. 브라우저가 미리 채워진 앱 매니페스트와 함께 GHES 인스턴스로 리디렉션됩니다. 구성을 검토하고 **GitHub App 만들기**를 클릭합니다. GHES가 앱 자격 증명이 자동으로 저장된 상태로 Claude로 리디렉션합니다.
  </Step>

  <Step title="저장소에 앱 설치">
    GHES 인스턴스의 GitHub App 페이지에서 Claude가 액세스하기를 원하는 저장소 또는 조직에 앱을 설치합니다. 부분 집합으로 시작하여 나중에 더 추가할 수 있습니다.
  </Step>

  <Step title="기능 활성화">
    [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)로 돌아가서 github.com과 동일한 구성을 사용하여 GHES 저장소에 대해 [코드 리뷰](/docs/ko/code-review#set-up-code-review), Claude Security 및 [기여도 메트릭](/docs/ko/analytics#enable-contribution-metrics)을 활성화합니다.
  </Step>
</Steps>

<h3 id="github-app-permissions">
  GitHub App 권한
</h3>

매니페스트는 웹 세션, 코드 리뷰, Claude Security 및 기여도 메트릭 전반에 걸쳐 Claude가 필요로 하는 권한 및 웹훅 이벤트로 GitHub App을 구성합니다:

| 권한               | 액세스     | 사용 목적              |
| :--------------- | :------ | :----------------- |
| Contents         | 읽기 및 쓰기 | 저장소 복제 및 분기 푸시     |
| Pull requests    | 읽기 및 쓰기 | PR 생성 및 리뷰 의견 게시   |
| Issues           | 읽기 및 쓰기 | 문제 언급에 응답          |
| Checks           | 읽기 및 쓰기 | 코드 리뷰 확인 실행 게시     |
| Actions          | 읽기      | 자동 수정을 위한 CI 상태 읽기 |
| Repository hooks | 읽기 및 쓰기 | 기여도 메트릭을 위한 웹훅 수신  |
| Metadata         | 읽기      | 모든 앱에 GitHub에서 필요  |

앱은 `pull_request`, `issue_comment`, `pull_request_review_comment`, `pull_request_review` 및 `check_run` 이벤트를 구독합니다.

<h3 id="manual-setup">
  수동 설정
</h3>

안내식 리디렉션 흐름이 네트워크 구성에 의해 차단되는 경우 연결 대신 **수동으로 추가**를 클릭합니다. [위의 권한 및 이벤트](#github-app-permissions)를 사용하여 GHES 인스턴스에서 GitHub App을 만든 다음 앱 자격 증명을 양식에 입력합니다: 호스트명, OAuth 클라이언트 ID 및 비밀, GitHub App ID, 클라이언트 ID, 클라이언트 비밀, 웹훅 비밀 및 개인 키.

<h3 id="network-requirements">
  네트워크 요구 사항
</h3>

GHES 인스턴스는 Claude가 저장소를 복제하고 리뷰 의견을 게시할 수 있도록 Anthropic 인프라에서 도달 가능해야 합니다. GHES 인스턴스가 방화벽 뒤에 있는 경우 [Anthropic API IP 주소](https://platform.claude.com/docs/en/api/ip-addresses)를 허용 목록에 추가합니다.

<h2 id="developer-workflow">
  개발자 워크플로우
</h2>

관리자가 GHES 인스턴스를 연결하면 개발자 측 구성이 필요하지 않습니다. Claude Code는 작업 디렉토리의 git 원격에서 GHES 호스트명을 자동으로 감지합니다.

평소처럼 GHES 인스턴스에서 저장소를 복제합니다:

```bash theme={null}
git clone git@github.example.com:platform/api-service.git
cd api-service
```

그런 다음 웹 세션을 시작합니다. Claude는 git 원격에서 GHES 호스트를 감지하고 세션을 조직의 구성된 인스턴스를 통해 라우팅합니다:

```bash theme={null}
claude --cloud "Add retry logic to the payment webhook handler"
```

세션은 Anthropic 인프라에서 실행되고, GHES에서 저장소를 복제하며, 변경 사항을 분기로 다시 푸시합니다. `/tasks`를 사용하거나 [claude.ai/code](https://claude.ai/code)에서 진행 상황을 모니터링합니다. diff 리뷰, 자동 수정 및 루틴을 포함한 전체 클라우드 세션 워크플로우는 [웹에서 Claude Code](/docs/ko/claude-code-on-the-web)를 참조하십시오.

<h3 id="teleport-sessions-to-your-terminal">
  터미널로 세션 Teleport
</h3>

`claude --teleport`를 사용하여 웹 세션을 로컬 터미널로 가져옵니다. Teleport는 분기를 가져오고 세션 기록을 로드하기 전에 동일한 GHES 저장소의 체크아웃에 있는지 확인합니다. 자세한 내용은 [teleport 요구 사항](/docs/ko/claude-code-on-the-web#teleport-requirements)을 참조하십시오.

<h2 id="plugin-marketplaces-on-ghes">
  GHES의 플러그인 마켓플레이스
</h2>

조직 전체에 내부 도구를 배포하기 위해 GHES 인스턴스에서 플러그인 마켓플레이스를 호스팅합니다. 마켓플레이스 구조는 github.com 호스팅 마켓플레이스와 동일하지만, 마켓플레이스를 추가하는 위치에 따라 설치 방식이 다르며, 표면에 따라 자격 증명이 다릅니다:

| 표면                                 | 설치 방식                                                                                                                  | 각 사용자가 필요한 것                                                                                                           |
| :--------------------------------- | :--------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------- |
| Claude Code CLI 및 데스크톱             | Claude Code는 머신의 기존 git 자격 증명을 사용하여 마켓플레이스 저장소를 복제합니다                                                                  | 머신에서 GHES 호스트에 대한 Git 액세스                                                                                              |
| 관리되는 설정 (`extraKnownMarketplaces`) | Claude Code는 항목을 등록하고 머신의 기존 git 자격 증명을 사용하여 저장소를 복제합니다                                                                | 머신에서 GHES 호스트에 대한 Git 액세스                                                                                              |
| claude.ai 조직 플러그인 설정               | 소유자가 GHES 인스턴스를 소스로 선택합니다. Anthropic의 백엔드는 [관리자 설정](#admin-setup)의 GitHub App을 사용하여 저장소를 가져오고 동기화합니다                   | 추가된 후 사용자당 필요한 것이 없습니다. 이를 추가하는 소유자는 액세스 확인으로 자신의 GitHub Enterprise 계정을 연결해야 하며, GitHub App은 마켓플레이스 저장소에 설치되어야 합니다     |
| claude.ai 사용자 설정                   | Anthropic의 백엔드는 제출하는 사용자의 GitHub Enterprise 연결을 사용하여 저장소를 가져옵니다                                                        | Claude에 연결된 자신의 GitHub Enterprise 계정                                                                                   |
| 웹의 Claude Code                     | 클라우드 세션은 세션 샌드박스 내에서 마켓플레이스를 복제합니다. 샌드박스는 세션의 저장소가 동일한 인스턴스에 있을 때만 GHES 인스턴스에 도달할 수 있으며, git 자격 증명은 세션의 저장소로 범위가 지정됩니다 | GHES 호스팅 마켓플레이스에는 신뢰할 수 없습니다: 세션의 저장소와 다른 호스트는 도달할 수 없으며, 동일한 인스턴스 설치도 실패할 수 있습니다. 대신 CLI, 관리되는 설정 또는 claude.ai를 사용합니다 |

<Warning>
  claude.ai의 GitHub Enterprise 연결은 사용자 설정에서 마켓플레이스를 추가할 때 사용자별입니다. [관리자 설정](#admin-setup)은 GHES 인스턴스를 조직에 연결하지만 개별 사용자 계정은 연결하지 않습니다: 자신의 설정에서 GHES 마켓플레이스를 추가하는 각 사용자는 먼저 자신의 GitHub Enterprise 계정을 연결해야 하며, 소유자를 포함한 한 사용자의 연결은 다른 사람을 포함하지 않습니다. 조직 플러그인 설정에서 소유자가 추가한 마켓플레이스는 지속적인 가져오기가 조직의 GitHub App을 사용하기 때문에 사용자에게 이 요구 사항을 부과하지 않습니다. 마켓플레이스를 추가하는 소유자는 여전히 추가 시간에 자신의 GitHub Enterprise 계정을 연결해야 합니다.
</Warning>

<h3 id="add-a-ghes-marketplace">
  GHES 마켓플레이스 추가
</h3>

`owner/repo` 단축형은 항상 github.com으로 확인됩니다. GHES 호스팅 마켓플레이스의 경우 전체 git URL을 사용합니다. HTTPS URL이 권장됩니다:

```bash theme={null}
/plugin marketplace add https://github.example.com/platform/claude-plugins.git
```

머신이 이미 GHES 호스트를 신뢰하는 경우 SSH URL이 작동합니다:

```bash theme={null}
/plugin marketplace add git@github.example.com:platform/claude-plugins.git
```

Claude Code는 git을 비대화형으로 실행하며 머신의 `known_hosts` 파일에 없는 호스트에 대한 SSH 연결을 거부합니다. git 자격 증명 도우미가 있는 HTTPS URL은 `known_hosts` 요구 사항을 피합니다.

마켓플레이스 구축에 대한 전체 가이드는 [플러그인 마켓플레이스 만들기 및 배포](/docs/ko/plugin-marketplaces)를 참조합니다.

<h3 id="pre-register-ghes-marketplaces-with-managed-settings">
  관리되는 설정으로 GHES 마켓플레이스 사전 등록
</h3>

`extraKnownMarketplaces` 설정은 마켓플레이스를 사전 등록하여 개발자가 수동 설정 없이 이를 얻을 수 있습니다. 이는 저장소의 `.claude/settings.json`을 포함한 [모든 설정 파일](/docs/ko/settings#extraknownmarketplaces)에서 작동합니다. 관리되는 설정은 조직 전체에 이를 제공합니다:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "internal-tools": {
      "source": {
        "source": "git",
        "url": "https://github.example.com/platform/claude-plugins.git"
      }
    }
  }
}
```

Claude Code는 이러한 마켓플레이스를 로컬로 설치합니다: 각 항목을 등록하고 머신의 기존 git 자격 증명으로 저장소를 복제합니다. 이 경로는 claude.ai를 거치지 않으므로 사용자별 GitHub Enterprise 연결이 필요하지 않습니다. 성공적인 배포를 위해:

* **전체 git URL을 사용합니다.** `owner/repo` 단축형은 항상 github.com으로 확인되며 GHES 호스트를 참조할 수 없습니다.
* **HTTPS URL을 선호합니다.** SSH 복제는 이미 GHES 호스트 키를 신뢰하지 않는 머신에서 실패합니다. 조직의 표준 git 자격 증명 도우미가 있는 HTTPS URL은 자격 증명이 구성된 모든 머신에서 작동합니다.
* **각 머신이 GHES 호스트에서 복제할 수 있는지 확인합니다.** 머신에 자격 증명이 없으면 마켓플레이스는 등록되지만 설치되지 않으며, 플러그인은 자격 증명을 요청하는 대신 찾을 수 없음으로 보고됩니다.
* **설정이 각 머신에 도달하는지 확인합니다.** 관리되는 설정 파일은 배포되는 머신에만 적용됩니다(예: 장치 관리 시스템을 통해). 파일 위치는 [관리되는 설정](/docs/ko/settings#settings-files)을 참조합니다.

<h3 id="allowlist-ghes-marketplaces-in-managed-settings">
  관리되는 설정에서 GHES 마켓플레이스 허용 목록
</h3>

조직이 [관리되는 설정](/docs/ko/settings)을 사용하여 개발자가 추가할 수 있는 마켓플레이스를 제한하는 경우 `hostPattern` 소스 유형을 사용하여 각 저장소를 열거하지 않고 GHES 인스턴스의 모든 마켓플레이스를 허용합니다:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "hostPattern",
      "hostPattern": "^github\\.example\\.com$"
    }
  ]
}
```

전체 스키마는 [strictKnownMarketplaces](/docs/ko/settings#strictknownmarketplaces) 및 [extraKnownMarketplaces](/docs/ko/settings#extraknownmarketplaces) 설정 참조를 참조하십시오.

<h2 id="limitations">
  제한 사항
</h2>

몇 가지 기능은 GHES에서 github.com과 다르게 동작합니다. [기능 표](#what-works-with-github-enterprise-server)는 지원을 요약합니다. 이 섹션에서는 해결 방법을 다룹니다.

* **`/install-github-app` 명령**: claude.ai에서 [관리자 설정](#admin-setup) 흐름을 따릅니다. GHES에서 GitHub Actions 워크플로우도 원하는 경우 [예제 워크플로우](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml)를 수동으로 조정합니다.
* **GitHub MCP 서버**: 대신 GHES 호스트에 대해 구성된 `gh` CLI를 사용합니다. `gh auth login --hostname github.example.com`을 실행하여 인증한 다음 Claude는 세션에서 `gh` 명령을 사용할 수 있습니다.

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="web-session-fails-to-clone-repository">
  웹 세션이 저장소 복제에 실패함
</h3>

`claude --cloud`가 복제 오류로 실패하면 Owner가 GHES 인스턴스에 대한 설정을 완료했는지 확인하고 GitHub App이 작업 중인 저장소에 설치되어 있는지 확인합니다. 인스턴스를 연결한 Owner에게 Claude 설정에 등록된 호스트명이 git 원격의 호스트명과 일치하는지 확인하도록 요청합니다.

<h3 id="marketplace-add-fails-with-a-policy-error">
  마켓플레이스 추가가 정책 오류로 실패함
</h3>

GHES URL에 대해 `/plugin marketplace add`가 차단되면 조직이 마켓플레이스 소스를 제한했습니다. 관리자에게 [관리되는 설정](#allowlist-ghes-marketplaces-in-managed-settings)에서 GHES 호스트명에 대한 `hostPattern` 항목을 추가하도록 요청합니다.

<h3 id="marketplace-add-on-claude-ai-fails-with-a-github-access-error">
  claude.ai에서 마켓플레이스 추가가 GitHub 액세스 오류로 실패함
</h3>

사용자 설정에서 GHES 마켓플레이스를 추가하는 것이 "마켓플레이스를 추가할 수 없음"과 같은 일반적인 오류로 실패하면 먼저 GitHub Enterprise 연결을 확인합니다. 이는 조직의 GHES 인스턴스가 구성되어 있고 다른 사용자가 연결되어 있더라도 자신의 GitHub Enterprise 계정이 Claude에 연결되지 않았을 때 나타나는 현상입니다. 대화 상자는 GitHub Enterprise 연결 흐름을 가리키지 않으며, Browse 탭의 "GitHub에 연결" 옵션은 github.com에 로그인하므로 GHES 저장소에 대한 액세스 권한을 부여하지 않습니다.

GitHub Enterprise 계정을 연결하려면: [claude.ai/code](https://claude.ai/code)의 저장소 선택기가 각 구성된 GHES 인스턴스에 대한 연결 옵션을 제공하며, Owner는 [Claude Code 관리자 설정](https://claude.ai/admin-settings/claude-code)의 GitHub Enterprise 섹션에서도 연결할 수 있습니다. 그런 다음 마켓플레이스를 다시 추가합니다. 또는 Owner에게 조직 플러그인 설정에서 마켓플레이스를 추가하도록 요청하면 사용자별 연결 요구 사항이 제거됩니다.

다른 claude.ai 표면에서 GHES 마켓플레이스의 "저장소를 찾을 수 없습니다. 비공개인 경우 GitHub 액세스가 필요합니다" 오류는 일반적으로 동일한 누락된 연결을 나타냅니다. 위의 경로 중 하나를 통해 GitHub Enterprise 계정을 연결한 다음 다시 시도합니다.

<h3 id="ghes-instance-not-reachable">
  GHES 인스턴스에 도달할 수 없음
</h3>

리뷰 또는 웹 세션이 시간 초과되면 GHES 인스턴스가 Anthropic 인프라에서 도달 가능하지 않을 수 있습니다. 방화벽이 [Anthropic API IP 주소](https://platform.claude.com/docs/ko/api/ip-addresses)에서 인바운드 연결을 허용하는지 확인합니다.

<h2 id="related-resources">
  관련 리소스
</h2>

이 페이지들은 이 가이드 전체에서 참조된 기능을 더 자세히 다룹니다:

* [웹에서 Claude Code](/docs/ko/claude-code-on-the-web): 클라우드 인프라에서 Claude Code 세션 실행
* [코드 리뷰](/docs/ko/code-review): 자동화된 PR 리뷰
* [플러그인 마켓플레이스](/docs/ko/plugin-marketplaces): 플러그인 카탈로그 구축 및 배포
* [분석](/docs/ko/analytics): 사용량 및 기여도 메트릭 추적
* [관리되는 설정](/docs/ko/settings): 조직 전체 정책 구성
* [네트워크 구성](/docs/ko/network-config): 방화벽 및 IP 허용 목록 요구 사항
